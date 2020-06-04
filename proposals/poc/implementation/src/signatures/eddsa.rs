use ring::signature::KeyPair as _;
use std::sync::Arc;
use zeroize::Zeroize;

use super::*;
use crate::asymmetric_common::*;
use crate::error::*;

#[derive(Debug, Clone)]
pub struct EddsaSignatureSecretKey {
    pub alg: SignatureAlgorithm,
}

#[derive(Clone, Debug)]
pub struct EddsaSignatureKeyPair {
    pub alg: SignatureAlgorithm,
    pub pkcs8: Vec<u8>,
    pub ring_kp: Arc<ring::signature::Ed25519KeyPair>,
}

impl EddsaSignatureKeyPair {
    pub fn from_pkcs8(alg: SignatureAlgorithm, pkcs8: &[u8]) -> Result<Self, CryptoError> {
        let ring_kp = ring::signature::Ed25519KeyPair::from_pkcs8(pkcs8)
            .map_err(|_| CryptoError::InvalidKey)?;
        let kp = EddsaSignatureKeyPair {
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
        let rng = ring::rand::SystemRandom::new();
        let pkcs8 = ring::signature::Ed25519KeyPair::generate_pkcs8(&rng)
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
        let kp = EddsaSignatureKeyPair::from_pkcs8(alg, encoded)?;
        Ok(kp)
    }

    pub fn raw_public_key(&self) -> &[u8] {
        self.ring_kp.public_key().as_ref()
    }
}

impl Drop for EddsaSignatureKeyPair {
    fn drop(&mut self) {
        self.pkcs8.zeroize();
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EddsaSignature(pub Vec<u8>);

impl EddsaSignature {
    pub fn new(encoded: Vec<u8>) -> Self {
        EddsaSignature(encoded)
    }
}

impl SignatureLike for EddsaSignature {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

#[derive(Debug)]
pub struct EddsaSignatureState {
    pub kp: EddsaSignatureKeyPair,
    pub input: Vec<u8>,
}

impl EddsaSignatureState {
    pub fn new(kp: EddsaSignatureKeyPair) -> Self {
        EddsaSignatureState { kp, input: vec![] }
    }
}

impl SignatureStateLike for EddsaSignatureState {
    fn update(&mut self, input: &[u8]) -> Result<(), CryptoError> {
        self.input.extend_from_slice(input);
        Ok(())
    }

    fn sign(&mut self) -> Result<Signature, CryptoError> {
        let signature_u8 = self.kp.ring_kp.sign(&self.input).as_ref().to_vec();
        let signature = EddsaSignature(signature_u8);
        Ok(Signature::new(Box::new(signature)))
    }
}

#[derive(Debug)]
pub struct EddsaSignatureVerificationState {
    pub pk: EddsaSignaturePublicKey,
    pub input: Vec<u8>,
}

impl EddsaSignatureVerificationState {
    pub fn new(pk: EddsaSignaturePublicKey) -> Self {
        EddsaSignatureVerificationState { pk, input: vec![] }
    }
}

impl SignatureVerificationStateLike for EddsaSignatureVerificationState {
    fn update(&mut self, input: &[u8]) -> Result<(), CryptoError> {
        self.input.extend_from_slice(input);
        Ok(())
    }

    fn verify(&self, signature: &Signature) -> Result<(), CryptoError> {
        let signature = signature.inner();
        let signature = signature
            .as_any()
            .downcast_ref::<EddsaSignature>()
            .ok_or(CryptoError::InvalidSignature)?;
        let ring_alg = match self.pk.alg {
            SignatureAlgorithm::Ed25519 => &ring::signature::ED25519,
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
pub struct EddsaSignaturePublicKey {
    pub alg: SignatureAlgorithm,
    pub raw: Vec<u8>,
}

impl EddsaSignaturePublicKey {
    pub fn from_raw(alg: SignatureAlgorithm, raw: &[u8]) -> Result<Self, CryptoError> {
        let pk = EddsaSignaturePublicKey {
            alg,
            raw: raw.to_vec(),
        };
        Ok(pk)
    }

    pub fn as_raw(&self) -> Result<&[u8], CryptoError> {
        Ok(&self.raw)
    }
}
