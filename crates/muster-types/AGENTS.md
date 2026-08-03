# crates/muster-types — the wire vocabulary

*(Created 2026-08-03, quality review T-3 — this crate had no AGENTS.md.)*

Serde wire DTOs only (ADR-0025): the single source of the REST/JSON
contract shared by `muster-server` and `muster-ui`. ~90 lines of derive
structs; deliberately tiny.

**Must never contain**: engine types (ids are bare `Uuid`s, instants are
µs `i64`s — the server maps at its boundary), location coordinates or any
anchor-shaped field (Rule 00.6/09 — the privacy boundary is enforced
*structurally*: a type that cannot carry a coordinate cannot leak one),
behavior (no `impl` blocks beyond derives), and any dependency beyond
serde + uuid.

## Testing

By design, this crate's contract tests live in **muster-server** — the
privacy boundary's single enforcement point (`privacy_wire` key
allowlist, `wire_names` pinning; see TESTING-STRATEGY's test-double
placement section). The crate gains its first own tests at Muster Alpha:
serde roundtrip properties for every DTO plus the cross-member privacy
test (muster/SPEC-03, T-1/T-2) — a justfile arrives with them.

## Gotchas

* A new DTO field is a **contract change for two consumers**; muster-ui
  compile-checks against it, but serialized *names* are pinned in
  muster-server's `wire_names.rs` — extend that test, don't rename
  casually.
* Conflicts carry kind/severity **strings** and event ids only — a
  member's payload names no other member. That cross-member rule is not
  checkable by the key allowlist; the T-2 test (Alpha) owns it.
