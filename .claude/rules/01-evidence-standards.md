# Rule 01 — Evidence standards

The design thread refuted three of its own strongest claims under measurement.
That rate is normal. These rules exist to keep it visible.

1. **Hypothesis before measurement.** Write what you expect and what would
   falsify it, before running anything.
2. **Acceptance criteria are pre-committed.** If a criterion proves
   mis-specified after seeing results, **say so in writing and explain why** —
   do not silently drop it. The source thread's Phase-4 gate had exactly this
   happen (the Q7 monotonicity criterion), and recording it was more useful than
   the criterion would have been.
3. **Report refutations at least as prominently as confirmations.** A research
   document that only confirms is a document that did not test anything.
4. **Label evidentiary status.** Distinguish: measured directly · strictly
   entailed · plausible inference · unverified premise. Do not let inference
   harden into fact as it propagates between documents.
5. **Cite the run.** Any number in a document names the script and scale that
   produced it. Numbers without provenance get deleted, not trusted.
6. **Check arithmetic on summary claims.** "Wins N of M" statements are the
   most-copied and least-checked sentences in any evaluation. The source package
   shipped a wrong one (`4 of 6`, actually 3 of 5) into two load-bearing
   documents before an adversarial review caught it.
7. **Measure the shape of the real workload.** The source harness returned
   `count(*)` for every query and therefore never measured result
   materialisation — the same class of error as recommending an index that was
   never benchmarked.
