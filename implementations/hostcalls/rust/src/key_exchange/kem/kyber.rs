use pqcrypto_kyber::{kyber1024, kyber768};
use pqcrypto_traits::kem::*;

use super::*;

#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct Kyber768PublicKey {
    alg: KxAlgorithm,
    #[derivative(Debug = "ignore")]
    pq_pk: kyber768::PublicKey,
}

impl Kyber768PublicKey {
    fn new(alg: KxAlgorithm, raw: &[u8]) -> Result<Self, CryptoError> {
        ensure!(
            raw.len() == kyber768::public_key_bytes(),
            CryptoError::InvalidKey
        );
        let mut raw_ = [0u8; kyber768::public_key_bytes()];
        raw_.copy_from_slice(raw);
        let pq_pk = kyber768::PublicKey::from_bytes(raw).map_err(|_| CryptoError::InvalidKey)?;
        Ok(Kyber768PublicKey { alg, pq_pk })
    }
}

#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct Kyber768SecretKey {
    alg: KxAlgorithm,
    #[derivative(Debug = "ignore")]
    pq_sk: kyber768::SecretKey,
}

impl Kyber768SecretKey {
    fn new(alg: KxAlgorithm, raw: Vec<u8>) -> Result<Self, CryptoError> {
        ensure!(
            raw.len() == kyber768::secret_key_bytes(),
            CryptoError::InvalidKey
        );
        let mut raw_ = [0u8; kyber768::secret_key_bytes()];
        raw_.copy_from_slice(&raw);
        let pq_sk = kyber768::SecretKey::from_bytes(&raw).map_err(|_| CryptoError::InvalidKey)?;
        Ok(Kyber768SecretKey { alg, pq_sk })
    }
}

#[derive(Clone, Debug)]
pub struct Kyber768KeyPair {
    alg: KxAlgorithm,
    pk: Kyber768PublicKey,
    sk: Kyber768SecretKey,
}

pub struct Kyber768KeyPairBuilder {
    alg: KxAlgorithm,
}

impl Kyber768KeyPairBuilder {
    pub fn new(alg: KxAlgorithm) -> Box<dyn KxKeyPairBuilder> {
        Box::new(Self { alg })
    }
}

impl KxKeyPairBuilder for Kyber768KeyPairBuilder {
    fn generate(&self, _options: Option<KxOptions>) -> Result<KxKeyPair, CryptoError> {
        let (pq_pk, pq_sk) = kyber768::keypair();
        let pk = Kyber768PublicKey {
            alg: self.alg,
            pq_pk,
        };
        let sk = Kyber768SecretKey {
            alg: self.alg,
            pq_sk,
        };
        let kp = Kyber768KeyPair {
            alg: self.alg,
            pk,
            sk,
        };
        Ok(KxKeyPair::new(Box::new(kp)))
    }
}

//

pub struct Kyber768SecretKeyBuilder {
    alg: KxAlgorithm,
}

impl KxSecretKeyBuilder for Kyber768SecretKeyBuilder {
    fn from_raw(&self, raw: &[u8]) -> Result<KxSecretKey, CryptoError> {
        ensure!(
            raw.len() == kyber768::secret_key_bytes(),
            CryptoError::InvalidKey
        );
        let sk = Kyber768SecretKey::new(self.alg, raw.to_vec())?;
        Ok(KxSecretKey::new(Box::new(sk)))
    }
}

impl Kyber768SecretKeyBuilder {
    pub fn new(alg: KxAlgorithm) -> Box<dyn KxSecretKeyBuilder> {
        Box::new(Self { alg })
    }
}

pub struct Kyber768PublicKeyBuilder {
    alg: KxAlgorithm,
}

impl KxPublicKeyBuilder for Kyber768PublicKeyBuilder {
    fn from_raw(&self, raw: &[u8]) -> Result<KxPublicKey, CryptoError> {
        ensure!(
            raw.len() == kyber768::public_key_bytes(),
            CryptoError::InvalidKey
        );
        let pk = Kyber768PublicKey::new(self.alg, raw)?;
        Ok(KxPublicKey::new(Box::new(pk)))
    }
}

impl Kyber768PublicKeyBuilder {
    pub fn new(alg: KxAlgorithm) -> Box<dyn KxPublicKeyBuilder> {
        Box::new(Self { alg })
    }
}

