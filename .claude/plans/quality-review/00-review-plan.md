# Quality-strategy review — plan (pre-committed 2026-08-02, before execution)

*Owner-directed. A structured review of the plans corpus — every crate in
`crates/**` plus the workspace level — against the full testing / benchmarking /
profiling / validation / telemetry taxonomy below. Modeled on the 2026-08-02
adversarial plan review (`orrery/artifacts/plan-review-2026-08-02.md`): method
and acceptance criteria pre-committed here; findings tiered; every disposition
a dated, visible edit. This document is the plan for the review — running it
is a separate, explicitly-started activity (execution slices at the end).*

## Purpose (the six goals, restated as commitments)

1. **Gap analysis** — identify quality dimensions absent from the plans.
2. **Robustness analysis** — identify dimensions present but under-specified:
   unmeasurable gates, unowned promises, broken documented doors (the MO-8 /
   MO-10 classes from the last review).
3. **Per-crate synthesis** — decide what each crate's plans *should* contain,
   and in which document each addition lives (Rule 07: one home per fact).
4. **Ordering** — a dependency-honest implementation order for every accepted
   addition (`feat-1 → feat-2`), with the ordering rules stated, not implied.
5. **Plan updates** — land the additions as dated amendments, including
   explicit **RR&P stages** (review, research & plan) for every decision we
   should not settle yet — above all, library selection for capabilities we
   cannot implement yet.
6. **ROADMAP update** — reflect the order with soft semver predictions,
   labeled as estimates (Rule 01.4: plausible inference, not commitment).

## Scope

* **Primary object: the plans corpus** — `plans/{orrery,muster,muster-sdk}/specs/**`,
  phase docs, PRDs, root PLAN/ROADMAP/CARRY-FORWARD, `.claude/rules/**`,
  justfiles (they are executable plan statements), `evidence/`.
* **Secondary object: the code**, as evidence for what plans claim
  (dev-dependency inventory, test families, harnesses). The review judges
  plans; code is the witness.
* **Six crates + one workspace row**: `orrery`, `muster-sdk`, `muster`,
  `muster-types`, `muster-server`, `muster-ui`, and `workspace/CI` (a real
  review row: CI, audit, licensing, coverage aggregation, and cross-crate
  matrices belong to no single crate).

## The dimension taxonomy (matrix rows)

Grouped; each dimension keeps its own matrix row. IDs for cell references.

**Correctness (C)**
| ID | Dimension | Notes / seed tools |
|---|---|---|
| C1 | Unit | existing baseline |
| C2 | Integration | cross-crate seams, `Engine<MemoryRepo>` |
| C3 | End-to-end | `e2e_` family exists (muster, muster-server) |
| C4 | Smoke | demo runs / consumer simulation (informal today) |
| C5 | Property-based | proptest in use; quickcheck as comparison seed |
| C6 | Parameterized | `test-case`; divan's generic params for benches |
| C7 | Fixture-based | `build_demo_world`, SPEC-05 seeded fixtures |
| C8 | Mock-based | none today; mockall / conditional-compilation / mock_shootout |
| C9 | Entrait / DI patterns | entrait crate + pattern essays (seed) |
| C10 | Mutation | cargo-mutants (maintained) vs mutagen (dormant — verify) |
| C11 | Model-based | `model` crate; command-sequence MBT vs MemoryRepo oracle |
| C12 | Test generation | test-generator (file-driven cases) |
| C13 | Regression | pinned-refutation tests exist (e.g. H1 travel regression) — is the *practice* written down? |
| C14 | API testing | HTTP contract (muster-server); `check-seam` for the library API |
| C15 | Documentation testing | doc tests, skeptic, docmatic; rustdoc examples policy |
| C16 | Generated-output matchers | specker (seed); snapshot testing generally (insta as candidate — seeds are a floor, not a ceiling) |
| C17 | Feature-combination | cargo-featomatic (seed; incomplete) vs cargo-hack; repo-* feature matrix is REAL (Rule 04) |
| C18 | FFI testing | expected N/A everywhere today — must be dispositioned, not skipped |
| C19 | Data validation | Rustdantic vs serde+validator vs serde+garde (seed list); DTO boundary = muster-types/-server |

