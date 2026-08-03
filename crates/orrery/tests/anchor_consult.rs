//! Phase 6a H6 — the anchor→first-event feasibility consult (ADR-0014's
//! core feature): verdicts only, judged against the FIRST placed event at
//! or after the departure instant, best across applicable anchors, and
//! `Unknown` is never an accusation.

use orrery::command::Command;
use orrery::engine::Engine;
use orrery::interval::{Interval, Timestamp};
use orrery::model::{
    Anchors, Event, EventId, Location, LocationId, Mode, Person, PersonId, Portal, Tier,
    TravelProvenance, Traverse,
};
use orrery::repo::memory::MemoryRepo;
use orrery::repo::Repository;
use orrery::travel::Feasibility;

const S: i64 = 1_000_000; // one second in µs

fn iv(s: i64, e: i64) -> Interval {
    Interval::new(Timestamp(s * S), Timestamp(e * S)).unwrap()
}

fn at(s: i64) -> Timestamp {
    Timestamp(s * S)
}

fn structure(name: &str) -> Location {
    Location {
        id: LocationId::new(),
        name: name.into(),
        tier: Tier::Structure,
        portal: Portal::None,
        capacity: None,
        ext: Default::default(),
    }
}

struct World {
    engine: Engine<MemoryRepo>,
    person: PersonId,
    home: LocationId,
    venue: LocationId,
    cabin: LocationId, // no route to anywhere
}

/// home ↔ venue: 300 s measured travel; cabin: isolated. The person
/// attends `event` (window [1000, 2000] s) at the venue.
fn world() -> World {
    let repo = MemoryRepo::new();
    let person = PersonId::new();
    let (home, venue, cabin) = (structure("home"), structure("venue"), structure("cabin"));
    let (home_id, venue_id, cabin_id) = (home.id, venue.id, cabin.id);
    let event = EventId::new();

    repo.apply(Command::UpsertPerson(Person {
        id: person,
        name: "p".into(),
        derived_digest: None,
        ext: Default::default(),
    }))
    .unwrap();
    for l in [home, venue, cabin] {
        repo.apply(Command::UpsertLocation(l)).unwrap();
    }
    repo.apply(Command::UpsertEvent(Event {
        id: event,
        name: "first of the day".into(),
        window: iv(1000, 2000),
        kind: "k".into(),
        timezone: None,
        ext: Default::default(),
    }))
    .unwrap();
    repo.apply(Command::HoldLocation {
        location: venue_id,
        event,
        during: iv(1000, 2000),
        overflow_for: None,
        capacity_override: None,
    })
    .unwrap();
    repo.apply(Command::AddTraversePair(Traverse {
        from: home_id,
        to: venue_id,
        mode: Mode("walk".into()),
        duration_typical_s: 300,
        duration_peak_s: None,
        peak_window: None,
        distance_m: None,
        provenance: TravelProvenance::Measured,
        computed_at: at(0),
        sibling_override: false,
    }))
    .unwrap();
    repo.apply(Command::AddAttendance {
        person,
        event,
        priority: None,
    })
    .unwrap();

    World {
        engine: Engine::new(repo).unwrap(),
        person,
        home: home_id,
        venue: venue_id,
        cabin: cabin_id,
    }
}

fn add_anchor(
    engine: &mut Engine<MemoryRepo>,
    person: PersonId,
    structure: LocationId,
    during: Interval,
) {
    engine
        .apply(Command::AddAnchor(Anchors {
            person,
            structure,
            label: "home".into(),
            during,
            applies_when: None,
        }))
        .unwrap();
}

#[test]
fn feasible_with_slack_when_the_gap_covers_travel() {
    let mut w = world();
    add_anchor(&mut w.engine, w.person, w.home, iv(0, 3000));
    // depart 400 s, event starts 1000 s → gap 600 s, travel 300 s.
    assert_eq!(
        w.engine
            .first_event_feasibility(w.person, iv(0, 3000), at(400))
            .unwrap(),
        Feasibility::Feasible { slack_s: 300 }
    );
}

#[test]
fn infeasible_with_deficit_and_provenance_when_it_does_not() {
    let mut w = world();
    add_anchor(&mut w.engine, w.person, w.home, iv(0, 3000));
    // depart 900 s → gap 100 s < 300 s travel.
    assert_eq!(
        w.engine
            .first_event_feasibility(w.person, iv(0, 3000), at(900))
            .unwrap(),
        Feasibility::Infeasible {
            deficit_s: 200,
            provenance: TravelProvenance::Measured
        }
    );
}

