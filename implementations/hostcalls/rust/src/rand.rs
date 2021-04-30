use crate::CryptoError;
use rand_core::{self, CryptoRng, OsRng, RngCore};
use rand_core_05::{self, CryptoRng as CryptoRng05, OsRng as OsRng05, RngCore as RngCore05};

pub struct SecureRandom;

impl SecureRandom {
    pub fn new() -> Self {
        SecureRandom
    }

    pub fn fill(&mut self, bytes: &mut [u8]) -> Result<(), CryptoError> {
        OsRng
            .try_fill_bytes(bytes)
            .map_err(|_| CryptoError::RNGError)
    }
}

impl CryptoRng for SecureRandom {}
impl CryptoRng05 for SecureRandom {}

impl RngCore for SecureRandom {
    fn next_u32(&mut self) -> u32 {
        OsRng.next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        OsRng.next_u64()
    }

    fn fill_bytes(&mut self, bytes: &mut [u8]) {
        OsRng.fill_bytes(bytes);
    }

    fn try_fill_bytes(&mut self, bytes: &mut [u8]) -> Result<(), rand_core::Error> {
        OsRng.try_fill_bytes(bytes)
    }
}

impl RngCore05 for SecureRandom {
    fn next_u32(&mut self) -> u32 {
        OsRng05.next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        OsRng05.next_u64()
    }

    fn fill_bytes(&mut self, bytes: &mut [u8]) {
        OsRng05.fill_bytes(bytes);
    }

    fn try_fill_bytes(&mut self, bytes: &mut [u8]) -> Result<(), rand_core_05::Error> {
        OsRng05.try_fill_bytes(bytes)
    }
}
