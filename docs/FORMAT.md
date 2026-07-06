# HVWS / HVBK Format

## HVWS Session

| Offset | Size | Field | Description |
|---:|---:|---|---|
| 0 | 4 | magic | `HVWS` (`0x53575648` little-endian) |
| 4 | 2 | version | `0x0001` |
| 6 | 2 | section_count | number of sections, parser caps at 64 |

Each section is:

| Size | Field | Description |
|---:|---|---|
| 1 | tag | dispatch key |
| 1 | flags | phase selector for cached-view sections |
| var | payload_len | unsigned LEB128 payload size |
| N | payload | handler-specific bytes |

Tags `0x01..0x0C` route to ordinary domain processors. Tags `0xA0..0xB3`
route to hard contract handlers used by the fuzzing benchmark.

## HVBK Bundle

HVBK packages begin with magic `HVBK`, version, section count, and a compact
section table of tag/offset/length tuples. The CLI validator checks table bounds
and emits per-section CRC health bytes.
