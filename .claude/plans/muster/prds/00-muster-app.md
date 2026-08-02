<!-- Imported from design thread 2026-08-01. Fable: review and improve. -->

# PRD — Muster (application)

## Problem

Coordinators, advisors, and organisers currently reconcile schedules by hand
across spreadsheets and calendars. They discover conflicts late, cannot see
travel infeasibility at all, and have no view of engagement.

Muster is the first application on Orrery: members self-select what they will
attend, coordinators layer group expectations over the top, and the system
surfaces what is impossible.

## Functional requirements

| ID | Requirement |
|---|---|
| FR-1 | Members browse events and self-select attendance |
| FR-2 | Members set a personal priority per attendance |
| FR-3 | Coordinators define groups and manage temporal membership |
| FR-4 | Coordinators attach expectations to groups with obligation level and default priority |
| FR-5 | Coordinators suggest or override priority; suggestions are visibly distinct from overrides |
| FR-6 | Members see provenance — why an event is on their schedule and who put it there |
| FR-7 | Violation inbox: unresolved violations, acknowledge, waive with reason |
| FR-8 | Per-person schedule view with conflicts and travel infeasibility marked |
| FR-9 | Capacity pressure view — signalled interest vs. allocated capacity |
| FR-10 | Engagement view — low/no engagement members by priority-weighted attendance |
| FR-11 | Priority divergence view — coordinator vs. member disagreement across a group |
| FR-12 | Room-assignment suggestion with accept/reject/edit, via Muster-SDK |
| FR-13 | Location and travel-network administration |

## Out of scope

* Feasibility computation → **Orrery**; search → **Muster-SDK**
* Calendar sync (Google/Outlook/ICS) — post-MVP
* Payments, ticketing, badge printing
* Native mobile applications
* Real-time collaborative editing

## User flows

**Flow A — member self-selection.** Sign in → browse → select → set priority →
see conflicts immediately → resolve or accept.

**Flow B — coordinator group expectation.** Create group → add members with
validity windows → attach expectation → preview blast radius **before commit**
→ commit → review resulting violations.

**Flow C — violation triage.** Open inbox → filter unresolved → inspect
subjects → resolve by editing, or waive with a reason → waiver recorded with
actor and timestamp.

**Flow D — room assignment.** Select events and rooms → request suggestion →
review assignment plus objective breakdown → accept, or edit and re-solve with
stability.

**Flow E — engagement review.** Select group and window → view priority-weighted
engagement → drill into individuals → view divergence between coordinator and
member priorities.

## Dependencies

* `orrery`, `muster-sdk`
* Web UI stack (TBD)
* Auth provider (TBD)

## Privacy requirement (non-negotiable)

Personal anchors are home addresses. Coordinators receive feasibility
**verdicts**, never coordinates. Enforced at the engine boundary, not the UI
(ADR-0014).

## Success criteria

* A coordinator schedules a full semester or conference track end to end
* Every violation class is discoverable and resolvable in the UI
* Blast-radius preview prevents at least one unintended mass change in testing
