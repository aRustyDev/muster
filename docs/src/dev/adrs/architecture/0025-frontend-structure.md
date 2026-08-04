# 25. Frontend structure: muster-server (axum) + muster-ui (dioxus) + muster-types

* Status: accepted
* Date: 2026-08-02
* Closes: QUESTION-0015

## Context and Problem Statement

Muster needs a delivery mechanism. QUESTION-0015 deferred the decision to
the Prototype slice by design (the PoC shipped headless: service layer +
CLI). The recorded leaning was a three-crate split — `muster-server`
(axum) + `muster-ui` (dioxus/WASM) + a thin `muster-types` DTO crate — and
the kickoff required verifying it against the **current** state of the
ecosystems by web search, not recall (the paper screen's "search, don't
recall" rule applied to dependency decisions).

## Decision Drivers

* Rule 00.6 / Rule 09: the privacy boundary must be enforceable at one
  place — a server — not by trusting a UI not to ask for coordinates.
* Rule 03: UI deps must never enter `muster-sdk` or `orrery`
  (`check-scope`, `check-seam`).
* Rule 06: dependency churn should be isolated where it lands.
* The API surface should outlive any one UI framework.

## Considered Options

1. **Dioxus fullstack, one crate** (server functions, `web`/`server`
   feature flags)
2. **`muster-server` (axum) + `muster-ui` (dioxus) + `muster-types`** —
   the recorded leaning
3. **axum + non-Rust frontend** (TS/React)

## Verification (all sources accessed 2026-08-02)

* **Dioxus is healthy but churns yearly.** 0.7.10 stable (2026-07-30),
  0.8.0-alpha.1 out; ~monthly patch cadence over the last year; company-
  backed (Dioxus Labs, YC S23), 38k stars, active main branch. Pattern:
  one breaking minor per year (0.5→0.6→0.7→0.8α), with the churn
  **concentrated in the fullstack/server-function layer** (0.7 changed the
  server-fn protocol attribute, default codec, and error type). The web/
  WASM renderer is its oldest, most mature target. (crates.io;
  github.com/DioxusLabs/dioxus; dioxuslabs.com/blog/release-070;
  /learn/0.7/migration/to_07/.)
* **axum is the boring choice.** 0.8.9 (2026-04-14), tokio-rs org, nothing
  breaking since 0.8.0 (2025-01-01); no verified 0.9/1.0 timeline.
  dioxus-fullstack 0.7.10 itself requires `axum ^0.8.4` — no version
  conflict either way. (crates.io; tokio.rs blog.)
* **The split is the ecosystem's own scaling endpoint, not a contrarian
  bet.** The Dioxus 0.7 tutorial explicitly recommends pulling the server
  into its own workspace crate as projects grow, and the 0.7 CLI serves
  multi-crate setups first-class (`dx serve @client --package … @server
  --package …`). (dioxuslabs.com/learn/0.7/tutorial/backend/.)
* **The honest counter-consideration:** dioxus-fullstack 0.7 can mount
  onto a self-owned axum `Router` (`dioxus::serve`, `DioxusRouterExt`), so
  the split no longer buys *capability* that fullstack withholds. What it
  buys is (a) **churn isolation** — the yearly Dioxus migration touches
  `muster-ui` only, while the API sits on stable axum — and (b) a
  **framework-independent wire surface**: server-function endpoints are
  framework-shaped and awkward for non-Rust clients (the one concrete
  lock-in account found: leptos-rs/leptos#3624 — same architecture class);
  a plain REST/JSON API is consumable by anything. What it costs is typed
  server functions and typed websockets. No real-world "we regretted
  fullstack and extracted a server" accounts were found — weak evidence
  either way (few 0.7-era codebases are old enough), flagged per Rule 01.4.
* **Option 3** loses compile-time type sharing — most of the value of a
  Rust UI here — and adds a second toolchain; nothing found makes it
  better for a small privacy-sensitive app. Leptos (0.8.20, healthy,
  mid-0.9-transition) is near-equivalent for a plain WASM SPA and would
  not change this structure decision.

## Decision Outcome

**Option 2.** Three crates, siblings under `crates/` (the QUESTION-0015
phrasing "under crates/muster/" is realised as siblings — Cargo handles
nested packages poorly):

* `muster-types` — serde wire DTOs only; no engine types leak to the wire.
* `muster-server` — axum **0.8** REST/JSON API over `MusterService`;
  installs the tracing subscriber and owns the figment config (Rule 05);
  the privacy boundary's single enforcement point.
* `muster-ui` — dioxus **0.7** (pinned 0.7.x; do not track 0.8-alpha),
  web/WASM, built with `dx` 0.7.x. Talks REST to muster-server. Thin at
  Prototype (structure is the deliverable; UI content is Alpha scope).

The API is **plain REST/JSON, not server functions** — that is the load-
bearing half of the decision, per the verification above.

### Consequences

* The yearly Dioxus breaking minor is contained in `muster-ui`; budget
  for one migration in 2026–27 (0.8).
* We forgo typed server functions/websockets; DTO drift between server
  and UI is compile-checked only through shared `muster-types` — which is
  exactly why that crate must stay the single wire-type source.
* `dx` joins the dev toolchain for UI work only; server and workspace
  tests never need it (CI runs cargo/nextest as before).
  *(Factual correction 2026-08-03, quality review F-2/F-11/UI-2: the
  parenthetical assumed a CI that has never existed — no CI configuration
  is present in this repository; the gates it describes run locally via
  `just ci`. The claim that the bare (no-features) muster-ui library is
  "checked by workspace CI" was likewise false as written: that
  configuration was first gate-exercised on 2026-08-03, locally, by the
  cargo-hack no-features leg of `just matrix`. CI bring-up is owned by
  RR&P-1 (GitHub Actions, owner-confirmed 2026-08-03) —
  `plans/quality-review/02-additions-and-order.md`.)*
* If a future consumer needs typed streaming (live violation inbox), add
  it as an explicit endpoint contract in `muster-types` first; reaching
  for dioxus-fullstack then would be a new ADR superseding this one.
* Small-team risk noted: Dioxus Labs is ~4 people, VC-backed, pre-revenue
  on tooling; mitigation is this ADR's structure itself — the UI crate is
  replaceable without touching the API or the engine.
