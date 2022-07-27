use super::public_key::*;
use super::secret_key::*;
use crate::asymmetric_common::*;
use crate::error::*;
use crate::raw;

#[derive(Debug)]
pub struct KxKeyPair(KeyPair);

impl KxKeyPair {
    pub fn generate(alg: &'static str) -> Result<Self, Error> {
        Ok(KxKeyPair(KeyPair::generate(
            raw::ALGORITHM_TYPE_KEY_EXCHANGE,
            alg,
        )?))
    }

    pub fn publickey(&self) -> Result<KxPublicKey, Error> {
        Ok(KxPublicKey(self.0.publickey()?))
    }

    pub fn secretkey(&self) -> Result<KxSecretKey, Error> {
        Ok(KxSecretKey(self.0.secretkey()?))
    }

    pub fn from_raw(alg: &'static str, encoded: impl AsRef<[u8]>) -> Result<Self, Error> {
        Ok(KxKeyPair(KeyPair::from_raw(
            raw::ALGORITHM_TYPE_KEY_EXCHANGE,
            alg,
            encoded,
        )?))
    }

    pub fn from_pkcs8(alg: &'static str, encoded: impl AsRef<[u8]>) -> Result<Self, Error> {
        Ok(KxKeyPair(KeyPair::from_pkcs8(
            raw::ALGORITHM_TYPE_KEY_EXCHANGE,
            alg,
            encoded,
        )?))
    }

    pub fn from_pem(alg: &'static str, encoded: impl AsRef<[u8]>) -> Result<Self, Error> {
        Ok(KxKeyPair(KeyPair::from_pem(
            raw::ALGORITHM_TYPE_KEY_EXCHANGE,
            alg,
            encoded,
        )?))
    }

    pub fn from_local(alg: &'static str, encoded: impl AsRef<[u8]>) -> Result<Self, Error> {
        Ok(KxKeyPair(KeyPair::from_local(
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
