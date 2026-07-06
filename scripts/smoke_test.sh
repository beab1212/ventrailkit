#!/bin/bash -eu
set -euo pipefail
cargo build
cargo run -p ventrail-cli --bin ventrail -- stats fuzz/corpus/seed_01.bin
