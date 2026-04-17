use std::collections::HashMap;

use crate::network::secure_channel::SecureChannel;

pub struct MessageRouter {
    channels: HashMap<u64, SecureChannel>,
}

impl MessageRouter {
    pub fn new() -> Self {
        Self {
            channels: HashMap::new(),
        }
    }

    pub fn register_peer(&mut self, peer_id: u64) {
        self.channels.insert(peer_id, SecureChannel::new(peer_id));
    }

    pub fn route(&self, peer_id: u64, message: &[u8]) -> Option<Vec<u8>> {
        self.channels.get(&peer_id).map(|ch| ch.send(message))
    }
}
