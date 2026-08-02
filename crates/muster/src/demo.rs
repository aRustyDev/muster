//! The PoC story (ROADMAP: "one member self-selects; conflict appears —
//! conflict visible end to end"), runnable as `cargo run -p muster -- demo`
//! and asserted by the `e2e_` test on the returned report, not on stdout.

use orrery::command::Command;
use orrery::engine::Engine;
use orrery::error::Result;
use orrery::interval::{Interval, Timestamp};
use orrery::model::{
    Actor, Event, EventId, Group, GroupId, Location, LocationId, Obligation, Person, PersonId,
    Portal, Role, Tier,
};
use orrery::repo::memory::MemoryRepo;
use orrery::repo::Repository;

use crate::service::{EntrySource, MusterService, ScheduleEntry};

const HOUR: i64 = 3_600 * 1_000_000;

fn iv(h_start: i64, h_end: i64) -> Interval {
    Interval::new(Timestamp(h_start * HOUR), Timestamp(h_end * HOUR)).expect("demo windows valid")
}

#[derive(Debug)]
pub struct DemoReport {
    pub member: PersonId,
    pub conflicts_after_selection: usize,
    pub schedule: Vec<ScheduleEntry>,
    pub derived_entries: usize,
    pub lines: Vec<String>,
}

/// Build the demo world and walk the PoC story:
/// Ada self-selects two overlapping talks (conflict appears, from engine
/// records); her cohort's coordinator expects the evening social (a
/// derived entry with provenance appears — no attendance write).
pub fn run_demo() -> Result<DemoReport> {
    let repo = MemoryRepo::new();
    let ada = PersonId::new();
    let cohort = GroupId::new();
    let (talk_rust, talk_systems, social) = (EventId::new(), EventId::new(), EventId::new());
    let room = Location {
        id: LocationId::new(),
        name: "Room 101".into(),
        tier: Tier::Room,
        portal: Portal::None,
        capacity: Some(40),
        ext: Default::default(),
    };
    let room2 = Location {
        id: LocationId::new(),
        name: "Room 202".into(),
        tier: Tier::Room,
        portal: Portal::None,
        capacity: Some(40),
        ext: Default::default(),
    };
    let (room_id, room2_id) = (room.id, room2.id);

    repo.apply(Command::UpsertPerson(Person {
        id: ada,
        name: "Ada".into(),
        derived_digest: None,
        ext: Default::default(),
    }))?;
    repo.apply(Command::UpsertGroup(Group {
        id: cohort,
        name: "cohort-26".into(),
        default_priority: None,
        timezone: None,
        ext: Default::default(),
    }))?;
    for l in [room, room2] {
        repo.apply(Command::UpsertLocation(l))?;
    }
    for (id, name, s, e) in [
        (talk_rust, "Intro to Rust", 9, 10),
        (talk_systems, "Systems Workshop", 9, 11), // overlaps Intro to Rust
        (social, "Evening Social", 18, 19),
    ] {
        repo.apply(Command::UpsertEvent(Event {
            id,
            name: name.into(),
            window: iv(s, e),
            kind: "session".into(),
            timezone: Some("America/New_York".into()),
            ext: Default::default(),
        }))?;
    }
    for (e, l) in [
        (talk_rust, room_id),
        (talk_systems, room2_id),
        (social, room_id),
    ] {
        let during = repo.event(e)?.expect("seeded").window;
        repo.apply(Command::HoldLocation {
            location: l,
            event: e,
            during,
            overflow_for: None,
            capacity_override: None,
        })?;
    }

    let mut svc = MusterService::new(Engine::new(repo)?);
    let day = iv(0, 24);
    let now = Timestamp(8 * HOUR);
    let mut lines = Vec::new();

    // -- Act 1: Ada self-selects two overlapping talks.
    svc.select(ada, talk_rust, Some(0.9), now, day)?;
    let outcome = svc.select(ada, talk_systems, Some(0.7), now, day)?;
    lines.push(format!(
        "Ada selected two talks; the system immediately shows {} problem(s).",
        outcome.conflicts.len()
    ));

    // -- Act 2: the cohort coordinator expects the social; Ada is a member.
    svc.engine_mut().apply(Command::AddMembership {
        person: ada,
        group: cohort,
        during: iv(0, 24 * 365),
        role: Role::Member,
    })?;
    svc.engine_mut().apply(Command::AddExpectation {
        group: cohort,
        event: social,
        obligation: Obligation::Expected,
        default_priority: 0.5,
        during: iv(0, 24 * 365),
        cascades: true,
        by: Actor::System,
    })?;

    // -- Act 3: Ada's schedule — conflict flagged, provenance visible.
    let view = svc.my_schedule(ada, day, now)?;
    for entry in &view.entries {
        let src = match &entry.source {
            EntrySource::SelfSelected => "you picked this".to_string(),
            EntrySource::Coordinator => "a coordinator placed this".to_string(),
            EntrySource::DerivedFromGroup { group_name, .. } => {
                format!("expected via group '{group_name}'")
            }
        };
        let flag = if entry.flagged { "  ⚠ CONFLICT" } else { "" };
        lines.push(format!("  {:18} — {}{}", entry.event_name, src, flag));
    }

    let derived_entries = view
        .entries
        .iter()
        .filter(|e| matches!(e.source, EntrySource::DerivedFromGroup { .. }))
        .count();
    Ok(DemoReport {
        member: ada,
        conflicts_after_selection: outcome.conflicts.len(),
        schedule: view.entries,
        derived_entries,
        lines,
    })
}
