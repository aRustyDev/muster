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
    Attends, Expects, GroupId, Held, LocationId, MemberOf, Mode, Person, PersonId, TravelCost,
};

/// Bound on `group_ancestors` traversal depth (orrery/SPEC-02: depth 5;
/// observed real depth 3–4).
pub const MAX_GROUP_DEPTH: usize = 5;

pub trait Repository: Send + Sync {
    fn person(&self, id: PersonId) -> Result<Option<Person>>;

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

    /// Layer-2-style point lookup: cheapest traverse cost `from → to` for a
    /// mode, if an edge exists.
    fn travel(&self, from: LocationId, to: LocationId, mode: &Mode) -> Result<Option<TravelCost>>;

    /// The only write path (Rule 00.2).
    fn apply(&self, cmd: Command) -> Result<CommandReceipt>;
}

#[cfg(feature = "repo-memory")]
pub mod memory;
