# Phase 06 — Muster application

* Status: `in-progress` — slice 1 (PoC) complete; Prototype next
* Blocks: nothing downstream yet (MVP chain continues within this phase)
* Blocked by: Phases 3–5 (complete — engine, travel, SDK all exist)

Surfaces land in dependency order (PLAN): member self-selection →
coordinator groups → violation inbox → analytics → room assignment.

* **Slice 1 (this pre-commitment): the PoC.** Muster spec set written
  (the last deliberately-thin gap); a headless service layer over
  `Engine<MemoryRepo>`; the PoC story end to end: one member self-selects,
  a conflict appears, a coordinator expectation shows up as a derived
  entry with provenance. CLI demo + `e2e_` test. **No frontend decision**
  — QUESTION-0015 stays open by design until Prototype.
* Later slices: Prototype (full member flow + the QUESTION-0015 ADR),
  Alpha (coordinator flow + blast preview), Beta (analytics + rooms).

## Hypotheses (slice 1, pre-committed)

| # | Hypothesis | Falsified by | Status |
|---|---|---|---|
| H1 | **PoC gate**: conflict visible end to end from engine records; no feasibility logic app-side | conflict invisible or app-side logic | **confirmed** — `e2e_member_selects_and_sees_conflict_with_provenance`; boundary grep (`ViolationKind::` / `detect::` / `overlaps(`) empty over `crates/muster/src`; the schedule's conflict flags are reads of open violation records |
| H2 | Derived entry with provenance, no attendance write | missing/wrong provenance | **confirmed** — "Evening Social — expected via group 'cohort-26'" appears via derivation only; e2e asserts source group by name |
| H3 | No frontend commitment needed for the PoC | UI/server dep entering | **confirmed** — lib + CLI binary; QUESTION-0015 annotated and stays open until Prototype |
| H4 | No new dependencies | any new dep | **confirmed** — none added (`figment` deferral recorded) |

## Acceptance criteria (slice 1, pre-committed)

| Criterion | Threshold | Actual | Verdict |
|---|---|---|---|
| Gates | all green | 66/66 workspace tests; clippy/fmt/xref clean (this host, 2026-08-02) | pass |
| PoC e2e | conflict + provenance | green | pass |
| Boundary | greps empty | empty | pass |
| Demo | stdout story, exit 0 | runs; three-act output with ⚠ flags and provenance line | pass |
| Specs | SPEC-01/02/03 | written at phase entry | pass |
| QUESTION-0015 | annotated | done | pass |
| Artifacts (standing) | plain-language explainer | `artifacts/phase-6-first-app-poc.md` | pass |

## Plan

1. Specs 01–03 (done first). 2. `muster` lib: `MusterService` (select,
my_schedule with provenance + conflict flags, demo world builder) +
`main.rs demo`. 3. `e2e_` test on the library surface (not stdout).
4. Gates, results, artifact, merge.

## Results (slice 1 — PoC)

One near-miss caught by *running* the demo rather than only testing it
(Rule 01.3): derived schedule entries initially displayed the
**expectation's validity window** (which is what a derived edge's `during`
correctly is, per ADR-0003) instead of the **event's window** — the
Evening Social sorted before breakfast. Fixed in the service layer with a
comment; a reminder that the ADR-0003 distinction is easy to trip over at
every consumer, worth a line in the eventual frontend guidelines.

Landed: muster/SPEC-01/02/03 (the last deliberately-thin spec set,
closed); `MusterService` (select with immediate conflict readback,
my_schedule with provenance + violation-record flags); `run_demo` +
`muster demo` CLI; `e2e_` gate green. The **Muster PoC stage gate
(ROADMAP: "conflict visible end to end") is met**, and with it every
product in the workspace has reached at least its PoC.

## Decisions produced

* None requiring ADRs. One additive engine read (`Repository::group()`)
  was needed for provenance display — recorded here per the
  trait-growth discipline.

## Carry-forward

| Item | Resolves in |
|---|---|
| **Prototype slice**: browse/priority surfaces, full member flow, and the QUESTION-0015 frontend ADR (leaning: muster-server + muster-ui + thin types crate) | Phase 6 slice 2 |
| Coordinator flow (groups/expectations service calls replacing the `engine_mut` escape hatch), blast-radius preview honesty gate, inbox + waive | Phase 6 Alpha |
| `select()` currently sweeps the whole window for immediacy — fine at PoC scale; interactive-budget measurement at 10⁵+ | Phase 7 |
| Privacy tests extend to service DTOs on worlds with anchors | Alpha (with coordinator surfaces) |
| `figment` + subscriber installation when the first deployment knob exists | Prototype |
