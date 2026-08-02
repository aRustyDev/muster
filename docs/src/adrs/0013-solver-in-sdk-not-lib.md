<!-- Imported from design thread 2026-08-01. Review, do not assume correct. -->

# 13. Optimization lives in Muster-SDK, not the Orrery lib

* Status: accepted
* Date: 2026-08-01

## Context and Problem Statement

Room-assignment suggestion, clustering objectives, and schedule optimisation
need a home.

## Decision Outcome

Orrery's contract is the feasibility oracle and the objective evaluator:

```
is_feasible(assignment) -> Vec<Violation>
score(assignment)       -> f64
```

Search strategy is a **consumer** of that contract, and lives in Muster-SDK.

### Consequences

* Solver can evolve greedy → local search → CP-SAT without touching the engine.
* Muster and any future app share one definition of "valid".
* Relevant complexity result: **with fixed start times, room assignment is
  interval graph colouring** — polynomial, and greedy-by-left-endpoint is
  provably optimal because interval graphs are perfect. NP-hardness enters only
  with free start times, heterogeneous room compatibility, or clustering
  objectives.
* Two objective notes recorded here because they are easy to get wrong:
  * **Type-clustering is the wrong proxy.** "Cluster similar events in
    spatially clustered locations" optimises for single-track attendees and
    actively harms cross-track ones. With `attends` priority scores available,
    optimise **expected attendee travel** directly.
  * **Optimise for stability.** The dominant real use is "here is last
    semester, three rooms are gone, re-solve." A schedule 5% worse that changes
    3 assignments beats an optimal one that changes 200. Add a
    distance-from-reference term.
