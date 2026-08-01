# Screening scorecard — OxiRS

* Repository / homepage: https://github.com/cool-japan/oxirs · crates.io `oxirs-core`
* Licence: Apache-2.0
* Language / bindings: Rust, native crates (27-crate workspace; Jena/Fuseki-styled)
* Latest release + date: v0.4.1, 2026-07-28
* Contributors (12mo) / commits (12mo): 2 contributors; **13 commits in repo lifetime**, each a giant code-drop per release; 74 stars, 1 open issue; no visible community
* Category (A-E): **B** — embedded RDF/SPARQL toolkit + Fuseki-style server

## Hard requirements

| # | Requirement | Verdict | Evidence |
|---|---|---|---|
| 1 | Per-hop edge filtering (stop-gate, category-B restatement) | **FAIL** | Same W3C SPARQL property-path limitation as Oxigraph: paths cannot bind intermediate edges, so per-hop edge-attribute filtering is inexpressible |
| 2 | Native Rust bindings | pass | native Rust |
| 3 | Concurrency model | unverified | "MVCC layer" claimed in README; no documented semantics |
| 4 | ACID transactions | unverified | "fsync-backed writes" claimed; no explicit ACID statement |
| 5 | Maintenance signal | **weak** | single-author squash-commit releases; headline claims ("Production Ready", "46,255 tests passing") entirely self-reported and structurally unauditable |
| 6 | Sync or async (Rule 04) | both claimed | sync APIs plus `AsyncRdfStore`; coverage unverified |

## Modelling fit

Not assessed — eliminated on requirement 1 (and 5).

## Verdict

**eliminated**

**Reason (one sentence):** Fails the stop-gate for the same structural SPARQL reason as Oxigraph, with a far weaker and largely unauditable maintenance signal on top.

## Uncertainty

Nearly everything beyond crates.io metadata is vendor-asserted: conformance, ACID, concurrency, quality.
