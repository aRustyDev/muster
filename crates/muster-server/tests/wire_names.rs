//! Wire-name pinning (added 2026-08-03, QF slice / QR-2 SRV-2, finding
//! F-18e): violation kind/severity cross the wire as `Debug` names, so an
//! engine enum rename would silently change the HTTP contract. This pins
//! the names a real conflict produces today — as STRING LITERALS only, so
//! the muster-family boundary greps (`ViolationKind::` / `detect::` /
//! `overlaps(`) stay empty over this crate (phases/06-app.md:103).
//!
//! Scope note: this pins the one violation class the demo world can
//! trigger over the wire. The full per-class matrix is Muster-Alpha work
//! (QR-2 item M-3); the remaining kind names are pinned engine-side by
//! that matrix when each class becomes wire-reachable.

use std::sync::{Arc, Mutex};

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use tower::ServiceExt;

use muster_server::api::AppState;
use muster_types::{SelectRequest, SelectionOutcomeDto};

#[tokio::test]
async fn wire_names_of_kind_and_severity_are_pinned() {
    let (svc, w) = muster::build_demo_world().expect("demo world");
    let state = AppState {
        svc: Arc::new(Mutex::new(svc)),
        default_window: w.day,
        default_at: w.now,
    };
    let app = muster_server::router(state);

    // Select two overlapping talks; the second response carries the conflict.
    for event in [w.talk_rust.0, w.talk_systems.0] {
        let req = Request::post("/api/select")
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(
                serde_json::to_vec(&SelectRequest {
                    person: w.member.0,
                    event,
                    priority: Some(0.5),
                })
                .unwrap(),
            ))
            .unwrap();
        let resp = app.clone().oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(resp.into_body(), 1 << 20)
            .await
            .unwrap();
        let out: SelectionOutcomeDto = serde_json::from_slice(&bytes).unwrap();
        if event == w.talk_systems.0 {
            let c = out
                .conflicts
                .first()
                .expect("overlapping talks must conflict");
            // The pinned contract. If an engine rename breaks these, that
            // is the test doing its job: the wire name is now load-bearing
            // and changing it is a (pre-1.0) breaking change to record.
            assert_eq!(c.kind, "TimeConflict");
            assert_eq!(c.severity, "Hard");
        }
    }
}
