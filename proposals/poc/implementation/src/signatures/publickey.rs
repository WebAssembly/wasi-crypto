use super::ecdsa::*;
use super::eddsa::*;
use super::rsa::*;
use super::*;
use crate::array_output::*;
use crate::error::*;
use crate::handles::*;
use crate::types as guest_types;
use crate::{CryptoCtx, HandleManagers};

use std::convert::TryFrom;

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
pub enum SignaturePublicKey {
    Ecdsa(EcdsaSignaturePublicKey),
    Eddsa(EddsaSignaturePublicKey),
    Rsa(RsaSignaturePublicKey),
}

impl SignaturePublicKey {
    fn import(
        alg: SignatureAlgorithm,
        encoded: &[u8],
        encoding: PublicKeyEncoding,
    ) -> Result<SignaturePublicKey, CryptoError> {
        match encoding {
            PublicKeyEncoding::Raw => {}
            _ => bail!(CryptoError::UnsupportedEncoding),
        }
        let pk = match alg {
            SignatureAlgorithm::ECDSA_P256_SHA256 | SignatureAlgorithm::ECDSA_P384_SHA384 => {
                SignaturePublicKey::Ecdsa(EcdsaSignaturePublicKey::from_raw(alg, encoded)?)
            }
            SignatureAlgorithm::Ed25519 => {
                SignaturePublicKey::Eddsa(EddsaSignaturePublicKey::from_raw(alg, encoded)?)
            }
            SignatureAlgorithm::RSA_PKCS1_2048_8192_SHA256
            | SignatureAlgorithm::RSA_PKCS1_2048_8192_SHA384
            | SignatureAlgorithm::RSA_PKCS1_2048_8192_SHA512
            | SignatureAlgorithm::RSA_PKCS1_3072_8192_SHA384 => {
                SignaturePublicKey::Rsa(RsaSignaturePublicKey::from_raw(alg, encoded)?)
            }
        };
        Ok(pk)
    }

    fn export(
        handles: &HandleManagers,
        pk: Handle,
        encoding: PublicKeyEncoding,
    ) -> Result<Vec<u8>, CryptoError> {
        match encoding {
            PublicKeyEncoding::Raw => {}
            _ => bail!(CryptoError::UnsupportedEncoding),
        }
        let pk = handles.signature_publickey.get(pk)?;
        let raw_pk = match pk {
            SignaturePublicKey::Ecdsa(pk) => pk.as_raw()?.to_vec(),
            SignaturePublicKey::Eddsa(pk) => pk.as_raw()?.to_vec(),
            SignaturePublicKey::Rsa(pk) => pk.as_raw()?.to_vec(),
        };
        Ok(raw_pk)
    }

    fn verify(_pk_handle: Handle) -> Result<(), CryptoError> {
        bail!(CryptoError::NotImplemented)
    }
}

impl CryptoCtx {
    pub fn signature_publickey_import(
        &self,
        alg_str: &str,
        encoded: &[u8],
        encoding: PublicKeyEncoding,
    ) -> Result<Handle, CryptoError> {
        let alg = SignatureAlgorithm::try_from(alg_str)?;
        let pk = SignaturePublicKey::import(alg, encoded, encoding)?;
        let handle = self.handles.signature_publickey.register(pk)?;
        Ok(handle)
    }

    pub fn signature_publickey_export(
        &self,
        pk: Handle,
        encoding: PublicKeyEncoding,
    ) -> Result<Handle, CryptoError> {
        let encoded = SignaturePublicKey::export(&self.handles, pk, encoding)?;
        let array_output_handle = ArrayOutput::register(&self.handles, encoded)?;
        Ok(array_output_handle)
    }

    pub fn signature_publickey_verify(&self, pk: Handle) -> Result<(), CryptoError> {
        SignaturePublicKey::verify(pk)
    }

    pub fn signature_publickey_close(&self, pk: Handle) -> Result<(), CryptoError> {
        self.handles.signature_publickey.close(pk)
    }
}
