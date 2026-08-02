<!-- Imported from design thread 2026-08-01. Fable: review and improve. -->

# QUESTION 0004 — Materialise derived attendance, or compute at read time?

* Status: **ANSWERED**
* Raised: 2026-08-01

## Question

When a group expects an event, is an `attends` edge written for each member, or
computed on read?

## Answer

**Derive semantically, cache physically.**

Materialising keeps the read path uniform (conflict detection never needs to know
groups exist) but drifts on membership change. Deriving cannot drift but pushes
complexity into every analytic query.

Initial recommendation was materialise-with-provenance, to protect the simple
read path. That was revised once the derived option was examined properly: the
measured expansion cost is 0.2 ms at 1M edges, so recomputation is not the
constraint — and the correctness argument for derivation is unconditional.

## Consequences / open threads

* **This decision was load-bearing for the datastore analysis.** Materialisation
  keeps the interval index on the hot path; derivation removes it. That coupling
  was initially missed and should be stated explicitly whenever ADR-0015 is
  revisited.
* Creates the change-detection problem — QUESTION-0007.
* See ADR-0004.
