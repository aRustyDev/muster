# Muster-SDK — product plan (derived; sequencing lives in ../PLAN.md)

| Phase | Scope for muster-sdk | Exit condition | Status |
|---|---|---|---|
| 02 | compiling stub crate in the workspace, scope guard (`check-scope`) | workspace builds | complete |
| 05 | greedy assignment, objective composition, local search, batch orchestration | greedy matches brute force on fixed-start instances (n ≤ 12); phases/05-sdk.md | blocked by orrery Phase 3 (needs feasibility oracle AND scoring) |

Deferred: CP-SAT (SDK RC or documented rejection — OR-Tools Rust binding
maturity unverified). Open question: 0005 (answered — approach), 0015 lives
with muster. Specs still thin: data model, API surface, testing criteria are
gaps to fill at Phase 5 entry (plans/README "known thin spots").
