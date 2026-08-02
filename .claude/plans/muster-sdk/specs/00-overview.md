<!-- Imported from design thread 2026-08-01. Fable: review and improve. -->

# SPEC 00 — Muster-SDK overview

Search and orchestration over Orrery's contract. Consumes
`is_feasible` / `score`; never redefines them.

## Module map

| Module | Responsibility |
|---|---|
| `assign` | room-assignment strategies |
| `objective` | weighted soft-constraint terms and composition |
| `search` | local-search moves, annealing schedule, anytime interrupt |
| `batch` | sweeps, closure refresh, digest recomputation |
| `notify` | change-set computation from digests |
| `explain` | which constraints bind, what a move would cost |

## Strategy tiers

| Tier | Approach | When |
|---|---|---|
| 0 | Greedy, most-constrained-first | always available; provably optimal for fixed start times |
| 1 | Greedy seed + local search | default for heterogeneous rooms or clustering objectives |
| 2 | CP-SAT | deferred; large or baroque instances only |

## Complexity note

With fixed start times and room assignment only, this is **interval graph
colouring**: polynomial, and greedy-by-left-endpoint achieves the chromatic
number because interval graphs are perfect. NP-hardness enters with free start
times, heterogeneous room compatibility, or clustering objectives.

Tier 0 is therefore not a placeholder — it is optimal for a real and common
case, and must be recognised as such rather than always deferring to search.
