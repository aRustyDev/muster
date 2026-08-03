# Phase 6a — Engine surfaces for the app

* Status: `complete` — pre-committed 2026-08-02 before implementation;
  closed 2026-08-02. **Orrery Alpha exit gate met at 10⁵** (release,
  MemoryRepo, this host — Results, with qualifications).
* Blocks: Muster **Alpha** slice (needs the digest preview), Muster **Beta**
  slice (needs the analytics surface) — ROADMAP hard-deps, 2026-08-02
* Blocked by: nothing (Phases 3–5 complete; runs parallel with Phase 6 the
  way 1b ran alongside 3–5)

Created by the 2026-08-02 plan review (CR-2, CR-4; MO-4 item 2): work the
ROADMAP promised (orrery "○ compute" preview; Orrery Alpha analytics) but
no phase owned. Branch `feat/phase-06a-engine-surfaces`. All work is
orrery-side except the owed muster worlds-with-anchors privacy fixture,
which becomes possible only when this slice lands the producer
(06-app.md slice-2 Results item 2).

## Objective

At close, the engine exposes the four surfaces Muster's next slices
consume, none of which exist today: (1) a **non-persisting digest
dry-run** whose result provably equals the post-commit `refresh_digests`
change set; (2) the **analytics surface** (engagement, capacity pressure,
divergence, bounded 2-hop co-attendance); (3) a **defined 10⁵ budget
set** making the Orrery Alpha exit gate measurable — and measured; (4) an
**anchor producer** plus the anchor→first-event feasibility consult
(ADR-0014's core feature), which also unblocks the owed worlds-with-anchors
privacy fixtures.

## Design decisions (pre-stated)

**Preview.** `Engine::preview_digests(&mut self, cmd: &Command, at) ->
Result<Vec<PersonId>>`, supported for exactly the three mirrored fact
classes (`add_membership`, `add_subgroup`, `add_expectation`); any other
kind returns a typed `PreviewUnsupported` error rather than a
possibly-lying answer (e.g. `UpsertPerson` changes the person set itself,
which an overlay cannot represent honestly). Mechanism: snapshot the salsa
`World` input, set the overlaid fact vector (built from **repo reads plus
the command's would-be fact** — the same mapping `refresh_after` applies
post-commit), demand every person's digest, compare against *stored*
digest records (the same comparison `refresh_digests` makes), restore the
snapshot, return the changed set. CR-1 discipline: all fallible work
(person list, stored digests) happens **before** the world is touched; no
`?` exists between set and restore, so no path leaves the mirror overlaid.
Nothing is persisted — `SetDerivedDigest` is never issued. Rule 00.2 is
untouched: preview is a read surface.

**Anchors.** `Command::AddAnchor(Anchors)` — validates person and location
exist and the location is `Structure`-tier (ADR-0014). Repo read
`anchors_for(person, at)` (entity-partitioned, constant instant).
`applies_when` is stored but unread in v1 (firms up with ADR-0017); every
`during`-valid anchor is applicable. **`refresh_after` audit (the standing
trap): `add_anchor` touches no mirrored fact class (memberships /
subgroups / expectations), so it must NOT bump the mirror — audited here
in writing, comment at the kind site, same as `RemoveAttendance`.**
Consult: `Engine::first_event_feasibility(person, window,
depart_not_before) -> Result<Feasibility>` — earliest placed event in
`window` starting at or after `depart_not_before`; gap = event start −
`depart_not_before`; best verdict across applicable anchors via
`travel_best` (any feasible anchor ⇒ `Feasible`, mirroring the
don't-accuse rule); no anchors, no placed event, or no known route ⇒
`Unknown` — not an accusation. Returns the existing `travel::Feasibility`
(durations + provenance only — verdicts-only by type). **Not** wired into
`sweep()`: a sweep-side anchor violation needs a `depart_not_before`
policy source the engine doesn't have (that's mobility-profile /
app-policy territory — carry-forward, not scope creep).

**Analytics.** New `analytics` module: pure functions over
`&dyn Repository` (no datastore type, no I/O). Definitions pre-committed:

* `engagement(repo, person, window, at) -> f64` — Σ `effective_priority()`
  over the person's **effective schedule** in `window` (explicit edges,
  plus non-shadowed derived edges at their seeded `priority_group` —
  a derived edge has no person/coord override by construction). Group
  roll-up `engagement_by_group(repo, group, window, at)` over members
  valid at `at`.
