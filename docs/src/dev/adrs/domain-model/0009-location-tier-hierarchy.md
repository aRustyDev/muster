<!-- Imported from design thread 2026-08-01. Review, do not assume correct. -->

# 9. Location is a tiered containment hierarchy

* Status: accepted
* Date: 2026-08-01

## Context and Problem Statement

Locations need types to make `bldg↔bldg` vs `room↔room` travel coherent, to
support overflow semantics, and to enable capacity planning.

## Decision Outcome

Not a type tag — a **containment hierarchy**:
`Room ⊂ Floor ⊂ Structure ⊂ Campus ⊂ Region`, related by `within`
(tier-ascending only). `Floor` and `Region` are optional tiers — `within` may
skip a tier where one does not apply (a single-storey building has no floor).

**Canonical tier list is five, defined here.** The evidence schema in
`evidence/orrery_schema.py` implements only Room/Structure/Campus, deliberately,
to keep the benchmark minimal. That is a **subset, not a contradiction** — treat
this ADR as authoritative.

A type tag says Room B is a room. It does not say Room B is *inside Building
Foo*, and that is the fact that does the work:

* **Travel decomposition.** `cost(roomA, roomB) = cost(roomA→exitA) +
  cost(bldgA→bldgB) + cost(entranceB→roomB)`. Collapses n_rooms² into
  n_buildings² plus small per-building terms.
* **Capacity roll-up.** Building capacity constrains the sum of its rooms, and
  is separately bounded by egress — so it needs its own value, not a computed
  sum.
* **Exclusivity correctness.** Without containment, "Building Foo" and
  "Building Foo Room A" can be booked simultaneously for different events.

**Sibling-only rule:** direct `traverse` edges require a common parent.

### Consequences

* **Verified empirically (5/5).** Modelling tiers as separate node tables in a
  property-graph store makes illegal edges *unrepresentable*: `transit`
  Room→Room rejected, `traverse` Room→Structure rejected, inverted `within`
  rejected. See RESEARCH-0002.
* The sibling rule is an **approximation** — it assumes cheapest paths route
  through each container's representative point. Skybridges violate it.
  Explicit override edges, marked as such, are permitted; the rule is a strong
  default for *generated* topology, not an invariant.
* Cross-tier edges are legitimate **at portals** (parking lot → building lobby).
* If the datastore is relational, this enforcement must be reproduced with a
  `tier` discriminator plus CHECK constraints — less elegant, write-time rather
  than type-level. Keep it in one module with exhaustive tests.
