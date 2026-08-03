//! Muster UI (ADR-0025): dioxus components rendering `muster-types` wire
//! DTOs. Structure is the Prototype deliverable — shared types compile-
//! checked against the same contract the server serves. Content, the REST
//! client, and the `dx` web entrypoint are Alpha scope (recorded in
//! phases/06-app.md); this library builds host-side under the local
//! workspace gates — `just ci`, plus the bare no-features leg of
//! `just matrix` (cargo-hack) — and gains the `web` renderer only under
//! `--features web`. (Corrected 2026-08-03, quality review F-13/F-11:
//! this header claimed "workspace CI" checks the bare library; no CI
//! exists yet — RR&P-1 — and the bare leg first ran on 2026-08-03.)

use dioxus::prelude::*;

use muster_types::{EventDto, ProvenanceDto, ScheduleDto};

const HOUR_US: i64 = 3_600 * 1_000_000;

/// Microsecond instant → "HH:MM" within the demo day. Display-only
/// convenience; no interval math (Rule 03) — the engine owns semantics,
/// this renders labels.
fn hhmm(us: i64) -> String {
    let h = us / HOUR_US;
    let m = (us % HOUR_US) / (60 * 1_000_000);
    format!("{h:02}:{m:02}")
}

/// PRD Flow A, browse: events with room and time, plus a select action.
#[component]
pub fn EventList(events: Vec<EventDto>, on_select: EventHandler<EventDto>) -> Element {
    rsx! {
        ul { class: "event-list",
            for e in events {
                li { key: "{e.id}",
                    span { class: "when", "{hhmm(e.start_us)}–{hhmm(e.end_us)}" }
                    span { class: "name", " {e.name} " }
                    span { class: "room",
                        match &e.room {
                            Some(r) => rsx! { "({r})" },
                            None => rsx! { "(no room yet)" },
                        }
                    }
                    button {
                        onclick: {
                            let e = e.clone();
                            move |_| on_select.call(e.clone())
                        },
                        "select"
                    }
                }
            }
        }
    }
}

/// PRD FR-6/FR-8: my-schedule with provenance and conflict flags — both
/// arrive on the wire from engine records; nothing is recomputed here.
#[component]
pub fn ScheduleView(schedule: ScheduleDto) -> Element {
    rsx! {
        ul { class: "schedule",
            for e in schedule.entries {
                li { key: "{e.event}",
                    span { class: "when", "{hhmm(e.start_us)}–{hhmm(e.end_us)}" }
                    span { class: "name", " {e.name} " }
                    span { class: "provenance",
                        match &e.provenance {
                            ProvenanceDto::SelfSelected => rsx! { "— you picked this" },
                            ProvenanceDto::Coordinator => rsx! { "— a coordinator placed this" },
                            ProvenanceDto::Group { name } => rsx! { "— expected via group '{name}'" },
                        }
                    }
                    if e.flagged {
                        span { class: "conflict", " ⚠ CONFLICT" }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::hhmm;

    #[test]
    fn hhmm_renders_hours_and_minutes() {
        assert_eq!(hhmm(9 * super::HOUR_US), "09:00");
        assert_eq!(hhmm(18 * super::HOUR_US + 30 * 60 * 1_000_000), "18:30");
    }
}
