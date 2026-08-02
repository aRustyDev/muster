<!-- Imported from design thread 2026-08-01. Fable: review and improve. -->

# RESEARCH 0001 — Ladybug capability grounding

* Date: 2026-08-01
* Method: primary-source documentation review (docs.ladybugdb.com, retrieved
  2026-08-01), supplemented by vendor site and Database of Databases entry.

## Objective

Establish Ladybug's actual capability surface before evaluating it as a sole
system of record, rather than reasoning from recalled Kuzu behaviour.

## Background

Ladybug is a revival fork of Kuzu, which was acquired by Apple and archived in
October 2025. First release November 2025. MIT licence, C++, embedded columnar
property graph with openCypher.

## Findings

### Confirmed capabilities
Property graph model with Cypher; embedded in-process; columnar disk storage;
CSR-based adjacency list and join indices; vectorized and factorized query
processing; multi-core parallelism; serializable ACID transactions. First-class
Rust bindings alongside Python/Node/Java/Go/Swift/C++/C/WASM. Graph algorithms as
an extension (shortest paths, PageRank, Louvain, K-core, SCC, WCC). Read-side
attach to PostgreSQL, SQLite, DuckDB, ADBC, and Iceberg/Delta/Unity. Polymorphic
`FROM`/`TO` — one relationship table can wire multiple node-type pairs.

### Constraint 1 — indexing is primary-key only
Automatic hash primary-key index per node table; ART indexes available for range
queries **on primary keys**; zone maps (min/max) maintained on all columns and
used to skip node groups during scans. **Only one primary key index per node
table.**

Read precisely: indexes are on **node tables**, on the **primary key**, one per
table. No secondary index on arbitrary properties; no index mechanism for
relationship tables at all. `attends.during` and `held.during` live on
relationship tables.

### Constraint 2 — one writer process, no mixing
Either one `READ_WRITE` Database object, or multiple `READ_ONLY` ones — never
mixed, even across processes, because the buffer manager caches disk state and
cannot notify other Database objects. Recommended multi-process pattern is a
single API server embedding the database.

The sharper constraint is **no mixing**: no read-only batch process, CLI, or
Explorer may attach while the server holds the file. SQLite's WAL permits
concurrent readers alongside a writer.

### Constraint 3 — no CDC, no triggers
Neither appears in the documentation navigation, which is comprehensive enough
that absence is meaningful — though this is absence of evidence, not a documented
denial. Attach is inbound scan only; no outbound write or change-feed path.

## Conclusions

1. Change detection must be application-level. The store offers no hook.
2. Interval predicates on relationship properties cannot be indexed; only
   zone-map pruning applies, and zone maps prune only when data is physically
   clustered on the scanned column — which erodes with edits and backfills.
3. The forced API-server topology is acceptable; the no-mixing rule is the real
   operational constraint.

## Sources

* https://docs.ladybugdb.com/ — overview, feature list
* https://docs.ladybugdb.com/cypher/indexes — index model
* https://docs.ladybugdb.com/concurrency — connection and concurrency model
* https://docs.ladybugdb.com/extensions/attach/rdbms — attach surface
* https://dbdb.io/db/ladybugdb — project provenance
