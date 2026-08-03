//! Analytics surface (Phase 6a; orrery/SPEC-02 §Analytics; ROADMAP
//! boundary matrix — orrery owns these, Muster Beta consumes them).
//!
//! Pure read functions over the repository seam: no datastore type, no
//! I/O, no clock. Definitions pre-committed in
//! phases/06a-engine-surfaces.md; each ships with a brute-force oracle
//! property test (the detector discipline, applied to analytics).
//!
//! Group roll-ups resolve members through `memberships_all` — a
//! population-scale read: these are batch/reporting surfaces, not
//! interactive ones (orrery/SPEC-03 keeps those budgets separate).

use crate::derive;
use crate::detect::capacity_exceeded;
use crate::error::Result;
use crate::interval::{Interval, Timestamp};
use crate::model::{EventId, GroupId, PersonId};
use crate::repo::Repository;

/// Priority-weighted attendance count for one person over `window`
/// (SPEC-02): the sum of `effective_priority()` over the explicit edges of
/// the person's effective schedule, plus the seeded `priority_group` of
/// each non-shadowed derived edge whose EVENT overlaps `window` — the
/// derived edge's own `during` is the expectation's window (ADR-0003), so
/// the event lookup is what keeps "attendance in this window" honest.
pub fn engagement(
    repo: &dyn Repository,
    person: PersonId,
    window: Interval,
    at: Timestamp,
) -> Result<f64> {
    let _span = tracing::info_span!("analytics.engagement").entered();
    let sched = derive::effective_schedule(repo, person, window, at)?;
    let mut score = 0.0f64;
    for a in &sched.explicit {
        score += a.effective_priority() as f64;
    }
    for d in &sched.derived {
        let Some(ev) = repo.event(d.event)? else {
            continue;
        };
        if ev.window.overlaps(&window) {
            score += d.priority_group as f64;
        }
    }
    Ok(score)
}

/// [`engagement`] for every member of `group` valid at `at`, sorted by
/// person id.
pub fn engagement_by_group(
    repo: &dyn Repository,
    group: GroupId,
    window: Interval,
    at: Timestamp,
) -> Result<Vec<(PersonId, f64)>> {
    let _span = tracing::info_span!("analytics.engagement_by_group").entered();
    members_at(repo, group, at)?
        .into_iter()
        .map(|p| engagement(repo, p, window, at).map(|s| (p, s)))
        .collect()
}

/// Signalled interest vs. allocated capacity for one event (SPEC-02,
/// ADR-0018: a ranking signal, not a forecast).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapacityPressure {
    pub event: EventId,
    /// Attends edges with effective priority strictly above the threshold
    /// — the same count the capacity detector uses.
    pub signalled: u32,
    /// Σ per-hold `capacity_override ?? location.capacity` over the holds
    /// that resolve a capacity; `None` when none do (unheld event, or no
    /// capacity data anywhere).
    pub allocated: Option<u32>,
}

/// Capacity pressure for every event overlapping `window`, in the browse
/// order (`events_in`: window start, then id). `interest_threshold` is
/// **caller-supplied** — the ADR-0018 attendance-model hook at primitive
/// level: the engine ships count-above-threshold, the consumer owns the
/// model. The sweep's detector keeps its own default (0.0) unchanged.
pub fn capacity_pressure(
    repo: &dyn Repository,
    window: Interval,
    interest_threshold: f32,
) -> Result<Vec<CapacityPressure>> {
    let _span = tracing::info_span!("analytics.capacity_pressure").entered();
    let mut out = Vec::new();
    for ev in repo.events_in(window)? {
        let attends = repo.attends_for_event(ev.id)?;
        let signalled = capacity_exceeded::signalled_interest(ev.id, &attends, interest_threshold);
        let mut allocated: Option<u32> = None;
        for h in repo.held_for_event(ev.id)? {
            let cap = match h.capacity_override {
                Some(c) => Some(c),
                None => repo.location(h.location)?.and_then(|l| l.capacity),
            };
            if let Some(c) = cap {
                allocated = Some(allocated.unwrap_or(0) + c);
            }
        }
        out.push(CapacityPressure {
            event: ev.id,
            signalled,
            allocated,
        });
    }
    Ok(out)
}

