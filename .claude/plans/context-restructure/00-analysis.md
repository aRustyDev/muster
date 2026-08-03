# Context/docs restructure — first-draft analysis (pre-planning)

*Written 2026-08-03 at owner direction, in the session that closed the
quality review. This is **analysis, not a plan**: it inventories what
exists, embeds the owner's target structure verbatim, reasons about the
seams and tensions, and sketches phasing + acceptance criteria for the
session that will plan and then implement the change. Nothing here edits
the corpus. The owner placed TODO markers in `.claude/CLAUDE.md`
(lines ~42–47) marking where the meta-instructions will land.*

## The high-level goal (owner statement, compressed)

Restructure how docs + context files work, at strategy level:

1. **Context-file lifecycle** — how/when/where to update, review, and add
   context files (CLAUDE.md, CLAUDE.local.md, `.claude/rules/**`,
   AGENTS.md). Instructions live in CLAUDE.md files.
2. **`docs/src/**` becomes the central reference point** for persistent
   decisions / strategies / policies / patterns / roadmaps. Instructions
   live in CLAUDE.md files.
3. **The structural pattern** `docs/src/**` and `.claude/rules/**` follow
   — lives in `.claude/rules/**`, referenced via `@` from CLAUDE.md.
4. **Migrate the relevant `.claude/plans/**` docs** into the new
   structure; update references/backlinks.
5. **Skills strategy** — how/when/what to make SKILLs (progressive
   disclosure, self-review, auto-updates, scripting, schemas).
   Instructions live in CLAUDE.md files.
6. **Custom-agents strategy** — how/when/what to make custom agents
   (skill inclusion, model definition, description optimization,
   permission definition, self-review, auto-updates). Instructions live
   in CLAUDE.md files.

## Owner-supplied target structure (2026-08-03, verbatim)

```
├── .claude/
│   ├── scripts/ # pre-written scripts or tools for doing specific checks of external-state that is common and relevant to this project; useful for project wide pre-authorization of certain tools
│   ├── plans/
│   ├── skills/
│   ├── rules/ # Context that applies specifically to a certain directories or files or tools. Narrow in scope and Sharp. (Use in conjunction with sub-dir local AGENTS.md files)
│   ├── CLAUDE.md # Project wide, less commonly editted; branch-level context, expect to commit
│   └── CLAUDE.local.md # Project wide personal preference context, easily editted; branch-level context, DO NOT commit
├── docs/src/
│   ├── dev/
│   │   ├── adrs # Decision Records about Why (MADR format); includes things like Tool/Library/crate choices, architecture pattern choices, etc
│   │   │   ├── testing/ # Unit, Integration, Regression, E2E, API Contract, Patterns (Entrait vs ... ; selections or hybrids), etc
│   │   │   ├── fuzzing/
│   │   │   ├── benchmarking/ # Micro, Macro, CI, etc
│   │   │   ├── data-validation/
│   │   │   ├── release/ # Versioning, Gates, etc
│   │   │   │   ├── gates.md
│   │   │   │   └── versioning.md
│   │   │   ├── deployment/
│   │   │   │   └── platforms.md
│   │   │   ├── integration/
│   │   │   │   └── platforms.md
│   │   │   ├── interface/ (desktop, cli, web, api, tui)
│   │   │   ├── performance-profiling/ (cache-performance.md, timing-consistency.md, profile-guided-optimization.md)
│   │   │   ├── telemetry/ # Tracing, Metrics, Logs, etc
│   │   │   ├── security/
│   │   │   └── safety/ (memory-safety.md)
│   │   ├── configuration/management
│   │   ├── strategies/  # General priorities, goals, objectives — same topic taxonomy as adrs/
│   │   ├── policies/    # Enforcements, MUST vs SHOULD — same topic taxonomy
│   │   ├── patterns/    # Implementation-specific patterns — same topic taxonomy
│   │   └── roadmaps/
│   ├── user/
│   └── admin/
├── AGENTS.md
├── CLAUDE.local.md # DO NOT commit
└── CLAUDE.md
```

*(Topic taxonomy under strategies/policies/patterns mirrors adrs/:
testing, fuzzing, benchmarking, data-validation, release, deployment,
integration, interface{desktop,cli,web,api,tui}, performance-profiling,
telemetry, security, safety. Full verbatim tree is in the owner's
message of 2026-08-03; this compression preserves every leaf class.)*

