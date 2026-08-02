//! `impossible_travel`: consecutive events where the gap is shorter than
//! the travel cost between their locations. Partition: person.
//!
//! **Consecutive pairs, not all ordered pairs** (orrery/SPEC-02): if A→C is
//! infeasible but B occurs between them and A→B→C is feasible, that is not
//! a violation. Overlapping events are time_conflict's domain, not travel's.
//!
//! Signature carries `person` now per Rule 00.5 — unused until mobility
//! profiles (ADR-0017); travel caches key on `(profile_id, from, to)` with
//! everyone sharing `default` (Phase 4).

use crate::detect::ViolationDraft;
use crate::interval::Interval;
use crate::model::{
    EntityRef, EventId, LocationId, PersonId, Severity, TravelCost, TravelProvenance, ViolationKind,
};

/// An event with a resolved location — the caller joins attends → event →
/// held.
#[derive(Debug, Clone, PartialEq)]
pub struct Placed {
    pub event: EventId,
    pub location: LocationId,
    pub window: Interval,
}

/// `travel` is a pure lookup (Layer-2 point read); `None` means no known
/// route — an incomplete closure must not spray false positives (design
/// decision in phases/03-engine-core.md), so no draft is emitted.
pub fn detect(
    person: PersonId,
    placed: &[Placed],
    travel: &dyn Fn(LocationId, LocationId) -> Option<TravelCost>,
) -> Vec<ViolationDraft> {
    let _ = person; // Rule 00.5: signature lands now, mobility later.
    let mut sorted: Vec<&Placed> = placed.iter().collect();
    sorted.sort_by_key(|p| (p.window.start(), p.window.end()));

    let mut out = Vec::new();
    for pair in sorted.windows(2) {
        let (a, b) = (pair[0], pair[1]);
        if b.window.start() < a.window.end() {
            continue; // overlap → time_conflict's domain
        }
        if a.location == b.location {
            continue;
        }
        let Some(cost) = travel(a.location, b.location) else {
            continue;
        };
        let gap_micros = b.window.start().micros() - a.window.end().micros();
        if gap_micros < cost.duration_s * 1_000_000 {
            out.push(ViolationDraft {
                kind: ViolationKind::ImpossibleTravel,
                // SPEC-03: more conservative on estimates.
                severity: match cost.provenance {
                    TravelProvenance::Measured => Severity::Hard,
                    TravelProvenance::Estimated => Severity::Warning,
                },
                subjects: vec![
                    EntityRef::Person(person),
                    EntityRef::Event(a.event),
                    EntityRef::Event(b.event),
                ],
            });
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interval::Timestamp;
    use proptest::prelude::*;

    fn placed(location: LocationId, s: i64, e: i64) -> Placed {
        Placed {
            event: EventId::new(),
            location,
            window: Interval::new(Timestamp(s), Timestamp(e)).unwrap(),
        }
    }

    fn const_travel(secs: i64) -> impl Fn(LocationId, LocationId) -> Option<TravelCost> {
        move |_, _| {
            Some(TravelCost {
                duration_s: secs,
                provenance: TravelProvenance::Measured,
            })
        }
    }

    /// Oracle: re-derive "consecutive" independently — for each
    /// non-overlapping ordered pair (a, b), b is a's successor iff no third
    /// event starts in [a.end, b.start).
    fn oracle(placed: &[Placed], travel_s: i64) -> usize {
        let mut n = 0;
        for a in placed {
            for b in placed {
                if std::ptr::eq(a, b) || b.window.start() < a.window.end() {
                    continue;
                }
                let between = placed.iter().any(|c| {
                    !std::ptr::eq(c, a)
                        && !std::ptr::eq(c, b)
                        && c.window.start() >= a.window.end()
                        && c.window.start() < b.window.start()
                });
                if !between
                    && a.location != b.location
                    && (b.window.start().micros() - a.window.end().micros()) < travel_s * 1_000_000
                {
                    n += 1;
                }
            }
        }
        n
    }

    proptest! {
        /// Non-overlapping schedules (built by accumulation) so the
        /// consecutive relation is unambiguous, random locations from a
        /// pool of 3, fixed 300 s travel.
        #[test]
        fn prop_matches_oracle(
            gaps in proptest::collection::vec((0i64..600, 1i64..600, 0usize..3), 1..8)
        ) {
            let pool = [LocationId::new(), LocationId::new(), LocationId::new()];
            let mut t = 0i64;
            let mut world = Vec::new();
            for (gap, dur, li) in &gaps {
                let start = t + gap * 1_000_000;
                let end = start + dur * 1_000_000;
                world.push(placed(pool[*li], start, end));
                t = end;
            }
            let got = detect(PersonId::new(), &world, &const_travel(300)).len();
            prop_assert_eq!(got, oracle(&world, 300));
        }
    }

    /// ADR-0024 spring-forward fixture: 2026-03-08, America/New_York.
    /// A ends 06:55 UTC (01:55 EST); B starts 07:05 UTC (03:05 EDT).
    /// Wall-clock rendering suggests a 70-minute gap; the true UTC gap is
    /// 10 minutes. With 30-minute travel, the violation MUST fire.
    #[test]
    fn dst_spring_forward_fires_on_true_gap() {
        const MAR8_0655_UTC: i64 = 1_772_952_900 * 1_000_000;
        const MIN: i64 = 60 * 1_000_000;
        let (r1, r2) = (LocationId::new(), LocationId::new());
        let world = vec![
            Placed {
                event: EventId::new(),
                location: r1,
                window: Interval::new(
                    Timestamp(MAR8_0655_UTC - 60 * MIN),
                    Timestamp(MAR8_0655_UTC),
                )
                .unwrap(),
            },
            Placed {
                event: EventId::new(),
                location: r2,
                window: Interval::new(
                    Timestamp(MAR8_0655_UTC + 10 * MIN),
                    Timestamp(MAR8_0655_UTC + 70 * MIN),
                )
                .unwrap(),
            },
        ];
        let drafts = detect(PersonId::new(), &world, &const_travel(30 * 60));
        assert_eq!(drafts.len(), 1, "10-minute true gap < 30-minute travel");
    }

    /// A→C looks infeasible, but B sits between and both legs are fine.
    #[test]
    fn intermediate_event_breaks_the_pair() {
        let (ra, rb, rc) = (LocationId::new(), LocationId::new(), LocationId::new());
        let m = 1_000_000i64;
        let world = vec![
            placed(ra, 0, 600 * m),
            placed(rb, 1200 * m, 1800 * m),
            placed(rc, 2400 * m, 3000 * m),
        ];
        // 300 s travel: A→B gap 600 s ok, B→C gap 600 s ok; A→C never paired.
        assert!(detect(PersonId::new(), &world, &const_travel(300)).is_empty());
    }

    #[test]
    fn unknown_route_is_not_a_violation() {
        let world = vec![
            placed(LocationId::new(), 0, 1_000_000),
            placed(LocationId::new(), 2_000_000, 3_000_000),
        ];
        let none = |_: LocationId, _: LocationId| -> Option<TravelCost> { None };
        assert!(detect(PersonId::new(), &world, &none).is_empty());
    }
}
