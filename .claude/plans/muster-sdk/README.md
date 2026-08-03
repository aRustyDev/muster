# plans/muster-sdk

Search-and-orchestration layer planning corpus. Derived `PLAN.md`/`ROADMAP.md`
here; cross-product truth at `../PLAN.md` / `../ROADMAP.md`.

* `prds/00-muster-sdk.md` — why a separate crate (search changes faster than
  feasibility semantics)
* `specs/` — 00 overview, 01 objectives/search, 02 API surface, 03 testing
  criteria *(list refreshed 2026-08-03, quality review F-13 — 02/03 landed
  at Phase-5 entry; this line still called them gaps)*
* `questions/0005` — room-schedule suggestion (answered: tiered
  greedy → local search → maybe CP-SAT; interval graph colouring result)
