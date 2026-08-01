# Screening scorecard — OverGraph

* Repository / homepage: https://github.com/bhensley5/overgraph · https://overgraph.io · crates.io `overgraph`
* Licence: MIT OR Apache-2.0
* Language / bindings: pure Rust; Node/Python connectors
* Latest release + date: 0.17.0, 2026-07-22 (crates.io; no GitHub releases); 554 downloads
* Contributors (12mo) / commits (12mo): 23 commits total, effectively single developer; repo created 2026-02-28 (~5 months old); 22 stars
* Category (A-E): **A** — embedded LPG store (log-structured segments, mmap, HNSW vector search)

## Hard requirements

| # | Requirement | Verdict | Evidence |
|---|---|---|---|
| 1 | Per-hop edge-property filtering in recursive traversal (stop-gate) | **FAIL** | Variable-length patterns exist (`[:KNOWS*1..3]`) but the grammar doc (docs/gql-subset.md) shows no inline predicate syntax inside them, and the path-function list has **no `all()`/`any()`/`none()`/`reduce()`** — even a whole-path post-filter over edge properties has no documented expression form |
| 2 | Native Rust bindings | pass | pure Rust |
| 3 | Concurrency model | restrictive, in-process | reads never block writes (mmap segments); writes serialise through a core write queue; **multi-process open "may cause data corruption"**; no named isolation level |
| 4 | ACID transactions | partial | atomic WAL-batch commits with optimistic conflict detection; no isolation level named |
| 5 | Maintenance signal | **weak** | ~5 months old, one developer, 23 commits, negligible adoption |
| 6 | Sync or async (Rule 04) | sync | "Synchronous core API" (README) |

## Modelling fit

Not assessed further — eliminated on requirement 1.

## Verdict

**eliminated**

**Reason (one sentence):** No expressible per-hop (or even whole-path) edge-property filter in the documented query subset, from a five-month-old single-developer project.

## Uncertainty

Whether undocumented syntax exists beyond docs/gql-subset.md (source not audited); isolation semantics.
