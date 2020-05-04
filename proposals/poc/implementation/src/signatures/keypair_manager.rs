use crate::error::*;
use crate::handles::*;
use crate::version::Version;
use crate::CryptoCtx;

impl CryptoCtx {
    pub fn signature_managed_keypair_generate(
        &self,
        _key_manager_handle: Handle,
        _alg_str: &str,
        _options_handle: Option<Handle>,
    ) -> Result<Handle, CryptoError> {
        bail!(CryptoError::UnsupportedFeature)
    }

    pub fn signature_keypair_from_id(
        &self,
        _key_manager_handle: Handle,
        _kp_id: &[u8],
        _kp_version: Version,
    ) -> Result<Handle, CryptoError> {
        bail!(CryptoError::UnsupportedFeature)
    }
}
