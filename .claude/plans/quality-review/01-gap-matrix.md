# QR-1 — Gap matrix and robustness findings (Stages A+B)

*Executed 2026-08-02/03 against `00-review-plan.md` (pre-committed
2026-08-02). Stage A ran delegated per the plan's execution
architecture: four reader subagents (orrery · muster family ·
muster-sdk · cross-cutting), each returning cited cells only, then one
completeness critic per slice (all four ran; their misses are folded
into the cells as "critic:" notes and into the findings — disposition
revisions are marked in place). All Stage-B tiering was done in the
main loop, which also spot-checked load-bearing citations by execution
(see Method log). Dispositions follow the fixed vocabulary: covered ·
partial · gap · N/A(reason). Line numbers are as-read on 2026-08-02.*

## Refutations first (Rule 01.3)

### R-1 — the plan's own matrix arithmetic is wrong: 40 dimensions, not 36

`00-review-plan.md` says "(36 dimensions × 7 rows)" (line 103) and its
acceptance criteria demand "all 36 × 7 cells" (line 220). The taxonomy the
same document enumerates is C1–C19 (19) + P1–P13 (13) + S1–S4 (4) + I1–I4
(4) = **40 dimensions**, so the matrix is 40 × 7 = **280 cells**. This is
exactly the Rule 01.6 class the plan warns about ("N of M" claims are the
most-copied and least-checked). Disposition: this review dispositions all
280 cells — completeness beats matching the stale count; the count is
corrected here rather than by editing the pre-committed plan (Rule 02
discipline: the plan stays as written, the correction is visible).

<!-- Further refutations of the plan's pre-stated expectations go here, before any confirmations. -->

## Stage A — the matrix

Disposition vocabulary (fixed by the plan): **covered** — plan exists,
measurable, owned · **partial** — mentioned but incomplete · **gap** —
absent though applicable · **N/A** — inapplicable, reason written. The
matrix judges the PLANS corpus; code is cited as witness. Every cell
carries a citation; line numbers are as-read on 2026-08-02.

### Overview (✓ covered · ◐ partial · ✗ gap · — N/A; evidence in the per-crate tables below)

| Dim | orrery | m-sdk | muster | m-types | m-server | m-ui | ws/CI |
|---|---|---|---|---|---|---|---|
| C1 Unit | ✓ | ◐ | ✗ | ✗ | ✗ | ◐ | ✓ |
| C2 Integration | ✓ | ✓ | ✓ | ◐ | ✓ | ◐ | ◐ |
| C3 End-to-end | — | ◐ | ✓ | — | ✓ | ✗ | ◐ |
| C4 Smoke | ◐ | ✗ | ✓ | — | ◐ | ✗ | ◐ |
| C5 Property | ✓ | ✓ | ◐ | ✗ | ✗ | ✗ | ◐ |
| C6 Parameterized | ◐ | ◐ | ✗ | — | ✗ | ✗ | ✗ |
| C7 Fixture | ✓ | ◐ | ◐ | — | ◐ | ✗ | ✗ |
| C8 Mock | ◐ | ◐ | ✗ | — | ✗ | ✗ | ✗ |
| C9 Entrait/DI | ✓ | ◐ | ◐ | — | ◐ | ✗ | ✓ |
| C10 Mutation | ✗ | ✗ | ✗ | — | ✗ | ✗ | ✗ |
| C11 Model-based | ✓ | ✓ | ✗ | — | ✗ | ✗ | ◐ |
| C12 Test generation | ✗ | ◐ | ✗ | ✗ | ✗ | ✗ | ✗ |
| C13 Regression | ◐ | ✗ | ✗ | ✗ | ✗ | ✗ | ◐ |
| C14 API testing | ✓ | ◐ | — | ◐ | ◐ | — | ◐ |
| C15 Doc testing | ✗ | ◐ | ✗ | ✗ | ✗ | ✗ | ◐ |
| C16 Snapshot/output | ✗ | ◐ | ✗ | ◐ | ◐ | ✗ | ✗ |
| C17 Feature combos | ◐ | — | — | — | — | ◐ | ◐ |
| C18 FFI | — | — | — | — | — | — | — |
| C19 Data validation | ✓ | ◐ | ◐ | ✓ | ✓ | — | ◐ |
| P1 Micro-bench | ◐ | ◐ | ✗ | — | ✗ | — | ◐ |
| P2 Macro-bench | ✓ | ◐ | ◐ | — | ✗ | — | ◐ |
| P3 CI bench | ◐ | ✗ | ✗ | — | ✗ | — | ✗ |
| P4 Load | ◐ | ✗ | — | — | ✗ | — | ◐ |
| P5 Stress | ✗ | ◐ | — | — | ✗ | — | ◐ |
| P6 Spike | — | ✗ | — | — | ✗ | — | ✗ |
| P7 Soak | ✗ | ✗ | — | — | ✗ | — | ✗ |
| P8 Cache perf | ◐ | ✗ | — | — | — | — | ◐ |
| P9 Timing consistency | ◐ | ✗ | ◐ | — | ◐ | — | ◐ |
| P10 PGO | — | ✗ | ✗ | — | ✗ | — | ✗ |
| P11 CPU profiling | ✗ | ✗ | ✗ | — | ✗ | — | ✗ |
| P12 Memory profiling | ✗ | ✗ | ✗ | — | ✗ | — | ✗ |
| P13 Network profiling | — | — | — | — | ✗ | — | ✗ |
| S1 Fuzzing | ◐ | ◐ | ✗ | ✗ | ✗ | — | ◐ |
| S2 Memory safety | ◐ | ◐ | ◐ | ◐ | ◐ | ◐ | ✗ |
| S3 Supply chain | ◐ | ◐ | ◐ | ◐ | ◐ | ◐ | ◐ |
| S4 License | ◐ | ✗ | ✗ | ✗ | ✗ | ✗ | ◐ |
| I1 Coverage | ✗ | ✗ | ✗ | ✗ | ✗ | ✗ | ✗ |
| I2 CI | ◐ | ◐ | ✗ | ✗ | ✗ | ✗ | ◐ |
| I3 Telemetry | ◐ | ◐ | ◐ | — | ✓ | ✗ | ✓ |
| I4 Usability | ◐ | ◐ | ◐ | ◐ | ◐ | ◐ | ◐ |

**Tallies (arithmetic checked, Rule 01.6): 280 cells = 23 covered +
89 partial + 106 gap + 62 N/A.** Per row (✓/◐/✗/—): orrery 9/17/9/5 ·
muster-sdk 3/20/14/3 · muster 3/10/18/9 · muster-types 1/6/9/24 ·
muster-server 4/9/24/3 · muster-ui 0/6/17/17 · workspace/CI 3/21/15/1.
Two dimensions are gap-or-worse in every applicable row: **C10 mutation**
and **I1 coverage** (I1 is ✗ in all seven). C18 FFI earned N/A in all
seven rows, each with its own written reason. The plan's "36 × 7" is
corrected to 40 × 7 by finding R-1.

### orrery (reader cells + critic pass applied)

