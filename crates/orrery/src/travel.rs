//! Travel: Layer-1 pathfinding and the Layer-2 closure (ADR-0006/0009),
//! and the feasibility check whose signature lands now with `person`
//! ignored (Rule 00.5, ADR-0017).
//!
//! Layer 1 is the sparse ground truth: `traverse` edges (per mode). The
//! `within` containment edges act as zero-cost decomposition glue
//! (ADR-0009: room→exit + building→building + entrance→room, exit/entry ≈ 0
//! for v1) — but ONLY to bridge pairs the real network cannot reach: the
//! H1 property test refuted the naive design where connectors join the
//! graph outright (free intra-building shortcuts; travel with no network).
//! See phases/04-travel.md Results.
//!
//! Layer 2 is the materialised answer cache, restricted to event-bearing
//! locations, batch-recomputed through `Command::ReplaceClosure`.

use std::collections::HashMap;

use petgraph::graph::{DiGraph, NodeIndex};

use crate::interval::Timestamp;
use crate::model::{
    ClosureEntry, LocationId, Mode, PersonId, TravelCost, TravelProvenance, Traverse, Within,
};

/// The verdict a travel check returns. Verdicts carry durations only —
/// never coordinates, never anchor references (Rule 00.6/09).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Feasibility {
    Feasible {
        slack_s: i64,
    },
    Infeasible {
        deficit_s: i64,
        provenance: TravelProvenance,
    },
    /// No known route. Unknown is not an accusation: detectors emit nothing
    /// for it (phases/03-engine-core.md design decision).
    Unknown,
}

/// A placed event for travel purposes (re-exported from the detector).
pub use crate::detect::impossible_travel::Placed;

/// `feasible(person, a, b)` — Rule 00.5: the `person` parameter lands now
/// and is ignored until mobility profiles (ADR-0017). Caches key on
/// `(profile_id, from, to)`; everyone currently shares `default`.
pub fn feasible(
    person: PersonId,
    a: &Placed,
    b: &Placed,
    lookup: &dyn Fn(LocationId, LocationId) -> Option<TravelCost>,
) -> Feasibility {
    let _ = person; // ADR-0017: signature now, mobility later.
    if a.location == b.location {
        return Feasibility::Feasible {
            slack_s: (b.window.start().micros() - a.window.end().micros()) / 1_000_000,
        };
    }
    let Some(cost) = lookup(a.location, b.location) else {
        return Feasibility::Unknown;
    };
    let gap_s = (b.window.start().micros() - a.window.end().micros()) / 1_000_000;
    if gap_s >= cost.duration_s {
        Feasibility::Feasible {
            slack_s: gap_s - cost.duration_s,
        }
    } else {
        Feasibility::Infeasible {
            deficit_s: cost.duration_s - gap_s,
            provenance: cost.provenance,
        }
    }
}

/// Scope of a closure refresh.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClosureScope {
    /// All-pairs over event-bearing locations only (the default — bounds
    /// the cache and any future routing-API bill, ADR-0006).
    EventBearing,
    /// All-pairs over every location. Diagnostic use.
    AllLocations,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClosureReport {
    pub sources: usize,
    pub pairs: usize,
}

