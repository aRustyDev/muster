# Phase 01b — Rust datastore screening harness (ADR-0021 Stage B)

* Status: `complete`
* Blocks: nothing (Stage B is non-blocking by ADR-0021); its output narrows the
  Stage C / Phase 7 finalist list
* Blocked by: Phase 01a (complete — survivors: Grafeo, agdb, Cozo-by-owner-override)

## Objective

The three Stage-A survivors are each exercised from Rust on this host: their
crates built, their bindings smoke-tested (CRUD + per-hop temporal filtering),
and the three acceptance queries (Q1 derived expansion, Q2 per-person
conflicts, Q7b windowed co-attendance) measured at S and M scale **with result
materialisation** — the exact measurement Phase 0 flagged as missing from every
Python number (evidence/README.md known-limitation 1). Order-of-magnitude
losers are eliminated per the pre-committed criterion below. The harness is
throwaway by design (ADR-0021: "Stage B is throwaway"); its findings are not.

## Method notes (pre-committed, Rule 01.1)

Written before any harness code existed or any candidate crate was compiled.

* **Data is distribution-equivalent to `evidence/orrery_spike.py`, not
  bit-identical.** The generator ports the *shape* (same entity counts per
  scale, same distributions: ~10% expired memberships, group tree of branching
  3, per-person sampled attends with naturally seeded conflicts, uniform event
  starts over 14 days) under a fixed Rust RNG (ChaCha8, seed 42). Python's
  Mersenne-Twister streams are not reproduced, so **absolute result counts
  will differ from the Python runs and must not be compared to them.** What
  must be identical — the differential check — is results ACROSS the three
  engines on the same generated data.
* Only the entities Q1/Q2/Q7b touch are generated: persons, events, groups,
  member_of, subgroup_of, expects, attends. Rooms/structures/held/traverse are
  out of scope for Stage B (they feed Q4–Q6, which are not acceptance queries
  here).
* **Q1 includes depth-0 direct-group expectations** (Phase 0 Refutation 2;
  the `*0..5` correction). Semantics: memberships of `pid` valid at `t` →
  ancestor closure via `subgroup_of` edges *each individually valid at `t`*,
  ≤5 hops, **including the direct groups themselves** → `expects` edges valid
  at `t` → deduplicated event-id set, returned as the actual id set.
* **Q7b semantics pinned:** distinct persons `q ≠ p` sharing an event with
  `p` where the *event's* `start_ts ∈ [300000, 600000]`; returned as the
  person-id set. (`q ≠ p` is explicit in every engine, so no engine's
  path-uniqueness rules can skew the differential check.)
* **Q2 semantics pinned:** unordered event pairs `(e1 < e2)` both attended by
  `pid` whose attends intervals overlap (`a1.start < a2.end AND a2.start <
  a1.end`), returned as the pair set.
* Both modes are measured per query: **materialised** (collect full result
  rows into a `Vec`, then hash + length — the values cross the binding
  boundary) and **count-only** (engine-native count where the query surface
  has one; otherwise the cheapest count the surface allows, recorded as
  such). ≥3 timed runs per cell (plan: 5); median and max reported.
* All engines run **in-memory** for parity: Grafeo default in-memory session
  (no `wal`), agdb in-memory db, Cozo `mem` engine. Engine choice per
  candidate is recorded in Results. RocksDB for Cozo only if `mem` fails.
* agdb `Distance` counts nodes AND edges (Phase 01a carry-forward): subgroup
  depth 5 from the person origin = distance 12 to the group, 14 to the event
  (person→member_of-edge→g0 is distance 2; each subgroup hop adds 2;
  expects-edge→event adds 2). Getting this wrong is expected to be caught by
  the differential check — that is what the check is for.
* Grafeo Q1 uses the **GQL dialect** (quantified patterns with per-hop WHERE,
  feature G050); the Cypher dialect lacks inline var-length predicates
  (Phase 01a scorecard). If the `{0,5}` zero-lower-bound quantifier is not
  accepted, Q1 falls back to a union of a depth-0 branch and a `{1,5}`
  quantified branch — semantically identical; the syntax outcome is recorded.
* Because the ported generator gives every `subgroup_of` and `expects` edge a
  `[0, 1e9]` validity window (faithful to the Python shape), the per-hop
  temporal filter never actually excludes anything in the benchmark data. The
  bindings smoke test therefore includes a **separate micro-fixture**
  (probe-01-style: a short group chain with one expired subgroup edge
  mid-chain) asserting each engine's per-hop filter excludes expectations
  reachable only through the expired edge, while retaining depth-0 and
  pre-expiry ones.
