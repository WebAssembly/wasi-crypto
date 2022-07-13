use ::sha2::{Digest, Sha256, Sha384, Sha512};
use boring::{bn, pkey, rsa};
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::ops::Deref;
use std::sync::Arc;
use zeroize::Zeroize;

use super::*;
use crate::asymmetric_common::*;
use crate::error::*;
use crate::rand::SecureRandom;

const RAW_ENCODING_VERSION: u16 = 2;
const RAW_ENCODING_ALG_ID: u16 = 1;
const MIN_MODULUS_SIZE: u32 = 2048;
const MAX_MODULUS_SIZE: u32 = 4096;

#[derive(Debug, Clone)]
pub struct RsaSignatureSecretKey {
    pub alg: SignatureAlgorithm,
}

#[derive(Serialize, Deserialize, Zeroize)]
struct RsaSignatureKeyPairParts {
    version: u16,
    alg_id: u16,
    n: Vec<u8>,
    e: Vec<u8>,
    d: Vec<u8>,
    p: Vec<u8>,
    q: Vec<u8>,
    dmp1: Vec<u8>,
    dmq1: Vec<u8>,
    iqmp: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct RsaSignatureKeyPair {
    pub alg: SignatureAlgorithm,
    ctx: rsa::Rsa<pkey::Private>,
}

fn modulus_bits(alg: SignatureAlgorithm) -> Result<u32, CryptoError> {
    let modulus_bits = match alg {
        SignatureAlgorithm::RSA_PKCS1_2048_SHA256
        | SignatureAlgorithm::RSA_PKCS1_2048_SHA384
        | SignatureAlgorithm::RSA_PKCS1_2048_SHA512
        | SignatureAlgorithm::RSA_PSS_2048_SHA256
        | SignatureAlgorithm::RSA_PSS_2048_SHA384
        | SignatureAlgorithm::RSA_PSS_2048_SHA512 => 2048,
        SignatureAlgorithm::RSA_PKCS1_3072_SHA384
        | SignatureAlgorithm::RSA_PKCS1_3072_SHA512
        | SignatureAlgorithm::RSA_PSS_3072_SHA384
        | SignatureAlgorithm::RSA_PSS_3072_SHA512 => 3072,
        SignatureAlgorithm::RSA_PKCS1_4096_SHA512 | SignatureAlgorithm::RSA_PSS_4096_SHA512 => 4096,
        _ => bail!(CryptoError::UnsupportedAlgorithm),
    };
    Ok(modulus_bits)
}

impl RsaSignatureKeyPair {
    fn from_pkcs8(alg: SignatureAlgorithm, der: &[u8]) -> Result<Self, CryptoError> {
        ensure!(der.len() < 4096, CryptoError::InvalidKey);
        let ctx: rsa::Rsa<pkey::Private> =
            rsa::Rsa::private_key_from_der(der).map_err(|_| CryptoError::InvalidKey)?;
        Ok(RsaSignatureKeyPair { alg, ctx })
    }

    fn from_pem(alg: SignatureAlgorithm, pem: &[u8]) -> Result<Self, CryptoError> {
        ensure!(pem.len() < 4096, CryptoError::InvalidKey);
        let ctx: rsa::Rsa<pkey::Private> =
            rsa::Rsa::private_key_from_pem(pem).map_err(|_| CryptoError::InvalidKey)?;
        Ok(RsaSignatureKeyPair { alg, ctx })
    }

