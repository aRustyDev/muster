//! `orphan_event`: an event with no `held` edge overlapping its window.
//! Partition: event.

use crate::detect::{ViolationDraft, SEVERITY_ORPHAN_EVENT};
use crate::model::{EntityRef, Event, Held, ViolationKind};

pub fn detect(event: &Event, holds: &[Held]) -> Option<ViolationDraft> {
    let housed = holds
        .iter()
        .any(|h| h.event == event.id && h.during.overlaps(&event.window));
    (!housed).then(|| ViolationDraft {
        kind: ViolationKind::OrphanEvent,
        severity: SEVERITY_ORPHAN_EVENT,
        subjects: vec![EntityRef::Event(event.id)],
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interval::{Interval, Timestamp};
    use crate::model::{EventId, LocationId, Posture};
    use proptest::prelude::*;

    fn event(s: i64, e: i64) -> Event {
        Event {
            id: EventId::new(),
            name: "e".into(),
            window: Interval::new(Timestamp(s), Timestamp(e)).unwrap(),
            kind: "talk".into(),
            timezone: None,
            ext: Default::default(),
        }
    }

    fn hold(event: EventId, s: i64, e: i64) -> Held {
        Held {
            location: LocationId::new(),
            event,
            during: Interval::new(Timestamp(s), Timestamp(e)).unwrap(),
            posture: Posture::OnSite,
            overflow_for: None,
            capacity_override: None,
        }
    }

    proptest! {
        #[test]
        fn prop_matches_naive(
            ev in (0i64..200, 1i64..100),
            hs in proptest::collection::vec((0i64..300, 1i64..100, prop::bool::ANY), 0..6)
        ) {
            let e = event(ev.0, ev.0 + ev.1);
            let holds: Vec<Held> = hs
                .iter()
                .map(|(s, len, mine)| {
                    let id = if *mine { e.id } else { EventId::new() };
                    hold(id, *s, s + len)
                })
                .collect();
            // naive: shared-instant check over this event's own holds
            let housed = holds.iter().any(|h| {
                h.event == e.id
                    && h.during.start().max(e.window.start())
                        < h.during.end().min(e.window.end())
            });
            prop_assert_eq!(detect(&e, &holds).is_none(), housed);
        }
    }

    /// ADR-0011: a hold that misses the event's window entirely does not
    /// house it — `held.during` is independent of the event's span.
    #[test]
    fn hold_outside_window_is_still_orphan() {
        let e = event(100, 200);
        let holds = [hold(e.id, 0, 50)];
        assert!(detect(&e, &holds).is_some());
    }
}
