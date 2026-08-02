# Rule 06 — Dependencies

Adding a dependency to `orrery` requires an ADR. Adding one to `muster-sdk` or
`muster` requires a line in the phase document. See ADR-0022 for the approved
baseline.

## Approved baseline

| Crate | Where | Purpose |
|---|---|---|
| `thiserror` | orrery, muster-sdk | typed library errors |
| `anyhow` | muster only | top-level context chaining |
| `serde` | all | model serialisation |
| `chrono` | API boundary only | see ADR-0022 on internal representation |
| `uuid` | all | entity identity — **v7, not v4** |
| `blake3` | orrery | derived-edge content addressing |
| `salsa` | orrery | incremental derivation |
| `petgraph` | orrery | Layer-1 pathfinding, in-memory |
| `tracing` | orrery, muster-sdk | instrumentation only |
| `opentelemetry*` | **muster only** | exporters — never in a library |
| `figment` | muster only | layered configuration |
| `rand` | muster-sdk | stochastic search |
| `dioxus` | **not muster-sdk** | see QUESTION-0015 |

## Bars for a new dependency in `orrery`

1. Cannot be reasonably implemented in under ~200 lines
2. Maintained — releases within the last year, more than one contributor
3. Does not pull a transitive async runtime unless async is already decided
4. Does not perform I/O outside the repository trait

## `orrery` performs no network I/O

Travel data arrives through an explicit port. A dependency that reaches a
routing API belongs in `muster-sdk` or an adapter crate, never the engine.
