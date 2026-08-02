# Glossary

Domain terms carry precise meanings here; several differ from common usage.

| Term | Meaning |
|---|---|
| **Orrery** | The engine. A clockwork model computing where bodies will be — here, people. |
| **Muster** | The application. To assemble personnel at a place and time and verify presence. |
| **attends** | Person -> Event. Carries the priority stack, provenance, and its own window (join/leave may differ from the event's span). |
| **held** | Location -> Event. **Carries its own window, not the event's** — this is what makes partial-duration overflow expressible. |
| **expects** | Group -> Event. A group-level expectation with obligation level, default priority, and its own validity window. |
| **member_of** | Person -> Group. Temporal and role-bearing; the role is where authorisation for coordinator override lives. |
| **anchors** | Person -> Structure. Personal origin (home, office). Many per person, time-conditioned. **Never exposed across the coordinator boundary.** |
| **traverse** | Continuous travel. Cost is a scalar duration; depart whenever. |
| **transit** | Scheduled travel. Cost is a *function of departure time*. Breaks the Layer-2 scalar cache; v2. |
| **tier** | Containment position: room / floor / structure / campus / region. |
| **portal** | Routing role: none / pedestrian / vehicle / rail. **Orthogonal to tier** — a station is a Structure that is a rail portal. |
| **posture** | Whether an event is held remote, hybrid, or on-site. |
| **overflow_for** | On `held`. A *location reference*, not a boolean, so spillover chains (C overflows B overflows A) are expressible. |
| **derived attendance** | Attendance computed from group expectations rather than written at authoring time. Has deterministic content-addressed identity. |
| **effective priority** | Result of the group/person/coordinator precedence stack. Computed in exactly one place. |
| **divergence** | \|coordinator priority - person priority\|. An analytic that exists only because the stack keeps components separate. |
| **violation** | A first-class record, not a computed result. Has lifecycle: detected, acknowledged, waived, resolved. |
| **blast radius** | The set of derived state affected by one write. Unbounded and invisible under derived semantics; made computable by salsa early cutoff. |
| **early cutoff** | Salsa property: if recomputation yields an unchanged value, dependents do not re-fire. The antidote to unbounded blast radius. |
| **entity-partitioned** | Every Orrery query filters by person or location *before* applying its interval predicate. The single most consequential fact about this workload. |
| **stop-gate** | A screening criterion whose failure is architectural rather than performance-related. Per-hop edge filtering is the one for datastore selection. |
