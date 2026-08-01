# Screening scorecard — Neo4rs

* Repository / homepage: https://github.com/neo4j-labs/neo4rs · crates.io `neo4rs`
* Licence: MIT OR Apache-2.0 (README); crates.io metadata says MIT
* Language / bindings: pure Rust **network driver** (Bolt protocol)
* Latest release + date: 0.9.0-rc.10, 2026-06-11 (max stable 0.8.0); ~948k downloads
* Contributors (12mo) / commits (12mo): 30 contributors all-time; active within 12 months; 0.9 in rc since mid-2026; README: "work in progress, and not all features are implemented yet"
* Category (A-E): **C** — client driver for the Neo4j server

## Hard requirements

Not screened — **category C disqualifies**: requires an external JVM Neo4j server; nothing embedded. (For the record: server-side Neo4j Cypher *does* support per-hop filtering via quantified path patterns — the capability exists in the ecosystem, just not embeddable through this crate.)

Rust API: async (tokio).

## Verdict

**eliminated (category C)** — exactly as RESEARCH-0005 predicted; confirmed rather than assumed.

**Reason (one sentence):** A Bolt driver, not a store; the datastore behind it is a JVM server process, which fails the embedded requirement by construction.

## Uncertainty

None material to this screen.
