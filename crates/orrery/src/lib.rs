//! Orrery — a spatiotemporal feasibility engine.
//!
//! Given a proposed assignment of people to events at locations over time,
//! Orrery returns the ways that assignment is impossible, and a score for
//! how good it is. It does not schedule; it decides whether a schedule is
//! possible.
//!
//! Architectural invariants (see `orrery/SPEC-00` and Rule 00):
//!
//! 1. Every relation carries a validity window (`during`); every conflict
//!    check is the same interval-overlap predicate.
//! 2. Every query is entity-partitioned before its interval predicate
//!    applies.
//! 3. Derived semantics, cached physically (Phase 3).
//! 4. All persistence behind [`repo::Repository`]; no concrete datastore
//!    type in this crate's public API.
//! 5. All mutations through [`command::Command`].

pub mod command;
pub mod error;
pub mod interval;
pub mod model;
pub mod repo;

pub use command::{Command, CommandReceipt};
pub use error::{OrreryError, Result};
pub use interval::{AllenRelation, Interval, Timestamp};
pub use repo::Repository;
