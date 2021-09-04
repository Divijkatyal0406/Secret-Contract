use super::*;

pub trait Hashable {
    fn bytes (&self) -> Vec<u8>;

    fn hash (&self) -> Hash {
        crypto_hash::digest(crypto_hash::Algorithm::SHA256, &self.bytes())
    }
    pub fn create_hashed_password(s1: &str) -> [u8; VIEWING_KEY_SIZE] {
        Sha256::digest(s1.as_bytes())
            .as_slice()
            .try_into()
            .expect("Wrong password length")
    }
}
