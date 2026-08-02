//! Muster wire DTOs (ADR-0025): the single source of the REST/JSON
//! contract, shared by `muster-server` and `muster-ui`.
//!
//! Deliberate shape constraints:
//! * **No engine types** — ids are bare `Uuid`s, instants are microsecond
//!   `i64`s. The UI never links orrery; the server maps at its boundary.
//! * **No location coordinates, no anchor-shaped fields, anywhere.** The
//!   privacy boundary (Rule 00.6 / Rule 09) is enforced structurally: a
//!   wire type that cannot carry a coordinate cannot leak one. Room
//!   *names* for browse are public event locations, not personal anchors.
//! * Conflicts carry kind/severity strings and the touched **event** ids
//!   only — a member's wire payload names no other member.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// One browsable event (PRD Flow A) — time and room, never coordinates.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventDto {
    pub id: Uuid,
    pub name: String,
    pub start_us: i64,
    pub end_us: i64,
    pub room: Option<String>,
}

/// Why an entry is on the schedule and who put it there (PRD FR-6).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ProvenanceDto {
    SelfSelected,
    Coordinator,
    Group { name: String },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScheduleEntryDto {
    pub event: Uuid,
    pub name: String,
    pub start_us: i64,
    pub end_us: i64,
    pub provenance: ProvenanceDto,
    /// Read from open violation records — never recomputed on the wire.
    pub flagged: bool,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ScheduleDto {
    pub entries: Vec<ScheduleEntryDto>,
}

/// A member-visible conflict: what kind, how bad, which events. No
/// subjects beyond events — other persons' involvement stays server-side.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConflictDto {
    pub kind: String,
    pub severity: String,
    pub events: Vec<Uuid>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelectionOutcomeDto {
    pub seq: u64,
    pub conflicts: Vec<ConflictDto>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelectRequest {
    pub person: Uuid,
    pub event: Uuid,
    pub priority: Option<f32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeselectRequest {
    pub person: Uuid,
    pub event: Uuid,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PriorityRequest {
    pub person: Uuid,
    pub event: Uuid,
    pub value: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SeqDto {
    pub seq: u64,
}
