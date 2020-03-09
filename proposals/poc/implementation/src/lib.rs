#[macro_use]
extern crate lazy_static;

mod ecdsa;
mod eddsa;
mod error;
mod handles;
mod rsa;
mod signature;
mod signature_keypair;
mod signature_op;
mod signature_publickey;

use handles::*;
use signature::*;
use signature_keypair::*;
use signature_op::*;
use signature_publickey::*;

pub use error::{CryptoError, WasiCryptoError};
pub use handles::Handle;
pub use signature::SignatureEncoding;
pub use signature_keypair::KeyPairEncoding;
pub use signature_publickey::PublicKeyEncoding;

pub use signature::{
    signature_close, signature_export, signature_import, signature_state_close,
    signature_state_open, signature_state_sign, signature_state_update,
    signature_verification_state_close, signature_verification_state_open,
    signature_verification_state_update, signature_verification_state_verify,
};

pub use signature_keypair::{
    signature_keypair_builder_close, signature_keypair_builder_open, signature_keypair_close,
    signature_keypair_export, signature_keypair_from_id, signature_keypair_generate,
    signature_keypair_id, signature_keypair_import, signature_keypair_publickey,
};

pub use signature_op::{signature_op_close, signature_op_open};

pub use signature_publickey::{
    signature_publickey_close, signature_publickey_export, signature_publickey_import,
};

pub struct WasiCryptoCtx {
    pub signature_op_manager: HandlesManager<SignatureOp>,
    pub signature_keypair_builder_manager: HandlesManager<SignatureKeyPairBuilder>,
    pub signature_keypair_manager: HandlesManager<SignatureKeyPair>,
    pub signature_state_manager: HandlesManager<ExclusiveSignatureState>,
    pub signature_manager: HandlesManager<Signature>,
    pub signature_publickey_manager: HandlesManager<SignaturePublicKey>,
    pub signature_verification_state_manager: HandlesManager<ExclusiveSignatureVerificationState>,
}

// These maps should be stored in a WASI context
lazy_static! {
    static ref WASI_CRYPTO_CTX: WasiCryptoCtx = WasiCryptoCtx {
        signature_op_manager: HandlesManager::new(0x00),
        signature_keypair_builder_manager: HandlesManager::new(0x01),
        signature_keypair_manager: HandlesManager::new(0x02),
        signature_state_manager: HandlesManager::new(0x03),
        signature_manager: HandlesManager::new(0x04),
        signature_publickey_manager: HandlesManager::new(0x05),
        signature_verification_state_manager: HandlesManager::new(0x06),
    };
}

#[test]
fn test_signatures() {
    let op_handle = signature_op_open("ECDSA_P256_SHA256").unwrap();
    let kp_builder_handle = signature_keypair_builder_open(op_handle).unwrap();
    let kp_handle = signature_keypair_generate(kp_builder_handle).unwrap();
    let state_handle = signature_state_open(kp_handle).unwrap();
    signature_state_update(state_handle, b"test").unwrap();
    let signature_handle = signature_state_sign(state_handle).unwrap();

    let pk_handle = signature_keypair_publickey(kp_handle).unwrap();

    let verification_state_handle = signature_verification_state_open(pk_handle).unwrap();
    signature_verification_state_update(verification_state_handle, b"test").unwrap();
    signature_verification_state_verify(verification_state_handle, signature_handle).unwrap();

    signature_op_close(op_handle).unwrap();
    signature_keypair_builder_close(kp_builder_handle).unwrap();
    signature_keypair_close(kp_handle).unwrap();
    signature_state_close(state_handle).unwrap();
    signature_verification_state_close(verification_state_handle).unwrap();
    signature_close(signature_handle).unwrap();
}
