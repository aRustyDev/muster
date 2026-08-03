//! The engine: command routing (with the `Prevent` call site), the salsa
//! mirror, digest refresh, batch sweeps, and the `FeasibilityOracle`.
//!
//! The engine reads no clock and mints no time: every evaluation instant is
//! caller-supplied, which keeps everything here deterministic and
//! replayable.

use crate::command::{Command, CommandReceipt};
use crate::detect::{self, location_exclusivity, time_conflict, Policy, PolicyMap, ViolationDraft};
use crate::error::{OrreryError, Result};
use crate::interval::{Interval, Timestamp};
use crate::model::LocationId;
use crate::model::{
    AttendanceSource, Attends, EntityRef, Held, MemberOf, PersonId, Severity, SubgroupOf,
    Violation, ViolationId, ViolationKind,
};
use crate::repo::Repository;
use crate::travel::{self, ClosureReport, ClosureScope, Feasibility};
use crate::{derive, incremental};

use crate::detect::impossible_travel::{self, Placed};

pub struct Engine<R: Repository> {
    repo: R,
    policies: PolicyMap,
    db: salsa::DatabaseImpl,
    world: incremental::World,
}

impl<R: Repository> Engine<R> {
    pub fn new(repo: R) -> Result<Self> {
        Self::with_policies(repo, PolicyMap::default())
    }

    pub fn with_policies(repo: R, policies: PolicyMap) -> Result<Self> {
        let db = salsa::DatabaseImpl::new();
        let (m, s, x) = incremental::mirror_from(&repo)?;
        let world = incremental::World::new(&db, m, s, x);
        Ok(Engine {
            repo,
            policies,
            db,
            world,
        })
    }

    pub fn repo(&self) -> &R {
        &self.repo
    }

    pub fn policies_mut(&mut self) -> &mut PolicyMap {
        &mut self.policies
    }

    /// The single mutation path. Runs the `Prevent` call site — the same
    /// detector functions the sweep uses, called against the prospective
    /// post-write state — then applies through the repository and refreshes
    /// the salsa mirror. MemoryRepo's transaction is the apply call itself;
    /// its single-writer enforcement makes check-then-apply race-free
    /// within one engine.
    pub fn apply(&mut self, cmd: Command) -> Result<CommandReceipt> {
        self.prevent_gate(&cmd)?;
        let receipt = self.repo.apply(cmd.clone())?;
        incremental::refresh_after(&mut self.db, self.world, &self.repo, cmd.kind())?;
        Ok(receipt)
    }

