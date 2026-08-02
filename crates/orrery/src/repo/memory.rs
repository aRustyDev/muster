//! `MemoryRepo` — the in-memory repository and the risk control for
//! deferring ADR-0015.
//!
//! It deliberately enforces the **most restrictive constraint intersection
//! across the surviving datastore candidates** (Rule 00b, ADR-0021):
//!
//! * **single writer** — a second concurrent writer gets an error, not a
//!   queue;
//! * **no read during an open write** — reads fail while a write is open
//!   instead of blocking, so code that would deadlock or silently serialise
//!   on a restrictive store fails loudly here;
//! * **no cross-hop traversal predicates** — excluded by construction at the
//!   trait (see `repo` module docs), so there is nothing to enforce at
//!   runtime; this file's traversals only ever apply constant instant
//!   filters.
//!
//! These checks are executable, not documentary: `try_read`/`try_write`
//! failures map to [`OrreryError::ConstraintViolated`] naming the violated
//! constraint.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard, TryLockError};

use crate::command::{Command, CommandReceipt};
use crate::error::{OrreryError, Result};
use crate::interval::{Interval, Timestamp};
use crate::model::{
    AttendanceSource, Attends, ClosureEntry, EntityRef, Event, EventId, Expects, Group, GroupId,
    Held, Location, LocationId, MemberOf, Mode, Person, PersonId, Posture, SubgroupOf, TravelCost,
    Traverse, Violation, ViolationId, Within,
};
use crate::repo::{Repository, MAX_GROUP_DEPTH};
use crate::tier;

pub const SINGLE_WRITER: &str = "single-writer (MemoryRepo: restrictive candidate intersection)";
pub const NO_READ_DURING_WRITE: &str =
    "no-read-during-open-write (MemoryRepo: restrictive candidate intersection)";

#[derive(Default)]
struct State {
    persons: HashMap<PersonId, Person>,
    groups: HashMap<GroupId, Group>,
    events: HashMap<EventId, Event>,
    locations: HashMap<LocationId, Location>,
    attends: Vec<Attends>,
    held: Vec<Held>,
    member_of: Vec<MemberOf>,
    subgroup_of: Vec<SubgroupOf>,
    expects: Vec<Expects>,
    traverse: Vec<Traverse>,
    within: Vec<Within>,
    closure: Vec<ClosureEntry>,
    violations: HashMap<ViolationId, Violation>,
}

#[derive(Default)]
pub struct MemoryRepo {
    state: RwLock<State>,
    seq: AtomicU64,
}

impl MemoryRepo {
    pub fn new() -> Self {
        Self::default()
    }

    fn read(&self) -> Result<RwLockReadGuard<'_, State>> {
        match self.state.try_read() {
            Ok(g) => Ok(g),
            Err(TryLockError::WouldBlock) => Err(OrreryError::constraint(NO_READ_DURING_WRITE)),
            Err(TryLockError::Poisoned(_)) => Err(OrreryError::constraint("lock-poisoned")),
        }
    }

    fn write(&self) -> Result<RwLockWriteGuard<'_, State>> {
        match self.state.try_write() {
            Ok(g) => Ok(g),
            Err(TryLockError::WouldBlock) => Err(OrreryError::constraint(SINGLE_WRITER)),
            Err(TryLockError::Poisoned(_)) => Err(OrreryError::constraint("lock-poisoned")),
        }
    }
}

impl Repository for MemoryRepo {
    fn person(&self, id: PersonId) -> Result<Option<Person>> {
        let _s = tracing::info_span!("repo.person", backend = "memory").entered();
        Ok(self.read()?.persons.get(&id).cloned())
    }

    fn event(&self, id: EventId) -> Result<Option<Event>> {
        let _s = tracing::info_span!("repo.event", backend = "memory").entered();
        Ok(self.read()?.events.get(&id).cloned())
    }

    fn group(&self, id: GroupId) -> Result<Option<Group>> {
        let _s = tracing::info_span!("repo.group", backend = "memory").entered();
        Ok(self.read()?.groups.get(&id).cloned())
    }

