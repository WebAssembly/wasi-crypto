use crate::error::*;
use crate::handles::*;
use crate::version::Version;
use crate::CryptoCtx;

impl CryptoCtx {
    pub fn symmetric_key_generate_managed(
        &self,
        _key_manager_handle: Handle,
        _alg_str: &str,
        _options_handle: Option<Handle>,
    ) -> Result<Handle, CryptoError> {
        bail!(CryptoError::UnsupportedFeature)
    }

    pub fn symmetric_key_replace_managed(
        &self,
        _key_manager_handle: Handle,
        kp_old_handle: Handle,
        kp_new_handle: Handle,
    ) -> Result<Version, CryptoError> {
        let _kp_old = self.handles.symmetric_key.get(kp_old_handle)?;
        let _kp_new = self.handles.symmetric_key.get(kp_new_handle)?;
        bail!(CryptoError::UnsupportedFeature)
    }

    pub fn symmetric_key_from_id(
        &self,
        _key_manager_handle: Handle,
        _symmetric_key_id: &[u8],
        _symmetric_key_version: Version,
    ) -> Result<Handle, CryptoError> {
        bail!(CryptoError::UnsupportedFeature)
    }
}
