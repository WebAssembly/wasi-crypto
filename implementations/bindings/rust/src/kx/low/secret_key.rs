use crate::asymmetric_common::*;
use crate::common::ArrayOutput;
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

    pub fn from_sec(alg: &'static str, encoded: impl AsRef<[u8]>) -> Result<Self, Error> {
        Ok(KxSecretKey(SecretKey::from_sec(
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

    pub fn sec(&self) -> Result<Vec<u8>, Error> {
        self.0.sec()
    }

    pub fn local(&self) -> Result<Vec<u8>, Error> {
        self.0.local()
    }

    pub fn decapsulate(&self, encapsulated_secret: impl AsRef<[u8]>) -> Result<Vec<u8>, Error> {
        let encapsulated_secret = encapsulated_secret.as_ref();
        let secret_handle = unsafe {
            raw::kx_decapsulate(
                self.0.handle,
                encapsulated_secret.as_ptr(),
                encapsulated_secret.len(),
            )
        }?;
        ArrayOutput::new(secret_handle).into_vec()
    }
}
