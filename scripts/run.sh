#!/usr/bin/env bash
set -e

echo "[Run] Launching Linux Enclave..."

cargo run --release

echo "[Run] Execution finished."
