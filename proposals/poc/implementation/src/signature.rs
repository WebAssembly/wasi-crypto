use std::sync::Arc;

use super::array_output::*;
use super::ecdsa::*;
use super::eddsa::*;
use super::error::*;
use super::handles::*;
use super::rsa::*;
use super::signature_keypair::*;
use super::signature_publickey::*;
use super::types as guest_types;
use super::{CryptoCtx, HandleManagers, WasiCryptoCtx};

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u16)]
pub enum SignatureAlgorithm {
    ECDSA_P256_SHA256,
    ECDSA_P384_SHA384,
    Ed25519,
    RSA_PKCS1_2048_8192_SHA256,
    RSA_PKCS1_2048_8192_SHA384,
    RSA_PKCS1_2048_8192_SHA512,
    RSA_PKCS1_3072_8192_SHA384,
}

#[derive(Clone, Debug)]
pub enum Signature {
    ECDSA(ECDSASignature),
    EdDSA(EdDSASignature),
    RSA(RSASignature),
}

impl AsRef<[u8]> for Signature {
    fn as_ref(&self) -> &[u8] {
        match self {
            Signature::ECDSA(signature) => signature.as_ref(),
            Signature::EdDSA(signature) => signature.as_ref(),
            Signature::RSA(signature) => signature.as_ref(),
        }
    }
}

impl PartialEq for Signature {
    fn eq(&self, other: &Self) -> bool {
        ring::constant_time::verify_slices_are_equal(self.as_ref(), other.as_ref()).is_ok()
    }
}

impl Eq for Signature {}

impl Signature {
    fn from_raw(alg: SignatureAlgorithm, encoded: &[u8]) -> Result<Self, CryptoError> {
        let signature = match alg {
            SignatureAlgorithm::ECDSA_P256_SHA256 => {
                ensure!(encoded.len() == 64, CryptoError::InvalidSignature);
                Signature::ECDSA(ECDSASignature::new(
                    SignatureEncoding::Raw,
                    encoded.to_vec(),
                ))
            }
            SignatureAlgorithm::ECDSA_P384_SHA384 => {
                ensure!(encoded.len() == 96, CryptoError::InvalidSignature);
                Signature::ECDSA(ECDSASignature::new(
                    SignatureEncoding::Raw,
                    encoded.to_vec(),
                ))
            }
            SignatureAlgorithm::Ed25519 => {
                ensure!(encoded.len() == 64, CryptoError::InvalidSignature);
                Signature::EdDSA(EdDSASignature::new(encoded.to_vec()))
            }
            SignatureAlgorithm::RSA_PKCS1_2048_8192_SHA256 => {
                Signature::RSA(RSASignature::new(encoded.to_vec()))
            }
            SignatureAlgorithm::RSA_PKCS1_2048_8192_SHA384 => {
                Signature::RSA(RSASignature::new(encoded.to_vec()))
            }
            SignatureAlgorithm::RSA_PKCS1_2048_8192_SHA512 => {
                Signature::RSA(RSASignature::new(encoded.to_vec()))
            }
            SignatureAlgorithm::RSA_PKCS1_3072_8192_SHA384 => {
                Signature::RSA(RSASignature::new(encoded.to_vec()))
            }
        };
        Ok(signature)
    }

    fn as_ecdsa(&self) -> Result<&ECDSASignature, CryptoError> {
        match self {
            Signature::ECDSA(signature) => Ok(signature),
            _ => bail!(CryptoError::InvalidSignature),
        }
    }

    fn as_eddsa(&self) -> Result<&EdDSASignature, CryptoError> {
        match self {
            Signature::EdDSA(signature) => Ok(signature),
            _ => bail!(CryptoError::InvalidSignature),
        }
    }

    fn as_rsa(&self) -> Result<&RSASignature, CryptoError> {
        match self {
            Signature::RSA(signature) => Ok(signature),
            _ => bail!(CryptoError::InvalidSignature),
        }
    }
}

#[derive(Debug)]
pub enum SignatureState {
    ECDSA(ECDSASignatureState),
    EdDSA(EdDSASignatureState),
    RSA(RSASignatureState),
}

