//! Layered configuration (Rule 05 / muster/SPEC-02 conventions): defaults,
//! then environment — env always last (Rule 09; secrets never come from a
//! checked-in file). The exporter knob is the slice's "first real
//! deployment knob"; `MUSTER_*` wins, `ORRERY_OTEL_EXPORTER` is honoured
//! as the SPEC-02 dev spelling.

use figment::providers::{Env, Serialized};
use figment::Figment;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Bind address for the API listener.
    pub bind: String,
    /// Trace exporter: `stdout` (fmt layer) or `none`. The OTLP exporter
    /// is deliberately deferred to Alpha (phase doc records it) — the
    /// knob exists so deployments configure it, not code.
    pub exporter: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            bind: "127.0.0.1:8080".into(),
            exporter: "stdout".into(),
        }
    }
}

impl ServerConfig {
    /// Boxed error: `figment::Error` is ~200 bytes and this is a
    /// once-at-startup call (clippy::result_large_err).
    pub fn load() -> Result<Self, Box<figment::Error>> {
        let mut cfg: ServerConfig = Figment::from(Serialized::defaults(ServerConfig::default()))
            .merge(Env::prefixed("MUSTER_"))
            .extract()?;
        // SPEC-02 dev spelling, honoured when the MUSTER_ knob is absent.
        if std::env::var_os("MUSTER_EXPORTER").is_none() {
            if let Ok(v) = std::env::var("ORRERY_OTEL_EXPORTER") {
                cfg.exporter = v;
            }
        }
        Ok(cfg)
    }
}
