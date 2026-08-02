# Evidence harness

Reproduces every figure in RESEARCH-0002, 0003, and 0004.

```bash
./evidence/run_all.sh          # ~8 minutes total (faster on Apple Silicon)
```

macOS note: `run_all.sh` invokes bare `pip`/`python3`. On macOS create a venv
first and put it on PATH (`uv venv --seed --python 3.12 .venv && PATH="$PWD/.venv/bin:$PATH"`).

## Corrections vs the handoff package (2026-08-01 — phases/00-grounding.md)

The handoff shipped the *pre-correction* harness while RESEARCH-0003
published corrected figures. Applied here, each commented in-file:

1. `sqlite_compare.py`: `ix_att_e(event_id, person_id)` restored — the
   published Q7b 1.2 ms is the indexed configuration (unindexed: ~2.6 s at L).
2. `orrery_spike.py` Q1: `*0..5` (includes direct-group expectations, the
   domain-correct semantics). Both engines now return identical Q1 results
   (58 at L, person 7) — superseding the published "Q1 44", which was the
   matched-but-domain-wrong `*1..5` form. See the RESEARCH-0002/0003 addenda.
3. `probe_01_recursive.py` Probe D relabelled — it tests a constant filter,
   not cross-hop monotonicity (probe_02 (b) tests that, and fails).

## Ordering is mandatory

| Script | Requires | Runtime |
|---|---|---|
| `probe_01_recursive.py` | nothing | ~10 s |
| `orrery_spike.py S\|M\|L` | nothing | 5 s / 30 s / **~4 min** |
| `probe_02_cascade.py` | `orrery_spike.py M` first | ~60 s |
| `sqlite_compare.py L` | `orrery_spike.py L` first | ~2 min |

Both dependent scripts now fail with an explicit prerequisite message rather
than a cryptic engine error. The L-scale spike takes **~4 minutes** with no
output during bulk load — it has not hung.

## Where artefacts go

`$ORRERY_WORK`, defaulting to `evidence/_work/`. Nothing is written outside the
package. Safe to delete.

## Known limitations (carry into any re-measurement)

1. **Every query returns `count(*)`** — result materialisation across the
   binding boundary is never exercised. The real engine returns hundreds to
   hundreds of thousands of rows. This could shift or reverse rankings.
2. **Python bindings, not Rust.** Ratios are probably directionally sound
   (overhead applies to both engines); absolute sub-millisecond figures are not.
3. **Uniform synthetic event distribution.** Real schedules cluster by time of
   day, which would improve Ladybug's zone-map pruning.
4. **No concurrency, no mixed read/write, no transaction contention.**
5. `WSHORTEST` syntax in `probe_01` fails — the weighted-shortest-path surface
   was never established. Resolve before relying on Q6 at scale.

## Repository placement

Keep at `evidence/` in the repository root. Referenced by
`.claude/plans/orrery/research/*` and by PROMPT.md Phase 0.
