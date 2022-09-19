mod key_pair;
mod public_key;
mod secret_key;

pub use key_pair::*;
pub use public_key::*;
pub use secret_key::*;

#[derive(Debug)]
pub struct EncapsulatedSecret {
    pub secret: Vec<u8>,
    pub encapsulated_secret: Vec<u8>,
}
