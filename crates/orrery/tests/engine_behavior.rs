//! Engine behaviour: digest refresh change-sets (H8 adjacent), sweep
//! lifecycle (H8), the Prevent call site (H9), and the oracle (Phase 3
//! slice 2).

use orrery::command::Command;
use orrery::detect::Policy;
use orrery::engine::{Assignment, Engine};
use orrery::interval::{Interval, Timestamp};
use orrery::model::{
    Actor, AttendanceSource, Attends, Event, EventId, Group, GroupId, Location, LocationId,
    Obligation, Person, PersonId, Portal, Role, Tier, ViolationKind,
};
use orrery::repo::memory::MemoryRepo;
use orrery::repo::Repository;
use orrery::FeasibilityOracle;

fn iv(s: i64, e: i64) -> Interval {
    Interval::new(Timestamp(s), Timestamp(e)).unwrap()
}

fn seeded() -> (
    Engine<MemoryRepo>,
    PersonId,
    GroupId,
    Vec<EventId>,
    LocationId,
) {
    let repo = MemoryRepo::new();
    let p = PersonId::new();
    let g = GroupId::new();
    let loc = LocationId::new();
    let events: Vec<EventId> = (0..3).map(|_| EventId::new()).collect();

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
    repo.apply(Command::UpsertLocation(Location {
        id: loc,
        name: "room".into(),
        tier: Tier::Room,
        portal: Portal::None,
        capacity: Some(2),
        ext: Default::default(),
    }))
    .unwrap();
    // Overlapping windows: e0 [0,100), e1 [50,150), e2 [200,300).
    for (i, (s, e)) in [(0, 100), (50, 150), (200, 300)].iter().enumerate() {
        repo.apply(Command::UpsertEvent(Event {
            id: events[i],
            name: format!("e{i}"),
            window: iv(*s, *e),
            kind: "k".into(),
            timezone: None,
            ext: Default::default(),
        }))
        .unwrap();
    }
    (Engine::new(repo).unwrap(), p, g, events, loc)
}

#[test]
fn incremental_refresh_digests_returns_exact_change_set() {
    let (mut engine, p, g, events, _) = seeded();
    let t = Timestamp(60);

    // First refresh: every person's digest is unset → everyone changes.
    let changed = engine.refresh_digests(t).unwrap();
    assert_eq!(changed, vec![p]);
    // Idempotent: nothing changed since.
    assert!(engine.refresh_digests(t).unwrap().is_empty());

    // A write that changes p's derived set → exactly p reported, digest
    // persisted on the person record (ADR-0016 B).
    engine
        .apply(Command::AddMembership {
            person: p,
            group: g,
            during: iv(0, 100),
            role: Role::Member,
        })
        .unwrap();
    engine
        .apply(Command::AddExpectation {
            group: g,
            event: events[2],
            obligation: Obligation::Expected,
            default_priority: 0.7,
            during: iv(0, 100),
            cascades: true,
            by: Actor::System,
        })
        .unwrap();
    let changed = engine.refresh_digests(t).unwrap();
    assert_eq!(changed, vec![p]);
    let stored = engine.repo().person(p).unwrap().unwrap().derived_digest;
    assert!(stored.is_some_and(|d| d.at == t));
    assert!(engine.refresh_digests(t).unwrap().is_empty());
}

#[test]
fn sweep_lifecycle_emit_dedupe_resolve_waive() {
    let (mut engine, p, _, events, _) = seeded();
    let window = iv(0, 1000);
    let t1 = Timestamp(500);

    // Overlapping attendance on e0/e1 → one time conflict.
    for e in [events[0], events[1]] {
        engine
            .apply(Command::AddAttendance {
                person: p,
                event: e,
                priority: None,
            })
            .unwrap();
    }
    // 1 time conflict + 3 orphan events (nothing is held in this fixture).
    let r1 = engine.sweep(t1, window).unwrap();
    assert_eq!((r1.emitted, r1.resolved), (4, 0));

    // Idempotent: same state, nothing new, nothing resolved.
    let r2 = engine.sweep(Timestamp(510), window).unwrap();
    assert_eq!((r2.emitted, r2.resolved), (0, 0));

    // Waive the conflict: subsequent sweeps must neither duplicate nor
    // auto-resolve it.
    let open = engine.repo().open_violations().unwrap();
    let conflict = open
        .iter()
        .find(|v| v.kind == ViolationKind::TimeConflict)
        .expect("conflict recorded");
    engine
        .apply(Command::WaiveViolation {
            id: conflict.id,
            by: Actor::Coordinator(PersonId::new()),
            reason: "presenting remotely from the other room".into(),
        })
        .unwrap();
    let r3 = engine.sweep(Timestamp(520), window).unwrap();
    assert_eq!(
        (r3.emitted, r3.resolved),
        (0, 0),
        "waived violation is neither duplicated nor resolved"
    );
    let still_open = engine.repo().open_violations().unwrap();
    assert!(
        still_open
            .iter()
            .any(|v| v.kind == ViolationKind::TimeConflict && v.waiver_reason.is_some()),
        "waiver keeps the record open, on the record"
    );
}

