# Adversarial plan review — forward-looking material (2026-08-02)

*Owner-directed (2026-08-02): mandatory before any Phase-6 Prototype work.
Run in the spirit of `.claude/agents/adversarial-reviewer.md` — arithmetic
integrity, cross-reference resolution, claim drift, consumer simulation —
scoped to the unbuilt ROADMAP stages (Muster Prototype→RC, SDK Beta/RC,
Orrery Alpha→RC), every phase doc's carry-forward table, the specs the
future work builds on, and ADR-0015/0021's Phase-7 plan.*

* Baseline at review: `main` @ `dee19ff`, clean tree, 66/66 workspace tests
  green (`cargo nextest run --workspace`, this host, 2026-08-02).
* Method: three parallel review passes (mechanical xref+arithmetic;
  engine-source verification of the kickoff's five suspects; completeness/
  ownership mapping across every carry-forward table and unbuilt stage
  gate) plus consumer simulation of documented commands by the main
  session. Findings below were re-judged against source before recording;
  each carries its evidence location.
* Companion deliverables produced by this review:
  * `.claude/plans/CARRY-FORWARD.md` — the consolidated cross-phase ledger
    (every live carry-forward, re-homed; this document tiers the defects,
    the ledger tracks the work).
  * Dated amendments listed per finding under **Disposition**, all made
    2026-08-02 as visible edits.

## The kickoff's five suspects — verdicts first (Rule 01.3)

| Suspect | Verdict |
|---|---|
| Salsa mirror assumes a single writer; unwritten as a Phase-7 requirement | **VERIFIED**, and sharper than stated (CR-1) |
| Blast-radius preview needs a non-persisting digest dry-run the engine lacks | **VERIFIED**, and the fallback is also foreclosed (CR-2, CR-6) |
| `expired_membership_effect` has no producer | **VERIFIED** — and the item fell off the ledger entirely (CR-3) |
| Engine analytics owned by no phase document | **VERIFIED** (CR-4) |
| `Warn` undefined; SPEC-03 budgets unmeasured; RC items unowned | **VERIFIED** with two corrections: the interactive budget lives in muster/SPEC-00 (not SPEC-03), and deterministic rebuild *verification* IS owned (Phase 7, PLAN.md) — though the rebuild *mechanism* is designed nowhere (MO-9) |

Two sub-claims **refuted**: `SetPriority` is not an engine gap (the command
exists — `command.rs:36`, applied `memory.rs:351`; only the service call is
missing, already scheduled for the Prototype slice), and
deterministic-rebuild-unowned is wrong at the verification level (see
above).

---

## Critical

### CR-1 — Salsa mirror invalidation silently breaks under a second writer, and no requirement document says so

The salsa mirror is built once at `Engine` construction
(`engine.rs:35-45`) and refreshed at exactly one site: this engine's own
`apply()` (`engine.rs:61-66`), keyed off the *command kind strings* the
engine itself applied (`incremental.rs:227-249` — `"add_membership"`,
`"add_subgroup"`, `"add_expectation"`). No version check, no re-mirror on
read, no subscription. A second writer process against a shared real
datastore (ADR-0015 Stage C deployment) would leave this engine serving
stale digests/derived results **while `sweep()` and `is_feasible()` read
the repo directly and see the new data** (`engine.rs:236-280, 371-424`) —
detection and digests silently disagreeing is worse than uniform
staleness. The in-process assumption is documented (`engine.rs:58-60`
"within one engine") but the deployment-level constraint appears in **no
requirement document** — not ADR-0015's criteria (criterion 3 covers the
*store's* concurrency model, not cross-process cache invalidation), not
ADR-0021, not PLAN.md's Phase 7 paragraph. Only the session-kickoff prompt
mentioned it.

Secondary fragility (inference, unverified in anger): `refresh_after`
string-matches `cmd.kind()` — any future mutating command touching the
three mirrored fact classes (e.g. the removal commands CR-6 adds) must be
added to the match by hand; the compiler will not catch omission.

