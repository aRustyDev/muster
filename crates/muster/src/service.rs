//! The service layer (muster/SPEC-02): what any frontend will call.

use orrery::command::Command;
use orrery::engine::Engine;
use orrery::error::Result;
use orrery::interval::{Interval, Timestamp};
use orrery::model::{AttendanceSource, EntityRef, EventId, GroupId, PersonId, Violation};
use orrery::repo::Repository;

pub struct MusterService<R: Repository> {
    engine: Engine<R>,
}

#[derive(Debug)]
pub struct SelectionOutcome {
    pub seq: u64,
    /// Open violations touching this member after the selection — shown
    /// immediately (PRD Flow A: "see conflicts immediately").
    pub conflicts: Vec<Violation>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EntrySource {
    /// The member chose this themselves.
    SelfSelected,
    /// A coordinator placed it.
    Coordinator,
    /// Derived from a group expectation — no attendance row exists; the
    /// entry names where it came from (provenance, PRD FR-6).
    DerivedFromGroup { group: GroupId, group_name: String },
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScheduleEntry {
    pub event: EventId,
    pub event_name: String,
    pub window: Interval,
    pub source: EntrySource,
    /// True when any open violation involves both this member and this
    /// event — read from engine records, never recomputed here.
    pub flagged: bool,
}

#[derive(Debug, Default)]
pub struct ScheduleView {
    pub entries: Vec<ScheduleEntry>,
}

impl<R: Repository> MusterService<R> {
    pub fn new(engine: Engine<R>) -> Self {
        MusterService { engine }
    }

    /// Escape hatch for admin/coordinator setup until those surfaces get
    /// first-class service calls (Alpha).
    pub fn engine_mut(&mut self) -> &mut Engine<R> {
        &mut self.engine
    }

    /// PRD Flow A: select → the engine records it → conflicts come back
    /// immediately, from violation records.
    pub fn select(
        &mut self,
        person: PersonId,
        event: EventId,
        priority: Option<f32>,
        at: Timestamp,
        window: Interval,
    ) -> Result<SelectionOutcome> {
        let receipt = self.engine.apply(Command::AddAttendance {
            person,
            event,
            priority,
        })?;
        self.engine.sweep(at, window)?;
        let conflicts = self.violations_touching(person)?;
        Ok(SelectionOutcome {
            seq: receipt.seq,
            conflicts,
        })
    }

    /// The member's effective schedule: explicit attendance unioned with
    /// derived expectations, each entry carrying provenance and a flag
    /// sourced from open violation records.
    pub fn my_schedule(
        &self,
        person: PersonId,
        window: Interval,
        at: Timestamp,
    ) -> Result<ScheduleView> {
        let repo = self.engine.repo();
        let sched = orrery::derive::effective_schedule(repo, person, window, at)?;
        let open = self.violations_touching(person)?;
        let flagged = |event: EventId| {
            open.iter()
                .any(|v| v.subjects.contains(&EntityRef::Event(event)))
        };
        let event_name = |event: EventId| -> Result<String> {
            Ok(repo
                .event(event)?
                .map(|e| e.name)
                .unwrap_or_else(|| "(unknown event)".into()))
        };

        let mut entries = Vec::new();
        for a in &sched.explicit {
            entries.push(ScheduleEntry {
                event: a.event,
                event_name: event_name(a.event)?,
                window: a.during,
                source: match a.source {
                    AttendanceSource::SelfSelected => EntrySource::SelfSelected,
                    AttendanceSource::Coordinator(_) => EntrySource::Coordinator,
                    AttendanceSource::Group(g) => EntrySource::DerivedFromGroup {
                        group: g,
                        group_name: self.group_name(g)?,
                    },
                },
                flagged: flagged(a.event),
            });
        }
        for d in &sched.derived {
            // Display the EVENT's window: a derived edge's own `during` is
            // the expectation's validity window (ADR-0003), which is not
            // when the event happens. Caught by running the PoC demo —
            // the Evening Social sorted before breakfast.
            let window = repo.event(d.event)?.map(|e| e.window).unwrap_or(d.during);
            entries.push(ScheduleEntry {
                event: d.event,
                event_name: event_name(d.event)?,
                window,
                source: EntrySource::DerivedFromGroup {
                    group: d.source_group,
                    group_name: self.group_name(d.source_group)?,
                },
                flagged: flagged(d.event),
            });
        }
        entries.sort_by_key(|e| (e.window.start(), e.event));
        Ok(ScheduleView { entries })
    }

    fn group_name(&self, g: GroupId) -> Result<String> {
        Ok(self
            .engine
            .repo()
            .group(g)?
            .map(|g| g.name)
            .unwrap_or_else(|| "(unknown group)".into()))
    }

    fn violations_touching(&self, person: PersonId) -> Result<Vec<Violation>> {
        Ok(self
            .engine
            .repo()
            .open_violations()?
            .into_iter()
            .filter(|v| v.subjects.contains(&EntityRef::Person(person)))
            .collect())
    }
}
