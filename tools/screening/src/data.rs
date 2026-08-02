//! Seeded synthetic data — ports the SHAPE of `evidence/orrery_spike.py`'s
//! generator (counts, distributions), NOT its RNG bit-stream. ChaCha8 seed 42.
//! Distribution-equivalent, not bit-identical: absolute counts differ from the
//! Python runs; cross-engine identity on THIS data is what the harness checks.
//!
//! Only the entities Q1/Q2/Q7b touch: persons, groups, events, member_of,
//! subgroup_of, expects, attends.

use rand::seq::index::sample;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

pub const HOUR: i64 = 3600;
pub const DAY: i64 = 24 * HOUR;
pub const T_MID: i64 = 7 * DAY;
pub const FOREVER: i64 = 1_000_000_000;
pub const Q7B_LO: i64 = 300_000;
pub const Q7B_HI: i64 = 600_000;

#[derive(Clone, Copy)]
pub struct EventRow {
    pub id: i64,
    pub start_ts: i64,
    pub end_ts: i64,
    pub kind: u8, // index into KINDS
}

#[derive(Clone, Copy)]
pub struct MemberRow {
    pub person: i64,
    pub group: i64,
    pub valid_from: i64,
    pub valid_to: i64,
}

#[derive(Clone, Copy)]
pub struct SubgroupRow {
    pub child: i64,
    pub parent: i64,
    pub valid_from: i64,
    pub valid_to: i64,
}

#[derive(Clone, Copy)]
pub struct ExpectRow {
    pub group: i64,
    pub event: i64,
    pub obligation: u8, // index into OBLIGATIONS
    pub default_priority: f64,
    pub valid_from: i64,
    pub valid_to: i64,
}

#[derive(Clone, Copy)]
pub struct AttendRow {
    pub person: i64,
    pub event: i64,
    pub start_ts: i64,
    pub end_ts: i64,
    pub priority_person: f64,
    pub priority_coord: f64,
    pub source: u8, // index into SOURCES
}

pub const KINDS: [&str; 4] = ["lecture", "talk", "lab", "seminar"];
pub const OBLIGATIONS: [&str; 3] = ["mandatory", "expected", "recommended"];
pub const SOURCES: [&str; 3] = ["self", "group", "coordinator"];

pub struct Dataset {
    pub persons: i64,
    pub groups: i64,
    pub events: Vec<EventRow>,
    pub member_of: Vec<MemberRow>,
    pub subgroup_of: Vec<SubgroupRow>,
    pub expects: Vec<ExpectRow>,
    pub attends: Vec<AttendRow>,
}

pub struct ScaleCfg {
    pub name: &'static str,
    pub persons: i64,
    pub events: i64,
    pub groups: i64,
    pub per_person: usize,
}

pub const SCALES: [ScaleCfg; 3] = [
    ScaleCfg { name: "S", persons: 100, events: 200, groups: 10, per_person: 10 },
    ScaleCfg { name: "M", persons: 2000, events: 2000, groups: 60, per_person: 50 },
    ScaleCfg { name: "L", persons: 10_000, events: 10_000, groups: 200, per_person: 100 },
];

