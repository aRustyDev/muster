# Next-session kickoff — Phase 6 Prototype

*Written 2026-08-02 at the Phase-6 PoC close. Paste the prompt below into a
fresh session in this repo. Delete or rewrite this file when the slice
merges.*

---

You are continuing the Orrery/Muster build in this repo. Your project
memory has the orientation pointers; the repo is the state of truth.

**Ground yourself (short — you have done this before):**
`.claude/CLAUDE.md` + `.claude/rules/**` (binding; Rule 01 especially),
`.claude/plans/PLAN.md` current-state table, then
`.claude/plans/muster/phases/06-app.md` — its Carry-forward table IS the
backlog for this slice. Also: `muster/specs/02-api-surface.md` (the
service surface you are extending), `muster/questions/0015`,
`PHASE-TEMPLATE.md` (note the standing plain-language-artifact criterion).
Workspace state: 66 tests green on `main`; run
`cargo nextest run --workspace` before you start and stop if that is not
what you see.

**Optional step 0 (owner to confirm): adversarial review.** If directed,
run the adversarial pass over remaining plans/ROADMAP first (scope sketch
in the phase-6 PoC session close-out) and fold findings into this slice's
pre-commitment before writing it.

**The slice: Muster Prototype** (ROADMAP gate: "browse, select, priority,
my-schedule with provenance — member flow complete").

1. Pre-commit hypotheses + acceptance criteria as a new slice section in
   `06-app.md` **before implementing** (include the standing artifact
   criterion). Branch `feat/phase-06-prototype`.
2. **Close QUESTION-0015 with ADR-0025.** The recorded leaning is
   muster-server (axum) + muster-ui (dioxus) + thin muster-types crate —
   but verify against the current state of those ecosystems by WEB SEARCH,
   not recall (the paper screen's "search, don't recall" rule applies to
   dependency decisions too). Whatever wins: new crates under
   `crates/muster/` per the leaning, or one fullstack crate with the
   extraction cost recorded. UI deps stay out of muster-sdk
   (`check-scope`) and orrery (`check-seam`).
3. Full member flow through the service layer: browse (events with
   room/time), select with priority, my-schedule (already landed —
   extend, don't rewrite). `SetPriority` gets a service call. Replace the
   PoC's whole-window sweep in `select()` with a person-scoped evaluation
   if the interactive budget demands it — measure before optimising.
4. `figment` + tracing-subscriber enter muster when the first real config
   knob exists (exporter selection per Rule 05) — likely this slice.
5. Gates as always (nextest, clippy -D warnings, fmt, check-seam grep
   fallback — no rustup on this host — check-scope, check-xrefs). e2e_
   test for the member flow. Privacy: any new coordinator-facing DTO gets
   a privacy_ test on a world WITH anchors.
6. Results (refutations first), plain-language artifact in
   `plans/muster/artifacts/`, merge `--no-ff`, update PLAN rows, report.

Carry-forwards NOT in this slice (leave them recorded): coordinator flow +
blast-radius preview (Alpha — preview needs a non-persisting digest
dry-run, an engine design item), engine analytics surface, Phase 7
dossier items. Conventional Commits; `Refs:` footers for work items;
`Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>`.
