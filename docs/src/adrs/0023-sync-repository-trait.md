# 23. The repository trait is synchronous

* Status: accepted
* Date: 2026-08-01
* Relates to: ADR-0015 (open), ADR-0021, ADR-0022; Rule 04 (async section)

## Context and Problem Statement

Rule 04 requires the sync-vs-async decision to be made once, during Phase 1a
screening, because the repository trait is the boundary that forces it: an
async-only backend would make the trait async and everything above it would
inherit that, and retrofitting async through a synchronous trait is a rewrite.

## Decision Drivers

* Every Phase-1a survivor (Grafeo, agdb) exposes a synchronous Rust API.
* The SQLite fallback path (`rusqlite`) is synchronous.
* `MemoryRepo` (Phase 2) is trivially synchronous.
* `salsa` derivation (ADR-0016 C) composes naturally over synchronous reads.
* Orrery performs no network I/O (Rule 06); the classic motivation for an
  async persistence boundary — network round-trips — is absent by design.

## Considered Options

* **Synchronous trait.** Matches every store actually in play; zero runtime
  dependency; callers that live in an async context (the `muster` server)
  wrap calls in `spawn_blocking`.
* **Async trait.** Insures against a future async-only backend at the cost of
  an async runtime dependency in `orrery` today (violating the spirit of
  Rule 06's transitive-runtime bar), function-colouring the entire engine, and
  complicating salsa integration — for no candidate that survived the screen.
* **Dual surface (sync core + async adapter).** Deferred: an async adapter
  crate can wrap a sync trait later; the reverse is the rewrite Rule 04 warns
  about.

## Decision Outcome

**The repository trait is synchronous.** Async candidates screened in Phase 1a
(Grust, LanceGraph, neo4rs) were each eliminated on independent grounds, so no
capability is lost. If ADR-0015's Phase-7 decision ever selects an async-only
store, that selection supersedes this ADR explicitly and prices in the
migration; it may not be eroded incrementally.

### Consequences

* `orrery` and `muster-sdk` take no async runtime dependency.
* `muster` bridges at its own boundary (`spawn_blocking` or an adapter).
* The Phase-1b screening harness is written against the sync surfaces of both
  survivors, doubling as the bindings smoke test ADR-0021 intends.
* The consequence to dislike, recorded per Rule 02: if the Rust graph
  landscape shifts toward async-only stores over the project's life, this
  decision ages badly and its supersession is a real migration, not an edit.
