use crate::error::*;
use crate::handles::*;
use crate::version::Version;
use crate::{AlgorithmType, CryptoCtx};

impl CryptoCtx {
    pub fn keypair_generate_managed(
        &self,
        _key_manager_handle: Handle,
        _alg_type: AlgorithmType,
        _alg_str: &str,
        _options_handle: Option<Handle>,
    ) -> Result<Handle, CryptoError> {
        bail!(CryptoError::UnsupportedFeature)
    }

    pub fn keypair_from_id(
        &self,
        _key_manager_handle: Handle,
        _symmetric_key_id: &[u8],
        _symmetric_key_version: Version,
    ) -> Result<Handle, CryptoError> {
        bail!(CryptoError::UnsupportedFeature)
    }
}
