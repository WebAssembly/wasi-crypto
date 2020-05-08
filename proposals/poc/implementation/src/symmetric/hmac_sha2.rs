use super::state::*;
use super::*;

use parking_lot::Mutex;
use ring::rand::SecureRandom;
use std::sync::Arc;
use zeroize::Zeroize;

#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct HmacSha2SymmetricState {
    pub alg: SymmetricAlgorithm,
    options: Option<SymmetricOptions>,
    #[derivative(Debug = "ignore")]
    pub ring_ctx: Arc<Mutex<ring::hmac::Context>>,
}

#[derive(Clone, Debug, Eq)]
pub struct HmacSha2SymmetricKey {
    alg: SymmetricAlgorithm,
    raw: Vec<u8>,
}

impl PartialEq for HmacSha2SymmetricKey {
    fn eq(&self, other: &Self) -> bool {
        self.alg == other.alg
            && ring::constant_time::verify_slices_are_equal(&self.raw, &other.raw).is_ok()
    }
}

impl Drop for HmacSha2SymmetricKey {
    fn drop(&mut self) {
        self.raw.zeroize();
    }
}

impl SymmetricKeyLike for HmacSha2SymmetricKey {
    fn alg(&self) -> SymmetricAlgorithm {
        self.alg
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_raw(&self) -> Result<&[u8], CryptoError> {
        Ok(&self.raw)
    }
}

impl HmacSha2SymmetricKey {
    pub fn new(alg: SymmetricAlgorithm, raw: &[u8]) -> Result<Self, CryptoError> {
        Ok(HmacSha2SymmetricKey {
            alg,
            raw: raw.to_vec(),
        })
    }
}

pub struct HmacSha2SymmetricKeyBuilder {
    alg: SymmetricAlgorithm,
}

impl HmacSha2SymmetricKeyBuilder {
    pub fn new(alg: SymmetricAlgorithm) -> Box<dyn SymmetricKeyBuilder> {
        Box::new(Self { alg })
    }
}

impl SymmetricKeyBuilder for HmacSha2SymmetricKeyBuilder {
    fn generate(&self, _options: Option<SymmetricOptions>) -> Result<SymmetricKey, CryptoError> {
        let rng = ring::rand::SystemRandom::new();
        let mut raw = vec![0u8; self.key_len()?];
        rng.fill(&mut raw).map_err(|_| CryptoError::RNGError)?;
        self.import(&raw)
    }

    fn import(&self, raw: &[u8]) -> Result<SymmetricKey, CryptoError> {
        let key = HmacSha2SymmetricKey::new(self.alg, raw)?;
        Ok(SymmetricKey::new(Box::new(key)))
    }

    fn key_len(&self) -> Result<usize, CryptoError> {
        match self.alg {
            SymmetricAlgorithm::HmacSha256 => Ok(ring::digest::SHA256_OUTPUT_LEN),
            SymmetricAlgorithm::HmacSha512 => Ok(ring::digest::SHA512_OUTPUT_LEN),
            _ => bail!(CryptoError::UnsupportedAlgorithm),
        }
    }
}

impl HmacSha2SymmetricState {
    pub fn new(
        alg: SymmetricAlgorithm,
        key: Option<SymmetricKey>,
        options: Option<SymmetricOptions>,
    ) -> Result<Self, CryptoError> {
        let key = key.ok_or(CryptoError::KeyRequired)?;
        let key = key.inner();
        let key = key
            .as_any()
            .downcast_ref::<HmacSha2SymmetricKey>()
            .ok_or(CryptoError::InvalidKey)?;
        let ring_alg = match alg {
            SymmetricAlgorithm::HmacSha256 => ring::hmac::HMAC_SHA256,
            SymmetricAlgorithm::HmacSha512 => ring::hmac::HMAC_SHA512,
            _ => bail!(CryptoError::UnsupportedAlgorithm),
        };
        let ring_key = ring::hmac::Key::new(ring_alg, key.as_raw()?);
        let ring_ctx = ring::hmac::Context::with_key(&ring_key);
        Ok(HmacSha2SymmetricState {
            alg,
            options,
            ring_ctx: Arc::new(Mutex::new(ring_ctx)),
        })
    }
}

impl SymmetricStateLike for HmacSha2SymmetricState {
    fn alg(&self) -> SymmetricAlgorithm {
        self.alg
    }

    fn options_get(&self, name: &str) -> Result<Vec<u8>, CryptoError> {
        self.options
            .as_ref()
            .ok_or(CryptoError::OptionNotSet)?
            .get(name)
    }

    fn options_get_u64(&self, name: &str) -> Result<u64, CryptoError> {
        self.options
            .as_ref()
            .ok_or(CryptoError::OptionNotSet)?
            .get_u64(name)
    }

    fn absorb(&mut self, data: &[u8]) -> Result<(), CryptoError> {
        self.ring_ctx.lock().update(data);
        Ok(())
    }

    fn squeeze_tag(&mut self) -> Result<SymmetricTag, CryptoError> {
        let raw = self.ring_ctx.lock().clone().sign().as_ref().to_vec();
        Ok(SymmetricTag::new(self.alg, raw))
    }
}
