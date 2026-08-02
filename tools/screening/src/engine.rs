//! The one small trait every candidate implements, plus bench helpers.

use crate::data::Dataset;
use anyhow::Result;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::Instant;

pub trait Engine {
    fn name(&self) -> &'static str;

    /// Bulk load the dataset (fresh instance). Timed by the caller.
    fn load(&mut self, ds: &Dataset) -> Result<()>;

    /// Basic create/read/update/delete round trip on a FRESH instance,
    /// using entities disjoint from the benchmark data.
    fn crud_smoke(&mut self) -> Result<()>;

    /// Q1 derived expansion: memberships of `pid` valid at `t` -> ancestor
    /// closure via subgroup_of edges each valid at `t`, depth <= 5 INCLUDING
    /// depth 0 -> expects valid at `t` -> deduplicated event-id set.
    fn q1(&self, pid: i64, t: i64) -> Result<Vec<i64>>;
    fn q1_count(&self, pid: i64, t: i64) -> Result<i64>;

    /// Q2: unordered event pairs (e1 < e2) both attended by `pid` whose
    /// attends intervals overlap. Returns the pair set.
    fn q2(&self, pid: i64) -> Result<Vec<(i64, i64)>>;
    fn q2_count(&self, pid: i64) -> Result<i64>;

    /// Q7b: distinct persons q != pid sharing an event with pid where the
    /// EVENT's start_ts is in [lo, hi]. Returns the person-id set.
    fn q7b(&self, pid: i64, lo: i64, hi: i64) -> Result<Vec<i64>>;
    fn q7b_count(&self, pid: i64, lo: i64, hi: i64) -> Result<i64>;

    /// One-line description of any expressiveness fallback the engine had to
    /// take (e.g. which Q1 syntax form actually ran). Empty = none.
    fn notes(&self) -> String {
        String::new()
    }
}

pub struct BenchCell {
    pub median_ms: f64,
    pub max_ms: f64,
    /// len + order-independent hash of the last run's materialised result
    /// ("-" for count-only mode, where this holds the count).
    pub check: String,
}

pub fn bench<T, F>(runs: usize, mut f: F) -> Result<(BenchCell, T)>
where
    T: Sized,
    F: FnMut() -> Result<T>,
    T: ResultDigest,
{
    let mut times = Vec::with_capacity(runs);
    let mut last: Option<T> = None;
    for _ in 0..runs {
        let t0 = Instant::now();
        let out = f()?;
        let out = std::hint::black_box(out);
        times.push(t0.elapsed().as_secs_f64() * 1000.0);
        last = Some(out);
    }
    times.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let median = times[times.len() / 2];
    let max = *times.last().unwrap();
    let last = last.unwrap();
    Ok((
        BenchCell { median_ms: median, max_ms: max, check: last.digest() },
        last,
    ))
}

/// Order-independent digest so cross-engine equality can be eyeballed
/// (the real differential check compares full sets).
pub trait ResultDigest {
    fn digest(&self) -> String;
}

fn hash_sorted<T: Hash + Ord + Clone>(v: &[T]) -> u64 {
    let mut sorted: Vec<T> = v.to_vec();
    sorted.sort();
    let mut h = DefaultHasher::new();
    sorted.hash(&mut h);
    h.finish()
}

impl ResultDigest for Vec<i64> {
    fn digest(&self) -> String {
        format!("n={} h={:016x}", self.len(), hash_sorted(self))
    }
}

impl ResultDigest for Vec<(i64, i64)> {
    fn digest(&self) -> String {
        format!("n={} h={:016x}", self.len(), hash_sorted(self))
    }
}

impl ResultDigest for i64 {
    fn digest(&self) -> String {
        format!("count={self}")
    }
}
