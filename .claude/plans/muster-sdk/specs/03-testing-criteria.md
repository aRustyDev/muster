<!-- Written 2026-08-02 at Phase 5 entry — filling the "known thin spot"
     plans/README records for muster-sdk. -->

# muster-sdk/SPEC-03 — testing criteria

## Property tests (the load-bearing ones)

* **Optimality (`optimality_` prefix — `just muster_sdk::test-optimality`)**:
  on fixed-start-time instances (n ≤ 12 events, k ≤ 3 rooms), greedy finds
  a complete zero-exclusivity assignment **iff** brute-force enumeration
  finds one, and **iff** max instantaneous overlap ≤ k (two independent
  oracles — interval graphs are perfect, so all three must agree).
* **Monotone (`monotone_` prefix — Alpha)**: local search never returns a
  worse-scoring assignment than its seed.
* **Stability (Beta)**: re-solve after removing one room changes < 10% of
  assignments (ROADMAP gate).
* **Breakdown additivity**: `total == Σ weight×cost` on arbitrary term sets;
  evaluation is deterministic.

## Boundary tests

* **Scope guard**: `just muster_sdk::check-scope` fails the build if a
  UI/server dependency enters the tree (QUESTION-0015).
* **No feasibility semantics**: SDK sources construct no `ViolationKind`
  and contain no interval-overlap re-implementation — violations arrive
  from the engine only. *(Automated 2026-08-03, QR-2 SDK-3: `just
  muster_sdk::check-oneway` fails the build on a match — this replaces
  the manual "source grep at phase close". Scope is `src/` only, by
  design: `tests/` contains the brute-force oracle, which this spec
  requires to independently re-implement overlap, and assertions that
  match on engine-produced kinds — consuming, not constructing. The
  gate's own first run surfaced this scope question: QF slice, QF-R2.)*
* **Determinism**: fixed inputs (and, later, fixed RNG seed) → identical
  `Suggestion` on re-run, asserted by in-memory equality. *(Corrected
  2026-08-03, QR-2 SDK-1 / review C16: this line used to promise
  "byte-for-byte on serialised output" — unimplementable as written,
  since the crate has no serde and nothing serialises a `Suggestion`.
  The test asserts what exists: in-memory equality. A true
  serialized-determinism test becomes a Muster-Alpha row where
  suggestions first cross a wire, in muster-server.)*

## Integration

* `suggest_room_schedule` against `Engine<MemoryRepo>`: placements land,
  an externally-seeded conflicting hold surfaces as a `ViolationCost` row
  and in `Suggestion.violations`; `unassigned` is populated (never
  silently dropped) when rooms run out.

## Release gates (ROADMAP)

| Stage | Gate |
|---|---|
| PoC | greedy matches brute-force optimum, n ≤ 12 |
| Prototype | assignment + breakdown returned |
| Alpha | local search improves on greedy for heterogeneous rooms |
| Beta | re-solve changes < 10% for one room removal |
| MVP | explain-assignment; organiser accepts a suggestion unedited |
| RC | CP-SAT integrated **or** rejection documented |

*(MVP row added 2026-08-02 — it existed in the ROADMAP but not here. The
Beta churn gate and the MVP/RC gates still need pre-committed definitions
— instance class, scale, trial protocol, and the SDK perf gates the RC
gate references (none exist yet): CARRY-FORWARD.md, review MO-8.)*

*(Perf-gate referent added 2026-08-03, QR-2 SDK-2 / review F-3: the RC
"perf gates green" gate is defined as suggest/search wall-time budgets on
Beta-scale instances. The budgets themselves are set when RR&P-2 closes
(harness pick + baselines, `plans/quality-review/02-additions-and-order.md`)
and are owed green at **Muster Beta** — the gate now has an owner and a
mechanism-to-be instead of referencing nothing. Churn/stress scale
definition (SDK-5) is pre-committed at Muster-Beta entry. Observability:
the sdk span table lives in Rule 05 as of 2026-08-03.)*
