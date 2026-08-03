# TESTING-STRATEGY — the cross-crate quality strategy

*Created 2026-08-03 by QR-3 (Stage E) of the quality-strategy review.
Provenance: `quality-review/00-review-plan.md` (method),
`quality-review/01-gap-matrix.md` (evidence), and
`quality-review/02-additions-and-order.md` (synthesis — item IDs like
W-2/O-3/RR&P-4 below resolve there). This file is the **single cross-crate
home** (Rule 07): product testing specs carry per-crate criteria and link
here; nothing stated here is restated there. Changes land as dated
amendments.*

## Taxonomy

Quality coverage is tracked against the review's dimension matrix:
C1–C19 (correctness), P1–P13 (performance), S1–S5 (safety & robustness),
I1–I5 (infrastructure & meta) — 42 dimensions after the two review-added
rows, whose definitions live here:

* **S5 Privacy testing** — *executable assertions that privacy invariants
  hold on every egress channel: wire payloads, logs, spans, errors,
  exports, session state.* The corpus's hardest gate class (Rule 00.6,
  Rule 09) gets its own row so written-but-unasserted promises are visible
  as such. Cell dispositions live in the gap matrix.
* **I5 Ops-validation / DR** — *executable validation of operational
  procedures: backup/restore, deterministic rebuild, config sanity,
  migration drills.* Exists so the RC gates ("backup/restore",
  "deterministic rebuild verified") have a dimension to live in; all cells
  stay gap/N/A until the RC pre-commitment defines them.

## Tool roster

Adopted tools enter by ADR (ADR-0026); RR&P picks land as dated
amendments to that ADR when their stage closes.

| Tool | Status | Door |
|---|---|---|
| cargo-nextest | incumbent runner (does **not** run doctests — see below) | `just test` |
| proptest | baseline property framework (ADR-0022); the only one — quickcheck rejected for single-framework discipline, not maintenance | `just test-prop` |
| cargo-deny | adopted 2026-08-03 (W-5): advisories + licenses + bans + sources; subsumes cargo-audit's advisories check (cargo-audit stays the named fallback) | `just deny` (in `ci`) |
| cargo-hack | adopted 2026-08-03 (W-6): each-feature legs incl. the no-features leg | `just matrix` |
| `cargo test --doc` | adopted (W-3) | `just test-doc` (in `ci`) |
| cargo-flamegraph | the documented CPU-profiling recipe (W-7; macOS backend is xctrace) | on demand |
| samply | interactive profiling alternate (operator tool, enters no manifest; R-9 release-clause caveat recorded) | on demand |
| Instruments | third profiling door (macOS) | on demand |

Profiling is **on-demand, never scheduled** (ordering rule D2: measure
before optimize; no binary hot path has ever been profiled, so PGO is
rejected-for-now with cargo-pgo as the presumptive tool if that changes).

### Open tool decisions (RR&P stages — full definitions in 02-additions-and-order.md §C.2)

| Stage | Question | Candidates | Closes |
|---|---|---|---|
| RR&P-1 | CI bring-up (platform: **GitHub Actions, owner-confirmed 2026-08-03**; runner strategy + first gate set open) | hosted macOS / Linux legs / self-hosted OrbStack | before Orrery-Beta entry at latest |
| RR&P-2 | micro/macro perf harness, baselines, gate mechanism | criterion (presumptive) · divan (R-9 caveat) · hyperfine · Bencher (CI tier) | pick + orrery bench skeleton ≈ Muster Alpha |
| RR&P-3 | coverage-guided fuzzing on this host; the real "incremental fuzz green" definition | afl.rs (front-runner: stable Rust, ARM64 macOS) · cargo-fuzz · honggfuzz | before Orrery-Beta entry |
| RR&P-4 | coverage tool + reporting policy (informational-first) | cargo-llvm-cov (front-runner: documented no-rustup path) · cargo-tarpaulin | local leg any time; CI leg after RR&P-1 |
| RR&P-5 | mutation rollout scope/cadence | cargo-mutants (expected sole survivor) | after RR&P-4 (D2) |
| RR&P-6 | API-freeze diff tooling | cargo-semver-checks (presumptive) · cargo-public-api (CI-Linux-leg only — needs rustup nightly) | before Orrery-Beta entry |
| RR&P-7 | wire-input validation library, or none | hand-rolled `TryFrom` (null hypothesis) · validator · garde · axum-valid | at Muster-Alpha pre-commitment |
| RR&P-8 | UI testing approach, REST-client double, insta?, wasm-perf + a11y dispositions | dioxus-ssr+insta · plain asserts · wasm-bindgen-test · defer-with-reason | at Muster-Alpha pre-commitment |
| RR&P-9 | HTTP load/stress/spike/soak harness + workload shapes | oha · goose · drill · k6 (deliberately unverified until the stage opens) | at Muster-Beta pre-commitment |

## Measurement-variance policy (W-2 — applies to every `measure_` harness and any successor)