* Differential correctness is asserted on **sets, not counts**, across ≥10
  fixed pids per scale, on every query, every scale, every engine pair. Any
  mismatch is a critical finding to diagnose, not paper over.
* Timings are taken on pid 7 (as in the Python runs).
* Scales: S then M (counts as in `orrery_spike.py` SCALES). L (1M attends)
  only if M completes in reasonable wall time; S+M with a complete
  differential check is pre-declared to beat L with a broken one.
* Host for every number in this file: Apple Silicon macOS (Darwin 25.4.0),
  rustc 1.97.1, `--release`. Every table names its invocation. (Written
  2026-08-01; the canonical run executed 2026-08-02 — provenance block in
  Results.)
* Candidate crate versions (crates.io, checked 2026-08-01): grafeo 0.5.42,
  agdb 0.13.2, cozo 0.7.6. A crate that fails to build at all is a
  first-class finding (the bindings smoke test is half the point of Stage B);
  the harness records it and continues with the others.

## Hypotheses (pre-committed)

| # | Hypothesis | Falsified by | Status |
|---|---|---|---|
| H1 | All three candidates' crates (grafeo 0.5.42, agdb 0.13.2, cozo 0.7.6) build on this host and pass a basic CRUD smoke test from Rust | any crate failing `cargo build --release` after bounded remediation (≤ ~1 h of feature/version adjustment), or failing a create/read/update/delete round-trip | **confirmed** — 3/3 build on rustc 1.97.1, 3/3 pass CRUD (cozo with `default-features = false, features = ["graph-algo"]` → mem engine; the sqlite-bundling default feature set was not exercised) |
| H2 | Each candidate can express Q1 with per-hop temporal filtering AND depth-0 inclusion, and all built candidates return set-identical Q1/Q2/Q7b results on identical generated data (≥10 pids, S and M) plus the expired-edge micro-fixture | any engine unable to express the semantics, or any cross-engine set mismatch that diagnoses to engine behaviour rather than a harness bug | **confirmed, two caveats reported prominently** (Results §2–§3): grafeo cannot express Q1 as ONE statement — its parser rejects a quantified pattern concatenated into a longer path, so Q1 runs as a 6-statement union; and the differential check caught a real grafeo evaluator bug in a discarded faster idiom (Results §1) |
| H3 | No candidate is an order of magnitude slower than the others on Q1/Q2/Q7b at M scale with result materialisation | a candidate ≥10× slower than the fastest engine on two of the three acceptance queries (→ that candidate is eliminated, per the criterion below) | **confirmed under the pre-committed clause — no elimination.** Per-query ≥10× gaps exist and are reported (Results §4): grafeo Q2 is 602× the fastest at M; cozo Q2 is 16×; each on that one query only |
| H4 | Result materialisation (full rows vs count-only) does not change the relative per-query ranking of the engines at M scale | any query whose engine ordering differs between modes beyond run noise | **confirmed** — same per-query engine order in both modes at S, M, and L (one order-preserving tie: M Q1 pid 23 count-only, grafeo 0.39 = cozo 0.39). Mode deltas mostly <10%; largest ~46% on a sub-ms cozo cell (M Q1 pid 23: 0.72 → 0.39, count pushdown), no rank inversion. Caveat: largest materialised result here is 2,172 rows; 10⁵-row materialisation remains unmeasured (Phase-0 caveat still open) |

## Acceptance criteria (pre-committed)

