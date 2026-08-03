<!-- Imported from design thread 2026-08-01. Fable: review and improve. -->

# SPEC 00 — Muster application overview

## Surfaces

| Surface | Primary user | Core job |
|---|---|---|
| Browse & select | member | choose events, set priority |
| My schedule | member | see conflicts, travel warnings, provenance |
| Groups | coordinator | membership with validity windows |
| Expectations | coordinator | attach events to groups, set obligation |
| Violation inbox | coordinator | triage, resolve, waive |
| Capacity | organiser | interest vs. capacity, overflow recommendation |
| Engagement | advisor | low-engagement members, divergence |
| Room assignment | organiser | request, review, accept, re-solve |
| Locations | admin | tiers, portals, travel network |

## Non-functional

* Interactive actions reflect within 100 ms of the engine returning
  *(definition reconciled 2026-08-03, quality review F-14/M-9: as
  measured since slice 2 — H4, `measure_select` — the budget is applied
  to the **whole service call including the engine sweep**, which is
  stricter than this line reads. That stricter definition is now the
  binding one: 100 ms covers the service-layer call end to end,
  excluding HTTP transport. The HTTP edge gets its own measurement and
  budget at the Alpha exit (SRV-7, RR&P-2 macro leg) — reconciled here
  before that edge is measured, so the two numbers can't silently mean
  the same thing.)*
* Blast-radius preview **before** committing any group-level change
* Provenance visible on every derived attendance
* Coordinator suggestions visually distinct from binding overrides
* **Coordinators never see personal anchor coordinates** — verdicts only

## Testing

* End-to-end: a coordinator schedules a full track and resolves every violation
  class
* Blast-radius preview matches the actual post-commit change set
* Privacy: assert no anchor coordinate appears in any coordinator-facing
  payload — an automated test, not a review checklist
