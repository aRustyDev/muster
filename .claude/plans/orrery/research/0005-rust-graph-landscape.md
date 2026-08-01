# RESEARCH 0005 — Rust-compatible embedded graph datastores (Stage A findings)

* Status: **Stage A complete** (paper screen, ADR-0021). Stages B (Rust
  screening harness) and C (repository implementations) remain; ADR-0015 stays
  `proposed` until Phase 7.
* Conducted: 2026-08-01, Fable 5. Method: all twenty owner-supplied candidates
  verified by web search against primary sources (repos, crates.io, project
  docs, specs); no benchmarking. Per-candidate evidence in
  `0005-scorecards/*.md` (one file per candidate, shared template). Survivors'
  stop-gate verdicts were re-verified first-hand against the projects' own
  documentation/conformance artefacts, not taken from the delegated research
  alone.

## Result

**Two survivors: Grafeo and agdb.** Outcome branch "2–4: proceed as planned;
carry to Phase 1b." The screen found no candidate without a serious caveat —
the two survivors carry youth (Grafeo, ~6 months old) and bus-factor-one
(agdb) risk respectively — but both pass all five hard requirements with
first-hand-verified per-hop filtering, and both are synchronous, pure-Rust,
Apache-2.0, ACID, embedded stores.

| Candidate | Category | Stop-gate (req 1) | Fatal requirement(s) | Verdict |
|---|---|---|---|---|
| **Grafeo** | A | **pass** — GQL G050 per-hop WHERE, conformance-file verified | — (maintenance caution: 6 months old) | **advance to 1b** |
| **agdb** | A | **pass** — per-element conditions during walk, ordered comparisons | — (caution: bus factor 1) | **advance to 1b** |
| Cozo | A | pass (Datalog semantics) | **5** — dormant 20 months, no release 2.5 yrs | eliminated |
| Oxigraph | B | **fail** — SPARQL paths cannot filter mid-path | 1 | eliminated |
| OxiRS | B | fail — same SPARQL limitation | 1, 5 | eliminated |
| GraphLite | A | unverified (parse-level only, equality-only) | 5 — stalled 6 months, crate at 0.0.1 | eliminated |
| sqlitegraph | **E** | **fail — verified in source** | 1 (+ GPL-3.0-only) | eliminated |
| IndraDB ("IndraGraph") | A | partial — per-hop yes, recursion absent | 4 (no transactions), 5 (dormant) | eliminated |
| OverGraph | A | fail — no per-hop, no path predicates at all | 1, 5 | eliminated |
| LoraDB ("LoraGraph") | A | fail — own support matrix: "Not in grammar" | 1, 5 (+ BSL 1.1) | eliminated |
| StromaDB | A/D | fail — fixed pipeline, post-traversal filter only | 1, 3, 4, 5 (+ ELv2) | eliminated |
| ocg | A | unverifiable (source behind IBM firewall) | 5 | eliminated |
| Grust | E-adjacent facade | fail — no recursion exists | 1, 4 (and not a store; async-only) | eliminated |
| SolomonDB | A | unverifiable | 4, 5 — abandoned 3.5 yrs | eliminated |
| Neo4rs | **C** — Bolt driver | n/a | category | eliminated |
| TypeDB | **C** — server-only | n/a | category | eliminated |
| Omnigraph | **C** — server-first lakehouse | n/a | category (also 1, 4) | eliminated |
| LanceGraph | **D** — read-only query engine | fail (also structurally: no writes) | category, 4 | eliminated — record |
| Raphtory | **D** — temporal analytics | n/a | category, 4 (+ GPL-3.0) | eliminated — record |
| forGQL | unlocatable | n/a | existence | eliminated pending owner clarification |

## Refutations and negative findings (Rule 01.3 — report these first-class)

1. **The category-E "sleeper" hypothesis is refuted for this list.** The
   screen predicted a graph-API-over-SQLite candidate was "the most likely to
   beat both measured options." Exactly one genuine E candidate exists
   (sqlitegraph), and its executor never inspects edge properties
   mid-traversal — verified in its source, not inferred — plus it is
   GPL-3.0-only. The E *shape* remains attractive on the RESEARCH-0003
   evidence, but no off-the-shelf project delivers it; if the shape is wanted,
   it is a build (a thin traversal layer inside our own SQLite repository
   implementation), not a buy.
2. **Cozo is the mandate's saddest result.** Its "note on Cozo specifically"
   hoped Datalog would dissolve the stop-gate, and it does — recursion with
   per-step edge constraints is native, and it even has first-class time
   travel. It fails anyway: no release since 2023-12, no commit since
   2024-12, one maintainer. The best semantic fit on the list is unmaintained.
3. **The taxonomy did less work than predicted.** The mandate expected the
   category cut to eliminate "roughly half of twenty in about an hour"; it
   eliminated six (3×C, 2×D, 1 unlocatable). The heavy lifting was done by
   maintenance signal (5 eliminations) and the stop-gate (5). Half the list is
   real-but-unfit, not miscategorised.
4. **Both owner-flagged candidates split.** Grafeo survives and is arguably
   the strongest candidate. LanceGraph cannot hold Orrery's data at all — it
   is a read-only Cypher-on-DataFusion layer with no write path; stated
   plainly per the mandate's instruction.
5. **Two list entries were name-drift and one is unlocatable.**
   "IndraGraph" → IndraDB, "LoraGraph" → LoraDB (both then eliminated on
   substance); "forGQL" matches nothing in any registry — most plausibly a
   garbled reference to an ISO-GQL project already on the list (GraphLite's
   grammar derives from the OpenGQL project). Owner should confirm.

## The async question (Rule 04, ADR-0022) — resolved

Every surviving candidate (and the SQLite baseline, and `MemoryRepo`) exposes a
**synchronous** API. The repository trait is therefore **sync**. Recorded as
ADR-0023. Async candidates existed (Grust, LanceGraph, neo4rs) and all were
eliminated on other grounds first — the trait shape did not have to be forced.

## Constraint intersection for MemoryRepo (Phase 2 input)

Across survivors, the restrictive intersection `MemoryRepo` must enforce
(Rule 00b) is confirmed as: **single writer · no concurrent read-during-write
· no cross-hop traversal predicates** — agdb documents exactly
one-mutable-or-many-immutable transactions; Grafeo's G051 (non-local, i.e.
cross-hop predicates) is explicitly *not supported*, matching Ladybug's
limitation. Per-hop **constant** predicates are safely inside the intersection
and Q1 may rely on them.

## What Stage A explicitly did not establish

No performance numbers (by design). For both survivors: bulk-load paths,
tier-constraint enforceability (ADR-0009 — likely a code module on agdb, needs
a probe on Grafeo), behaviour at 1M edges, result-set materialisation cost,
and the durability of maintenance signals (a six-month-old project and a
bus-factor-one project can both look different in six months). Phase 1b's
Rust harness measures; Phase 7 decides against the pre-committed ADR-0015
criteria, with the SQLite baseline re-measured on the decision host
(00-grounding.md, carry-forward).

## Provenance note

The superseded mandate text (candidate list, taxonomy, outcome table) is
preserved in the handoff package at
`orrery-handoff/.claude/plans/orrery/research/0005-rust-graph-landscape-MANDATE.md`.
