use super::ecdsa::*;
use super::eddsa::*;
use super::publickey::*;
use super::rsa::*;
use super::*;
use crate::array_output::*;
use crate::error::*;
use crate::handles::*;
use crate::types as guest_types;
use crate::version::Version;
use crate::{CryptoCtx, WasiCryptoCtx};

use std::convert::{TryFrom, TryInto};

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
        alg_str: &str,
        options: Option<SignatureOptions>,
    ) -> Result<SignatureKeyPair, CryptoError> {
        let alg = SignatureAlgorithm::try_from(alg_str)?;
        let kp = match alg {
            SignatureAlgorithm::ECDSA_P256_SHA256 | SignatureAlgorithm::ECDSA_P384_SHA384 => {
                SignatureKeyPair::Ecdsa(EcdsaSignatureKeyPair::generate(alg, options)?)
            }
            SignatureAlgorithm::Ed25519 => {
                SignatureKeyPair::Eddsa(EddsaSignatureKeyPair::generate(alg, options)?)
            }
            SignatureAlgorithm::RSA_PKCS1_2048_8192_SHA256
            | SignatureAlgorithm::RSA_PKCS1_2048_8192_SHA384
            | SignatureAlgorithm::RSA_PKCS1_2048_8192_SHA512
            | SignatureAlgorithm::RSA_PKCS1_3072_8192_SHA384 => {
                SignatureKeyPair::Rsa(RsaSignatureKeyPair::generate(alg, options)?)
            }
        };
        Ok(kp)
    }

    fn import(
        alg_str: &str,
        encoded: &[u8],
        encoding: KeyPairEncoding,
    ) -> Result<SignatureKeyPair, CryptoError> {
        let alg = SignatureAlgorithm::try_from(alg_str)?;
        let kp = match alg {
            SignatureAlgorithm::ECDSA_P256_SHA256 | SignatureAlgorithm::ECDSA_P384_SHA384 => {
                SignatureKeyPair::Ecdsa(EcdsaSignatureKeyPair::import(alg, encoded, encoding)?)
            }
            SignatureAlgorithm::Ed25519 => {
                SignatureKeyPair::Eddsa(EddsaSignatureKeyPair::import(alg, encoded, encoding)?)
            }
            SignatureAlgorithm::RSA_PKCS1_2048_8192_SHA256
            | SignatureAlgorithm::RSA_PKCS1_2048_8192_SHA384
            | SignatureAlgorithm::RSA_PKCS1_2048_8192_SHA512
            | SignatureAlgorithm::RSA_PKCS1_3072_8192_SHA384 => {
                SignatureKeyPair::Rsa(RsaSignatureKeyPair::import(alg, encoded, encoding)?)
            }
        };
        Ok(kp)
    }

    fn public_key(&self) -> Result<SignaturePublicKey, CryptoError> {
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
        Ok(pk)
    }
}

impl CryptoCtx {
    pub fn signature_keypair_generate(
        &self,
        alg_str: &str,
        options_handle: Option<Handle>,
    ) -> Result<Handle, CryptoError> {
        let options = match options_handle {
            None => None,
            Some(options_handle) => Some(
                self.handles
                    .options
                    .get(options_handle)?
                    .into_signatures()?,
            ),
        };
        let kp = SignatureKeyPair::generate(alg_str, options)?;
        let handle = self.handles.signature_keypair.register(kp)?;
        Ok(handle)
    }

    pub fn signature_keypair_import(
        &self,
        alg_str: &str,
        encoded: &[u8],
        encoding: KeyPairEncoding,
    ) -> Result<Handle, CryptoError> {
        let kp = SignatureKeyPair::import(alg_str, encoded, encoding)?;
        let handle = self.handles.signature_keypair.register(kp)?;
        Ok(handle)
    }

    pub fn signature_keypair_id(
        &self,
        kp_handle: Handle,
    ) -> Result<(Vec<u8>, Version), CryptoError> {
        let _kp = self.handles.signature_keypair.get(kp_handle)?;
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
        let pk = kp.public_key()?;
        let handle = self.handles.signature_publickey.register(pk)?;
        Ok(handle)
    }

    pub fn signature_keypair_close(&self, kp_handle: Handle) -> Result<(), CryptoError> {
        self.handles.signature_keypair.close(kp_handle)
    }
}

impl WasiCryptoCtx {
    pub fn signature_keypair_generate(
        &self,
        alg_str: &wiggle::GuestPtr<'_, str>,
        options_handle: &guest_types::OptOptions,
    ) -> Result<guest_types::SignatureKeypair, CryptoError> {
        let mut guest_borrow = wiggle::GuestBorrows::new();
        let alg_str: &str = unsafe { &*alg_str.as_raw(&mut guest_borrow)? };
        let options_handle = match *options_handle {
            guest_types::OptOptions::Some(options_handle) => Some(options_handle),
            guest_types::OptOptions::None => None,
        };
        Ok(self
            .ctx
            .signature_keypair_generate(alg_str, options_handle.map(Into::into))?
            .into())
    }

    pub fn signature_keypair_import(
        &self,
        alg_str: &wiggle::GuestPtr<'_, str>,
        encoded_ptr: &wiggle::GuestPtr<'_, u8>,
        encoded_len: guest_types::Size,
        encoding: guest_types::KeypairEncoding,
    ) -> Result<guest_types::SignatureKeypair, CryptoError> {
        let mut guest_borrow = wiggle::GuestBorrows::new();
        let alg_str: &str = unsafe { &*alg_str.as_raw(&mut guest_borrow)? };
        let encoded: &[u8] = unsafe {
            &*encoded_ptr
                .as_array(encoded_len as _)
                .as_raw(&mut guest_borrow)?
        };
        Ok(self
            .ctx
            .signature_keypair_import(alg_str, encoded, encoding.into())?
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

#[derive(Copy, Clone, Debug)]
pub struct SignatureKeyPairManager;
