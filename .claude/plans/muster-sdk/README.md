# plans/muster-sdk

Search-and-orchestration layer planning corpus. Derived `PLAN.md`/`ROADMAP.md`
here; cross-product truth at `../PLAN.md` / `../ROADMAP.md`.

* `prds/00-muster-sdk.md` — why a separate crate (search changes faster than
  feasibility semantics)
* `specs/` — 00 overview, 01 objectives/search. **Deliberately thin**: data
  model, API surface, and testing specs are gaps to fill at Phase 5 entry.
* `questions/0005` — room-schedule suggestion (answered: tiered
  greedy → local search → maybe CP-SAT; interval graph colouring result)