    fn from_local(alg: SignatureAlgorithm, local: &[u8]) -> Result<Self, CryptoError> {
        ensure!(local.len() < 2048, CryptoError::InvalidKey);
        let parts: RsaSignatureKeyPairParts =
            bincode::deserialize(local).map_err(|_| CryptoError::InvalidKey)?;
        ensure!(
            parts.version == RAW_ENCODING_VERSION && parts.alg_id == RAW_ENCODING_ALG_ID,
            CryptoError::InvalidKey
        );
        let n = bn::BigNum::from_slice(&parts.n).map_err(|_| CryptoError::InvalidKey)?;
        let e = bn::BigNum::from_slice(&parts.e).map_err(|_| CryptoError::InvalidKey)?;
        let d = bn::BigNum::from_slice(&parts.d).map_err(|_| CryptoError::InvalidKey)?;
        let p = bn::BigNum::from_slice(&parts.p).map_err(|_| CryptoError::InvalidKey)?;
        let q = bn::BigNum::from_slice(&parts.q).map_err(|_| CryptoError::InvalidKey)?;
        let dmp1 = bn::BigNum::from_slice(&parts.dmp1).map_err(|_| CryptoError::InvalidKey)?;
        let dmq1 = bn::BigNum::from_slice(&parts.dmq1).map_err(|_| CryptoError::InvalidKey)?;
        let iqmp = bn::BigNum::from_slice(&parts.iqmp).map_err(|_| CryptoError::InvalidKey)?;
        let ctx: rsa::Rsa<pkey::Private> =
            rsa::Rsa::from_private_components(n, e, d, p, q, dmp1, dmq1, iqmp)
                .map_err(|_| CryptoError::InvalidKey)?;
        Ok(RsaSignatureKeyPair { alg, ctx })
    }

    fn to_pkcs8(&self) -> Result<Vec<u8>, CryptoError> {
        self.ctx
            .private_key_to_der()
            .map_err(|_| CryptoError::InternalError)
    }

    fn to_pem(&self) -> Result<Vec<u8>, CryptoError> {
        self.ctx
            .private_key_to_pem()
            .map_err(|_| CryptoError::InternalError)
    }

    fn to_local(&self) -> Result<Vec<u8>, CryptoError> {
        let parts = RsaSignatureKeyPairParts {
            version: RAW_ENCODING_VERSION,
            alg_id: RAW_ENCODING_ALG_ID,
            n: self.ctx.n().to_vec(),
            e: self.ctx.e().to_vec(),
            d: self.ctx.d().to_vec(),
            p: self.ctx.p().ok_or(CryptoError::InternalError)?.to_vec(),
            q: self.ctx.q().ok_or(CryptoError::InternalError)?.to_vec(),
            dmp1: self.ctx.dmp1().ok_or(CryptoError::InternalError)?.to_vec(),
            dmq1: self.ctx.dmq1().ok_or(CryptoError::InternalError)?.to_vec(),
            iqmp: self.ctx.iqmp().ok_or(CryptoError::InternalError)?.to_vec(),
        };
        let local = bincode::serialize(&parts).map_err(|_| CryptoError::InternalError)?;
        Ok(local)
    }

    pub fn generate(
        alg: SignatureAlgorithm,
        _options: Option<SignatureOptions>,
    ) -> Result<Self, CryptoError> {
        let modulus_bits = modulus_bits(alg)?;
        let ctx: rsa::Rsa<pkey::Private> =
            rsa::Rsa::generate(modulus_bits).map_err(|_| CryptoError::UnsupportedAlgorithm)?;
        Ok(RsaSignatureKeyPair { alg, ctx })
    }

    pub fn import(
        alg: SignatureAlgorithm,
        encoded: &[u8],
        encoding: KeyPairEncoding,
    ) -> Result<Self, CryptoError> {
        match alg.family() {
            SignatureAlgorithmFamily::RSA => {}
            _ => bail!(CryptoError::UnsupportedAlgorithm),
        };
        let kp = match encoding {
            KeyPairEncoding::Pkcs8 => Self::from_pkcs8(alg, encoded)?,
            KeyPairEncoding::Pem => Self::from_pem(alg, encoded)?,
            KeyPairEncoding::Local => Self::from_local(alg, encoded)?,
            _ => bail!(CryptoError::UnsupportedEncoding),
        };
        let modulus_size = kp.ctx.size();
        let min_modulus_bits = modulus_bits(alg)?;
        ensure!(
            (min_modulus_bits / 8..=MAX_MODULUS_SIZE / 8).contains(&modulus_size),
            CryptoError::InvalidKey
        );
        kp.ctx.check_key().map_err(|_| CryptoError::InvalidKey)?;
        Ok(kp)
    }