**Performance (P)**
| ID | Dimension | Notes |
|---|---|---|
| P1 | Micro-benchmarking | criterion vs divan; SPEC-05 "seven canonical queries" promise |
| P2 | Macro-benchmarking | hyperfine (binaries); measure_ harnesses are proto-macro-benches |
| P3 | CI benchmarking | Bencher (seed) — dormant until CI exists |
| P4 | Load testing | muster-server only |
| P5 | Stress testing | muster-server (HTTP) + muster-sdk (instance scale) |
| P6 | Spike testing | muster-server only |
| P7 | Soak testing | muster-server; also salsa-mirror memory growth over long command streams |
| P8 | Cache performance | cachegrind/iai class; entity-partitioned b-tree claim is cache-behavior-adjacent |
| P9 | Timing consistency | variance treatment of measure_ harnesses (single-run today); knife-edge H4 precedent |
| P10 | PGO | binaries only (muster, muster-server); kobzol cargo-pgo guide (seed) |
| P11 | CPU profiling / flamegraphs | samply / cargo-flamegraph / Instruments on macOS; tracing-flame |
| P12 | Memory profiling / allocation testing | dhat-rs, allocation counters; salsa mirror + violation-record growth |
| P13 | Network profiling | muster-server only; lowest priority — disposition honestly |

**Safety & robustness (S)**
| ID | Dimension | Notes |
|---|---|---|
| S1 | Fuzzing | cargo-fuzz / afl.rs / honggfuzz (seeds); prime targets: interval algebra, command apply, preview honesty, HTTP payloads |
| S2 | Memory safety | valgrind (seed) — **not viable on this host** (macOS/Darwin 25); miri/sanitizers need rustup nightly (also blocked); container path exists |
| S3 | Supply chain | cargo-audit (Rule 09 already promises it "once CI exists" — CI does not exist) |
| S4 | License compliance | cargo-lichking (seed) vs cargo-deny (subsumes S3+S4 — evaluate) |

**Infrastructure & meta (I)**
| ID | Dimension | Notes |
|---|---|---|
| I1 | Coverage | none today; tooling constrained by no-rustup host (llvm-tools/tarpaulin questions) — prime RR&P |
| I2 | CI itself | **does not exist** (`.github/` absent) — a dependency of P3/S3/I1 aggregation and much else |
| I3 | Telemetry: otel tracing/metrics/structured logging | Rule 05 is the binding split; OTLP deferred pending a collector; metrics story unwritten |
| I4 | Usability testing | muster-ui + API/doc ergonomics; human-protocol territory (MO-8 already flags trial-protocol debts) |

## Method — five stages

**Stage A — inventory and gap matrix.** Build the (36 dimensions × 7 rows)
matrix. Every cell gets a disposition from a fixed vocabulary: **covered**
(plan exists, measurable, owned — cite doc:line) · **partial** (mentioned but
incomplete) · **gap** (absent, applicable) · **N/A** (inapplicable — with the
reason written; C18/FFI must earn its N/A per crate, not receive it). Evidence
pointers mandatory; a cell without a citation is a gap claim, not a coverage
claim.

**Stage B — robustness pass** over every covered/partial cell, asking the
last review's questions: is the gate *measurable as written* (MO-8 class)?
Is the promise *owned by a phase* (CR-4 class)? Does the documented door
*actually open* (MO-10 class — e.g. `just orrery::bench-canonical` references
a bench target that does not exist)? Did a claim harden while propagating
(Rule 01.4)? Findings tiered Critical / Moderate / Low, refutations of the
review's own expectations reported first.

**Stage C — synthesis.** Per crate, the 'to add' list. Every item classified:

