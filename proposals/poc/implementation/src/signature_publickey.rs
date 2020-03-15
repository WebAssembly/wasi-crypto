use super::array_output::*;
use super::ecdsa::*;
use super::eddsa::*;
use super::error::*;
use super::handles::*;
use super::rsa::*;
use super::signature_op::*;
use super::{HandleManagers, WasiCryptoCtx};

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
        handles: &HandleManagers,
        signature_op: Handle,
        encoded: &[u8],
        encoding: PublicKeyEncoding,
    ) -> Result<Handle, Error> {
        match encoding {
            PublicKeyEncoding::Raw => {}
            _ => bail!(CryptoError::UnsupportedEncoding),
        }
        let signature_op = handles.signature_op.get(signature_op)?;
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
        let handle = handles.signature_publickey.register(pk)?;
        Ok(handle)
    }

    fn export(
        handles: &HandleManagers,
        pk: Handle,
        encoding: PublicKeyEncoding,
    ) -> Result<Vec<u8>, Error> {
        match encoding {
            PublicKeyEncoding::Raw => {}
            _ => bail!(CryptoError::UnsupportedEncoding),
        }
        let pk = handles.signature_publickey.get(pk)?;
        let raw_pk = match pk {
            SignaturePublicKey::ECDSA(pk) => pk.as_raw()?.to_vec(),
            SignaturePublicKey::EdDSA(pk) => pk.as_raw()?.to_vec(),
            SignaturePublicKey::RSA(pk) => pk.as_raw()?.to_vec(),
        };
        Ok(raw_pk)
    }

    fn verify(_pk_handle: Handle) -> Result<(), Error> {
        bail!(CryptoError::UnsupportedOperation)
    }
}

impl WasiCryptoCtx {
    pub fn signature_publickey_import(
        &self,
        signature_op: Handle,
        encoded: &[u8],
        encoding: PublicKeyEncoding,
    ) -> Result<Handle, Error> {
        SignaturePublicKey::import(&self.handles, signature_op, encoded, encoding)
    }

    pub fn signature_publickey_export(
        &self,
        pk: Handle,
        encoding: PublicKeyEncoding,
    ) -> Result<Handle, Error> {
        let encoded = SignaturePublicKey::export(&self.handles, pk, encoding)?;
        let array_output_handle = ArrayOutput::register(&self.handles, encoded)?;
        Ok(array_output_handle)
    }

    pub fn signature_publickey_verify(&self, pk: Handle) -> Result<(), Error> {
        SignaturePublicKey::verify(pk)
    }

    pub fn signature_publickey_close(&self, handle: Handle) -> Result<(), Error> {
        self.handles.signature_publickey.close(handle)
    }
}
