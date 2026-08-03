# crates/muster-ui — the browser UI

*(Created 2026-08-03, quality review UI-5 — this crate had no AGENTS.md.)*

Dioxus **0.7** (pinned 0.7.x; do not track 0.8-alpha — ADR-0025) web/WASM
components rendering `muster-types` DTOs. Structure landed at Prototype;
content, the REST client, and the `dx` web entrypoint are Alpha scope.
Talks plain REST/JSON to muster-server — never server functions (the
load-bearing half of ADR-0025).

**Must never contain**: interval math or any feasibility logic (Rule 03 —
the engine owns semantics; this crate renders labels), an orrery
dependency (DTOs only), or anchor/coordinate data in any form.

## Build / test

* `just test` at the workspace root runs its unit tests host-side
  (`--all-features`); the bare no-features library is compiled by the
  cargo-hack leg of `just matrix` (first exercised 2026-08-03 — F-11).
* The WASM app exists only via `dx serve --features web`; `dx` is a dev
  tool for UI work only.
* The UI testing approach (render/snapshot mechanism, REST-client double,
  wasm-perf, a11y floor) is **RR&P-8**, decided at the Muster-Alpha
  pre-commitment — until then the single `hhmm` unit test is the honest
  extent of coverage.

## Gotchas

* No instrumentation before Beta, by decision (Rule 05, UI-4 line).
* The yearly Dioxus breaking minor is contained here by design; one 0.8
  migration is budgeted for 2026–27 (ADR-0025).
* When the Alpha frontend guidelines are written they must carry the
  ADR-0003 window trap (group expectations have validity windows —
  render the window, not just the event).
