<!-- Imported from design thread 2026-08-01. Review, do not assume correct. -->

# 3. Group-universal events are expressed via an `expects` relation

* Status: accepted
* Date: 2026-08-01

## Context and Problem Statement

A coordinator marks an event as expected for a whole group. Where does that
flag live — on the group node, on the event node, or on a relation?

## Decision Drivers

* The expectation itself carries attributes.
* An array of foreign keys cannot hold attributes.

## Considered Options

* `event.required_for_groups: [id]`
* `group.universal_events: [id]`
* `group -[expects {...}]-> event`

## Decision Outcome

`group -[expects]-> event`, carrying:

| Attribute | Purpose |
|---|---|
| `obligation` | `mandatory` / `expected` / `recommended` — collapses three edge types into one |
| `default_priority` | seeds member `attends.priority_group` |
| `during` | window the expectation applies over — **not** the event's window |
| `cascades` | does this flow to subgroups? |
| `can_decline` | may a member remove it? |
| provenance | who set it, when |

`during` on the expectation is distinct from the event's window: an expectation
added two weeks late must not apply retroactively.

### Consequences

* Same reasoning that put `priority_score` on `attends` rather than `person`.
* Requires the derived-expansion query (Q1).