    pub fn export(&self, encoding: KeyPairEncoding) -> Result<Vec<u8>, CryptoError> {
        match encoding {
            KeyPairEncoding::Pkcs8 => self.to_pkcs8(),
            KeyPairEncoding::Pem => self.to_pem(),
            KeyPairEncoding::Local => self.to_local(),
            _ => bail!(CryptoError::UnsupportedEncoding),
        }
    }

    pub fn public_key(&self) -> Result<RsaSignaturePublicKey, CryptoError> {
        let ctx = rsa::Rsa::from_public_components(
            self.ctx
                .n()
                .to_owned()
                .map_err(|_| CryptoError::InternalError)?,
            self.ctx
                .e()
                .to_owned()
                .map_err(|_| CryptoError::InternalError)?,
        )
        .map_err(|_| CryptoError::InvalidKey)?;
        Ok(RsaSignaturePublicKey { alg: self.alg, ctx })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RsaSignature {
    pub raw: Vec<u8>,
}

impl RsaSignature {
    pub fn new(raw: Vec<u8>) -> Self {
        RsaSignature { raw }
    }

    pub fn from_raw(alg: SignatureAlgorithm, raw: &[u8]) -> Result<Self, CryptoError> {
        let expected_len = (modulus_bits(alg)? / 8) as _;
        ensure!(raw.len() == expected_len, CryptoError::InvalidSignature);
        Ok(Self::new(raw.to_vec()))
    }
}

impl SignatureLike for RsaSignature {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_ref(&self) -> &[u8] {
        &self.raw
    }
}

fn padding_scheme(alg: SignatureAlgorithm) -> (rsa::Padding, boring::hash::MessageDigest) {
    match alg {
        SignatureAlgorithm::RSA_PKCS1_2048_SHA256 => {
            (rsa::Padding::PKCS1, boring::hash::MessageDigest::sha256())
        }
        SignatureAlgorithm::RSA_PKCS1_2048_SHA384 | SignatureAlgorithm::RSA_PKCS1_3072_SHA384 => {
            (rsa::Padding::PKCS1, boring::hash::MessageDigest::sha384())
        }
        SignatureAlgorithm::RSA_PKCS1_2048_SHA512
        | SignatureAlgorithm::RSA_PKCS1_3072_SHA512
        | SignatureAlgorithm::RSA_PKCS1_4096_SHA512 => {
            (rsa::Padding::PKCS1, boring::hash::MessageDigest::sha512())
        }

        SignatureAlgorithm::RSA_PSS_2048_SHA256 => {
            (rsa::Padding::PKCS1, boring::hash::MessageDigest::sha256())
        }
        SignatureAlgorithm::RSA_PSS_2048_SHA384 | SignatureAlgorithm::RSA_PSS_3072_SHA384 => (
            rsa::Padding::PKCS1_PSS,
            boring::hash::MessageDigest::sha384(),
        ),
        SignatureAlgorithm::RSA_PSS_2048_SHA512
        | SignatureAlgorithm::RSA_PSS_3072_SHA512
        | SignatureAlgorithm::RSA_PSS_4096_SHA512 => (
            rsa::Padding::PKCS1_PSS,
            boring::hash::MessageDigest::sha512(),
        ),
        _ => unreachable!(),
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
enum HashVariant {
    Sha256(Sha256),
    Sha384(Sha384),
    Sha512(Sha512),
}

impl HashVariant {
    fn for_alg(alg: SignatureAlgorithm) -> Result<Self, CryptoError> {
        let h = match alg {
            SignatureAlgorithm::RSA_PKCS1_2048_SHA256 | SignatureAlgorithm::RSA_PSS_2048_SHA256 => {
                HashVariant::Sha256(Sha256::new())
            }
            SignatureAlgorithm::RSA_PKCS1_2048_SHA384
            | SignatureAlgorithm::RSA_PKCS1_3072_SHA384
            | SignatureAlgorithm::RSA_PSS_2048_SHA384
            | SignatureAlgorithm::RSA_PSS_3072_SHA384 => HashVariant::Sha384(Sha384::new()),
            SignatureAlgorithm::RSA_PKCS1_2048_SHA512
            | SignatureAlgorithm::RSA_PKCS1_3072_SHA512
            | SignatureAlgorithm::RSA_PKCS1_4096_SHA512
            | SignatureAlgorithm::RSA_PSS_2048_SHA512
            | SignatureAlgorithm::RSA_PSS_3072_SHA512
            | SignatureAlgorithm::RSA_PSS_4096_SHA512 => HashVariant::Sha512(Sha512::new()),
            _ => bail!(CryptoError::UnsupportedAlgorithm),
        };
        Ok(h)
    }
}

pub struct RsaSignatureState<'z> {
    ctx: Box<pkey::PKey<pkey::Private>>,
    signer: boring::sign::Signer<'z>,
}

impl<'z> RsaSignatureState<'z> {
    pub fn new(kp: RsaSignatureKeyPair) -> Self {
        let ctx = Box::new(pkey::PKey::from_rsa(kp.ctx).unwrap());
        let (padding_alg, padding_hash) = padding_scheme(kp.alg);
        let pkr: *const pkey::PKeyRef<pkey::Private> = ctx.as_ref().borrow();
        let mut signer = boring::sign::Signer::new(padding_hash, unsafe { &*pkr }).unwrap();
        signer
            .set_rsa_padding(padding_alg)
            .expect("Unexpected padding");
        RsaSignatureState { ctx, signer }
    }
}

impl<'z> SignatureStateLike for RsaSignatureState<'z> {
    fn update(&mut self, input: &[u8]) -> Result<(), CryptoError> {
        self.signer
            .update(input)
            .map_err(|_| CryptoError::InternalError)?;
        Ok(())
    }

    fn sign(&mut self) -> Result<Signature, CryptoError> {
        let signature = self
            .signer
            .sign_to_vec()
            .map_err(|_| CryptoError::InternalError)?;
        let signature = RsaSignature::new(signature);
        Ok(Signature::new(Box::new(signature)))
    }
}

pub struct RsaSignatureVerificationState<'z> {
    ctx: Box<pkey::PKey<pkey::Public>>,
    verifier: boring::sign::Verifier<'z>,
}

impl<'z> RsaSignatureVerificationState<'z> {
    pub fn new(pk: RsaSignaturePublicKey) -> Self {
        let ctx = Box::new(pkey::PKey::from_rsa(pk.ctx).unwrap());
        let (padding_alg, padding_hash) = padding_scheme(pk.alg);
        let pkr: *const pkey::PKeyRef<pkey::Public> = ctx.as_ref().borrow();
        let mut verifier = boring::sign::Verifier::new(padding_hash, unsafe { &*pkr }).unwrap();
        verifier
            .set_rsa_padding(padding_alg)
            .expect("Unexpected padding");

        RsaSignatureVerificationState { ctx, verifier }
    }
}

impl<'t> SignatureVerificationStateLike for RsaSignatureVerificationState<'t> {
    fn update(&mut self, input: &[u8]) -> Result<(), CryptoError> {
        self.verifier
            .update(input)
            .map_err(|_| CryptoError::InternalError)?;
        Ok(())
    }

    fn verify(&self, signature: &Signature) -> Result<(), CryptoError> {
        let signature = signature.inner();
        let signature = signature
            .as_any()
            .downcast_ref::<RsaSignature>()
            .ok_or(CryptoError::InvalidSignature)?;
        if !self
            .verifier
            .verify(signature.as_ref())
            .map_err(|_| CryptoError::InvalidSignature)?
        {
            return Err(CryptoError::InvalidSignature);
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Zeroize)]
struct RsaSignaturePublicKeyParts {
    version: u16,
    alg_id: u16,
    n: Vec<u8>,
    e: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct RsaSignaturePublicKey {
    pub alg: SignatureAlgorithm,
    ctx: rsa::Rsa<pkey::Public>,
}

impl RsaSignaturePublicKey {
    fn from_pkcs8(alg: SignatureAlgorithm, der: &[u8]) -> Result<Self, CryptoError> {
        ensure!(der.len() < 4096, CryptoError::InvalidKey);
        let ctx = rsa::Rsa::public_key_from_der(der).map_err(|_| CryptoError::InvalidKey)?;
        Ok(RsaSignaturePublicKey { alg, ctx })
    }

    fn from_pem(alg: SignatureAlgorithm, pem: &[u8]) -> Result<Self, CryptoError> {
        ensure!(pem.len() < 4096, CryptoError::InvalidKey);
        let ctx = rsa::Rsa::public_key_from_pem(pem)
            .or_else(|_| rsa::Rsa::public_key_from_pem_pkcs1(pem))
            .map_err(|_| CryptoError::InvalidKey)?;
        Ok(RsaSignaturePublicKey { alg, ctx })
    }

    fn from_local(alg: SignatureAlgorithm, local: &[u8]) -> Result<Self, CryptoError> {
        ensure!(local.len() < 1024, CryptoError::InvalidKey);
        let parts: RsaSignaturePublicKeyParts =
            bincode::deserialize(local).map_err(|_| CryptoError::InvalidKey)?;
        ensure!(
            parts.version == RAW_ENCODING_VERSION && parts.alg_id == RAW_ENCODING_ALG_ID,
            CryptoError::InvalidKey
        );
        let n = bn::BigNum::from_slice(&parts.n).map_err(|_| CryptoError::InvalidKey)?;
        let e = bn::BigNum::from_slice(&parts.e).map_err(|_| CryptoError::InvalidKey)?;
        let ctx: rsa::Rsa<pkey::Public> =
            rsa::Rsa::from_public_components(n, e).map_err(|_| CryptoError::InvalidKey)?;
        Ok(RsaSignaturePublicKey { alg, ctx })
    }

    fn to_pkcs8(&self) -> Result<Vec<u8>, CryptoError> {
        self.ctx
            .public_key_to_der()
            .map_err(|_| CryptoError::InternalError)
    }

    fn to_pem(&self) -> Result<Vec<u8>, CryptoError> {
        self.ctx
            .public_key_to_pem()
            .map_err(|_| CryptoError::InternalError)
    }

    fn to_local(&self) -> Result<Vec<u8>, CryptoError> {
        let parts = RsaSignaturePublicKeyParts {
            version: RAW_ENCODING_VERSION,
            alg_id: RAW_ENCODING_ALG_ID,
            n: self.ctx.n().to_vec(),
            e: self.ctx.e().to_vec(),
        };
        let local = bincode::serialize(&parts).map_err(|_| CryptoError::InternalError)?;
        Ok(local)
    }

    pub fn import(
        alg: SignatureAlgorithm,
        encoded: &[u8],
        encoding: PublicKeyEncoding,
    ) -> Result<Self, CryptoError> {
        let pk = match encoding {
            PublicKeyEncoding::Pkcs8 => Self::from_pkcs8(alg, encoded)?,
            PublicKeyEncoding::Pem => Self::from_pem(alg, encoded)?,
            PublicKeyEncoding::Local => Self::from_local(alg, encoded)?,
            _ => bail!(CryptoError::UnsupportedEncoding),
        };
        let modulus_size = pk.ctx.size();
        let min_modulus_bits = modulus_bits(alg)?;
        ensure!(
            modulus_size >= min_modulus_bits / 8 && modulus_size <= MAX_MODULUS_SIZE / 8,
            CryptoError::InvalidKey
        );
        Ok(pk)
    }

    pub fn export(&self, encoding: PublicKeyEncoding) -> Result<Vec<u8>, CryptoError> {
        match encoding {
            PublicKeyEncoding::Pkcs8 => self.to_pkcs8(),
            PublicKeyEncoding::Pem => self.to_pem(),
            PublicKeyEncoding::Local => self.to_local(),
            _ => bail!(CryptoError::UnsupportedEncoding),
        }
    }
}
