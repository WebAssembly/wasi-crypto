use ring::signature::KeyPair as _;
use std::sync::Arc;
use zeroize::Zeroize;

use super::*;
use crate::asymmetric_common::*;
use crate::error::*;

#[derive(Debug, Clone)]
pub struct RsaSignatureSecretKey {
    pub alg: SignatureAlgorithm,
}

#[derive(Clone, Debug)]
pub struct RsaSignatureKeyPair {
    pub alg: SignatureAlgorithm,
    pub pkcs8: Vec<u8>,
    pub ring_kp: Arc<ring::signature::RsaKeyPair>,
}

impl Drop for RsaSignatureKeyPair {
    fn drop(&mut self) {
        self.pkcs8.zeroize();
    }
}

impl RsaSignatureKeyPair {
    pub fn from_pkcs8(alg: SignatureAlgorithm, pkcs8: &[u8]) -> Result<Self, CryptoError> {
        let ring_kp =
            ring::signature::RsaKeyPair::from_pkcs8(pkcs8).map_err(|_| CryptoError::InvalidKey)?;
        let kp = RsaSignatureKeyPair {
            alg,
            pkcs8: pkcs8.to_vec(),
            ring_kp: Arc::new(ring_kp),
        };
        Ok(kp)
    }

    pub fn as_pkcs8(&self) -> Result<&[u8], CryptoError> {
        Ok(&self.pkcs8)
    }

    #[allow(dead_code)]
    pub fn generate(
        _alg: SignatureAlgorithm,
        _options: Option<SignatureOptions>,
    ) -> Result<Self, CryptoError> {
        bail!(CryptoError::NotImplemented)
    }

    pub fn import(
        alg: SignatureAlgorithm,
        encoded: &[u8],
        encoding: KeyPairEncoding,
    ) -> Result<Self, CryptoError> {
        match encoding {
            KeyPairEncoding::Pkcs8 => {}
            _ => bail!(CryptoError::UnsupportedEncoding),
        };
        let kp = RsaSignatureKeyPair::from_pkcs8(alg, encoded)?;
        Ok(kp)
    }

    pub fn raw_public_key(&self) -> &[u8] {
        self.ring_kp.public_key().as_ref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RsaSignature(pub Vec<u8>);

impl RsaSignature {
    pub fn new(encoded: Vec<u8>) -> Self {
        RsaSignature(encoded)
    }
}

impl SignatureLike for RsaSignature {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

#[derive(Debug)]
pub struct RsaSignatureState {
    pub kp: RsaSignatureKeyPair,
    pub input: Vec<u8>,
}

impl RsaSignatureState {
    pub fn new(kp: RsaSignatureKeyPair) -> Self {
        RsaSignatureState { kp, input: vec![] }
    }
}

impl SignatureStateLike for RsaSignatureState {
    fn update(&mut self, input: &[u8]) -> Result<(), CryptoError> {
        self.input.extend_from_slice(input);
        Ok(())
    }

    fn sign(&mut self) -> Result<Signature, CryptoError> {
        let rng = ring::rand::SystemRandom::new();
        let mut signature_u8 = vec![];
        let padding_alg = match self.kp.alg {
            SignatureAlgorithm::RSA_PKCS1_2048_8192_SHA256 => &ring::signature::RSA_PKCS1_SHA256,
            SignatureAlgorithm::RSA_PKCS1_2048_8192_SHA384 => &ring::signature::RSA_PKCS1_SHA384,
            SignatureAlgorithm::RSA_PKCS1_2048_8192_SHA512 => &ring::signature::RSA_PKCS1_SHA512,
            SignatureAlgorithm::RSA_PKCS1_3072_8192_SHA384 => &ring::signature::RSA_PKCS1_SHA384,
            _ => bail!(CryptoError::UnsupportedAlgorithm),
        };
        self.kp
            .ring_kp
            .sign(padding_alg, &rng, &self.input, &mut signature_u8)
            .map_err(|_| CryptoError::AlgorithmFailure)?;
        let signature = RsaSignature(signature_u8);
        Ok(Signature::new(Box::new(signature)))
    }
}

#[derive(Debug)]
pub struct RsaSignatureVerificationState {
    pub pk: RsaSignaturePublicKey,
    pub input: Vec<u8>,
}

impl RsaSignatureVerificationState {
    pub fn new(pk: RsaSignaturePublicKey) -> Self {
        RsaSignatureVerificationState { pk, input: vec![] }
    }
}

impl SignatureVerificationStateLike for RsaSignatureVerificationState {
    fn update(&mut self, input: &[u8]) -> Result<(), CryptoError> {
        self.input.extend_from_slice(input);
        Ok(())
    }

    fn verify(&self, signature: &Signature) -> Result<(), CryptoError> {
        let signature = signature.inner();
        let signature = signature
            .as_any()
            .downcast_ref::<RsaSignature>()
            .ok_or(CryptoError::InvalidSignature)?;
        let ring_alg = match self.pk.alg {
            SignatureAlgorithm::RSA_PKCS1_2048_8192_SHA256 => {
                &ring::signature::RSA_PKCS1_2048_8192_SHA256
            }
            SignatureAlgorithm::RSA_PKCS1_2048_8192_SHA384 => {
                &ring::signature::RSA_PKCS1_2048_8192_SHA384
            }
            SignatureAlgorithm::RSA_PKCS1_2048_8192_SHA512 => {
                &ring::signature::RSA_PKCS1_2048_8192_SHA512
            }
            SignatureAlgorithm::RSA_PKCS1_3072_8192_SHA384 => {
                &ring::signature::RSA_PKCS1_3072_8192_SHA384
            }
            _ => bail!(CryptoError::UnsupportedAlgorithm),
        };
        let ring_pk = ring::signature::UnparsedPublicKey::new(ring_alg, self.pk.as_raw()?);
        ring_pk
            .verify(self.input.as_ref(), signature.as_ref())
            .map_err(|_| CryptoError::VerificationFailed)?;
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct RsaSignaturePublicKey {
    pub alg: SignatureAlgorithm,
    pub raw: Vec<u8>,
}

impl RsaSignaturePublicKey {
    pub fn from_raw(alg: SignatureAlgorithm, raw: &[u8]) -> Result<Self, CryptoError> {
        let pk = RsaSignaturePublicKey {
            alg,
            raw: raw.to_vec(),
        };
        Ok(pk)
    }

    pub fn as_raw(&self) -> Result<&[u8], CryptoError> {
        Ok(&self.raw)
    }
}
