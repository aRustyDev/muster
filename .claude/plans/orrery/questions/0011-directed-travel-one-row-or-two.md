<!-- Imported from design thread 2026-08-01. Fable: review and improve. -->

# QUESTION 0011 — Two relations for direction, or one relation with two attributes?

* Status: **ANSWERED**
* Raised: 2026-08-01

## Question

Travel a→b ≠ b→a. Stated accuracy tolerance is 'reasonable estimates', not
±1 minute.

## Answer

**Two rows.** Given the tolerance this is not an accuracy question at all — it is
a query-surface question.

Two rows make every lookup an unconditional `WHERE from = ? AND to = ?`. One row
with two columns makes every lookup, every cache key, and every path-expansion
step carry a forward/reverse conditional that infects the closure computation,
the cache layer, and the client.

## Consequences / open threads

* Symmetric by default, written from a single function so they cannot drift.
* A genuinely asymmetric pair later is a row update, not a schema change.
* See ADR-0008.
