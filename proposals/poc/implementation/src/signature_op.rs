use super::ecdsa::*;
use super::eddsa::*;
use super::error::*;
use super::handles::*;
use super::rsa::*;
use super::signature::*;
use super::types as guest_types;
use super::{CryptoCtx, HandleManagers, WasiCryptoCtx};

#[derive(Clone, Copy, Debug)]
pub enum SignatureOp {
    ECDSA(ECDSASignatureOp),
    EdDSA(EdDSASignatureOp),
    RSA(RSASignatureOp),
}

impl SignatureOp {
    pub fn alg(self) -> SignatureAlgorithm {
        match self {
            SignatureOp::ECDSA(op) => op.alg,
            SignatureOp::EdDSA(op) => op.alg,
            SignatureOp::RSA(op) => op.alg,
        }
    }

    fn open(handles: &HandleManagers, alg_str: &str) -> Result<Handle, CryptoError> {
        let signature_op = match alg_str {
            "ECDSA_P256_SHA256" => {
                SignatureOp::ECDSA(ECDSASignatureOp::new(SignatureAlgorithm::ECDSA_P256_SHA256))
            }
            "ECDSA_P384_SHA384" => {
                SignatureOp::ECDSA(ECDSASignatureOp::new(SignatureAlgorithm::ECDSA_P384_SHA384))
            }
            "Ed25519" => SignatureOp::EdDSA(EdDSASignatureOp::new(SignatureAlgorithm::Ed25519)),
            "RSA_PKCS1_2048_8192_SHA256" => SignatureOp::RSA(RSASignatureOp::new(
                SignatureAlgorithm::RSA_PKCS1_2048_8192_SHA256,
            )),
            "RSA_PKCS1_2048_8192_SHA384" => SignatureOp::RSA(RSASignatureOp::new(
                SignatureAlgorithm::RSA_PKCS1_2048_8192_SHA384,
            )),
            "RSA_PKCS1_2048_8192_SHA512" => SignatureOp::RSA(RSASignatureOp::new(
                SignatureAlgorithm::RSA_PKCS1_2048_8192_SHA512,
            )),
            "RSA_PKCS1_3072_8192_SHA384" => SignatureOp::RSA(RSASignatureOp::new(
                SignatureAlgorithm::RSA_PKCS1_3072_8192_SHA384,
            )),
            _ => bail!(CryptoError::UnsupportedAlgorithm),
        };
        let handle = handles.signature_op.register(signature_op)?;
        Ok(handle)
    }
}

impl CryptoCtx {
    pub fn signature_op_open(&self, alg_str: &str) -> Result<Handle, CryptoError> {
        SignatureOp::open(&self.handles, alg_str)
    }

    pub fn signature_op_close(&self, handle: Handle) -> Result<(), CryptoError> {
        self.handles.signature_op.close(handle)
    }
}

impl WasiCryptoCtx {
    pub fn signature_op_open(
        &self,
        alg_str: &wiggle_runtime::GuestPtr<'_, str>,
    ) -> Result<guest_types::SignatureOp, CryptoError> {
        let mut guest_borrow = wiggle_runtime::GuestBorrows::new();
        let alg_str: &str = unsafe { &*alg_str.as_raw(&mut guest_borrow)? };
        Ok(self.ctx.signature_op_open(alg_str)?.into())
    }

    pub fn signature_op_close(
        &self,
        op_handle: guest_types::SignatureOp,
    ) -> Result<(), CryptoError> {
        self.ctx.signature_op_close(op_handle.into())
    }
}
