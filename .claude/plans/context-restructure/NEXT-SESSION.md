# Next-session kickoff — context/docs restructure (CR-3)

*Rewritten 2026-08-03 at CR-2 close (standing protocol). Delete this
file when the restructure lands fully (CR-3 + the deferred CR-4).*

---

**CR-2 is merged.** ADR-0027 is `accepted`; the ADR corpus lives at
`docs/src/dev/adrs/<topic>/`; the TESTING-STRATEGY pilot decomposition
and GLOSSARY move landed with the old→new map in `plans/README.md`;
W1 instructions replaced the six TODO markers in `.claude/CLAUDE.md`;
Rule 10 is live; Rules 04/05 are `paths:`-scoped. Context budget:
**344 unconditional lines vs the 404 baseline** (verify with
`.claude/scripts/context-cost.sh`, never by trusting this number).

**Owed from CR-2 (verify before anything else):**
1. The SessionStart(compact) hook (`.claude/settings.json` →
   `compact-self-review.sh`) has **not yet been observed firing**
   (§8.7): run `/compact` once in a scratch session and confirm the
   reminder text appears post-compaction; record the observation in
   the ledger. If it does not fire, debug before building on W6.

You are executing **CR-3 — skills, agents, scripts** (01-plan §10;
separable, can lag). Read `01-plan.md` §10 + §1 (research digest) and
`.claude/observations.md` first. Scope:

1. Build only what the ledger supports (rule of three — ADR-0027 §7).
   Current standings: pre-commit generated-dir check (2 rows),
   slice-close skill (1), dated-amendment (1), research-brief (1),
   context-cost script (built at CR-2), `just gates` recipe (1 — cheap,
   consider bundling with any justfile touch).
2. Agent roster review against current sub-agents docs (cite version +
   date; search, don't recall): descriptions, model/tools scoping,
   instruction drift (adr-author was updated by CR-2 — verify, don't
   assume), skill-vs-agent dispositions per ADR-0027 §8.
3. W4/W5 authoring instructions: keep `.claude/CLAUDE.md` additions
   inside the context budget; specifics belong in a docs/src pattern
   page, not in every-session context.

Mechanics: docs-typed work may land on `main` (Rule 08); run
`just audit && just docs::build && just docs::check-links` after every
commit; tag every code fence (mdbook test compiles untagged fences as
Rust). Close per the standing protocol: commit, update
`context-restructure-state` memory, rewrite this file (name commands to
verify state, never expected numbers). Conventional Commits;
`Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>`.
