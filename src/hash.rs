/// Defines a trait for hashing.
pub trait Hash {
    fn bytes (&self) -> Vec<u8>;

    fn hash (&self) -> Vec<u8> {
        crypto_hash::digest(crypto_hash::Algorithm::SHA256, &self.bytes())
    }
}