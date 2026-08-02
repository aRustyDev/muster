<!-- Imported from design thread 2026-08-01. Fable: review and improve. -->

# Screening scorecard — <candidate>

One per candidate. **Twenty inconsistent write-ups are unusable** — use this
verbatim.

* Repository / homepage:
* Licence:
* Language / bindings:
* Latest release + date:
* Contributors (12mo) / commits (12mo):
* Category (A-E, see RESEARCH-0005):

## Hard requirements

| # | Requirement | Verdict | Evidence |
|---|---|---|---|
| 1 | **Per-hop edge-property filtering in recursive traversal** (stop-gate) | pass / fail / n/a | link + quote |
| 2 | Native or first-class Rust bindings (not FFI wrapper) | | |
| 3 | Concurrency model permits intended deployment | | |
| 4 | ACID transactions | | |
| 5 | Maintenance signal | | |
| 6 | **Sync or async trait shape** (Rule 04) | sync / async / both | |

Requirement 1 is architectural: a failure disqualifies regardless of speed.
For **category B**, restate it in that model's terms before judging — the
question is whether each recursive step can filter on that step's edge
attributes, not whether the Cypher syntax exists.

## Modelling fit

| Question | Answer |
|---|---|
| Can location tiers be structurally enforced (ADR-0009)? | |
| Can relations carry attributes, including intervals? | |
| Native interval/range type or overlap operator? | |
| Bulk load path for 1e6 edges? |

## Verdict

`advance to Phase 1b` / `eliminated` / `record but not as SoR (category D)`

**Reason (one sentence):**

## Uncertainty

What could not be established from documentation and needs a spike.
