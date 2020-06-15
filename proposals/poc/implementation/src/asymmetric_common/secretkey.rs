use super::*;
use crate::signatures::SignatureSecretKey;
use crate::types as guest_types;
use crate::{AlgorithmType, CryptoCtx, HandleManagers};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SecretKeyEncoding {
    Raw,
    Der,
    Pem,
    Sec,
    CompressedSec,
}

impl From<guest_types::SecretkeyEncoding> for SecretKeyEncoding {
    fn from(encoding: guest_types::SecretkeyEncoding) -> Self {
        match encoding {
            guest_types::SecretkeyEncoding::Raw => SecretKeyEncoding::Raw,
            guest_types::SecretkeyEncoding::Der => SecretKeyEncoding::Der,
            guest_types::SecretkeyEncoding::Pem => SecretKeyEncoding::Pem,
            guest_types::SecretkeyEncoding::Sec => SecretKeyEncoding::Sec,
            guest_types::SecretkeyEncoding::CompressedSec => SecretKeyEncoding::CompressedSec,
        }
    }
}

#[derive(Clone, Debug)]
pub enum SecretKey {
    Signature(SignatureSecretKey),
}

impl SecretKey {
    fn import(
        _alg_type: AlgorithmType,
        _alg_str: &str,
        _encoded: &[u8],
        _encoding: SecretKeyEncoding,
    ) -> Result<SecretKey, CryptoError> {
        bail!(CryptoError::NotImplemented)
    }

    fn export(
        _handles: &HandleManagers,
        _sk: Handle,
        _encoding: SecretKeyEncoding,
    ) -> Result<Vec<u8>, CryptoError> {
        bail!(CryptoError::NotImplemented)
    }
}

impl CryptoCtx {
    pub fn secretkey_import(
        &self,
        alg_type: AlgorithmType,
        alg_str: &str,
        encoded: &[u8],
        encoding: SecretKeyEncoding,
    ) -> Result<Handle, CryptoError> {
        let sk = SecretKey::import(alg_type, alg_str, encoded, encoding)?;
        let handle = self.handles.secretkey.register(sk)?;
        Ok(handle)
    }

    pub fn secretkey_export(
        &self,
        sk: Handle,
        encoding: SecretKeyEncoding,
    ) -> Result<Handle, CryptoError> {
        let encoded = SecretKey::export(&self.handles, sk, encoding)?;
        let array_output_handle = ArrayOutput::register(&self.handles, encoded)?;
        Ok(array_output_handle)
    }

    pub fn secretkey_close(&self, sk: Handle) -> Result<(), CryptoError> {
        self.handles.secretkey.close(sk)
    }
}
