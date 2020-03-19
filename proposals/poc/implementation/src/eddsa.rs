use parking_lot::Mutex;
use ring::signature::KeyPair as _;
use std::sync::Arc;
use zeroize::Zeroize;

use super::error::*;
use super::handles::*;
use super::signature::*;
use super::signature_keypair::*;
use super::HandleManagers;

#[derive(Clone, Copy, Debug)]
pub struct EddsaSignatureOp {
    pub alg: SignatureAlgorithm,
}

impl EddsaSignatureOp {
    pub fn new(alg: SignatureAlgorithm) -> Self {
        EddsaSignatureOp { alg }
    }
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

    pub fn generate(alg: SignatureAlgorithm) -> Result<Self, CryptoError> {
        let rng = ring::rand::SystemRandom::new();
        let pkcs8 = ring::signature::Ed25519KeyPair::generate_pkcs8(&rng)
            .map_err(|_| CryptoError::RNGError)?;
        Self::from_pkcs8(alg, pkcs8.as_ref())
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

#[derive(Clone, Copy, Debug)]
pub struct EddsaSignatureKeyPairBuilder {
    pub alg: SignatureAlgorithm,
}

impl EddsaSignatureKeyPairBuilder {
    pub fn new(alg: SignatureAlgorithm) -> Self {
        EddsaSignatureKeyPairBuilder { alg }
    }

    pub fn generate(&self, handles: &HandleManagers) -> Result<Handle, CryptoError> {
        let kp = EddsaSignatureKeyPair::generate(self.alg)?;
        let handle = handles
            .signature_keypair
            .register(SignatureKeyPair::Eddsa(kp))?;
        Ok(handle)
    }

    pub fn import(
        &self,
        handles: &HandleManagers,
        encoded: &[u8],
        encoding: KeyPairEncoding,
    ) -> Result<Handle, CryptoError> {
        match encoding {
            KeyPairEncoding::Pkcs8 => {}
            _ => bail!(CryptoError::UnsupportedEncoding),
        };
        let kp = EddsaSignatureKeyPair::from_pkcs8(self.alg, encoded)?;
        let handle = handles
            .signature_keypair
            .register(SignatureKeyPair::Eddsa(kp))?;
        Ok(handle)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EddsaSignature(pub Vec<u8>);

impl AsRef<[u8]> for EddsaSignature {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl EddsaSignature {
    pub fn new(encoded: Vec<u8>) -> Self {
        EddsaSignature(encoded)
    }
}

#[derive(Debug)]
pub struct EddsaSignatureState {
    pub kp: EddsaSignatureKeyPair,
    pub input: Mutex<Vec<u8>>,
}

impl EddsaSignatureState {
    pub fn new(kp: EddsaSignatureKeyPair) -> Self {
        EddsaSignatureState {
            kp,
            input: Mutex::new(vec![]),
        }
    }

    pub fn update(&self, input: &[u8]) -> Result<(), CryptoError> {
        self.input.lock().extend_from_slice(input);
        Ok(())
    }

    pub fn sign(&self) -> Result<EddsaSignature, CryptoError> {
        let input = self.input.lock();
        let signature_u8 = self.kp.ring_kp.sign(&input).as_ref().to_vec();
        let signature = EddsaSignature(signature_u8);
        Ok(signature)
    }
}

#[derive(Debug)]
pub struct EddsaSignatureVerificationState {
    pub pk: EddsaSignaturePublicKey,
    pub input: Mutex<Vec<u8>>,
}

impl EddsaSignatureVerificationState {
    pub fn new(pk: EddsaSignaturePublicKey) -> Self {
        EddsaSignatureVerificationState {
            pk,
            input: Mutex::new(vec![]),
        }
    }

    pub fn update(&self, input: &[u8]) -> Result<(), CryptoError> {
        self.input.lock().extend_from_slice(input);
        Ok(())
    }

    pub fn verify(&self, signature: &EddsaSignature) -> Result<(), CryptoError> {
        let ring_alg = match self.pk.alg {
            SignatureAlgorithm::Ed25519 => &ring::signature::ED25519,
            _ => bail!(CryptoError::UnsupportedAlgorithm),
        };
        let ring_pk = ring::signature::UnparsedPublicKey::new(ring_alg, self.pk.as_raw()?);
        ring_pk
            .verify(self.input.lock().as_ref(), signature.as_ref())
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
