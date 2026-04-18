use crate::crypto::sealing::{seal, unseal};

#[test]
fn test_seal_unseal() {
    let data = b"secret";
    let key = b"key";

    let sealed = seal(data, key);
    let unsealed = unseal(&sealed, key).unwrap();

    assert_eq!(data.to_vec(), unsealed);
}