/// Compute the Layer-2 closure from Layer-1 edges. Pure: no repository, no
/// clock — `computed_at` is caller-supplied.
///
/// Per mode, two passes: Dijkstra over the real `traverse` network first
/// (authoritative); then, with zero-cost `within` connectors added, a
/// bridging pass fills only the pairs the real network cannot reach.
/// Entries carry `Estimated` provenance unless an un-bridged shortest cost
/// equals a direct edge of that mode, which donates its provenance.
pub fn compute_closure(
    traverse: &[Traverse],
    within: &[Within],
    targets: &[LocationId],
    computed_at: Timestamp,
) -> Vec<ClosureEntry> {
    let mut modes: Vec<&Mode> = traverse.iter().map(|t| &t.mode).collect();
    modes.sort_by(|a, b| a.0.cmp(&b.0));
    modes.dedup();

    // Two passes per mode (a Phase-4 H1 refutation drove this design — see
    // phases/04-travel.md Results): pure-traverse paths are authoritative;
    // zero-cost containment connectors would otherwise create free
    // intra-building shortcuts undercutting real room→room edges, and
    // "travel" between nested locations with no network at all. Connectors
    // therefore only BRIDGE pairs that are unreachable through the mode's
    // real edges.
    let mut entries = Vec::new();
    for mode in modes {
        let mode_edges: Vec<&Traverse> = traverse.iter().filter(|t| &t.mode == mode).collect();

        let mut graph: DiGraph<LocationId, i64> = DiGraph::new();
        let mut idx: HashMap<LocationId, NodeIndex> = HashMap::new();
        let node =
            |g: &mut DiGraph<LocationId, i64>,
             idx: &mut HashMap<LocationId, NodeIndex>,
             l: LocationId| { *idx.entry(l).or_insert_with(|| g.add_node(l)) };
        for t in &mode_edges {
            let (a, b) = (
                node(&mut graph, &mut idx, t.from),
                node(&mut graph, &mut idx, t.to),
            );
            graph.add_edge(a, b, t.duration_typical_s);
        }
        // Pass 1: real network only.
        let mut pure: HashMap<(LocationId, LocationId), i64> = HashMap::new();
        for from in targets {
            let Some(&src) = idx.get(from) else { continue };
            let costs = petgraph::algo::dijkstra(&graph, src, None, |e| *e.weight());
            for to in targets {
                if from == to {
                    continue;
                }
                if let Some(&dst) = idx.get(to) {
                    if let Some(&cost) = costs.get(&dst) {
                        pure.insert((*from, *to), cost);
                    }
                }
            }
        }
        // Pass 2: add connectors, fill only the still-unreachable pairs.
        for w in within {
            let (c, p) = (
                node(&mut graph, &mut idx, w.child),
                node(&mut graph, &mut idx, w.parent),
            );
            // ADR-0009 decomposition glue, exit/entry ≈ 0 for v1.
            graph.add_edge(c, p, 0);
            graph.add_edge(p, c, 0);
        }
        let mut bridged: HashMap<(LocationId, LocationId), i64> = HashMap::new();
        for from in targets {
            if targets
                .iter()
                .all(|to| to == from || pure.contains_key(&(*from, *to)))
            {
                continue; // every pair already served by the real network
            }
            let Some(&src) = idx.get(from) else { continue };
            let costs = petgraph::algo::dijkstra(&graph, src, None, |e| *e.weight());
            for to in targets {
                if from == to || pure.contains_key(&(*from, *to)) {
                    continue;
                }
                if let Some(&dst) = idx.get(to) {
                    if let Some(&cost) = costs.get(&dst) {
                        bridged.insert((*from, *to), cost);
                    }
                }
            }
        }

        for from in targets {
            for to in targets {
                if from == to {
                    continue;
                }
                let (cost, via_bridge) = match pure.get(&(*from, *to)) {
                    Some(c) => (*c, false),
                    None => match bridged.get(&(*from, *to)) {
                        Some(c) => (*c, true),
                        None => continue,
                    },
                };
                let direct = mode_edges
                    .iter()
                    .filter(|t| t.from == *from && t.to == *to)
                    .min_by_key(|t| t.duration_typical_s);
                let provenance = match direct {
                    Some(d) if !via_bridge && d.duration_typical_s == cost => d.provenance,
                    _ => TravelProvenance::Estimated,
                };
                entries.push(ClosureEntry {
                    from: *from,
                    to: *to,
                    mode: mode.clone(),
                    duration_s: cost,
                    provenance,
                    computed_at,
                });
            }
        }
    }
    entries
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interval::Interval;
    use crate::model::EventId;
    use proptest::prelude::*;

    fn trav(from: LocationId, to: LocationId, s: i64) -> Traverse {
        Traverse {
            from,
            to,
            mode: Mode("walk".into()),
            duration_typical_s: s,
            duration_peak_s: None,
            peak_window: None,
            distance_m: None,
            provenance: TravelProvenance::Measured,
            computed_at: Timestamp(0),
            sibling_override: false,
        }
    }

    /// Independent oracle: Floyd–Warshall over the same edge + connector
    /// set, adjacency-matrix style, no petgraph.
    fn floyd(
        n: usize,
        edges: &[(usize, usize, i64)],
        connectors: &[(usize, usize)],
    ) -> Vec<Vec<Option<i64>>> {
        let mut d = vec![vec![None::<i64>; n]; n];
        for (i, row) in d.iter_mut().enumerate() {
            row[i] = Some(0);
        }
        for (a, b, w) in edges {
            let cur = d[*a][*b];
            if cur.is_none() || cur.unwrap() > *w {
                d[*a][*b] = Some(*w);
            }
        }
        for (a, b) in connectors {
            for (x, y) in [(*a, *b), (*b, *a)] {
                if d[x][y].is_none() || d[x][y].unwrap() > 0 {
                    d[x][y] = Some(0);
                }
            }
        }
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    if let (Some(ik), Some(kj)) = (d[i][k], d[k][j]) {
                        if d[i][j].is_none() || d[i][j].unwrap() > ik + kj {
                            d[i][j] = Some(ik + kj);
                        }
                    }
                }
            }
        }
        d
    }

    proptest! {
        /// H1: closure costs equal Floyd–Warshall on random sparse graphs
        /// with random containment connectors, targets = all nodes.
        #[test]
        fn prop_closure_matches_floyd(
            n in 2usize..8,
            edge_specs in proptest::collection::vec((0usize..8, 0usize..8, 1i64..500), 0..20),
            conn_specs in proptest::collection::vec((0usize..8, 0usize..8), 0..4),
        ) {
            let ids: Vec<LocationId> = (0..n).map(|_| LocationId::new()).collect();
            let edges: Vec<(usize, usize, i64)> = edge_specs
                .into_iter()
                .filter(|(a, b, _)| *a < n && *b < n && a != b)
                .collect();
            let connectors: Vec<(usize, usize)> = conn_specs
                .into_iter()
                .filter(|(a, b)| *a < n && *b < n && a != b)
                .collect();

            let traverse: Vec<Traverse> =
                edges.iter().map(|(a, b, w)| trav(ids[*a], ids[*b], *w)).collect();
            let within: Vec<Within> = connectors
                .iter()
                .map(|(a, b)| Within { child: ids[*a], parent: ids[*b] })
                .collect();

            let closure = compute_closure(&traverse, &within, &ids, Timestamp(0));
            // Oracle mirrors the two-pass semantics: real network is
            // authoritative; connectors only bridge unreachable pairs; no
            // network (no mode) means no closure at all.
            let pure = floyd(n, &edges, &[]);
            let bridged = floyd(n, &edges, &connectors);

            for i in 0..n {
                for j in 0..n {
                    if i == j { continue; }
                    let got = closure
                        .iter()
                        .find(|e| e.from == ids[i] && e.to == ids[j])
                        .map(|e| e.duration_s);
                    let want = if edges.is_empty() {
                        None
                    } else {
                        pure[i][j].or(bridged[i][j])
                    };
                    prop_assert_eq!(got, want, "pair {} -> {}", i, j);
                }
            }
        }
    }

    #[test]
    fn closure_restricted_to_targets_and_idempotent() {
        // room-a -within-> bldg-x, room-b -within-> bldg-y, x <-> y traverse.
        let (ra, rb, bx, by) = (
            LocationId::new(),
            LocationId::new(),
            LocationId::new(),
            LocationId::new(),
        );
        let traverse = vec![trav(bx, by, 300), trav(by, bx, 300)];
        let within = vec![
            Within {
                child: ra,
                parent: bx,
            },
            Within {
                child: rb,
                parent: by,
            },
        ];
        // Only the rooms host events → only room-pair entries appear;
        // buildings are intermediates, never endpoints.
        let c1 = compute_closure(&traverse, &within, &[ra, rb], Timestamp(0));
        assert_eq!(c1.len(), 2, "ra→rb and rb→ra only: {c1:?}");
        assert!(c1.iter().all(|e| e.duration_s == 300));
        assert!(c1
            .iter()
            .all(|e| e.provenance == TravelProvenance::Estimated));
        let c2 = compute_closure(&traverse, &within, &[ra, rb], Timestamp(0));
        assert_eq!(c1, c2, "idempotent on identical inputs");
    }

    /// The H1 refutation, pinned as a regression: a real room→room edge
    /// must not be undercut by the zero-cost containment connectors; the
    /// reverse direction (no real edge) is bridged and marked Estimated.
    #[test]
    fn connectors_do_not_undercut_real_edges() {
        let (ra, rb, bx) = (LocationId::new(), LocationId::new(), LocationId::new());
        let traverse = vec![trav(ra, rb, 120)];
        let within = vec![
            Within {
                child: ra,
                parent: bx,
            },
            Within {
                child: rb,
                parent: bx,
            },
        ];
        let c = compute_closure(&traverse, &within, &[ra, rb], Timestamp(0));
        let ab = c.iter().find(|e| e.from == ra && e.to == rb).unwrap();
        assert_eq!(ab.duration_s, 120, "real edge wins over 0-cost shortcut");
        assert_eq!(ab.provenance, TravelProvenance::Measured);
        let ba = c.iter().find(|e| e.from == rb && e.to == ra).unwrap();
        assert_eq!(
            ba.duration_s, 0,
            "unreachable pair bridged (v1 approximation)"
        );
        assert_eq!(ba.provenance, TravelProvenance::Estimated);
    }

    #[test]
    fn direct_edge_donates_provenance() {
        let (a, b) = (LocationId::new(), LocationId::new());
        let traverse = vec![trav(a, b, 120)];
        let c = compute_closure(&traverse, &[], &[a, b], Timestamp(0));
        assert_eq!(c.len(), 1);
        assert_eq!(c[0].provenance, TravelProvenance::Measured);
    }

    #[test]
    fn feasibility_verdicts() {
        let iv = |s: i64, e: i64| Interval::new(Timestamp(s), Timestamp(e)).unwrap();
        let m = 1_000_000i64;
        let (la, lb) = (LocationId::new(), LocationId::new());
        let a = Placed {
            event: EventId::new(),
            location: la,
            window: iv(0, 600 * m),
        };
        let b = Placed {
            event: EventId::new(),
            location: lb,
            window: iv(900 * m, 1500 * m),
        };

        let lookup = |cost: Option<i64>| {
            move |_: LocationId, _: LocationId| {
                cost.map(|duration_s| TravelCost {
                    duration_s,
                    provenance: TravelProvenance::Measured,
                })
            }
        };
        let p = PersonId::new();
        assert_eq!(
            feasible(p, &a, &b, &lookup(Some(200))),
            Feasibility::Feasible { slack_s: 100 }
        );
        assert_eq!(
            feasible(p, &a, &b, &lookup(Some(400))),
            Feasibility::Infeasible {
                deficit_s: 100,
                provenance: TravelProvenance::Measured
            }
        );
        assert_eq!(feasible(p, &a, &b, &lookup(None)), Feasibility::Unknown);
    }
}
