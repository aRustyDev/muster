//! Greedy room assignment (Tier 0, muster-sdk/SPEC-00).
//!
//! Fixed start times make this interval graph colouring: interval graphs
//! are perfect, so greedy by **left endpoint** achieves the chromatic
//! number — if any zero-conflict assignment exists, this one finds one
//! (QUESTION-0005, ADR-0013). Room *choice* among the free rooms is a
//! quality knob, not a correctness one: any free room preserves the
//! invariant, so we take the best capacity fit (smallest adequate room,
//! ties to the smaller location id — deterministic, no RNG).

use orrery::interval::Timestamp;
use orrery::model::EventId;

use crate::{Placement, RoomOption, RoomRequest};

/// Assign rooms to fixed-start events. Returns placements plus the events
/// that could not be placed (never silently dropped).
pub fn greedy(requests: &[RoomRequest], rooms: &[RoomOption]) -> (Vec<Placement>, Vec<EventId>) {
    let mut order: Vec<&RoomRequest> = requests.iter().collect();
    order.sort_by_key(|r| (r.window.start(), r.window.end(), r.event));

    // Each room's occupied-until frontier. Half-open windows: a room is
    // free for a request iff its frontier <= request start.
    let mut frontier: Vec<(&RoomOption, Timestamp)> =
        rooms.iter().map(|r| (r, Timestamp(i64::MIN))).collect();

    let mut placements = Vec::new();
    let mut unassigned = Vec::new();
    for req in order {
        let mut free: Vec<usize> = frontier
            .iter()
            .enumerate()
            .filter(|(_, (_, until))| *until <= req.window.start())
            .map(|(i, _)| i)
            .collect();
        // Best fit: smallest adequate capacity first, capacity-less rooms
        // last, ties to smaller location id.
        free.sort_by_key(|&i| {
            let room = frontier[i].0;
            let adequacy = match (room.capacity, req.expected_size) {
                (Some(cap), Some(need)) if cap >= need => (0u8, cap),
                (Some(cap), None) => (1, cap),
                (Some(cap), Some(_)) => (2, cap), // too small — last resort
                (None, _) => (1, u32::MAX),
            };
            (adequacy, room.location)
        });
        match free.first() {
            Some(&i) => {
                placements.push(Placement {
                    event: req.event,
                    location: frontier[i].0.location,
                });
                frontier[i].1 = req.window.end();
            }
            None => unassigned.push(req.event),
        }
    }
    (placements, unassigned)
}
