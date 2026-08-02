<!-- Imported from design thread 2026-08-01. Fable: review and improve. -->

# SPEC 02 — Orrery functional requirements

## Detectors

| Kind | Rule | Partition |
|---|---|---|
| `time_conflict` | two `attends` for one person with overlapping `during` | person |
| `location_exclusivity` | two `held` into one location with overlapping `during`, excluding declared overflow | location |
| `containment_exclusivity` | a container and its content held for different events in an overlapping window | location |
| `impossible_travel` | consecutive events where `gap < travel_cost` | person |
| `capacity_exceeded` | count-above-threshold > `capacity_override ?? location.capacity` | event |
| `orphan_event` | event with no `held` edge in its window | event |
| `expired_membership_effect` | derived attendance whose source membership expired | person |

Each detector: pure function, own module, property-tested against a brute-force
oracle.

## Policy toggle

```rust
enum Policy { Off, Detect, Warn, Prevent }
fn policy(kind: ViolationKind) -> Policy;
```

`Prevent` runs the same detector inside the write transaction and aborts on a
non-empty result. One implementation, two call sites.

## Derived expansion (Q1)

1. Resolve the person's `member_of` edges valid at T.
2. Traverse `subgroup_of` with **per-hop** temporal validity — not whole-path
   post-filtering.
3. Collect `expects` edges valid at T, honouring `cascades`.
4. Union with explicit `attends`.
5. Apply the priority stack; attach provenance and `derived_id`.

Depth bound 5. Observed real depth 3–4.

## Impossible travel (Q4)

Consecutive pairs, not all ordered pairs. If A→C is infeasible but B occurs
between them and A→B→C is feasible, that is **not** a violation.

Cypher and SQL differ materially here: SQL expresses "consecutive" with a
window function (`LAG`); Cypher has none and requires an anti-join
(`NOT EXISTS` an event between). Record whichever the chosen store forces.

Signature is `feasible(person, e1, e2)` from day one, with `person` ignored
until mobility profiles land (ADR-0017).

## Analytics

* Engagement — priority-weighted attendance count per person per window
* Capacity pressure — signalled interest vs. allocated capacity per event
* Divergence — `|priority_coord − priority_person|` aggregated per group
* Co-attendance — **bounded 2-hop with a time window** (ADR-0020)
