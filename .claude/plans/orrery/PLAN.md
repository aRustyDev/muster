# Orrery — product plan (derived; sequencing lives in ../PLAN.md)

Phases touching the engine. Entry/exit here; cross-product order at the root.

| Phase | Scope for orrery | Exit condition | Status |
|---|---|---|---|
| 00 | ground and verify the handoff evidence | phases/00-grounding.md written | complete |
| 1a | datastore paper screen (blocking) | 2–4 survivors; phases/01a-paper-screen.md | complete — Grafeo, agdb, Cozo (owner amendment) |
| 02 | workspace, model, interval algebra, `Command`, `Repository` trait, `MemoryRepo` (Rule 00b enforcement) | phases/02-workspace.md criteria green | complete |
| 1b | Rust screening harness over the three survivors (non-blocking, parallel with 3–5) | order-of-magnitude losers eliminated; 2 finalists | not started |
| 03 | detectors + brute-force oracles, derived expansion, salsa, digests | property tests green vs oracle | blocked by 02; QUESTION-0014 must close first |
| 04 | travel Layer 1/2, `feasible(person, e1, e2)` landed | phases/04-travel.md | blocked by 03 |
| 07 | both finalist repository impls, differential tests, **ADR-0015 closes** | Orrery Beta gate | blocked by 03–06 |

README: `prds/00-orrery-engine.md` is the why; `specs/00..05` the what;
`questions/` the record of forks taken. Open questions: 0001 (with ADR-0015),
0012 (Stage A answered; performance open), **0014 (time/DST — gates Phase 3)**.
