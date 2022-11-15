use ::sha2::{Digest, Sha256, Sha384, Sha512, Sha512_256};

use super::state::*;
use super::*;

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
enum HashVariant {
    Sha256(Sha256),
    Sha384(Sha384),
    Sha512(Sha512),
    Sha512_256(Sha512_256),
}

#[derive(Derivative, Clone)]
#[derivative(Debug)]
pub struct Sha2SymmetricState {
    pub alg: SymmetricAlgorithm,
    options: Option<SymmetricOptions>,
    size_limit: Option<usize>,
    ctx: HashVariant,
}

impl Sha2SymmetricState {
    pub fn new(
        alg: SymmetricAlgorithm,
        key: Option<&SymmetricKey>,
        options: Option<SymmetricOptions>,
        size_limit: Option<usize>,
    ) -> Result<Self, CryptoError> {
        if key.is_some() {
            return Err(CryptoError::KeyNotSupported);
        }
        let ctx = match alg {
            SymmetricAlgorithm::Sha256 => HashVariant::Sha256(Sha256::new()),
            SymmetricAlgorithm::Sha384 => HashVariant::Sha384(Sha384::new()),
            SymmetricAlgorithm::Sha512 => HashVariant::Sha512(Sha512::new()),
            SymmetricAlgorithm::Sha512_256 => HashVariant::Sha512_256(Sha512_256::new()),
            _ => bail!(CryptoError::UnsupportedAlgorithm),
        };
        Ok(Sha2SymmetricState {
            alg,
            options,
            size_limit,
            ctx,
        })
    }
}

impl SymmetricStateLike for Sha2SymmetricState {
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

    fn size_limit(&self) -> Option<usize> {
        self.size_limit
    }

    fn absorb_unchecked(&mut self, data: &[u8]) -> Result<(), CryptoError> {
        match &mut self.ctx {
            HashVariant::Sha256(x) => x.update(data),
            HashVariant::Sha384(x) => x.update(data),
            HashVariant::Sha512(x) => x.update(data),
            HashVariant::Sha512_256(x) => x.update(data),
        };
        Ok(())
    }

    fn squeeze_unchecked(&mut self, out: &mut [u8]) -> Result<(), CryptoError> {
        let raw = match &self.ctx {
            HashVariant::Sha256(x) => x.clone().finalize().to_vec(),
            HashVariant::Sha384(x) => x.clone().finalize().to_vec(),
            HashVariant::Sha512(x) => x.clone().finalize().to_vec(),
            HashVariant::Sha512_256(x) => x.clone().finalize().to_vec(),
        };
        ensure!(raw.len() >= out.len(), CryptoError::InvalidLength);
        out.copy_from_slice(&raw[..out.len()]);
        Ok(())
    }
}
