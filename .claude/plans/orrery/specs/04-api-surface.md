<!-- Imported from design thread 2026-08-01 as an illustrative draft.
     Rewritten 2026-08-02 (Phase 3 slice 2) to describe the LANDED surface;
     the draft's shapes survived with additive refinements only, recorded in
     phases/02-workspace.md H3 and phases/03-engine-core.md. -->

# SPEC 04 — Orrery API surface (as landed, Phase 3)

## Core contract (`engine`)

```rust
pub trait FeasibilityOracle {
    fn is_feasible(&self, a: &Assignment) -> Vec<Violation>;
    fn score(&self, a: &Assignment) -> f64;   // −Σ severity weights (Hard 100 / Warning 10 / Info 1)
}

/// Proposed overlay, evaluated read-only. `at`/`window` are caller-supplied:
/// the engine reads no clock, anywhere.
pub struct Assignment { pub attends: Vec<Attends>, pub held: Vec<Held>,
                        pub at: Timestamp, pub window: Interval }
```

Implemented by `Engine<R: Repository>`, which also owns:

```rust
impl<R: Repository> Engine<R> {
    fn apply(&mut self, cmd: Command) -> Result<CommandReceipt>;   // Prevent gate + repo + salsa mirror
    fn digest(&mut self, person: PersonId, at: Timestamp) -> [u8; 32];      // salsa-memoized
    fn refresh_digests(&mut self, at: Timestamp) -> Result<Vec<PersonId>>;  // persists; returns changed set
    fn sweep(&mut self, at: Timestamp, window: Interval) -> Result<SweepReport>;
    // -- Phase 6a additions (dated notes below) --
    fn preview_digests(&mut self, cmd: &Command, at: Timestamp) -> Result<Vec<PersonId>>;
    fn first_event_feasibility(&self, person: PersonId, window: Interval,
                               depart_not_before: Timestamp) -> Result<Feasibility>;
}
```

*(Added 2026-08-02, Phase 6a: `preview_digests` is the non-persisting
digest dry-run — the change set `apply` + `refresh_digests` WOULD produce,
supported for exactly the three mirrored command kinds, typed
`PreviewUnsupported` otherwise; honesty property-tested against the real
post-commit path, mirror overlaid-and-restored per review CR-1.
`first_event_feasibility` is the ADR-0014 anchor→first-event consult:
`travel::Feasibility` verdicts only — durations and provenance, never an
anchor or location; deliberately not wired into `sweep()` until a
`depart_not_before` policy source exists.)*

Scope note: `is_feasible` covers time-conflict and location/containment
exclusivity; travel feasibility joins in Phase 4. Slice-2 scope decision,
phases/03-engine-core.md.

## Persistence seam (`repo`, sync — ADR-0023)

No concrete datastore type anywhere in this crate's API (Rule 00.1).
Traversal methods take only constant `at: Timestamp` filters — cross-hop
predicates are unrepresentable by construction (Phase-1a constraint
intersection).

```rust
pub trait Repository: Send + Sync {
    // point reads
    fn person(&self, id: PersonId) -> Result<Option<Person>>;
    fn event(&self, id: EventId) -> Result<Option<Event>>;
    fn location(&self, id: LocationId) -> Result<Option<Location>>;
    // entity-partitioned interval reads (partition first, interval second)
    fn attends_for(&self, id: PersonId, window: Interval) -> Result<Vec<Attends>>;
    fn held_for(&self, id: LocationId, window: Interval) -> Result<Vec<Held>>;
    fn memberships(&self, id: PersonId, at: Timestamp) -> Result<Vec<MemberOf>>;
    fn anchors_for(&self, id: PersonId, at: Timestamp) -> Result<Vec<Anchors>>;    // Phase 6a, ADR-0014
    fn group_ancestors(&self, id: GroupId, at: Timestamp) -> Result<Vec<GroupId>>; // strict, depth ≤ 5
    fn expectations(&self, groups: &[GroupId], at: Timestamp) -> Result<Vec<Expects>>;
    fn travel(&self, from: LocationId, to: LocationId, mode: &Mode) -> Result<Option<TravelCost>>;
    fn containment(&self) -> Result<Vec<Within>>;
    // sweep/mirror support (bounded entity sets)
    fn persons(&self) -> Result<Vec<PersonId>>;
    fn locations(&self) -> Result<Vec<LocationId>>;
    fn events(&self) -> Result<Vec<Event>>;
    fn attends_for_event(&self, id: EventId) -> Result<Vec<Attends>>;
    fn open_violations(&self) -> Result<Vec<Violation>>;
    fn memberships_all(&self) -> Result<Vec<MemberOf>>;
    fn subgroups_all(&self) -> Result<Vec<SubgroupOf>>;
    fn expectations_all(&self) -> Result<Vec<Expects>>;
    // the only write path (Rule 00.2)
    fn apply(&self, cmd: Command) -> Result<CommandReceipt>;
}
```

