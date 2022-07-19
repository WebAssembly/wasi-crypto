#[cfg(feature = "pqcrypto")]
mod kyber;

#[cfg(feature = "pqcrypto")]
pub use kyber::*;

use super::*;

#[derive(Clone, Debug)]
pub struct EncapsulatedSecret {
    pub encapsulated_secret: Vec<u8>,
    pub secret: Vec<u8>,
}
