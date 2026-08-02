//! Early-cutoff H7(a): a membership write for person A must not re-execute
//! the expansion or digest for unrelated person B (Phase 3 slice 2).
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
fn incremental_early_cutoff_unrelated_person() {
    let repo = MemoryRepo::new();
    let (a, b) = (PersonId::new(), PersonId::new());
    let (ga, gb, gc) = (GroupId::new(), GroupId::new(), GroupId::new());
    let (ea, eb) = (EventId::new(), EventId::new());

    for p in [a, b] {
        repo.apply(Command::UpsertPerson(Person {
            id: p,
            name: "p".into(),
            derived_digest: None,
            ext: Default::default(),
        }))
        .unwrap();
    }
    for g in [ga, gb, gc] {
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
    for (p, g, e) in [(a, ga, ea), (b, gb, eb)] {
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
                event: e,
                obligation: Obligation::Expected,
                default_priority: 0.5,
                during: iv(0, 100),
                cascades: true,
                by: Actor::System,
            })
            .unwrap();
    }

    let t = Timestamp(50);
    let da0 = engine.digest(a, t);
    let db0 = engine.digest(b, t);
    let warm = probe::snapshot();

    // The write that defines the test: A joins gc (no expectations there).
    engine
        .apply(Command::AddMembership {
            person: a,
            group: gc,
            during: iv(0, 100),
            role: Role::Member,
        })
        .unwrap();

    // B's digest: the cheap extraction may re-run and backdate, but B's
    // expansion (derived_ids) and digest must NOT re-execute — that is the
    // blast-radius bound ADR-0016 promises.
    let db1 = engine.digest(b, t);
    let after_b = probe::snapshot();
    assert_eq!(db1, db0, "B's digest value is unchanged");
    assert_eq!(
        after_b[2], warm[2],
        "B's derived_ids must not re-execute after A's membership write"
    );
    assert_eq!(
        after_b[3], warm[3],
        "B's digest must not re-execute after A's membership write"
    );

    // A's own chain re-derives (its direct groups really changed), but the
    // id set is unchanged (gc has no expectations) so A's digest backdates
    // mid-chain: derived_ids re-executes, digest does not.
    let da1 = engine.digest(a, t);
    let after_a = probe::snapshot();
    assert_eq!(
        da1, da0,
        "A's digest value is unchanged (gc adds no events)"
    );
    assert!(
        after_a[2] > after_b[2],
        "A's derived_ids re-executes (its inputs really changed)"
    );
    assert_eq!(
        after_a[3], after_b[3],
        "A's digest backdates: equal id set stops the chain before the digest"
    );
}