**Disposition (amended 2026-08-02):** ADR-0015 acceptance criteria gain
criterion 6 (external-writer invalidation posture — dated addition, marked
post-Stage-B/pre-Stage-C); PLAN.md Phase 7 scope now names it. The
`refresh_after` fragility is a ledger item for the Prototype slice, which
adds removal commands (CR-6).

### CR-2 — Blast-radius preview requires an engine primitive that does not exist and that no orrery phase owns

`muster/SPEC-02:18` commits to `preview_expectation(...) -> Vec<PersonId>`
as a "dry-run: digests before vs after on a scratch evaluation", with a
property-tested honesty gate (`muster/SPEC-03:17-21`: preview equals the
post-commit `refresh_digests` change set). The engine has no such path:
`refresh_digests` persists as its semantics (`engine.rs:209-228`,
`Command::SetDerivedDigest`); the read-only `digest()` cannot evaluate a
hypothetical; the overlay oracle (`Assignment`, `engine.rs:344-350`)
carries only `attends`/`held` overlays — no expectation/membership overlay
— and computes violations, never digests. The
`oracle_scores_overlay_without_writing` test covers scoring only.
Commit-then-revert is foreclosed too (CR-6: no removal commands). Muster
Alpha's exit gate depends on this primitive; ROADMAP marks orrery
"○ compute" for preview (`ROADMAP.md:22`), but no orrery phase or PLAN row
owns building it.

**Disposition (amended 2026-08-02):** PLAN.md gains Phase 6a (engine
surfaces for the app, parallel with Phase 6): the non-persisting
expectation-overlay digest preview blocks the Muster Alpha slice;
`ROADMAP.md` hard-deps now record it. Design lands with Phase 6a's own
pre-committed phase doc, not here.

### CR-3 — `expired_membership_effect` is orphaned: both phases named as its resolver closed without it

Phase 3's carry-forward (`03-engine-core.md:215`) assigned the producer
("persisted derived-edge cache … reconciliation") to "Phase 5 batch
orchestration (SDK) or Phase 4". Phase 4 closed without mentioning it;
Phase 5's `batch::run` is closure→digests→sweep with zero engine edits,
and its carry-forward does not re-carry the item. The detector
(`detect/expired_membership_effect.rs`) is called by nothing but its own
tests; `sweep()` sweeps 6 of the 7 violation kinds (`engine.rs:287-294`,
`ExpiredMembershipEffect` absent); and the physical cache it audits never
came to exist — slice-2 "caching" became salsa memoization + digests,
which is non-durable and cannot go stale the way the detector checks.
The engine comment (`engine.rs:233-235`) is half-stale: travel has since
joined the sweep; expired-membership never did.

**Disposition (amended 2026-08-02):** re-homed to Phase 7 in PLAN.md and
the ledger, with the honest framing: if Phase 7's datastore decision keeps
derivation non-durable, the right move is to re-scope or withdraw the
detector by ADR — not to build a cache solely to give the detector
something to audit.

### CR-4 — Engine analytics are fully unowned, and the Orrery Alpha stage is unownable as scheduled

Engagement, capacity pressure, divergence, and bounded 2-hop co-attendance
are promised by the boundary matrix (`ROADMAP.md:12-13`), Orrery Alpha's
contents (`ROADMAP.md:44`), `orrery/SPEC-02` FR rows (:68-73), muster PRD
FR-9/10/11, and `muster/SPEC-02:21` service calls — and owned by **no
phase document or PLAN row** (orrery phases end at 04, then 07; Phase 7's
paragraph is hardening + ADR-0015 only). Code: only a per-relation
`divergence()` helper exists (`model.rs:241-243`). The placement is also
self-contradictory three ways: `muster/SPEC-02:21` says the engine surface
lands "Phase 6 Alpha", the same file's slicing (:33) and both ROADMAPs say
Muster **Beta** consumes it, and the ROADMAP hard-deps list never records
the Muster-Beta→analytics dependency at all. Compounding: Orrery Alpha's
exit gate ("budgets met at 10⁵") references a budget table headed
"Measured at 10⁶" (`orrery/SPEC-03:5-17` — no 10⁵ budget set exists), and
every budget measurement is deferred to Phase 7 — i.e. Orrery Alpha as a
distinct stage cannot currently be exited before the phase that closes
Beta (stage ladder and phase plan have decoupled).

