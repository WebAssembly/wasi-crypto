mod key_pair;
mod public_key;
mod secret_key;

pub use key_pair::*;
pub use public_key::*;
pub use secret_key::*;

use crate::common::ArrayOutput;
use crate::error::*;
use crate::raw;

pub fn kx_dh(public_key: KxPublicKey, secret_key: KxSecretKey) -> Result<Vec<u8>, Error> {
    let shared_secret_handle = unsafe { raw::kx_dh(public_key.0.handle, secret_key.0.handle)? };
    ArrayOutput::new(shared_secret_handle).into_vec()
}
