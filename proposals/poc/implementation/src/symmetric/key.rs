use super::*;
use crate::types as guest_types;
use crate::version::*;
use crate::{CryptoCtx, WasiCryptoCtx};

use parking_lot::{Mutex, MutexGuard};
use std::convert::TryInto;
use std::sync::Arc;

#[derive(Clone)]
pub struct SymmetricKey {
    inner: Arc<Mutex<Box<dyn SymmetricKeyLike>>>,
}

pub trait SymmetricKeyBuilder {
    fn generate(&self, options: Option<SymmetricOptions>) -> Result<SymmetricKey, CryptoError>;

    fn import(&self, raw: &[u8]) -> Result<SymmetricKey, CryptoError>;

    fn key_len(&self) -> Result<usize, CryptoError>;
}

impl SymmetricKey {
    pub fn new(symmetric_key_like: Box<dyn SymmetricKeyLike>) -> Self {
        SymmetricKey {
            inner: Arc::new(Mutex::new(symmetric_key_like)),
        }
    }

    pub fn inner(&self) -> MutexGuard<Box<dyn SymmetricKeyLike>> {
        self.inner.lock()
    }

    pub fn locked<T, U>(&self, mut f: T) -> U
    where
        T: FnMut(MutexGuard<Box<dyn SymmetricKeyLike>>) -> U,
    {
        f(self.inner())
    }

    pub fn alg(&self) -> SymmetricAlgorithm {
        self.inner().alg()
    }

    pub fn builder(alg_str: &str) -> Result<Box<dyn SymmetricKeyBuilder>, CryptoError> {
        let alg = SymmetricAlgorithm::try_from(alg_str)?;
        let builder = match alg {
            SymmetricAlgorithm::HmacSha256 | SymmetricAlgorithm::HmacSha512 => {
                HmacSha2SymmetricKeyBuilder::new(alg)
            }
            SymmetricAlgorithm::HkdfSha256Expand
            | SymmetricAlgorithm::HkdfSha256Extract
            | SymmetricAlgorithm::HkdfSha512Expand
            | SymmetricAlgorithm::HkdfSha512Extract => HkdfSymmetricKeyBuilder::new(alg),
            SymmetricAlgorithm::Aes128Gcm | SymmetricAlgorithm::Aes256Gcm => {
                AesGcmSymmetricKeyBuilder::new(alg)
            }
            SymmetricAlgorithm::Xoodyak128 | SymmetricAlgorithm::Xoodyak256 => {
                XoodyakSymmetricKeyBuilder::new(alg)
            }
            _ => bail!(CryptoError::InvalidOperation),
        };
        Ok(builder)
    }

    fn generate(
        alg_str: &str,
        options: Option<SymmetricOptions>,
    ) -> Result<SymmetricKey, CryptoError> {
        let builder = Self::builder(alg_str)?;
        builder.generate(options)
    }

    fn import(alg_str: &str, raw: &[u8]) -> Result<SymmetricKey, CryptoError> {
        let builder = Self::builder(alg_str)?;
        builder.import(raw)
    }
}

pub trait SymmetricKeyLike: Sync + Send {
    fn as_any(&self) -> &dyn Any;
    fn alg(&self) -> SymmetricAlgorithm;
    fn as_raw(&self) -> Result<&[u8], CryptoError>;
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
