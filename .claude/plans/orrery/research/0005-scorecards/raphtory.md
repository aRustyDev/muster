# Screening scorecard — Raphtory

* Repository / homepage: https://github.com/Pometry/Raphtory · https://docs.raphtory.com · crates.io `raphtory`
* Licence: **GPL-3.0**
* Language / bindings: native Rust crate + Python bindings
* Latest release + date: GitHub v0.18.5, 2026-06-23; crates.io lags at 0.17.0 (2026-03-10)
* Contributors (12mo) / commits (12mo): very active — pushed 2026-08-01, 43 contributors, major storage rework (`db_v4`) in flight
* Category (A-E): **D** — in-memory temporal graph **analytics** engine; persistence is whole-graph Parquet save/load; out-of-core operation is a commercial tier; no transaction API documented

## Hard requirements

| # | Requirement | Verdict | Evidence |
|---|---|---|---|
| 1 | Per-hop edge filtering (stop-gate) | n/a as engine feature | programmatic API — user code can filter each hop, but there is no engine-evaluated recursive query surface; category D disposes of it first |
| 2 | Native Rust bindings | pass | native Rust |
| 3 | Concurrency model | not documented | multithreaded analytics; reader/writer isolation unspecified |
| 4 | ACID transactions | **FAIL** | no transaction API documented anywhere found |
| 5 | Maintenance signal | pass | genuinely active, funded company (Pometry) |
| 6 | Sync or async (Rule 04) | sync core | plain blocking API on docs.rs |

## Modelling fit

Not assessed as SoR — category D.

## Verdict

**record but not as SoR (category D).** The mandate says temporal-graph analytics engines may earn a place *alongside* the store. Recording honestly: Orrery's Layer-1 pathfinding is already assigned to `petgraph` (in-memory, small), and the withdrawn cascade analytic (ADR-0020) removed the workload Raphtory would shine on. **GPL-3.0 would additionally contaminate a permissively-licensed workspace** — any future use needs a licence decision first.

**Reason (one sentence):** An excellent, active temporal analytics engine that is not a system of record, has no transactions, is GPL-licensed, and currently has no workload in Orrery that petgraph does not already cover.

## Uncertainty

Whether the `db_v4` rework changes the durability story; why crates.io trails GitHub by two minor versions.
