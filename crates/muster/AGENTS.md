# crates/muster — the application

Service layer + CLI demo over `Engine<MemoryRepo>` and muster-sdk: member
flow (browse / select / priority / my-schedule with provenance / deselect)
is complete (Prototype, Phase 6 slice 2); coordinator flow is the Alpha
slice. Delivery lives in the sibling crates (ADR-0025): `muster-server`
(axum REST, the privacy boundary's enforcement point), `muster-ui`
(dioxus), `muster-types` (wire DTOs).

*(Rewritten 2026-08-03, quality review F-13 — this file said "currently a
compiling stub binary", claimed this crate installs the tracing
subscriber, and placed the automated privacy tests in orrery alone. All
three were stale.)*

**Must never contain** (Rule 03): feasibility computation or search
algorithms — those are `orrery` and `muster-sdk` respectively.

Conventions that differ from the libraries:

* `anyhow` at the binary edge is correct here (Rule 04) — service
  functions still return typed errors.
* Observability: **`muster-server` installs the subscriber** and owns the
  `figment` exporter knob (Rule 05, corrected 2026-08-03). This crate has
  no tracing dependency today; `just muster_server::run-dev` is the live
  dev knob.
* Privacy boundary: coordinators receive feasibility verdicts, never
  anchor coordinates (Rule 09). The automated tests live in **orrery**
  (engine boundary, `privacy_` family) and **muster-server**
  (`privacy_wire` key allowlist); this crate's `privacy_` family extends
  to coordinator-facing DTOs at Alpha (muster/SPEC-03).

Testing: `just muster::test`, `just muster::e2e` (e2e_ family). Criteria:
`.claude/plans/muster/specs/03-testing-criteria.md`; cross-crate policy:
`docs/src/dev/policies/testing/` (ADR-0027).
