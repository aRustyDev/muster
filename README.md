# Orrery / Muster

Event and personnel attendance planning + analysis.

**Orrery** is a spatiotemporal feasibility engine: given a proposed assignment
of people to events at locations over time, it returns the ways that
assignment is impossible, and a score for how good it is. **Muster** is the
first application on it; **Muster-SDK** is the search layer between. Orrery
does not schedule — it decides whether a schedule is possible.

## Layout

```
crates/orrery       engine: model, intervals, detectors, repository trait
crates/muster-sdk   search & orchestration over the engine's contract
crates/muster       application: UI, coordinator workflows, delivery
docs/               mdbook + ADRs (docs/src/adrs/)
evidence/           runnable benchmark harness behind the datastore decision
.claude/plans/      PLAN, ROADMAP, specs, PRDs, questions, research, phases
```

Start with `AGENTS.md`, then `.claude/plans/PLAN.md`. Build/test via `just`
(`just doctor` checks the toolchain). The one open architectural decision is
the datastore — ADR-0015, `proposed`, closing at Orrery Beta through the
funnel in ADR-0021.
