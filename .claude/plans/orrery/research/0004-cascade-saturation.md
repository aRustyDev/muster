<!-- Imported from design thread 2026-08-01. Fable: review and improve. -->

# RESEARCH 0004 — Co-attendance cascade saturation

* Date: 2026-08-01
* Method: reachability measurement by hop depth. Reproduce with
  `evidence/probe_02_cascade.py`.

## Objective

Cascade blast-radius analysis — 'if I move this seminar, what is the full
downstream impact?' — was identified as the analytic most justifying a graph
database: unbounded depth, no natural termination, painful in SQL. Test whether
it is viable.

## Hypothesis

Unbounded-depth co-attendance traversal is a useful analytic that relational
stores serve poorly.

## Method

2,000 persons, 2,000 events, 100,000 `attends` edges. Undirected variable-length
traversal over `attends` alternates Person → Event → Person → Event naturally.
Measured distinct reachable nodes and latency at depths 1–4 from a single person.

## Results

| depth | reached | % of graph | latency |
|---:|---:|---:|---:|
| 1 | 50 | 1.2% | 25 ms |
| 2 | 1,511 | 37.8% | 11 ms |
| 3 | 3,461 | 86.5% | 120 ms |
| 4 | 4,000 | **100.0%** | **19,578 ms** |

## Conclusion

**Hypothesis refuted, on two independent grounds.**

**Semantic.** The co-attendance graph saturates by hop 3. At depth 4 the blast
radius is *everyone* — a vacuous answer costing 19.6 seconds. This is a property
of the domain, not any engine: co-attendance graphs are small-world, so unbounded
traversal is not a meaningful operation on them. Any engine returns 'everyone'.

**Expressive.** The temporally-correct form — each hop strictly after the previous
— is not expressible in Ladybug at all (RESEARCH-0002). A constant-bound
approximation still reached 3,515 nodes in 6,412 ms.

## Replacement

Bounded 2-hop co-attendance with a time window: **2,808 persons in 11.3 ms**
(Ladybug) / **1.2 ms** (SQLite). A fixed-depth bipartite join.

## Significance

This was the **strongest marginal argument** for adopting a graph database.
Withdrawing the requirement does not rescue the graph option — it removes its best
justification. Preserve that asymmetry when revisiting ADR-0015.

## Follow-up

Reopen only if a bounded variant with strong pruning proves useful at depth > 2 —
for example, restricting hops to shared *mandatory* events, or to events within a
narrow window, either of which might keep the neighbourhood sub-saturating.