## Inventory — what exists today (verified 2026-08-03)

| Surface | State |
|---|---|
| Root `CLAUDE.md` | thin pointer (Rule 07 mandates it stay so) |
| `.claude/CLAUDE.md` | working agreement; **owner's 6 TODO markers** for lifecycle/skills instructions now sit at the bottom |
| `.claude/rules/00–09` | ten rules, **all auto-loaded every session** (observed: the full rules text appears in session context) — every line is a per-session token cost |
| Root + per-crate `AGENTS.md` | all six crates covered as of the quality review; root refreshed |
| `CLAUDE.local.md` | does not exist; **not gitignored anywhere** (trap: first one created would be committable) |
| `.claude/agents/` | **exists** — six designed agents (adversarial-reviewer, datastore-screener, detector-author, benchmark-runner, adr-author, phase-scribe) + a README with a strong convention ("an agent without a named output artefact is a chat"). Written 2026-08-01; predates current frontmatter/best-practice review |
| `.claude/skills/`, `.claude/scripts/` | do not exist — greenfield |
| `docs/src/` | mdbook; **flat `adrs/` only** (0001–0026) + README + SUMMARY.md (manual) |
| Structure-coupled tooling | `docs/scripts/check-xrefs.sh` (`ls docs/src/adrs/*.md` — flat-path assumption), `docs/justfile adr-next` (same), SUMMARY.md (hand-maintained), `mdbook test` via `docs::check-links` |
| `.claude/plans/` | the corpus: durable policy docs (TESTING-STRATEGY.md — created 2026-08-03, GLOSSARY.md), roadmap docs (PLAN.md, ROADMAP.md ×4), living ledgers (CARRY-FORWARD.md, NEXT-SESSION.md), historical records (phase docs, quality-review/*, artifacts), product corpus (prds/, specs/, questions/, research/) |
| Governing rules touched by this goal | Rule 02 (ADR location = `docs/src/adrs/NNNN`, sequential, flat), Rule 07 (file split, ~80-line root budget, one-home-per-fact), plans/README (derivation rules, "quality-review section", thin-spots) |

## Seam analysis — five workstreams, and where they cut

**W1 — Context-file lifecycle meta-instructions** (goal items 1, 2).
New prose in CLAUDE.md files answering: what triggers an update (phase
close? refutation? drift found?), what triggers a *review* (the F-13
lesson: twelve→sixteen stale instances existed because **no refresh
trigger exists** — this workstream is the systemic fix for that
finding), and where each kind of context goes. Landing markers already
placed by the owner. Depends on W2 (the where-things-go answer is the
structural rule).

**W2 — Structural conventions rule** (goal item 3). One or two new
`.claude/rules/**` files defining the docs/src taxonomy (what counts as
ADR vs strategy vs policy vs pattern), the rules-dir conventions
("narrow and sharp", scoped, used with sub-dir AGENTS.md), and the
`@`-reference pattern from CLAUDE.md. This partially supersedes/extends
Rule 07 and **amends Rule 02** (ADR location). Rule 02 says changing it
requires an ADR — **the restructure needs its own governing ADR
(0027?)** before implementation.

**W3 — docs/src/dev taxonomy + migration + backlinks** (goal items 2,
4). The heavy, risky slice: create the taxonomy (on-demand, see T4),
decompose/move the relevant plans docs, sweep every backlink, update the
structure-coupled tooling (check-xrefs, adr-next, SUMMARY), keep
`just audit` + mdbook build green at every step.

**W4 — Skills strategy + `.claude/scripts/`** (goal item 5). Greenfield.
Candidate skills visible from existing practice: slice close-out
protocol, phase pre-commitment authoring, measurement-run protocol
(W-2 policy), xref-audit-and-fix loop, ADR authoring (overlaps agent
roster — see T8). Requires research against **current** Claude Code
skills guidance (progressive disclosure, frontmatter, allowed-tools,
scripts embedding) — search, don't recall (the ADR-0025 discipline).

**W5 — Custom-agents strategy** (goal item 6). NOT greenfield: six
agents exist with a good local convention but predate current best
practices. Work: review each against current guidance (description
optimization for delegation, model selection, tool permissions, skill
inclusion), write the when-to-create instructions, decide the
skill-vs-agent boundary (see T8).

Dependency order: **W2 (+ governing ADR) → W1 → W3; W4/W5 parallel after
W1's instructions exist.** W3 is the only workstream that moves existing
content; everything else is additive.

## Tensions and open decisions (the part worth arguing about)

**T1 — Every-session context budget vs richer standing instructions.**
The empirical baseline: all ten rules already load into every session,
and `@`-references from CLAUDE.md also load eagerly. Rule 07's budget
logic ("a 400-line root AGENTS.md is a context tax paid on every turn")
applies with full force to this goal: the owner's tree implies MORE
standing instruction text. The design must decide, per instruction,
*every-session* (rules/CLAUDE.md) vs *on-demand* (docs/src, skills —
skills are the progressive-disclosure mechanism designed for exactly
this). **The next session must verify actual loading semantics** (are
`.claude/rules/**` unconditionally loaded? is there path-scoped
conditional loading? what does CLAUDE.local.md actually do in current
Claude Code?) via the claude-code-guide agent / current docs — not from
memory. A before/after token-budget measurement should be an acceptance
criterion.

**T2 — Rule 02's flat sequential ADRs vs topic-nested `adrs/<topic>/`.**
26 ADRs exist; the corpus cites `ADR-NNNN` hundreds of times; tooling
assumes flat paths. Options: (a) keep global sequential numbers, nest
files in topic dirs, update tooling (preserves "never reused", breaks
only paths); (b) leave 0001–0026 flat, apply the taxonomy to new ADRs
only (two regimes forever); (c) renumber per-topic (**reject**: breaks
every reference and Rule 02's never-reuse). Leaning: (a), with
`git mv`, tooling updates, and the governing ADR recording it. Note the
existing `adrs/README.md` and SUMMARY.md must move in lockstep.

**T3 — The 4-way split (ADR/strategy/policy/pattern) vs one-home-per-fact.**
The taxonomy multiplies homes per topic ×4; Rule 07 says duplicated
facts drift. The split only works with **crisp boundary definitions**,
which W2's rule must state. Draft to refine: *ADR* = why we chose
(immutable; superseded, never edited) · *strategy* = goals, approach,
priorities (evolves; dated amendments) · *policy* = MUST/SHOULD +
gates (binding; executable where possible; what Rules 00/01/09 are
today at workspace level) · *pattern* = how-to recipes with examples.
Pilot decomposition: **TESTING-STRATEGY.md** (created 2026-08-03) maps
cleanly — taxonomy/roster → strategies/testing; W-2 variance +
naming + regression MUSTs → policies/testing (and benchmarking);
test-double placement → patterns/testing; adoptions → ADR-0026 (already
done). Use it as the migration template before touching anything else.

**T4 — Pre-created scaffolding vs create-on-first-content.** Most tree
leaves are inapplicable today (desktop/tui interface, deployment,
configuration-management, user/, admin/). Recommendation: the tree is a
**namespace contract** (documented in W2's rule), directories created on
first real document — matches house ethos (MVP scope, no empty
promises) and avoids a forest of stub files that read as coverage
(the F-15/"silent gap" lesson inverted).

**T5 — Migration inventory: which plans docs are "relevant".** First
cut — *move*: TESTING-STRATEGY.md (pilot, ×4 decomposition), root
ROADMAP.md + PLAN.md content that is roadmap-shaped → `dev/roadmaps/`
(but note per-product ROADMAPs derive from root — the derivation rule
in plans/README must move with them), GLOSSARY.md → docs/src/dev/.
*Stay in plans/*: phase docs, quality-review deliverables, artifacts
(historical records — Rule 02's "documents get dated addenda" protects
them), CARRY-FORWARD + NEXT-SESSION (living session machinery), PRDs /
specs / questions / research (product corpus; a later phase could
migrate specs to `dev/` but that is NOT this goal — scope it out
explicitly or the migration balloons). Open: do `.claude/rules/**`
policies (e.g. Rule 05's span tables, Rule 09's channel list) become
`docs/src/dev/policies/telemetry|security/` with rules shrinking to
pointers? That is the "narrow and sharp" end-state but doubles W3's
blast radius — decide in the plan, possibly defer to a follow-up slice.

**T6 — mdbook consequences.** docs/src gains dev/user/admin audiences;
SUMMARY.md is hand-maintained (26 manual ADR lines today — one was
already missed, caught 2026-08-03); at taxonomy scale manual SUMMARY
will not survive. Evaluate mdbook auto-summary options or a generated
SUMMARY section in the plan. `docs::check-links` (mdbook test +
check-xrefs) is the standing gate; extend check-xrefs to the new roots.

**T7 — CLAUDE.local.md.** Two gitignore entries needed (root and
`.claude/`) **before** any local file is created. Verify current
harness behavior for CLAUDE.local.md (it has changed across Claude Code
versions — verify, don't recall).

**T8 — Skill-vs-agent boundary.** The existing agent roster and the
candidate skills overlap (adr-author agent vs adr-authoring skill;
phase-scribe agent vs pre-commitment skill). Current platform guidance
distinguishes them (skills = loadable instructions/workflows in the
main loop; agents = separate context + tool policy). The when-to-use
instructions (W4/W5) must state the boundary so the two rosters don't
compete. The existing README's bar ("named output artefact or it's a
chat") is worth preserving verbatim wherever the instructions land.

**T9 — History and no-rewrite discipline.** `git mv` for every move
(preserves blame); no published-history rewrite; moved docs leave a
pointer at the old path only where external references are plausible
(plans/README gets the map); Rule 02 forbids silently editing accepted
ADRs — path moves are fine, content edits are not.

**T10 — The restructure is itself ADR-worthy and rule-governed.** It
changes Rule 02 (a non-negotiable-adjacent rule) and Rule 07. Per
Rule 02: write the ADR first (next free number after 0026), then
implement. The ADR records the taxonomy boundaries (T3) and the ADR
relocation decision (T2) — including the consequence disliked.

## Recommended phasing (for the next session's plan to adopt or refute)

* **Slice CR-1 — research + decisions + pre-committed plan** (one
  session, no corpus edits): verify harness semantics (T1, T7) and
  current skills/agents best practices (W4/W5) via claude-code-guide /
  live docs; settle T2/T3/T5 with the owner where needed; write the
  governing ADR (proposed status) + draft the W2 structural rule; emit
  a pre-committed migration plan (`01-plan.md`) with acceptance
  criteria and the backlink-sweep method. Reviewable before anything
  moves — the QR-2/QR-3 separation, same reason.
* **Slice CR-2 — migrate + land** (one session): tooling updates first
  (check-xrefs, adr-next, SUMMARY strategy), then the pilot
  (TESTING-STRATEGY decomposition), then the rest of the inventory;
  CLAUDE.md meta-instructions land at the owner's TODO markers; rules
  land; gitignore entries; gates green at every commit.
* **Slice CR-3 — skills + agents** (one session): bootstrap the first
  skills (+ `.claude/scripts/`), upgrade the six agents to current best
  practices, land the when-to-create instructions. Separable; can lag.

## Acceptance-criteria sketch (CR-1 pre-commits the real set)

1. `just audit` + `just docs::build` + `just docs::check-links` green
   after every landing commit; zero dangling `ADR-NNNN` references.
2. Every moved fact has exactly one home (Rule 07 spot-check:
   no statement in both a docs/src file and a rule/plan).
3. **Every-session context cost measured before/after** (lines or
   tokens of auto-loaded context); an increase needs a written reason.
4. Rule 02 held: governing ADR exists before implementation; ADR
   numbering globally sequential and never reused; no accepted ADR's
   content silently edited (moves ≠ edits).
5. The owner's six TODO markers in `.claude/CLAUDE.md` all resolved —
   filled or explicitly deferred with a dated line.
6. `CLAUDE.local.md` gitignored (root + `.claude/`) before any exists.
7. Old paths for moved docs: `git mv` used; plans/README carries the
   old→new map; no broken inbound reference from phase docs/artifacts
   (they are historical records — their inline paths get a dated
   correction only where a reader would actually be misled).
8. Skills/agents instructions cite the current-version guidance they
   were checked against (search-don't-recall, with dates).

## Known constraints carried from house rules

Conventional Commits; docs-typed work is version-invisible; `--no-ff`
only for implementation phases (this is docs work — main is fine per
Rule 08); refutations-first in any results write-up; the ADR-0015
funnel is untouched by all of this.