/// `|priority_coord − priority_person|` aggregated per group (SPEC-02) —
/// the analytic that exists only because the priority stack keeps its
/// components separate (ADR-0005).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DivergenceSummary {
    /// Edges considered: members' explicit attends overlapping `window`
    /// where both priorities are present.
    pub edges: usize,
    pub mean: Option<f64>,
    pub max: Option<f64>,
}

pub fn divergence(
    repo: &dyn Repository,
    group: GroupId,
    window: Interval,
    at: Timestamp,
) -> Result<DivergenceSummary> {
    let _span = tracing::info_span!("analytics.divergence").entered();
    let mut sum = 0.0f64;
    let mut max = f64::MIN;
    let mut edges = 0usize;
    for p in members_at(repo, group, at)? {
        for a in repo.attends_for(p, window)? {
            if let Some(d) = a.divergence() {
                let d = d as f64;
                sum += d;
                max = max.max(d);
                edges += 1;
            }
        }
    }
    Ok(DivergenceSummary {
        edges,
        mean: (edges > 0).then(|| sum / edges as f64),
        max: (edges > 0).then_some(max),
    })
}

/// Bounded 2-hop co-attendance with a time window (ADR-0020: the analytic
/// that replaced unbounded cascade — a fixed-depth bipartite join):
/// person → their attends edges overlapping `window` → those events'
/// attends edges overlapping `window` → distinct other persons, sorted.
///
/// Explicit edges only: the ADR-0020 measured analytic was edge-based,
/// and derived co-attendance would require population-scale expansion
/// (recorded in phases/06a-engine-surfaces.md — revisit on demand).
/// Budget: < 50 ms p95 (orrery/SPEC-03).
pub fn co_attendance(
    repo: &dyn Repository,
    person: PersonId,
    window: Interval,
) -> Result<Vec<PersonId>> {
    let _span = tracing::info_span!("analytics.co_attendance").entered();
    let mut out = Vec::new();
    for a in repo.attends_for(person, window)? {
        for b in repo.attends_for_event(a.event)? {
            if b.person != person && b.during.overlaps(&window) {
                out.push(b.person);
            }
        }
    }
    out.sort();
    out.dedup();
    Ok(out)
}

