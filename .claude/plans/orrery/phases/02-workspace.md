# Phase 02 — Workspace, seams, and MemoryRepo

* Status: `in-progress`
* Blocks: Phases 3–5 (they build on the model types, interval algebra,
  repository trait, and `MemoryRepo` landed here) and Phase 1b (the Rust
  screening harness reuses the workspace)
* Blocked by: Phase 1a (complete — three candidates advance, trait shape
  fixed sync by ADR-0023)

## Objective

A compiling three-crate workspace exists at the repository root with the
handoff package's artefacts migrated into their target locations (with the
Phase-0 corrections applied, visibly). `orrery` contains the model newtypes,
the interval algebra with property tests against a naive oracle, the
`Command` enum, the `Repository` trait with no concrete datastore type in the
public API, and `MemoryRepo` — which *executably* enforces the restrictive
constraint intersection confirmed in Phase 1a (Rule 00b).

## Hypotheses (pre-committed, before implementation)

| # | Hypothesis | Falsified by | Status |
|---|---|---|---|
| H1 | MemoryRepo's constraints (single writer; no read during open write) can be enforced with **typed errors naming the constraint**, using only std synchronisation, without the trait leaking any enforcement type | enforcement requiring unsafe, global state, or constraint-specific types in the `Repository` signature | untested |
| H2 | Half-open `[start, end)` interval algebra over `i64` microseconds passes property tests against a naive reference, including "exactly one of the 13 Allen relations holds" for positive-length pairs and converse-symmetry | any proptest counterexample | untested |
| H3 | The draft SPEC-04 trait shape suffices for the Q1-shaped read path (memberships → bounded ancestor walk with constant-time per-hop filter → expectations) with only additive refinements | a required signature change that removes or semantically alters a drafted method | untested |
| H4 | Phase 2 needs no dependency beyond the Rule-06 baseline subset {thiserror, serde, uuid v7, tracing} + proptest (dev) | any additional dependency (each would need its own phase-doc line per Rule 06) | untested |
| H5 | Cross-hop traversal predicates are excluded **by construction** — the trait exposes no parameter through which a caller could express one — which is stronger than a runtime panic | any trait method accepting a caller-supplied per-hop callback/predicate over traversal state | untested |

## Acceptance criteria (pre-committed)

| Criterion | Threshold | Actual | Verdict |
|---|---|---|---|
| Workspace builds | `cargo build --workspace` clean | | |
| Lint/format gates | `cargo fmt --check` and `cargo clippy --workspace --all-targets -- -D warnings` clean | | |
| Tests | `cargo test --workspace` green, including proptest suites (default case counts) | | |
| Rule 00b executable | tests assert: second concurrent writer → error naming "single-writer"; read during open write → error naming "read-during-write" | | |
| Seam (Rule 00.1) | no concrete datastore identifier (sqlite/ladybug/cozo/grafeo/agdb/rusqlite) in `orrery`'s public API (grep of public surface; cargo-public-api when installed) | | |
| Interval constructors | inverted intervals rejected; zero-length only via the explicit constructor (Rule 04) | | |
| Migration audit | `./docs/scripts/check-xrefs.sh` passes from the repo root after migration | | |
| Corrections visible | RESEARCH-0002/0003 carry dated addenda; harness fixes commented with provenance; no silent edits | | |

## Plan

1. Migrate the handoff package into the target tree (plans, rules, ADRs,
   evidence, agents), applying the Phase-0 queued corrections as **visible,
   dated addenda** — not silent rewrites. Write Rules 08 and 09. Generate
   root/crate/docs context files, book scaffolding, per-product plan
   derivations, and the six roster agents. (Reversible; the untracked
   `orrery-handoff/` stays as the archival source.)
2. Cargo workspace: `crates/{orrery,muster-sdk,muster}`, shared
   `[workspace.dependencies]`, `repo-memory` as default feature.
3. `orrery`: `error` → `model` (id newtypes, entities, relations) →
   `interval` (Timestamp, Interval, Allen relations) → `command` →
   `repo` (trait) → `repo::memory` (MemoryRepo + Rule 00b enforcement) —
   with tracing spans carrying `backend = "memory"` from day one (Rule 05).
4. Property tests: interval algebra vs naive oracle; MemoryRepo constraint
   tests; Q1-shaped read-path integration test on seeded data including a
   mid-chain expired `subgroup_of` edge (the SPEC-05 critical fixture).
5. Run gates, fill Actual/Verdict, merge per Rule 08.

Design notes fixed in advance:

* **Time representation**: `Timestamp(i64 microseconds UTC)` as the
  comparison key. This is safe under all four QUESTION-0014 options — every
  option stores an instant; what remains open there (authoring-zone
  retention, recurrence) does not affect interval algebra. QUESTION-0014
  still must close before Phase 3 (it gates Event fields and fixtures), and
  `chrono` stays out of Phase 2 entirely.
* **Interval semantics**: half-open `[start, end)` — matches every predicate
  in the evidence harness (`a.start < b.end AND b.start < a.end`).
* **Enforcement mechanism**: `std::sync::RwLock` with `try_read`/`try_write`
  mapped to `RepoError::ConstraintViolated { constraint }` — reads *fail*
  rather than block while a write is open, making the restrictive model
  executable rather than documentary.

## Results

*(pending)*

## Decisions produced

*(pending)*

## Carry-forward

*(pending)*
