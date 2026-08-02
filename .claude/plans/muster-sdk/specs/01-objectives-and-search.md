<!-- Imported from design thread 2026-08-01. Fable: review and improve. -->

# SPEC 01 — Objectives and search

## Objective composition

```rust
pub trait Term { fn cost(&self, a: &Assignment) -> f64; fn weight(&self) -> f64; }
pub struct Objective { terms: Vec<Box<dyn Term>> }
```

## Required terms

| Term | Rationale |
|---|---|
| `ViolationCost` | weighted by severity; hard violations dominate |
| `ExpectedAttendeeTravel` | **replaces type-clustering** — see below |
| `StabilityFromReference` | churn against a reference assignment |
| `CapacityHeadroom` | penalise assignments near or over capacity |
| `RoomUtilisation` | penalise large rooms for small events |

### Why not type-clustering

"Cluster similar events in spatially clustered locations" optimises for an
attendee following exactly one track and actively harms cross-track attendees.
Since `attends` edges carry priority scores, expected attendee travel can be
optimised **directly** — same machinery, strictly better objective.

### Why stability matters more than optimality

The dominant real use is "here is last semester, three rooms are gone,
re-solve." An assignment 5% worse that changes 3 slots beats an optimal one
that changes 200. Stability is a first-class term, not a nicety.

## Search

Moves: `Relocate(event, room)`, `Swap(e1, e2)`, `Shift(event, delta)`.

Local search **must** be permitted to traverse infeasible regions — penalty
formulations depend on it. This is why ADR-0012 chose detection over
prevention; the two decisions are coupled.

Anytime: best-so-far returned on interrupt, with the objective breakdown.

## Testing

* Greedy matches brute-force optimum on fixed-start-time instances up to n=12
* Local search never returns worse than its seed
* Stability term measurably reduces churn on a seeded re-solve
* Objective is deterministic under a fixed RNG seed
