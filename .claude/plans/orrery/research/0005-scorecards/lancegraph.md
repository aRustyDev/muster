# Screening scorecard — LanceGraph

* Repository / homepage: https://github.com/lance-format/lance-graph (crates.io still points at the predecessor lancedb org) · crates.io `lance-graph`
* Licence: Apache-2.0
* Language / bindings: native Rust crate + PyO3 Python bindings
* Latest release + date: v0.5.4, 2026-03-21; 63,844 downloads
* Contributors (12mo) / commits (12mo): 119 commits total, 13 contributors, last commit 2026-06-21; active
* Category (A-E): **D** — a Cypher→DataFusion **query engine over Lance columnar datasets**, read-only; not a system of record

## Hard requirements

| # | Requirement | Verdict | Evidence |
|---|---|---|---|
| 1 | Per-hop edge-property filtering in recursive traversal (stop-gate) | **FAIL** | planner source: `build_expand` (single hop) accepts relationship properties, but `build_variable_length_expand(...)` takes **no relationship-property parameter** and compiles var-length as "1-hop UNION 2-hop UNION 3-hop"; no `relationships(p)`/list-predicate post-filter functions found either (crates/lance-graph/src/datafusion_planner/builder/expand_ops.rs) |
| 2 | Native Rust bindings | pass | native Rust |
| 3 | Concurrency model | n/a | no write path exists to have writer semantics |
| 4 | ACID transactions | **FAIL** | no transactions, no writes — "queries operate on PyArrow tables passed at execution time"; durability belongs to the underlying Lance format |
| 5 | Maintenance signal | pass | 13 contributors, active org (lance-format) |
| 6 | Sync or async (Rule 04) | **async** | `async fn execute`, tokio throughout |

## Modelling fit

Not assessed — category D; there is nothing to be a system of record with.

## Verdict

**eliminated (category D — record, not SoR).** Owner flagged high interest, so stating it plainly per the mandate: **LanceGraph cannot hold Orrery's data at all** — it is a read-only query layer.

**Reason (one sentence):** A read-only Cypher-on-DataFusion analytics engine with no write path, no transactions, and no per-hop filtering in variable-length expansion.

## Uncertainty

The supported Cypher subset is undocumented (inferred from parser/planner/tests); the exact `MAX_VARIABLE_LENGTH_HOPS` value. If a future Lance-native *store* with transactions emerges from this org, that is a different candidate.
