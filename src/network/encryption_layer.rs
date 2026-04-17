use sha2::{Sha256, Digest};

pub fn generate_session_key() -> Vec<u8> {
    vec![42, 99, 13, 7, 255] // placeholder
}

pub fn encrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.update(key);

    let mut out = hasher.finalize().to_vec();
    out.extend_from_slice(data);
    out
}

pub fn decrypt(data: &[u8], _key: &[u8]) -> Vec<u8> {
    if data.len() <= 32 {
        return vec![];
    }
    data[32..].to_vec()
}
