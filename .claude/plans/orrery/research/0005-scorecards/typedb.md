# Screening scorecard — TypeDB

* Repository / homepage: https://github.com/typedb/typedb · https://typedb.com
* Licence: MPL-2.0
* Language / bindings: server implemented in Rust; Rust access is the `typedb-driver` **network driver** (gRPC)
* Latest release + date: 3.12.1, 2026-07-13 (server); driver 3.12.1, 2026-07-21
* Contributors (12mo) / commits (12mo): very active; ~monthly release cadence; funded company
* Category (A-E): **C** — server-only database

## Hard requirements

Not screened — **category C disqualifies**: no embedded mode exists. TypeDB staff, on running in-process: "TypeDB does indeed need to be run as a separate process" (2024-06) and, re the Rust 3.0 core, "it's definitely more possible, but we've not spent resources towards doing it yet!" (2025-02, forum.typedb.com/t/can-typedb-be-run-in-process/493).

Rust driver: async, with a `sync` feature gate. Server: ACID up to snapshot isolation, MVCC.

## Verdict

**eliminated (category C)** — as RESEARCH-0005 predicted; confirmed with primary-source statements rather than assumed.

**Reason (one sentence):** A healthy, well-funded server database with no embedded mode — the deployment model, not the technology, disqualifies it.

## Uncertainty

The no-embedded evidence is a Feb-2025 staff forum post rather than a current doc page (docs URL 403'd); if TypeDB ever ships an embeddable core, re-screen.
