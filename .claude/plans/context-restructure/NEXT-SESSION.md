# Next-session kickoff — context/docs restructure (CR-1)

*Written 2026-08-03 in the session that produced `00-analysis.md`. Paste
the prompt below into a fresh session in this repo. Rewrite this file at
each CR slice close (the standing close-out protocol); delete it when
the restructure lands.*

---

You are starting the Orrery/Muster **context/docs restructure**. Your
project memory has `context-restructure-state` — trust it for
orientation; the repo is the state of truth. Read, in order:

1. `.claude/plans/context-restructure/00-analysis.md` — the first-draft
   analysis: the owner's target tree (verbatim), the inventory of what
   exists, five workstreams (W1–W5), ten tensions (T1–T10), phasing,
   and the acceptance-criteria sketch. **This is your scope contract;
   do not re-derive it.**
2. `.claude/rules/02-decision-records.md` and
   `.claude/rules/07-context-files.md` — the two rules this goal amends
   (they are also already in your loaded context).
3. `.claude/plans/README.md` + `.claude/plans/TESTING-STRATEGY.md` —
   the pilot decomposition target (T3).
4. The owner's TODO markers at the bottom of `.claude/CLAUDE.md` —
   the landing sites for W1's meta-instructions.

This session: **CR-1 — research, decisions, pre-committed plan.**

* **Research first, search-don't-recall** (delegate to the
  claude-code-guide agent / current docs; cite versions and dates):
  (a) actual loading semantics of `.claude/rules/**`, `@`-references,
  and CLAUDE.local.md in current Claude Code — T1/T7 turn on this;
  (b) current best practices for SKILLS (progressive disclosure,
  frontmatter, scripts, schemas) and CUSTOM AGENTS (description
  optimization, model/tool/permission definition) — W4/W5;
  (c) mdbook SUMMARY automation options — T6.
* **Decide** T2 (ADR relocation: leaning = keep global numbers, nest by
  topic, update tooling), T3 (the four boundary definitions), T5 (the
  migration inventory; scope creep is the failure mode — specs/PRDs
  are OUT unless the owner says otherwise). Queue anything genuinely
  owner-shaped rather than guessing.
* **Write**: the governing ADR (next free number — verify via
  `just docs::adr-next`; status `proposed` until the owner reviews),
  the draft W2 structural rule(s), and the pre-committed implementation
  plan `context-restructure/01-plan.md` with acceptance criteria
  (extend the sketch in 00-analysis §criteria — the context-budget
  before/after measurement is non-negotiable).

**Present the plan to the owner before executing CR-2** (the QR-2/QR-3
separation: synthesis reviewable before the corpus is rewritten). If
the owner approves in-session and context budget allows, proceed to
CR-2 (migration) in this session; otherwise close and rewrite this
kickoff for CR-2.

Gates: `just audit`, `just docs::build` green on anything you commit.
Close per the standing protocol: commit, update
`context-restructure-state` memory, rewrite this file. Conventional
Commits; `Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>`.
