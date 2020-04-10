use super::*;
use state::*;

use byteorder::{ByteOrder, LittleEndian};
use parking_lot::Mutex;
use ring::aead::BoundKey;
use ring::rand::SecureRandom;
use std::sync::Arc;
use zeroize::Zeroize;

pub struct AesGcmSymmetricStateInner {
    ring_sealing_key: ring::aead::SealingKey<AesGcmNonceSequence>,
    ring_opening_key: ring::aead::OpeningKey<AesGcmNonceSequence>,
    ad: Vec<u8>,
}

#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct AesGcmSymmetricState {
    pub alg: SymmetricAlgorithm,
    options: SymmetricOptions,
    #[derivative(Debug = "ignore")]
    inner: Arc<Mutex<AesGcmSymmetricStateInner>>,
}

#[derive(Clone, Debug, Eq)]
pub struct AesGcmSymmetricKey {
    alg: SymmetricAlgorithm,
    raw: Vec<u8>,
}

impl PartialEq for AesGcmSymmetricKey {
    fn eq(&self, other: &Self) -> bool {
        self.alg == other.alg
            && ring::constant_time::verify_slices_are_equal(&self.raw, &other.raw).is_ok()
    }
}

impl Drop for AesGcmSymmetricKey {
    fn drop(&mut self) {
        self.raw.zeroize();
    }
}

