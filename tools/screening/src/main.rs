//! Phase 1b screening harness (ADR-0021 Stage B) — THROWAWAY BY DESIGN.
//!
//! Usage: cargo run --release -- [S] [M] [L]      (default: S M)
//!
//! Order of operations (mirrors the pre-committed plan in
//! .claude/plans/orrery/phases/01b-screening.md):
//!   1. per-engine CRUD smoke on a fresh instance
//!   2. per-engine expired-edge micro-fixture (per-hop temporal filtering +
//!      depth-0 inclusion): Q1 must be exactly {0, 1}
//!   3. per scale: generate (ChaCha8 seed 42) -> load (timed) ->
//!      differential check (Q1/Q2/Q7b result SETS identical across engines,
//!      10 fixed pids) -> bench pid 7, 5 runs/cell, materialised + count-only.

mod data;
mod eng_agdb;
mod eng_cozo;
mod eng_grafeo;
mod engine;

use anyhow::Result;
use data::{generate, micro_fixture, Dataset, SCALES, T_MID, Q7B_HI, Q7B_LO};
use engine::{bench, Engine};
use std::collections::BTreeSet;
use std::time::Instant;

const PIDS: [i64; 10] = [1, 3, 7, 11, 19, 23, 42, 57, 73, 91];
const BENCH_PID: i64 = 7;
const RUNS: usize = 5;

type Factory = fn() -> Result<Box<dyn Engine>>;

fn factories() -> Vec<(&'static str, Factory)> {
    vec![
        ("grafeo", || Ok(Box::new(eng_grafeo::GrafeoEngine::new()?))),
        ("agdb", || Ok(Box::new(eng_agdb::AgdbEngine::new()?))),
        ("cozo", || Ok(Box::new(eng_cozo::CozoEngine::new()?))),
    ]
}

fn smoke(name: &str, factory: Factory) -> bool {
    // CRUD on a fresh instance
    let crud = factory().and_then(|mut e| e.crud_smoke());
    match &crud {
        Ok(()) => println!("  {name:8} CRUD smoke: PASS"),
        Err(e) => println!("  {name:8} CRUD smoke: FAIL — {e}"),
    }
    // expired-edge micro-fixture on another fresh instance
    let fixture = factory().and_then(|mut e| {
        e.load(&micro_fixture())?;
        let got: BTreeSet<i64> = e.q1(0, T_MID)?.into_iter().collect();
        let want: BTreeSet<i64> = [0i64, 1].into_iter().collect();
        anyhow::ensure!(
            got == want,
            "q1 fixture: got {got:?}, want {want:?} (depth-0 inclusion + expired-edge exclusion)"
        );
        let c = e.q1_count(0, T_MID)?;
        anyhow::ensure!(c == 2, "q1_count fixture: got {c}, want 2");
        println!("  {name:8} temporal micro-fixture: PASS ({})", e.notes());
        Ok(())
    });
    if let Err(e) = &fixture {
        println!("  {name:8} temporal micro-fixture: FAIL — {e}");
    }
    crud.is_ok() && fixture.is_ok()
}

struct Loaded {
    name: &'static str,
    eng: Box<dyn Engine>,
    load_ms: f64,
}

fn differential(loaded: &[Loaded], ds: &Dataset) -> bool {
    let pids: Vec<i64> = PIDS.iter().copied().filter(|p| *p < ds.persons).collect();
    let mut ok = true;
    let mut totals = [0usize; 3]; // aggregate q1/q2/q7b result rows (engine 0)
    for pid in &pids {
        // Q1
        let sets: Vec<BTreeSet<i64>> = loaded
            .iter()
            .map(|l| l.eng.q1(*pid, T_MID).map(|v| v.into_iter().collect()))
            .collect::<Result<_>>()
            .unwrap_or_else(|e| {
                println!("  DIFF ERROR q1 pid={pid}: {e}");
                Vec::new()
            });
        totals[0] += sets.first().map_or(0, |s| s.len());
        ok &= report_mismatch("q1", *pid, loaded, &sets);
        // Q2
        let sets: Vec<BTreeSet<(i64, i64)>> = loaded
            .iter()
            .map(|l| l.eng.q2(*pid).map(|v| v.into_iter().collect()))
            .collect::<Result<_>>()
            .unwrap_or_else(|e| {
                println!("  DIFF ERROR q2 pid={pid}: {e}");
                Vec::new()
            });
        totals[1] += sets.first().map_or(0, |s| s.len());
        ok &= report_mismatch("q2", *pid, loaded, &sets);
        // Q7b
        let sets: Vec<BTreeSet<i64>> = loaded
            .iter()
            .map(|l| l.eng.q7b(*pid, Q7B_LO, Q7B_HI).map(|v| v.into_iter().collect()))
            .collect::<Result<_>>()
            .unwrap_or_else(|e| {
                println!("  DIFF ERROR q7b pid={pid}: {e}");
                Vec::new()
            });
        totals[2] += sets.first().map_or(0, |s| s.len());
        ok &= report_mismatch("q7b", *pid, loaded, &sets);
    }
    if ok {
        println!(
            "  differential: IDENTICAL result sets across {} engines, {} pids x q1/q2/q7b \
             (aggregate rows compared: q1={} q2={} q7b={})",
            loaded.len(),
            pids.len(),
            totals[0],
            totals[1],
            totals[2]
        );
    }
    ok
}

