# Phase 06 — Muster application

* Status: `in-progress` — slices 1 (PoC) and 2 (Prototype) complete; Alpha
  next, **blocked by Phase 6a** (non-persisting digest preview — engine
  work, see PLAN.md)
* Blocks: nothing downstream yet (MVP chain continues within this phase)
* Blocked by: Phases 3–5 (complete — engine, travel, SDK all exist)

Surfaces land in dependency order (PLAN): member self-selection →
coordinator groups → violation inbox → analytics → room assignment.

* **Slice 1 (this pre-commitment): the PoC.** Muster spec set written
  (the last deliberately-thin gap); a headless service layer over
  `Engine<MemoryRepo>`; the PoC story end to end: one member self-selects,
  a conflict appears, a coordinator expectation shows up as a derived
  entry with provenance. CLI demo + `e2e_` test. **No frontend decision**
  — QUESTION-0015 stays open by design until Prototype.
* Later slices: Prototype (full member flow + the QUESTION-0015 ADR),
  Alpha (coordinator flow + blast preview), Beta (analytics + rooms).

## Hypotheses (slice 1, pre-committed)

| # | Hypothesis | Falsified by | Status |
|---|---|---|---|
| H1 | **PoC gate**: conflict visible end to end from engine records; no feasibility logic app-side | conflict invisible or app-side logic | **confirmed** — `e2e_member_selects_and_sees_conflict_with_provenance`; boundary grep (`ViolationKind::` / `detect::` / `overlaps(`) empty over `crates/muster/src`; the schedule's conflict flags are reads of open violation records |
| H2 | Derived entry with provenance, no attendance write | missing/wrong provenance | **confirmed** — "Evening Social — expected via group 'cohort-26'" appears via derivation only; e2e asserts source group by name |
| H3 | No frontend commitment needed for the PoC | UI/server dep entering | **confirmed** — lib + CLI binary; QUESTION-0015 annotated and stays open until Prototype |
| H4 | No new dependencies | any new dep | **confirmed** — none added (`figment` deferral recorded) |

## Acceptance criteria (slice 1, pre-committed)

| Criterion | Threshold | Actual | Verdict |
|---|---|---|---|
| Gates | all green | 66/66 workspace tests; clippy/fmt/xref clean (this host, 2026-08-02) | pass |
| PoC e2e | conflict + provenance | green | pass |
| Boundary | greps empty | empty | pass |
| Demo | stdout story, exit 0 | runs; three-act output with ⚠ flags and provenance line | pass |
| Specs | SPEC-01/02/03 | written at phase entry | pass |
| QUESTION-0015 | annotated | done | pass |
| Artifacts (standing) | plain-language explainer | `artifacts/phase-6-first-app-poc.md` | pass |

## Plan

1. Specs 01–03 (done first). 2. `muster` lib: `MusterService` (select,
my_schedule with provenance + conflict flags, demo world builder) +
`main.rs demo`. 3. `e2e_` test on the library surface (not stdout).
4. Gates, results, artifact, merge.

## Results (slice 1 — PoC)

One near-miss caught by *running* the demo rather than only testing it
(Rule 01.3): derived schedule entries initially displayed the
**expectation's validity window** (which is what a derived edge's `during`
correctly is, per ADR-0003) instead of the **event's window** — the
Evening Social sorted before breakfast. Fixed in the service layer with a
comment; a reminder that the ADR-0003 distinction is easy to trip over at
every consumer, worth a line in the eventual frontend guidelines.

Landed: muster/SPEC-01/02/03 (the last deliberately-thin spec set,
closed); `MusterService` (select with immediate conflict readback,
my_schedule with provenance + violation-record flags); `run_demo` +
`muster demo` CLI; `e2e_` gate green. The **Muster PoC stage gate
(ROADMAP: "conflict visible end to end") is met**, and with it every
product in the workspace has reached at least its PoC.

## Decisions produced

* None requiring ADRs. One additive engine read (`Repository::group()`)
  was needed for provenance display — recorded here per the
  trait-growth discipline.

## Carry-forward

| Item | Resolves in |
|---|---|
| **Prototype slice**: browse/priority surfaces, full member flow, and the QUESTION-0015 frontend ADR (leaning: muster-server + muster-ui + thin types crate) | Phase 6 slice 2 |
| Coordinator flow (groups/expectations service calls replacing the `engine_mut` escape hatch), blast-radius preview honesty gate, inbox + waive | Phase 6 Alpha |
| `select()` currently sweeps the whole window for immediacy — fine at PoC scale; interactive-budget measurement at 10⁵+ | Phase 7 *(superseded 2026-08-02: first measurement pulled into slice 2 — see H4 below; the 10⁵+ engine-scale pass stays Phase 7)* |
| Privacy tests extend to service DTOs on worlds with anchors | Alpha (with coordinator surfaces) |
| `figment` + subscriber installation when the first deployment knob exists | Prototype |

