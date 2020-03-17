pub use anyhow::{bail, ensure, Error};

#[derive(thiserror::Error, Debug)]
pub enum CryptoError {
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u16)]
pub enum WasiCryptoError {
    Success = 0,
    NotImplemented = 1,
    UnsupportedFeature = 2,
    ProhibitedOperation = 3,
    UnsupportedEncoding = 4,
    UnsupportedAlgorithm = 5,
    InvalidKey = 6,
    VerificationFailed = 7,
    RNGError = 8,
    AlgorithmFailure = 9,
    InvalidSignature = 10,
    Closed = 11,
    InvalidHandle = 12,
    Overflow = 13,
    InternalError = 14,
    TooManyHandles = 15,
}

impl CryptoError {
    pub fn as_raw_errno(&self) -> WasiCryptoError {
        match self {
            CryptoError::NotImplemented => WasiCryptoError::NotImplemented,
            CryptoError::UnsupportedFeature => WasiCryptoError::UnsupportedFeature,
            CryptoError::ProhibitedOperation => WasiCryptoError::ProhibitedOperation,
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
