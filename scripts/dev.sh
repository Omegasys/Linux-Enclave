#!/usr/bin/env bash
set -e

echo "[Dev] Running in development mode..."

export RUST_LOG=debug

cargo run

echo "[Dev] Done."
