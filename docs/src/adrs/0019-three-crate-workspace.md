<!-- Imported from design thread 2026-08-01. Review, do not assume correct. -->

# 19. Three-crate Rust workspace

* Status: accepted
* Date: 2026-08-01

## Context and Problem Statement

Engine, solver, and application have different change rates, dependency
weights, and audiences.

## Decision Outcome

```
crates/orrery      -- entities, interval algebra, violations, travel,
                      derived expansion, feasibility oracle, scoring,
                      persistence trait
crates/muster-sdk  -- solver strategies, objectives, batch orchestration,
                      attendance-model hooks, notification computation
crates/muster      -- application: UI, coordinator workflows, auth
```

### Consequences

* `orrery` takes no dependency on any solver.
* `orrery` exposes persistence as a trait; no concrete datastore in its public
  API (ADR-0015 reversibility).
* `muster-sdk` depends on `orrery`; `muster` depends on both.
* Any future application reuses `orrery` + `muster-sdk` unchanged.
