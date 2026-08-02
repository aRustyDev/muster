//! `expired_membership_effect`: cached derived attendance whose source
//! membership is no longer valid. Partition: person.
//!
//! Under pure derivation this state cannot arise (expansion filters at
//! evaluation time); it exists precisely because derived semantics are
//! **cached physically** (ADR-0004) and a membership can expire after the
//! cache was written. This detector is the staleness audit for that cache.

use crate::derive::DerivedAttends;
use crate::detect::{ViolationDraft, SEVERITY_EXPIRED_MEMBERSHIP};
use crate::interval::Timestamp;
use crate::model::{EntityRef, MemberOf, PersonId, ViolationKind};

pub fn detect(
    person: PersonId,
    cached: &[DerivedAttends],
    memberships: &[MemberOf],
    at: Timestamp,
) -> Vec<ViolationDraft> {
    cached
        .iter()
        .filter(|d| d.person == person)
        .filter(|d| {
            !memberships.iter().any(|m| {
                m.person == person && m.group == d.source_group && m.during.contains_point(at)
            })
        })
        .map(|d| ViolationDraft {
            kind: ViolationKind::ExpiredMembershipEffect,
            severity: SEVERITY_EXPIRED_MEMBERSHIP,
            subjects: vec![
                EntityRef::Person(person),
                EntityRef::Event(d.event),
                EntityRef::Group(d.source_group),
            ],
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::derive::derived_id;
    use crate::interval::Interval;
    use crate::model::{EventId, GroupId, Obligation, Role};
    use proptest::prelude::*;

    fn iv(s: i64, e: i64) -> Interval {
        Interval::new(Timestamp(s), Timestamp(e)).unwrap()
    }

    fn cached_edge(person: PersonId, group: GroupId) -> DerivedAttends {
        let event = EventId::new();
        DerivedAttends {
            id: derived_id(person, event, group, Timestamp(0)),
            person,
            event,
            source_group: group,
            priority_group: 0.5,
            obligation: Obligation::Expected,
            during: iv(0, 10),
        }
    }

    proptest! {
        /// For each cached edge, membership validity at `at` decides —
        /// oracle is a naive containment recheck.
        #[test]
        fn prop_matches_naive(
            edges in proptest::collection::vec((0i64..200, 1i64..200), 1..8),
            at in 0i64..400,
        ) {
            let p = PersonId::new();
            let at = Timestamp(at);
            let mut cached = Vec::new();
            let mut memberships = Vec::new();
            for (from, len) in &edges {
                let g = GroupId::new();
                cached.push(cached_edge(p, g));
                memberships.push(MemberOf {
                    person: p,
                    group: g,
                    during: iv(*from, from + len),
                    role: Role::Member,
                });
            }
            let expected = memberships
                .iter()
                .filter(|m| !(m.during.start() <= at && at < m.during.end()))
                .count();
            prop_assert_eq!(detect(p, &cached, &memberships, at).len(), expected);
        }
    }

    #[test]
    fn valid_membership_is_clean() {
        let p = PersonId::new();
        let g = GroupId::new();
        let cached = [cached_edge(p, g)];
        let memberships = [MemberOf {
            person: p,
            group: g,
            during: iv(0, 100),
            role: Role::Member,
        }];
        assert!(detect(p, &cached, &memberships, Timestamp(50)).is_empty());
        assert_eq!(detect(p, &cached, &memberships, Timestamp(150)).len(), 1);
    }
}
