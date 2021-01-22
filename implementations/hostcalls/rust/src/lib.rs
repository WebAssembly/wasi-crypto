#![allow(
    clippy::unit_arg,
    clippy::useless_conversion,
    clippy::new_without_default,
    clippy::new_ret_no_self,
    clippy::unnecessary_wraps,
    clippy::too_many_arguments
)]
#![allow(unused_imports, dead_code)]
#[macro_use]
extern crate derivative;

mod array_output;
mod asymmetric_common;
mod error;
mod handles;
mod key_exchange;
mod options;
mod rand;
mod secrets_manager;
mod signatures;
mod symmetric;
mod version;

use std::collections::HashMap;
use std::rc::Rc;

use array_output::*;
use asymmetric_common::*;
use handles::*;
use key_exchange::*;
use options::*;
use secrets_manager::*;
use signatures::*;
use symmetric::*;

pub use asymmetric_common::{KeyPairEncoding, PublicKeyEncoding, SecretKeyEncoding};
pub use error::CryptoError;
pub use handles::Handle;
pub use signatures::SignatureEncoding;
pub use version::Version;

pub(crate) struct HandleManagers {
    pub array_output: HandlesManager<ArrayOutput>,
    pub options: HandlesManager<Options>,
    pub keypair: HandlesManager<KeyPair>,
    pub publickey: HandlesManager<PublicKey>,
    pub secretkey: HandlesManager<SecretKey>,
    pub signature_state: HandlesManager<SignatureState>,
    pub signature: HandlesManager<Signature>,
    pub signature_verification_state: HandlesManager<SignatureVerificationState>,
    pub symmetric_state: HandlesManager<SymmetricState>,
    pub symmetric_key: HandlesManager<SymmetricKey>,
    pub symmetric_tag: HandlesManager<SymmetricTag>,
    pub secrets_manager: HandlesManager<SecretsManager>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AlgorithmType {
    Signatures,
    Symmetric,
    KeyExchange,
}

pub struct CryptoCtx {
    pub(crate) handles: HandleManagers,
}

impl CryptoCtx {
    pub fn new() -> Self {
        CryptoCtx {
            handles: HandleManagers {
                array_output: HandlesManager::new(0x00),
                options: HandlesManager::new(0x01),
                keypair: HandlesManager::new(0x02),
                publickey: HandlesManager::new(0x03),
                secretkey: HandlesManager::new(0x04),
                signature_state: HandlesManager::new(0x05),
                signature: HandlesManager::new(0x06),
                signature_verification_state: HandlesManager::new(0x07),
                symmetric_state: HandlesManager::new(0x08),
                symmetric_key: HandlesManager::new(0x09),
                symmetric_tag: HandlesManager::new(0x0a),
                secrets_manager: HandlesManager::new(0x0b),
            },
        }
    }
}
