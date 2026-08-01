# Screening scorecard — Grafeo

* Repository / homepage: https://github.com/GrafeoDB/grafeo · https://grafeo.dev · crates.io `grafeo`
* Licence: Apache-2.0
* Language / bindings: pure Rust, native crate ("no required C dependencies"); Python/Node/Go/C/C#/Dart/WASM bindings besides
* Latest release + date: v0.5.42, 2026-05-04 (GitHub + crates.io)
* Contributors (12mo) / commits (12mo): 9 contributors, ≥100 commits (API page cap); repo created 2026-01-26; last `main` commit 2026-05-10, `pushed_at` 2026-07-20 (branch activity)
* Category (A-E): **A** — embedded LPG store (RDF/SPARQL advertised as secondary model)

## Hard requirements

| # | Requirement | Verdict | Evidence |
|---|---|---|---|
| 1 | **Per-hop edge-property filtering in recursive traversal** (stop-gate) | **pass** | GQL quantified patterns with WHERE referencing the hop's edge variable: `MATCH (TRAIL (a)-[e:KNOWS]->(b) WHERE e.since > 2020){1,4}` (docs/user-guide/gql/paths.md, "WHERE Inside Parenthesized Patterns (G050)"). Conformance file `docs/gql-dialect.json`: `{"id":"G050","status":"supported","tested":true,"test_count":12}` — verified first-hand. Ordering comparisons shown, so interval predicates (`e.valid_from <= $t AND e.valid_to >= $t`) are expressible. Note: G051 (non-local predicates, i.e. cross-hop) **not supported** — same restriction as Ladybug, consistent with the MemoryRepo constraint |
| 2 | Native or first-class Rust bindings (not FFI wrapper) | pass | pure Rust, Rust-first crate |
| 3 | Concurrency model permits intended deployment | pass | MVCC snapshot isolation; readers do not block writers; optimistic write-write conflict detection (docs/user-guide/transactions.md). In-process; single API-server topology fine. Explicit multi-process story not documented |
| 4 | ACID transactions | pass | "Grafeo provides ACID transactions with Snapshot Isolation semantics"; read_committed / snapshot (default) / serializable levels (transactions.md) |
| 5 | Maintenance signal | **caution** | ~6 months old (created 2026-01-26); 9 contributors, 722 stars, 1,529 commits; but ~2.5-month gap on `main` since 2026-05-10 and no known funding model. No long-term track record by construction |
| 6 | **Sync or async trait shape** (Rule 04) | sync | `session.execute(...)?` — blocking API, no tokio (docs/user-guide/rust/database.md); persistence requires `wal` feature |

## Modelling fit

| Question | Answer |
|---|---|
| Can location tiers be structurally enforced (ADR-0009)? | Unverified — typed node/edge tables with polymorphic endpoints not confirmed in Stage A; test in Phase 1b |
| Can relations carry attributes, including intervals? | Yes (LPG; edge properties in patterns) |
| Native interval/range type or overlap operator? | Not established; INT64 pairs + comparisons suffice |
| Bulk load path for 1e6 edges? | Not established in Stage A; test in Phase 1b |

## Verdict

**advance to Phase 1b**

**Reason (one sentence):** Only candidate passing all five hard requirements with the stop-gate verified against a machine-readable conformance file, at the cost of a six-month-old project with no maintenance track record.

## Uncertainty

Whether G050 per-hop filtering holds up under load and at depth (12 tests prove existence, not performance); multi-writer semantics beyond optimistic conflict detection; tier-constraint enforceability; bulk-load path; the May–July `main` gap and the project's funding model. Cypher-dialect variable-length patterns have NO documented inline predicate — Q1 must use the GQL dialect.
