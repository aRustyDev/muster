# Screening scorecard — sqlitegraph

* Repository / homepage: https://github.com/oldnordic/sqlitegraph · crates.io `sqlitegraph` (distinct from the C extension `simple-graph`)
* Licence: **GPL-3.0-only** (verified in crates.io metadata, README, and the LICENSE file; GitHub's "Other" label is a detection artefact)
* Language / bindings: native Rust crate linking SQLite; Python bindings
* Latest release + date: 3.9.0, 2026-07-11 (9 minor/patch releases inside two weeks of June–July 2026; no GitHub releases/tags)
* Contributors (12mo) / commits (12mo): 1,170 commits, **1,161 by one author**; 12 stars, 0 open issues (no external user base filing anything)
* Category (A-E): **E** — graph API over SQLite (the taxonomy's "sleeper" shape); experimental native backend would be A

## Hard requirements

| # | Requirement | Verdict | Evidence |
|---|---|---|---|
| 1 | Per-hop edge-property filtering in recursive traversal (stop-gate) | **FAIL — verified in source** | `cypher.rs` grammar: variable-depth exists (`[:X*1..3]`) but every WHERE form is node-only; no relationship variable binding, no edge property map. Executor `execute_variable_depth()` calls `k_hop_filtered(... &edge_types)` — filters edge *labels* per hop, then applies node/WHERE checks **only to start and end nodes**. Intermediate hops' edge properties are never inspected. README flags "multi-hop Cypher still fails on V3" |
| 2 | Native Rust bindings | pass | native Rust |
| 3 | Concurrency model | restrictive | handle is `!Sync` (RefCell interior mutability) — single-thread writes; concurrent reads only via cloned `GraphSnapshot`s |
| 4 | ACID transactions | partial | SQLite backend inherits SQLite transactional guarantees; native/combined backends have only "atomic batch commits" prose |
| 5 | Maintenance signal | **weak** | solo author at extreme commit velocity, zero external users, no tags; GPL-3.0-only is additionally incompatible with a permissively-licensed product |
| 6 | Sync or async (Rule 04) | sync | blocking core API |

## Modelling fit

Not assessed further — eliminated on requirement 1.

## Verdict

**eliminated**

**Reason (one sentence):** The one genuine category-E candidate — the shape RESEARCH-0005 called most likely to win — fails the stop-gate in verified source (edge properties are never inspected mid-traversal), which refutes the sleeper hypothesis for this list; GPL-3.0-only and a zero-user maintenance profile would eliminate it independently.

## Uncertainty

Durability of the native/combined backends; what "MVCC delta logging" isolates; whether the author would accept per-hop filtering upstream (moot given licence).
