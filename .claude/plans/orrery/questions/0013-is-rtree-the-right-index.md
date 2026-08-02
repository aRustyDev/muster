<!-- Imported from design thread 2026-08-01. Fable: review and improve. -->

# QUESTION 0013 — Is an R*Tree the right index for interval overlap?

* Status: **ANSWERED — NO**
* Raised: 2026-08-01

## Question

SQLite's R*Tree module was recommended repeatedly as the concrete advantage of the
relational option: a 1-D R-tree over `(start, end)` turning interval-overlap
detection from pair enumeration into an indexed range query.

## Answer

**No. Measured 2× slower** than a plain composite b-tree on the global sweep
(9,372 ms vs 4,679 ms at 1M edges).

The reason generalises beyond SQLite: Orrery's overlap queries are always
**entity-partitioned** — by person, or by room — before the interval predicate
applies. A global 1-D R*Tree has no knowledge of that partition, so it finds every
temporally-overlapping pair across the whole population and discards most of them.

The correct index is the boring composite b-tree `(person_id, start_ts, end_ts)`.
An interval index would only pay off for genuinely global interval search, which
Orrery never performs.

## Consequences / open threads

* Recommended in three consecutive messages before being measured. A clean
  example of a plausible-sounding index choice that the workload does not support.
* Generalises: **evaluate any index against the partition structure of the query,
  not against the datatype of the column.**
* See RESEARCH-0003.
