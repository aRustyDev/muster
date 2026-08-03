# Context/docs restructure — pre-committed implementation plan (CR-1 output)

*Written 2026-08-03 by CR-1. Scope contract: `00-analysis.md` (not
re-derived here). Governing decision record: **ADR-0027 (proposed)** —
the taxonomy, boundaries, and relocation decisions live there; this
plan carries the execution order, inventories, sweep method, and
acceptance criteria. **CR-2 must not start until the owner has reviewed
ADR-0027 and this plan** (the QR-2/QR-3 separation: synthesis reviewable
before the corpus is rewritten).*

## 1. Research digest (search-don't-recall; all verified 2026-08-03)

Full reports in the CR-1 session record; load-bearing findings:

| Finding | Consequence here | Source |
|---|---|---|
| `.claude/rules/**` auto-loads recursively at launch; a rule with `paths:` frontmatter loads only when Claude reads matching files (v2.1.198+) | Rules 04/05 get `paths:`; Rule 10 scoped to docs/plans; no `@`-imports for rules | code.claude.com/docs/en/memory.md |
| `@`-imports are eager at launch, max depth 4; they would defeat `paths:` scoping | Deviation from owner sketch recorded in ADR-0027 §5 | memory.md |
| CLAUDE.local.md fully supported, loads alongside CLAUDE.md, NOT auto-gitignored | `**/CLAUDE.local.md` gitignore entry lands in CR-1 close | memory.md |
| AGENTS.md is not natively read by Claude Code (import or symlink required) | Keep read-on-demand by reference (Rule 07 design); do not eager-import | memory.md |
| SessionStart hook: `source` ∈ {startup, resume, clear, **compact**, fork}; matcher can select `compact`; `additionalContext` output supported | W6's mechanical trigger | hooks.md |
| PreCompact (input `trigger: manual\|auto`) and PostCompact **cannot** inject context | rejected as W6 trigger | hooks.md |
| No CLI command isolates CLAUDE.md+rules token cost; `/context` lists loaded files without per-file tokens | budget criterion measured in **lines** via script (`wc -l` set), not tokens | costs.md, context-window.md |
| Skills: `.claude/skills/<name>/SKILL.md`; description+`when_to_use` ≤1,536 chars always in context, body (<500 lines) on invoke, supporting files on demand; `allowed-tools`, `context: fork`, `paths`, hooks per-skill; scripts under `scripts/` pre-authorizable | CR-3 skill authoring spec | skills.md |
| Slash commands (`.claude/commands/`) legacy-but-supported; converge on skills | CR-3: no new commands | skills.md |
| Agents: frontmatter incl. `tools`, `disallowedTools`, `model` (default `inherit`), `permissionMode`, `skills` (preload), `memory`, `maxTurns`; descriptions with "use proactively"/trigger phrases drive auto-delegation | CR-3 roster review spec | sub-agents.md |
| Skills vs agents (official): skill = procedure/reference in main context; agent = separate context + tool sandbox + own model | ADR-0027 §8 boundary | sub-agents.md |
| mdBook 0.5.0 broke the 0.4 preprocessor protocol; all auto-summary preprocessors are 0.4-era (dead/archived); repo runs mdBook 0.5.4 | SUMMARY generation must be a repo script, not a preprocessor — §6 | rust-lang/mdBook CHANGELOG, PR #2813; crates.io/GitHub per-tool, §6 |

## 2. Decisions (details and rationale in ADR-0027)

| Tension | Disposition |
|---|---|
| T1 budget | Native rules auto-load; `paths:`-scope Rules 04/05 (crates/Rust) and new Rule 10 (docs/plans); measured-lines criterion (§8.3) |
| T2 ADR location | Option (a): keep global numbers, `git mv` into `dev/adrs/<topic>/`, tooling recurses |
| T3 boundaries | Four classes defined (ADR-0027 §2 + draft Rule 10 table); one-home test: most-binding class wins, others link |
| T4 scaffolding | Namespace contract; create-on-first-content; `user/`/`admin/` reserved-empty |
| T5 inventory | §4 below; specs/PRDs OUT; rule-content decomposition DEFERRED to CR-4 (recorded in ADR-0027 §4) |
| T6 SUMMARY | §6 below |
| T7 CLAUDE.local.md | Supported; gitignore `**/CLAUDE.local.md` now (CR-1 close) |
| T8 skill-vs-agent | ADR-0027 §8; "named output artefact" bar preserved; per-item dispositions at CR-3 |
| T9 history | `git mv` everywhere; historical docs keep old inline paths; plans/README carries the map |
| T10 governance | ADR-0027 written first, `proposed`; CR-2 blocked on owner review |

