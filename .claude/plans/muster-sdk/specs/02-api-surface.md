<!-- Written 2026-08-02 at Phase 5 entry — filling the "known thin spot"
     plans/README records for muster-sdk (the handoff shipped no data-model,
     API, or testing spec for this crate; deliberate gap, now closed). -->

# muster-sdk/SPEC-02 — API surface and data model

Search and orchestration over Orrery's contract. Consumes
`is_feasible`/`score`; never redefines them (Rule 03, ADR-0013). Sync,
inheriting ADR-0023 through `orrery`.

## Data model (the SDK's own types — thin by design)

The SDK owns *request/response* shapes; every domain fact stays `orrery`'s:

```rust
pub struct RoomRequest {          // an event needing a room, fixed start time
    pub event: EventId,
    pub window: Interval,         // orrery interval — no SDK time types
    pub expected_size: Option<u32>,
}
pub struct RoomOption {           // a candidate room
    pub location: LocationId,
    pub capacity: Option<u32>,
}
pub struct Placement { pub event: EventId, pub location: LocationId }

pub struct Suggestion {           // PRD Flow A's return value
    pub placements: Vec<Placement>,
    pub unassigned: Vec<EventId>, // never silently dropped
    pub violations: Vec<Violation>,   // from engine.is_feasible — verbatim
    pub breakdown: Breakdown,     // objective decomposition
}
```

## Modules

| Module | Surface (Phase 5 slice 1) | Later slices |
|---|---|---|
| `assign` | `greedy(requests, rooms) -> (Vec<Placement>, Vec<EventId>)` — left-endpoint order (provably optimal for fixed start times: interval graphs are perfect), best-fit-by-capacity among free rooms | most-constrained-first generalisation when room-compatibility constraints arrive |
| `objective` | `trait Term { name, weight, cost(&EvalCtx) -> f64 }`, `Objective::evaluate -> Breakdown` (rows + additive total); terms: `ViolationCost` (severity-weighted, same weights as `engine.score`), `RoomUtilisation` (waste + overfill ratios) | `StabilityFromReference`, `ExpectedAttendeeTravel`, `CapacityHeadroom` (muster-sdk/SPEC-01) |
| `suggest` | `suggest_room_schedule(&Engine<R>, &[RoomRequest], &[RoomOption], at, window) -> Suggestion` — greedy seed → oracle overlay evaluation → breakdown | optional local-search refinement pass |
| `search` | — | relocate/swap/shift moves, annealing, anytime interrupt |
| `batch` | — | closure refresh + digest recompute + sweep orchestration |
| `notify` | — | change-set computation from `refresh_digests` output |
| `explain` | — | which constraints bind, what a move would cost |

## Contracts

* **Violations flow one way**: the SDK never constructs a `ViolationKind`
  or re-implements a detector; everything in `Suggestion.violations` came
  from `engine.is_feasible` on a proposed overlay, verbatim.
* `Breakdown.total == Σ (weight × cost)` over its rows, exactly — the
  breakdown is an explanation, not a second objective.
* Determinism: same inputs → same suggestion (ties in greedy break on
  location id; no RNG in slice 1; stochastic search takes a caller seed).
* Errors: typed (`thiserror`), no `anyhow` (Rule 04).
