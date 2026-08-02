<!-- Imported from design thread 2026-08-01. Fable: review and improve. -->

# PRD — Orrery (engine library)

## Problem

Planning tools track *what* is scheduled but not whether the result is
physically possible. Nobody answers: can this person actually be in both
places? Is this room double-booked? Is anyone in this cohort disengaged?

Orrery is a **spatiotemporal feasibility engine**: given a proposed assignment
of people to events at locations over time, it returns the ways that assignment
is impossible, and a score for how good it is.

## Why now

Two concrete use cases with opposite characteristics stress the same core:

* **Academic advising** — rigid student schedules, near-deterministic
  enrolment, hard prerequisite and room constraints.
* **Conference planning** — soft attendance, probabilistic turnout, heavy
  travel-feasibility pressure between sessions.

A single engine serving both must expose primitives and hooks rather than a
built-in model tuned to one.

## Functional requirements

| ID | Requirement |
|---|---|
| FR-1 | Model `Person`, `Group`, `Event`, and tiered `Location` entities |
| FR-2 | Model `attends`, `held`, `member_of`, `subgroup_of`, `expects`, `within`, `traverse`, `transit`, `anchors` relations, each carrying a validity window |
| FR-3 | Detect per-person time conflicts via interval overlap |
| FR-4 | Detect location exclusivity violations, containment-aware and overflow-aware |
| FR-5 | Detect impossible travel between consecutive events |
| FR-6 | Derive group-expected attendance through a nested group hierarchy with per-hop temporal validity |
| FR-7 | Compute an effective priority from the group/person/coordinator precedence stack |
| FR-8 | Emit `Violation` records with kind, severity, subjects, and lifecycle |
| FR-9 | Support per-kind constraint policy: `off` / `detect` / `warn` / `prevent` |
| FR-10 | Maintain the Layer-2 travel closure from the Layer-1 network |
| FR-11 | Expose `is_feasible(assignment) -> Vec<Violation>` and `score(assignment) -> f64` |
| FR-12 | Expose engagement analytics: priority distribution, count-above-threshold, capacity pressure |
| FR-13 | Provide bounded 2-hop co-attendance queries with a time window |
| FR-14 | Persist through a repository trait with no concrete datastore in the public API |
| FR-15 | Provide deterministic identity for derived edges |
| FR-16 | Route all mutations through a single command layer |

## Out of scope

* Search, optimisation, and room-assignment suggestion → **Muster-SDK**
* User interface, authentication, notification delivery → **Muster**
* Attendance forecasting — the engine ships a *hook*, not a model
* Real-time routing — travel data is imported, not computed from live traffic
* Calendar sync, SSO, tenancy
* `transit` (scheduled travel) — v2; see ADR-0007
* Mobility profiles — v2 signature only; see ADR-0017

## User flows

Orrery has no human users. Its consumers are Muster-SDK and applications.

**Flow A — feasibility check.** Caller supplies an assignment delta → engine
recomputes affected derived state → runs enabled detectors → returns
violations. Must complete inside an interactive budget for single-person deltas.

**Flow B — batch sweep.** Caller requests a full-population sweep → engine
scans all persons/rooms → emits/updates violation records → returns a summary.
Batch latency budget.

**Flow C — travel closure refresh.** External process updates Layer-1 →
engine recomputes the closure over event-bearing locations → writes Layer 2.

**Flow D — derived expansion.** Caller asks for a person's effective schedule
at time T → engine walks group hierarchy with per-hop temporal filters, unions
explicit attendance, applies the priority stack, returns edges with provenance.

## Dependencies

* Rust stable
* A datastore — **ADR-0015 is open**
* `salsa` for incremental derivation (ADR-0016)
* `petgraph` for Layer-1 pathfinding
* `blake3` for derived-edge identity

## Success criteria

* All seven canonical queries implemented and passing property tests
* Per-person interactive queries within budget at 10⁶ `attends` edges
* Datastore swappable behind the repository trait, demonstrated with two
  implementations
