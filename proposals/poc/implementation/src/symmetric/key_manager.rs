use crate::error::*;
use crate::handles::*;
use crate::types as guest_types;
use crate::version::Version;
use crate::{CryptoCtx, WasiCryptoCtx};

impl CryptoCtx {
    pub fn symmetric_managed_key_generate(
        &self,
        _key_manager_handle: Handle,
        _alg_str: &str,
        _options_handle: Option<Handle>,
    ) -> Result<Handle, CryptoError> {
        bail!(CryptoError::UnsupportedFeature)
    }

    pub fn symmetric_key_from_id(
        &self,
        _key_manager_handle: Handle,
        _symmetric_key_id: &[u8],
        _symmetric_key_version: Version,
    ) -> Result<Handle, CryptoError> {
        bail!(CryptoError::UnsupportedFeature)
    }
}

impl WasiCryptoCtx {
    pub fn symmetric_managed_key_generate(
        &self,
        key_manager_handle: guest_types::KeyManager,
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
            .symmetric_managed_key_generate(
                key_manager_handle.into(),
                alg_str,
                options_handle.map(Into::into),
            )?
            .into())
    }

    pub fn symmetric_key_from_id(
        &self,
        key_manager_handle: guest_types::KeyManager,
        symmetric_key_id_ptr: &wiggle::GuestPtr<'_, u8>,
        symmetric_key_id_len: guest_types::Size,
        symmetric_key_version: guest_types::Version,
    ) -> Result<guest_types::SymmetricKey, CryptoError> {
        let mut guest_borrow = wiggle::GuestBorrows::new();
        let symmetric_key_id: &[u8] = unsafe {
            &*symmetric_key_id_ptr
                .as_array(symmetric_key_id_len as _)
                .as_raw(&mut guest_borrow)?
        };
        Ok(self
            .ctx
            .symmetric_key_from_id(
                key_manager_handle.into(),
                symmetric_key_id,
                symmetric_key_version.into(),
            )?
            .into())
    }
}
