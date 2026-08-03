# 27. Documentation architecture: docs/src taxonomy, ADR relocation, and context-loading strategy

* Status: proposed *(owner review required before CR-2 implements it)*
* Date: 2026-08-03
* Source: context-restructure CR-1
  (`plans/context-restructure/00-analysis.md` — owner target structure of
  2026-08-03; harness semantics verified against current Claude Code docs
  the same day, citations inline)

## Context and Problem Statement

The owner directed a restructure of how documentation and agent context
work: `docs/src/**` becomes the central reference for persistent
decisions, strategies, policies, patterns, and roadmaps; context files
(CLAUDE.md, rules, AGENTS.md) get a written lifecycle; skills, custom
agents, and pre-authorized scripts get a creation strategy; and a
compaction-time self-review turns "what should we build" into an
evidence question (an indicator ledger) rather than a taste question.

Today: `docs/src/adrs/` is flat (0001–0026) and is the only docs/src
content; durable policy content lives in `.claude/plans/`
(TESTING-STRATEGY.md, GLOSSARY.md, PLAN.md, ROADMAP.md); all ten rules
auto-load into every session — the auto-loaded corpus measured **404
lines on 2026-08-03** (rules 345 incl. README, `.claude/CLAUDE.md` 47,
root CLAUDE.md 12); `docs/justfile adr-next` and
`docs/scripts/check-xrefs.sh` assume flat ADR paths; SUMMARY.md is
hand-maintained and its 0001–0022 entries carry import-comment garbage
as chapter titles; `CLAUDE.local.md` is gitignored nowhere; and the
quality review's F-13 showed stale context survives because **no
refresh trigger exists**.

Rule 02 fixes the ADR location and requires an ADR to change it; Rule 07
fixes the context-file split. This ADR is that change record for both.

## Decision Drivers

* Every-session context budget (Rule 07): standing text is a tax paid
  each turn; the owner's target implies *more* standing instruction.
* One fact, one home (Rule 07): a 4-way doc split multiplies candidate
  homes and therefore drift risk.
* Never rewrite history (Rule 08); ADR numbers are never reused
  (Rule 02).
* F-13: review/refresh triggers must be mechanical where the harness
  allows, discipline-based only as fallback.
* Skills/agents/hooks get built on accumulated evidence, not taste.
* MVP ethos: no empty scaffolding that reads as coverage (F-15 inverted).
* Verified harness semantics (code.claude.com/docs `memory.md`,
  `hooks.md`, `skills.md`, `sub-agents.md`, all read 2026-08-03,
  Claude Code ~v2.1.219): `.claude/rules/**` auto-loads recursively at
  launch; a rule with `paths:` frontmatter loads only when matching
  files are read (v2.1.198+); `@`-imports are eager, max 4 hops;
  CLAUDE.local.md is supported and NOT auto-gitignored; AGENTS.md is
  not natively read; SessionStart hooks can filter on
  `source: compact` and inject `additionalContext`; PreCompact and
  PostCompact hooks **cannot** inject context.

## Considered Options

1. **ADR relocation** — (a) keep global sequential numbers, nest files
   under topic directories, update tooling; (b) freeze 0001–0026 flat,
   nest only new ADRs (two regimes forever); (c) renumber per topic
   (breaks every `ADR-NNNN` reference and Rule 02's never-reuse —
   rejected outright).
2. **Document classes** — the owner's 4-way split
   (ADR/strategy/policy/pattern) · status quo (ADRs plus ad-hoc plans
   docs) · a 2-way split (ADR + everything-else guide).