| Dim | Disposition | Citation | Evidence |
|---|---|---|---|
| C1 | covered | plans/orrery/specs/05-testing-criteria.md:7-8 | "Unit. Interval algebra: all thirteen Allen relations, boundary cases" — witnessed tests/prop_interval.rs, Phase-2 criteria (phases/02:36,39) |
| C2 | covered | plans/orrery/phases/02-workspace.md:58-59 | "Q1-shaped read-path integration test on seeded data" pre-committed; Engine<MemoryRepo> criterion phases/04:35; critic carry-over from the prior review (CR-3): `expired_membership_effect` is called by nothing but its own tests and sweep() covers 6 of 7 kinds — the Prototype "all detectors" gate is weaker than it reads (re-homed to Phase 7, artifacts/plan-review:98-117) |
| C3 | N/A | rules/03-scope-boundaries.md:5-8 | e2e is app-scoped by design — orrery may never contain UI/network I/O; workspace e2e family is muster-owned |
| C4 | partial | plans/orrery/ROADMAP.md:5 + prds/00-orrery-engine.md:87 | PoC gate "canonical queries run against MemoryRepo" is a smoke-level statement; critic: the PRD raises it to a success criterion ("all seven canonical queries implemented and passing property tests"); no smoke suite exists, and no artifact shows all seven ran as such (Phase 2 evidences Q1-shaped only; 6a measured all 7 classes later — see F-15) |
| C5 | covered | plans/orrery/specs/05-testing-criteria.md:10-17 | "Each detector against a brute-force oracle over generated worlds"; proptest sole dev-dep; 13 proptest! blocks; oracle-independence criterion phases/03:33 |
| C6 | partial | plans/orrery/phases/03-engine-core.md:37 | one plan-stated case matrix ("25-pair containment matrix test"); no general parameterized practice written |
| C7 | covered | plans/orrery/specs/05-testing-criteria.md:35-54 | "Generated worlds must deliberately contain" enumerated fixture list incl. DST pairs, mid-chain expired edge |
| C8 | partial | docs/src/adrs/0021-datastore-selection-funnel.md:47-61 | the deliberate test double is the restrictive MemoryRepo *fake*; mock frameworks neither used nor explicitly rejected |
| C9 | covered | rules/00-non-negotiables.md:5-7 + plans/orrery/specs/04-api-surface.md:25,59 | DI via `Repository` trait + `Engine<R>` is the load-bearing seam; per-backend feature flags mandated (Rule 04) |
| C10 | gap | absent — scanned Cargo.tomls, justfiles, specs 00-05, phases, rules | no mutation-testing tool, target, or mention |
| C11 | covered | plans/orrery/specs/05-testing-criteria.md:28-29 | "salsa-derived results must equal a cold recomputation after an arbitrary mutation sequence. Fuzz the sequence" — witness tests/prop_incremental.rs (64 cases × 25-command sequences); critic: SPEC-05:19-26's separate Differential *level* (full canonical set against both repo impls) was cited by no cell — it awaits the second impl and its only door is broken (F-1) |
| C12 | gap | absent — scanned tests/, specs, justfiles | no file-driven case corpus; proptest-regressions/travel.txt is seed persistence, not case files |
| C13 | partial | plans/orrery/phases/04-travel.md:20 | "the shortcut case is pinned as a named regression test" + proptest-regressions — practice real but no written rule that refutations become pinned tests |
| C14 | covered | crates/orrery/justfile:34-44 | executable seam gate ("SEAM VIOLATION: datastore type in public API"); Beta freezes API (ROADMAP:8) |
| C15 | gap | absent — lib.rs has zero doc examples (grep); no rustdoc policy | worse: every documented gate uses nextest, which does not run doctests — a future doctest would silently never execute |
| C16 | gap | absent — scanned dev-deps, tests/, justfiles | no snapshot/golden-output tooling and no plan mention |
| C17 | partial | rules/04-rust-conventions.md:24-27 + justfile:63-66 | repo-* matrix mandated and root `matrix` recipe exists, but its repo-sqlite leg errors (verified); no plan line owns activating the matrix at Phase 7 |
| C18 | N/A | crates/orrery/src grep: no `extern` blocks | pure-Rust API; 01a screening required first-class Rust bindings precisely to avoid FFI. Contingency: cozo carries C transitives — N/A needs revisiting if cozo wins Phase 7 |
| C19 | covered | rules/04-rust-conventions.md:16-18 | "Reject inverted and … zero-length intervals at construction" — tested per phases/02:39; command-layer validation typed (03:25 H5) |
| P1 | partial | plans/orrery/specs/05-testing-criteria.md:31-33 | "seven canonical queries at 10³/10⁵/10⁶ … Regression gate" promised; no benches/, no criterion, `--bench canonical` errors (verified); no micro-bench mechanism named anywhere; critic: two pre-committed Phase-7 preconditions no cell carried — re-measure the SQLite baseline on the decision host, don't quote 1.2 ms (phases/00:157-159), and materialisation at 10⁵-row results still unmeasured, max ever 2,172 rows (01b:308-309) |
| P2 | covered | plans/orrery/specs/03-non-functional-requirements.md:22-30 | dated 10⁵ budget set names its harness (measure_alpha_budgets.rs); measured, all 7 classes (06a:183-195); 10⁶ re-measure owned by Phase 7; critic caveat: two budget classes (closure refresh at 2k+ locations, cold open) read "not yet measured" at 10⁶ — SPEC-03:16-17, phases/04:109 |
| P3 | partial | plans/orrery/specs/05-testing-criteria.md:33 + phases/06a:174-176 | "Regression gate on the budgets" stated; critic correction: a coarse mechanism DOES exist — the #[ignore] harness's in-test 10× sanity bounds catch order-of-magnitude regressions on explicit runs — but it is explicit-run-only, 10×-granular, and no phase owns a real gate; no CI |
| P4 | partial | plans/orrery/specs/03-non-functional-requirements.md:32-40 + docs/src/adrs/0015:111-123 | scale targets give a load-shape plan (Phase 7/Beta); critic correction (reader evidence revised): the concurrency/external-writer posture IS stated — ADR-0015 criterion 6 (dated 2026-08-02) requires each Stage-C candidate to state its cache-invalidation posture, "'Unexamined' is not an option" — the residual gap is a *test*, not a statement; plus ADR-0021:75-77's realistic-workload-shape requirement has no mechanism |
| P5 | gap | absent — scanned SPEC-03/05, phases | Stretch column exists (10⁷ attends, SPEC-03:38) but nothing exercises beyond-target behavior |
| P6 | N/A | rules/06-dependencies.md:33-35 | embedded synchronous library, caller-paced — no arrival-rate dimension to spike |
| P7 | gap | absent — scanned SPEC-03/05, phases 03/06a | salsa mirror + append-only violation history (SPEC-03:56) grow over long command streams; no soak/memory-growth test planned |
| P8 | partial | rules/05-observability.md:14 + plans/orrery/phases/03-engine-core.md:140 | salsa hit/miss attribute planned; cutoff probes assert re-execution counts — cache *effectiveness* tested, cache *performance* measured nowhere |
| P9 | partial | crates/orrery/tests/measure_alpha_budgets.rs:6-9 | single run, stride-sampled, p50/p95 — no warm-up/variance treatment; the 01b screening discipline (5 runs, median+max, 01b:52) was not carried into the engine harness |
| P10 | N/A | ADR-0019 via Rule 04 | orrery ships no binary; PGO is a binary concern (muster/muster-server) |
| P11 | gap | absent — scanned phases, specs, justfiles | no profiling plan; nearest is 06a's travel_best linear-scan diagnosed by reasoning, not by profile (06a:167-169) |
| P12 | gap | absent — scanned SPEC-03/05, phases | sweep persists ~47k violations/run (06a:180-182), mirror growth unmeasured; only candidate-store disk size ever measured (00-grounding:148) |
| P13 | N/A | plans/orrery/specs/03-non-functional-requirements.md:58 | "Engine performs no network I/O; travel data arrives through an explicit port" |
| S1 | partial | plans/orrery/specs/05-testing-criteria.md:8,28-29 | plan's "fuzz" means randomized property testing (proptest); no coverage-guided fuzzing for interval algebra / command apply / preview honesty, and no written exclusion |
| S2 | partial | plans/orrery/phases/02-workspace.md:24 | H1 pre-committed "without unsafe" (confirmed); no #![forbid(unsafe_code)] (grep), no miri/sanitizer plan |
| S3 | partial | rules/09-security-and-secrets.md:16-17 | cargo-audit "once CI exists" + exception protocol; no CI, no owner, no record of it ever running |
| S4 | partial | plans/orrery/phases/01a-paper-screen.md:36,53 | license screening real at selection (GPL-3.0 candidate eliminated); no continuous compliance tooling or written policy for future deps |
| I1 | gap | absent — scanned justfiles, Cargo.tomls, SPEC-05, rules, phases | no coverage tool, threshold, or plan statement |
| I2 | partial | justfile:36-38 | root `ci:` defines the gate set; .github/ absent (verified), no owner |
| I3 | partial | rules/05-observability.md:10-24 | disposition revised by critic (was covered): the span table + load-bearing `backend` attribute are real and asserted (02:98-99, 04:37) — but span-attribute privacy is "by convention today; automated span-capture check queued for RC" and designed nowhere (artifacts/phase-4-privacy.md:67), and SPEC-03:65's "structured logging with a correlation ID per command" is implemented and owned by nothing |
| I4 | partial | plans/orrery/specs/05-testing-criteria.md:64 | RC gate "docs complete" has no criterion; crate AGENTS.md stale (see findings) |

### muster-sdk (reader cells + critic pass applied)

| Dim | Disposition | Citation | Evidence |
|---|---|---|---|
| C1 | partial | plans/muster-sdk/phases/05-sdk.md:60 | "Tests: optimality proptest (two oracles), breakdown units, engine integration" — but src/ has zero inline `#[test]`; no general unit policy in SPEC-03 |
| C2 | covered | plans/muster-sdk/specs/03-testing-criteria.md:30-35 + phases/05-sdk.md:125 | `suggest_room_schedule` against `Engine<MemoryRepo>`; conflicting hold surfaces; unassigned populated. Critic addition: a second integration surface exists — batch vs engine with an idempotent-second-run gate (batch_run.rs:121-124) — SPEC-03 covers only suggest |
| C3 | partial | plans/muster-sdk/phases/05-sdk.md:12-13 | E2E stops at the engine seam; nothing states app-level E2E is muster-owned |
| C4 | gap | absent — scanned SPEC-03, phases/05, both justfiles | no smoke tier; only full `just test` |
| C5 | covered | plans/muster-sdk/specs/03-testing-criteria.md:6-18 | "Property tests (the load-bearing ones)" with named prefixes + just targets; proptest dev-dep (Cargo.toml:17-18) |
| C6 | partial | crates/muster-sdk/tests/optimality.rs:59 | proptest-generated instances serve parameterization; no table-driven statement in any sdk plan doc |
| C7 | partial | crates/muster-sdk/tests/suggest_integration.rs:14-27 | helper-built fixtures in code; plan silent on fixture corpora |
| C8 | partial | crates/muster-sdk/tests/search_quality.rs:15-17 | "Pure evaluation (no engine)" hand-rolled stub; plan silent on test doubles |
| C9 | partial | plans/muster-sdk/specs/02-api-surface.md:41 | generic-over-repo seam specified and exploited; no DI-testing statement per se |
| C10 | gap | absent — scanned specs/, phases/, justfiles, Cargo.toml | no mutation testing anywhere |
| C11 | covered | plans/muster-sdk/specs/03-testing-criteria.md:8-11 | greedy iff brute-force iff max-overlap ≤ k — two independent oracles, just-target-owned |
| C12 | partial | plans/muster-sdk/phases/05-sdk.md:24 | proptest default generation only; no generation strategy/corpus plan |
| C13 | gap | absent — grep "regression" over plans/muster-sdk/** empty | regression practice not written down in the sdk corpus |
| C14 | partial | plans/muster-sdk/specs/02-api-surface.md:37-46 | API surface specified; no API-diff gate (check-seam is orrery-only) |
| C15 | partial | justfile:57 | `doc-check` is build-only; zero doctest fences in crates/muster-sdk/src (grep); doctests unplanned |
| C16 | partial | plans/muster-sdk/specs/03-testing-criteria.md:27-28 | "byte-for-byte on serialised output" — but no serde anywhere in the crate (verified by grep, main loop); actual test asserts in-memory re-run (05-sdk.md:25) |
| C17 | N/A | crates/muster-sdk/Cargo.toml:9-18 | crate declares no `[features]`; nothing to combine |
| C18 | N/A | plans/muster-sdk/prds/00-muster-sdk.md:59 + phases/05-sdk.md:109 | no FFI exists; sole candidate CP-SAT deferred — earned N/A, with RC contingency flagged (F-class below) |
| C19 | partial | crates/muster-sdk/src/lib.rs:28-29 + specs/02-api-surface.md:56 | `InvalidRequest{reason}` + "Errors: typed (thiserror)"; no validation test family planned |
| P1 | partial | crates/muster-sdk/justfile:17-18 | `bench:` recipe exists; no benches/ dir, no criterion (verified, main loop); SPEC-03:49-51 concedes perf gates "none exist yet" |
| P2 | partial | plans/muster-sdk/ROADMAP.md:10 | RC exit "perf gates green" — gates undefined by SPEC-03's own admission; owning phase "not planned" (PLAN.md:8) |
| P3 | gap | absent — no CI exists | no CI benchmarking plan |
| P4 | gap | absent — grep "load" over sdk plans | no load-testing dimension |
| P5 | partial | plans/muster-sdk/PLAN.md:7 + phases/05-sdk.md:134-136 | "realistic-scale"/"Beta-scale instances" seed the concern; "class, scale, removal rule, seeds — none defined today"; the word "stress" appears nowhere |
| P6 | gap | absent — grep "spike" over sdk plans | unaddressed (not declared N/A either) |
| P7 | gap | plans/muster-sdk/artifacts/phase-5-polish-and-nightly.md:60-79 | nightly caretaker (recurring batch runs) exists yet no soak/long-run plan; idempotence tested for one repeat only |
| P8 | gap | absent — scanned specs, phases | `attendee_flow` precomputation landed (05-sdk.md:140-141) with no cache-performance measurement planned |
| P9 | gap | absent — scanned SPEC-02/03 | result determinism specified (specs/02:55) but timing consistency never mentioned |
| P10 | gap | absent — grep "PGO/profile-guided" | no PGO consideration (library crate; low expected relevance, but undispositioned) |
| P11 | gap | absent — grep "profil\|flamegraph" over sdk plans | stochastic-search hot loop landed with eval budgets; no profiling plan |
| P12 | gap | absent — grep "memory\|alloc" over sdk plans | no allocation/memory profiling plan |
| P13 | N/A | crates/muster-sdk/AGENTS.md:7-9 + Cargo.toml:9-15 | Rule 03: "never contain… UI, delivery"; no network dep — no network path to profile |
| S1 | partial | plans/muster-sdk/phases/05-sdk.md:10-11 | "discharged by fuzzing against two independent oracles" — this is proptest, not coverage-guided fuzzing; no cargo-fuzz plan, no written out-of-scope reason |
| S2 | partial | absent from plans; crate unsafe-free in practice (grep) | no plan statement (safe-Rust claim, miri, or exemption) |
| S3 | partial | rules/09-security-and-secrets.md:19-20 | cargo-audit "once CI exists" — conditional on a CI no phase owns |
| S4 | gap | absent — scanned justfiles, rules, sdk plans | no license-compliance check planned |
| I1 | gap | absent — grep "coverage\|llvm-cov\|tarpaulin" | no coverage tool, target, or threshold |
| I2 | partial | justfile:35-36 | `ci:` recipe defines CI's content; no CI system exists, no phase owns creating it |
| I3 | partial | rules/05-observability.md:5-7 | sdk emits tracing, never installs subscriber — code complies; but Rule 05's span table lists only orrery spans, while `sdk.suggest`/`sdk.search`/`sdk.batch` spans have LANDED with ad-hoc attributes and no spec'd table anywhere (critic: src/{suggest.rs:22-23, search.rs:68-71, batch.rs:29}); no automated gate enforces the no-subscriber prohibition |
| I4 | partial | plans/muster-sdk/phases/05-sdk.md:39 | standing plain-language-explainer criterion; no API-ergonomics/usability review planned |

**Critic additions (muster-sdk slice — statements the reader's cells missed; dispositions unchanged unless noted):**

* Public artifact claims "Thousands of random schedules were solved three
  ways at once" (artifacts/phase-5-room-suggestions.md:32-33) — the measured
  config is `ProptestConfig::with_cases(48)` (tests/optimality.rs:60).
  ~40x scale inflation, unprovenanced number (Rule 01.5). → Stage B.
* The no-feasibility-semantics boundary — violations flow one way — is
  guarded only by a *manual* "source grep at phase close + review"
  (specs/03:24-26), unlike the automated build-failing check-scope. → Stage B.
* check-scope's mechanism is a four-name denylist (`dioxus|leptos|yew|axum`,
  crate justfile:24-26) — far narrower than its stated intent; actix/rocket/
  warp/etc. would pass. → Stage B.
* Severity weights duplicated between `engine::severity_weight` and
  `objective::ViolationCost`, flagged in-plan for a shared constant
  (phases/05:80-82,159); the only executable guard is `vc.cost >= 100.0`
  while phases/05:36 claims "severity table consistent — asserted in
  integration" — over-claimed. → Stage B.
* check-xrefs is a named acceptance gate at both slice closes
  (phases/05:33,120) — a docs-consistency gate no dimension captured;
  counted under C15/docs gates at the workspace row.
* Gate runs self-record single-host provenance ("65/65 … this host,
  2026-08-02") — in-corpus acknowledgment that no CI executes the gates.
* Both proptests are pinned to 48 cases with adversarial 1-eval budget
  bounds (search_quality.rs:32) — test-design facts recorded here so the
  C5/C6 "covered/partial" cells aren't read as larger than they are.
* crates/muster-sdk/AGENTS.md is stale ("currently a compiling stub";
  optimality tests "once the solver lands" — solver landed Phase 5).
* plans/muster-sdk/research/ is an empty directory (swept; nothing missed).

### muster (reader cells + critic pass applied)

| Dim | Disposition | Citation | Evidence |
|---|---|---|---|
| C1 | gap | absent — crates/muster/src has 0 `#[test]`; scanned SPEC-03 | SPEC-03 defines only e2e_/privacy_ families; no unit-level plan statement |
| C2 | covered | plans/muster/phases/06-app.md:103,117 | pre-committed boundary greps ("empty across all four muster-family crates") + check-seam/check-scope gates |
| C3 | covered | plans/muster/specs/03-testing-criteria.md:5-15 | e2e_ family specified per stage gate; `just muster::e2e` runs it (crate justfile:11-12); critic caveat: "resolves every violation class" (specs/00:29-30, prds/00:76) implies a per-violation-class coverage matrix nobody owns → F-18 |
| C4 | covered | plans/muster/phases/06-app.md:38,194-198 | "Demo: stdout story, exit 0" pre-committed; "consumer simulation both ways" recorded |
| C5 | partial | plans/muster/specs/03-testing-criteria.md:17-21 + plans/CARRY-FORWARD.md:31 | one property planned (blast-preview honesty, Alpha-owned); no general property plan; proptest not a muster dep; critic: the PRD also pre-commits a preview *utility* criterion ("prevents at least one unintended mass change in testing", prds/00:78) distinct from the equality property — unowned → F-18 |
| C6 | gap | absent — scanned family Cargo.tomls, SPEC-03, 06-app.md | no parameterized-testing statement or tooling |
| C7 | partial | plans/muster/phases/06-app.md:188 | `build_demo_world` extraction recorded; no plan statement defining fixture strategy; critic (orrery slice): ADR-0024:57-64 *mandates* "Muster's Phase-6 specs must include transition-day expansion tests" — the engine cannot detect a mis-expanded recurrence, only consumer tests guard it — and no such tests appear in muster SPEC-03 or the phase doc → F-18 |
| C8 | gap | absent — grep mockall/mock over manifests+plans | MemoryRepo is a real backend, not a mock; plan silent |
| C9 | partial | plans/muster/specs/02-api-surface.md:8 | `MusterService<R: Repository>` matches spec; DI-for-testing never stated as such |
| C10 | gap | absent — grep mutants over manifests/justfiles/plans | no mutation tooling or plan in family |
| C11 | gap | absent — same scan | no model-based testing mention |
| C12 | gap | absent — same scan | no test-generation mention |
| C13 | gap | absent — grep "regression" over plans/muster + rules | practice exists (e2e gates, refutations-first); policy never written |
| C14 | N/A | docs/src/adrs/0025-frontend-structure.md:78-82 | muster has no HTTP surface; the wire lives in muster-server by design |
| C15 | gap | absent — root justfile doc-check is build-only | nextest skips doctests; no doctest plan (family-wide); critic: doc-check and check-xrefs ARE named phase gates (06-app.md:113) — docs build/xref quality is gated even though doctests aren't |
| C16 | gap | absent — grep insta/snapshot | demo output asserted via structured report, not snapshots |
| C17 | N/A | crates/muster/Cargo.toml | crate declares no cargo features |
| C18 | N/A | grep unsafe/extern over crates/muster/src: none | pure-Rust lib+CLI over workspace crates; no FFI boundary |
| C19 | partial | plans/muster/specs/01-data-and-roles.md:15-17 + PLAN.md:8 | disposition revised by critic (was N/A): session state "never contains: anchors, coordinates" is an app-owned-state constraint with no covering test, and the RC row gates "Parquet/CSV egress with anchors excluded" with zero test statement — both muster-owned validation surfaces → F-4 |
| P1 | gap | absent — no benches/, no criterion | nearest artifact says "a measurement, not a tuned benchmark" (measure_select.rs:8-9) |
| P2 | partial | plans/muster/phases/06-app.md:118,79 + measure_select.rs:82-84,115-120,6-8 | service-level latency measurement pre-committed (H4); critic caveats: the fixture is deliberately non-conflicting so select() under violation-heavy worlds is by construction unmeasured (Rule 01.7 class); my_schedule/events are timed but never budgeted; the harness header still claims "the assertion is intentionally the pre-committed threshold" while the shipped assertion is order-of-magnitude (p50 < 1s) → F-5 |
| P3 | gap | absent — .github/ does not exist | no CI benchmarking; no phase owns CI creation; critic: ROADMAP:13-15 already names a perf precondition ("needs salsa early cutoff or the preview is a full recompute") with no measurement criterion attached → F-3 |
| P4 | N/A | crates/muster/src/main.rs | library + one-shot CLI; sustained load arrives via muster-server |
| P5 | N/A | same as P4 | no service surface to stress |
| P6 | N/A | same as P4 | no traffic surface |
| P7 | N/A | same as P4 | no long-running process |
| P8 | N/A | rules/05-observability.md:14 | app owns no cache; derivation caching is orrery's salsa |
| P9 | partial | plans/muster/phases/06-app.md:155-165 + measure_select.rs:6-9 | p50/p95 recorded with provenance; assertion deliberately loosened to order-of-magnitude; no ongoing variance plan |
| P10 | gap | absent — grep PGO over plans/justfiles | binary exists (main.rs); PGO never dispositioned |
| P11 | gap | absent — grep flamegraph/profil over plans | sweep cost-cause inferred, not profiled |
| P12 | gap | absent — same scan | no memory/allocation profiling mention |
| P13 | N/A | crates/muster/Cargo.toml (deps: orrery, muster-sdk, anyhow) | no network I/O |
| S1 | gap | absent — grep fuzz over manifests/plans | typed API lowers value but fuzzing never dispositioned at all |
| S2 | partial | grep unsafe clean; no #![forbid(unsafe_code)] in any family lib.rs | safe-Rust practice without written policy or lint |
| S3 | partial | rules/09-security-and-secrets.md:16-17 | cargo-audit conditional on a CI no phase owns; dep bars exist (Rule 06) |
| S4 | gap | Cargo.toml:20 | license declared; no compliance checking planned |
| I1 | gap | absent — grep tarpaulin/llvm-cov | coverage never mentioned (family-wide) |
| I2 | gap | .github/ absent; root justfile:35-36 the only artifact | local `just ci` aggregate exists; CI system has no owning phase |
| I3 | partial | rules/05-observability.md:8 + crates/muster/AGENTS.md:13-15 vs phases/06-app.md:107 | rule/AGENTS say *muster* installs subscriber via figment; as landed it has no tracing/figment dep and run-dev's env knob is dead (crate justfile:17-19) → Stage B |
| I4 | partial | plans/muster/specs/00-overview.md:21-25 + specs/03:50-53 | non-functional list exists; MVP "unaided" gate acknowledged as lacking a pre-committed definition (MO-8) |

### muster-types (reader cells + critic pass applied)

| Dim | Disposition | Citation | Evidence |
|---|---|---|---|
| C1 | gap | crates/muster-types/src/lib.rs (90 lines, 0 tests, no tests/) | plan silent on testing this crate directly; exercised only via muster-server |
| C2 | partial | docs/src/adrs/0025-frontend-structure.md:93-95 | "DTO drift compile-checked only through shared muster-types" — stated mechanism; no test of its own |
| C3 | N/A | crates/muster-server/tests/e2e_http.rs:11 | types-only crate; e2e exercises its DTOs through muster-server |
| C4 | N/A | crates/muster-types/src/lib.rs:19-88 | struct/enum definitions only; nothing runnable |
| C5 | gap | absent — scanned SPEC-03, CARRY-FORWARD, manifests | serde roundtrip properties applicable to wire DTOs; nothing planned |
| C6 | N/A | crates/muster-types/src/lib.rs | derive-only crate; no logic to parameterize |
| C7 | N/A | crates/muster/src/demo.rs | fixture worlds live in muster::build_demo_world by design |
| C8 | N/A | crates/muster-types/Cargo.toml (serde, uuid only) | no behavior to mock |
| C9 | N/A | same | no injectable behavior |
| C10 | N/A | src/lib.rs: no pub fn, no impl blocks | no branching logic to mutate |
| C11 | N/A | same | no stateful behavior to model |
| C12 | gap | absent — same scan as muster C12 | contract-driven roundtrip generation conceivable; nothing planned |
| C13 | gap | same as muster row | no written regression practice |
| C14 | partial | docs/src/adrs/0025-frontend-structure.md:87 | contract single-sourced here ("load-bearing half"), tested only in muster-server; no contract-test plan naming this crate |
| C15 | gap | same as muster row | doctests never run or planned |
| C16 | partial | crates/muster-server/tests/privacy_wire.rs:1-4 + phases/06-app.md:120 | recursive JSON key allowlist pins these DTOs' output shape — lives in muster-server |
| C17 | N/A | crates/muster-types/Cargo.toml | no features declared |
| C18 | N/A | grep unsafe/extern: none | serde DTO definitions only |
| C19 | covered | plans/muster/specs/01-data-and-roles.md:37-43 + src/lib.rs:4-12 + plans/CARRY-FORWARD.md:32 | DTO anchor rule specified, mechanically tested (privacy_wire), Alpha extension owned; critic caveat: the crate also states a *cross-member* privacy contract ("a member's wire payload names no other member", src/lib.rs:11-13,53-54) that the key allowlist cannot check (person-shaped leaks inside allowed keys) → F-4 |
| P1–P13 | N/A | crates/muster-types/src/lib.rs | 90-line derive-only crate: no algorithmic surface, no binary, no service, no cache, no timing surface, no network code (13 cells, one shared reason) |
| S1 | gap | absent — no fuzz tooling anywhere | deser fuzzing of wire DTOs is the natural target and these are the types; unplanned |
| S2 | partial | same as muster row | no unsafe, no forbid lint, no written policy |
| S3 | partial | rules/09-security-and-secrets.md:16-17 | audit conditional on unowned CI |
| S4 | gap | same as muster row | — |
| I1 | gap | same as muster row | — |
| I2 | gap | same as muster row | — |
| I3 | N/A | crates/muster-types/Cargo.toml (no tracing dep) | types crate has nothing to instrument |
| I4 | partial | crates/muster-types/src/lib.rs:1-12 | shape constraints documented in code; no plan statement on API/doc ergonomics |

### muster-server (reader cells + critic pass applied)

| Dim | Disposition | Citation | Evidence |
|---|---|---|---|
| C1 | gap | crates/muster-server/src: 0 `#[test]` (grep) | only e2e_/privacy_ integration tests; plan silent on unit level |
| C2 | covered | plans/muster/phases/06-app.md:103,114 | boundary greps span all four crates; HTTP retelling of the service-seam story pre-committed |
| C3 | covered | plans/muster/phases/06-app.md:114 + tests/e2e_http.rs:40 | "repeated over HTTP (tower oneshot)" was an acceptance criterion; critic caveat: negative-path contract coverage is 1 of 5 error variants (only NotFound→404 tested; api.rs:80-93 vs e2e_http.rs:115-126) → F-18 |
| C4 | partial | plans/muster/phases/06-app.md:196-198 + crate justfile:18-20 | "booted and driven with curl… before the tests were trusted" — recorded practice, not a pre-committed criterion |
| C5 | gap | absent — scanned SPEC-03, CF Alpha rows | blast-honesty property is engine/muster-scoped; no server-level property plan |
| C6 | gap | same as muster row | — |
| C7 | partial | tests/e2e_http.rs:13-15 | reuses muster::build_demo_world via demo_state(); fixture strategy never planned in writing |
| C8 | gap | crates/muster-server/Cargo.toml:23-24 (only dev-dep: tower) | tests use the real service; no mocking tooling or plan |
| C9 | partial | tests/e2e_http.rs:16-20 | AppState-injected service + tower oneshot is a working test seam; never stated as a plan |
| C10–C13 | gap | same as muster row | — (4 cells) |
| C14 | partial | plans/muster/phases/06-app.md:114 + docs/src/adrs/0025-frontend-structure.md:87 | disposition revised by critic (was covered): the DTO half is single-sourced and gate-exercised, but violation kind/severity cross the wire as `Debug` names (api.rs:5-8,95-99 — "so the boundary grep stays meaningful") — an un-pinned contract: an engine enum rename silently changes the wire → F-18 |
| C15 | gap | same as muster row | — |
| C16 | partial | tests/privacy_wire.rs:1-4 + phases/06-app.md:120 | key-set allowlist is a real generated-output matcher with a plan row; no general snapshot practice |
| C17 | N/A | crates/muster-server/Cargo.toml | no features declared |
| C18 | N/A | grep unsafe/extern: none | boundary is HTTP, not FFI |
| C19 | covered | docs/src/adrs/0025-frontend-structure.md:80-82 + plans/muster/specs/01-data-and-roles.md:37-43 | "the privacy boundary's single enforcement point"; allowlist test green; coordinator-DTO extension Alpha-owned (CF:32) |
| P1 | gap | absent — no benches/, no criterion | — |
| P2 | gap | absent — scanned plans, justfiles | binary exists; only the service layer beneath was measured (in muster) |
| P3 | gap | same as muster row | — |
| P4 | gap | absent — scanned SPEC-03, CF ledger, ROADMAP | HTTP surface exists; load testing never mentioned at any stage |
| P5 | gap | same scan | stress testing absent though applicable |
| P6 | gap | same scan | spike testing absent |
| P7 | gap | same scan | soak testing absent; a long-running binary with a Mutex-serialized service is exactly the soak-relevant shape; critic: the Mutex is a *deliberate* single-writer ceiling and "the engine reads no clock; the binary edge owns time" (api.rs:29-32) — a determinism-by-design statement no cell had captured |
| P8 | N/A | tests/e2e_http.rs:16 (AppState = Arc<Mutex<MusterService>>) | no server-side cache; caching is orrery's |
| P9 | partial | plans/muster/specs/00-overview.md:21 | 100 ms interactive budget stated; measured only one layer down (measure_select), never at the HTTP edge |
| P10 | gap | absent — same scan as muster P10 | deployable binary; PGO undispositioned |
| P11 | gap | same as muster row | — |
| P12 | gap | same as muster row | — |
| P13 | gap | absent — scanned plans corpus | applicable and honestly lowest-priority; plan says nothing, not even a deferral |
| S1 | gap | absent — no fuzz tooling/plan anywhere | HTTP payloads are the family's prime fuzz target; undispositioned |
| S2 | partial | same as muster row | — |
| S3 | partial | same as muster row | — |
| S4 | gap | same as muster row | — |
| I1 | gap | same as muster row | — |
| I2 | gap | same as muster row | — |
| I3 | covered | plans/muster/phases/06-app.md:107 (H5) + src/telemetry.rs:1-4 + src/config.rs:12-19 + plans/CARRY-FORWARD.md:38 | subscriber install + figment exporter knob pre-committed and landed; OTLP wiring Alpha-owned — but Rule 05's text names `muster`, not this crate → Stage B |
| I4 | partial | plans/muster/phases/06-app.md:190 | typed error→status mapping recorded; no API-ergonomics or ops-usability plan before RC |

### muster-ui (reader cells + critic pass applied)

| Dim | Disposition | Citation | Evidence |
|---|---|---|---|
| C1 | partial | crates/muster-ui/src/lib.rs:81-85 | ONE unit test (`hhmm_renders_hours_and_minutes`) exists — code practice with zero plan statement (SPEC-03 has no UI criteria). Refutes the review plan's "no tests at all" seed |
| C2 | partial | crates/muster-ui/src/lib.rs:1-6 + docs/src/adrs/0025-frontend-structure.md:93-95 | compile-checked against shared DTOs is the stated mechanism; no runtime integration test or plan |
| C3 | gap | absent — scanned SPEC-03 (all stages), plans/CARRY-FORWARD.md:39 | Alpha row names REST client/dx/content deliverables with no test statement; no UI e2e planned |
| C4 | gap | crates/muster-ui/src/lib.rs:3-5 | dx entrypoint is Alpha scope; nothing runnable yet and no smoke plan for when there is |
| C5 | gap | same scan as C3 | — |
| C6 | gap | same as muster row | — |
| C7 | gap | absent — the one test uses inline values | no fixture use or plan |
| C8 | gap | absent — scanned CF Alpha rows | mocking the REST client will be needed at Alpha; unplanned |
| C9 | gap | absent — no DI/test-seam statement for UI | component props are framework-standard, not a planned seam |
| C10–C13 | gap | same as muster row | — (4 cells) |
| C14 | N/A | crates/muster-ui/src/lib.rs:3-4 | consumes the HTTP contract; owns no HTTP surface (REST client is Alpha scope) |
| C15 | gap | same as muster row | — |
| C16 | gap | absent — scanned manifests/plans | component render/snapshot testing (dioxus ssr) applicable; nothing planned |
| C17 | partial | crates/muster-ui/Cargo.toml:14-19 + docs/src/adrs/0025:96-97 + justfile:48 | critic-sharpened: NEITHER documented configuration is gate-exercised — the bare library (which ADR-0025 and lib.rs claim "checked by workspace CI on the host") never runs bare because gates pass `--all-features`, and the `web`/WASM build only exists via `dx serve`; what gates actually exercise is web-feature-on-host, a configuration documented nowhere → F-11 |
| C18 | N/A | grep unsafe/extern: none | WASM bindings are toolchain-generated (dioxus/wasm-bindgen); no hand-written FFI |
| C19 | N/A | docs/src/adrs/0025-frontend-structure.md:80-82 | validation enforced server-side by design; UI has no parsing code yet |
| P1–P13 | N/A | crates/muster-ui/src/lib.rs (86-line component skeleton) | structure-only: no algorithmic surface, no binary (dx entrypoint Alpha), no HTTP surface, no cache, WASM target (13 cells, one shared reason) — but NO UI perf plan exists for Alpha either (wasm profiling undispositioned) → Stage B |
| S1 | N/A | same | no input-parsing surface yet |
| S2 | partial | same as muster row | — |
| S3 | partial | docs/src/adrs/0025-frontend-structure.md:82,91-92,101-103 | dioxus small-team risk recorded with structural mitigation; critic: plus an explicit pin policy ("pinned 0.7.x; do not track 0.8-alpha") and a planned migration budget (2026–27) — the family's best supply-chain statement; audit still CI-conditional |
| S4 | gap | same as muster row | — |
| I1 | gap | same as muster row | — |
| I2 | gap | same as muster row; also lib.rs:5 says "workspace CI" though none exists | — |
| I3 | gap | crates/muster-ui/Cargo.toml (no tracing dep) | Rule 05 says libraries instrument; UI instrumentation never mentioned for any stage |
| I4 | partial | plans/muster/specs/00-overview.md:24 + PLAN.md:8 + plans/CARRY-FORWARD.md:95 | "visually distinct" requirement + RC accessibility exist, but accessibility "level TBD at entry" is unmeasurable as written |

**Critic additions (muster family slice):** most misses are folded into
cells and findings above (F-4 b/c/d, F-5, F-6 b, F-11, F-13, F-18).
Remaining item with no cell home: the ADR-0003 window trap is "worth a
line in the eventual frontend guidelines" (phases/06-app.md:56-58) — a
planned defect-prevention guideline for every schedule consumer with no
owner or stage; queued for the QR-2 muster-ui 'to add' list. Also
verified by the critic: muster-server's ORRERY_OTEL_EXPORTER knob is
live (config.rs:38-42) — only muster's twin is dead.

### workspace/CI (reader cells + critic pass applied)

| Dim | Disposition | Citation | Evidence |
|---|---|---|---|
| C1 | covered | justfile:47-48 + plans/NEXT-SESSION.md:20-23 | `test: cargo nextest run --workspace --all-features` + standing "91 tests green on main" invariant |
| C2 | partial | justfile:47-48 | single workspace sweep; no cross-crate integration policy distinct from the sweep |
| C3 | partial | crates/muster/justfile:10-11 + crates/muster-server/justfile:9-10 | e2e_ families run in the sweep; no workspace e2e policy names them |
| C4 | partial | justfile:35-36, 22-27 | `ci:` + `doctor` toolchain check; no named smoke policy or pre-push hook |
| C5 | partial | plans/PLAN.md:77-78 + ROADMAP.md:43 | property+oracle is gate material, but `just test-prop` is broken — no crate defines a `proptest` feature (reproduced by execution, main loop + reader independently); critic: workspace Cargo.toml:52 pins `proptest = "1"` as a dependency — the recipe confused a dep for a cargo feature |
| C6 | gap | absent — scanned rules, PLAN, ROADMAP, justfiles, Cargo.lock | no parameterized-testing policy or tooling |
| C7 | gap | absent — scanned root plans + justfiles | fixtures exist per-crate; no workspace fixture-management statement |
| C8 | gap | absent — Cargo.lock grep mockall/mockito empty | no mocking tooling or policy |
| C9 | covered | rules/00-non-negotiables.md:5-8 + 25-33 (00b) + rules/04-rust-conventions.md:29-32 | repository-trait seam + repo-memory default; critic: Rule 00b separately mandates *executable* enforcement — assertions "worthless if they are not executable" |
| C10 | gap | absent — scanned rules/plans/Cargo.lock | mutation testing appears only in the review's own plan |
| C11 | partial | plans/PLAN.md:147-148 + docs/src/adrs/0021:17-20,70-74 | Phase-7 "Differential testing against MemoryRepo and each other"; critic: spec-mandated by accepted ADR ("specs already require two repository implementations"; second impl "a hard gate on Orrery Beta") — yet its only documented door, `just differential`, fails (no `differential` feature; reproduced) |
| C12 | gap | absent — scanned rules/plans/justfiles | no test-generation tooling or policy |
| C13 | partial | plans/NEXT-SESSION.md:20-23 + 48-50,59-61 | green-baseline-on-main plus (critic) one standing regression obligation: `refresh_after` string-matches command kinds and every new command "MUST be added to the match by hand (and tested)"; still no general bug→test rule |
| C14 | partial | crates/orrery/justfile:34-44 + plans/CARRY-FORWARD.md:96 + docs/src/adrs/0022:48 | check-seam guards the API; critic: check-scope ("fails the build" on a scope violation) is a second boundary gate no cell had captured; Beta API-freeze diff tooling owed — "grep fallback is not an API-diff" |
| C15 | partial | justfile:57-58 + docs/justfile:13-15 | `doc-check` in `ci`; `docs::check-links` belongs to no gate; no doctest policy |
| C16 | gap | absent — Cargo.lock grep insta empty | no snapshot-testing tooling or policy |
| C17 | partial | rules/04-rust-conventions.md:29-32 + justfile:68-72 | repo-* matrix mandated and `matrix` recipe exists, but fails today — "orrery does not contain feature repo-sqlite" (reproduced); nothing scheduled to run it |
| C18 | N/A | Cargo.toml:15 | no FFI in workspace members; Python harness is pre-implementation research; datastore bindings live in excluded tools/screening — revisit at Phase-7 backend adoption |
| C19 | partial | rules/04-rust-conventions.md:19-20 | construction-time rejection mandated workspace-wide; no test policy verifies it |
| P1 | partial | justfile:54-55 + crates/orrery/justfile:24-26 | bench recipes exist; zero bench targets in any crate; no harness pick (no criterion/divan in Cargo.lock) |
| P2 | partial | plans/ROADMAP.md:44-45 + evidence/README.md:46-52,54-55 | scale gates 10⁵/10⁶ are stage gates; 10⁵ measured via ignored harness; 10⁶ unmeasured; critic: four recorded measurement-validity debts still open (count(*) never materialises results; Python-not-Rust bindings; uniform synthetic distribution; WSHORTEST "resolve before relying on Q6 at scale") |
| P3 | gap | absent — no CI exists | nothing plans continuous benchmarking |
| P4 | partial | plans/PLAN.md:148-151 + docs/src/adrs/0021:75-77 | disposition revised by critic (was gap): Phase 7 commits to benchmarks "through the real engine — mixed read/write, transactions, salsa interleaving, realistic result sizes" — a realistic-workload commitment, though nothing covers the HTTP edge and no thresholds exist |
| P5 | partial | plans/CARRY-FORWARD.md:78 + evidence/README.md:53 | Phase-7 concurrency/mixed-read-write row exists; no thresholds pre-committed |
| P6 | gap | absent — scanned plans | orrery_spike.py is a research spike, not spike testing; no spike-load policy |
| P7 | gap | absent — scanned rules/plans | no soak/endurance policy |
| P8 | partial | rules/05-observability.md:20,24 + plans/CARRY-FORWARD.md:76 | salsa hit/miss + `backend` span attributes mandated; "cached/prepared query paths" Phase-7 row; no cache-perf test policy |
| P9 | partial | evidence/run_all.sh:15 + rules/01-evidence-standards.md:6-27 + plans/PLAN.md:172-179 + plans/PHASE-TEMPLATE.md:11-38 | critic: the pre-commitment protocol (hypothesis before measurement; criteria pre-committed; refutations-first Results) is strong workspace policy — but "should reproduce within run-to-run noise" has no tolerance, no variance/iteration policy exists, and run_all.sh:8 installs ladybug unpinned (reproduction not version-locked) |
| P10 | gap | absent — no [profile.*] sections in Cargo.toml at all; scanned rules/plans | no PGO mention |
| P11 | gap | absent — scanned rules/plans/justfiles | no CPU-profiling policy or tooling |
| P12 | gap | absent — scanned rules/plans | evidence harness never measured memory; no memory-profiling policy |
| P13 | gap | absent — scanned plans | applicable (muster-server wire exists); no network-profiling policy |
| S1 | partial | plans/ROADMAP.md:45 | Orrery Beta gate "incremental fuzz green" — no tooling, corpus, targets, or owning row (absent from CARRY-FORWARD Phase-7 dossier and PLAN.md Phase 7) |
| S2 | gap | absent — scanned rules/plans; justfile:25 notes no-rustup host | no miri/sanitizer mandate; nightly tooling host-blocked; no container plan in binding corpus |
| S3 | partial | rules/09-security-and-secrets.md:19 + rules/06-dependencies.md:25-30 + rules/08-git-and-commits.md:3-5 | cargo-audit "once CI exists" — dormant; critic: Rule 06's vetting bars (maintained, no transitive runtime, no I/O outside the trait) are real supply-chain policy, and release-please/Conventional-Commits automation is likewise committed with no CI to run it |
| S4 | partial | Cargo.toml:20 + plans/CARRY-FORWARD.md:74 | workspace license declared + Cozo vendoring legal-precondition row; no dependency-license policy or tooling (no deny.toml) |
| I1 | gap | absent — grep coverage/tarpaulin/llvm-cov hits only the review's own plan | no coverage measurement or aggregation policy |
| I2 | partial | justfile:35-36 + AGENTS.md:22 + docs/src/adrs/0025:97 | named `ci` entrypoint exists and is documented; .github/ absent, no CI config repo-wide (verified), no plan row owns standing CI up; critic: an *accepted ADR* asserts "CI runs cargo/nextest as before" — a written false premise |
| I3 | covered | rules/05-observability.md:1-33 + docs/src/adrs/0022-dependency-baseline.md:20-25 | libraries-instrument/binaries-configure split + span table + load-bearing `backend` attribute, dependency-enforced |
| I4 | partial | plans/ROADMAP.md:47,57,68,107-111 + plans/CARRY-FORWARD.md:92-94 + rules/08:12-14 + plans/NEXT-SESSION.md:66-70 | human-outcome MVP gates exist but not measurable as written; critic: the gate *machinery* is stronger than the reader's cells showed — merge `--no-ff` only when acceptance criteria green, the full standing gate suite enumerated, PHASE-TEMPLATE forcing pre-committed criteria + refutations-first Results, and ADR-0021's second-impl hard gate on Orrery Beta |

**Critic additions (cross-cutting slice — statements the reader's cells missed):**

* **The taxonomy has no privacy-testing dimension.** The corpus's hardest
  gate class — Rule 00.6 "asserted by an automated test — not a review
  checklist" (rules/00:20-22), Rule 09's `just orrery::test-privacy`
  asserting no coordinate crosses the boundary "in any payload, log, or
  error" (rules/09:14-16) — has no row in the 40-dimension matrix. The
  privacy evidence is scattered across C19/C16/S-cells. Recorded here as a
  review-added taxonomy observation for Stage C (a candidate 41st row).
* plans/README.md:25-30 already commits the durable strategy home to
  `plans/TESTING-STRATEGY.md` with per-crate criteria in product testing
  specs — Stage E's landing rule is pre-agreed in the corpus.
* ADR-0016:31-35,62 commits determinism-by-construction (blake3 derived-edge
  identity; salsa state a rebuildable cache) — uncited determinism
  commitments relevant to the RC "deterministic rebuild" gate.
* evidence harness determinism: fixed seed `random.Random(42)`
  (orrery_spike.py:29) and post-load `verify_tier_constraints`
  (orrery_schema.py:1,47) — measurement-discipline positives.
* plans/README.md:45-49's stale thin-spots table is itself an *admitted*
  cross-crate testing-spec gap statement (content, not just staleness).
* **Second taxonomy observation (muster-family critic): no ops-validation
  dimension.** The muster RC row gates "backup/restore" (specs/03:48,
  ROADMAP:10) — a disaster-recovery validation gate with no matrix row to
  live in. Recorded for Stage C alongside the privacy-dimension candidate.

## Stage B — tiered robustness findings

Tiering is the main loop's judgment over every covered/partial cell,
using the prior review's question set: measurable as written (MO-8)?
owned by a phase (CR-4)? does the documented door open (MO-10)? did a
claim harden while propagating (Rule 01.4)? Tier definitions: **Critical**
— a stage gate or non-negotiable is falsified, unfoundable, or silently
narrower than written; **Moderate** — an under-specified promise that
will bite at a stage entry or a latent trap that will misreport quality;
**Low** — drift and hygiene. Every Critical/Moderate carries a
disposition line (what QR-2/QR-3 should do with it).

### Further refutations of the review's own seeds (Rule 01.3 — before any findings)

**R-2 — "muster-ui has no tests at all" is wrong.** It has exactly one
(`hhmm_renders_hours_and_minutes`, crates/muster-ui/src/lib.rs:81-85).
Narrow, but the seed was stated as fact in the pre-committed plan and
would have propagated. The substantive claim (no *planned* UI testing at
any stage) survives.

**R-3 — "SPEC-05's Benchmark level is otherwise unowned" is stale by one
day.** Since the 2026-08-02 amendments, the 10⁵ leg is owned and measured
(orrery/SPEC-03:22-30 names measure_alpha_budgets.rs; Phase-6a results),
and Phase 7 owns the 10⁶ re-measure (plans/PLAN.md:146-151). What remains
unowned is narrower: the seven-query *micro-bench mechanism* and the
*regression-gate mechanism* (see F-3).

**R-4 — the taxonomy's seed note "P5 Stress (muster-sdk instance scale)"
overstates the corpus.** The word "stress" appears nowhere in the sdk
plans; what exists is undefined churn-gate scale language ("class, scale,
removal rule, seeds — none defined today", plans/muster-sdk/PLAN.md:7).

**R-5 — the plan's expectation that I2 (CI) is the highest-leverage single
item: CONFIRMED, with a caveat.** Confirmed: cargo-audit, release-please,
P3, I1-aggregation, and multi-host gate provenance are all dormant on it,
and phase docs self-record "this host" provenance for every gate run. The
caveat: the highest leverage *per unit cost* is F-1 (the broken doors) —
five one-line fixes that restore the documented gates today, no
infrastructure required.

**Seed count check (Rule 01.6):** "91-test suite" — verified as 92 test
functions repo-wide (91 green + 1 deliberately-ignored measurement
harness; orrery's share 77+1). The seed number was the green count, not
the function count; both are now recorded.

### Critical

**F-1 (MO-10) — the corpus's documented quality doors fail: five hard
errors, two silent no-ops, all reproduced by execution this session.**
`just test-prop` (justfile:51-52), `just orrery::test-detectors` and
`test-incremental` (crates/orrery/justfile:11,15) all pass
`--features proptest` — no crate defines that feature (proptest is a
*dependency*, workspace Cargo.toml:52; the recipes confused a dep for a
feature); `just matrix`'s repo-sqlite leg (justfile:68-72) errors — the
feature doesn't exist yet and no plan line owns activating the matrix at
Phase 7; `just differential` (justfile:75-76) errors on a nonexistent
`differential` feature — and this is the *only documented door* of the
differential-testing mechanism an accepted ADR makes a hard Orrery-Beta
gate (ADR-0021:17-20,70-74). `just orrery::bench-canonical` errors ("no
bench target named `canonical`"); root/crate `bench` recipes succeed
while running zero benchmarks. Compounding: even with the feature bug
fixed, `test-detectors`' filter `test(detect_)` matches zero tests
(detector tests are `prop_matches_oracle`/`oracle_*`), and `test-prop`'s
filter `test(prop_)` excludes every SDK property test (`optimality_`,
`monotone_`). Nobody noticed any of this — which is itself evidence for
F-2. *Disposition: QR-2 implement-now items — recipe fixes are one-line;
naming-convention decision (`prop_` prefix policy vs filter union) needed
first; matrix/differential recipes get "arrives with Phase 7" guards or
comments rather than silent failure.*

**F-2 (CR-4/keystone) — CI does not exist, nothing owns creating it, and
the corpus increasingly writes as if it exists.** `.github/` absent, no
CI config repo-wide (verified); Rule 09 conditions cargo-audit on "once
CI exists"; release-please/Conventional-Commits automation has no runner;
P3 and I1-aggregation are dormant; an *accepted ADR* states "CI runs
cargo/nextest as before" (ADR-0025:97) and muster-ui's lib.rs says
"workspace CI" — written false premises (Rule 01.4 hardening in the
canonical direction: inference → fact). Every gate run in every phase doc
carries self-recorded single-host provenance ("65/65 … this host,
2026-08-02"). *Disposition: QR-2 defines the I2 RR&P stage (platform
confirmation is owner touchpoint #2) and makes CI-creation an owned
ledger row; until it lands, phase docs keep recording single-host
provenance honestly.*

**F-3 (MO-8) — the performance-gate class is undefined at both ends of
the stack.** SDK: the RC gate says "perf gates green" while SDK SPEC-03
admits "the SDK perf gates the RC gate references (none exist yet)" and
the owning phase is "not planned" (plans/muster-sdk/ROADMAP.md:10,
specs/03:49-51, PLAN.md:8) — a stage gate defined by reference to
nothing. Orrery: SPEC-05:33's "Regression gate on the budgets" has, as
its only existing mechanism, the #[ignore] harness's explicit-run 10×
sanity bounds (a coarse gate, honestly recorded — critic correction to
the reader's "no mechanism"); no micro-bench harness or tool choice
exists anywhere (P1 cells). Named perf preconditions accumulate without
criteria: the Alpha preview "needs salsa early cutoff or the preview is
a full recompute" (muster ROADMAP:13-15); Phase 7 must re-measure the
SQLite baseline on the decision host rather than quote 1.2 ms
(phases/00:157-159); result materialisation above 2,172 rows has never
been measured (01b:308-309). *Disposition: QR-2 — a perf-gating RR&P
stage (criterion vs divan; gate mechanism; where regression baselines
live), explicitly sequenced behind I2 for the CI-gated part and behind
nothing for the local baseline part; the Phase-7 preconditions become
ledger-visible ordering edges.*

**F-4 (MO-8, non-negotiable class) — the written privacy scope has
outrun the executable scope in six places.** Rule 00.6 requires
anchors-never-cross "asserted by an automated test — not a review
checklist". Shipped: orrery's two privacy_ tests + muster-server's
privacy_wire.rs cover engine-boundary verdicts and serialized wire
payloads. Written but not asserted anywhere: **(a)** log-channel and
error-channel coverage (Rule 09:14-16 "in any payload, log, or error";
muster SPEC-03:23-29 promises log lines and errors); **(b)** the RC
Parquet/CSV egress gate "anchors excluded" (muster PLAN.md:8 — a whole
egress surface, zero test statement); **(c)** the cross-member wire
contract ("a member's payload names no other member",
muster-types/src/lib.rs:11-13 — the key allowlist cannot see
person-shaped leaks inside allowed keys); **(d)** app-owned session
state "never contains: anchors, coordinates" (muster specs/01:15-17);
**(e)** the automated span-capture privacy check "queued for RC" and
designed nowhere (orrery artifacts/phase-4-privacy.md:67); **(f)** the
RC end-to-end "no anchor coordinate in any coordinator-facing payload"
commitment, unowned (phase-4-privacy.md:101-104). *Disposition: QR-2
implement-now for the cheap extensions (log/error channel assertion;
session-state check), RR&P or owned ledger rows for the RC-scoped ones
(egress, span capture, e2e), and honest dated narrowing wherever the
promise is deliberately deferred — Rule 01.2 either way; flag all six in
the Muster-Alpha pre-commitment since CF:32 already owns the
coordinator-DTO extension.*

### Moderate

**F-5 (P9) — the measurement-variance discipline regressed and nobody
wrote that down.** The 01b screening harness pre-committed ≥3 runs (ran
5) with median+max; measure_alpha_budgets and measure_select are
single-run with stride sampling; evidence/run_all.sh promises
reproduction "within run-to-run noise" with no tolerance; the ladybug
install is unpinned. The knife-edge H4 precedent (a 6% margin on a
single run) makes this concrete, and the entire budget-gate edifice
(SPEC-03 NFRs, ROADMAP scale gates) rests on these numbers. Critic
additions: measure_select's header still claims "the assertion is
intentionally the pre-committed threshold" while the shipped assertion
is order-of-magnitude (p50 < 1s) — header/code contradiction; its
fixture is deliberately non-conflicting, so select() under
violation-heavy worlds is by construction unmeasured (Rule 01.7); and
my_schedule/events are timed but never budgeted. *Disposition: QR-2
implement-now — a written variance policy (runs, warm-up, statistic,
tolerance) in TESTING-STRATEGY.md, applied to the existing harnesses;
fix the measure_select header as a dated correction; cheap and pre-CI.*

**F-6 (Rule 01.5/01.6) — artifact summary claims drift from their
measurements, twice.** (a) "Thousands of random schedules were solved
three ways at once" (plans/muster-sdk/artifacts/phase-5-room-suggestions.md:32-33)
vs the measured config `ProptestConfig::with_cases(48)`
(tests/optimality.rs:60; both sdk proptests pinned to 48 cases) — ~40x
inflation. (b) "Everything above is asserted by 72 automated tests"
(plans/muster/artifacts/phase-6-prototype-member-flow.md:76) — 72 is the
whole-workspace count including orrery, not tests of that artifact's
claims. Exactly the most-copied, least-checked claim class Rule 01.6
warns about, in the plain-language artifacts — the document class
written to be quoted. *Disposition: QR-3 dated corrections in both
artifacts; QR-2 adds a written proptest case-budget rationale (48 is
defensible; undocumented is the defect).*

**F-7 (Rule 01.4) — "fuzz" means proptest everywhere it appears, and
true fuzzing is dispositioned nowhere.** Orrery SPEC-05 "fuzz the
sequence" = proptest; sdk phase doc "48 fuzz cases", "discharged by
fuzzing" = proptest; ROADMAP's Orrery-Beta gate "incremental fuzz green"
most plausibly means the existing prop_incremental family but no document
says which tests constitute the gate; no corpus document either plans
coverage-guided fuzzing (cargo-fuzz class, S1) or writes a reason for
its absence — despite wire payloads (muster-server) and interval/command
surfaces (orrery) being prime targets and the host constraint being a
known open question. *Disposition: QR-2 — fuzzing RR&P stage (the plan's
Appendix already seeds afl.rs/cargo-fuzz/honggfuzz; must answer the
macOS/no-rustup constraint); plus a one-line gate definition for
"incremental fuzz green".*

**F-8 (MO-8) — several guards are weaker than the sentences describing
them.** check-scope is a four-name denylist (`dioxus|leptos|yew|axum`) —
actix/rocket/warp/etc. pass a gate described as "fails if a UI dependency
leaks in"; the sdk's violations-flow-one-way boundary is guarded by a
*manual* "source grep at phase close" unlike its automated siblings;
check-seam's grep fallback is line-anchored on `^\s*pub` (misses
re-exports and multi-line signatures) and phase docs record "check-seam
green" without noting which arm ran; the sdk severity-weight duplication
is guarded only by `vc.cost >= 100.0` while the phase doc claims
"severity table consistent — asserted in integration". *Disposition:
QR-2 implement-now items (strengthen or honestly re-describe each guard);
the check-seam fallback caveat also feeds the C14/API-diff RR&P.*

**F-9 (Rule 05 drift) — the telemetry plan and the landed architecture
disagree, undated.** Rule 05:8 and muster SPEC-02:39 assign subscriber
installation to `muster`; it landed in `muster-server` (H5, telemetry.rs)
and muster has no tracing/figment dep at all — its `run-dev` env knob is
dead (crate justfile:17-19; muster-server's twin knob is live via
config.rs:38-42). Meanwhile sdk spans (`sdk.suggest`/`search`/`batch`)
landed with ad-hoc attributes and no spec'd table (Rule 05's table is
orrery-only); muster-ui instrumentation is unmentioned for any stage;
and orrery SPEC-03:65's "structured logging with a correlation ID per
command" is implemented and owned by nothing. The `backend` attribute —
load-bearing for Phase 7 — is implemented and asserted (confirmed
strength). *Disposition: QR-3 dated amendment to Rule 05/SPEC-02
(subscriber owner = muster-server; add the sdk span table; disposition
UI telemetry and the correlation-ID promise); delete or rewire the dead
knob.*

**F-10 (MO-8, acknowledged) — the human-outcome and completeness gates
remain unmeasurable as written, though ownership now exists.** Beta "full
track", MVP "unaided" / "organiser accepts a suggestion unedited", RC
accessibility "level TBD at entry", RC "docs complete" (no criterion),
RC "deterministic rebuild verified" (the rebuild operation designed
nowhere — MO-9 carryover). All are CF-owned at stage entry (CF:91-97) —
the defect is that none has a pre-committed definition yet and Muster
Alpha is next. *Disposition: QR-2 confirms these stay owned and adds the
trial-protocol question to owner touchpoint #3; no silent tightening.*

**F-11 (C17/I2) — the only gate configuration that runs is
`--all-features`; every documented alternative is unexercised.**
muster-ui: NEITHER documented configuration is gate-checked — the bare
library ("checked by workspace CI on the host", ADR-0025:96-97 and
lib.rs:5-6) never runs bare because gates pass `--all-features`, and the
web/WASM build exists only via `dx serve`; what gates actually exercise
is web-feature-on-host, documented nowhere. The repo-* matrix has no
runnable second leg (F-1) and no owner. muster-types and muster-ui have
no justfile, no per-crate quality entrypoints (root justfile `mod`s only
muster/muster_server), and no AGENTS.md (the Rule 07 skeleton mandates
"how to run its tests and benches" per crate) — with no statement that
any of this is deliberate. *Disposition: QR-2 — feature-matrix policy
item (cargo-hack is a review-added candidate per the plan's seed
protocol) + per-crate entrypoint/AGENTS decision.*

**F-12 (C15) — the doctest blind spot is structural.** Zero doctests
exist; every documented gate runs nextest, which does not execute
doctests; `doc-check` is `cargo doc` (build only). A future rustdoc
example would pass every gate while never executing — the trap is armed
by the tooling choice itself. *Disposition: QR-2 implement-now — one
`cargo test --doc` recipe line + a rustdoc-examples policy sentence in
TESTING-STRATEGY.md.*

**F-18 (CR-4) — specified-but-untested behavioral contracts, seven
instances.** Written, testable, and owned by no test anywhere: **(a)**
ADR-0024:57-64's *mandate* that Muster's Phase-6 specs include
transition-day recurrence-expansion tests (the engine cannot detect
mis-expansion; only consumer tests guard it) — absent from muster
SPEC-03; **(b)** the lapsed-coordinator temporal-authz property ("loses
power the moment the membership window ends", muster specs/01:33-35);
**(c)** the per-violation-class resolvability matrix implied by
"resolves every violation class" (specs/00:29-30, prds/00:76); **(d)**
the error→status contract, tested for 1 of 5 variants
(muster-server api.rs:80-93); **(e)** violation kind/severity crossing
the wire as un-pinned `Debug` names — an engine enum rename silently
changes the contract (api.rs:5-8,95-99); **(f)** the preview *utility*
criterion ("prevents at least one unintended mass change in testing",
prds/00:78), distinct from the equality property; **(g)** carried from
the prior review (CR-3): `expired_membership_effect` is exercised only
by its own tests and sweep() covers 6 of 7 violation kinds — already
re-homed to Phase 7, recorded here so the matrix carries it.
*Disposition: QR-2 — these become the core of the per-crate 'to add'
lists; (a) and (b) are Muster-Alpha-entry material, (e) wants a pinned
wire-name test now.*

### Low

**F-13 — context-file and README staleness cluster (twelve instances
across eight files, one cause: no refresh trigger).** plans/README
thin-spots table (understates coverage both ways); sdk README
"deliberately thin"; muster README stale twice (specs list; Q15 "OPEN"
though closed by ADR-0025); orrery AGENTS.md stale twice (Phase-2 module
map; Q14 "still open" though closed by ADR-0024); muster AGENTS.md stale
three times ("compiling stub"; claims subscriber ownership; "the
automated privacy tests live in orrery" — they live in muster-server);
sdk AGENTS.md stale twice ("compiling stub"; "once the solver lands");
root AGENTS.md ("cargo-nextest optional" vs hard-required by every
recipe); muster-ui lib.rs ("workspace CI"). *Disposition: QR-3 sweep-fix
with dated lines; QR-2 considers a "context files refreshed at phase
close" checklist line in the phase template.*

**F-14 — the 100 ms budget's definition shifted while propagating,
conservatively.** SPEC-00:21 scopes it "within 100 ms of the engine
returning"; H4/measure_select measured the whole select() including the
engine sweep. Gate-as-measured is *stricter* than gate-as-written, so no
falsification — but the definitions should be reconciled before the HTTP
edge (which SPEC-00 arguably scopes) is measured (muster-server P9 gap).

**F-15 — the PoC gate "canonical queries run against MemoryRepo" was
never evidenced beyond the Q1-shaped test.** Overtaken by Phase 6a's
seven-class measurement at 10⁵, so historical; recorded because Rule 01
says gates don't get to quietly pass.

**F-16 — proptest case budgets are pinned low (48) corpus-wide with no
written rationale;** the adversarial 1-eval budget bound is likewise
undocumented test design. Fine numbers, absent provenance.

**F-17 — no `[workspace.lints]` or `[profile.*]` sections exist;** lint
severity lives only in justfile clippy flags, and the release-profile
pre-pinning rule (measure_ harness headers) is practice without policy.

**F-19 — the UUIDv7 insert-locality premise was never benchmarked.**
ADR-0022:29-33 chose v7 because v4 "degrade[s] exactly the access path
the engine depends on" — asserted from RESEARCH-0003 inference, never
measured (Rule 01.4 labeling: plausible inference recorded as decision
driver). Fine decision, mislabeled evidence class; a Phase-7 bench
candidate at most.

### Confirmed strengths (build on, don't duplicate — Stage C input)

Property+oracle discipline with oracle-independence criteria; the
privacy_ family and its single-enforcement-point design (ADR-0025);
measure_ harnesses with pre-pinned profiles and named provenance;
check-seam/check-scope as *executable* boundary statements; MemoryRepo
restrictive-fake strategy (ADR-0021) with Rule 00b executable
constraints; salsa-vs-cold-recompute model-based family
(prop_incremental); pre-commitment protocol (PHASE-TEMPLATE + Rule 01)
including refutations-first Results sections — which this review's own
seed errors (R-2..R-4) validate as necessary; `backend` span attribute
implemented from day one; e2e_ families at both app seams; DTO privacy
rule mechanically tested (privacy_wire); dated corrective notes already
present at both sites of the one previously-caught hardened claim.

## Method log (main-loop verification)

Execution architecture as run: four reader subagents (orrery · muster
family · muster-sdk · cross-cutting), then four completeness critics
(one per slice), all eight returned; the critics produced 5 disposition
revisions (orrery I3, P3, P4 evidence; muster C19; muster-server C14;
muster-ui C17 sharpened) and ~50 cell/finding refinements, which is the
strongest available argument that delegated reading without a critic
pass would have been hope, not method. The cross-cutting reader
reproduced every broken door by execution rather than by reading.
Refutations of "covered" claims were specifically hunted (three
covered→partial downgrades resulted).

Spot-checks executed inline this session (not delegated):

* `specs/03-testing-criteria.md:27-28` byte-for-byte gate — read directly;
  `grep -rn serde crates/muster-sdk/` empty. Confirmed unimplementable as
  written.
* `cargo tree --workspace --features proptest` → "error: none of the
  selected packages contains this feature: proptest". Confirms `just
  test-prop` (justfile:51-52) is broken; additionally its filter
  `test(prop_)` would exclude every SDK property test (`optimality_`,
  `monotone_` prefixes — verified by grep) even after the feature error
  is fixed. orrery's property tests ARE `prop_`-prefixed.
* `[features]` sections of all seven Cargo.tomls listed: only orrery
  (`repo-memory`) and muster-ui (`web`) declare features. Confirms both
  the `matrix` recipe failure (no `repo-sqlite` yet) and muster-sdk C17
  N/A.
* `ls crates/muster-sdk/benches` → absent; `benches/` absent in all crates.
* `.github/` absent; no CI config repo-wide (reader-verified by pruned
  find; consistent with the plan's pre-stated constraint).

## Acceptance-criteria self-check (QR-1 scope only)

| Criterion (from 00-review-plan.md) | Status |
|---|---|
| Matrix completeness — all cells dispositioned with evidence pointers; N/A cells carry reasons | Met, at 40×7=280 (not 36×7 — R-1); every cell cited; all seven C18 N/As carry per-crate reasons |
| Findings integrity — tiered; refutations-first; every Critical/Moderate has a disposition line; N-of-M arithmetic checked | Met: R-1..R-5 precede findings; F-1..F-4 Critical, F-5..F-12+F-18 Moderate, F-13..F-17+F-19 Low, each C/M with a disposition line; tallies and counts checked (280; 92 tests; 12 staleness instances; 1-of-5 error variants; 6-of-7 sweep kinds) |
| Seed triage · RR&P shape · Ordering · Semver · Landing · check-xrefs/artifact gates | Not QR-1 scope (Stages C–E); the plan's Appendix seeds remain untriaged by design until QR-2 |

Stage-B findings feed QR-2 as follows: implement-now candidates F-1,
F-4(a,d), F-5, F-8, F-12, F-18(e); RR&P candidates F-2 (CI platform),
F-3 (perf gating + tool pick), F-7 (fuzzing on this host), F-11
(feature-matrix policy); QR-3 landing items F-6, F-9, F-13; taxonomy
candidates for Stage C: privacy testing, ops-validation/DR.
