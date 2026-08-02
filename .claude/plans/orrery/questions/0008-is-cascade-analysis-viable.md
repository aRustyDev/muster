<!-- Imported from design thread 2026-08-01. Fable: review and improve. -->

# QUESTION 0008 — Is unbounded cascade / blast-radius analysis viable?

* Status: **ANSWERED — NO**
* Raised: 2026-08-01

## Question

'If I move this seminar, what is the full downstream impact?' was identified as
the analytic most justifying a graph database: unbounded depth, no natural
termination, painful in SQL.

## Answer

**No.** Measured on a 2,000-person graph, reachability saturates by hop 3 and hits
**100% of the graph at hop 4**, costing 19.6 seconds to answer 'everyone'.

This is a property of the domain, not any engine — co-attendance graphs are
small-world. Separately, the temporally-correct form is not expressible in
Ladybug at all: per-hop filters are stateless and cannot reference the previous
hop.

Replaced by bounded 2-hop co-attendance with a time window: 2,808 persons in
1.2–11.3 ms.

## Consequences / open threads

* **This removed the strongest marginal argument for a graph database.**
  Withdrawing the requirement does not rescue the graph option — it weakens it.
  That asymmetry must be preserved when ADR-0015 is revisited.
* Reopen only if a bounded variant with strong pruning proves useful at depth > 2.
* See ADR-0020, RESEARCH-0004.
