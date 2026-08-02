<!-- Imported from design thread 2026-08-01. Fable: review and improve. -->

# QUESTION 0010 — Is the travel graph complete or sparse?

* Status: **ANSWERED**
* Raised: 2026-08-01

## Question

The proposal simultaneously described every location connecting to every other
location, **and** intermediate nodes hosting no events (bldg-1 → lot-a → lot-b →
bldg-2). These are incompatible.

## Answer

They are two different layers, and both are needed.

**Layer 1** is the sparse ground-truth network where edges are *segments* and
answering a query requires pathfinding through intermediates. **Layer 2** is a
dense materialised cache of precomputed *answers*, restricted to event-bearing
locations.

## Consequences / open threads

* Matches the stated large-read / small-write profile.
* Restricting the closure to event-bearing locations bounds routing-API cost,
  which bills per origin×destination element.
* Layer-1 pathfinding is the only genuine graph workload, is small (hundreds to
  low thousands of nodes), and fits in memory — `petgraph`, not a datastore.
* See ADR-0006.
