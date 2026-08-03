# Orrery — product roadmap (derived; boundary matrix lives in ../ROADMAP.md)

| Stage | Contents | Exit gate |
|---|---|---|
| PoC | paper screen closed; `MemoryRepo` + repository trait + command layer | canonical queries run against `MemoryRepo` |
| Prototype | model, interval algebra, all detectors, derived expansion | property tests green vs. brute-force oracle |
| Alpha | salsa incrementality, digests, travel Layer 1/2, analytics | budgets met at 10⁵ edges |
| Beta | API frozen, **both finalist repository impls**, differential tests | budgets met at 10⁶; incremental fuzz green; **ADR-0015 closed** |
| MVP | whatever Muster MVP requires, nothing more | Muster MVP ships on it |
| RC | privacy boundary tested, deterministic rebuild, docs | all orrery/SPEC-05 gates pass |

*Status note 2026-08-02 (Phase 6a): Alpha contents are complete (salsa
incrementality, digests, travel Layer 1/2, analytics) and the exit gate
measured green at the SPEC-03 10⁵ budget set — release profile,
MemoryRepo, decision host (`phases/06a-engine-surfaces.md` Results,
with qualifications). **Orrery Alpha: met.** Beta still requires the
finalist repository impls, 10⁶ budgets, and ADR-0015 closing.*

Dependencies on siblings: SDK Prototype needs this crate's feasibility oracle
**and** scoring; Muster Alpha needs derived expansion and blast-radius
computation. The datastore decision is a **stage gate on Beta, not a date** —
the likeliest failure mode is the decision never being made once something
works (ADR-0021).
