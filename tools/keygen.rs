use rand::RngCore;

fn main() {
    println!("[KeyGen] Generating simulated keypair...");

    let mut pub_key = vec![0u8; 32];
    let mut priv_key = vec![0u8; 32];

    rand::thread_rng().fill_bytes(&mut pub_key);
    rand::thread_rng().fill_bytes(&mut priv_key);

    println!("Public Key: {:02x?}", pub_key);
    println!("Private Key: {:02x?}", priv_key);
}
