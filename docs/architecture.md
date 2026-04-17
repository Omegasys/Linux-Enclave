# Linux Enclave Architecture

## High-Level Design

Linux Enclave is structured as a layered security system:
+---------------------------+
| Application Layer |
+---------------------------+
| Enclave Runtime (core) |
+---------------------------+
| Security & Policy Layer |
+---------------------------+
| Crypto Layer |
+---------------------------+
| Linux Host OS (UNTRUSTED)|
+---------------------------+


## Design Principles

### 1. Minimal Trusted Computing Base (TCB)
Only the enclave runtime and crypto layer are considered trusted.

### 2. Explicit Trust Boundaries
All communication between modules is verified and controlled.

### 3. OS is Untrusted
The Linux kernel is treated as potentially compromised.

### 4. Cryptographic Enforcement
Security relies on cryptographic verification, not assumption.

---

## Core Components

### Enclave Runtime (`core/`)
Responsible for:
- Enclave lifecycle (init, run, destroy)
- Task scheduling inside enclave
- Execution isolation logic

### Crypto Layer (`crypto/`)
Handles:
- Key generation
- Digital signatures
- Sealing/unsealing data
- Secure randomness

### Security Layer (`security/`)
Handles:
- Policy enforcement
- Attestation generation
- Sandboxing rules
- Access control decisions

### Network Layer (`network/`)
Handles:
- Encrypted messaging
- Secure channel creation
- Message verification

---

## Execution Model

1. Application enters enclave runtime
2. Runtime validates execution policy
3. Crypto layer verifies integrity
4. Code executes in isolated context
5. Output is sealed or signed before leaving enclave