* `capacity_pressure(repo, window, interest_threshold) ->
  Vec<CapacityPressure { event, signalled, allocated }>` for events
  overlapping `window`: `signalled` = the detector's own
  `signalled_interest` (identical semantics, count above threshold);
  `allocated` = Σ per-hold `capacity_override ?? location.capacity` over
  resolvable holds, `None` when no hold resolves. The threshold is
  **caller-supplied** — this is the ADR-0018 / ROADMAP "attendance-model
  hook" delivered at primitive level; the sweep's detector default (0.0)
  is unchanged, and a richer strategy object stays SDK-owned.
* `divergence(repo, group, window, at) -> DivergenceSummary { edges,
  mean: Option, max: Option }` — over members-valid-at-`at`'s explicit
  attends edges overlapping `window` where `Attends::divergence()` is
  `Some`.
* `co_attendance(repo, person, window) -> Vec<PersonId>` — the ADR-0020
  bounded 2-hop with a time window: person → their attends edges
  overlapping `window` → those events' attends edges overlapping `window`
  → distinct other persons, sorted. **Explicit edges only** (the ADR-0020
  measured analytic was edge-based; derived co-attendance would require
  population-scale expansion — recorded, revisit on demand).

Each function ships with a brute-force oracle property test (the detector
discipline, applied to analytics).

**The 10⁵ budget set (Orrery Alpha gate).** Dated orrery/SPEC-03 addition:
the Alpha gate ("budgets met at 10⁵") uses the **same thresholds** as the
10⁶ table, evaluated at **10⁵ `attends` / 10³ persons / 10³ events / 200
locations** (the 10⁶ dimensions scaled ×0.1). Rationale: budgets are
ceilings; Alpha is the same contract at a tenth the scale; Beta re-checks
at 10⁶. Measurement harness: a `measure_` test (deterministic fixture,
stride-sampled persons, no new deps) records p50/p95 per class on
**MemoryRepo, this host**. Framing pre-stated: MemoryRepo is unindexed
`Vec` scans — a miss indicts the engine+MemoryRepo pairing at this scale,
not the engine design; either way the verdict is recorded, untuned
(Rule 01; the slice-2 H4 precedent). Phase 7 re-measures on real
candidate stores through the real engine.

## Hypotheses (pre-committed)

| # | Hypothesis | Falsified by | Status |
|---|---|---|---|
| H1 | **Preview honesty** (muster/SPEC-03:17-21, engine side): for each of the three mirrored command kinds, `preview_digests(cmd, at)` equals the change set of `apply(cmd)` followed by `refresh_digests(at)`, on proptest-generated worlds — and the preview itself performs zero repository writes (stored digests untouched, next receipt seq unchanged) | any proptest counterexample; any observed write | **confirmed** — `prop_preview_matches_post_commit_refresh` (64 cases, warmed and cold) + deterministic case; zero-write assertions green |
| H2 | **Mirror integrity** (review CR-1): after a preview, the salsa mirror is exactly as it was found — every person's incremental digest still equals the cold-path digest (`derive::expand` → `digest_of_ids`), including after further ordinary commands | any incremental-vs-cold divergence on a previewed engine | **confirmed** — `incremental_mirror_intact_after_previews_of_every_kind`, including after post-preview commands and at a second instant |
| H3 | **Preview is blast-radius-bounded** (the ROADMAP "needs salsa early cutoff" claim, tested via the probe): on a warmed engine, a preview affecting k of N persons re-executes the digest layer exactly k times, not N | probe `DIGEST` delta scaling with N on an unrelated-persons world | **confirmed** — exact: digest and derived_ids re-executed once for the 1 affected of 12 persons; extraction N times (the backdating cost) |
| H4 | **2-hop budget**: `co_attendance` meets the pre-committed < 50 ms p95 (orrery/SPEC-03:14) at the 10⁵ Alpha scale on MemoryRepo, this host | measured p95 ≥ 50 ms | **confirmed** — p95 7.4 ms release (debug: 52.1 ms — see Results qualification 1) |
| H5 | **Orrery Alpha gate**: with the 10⁵ budget set defined, every SPEC-03 budget class measured at 10⁵ on MemoryRepo fits its budget — i.e. the Orrery Alpha exit gate is met | any class over budget (each miss recorded per class; the gate then stays open and the misses become Phase-7 dossier input) | **confirmed** — all seven classes within budget in release (debug fails sweep and 2-hop — the profile rule was pinned pre-measurement; Results table) |
| H6 | **Anchors, verdicts-only**: the producer + consult land ADR-0014's core feature with no anchor association crossing the query boundary — verdict types carry durations/provenance only; anchored-world payload tests stay clean engine-side and in muster's wire-shape fixture (the slice-2 owed item) | any payload/serialisation/Debug of a verdict carrying an anchor location id or label; the muster fixture still impossible or red | **confirmed** — `privacy_` tests green both sides; demo world now carries a real anchor |

