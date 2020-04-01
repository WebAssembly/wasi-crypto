use super::types as guest_types;
use super::WasiCryptoCtx;

pub use anyhow::Error;

use std::num::TryFromIntError;

#[derive(thiserror::Error, Debug)]
pub enum CryptoError {
    #[error("Success")]
    Success,
    #[error("Guest error")]
    GuestError(#[from] wiggle::GuestError),
    #[error("Not implemented")]
    NotImplemented,
    #[error("Unsupported feature")]
    UnsupportedFeature,
    #[error("Prohibited by local policy")]
    ProhibitedOperation,
    #[error("Unsupported encoding")]
    UnsupportedEncoding,
    #[error("Unsupported algorithm")]
    UnsupportedAlgorithm,
    #[error("Invalid key")]
    InvalidKey,
    #[error("Verification failed")]
    VerificationFailed,
    #[error("RNG error")]
    RNGError,
    #[error("Operation failed")]
    AlgorithmFailure,
    #[error("Invalid signature")]
    InvalidSignature,
    #[error("Handle already closed")]
    Closed,
    #[error("Invalid handle")]
    InvalidHandle,
    #[error("Overflow")]
    Overflow,
    #[error("Internal error")]
    InternalError,
    #[error("Too many open handles")]
    TooManyHandles,
}

impl From<TryFromIntError> for CryptoError {
    fn from(_: TryFromIntError) -> Self {
        CryptoError::Overflow
    }
}

impl From<CryptoError> for guest_types::CryptoErrno {
    fn from(e: CryptoError) -> Self {
        match e {
            CryptoError::Success => guest_types::CryptoErrno::Success,
            CryptoError::GuestError(_wiggle_error) => guest_types::CryptoErrno::GuestError,
            CryptoError::NotImplemented => guest_types::CryptoErrno::NotImplemented,
            CryptoError::UnsupportedFeature => guest_types::CryptoErrno::UnsupportedFeature,
            CryptoError::ProhibitedOperation => guest_types::CryptoErrno::ProhibitedOperation,
            CryptoError::UnsupportedEncoding => guest_types::CryptoErrno::UnsupportedEncoding,
            CryptoError::UnsupportedAlgorithm => guest_types::CryptoErrno::UnsupportedAlgorithm,
            CryptoError::InvalidKey => guest_types::CryptoErrno::InvalidKey,
            CryptoError::VerificationFailed => guest_types::CryptoErrno::VerificationFailed,
            CryptoError::RNGError => guest_types::CryptoErrno::RngError,
            CryptoError::AlgorithmFailure => guest_types::CryptoErrno::AlgorithmFailure,
            CryptoError::InvalidSignature => guest_types::CryptoErrno::InvalidSignature,
            CryptoError::Closed => guest_types::CryptoErrno::Closed,
            CryptoError::InvalidHandle => guest_types::CryptoErrno::InvalidHandle,
            CryptoError::Overflow => guest_types::CryptoErrno::Overflow,
            CryptoError::InternalError => guest_types::CryptoErrno::InternalError,
            CryptoError::TooManyHandles => guest_types::CryptoErrno::TooManyHandles,
        }
    }
}

#[macro_export]
macro_rules! ensure {
    ($cond:expr, $err:expr $(,)?) => {
        if !$cond {
            return Err($err);
        }
    };
    ($cond:expr, $fmt:expr, $($arg:tt)*) => {
        if !$cond {
            return Err($fmt, $($arg)*);
        }
    };
}

#[macro_export]
macro_rules! bail {
    ($err:expr $(,)?) => {
        return Err($err);
    };
    ($fmt:expr, $($arg:tt)*) => {
        return Err($fmt, $($arg)*);
    };
}

pub use {bail, ensure};

impl From<CryptoError> for i32 {
    fn from(e: CryptoError) -> Self {
        e.into()
    }
}

impl<'a> wiggle::GuestErrorType<'a> for guest_types::CryptoErrno {
    type Context = WasiCryptoCtx;

    fn success() -> Self {
        guest_types::CryptoErrno::Success
    }

    fn from_error(e: wiggle::GuestError, _ctx: &Self::Context) -> Self {
        eprintln!("GUEST ERROR: {:?}", e);
        guest_types::CryptoErrno::GuestError
    }
}
