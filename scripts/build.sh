#!/usr/bin/env bash
set -e

echo "[Build] Starting build process..."

# Ensure dependencies are fetched
cargo fetch

# Build project
cargo build --release

echo "[Build] Completed successfully."
