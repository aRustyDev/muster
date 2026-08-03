# Agent roster — what to create here

Six subagents, in priority order. Each gets a narrow mandate and a defined
output artefact. **An agent without a named output artefact is a chat, not an
agent** — do not create one.

## 1. `adversarial-reviewer` (highest value — build first)

Reviews any document set or code change for defects the author cannot see.
Runs the four passes that caught real defects in this package:

1. **Arithmetic integrity** — recompute every summary claim against its
   underlying table. "Wins N of M" statements are the most-copied and
   least-checked sentences in any evaluation; this package shipped `4 of 6`
   when the truth was `3 of 5`, into two load-bearing documents.
2. **Cross-reference resolution** — every `ADR-NNNN`, `SPEC`, `QUESTION-NNNN`,
   `RESEARCH-NNNN` reference resolves to an existing file; spec references are
   product-qualified.
3. **Claim drift** — statements that hardened from inference to fact while
   propagating between documents. Compare each assertion to its origin.
4. **Consumer simulation** — execute the document's own instructions literally,
   from a clean environment, in the stated order. This is how the
   `/home/claude` hardcoding and the undocumented script ordering were found.

Output: a findings document, tiered critical/moderate/low, each with a
reproduction.

## 2. `datastore-screener`

Executes RESEARCH-0005 Stage A. Applies the category taxonomy and the five hard
requirements. **Must produce one scorecard per candidate using the shared
template** — 20 candidates with inconsistent write-ups are unusable.

Output: `research/0005-*` scorecards + survivor list with elimination reasons.

## 3. `detector-author`

Writes a violation detector, its brute-force oracle, and its property tests **as
one unit**. Refuses to emit a detector without an oracle. This is the single
highest-leverage constraint on engine correctness.

Output: detector module + oracle + proptest suite, all three or none.

## 4. `benchmark-runner`

Runs the harness, records provenance (script, scale, commit, host), compares
against committed budgets, flags regressions. Never reports a number without
naming the run that produced it (Rule 01.5).

Output: benchmark record appended to the phase document.

## 5. `adr-author`

MADR format, sequential numbering, supersession handling, status vocabulary.
Refuses to silently edit an accepted ADR. Enforces Rule 02 — including the
requirement to record the consequence the author dislikes.

Output: `docs/src/dev/adrs/<topic>/NNNN-*.md`.

## 6. `phase-scribe`

Fills `PHASE-TEMPLATE.md`. Enforces pre-committed hypotheses and acceptance
criteria, and refuses to backfill a hypothesis after results are known.

Output: `phases/NN-*.md`.

## Not worth creating

* A general "coder" agent — that is the main session.
* A "documentation" agent — docs are written by whoever made the change.
* Per-crate agents — scope is enforced by Rule 03, not by agent identity.
