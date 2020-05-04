use std::convert::TryInto;

use crate::error::*;
use crate::types as guest_types;
use crate::WasiCryptoCtx;

impl crate::wasi_ephemeral_crypto_signatures::WasiEphemeralCryptoSignatures for WasiCryptoCtx {
    // --- keypair_manager

    fn signature_managed_keypair_generate(
        &self,
        key_manager_handle: guest_types::KeyManager,
        alg_str: &wiggle::GuestPtr<'_, str>,
        options_handle: &guest_types::OptOptions,
    ) -> Result<guest_types::SignatureKeypair, guest_types::CryptoErrno> {
        let mut guest_borrow = wiggle::GuestBorrows::new();
        let alg_str: &str = unsafe { &*alg_str.as_raw(&mut guest_borrow)? };
        let options_handle = match *options_handle {
            guest_types::OptOptions::Some(options_handle) => Some(options_handle),
            guest_types::OptOptions::None => None,
        };
        Ok(self
            .ctx
            .signature_managed_keypair_generate(
                key_manager_handle.into(),
                alg_str,
                options_handle.map(Into::into),
            )?
            .into())
    }

    fn signature_keypair_from_id(
        &self,
        key_manager_handle: guest_types::KeyManager,
        kp_id_ptr: &wiggle::GuestPtr<'_, u8>,
        kp_id_len: guest_types::Size,
        kp_version: guest_types::Version,
    ) -> Result<guest_types::SignatureKeypair, guest_types::CryptoErrno> {
        let mut guest_borrow = wiggle::GuestBorrows::new();
        let kp_id: &[u8] = unsafe {
            &*kp_id_ptr
                .as_array(kp_id_len as _)
                .as_raw(&mut guest_borrow)?
        };
        Ok(self
            .ctx
            .signature_keypair_from_id(key_manager_handle.into(), kp_id, kp_version.into())?
            .into())
    }

    // --- keypair

