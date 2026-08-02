<!-- Written 2026-08-02 at Phase 6 entry. -->

# muster/SPEC-03 — testing criteria

## The e2e family (`e2e_` prefix — `just muster::e2e`)

* **PoC gate**: a member self-selects two overlapping sessions → the
  conflict is visible end to end through *engine records only* (the app
  recomputes nothing); a group expectation appears on the member's
  schedule as a **derived** entry with provenance naming the group —
  without any attendance write.
* **Prototype gate**: full member flow (browse → select → priority →
  my-schedule with provenance) through the service layer.
* **Alpha gate**: coordinator flow — group, expectation, blast preview,
  inbox triage, waiver recorded with actor + timestamp.

## Blast-radius preview honesty (Alpha)

`preview_expectation` result must **equal** the actual post-commit change
set (the `refresh_digests` output after really committing) on generated
worlds — a preview that lies is worse than none. Property-tested.

## Privacy (`privacy_` prefix, extends the orrery family)

No coordinator-facing DTO, serialised payload, log line, or error carries
an anchor location — asserted mechanically over the service layer's
outputs on worlds *with* anchors present. The engine already guarantees
verdicts-only; these tests catch app-added leaks (the join-table
temptation).

## Boundary

* No feasibility logic in the app: muster sources construct no
  `ViolationKind`, call no detector directly, and re-implement no interval
  predicate — conflicts arrive as violation records or oracle output.
* Scope: `cargo tree -p muster` may gain UI deps at Prototype (per the
  QUESTION-0015 decision), never solver or datastore deps.

## Release gates (ROADMAP)

| Stage | Gate |
|---|---|
| PoC | conflict visible end to end |
| Prototype | member flow complete |
| Alpha | coordinator flow complete |
| Beta | full track scheduled end to end |
| MVP | a real coordinator uses it unaided |
| RC | privacy assertions automated and green; accessibility; ops docs; backup/restore |

*(RC row completed 2026-08-02 — backup/restore was in the ROADMAP RC
contents but missing here. The Beta "full track" and MVP "unaided" gates
still need pre-committed definitions at stage entry: CARRY-FORWARD.md,
review MO-8.)*
