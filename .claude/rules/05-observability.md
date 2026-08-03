---
paths:
  - "crates/**"
---

# Rule 05 — Observability
<!-- paths:-scoped 2026-08-03 (ADR-0027): loads when crate files are read -->


## Libraries instrument; binaries configure

* `orrery` and `muster-sdk` emit `tracing` spans and events. They **never**
  install a global subscriber, never initialise an exporter, never depend on
  `opentelemetry-*` directly.
* **`muster-server` installs the subscriber** and selects the exporter via
  `figment` config (`ORRERY_OTEL_EXPORTER`): `opentelemetry-stdout` in
  development, `opentelemetry-otlp` or `opentelemetry-prometheus` in
  deployment, bridged with `tracing-opentelemetry`. *(Corrected 2026-08-03,
  quality review F-9/SRV-6: this rule named `muster`, but the subscriber
  landed in `muster-server` — the deployable binary and the privacy
  boundary's enforcement point (ADR-0025) — and `muster` carries no
  tracing/figment dependency at all; its dead `run-dev` knob was removed in
  the QF slice. The landed architecture is right; the text was wrong.)*

Violating this makes the engine untestable without a collector and unusable as a
library.

## What to instrument in `orrery`

| Span | Attributes |
|---|---|
| `command.apply` | command variant, actor, receipt id |
| `derive.expand` | person, window, group depth, edges produced, **salsa hit/miss** |
| `detect.run` | detector kind, subjects scanned, violations emitted |
| `travel.closure_refresh` | scope, pairs computed, source (measured/estimated) |
| `repo.<operation>` | **`backend` attribute** — memory / sqlite / graph |

## What to instrument in `muster-sdk` *(added 2026-08-03, quality review SDK-4 — these spans landed ad-hoc in Phase 5; this table makes them spec)*

| Span | Attributes |
|---|---|
| `sdk.suggest` | requests, rooms |
| `sdk.search` | seed_n, max_evals |
| `sdk.batch` | *(none yet — alignment owed at the Muster-Alpha slice: window, digests changed, violations swept)* |

Other dispositions, same date: **muster-ui** carries no instrumentation
before Beta — revisit only with a real diagnostic need (UI-4); the orrery
correlation-ID-per-command promise is implemented and tracked as a
CARRY-FORWARD conditional row (operability work), not owed by any stage.

## The `backend` attribute is load-bearing

Labelling every repository span with its backend means the Phase-7 datastore
decision can be made from **production telemetry under real workload shape**,
not only from synthetic benchmarks. This directly addresses the largest known
weakness in the existing evidence — that every benchmark query returned
`count(*)` and never measured realistic result sizes.

Instrument this from the first repository implementation, not retroactively.

## Never in spans

Personal anchor coordinates, addresses, or anything derived from them.
Span attributes are exported wholesale; a coordinate placed there has left the
engine boundary (Rule 00.6).
