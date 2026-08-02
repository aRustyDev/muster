<!-- Imported from design thread 2026-08-01. Fable: review and improve. -->

# RESEARCH 0005 — Rust-compatible embedded graph datastores

* Status: **NOT CONDUCTED. This is a mandate for the next session.**
* Assigned: Fable 5

## Objective

Survey embedded graph datastores usable from Rust and determine whether any beats
the measured SQLite baseline for Orrery's workload.

## Why this gap exists

The graph option was tested only through Ladybug, which is C++ and reached via
FFI. The Rust-native landscape was never surveyed. The project owner prefers an
embedded graph store; the measured evidence currently favours embedded relational.
That tension is the reason for this mandate — it is not settled.

## Staging (ADR-0021)

**This document covers Stage A only — the paper screen.** It blocks
implementation and should take hours, not days. Stage B (Rust screening harness)
and Stage C (repository implementations) are non-blocking and happen later.

## Required method

1. Enumerate candidates. Verify each **actually exists and is maintained** — do
   not rely on the starting list below, which is drawn from training data and is
   unverified.
2. Screen on hard requirements before benchmarking anything.
3. **Do not benchmark in Stage A.** Record survivors and eliminations with
   reasons, then hand off to Phase 2.
4. Report refutations as prominently as confirmations. Three hypotheses were
   refuted in RESEARCH-0003; expect similar.

## Candidate list (supplied by project owner)

**Confidence declaration, stated plainly: I do not reliably recognise most of
these.** Twenty entries, and I have usable prior signal on roughly seven.
Writing capsule descriptions for the rest would manufacture exactly the false
confidence this package exists to prevent. Category assignments below marked
*inferred* are **guesses from the project name only** and carry no weight —
verify every one.

| Candidate | Provisional category | My signal |
|---|---|---|
| **Grafeo** | ? | **none** — owner flagged high interest |
| **LanceGraph** | A or E *(inferred)* | **weak** — if built on the Lance columnar format, embedded + Rust is plausible; owner flagged high interest |
| Cozo | A *(Datalog, not Cypher)* | moderate — embedded, Rust, pluggable backends |
| Oxigraph | B — RDF/SPARQL | moderate — embedded, Rust |
| OxiRS | B *(inferred)* | weak |
| Neo4rs | **C — driver, not a store** | moderate — Bolt client for Neo4j |
| TypeDB | **C — server** | moderate — own query language, not embedded |
| Raphtory | **D — analytics** | weak-moderate — temporal graph analytics |
| IndraGraph | A *(inferred — likely IndraDB?)* | weak — verify the name |
| agdb | A *(inferred)* | weak |
| sqlitegraph | **E** *(inferred from name)* | weak |
| Graphlite | E or A *(inferred)* | weak |
| SolomonDB | ? | none |
| OverGraph | ? | none |
| LoraGraph | ? | none |
| forGQL | ? | none |
| Omnigraph | ? | none |
| ocg | ? | none |
| Grust | ? | none |
| Stromadb | ? | none |

## Category taxonomy — apply this before anything else

Twenty candidates is far too many to benchmark. Categorisation is a one-hour cut
that should eliminate roughly half.

| Cat | Shape | Verdict |
|---|---|---|
| **A** | Embedded labelled-property-graph store | target shape — proceed |
| **B** | Embedded RDF/SPARQL triple store | viable, but **Q1 must be re-expressed** — SPARQL property paths are not Cypher per-hop filters, and whether per-hop edge predicates are expressible is a genuinely different question. Screen it explicitly. |
| **C** | Client driver for a server database | **disqualified** — not embedded. Neo4rs and TypeDB appear to fall here. Confirm before cutting. |
| **D** | Analytics engine, not a system of record | **disqualified as SoR.** But note: a temporal graph analytics engine may still earn a place as a Layer-1 pathfinding or analytics tool alongside the store. Record rather than discard. |
| **E** | Graph API layered over a relational store | **the sleeper — do not dismiss.** |

### Why category E deserves real attention

