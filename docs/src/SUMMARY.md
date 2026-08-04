# Summary

[Orrery / Muster](README.md)

<!-- generated below by docs/scripts/gen-summary.py — hand-edit above this line only -->

# Decision records

- [1. Project naming: Orrery, Muster-SDK, Muster](dev/adrs/project/0001-project-naming.md)
- [2. Group is a first-class entity, not a string list on Person](dev/adrs/domain-model/0002-group-as-entity.md)
- [3. Group-universal events are expressed via an `expects` relation](dev/adrs/domain-model/0003-group-expectations-via-relation.md)
- [4. Derived attendance semantics, cached physically](dev/adrs/domain-model/0004-derived-not-materialized.md)
- [5. Priority is a precedence stack, not a single column](dev/adrs/domain-model/0005-priority-precedence-stack.md)
- [6. Travel is modelled in two layers](dev/adrs/travel/0006-travel-two-layers.md)
- [7. Travel relations split on continuous vs. scheduled, not on mode](dev/adrs/travel/0007-travel-split-continuous-scheduled.md)
- [8. Directed travel edges are stored as two rows](dev/adrs/travel/0008-directed-travel-two-rows.md)
- [9. Location is a tiered containment hierarchy](dev/adrs/domain-model/0009-location-tier-hierarchy.md)
- [10. Portal is orthogonal to tier; stations are Structures](dev/adrs/domain-model/0010-portal-orthogonal-to-tier.md)
- [11. `held` carries its own window; overflow is a location reference](dev/adrs/domain-model/0011-held-own-window-and-overflow.md)
- [12. Violations are first-class records; detection over prevention](dev/adrs/domain-model/0012-violations-first-class.md)
- [13. Optimization lives in Muster-SDK, not the Orrery lib](dev/adrs/architecture/0013-solver-in-sdk-not-lib.md)
- [14. Personal origins are `anchors` relations, not a field](dev/adrs/domain-model/0014-person-anchors.md)
- [15. Datastore selection](dev/adrs/datastore/0015-datastore-selection.md)
- [16. Change detection for derived state](dev/adrs/architecture/0016-change-detection-strategy.md)
- [17. Mobility profiles deferred, but the signature lands now](dev/adrs/domain-model/0017-mobility-profile-deferred.md)
- [18. Capacity is per-location with per-event override; interest is not a forecast](dev/adrs/domain-model/0018-capacity-and-interest.md)
- [19. Three-crate Rust workspace](dev/adrs/architecture/0019-three-crate-workspace.md)
- [20. Unbounded cascade analysis is withdrawn as a requirement](dev/adrs/datastore/0020-cascade-analysis-withdrawn.md)
- [21. Datastore selection is a funnel, and only the paper screen blocks](dev/adrs/datastore/0021-datastore-selection-funnel.md)
- [22. Dependency baseline](dev/adrs/dependencies/0022-dependency-baseline.md)
- [23. The repository trait is synchronous](dev/adrs/architecture/0023-sync-repository-trait.md)
- [24. Time is UTC instants internally; zones are metadata; recurrence is not an engine concern](dev/adrs/domain-model/0024-time-representation.md)
- [25. Frontend structure: muster-server (axum) + muster-ui (dioxus) + muster-types](dev/adrs/architecture/0025-frontend-structure.md)
- [26. Quality tooling baseline: cargo-deny, cargo-hack, and the profiling door](dev/adrs/testing/0026-quality-tooling-baseline.md)
- [27. Documentation architecture: docs/src taxonomy, ADR relocation, and context-loading strategy](dev/adrs/project/0027-docs-and-context-architecture.md)

# Strategies

- [Testing strategy: the coverage taxonomy](dev/strategies/testing/coverage-taxonomy.md)
- [Testing strategy: tool roster and open decisions](dev/strategies/testing/tool-roster.md)

# Policies

- [Benchmarking policy: measurement variance (W-2)](dev/policies/benchmarking/measurement-variance.md)
- [Testing policy: properties and regressions](dev/policies/testing/property-and-regression.md)
- [Testing policy: standing gates](dev/policies/testing/standing-policies.md)

# Patterns

- [Testing pattern: test doubles — placement and strategy (T-4, C8)](dev/patterns/testing/test-doubles.md)

# Reference

- [Glossary](dev/glossary.md)
