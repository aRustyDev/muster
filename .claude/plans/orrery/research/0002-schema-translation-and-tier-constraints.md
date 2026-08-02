<!-- Imported from design thread 2026-08-01. Fable: review and improve. -->

# RESEARCH 0002 — Schema translation and tier-constraint verification

* Date: 2026-08-01
* Method: implemented the full Orrery schema in Ladybug 0.19.0 and probed
  behaviour empirically. Reproduce with `evidence/orrery_schema.py`,
  `evidence/probe_01_recursive.py`.

## Objective

Determine whether the Orrery model translates cleanly to a typed property graph,
and test two specific hypotheses.

## Hypothesis 1 — per-hop edge filtering in recursive patterns

**Stated as the stop-gate:** Q1 requires filtering each hop of `subgroup_of*` on
that hop's temporal validity. Whole-path post-filtering would degrade it badly.

**Method.** Built a group chain g1→g2→g3→g4 with all edges valid, plus g2→g5 with
an **expired** edge. Ground truth at t=150 is {g2, g3, g4}; g5 must be excluded.

**Result — CONFIRMED for constant-bound predicates.**

| Query | Returns |
|---|---|
| unfiltered `*1..5` | g2, g3, g4, **g5** |
| `(rel, n \| WHERE rel.valid_from <= $t AND rel.valid_to >= $t)` | g2, g3, g4 |

**Result — REFUTED for cross-hop predicates.** Per-hop filters are **stateless**:
they compare an edge against constants and parameters only.

| Attempt | Outcome |
|---|---|
| `WHERE r.start_ts >= 300000` (constant) | works |
| `WHERE r.start_ts >= e.start_ts` (prev-hop alias) | `Binder exception: Variable e is not in scope` |
| `rels(path)[i] >= rels(path)[i-1]` | Parser exception |

Temporal monotonicity across hops is **not expressible**.

## Hypothesis 2 — tier constraints become schema

**Stated:** modelling location tiers as separate node tables makes illegal edges
*unrepresentable* rather than merely invalid.

**Method.** Declared `traverse(FROM Room TO Room, FROM Structure TO Structure,
FROM Campus TO Campus)`, `transit(FROM Structure TO Structure)`,
`within(FROM Room TO Structure, FROM Structure TO Campus)`. Attempted five edge
creations.

**Result — CONFIRMED, 5/5.**

| Case | Outcome |
|---|---|
| `traverse` Room→Room | accepted |
| `transit` Structure→Structure | accepted |
| `transit` Room→Room | **rejected** |
| `traverse` Room→Structure | **rejected** |
| `within` Structure→Room (inverted tier) | **rejected** |

'No train between rooms' is enforced by the type system, not a CHECK constraint.

## Incidental findings

* Polymorphic relationship tables require an explicit pair on bulk load —
  `COPY within FROM 'f.csv' (from='Room', to='Structure')` — so each tier pair
  needs its own file. Modest friction on ingest.
* Variable-length patterns did not accept a `*0..N` lower bound in the form
  attempted; the zero-hop case (a person's own groups) must be unioned
  separately. This produced a genuine result discrepancy against the SQL
  implementation until the semantics were matched — see RESEARCH-0003.

  > **Addendum (Phase 0, 2026-08-01) — the claim above is refuted.**
  > `-[r:subgroup_of*0..5 (rel, n | WHERE ...)]->` parses and runs correctly
  > on ladybug 0.19.0 (58 events, 2.1 ms at L, matching SQLite exactly).
  > Whatever form was "attempted" in the design thread, the capability
  > exists — so the union workaround was unnecessary, and the RESEARCH-0003
  > fairness correction aligned SQLite *down* to the domain-wrong `*1..5`
  > semantics instead of aligning Ladybug *up* to include depth 0. The
  > harness now uses `*0..5`. See phases/00-grounding.md.

## Conclusions

1. Q1 is expressible. The stop-gate passes.
2. Q7 in its temporally-correct form is **not** expressible.
3. The tier-enforcement advantage is real and is the one benefit unique to the
   graph model. Any relational alternative must reproduce it with a discriminator
   plus CHECK constraints and tests — write-time rather than type-level.
