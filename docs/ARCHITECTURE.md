# Architecture

VentrailKit follows a three-layer parser architecture.

1. `wire::session_feeder` validates the HVWS frame and dispatches sections.
2. Domain modules process realistic ocean-observatory records into a shared
   `ByteBuffer` without unsafe code.
3. `risk` contains the intentionally low-level contract handlers:
   extent copies, phase-window exports, and cached view probes.

The fuzzing harness calls only `SessionFeeder::push(data)`, so every behavior is
reachable through one binary wire format and a single ClusterFuzzLite target.
