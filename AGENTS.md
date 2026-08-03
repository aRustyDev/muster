# AGENTS.md — workspace orientation

Orrery is a spatiotemporal feasibility engine; Muster is the first
application on it; Muster-SDK is the search layer between. The engine
answers "is this arrangement of people/events/locations possible, and how
good is it" — it never searches for arrangements (Rule 03).

## Crate map

| Crate | One line | Never contains |
|---|---|---|
| `crates/orrery` | model, interval algebra, violations, derivation, travel, repository trait, command layer | search, UI, network I/O, concrete datastore types in public API |
| `crates/muster-sdk` | solvers, objectives, batch orchestration, change-set computation | feasibility semantics, UI, delivery |
| `crates/muster` | application: UI, auth, coordinator workflows, notification delivery | feasibility computation, search algorithms |

Each crate's own `AGENTS.md` has its module map and gotchas.

## Build / test

```
just doctor          # toolchain check (nextest/deny/hack are required by the recipes)
just ci              # fmt-check + clippy -D warnings + tests + doctests + doc build + deny
cargo test --workspace          # plain fallback
just orrery::check-seam         # no datastore type in orrery's public API
./docs/scripts/check-xrefs.sh   # docs cross-reference audit (= just audit)
./evidence/run_all.sh           # reproduce the datastore benchmarks (~8 min)
```

*(Refreshed 2026-08-03, quality review F-13: nextest was described as
"optional" while every test recipe requires it. Cross-crate testing
policy: `.claude/plans/TESTING-STRATEGY.md`.)*

## Invariants worth memorising

* Every relation carries a validity window; every conflict check is the same
  interval-overlap predicate (half-open `[start, end)`).
* **Every query is entity-partitioned before its interval predicate applies**
  — the single fact that drove index and datastore analysis.
* Derived semantics, cached physically; blast radius made computable by salsa
  early cutoff (Phase 3+).
* All mutations through the `Command` enum; all persistence behind the
  `Repository` trait (sync — ADR-0023); `MemoryRepo` enforces the restrictive
  constraint intersection *executably* (Rule 00b).
* Personal anchors never cross the coordinator boundary (Rule 09).

## Where to look

Plans and phase records: `.claude/plans/` (start at `PLAN.md`; per-product
under `{orrery,muster-sdk,muster}/`). Decisions: `docs/src/adrs/` (MADR;
ADR-0015 is the open one). Evidence: `evidence/` + RESEARCH docs under
`.claude/plans/orrery/research/`.
