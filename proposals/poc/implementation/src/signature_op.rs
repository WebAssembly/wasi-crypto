use super::ecdsa::*;
use super::eddsa::*;
use super::error::*;
use super::handles::*;
use super::rsa::*;
use super::signature::*;
use super::WASI_CRYPTO_CTX;

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

    fn open(alg_str: &str) -> Result<Handle, Error> {
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
            _ => bail!(CryptoError::NotAvailable),
        };
        let handle = WASI_CRYPTO_CTX
            .signature_op_manager
            .register(signature_op)?;
        Ok(handle)
    }
}

pub fn signature_op_open(alg_str: &str) -> Result<Handle, Error> {
    SignatureOp::open(alg_str)
}

pub fn signature_op_close(handle: Handle) -> Result<(), Error> {
    WASI_CRYPTO_CTX.signature_op_manager.close(handle)
}
