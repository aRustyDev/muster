<!-- Imported from design thread 2026-08-01. Review, do not assume correct. -->

# 17. Mobility profiles deferred, but the signature lands now

* Status: accepted
* Date: 2026-08-01

## Context and Problem Statement

Personal walking speed, mobility constraints, transfer buffers, and elevation
affect travel feasibility. Deferred as a v2 goal.

## Decision Outcome

Defer the feature. **Land the signature now.**

`elevation_delta` is a pure attribute on the travel edge. `mobility_profile` is
per-person-per-mode — relationally a small side table, conceptually an
attribute. Neither is a new entity.

The exception is not schema, it is **signature**. Mobility changes the check
from `feasible(e1, e2)` to `feasible(person, e1, e2)`. That invalidates any
cache keyed on event pairs and makes "impossible travel" a person-relative
property rather than a global one — which is arguably the correct semantics all
along.

**Therefore:** define the function taking `person` today and ignore the
argument. Key feasibility caches on `(profile_id, e1, e2)` where everyone
currently shares a `default` profile.

### Consequences

* Costs nothing now; makes the later change a pure fill-in.
* Avoids a cache-invalidation refactor across the engine.
