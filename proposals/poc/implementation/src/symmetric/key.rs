use super::*;
use crate::types as guest_types;
use crate::version::*;
use crate::{CryptoCtx, WasiCryptoCtx};

use std::convert::TryInto;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SymmetricKey {
    HmacSha2(HmacSha2SymmetricKey),
    Hkdf(HkdfSymmetricKey),
    AesGcm(AesGcmSymmetricKey),
}

impl SymmetricKey {
    pub fn alg(&self) -> SymmetricAlgorithm {
        match self {
            SymmetricKey::HmacSha2(key) => key.alg(),
            SymmetricKey::Hkdf(key) => key.alg(),
            SymmetricKey::AesGcm(key) => key.alg(),
        }
    }

    fn generate(
        alg_str: &str,
        options: Option<SymmetricOptions>,
    ) -> Result<SymmetricKey, CryptoError> {
        let alg = SymmetricAlgorithm::try_from(alg_str)?;
        let symmetric_key = match alg {
            SymmetricAlgorithm::HmacSha256 | SymmetricAlgorithm::HmacSha512 => {
                SymmetricKey::HmacSha2(HmacSha2SymmetricKey::generate(alg, options)?)
            }
            SymmetricAlgorithm::HkdfSha256Expand
            | SymmetricAlgorithm::HkdfSha512Expand
            | SymmetricAlgorithm::HkdfSha256Extract
            | SymmetricAlgorithm::HkdfSha512Extract => {
                SymmetricKey::Hkdf(HkdfSymmetricKey::generate(alg, options)?)
            }
            SymmetricAlgorithm::Aes128Gcm | SymmetricAlgorithm::Aes256Gcm => {
                SymmetricKey::AesGcm(AesGcmSymmetricKey::generate(alg, options)?)
            }
            _ => bail!(CryptoError::KeyNotSupported),
        };
        Ok(symmetric_key)
    }

    fn import(alg_str: &str, raw: &[u8]) -> Result<SymmetricKey, CryptoError> {
        let alg = SymmetricAlgorithm::try_from(alg_str)?;
        let symmetric_key = match alg {
            SymmetricAlgorithm::HmacSha256 | SymmetricAlgorithm::HmacSha512 => {
                SymmetricKey::HmacSha2(HmacSha2SymmetricKey::import(alg, raw)?)
            }
            SymmetricAlgorithm::HkdfSha256Expand
            | SymmetricAlgorithm::HkdfSha512Expand
            | SymmetricAlgorithm::HkdfSha256Extract
            | SymmetricAlgorithm::HkdfSha512Extract => {
                SymmetricKey::Hkdf(HkdfSymmetricKey::import(alg, raw)?)
            }
            SymmetricAlgorithm::Aes128Gcm | SymmetricAlgorithm::Aes256Gcm => {
                SymmetricKey::AesGcm(AesGcmSymmetricKey::import(alg, raw)?)
            }
            _ => bail!(CryptoError::KeyNotSupported),
        };
        Ok(symmetric_key)
    }

    pub fn as_raw(&self) -> Result<Vec<u8>, CryptoError> {
        let raw = match self {
            SymmetricKey::HmacSha2(key) => key.as_raw()?.to_vec(),
            SymmetricKey::Hkdf(key) => key.as_raw()?.to_vec(),
            SymmetricKey::AesGcm(key) => key.as_raw()?.to_vec(),
        };
        Ok(raw)
    }
}

impl CryptoCtx {
    pub fn symmetric_key_generate(
        &self,
        alg_str: &str,
        options_handle: Option<Handle>,
    ) -> Result<Handle, CryptoError> {
        let options = match options_handle {
            None => None,
            Some(options_handle) => {
                Some(self.handles.options.get(options_handle)?.into_symmetric()?)
            }
        };
        let symmetric_key = SymmetricKey::generate(alg_str, options)?;
        let handle = self.handles.symmetric_key.register(symmetric_key)?;
        Ok(handle)
    }

    pub fn symmetric_key_import(&self, alg_str: &str, raw: &[u8]) -> Result<Handle, CryptoError> {
        let symmetric_key = SymmetricKey::import(alg_str, raw)?;
        let handle = self.handles.symmetric_key.register(symmetric_key)?;
        Ok(handle)
    }

    pub fn symmetric_key_id(
        &self,
        symmetric_key_handle: Handle,
    ) -> Result<(Vec<u8>, Version), CryptoError> {
        let _symmetric_key = self.handles.symmetric_key.get(symmetric_key_handle)?;
        bail!(CryptoError::UnsupportedFeature)
    }

    pub fn symmetric_key_close(&self, symmetric_key_handle: Handle) -> Result<(), CryptoError> {
        self.handles.symmetric_key.close(symmetric_key_handle)
    }
}

impl WasiCryptoCtx {
    pub fn symmetric_key_generate(
        &self,
        alg_str: &wiggle::GuestPtr<'_, str>,
        options_handle: &guest_types::OptOptions,
    ) -> Result<guest_types::SymmetricKey, CryptoError> {
        let mut guest_borrow = wiggle::GuestBorrows::new();
        let alg_str: &str = unsafe { &*alg_str.as_raw(&mut guest_borrow)? };
        let options_handle = match *options_handle {
            guest_types::OptOptions::Some(options_handle) => Some(options_handle),
            guest_types::OptOptions::None => None,
        };
        Ok(self
            .ctx
            .symmetric_key_generate(alg_str, options_handle.map(Into::into))?
            .into())
    }

    pub fn symmetric_key_import(
        &self,
        alg_str: &wiggle::GuestPtr<'_, str>,
        raw_ptr: &wiggle::GuestPtr<'_, u8>,
        raw_len: guest_types::Size,
    ) -> Result<guest_types::SymmetricKey, CryptoError> {
        let mut guest_borrow = wiggle::GuestBorrows::new();
        let alg_str: &str = unsafe { &*alg_str.as_raw(&mut guest_borrow)? };
        let raw: &[u8] = unsafe { &*raw_ptr.as_array(raw_len as _).as_raw(&mut guest_borrow)? };
        Ok(self.ctx.symmetric_key_import(alg_str, raw)?.into())
    }

    pub fn symmetric_key_id(
        &self,
        symmetric_key_handle: guest_types::SymmetricKey,
        symmetric_key_id_ptr: &wiggle::GuestPtr<'_, u8>,
        symmetric_key_id_max_len: guest_types::Size,
    ) -> Result<(guest_types::Size, guest_types::Version), CryptoError> {
        let mut guest_borrow = wiggle::GuestBorrows::new();
        let key_id_buf: &mut [u8] = unsafe {
            &mut *symmetric_key_id_ptr
                .as_array(symmetric_key_id_max_len as _)
                .as_raw(&mut guest_borrow)?
        };
        let (key_id, version) = self.ctx.symmetric_key_id(symmetric_key_handle.into())?;
        ensure!(key_id.len() <= key_id_buf.len(), CryptoError::Overflow);
        key_id_buf.copy_from_slice(&key_id);
        Ok((key_id.len().try_into()?, version.into()))
    }

    pub fn symmetric_key_close(
        &self,
        key_handle: guest_types::SymmetricKey,
    ) -> Result<(), CryptoError> {
        Ok(self.ctx.symmetric_key_close(key_handle.into())?.into())
    }
}
