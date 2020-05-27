use crate::error::*;
use crate::types as guest_types;
use crate::WasiCryptoCtx;

use std::convert::TryInto;

impl crate::wasi_ephemeral_crypto_asymmetric_common::WasiEphemeralCryptoAsymmetricCommon
    for WasiCryptoCtx
{
    // --- keypair_manager

    fn keypair_generate_managed(
        &self,
        key_manager_handle: guest_types::KeyManager,
        alg_type: guest_types::AlgorithmType,
        alg_str: &wiggle::GuestPtr<'_, str>,
        options_handle: &guest_types::OptOptions,
    ) -> Result<guest_types::Keypair, guest_types::CryptoErrno> {
        let mut guest_borrow = wiggle::GuestBorrows::new();
        let alg_str: &str = unsafe { &*alg_str.as_raw(&mut guest_borrow)? };
        let options_handle = match *options_handle {
            guest_types::OptOptions::Some(options_handle) => Some(options_handle),
            guest_types::OptOptions::None => None,
        };
        Ok(self
            .ctx
            .keypair_generate_managed(
                key_manager_handle.into(),
                alg_type.into(),
                alg_str,
                options_handle.map(Into::into),
            )?
            .into())
    }

    fn keypair_from_id(
        &self,
        key_manager_handle: guest_types::KeyManager,
        kp_id_ptr: &wiggle::GuestPtr<'_, u8>,
        kp_id_len: guest_types::Size,
        kp_version: guest_types::Version,
    ) -> Result<guest_types::Keypair, guest_types::CryptoErrno> {
        let mut guest_borrow = wiggle::GuestBorrows::new();
        let kp_id: &[u8] = unsafe {
            &*kp_id_ptr
                .as_array(kp_id_len as _)
                .as_raw(&mut guest_borrow)?
        };
        Ok(self
            .ctx
            .keypair_from_id(key_manager_handle.into(), kp_id, kp_version.into())?
            .into())
    }

    // --- keypair

    fn keypair_generate(
        &self,
        alg_type: guest_types::AlgorithmType,
        alg_str: &wiggle::GuestPtr<'_, str>,
        options_handle: &guest_types::OptOptions,
    ) -> Result<guest_types::Keypair, guest_types::CryptoErrno> {
        let mut guest_borrow = wiggle::GuestBorrows::new();
        let alg_str: &str = unsafe { &*alg_str.as_raw(&mut guest_borrow)? };
        let options_handle = match *options_handle {
            guest_types::OptOptions::Some(options_handle) => Some(options_handle),
            guest_types::OptOptions::None => None,
        };
        Ok(self
            .ctx
            .keypair_generate(alg_type.into(), alg_str, options_handle.map(Into::into))?
            .into())
    }

    fn keypair_import(
        &self,
        alg_type: guest_types::AlgorithmType,
        alg_str: &wiggle::GuestPtr<'_, str>,
        encoded_ptr: &wiggle::GuestPtr<'_, u8>,
        encoded_len: guest_types::Size,
        encoding: guest_types::KeypairEncoding,
    ) -> Result<guest_types::Keypair, guest_types::CryptoErrno> {
        let mut guest_borrow = wiggle::GuestBorrows::new();
        let alg_str: &str = unsafe { &*alg_str.as_raw(&mut guest_borrow)? };
        let encoded: &[u8] = unsafe {
            &*encoded_ptr
                .as_array(encoded_len as _)
                .as_raw(&mut guest_borrow)?
        };
        Ok(self
            .ctx
            .keypair_import(alg_type.into(), alg_str, encoded, encoding.into())?
            .into())
    }

    fn keypair_id(
        &self,
        kp_handle: guest_types::Keypair,
        kp_id_ptr: &wiggle::GuestPtr<'_, u8>,
        kp_id_max_len: guest_types::Size,
    ) -> Result<(guest_types::Size, guest_types::Version), guest_types::CryptoErrno> {
        let mut guest_borrow = wiggle::GuestBorrows::new();
        let kp_id_buf: &mut [u8] = unsafe {
            &mut *kp_id_ptr
                .as_array(kp_id_max_len as _)
                .as_raw(&mut guest_borrow)?
        };
        let (kp_id, version) = self.ctx.keypair_id(kp_handle.into())?;
        ensure!(kp_id.len() <= kp_id_buf.len(), CryptoError::Overflow.into());
        kp_id_buf.copy_from_slice(&kp_id);
        Ok((kp_id.len().try_into()?, version.into()))
    }

    fn keypair_export(
        &self,
        kp_handle: guest_types::Keypair,
        encoding: guest_types::KeypairEncoding,
    ) -> Result<guest_types::ArrayOutput, guest_types::CryptoErrno> {
        Ok(self
            .ctx
            .keypair_export(kp_handle.into(), encoding.into())?
            .into())
    }

    fn keypair_publickey(
        &self,
        kp_handle: guest_types::Keypair,
    ) -> Result<guest_types::Publickey, guest_types::CryptoErrno> {
        Ok(self.ctx.keypair_publickey(kp_handle.into())?.into())
    }

    fn keypair_close(
        &self,
        kp_handle: guest_types::Keypair,
    ) -> Result<(), guest_types::CryptoErrno> {
        Ok(self.ctx.keypair_close(kp_handle.into())?.into())
    }

    // --- publickey

    fn publickey_import(
        &self,
        alg_type: guest_types::AlgorithmType,
        alg_str: &wiggle::GuestPtr<'_, str>,
        encoded_ptr: &wiggle::GuestPtr<'_, u8>,
        encoded_len: guest_types::Size,
        encoding: guest_types::PublickeyEncoding,
    ) -> Result<guest_types::Publickey, guest_types::CryptoErrno> {
        let mut guest_borrow = wiggle::GuestBorrows::new();
        let alg_str: &str = unsafe { &*alg_str.as_raw(&mut guest_borrow)? };
        let encoded: &[u8] = unsafe {
            &*encoded_ptr
                .as_array(encoded_len as _)
                .as_raw(&mut guest_borrow)?
        };
        Ok(self
            .ctx
            .publickey_import(alg_type.into(), alg_str, encoded, encoding.into())?
            .into())
    }

    fn publickey_export(
        &self,
        pk: guest_types::Publickey,
        encoding: guest_types::PublickeyEncoding,
    ) -> Result<guest_types::ArrayOutput, guest_types::CryptoErrno> {
        Ok(self
            .ctx
            .publickey_export(pk.into(), encoding.into())?
            .into())
    }

    fn publickey_verify(&self, pk: guest_types::Publickey) -> Result<(), guest_types::CryptoErrno> {
        Ok(self.ctx.publickey_verify(pk.into())?.into())
    }

    fn publickey_close(&self, pk: guest_types::Publickey) -> Result<(), guest_types::CryptoErrno> {
        Ok(self.ctx.publickey_close(pk.into())?.into())
    }
}
