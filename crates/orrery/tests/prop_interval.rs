//! Property tests: interval algebra vs an independent naive oracle
//! (orrery/SPEC-05 "fuzz against a reference implementation").

use orrery::{AllenRelation, Interval, Timestamp};
use proptest::prelude::*;

fn interval_strategy() -> impl Strategy<Value = Interval> {
    (-1_000i64..1_000, 1i64..200).prop_map(|(s, len)| {
        Interval::new(Timestamp(s), Timestamp(s + len)).expect("positive length by construction")
    })
}

/// Independent overlap oracle: two half-open intervals share an instant iff
/// max(start) < min(end).
fn naive_overlaps(a: &Interval, b: &Interval) -> bool {
    a.start().max(b.start()) < a.end().min(b.end())
}

/// Independent Allen classifier, structured differently from the
/// implementation (guard chain vs tuple match).
fn naive_allen(a: &Interval, b: &Interval) -> AllenRelation {
    use AllenRelation::*;
    if a.end() < b.start() {
        Before
    } else if a.end() == b.start() {
        Meets
    } else if b.end() < a.start() {
        After
    } else if b.end() == a.start() {
        MetBy
    } else {
        match (a.start().cmp(&b.start()), a.end().cmp(&b.end())) {
            (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => Overlaps,
            (std::cmp::Ordering::Less, std::cmp::Ordering::Equal) => FinishedBy,
            (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => Contains,
            (std::cmp::Ordering::Equal, std::cmp::Ordering::Less) => Starts,
            (std::cmp::Ordering::Equal, std::cmp::Ordering::Equal) => Equals,
            (std::cmp::Ordering::Equal, std::cmp::Ordering::Greater) => StartedBy,
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => During,
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Equal) => Finishes,
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => OverlappedBy,
        }
    }
}

proptest! {
    #[test]
    fn prop_overlap_matches_oracle(a in interval_strategy(), b in interval_strategy()) {
        prop_assert_eq!(a.overlaps(&b), naive_overlaps(&a, &b));
    }

    #[test]
    fn prop_overlap_symmetric(a in interval_strategy(), b in interval_strategy()) {
        prop_assert_eq!(a.overlaps(&b), b.overlaps(&a));
    }

    #[test]
    fn prop_allen_matches_oracle(a in interval_strategy(), b in interval_strategy()) {
        prop_assert_eq!(a.allen(&b), naive_allen(&a, &b));
    }

    #[test]
    fn prop_allen_converse(a in interval_strategy(), b in interval_strategy()) {
        prop_assert_eq!(b.allen(&a), a.allen(&b).converse());
    }

    #[test]
    fn prop_allen_iff_overlap(a in interval_strategy(), b in interval_strategy()) {
        prop_assert_eq!(a.allen(&b).implies_overlap(), a.overlaps(&b));
    }

    #[test]
    fn prop_merge_iff_touching(a in interval_strategy(), b in interval_strategy()) {
        let touching = a.overlaps(&b) || a.end() == b.start() || b.end() == a.start();
        match a.merge(&b) {
            Some(m) => {
                prop_assert!(touching);
                prop_assert_eq!(m.start(), a.start().min(b.start()));
                prop_assert_eq!(m.end(), a.end().max(b.end()));
            }
            None => prop_assert!(!touching),
        }
    }

    #[test]
    fn prop_new_rejects_non_positive(s in -1_000i64..1_000, d in -200i64..=0) {
        prop_assert!(Interval::new(Timestamp(s), Timestamp(s + d)).is_err());
    }
}
