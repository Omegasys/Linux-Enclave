use crate::network::secure_channel::SecureChannel;

#[test]
fn test_secure_channel() {
    let channel = SecureChannel::new(1);

    let msg = b"hello";
    let encrypted = channel.send(msg);
    let decrypted = channel.receive(&encrypted);

    assert_eq!(msg.to_vec(), decrypted);
}