    fn signature_keypair_generate(
        &self,
        alg_str: &wiggle::GuestPtr<'_, str>,
        options_handle: &guest_types::OptOptions,
    ) -> Result<guest_types::SignatureKeypair, guest_types::CryptoErrno> {
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

    fn signature_keypair_import(
        &self,
        alg_str: &wiggle::GuestPtr<'_, str>,
        encoded_ptr: &wiggle::GuestPtr<'_, u8>,
        encoded_len: guest_types::Size,
        encoding: guest_types::KeypairEncoding,
    ) -> Result<guest_types::SignatureKeypair, guest_types::CryptoErrno> {
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

    fn signature_keypair_id(
        &self,
        kp_handle: guest_types::SignatureKeypair,
        kp_id_ptr: &wiggle::GuestPtr<'_, u8>,
        kp_id_max_len: guest_types::Size,
    ) -> Result<(guest_types::Size, guest_types::Version), guest_types::CryptoErrno> {
        let mut guest_borrow = wiggle::GuestBorrows::new();
        let kp_id_buf: &mut [u8] = unsafe {
            &mut *kp_id_ptr
                .as_array(kp_id_max_len as _)
                .as_raw(&mut guest_borrow)?
        };
        let (kp_id, version) = self.ctx.signature_keypair_id(kp_handle.into())?;
        ensure!(kp_id.len() <= kp_id_buf.len(), CryptoError::Overflow.into());
        kp_id_buf.copy_from_slice(&kp_id);
        Ok((kp_id.len().try_into()?, version.into()))
    }

    fn signature_keypair_export(
        &self,
        kp_handle: guest_types::SignatureKeypair,
        encoding: guest_types::KeypairEncoding,
    ) -> Result<guest_types::ArrayOutput, guest_types::CryptoErrno> {
        Ok(self
            .ctx
            .signature_keypair_export(kp_handle.into(), encoding.into())?
            .into())
    }

    fn signature_keypair_publickey(
        &self,
        kp_handle: guest_types::SignatureKeypair,
    ) -> Result<guest_types::SignaturePublickey, guest_types::CryptoErrno> {
        Ok(self
            .ctx
            .signature_keypair_publickey(kp_handle.into())?
            .into())
    }

    fn signature_keypair_close(
        &self,
        kp_handle: guest_types::SignatureKeypair,
    ) -> Result<(), guest_types::CryptoErrno> {
        Ok(self.ctx.signature_keypair_close(kp_handle.into())?.into())
    }

    // --- publickey

    fn signature_publickey_import(
        &self,
        alg_str: &wiggle::GuestPtr<'_, str>,
        encoded_ptr: &wiggle::GuestPtr<'_, u8>,
        encoded_len: guest_types::Size,
        encoding: guest_types::PublickeyEncoding,
    ) -> Result<guest_types::SignaturePublickey, guest_types::CryptoErrno> {
        let mut guest_borrow = wiggle::GuestBorrows::new();
        let alg_str: &str = unsafe { &*alg_str.as_raw(&mut guest_borrow)? };
        let encoded: &[u8] = unsafe {
            &*encoded_ptr
                .as_array(encoded_len as _)
                .as_raw(&mut guest_borrow)?
        };
        Ok(self
            .ctx
            .signature_publickey_import(alg_str, encoded, encoding.into())?
            .into())
    }

    fn signature_publickey_export(
        &self,
        pk: guest_types::SignaturePublickey,
        encoding: guest_types::PublickeyEncoding,
    ) -> Result<guest_types::ArrayOutput, guest_types::CryptoErrno> {
        Ok(self
            .ctx
            .signature_publickey_export(pk.into(), encoding.into())?
            .into())
    }

    fn signature_publickey_verify(
        &self,
        pk: guest_types::SignaturePublickey,
    ) -> Result<(), guest_types::CryptoErrno> {
        Ok(self.ctx.signature_publickey_verify(pk.into())?.into())
    }

    fn signature_publickey_close(
        &self,
        pk: guest_types::SignaturePublickey,
    ) -> Result<(), guest_types::CryptoErrno> {
        Ok(self.ctx.signature_publickey_close(pk.into())?.into())
    }

    // --- signature

    fn signature_export(
        &self,
        signature_handle: guest_types::Signature,
        encoding: guest_types::SignatureEncoding,
    ) -> Result<guest_types::ArrayOutput, guest_types::CryptoErrno> {
        Ok(self
            .ctx
            .signature_export(signature_handle.into(), encoding.into())?
            .into())
    }

    fn signature_import(
        &self,
        alg_str: &wiggle::GuestPtr<'_, str>,
        encoding: guest_types::SignatureEncoding,
        encoded_ptr: &wiggle::GuestPtr<'_, u8>,
        encoded_len: guest_types::Size,
    ) -> Result<guest_types::Signature, guest_types::CryptoErrno> {
        let mut guest_borrow = wiggle::GuestBorrows::new();
        let alg_str: &str = unsafe { &*alg_str.as_raw(&mut guest_borrow)? };
        let encoded: &[u8] = unsafe {
            &*encoded_ptr
                .as_array(encoded_len as _)
                .as_raw(&mut guest_borrow)?
        };
        Ok(self
            .ctx
            .signature_import(alg_str, encoding.into(), encoded)?
            .into())
    }

    fn signature_state_open(
        &self,
        kp_handle: guest_types::SignatureKeypair,
    ) -> Result<guest_types::SignatureState, guest_types::CryptoErrno> {
        Ok(self.ctx.signature_state_open(kp_handle.into())?.into())
    }

    fn signature_state_update(
        &self,
        state_handle: guest_types::SignatureState,
        input_ptr: &wiggle::GuestPtr<'_, u8>,
        input_len: guest_types::Size,
    ) -> Result<(), guest_types::CryptoErrno> {
        let mut guest_borrow = wiggle::GuestBorrows::new();
        let input: &[u8] = unsafe {
            &*input_ptr
                .as_array(input_len as _)
                .as_raw(&mut guest_borrow)?
        };
        Ok(self
            .ctx
            .signature_state_update(state_handle.into(), input)?
            .into())
    }

    fn signature_state_sign(
        &self,
        signature_state_handle: guest_types::SignatureState,
    ) -> Result<guest_types::ArrayOutput, guest_types::CryptoErrno> {
        Ok(self
            .ctx
            .signature_state_sign(signature_state_handle.into())?
            .into())
    }

    fn signature_state_close(
        &self,
        signature_state_handle: guest_types::SignatureState,
    ) -> Result<(), guest_types::CryptoErrno> {
        Ok(self
            .ctx
            .signature_state_close(signature_state_handle.into())?
            .into())
    }

    fn signature_verification_state_open(
        &self,
        pk_handle: guest_types::SignaturePublickey,
    ) -> Result<guest_types::SignatureVerificationState, guest_types::CryptoErrno> {
        Ok(self
            .ctx
            .signature_verification_state_open(pk_handle.into())?
            .into())
    }

    fn signature_verification_state_update(
        &self,
        verification_state_handle: guest_types::SignatureVerificationState,
        input_ptr: &wiggle::GuestPtr<'_, u8>,
        input_len: guest_types::Size,
    ) -> Result<(), guest_types::CryptoErrno> {
        let mut guest_borrow = wiggle::GuestBorrows::new();
        let input: &[u8] = unsafe {
            &*input_ptr
                .as_array(input_len as _)
                .as_raw(&mut guest_borrow)?
        };
        Ok(self
            .ctx
            .signature_verification_state_update(verification_state_handle.into(), input)?
            .into())
    }

    fn signature_verification_state_verify(
        &self,
        verification_state_handle: guest_types::SignatureVerificationState,
        signature_handle: guest_types::Signature,
    ) -> Result<(), guest_types::CryptoErrno> {
        Ok(self
            .ctx
            .signature_verification_state_verify(
                verification_state_handle.into(),
                signature_handle.into(),
            )?
            .into())
    }

    fn signature_verification_state_close(
        &self,
        verification_state_handle: guest_types::SignatureVerificationState,
    ) -> Result<(), guest_types::CryptoErrno> {
        Ok(self
            .ctx
            .signature_verification_state_close(verification_state_handle.into())?
            .into())
    }

    fn signature_close(
        &self,
        signature_handle: guest_types::Signature,
    ) -> Result<(), guest_types::CryptoErrno> {
        Ok(self.ctx.signature_close(signature_handle.into())?.into())
    }
}
