# Phase 00 — Grounding and verification

* Status: `complete`
* Blocks: Phase 1a (paper screen) — grounding had to establish whether the
  SQLite baseline numbers are trustworthy before they are used as screening
  criteria
* Blocked by: nothing

## Objective

Every artefact in the handoff package read; every reproducible claim re-run on
this machine; every discrepancy either diagnosed to root cause or recorded as
open. At the end of this phase the package's evidence is graded — reproduced,
reproduced-with-correction, or refuted — rather than trusted.

## Run provenance (Rule 01.5)

All numbers below: `./evidence/run_all.sh`, this host (macOS / Darwin 25.4.0,
Apple Silicon), Python 3.12.x venv (uv), **ladybug 0.19.0 (PyPI)**, **SQLite
3.53.1** (published run used SQLite 3.45). Scales as noted; L = 1M `attends`
edges. Log: session scratchpad `run_all.log`; artefacts `evidence/_work/`.
This host is uniformly ~2.5–3× faster than the published host, so absolute
latencies are compared by ratio, not value.

## Hypotheses

| # | Hypothesis | Falsified by | Status |
|---|---|---|---|
| H1 | `run_all.sh` reproduces the RESEARCH-0002/0003/0004 figures within run-to-run noise (the runner's own closing claim) | any published cell that the shipped scripts cannot produce | **refuted for two cells** — Q7b SQLite latency and Q1 result identity; see Refutations 1–2. All other cells reproduce directionally |
| H2 | Summary claims match their underlying tables (Rule 01.6) | any count that disagrees with the artefact set | **refuted for the framing docs** — PROMPT.md/PLAN.md say "20 ADRs, 13 questions, 4 research"; the package contains 22 / 15 / 5(+template). Root README.md says "20 decisions" two lines above a listing of `0001..0022`. MANIFEST.md is correct. The corrected "3 of 5" win count checks out |
| H3 | The stop-gate result (per-hop edge filtering works in Ladybug; cross-hop predicates do not) reproduces | probe_01 / probe_02 diverging from RESEARCH-0002 | confirmed exactly, including the specific error messages |
| H4 | Cascade saturation (100% of graph by hop 4) reproduces | different reachability counts | confirmed exactly — counts are seed-deterministic (50 / 1,511 / 3,461 / 4,000) |

## Acceptance criteria

| Criterion | Threshold | Actual | Verdict |
|---|---|---|---|
| Harness runs end to end | exit 0, all probes execute | exit 0 (after venv setup; see toolchain note) | pass |
| Tier constraints | 5/5 accept/reject as published | 5/5 | pass |
| Headline ratios (Q1–Q7b, load, disk) | same winner per row as ADR-0015 table | same winner on Q1, Q2, Q3, Q5, load, disk; **Q7b flips as shipped** (see Refutation 1) | pass with one diagnosed exception |
| Any non-reproducing cell | diagnosed to root cause, not waved off | both non-reproducing cells diagnosed and independently verified | pass |

## Results — refutations first (Rule 01.3)

### 1. The shipped harness is the *pre-correction* version; two published cells cannot be reproduced by `run_all.sh`

RESEARCH-0003's "methodology note on fairness" records that the first SQLite
run was unfair — `attends(event_id)` unindexed (Q7b), and Q1 semantics
mismatched — and says both were **corrected before drawing conclusions**. The
headline tables publish the corrected figures. **The packaged scripts are the
uncorrected ones.** Consequences, measured here:

* **Q7b**: published SQLite 1.2 ms. Shipped script: **2,638.5 ms** at L
  (`sqlite_compare.py L`) — the plan scans `attends` for the `event_id` join
  (`EXPLAIN QUERY PLAN`: `SCAN a2 USING INDEX ix_att_p`). Adding the obvious
  index (`CREATE INDEX ix_att_e ON attends(event_id, person_id)`) yields
  **0.46 ms**, same result count (2,808) — consistent with the published
  1.2 ms on a ~2.7× slower host. The published number is *right for an
  indexed store and unproducible by the package as shipped*.
* **Q1 result identity**: ADR-0015 and RESEARCH-0003 claim "identical results
  on every shared query". Shipped scripts return **44 (Ladybug) vs 58
  (SQLite)** at L. Diagnosed: the Cypher walks `subgroup_of*1..5` (excludes
  expectations on the person's *direct* groups); the CTE seeds at depth 0
  (includes them). Removing depth 0 from the CTE reproduces 44 exactly.

Both published *conclusions* survive — I verified each with a corrected run —
but Rule 01.5 is violated: the scripts that produced the headline table are
not in the package. **Carry-forward: commit the corrected harness.**

### 2. RESEARCH-0002's `*0..N` claim is wrong, and the "fairness correction" matched to the wrong semantics

RESEARCH-0002 states zero-hop lower bounds were "not accepted", forcing the
union workaround and, downstream, the decision to align SQLite *down* to
`*1..5` semantics (result 44). On this ladybug 0.19.0,
`-[r:subgroup_of*0..5 (rel, n | WHERE ...)]->` **parses and runs**: 58 events,
2.1 ms at L — identical to SQLite's depth-0-inclusive 58.

This matters beyond pedantry: per ADR-0003/0004, derived expansion **must**
include expectations on directly-joined groups — a person in group *g* where
*g* expects event *e* is the base case. The published "corrected" Q1 compared
two implementations of the domain-*wrong* query. The domain-correct comparison
is: SQLite ≈0.0–0.2 ms vs Ladybug 2.1 ms, result 58 both sides. **Winner and
magnitude unchanged (SQLite, order-of-magnitude), so ADR-0015's evidence
summary stands — but SPEC orrery/02's Q1 definition should explicitly include
depth 0, and the harness should use `*0..5`.**

### 3. probe_01's Probe D is mislabeled and cosmetically misleading

It prints `[OK] increasing property along path` under the heading "temporal
MONOTONICITY across hops", but the query applies only a **constant** per-hop
filter (`r.valid_from >= 0`) and returns path lengths — it does not test
monotonicity at all (its own output includes the expired-edge target g5). The
real cross-hop tests live in `probe_02_cascade.py` (b) and **fail**, exactly
as RESEARCH-0002 records. Conclusion unaffected — cross-hop predicates are
inexpressible (`Binder exception: Variable e is not in scope`, reproduced
verbatim) — but the probe's [OK] would mislead a skim. Fix the label.

### 4. RESEARCH-0004's 19.6 s depth-4 figure is host-specific, not a domain constant

Reachability counts reproduce **exactly** (depth 1–4: 50 / 1,511 / 3,461 /
4,000 = 100%). The latency does not scale like the rest of the harness: this
host measured **1,044 ms** where the package says 19,578 ms — 19× faster,
where every other query is ~2.5–3× faster. Suspect memory pressure on the
original host. The withdrawal argument (ADR-0020) rests on the *semantic*
ground — the answer saturates to "everyone" — which reproduces perfectly; the
"costing 19.6 seconds" garnish should not be re-quoted. Minor provenance note:
RESEARCH-0004's replacement paragraph quotes L-scale Q7b figures (2,808 /
11.3 ms / 1.2 ms) inside an M-scale study without flagging the scale switch.

### 5. Framing-document count drift (Rule 01.6)

PROMPT.md and PLAN.md say "20 ADRs, 13 questions, 4 research"; the package
ships 22 ADRs, 15 questions, 5 research documents plus the scorecard template.
Root README.md says "20 decisions" directly above a contents listing of
`docs/src/adrs/0001..0022`, and its rules listing names 4 of the 9 rule files.
MANIFEST.md is correct throughout. Presumably PROMPT/PLAN predate ADR-0021/0022
and QUESTIONS-0014/0015. No decision content is affected, but it is exactly the
propagation pattern Rule 01.6 warns about.

### 6. Tooling nits found by executing the instructions literally

* `run_all.sh` calls bare `pip` (absent on stock macOS; and
  `--break-system-packages` implies a Linux system Python). Ran under a
  Python 3.12 venv with the venv on PATH. Works unmodified on Linux
  presumably; document the venv path for macOS.
* `just audit` / `check-xrefs.sh` **fails after running the evidence harness**:
  its file count includes generated `evidence/_work/**` and
  `evidence/__pycache__/**` (143 vs 91). Excluding generated paths, the count
  is exactly 91 = MANIFEST. The audit script should exclude generated
  directories, or it will cry wolf on every post-run audit.

## Results — confirmations

All from this host's run; script and scale per row.

| Claim | Source | This run | Verdict |
|---|---|---|---|
| Tier constraints unrepresentable, 5/5 | RESEARCH-0002 H2 | 5/5 accept/reject (`orrery_spike.py`, all scales) | reproduced |
| Per-hop **constant** edge filtering works; g5 excluded | RESEARCH-0002 H1 | `probe_01` Probe B `[OK]`, {g2,g3,g4} | reproduced |
| Inline `WHERE` in rel pattern unsupported | RESEARCH-0002 | Parser exception, verbatim | reproduced |
| Cross-hop reference inexpressible | RESEARCH-0002 / ADR-0020 | both syntaxes fail, errors verbatim (`probe_02` b) | reproduced |
| `WSHORTEST` broken (known limitation 5) | evidence/README | `Cannot evaluate expression with type PROPERTY` | reproduced |
| Q2 per-person: SQLite wins ~6× | RESEARCH-0003 (5.7×) | 0.2 ms vs 1.2 ms (`sqlite_compare.py L` / `orrery_spike.py L`) | reproduced |
| Q3 global sweep: Ladybug wins ~2× | RESEARCH-0003 (2.3×) | 730.6 ms vs 1,380.0 ms | reproduced (1.9×) |
| R\*Tree 2× slower than composite b-tree on Q3 | QUESTION-0013 | 3,438.5 ms vs 1,380.0 ms = 2.49× | reproduced |
| Q5 ≈ tie | RESEARCH-0003 | 5.2 ms vs 4.8 ms | reproduced |
| Bulk load: SQLite ~3× faster | RESEARCH-0003 (3.2×) | 13.5 s vs 6.0 s (2.25×) | direction reproduced |
| On-disk: Ladybug 3× smaller | RESEARCH-0003 | 41 MB vs 123 MB | reproduced exactly |
| Result counts Q2/Q3/Q5/Q7b | RESEARCH-0003 | 60 / 514,122 / 1,231 / 2,808 | exact match |
| Saturation counts by hop | RESEARCH-0004 | 50 / 1,511 / 3,461 / 4,000 | exact match |
| "SQLite wins 3 of 5 comparable" | adversarial-review correction | verified against corrected configuration (Q1, Q2, Q7b vs Q3; Q5 tie) | arithmetic correct — **but only under the corrected harness; as shipped the count would read 2 of 5** |

## What I believe is wrong or unproven in the artefacts (beyond the above)

1. **ADR-0015's Q7b acceptance baseline (1.2 ms) carries hidden fragility**:
   it presumes the `event_id` index that any competent SQLite repository
   implementation would create — fine — but the package cannot demonstrate it.
   Phase 7 comparisons must re-measure the baseline on the decision host with
   the corrected harness, not quote 1.2 ms.
2. **SPEC orrery/05's differential-testing sentence** ("this caught nothing
   because results agreed exactly") is true only of the unshipped corrected
   run; as shipped, Q1 disagrees 44 vs 58. Same root cause as Refutation 1.
3. **Both known-limitation caveats remain fully live**: every benchmark query
   returns `count(*)` (materialisation unmeasured — Ladybug is columnar and
   the published sub-ms SQLite wins could shrink or flip on realistic result
   sets), and all measurements are through Python bindings. Phase 1b's Rust
   harness must return real rows.

## Decisions produced

None — Phase 0 changes no decisions. Two artefact corrections queued for the
package migration (Phase 2): fix probe D's label; exclude generated paths in
`check-xrefs.sh`. The corrected `sqlite_compare.py` (event_id index; `*0..5`
Q1 on both engines) becomes the Phase 1b starting point.

## Carry-forward

| Item | Resolves in |
|---|---|
| Commit corrected harness (indexed Q7b, `*0..5` Q1 both engines, row materialisation) | Phase 1b |
| SPEC orrery/02: state explicitly that Q1 includes depth-0 (direct-group) expectations | Phase 2 spec review |
| Re-measure SQLite baseline on decision host before ADR-0015 closes | Phase 7 |
| QUESTION-0014 (time representation / DST) — open, must close before Phase 3 | Phase 2/3 boundary |
| Async-vs-sync repository trait (Rule 04) — screened as hard requirement 6 | Phase 1a (next) |
