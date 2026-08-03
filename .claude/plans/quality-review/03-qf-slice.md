# QF slice — quality fixes (Tranche QF of 02-additions-and-order.md)

*Pre-committed 2026-08-03 before implementation, per the phase cadence.
Branch `chore/qf-quality-fixes`, merged `--no-ff` when the criteria below
are green. Scope is exactly Tranche QF: W-1, W-2 (harness half), W-3,
W-4, W-5, W-6, W-13, W-15, SDK-3, SDK-7, SRV-1, SRV-2, M-7. No spec or
rule text changes here — those are QR-3's landing. Owner decisions
consumed: GitHub Actions confirmed (2026-08-03); W-5 license allow-list
implemented provisional pending the one-line confirmation.*

## Pre-committed acceptance criteria

| # | Criterion (executable) |
|---|---|
| 1 | `just test-prop` exits 0 and runs >0 tests from BOTH families: orrery `prop_` and sdk `optimality_`/`monotone_` |
| 2 | `just orrery::test-detectors` exits 0 and runs >0 tests; `just orrery::test-incremental` exits 0 and runs >0 tests |
| 3 | `just matrix` exits 0: cargo-hack each-feature legs (incl. the muster-ui bare-library leg, F-11) + the repo-memory test leg; prints the Phase-7 deferral instead of erroring on repo-sqlite |
| 4 | `just differential`, `just bench`, `just orrery::bench`, `just orrery::bench-canonical` each FAIL loudly with a pointer (no silent success, no bare cargo error) |
| 5 | `just test-doc` exists, runs workspace doctests (0 today), and is part of `just ci` |
| 6 | `just deny` runs `cargo deny check` green with a provisional permissive allow-list; cargo-deny/cargo-hack added to `doctor` |
| 7 | check-scope FAILS the build on an injected match (the current recipe provably cannot fail — see refutation QF-R1); denylist broadened beyond four names |
| 8 | `just muster_sdk::check-oneway` exists, green, and greps the same pattern the phase-doc boundary greps pre-committed (`ViolationKind::` / `detect::` / `overlaps(`) |
| 9 | check-seam output names which arm ran (public-api vs grep-fallback) |
| 10 | One definition of severity cost: orrery exposes `severity_weight` publicly; sdk `ViolationCost` consumes it; the 100/10/1 table appears in exactly one place; suite green |
| 11 | Error→status mapping tested for all 5 `OrreryError` variants (was 1 of 5) |
| 12 | Wire names pinned: an e2e asserts the Debug-name strings that cross the wire today (kind + severity of the demo conflict), using string literals only — the muster-family boundary greps stay empty |
| 13 | `[workspace.lints]` forbids `unsafe_code` workspace-wide via `[lints] workspace = true` in every crate; `[profile.release]` pins the defaults the measure_ baselines were taken under; workspace builds |
| 14 | measure_select: header contradiction corrected (dated); warm-up + 3 measured rounds, median/max across rounds reported; sanity bound kept |
| 15 | measure_alpha_budgets: variance protocol documented in header (process-level ≥3 runs; in-process repetition would measure warm paths — stated); per-person sample blocks get an untimed warm-up call |
| 16 | run_all.sh pins ladybug==0.19.0 (the version 00-grounding records) with a dated comment |
| 17 | Full suite green (`just test` — 91 green + 1 ignored, plus the new tests); `just lint`, `just fmt-check`, `just audit` green |

Deviation from the deliverable, recorded: W-13 lands as
`[workspace.lints.rust] unsafe_code = "forbid"` + per-crate
`[lints] workspace = true` instead of six `#![forbid(unsafe_code)]`
attributes — same enforcement, one home (Rule 07), covers binary targets
too. Clippy severity deliberately stays in the `lint` recipe (`-D
warnings` at gate time, not at every dev build); the lints table carries
only always-true policy.

## Hypotheses

* H-QF1: all five broken doors are one-recipe-line fixes; no test
  renames needed (filter unions suffice).
* H-QF2: the workspace's transitive licenses fit a standard permissive
  allow-list without exceptions.
* H-QF3: single-sourcing severity weights changes no behavior (suite
  stays green with zero test edits beyond the strengthened assert site).

## Results (refutations first)

