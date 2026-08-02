//! Violation detection (orrery/SPEC-02, ADR-0012).
//!
//! Each detector is a **pure function in its own module**: no repository
//! types in any signature, no ids minted, no clocks read. Detectors return
//! [`ViolationDraft`]s; the engine assigns `ViolationId` and `detected_at`
//! when persisting (slice 2), which keeps every detector deterministic and
//! property-testable against a brute-force oracle.
//!
//! Detection, not prevention, by default (Rule 00.4): `Prevent` is the same
//! detector run inside the write transaction — one implementation, two call
//! sites. The transactional call site lands with sweep orchestration
//! (slice 2).

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::model::{EntityRef, Severity, ViolationKind};

pub mod capacity_exceeded;
pub mod containment_exclusivity;
pub mod expired_membership_effect;
pub mod impossible_travel;
pub mod location_exclusivity;
pub mod orphan_event;
pub mod time_conflict;

/// What a detector emits: everything a `Violation` needs except identity
/// and detection time, which the persisting engine assigns.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ViolationDraft {
    pub kind: ViolationKind,
    pub severity: Severity,
    pub subjects: Vec<EntityRef>,
}

/// Per-kind policy toggle (SPEC-02). `Prevent` aborts the writing
/// transaction on a non-empty detector result — same detector, second call
/// site.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Policy {
    Off,
    #[default]
    Detect,
    Warn,
    Prevent,
}

/// Policy per violation kind; unset kinds default to `Detect`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PolicyMap {
    overrides: HashMap<ViolationKind, Policy>,
}

impl PolicyMap {
    pub fn set(&mut self, kind: ViolationKind, policy: Policy) {
        self.overrides.insert(kind, policy);
    }

    pub fn policy(&self, kind: ViolationKind) -> Policy {
        self.overrides.get(&kind).copied().unwrap_or_default()
    }
}

/// Severity defaults, recorded in phases/03-engine-core.md as pre-committed
/// design decisions pending product input. Impossible-travel severity is
/// provenance-dependent (SPEC-03: conservative on estimates) and set in its
/// module.
pub const SEVERITY_TIME_CONFLICT: Severity = Severity::Hard;
pub const SEVERITY_LOCATION_EXCLUSIVITY: Severity = Severity::Hard;
pub const SEVERITY_CONTAINMENT_EXCLUSIVITY: Severity = Severity::Hard;
pub const SEVERITY_CAPACITY_EXCEEDED: Severity = Severity::Warning;
pub const SEVERITY_ORPHAN_EVENT: Severity = Severity::Info;
pub const SEVERITY_EXPIRED_MEMBERSHIP: Severity = Severity::Warning;
