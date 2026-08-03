# Orrery — Roadmap

## Product boundaries

| Capability | Orrery (lib) | Muster-SDK | Muster (app) |
|---|:--:|:--:|:--:|
| Entities, relations, validity windows | ● | | |
| Interval algebra | ● | | |
| Violation detectors + policy toggle | ● | | |
| Derived expansion + priority stack | ● | | |
| Travel Layer 1/2, feasibility | ● | | |
| Engagement, capacity, divergence analytics | ● | | |
| Bounded 2-hop co-attendance | ● | | |
| Repository trait + command layer | ● | | |
| Greedy / local search / CP-SAT | | ● | |
| Objective composition, stability term | | ● | |
| Attendance-model hook | ○ hook | ● impl | |
| Batch orchestration, digest recompute | | ● | |
| Change-set computation | | ● | |
| Notification **delivery** | | | ● |
| UI, auth, tenancy | | | ● |
| Blast-radius preview | ○ compute | ● orchestrate | ● present |
| Waiver workflow | ○ record | | ● UI |

● owns · ○ provides primitive

## Stage definitions

| Stage | Meaning |
|---|---|
| **PoC** | proves the risky thing works at all; throwaway code acceptable |
| **Prototype** | end-to-end shape with a real API; no hardening |
| **Alpha** | feature-complete for core scope; breaking changes expected |
| **Beta** | API frozen; external users; bugs expected |
| **MVP** | minimum shippable solving a real user's real problem |
| **RC** | production-ready; docs complete; no known blockers |

## Orrery (lib)

| Stage | Contents | Exit gate |
|---|---|---|
| PoC | paper screen closed; `MemoryRepo` + repository trait + command layer | canonical queries run against `MemoryRepo` |
| Prototype | model, interval algebra, all detectors, derived expansion | property tests green vs. brute-force oracle |
| Alpha | salsa incrementality, digests, travel Layer 1/2, analytics | budgets met at 10⁵ edges |
| Beta | API frozen, **both finalist repository impls**, differential tests | budgets met at 10⁶; incremental fuzz green; **ADR-0015 closed** |
| MVP | whatever Muster MVP requires, nothing more | Muster MVP ships on it |
| RC | privacy boundary tested, deterministic rebuild, docs | all `orrery/SPEC-05` gates pass |

*(Gate definition added 2026-08-03, quality review O-5/F-7: until RR&P-3
closes, the Beta gate "incremental fuzz green" means **the
`prop_incremental` family green at its documented case budget** — the
corpus's "fuzz" has always meant proptest, and no document said which
tests constitute the gate. This is an honest narrowing, not a new
promise; RR&P-3 (coverage-guided fuzzing viability on this host) redefines
the gate before Orrery-Beta entry — Rule 01.2 either way.)*

## Muster-SDK

| Stage | Contents | Exit gate |
|---|---|---|
| PoC | greedy assignment on fixed start times | matches brute-force optimum, n ≤ 12 |
| Prototype | objective composition, violation-cost term | assignment + breakdown returned |
| Alpha | local search, stability term, expected-attendee-travel | improves on greedy for heterogeneous rooms |
| Beta | batch orchestration, digests, change sets, anytime | re-solve changes < 10% for one room removal |
| MVP | explain-assignment | organiser accepts a suggestion unedited |
| RC | CP-SAT **or** documented rejection | perf gates green |

## Muster (app)

| Stage | Contents | Exit gate |
|---|---|---|
| PoC | one member self-selects; conflict appears | conflict visible end to end |
| Prototype | browse, select, priority, my-schedule with provenance | member flow complete |
| Alpha | groups, expectations, blast-radius preview, violation inbox | coordinator flow complete |
| Beta | capacity, engagement, divergence, room assignment | full track scheduled end to end |
| MVP | auth, admin, location management | a real coordinator uses it unaided |
| RC | accessibility, ops docs, backup/restore | privacy assertions automated and green |

## Dependency graph

```
ADR-0015 paper screen (Phase 1a, hours)
   └─> Orrery PoC (MemoryRepo)
        └─> Orrery Prototype ──> Orrery Alpha ──> Orrery Beta ──> Orrery RC
                 │                    │                │
                 v                    v                v
            SDK PoC ──────────> SDK Prototype ──> SDK Alpha ──> SDK Beta
                 │                                     │
                 v                                     v
            Muster PoC ──> Prototype ──> Alpha ──> Muster Beta ──> MVP
```

*(Diagram corrected 2026-08-02: the SDK-Alpha arrow lands on Muster **Beta**
— room assignment — matching the hard-deps list below; it previously pointed
at Muster Prototype, which needs no SDK at all.)*

