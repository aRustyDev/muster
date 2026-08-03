<!-- Imported from design thread 2026-08-01. Review, do not assume correct. -->

# 1. Project naming: Orrery, Muster-SDK, Muster

* Status: accepted
* Date: 2026-08-01

## Context and Problem Statement

The system needed a name. The obvious namespace ("scheduler", "planner") is
crowded and describes the wrong thing: the system does not produce schedules,
it decides whether a proposed arrangement of people in time and space is
physically possible.

## Decision Drivers

* Name the mechanism, not the CRUD.
* Distinguish engine from application from the layer between them.
* Low collision risk.

## Considered Options

* **Worldline** — a particle's single path through spacetime; conflicts are
  worldline forks. Most precise, but collides with a large European payments
  company.
* **Ephemeris** — a table of positions of bodies at specified times. Literally
  the primary data structure. Heavy prior use in astronomy tooling.
* **Orrery** — a clockwork model that computes where bodies will be.
* **Muster** — to assemble personnel at a place and time and verify presence.
* **Lightcone**, **Chronotope**, **Spacelike** — considered, narrower.

## Decision Outcome

**Orrery** for the engine, **Muster** for the first application,
**Muster-SDK** for the layer between.

Orrery names the computing mechanism; Muster names the act the application
performs. "Muster" already implies verifying presence at a place and time,
which is exactly what the engine computes.

### Consequences

* Component vocabulary is available and consistent.
* Trademark/namespace check on crates.io, npm, and USPTO still outstanding.
