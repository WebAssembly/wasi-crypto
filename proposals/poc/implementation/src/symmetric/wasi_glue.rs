use std::convert::TryInto;

use crate::error::*;
use crate::types as guest_types;
use crate::WasiCryptoCtx;

impl crate::wasi_ephemeral_crypto_symmetric::WasiEphemeralCryptoSymmetric for WasiCryptoCtx {
    // --- key_manager

    fn symmetric_key_generate_managed(
        &self,
        key_manager_handle: guest_types::KeyManager,
        alg_str: &wiggle::GuestPtr<'_, str>,
        options_handle: &guest_types::OptOptions,
    ) -> Result<guest_types::SymmetricKey, guest_types::CryptoErrno> {
        let mut guest_borrow = wiggle::GuestBorrows::new();
        let alg_str: &str = unsafe { &*alg_str.as_raw(&mut guest_borrow)? };
        let options_handle = match *options_handle {
            guest_types::OptOptions::Some(options_handle) => Some(options_handle),
            guest_types::OptOptions::None => None,
        };
        Ok(self
            .ctx
            .symmetric_key_generate_managed(
                key_manager_handle.into(),
                alg_str,
                options_handle.map(Into::into),
            )?
            .into())
    }

    fn symmetric_key_from_id(
        &self,
        key_manager_handle: guest_types::KeyManager,
        symmetric_key_id_ptr: &wiggle::GuestPtr<'_, u8>,
        symmetric_key_id_len: guest_types::Size,
        symmetric_key_version: guest_types::Version,
    ) -> Result<guest_types::SymmetricKey, guest_types::CryptoErrno> {
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

    // --- key

    fn symmetric_key_generate(
        &self,
        alg_str: &wiggle::GuestPtr<'_, str>,
        options_handle: &guest_types::OptOptions,
    ) -> Result<guest_types::SymmetricKey, guest_types::CryptoErrno> {
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

    fn symmetric_key_import(
        &self,
        alg_str: &wiggle::GuestPtr<'_, str>,
        raw_ptr: &wiggle::GuestPtr<'_, u8>,
        raw_len: guest_types::Size,
    ) -> Result<guest_types::SymmetricKey, guest_types::CryptoErrno> {
        let mut guest_borrow = wiggle::GuestBorrows::new();
        let alg_str: &str = unsafe { &*alg_str.as_raw(&mut guest_borrow)? };
        let raw: &[u8] = unsafe { &*raw_ptr.as_array(raw_len as _).as_raw(&mut guest_borrow)? };
        Ok(self.ctx.symmetric_key_import(alg_str, raw)?.into())
    }

    fn symmetric_key_export(
        &self,
        symmetric_key_handle: guest_types::SymmetricKey,
    ) -> Result<guest_types::ArrayOutput, guest_types::CryptoErrno> {
        Ok(self
            .ctx
            .symmetric_key_export(symmetric_key_handle.into())?
            .into())
    }

    fn symmetric_key_id(
        &self,
        symmetric_key_handle: guest_types::SymmetricKey,
        symmetric_key_id_ptr: &wiggle::GuestPtr<'_, u8>,
        symmetric_key_id_max_len: guest_types::Size,
    ) -> Result<(guest_types::Size, guest_types::Version), guest_types::CryptoErrno> {
        let mut guest_borrow = wiggle::GuestBorrows::new();
        let key_id_buf: &mut [u8] = unsafe {
            &mut *symmetric_key_id_ptr
                .as_array(symmetric_key_id_max_len as _)
                .as_raw(&mut guest_borrow)?
        };
        let (key_id, version) = self.ctx.symmetric_key_id(symmetric_key_handle.into())?;
        ensure!(
            key_id.len() <= key_id_buf.len(),
            CryptoError::Overflow.into()
        );
        key_id_buf.copy_from_slice(&key_id);
        Ok((key_id.len().try_into()?, version.into()))
    }

    fn symmetric_key_close(
        &self,
        key_handle: guest_types::SymmetricKey,
    ) -> Result<(), guest_types::CryptoErrno> {
        Ok(self.ctx.symmetric_key_close(key_handle.into())?.into())
    }

    // --- state

    fn symmetric_state_open(
        &self,
        alg_str: &wiggle::GuestPtr<'_, str>,
        key_handle: &guest_types::OptSymmetricKey,
        options_handle: &guest_types::OptOptions,
    ) -> Result<guest_types::SymmetricState, guest_types::CryptoErrno> {
        let mut guest_borrow = wiggle::GuestBorrows::new();
        let alg_str: &str = unsafe { &*alg_str.as_raw(&mut guest_borrow)? };
        let key_handle = match *key_handle {
            guest_types::OptSymmetricKey::Some(key_handle) => Some(key_handle),
            guest_types::OptSymmetricKey::None => None,
        };
        let options_handle = match *options_handle {
            guest_types::OptOptions::Some(options_handle) => Some(options_handle),
            guest_types::OptOptions::None => None,
        };
        Ok(self
            .ctx
            .symmetric_state_open(
                alg_str,
                key_handle.map(Into::into),
                options_handle.map(Into::into),
            )?
            .into())
    }

    fn symmetric_state_options_get(
        &self,
        symmetric_state_handle: guest_types::SymmetricState,
        name_str: &wiggle::GuestPtr<'_, str>,
        value_ptr: &wiggle::GuestPtr<'_, u8>,
        value_max_len: guest_types::Size,
    ) -> Result<guest_types::Size, guest_types::CryptoErrno> {
        let mut guest_borrow = wiggle::GuestBorrows::new();
        let name_str: &str = unsafe { &*name_str.as_raw(&mut guest_borrow)? };
        let value: &mut [u8] = unsafe {
            &mut *value_ptr
                .as_array(value_max_len as _)
                .as_raw(&mut guest_borrow)?
        };
        Ok(self
            .ctx
            .options_get(symmetric_state_handle.into(), name_str, value)?
            .try_into()?)
    }

    fn symmetric_state_options_get_u64(
        &self,
        symmetric_state_handle: guest_types::SymmetricState,
        name_str: &wiggle::GuestPtr<'_, str>,
    ) -> Result<u64, guest_types::CryptoErrno> {
        let mut guest_borrow = wiggle::GuestBorrows::new();
        let name_str: &str = unsafe { &*name_str.as_raw(&mut guest_borrow)? };
        Ok(self
            .ctx
            .options_get_u64(symmetric_state_handle.into(), name_str)?
            .into())
    }

    fn symmetric_state_close(
        &self,
        symmetric_state_handle: guest_types::SymmetricState,
    ) -> Result<(), guest_types::CryptoErrno> {
        Ok(self
            .ctx
            .symmetric_state_close(symmetric_state_handle.into())?)
    }

    fn symmetric_state_absorb(
        &self,
        symmetric_state_handle: guest_types::SymmetricState,
        data_ptr: &wiggle::GuestPtr<'_, u8>,
        data_len: guest_types::Size,
    ) -> Result<(), guest_types::CryptoErrno> {
        let mut guest_borrow = wiggle::GuestBorrows::new();
        let data: &[u8] = unsafe { &*data_ptr.as_array(data_len as _).as_raw(&mut guest_borrow)? };
        Ok(self
            .ctx
            .symmetric_state_absorb(symmetric_state_handle.into(), data)?
            .into())
    }

    fn symmetric_state_squeeze(
        &self,
        symmetric_state_handle: guest_types::SymmetricState,
        out_ptr: &wiggle::GuestPtr<'_, u8>,
        out_len: guest_types::Size,
    ) -> Result<(), guest_types::CryptoErrno> {
        let mut guest_borrow = wiggle::GuestBorrows::new();
        let out: &mut [u8] =
            unsafe { &mut *out_ptr.as_array(out_len as _).as_raw(&mut guest_borrow)? };
        Ok(self
            .ctx
            .symmetric_state_squeeze(symmetric_state_handle.into(), out)?
            .into())
    }

    fn symmetric_state_squeeze_tag(
        &self,
        symmetric_state_handle: guest_types::SymmetricState,
    ) -> Result<guest_types::SymmetricTag, guest_types::CryptoErrno> {
        Ok(self
            .ctx
            .symmetric_state_squeeze_tag(symmetric_state_handle.into())?
            .into())
    }

    fn symmetric_state_squeeze_key(
        &self,
        symmetric_state_handle: guest_types::SymmetricState,
        alg_str: &wiggle::GuestPtr<'_, str>,
    ) -> Result<guest_types::SymmetricKey, guest_types::CryptoErrno> {
        let mut guest_borrow = wiggle::GuestBorrows::new();
        let alg_str: &str = unsafe { &*alg_str.as_raw(&mut guest_borrow)? };
        Ok(self
            .ctx
            .symmetric_state_squeeze_key(symmetric_state_handle.into(), alg_str)?
            .into())
    }

    fn symmetric_state_max_tag_len(
        &self,
        symmetric_state_handle: guest_types::SymmetricState,
    ) -> Result<guest_types::Size, guest_types::CryptoErrno> {
        Ok(self
            .ctx
            .symmetric_state_max_tag_len(symmetric_state_handle.into())?
            .try_into()?)
    }

    fn symmetric_state_encrypt(
        &self,
        symmetric_state_handle: guest_types::SymmetricState,
        out_ptr: &wiggle::GuestPtr<'_, u8>,
        out_len: guest_types::Size,
        data_ptr: &wiggle::GuestPtr<'_, u8>,
        data_len: guest_types::Size,
    ) -> Result<guest_types::Size, guest_types::CryptoErrno> {
        let mut guest_borrow = wiggle::GuestBorrows::new();
        let out: &mut [u8] =
            unsafe { &mut *out_ptr.as_array(out_len as _).as_raw(&mut guest_borrow)? };
        let data: &[u8] = unsafe { &*data_ptr.as_array(data_len as _).as_raw(&mut guest_borrow)? };
        Ok(self
            .ctx
            .symmetric_state_encrypt(symmetric_state_handle.into(), out, data)?
            .try_into()?)
    }

    fn symmetric_state_encrypt_detached(
        &self,
        symmetric_state_handle: guest_types::SymmetricState,
        out_ptr: &wiggle::GuestPtr<'_, u8>,
        out_len: guest_types::Size,
        data_ptr: &wiggle::GuestPtr<'_, u8>,
        data_len: guest_types::Size,
    ) -> Result<guest_types::SymmetricTag, guest_types::CryptoErrno> {
        let mut guest_borrow = wiggle::GuestBorrows::new();
        let out: &mut [u8] =
            unsafe { &mut *out_ptr.as_array(out_len as _).as_raw(&mut guest_borrow)? };
        let data: &[u8] = unsafe { &*data_ptr.as_array(data_len as _).as_raw(&mut guest_borrow)? };
        Ok(self
            .ctx
            .symmetric_state_encrypt_detached(symmetric_state_handle.into(), out, data)?
            .into())
    }

    fn symmetric_state_decrypt(
        &self,
        symmetric_state_handle: guest_types::SymmetricState,
        out_ptr: &wiggle::GuestPtr<'_, u8>,
        out_len: guest_types::Size,
        data_ptr: &wiggle::GuestPtr<'_, u8>,
        data_len: guest_types::Size,
    ) -> Result<guest_types::Size, guest_types::CryptoErrno> {
        let mut guest_borrow = wiggle::GuestBorrows::new();
        let out: &mut [u8] =
            unsafe { &mut *out_ptr.as_array(out_len as _).as_raw(&mut guest_borrow)? };
        let data: &[u8] = unsafe { &*data_ptr.as_array(data_len as _).as_raw(&mut guest_borrow)? };
        Ok(self
            .ctx
            .symmetric_state_decrypt(symmetric_state_handle.into(), out, data)?
            .try_into()?)
    }

    fn symmetric_state_decrypt_detached(
        &self,
        symmetric_state_handle: guest_types::SymmetricState,
        out_ptr: &wiggle::GuestPtr<'_, u8>,
        out_len: guest_types::Size,
        data_ptr: &wiggle::GuestPtr<'_, u8>,
        data_len: guest_types::Size,
        raw_tag_ptr: &wiggle::GuestPtr<'_, u8>,
        raw_tag_len: guest_types::Size,
    ) -> Result<guest_types::Size, guest_types::CryptoErrno> {
        let mut guest_borrow = wiggle::GuestBorrows::new();
        let out: &mut [u8] =
            unsafe { &mut *out_ptr.as_array(out_len as _).as_raw(&mut guest_borrow)? };
        let data: &[u8] = unsafe { &*data_ptr.as_array(data_len as _).as_raw(&mut guest_borrow)? };
        let raw_tag: &[u8] = unsafe {
            &*raw_tag_ptr
                .as_array(raw_tag_len as _)
                .as_raw(&mut guest_borrow)?
        };
        Ok(self
            .ctx
            .symmetric_state_decrypt_detached(symmetric_state_handle.into(), out, data, raw_tag)?
            .try_into()?)
    }

    fn symmetric_state_ratchet(
        &self,
        symmetric_state_handle: guest_types::SymmetricState,
    ) -> Result<(), guest_types::CryptoErrno> {
        Ok(self
            .ctx
            .symmetric_state_ratchet(symmetric_state_handle.into())?)
    }

    // --- tag

    fn symmetric_tag_len(
        &self,
        symmetric_tag_handle: guest_types::SymmetricTag,
    ) -> Result<guest_types::Size, guest_types::CryptoErrno> {
        Ok(self
            .ctx
            .symmetric_tag_len(symmetric_tag_handle.into())?
            .try_into()?)
    }

    fn symmetric_tag_pull(
        &self,
        symmetric_tag_handle: guest_types::SymmetricTag,
        buf_ptr: &wiggle::GuestPtr<'_, u8>,
        buf_len: guest_types::Size,
    ) -> Result<guest_types::Size, guest_types::CryptoErrno> {
        let mut guest_borrow = wiggle::GuestBorrows::new();
        let buf: &mut [u8] =
            unsafe { &mut *buf_ptr.as_array(buf_len as _).as_raw(&mut guest_borrow)? };
        Ok(self
            .ctx
            .symmetric_tag_pull(symmetric_tag_handle.into(), buf)?
            .try_into()?)
    }

    fn symmetric_tag_verify(
        &self,
        symmetric_tag_handle: guest_types::SymmetricTag,
        expected_raw_ptr: &wiggle::GuestPtr<'_, u8>,
        expected_raw_len: guest_types::Size,
    ) -> Result<(), guest_types::CryptoErrno> {
        let mut guest_borrow = wiggle::GuestBorrows::new();
        let expected_raw: &[u8] = unsafe {
            &*expected_raw_ptr
                .as_array(expected_raw_len as _)
                .as_raw(&mut guest_borrow)?
        };
        Ok(self
            .ctx
            .symmetric_tag_verify(symmetric_tag_handle.into(), expected_raw)?
            .into())
    }

    fn symmetric_tag_close(
        &self,
        symmetric_tag_handle: guest_types::SymmetricTag,
    ) -> Result<(), guest_types::CryptoErrno> {
        Ok(self
            .ctx
            .symmetric_tag_close(symmetric_tag_handle.into())?
            .into())
    }
}
