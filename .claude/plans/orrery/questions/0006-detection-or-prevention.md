<!-- Imported from design thread 2026-08-01. Fable: review and improve. -->

# QUESTION 0006 — Should exclusivity be enforced by schema, or detected by analysis?

* Status: **ANSWERED**
* Raised: 2026-08-01

## Question

Enforce 'one event per location at a time' as a database constraint, or detect
violations after the fact?

## Answer

**Detection**, with a per-kind policy toggle to `prevent` where wanted.

This is not a preference. Planning **requires transient invalid states**: dragging
an event out of a room leaves it double-booked mid-drag, and local search must
traverse infeasible regions because penalty formulations depend on it. Hard schema
constraints make both impossible.

Violations are first-class records rather than ad-hoc computed results, which buys
waivers, history, and 'show unresolved' as an indexed query.

## Consequences / open threads

* Neutralised what had been the strongest argument for PostgreSQL — declarative
  `EXCLUDE USING gist` range-exclusion constraints. That option was live until
  this question was answered.
* Coupled to the solver design (SPEC muster-sdk/01).
* See ADR-0012.
