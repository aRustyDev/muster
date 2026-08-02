//! The REST surface (muster/SPEC-02, member flow slice). Handlers map
//! wire DTOs ↔ service types and nothing else: no feasibility logic, no
//! interval math, no violation construction — conflicts and flags arrive
//! as engine records via the service (Rule 03).
//!
//! Violation kind/severity reach the wire via their `Debug` names on
//! purpose: this crate never names engine violation variants, so the
//! boundary grep stays meaningful over all muster-family sources.

use std::sync::{Arc, Mutex};

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Deserialize;
use uuid::Uuid;

use muster::{EntrySource, MusterService, ScheduleView, SelectionOutcome};
use muster_types::{
    ConflictDto, DeselectRequest, EventDto, PriorityRequest, ProvenanceDto, ScheduleDto,
    ScheduleEntryDto, SelectRequest, SelectionOutcomeDto, SeqDto,
};
use orrery::error::OrreryError;
use orrery::interval::{Interval, Timestamp};
use orrery::model::{EntityRef, EventId, PersonId, Violation};
use orrery::repo::memory::MemoryRepo;

/// Server state: the service behind a mutex (MemoryRepo is single-writer
/// by design — Rule 00b; the mutex serialises the HTTP side to match),
/// plus the default evaluation window/instant for requests that don't
/// supply their own. The engine reads no clock; the binary edge owns time.
#[derive(Clone)]
pub struct AppState {
    pub svc: Arc<Mutex<MusterService<MemoryRepo>>>,
    pub default_window: Interval,
    pub default_at: Timestamp,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/api/events", get(list_events))
        .route("/api/select", post(select))
        .route("/api/deselect", post(deselect))
        .route("/api/priority", post(set_priority))
        .route("/api/schedule/{person}", get(schedule))
        .with_state(state)
}

#[derive(Debug, Default, Deserialize)]
pub struct WindowQuery {
    pub start_us: Option<i64>,
    pub end_us: Option<i64>,
    pub at_us: Option<i64>,
}

impl WindowQuery {
    fn resolve(&self, s: &AppState) -> Result<(Interval, Timestamp), ApiError> {
        let window = match (self.start_us, self.end_us) {
            (Some(a), Some(b)) => Interval::new(Timestamp(a), Timestamp(b))
                .map_err(|e| ApiError(StatusCode::BAD_REQUEST, e.to_string()))?,
            _ => s.default_window,
        };
        let at = self.at_us.map(Timestamp).unwrap_or(s.default_at);
        Ok((window, at))
    }
}

/// One error shape at the edge; the typed engine error decides the status.
pub struct ApiError(StatusCode, String);

impl axum::response::IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        // The body is the error's Display text. OrreryError carries no
        // coordinates by construction (Rule 09 — enforced in orrery).
        (self.0, self.1).into_response()
    }
}

impl From<OrreryError> for ApiError {
    fn from(e: OrreryError) -> Self {
        let status = match &e {
            OrreryError::NotFound(_) => StatusCode::NOT_FOUND,
            OrreryError::CommandRejected { .. } => StatusCode::CONFLICT,
            OrreryError::InvalidInterval { .. } => StatusCode::BAD_REQUEST,
            OrreryError::ConstraintViolated { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        };
        ApiError(status, e.to_string())
    }
}

fn conflict_dto(v: &Violation) -> ConflictDto {
    ConflictDto {
        kind: format!("{:?}", v.kind),
        severity: format!("{:?}", v.severity),
        events: v
            .subjects
            .iter()
            .filter_map(|s| match s {
                EntityRef::Event(e) => Some(e.0),
                _ => None,
            })
            .collect(),
    }
}

fn outcome_dto(o: SelectionOutcome) -> SelectionOutcomeDto {
    SelectionOutcomeDto {
        seq: o.seq,
        conflicts: o.conflicts.iter().map(conflict_dto).collect(),
    }
}

fn schedule_dto(view: ScheduleView) -> ScheduleDto {
    ScheduleDto {
        entries: view
            .entries
            .into_iter()
            .map(|e| ScheduleEntryDto {
                event: e.event.0,
                name: e.event_name,
                start_us: e.window.start().0,
                end_us: e.window.end().0,
                provenance: match e.source {
                    EntrySource::SelfSelected => ProvenanceDto::SelfSelected,
                    EntrySource::Coordinator => ProvenanceDto::Coordinator,
                    EntrySource::DerivedFromGroup { group_name, .. } => {
                        ProvenanceDto::Group { name: group_name }
                    }
                },
                flagged: e.flagged,
            })
            .collect(),
    }
}

async fn list_events(
    State(s): State<AppState>,
    Query(q): Query<WindowQuery>,
) -> Result<Json<Vec<EventDto>>, ApiError> {
    let (window, _) = q.resolve(&s)?;
    let svc = s.svc.lock().expect("service mutex");
    let events = svc.events(window)?;
    Ok(Json(
        events
            .into_iter()
            .map(|e| EventDto {
                id: e.event.0,
                name: e.name,
                start_us: e.window.start().0,
                end_us: e.window.end().0,
                room: e.room,
            })
            .collect(),
    ))
}

async fn select(
    State(s): State<AppState>,
    Query(q): Query<WindowQuery>,
    Json(req): Json<SelectRequest>,
) -> Result<Json<SelectionOutcomeDto>, ApiError> {
    let (window, at) = q.resolve(&s)?;
    let mut svc = s.svc.lock().expect("service mutex");
    let outcome = svc.select(
        PersonId(req.person),
        EventId(req.event),
        req.priority,
        at,
        window,
    )?;
    Ok(Json(outcome_dto(outcome)))
}

async fn deselect(
    State(s): State<AppState>,
    Query(q): Query<WindowQuery>,
    Json(req): Json<DeselectRequest>,
) -> Result<Json<SelectionOutcomeDto>, ApiError> {
    let (window, at) = q.resolve(&s)?;
    let mut svc = s.svc.lock().expect("service mutex");
    let outcome = svc.deselect(PersonId(req.person), EventId(req.event), at, window)?;
    Ok(Json(outcome_dto(outcome)))
}

async fn set_priority(
    State(s): State<AppState>,
    Json(req): Json<PriorityRequest>,
) -> Result<Json<SeqDto>, ApiError> {
    let mut svc = s.svc.lock().expect("service mutex");
    let seq = svc.set_priority(PersonId(req.person), EventId(req.event), req.value)?;
    Ok(Json(SeqDto { seq }))
}

async fn schedule(
    State(s): State<AppState>,
    Path(person): Path<Uuid>,
    Query(q): Query<WindowQuery>,
) -> Result<Json<ScheduleDto>, ApiError> {
    let (window, at) = q.resolve(&s)?;
    let svc = s.svc.lock().expect("service mutex");
    let view = svc.my_schedule(PersonId(person), window, at)?;
    Ok(Json(schedule_dto(view)))
}
