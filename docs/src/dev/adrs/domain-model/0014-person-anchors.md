<!-- Imported from design thread 2026-08-01. Review, do not assume correct. -->

# 14. Personal origins are `anchors` relations, not a field

* Status: accepted
* Date: 2026-08-01

## Context and Problem Statement

Feasibility for the first and last event of a day requires a starting location
("can you make the 8am from home?"). Proposal was a special `origin` location
node connected to each person.

## Decision Outcome

`person -[anchors {label, during, applies_when}]-> location`, not 1:1.

People have several anchors (home, office, partner's place) and which applies
depends on the day. "Home on Saturday, office on Tuesday" is the common case.

### Consequences

* **Privacy is a design-now concern, not a retrofit.** Personal anchors are
  home addresses. Coordinators need feasibility *verdicts*, not coordinates.
  Compute feasibility engine-side and return the verdict; never expose the
  anchor across the coordinator boundary.
* Anchors are `Structure`-tier, consistent with ADR-0009/0010.