    fn location(&self, id: LocationId) -> Result<Option<Location>> {
        let _s = tracing::info_span!("repo.location", backend = "memory").entered();
        Ok(self.read()?.locations.get(&id).cloned())
    }

    fn containment(&self) -> Result<Vec<Within>> {
        let _s = tracing::info_span!("repo.containment", backend = "memory").entered();
        Ok(self.read()?.within.clone())
    }

    fn persons(&self) -> Result<Vec<PersonId>> {
        let _s = tracing::info_span!("repo.persons", backend = "memory").entered();
        let mut v: Vec<PersonId> = self.read()?.persons.keys().copied().collect();
        v.sort();
        Ok(v)
    }

    fn locations(&self) -> Result<Vec<LocationId>> {
        let _s = tracing::info_span!("repo.locations", backend = "memory").entered();
        let mut v: Vec<LocationId> = self.read()?.locations.keys().copied().collect();
        v.sort();
        Ok(v)
    }

    fn events(&self) -> Result<Vec<Event>> {
        let _s = tracing::info_span!("repo.events", backend = "memory").entered();
        let mut v: Vec<Event> = self.read()?.events.values().cloned().collect();
        v.sort_by_key(|e| e.id);
        Ok(v)
    }

    fn events_in(&self, window: Interval) -> Result<Vec<Event>> {
        let _s = tracing::info_span!("repo.events_in", backend = "memory").entered();
        let mut v: Vec<Event> = self
            .read()?
            .events
            .values()
            .filter(|e| e.window.overlaps(&window))
            .cloned()
            .collect();
        v.sort_by_key(|e| (e.window.start(), e.id));
        Ok(v)
    }

    fn attends_for_event(&self, id: EventId) -> Result<Vec<Attends>> {
        let _s = tracing::info_span!("repo.attends_for_event", backend = "memory").entered();
        Ok(self
            .read()?
            .attends
            .iter()
            .filter(|a| a.event == id)
            .cloned()
            .collect())
    }

    fn held_for_event(&self, id: EventId) -> Result<Vec<Held>> {
        let _s = tracing::info_span!("repo.held_for_event", backend = "memory").entered();
        Ok(self
            .read()?
            .held
            .iter()
            .filter(|h| h.event == id)
            .cloned()
            .collect())
    }

    fn open_violations(&self) -> Result<Vec<Violation>> {
        let _s = tracing::info_span!("repo.open_violations", backend = "memory").entered();
        let mut v: Vec<Violation> = self
            .read()?
            .violations
            .values()
            .filter(|v| v.resolved_at.is_none())
            .cloned()
            .collect();
        v.sort_by_key(|v| v.id);
        Ok(v)
    }

    fn memberships_all(&self) -> Result<Vec<MemberOf>> {
        let _s = tracing::info_span!("repo.memberships_all", backend = "memory").entered();
        Ok(self.read()?.member_of.clone())
    }

    fn subgroups_all(&self) -> Result<Vec<SubgroupOf>> {
        let _s = tracing::info_span!("repo.subgroups_all", backend = "memory").entered();
        Ok(self.read()?.subgroup_of.clone())
    }

    fn expectations_all(&self) -> Result<Vec<Expects>> {
        let _s = tracing::info_span!("repo.expectations_all", backend = "memory").entered();
        Ok(self.read()?.expects.clone())
    }

    fn attends_for(&self, id: PersonId, window: Interval) -> Result<Vec<Attends>> {
        let _s = tracing::info_span!("repo.attends_for", backend = "memory").entered();
        Ok(self
            .read()?
            .attends
            .iter()
            .filter(|a| a.person == id && a.during.overlaps(&window))
            .cloned()
            .collect())
    }

    fn held_for(&self, id: LocationId, window: Interval) -> Result<Vec<Held>> {
        let _s = tracing::info_span!("repo.held_for", backend = "memory").entered();
        Ok(self
            .read()?
            .held
            .iter()
            .filter(|h| h.location == id && h.during.overlaps(&window))
            .cloned()
            .collect())
    }

