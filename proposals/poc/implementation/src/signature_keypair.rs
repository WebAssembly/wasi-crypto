use super::array_output::*;
use super::ecdsa::*;
use super::eddsa::*;
use super::error::*;
use super::handles::*;
use super::rsa::*;
use super::signature_op::*;
use super::signature_publickey::*;
use super::{HandleManagers, WasiCryptoCtx};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Version(u64);

impl Version {
    pub const UNSPECIFIED: Version = Version(0xff00_0000_0000_0000);
    pub const LATEST: Version = Version(0xff00_0000_0000_0000);
    pub const ALL: Version = Version(0xff00_0000_0000_0000);
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u16)]
pub enum KeyPairEncoding {
    Raw = 1,
    PKCS8 = 2,
    DER = 3,
    PEM = 4,
}

#[derive(Clone, Debug)]
pub enum SignatureKeyPair {
    ECDSA(ECDSASignatureKeyPair),
    EdDSA(EdDSASignatureKeyPair),
    RSA(RSASignatureKeyPair),
}

impl SignatureKeyPair {
    fn export(&self, encoding: KeyPairEncoding) -> Result<Vec<u8>, Error> {
        let encoded = match encoding {
            KeyPairEncoding::PKCS8 => match self {
                SignatureKeyPair::ECDSA(kp) => kp.as_pkcs8()?.to_vec(),
                SignatureKeyPair::EdDSA(kp) => kp.as_pkcs8()?.to_vec(),
                SignatureKeyPair::RSA(kp) => kp.as_pkcs8()?.to_vec(),
            },
            _ => bail!(CryptoError::UnsupportedEncoding),
        };
        Ok(encoded)
    }

    fn generate(handles: &HandleManagers, kp_builder_handle: Handle) -> Result<Handle, Error> {
        let kp_builder = handles.signature_keypair_builder.get(kp_builder_handle)?;
        let handle = match kp_builder {
            SignatureKeyPairBuilder::ECDSA(kp_builder) => kp_builder.generate(handles)?,
            SignatureKeyPairBuilder::EdDSA(kp_builder) => kp_builder.generate(handles)?,
            SignatureKeyPairBuilder::RSA(kp_builder) => kp_builder.generate(handles)?,
        };
        Ok(handle)
    }

    fn import(
        handles: &HandleManagers,
        kp_builder_handle: Handle,
        encoded: &[u8],
        encoding: KeyPairEncoding,
    ) -> Result<Handle, Error> {
        let kp_builder = handles.signature_keypair_builder.get(kp_builder_handle)?;
        let handle = match kp_builder {
            SignatureKeyPairBuilder::ECDSA(kp_builder) => {
                kp_builder.import(handles, encoded, encoding)?
            }
            SignatureKeyPairBuilder::EdDSA(kp_builder) => {
                kp_builder.import(handles, encoded, encoding)?
            }
            SignatureKeyPairBuilder::RSA(kp_builder) => {
                kp_builder.import(handles, encoded, encoding)?
            }
        };
        Ok(handle)
    }

    fn public_key(&self, handles: &HandleManagers) -> Result<Handle, Error> {
        let pk = match self {
            SignatureKeyPair::ECDSA(kp) => {
                let raw_pk = kp.raw_public_key();
                SignaturePublicKey::ECDSA(ECDSASignaturePublicKey::from_raw(kp.alg, raw_pk)?)
            }
            SignatureKeyPair::EdDSA(kp) => {
                let raw_pk = kp.raw_public_key();
                SignaturePublicKey::EdDSA(EdDSASignaturePublicKey::from_raw(kp.alg, raw_pk)?)
            }
            SignatureKeyPair::RSA(kp) => {
                let raw_pk = kp.raw_public_key();
                SignaturePublicKey::RSA(RSASignaturePublicKey::from_raw(kp.alg, raw_pk)?)
            }
        };
        let handle = handles.signature_publickey.register(pk)?;
        Ok(handle)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SignatureKeyPairBuilder {
    ECDSA(ECDSASignatureKeyPairBuilder),
    EdDSA(EdDSASignatureKeyPairBuilder),
    RSA(RSASignatureKeyPairBuilder),
}

impl SignatureKeyPairBuilder {
    fn open(handles: &HandleManagers, op_handle: Handle) -> Result<Handle, Error> {
        let signature_op = handles.signature_op.get(op_handle)?;
        let kp_builder = match signature_op {
            SignatureOp::ECDSA(_) => SignatureKeyPairBuilder::ECDSA(
                ECDSASignatureKeyPairBuilder::new(signature_op.alg()),
            ),
            SignatureOp::EdDSA(_) => SignatureKeyPairBuilder::EdDSA(
                EdDSASignatureKeyPairBuilder::new(signature_op.alg()),
            ),
            SignatureOp::RSA(_) => {
                SignatureKeyPairBuilder::RSA(RSASignatureKeyPairBuilder::new(signature_op.alg()))
            }
        };
        let handle = handles.signature_keypair_builder.register(kp_builder)?;
        Ok(handle)
    }
}

impl WasiCryptoCtx {
    pub fn signature_keypair_builder_open(&self, op_handle: Handle) -> Result<Handle, Error> {
        SignatureKeyPairBuilder::open(&self.handles, op_handle)
    }

    pub fn signature_keypair_builder_close(&self, handle: Handle) -> Result<(), Error> {
        self.handles.signature_keypair_builder.close(handle)
    }

    pub fn signature_keypair_generate(&self, kp_builder_handle: Handle) -> Result<Handle, Error> {
        SignatureKeyPair::generate(&self.handles, kp_builder_handle)
    }

    pub fn signature_keypair_import(
        &self,
        kp_builder_handle: Handle,
        encoded: &[u8],
        encoding: KeyPairEncoding,
    ) -> Result<Handle, Error> {
        SignatureKeyPair::import(&self.handles, kp_builder_handle, encoded, encoding)
    }

    pub fn signature_keypair_from_id(
        &self,
        _kp_builder_handle: Handle,
        _kp_id: &[u8],
        _kp_version: Version,
    ) -> Result<Handle, Error> {
        bail!(CryptoError::UnsupportedFeature)
    }

    pub fn signature_keypair_id(&self, kp_handle: Handle) -> Result<(Handle, Version), Error> {
        let _kp = self.handles.signature_keypair.get(kp_handle)?;
        bail!(CryptoError::UnsupportedFeature)
    }

    pub fn signature_keypair_invalidate(
        &self,
        _kp_builder_handle: Handle,
        _kp_id: &[u8],
        _kp_version: Version,
    ) -> Result<(), Error> {
        bail!(CryptoError::UnsupportedFeature)
    }

    pub fn signature_keypair_export(
        &self,
        kp_handle: Handle,
        encoding: KeyPairEncoding,
    ) -> Result<Handle, Error> {
        let kp = self.handles.signature_keypair.get(kp_handle)?;
        let encoded = kp.export(encoding)?;
        let array_output_handle = ArrayOutput::register(&self.handles, encoded)?;
        Ok(array_output_handle)
    }

    pub fn signature_keypair_publickey(&self, kp_handle: Handle) -> Result<Handle, Error> {
        let kp = self.handles.signature_keypair.get(kp_handle)?;
        let handle = kp.public_key(&self.handles)?;
        Ok(handle)
    }

    pub fn signature_keypair_close(&self, handle: Handle) -> Result<(), Error> {
        self.handles.signature_keypair.close(handle)
    }
}
