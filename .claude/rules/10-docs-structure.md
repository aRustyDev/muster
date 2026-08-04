---
paths:
  - "docs/**"
  - ".claude/**"
  - "*.md"
---

# Rule 10 — Documentation structure (ADR-0027)

## Where a durable fact lives (one home, Rule 07)

| The statement is… | It lives in | Class rules |
|---|---|---|
| why we chose, among alternatives | `docs/src/dev/adrs/<topic>/NNNN-*.md` | MADR; immutable once accepted; supersede, never edit (Rule 02) |
| what we're trying to achieve, in what order | `docs/src/dev/strategies/<topic>/` | evolves by dated amendment |
| what is enforced (MUST/SHOULD + its gate) | `docs/src/dev/policies/<topic>/` | names the gate command; executable where possible |
| how to do X here, with an example | `docs/src/dev/patterns/<topic>/` | advisory, never binding |
| cross-plan roadmap summaries (shared features, dependencies between plans) | `docs/src/dev/roadmaps/` | living; plan-specific PLAN/ROADMAP stay in `.claude/plans/**` |
| a defined term | `docs/src/dev/glossary.md` | — |

If a statement fits two classes it lives in the **more binding** one and
the other links. Working documents (phase docs, specs, PRDs, questions,
research, session machinery) stay in `.claude/plans/**` — they record
work, not reference.

## Topic taxonomy

One open vocabulary shared by adrs/strategies/policies/patterns
(current table: `docs/src/dev/adrs/README.md`). **Directories are
created on first real document** — a stub tree reads as coverage and is
a lie (namespace contract, ADR-0027).

## ADR mechanics

Numbering stays global, sequential, never reused (Rule 02 — only the
*location* changed). `just docs::adr-next` gives the number regardless
of topic. Never hand-edit SUMMARY.md's generated section — run
`just docs::summary`; `just docs::check-links` fails on a stale index.
Tag every code fence (```text etc.) — mdbook test compiles untagged
fences as Rust.

## Rules-directory conventions (`.claude/rules/**`)

* Rules are **narrow and sharp**: binding constraints, not explanations.
  A rule needing >~60 lines decomposes into a `docs/src/dev/policies/`
  page plus a pointer here.
* File-type-specific rules carry `paths:` frontmatter so they load only
  when relevant files are read; universal rules stay unconditional.
  **Every scoped rule leaves a one-line pointer in `rules/README.md`**
  (unconditional) so the constraint's existence is never invisible.
* Never `@`-import a rule from CLAUDE.md — imports are eager and defeat
  `paths:` scoping. `.claude/rules/**` auto-loads natively (recursive).
