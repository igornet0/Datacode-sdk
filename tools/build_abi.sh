#!/usr/bin/env bash
# Build a DataCode native module (Rust cdylib) and optionally copy the output.
#
# Usage:
#   ./build_abi.sh [PROJECT_DIR] [OUTPUT_DIR]
#
# PROJECT_DIR: path to Cargo project (default: current directory)
# OUTPUT_DIR:  where to copy the built .so/.dylib (default: no copy)
#
# Examples:
#   ./build_abi.sh                          # build in current dir
#   ./build_abi.sh ../examples/hello_module  # build hello_module
#   ./build_abi.sh . ./out                   # build and copy to ./out

set -e

PROJECT_DIR="${1:-.}"
OUTPUT_DIR="${2:-}"

SCRIPT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$SCRIPT_ROOT"

if [[ -n "$PROJECT_DIR" && "$PROJECT_DIR" != "." ]]; then
  cd "$PROJECT_DIR"
fi

echo "Building native module in $(pwd) ..."
cargo build --release --lib 2>&1

LIB_NAME=$(grep -E '^name\s*=' Cargo.toml 2>/dev/null | head -1 | sed 's/.*"\(.*\)".*/\1/')
[[ -z "$LIB_NAME" ]] && LIB_NAME="$(basename "$(pwd)")"

if [[ "$(uname -s)" == "Darwin" ]]; then
  BUILT="target/release/lib${LIB_NAME}.dylib"
else
  BUILT="target/release/lib${LIB_NAME}.so"
fi

if [[ ! -f "$BUILT" ]]; then
  echo "Could not find built library. Ensure [lib] crate-type = [\"cdylib\"] in Cargo.toml."
  exit 1
fi

echo "Built: $BUILT"

if [[ -n "$OUTPUT_DIR" ]]; then
  mkdir -p "$OUTPUT_DIR"
  cp "$BUILT" "$OUTPUT_DIR/"
  echo "Copied to $OUTPUT_DIR/$(basename "$BUILT")"
fi
