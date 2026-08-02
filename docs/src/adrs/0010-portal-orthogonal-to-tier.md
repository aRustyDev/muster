<!-- Imported from design thread 2026-08-01. Review, do not assume correct. -->

# 10. Portal is orthogonal to tier; stations are Structures

* Status: accepted
* Date: 2026-08-01

## Context and Problem Statement

Should `*_station` be its own location tier, or a sibling of `campus`, or
something else?

## Decision Outcome

Collapse stations into `Structure`. A station conflates two orthogonal facts:
what it **is** (a containable structure that contains platforms) and what
**role** it plays (a network attachment point). Separate them:

```
location.tier   ∈ {room, floor, structure, campus, region}  -- containment
location.portal ∈ {none, pedestrian, vehicle, rail, ...}    -- routing role
```

### Consequences

* A parking lot is a Structure that is a vehicle portal; a station is a
  Structure that is a rail portal; a building entrance is a pedestrian portal.
* Adding ferry terminals later is a **value**, not a schema change.
* Portals are exactly where the strict sibling-tier rule (ADR-0009) is allowed
  to bend.
