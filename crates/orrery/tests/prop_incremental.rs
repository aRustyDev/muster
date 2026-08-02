//! SPEC-05 incremental correctness: salsa-derived results must equal a cold
//! recomputation after an arbitrary mutation sequence (Phase 3 slice 2 H6).
//!
//! The engine's memoized digest is compared against a digest computed cold
//! from `derive::expand` over the repository — two independent read paths
//! (salsa mirror vs repository traversal) that must agree for every person
//! after every fuzzed command sequence.

use orrery::command::Command;
use orrery::derive::{digest_of_ids, expand};
use orrery::engine::Engine;
use orrery::interval::{Interval, Timestamp};
use orrery::model::{Actor, Event, Group, GroupId, Obligation, Person, PersonId, Role};
use orrery::repo::memory::MemoryRepo;
use orrery::repo::Repository;
use proptest::prelude::*;

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

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]
    #[test]
    fn prop_incremental_digest_matches_cold(
        ops in proptest::collection::vec(op_strategy(), 1..25),
        at in 0i64..400,
    ) {
        let iv = |s: i64, e: i64| Interval::new(Timestamp(s), Timestamp(e)).unwrap();
        let persons: Vec<PersonId> = (0..4).map(|_| PersonId::new()).collect();
        let groups: Vec<GroupId> = (0..6).map(|_| GroupId::new()).collect();
        let events: Vec<_> = (0..6)
            .map(|_| Event {
                id: orrery::model::EventId::new(),
                name: "e".into(),
                window: iv(0, 1000),
                kind: "k".into(),
                timezone: None,
                ext: Default::default(),
            })
            .collect();

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
            repo.apply(Command::UpsertEvent(e.clone())).unwrap();
        }

        let mut engine = Engine::new(repo).unwrap();
        for op in &ops {
            let cmd = match op {
                Op::Membership { p, g, from, len } => Command::AddMembership {
                    person: persons[*p],
                    group: groups[*g],
                    during: iv(*from, from + len),
                    role: Role::Member,
                },
                Op::Subgroup { child, parent, from, len } => Command::AddSubgroup {
                    child: groups[*child],
                    parent: groups[*parent],
                    during: iv(*from, from + len),
                },
                Op::Expectation { g, e, prio, from, len, cascades } => Command::AddExpectation {
                    group: groups[*g],
                    event: events[*e].id,
                    obligation: Obligation::Expected,
                    default_priority: (*prio as f32) / 100.0,
                    during: iv(*from, from + len),
                    cascades: *cascades,
                    by: Actor::System,
                },
            };
            engine.apply(cmd).unwrap();
        }

        let at_t = Timestamp(at);
        for p in &persons {
            let incremental = engine.digest(*p, at_t);
            let cold_ids: Vec<_> = expand(engine.repo(), *p, at_t)
                .unwrap()
                .into_iter()
                .map(|d| d.id)
                .collect();
            prop_assert_eq!(
                incremental,
                digest_of_ids(&cold_ids),
                "divergence for person {} at t={}",
                p,
                at
            );
        }
    }
}
