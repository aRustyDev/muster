# Muster-SDK — product plan (derived; sequencing lives in ../PLAN.md)

| Phase | Scope for muster-sdk | Exit condition | Status |
|---|---|---|---|
| 02 | compiling stub crate in the workspace, scope guard (`check-scope`) | workspace builds | complete |
| 05 | greedy assignment, objective composition, local search, batch orchestration | greedy matches brute force on fixed-start instances (n ≤ 12); phases/05-sdk.md | complete — PoC/Prototype gates met, Alpha behaviours landed; Beta churn gate stays open for realistic-scale measurement |
| Beta-close | churn-gate measurement with a **pre-committed instance definition** (class, scale, removal rule, seeds — none defined today) | measurement recorded refutations-first | not planned — folded into the Phase-7 dossier (CARRY-FORWARD.md) *(row added 2026-08-02, plan review MO-8: "SDK Beta / Phase 7" was ambiguous and Phase 7's text never mentioned SDK churn)* |
| MVP / RC | explain-assignment + acceptance trial protocol; CP-SAT evaluate-or-reject + **the SDK perf gates the RC gate references (none exist yet)** | phase pre-commitment required before entry | not planned *(added 2026-08-02, plan review MO-5/MO-8)* |

Deferred: CP-SAT (SDK RC or documented rejection — OR-Tools Rust binding
maturity unverified). Open question: 0005 (answered — approach), 0015 lives
with muster. Spec gap closed at Phase-5 entry: SPEC-02 (API + data model) and SPEC-03 (testing) written 2026-08-02.
