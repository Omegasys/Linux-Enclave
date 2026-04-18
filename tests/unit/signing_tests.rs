use crate::crypto::signing::{sign, verify};

#[test]
fn test_sign_verify() {
    let data = b"important";
    let key = b"private";

    let sig = sign(data, key);

    assert!(verify(data, &sig, key));
}
