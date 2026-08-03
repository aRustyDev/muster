# Testing policy: properties and regressions

*Decomposed 2026-08-03 from `plans/TESTING-STRATEGY.md` (QR-3/Stage E)
by CR-2 under ADR-0027 — full provenance note in
[coverage-taxonomy](../../strategies/testing/coverage-taxonomy.md).
Changes land as dated amendments.*

## Property tests: naming and case budgets (W-1 policy, W-12)

* **Naming**: new property tests take the `prop_` prefix, workspace-wide.
  The legacy sdk families (`optimality_`, `monotone_`) stay as named;
  `just test-prop` unions all three filters.
* **Case budget**: 48 cases is the deliberate default — wall-clock
  affordable on the gate path (the whole suite stays fast enough to run on
  every merge). Deep runs raise it via `PROPTEST_CASES` without code
  changes. The adversarial 1-eval search budget bound
  (`search_quality.rs`) is likewise deliberate test design — it pins
  behaviour under the *tightest* budget, not a typo to "fix".

## Regression policy (W-14)

Every refuted hypothesis and every fixed defect lands a **pinned test
named for the finding** (the H1 travel-shortcut practice, now written
down). proptest-regressions seed files are committed. A refutation
without a pinned test is an undone fix.
