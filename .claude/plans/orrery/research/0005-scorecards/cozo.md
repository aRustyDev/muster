# Screening scorecard — Cozo (CozoDB)

* Repository / homepage: https://github.com/cozodb/cozo · https://www.cozodb.org · crates.io `cozo`
* Licence: MPL-2.0
* Language / bindings: Rust, native crate (`DbInstance`); many other bindings
* Latest release + date: v0.7.6, **2023-12-11** (GitHub + crates.io — no release in >2.5 years)
* Contributors (12mo) / commits (12mo): **0 commits since 2025-08-01** (GitHub commits API); last `main` commit 2024-12-04; effectively single-author (zh217: 1,735 of ~1,813 commits)
* Category (A-E): **A** — embedded Datalog (CozoScript) relational-graph store; backends: mem, SQLite, RocksDB, Sled, TiKV

## Hard requirements

| # | Requirement | Verdict | Evidence |
|---|---|---|---|
| 1 | Per-hop edge-property filtering in recursive traversal (stop-gate) | **pass (semantics)** | Recursion is native to Datalog: a recursive rule's body joins the edge relation and applies conditions at every expansion step (e.g. the shortest-distance rule in docs.cozodb.org/en/latest/queries.html). A validity-window column is just another filtered attribute per step — exactly the shape RESEARCH-0005's "note on Cozo" anticipated. Also first-class time travel per relation |
| 2 | Native or first-class Rust bindings | pass | native Rust crate |
| 3 | Concurrency model | pass (per backend) | MVCC; mem = "concurrent readers but only a single writer"; RocksDB = "extremely high level of concurrency" (docs.rs) |
| 4 | ACID transactions | partial | MVCC + multi-statement transactions documented; per-engine durability guarantees not spelled out |
| 5 | Maintenance signal | **FAIL** | No release since 2023-12; no commit since 2024-12; bus factor 1; 48 open issues; no archive notice or maintainer statement either way. Fails Rule 06's own bar (release within a year, >1 contributor) |
| 6 | Sync or async trait shape (Rule 04) | sync | `db.run_script(...)` blocking (docs.rs) |

## Modelling fit

Not assessed further — eliminated on requirement 5.

## Verdict

**eliminated**

**Reason (one sentence):** The best semantic fit for Q1 on the entire list — native recursion with per-step edge constraints plus built-in time travel — but the project has had no release in 2.5 years and no commit in 20 months, and adopting an unmaintained single-author store as system of record fails the pre-committed maintenance requirement.

## Uncertainty

Whether maintenance resumes (no archive notice exists); formal durability semantics per backend; recursion-with-filter performance (vendor QPS claims only). If Cozo revives, it deserves an immediate re-screen — record, do not forget.
