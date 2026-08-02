//! Change-set computation (muster-sdk FR-10): who needs to hear that
//! their derived schedule changed. Computed from the engine's persisted
//! digests; **delivery** is Muster's job, never the SDK's (Rule 03).

use orrery::model::PersonId;

/// The people whose derived schedules changed in a batch pass — exactly
/// the set `Engine::refresh_digests` reported, in repository order.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ChangeSet {
    pub persons: Vec<PersonId>,
}

impl ChangeSet {
    pub fn is_empty(&self) -> bool {
        self.persons.is_empty()
    }
}
