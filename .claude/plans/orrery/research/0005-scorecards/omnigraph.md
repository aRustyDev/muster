# Screening scorecard — Omnigraph (ModernRelay/omnigraph)

* Repository / homepage: https://github.com/ModernRelay/omnigraph · https://www.omnigraph.dev. (The crates.io crate named `omnigraph` 0.0.1/2023 is an unrelated name-squat; real crates are `omnigraph-engine` etc.)
* Licence: MIT
* Language / bindings: Rust; documented client path is **HTTP** — no documented in-process application API ("Python SDK coming soon")
* Latest release + date: v0.8.1, 2026-07-06; crates 0.8.0, 2026-07-01
* Contributors (12mo) / commits (12mo): 690 commits in 4 months (repo created 2026-04-10); 2 core humans + heavy AI-agent-authored commits; 1,039 stars
* Category (A-E): **C** (server-first lakehouse graph DB on the Lance format), with D characteristics

## Hard requirements

| # | Requirement | Verdict | Evidence |
|---|---|---|---|
| 1 | Per-hop edge filtering (stop-gate) | **FAIL** | query docs: traversal `$a <knows>{1,3} $b` binds no edge variable and shows no edge-property constraint syntax at all |
| 2 | Native Rust bindings | fail as documented | engine crates exist but embedding is undocumented/unsupported; access is HTTP |
| 3 | Concurrency model | mismatched | concurrency is git-style branching for agent fleets; "single-writer apply" cluster ops; in-branch reader/writer semantics undocumented |
| 4 | ACID transactions | **FAIL** | no isolation/durability guarantees in user docs; WAL/write-path design still in-flux (rfc-013, wal-options) |
| 5 | Maintenance signal | young | 4 months old, 2 core developers, well-starred but unproven |
| 6 | Sync or async (Rule 04) | n/a | no in-process API to classify |

## Modelling fit

Not assessed — eliminated on category.

## Verdict

**eliminated (category C)**

**Reason (one sentence):** A four-month-old server-first lakehouse graph platform aimed at agent-fleet context assembly — no embeddable API, no transactions, no edge-property predicates — solving a different problem than ADR-0015's.

## Uncertainty

Whether the mentioned "embedded, local file-backed graph" CLI mode ever becomes a supported library; grammar source may permit edge predicates the docs omit.
