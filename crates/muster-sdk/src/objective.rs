//! Objective composition (muster-sdk/SPEC-01): weighted soft-constraint
//! terms with an additive, explainable breakdown. The breakdown is an
//! explanation of the objective, not a second objective — `total` is
//! exactly `Σ weight × cost` over its rows.

use orrery::model::Violation;

use crate::{Placement, RoomOption, RoomRequest};

/// Everything a term may look at. Violations come from the engine,
/// verbatim; the SDK adds no feasibility semantics.
pub struct EvalCtx<'a> {
    pub placements: &'a [Placement],
    pub requests: &'a [RoomRequest],
    pub rooms: &'a [RoomOption],
    pub violations: &'a [Violation],
}

pub trait Term {
    fn name(&self) -> &'static str;
    fn weight(&self) -> f64;
    /// Raw (unweighted) cost; 0.0 is "this term is satisfied".
    fn cost(&self, ctx: &EvalCtx<'_>) -> f64;
}

#[derive(Default)]
pub struct Objective {
    terms: Vec<Box<dyn Term>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TermRow {
    pub name: &'static str,
    pub weight: f64,
    pub cost: f64,
    pub weighted: f64,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Breakdown {
    pub rows: Vec<TermRow>,
    pub total: f64,
}

impl Objective {
    pub fn with(mut self, term: impl Term + 'static) -> Self {
        self.terms.push(Box::new(term));
        self
    }

    /// The default composition `suggest` uses: violation cost dominant,
    /// utilisation as a tie-breaker.
    pub fn standard() -> Self {
        Objective::default()
            .with(ViolationCost { weight: 1.0 })
            .with(RoomUtilisation { weight: 0.1 })
    }

    pub fn evaluate(&self, ctx: &EvalCtx<'_>) -> Breakdown {
        let rows: Vec<TermRow> = self
            .terms
            .iter()
            .map(|t| {
                let cost = t.cost(ctx);
                TermRow {
                    name: t.name(),
                    weight: t.weight(),
                    cost,
                    weighted: t.weight() * cost,
                }
            })
            .collect();
        let total = rows.iter().map(|r| r.weighted).sum();
        Breakdown { rows, total }
    }
}

/// Severity-weighted violation cost — literally the engine's weights
/// (`orrery::engine::severity_weight`), so `ViolationCost` at weight 1.0
/// equals `−score` on the same overlay. Single-sourced 2026-08-03 (QF
/// slice, QR-2 SDK-7): the local Hard/Warning/Info table this carried
/// could diverge from the engine's; now there is one definition and two
/// call sites, as the doc always claimed.
pub struct ViolationCost {
    pub weight: f64,
}

impl Term for ViolationCost {
    fn name(&self) -> &'static str {
        "violation_cost"
    }
    fn weight(&self) -> f64 {
        self.weight
    }
    fn cost(&self, ctx: &EvalCtx<'_>) -> f64 {
        ctx.violations
            .iter()
            .map(|v| orrery::engine::severity_weight(v.severity))
            .sum()
    }
}

/// Penalises waste (a 300-seat hall for a 12-person workshop) and, more
/// steeply, overfill. Ratios per placement, summed; placements without
/// both capacity and expected size contribute 0.
pub struct RoomUtilisation {
    pub weight: f64,
}

impl Term for RoomUtilisation {
    fn name(&self) -> &'static str {
        "room_utilisation"
    }
    fn weight(&self) -> f64 {
        self.weight
    }
    fn cost(&self, ctx: &EvalCtx<'_>) -> f64 {
        ctx.placements
            .iter()
            .filter_map(|p| {
                let cap = ctx
                    .rooms
                    .iter()
                    .find(|r| r.location == p.location)?
                    .capacity? as f64;
                let need = ctx
                    .requests
                    .iter()
                    .find(|q| q.event == p.event)?
                    .expected_size? as f64;
                Some(if need <= cap {
                    (cap - need) / cap
                } else {
                    10.0 * (need - cap) / cap
                })
            })
            .sum()
    }
}

/// Distance-from-reference stability (muster-sdk/SPEC-01): "a schedule 5%
/// worse that changes 3 assignments beats an optimal one that changes
/// 200." Cost = number of events placed on a different room than the
/// reference (events absent from the reference cost nothing).
pub struct StabilityFromReference {
    pub reference: Vec<crate::Placement>,
    pub weight: f64,
}

impl Term for StabilityFromReference {
    fn name(&self) -> &'static str {
        "stability_from_reference"
    }
    fn weight(&self) -> f64 {
        self.weight
    }
    fn cost(&self, ctx: &EvalCtx<'_>) -> f64 {
        ctx.placements
            .iter()
            .filter(|p| {
                self.reference
                    .iter()
                    .any(|r| r.event == p.event && r.location != p.location)
            })
            .count() as f64
    }
}

