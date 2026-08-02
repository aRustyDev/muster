//! Serve the demo world over REST (Prototype: MemoryRepo is the only
//! repository — ADR-0015 stays open; persistence arrives with Phase 7).

use std::sync::{Arc, Mutex};

use anyhow::Context;

use muster_server::{api, router, ServerConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg = ServerConfig::load().context("loading configuration")?;
    muster_server::telemetry::install(&cfg.exporter);

    let (svc, world) = muster::build_demo_world().context("seeding demo world")?;
    let state = api::AppState {
        svc: Arc::new(Mutex::new(svc)),
        default_window: world.day,
        default_at: world.now,
    };
    tracing::info!(bind = %cfg.bind, member = %world.member, "muster-server up (demo world)");
    println!(
        "muster-server listening on {} — demo member {}",
        cfg.bind, world.member
    );

    let listener = tokio::net::TcpListener::bind(&cfg.bind)
        .await
        .with_context(|| format!("binding {}", cfg.bind))?;
    axum::serve(listener, router(state))
        .await
        .context("serving")?;
    Ok(())
}
