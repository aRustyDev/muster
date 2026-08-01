# Screening scorecard — Grust

* Repository / homepage: https://github.com/querygraph/grust · crates `grust-graph`/`grust-core`/`grust-cypher` + backend adapters. (The crates.io crate literally named `grust` is an unrelated 2015 GObject bindings project — do not conflate)
* Licence: Apache-2.0 OR MIT
* Language / bindings: native Rust; facade + adapter crates (SurrealDB, PostgreSQL/SQL-PGQ, FalkorDB, HelixDB, LadybugDB, LanceDB, Turso, in-memory test store)
* Latest release + date: 0.12.0, 2026-07-04; README self-describes as "pre-release"
* Contributors (12mo) / commits (12mo): repo created **2026-05-31 (~2 months old)**; 293 commits; contributor breadth unestablished
* Category (A-E): **E-adjacent, but not a store** — a backend-neutral graph *API facade*; "Grust itself has no persistent storage; it delegates to backends"

## Hard requirements

| # | Requirement | Verdict | Evidence |
|---|---|---|---|
| 1 | Per-hop edge-property filtering in recursive traversal (stop-gate) | **FAIL** | **no variable-length/recursive traversal is documented at all**; WHERE support is "property comparisons against literals or parameters joined with AND" on single steps |
| 2 | Native Rust bindings | pass | native Rust |
| 3 | Concurrency model | n/a | delegated entirely to backends |
| 4 | ACID transactions | **FAIL as-a-thing-itself** | "the default mutation path is ordered but not atomic"; only pgGraph and SurrealDB adapters report Transactional |
| 5 | Maintenance signal | weak | two months old, pre-release, README trails published versions |
| 6 | Sync or async (Rule 04) | **async** | `GraphStore` trait is `async fn` throughout — would force an async repository trait |
|

## Modelling fit

Not assessed — not a datastore; every persistence property belongs to the chosen backend.

## Verdict

**eliminated**

**Reason (one sentence):** Not a database but a two-month-old pre-release facade with no recursive traversal, non-atomic default mutations, and an async-only API — it answers a different question than ADR-0015 asks.

## Uncertainty

Whether its LadybugDB/Turso adapters ever become a useful *implementation shortcut* for our own repository trait — worth a glance at Phase 7, nothing more.
