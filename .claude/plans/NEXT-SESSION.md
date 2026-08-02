# Next-session kickoff — plan review, then Phase 6 Prototype

*Written 2026-08-02 at the Phase-6 PoC close; review step made mandatory by
owner direction the same day. Paste the prompt below into a fresh session
in this repo. Delete or rewrite this file when the slice merges.*

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

**Step 0 — MANDATORY, before any Prototype work: adversarial review of
the remaining plans and ROADMAP** (owner-directed 2026-08-02). Run it in
the spirit of `.claude/agents/adversarial-reviewer.md` (arithmetic
integrity, cross-reference resolution, claim drift, consumer simulation),
scoped to *forward-looking* material: the unbuilt ROADMAP stages
(Muster Prototype→RC, SDK Beta/RC, Orrery Alpha→RC), every phase doc's
Carry-forward table, the specs the future work builds on, and
ADR-0015/0021's Phase-7 plan. Hunt for completeness/robustness gaps, not
style. Known suspects to verify or refute — do not stop at them:

* **The salsa mirror assumes a single writer through one `Engine`** — a
  second writer process on a real datastore (Stage C!) silently breaks
  invalidation. Nobody has written this down as a Phase-7 requirement.
* Blast-radius preview (Muster Alpha) needs a **non-persisting** digest
  dry-run; the engine only has the persisting `refresh_digests`.
* `expired_membership_effect` has no producer (no persisted derived
  cache exists); who builds it, and when?
* Engine analytics (engagement/capacity/divergence) — ROADMAP'd for
  Orrery Alpha, needed by Muster Beta, owned by **no phase document**.
* `Warn` policy semantics undefined; SPEC-03 interactive budgets never
  measured (`select()` sweeps the whole window); RC items
  (backup/restore, deterministic rebuild, auth/tenancy) have no owning
  phase.

Deliverables: a findings document at
`.claude/plans/orrery/artifacts/plan-review-YYYY-MM-DD.md` (tiered
critical/moderate/low, each with the doc+line it stems from), a
consolidated cross-phase carry-forward ledger, and — where a finding
demands new work — phase-doc/ROADMAP amendments as visible, dated edits.
Zero findings in a category is a reportable result, not a skipped one.
**Fold the findings into the Prototype pre-commitment below before
writing it**; if a critical finding reshapes the slice, say so and
reshape it.

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
