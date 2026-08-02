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

## Addendum — write-path investigation (owner question, 2026-08-01)

The owner asked whether a Lance *write* path could pair with LanceGraph to
form an in-process store. Verified against current docs (URLs cited inline):

**Write surfaces exist and are real.** The `lance` crate (v9.0.0, 2026-07-24,
lance-format org; docs.rs `Dataset`) offers full in-process CRUD —
`write`/`append`/`delete`-by-predicate, `UpdateBuilder`, `MergeInsertBuilder`,
explicit commit builders. The `lancedb` crate (v0.33.0, embedded/in-process,
lancedb org) gives table-level `add`/`update`/`delete`/`merge_insert`.
Per-dataset ACID via MVCC with optimistic concurrency is in the format spec
(lance.org/format/table/transaction: "MVCC to provide ACID transaction
guarantees for concurrent readers and writers").

**Why the pairing still fails as SoR — five findings, decisive first:**

1. The stop-gate failure is unchanged: writes don't touch the query engine,
   and LanceGraph's variable-length expansion still takes no relationship
   properties. Q1 would be restructured into app-side chained single hops —
   the architectural concession ADR-0021 blocks on.
2. No cross-dataset atomicity, by spec — commits are per-table only; Orrery
   commands touching several relations would need hand-built ordering +
   idempotency under Rule 00.2.
3. Write-shape mismatch: every commit is a new immutable version, no WAL in
   OSS; docs flag per-row writes as pathological and a documented failure
   mode exists (lancedb#3086: 5,000 single-row adds → ~800 MB, cleanup
   wedging writes). Muster's interactive workload is exactly this shape.
4. Both crates are async-only (tokio) — conflicts with ADR-0023.
5. Version skew today: lance-graph pins `lance =1.0.0` vs current lance
   9.0.0 (lancedb pins 8/9) — they cannot share Arrow types in one binary;
   lance-graph also has no refresh contract (readers pinned at open; reopen
   per version is the caller's job; CSR rebuild cost undocumented).

**Where Lance does fit:** ADR-0015 already requires ingest/egress via a
portable format; Lance (or Parquet, which Lance interoperates with) is a
strong egress-format candidate, and a Lance/LanceGraph analytics sidecar
could be revisited at Phase 7 without touching the SoR decision. Verdict
unchanged: eliminated as system of record.
