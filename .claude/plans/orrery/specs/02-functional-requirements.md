<!-- Imported from design thread 2026-08-01. Fable: review and improve. -->

# SPEC 02 — Orrery functional requirements

## Detectors

| Kind | Rule | Partition |
|---|---|---|
| `time_conflict` | two `attends` for one person with overlapping `during` | person |
| `location_exclusivity` | two `held` for **different events** into one location with overlapping `during` | location |
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

> *Wording corrected 2026-08-02 (Phase 3): the draft said "excluding declared
> overflow", but the legitimate overflow pattern — one event held in a primary
> room and an overflow room — involves two locations and never produces a
> same-location pair, so no exclusion exists. An overflow-declared hold still
> physically occupies its room and conflicts with other events there. (The
> Phase-0 harness's Q5 exclusion clause is dead code on its own generated
> data.) Same-event pairs are excluded — an event does not conflict with
> itself.*

## Derived expansion (Q1)

1. Resolve the person's `member_of` edges valid at T — **the direct groups
   (depth 0) are part of the result set** *(made explicit 2026-08-02; the
   Phase-0 audit found the original benchmark implemented the domain-wrong
   `*1..5` semantics — see phases/00-grounding.md Refutation 2)*.
2. Traverse `subgroup_of` with **per-hop** temporal validity — not whole-path
   post-filtering.
3. Collect `expects` edges valid at T, honouring `cascades` (direct-group
   expectations apply unconditionally; strict-ancestor expectations only when
   they cascade).
4. Union with explicit `attends` (explicit shadows derived per event).
5. Apply the priority stack; attach provenance and `derived_id` (winner per
   event: highest `default_priority`, ties to the smaller group id).

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
