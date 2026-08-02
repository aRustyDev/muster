//! Salsa-backed incremental derivation (ADR-0016 C).
//!
//! The reframe: blast radius is an incremental-computation problem, not a
//! database problem. Base facts the derived chain reads (memberships,
//! subgroup edges, expectation keys) are mirrored into one [`World`] input;
//! per-person tracked extraction feeds reach → derived-ids → digest.
//! **Early cutoff at the extraction layer is the mechanism that bounds
//! blast radius**: a write for person A bumps the `World` revision, so B's
//! cheap extraction re-runs — but it returns an equal value, salsa
//! backdates it, and B's expansion and digest never re-execute.
//!
//! The digest chain is float-free by construction: it flows id sets, never
//! priorities (ADR-0016 B — the digest hashes the sorted derived-edge ids).
//!
//! `probe` holds execution counters — the one piece of global state in this
//! crate, existing solely so tests can assert early cutoff actually
//! happened rather than trusting that it did.

use salsa::Setter;

use crate::derive::{derived_id, digest_of_ids, DerivedId};
use crate::error::Result;
use crate::interval::Timestamp;
use crate::model::{EventId, GroupId, MemberOf, PersonId, SubgroupOf};
use crate::repo::{Repository, MAX_GROUP_DEPTH};

/// Float-free projection of an `Expects` edge — everything the derived-id
/// chain needs, nothing the digest must not depend on.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExpectKey {
    pub group: GroupId,
    pub event: EventId,
    pub valid_from: Timestamp,
    pub valid_to_excl: Timestamp,
    pub cascades: bool,
    /// `default_priority` as an order-preserving bit pattern (see
    /// [`priority_key`]) so the winner-per-event rule matches
    /// `derive::expand` exactly without a float entering the Eq/Hash chain.
    pub priority_key: u32,
}

/// Order-preserving f32 → u32 encoding (finite floats; NaN never enters —
/// priorities come from validated commands): flips the sign bit for
/// positives and all bits for negatives, so integer order == float order.
pub fn priority_key(p: f32) -> u32 {
    let bits = p.to_bits();
    if bits & 0x8000_0000 != 0 {
        !bits
    } else {
        bits | 0x8000_0000
    }
}

#[salsa::input]
pub struct World {
    pub memberships: Vec<MemberOf>,
    pub subgroups: Vec<SubgroupOf>,
    pub expect_keys: Vec<ExpectKey>,
}

/// Execution counters for early-cutoff assertions (relaxed atomics; tests
/// are single-threaded over their own database).
pub mod probe {
    use std::sync::atomic::{AtomicU64, Ordering};

    pub static DIRECT_GROUPS: AtomicU64 = AtomicU64::new(0);
    pub static REACH: AtomicU64 = AtomicU64::new(0);
    pub static DERIVED_IDS: AtomicU64 = AtomicU64::new(0);
    pub static DIGEST: AtomicU64 = AtomicU64::new(0);

    pub fn snapshot() -> [u64; 4] {
        [
            DIRECT_GROUPS.load(Ordering::Relaxed),
            REACH.load(Ordering::Relaxed),
            DERIVED_IDS.load(Ordering::Relaxed),
            DIGEST.load(Ordering::Relaxed),
        ]
    }

    pub(super) fn hit(c: &AtomicU64) {
        c.fetch_add(1, Ordering::Relaxed);
    }
}

/// Direct groups of `person` valid at `at`, sorted. Cheap extraction layer:
/// re-executes on every `World` change, exists to backdate.
#[salsa::tracked]
pub fn direct_groups(
    db: &dyn salsa::Database,
    world: World,
    person: PersonId,
    at: i64,
) -> Vec<GroupId> {
    probe::hit(&probe::DIRECT_GROUPS);
    let at = Timestamp(at);
    let mut out: Vec<GroupId> = world
        .memberships(db)
        .iter()
        .filter(|m| m.person == person && m.during.contains_point(at))
        .map(|m| m.group)
        .collect();
    out.sort();
    out.dedup();
    out
}

