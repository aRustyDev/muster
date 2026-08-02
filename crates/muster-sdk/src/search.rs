//! Local search (Tier 1, muster-sdk/SPEC-00): relocate/swap moves over a
//! seed assignment, first-improvement hill-climbing with a caller-supplied
//! RNG seed and evaluation budget.
//!
//! Two contracts matter more than the strategy:
//!
//! * **Monotone / anytime**: the result is the best assignment *seen*,
//!   which includes the seed — search can never return something worse,
//!   and interrupting it early (small budget) still returns the best so
//!   far (SPEC-01).
//! * **Infeasible states are traversable**: candidate moves may create
//!   violations; the objective prices them (via `ViolationCost`) rather
//!   than a rule forbidding them. This is why ADR-0012 chose detection
//!   over prevention — penalty-based search depends on it.
//!
//! `Shift(event, Δt)` from SPEC-01 is deferred until free start times
//! exist (pre-committed scope decision, phases/05-sdk.md slice 2).

use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;

use crate::objective::Breakdown;
use crate::{Placement, RoomOption};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Move {
    /// Move one event to a different room.
    Relocate {
        placement_idx: usize,
        room_idx: usize,
    },
    /// Exchange the rooms of two placements.
    Swap { a: usize, b: usize },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SearchBudget {
    /// Maximum number of candidate evaluations (objective calls).
    pub max_evals: u32,
}

impl Default for SearchBudget {
    fn default() -> Self {
        SearchBudget { max_evals: 200 }
    }
}

#[derive(Debug)]
pub struct SearchOutcome {
    pub placements: Vec<Placement>,
    pub breakdown: Breakdown,
    pub evals_used: u32,
    /// True when the result strictly beats the seed.
    pub improved: bool,
}

/// First-improvement hill-climb. `evaluate` prices a candidate (typically:
/// engine overlay violations + objective terms — supplied by the caller so
/// this module stays pure and strategy-only).
pub fn improve(
    seed_placements: &[Placement],
    rooms: &[RoomOption],
    evaluate: &dyn Fn(&[Placement]) -> Breakdown,
    budget: SearchBudget,
    rng_seed: u64,
) -> SearchOutcome {
    let _span = tracing::info_span!(
        "sdk.search",
        seed_n = seed_placements.len(),
        max_evals = budget.max_evals
    )
    .entered();

    let mut rng = StdRng::seed_from_u64(rng_seed);
    let mut best: Vec<Placement> = seed_placements.to_vec();
    let mut best_score = evaluate(&best);
    let seed_total = best_score.total;
    let mut evals: u32 = 1;

    loop {
        // Candidate moves from the current best, in seeded random order.
        let mut moves: Vec<Move> = Vec::new();
        for pi in 0..best.len() {
            for (ri, room) in rooms.iter().enumerate() {
                if room.location != best[pi].location {
                    moves.push(Move::Relocate {
                        placement_idx: pi,
                        room_idx: ri,
                    });
                }
            }
            for pj in (pi + 1)..best.len() {
                if best[pi].location != best[pj].location {
                    moves.push(Move::Swap { a: pi, b: pj });
                }
            }
        }
        moves.shuffle(&mut rng);

        let mut stepped = false;
        for mv in moves {
            if evals >= budget.max_evals {
                break;
            }
            let mut cand = best.clone();
            match mv {
                Move::Relocate {
                    placement_idx,
                    room_idx,
                } => {
                    cand[placement_idx].location = rooms[room_idx].location;
                }
                Move::Swap { a, b } => {
                    let tmp = cand[a].location;
                    cand[a].location = cand[b].location;
                    cand[b].location = tmp;
                }
            }
            let score = evaluate(&cand);
            evals += 1;
            if score.total < best_score.total {
                best = cand;
                best_score = score;
                stepped = true;
                break; // first improvement: restart move generation
            }
        }
        if !stepped || evals >= budget.max_evals {
            break;
        }
    }

    let improved = best_score.total < seed_total;
    SearchOutcome {
        placements: best,
        breakdown: best_score,
        evals_used: evals,
        improved,
    }
}
