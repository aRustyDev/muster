//! Tier legality (ADR-0009) — the relational-style enforcement module.
//!
//! On a typed-graph store these rules are schema; everywhere else they are
//! this one module plus exhaustive tests. The command layer calls these
//! before writing `within` / `traverse` edges; illegal edges are rejected
//! with typed errors, never panics.

use crate::model::{Location, Portal, Tier};

fn rank(t: Tier) -> u8 {
    match t {
        Tier::Room => 0,
        Tier::Floor => 1,
        Tier::Structure => 2,
        Tier::Campus => 3,
        Tier::Region => 4,
    }
}

/// `within` is tier-ascending only. Skipping tiers is legal (a
/// single-storey building has no floor); equal or descending is not.
pub fn containment_legal(child: Tier, parent: Tier) -> bool {
    rank(child) < rank(parent)
}

/// Direct `traverse` edges: sibling tier by default; cross-tier is
/// legitimate at portals (parking lot → building lobby, ADR-0010); an
/// explicit `sibling_override` marker is permitted per ADR-0009 (the rule
/// is "a strong default for generated topology, not an invariant" —
/// skybridges exist).
///
/// The full sibling rule also requires a **common parent**; that refinement
/// needs containment lookups at write time and is deferred (slice-2
/// carry-forward in phases/03-engine-core.md).
pub fn traverse_legal(a: &Location, b: &Location, sibling_override: bool) -> bool {
    if sibling_override {
        return true;
    }
    if a.tier == b.tier {
        return true;
    }
    a.portal != Portal::None || b.portal != Portal::None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{LocationId, Tier::*};

    const TIERS: [Tier; 5] = [Room, Floor, Structure, Campus, Region];

    /// Exhaustive 25-pair containment matrix: legal iff strictly ascending.
    #[test]
    fn containment_matrix_exhaustive() {
        for (ci, c) in TIERS.iter().enumerate() {
            for (pi, p) in TIERS.iter().enumerate() {
                assert_eq!(
                    containment_legal(*c, *p),
                    ci < pi,
                    "containment {c:?} -> {p:?}"
                );
            }
        }
    }

    fn loc(tier: Tier, portal: Portal) -> Location {
        Location {
            id: LocationId::new(),
            name: "t".into(),
            tier,
            portal,
            capacity: None,
            ext: Default::default(),
        }
    }

    #[test]
    fn traverse_sibling_portal_and_override() {
        use crate::model::Portal::*;
        let room = loc(Room, None);
        let room2 = loc(Room, None);
        let bldg = loc(Structure, None);
        let lot = loc(Structure, Vehicle);

        assert!(traverse_legal(&room, &room2, false), "same tier");
        assert!(
            !traverse_legal(&room, &bldg, false),
            "cross-tier, no portal"
        );
        assert!(traverse_legal(&lot, &room, false), "cross-tier at a portal");
        assert!(
            traverse_legal(&room, &bldg, true),
            "explicit override marker"
        );
    }
}
