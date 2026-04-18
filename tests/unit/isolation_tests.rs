use crate::security::isolation::{IsolationContext, enforce_isolation};

#[test]
fn test_isolation_enforced() {
    let ctx = IsolationContext {
        process_id: 1,
        memory_region_id: 1,
        privilege_level: 0,
    };

    assert!(enforce_isolation(&ctx));
}
