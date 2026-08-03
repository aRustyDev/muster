# 26. Quality tooling baseline: cargo-deny, cargo-hack, and the profiling door

* Status: accepted
* Date: 2026-08-03
* Source: quality-strategy review (QR-2 synthesis,
  `plans/quality-review/02-additions-and-order.md`; landed by the QF
  slice and QR-3)

## Context and Problem Statement

The quality review found the supply-chain and license dimensions dormant
(Rule 09 promised cargo-audit "once CI exists" — CI does not exist; no
license tooling at all), the feature matrix broken (the `matrix` recipe's
repo-sqlite leg errored, and neither documented muster-ui configuration
was ever gate-exercised — findings F-1/F-11), and no documented profiling
path. Rule 06 requires an ADR for tools entering `orrery`'s dependency
set — ADR-0022 lists dev-dependencies — so tool adoptions from the review
land by ADR even where the tool is a CLI gate that enters no `Cargo.toml`.

## Decision Drivers

* Rule 06 bars: maintained (releases within a year, >1 contributor), not
  reasonably self-implementable, no transitive runtime.
* Rule 09's standing supply-chain promise needs a runnable local door
  now, not a CI-conditional one.
* One tool per job: subsumption preferred over tool-per-dimension.
* Ordering rule D2 (measure before optimize): profiling is a documented
  door, not a scheduled activity.

## Considered Options

Per dimension, from the review's 50-tool triage (verdicts as of
2026-08-03 in `02-additions-and-order.md` §C.3):

1. Supply chain + licenses: **cargo-deny** · cargo-audit (advisories
   only) · cargo-lichking (dormant since 2020)
2. Feature combinations: **cargo-hack** · cargo-featomatic (never
   released on crates.io)
3. CPU profiling door: **cargo-flamegraph** · samply (active repo, no
   release since 2025-02 — bar caveat R-9) · tracing-flame (no release
   in ~4.5 years) · pprof-rs (host fit poor)
4. Doctests: **`cargo test --doc`** · skeptic (dormant) · docmatic
   (archived)

## Decision Outcome

* **cargo-deny** (0.20.x, EmbarkStudios) is the supply-chain and license
  gate: `just deny` runs `cargo deny check` (advisories, licenses, bans,
  sources) and is part of `just ci`. It **subsumes cargo-audit's
  advisories check**; cargo-audit (rustsec, PASS) remains the named
  fallback. Rule 09's wording is amended accordingly (dated,
  2026-08-03). The `deny.toml` license allow-list was tightened on owner
  direction (2026-08-03) to the seven licenses the tree strictly
  requires, each entry justified in the file; `wildcards` stays `warn`
  pending the workspace publish-intent decision (owner question, queued).
* **cargo-hack** (0.6.x, taiki-e) provides the feature-matrix legs:
  `cargo hack check --workspace --each-feature` including the
  no-features leg — which makes muster-ui's documented bare-library
  configuration gate-exercised for the first time. Full repo-* backend
  legs remain Phase-7-owned (ADR-0021 funnel).
* **cargo-flamegraph** is the documented CPU-profiling recipe (macOS
  backend: xctrace), with **samply** as the interactive alternate (bar
  caveat recorded; an operator tool entering no manifest) and Instruments
  direct as the third door. On-demand only (D2).
* **Doctests** run via `cargo test --doc` (`just test-doc`, in `ci`) —
  no dependency; nextest does not execute doctests, which made this a
  structural blind spot (F-12).

**Recorded exception**: W-5/W-6 adopted without an RR&P stage despite
the review's "library picks default to RR&P" rule — the triage verdicts
were one-sided (each candidate's alternatives all failed the bar) and
both are CLI gate tools outside every manifest. The exception is written
down so the rule stays falsifiable.

**Amendment protocol**: dev-dependency picks from the open RR&P stages —
RR&P-2 (bench harness), RR&P-4 (coverage), RR&P-5 (mutation), RR&P-7
(validation) — land as **dated amendments to this ADR** when their stage
closes, not as new ADRs, unless a pick violates a Rule 06 bar (that
would need its own reasoning).

### Consequences

* Two new toolchain prerequisites (`cargo-deny`, `cargo-hack`) for every
  contributor and eventually CI; `just doctor` checks both.
* A new license entering the dependency tree fails `just deny` until the
  allow-list is deliberately extended — friction by design.
* The consequence we dislike, recorded (ADR-0020 discipline): adopting
  gate tools **before CI exists** means they run single-host on operator
  discipline; their guarantee is only as continuous as the habit of
  running `just ci`. RR&P-1 is what turns them into standing gates —
  until it closes, this ADR hardens doors, not schedules.
