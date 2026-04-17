use sha2::{Sha256, Digest};

pub fn seal(data: &[u8], key: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();

    hasher.update(data);
    hasher.update(key);

    let mut output = hasher.finalize().to_vec();

    // append original data (simulated encryption boundary)
    output.extend_from_slice(data);

    output
}

pub fn unseal(sealed: &[u8], key: &[u8]) -> Option<Vec<u8>> {
    if sealed.len() < 32 {
        return None;
    }

    let data = &sealed[32..];

    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.update(key);

    let expected = hasher.finalize().to_vec();

    if &sealed[..32] == expected {
        Some(data.to_vec())
    } else {
        None
    }
}
