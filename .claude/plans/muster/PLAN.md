# Muster — product plan (derived; sequencing lives in ../PLAN.md)

| Phase | Scope for muster | Exit condition | Status |
|---|---|---|---|
| 02 | compiling stub binary in the workspace | workspace builds | in progress |
| 06 | application surfaces in dependency order: member self-selection → coordinator groups → violation inbox → analytics → room assignment | phases/06-app.md | blocked by Phase 5 |

**The muster spec set is deliberately thin** (one overview file) because the
application surface was the least-discussed part of the design thread. Before
Phase 6: write data model, API surface, user flows, non-functional, and
testing specs — a real gap to fill, not an oversight to copy. Frontend
structure is QUESTION-0015, deliberately open until Muster PoC.
