//! The single mutation chokepoint (Rule 00.2, ADR-0016).
//!
//! A `Command` enum, not a method set: the event-log upgrade (ADR-0016 D)
//! then becomes a serialisation concern at one insertion point rather than a
//! refactor. Every mutation — including entity creation and test seeding —
//! goes through [`Command`]; a repository exposes no other write path.
//!
//! The variant set extends the SPEC-04 draft additively (entity upserts,
//! subgroup/traverse edges, `by` on expectations for provenance) — recorded
//! in phases/02-workspace.md.

use serde::{Deserialize, Serialize};

use crate::interval::{Interval, Timestamp};
use crate::model::{
    Actor, ClosureEntry, Event, EventId, Group, GroupId, Location, LocationId, Obligation, Person,
    PersonId, Role, Traverse, Violation, ViolationId,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Command {
    // -- entity upserts (idempotent on id) --
    UpsertPerson(Person),
    UpsertGroup(Group),
    UpsertEvent(Event),
    UpsertLocation(Location),

    // -- relations --
    AddAttendance {
        person: PersonId,
        event: EventId,
        /// Sets `priority_person`; `priority_group` seeds from the group
        /// default when the edge is derived (Phase 3).
        priority: Option<f32>,
    },
    SetPriority {
        person: PersonId,
        event: EventId,
        by: Actor,
        /// `true` = coordinator override, `false` = suggestion (ADR-0005).
        binding: bool,
        value: f32,
    },
    AddMembership {
        person: PersonId,
        group: GroupId,
        during: Interval,
        role: Role,
    },
    AddSubgroup {
        child: GroupId,
        parent: GroupId,
        during: Interval,
    },
    AddExpectation {
        group: GroupId,
        event: EventId,
        obligation: Obligation,
        default_priority: f32,
        during: Interval,
        cascades: bool,
        by: Actor,
    },
    HoldLocation {
        location: LocationId,
        event: EventId,
        during: Interval,
        overflow_for: Option<LocationId>,
        /// ADR-0018: capacity constraint is per-(location, event).
        capacity_override: Option<u32>,
    },
    /// Containment edge; tier-ascending only, validated against the tier
    /// module (ADR-0009) — illegal edges are rejected, not stored.
    AddContainment {
        child: LocationId,
        parent: LocationId,
    },
    /// Writes both directed rows from this single call site so symmetric
    /// values cannot drift (ADR-0008). Sibling-tier rule validated unless
    /// the edge carries `sibling_override` or touches a portal.
    AddTraversePair(Traverse),
    WaiveViolation {
        id: ViolationId,
        by: Actor,
        reason: String,
    },
    /// Persist a violation record. The ENGINE mints the id and detected_at
    /// (from the sweep instant — no clock reads); detectors only draft.
    RecordViolation(Violation),
    /// Close an open violation whose cause is no longer detected.
    ResolveViolation {
        id: ViolationId,
        at: Timestamp,
    },
    /// Persist a person's derivation digest (ADR-0016 B).
    SetDerivedDigest {
        person: PersonId,
        digest: [u8; 32],
        at: Timestamp,
    },
    /// Atomically replace the Layer-2 travel closure (ADR-0006:
    /// batch-recomputed, read-optimised point lookups).
    ReplaceClosure {
        entries: Vec<ClosureEntry>,
    },
}

impl Command {
    /// Stable variant name — span attribute (Rule 05) and, later, the event
    /// log's discriminator.
    pub fn kind(&self) -> &'static str {
        match self {
            Command::UpsertPerson(_) => "upsert_person",
            Command::UpsertGroup(_) => "upsert_group",
            Command::UpsertEvent(_) => "upsert_event",
            Command::UpsertLocation(_) => "upsert_location",
            Command::AddAttendance { .. } => "add_attendance",
            Command::SetPriority { .. } => "set_priority",
            Command::AddMembership { .. } => "add_membership",
            Command::AddSubgroup { .. } => "add_subgroup",
            Command::AddExpectation { .. } => "add_expectation",
            Command::HoldLocation { .. } => "hold_location",
            Command::AddContainment { .. } => "add_containment",
            Command::AddTraversePair(_) => "add_traverse_pair",
            Command::WaiveViolation { .. } => "waive_violation",
            Command::RecordViolation(_) => "record_violation",
            Command::ResolveViolation { .. } => "resolve_violation",
            Command::SetDerivedDigest { .. } => "set_derived_digest",
            Command::ReplaceClosure { .. } => "replace_closure",
        }
    }
}

/// Proof a command was applied. `seq` is the repository's monotonic apply
/// counter — the future event log's sequence number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CommandReceipt {
    pub seq: u64,
}
