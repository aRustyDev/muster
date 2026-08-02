# Muster-SDK — product plan (derived; sequencing lives in ../PLAN.md)

| Phase | Scope for muster-sdk | Exit condition | Status |
|---|---|---|---|
| 02 | compiling stub crate in the workspace, scope guard (`check-scope`) | workspace builds | complete |
| 05 | greedy assignment, objective composition, local search, batch orchestration | greedy matches brute force on fixed-start instances (n ≤ 12); phases/05-sdk.md | complete — PoC/Prototype gates met, Alpha behaviours landed; Beta churn gate stays open for realistic-scale measurement |

Deferred: CP-SAT (SDK RC or documented rejection — OR-Tools Rust binding
maturity unverified). Open question: 0005 (answered — approach), 0015 lives
with muster. Spec gap closed at Phase-5 entry: SPEC-02 (API + data model) and SPEC-03 (testing) written 2026-08-02.
