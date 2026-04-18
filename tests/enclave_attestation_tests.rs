use crate::security::attestation::{generate_attestation, verify_attestation};

#[test]
fn test_attestation_generation() {
    let state = b"enclave_state";
    let report = generate_attestation(state, 1);

    assert!(!report.measurement_hash.is_empty());
}

#[test]
fn test_attestation_verification() {
    let state = b"enclave_state";
    let report = generate_attestation(state, 1);

    let valid = verify_attestation(&report, &report.measurement_hash);

    assert!(valid);
}
