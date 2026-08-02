# Phase 05 — Muster-SDK: greedy, objectives, suggestion

* Status: `in-progress` — slice 1 complete, slice 2 open
* Blocks: Phase 6 (Muster's room-assignment surface consumes `suggest`)
* Blocked by: orrery Phases 3–4 (complete — oracle, scoring, travel exist)

Two slices (working software early, Rule 03):

* **Slice 1 (this document's hypotheses):** SPEC-02/03 written (the
  handoff's deliberate gap); greedy assignment with the optimality proof
  obligation discharged by fuzzing against two independent oracles;
  objective composition with additive breakdowns; `suggest_room_schedule`
  end-to-end against the engine. This is the SDK PoC **and** Prototype
  gates (greedy-matches-brute-force; assignment + breakdown returned).
* **Slice 2 (pre-commit when it starts):** local search (relocate/swap/
  shift, anytime), `StabilityFromReference` and `ExpectedAttendeeTravel`
  terms, batch orchestration (closure refresh + digest recompute + sweep +
  change sets), `rand` enters with a caller-supplied seed.

## Hypotheses (pre-committed, before implementation)

| # | Hypothesis | Falsified by | Status |
|---|---|---|---|
| H1 | **The interval-graph-colouring claim holds in code**: greedy ⟺ brute force ⟺ max-overlap ≤ k, at n ≤ 12, k ≤ 3 | any instance where the three disagree | **confirmed** — `optimality_greedy_matches_both_oracles`, 48 fuzz cases, three-way agreement asserted per case plus internal conflict-freedom of greedy's own output |
| H2 | Breakdown additive/deterministic; `ViolationCost` ≡ severity weights | non-additivity, nondeterminism, drift | **confirmed** — additivity to 1e-9 and bit-identical re-run asserted in integration; ViolationCost uses the same Hard 100/Warning 10/Info 1 table as `engine.score` (one definition, two call sites — noted for a future shared constant) |
| H3 | No feasibility semantics in the SDK | violation minted or predicate re-derived in SDK source | **confirmed** — `grep ViolationKind:: crates/muster-sdk/src` empty; the only overlap restatement lives in *test oracle* code, where independence is the point; `Suggestion.violations` relayed verbatim |
| H4 | Deps = {orrery, thiserror, tracing} | any other | **confirmed** |

## Acceptance criteria (pre-committed)

| Criterion | Threshold | Actual | Verdict |
|---|---|---|---|
| Gates | all green | 60/60 workspace tests; clippy/fmt/xref/check-scope clean (this host, 2026-08-02) | pass |
| Optimality | n ≤ 12, k ≤ 3, both oracles | green (`just muster_sdk::test-optimality`) | pass |
| Prototype gate | placements + violations + breakdown; unassigned surfaced | `suggestion_places_flags_and_explains`, `unassignable_events_are_surfaced_not_dropped` | pass |
| Breakdown | additive + deterministic; severity table consistent | asserted in integration | pass |
| Specs | SPEC-02/03 written | done, dated at Phase-5 entry | pass |
| Plans | rows updated | done | pass |
| Artifacts (standing criterion, added 2026-08-02) | plain-language explainer for non-experts | `artifacts/phase-5-room-suggestions.md` | pass |

Design decisions fixed in advance:

* **Greedy ordering is left-endpoint** for fixed start times — that is the
  provably-optimal case (interval graphs are perfect); "most-constrained-
  first" (SPEC-01/FR-1) becomes the ordering when heterogeneous
  compatibility arrives, and is recorded as slice-2+ scope.
* **Best-fit room choice** (smallest adequate capacity among free rooms,
  ties to smaller location id): any free room preserves the optimality
  invariant, so room *choice* is a quality knob, not a correctness one.
* **Unassignable events are returned, never dropped** — a suggestion that
  silently loses events is worse than no suggestion.
* Severity weights stay the engine's (Hard 100 / Warning 10 / Info 1);
  the SDK weights *terms*, not severities.

## Plan

1. SPEC-02/03 (done first — they gate the surface).
2. `assign::greedy` + `objective::{Term, Objective, Breakdown,
   ViolationCost, RoomUtilisation}` + `suggest::suggest_room_schedule`.
3. Tests: optimality proptest (two oracles), breakdown units, engine
   integration; gates; results; merge.

## Results (slice 1)

No hypothesis refuted (noted per Rule 01.3 — the risky quantitative claims
here were mathematical ones with known proofs, and the fuzzing confirmed
the implementation matches the theorem rather than testing the theorem).
The SDK PoC gate (greedy ≡ brute force, n ≤ 12) and Prototype gate
(assignment + breakdown returned) are both met.

Landed: `assign::greedy` (left-endpoint order, best-fit-by-capacity room
choice, deterministic ties, unassigned surfaced); `objective` (`Term`,
`Objective`, additive `Breakdown`, `ViolationCost`, `RoomUtilisation`);
`suggest::suggest_room_schedule` (greedy seed → engine overlay evaluation
→ breakdown, `sdk.suggest` span); muster-sdk/SPEC-02 and SPEC-03 filling
the handoff's deliberate gap.

## Decisions produced

* None requiring ADRs. Severity weights are duplicated between
  `engine::severity_weight` and `objective::ViolationCost` — flagged for a
  shared constant when the engine exposes one publicly (slice 2 nicety).

## Carry-forward

| Item | Resolves in |
|---|---|
| **Slice 2**: local search (relocate/swap/shift, anytime, `monotone_` tests), `StabilityFromReference` + `ExpectedAttendeeTravel` terms, batch orchestration (closure refresh → digest recompute → sweep → change set), `rand` with caller seed | Phase 5 slice 2 (own pre-committed criteria) |
| Most-constrained-first ordering when room-compatibility constraints arrive | SDK Alpha |
| Severity-weight table shared constant between engine and SDK | slice 2 |
| Explain-assignment (`explain` module) | SDK MVP |
| CP-SAT evaluate-or-reject | SDK RC |
