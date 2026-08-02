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

/// Ids and instants of the seeded demo world — what a test needs to drive
/// the member flow against it (Phase 6 slice 2: extracted from `run_demo`
/// so the Prototype e2e reuses the same world; extend, don't rewrite).
#[derive(Debug, Clone, Copy)]
pub struct DemoWorld {
    pub member: PersonId,
    pub cohort: GroupId,
    pub talk_rust: EventId,
    pub talk_systems: EventId,
    pub social: EventId,
    /// A whole-day evaluation window and a morning "now".
    pub day: Interval,
    pub now: Timestamp,
}

/// Seed the demo world: Ada, her cohort (with membership and the evening
/// social expectation), three events, two rooms with holds. No selections
/// — stories and tests drive those.
pub fn build_demo_world() -> Result<(MusterService<MemoryRepo>, DemoWorld)> {
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

    // The cohort coordinator expects the social; Ada is a member. (Part of
    // the seeded world since slice 2; the PoC applied these mid-story, but
    // nothing in the story depends on the ordering — the derived entry has
    // no attendance row either way.)
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

    Ok((
        svc,
        DemoWorld {
            member: ada,
            cohort,
            talk_rust,
            talk_systems,
            social,
            day: iv(0, 24),
            now: Timestamp(8 * HOUR),
        },
    ))
}

/// Build the demo world and walk the PoC story:
/// Ada self-selects two overlapping talks (conflict appears, from engine
/// records); her cohort's expectation of the evening social shows as a
/// derived entry with provenance — no attendance write.
pub fn run_demo() -> Result<DemoReport> {
    let (mut svc, w) = build_demo_world()?;
    let (ada, day, now) = (w.member, w.day, w.now);
    let mut lines = Vec::new();

    // -- Act 1: Ada self-selects two overlapping talks.
    svc.select(ada, w.talk_rust, Some(0.9), now, day)?;
    let outcome = svc.select(ada, w.talk_systems, Some(0.7), now, day)?;
    lines.push(format!(
        "Ada selected two talks; the system immediately shows {} problem(s).",
        outcome.conflicts.len()
    ));

    // -- Act 2 (since slice 2, seeded in the world): the expectation is
    // already in force — Act 3 shows it as provenance.

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
