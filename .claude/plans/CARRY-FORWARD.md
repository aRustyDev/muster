# Consolidated carry-forward ledger (cross-phase)

*Created 2026-08-02 by the adversarial plan review
(`orrery/artifacts/plan-review-2026-08-02.md`). Phase docs remain the
historical record; **this file is the living backlog** — every unresolved
carry-forward from every phase doc, re-homed where its original resolver
expired. Maintenance rule: when a phase doc writes a carry-forward row, add
it here; when a phase closes, tick its rows here or re-home them
explicitly. A row silently vanishing is the failure mode this file exists
to stop (it has happened at least five times — review findings MO-4, CR-3).*

Columns: origin = doc:line that recorded it · owner = who resolves it now.
`(review)` = item created by the 2026-08-02 review itself.

## Phase 6 slice 2 — Prototype ✅ closed 2026-08-02 (phases/06-app.md slice-2 Results)

| Item | Origin | Owner status |
|---|---|---|
| QUESTION-0015 frontend ADR (ADR-0025) + crates per its outcome | 06-app.md:74 | ✅ done |
| Browse/priority surfaces, full member flow, `SetPriority` service call | 06-app.md:74 | ✅ done |
| `RemoveAttendance` command + `deselect` service call (Flow A "resolve") — `refresh_after` audited: the kind touches no mirrored fact class | review CR-6 | ✅ done |
| `select()` interactive latency measured | 06-app.md:76 | ✅ measured — p50 97.8 ms / p95 102.4 ms at 10³ persons; **H4 refuted in spirit → new Alpha row below** |
| `figment` + tracing-subscriber when the first real config knob exists | 06-app.md:78 | ✅ done (muster-server; OTLP explicitly deferred to Alpha) |
| Fix `just muster::e2e` (`--features e2e` flag, no such feature) | review MO-10 | ✅ done |

## Muster Alpha (Phase 6 slice 3) — pre-commitment must cover

| Item | Origin | Owner status |
|---|---|---|
| Coordinator flow: groups/expectations service calls replacing `engine_mut`, inbox + waive | 06-app.md:75 | owned (06-app later slices) |
| Blast-radius preview UI + honesty gate — ~~blocked by Phase 6a preview primitive~~ **unblocked 2026-08-02**: `Engine::preview_digests` landed, honesty property already proven engine-side; the slice owes the service call + UI + the muster-level property test | 06-app.md:75, muster/SPEC-03:17-21 | owned |
| Privacy tests extend to coordinator-facing DTOs on worlds with anchors | 06-app.md:77 | owned — *anchored worlds exist since 2026-08-02 (Phase 6a producer; member-facing fixture already green); the coordinator-facing DTOs are what remain* |
| `Warn` policy: define observable semantics (likely Detect + notification ChangeSet inclusion) or shrink the enum by ADR; document partial-`Prevent` (2 of 7 kinds) | 03-engine-core.md:218 (expired "Phase 6"); review MO-1 | **re-homed here** |
| Group-scoped violation query (`inbox(filter)`, "touching G") — repo query vs engine surface vs measured app-side join | muster/SPEC-01:29, SPEC-02:19; review MO-11 | **re-homed here** |
| Retraction commands beyond `RemoveAttendance` (membership/expectation/hold end-or-shorten) | review CR-6 | **new** |
| Severity defaults product confirmation (expired "Muster PoC feedback") | 03-engine-core.md:123; review MO-4 | **re-homed here** — coordinator flow is where severity is felt |
| Person-scoped `select()` evaluation: replace the whole-window sweep on the interactive path (measured 2026-08-02 at p50 97.8 ms / p95 102.4 ms @ 10³ persons — budget knife-edge, zero headroom); conflicts must still land as records | 06-app.md slice-2 H4 (refuted in spirit) | **new — pre-committed for Alpha** |
| OTLP exporter wiring behind the existing `exporter` knob (deferred from slice 2; needs a collector to receive it) | 06-app.md slice-2 dep note | **new** |
| muster-ui REST client + `dx` web entrypoint + UI content (components/type-sharing landed in slice 2) | ADR-0025; 06-app.md slice-2 | **new** |