* **implement-now** — no open questions; lands as a spec/phase amendment with
  an owner. (Bar: tool choice obvious or already in baseline; Rule 06
  satisfiable in one line.)
* **RR&P stage** — a named review-research-plan work item with: the question,
  the candidates (seeded from this plan's Appendix), the deciding criteria,
  the host/CI constraints that bear on it, and the phase whose entry it gates.
  Library picks default here, not to implement-now.
* **reject** — with the reason written (the ADR-0020 discipline: record the
  consequence you dislike).

Seed-source triage protocol: every seed in the Appendix gets exactly one of
{adopt-now, fold-into-RR&P-item-N, reject-with-reason}. Seeds are a floor —
Stage C may add candidates (e.g. insta alongside specker, cargo-hack alongside
featomatic, cargo-deny alongside lichking, divan alongside criterion) but must
mark them as review-added.

**Stage D — ordering + semver.** Dependency rules stated up front (and
falsifiable against the result):

1. Infrastructure before consumers: CI (I2) precedes CI-benchmarking (P3),
   audit-in-CI (S3), coverage aggregation (I1-reporting).
2. Measurement before optimization: coverage (I1) before mutation (C10);
   criterion baselines (P1) before regression gates (P3); profiling (P11/P12)
   before PGO (P10); never tune what has not been measured (Rule 01, the
   slice-2 H4 precedent).
3. Surfaces before their tests: load/spike/soak (P4-P7) need the coordinator
   flow and a deployable server; UI testing needs the Alpha UI content.
4. Funnel discipline: nothing benchmarks datastore candidates ahead of the
   ADR-0021 stage — Phase-7-adjacent perf work slots behind the down-select
   (root CLAUDE.md's one open decision stays open).
5. RR&P stages precede their implement items by at least one slice.

Semver method: release-please + Conventional Commits on a 0.1.0 workspace.
Predictions map ordered tranches to stage gates (e.g. "with Muster Alpha ≈
0.2.0"), are labeled *soft estimates*, and note that `test:`/`chore:`-typed
work moves no version — only the `feat:` tranches appear in the mapping.

**Stage E — landing.** Dated amendments only, placed by these rules:
cross-crate strategy (tool choices, tiers, CI shape) → ONE new root document
`plans/TESTING-STRATEGY.md` (Rule 07: single home; product specs link, never
restate). Per-crate criteria → that product's testing spec (orrery/SPEC-05,
muster/SPEC-03, muster-sdk/SPEC-03; muster-types/-server/-ui inherit muster's
spec set — whether they need their own spec section is itself a Stage C
question). Work items → CARRY-FORWARD rows with owners; RR&P stages → both a
ledger row and a PLAN.md mention if they gate a phase entry. ROADMAP gets one
dated ordering-and-semver section. Nothing edits an accepted ADR (Rule 02);
new tool adoptions that touch orrery need ADRs (Rule 06 — dev-dependencies
included, since ADR-0022 lists `proptest`).

## Constraints the review must respect (pre-stated so findings don't rediscover them)

* **Host**: macOS (Darwin 25), **no rustup** (Homebrew rust). Directly blocks:
  miri, sanitizers, llvm-tools-preview-based coverage, valgrind (no macOS
  support). Viable natively: samply/cargo-flamegraph/Instruments, dhat-rs,
  criterion/divan, hyperfine, cargo-mutants, cargo-fuzz (verify on this host —
  needs nightly for sanitizer-backed runs; libFuzzer mode may also be blocked
  → the fuzzing RR&P must answer this). Escape hatch: OrbStack Linux
  containers — which is really the CI question (I2) wearing a costume.
* **No CI exists.** Anything phrased "in CI" is dormant until I2 lands; I2 is
  expected to be the review's highest-leverage single item (verify, don't
  assume).
* **Rule 05 split is binding**: libraries instrument, binaries configure.
  Telemetry findings must not propose exporters in orrery/muster-sdk. The
  `backend` span attribute is load-bearing for Phase 7 — telemetry robustness
  review checks that plan is still real.
* **Rule 09**: privacy tests are a *hard* gate class; fuzzing/validation
  additions at the wire boundary must extend, never bypass, the `privacy_`
  family. Anchor data never in fixtures that leave the repo.
* **ADR-0015 is open.** No review outcome may presuppose a datastore.
* **Rule 06 bars** apply to every proposed tool (maintained? <200-lines
  self-implementable? transitive runtime?). Several seeds are expected to fail
  the maintenance bar (mutagen, cargo-featomatic is explicitly incomplete,
  specker's activity — verify in Stage C, reject with citations).

## Inventory observations already in hand (Stage-A seeds — verified this session unless marked; not yet findings)

* Dev-dependency surface today: `proptest` (orrery, muster-sdk), `tower`
  (muster-server). No criterion, no mock/mutation/coverage/fuzz tooling.
* No `benches/` anywhere; `just orrery::bench` and `bench-canonical` reference
  nonexistent targets; SPEC-05's "Benchmark" level (seven canonical queries at
  three scales, regression-gated) is otherwise unowned — MO-10/CR-4 hybrid.
* `.github/` absent → I2 gap; Rule 09's cargo-audit clause dormant.
* `plans/README.md` "known thin spots" table is stale (muster-sdk/muster
  testing specs now exist) — small drift finding to fix in Stage E.
* Existing strength to build on, not duplicate: property+oracle discipline,
  privacy_ family, measure_ harnesses with pre-pinned profiles, boundary
  greps (check-seam/check-scope), 91-test suite, differential-testing plan
  (SPEC-05) awaiting a second repo impl.
* measure_ harnesses take single-run numbers with stride sampling; no
  variance/warm-up treatment → the P9 row starts from a known weakness.
* muster-types has no tests of its own (wire-shape tests live in
  muster-server); muster-ui has no tests at all (structure landed slice 2,
  content is Alpha scope) — expected Stage-A gaps, cited here so the matrix
  can't miss them.

## Acceptance criteria for the review itself (pre-committed)

| Criterion | Threshold |
|---|---|
| Matrix completeness | all 36 × 7 cells dispositioned with evidence pointers; N/A cells carry reasons |
| Seed triage | every Appendix seed dispositioned {adopt-now, RR&P-item, reject+reason}; review-added candidates marked as such |
| Findings integrity | tiered; refutations-first; every Critical/Moderate has a disposition line; arithmetic on any "N of M" claim checked (Rule 01.6) |
| RR&P stages | each has question, candidates, deciding criteria, constraints, and the phase entry it gates — no naked "investigate X" rows |
| Ordering | every accepted item appears exactly once in the order; each ordering edge cites one of rules D1–D5; violations called out, not smoothed |
| Semver | every `feat:` tranche mapped to a predicted 0.x, labeled soft estimate |
| Landing | Rule 07 single-home respected (spot-check: no fact in both TESTING-STRATEGY.md and a product spec); all edits dated; ledger rows added for every accepted item; no row of the existing ledger silently dropped |
| Gates | check-xrefs green after edits; plain-language artifact for the review (standing criterion) |

## Deliverables (execution outputs, by path)

1. `plans/quality-review/01-gap-matrix.md` — Stage A+B: matrix + tiered findings.
2. `plans/quality-review/02-additions-and-order.md` — Stage C+D: per-crate
   'to add' lists, RR&P stage definitions, seed triage table, ordered plan,
   semver mapping (the reviewable synthesis before anything is landed).
3. `plans/TESTING-STRATEGY.md` — Stage E: the single cross-crate home.
4. Dated amendments: product testing specs, ROADMAP (ordering + semver
   section), CARRY-FORWARD (new section: "Quality strategy — accepted items"),
   plans/README (layout line for `quality-review/`, thin-spots fix).
5. `plans/orrery/artifacts/quality-review-<date>.md` — plain-language artifact.

## Execution slices (each opens by re-reading this plan; each closes with a commit)

* **Slice QR-1 (Stages A+B)** — inventory sweep + matrix + findings. Heaviest
  reading; no repo edits beyond deliverable 1. Est. one focused session.
* **Slice QR-2 (Stages C+D)** — synthesis, triage, ordering, semver. Owner
  touchpoints likely (see below). Deliverable 2. Est. one session.
* **Slice QR-3 (Stage E)** — land everything as dated amendments; gates;
  artifact; ledger. Est. one session, mechanical if QR-2 was honest.

Compression to two sessions (QR-1+2, QR-3) is acceptable; compressing QR-3
into QR-2 is not — the synthesis must be reviewable before it rewrites the
plans corpus (the same reason the phase cadence separates pre-commitment from
implementation).

### Slice close-out protocol *(added 2026-08-02 — every slice ends with both steps)*

**1. Compaction-ready close.** Nothing load-bearing may live only in the
conversation. At close: the slice's deliverable is committed; any working
notes are folded into it or discarded; the persistent project memory
gains (QR-1) or updates (QR-2/3) a `quality-review-state` entry recording
which slice closed, the deliverable path, and the two or three facts the
next slice needs that its deliverable doesn't make obvious (e.g. a
Stage-A disposition surprising enough to reshape Stage C). Write-as-you-go
during the slice too, so an *unplanned* mid-session compaction also loses
nothing — the repo and memory, never the transcript, are the state of
truth.

