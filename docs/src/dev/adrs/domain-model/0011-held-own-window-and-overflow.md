<!-- Imported from design thread 2026-08-01. Review, do not assume correct. -->

# 11. `held` carries its own window; overflow is a location reference

* Status: accepted
* Date: 2026-08-01

## Context and Problem Statement

The original spec gave `held` an `is_overflow` boolean and no time window of
its own, inheriting the event's. The requirement "Room B is overflow for Room A
during event Bar, but before and after that event it is for its own event"
cannot be expressed that way.

## Decision Outcome

* **`held.during` is independent of the event's window.** Room B can be held
  for Bar from 14:00–15:30 while Bar runs 13:00–17:00, leaving B free for its
  own event at 16:00.
* **`held.overflow_for: location_id?`** replaces the boolean, so spillover
  chains ("C overflows B overflows A") are expressible. With three rooms and an
  ordering, a boolean loses the structure.

### Consequences

* Exclusivity becomes: *no two `held` edges into the same location with
  overlapping `held.during`* — structurally identical to person conflict
  detection. Same predicate, same index, same code path.
* That uniformity is worth more than it appears: one piece of interval
  machinery serves `attends`, `held`, and `member_of`.