**Elimination criterion (pre-committed, mirrors ADR-0021 "order-of-magnitude
losers"):** a candidate that is **≥10× slower than the fastest candidate on
two of the three acceptance queries (Q1, Q2, Q7b) at M scale, in materialised
mode (median of ≥3 runs)** is eliminated from Stage C. Additionally, a
candidate whose crate fails to build on this host after bounded remediation,
or which cannot express Q1's per-hop + depth-0 semantics at all, fails the
bindings smoke test and is eliminated.

| Criterion | Threshold | Actual | Verdict |
|---|---|---|---|
| Builds | 3/3 crates compile in `tools/screening` on this host | 3/3 (grafeo 0.5.42, agdb 0.13.2, cozo 0.7.6 on rustc 1.97.1) | pass |
| CRUD smoke | pass create/read/update/delete per built engine | 3/3 PASS (canonical run, smoke section) | pass |
| Per-hop micro-fixture | each built engine excludes the expired-edge subtree, retains depth-0 | 3/3: Q1 fixture = {0,1} exactly (e2/e3/e4 excluded); plus a direct pure-form G050 probe on grafeo (expired hop prunes: {11}, not {11,12}) | pass |
| Differential check | Q1/Q2/Q7b result sets identical across built engines, ≥10 pids, S and M | identical at S, M, **and L**; 10 pids × 3 queries × 3 scales; aggregate rows compared: q1 125/196/353, q2 2/114/550, q7b 122/4,885/22,091. The check also caught one real engine bug during development (Results §1) — it has teeth | pass |
| Performance screen | no candidate hits the elimination criterion; any that does is eliminated | grafeo ≥10× on 1 of 3 at M (Q2, 602×); cozo ≥10× on 1 of 3 (Q2, 16×); agdb 0 of 3 → **no candidate eliminated** | pass — empty elimination set |
| Materialisation measured | every timing cell exists in both modes, ≥3 runs, median+max | 5 runs/cell, both modes, S/M/L, median+max recorded (Results §4) | pass |
| Provenance | every number names scale + host + invocation | run block at top of Results; single canonical log | pass |

## Plan

1. This document (pre-commitment) — written before any harness code. done
2. Scaffold `tools/screening/` as a standalone cargo workspace (own
   `[workspace]` table; root `Cargo.toml` already excludes the path).
   Reversible: delete the directory.
3. Seeded generator (ChaCha8 seed 42) porting the Python shape at S/M(/L).
4. `Engine` trait: `load`, `q1/q2/q7b` in materialised + count-only forms;
   three impls (Grafeo GQL, agdb QueryBuilder, CozoScript). CRUD +
   expired-edge micro-fixture per engine.
5. Differential check (sets) across engines, then timings: 5 runs/cell,
   median+max, both modes, S then M; L only if M wall time is reasonable.
6. Fill Actual/Verdict; refutations first; report.

All steps reversible; nothing outside `tools/screening/**` and this file is
touched; nothing is committed.

## Results

Run provenance (Rule 01.5): canonical run `tools/screening/logs/screen-SML-final2.log` — Apple
Silicon macOS (Darwin 25.4.0), rustc 1.97.1, 2026-08-02, working dir
`tools/screening/`, invocation `cargo run --release -- S M L`, ChaCha8 seed
42. Engines: grafeo 0.5.42 (in-memory, GQL dialect, property index on `id`),
agdb 0.13.2 (`DbMemory`), cozo 0.7.6 (`mem` engine, non-default features
`graph-algo` only, explicit `attends:by_event` index). Development runs
(2026-08-01: `tools/screening/logs/screen-SM-run1.log`, `tools/screening/logs/screen-SML-run2.log`) are cited only for
the two idiom comparisons explicitly marked as such. Timings: median/max of
5 runs. Differential: 10 pids {1,3,7,11,19,23,42,57,73,91} × 3 queries × 3
scales, compared as sets.

### Findings that must not be skimmed past (Rule 01.3 — warnings first)

**§1 — The differential check caught a real grafeo evaluator bug.** A
faster Q2 idiom (two separately-anchored patterns:
`MATCH (p:Person {id: 3})-[a1:attends]->(e1), (p2:Person {id: 3})-[a2:attends]->(e2) WHERE e1.id < e2.id AND a1.start_ts < a2.end_ts AND a2.start_ts < a1.end_ts`)
returns the pair **(8, 178)** for pid 3 at S although person 3's intervals
are e8 = [694800, 698400) and e178 = [619200, 622800) — disjoint
(694800 < 622800 is false). agdb, cozo, and grafeo's own shared-variable
form (`(p)-[a1]->(e1), (p)-[a2]->(e2)`) all return the correct empty set.
The harness ships the correct shared-variable form; the reproduction is
preserved as `cargo run --release -- debug-q2`. Two consequences: (a)
grafeo 0.5.42's evaluation of cross-pattern edge-property predicates over
comma-joined anchored patterns is not trustworthy; (b) this is direct
evidence for keeping set-level differential testing in Stage C — a
count-level check would ALSO have caught this one (0 vs 1), but only by
luck of the sign.

**§2 — Q1 cannot be one statement in grafeo 0.5.42.** The parser rejects a
quantified parenthesized pattern concatenated inside a longer path:
`...(g0:Grp)((qa:Grp)-[qr:subgroup_of]->(qb:Grp) WHERE ...){0,5}(g:Grp)...`
fails with `syntax error: [GQL] Expected RETURN, FINISH, or SELECT` at the
concatenation point (same for `{1,5}`; verbatim in the canonical log). G050
per-hop WHERE **does** work when the quantified pattern is the *entire*
MATCH — verified first-hand by a pure-form probe (expired mid-chain hop
prunes correctly). So the Stage-A stop-gate verdict stands, but its useful
scope is narrower than the scorecard implied: Q1 runs as a client-side
union of 6 fixed-depth statements (per-edge WHERE at each depth), and the
count is client-side. Phase-01a's "Q1 does not need restructuring" is
**refuted for grafeo** at the single-query level; the semantics survive,
the ergonomics do not.

**§3 — Grafeo's planner ignores `WHERE p.id = N` as an anchor.** With a
property index on `id` present, `MATCH (p:Person) WHERE p.id = 7 ...` scans
(Q2 at S: 19.5–19.9 ms, `screen-SM-run1.log`); the inline form
`MATCH (p:Person {id: 7}) ...` anchors (0.45–0.62 ms at S) — a ~30–40×
planner cliff on the identical query. All shipped grafeo queries use the
inline anchor. Repository code built on grafeo would have to know this.

**§4 — The pre-committed elimination criterion fired on nobody** — the
screen found no order-of-magnitude loser *across two of three queries*.
What it did find, reported with equal prominence: **grafeo is 602× the
fastest engine on Q2 at M** (36.1 ms vs agdb 0.06 ms; cozo 0.97 ms), the
one acceptance query that is a pure self-join. At L this grows to 436 ms
(~3,960× agdb, 140× cozo) and grafeo also crosses 10× on Q7b at L (10.25 ms
vs cozo 1.01 ms) — i.e. **at L grafeo would be ≥10× on two of three, but
the criterion was pre-committed at M and is honoured as written** (Rule
01.2: the criterion is not respecified after seeing data; the L trend is
recorded for Phase 7). cozo's single ≥10× cell (Q2, 16×) is fixed per-call
overhead — CozoScript is parsed and planned per `run_script`, a
~0.1–1 ms floor that dominates every small per-person query and would push
toward caching/pre-planning in a real repository.

**§5 — Method supplement disclosed (Rule 01.2).** Under seed 42 at M, the
pre-committed bench pid 7 has zero valid memberships at t = T_MID, making
the M/L Q1 cell at pid 7 near-degenerate (it measures early-exit). After
seeing this, a *second deterministic* Q1 cell was added: the pid with the
largest Q1 result among the 10 differential pids (engine-neutral — all
engines return identical sets; M: pid 23, |Q1| = 44; L: pid 1, |Q1| = 64;
S: pid 1, |Q1| = 18). The pid-7 cells are still reported. No other
criterion or hypothesis was touched after measurement began.

### Measurements (canonical run)

Data (ChaCha8 seed 42; distribution-equivalent to the Python shape, NOT
bit-identical — do not compare counts to Python runs):

| scale | persons | events | groups | member_of | subgroup_of | expects | attends |
|---|---|---|---|---|---|---|---|
| S | 100 | 200 | 10 | 210 | 9 | 37 | 1,000 |
| M | 2,000 | 2,000 | 60 | 4,018 | 59 | 303 | 100,000 |
| L | 10,000 | 10,000 | 200 | 19,980 | 199 | 921 | 1,000,000 |

Bulk load (single-threaded, in-memory engines): 

| scale | grafeo | agdb | cozo |
|---|---|---|---|
| M | 82 ms | 205 ms | 167 ms |
| L | 1,332 ms | 2,261 ms | 1,476 ms |

Query latencies, ms, **median (max) of 5**, materialised mode (count-only
mode differed by <10% on nearly every cell and changed no ranking — full
tables in the canonical log):

| query | scale | n rows | grafeo | agdb | cozo |
|---|---|---|---|---|---|
| Q1 pid 7 | S | 12 | 0.23 (0.24) | 0.05 (0.06) | 0.55 (0.59) |
| Q1 pid 1 | S | 18 | 0.28 (0.31) | 0.07 (0.07) | 0.60 (0.67) |
| Q2 pid 7 | S | 0 | 0.62 (0.75) | 0.02 (0.02) | 0.13 (0.16) |
| Q7b pid 7 | S | 19 | 0.09 (0.11) | 0.04 (0.05) | 0.28 (0.34) |
| Q1 pid 7 | M | 0 | 0.32 (0.33) | 0.05 (0.05) | 0.16 (0.20) |
| Q1 pid 23 | M | 44 | 0.41 (0.47) | 0.13 (0.15) | 0.72 (0.88) |
| Q2 pid 7 | M | 10 | **36.11 (36.79)** | 0.06 (0.08) | 0.97 (1.02) |
| Q7b pid 7 | M | 355 | 1.99 (2.40) | 0.34 (0.56) | 0.30 (0.32) |
| Q1 pid 7 | L | 22 | 1.23 (1.27) | 0.14 (0.17) | 0.48 (0.61) |
| Q1 pid 1 | L | 64 | 1.39 (1.53) | 0.22 (0.27) | 0.52 (0.64) |
| Q2 pid 7 | L | 62 | **436.0 (459.2)** | 0.11 (0.15) | 3.11 (3.20) |
| Q7b pid 7 | L | 2,172 | 10.25 (10.60) | 2.69 (3.92) | 1.01 (1.20) |

Ratios to fastest, **M materialised (the elimination scale)**: Q1(pid 23)
— agdb 1×, grafeo 3.2×, cozo 5.5×. Q2 — agdb 1×, cozo 16×, grafeo
**602×**. Q7b — cozo 1×, agdb 1.1×, grafeo 6.6×.

Fair-usage notes baked into these numbers: agdb's Q2 pair-join and Q7b
per-event fan-in union are computed in Rust after builder searches (its
documented usage shape — the builder has no join surface), so its Q2 is
essentially a 50-row fetch plus a Rust loop; grafeo queries use the inline
`{id: N}` anchor (§3); cozo uses key-prefix bindings and the explicit
`attends:by_event` index (the Phase-0 `ix_att_e` parity). Every engine got
the best *correct* idiom found within the screening budget; per-call parse
overhead (grafeo, cozo) is included because that is what the documented
call path costs.

### Confirmations

* **H1**: all three crates build clean on rustc 1.97.1 from crates.io and
  pass CRUD — the bindings smoke test found no build-level blocker,
  including for the 2023-vintage cozo (with non-default features; the
  default `compact` feature set, which bundles sqlite, was not exercised).
* **H2**: all three engines expressed Q1 per-hop temporal filtering with
  depth-0 inclusion and returned **set-identical** Q1/Q2/Q7b results at
  S, M and L across 10 pids; the expired-edge micro-fixture excluded the
  poisoned subtree (and the dead membership, and the expired expects edge)
  in all three. agdb's Distance=nodes+edges trap (≤5 subgroup hops =
  distance ≤14 from the person) was encoded correctly on the first try —
  the differential check was armed to catch it and had nothing to catch.
