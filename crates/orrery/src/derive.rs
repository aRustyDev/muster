//! Derived attendance expansion (Q1) — derived semantics, cached physically
//! (ADR-0004), with deterministic content-addressed identity (ADR-0016 A).
//!
//! Expansion is per-hop temporally filtered (the datastore stop-gate
//! requirement, RESEARCH-0002) and **includes depth-0 direct groups** — the
//! semantics Phase 0 showed the original benchmark got wrong. `cascades`
//! controls whether an ancestor group's expectation flows down to members
//! of its subgroups; expectations on a person's direct groups always apply.

use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::interval::{Interval, Timestamp};
use crate::model::{Attends, EventId, GroupId, Obligation, PersonId};
use crate::repo::Repository;

/// Content-addressed identity for a derived edge: stable across
/// recomputation with unchanged inputs, so violations, pins, and overrides
/// can reference it (ADR-0016).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct DerivedId(pub [u8; 32]);

/// SPEC-01 gives expectations no surrogate id, so expectation identity is
/// content-derived: `group ‖ event ‖ expectation-window-start` (flagged for
/// the slice-2 spec update).
pub fn derived_id(
    person: PersonId,
    event: EventId,
    group: GroupId,
    expectation_start: Timestamp,
) -> DerivedId {
    let mut h = blake3::Hasher::new();
    h.update(person.0.as_bytes());
    h.update(event.0.as_bytes());
    h.update(group.0.as_bytes());
    h.update(&expectation_start.0.to_le_bytes());
    DerivedId(*h.finalize().as_bytes())
}

/// Digest of a derived-edge id set (ADR-0016 B): blake3 over the sorted
/// ids. Shared by the incremental chain and the cold recomputation path so
/// the fuzz comparison (SPEC-05 incremental correctness) compares the id
/// sets themselves.
pub fn digest_of_ids(ids: &[DerivedId]) -> [u8; 32] {
    let mut sorted: Vec<&DerivedId> = ids.iter().collect();
    sorted.sort();
    let mut h = blake3::Hasher::new();
    for id in sorted {
        h.update(&id.0);
    }
    *h.finalize().as_bytes()
}

/// A group-derived attendance edge, pre-reconciliation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DerivedAttends {
    pub id: DerivedId,
    pub person: PersonId,
    pub event: EventId,
    pub source_group: GroupId,
    /// Seeds `attends.priority_group` (ADR-0005).
    pub priority_group: f32,
    pub obligation: Obligation,
    /// The expectation's own window — not the event's (ADR-0003).
    pub during: Interval,
}

/// Q1: expand a person's group-derived attendance at instant `at`.
///
/// Steps per orrery/SPEC-02: memberships valid at `at` → ancestors through
/// `subgroup_of` edges each valid at `at` (depth ≤ 5, repository-enforced)
/// → expectations valid at `at`, honouring `cascades` — then one edge per
/// distinct event, keeping the highest `default_priority` (ties broken by
/// group id for determinism; recorded in phases/03-engine-core.md).
pub fn expand(
    repo: &dyn Repository,
    person: PersonId,
    at: Timestamp,
) -> Result<Vec<DerivedAttends>> {
    let _span = tracing::info_span!("derive.expand", person = %person).entered();

    let direct: Vec<GroupId> = repo
        .memberships(person, at)?
        .into_iter()
        .map(|m| m.group)
        .collect();

    // (group, reached-via-cascade-only?) — direct groups take expectations
    // unconditionally; strict ancestors only when the expectation cascades.
    let mut groups: Vec<(GroupId, bool)> = direct.iter().map(|g| (*g, false)).collect();
    for g in &direct {
        for anc in repo.group_ancestors(*g, at)? {
            if !groups.iter().any(|(x, _)| *x == anc) {
                groups.push((anc, !direct.contains(&anc)));
            }
        }
    }

    let ids: Vec<GroupId> = groups.iter().map(|(g, _)| *g).collect();
    let mut out: Vec<DerivedAttends> = Vec::new();
    for x in repo.expectations(&ids, at)? {
        let cascade_only = groups
            .iter()
            .find(|(g, _)| *g == x.group)
            .map(|(_, c)| *c)
            .unwrap_or(true);
        if cascade_only && !x.cascades {
            continue;
        }
        let candidate = DerivedAttends {
            id: derived_id(person, x.event, x.group, x.during.start()),
            person,
            event: x.event,
            source_group: x.group,
            priority_group: x.default_priority,
            obligation: x.obligation,
            during: x.during,
        };
        match out.iter_mut().find(|d| d.event == x.event) {
            None => out.push(candidate),
            Some(existing) => {
                let better = candidate.priority_group > existing.priority_group
                    || (candidate.priority_group == existing.priority_group
                        && candidate.source_group.0 < existing.source_group.0);
                if better {
                    *existing = candidate;
                }
            }
        }
    }
    // Deterministic output order regardless of repository iteration order.
    out.sort_by_key(|d| d.event.0);
    Ok(out)
}

