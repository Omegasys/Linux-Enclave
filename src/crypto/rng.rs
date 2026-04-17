pub fn secure_random_bytes(len: usize) -> Vec<u8> {
    use rand::RngCore;

    let mut buf = vec![0u8; len];
    rand::thread_rng().fill_bytes(&mut buf);

    buf
}
