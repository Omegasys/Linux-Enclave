use crate::crypto::sealing::{seal, unseal};

#[test]
fn test_tampering_detected() {
    let data = b"secure";
    let key = b"key";

    let mut sealed = seal(data, key);

    // Tamper
    sealed[0] ^= 0xFF;

    let result = unseal(&sealed, key);

    assert!(result.is_none());
}
