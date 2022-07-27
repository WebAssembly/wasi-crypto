use crate::asymmetric_common::*;
use crate::error::*;
use crate::raw;

#[derive(Debug)]
pub struct KxSecretKey(pub(crate) SecretKey);

impl KxSecretKey {
    pub fn from_raw(alg: &'static str, encoded: impl AsRef<[u8]>) -> Result<Self, Error> {
        Ok(KxSecretKey(SecretKey::from_raw(
            raw::ALGORITHM_TYPE_KEY_EXCHANGE,
            alg,
            encoded,
        )?))
    }

    pub fn from_pkcs8(alg: &'static str, encoded: impl AsRef<[u8]>) -> Result<Self, Error> {
        Ok(KxSecretKey(SecretKey::from_pkcs8(
            raw::ALGORITHM_TYPE_KEY_EXCHANGE,
            alg,
            encoded,
        )?))
    }

    pub fn from_pem(alg: &'static str, encoded: impl AsRef<[u8]>) -> Result<Self, Error> {
        Ok(KxSecretKey(SecretKey::from_pem(
            raw::ALGORITHM_TYPE_KEY_EXCHANGE,
            alg,
            encoded,
        )?))
    }

    pub fn from_local(alg: &'static str, encoded: impl AsRef<[u8]>) -> Result<Self, Error> {
        Ok(KxSecretKey(SecretKey::from_local(
            raw::ALGORITHM_TYPE_KEY_EXCHANGE,
            alg,
            encoded,
        )?))
    }

    pub fn raw(&self) -> Result<Vec<u8>, Error> {
        self.0.raw()
    }

    pub fn pkcs8(&self) -> Result<Vec<u8>, Error> {
        self.0.pkcs8()
    }

    pub fn pem(&self) -> Result<Vec<u8>, Error> {
        self.0.pem()
    }

    pub fn local(&self) -> Result<Vec<u8>, Error> {
        self.0.local()
    }
}
