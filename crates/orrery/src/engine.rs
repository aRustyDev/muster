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
use crate::model::{
    AttendanceSource, Attends, EntityRef, Held, PersonId, Severity, Violation, ViolationId,
    ViolationKind,
};
use crate::repo::Repository;
use crate::{derive, incremental};

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
