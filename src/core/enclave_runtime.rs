use crate::crypto::{
    keys::KeyPair,
    sealing,
    signing,
};

use crate::core::lifecycle::LifecycleState;

#[derive(Debug)]
pub struct EnclaveRuntime {
    state: LifecycleState,
    keys: Option<KeyPair>,
}

impl EnclaveRuntime {
    pub fn new() -> Self {
        Self {
            state: LifecycleState::Created,
            keys: None,
        }
    }

    pub fn init(&mut self) {
        println!("[Runtime] Initializing enclave...");

        self.keys = Some(crate::crypto::keys::generate_keypair());
        self.state = LifecycleState::Running;

        println!("[Runtime] Enclave initialized.");
    }

    pub fn execute(&self, input: Vec<u8>) -> Result<Vec<u8>, String> {
        if self.state != LifecycleState::Running {
            return Err("Enclave not running".into());
        }

        let keys = self.keys.as_ref().ok_or("Missing keys")?;

        // Seal input (simulate secure boundary)
        let sealed = sealing::seal(&input, &keys.public_key);

        // Sign sealed data
        let signature = signing::sign(&sealed, &keys.private_key);

        let mut output = sealed.clone();
        output.extend(signature);

        Ok(output)
    }

    pub fn shutdown(&mut self) {
        println!("[Runtime] Shutting down enclave...");

        self.state = LifecycleState::Stopped;
        self.keys = None;

        println!("[Runtime] Secure memory cleared (simulated).");
    }
}
