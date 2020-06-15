use super::*;
use crate::types as guest_types;
use crate::{AlgorithmType, CryptoCtx, HandleManagers};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PublicKeyEncoding {
    Raw,
    Der,
    Pem,
    Sec,
    CompressedSec,
}

impl From<guest_types::PublickeyEncoding> for PublicKeyEncoding {
    fn from(encoding: guest_types::PublickeyEncoding) -> Self {
        match encoding {
            guest_types::PublickeyEncoding::Raw => PublicKeyEncoding::Raw,
            guest_types::PublickeyEncoding::Der => PublicKeyEncoding::Der,
            guest_types::PublickeyEncoding::Pem => PublicKeyEncoding::Pem,
            guest_types::PublickeyEncoding::Sec => PublicKeyEncoding::Sec,
            guest_types::PublickeyEncoding::CompressedSec => PublicKeyEncoding::CompressedSec,
        }
    }
}

#[derive(Clone, Debug)]
pub enum PublicKey {
    Signature(SignaturePublicKey),
}

impl PublicKey {
    pub(crate) fn into_signature_public_key(self) -> Result<SignaturePublicKey, CryptoError> {
        match self {
            PublicKey::Signature(pk) => Ok(pk),
        }
    }

    fn import(
        alg_type: AlgorithmType,
        alg_str: &str,
        encoded: &[u8],
        encoding: PublicKeyEncoding,
    ) -> Result<PublicKey, CryptoError> {
        match alg_type {
            AlgorithmType::Signatures => Ok(PublicKey::Signature(SignaturePublicKey::import(
                SignatureAlgorithm::try_from(alg_str)?,
                encoded,
                encoding,
            )?)),
            _ => bail!(CryptoError::InvalidOperation),
        }
    }

    fn export(pk: PublicKey, encoding: PublicKeyEncoding) -> Result<Vec<u8>, CryptoError> {
        match pk {
            PublicKey::Signature(pk) => SignaturePublicKey::export(pk, encoding),
        }
    }

    fn from_secretkey(sk: SecretKey) -> Result<PublicKey, CryptoError> {
        match sk {
            SecretKey::Signature(sk) => Ok(PublicKey::Signature(
                SignaturePublicKey::from_secretkey(sk)?,
            )),
        }
    }

    fn verify(handles: &HandleManagers, pk_handle: Handle) -> Result<(), CryptoError> {
        match handles.publickey.get(pk_handle)? {
            PublicKey::Signature(pk) => SignaturePublicKey::verify(pk),
        }
    }
}

impl CryptoCtx {
    pub fn publickey_import(
        &self,
        alg_type: AlgorithmType,
        alg_str: &str,
        encoded: &[u8],
        encoding: PublicKeyEncoding,
    ) -> Result<Handle, CryptoError> {
        let pk = PublicKey::import(alg_type, alg_str, encoded, encoding)?;
        let handle = self.handles.publickey.register(pk)?;
        Ok(handle)
    }

    pub fn publickey_export(
        &self,
        pk_handle: Handle,
        encoding: PublicKeyEncoding,
    ) -> Result<Handle, CryptoError> {
        let pk = self.handles.publickey.get(pk_handle)?;
        let encoded = PublicKey::export(pk, encoding)?;
        let array_output_handle = ArrayOutput::register(&self.handles, encoded)?;
        Ok(array_output_handle)
    }

    pub fn publickey_from_secretkey(&self, sk_handle: Handle) -> Result<Handle, CryptoError> {
        let sk = self.handles.secretkey.get(sk_handle)?;
        let pk = PublicKey::from_secretkey(sk)?;
        let handle = self.handles.publickey.register(pk)?;
        Ok(handle)
    }

    pub fn publickey_verify(&self, pk: Handle) -> Result<(), CryptoError> {
        PublicKey::verify(&self.handles, pk)
    }

    pub fn publickey_close(&self, pk: Handle) -> Result<(), CryptoError> {
        self.handles.publickey.close(pk)
    }
}
