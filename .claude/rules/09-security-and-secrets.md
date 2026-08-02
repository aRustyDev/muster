# Rule 09 — Security and secrets

* **No credentials in anything committed.** `muster`'s `figment` config
  layers environment last; secrets arrive by env var or external secret
  store, never a checked-in file. `.env` files are gitignored and still
  should not hold production values.
* **Personal anchor coordinates are secrets** (Rule 00.6). Concretely:
  * never logged, at any level;
  * never in `tracing` span attributes (Rule 05 — exporters ship them
    wholesale);
  * **never in error values.** This is the easy leak: a
    `Err(TravelError { from: <anchor coords>, .. })` built "for debugging"
    ends up in a trace exporter. Error types carry entity IDs, never
    coordinates. Enforced by the privacy tests (`just orrery::test-privacy`),
    which assert no coordinate crosses the coordinator boundary in any
    payload, log, or error.
* Violation records and waivers carry actor IDs and timestamps — not
  addresses, not free-text copied from anchor data.
* Dependencies: `cargo audit` runs in CI once CI exists; a vulnerable-dep
  exception requires a phase-doc line with a removal date.
* Data egress (Parquet/CSV export, ADR-0015 consequence) excludes `anchors`
  relations by default; exporting them is a separate, explicit, logged
  operation.
