<!-- Imported from design thread 2026-08-01. Fable: review and improve. -->

# QUESTION 0001 — Graph database or relational for the system of record?

* Status: **OPEN — research mandate**
* Raised: 2026-08-01

## Question

The model has nodes and edges, which suggests a graph store. But the shape of the
*queries* may not follow the shape of the *data*. Which store fits Orrery's actual
workload?

## Answer

Provisionally **unresolved**, and deliberately so.

Empirical result (RESEARCH-0003) leans embedded relational, but by a narrower
margin than a casual reading suggests: of **five** directly comparable queries,
SQLite wins three (Q1, Q2, Q7b) by 5.7-21x, **loses Q3 by 2.3x**, and ties Q5.
It loads 3.2x faster; Ladybug uses 3x less disk. Q4 and Q6 have no SQLite
counterpart. The mechanism is
`orrery/SPEC-00` invariant 2 — **every Orrery query is entity-partitioned before its
interval predicate applies**, leaving partitions of tens of rows, which is
exactly what b-tree-indexed row stores do best.

However, the graph option was tested only through Ladybug (C++, FFI from Rust).
The **Rust-native embedded graph landscape was never surveyed**, and the project
owner has stated a continued preference for an embedded graph store. See
QUESTION-0012 and ADR-0015.

## Consequences / open threads

* ADR-0015 remains `proposed`, not `accepted`.
* All persistence sits behind a repository trait regardless of outcome.
* The measured SQLite numbers are the **baseline any graph candidate must beat**
  or justify losing to.