    /// `Prevent` = the same detector, second call site (ADR-0012).
    fn prevent_gate(&self, cmd: &Command) -> Result<()> {
        match cmd {
            Command::AddAttendance { person, event, .. }
                if self.policies.policy(ViolationKind::TimeConflict) == Policy::Prevent =>
            {
                let window = self
                    .repo
                    .event(*event)?
                    .ok_or(OrreryError::NotFound(EntityRef::Event(*event)))?
                    .window;
                let mut attends = self.repo.attends_for(*person, window)?;
                attends.push(Attends {
                    person: *person,
                    event: *event,
                    during: window,
                    priority_group: 0.0,
                    priority_person: None,
                    priority_coord: None,
                    coord_binding: false,
                    source: AttendanceSource::SelfSelected,
                    pinned: false,
                });
                let drafts = time_conflict::detect(*person, &attends);
                if !drafts.is_empty() {
                    return Err(OrreryError::CommandRejected {
                        reason: format!(
                            "prevented: {} time conflict(s) would result (policy Prevent)",
                            drafts.len()
                        ),
                    });
                }
            }
            Command::HoldLocation {
                location, during, ..
            } if self.policies.policy(ViolationKind::LocationExclusivity) == Policy::Prevent => {
                let mut holds = self.repo.held_for(*location, *during)?;
                if let Command::HoldLocation {
                    location,
                    event,
                    during,
                    overflow_for,
                    capacity_override,
                } = cmd
                {
                    holds.push(Held {
                        location: *location,
                        event: *event,
                        during: *during,
                        posture: crate::model::Posture::OnSite,
                        overflow_for: *overflow_for,
                        capacity_override: *capacity_override,
                    });
                }
                let drafts = location_exclusivity::detect(*location, &holds);
                if !drafts.is_empty() {
                    return Err(OrreryError::CommandRejected {
                        reason: format!(
                            "prevented: {} location-exclusivity violation(s) would result \
                             (policy Prevent)",
                            drafts.len()
                        ),
                    });
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// Recompute the Layer-2 closure from Layer-1 (ADR-0006) and replace it
    /// atomically through the command layer. `at` stamps `computed_at` —
    /// the engine reads no clock.
    pub fn refresh_closure(&mut self, scope: ClosureScope, at: Timestamp) -> Result<ClosureReport> {
        let traverse = self.repo.traverse_all()?;
        let within = self.repo.containment()?;
        let all = Interval::new(Timestamp(i64::MIN), Timestamp(i64::MAX))
            .expect("MIN < MAX is a const invariant");
        let targets: Vec<LocationId> = match scope {
            ClosureScope::AllLocations => self.repo.locations()?,
            ClosureScope::EventBearing => {
                let mut out = Vec::new();
                for lid in self.repo.locations()? {
                    if !self.repo.held_for(lid, all)?.is_empty() {
                        out.push(lid);
                    }
                }
                out
            }
        };
        let entries = travel::compute_closure(&traverse, &within, &targets, at);
        let report = ClosureReport {
            sources: targets.len(),
            pairs: entries.len(),
        };
        let _span = tracing::info_span!(
            "travel.closure_refresh",
            scope = ?scope,
            pairs = report.pairs,
            sources = report.sources
        )
        .entered();
        self.repo.apply(Command::ReplaceClosure { entries })?;
        Ok(report)
    }

    /// Join a person's attendance (plus optional overlay edges) to placed
    /// events: window = the attends edge's own window; location = the
    /// event's primary hold (first non-overflow, ties to the smaller
    /// location id); unheld events are skipped (orphan_event's business).
    fn placed_for(
        &self,
        person: PersonId,
        window: Interval,
        overlay: &[Attends],
    ) -> Result<Vec<Placed>> {
        let mut attends = self.repo.attends_for(person, window)?;
        attends.extend(overlay.iter().filter(|a| a.person == person).cloned());
        let mut out = Vec::new();
        for a in attends {
            let mut holds = self.repo.held_for_event(a.event)?;
            holds.sort_by_key(|h| (h.overflow_for.is_some(), h.location));
            if let Some(h) = holds.first() {
                out.push(Placed {
                    event: a.event,
                    location: h.location,
                    window: a.during,
                });
            }
        }
        Ok(out)
    }

    /// ADR-0014's core feature (Phase 6a): can `person` reach their first
    /// placed event in `window` from one of their anchors, departing no
    /// earlier than `depart_not_before`? Returns a verdict only —
    /// durations and provenance, never an anchor or a location
    /// (Rule 00.6/09; asserted by the `privacy_` tests).
    ///
    /// Semantics as pre-committed in phases/06a-engine-surfaces.md: the
    /// first event is the earliest placed event in `window` starting at or
    /// after `depart_not_before`; applicable anchors are those valid at
    /// `depart_not_before` (`applies_when` is stored but unread until
    /// ADR-0017); the verdict is the best across applicable anchors —
    /// `Feasible` if any anchor makes it (the don't-accuse rule), else the
    /// least-bad known route's `Infeasible` (this is a query, not a
    /// violation: the caller decides whether to accuse), else `Unknown`
    /// (no anchors, no placed event, or no known route — never an
    /// accusation). Deliberately NOT wired into `sweep()`: a sweep-side
    /// anchor violation needs a `depart_not_before` policy source the
    /// engine doesn't have (mobility profiles, ADR-0017, or app-supplied
    /// day boundaries — phase-doc carry-forward).
    pub fn first_event_feasibility(
        &self,
        person: PersonId,
        window: Interval,
        depart_not_before: Timestamp,
    ) -> Result<Feasibility> {
        let placed = self.placed_for(person, window, &[])?;
        let Some(first) = placed
            .iter()
            .filter(|p| p.window.start() >= depart_not_before)
            .min_by_key(|p| (p.window.start(), p.event))
        else {
            return Ok(Feasibility::Unknown);
        };
        let gap_s = (first.window.start().micros() - depart_not_before.micros()) / 1_000_000;

        let mut best = Feasibility::Unknown;
        for anchor in self.repo.anchors_for(person, depart_not_before)? {
            let verdict = if anchor.structure == first.location {
                Feasibility::Feasible { slack_s: gap_s }
            } else {
                match self.repo.travel_best(anchor.structure, first.location)? {
                    None => Feasibility::Unknown,
                    Some(cost) if gap_s >= cost.duration_s => Feasibility::Feasible {
                        slack_s: gap_s - cost.duration_s,
                    },
                    Some(cost) => Feasibility::Infeasible {
                        deficit_s: cost.duration_s - gap_s,
                        provenance: cost.provenance,
                    },
                }
            };
            best = match (best, verdict) {
                (Feasibility::Feasible { slack_s: a }, Feasibility::Feasible { slack_s: b }) => {
                    Feasibility::Feasible { slack_s: a.max(b) }
                }
                (f @ Feasibility::Feasible { .. }, _) | (_, f @ Feasibility::Feasible { .. }) => f,
                (
                    i @ Feasibility::Infeasible { deficit_s: a, .. },
                    j @ Feasibility::Infeasible { deficit_s: b, .. },
                ) => {
                    if b < a {
                        j
                    } else {
                        i
                    }
                }
                (i @ Feasibility::Infeasible { .. }, Feasibility::Unknown) => i,
                (Feasibility::Unknown, v) => v,
            };
        }
        Ok(best)
    }

    /// Incremental digest for one person at `at` (salsa-memoized).
    pub fn digest(&mut self, person: PersonId, at: Timestamp) -> [u8; 32] {
        *incremental::digest(&self.db, self.world, person, at.micros())
    }

    /// Recompute digests for every person; persist changes on the person
    /// record (ADR-0016 B) and return exactly the changed set — the
    /// change-notification primitive muster-sdk consumes.
    pub fn refresh_digests(&mut self, at: Timestamp) -> Result<Vec<PersonId>> {
        let _span = tracing::info_span!("derive.refresh_digests").entered();
        let mut changed = Vec::new();
        for pid in self.repo.persons()? {
            let fresh = *incremental::digest(&self.db, self.world, pid, at.micros());
            let stored = self
                .repo
                .person(pid)?
                .and_then(|p| p.derived_digest.map(|d| d.digest));
            if stored != Some(fresh) {
                self.repo.apply(Command::SetDerivedDigest {
                    person: pid,
                    digest: fresh,
                    at,
                })?;
                changed.push(pid);
            }
        }
        Ok(changed)
    }

    /// Non-persisting digest dry-run (Phase 6a): the change set that
    /// `apply(cmd)` followed by `refresh_digests(at)` WOULD produce,
    /// without writing anything — the blast-radius preview primitive
    /// (muster/SPEC-03 honesty gate: preview must equal the post-commit
    /// change set, property-tested in `tests/preview.rs`).
    ///
    /// Supported for exactly the three mirrored fact classes; any other
    /// kind gets [`OrreryError::PreviewUnsupported`]. The overlay is built
    /// from repository reads plus the command's would-be fact — the same
    /// mapping `incremental::refresh_after` applies post-commit.
    ///
    /// Mirror discipline (plan-review CR-1): every fallible read completes
    /// before the salsa input is overlaid, and no fallible operation exists
    /// between overlay and restore — no path leaves the mirror corrupted.
    /// The digest chain itself cannot fail or panic (pure computation over
    /// the mirrored vectors). Restoring bumps the revision again; salsa
    /// backdates unaffected persons at the extraction layer both times, so
    /// preview cost stays bounded by the blast radius, not the population.
    pub fn preview_digests(&mut self, cmd: &Command, at: Timestamp) -> Result<Vec<PersonId>> {
        enum Overlay {
            Memberships(Vec<MemberOf>),
            Subgroups(Vec<SubgroupOf>),
            ExpectKeys(Vec<incremental::ExpectKey>),
        }
        let overlay = match cmd {
            Command::AddMembership {
                person,
                group,
                during,
                role,
            } => {
                let mut v = self.repo.memberships_all()?;
                v.push(MemberOf {
                    person: *person,
                    group: *group,
                    during: *during,
                    role: role.clone(),
                });
                Overlay::Memberships(v)
            }
            Command::AddSubgroup {
                child,
                parent,
                during,
            } => {
                let mut v = self.repo.subgroups_all()?;
                v.push(SubgroupOf {
                    child: *child,
                    parent: *parent,
                    during: *during,
                });
                Overlay::Subgroups(v)
            }
            Command::AddExpectation {
                group,
                event,
                default_priority,
                during,
                cascades,
                ..
            } => {
                let (_, _, mut keys) = incremental::mirror_from(&self.repo)?;
                keys.push(incremental::ExpectKey {
                    group: *group,
                    event: *event,
                    valid_from: during.start(),
                    valid_to_excl: during.end(),
                    cascades: *cascades,
                    priority_key: incremental::priority_key(*default_priority),
                });
                Overlay::ExpectKeys(keys)
            }
            other => {
                return Err(OrreryError::PreviewUnsupported { kind: other.kind() });
            }
        };

        // Remaining fallible reads, completed before the mirror is touched:
        // the person set and their stored digest records — the same
        // comparison base `refresh_digests` uses.
        let mut stored: Vec<(PersonId, Option<[u8; 32]>)> = Vec::new();
        for pid in self.repo.persons()? {
            stored.push((
                pid,
                self.repo
                    .person(pid)?
                    .and_then(|p| p.derived_digest.map(|d| d.digest)),
            ));
        }

        let _span = tracing::info_span!("derive.preview_digests", kind = cmd.kind()).entered();

        // Overlay → compute → restore. Infallible section: no `?` between
        // the set and the restore (CR-1).
        use salsa::Setter;
        let changed = match overlay {
            Overlay::Memberships(v) => {
                let saved = self.world.set_memberships(&mut self.db).to(v);
                let changed = Self::digest_changes(&self.db, self.world, &stored, at);
                self.world.set_memberships(&mut self.db).to(saved);
                changed
            }
            Overlay::Subgroups(v) => {
                let saved = self.world.set_subgroups(&mut self.db).to(v);
                let changed = Self::digest_changes(&self.db, self.world, &stored, at);
                self.world.set_subgroups(&mut self.db).to(saved);
                changed
            }
            Overlay::ExpectKeys(v) => {
                let saved = self.world.set_expect_keys(&mut self.db).to(v);
                let changed = Self::digest_changes(&self.db, self.world, &stored, at);
                self.world.set_expect_keys(&mut self.db).to(saved);
                changed
            }
        };
        Ok(changed)
    }

    /// Persons whose fresh digest differs from their stored record —
    /// `refresh_digests`'s comparison, minus the persistence.
    fn digest_changes(
        db: &salsa::DatabaseImpl,
        world: incremental::World,
        stored: &[(PersonId, Option<[u8; 32]>)],
        at: Timestamp,
    ) -> Vec<PersonId> {
        stored
            .iter()
            .filter_map(|(pid, s)| {
                let fresh = *incremental::digest(db, world, *pid, at.micros());
                (*s != Some(fresh)).then_some(*pid)
            })
            .collect()
    }

    /// Batch sweep (PRD Flow B): run the repo-data detectors over the whole
    /// population, upsert violation records through the command layer, and
    /// resolve open violations whose cause has disappeared. Waived
    /// violations are never auto-resolved or duplicated. Travel and
    /// expired-membership detectors join once Phase 4 lands travel and the
    /// derived cache exists.
    pub fn sweep(&mut self, at: Timestamp, window: Interval) -> Result<SweepReport> {
        let _span = tracing::info_span!("detect.sweep").entered();
        let mut drafts: Vec<ViolationDraft> = Vec::new();

        for pid in self.repo.persons()? {
            let attends = self.repo.attends_for(pid, window)?;
            drafts.extend(time_conflict::detect(pid, &attends));
            let placed = self.placed_for(pid, window, &[])?;
            let repo = &self.repo;
            drafts.extend(impossible_travel::detect(pid, &placed, &|f, t| {
                repo.travel_best(f, t).ok().flatten()
            }));
        }
        let containment = self.repo.containment()?;
        let mut all_holds = Vec::new();
        for lid in self.repo.locations()? {
            let holds = self.repo.held_for(lid, window)?;
            drafts.extend(location_exclusivity::detect(lid, &holds));
            all_holds.extend(holds);
        }
        drafts.extend(detect::containment_exclusivity::detect(
            &all_holds,
            &containment,
        ));
        for event in self.repo.events()? {
            if !event.window.overlaps(&window) {
                continue;
            }
            let holds: Vec<Held> = all_holds
                .iter()
                .filter(|h| h.event == event.id)
                .cloned()
                .collect();
            drafts.extend(detect::orphan_event::detect(&event, &holds));

            let attends = self.repo.attends_for_event(event.id)?;
            let signalled = detect::capacity_exceeded::signalled_interest(event.id, &attends, 0.0);
            let repo = &self.repo;
            drafts.extend(detect::capacity_exceeded::detect(
                event.id,
                &holds,
                signalled,
                &|lid| repo.location(lid).ok().flatten().and_then(|l| l.capacity),
            ));
        }

        // Dedup within the sweep, then reconcile against open records.
        drafts.sort_by(|a, b| (a.kind, &a.subjects).cmp(&(b.kind, &b.subjects)));
        drafts.dedup();

        let open = self.repo.open_violations()?;
        let swept_kinds = [
            ViolationKind::TimeConflict,
            ViolationKind::LocationExclusivity,
            ViolationKind::ContainmentExclusivity,
            ViolationKind::ImpossibleTravel,
            ViolationKind::OrphanEvent,
            ViolationKind::CapacityExceeded,
        ];

        let mut emitted = 0;
        for d in &drafts {
            let already = open
                .iter()
                .any(|v| v.kind == d.kind && v.subjects == d.subjects);
            if !already {
                self.repo.apply(Command::RecordViolation(Violation {
                    id: ViolationId::new(),
                    kind: d.kind,
                    severity: d.severity,
                    subjects: d.subjects.clone(),
                    detected_at: at,
                    resolved_at: None,
                    acknowledged_by: None,
                    waiver_reason: None,
                }))?;
                emitted += 1;
            }
        }

        let mut resolved = 0;
        for v in &open {
            if !swept_kinds.contains(&v.kind) || v.waiver_reason.is_some() {
                continue;
            }
            let still = drafts
                .iter()
                .any(|d| d.kind == v.kind && d.subjects == v.subjects);
            if !still {
                self.repo
                    .apply(Command::ResolveViolation { id: v.id, at })?;
                resolved += 1;
            }
        }

        Ok(SweepReport { emitted, resolved })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SweepReport {
    pub emitted: usize,
    pub resolved: usize,
}

/// A proposed assignment overlay: evaluated against current state without
/// writing anything. `at` is the evaluation instant (the engine reads no
/// clock).
#[derive(Debug, Clone, PartialEq)]
pub struct Assignment {
    pub attends: Vec<Attends>,
    pub held: Vec<Held>,
    pub at: Timestamp,
    pub window: Interval,
}

/// SPEC-00 contract. Slice-2 scope: time-conflict, location- and
/// containment-exclusivity over the overlay; travel feasibility joins in
/// Phase 4 (recorded in phases/03-engine-core.md).
pub trait FeasibilityOracle {
    fn is_feasible(&self, a: &Assignment) -> Vec<Violation>;
    fn score(&self, a: &Assignment) -> f64;
}

/// Severity weights for `score` (documented in phases/03-engine-core.md;
/// muster-sdk composes richer objectives on top — ADR-0013).
fn severity_weight(s: Severity) -> f64 {
    match s {
        Severity::Hard => 100.0,
        Severity::Warning => 10.0,
        Severity::Info => 1.0,
    }
}

impl<R: Repository> FeasibilityOracle for Engine<R> {
    fn is_feasible(&self, a: &Assignment) -> Vec<Violation> {
        let mut drafts: Vec<ViolationDraft> = Vec::new();

        // Persons touched by the proposal: overlay = stored ∪ proposed.
        let mut persons: Vec<PersonId> = a.attends.iter().map(|x| x.person).collect();
        persons.sort();
        persons.dedup();
        for pid in persons {
            let mut attends = self.repo.attends_for(pid, a.window).unwrap_or_default();
            attends.extend(a.attends.iter().filter(|x| x.person == pid).cloned());
            drafts.extend(time_conflict::detect(pid, &attends));
            if let Ok(placed) = self.placed_for(pid, a.window, &a.attends) {
                let repo = &self.repo;
                drafts.extend(impossible_travel::detect(pid, &placed, &|f, t| {
                    repo.travel_best(f, t).ok().flatten()
                }));
            }
        }

        // Locations touched by the proposal.
        let mut locations: Vec<_> = a.held.iter().map(|h| h.location).collect();
        locations.sort();
        locations.dedup();
        let mut overlay_holds: Vec<Held> = Vec::new();
        for lid in &locations {
            let mut holds = self.repo.held_for(*lid, a.window).unwrap_or_default();
            holds.extend(a.held.iter().filter(|h| h.location == *lid).cloned());
            drafts.extend(location_exclusivity::detect(*lid, &holds));
            overlay_holds.extend(holds);
        }
        if !overlay_holds.is_empty() {
            let containment = self.repo.containment().unwrap_or_default();
            drafts.extend(detect::containment_exclusivity::detect(
                &overlay_holds,
                &containment,
            ));
        }

        drafts.sort_by(|x, y| (x.kind, &x.subjects).cmp(&(y.kind, &y.subjects)));
        drafts.dedup();
        drafts
            .into_iter()
            .map(|d| Violation {
                id: ViolationId::new(),
                kind: d.kind,
                severity: d.severity,
                subjects: d.subjects,
                detected_at: a.at,
                resolved_at: None,
                acknowledged_by: None,
                waiver_reason: None,
            })
            .collect()
    }

    fn score(&self, a: &Assignment) -> f64 {
        -self
            .is_feasible(a)
            .iter()
            .map(|v| severity_weight(v.severity))
            .sum::<f64>()
    }
}

// Re-exported for callers assembling schedules through the engine.
pub use derive::{effective_schedule, expand};