#[test]
fn unknown_without_anchors_without_route_or_without_event() {
    let mut w = world();
    // No anchors at all.
    assert_eq!(
        w.engine
            .first_event_feasibility(w.person, iv(0, 3000), at(400))
            .unwrap(),
        Feasibility::Unknown
    );
    // Anchor with no known route: still not an accusation.
    add_anchor(&mut w.engine, w.person, w.cabin, iv(0, 3000));
    assert_eq!(
        w.engine
            .first_event_feasibility(w.person, iv(0, 3000), at(400))
            .unwrap(),
        Feasibility::Unknown
    );
    // No placed event at or after the departure instant.
    add_anchor(&mut w.engine, w.person, w.home, iv(0, 3000));
    assert_eq!(
        w.engine
            .first_event_feasibility(w.person, iv(0, 3000), at(1500))
            .unwrap(),
        Feasibility::Unknown,
        "an event already started is not the next event"
    );
}

#[test]
fn best_verdict_across_anchors_feasible_beats_infeasible_beats_unknown() {
    let mut w = world();
    add_anchor(&mut w.engine, w.person, w.cabin, iv(0, 3000)); // Unknown route
    add_anchor(&mut w.engine, w.person, w.home, iv(0, 3000)); // known route
    assert_eq!(
        w.engine
            .first_event_feasibility(w.person, iv(0, 3000), at(400))
            .unwrap(),
        Feasibility::Feasible { slack_s: 300 },
        "any feasible anchor wins (don't-accuse)"
    );
    assert_eq!(
        w.engine
            .first_event_feasibility(w.person, iv(0, 3000), at(900))
            .unwrap(),
        Feasibility::Infeasible {
            deficit_s: 200,
            provenance: TravelProvenance::Measured
        },
        "a known-but-short route outranks no-data"
    );
}

#[test]
fn expired_anchor_is_not_applicable() {
    let mut w = world();
    add_anchor(&mut w.engine, w.person, w.home, iv(0, 100)); // expired by depart time
    assert_eq!(
        w.engine
            .first_event_feasibility(w.person, iv(0, 3000), at(400))
            .unwrap(),
        Feasibility::Unknown
    );
}

#[test]
fn anchor_at_the_event_location_is_trivially_feasible() {
    let mut w = world();
    add_anchor(&mut w.engine, w.person, w.venue, iv(0, 3000));
    assert_eq!(
        w.engine
            .first_event_feasibility(w.person, iv(0, 3000), at(400))
            .unwrap(),
        Feasibility::Feasible { slack_s: 600 },
        "same location: slack is the whole gap, no route needed"
    );
}

#[test]
fn the_first_event_is_judged_not_the_reachable_one() {
    let mut w = world();
    add_anchor(&mut w.engine, w.person, w.home, iv(0, 3000));
    // An EARLIER event at the isolated cabin becomes the first event; the
    // reachable venue event later must not be the one judged.
    let early = EventId::new();
    w.engine
        .apply(Command::UpsertEvent(Event {
            id: early,
            name: "earlier, unreachable".into(),
            window: iv(500, 600),
            kind: "k".into(),
            timezone: None,
            ext: Default::default(),
        }))
        .unwrap();
    w.engine
        .apply(Command::HoldLocation {
            location: w.cabin,
            event: early,
            during: iv(500, 600),
            overflow_for: None,
            capacity_override: None,
        })
        .unwrap();
    w.engine
        .apply(Command::AddAttendance {
            person: w.person,
            event: early,
            priority: None,
        })
        .unwrap();
    assert_eq!(
        w.engine
            .first_event_feasibility(w.person, iv(0, 3000), at(0))
            .unwrap(),
        Feasibility::Unknown,
        "the consult judges the first event (no route to it), not the later reachable one"
    );
}

/// Rule 00.6/09: every verdict variant, rendered, carries no anchor
/// association — neither the anchor's location id nor its label. Pins the
/// verdict type against future regression (`Feasibility` also derives no
/// serde by design: there is nothing to serialize but durations).
#[test]
fn privacy_first_event_verdicts_carry_no_anchor_association() {
    let mut w = world();
    add_anchor(&mut w.engine, w.person, w.home, iv(0, 3000));

    let feasible = w
        .engine
        .first_event_feasibility(w.person, iv(0, 3000), at(400))
        .unwrap();
    let infeasible = w
        .engine
        .first_event_feasibility(w.person, iv(0, 3000), at(900))
        .unwrap();
    let unknown = w
        .engine
        .first_event_feasibility(w.person, iv(0, 3000), at(1500))
        .unwrap();

    let home_uuid = w.home.to_string();
    let venue_uuid = w.venue.to_string();
    for v in [feasible, infeasible, unknown] {
        let rendered = format!("{v:?}").to_lowercase();
        assert!(
            !rendered.contains(&home_uuid) && !rendered.contains(&venue_uuid),
            "verdict leaked a location id: {rendered}"
        );
        assert!(
            !rendered.contains("home") && !rendered.contains("label"),
            "verdict leaked anchor data: {rendered}"
        );
    }
}
