# crates/muster — the application

UI, auth, coordinator workflows, notification delivery (Phase 6). Currently a
compiling stub binary.

**Must never contain** (Rule 03): feasibility computation or search
algorithms — those are `orrery` and `muster-sdk` respectively.

Conventions that differ from the libraries:

* `anyhow` at the top level is correct here (Rule 04) — the only crate where
  it is.
* This crate owns observability configuration: it installs the `tracing`
  subscriber and selects exporters via `figment` (`ORRERY_OTEL_EXPORTER=stdout`
  in dev — `just run-dev`). Libraries only *emit* (Rule 05).
* Privacy boundary: coordinators receive feasibility verdicts, never anchor
  coordinates (Rule 09); the automated privacy tests live in `orrery` but the
  payload discipline is enforced here too.
