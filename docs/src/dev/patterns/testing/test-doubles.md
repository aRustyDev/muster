# Testing pattern: test doubles — placement and strategy (T-4, C8)

*Decomposed 2026-08-03 from `plans/TESTING-STRATEGY.md` (QR-3/Stage E)
by CR-2 under ADR-0027 — full provenance note in
[coverage-taxonomy](../../strategies/testing/coverage-taxonomy.md).
Changes land as dated amendments.*

* The restrictive **MemoryRepo fake is the deliberate double** for every
  repository consumer (ADR-0021; Rule 00b makes its constraints
  executable). Mock frameworks are rejected: a mock would let the
  `Repository` trait absorb assumptions the fake exists to block. The DI
  seams are generic parameters (`Engine<R>`, `MusterService<R>`), not
  injected trait objects.
* **DTO contract tests live in muster-server by design** (ADR-0025: the
  privacy boundary's single enforcement point). muster-types having no
  tests of its own is a decision, not an accident; its first own tests
  (serde roundtrip properties, muster/SPEC-03) arrive with its first
  input surface at Muster Alpha.
* **Time**: the engine reads no clock; the binary edge owns time —
  time-mocking gotchas are designed out, not mocked around.
* **UI REST-client double**: RR&P-8's call, at the Alpha pre-commitment.
