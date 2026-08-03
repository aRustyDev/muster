# Phase 6a in plain language — four things the engine can do now

*2026-08-02. Companion to `phases/06a-engine-surfaces.md`, written for a
reader who doesn't live in this codebase.*

Orrery is the engine that checks whether a plan — people, events, rooms,
times — is actually possible. Muster is the scheduling app built on top
of it. This phase gave the engine four abilities the app's next two
releases were waiting on.

## 1. "What would happen if…?" — a preview that cannot lie

Imagine a coordinator about to tell an entire department "you're all
expected at Friday's all-hands." Before committing, they want to know:
*whose schedules does this actually touch?*

The engine can now answer that question **without saving anything** —
like a spell-checker showing you the red underlines before you hit
Send. Ask it "what if this group got this expectation?" and it returns
exactly the list of people whose schedules would change.

The dangerous failure mode for any preview is drift: the preview says
one thing, the real commit does another. A preview that lies is worse
than no preview. So the honesty is enforced by an automated test that
generates hundreds of random worlds, runs the preview, then *really
commits* the same change and checks the two answers are **identical** —
every time, with proof the preview wrote nothing.

It's also fast in the way that matters: if a change touches 3 people
out of 10,000, the engine re-computes 3 schedules, not 10,000. We don't
assume that — a counter inside the engine proves the expensive work ran
exactly 3 times.

## 2. "Can you make the 8am from home?" — without revealing where home is

People can now register **anchors** — home, the office, a partner's
place — as the spot their day starts from. That lets the engine answer
the question every commuter asks: *if I leave home at 7, do I make the
8 o'clock?*

Anchors are home addresses, so privacy is built into the shape of the
answer, not bolted on: the engine replies only **"yes, with 25 minutes
to spare," "no, you'd be 10 minutes short,"** or **"I don't know."**
The reply physically cannot carry the anchor — the answer type has no
field for it, and automated privacy tests now run against worlds that
*contain* real anchors (something that was literally impossible to test
before this phase, because anchors couldn't be stored at all).

One deliberate courtesy: **"I don't know" is never an accusation.** If
the engine has no travel data for a route, it says so rather than
flagging a person as impossibly scheduled.

## 3. The pulse of a schedule — four analytics

Numbers coordinators will see in the app's next releases, computed by
the engine so every consumer gets the same truth:

* **Engagement** — how loaded is each person, weighted by how much each
  commitment matters to them?
* **Capacity pressure** — for each event, how many people signalled
  real interest vs. how many seats exist? (This is a *ranking signal*,
  deliberately not a attendance forecast — predicting turnout honestly
  requires data on actual turnout, which doesn't exist yet.)
* **Divergence** — where coordinators' priorities and members' own
  priorities disagree, and by how much. A quiet early-warning metric:
  a group with high divergence is a group being scheduled against its
  will.
* **Who crosses paths** — for one person, everyone who shares at least
  one event with them in a time window. Bounded at exactly two hops
  because earlier research showed anything deeper degenerates into
  "everyone is connected to everyone" (true, but useless).

Each one is checked against a deliberately dumb, obviously-correct
recomputation across hundreds of random worlds — the same discipline
every conflict detector in the engine already follows.

## 4. A speed bar you can actually measure — and it's cleared

The project's roadmap said the engine's "Alpha" stage exits when
"budgets are met at 10⁵" — but no one had ever defined what the budget
set at that scale *was*, making the stage impossible to exit. A review
in early August flagged this.

This phase fixed the definition (same time limits as the existing
large-scale table, at a world of **100,000 attendances, 1,000 people,
1,000 events, 200 buildings**) and then measured all seven checks. For
scale: the strictest budgets are 25–50 milliseconds — about half the
blink of an eye. Results, in an optimized build:

| Check | Allowed | Measured |
|---|---|---|
| Open the engine cold | 1 s | 0.000126 s |
| Rebuild the travel-time table (39,800 routes) | 60 s | 0.017 s |
| Scan the whole world for every conflict | 10 s | 2.5 s |
| One person's derived schedule | 25 ms | 0.003 ms |
| One person's conflict check | 25 ms | 0.077 ms |
| One person's travel check | 25 ms | 2.4 ms |
| One person's who-crosses-paths | 50 ms | 7.4 ms |

Every check passes, so **the engine's Alpha stage gate is met**. Two
honest footnotes, recorded in full in the phase document: the verdict
comes from an optimized build (an unoptimized debug build fails the two
heaviest checks — the whole-world scan and who-crosses-paths), and it
was measured on the deliberately simple in-memory store, not the final
database — that decisive contest happens at ten times this scale in
Phase 7.

## Why this mattered now

Muster's next release (the coordinator experience) needs the preview;
the one after (analytics dashboards) needs the metrics. Both were
promised by planning documents but owned by no phase — the August plan
review caught the orphaned work, and this phase is the fix. With it,
nothing on the engine side blocks the app's next two releases.
