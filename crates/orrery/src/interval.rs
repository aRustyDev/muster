//! Timestamps and half-open intervals — the one piece of interval machinery
//! every conflict check shares (SPEC-00 invariant 1).
//!
//! Time representation: `i64` microseconds UTC as the comparison key. Safe
//! under every QUESTION-0014 option (all store an instant); what that
//! question leaves open — authoring-zone retention, recurrence — does not
//! affect this algebra. `chrono` stays at the API boundary and is not a
//! dependency here.

use serde::{Deserialize, Serialize};

use crate::error::{OrreryError, Result};

/// Microseconds since the Unix epoch, UTC.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
pub struct Timestamp(pub i64);

impl Timestamp {
    pub const MIN: Timestamp = Timestamp(i64::MIN);
    pub const MAX: Timestamp = Timestamp(i64::MAX);

    pub fn micros(self) -> i64 {
        self.0
    }
}

/// Half-open interval `[start, end)`.
///
/// Construction rejects inverted intervals; zero-length intervals only via
/// the explicit [`Interval::at_point`] (Rule 04).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Interval {
    start: Timestamp,
    end: Timestamp,
}

impl Interval {
    /// A positive-length interval. `end <= start` is rejected at
    /// construction, not at use.
    pub fn new(start: Timestamp, end: Timestamp) -> Result<Self> {
        if end <= start {
            return Err(OrreryError::InvalidInterval {
                reason: "end must be strictly after start (use at_point for zero-length)",
            });
        }
        Ok(Interval { start, end })
    }

    /// The explicitly-permitted zero-length interval `[t, t)`.
    pub fn at_point(t: Timestamp) -> Self {
        Interval { start: t, end: t }
    }

    pub fn start(&self) -> Timestamp {
        self.start
    }

    pub fn end(&self) -> Timestamp {
        self.end
    }

    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    /// Duration in microseconds.
    pub fn duration_micros(&self) -> i64 {
        self.end.0 - self.start.0
    }

    /// The overlap predicate every detector shares:
    /// `a.start < b.end && b.start < a.end`. Half-open, so intervals that
    /// merely meet do not overlap, and an empty interval (an empty set of
    /// instants) overlaps nothing.
    pub fn overlaps(&self, other: &Interval) -> bool {
        !self.is_empty() && !other.is_empty() && self.start < other.end && other.start < self.end
    }

    /// Whether an instant falls inside `[start, end)`.
    pub fn contains_point(&self, t: Timestamp) -> bool {
        self.start <= t && t < self.end
    }

    /// Whether `other` lies entirely within `self` (both half-open).
    pub fn contains_interval(&self, other: &Interval) -> bool {
        self.start <= other.start && other.end <= self.end
    }

    /// `[a, b)` meets `[b, c)`.
    pub fn meets(&self, other: &Interval) -> bool {
        self.end == other.start
    }

    /// Union of two intervals that overlap or meet; `None` when a gap
    /// separates them (the union would not be an interval).
    pub fn merge(&self, other: &Interval) -> Option<Interval> {
        if self.overlaps(other) || self.meets(other) || other.meets(self) {
            Some(Interval {
                start: self.start.min(other.start),
                end: self.end.max(other.end),
            })
        } else {
            None
        }
    }

    /// The Allen relation from `self` to `other`.
    ///
    /// Defined for positive-length intervals; callers holding possibly-empty
    /// intervals check `is_empty` first (Allen's algebra is not defined for
    /// points).
    pub fn allen(&self, other: &Interval) -> AllenRelation {
        use std::cmp::Ordering::*;
        let (a, b) = (self, other);
        match (
            a.start.cmp(&b.start),
            a.end.cmp(&b.end),
            a.end.cmp(&b.start),
            b.end.cmp(&a.start),
        ) {
            (Equal, Equal, _, _) => AllenRelation::Equals,
            (_, _, Less, _) => AllenRelation::Before,
            (_, _, Equal, _) => AllenRelation::Meets,
            (_, _, _, Less) => AllenRelation::After,
            (_, _, _, Equal) => AllenRelation::MetBy,
            (Less, Less, _, _) => AllenRelation::Overlaps,
            (Less, Equal, _, _) => AllenRelation::FinishedBy,
            (Less, Greater, _, _) => AllenRelation::Contains,
            (Equal, Less, _, _) => AllenRelation::Starts,
            (Equal, Greater, _, _) => AllenRelation::StartedBy,
            (Greater, Less, _, _) => AllenRelation::During,
            (Greater, Equal, _, _) => AllenRelation::Finishes,
            (Greater, Greater, _, _) => AllenRelation::OverlappedBy,
        }
    }
}