**Owner queue (decide at ADR-0027 review, before CR-2):**
1. ADR-0027 overall — especially the **@-import deviation** (§5).
   ~~The PLAN.md/ROADMAP.md move~~ — **resolved 2026-08-03**: they stay
   in plans/ (see §4 amendment).
2. The CR-4 deferral of rule-content decomposition (Rule 05 span tables
   etc.) — accept, or pull into CR-2 (doubles W3 blast radius; not
   recommended).
3. Topic assignments in §3 that are judgment calls: 0020→datastore,
   0026→testing, 0001/0027→project.

## 3. ADR relocation mapping (pre-committed; CR-2 executes verbatim)

Target root: `docs/src/dev/adrs/`. Topics (open vocabulary, ADR-0027 §1):

| Topic | ADRs |
|---|---|
| `domain-model/` | 0002 0003 0004 0005 0009 0010 0011 0012 0014 0017 0018 0024 |
| `travel/` | 0006 0007 0008 |
| `datastore/` | 0015 0020 0021 |
| `architecture/` | 0013 0016 0019 0023 0025 |
| `dependencies/` | 0022 |
| `testing/` | 0026 |
| `project/` | 0001 0027 |

`adrs/README.md` → `dev/adrs/README.md` (gains the topic table).
Filenames unchanged (`NNNN-kebab-title.md`). 12+3+3+5+1+1+2 = 27 ✓.

## 4. Migration inventory (T5, final)

| Doc | From `.claude/plans/` | To `docs/src/` | Inbound refs measured 2026-08-03 |
|---|---|---|---|
| TESTING-STRATEGY.md | ×4 decomposition (pilot, §5) | `dev/strategies/testing/`, `dev/policies/testing/`, `dev/policies/benchmarking/`, `dev/patterns/testing/` | 38 lines / 16 files |
| GLOSSARY.md | move | `dev/glossary.md` | 1 file |

*(Amended 2026-08-03 at CR-1 review — owner ruling, Rule 01.2: the
original inventory also moved PLAN.md and ROADMAP.md to
`dev/roadmaps/`. **They stay in `.claude/plans/**`** — they are
plan-specific documentation, not persistent reference.
`docs/src/dev/roadmaps/` is reserved for **cross-plan summarizing
roadmaps** — documents that keep independently-progressing plans aware
of shared features and dependencies — created on first need; nothing
moves there in CR-2.)*

Plus the ADR corpus per §3 (`docs/src/adrs/*` path refs: 9 md files,
`check-xrefs.sh:20`, `docs/justfile:19`, `.claude/agents/adr-author.md`).

