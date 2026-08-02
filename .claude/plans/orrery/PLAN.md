# Orrery — product plan (derived; sequencing lives in ../PLAN.md)

Phases touching the engine. Entry/exit here; cross-product order at the root.

| Phase | Scope for orrery | Exit condition | Status |
|---|---|---|---|
| 00 | ground and verify the handoff evidence | phases/00-grounding.md written | complete |
| 1a | datastore paper screen (blocking) | 2–4 survivors; phases/01a-paper-screen.md | complete — Grafeo, agdb, Cozo (owner amendment) |
| 02 | workspace, model, interval algebra, `Command`, `Repository` trait, `MemoryRepo` (Rule 00b enforcement) | phases/02-workspace.md criteria green | complete |
| 1b | Rust screening harness over the three survivors (non-blocking, parallel with 3–5) | order-of-magnitude losers eliminated | complete — no eliminations (criterion unmet by all); qualitative signals against grafeo recorded for Phase 7 |
| 03 | detectors + brute-force oracles, derived expansion, salsa, digests | property tests green vs oracle | complete — **Orrery Prototype reached** (phases/03-engine-core.md) |
| 04 | travel Layer 1/2, `feasible(person, e1, e2)` landed | phases/04-travel.md | complete |
| 6a | engine surfaces for the app: non-persisting digest dry-run (blocks Muster Alpha slice), analytics + 2-hop co-attendance (blocks Muster Beta slice), 10⁵ budget set defined | phases/06a-engine-surfaces.md pre-committed at entry | not started *(added 2026-08-02, plan review — this work was promised by ROADMAP/specs but owned by no phase)* |
| 07 | down-select 3→2 (ADR-0021 addendum), both finalist repository impls, differential tests, dossier in CARRY-FORWARD.md, **ADR-0015 closes** | Orrery Beta gate | blocked by 06 *(corrected 2026-08-02; previously "03–06" with 03–05 long complete)* |

README: `prds/00-orrery-engine.md` is the why; `specs/00..05` the what;
`questions/` the record of forks taken. Open questions: 0001 (with ADR-0015),
0012 (Stage A answered; performance open). *(0014 closed by ADR-0024 —
stale "open, gates Phase 3" line corrected 2026-08-02.)*
