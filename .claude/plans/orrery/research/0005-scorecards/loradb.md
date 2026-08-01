# Screening scorecard — LoraDB (list entry "LoraGraph")

* Repository / homepage: https://github.com/lora-db/lora · https://loradb.com · crates.io `lora-database`. "LoraGraph" as given does not exist; LoraDB is the evident referent (owner should confirm)
* Licence: **Business Source License 1.1** (converts to Apache-2.0 on 2029-04-19) — not open source until then; embedding allowed, hosting-as-a-service not
* Language / bindings: Rust, native crate; other languages via a shared C-ABI FFI layer
* Latest release + date: v0.15.0, 2026-05-28
* Contributors (12mo) / commits (12mo): **1 contributor**; ~100 commits all within Apr–May 2026; repo ~3.5 months old; no commits in the ~2 months before the screen date
* Category (A-E): **A** — embedded in-memory-first LPG store with WAL/snapshot persistence

## Hard requirements

| # | Requirement | Verdict | Evidence |
|---|---|---|---|
| 1 | Per-hop edge-property filtering in recursive traversal (stop-gate) | **FAIL — by the project's own support matrix** | docs/reference/cypher-support-matrix.md: "Inline WHERE inside variable-length — Not yet implemented — Not in grammar"; quantified path patterns listed as future. Whole-path post-filter (`all(x IN path.edges(p) WHERE ...)`) is the only expressible form |
| 2 | Native Rust bindings | pass | native Rust |
| 3 | Concurrency model | pass, restrictive | single writer (writer mutex) + concurrent readers on an `ArcSwap` store pointer |
| 4 | ACID transactions | unverified | explicit transactions + WAL durability described; no ACID/isolation claim anywhere |
| 5 | Maintenance signal | **FAIL** | single author, 3.5 months old, 2-month commit gap, BSL licence |
| 6 | Sync or async (Rule 04) | sync | blocking `db.execute(...)` |

## Modelling fit

Not assessed further — eliminated on requirements 1 and 5.

## Verdict

**eliminated**

**Reason (one sentence):** The project's own Cypher support matrix says per-hop filtering is not in the grammar, and a single-author BSL project three months old with a two-month commit gap fails the maintenance bar besides.

## Uncertainty

Identity ("LoraGraph" → LoraDB) is high-confidence but owner should confirm; whether inline property maps work inside variable-length patterns (matrix only rules out inline WHERE — moot given equality-only anyway).
