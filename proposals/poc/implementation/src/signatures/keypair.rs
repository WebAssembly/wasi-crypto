use super::ecdsa::*;
use super::eddsa::*;
use super::publickey::*;
use super::rsa::*;
use super::*;
use crate::asymmetric_common::KeyPairEncoding;
use crate::error::*;

#[derive(Clone, Debug)]
pub enum SignatureKeyPair {
    Ecdsa(EcdsaSignatureKeyPair),
    Eddsa(EddsaSignatureKeyPair),
    Rsa(RsaSignatureKeyPair),
}

impl SignatureKeyPair {
    pub(crate) fn export(&self, encoding: KeyPairEncoding) -> Result<Vec<u8>, CryptoError> {
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

    pub(crate) fn generate(
        alg: SignatureAlgorithm,
        options: Option<SignatureOptions>,
    ) -> Result<SignatureKeyPair, CryptoError> {
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

    pub(crate) fn import(
        alg: SignatureAlgorithm,
        encoded: &[u8],
        encoding: KeyPairEncoding,
    ) -> Result<SignatureKeyPair, CryptoError> {
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

    pub(crate) fn public_key(&self) -> Result<SignaturePublicKey, CryptoError> {
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
