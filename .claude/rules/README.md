# Rules — index and what remains

Rules are **binding constraints on how you work**, distinct from `AGENTS.md`
which describes *what the codebase is*. See Rule 07 for the split.

| # | Rule | Supplied |
|---|---|:--:|
| 00 | Non-negotiables | yes |
| 01 | Evidence standards | yes |
| 02 | Decision records | yes |
| 03 | Scope boundaries | yes |
| 04 | Rust conventions | yes |
| 05 | Observability | yes |
| 06 | Dependencies | yes |
| 07 | Context files (CLAUDE.md vs AGENTS.md) | yes |
| 08 | Git and commits | yes *(written Phase 2)* |
| 09 | Security and secrets | yes *(written Phase 2)* |
| 10 | Documentation structure | yes *(written by ADR-0027, 2026-08-03)* |

## Path-scoped rules (ADR-0027)

These load only when a matching file is read — the constraint exists
even when its text is not in context: **Rule 04** (`crates/**`, `*.rs`,
`Cargo.toml`) · **Rule 05** (`crates/**`) · **Rule 10** (`docs/**`,
`.claude/**`, `*.md`).

## Keep rules short

A rule file over ~60 lines gets skimmed. If a rule needs more explanation than
that, the explanation belongs in an ADR and the rule links to it.
