use std::sync::Arc;

use super::ecdsa::*;
use super::eddsa::*;
use super::error::*;
use super::handles::*;
use super::rsa::*;
use super::signature_keypair::*;
use super::signature_publickey::*;
use super::WASI_CRYPTO_CTX;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u16)]
pub enum SignatureAlgorithm {
    ECDSA_P256_SHA256,
    ECDSA_P384_SHA384,
    Ed25519,
    RSA_PKCS1_2048_8192_SHA256,
    RSA_PKCS1_2048_8192_SHA384,
    RSA_PKCS1_2048_8192_SHA512,
    RSA_PKCS1_3072_8192_SHA384,
}

#[derive(Clone, Debug)]
pub enum Signature {
    ECDSA(ECDSASignature),
    EdDSA(EdDSASignature),
    RSA(RSASignature),
}

impl AsRef<[u8]> for Signature {
    fn as_ref(&self) -> &[u8] {
        match self {
            Signature::ECDSA(signature) => signature.as_ref(),
            Signature::EdDSA(signature) => signature.as_ref(),
            Signature::RSA(signature) => signature.as_ref(),
        }
    }
}

impl PartialEq for Signature {
    fn eq(&self, other: &Self) -> bool {
        ring::constant_time::verify_slices_are_equal(self.as_ref(), other.as_ref()).is_ok()
    }
}

impl Eq for Signature {}

impl Signature {
    fn from_raw(alg: SignatureAlgorithm, encoded: &[u8]) -> Result<Self, Error> {
        let signature = match alg {
            SignatureAlgorithm::ECDSA_P256_SHA256 => {
                ensure!(encoded.len() == 64, "Unexpected signature length");
                Signature::ECDSA(ECDSASignature::new(
                    SignatureEncoding::Raw,
                    encoded.to_vec(),
                ))
            }
            SignatureAlgorithm::ECDSA_P384_SHA384 => {
                ensure!(encoded.len() == 96, "Unexpected signature length");
                Signature::ECDSA(ECDSASignature::new(
                    SignatureEncoding::Raw,
                    encoded.to_vec(),
                ))
            }
            SignatureAlgorithm::Ed25519 => {
                ensure!(encoded.len() == 64, "Unexpected signature length");
                Signature::EdDSA(EdDSASignature::new(encoded.to_vec()))
            }
            SignatureAlgorithm::RSA_PKCS1_2048_8192_SHA256 => {
                Signature::RSA(RSASignature::new(encoded.to_vec()))
            }
            SignatureAlgorithm::RSA_PKCS1_2048_8192_SHA384 => {
                Signature::RSA(RSASignature::new(encoded.to_vec()))
            }
            SignatureAlgorithm::RSA_PKCS1_2048_8192_SHA512 => {
                Signature::RSA(RSASignature::new(encoded.to_vec()))
            }
            SignatureAlgorithm::RSA_PKCS1_3072_8192_SHA384 => {
                Signature::RSA(RSASignature::new(encoded.to_vec()))
            }
        };
        Ok(signature)
    }

    fn as_ecdsa(&self) -> Result<&ECDSASignature, Error> {
        match self {
            Signature::ECDSA(signature) => Ok(signature),
            _ => bail!(CryptoError::InvalidSignature),
        }
    }

    fn as_eddsa(&self) -> Result<&EdDSASignature, Error> {
        match self {
            Signature::EdDSA(signature) => Ok(signature),
            _ => bail!(CryptoError::InvalidSignature),
        }
    }

    fn as_rsa(&self) -> Result<&RSASignature, Error> {
        match self {
            Signature::RSA(signature) => Ok(signature),
            _ => bail!(CryptoError::InvalidSignature),
        }
    }
}

#[derive(Debug)]
pub enum SignatureState {
    ECDSA(ECDSASignatureState),
    EdDSA(EdDSASignatureState),
    RSA(RSASignatureState),
}

#[derive(Debug, Clone)]
pub struct ExclusiveSignatureState {
    state: Arc<SignatureState>,
}

impl ExclusiveSignatureState {
    fn new(signature_state: SignatureState) -> Self {
        ExclusiveSignatureState {
            state: Arc::new(signature_state),
        }
    }

    fn open(kp_handle: Handle) -> Result<Handle, Error> {
        let kp = WASI_CRYPTO_CTX.signature_keypair_manager.get(kp_handle)?;
        let signature_state = match kp {
            SignatureKeyPair::ECDSA(kp) => {
                ExclusiveSignatureState::new(SignatureState::ECDSA(ECDSASignatureState::new(kp)))
            }
            SignatureKeyPair::EdDSA(kp) => {
                ExclusiveSignatureState::new(SignatureState::EdDSA(EdDSASignatureState::new(kp)))
            }
            SignatureKeyPair::RSA(kp) => {
                ExclusiveSignatureState::new(SignatureState::RSA(RSASignatureState::new(kp)))
            }
        };
        let handle = WASI_CRYPTO_CTX
            .signature_state_manager
            .register(signature_state)?;
        Ok(handle)
    }

    fn update(&mut self, input: &[u8]) -> Result<(), Error> {
        match self.state.as_ref() {
            SignatureState::ECDSA(state) => state.update(input),
            SignatureState::EdDSA(state) => state.update(input),
            SignatureState::RSA(state) => state.update(input),
        }
    }

