<!-- Imported from design thread 2026-08-01. Fable: review and improve. -->

# QUESTION 0005 — How should room-schedule suggestion be approached?

* Status: **ANSWERED**
* Raised: 2026-08-01

## Question

Given events with only duration requirements, plus optional grouping to cluster
similar events spatially, suggest an assignment of rooms within a time window.

## Answer

Tiered, driven by a complexity result that determines the architecture:

**With fixed start times and room assignment only, this is interval graph
colouring** — polynomial, and greedy-by-left-endpoint is provably optimal because
interval graphs are perfect. NP-hardness enters only with free start times,
heterogeneous room compatibility, or clustering objectives.

Tier 0 greedy (most-constrained-first) → Tier 1 greedy seed + local search →
Tier 2 CP-SAT if instances grow.

## Consequences / open threads

Three points that matter more than solver choice:

* **Keep the solver out of Orrery.** The engine's contract is the feasibility
  oracle and objective evaluator; search is a consumer. Lets the solver evolve
  without touching the definition of 'valid'.
* **Type-clustering is the wrong objective.** It optimises for single-track
  attendees and harms cross-track ones. With `attends` priorities available,
  optimise expected attendee travel directly.
* **Optimise for stability.** The dominant real use is re-solve after a small
  change; 5% worse with 3 changes beats optimal with 200.

CP-SAT deferred — OR-Tools Rust bindings maturity unverified.
See ADR-0013.
