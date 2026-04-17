#[derive(Debug, Clone)]
pub struct KeyPair {
    pub public_key: Vec<u8>,
    pub private_key: Vec<u8>,
}

pub fn generate_keypair() -> KeyPair {
    // NOTE: placeholder key generation (NOT cryptographically secure)
    let public_key = vec![1, 2, 3, 4, 5];
    let private_key = vec![5, 4, 3, 2, 1];

    KeyPair {
        public_key,
        private_key,
    }
}
