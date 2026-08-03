# Orrery — Plan (overall)

This is the overall plan; per-product derivations live in
`{orrery,muster-sdk,muster}/PLAN.md` and link back here for cross-product
sequencing (never duplicate — plans/README.md).

> Placement: this and `ROADMAP.md` sit at the `plans/` root because the
> roadmap's job is drawing boundaries *between* the three products.

## Where this came from

A design thread that produced 22 ADRs, 15 questions, and 5 research documents
*(corrected 2026-08-01: the handoff PROMPT/PLAN said "20/13/4", a count drift
against the package's own MANIFEST — see phases/00-grounding.md)*, including a
working benchmark harness that ran against a real embedded graph database and
SQLite at 1M edges. **Nothing here is assumed — the numbers are reproducible
from `evidence/`.**

Three hypotheses were refuted during that work. Expect more.

## Current state

| Artefact | Status |
|---|---|
| Domain model | settled — ADRs 0002–0014, 0017–0019 |
| Violation model | settled — ADR-0012 |
| Change detection | settled — ADR-0016 |
| Product boundaries | settled — ADR-0019 |
| Cascade analytic | withdrawn — ADR-0020 |
| **Datastore** | **OPEN — ADR-0015**; funnel: Stage A complete (phases/01a), Stage B complete (phases/01b — no eliminations; all three to Stage C; qualitative findings against grafeo) |
| Rust graph landscape | surveyed — RESEARCH-0005 Stage A findings + 20 scorecards |
| Repository trait shape | sync — ADR-0023 |
| Implementation | Phases 2, 1b, 3, 4, 5, **6a** complete; Phase 6 slices 1–2 done — **Muster Prototype met**; **Orrery Alpha gate met at 10⁵** (2026-08-02, phases/06a — release/MemoryRepo qualifications recorded); next: Muster Alpha slice (Phase 6 slice 3 — engine side now unblocked) |

## Phases

**Execution order is 0 -> 1a -> 2 -> 3 -> 4 -> 5 -> 6 -> 7.** Phase 1b runs
*alongside* Phases 3-5 and blocks nothing; it is numbered 1b because it belongs
to the datastore workstream, not because it runs second. Only Phase 1a blocks.

```
0 --> 1a --> 2 --> 3 --> 4 --> 5 --> 6 --> 7 (ADR-0015 closes)
                    \____ 1b (parallel, non-blocking) ____/
```


### Phase 0 — Ground and verify
Read every artefact. **Do not treat any as correct.** Re-derive the key claims
from `evidence/`. Specifically re-run `probe_01_recursive.py`,
`orrery_spike.py`, and `sqlite_compare.py` and confirm the numbers reproduce.

Output: `phases/00-grounding.md`, listing anything that fails to reproduce.

### Phase 1a — Paper screen (BLOCKING, hours not days)
Execute the Stage-A screen in RESEARCH-0005. Enumerate Rust-compatible embedded
graph datastores, verify each exists and is maintained, screen on the five hard
requirements. **No benchmarking.**

This blocks because a negative result is architectural, not performance-related:
if nothing supports per-hop edge filtering in recursive patterns, Q1 must be
restructured and the engine changes.

Output: `phases/01a-paper-screen.md` listing survivors and eliminations with
reasons. Target: 2-4 survivors. **Then proceed immediately to Phase 2.**

### Phase 2 — Workspace, seams, and MemoryRepo
Three-crate workspace. Repository trait, command layer, model types, interval
algebra, and **`MemoryRepo`** — an in-memory repository implementation.

`MemoryRepo` deliberately enforces the **most restrictive constraints across all
surviving candidates**: single writer, no concurrent read-during-write, no
cross-hop predicates in traversal. This is the risk control for deferring
ADR-0015. A trait designed against a permissive store accretes assumptions a
restrictive store cannot satisfy; an over-constrained in-memory implementation
prevents that more cheaply than either real store.

Output: `phases/02-workspace.md`, compiling workspace, `MemoryRepo`, interval
algebra with property tests.

### Phase 1b — Screening harness (NON-BLOCKING · parallel with Phases 3-5)
Port the Phase-0 harness **to Rust** for surviving candidates. Throwaway by
design; purpose is eliminating order-of-magnitude losers, not picking a winner.
Rust rather than Python because Stage A already required first-class Rust
bindings, so it costs little extra and doubles as a bindings smoke test.

Output: `phases/01b-screening.md`, 2 finalists.

> *Amended 2026-08-02 (plan review): Stage B closed with **no eliminations —
> all three candidates advanced** (phases/01b). The 3→2 narrowing now happens
> at Phase-7 entry as an explicit down-select; see the ADR-0021 addendum and
> the Phase 7 section below.*

### Phase 3 — Detectors and derivation
Every detector as a pure function with a brute-force property-test oracle.
Derived expansion with per-hop temporal filtering. Salsa incrementality. Digests.

Output: `phases/03-engine-core.md`, Orrery Prototype.

### Phase 4 — Travel
Layer-1 network, `petgraph` closure computation, Layer-2 cache, feasibility
check with the `feasible(person, e1, e2)` signature landed but ignored.

Output: `phases/04-travel.md`.

### Phase 5 — SDK
Greedy assignment, objective composition, local search, batch orchestration.
Verify greedy optimality on fixed-start-time instances against brute force.

Output: `phases/05-sdk.md`, Muster-SDK Prototype.

### Phase 6 — Application
Muster surfaces in dependency order: member self-selection → coordinator groups
→ violation inbox → analytics → room assignment.

Output: `phases/06-app.md`.

### Phase 6a — Engine surfaces for the app (parallel with Phase 6)
*Added 2026-08-02 (plan review CR-2/CR-4): this work was promised by the
ROADMAP (orrery "○ compute" preview; Orrery Alpha analytics) but owned by no
phase.* Orrery-side work Muster's later slices consume, run alongside
Phase 6 the way 1b ran alongside 3–5:

1. **Non-persisting digest dry-run** (expectation/membership overlay →
   change-set preview; must equal post-commit `refresh_digests` — the
   muster/SPEC-03 honesty gate). Blocks the Muster **Alpha** slice.
2. **Analytics surface**: engagement, capacity pressure, divergence,
   bounded 2-hop co-attendance (orrery/SPEC-02 FRs; 2-hop budget
   pre-committed in orrery/SPEC-03). Blocks the Muster **Beta** slice.
3. Define the 10⁵ budget set the Orrery Alpha gate references (or restate
   the gate at 10⁶) — the stage is unexitable as currently written.
4. *(Added 2026-08-02 during slice-2 implementation)* Anchor producer +
   anchor→first-event feasibility consult (ADR-0014's core feature —
   `Anchors` currently has no command and no storage, which also blocks
   worlds-with-anchors privacy fixtures).

Output: `phases/06a-engine-surfaces.md` (pre-committed at entry, per
convention).

*Complete 2026-08-02: all four items delivered (preview honesty
property-tested against the real commit path; analytics oracle-tested;
10⁵ budget set defined in SPEC-03 and measured green — the Orrery Alpha
exit gate is met, release/MemoryRepo qualifications in the phase doc;
anchor producer + consult landed, worlds-with-anchors privacy fixtures
now real).*

### Phase 7 — Hardening and the ADR-0015 decision
Repository implementations for both finalists. Differential testing against
`MemoryRepo` and each other. Benchmark **through the real engine** — mixed
read/write, transactions, salsa interleaving, realistic result-set sizes, none
of which the Phase-0 harness exercised. Privacy boundary tests. Deterministic
rebuild verification.

**Close ADR-0015 here**, against the criteria pre-committed in Phase 1a.

*Amended 2026-08-02 (plan review): Phase 7 additionally opens with an
explicit **down-select from the three Stage-C survivors to two finalists**
(ADR-0021 addendum) before any repository implementation, and its
pre-commitment must cover the full dossier consolidated in
`CARRY-FORWARD.md` (Phase 7 section) — notably the **external-writer
cache-invalidation posture** (ADR-0015 criterion 6: the salsa mirror
invalidates only through its own `Engine`; a second writer process silently
desynchronises digests from detection), the `expired_membership_effect`
producer decision (build the persisted derived cache, or re-scope/withdraw
the detector by ADR), the deterministic rebuild **operation** design (only
its verification was named here before), and the Cozo fork-readiness plan.
Dropping any dossier item requires a written waiver (Rule 01.2).*

Output: `phases/07-hardening.md`, ADR-0015 `accepted` or `rejected`.

## Working method

**Experiment-oriented.** Every non-obvious decision gets a falsifiable
hypothesis, a measurement, and a written result — including when the result
refutes the hypothesis. The design thread refuted three of its own strongest
claims; that rate is normal and should not be suppressed.

Pre-commit acceptance criteria **before** running any spike. The Phase-4 gate in
the source thread was committed in advance and one criterion still turned out to
be mis-specified — record that rather than quietly dropping it.

Ship working software early. Prefer a narrow vertical slice over a broad
horizontal layer.