**2. Memory-aware kickoff for the next slice.** Rewrite
`quality-review/NEXT-QR-SESSION.md` with a paste-ready prompt from this
template; after QR-3 (no next slice) delete the file, mark the state
memory complete, and record review completion in CARRY-FORWARD:

> You are continuing the Orrery/Muster **quality-strategy review**. Your
> project memory has `quality-review-state` — trust it for orientation;
> the repo is the state of truth. Read ONLY
> `plans/quality-review/00-review-plan.md` (binding: method, acceptance
> criteria, execution architecture, this protocol) and
> `<prior deliverable path>`. Do NOT pull the plans corpus into the main
> context — reading is delegated per the plan's Execution architecture.
> This session: **QR-<N> (Stages <X>)** — <one-line scope>; deliverable
> `<path>`. Close per the slice close-out protocol (compaction-ready
> close + rewrite/retire this file). Conventional Commits;
> `Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>`.

## Execution architecture *(added 2026-08-02, same day — context-budget amendment)*

Inline execution was estimated at ~450–700k tokens of session context
(QR-1 alone ~200–300k: the corpus is ~14 spec files, 9 phase docs, PRDs,
ADR skims, plus code evidence). The review therefore runs **delegated
where the work is reading, inline where the work is judgment**:

* **QR-1 — fan-out.** One reader subagent per corpus slice: `orrery`
  (specs + phase docs + crate evidence), `muster` family (muster,
  muster-types, muster-server, muster-ui — they share muster's plan
  corpus), `muster-sdk`, and one cross-cutting reader (rules, root plan
  docs, justfiles, evidence/, CI absence). Each receives the dimension
  taxonomy and disposition vocabulary from this plan and returns **matrix
  cells only** — disposition + doc:line citation + one-line evidence
  quote, no file dumps. Then one **completeness critic per corpus slice**:
  "find quality-relevant plan statements NOT captured in the returned
  cells" (the miss-risk control; delegated reading without a critic pass
  is hope, not method). The main loop merges cells, spot-checks citations,
  and does ALL Stage-B tiering itself — finding severity is judgment, not
  reading.
