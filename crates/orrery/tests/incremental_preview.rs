//! Phase 6a H2 — mirror integrity (plan-review CR-1): a preview must leave
//! the salsa mirror exactly as it found it. After previews of each mirrored
//! kind — and after further ordinary commands — every person's incremental
//! digest must still equal a cold recomputation from the repository, the
//! same two-independent-read-paths comparison the fuzz suite makes.

use orrery::command::Command;
use orrery::derive::{digest_of_ids, expand};
use orrery::engine::Engine;
use orrery::interval::{Interval, Timestamp};
use orrery::model::{Actor, Event, EventId, Group, GroupId, Obligation, Person, PersonId, Role};
use orrery::repo::memory::MemoryRepo;
use orrery::repo::Repository;

fn iv(s: i64, e: i64) -> Interval {
    Interval::new(Timestamp(s), Timestamp(e)).unwrap()
}

fn assert_incremental_matches_cold(
    engine: &mut Engine<MemoryRepo>,
    persons: &[PersonId],
    at: Timestamp,
    context: &str,
) {
    for p in persons {
        let incremental = engine.digest(*p, at);
        let cold_ids: Vec<_> = expand(engine.repo(), *p, at)
            .unwrap()
            .into_iter()
            .map(|d| d.id)
            .collect();
        assert_eq!(
            incremental,
            digest_of_ids(&cold_ids),
            "mirror corrupted ({context}) for person {p}"
        );
    }
}

#[test]
fn incremental_mirror_intact_after_previews_of_every_kind() {
    let persons: Vec<PersonId> = (0..3).map(|_| PersonId::new()).collect();
    let groups: Vec<GroupId> = (0..3).map(|_| GroupId::new()).collect();
    let events: Vec<EventId> = (0..2).map(|_| EventId::new()).collect();

    let repo = MemoryRepo::new();
    for p in &persons {
        repo.apply(Command::UpsertPerson(Person {
            id: *p,
            name: "p".into(),
            derived_digest: None,
            ext: Default::default(),
        }))
        .unwrap();
    }
    for g in &groups {
        repo.apply(Command::UpsertGroup(Group {
            id: *g,
            name: "g".into(),
            default_priority: None,
            timezone: None,
            ext: Default::default(),
        }))
        .unwrap();
    }
    for e in &events {
        repo.apply(Command::UpsertEvent(Event {
            id: *e,
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
            person: persons[0],
            group: groups[0],
            during: iv(0, 100),
            role: Role::Member,
        })
        .unwrap();
    engine
        .apply(Command::AddExpectation {
            group: groups[0],
            event: events[0],
            obligation: Obligation::Expected,
            default_priority: 0.5,
            during: iv(0, 100),
            cascades: true,
            by: Actor::System,
        })
        .unwrap();

    let t = Timestamp(50);
    engine.refresh_digests(t).unwrap();

    // Preview one command of each mirrored kind; check the mirror after each.
    let previews = [
        Command::AddMembership {
            person: persons[1],
            group: groups[0],
            during: iv(0, 100),
            role: Role::Member,
        },
        Command::AddSubgroup {
            child: groups[0],
            parent: groups[1],
            during: iv(0, 100),
        },
        Command::AddExpectation {
            group: groups[0],
            event: events[1],
            obligation: Obligation::Expected,
            default_priority: 0.9,
            during: iv(0, 100),
            cascades: false,
            by: Actor::System,
        },
    ];
    for cmd in &previews {
        engine.preview_digests(cmd, t).unwrap();
        assert_incremental_matches_cold(&mut engine, &persons, t, cmd.kind());
    }

    // Ordinary life continues after previews: apply real commands, refresh,
    // and the two read paths must still agree — including at another instant.
    engine
        .apply(Command::AddMembership {
            person: persons[2],
            group: groups[2],
            during: iv(0, 100),
            role: Role::Member,
        })
        .unwrap();
    engine
        .apply(Command::AddExpectation {
            group: groups[2],
            event: events[1],
            obligation: Obligation::Expected,
            default_priority: 0.7,
            during: iv(0, 100),
            cascades: true,
            by: Actor::System,
        })
        .unwrap();
    engine.refresh_digests(t).unwrap();
    assert_incremental_matches_cold(&mut engine, &persons, t, "after post-preview commands");
    assert_incremental_matches_cold(
        &mut engine,
        &persons,
        Timestamp(150),
        "after post-preview commands, second instant",
    );
}