/// Expected-attendee-travel (muster-sdk/SPEC-01 FR-6): prices the walking
/// implied by placing time-adjacent events with shared interested
/// attendees in distant rooms — the direct objective that replaces the
/// type-clustering proxy. Flows and travel costs are precomputed by
/// [`attendee_flow`] so evaluation stays pure; unknown routes cost 0
/// (unmeasured must not punish).
pub struct ExpectedAttendeeTravel {
    /// (earlier event, later event, shared interest weight).
    pub flows: Vec<(orrery::model::EventId, orrery::model::EventId, f64)>,
    /// Best-mode cost in seconds per room pair.
    pub travel_s:
        std::collections::HashMap<(orrery::model::LocationId, orrery::model::LocationId), i64>,
    pub weight: f64,
}

impl Term for ExpectedAttendeeTravel {
    fn name(&self) -> &'static str {
        "expected_attendee_travel"
    }
    fn weight(&self) -> f64 {
        self.weight
    }
    fn cost(&self, ctx: &EvalCtx<'_>) -> f64 {
        let room_of = |e| {
            ctx.placements
                .iter()
                .find(|p| p.event == e)
                .map(|p| p.location)
        };
        self.flows
            .iter()
            .filter_map(|(a, b, interest)| {
                let (ra, rb) = (room_of(*a)?, room_of(*b)?);
                if ra == rb {
                    return Some(0.0);
                }
                let secs = *self.travel_s.get(&(ra, rb))?;
                Some(interest * (secs as f64) / 60.0) // interest-weighted minutes
            })
            .sum()
    }
}

/// Precompute attendee flows and the room-pair travel matrix for
/// [`ExpectedAttendeeTravel`]. Shared interest between two time-adjacent
/// requests = Σ over persons attending both of min(effective priority),
/// clamped at 0 — priority-weighted, per SPEC-01 ("not type clustering").
pub fn attendee_flow<R: orrery::repo::Repository>(
    repo: &R,
    requests: &[RoomRequest],
    rooms: &[RoomOption],
) -> orrery::Result<ExpectedAttendeeTravel> {
    let mut flows = Vec::new();
    let mut ordered: Vec<&RoomRequest> = requests.iter().collect();
    ordered.sort_by_key(|r| (r.window.start(), r.window.end(), r.event));
    for (i, a) in ordered.iter().enumerate() {
        for b in ordered.iter().skip(i + 1) {
            if b.window.start() < a.window.end() {
                continue; // overlapping: nobody travels between them
            }
            let aa = repo.attends_for_event(a.event)?;
            let bb = repo.attends_for_event(b.event)?;
            let interest: f64 = aa
                .iter()
                .filter_map(|x| {
                    let y = bb.iter().find(|y| y.person == x.person)?;
                    let w = x.effective_priority().min(y.effective_priority());
                    (w > 0.0).then_some(w as f64)
                })
                .sum();
            if interest > 0.0 {
                flows.push((a.event, b.event, interest));
            }
        }
    }
    let mut travel_s = std::collections::HashMap::new();
    for x in rooms {
        for y in rooms {
            if x.location != y.location {
                if let Some(c) = repo.travel_best(x.location, y.location)? {
                    travel_s.insert((x.location, y.location), c.duration_s);
                }
            }
        }
    }
    Ok(ExpectedAttendeeTravel {
        flows,
        travel_s,
        weight: 0.05,
    })
}