impl KxKeyPairLike for Kyber768KeyPair {
    fn alg(&self) -> KxAlgorithm {
        self.alg
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn publickey(&self) -> Result<KxPublicKey, CryptoError> {
        Ok(KxPublicKey::new(Box::new(self.pk.clone())))
    }

    fn secretkey(&self) -> Result<KxSecretKey, CryptoError> {
        Ok(KxSecretKey::new(Box::new(self.sk.clone())))
    }
}

impl KxPublicKeyLike for Kyber768PublicKey {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn alg(&self) -> KxAlgorithm {
        self.alg
    }

    fn len(&self) -> Result<usize, CryptoError> {
        Ok(kyber768::public_key_bytes())
    }

    fn as_raw(&self) -> Result<&[u8], CryptoError> {
        Ok(self.pq_pk.as_bytes())
    }

    fn verify(&self) -> Result<(), CryptoError> {
        Ok(())
    }

    fn encapsulate(&self) -> Result<EncapsulatedSecret, CryptoError> {
        let (secret, encapsulated_secret) = kyber768::encapsulate(&self.pq_pk);
        Ok(EncapsulatedSecret {
            secret: secret.as_bytes().to_vec(),
            encapsulated_secret: encapsulated_secret.as_bytes().to_vec(),
        })
    }
}

impl Kyber768SecretKey {
    fn kyber768_publickey(&self) -> Result<Kyber768PublicKey, CryptoError> {
        bail!(CryptoError::UnsupportedFeature);
    }
}

impl KxSecretKeyLike for Kyber768SecretKey {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn alg(&self) -> KxAlgorithm {
        self.alg
    }

    fn len(&self) -> Result<usize, CryptoError> {
        Ok(kyber768::secret_key_bytes())
    }

    fn as_raw(&self) -> Result<&[u8], CryptoError> {
        Ok(self.pq_sk.as_bytes())
    }

    fn publickey(&self) -> Result<KxPublicKey, CryptoError> {
        Ok(KxPublicKey::new(Box::new(self.kyber768_publickey()?)))
    }

    fn decapsulate(&self, encapsulated_secret: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let pq_encapsulated_secret = kyber768::Ciphertext::from_bytes(encapsulated_secret)
            .map_err(|_| CryptoError::VerificationFailed)?;
        Ok(kyber768::decapsulate(&pq_encapsulated_secret, &self.pq_sk)
            .as_bytes()
            .to_vec())
    }
}

//

#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct Kyber1024PublicKey {
    alg: KxAlgorithm,
    #[derivative(Debug = "ignore")]
    pq_pk: kyber1024::PublicKey,
}

impl Kyber1024PublicKey {
    fn new(alg: KxAlgorithm, raw: &[u8]) -> Result<Self, CryptoError> {
        ensure!(
            raw.len() == kyber1024::public_key_bytes(),
            CryptoError::InvalidKey
        );
        let mut raw_ = [0u8; kyber1024::public_key_bytes()];
        raw_.copy_from_slice(raw);
        let pq_pk = kyber1024::PublicKey::from_bytes(raw).map_err(|_| CryptoError::InvalidKey)?;
        Ok(Kyber1024PublicKey { alg, pq_pk })
    }
}

#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct Kyber1024SecretKey {
    alg: KxAlgorithm,
    #[derivative(Debug = "ignore")]
    pq_sk: kyber1024::SecretKey,
}

impl Kyber1024SecretKey {
    fn new(alg: KxAlgorithm, raw: Vec<u8>) -> Result<Self, CryptoError> {
        ensure!(
            raw.len() == kyber1024::secret_key_bytes(),
            CryptoError::InvalidKey
        );
        let mut raw_ = [0u8; kyber1024::secret_key_bytes()];
        raw_.copy_from_slice(&raw);
        let pq_sk = kyber1024::SecretKey::from_bytes(&raw).map_err(|_| CryptoError::InvalidKey)?;
        Ok(Kyber1024SecretKey { alg, pq_sk })
    }
}

#[derive(Clone, Debug)]
pub struct Kyber1024KeyPair {
    alg: KxAlgorithm,
    pk: Kyber1024PublicKey,
    sk: Kyber1024SecretKey,
}

pub struct Kyber1024KeyPairBuilder {
    alg: KxAlgorithm,
}

impl Kyber1024KeyPairBuilder {
    pub fn new(alg: KxAlgorithm) -> Box<dyn KxKeyPairBuilder> {
        Box::new(Self { alg })
    }
}

