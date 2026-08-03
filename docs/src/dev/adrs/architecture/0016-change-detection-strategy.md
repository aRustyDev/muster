<!-- Imported from design thread 2026-08-01. Review, do not assume correct. -->

# 16. Change detection for derived state

* Status: accepted
* Date: 2026-08-01

## Context and Problem Statement

ADR-0004 chose derived attendance. Materialised rows provided things derivation
does not:

| Lost | Consequence |
|---|---|
| Stable edge identity | violations, pins, overrides have nothing to reference |
| Write observability | **the causal write is far from the semantic effect** |
| Diffability | cannot answer "what changed since T" |
| Notification | cannot tell someone their schedule changed |
| Incremental recompute | must recompute all violations, not affected ones |

Row two is the core problem: adding one `member_of` edge silently changes 400
people's derived schedules with no write to `attends` to observe.

The chosen store may provide no help — Ladybug has no CDC or trigger mechanism
documented, and its attach extension is inbound-scan only.

## Decision Outcome

Layered, in this order.

**A. Deterministic identity.** `blake3(person_id ‖ event_id ‖ expectation_id)`.
Stable across recomputation. Prerequisite for everything else; costs nothing.

**B. Persisted digests.** Per person, `derived_digest` (hash of the sorted
derived-edge ID set) and `derived_digest_at`. Detects *that* a schedule
changed; a second pass diffs *what*. Stored on the person record — **no second
store, sole-SoR premise preserved.**

**C. Incremental computation (`salsa`).** The reframe: this is not a database
problem, it is an incremental computation problem. Model
`derived_attends(person, window)` as a memoised query over base facts; salsa
tracks dependencies and invalidates precisely.

**Early cutoff is the feature that matters**: adding a group member re-runs
derivation for that person, and if the resulting set is unchanged, nothing
downstream fires — no violation recompute, no notification, no UI refresh.
The blast radius gets *computed* rather than guessed.

**D. Event log as system of record — v2, not initial.** Gives audit, replay,
time travel, external subscribers. **Note honestly: this inverts the sole-SoR
premise**, making the log authoritative and the store a rebuildable projection.

It is nonetheless a *cheaper* second store than a duplicated system of record:
no schema overlap, never queried except "events since seq N", never written
back to. Divergence is recoverable by replay rather than reconciliation.

### Consequences

* **Route every mutation through one command layer from day one.** Adding the
  log later then becomes inserting a write at one chokepoint rather than a
  refactor. Costs a trait and an enum now.
* Salsa state is a rebuildable cache, non-durable; cold start recomputes.
