//! Entities and relations (orrery/SPEC-01).
//!
//! Two rules shape everything here: every identifier is a newtype (Rule 04 —
//! a bare u64/Uuid crossing a boundary is a transposed-argument bug waiting),
//! and **every relation carries `during`** (Rule 00.3 — a relation without a
//! validity window is a bug).

use std::collections::BTreeMap;
use std::fmt;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::interval::{Interval, Timestamp};

macro_rules! id_newtype {
    ($(#[$doc:meta])* $name:ident) => {
        $(#[$doc])*
        #[derive(
            Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
        )]
        pub struct $name(pub Uuid);

        impl $name {
            /// A fresh time-ordered identifier (UUIDv7 — ADR-0022: insert
            /// locality matters because this workload is entity-partitioned
            /// b-tree access).
            pub fn new() -> Self {
                Self(Uuid::now_v7())
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }
    };
}

id_newtype!(PersonId);
id_newtype!(GroupId);
id_newtype!(EventId);
id_newtype!(LocationId);
id_newtype!(ViolationId);

/// A typed reference to any entity — used in violation subjects and errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EntityRef {
    Person(PersonId),
    Group(GroupId),
    Event(EventId),
    Location(LocationId),
    Violation(ViolationId),
}

impl fmt::Display for EntityRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EntityRef::Person(id) => write!(f, "person {id}"),
            EntityRef::Group(id) => write!(f, "group {id}"),
            EntityRef::Event(id) => write!(f, "event {id}"),
            EntityRef::Location(id) => write!(f, "location {id}"),
            EntityRef::Violation(id) => write!(f, "violation {id}"),
        }
    }
}

/// Extension attributes — application data the engine stores but never
/// interprets.
pub type Ext = BTreeMap<String, String>;

// ---------------------------------------------------------------- entities

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Person {
    pub id: PersonId,
    pub name: String,
    #[serde(default)]
    pub ext: Ext,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Group {
    pub id: GroupId,
    pub name: String,
    /// Seed for `attends.priority_group` when an expectation doesn't set one.
    pub default_priority: Option<f32>,
    /// IANA zone name. Retained per the QUESTION-0014 leaning; interval
    /// algebra never reads it.
    pub timezone: Option<String>,
    #[serde(default)]
    pub ext: Ext,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Event {
    pub id: EventId,
    pub name: String,
    pub window: Interval,
    pub kind: String,
    #[serde(default)]
    pub ext: Ext,
}

/// Containment position (ADR-0009). `Floor` and `Region` are optional tiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Tier {
    Room,
    Floor,
    Structure,
    Campus,
    Region,
}

/// Routing role — orthogonal to tier (ADR-0010). A station is a Structure
/// that is a rail portal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Portal {
    None,
    Pedestrian,
    Vehicle,
    Rail,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Location {
    pub id: LocationId,
    pub name: String,
    pub tier: Tier,
    pub portal: Portal,
    pub capacity: Option<u32>,
    #[serde(default)]
    pub ext: Ext,
}

// --------------------------------------------------------------- violations

/// Detector kinds (orrery/SPEC-02). Detectors themselves land in Phase 3;
/// the record type is here because commands can waive them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ViolationKind {
    TimeConflict,
    LocationExclusivity,
    ContainmentExclusivity,
    ImpossibleTravel,
    CapacityExceeded,
    OrphanEvent,
    ExpiredMembershipEffect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Severity {
    Info,
    Warning,
    Hard,
}

/// First-class violation record with lifecycle (ADR-0012): the inbox is the
/// product. Subjects are entity references — never coordinates (Rule 09).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Violation {
    pub id: ViolationId,
    pub kind: ViolationKind,
    pub severity: Severity,
    pub subjects: Vec<EntityRef>,
    pub detected_at: Timestamp,
    pub resolved_at: Option<Timestamp>,
    pub acknowledged_by: Option<PersonId>,
    pub waiver_reason: Option<String>,
}

// ---------------------------------------------------------------- relations

/// Who performed a mutation — provenance for the priority stack and waivers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Actor {
    Member(PersonId),
    Coordinator(PersonId),
    System,
}

