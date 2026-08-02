//! The persistence seam (Rule 00.1, ADR-0021, ADR-0023).
//!
//! No concrete datastore type appears in this crate's public API — that is
//! the entire reason ADR-0015 can stay open while implementation proceeds.
//! The trait is **synchronous** (ADR-0023): every surviving candidate, the
//! SQLite fallback, and `MemoryRepo` are sync; async callers bridge at their
//! own boundary.
//!
//! Cross-hop traversal predicates are excluded **by construction**: every
//! traversal-shaped method takes only a constant `at: Timestamp` filter.
//! There is deliberately no method accepting a caller-supplied per-hop
//! predicate or closure over traversal state — the restrictive candidate
//! intersection (Ladybug G051-class limits, Grafeo, agdb) cannot back one,
//! and the trait must not absorb capabilities only a permissive store has.

use crate::command::{Command, CommandReceipt};
use crate::error::Result;
use crate::interval::{Interval, Timestamp};
use crate::model::{
    Attends, Event, EventId, Expects, Group, GroupId, Held, Location, LocationId, MemberOf, Mode,
    Person, PersonId, SubgroupOf, TravelCost, Traverse, Violation, Within,
};

/// Bound on `group_ancestors` traversal depth (orrery/SPEC-02: depth 5;
/// observed real depth 3–4).
pub const MAX_GROUP_DEPTH: usize = 5;

pub trait Repository: Send + Sync {
    fn person(&self, id: PersonId) -> Result<Option<Person>>;

    fn event(&self, id: EventId) -> Result<Option<Event>>;

    fn group(&self, id: GroupId) -> Result<Option<Group>>;

    fn location(&self, id: LocationId) -> Result<Option<Location>>;

    /// The full containment edge set — small (locations number in the
    /// thousands) and needed whole by the containment-exclusivity detector.
    fn containment(&self) -> Result<Vec<Within>>;

    /// `attends` edges for one person whose window overlaps `window` —
    /// entity-partitioned before the interval predicate, like every Orrery
    /// query.
    fn attends_for(&self, id: PersonId, window: Interval) -> Result<Vec<Attends>>;

    /// `held` edges into one location whose window overlaps `window`.
    fn held_for(&self, id: LocationId, window: Interval) -> Result<Vec<Held>>;

    /// Memberships valid at `at`.
    fn memberships(&self, id: PersonId, at: Timestamp) -> Result<Vec<MemberOf>>;

    /// Strict ancestors of `id` reachable through `subgroup_of` edges valid
    /// at `at`, each hop filtered by the same constant instant, depth
    /// bounded by [`MAX_GROUP_DEPTH`]. Does not include `id` itself — Q1
    /// unions the direct groups (depth 0) with this set.
    fn group_ancestors(&self, id: GroupId, at: Timestamp) -> Result<Vec<GroupId>>;

    /// Expectations attached to any of `groups`, valid at `at`.
    fn expectations(&self, groups: &[GroupId], at: Timestamp) -> Result<Vec<Expects>>;

    /// Layer-2 point lookup for one mode: the closure cache when populated,
    /// falling back to a direct-edge scan (Phase-3 behaviour) when not.
    fn travel(&self, from: LocationId, to: LocationId, mode: &Mode) -> Result<Option<TravelCost>>;

    /// Best (minimum-duration) cost across all modes — what
    /// impossible-travel judges against: a person who could have driven is
    /// not accused because walking is slow.
    fn travel_best(&self, from: LocationId, to: LocationId) -> Result<Option<TravelCost>>;

    /// The full Layer-1 edge set — input to the closure computation.
    fn traverse_all(&self) -> Result<Vec<Traverse>>;

    // -- sweep and mirror support (small full-set reads; entity sets are
    // -- the bounded dimension, edges are the unbounded one) --

    fn persons(&self) -> Result<Vec<PersonId>>;
    fn locations(&self) -> Result<Vec<LocationId>>;
    fn events(&self) -> Result<Vec<Event>>;
    /// Events whose window overlaps `window` — the browse read. Additive
    /// trait growth (Phase 6 slice 2, recorded per the trait-growth
    /// discipline): the interval predicate belongs on the repo side of the
    /// seam, and stores may index it; callers must not re-filter.
    fn events_in(&self, window: Interval) -> Result<Vec<Event>>;
    fn attends_for_event(&self, id: EventId) -> Result<Vec<Attends>>;
    fn held_for_event(&self, id: EventId) -> Result<Vec<Held>>;
    fn open_violations(&self) -> Result<Vec<Violation>>;
    fn memberships_all(&self) -> Result<Vec<MemberOf>>;
    fn subgroups_all(&self) -> Result<Vec<SubgroupOf>>;
    fn expectations_all(&self) -> Result<Vec<Expects>>;

    /// The only write path (Rule 00.2).
    fn apply(&self, cmd: Command) -> Result<CommandReceipt>;
}

#[cfg(feature = "repo-memory")]
pub mod memory;
