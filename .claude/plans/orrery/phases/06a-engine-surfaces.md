# Phase 6a — Engine surfaces for the app

* Status: `in-progress` — pre-committed 2026-08-02, before implementation
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
| H1 | **Preview honesty** (muster/SPEC-03:17-21, engine side): for each of the three mirrored command kinds, `preview_digests(cmd, at)` equals the change set of `apply(cmd)` followed by `refresh_digests(at)`, on proptest-generated worlds — and the preview itself performs zero repository writes (stored digests untouched, next receipt seq unchanged) | any proptest counterexample; any observed write | untested |
| H2 | **Mirror integrity** (review CR-1): after a preview, the salsa mirror is exactly as it was found — every person's incremental digest still equals the cold-path digest (`derive::expand` → `digest_of_ids`), including after further ordinary commands | any incremental-vs-cold divergence on a previewed engine | untested |
| H3 | **Preview is blast-radius-bounded** (the ROADMAP "needs salsa early cutoff" claim, tested via the probe): on a warmed engine, a preview affecting k of N persons re-executes the digest layer exactly k times, not N | probe `DIGEST` delta scaling with N on an unrelated-persons world | untested |
| H4 | **2-hop budget**: `co_attendance` meets the pre-committed < 50 ms p95 (orrery/SPEC-03:14) at the 10⁵ Alpha scale on MemoryRepo, this host | measured p95 ≥ 50 ms | untested |
| H5 | **Orrery Alpha gate**: with the 10⁵ budget set defined, every SPEC-03 budget class measured at 10⁵ on MemoryRepo fits its budget — i.e. the Orrery Alpha exit gate is met | any class over budget (each miss recorded per class; the gate then stays open and the misses become Phase-7 dossier input) | untested |
| H6 | **Anchors, verdicts-only**: the producer + consult land ADR-0014's core feature with no anchor association crossing the query boundary — verdict types carry durations/provenance only; anchored-world payload tests stay clean engine-side and in muster's wire-shape fixture (the slice-2 owed item) | any payload/serialisation/Debug of a verdict carrying an anchor location id or label; the muster fixture still impossible or red | untested |

## Acceptance criteria (pre-committed)

| Criterion | Threshold | Actual | Verdict |
|---|---|---|---|
| Gates | nextest workspace green; clippy -D warnings; fmt; doc-check; check-seam (grep fallback — no rustup on this host); check-scope; check-xrefs | | |
| Preview honesty | H1 property test green; unsupported kinds get the typed error, tested | | |
| Mirror integrity | H2 differential test green; H3 probe assertion green | | |
| Analytics | four functions landed with brute-force oracle property tests green | | |
| 10⁵ budget set | dated SPEC-03 addition defining the Alpha set; all seven classes measured at 10⁵; per-class verdicts + method + numbers in Results (untuned first pass) | | |
| 2-hop | p95 < 50 ms at 10⁵ (H4) | | |
| Anchors | `AddAnchor` (Structure-tier enforced, tested) · `anchors_for` · `first_event_feasibility` with unit tests incl. Unknown-is-not-an-accusation · `refresh_after` audit recorded at the kind site · muster worlds-with-anchors wire-shape fixture green | | |
| Trait/command growth | SPEC-04 dated notes for `AddAnchor`, `anchors_for`, `preview_digests`, `first_event_feasibility`, analytics module | | |
| Artifacts (standing) | plain-language artifact at `plans/orrery/artifacts/phase-06a-*.md` explaining what this phase added and why it is valuable, readable by a non-domain reader | | |

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

*(after measurement; refutations first per Rule 01.3)*

## Decisions produced

*(expected: none requiring ADRs — additive engine growth recorded here
and in SPEC-04; to be confirmed at close)*

## Carry-forward

*(at close; known already: sweep-side anchor violations await a
`depart_not_before` policy source — mobility profiles ADR-0017 or
app-supplied day boundaries.)*
