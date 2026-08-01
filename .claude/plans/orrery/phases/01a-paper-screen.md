# Phase 01a — Paper screen (RESEARCH-0005 Stage A)

* Status: `complete`
* Blocks: nothing further — the screen returned survivors, so Phase 2 is
  unblocked (ADR-0021: only a zero-survivor result would have stopped the line)
* Blocked by: Phase 00 (complete)

## Objective

The twenty owner-supplied candidates in RESEARCH-0005 are each verified
(existence, identity, category, maintenance), the five-category taxonomy is
applied, and survivors are screened against the six hard requirements
(the five from ADR-0015 plus the sync/async trait-shape question, Rule 04).
Output: 2–4 survivors carried to Phase 1b, every elimination reasoned, no
benchmarking performed.

## Method note (Rule 01.1)

Hypotheses and acceptance criteria below were committed to this file **before
any candidate research results were read** (research was delegated to four
parallel search agents; the skeleton was written while they ran, prior to
their reports arriving). Verification is web-search-grounded per the mandate —
no candidate was judged from model recall. Both survivors' stop-gate verdicts
were additionally verified first-hand against primary sources: Grafeo's
`docs/gql-dialect.json` conformance file (G050 `supported, tested,
test_count: 12`) and agdb's queries reference (per-element condition
evaluation, `Beyond`/`NotBeyond` traversal gating, ordered comparisons).

## Hypotheses (pre-committed)

| # | Hypothesis | Falsified by | Status |
|---|---|---|---|
| H1 | At least two candidates are verifiable, maintained, category-A embedded property-graph stores | fewer than two surviving existence + category + maintenance checks | **confirmed** — Grafeo, agdb (each with a named caveat: youth; bus factor 1) |
| H2 | **Stop-gate:** at least one surviving candidate supports per-hop edge-property filtering in recursive traversal | zero candidates passing hard requirement 1 | **confirmed** — both survivors pass, first-hand verified; Q1 does not need restructuring |
| H3 | The category taxonomy alone eliminates roughly half the list | fewer than 6 or more than 16 eliminated before the hard-requirement screen | **confirmed at the lower bound, and misleading as prophecy** — exactly 6 fell to taxonomy (3×C, 2×D, 1 unlocatable). The mandate's "roughly half in an hour" overestimated the taxonomy's share: maintenance signal (5) and the stop-gate (5) did the heavy lifting |
| H4 | At least one candidate is genuinely category E and survives — the "sleeper" | no verified category-E candidate, or none passing | **REFUTED** — the sole true E candidate (sqlitegraph) fails the stop-gate in verified source (edge properties never inspected mid-traversal) and is GPL-3.0-only. The E shape survives only as a potential build inside our own SQLite repo impl, not as a buy |
| H5 | At least one survivor offers a synchronous Rust API | every survivor async-only | **confirmed** — both survivors sync; trait stays sync (ADR-0023) |

## Acceptance criteria (pre-committed)

| Criterion | Threshold | Actual | Verdict |
|---|---|---|---|
| Coverage | all 20 candidates scored, zero silent drops | 20 scorecards in `research/0005-scorecards/` | pass |
| Elimination hygiene | every elimination names the failed rule/requirement | see summary table in RESEARCH-0005 findings | pass |
| Stop-gate rigour | survivor req-1 verdicts backed by quoted primary documentation | Grafeo: conformance JSON + syntax docs; agdb: queries reference, fetched directly | pass |
| Survivor count | 2–4 → proceed | **2** (Grafeo, agdb) | pass — proceed |
| Trait shape (Rule 04) | resolved with written recommendation | sync — ADR-0023 written | pass |
| No benchmarking | no performance numbers produced | none produced | pass |

## Results

**Survivors from the screen: Grafeo and agdb** — both category-A, pure-Rust,
Apache-2.0, embedded, ACID, synchronous, with first-hand-verified per-hop
edge-property filtering. Neither is risk-free: Grafeo is ~6 months old with a
~2.5-month quiet spell on `main`; agdb has years of steady cadence (release 4
days before the screen) but effectively one maintainer.

### Amendment — owner decision, 2026-08-01

After reviewing the screen result, the project owner accepted Cozo's
maintenance risk ("the lack of activity is acceptable for Cozo").
**Cozo is reinstated; three candidates advance to Phase 1b** — still inside
the pre-committed 2–4 proceed branch, so no other outcome-table row changes.
The requirement-5 criterion itself stands and was not mis-specified; this is
the owner input that ADR-0015 explicitly names as legitimate, exercised
against a named risk. The accepted consequences (de-facto self-maintenance,
fork-readiness before Cozo could win Phase 7) are recorded on the Cozo
scorecard, and ADR-0015's closing ADR must restate them if Cozo wins.

Full elimination table, refutations, and the MemoryRepo constraint
intersection: `.claude/plans/orrery/research/0005-rust-graph-landscape.md`.
Headlines:

* **Refuted: the category-E sleeper** (H4 above) — reported first per Rule 01.3.
* **Cozo passes the semantics and fails the project**: native Datalog
  recursion with per-step constraints — the mandate's hoped-for shape — on a
  store with no release since 2023 and no commit since 2024.
* **Category B fails structurally, as the mandate's caveat predicted**:
  SPARQL property paths cannot bind intermediate edges, so Oxigraph (healthy)
  and OxiRS (not) both fail requirement 1 regardless of quality.
* **Both owner-flagged candidates split**: Grafeo survives; LanceGraph is a
  read-only query engine with no write path — it cannot hold Orrery's data.
* **Name drift**: "IndraGraph"→IndraDB, "LoraGraph"→LoraDB (eliminated on
  substance); **"forGQL" is unlocatable** — owner should say what it meant.

## Decisions produced

* **ADR-0023** (new, accepted): the repository trait is synchronous.
* ADR-0015 remains `proposed` — Phase 7 closes it against the pre-committed
  criteria, with the Q7b SQLite baseline re-measured there (00-grounding.md
  found the published 1.2 ms unreproducible from the shipped harness).
* QUESTION-0012 partially answered (landscape surveyed; performance question
  deferred to Phases 1b/7 by design).

## Carry-forward

| Item | Resolves in |
|---|---|
| Phase 1b Rust screening harness over Grafeo + agdb + Cozo (bulk load, Q1/Q2/Q7b shapes, result materialisation — carry Phase 0's harness corrections) | Phase 1b (parallel with 3–5) |
| Cozo fork/vendoring readiness plan (owner accepted dormancy risk; we self-support) | before Phase 7 could select Cozo |
| Grafeo: probe tier-constraint enforceability (ADR-0009) and G050 behaviour at depth/scale | Phase 1b |
| agdb: `Distance` counts nodes+edges — depth-5 subgroup traversal ≈ distance 10; encode in harness | Phase 1b |
| MemoryRepo enforces the confirmed restrictive intersection (single writer, no read-during-write, no cross-hop predicates) | Phase 2 |
| Ask owner: what did "forGQL" refer to? Confirm "LoraGraph"=LoraDB, "IndraGraph"=IndraDB readings | next owner touchpoint |
| Watch-list: GraphLite (if development resumes) warrants re-screen; Cozo moved off the watch-list into Phase 1b by the owner amendment | Phase 7 at latest |
