<!-- Imported from design thread 2026-08-01. Fable: review and improve. -->

# 22. Dependency baseline

* Status: accepted
* Date: 2026-08-01

## Decision Outcome

Baseline recorded in Rule 06. Points requiring reasoning rather than a list:

### `thiserror` in libraries, `anyhow` only in `muster`

`anyhow` erases the error type. A library that returns `anyhow::Error` forces
every caller into string matching. `snafu` is an acceptable substitute for
`thiserror` if context selectors are wanted — but pick one and apply it
uniformly; mixing two error-derivation styles across three crates produces
conversion boilerplate at every boundary.

### `opentelemetry-*` never appears in a library

`orrery` and `muster-sdk` depend on `tracing` only. `muster` owns subscriber
installation and exporter selection via `figment`. A library that installs a
global subscriber cannot be embedded in a host that already has one, and cannot
be tested without a collector. See Rule 05.

### UUIDv7, not v4

Entity identifiers are time-ordered. Given RESEARCH-0003's finding that this
workload is dominated by **entity-partitioned b-tree access**, insert locality
is not incidental — random v4 identifiers scatter inserts across the index and
degrade exactly the access path the engine depends on. Low cost now, awkward to
change once data exists.

`uuid` is for entity identity. **Derived-edge identity remains `blake3`
content-addressing** (ADR-0016 A) — it must be reproducible from inputs, which a
random or time-based UUID is not. These are two different mechanisms and should
not be collapsed.

### `chrono` at the API boundary only

See QUESTION-0014. Internal representation is unresolved and has a correctness
dimension beyond ergonomics.

### `dioxus` is not a `muster-sdk` dependency

Placing a UI framework in the search-and-orchestration crate violates Rule 03.
See QUESTION-0015. `just muster_sdk::check-scope` fails the build if it appears.

## Consequences

* `[workspace.dependencies]` is the single version source; crates use
  `dep.workspace = true`.
* Async runtime choice is deferred to Phase 1a — it is a *screening requirement*
  for datastore candidates, because an async-only backend forces an async
  repository trait and everything above it inherits that.
