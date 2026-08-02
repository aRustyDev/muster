//! `suggest_room_schedule` — PRD muster-sdk Flow A: greedy seed → engine
//! overlay evaluation → objective breakdown. (Local-search refinement is
//! the slice-2 insertion point, between those two steps.)

use orrery::engine::{Assignment, Engine};
use orrery::interval::{Interval, Timestamp};
use orrery::model::{Held, Posture};
use orrery::repo::Repository;
use orrery::FeasibilityOracle;

use crate::objective::{EvalCtx, StabilityFromReference};
use crate::search::{self, SearchBudget};
use crate::{assign, Objective, Placement, Result, RoomOption, RoomRequest, Suggestion};

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

/// Refinement controls for [`suggest_and_refine`].
pub struct RefineOptions<'a> {
    /// Anchor churn against a prior assignment (PRD Flow B: "here is last
    /// semester, three rooms are gone, re-solve").
    pub reference: Option<&'a [Placement]>,
    pub budget: SearchBudget,
    pub rng_seed: u64,
}

/// PRD Flow B: greedy seed → local-search refinement, optionally anchored
/// to a reference assignment — a slightly worse schedule that changes 3
/// placements beats an optimal one that changes 200.
pub fn suggest_and_refine<R: Repository>(
    engine: &Engine<R>,
    requests: &[RoomRequest],
    rooms: &[RoomOption],
    at: Timestamp,
    window: Interval,
    opts: RefineOptions<'_>,
) -> Result<Suggestion> {
    let RefineOptions {
        reference,
        budget,
        rng_seed,
    } = opts;
    let (seed_placements, unassigned) = assign::greedy(requests, rooms);

    let objective = match reference {
        Some(r) => Objective::standard().with(StabilityFromReference {
            reference: r.to_vec(),
            weight: 1.0,
        }),
        None => Objective::standard(),
    };

    let evaluate = |placements: &[Placement]| {
        let held: Vec<Held> = placements
            .iter()
            .filter_map(|p| {
                let req = requests.iter().find(|q| q.event == p.event)?;
                Some(Held {
                    location: p.location,
                    event: p.event,
                    during: req.window,
                    posture: Posture::OnSite,
                    overflow_for: None,
                    capacity_override: None,
                })
            })
            .collect();
        let violations = engine.is_feasible(&Assignment {
            attends: vec![],
            held,
            at,
            window,
        });
        objective.evaluate(&EvalCtx {
            placements,
            requests,
            rooms,
            violations: &violations,
        })
    };

    let outcome = search::improve(&seed_placements, rooms, &evaluate, budget, rng_seed);

    // Re-derive the violations for the FINAL placements so the suggestion
    // reports what it actually proposes.
    let held: Vec<Held> = outcome
        .placements
        .iter()
        .filter_map(|p| {
            let req = requests.iter().find(|q| q.event == p.event)?;
            Some(Held {
                location: p.location,
                event: p.event,
                during: req.window,
                posture: Posture::OnSite,
                overflow_for: None,
                capacity_override: None,
            })
        })
        .collect();
    let violations = engine.is_feasible(&Assignment {
        attends: vec![],
        held,
        at,
        window,
    });

    Ok(Suggestion {
        placements: outcome.placements,
        unassigned,
        violations,
        breakdown: outcome.breakdown,
    })
}
