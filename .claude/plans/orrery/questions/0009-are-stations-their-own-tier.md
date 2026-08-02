<!-- Imported from design thread 2026-08-01. Fable: review and improve. -->

# QUESTION 0009 — Should `*_station` be its own location tier?

* Status: **ANSWERED**
* Raised: 2026-08-01

## Question

Proposed tiers were room / floor / structure / campus / *_station. Is station a
tier, a sibling of campus, or something else?

## Answer

Neither — it is a `Structure` with a `portal` value.

'Station' conflates two orthogonal facts: what it **is** (a containable structure
containing platforms) and what **role** it plays (a network attachment point).
Split them into `tier` and `portal`.

## Consequences / open threads

* A parking lot is a Structure that is a vehicle portal; a station is a Structure
  that is a rail portal. Ferry terminals later become a value, not a schema change.
* Portals are exactly where the strict sibling-tier rule is allowed to bend.
* See ADR-0010.
