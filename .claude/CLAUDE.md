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

<!-- TODO: Updating Context Files -->
<!-- TODO: Updating Context Files - When (ie what triggers updates, what should be kept in context files?) -->
<!-- TODO: Updating Context Files - Where (ie what kind of context update goes where?) -->
<!-- TODO: Reviewing Context Files - When (ie what triggers reviews of context files?) -->
<!-- TODO: Adding/Recording Skills -->
<!-- TODO: Reviewing/Updating Skills -->
