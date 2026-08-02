<!-- Imported from design thread 2026-08-01. Review, do not assume correct. -->

# 2. Group is a first-class entity, not a string list on Person

* Status: accepted
* Date: 2026-08-01

## Context and Problem Statement

Group membership was initially imagined as an attribute holding a list of
string values on each `person`. Coordinators need to attach expectations to
groups and have them flow to members.

## Decision Drivers

* Every other relation in Orrery carries a validity window.
* Coordinator override requires an authorization model.
* Organisational structure nests.

## Considered Options

* `person.groups: [String]`
* `person -[member_of {during, role}]-> group` plus `group -[subgroup_of]-> group`

## Decision Outcome

Group is a first-class entity with temporal, role-bearing membership edges.

The deciding argument is **temporality**, and it is structurally fatal to the
string approach: membership is time-bounded (a student is in a section for a
semester; an employee changes teams mid-quarter). A string cannot carry a
validity window. Without one, a coordinator adding a group expectation in April
either backfills onto someone who left in February, or silently derives the
wrong attendance.

Secondary but real: membership needs a `role` (the authorization model for
coordinator override lives there); groups need their own attributes; hierarchy
nests.

### Consequences

* Establishes the architectural invariant: **every relation carries `during`,
  and every conflict check is the same interval-overlap predicate.**
* Adds recursive traversal to the query set (`subgroup_of*`).
* Membership changes now have derived-state blast radius (see ADR-0004, 0016).
