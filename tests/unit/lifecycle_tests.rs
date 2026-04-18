use crate::core::lifecycle::LifecycleState;

#[test]
fn test_lifecycle_states() {
    let state = LifecycleState::Created;
    assert_eq!(state, LifecycleState::Created);
}
