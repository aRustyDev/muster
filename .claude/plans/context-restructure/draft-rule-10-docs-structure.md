# DRAFT — Rule 10 — Documentation structure (lands as `.claude/rules/10-docs-structure.md` at CR-2)

*Drafted 2026-08-03 by CR-1 under ADR-0027 (proposed). Not binding until
ADR-0027 is accepted and CR-2 moves this file into `.claude/rules/`.
The `paths:` frontmatter below activates then; it is quoted here as
content. CR-2 also applies the two amendments listed at the bottom.*

---

```yaml
---
paths:
  - "docs/**"
  - ".claude/**"
  - "*.md"
---
```

# Rule 10 — Documentation structure

## Where a durable fact lives (one home, Rule 07)

| The statement is… | It lives in | Class rules |
|---|---|---|
| why we chose, among alternatives | `docs/src/dev/adrs/<topic>/NNNN-*.md` | MADR; immutable once accepted; supersede, never edit (Rule 02) |
| what we're trying to achieve, in what order | `docs/src/dev/strategies/<topic>/` | evolves by dated amendment |
| what is enforced (MUST/SHOULD + its gate) | `docs/src/dev/policies/<topic>/` | names the gate command; executable where possible |
| how to do X here, with an example | `docs/src/dev/patterns/<topic>/` | advisory, never binding |
| phases, stage gates, sequencing | `docs/src/dev/roadmaps/` | living; per-product derivations stay in plans/ |
| a defined term | `docs/src/dev/glossary.md` | — |

If a statement fits two classes it lives in the **more binding** one and
the other links. Working documents (phase docs, specs, PRDs, questions,
research, session machinery) stay in `.claude/plans/**` — they record
work, not reference.

## Topic taxonomy

One open vocabulary shared by adrs/strategies/policies/patterns
(testing, benchmarking, telemetry, security, domain-model,
architecture, …). **Directories are created on first real document** —
a stub tree reads as coverage and is a lie (namespace contract,
ADR-0027).

## ADR mechanics

Numbering stays global, sequential, never reused (Rule 02 — only the
*location* changed). `just docs::adr-next` gives the number regardless
of topic. Do not hand-edit generated SUMMARY sections; regenerate
(`just docs::summary` after CR-2).

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

---

## Amendments CR-2 applies alongside this rule

* **Rule 02**: location line becomes
  `docs/src/dev/adrs/<topic>/NNNN-kebab-title.md`, topic per Rule 10;
  everything else unchanged.
* **Rule 07**: placement diagram gains `docs/src/dev/**` split and
  `.claude/skills|scripts/`; the AGENTS.md/CLAUDE.md split and budgets
  are unchanged.
* **rules/README.md**: index row for Rule 10; pointer lines for scoped
  Rules 04/05.
