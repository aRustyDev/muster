<!-- Imported from design thread 2026-08-01. Fable: review and improve. -->

# QUESTION 0003 — Where is the 'group universal' requirement set?

* Status: **ANSWERED**
* Raised: 2026-08-01

## Question

On the group node, on the event node, or on a group↔event relation?

## Answer

**On the relation** — `group -[expects]-> event`.

The flag itself has attributes: obligation level, default priority, its own
validity window (distinct from the event's), cascade behaviour, whether members
may decline, and provenance. An array of foreign keys on either node cannot hold
any of that.

Same reasoning that put `priority_score` on `attends` rather than on `person`.

## Consequences / open threads

* The real fork was **not** where the flag lives — that was settled quickly — but
  materialise vs. derive. See QUESTION-0004.
* See ADR-0003.
