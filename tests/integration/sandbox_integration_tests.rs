use crate::security::sandbox::Sandbox;
use crate::security::policy_engine::Policy;

#[test]
fn test_sandbox_allows() {
    let policy = Policy {
        allow_execution: true,
        max_payload_size: 100,
    };

    let result = Sandbox::execute(&policy, vec![1,2,3]);

    assert!(result.is_ok());
}