## Muster Beta (Phase 6 slice 4)

| Item | Origin | Owner status |
|---|---|---|
| Capacity/engagement/divergence surfaces + room assignment — ~~blocked by Phase 6a analytics~~ **engine side landed 2026-08-02** (`orrery::analytics`); the slice owes service calls, DTOs, and room assignment | 06-app.md:17, ROADMAP | owned |
| Notification delivery + `pending_changes()` naming/semantics (read-shaped spec, write-shaped impl) | ROADMAP.md:20, muster/SPEC-02:22; review MO-5/L-9 | **pinned here** (was: no stage home) |
| "Full track" Beta gate: pre-commit fixture + acceptance criterion at slice entry | review MO-8 | **new** |

## Phase 6a — engine surfaces for the app ✅ closed 2026-08-02 (phases/06a-engine-surfaces.md)

*Created by review amendment 2026-08-02 (PLAN.md). All rows resolved in
the one slice; the Orrery Alpha exit gate is met at 10⁵ (release,
MemoryRepo — qualifications in the phase doc).*

| Item | Origin | Owner status |
|---|---|---|
| Non-persisting digest dry-run (expectation/membership overlay → change-set preview equal to post-commit `refresh_digests`) | review CR-2; muster/SPEC-02:18 | ✅ done — `Engine::preview_digests`, honesty property-tested against the real commit path (subgroup kind included for free); typed `PreviewUnsupported` otherwise |
| Anchor producer + anchor→first-event feasibility consult (ADR-0014 core feature; `Anchors` has no command and no storage today, so worlds-with-anchors fixtures are impossible — the slice-2 privacy test records this) | 04-travel.md:107 (expired "Phase 5/6"); review MO-4 | ✅ done — `AddAnchor` (Structure-tier enforced) + `anchors_for` + `first_event_feasibility` (verdicts only); demo world now anchored, privacy fixtures real. Sweep-side anchor violations stay a query surface → new Conditional row below |
| Analytics surface: engagement, capacity pressure, divergence, bounded 2-hop co-attendance | review CR-4; orrery/SPEC-02:68-73 | ✅ done — `analytics` module, all four oracle-tested; 2-hop measured p95 7.4 ms at 10⁵ (release) vs the 50 ms budget |
| Define the 10⁵ budget set (or restate Orrery Alpha gate at 10⁶) | review CR-4 | ✅ done — SPEC-03 dated addition (same thresholds, ×0.1 dimensions); all seven classes measured green → **Orrery Alpha gate met** |
| Attendance-model hook (orrery "○ hook" side): make the capacity-interest threshold an injectable strategy | ROADMAP.md:17; review (engine item 7) | ✅ done at primitive level — `capacity_pressure(.., interest_threshold)` caller-supplied; sweep default 0.0 unchanged; richer strategy objects remain SDK-owned |

## Phase 7 — hardening + ADR-0015 close (the dossier)

*~15 commitments now point here. The Phase-7 doc pre-commits against this
list at entry; anything dropped needs a written waiver (Rule 01.2).*

