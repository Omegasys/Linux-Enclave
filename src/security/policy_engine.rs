#[derive(Debug)]
pub enum PolicyDecision {
    Allow,
    Deny,
}

#[derive(Debug)]
pub struct Policy {
    pub allow_execution: bool,
    pub max_payload_size: usize,
}

pub fn evaluate_policy(policy: &Policy, input_size: usize) -> PolicyDecision {
    if !policy.allow_execution {
        return PolicyDecision::Deny;
    }

    if input_size > policy.max_payload_size {
        return PolicyDecision::Deny;
    }

    PolicyDecision::Allow
}
