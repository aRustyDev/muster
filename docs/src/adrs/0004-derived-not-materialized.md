<!-- Imported from design thread 2026-08-01. Review, do not assume correct. -->

# 4. Derived attendance semantics, cached physically

* Status: accepted
* Date: 2026-08-01

## Context and Problem Statement

When a group expects an event, is an `attends` edge written for each member
(materialized), or computed at read time (derived)?

## Decision Drivers

* Membership changes must not leave stale attendance.
* User overrides must survive recomputation.
* Analytic queries should stay simple.

## Considered Options

| | Materialize | Derive |
|---|---|---|
| Read path | uniform; conflict detection never sees groups | every read unions explicit + inherited |
| Membership change | requires reconciliation | propagates instantly |
| User override | natural — edit the row | needs a separate exception table |
| Failure mode | drift | read complexity, override ambiguity |

## Decision Outcome

**Derived semantics, cached physically.** Expectations are not written as rows
at authoring time; they are computed, then memoized with explicit dependency
tracking.

Provenance is carried regardless:

```
attends.source ∈ {self, group:<id>, coordinator:<id>}
attends.pinned bool   -- true when the user edits it
```

Reconciliation touches only unpinned edges whose `source` matches.

### Consequences

* No drift; membership changes propagate.
* **Change detection is lost and must be reconstructed** — see ADR-0016.
* A single `member_of` write has unbounded, invisible blast radius on derived
  state. This is the central engineering problem the derived model creates.
* Measured: the derived expansion (Q1) costs 0.2 ms at 1M edges in SQLite,
  4.3 ms in Ladybug. Recomputation cost is not the constraint.
