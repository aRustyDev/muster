# Next QR session — QR-3 (Stage E)

*Rewritten 2026-08-03 at QR-2 close per the slice close-out protocol.
Paste the prompt below into a fresh session in this repo. After QR-3
closes: delete this file, mark the `quality-review-state` memory
complete, and record review completion in CARRY-FORWARD.*

---

You are continuing the Orrery/Muster **quality-strategy review**. Your
project memory has `quality-review-state` — trust it for orientation;
the repo is the state of truth. Read ONLY
`plans/quality-review/00-review-plan.md` (binding: method, acceptance
criteria, execution architecture, this protocol) and
`plans/quality-review/02-additions-and-order.md` (QR-2's deliverable:
per-crate 'to add' lists W-*/O-*/SDK-*/M-*/T-*/SRV-*/UI-*, RR&P-1..9,
the 50-seed triage table, ordering tranches, semver mapping — and the
**Stage-E handoff section, which is your landing map**). QR-3 runs
INLINE per the plan's execution architecture — no fan-out; Rule 07
single-home placement needs one context holding the corpus map. You
will necessarily open the ~10 documents you edit; that is landing, not
corpus re-reading.

This session: **QR-3 (Stage E)** — land everything as dated amendments
per the handoff map: create `plans/TESTING-STRATEGY.md` + the
quality-tooling ADR (next free number — verify against docs/src/adrs/);
amend product testing specs, Rules 05/09, ROADMAP (ordering+semver
section), CARRY-FORWARD ("Quality strategy — accepted items"),
plans/README; dated corrections (F-6, F-9, F-13 sweep, F-14,
ADR-0025:97, measure_select header); new AGENTS.md for
muster-types/muster-ui; plain-language artifact
`plans/orrery/artifacts/quality-review-2026-08-03.md`. Deliverable: the
amendments themselves — mechanical if QR-2 was honest. NOT in scope:
the QF implementation slice (justfile/code fixes) — that is a separate
implementation branch after QR-3. Gates: `just audit` green (the
xref script; there is no `check-xrefs` recipe), no silently dropped
ledger rows, refutation numbering continues at R-11. Close per the
slice close-out protocol (compaction-ready close: commit, update
`quality-review-state`, then retire this file as described above).
Conventional Commits;
`Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>`.
