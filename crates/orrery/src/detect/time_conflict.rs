//! `time_conflict`: two `attends` for one person with overlapping `during`.
//! Partition: person.

use crate::detect::{ViolationDraft, SEVERITY_TIME_CONFLICT};
use crate::model::{Attends, EntityRef, PersonId, ViolationKind};

/// Pure over one person's attends edges (entity-partitioned input, like the
/// query that feeds it). Pairs are reported once, ordered by input index.
pub fn detect(person: PersonId, attends: &[Attends]) -> Vec<ViolationDraft> {
    let mine: Vec<&Attends> = attends.iter().filter(|a| a.person == person).collect();
    let mut out = Vec::new();
    for i in 0..mine.len() {
        for j in (i + 1)..mine.len() {
            let (a, b) = (mine[i], mine[j]);
            if a.event != b.event && a.during.overlaps(&b.during) {
                out.push(ViolationDraft {
                    kind: ViolationKind::TimeConflict,
                    severity: SEVERITY_TIME_CONFLICT,
                    subjects: vec![
                        EntityRef::Person(person),
                        EntityRef::Event(a.event),
                        EntityRef::Event(b.event),
                    ],
                });
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interval::{Interval, Timestamp};
    use crate::model::{AttendanceSource, EventId};
    use proptest::prelude::*;

    fn att(person: PersonId, event: EventId, s: i64, e: i64) -> Attends {
        Attends {
            person,
            event,
            during: Interval::new(Timestamp(s), Timestamp(e)).unwrap(),
            priority_group: 0.0,
            priority_person: None,
            priority_coord: None,
            coord_binding: false,
            source: AttendanceSource::SelfSelected,
            pinned: false,
        }
    }

    /// Independent oracle: shared-instant test on every unordered pair.
    fn oracle(person: PersonId, attends: &[Attends]) -> usize {
        let mine: Vec<&Attends> = attends.iter().filter(|a| a.person == person).collect();
        let mut n = 0;
        for i in 0..mine.len() {
            for j in 0..mine.len() {
                if i < j
                    && mine[i].event != mine[j].event
                    && mine[i].during.start().max(mine[j].during.start())
                        < mine[i].during.end().min(mine[j].during.end())
                {
                    n += 1;
                }
            }
        }
        n
    }

    proptest! {
        #[test]
        fn prop_matches_oracle(windows in proptest::collection::vec((0i64..500, 1i64..100), 0..12)) {
            let p = PersonId::new();
            let attends: Vec<Attends> = windows
                .iter()
                .map(|(s, len)| att(p, EventId::new(), *s, s + len))
                .collect();
            prop_assert_eq!(detect(p, &attends).len(), oracle(p, &attends));
        }
    }

    /// ADR-0024 fall-back fixture: 2026-11-01, America/New_York. Event C
    /// 05:00–06:00 UTC (01:00–02:00 EDT) and event D 06:00–07:00 UTC
    /// (01:00–02:00 EST — the repeated wall-clock hour). Wall-clock
    /// rendering suggests a full overlap; in UTC they merely meet. Must NOT
    /// fire.
    #[test]
    fn dst_fall_back_no_false_conflict() {
        const NOV1_0500_UTC: i64 = 1_793_509_200 * 1_000_000;
        const HOUR: i64 = 3_600 * 1_000_000;
        let p = PersonId::new();
        let attends = vec![
            att_us(p, EventId::new(), NOV1_0500_UTC, NOV1_0500_UTC + HOUR),
            att_us(
                p,
                EventId::new(),
                NOV1_0500_UTC + HOUR,
                NOV1_0500_UTC + 2 * HOUR,
            ),
        ];
        assert!(
            detect(p, &attends).is_empty(),
            "meets-in-UTC is not a conflict"
        );
    }

    fn att_us(person: PersonId, event: EventId, s: i64, e: i64) -> Attends {
        Attends {
            during: Interval::new(Timestamp(s), Timestamp(e)).unwrap(),
            ..att(person, event, 0, 1)
        }
    }

    #[test]
    fn same_event_twice_is_not_a_conflict() {
        let p = PersonId::new();
        let e = EventId::new();
        let attends = vec![att(p, e, 0, 10), att(p, e, 5, 15)];
        assert!(detect(p, &attends).is_empty());
    }
}
