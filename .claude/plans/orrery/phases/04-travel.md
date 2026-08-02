# Phase 04 — Travel: Layer 1, closure, Layer 2, feasibility

* Status: `complete`
* Blocks: Phase 5 (SDK objectives use travel), full `is_feasible` coverage
* Blocked by: Phase 03 (complete)

## Objective

The two-layer travel model (ADR-0006) becomes real: Layer 1 is the sparse
ground-truth `traverse` network already in the repository; a `petgraph`
closure computation materialises Layer 2 (all-pairs costs restricted to
event-bearing locations) through the command layer; `feasible(person, e1,
e2)` lands with `person` carried and ignored (Rule 00.5, ADR-0017); and
impossible-travel detection joins both the sweep and the oracle overlay.

## Hypotheses (pre-committed, before implementation)

| # | Hypothesis | Falsified by | Status |
|---|---|---|---|
| H1 | The petgraph closure (per mode, with containment-connector edges) agrees with a naive Floyd–Warshall oracle on random sparse graphs | any cost mismatch on any generated graph | **REFUTED as designed, then confirmed as redesigned.** The oracle found two defects in the pre-committed connector design within the first proptest run: (1) with no traverse edges at all, connectors alone manufactured zero-cost "travel" between nested locations; (2) zero-cost connectors created free intra-building shortcuts undercutting real room→room edges (Dijkstra prefers room→building→room at 0 s over the measured 120 s edge). Redesigned to two passes — the real network is authoritative; connectors only bridge otherwise-unreachable pairs — after which the oracle (mirrored to the same semantics) passes all cases, and the shortcut case is pinned as a named regression test |
| H2 | Event-bearing restriction bounds the cache; intermediates never endpoints; refresh idempotent | oversized cache, intermediate endpoint, non-idempotent refresh | **confirmed** — `closure_restricted_to_targets_and_idempotent`; integration test's report: 2 sources, 2 pairs with buildings as intermediates only |
| H3 | Unknown ⇒ never a violation; exact slack/deficit | false accusation or off-by-one | **confirmed** — `feasibility_verdicts` (slack 100 / deficit 100 / Unknown), `unknown_route_is_not_a_violation`, and the integration test's pre-closure sweep emitting nothing |
| H4 | Zero regressions; overlay + sweep detect travel | prior test breaking; overlay miss | **confirmed** — all 50 prior tests green; 57 total; sweep fires only after `refresh_closure`; DST spring-forward fixture untouched |
| H5 | Privacy: verdicts and violations carry no location/anchor identity | any leak | **confirmed** — `privacy_travel_violation_subjects_are_person_and_events_only` (the `just orrery::test-privacy` filter now has a real member); `Feasibility` carries durations only by construction |
| H6 | Exactly one new dependency: `petgraph` | any other | **confirmed** |

## Acceptance criteria (pre-committed)

| Criterion | Threshold | Actual | Verdict |
|---|---|---|---|
| Gates | nextest, clippy `-D warnings`, fmt, seam, xref green | 57/57; all clean (this host, 2026-08-02) | pass |
| Closure oracle | proptest vs Floyd–Warshall | green after the H1 redesign; oracle mirrors two-pass semantics | pass |
| Event-bearing restriction | tested; idempotent | `closure_restricted_to_targets_and_idempotent` | pass |
| Verdicts | three-way verdicts; Unknown silent | `feasibility_verdicts` + integration pre-closure sweep | pass |
| Integration | sweep + overlay + DST fixture | `travel_sweep_fires_only_after_closure_refresh` (Warning severity via Estimated bridge) | pass |
| Privacy | privacy_ test green | 1 test in the filter, green | pass |
| Rule 05 spans | closure_refresh span attrs | scope/pairs/sources attributes emitted | pass |

Design decisions fixed in advance (recorded so results can refute them):