**QF-R1 — check-scope was worse than F-8 recorded: not narrow, INERT.**
The recipe's `grep … && (echo … && exit 1) || true` shape meant the
subshell's `exit 1` was swallowed by `|| true` — verified by execution
before the fix (`echo "axum v0.8" | grep axum && (exit 1) || true`
exits 0). The gate that "fails the build if a UI dependency leaks in"
could never fail, on any dependency, since it was written. F-8 said the
denylist was too narrow; the truth is the denylist was never consulted
for the exit code at all. Fixed with a bash recipe that word-boundary
matches `<crate> v` in cargo-tree output; an injected match now exits 1
(proven in the gate run below).

**QF-R2 — automating the one-way grep immediately surfaced a scope
question the manual protocol never answered.** The first run of
`check-oneway` over `src/ tests/` FAILED: tests/optimality.rs
re-implements `overlaps()` — deliberately, it is the brute-force oracle
SPEC-03 requires to be independent — and suggest_integration.rs matches
on an engine-produced kind (consuming, not constructing). The spec's
sentence is "SDK *sources* construct no ViolationKind"; the gate is now
scoped to `src/` with the exclusion reason written into the recipe.
QR-3 should mirror this scope note into sdk SPEC-03's boundary section.

**H-QF1 (one-line fixes, no test renames) — held**, with one addition:
the detector filter needed the real family names
(`matches_oracle`/`matches_naive`/`oracle_`), not a `prop_` variant.
**H-QF2 (standard permissive allow-list suffices) — held**: cargo-deny
green with zero exceptions (advisories, licenses, bans, sources all ok).
**H-QF3 (severity single-sourcing changes no behavior) — held**: suite
green with zero test edits; the strengthened-assert option was not
needed — the integration assert stands and the claim "one definition,
two call sites" is now true by construction.

### Acceptance criteria — all 17 met (this host, 2026-08-03)

| # | Evidence |
|---|---|
| 1 | `just test-prop`: **25 passed** (orrery `prop_` 23 + sdk `optimality_`/`monotone_` 2) |
| 2 | `test-detectors` **18 passed**; `test-incremental` **6 passed** |
| 3 | `just matrix`: cargo-hack each-feature legs green (incl. muster-ui bare) + 77 repo-memory tests + Phase-7 deferral line, exit 0 |
| 4 | `differential` / `bench` / `orrery::bench` / `bench-canonical` all exit 1 with RR&P-2/Phase-7 pointers |
| 5 | `just test-doc` runs 6 doctest targets (0 doctests yet, exit 0); in `ci` |
| 6 | `just deny` green ("advisories ok, bans ok, licenses ok, sources ok"); doctor checks both new tools |
| 7 | injected `axum v0.8.4` line → exit 1; denylist now 14 name-stems, word-boundary matched |
| 8 | `check-oneway` green at src/ scope (see QF-R2) |
| 9 | check-seam prints `arm: grep-fallback (…)` on this host |
| 10 | `orrery::engine::severity_weight` public; sdk `ViolationCost` consumes it; the 100/10/1 table exists once (engine.rs); 93/93 green |
| 11 | `error_contract.rs`: all 5 variants pinned at the mapping seam |
| 12 | `wire_names.rs`: `TimeConflict`/`Hard` pinned via string literals over a real wire conflict; boundary greps stay empty |
| 13 | `[workspace.lints.rust] unsafe_code = "forbid"` + 6 crate opt-ins; `[profile.release]` pins the baseline defaults; builds clean |
| 14 | measure_select: dated header correction; warm-up + 3 rounds; median-of-p50 asserted; ran green in-suite (6.4s debug) |
| 15 | measure_alpha_budgets: variance protocol in header (process-level ≥3 runs; in-process repetition would measure warm paths); per-block warm-up call; execution-verified by a release run at close |
| 16 | run_all.sh pins `ladybug==0.19.0` (the 00-grounding version) with dated comment |
| 17 | `just ci` green end-to-end (fmt-check, lint, test 93/93+1 skip, test-doc, doc-check, deny); `just audit` green |

Suite count: 91 → **93 green** (+error_contract, +wire_names), 1
deliberately-ignored harness unchanged.

### Handoff notes (consumed by QR-3 and RR&P-2)

* QR-3: mirror QF-R2's src/-scope note into sdk SPEC-03; the SPEC-03
  boundary line can now cite `just muster_sdk::check-oneway` instead of
  "source grep at phase close"; muster AGENTS/docs referencing `run-dev`
  on muster should point at `muster_server::run-dev`.
* RR&P-2: the `bench`/`bench-canonical` doors now point here — closing
  RR&P-2 replaces both guards with real targets.
* Owner (one line, queued): confirm the provisional deny.toml license
  allow-list.
