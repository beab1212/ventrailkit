# Demo

```bash
cargo build
python3 ../ventrailkit-local/scripts/gen_pocs.py
cargo run -p ventrail-cli --bin ventrail -- stats fuzz/corpus/seed_01.bin
./.clusterfuzzlite/build.sh
```