## Acceptance criteria (pre-committed)

| Criterion | Threshold | Actual | Verdict |
|---|---|---|---|
| Gates | nextest workspace green; clippy -D warnings; fmt; doc-check; check-seam (grep fallback — no rustup on this host); check-scope; check-xrefs | 91/91 tests (1 skipped = the `#[ignore]` measurement harness, run explicitly); all listed gates green (this host, 2026-08-02) | pass |
| Preview honesty | H1 property test green; unsupported kinds get the typed error, tested | green; `PreviewUnsupported` covered for attendance/removal/upsert kinds | pass |
| Mirror integrity | H2 differential test green; H3 probe assertion green | both green (own probe binary, one test per process) | pass |
| Analytics | four functions landed with brute-force oracle property tests green | `analytics.rs`: engagement(+by_group), capacity_pressure, divergence, co_attendance; 4 oracle proptests + honest-empty-world test | pass |
| 10⁵ budget set | dated SPEC-03 addition defining the Alpha set; all seven classes measured at 10⁵; per-class verdicts + method + numbers in Results (untuned first pass) | SPEC-03 amended; `measure_alpha_budgets.rs`; Results table (release + debug) | pass |
| 2-hop | p95 < 50 ms at 10⁵ (H4) | 7.4 ms p95 release; 52.1 ms debug (qualification 1) | pass |
| Anchors | `AddAnchor` (Structure-tier enforced, tested) · `anchors_for` · `first_event_feasibility` with unit tests incl. Unknown-is-not-an-accusation · `refresh_after` audit recorded at the kind site · muster worlds-with-anchors wire-shape fixture green | all landed; 8 consult tests + repo validation test; audit comment at the kind site; fixture green with the slice-2 honesty note resolved by dated addendum | pass |
| Trait/command growth | SPEC-04 dated notes for `AddAnchor`, `anchors_for`, `preview_digests`, `first_event_feasibility`, analytics module | all recorded, dated | pass |
| Artifacts (standing) | plain-language artifact at `plans/orrery/artifacts/phase-06a-*.md` explaining what this phase added and why it is valuable, readable by a non-domain reader | `artifacts/phase-06a-engine-surfaces.md` | pass |

## Plan

1. Pre-commitment (this document) — commit before implementation.
2. Preview primitive + `PreviewUnsupported` error + H1/H2/H3 tests.
   *(First: it blocks the Muster Alpha slice.)*
3. Anchors: command, storage, read, consult, privacy tests both sides,
   `refresh_after` audit.
4. Analytics module + oracle property tests.
5. SPEC-03 dated 10⁵ addition; measurement harness; record all classes.
6. Gates; Results refutations-first; artifact; SPEC-04/PLAN/ledger
   updates; merge `--no-ff`; NEXT-SESSION rewritten.

Steps 2–5 are reversible until merged; spec edits are dated and additive
(Rule 02). No new dependencies anywhere (Rule 06 — nothing to record).

## Results

**Qualifications first (Rule 01.3). No pre-committed hypothesis was
refuted this slice; the closest calls are therefore promoted to the top
so the confirmations below don't read cleaner than they are:**

