# Orrery / Muster — working agreement

**Orrery** is a spatiotemporal feasibility engine: given a proposed assignment of
people to events at locations over time, it returns the ways that assignment is
impossible, and a score for how good it is. **Muster** is the first application
on it. **Muster-SDK** is the search layer between.

Orrery does not schedule. It decides whether a schedule is possible.

## Read in this order

1. `.claude/rules/**` — binding constraints on how you work
2. `PLAN.md` — phases · `ROADMAP.md` — boundaries and stage gates
3. `.claude/plans/*/prds/` — what each product is and why
4. `docs/src/dev/adrs/` — the decision corpus; **ADR-0015 is open**
5. `.claude/plans/orrery/specs/` — model, requirements, API, tests
6. `evidence/README.md` — reproduce before trusting

## The one open decision

**ADR-0015 (datastore) is `proposed`, not `accepted`.** Measured evidence leans
embedded relational; the project owner prefers an embedded graph store. Both are
legitimate inputs. ADR-0021 defines the selection funnel: only the paper screen
blocks implementation.

## Working style

Direct and evidence-grounded. Flag genuine uncertainty; do not hedge to be
polite. Play devil's advocate on ambiguous findings. Show the plan before
executing. Prefer incremental delivery — MVP scope first.

## Invariants worth memorising

* Every relation carries a validity window; every conflict check is the same
  interval-overlap predicate.
* **Every query is entity-partitioned before its interval predicate applies.**
  This single fact drives index selection, datastore selection, and why an
  R\*Tree measured 2x *slower* than a plain composite b-tree.
* Derived semantics, cached physically. The blast radius of a membership write
  is unbounded and invisible; salsa early cutoff is what makes it computable.

## Context files: what, when, where (ADR-0027)

Context files carry **constraints and orientation**, not knowledge —
durable knowledge goes to `docs/src/` (placement table: Rule 10). A
binding convention stated in conversation lands in a rule or `docs/src/`
**before the session ends**, or becomes a ledger row.

* **Update on**: slice close (protocol step) · a refutation or correction
  landing · an owner decision changing a constraint.
* **Review on**: every compaction (hook-injected reminder) and every
  slice close — scan loaded instructions against repo state; fix stale
  instances now or ledger them (the F-13 fix).
* **Where**: decision→ADR · enforcement→policy (artifact gate) or rule
  (agent behavior) · goal→strategy · recipe→pattern · orientation→
  AGENTS.md · personal→`CLAUDE.local.md` (gitignored, never committed).

## Skills, agents, scripts, hooks: built from evidence (ADR-0027 §7–8)

Create from **ledger evidence** (`.claude/observations.md` — harvest at
compaction and slice close; rule of three graduates a candidate), never
from taste. Boundary: a *skill* packages a repeatable procedure for the
main context; an *agent* buys a separate context window and tool
sandbox; **an agent without a named output artefact is a chat**. A
misfiring or drifted skill/agent is a ledger row; the roster is reviewed
against current platform guidance at each CR slice, citing the guidance
version checked against.
