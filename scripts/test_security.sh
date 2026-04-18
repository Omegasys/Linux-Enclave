#!/usr/bin/env bash
set -e

echo "[Security Tests] Running security-focused tests..."

# Run all tests
cargo test

# Optional: run clippy for linting
cargo clippy -- -D warnings || true

# Optional: check formatting
cargo fmt -- --check || true

echo "[Security Tests] Completed."
