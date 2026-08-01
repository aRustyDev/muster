# Screening scorecard — agdb (Agnesoft Graph Database)

* Repository / homepage: https://github.com/agnesoft/agdb · https://agdb.agnesoft.com · crates.io `agdb`
* Licence: Apache-2.0
* Language / bindings: pure Rust, native crate; other languages via the separate server mode only
* Latest release + date: 0.13.2, 2026-07-28 (GitHub + crates.io; 44 versions total)
* Contributors (12mo) / commits (12mo): 136 commits in last 52 weeks; 6 contributors all-time but ~1 primary (michaelvlach 796/~930); issues actively closed through 2026-07
* Category (A-E): **A** — embedded LPG store, memory-mapped file, no text query language (Rust query-builder objects)

## Hard requirements

| # | Requirement | Verdict | Evidence |
|---|---|---|---|
| 1 | **Per-hop edge-property filtering in recursive traversal** (stop-gate) | **pass** | Search conditions are evaluated element-by-element during the walk: "conditions are applied one at a time to each visited element" and `Beyond`/`NotBeyond` "control traversal (whether the search continues or stops at an element)" (docs/references/queries — verified first-hand). Edge conditions exist (`.where_().edge().and().key(..).value(..)`), and `Comparison` includes `GreaterThanOrEqual`/`LessThanOrEqual`, so the per-hop interval predicate is expressible and prunes during traversal. Depth bounded via `Distance` condition |
| 2 | Native or first-class Rust bindings (not FFI wrapper) | pass | pure Rust |
| 3 | Concurrency model permits intended deployment | pass | "either unlimited number of concurrent immutable transactions or exactly one mutable transaction"; caller synchronises with `Arc<RwLock<_>>` (concepts + efficient-agdb guides). Matches the MemoryRepo restrictive model; fine for a single API server |
| 4 | ACID transactions | pass | `Db` documented "full ACID", WAL-based durability: "durability is provided by the write-ahead-log (WAL) file" (concepts guide); `Transaction`/`TransactionMut` first-class |
| 5 | Maintenance signal | **caution** | Continuously maintained since 2023, release 4 days before screen date, working issue backlog — but effectively bus-factor 1 and no known funding model |
| 6 | **Sync or async trait shape** (Rule 04) | sync | `exec()`/`exec_mut()` blocking; no async items in the crate (docs.rs) |

## Modelling fit

| Question | Answer |
|---|---|
| Can location tiers be structurally enforced (ADR-0009)? | Unlikely at schema level — schema-less elements with key-value properties; tier rules would be a code module (as with relational). Verify in Phase 1b |
| Can relations carry attributes, including intervals? | Yes — edges carry arbitrary key-value pairs |
| Native interval/range type or overlap operator? | No; INT64 pairs + `Comparison` ordering operators |
| Bulk load path for 1e6 edges? | Not established in Stage A; `QueryBuilder::insert()` batching exists; test in Phase 1b |

## Verdict

**advance to Phase 1b**

**Reason (one sentence):** Passes all five hard requirements with the strongest maintenance cadence of any category-A candidate, at the cost of a bus factor of one and a programmatic-only query surface.

## Uncertainty

Isolation level never named (docs say "isolation", not snapshot/serializable); whether storage-layer readers block during writes independent of the user-supplied `RwLock`; `Distance` counts edges AND nodes (subgroup depth 5 ≈ distance 10 — harness must account); no schema-level tier enforcement expected; performance entirely vendor-reported.
