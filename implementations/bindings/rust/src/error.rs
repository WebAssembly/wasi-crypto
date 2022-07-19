use std::convert::TryFrom;

use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::raw;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, TryFromPrimitive, IntoPrimitive)]
#[repr(u16)]
pub enum Error {
    GuestError = raw::CRYPTO_ERRNO_GUEST_ERROR.raw(),
    NotImplemented = raw::CRYPTO_ERRNO_NOT_IMPLEMENTED.raw(),
    UnsupportedFeature = raw::CRYPTO_ERRNO_UNSUPPORTED_FEATURE.raw(),
    ProhibitedOperation = raw::CRYPTO_ERRNO_PROHIBITED_OPERATION.raw(),
    UnsupportedEncoding = raw::CRYPTO_ERRNO_UNSUPPORTED_ENCODING.raw(),
    UnsupportedAlgorithm = raw::CRYPTO_ERRNO_UNSUPPORTED_ALGORITHM.raw(),
    UnsupportedOption = raw::CRYPTO_ERRNO_UNSUPPORTED_OPTION.raw(),
    InvalidKey = raw::CRYPTO_ERRNO_INVALID_KEY.raw(),
    InvalidLength = raw::CRYPTO_ERRNO_INVALID_LENGTH.raw(),
    VerificationFailed = raw::CRYPTO_ERRNO_VERIFICATION_FAILED.raw(),
    RngError = raw::CRYPTO_ERRNO_RNG_ERROR.raw(),
    AlgorithmFailure = raw::CRYPTO_ERRNO_ALGORITHM_FAILURE.raw(),
    InvalidSignature = raw::CRYPTO_ERRNO_INVALID_SIGNATURE.raw(),
    Closed = raw::CRYPTO_ERRNO_CLOSED.raw(),
    InvalidHandle = raw::CRYPTO_ERRNO_INVALID_HANDLE.raw(),
    Overflow = raw::CRYPTO_ERRNO_OVERFLOW.raw(),
    InternalError = raw::CRYPTO_ERRNO_INTERNAL_ERROR.raw(),
    TooManyHandles = raw::CRYPTO_ERRNO_TOO_MANY_HANDLES.raw(),
    KeyNotSupported = raw::CRYPTO_ERRNO_KEY_NOT_SUPPORTED.raw(),
    KeyRequired = raw::CRYPTO_ERRNO_KEY_REQUIRED.raw(),
    InvalidTag = raw::CRYPTO_ERRNO_INVALID_TAG.raw(),
    InvalidOperation = raw::CRYPTO_ERRNO_INVALID_OPERATION.raw(),
    NonceRequired = raw::CRYPTO_ERRNO_NONCE_REQUIRED.raw(),
    InvalidNonce = raw::CRYPTO_ERRNO_INVALID_NONCE.raw(),
    OptionNotSet = raw::CRYPTO_ERRNO_OPTION_NOT_SET.raw(),
    NotFound = raw::CRYPTO_ERRNO_NOT_FOUND.raw(),
    ParametersMissing = raw::CRYPTO_ERRNO_PARAMETERS_MISSING.raw(),
    InProgress = raw::CRYPTO_ERRNO_IN_PROGRESS.raw(),
    IncompatibleKeys = raw::CRYPTO_ERRNO_INCOMPATIBLE_KEYS.raw(),
    Expired = raw::CRYPTO_ERRNO_EXPIRED.raw(),
}

impl From<raw::CryptoErrno> for Error {
    fn from(e: raw::CryptoErrno) -> Error {
        Error::from_raw_error(e.raw()).unwrap()
    }
}

impl Error {
    pub fn from_raw_error(e: u16) -> Option<Self> {
        match e {
            0 => None,
            e => Some(Error::try_from(e).expect("Unexpected error")),
        }
    }
}
