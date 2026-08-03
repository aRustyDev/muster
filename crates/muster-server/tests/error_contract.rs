//! The error→status contract, pinned at the mapping seam for every
//! `OrreryError` variant (added 2026-08-03, QF slice / QR-2 SRV-1,
//! finding F-18d: only NotFound was covered, 1 of 5). e2e_http keeps
//! exercising NotFound over the wire; the rest are pinned here directly
//! at the `From<OrreryError> for ApiError` seam, because several variants
//! (ConstraintViolated in particular) cannot be triggered through the
//! router without contorting the fixture.

use axum::http::StatusCode;
use axum::response::IntoResponse;

use muster_server::api::ApiError;
use orrery::error::OrreryError;
use orrery::model::{EntityRef, EventId};

fn status_for(e: OrreryError) -> StatusCode {
    ApiError::from(e).into_response().status()
}

#[test]
fn error_contract_every_variant_maps_to_its_pinned_status() {
    assert_eq!(
        status_for(OrreryError::NotFound(EntityRef::Event(EventId::new()))),
        StatusCode::NOT_FOUND
    );
    assert_eq!(
        status_for(OrreryError::CommandRejected {
            reason: "test".into()
        }),
        StatusCode::CONFLICT
    );
    assert_eq!(
        status_for(OrreryError::InvalidInterval { reason: "test" }),
        StatusCode::BAD_REQUEST
    );
    assert_eq!(
        status_for(OrreryError::ConstraintViolated { constraint: "test" }),
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        status_for(OrreryError::PreviewUnsupported { kind: "test" }),
        StatusCode::BAD_REQUEST
    );
}