* **H3/H4** as tabled above. Single-statement Q1 exists in cozo (native
  recursion with depth counter) and nearly in agdb (one search query);
  grafeo needs 6.
* L scale ran in full with a complete differential check (the phase's
  "S+M complete beats L broken" trade-off did not have to be taken).

## Decisions produced

None — Stage B is non-blocking and eliminates only on the pre-committed
criterion, which no candidate met. **All three candidates survive to Stage
C consideration on performance.** Inputs handed to Phase 7 / ADR-0015:

* The Stage-B differentiators are now *qualitative*, and they point away
  from grafeo: an evaluator correctness bug (§1), no single-statement Q1
  (§2), a planner anchor cliff (§3), and the worst join performance by 2–3
  orders of magnitude (§4, Q2 at M/L) with a worsening L trend. Whether
  that disqualifies grafeo is a Phase-7/owner decision — it is *not* an
  elimination under this phase's pre-committed rules.
* agdb and cozo both look screen-clean: agdb fastest on person-anchored
  expansion (Q1) and trivial-fan joins (Q2, done client-side), cozo fastest
  on the fan-in join (Q7b) and the only candidate with whole-query
  server-side expression of all three shapes; neither is ever the
  order-of-magnitude loser.
* Q1 result-count note for spec work: all engines agree Q1 includes
  depth-0 (the Phase-0 correction is now triple-implemented and
  differential-checked in Rust).

## Carry-forward

| Item | Resolves in |
|---|---|
| grafeo dual-anchor evaluator bug: minimal reproduction exists (`debug-q2`); consider reporting upstream with the counterexample | opportunistic; before any grafeo selection |
| L-scale trend: grafeo ≥10× on 2 of 3 at L — if Phase 7 benchmarks at L or above, re-evaluate with the Phase-7 criteria (this phase's criterion was M-scale and is closed) | Phase 7 |
| Per-call parse/plan overhead for cozo (and grafeo): a real repository would want cached/prepared query paths; screening measured the raw documented call | Phase 7 harness design |
| Materialisation at 10⁵-row results still unmeasured (max here 2,172 rows) — Phase-0 caveat remains open | Phase 7 |
| No concurrency, no mixed read/write, no transactions exercised (single-threaded harness; matches MemoryRepo restrictive intersection but proves nothing about writers) | Phase 7 / Stage C |
| cozo default feature set (sqlite-bundling `compact`) never compiled here; if Stage C uses a persistent engine, build it then | Stage C |
| Harness disposal: `tools/screening/` is throwaway per ADR-0021; keep until Phase 7 has mined it for query idioms (the correct per-engine idioms are documented in its source) | Phase 7 close |
