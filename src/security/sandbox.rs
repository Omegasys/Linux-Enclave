use crate::security::policy_engine::{Policy, evaluate_policy, PolicyDecision};

pub struct Sandbox;

impl Sandbox {
    pub fn execute(policy: &Policy, input: Vec<u8>) -> Result<Vec<u8>, String> {
        match evaluate_policy(policy, input.len()) {
            PolicyDecision::Allow => {
                println!("[Sandbox] Execution allowed");
                Ok(input) // passthrough simulation
            }
            PolicyDecision::Deny => {
                Err("Sandbox policy denied execution".into())
            }
        }
    }
}