/// Reachable groups with cascade-only marking, sorted. Same per-hop
/// constant-instant semantics as `Repository::group_ancestors` plus the
/// depth-0 direct groups.
#[salsa::tracked]
pub fn reach(
    db: &dyn salsa::Database,
    world: World,
    person: PersonId,
    at: i64,
) -> Vec<(GroupId, bool)> {
    probe::hit(&probe::REACH);
    let direct = direct_groups(db, world, person, at);
    let at_t = Timestamp(at);
    let subgroups = world.subgroups(db);

    let mut out: Vec<(GroupId, bool)> = direct.iter().map(|g| (*g, false)).collect();
    let mut frontier = direct.clone();
    for _ in 0..MAX_GROUP_DEPTH {
        let mut next = Vec::new();
        for e in subgroups
            .iter()
            .filter(|e| frontier.contains(&e.child) && e.during.contains_point(at_t))
        {
            if !out.iter().any(|(g, _)| *g == e.parent) {
                out.push((e.parent, !direct.contains(&e.parent)));
                next.push(e.parent);
            }
        }
        if next.is_empty() {
            break;
        }
        frontier = next;
    }
    out.sort();
    out
}

/// The person's derived-edge id set at `at`, sorted (deduplicated per
/// event with the same max-priority rule as `derive::expand` — but ids do
/// not depend on priority, so the id for an event is the winning
/// expectation's id; ties broken by group id as in `expand`).
#[salsa::tracked]
pub fn derived_ids(
    db: &dyn salsa::Database,
    world: World,
    person: PersonId,
    at: i64,
) -> Vec<DerivedId> {
    probe::hit(&probe::DERIVED_IDS);
    let groups = reach(db, world, person, at);
    let at_t = Timestamp(at);

    // Winner-per-event replicates derive::expand exactly: higher priority
    // wins; equal priority breaks ties toward the smaller group id; first
    // occurrence wins outright ties. The priority travels as an
    // order-preserving bit pattern so no float enters the memoized chain.
    let expect_keys = world.expect_keys(db);
    let mut best: Vec<(EventId, u32, GroupId, Timestamp)> = Vec::new();
    for x in expect_keys.iter() {
        let Some((_, cascade_only)) = groups.iter().find(|(g, _)| *g == x.group) else {
            continue;
        };
        if *cascade_only && !x.cascades {
            continue;
        }
        if !(x.valid_from <= at_t && at_t < x.valid_to_excl) {
            continue;
        }
        match best.iter_mut().find(|(e, ..)| *e == x.event) {
            None => best.push((x.event, x.priority_key, x.group, x.valid_from)),
            Some(entry) => {
                let better = x.priority_key > entry.1
                    || (x.priority_key == entry.1 && x.group.0 < entry.2 .0);
                if better {
                    entry.1 = x.priority_key;
                    entry.2 = x.group;
                    entry.3 = x.valid_from;
                }
            }
        }
    }

    let mut ids: Vec<DerivedId> = best
        .into_iter()
        .map(|(event, _, group, start)| derived_id(person, event, group, start))
        .collect();
    ids.sort();
    ids
}

/// Digest of the person's derived-edge id set (ADR-0016 B).
#[salsa::tracked]
pub fn digest(db: &dyn salsa::Database, world: World, person: PersonId, at: i64) -> [u8; 32] {
    probe::hit(&probe::DIGEST);
    digest_of_ids(derived_ids(db, world, person, at))
}

/// Build the three mirrored fact vectors from a repository snapshot.
pub fn mirror_from(
    repo: &dyn Repository,
) -> Result<(Vec<MemberOf>, Vec<SubgroupOf>, Vec<ExpectKey>)> {
    let memberships = repo.memberships_all()?;
    let subgroups = repo.subgroups_all()?;
    let expect_keys = repo
        .expectations_all()?
        .into_iter()
        .map(|x| ExpectKey {
            group: x.group,
            event: x.event,
            valid_from: x.during.start(),
            valid_to_excl: x.during.end(),
            cascades: x.cascades,
            priority_key: priority_key(x.default_priority),
        })
        .collect();
    Ok((memberships, subgroups, expect_keys))
}

/// Refresh the mirrored field(s) a command class touches. Only the three
/// fact classes the derived chain reads trigger a revision bump.
pub fn refresh_after(
    db: &mut salsa::DatabaseImpl,
    world: World,
    repo: &dyn Repository,
    command_kind: &str,
) -> Result<()> {
    match command_kind {
        "add_membership" => {
            let v = repo.memberships_all()?;
            world.set_memberships(db).to(v);
        }
        "add_subgroup" => {
            let v = repo.subgroups_all()?;
            world.set_subgroups(db).to(v);
        }
        "add_expectation" => {
            let (_, _, keys) = mirror_from(repo)?;
            world.set_expect_keys(db).to(keys);
        }
        _ => {}
    }
    Ok(())
}
