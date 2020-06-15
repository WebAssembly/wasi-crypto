use super::ecdsa::*;
use super::eddsa::*;
use super::rsa::*;
use super::*;

#[derive(Clone, Debug)]
pub enum SignatureSecretKey {
    Ecdsa(EcdsaSignatureSecretKey),
    Eddsa(EddsaSignatureSecretKey),
    Rsa(RsaSignatureSecretKey),
}

impl SignatureSecretKey {
    pub fn alg(&self) -> SignatureAlgorithm {
        match self {
            SignatureSecretKey::Ecdsa(x) => x.alg,
            SignatureSecretKey::Eddsa(x) => x.alg,
            SignatureSecretKey::Rsa(x) => x.alg,
        }
    }
}
