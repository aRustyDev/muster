# Rule 02 — Decision records

* **Format: MADR.** Context and Problem Statement · Decision Drivers ·
  Considered Options · Decision Outcome · Consequences.
* **Location:** `docs/src/dev/adrs/<topic>/NNNN-kebab-title.md` —
  numbering global and sequential, never reused; topic per Rule 10
  *(location amended 2026-08-03 by ADR-0027; was flat `docs/src/adrs/`)*.
* **Write an ADR when:** a choice constrains future work, a non-negotiable is
  being changed, a dependency is added, or a stated plan is being deviated from.
* **Never silently edit an accepted ADR.** Supersede it with a new one and mark
  the old `superseded by NNNN`. Amendment is acceptable only for correcting a
  factual error, and the correction must be visible in the text.
* **Record the consequence you dislike.** ADR-0020 states plainly that
  withdrawing cascade analysis *weakens* the graph case rather than rescuing it.
  That asymmetry is the most valuable sentence in the document and the one most
  likely to be lost in a rewrite.
* **Status vocabulary:** `proposed` · `accepted` · `rejected` · `superseded`.
  ADR-0015 is `proposed` and must not be treated as settled.
