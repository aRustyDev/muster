//! Batch orchestration (PRD muster-sdk Flow C): the scheduled maintenance
//! run, composing engine primitives in dependency order — refresh the
//! travel closure, recompute derivation digests, sweep violations — and
//! report the change set. The SDK orchestrates; every primitive is the
//! engine's (Rule 03).

use orrery::engine::Engine;
use orrery::interval::{Interval, Timestamp};
use orrery::repo::Repository;
use orrery::travel::{ClosureReport, ClosureScope};

use crate::notify::ChangeSet;
use crate::Result;

#[derive(Debug)]
pub struct BatchReport {
    pub closure: ClosureReport,
    pub changes: ChangeSet,
    pub sweep: orrery::engine::SweepReport,
}

/// One maintenance pass. `at` stamps everything (the engine reads no
/// clock); `window` bounds the sweep.
pub fn run<R: Repository>(
    engine: &mut Engine<R>,
    at: Timestamp,
    window: Interval,
) -> Result<BatchReport> {
    let _span = tracing::info_span!("sdk.batch").entered();
    let closure = engine.refresh_closure(ClosureScope::EventBearing, at)?;
    let changed = engine.refresh_digests(at)?;
    let sweep = engine.sweep(at, window)?;
    Ok(BatchReport {
        closure,
        changes: ChangeSet { persons: changed },
        sweep,
    })
}
