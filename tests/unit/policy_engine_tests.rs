use crate::security::policy_engine::{Policy, evaluate_policy, PolicyDecision};

#[test]
fn test_policy_allow() {
    let policy = Policy {
        allow_execution: true,
        max_payload_size: 100,
    };

    let decision = evaluate_policy(&policy, 50);
    assert!(matches!(decision, PolicyDecision::Allow));
}
