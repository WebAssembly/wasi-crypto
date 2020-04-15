use crate::error::*;
use crate::handles::*;
use crate::types as guest_types;
use crate::version::Version;
use crate::{CryptoCtx, WasiCryptoCtx};

impl CryptoCtx {
    pub fn key_manager_open(&self, _options: Option<Handle>) -> Result<Handle, CryptoError> {
        bail!(CryptoError::UnsupportedFeature)
    }

    pub fn key_manager_close(&self, _key_manager_handle: Handle) -> Result<(), CryptoError> {
        bail!(CryptoError::UnsupportedFeature)
    }

    pub fn key_manager_invalidate(
        &self,
        _key_manager_handle: Handle,
        _key_id: &[u8],
        _key_version: Version,
    ) -> Result<(), CryptoError> {
        bail!(CryptoError::UnsupportedFeature)
    }
}

impl WasiCryptoCtx {
    pub fn key_manager_open(
        &self,
        options_handle: &guest_types::OptOptions,
    ) -> Result<guest_types::KeyManager, CryptoError> {
        let options_handle = match *options_handle {
            guest_types::OptOptions::Some(options_handle) => Some(options_handle),
            guest_types::OptOptions::None => None,
        };
        Ok(self
            .ctx
            .key_manager_open(options_handle.map(Into::into))?
            .into())
    }

    pub fn key_manager_close(
        &self,
        key_manager_handle: guest_types::KeyManager,
    ) -> Result<(), CryptoError> {
        self.ctx.key_manager_close(key_manager_handle.into())
    }

    pub fn key_manager_invalidate(
        &self,
        key_manager_handle: guest_types::KeyManager,
        key_id_ptr: &wiggle::GuestPtr<'_, u8>,
        key_id_len: guest_types::Size,
        key_version: guest_types::Version,
    ) -> Result<(), CryptoError> {
        let mut guest_borrow = wiggle::GuestBorrows::new();
        let key_id: &[u8] = unsafe {
            &*key_id_ptr
                .as_array(key_id_len as _)
                .as_raw(&mut guest_borrow)?
        };
        Ok(self
            .ctx
            .key_manager_invalidate(key_manager_handle.into(), key_id, key_version.into())?
            .into())
    }
}
