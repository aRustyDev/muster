//! Phase 6a H4/H5 — the Orrery Alpha budget gate, measured at the 10⁵
//! scale defined in orrery/SPEC-03 (dated addition 2026-08-02): 10⁵
//! `attends` / 10³ persons / 10³ events / 200 locations, same thresholds
//! as the 10⁶ table.
//!
//! Method, fixed before measuring (Rule 01.1): MemoryRepo (the only
//! backend), untuned first pass, deterministic fixture (LCG, constant
//! seed), stride-sampled persons, p50/p95 from sorted samples. Budget
//! verdicts are taken from a **release** run — budgets compare against
//! optimized-build baselines (the 10⁶ table came from SQLite/Ladybug
//! binaries), and a debug-profile verdict would indict the compiler, not
//! the engine. Debug numbers may be recorded alongside for transparency.
//!
//! Variance treatment (added 2026-08-03, QF slice / QR-2 W-2, F-5): the
//! per-person classes carry within-run distributions (stride samples,
//! p50/p95) and now take one untimed warm-up call per block. The one-shot
//! rows (cold open, closure refresh, sweep) are deliberately NOT repeated
//! in-process — a second in-process run would measure warm salsa/cache
//! paths, not the cold semantics the budgets are written against. Their
//! run-to-run variance is taken at process level: run the whole harness
//! ≥3 times and record the median and max of every row in the phase doc
//! (the 01b screening discipline, restored).
//!
//! `#[ignore]` by design: the whole-horizon sweep at this scale costs
//! tens of seconds (dominated by `travel_best`'s linear closure scan),
//! which would tax every workspace gate run. Run explicitly:
//!
//! ```text
//! cargo nextest run -p orrery --release -E 'test(measure_alpha)' \
//!     --run-ignored all --no-capture
//! ```
//!
//! Verdicts live in phases/06a-engine-surfaces.md Results, not here; the
//! in-test assertions are order-of-magnitude sanity bounds only, so an
//! explicit run cannot silently pass a 10× regression.

use std::time::Instant;

use orrery::analytics;
use orrery::command::Command;
use orrery::derive;
use orrery::detect::{impossible_travel, time_conflict};
use orrery::engine::Engine;
use orrery::interval::{Interval, Timestamp};
use orrery::model::{
    Actor, Event, EventId, Group, GroupId, Location, LocationId, Mode, Obligation, Person,
    PersonId, Portal, Role, Tier, TravelProvenance, Traverse,
};
use orrery::repo::memory::MemoryRepo;
use orrery::repo::Repository;
use orrery::travel::ClosureScope;

const HOUR: i64 = 3_600 * 1_000_000;
const HORIZON_H: i64 = 336; // 14 days

const PERSONS: usize = 1_000;
const EVENTS: usize = 1_000;
const LOCATIONS: usize = 200;
const ATTENDS_PER_PERSON: usize = 100; // 10⁵ attends total
const LEAF_GROUPS: usize = 100;
const MID_GROUPS: usize = 10;
const EXPECTATIONS: usize = 200;

fn iv(a_us: i64, b_us: i64) -> Interval {
    Interval::new(Timestamp(a_us), Timestamp(b_us)).unwrap()
}

/// Deterministic 64-bit LCG (Knuth constants) — no `rand` in this crate.
fn lcg(state: &mut u64) -> u64 {
    *state = state
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    *state >> 33
}

fn pctl(sorted_us: &[u128], p: usize) -> u128 {
    sorted_us[(sorted_us.len() * p / 100)
        .saturating_sub(1)
        .min(sorted_us.len() - 1)]
}

fn sample<F: FnMut(PersonId)>(persons: &[PersonId], stride: usize, mut f: F) -> Vec<u128> {
    // One untimed warm-up call per block (W-2, 2026-08-03): first-call
    // effects belong to no sample's distribution.
    if let Some(p) = persons.first() {
        f(*p);
    }
    let mut out = Vec::new();
    for p in persons.iter().step_by(stride) {
        let t0 = Instant::now();
        f(*p);
        out.push(t0.elapsed().as_micros());
    }
    out.sort();
    out
}

