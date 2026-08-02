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

## Phase 6 slice 2 — Prototype (this slice)

| Item | Origin | Owner status |
|---|---|---|
| QUESTION-0015 frontend ADR (ADR-0025) + crates per its outcome | 06-app.md:74 | this slice |
| Browse/priority surfaces, full member flow, `SetPriority` service call | 06-app.md:74 | this slice |
| `RemoveAttendance` command + `deselect` service call (Flow A "resolve") — includes teaching `incremental::refresh_after` the new kind | review CR-6 | this slice (reshaped by review) |
| `select()` interactive latency measured (not optimised unless the number demands it) | 06-app.md:76 vs kickoff — conflict resolved: measure now | this slice |
| `figment` + tracing-subscriber when the first real config knob exists | 06-app.md:78 | this slice (expected) |
| Fix `just muster::e2e` (`--features e2e` flag, no such feature) | review MO-10 | this slice |

## Muster Alpha (Phase 6 slice 3) — pre-commitment must cover

| Item | Origin | Owner status |
|---|---|---|
| Coordinator flow: groups/expectations service calls replacing `engine_mut`, inbox + waive | 06-app.md:75 | owned (06-app later slices) |
| Blast-radius preview UI + honesty gate — **blocked by Phase 6a preview primitive** | 06-app.md:75, muster/SPEC-03:17-21 | owned; engine side → Phase 6a |
| Privacy tests extend to coordinator-facing DTOs on worlds with anchors | 06-app.md:77 | owned |
| `Warn` policy: define observable semantics (likely Detect + notification ChangeSet inclusion) or shrink the enum by ADR; document partial-`Prevent` (2 of 7 kinds) | 03-engine-core.md:218 (expired "Phase 6"); review MO-1 | **re-homed here** |
| Group-scoped violation query (`inbox(filter)`, "touching G") — repo query vs engine surface vs measured app-side join | muster/SPEC-01:29, SPEC-02:19; review MO-11 | **re-homed here** |
| Retraction commands beyond `RemoveAttendance` (membership/expectation/hold end-or-shorten) | review CR-6 | **new** |
| Severity defaults product confirmation (expired "Muster PoC feedback") | 03-engine-core.md:123; review MO-4 | **re-homed here** — coordinator flow is where severity is felt |

## Muster Beta (Phase 6 slice 4)

| Item | Origin | Owner status |
|---|---|---|
| Capacity/engagement/divergence surfaces + room assignment — **blocked by Phase 6a analytics** | 06-app.md:17, ROADMAP | owned |
| Notification delivery + `pending_changes()` naming/semantics (read-shaped spec, write-shaped impl) | ROADMAP.md:20, muster/SPEC-02:22; review MO-5/L-9 | **pinned here** (was: no stage home) |
| "Full track" Beta gate: pre-commit fixture + acceptance criterion at slice entry | review MO-8 | **new** |

## Phase 6a — engine surfaces for the app (new; parallel with Phase 6; orrery work)

*Created by review amendment 2026-08-02 (PLAN.md). Own phase doc with
pre-committed hypotheses required at entry. Sequencing: preview primitive
before the Muster Alpha slice opens; analytics before the Muster Beta
slice.*

| Item | Origin | Notes |
|---|---|---|
| Non-persisting digest dry-run (expectation/membership overlay → change-set preview equal to post-commit `refresh_digests`) | review CR-2; muster/SPEC-02:18 | the honesty gate is already spec'd (muster/SPEC-03:17-21) |
| Analytics surface: engagement, capacity pressure, divergence, bounded 2-hop co-attendance | review CR-4; orrery/SPEC-02:68-73 | 2-hop has a pre-committed budget (<50 ms p95, orrery/SPEC-03:14) |
| Define the 10⁵ budget set (or restate Orrery Alpha gate at 10⁶) | review CR-4 | Orrery Alpha is unexitable as written |
| Attendance-model hook (orrery "○ hook" side): make the capacity-interest threshold an injectable strategy | ROADMAP.md:17; review (engine item 7) | SDK impl side stays SDK-owned |

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
| check-xrefs: also catch hyphenated bare `SPEC-NN` outside product dirs | next script touch | review L-7 |

## Owner touchpoint queue

| Question | Origin |
|---|---|
| What did "forGQL" refer to? (Confirm LoraGraph=LoraDB, IndraGraph=IndraDB stands recorded) | 01a-paper-screen.md:104 |
| Accept the Phase-7-entry down-select rule (2 impls), or direct 3 impls with ADR-0021 superseded? | review CR-5 |
