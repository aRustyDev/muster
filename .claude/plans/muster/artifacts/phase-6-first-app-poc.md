# Phase 6 (PoC), in plain language — the first thing a person can actually use

*For any reader. Technical record: `../phases/06-app.md`; try it yourself:
`cargo run -p muster -- demo`.*

## What got built

Everything before this phase was machinery: a referee that spots impossible
schedules, an assistant that drafts room plans, a caretaker that tidies up
nightly. All of it invisible. This phase built the first sliver of the thing
a human touches — and proved the machinery underneath actually carries a
person's story end to end.

The story is deliberately tiny, and you can run it:

```text
muster demo — the PoC story

Ada selected two talks; the system immediately shows 1 problem(s).
  Intro to Rust      — you picked this  ⚠ CONFLICT
  Systems Workshop   — you picked this  ⚠ CONFLICT
  Evening Social     — expected via group 'cohort-26'

1 conflict(s) detected · 1 derived entry with provenance
```

Three things happened there, and each one is a promise the whole product
rests on:

**1. Ada picks two talks that overlap — and finds out immediately.**
Not at publishing time, not when a colleague notices: the moment she
selects the second talk, the conflict is on her screen. And she's still
*allowed* to keep both — this system flags problems, it doesn't slap your
hand. Maybe she plans to leave one talk early. The flag, and her choice,
both stay on the record.

**2. An event appears on her schedule that she never picked** — the
Evening Social — because her cohort's coordinator expects the cohort to be
there. Notice what the line says: *expected via group 'cohort-26'*. Every
entry on a schedule answers "why is this here, and who put it there?"
Nothing appears by magic; provenance is part of the data, not a tooltip
someone remembered to add.

**3. The app itself is deliberately dumb.** The conflict warning wasn't
computed by the screen showing it — it was read from the referee's records.
We check this mechanically: the app's source code contains no
conflict-detection logic at all. Why insist? Because two definitions of
"conflict" — one in the engine, one in the UI — eventually disagree, and
then the screen lies. There is exactly one place in the whole system that
knows what a conflict is.

## A small honest confession

Running the demo (not just its tests) caught a bug: the Evening Social
printed *first*, before the morning talks. Cause: an "expected via group"
entry internally carries the window during which the *expectation* applies
(the coordinator set it for the whole year), and the display used that
instead of when the event actually happens. One line fixed it — but it's a
useful reminder of why this project keeps insisting on running the real
thing and writing down what went wrong.

## What was deliberately NOT decided

You may notice there's no website yet — the demo is a command-line
program. That's on purpose. The question "what web technology should the
interface use?" was explicitly parked months ago with the rule *don't
decide before something real works end to end*. Something real now works
end to end, without committing to any web stack — so the decision can be
made next, once, with evidence, instead of early and twice.

## Where the project stands

Every layer of the system has now reached at least its proof-of-concept
milestone:

| Layer | State |
|---|---|
| Engine (the referee) | prototype+ — detects, derives, scores, stays incremental |
| SDK (the assistant) | complete for v1 — drafts, polishes, explains, maintains |
| App (what people touch) | proof-of-concept — one member's story works end to end |

Next: the full member experience (browse, priorities, a real interface —
and that parked technology decision), then the coordinator's tools: groups,
expectations, the "who would this affect?" preview, and the problem inbox.