/// Where an `attends` edge came from (ADR-0004 provenance).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttendanceSource {
    SelfSelected,
    Group(GroupId),
    Coordinator(PersonId),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Attends {
    pub person: PersonId,
    pub event: EventId,
    /// Own window — join/leave may differ from the event's span.
    pub during: Interval,
    pub priority_group: f32,
    pub priority_person: Option<f32>,
    pub priority_coord: Option<f32>,
    pub coord_binding: bool,
    pub source: AttendanceSource,
    pub pinned: bool,
}

impl Attends {
    /// The effective priority — computed in exactly one place (ADR-0005).
    /// User input is never destroyed; a binding override changes the
    /// effective value without erasing what the person wanted.
    pub fn effective_priority(&self) -> f32 {
        if self.coord_binding {
            if let Some(pc) = self.priority_coord {
                return pc;
            }
        }
        self.priority_person
            .or(self.priority_coord)
            .unwrap_or(self.priority_group)
    }

    /// `|priority_coord − priority_person|` — the divergence analytic that
    /// exists only because the stack keeps components separate.
    pub fn divergence(&self) -> Option<f32> {
        Some((self.priority_coord? - self.priority_person?).abs())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Posture {
    OnSite,
    Remote,
    Hybrid,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Held {
    pub location: LocationId,
    pub event: EventId,
    /// Own window, independent of the event's (ADR-0011) — partial-duration
    /// overflow depends on this.
    pub during: Interval,
    pub posture: Posture,
    /// A location reference, not a boolean, so spillover chains are
    /// expressible (ADR-0011).
    pub overflow_for: Option<LocationId>,
    pub capacity_override: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Role {
    Member,
    Coordinator,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemberOf {
    pub person: PersonId,
    pub group: GroupId,
    pub during: Interval,
    /// Authorisation for coordinator override lives here (ADR-0002).
    pub role: Role,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubgroupOf {
    pub child: GroupId,
    pub parent: GroupId,
    pub during: Interval,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Obligation {
    Mandatory,
    Expected,
    Recommended,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Expects {
    pub group: GroupId,
    pub event: EventId,
    pub obligation: Obligation,
    pub default_priority: f32,
    /// Window the expectation applies over — **not** the event's window
    /// (ADR-0003): an expectation added late must not apply retroactively.
    pub during: Interval,
    pub cascades: bool,
    pub can_decline: bool,
    pub set_by: Actor,
    pub set_at: Timestamp,
}

/// Travel mode — an attribute, not a relation split (ADR-0007).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Mode(pub String);

/// Continuous travel: cost is a scalar duration, depart whenever. Directed;
/// stored as two rows written from one function so they cannot drift
/// (ADR-0008). `transit` (scheduled) is deliberately v2 (ADR-0007).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Traverse {
    pub from: LocationId,
    pub to: LocationId,
    pub mode: Mode,
    pub duration_typical_s: i64,
    pub duration_peak_s: Option<i64>,
    pub peak_window: Option<Interval>,
    pub distance_m: Option<f64>,
    pub provenance: TravelProvenance,
    pub computed_at: Timestamp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TravelProvenance {
    Measured,
    Estimated,
}

/// The answer a travel lookup returns (SPEC-04). Carries provenance so
/// detectors can be more conservative on estimates.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TravelCost {
    pub duration_s: i64,
    pub provenance: TravelProvenance,
}

/// Containment edge, tier-ascending only (ADR-0009). Tier legality is
/// checked at the command layer (relational-style enforcement: one module,
/// exhaustive tests — Phase 3).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Within {
    pub child: LocationId,
    pub parent: LocationId,
}

/// Personal origin (ADR-0014). **Never crosses the coordinator boundary**
/// (Rule 00.6 / Rule 09): feasibility verdicts leave the engine, anchors do
/// not — and this struct must never appear in span attributes or errors.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Anchors {
    pub person: PersonId,
    pub structure: LocationId,
    pub label: String,
    pub during: Interval,
    /// Day-of-week / condition selector; representation firms up with
    /// mobility profiles (ADR-0017).
    pub applies_when: Option<String>,
}
