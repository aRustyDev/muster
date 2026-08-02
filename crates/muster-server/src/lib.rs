//! Muster REST/JSON API (ADR-0025): axum over [`muster::MusterService`].
//!
//! This crate is the privacy boundary's single enforcement point
//! (Rule 00.6 / Rule 09): everything that leaves it is a `muster-types`
//! DTO, and those types structurally cannot carry coordinates or
//! anchor-shaped data. It is also the binary edge (Rule 04: `anyhow` in
//! `main` only) and the observability configurator (Rule 05: installs the
//! subscriber; the libraries only emit).

pub mod api;
pub mod config;
pub mod telemetry;

pub use api::{router, AppState};
pub use config::ServerConfig;
