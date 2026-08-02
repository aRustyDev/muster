//! Muster — the application (Phase 6). Stub binary so the workspace seam
//! exists; `anyhow` at the top level is the Rule 04 pattern for binaries.

fn main() -> anyhow::Result<()> {
    println!("muster {} (Phase 2 stub)", env!("CARGO_PKG_VERSION"));
    Ok(())
}
