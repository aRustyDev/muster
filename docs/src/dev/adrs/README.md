# Decision records — layout

MADR-format ADRs, numbered **globally and sequentially** (never reused,
never renumbered — Rule 02), nested by topic (ADR-0027). The number is
the identity; the topic directory is just shelving. `just docs::adr-next`
gives the next number regardless of topic. SUMMARY.md lists every ADR
in numeric order via `just docs::summary` — never hand-edit its
generated section.

| Topic | ADRs |
|---|---|
| `domain-model/` | 0002–0005, 0009–0012, 0014, 0017, 0018, 0024 |
| `travel/` | 0006–0008 |
| `datastore/` | 0015, 0020, 0021 — **ADR-0015 is open (`proposed`)** |
| `architecture/` | 0013, 0016, 0019, 0023, 0025 |
| `dependencies/` | 0022 |
| `testing/` | 0026 |
| `project/` | 0001, 0027 |

Topic vocabulary is open (shared with `dev/strategies|policies|patterns`);
a new topic directory is created on first real document, never as a stub.
