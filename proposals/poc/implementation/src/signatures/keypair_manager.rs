use crate::error::*;
use crate::handles::*;
use crate::types as guest_types;
use crate::version::Version;
use crate::{CryptoCtx, WasiCryptoCtx};

impl CryptoCtx {
    pub fn signature_keypair_manager_open(
        &self,
        _options: Option<Handle>,
    ) -> Result<Handle, CryptoError> {
        bail!(CryptoError::UnsupportedFeature)
    }

    pub fn signature_keypair_manager_close(
        &self,
        _kp_manager_handle: Handle,
    ) -> Result<(), CryptoError> {
        bail!(CryptoError::UnsupportedFeature)
    }

    pub fn signature_managed_keypair_generate(
        &self,
        _kp_manager_handle: Handle,
        _alg_str: &str,
        _options_handle: Option<Handle>,
    ) -> Result<Handle, CryptoError> {
        bail!(CryptoError::UnsupportedFeature)
    }

    pub fn signature_keypair_from_id(
        &self,
        _kp_manager_handle: Handle,
        _kp_id: &[u8],
        _kp_version: Version,
    ) -> Result<Handle, CryptoError> {
        bail!(CryptoError::UnsupportedFeature)
    }

    pub fn signature_keypair_invalidate(
        &self,
        _kp_manager_handle: Handle,
        _kp_id: &[u8],
        _kp_version: Version,
    ) -> Result<(), CryptoError> {
        bail!(CryptoError::UnsupportedFeature)
    }
}

impl WasiCryptoCtx {
    pub fn signature_keypair_manager_open(
        &self,
        options_handle: &guest_types::OptOptions,
    ) -> Result<guest_types::SignatureKeypairManager, CryptoError> {
        let options_handle = match *options_handle {
            guest_types::OptOptions::Some(options_handle) => Some(options_handle),
            guest_types::OptOptions::None => None,
        };
        Ok(self
            .ctx
            .signature_keypair_manager_open(options_handle.map(Into::into))?
            .into())
    }

    pub fn signature_keypair_manager_close(
        &self,
        kp_manager_handle: guest_types::SignatureKeypairManager,
    ) -> Result<(), CryptoError> {
        self.ctx
            .signature_keypair_manager_close(kp_manager_handle.into())
    }

    pub fn signature_managed_keypair_generate(
        &self,
        kp_manager_handle: guest_types::SignatureKeypairManager,
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
            .signature_managed_keypair_generate(
                kp_manager_handle.into(),
                alg_str,
                options_handle.map(Into::into),
            )?
            .into())
    }

    pub fn signature_keypair_from_id(
        &self,
        kp_manager_handle: guest_types::SignatureKeypairManager,
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
            .signature_keypair_from_id(kp_manager_handle.into(), kp_id, kp_version.into())?
            .into())
    }

    pub fn signature_keypair_invalidate(
        &self,
        kp_manager_handle: guest_types::SignatureKeypairManager,
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
            .signature_keypair_invalidate(kp_manager_handle.into(), kp_id, kp_version.into())?
            .into())
    }
}
