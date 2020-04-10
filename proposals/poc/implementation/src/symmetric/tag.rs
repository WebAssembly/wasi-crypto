use super::*;
use crate::types as guest_types;
use crate::{CryptoCtx, WasiCryptoCtx};

use std::convert::TryInto;
use zeroize::Zeroize;

#[derive(Debug, Clone, Eq)]
pub struct SymmetricTag {
    alg: SymmetricAlgorithm,
    raw: Vec<u8>,
}

impl PartialEq for SymmetricTag {
    fn eq(&self, other: &Self) -> bool {
        self.alg == other.alg
            && ring::constant_time::verify_slices_are_equal(&self.raw, &other.raw).is_ok()
    }
}

impl Drop for SymmetricTag {
    fn drop(&mut self) {
        self.raw.zeroize();
    }
}

impl SymmetricTag {
    pub fn new(alg: SymmetricAlgorithm, raw: Vec<u8>) -> Self {
        SymmetricTag { alg, raw }
    }

    pub fn verify(&self, expected_raw: &[u8]) -> Result<(), CryptoError> {
        ring::constant_time::verify_slices_are_equal(&self.raw, expected_raw)
            .map_err(|_| CryptoError::InvalidTag)
    }
}

impl AsRef<[u8]> for SymmetricTag {
    fn as_ref(&self) -> &[u8] {
        &self.raw
    }
}

impl CryptoCtx {
    pub fn symmetric_tag_len(&self, symmetric_tag_handle: Handle) -> Result<usize, CryptoError> {
        let symmetric_tag = self.handles.symmetric_tag.get(symmetric_tag_handle)?;
        Ok(symmetric_tag.as_ref().len())
    }

    pub fn symmetric_tag_pull(
        &self,
        symmetric_tag_handle: Handle,
        buf: &mut [u8],
    ) -> Result<usize, CryptoError> {
        let symmetric_tag = self.handles.symmetric_tag.get(symmetric_tag_handle)?;
        let raw = symmetric_tag.as_ref();
        let raw_len = raw.len();
        let buf_len = buf.len();
        ensure!(raw_len <= buf_len, CryptoError::Overflow);
        &buf[..raw_len].copy_from_slice(raw);
        self.handles.symmetric_tag.close(symmetric_tag_handle)?;
        Ok(raw_len)
    }

    pub fn symmetric_tag_verify(
        &self,
        symmetric_tag_handle: Handle,
        expected_raw: &[u8],
    ) -> Result<(), CryptoError> {
        let symmetric_tag = self.handles.symmetric_tag.get(symmetric_tag_handle)?;
        symmetric_tag.verify(expected_raw)
    }

    pub fn symmetric_tag_close(&self, symmetric_tag_handle: Handle) -> Result<(), CryptoError> {
        self.handles.symmetric_tag.close(symmetric_tag_handle)
    }
}

impl WasiCryptoCtx {
    pub fn symmetric_tag_len(
        &self,
        symmetric_tag_handle: guest_types::SymmetricTag,
    ) -> Result<guest_types::Size, CryptoError> {
        Ok(self
            .ctx
            .symmetric_tag_len(symmetric_tag_handle.into())?
            .try_into()?)
    }

    pub fn symmetric_tag_pull(
        &self,
        symmetric_tag_handle: guest_types::SymmetricTag,
        buf_ptr: &wiggle::GuestPtr<'_, u8>,
        buf_len: guest_types::Size,
    ) -> Result<guest_types::Size, CryptoError> {
        let mut guest_borrow = wiggle::GuestBorrows::new();
        let buf: &mut [u8] =
            unsafe { &mut *buf_ptr.as_array(buf_len as _).as_raw(&mut guest_borrow)? };
        Ok(self
            .ctx
            .symmetric_tag_pull(symmetric_tag_handle.into(), buf)?
            .try_into()?)
    }

    pub fn symmetric_tag_verify(
        &self,
        symmetric_tag_handle: guest_types::SymmetricTag,
        expected_raw_ptr: &wiggle::GuestPtr<'_, u8>,
        expected_raw_len: guest_types::Size,
    ) -> Result<(), CryptoError> {
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

    pub fn symmetric_tag_close(
        &self,
        symmetric_tag_handle: guest_types::SymmetricTag,
    ) -> Result<(), CryptoError> {
        Ok(self
            .ctx
            .symmetric_tag_close(symmetric_tag_handle.into())?
            .into())
    }
}