/// Members of `group` valid at `at`, sorted and deduplicated. A
/// population-scale scan by design — see the module note.
fn members_at(repo: &dyn Repository, group: GroupId, at: Timestamp) -> Result<Vec<PersonId>> {
    let mut persons: Vec<PersonId> = repo
        .memberships_all()?
        .into_iter()
        .filter(|m| m.group == group && m.during.contains_point(at))
        .map(|m| m.person)
        .collect();
    persons.sort();
    persons.dedup();
    Ok(persons)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::Command;
    use crate::model::{Actor, Event, Group, Location, LocationId, Obligation, Person, Role, Tier};
    use crate::repo::memory::MemoryRepo;
    use proptest::prelude::*;

    fn iv(s: i64, e: i64) -> Interval {
        Interval::new(Timestamp(s), Timestamp(e)).unwrap()
    }

    fn seed_entities(
        repo: &MemoryRepo,
        persons: &[PersonId],
        groups: &[GroupId],
        events: &[(EventId, Interval)],
    ) {
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
        for (e, w) in events {
            repo.apply(Command::UpsertEvent(Event {
                id: *e,
                name: "e".into(),
                window: *w,
                kind: "k".into(),
                timezone: None,
                ext: Default::default(),
            }))
            .unwrap();
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(64))]

        /// Engagement ≡ naive recomputation on direct-membership worlds
        /// (subgroup traversal has its own oracle suite in `derive`): sum
        /// of explicit effective priorities over window-overlapping edges,
        /// plus the winning direct expectation per non-shadowed event whose
        /// event window overlaps.
        #[test]
        fn prop_engagement_matches_oracle(
            member_windows in proptest::collection::vec((0i64..200, 1i64..200), 3),
            event_windows in proptest::collection::vec((0i64..300, 1i64..200), 5),
            expect_specs in proptest::collection::vec(
                (0usize..3, 0usize..5, 0u8..100, 0i64..250, 1i64..200), 0..8),
            attend_specs in proptest::collection::vec((0usize..5, 0u8..100), 0..5),
            window in (0i64..200, 1i64..300),
            at in 0i64..300,
        ) {
            let person = PersonId::new();
            let groups: Vec<GroupId> = (0..3).map(|_| GroupId::new()).collect();
            let events: Vec<(EventId, Interval)> = event_windows
                .iter()
                .map(|(s, l)| (EventId::new(), iv(*s, s + l)))
                .collect();
            let window = iv(window.0, window.0 + window.1);
            let at_t = Timestamp(at);

            let repo = MemoryRepo::new();
            seed_entities(&repo, &[person], &groups, &events);
            for (g, (s, l)) in groups.iter().zip(&member_windows) {
                repo.apply(Command::AddMembership {
                    person,
                    group: *g,
                    during: iv(*s, s + l),
                    role: Role::Member,
                })
                .unwrap();
            }
            for (gi, ei, prio, s, l) in &expect_specs {
                repo.apply(Command::AddExpectation {
                    group: groups[*gi],
                    event: events[*ei].0,
                    obligation: Obligation::Expected,
                    default_priority: (*prio as f32) / 100.0,
                    during: iv(*s, s + l),
                    cascades: false,
                    by: Actor::System,
                })
                .unwrap();
            }
            let mut attended: Vec<(EventId, f32)> = Vec::new();
            for (ei, prio) in &attend_specs {
                let (eid, _) = events[*ei];
                if attended.iter().any(|(e, _)| *e == eid) {
                    continue;
                }
                let p = (*prio as f32) / 100.0;
                repo.apply(Command::AddAttendance {
                    person,
                    event: eid,
                    priority: Some(p),
                })
                .unwrap();
                attended.push((eid, p));
            }

            // Oracle. Explicit: attends.during == the event's window, so an
            // edge counts iff its event window overlaps the query window.
            let mut want = 0.0f64;
            for (eid, p) in &attended {
                let (_, ew) = events.iter().find(|(e, _)| e == eid).unwrap();
                if ew.overlaps(&window) {
                    want += *p as f64;
                }
            }
            // Derived: per event not explicitly attended, the winning
            // (max-priority, ties to smaller group id) expectation among
            // groups whose membership is valid at `at`, expectation valid
            // at `at`, event window overlapping the query window.
            for (ei, (eid, ew)) in events.iter().enumerate() {
                if attended.iter().any(|(e, _)| e == eid) || !ew.overlaps(&window) {
                    continue;
                }
                let mut best: Option<(f32, GroupId)> = None;
                for (gi, xei, prio, s, l) in &expect_specs {
                    if *xei != ei {
                        continue;
                    }
                    let (ms, ml) = member_windows[*gi];
                    let member_ok = ms <= at && at < ms + ml;
                    let expect_ok = *s <= at && at < s + l;
                    if !member_ok || !expect_ok {
                        continue;
                    }
                    let p = (*prio as f32) / 100.0;
                    let g = groups[*gi];
                    best = match best {
                        None => Some((p, g)),
                        Some((bp, bg)) if p > bp || (p == bp && g.0 < bg.0) => Some((p, g)),
                        keep => keep,
                    };
                }
                if let Some((p, _)) = best {
                    want += p as f64;
                }
            }

            let got = engagement(&repo, person, window, at_t).unwrap();
            prop_assert!((got - want).abs() < 1e-9, "got {got}, want {want}");
        }

        /// Capacity pressure ≡ naive recount: strictly-above-threshold
        /// interest per event, override-else-base capacity summed over
        /// resolvable holds, None when nothing resolves.
        #[test]
        fn prop_capacity_pressure_matches_oracle(
            event_windows in proptest::collection::vec((0i64..300, 1i64..200), 4),
            attend_specs in proptest::collection::vec((0usize..4, 0usize..6, 0u8..100), 0..12),
            hold_specs in proptest::collection::vec(
                (0usize..4, prop::bool::ANY, proptest::option::of(1u32..50)), 0..6),
            threshold in 0u8..100,
            window in (0i64..200, 1i64..300),
        ) {
            let persons: Vec<PersonId> = (0..6).map(|_| PersonId::new()).collect();
            let events: Vec<(EventId, Interval)> = event_windows
                .iter()
                .map(|(s, l)| (EventId::new(), iv(*s, s + l)))
                .collect();
            let window = iv(window.0, window.0 + window.1);
            let threshold = (threshold as f32) / 100.0;

            // Two locations: one with base capacity, one without.
            let with_cap = Location {
                id: LocationId::new(),
                name: "cap".into(),
                tier: Tier::Room,
                portal: crate::model::Portal::None,
                capacity: Some(10),
                ext: Default::default(),
            };
            let no_cap = Location {
                id: LocationId::new(),
                name: "nocap".into(),
                tier: Tier::Room,
                portal: crate::model::Portal::None,
                capacity: None,
                ext: Default::default(),
            };
            let (cap_id, nocap_id) = (with_cap.id, no_cap.id);

            let repo = MemoryRepo::new();
            seed_entities(&repo, &persons, &[], &events);
            repo.apply(Command::UpsertLocation(with_cap)).unwrap();
            repo.apply(Command::UpsertLocation(no_cap)).unwrap();

            let mut attend_pairs: Vec<(usize, usize, f32)> = Vec::new();
            for (ei, pi, prio) in &attend_specs {
                if attend_pairs.iter().any(|(e, p, _)| e == ei && p == pi) {
                    continue;
                }
                let p = (*prio as f32) / 100.0;
                repo.apply(Command::AddAttendance {
                    person: persons[*pi],
                    event: events[*ei].0,
                    priority: Some(p),
                })
                .unwrap();
                attend_pairs.push((*ei, *pi, p));
            }
            let mut holds: Vec<(usize, LocationId, Option<u32>)> = Vec::new();
            for (ei, use_cap_loc, over) in &hold_specs {
                let loc = if *use_cap_loc { cap_id } else { nocap_id };
                repo.apply(Command::HoldLocation {
                    location: loc,
                    event: events[*ei].0,
                    during: events[*ei].1,
                    overflow_for: None,
                    capacity_override: *over,
                })
                .unwrap();
                holds.push((*ei, loc, *over));
            }

            let got = capacity_pressure(&repo, window, threshold).unwrap();

            // Oracle over the same event set, independently ordered.
            let mut want: Vec<CapacityPressure> = Vec::new();
            let mut order: Vec<usize> = (0..events.len())
                .filter(|ei| events[*ei].1.overlaps(&window))
                .collect();
            order.sort_by_key(|ei| (events[*ei].1.start(), events[*ei].0));
            for ei in order {
                let signalled = attend_pairs
                    .iter()
                    .filter(|(e, _, p)| *e == ei && *p > threshold)
                    .count() as u32;
                let mut allocated: Option<u32> = None;
                for (he, loc, over) in &holds {
                    if *he != ei {
                        continue;
                    }
                    let cap = over.or(if *loc == cap_id { Some(10) } else { None });
                    if let Some(c) = cap {
                        allocated = Some(allocated.unwrap_or(0) + c);
                    }
                }
                want.push(CapacityPressure {
                    event: events[ei].0,
                    signalled,
                    allocated,
                });
            }
            prop_assert_eq!(got, want);
        }

        /// Divergence ≡ naive recount over members' window-overlapping
        /// edges where both priorities exist.
        #[test]
        fn prop_divergence_matches_oracle(
            member_flags in proptest::collection::vec(prop::bool::ANY, 4),
            event_windows in proptest::collection::vec((0i64..300, 1i64..200), 4),
            attend_specs in proptest::collection::vec(
                (0usize..4, 0usize..4,
                 proptest::option::of(0u8..100), proptest::option::of(0u8..100)), 0..10),
            window in (0i64..200, 1i64..300),
        ) {
            let persons: Vec<PersonId> = (0..4).map(|_| PersonId::new()).collect();
            let group = GroupId::new();
            let events: Vec<(EventId, Interval)> = event_windows
                .iter()
                .map(|(s, l)| (EventId::new(), iv(*s, s + l)))
                .collect();
            let window = iv(window.0, window.0 + window.1);
            let at_t = Timestamp(100);

            let repo = MemoryRepo::new();
            seed_entities(&repo, &persons, &[group], &events);
            for (p, is_member) in persons.iter().zip(&member_flags) {
                if *is_member {
                    repo.apply(Command::AddMembership {
                        person: *p,
                        group,
                        during: iv(0, 200),
                        role: Role::Member,
                    })
                    .unwrap();
                }
            }
            let mut edges: Vec<(usize, usize, Option<f32>, Option<f32>)> = Vec::new();
            for (pi, ei, pp, pc) in &attend_specs {
                if edges.iter().any(|(p, e, ..)| p == pi && e == ei) {
                    continue;
                }
                let pp = pp.map(|v| (v as f32) / 100.0);
                let pc = pc.map(|v| (v as f32) / 100.0);
                repo.apply(Command::AddAttendance {
                    person: persons[*pi],
                    event: events[*ei].0,
                    priority: pp,
                })
                .unwrap();
                if let Some(c) = pc {
                    repo.apply(Command::SetPriority {
                        person: persons[*pi],
                        event: events[*ei].0,
                        by: Actor::Coordinator(persons[0]),
                        binding: false,
                        value: c,
                    })
                    .unwrap();
                }
                edges.push((*pi, *ei, pp, pc));
            }

            let got = divergence(&repo, group, window, at_t).unwrap();

            let mut divs: Vec<f64> = Vec::new();
            for (pi, ei, pp, pc) in &edges {
                if !member_flags[*pi] || !events[*ei].1.overlaps(&window) {
                    continue;
                }
                if let (Some(p), Some(c)) = (pp, pc) {
                    divs.push((c - p).abs() as f64);
                }
            }
            prop_assert_eq!(got.edges, divs.len());
            match (got.mean, got.max) {
                (None, None) => prop_assert!(divs.is_empty()),
                (Some(mean), Some(max)) => {
                    let want_mean: f64 = divs.iter().sum::<f64>() / divs.len() as f64;
                    let want_max = divs.iter().cloned().fold(f64::MIN, f64::max);
                    prop_assert!((mean - want_mean).abs() < 1e-9);
                    prop_assert!((max - want_max).abs() < 1e-12);
                }
                other => prop_assert!(false, "inconsistent summary: {:?}", other),
            }
        }

        /// 2-hop co-attendance ≡ the naive bipartite double loop.
        #[test]
        fn prop_co_attendance_matches_oracle(
            event_windows in proptest::collection::vec((0i64..300, 1i64..200), 5),
            attend_specs in proptest::collection::vec((0usize..5, 0usize..5), 0..15),
            window in (0i64..200, 1i64..300),
        ) {
            let persons: Vec<PersonId> = (0..5).map(|_| PersonId::new()).collect();
            let events: Vec<(EventId, Interval)> = event_windows
                .iter()
                .map(|(s, l)| (EventId::new(), iv(*s, s + l)))
                .collect();
            let window = iv(window.0, window.0 + window.1);

            let repo = MemoryRepo::new();
            seed_entities(&repo, &persons, &[], &events);
            let mut pairs: Vec<(usize, usize)> = Vec::new();
            for (pi, ei) in &attend_specs {
                if pairs.contains(&(*pi, *ei)) {
                    continue;
                }
                repo.apply(Command::AddAttendance {
                    person: persons[*pi],
                    event: events[*ei].0,
                    priority: None,
                })
                .unwrap();
                pairs.push((*pi, *ei));
            }

            let got = co_attendance(&repo, persons[0], window).unwrap();

            let mut want: Vec<PersonId> = Vec::new();
            for (q, qid) in persons.iter().enumerate().skip(1) {
                let shares = pairs.iter().any(|(pi, ei)| {
                    *pi == 0
                        && events[*ei].1.overlaps(&window)
                        && pairs.contains(&(q, *ei))
                });
                if shares {
                    want.push(*qid);
                }
            }
            want.sort();
            prop_assert_eq!(got, want);
        }
    }

    /// Empty worlds: zero engagement, empty pressure, empty co-attendance,
    /// and a divergence summary that says so rather than faking zeros.
    #[test]
    fn empty_world_summaries_are_honest() {
        let repo = MemoryRepo::new();
        let p = PersonId::new();
        let g = GroupId::new();
        seed_entities(&repo, &[p], &[g], &[]);
        let w = iv(0, 100);
        let t = Timestamp(50);
        assert_eq!(engagement(&repo, p, w, t).unwrap(), 0.0);
        assert!(capacity_pressure(&repo, w, 0.0).unwrap().is_empty());
        assert!(co_attendance(&repo, p, w).unwrap().is_empty());
        let d = divergence(&repo, g, w, t).unwrap();
        assert_eq!(
            d,
            DivergenceSummary {
                edges: 0,
                mean: None,
                max: None
            }
        );
    }
}
