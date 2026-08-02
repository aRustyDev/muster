<!-- Imported from design thread 2026-08-01. Fable: review and improve. -->

# SPEC 00 — Orrery overview

## Contract

```rust
pub trait FeasibilityOracle {
    fn is_feasible(&self, a: &Assignment) -> Vec<Violation>;
    fn score(&self, a: &Assignment) -> f64;
}
```

Everything else consumes this. Orrery does not schedule; it decides whether a
schedule is possible and how good it is.

## Architectural invariants

1. **Every relation carries a `during` window.** Every conflict check is the
   same interval-overlap predicate. One piece of interval machinery, N uses.
2. **Every query is entity-partitioned before its interval predicate applies.**
   Partition by person, or by room. This is the single most important fact for
   datastore and index selection.
3. **Derived semantics, cached physically.** Group expectations are computed,
   not written at authoring time.
4. **All persistence behind a repository trait.** No concrete datastore in the
   public API.
5. **All mutations through one command layer.** Preserves the event-log upgrade
   path at near-zero present cost.

## Module map

| Module | Responsibility |
|---|---|
| `model` | entities, relations, newtypes, validity windows |
| `interval` | Allen relations, overlap, containment, merge |
| `derive` | group expansion, priority stack, provenance, salsa queries |
| `detect` | violation detectors, one per kind, plus the policy toggle |
| `travel` | Layer-1 graph, closure computation, feasibility check |
| `analytics` | engagement, capacity pressure, divergence, 2-hop co-attendance |
| `repo` | persistence trait and implementations |
| `command` | the single mutation chokepoint |
