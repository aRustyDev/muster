# Phase 3 artifact — Architecture at the Orrery Prototype gate

*State as of 2026-08-02, Phase 3 complete (phases/03-engine-core.md). Sibling
artifacts: [incremental derivation](phase-3-incremental-derivation.md) ·
[violations & oracle](phase-3-violations-and-oracle.md).*

Orrery is a **spatiotemporal feasibility engine**: given people assigned to
events at locations over time, it returns the ways that assignment is
impossible, and a score for how good it is. It does not schedule — deciding
*whether* a schedule is possible is the whole job (searching for one is
Muster-SDK's job, presenting it is Muster's).

## The workspace

```mermaid
flowchart LR
    subgraph apps [consumers]
        MUSTER["crates/muster\n(app: UI, auth, delivery)\nPhase 6 — stub"]
        SDK["crates/muster-sdk\n(solvers, objectives, batches)\nPhase 5 — stub"]
    end
    ORRERY["crates/orrery\nthe engine — Prototype reached"]
    SDK --> ORRERY
    MUSTER --> SDK
    MUSTER --> ORRERY
```

Rule 03's test: if trying a different solver would require changing
`orrery`, the boundary is broken. The engine exposes `is_feasible` and
`score`; how a caller searches the space is not its concern.

## Inside the engine

```mermaid
flowchart TB
    subgraph orrery [crates/orrery modules]
        MODEL["model\nid newtypes · entities · relations\n(every relation carries a validity window)"]
        INTERVAL["interval\nTimestamp (µs UTC) · half-open Interval\n13 Allen relations · THE overlap predicate"]
        TIER["tier\ncontainment/traverse legality (ADR-0009)"]
        CMD["command\nCommand enum — the ONLY write path"]
        REPO["repo\nRepository trait (sync, ADR-0023)\n+ MemoryRepo (Rule 00b enforcement)"]
        DERIVE["derive\nQ1 expansion · DerivedId (blake3)\neffective_schedule"]
        INC["incremental\nsalsa World mirror · digest chain\nearly cutoff"]
        DETECT["detect\n7 pure detectors + oracles\nPolicy map"]
        ENGINE["engine\nEngine: apply gate · sweeps ·\nrefresh_digests · FeasibilityOracle"]
    end
    ENGINE --> CMD & REPO & DERIVE & INC & DETECT
    DERIVE --> REPO
    CMD --> TIER
    DETECT --> INTERVAL
    MODEL --> INTERVAL
```

## The write path — one chokepoint

Every mutation, including test seeding and violation records, is a `Command`
variant (Rule 00.2). That single enum is what makes the future event log
(ADR-0016 D) an insertion rather than a refactor: serialize the enum, done.

```mermaid
sequenceDiagram
    participant C as Caller (SDK / app / test)
    participant E as Engine
    participant D as detect (same fns as sweeps)
    participant R as Repository
    participant S as salsa World mirror

    C->>E: apply(Command::AddAttendance{...})
    alt policy(TimeConflict) == Prevent
        E->>D: time_conflict::detect(prospective state)
        D-->>E: drafts?
        alt drafts non-empty
            E-->>C: Err(CommandRejected "prevented: ...")
        end
    end
    E->>R: apply(cmd)  — MemoryRepo: single writer, reads fail during write
    R-->>E: CommandReceipt{seq}
    E->>S: refresh mirrored field (only if cmd touches a fact class)
    E-->>C: receipt
```

## Worked example — the two personas, one engine

The PRD stresses one engine with two opposite workloads:

* **Academic advising** (rigid): student `p` self-selects CS-101; their
  advisor's cohort group carries an expectation for the mandatory seminar.
  `effective_schedule(repo, p, window, at)` returns the explicit CS-101 edge
  plus a *derived* seminar edge with provenance (`source_group`), and the
  conflict detector flags the overlap between them — same interval
  predicate as everything else.
* **Conference planning** (soft): an organiser drags a talk into an occupied
  room. Nothing stops the drag — detection, not prevention (Rule 00.4) —
  but the next `is_feasible` overlay evaluation returns the
  `LocationExclusivity` violation and the score drops by its severity
  weight. Local search *needs* to traverse exactly these infeasible states.

## Non-negotiables, as they exist in code today

| Rule 00 item | Where it is enforced |
|---|---|
| Persistence behind a trait | `repo::Repository`; seam check greps the public API for datastore names |
| One command layer | `command::Command`; repositories expose no other write |
| Every relation carries a window | `model` — every relation struct has `during`; `Interval::new` rejects inversions |
| Detection, not prevention | `detect` default policy; `Prevent` is per-kind opt-in running the same detector |
| `feasible(person, …)` signature now | `impossible_travel::detect(person, …)` carries and ignores it; full landing in Phase 4 |
| Anchors never cross the boundary | `Anchors` doc-contract + Rule 09; automated privacy test lands with Phase 4 travel |
| Solver lives in the SDK | `orrery` has no search code and no `rand` |

## Verification posture

50 tests: 15 property suites (interval algebra, all 7 detectors, derivation,
incremental-vs-cold fuzz) each against an independently-written naive
oracle; DST fixtures with real 2026 transition instants (ADR-0024); early
cutoff asserted by execution counters, not assumed; Rule-00b constraint
errors asserted by name. Gates: `cargo nextest`, `clippy -D warnings`,
`fmt`, seam grep, xref audit.

## Datastore status (the one open decision)

ADR-0015 stays `proposed`. Funnel: Stage A screened 20 candidates → Grafeo,
agdb, Cozo (owner risk-acceptance); Stage B (Rust harness) eliminated
nobody at M scale but recorded qualitative findings against Grafeo (an
evaluator bug, no single-statement Q1, a planner cliff, 600× join gap).
Stage C / Phase 7 decides through real repository implementations against
the pre-committed ADR-0015 criteria. Everything above runs on `MemoryRepo`,
which deliberately enforces the *most restrictive* surviving-candidate
constraints so the trait can't silently absorb permissive-store assumptions.
