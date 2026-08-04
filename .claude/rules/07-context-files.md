# Rule 07 — CLAUDE.md vs AGENTS.md

## The split

| | `AGENTS.md` | `CLAUDE.md` |
|---|---|---|
| Answers | *What is this, how do I build and test it* | *How should I behave while doing it* |
| Audience | any coding agent, vendor-neutral | Claude specifically |
| Content | layout, commands, conventions, invariants | process, rules pointer, working style |
| Scope | **per-directory**, read on demand | **root only** |
| Changes when | the code changes | the working agreement changes |

## Placement

```text
/AGENTS.md              workspace orientation: build, test, lint, crate map
/CLAUDE.md              thin: points at AGENTS.md + .claude/rules/**
/CLAUDE.local.md        personal, gitignored, never committed
/.claude/CLAUDE.md      the working agreement itself
/.claude/rules/         binding constraints; paths:-scoped when file-type-specific (Rule 10)
/.claude/skills|scripts/ evidence-graduated procedures + pre-authorized checks (ADR-0027)
docs/src/dev/**         durable reference: adrs/strategies/policies/patterns/roadmaps (Rule 10)
crates/orrery/AGENTS.md what this crate owns, must never contain, how to test
crates/*/AGENTS.md      same, per crate
docs/AGENTS.md          mdbook build, ADR conventions, numbering
```

*(Placement amended 2026-08-03 by ADR-0027: added CLAUDE.local.md,
rules scoping, skills/scripts, and the docs/src/dev split.)*

**No per-crate `CLAUDE.md`.** Behavioural rules do not vary by directory; scope
rules do, and those live in each crate's `AGENTS.md` and in Rule 03.

## Two hard constraints

**1. Every fact lives in exactly one file.** If build commands appear in both
root `AGENTS.md` and `crates/orrery/AGENTS.md`, they will drift, and the drift
will not be noticed until an agent follows the stale one. Link instead.

**2. Root files are read every session — budget them.** Keep root `AGENTS.md`
and `CLAUDE.md` under ~80 lines each. Depth belongs in `.claude/plans/**` and
`docs/`, referenced by path. A 400-line root `AGENTS.md` is a context tax paid
on every single turn, and the tail of it will be ignored anyway.

## Skeletons

Root `AGENTS.md`: one-paragraph project statement · crate map with one line each
· `just` entrypoints · the three or four invariants worth memorising · where to
look for more.

Crate `AGENTS.md`: what this crate owns · what it must never contain (quote
Rule 03) · module map · how to run its tests and benches · its specific gotchas.

Root `CLAUDE.md`: pointer to `AGENTS.md`, pointer to `.claude/rules/**`, the one
open decision (ADR-0015), and nothing else. Resist growing it.
