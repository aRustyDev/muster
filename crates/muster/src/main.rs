//! Muster CLI (Phase 6 PoC). `anyhow` at the binary edge is the Rule 04
//! pattern; the library beneath returns typed errors.

fn main() -> anyhow::Result<()> {
    match std::env::args().nth(1).as_deref() {
        Some("demo") => {
            let report = muster::run_demo()?;
            println!("muster demo — the PoC story\n");
            for line in &report.lines {
                println!("{line}");
            }
            println!(
                "\n{} conflict(s) detected · {} derived entr{} with provenance",
                report.conflicts_after_selection,
                report.derived_entries,
                if report.derived_entries == 1 {
                    "y"
                } else {
                    "ies"
                }
            );
            Ok(())
        }
        _ => {
            println!("muster {} — usage: muster demo", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
    }
}
