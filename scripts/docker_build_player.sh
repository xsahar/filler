#!/usr/bin/env bash
set -euo pipefail

# Build the player in Docker and extract the Linux binary to solution/bin/filler.
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"
OUT_DIR="${ROOT_DIR}/solution/bin"

mkdir -p "${OUT_DIR}"
cd "${ROOT_DIR}"

echo "[docker] Building player image..."
docker build -f docker/Dockerfile.player -t filler-player:latest .

echo "[docker] Exporting binary..."
container_id="$(docker create filler-player:latest)"
docker cp "${container_id}:/app/filler" "${OUT_DIR}/filler"
docker rm "${container_id}" >/dev/null
chmod +x "${OUT_DIR}/filler"

echo "[ok] Player binary ready at: ${OUT_DIR}/filler"
