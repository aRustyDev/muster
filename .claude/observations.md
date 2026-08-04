# Indicator ledger — evidence for what to build (W6, ADR-0027)

*Append-only. One row per observation, harvested at compaction (the
SessionStart-compact hook injects the reminder from CR-2 onward) and at
every slice close. **A candidate graduates when three independent rows
argue for it** (rule of three) — graduation means building it in the
next appropriate slice, or retiring the candidate with a dated line
explaining why not. Never edit or delete a row; corrections are new
rows. Indicator taxonomy: ADR-0027 / `context-restructure/00-analysis.md`.*

Classes: repeated procedure → skill · context pollution → agent ·
repeated delegation shape → agent/skill · correction pattern → rule ·
convention stated, not recorded → rule/docs · "whenever X" by memory →
hook · permission friction → hook/allowlist/script · re-composed
pipeline → script · blocked-on-owner → policy/ADR gap.

*Amended 2026-08-03 (owner-directed): rows may also be appended by the
user-level retro agents (`retro-debrief` · `retro-lessons` ·
`retro-knowledge`) through `~/.claude/scripts/append-observation.sh` —
the single write door, kept so the storage backend can change in one
place without touching callers. Retro rows may use the extended
classes: problem→solution · lesson · advice to self · entity ·
relation · effort sink.*

| Date | Session | Class | Observation | Candidate |
|---|---|---|---|---|
| 2026-08-03 | QR-3 | repeated procedure | slice close-out protocol (commit → memory update → rewrite/retire kickoff) executed manually in every QR/phase session; steps re-read from the plan doc each time | skill: `slice-close` |
| 2026-08-03 | QR-3 | repeated procedure | dated-amendment pattern (edit + dated italic note + finding ID + `just audit`) applied ~15× in one session, shape re-derived each time | skill: `dated-amendment` (or pattern doc + snippet) |
| 2026-08-03 | QR-3 | correction pattern | `git add -A` swept generated `docs/book/` into a commit; caught after the fact, fixed by amend + gitignore | hook: pre-commit check for untracked generated dirs |
| 2026-08-03 | QR-3 | "whenever X" by memory | "run `just audit` after any docs/plans edit" enforced only by discipline | hook: PostToolUse audit trigger |
| 2026-08-03 | QR-3 | convention stated, not recorded | kickoff files pinning volatile facts ("91 tests green … stop if not") go stale and halt successor sessions (R-11); fixed ad-hoc each time | rule: kickoffs name the *command* to verify state, never the expected number |
| 2026-08-03 | CR-1 | re-composed pipeline | auto-loaded-context cost measurement (`wc -l` over rules + CLAUDE.md files) composed ad-hoc; needed again at CR-2 for the before/after acceptance criterion | script: `.claude/scripts/context-cost.sh` |
| 2026-08-03 | CR-1 | repeated delegation shape | the "verify against current docs, cite URL+version+date, per-question structured report" research brief written from scratch 3× this session (and previously in ADR-0025-era sessions) | skill: `research-brief` (wraps the search-don't-recall prompt shape) |
| 2026-08-03 | CR-1 | convention stated, not recorded | adr-author agent's instructions hand-maintain SUMMARY.md; any tooling change (generated SUMMARY at CR-2) silently strands such embedded instructions | CR-3 roster review checks each agent's instructions for drift against current tooling |
| 2026-08-03 | CR-1 | "whenever X" by memory | `just docs::check-links` had been red since the 2026-08-01 ADR import (6 untagged fences compiled as Rust doctests) — the gate existed but no session ran it; found only because CR-1 pre-committed it as a criterion (F-2's class: a gate conditioned on nobody) | close-out protocol names the full gate set; RR&P-1 CI runs `docs::check-links` |
| 2026-08-03 | CR-2 | correction pattern | the `git add -A` reflex reappeared (blocked this time by the QR-3 gitignore fix; explicit-path staging used after) — 2nd row for this candidate | hook: pre-commit check for untracked generated dirs |
| 2026-08-03 | CR-2 | re-composed pipeline | `just audit && just docs::build && just docs::check-links` hand-composed ~6× as the per-commit gate loop | `just gates` (or equivalent) recipe naming the full docs gate set |
| 2026-08-03 | CR-2+ | owner directive | owner directed creation of the global retro agents (debrief/lessons/knowledge) and the append-observation.sh single write door, ahead of rule-of-three; gitkraken-hooks plugin disabled at the same time | CR-3 roster review covers the retro-* agents and the append script |
| 2026-08-04 | CR-2+ | owner directive | owner directed four project specialist agents (dioxus-engineer, library-architect-rs-crates, qa-architect-rs, test-engineer-rs), ahead of rule-of-three | CR-3 roster review covers all four alongside the original six |