fn report_mismatch<T: Ord + Clone + std::fmt::Debug>(
    q: &str,
    pid: i64,
    loaded: &[Loaded],
    sets: &[BTreeSet<T>],
) -> bool {
    if sets.is_empty() {
        return false;
    }
    let mut ok = true;
    for i in 1..sets.len() {
        if sets[i] != sets[0] {
            ok = false;
            let only_first: Vec<_> = sets[0].difference(&sets[i]).take(5).cloned().collect();
            let only_other: Vec<_> = sets[i].difference(&sets[0]).take(5).cloned().collect();
            println!(
                "  MISMATCH {q} pid={pid}: {}={} rows vs {}={} rows; only-{}: {:?}; only-{}: {:?}",
                loaded[0].name,
                sets[0].len(),
                loaded[i].name,
                sets[i].len(),
                loaded[0].name,
                only_first,
                loaded[i].name,
                only_other,
            );
        }
    }
    ok
}

fn run_scale(scale_name: &str, live: &[(&'static str, Factory)]) -> Result<()> {
    let cfg = SCALES
        .iter()
        .find(|c| c.name == scale_name)
        .unwrap_or_else(|| panic!("unknown scale {scale_name}"));
    let ds = generate(cfg);
    println!(
        "\n=== SCALE {} — persons={} events={} groups={} member_of={} subgroup_of={} expects={} attends={} (ChaCha8 seed 42) ===",
        cfg.name,
        ds.persons,
        ds.events.len(),
        ds.groups,
        ds.member_of.len(),
        ds.subgroup_of.len(),
        ds.expects.len(),
        ds.attends.len()
    );

    let mut loaded: Vec<Loaded> = Vec::new();
    for (_, factory) in live {
        let mut eng = factory()?;
        let name = eng.name();
        let t0 = Instant::now();
        eng.load(&ds)?;
        let load_ms = t0.elapsed().as_secs_f64() * 1000.0;
        println!("  {name:8} load: {load_ms:9.1} ms");
        loaded.push(Loaded { name, eng, load_ms });
    }

    if !differential(&loaded, &ds) {
        println!("  !! differential check FAILED at scale {} — timings below are for diagnosis only", cfg.name);
    }

    // Q1 at BENCH_PID can be empty (e.g. pid 7 at M under seed 42: all
    // memberships expired). Deterministic second Q1 cell: the pid with the
    // largest Q1 result among the differential pids (engine-neutral — results
    // are set-identical across engines; ties break to the smallest pid).
    let q1_max_pid = {
        let pids: Vec<i64> = PIDS.iter().copied().filter(|p| *p < ds.persons).collect();
        let mut best = (0usize, pids[0]);
        for pid in pids {
            let n = loaded[0].eng.q1(pid, T_MID)?.len();
            if n > best.0 {
                best = (n, pid);
            }
        }
        println!("  q1 secondary bench pid: {} (|q1| = {})", best.1, best.0);
        best.1
    };

    println!(
        "\n  {:22} {}",
        format!("query (pid={BENCH_PID})"),
        loaded
            .iter()
            .map(|l| format!("{:>30}", l.name))
            .collect::<Vec<_>>()
            .join(" ")
    );
    println!("  {}", "-".repeat(24 + 31 * loaded.len()));

    let mut rows: Vec<(String, Vec<String>)> = Vec::new();
    for (label, mode) in [("materialised", true), ("count-only", false)] {
        for q in ["q1", "q1x", "q2", "q7b"] {
            let mut cells = Vec::new();
            for l in &loaded {
                let cell = match (q, mode) {
                    ("q1", true) => bench(RUNS, || l.eng.q1(BENCH_PID, T_MID)).map(|(c, _)| c),
                    ("q1", false) => bench(RUNS, || l.eng.q1_count(BENCH_PID, T_MID)).map(|(c, _)| c),
                    ("q1x", true) => bench(RUNS, || l.eng.q1(q1_max_pid, T_MID)).map(|(c, _)| c),
                    ("q1x", false) => {
                        bench(RUNS, || l.eng.q1_count(q1_max_pid, T_MID)).map(|(c, _)| c)
                    }
                    ("q2", true) => bench(RUNS, || l.eng.q2(BENCH_PID)).map(|(c, _)| c),
                    ("q2", false) => bench(RUNS, || l.eng.q2_count(BENCH_PID)).map(|(c, _)| c),
                    ("q7b", true) => {
                        bench(RUNS, || l.eng.q7b(BENCH_PID, Q7B_LO, Q7B_HI)).map(|(c, _)| c)
                    }
                    ("q7b", false) => {
                        bench(RUNS, || l.eng.q7b_count(BENCH_PID, Q7B_LO, Q7B_HI)).map(|(c, _)| c)
                    }
                    _ => unreachable!(),
                };
                match cell {
                    Ok(c) => cells.push(format!(
                        "{:>9.2}/{:<9.2} {}",
                        c.median_ms, c.max_ms, c.check
                    )),
                    Err(e) => cells.push(format!("ERROR: {e}")),
                }
            }
            let ql = if q == "q1x" {
                format!("q1 pid={q1_max_pid} {label}")
            } else {
                format!("{q} {label}")
            };
            rows.push((ql, cells));
        }
    }
    for (label, cells) in rows {
        println!(
            "  {:22} {}",
            label,
            cells.iter().map(|c| format!("{c:>30}")).collect::<Vec<_>>().join(" ")
        );
    }
    println!("\n  load summary: {}", loaded.iter().map(|l| format!("{}={:.0}ms", l.name, l.load_ms)).collect::<Vec<_>>().join("  "));
    for l in &loaded {
        let n = l.eng.notes();
        if !n.is_empty() {
            println!("  note[{}]: {}", l.name, n);
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.first().map(String::as_str) == Some("debug-q2") {
        return debug_q2();
    }
    let scales: Vec<String> = if args.is_empty() {
        vec!["S".into(), "M".into()]
    } else {
        args
    };

    println!("=== smoke tests (fresh instances) ===");
    let mut live: Vec<(&'static str, Factory)> = Vec::new();
    for (name, factory) in factories() {
        if smoke(name, factory) {
            live.push((name, factory));
        } else {
            println!("  {name:8} EXCLUDED from benchmarks (smoke failure is a first-class finding)");
        }
    }

    for s in &scales {
        run_scale(s, &live)?;
    }
    Ok(())
}

/// Reproduction of the grafeo 0.5.42 correctness bug the differential check
/// caught (run: `cargo run --release -- debug-q2`): with TWO separately
/// anchored patterns `(p:Person {id:3})-[a1:attends]->(e1), (p2:Person
/// {id:3})-[a2:attends]->(e2)`, grafeo returns the pair (8, 178) although
/// person 3's intervals are e8=[694800,698400) and e178=[619200,622800) —
/// non-overlapping (694800 < 622800 is false). The shared-variable form
/// `(p)-[a1]->(e1), (p)-[a2]->(e2)` returns the correct empty set and agrees
/// with agdb and cozo; the harness uses that form.
fn debug_q2() -> Result<()> {
    let cfg = &SCALES[0];
    let ds = generate(cfg);
    println!("person 3 attends rows (event, start, end):");
    for a in ds.attends.iter().filter(|a| a.person == 3) {
        println!("  e={} [{}, {})", a.event, a.start_ts, a.end_ts);
    }
    for ev in [8i64, 178] {
        let e = &ds.events[ev as usize];
        println!("event {} start={} end={}", ev, e.start_ts, e.end_ts);
    }
    let mut g = eng_grafeo::GrafeoEngine::new()?;
    use engine::Engine as _;
    g.load(&ds)?;
    println!("grafeo q2(3) shared-p form = {:?}", g.q2(3)?);
    println!(
        "grafeo dual-anchor form (BUGGY) = {:?}",
        g.raw_pairs(
            "MATCH (p:Person {id: 3})-[a1:attends]->(e1:Event), \
                    (p2:Person {id: 3})-[a2:attends]->(e2:Event) \
             WHERE e1.id < e2.id AND a1.start_ts < a2.end_ts AND a2.start_ts < a1.end_ts \
             RETURN e1.id, e2.id"
        )?
    );
    Ok(())
}
