pub use anyhow::{bail, ensure, Error};

#[derive(thiserror::Error, Debug)]
pub enum CryptoError {
    #[error("Operation not available")]
    NotAvailable,
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
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u16)]
pub enum WasiCryptoError {
    Success = 0,
    NotAvailable = 1,
    InvalidKey = 2,
    VerificationFailed = 3,
    RNGError = 4,
    AlgorithmFailure = 5,
    InvalidSignature = 6,
    Closed = 7,
    InvalidHandle = 8,
    Overflow = 9,
}

impl CryptoError {
    pub fn as_raw_errno(&self) -> WasiCryptoError {
        match self {
            CryptoError::NotAvailable => WasiCryptoError::NotAvailable,
            CryptoError::InvalidKey => WasiCryptoError::InvalidKey,
            CryptoError::VerificationFailed => WasiCryptoError::VerificationFailed,
            CryptoError::RNGError => WasiCryptoError::RNGError,
            CryptoError::AlgorithmFailure => WasiCryptoError::AlgorithmFailure,
            CryptoError::InvalidSignature => WasiCryptoError::InvalidSignature,
            CryptoError::Closed => WasiCryptoError::Closed,
            CryptoError::InvalidHandle => WasiCryptoError::InvalidHandle,
            CryptoError::Overflow => WasiCryptoError::Overflow,
        }
    }
}
