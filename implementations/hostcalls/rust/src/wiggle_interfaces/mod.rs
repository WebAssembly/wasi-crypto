use crate::CryptoCtx;
use std::collections::HashMap;
use std::rc::Rc;
pub use types as guest_types;

pub fn witx_interfaces() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert(
        "proposal_common.witx",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../witx/proposal_common.witx"
        )),
    );
    map.insert(
        "proposal_asymmetric_common.witx",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../witx/proposal_asymmetric_common.witx"
        )),
    );
    map.insert(
        "proposal_signatures.witx",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../witx/proposal_signatures.witx"
        )),
    );
    map.insert(
        "proposal_symmetric.witx",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../witx/proposal_symmetric.witx"
        )),
    );
    map.insert(
        "proposal_kx.witx",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../witx/proposal_kx.witx"
        )),
    );
    map.insert(
        "wasi_ephemeral_crypto.witx",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../witx/wasi_ephemeral_crypto.witx"
        )),
    );
    map
}

wiggle::from_witx!({
    witx: ["$CARGO_MANIFEST_DIR/../../../witx/wasi_ephemeral_crypto.witx"],
    ctx: WasiCryptoCtx
});

pub mod wasi_modules {
    pub use super::{
        wasi_ephemeral_crypto_asymmetric_common, wasi_ephemeral_crypto_common,
        wasi_ephemeral_crypto_kx, wasi_ephemeral_crypto_signatures,
        wasi_ephemeral_crypto_symmetric,
    };
}

#[derive(Clone)]
pub struct WasiCryptoCtx {
    pub(crate) ctx: Rc<CryptoCtx>,
}

impl WasiCryptoCtx {
    pub fn new() -> Self {
        WasiCryptoCtx {
            ctx: Rc::new(CryptoCtx::new()),
        }
    }
}

pub mod asymmetric_common;
pub mod common;
pub mod error;
pub mod key_exchange;
pub mod signatures;
pub mod symmetric;