#[test]
fn sweep_resolves_disappeared_violation() {
    let (mut engine, _, _, events, loc) = seeded();
    let window = iv(0, 1000);

    // Double-book the room on overlapping events e0/e1.
    for e in [events[0], events[1]] {
        engine
            .apply(Command::HoldLocation {
                location: loc,
                event: e,
                during: engine.repo().event(e).unwrap().unwrap().window,
                overflow_for: None,
                capacity_override: None,
            })
            .unwrap();
    }
    // e0/e1 double-book the room (1 exclusivity); e2 is unheld (1 orphan).
    let r1 = engine.sweep(Timestamp(400), window).unwrap();
    assert_eq!(r1.emitted, 2);
    let open = engine.repo().open_violations().unwrap();
    assert!(open
        .iter()
        .any(|v| v.kind == ViolationKind::LocationExclusivity));
    assert!(open.iter().any(|v| v.kind == ViolationKind::OrphanEvent));

    // Give e2 a hold in a second sweep world state: orphan disappears.
    engine
        .apply(Command::HoldLocation {
            location: loc,
            event: events[2],
            during: engine.repo().event(events[2]).unwrap().unwrap().window,
            overflow_for: None,
            capacity_override: None,
        })
        .unwrap();
    let r2 = engine.sweep(Timestamp(410), window).unwrap();
    assert_eq!(
        r2.resolved, 1,
        "orphan violation resolved once e2 is housed"
    );
    let open = engine.repo().open_violations().unwrap();
    assert!(open.iter().all(|v| v.kind != ViolationKind::OrphanEvent));
}

#[test]
fn prevent_is_the_same_detector_at_a_second_call_site() {
    let (mut engine, p, _, events, _) = seeded();
    engine
        .policies_mut()
        .set(ViolationKind::TimeConflict, Policy::Prevent);

    engine
        .apply(Command::AddAttendance {
            person: p,
            event: events[0],
            priority: None,
        })
        .unwrap();

    // e1 overlaps e0 → prevented, and no state change.
    let err = engine
        .apply(Command::AddAttendance {
            person: p,
            event: events[1],
            priority: None,
        })
        .unwrap_err();
    assert!(err.to_string().contains("prevented"), "{err}");
    assert_eq!(
        engine.repo().attends_for(p, iv(0, 1000)).unwrap().len(),
        1,
        "prevented write must not mutate state"
    );

    // e2 is disjoint → lands.
    engine
        .apply(Command::AddAttendance {
            person: p,
            event: events[2],
            priority: None,
        })
        .unwrap();

    // Under Detect the same write lands (same detector, different policy).
    let (mut engine2, p2, _, events2, _) = seeded();
    engine2
        .policies_mut()
        .set(ViolationKind::TimeConflict, Policy::Detect);
    for e in [events2[0], events2[1]] {
        engine2
            .apply(Command::AddAttendance {
                person: p2,
                event: e,
                priority: None,
            })
            .unwrap();
    }
    assert_eq!(
        engine2.repo().attends_for(p2, iv(0, 1000)).unwrap().len(),
        2
    );
}

#[test]
fn oracle_scores_overlay_without_writing() {
    let (mut engine, p, _, events, _) = seeded();
    engine
        .apply(Command::AddAttendance {
            person: p,
            event: events[0],
            priority: None,
        })
        .unwrap();

    let overlay = |event: EventId, window: Interval| Assignment {
        attends: vec![Attends {
            person: p,
            event,
            during: window,
            priority_group: 0.0,
            priority_person: None,
            priority_coord: None,
            coord_binding: false,
            source: AttendanceSource::SelfSelected,
            pinned: false,
        }],
        held: vec![],
        at: Timestamp(60),
        window: iv(0, 1000),
    };

    // Proposing overlapping e1: one Hard time conflict → score -100.
    let bad = overlay(events[1], iv(50, 150));
    let violations = engine.is_feasible(&bad);
    assert_eq!(violations.len(), 1);
    assert_eq!(engine.score(&bad), -100.0);

    // Proposing disjoint e2: clean → score 0.
    let good = overlay(events[2], iv(200, 300));
    assert!(engine.is_feasible(&good).is_empty());
    assert_eq!(engine.score(&good), 0.0);

    // Read-only: nothing was written by evaluation.
    assert_eq!(engine.repo().attends_for(p, iv(0, 1000)).unwrap().len(), 1);
}