#[test]
#[ignore = "measurement harness (tens of seconds) — see the module doc for the invocation"]
fn measure_alpha_budgets_at_1e5() {
    let mut rng: u64 = 0x006a_2026_0802;
    let horizon = iv(0, HORIZON_H * HOUR);
    let at = Timestamp((HORIZON_H / 2) * HOUR);

    // ---- fixture: 10⁵-scale world, seeded through the command layer ----
    let t_build = Instant::now();
    let repo = MemoryRepo::new();

    let locations: Vec<LocationId> = (0..LOCATIONS)
        .map(|i| {
            let l = Location {
                id: LocationId::new(),
                name: format!("bldg-{i}"),
                tier: Tier::Structure,
                portal: Portal::None,
                capacity: Some(30),
                ext: Default::default(),
            };
            let id = l.id;
            repo.apply(Command::UpsertLocation(l)).unwrap();
            id
        })
        .collect();
    // Sparse Layer-1: a ring plus chords — all Structure-tier siblings.
    for i in 0..LOCATIONS {
        let edge = |to: usize, secs: i64| {
            repo.apply(Command::AddTraversePair(Traverse {
                from: locations[i],
                to: locations[to],
                mode: Mode("walk".into()),
                duration_typical_s: secs,
                duration_peak_s: None,
                peak_window: None,
                distance_m: None,
                provenance: TravelProvenance::Measured,
                computed_at: Timestamp(0),
                sibling_override: false,
            }))
            .unwrap();
        };
        edge((i + 1) % LOCATIONS, 300);
        if i % 5 == 0 {
            edge((i + 37) % LOCATIONS, 600);
        }
    }

    let events: Vec<(EventId, Interval)> = (0..EVENTS)
        .map(|i| {
            let start = (lcg(&mut rng) % (HORIZON_H as u64 - 1)) as i64 * HOUR;
            let w = iv(start, start + HOUR);
            let e = Event {
                id: EventId::new(),
                name: format!("ev-{i}"),
                window: w,
                kind: "session".into(),
                timezone: None,
                ext: Default::default(),
            };
            let id = e.id;
            repo.apply(Command::UpsertEvent(e)).unwrap();
            repo.apply(Command::HoldLocation {
                location: locations[i % LOCATIONS],
                event: id,
                during: w,
                overflow_for: None,
                capacity_override: None,
            })
            .unwrap();
            (id, w)
        })
        .collect();

    let leaf: Vec<GroupId> = (0..LEAF_GROUPS).map(|_| GroupId::new()).collect();
    let mid: Vec<GroupId> = (0..MID_GROUPS).map(|_| GroupId::new()).collect();
    let top = GroupId::new();
    for g in leaf.iter().chain(mid.iter()).chain([&top]) {
        repo.apply(Command::UpsertGroup(Group {
            id: *g,
            name: "g".into(),
            default_priority: None,
            timezone: None,
            ext: Default::default(),
        }))
        .unwrap();
    }
    for (i, g) in leaf.iter().enumerate() {
        repo.apply(Command::AddSubgroup {
            child: *g,
            parent: mid[i / MID_GROUPS],
            during: horizon,
        })
        .unwrap();
    }
    for m in &mid {
        repo.apply(Command::AddSubgroup {
            child: *m,
            parent: top,
            during: horizon,
        })
        .unwrap();
    }
    for k in 0..EXPECTATIONS {
        let group = match k % 10 {
            0..=6 => leaf[k % LEAF_GROUPS],
            7 | 8 => mid[k % MID_GROUPS],
            _ => top,
        };
        repo.apply(Command::AddExpectation {
            group,
            event: events[(lcg(&mut rng) as usize) % EVENTS].0,
            obligation: Obligation::Expected,
            default_priority: 0.5,
            during: horizon,
            cascades: k % 2 == 0,
            by: Actor::System,
        })
        .unwrap();
    }

    let persons: Vec<PersonId> = (0..PERSONS)
        .map(|i| {
            let p = Person {
                id: PersonId::new(),
                name: format!("p-{i}"),
                derived_digest: None,
                ext: Default::default(),
            };
            let id = p.id;
            repo.apply(Command::UpsertPerson(p)).unwrap();
            repo.apply(Command::AddMembership {
                person: id,
                group: leaf[i % LEAF_GROUPS],
                during: horizon,
                role: Role::Member,
            })
            .unwrap();
            id
        })
        .collect();
    for p in &persons {
        let mut chosen = [false; EVENTS];
        let mut n = 0;
        while n < ATTENDS_PER_PERSON {
            let ei = (lcg(&mut rng) as usize) % EVENTS;
            if chosen[ei] {
                continue;
            }
            chosen[ei] = true;
            n += 1;
            repo.apply(Command::AddAttendance {
                person: *p,
                event: events[ei].0,
                priority: Some(0.5),
            })
            .unwrap();
        }
    }
    let build_ms = t_build.elapsed().as_millis();

    // ---- cold open: Engine::new (mirror build) — budget < 1 s ----
    let t0 = Instant::now();
    let mut engine = Engine::new(repo).unwrap();
    let cold_open_us = t0.elapsed().as_micros();

    // ---- Layer-2 closure refresh — budget < 60 s ----
    let t0 = Instant::now();
    let closure = engine
        .refresh_closure(ClosureScope::EventBearing, at)
        .unwrap();
    let closure_us = t0.elapsed().as_micros();

    // ---- global conflict sweep, whole horizon — budget < 10 s ----
    let t0 = Instant::now();
    let sweep = engine.sweep(at, horizon).unwrap();
    let sweep_us = t0.elapsed().as_micros();

    // ---- per-person classes, stride-sampled — budgets < 25 ms p95 ----
    let repo = engine.repo();
    let expansion = sample(&persons, 5, |p| {
        derive::expand(repo, p, at).unwrap();
    });
    let detection = sample(&persons, 5, |p| {
        let attends = repo.attends_for(p, horizon).unwrap();
        time_conflict::detect(p, &attends);
    });
    // Travel feasibility replicates the engine's placed_for join: attends →
    // primary hold per event → consecutive-pair check via travel_best.
    let travel = sample(&persons, 10, |p| {
        let attends = repo.attends_for(p, horizon).unwrap();
        let mut placed = Vec::new();
        for a in attends {
            let mut holds = repo.held_for_event(a.event).unwrap();
            holds.sort_by_key(|h| (h.overflow_for.is_some(), h.location));
            if let Some(h) = holds.first() {
                placed.push(impossible_travel::Placed {
                    event: a.event,
                    location: h.location,
                    window: a.during,
                });
            }
        }
        impossible_travel::detect(p, &placed, &|f, t| repo.travel_best(f, t).ok().flatten());
    });
    // ---- bounded 2-hop co-attendance — budget < 50 ms p95 (H4) ----
    let two_hop = sample(&persons, 10, |p| {
        analytics::co_attendance(repo, p, horizon).unwrap();
    });

    let profile = if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    };
    eprintln!(
        "measure_alpha_budgets [{profile}] @ 1e5 attends / 1e3 persons / 1e3 events / 200 locations \
         (build {build_ms} ms)\n\
         cold_open           {:>10} us   (budget 1,000,000)\n\
         closure_refresh     {:>10} us   (budget 60,000,000; {} sources, {} pairs)\n\
         sweep               {:>10} us   (budget 10,000,000; emitted {}, resolved {})\n\
         expansion    p50 {:>7} p95 {:>7} us (budget p95 25,000; n={})\n\
         detection    p50 {:>7} p95 {:>7} us (budget p95 25,000; n={})\n\
         travel       p50 {:>7} p95 {:>7} us (budget p95 25,000; n={})\n\
         two_hop      p50 {:>7} p95 {:>7} us (budget p95 50,000; n={})",
        cold_open_us,
        closure_us,
        closure.sources,
        closure.pairs,
        sweep_us,
        sweep.emitted,
        sweep.resolved,
        pctl(&expansion, 50),
        pctl(&expansion, 95),
        expansion.len(),
        pctl(&detection, 50),
        pctl(&detection, 95),
        detection.len(),
        pctl(&travel, 50),
        pctl(&travel, 95),
        travel.len(),
        pctl(&two_hop, 50),
        pctl(&two_hop, 95),
        two_hop.len(),
    );

    // Order-of-magnitude sanity bounds only (10× each budget) — the real
    // verdicts are recorded in phases/06a-engine-surfaces.md from a
    // release run of this harness, knife-edges and all.
    assert!(cold_open_us < 10_000_000, "cold open 10x over budget");
    assert!(closure_us < 600_000_000, "closure refresh 10x over budget");
    assert!(sweep_us < 100_000_000, "sweep 10x over budget");
    for (name, sorted, budget_us) in [
        ("expansion", &expansion, 25_000u128),
        ("detection", &detection, 25_000),
        ("travel", &travel, 25_000),
        ("two_hop", &two_hop, 50_000),
    ] {
        let p95 = pctl(sorted, 95);
        assert!(
            p95 < budget_us * 10,
            "{name} p95 {p95}us is 10x over budget"
        );
    }
}
