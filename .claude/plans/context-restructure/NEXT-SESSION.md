# Next-session kickoff — context/docs restructure (CR-2)

*Rewritten 2026-08-03 at CR-1 close (standing protocol). Delete this
file when the restructure lands.*

---

**GATE: CR-2 is blocked on owner review.** ADR-0027 (`docs/src/adrs/
0027-docs-and-context-architecture.md`, status `proposed`) and
`01-plan.md` §2's owner queue must be reviewed first — the @-import
deviation, the CR-4 deferral, and the ADR topic assignments (the
PLAN/ROADMAP question was resolved 2026-08-03: they stay in plans/;
`dev/roadmaps/` is for cross-plan summaries only). If the owner has
not approved, stop and ask.

You are executing **CR-2 — migrate + land** of the context/docs
restructure. Project memory `context-restructure-state` orients you;
the repo is the state of truth. Read, in order:

1. `.claude/plans/context-restructure/01-plan.md` — **the pre-committed
   plan; execute §7 verbatim, acceptance criteria §8.** Do not re-derive
   decisions; they are in ADR-0027.
2. `.claude/plans/context-restructure/draft-rule-10-docs-structure.md`
   — lands as `.claude/rules/10-docs-structure.md` in §7 step 7, with
   the amendments listed at its foot.
3. `.claude/observations.md` — the W6 ledger (live; harvest into it at
   close).

Mechanics that will bite:

* Branch `feat/cr2-docs-restructure`; merge `--no-ff` when §8 is green.
* Gates after every commit — verify by running them, never by trusting
  a prior session's claim: `just audit && just docs::build && just
  docs::check-links` (check-links was red until CR-1 fixed six fence
  tags; it is now a load-bearing criterion).
* Tooling updates land and prove green on the FLAT tree before any
  `git mv` (§7 step 2); the pilot decomposition (§5) has a STOP-check.
* `mdbook test` compiles untagged fences as Rust — tag every fence in
  every migrated/created doc.
* Measure after-cost with `.claude/scripts/context-cost.sh` (create in
  preflight) against the 404-line baseline (§8.3).

Close per the standing protocol: commit, update
`context-restructure-state` memory, rewrite this file for CR-3 (skills
+ agents + scripts — outline in `01-plan.md` §10). Conventional
Commits; `Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>`.