* **QR-2 — triage fan-out, synthesis inline.** Seed-source maintenance
  verification (Rule 06 bar 2: releases, contributors — ~25–30
  crate-shaped seeds) fans out to web-checking subagents returning
  verdict rows. Synthesis, ordering (rules D1–D5), and the semver mapping
  stay in the main loop.
* **QR-3 — inline.** Rule 07 single-home placement across ~8 documents
  needs one context holding the whole corpus map; the edits are cheap.
* **Fresh session per slice.** Each QR slice starts clean; the only
  carried context is this plan plus the prior slice's deliverable. No QR
  slice piggybacks on a session that already carries other work.

Why this is safe here: the review's own acceptance criteria already
require per-cell citations, so delegated cells are spot-checkable without
re-reading their sources. Honest cost note: delegation raises total
billed tokens (each agent re-reads its slice) — the gain is main-context
health and wall-clock parallelism, not spend.

## Owner touchpoints this review will surface (queue now, don't block on them)

1. **Sequencing vs Muster Alpha**: recommended — run QR-1/QR-2 *before*
   writing the Muster Alpha pre-commitment (its testing criteria are exactly
   what this review upgrades); QR-3 can land in parallel with Alpha
   implementation. Owner may instead run Alpha first, accepting one slice of
   pre-review-quality criteria.
