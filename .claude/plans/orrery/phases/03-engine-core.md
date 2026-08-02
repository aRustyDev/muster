# Phase 03 — Engine core: detectors and derivation

* Status: `in-progress`
* Blocks: Phase 4 (travel), Phase 5 (SDK needs oracle + scoring)
* Blocked by: Phase 02 (complete), QUESTION-0014 (closed — ADR-0024)

Two slices, so working software ships early (Rule 03):

* **Slice 1 (this document's hypotheses):** all seven detectors as pure
  functions with brute-force oracles; derived expansion with per-hop
  temporal filtering, cascade semantics, and content-addressed identity;
  tier-legality module; policy toggle; DST fixtures per ADR-0024.
* **Slice 2 (criteria to be pre-committed when it starts):** salsa
  incrementality, persisted digests, sweep orchestration that writes
  `Violation` records through the command layer, `score()`, SPEC-04 update.

## Hypotheses (pre-committed, before implementation)

| # | Hypothesis | Falsified by | Status |
|---|---|---|---|
| H1 | Each of the seven SPEC-02 detectors, written as a pure function in its own module, agrees with an independently-written brute-force oracle on proptest-generated worlds | any counterexample world | **confirmed** — seven detector modules, each with a proptest suite vs its own naive oracle; 42-test workspace suite green (cargo nextest, this host, 2026-08-01) |
| H2 | `derive::expand` (per-hop validity, depth ≤ 5, depth-0 inclusion, `cascades` honoured) agrees with a naive full-materialisation oracle, and `DerivedId` is bit-stable across recomputation on unchanged inputs | any counterexample; any id instability | **confirmed** — `prop_expand_matches_oracle` over random 3-level chains with random per-hop windows and cascade flags; `prop_derived_id_stable` |
| H3 | ADR-0024 holds in practice: spring-forward fires on the true 10-minute UTC gap; fall-back does not fire on apparent wall-clock overlap | either fixture judging by wall-clock appearance | **confirmed** — both fixtures use real 2026 US-transition epoch instants and assert as predicted |
| H4 | Slice 1 adds exactly one dependency beyond the Phase-2 set: `blake3` | any other new dependency | **confirmed** — blake3 only (ADR-0022 baseline; Rule 06 line: this row) |
| H5 | Tier legality is a total function plus portal/override escape hatch, exhaustively tested; command layer rejects illegal edges with typed errors | any accepted illegal edge; any panic-based rejection | **confirmed** — 25-pair containment matrix test; `tier_rules_enforced_at_write` asserts `CommandRejected` for inverted containment and portal-less cross-tier traverse, and accepts the explicit override marker |

## Acceptance criteria (pre-committed)

| Criterion | Threshold | Actual | Verdict |
|---|---|---|---|
| Gates | nextest, clippy `-D warnings`, fmt all green | 42/42 tests; clippy and fmt clean (this host, 2026-08-01) | pass |
| Detector purity | no repo types in detector signatures; no ids/clocks | signatures take slices + pure lookup closures only; drafts carry no id/timestamp | pass |
| Oracle independence | nothing shared beyond `Interval` | each oracle restates the overlap/reachability predicate from scratch (`max(start) < min(end)` form, raw edge-list walks) | pass |
| Property coverage | proptest per detector + derive | 8 property suites across detect/derive (+7 interval suites from Phase 2) | pass |
| DST fixtures | both ADR-0024 fixtures with real 2026 instants | `dst_spring_forward_fires_on_true_gap`, `dst_fall_back_no_false_conflict` | pass |
| Policy toggle | Policy + per-kind override present; Prevent documented | `detect::{Policy, PolicyMap}`; Prevent call site deferred to sweeps (slice 2) as pre-committed | pass |
| Tier module | 25-pair matrix + traverse portal/override paths | `containment_matrix_exhaustive`, `traverse_sibling_portal_and_override`, write-time rejection test | pass |
| Seam & audit | seam clean; xrefs green | both green post-change | pass |

## Plan

1. `tier` module + command-layer enforcement (`AddContainment`, traverse
   checks with `sibling_override` marker per ADR-0009).
2. `detect`: `ViolationDraft` (kind, severity, subjects — no ids, no
   timestamps: the engine assigns those when persisting, keeping detectors
   deterministic), severity defaults recorded as constants, `Policy` map,
   then the seven detectors each in its own module with oracle + proptests.
3. `derive`: `DerivedAttends` with `blake3` content-addressed `DerivedId`
   (expectation identity is content-derived — `group ‖ event ‖ during.start`
   — because SPEC-01 gives expectations no surrogate id; flagged for the
   SPEC-04/01 update in slice 2), `expand`, `effective_schedule` union where
   explicit attendance shadows derived.
4. Repo trait additions (additive): `event()`, `location()`,
   `containment()`. Command additions: `AddContainment`,
   `capacity_override` on `HoldLocation`.
5. Gates, results, merge; slice 2 opens with its own pre-committed criteria.

Design decisions fixed in advance (recorded so results can refute them):

* **Severity defaults** (pending product input): time_conflict,
  location_exclusivity, containment_exclusivity = Hard; impossible_travel =
  Hard on `Measured` travel data, Warning on `Estimated` (SPEC-03
  "conservative on estimated"); capacity_exceeded = Warning; orphan_event =
  Info; expired_membership_effect = Warning.
* **Missing travel edge ⇒ no impossible-travel violation.** An incomplete
  Layer-2 closure must not spray false positives; coverage gaps are a
  closure-refresh concern (Phase 4), not a per-person violation.
* **location_exclusivity pairs on different events only**, and the SPEC-02
  "excluding declared overflow" clause is interpreted as: the same-event
  multi-location overflow pattern simply never produces a same-location
  pair, so no special exclusion exists in the detector. The harness Q5's
  `overflow_for <> room` clause is dead code on its own generated data
  (verify against `evidence/_work` if disputed) and physically-impossible
  double-booking is a violation regardless of overflow declarations.
  Flagged for SPEC-02 wording cleanup in slice 2.
* **Consecutive-pairs semantics for impossible_travel**: events sorted by
  start; a pair is consecutive when the later starts at or after the
  earlier ends and no third event lies between; overlapping pairs belong to
  time_conflict, not travel.

## Results (slice 1)

No hypothesis was refuted this slice — noted per Rule 01.3 with the caveat
that the riskiest claims (incrementality, realistic-scale behaviour) belong
to slice 2 and remain untested. Near-misses worth recording:

* The impossible-travel oracle needed careful "consecutive" semantics —
  the naive all-ordered-pairs formulation disagrees with the detector by
  design, and the oracle re-derives successor-ship independently (no third
  event starting in the gap). The property suite constrains worlds to
  non-overlapping schedules because "consecutive" is ambiguous under
  overlap; overlapping pairs are time_conflict's domain (pre-committed
  design decision above).
