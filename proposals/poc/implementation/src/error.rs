pub use anyhow::{bail, ensure, Error};

#[derive(thiserror::Error, Debug)]
pub enum CryptoError {
    #[error("Unsupported operation")]
    UnsupportedOperation,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u16)]
pub enum WasiCryptoError {
    Success = 0,
    UnsupportedEncoding = 1,
    UnsupportedAlgorithm = 2,
    UnsupportedOperation = 3,
    InvalidKey = 4,
    VerificationFailed = 5,
    RNGError = 6,
    AlgorithmFailure = 7,
    InvalidSignature = 8,
    Closed = 9,
    InvalidHandle = 10,
    Overflow = 11,
    InternalError = 12,
    TooManyHandles = 13,
}

impl CryptoError {
    pub fn as_raw_errno(&self) -> WasiCryptoError {
        match self {
            CryptoError::UnsupportedOperation => WasiCryptoError::UnsupportedOperation,
            CryptoError::UnsupportedEncoding => WasiCryptoError::UnsupportedEncoding,
            CryptoError::UnsupportedAlgorithm => WasiCryptoError::UnsupportedAlgorithm,
            CryptoError::InvalidKey => WasiCryptoError::InvalidKey,
            CryptoError::VerificationFailed => WasiCryptoError::VerificationFailed,
            CryptoError::RNGError => WasiCryptoError::RNGError,
            CryptoError::AlgorithmFailure => WasiCryptoError::AlgorithmFailure,
            CryptoError::InvalidSignature => WasiCryptoError::InvalidSignature,
            CryptoError::Closed => WasiCryptoError::Closed,
            CryptoError::InvalidHandle => WasiCryptoError::InvalidHandle,
            CryptoError::Overflow => WasiCryptoError::Overflow,
            CryptoError::InternalError => WasiCryptoError::InternalError,
            CryptoError::TooManyHandles => WasiCryptoError::TooManyHandles,
        }
    }
}
