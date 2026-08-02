# Next-session kickoff — Phase 6a: engine surfaces for the app

*Rewritten 2026-08-02 at the Phase-6 Prototype close (this file is
rewritten at each slice merge). Paste the prompt below into a fresh
session in this repo.*

---

You are continuing the Orrery/Muster build. Your project memory has the
orientation pointers; the repo is the state of truth.

**Ground yourself (short — you have done this before):**
`.claude/CLAUDE.md` + `.claude/rules/**` (binding; Rules 01/03/04
especially), `.claude/plans/PLAN.md` current-state table and its
**Phase 6a** section, `.claude/plans/CARRY-FORWARD.md` (the living
backlog — Phase 6a and Muster Alpha sections are your scope), the
2026-08-02 plan review (`orrery/artifacts/plan-review-2026-08-02.md`,
findings CR-1/CR-2/CR-4), and `muster/phases/06-app.md` slice-2 Results.
Workspace state: **72 tests green on `main`**; run
`cargo nextest run --workspace` before you start and stop if that is not
what you see.

**The slice: Phase 6a (orrery) — engine surfaces the app's next stages
need.** Branch `feat/phase-06a-engine-surfaces`. Write
`.claude/plans/orrery/phases/06a-engine-surfaces.md` with pre-committed
hypotheses/criteria (PHASE-TEMPLATE; standing plain-language-artifact
criterion) BEFORE implementing. Contents, from PLAN.md Phase 6a:

1. **Non-persisting digest dry-run** — evaluate a hypothetical
   expectation/membership against current state and return the would-be
   change set WITHOUT writing (blocks Muster Alpha's blast-radius
   preview; the honesty gate is pre-specified: preview must equal the
   post-commit `refresh_digests` change set, property-tested —
   muster/SPEC-03:17-21). Design constraint from review CR-1: the salsa
   mirror invalidates only through `Engine::apply` — an overlay
   evaluation must not corrupt it.
2. **Analytics surface**: engagement, capacity pressure, divergence,
   bounded 2-hop co-attendance (orrery/SPEC-02 FRs; 2-hop budget <50 ms
   p95 pre-committed in orrery/SPEC-03:14). Blocks Muster Beta.
3. **Define the 10⁵ budget set** the Orrery Alpha gate references (or
   restate the gate at 10⁶ with a dated ROADMAP edit).
4. **Anchor producer** (command + storage) and the anchor→first-event
   feasibility consult (ADR-0014's core feature) — this also unblocks the
   owed worlds-with-anchors privacy fixtures (slice-2 Results item 2).

Known trap: `incremental::refresh_after` string-matches command kinds —
any new command touching memberships/subgroups/expectations must be added
by hand, and an anchor command must NOT be (audit it in writing, as
slice 2 did for `RemoveAttendance`).

Then, if the slice closes with room to spare, open the **Muster Alpha**
pre-commitment (06-app.md slice 3) — its ledger section is already
populated (coordinator flow, inbox+waive, Warn semantics decision,
group-scoped violation query, retraction commands, severity defaults,
person-scoped `select()` — the H4 refutation made that one
non-optional).

Gates as always (nextest, clippy -D warnings, fmt, check-seam grep
fallback — no rustup on this host — check-scope, check-xrefs). Results
refutations-first; plain-language artifact in
`plans/orrery/artifacts/`; merge `--no-ff`; update PLAN rows and tick
the ledger; rewrite this file. Conventional Commits; `Refs:` footers;
`Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>`.
