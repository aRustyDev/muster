# Phase 03 — Engine core: detectors and derivation

* Status: `complete`
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
| **Slice 2**: salsa incrementality, persisted digests, sweep orchestration writing `Violation`s through the command layer (incl. the `Prevent` call site), `score()`, SPEC-01/02/04 updates | Phase 3 slice 2 (below) |
| Sibling rule common-parent refinement (needs containment lookup at traverse write) | Phase 4 |
| Severity defaults need product confirmation | Muster PoC feedback |
| Q5/SPEC-02 "excluding declared overflow" wording — detector implements the physical reading; harness clause shown dead on generated data | slice 2 spec cleanup |

---

# Slice 2 — incrementality, digests, sweeps, oracle

Pre-committed 2026-08-02, before any slice-2 implementation. Salsa API
verified against docs.rs (salsa 0.28.1, 2026-07-22): multi-argument tracked
functions intern their argument tuple; backdating compares results with
`PartialEq`; `DatabaseImpl` for a plain database.

## Hypotheses (slice 2, pre-committed)

| # | Hypothesis | Falsified by | Status |
|---|---|---|---|
| H6 | **Incremental ≡ cold** (SPEC-05): after an arbitrary command sequence, the salsa-backed digest for every person equals the digest computed cold from `derive::expand` over the repository — fuzzed over random sequences | any divergence for any person after any sequence | **confirmed** — `prop_incremental_digest_matches_cold`, 64 cases × 4 persons, sequences of up to 25 mixed membership/subgroup/expectation commands, random evaluation instants. One divergence was designed OUT before testing: the salsa chain is float-free, so the winner-per-event rule had to carry priority as an order-preserving bit pattern to match `expand` exactly — see Results |
| H7 | **Early cutoff is real** (ADR-0016 C): (a) a write for A does not re-execute B's expansion or digest; (b) an unrelated expectation write re-executes A's derived-ids but not A's digest | execution counters showing the forbidden re-executions | **confirmed, stronger than hypothesised** — salsa 0.28 tracks dependencies per input *field*, so in case (b) even A's `direct_groups`/`reach` never re-ran (they depend only on the memberships/subgroups fields). Asserted with execution counters in two process-isolated test binaries |
| H8 | **Sweep lifecycle idempotent and complete** | duplicates, missing resolutions, clobbered waivers | **confirmed** — emit → idempotent re-sweep → waive → neither duplicated nor auto-resolved; disappeared-cause resolution sets `resolved_at` |
| H9 | **`Prevent` = same detector, second call site** | prevented write mutating state, or a second detector implementation | **confirmed** — the gate calls the identical `time_conflict::detect` / `location_exclusivity::detect` functions; rejected write leaves state untouched; `Detect` lands the same write |
| H10 | Slice 2 adds exactly one dependency: `salsa` | any other new dependency | **confirmed** — salsa 0.28 only |

## Acceptance criteria (slice 2, pre-committed)

| Criterion | Threshold | Actual | Verdict |
|---|---|---|---|
| Gates | nextest, clippy `-D warnings`, fmt, seam, xref all green | 50/50 tests; all gates clean (this host, 2026-08-02) | pass |
| Incremental fuzz | salsa digest == cold digest, all persons, every case | 64 cases green (`tests/prop_incremental.rs`) | pass |
| Early cutoff | both H7 scenarios via execution counters | `incremental_cutoff_person` + `incremental_cutoff_ids` binaries (process-isolated counters) | pass |
| Digest persistence | on the person record via command; exact change-set; idempotent | `incremental_refresh_digests_returns_exact_change_set` | pass |
| Oracle | overlay evaluation + deterministic score | `oracle_scores_overlay_without_writing` (−100 conflicting / 0 clean; read-only verified) | pass |
| Spec updates | SPEC-04 rewrite, SPEC-01 note, SPEC-02 cleanups — all dated | done 2026-08-02 | pass |

Design decisions fixed in advance:

* **Salsa layering**: one `World` input (memberships, subgroup edges,
  float-free expectation keys) with per-person tracked extraction feeding
  reach → derived-ids → digest; early cutoff at the extraction layer is the
  mechanism that bounds blast radius. Digest chain is float-free by
  construction (id sets, not priorities — per ADR-0016 B the digest hashes
  the sorted derived-edge id set).
* **The engine mirrors base facts into `World` after each successful
  command**; only the three fact classes the derived chain reads trigger a
  mirror refresh.
* **Sweep dedup key** is `(kind, subjects)` against open violations; sweeps
  resolve only kinds they cover; waived violations are skipped entirely.
* **`is_feasible` returns freshly-minted `Violation` records** with
  `detected_at = assignment.at` (caller-supplied instant — the engine still
  reads no clock).

## Results (slice 2)

Near-refutation first (Rule 01.3): **the float-free salsa chain and
`derive::expand` would have diverged** on multi-expectation events — the
chain originally picked winners by group id alone while `expand` picks by
priority-then-group, and the two produce different `DerivedId`s whenever a
higher-priority expectation lives on a higher group id. Caught during
implementation review before the fuzz ran; fixed by carrying
`default_priority` as an order-preserving bit pattern (`priority_key`) so the
winner rule is bit-identical without a float entering the memoized chain.
The H6 fuzz then passed 64/64 — and exists precisely to catch this class.

Second finding: **salsa 0.28's dependency tracking is per input field**, not
per input struct, so early cutoff is finer than the design assumed — an
expectation write leaves the membership-derived layers completely untouched
(zero re-executions), not merely backdated. Blast radius is bounded one
layer earlier than ADR-0016's minimum requirement.

Landed: `incremental` (World mirror; `direct_groups → reach → derived_ids →
digest` tracked chain; probe counters — the crate's one piece of global
state, existing so cutoff is asserted rather than trusted); `Engine`
(Prevent gate, salsa mirror refresh scoped to the three fact classes the
chain reads, `refresh_digests` change-sets persisted on the person record,
sweep with `(kind, subjects)` dedup + disappeared-cause resolution +
waiver protection); `FeasibilityOracle` with `Assignment` overlay and
documented severity weights; commands `RecordViolation` / `ResolveViolation`
/ `SetDerivedDigest`; eight sweep-support repository methods; SPEC-01/02/04
updated with dated notes. **Orrery Prototype gate (ROADMAP): model,
interval algebra, all detectors, derived expansion — property tests green
vs brute-force oracles. Reached.**

## Decisions produced (slice 2)

* No new ADRs; ADR-0016 A–C are now implemented (D, the event log, remains
  v2 by design). Severity weights for `score` recorded here pending SDK
  objective composition (ADR-0013 keeps richer objectives out of the engine).

## Carry-forward (slice 2 close)

| Item | Resolves in |
|---|---|
| Travel feasibility into `is_feasible` + sweep (`feasible(person, e1, e2)` signature, Layer-2 lookups) | Phase 4 |
| `expired_membership_effect` needs a persisted derived-edge cache to audit (digests detect *that*, not *what*); producer lands with reconciliation | Phase 5 batch orchestration (SDK) or Phase 4 |
| Sibling-rule common-parent refinement | Phase 4 |
| Sweep performance at 10⁵–10⁶ edges vs SPEC-03 budgets (functional now, unmeasured) | Phase 7 benchmarks |
| `Warn` policy currently behaves as `Detect` (no notification channel exists yet — muster owns delivery) | Phase 6 |
