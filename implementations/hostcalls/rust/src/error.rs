pub use anyhow::Error;
use std::num::TryFromIntError;

#[derive(thiserror::Error, Debug)]
pub enum CryptoError {
    #[error("Success")]
    Success,
    #[error("Guest error")]
    GuestError(#[from] Error),
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
    #[error("Unsupported option")]
    UnsupportedOption,
    #[error("Invalid key")]
    InvalidKey,
    #[error("Verification failed")]
    InvalidLength,
    #[error("Invalid length")]
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
    #[error("Selected algorithm doesn't support a key")]
    KeyNotSupported,
    #[error("Selected algorithm requires a key")]
    KeyRequired,
    #[error("Authentication tag did not verify")]
    InvalidTag,
    #[error("Operation invalid for the selected algorithm")]
    InvalidOperation,
    #[error("Nonce required")]
    NonceRequired,
    #[error("Nonce doesn't have a correct size")]
    InvalidNonce,
    #[error("Option not set")]
    OptionNotSet,
    #[error("Key not found")]
    NotFound,
    #[error("Parameters missing")]
    ParametersMissing,
    #[error("Incompatible keys")]
    IncompatibleKeys,
    #[error("Expired secret")]
    Expired,
}

impl From<TryFromIntError> for CryptoError {
    fn from(_: TryFromIntError) -> Self {
        CryptoError::Overflow
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
        return Err($err)
    };
    ($fmt:expr, $($arg:tt)*) => {
        return Err($fmt, $($arg)*)
    };
}

pub use {bail, ensure};
