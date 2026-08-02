//! `location_exclusivity`: two `held` into one location with overlapping
//! `during`, different events. Partition: location.
//!
//! Structurally identical to person time-conflict — same predicate, same
//! shape (ADR-0011). The "legitimate declared overflow" case (one event
//! held in a primary room and an overflow room) involves two *locations*
//! and therefore never produces a same-location pair; no special exclusion
//! exists (design decision recorded in phases/03-engine-core.md).

use crate::detect::{ViolationDraft, SEVERITY_LOCATION_EXCLUSIVITY};
use crate::model::{EntityRef, Held, LocationId, ViolationKind};

pub fn detect(location: LocationId, holds: &[Held]) -> Vec<ViolationDraft> {
    let here: Vec<&Held> = holds.iter().filter(|h| h.location == location).collect();
    let mut out = Vec::new();
    for i in 0..here.len() {
        for j in (i + 1)..here.len() {
            let (a, b) = (here[i], here[j]);
            if a.event != b.event && a.during.overlaps(&b.during) {
                out.push(ViolationDraft {
                    kind: ViolationKind::LocationExclusivity,
                    severity: SEVERITY_LOCATION_EXCLUSIVITY,
                    subjects: vec![
                        EntityRef::Location(location),
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
    use crate::model::{EventId, Posture};
    use proptest::prelude::*;

    fn hold(location: LocationId, event: EventId, s: i64, e: i64) -> Held {
        Held {
            location,
            event,
            during: Interval::new(Timestamp(s), Timestamp(e)).unwrap(),
            posture: Posture::OnSite,
            overflow_for: None,
            capacity_override: None,
        }
    }

    fn oracle(location: LocationId, holds: &[Held]) -> usize {
        let here: Vec<&Held> = holds.iter().filter(|h| h.location == location).collect();
        let mut n = 0;
        for i in 0..here.len() {
            for j in 0..here.len() {
                if i < j
                    && here[i].event != here[j].event
                    && here[i].during.start().max(here[j].during.start())
                        < here[i].during.end().min(here[j].during.end())
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
            let loc = LocationId::new();
            let holds: Vec<Held> = windows
                .iter()
                .map(|(s, len)| hold(loc, EventId::new(), *s, s + len))
                .collect();
            prop_assert_eq!(detect(loc, &holds).len(), oracle(loc, &holds));
        }
    }

    /// An overflow-declared hold still physically occupies the room: it
    /// conflicts with a different event's hold on the same room.
    #[test]
    fn overflow_hold_still_occupies() {
        let (a, b) = (LocationId::new(), LocationId::new());
        let (e1, e2) = (EventId::new(), EventId::new());
        let mut overflow = hold(b, e1, 0, 10);
        overflow.overflow_for = Some(a);
        let own = hold(b, e2, 5, 15);
        assert_eq!(detect(b, &[overflow, own]).len(), 1);
    }

    /// The legitimate overflow pattern — one event, two locations — never
    /// pairs on a single location.
    #[test]
    fn same_event_two_locations_is_clean() {
        let (a, b) = (LocationId::new(), LocationId::new());
        let e = EventId::new();
        let primary = hold(a, e, 0, 10);
        let mut overflow = hold(b, e, 0, 10);
        overflow.overflow_for = Some(a);
        let holds = [primary, overflow];
        assert!(detect(a, &holds).is_empty());
        assert!(detect(b, &holds).is_empty());
    }
}
