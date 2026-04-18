use crate::core::enclave_runtime::EnclaveRuntime;

#[test]
fn test_full_pipeline() {
    let mut runtime = EnclaveRuntime::new();
    runtime.init();

    let input = b"pipeline test".to_vec();
    let result = runtime.execute(input);

    assert!(result.is_ok());
}