#[derive(Debug, Clone)]
pub struct ExclusiveSignatureState {
    state: Arc<SignatureState>,
}

impl ExclusiveSignatureState {
    fn new(signature_state: SignatureState) -> Self {
        ExclusiveSignatureState {
            state: Arc::new(signature_state),
        }
    }

    fn open(handles: &HandleManagers, kp_handle: Handle) -> Result<Handle, CryptoError> {
        let kp = handles.signature_keypair.get(kp_handle)?;
        let signature_state = match kp {
            SignatureKeyPair::ECDSA(kp) => {
                ExclusiveSignatureState::new(SignatureState::ECDSA(ECDSASignatureState::new(kp)))
            }
            SignatureKeyPair::EdDSA(kp) => {
                ExclusiveSignatureState::new(SignatureState::EdDSA(EdDSASignatureState::new(kp)))
            }
            SignatureKeyPair::RSA(kp) => {
                ExclusiveSignatureState::new(SignatureState::RSA(RSASignatureState::new(kp)))
            }
        };
        let handle = handles.signature_state.register(signature_state)?;
        Ok(handle)
    }

    fn update(&mut self, input: &[u8]) -> Result<(), CryptoError> {
        match self.state.as_ref() {
            SignatureState::ECDSA(state) => state.update(input),
            SignatureState::EdDSA(state) => state.update(input),
            SignatureState::RSA(state) => state.update(input),
        }
    }

