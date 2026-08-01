# Screening scorecard — GraphLite

* Repository / homepage: https://github.com/GraphLite-AI/GraphLite · crates.io `graphlite`
* Licence: Apache-2.0
* Language / bindings: pure Rust (sled-backed — despite the name, NOT layered over SQLite); Python/Java via FFI on top
* Latest release + date: crates.io 0.0.1, **2025-11-21** — the only version ever published; no GitHub releases or tags; repo is ~6 months ahead of the published crate
* Contributors (12mo) / commits (12mo): 28 commits in last 52 weeks across 6 contributors; **last commit 2026-01-30 — stalled 6 months** with 24 open issues rising
* Category (A-E): **A** — embedded LPG store with ISO GQL 2024 query language

## Hard requirements

| # | Requirement | Verdict | Evidence |
|---|---|---|---|
| 1 | Per-hop edge-property filtering in recursive traversal (stop-gate) | **unverified — parse-level only** | Parser accepts a property map inside brackets plus an ISO-GQL quantifier after them (`opt(property_map) … opt(path_quantifier)` in `src/ast/parser.rs`), and quantified paths are tested (`-[:NEXT]{1,3}->`). But no test or doc exercises property map + quantifier together, and executor enforcement per intermediate hop is unconfirmed. Property maps are equality-only; a WHERE-style per-hop inequality (needed for interval predicates) was not found |
| 2 | Native Rust bindings | pass | pure Rust |
| 3 | Concurrency model | **not documented** | nothing on writers/readers/multi-process; sled's constraints would apply but are not discussed |
| 4 | ACID transactions | claimed, unsubstantiated | "ACID Transactions", "isolation levels" asserted; no isolation level ever named; no durability/crash-recovery discussion |
| 5 | Maintenance signal | **FAIL** | development stopped 2026-01-30; sole crates.io publish is 6 months stale; maintainers position it as a "reference implementation" for ISO GQL, not production infrastructure (HN thread) |
| 6 | Sync or async (Rule 04) | sync | `session.execute()` blocking |

## Modelling fit

Not assessed further — eliminated on requirement 5 with requirement 1 unverified.

## Verdict

**eliminated**

**Reason (one sentence):** A genuine embedded ISO-GQL store whose per-hop filtering is plausible but unproven, stalled for six months at crate version 0.0.1 and self-described as a reference implementation — failing the maintenance requirement and leaving the stop-gate unverifiable without a spike the outcome does not justify.

## Uncertainty

Whether the executor enforces edge property maps per hop in quantified paths; whether inequality predicates per hop are expressible at all (equality-only property maps would fail Q1's window predicate regardless); isolation and durability semantics. Re-screen if the project revives. Likely the referent of the unlocatable "forGQL" list entry — confirm with the owner.
