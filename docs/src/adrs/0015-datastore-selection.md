<!-- Imported from design thread 2026-08-01. Review, do not assume correct. -->

# 15. Datastore selection

* Status: **proposed — OPEN.** Selection process defined in ADR-0021.
* Date: 2026-08-01
* Deciders: pending

## Context and Problem Statement

Orrery needs a system of record. Candidate classes: embedded property graph,
embedded relational, server relational.

## Decision Drivers

* Every Orrery query is **entity-partitioned** (by person, or by room) before
  its interval predicate applies.
* Containment/tier rules benefit from structural enforcement (ADR-0009).
* Rust is the implementation language — bindings maturity matters.
* Embedded deployment is preferred.

## Considered Options

Evaluated empirically at 1M `attends` edges (see RESEARCH-0002, RESEARCH-0003).

### Measured baseline

| Query | Ladybug 0.19 | SQLite 3.45 | Winner |
|---|---:|---:|---|
| Q1 derived expansion | 4.3 ms | **0.2 ms** | SQLite 21× |
| Q2 per-person conflict | 3.4 ms | **0.6 ms** | SQLite 5.7× |
| Q3 global sweep | **2,051 ms** | 4,775 ms | Ladybug 2.3× |
| Q5 room exclusivity | 16.7 ms | 15.8 ms | tie |
| Q7b co-attend 2-hop | 11.3 ms | **1.2 ms** | SQLite 9.4× |
| Bulk load | 66.7 s | **20.8 s** | SQLite 3.2× |
| On-disk | **41 MB** | 123 MB | Ladybug 3× |

Both engines returned identical results on every shared query.

> *Phase-0 provenance note (2026-08-01, factual correction per Rule 02):
> these figures come from RESEARCH-0003's corrected run, whose scripts the
> handoff did not package. The Q7b 1.2 ms presumes the `attends(event_id)`
> index (now restored in `evidence/`); the Q1 row compared `*1..5`-matched
> semantics — the corrected harness uses the domain-correct depth-0-inclusive
> form, identical results 58/58. Winners and the "3 of 5" count are
> unaffected. Re-measure this baseline on the decision host before closing
> this ADR. Details: phases/00-grounding.md.*

### Evidence summary

**For relational:** of five directly comparable queries SQLite wins three
(Q1, Q2, Q7b) by 5.7-21x, loses one (Q3), and ties one (Q5); it also loads 3.2x
faster. Q4 and Q6 have no SQLite counterpart and are excluded from the count.
Mechanism: Orrery's queries partition by entity first, leaving partitions of
tens of rows — exactly what b-tree-indexed row stores do best and where
columnar scan-oriented engines are weakest.

**For graph:** tier constraints become *unrepresentable* rather than merely
invalid (5/5 verified); wins unpartitioned global sweeps 2.3×; 3× smaller
on disk.

**Falsified during the spike** (see RESEARCH-0003):
* SQLite R\*Tree interval indexing — measured **2× slower** than a plain
  composite b-tree, because Orrery never performs global interval search.
* "Derived reads favour columnar/WCOJ engines" — SQLite 21× faster on Q1.
* "Missing rel-table indexes hurt global sweeps" — Ladybug was *faster* there.
* Unbounded cascade traversal as a graph-DB justification — see ADR-0020.

## Decision Outcome

**Deferred.** The measured evidence favours embedded relational. The project
owner has stated a continued preference for an **embedded graph database**, and
Ladybug is C++ (FFI from Rust), so the Rust-native graph landscape was never
surveyed.

**Mandate for the next session:** conduct grounded research on Rust-compatible
embedded graph datastores before accepting or rejecting this ADR.

**Only the paper screen blocks implementation** (ADR-0021). Benchmarking happens
through real repository implementations, which `orrery/SPEC-03` requires anyway.
Acceptance criteria below are pre-committed and must not be revised after
results are seen — if a criterion proves mis-specified, say so in writing.

### Candidate list

**Maintained in RESEARCH-0005, not here.** Twenty candidates supplied by the
project owner. An earlier revision of this ADR carried a nine-item list that I
had recalled from training data; it overlapped the owner's list only partially
and has been removed rather than merged, because a decision record duplicating a
research document is a decision record that will go stale (Rule 07).

Provenance matters to how the list is weighed: **the twenty entries are
owner-supplied, not model-recalled.** They are unverified in the sense that
existence, maintenance status, and category must be confirmed — not in the sense
that they may be fabrications.

### Acceptance criteria for a graph candidate

A candidate must either **beat the measured SQLite baseline** on Q1, Q2, and
Q7b, or lose by a margin justified by a compensating advantage that is named
in advance. Additionally:

1. Per-hop edge-property filtering in recursive patterns (Q1 requires it).
2. Native or first-class Rust bindings — not a C/C++ FFI wrapper, unless the
   wrapper is separately justified.
3. A concurrency model permitting the intended deployment. *(Ladybug permits
   one `READ_WRITE` process OR many `READ_ONLY`, never mixed — more
   restrictive than SQLite WAL. Verify each candidate.)*
4. ACID transactions.
5. Maintenance signal: contributors, release cadence, funding model.
6. *(Added 2026-08-02 — after Stage B, before any Stage-C work; recorded
   per Rule 01.2 rather than slipped in silently. Plan review CR-1.)*
   **External-writer cache-invalidation posture.** The engine's salsa
   mirror invalidates only through its own `Engine::apply`; a second
   writer process against a shared store silently desynchronises digests
   from detection (sweep reads the store directly, the derive chain does
   not). For each candidate, the Stage-C evaluation must state which is
   true of the intended deployment: (a) the store's concurrency model
   structurally guarantees a single writer process (e.g. exclusive-writer
   locking), (b) the store provides a change signal the mirror can
   consume, or (c) the deployment accepts a documented single-Engine
   constraint enforced outside the engine. "Unexamined" is not an option;
   this criterion adds a requirement and relaxes nothing pre-committed.

### Consequences

* All persistence sits behind a **repository trait** so this decision stays
  reversible at bounded cost. Non-negotiable regardless of outcome.
* Data ingest/egress via a portable format (Parquet/CSV) so data is never
  trapped in a proprietary file.
* If relational is chosen, ADR-0009's structural enforcement must be replaced
  by a tier-rule module with exhaustive tests.
