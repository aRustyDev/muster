<!-- Imported from design thread 2026-08-01. Fable: review and improve. -->

# PRD — Muster-SDK

## Problem

Orrery says whether an arrangement is possible and how good it is. It does not
*find* good arrangements. Muster-SDK is the search and orchestration layer
between the engine and any application.

## Why it is a separate crate

Search strategy changes far more often than feasibility semantics, and any
future application needs the same strategies. Keeping search out of the engine
means the solver can evolve greedy → local search → CP-SAT without touching
the definition of "valid" (ADR-0013).

## Functional requirements

| ID | Requirement |
|---|---|
| FR-1 | Greedy room assignment with most-constrained-first ordering |
| FR-2 | Provably optimal greedy for the fixed-start-time case (interval graph colouring) |
| FR-3 | Local search (hill-climb / simulated annealing) over relocate, swap, shift moves |
| FR-4 | Anytime operation — return the best found so far on interrupt |
| FR-5 | Pluggable objective composed of weighted soft-constraint terms |
| FR-6 | Expected-attendee-travel objective, using `attends` priority (**not** type clustering) |
| FR-7 | Distance-from-reference stability term |
| FR-8 | `attendance_model` hook with a count-above-threshold default |
| FR-9 | Batch orchestration: sweeps, closure refresh, digest recomputation |
| FR-10 | Change-notification computation from persisted digests |
| FR-11 | Explain a proposed assignment — which constraints bind, what a move would cost |

## Out of scope

* Feasibility semantics and violation definitions → **Orrery**
* UI and delivery of notifications → **Muster**
* CP-SAT integration in v1 — evaluated but deferred; see QUESTION-0005
* Multi-tenant job scheduling

## User flows

**Flow A — suggest a room schedule.** Caller supplies events with duration
requirements, a room set, and a time window → SDK orders by most-constrained-
first → greedy seed → optional local-search refinement → returns assignment
plus violations plus objective breakdown.

**Flow B — re-solve with stability.** Caller supplies a reference assignment
and a change (rooms removed) → SDK seeds from the reference → local search with
a stability term → returns a minimal-churn assignment and a diff.

**Flow C — batch maintenance.** Scheduled run: refresh closure, recompute
digests, sweep violations, emit change set.

## Dependencies

* `orrery`
* `rand` for stochastic search
* Optional: OR-Tools CP-SAT (deferred; Rust bindings maturity unverified)

## Success criteria

* Greedy matches the provable optimum on fixed-start-time instances
* Local search improves on greedy on instances with heterogeneous rooms
* Re-solve changes < 10% of assignments for a single-room removal
