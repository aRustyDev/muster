//! Muster — the application layer (Phase 6). The service wraps the engine
//! and SDK; it adds DTO mapping and orchestration, never feasibility logic
//! (Rule 03): conflicts arrive as engine violation records, provenance
//! arrives from derived expansion, and this crate recomputes neither.
//!
//! Delivery-mechanism-neutral by design — QUESTION-0015 (frontend
//! structure) stays open until Prototype; the PoC is this library plus a
//! CLI demo.

pub mod demo;
pub mod service;

pub use demo::{run_demo, DemoReport};
pub use service::{EntrySource, MusterService, ScheduleEntry, ScheduleView, SelectionOutcome};
