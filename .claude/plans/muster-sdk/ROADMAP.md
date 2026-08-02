# Muster-SDK — product roadmap (derived; boundary matrix lives in ../ROADMAP.md)

| Stage | Contents | Exit gate |
|---|---|---|
| PoC | greedy assignment on fixed start times | matches brute-force optimum, n ≤ 12 |
| Prototype | objective composition, violation-cost term | assignment + breakdown returned |
| Alpha | local search, stability term, expected-attendee-travel | improves on greedy for heterogeneous rooms |
| Beta | batch orchestration, digests, change sets, anytime | re-solve changes < 10% for one room removal |
| MVP | explain-assignment | organiser accepts a suggestion unedited |
| RC | CP-SAT **or** documented rejection | perf gates green |

Depends on: Orrery Prototype (oracle + scoring) for PoC; Orrery Alpha for
Beta-stage batch work. Muster Beta's room assignment needs this crate's Alpha
local search.
