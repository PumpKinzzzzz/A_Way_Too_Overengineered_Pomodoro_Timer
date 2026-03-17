#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR/AWTOPT"

echo "[1/3] npm audit"
npm audit --audit-level moderate

echo "[2/3] cargo audit"
cd src-tauri
cargo audit

echo "[3/3] cargo deny"
cargo deny check advisories licenses

echo "OK: security checks passed."