1. **Warm-up**: at least one untimed warm-up pass before any timed sample.
2. **Iterations**: at least 3 measured iterations — in-process rounds
   where state permits (measure_select), separate process-level runs where
   in-process repetition would measure warm paths (measure_alpha_budgets;
   the harness header must say which and why).
3. **Statistic**: report the median across rounds/runs and the max
   (or cross-run p50/p95); never a single run.
4. **Profile**: measurements run under the release profile pinned in the
   workspace `Cargo.toml` `[profile.release]`; the header names the profile.
5. **Assertions**: in-suite assertions are order-of-magnitude sanity
   bounds (10×), never knife-edge budgets — the H4 precedent (97.8 ms
   median vs a 100 ms budget) is exactly the flake a threshold assertion
   would create. Budget *verdicts* live in phase docs, with provenance.
6. **Reproduction tolerance**: a quoted median reproduces if a re-run
   median lands within 2× (the order-of-magnitude claim class these
   harnesses make). Any gate needing finer resolution pre-commits its own
   tolerance in the harness header *before* measuring (Rule 01.1/01.2).
7. **Provenance and pins**: every quoted number names its script, scale,
   host, and date (Rule 01.5); measurement toolchains are version-pinned
   (`evidence/run_all.sh` pins `ladybug==0.19.0`).

## Property tests: naming and case budgets (W-1 policy, W-12)

* **Naming**: new property tests take the `prop_` prefix, workspace-wide.
  The legacy sdk families (`optimality_`, `monotone_`) stay as named;
  `just test-prop` unions all three filters.
* **Case budget**: 48 cases is the deliberate default — wall-clock
  affordable on the gate path (the whole suite stays fast enough to run on
  every merge). Deep runs raise it via `PROPTEST_CASES` without code
  changes. The adversarial 1-eval search budget bound
  (`search_quality.rs`) is likewise deliberate test design — it pins
  behaviour under the *tightest* budget, not a typo to "fix".

## Regression policy (W-14)

Every refuted hypothesis and every fixed defect lands a **pinned test
named for the finding** (the H1 travel-shortcut practice, now written
down). proptest-regressions seed files are committed. A refutation
without a pinned test is an undone fix.

## Test doubles: placement and strategy (T-4, C8)

* The restrictive **MemoryRepo fake is the deliberate double** for every
  repository consumer (ADR-0021; Rule 00b makes its constraints
  executable). Mock frameworks are rejected: a mock would let the
  `Repository` trait absorb assumptions the fake exists to block. The DI
  seams are generic parameters (`Engine<R>`, `MusterService<R>`), not
  injected trait objects.
* **DTO contract tests live in muster-server by design** (ADR-0025: the
  privacy boundary's single enforcement point). muster-types having no
  tests of its own is a decision, not an accident; its first own tests
  (serde roundtrip properties, muster/SPEC-03) arrive with its first
  input surface at Muster Alpha.
* **Time**: the engine reads no clock; the binary edge owns time —
  time-mocking gotchas are designed out, not mocked around.
* **UI REST-client double**: RR&P-8's call, at the Alpha pre-commitment.

## Standing policies

* **Doctests**: rustdoc examples are tests; nextest does not run them.
  `just test-doc` is the door and is part of `just ci` (W-3). Orrery's
  public API gets rustdoc examples at Beta freeze (O-4).
* **Safe Rust**: `unsafe_code = "forbid"` workspace-wide via
  `[workspace.lints.rust]` (W-13). miri/sanitizer runs are CI-Linux-leg
  items inside RR&P-1 — the no-rustup host cannot run them natively.
* **Data egress** (S5/M-8): Parquet/CSV export excludes `anchors` by
  default (Rule 09); the RC pre-commitment owes the executable test.
* **Funnel discipline** (D4): no benchmark touches a datastore candidate
  before the ADR-0021 stage permits one; the RR&P-2 bench skeleton is
  MemoryRepo-only until Phase 7.
* **Gate honesty**: recipes that cannot run yet **fail loudly with a
  pointer** (`bench`, `differential`) rather than passing over nothing;
  gate output records which arm ran where fallbacks exist (check-seam).

## Reading list (references folded from the review's seed triage)

Micro-bench: criterion book · bencher.dev docs. Perf: nnethercote's
perf-book · brendangregg flamegraphs · cargo-pgo (kobzol) — with the PGO
reject-for-now note. Mocking rationale: mock_shootout ·
conditional-compilation pattern (klau.si) · time-mocking gotchas
(blog.iany.me). Coverage: tarpaulin/users.rust-lang threads. Fuzzing:
rust-fuzz book. General: awesome-rust-testing · llogiq on testing.
Validation surveys: folded into RR&P-7. wasm time-profiling (rustwasm
book): folded into RR&P-8. Async-profiling material: noted low-relevance
— the workspace is synchronous by design (ADR-0023; async undecided per
Rule 04).
