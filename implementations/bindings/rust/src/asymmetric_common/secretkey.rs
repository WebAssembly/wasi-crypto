use crate::common::*;
use crate::error::*;
use crate::raw;

#[derive(Debug)]
pub(crate) struct SecretKey {
    pub handle: raw::Secretkey,
    pub alg: &'static str,
}

impl Drop for SecretKey {
    fn drop(&mut self) {
        unsafe { raw::secretkey_close(self.handle) }.unwrap()
    }
}

impl SecretKey {
    fn decode_from(
        alg_type: raw::AlgorithmType,
        alg: &'static str,
        encoded: impl AsRef<[u8]>,
        encoding: raw::SecretkeyEncoding,
    ) -> Result<Self, Error> {
        let encoded = encoded.as_ref();
        let handle = unsafe {
            raw::secretkey_import(alg_type, alg, encoded.as_ptr(), encoded.len(), encoding)
        }?;
        Ok(SecretKey { handle, alg })
    }

    pub fn from_raw(
        alg_type: raw::AlgorithmType,
        alg: &'static str,
        encoded: impl AsRef<[u8]>,
    ) -> Result<Self, Error> {
        Self::decode_from(alg_type, alg, encoded, raw::SECRETKEY_ENCODING_RAW)
    }

    pub fn from_pkcs8(
        alg_type: raw::AlgorithmType,
        alg: &'static str,
        encoded: impl AsRef<[u8]>,
    ) -> Result<Self, Error> {
        Self::decode_from(alg_type, alg, encoded, raw::SECRETKEY_ENCODING_PKCS8)
    }

    pub fn from_pem(
        alg_type: raw::AlgorithmType,
        alg: &'static str,
        encoded: impl AsRef<[u8]>,
    ) -> Result<Self, Error> {
        Self::decode_from(alg_type, alg, encoded, raw::SECRETKEY_ENCODING_PEM)
    }

    pub fn from_sec(
        alg_type: raw::AlgorithmType,
        alg: &'static str,
        encoded: impl AsRef<[u8]>,
    ) -> Result<Self, Error> {
        Self::decode_from(alg_type, alg, encoded, raw::SECRETKEY_ENCODING_SEC)
    }

    pub fn from_local(
        alg_type: raw::AlgorithmType,
        alg: &'static str,
        encoded: impl AsRef<[u8]>,
    ) -> Result<Self, Error> {
        Self::decode_from(alg_type, alg, encoded, raw::SECRETKEY_ENCODING_LOCAL)
    }

    fn encode_as(&self, encoding: raw::SecretkeyEncoding) -> Result<Vec<u8>, Error> {
        let array_handle = unsafe { raw::secretkey_export(self.handle, encoding) }?;
        ArrayOutput::new(array_handle).into_vec()
    }

    pub fn raw(&self) -> Result<Vec<u8>, Error> {
        self.encode_as(raw::SECRETKEY_ENCODING_RAW)
    }

    pub fn pkcs8(&self) -> Result<Vec<u8>, Error> {
        self.encode_as(raw::SECRETKEY_ENCODING_PKCS8)
    }

    pub fn pem(&self) -> Result<Vec<u8>, Error> {
        self.encode_as(raw::SECRETKEY_ENCODING_PEM)
    }

    pub fn sec(&self) -> Result<Vec<u8>, Error> {
        self.encode_as(raw::SECRETKEY_ENCODING_SEC)
    }

    pub fn local(&self) -> Result<Vec<u8>, Error> {
        self.encode_as(raw::SECRETKEY_ENCODING_LOCAL)
    }
}