    fn sign(&mut self) -> Result<Signature, Error> {
        let signature = match self.state.as_ref() {
            SignatureState::ECDSA(state) => Signature::ECDSA(state.sign()?),
            SignatureState::EdDSA(state) => Signature::EdDSA(state.sign()?),
            SignatureState::RSA(state) => Signature::RSA(state.sign()?),
        };
        Ok(signature)
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SignatureEncoding {
    Raw = 1,
    Hex = 2,
    Base64Original = 3,
    Base64OriginalNoPadding = 4,
    Base64URLSafe = 5,
    Base64URLSafeNoPadding = 6,
    DER = 7,
}

#[derive(Debug)]
pub enum SignatureVerificationState {
    ECDSA(ECDSASignatureVerificationState),
    EdDSA(EdDSASignatureVerificationState),
    RSA(RSASignatureVerificationState),
}

#[derive(Debug, Clone)]
pub struct ExclusiveSignatureVerificationState {
    state: Arc<SignatureVerificationState>,
}

impl ExclusiveSignatureVerificationState {
    fn new(signature_verification_state: SignatureVerificationState) -> Self {
        ExclusiveSignatureVerificationState {
            state: Arc::new(signature_verification_state),
        }
    }

    fn open(pk_handle: Handle) -> Result<Handle, Error> {
        let pk = WASI_CRYPTO_CTX.signature_publickey_manager.get(pk_handle)?;
        let signature_verification_state = match pk {
            SignaturePublicKey::ECDSA(pk) => ExclusiveSignatureVerificationState::new(
                SignatureVerificationState::ECDSA(ECDSASignatureVerificationState::new(pk)),
            ),
            SignaturePublicKey::EdDSA(pk) => ExclusiveSignatureVerificationState::new(
                SignatureVerificationState::EdDSA(EdDSASignatureVerificationState::new(pk)),
            ),
            SignaturePublicKey::RSA(pk) => ExclusiveSignatureVerificationState::new(
                SignatureVerificationState::RSA(RSASignatureVerificationState::new(pk)),
            ),
        };
        let handle = WASI_CRYPTO_CTX
            .signature_verification_state_manager
            .register(signature_verification_state)?;
        Ok(handle)
    }

    fn update(&mut self, input: &[u8]) -> Result<(), Error> {
        match self.state.as_ref() {
            SignatureVerificationState::ECDSA(state) => state.update(input),
            SignatureVerificationState::EdDSA(state) => state.update(input),
            SignatureVerificationState::RSA(state) => state.update(input),
        }
    }

    fn verify(&self, signature_handle: Handle) -> Result<(), Error> {
        let signature = WASI_CRYPTO_CTX.signature_manager.get(signature_handle)?;
        match self.state.as_ref() {
            SignatureVerificationState::ECDSA(state) => state.verify(signature.as_ecdsa()?),
            SignatureVerificationState::EdDSA(state) => state.verify(signature.as_eddsa()?),
            SignatureVerificationState::RSA(state) => state.verify(signature.as_rsa()?),
        }
    }
}

pub fn signature_export(
    signature_handle: Handle,
    encoding: SignatureEncoding,
) -> Result<Vec<u8>, Error> {
    match encoding {
        SignatureEncoding::Raw => {}
        _ => bail!(CryptoError::NotAvailable),
    }
    let signature = WASI_CRYPTO_CTX.signature_manager.get(signature_handle)?;
    Ok(signature.as_ref().to_vec())
}

pub fn signature_import(
    op_handle: Handle,
    encoding: SignatureEncoding,
    encoded: &[u8],
) -> Result<Handle, Error> {
    let signature_op = WASI_CRYPTO_CTX.signature_op_manager.get(op_handle)?;
    let signature = match encoding {
        SignatureEncoding::Raw => Signature::from_raw(signature_op.alg(), encoded)?,
        _ => bail!(CryptoError::NotAvailable),
    };
    let handle = WASI_CRYPTO_CTX.signature_manager.register(signature)?;
    Ok(handle)
}

pub fn signature_state_open(kp_handle: Handle) -> Result<Handle, Error> {
    ExclusiveSignatureState::open(kp_handle)
}

pub fn signature_state_update(state_handle: Handle, input: &[u8]) -> Result<(), Error> {
    let mut state = WASI_CRYPTO_CTX.signature_state_manager.get(state_handle)?;
    state.update(input)
}

pub fn signature_state_sign(state_handle: Handle) -> Result<Handle, Error> {
    let mut state = WASI_CRYPTO_CTX.signature_state_manager.get(state_handle)?;
    let signature = state.sign()?;
    let handle = WASI_CRYPTO_CTX.signature_manager.register(signature)?;
    Ok(handle)
}

pub fn signature_state_close(handle: Handle) -> Result<(), Error> {
    WASI_CRYPTO_CTX.signature_state_manager.close(handle)
}

pub fn signature_verification_state_open(pk_handle: Handle) -> Result<Handle, Error> {
    ExclusiveSignatureVerificationState::open(pk_handle)
}

pub fn signature_verification_state_update(
    verification_state_handle: Handle,
    input: &[u8],
) -> Result<(), Error> {
    let mut state = WASI_CRYPTO_CTX
        .signature_verification_state_manager
        .get(verification_state_handle)?;
    state.update(input)
}

pub fn signature_verification_state_verify(
    verification_state_handle: Handle,
    signature_handle: Handle,
) -> Result<(), Error> {
    let state = WASI_CRYPTO_CTX
        .signature_verification_state_manager
        .get(verification_state_handle)?;
    state.verify(signature_handle)
}

pub fn signature_verification_state_close(handle: Handle) -> Result<(), Error> {
    WASI_CRYPTO_CTX
        .signature_verification_state_manager
        .close(handle)
}

pub fn signature_close(handle: Handle) -> Result<(), Error> {
    WASI_CRYPTO_CTX.signature_manager.close(handle)
}
