//! Phase 6 slice 2 — Prototype gate (muster/SPEC-03): full member flow
//! through the service layer. Browse → select(+priority) → conflict
//! visible → set priority → deselect → conflict RESOLVED — everything
//! read from engine records, nothing recomputed here (plan-review CR-6
//! added the deselect leg: PRD Flow A ends "resolve or accept").

use muster::{build_demo_world, EntrySource};

#[test]
fn e2e_member_browses_selects_prioritises_deselects_and_resolves() {
    let (mut svc, w) = build_demo_world().expect("demo world builds");
    let (ada, day, now) = (w.member, w.day, w.now);

    // -- Browse: three events, each with room and time (PRD FR-1).
    let events = svc.events(day).expect("browse");
    assert_eq!(events.len(), 3, "{events:#?}");
    let rust = events.iter().find(|e| e.event == w.talk_rust).unwrap();
    assert_eq!(rust.room.as_deref(), Some("Room 101"), "room joined");
    assert!(
        events
            .windows(2)
            .all(|p| p[0].window.start() <= p[1].window.start()),
        "browse is time-ordered"
    );

    // -- Select two overlapping talks: the conflict is immediate and comes
    // from violation records (H1).
    svc.select(ada, w.talk_rust, Some(0.9), now, day).unwrap();
    let outcome = svc
        .select(ada, w.talk_systems, Some(0.7), now, day)
        .unwrap();
    assert!(!outcome.conflicts.is_empty(), "conflict must be visible");

    // -- Set a personal priority on an existing selection (PRD FR-2).
    let seq = svc
        .set_priority(ada, w.talk_systems, 0.95)
        .expect("set_priority");
    assert!(seq > 0);

    // -- Schedule shows both talks flagged, the social derived with
    // provenance (PoC guarantees still hold under the extended flow).
    let view = svc.my_schedule(ada, day, now).unwrap();
    assert_eq!(view.entries.len(), 3, "{view:#?}");
    assert!(view.entries.iter().any(
        |e| matches!(&e.source, EntrySource::DerivedFromGroup { group_name, .. } if group_name == "cohort-26")
    ));
    assert_eq!(view.entries.iter().filter(|e| e.flagged).count(), 2);

    // -- Resolve (H3): deselect one talk; the sweep auto-resolves the
    // violation — zero app-side logic.
    let after = svc.deselect(ada, w.talk_rust, now, day).expect("deselect");
    assert!(
        after.conflicts.is_empty(),
        "conflict must auto-resolve: {:#?}",
        after.conflicts
    );
    let view = svc.my_schedule(ada, day, now).unwrap();
    assert_eq!(view.entries.len(), 2, "dropped talk gone: {view:#?}");
    assert!(
        view.entries.iter().all(|e| !e.flagged),
        "no flags remain after resolution"
    );
}
