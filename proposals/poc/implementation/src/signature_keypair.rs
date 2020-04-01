use std::convert::TryInto;

use super::array_output::*;
use super::ecdsa::*;
use super::eddsa::*;
use super::error::*;
use super::handles::*;
use super::rsa::*;
use super::signature_op::*;
use super::signature_publickey::*;
use super::types as guest_types;
use super::{CryptoCtx, HandleManagers, WasiCryptoCtx};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Version(u64);

impl Version {
    pub const UNSPECIFIED: Version = Version(0xff00_0000_0000_0000);
    pub const LATEST: Version = Version(0xff00_0000_0000_0000);
    pub const ALL: Version = Version(0xff00_0000_0000_0000);
}

impl From<guest_types::Version> for Version {
    fn from(version: guest_types::Version) -> Self {
        Version(version.into())
    }
}

impl From<Version> for guest_types::Version {
    fn from(version: Version) -> Self {
        version.into()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KeyPairEncoding {
    Raw,
    Pkcs8,
    Der,
    Pem,
}

impl From<guest_types::KeypairEncoding> for KeyPairEncoding {
    fn from(encoding: guest_types::KeypairEncoding) -> Self {
        match encoding {
            guest_types::KeypairEncoding::Raw => KeyPairEncoding::Raw,
            guest_types::KeypairEncoding::Pkcs8 => KeyPairEncoding::Pkcs8,
            guest_types::KeypairEncoding::Der => KeyPairEncoding::Der,
            guest_types::KeypairEncoding::Pem => KeyPairEncoding::Pem,
        }
    }
}

#[derive(Clone, Debug)]
pub enum SignatureKeyPair {
    Ecdsa(EcdsaSignatureKeyPair),
    Eddsa(EddsaSignatureKeyPair),
    Rsa(RsaSignatureKeyPair),
}

impl SignatureKeyPair {
    fn export(&self, encoding: KeyPairEncoding) -> Result<Vec<u8>, CryptoError> {
        let encoded = match encoding {
            KeyPairEncoding::Pkcs8 => match self {
                SignatureKeyPair::Ecdsa(kp) => kp.as_pkcs8()?.to_vec(),
                SignatureKeyPair::Eddsa(kp) => kp.as_pkcs8()?.to_vec(),
                SignatureKeyPair::Rsa(kp) => kp.as_pkcs8()?.to_vec(),
            },
            _ => bail!(CryptoError::UnsupportedEncoding),
        };
        Ok(encoded)
    }

    fn generate(
        handles: &HandleManagers,
        kp_builder_handle: Handle,
    ) -> Result<Handle, CryptoError> {
        let kp_builder = handles.signature_keypair_builder.get(kp_builder_handle)?;
        let handle = match kp_builder {
            SignatureKeyPairBuilder::Ecdsa(kp_builder) => kp_builder.generate(handles)?,
            SignatureKeyPairBuilder::Eddsa(kp_builder) => kp_builder.generate(handles)?,
            SignatureKeyPairBuilder::Rsa(kp_builder) => kp_builder.generate(handles)?,
        };
        Ok(handle)
    }

    fn import(
        handles: &HandleManagers,
        kp_builder_handle: Handle,
        encoded: &[u8],
        encoding: KeyPairEncoding,
    ) -> Result<Handle, CryptoError> {
        let kp_builder = handles.signature_keypair_builder.get(kp_builder_handle)?;
        let handle = match kp_builder {
            SignatureKeyPairBuilder::Ecdsa(kp_builder) => {
                kp_builder.import(handles, encoded, encoding)?
            }
            SignatureKeyPairBuilder::Eddsa(kp_builder) => {
                kp_builder.import(handles, encoded, encoding)?
            }
            SignatureKeyPairBuilder::Rsa(kp_builder) => {
                kp_builder.import(handles, encoded, encoding)?
            }
        };
        Ok(handle)
    }

    fn public_key(&self, handles: &HandleManagers) -> Result<Handle, CryptoError> {
        let pk = match self {
            SignatureKeyPair::Ecdsa(kp) => {
                let raw_pk = kp.raw_public_key();
                SignaturePublicKey::Ecdsa(EcdsaSignaturePublicKey::from_raw(kp.alg, raw_pk)?)
            }
            SignatureKeyPair::Eddsa(kp) => {
                let raw_pk = kp.raw_public_key();
                SignaturePublicKey::Eddsa(EddsaSignaturePublicKey::from_raw(kp.alg, raw_pk)?)
            }
            SignatureKeyPair::Rsa(kp) => {
                let raw_pk = kp.raw_public_key();
                SignaturePublicKey::Rsa(RsaSignaturePublicKey::from_raw(kp.alg, raw_pk)?)
            }
        };
        let handle = handles.signature_publickey.register(pk)?;
        Ok(handle)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SignatureKeyPairBuilder {
    Ecdsa(EcdsaSignatureKeyPairBuilder),
    Eddsa(EddsaSignatureKeyPairBuilder),
    Rsa(RsaSignatureKeyPairBuilder),
}

impl SignatureKeyPairBuilder {
    fn open(handles: &HandleManagers, op_handle: Handle) -> Result<Handle, CryptoError> {
        let signature_op = handles.signature_op.get(op_handle)?;
        let kp_builder = match signature_op {
            SignatureOp::Ecdsa(_) => SignatureKeyPairBuilder::Ecdsa(
                EcdsaSignatureKeyPairBuilder::new(signature_op.alg()),
            ),
            SignatureOp::Eddsa(_) => SignatureKeyPairBuilder::Eddsa(
                EddsaSignatureKeyPairBuilder::new(signature_op.alg()),
            ),
            SignatureOp::Rsa(_) => {
                SignatureKeyPairBuilder::Rsa(RsaSignatureKeyPairBuilder::new(signature_op.alg()))
            }
        };
        let handle = handles.signature_keypair_builder.register(kp_builder)?;
        Ok(handle)
    }
}

impl CryptoCtx {
    pub fn signature_keypair_builder_open(&self, op_handle: Handle) -> Result<Handle, CryptoError> {
        SignatureKeyPairBuilder::open(&self.handles, op_handle)
    }

    pub fn signature_keypair_builder_close(
        &self,
        kp_builder_handle: Handle,
    ) -> Result<(), CryptoError> {
        self.handles
            .signature_keypair_builder
            .close(kp_builder_handle)
    }

    pub fn signature_keypair_generate(
        &self,
        kp_builder_handle: Handle,
    ) -> Result<Handle, CryptoError> {
        SignatureKeyPair::generate(&self.handles, kp_builder_handle)
    }

    pub fn signature_keypair_import(
        &self,
        kp_builder_handle: Handle,
        encoded: &[u8],
        encoding: KeyPairEncoding,
    ) -> Result<Handle, CryptoError> {
        SignatureKeyPair::import(&self.handles, kp_builder_handle, encoded, encoding)
    }

    pub fn signature_keypair_from_id(
        &self,
        _kp_builder_handle: Handle,
        _kp_id: &[u8],
        _kp_version: Version,
    ) -> Result<Handle, CryptoError> {
        bail!(CryptoError::UnsupportedFeature)
    }

    pub fn signature_keypair_id(
        &self,
        kp_handle: Handle,
    ) -> Result<(Vec<u8>, Version), CryptoError> {
        let _kp = self.handles.signature_keypair.get(kp_handle)?;
        bail!(CryptoError::UnsupportedFeature)
    }

    pub fn signature_keypair_invalidate(
        &self,
        _kp_builder_handle: Handle,
        _kp_id: &[u8],
        _kp_version: Version,
    ) -> Result<(), CryptoError> {
        bail!(CryptoError::UnsupportedFeature)
    }

    pub fn signature_keypair_export(
        &self,
        kp_handle: Handle,
        encoding: KeyPairEncoding,
    ) -> Result<Handle, CryptoError> {
        let kp = self.handles.signature_keypair.get(kp_handle)?;
        let encoded = kp.export(encoding)?;
        let array_output_handle = ArrayOutput::register(&self.handles, encoded)?;
        Ok(array_output_handle)
    }

    pub fn signature_keypair_publickey(&self, kp_handle: Handle) -> Result<Handle, CryptoError> {
        let kp = self.handles.signature_keypair.get(kp_handle)?;
        let handle = kp.public_key(&self.handles)?;
        Ok(handle)
    }

    pub fn signature_keypair_close(&self, kp_handle: Handle) -> Result<(), CryptoError> {
        self.handles.signature_keypair.close(kp_handle)
    }
}

impl WasiCryptoCtx {
    pub fn signature_keypair_builder_open(
        &self,
        op_handle: guest_types::SignatureOp,
    ) -> Result<guest_types::SignatureKeypairBuilder, CryptoError> {
        Ok(self
            .ctx
            .signature_keypair_builder_open(op_handle.into())?
            .into())
    }

    pub fn signature_keypair_builder_close(
        &self,
        kp_builder_handle: guest_types::SignatureKeypairBuilder,
    ) -> Result<(), CryptoError> {
        self.ctx
            .signature_keypair_builder_close(kp_builder_handle.into())
    }

    pub fn signature_keypair_generate(
        &self,
        kp_builder_handle: guest_types::SignatureKeypairBuilder,
    ) -> Result<guest_types::SignatureKeypair, CryptoError> {
        Ok(self
            .ctx
            .signature_keypair_generate(kp_builder_handle.into())?
            .into())
    }

    pub fn signature_keypair_import(
        &self,
        kp_builder_handle: guest_types::SignatureKeypairBuilder,
        encoded_ptr: &wiggle::GuestPtr<'_, u8>,
        encoded_len: guest_types::Size,
        encoding: guest_types::KeypairEncoding,
    ) -> Result<guest_types::SignatureKeypair, CryptoError> {
        let mut guest_borrow = wiggle::GuestBorrows::new();
        let encoded: &[u8] = unsafe {
            &*encoded_ptr
                .as_array(encoded_len as _)
                .as_raw(&mut guest_borrow)?
        };
        Ok(self
            .ctx
            .signature_keypair_import(kp_builder_handle.into(), encoded, encoding.into())?
            .into())
    }

    pub fn signature_keypair_from_id(
        &self,
        kp_builder_handle: guest_types::SignatureKeypairBuilder,
        kp_id_ptr: &wiggle::GuestPtr<'_, u8>,
        kp_id_len: guest_types::Size,
        kp_version: guest_types::Version,
    ) -> Result<guest_types::SignatureKeypair, CryptoError> {
        let mut guest_borrow = wiggle::GuestBorrows::new();
        let kp_id: &[u8] = unsafe {
            &*kp_id_ptr
                .as_array(kp_id_len as _)
                .as_raw(&mut guest_borrow)?
        };
        Ok(self
            .ctx
            .signature_keypair_from_id(kp_builder_handle.into(), kp_id, kp_version.into())?
            .into())
    }

    pub fn signature_keypair_id(
        &self,
        kp_handle: guest_types::SignatureKeypair,
        kp_id_ptr: &wiggle::GuestPtr<'_, u8>,
        kp_id_max_len: guest_types::Size,
    ) -> Result<(guest_types::Size, guest_types::Version), CryptoError> {
        let mut guest_borrow = wiggle::GuestBorrows::new();
        let kp_id_buf: &mut [u8] = unsafe {
            &mut *kp_id_ptr
                .as_array(kp_id_max_len as _)
                .as_raw(&mut guest_borrow)?
        };
        let (kp_id, version) = self.ctx.signature_keypair_id(kp_handle.into())?;
        ensure!(kp_id.len() <= kp_id_buf.len(), CryptoError::Overflow);
        kp_id_buf.copy_from_slice(&kp_id);
        Ok((kp_id.len().try_into()?, version.into()))
    }

    pub fn signature_keypair_invalidate(
        &self,
        kp_builder_handle: guest_types::SignatureKeypairBuilder,
        kp_id_ptr: &wiggle::GuestPtr<'_, u8>,
        kp_id_len: guest_types::Size,
        kp_version: guest_types::Version,
    ) -> Result<(), CryptoError> {
        let mut guest_borrow = wiggle::GuestBorrows::new();
        let kp_id: &[u8] = unsafe {
            &*kp_id_ptr
                .as_array(kp_id_len as _)
                .as_raw(&mut guest_borrow)?
        };
        Ok(self
            .ctx
            .signature_keypair_invalidate(kp_builder_handle.into(), kp_id, kp_version.into())?
            .into())
    }

    pub fn signature_keypair_export(
        &self,
        kp_handle: guest_types::SignatureKeypair,
        encoding: guest_types::KeypairEncoding,
    ) -> Result<guest_types::ArrayOutput, CryptoError> {
        Ok(self
            .ctx
            .signature_keypair_export(kp_handle.into(), encoding.into())?
            .into())
    }

    pub fn signature_keypair_publickey(
        &self,
        kp_handle: guest_types::SignatureKeypair,
    ) -> Result<guest_types::SignaturePublickey, CryptoError> {
        Ok(self
            .ctx
            .signature_keypair_publickey(kp_handle.into())?
            .into())
    }

    pub fn signature_keypair_close(
        &self,
        kp_handle: guest_types::SignatureKeypair,
    ) -> Result<(), CryptoError> {
        Ok(self.ctx.signature_keypair_close(kp_handle.into())?.into())
    }
}