*(From slice 2 on, the cross-phase ledger `plans/CARRY-FORWARD.md` mirrors
these rows — plan review, 2026-08-02.)*

---

# Slice 2 — Prototype (pre-committed 2026-08-02, before implementation)

ROADMAP gate: **"browse, select, priority, my-schedule with provenance —
member flow complete"**, tested through the service layer
(muster/SPEC-03 Prototype gate). Reshaped by the plan review
(`orrery/artifacts/plan-review-2026-08-02.md`): finding CR-6 (the Command
enum has no removal variants) makes **deselect** part of the member flow —
PRD Flow A ends "see conflicts immediately → *resolve* or accept", and
resolving a self-selection conflict means removing one of the selections.
Branch: `feat/phase-06-prototype`.

## Hypotheses (slice 2, pre-committed)

| # | Hypothesis | Falsified by | Status |
|---|---|---|---|
| H1 | **Prototype gate**: full member flow (browse → select+priority → deselect → my-schedule with provenance) through the service layer, conflicts appearing AND resolving from engine records only | e2e red, or boundary greps (`ViolationKind::` / `detect::` / `overlaps(`) non-empty over muster sources | **confirmed** — `e2e_member_browses_selects_prioritises_deselects_and_resolves` + the same story over HTTP; boundary greps empty across all four muster-family crates |
| H2 | The QUESTION-0015 leaning (muster-server axum + muster-ui dioxus + thin muster-types) survives web-verification of the 2026-08 ecosystems, and the split lands without UI/server deps leaking into muster-sdk or orrery | research refuting the leaning; `check-scope` / `check-seam` failing | **confirmed** — ADR-0025 (with the honest counter-consideration recorded); check-scope/check-seam green; `muster` itself gained zero deps |
| H3 | `RemoveAttendance` + service `deselect` complete Flow A: after deselecting one of two conflicting events, the next sweep auto-resolves the violation with zero app-side logic | violation stays open, or resolution requires app-side computation | **confirmed** — e2e asserts conflicts empty and flags gone after deselect; no app-side computation exists to do it with |
| H4 | The whole-window sweep inside `select()` fits the muster/SPEC-00 100 ms interactive budget at Prototype scale (10³ persons, ~5×10³ attends) — measured, not assumed (review MO-2: zero measurements exist) | measured p50 > 100 ms at that scale → person-scoped evaluation becomes a pre-committed Alpha item | **refuted in spirit** — p50 97.8 ms, p95 102.4 ms (this host, 2026-08-02, `measure_select.rs`): the letter (p50 < 100 ms) survives by 2.2%, p95 is already over, headroom zero. Person-scoped evaluation pre-committed for Alpha (ledger row added) |
| H5 | The first real deployment knob (exporter selection, Rule 05) lands with `figment` + `tracing-subscriber` in muster-server; libraries stay subscriber-free | any subscriber/exporter dep entering orrery or muster-sdk | **confirmed** — `ServerConfig{bind, exporter}` (defaults→`MUSTER_*` env→`ORRERY_OTEL_EXPORTER` dev spelling); subscriber installed in muster-server only |

## Acceptance criteria (slice 2, pre-committed)

| Criterion | Threshold | Actual | Verdict |
|---|---|---|---|
| Gates | nextest workspace green; clippy -D warnings; fmt; doc-check; check-seam (grep fallback); check-scope; check-xrefs (ADR-0025 dangler resolves) | 72/72 tests; all listed gates green (this host, 2026-08-02) | pass |
| Prototype e2e | `e2e_` member flow: browse → select(+priority) → conflict visible → deselect → conflict resolved → my-schedule provenance intact; service-level | green — service-level AND repeated over HTTP (tower oneshot) | pass |
| SetPriority | service call wired to the existing command, covered by e2e | `set_priority` (member spelling: `Actor::Member`, non-binding); e2e-covered | pass |
| ADR-0025 | written from web-verified evidence; QUESTION-0015 closed with pointer; crates exist per its outcome | ADR-0025 accepted; question closed; muster-types/-server/-ui in the workspace | pass |
| Boundary | muster-source greps empty; `cargo tree -p muster-sdk` free of UI/server deps; orrery seam grep clean | all empty/clean (incl. tests dirs) | pass |
| Interactive latency | `select()` measured at ~10³ persons scale; number + method recorded in Results (pass/fail per H4, not tuned first) | p50 97.8 ms / p95 102.4 ms; method in `measure_select.rs`; not tuned | pass (measurement made; H4 verdict above) |
| `just muster::e2e` | fixed (review MO-10) and exercised | recipe fixed; runs 2 e2e tests | pass |
| Privacy | serialized server-facing payloads on a world WITH anchors carry no coordinate; `privacy_` test (extends the standing family early — cheap now, mandatory at Alpha) | **criterion as written is unsatisfiable** — see Results (Rule 01.2); wire-shape allowlist test green instead | pass, qualified |
| Artifacts (standing) | plain-language artifact at `plans/muster/artifacts/phase-6-prototype-*.md` | `phase-6-prototype-member-flow.md` | pass |

