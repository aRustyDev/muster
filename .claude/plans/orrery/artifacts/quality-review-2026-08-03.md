# The quality review, in plain language — we audited our own safety net

*For any reader. The technical record lives in
`../../quality-review/` (plan, evidence matrix, synthesis, fix slice);
this artifact covers the whole review, closed 2026-08-03.*

## What we did

This project makes strong promises about how it tests itself: privacy
guarantees "asserted by an automated test, not a review checklist",
performance budgets, gates that must be green before any stage advances.
Over two days we audited every one of those promises against reality —
all six crates plus the workspace itself, across 42 distinct quality
dimensions (from ordinary unit tests to fuzzing, license compliance, and
disaster-recovery drills). 294 cells, each judged with a citation:
covered, partial, gap, or not-applicable-with-a-written-reason.

The method was the same one this project uses for engineering: write
down what you expect first, then report the ways you were wrong before
the ways you were right.

## The embarrassing findings first (that's the house rule)

* **Several of our documented quality "doors" didn't open.** Commands
  the docs told you to run — the property-test suite, the detector
  suite, the benchmark runner — failed with errors or, worse, silently
  succeeded while running *zero tests*. One gate that claims to fail the
  build if a forbidden dependency sneaks in **could never fail at all**:
  a shell scripting bug swallowed its alarm. It had been broken since
  the day it was written, and nobody noticed — because nothing was
  checking the checkers. All fixed and now proven to fail on demand.
* **We write "CI" as if it exists. It doesn't.** No continuous
  integration is set up: every "all tests green" in the record was run
  by hand on one laptop, honestly labeled as such. An accepted decision
  record even stated "CI runs the tests as before" — a false premise now
  corrected in place. Standing CI up is the review's single
  highest-leverage item, and it now has an owner and a plan.
* **Our public write-ups inflated two numbers.** "Thousands of random
  schedules" was really 48 per property; "asserted by 72 automated
  tests" counted the whole workspace, not that document's claims. Both
  corrected with dated notes. The lesson is old: summary claims are the
  most-copied and least-checked sentences anywhere.
* **The review's own bookkeeping had the same disease.** Its plan said
  the matrix was 36 dimensions (it was 40), one finding counted "twelve
  stale statements" while listing thirteen, and the fix-slice closed
  while a kickoff file still pinned the old test count — which would
  have halted the next session at its front door. All caught by applying
  the review's rules to the review itself.

## What was already strong

The core disciplines held up well: every detector is tested against a
brute-force referee; privacy of home addresses is enforced at a single
choke point and mechanically tested at the two boundaries that exist
today; measurements come with pinned provenance; refuted hypotheses stay
pinned as regression tests. The review's job was mostly to extend that
rigor to the channels and crates it hadn't reached yet.

## What changed immediately (the fix slice, merged the same day)

Fourteen items needed no debate: broken doors repaired; a supply-chain
and license gate (`cargo deny`) that runs today, not "once CI exists";
every feature combination now compile-checked, including one documented
configuration that had *never* been exercised; unsafe Rust forbidden by
the compiler workspace-wide; error-handling and wire-format contracts
pinned by new tests (the suite grew 91 → 93); and a written
measurement-variance policy — warm-ups, repeated rounds, medians — so a
single lucky run can't set a record again.

## What's scheduled rather than guessed

Nine tool decisions were deliberately **not** made on the spot — each is
a written stage with the question, the candidates (all 50 candidate
tools were verified for maintenance health; 23 failed), the deciding
criteria, and the milestone it must precede. Coverage measurement,
mutation testing, real fuzzing, benchmark harnesses, CI shape, input
validation, UI testing, load testing — each closes right before the work
that needs it, never after.

Two additions matter beyond tooling: **privacy testing** and
**operational drills** (backup/restore, rebuild-from-scratch) are now
first-class dimensions in the quality matrix, so a written promise in
those areas can't float unowned again — six specific unasserted privacy
promises now have named owners and deadlines.

## Where the strategy now lives

One new document — `plans/TESTING-STRATEGY.md` — is the single home for
cross-crate policy (tools, naming, variance, regression rules). Each
product's testing spec carries only its own criteria and links there.
Every accepted item sits in the project ledger with an owner; the
version-number forecast for the road ahead is written down and labeled
as the estimate it is. Nothing in the review touched the one open
architectural decision (the datastore stays open, by rule, until its
evidence stage).
