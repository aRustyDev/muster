<!-- Imported from design thread 2026-08-01. Fable: review and improve. -->

# QUESTION 0007 — How is change detection recovered after moving to derived reads?

* Status: **ANSWERED**
* Raised: 2026-08-01

## Question

Materialised rows can be timestamped, diffed, and watched. Derived sets exist only
as query output. What is lost, and how is it recovered — possibly via external
datastores?

## Answer

Five distinct losses, of which one dominates: **the causal write is far from the
semantic effect.** Adding one `member_of` edge silently changes 400 people's
schedules with no write to `attends` to observe.

Recovery, layered:
**A** deterministic content-addressed identity for derived edges (prerequisite);
**B** persisted per-person digests (detects *that* something changed, stays inside
the primary store); **C** `salsa` incremental computation — the reframe being that
this is an incremental computation problem, not a database problem, and **early
cutoff** means an unchanged derived set fires nothing downstream; **D** an event
log as system of record, deferred to v2.

## Consequences / open threads

* **D inverts the sole-system-of-record premise** and that should not be glossed.
  It is nonetheless a cheaper second store than a duplicated SoR — no schema
  overlap, never queried except 'events since N', never written back to.
* Requires routing all mutations through one command layer **now**, so the v2
  upgrade is an insertion rather than a refactor.
* See ADR-0016.
