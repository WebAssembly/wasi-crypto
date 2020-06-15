mod ecdsa;
mod eddsa;
mod keypair;
mod publickey;
mod rsa;
mod secretkey;
mod signature;
mod wasi_glue;

use crate::error::*;
use crate::options::*;

pub use keypair::*;
pub use publickey::*;
pub use secretkey::*;
pub use signature::*;

use std::any::Any;
use std::convert::TryFrom;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SignatureAlgorithm {
    ECDSA_P256_SHA256,
    ECDSA_P384_SHA384,
    Ed25519,
    RSA_PKCS1_2048_8192_SHA256,
    RSA_PKCS1_2048_8192_SHA384,
    RSA_PKCS1_2048_8192_SHA512,
    RSA_PKCS1_3072_8192_SHA384,
}

impl TryFrom<&str> for SignatureAlgorithm {
    type Error = CryptoError;

    fn try_from(alg_str: &str) -> Result<Self, CryptoError> {
        match alg_str {
            "ECDSA_P256_SHA256" => Ok(SignatureAlgorithm::ECDSA_P256_SHA256),
            "ECDSA_P384_SHA384" => Ok(SignatureAlgorithm::ECDSA_P384_SHA384),
            "Ed25519" => Ok(SignatureAlgorithm::Ed25519),
            "RSA_PKCS1_2048_8192_SHA256" => Ok(SignatureAlgorithm::RSA_PKCS1_2048_8192_SHA256),
            "RSA_PKCS1_2048_8192_SHA384" => Ok(SignatureAlgorithm::RSA_PKCS1_2048_8192_SHA384),
            "RSA_PKCS1_2048_8192_SHA512" => Ok(SignatureAlgorithm::RSA_PKCS1_2048_8192_SHA512),
            "RSA_PKCS1_3072_8192_SHA384" => Ok(SignatureAlgorithm::RSA_PKCS1_3072_8192_SHA384),
            _ => bail!(CryptoError::UnsupportedAlgorithm),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct SignatureOptions {}

impl OptionsLike for SignatureOptions {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn set(&mut self, _name: &str, _value: &[u8]) -> Result<(), CryptoError> {
        bail!(CryptoError::UnsupportedOption)
    }

    fn set_u64(&mut self, _name: &str, _value: u64) -> Result<(), CryptoError> {
        bail!(CryptoError::UnsupportedOption)
    }
}

#[test]
fn test_signatures() {
    use crate::{AlgorithmType, CryptoCtx};

    let ctx = CryptoCtx::new();

    let kp_handle = ctx
        .keypair_generate(AlgorithmType::Signatures, "ECDSA_P256_SHA256", None)
        .unwrap();
    let pk_handle = ctx.keypair_publickey(kp_handle).unwrap();

    let state_handle = ctx.signature_state_open(kp_handle).unwrap();
    ctx.signature_state_update(state_handle, b"test").unwrap();
    let signature_handle = ctx.signature_state_sign(state_handle).unwrap();

    let verification_state_handle = ctx.signature_verification_state_open(pk_handle).unwrap();
    ctx.signature_verification_state_update(verification_state_handle, b"test")
        .unwrap();
    ctx.signature_verification_state_verify(verification_state_handle, signature_handle)
        .unwrap();

    ctx.signature_verification_state_close(verification_state_handle)
        .unwrap();
    ctx.signature_state_close(state_handle).unwrap();
    ctx.keypair_close(kp_handle).unwrap();
    ctx.publickey_close(pk_handle).unwrap();
    ctx.signature_close(signature_handle).unwrap();
}