/// A person's effective schedule: explicit attendance unioned with derived,
/// where an explicit edge for the same event shadows the derived one
/// (ADR-0004: user overrides survive recomputation; full pin-aware
/// reconciliation lands with caching in slice 2).
pub struct EffectiveSchedule {
    pub explicit: Vec<Attends>,
    pub derived: Vec<DerivedAttends>,
}

pub fn effective_schedule(
    repo: &dyn Repository,
    person: PersonId,
    window: Interval,
    at: Timestamp,
) -> Result<EffectiveSchedule> {
    let explicit = repo.attends_for(person, window)?;
    let derived = expand(repo, person, at)?
        .into_iter()
        .filter(|d| !explicit.iter().any(|a| a.event == d.event))
        .collect();
    Ok(EffectiveSchedule { explicit, derived })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::Command;
    use crate::model::{Actor, Group, MemberOf, Person, Role, SubgroupOf};
    use crate::repo::memory::MemoryRepo;
    use crate::repo::MAX_GROUP_DEPTH;
    use proptest::prelude::*;

    fn iv(s: i64, e: i64) -> Interval {
        Interval::new(Timestamp(s), Timestamp(e)).unwrap()
    }

    /// Naive oracle: full materialisation. Walks every group path with
    /// per-hop validity by direct edge scans (no repository), collects
    /// applicable expectations, keeps max-priority per event.
    #[allow(clippy::too_many_arguments)]
    fn oracle(
        person: PersonId,
        at: Timestamp,
        memberships: &[MemberOf],
        subgroups: &[SubgroupOf],
        expects: &[(GroupId, EventId, f32, bool, Interval)],
    ) -> Vec<(EventId, f32)> {
        let direct: Vec<GroupId> = memberships
            .iter()
            .filter(|m| m.person == person && m.during.contains_point(at))
            .map(|m| m.group)
            .collect();
        let mut reach: Vec<(GroupId, bool)> = direct.iter().map(|g| (*g, false)).collect();
        let mut frontier = direct.clone();
        for _ in 0..MAX_GROUP_DEPTH {
            let mut next = Vec::new();
            for e in subgroups {
                if frontier.contains(&e.child) && e.during.contains_point(at) {
                    let cascade_only = !direct.contains(&e.parent);
                    if !reach.iter().any(|(g, _)| *g == e.parent) {
                        reach.push((e.parent, cascade_only));
                        next.push(e.parent);
                    }
                }
            }
            frontier = next;
        }
        let mut best: Vec<(EventId, f32, GroupId)> = Vec::new();
        for (g, ev, prio, cascades, during) in expects {
            let Some((_, cascade_only)) = reach.iter().find(|(x, _)| x == g) else {
                continue;
            };
            if *cascade_only && !cascades {
                continue;
            }
            if !during.contains_point(at) {
                continue;
            }
            match best.iter_mut().find(|(e, _, _)| e == ev) {
                None => best.push((*ev, *prio, *g)),
                Some(entry) => {
                    if *prio > entry.1 || (*prio == entry.1 && g.0 < entry.2 .0) {
                        entry.1 = *prio;
                        entry.2 = *g;
                    }
                }
            }
        }
        let mut out: Vec<(EventId, f32)> = best.into_iter().map(|(e, p, _)| (e, p)).collect();
        out.sort_by_key(|(e, _)| e.0);
        out
    }

    fn seed_repo(
        person: PersonId,
        memberships: &[MemberOf],
        subgroups: &[SubgroupOf],
        expects: &[(GroupId, EventId, f32, bool, Interval)],
        groups: &[GroupId],
        events: &[EventId],
    ) -> MemoryRepo {
        let repo = MemoryRepo::new();
        repo.apply(Command::UpsertPerson(Person {
            id: person,
            name: "p".into(),
            derived_digest: None,
            ext: Default::default(),
        }))
        .unwrap();
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
            repo.apply(Command::UpsertEvent(crate::model::Event {
                id: *e,
                name: "e".into(),
                window: iv(0, 1000),
                kind: "k".into(),
                timezone: None,
                ext: Default::default(),
            }))
            .unwrap();
        }
        for m in memberships {
            repo.apply(Command::AddMembership {
                person: m.person,
                group: m.group,
                during: m.during,
                role: m.role.clone(),
            })
            .unwrap();
        }
        for s in subgroups {
            repo.apply(Command::AddSubgroup {
                child: s.child,
                parent: s.parent,
                during: s.during,
            })
            .unwrap();
        }
        for (g, e, prio, cascades, during) in expects {
            repo.apply(Command::AddExpectation {
                group: *g,
                event: *e,
                obligation: Obligation::Expected,
                default_priority: *prio,
                during: *during,
                cascades: *cascades,
                by: Actor::System,
            })
            .unwrap();
        }
        repo
    }

    proptest! {
        /// Random 3-level group chains, random validity windows on every
        /// hop, random cascade flags: expand ≡ naive materialisation.
        #[test]
        fn prop_expand_matches_oracle(
            edge_windows in proptest::collection::vec((0i64..200, 1i64..200), 3),
            exp_specs in proptest::collection::vec(
                (0usize..3, 0i64..200, 1i64..200, 0u8..100, prop::bool::ANY), 0..8),
            at in 50i64..250,
        ) {
            let person = PersonId::new();
            let at = Timestamp(at);
            let g: Vec<GroupId> = (0..3).map(|_| GroupId::new()).collect();
            let memberships = vec![MemberOf {
                person, group: g[0],
                during: iv(edge_windows[0].0, edge_windows[0].0 + edge_windows[0].1),
                role: Role::Member,
            }];
            let subgroups = vec![
                SubgroupOf { child: g[0], parent: g[1],
                    during: iv(edge_windows[1].0, edge_windows[1].0 + edge_windows[1].1) },
                SubgroupOf { child: g[1], parent: g[2],
                    during: iv(edge_windows[2].0, edge_windows[2].0 + edge_windows[2].1) },
            ];
            let events: Vec<EventId> = (0..exp_specs.len()).map(|_| EventId::new()).collect();
            let expects: Vec<(GroupId, EventId, f32, bool, Interval)> = exp_specs
                .iter()
                .zip(&events)
                .map(|((gi, s, len, prio, casc), ev)| {
                    (g[*gi], *ev, (*prio as f32) / 100.0, *casc, iv(*s, s + len))
                })
                .collect();

            let repo = seed_repo(person, &memberships, &subgroups, &expects, &g, &events);
            let got: Vec<(EventId, f32)> = expand(&repo, person, at)
                .unwrap()
                .into_iter()
                .map(|d| (d.event, d.priority_group))
                .collect();
            prop_assert_eq!(got, oracle(person, at, &memberships, &subgroups, &expects));
        }

        /// ADR-0016 A: identity is bit-stable across recomputation.
        #[test]
        fn prop_derived_id_stable(at in 0i64..100) {
            let (p, e, g) = (PersonId::new(), EventId::new(), GroupId::new());
            prop_assert_eq!(
                derived_id(p, e, g, Timestamp(at)),
                derived_id(p, e, g, Timestamp(at))
            );
        }
    }

    /// cascades=false on an ancestor's expectation must not reach a
    /// subgroup member; the same expectation on a direct group applies.
    #[test]
    fn cascade_flag_respected() {
        let person = PersonId::new();
        let (child, parent) = (GroupId::new(), GroupId::new());
        let ev = EventId::new();
        let memberships = [MemberOf {
            person,
            group: child,
            during: iv(0, 100),
            role: Role::Member,
        }];
        let subgroups = [SubgroupOf {
            child,
            parent,
            during: iv(0, 100),
        }];
        let expects = [(parent, ev, 0.5f32, false, iv(0, 100))];
        let repo = seed_repo(
            person,
            &memberships,
            &subgroups,
            &expects,
            &[child, parent],
            &[ev],
        );
        assert!(
            expand(&repo, person, Timestamp(50)).unwrap().is_empty(),
            "non-cascading ancestor expectation must not flow down"
        );
    }

    /// Explicit attendance shadows derived for the same event.
    #[test]
    fn explicit_shadows_derived() {
        let person = PersonId::new();
        let g = GroupId::new();
        let ev = EventId::new();
        let memberships = [MemberOf {
            person,
            group: g,
            during: iv(0, 100),
            role: Role::Member,
        }];
        let expects = [(g, ev, 0.5f32, true, iv(0, 100))];
        let repo = seed_repo(person, &memberships, &[], &expects, &[g], &[ev]);
        repo.apply(Command::AddAttendance {
            person,
            event: ev,
            priority: Some(0.9),
        })
        .unwrap();

        let sched = effective_schedule(&repo, person, iv(0, 1000), Timestamp(50)).unwrap();
        assert_eq!(sched.explicit.len(), 1);
        assert!(
            sched.derived.is_empty(),
            "derived edge shadowed by explicit"
        );
    }
}
