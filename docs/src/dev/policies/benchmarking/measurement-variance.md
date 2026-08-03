# Benchmarking policy: measurement variance (W-2)

*Decomposed 2026-08-03 from `plans/TESTING-STRATEGY.md` (QR-3/Stage E)
by CR-2 under ADR-0027 — full provenance note in
[coverage-taxonomy](../../strategies/testing/coverage-taxonomy.md).
Binding on every `measure_` harness and any successor. Changes land as
dated amendments.*

1. **Warm-up**: at least one untimed warm-up pass before any timed sample.
2. **Iterations**: at least 3 measured iterations — in-process rounds
   where state permits (measure_select), separate process-level runs where
   in-process repetition would measure warm paths (measure_alpha_budgets;
   the harness header must say which and why).
3. **Statistic**: report the median across rounds/runs and the max
   (or cross-run p50/p95); never a single run.
4. **Profile**: measurements run under the release profile pinned in the
   workspace `Cargo.toml` `[profile.release]`; the header names the profile.
5. **Assertions**: in-suite assertions are order-of-magnitude sanity
   bounds (10×), never knife-edge budgets — the H4 precedent (97.8 ms
   median vs a 100 ms budget) is exactly the flake a threshold assertion
   would create. Budget *verdicts* live in phase docs, with provenance.
6. **Reproduction tolerance**: a quoted median reproduces if a re-run
   median lands within 2× (the order-of-magnitude claim class these
   harnesses make). Any gate needing finer resolution pre-commits its own
   tolerance in the harness header *before* measuring (Rule 01.1/01.2).
7. **Provenance and pins**: every quoted number names its script, scale,
   host, and date (Rule 01.5); measurement toolchains are version-pinned
   (`evidence/run_all.sh` pins `ladybug==0.19.0`).
