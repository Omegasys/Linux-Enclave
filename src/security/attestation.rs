use sha2::{Sha256, Digest};

#[derive(Debug)]
pub struct AttestationReport {
    pub measurement_hash: Vec<u8>,
    pub enclave_id: u64,
}

pub fn generate_attestation(enclave_state: &[u8], enclave_id: u64) -> AttestationReport {
    let mut hasher = Sha256::new();
    hasher.update(enclave_state);

    AttestationReport {
        measurement_hash: hasher.finalize().to_vec(),
        enclave_id,
    }
}

pub fn verify_attestation(report: &AttestationReport, expected: &[u8]) -> bool {
    &report.measurement_hash == expected
}
