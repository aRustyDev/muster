---
paths:
  - "crates/**"
  - "**/*.rs"
  - "**/Cargo.toml"
---

# Rule 04 — Rust conventions
<!-- paths:-scoped 2026-08-03 (ADR-0027): loads when Rust/manifest files are read -->


## Errors

* **`orrery` and `muster-sdk` are libraries: typed errors, `thiserror`.** Never
  `anyhow` in a library — it erases the error type callers need to match on.
* **`muster` is a binary: `anyhow` at the top level** for context chaining.
* `snafu` is acceptable in place of `thiserror` if context selectors are wanted
  for deep call stacks — pick one and use it uniformly. Do not mix.
* **No `unwrap()` or `expect()` in library code** outside tests and
  const-evaluable invariants. `MemoryRepo`'s deliberate constraint panics
  (Rule 00b) are the documented exception and must carry a message naming the
  violated constraint.

## Types

* Newtype every identifier — `PersonId`, `EventId`, `LocationId`. A bare `u64`
  crossing a function boundary is a bug waiting for a transposed argument.
* Newtype `Interval` with its own constructors. Reject inverted and (unless
  explicitly permitted) zero-length intervals at construction, not at use.
* Derive `serde::{Serialize, Deserialize}` on model types; keep serde attributes
  out of business logic.

## Layout

* Workspace with shared `[workspace.dependencies]`. Crates declare
  `dep.workspace = true` — no version drift across three crates.
* Public API of `orrery` mentions no concrete datastore type (Rule 00.1).
* Feature flags for each repository backend: `--features repo-memory`,
  `repo-sqlite`, `repo-<graph>`. Default is `repo-memory` so tests need no
  external engine.

## Async

Decide once and write an ADR. The repository trait is the boundary that forces
it: if any candidate backend is async-only, the trait is async and everything
above it inherits that. **Resolve during Phase 1a screening, not after** — this
is a hard requirement to add to the screen, and retrofitting async through a
synchronous trait is a rewrite.
