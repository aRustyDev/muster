//! Phase 6a H3 — preview cost is bounded by the blast radius, not the
//! population (the ROADMAP claim that the preview needs salsa early
//! cutoff, asserted via the probe rather than trusted).
//!
//! Own test binary: the probe counters are process-global, and one test per
//! process keeps them deterministic under both cargo test and nextest.

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
fn incremental_preview_digest_cost_is_blast_radius_bounded() {
    const N: usize = 12;
    let repo = MemoryRepo::new();
    let persons: Vec<PersonId> = (0..N).map(|_| PersonId::new()).collect();
    let groups: Vec<GroupId> = (0..N).map(|_| GroupId::new()).collect();
    let target_group = GroupId::new();
    let target_event = EventId::new();

    for p in &persons {
        repo.apply(Command::UpsertPerson(Person {
            id: *p,
            name: "p".into(),
            derived_digest: None,
            ext: Default::default(),
        }))
        .unwrap();
    }
    for g in groups.iter().chain([&target_group]) {
        repo.apply(Command::UpsertGroup(Group {
            id: *g,
            name: "g".into(),
            default_priority: None,
            timezone: None,
            ext: Default::default(),
        }))
        .unwrap();
    }
    repo.apply(Command::UpsertEvent(Event {
        id: target_event,
        name: "e".into(),
        window: iv(0, 1000),
        kind: "k".into(),
        timezone: None,
        ext: Default::default(),
    }))
    .unwrap();

    let mut engine = Engine::new(repo).unwrap();
    // Every person in their own group; the target group carries the only
    // expectation, and nobody is a member of it yet.
    for (p, g) in persons.iter().zip(&groups) {
        engine
            .apply(Command::AddMembership {
                person: *p,
                group: *g,
                during: iv(0, 100),
                role: Role::Member,
            })
            .unwrap();
    }
    engine
        .apply(Command::AddExpectation {
            group: target_group,
            event: target_event,
            obligation: Obligation::Expected,
            default_priority: 0.5,
            during: iv(0, 100),
            cascades: true,
            by: Actor::System,
        })
        .unwrap();

    let t = Timestamp(50);
    engine.refresh_digests(t).unwrap();
    let digest_before = engine.digest(persons[0], t);
    let warm = probe::snapshot();

    // Preview: person 0 would join the target group → exactly one person's
    // derived set changes.
    let changed = engine
        .preview_digests(
            &Command::AddMembership {
                person: persons[0],
                group: target_group,
                during: iv(0, 100),
                role: Role::Member,
            },
            t,
        )
        .unwrap();
    let after = probe::snapshot();

    assert_eq!(changed, vec![persons[0]]);
    // The cheap extraction layer re-runs for everyone the preview demands
    // (that is what backdating costs)...
    assert_eq!(
        after[0] - warm[0],
        N as u64,
        "direct_groups re-executes once per person during the preview"
    );
    // ...but the expansion and digest layers re-execute ONLY for the
    // affected person — the blast-radius bound. A population-scaling delta
    // here falsifies H3.
    assert_eq!(
        after[2] - warm[2],
        1,
        "derived_ids must re-execute only for the affected person"
    );
    assert_eq!(
        after[3] - warm[3],
        1,
        "digest must re-execute only for the affected person"
    );

    // And the restore leaves the previewed person's digest as it was.
    assert_eq!(
        engine.digest(persons[0], t),
        digest_before,
        "post-preview digest must equal the pre-preview value"
    );
}
