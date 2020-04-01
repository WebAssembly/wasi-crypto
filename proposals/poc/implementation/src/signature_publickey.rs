use super::array_output::*;
use super::ecdsa::*;
use super::eddsa::*;
use super::error::*;
use super::handles::*;
use super::rsa::*;
use super::signature_op::*;
use super::types as guest_types;
use super::{CryptoCtx, HandleManagers, WasiCryptoCtx};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PublicKeyEncoding {
    Raw,
    Hex,
    Base64Original,
    Base64OriginalNoPadding,
    Base64UrlSafe,
    Base64UrlSafeNoPadding,
}

impl From<guest_types::PublickeyEncoding> for PublicKeyEncoding {
    fn from(encoding: guest_types::PublickeyEncoding) -> Self {
        match encoding {
            guest_types::PublickeyEncoding::Raw => PublicKeyEncoding::Raw,
            guest_types::PublickeyEncoding::Hex => PublicKeyEncoding::Hex,
            guest_types::PublickeyEncoding::Base64Original => PublicKeyEncoding::Base64Original,
            guest_types::PublickeyEncoding::Base64OriginalNoPadding => {
                PublicKeyEncoding::Base64OriginalNoPadding
            }
            guest_types::PublickeyEncoding::Base64UrlSafe => PublicKeyEncoding::Base64UrlSafe,
            guest_types::PublickeyEncoding::Base64UrlSafeNoPadding => {
                PublicKeyEncoding::Base64UrlSafeNoPadding
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum SignaturePublicKey {
    Ecdsa(EcdsaSignaturePublicKey),
    Eddsa(EddsaSignaturePublicKey),
    Rsa(RsaSignaturePublicKey),
}

impl SignaturePublicKey {
    fn import(
        handles: &HandleManagers,
        signature_op: Handle,
        encoded: &[u8],
        encoding: PublicKeyEncoding,
    ) -> Result<Handle, CryptoError> {
        match encoding {
            PublicKeyEncoding::Raw => {}
            _ => bail!(CryptoError::UnsupportedEncoding),
        }
        let signature_op = handles.signature_op.get(signature_op)?;
        let pk =
            match signature_op {
                SignatureOp::Ecdsa(_) => SignaturePublicKey::Ecdsa(
                    EcdsaSignaturePublicKey::from_raw(signature_op.alg(), encoded)?,
                ),
                SignatureOp::Eddsa(_) => SignaturePublicKey::Eddsa(
                    EddsaSignaturePublicKey::from_raw(signature_op.alg(), encoded)?,
                ),
                SignatureOp::Rsa(_) => SignaturePublicKey::Rsa(RsaSignaturePublicKey::from_raw(
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
    ) -> Result<Vec<u8>, CryptoError> {
        match encoding {
            PublicKeyEncoding::Raw => {}
            _ => bail!(CryptoError::UnsupportedEncoding),
        }
        let pk = handles.signature_publickey.get(pk)?;
        let raw_pk = match pk {
            SignaturePublicKey::Ecdsa(pk) => pk.as_raw()?.to_vec(),
            SignaturePublicKey::Eddsa(pk) => pk.as_raw()?.to_vec(),
            SignaturePublicKey::Rsa(pk) => pk.as_raw()?.to_vec(),
        };
        Ok(raw_pk)
    }

    fn verify(_pk_handle: Handle) -> Result<(), CryptoError> {
        bail!(CryptoError::NotImplemented)
    }
}

impl CryptoCtx {
    pub fn signature_publickey_import(
        &self,
        signature_op: Handle,
        encoded: &[u8],
        encoding: PublicKeyEncoding,
    ) -> Result<Handle, CryptoError> {
        SignaturePublicKey::import(&self.handles, signature_op, encoded, encoding)
    }

    pub fn signature_publickey_export(
        &self,
        pk: Handle,
        encoding: PublicKeyEncoding,
    ) -> Result<Handle, CryptoError> {
        let encoded = SignaturePublicKey::export(&self.handles, pk, encoding)?;
        let array_output_handle = ArrayOutput::register(&self.handles, encoded)?;
        Ok(array_output_handle)
    }

    pub fn signature_publickey_verify(&self, pk: Handle) -> Result<(), CryptoError> {
        SignaturePublicKey::verify(pk)
    }

    pub fn signature_publickey_close(&self, pk: Handle) -> Result<(), CryptoError> {
        self.handles.signature_publickey.close(pk)
    }
}

impl WasiCryptoCtx {
    pub fn signature_publickey_import(
        &self,
        signature_op: guest_types::SignatureOp,
        encoded_ptr: &wiggle::GuestPtr<'_, u8>,
        encoded_len: guest_types::Size,
        encoding: guest_types::PublickeyEncoding,
    ) -> Result<guest_types::SignaturePublickey, CryptoError> {
        let mut guest_borrow = wiggle::GuestBorrows::new();
        let encoded: &[u8] = unsafe {
            &*encoded_ptr
                .as_array(encoded_len as _)
                .as_raw(&mut guest_borrow)?
        };
        Ok(self
            .ctx
            .signature_publickey_import(signature_op.into(), encoded, encoding.into())?
            .into())
    }

    pub fn signature_publickey_export(
        &self,
        pk: guest_types::SignaturePublickey,
        encoding: guest_types::PublickeyEncoding,
    ) -> Result<guest_types::ArrayOutput, CryptoError> {
        Ok(self
            .ctx
            .signature_publickey_export(pk.into(), encoding.into())?
            .into())
    }

    pub fn signature_publickey_verify(
        &self,
        pk: guest_types::SignaturePublickey,
    ) -> Result<(), CryptoError> {
        Ok(self.ctx.signature_publickey_verify(pk.into())?.into())
    }

    pub fn signature_publickey_close(
        &self,
        pk: guest_types::SignaturePublickey,
    ) -> Result<(), CryptoError> {
        Ok(self.ctx.signature_publickey_close(pk.into())?.into())
    }
}
