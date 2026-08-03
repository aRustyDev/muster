# Testing strategy: the coverage taxonomy

*Decomposed 2026-08-03 from `plans/TESTING-STRATEGY.md` (created by
QR-3/Stage E of the quality-strategy review; method, evidence, and
synthesis: `plans/quality-review/00-review-plan.md`, `01-gap-matrix.md`,
`02-additions-and-order.md`) by CR-2 under ADR-0027. This is the single
cross-crate home (Rule 07/Rule 10): product testing specs carry
per-crate criteria and link here; nothing stated here is restated
there. Changes land as dated amendments. Item IDs (W-n/O-n/RR&P-n)
resolve in `02-additions-and-order.md`.*

Quality coverage is tracked against the review's dimension matrix:
C1–C19 (correctness), P1–P13 (performance), S1–S5 (safety & robustness),
I1–I5 (infrastructure & meta) — 42 dimensions after the two review-added
rows, whose definitions live here:

* **S5 Privacy testing** — *executable assertions that privacy invariants
  hold on every egress channel: wire payloads, logs, spans, errors,
  exports, session state.* The corpus's hardest gate class (Rule 00.6,
  Rule 09) gets its own row so written-but-unasserted promises are visible
  as such. Cell dispositions live in the gap matrix.
* **I5 Ops-validation / DR** — *executable validation of operational
  procedures: backup/restore, deterministic rebuild, config sanity,
  migration drills.* Exists so the RC gates ("backup/restore",
  "deterministic rebuild verified") have a dimension to live in; all cells
  stay gap/N/A until the RC pre-commitment defines them.

Related: [tool roster](tool-roster.md) ·
[measurement variance](../../policies/benchmarking/measurement-variance.md) ·
[properties and regressions](../../policies/testing/property-and-regression.md) ·
[standing gates](../../policies/testing/standing-policies.md) ·
[test doubles](../../patterns/testing/test-doubles.md)
