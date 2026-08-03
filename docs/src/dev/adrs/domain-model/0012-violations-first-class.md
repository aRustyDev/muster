<!-- Imported from design thread 2026-08-01. Review, do not assume correct. -->

# 12. Violations are first-class records; detection over prevention

* Status: accepted
* Date: 2026-08-01

## Context and Problem Statement

Should "no two events in one location at overlapping times" be a schema-level
constraint or an analysis function?

## Decision Drivers

Not preference — **planning requires transient invalid states.** Dragging an
event out of Room A leaves A double-booked for the duration of the drag. Local
search *needs* to traverse infeasible regions; penalty-based formulations
depend on it. Hard schema constraints make both impossible.

## Decision Outcome

Detection, with violations as first-class records rather than ad-hoc computed
results:

```text
violation(id, kind, severity, subjects[], detected_at, resolved_at,
          acknowledged_by, waiver_reason)
```

Policy is a per-kind toggle: `constraint_policy(kind) ∈ {off, detect, warn,
prevent}`. `prevent` runs the same detector inside the write transaction and
aborts on a non-empty result — **one implementation, two call sites.**

### Consequences

* Buys waivers ("yes, she is presenting remotely from the other room"),
  history, and "show unresolved" as an indexed query rather than a full
  recompute.
* This is the difference between a linter and an inbox. **The inbox is the
  product.**
* Requires stable identity for derived edges so violations can reference them
  (ADR-0016).