## Plan (slice 2)

1. ADR-0025 + QUESTION-0015 close (decision from the web-verified research,
   counter-considerations recorded).
2. Engine: `RemoveAttendance` command (repo + MemoryRepo + receipt), the
   `incremental::refresh_after` audit for the new kind (review CR-1
   fragility note), tests.
3. Service: `events(window)` browse, `set_priority`, `deselect`; extend
   `my_schedule` only if the flow demands it.
4. Crates per ADR-0025: `muster-types` (wire DTOs), `muster-server` (axum
   router over `MusterService`, figment config, subscriber install),
   `muster-ui` (compiling dioxus skeleton — the UI *content* is Alpha
   scope; the *structure* is this slice's deliverable). Dependency lines
   recorded below per Rule 06.
5. Measurement (H4), e2e + privacy tests, gates.
6. Results (refutations first), artifact, PLAN row updates, merge
   `--no-ff`, NEXT-SESSION rewritten.

**New dependencies (Rule 06 — muster-family only):** `axum`, `tokio`,
`serde_json`, `figment`, `tracing-subscriber` (muster-server);
`dioxus` (muster-ui only); `serde` (muster-types, already in baseline).
The full `opentelemetry-*` bridge is explicitly deferred to Alpha: the
Rule-05 knob (exporter selection) lands now via figment with `stdout`
(fmt-layer) and `none`; OTLP wiring arrives when a collector exists to
receive it. None of these enter orrery or muster-sdk (`check-seam`,
`check-scope`). *(As landed: also `tower` as a muster-server dev-dep for
oneshot router tests; `uuid` in muster-types/-server — already baseline.)*

## Results (slice 2 — Prototype)

**Refutations and qualifications first (Rule 01.3):**

1. **H4 refuted in spirit.** First-ever interactive measurement (review
   MO-2 found none existed): `select()` with its whole-window sweep costs
   **p50 97.8 ms / p95 102.4 ms** at 10³ persons / 5×10³ attends (this
   host, 2026-08-02; `crates/muster/tests/measure_select.rs`, run with
   `--no-capture`). The pre-committed letter (p50 < 100 ms) survives by
   2.2%; p95 is already over budget; growth headroom is zero — 10³
   persons is a *small* deployment. Recorded as refuted-in-spirit rather
   than confirmed-on-a-technicality: **person-scoped selection evaluation
   is pre-committed for the Alpha slice** (ledger row added). The test
   asserts only an order-of-magnitude sanity bound so the suite doesn't
   flake on a knife-edge (rationale in the test header).
2. **The privacy criterion was mis-specified as pre-committed
   (Rule 01.2).** It demanded a "world WITH anchors"; such a world cannot
   be built — `orrery::model::Anchors` has **no producing command and no
   repository storage** (found while writing the test; the plan review's
   F2/MO-4 finding, now also a Phase 6a ledger row). What shipped instead:
   `privacy_member_wire_payloads_carry_no_anchor_or_coordinate_shape` — a
   recursive JSON key allowlist over every member-flow response, plus
   `muster-types` DTOs that structurally cannot carry coordinates. The
   with-anchors fixture is owed the moment Phase 6a lands the producer.
3. One review input refuted during grounding (already in the findings
   doc): `SetPriority` was suspected missing engine-side — the command
   existed; only the service call was owed.

**Confirmed:** H1, H2, H3, H5 (table above). The **Muster Prototype stage
gate (ROADMAP: "browse, select, priority, my-schedule with provenance —
member flow complete") is met**, including the review-added deselect leg,
and the flow is proven twice: at the service seam and over real HTTP.

Landed: `RemoveAttendance` (first removal command; `refresh_after` audit
at the kind site — it touches no mirrored fact class) ·
`Repository::events_in(window)` (browse read; interval predicate stays
repo-side) · service `events`/`set_priority`/`deselect` ·
`build_demo_world` extraction (PoC demo unchanged in behaviour) ·
`muster-types` (coordinate-free wire DTOs) · `muster-server` (axum REST,
figment config, subscriber install, typed error→status mapping) ·
`muster-ui` (dioxus 0.7 components over the shared DTOs; `web` renderer
feature-gated so host CI builds it; REST client + `dx` entrypoint are
Alpha scope) · `just muster::e2e` fixed · 66 → **72 tests**.

Consumer simulation both ways (the PoC's lesson): the CLI demo still
tells its three-act story, and the server was booted and driven with
curl — browse, select, schedule with provenance — before the tests were
trusted.

## Decisions produced (slice 2)

* **ADR-0025** (frontend structure) — QUESTION-0015 closed.
* Additive engine growth recorded per the trait-growth discipline:
  `Command::RemoveAttendance`, `Repository::events_in` (orrery/SPEC-04
  updated with dated notes).
