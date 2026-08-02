<!-- Imported from design thread 2026-08-01. Fable: review and improve. -->

# RESEARCH 0003 — Ladybug vs. SQLite benchmark

* Date: 2026-08-01
* Method: identical generated datasets loaded into both engines; median of 3–5
  runs; single container. Reproduce with `evidence/orrery_spike.py` and
  `evidence/sqlite_compare.py`.

## Objective

Decide the datastore by measurement rather than by architectural argument.

## Setup

Ladybug 0.19.0 (PyPI) and SQLite 3.45 / Python 3.12. Three scales; results below
at **L**: 10k persons, 10k events, 500 rooms, 200 groups, 20k memberships,
**1,000,000 `attends` edges**.

Both engines returned **identical results on every shared query** — Q3 514,122;
Q5 1,231; Q7b 2,808; Q1 44 — cross-validating correctness of both implementations.

## Results

| Query | Ladybug | SQLite (fair idx) | Winner |
|---|---:|---:|---|
| Q1 derived expansion (recursive) | 4.3 ms | **0.2 ms** | SQLite 21× |
| Q2 per-person conflict | 3.4 ms | **0.6 ms** | SQLite 5.7× |
| Q3 global conflict sweep | **2,051 ms** | 4,775 ms | Ladybug 2.3× |
| Q4 impossible travel | 5.3 ms | — | — |
| Q5 room exclusivity | 16.7 ms | 15.8 ms | tie |
| Q6 travel path (shortest) | 0.8 ms | — | — |
| Q7b co-attend 2-hop | 11.3 ms | **1.2 ms** | SQLite 9.4× |
| Bulk load | 66.7 s | **20.8 s** | SQLite 3.2× |
| On-disk | **41 MB** | 123 MB | Ladybug 3× |

### Methodology note on fairness

The first SQLite run was **unfair** in two ways and was rerun: `attends(event_id)`
was unindexed, penalising Q7b (1,700 ms → 1.2 ms once indexed), and Q1 used
depth-0 semantics against Ladybug's `*1..5`, producing 58 vs 44. Both were
corrected before drawing conclusions. Recording this because the uncorrected
numbers would have supported the same overall conclusion for the wrong reasons.

> **Addendum (Phase 0, 2026-08-01).** The handoff packaged the
> *pre-correction* scripts, so `run_all.sh` as shipped could not reproduce
> two cells of this document: Q7b SQLite (measured 2,638 ms unindexed at L;
> 0.46 ms once `ix_att_e` was added on a ~2.7× faster host — consistent with
> the 1.2 ms here) and the "identical results — Q1 44" claim (shipped
> scripts return 44 vs 58). Both corrections are now applied to `evidence/`
> — **with one deliberate change**: Q1 uses `*0..5` on the Ladybug side
> (depth-0 *included*, the domain-correct semantics per ADR-0003/0004, which
> RESEARCH-0002's addendum shows was expressible all along), so re-runs
> yield **Q1 = 58 on both engines**, superseding the 44 above. Winners and
> ratios are unaffected (corrected-comparison spot-check: SQLite ≈0.0–0.2 ms
> vs Ladybug 2.1 ms at L). Every other row reproduced directionally on a
> ~2.5–3× faster host. Full audit: phases/00-grounding.md.

## Hypotheses refuted

**H1: SQLite's R*Tree gives indexed interval overlap.** Recommended in three
consecutive design messages.

| Q3 access path | Latency |
|---|---:|
| composite b-tree `(person_id, start_ts, end_ts)` | 4,679 ms |
| R*Tree 1-D interval index | **9,372 ms — 2× slower** |

*Two SQLite Q3 figures appear in this document and they are not a typo:
**4,679 ms** is the first-run composite b-tree; **4,775 ms** is the same query
re-measured in the correction run with an added covering index. The 2%
difference is run-to-run noise. Headline tables use 4,775 ms because that run
also carried the corrected Q1 and Q7b figures.*

Mechanism: Orrery's overlap queries are entity-partitioned before the interval
predicate applies. A global 1-D R*Tree cannot exploit that partition.

**H2: derived reads favour columnar/WCOJ engines.** Argued that the recursive
expansion is where factorization shines. SQLite is 21× faster. A 4-deep hierarchy
over a few thousand rows is far too small for columnar advantages to appear.

**H3: missing relationship-table indexes hurt global sweeps.** Ladybug is 2.3×
*faster* on Q3 despite having no applicable index — a global sweep scans
everything regardless, and columnar vectorized scanning beats row-store nested
loop.

## Hypotheses confirmed

**H4: CSR adjacency serves per-person access without an index.** Every per-person
query ran under 6 ms at 1M edges in both engines. The absence of a relationship
index is irrelevant to the interactive path.

## Conclusion

The mechanism, stated generally:

> Every Orrery query is entity-partitioned before its interval predicate applies.
> Partition by person or room and each partition holds tens of rows — exactly what
> b-tree-indexed row stores do best, and where a columnar scan-oriented engine is
> weakest. The graph engine wins only on unpartitioned full-graph sweeps, which
> Orrery runs as batch jobs where 2 s vs 5 s is irrelevant.

## Limitations

* **Every query returned `count(*)` — result materialisation was never
  exercised.** Join and scan work is measured correctly, but transferring N
  result rows across the binding boundary is a code path this harness never
  touched. The real engine returns hundreds of derived edges (Q1), violation
  tuples (Q2), and 514,122 conflict pairs (Q3). Ladybug is columnar and
  plausibly batch-transfers; SQLite steps row-at-a-time. **This could shift or
  reverse rankings**, particularly where SQLite won by the largest margins.
  Same class of error as the refuted R*Tree recommendation: a plausible
  measurement whose shape does not match the workload.
* Python bindings used for both; **Rust binding behaviour untested**. FFI
  overhead (~10-50 us/call) is noise above ~1 ms and applies to both engines
  equally, so the *ratios* are probably directionally sound — but absolute
  sub-millisecond figures are unreliable, and binding maturity, panics, memory
  growth, transaction ergonomics, and async behaviour are entirely unassessed.
* Single container, no I/O contention, cold-cache effects not isolated.
* Synthetic data with uniform event distribution; real schedules cluster by time
  of day, which would change zone-map pruning effectiveness in Ladybug's favour.
* Only one graph engine tested, and it is C++ rather than Rust-native.
* **Count of wins is easy to overstate.** Seven query rows appear in the table
  but only five are directly comparable (Q4 and Q6 have no SQLite counterpart).
  Score is SQLite 3, Ladybug 1, tie 1 — not the 4-of-6 that an earlier draft of
  this package asserted and that propagated into ADR-0015. Corrected.
* No concurrency or multi-writer testing.
