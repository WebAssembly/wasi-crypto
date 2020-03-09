use super::ecdsa::*;
use super::eddsa::*;
use super::error::*;
use super::handles::*;
use super::rsa::*;
use super::signature_op::*;
use super::WASI_CRYPTO_CTX;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u16)]
pub enum PublicKeyEncoding {
    Raw = 1,
    Hex = 2,
    Base64Original = 3,
    Base64OriginalNoPadding = 4,
    Base64URLSafe = 5,
    Base64URLSafeNoPadding = 6,
}

#[derive(Clone, Debug)]
pub enum SignaturePublicKey {
    ECDSA(ECDSASignaturePublicKey),
    EdDSA(EdDSASignaturePublicKey),
    RSA(RSASignaturePublicKey),
}

impl SignaturePublicKey {
    fn import(
        signature_op: Handle,
        encoded: &[u8],
        encoding: PublicKeyEncoding,
    ) -> Result<Handle, Error> {
        match encoding {
            PublicKeyEncoding::Raw => {}
            _ => bail!(CryptoError::NotAvailable),
        }
        let signature_op = WASI_CRYPTO_CTX.signature_op_manager.get(signature_op)?;
        let pk =
            match signature_op {
                SignatureOp::ECDSA(_) => SignaturePublicKey::ECDSA(
                    ECDSASignaturePublicKey::from_raw(signature_op.alg(), encoded)?,
                ),
                SignatureOp::EdDSA(_) => SignaturePublicKey::EdDSA(
                    EdDSASignaturePublicKey::from_raw(signature_op.alg(), encoded)?,
                ),
                SignatureOp::RSA(_) => SignaturePublicKey::RSA(RSASignaturePublicKey::from_raw(
                    signature_op.alg(),
                    encoded,
                )?),
            };
        let handle = WASI_CRYPTO_CTX.signature_publickey_manager.register(pk)?;
        Ok(handle)
    }

    fn export(pk: Handle, encoding: PublicKeyEncoding) -> Result<Vec<u8>, Error> {
        match encoding {
            PublicKeyEncoding::Raw => {}
            _ => bail!(CryptoError::NotAvailable),
        }
        let pk = WASI_CRYPTO_CTX.signature_publickey_manager.get(pk)?;
        let raw_pk = match pk {
            SignaturePublicKey::ECDSA(pk) => pk.as_raw()?.to_vec(),
            SignaturePublicKey::EdDSA(pk) => pk.as_raw()?.to_vec(),
            SignaturePublicKey::RSA(pk) => pk.as_raw()?.to_vec(),
        };
        Ok(raw_pk)
    }
}

pub fn signature_publickey_import(
    signature_op: Handle,
    encoded: &[u8],
    encoding: PublicKeyEncoding,
) -> Result<Handle, Error> {
    SignaturePublicKey::import(signature_op, encoded, encoding)
}

pub fn signature_publickey_export(
    pk: Handle,
    encoding: PublicKeyEncoding,
) -> Result<Vec<u8>, Error> {
    SignaturePublicKey::export(pk, encoding)
}

pub fn signature_publickey_close(handle: Handle) -> Result<(), Error> {
    WASI_CRYPTO_CTX.signature_publickey_manager.close(handle)
}
