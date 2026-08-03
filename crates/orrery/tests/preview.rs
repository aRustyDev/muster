//! Phase 6a H1 — preview honesty (muster/SPEC-03:17-21, engine side):
//! `preview_digests(cmd, at)` must equal the change set of actually
//! applying `cmd` and running `refresh_digests(at)`, and the preview
//! itself must write nothing. A preview that lies is worse than none.

use orrery::command::Command;
use orrery::engine::Engine;
use orrery::error::OrreryError;
use orrery::interval::{Interval, Timestamp};
use orrery::model::{Actor, Event, EventId, Group, GroupId, Obligation, Person, PersonId, Role};
use orrery::repo::memory::MemoryRepo;
use orrery::repo::Repository;
use proptest::prelude::*;

fn iv(s: i64, e: i64) -> Interval {
    Interval::new(Timestamp(s), Timestamp(e)).unwrap()
}

fn seeded_engine(
    persons: &[PersonId],
    groups: &[GroupId],
    events: &[EventId],
) -> Engine<MemoryRepo> {
    let repo = MemoryRepo::new();
    for p in persons {
        repo.apply(Command::UpsertPerson(Person {
            id: *p,
            name: "p".into(),
            derived_digest: None,
            ext: Default::default(),
        }))
        .unwrap();
    }
    for g in groups {
        repo.apply(Command::UpsertGroup(Group {
            id: *g,
            name: "g".into(),
            default_priority: None,
            timezone: None,
            ext: Default::default(),
        }))
        .unwrap();
    }
    for e in events {
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
    Engine::new(repo).unwrap()
}

#[derive(Debug, Clone)]
enum Op {
    Membership {
        p: usize,
        g: usize,
        from: i64,
        len: i64,
    },
    Subgroup {
        child: usize,
        parent: usize,
        from: i64,
        len: i64,
    },
    Expectation {
        g: usize,
        e: usize,
        prio: u8,
        from: i64,
        len: i64,
        cascades: bool,
    },
}

fn op_strategy() -> impl Strategy<Value = Op> {
    prop_oneof![
        (0usize..4, 0usize..6, 0i64..300, 1i64..300).prop_map(|(p, g, from, len)| Op::Membership {
            p,
            g,
            from,
            len
        }),
        (0usize..6, 0usize..6, 0i64..300, 1i64..300).prop_map(|(child, parent, from, len)| {
            Op::Subgroup {
                child,
                parent,
                from,
                len,
            }
        }),
        (
            0usize..6,
            0usize..6,
            0u8..100,
            0i64..300,
            1i64..300,
            prop::bool::ANY
        )
            .prop_map(|(g, e, prio, from, len, cascades)| Op::Expectation {
                g,
                e,
                prio,
                from,
                len,
                cascades
            }),
    ]
}

fn to_command(op: &Op, persons: &[PersonId], groups: &[GroupId], events: &[EventId]) -> Command {
    let iv = |s: i64, e: i64| Interval::new(Timestamp(s), Timestamp(e)).unwrap();
    match op {
        Op::Membership { p, g, from, len } => Command::AddMembership {
            person: persons[*p],
            group: groups[*g],
            during: iv(*from, from + len),
            role: Role::Member,
        },
        Op::Subgroup {
            child,
            parent,
            from,
            len,
        } => Command::AddSubgroup {
            child: groups[*child],
            parent: groups[*parent],
            during: iv(*from, from + len),
        },
        Op::Expectation {
            g,
            e,
            prio,
            from,
            len,
            cascades,
        } => Command::AddExpectation {
            group: groups[*g],
            event: events[*e],
            obligation: Obligation::Expected,
            default_priority: (*prio as f32) / 100.0,
            during: iv(*from, from + len),
            cascades: *cascades,
            by: Actor::System,
        },
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]

    /// H1: on fuzzed worlds — warmed or cold — the preview of any mirrored
    /// command equals the post-commit `refresh_digests` change set, and the
    /// preview issues zero repository writes (stored digest records
    /// untouched; the receipt sequence advances by exactly one across the
    /// preview + real apply).
    #[test]
    fn prop_preview_matches_post_commit_refresh(
        ops in proptest::collection::vec(op_strategy(), 0..20),
        previewed in op_strategy(),
        warm in prop::bool::ANY,
        at in 0i64..400,
    ) {
        let persons: Vec<PersonId> = (0..4).map(|_| PersonId::new()).collect();
        let groups: Vec<GroupId> = (0..6).map(|_| GroupId::new()).collect();
        let events: Vec<EventId> = (0..6).map(|_| EventId::new()).collect();
        let mut engine = seeded_engine(&persons, &groups, &events);
        for op in &ops {
            engine.apply(to_command(op, &persons, &groups, &events)).unwrap();
        }
        let at_t = Timestamp(at);
        if warm {
            engine.refresh_digests(at_t).unwrap();
        }

        let stored_before: Vec<_> = persons
            .iter()
            .map(|p| engine.repo().person(*p).unwrap().unwrap().derived_digest)
            .collect();
        let seq_before = engine
            .apply(Command::UpsertGroup(Group {
                id: groups[0],
                name: "g".into(),
                default_priority: None,
                timezone: None,
                ext: Default::default(),
            }))
            .unwrap()
            .seq;

        let cmd = to_command(&previewed, &persons, &groups, &events);
        let mut preview = engine.preview_digests(&cmd, at_t).unwrap();

        // Zero writes: stored digests untouched, no receipt consumed.
        let stored_after: Vec<_> = persons
            .iter()
            .map(|p| engine.repo().person(*p).unwrap().unwrap().derived_digest)
            .collect();
        prop_assert_eq!(&stored_before, &stored_after, "preview persisted a digest");
        let receipt = engine.apply(cmd).unwrap();
        prop_assert_eq!(receipt.seq, seq_before + 1, "preview consumed a receipt");

        let mut actual = engine.refresh_digests(at_t).unwrap();
        preview.sort();
        actual.sort();
        prop_assert_eq!(preview, actual);
    }
}

/// Deterministic honesty case, readable on failure: an expectation on a
/// group with one member previews to exactly that member.
#[test]
fn preview_expectation_names_exactly_the_affected_member() {
    let persons: Vec<PersonId> = (0..3).map(|_| PersonId::new()).collect();
    let groups: Vec<GroupId> = (0..2).map(|_| GroupId::new()).collect();
    let events: Vec<EventId> = (0..1).map(|_| EventId::new()).collect();
    let mut engine = seeded_engine(&persons, &groups, &events);
    engine
        .apply(Command::AddMembership {
            person: persons[0],
            group: groups[0],
            during: iv(0, 100),
            role: Role::Member,
        })
        .unwrap();
    let t = Timestamp(50);
    engine.refresh_digests(t).unwrap();

    let cmd = Command::AddExpectation {
        group: groups[0],
        event: events[0],
        obligation: Obligation::Expected,
        default_priority: 0.5,
        during: iv(0, 100),
        cascades: true,
        by: Actor::System,
    };
    let preview = engine.preview_digests(&cmd, t).unwrap();
    assert_eq!(preview, vec![persons[0]]);

    engine.apply(cmd).unwrap();
    assert_eq!(engine.refresh_digests(t).unwrap(), vec![persons[0]]);
}

/// Unsupported kinds are a typed refusal, not a possibly-lying answer.
#[test]
fn preview_unsupported_kind_is_a_typed_error() {
    let persons: Vec<PersonId> = vec![PersonId::new()];
    let events: Vec<EventId> = vec![EventId::new()];
    let mut engine = seeded_engine(&persons, &[], &events);

    for cmd in [
        Command::AddAttendance {
            person: persons[0],
            event: events[0],
            priority: None,
        },
        Command::RemoveAttendance {
            person: persons[0],
            event: events[0],
        },
        Command::UpsertPerson(Person {
            id: persons[0],
            name: "p".into(),
            derived_digest: None,
            ext: Default::default(),
        }),
    ] {
        let err = engine.preview_digests(&cmd, Timestamp(0)).unwrap_err();
        assert!(
            matches!(err, OrreryError::PreviewUnsupported { .. }),
            "expected PreviewUnsupported for {}, got {err:?}",
            cmd.kind()
        );
    }
}
