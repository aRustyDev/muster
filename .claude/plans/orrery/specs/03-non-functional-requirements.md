<!-- Imported from design thread 2026-08-01. Fable: review and improve. -->

# SPEC 03 — Orrery non-functional requirements

## Performance budgets

Measured at 10⁶ `attends` edges / 10k persons / 10k events.

| Class | Budget | Measured baseline |
|---|---|---|
| Per-person derived expansion | < 25 ms p95 | 0.2 ms (SQLite) / 4.3 ms (Ladybug) |
| Per-person conflict detection | < 25 ms p95 | 0.6 ms / 3.4 ms |
| Per-person travel feasibility | < 25 ms p95 | 5.3 ms (Ladybug) |
| Bounded 2-hop co-attendance | < 50 ms p95 | 1.2 ms / 11.3 ms |
| Global conflict sweep | < 10 s | 4,775 ms / 2,051 ms |
| Layer-2 closure refresh | < 60 s | not yet measured |
| Cold open | < 1 s | not yet measured |

Interactive and batch budgets are tracked **separately** — they have different
bottlenecks and conflating them misleads.

## Scale targets

| Dimension | v1 target | Stretch |
|---|---:|---:|
| Persons | 10,000 | 100,000 |
| Events | 10,000 | 50,000 |
| `attends` edges | 1,000,000 | 10,000,000 |
| Locations | 2,000 | 20,000 |
| Group depth | 5 | 8 |

## Correctness

* Every detector property-tested against a brute-force oracle
* Interval algebra fuzz-tested against a reference implementation
* Cross-engine differential testing where two repository implementations exist
  — this caught nothing in the spike because results agreed exactly, which is
  itself the evidence the harness works

## Security and privacy

* Personal anchors never cross the coordinator boundary — verdicts only
  (ADR-0014). Enforced at the engine boundary and tested there.
* Waivers record actor and timestamp; violation history is append-only
* No PII in logs or violation payloads
* Engine performs no network I/O; travel data arrives through an explicit port

## Operability

* Single-file or single-directory embedded store
* Export to a portable format (Parquet/CSV) so data is never trapped
* Deterministic rebuild of all derived state from base facts
* Structured logging with a correlation ID per command

## Reversibility

* No concrete datastore type in `orrery`'s public API
* At least two repository implementations before v1 — proves the seam is real