    fn memberships(&self, id: PersonId, at: Timestamp) -> Result<Vec<MemberOf>> {
        let _s = tracing::info_span!("repo.memberships", backend = "memory").entered();
        Ok(self
            .read()?
            .member_of
            .iter()
            .filter(|m| m.person == id && m.during.contains_point(at))
            .cloned()
            .collect())
    }

    fn group_ancestors(&self, id: GroupId, at: Timestamp) -> Result<Vec<GroupId>> {
        let _s = tracing::info_span!("repo.group_ancestors", backend = "memory").entered();
        let state = self.read()?;
        // Bounded BFS upward. The per-hop filter is a CONSTANT instant —
        // never a predicate over prior hops (restrictive intersection).
        let mut frontier = vec![id];
        let mut seen: Vec<GroupId> = Vec::new();
        for _ in 0..MAX_GROUP_DEPTH {
            let mut next = Vec::new();
            for edge in state
                .subgroup_of
                .iter()
                .filter(|e| frontier.contains(&e.child) && e.during.contains_point(at))
            {
                if edge.parent != id && !seen.contains(&edge.parent) {
                    seen.push(edge.parent);
                    next.push(edge.parent);
                }
            }
            if next.is_empty() {
                break;
            }
            frontier = next;
        }
        Ok(seen)
    }

    fn expectations(&self, groups: &[GroupId], at: Timestamp) -> Result<Vec<Expects>> {
        let _s = tracing::info_span!("repo.expectations", backend = "memory").entered();
        Ok(self
            .read()?
            .expects
            .iter()
            .filter(|x| groups.contains(&x.group) && x.during.contains_point(at))
            .cloned()
            .collect())
    }

    fn travel(&self, from: LocationId, to: LocationId, mode: &Mode) -> Result<Option<TravelCost>> {
        let _s = tracing::info_span!("repo.travel", backend = "memory").entered();
        let state = self.read()?;
        if let Some(e) = state
            .closure
            .iter()
            .filter(|e| e.from == from && e.to == to && &e.mode == mode)
            .min_by_key(|e| e.duration_s)
        {
            return Ok(Some(TravelCost {
                duration_s: e.duration_s,
                provenance: e.provenance,
            }));
        }
        Ok(state
            .traverse
            .iter()
            .filter(|t| t.from == from && t.to == to && &t.mode == mode)
            .min_by_key(|t| t.duration_typical_s)
            .map(|t| TravelCost {
                duration_s: t.duration_typical_s,
                provenance: t.provenance,
            }))
    }

    fn travel_best(&self, from: LocationId, to: LocationId) -> Result<Option<TravelCost>> {
        let _s = tracing::info_span!("repo.travel_best", backend = "memory").entered();
        let state = self.read()?;
        let closure_best = state
            .closure
            .iter()
            .filter(|e| e.from == from && e.to == to)
            .min_by_key(|e| e.duration_s)
            .map(|e| TravelCost {
                duration_s: e.duration_s,
                provenance: e.provenance,
            });
        if closure_best.is_some() {
            return Ok(closure_best);
        }
        Ok(state
            .traverse
            .iter()
            .filter(|t| t.from == from && t.to == to)
            .min_by_key(|t| t.duration_typical_s)
            .map(|t| TravelCost {
                duration_s: t.duration_typical_s,
                provenance: t.provenance,
            }))
    }

    fn traverse_all(&self) -> Result<Vec<Traverse>> {
        let _s = tracing::info_span!("repo.traverse_all", backend = "memory").entered();
        Ok(self.read()?.traverse.clone())
    }

