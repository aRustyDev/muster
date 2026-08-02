<!-- Imported from design thread 2026-08-01. Fable: review and improve. -->

# QUESTION 0012 — What Rust-compatible embedded graph datastores exist, and do any beat the baseline?

* Status: **OPEN — assigned to Fable**
* Raised: 2026-08-01

## Question

Ladybug is C++ (FFI from Rust). The Rust-native embedded graph landscape was never
surveyed. The project owner prefers an embedded graph store; the measured evidence
currently favours embedded relational.

## Answer

**Unanswered. This is the primary research mandate for the next session.**

**Candidate list lives in RESEARCH-0005** — twenty entries, owner-supplied.
Confirm existence, maturity, licence, and current status for each; the list is
unverified in that sense, not in the sense that entries may be invented.

An earlier revision of this document listed nine model-recalled candidates. That
list has been removed in favour of the owner's, which is authoritative.

## Consequences / open threads

Acceptance criteria — a candidate must beat the measured SQLite baseline on Q1,
Q2, and Q7b, or lose by a margin justified by a compensating advantage named in
advance. Additionally:

1. Per-hop edge-property filtering in recursive patterns (Q1 requires it)
2. Native or first-class Rust bindings, not a C/C++ FFI wrapper unless separately
   justified
3. A concurrency model permitting the intended deployment — Ladybug permits one
   `READ_WRITE` process **or** many `READ_ONLY`, never mixed
4. ACID transactions
5. Maintenance signal: contributors, release cadence, funding model

Reproduce with the harness in `evidence/`. See ADR-0015.
