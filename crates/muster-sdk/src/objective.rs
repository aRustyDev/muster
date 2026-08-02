//! Objective composition (muster-sdk/SPEC-01): weighted soft-constraint
//! terms with an additive, explainable breakdown. The breakdown is an
//! explanation of the objective, not a second objective — `total` is
//! exactly `Σ weight × cost` over its rows.

use orrery::model::{Severity, Violation};

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

/// Severity-weighted violation cost — the same weights as `engine.score`
/// (Hard 100 / Warning 10 / Info 1), so `ViolationCost` at weight 1.0
/// equals `−score` on the same overlay: one definition of severity cost,
/// two call sites.
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
            .map(|v| match v.severity {
                Severity::Hard => 100.0,
                Severity::Warning => 10.0,
                Severity::Info => 1.0,
            })
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
