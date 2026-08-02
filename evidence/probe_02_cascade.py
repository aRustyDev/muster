"""Q7 forensics.

Two questions:
  (a) Does the co-attendance neighbourhood saturate? If 4-hop = "everyone",
      the query is semantically vacuous regardless of speed.
  (b) Can per-hop filters express TEMPORAL MONOTONICITY (each hop strictly
      after the previous)? Part-5 criterion assumed yes. Verify.
"""
import os as _os, pathlib as _pl, sys as _sys
_HERE = _pl.Path(__file__).resolve().parent
WORK = _os.environ.get("ORRERY_WORK", str(_HERE / "_work"))
_os.makedirs(WORK, exist_ok=True)
_sys.path.insert(0, str(_HERE))

import ladybug, time

_db = f"{WORK}/orrery_M.lbdb"
if not _os.path.exists(_db):
    _sys.exit(f"PREREQUISITE MISSING: {_db}\n"
              f"This probe reads the M-scale database built by orrery_spike.py.\n"
              f"Run first:  python3 evidence/orrery_spike.py M   (~30s)")
db = ladybug.Database(_db, read_only=True)
conn = ladybug.Connection(db)

def q(query, params=None):
    t0 = time.perf_counter()
    r = conn.execute(query, parameters=params) if params else conn.execute(query)
    rows = []
    while r.has_next():
        rows.append(r.get_next())
    return (time.perf_counter() - t0) * 1000, rows

tot = q("MATCH (p:Person) RETURN count(*)")[1][0][0]
tote = q("MATCH (e:Event) RETURN count(*)")[1][0][0]
print(f"population: {tot} persons, {tote} events\n")

print("(a) SATURATION — reachable set by hop depth")
print(f"{'depth':>6} {'reached':>9} {'% of graph':>11} {'latency':>11}")
print("-" * 42)
for d in range(1, 5):
    ms, rows = q(f"MATCH (p:Person)-[e:attends*1..{d}]-(x) WHERE p.id=7 "
                 f"RETURN count(DISTINCT x)", None)
    n = rows[0][0]
    print(f"{d:>6} {n:>9,} {100*n/(tot+tote):>10.1f}% {ms:>9.0f}ms")

print("\n(b) TEMPORAL MONOTONICITY — can a hop reference the previous hop?")
tests = [
    ("constant-bound filter (all hops after T)",
     "MATCH (p:Person)-[e:attends*1..4 (r,n | WHERE r.start_ts >= 300000)]-(x) "
     "WHERE p.id=7 RETURN count(DISTINCT x)"),
    ("cross-hop: reference prev edge via alias",
     "MATCH (p:Person)-[e:attends*1..4 (r,n | WHERE r.start_ts >= e.start_ts)]-(x) "
     "WHERE p.id=7 RETURN count(DISTINCT x)"),
    ("cross-hop: ordered pairs over rels(path)",
     "MATCH path = (p:Person)-[e:attends*1..4]-(x) WHERE p.id=7 "
     "AND ALL(i IN range(1, size(rels(path))-1) WHERE "
     "    rels(path)[i].start_ts >= rels(path)[i-1].start_ts) "
     "RETURN count(DISTINCT x)"),
]
for label, query in tests:
    try:
        ms, rows = q(query)
        print(f"  [OK]   {label}\n         -> {rows[0][0]:,} reached in {ms:.0f}ms")
    except Exception as e:
        print(f"  [FAIL] {label}\n         -> {str(e).splitlines()[0][:110]}")

print("\n(c) BOUNDED ALTERNATIVE — 2-hop co-attendance with time window")
ms, rows = q("""
MATCH (p:Person)-[a1:attends]->(e:Event)<-[a2:attends]-(q:Person)
WHERE p.id=7 AND e.start_ts >= 300000 AND e.start_ts <= 600000
RETURN count(DISTINCT q)""")
print(f"  2-hop, windowed: {rows[0][0]:,} persons in {ms:.0f}ms")
conn.close(); db.close()