pub fn generate(cfg: &ScaleCfg) -> Dataset {
    let mut rng = ChaCha8Rng::seed_from_u64(42);
    let (p_n, e_n, g_n) = (cfg.persons, cfg.events, cfg.groups);

    // events: start on a 1800s grid within 14 days, duration 1|2|3 h (as Python)
    let mut events = Vec::with_capacity(e_n as usize);
    for i in 0..e_n {
        let st = rng.random_range(0..(14 * DAY / 1800)) * 1800;
        let dur = [HOUR, 2 * HOUR, 3 * HOUR][rng.random_range(0..3)];
        events.push(EventRow { id: i, start_ts: st, end_ts: st + dur, kind: rng.random_range(0..4) as u8 });
    }

    // group hierarchy: tree of branching 3, edge child -> parent, always valid
    let subgroup_of: Vec<SubgroupRow> = (1..g_n)
        .map(|g| SubgroupRow { child: g, parent: (g - 1) / 3, valid_from: 0, valid_to: FOREVER })
        .collect();

    // memberships: 1..3 distinct groups per person, ~10% expired at T_MID - DAY
    let mut member_of = Vec::new();
    for p in 0..p_n {
        let k = rng.random_range(1..4usize);
        for gi in sample(&mut rng, g_n as usize, k.min(g_n as usize)) {
            let expired = rng.random::<f64>() < 0.1;
            member_of.push(MemberRow {
                person: p,
                group: gi as i64,
                valid_from: 0,
                valid_to: if expired { T_MID - DAY } else { FOREVER },
            });
        }
    }

    // expectations: 2..7 distinct events per group, always valid
    let mut expects = Vec::new();
    for g in 0..g_n {
        let k = rng.random_range(2..8usize).min(e_n as usize);
        for ei in sample(&mut rng, e_n as usize, k) {
            expects.push(ExpectRow {
                group: g,
                event: ei as i64,
                obligation: rng.random_range(0..3) as u8,
                default_priority: (rng.random_range(20..=100) as f64) / 100.0,
                valid_from: 0,
                valid_to: FOREVER,
            });
        }
    }

    // attends: per_person distinct events per person; intervals copied from the
    // event (conflicts arise naturally from overlapping sampled events)
    let mut attends = Vec::new();
    for p in 0..p_n {
        for ei in sample(&mut rng, e_n as usize, cfg.per_person.min(e_n as usize)) {
            let ev = &events[ei];
            attends.push(AttendRow {
                person: p,
                event: ev.id,
                start_ts: ev.start_ts,
                end_ts: ev.end_ts,
                priority_person: rng.random_range(-100..=100) as f64 / 100.0,
                priority_coord: rng.random_range(-100..=100) as f64 / 100.0,
                source: rng.random_range(0..3) as u8,
            });
        }
    }

    Dataset { persons: p_n, groups: g_n, events, member_of, subgroup_of, expects, attends }
}

/// Micro-fixture for the per-hop temporal smoke test (probe-01 style).
/// Person 0: member of g0 (valid) and g3 (EXPIRED).
/// Chain: g0 -[valid]-> g1 -[EXPIRED]-> g2.
/// Expects: g0->e0 valid (depth-0 case), g1->e1 valid (per-hop pass),
///          g2->e2 valid (reachable only through the expired edge),
///          g0->e3 EXPIRED (expects-window case), g3->e4 valid (dead membership).
/// Correct Q1(pid=0, t=T_MID) = {0, 1}.
pub fn micro_fixture() -> Dataset {
    let ev = |id: i64| EventRow { id, start_ts: 1000, end_ts: 4600, kind: 0 };
    Dataset {
        persons: 1,
        groups: 4,
        events: (0..5).map(ev).collect(),
        member_of: vec![
            MemberRow { person: 0, group: 0, valid_from: 0, valid_to: FOREVER },
            MemberRow { person: 0, group: 3, valid_from: 0, valid_to: 100 },
        ],
        subgroup_of: vec![
            SubgroupRow { child: 0, parent: 1, valid_from: 0, valid_to: FOREVER },
            SubgroupRow { child: 1, parent: 2, valid_from: 0, valid_to: 100 },
        ],
        expects: vec![
            ExpectRow { group: 0, event: 0, obligation: 0, default_priority: 1.0, valid_from: 0, valid_to: FOREVER },
            ExpectRow { group: 1, event: 1, obligation: 0, default_priority: 1.0, valid_from: 0, valid_to: FOREVER },
            ExpectRow { group: 2, event: 2, obligation: 0, default_priority: 1.0, valid_from: 0, valid_to: FOREVER },
            ExpectRow { group: 0, event: 3, obligation: 0, default_priority: 1.0, valid_from: 0, valid_to: 100 },
            ExpectRow { group: 3, event: 4, obligation: 0, default_priority: 1.0, valid_from: 0, valid_to: FOREVER },
        ],
        attends: vec![],
    }
}
