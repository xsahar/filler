#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"
OUT_DIR="${ROOT_DIR}/solution/bin"

mkdir -p "${OUT_DIR}"

echo "[build] Building release binary with Cargo..."
cd "${ROOT_DIR}"
cargo build --release

cp "${ROOT_DIR}/target/release/filler" "${OUT_DIR}/filler"
chmod +x "${OUT_DIR}/filler"

echo "[ok] Player binary ready at: ${OUT_DIR}/filler"
