# Threat Model — Linux Enclave

## Assumptions

This system assumes a **strong adversary model**:

### Adversary Capabilities
- Full root access on Linux host
- Kernel-level compromise possible
- Ability to inspect memory outside enclave
- Ability to intercept network traffic
- Ability to modify user-space binaries

### Out of Scope
- Physical hardware attacks (initial version)
- Side-channel attacks (initial prototype phase)
- Supply chain attacks (future consideration)

---

## Security Goals

Linux Enclave aims to provide:

### 1. Execution Integrity
Code running inside the enclave cannot be modified externally.

### 2. Data Confidentiality
Sensitive data remains encrypted outside enclave boundaries.

### 3. Identity Assurance
Each enclave has a cryptographic identity.

### 4. Attestation
External systems can verify enclave state.

---

## Threat Categories

### 1. OS-Level Attacker
- Can inspect processes
- Can modify system calls
- Can attempt memory injection

**Mitigation:**
- Enclave isolation layer
- Memory encryption (simulated)
- Integrity checks

---

### 2. Network Attacker
- Can intercept or modify traffic

**Mitigation:**
- End-to-end encryption
- Message signing
- Replay protection

---

### 3. Privilege Escalation Attacker
- Gains root or kernel privileges

**Mitigation:**
- Treat OS as untrusted
- Cryptographic verification of all enclave inputs

---

### 4. Binary Tampering
- Modified enclave runtime or modules

**Mitigation:**
- Signed binaries
- Hash-based integrity checks

---

## Security Boundaries

Trusted:
- Enclave runtime
- Crypto module (conceptual trust anchor)

Untrusted:
- Linux kernel
- User-space applications
- Network stack
