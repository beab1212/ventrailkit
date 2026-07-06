#!/bin/bash -eu
set -euo pipefail
cargo build
python3 ../ventrailkit-local/scripts/gen_pocs.py
python3 ../ventrailkit-local/scripts/gen_patches.py
