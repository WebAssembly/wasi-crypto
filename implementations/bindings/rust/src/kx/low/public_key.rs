use crate::asymmetric_common::*;
use crate::error::*;
use crate::raw;

#[derive(Debug)]
pub struct KxPublicKey(pub(crate) PublicKey);

impl KxPublicKey {
    pub fn from_raw(alg: &'static str, encoded: impl AsRef<[u8]>) -> Result<Self, Error> {
        Ok(KxPublicKey(PublicKey::from_raw(
            raw::ALGORITHM_TYPE_KEY_EXCHANGE,
            alg,
            encoded,
        )?))
    }

    pub fn from_pkcs8(alg: &'static str, encoded: impl AsRef<[u8]>) -> Result<Self, Error> {
        Ok(KxPublicKey(PublicKey::from_pkcs8(
            raw::ALGORITHM_TYPE_KEY_EXCHANGE,
            alg,
            encoded,
        )?))
    }

    pub fn from_pem(alg: &'static str, encoded: impl AsRef<[u8]>) -> Result<Self, Error> {
        Ok(KxPublicKey(PublicKey::from_pem(
            raw::ALGORITHM_TYPE_KEY_EXCHANGE,
            alg,
            encoded,
        )?))
    }

    pub fn from_sec(alg: &'static str, encoded: impl AsRef<[u8]>) -> Result<Self, Error> {
        Ok(KxPublicKey(PublicKey::from_sec(
            raw::ALGORITHM_TYPE_KEY_EXCHANGE,
            alg,
            encoded,
        )?))
    }

    pub fn from_local(alg: &'static str, encoded: impl AsRef<[u8]>) -> Result<Self, Error> {
        Ok(KxPublicKey(PublicKey::from_local(
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

    pub fn sec(&self) -> Result<Vec<u8>, Error> {
        self.0.sec()
    }

    pub fn local(&self) -> Result<Vec<u8>, Error> {
        self.0.local()
    }
}
