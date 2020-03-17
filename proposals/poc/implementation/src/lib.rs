mod array_output;
mod ecdsa;
mod eddsa;
mod error;
mod handles;
mod rsa;
mod signature;
mod signature_keypair;
mod signature_op;
mod signature_publickey;

use array_output::*;
use handles::*;
use signature::*;
use signature_keypair::*;
use signature_op::*;
use signature_publickey::*;

pub use error::{CryptoError, WasiCryptoError};
pub use handles::Handle;
pub use signature::SignatureEncoding;
pub use signature_keypair::{KeyPairEncoding, Version};
pub use signature_publickey::PublicKeyEncoding;

pub struct HandleManagers {
    pub signature_op: HandlesManager<SignatureOp>,
    pub signature_keypair_builder: HandlesManager<SignatureKeyPairBuilder>,
    pub signature_keypair: HandlesManager<SignatureKeyPair>,
    pub signature_state: HandlesManager<ExclusiveSignatureState>,
    pub signature: HandlesManager<Signature>,
    pub signature_publickey: HandlesManager<SignaturePublicKey>,
    pub signature_verification_state: HandlesManager<ExclusiveSignatureVerificationState>,
    pub array_output: HandlesManager<ArrayOutput>,
}

pub struct WasiCryptoCtx {
    pub(crate) handles: HandleManagers,
}

impl WasiCryptoCtx {
    pub fn new() -> Self {
        WasiCryptoCtx {
            handles: HandleManagers {
                array_output: HandlesManager::new(0x00),
                signature_op: HandlesManager::new(0x01),
                signature_keypair_builder: HandlesManager::new(0x02),
                signature_keypair: HandlesManager::new(0x03),
                signature_state: HandlesManager::new(0x04),
                signature: HandlesManager::new(0x05),
                signature_publickey: HandlesManager::new(0x06),
                signature_verification_state: HandlesManager::new(0x07),
            },
        }
    }
}

#[test]
fn test_signatures() {
    let ctx = WasiCryptoCtx::new();
    let op_handle = ctx.signature_op_open("ECDSA_P256_SHA256").unwrap();
    let kp_builder_handle = ctx.signature_keypair_builder_open(op_handle).unwrap();
    let kp_handle = ctx.signature_keypair_generate(kp_builder_handle).unwrap();
    let state_handle = ctx.signature_state_open(kp_handle).unwrap();
    ctx.signature_state_update(state_handle, b"test").unwrap();
    let signature_handle = ctx.signature_state_sign(state_handle).unwrap();

    let pk_handle = ctx.signature_keypair_publickey(kp_handle).unwrap();

    let verification_state_handle = ctx.signature_verification_state_open(pk_handle).unwrap();
    ctx.signature_verification_state_update(verification_state_handle, b"test")
        .unwrap();
    ctx.signature_verification_state_verify(verification_state_handle, signature_handle)
        .unwrap();

    ctx.signature_op_close(op_handle).unwrap();
    ctx.signature_keypair_builder_close(kp_builder_handle)
        .unwrap();
    ctx.signature_keypair_close(kp_handle).unwrap();
    ctx.signature_state_close(state_handle).unwrap();
    ctx.signature_verification_state_close(verification_state_handle)
        .unwrap();
    ctx.signature_close(signature_handle).unwrap();
}
