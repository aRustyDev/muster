# Screening scorecard — ocg

* Repository / homepage: crates.io `ocg` / docs.rs; declared repository is **https://github.ibm.com/enjoycode/ocg — IBM-internal, not publicly reachable**
* Licence: Apache-2.0
* Language / bindings: pure Rust (petgraph-based default backend + 3 alternates behind a `GraphBackend` trait); Python via PyO3
* Latest release + date: 0.4.5, 2026-02-26; all publishes fall in a 3-day window (2026-02-23→26), nothing since; 215 downloads
* Contributors (12mo) / commits (12mo): **unverifiable** — source behind IBM's firewall; single visible owner
* Category (A-E): **A** — embedded, in-memory-first openCypher LPG engine with optional WAL/Parquet persistence

## Hard requirements

| # | Requirement | Verdict | Evidence |
|---|---|---|---|
| 1 | Per-hop edge-property filtering in recursive traversal (stop-gate) | **unverifiable / likely FAIL** | openCypher 9 has no quantified-pattern WHERE; the only per-hop mechanism is a property map on a variable-length relationship (grammar permits `*range` + `{props}` together), which is **equality-only** — Q1's interval predicate needs `<=`/`>=` per hop, inexpressible in that form. No ocg-specific extension is documented, and the source cannot be read |
| 2 | Native Rust bindings | pass | pure Rust |
| 3 | Concurrency model | unverified | "MVCC transaction support for snapshot-isolated graph operations" (docs.rs); single-vs-multi-writer undocumented |
| 4 | ACID transactions | partial claim | MVCC snapshot isolation claimed; WAL durability undocumented |
| 5 | Maintenance signal | **FAIL** | source unauditable (internal GitHub Enterprise), one-burst publish then 5 months quiet, self-reported TCK numbers that disagree with each other across lib.rs/crates.io/docs.rs (3,897/3,897 vs 3,874/3,897 vs "96.4%") |
| 6 | Sync or async (Rule 04) | sync | `execute()` blocking (docs.rs) |

## Modelling fit

Not assessed further — eliminated on requirement 5 with requirement 1 unverifiable.

## Verdict

**eliminated**

**Reason (one sentence):** A closed-development crate whose maintenance, concurrency, and conformance claims cannot be audited fails the maintenance requirement outright, and openCypher's equality-only per-hop property maps cannot express Q1's interval predicate anyway.

## Uncertainty

Everything not on crates.io/docs.rs. If the source ever becomes public and the burst pattern turns into cadence, re-screen.
