//! Slice-2 search contracts: monotone/anytime (H5), the heterogeneous-
//! rooms Alpha gate (H6), stability-steered re-solves (H7), and
//! determinism under a fixed seed (H8).

use muster_sdk::objective::{EvalCtx, StabilityFromReference};
use muster_sdk::{assign, search, Objective, Placement, RoomOption, RoomRequest, SearchBudget};
use orrery::interval::{Interval, Timestamp};
use orrery::model::{EventId, LocationId};
use proptest::prelude::*;

fn iv(s: i64, e: i64) -> Interval {
    Interval::new(Timestamp(s), Timestamp(e)).unwrap()
}

/// Pure evaluation (no engine): utilisation only — enough to exercise the
/// search contracts without a repository.
fn util_eval<'a>(
    requests: &'a [RoomRequest],
    rooms: &'a [RoomOption],
) -> impl Fn(&[Placement]) -> muster_sdk::Breakdown + 'a {
    move |placements| {
        Objective::standard().evaluate(&EvalCtx {
            placements,
            requests,
            rooms,
            violations: &[],
        })
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(48))]
    #[test]
    fn monotone_improve_never_worse_than_seed(
        specs in proptest::collection::vec((0i64..200, 1i64..80, 1u32..300), 1..8),
        caps in proptest::collection::vec(1u32..300, 1..4),
        rng_seed in any::<u64>(),
        max_evals in 1u32..120,
    ) {
        let requests: Vec<RoomRequest> = specs
            .iter()
            .map(|(s, len, size)| RoomRequest {
                event: EventId::new(),
                window: iv(*s, s + len),
                expected_size: Some(*size),
            })
            .collect();
        let rooms: Vec<RoomOption> = caps
            .iter()
            .map(|c| RoomOption { location: LocationId::new(), capacity: Some(*c) })
            .collect();
        let (seed_placements, _) = assign::greedy(&requests, &rooms);
        let eval = util_eval(&requests, &rooms);
        let seed_total = eval(&seed_placements).total;

        let out = search::improve(
            &seed_placements,
            &rooms,
            &eval,
            SearchBudget { max_evals },
            rng_seed,
        );
        prop_assert!(
            out.breakdown.total <= seed_total + 1e-9,
            "monotone: {} > seed {}",
            out.breakdown.total,
            seed_total
        );
        prop_assert!(out.evals_used <= max_evals.max(1) + 1);
    }
}

/// H6 (SDK Alpha gate): time-ordered greedy commits the small room to the
/// early small event; the later, slightly larger event is forced into the
/// huge hall. Search finds the swap greedy could not foresee.
#[test]
fn search_improves_on_greedy_for_heterogeneous_rooms() {
    let (e1, e2) = (EventId::new(), EventId::new());
    let requests = vec![
        RoomRequest {
            event: e1,
            window: iv(0, 100),
            expected_size: Some(10),
        },
        RoomRequest {
            event: e2,
            window: iv(50, 150),
            expected_size: Some(15),
        },
    ];
    let rooms = vec![
        RoomOption {
            location: LocationId::new(),
            capacity: Some(20),
        },
        RoomOption {
            location: LocationId::new(),
            capacity: Some(200),
        },
    ];
    let (seed, unassigned) = assign::greedy(&requests, &rooms);
    assert!(unassigned.is_empty());

    let eval = util_eval(&requests, &rooms);
    let seed_total = eval(&seed).total;
    let out = search::improve(&seed, &rooms, &eval, SearchBudget::default(), 7);
    assert!(
        out.improved && out.breakdown.total < seed_total,
        "search must beat greedy here: seed {} vs {}",
        seed_total,
        out.breakdown.total
    );
}

/// H7: a room disappears; with the reference term, every placement whose
/// room still exists stays put — even where greedy's tie-break would have
/// moved it. Churn is confined to the displaced event.
#[test]
fn stability_confines_churn_to_displaced_events() {
    let events: Vec<EventId> = (0..3).map(|_| EventId::new()).collect();
    // Non-overlapping, so a single room could host everything — maximal
    // freedom for greedy to ignore the reference.
    let requests: Vec<RoomRequest> = (0..3)
        .map(|i| RoomRequest {
            event: events[i],
            window: iv(i as i64 * 200, i as i64 * 200 + 100),
            expected_size: None,
        })
        .collect();
    let mut ids: Vec<LocationId> = (0..3).map(|_| LocationId::new()).collect();
    ids.sort();
    let (r_low, r_high, r_gone) = (ids[0], ids[1], ids[2]);

    // Reference: e0→r_low, e1→r_high (greedy's tie-break would prefer
    // r_low for everything), e2→the room that will disappear.
    let reference = vec![
        Placement {
            event: events[0],
            location: r_low,
        },
        Placement {
            event: events[1],
            location: r_high,
        },
        Placement {
            event: events[2],
            location: r_gone,
        },
    ];
    let rooms_now = vec![
        RoomOption {
            location: r_low,
            capacity: None,
        },
        RoomOption {
            location: r_high,
            capacity: None,
        },
    ];

    let (seed, unassigned) = assign::greedy(&requests, &rooms_now);
    assert!(unassigned.is_empty());

    let objective = Objective::standard().with(StabilityFromReference {
        reference: reference.clone(),
        weight: 1.0,
    });
    let eval = |placements: &[Placement]| {
        objective.evaluate(&EvalCtx {
            placements,
            requests: &requests,
            rooms: &rooms_now,
            violations: &[],
        })
    };
    let out = search::improve(&seed, &rooms_now, &eval, SearchBudget::default(), 11);

    let room_of = |e: EventId| {
        out.placements
            .iter()
            .find(|p| p.event == e)
            .unwrap()
            .location
    };
    assert_eq!(room_of(events[0]), r_low, "unaffected placement stays");
    assert_eq!(
        room_of(events[1]),
        r_high,
        "stability overrides greedy's tie-break preference"
    );
    // e2's room is gone — it may land anywhere that exists; that is the
    // only churn permitted.
    assert!(room_of(events[2]) == r_low || room_of(events[2]) == r_high);
}

/// H8: fixed seed → identical outcome, twice.
#[test]
fn search_deterministic_under_fixed_seed() {
    let requests: Vec<RoomRequest> = (0..5)
        .map(|i| RoomRequest {
            event: EventId::new(),
            window: iv(i * 60, i * 60 + 90),
            expected_size: Some(10 + i as u32 * 30),
        })
        .collect();
    let rooms: Vec<RoomOption> = [25u32, 80, 300]
        .iter()
        .map(|c| RoomOption {
            location: LocationId::new(),
            capacity: Some(*c),
        })
        .collect();
    let (seed, _) = assign::greedy(&requests, &rooms);
    let eval = util_eval(&requests, &rooms);
    let a = search::improve(&seed, &rooms, &eval, SearchBudget::default(), 42);
    let b = search::improve(&seed, &rooms, &eval, SearchBudget::default(), 42);
    assert_eq!(a.placements, b.placements);
    assert_eq!(a.breakdown.total, b.breakdown.total);
    assert_eq!(a.evals_used, b.evals_used);
}
