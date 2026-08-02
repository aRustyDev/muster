# Muster — product plan (derived; sequencing lives in ../PLAN.md)

| Phase | Scope for muster | Exit condition | Status |
|---|---|---|---|
| 02 | compiling stub binary in the workspace | workspace builds | complete |
| 06 | application surfaces in dependency order: member self-selection → coordinator groups → violation inbox → analytics → room assignment | phases/06-app.md | slice 1 (PoC) complete — conflict visible end to end; Prototype next |
| MVP | auth, tenancy, admin, location management (incl. portal-cost import, 04-travel carry-forward) | phase pre-commitment required before entry | not planned *(row added 2026-08-02, plan review MO-5 — these stages previously had no owner anywhere)* |
| RC | accessibility (level TBD at entry), ops docs, backup/restore, Parquet/CSV egress with anchors excluded | phase pre-commitment required before entry | not planned *(added 2026-08-02, same)* |

Spec gap closed at Phase-6 entry (2026-08-02): SPEC-01 (data & roles),
SPEC-02 (service API), SPEC-03 (testing). Frontend structure is
QUESTION-0015 — PoC shipped headless by design; decision lands at Prototype.
