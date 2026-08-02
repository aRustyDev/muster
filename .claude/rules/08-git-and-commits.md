# Rule 08 — Git and commits

* **Conventional Commits** on every subject: `type(scope): summary` —
  `feat`, `fix`, `chore`, `refactor`, `docs`, `perf`, `ci`, `test`.
  `release-please` parses these; a malformed subject is a skipped commit.
* **Work-item IDs go in a footer** (`Refs: <id>`), never the subject — a
  prefix breaks Conventional Commits, a suffix leaks into the changelog.
  Never use `BREAKING CHANGE:` as an ID token; it forces a major bump.
* **Any commit that changes a non-negotiable (Rule 00), adds a dependency to
  `orrery` (Rule 06), or deviates from a stated plan references its ADR
  number in the body.** No ADR, no commit — write the ADR first.
* **Branch per implementation phase** (`feat/phase-NN-<slug>`), merged
  `--no-ff` when the phase's acceptance criteria are green. Documentation
  amendments (scorecards, addenda, phase-doc updates) may land on `main`
  directly.
* **Never rewrite published history.** Corrections are new commits;
  documents get dated addenda (Rule 02), not edits that hide the past.
* Commit messages state what changed and why in sentences; the phase
  document, not the commit body, carries the full evidence trail.
