<!-- Imported from design thread 2026-08-01. Review, do not assume correct. -->

# 20. Unbounded cascade analysis is withdrawn as a requirement

* Status: accepted
* Date: 2026-08-01

## Context and Problem Statement

"Cascade blast radius" — *if I move this seminar, what is the full set of
downstream affected events and people?* — was identified as the analytic that
most justified a graph database, being unbounded-depth traversal with no
natural termination.

## Decision Outcome

**Withdrawn.** Measured reachability from one person in a 2,000-person,
2,000-event graph:

| depth | reached | % of graph | latency |
|---:|---:|---:|---:|
| 1 | 50 | 1.2% | 25 ms |
| 2 | 1,511 | 37.8% | 11 ms |
| 3 | 3,461 | 86.5% | 120 ms |
| 4 | 4,000 | **100.0%** | **19,578 ms** |

The co-attendance graph saturates by hop 3. At depth 4 the "blast radius" is
*everyone* — a vacuous answer costing 19.6 seconds.

This is a property of the **domain**, not of any engine: co-attendance graphs
are small-world, so unbounded traversal is not a meaningful operation on them.

Separately, the temporally-correct form is not expressible in Ladybug at all:
per-hop filters are stateless and cannot reference the previous hop
(`Variable e is not in scope`), so monotonic-time traversal cannot be written.

## Consequences

* Replaced by **bounded 2-hop co-attendance with a time window** — 2,808
  persons in 11 ms (Ladybug) / 1.2 ms (SQLite). A fixed-depth bipartite join.
* **This removed the strongest marginal argument for a graph database.**
  Withdrawing the requirement does not rescue the graph option; it weakens it.
* If a bounded variant with strong pruning is later found useful at depth > 2,
  reopen ADR-0015.
