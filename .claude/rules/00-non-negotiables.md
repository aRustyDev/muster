# Rule 00 — Non-negotiables

Carried from the design thread. **Changing any of these requires a new ADR with
reasoning — not a commit message.**

1. **All persistence behind a repository trait.** No concrete datastore type
   appears in `orrery`'s public API. This is the entire reason ADR-0015 can stay
   open while implementation proceeds.
2. **All mutations through one command layer.** A `Command` enum, not a method
   set. Preserves the event-log upgrade (ADR-0016 D) as an insertion rather than
   a refactor.
3. **Every relation carries a validity window.** One interval predicate, many
   uses. A relation without `during` is a bug.
4. **Detection, not prevention, by default.** Planning requires transient
   invalid states; local search depends on traversing infeasible regions.
   `Prevent` is a per-kind policy running the same detector, never a schema
   constraint.
5. **`feasible(person, e1, e2)` signature lands now**, `person` ignored until
   mobility profiles arrive. Caches key on `(profile_id, e1, e2)`.
6. **Personal anchors never cross the coordinator boundary.** Verdicts only,
   enforced at the engine boundary, asserted by an automated test — not a review
   checklist.
7. **The solver lives in `muster-sdk`, never in `orrery`.**

## Rule 00b — MemoryRepo constraints are enforced, not documented

`MemoryRepo` must **panic or return an error** on: a second concurrent writer, a
read during an open write, or any traversal predicate referencing a prior hop.

A comment saying "don't do this" does not satisfy this rule. These assertions
exist to stop the repository trait absorbing assumptions that a restrictive
store cannot back, and they are worthless if they are not executable.
