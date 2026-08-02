<!-- Written 2026-08-02 at Phase 6 entry. Delivery-mechanism-neutral by
     design: QUESTION-0015 (frontend structure) stays open until Muster
     Prototype, so this spec defines the SERVICE layer — the functions any
     frontend (Dioxus fullstack, axum+ui, CLI) would call. -->

# muster/SPEC-02 — service-layer API surface

One service struct wrapping `Engine<R>` + muster-sdk. Every mutation goes
through `engine.apply` (Rule 00.2 transitively); the service adds authz,
DTO mapping, and orchestration — never feasibility logic (Rule 03).

| Surface (PRD flow) | Service call | Maps to |
|---|---|---|
| Browse & select (A) | `events(window)` · `select(person, event, priority?)` | `repo.events` · `AddAttendance` + immediate conflict readback |
| My schedule (A) | `my_schedule(person, window, at)` | `derive::effective_schedule` + open violations touching the person; provenance (self / group / coordinator) on every entry |
| Groups (B) | `create_group` · `add_member(person, group, during, role)` | `UpsertGroup` · `AddMembership` |
| Expectations (B) | `expect(group, event, obligation, prio, during, cascades, by)` | `AddExpectation` |
| Blast preview (B) | `preview_expectation(...) -> Vec<PersonId>` | dry-run: digests before vs after on a scratch evaluation — **preview must equal the post-commit change set** (muster/SPEC-03 gate) |
| Violation inbox (C) | `inbox(filter)` · `waive(id, by, reason)` | `repo.open_violations` · `WaiveViolation` |
| Room assignment (D) | `suggest_rooms(requests, rooms, opts)` | `muster_sdk::suggest_and_refine` |
| Capacity / engagement / divergence (E) | `capacity(window)` · `engagement(group, window)` · `divergence(group, window)` | engine analytics — consumed at Muster **Beta**; engine surface lands in **Phase 6a** *(corrected 2026-08-02: this row said "Phase 6 Alpha", contradicting the slicing below and both ROADMAPs)* |
| Notifications | `pending_changes() -> ChangeSet` | `muster_sdk::batch::run` output; **delivery** is this crate's job and no one else's |
| Locations (admin) | `add_location` · `add_containment` · `add_traverse` | the corresponding commands (tier rules enforce themselves) |

## Phase-6 slicing

* **PoC (slice 1)**: `select` + `my_schedule` end to end — one member
  self-selects, a conflict appears, provenance shows; headless (library +
  CLI demo). No frontend decision needed or taken.
* **Prototype**: browse/select/priority/my-schedule complete; QUESTION-0015
  closes here with an ADR.
* **Alpha**: groups, expectations, blast preview, inbox.
* **Beta**: capacity/engagement/divergence, room assignment.

## Conventions

`anyhow` at the binary edge only; service functions return typed errors.
`figment` configuration enters when the first real deployment knob exists,
not before. Observability: this crate installs the subscriber (Rule 05);
`ORRERY_OTEL_EXPORTER=stdout` in dev.
