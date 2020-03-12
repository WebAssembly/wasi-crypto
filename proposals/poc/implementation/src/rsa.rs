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
pub struct RSASignatureOp {
    pub alg: SignatureAlgorithm,
}

impl RSASignatureOp {
    pub fn new(alg: SignatureAlgorithm) -> Self {
        RSASignatureOp { alg }
    }
}

#[derive(Clone, Debug)]
pub struct RSASignatureKeyPair {
    pub alg: SignatureAlgorithm,
    pub pkcs8: Vec<u8>,
    pub ring_kp: Arc<ring::signature::RsaKeyPair>,
}

impl Drop for RSASignatureKeyPair {
    fn drop(&mut self) {
        self.pkcs8.zeroize();
    }
}

impl RSASignatureKeyPair {
    pub fn from_pkcs8(alg: SignatureAlgorithm, pkcs8: &[u8]) -> Result<Self, Error> {
        let ring_kp =
            ring::signature::RsaKeyPair::from_pkcs8(pkcs8).map_err(|_| CryptoError::InvalidKey)?;
        let kp = RSASignatureKeyPair {
            alg,
            pkcs8: pkcs8.to_vec(),
            ring_kp: Arc::new(ring_kp),
        };
        Ok(kp)
    }

    pub fn as_pkcs8(&self) -> Result<&[u8], Error> {
        Ok(&self.pkcs8)
    }

    #[allow(dead_code)]
    pub fn generate(_alg: SignatureAlgorithm) -> Result<Self, Error> {
        bail!(CryptoError::UnsupportedOperation)
    }

    pub fn raw_public_key(&self) -> &[u8] {
        self.ring_kp.public_key().as_ref()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct RSASignatureKeyPairBuilder {
    pub alg: SignatureAlgorithm,
}

impl RSASignatureKeyPairBuilder {
    pub fn new(alg: SignatureAlgorithm) -> Self {
        RSASignatureKeyPairBuilder { alg }
    }

    pub fn generate(&self) -> Result<Handle, Error> {
        bail!(CryptoError::UnsupportedOperation)
    }

    pub fn import(&self, encoded: &[u8], encoding: KeyPairEncoding) -> Result<Handle, Error> {
        match encoding {
            KeyPairEncoding::PKCS8 => {}
            _ => bail!(CryptoError::UnsupportedEncoding),
        };
        let kp = RSASignatureKeyPair::from_pkcs8(self.alg, encoded)?;
        let handle = WASI_CRYPTO_CTX
            .signature_keypair_manager
            .register(SignatureKeyPair::RSA(kp))?;
        Ok(handle)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RSASignature(pub Vec<u8>);

impl AsRef<[u8]> for RSASignature {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl RSASignature {
    pub fn new(encoded: Vec<u8>) -> Self {
        RSASignature(encoded)
    }
}
#[derive(Debug)]
pub struct RSASignatureState {
    pub kp: RSASignatureKeyPair,
    pub input: Mutex<Vec<u8>>,
}

impl RSASignatureState {
    pub fn new(kp: RSASignatureKeyPair) -> Self {
        RSASignatureState {
            kp,
            input: Mutex::new(vec![]),
        }
    }

    pub fn update(&self, input: &[u8]) -> Result<(), Error> {
        self.input.lock().extend_from_slice(input);
        Ok(())
    }

    pub fn sign(&self) -> Result<RSASignature, Error> {
        let rng = ring::rand::SystemRandom::new();
        let input = self.input.lock();
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
            .sign(padding_alg, &rng, &input, &mut signature_u8)
            .map_err(|_| CryptoError::AlgorithmFailure)?;
        let signature = RSASignature(signature_u8);
        Ok(signature)
    }
}

#[derive(Debug)]
pub struct RSASignatureVerificationState {
    pub pk: RSASignaturePublicKey,
    pub input: Mutex<Vec<u8>>,
}

impl RSASignatureVerificationState {
    pub fn new(pk: RSASignaturePublicKey) -> Self {
        RSASignatureVerificationState {
            pk,
            input: Mutex::new(vec![]),
        }
    }

    pub fn update(&self, input: &[u8]) -> Result<(), Error> {
        self.input.lock().extend_from_slice(input);
        Ok(())
    }

    pub fn verify(&self, signature: &RSASignature) -> Result<(), Error> {
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
            .verify(self.input.lock().as_ref(), signature.as_ref())
            .map_err(|_| CryptoError::VerificationFailed)?;
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct RSASignaturePublicKey {
    pub alg: SignatureAlgorithm,
    pub raw: Vec<u8>,
}

impl RSASignaturePublicKey {
    pub fn from_raw(alg: SignatureAlgorithm, raw: &[u8]) -> Result<Self, Error> {
        let pk = RSASignaturePublicKey {
            alg,
            raw: raw.to_vec(),
        };
        Ok(pk)
    }

    pub fn as_raw(&self) -> Result<&[u8], Error> {
        Ok(&self.raw)
    }
}
