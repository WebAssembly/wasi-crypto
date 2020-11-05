mod kyber;

use super::*;
pub use kyber::*;

#[derive(Clone, Debug)]
pub struct EncapsulatedSecret {
    pub encapsulated_secret: Vec<u8>,
    pub secret: Vec<u8>,
}
