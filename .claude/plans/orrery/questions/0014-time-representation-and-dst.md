<!-- Imported from design thread 2026-08-01. Fable: review and improve. -->

# QUESTION 0014 — Time representation, and what DST does to an interval

* Status: **OPEN — has a correctness dimension, not just an ergonomic one**
* Raised: 2026-08-01

## Question

`chrono` is on the approved dependency list. But what does Orrery store
*internally* for interval endpoints, and what does a "one hour class" mean
across a daylight-saving transition?

## Answer

**Unresolved.** A leaning is recorded below but no decision has been taken.
This question has a **correctness dimension, not merely an ergonomic one** — see
the three failure modes. It must close before the engine core lands (Phase 3),
because interval representation is load-bearing for every detector.

## Why this is not merely a types question

Both target use cases span DST transitions. A semester crosses at least one; a
multi-day conference can. Three failure modes follow, and none are caught by
ordinary tests:

1. **A recurring event defined in wall-clock time changes duration.** "Tuesdays
   14:00-15:00 local" is 3600 s on most Tuesdays and 0 or 7200 s on the
   transition day if the rule is expanded naively in UTC.
2. **Travel feasibility silently inverts.** `gap = next.start - prev.end`
   computed in local wall-clock across a spring-forward yields a negative or
   impossibly small gap, and the impossible-travel detector fires on a schedule
   that is actually fine — or, worse, misses one that is not.
3. **Interval overlap is not transitive across representations.** Two events
   stored in different local zones may overlap in UTC but not in either local
   view, or the reverse, if comparison happens after conversion.

## Options

| Option | Note |
|---|---|
| `i64` microseconds UTC internally, `chrono` at the boundary | fastest comparison, smallest storage, matches the benchmark harness; loses the originating zone unless carried alongside |
| `chrono::DateTime<Utc>` throughout | ergonomic, still loses originating zone |
| `DateTime<Tz>` with zone retained per event | correct for wall-clock recurrence; heavier, and comparison must normalise |
| Store **both** an instant and the authoring zone | most likely correct; two fields to keep consistent |

## Leaning, not a decision

Store `i64` micros UTC as the comparison key **and** retain the authoring
timezone on `Event` — the instant is what the interval algebra operates on, the
zone is what recurrence expansion and display need. Expose `chrono` types at the
API boundary only.

`Group.timezone` already exists in SPEC orrery/01, which suggests this was
half-anticipated and never resolved.

## Required before closing

* Decide whether recurrence expansion is an Orrery concern at all, or belongs in
  Muster-SDK. If the engine only ever sees concrete instances, failure mode 1
  moves out of scope entirely — which is probably the right answer.
* Add DST-crossing fixtures to the SPEC orrery/05 seeded worlds. **The current
  fixture list has none**, so all three failure modes would ship undetected.
