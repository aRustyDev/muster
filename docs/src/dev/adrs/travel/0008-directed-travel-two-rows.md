<!-- Imported from design thread 2026-08-01. Review, do not assume correct. -->

# 8. Directed travel edges are stored as two rows

* Status: accepted
* Date: 2026-08-01

## Context and Problem Statement

Travel a→b ≠ b→a (one-way streets, uphill walking, escalators, one-directional
secured doors). Stated accuracy tolerance is "reasonable estimates", not
±1 minute.

## Decision Drivers

Given the accuracy tolerance, this is not an accuracy question. It is a
**query-surface** question.

## Considered Options

* One row, `duration_ab` / `duration_ba` columns.
* Two rows, one direction each.

## Decision Outcome

Two rows. Every lookup becomes an unconditional
`WHERE from = ? AND to = ?`. The single-row form makes every lookup, every
cache key, and every path-expansion step carry a "am I forward or reverse,
which column do I read" conditional — a persistent bug source infecting the
closure computation, the cache layer, and the client, in exchange for halving a
table of a few thousand rows.

### Consequences

* Symmetric values by default, written from a **single function** so they
  cannot drift.
* A genuinely asymmetric pair is later a row update — no schema change, no
  special case, no migration.
