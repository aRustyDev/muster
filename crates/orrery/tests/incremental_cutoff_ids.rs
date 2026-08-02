//! Early-cutoff H7(b): an expectation write on a group unrelated to A
//! re-executes A's derived-ids (its input field really changed) but NOT A's
//! digest, because the id set is unchanged and salsa backdates mid-chain.
//!
//! Own test binary: probe counters are process-global.

use orrery::command::Command;
use orrery::engine::Engine;
use orrery::incremental::probe;
use orrery::interval::{Interval, Timestamp};
use orrery::model::{Actor, Event, EventId, Group, GroupId, Obligation, Person, PersonId, Role};
use orrery::repo::memory::MemoryRepo;
use orrery::repo::Repository;

fn iv(s: i64, e: i64) -> Interval {
    Interval::new(Timestamp(s), Timestamp(e)).unwrap()
}

#[test]
fn incremental_cutoff_stops_at_equal_id_set() {
    let repo = MemoryRepo::new();
    let a = PersonId::new();
    let (ga, gb) = (GroupId::new(), GroupId::new());
    let (ea, eb) = (EventId::new(), EventId::new());

    repo.apply(Command::UpsertPerson(Person {
        id: a,
        name: "a".into(),
        derived_digest: None,
        ext: Default::default(),
    }))
    .unwrap();
    for g in [ga, gb] {
        repo.apply(Command::UpsertGroup(Group {
            id: g,
            name: "g".into(),
            default_priority: None,
            timezone: None,
            ext: Default::default(),
        }))
        .unwrap();
    }
    for e in [ea, eb] {
        repo.apply(Command::UpsertEvent(Event {
            id: e,
            name: "e".into(),
            window: iv(0, 1000),
            kind: "k".into(),
            timezone: None,
            ext: Default::default(),
        }))
        .unwrap();
    }

    let mut engine = Engine::new(repo).unwrap();
    engine
        .apply(Command::AddMembership {
            person: a,
            group: ga,
            during: iv(0, 100),
            role: Role::Member,
        })
        .unwrap();
    engine
        .apply(Command::AddExpectation {
            group: ga,
            event: ea,
            obligation: Obligation::Expected,
            default_priority: 0.5,
            during: iv(0, 100),
            cascades: true,
            by: Actor::System,
        })
        .unwrap();

    let t = Timestamp(50);
    let d0 = engine.digest(a, t);
    let warm = probe::snapshot();

    // Expectation on gb — A is not a member of gb; A's id set is unchanged.
    engine
        .apply(Command::AddExpectation {
            group: gb,
            event: eb,
            obligation: Obligation::Expected,
            default_priority: 0.9,
            during: iv(0, 100),
            cascades: true,
            by: Actor::System,
        })
        .unwrap();

    let d1 = engine.digest(a, t);
    let after = probe::snapshot();
    assert_eq!(d1, d0, "A's digest value unchanged");
    assert!(
        after[2] > warm[2],
        "derived_ids re-executes (the expectation input really changed)"
    );
    assert_eq!(
        after[3], warm[3],
        "digest does NOT re-execute: equal id set backdates mid-chain — \
         early cutoff (ADR-0016 C) demonstrated"
    );
}
