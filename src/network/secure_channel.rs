use crate::network::encryption_layer;

#[derive(Debug)]
pub struct SecureChannel {
    pub peer_id: u64,
    key: Vec<u8>,
}

impl SecureChannel {
    pub fn new(peer_id: u64) -> Self {
        Self {
            peer_id,
            key: encryption_layer::generate_session_key(),
        }
    }

    pub fn send(&self, data: &[u8]) -> Vec<u8> {
        encryption_layer::encrypt(data, &self.key)
    }

    pub fn receive(&self, data: &[u8]) -> Vec<u8> {
        encryption_layer::decrypt(data, &self.key)
    }
}
