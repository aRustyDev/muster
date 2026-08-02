# Rule 03 — Scope boundaries

| Belongs in | Never contains |
|---|---|
| `crates/orrery` | search, optimisation, UI, network I/O, concrete datastore types |
| `crates/muster-sdk` | feasibility semantics, violation definitions, UI, delivery |
| `crates/muster` | feasibility computation, search algorithms |

**Test:** if a change to `orrery` would be needed to try a different solver, the
boundary has been violated. `orrery` exposes `is_feasible` and `score`; how a
caller searches the space is not its concern.

**Second test:** if `orrery` gains a dependency that performs I/O beyond the
repository trait, stop and write an ADR.

## Incremental delivery

Prefer a narrow vertical slice over a broad horizontal layer. A single
person-event-conflict working end to end beats a complete data model with no
detectors. Ship working software early; refine second.
