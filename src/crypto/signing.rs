use sha2::{Sha256, Digest};

pub fn sign(data: &[u8], private_key: &[u8]) -> Vec<u8> {
    // VERY simplified signature (hash-based placeholder)
    let mut hasher = Sha256::new();

    hasher.update(data);
    hasher.update(private_key);

    hasher.finalize().to_vec()
}

pub fn verify(data: &[u8], signature: &[u8], private_key: &[u8]) -> bool {
    sign(data, private_key) == signature
}
