# Linux Enclave Documentation

Welcome to the Linux Enclave project.

Linux Enclave is a research-grade, GPLv3-licensed security system inspired by:
- Trusted Execution Environments (TEE)
- Secure Boot chains
- Hardware-rooted trust systems (TPM / Secure Enclave concepts)
- Confidential computing models

## Overview

This system simulates enclave-like isolation on Linux, focusing on:

- Cryptographic trust boundaries
- Isolated execution environments
- Attestation mechanisms
- Secure communication channels
- Policy-driven execution control

## Core Modules

- `core/` → enclave runtime and lifecycle management
- `crypto/` → cryptographic primitives and key handling
- `security/` → sandboxing, policy, attestation
- `network/` → encrypted communication layer

## Documentation Sections

- Architecture → system design overview
- Threat Model → assumed attacker capabilities
- Enclave Design → runtime and isolation model
- API Reference → internal interfaces

---

This project is experimental and not intended for production security use.
