# API Reference — Linux Enclave

## Core API Overview

This document describes internal interfaces for Linux Enclave modules.

---

## core/ API

### `EnclaveRuntime`

```rust
fn init() -> Result<EnclaveRuntime, Error>;
fn execute(task: EnclaveTask) -> Result<EnclaveResult, Error>;
fn terminate() -> Result<(), Error>;
Description

Manages lifecycle and execution of enclave environment.

crypto/ API
Key Management
fn generate_keypair() -> KeyPair;
fn sign(data: &[u8], key: &PrivateKey) -> Signature;
fn verify(data: &[u8], sig: &Signature, key: &PublicKey) -> bool;
Sealing
fn seal(data: &[u8], key: &SealingKey) -> Vec<u8>;
fn unseal(data: &[u8], key: &SealingKey) -> Vec<u8>;
RNG
fn secure_random_bytes(len: usize) -> Vec<u8>;
security/ API
Policy Engine
fn evaluate_policy(task: &Task) -> PolicyDecision;
Attestation
fn generate_attestation(state: &EnclaveState) -> AttestationReport;
fn verify_attestation(report: &AttestationReport) -> bool;
Sandbox
fn enforce_sandbox(context: &ExecutionContext) -> Result<(), Error>;
network/ API
Secure Channel
fn create_channel(peer: PeerId) -> SecureChannel;
fn send(channel: &SecureChannel, msg: Message) -> Result<(), Error>;
fn receive(channel: &SecureChannel) -> Result<Message, Error>;
Data Structures
EnclaveTask
input: encrypted payload
policy_id: execution ruleset
metadata: optional context
EnclaveResult
output: encrypted result
signature: integrity proof
AttestationReport
measurement_hash
signature
enclave_id
Notes
All APIs assume untrusted host environment
All sensitive outputs must be sealed or signed
No plaintext sensitive data should leave enclave boundary

---

If you want next step, I can:
- :contentReference[oaicite:0]{index=0}
- :contentReference[oaicite:1]{index=1}
- or :contentReference[oaicite:2]{index=2}