RESEARCH-0003's central finding is that **every Orrery query is
entity-partitioned before its interval predicate applies**, which is precisely
what b-tree-indexed row stores do best. A graph API over SQLite would inherit
that index behaviour while providing traversal ergonomics and possibly the
tier-enforcement modelling of ADR-0009.

That combination is the one shape the existing benchmark never tested, and on
the evidence it is the most likely to beat both measured options. If
`sqlitegraph` or `Graphlite` turn out to be category E, prioritise them.

### A note on Cozo specifically

If Cozo is Datalog-based, recursion is **native to the query language** rather
than a special traversal construct. Q1 — recursive group expansion with per-hop
temporal validity — may express more naturally there than in Cypher, and the
per-hop-filter stop-gate may not even apply in its Cypher-shaped form. Screen it
on the *semantics* (can each recursive step filter on that step's edge
attributes?) rather than on the syntax.

### Owner-flagged candidates

Grafeo and LanceGraph carry stated high interest. They receive **the same
screen as everything else** — interest earns a careful look, not a pass on the
hard requirements. If either fails the stop-gate, say so plainly.

## Hard requirements (screen before benchmarking)

1. Per-hop edge-property filtering in recursive patterns — Q1 requires it, and
   this was the stop-gate that Ladybug passed
2. Native or first-class Rust bindings, not a C/C++ FFI wrapper unless separately
   justified
3. Concurrency model permitting the intended deployment — Ladybug permits one
   `READ_WRITE` process **or** many `READ_ONLY`, never mixed
4. ACID transactions
5. Maintenance signal: contributors, release cadence, funding model, licence

## Baseline to beat

| Query | SQLite 3.45 @ 1M edges |
|---|---:|
| Q1 derived expansion | 0.2 ms |
| Q2 per-person conflict | 0.6 ms |
| Q3 global sweep | 4,775 ms |
| Q5 room exclusivity | 15.8 ms |
| Q7b co-attend 2-hop | 1.2 ms |
| Bulk load (1M edges) | 20.8 s |

A candidate must beat this on Q1, Q2, and Q7b, or lose by a margin justified by a
compensating advantage **named in advance** — not discovered afterwards.

## Deliverable

Replace this file with findings. Update ADR-0015 from `proposed` to `accepted`
or `rejected`, with the evidence inline.


## Outcome branches — decide by this table, not by improvisation

The screen has four possible shapes. Only the first proceeds as written.

| Survivors | Action |
|---:|---|
| **2-4** | Proceed as planned. Carry all to Phase 1b. |
| **> 4** | Do **not** benchmark all of them. Tighten on maintenance signal (release cadence, contributor count, funding) until 3 remain. Record who was cut and why. |
| **1** | Proceed, but Phase 7 still requires a second repository implementation for differential testing (SPEC orrery/03). Use SQLite as the second. The decision becomes "graph candidate vs. SQLite" rather than "graph vs. graph". |
| **0** | **Stop and report before choosing.** Present the three fallbacks below with a recommendation; do not pick one unilaterally — the graph preference is the project owner's and a null result changes their input, not just yours. |

### Zero-survivor fallbacks

1. **FFI-wrapped Ladybug/Kuzu.** Measured baseline exists. Cost: C++ toolchain
   in a Rust workspace, single-writer concurrency, fork-of-archived-project
   governance risk. Requires an ADR explicitly accepting the FFI cost.
2. **Embedded relational (SQLite/libSQL).** Measured, fastest on three of five
   comparable queries, mature Rust bindings. Cost: ADR-0009's tier enforcement
   must be reproduced with a discriminator plus CHECK constraints and tests.
3. **Hand-rolled graph layer over `redb`.** Full control, native Rust, no query
   language to fight. Cost: you are now maintaining a storage engine, and every
   traversal is code you wrote. Only defensible if 1 and 2 both fail on a hard
   requirement — not merely on preference.

**Do not silently fall through to relational because the screen was
disappointing.** A null result on the graph landscape is itself a finding worth
reporting, and it changes the terms of the owner's preference rather than
overriding it.