1. **The Alpha gate verdict is profile- and backend-qualified.** All
   seven classes pass at 10⁵ in **release** on **MemoryRepo**. The same
   fixture in debug fails two classes (sweep 23.8 s vs < 10 s; 2-hop p95
   52.1 ms vs < 50 ms) and puts travel at 24.0 ms against a 25 ms
   budget. The release-profile rule was pinned in the harness header
   before any run (Rule 01.1) — the 10⁶ baselines came from optimized
   binaries, and a debug verdict would indict the compiler — but the
   margin is real information: it is being spent on MemoryRepo's
   unindexed linear scans (`travel_best` walks all ~39.8k closure rows
   per lookup). Phase 7's re-measure through real candidates at 10⁶
   remains the decisive pass; this gate certifies the engine +
   MemoryRepo pairing at a tenth that scale.
2. **The harness is `#[ignore]`, a deliberate deviation from
   measure_select's in-suite pattern**: 31.6 s in debug would tax every
   workspace gate run. The suite therefore reports "1 skipped" — the
   invocation is in the module doc, and the in-test assertions are 10×
   sanity bounds so an explicit run still catches order-of-magnitude
   regressions.
3. **`resolved 0` in the sweep is expected, not suspicious** — the
   fixture starts with no open violations; the 46,664 emissions also
   mean the measured sweep cost includes ~47k `RecordViolation` writes,
   which is the honest shape (Rule 01.7): this sweep materialises and
   persists its results.

**Measured (this host, 2026-08-02, `measure_alpha_budgets.rs`; 10⁵
attends / 10³ persons / 10³ events / 200 locations; verdicts from the
release column):**

| Class | Budget | Release | Debug (transparency) | Verdict |
|---|---|---|---|---|
| Cold open (`Engine::new`) | < 1 s | 126 µs | 437 µs | pass |
| Closure refresh (200 sources, 39,800 pairs) | < 60 s | 17.2 ms | 173 ms | pass |
| Global sweep (46,664 emitted) | < 10 s | **2.53 s** | 23.8 s | pass (release) |
| Derived expansion p95 | < 25 ms | 3 µs | 45 µs | pass |
| Conflict detection p95 | < 25 ms | 77 µs | 638 µs | pass |
| Travel feasibility p95 | < 25 ms | 2.35 ms | 24.0 ms | pass |
| 2-hop co-attendance p95 | < 50 ms | **7.44 ms** | 52.1 ms | pass (release) |

**Confirmed:** H1–H6 (table above). With the analytics surface landed,
every Orrery Alpha content item exists (salsa incrementality, digests,
travel Layer 1/2, analytics) and the newly-defined exit gate measures
green: **the Orrery Alpha stage gate is met**, qualified as above.

Landed: `Engine::preview_digests` (+ `OrreryError::PreviewUnsupported`;
muster-server maps it to 400) · `Command::AddAnchor` +
`Repository::anchors_for` + `Engine::first_event_feasibility` (verdicts
only; consult tests incl. first-event-not-reachable-event) ·
`analytics` module (four surfaces, oracle-tested; ADR-0018 threshold
hook as a caller-supplied parameter) · SPEC-03 10⁵ budget set (dated) ·
`measure_alpha_budgets.rs` · muster demo world seeds a real anchor
(slice-2 privacy honesty note resolved by dated addendum) · SPEC-04
dated growth notes · 72 → 91 tests (+1 ignored harness).

## Decisions produced

* **None requiring ADRs** (confirmed at close): no new dependencies, no
  non-negotiable changes; all engine growth additive and recorded in
  SPEC-04 with dated notes. The two design decisions of record —
  preview restricted to mirrored kinds with a typed refusal, and the
  consult's `depart_not_before` caller-supplied instant — live in this
  document's Design section and the code docs.

## Carry-forward

| Item | Resolves in |
|---|---|
| Sweep-side anchor violations: the consult stays a query surface until a `depart_not_before` policy source exists | conditional — mobility profiles (ADR-0017) or app-supplied day boundaries (ledger, Conditional table) |
| Debug-profile budget misses (sweep, 2-hop) quantify MemoryRepo's index-free scan cost; index only if a real consumer hits it (untuned-first discipline) | Phase 7 dossier (per-backend measurement at 10⁶ already owned there) |
| `engagement_by_group` / `divergence` resolve members via a `memberships_all` population scan — fine for batch analytics; a group-indexed member read is a possible future trait growth | next engine API touch, if a consumer needs it |
