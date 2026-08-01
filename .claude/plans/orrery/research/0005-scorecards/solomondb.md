# Screening scorecard — SolomonDB

* Repository / homepage: https://github.com/chungquantin/solomon-db · crates.io `solomondb`
* Licence: MIT
* Language / bindings: native Rust crate over pluggable KV stores (RocksDB, redb)
* Latest release + date: GitHub "v1.0.0-beta" 2022-11-06; crates.io **0.0.1-beta.2, 2022-12-04** (the version mismatch is itself a maturity flag)
* Contributors (12mo) / commits (12mo): **zero commits since 2023-02-08** — dormant >3.5 years; 2 contributors; README still says "In development"
* Category (A-E): **A** (embedded Gremlin-flavoured LPG over KV) — abandoned pre-alpha

## Hard requirements

| # | Requirement | Verdict | Evidence |
|---|---|---|---|
| 1 | Per-hop edge filtering (stop-gate) | unverifiable | Gremlin semantics would allow it (`repeat(outE().has(..).inV())`) but the implemented step set is undocumented; at 0.0.1-beta.2 assume `repeat()`/`until()` never landed |
| 2 | Native Rust bindings | pass | native Rust |
| 3 | Concurrency model | not documented | — |
| 4 | ACID transactions | **FAIL** | roadmap lists ACID transactions as an unchecked item |
| 5 | Maintenance signal | **FAIL** | abandoned since Feb 2023 |
| 6 | Sync or async (Rule 04) | async (moderate confidence) | async-trait/futures dependencies; docs.rs 404 |

## Modelling fit

Not assessed — eliminated.

## Verdict

**eliminated**

**Reason (one sentence):** Abandoned for 3.5 years at version 0.0.1-beta.2 with ACID still an unchecked roadmap item.

## Uncertainty

None that matters; nothing here would change the verdict.