## The mutation chokepoint (`command`)

`Command` is an enum, not a method set, so the event log (ADR-0016 D) is a
serialisation concern rather than a refactor. Variants as landed:

`UpsertPerson/Group/Event/Location` · `AddAttendance` ·
`RemoveAttendance` *(added Phase 6 slice 2, plan-review CR-6 — the first
removal variant: member deselect; touches no mirrored fact class, audited
against `incremental::refresh_after` at introduction; broader retraction
is an Alpha pre-commitment item)* · `SetPriority` ·
`AddMembership` · `AddSubgroup` · `AddExpectation` (carries `by: Actor` for
provenance) · `HoldLocation` (carries `capacity_override`) ·
`AddContainment` (tier-validated) · `AddTraversePair` (one call site writes
both directed rows; sibling rule validated, `sibling_override` marker) ·
`WaiveViolation` · `RecordViolation` · `ResolveViolation` ·
`SetDerivedDigest` · `ReplaceClosure` ·
`AddAnchor` *(added Phase 6a — the ADR-0014 producer; Structure-tier
target enforced at the command layer; touches no mirrored fact class,
audited against `incremental::refresh_after` at introduction like
`RemoveAttendance`)*.

*(Read surface note, same date: `Repository::events_in(window)` added —
the browse read; entity-set filter with the interval predicate on the
repo side of the seam, per the trait-growth discipline.)*

`CommandReceipt { seq: u64 }` — the future event log's sequence number.

## Derivation (`derive` + `incremental`)

```rust
pub fn expand(repo: &dyn Repository, person: PersonId, at: Timestamp)
    -> Result<Vec<DerivedAttends>>;                       // Q1, cold path
pub fn effective_schedule(repo, person, window, at)
    -> Result<EffectiveSchedule>;                          // explicit shadows derived
pub fn derived_id(person, event, group, expectation_start) -> DerivedId;  // blake3
pub fn digest_of_ids(ids: &[DerivedId]) -> [u8; 32];       // ADR-0016 B
```

`incremental` is the salsa-backed hot path: a `World` input mirroring the
three fact classes the chain reads, tracked layers
`direct_groups → reach → derived_ids → digest`, float-free by construction.
Early cutoff at the extraction layer bounds blast radius; the cold and
incremental paths are fuzz-compared (SPEC-05 incremental correctness).

## Analytics (`analytics`) — added 2026-08-02, Phase 6a

Pure reads over the repository seam (SPEC-02 §Analytics; the ROADMAP
boundary matrix's orrery-owned analytics, consumed at Muster Beta):

```rust
pub fn engagement(repo, person, window, at) -> Result<f64>;          // Σ effective priority, effective schedule
pub fn engagement_by_group(repo, group, window, at) -> Result<Vec<(PersonId, f64)>>;
pub fn capacity_pressure(repo, window, interest_threshold: f32)
    -> Result<Vec<CapacityPressure>>;   // threshold caller-supplied: the ADR-0018 hook
pub fn divergence(repo, group, window, at) -> Result<DivergenceSummary>;
pub fn co_attendance(repo, person, window) -> Result<Vec<PersonId>>;  // ADR-0020 bounded 2-hop
```

Group roll-ups resolve members via `memberships_all` — batch/reporting
surfaces, not interactive ones. Each function carries a brute-force
oracle property test.

## Detection (`detect`)

Seven pure detectors, one module each, returning `ViolationDraft` (no ids,
no clocks — the engine assigns identity and `detected_at` when persisting).
`PolicyMap` per kind: `Off / Detect / Warn / Prevent`; `Prevent` is the same
detector called in the engine's apply gate.

## Design invariants carried forward

* `feasible(person, e1, e2)` lands with Phase 4 travel, `person` ignored
  until ADR-0017. Two caches, two keys *(distinguished 2026-08-02, plan
  review MO-6 — the conflation was drifting against Rule 00.5)*: the
  **Layer-2 travel cache** keys on `(profile_id, from, to)` (location
  pair; as built, profile-less via `travel_best`); any future
  **feasibility-verdict cache** keys on `(profile_id, e1, e2)` (event
  pair) per Rule 00.5 / ADR-0017 — keying verdicts any other way is an
  ADR, not a comment.
* Every evaluation instant is caller-supplied — determinism and replay.
* No `unwrap`/`expect` outside tests (Rule 04); constraint violations are
  typed errors naming the constraint (Rule 00b).
