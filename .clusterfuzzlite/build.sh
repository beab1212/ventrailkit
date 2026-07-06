#!/bin/bash -eu
# ClusterFuzzLite build - Rust libFuzzer + AddressSanitizer.
set -euo pipefail

find_repo_root() {
  local dir="$1"
  while [[ "$dir" != "/" ]]; do
    if [[ -f "$dir/Cargo.toml" && -f "$dir/fuzz/Cargo.toml" ]]; then
      echo "$dir"
      return 0
    fi
    dir="$(dirname "$dir")"
  done
  return 1
}

BUILD_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
if [[ -n "${SRC:-}" && -f "$SRC/fuzz/Cargo.toml" ]]; then
  SCRIPT_ROOT="$SRC"
elif repo_root="$(find_repo_root "$BUILD_DIR")"; then
  SCRIPT_ROOT="$repo_root"
else
  echo "Could not locate repository root (started from ${BUILD_DIR})" >&2
  exit 1
fi

OUT="${OUT:-$SCRIPT_ROOT/out}"
mkdir -p "$OUT"
cd "$SCRIPT_ROOT"

if [[ "${RUSTFLAGS:-}" != *"fuzzing"* ]]; then
  export RUSTFLAGS="${RUSTFLAGS:-} --cfg fuzzing"
fi

if command -v rustup >/dev/null 2>&1 && rustup toolchain list 2>/dev/null | rg -q nightly; then
  cargo +nightly build --release -p session_fuzzer --target-dir "$SCRIPT_ROOT/fuzz/target"
else
  cargo build --release -p session_fuzzer --target-dir "$SCRIPT_ROOT/fuzz/target"
fi

cp "$SCRIPT_ROOT/fuzz/target/release/session_fuzzer" "$OUT/session_fuzzer"
echo "Built session_fuzzer -> $OUT/session_fuzzer"
