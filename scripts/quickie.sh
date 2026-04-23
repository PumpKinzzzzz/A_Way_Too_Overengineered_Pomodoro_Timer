#!/usr/bin/env bash
set -euo pipefail
cd AWTOPT

echo "Formatting frontend..."
npm run format

echo "Formatting Rust code..."
cd src-tauri
cargo fmt --all

echo "Checking Rust compilation..."
cargo check --all-targets --all-features

echo "Running Clippy..."
cargo clippy --all-targets --all-features -- -D warnings

echo "Done! Code formatted and checked."