* **Containment connectors.** Layer-1 pathfinding needs to cross tiers
  (room → building door → other building): `within` edges enter the graph
  as zero-cost bidirectional connectors. This implements ADR-0009's
  decomposition (room→exit + building→building + entrance→room) with
  exit/entry costs approximated at 0 for v1 — a known under-estimate,
  refined when measured data arrives.
* **Closure provenance is `Estimated`** unless the shortest path equals a
  direct traverse edge, which then donates its provenance — a computed
  multi-hop path is an estimate by construction, and detectors are already
  conservative on estimates (Warning severity).
* **`travel()` reads Layer 2 first, falls back to a direct-edge scan** —
  the cache supersedes when present; an empty cache degrades to Phase-3
  behaviour rather than to lies.
* **Best-cost lookup for detection** (`travel_best`): impossible-travel
  judges against the minimum cost across modes — a person who could have
  driven is not accused because walking is slow.
* **Placement rule**: an event's location for travel purposes is its
  primary hold (first non-overflow hold; deterministic tie-break by
  location id); unheld events are skipped by travel evaluation (they are
  `orphan_event`'s business, not travel's).
* **Closure replacement is atomic through one command**
  (`ReplaceClosure`), matching ADR-0006's batch-recompute write profile.

## Plan

1. `petgraph` dep; `ClosureEntry` model type; repo: `traverse_all`,
   closure store, `travel()` Layer-2-first, `travel_best`;
   `Command::ReplaceClosure`.
2. `travel` module: graph build (traverse + connectors), per-mode Dijkstra
   from event-bearing sources, `Feasibility`, `feasible(person, a, b, …)`.
3. `Engine::refresh_closure` (span per Rule 05) + `placed_for` join +
   sweep/oracle wiring (`ImpossibleTravel` added to swept kinds).
4. Tests: oracle proptest, restriction/idempotence, verdicts, integration,
   privacy; gates; results; merge.

## Results

**The headline is H1's refutation** (details in the hypothesis table): the
pre-committed connector design was wrong in two ways the property oracle
caught immediately, and the shipped design is the corrected one — real
network authoritative, connectors bridge-only. The bridge still
under-estimates (exit/entry ≈ 0), which is why bridged entries always carry
`Estimated` provenance and detectors stay at Warning severity for them.

Landed: `travel` module (`Feasibility` verdicts; `feasible(person, a, b)`
with `person` carried and ignored per Rule 00.5; two-pass per-mode closure
computation); `ClosureEntry` + atomic `ReplaceClosure` command;
`travel()` reads Layer 2 first with direct-edge fallback; `travel_best`
(minimum across modes — nobody is accused because walking is slow when
they could drive); `Engine::refresh_closure` (Rule 05 span) and
`placed_for` (primary-hold placement rule); impossible-travel wired into
both sweep and the oracle overlay; `ImpossibleTravel` added to swept kinds;
first `privacy_` test.

## Decisions produced

* No new ADRs. The two-pass bridging rule and the primary-hold placement
  rule are recorded here as Phase-4 design decisions; both are candidates
  for refinement when measured exit/entry costs exist (post-MVP, with
  mobility profiles or routing-API data).

## Carry-forward

| Item | Resolves in |
|---|---|
| Bridged-pair costs under-estimate (exit/entry ≈ 0): replace connector cost with measured/estimated per-portal values when travel data imports arrive | Phase 6+ (location admin) or routing-API import |
| Anchor→first-event feasibility (ADR-0014) — verdict-only surface exists (`Feasibility`), anchors not yet consulted; needs the coordinator-boundary privacy test extended when it lands | Phase 5/6 |
| `transit` (scheduled travel) stays v2 — breaks the scalar cache by design (ADR-0007) | post-RC |
| Closure refresh performance at 2k+ locations vs SPEC-03 60 s budget (functional, unmeasured) | Phase 7 |
| Sibling-rule common-parent refinement (from Phase 3) — still open | Phase 5 window or Phase 7 |
