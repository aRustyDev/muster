//! Phase 5 H1: greedy discharges the interval-graph-colouring optimality
//! claim against TWO independent oracles (muster-sdk/SPEC-03):
//! brute-force k^n enumeration, and the max-overlap ≤ k criterion
//! (interval graphs are perfect, so all three must agree).

use muster_sdk::{assign, RoomOption, RoomRequest};
use orrery::interval::{Interval, Timestamp};
use orrery::model::{EventId, LocationId};
use proptest::prelude::*;

fn overlaps(a: &Interval, b: &Interval) -> bool {
    // Local restatement for the oracle (test code; the SDK itself never
    // re-implements this — H3).
    a.start().max(b.start()) < a.end().min(b.end())
}

/// Oracle 1: does ANY complete zero-conflict assignment exist? k^n search
/// with pruning.
fn brute_force_feasible(windows: &[Interval], k: usize) -> bool {
    fn go(windows: &[Interval], k: usize, i: usize, rooms: &mut Vec<Vec<usize>>) -> bool {
        if i == windows.len() {
            return true;
        }
        for r in 0..k {
            if rooms[r]
                .iter()
                .all(|&j| !overlaps(&windows[i], &windows[j]))
            {
                rooms[r].push(i);
                if go(windows, k, i + 1, rooms) {
                    return true;
                }
                rooms[r].pop();
            }
        }
        false
    }
    go(windows, k, 0, &mut vec![Vec::new(); k])
}

/// Oracle 2: perfection of interval graphs — feasible iff no instant is
/// covered by more than k windows.
fn max_overlap(windows: &[Interval]) -> usize {
    let mut points: Vec<(Timestamp, i32)> = Vec::new();
    for w in windows {
        points.push((w.start(), 1));
        points.push((w.end(), -1));
    }
    // Half-open: ends sort before starts at the same instant.
    points.sort_by_key(|(t, d)| (*t, *d));
    let (mut cur, mut max) = (0i32, 0i32);
    for (_, d) in points {
        cur += d;
        max = max.max(cur);
    }
    max as usize
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(48))]
    #[test]
    fn optimality_greedy_matches_both_oracles(
        specs in proptest::collection::vec((0i64..200, 1i64..80), 1..13),
        k in 1usize..4,
    ) {
        let windows: Vec<Interval> = specs
            .iter()
            .map(|(s, len)| Interval::new(Timestamp(*s), Timestamp(s + len)).unwrap())
            .collect();
        let requests: Vec<RoomRequest> = windows
            .iter()
            .map(|w| RoomRequest { event: EventId::new(), window: *w, expected_size: None })
            .collect();
        let rooms: Vec<RoomOption> = (0..k)
            .map(|_| RoomOption { location: LocationId::new(), capacity: None })
            .collect();

        let (placements, unassigned) = assign::greedy(&requests, &rooms);

        // Greedy's own output must be internally conflict-free.
        for a in &placements {
            for b in &placements {
                if a.event != b.event && a.location == b.location {
                    let wa = &requests.iter().find(|r| r.event == a.event).unwrap().window;
                    let wb = &requests.iter().find(|r| r.event == b.event).unwrap().window;
                    prop_assert!(!overlaps(wa, wb), "greedy produced a conflict");
                }
            }
        }

        let greedy_complete = unassigned.is_empty();
        let brute = brute_force_feasible(&windows, k);
        let overlap_ok = max_overlap(&windows) <= k;

        prop_assert_eq!(brute, overlap_ok, "the two oracles must agree (perfect graphs)");
        prop_assert_eq!(
            greedy_complete, brute,
            "greedy finds a complete assignment iff one exists (n={}, k={})",
            windows.len(), k
        );
    }
}
