use parking_lot::Mutex;
use ring::signature::KeyPair as _;
use std::sync::Arc;
use zeroize::Zeroize;

use super::error::*;
use super::handles::*;
use super::signature::*;
use super::signature_keypair::*;
use super::WASI_CRYPTO_CTX;

#[derive(Clone, Copy, Debug)]
pub struct ECDSASignatureOp {
    pub alg: SignatureAlgorithm,
}

impl ECDSASignatureOp {
    pub fn new(alg: SignatureAlgorithm) -> Self {
        ECDSASignatureOp { alg }
    }
}

#[derive(Debug, Clone)]
pub struct ECDSASignatureKeyPair {
    pub alg: SignatureAlgorithm,
    pub pkcs8: Vec<u8>,
    pub ring_kp: Arc<ring::signature::EcdsaKeyPair>,
}

impl Drop for ECDSASignatureKeyPair {
    fn drop(&mut self) {
        self.pkcs8.zeroize();
    }
}

impl ECDSASignatureKeyPair {
    fn ring_alg_from_alg(
        alg: SignatureAlgorithm,
    ) -> Result<&'static ring::signature::EcdsaSigningAlgorithm, Error> {
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

    pub fn from_pkcs8(alg: SignatureAlgorithm, pkcs8: &[u8]) -> Result<Self, Error> {
        let ring_alg = Self::ring_alg_from_alg(alg)?;
        let ring_kp = ring::signature::EcdsaKeyPair::from_pkcs8(ring_alg, pkcs8)
            .map_err(|_| CryptoError::InvalidKey)?;
        let kp = ECDSASignatureKeyPair {
            alg,
            pkcs8: pkcs8.to_vec(),
            ring_kp: Arc::new(ring_kp),
        };
        Ok(kp)
    }

    pub fn as_pkcs8(&self) -> Result<&[u8], Error> {
        Ok(&self.pkcs8)
    }

    pub fn generate(alg: SignatureAlgorithm) -> Result<Self, Error> {
        let ring_alg = Self::ring_alg_from_alg(alg)?;
        let rng = ring::rand::SystemRandom::new();
        let pkcs8 = ring::signature::EcdsaKeyPair::generate_pkcs8(ring_alg, &rng)
            .map_err(|_| CryptoError::RNGError)?;
        Self::from_pkcs8(alg, pkcs8.as_ref())
    }

    pub fn raw_public_key(&self) -> &[u8] {
        self.ring_kp.public_key().as_ref()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ECDSASignatureKeyPairBuilder {
    pub alg: SignatureAlgorithm,
}

impl ECDSASignatureKeyPairBuilder {
    pub fn new(alg: SignatureAlgorithm) -> Self {
        ECDSASignatureKeyPairBuilder { alg }
    }

    pub fn generate(&self) -> Result<Handle, Error> {
        let kp = ECDSASignatureKeyPair::generate(self.alg)?;
        let handle = WASI_CRYPTO_CTX
            .signature_keypair_manager
            .register(SignatureKeyPair::ECDSA(kp))?;
        Ok(handle)
    }

    pub fn import(&self, encoded: &[u8], encoding: KeyPairEncoding) -> Result<Handle, Error> {
        match encoding {
            KeyPairEncoding::PKCS8 => {}
            _ => bail!(CryptoError::UnsupportedEncoding),
        };
        let kp = ECDSASignatureKeyPair::from_pkcs8(self.alg, encoded)?;
        let handle = WASI_CRYPTO_CTX
            .signature_keypair_manager
            .register(SignatureKeyPair::ECDSA(kp))?;
        Ok(handle)
    }
}

#[derive(Debug)]
pub struct ECDSASignatureState {
    pub kp: ECDSASignatureKeyPair,
    pub input: Mutex<Vec<u8>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ECDSASignature {
    pub encoding: SignatureEncoding,
    pub encoded: Vec<u8>,
}

impl AsRef<[u8]> for ECDSASignature {
    fn as_ref(&self) -> &[u8] {
        &self.encoded
    }
}

impl ECDSASignature {
    pub fn new(encoding: SignatureEncoding, encoded: Vec<u8>) -> Self {
        ECDSASignature { encoding, encoded }
    }
}

impl ECDSASignatureState {
    pub fn new(kp: ECDSASignatureKeyPair) -> Self {
        ECDSASignatureState {
            kp,
            input: Mutex::new(vec![]),
        }
    }

    pub fn update(&self, input: &[u8]) -> Result<(), Error> {
        self.input.lock().extend_from_slice(input);
        Ok(())
    }

    pub fn sign(&self) -> Result<ECDSASignature, Error> {
        let rng = ring::rand::SystemRandom::new();
        let input = self.input.lock();
        let encoded_signature = self
            .kp
            .ring_kp
            .sign(&rng, &input)
            .map_err(|_| CryptoError::AlgorithmFailure)?
            .as_ref()
            .to_vec();
        let signature = ECDSASignature::new(SignatureEncoding::Raw, encoded_signature);
        Ok(signature)
    }
}

#[derive(Debug)]
pub struct ECDSASignatureVerificationState {
    pub pk: ECDSASignaturePublicKey,
    pub input: Mutex<Vec<u8>>,
}

impl ECDSASignatureVerificationState {
    pub fn new(pk: ECDSASignaturePublicKey) -> Self {
        ECDSASignatureVerificationState {
            pk,
            input: Mutex::new(vec![]),
        }
    }

    pub fn update(&self, input: &[u8]) -> Result<(), Error> {
        self.input.lock().extend_from_slice(input);
        Ok(())
    }

    pub fn verify(&self, signature: &ECDSASignature) -> Result<(), Error> {
        let ring_alg = match (self.pk.alg, signature.encoding) {
            (SignatureAlgorithm::ECDSA_P256_SHA256, SignatureEncoding::Raw) => {
                &ring::signature::ECDSA_P256_SHA256_FIXED
            }
            (SignatureAlgorithm::ECDSA_P384_SHA384, SignatureEncoding::Raw) => {
                &ring::signature::ECDSA_P384_SHA384_FIXED
            }
            (SignatureAlgorithm::ECDSA_P256_SHA256, SignatureEncoding::DER) => {
                &ring::signature::ECDSA_P256_SHA256_ASN1
            }
            (SignatureAlgorithm::ECDSA_P384_SHA384, SignatureEncoding::DER) => {
                &ring::signature::ECDSA_P384_SHA384_ASN1
            }
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
pub struct ECDSASignaturePublicKey {
    pub alg: SignatureAlgorithm,
    pub raw: Vec<u8>,
}

impl ECDSASignaturePublicKey {
    pub fn from_raw(alg: SignatureAlgorithm, raw: &[u8]) -> Result<Self, Error> {
        let pk = ECDSASignaturePublicKey {
            alg,
            raw: raw.to_vec(),
        };
        Ok(pk)
    }

    pub fn as_raw(&self) -> Result<&[u8], Error> {
        Ok(&self.raw)
    }
}
