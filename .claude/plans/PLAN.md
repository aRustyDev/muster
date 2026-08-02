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
| Implementation | Phases 2, 1b, 3, 4 complete — Orrery Prototype + travel; Phase 5 (SDK) next |

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

### Phase 7 — Hardening and the ADR-0015 decision
Repository implementations for both finalists. Differential testing against
`MemoryRepo` and each other. Benchmark **through the real engine** — mixed
read/write, transactions, salsa interleaving, realistic result-set sizes, none
of which the Phase-0 harness exercised. Privacy boundary tests. Deterministic
rebuild verification.

**Close ADR-0015 here**, against the criteria pre-committed in Phase 1a.

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
