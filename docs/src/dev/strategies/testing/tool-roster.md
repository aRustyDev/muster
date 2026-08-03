# Testing strategy: tool roster and open decisions

*Decomposed 2026-08-03 from `plans/TESTING-STRATEGY.md` (QR-3/Stage E)
by CR-2 under ADR-0027 — see [coverage-taxonomy](coverage-taxonomy.md)
for the full provenance note. Changes land as dated amendments.*

Adopted tools enter by ADR (ADR-0026); RR&P picks land as dated
amendments to that ADR when their stage closes.

| Tool | Status | Door |
|---|---|---|
| cargo-nextest | incumbent runner (does **not** run doctests — see [standing gates](../../policies/testing/standing-policies.md)) | `just test` |
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

## Open tool decisions (RR&P stages — full definitions in `plans/quality-review/02-additions-and-order.md` §C.2)

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
