<!-- Imported from design thread 2026-08-01. Fable: review and improve. -->

# SPEC 05 — Orrery testing criteria

## Levels

**Unit.** Interval algebra: all thirteen Allen relations, boundary cases
(zero-length, adjacent, identical), fuzz against a reference implementation.

**Property.** Each detector against a brute-force oracle over generated
worlds. Invariants:

* A detector never reports a violation the oracle does not find, and vice versa
* Derived expansion is order-independent
* `derived_id` is stable across recomputation with unchanged inputs
* Effective priority is a pure function of the stack
* Two-row travel edges remain symmetric unless explicitly diverged

**Differential.** Where two repository implementations exist, run the full
canonical query set against both on identical data and assert equality. In the
spike this caught nothing because results agreed exactly — that agreement is
the evidence the harness is wired correctly, and it must be maintained.

**Incremental correctness.** Salsa-derived results must equal a cold
recomputation after an arbitrary mutation sequence. Fuzz the sequence.

**Benchmark.** The seven canonical queries at 10³ / 10⁵ / 10⁶ `attends` edges.
Interactive and batch classes tracked separately. Regression gate on the
budgets in SPEC 03.

## Seeded fixtures

Generated worlds must deliberately contain:

* time conflicts (overlapping `attends`)
* impossible travel (insufficient gap between distant rooms)
* location double-booking, including legitimate declared overflow
* expired memberships that must not contribute derived attendance
* a group hierarchy of depth ≥ 4 with a mid-chain expired `subgroup_of` edge
* locations hosting no events (intermediate travel nodes)
* over-capacity events
* **DST-crossing pairs (ADR-0024, added 2026-08-01):** a spring-forward pair
  whose wall-clock rendering misstates the true UTC gap (travel feasibility
  must judge the instants), and a fall-back pair whose wall-clock rendering
  suggests an overlap that does not exist in UTC (conflict detection must
  not fire). The original fixture list had none — all three QUESTION-0014
  failure modes would have shipped undetected.

The expired mid-chain edge is the critical one: it is what distinguishes true
per-hop temporal filtering from whole-path post-filtering.

## Release gates

| Stage | Gate |
|---|---|
| PoC | canonical queries run; per-hop filtering verified on the chosen store |
| Prototype | all detectors implemented; property tests green |
| Alpha | benchmarks within budget at 10⁵; two repository impls agree |
| Beta | benchmarks within budget at 10⁶; incremental correctness fuzz green |
| RC | privacy boundary tested; deterministic rebuild verified; docs complete |
