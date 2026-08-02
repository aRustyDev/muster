//! `capacity_exceeded`: signalled interest above allocated capacity.
//! Partition: event.
//!
//! ADR-0018: the count is a **ranking signal, not a forecast** — we count
//! `attends` with effective priority above a threshold and compare against
//! `held.capacity_override ?? location.capacity`. The principled estimator
//! is a hook for consumers once actuals exist.

use crate::detect::{ViolationDraft, SEVERITY_CAPACITY_EXCEEDED};
use crate::model::{Attends, EntityRef, EventId, Held, LocationId, ViolationKind};

/// Count of signalled interest for one event: effective priority strictly
/// above `threshold` among that event's attends edges.
pub fn signalled_interest(event: EventId, attends: &[Attends], threshold: f32) -> u32 {
    attends
        .iter()
        .filter(|a| a.event == event && a.effective_priority() > threshold)
        .count() as u32
}

/// One draft per over-capacity `held` edge. `location_capacity` resolves
/// the location's base capacity (pure lookup supplied by the caller).
pub fn detect(
    event: EventId,
    holds: &[Held],
    signalled: u32,
    location_capacity: &dyn Fn(LocationId) -> Option<u32>,
) -> Vec<ViolationDraft> {
    holds
        .iter()
        .filter(|h| h.event == event)
        .filter_map(|h| {
            let cap = h
                .capacity_override
                .or_else(|| location_capacity(h.location))?;
            (signalled > cap).then(|| ViolationDraft {
                kind: ViolationKind::CapacityExceeded,
                severity: SEVERITY_CAPACITY_EXCEEDED,
                subjects: vec![EntityRef::Event(event), EntityRef::Location(h.location)],
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interval::{Interval, Timestamp};
    use crate::model::{AttendanceSource, PersonId, Posture};
    use proptest::prelude::*;

    fn att(event: EventId, prio: f32) -> Attends {
        Attends {
            person: PersonId::new(),
            event,
            during: Interval::new(Timestamp(0), Timestamp(10)).unwrap(),
            priority_group: prio,
            priority_person: None,
            priority_coord: None,
            coord_binding: false,
            source: AttendanceSource::SelfSelected,
            pinned: false,
        }
    }

    fn hold(event: EventId, location: LocationId, cap_override: Option<u32>) -> Held {
        Held {
            location,
            event,
            during: Interval::new(Timestamp(0), Timestamp(10)).unwrap(),
            posture: Posture::OnSite,
            overflow_for: None,
            capacity_override: cap_override,
        }
    }

    proptest! {
        /// Oracle inline: recount above-threshold interest naively and
        /// compare per hold against override-else-base capacity.
        #[test]
        fn prop_matches_naive(
            prios in proptest::collection::vec(0.0f32..1.0, 0..30),
            threshold in 0.0f32..1.0,
            base_cap in proptest::option::of(0u32..20),
            over_cap in proptest::option::of(0u32..20),
        ) {
            let e = EventId::new();
            let loc = LocationId::new();
            let attends: Vec<Attends> = prios.iter().map(|p| att(e, *p)).collect();
            let holds = [hold(e, loc, over_cap)];

            let naive_signalled =
                prios.iter().filter(|p| **p > threshold).count() as u32;
            let signalled = signalled_interest(e, &attends, threshold);
            prop_assert_eq!(signalled, naive_signalled);

            let drafts = detect(e, &holds, signalled, &|_| base_cap);
            let expected = match over_cap.or(base_cap) {
                Some(c) => usize::from(naive_signalled > c),
                None => 0,
            };
            prop_assert_eq!(drafts.len(), expected);
        }
    }

    /// ADR-0018: a room seating 60 with a workshop capped at 30 —
    /// the override binds, not the room capacity.
    #[test]
    fn override_beats_location_capacity() {
        let e = EventId::new();
        let loc = LocationId::new();
        let holds = [hold(e, loc, Some(30))];
        assert_eq!(detect(e, &holds, 45, &|_| Some(60)).len(), 1);
        assert!(detect(e, &holds, 25, &|_| Some(60)).is_empty());
    }

    #[test]
    fn no_capacity_anywhere_means_no_violation() {
        let e = EventId::new();
        let holds = [hold(e, LocationId::new(), None)];
        assert!(detect(e, &holds, 1_000, &|_| None).is_empty());
    }
}
