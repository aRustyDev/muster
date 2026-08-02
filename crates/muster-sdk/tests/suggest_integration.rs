//! Phase 5 Prototype gate: `suggest_room_schedule` end-to-end against
//! `Engine<MemoryRepo>` — placements + violations + additive breakdown,
//! with unassignable events surfaced and an externally-seeded conflict
//! flowing back verbatim from the engine.

use muster_sdk::{suggest_room_schedule, RoomOption, RoomRequest};
use orrery::command::Command;
use orrery::engine::Engine;
use orrery::interval::{Interval, Timestamp};
use orrery::model::{Event, EventId, Location, LocationId, Portal, Tier, ViolationKind};
use orrery::repo::memory::MemoryRepo;
use orrery::repo::Repository;

fn iv(s: i64, e: i64) -> Interval {
    Interval::new(Timestamp(s), Timestamp(e)).unwrap()
}

fn room(name: &str, cap: u32) -> Location {
    Location {
        id: LocationId::new(),
        name: name.into(),
        tier: Tier::Room,
        portal: Portal::None,
        capacity: Some(cap),
        ext: Default::default(),
    }
}

fn event(id: EventId, s: i64, e: i64) -> Event {
    Event {
        id,
        name: "e".into(),
        window: iv(s, e),
        kind: "talk".into(),
        timezone: None,
        ext: Default::default(),
    }
}

#[test]
fn suggestion_places_flags_and_explains() {
    let repo = MemoryRepo::new();
    let (small, large) = (room("small", 20), room("large", 200));
    let (small_id, large_id) = (small.id, large.id);
    let talks: Vec<EventId> = (0..3).map(|_| EventId::new()).collect();
    // t0 [0,100) and t1 [50,150) overlap; t2 [200,300) is clear.
    let windows = [(0i64, 100i64), (50, 150), (200, 300)];

    repo.apply(Command::UpsertLocation(small)).unwrap();
    repo.apply(Command::UpsertLocation(large)).unwrap();
    for (id, (s, e)) in talks.iter().zip(windows) {
        repo.apply(Command::UpsertEvent(event(*id, s, e))).unwrap();
    }
    // Externally-seeded conflict: an existing hold occupies `small`
    // during [0,100) for a fourth event.
    let existing = EventId::new();
    repo.apply(Command::UpsertEvent(event(existing, 0, 100)))
        .unwrap();
    repo.apply(Command::HoldLocation {
        location: small_id,
        event: existing,
        during: iv(0, 100),
        overflow_for: None,
        capacity_override: None,
    })
    .unwrap();

    let engine = Engine::new(repo).unwrap();
    let requests: Vec<RoomRequest> = talks
        .iter()
        .zip(windows)
        .map(|(id, (s, e))| RoomRequest {
            event: *id,
            window: iv(s, e),
            expected_size: Some(15),
        })
        .collect();
    let rooms = vec![
        RoomOption {
            location: small_id,
            capacity: Some(20),
        },
        RoomOption {
            location: large_id,
            capacity: Some(200),
        },
    ];

    let s = suggest_room_schedule(&engine, &requests, &rooms, Timestamp(1), iv(0, 1000)).unwrap();

    // Two rooms, two overlapping talks + one clear one → all three placed.
    assert_eq!(s.placements.len(), 3);
    assert!(s.unassigned.is_empty());

    // Whichever talk landed in `small` during [0,100) collides with the
    // pre-existing hold — the engine reports it; the SDK relays verbatim.
    assert!(
        s.violations
            .iter()
            .any(|v| v.kind == ViolationKind::LocationExclusivity),
        "seeded conflict must surface: {:?}",
        s.violations
    );

    // Breakdown is additive and carries the violation cost.
    let sum: f64 = s.breakdown.rows.iter().map(|r| r.weighted).sum();
    assert!((s.breakdown.total - sum).abs() < 1e-9, "additivity");
    let vc = s
        .breakdown
        .rows
        .iter()
        .find(|r| r.name == "violation_cost")
        .unwrap();
    assert!(vc.cost >= 100.0, "Hard exclusivity weighs 100: {vc:?}");

    // Determinism: run it again, bit-identical placements and total.
    let s2 = suggest_room_schedule(&engine, &requests, &rooms, Timestamp(1), iv(0, 1000)).unwrap();
    assert_eq!(s.placements, s2.placements);
    assert_eq!(s.breakdown.total, s2.breakdown.total);
}

#[test]
fn unassignable_events_are_surfaced_not_dropped() {
    let repo = MemoryRepo::new();
    let only = room("only", 30);
    let only_id = only.id;
    repo.apply(Command::UpsertLocation(only)).unwrap();
    let talks: Vec<EventId> = (0..2).map(|_| EventId::new()).collect();
    for id in &talks {
        repo.apply(Command::UpsertEvent(event(*id, 0, 100)))
            .unwrap();
    }
    let engine = Engine::new(repo).unwrap();
    let requests: Vec<RoomRequest> = talks
        .iter()
        .map(|id| RoomRequest {
            event: *id,
            window: iv(0, 100),
            expected_size: None,
        })
        .collect();
    let rooms = vec![RoomOption {
        location: only_id,
        capacity: Some(30),
    }];

    let s = suggest_room_schedule(&engine, &requests, &rooms, Timestamp(1), iv(0, 1000)).unwrap();
    assert_eq!(s.placements.len(), 1);
    assert_eq!(
        s.unassigned.len(),
        1,
        "the second talk is reported, not lost"
    );
    assert!(s.violations.is_empty(), "greedy never double-books");
}
