<!-- Written 2026-08-02 at Phase 5 entry — filling the "known thin spot"
     plans/README records for muster-sdk. -->

# muster-sdk/SPEC-03 — testing criteria

## Property tests (the load-bearing ones)

* **Optimality (`optimality_` prefix — `just muster_sdk::test-optimality`)**:
  on fixed-start-time instances (n ≤ 12 events, k ≤ 3 rooms), greedy finds
  a complete zero-exclusivity assignment **iff** brute-force enumeration
  finds one, and **iff** max instantaneous overlap ≤ k (two independent
  oracles — interval graphs are perfect, so all three must agree).
* **Monotone (`monotone_` prefix — Alpha)**: local search never returns a
  worse-scoring assignment than its seed.
* **Stability (Beta)**: re-solve after removing one room changes < 10% of
  assignments (ROADMAP gate).
* **Breakdown additivity**: `total == Σ weight×cost` on arbitrary term sets;
  evaluation is deterministic.

## Boundary tests

* **Scope guard**: `just muster_sdk::check-scope` fails the build if a
  UI/server dependency enters the tree (QUESTION-0015).
* **No feasibility semantics**: SDK sources construct no `ViolationKind`
  and contain no interval-overlap re-implementation — violations arrive
  from the engine only. (Checked by source grep at phase close + review.)
* **Determinism**: fixed inputs (and, later, fixed RNG seed) → identical
  `Suggestion`, byte-for-byte on serialised output.

## Integration

* `suggest_room_schedule` against `Engine<MemoryRepo>`: placements land,
  an externally-seeded conflicting hold surfaces as a `ViolationCost` row
  and in `Suggestion.violations`; `unassigned` is populated (never
  silently dropped) when rooms run out.

## Release gates (ROADMAP)

| Stage | Gate |
|---|---|
| PoC | greedy matches brute-force optimum, n ≤ 12 |
| Prototype | assignment + breakdown returned |
| Alpha | local search improves on greedy for heterogeneous rooms |
| Beta | re-solve changes < 10% for one room removal |
| RC | CP-SAT integrated **or** rejection documented |
