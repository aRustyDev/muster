//! `suggest_room_schedule` — PRD muster-sdk Flow A: greedy seed → engine
//! overlay evaluation → objective breakdown. (Local-search refinement is
//! the slice-2 insertion point, between those two steps.)

use orrery::engine::{Assignment, Engine};
use orrery::interval::{Interval, Timestamp};
use orrery::model::{Held, Posture};
use orrery::repo::Repository;
use orrery::FeasibilityOracle;

use crate::objective::EvalCtx;
use crate::{assign, Objective, Result, RoomOption, RoomRequest, Suggestion};

pub fn suggest_room_schedule<R: Repository>(
    engine: &Engine<R>,
    requests: &[RoomRequest],
    rooms: &[RoomOption],
    at: Timestamp,
    window: Interval,
) -> Result<Suggestion> {
    let _span = tracing::info_span!(
        "sdk.suggest",
        requests = requests.len(),
        rooms = rooms.len()
    )
    .entered();

    let (placements, unassigned) = assign::greedy(requests, rooms);

    // Evaluate the proposal through the engine — the SDK never judges
    // feasibility itself (Rule 03).
    let held: Vec<Held> = placements
        .iter()
        .map(|p| {
            let req = requests
                .iter()
                .find(|q| q.event == p.event)
                .expect("placement always originates from a request");
            Held {
                location: p.location,
                event: p.event,
                during: req.window,
                posture: Posture::OnSite,
                overflow_for: None,
                capacity_override: None,
            }
        })
        .collect();
    let overlay = Assignment {
        attends: vec![],
        held,
        at,
        window,
    };
    let violations = engine.is_feasible(&overlay);

    let breakdown = Objective::standard().evaluate(&EvalCtx {
        placements: &placements,
        requests,
        rooms,
        violations: &violations,
    });

    Ok(Suggestion {
        placements,
        unassigned,
        violations,
        breakdown,
    })
}
