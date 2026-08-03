# Summary

[Orrery / Muster](README.md)

<!-- generated below by docs/scripts/gen-summary.py — hand-edit above this line only -->

# Decision records

- [1. Project naming: Orrery, Muster-SDK, Muster](adrs/0001-project-naming.md)
- [2. Group is a first-class entity, not a string list on Person](adrs/0002-group-as-entity.md)
- [3. Group-universal events are expressed via an `expects` relation](adrs/0003-group-expectations-via-relation.md)
- [4. Derived attendance semantics, cached physically](adrs/0004-derived-not-materialized.md)
- [5. Priority is a precedence stack, not a single column](adrs/0005-priority-precedence-stack.md)
- [6. Travel is modelled in two layers](adrs/0006-travel-two-layers.md)
- [7. Travel relations split on continuous vs. scheduled, not on mode](adrs/0007-travel-split-continuous-scheduled.md)
- [8. Directed travel edges are stored as two rows](adrs/0008-directed-travel-two-rows.md)
- [9. Location is a tiered containment hierarchy](adrs/0009-location-tier-hierarchy.md)
- [10. Portal is orthogonal to tier; stations are Structures](adrs/0010-portal-orthogonal-to-tier.md)
- [11. `held` carries its own window; overflow is a location reference](adrs/0011-held-own-window-and-overflow.md)
- [12. Violations are first-class records; detection over prevention](adrs/0012-violations-first-class.md)
- [13. Optimization lives in Muster-SDK, not the Orrery lib](adrs/0013-solver-in-sdk-not-lib.md)
- [14. Personal origins are `anchors` relations, not a field](adrs/0014-person-anchors.md)
- [15. Datastore selection](adrs/0015-datastore-selection.md)
- [16. Change detection for derived state](adrs/0016-change-detection-strategy.md)
- [17. Mobility profiles deferred, but the signature lands now](adrs/0017-mobility-profile-deferred.md)
- [18. Capacity is per-location with per-event override; interest is not a forecast](adrs/0018-capacity-and-interest.md)
- [19. Three-crate Rust workspace](adrs/0019-three-crate-workspace.md)
- [20. Unbounded cascade analysis is withdrawn as a requirement](adrs/0020-cascade-analysis-withdrawn.md)
- [21. Datastore selection is a funnel, and only the paper screen blocks](adrs/0021-datastore-selection-funnel.md)
- [22. Dependency baseline](adrs/0022-dependency-baseline.md)
- [23. The repository trait is synchronous](adrs/0023-sync-repository-trait.md)
- [24. Time is UTC instants internally; zones are metadata; recurrence is not an engine concern](adrs/0024-time-representation.md)
- [25. Frontend structure: muster-server (axum) + muster-ui (dioxus) + muster-types](adrs/0025-frontend-structure.md)
- [26. Quality tooling baseline: cargo-deny, cargo-hack, and the profiling door](adrs/0026-quality-tooling-baseline.md)
- [27. Documentation architecture: docs/src taxonomy, ADR relocation, and context-loading strategy](adrs/0027-docs-and-context-architecture.md)
