# Muster — product plan (derived; sequencing lives in ../PLAN.md)

| Phase | Scope for muster | Exit condition | Status |
|---|---|---|---|
| 02 | compiling stub binary in the workspace | workspace builds | complete |
| 06 | application surfaces in dependency order: member self-selection → coordinator groups → violation inbox → analytics → room assignment | phases/06-app.md | slice 1 (PoC) complete — conflict visible end to end; Prototype next |

Spec gap closed at Phase-6 entry (2026-08-02): SPEC-01 (data & roles),
SPEC-02 (service API), SPEC-03 (testing). Frontend structure is
QUESTION-0015 — PoC shipped headless by design; decision lands at Prototype.
