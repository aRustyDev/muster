//! H4 (slice-2 pre-commitment): first-ever interactive latency measurement
//! (plan-review MO-2 — none existed). `select()` deliberately runs the
//! whole-window sweep for immediacy; the hypothesis is that this fits the
//! muster/SPEC-00 100 ms budget at Prototype scale (10³ persons, ~5×10³
//! attends). Numbers are printed for the phase doc (run with
//! `cargo nextest run -p muster -E 'test(measure_)' --no-capture`).
//!
//! Corrected 2026-08-03 (QF slice / QR-2 W-2, finding F-5): this header
//! used to claim "the assertion is intentionally the pre-committed
//! threshold" while the shipped assertion was — and remains — an
//! order-of-magnitude sanity bound (p50 < 1 s), because the measured H4
//! verdict sat on a knife edge (97.8 ms p50 vs a 100 ms budget) and a
//! threshold assertion would make the suite flaky. The H4 verdict lives
//! in phases/06-app.md, not here. Same date, variance treatment added
//! per the W-2 policy: untimed warm-up, then 3 measured rounds; the
//! recorded statistic is the median of round p50s (max also printed).

use std::time::Instant;

use muster::MusterService;
use orrery::command::Command;
use orrery::engine::Engine;
use orrery::interval::{Interval, Timestamp};
use orrery::model::{Event, EventId, Location, LocationId, Person, PersonId, Portal, Tier};
use orrery::repo::memory::MemoryRepo;
use orrery::repo::Repository;

const HOUR: i64 = 3_600 * 1_000_000;

fn iv(a: i64, b: i64) -> Interval {
    Interval::new(Timestamp(a * HOUR), Timestamp(b * HOUR)).unwrap()
}

#[test]
fn measure_select_latency_at_prototype_scale() {
    const PERSONS: usize = 1_000;
    const EVENTS: usize = 20;
    const SELECTS_PER_PERSON: usize = 5;

    let repo = MemoryRepo::new();
    let mut rooms = Vec::new();
    for i in 0..EVENTS {
        let room = Location {
            id: LocationId::new(),
            name: format!("Room {i}"),
            tier: Tier::Room,
            portal: Portal::None,
            capacity: Some(10_000),
            ext: Default::default(),
        };
        rooms.push(room.id);
        repo.apply(Command::UpsertLocation(room)).unwrap();
    }
    let mut events = Vec::new();
    for (i, &room) in rooms.iter().enumerate() {
        let id = EventId::new();
        let start = (i % 12) as i64;
        let ev = Event {
            id,
            name: format!("Session {i}"),
            window: iv(start, start + 1),
            kind: "session".into(),
            timezone: None,
            ext: Default::default(),
        };
        let during = ev.window;
        repo.apply(Command::UpsertEvent(ev)).unwrap();
        repo.apply(Command::HoldLocation {
            location: room,
            event: id,
            during,
            overflow_for: None,
            capacity_override: None,
        })
        .unwrap();
        events.push(id);
    }
    let mut persons = Vec::new();
    for i in 0..PERSONS {
        let id = PersonId::new();
        repo.apply(Command::UpsertPerson(Person {
            id,
            name: format!("p{i}"),
            derived_digest: None,
            ext: Default::default(),
        }))
        .unwrap();
        persons.push(id);
    }
    // Non-conflicting spread: person i attends events i, i+4, i+8, ... so
    // the population carries ~5×10³ attends without drowning in violations
    // (the measurement is sweep cost, not violation churn).
    for (i, p) in persons.iter().enumerate() {
        for k in 0..SELECTS_PER_PERSON {
            let e = events[(i + k * 4) % EVENTS];
            repo.apply(Command::AddAttendance {
                person: *p,
                event: e,
                priority: None,
            })
            .unwrap();
        }
    }

    let day = iv(0, 24);
    let now = Timestamp(0);
    let mut svc = MusterService::new(Engine::new(repo).unwrap());

    // Measure interactive select() — the command + whole-window sweep +
    // conflict readback path a member hits per click.
    let subject = persons[0];

    // Untimed warm-up (W-2): first-call effects (lazy allocation, page
    // faults) belong to no round's distribution. Re-selecting an already
    // attended event is an upsert, so repeated rounds are well-defined.
    for k in 0..3 {
        svc.select(subject, events[(1 + k * 3) % EVENTS], Some(0.5), now, day)
            .unwrap();
    }

    // W-2 variance treatment: 3 measured rounds × 20 samples each.
    let mut round_p50: Vec<u128> = Vec::new();
    let mut round_p95: Vec<u128> = Vec::new();
    for _round in 0..3 {
        let mut times_us: Vec<u128> = Vec::new();
        for k in 0..20 {
            let e = events[(1 + k * 3) % EVENTS];
            let t0 = Instant::now();
            svc.select(subject, e, Some(0.5), now, day).unwrap();
            times_us.push(t0.elapsed().as_micros());
        }
        times_us.sort();
        round_p50.push(times_us[times_us.len() / 2]);
        round_p95.push(times_us[(times_us.len() * 95) / 100 - 1]);
    }
    let mut p50s = round_p50.clone();
    p50s.sort();
    let p50_med = p50s[p50s.len() / 2];
    let p50_max = *p50s.last().unwrap();

    let t0 = Instant::now();
    let _ = svc.my_schedule(subject, day, now).unwrap();
    let sched_us = t0.elapsed().as_micros();
    let t0 = Instant::now();
    let _ = svc.events(day).unwrap();
    let browse_us = t0.elapsed().as_micros();

    eprintln!(
        "measure_select @ {PERSONS} persons / {} attends: \
         select p50/round={round_p50:?}us p95/round={round_p95:?}us \
         (median-of-p50 {p50_med}us, max {p50_max}us); \
         my_schedule={sched_us}us; events={browse_us}us",
        PERSONS * SELECTS_PER_PERSON
    );

    // H4 verdict lives in phases/06-app.md, not here: measured 2026-08-02
    // at p50=97.8ms / p95=102.4ms — under the 100 ms budget on the letter
    // (p50), over it at p95, zero headroom. Recorded as refuted-in-spirit;
    // person-scoped evaluation is pre-committed for Alpha. A knife-edge
    // assertion would make the suite flaky, so this asserts only a sanity
    // bound against order-of-magnitude regression.
    assert!(
        p50_med < 1_000_000,
        "select median-of-round-p50 {p50_med}us regressed an order of \
         magnitude past the measured 2026-08-02 baseline (97.8ms) — see \
         phases/06-app.md H4"
    );
}