impl AesGcmSymmetricKey {
    pub fn new(alg: SymmetricAlgorithm, raw: &[u8]) -> Result<Self, CryptoError> {
        Ok(AesGcmSymmetricKey {
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
    ) -> Result<AesGcmSymmetricKey, CryptoError> {
        let key_len = match alg {
            SymmetricAlgorithm::Aes128Gcm => ring::aead::AES_128_GCM.key_len(),
            SymmetricAlgorithm::Aes256Gcm => ring::aead::AES_256_GCM.key_len(),
            _ => bail!(CryptoError::UnsupportedAlgorithm),
        };
        let rng = ring::rand::SystemRandom::new();
        let mut raw = vec![0u8; key_len];
        rng.fill(&mut raw).map_err(|_| CryptoError::RNGError)?;
        Self::import(alg, &raw)
    }

    pub fn import(alg: SymmetricAlgorithm, raw: &[u8]) -> Result<AesGcmSymmetricKey, CryptoError> {
        let key = AesGcmSymmetricKey::new(alg, raw)?;
        Ok(key)
    }
}

#[derive(Debug)]
pub struct AesGcmNonceSequence {
    nonce: [u8; ring::aead::NONCE_LEN],
}

impl AesGcmNonceSequence {
    fn new(nonce: [u8; ring::aead::NONCE_LEN]) -> Self {
        AesGcmNonceSequence { nonce }
    }
}

impl ring::aead::NonceSequence for AesGcmNonceSequence {
    fn advance(&mut self) -> Result<ring::aead::Nonce, ring::error::Unspecified> {
        debug_assert_eq!(self.nonce.len(), 12);
        let b0 = LittleEndian::read_u64(&self.nonce[..8]);
        let b1 = LittleEndian::read_u32(&self.nonce[8..]);
        let (b0, of) = b0.overflowing_add(1);
        let b1 = b1.wrapping_add(of as _);
        LittleEndian::write_u64(&mut self.nonce[..8], b0);
        LittleEndian::write_u32(&mut self.nonce[8..], b1);
        let ring_nonce = ring::aead::Nonce::assume_unique_for_key(self.nonce);
        Ok(ring_nonce)
    }
}

impl AesGcmSymmetricState {
    pub fn new(
        alg: SymmetricAlgorithm,
        key: Option<SymmetricKey>,
        options: Option<SymmetricOptions>,
    ) -> Result<Self, CryptoError> {
        let key = match key {
            None => bail!(CryptoError::KeyRequired),
            Some(SymmetricKey::AesGcm(key)) => key,
            _ => bail!(CryptoError::InvalidKey),
        };
        let ring_alg = match alg {
            SymmetricAlgorithm::Aes128Gcm => &ring::aead::AES_128_GCM,
            SymmetricAlgorithm::Aes256Gcm => &ring::aead::AES_256_GCM,
            _ => bail!(CryptoError::UnsupportedAlgorithm),
        };
        let options = options.as_ref().ok_or(CryptoError::NonceRequired)?;
        let inner = options.inner.lock();
        let nonce_vec = inner.nonce.as_ref().ok_or(CryptoError::NonceRequired)?;
        let mut nonce = [0u8; ring::aead::NONCE_LEN];
        nonce.copy_from_slice(&nonce_vec);
        let nonce_sequence = AesGcmNonceSequence::new(nonce);
        let ring_unbound_key = ring::aead::UnboundKey::new(ring_alg, key.as_raw()?)
            .map_err(|_| CryptoError::InvalidKey)?;
        let ring_sealing_key = ring::aead::SealingKey::new(ring_unbound_key, nonce_sequence);
        let nonce_sequence = AesGcmNonceSequence::new(nonce);
        let ring_unbound_key = ring::aead::UnboundKey::new(ring_alg, key.as_raw()?)
            .map_err(|_| CryptoError::InvalidKey)?;
        let ring_opening_key = ring::aead::OpeningKey::new(ring_unbound_key, nonce_sequence);
        let inner = AesGcmSymmetricStateInner {
            ring_sealing_key,
            ring_opening_key,
            ad: vec![],
        };
        Ok(AesGcmSymmetricState {
            alg,
            options: options.clone(),
            inner: Arc::new(Mutex::new(inner)),
        })
    }
}

impl SymmetricAlgorithmStateLike for AesGcmSymmetricState {
    fn alg(&self) -> SymmetricAlgorithm {
        self.alg
    }

    fn options_get(&self, name: &str) -> Result<Vec<u8>, CryptoError> {
        self.options.get(name)
    }

    fn options_get_u64(&self, name: &str) -> Result<u64, CryptoError> {
        self.options.get_u64(name)
    }

    fn absorb(&mut self, data: &[u8]) -> Result<(), CryptoError> {
        self.inner.lock().ad.extend_from_slice(data);
        Ok(())
    }

    fn max_tag_len(&mut self) -> Result<usize, CryptoError> {
        Ok(ring::aead::MAX_TAG_LEN)
    }

    fn encrypt(&mut self, out: &mut [u8], data: &[u8]) -> Result<usize, CryptoError> {
        let data_len = data.len();
        let tag = self.encrypt_detached(&mut out[..data_len], data)?;
        let out_len = data_len + tag.as_ref().len();
        out[data_len..out_len].copy_from_slice(tag.as_ref());
        Ok(out_len)
    }

    fn encrypt_detached(
        &mut self,
        out: &mut [u8],
        data: &[u8],
    ) -> Result<SymmetricTag, CryptoError> {
        out[..data.len()].copy_from_slice(data);
        let mut inner = self.inner.lock();
        let ring_ad = ring::aead::Aad::from(inner.ad.clone());
        let ring_tag = inner
            .ring_sealing_key
            .seal_in_place_separate_tag(ring_ad, out)
            .map_err(|_| CryptoError::AlgorithmFailure)?;
        let symmetric_tag = SymmetricTag::new(self.alg, ring_tag.as_ref().to_vec());
        Ok(symmetric_tag)
    }

    fn decrypt(&mut self, out: &mut [u8], data: &[u8]) -> Result<usize, CryptoError> {
        let mut in_out = data.to_vec();
        let mut inner = self.inner.lock();
        let ring_ad = ring::aead::Aad::from(inner.ad.clone());
        let out_len = inner
            .ring_opening_key
            .open_in_place(ring_ad, &mut in_out)
            .map_err(|_| CryptoError::InvalidTag)?
            .len();
        out[..out_len].copy_from_slice(&in_out[..out_len]);
        Ok(out_len)
    }

    fn decrypt_detached(
        &mut self,
        out: &mut [u8],
        data: &[u8],
        raw_tag: &[u8],
    ) -> Result<usize, CryptoError> {
        let mut in_out = data.to_vec();
        in_out.extend_from_slice(raw_tag);
        let mut inner = self.inner.lock();
        let ring_ad = ring::aead::Aad::from(inner.ad.clone());
        let out_len = inner
            .ring_opening_key
            .open_in_place(ring_ad, &mut in_out)
            .map_err(|_| CryptoError::InvalidTag)?
            .len();
        out[..out_len].copy_from_slice(&in_out[..out_len]);
        Ok(out_len)
    }
}
