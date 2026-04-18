use crate::crypto::rng::secure_random_bytes;

#[test]
fn test_rng_length() {
    let bytes = secure_random_bytes(32);
    assert_eq!(bytes.len(), 32);
}
