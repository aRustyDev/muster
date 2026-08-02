<!-- Imported from design thread 2026-08-01. Fable: review and improve. -->

# QUESTION 0015 — Frontend structure, and where Dioxus may live

* Status: **CLOSED — ADR-0025 (2026-08-02)**: muster-server (axum 0.8) +
  muster-ui (dioxus 0.7, REST client, no server functions) + muster-types;
  the leaning held under web-verification, with the counter-consideration
  (fullstack can mount a self-owned axum router — the split buys churn
  isolation and a framework-independent API, not capability) recorded in
  the ADR.
* Raised: 2026-08-01

## Question

Dioxus was proposed "for muster / muster-sdk". Frontend structure is undecided.

## Answer

**Unresolved, and deliberately deferred.** The immediate finding is negative
rather than positive: `muster-sdk` is the wrong home for a UI framework. Where
it *does* live is open until Muster PoC — the frontend is not on the critical
path and deciding early buys nothing.

## Immediate finding: `muster-sdk` is the wrong home

`muster-sdk` is the search and orchestration layer. A UI framework there
violates Rule 03 directly — and the concrete cost is that **any future
application would inherit a UI dependency it does not use**, which is the
specific reason the SDK was split out of the app in ADR-0019.

`just muster_sdk::check-scope` fails the build if `dioxus`, `leptos`, `yew`, or
`axum` appear in its dependency tree.

## Options for where it does live

| Option | Shape | Trade |
|---|---|---|
| **Dioxus fullstack in `muster`** | one crate, server functions | simplest; type sharing is free; couples rendering and orchestration in one binary |
| **`muster-server` (axum) + `muster-ui` (dioxus)** | two crates under `crates/muster/` | clean boundary; needs a shared DTO crate; more scaffolding |
| **`muster-server` + non-Rust frontend** | axum + TS | widest hiring pool; loses type sharing, which is most of Dioxus's value here |

## Leaning

Option 2, with a thin `muster-types` crate holding DTOs. It keeps the
privacy boundary (Rule 00.6) enforceable at one place — the server — rather
than relying on the UI not to request coordinates.

Option 1 is defensible for an MVP and cheaper to start. If chosen, write the ADR
noting that extracting the server later is a real cost.

## Do not decide before

Muster PoC. The frontend is not on the critical path — Orrery and the SDK are —
and deciding early buys nothing. Revisit at Muster Prototype.

> *Status note (2026-08-02, Phase 6 slice 1): the PoC shipped headless — a
> service layer plus CLI demo — exactly so this question could stay open.
> The decision point is now Muster Prototype, where the ADR gets written;
> the leaning above (option 2, muster-server + muster-ui + thin types
> crate) stands unchanged by anything the PoC learned.*
