# VentrailKit

VentrailKit is a Rust toolkit for hydrothermal vent observatory data: ROV expedition
sessions, plume-chemistry sweeps, sonar mosaic tiles, sampler carousels, mooring
telemetry, and expedition bundle packages.

The repository is structured as a realistic parser/CLI/fuzzing workspace:

- `ventrail-core` implements the HVWS wire-session parser, HVBK package validator,
  and domain modules for ocean-floor observatory processing.
- `ventrail-cli` provides `ingest`, `validate`, and `stats` commands.
- `fuzz` contains a single libFuzzer harness and deterministic PoC runner.
- `.clusterfuzzlite` builds the Rust libFuzzer target with AddressSanitizer.

PoC files and patch tasks are intentionally kept outside the submitted repository in
`ventrailkit-local/`.
# ventrailkit
