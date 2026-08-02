# crates/orrery — the engine

Feasibility oracle and objective evaluator: `is_feasible(assignment) ->
Vec<Violation>` and `score(assignment) -> f64` (Phase 3+). Owns entities,
relations with validity windows, interval algebra, violation detectors,
derived expansion, travel layers, analytics, the repository trait, and the
command layer.

**Must never contain** (Rule 03): search or optimisation, UI, network I/O,
or a concrete datastore type in the public API. If trying a different solver
would require changing this crate, the boundary is broken.

## Module map (Phase 2 state)

| Module | Responsibility |
|---|---|
| `error` | typed errors (`thiserror`); constraint violations name the constraint |
| `model` | id newtypes (UUIDv7), entities, relations — every relation has `during` |
| `interval` | `Timestamp` (i64 µs UTC), half-open `Interval`, Allen relations |
| `command` | the single mutation chokepoint (`Command` enum, receipts) |
| `repo` | `Repository` trait (sync — ADR-0023) + `repo::memory::MemoryRepo` |

Phase 3 adds `derive`, `detect`; Phase 4 adds `travel`; later `analytics`.

## Testing

`just test` (or `cargo test -p orrery`). Property tests live in `tests/`
with `prop_` prefixes; detectors (Phase 3) ship with brute-force oracles or
not at all. `just check-seam` greps the public API for datastore names.

## Gotchas

* `MemoryRepo` **errors by design** on a second concurrent writer and on any
  read during an open write (Rule 00b). Tests rely on this; do not "fix" it.
* `Interval::new` rejects `end <= start`; zero-length needs `at_point`.
* Timestamps are µs since epoch UTC; `chrono` is API-boundary only and not
  yet a dependency (QUESTION-0014 still open — closes before Phase 3).
* Traversal APIs take constant `at: Timestamp` filters only — cross-hop
  predicates are excluded by construction; do not add a predicate callback.