| Item | Origin |
|---|---|
| **Down-select 3→2 at entry** (Stage-B data + this dossier) before any repository implementation; 3 impls would need ADR-0021 superseded | review CR-5; ADR-0021 addendum 2026-08-02 |
| **External-writer invalidation posture** (salsa mirror single-writer) — ADR-0015 criterion 6 | review CR-1; ADR-0015 dated addition |
| `expired_membership_effect` producer decision: persisted derived cache, or re-scope/withdraw the detector by ADR | 03-engine-core.md:215 (expired); review CR-3 |
| Re-measure SQLite baseline on the decision host | 00-grounding.md:182; ADR-0015:46-47 |
| Cozo fork/vendoring readiness plan (legal precondition for selecting Cozo) | 01a-paper-screen.md:100 |
| Grafeo: report dual-anchor evaluator bug upstream; re-evaluate L-scale trend (≥10× on 2 of 3 at L); tier-constraint enforceability probe (dropped from 1b without waiver) | 01b:305-306; 01a:101; review MO-4 |
| Per-call parse/plan overhead: cached/prepared query paths in the Stage-C harness | 01b:307 |
| Result materialisation at 10⁵-row results | 01b:308 |
| Concurrency / mixed read-write / transactions / salsa interleaving through the real engine | 01b:309; PLAN.md Phase 7 |
| Cozo default feature set (`compact`) compile check | 01b:310 |
| Sweep performance vs orrery/SPEC-03 budgets at 10⁵–10⁶; closure refresh at 2k+ locations vs 60 s budget | 03:217; 04:109 |
| Sibling-rule common-parent refinement (deferred 3→4→5→here) | 04-travel.md:110 |
| Deterministic rebuild: design the rebuild **operation** (SPEC-04 addition), then verify | PLAN.md:116; review MO-9 |
| SDK Beta churn gate measurement (pre-commit instance class, scale, removal rule, seeds — MO-8) | 05-sdk.md:155 |
| Screening-harness disposal after mining | 01b:311 |
| GraphLite watch-list re-screen if development resumed | 01a:105 |

## Stage-entry pre-commitments owed (no phase doc yet — rows added to product PLANs 2026-08-02)

| Stage | Must define at entry | Origin |
|---|---|---|
| SDK Beta close | churn-gate instance definition (see Phase 7 row) | review MO-8 |
| SDK MVP | explain-assignment scope + "organiser accepts unedited" trial protocol | 05-sdk.md:158; review MO-8 |
| SDK RC | CP-SAT evaluate-or-reject **and the SDK perf gates the RC gate references (none exist today)** | 05-sdk.md:160; review MO-8 |
| Muster MVP | auth/tenancy/admin/location management scope; portal-cost import (bridged-pair under-estimate, from 04:106) | ROADMAP.md:68; 04-travel.md:106; review MO-5 |
| Muster RC | accessibility standard (name a level), ops-doc inventory, backup/restore design; Parquet/CSV egress (ADR-0015 consequence + Rule 09) with anchors excluded by default | ROADMAP.md:69; review MO-5 |
| Orrery Beta | API-freeze definition + diff tooling (cargo-public-api absent on this host — grep fallback is not an API-diff) | 02-workspace.md:126; review (gate check) |
| Orrery RC | docs-complete inventory | orrery/SPEC-05:60 |

## Conditional / unscheduled (tracked so they can't silently vanish)

| Item | Trigger | Origin |
|---|---|---|
| `Shift` move + free-start-time problem shape | a consumer needs it (spec'd as required: muster-sdk/SPEC-01:37 — spec vs phase-doc tension recorded) | 05-sdk.md:156 |
| Simulated annealing | instances demand escape from local optima (PRD FR-3 names it) | 05-sdk.md:157 |
| `CapacityHeadroom` objective term | later SDK slice | muster-sdk/SPEC-01:20, SPEC-02:40 |
| Most-constrained-first greedy ordering | room-compatibility constraints arrive | 05-sdk.md:89 |
| Severity-weight shared constant (engine ↔ SDK) | next engine API touch | 05-sdk.md:159 |
| Correlation ID per command | operability work (orrery/SPEC-03:54) | review |
| Sweep-side anchor violations (first/last-event infeasibility as a swept kind) | a `depart_not_before` policy source exists: mobility profiles (ADR-0017) or app-supplied day boundaries | 06a-engine-surfaces.md carry-forward |
| check-xrefs: also catch hyphenated bare `SPEC-NN` outside product dirs | next script touch | review L-7 |

## Owner touchpoint queue

| Question | Origin |
|---|---|
| What did "forGQL" refer to? (Confirm LoraGraph=LoraDB, IndraGraph=IndraDB stands recorded) | 01a-paper-screen.md:104 |
| Accept the Phase-7-entry down-select rule (2 impls), or direct 3 impls with ADR-0021 superseded? | review CR-5 |
