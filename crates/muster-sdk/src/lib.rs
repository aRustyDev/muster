//! Muster-SDK — search and orchestration over Orrery's contract.
//!
//! Consumes `is_feasible` / `score`; never redefines them (Rule 03,
//! ADR-0013). Solver tiers land in Phase 5: greedy (provably optimal for
//! fixed start times — interval graph colouring), local search, maybe
//! CP-SAT. Currently a compiling stub so the workspace seam exists.

pub use orrery;
