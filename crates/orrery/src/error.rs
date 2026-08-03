//! Typed errors (Rule 04: libraries never use `anyhow`).

use crate::model::EntityRef;

pub type Result<T> = std::result::Result<T, OrreryError>;

#[derive(Debug, thiserror::Error)]
pub enum OrreryError {
    /// A repository constraint was violated. `constraint` names it
    /// (Rule 00b: enforcement must be executable and self-describing).
    #[error("repository constraint violated: {constraint}")]
    ConstraintViolated { constraint: &'static str },

    #[error("not found: {0}")]
    NotFound(EntityRef),

    /// An interval could not be constructed. Carries no location or
    /// coordinate data (Rule 09).
    #[error("invalid interval: {reason}")]
    InvalidInterval { reason: &'static str },

    #[error("command rejected: {reason}")]
    CommandRejected { reason: String },

    /// A digest preview was requested for a command kind that does not
    /// touch the mirrored derivation inputs (memberships / subgroups /
    /// expectations). A typed refusal beats a possibly-lying change set:
    /// e.g. an entity upsert changes the person set itself, which an
    /// overlay cannot represent honestly (Phase 6a).
    #[error(
        "no digest preview for command kind `{kind}`: only membership, \
         subgroup, and expectation commands have one"
    )]
    PreviewUnsupported { kind: &'static str },
}

impl OrreryError {
    pub(crate) fn constraint(name: &'static str) -> Self {
        OrreryError::ConstraintViolated { constraint: name }
    }
}
