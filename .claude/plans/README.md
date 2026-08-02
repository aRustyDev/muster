# plans/ — layout and derivation

## Why the overall PLAN.md and ROADMAP.md live at the plans/ root

**The roadmap's primary job is drawing boundaries *between* the three
products** — triplicating it per product would fragment the one thing it
exists to show. Per-product copies are derivations, not duplicates.

## What to put in the per-product copies

Derive, do not duplicate.

`plans/{product}/PLAN.md` — only the phases touching that product, with its
specific entry and exit conditions. Link to root `PLAN.md` for cross-product
sequencing rather than restating it.

`plans/{product}/ROADMAP.md` — only that product's stage table and its
dependencies on the other two. The full boundary matrix stays at the root.

**Rule: any fact appearing in both a root and a per-product document is a fact
that will drift.** Link instead.

## Numbering

Question, research, ADR, and phase numbers are **global**, not per-directory.
QUESTION-0005 sits in `muster-sdk/questions/` and creates an apparent gap in
`orrery/questions/`. That is expected — see `orrery/questions/README.md`.

## Spec references

Spec numbers are **per-product** and collide across products. Always qualify:
`orrery/SPEC-03`, not `SPEC 03`. Three products each have a `00-overview.md`.

## Known thin spots — Fable should fill

| Product | Has | Missing |
|---|---|---|
| orrery | 00-05: overview, data model, functional, non-functional, API, testing | — |
| muster-sdk | 00 overview, 01 objectives/search | data model, API surface, testing criteria |
| **muster** | 00 overview only | **data model, API surface, user flows, non-functional, testing criteria** |

The target tree marks `specs/` as Fable-generated. Those supplied here are
drafts to build on, not a finished set — and the muster set is deliberately
incomplete because the application surface was the least-discussed part of the
design thread.