2. **CI platform** (gates S3/P3/I1 aggregation): GitHub Actions is implied by
   the repo host and release-please, but that is an inference (Rule 01.4) —
   confirm before the I2 RR&P closes.
3. **Human-protocol testing** (I4, and MO-8's trial-protocol debts): how much
   usability formality this project actually wants, pre-1.0.

## Appendix — seed sources (verbatim input to Stage-C triage; a floor, not a ceiling)

* Fuzzing: afl.rs · cargo-fuzz · honggfuzz
* Micro-bench: criterion (docs/book/bencher.dev/generalist guides) · divan
* Macro-bench: hyperfine · rustc-perf (harness pattern, not a dependency)
* CI bench: Bencher (bencher.dev)
* Validation: Rustdantic (mmgehlot.github.io/rusdantic) · serde+validator ·
  serde+garde · valida · verify · validatron · accord · satya · valust-rs ·
  scrutiny · axum-valid · (surveys: agileengine c4, masteringbackend series,
  codezup, leapcell, rustfaq, ruststepbystep, dev.to form-validation)
* Property: proptest(+derive) · quickcheck(+macros)
* Mocking: mock_shootout comparison · conditional-compilation pattern
  (klau.si) · time-mocking gotchas (blog.iany.me) · oliverwyman essay ·
  generic-parameter mocking as an explicit evaluation criterion
* Parameterized: test-case · Snapshot/output: specker · Test generation:
  test-generator · Model-based: `model` crate
* Mutation: cargo-mutants · mutagen · bough
* Docs: skeptic · docmatic · doc tests
* Feature combos: cargo-featomatic (incomplete) · Licenses: cargo-lichking ·
  Audit: cargo-audit
* Patterns: entrait (crate + audunhalland essays + philosophy) · OO DI
* Profiling/PGO corpus: cargo-pgo (kobzol) · rustc PGO docs + dev-guide ·
  llvm PGO thread · nnethercote perf-book · brendangregg flamegraphs ·
  flamegraph-rs · inferno · tracing-flame · pprof-rs · minicov · hotpath.rs
  series (modes/overhead/async/tokio/locks/io/sql/http) · oneuptime series ·
  criterion-flamegraph integrations (zimmerer, ritik-chopra) · dtrace on
  macOS (carol-nichols) · infinilabs macOS guide · databend/greptime/tikv
  writeups · patrickfreed "making slow rust fast" · wasm time-profiling
  (rustwasm book — muster-ui relevance) · async flamegraphs (rustyrazorblade,
  medium/rustaceans, hegdenu divan)
* General: awesome-rust-testing · llogiq test post · devgenius testing
  mastery · elitedev 7-performance-patterns · tarpaulin/coverage threads
  (knoldus, users.rust-lang) · boundaryml explore

*Triage duty includes verifying maintenance status (Rule 06 bar 2) for every
crate-shaped seed — several are expected to fail it; record, don't assume.*
