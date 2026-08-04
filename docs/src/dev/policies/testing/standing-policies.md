# Testing policy: standing gates

*Decomposed 2026-08-03 from `plans/TESTING-STRATEGY.md` (QR-3/Stage E)
by CR-2 under ADR-0027 — full provenance note in
[coverage-taxonomy](../../strategies/testing/coverage-taxonomy.md).
Changes land as dated amendments.*

* **Doctests**: rustdoc examples are tests; nextest does not run them.
  `just test-doc` is the door and is part of `just ci` (W-3). Orrery's
  public API gets rustdoc examples at Beta freeze (O-4).
* **Safe Rust**: `unsafe_code = "forbid"` workspace-wide via
  `[workspace.lints.rust]` (W-13). miri/sanitizer runs are CI-Linux-leg
  items inside RR&P-1 — the no-rustup host cannot run them natively.
* **Data egress** (S5/M-8): Parquet/CSV export excludes `anchors` by
  default (Rule 09); the RC pre-commitment owes the executable test.
* **Funnel discipline** (D4): no benchmark touches a datastore candidate
  before the ADR-0021 stage permits one; the RR&P-2 bench skeleton is
  MemoryRepo-only until Phase 7.
* **Gate honesty**: recipes that cannot run yet **fail loudly with a
  pointer** (`bench`, `differential`) rather than passing over nothing;
  gate output records which arm ran where fallbacks exist (check-seam).
