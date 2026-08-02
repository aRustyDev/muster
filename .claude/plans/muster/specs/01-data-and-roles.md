<!-- Written 2026-08-02 at Phase 6 entry — filling the deliberately-thin
     muster spec set (plans/README: the application surface was the least-
     discussed part of the design thread; a gap to fill, not copy). -->

# muster/SPEC-01 — data and roles

## The rule that shapes everything: the app owns almost no data

Every domain fact — people, groups, events, locations, attendance,
expectations, violations, travel — lives in Orrery behind the command
layer. Muster adds only what the *application* needs:

| App-owned data | Purpose | Never contains |
|---|---|---|
| Account (`account_id ↔ PersonId`, credentials ref, display prefs) | authn/authz binding | domain facts |
| Session state | signed-in context | anchors, coordinates |
| UI preferences | display only | anything the engine reads |

If a proposed table duplicates an engine relation, the answer is no
(Rule 07's drift argument applied to data).

## Roles

Roles are *derived from engine facts*, not stored as app flags:

| Role | Derivation | May do |
|---|---|---|
| Member | any `PersonId` bound to an account | browse, self-select, set own priority, see own schedule + provenance, decline declinable expectations |
| Coordinator | `member_of.role == Coordinator` for group G, valid *now* | manage G's membership windows, attach expectations, suggest/override priorities (visibly distinct — ADR-0005), waive violations touching G, see blast-radius previews |
| Organiser | coordinator of an organiser-designated group | capacity views, room-assignment suggestions |
| Admin | app-level flag (the one true app-owned role) | locations, travel network, accounts |

Authorisation checks read `member_of` at request time — a lapsed
coordinator loses power the moment the membership window ends, with no
app-side revocation step (the validity-window invariant doing authz work).

## The DTO rule (Rule 00.6 / Rule 09)

Coordinator- and organiser-facing DTOs carry **verdicts, never anchor
data**: travel feasibility crosses as `Feasibility` (durations only);
schedules cross as event/room references; no DTO type may have a field
capable of holding a personal anchor's location. Enforced by the privacy
test family at the service layer (muster/SPEC-03), not by UI discipline.