3. **Rules loading mechanism** — `@`-import rules from CLAUDE.md (the
   target sketch's mechanism) · native auto-load, all unconditional
   (status quo) · native auto-load with `paths:`-scoped loading for
   file-type-specific rules.
4. **SUMMARY.md maintenance** — by hand (status quo) · an mdbook
   auto-summary preprocessor · a repo script that generates the ADR/dev
   sections deterministically, with a CI currency check.

## Decision Outcome

1. **Taxonomy.** `docs/src/dev/{adrs,strategies,policies,patterns,roadmaps}/`
   with one shared, open topic vocabulary across the first four
   (testing, benchmarking, telemetry, security, … — plus
   domain/architecture topics for the existing decision corpus; the
   owner's list is exemplary, not exhaustive). The tree is a **namespace
   contract**: directories are created on first real document, never as
   stubs. `docs/src/user/` and `docs/src/admin/` are reserved, empty.
2. **Class boundaries** (the one-home test; if a statement fits two
   classes, it lives in the more binding one and the other links):
   * **ADR** — why we chose: dated choice among alternatives, with
     drivers and consequences. Immutable once accepted; superseded,
     never edited (Rule 02 unchanged).
   * **Strategy** — what we are trying to achieve, in what order;
     evolves by dated amendment.
   * **Policy** — what is enforced: MUST/SHOULD plus its gate;
     executable where possible (the gate command is part of the policy).
   * **Pattern** — how to do X here: recipe with a worked example;
     advisory, never binding.
3. **ADR relocation: option (a).** Global sequential numbering is
   permanent; the 26 existing ADRs `git mv` into
   `docs/src/dev/adrs/<topic>/` per a mapping table pre-committed in
   `01-plan.md`; `adr-next` and `check-xrefs.sh` switch to recursive
   globs; SUMMARY.md is regenerated (which also fixes the 0001–0022
   garbage titles); `adrs/README.md` moves with the corpus.
4. **The plans/docs boundary.** `docs/src/` holds durable reference
   (decisions, strategies, policies, patterns, roadmaps, glossary);
   `.claude/plans/` holds the working corpus (phase docs, specs, PRDs,
   questions, research, and session machinery: CARRY-FORWARD,
   NEXT-SESSION, kickoffs, quality-review records). Migration set, in
   full: TESTING-STRATEGY.md (pilot ×4 decomposition) and GLOSSARY.md.
   PLAN.md and ROADMAP.md **stay in plans/** *(owner ruling 2026-08-03,
   amending this ADR's first draft)*: they are plan-specific
   documentation, not persistent reference. `dev/roadmaps/` is reserved
   for **cross-plan summarizing roadmaps** — documents that keep
   independently-progressing plans aware of shared features and
   dependencies — created on first need. Specs and PRDs are explicitly
   out of scope.
   Decomposing rule *content* (e.g. Rule 05's span tables) into
   `policies/telemetry/` is **deferred** to a follow-up slice —
   recorded so the "narrow and sharp" end-state stays owed.
5. **Rules loading.** Rely on the native recursive auto-load; do NOT
   `@`-import rules from CLAUDE.md — imports are eager and would defeat
   `paths:` scoping (this deviates from the owner's target sketch,
   which predates verification; the intent — rules always available —
   is met natively). Code-facing rules (04 Rust conventions, 05
   observability) gain `paths:` frontmatter scoped to `crates/**` and
   Rust/manifest files; the new structural rule (Rule 10) is scoped to
   `docs/**` + `.claude/**`. Before/after auto-load cost is measured
   against the 404-line baseline; an increase needs a written reason.
6. **CLAUDE.local.md** is the supported personal-context mechanism;
   `**/CLAUDE.local.md` enters `.gitignore` before any such file
   exists.
7. **Compaction-time self-review (W6).** Ledger at
   `.claude/observations.md` (auto-loads nowhere): append-only rows
   `date · session · indicator class · observation · candidate`;
   a candidate **graduates at three independent rows** (rule of three)
   or is retired with a dated line. Triggers: a **SessionStart hook
   with the `compact` matcher** injects the self-review instruction
   mechanically after every compaction (`additionalContext` — verified
   supported); slice close-out keeps the deliberate harvest step.
   PreCompact/PostCompact were considered and rejected: neither can
   inject context.
8. **Skills vs agents boundary** (for CR-3 and all future creation
   decisions): a **skill** packages a repeatable procedure or reference
   for the main context (close-out protocol, dated-amendment mechanics,
   measurement-run protocol); an **agent** buys a separate context
   window and tool sandbox (bulk output, restricted tools, different
   model). The existing bar is preserved verbatim: *an agent without a
   named output artefact is a chat, not an agent.* The six existing
   agents are reviewed against current guidance at CR-3, not before.
9. **SUMMARY.md maintenance** — see `01-plan.md` §SUMMARY for the
   evaluated options and pick *(filled from live research in this
   slice; the pick must be deterministic and CI-checkable)*.

### Consequences

* Every `docs/src/adrs/` path in the corpus changes once; `git mv`
  preserves blame, `01-plan.md` pre-commits the sweep method, and
  `plans/README.md` carries the old→new map.
* Sessions that never touch Rust files stop paying for Rules 04/05
  (~97 lines); docs-touching sessions pay for Rule 10 instead.
* Two mechanical triggers (SessionStart-compact hook, gitignore) replace
  two discipline-only promises — the F-13 class shrinks.
* The consequences we dislike, recorded (ADR-0020 discipline):
  * The 4-way split adds a classification decision to every new durable
    fact and creates a boundary-dispute surface Rule 07's two-way split
    did not have; the one-home test is a judgment call, not a gate.
  * Path-scoped rules trade *guaranteed* presence for
    presence-by-path-correlation: a session editing only a plan document
    that promises telemetry spans will not have Rule 05 loaded. The
    rules README (unconditional) must carry one pointer line per scoped
    rule so the existence of the constraint is never invisible.
  * Historical documents (phase docs, quality-review records) keep
    their old inline paths forever — Rule 02 forbids rewriting them —
    so readers of history will follow paths that no longer resolve and
    must rely on the plans/README map.
  * The indicator ledger is itself a discipline artifact: the hook makes
    the *reminder* mechanical, but honest rows still depend on the
    session writing them.
