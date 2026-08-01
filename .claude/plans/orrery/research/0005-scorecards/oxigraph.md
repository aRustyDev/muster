# Screening scorecard — Oxigraph

* Repository / homepage: https://github.com/oxigraph/oxigraph · crates.io `oxigraph`
* Licence: Apache-2.0 OR MIT
* Language / bindings: Rust, native crate; Python/JS/WASM bindings
* Latest release + date: v0.5.9, 2026-06-18
* Contributors (12mo) / commits (12mo): 286 commits in last 52 weeks; dominant author Tpt plus a real contributor tail; pushed 2026-08-01. Healthy
* Category (A-E): **B** — embedded RDF/SPARQL triple store (RocksDB-backed; optional server)

## Hard requirements

| # | Requirement | Verdict | Evidence |
|---|---|---|---|
| 1 | Per-hop edge-property filtering in recursive traversal (stop-gate) — restated for category B per RESEARCH-0005: can each recursive step filter on that step's edge attributes? | **FAIL** | SPARQL's only recursion is property paths, whose elements are IRIs/operators only; "Variables can not be used as part of the path itself, only the ends" (W3C SPARQL 1.1 §9, quoted). A `*`/`+` path cannot bind intermediate edges, so RDF-star/reified edge annotations (the validity window) cannot be consulted mid-path. Only whole-path endpoints or manual bounded hop-unrolling with per-hop FILTERs — the latter is a 5-way UNION of explicit joins, i.e. Q1 restructured, which is what the stop-gate exists to prevent |
| 2 | Native or first-class Rust bindings | pass | native Rust |
| 3 | Concurrency model | pass, restrictive | snapshot reads during writes in-process; "Only one read-write Store can exist at the same time"; read-only open while another process writes is undefined behaviour (docs.rs `Store`) |
| 4 | ACID transactions | pass, qualified | atomic transactions at "repeatable read"; transactions buffer entirely in memory (docs.rs) |
| 5 | Maintenance signal | pass | active, long-lived, real contributor tail |
| 6 | Sync or async (Rule 04) | sync | blocking API |

## Modelling fit

Not assessed further — eliminated on requirement 1. Additionally the RDF triple model has no native edge properties at all: every relation attribute (windows, priorities, provenance) requires RDF-star triple terms ("preliminary" support) or reification, a poor fit for a model where *every* relation carries a window.

## Verdict

**eliminated**

**Reason (one sentence):** The healthiest project screened, but SPARQL property paths structurally cannot filter on edge annotations mid-path, which fails the stop-gate in exactly the way the category-B caveat predicted.

## Uncertainty

Maturity of SPARQL 1.2/RDF-star support (moot given the path limitation); write throughput (self-reported as unoptimised).