    fn apply(&self, cmd: Command) -> Result<CommandReceipt> {
        let _s = tracing::info_span!("command.apply", backend = "memory", variant = cmd.kind())
            .entered();
        let mut state = self.write()?;
        match cmd {
            Command::UpsertPerson(p) => {
                state.persons.insert(p.id, p);
            }
            Command::UpsertGroup(g) => {
                state.groups.insert(g.id, g);
            }
            Command::UpsertEvent(e) => {
                state.events.insert(e.id, e);
            }
            Command::UpsertLocation(l) => {
                state.locations.insert(l.id, l);
            }
            Command::AddAttendance {
                person,
                event,
                priority,
            } => {
                if !state.persons.contains_key(&person) {
                    return Err(OrreryError::NotFound(EntityRef::Person(person)));
                }
                let window = state
                    .events
                    .get(&event)
                    .ok_or(OrreryError::NotFound(EntityRef::Event(event)))?
                    .window;
                state.attends.push(Attends {
                    person,
                    event,
                    during: window,
                    priority_group: 0.0,
                    priority_person: priority,
                    priority_coord: None,
                    coord_binding: false,
                    source: AttendanceSource::SelfSelected,
                    pinned: false,
                });
            }
            Command::RemoveAttendance { person, event } => {
                let before = state.attends.len();
                state
                    .attends
                    .retain(|a| !(a.person == person && a.event == event));
                if state.attends.len() == before {
                    return Err(OrreryError::NotFound(EntityRef::Person(person)));
                }
            }
            Command::SetPriority {
                person,
                event,
                by,
                binding,
                value,
            } => {
                let edge = state
                    .attends
                    .iter_mut()
                    .find(|a| a.person == person && a.event == event)
                    .ok_or(OrreryError::NotFound(EntityRef::Person(person)))?;
                match by {
                    crate::model::Actor::Member(_) => edge.priority_person = Some(value),
                    crate::model::Actor::Coordinator(_) | crate::model::Actor::System => {
                        edge.priority_coord = Some(value);
                        edge.coord_binding = binding;
                    }
                }
            }
            Command::AddMembership {
                person,
                group,
                during,
                role,
            } => {
                state.member_of.push(MemberOf {
                    person,
                    group,
                    during,
                    role,
                });
            }
            Command::AddSubgroup {
                child,
                parent,
                during,
            } => {
                state.subgroup_of.push(SubgroupOf {
                    child,
                    parent,
                    during,
                });
            }
            Command::AddExpectation {
                group,
                event,
                obligation,
                default_priority,
                during,
                cascades,
                by,
            } => {
                state.expects.push(Expects {
                    group,
                    event,
                    obligation,
                    default_priority,
                    during,
                    cascades,
                    can_decline: true,
                    set_by: by,
                    set_at: during.start(),
                });
            }
            Command::HoldLocation {
                location,
                event,
                during,
                overflow_for,
                capacity_override,
            } => {
                if !state.locations.contains_key(&location) {
                    return Err(OrreryError::NotFound(EntityRef::Location(location)));
                }
                if !state.events.contains_key(&event) {
                    return Err(OrreryError::NotFound(EntityRef::Event(event)));
                }
                state.held.push(Held {
                    location,
                    event,
                    during,
                    posture: Posture::OnSite,
                    overflow_for,
                    capacity_override,
                });
            }
            Command::AddContainment { child, parent } => {
                let c = state
                    .locations
                    .get(&child)
                    .ok_or(OrreryError::NotFound(EntityRef::Location(child)))?;
                let p = state
                    .locations
                    .get(&parent)
                    .ok_or(OrreryError::NotFound(EntityRef::Location(parent)))?;
                if !tier::containment_legal(c.tier, p.tier) {
                    return Err(OrreryError::CommandRejected {
                        reason: format!(
                            "within must be tier-ascending (ADR-0009): {:?} -> {:?}",
                            c.tier, p.tier
                        ),
                    });
                }
                state.within.push(Within { child, parent });
            }
            Command::AddTraversePair(t) => {
                let a = state
                    .locations
                    .get(&t.from)
                    .ok_or(OrreryError::NotFound(EntityRef::Location(t.from)))?;
                let b = state
                    .locations
                    .get(&t.to)
                    .ok_or(OrreryError::NotFound(EntityRef::Location(t.to)))?;
                if !tier::traverse_legal(a, b, t.sibling_override) {
                    return Err(OrreryError::CommandRejected {
                        reason: format!(
                            "traverse requires sibling tiers, a portal, or an explicit \
                             override (ADR-0009): {:?} -> {:?}",
                            a.tier, b.tier
                        ),
                    });
                }
                // Both directed rows from one call site (ADR-0008).
                let reverse = Traverse {
                    from: t.to,
                    to: t.from,
                    ..t.clone()
                };
                state.traverse.push(t);
                state.traverse.push(reverse);
            }
            Command::WaiveViolation { id, by, reason } => {
                let v = state
                    .violations
                    .get_mut(&id)
                    .ok_or(OrreryError::NotFound(EntityRef::Violation(id)))?;
                v.waiver_reason = Some(reason);
                v.acknowledged_by = match by {
                    crate::model::Actor::Member(p) | crate::model::Actor::Coordinator(p) => Some(p),
                    crate::model::Actor::System => None,
                };
            }
            Command::RecordViolation(v) => {
                state.violations.insert(v.id, v);
            }
            Command::ResolveViolation { id, at } => {
                let v = state
                    .violations
                    .get_mut(&id)
                    .ok_or(OrreryError::NotFound(EntityRef::Violation(id)))?;
                v.resolved_at = Some(at);
            }
            Command::SetDerivedDigest { person, digest, at } => {
                let p = state
                    .persons
                    .get_mut(&person)
                    .ok_or(OrreryError::NotFound(EntityRef::Person(person)))?;
                p.derived_digest = Some(crate::model::DigestRecord { digest, at });
            }
            Command::ReplaceClosure { entries } => {
                state.closure = entries;
            }
        }
        Ok(CommandReceipt {
            seq: self.seq.fetch_add(1, Ordering::SeqCst) + 1,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Actor, Obligation, Role};

    fn ts(t: i64) -> Timestamp {
        Timestamp(t)
    }
    fn iv(s: i64, e: i64) -> Interval {
        Interval::new(ts(s), ts(e)).unwrap()
    }

    fn person(name: &str) -> Person {
        Person {
            id: PersonId::new(),
            name: name.into(),
            derived_digest: None,
            ext: Default::default(),
        }
    }
    fn group(name: &str) -> Group {
        Group {
            id: GroupId::new(),
            name: name.into(),
            default_priority: None,
            timezone: None,
            ext: Default::default(),
        }
    }
    fn event(name: &str, w: Interval) -> Event {
        Event {
            id: EventId::new(),
            name: name.into(),
            window: w,
            kind: "seminar".into(),
            timezone: None,
            ext: Default::default(),
        }
    }

    /// Rule 00b: a read while a write is open must ERROR, naming the
    /// constraint — never block, never succeed.
    #[test]
    fn read_during_open_write_errors() {
        let repo = MemoryRepo::new();
        let _open_write = repo.state.try_write().unwrap();
        let err = repo.person(PersonId::new()).unwrap_err();
        match err {
            OrreryError::ConstraintViolated { constraint } => {
                assert!(
                    constraint.contains("no-read-during-open-write"),
                    "{constraint}"
                );
            }
            other => panic!("expected ConstraintViolated, got {other:?}"),
        }
    }

    /// Rule 00b: a second concurrent writer must ERROR, naming the
    /// constraint.
    #[test]
    fn second_writer_errors() {
        let repo = MemoryRepo::new();
        let _open_write = repo.state.try_write().unwrap();
        let err = repo.apply(Command::UpsertPerson(person("p"))).unwrap_err();
        match err {
            OrreryError::ConstraintViolated { constraint } => {
                assert!(constraint.contains("single-writer"), "{constraint}");
            }
            other => panic!("expected ConstraintViolated, got {other:?}"),
        }
    }

    /// The Q1-shaped read path over the SPEC-05 critical fixture: a
    /// mid-chain EXPIRED subgroup edge whose subtree must not contribute.
    /// Direct-group (depth-0) expectations MUST contribute — the semantics
    /// Phase 0 showed the original harness got wrong.
    #[test]
    fn q1_read_path_per_hop_expiry() {
        let repo = MemoryRepo::new();
        let t = ts(150);

        let p = person("adam");
        let (g1, g2, g3, g5) = (group("g1"), group("g2"), group("g3"), group("g5"));
        let e_direct = event("direct", iv(0, 10));
        let e_deep = event("deep", iv(0, 10));
        let e_expired = event("expired-subtree", iv(0, 10));

        let pid = p.id;
        let (g1id, g2id, g3id, g5id) = (g1.id, g2.id, g3.id, g5.id);
        let (ed, edeep, eexp) = (e_direct.id, e_deep.id, e_expired.id);

        for cmd in [
            Command::UpsertPerson(p),
            Command::UpsertGroup(g1),
            Command::UpsertGroup(g2),
            Command::UpsertGroup(g3),
            Command::UpsertGroup(g5),
            Command::UpsertEvent(e_direct),
            Command::UpsertEvent(e_deep),
            Command::UpsertEvent(e_expired),
            Command::AddMembership {
                person: pid,
                group: g1id,
                during: iv(0, 999),
                role: Role::Member,
            },
            // chain g1 -> g2 -> g3 valid at t; g2 -> g5 EXPIRED (ends at 50)
            Command::AddSubgroup {
                child: g1id,
                parent: g2id,
                during: iv(0, 999),
            },
            Command::AddSubgroup {
                child: g2id,
                parent: g3id,
                during: iv(0, 999),
            },
            Command::AddSubgroup {
                child: g2id,
                parent: g5id,
                during: iv(0, 50),
            },
        ] {
            repo.apply(cmd).unwrap();
        }
        for (g, e) in [(g1id, ed), (g3id, edeep), (g5id, eexp)] {
            repo.apply(Command::AddExpectation {
                group: g,
                event: e,
                obligation: Obligation::Expected,
                default_priority: 0.5,
                during: iv(0, 999),
                cascades: true,
                by: Actor::System,
            })
            .unwrap();
        }

        let direct: Vec<GroupId> = repo
            .memberships(pid, t)
            .unwrap()
            .into_iter()
            .map(|m| m.group)
            .collect();
        assert_eq!(direct, vec![g1id]);

        let mut reach = direct.clone();
        for g in &direct {
            reach.extend(repo.group_ancestors(*g, t).unwrap());
        }
        assert!(reach.contains(&g2id) && reach.contains(&g3id), "{reach:?}");
        assert!(
            !reach.contains(&g5id),
            "expired mid-chain edge must prune: {reach:?}"
        );

        let events: Vec<EventId> = repo
            .expectations(&reach, t)
            .unwrap()
            .into_iter()
            .map(|x| x.event)
            .collect();
        assert!(events.contains(&ed), "depth-0 expectation must count");
        assert!(events.contains(&edeep));
        assert!(!events.contains(&eexp));
    }

    /// ADR-0005: binding override changes the effective value without
    /// destroying what the person wanted.
    #[test]
    fn priority_stack_preserves_person_input() {
        let repo = MemoryRepo::new();
        let p = person("m");
        let e = event("e", iv(0, 10));
        let (pid, eid) = (p.id, e.id);
        repo.apply(Command::UpsertPerson(p)).unwrap();
        repo.apply(Command::UpsertEvent(e)).unwrap();
        repo.apply(Command::AddAttendance {
            person: pid,
            event: eid,
            priority: Some(0.9),
        })
        .unwrap();
        repo.apply(Command::SetPriority {
            person: pid,
            event: eid,
            by: Actor::Coordinator(PersonId::new()),
            binding: true,
            value: 0.1,
        })
        .unwrap();

        let a = &repo.attends_for(pid, iv(0, 10)).unwrap()[0];
        assert_eq!(a.effective_priority(), 0.1);
        assert_eq!(a.priority_person, Some(0.9), "user input never destroyed");
        let div = a.divergence().unwrap();
        assert!((div - 0.8).abs() < 1e-6, "divergence ~0.8, got {div}");
    }

    /// Phase 6 slice 2 (plan-review CR-6): deselect removes exactly the
    /// (person, event) edge; removing a non-existent edge is an error, not
    /// a silent no-op.
    #[test]
    fn remove_attendance_removes_edge_and_errors_when_absent() {
        let repo = MemoryRepo::new();
        let p = person("m");
        let (e1, e2) = (event("kept", iv(0, 10)), event("dropped", iv(0, 10)));
        let (pid, kept, dropped) = (p.id, e1.id, e2.id);
        repo.apply(Command::UpsertPerson(p)).unwrap();
        repo.apply(Command::UpsertEvent(e1)).unwrap();
        repo.apply(Command::UpsertEvent(e2)).unwrap();
        for eid in [kept, dropped] {
            repo.apply(Command::AddAttendance {
                person: pid,
                event: eid,
                priority: None,
            })
            .unwrap();
        }

        repo.apply(Command::RemoveAttendance {
            person: pid,
            event: dropped,
        })
        .unwrap();
        let left = repo.attends_for(pid, iv(0, 10)).unwrap();
        assert_eq!(left.len(), 1);
        assert_eq!(left[0].event, kept);

        let err = repo
            .apply(Command::RemoveAttendance {
                person: pid,
                event: dropped,
            })
            .unwrap_err();
        assert!(matches!(err, OrreryError::NotFound(_)), "{err:?}");
    }

    fn room(name: &str) -> Location {
        Location {
            id: LocationId::new(),
            name: name.into(),
            tier: crate::model::Tier::Room,
            portal: crate::model::Portal::None,
            capacity: Some(40),
            ext: Default::default(),
        }
    }

    /// ADR-0008: one command writes both directed rows.
    #[test]
    fn traverse_pair_writes_two_rows() {
        let repo = MemoryRepo::new();
        let (ra, rb) = (room("a"), room("b"));
        let (a, b) = (ra.id, rb.id);
        repo.apply(Command::UpsertLocation(ra)).unwrap();
        repo.apply(Command::UpsertLocation(rb)).unwrap();
        repo.apply(Command::AddTraversePair(Traverse {
            from: a,
            to: b,
            mode: Mode("walk".into()),
            duration_typical_s: 120,
            duration_peak_s: None,
            peak_window: None,
            distance_m: None,
            provenance: crate::model::TravelProvenance::Estimated,
            computed_at: ts(0),
            sibling_override: false,
        }))
        .unwrap();
        let walk = Mode("walk".into());
        assert_eq!(repo.travel(a, b, &walk).unwrap().unwrap().duration_s, 120);
        assert_eq!(repo.travel(b, a, &walk).unwrap().unwrap().duration_s, 120);
    }

    /// ADR-0009 at the command layer: inverted containment and cross-tier
    /// traverse without portal/override are rejected with typed errors.
    #[test]
    fn tier_rules_enforced_at_write() {
        let repo = MemoryRepo::new();
        let r = room("r");
        let bldg = Location {
            id: LocationId::new(),
            name: "bldg".into(),
            tier: crate::model::Tier::Structure,
            portal: crate::model::Portal::None,
            capacity: None,
            ext: Default::default(),
        };
        let (rid, bid) = (r.id, bldg.id);
        repo.apply(Command::UpsertLocation(r)).unwrap();
        repo.apply(Command::UpsertLocation(bldg)).unwrap();

        // legal: room within structure
        repo.apply(Command::AddContainment {
            child: rid,
            parent: bid,
        })
        .unwrap();
        // inverted: rejected
        assert!(matches!(
            repo.apply(Command::AddContainment {
                child: bid,
                parent: rid
            }),
            Err(OrreryError::CommandRejected { .. })
        ));
        // cross-tier traverse without portal or override: rejected
        let bad = Traverse {
            from: rid,
            to: bid,
            mode: Mode("walk".into()),
            duration_typical_s: 60,
            duration_peak_s: None,
            peak_window: None,
            distance_m: None,
            provenance: crate::model::TravelProvenance::Estimated,
            computed_at: ts(0),
            sibling_override: false,
        };
        assert!(matches!(
            repo.apply(Command::AddTraversePair(bad.clone())),
            Err(OrreryError::CommandRejected { .. })
        ));
        // same edge with the explicit override marker: accepted
        repo.apply(Command::AddTraversePair(Traverse {
            sibling_override: true,
            ..bad
        }))
        .unwrap();
    }
}
