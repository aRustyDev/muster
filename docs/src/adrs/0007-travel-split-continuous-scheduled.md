<!-- Imported from design thread 2026-08-01. Review, do not assume correct. -->

# 7. Travel relations split on continuous vs. scheduled, not on mode

* Status: accepted
* Date: 2026-08-01

## Context and Problem Statement

Proposal was to split `travel` into `travel_by_walk`, `travel_by_train`, etc.,
partly to model shuttle service hours and partly to constrain which node types
a mode may connect.

## Decision Drivers

* Walk/drive/cycle share a schema and an algorithm; transit shares neither.
* Type constraints are better served structurally (ADR-0009).

## Decision Outcome

Two relations, mode as an attribute:

* **`traverse`** — continuous. Cost is a scalar duration; depart whenever.
  `arrive = depart + duration`.
* **`transit`** — scheduled. Cost is a *function of departure time*.
  `arrive = next_departure(t) + ride_time`. Service calendars, headways.

### Consequences

* Avoids N near-duplicate tables and an N-way union on every path query.
* **Scheduled edges break the Layer-2 scalar cache.** `travel(a,b).duration`
  becomes a curve over departure time, requiring time-bucketed profiles or
  on-demand fallback. This is an architectural change, not just a new table.
* `transit` is therefore deliberately deferred to v2.
* "No train between rooms" is enforced by ADR-0009, not by table proliferation.