**Disposition (amended 2026-08-02):** Phase 6a (new PLAN entry) owns the
analytics surface, ordered before the Muster Beta slice; ROADMAP hard-deps
now record the dependency; `muster/SPEC-02:21` corrected to Beta. The
Orrery Alpha 10⁵-budget ambiguity is a ledger item: define the 10⁵ budget
set (or re-state the gate at 10⁶) in Phase 6a's pre-commitment.

### CR-5 — The funnel didn't narrow, but every forward document still plans for 2 finalists

Stage B eliminated nobody ("All three candidates survive to Stage C",
`01b-screening.md:283-284`; root `PLAN.md:30` concurs), yet ADR-0021:35
("decide between 2 finalists"), ADR-0021:70 ("Three repository
implementations total"), `PLAN.md:86,113`, `ROADMAP.md:45`,
`orrery/ROADMAP.md:8`, and `orrery/PLAN.md:13` all still assume two.
Nobody owns the 3→2 narrowing; no document says whether Phase 7 builds
two or three repository implementations (three means four total including
`MemoryRepo`, invalidating ADR-0021's cost arithmetic).

**Disposition (amended 2026-08-02):** dated addendum on ADR-0021: Phase-7
entry begins with an explicit, criteria-driven down-select to 2 (using
Stage-B data + the Phase-7 dossier: grafeo's qualitative findings, cozo's
dormancy/fork-readiness, agdb's screening profile) *before* repository
implementation starts; carrying three to full implementations would
require superseding ADR-0021 with the cost re-stated. PLAN.md Phase 7
scope updated to match.

### CR-6 — The Command enum has no removal or reversal variants at all

`command.rs:21-106` contains zero Remove/End/Retract/Cancel commands: a
member cannot deselect an event, and no membership, expectation, or hold
can be retracted or shortened after the fact. This (a) blocks the
**Prototype member flow as the PRD defines it** — Flow A ends "see
conflicts immediately → **resolve** or accept", and resolving a
self-selection conflict means removing one of the selections; (b)
forecloses the commit-then-revert fallback for CR-2's preview; (c) means
`sweep()`'s auto-resolve path (violations whose cause disappeared,
`engine.rs:316-329`) is unreachable for attendance causes in production
use. Not recorded in any carry-forward.

**Disposition:** reshapes the Prototype slice (per the kickoff's standing
instruction): `RemoveAttendance` lands in this slice as the minimal
removal command, with a `deselect` service call completing Flow A;
retraction of memberships/expectations/holds goes to the ledger for the
Alpha pre-commitment. Recorded in the 06-app.md slice-2 pre-commitment.
Note for implementers: CR-1's `refresh_after` string-match must learn the
new command kind, and the mirror-refresh test family should cover it.

---

## Moderate

### MO-1 — Policy semantics: `Warn` undefined everywhere, `Off` is a no-op, `Prevent` silently covers 2 of 7 kinds

`Policy { Off, Detect, Warn, Prevent }` (`detect.rs:40-47`) is read only
by `prevent_gate` (`engine.rs:69-136`) and only for `TimeConflict` and
`LocationExclusivity`. At runtime `Warn ≡ Detect ≡ Off` — `Off` does not
even suppress detection — and `Prevent` on the other five kinds behaves as
`Detect` with no spec stating the limitation (ADR-0012, `orrery/SPEC-02`,
`orrery/SPEC-04` all list the vocabulary and define only `Prevent`).
Phase 3 carried "`Warn` behaves as `Detect` (no notification channel)" to
"Phase 6", but no Phase-6 slice owns it. **Disposition:** ledger →
Muster Alpha pre-commitment must either define Warn's observable semantics
(likely: Warn = Detect + inclusion in the notification `ChangeSet`) or
shrink the enum by ADR; the partial-Prevent limitation gets a spec note
then. Not this slice: Warn without a delivery channel is undefinable.

### MO-2 — `select()` runs a whole-population sweep per interactive click; the 100 ms budget is worded to exclude it; nothing has ever been measured

`service.rs:62-81` calls `engine.sweep(at, window)` — all persons, all
locations, all events (`engine.rs:236-280`) — on every selection. The only
interactive budget is `muster/SPEC-00:21` "within 100 ms *of the engine
returning*", which excludes the sweep; `orrery/SPEC-03` classes the global
sweep as **batch** (<10 s at 10⁶) and warns against exactly this
conflation. Zero latency measurements exist (no benches, no timing in any
test, `evidence/` is datastore-only). Pointer conflict: `06-app.md:76`
defers measurement to Phase 7; the kickoff says measure in the Prototype
slice. **Disposition:** measured in this slice (pre-committed criterion in
the 06-app slice-2 section); optimisation only if the number demands it.

### MO-3 — Repository-impl agreement gated at Alpha by SPEC-05, at Beta by everything else

`orrery/SPEC-05:58` (Alpha: "two repository impls agree") vs
`ROADMAP.md:45`, `orrery/ROADMAP.md:8`, ADR-0021:74 (second impl is a
**Beta** gate). As written, Orrery Alpha could not exit without Phase-7
work every other doc schedules a stage later. **Disposition (amended
2026-08-02):** SPEC-05 release-gate table corrected to Beta, dated.

### MO-4 — Five carry-forward pointers expired: their resolver phases closed without resolving or re-carrying them

(1) Severity defaults ← "Muster PoC feedback" (`03-engine-core.md:123`) —
PoC closed, no feedback recorded. (2) Anchor→first-event feasibility
(ADR-0014's core feature) ← "Phase 5/6" (`04-travel.md:107`) — 5 closed,
6's slices silent. (3) Sibling-rule common-parent refinement — bounced
3→4→"5 or 7", now implicitly Phase-7-only. (4) Grafeo tier-constraint
enforceability probe ← "Phase 1b" (`01a:101`) — 1b's results contain no
tier probe and no written waiver (the Rule 01.2 pattern the project
guards against). (5) `Warn` ← "Phase 6" (see MO-1). **Disposition:** all
five re-homed with explicit owners in the ledger; the grafeo probe is
additionally a Phase-7-dossier item (it conditions a legal Cozo/grafeo
selection).

### MO-5 — Late stages have no owning phase rows: SDK Beta/MVP/RC, Muster MVP/RC

`muster-sdk/PLAN.md` ends at 05 (churn gate "SDK Beta / Phase 7" — Phase 7
is an orrery phase that never mentions SDK churn); `muster/PLAN.md` ends
at 06, whose slice list stops at Beta — MVP (auth/admin/locations) and RC
(accessibility, ops docs, backup/restore) are unowned, which also strands
Phase 4's portal-cost item (resolver "Phase 6+ location admin").
Notification delivery is a boundary-matrix obligation
(`ROADMAP.md:20`, `muster/SPEC-02:22` "delivery is this crate's job and no
one else's") that appears in **no** muster stage contents at all.
**Disposition (amended 2026-08-02):** both product PLANs gain explicit
not-started rows stating "phase pre-commitment required before stage
entry"; notification delivery pinned to the Muster Beta row (it needs SDK
change sets, available since Phase 5). Gate-definition debts → MO-8.

### MO-6 — Feasibility-cache key: Rule 00.5/ADR-0017 vs SPEC-04/code use different keys for different caches without saying so

Rule 00.5 and ADR-0017:28 say caches key on `(profile_id, e1, e2)` (event
pair — the verdict cache mobility would invalidate); `orrery/SPEC-04:114`
and `travel.rs:45-46` say `(profile_id, from, to)` (location pair — the
as-built Layer-2 travel cache, currently profile-less via
`travel_best(from,to)`). No verdict cache exists yet, so nothing is
violated — but the drift is exactly how a non-negotiable erodes: a future
implementer keying verdicts on location pairs would change Rule 00.5
semantics without noticing. **Disposition (amended 2026-08-02):** SPEC-04
now distinguishes the two caches, dated; if a verdict cache is ever keyed
otherwise than Rule 00.5 states, that is an ADR, not a comment.

### MO-7 — A refuted differential-testing claim still stands uncorrected in two specs

`orrery/SPEC-03:37` and `orrery/SPEC-05:21-22` assert the spike's
differential test "caught nothing because results agreed exactly";
`00-grounding.md:160-162` established that as-shipped Q1 disagreed 44 vs
58 and only the corrected run agrees. Rule 01.4 propagation risk straight
into Phase-7 harness design. **Disposition (amended 2026-08-02):** dated
qualifying notes added at both sites.

### MO-8 — Four unbuilt-stage exit gates are not measurable as written

* SDK Beta "re-solve changes < 10% for one room removal": instance class,
  scale, room-selection rule, and statistical treatment all undefined;
  "realistic scale" quantified nowhere (no SDK-level scale targets exist).
* SDK RC "perf gates green": **no SDK perf gates exist anywhere** — an
  undefined referent, the worst gate in the set.
* Muster Beta "full track scheduled end to end": "full track" has no size,
  fixture, or acceptance criterion.
* Muster MVP / SDK MVP human-outcome gates ("real coordinator unaided",
  "organiser accepts unedited"): no trial protocol, population, or n.

**Disposition:** ledger items pinned to each stage's entry
pre-commitment, consistent with Rule 01.1 (pre-commit before measuring);
ROADMAP carries a dated pointer so the debts are visible where the gates
live.

### MO-9 — Phase 7's scope exists only as two PLAN paragraphs while ~15 carry-forward rows point at it

Committed Phase-7 preconditions scattered across six documents with no
consolidated dossier: baseline re-measure on the decision host
(ADR-0015:46), cozo fork/vendoring readiness (`01a:100` — a legal
precondition for selecting Cozo that nothing owns), grafeo upstream-bug
report + L-scale re-evaluation + parse/plan overhead (`01b:305-307`),
10⁵-row materialisation, mixed read/write + salsa interleaving, sweep and
closure budget measurements, screening-harness disposal, the CR-1
single-writer requirement, the CR-3 detector decision, and now the CR-5
down-select. Deterministic rebuild: *verification* is named
(`PLAN.md:116-117`) but the rebuild **operation** is designed nowhere.
**Disposition (amended 2026-08-02):** PLAN.md Phase 7 paragraph now
carries the full item list by pointer to the ledger's Phase-7 section —
the dossier the eventual phase doc pre-commits against.

### MO-10 — Consumer simulation: the documented e2e entry point is broken

`muster/SPEC-03:5` documents the e2e family as `just muster::e2e`;
`crates/muster/justfile:10` passes `--features e2e`, but the crate
declares no such feature — the recipe fails on execution
(`error: the package 'muster' does not contain this feature: e2e`;
reproduced this session). The PoC's e2e evidence was real (the test is
unconditional and runs in the workspace suite); the documented door to it
is what's broken. **Disposition:** fixed in the Prototype slice (recipe
drops the flag).

### MO-11 — Group-scoped violation queries have no supporting primitive

`muster/SPEC-01:29` scopes coordinators to waiving violations "touching
G"; `muster/SPEC-02:19` promises `inbox(filter)`. The only primitive is
unfiltered `open_violations()` (`repo.rs:81`), and violation subjects
carry no group refs for the kinds coordinators triage (time-conflict
subjects are person+events). An app-side join through memberships is
possible but unindexed and unowned. **Disposition:** ledger → Muster
Alpha pre-commitment (decide: repo query, engine surface, or app-side
join with measured cost).

---

## Low

* **L-1** `.claude/CLAUDE.md` said "23 decisions"; truth 24 (25 after this
  slice's ADR-0025). Count dropped from the line (amended 2026-08-02) —
  the number re-stales on every ADR; the pointer doesn't.
* **L-2** `orrery/PLAN.md` staleness: open-questions line still listed
  QUESTION-0014 as open/gating (closed by ADR-0024); Phase 07 row said
  "blocked by 03–06" with 03–05 complete. Both corrected (amended
  2026-08-02).
* **L-3** Room-assignment dependency stated as SDK Prototype in
  `muster/ROADMAP.md:12` vs SDK Alpha in `ROADMAP.md:96`; ASCII dependency
  graph (`ROADMAP.md:79-82`) draws the SDK-Alpha arrow at Muster
  Prototype, which needs no SDK at all. Aligned to SDK Alpha, diagram
  corrected (amended 2026-08-02).
* **L-4** Gate-table omissions: `muster-sdk/SPEC-03` release-gate table
  lacked the MVP row; `muster/SPEC-03`'s RC row omitted backup/restore.
  Rows added, dated (amended 2026-08-02).
* **L-5** `ADR-0025` dangles from NEXT-SESSION.md — deliberate forward
  reference; resolves this slice when the ADR is written.
* **L-6** `01a-paper-screen.md:35` elimination-attribution sentence covers
  16 of 18 (StromaDB, Grust unattributed); RESEARCH-0005's table is
  complete and the sentence claims no partition. Cosmetic; left.
* **L-7** Bare `SPEC-03` reference at plans root (`NEXT-SESSION.md:42`,
  means orrery/SPEC-03); the checker misses hyphenated bare refs. File is
  deleted at slice close; the checker gap is a ledger nice-to-have.
* **L-8** Non-phase resolver targets ("next owner touchpoint",
  "opportunistic", "when a consumer needs it", "next engine API touch",
  "next toolchain touch") are now tracked in the ledger rather than left
  floating. The owner question from `01a:104` (what was "forGQL"?) remains
  unanswered and is in the ledger's owner-touchpoint section.
* **L-9** `events()` takes no window (`repo.rs:78`) — browse filters
  app-side; fine at Prototype scale, noted in the slice pre-commitment.
  `pending_changes()` is read-shaped in SPEC-02 but write-implemented by
  `batch::run` — naming/semantics to settle at Muster Beta (ledger).
* **L-10** `00-grounding.md:115` "QUESTIONS-0014/0015" typo form; targets
  resolve. Historical doc; left.

## Checked and clean (zero-finding categories — reported per Rule 01.3)

* **Arithmetic** (recomputed by command): "22 ADRs / 15 questions / 5
  research documents" ✓; "20 scorecards" ✓; "3 of 5" and the "2 of 5
  as-shipped" provenance note ✓; 01a's 20-scored / 6-taxonomy / 2–4
  survivor-branch counts ✓; 01b's threshold arithmetic (no candidate at
  ≥10× on 2 of 3 at M) ✓; the test-count chain 42→50→57→60→65→66 across
  phase docs ✓; stage-completion claims vs phase-doc status lines ✓. Only
  arithmetic failure: the ADR count (L-1).
* **Cross-references**: every ADR-0001..0024, QUESTION, RESEARCH, and
  product-qualified SPEC reference in the forward-looking set resolves;
  the only dangler is the deliberate ADR-0025 (L-5).
* **Semantic spot-checks passed**: ADR-0021's SPEC-03/05 differential
  claims; NEXT-SESSION's QUESTION-0015 leaning quoted verbatim-correct;
  the Muster Prototype gate quote matches `muster/ROADMAP.md:6`;
  ADR-0016 D cited correctly everywhere; `orrery/PLAN.md:10`'s
  no-elimination summary accurately reflects 01b's pre-committed
  criterion.
* **Deliberate deferrals consistent**: the ROADMAP Deferred table
  (transit, mobility, event-log, CP-SAT timing, calendar, forecasting,
  cascade) matches the phase docs' deferral records pointer-for-pointer.
* **SDK Alpha "expected-attendee-travel"**: owned and delivered
  (05-sdk.md slice 2) — checked because the kickoff's suspect list implied
  otherwise by adjacency.
* **Inverse finding**: SDK Beta's *contents* (batch, digests, change sets,
  anytime) already shipped in Phase 5; the stage's only remaining
  substance is the churn-gate measurement (MO-8's first item).
