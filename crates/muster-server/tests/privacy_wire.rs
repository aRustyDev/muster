//! privacy_ family (muster/SPEC-03): nothing anchor- or coordinate-shaped
//! leaves this server. Asserted mechanically over every member-flow
//! response: the recursive key set of each JSON payload must stay inside
//! the wire allowlist, and no forbidden substring may appear as a key.
//!
//! Honesty note (Rule 01.2): the slice pre-commitment said "on a world
//! WITH anchors" — that fixture is IMPOSSIBLE today: `orrery::model::
//! Anchors` has no producing command and no repository storage (the
//! plan-review re-homed that gap to Phase 6a). Until it lands, this test
//! pins the wire *shape*, which is the structural half of the guarantee;
//! extend the fixture with real anchors the moment a producer exists.

use std::collections::BTreeSet;
use std::sync::{Arc, Mutex};

use axum::body::Body;
use axum::http::{header, Request};
use tower::ServiceExt;

use muster_server::api::AppState;
use muster_types::SelectRequest;

const ALLOWED_KEYS: &[&str] = &[
    // EventDto
    "id",
    "name",
    "start_us",
    "end_us",
    "room",
    // SelectionOutcomeDto / ConflictDto
    "seq",
    "conflicts",
    "kind",
    "severity",
    "events",
    // ScheduleDto / ScheduleEntryDto / ProvenanceDto
    "entries",
    "event",
    "provenance",
    "flagged",
];

const FORBIDDEN_KEY_FRAGMENTS: &[&str] = &[
    "anchor",
    "lat",
    "lon",
    "coord",
    "address",
    "structure",
    "applies_when",
    "label",
];

fn collect_keys(v: &serde_json::Value, out: &mut BTreeSet<String>) {
    match v {
        serde_json::Value::Object(m) => {
            for (k, v) in m {
                out.insert(k.clone());
                collect_keys(v, out);
            }
        }
        serde_json::Value::Array(a) => a.iter().for_each(|v| collect_keys(v, out)),
        _ => {}
    }
}

#[tokio::test]
async fn privacy_member_wire_payloads_carry_no_anchor_or_coordinate_shape() {
    let (svc, w) = muster::build_demo_world().expect("demo world");
    let app = muster_server::router(AppState {
        svc: Arc::new(Mutex::new(svc)),
        default_window: w.day,
        default_at: w.now,
    });

    let mut responses: Vec<serde_json::Value> = Vec::new();
    for req in [
        Request::get("/api/events").body(Body::empty()).unwrap(),
        Request::post("/api/select")
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(
                serde_json::to_vec(&SelectRequest {
                    person: w.member.0,
                    event: w.talk_rust.0,
                    priority: Some(0.9),
                })
                .unwrap(),
            ))
            .unwrap(),
        Request::get(format!("/api/schedule/{}", w.member.0))
            .body(Body::empty())
            .unwrap(),
    ] {
        let resp = app.clone().oneshot(req).await.unwrap();
        assert!(resp.status().is_success());
        let bytes = axum::body::to_bytes(resp.into_body(), 1 << 20)
            .await
            .unwrap();
        responses.push(serde_json::from_slice(&bytes).unwrap());
    }

    let mut keys = BTreeSet::new();
    responses.iter().for_each(|v| collect_keys(v, &mut keys));

    for k in &keys {
        assert!(
            ALLOWED_KEYS.contains(&k.as_str()),
            "wire payload grew an un-reviewed key '{k}' — extend the \
             allowlist ONLY after checking it against Rule 00.6/Rule 09"
        );
        let lower = k.to_lowercase();
        for frag in FORBIDDEN_KEY_FRAGMENTS {
            assert!(
                !lower.contains(frag),
                "forbidden key fragment '{frag}' in wire key '{k}'"
            );
        }
    }
}
