use crate::crypto::keys::generate_keypair;

#[test]
fn test_key_generation() {
    let keys = generate_keypair();
    assert!(!keys.public_key.is_empty());
    assert!(!keys.private_key.is_empty());
}
