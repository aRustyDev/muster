//! Subscriber installation (Rule 05: binaries configure, libraries emit).
//! `stdout` = tracing-subscriber fmt layer; `none` = nothing installed.
//! OTLP lands at Alpha when a collector exists to receive it — the knob
//! is here so that arrival is configuration, not surgery.

use tracing_subscriber::EnvFilter;

/// Install the global subscriber per the exporter knob. Safe to call once
/// per process; unknown values fall back to `stdout` with a warning line
/// (stderr — the subscriber may not exist yet).
pub fn install(exporter: &str) {
    match exporter {
        "none" => {}
        other => {
            if other != "stdout" {
                eprintln!("muster-server: unknown exporter '{other}', using stdout");
            }
            tracing_subscriber::fmt()
                .with_env_filter(
                    EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
                )
                .init();
        }
    }
}