**Stays in plans/**: phase docs, quality-review records, artifacts,
CARRY-FORWARD, NEXT-SESSION + kickoffs, PHASE-TEMPLATE, PRDs, specs,
questions, research, per-product PLAN/ROADMAP derivations (their
root-link targets update). `plans/README.md` stays and gains the
**old→new map** (T9) plus updated derivation-rule pointers. On-move
hygiene: path-qualify formerly-relative references inside moved docs
(e.g. PLAN.md's `phases/00-grounding.md` → `.claude/plans/orrery/phases/00-grounding.md`),
as backticked text, never mdbook links out of the book tree.

## 5. Pilot decomposition — TESTING-STRATEGY.md (execute before all other moves)

| Section (current) | Target | Class rationale |
|---|---|---|
| Taxonomy (C/P/S/I dimensions, S5+I5 definitions) | `dev/strategies/testing/coverage-taxonomy.md` | what we track and why — direction, evolves |
| Tool roster + open RR&P table | `dev/strategies/testing/tool-roster.md` | evolving picks; adoptions stay in ADR-0026 (links) |
| Measurement-variance policy (W-2, 7 clauses) | `dev/policies/benchmarking/measurement-variance.md` | MUSTs with gates |
| Property naming + case budgets; regression policy (W-14) | `dev/policies/testing/property-and-regression.md` | MUSTs |
| Test doubles placement | `dev/patterns/testing/test-doubles.md` | how-to with rationale examples |
| Standing policies (doctests, safe Rust, egress, funnel discipline, gate honesty) | `dev/policies/testing/standing-policies.md` | MUSTs with doors |
| Reading list | stays with tool-roster (appendix) | reference |

Each target carries a provenance header naming TESTING-STRATEGY.md and
the QR docs (Rule 01.5). The 16 referencing files are swept in the same
commit; `plans/TESTING-STRATEGY.md` is deleted (not stubbed) — the
plans/README map is the single tombstone. **Gate: the pilot lands as
one commit, `just audit` + `docs::check-links` green, before any other
move begins.** If the pilot reveals the 4-way split doesn't cut
cleanly, STOP and take findings back to the owner (falsifiable-pilot
discipline, Rule 01).

## 6. SUMMARY.md strategy (T6)

*(Researched 2026-08-03 against live crates.io/GitHub state.)*
Decision: **generate SUMMARY.md's dev sections with a repo Python
script** (`docs/scripts/gen-summary.py`, python3 is already a justfile
prerequisite; door `just docs::summary`): hand-authored prefix (README,
part headers) above a `<!-- generated -->` marker, then a deterministic
sorted walk of `docs/src/dev/**` taking each chapter title from the
file's first `# ` heading — which also fixes the current SUMMARY's
0001–0022 garbage titles. Currency gate inside `docs::check-links`:
regenerate and `git diff --exit-code docs/src/SUMMARY.md` before
`mdbook build`/`mdbook test`.

Evidence for rejecting the alternatives: this repo runs **mdBook
v0.5.4**, and mdBook 0.5.0's breaking release changed the preprocessor
wire protocol (`Book.sections` → `items`, rust-lang/mdBook PR #2813);
every auto-summary preprocessor is 0.4-era — mdbook-autosummary
(archived 2026-03-28, pins mdbook 0.4.45), mdbook-fs-summary (dormant
2022, virtual-book approach leaves the on-disk SUMMARY permanently
wrong), mdbook-auto-gen-summary (dead 2021). The only live tool,
book-summary (standalone CLI, repo pushed 2026-03), has a 3.5-year-stale
release and weak ordering control. mdBook has no native auto-summary and
the request (issue #2466) is unowned. Direct prior art for the script:
rust-lang/rfcs `generate-book.py`.

## 7. CR-2 execution order (pre-committed)

Branch `feat/cr2-docs-restructure`, merged `--no-ff` when §8 is green
(structural change at implementation-phase blast radius; Rule 08).

1. **Preflight**: create `.claude/scripts/context-cost.sh` (the
   line-count measurement, ledger row CR-1); record before-numbers.
2. **Tooling first, proven on the flat tree**: `adr-next` → recursive
   (`find src/adrs src/dev/adrs -name '[0-9]*.md'` union);
   `check-xrefs.sh:20` → recursive glob over both roots;
   `gen-summary.py` + `just docs::summary` + currency check. Commit;
   gates green (tooling must tolerate both regimes during migration).
3. **ADR relocation** per §3 (`git mv`), regenerate SUMMARY (fixes the
   0001–0022 garbage titles), sweep the 9+1 md files + adr-author agent
   + `docs/src/README.md`. Commit; gates green.
4. **Pilot** per §5. Commit; gates green; STOP-check.
5. **Remaining moves** per §4 + plans/README map + derivation-rule
   update. Commit; gates green.
6. **W1**: fill the six TODO markers in `.claude/CLAUDE.md` (draft text
   §9; keep total addition ≤~30 lines), update its read-order list for
   new paths.
7. **Rules**: land Rule 10 from `draft-rule-10-docs-structure.md`;
   amend Rule 02 + Rule 07 + rules README (amendment list at the
   draft's foot); add `paths:` frontmatter to Rules 04/05.
8. **W6 wiring**: `.claude/scripts/compact-self-review.sh` emitting
   `{"hookSpecificOutput":{"hookEventName":"SessionStart","additionalContext":
   "Compaction just occurred. Run the ADR-0027 self-review: scan retained
   context against the indicator classes and append dated rows to
   .claude/observations.md before resuming."}}`; register under
   `hooks.SessionStart` matcher `compact` in `.claude/settings.json`;
   add the harvest step to the close-out protocol text. Verify with a
   manual `/compact` and record the observation.
9. **After-measurement** (`context-cost.sh`) recorded next to the
   before-numbers; close-out per standing protocol (commit, memory,
   kickoff rewrite for CR-3).

## 8. Acceptance criteria (pre-committed; supersedes the 00-analysis sketch)

1. `just audit`, `just docs::build`, `just docs::check-links` green
   after **every** landing commit; zero dangling `ADR-NNNN`.
2. Sweep check: `grep -rn 'docs/src/adrs/' --include='*.md'` (and
   `plans/TESTING-STRATEGY\|plans/GLOSSARY`) returns hits only in
   historical records (phase docs, quality-review, artifacts) and the
   plans/README map — zero in living docs, rules, agents, tooling.
3. Context budget: at-launch unconditional auto-load ≤ **404 lines**
   (2026-08-03 baseline: rules 345 + `.claude/CLAUDE.md` 47 + root 12);
   expected ≈350 after scoping 04/05 (−97) and W1 additions (≤+30) +
   README pointer lines. Any excess over baseline needs a written
   reason in the CR-2 close note. *(Token cross-check, measured
   2026-08-03 via `/context`: project auto-load ≈ **7.6k tokens** —
   root CLAUDE.md 283 + `.claude/CLAUDE.md` 677 + rules ≈6.6k; Rules
   04+05 ≈ 1.8k tokens are what `paths:` scoping removes from non-Rust
   sessions.)*
4. Rule 02 held: ADR-0027 precedes implementation; numbering global,
   sequential; no accepted ADR content-edited (moves ≠ edits).
5. All six TODO markers in `.claude/CLAUDE.md` resolved — filled or
   explicitly deferred with a dated line.
6. `**/CLAUDE.local.md` gitignored before any such file exists
   *(landed with CR-1)*.
7. W6 live: taxonomy written (ADR-0027 §7), ledger exists with real
   rows *(landed with CR-1 — 8 rows)*, graduation threshold stated,
   SessionStart-compact hook registered and observed firing once
   (manual `/compact` test documented in the CR-2 close note).
8. Every move used `git mv`; plans/README old→new map complete;
   historical docs unedited except dated corrections where a reader
   would actually be misled.
9. SUMMARY: generated sections regenerate deterministically; currency
   check part of `docs::check-links`; 0001–0022 titles are real titles.
10. Skills/agents instructions (CR-3) cite the guidance version/date
    they were checked against; until CR-3, no new skills/agents/commands.

## 9. W1 draft text (lands at the TODO markers, budget ≤~30 lines)

* **What context files carry**: constraints and orientation only;
  durable knowledge goes to `docs/src/` per Rule 10. If a session
  states a binding convention, it lands in a rule or docs/src *before
  the session ends* (else it's a ledger row).
* **Update triggers**: slice close (protocol step) · a refutation or
  correction lands · an owner decision changes a constraint.
* **Review triggers**: every compaction (hook-injected) and slice close
  — scan loaded instructions against repo state; a stale instance is
  fixed now or becomes a ledger row (the F-13 fix).
* **Where**: decision→ADR · enforcement→policy (artifact gate) or rule
  (agent behavior) · goal→strategy · recipe→pattern · orientation→
  AGENTS.md · personal→CLAUDE.local.md (never committed).
* **Skills/agents/scripts/hooks**: built from ledger evidence (rule of
  three), never from taste; boundary per ADR-0027 §8; authoring
  specifics per the CR-3 instructions.

## 10. CR-3 outline (separable; can lag)

1. `.claude/scripts/` bootstrap: `context-cost.sh` (CR-2 preflight),
   candidates from the ledger as they graduate.
2. First skills — build only what the ledger supports; nearest to
   graduation: `slice-close` (2 rows), `dated-amendment`,
   `research-brief`. Author per skills.md (description ≤1,536 chars
   with triggers, body <500 lines, supporting files on demand).
3. Agent roster review against sub-agents.md (2026-08-03 semantics):
   per-agent — description with proactive triggers? `model`/`tools`
   scoping? instruction drift (ledger row CR-1: adr-author
   hand-maintains SUMMARY — stale after CR-2)? skill-vs-agent
   disposition per ADR-0027 §8 (adr-author and phase-scribe are
   skill-shaped: they need conversation context; adversarial-reviewer,
   datastore-screener, benchmark-runner are agent-shaped: bulk
   output, separate context).
4. W4/W5 when-to-create instructions land at the CLAUDE.md markers
   (kept ≤ budget), citing guidance version/date.

## 11. Risks

* **Both-regimes window** during CR-2 steps 2–5: tooling recurses over
  both roots until the last move; gates green at every commit is the
  control.
* **mdbook link breakage**: moved docs must not carry md-links to
  files outside `docs/src` (backticked paths only); `docs::build`
  catches the rest.
* **Path-scoped rule absence** (ADR-0027 disliked-consequence): rules
  README pointer lines are the mitigation; watch the ledger for
  correction-pattern rows implicating a scoped rule.
* **Scope creep** (the named failure mode): specs/PRDs stay put; rule
  decomposition stays CR-4; anything else new goes to the owner queue,
  not into CR-2.
