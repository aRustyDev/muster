//! Phase 6 PoC gate (muster/SPEC-03): conflict visible end to end through
//! engine records; derived entry with provenance, no attendance write.

use muster::{run_demo, EntrySource};

#[test]
fn e2e_member_selects_and_sees_conflict_with_provenance() {
    let report = run_demo().expect("demo world builds and runs");

    // The conflict is visible immediately after the second selection.
    assert!(
        report.conflicts_after_selection >= 1,
        "overlapping self-selections must surface a conflict"
    );

    // The schedule shows all three entries: two picked, one derived.
    assert_eq!(report.schedule.len(), 3, "{:#?}", report.schedule);
    assert_eq!(report.derived_entries, 1);

    // The derived entry names its source group (provenance, PRD FR-6).
    let derived = report
        .schedule
        .iter()
        .find(|e| matches!(e.source, EntrySource::DerivedFromGroup { .. }))
        .expect("derived entry present");
    match &derived.source {
        EntrySource::DerivedFromGroup { group_name, .. } => {
            assert_eq!(group_name, "cohort-26");
        }
        _ => unreachable!(),
    }
    assert_eq!(derived.event_name, "Evening Social");
    assert!(!derived.flagged, "the social conflicts with nothing");

    // Both overlapping talks are flagged — and the flags came from open
    // violation records (the service never computes feasibility).
    let flagged: Vec<_> = report.schedule.iter().filter(|e| e.flagged).collect();
    assert_eq!(flagged.len(), 2, "{:#?}", report.schedule);
}
