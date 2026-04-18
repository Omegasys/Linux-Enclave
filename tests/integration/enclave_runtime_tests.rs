use crate::core::enclave_runtime::EnclaveRuntime;

#[test]
fn test_runtime_execute() {
    let mut runtime = EnclaveRuntime::new();
    runtime.init();

    let result = runtime.execute(b"test".to_vec());

    assert!(result.is_ok());
}
