# Rule 05 — Observability

## Libraries instrument; binaries configure

* `orrery` and `muster-sdk` emit `tracing` spans and events. They **never**
  install a global subscriber, never initialise an exporter, never depend on
  `opentelemetry-*` directly.
* `muster` installs the subscriber and selects the exporter via `figment`
  config: `opentelemetry-stdout` in development, `opentelemetry-otlp` or
  `opentelemetry-prometheus` in deployment, bridged with
  `tracing-opentelemetry`.

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