impl KxKeyPairBuilder for Kyber1024KeyPairBuilder {
    fn generate(&self, _options: Option<KxOptions>) -> Result<KxKeyPair, CryptoError> {
        let (pq_pk, pq_sk) = kyber1024::keypair();
        let pk = Kyber1024PublicKey {
            alg: self.alg,
            pq_pk,
        };
        let sk = Kyber1024SecretKey {
            alg: self.alg,
            pq_sk,
        };
        let kp = Kyber1024KeyPair {
            alg: self.alg,
            pk,
            sk,
        };
        Ok(KxKeyPair::new(Box::new(kp)))
    }
}

// Kyber-1024

pub struct Kyber1024SecretKeyBuilder {
    alg: KxAlgorithm,
}

impl KxSecretKeyBuilder for Kyber1024SecretKeyBuilder {
    fn from_raw(&self, raw: &[u8]) -> Result<KxSecretKey, CryptoError> {
        ensure!(
            raw.len() == kyber1024::secret_key_bytes(),
            CryptoError::InvalidKey
        );
        let sk = Kyber1024SecretKey::new(self.alg, raw.to_vec())?;
        Ok(KxSecretKey::new(Box::new(sk)))
    }
}

impl Kyber1024SecretKeyBuilder {
    pub fn new(alg: KxAlgorithm) -> Box<dyn KxSecretKeyBuilder> {
        Box::new(Self { alg })
    }
}

pub struct Kyber1024PublicKeyBuilder {
    alg: KxAlgorithm,
}

impl KxPublicKeyBuilder for Kyber1024PublicKeyBuilder {
    fn from_raw(&self, raw: &[u8]) -> Result<KxPublicKey, CryptoError> {
        ensure!(
            raw.len() == kyber1024::public_key_bytes(),
            CryptoError::InvalidKey
        );
        let pk = Kyber1024PublicKey::new(self.alg, raw)?;
        Ok(KxPublicKey::new(Box::new(pk)))
    }
}

impl Kyber1024PublicKeyBuilder {
    pub fn new(alg: KxAlgorithm) -> Box<dyn KxPublicKeyBuilder> {
        Box::new(Self { alg })
    }
}

impl KxKeyPairLike for Kyber1024KeyPair {
    fn alg(&self) -> KxAlgorithm {
        self.alg
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn publickey(&self) -> Result<KxPublicKey, CryptoError> {
        Ok(KxPublicKey::new(Box::new(self.pk.clone())))
    }

    fn secretkey(&self) -> Result<KxSecretKey, CryptoError> {
        Ok(KxSecretKey::new(Box::new(self.sk.clone())))
    }
}

impl KxPublicKeyLike for Kyber1024PublicKey {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn alg(&self) -> KxAlgorithm {
        self.alg
    }

    fn len(&self) -> Result<usize, CryptoError> {
        Ok(kyber1024::public_key_bytes())
    }

    fn as_raw(&self) -> Result<&[u8], CryptoError> {
        Ok(self.pq_pk.as_bytes())
    }

    fn verify(&self) -> Result<(), CryptoError> {
        Ok(())
    }

    fn encapsulate(&self) -> Result<EncapsulatedSecret, CryptoError> {
        let (secret, encapsulated_secret) = kyber1024::encapsulate(&self.pq_pk);
        Ok(EncapsulatedSecret {
            secret: secret.as_bytes().to_vec(),
            encapsulated_secret: encapsulated_secret.as_bytes().to_vec(),
        })
    }
}

impl Kyber1024SecretKey {
    fn kyber1024_publickey(&self) -> Result<Kyber1024PublicKey, CryptoError> {
        bail!(CryptoError::UnsupportedFeature);
    }
}

impl KxSecretKeyLike for Kyber1024SecretKey {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn alg(&self) -> KxAlgorithm {
        self.alg
    }

    fn len(&self) -> Result<usize, CryptoError> {
        Ok(kyber1024::secret_key_bytes())
    }

    fn as_raw(&self) -> Result<&[u8], CryptoError> {
        Ok(self.pq_sk.as_bytes())
    }

    fn publickey(&self) -> Result<KxPublicKey, CryptoError> {
        Ok(KxPublicKey::new(Box::new(self.kyber1024_publickey()?)))
    }

    fn decapsulate(&self, encapsulated_secret: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let pq_encapsulated_secret = kyber1024::Ciphertext::from_bytes(encapsulated_secret)
            .map_err(|_| CryptoError::VerificationFailed)?;
        Ok(kyber1024::decapsulate(&pq_encapsulated_secret, &self.pq_sk)
            .as_bytes()
            .to_vec())
    }
}