Hard dependencies:

* **Only the paper screen blocks** (ADR-0021). `MemoryRepo` unblocks Phases 2-5
  entirely. The datastore decision closes at Orrery Beta — a **stage gate, not a
  date**, because the likeliest failure mode is the decision never being made
  once something works.
* SDK Prototype needs Orrery's feasibility oracle **and** scoring — not just
  detectors.
* Muster Alpha needs derived expansion **and** blast-radius computation.
* Blast-radius preview needs salsa early cutoff; without it the preview is a full
  recompute and too slow for interactive use.
* Room assignment in Muster Beta needs SDK Alpha local search.
* *(Added 2026-08-02, plan review)* **Muster Alpha needs the non-persisting
  digest dry-run and Muster Beta needs the engine analytics surface — both
  are orrery work owned by Phase 6a (PLAN.md), which therefore blocks those
  Muster slices.** Neither dependency was recorded here before, and the
  analytics one was contradictorily placed (Alpha in muster/SPEC-02:21,
  Beta everywhere else — now aligned to Beta).
* *(Added 2026-08-02, plan review)* Several unbuilt-stage exit gates are not
  yet measurable as written (SDK Beta churn instance, SDK RC perf gates,
  Muster Beta "full track", both MVP human-outcome gates). Each stage's
  entry pre-commitment must define its gate before work starts —
  the debts are itemised in `CARRY-FORWARD.md`.

## Quality-strategy ordering and semver *(added 2026-08-03, QR-3 — mirrors `quality-review/02-additions-and-order.md` §D.2/D.3, which holds the full item lists and edge citations)*

Accepted quality work lands in dependency-honest tranches (ordering rules
D1–D5: infrastructure before consumers · measurement before optimization ·
surfaces before their tests · ADR-0021 funnel discipline · RR&P closes one
slice before its implement items):

* **QF** (done 2026-08-03, merged `--no-ff`): broken doors fixed,
  cargo-deny/cargo-hack adopted, variance policy applied, boundary gates
  hardened, error→status + wire names pinned. **L** (this landing):
  the documentary amendments, dated 2026-08-03.
* **A — Muster Alpha entry**: M-1..M-6, T-1/T-2, O-1/O-2-design;
  RR&P-7 (validation) and RR&P-8 (UI testing) close at the
  pre-commitment. **B — concurrent with Alpha**: RR&P-1 (CI bring-up;
  GitHub Actions owner-confirmed) and RR&P-2's local leg (bench harness
  pick + MemoryRepo-only skeleton).
* **CI** (after RR&P-1): deny CI wiring, regression gates, coverage
  aggregation, release-please activation, Linux miri/sanitizer legs.
  **M**: RR&P-4 (coverage) → RR&P-5 (mutation, informational on orrery
  before Beta freeze).
* **PB — before Orrery Beta**: RR&P-3 (fuzzing) redefines the fuzz gate,
  RR&P-6 (API-diff tooling), O-3 (memory growth), O-4 (rustdoc examples),
  SDK-2 budgets defined. **MB**: RR&P-9 (load harness) + SDK-5 at the
  Muster-Beta pre-commitment. **P7**: everything datastore-shaped (O-7
  bundle). **RC**: M-8 egress test, O-2 e2e sweep, I5 ops-validation rows.

Semver (soft estimates, Rule 01.4 — one workspace version, release-please
over Conventional Commits, **inactive until RR&P-1 provides a runner**;
recommendation: baseline-tag at activation): quality work itself is
version-invisible (`test:`/`chore:`). Muster Alpha ≈ **0.2.0** · Orrery
Beta + Phase 7 ≈ **0.3.0** · Muster Beta ≈ **0.4.0** · MVP ≈ **0.5.0** ·
RC ≈ 0.6.x → **1.0.0** at RC exit.

## Deferred

| Item | Stage | Reason |
|---|---|---|
| `transit` scheduled travel | post-RC | breaks the Layer-2 scalar cache; needs time-bucketed profiles (ADR-0007) |
| Mobility profiles | post-MVP | signature lands now, implementation later (ADR-0017) |
| Event log as SoR | post-RC | command layer preserves the path (ADR-0016 D) |
| CP-SAT | SDK RC or rejected | OR-Tools Rust binding maturity unverified |
| Calendar sync | post-MVP | not on the critical path |
| Attendance forecasting | indefinite | requires actuals to calibrate (ADR-0018) |
| Cascade analysis > 2 hops | reopen only on evidence | saturates (ADR-0020) |
