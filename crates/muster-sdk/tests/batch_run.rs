//! Slice-2 H9: batch orchestration composes engine primitives — closure
//! refresh → digest recompute → sweep → change set — against
//! `Engine<MemoryRepo>`, with no new engine surface.

use muster_sdk::batch;
use orrery::command::Command;
use orrery::engine::Engine;
use orrery::interval::{Interval, Timestamp};
use orrery::model::{
    Actor, Event, EventId, Group, GroupId, Location, LocationId, Obligation, Person, PersonId,
    Portal, Role, Tier,
};
use orrery::repo::memory::MemoryRepo;
use orrery::repo::Repository;

fn iv(s: i64, e: i64) -> Interval {
    Interval::new(Timestamp(s), Timestamp(e)).unwrap()
}

#[test]
fn batch_run_reports_closure_changes_and_sweep() {
    let repo = MemoryRepo::new();
    let p = PersonId::new();
    let g = GroupId::new();
    let (e0, e1) = (EventId::new(), EventId::new());
    let room = Location {
        id: LocationId::new(),
        name: "room".into(),
        tier: Tier::Room,
        portal: Portal::None,
        capacity: Some(10),
        ext: Default::default(),
    };
    let room_id = room.id;

    repo.apply(Command::UpsertPerson(Person {
        id: p,
        name: "p".into(),
        derived_digest: None,
        ext: Default::default(),
    }))
    .unwrap();
    repo.apply(Command::UpsertGroup(Group {
        id: g,
        name: "g".into(),
        default_priority: None,
        timezone: None,
        ext: Default::default(),
    }))
    .unwrap();
    repo.apply(Command::UpsertLocation(room)).unwrap();
    for (e, s, end) in [(e0, 0i64, 100i64), (e1, 50, 150)] {
        repo.apply(Command::UpsertEvent(Event {
            id: e,
            name: "e".into(),
            window: iv(s, end),
            kind: "k".into(),
            timezone: None,
            ext: Default::default(),
        }))
        .unwrap();
    }

    let mut engine = Engine::new(repo).unwrap();
    // Derived schedule: membership + expectation → p's digest will change.
    engine
        .apply(Command::AddMembership {
            person: p,
            group: g,
            during: iv(0, 1000),
            role: Role::Member,
        })
        .unwrap();
    engine
        .apply(Command::AddExpectation {
            group: g,
            event: e0,
            obligation: Obligation::Expected,
            default_priority: 0.8,
            during: iv(0, 1000),
            cascades: true,
            by: Actor::System,
        })
        .unwrap();
    // Conflicting explicit attendance → the sweep has something to find.
    for e in [e0, e1] {
        engine
            .apply(Command::AddAttendance {
                person: p,
                event: e,
                priority: Some(0.9),
            })
            .unwrap();
    }
    // One hold so exactly one event is housed → one orphan for the other.
    engine
        .apply(Command::HoldLocation {
            location: room_id,
            event: e0,
            during: iv(0, 100),
            overflow_for: None,
            capacity_override: None,
        })
        .unwrap();

    let report = batch::run(&mut engine, Timestamp(500), iv(0, 1000)).unwrap();

    assert_eq!(
        report.changes.persons,
        vec![p],
        "digest change-set is exact"
    );
    assert!(
        report.sweep.emitted >= 2,
        "conflict + orphan: {:?}",
        report.sweep
    );
    assert_eq!(report.closure.pairs, 0, "single room → no travel pairs");

    // Second run immediately after: nothing changed, nothing new.
    let again = batch::run(&mut engine, Timestamp(501), iv(0, 1000)).unwrap();
    assert!(again.changes.is_empty(), "idempotent change-set");
    assert_eq!(again.sweep.emitted, 0, "idempotent sweep");
}
