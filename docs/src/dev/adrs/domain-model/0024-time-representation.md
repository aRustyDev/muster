# 24. Time is UTC instants internally; zones are metadata; recurrence is not an engine concern

* Status: accepted
* Date: 2026-08-01
* Closes: QUESTION-0014. Relates to: ADR-0022 (`chrono` boundary rule),
  orrery/SPEC-01, orrery/SPEC-05.

## Context and Problem Statement

QUESTION-0014 identified three DST failure modes with a correctness
dimension: (1) wall-clock recurring events change duration across a
transition; (2) travel-gap arithmetic in local time silently inverts across
spring-forward; (3) interval overlap is not transitive across mixed
representations. Interval representation is load-bearing for every detector,
so this had to close before Phase 3.

## Decision Drivers

* Every conflict check is one interval-overlap predicate (Rule 00.3) — it
  must be closed under comparison, with no representation mixing.
* Both target use cases span DST transitions.
* The engine's consumers, not the engine, own calendars and authoring UX.

## Considered Options

The four from QUESTION-0014: bare UTC instants; `chrono::DateTime<Utc>`
throughout; `DateTime<Tz>` retained per event; instant + authoring zone.

## Decision Outcome

1. **Internal comparison key: `Timestamp(i64 microseconds UTC)`** — landed in
   Phase 2. All interval algebra, all detectors, all repository filters
   operate on instants only. Failure modes 2 and 3 are thereby
   **structurally absent**: there is exactly one comparison representation,
   so gaps cannot invert and overlap cannot depend on rendering.
2. **Authoring zone retained as metadata**: `Event.timezone` (IANA name,
   optional) joins the existing `Group.timezone`. The algebra never reads
   them; recurrence expansion and display do.
3. **Recurrence expansion is not an Orrery concern.** The engine only ever
   sees concrete instances. Expansion of "Tuesdays 14:00 local" into
   instants — including the transition-day 0/7200-second anomalies of
   failure mode 1 — happens in the application layer (Muster), in the
   authoring zone, before commands reach the engine. Shared expansion
   helpers may later live in `muster-sdk`; nothing in the engine changes
   either way.
4. **`chrono` stays at the API boundary** (ADR-0022) and is not an `orrery`
   dependency until a boundary layer actually needs it.
5. **DST fixtures are mandatory** in the SPEC-05 seeded worlds: a
   spring-forward pair whose wall-clock rendering misstates the real gap,
   and a fall-back pair whose wall-clock rendering suggests an overlap that
   does not exist in UTC. Detector tests assert instant-based verdicts.

## Consequences

* Detectors stay pure over instants; no zone database enters the engine.
* Leap seconds are out of scope by construction (Unix-epoch microseconds).
* The consequence to dislike, recorded per Rule 02: **the engine cannot
  detect a mis-expanded recurrence.** By pushing expansion out, a consumer
  that expands "14:00 local" incorrectly feeds the engine plausible-looking
  instants and every verdict downstream is correct-relative-to-garbage. The
  privacy boundary has an automated test; this boundary cannot have one —
  only the consumer's own expansion tests guard it. Muster's Phase-6 specs
  must include transition-day expansion tests, and that requirement is
  recorded here because the engine cannot enforce it.
* `Group.timezone`'s role narrows to "default authoring zone for
  group-created expectations" — display metadata, nothing more.
