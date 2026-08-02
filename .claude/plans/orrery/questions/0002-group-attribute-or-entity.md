<!-- Imported from design thread 2026-08-01. Fable: review and improve. -->

# QUESTION 0002 — Should groups be a string list on Person, or a first-class entity?

* Status: **ANSWERED**
* Raised: 2026-08-01

## Question

Original instinct: `person.groups: [String]`. Is an entity meaningfully more
powerful, or just heavier?

## Answer

**Entity**, and the deciding argument is temporality rather than any of the usual
normalisation reasons.

Group membership is unavoidably time-bounded. A string cannot carry a validity
window. Without one, a coordinator adding a group expectation in April either
backfills onto someone who left in February, or silently derives wrong
attendance. Every other relation in Orrery carries `during`; membership must too.

Secondary: membership needs a `role` (that is where the authorisation model for
coordinator override lives), groups need their own attributes, and hierarchy
nests.

## Consequences / open threads

* Established the invariant "every relation carries `during`".
* Added recursive traversal (`subgroup_of*`) to the query set — which became the
  stop-gate for datastore evaluation.
* See ADR-0002.
