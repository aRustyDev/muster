<!-- Imported from design thread 2026-08-01. Review, do not assume correct. -->

# 18. Capacity is per-location with per-event override; interest is not a forecast

* Status: accepted
* Date: 2026-08-01

## Context and Problem Statement

Capacity planning based on signalled interest was proposed.

## Decision Outcome

Add capacity, with two constraints.

**Capacity is per-location, but the constraint is per-(location, event).** A
room seats 60 but a workshop is capped at 30 — hence `held.capacity_override`.

**Signalled interest ≠ expected attendance.** The honest MVP counts `attends`
with `effective_priority > threshold`, flags over-capacity, and treats the
result as a **ranking signal, not a forecast**. The principled version
(`E[attendance] = Σ P(attend | priority)`) requires calibrating priority
against actual turnout, which is impossible before actuals exist.

### Consequences

* Ship the count; log actuals; calibrate later.
* Because the two target use cases have opposite rigidity — enrolment is
  near-deterministic, conference attendance is probabilistic — the engine
  exposes **primitives** (priority distribution, capacity, count-above-
  threshold) and an `attendance_model` **hook**, rather than shipping an
  estimator that is wrong for one of them.
* Unlocks a genuinely new analytic that justifies capacity existing at all:
  events whose signalled interest exceeds allocated capacity → recommend
  overflow allocation.
