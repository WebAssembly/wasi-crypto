use super::state::*;
use super::*;

use ring::rand::SecureRandom;
use zeroize::Zeroize;

#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct HkdfSymmetricState {
    pub alg: SymmetricAlgorithm,
    options: Option<SymmetricOptions>,
    key: Vec<u8>,
    data: Vec<u8>,
}

impl Drop for HkdfSymmetricState {
    fn drop(&mut self) {
        self.key.zeroize();
    }
}

#[derive(Clone, Debug, Eq)]
pub struct HkdfSymmetricKey {
    alg: SymmetricAlgorithm,
    raw: Vec<u8>,
}

impl PartialEq for HkdfSymmetricKey {
    fn eq(&self, other: &Self) -> bool {
        self.alg == other.alg
            && ring::constant_time::verify_slices_are_equal(&self.raw, &other.raw).is_ok()
    }
}

impl Drop for HkdfSymmetricKey {
    fn drop(&mut self) {
        self.raw.zeroize();
    }
}

impl HkdfSymmetricKey {
    pub fn new(alg: SymmetricAlgorithm, raw: &[u8]) -> Result<Self, CryptoError> {
        Ok(HkdfSymmetricKey {
            alg,
            raw: raw.to_vec(),
        })
    }

    pub fn alg(&self) -> SymmetricAlgorithm {
        self.alg
    }

    pub fn as_raw(&self) -> Result<&[u8], CryptoError> {
        Ok(&self.raw)
    }

    pub fn generate(
        alg: SymmetricAlgorithm,
        _options: Option<SymmetricOptions>,
    ) -> Result<HkdfSymmetricKey, CryptoError> {
        let key_len = match alg {
            SymmetricAlgorithm::HkdfSha256Expand | SymmetricAlgorithm::HkdfSha256Extract => {
                ring::digest::SHA256_OUTPUT_LEN
            }
            SymmetricAlgorithm::HkdfSha512Expand | SymmetricAlgorithm::HkdfSha512Extract => {
                ring::digest::SHA512_OUTPUT_LEN
            }
            _ => bail!(CryptoError::UnsupportedAlgorithm),
        };
        let rng = ring::rand::SystemRandom::new();
        let mut raw = vec![0u8; key_len];
        rng.fill(&mut raw).map_err(|_| CryptoError::RNGError)?;
        Self::import(alg, &raw)
    }

    pub fn import(alg: SymmetricAlgorithm, raw: &[u8]) -> Result<HkdfSymmetricKey, CryptoError> {
        let key = HkdfSymmetricKey::new(alg, raw)?;
        Ok(key)
    }
}

impl HkdfSymmetricState {
    pub fn new(
        alg: SymmetricAlgorithm,
        key: Option<SymmetricKey>,
        options: Option<SymmetricOptions>,
    ) -> Result<Self, CryptoError> {
        let key = match key {
            None => bail!(CryptoError::KeyRequired),
            Some(SymmetricKey::Hkdf(key)) => key,
            _ => bail!(CryptoError::InvalidKey),
        };
        let key = key.as_raw()?.to_vec();
        Ok(HkdfSymmetricState {
            alg,
            options,
            key: key,
            data: vec![],
        })
    }
}

impl SymmetricStateLike for HkdfSymmetricState {
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
        self.data.extend_from_slice(data);
        Ok(())
    }

    fn squeeze_key(&mut self, out: &mut [u8]) -> Result<(), CryptoError> {
        let ring_alg = match self.alg {
            SymmetricAlgorithm::HkdfSha256Extract => ring::hmac::HMAC_SHA256,
            SymmetricAlgorithm::HkdfSha512Extract => ring::hmac::HMAC_SHA512,
            _ => bail!(CryptoError::InvalidOperation),
        };
        let ring_salt = ring::hmac::Key::new(ring_alg, &self.data);
        let ring_prk = ring::hmac::sign(&ring_salt, &self.key);
        let raw = ring_prk.as_ref();
        ensure!(out.len() >= raw.len(), CryptoError::Overflow);
        out.copy_from_slice(raw);
        Ok(())
    }

    fn squeeze(&mut self, out: &mut [u8]) -> Result<(), CryptoError> {
        let ring_alg = match self.alg {
            SymmetricAlgorithm::HkdfSha256Expand => ring::hkdf::HKDF_SHA256,
            SymmetricAlgorithm::HkdfSha512Expand => ring::hkdf::HKDF_SHA512,
            _ => bail!(CryptoError::InvalidOperation),
        };
        let ring_prk = ring::hkdf::Prk::new_less_safe(ring_alg, &self.key);
        let ring_data = [self.data.as_ref()];
        let ring_ohm = ring_prk
            .expand(&ring_data, HkdfOutLen::new(out.len()))
            .map_err(|_| CryptoError::Overflow)?;
        ring_ohm.fill(out).map_err(|_| CryptoError::Overflow)?;
        Ok(())
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct HkdfOutLen(usize);

impl HkdfOutLen {
    fn new(len: usize) -> Self {
        HkdfOutLen(len)
    }
}

impl ring::hkdf::KeyType for HkdfOutLen {
    fn len(&self) -> usize {
        self.0
    }
}
