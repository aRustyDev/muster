# Phase 02 — Workspace, seams, and MemoryRepo

* Status: `complete`
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
| H1 | MemoryRepo's constraints (single writer; no read during open write) can be enforced with **typed errors naming the constraint**, using only std synchronisation, without the trait leaking any enforcement type | enforcement requiring unsafe, global state, or constraint-specific types in the `Repository` signature | **confirmed** — `std::sync::RwLock` `try_read`/`try_write` → `OrreryError::ConstraintViolated{constraint}`; no unsafe, no globals, trait untouched |
| H2 | Half-open `[start, end)` interval algebra over `i64` microseconds passes property tests against a naive reference, including "exactly one of the 13 Allen relations holds" for positive-length pairs and converse-symmetry | any proptest counterexample | **confirmed after one real defect** — the first `overlaps()` implementation contradicted its own documented empty-interval semantics (a zero-length point "overlapped" enclosing intervals); the unit suite caught it pre-commit. Fixed; all 7 property suites green |
| H3 | The draft SPEC-04 trait shape suffices for the Q1-shaped read path with only additive refinements | a required signature change that removes or semantically alters a drafted method | **confirmed** — refinements were purely additive: entity `Upsert*` commands (the draft had no way to create entities, yet Rule 00.2 routes *all* mutations through the command layer), `AddSubgroup`, `AddTraversePair` (ADR-0008's single write site), `by: Actor` on `AddExpectation` (provenance), `travel` returning `TravelCost` |
| H4 | Phase 2 needs no dependency beyond the Rule-06 baseline subset {thiserror, serde, uuid v7, tracing} + proptest (dev) | any additional dependency | **confirmed** — exactly that set; `anyhow` in `muster` only (Rule 04) |
| H5 | Cross-hop traversal predicates are excluded **by construction** | any trait method accepting a caller-supplied per-hop callback/predicate over traversal state | **confirmed** — traversal methods take only constant `at: Timestamp`; there is no predicate-bearing parameter anywhere in `Repository` |

## Acceptance criteria (pre-committed)

| Criterion | Threshold | Actual | Verdict |
|---|---|---|---|
| Workspace builds | `cargo build --workspace` clean | clean (rustc 1.97.1, 2026-08-01, this host) | pass |
| Lint/format gates | `cargo fmt --check` and `cargo clippy --workspace --all-targets -- -D warnings` clean | both clean | pass |
| Tests | `cargo test --workspace` green, including proptest suites (default case counts) | 9 unit + 7 proptest suites (256 cases each), all green | pass |
| Rule 00b executable | tests assert: second concurrent writer → error naming "single-writer"; read during open write → error naming "read-during-write" | `second_writer_errors`, `read_during_open_write_errors` — both assert the constraint name in the error | pass |
| Seam (Rule 00.1) | no concrete datastore identifier in `orrery`'s public API | grep of `crates/orrery/src` outside comments: clean (cargo-public-api not installed on this host — re-run `just orrery::check-seam` once it is) | pass |
| Interval constructors | inverted rejected; zero-length only via explicit constructor | `Interval::new` rejects `end <= start`; `at_point` is the explicit path; tested | pass |
| Migration audit | `./docs/scripts/check-xrefs.sh` passes from repo root | green (dangling: none; unqualified SPEC: none) | pass |
| Corrections visible | dated addenda, no silent edits | RESEARCH-0002/0003 addenda; ADR-0015 provenance note; harness fixes commented in-file | pass |

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

Refutations and defects first (Rule 01.3):

* **One real implementation defect caught by the gate**: `overlaps()` as
  first written returned `true` for a zero-length interval against an
  enclosing one, contradicting its own doc ("empty intervals overlap
  nothing"). The unit suite caught it before commit; fixed with an
  `is_empty` guard, and the property oracle (`max(start) < min(end)`)
  independently agrees. A reminder that the doc-comment is a claim, not a
  fact, until a test says so.
* One test defect: an exact `f32` equality on the divergence analytic
  (`|0.1 − 0.9|` ≠ `0.8` exactly in f32); replaced with a tolerance.

Landed: three-crate workspace (`orrery` real; `muster-sdk`/`muster`
compiling stubs); `Timestamp`/`Interval` with the 13 Allen relations,
converse, merge, and the shared overlap predicate; id newtypes (UUIDv7,
ADR-0022); all SPEC-01 relations including `Anchors` (with its Rule-09
warning) and the effective-priority stack computed in exactly one place
(ADR-0005); `Command` enum with stable variant names for spans and the
future event log; sync `Repository` trait (ADR-0023) with cross-hop
predicates unrepresentable; `MemoryRepo` with executable Rule-00b
enforcement and `backend = "memory"` tracing spans on every operation
(Rule 05, from the first implementation). The Q1-shaped integration test
exercises the SPEC-05 critical fixture — a mid-chain expired `subgroup_of`
edge — and asserts depth-0 expectations count (the semantics Phase 0 showed
the original harness got wrong).

Also in this phase: the handoff package migrated into the target tree with
all Phase-0 corrections applied visibly (RESEARCH-0002/0003 addenda,
ADR-0015 provenance note, harness fixes, count-drift fixes, audit-script
hardening); Rules 08–09 written; six roster agents created; per-product
plan derivations; root/crate/docs context files per Rule 07.

## Decisions produced

* No new ADRs — ADR-0023 (sync trait) was Phase 1a's. Command-set additive
  refinements recorded under H3 above; SPEC-04 remains the draft they refine
  (spec update queued for Phase 3 when `Derive` lands).
* QUESTION-0014 deliberately **not** closed here: `Timestamp(i64 µs UTC)` is
  safe under all four options; the open part (zone retention, recurrence,
  DST fixtures) gates Phase 3.

## Carry-forward

| Item | Resolves in |
|---|---|
| Close QUESTION-0014 (zone retention on Event, recurrence ownership, DST fixtures per SPEC-05) | before Phase 3 detectors |
| SPEC-04 updated from draft to match the landed trait + command set | Phase 3 |
| `cargo-nextest` + `cargo-public-api` not installed on this host — `just ci`/`check-seam` degrade; install or adapt justfile | next toolchain touch |
| Violation creation path (detectors write violations; only waive exists today) | Phase 3 |
| Tier-legality checks on `within`/`traverse` (one module + exhaustive tests, relational-style) | Phase 3 |
| Phase 1b screening harness reuses this workspace (Grafeo, agdb, Cozo) | Phase 1b |