    fn sign(&mut self) -> Result<Signature, CryptoError> {
        let signature = match self.state.as_ref() {
            SignatureState::ECDSA(state) => Signature::ECDSA(state.sign()?),
            SignatureState::EdDSA(state) => Signature::EdDSA(state.sign()?),
            SignatureState::RSA(state) => Signature::RSA(state.sign()?),
        };
        Ok(signature)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SignatureEncoding {
    Raw,
    Hex,
    Base64Original,
    Base64OriginalNoPadding,
    Base64URLSafe,
    Base64URLSafeNoPadding,
    DER,
}

impl From<guest_types::SignatureEncoding> for SignatureEncoding {
    fn from(encoding: guest_types::SignatureEncoding) -> Self {
        match encoding {
            guest_types::SignatureEncoding::Raw => SignatureEncoding::Raw,
            guest_types::SignatureEncoding::Hex => SignatureEncoding::Hex,
            guest_types::SignatureEncoding::Base64Original => SignatureEncoding::Base64Original,
            guest_types::SignatureEncoding::Base64OriginalNopadding => {
                SignatureEncoding::Base64OriginalNoPadding
            }
            guest_types::SignatureEncoding::Base64Urlsafe => SignatureEncoding::Base64URLSafe,
            guest_types::SignatureEncoding::Base64UrlsafeNopadding => {
                SignatureEncoding::Base64URLSafeNoPadding
            }
            guest_types::SignatureEncoding::Der => SignatureEncoding::DER,
        }
    }
}

#[derive(Debug)]
pub enum SignatureVerificationState {
    ECDSA(ECDSASignatureVerificationState),
    EdDSA(EdDSASignatureVerificationState),
    RSA(RSASignatureVerificationState),
}

#[derive(Debug, Clone)]
pub struct ExclusiveSignatureVerificationState {
    state: Arc<SignatureVerificationState>,
}

impl ExclusiveSignatureVerificationState {
    fn new(signature_verification_state: SignatureVerificationState) -> Self {
        ExclusiveSignatureVerificationState {
            state: Arc::new(signature_verification_state),
        }
    }

    fn open(handles: &HandleManagers, pk_handle: Handle) -> Result<Handle, CryptoError> {
        let pk = handles.signature_publickey.get(pk_handle)?;
        let signature_verification_state = match pk {
            SignaturePublicKey::ECDSA(pk) => ExclusiveSignatureVerificationState::new(
                SignatureVerificationState::ECDSA(ECDSASignatureVerificationState::new(pk)),
            ),
            SignaturePublicKey::EdDSA(pk) => ExclusiveSignatureVerificationState::new(
                SignatureVerificationState::EdDSA(EdDSASignatureVerificationState::new(pk)),
            ),
            SignaturePublicKey::RSA(pk) => ExclusiveSignatureVerificationState::new(
                SignatureVerificationState::RSA(RSASignatureVerificationState::new(pk)),
            ),
        };
        let handle = handles
            .signature_verification_state
            .register(signature_verification_state)?;
        Ok(handle)
    }

    fn update(&mut self, input: &[u8]) -> Result<(), CryptoError> {
        match self.state.as_ref() {
            SignatureVerificationState::ECDSA(state) => state.update(input),
            SignatureVerificationState::EdDSA(state) => state.update(input),
            SignatureVerificationState::RSA(state) => state.update(input),
        }
    }

    fn verify(
        &self,
        handles: &HandleManagers,
        signature_handle: Handle,
    ) -> Result<(), CryptoError> {
        let signature = handles.signature.get(signature_handle)?;
        match self.state.as_ref() {
            SignatureVerificationState::ECDSA(state) => state.verify(signature.as_ecdsa()?),
            SignatureVerificationState::EdDSA(state) => state.verify(signature.as_eddsa()?),
            SignatureVerificationState::RSA(state) => state.verify(signature.as_rsa()?),
        }
    }
}

impl CryptoCtx {
    pub fn signature_export(
        &self,
        signature_handle: Handle,
        encoding: SignatureEncoding,
    ) -> Result<Handle, CryptoError> {
        match encoding {
            SignatureEncoding::Raw => {}
            _ => bail!(CryptoError::UnsupportedEncoding),
        }
        let signature = self.handles.signature.get(signature_handle)?;
        let array_output_handle =
            ArrayOutput::register(&self.handles, signature.as_ref().to_vec())?;
        Ok(array_output_handle)
    }

    pub fn signature_import(
        &self,
        op_handle: Handle,
        encoding: SignatureEncoding,
        encoded: &[u8],
    ) -> Result<Handle, CryptoError> {
        let signature_op = self.handles.signature_op.get(op_handle)?;
        let signature = match encoding {
            SignatureEncoding::Raw => Signature::from_raw(signature_op.alg(), encoded)?,
            _ => bail!(CryptoError::UnsupportedEncoding),
        };
        let handle = self.handles.signature.register(signature)?;
        Ok(handle)
    }

    pub fn signature_state_open(&self, kp_handle: Handle) -> Result<Handle, CryptoError> {
        ExclusiveSignatureState::open(&self.handles, kp_handle)
    }

    pub fn signature_state_update(
        &self,
        state_handle: Handle,
        input: &[u8],
    ) -> Result<(), CryptoError> {
        let mut state = self.handles.signature_state.get(state_handle)?;
        state.update(input)
    }

    pub fn signature_state_sign(&self, state_handle: Handle) -> Result<Handle, CryptoError> {
        let mut state = self.handles.signature_state.get(state_handle)?;
        let signature = state.sign()?;
        let handle = self.handles.signature.register(signature)?;
        Ok(handle)
    }

    pub fn signature_state_close(&self, handle: Handle) -> Result<(), CryptoError> {
        self.handles.signature_state.close(handle)
    }

    pub fn signature_verification_state_open(
        &self,
        pk_handle: Handle,
    ) -> Result<Handle, CryptoError> {
        ExclusiveSignatureVerificationState::open(&self.handles, pk_handle)
    }

    pub fn signature_verification_state_update(
        &self,
        verification_state_handle: Handle,
        input: &[u8],
    ) -> Result<(), CryptoError> {
        let mut state = self
            .handles
            .signature_verification_state
            .get(verification_state_handle)?;
        state.update(input)
    }

    pub fn signature_verification_state_verify(
        &self,
        verification_state_handle: Handle,
        signature_handle: Handle,
    ) -> Result<(), CryptoError> {
        let state = self
            .handles
            .signature_verification_state
            .get(verification_state_handle)?;
        state.verify(&self.handles, signature_handle)
    }

    pub fn signature_verification_state_close(
        &self,
        verification_state_handle: Handle,
    ) -> Result<(), CryptoError> {
        self.handles
            .signature_verification_state
            .close(verification_state_handle)
    }

    pub fn signature_close(&self, signature_handle: Handle) -> Result<(), CryptoError> {
        self.handles.signature.close(signature_handle)
    }
}

impl WasiCryptoCtx {
    pub fn signature_export(
        &self,
        signature_handle: guest_types::Signature,
        encoding: guest_types::SignatureEncoding,
    ) -> Result<guest_types::ArrayOutput, CryptoError> {
        Ok(self
            .ctx
            .signature_export(signature_handle.into(), encoding.into())?
            .into())
    }

    pub fn signature_import(
        &self,
        op_handle: guest_types::SignatureOp,
        encoding: guest_types::SignatureEncoding,
        encoded_ptr: wiggle_runtime::GuestPtr<'_, u8>,
        encoded_len: guest_types::Size,
    ) -> Result<guest_types::Signature, CryptoError> {
        let mut guest_borrow = wiggle_runtime::GuestBorrows::new();
        let encoded: &[u8] = unsafe {
            &*encoded_ptr
                .as_array(encoded_len as _)
                .as_raw(&mut guest_borrow)?
        };
        Ok(self
            .ctx
            .signature_import(op_handle.into(), encoding.into(), encoded)?
            .into())
    }

    pub fn signature_state_open(
        &self,
        kp_handle: guest_types::SignatureKeypair,
    ) -> Result<guest_types::SignatureState, CryptoError> {
        Ok(self.ctx.signature_state_open(kp_handle.into())?.into())
    }

    pub fn signature_state_update(
        &self,
        state_handle: guest_types::SignatureState,
        input_ptr: wiggle_runtime::GuestPtr<'_, u8>,
        input_len: guest_types::Size,
    ) -> Result<(), CryptoError> {
        let mut guest_borrow = wiggle_runtime::GuestBorrows::new();
        let input: &[u8] = unsafe {
            &*input_ptr
                .as_array(input_len as _)
                .as_raw(&mut guest_borrow)?
        };
        Ok(self
            .ctx
            .signature_state_update(state_handle.into(), input)?
            .into())
    }

    pub fn signature_state_sign(
        &self,
        state_handle: guest_types::SignatureState,
    ) -> Result<guest_types::ArrayOutput, CryptoError> {
        Ok(self.ctx.signature_state_sign(state_handle.into())?.into())
    }

    pub fn signature_state_close(
        &self,
        state_handle: guest_types::SignatureState,
    ) -> Result<(), CryptoError> {
        Ok(self.ctx.signature_state_close(state_handle.into())?.into())
    }

    pub fn signature_verification_state_open(
        &self,
        pk_handle: guest_types::SignaturePublickey,
    ) -> Result<guest_types::SignatureVerificationState, CryptoError> {
        Ok(self
            .ctx
            .signature_verification_state_open(pk_handle.into())?
            .into())
    }

    pub fn signature_verification_state_update(
        &self,
        verification_state_handle: guest_types::SignatureVerificationState,
        input_ptr: wiggle_runtime::GuestPtr<'_, u8>,
        input_len: guest_types::Size,
    ) -> Result<(), CryptoError> {
        let mut guest_borrow = wiggle_runtime::GuestBorrows::new();
        let input: &[u8] = unsafe {
            &*input_ptr
                .as_array(input_len as _)
                .as_raw(&mut guest_borrow)?
        };
        Ok(self
            .ctx
            .signature_verification_state_update(verification_state_handle.into(), input)?
            .into())
    }

    pub fn signature_verification_state_verify(
        &self,
        verification_state_handle: guest_types::SignatureVerificationState,
        signature_handle: guest_types::Signature,
    ) -> Result<(), CryptoError> {
        Ok(self
            .ctx
            .signature_verification_state_verify(
                verification_state_handle.into(),
                signature_handle.into(),
            )?
            .into())
    }

    pub fn signature_verification_state_close(
        &self,
        verification_state_handle: guest_types::SignatureVerificationState,
    ) -> Result<(), CryptoError> {
        Ok(self
            .ctx
            .signature_verification_state_close(verification_state_handle.into())?
            .into())
    }

    pub fn signature_close(
        &self,
        signature_handle: guest_types::Signature,
    ) -> Result<(), CryptoError> {
        Ok(self.ctx.signature_close(signature_handle.into())?.into())
    }
}
