<!-- Imported from design thread 2026-08-01. Fable: review and improve. -->

# SPEC 04 — Orrery API surface (draft)

Illustrative, not final. Fable should refine during Phase 2.

```rust
// ---- core contract
pub trait FeasibilityOracle {
    fn is_feasible(&self, a: &Assignment) -> Vec<Violation>;
    fn score(&self, a: &Assignment) -> f64;
}

// ---- persistence seam (ADR-0015 reversibility)
pub trait Repository: Send + Sync {
    fn person(&self, id: PersonId) -> Result<Option<Person>>;
    fn attends_for(&self, id: PersonId, w: Interval) -> Result<Vec<Attends>>;
    fn held_for(&self, id: LocationId, w: Interval) -> Result<Vec<Held>>;
    fn memberships(&self, id: PersonId, at: Timestamp) -> Result<Vec<MemberOf>>;
    fn group_ancestors(&self, id: GroupId, at: Timestamp) -> Result<Vec<GroupId>>;
    fn expectations(&self, g: &[GroupId], at: Timestamp) -> Result<Vec<Expects>>;
    fn travel(&self, from: LocationId, to: LocationId, mode: Mode)
        -> Result<Option<TravelCost>>;
    fn apply(&self, cmd: Command) -> Result<CommandReceipt>;
}

// ---- the single mutation chokepoint (ADR-0016)
pub enum Command {
    AddAttendance { person: PersonId, event: EventId, priority: Option<f32> },
    SetPriority   { person: PersonId, event: EventId, by: Actor, binding: bool,
                    value: f32 },
    AddMembership { person: PersonId, group: GroupId, during: Interval,
                    role: Role },
    AddExpectation{ group: GroupId, event: EventId, obligation: Obligation,
                    default_priority: f32, during: Interval, cascades: bool },
    HoldLocation  { location: LocationId, event: EventId, during: Interval,
                    overflow_for: Option<LocationId> },
    WaiveViolation{ id: ViolationId, by: Actor, reason: String },
    // ...
}

// ---- derivation (salsa-backed)
pub trait Derive {
    fn derived_attends(&self, p: PersonId, w: Interval) -> Vec<DerivedAttends>;
    fn effective_priority(&self, p: PersonId, e: EventId) -> f32;
    fn digest(&self, p: PersonId) -> Digest;
}

// ---- travel
pub trait Travel {
    fn feasible(&self, p: PersonId, a: &Placed, b: &Placed) -> Feasibility;
    fn refresh_closure(&self, scope: ClosureScope) -> Result<ClosureReport>;
}

// ---- analytics
pub trait Analytics {
    fn engagement(&self, g: GroupId, w: Interval) -> Vec<EngagementRow>;
    fn capacity_pressure(&self, w: Interval) -> Vec<CapacityRow>;
    fn divergence(&self, g: GroupId, w: Interval) -> Vec<DivergenceRow>;
    fn co_attendance_2hop(&self, p: PersonId, w: Interval) -> Vec<PersonId>;
}
```

## Design notes

* `feasible` takes `person` **now** and ignores it until ADR-0017 lands.
* Caches key on `(profile_id, e1, e2)`, everyone sharing `default` initially.
* `Command` is an enum, not a set of methods, so the event log (ADR-0016 D) is
  a serialisation concern rather than a refactor.
