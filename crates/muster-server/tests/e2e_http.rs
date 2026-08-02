//! The member flow again, this time over the wire (tower oneshot — no
//! sockets): the REST surface must tell the same story the service does.

use std::sync::{Arc, Mutex};

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use tower::ServiceExt;

use muster_server::api::AppState;
use muster_types::{EventDto, ScheduleDto, SelectRequest, SelectionOutcomeDto};

fn demo_state() -> (AppState, muster::DemoWorld) {
    let (svc, world) = muster::build_demo_world().expect("demo world");
    (
        AppState {
            svc: Arc::new(Mutex::new(svc)),
            default_window: world.day,
            default_at: world.now,
        },
        world,
    )
}

async fn json_body<T: serde::de::DeserializeOwned>(resp: axum::response::Response) -> T {
    let bytes = axum::body::to_bytes(resp.into_body(), 1 << 20)
        .await
        .expect("body");
    serde_json::from_slice(&bytes).expect("json")
}

fn post(uri: &str, body: &impl serde::Serialize) -> Request<Body> {
    Request::post(uri)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_vec(body).unwrap()))
        .unwrap()
}

#[tokio::test]
async fn e2e_http_member_flow_browse_select_conflict_deselect_resolve() {
    let (state, w) = demo_state();
    let app = muster_server::router(state);

    // Browse.
    let resp = app
        .clone()
        .oneshot(Request::get("/api/events").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let events: Vec<EventDto> = json_body(resp).await;
    assert_eq!(events.len(), 3);
    assert!(events.iter().any(|e| e.room.as_deref() == Some("Room 101")));

    // Select two overlapping talks — second response carries the conflict.
    for (event, priority, expect_conflict) in [
        (w.talk_rust.0, 0.9_f32, false),
        (w.talk_systems.0, 0.7, true),
    ] {
        let resp = app
            .clone()
            .oneshot(post(
                "/api/select",
                &SelectRequest {
                    person: w.member.0,
                    event,
                    priority: Some(priority),
                },
            ))
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let out: SelectionOutcomeDto = json_body(resp).await;
        assert_eq!(!out.conflicts.is_empty(), expect_conflict, "{out:#?}");
        if expect_conflict {
            assert!(out.conflicts[0].events.contains(&w.talk_rust.0));
        }
    }

    // Schedule: three entries, two flagged, provenance on the derived one.
    let resp = app
        .clone()
        .oneshot(
            Request::get(format!("/api/schedule/{}", w.member.0))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let sched: ScheduleDto = json_body(resp).await;
    assert_eq!(sched.entries.len(), 3);
    assert_eq!(sched.entries.iter().filter(|e| e.flagged).count(), 2);
    assert!(sched.entries.iter().any(|e| matches!(
        &e.provenance,
        muster_types::ProvenanceDto::Group { name } if name == "cohort-26"
    )));

    // Deselect resolves.
    let resp = app
        .clone()
        .oneshot(post(
            "/api/deselect",
            &muster_types::DeselectRequest {
                person: w.member.0,
                event: w.talk_rust.0,
            },
        ))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let out: SelectionOutcomeDto = json_body(resp).await;
    assert!(out.conflicts.is_empty(), "{out:#?}");

    // Unknown event → typed 404, not a 500.
    let resp = app
        .oneshot(post(
            "/api/select",
            &SelectRequest {
                person: w.member.0,
                event: uuid::Uuid::now_v7(),
                priority: None,
            },
        ))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}
