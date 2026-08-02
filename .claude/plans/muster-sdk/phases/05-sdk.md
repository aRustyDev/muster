# Phase 05 — Muster-SDK: greedy, objectives, suggestion

* Status: `complete`
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
| **Slice 2**: local search (relocate/swap/shift, anytime, `monotone_` tests), `StabilityFromReference` + `ExpectedAttendeeTravel` terms, batch orchestration (closure refresh → digest recompute → sweep → change set), `rand` with caller seed | Phase 5 slice 2 (below) |
| Most-constrained-first ordering when room-compatibility constraints arrive | SDK Alpha |
| Severity-weight table shared constant between engine and SDK | slice 2 |
| Explain-assignment (`explain` module) | SDK MVP |
| CP-SAT evaluate-or-reject | SDK RC |

---

# Slice 2 — local search, stability, batch orchestration

Pre-committed 2026-08-02, before any slice-2 implementation.

## Hypotheses (slice 2, pre-committed)

| # | Hypothesis | Falsified by | Status |
|---|---|---|---|
| H5 | **Monotone / anytime** | any fuzz case above seed total | **confirmed** — `monotone_improve_never_worse_than_seed`, 48 cases across random instances, seeds, and budgets down to 1 eval |
| H6 | **Heterogeneous-rooms improvement** (SDK Alpha gate) | search failing to beat greedy | **confirmed** — the small-room-committed-early instance; search finds the swap greedy cannot foresee |
| H7 | **Stability confines churn** | any unaffected placement moving | **confirmed** — stability holds a placement against greedy's own tie-break preference; only the displaced event moves |
| H8 | **Determinism under fixed seed** | divergence | **confirmed** — identical placements, totals, and eval counts across runs |
| H9 | Batch = engine primitives composed | any engine change required | **confirmed** — `batch::run` is closure→digests→sweep verbatim; zero engine edits in this slice |
| H10 | Deps += `rand` only | any other | **confirmed** |

Scope decision (pre-committed): the `Shift(event, Δt)` move from SPEC-01 is
**deferred** — it requires the free-start-times problem, which nothing
upstream produces yet; relocate and swap cover the fixed-start domain this
slice serves. Recorded here rather than silently dropped (Rule 01.2).

## Acceptance criteria (slice 2, pre-committed)

| Criterion | Threshold | Actual | Verdict |
|---|---|---|---|
| Gates | all green | 65/65 workspace tests; clippy/fmt/xref/check-scope clean (this host, 2026-08-02) | pass |
| Monotone | `monotone_` proptest green | `just muster_sdk::test-monotone` — green | pass |
| Alpha gate | refined < greedy on the constructed instance | `search_improves_on_greedy_for_heterogeneous_rooms` | pass |
| Stability | churn confined to displaced events | `stability_confines_churn_to_displaced_events` | pass |
| Determinism | identical outcome under fixed seed | `search_deterministic_under_fixed_seed` | pass |
| Batch | reports against `Engine<MemoryRepo>`, idempotent second run | `batch_run_reports_closure_changes_and_sweep` | pass |
| Artifacts (standing) | plain-language explainer | `artifacts/phase-5-polish-and-nightly.md` | pass |

## Results (slice 2)

No hypothesis refuted; the risky contracts (monotone under tiny budgets,
stability vs greedy's own preferences) were fuzz/adversarially tested
rather than assumed. The SDK Alpha behaviours exist ahead of schedule
(improvement on heterogeneous rooms, stability term); **the Beta churn
gate (< 10% on one room removal at realistic scale) deliberately remains
open** — the slice-2 stability test is qualitative, and the quantitative
gate needs Beta-scale instances (Phase 7 measurement discipline applies).

Landed: `search::improve` (relocate/swap, first-improvement, seeded,
budgeted, anytime), `StabilityFromReference` and `ExpectedAttendeeTravel`
terms (+ `attendee_flow` precomputation over repo data — priority-weighted,
per SPEC-01 "not type clustering"; unknown routes cost 0), `batch::run`
(closure → digests → sweep → `ChangeSet`), `notify::ChangeSet`,
`suggest_and_refine` with `RefineOptions`. `Shift` move deferred as
pre-committed.

## Decisions produced (slice 2)

* None requiring ADRs. `rand` entered per baseline with caller-supplied
  seeds only — no ambient randomness anywhere in the SDK.

## Carry-forward (phase close)

| Item | Resolves in |
|---|---|
| Beta churn gate (< 10%, realistic scale) — quantitative measurement | SDK Beta / Phase 7 |
| `Shift` move + free-start-time problem shape | when a consumer needs it |
| Simulated-annealing escape from local optima (first-improvement can stall) | SDK Alpha+ if instances demand it |
| `explain` module (which constraints bind, what a move would cost) | SDK MVP |
| Severity-weight shared constant (engine ↔ SDK) | next engine API touch |
| CP-SAT evaluate-or-reject | SDK RC |