* `expired_membership_effect` is meaningful only against a *cached*
  expansion (pure expansion cannot produce stale edges) — its tests
  construct staleness explicitly; the real producer arrives with slice-2
  caching, making this detector the cache's staleness audit.

Landed: `tier` (containment matrix, traverse sibling/portal/override rules,
enforced at the command layer with typed rejections); `detect` (drafts,
policy map, severity defaults, seven detectors each in its own module with
oracle + property tests); `derive` (blake3 `DerivedId`, `expand` with
cascade semantics and deterministic tie-breaking, `effective_schedule` with
explicit-shadows-derived); trait additions `event`/`location`/`containment`;
commands `AddContainment`, `capacity_override` on `HoldLocation`,
existence checks on holds/traversals; `sibling_override` marker on
`Traverse` (ADR-0009's "marked as such").

## Decisions produced

* No new ADRs. Severity defaults and the location-exclusivity overflow
  interpretation stand as recorded in the Plan section — both flagged for
  SPEC-02 wording cleanup in slice 2.
* Expectation identity remains content-derived (`group ‖ event ‖
  window-start`) pending the slice-2 SPEC-01/04 update.

## Carry-forward

| Item | Resolves in |
|---|---|
| **Slice 2**: salsa incrementality, persisted digests, sweep orchestration writing `Violation`s through the command layer (incl. the `Prevent` call site), `score()`, SPEC-01/02/04 updates | Phase 3 slice 2 (own pre-committed criteria) |
| Sibling rule common-parent refinement (needs containment lookup at traverse write) | slice 2 or Phase 4 |
| Severity defaults need product confirmation | Muster PoC feedback |
| Q5/SPEC-02 "excluding declared overflow" wording — detector implements the physical reading; harness clause shown dead on generated data | slice 2 spec cleanup |
