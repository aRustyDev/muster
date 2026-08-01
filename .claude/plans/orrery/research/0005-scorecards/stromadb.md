# Screening scorecard — StromaDB

* Repository / homepage: https://github.com/katsut/stromadb · crates.io `stromadb-store` et al.
* Licence: **Elastic License 2.0** — source-available, not open source; forbids offering as a managed service
* Language / bindings: native Rust workspace (`stromadb-store` embeds; HTTP/MCP servers separate)
* Latest release + date: v0.2.0, 2026-07-30
* Contributors (12mo) / commits (12mo): 182 commits, **all by one author**; repo created 2026-06-23 — six weeks old; 3 stars, ~zero adoption
* Category (A-E): **A with a strong caveat, borderline D** — embedded durable fact store, but the query surface is a fixed GraphRAG retrieval pipeline (`point / type-ANN / expand / filter / top-k`), not a graph query language

## Hard requirements

| # | Requirement | Verdict | Evidence |
|---|---|---|---|
| 1 | Per-hop edge-property filtering in recursive traversal (stop-gate) | **FAIL** | `expand` takes exactly one predicate (edge type) + `max_depth` + `valid_at`; no per-hop predicate expression exists, and `filter` applies only to the returned node set — the whole-path post-filter pattern. Notable near-miss: `valid_at` DOES apply at every hop ("with valid_at every hop answers from the state in effect at T") — per-hop *temporal* semantics exist, but only against the store's bitemporal versioning, not against arbitrary edge attributes like Q1's relation windows |
| 2 | Native Rust bindings | pass | native Rust |
| 3 | Concurrency model | **FAIL for deployment** | "Pre-1.0: single-node, single-threaded serving"; a write holds the write mutex for its entire ETL |
| 4 | ACID transactions | **FAIL** | WAL durability + atomic ingest batches only; no transaction API; the word ACID does not appear |
| 5 | Maintenance signal | **FAIL** | six weeks old, solo author, ELv2, no adoption |
| 6 | Sync or async (Rule 04) | sync | blocking `Db` API |

## Modelling fit

Not assessed further — eliminated on requirements 1, 3, 4, 5.

## Verdict

**eliminated**

**Reason (one sentence):** A six-week-old solo GraphRAG pipeline with no transactions and no per-hop predicates — four independent hard-requirement failures — though its every-hop `valid_at` semantics are worth remembering as prior art for bitemporal traversal.

## Uncertainty

Whether facts/edges can carry arbitrary filterable properties at all (docs describe typed predicates, not property maps).
