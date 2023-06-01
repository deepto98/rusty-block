// Trait is similar to an interface
// use crypto_hash::{hex_digest, Algorithm};

pub trait Hashable {
    fn bytes(&self) -> Vec<u8>; // To be implemented by others

    fn hash(&self) -> Vec<u8> {
        crypto_hash::digest(crypto_hash::Algorithm::SHA256, &self.bytes())
    }
}
