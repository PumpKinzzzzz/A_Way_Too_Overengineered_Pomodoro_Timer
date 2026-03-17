#!/usr/bin/env bash
set -euo pipefail
cd AWTOPT

npm run check
cd src-tauri
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings