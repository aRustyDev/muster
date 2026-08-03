# QR-2 — Additions and order (Stages C+D)

*Executed 2026-08-03 against `00-review-plan.md` (pre-committed 2026-08-02)
and `01-gap-matrix.md` (QR-1, closed 2026-08-03). Method as amended in the
plan's Execution architecture: seed maintenance verification (Rule 06 bar 2)
ran as six parallel web-verification subagents returning verdict rows —
checked against crates.io release data and repository activity as of
2026-08-03; one repo reader returned the stage-gate ladder for the semver
mapping. ALL synthesis, classification, ordering (rules D1–D5), and the
semver mapping were done inline in the main loop. This document is the
reviewable synthesis; nothing here edits the plans corpus — QR-3 (Stage E)
lands the amendments this document specifies.*

## Refutations first (Rule 01.3 — where QR-2's expectations broke)

Numbering continues QR-1's refutation ledger (R-1..R-5 live in
`01-gap-matrix.md`). Verdicts are as of 2026-08-03; sources are crates.io
release data and repository activity, fetched this session.

**R-6 — the host-blocked-fuzzing premise is softer than pre-stated.** The
plan's constraints section said cargo-fuzz "needs nightly for
sanitizer-backed runs; libFuzzer mode may also be blocked" and pointed the
escape hatch at containers. Verified: **afl.rs runs on stable Rust and
explicitly supports ARM64 macOS** (rust-fuzz book; 0.18.2, 2026-05);
cargo-fuzz supports Aarch64 macOS too (nightly required for libFuzzer).
Local fuzzing on this host is viable without rustup. RR&P-3 is therefore
not forced through the CI Linux leg — the plan's "CI question wearing a
costume" prediction was only half right.

**R-7 — the host-blocked-coverage premise is refuted.** The constraints
pre-stated llvm-tools-preview-based coverage as "directly blocked" by the
no-rustup host. Verified: **cargo-llvm-cov documents a no-rustup path**
(`LLVM_COV`/`LLVM_PROFDATA` env vars pointed at a matching system LLVM;
README covers "toolchain installed without rustup"; doctest coverage still
needs nightly), and **cargo-tarpaulin's default engine on macOS (incl.
arm64) is llvm**, not ptrace. I1 stays an RR&P for the *policy* question,
but viability on this host is confirmed, not open.

**R-8 — quickcheck is not dormant.** The plan seeded it as a comparison
point with dormancy implied; it released 1.1.0 on 2026-02-10 with repo
activity into 2026-04 and no passively-maintained notice in the current
README. It passes the bar. Its rejection below rests entirely on
single-framework discipline (proptest is the ADR-0022 baseline), not on
maintenance — the honest reason, recorded per the ADR-0020 discipline.

**R-9 — the maintenance bar's release clause and real activity diverge on
three tools this review expected to clear it.** divan (last release
2025-04, repo active 2026-07), samply (last release 2025-02, repo very
active, ships arm64 binaries), pprof (release misses the window by ~11
days). The bar as written measures releases, not life. Dispositions below
apply the bar's letter with the caveat visible rather than silently
bending it; whether the bar itself should read "releases *or sustained
commit activity*" is queued as a Rule 06 wording question for the owner
(it changes divan's and samply's verdicts, nothing else here).

**R-10 — two expected defaults failed outright.** dhat-rs — the taxonomy's
named P12 tool — last released 2024-02 and its README self-declares
"maintenance is not a high priority": as an orrery dev-dependency it fails
Rule 06, so the memory-growth harness (O-3) goes dependency-free instead
(a ~50-line counting `GlobalAlloc` wrapper — Rule 06 bar 1 says
self-implement). cargo-public-api requires a rustup-installed nightly to
build rustdoc JSON — dead on this host — leaving cargo-semver-checks
(stable, released 2026-08-01) as RR&P-6's presumptive sole survivor.

**Smaller corrections from triage** (seed-list hygiene, Rule 01.6 class):
`satya` is not a Rust crate at all — it is a PyPI package with a Rust
core; `cargo-featomatic` has never existed on crates.io (a 2-star 2017
GitHub repo only); the Bencher CLI is not on crates.io either — the
crates.io crate named `bencher` is an unrelated old harness (install is
script or `cargo install --git`); and criterion's maintainer-churn worry
is resolved (repo moved to the criterion-rs org with named maintainers,
0.8.2 released 2026-02).

## Stage C — synthesis

### C.0 Taxonomy decisions (the two review-added candidates)

**S5 Privacy testing — ADOPTED as a first-class dimension** (Safety &
robustness group). Definition: *executable assertions that privacy
invariants hold on every egress channel — wire payloads, logs, spans,
errors, exports, session state.* Rationale: the corpus's hardest gate class
(Rule 00.6 "asserted by an automated test", Rule 09's channel list) had no
matrix row, and QR-1's F-4 found six written-but-unasserted privacy
promises scattered across C19/C16/S cells — exactly the failure mode a
dedicated row prevents. Initial dispositions (provisional, to be maintained
with the matrix at QR-3): orrery ◐ (privacy_ family real at the engine
boundary; log/error/span channels unasserted — F-4 a,e) · muster-sdk ◐ (no
anchor data by design, inherits the engine guarantee; one plan sentence
owed, no test) · muster ◐ (session state and egress unasserted — F-4 b,d) ·
muster-types ◐ (key allowlist real; cross-member contract unasserted —
F-4 c) · muster-server ◐ (privacy_wire real; span/log channels unasserted)
· muster-ui ✗ at Alpha (renders member payloads; no plan) · workspace ◐
(Rule 09 policy real; no aggregate gate).

**I5 Ops-validation / DR — ADOPTED** (Infrastructure & meta group).
Definition: *executable validation of operational procedures —
backup/restore, deterministic rebuild, config sanity, migration drills.*
Rationale: the muster RC row gates "backup/restore" (specs/03:48,
ROADMAP:10) and RC gates "deterministic rebuild verified" (F-10/MO-9) with
no dimension to live in. Initial dispositions: all rows N/A or gap until
the RC pre-commitment; the row exists so that pre-commitment has a home
and the matrix stops losing these gates.

### C.1 Per-crate 'to add' lists

Classification vocabulary is the plan's: **implement-now** (no open
questions; owner named) · **RR&P-N** (defined in C.2) · **reject** (reason
written). Every item names its landing home for QR-3 (Rule 07: one home
per fact) and the finding/cell it consumes.

#### workspace / CI

| ID | Item | Class | Source | Home (QR-3) | Owner |
|---|---|---|---|---|---|
| W-1 | Fix the broken doors: `test-prop` drops `--features proptest` and unions the filter to `test(/^(prop_\|optimality_\|monotone_)/)`; `orrery::test-detectors`/`test-incremental` drop the feature flag and fix filters to the real test names (`prop_matches_oracle`/`oracle_*`); `matrix` repo-sqlite leg and `differential` get explicit "arrives with Phase 7 (ADR-0021)" guard messages; `bench`/`bench-canonical` fail loudly ("no bench targets yet — RR&P-2") instead of silently passing. Naming policy: new property tests take the `prop_` prefix workspace-wide | implement-now | F-1 | justfiles (code); naming policy → TESTING-STRATEGY | QF slice |
| W-2 | Written measurement-variance policy: every measure_ harness runs ≥3 measured iterations after ≥1 warm-up, reports median+max (or cross-run p50/p95), pins the release profile, and states a numeric reproduction tolerance; applied to measure_alpha_budgets.rs and measure_select.rs; ladybug install pinned in evidence/run_all.sh; measure_select header contradiction corrected (dated) | implement-now | F-5, P9 row | TESTING-STRATEGY (policy); harness files (code) | QF slice |
| W-3 | Doctest door: root `test-doc: cargo test --doc --workspace` recipe, added to `ci`; policy sentence: rustdoc examples are tests and nextest does not run them | implement-now | F-12, C15 row | justfile + TESTING-STRATEGY | QF slice |
| W-4 | `[workspace.lints]` carrying the clippy severity now living in justfile flags; `[profile.*]` sections encoding the pre-pinned release/bench profile the measure_ headers demand | implement-now | F-17 | workspace Cargo.toml | QF slice |
| W-5 | cargo-deny adoption (verdict: PASS — 0.20.2, 2026-07, 10+ contributors): deny.toml (advisories, licenses, bans, sources) + `just deny` local door now; CI wiring behind RR&P-1. Satisfies Rule 09's cargo-audit promise via `cargo deny check advisories` (Rule 09 wording amended, dated, at QR-3; cargo-audit itself PASSES and stays the named fallback). License allow-list is a one-line owner decision at implementation | implement-now | S3/S4 rows | deny.toml; quality-tooling ADR; Rule 09 amendment | QF slice |
| W-6 | cargo-hack feature legs (verdict: PASS — 0.6.45, 2026-05, taiki-e): `cargo hack check --each-feature --workspace` (including a no-features leg) replaces the broken `matrix` legs honestly and gives muster-ui its documented-but-never-run bare-library configuration; full repo-* matrix activation stays Phase-7-owned | implement-now | F-11, C17 rows | justfile; quality-tooling ADR | QF slice |
| W-7 | Profiling door: TESTING-STRATEGY names cargo-flamegraph (verdict: PASS — 0.6.13, 2026-06; macOS backend is xctrace/Instruments, sudo optional) as the documented recipe, with samply as the interactive alternate (bar caveat per R-9: no release since 2025-02 but very active, arm64 binaries shipped — acceptable for an operator tool that enters no Cargo.toml) and Instruments direct as the third door; profiling remains on-demand (D2: measure before optimize), no dependency enters any crate | implement-now | P11 rows | TESTING-STRATEGY | QF slice |
| W-13 | `#![forbid(unsafe_code)]` in every library crate (all are unsafe-free today, verified in QR-1) — turns the unwritten safe-Rust practice into a compiler-enforced policy; S2 cells move from partial-by-practice to enforced-by-lint; miri/sanitizers stay a CI-era item inside RR&P-1's Linux leg | implement-now | S2 rows | crate lib.rs files; TESTING-STRATEGY sentence | QF slice |
| W-14 | Regression policy sentence: every refuted hypothesis or fixed defect lands a pinned test named for the finding (the H1-travel practice, written down) | implement-now | C13 rows | TESTING-STRATEGY | QR-3 |
| W-15 | Guard honesty (F-8): check-scope denylist broadened beyond four names + its intent documented; check-seam records which arm (nm vs grep fallback) ran in gate output; grep-fallback caveat feeds RR&P-6 | implement-now | F-8 | crate justfiles | QF slice |
| W-12 | Proptest case-budget rationale: 48 cases as the wall-clock-affordable default, raised via `PROPTEST_CASES` for deep runs; the adversarial 1-eval budget bound documented as deliberate test design | implement-now | F-16, F-6 disposition | TESTING-STRATEGY | QR-3 |
| W-8 | CI bring-up | **RR&P-1** | F-2, I2 row | — | — |
| W-9 | Perf measurement & gating | **RR&P-2** | F-3, P1/P3/P9 | — | — |
| W-10 | Coverage tooling | **RR&P-4** | I1 row | — | — |
| W-11 | Mutation testing | **RR&P-5** | C10 row | — | — |

#### orrery

| ID | Item | Class | Source | Home (QR-3) | Owner |
|---|---|---|---|---|---|
| O-1 | Log/error-channel privacy assertions: extend the privacy_ family with a capturing test subscriber asserting no coordinate reaches any log line or span attribute, and a type-level test that no error variant carries coordinate fields (Rule 09's "payload, log, or error" — currently only payload is asserted) | implement-now | F-4 a | orrery SPEC-05 amendment; tests | Muster-Alpha entry |
| O-2 | Span-capture privacy check (the "queued for RC, designed nowhere" item): design lands with O-1's capturing subscriber; the RC-scoped end-to-end sweep (F-4 f) becomes an owned RC ledger row | implement-now (design) + owned RC row | F-4 e,f | SPEC-05; CARRY-FORWARD row | Alpha (design) / RC (e2e) |
| O-3 | Memory-growth harness: salsa-mirror + violation-store growth measured over long command streams (the P7/P12 soak shape), run explicitly like the measure_ family, variance policy W-2 applies. Tool: a ~50-line counting `GlobalAlloc` wrapper in the harness — dependency-free per Rule 06 bar 1, because dhat-rs failed the bar (R-10: last release 2024-02, self-declared low-priority maintenance) and hotpath is young and effectively single-maintainer (watch-listed) | implement-now | P7/P12 rows | SPEC-05 amendment | Orrery-Beta prep |
| O-4 | Rustdoc examples for the public API at Beta freeze (feeds the RC "docs complete" criterion with something executable — W-3 makes them run) | implement-now | C15 row, F-10 | SPEC-05 line | Orrery Beta |
| O-5 | "Incremental fuzz green" interim definition: the Beta gate means the prop_incremental family at its documented case budget until RR&P-3 redefines it — honest narrowing, dated (Rule 01.2) | implement-now | F-7 | ROADMAP gate line amendment | QR-3 |
| O-6 | Coverage-guided fuzz targets (interval algebra, command apply, preview honesty) | **RR&P-3** | S1 row, F-7 | — | — |
| O-7 | Differential-testing activation, repo-* matrix full legs, 10⁶ re-measure, SQLite-baseline re-measure on the decision host, result-materialisation measurement, UUIDv7 locality bench candidate (F-19) | owned Phase-7 rows (no new work now) | C11/C17, P1 critic notes, F-19 | CARRY-FORWARD visibility rows | Phase 7 (D4) |

#### muster-sdk

| ID | Item | Class | Source | Home (QR-3) | Owner |
|---|---|---|---|---|---|
| SDK-1 | Fix the unimplementable spec line: SPEC-03's "byte-for-byte on serialised output" amended (dated) to what the test does (in-memory equality on re-run); a true serialized-determinism test becomes an Alpha row where suggestions first cross a wire (muster-server) | implement-now (spec correction) | C16 row | sdk SPEC-03 dated amendment; Alpha ledger row | QR-3 / Alpha |
| SDK-2 | SDK perf-gate definition: the RC "perf gates green" reference gets a referent — suggest/search wall-time budgets at Beta-scale instances, defined at RR&P-2 close, owned by a named phase (currently "not planned") | fold into **RR&P-2** | F-3 | sdk SPEC-03 + ROADMAP amendment | RR&P-2 → Muster Beta |
| SDK-3 | One-way boundary automation: the manual "source grep at phase close" becomes a `check-oneway` recipe failing the build, sibling to check-scope | implement-now | F-8 | sdk justfile | QF slice |
| SDK-7 | Severity-weight single-sourcing: the engine/objective duplication gets the shared constant already flagged in-plan, or the phase-doc claim "asserted in integration" is corrected to match the `vc.cost >= 100.0` reality (dated) | implement-now | F-8 | sdk code or phase-doc correction | QF slice |
| SDK-4 | sdk span table: `sdk.suggest`/`sdk.search`/`sdk.batch` attributes spec'd (Rule 05's table is orrery-only; these landed ad-hoc) | implement-now | F-9, I3 row | Rule 05 / sdk SPEC amendment | QR-3 (text) / Alpha (align) |
| SDK-5 | Churn/stress scale definition ("class, scale, removal rule, seeds — none defined today") pre-committed at Muster-Beta entry | owned row | P5 row, R-4 | CARRY-FORWARD row | Muster-Beta pre-commitment |
| SDK-6 | Smoke tier for the sdk | **reject** — the whole workspace suite is fast today; a smoke cadence adds tiering complexity with no wait-time pain. Revisit if the suite exceeds ~5 minutes | C4 row | recorded here | — |

#### muster

| ID | Item | Class | Source | Home (QR-3) | Owner |
|---|---|---|---|---|---|
| M-1 | Transition-day recurrence-expansion tests — an accepted ADR *mandates* them ("Muster's Phase-6 specs must include…", ADR-0024:57-64) and no spec carries them; the engine cannot detect mis-expansion | implement-now | F-18 a | muster SPEC-03 amendment | Muster-Alpha entry |
| M-2 | Lapsed-coordinator temporal-authz property ("loses power the moment the membership window ends") | implement-now | F-18 b | muster SPEC-03 | Muster-Alpha entry |
| M-3 | Per-violation-class resolvability matrix: "resolves every violation class" gets a coverage table — one e2e per class, tracked | implement-now | F-18 c | muster SPEC-03 | Muster-Alpha entry |
| M-4 | Preview utility criterion: pre-committed fixture scenario where preview flags an unintended mass change ("prevents at least one unintended mass change in testing", prds/00:78) | implement-now | F-18 f | muster SPEC-03 | Muster-Alpha entry |
| M-5 | Session-state privacy check: executable assertion that app-owned session state contains no anchors/coordinates | implement-now | F-4 d | muster SPEC-03 | Muster-Alpha entry |
| M-6 | Muster-Alpha pre-commitment carries all six F-4 channels explicitly (a–f), each either tested at Alpha or honestly deferred with a dated owner (Rule 01.2) | implement-now (pre-commitment content) | F-4 | Alpha pre-commitment doc | Alpha entry |
| M-7 | Dead `run-dev` telemetry knob deleted or rewired when subscriber ownership is corrected (muster has no tracing/figment dep; muster-server's twin is live) | implement-now | F-9 | crate justfile; Rule 05 amendment | QF slice |
| M-8 | RC egress gate ("Parquet/CSV with anchors excluded") gets a pre-committed test statement at Beta/RC pre-commitment; excluded-by-default posture restated in TESTING-STRATEGY | owned RC row | F-4 b | CARRY-FORWARD row; TESTING-STRATEGY | RC pre-commitment |
| M-9 | 100 ms budget definition reconciled (SPEC-00 scopes it "of the engine returning"; H4 measured whole-select; stricter-than-written today) before the HTTP edge is measured | implement-now (dated spec line) | F-14 | muster SPEC-00 amendment | QR-3 |

#### muster-types

| ID | Item | Class | Source | Home (QR-3) | Owner |
|---|---|---|---|---|---|
| T-1 | Serde roundtrip property tests for every wire DTO (proptest strategies; unknown-field posture decided with the coordinator DTOs) — the crate's first own tests, arriving with its first input surface | implement-now | C5/C12 rows | muster SPEC-03 (types section) | Muster-Alpha entry |
| T-2 | Cross-member privacy test: member A's payload contains no identifier of member B (the contract the key allowlist cannot see) | implement-now | F-4 c | muster SPEC-03 | Muster-Alpha entry |
| T-3 | AGENTS.md (Rule 07 skeleton); justfile arrives with T-1 when the crate first has runnable tests | implement-now | F-11 | new crate AGENTS.md | QR-3 |
| T-4 | Placement statement: DTO contract tests live in muster-server by design (ADR-0025 single enforcement point) — written down so the absence of tests here reads as a decision, not an accident | implement-now | C14 row | TESTING-STRATEGY | QR-3 |

#### muster-server

| ID | Item | Class | Source | Home (QR-3) | Owner |
|---|---|---|---|---|---|
| SRV-1 | error→status contract tests for the four untested variants (1 of 5 covered today) | implement-now | F-18 d | tests (no spec change needed — SPEC-03 already implies it) | QF slice |
| SRV-2 | Wire-name pinning test: serialized violation kind/severity names asserted so an engine enum rename breaks a test, not the contract (plain asserts now; snapshot tooling is RR&P-8's call) | implement-now | F-18 e | tests | QF slice |
| SRV-6 | Subscriber-ownership correction: Rule 05 and muster SPEC-02 name muster-server as the configurator (dated amendment; the landed architecture is right, the text is wrong) | implement-now | F-9 | Rule 05 + SPEC-02 amendments | QR-3 |
| SRV-7 | HTTP-edge latency measurement (tower-oneshot timing harness, budgets from M-9's reconciled definition) | fold into **RR&P-2** (macro leg) | P9 row | — | Alpha exit |
| SRV-3 | Wire-payload deser fuzzing | fold into **RR&P-3** (target list) | S1 row | — | — |
| SRV-4 | Load/stress/spike/soak definition for the coordinator flow | **RR&P-9** | P4–P7 rows | — | — |
| SRV-5 | Network profiling | **reject** — honestly lowest-priority (the plan pre-stated it); no criterion depends on it; revisit only if HTTP-edge budgets (SRV-7) miss and the cause isn't in the service layer | P13 row | recorded here | — |

#### muster-ui

| ID | Item | Class | Source | Home (QR-3) | Owner |
|---|---|---|---|---|---|
| UI-1 | UI testing approach (render/snapshot mechanism, REST-client double, wasm-perf disposition, a11y measurability) | **RR&P-8** | C1–C16 ui rows, F-10 | — | — |
| UI-2 | Bare-library gate leg — delivered by W-6 (cargo-hack no-features leg); recorded here as the beneficiary; ADR-0025's "checked by workspace CI" claim corrected at QR-3 (F-2/F-13 kin) | see W-6 | F-11, C17 row | ADR-0025 dated correction | QF slice |
| UI-3 | ADR-0003 window-trap line in the frontend guidelines when they exist (the critic's homeless item) | implement-now | QR-1 critic note | Alpha UI slice guideline doc | Alpha |
| UI-4 | UI instrumentation disposition: none pre-Beta, revisit with a real diagnostic need — dated one-liner so I3's silence becomes a decision | implement-now | I3 ui row | Rule 05 amendment line | QR-3 |
| UI-5 | AGENTS.md (Rule 07 skeleton) | implement-now | F-11 | new crate AGENTS.md | QR-3 |

### C.2 RR&P stage definitions

Every stage has the plan-required shape: question · candidates · deciding
criteria · constraints · the phase entry it gates. Candidates marked ⊕ are
review-added (the Appendix is a floor, not a ceiling).

**RR&P-1 — CI bring-up (I2; consumes F-2).**
*Question:* which CI platform, which runner strategy (hosted macOS vs
Linux legs vs self-hosted OrbStack), and what minimal gate set stands up
first? *Candidates:* GitHub Actions (the inference to confirm — owner
touchpoint #2); self-hosted runner on OrbStack Linux; forge-native CI if
the repo host turns out not to be GitHub. *Criteria:* repo-host reality;
release-please compatibility; a Linux leg (unblocks miri/sanitizers-class
S2 work, coverage without host constraints, fuzzing); secrets posture
(Rule 09); maintenance cost of self-hosting. *Constraints:* every gate run
today has single-host provenance; an accepted ADR asserts CI exists
(ADR-0025:97 — false premise, dated correction at QR-3); phase docs keep
recording single-host provenance honestly until this closes. *Gates:*
S3/W-5 CI wiring, P3 (RR&P-2's CI leg), I1 aggregation (RR&P-4's CI leg),
release-please activation, RR&P-5 cadence. *Closes:* before Orrery-Beta
entry at the latest; recommended concurrent with Muster Alpha.

**RR&P-2 — Perf measurement & gating (P1/P2/P3/P9; consumes F-3).**
*Question:* which micro-bench harness (criterion vs divan), what the
macro tier is (hyperfine for binaries; the measure_ family's evolution),
where regression baselines live, and what the gate mechanism is locally
vs in CI. *Candidates:* criterion (PASS — 0.8.2 2026-02, maintainer handoff
resolved to the criterion-rs org; the presumptive pick) · divan (bar
FAIL by the release clause, repo active — enters only if the owner
relaxes the bar per R-9 or a release lands before this closes) ·
hotpath (PASS but young, single-maintainer-dominated — watch-list, not
a candidate) (micro); hyperfine (PASS) · rustc-perf-as-pattern (macro);
Bencher (PASS; self-hostable; CLI installs via script/git, *not*
crates.io — the crates.io `bencher` crate is unrelated; CI tier, dormant
until RR&P-1); the incumbent in-test 10× sanity bounds (the honest
coarse mechanism). *Criteria:* stable-Rust operation; baseline
persistence and compare ergonomics; wall-clock cost per run; macOS arm64
support; nextest coexistence. *Constraints:* the local
baseline part is unblocked now, the CI part sits behind RR&P-1 (D1);
variance policy W-2 applies to whatever harness wins; **funnel discipline
D4 — the seven-canonical-queries bench must not become a datastore-
candidate benchmark before the ADR-0021 stage permits one.* *Gates:*
SPEC-05's Benchmark level (seven queries × three scales) finally gets a
mechanism; SDK-2's perf-gate definition; SRV-7's HTTP-edge measurement;
the Phase-7 preconditions (SQLite re-measure on the decision host;
materialisation above 2,172 rows) become executable rather than quoted.
*Closes:* pick + orrery bench skeleton during/just after Muster Alpha;
regression gates when CI exists.

**RR&P-3 — Fuzzing viability on this host (S1; consumes F-7).**
*Question:* is coverage-guided fuzzing viable on macOS arm64 with
Homebrew rust (no rustup), with which harness, on which targets, and what
does the Orrery-Beta gate "incremental fuzz green" mean once true fuzzing
exists? *Candidates (all three PASS the bar; host facts per R-6):*
afl.rs (stable Rust, ARM64 macOS explicitly supported — the local
front-runner) · cargo-fuzz (Aarch64 macOS works but libFuzzer needs
nightly) · honggfuzz-rs (macOS listed, Linux-first tooling,
near-single-maintainer) · the RR&P-1 Linux leg as a *complementary*
venue for sanitizer-backed runs (no longer the forced answer — R-6
softened the plan's "CI question in costume" prediction). *Criteria:* toolchain requirement vs this host;
target value ranking (interval algebra, command apply, preview honesty,
wire deser SRV-3); corpus management; schedulability in CI. *Constraints:*
no rustup; Rule 09 — fuzz corpora never contain anchor fixtures, and
wire-boundary fuzzing extends the privacy_ family, never bypasses it;
O-5's interim gate definition holds until this closes. *Gates:* the
Orrery-Beta "incremental fuzz green" gate's real definition. *Closes:*
before Orrery-Beta entry; expected dependent on RR&P-1's Linux leg (D1).

**RR&P-4 — Coverage tooling (I1).**
*Question:* which tool measures coverage on this host and in CI, and what
the reporting policy is (informational-first vs threshold vs ratchet)?
*Candidates (both PASS the bar and both are host-viable per R-7):*
cargo-llvm-cov ⊕ (documented no-rustup path via `LLVM_COV`/
`LLVM_PROFDATA` against a version-matched system LLVM; nextest support;
doctest coverage still needs nightly — the one residual host gap) ·
cargo-tarpaulin (0.37.0, 2026-07; llvm engine is the macOS default,
arm64 supported, build-from-source install) · CI-Linux-only coverage as
the fallback venue only if the LLVM-version-matching proves brittle.
*Criteria:* the LLVM-match reliability on Homebrew rust; nextest
integration; doctest inclusion (W-3 interplay); report formats CI can
aggregate. *Constraints:* aggregation shape depends on RR&P-1's platform
answer; start informational — a threshold with no baseline is a gate
that lies.
*Gates:* RR&P-5 (D2: coverage before mutation — the plan's own rule).
*Closes:* local leg any time after QF; CI leg after RR&P-1.

**RR&P-5 — Mutation rollout (C10).**
*Question:* adopt cargo-mutants with what scope (engine detectors +
interval algebra first), wall-clock budget, and cadence (scheduled, never
per-push)? *Candidates:* cargo-mutants (expected sole survivor — triage
verdicts for mutagen/bough recorded in the table) · mutagen · bough.
*Criteria:* run time against the suite; nextest compat; file-scoped
incremental runs; signal quality on property-heavy suites (a mutant
surviving because only a property covers it is information about the
property, not noise — the rollout notes must say how survivors are
triaged). *Constraints:* D2 — after RR&P-4 produces a baseline; cadence
needs RR&P-1. *Gates:* none stage-critical; recommended green-ish on
orrery before Beta freeze (mutation is the check on the freeze's test
quality). *Closes:* after RR&P-4.

**RR&P-6 — API-freeze diff tooling (C14).**
*Question:* what mechanically asserts "no unintended API break" at and
after Orrery Beta's freeze — the corpus itself says "grep fallback is not
an API-diff"? *Candidates:* cargo-semver-checks ⊕ (PASS — 0.50.0
released 2026-08-01, runs on stable; the presumptive survivor) ·
cargo-public-api ⊕ (PASS on maintenance but requires a rustup-installed
nightly to build rustdoc JSON — dead on this host per R-10; viable only
in a CI Linux leg) (both review-added; neither was in the Appendix);
the incumbent check-seam stays regardless — it guards datastore-type
leakage, a different job. *Criteria:* CI integration; false-positive
rate under pre-1.0 semantics; whether a diff-listing tool
(cargo-public-api's strength) is wanted alongside the semver linter. *Constraints:* 0.x semver —
breaking is allowed pre-1.0, so the gate's job is *awareness and
intentionality*, wired to release-please majors only at 1.0. *Gates:*
Orrery-Beta API freeze (the Beta pre-commitment carries the chosen
tool's gate line). *Closes:* before Orrery-Beta entry.

**RR&P-7 — Wire-input validation (C19 at the input boundary).**
*Question:* when coordinator commands start arriving over HTTP at Muster
Alpha, is a validation library warranted at all — against the incumbent
pattern of typed constructors plus engine-side construction-time
rejection — and if so, which? *Candidates:* hand-rolled `TryFrom` DTO→
domain conversions (the null hypothesis — Rule 06 bar 1 asks whether a
library beats ~200 lines); validator (PASS — 0.21.0 2026-07, 83
contributors, the de-facto standard) · garde (PASS — 0.23.0, 22
contributors, the modern alternative) · axum-valid (PASS — bridges
validator/garde/validify on axum 0.8, though it pins validator one
minor behind); the Appendix tail (Rustdantic, valida, verify,
validatron, accord, satya, valust, scrutiny) all failed triage — see
the table — leaving a clean three-way-plus-null decision. *Criteria:* axum integration; error-shape control (must compose
with the SRV-1 error→status contract); derive ergonomics on a small DTO
set; maintenance verdicts. *Constraints:* muster-server/muster-types
only — never orrery (Rules 03/06); validation errors must not echo
payload contents that could carry anchor data (Rule 09 error channel —
S5 applies to rejects too). *Gates:* Muster-Alpha server-input slice
(the coordinator-DTO extension CF row). *Closes:* at Alpha
pre-commitment (D5: one slice ahead of its implement item).

**RR&P-8 — UI testing approach (muster-ui; C1–C16 ui column).**
*Question:* how Alpha's UI content gets tested — render-to-string
snapshots, DOM-level assertions, or an honest none-yet — what doubles the
REST client, whether insta enters the workspace, and what the wasm-perf
and a11y dispositions are. *Candidates:* dioxus ssr render-to-string +
insta ⊕ snapshots · plain string asserts · wasm-bindgen-test ⊕
(browser-driven) · defer-with-reason. *Criteria:* compatibility with the
dioxus 0.7.x pin (ADR-0025 — no 0.8-alpha tracking); rendered-output
determinism; cost against Alpha's actual UI scope; the a11y "level TBD"
gate needs a measurable proposal (feeds owner touchpoint #3).
*Constraints:* D3 — the UI content this tests does not exist yet; the
dioxus pin bounds tool choices. *Gates:* Muster-Alpha UI slice entry.
*Closes:* at Alpha pre-commitment.

**RR&P-9 — HTTP load/stress harness (P4–P7 muster-server; SRV-4).**
*Question:* which load-generation tool, and what workload shapes and
thresholds define load, stress, spike, and soak for the coordinator flow?
*Candidates (all ⊕ — the Appendix seeded none):* oha · goose · drill ·
k6; soak's memory leg reuses O-3's approach at the server. Candidate
verification happens when this stage opens (deliberately unverified now —
it is two stages out and tools churn). *Criteria:* multi-step workload
scripting vs single-endpoint hammering; maintenance; local-first
operation; result stability under W-2's variance policy. *Constraints:*
D3 — needs the deployed coordinator flow (post-Alpha); thresholds derive
from M-9's reconciled budget definition; the Mutex-serialized service is
a *deliberate* single-writer ceiling — stress results must be read
against that design, not as a defect discovery. *Gates:* Muster-Beta
entry (its pre-commitment defines the four P-legs with thresholds).
*Closes:* at Muster-Beta pre-commitment.

### C.3 Seed triage table

Every Appendix seed dispositioned exactly once: **adopt-now** ·
**fold→RR&P-N** · **reject** (reason written). Verdicts are the Rule 06
bar-2 check ("releases within the last year, more than one contributor")
as of 2026-08-03, from crates.io release data + repository activity;
⊕ marks review-added candidates (the Appendix is a floor). Non-crate
seeds (essays, guides, surveys) carry no verdict — the bar doesn't apply
to reading material; they fold as references.

**Arithmetic (Rule 01.6): 50 crate-shaped tools verified = 25 PASS +
23 FAIL + 2 UNCLEAR.** The plan estimated "~25–30 crate-shaped seeds";
the count grew to 50 because the taxonomy's Notes column names tools the
Appendix didn't repeat (mockall, samply, dhat-rs, insta, cargo-hack,
cargo-deny, tarpaulin) and this review added candidates (⊕) where the
Appendix seeded none.

| Seed | Verdict (bar 2) | Disposition |
|---|---|---|
| **Fuzzing** | | |
| afl.rs | PASS (0.18.2, 2026-05; stable Rust; ARM64 macOS) | fold→RR&P-3 (local front-runner, R-6) |
| cargo-fuzz | PASS (0.13.2, 2026-06; nightly for libFuzzer; Aarch64 ok) | fold→RR&P-3 |
| honggfuzz | PASS (0.5.61, 2026-06; macOS listed; near-single-maintainer) | fold→RR&P-3 (third) |
| **Micro-bench** | | |
| criterion | PASS (0.8.2, 2026-02; handoff resolved to criterion-rs org) | fold→RR&P-2 (presumptive pick) |
| divan | FAIL release clause (0.1.21, 2025-04; repo active 2026-07) | fold→RR&P-2 with R-9 caveat — in only if the bar is relaxed or a release lands |
| criterion/bencher.dev/generalist guides | n/a (reading) | fold→RR&P-2 references |
| **Macro-bench** | | |
| hyperfine | PASS (1.20.0, 2025-11; sharkdp) | fold→RR&P-2 (macro tier; lean-adopt when a binary-level timing question exists) |
| rustc-perf | n/a (harness pattern, not a dependency) | fold→RR&P-2 reference |
| **CI bench** | | |
| Bencher (bencher.dev) | PASS (v0.6.11, 2026-07; self-hostable; CLI not on crates.io — `bencher` crate is unrelated) | fold→RR&P-2 CI tier (dormant until RR&P-1) |
| **Validation** | | |
| Rustdantic/rusdantic | FAIL (0.1.0 sole release 2026-03; solo; 219 downloads; pure-Rust derive, not PyO3) | reject — fails bar; no adoption |
| serde+validator | PASS (0.21.0, 2026-07; 83 contributors; 58.2M downloads) | fold→RR&P-7 |
| serde+garde | PASS (0.23.0, 2026-05; 22 contributors) | fold→RR&P-7 |
| valida | FAIL (1.1.2, 2025-07-24 — 10 days outside window; solo) | reject |
| verify | FAIL (0.3.2, 2020; dormant ~5 yrs) | reject |
| validatron | FAIL (0.5.0, 2022; dormant ~3 yrs) | reject |
| accord | FAIL (0.2.2, 2017; dormant ~8 yrs) | reject |
| satya | FAIL (not a Rust crate — PyPI package with a Rust core) | reject — seed-list hygiene, see Refutations |
| valust-rs | FAIL (valust 0.8.0, 2025-01; solo) | reject |
| scrutiny | FAIL (0.1.2, 2026-04; solo, 127 downloads, 2-day release burst) | reject — a validation crate, but zero adoption |
| axum-valid | PASS (0.25.0, 2026-06; 7 contributors; bridges validator/garde/validify) | fold→RR&P-7 (bridge) |
| validation surveys (agileengine, masteringbackend, codezup, leapcell, rustfaq, ruststepbystep, dev.to) | n/a (reading) | fold→RR&P-7 references |
| **Property** | | |
| proptest(+derive) | already in baseline (ADR-0022; sole dev-dep in orrery/sdk) | adopt-now (no-op — already adopted) |
| quickcheck(+macros) | PASS (1.1.0, 2026-02 — alive, R-8) | reject — single-framework discipline: proptest is the baseline; a second property framework buys idiom drift, no capability |
| **Mocking** | | |
| mockall *(taxonomy-named)* | PASS (0.15.0, 2026-06) | reject — strategy, not maintenance: the restrictive MemoryRepo *fake* is the established double (ADR-0021, Rule 00b); mocks would let the trait absorb assumptions the fake exists to block. UI REST double → RR&P-8 |
| mock_shootout comparison | n/a (reading) | fold — reference recorded with the C8 rejection |
| conditional-compilation pattern (klau.si) | n/a (reading) | fold→TESTING-STRATEGY test-double section reference |
| time-mocking gotchas (blog.iany.me) | n/a (reading) | fold — the gotcha is designed out here ("the engine reads no clock; the binary edge owns time"); reference kept with that note |
| oliverwyman essay · generic-parameter mocking criterion | n/a (reading) | fold — C8 rationale references |
| **Parameterized / snapshot / generation / MBT** | | |
| test-case | FAIL (3.3.1, 2023-11; last push 2024-05) | reject — also no table-driven pain today; hand-written case matrices stay readable |
| rstest ⊕ | UNCLEAR (0.26.1, 2025-07-27 — 7 days outside window; commits into 2026-03) | recorded as the future parameterized candidate if a real need appears; no adoption now |
| specker | FAIL (0.3.5, 2018; solo) | reject — dormant as the plan predicted |
| insta ⊕ | PASS (1.48.0, 2026-06; mitsuhiko + 5+) | fold→RR&P-8 (snapshot decision made once, at the UI/wire surface; SRV-2 may adopt it later) |
| test-generator | FAIL (0.3.1, 2022; marginal contributors) | reject — no file-driven corpus exists; **datatest-stable ⊕** (0.3.3, 2025-09, nextest-rs) recorded as the first candidate if Phase-7 differential worlds want a file corpus |
| model | FAIL (0.1.2, 2019; tiny history) | reject — MBT is already realized without a framework (prop_incremental vs cold recompute; two-oracle sdk discipline) |
| **Mutation** | | |
| cargo-mutants | PASS (27.1.0, 2026-06; sourcefrog) | fold→RR&P-5 (primary and expected sole survivor — confirmed) |
| mutagen | FAIL (0.1.2, 2018; last push 2023) | reject — dormant as the plan predicted |
| bough | FAIL (no crates.io release — the `bough` crate is an unrelated placeholder; GitHub repo solo, 1 star) | reject |
| **Docs** | | |
| skeptic | FAIL (0.13.7, 2022; effectively dormant) | reject — W-3 (`cargo test --doc`) covers the need without a dependency |
| docmatic | FAIL (0.1.2, 2018; repo archived) | reject |
| doc tests | n/a (built into cargo) | adopt-now (W-3) |
| **Feature combos / licenses / audit** | | |
| cargo-featomatic | FAIL (never on crates.io; 2017 solo repo) | reject — confirmed beyond the plan's "incomplete" expectation |
| cargo-hack ⊕ | PASS (0.6.45, 2026-05; taiki-e) | **adopt-now** (W-6) |
| cargo-lichking | FAIL (0.9.0, 2020) | reject — cargo-deny licenses subsumes |
| cargo-deny ⊕ | PASS (0.20.2, 2026-07; EmbarkStudios) | **adopt-now** (W-5) — subsumes S3+S4, answering the plan's "evaluate" |
| cargo-audit | PASS (0.22.2, 2026-06; rustsec) | fold→W-5 — subsumed by `cargo deny check advisories`; kept as the named fallback and the Rule 09 referent until the wording amendment lands |
| **Patterns** | | |
| entrait | FAIL (0.7.1, 2024-10; effectively solo) | reject — and strategy besides: the hand-written `Repository` trait seam is load-bearing and legible (Rule 00.1); macro-generated DI hides the seam the boundary gates grep for. Essays remain reading |
| OO DI essays | n/a (reading) | fold — C9 references |
| **Profiling / PGO corpus** | | |
| cargo-pgo | PASS (0.3.0, 2026-01; kobzol) | reject-for-now — D2: no binary hot path has ever been profiled; recorded as the presumptive tool for a post-Beta PGO revisit |
| rustc PGO docs · dev-guide · llvm thread | n/a (reading) | fold — filed with the PGO reject note |
| nnethercote perf-book · brendangregg flamegraphs | n/a (reading) | fold→W-7/RR&P-2 references |
| flamegraph-rs (cargo-flamegraph) | PASS (0.6.13, 2026-06; macOS via xctrace) | **adopt-now** (W-7 documented door) |
| inferno | PASS (0.12.8, 2026-07; jonhoo) | no direct adoption — it is the library layer under the flamegraph tooling; noted as transitive |
| tracing-flame | FAIL (0.2.0, 2021-12 — no release in ~4.5 yrs despite active parent repo) | reject — W-7's tools cover the need without a dependency |
| pprof-rs | UNCLEAR (0.14.1, 2025-07 — ~11 days outside window; macOS arm64 build-only in CI) | reject — samply/cargo-flamegraph cover the P11 need with better host fit |
| minicov | PASS (0.3.8, 2025-12) but embedded/no_std scope | reject — off-target: it is a coverage/PGO runtime for no_std, not a host coverage tool |
| hotpath (+ hotpath.rs series) | PASS (0.23.0, 2026-08; young — first release 2025-09, single-maintainer-dominated) | watch-list under RR&P-2; series folds as reference |
| oneuptime series · criterion-flamegraph integrations · dtrace-on-macOS · infinilabs guide · databend/greptime/tikv writeups · patrickfreed | n/a (reading) | fold→W-7/RR&P-2 references |
| wasm time-profiling (rustwasm book) | n/a (reading) | fold→RR&P-8 (wasm-perf disposition input) |
| async flamegraphs (rustyrazorblade · medium · hegdenu) | n/a (reading) | fold — noted low-relevance: the workspace is synchronous by design (async undecided per Rule 04; no async runtime exists) |
| **General** | | |
| awesome-rust-testing · llogiq · devgenius · elitedev | n/a (reading) | fold→TESTING-STRATEGY reading list |
| tarpaulin/coverage threads (knoldus, users.rust-lang) | n/a (reading) | fold→RR&P-4 references |
| boundaryml explore | n/a (reading) | fold→TESTING-STRATEGY reading list |
| **Review-added tools not seeded above** | | |
| cargo-llvm-cov ⊕ | PASS (0.8.7, 2026-05; no-rustup path documented — R-7) | fold→RR&P-4 (front-runner) |
| cargo-tarpaulin ⊕ *(seeded only via threads)* | PASS (0.37.0, 2026-07; macOS arm64 via llvm engine) | fold→RR&P-4 |
| cargo-semver-checks ⊕ | PASS (0.50.0, 2026-08-01; stable Rust) | fold→RR&P-6 (presumptive survivor) |
| cargo-public-api ⊕ | PASS on maintenance; requires rustup nightly (R-10) | fold→RR&P-6 (CI-leg-only candidate) |
| samply *(taxonomy-named)* | FAIL release clause (0.13.1, 2025-02; repo very active; arm64 binaries) | fold→W-7 as interactive alternate, caveat recorded (R-9) |
| dhat-rs *(taxonomy-named)* | FAIL (0.3.3, 2024-02; README self-declares low-priority maintenance) | reject — O-3 goes dependency-free (R-10) |
| oha ⊕ · goose ⊕ · drill ⊕ · k6 ⊕ | deliberately unverified — two stages out; tools churn | fold→RR&P-9 (verification is that stage's first task) |
| datatest-stable ⊕ | spot-verified (0.3.3, 2025-09, nextest-rs) | recorded with the test-generator reject; no adoption |

## Stage D — ordering and semver

### D.1 The ordering rules, restated (from the plan; falsifiable against the result)

1. **D1** Infrastructure before consumers (CI before CI-bench, audit-in-CI,
   coverage aggregation). Reading used here: documentary infrastructure
   counts — a spec amendment precedes the pre-commitment that cites it.
2. **D2** Measurement before optimization (coverage before mutation;
   baselines before regression gates; profiling before PGO).
3. **D3** Surfaces before their tests (load/spike/soak need the deployed
   flow; UI testing needs Alpha UI content).
4. **D4** Funnel discipline: nothing benchmarks datastore candidates ahead
   of the ADR-0021 stage.
5. **D5** RR&P stages precede their implement items by at least one slice.

### D.2 The order

Context (verified this session): workspace at **0.1.0**; Prototype met;
**next implementation work is Muster Alpha (Phase 6 slice 3, coordinator
flow)**; Phase 7 (down-select + ADR-0015 close) after Phase 6; no
`[workspace.lints]`/`[profile.*]` sections exist (F-17 confirmed current).
Every accepted item appears exactly once; multi-leg items are listed once
with their later leg shown as an edge, not a second item.

**Tranche QF — quality-fixes slice (immediate; before Muster-Alpha
implementation starts).** Its own small branch, merged `--no-ff` (Rule
08), so Alpha's gates run on repaired doors. No incoming edges — these
are free. Items: W-1, W-2, W-3, W-4, W-5 (local door; CI leg → Tranche
CI, edge D1), W-6, W-7, W-13, W-15, SDK-3, SDK-7, SRV-1, SRV-2, M-7.
One deliberate tension, called out rather than smoothed: SRV-2 lands
with plain asserts *before* RR&P-8 decides snapshot tooling — a Critical
finding's fix should not wait on a tool choice; if insta is adopted
later, rewriting one test is cheap. Second called-out exception: W-5 and
W-6 adopt tools without an RR&P despite the "library picks default to
RR&P" rule — the Stage-C bar ("tool choice obvious") is met by
one-sided triage verdicts (cargo-deny and cargo-hack PASS while every
alternative failed), and both are CLI gate tools that enter no
Cargo.toml; the exception is recorded so the rule stays falsifiable.

**Tranche L — QR-3 landing (Stage E; parallel with QF, before the Alpha
pre-commitment is written).** Edge: L → Alpha pre-commitment (D1,
documentary reading — the pre-commitment cites these amendments). Items:
W-12, W-14, O-5, SDK-1 (spec correction; its wire-determinism test row →
Tranche A, edge D3), SDK-4 (table text; code alignment → Tranche A, edge
D3), M-9, T-3 (AGENTS.md; its justfile leg rides T-1, edge D3), T-4,
SRV-6, UI-2 (ADR-0025 correction), UI-4, UI-5 — plus QR-3's own
carryovers from QR-1: F-6/F-13 dated corrections, the S5/I5 rows entering
the matrix, TESTING-STRATEGY.md creation, the quality-tooling ADR.

**Tranche A — Muster-Alpha entry and implementation (Phase 6 slice 3).**
Edges: RR&P-7 close → server-input slice (D5); RR&P-8 close → UI slice
(D5) — both close *at the pre-commitment*, one slice ahead of their
implement items, satisfying D5's letter exactly (called out: this is the
minimum spacing D5 permits, not comfortable slack). M-1, M-2, M-3, M-4,
M-5, M-6, T-1, T-2 (all D3: the coordinator surfaces and DTOs these test
arrive in this slice); O-1, O-2-design (D3: rides the Alpha OTLP-wiring
ledger row); SDK-1's wire-determinism test (D3).

**Tranche B — concurrent with Alpha (no Alpha dependency).**
RR&P-1 (D1 feeder for everything CI-conditional; closes before
Orrery-Beta entry at the latest; owner touchpoint #2 is its opening
move). RR&P-2 local leg — harness pick, orrery `benches/` skeleton,
first baselines (D2: baselines must exist before any regression gate;
D4: the canonical-query bench runs MemoryRepo only until Phase 7).

**Tranche CI — consumers of RR&P-1 (all edges D1).**
W-5's CI wiring; RR&P-2's CI-regression leg + the Bencher decision;
RR&P-4's aggregation leg; release-please activation (semver consequence
below); S2's Linux legs (miri/sanitizer scheduled runs — the honest
home for what the host cannot run).

**Tranche M — the measurement chain.**
RR&P-4 local close (can start any time after QF; its aggregation leg
sits in Tranche CI) → RR&P-5 (D2, the plan's own explicit edge:
coverage before mutation; RR&P-5's cadence additionally needs RR&P-1 —
D1). Recommended landing: mutation informational runs on orrery before
the Beta freeze, since mutation is the check on the freeze's test
quality.

**Tranche PB — pre-Orrery-Beta (all gate Beta entry).**
RR&P-3 close (D5: it redefines the Beta gate "incremental fuzz green";
venue evidence per R-6 — local afl.rs viable, CI leg complementary) →
O-6/SRV-3 fuzz targets (D5: targets follow the harness pick). RR&P-6
close (D5: the freeze needs its diff gate defined before it freezes).
O-3 (D2: memory growth must be *measured* before the 10⁶ scale gate is
trusted). O-4 (D3: examples document the API that freezes here). SDK-2
(defined at RR&P-2 close; green owed at Muster Beta — D5).

**Tranche MB — Muster-Beta pre-commitment.**
RR&P-9 close (D5 for the P4–P7 implement items; D3: needs the deployed
coordinator flow Alpha ships). SDK-5 (D3: churn-gate instance defined
when the churn surface exists).

**Tranche P7 — Phase-7-locked (all D4).**
O-7 bundle as one ordered item: down-select, differential-testing
activation, repo-* matrix full legs, 10⁶ re-measure, SQLite-baseline
re-measure on the decision host, result-materialisation measurement,
F-19's UUIDv7 locality bench candidate, C18's N/A revisit if Cozo
advances. Nothing in any earlier tranche touches datastore candidates —
checked: RR&P-2's bench skeleton is MemoryRepo-only by construction.

**Tranche RC — RC-scoped (owned rows, defined at their pre-commitments).**
M-8 (D3: the egress surface exists by then); O-2's e2e leg (F-4 f, D3);
the I5 rows — backup/restore validation and deterministic-rebuild
verification (D3; the rebuild operation must be *designed* before it
can be verified — the MO-9 carryover stays visible); F-10's
human-outcome gate definitions (blocked on owner touchpoint #3, not on
engineering).

**Ordering self-check:** every accepted item above appears in exactly
one tranche (rejected items SDK-6 and SRV-5 are deliberately absent);
every edge cites a D-rule; the two rule exceptions (QF's SRV-2 timing,
W-5/W-6 adopt-without-RR&P) are called out in place rather than
smoothed. One known softness, stated: Tranche L "parallel with QF" has
no D-rule forcing internal order between QR-3's documentary items —
they are order-free among themselves by design.

### D.3 Semver mapping (soft estimates — Rule 01.4: plausible inference, not commitment)

Mechanics, stated before the numbers: one workspace version line
(`workspace.package.version = 0.1.0`), release-please over Conventional
Commits, so **all three product ladders share one number** — SDK feat
work rides whatever window it lands in. `test:`/`chore:`/`ci:`/`docs:`
move nothing; `fix:` moves patch; only `feat:` tranches appear below.
And the honest premise first: **release-please has no runner until
RR&P-1 lands (F-2), so nothing moves at all until Tranche CI** — at
activation it either parses accumulated history or starts from a
baseline tag (an RR&P-1 implementation detail; recommend baseline-tag
to avoid retroactive version archaeology).

| Tranche | feat content | Predicted version |
|---|---|---|
| QF + L | none — test/chore/docs/fix only | 0.1.x (patch drift at most, and only once release-please activates) |
| Muster Alpha (Phase 6 slice 3) | coordinator flow, coordinator DTOs, OTLP wiring, UI content; validation lib if RR&P-7 picks one | **≈ 0.2.0** |
| Orrery Beta + Phase 7 | second repository backend, differential activation, API freeze; ADR-0015 closes | **≈ 0.3.0** |
| Muster Beta (slice 4) | capacity, engagement, divergence, room assignment; load-tested server | **≈ 0.4.0** |
| MVP | auth, admin, location management; explain-assignment | **≈ 0.5.0** |
| RC → 1.0 | accessibility, ops, backup/restore; CP-SAT or documented rejection | **≈ 0.6.x, then 1.0.0 at RC exit** |

The quality-review's own accepted work is deliberately version-invisible:
it hardens gates without shipping features, which is exactly what
`test:`/`chore:` typing encodes. The only quality items that can move
the version are RR&P-7's validation adoption (feat(muster-server), rides
0.2.0) and Phase 7's second backend (feat(orrery), rides 0.3.0).

## Owner touchpoints (queued, not blocking)

1. **Sequencing vs Muster Alpha** — the plan's recommendation is
   satisfied by construction: QR-1/QR-2 closed before the Alpha
   pre-commitment; QR-3 can land in parallel with Alpha implementation.
   Residual question: confirm the Alpha pre-commitment adopts the
   Tranche-1 items (M-1..M-6, T-1/T-2, O-1/O-2, RR&P-7/8 closes) as
   entry criteria.
2. **CI platform** — GitHub Actions remains an inference (Rule 01.4);
   RR&P-1 cannot close without the owner confirming the repo host and
   hosted-vs-self-hosted preference. Sub-decision: the cargo-deny license
   allow-list (W-5) is a one-line owner call.
   *Answered 2026-08-03 (owner): **GitHub Actions confirmed.** The
   inference is now a decision; RR&P-1's remaining questions are runner
   strategy (hosted macOS vs Linux legs vs self-hosted) and the minimal
   first gate set. The W-5 license allow-list was implemented provisional
   (permissive set matching the workspace's MIT OR Apache-2.0) pending
   the one-line confirmation — and then **tightened on owner direction,
   same day**: the allow-list now carries only the seven load-bearing
   licenses the tree actually requires (deny.toml documents which crate
   forces each entry); a new license entering the tree fails `just deny`.
   One knob deliberately NOT tightened: `wildcards` stays `warn` because
   the only wildcards are workspace path deps and cargo-deny's exemption
   requires `publish = false` — publish intent is a new small owner
   question, queued for the first release-please release.*
3. **Human-protocol appetite** — how much usability formality pre-1.0:
   the "unaided" MVP gate definition, accessibility level for RC
   (RR&P-8 will propose a measurable floor), and MO-8's trial-protocol
   debts. F-10's gates stay owned-but-undefined until this is answered.
   *Status 2026-08-03 (owner): deferred knowingly — the owner wants to
   understand the question better before answering. Shape of when it
   actually bites: the a11y half arrives as RR&P-8's concrete proposal
   at the Alpha pre-commitment (react, don't pre-decide); the
   unaided/trial-protocol half is owed no earlier than the MVP
   pre-commitment. Nothing blocks before those points.*

## Stage-E handoff (what QR-3 lands, where — the mechanical map)

1. **`plans/TESTING-STRATEGY.md`** (new; the single cross-crate home,
   pre-agreed by plans/README:25-30): dimension taxonomy including the
   adopted S5/I5 rows · tool roster (adopted: cargo-deny, cargo-hack,
   cargo-flamegraph door, doc tests; per-RR&P candidate slates) ·
   variance policy (W-2) · property-test naming + case-budget rationale
   (W-1 policy, W-12) · regression policy (W-14) · test-double placement
   statements (T-4, C8 rationale) · reading list (folded references).
2. **Quality-tooling ADR** (next free number; Rule 06): records
   cargo-deny + cargo-hack adoption, the cargo-audit subsumption, the
   flamegraph door, and declares that RR&P-2/-4/-5/-7 dev-dependency
   picks land as dated amendments to it.
3. **Product testing specs**: orrery SPEC-05 (O-1..O-5 lines), muster
   SPEC-03 (M-1..M-6, T-1/T-2 sections), sdk SPEC-03 (SDK-1 correction,
   SDK-2 referent). muster-types/-server/-ui continue inheriting
   muster's spec set — QR-1's open question answered: no separate spec
   files; per-crate sections inside muster SPEC-03 instead (Rule 07:
   fewer homes).
4. **Rules**: Rule 09 wording (cargo-audit → cargo-deny advisories),
   Rule 05 (subscriber owner = muster-server; sdk span table; UI
   disposition UI-4).
5. **Dated corrections**: F-6 (two artifacts), F-9 (Rule 05/SPEC-02),
   F-13 sweep (twelve instances), F-14 (M-9), ADR-0025:97 false premise,
   measure_select header (W-2's doc half).
6. **CARRY-FORWARD**: new "Quality strategy — accepted items" section
   holding every implement-now item + RR&P row with owners exactly as
   classified here; no existing ledger row dropped.
7. **ROADMAP**: one dated ordering-and-semver section mirroring D.2/D.3.
8. **plans/README**: layout line for `quality-review/`, thin-spots fix.
9. **New files**: muster-types/AGENTS.md, muster-ui/AGENTS.md (T-3, UI-5).
10. **Gates**: check-xrefs after all edits; plain-language artifact
    `plans/orrery/artifacts/quality-review-<date>.md` (deliverable 5).

The QF slice (Tranche QF) is implementation, not landing — it runs as
its own small branch after QR-3, or in parallel by another session; QR-3
must not absorb it (the review plan's rule: synthesis is reviewable
before the corpus is rewritten, and code fixes are not amendments).

## Acceptance-criteria self-check (QR-2 scope)

| Criterion (from 00-review-plan.md) | Status |
|---|---|
| Seed triage — every Appendix seed dispositioned {adopt-now, RR&P-item, reject+reason}; review-added marked | Met: C.3 covers every Appendix line (crate-shaped and reading-material seeds) plus taxonomy-named and ⊕ review-added tools; 50 verified = 25 PASS + 23 FAIL + 2 UNCLEAR (arithmetic shown); every reject carries a written reason; ⊕ marks all review-added candidates; RR&P-9's four candidates deliberately unverified, reason stated |
| RR&P stages — question, candidates, deciding criteria, constraints, gated phase entry; no naked "investigate X" | Met: RR&P-1..9 each carry all five elements (C.2) |
| Findings integrity — every QR-1 Critical/Moderate disposition consumed | Met: F-1→W-1 · F-2→RR&P-1 · F-3→RR&P-2+SDK-2 · F-4→O-1/O-2/M-5/M-6/M-8/T-2 · F-5→W-2 · F-6→W-12+Tranche-L corrections · F-7→RR&P-3+O-5 · F-8→W-15/SDK-3/SDK-7 · F-9→SRV-6/SDK-4/M-7/UI-4 · F-10→RC tranche+touchpoint #3 · F-11→W-6/T-3/UI-2/UI-5 · F-12→W-3 · F-18→M-1..M-4/SRV-1/SRV-2+CR-3 carry (in O-7's Phase-7 bundle context) · Lows: F-13→Tranche L · F-14→M-9 · F-15→historical, no action (recorded) · F-16→W-12 · F-17→W-4 · F-19→O-7 bundle |
| Ordering — every accepted item exactly once; each edge cites D1–D5; violations called out | Met: D.2; two rule exceptions and one softness called out in place |
| Semver — every feat tranche mapped to a predicted 0.x, labeled soft | Met: D.3, with the release-please-has-no-runner premise stated first |
| Refutations-first · arithmetic checked (Rule 01.6) | Met: R-6..R-10 + hygiene corrections precede Stage C; counts checked (50/25/23/2; 19 findings consumed) |
| Taxonomy candidates decided | Met: S5 and I5 both adopted with definitions and initial dispositions (C.0) |
| Landing rules (Rule 07 single-home) | QR-3 scope; the handoff map above pre-assigns one home per fact so QR-3 can be mechanical |
