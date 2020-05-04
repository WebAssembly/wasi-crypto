use super::ecdsa::*;
use super::eddsa::*;
use super::publickey::*;
use super::rsa::*;
use super::*;
use crate::array_output::*;
use crate::error::*;
use crate::handles::*;
use crate::types as guest_types;
use crate::version::Version;
use crate::CryptoCtx;

use std::convert::TryFrom;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KeyPairEncoding {
    Raw,
    Pkcs8,
    Der,
    Pem,
}

impl From<guest_types::KeypairEncoding> for KeyPairEncoding {
    fn from(encoding: guest_types::KeypairEncoding) -> Self {
        match encoding {
            guest_types::KeypairEncoding::Raw => KeyPairEncoding::Raw,
            guest_types::KeypairEncoding::Pkcs8 => KeyPairEncoding::Pkcs8,
            guest_types::KeypairEncoding::Der => KeyPairEncoding::Der,
            guest_types::KeypairEncoding::Pem => KeyPairEncoding::Pem,
        }
    }
}

#[derive(Clone, Debug)]
pub enum SignatureKeyPair {
    Ecdsa(EcdsaSignatureKeyPair),
    Eddsa(EddsaSignatureKeyPair),
    Rsa(RsaSignatureKeyPair),
}

impl SignatureKeyPair {
    fn export(&self, encoding: KeyPairEncoding) -> Result<Vec<u8>, CryptoError> {
        let encoded = match encoding {
            KeyPairEncoding::Pkcs8 => match self {
                SignatureKeyPair::Ecdsa(kp) => kp.as_pkcs8()?.to_vec(),
                SignatureKeyPair::Eddsa(kp) => kp.as_pkcs8()?.to_vec(),
                SignatureKeyPair::Rsa(kp) => kp.as_pkcs8()?.to_vec(),
            },
            _ => bail!(CryptoError::UnsupportedEncoding),
        };
        Ok(encoded)
    }

    fn generate(
        alg_str: &str,
        options: Option<SignatureOptions>,
    ) -> Result<SignatureKeyPair, CryptoError> {
        let alg = SignatureAlgorithm::try_from(alg_str)?;
        let kp = match alg {
            SignatureAlgorithm::ECDSA_P256_SHA256 | SignatureAlgorithm::ECDSA_P384_SHA384 => {
                SignatureKeyPair::Ecdsa(EcdsaSignatureKeyPair::generate(alg, options)?)
            }
            SignatureAlgorithm::Ed25519 => {
                SignatureKeyPair::Eddsa(EddsaSignatureKeyPair::generate(alg, options)?)
            }
            SignatureAlgorithm::RSA_PKCS1_2048_8192_SHA256
            | SignatureAlgorithm::RSA_PKCS1_2048_8192_SHA384
            | SignatureAlgorithm::RSA_PKCS1_2048_8192_SHA512
            | SignatureAlgorithm::RSA_PKCS1_3072_8192_SHA384 => {
                SignatureKeyPair::Rsa(RsaSignatureKeyPair::generate(alg, options)?)
            }
        };
        Ok(kp)
    }

    fn import(
        alg_str: &str,
        encoded: &[u8],
        encoding: KeyPairEncoding,
    ) -> Result<SignatureKeyPair, CryptoError> {
        let alg = SignatureAlgorithm::try_from(alg_str)?;
        let kp = match alg {
            SignatureAlgorithm::ECDSA_P256_SHA256 | SignatureAlgorithm::ECDSA_P384_SHA384 => {
                SignatureKeyPair::Ecdsa(EcdsaSignatureKeyPair::import(alg, encoded, encoding)?)
            }
            SignatureAlgorithm::Ed25519 => {
                SignatureKeyPair::Eddsa(EddsaSignatureKeyPair::import(alg, encoded, encoding)?)
            }
            SignatureAlgorithm::RSA_PKCS1_2048_8192_SHA256
            | SignatureAlgorithm::RSA_PKCS1_2048_8192_SHA384
            | SignatureAlgorithm::RSA_PKCS1_2048_8192_SHA512
            | SignatureAlgorithm::RSA_PKCS1_3072_8192_SHA384 => {
                SignatureKeyPair::Rsa(RsaSignatureKeyPair::import(alg, encoded, encoding)?)
            }
        };
        Ok(kp)
    }

    fn public_key(&self) -> Result<SignaturePublicKey, CryptoError> {
        let pk = match self {
            SignatureKeyPair::Ecdsa(kp) => {
                let raw_pk = kp.raw_public_key();
                SignaturePublicKey::Ecdsa(EcdsaSignaturePublicKey::from_raw(kp.alg, raw_pk)?)
            }
            SignatureKeyPair::Eddsa(kp) => {
                let raw_pk = kp.raw_public_key();
                SignaturePublicKey::Eddsa(EddsaSignaturePublicKey::from_raw(kp.alg, raw_pk)?)
            }
            SignatureKeyPair::Rsa(kp) => {
                let raw_pk = kp.raw_public_key();
                SignaturePublicKey::Rsa(RsaSignaturePublicKey::from_raw(kp.alg, raw_pk)?)
            }
        };
        Ok(pk)
    }
}

impl CryptoCtx {
    pub fn signature_keypair_generate(
        &self,
        alg_str: &str,
        options_handle: Option<Handle>,
    ) -> Result<Handle, CryptoError> {
        let options = match options_handle {
            None => None,
            Some(options_handle) => Some(
                self.handles
                    .options
                    .get(options_handle)?
                    .into_signatures()?,
            ),
        };
        let kp = SignatureKeyPair::generate(alg_str, options)?;
        let handle = self.handles.signature_keypair.register(kp)?;
        Ok(handle)
    }

    pub fn signature_keypair_import(
        &self,
        alg_str: &str,
        encoded: &[u8],
        encoding: KeyPairEncoding,
    ) -> Result<Handle, CryptoError> {
        let kp = SignatureKeyPair::import(alg_str, encoded, encoding)?;
        let handle = self.handles.signature_keypair.register(kp)?;
        Ok(handle)
    }

    pub fn signature_keypair_id(
        &self,
        kp_handle: Handle,
    ) -> Result<(Vec<u8>, Version), CryptoError> {
        let _kp = self.handles.signature_keypair.get(kp_handle)?;
        bail!(CryptoError::UnsupportedFeature)
    }

    pub fn signature_keypair_export(
        &self,
        kp_handle: Handle,
        encoding: KeyPairEncoding,
    ) -> Result<Handle, CryptoError> {
        let kp = self.handles.signature_keypair.get(kp_handle)?;
        let encoded = kp.export(encoding)?;
        let array_output_handle = ArrayOutput::register(&self.handles, encoded)?;
        Ok(array_output_handle)
    }

    pub fn signature_keypair_publickey(&self, kp_handle: Handle) -> Result<Handle, CryptoError> {
        let kp = self.handles.signature_keypair.get(kp_handle)?;
        let pk = kp.public_key()?;
        let handle = self.handles.signature_publickey.register(pk)?;
        Ok(handle)
    }

    pub fn signature_keypair_close(&self, kp_handle: Handle) -> Result<(), CryptoError> {
        self.handles.signature_keypair.close(kp_handle)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct SignatureKeyPairManager;
