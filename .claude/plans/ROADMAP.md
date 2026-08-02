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
