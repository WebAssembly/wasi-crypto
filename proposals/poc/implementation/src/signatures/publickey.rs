use super::ecdsa::*;
use super::eddsa::*;
use super::rsa::*;
use super::*;
use crate::asymmetric_common::*;
use crate::error::*;
#[derive(Clone, Debug)]
pub enum SignaturePublicKey {
    Ecdsa(EcdsaSignaturePublicKey),
    Eddsa(EddsaSignaturePublicKey),
    Rsa(RsaSignaturePublicKey),
}

impl SignaturePublicKey {
    pub fn alg(&self) -> SignatureAlgorithm {
        match self {
            SignaturePublicKey::Ecdsa(x) => x.alg,
            SignaturePublicKey::Eddsa(x) => x.alg,
            SignaturePublicKey::Rsa(x) => x.alg,
        }
    }

    pub(crate) fn import(
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

    pub(crate) fn export(
        pk: SignaturePublicKey,
        encoding: PublicKeyEncoding,
    ) -> Result<Vec<u8>, CryptoError> {
        match encoding {
            PublicKeyEncoding::Raw => {}
            _ => bail!(CryptoError::UnsupportedEncoding),
        }
        let raw_pk = match pk {
            SignaturePublicKey::Ecdsa(pk) => pk.as_raw()?.to_vec(),
            SignaturePublicKey::Eddsa(pk) => pk.as_raw()?.to_vec(),
            SignaturePublicKey::Rsa(pk) => pk.as_raw()?.to_vec(),
        };
        Ok(raw_pk)
    }

    pub(crate) fn from_secretkey(
        _sk: SignatureSecretKey,
    ) -> Result<SignaturePublicKey, CryptoError> {
        bail!(CryptoError::NotImplemented)
    }

    pub(crate) fn verify(_pk: SignaturePublicKey) -> Result<(), CryptoError> {
        bail!(CryptoError::NotImplemented)
    }
}
