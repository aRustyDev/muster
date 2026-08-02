//! `containment_exclusivity`: a container and something it (transitively)
//! contains, held for different events in overlapping windows — "Building
//! Foo" and "Building Foo Room A" cannot be booked simultaneously for
//! different events (ADR-0009). Partition: location (the container).

use std::collections::HashMap;

use crate::detect::{ViolationDraft, SEVERITY_CONTAINMENT_EXCLUSIVITY};
use crate::model::{EntityRef, Held, LocationId, ViolationKind, Within};

/// Pure over the holds and the containment edge set. Ancestor closure is
/// computed internally from `within` (small: locations number in the
/// thousands).
pub fn detect(holds: &[Held], within: &[Within]) -> Vec<ViolationDraft> {
    let parent: HashMap<LocationId, LocationId> =
        within.iter().map(|w| (w.child, w.parent)).collect();

    let ancestors = |mut loc: LocationId| -> Vec<LocationId> {
        let mut out = Vec::new();
        while let Some(p) = parent.get(&loc) {
            if out.contains(p) {
                break; // defensive: cycles are illegal but must not hang
            }
            out.push(*p);
            loc = *p;
        }
        out
    };

    let mut out = Vec::new();
    for content in holds {
        for anc in ancestors(content.location) {
            for container in holds.iter().filter(|h| h.location == anc) {
                if container.event != content.event && container.during.overlaps(&content.during) {
                    out.push(ViolationDraft {
                        kind: ViolationKind::ContainmentExclusivity,
                        severity: SEVERITY_CONTAINMENT_EXCLUSIVITY,
                        subjects: vec![
                            EntityRef::Location(container.location),
                            EntityRef::Location(content.location),
                            EntityRef::Event(container.event),
                            EntityRef::Event(content.event),
                        ],
                    });
                }
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

    /// Oracle: independently recompute transitive containment by repeated
    /// single-step lookup over the raw edge list (no map), then count
    /// overlapping different-event pairs in either containment direction —
    /// but only the (container=ancestor, content=descendant) orientation,
    /// matching the detector's contract.
    fn oracle(holds: &[Held], within: &[Within]) -> usize {
        fn is_ancestor(w: &[Within], anc: LocationId, mut x: LocationId) -> bool {
            for _ in 0..w.len() + 1 {
                match w.iter().find(|e| e.child == x) {
                    Some(e) if e.parent == anc => return true,
                    Some(e) => x = e.parent,
                    None => return false,
                }
            }
            false
        }
        let mut n = 0;
        for content in holds {
            for container in holds {
                if container.event != content.event
                    && is_ancestor(within, container.location, content.location)
                    && container.during.start().max(content.during.start())
                        < container.during.end().min(content.during.end())
                {
                    n += 1;
                }
            }
        }
        n
    }

    proptest! {
        /// Random three-level chains (room -> structure -> campus) with
        /// random holds across all three.
        #[test]
        fn prop_matches_oracle(
            windows in proptest::collection::vec((0i64..300, 1i64..80, 0usize..3), 1..10)
        ) {
            let room = LocationId::new();
            let bldg = LocationId::new();
            let campus = LocationId::new();
            let within = vec![
                Within { child: room, parent: bldg },
                Within { child: bldg, parent: campus },
            ];
            let locs = [room, bldg, campus];
            let holds: Vec<Held> = windows
                .iter()
                .map(|(s, len, li)| hold(locs[*li], EventId::new(), *s, s + len))
                .collect();
            prop_assert_eq!(detect(&holds, &within).len(), oracle(&holds, &within));
        }
    }

    #[test]
    fn skip_tier_containment_still_detected() {
        // room directly within campus (skipped tier is legal, ADR-0009)
        let (room, campus) = (LocationId::new(), LocationId::new());
        let within = vec![Within {
            child: room,
            parent: campus,
        }];
        let holds = vec![
            hold(campus, EventId::new(), 0, 10),
            hold(room, EventId::new(), 5, 15),
        ];
        assert_eq!(detect(&holds, &within).len(), 1);
    }
}
