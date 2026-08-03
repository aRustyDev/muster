# Next-session kickoff — Muster Alpha (Phase 6 slice 3)

*Rewritten 2026-08-02 at the Phase-6a close (this file is rewritten at
each slice merge). Paste the prompt below into a fresh session in this
repo.*

---

You are continuing the Orrery/Muster build. Your project memory has the
orientation pointers; the repo is the state of truth.

**Ground yourself (short — you have done this before):**
`.claude/CLAUDE.md` + `.claude/rules/**` (binding; Rules 01/03/04/09
especially), `.claude/plans/PLAN.md` current-state table,
`.claude/plans/CARRY-FORWARD.md` **Muster Alpha section — the slice
pre-commits against every row there**, `muster/phases/06-app.md`
(slices 1–2 Results), and `orrery/phases/06a-engine-surfaces.md` (the
engine surfaces you now have: `preview_digests` with the honesty
property already proven engine-side, `analytics`, `AddAnchor` +
`first_event_feasibility`). Workspace state: **93 tests green on
`main`** (+1 deliberately ignored measurement harness; count updated
2026-08-03 — the QF quality-fixes slice added `error_contract` and
`wire_names`); run `cargo nextest run --workspace` before you start and
stop if that is not what you see.

**Quality-review inputs to the pre-commitment (added 2026-08-03, QR-3):**
the slice pre-commitment also adopts the Alpha-entry quality items —
M-1..M-6, T-1/T-2, O-1 + O-2-design (all specified in the products'
testing specs, dated 2026-08-03) — and closes **RR&P-7** (wire-input
validation: library or hand-rolled `TryFrom`) and **RR&P-8** (UI testing
approach + a11y floor) *in* the pre-commitment, one slice ahead of their
implement items (rule D5, minimum spacing). See CARRY-FORWARD "Quality
strategy — accepted items" and the testing strategy/policies under
`docs/src/dev/` (decomposed 2026-08-03, ADR-0027).

**The slice: Muster Alpha (06-app.md slice 3) — coordinator flow.**
ROADMAP gate: **"groups, expectations, blast-radius preview, violation
inbox — coordinator flow complete."** Branch `feat/phase-06-alpha`.
Write the slice-3 pre-commitment in `muster/phases/06-app.md` BEFORE
implementing (hypotheses + criteria + the standing plain-language
artifact row). The ledger's Muster Alpha section is the scope contract;
its rows, compressed:

1. Coordinator service calls (`create_group`, `add_member`, `expect`)
   replacing the `engine_mut` escape hatch; inbox + waive (actor +
   timestamp per Rule 09).
2. Blast-radius preview: service `preview_expectation` over
   `Engine::preview_digests` + the muster-level honesty property test
   (muster/SPEC-03:17-21) + presentation.
3. **Person-scoped `select()` evaluation** — non-optional (slice-2 H4
   refuted in spirit: whole-window sweep measured p50 97.8 ms / p95
   102.4 ms at 10³ persons, zero headroom). Conflicts must still land
   as records.
4. `Warn` policy: define observable semantics or shrink the enum by ADR;
   document partial-`Prevent` (2 of 7 kinds) in a spec note.
5. Group-scoped violation query (`inbox(filter)`): decide repo query vs
   engine surface vs measured app-side join.
6. Retraction commands beyond `RemoveAttendance` (membership /
   expectation / hold end-or-shorten) — CR-6 follow-through; audit
   `refresh_after` for every new kind (membership/expectation kinds DO
   touch mirrored facts, unlike the removal precedent).
7. Privacy tests extend to coordinator-facing DTOs on worlds with
   anchors (worlds exist now — Phase 6a).
8. Severity defaults product confirmation (owner touchpoint if needed).
9. OTLP exporter wiring behind the existing `exporter` knob (needs a
   collector; keep deferral honest if none exists).
10. muster-ui REST client + `dx` web entrypoint + UI content
    (components/type-sharing landed in slice 2).

Known traps: `incremental::refresh_after` string-matches command kinds —
new membership/expectation retraction commands MUST be added to the
match by hand (and tested); the `AddAnchor`/`RemoveAttendance` audit
comments show the no-op form. `Engine::preview_digests` rejects
non-mirrored kinds with `PreviewUnsupported` (muster-server already maps
it to 400).

Gates as always (nextest, clippy -D warnings, fmt, test-doc, doc-check,
deny, check-seam grep fallback — no rustup on this host — check-scope,
check-oneway, check-xrefs; `just ci` covers the workspace set since the
QF slice).
Results refutations-first; plain-language artifact in
`plans/muster/artifacts/`; merge `--no-ff`; update PLAN rows and tick
the ledger; rewrite this file. Conventional Commits; `Refs:` footers;
`Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>`.
