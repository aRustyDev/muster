//! Muster-SDK — search and orchestration over Orrery's contract.
//!
//! Consumes `is_feasible` / `score`; never redefines them (Rule 03,
//! ADR-0013). The SDK owns request/response shapes and strategy; every
//! domain fact and every violation definition stays Orrery's. Sync
//! throughout (inherits ADR-0023).

pub mod assign;
pub mod objective;
pub mod suggest;

use orrery::interval::Interval;
use orrery::model::{EventId, LocationId, Violation};

pub use objective::{Breakdown, Objective, Term};
pub use suggest::suggest_room_schedule;

pub type Result<T> = std::result::Result<T, SdkError>;

#[derive(Debug, thiserror::Error)]
pub enum SdkError {
    #[error("engine error: {0}")]
    Engine(#[from] orrery::OrreryError),
    #[error("invalid request: {reason}")]
    InvalidRequest { reason: String },
}

/// An event needing a room; start time is fixed (the provably-optimal
/// greedy case — interval graph colouring).
#[derive(Debug, Clone, PartialEq)]
pub struct RoomRequest {
    pub event: EventId,
    pub window: Interval,
    pub expected_size: Option<u32>,
}

/// A candidate room.
#[derive(Debug, Clone, PartialEq)]
pub struct RoomOption {
    pub location: LocationId,
    pub capacity: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Placement {
    pub event: EventId,
    pub location: LocationId,
}

/// PRD Flow A's return value: assignment + violations + objective
/// breakdown. Unassignable events are surfaced, never silently dropped.
#[derive(Debug)]
pub struct Suggestion {
    pub placements: Vec<Placement>,
    pub unassigned: Vec<EventId>,
    /// Verbatim from `engine.is_feasible` — the SDK mints no violations.
    pub violations: Vec<Violation>,
    pub breakdown: Breakdown,
}
