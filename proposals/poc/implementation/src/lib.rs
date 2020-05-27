#![allow(
    clippy::unit_arg,
    clippy::identity_conversion,
    clippy::new_without_default,
    clippy::new_ret_no_self,
    clippy::too_many_arguments
)]
#[macro_use]
extern crate derivative;

mod array_output;
mod asymmetric_common;
mod error;
mod handles;
mod key_manager;
mod options;
mod signatures;
mod symmetric;
mod version;
mod wasi_glue;

use crate::types as guest_types;
use array_output::*;
use asymmetric_common::*;
use handles::*;
use options::*;
use signatures::*;
use symmetric::*;

pub use asymmetric_common::{KeyPairEncoding, PublicKeyEncoding};
pub use error::CryptoError;
pub use handles::Handle;
pub use signatures::SignatureEncoding;
pub use version::Version;

#[allow(unused)]
static REBUILD_IF_WITX_FILE_IS_UPDATED: [&str; 4] = [
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../witx/proposal_common.witx"
    )),
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../witx/proposal_asymmetric_common.witx"
    )),
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../witx/proposal_signatures.witx"
    )),
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../witx/proposal_symmetric.witx"
    )),
];

wiggle::from_witx!({
    witx: ["../witx/wasi_ephemeral_crypto.witx"],
    ctx: WasiCryptoCtx
});

pub mod wasi_modules {
    pub use crate::{
        wasi_ephemeral_crypto_asymmetric_common, wasi_ephemeral_crypto_common,
        wasi_ephemeral_crypto_signatures, wasi_ephemeral_crypto_symmetric,
    };
}

pub struct HandleManagers {
    pub array_output: HandlesManager<ArrayOutput>,
    pub options: HandlesManager<Options>,
    pub signature_keypair_manager: HandlesManager<SignatureKeyPairManager>,
    pub keypair: HandlesManager<KeyPair>,
    pub signature_state: HandlesManager<SignatureState>,
    pub signature: HandlesManager<Signature>,
    pub publickey: HandlesManager<PublicKey>,
    pub signature_verification_state: HandlesManager<SignatureVerificationState>,
    pub symmetric_state: HandlesManager<SymmetricState>,
    pub symmetric_key: HandlesManager<SymmetricKey>,
    pub symmetric_tag: HandlesManager<SymmetricTag>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AlgorithmType {
    Signatures,
    Symmetric,
}

impl From<guest_types::AlgorithmType> for AlgorithmType {
    fn from(options_type: guest_types::AlgorithmType) -> Self {
        match options_type {
            guest_types::AlgorithmType::Signatures => AlgorithmType::Signatures,
            guest_types::AlgorithmType::Symmetric => AlgorithmType::Symmetric,
        }
    }
}

pub struct CryptoCtx {
    pub(crate) handles: HandleManagers,
}

pub struct WasiCryptoCtx {
    ctx: CryptoCtx,
}

impl CryptoCtx {
    pub fn new() -> Self {
        CryptoCtx {
            handles: HandleManagers {
                array_output: HandlesManager::new(0x00),
                options: HandlesManager::new(0x01),
                signature_keypair_manager: HandlesManager::new(0x02),
                keypair: HandlesManager::new(0x03),
                signature_state: HandlesManager::new(0x04),
                signature: HandlesManager::new(0x05),
                publickey: HandlesManager::new(0x06),
                signature_verification_state: HandlesManager::new(0x07),
                symmetric_state: HandlesManager::new(0x09),
                symmetric_key: HandlesManager::new(0x0a),
                symmetric_tag: HandlesManager::new(0x0b),
            },
        }
    }
}

impl WasiCryptoCtx {
    pub fn new() -> Self {
        WasiCryptoCtx {
            ctx: CryptoCtx::new(),
        }
    }
}
