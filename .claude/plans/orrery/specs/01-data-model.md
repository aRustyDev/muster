<!-- Imported from design thread 2026-08-01. Fable: review and improve. -->

# SPEC 01 — Orrery data model

## Entities

```
Person   { id, name, ext: Map }
Group    { id, name, defaults, timezone?, ext: Map }
Event    { id, name, window: Interval, kind, timezone?, ext: Map }
Location { id, name, tier, portal, capacity?, geo?, ext: Map }
Violation{ id, kind, severity, subjects[], detected_at, resolved_at,
           acknowledged_by, waiver_reason }
```

Time (ADR-0024): `Interval` endpoints are UTC instants (`i64` µs) — the only
comparison representation anywhere in the engine. `timezone` fields are IANA
names, metadata for display and application-side recurrence expansion; the
algebra never reads them. The engine only ever sees concrete instances.

`tier ∈ {room, floor, structure, campus, region}` — containment position.
`portal ∈ {none, pedestrian, vehicle, rail, ...}` — routing role.
The two are **orthogonal** (ADR-0010).

## Relations

| Relation | From → To | Attributes |
|---|---|---|
| `attends` | Person → Event | `during`, `priority_group`, `priority_person?`, `priority_coord?`, `coord_binding`, `source`, `pinned` |
| `held` | Location → Event | `during` *(own)*, `posture`, `overflow_for: LocationId?`, `capacity_override?` |
| `member_of` | Person → Group | `during`, `role` |
| `subgroup_of` | Group → Group | `during` |
| `expects` | Group → Event | `obligation`, `default_priority`, `during`, `cascades`, `can_decline`, `set_by`, `set_at` |
| `within` | Location → Location | tier-ascending only |
| `traverse` | Location → Location | `mode`, `duration_typical`, `duration_peak?`, `peak_window?`, `distance`, `elevation_delta?`, `provenance`, `computed_at` |
| `transit` | Structure → Structure | `line`, `headway`, `ride_time`, `service_window` — **v2** |
| `anchors` | Person → Structure | `label`, `during`, `applies_when` |

## Effective priority

```
effective = if coord_binding && priority_coord.is_some() { priority_coord }
            else { priority_person ?? priority_coord ?? priority_group }
```

Computed in exactly one place. Divergence
`|priority_coord − priority_person|` is a first-class analytic.

## Derived edge identity

```
derived_id = blake3(person_id ‖ event_id ‖ expectation_id)
```

Stable across recomputation so violations, pins, and overrides can reference
derived edges (ADR-0016).

## Travel layers

**Layer 1** — sparse ground truth. Includes intermediate nodes hosting no
events. Directed, two rows per pair, written from a single function.

**Layer 2** — all-pairs closure over **event-bearing locations only**.
Batch-recomputed. Carries `computed_at` and `provenance`; detectors are more
conservative on `estimated` than `measured`.

## Constraints the store may not enforce

| Constraint | Enforcement |
|---|---|
| `within` is tier-ascending | schema (graph) or CHECK + tests (relational) |
| `traverse` connects siblings | schema or CHECK, with marked override edges permitted |
| `transit` connects Structures only | schema or CHECK |
| No overlapping `held` per location | **detector, not constraint** (ADR-0012) |
| No overlapping `attends` per person | **detector, not constraint** |
