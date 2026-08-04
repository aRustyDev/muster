<!-- Imported from design thread 2026-08-01. Fable: review and improve. -->

# 21. Datastore selection is a funnel, and only the paper screen blocks

* Status: accepted
* Date: 2026-08-01
* Amends: ADR-0015

## Context and Problem Statement

ADR-0015 left the datastore open. The initial plan made "resolve ADR-0015" a
single blocking phase, which would have (a) held all implementation behind a
research task and (b) required a standalone benchmark harness duplicating query
work the engine must implement anyway.

## Decision Drivers

* `orrery/SPEC-03` and `orrery/SPEC-05` **already require two repository implementations** for
  differential testing and to prove the persistence seam is real. That work is
  committed regardless of how the datastore decision is reached.
* Screening candidates is cheap and disposable; implementing a repository is
  expensive and permanent. These should not be the same activity.
* A trait designed against a permissive store accretes assumptions a restrictive
  store cannot satisfy.

## Decision Outcome

Selection proceeds as a three-stage funnel. **Only Stage A blocks
implementation.**

| Stage | Cost | Purpose | Blocking |
|---|---|---|:--:|
| **A — paper screen** | hours | eliminate on hard requirements (RESEARCH-0005) | **yes** |
| **B — screening harness** | ~1 day/candidate | eliminate order-of-magnitude losers | no |
| **C — repository impls** | already committed | decide between 2 finalists | no |

Stage A blocks because a negative result there is architectural, not
performance-related: if no candidate supports per-hop edge filtering in
recursive patterns, Q1 must be restructured, and that changes the engine.

Stage B is throwaway. **Written in Rust**, not Python — Stage A already requires
first-class Rust bindings, so the harness costs little extra and doubles as a
bindings smoke test.

Stage C is the differential-testing infrastructure `orrery/SPEC-03` already mandates.

### First implementation is in-memory

Phase 2 lands `MemoryRepo` before any datastore code. It:

* unblocks Phases 2–5 with zero dependency on ADR-0015
* serves as the differential-testing oracle
* **deliberately enforces the most restrictive constraints across all
  candidates** — single writer, no concurrent read-during-write, no cross-hop
  predicates in traversal

The last point is the actual risk control. Building against a permissive store
(SQLite WAL: concurrent readers, arbitrary SQL) for months would silently
produce a trait that a restrictive store (Ladybug: one `READ_WRITE` process,
never mixed with readers) cannot back. An over-constrained in-memory
implementation prevents that more cheaply than either real store.

## Consequences

* Implementation starts after a hours-long paper screen, not a multi-day
  benchmark effort.
* Three repository implementations total; the first is nearly free.
* **Deferral risks, named with mitigations:**
  * *Seam leakage* → `MemoryRepo` enforcing the restrictive intersection.
  * *Sunk cost after committing to store A* → acceptance criteria pre-committed
    in ADR-0015 before any implementation begins.
  * *Decision never made* — the likeliest failure; "it works, why change" is a
    powerful default → **the second repository implementation is a hard gate on
    Orrery Beta.** The decision has a stage attached, not a date.
* Benchmarking through the real engine measures realistic workload shape —
  mixed read/write, transactions, salsa interleaving, and realistic result-set
  sizes — none of which the Phase-0 harness exercised.

## Addendum (2026-08-02 — Stage B outcome vs the "2 finalists" plan)

Stage B closed with **no eliminations**: all three Stage-A survivors
(Grafeo, agdb, Cozo) advanced, because none hit the pre-committed
order-of-magnitude criterion (phases/01b-screening.md). This document's
"decide between 2 finalists" and "three repository implementations total"
arithmetic therefore no longer describes reality unaided.

**Resolution:** Stage C opens with an explicit **down-select from 3 to 2**,
decided against the Phase-7 dossier (`CARRY-FORWARD.md`: Stage-B
quantitative profiles, grafeo's qualitative findings and L-scale trend,
Cozo's dormancy/fork-readiness plan) before any repository implementation
begins. This preserves the cost bound above. Implementing all three
(four repositories total including `MemoryRepo`) would invalidate that
arithmetic and requires superseding this ADR, not a commit message.
The owner may of course direct the latter; the queue item is recorded in
`CARRY-FORWARD.md`.
