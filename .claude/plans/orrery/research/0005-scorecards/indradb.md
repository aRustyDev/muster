# Screening scorecard — IndraDB (list entry "IndraGraph")

* Repository / homepage: https://github.com/indradb/indradb · crates.io `indradb-lib`. "IndraGraph" is unlocatable; **IndraDB** is the correct current name (RESEARCH-0005's own guess was right)
* Licence: MPL-2.0
* Language / bindings: native Rust crate (`indradb-lib`); separate async gRPC server/client
* Latest release + date: v5.0.0, 2025-08-16
* Contributors (12mo) / commits (12mo): **1 commit in the last 52 weeks** (the v5.0.0 release commit); single maintainer (ysimonson 904/~962); newest issue (2025-09-12) unanswered
* Category (A-E): **A** — embedded typed property-graph library with pluggable backends (RocksDB/memory/sled/postgres)

## Hard requirements

| # | Requirement | Verdict | Evidence |
|---|---|---|---|
| 1 | Per-hop edge-property filtering in recursive traversal (stop-gate) | **partial** | Per-hop yes: `QueryExt` chains `outbound()` → `with_property_equal_to(..)` per hop. Recursive no: there is **no variable-length primitive in the `Query` enum at all** — depth must be manually unrolled hop by hop (Q1's bound of 5 makes that possible but ugly). Equality-only property filters also cannot express the interval inequality per hop |
| 2 | Native Rust bindings | pass | native Rust |
| 3 | Concurrency model | **not documented** | no statement on writers/readers/isolation anywhere in the docs |
| 4 | ACID transactions | **FAIL** | no user-facing transaction API; internal `Transaction` trait has no commit/rollback among its 26 methods; durability is a manual `sync()` that "potentially [is] a no-op" per backend |
| 5 | Maintenance signal | **FAIL** | dormant ~11.5 months; bus factor 1; issues unanswered since before the last commit |
| 6 | Sync or async (Rule 04) | sync (lib) | `db.get(q)?` blocking; gRPC client is the async surface |

## Modelling fit

Not assessed further — eliminated on requirements 4 and 5 (and 1 borderline).

## Verdict

**eliminated**

**Reason (one sentence):** No transactions, equality-only property filters, no recursion primitive, and a year of dormancy — three independent hard-requirement failures.

## Uncertainty

Whether v5.0.0's backends provide any transactional semantics internally (undocumented either way).
