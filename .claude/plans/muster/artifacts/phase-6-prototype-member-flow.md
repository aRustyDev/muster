# Muster can now do everything a member needs — and it has a real web API

*Phase 6 slice 2 (Prototype), 2026-08-02. Plain-language explainer (the
standing artifact criterion). The previous artifact told the story of one
conflict becoming visible; this one is about the loop closing.*

## What a member can do now

Imagine Ada at a conference. As of this slice, the complete everyday loop
works:

1. **Browse** — "what's on today?" She sees every session with its time
   and its room: *09:00–10:00 Intro to Rust (Room 101)*.
2. **Select** — she picks Intro to Rust, and also the Systems Workshop,
   which overlaps it. The moment she picks the second one, the system
   answers: *these two collide.* Nobody recomputed anything in the app —
   the scheduling engine recorded the collision, and the app just shows
   the record.
3. **Prioritise** — she marks the workshop as the one she cares about
   more (0.95). Her own preference is stored as *hers*; if a coordinator
   later suggests otherwise, both opinions survive side by side.
4. **See her schedule with provenance** — every entry says *why it's
   there*: "you picked this", or "expected via group 'cohort-26'" for the
   evening social her cohort's coordinator expects everyone at. That
   social was never written into her selections — it's *derived* from the
   group expectation, live.
5. **Resolve** — new this slice, and the piece that makes it a *loop*:
   she can **deselect** the talk she's dropping, and the conflict warning
   disappears on its own. Until now the whole system, engine included,
   had no way to un-choose anything — an adversarial review of our plans
   caught that the day this slice started, and the fix went in at the
   engine level, not as an app workaround.

## The app grew a real shape

Until now Muster was a library and a command-line demo. It's now three
new pieces, chosen by a decision record that was verified against the
current state of the Rust ecosystem (by search, not memory):

* **muster-server** — a small web server (axum). Every piece of data
  leaving the system goes through here, which is exactly the point: this
  is the single door where privacy is enforced. Home addresses
  (*anchors*) can never pass through it — the message formats simply
  have no field that could carry one.
* **muster-ui** — the beginnings of a browser interface (Dioxus, Rust
  compiled to run in the browser). This slice ships its skeleton: the
  screens compile against the very same message types the server sends,
  so the two cannot drift apart without the build failing.
* **muster-types** — that shared vocabulary of messages, deliberately
  tiny and deliberately coordinate-free.

We proved the loop twice: once directly against the service, and once
over genuine HTTP — the server was booted for real and driven with curl
before any test was trusted.

## We timed the click — and the news is mixed on purpose

This project's rules require measuring before optimising, and reporting
bad news first. First-ever timing of the "select" click, at a
1,000-person scale: **about 98 milliseconds typically — and about 102 ms
at the slow end, against a 100 ms budget.** It passes the letter of the
budget with 2% to spare and fails its spirit: there is no headroom, and
1,000 people is a small deployment. The cause is known (every click
currently re-checks the *whole world*, not just the person clicking),
the fix is designed (check just that person), and it is now a committed
item for the next slice rather than a hopeful someday.

## What deliberately isn't here yet

Coordinator tools (creating groups, setting expectations, previewing the
blast radius of a change before committing it), the violation inbox, and
the browser UI's actual content — all next slices, in that order. One
piece of engine work (a "what would change if…" preview that doesn't
write anything) has to land first; it's scheduled and tracked.

*Everything above is asserted by 72 automated tests, all green.*

*(Corrected 2026-08-03, quality review F-6: 72 was the whole-workspace
test count on that date — the engine's tests included — not the number of
tests asserting this artifact's claims. The member-flow behaviours above
are asserted specifically by the muster/muster-server `e2e_` and
`privacy_` families; the workspace total has since grown and is not a
property of this document.)*
