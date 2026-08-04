<!-- Written 2026-08-02 at Phase 6 entry. -->

# muster/SPEC-03 — testing criteria

## The e2e family (`e2e_` prefix — `just muster::e2e`)

* **PoC gate**: a member self-selects two overlapping sessions → the
  conflict is visible end to end through *engine records only* (the app
  recomputes nothing); a group expectation appears on the member's
  schedule as a **derived** entry with provenance naming the group —
  without any attendance write.
* **Prototype gate**: full member flow (browse → select → priority →
  my-schedule with provenance) through the service layer.
* **Alpha gate**: coordinator flow — group, expectation, blast preview,
  inbox triage, waiver recorded with actor + timestamp.

## Blast-radius preview honesty (Alpha)

`preview_expectation` result must **equal** the actual post-commit change
set (the `refresh_digests` output after really committing) on generated
worlds — a preview that lies is worse than none. Property-tested.

## Privacy (`privacy_` prefix, extends the orrery family)

No coordinator-facing DTO, serialised payload, log line, or error carries
an anchor location — asserted mechanically over the service layer's
outputs on worlds *with* anchors present. The engine already guarantees
verdicts-only; these tests catch app-added leaks (the join-table
temptation).

## Boundary

* No feasibility logic in the app: muster sources construct no
  `ViolationKind`, call no detector directly, and re-implement no interval
  predicate — conflicts arrive as violation records or oracle output.
* Scope: `cargo tree -p muster` may gain UI deps at Prototype (per the
  QUESTION-0015 decision), never solver or datastore deps.

## Quality-review additions (2026-08-03, QR-3 — items resolve in `plans/quality-review/02-additions-and-order.md`; cross-crate policy in `docs/src/dev/policies/testing/`)

All Alpha-entry items below are pre-commitment content for the Muster
Alpha slice (M-6): the pre-commitment carries the six F-4 privacy
channels (a–f) explicitly, each either tested at Alpha or honestly
deferred with a dated owner (Rule 01.2).

* **Transition-day recurrence expansion (M-1 — Alpha entry).** ADR-0024
  *mandates* these tests ("Muster's Phase-6 specs must include…") and no
  spec carried them: expansion across DST transition days, asserted
  against hand-computed UTC instants. The engine cannot detect a
  mis-expanded recurrence; only consumer tests guard it.
* **Lapsed-coordinator authz property (M-2 — Alpha entry).** A
  coordinator loses power the moment their membership window ends
  (SPEC-01): property-tested at the service layer with windows that
  lapse mid-scenario.
* **Per-violation-class resolvability matrix (M-3 — Alpha entry).**
  "Resolves every violation class" (SPEC-00) becomes a tracked coverage
  table: one e2e per violation class, absences visible.
* **Preview utility criterion (M-4 — Alpha entry).** Distinct from the
  honesty property above: a pre-committed fixture scenario in which the
  blast preview flags an unintended mass change (the PRD's "prevents at
  least one unintended mass change in testing").
* **Session-state privacy (M-5, F-4 d — Alpha entry).** Executable
  assertion that app-owned session state contains no anchors or
  coordinates (SPEC-01's "never contains" list, currently untested).
* **Egress gate (M-8, F-4 b — owner: RC pre-commitment).** The
  Parquet/CSV "anchors excluded by default" posture gets its executable
  test statement at the Beta/RC pre-commitment; posture restated in
  `docs/src/dev/policies/testing/standing-policies.md`.

### muster-types (inherits this spec — per-crate sections, not separate files)

* **Serde roundtrip properties (T-1 — Alpha entry).** Every wire DTO
  gets a proptest roundtrip (serialize → deserialize → equality); the
  unknown-field posture is decided alongside the coordinator DTOs. These
  are the crate's first own tests, arriving with its first input
  surface; until then its test absence is by design
  (`docs/src/dev/patterns/testing/test-doubles.md`).
* **Cross-member privacy (T-2, F-4 c — Alpha entry).** Member A's wire
  payload contains no identifier of member B — the contract the
  privacy_wire key allowlist cannot see (person-shaped leaks inside
  allowed keys).

### muster-server

* **Error→status contract**: all five `OrreryError` variants pinned at
  the mapping seam — landed 2026-08-03 (QF slice, `error_contract.rs`;
  was 1 of 5).
* **Wire-name pinning**: the kind/severity `Debug` names crossing the
  wire are asserted as string literals — landed 2026-08-03 (QF slice,
  `wire_names.rs`); an engine enum rename now breaks a test, not the
  contract. Snapshot tooling, if any, is RR&P-8's call.
* **HTTP-edge latency (SRV-7 — Alpha exit)**: tower-oneshot timing
  harness under the RR&P-2 macro leg; budgets derive from SPEC-00's
  reconciled 100 ms definition.
* **Wire-payload deser fuzzing**: RR&P-3 target list.
  **Load/stress/spike/soak**: RR&P-9, defined at the Muster-Beta
  pre-commitment (the Mutex-serialized service is a deliberate
  single-writer ceiling — results read against that design).
  **Network profiling: rejected** (SRV-5) — no criterion depends on it;
  revisit only if HTTP-edge budgets miss with no service-layer cause.

### muster-ui

* **Testing approach is RR&P-8**, closing at the Alpha pre-commitment:
  render/snapshot mechanism, REST-client double, wasm-perf disposition,
  and a measurable a11y floor proposal. Until then the crate's one unit
  test is its honest coverage.
* **Bare-library leg**: gate-exercised since 2026-08-03 via `just
  matrix` (cargo-hack no-features leg, W-6/F-11).
* **ADR-0003 window trap (UI-3 — Alpha UI slice)**: the frontend
  guidelines written for the Alpha UI content carry the window-trap
  line (phases/06-app.md:56-58's homeless item).

## Release gates (ROADMAP)

| Stage | Gate |
|---|---|
| PoC | conflict visible end to end |
| Prototype | member flow complete |
| Alpha | coordinator flow complete |
| Beta | full track scheduled end to end |
| MVP | a real coordinator uses it unaided |
| RC | privacy assertions automated and green; accessibility; ops docs; backup/restore |

*(RC row completed 2026-08-02 — backup/restore was in the ROADMAP RC
contents but missing here. The Beta "full track" and MVP "unaided" gates
still need pre-committed definitions at stage entry: CARRY-FORWARD.md,
review MO-8.)*
