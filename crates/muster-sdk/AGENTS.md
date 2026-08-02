# crates/muster-sdk — search and orchestration

Consumes `orrery`'s contract (`is_feasible` / `score`); never redefines it.
Will own (Phase 5): greedy assignment, objective composition, local search,
batch orchestration, change-set computation. Currently a compiling stub.

**Must never contain** (Rule 03): feasibility semantics, violation
definitions, UI, delivery. `just check-scope` fails the build if a UI/server
dependency (`dioxus`, `leptos`, `yew`, `axum`) enters the tree
(QUESTION-0015).

Testing: `just test`; optimality tests (`optimality_` prefix) must show
greedy matches brute force on fixed-start-time instances (n ≤ 12) once the
solver lands.
