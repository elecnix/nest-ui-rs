#!/usr/bin/env bash
set -euo pipefail

OUTPUT_PATH=${1:-artifacts/ui-snapshot.png}
CARGO_BIN=${CARGO_BIN:-$HOME/.cargo/bin/cargo}
ROOT_DIR=$(cd -- "$(dirname "$0")/.." && pwd)
cd "$ROOT_DIR"

mkdir -p "$(dirname "$OUTPUT_PATH")"

echo "[capture] Building simulator (release)..."
EG_SIMULATOR_DUMP="$OUTPUT_PATH" "$CARGO_BIN" run --example simulator --release --quiet

echo "[capture] Screenshot written to $OUTPUT_PATH"
