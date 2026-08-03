# crates/muster-sdk — search and orchestration

Consumes `orrery`'s contract (`is_feasible` / `score`); never redefines it.
Owns (since Phase 5): greedy room assignment (provably optimal on
fixed-start-time instances — interval graphs are perfect), objective
composition with an additive breakdown, local search over a seed
assignment, batch orchestration (`sdk.batch`), change-set computation.

*(Rewritten 2026-08-03, quality review F-13 — this file said "currently a
compiling stub" and "once the solver lands"; the solver landed in
Phase 5.)*

**Must never contain** (Rule 03): feasibility semantics, violation
definitions, UI, delivery. Two executable gates:

* `just muster_sdk::check-scope` — fails the build if a UI/server
  dependency enters the tree (14 name-stems, word-boundary matched;
  rewritten 2026-08-03 after the original was proven inert — QF-R1).
* `just muster_sdk::check-oneway` — fails the build if `src/` constructs
  a `ViolationKind`, calls a detector, or re-implements interval overlap.
  Scoped to `src/` by design: `tests/` holds the brute-force oracle,
  which SPEC-03 requires to re-implement overlap independently.

Testing: `just muster_sdk::test`; `test-optimality` (`optimality_`
prefix: greedy ⇔ brute force ⇔ max-overlap ≤ k, n ≤ 12);
`test-monotone` (`monotone_`: local search never worse than its seed).
New property tests take the `prop_` prefix (naming policy:
`docs/src/dev/policies/testing/property-and-regression.md`).
Criteria: `.claude/plans/muster-sdk/specs/03-testing-criteria.md`.
