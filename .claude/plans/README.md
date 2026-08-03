# plans/ — layout and derivation

## Moved to docs/src (2026-08-03, ADR-0027 — the old→new map)

`docs/src/` is the durable reference home (decisions, strategies,
policies, patterns, roadmaps, glossary); `plans/` holds the working
corpus. Historical documents below this date reference the old paths —
resolve them here:

| Old path | New home |
|---|---|
| `docs/src/adrs/NNNN-*.md` (flat) | `docs/src/dev/adrs/<topic>/NNNN-*.md` (topic table: `dev/adrs/README.md`) |
| `plans/GLOSSARY.md` | `docs/src/dev/glossary.md` |
| `plans/TESTING-STRATEGY.md` §Taxonomy | `docs/src/dev/strategies/testing/coverage-taxonomy.md` |
| §Tool roster + RR&P + reading list | `docs/src/dev/strategies/testing/tool-roster.md` |
| §Measurement-variance (W-2) | `docs/src/dev/policies/benchmarking/measurement-variance.md` |
| §Property naming/budgets + §Regression | `docs/src/dev/policies/testing/property-and-regression.md` |
| §Standing policies | `docs/src/dev/policies/testing/standing-policies.md` |
| §Test doubles | `docs/src/dev/patterns/testing/test-doubles.md` |

`docs/src/dev/roadmaps/` is reserved for cross-plan summarizing
roadmaps (owner ruling 2026-08-03): PLAN.md and ROADMAP.md are
plan-specific and **stay here**.

## Why the overall PLAN.md and ROADMAP.md live at the plans/ root

**The roadmap's primary job is drawing boundaries *between* the three
products** — triplicating it per product would fragment the one thing it
exists to show. Per-product copies are derivations, not duplicates.

## What to put in the per-product copies

Derive, do not duplicate.

`plans/{product}/PLAN.md` — only the phases touching that product, with its
specific entry and exit conditions. Link to root `PLAN.md` for cross-product
sequencing rather than restating it.

`plans/{product}/ROADMAP.md` — only that product's stage table and its
dependencies on the other two. The full boundary matrix stays at the root.

**Rule: any fact appearing in both a root and a per-product document is a fact
that will drift.** Link instead.

## quality-review/ *(added 2026-08-02; review completed 2026-08-03)*

`quality-review/00-review-plan.md` is the pre-committed plan for the
cross-product quality-strategy review (testing / benchmarking / profiling /
validation / telemetry coverage of every crate). Executed 2026-08-02/03:
`01-gap-matrix.md` (Stage A+B evidence), `02-additions-and-order.md`
(Stage C+D synthesis — RR&P stages, ordering, semver), `03-qf-slice.md`
(the QF implementation slice). The durable cross-crate strategy lived at
`plans/TESTING-STRATEGY.md` until 2026-08-03, when ADR-0027 decomposed it
into `docs/src/dev/` (map below); per-crate criteria in each product's
testing spec; the accepted-items ledger in `CARRY-FORWARD.md`; tool
adoptions in ADR-0026.

## Numbering

Question, research, ADR, and phase numbers are **global**, not per-directory.
QUESTION-0005 sits in `muster-sdk/questions/` and creates an apparent gap in
`orrery/questions/`. That is expected — see `orrery/questions/README.md`.

## Spec references

Spec numbers are **per-product** and collide across products. Always qualify:
`orrery/SPEC-03`, not `SPEC 03`. Three products each have a `00-overview.md`.

## Known thin spots *(corrected 2026-08-03, quality review F-13 — the table below was stale in both directions: it still listed spec sets that landed 2026-08-02 as missing)*

| Product | Has |
|---|---|
| orrery | 00–05: overview, data model, functional, non-functional, API, testing |
| muster-sdk | 00–03: overview, objectives/search, API surface, testing criteria |
| muster | 00–03: overview (incl. non-functional), data-and-roles, service API, testing criteria |

No product spec set is missing wholesale any more. What remains thin is
tracked where it can't silently vanish: stage-gate definitions owed at
entry live in `CARRY-FORWARD.md` ("Stage-entry pre-commitments owed" and
"Quality strategy — accepted items"); muster has no separate user-flows
spec — flows live in the PRD and SPEC-02's surface table, a deliberate
placement, not a gap.
