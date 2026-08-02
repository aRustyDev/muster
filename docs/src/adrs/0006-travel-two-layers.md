<!-- Imported from design thread 2026-08-01. Review, do not assume correct. -->

# 6. Travel is modelled in two layers

* Status: accepted
* Date: 2026-08-01

## Context and Problem Statement

The initial proposal contained a contradiction: "each location connects to
every other location" (a complete graph of precomputed answers) and "some
locations have no connection to any event, e.g. bldg-1 → lot-a → lot-b →
bldg-2" (a sparse network requiring pathfinding). Both cannot hold — if the
graph is complete, intermediate nodes are never traversed.

## Decision Outcome

Two distinct layers.

**Layer 1 — `traverse` / `transit` (sparse, ground truth).** The real network,
including intermediate nodes hosting no events. Directed. Rarely written.

**Layer 2 — materialized all-pairs cache**, computed from Layer 1 or an
external routing API, **restricted to event-bearing locations**. Read-optimised
point lookups. Batch-recomputed.

### Consequences

* Matches the stated read/write profile (large-scale read, small-scale write).
* Restricting the closure to event-bearing locations bounds the API cost:
  matrix routing APIs bill per origin×destination element, so a naive complete
  graph over 2,000 locations is ~4M billable elements per refresh.
  *(Pricing model unverified — confirm current terms before sizing.)*
* Store components (`distance`, `mode`, elevation) not just the answer, so
  clients can recompute for personal mobility profiles.
* Travel time is not truly static: a single average under-predicts precisely at
  congested times, which is when events cluster. Carry `duration_typical`,
  optional `duration_peak` + window, `provenance` (`measured`/`estimated`), and
  `computed_at`.