/// Allen's thirteen interval relations, `self R other`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AllenRelation {
    Before,
    Meets,
    Overlaps,
    FinishedBy,
    Contains,
    Starts,
    Equals,
    StartedBy,
    During,
    Finishes,
    OverlappedBy,
    MetBy,
    After,
}

impl AllenRelation {
    /// The converse relation: `a R b  ⇔  b R.converse() a`.
    pub fn converse(self) -> AllenRelation {
        use AllenRelation::*;
        match self {
            Before => After,
            After => Before,
            Meets => MetBy,
            MetBy => Meets,
            Overlaps => OverlappedBy,
            OverlappedBy => Overlaps,
            FinishedBy => Finishes,
            Finishes => FinishedBy,
            Contains => During,
            During => Contains,
            Starts => StartedBy,
            StartedBy => Starts,
            Equals => Equals,
        }
    }

    /// Relations under which the two intervals share at least one instant
    /// (half-open semantics: Meets/MetBy share none).
    pub fn implies_overlap(self) -> bool {
        use AllenRelation::*;
        !matches!(self, Before | After | Meets | MetBy)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn iv(s: i64, e: i64) -> Interval {
        Interval::new(Timestamp(s), Timestamp(e)).unwrap()
    }

    #[test]
    fn inverted_rejected_zero_length_explicit() {
        assert!(Interval::new(Timestamp(5), Timestamp(5)).is_err());
        assert!(Interval::new(Timestamp(5), Timestamp(4)).is_err());
        let p = Interval::at_point(Timestamp(5));
        assert!(p.is_empty());
        assert!(!p.overlaps(&iv(0, 10)));
    }

    #[test]
    fn half_open_adjacent_do_not_overlap() {
        let a = iv(0, 10);
        let b = iv(10, 20);
        assert!(!a.overlaps(&b));
        assert!(a.meets(&b));
        assert_eq!(a.allen(&b), AllenRelation::Meets);
        assert_eq!(b.allen(&a), AllenRelation::MetBy);
    }

    #[test]
    fn allen_thirteen_examples() {
        use AllenRelation::*;
        let cases = [
            (iv(0, 2), iv(5, 8), Before),
            (iv(0, 5), iv(5, 8), Meets),
            (iv(0, 6), iv(5, 8), Overlaps),
            (iv(0, 8), iv(5, 8), FinishedBy),
            (iv(0, 9), iv(5, 8), Contains),
            (iv(5, 6), iv(5, 8), Starts),
            (iv(5, 8), iv(5, 8), Equals),
            (iv(5, 9), iv(5, 8), StartedBy),
            (iv(6, 7), iv(5, 8), During),
            (iv(6, 8), iv(5, 8), Finishes),
            (iv(6, 9), iv(5, 8), OverlappedBy),
            (iv(8, 9), iv(5, 8), MetBy),
            (iv(9, 12), iv(5, 8), After),
        ];
        for (a, b, want) in cases {
            assert_eq!(a.allen(&b), want, "{a:?} vs {b:?}");
            assert_eq!(b.allen(&a), want.converse(), "converse of {want:?}");
        }
    }

    #[test]
    fn merge_only_when_touching() {
        assert_eq!(iv(0, 5).merge(&iv(5, 9)), Some(iv(0, 9)));
        assert_eq!(iv(0, 6).merge(&iv(5, 9)), Some(iv(0, 9)));
        assert_eq!(iv(0, 4).merge(&iv(5, 9)), None);
    }
}
