<!-- Imported from design thread 2026-08-01. Review, do not assume correct. -->

# 5. Priority is a precedence stack, not a single column

* Status: accepted
* Date: 2026-08-01

## Context and Problem Statement

Three parties claim `priority_score`: the group default, the individual, and a
coordinator who may either *suggest* or *override*.

## Decision Outcome

Store all three plus a binding flag; compute an effective value.

```text
attends.priority_group        f32   -- from expects.default_priority
attends.priority_person       f32?  -- the user's own
attends.priority_coord        f32?
attends.coord_binding         bool  -- true = override, false = suggest

effective = if coord_binding && priority_coord.is_some() { priority_coord }
            else { priority_person ?? priority_coord ?? priority_group }
```

### Consequences

* User input is never destroyed. A binding override changes the effective value
  without erasing what the person wanted.
* **Divergence becomes a signal.** `|priority_coord − priority_person|`
  aggregated over a group is a real analytic ("events I flagged as important
  that my cohort is deprioritizing") that exists only because the components
  were kept separate.
* Three columns instead of one; effective value must be computed consistently
  in exactly one place.
