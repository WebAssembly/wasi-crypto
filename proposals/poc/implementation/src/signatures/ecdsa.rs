use ring::signature::KeyPair as _;
use std::sync::Arc;
use zeroize::Zeroize;

use super::signature::*;
use super::*;
use crate::asymmetric_common::*;
use crate::error::*;

#[derive(Debug, Clone)]
pub struct EcdsaSignatureKeyPair {
    pub alg: SignatureAlgorithm,
    pub pkcs8: Vec<u8>,
    pub ring_kp: Arc<ring::signature::EcdsaKeyPair>,
}

impl Drop for EcdsaSignatureKeyPair {
    fn drop(&mut self) {
        self.pkcs8.zeroize();
    }
}

impl EcdsaSignatureKeyPair {
    fn ring_alg_from_alg(
        alg: SignatureAlgorithm,
    ) -> Result<&'static ring::signature::EcdsaSigningAlgorithm, CryptoError> {
        let ring_alg = match alg {
            SignatureAlgorithm::ECDSA_P256_SHA256 => {
                &ring::signature::ECDSA_P256_SHA256_FIXED_SIGNING
            }
            SignatureAlgorithm::ECDSA_P384_SHA384 => {
                &ring::signature::ECDSA_P384_SHA384_FIXED_SIGNING
            }
            _ => bail!(CryptoError::UnsupportedAlgorithm),
        };
        Ok(ring_alg)
    }

    pub fn from_pkcs8(alg: SignatureAlgorithm, pkcs8: &[u8]) -> Result<Self, CryptoError> {
        let ring_alg = Self::ring_alg_from_alg(alg)?;
        let ring_kp = ring::signature::EcdsaKeyPair::from_pkcs8(ring_alg, pkcs8)
            .map_err(|_| CryptoError::InvalidKey)?;
        let kp = EcdsaSignatureKeyPair {
            alg,
            pkcs8: pkcs8.to_vec(),
            ring_kp: Arc::new(ring_kp),
        };
        Ok(kp)
    }

    pub fn as_pkcs8(&self) -> Result<&[u8], CryptoError> {
        Ok(&self.pkcs8)
    }

    pub fn generate(
        alg: SignatureAlgorithm,
        _options: Option<SignatureOptions>,
    ) -> Result<Self, CryptoError> {
        let ring_alg = Self::ring_alg_from_alg(alg)?;
        let rng = ring::rand::SystemRandom::new();
        let pkcs8 = ring::signature::EcdsaKeyPair::generate_pkcs8(ring_alg, &rng)
            .map_err(|_| CryptoError::RNGError)?;
        Self::from_pkcs8(alg, pkcs8.as_ref())
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
        let kp = EcdsaSignatureKeyPair::from_pkcs8(alg, encoded)?;
        Ok(kp)
    }

    pub fn raw_public_key(&self) -> &[u8] {
        self.ring_kp.public_key().as_ref()
    }
}

#[derive(Debug)]
pub struct EcdsaSignatureState {
    pub kp: EcdsaSignatureKeyPair,
    pub input: Vec<u8>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EcdsaSignature {
    pub encoding: SignatureEncoding,
    pub encoded: Vec<u8>,
}

impl EcdsaSignature {
    pub fn new(encoding: SignatureEncoding, encoded: Vec<u8>) -> Self {
        EcdsaSignature { encoding, encoded }
    }
}

impl SignatureLike for EcdsaSignature {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_ref(&self) -> &[u8] {
        &self.encoded
    }
}

impl EcdsaSignatureState {
    pub fn new(kp: EcdsaSignatureKeyPair) -> Self {
        EcdsaSignatureState { kp, input: vec![] }
    }
}

impl SignatureStateLike for EcdsaSignatureState {
    fn update(&mut self, input: &[u8]) -> Result<(), CryptoError> {
        self.input.extend_from_slice(input);
        Ok(())
    }

    fn sign(&mut self) -> Result<Signature, CryptoError> {
        let rng = ring::rand::SystemRandom::new();
        let encoded_signature = self
            .kp
            .ring_kp
            .sign(&rng, &self.input)
            .map_err(|_| CryptoError::AlgorithmFailure)?
            .as_ref()
            .to_vec();
        let signature = EcdsaSignature::new(SignatureEncoding::Raw, encoded_signature);
        Ok(Signature::new(Box::new(signature)))
    }
}

#[derive(Debug)]
pub struct EcdsaSignatureVerificationState {
    pub pk: EcdsaSignaturePublicKey,
    pub input: Vec<u8>,
}

impl EcdsaSignatureVerificationState {
    pub fn new(pk: EcdsaSignaturePublicKey) -> Self {
        EcdsaSignatureVerificationState { pk, input: vec![] }
    }
}

impl SignatureVerificationStateLike for EcdsaSignatureVerificationState {
    fn update(&mut self, input: &[u8]) -> Result<(), CryptoError> {
        self.input.extend_from_slice(input);
        Ok(())
    }

    fn verify(&self, signature: &Signature) -> Result<(), CryptoError> {
        let signature = signature.inner();
        let signature = signature
            .as_any()
            .downcast_ref::<EcdsaSignature>()
            .ok_or(CryptoError::InvalidSignature)?;
        let ring_alg = match (self.pk.alg, signature.encoding) {
            (SignatureAlgorithm::ECDSA_P256_SHA256, SignatureEncoding::Raw) => {
                &ring::signature::ECDSA_P256_SHA256_FIXED
            }
            (SignatureAlgorithm::ECDSA_P384_SHA384, SignatureEncoding::Raw) => {
                &ring::signature::ECDSA_P384_SHA384_FIXED
            }
            (SignatureAlgorithm::ECDSA_P256_SHA256, SignatureEncoding::Der) => {
                &ring::signature::ECDSA_P256_SHA256_ASN1
            }
            (SignatureAlgorithm::ECDSA_P384_SHA384, SignatureEncoding::Der) => {
                &ring::signature::ECDSA_P384_SHA384_ASN1
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
pub struct EcdsaSignaturePublicKey {
    pub alg: SignatureAlgorithm,
    pub raw: Vec<u8>,
}

impl EcdsaSignaturePublicKey {
    pub fn from_raw(alg: SignatureAlgorithm, raw: &[u8]) -> Result<Self, CryptoError> {
        let pk = EcdsaSignaturePublicKey {
            alg,
            raw: raw.to_vec(),
        };
        Ok(pk)
    }

    pub fn as_raw(&self) -> Result<&[u8], CryptoError> {
        Ok(&self.raw)
    }
}
