"""
PHASE 1 PROBE — the stop-gate unknown.

Q1 (derived attends) needs: traverse subgroup_of* while filtering each hop on
temporal validity of that hop's edge.
Q7 (cascade) needs: traverse co-attendance while enforcing temporal monotonicity
across hops.

Both require PER-HOP edge property predicates inside a variable-length pattern.
If only whole-path post-filtering is available, both queries degrade badly.
"""
import os as _os, pathlib as _pl, sys as _sys
_HERE = _pl.Path(__file__).resolve().parent
WORK = _os.environ.get("ORRERY_WORK", str(_HERE / "_work"))
_os.makedirs(WORK, exist_ok=True)
_sys.path.insert(0, str(_HERE))

import ladybug, shutil, os, traceback

DB = f"{WORK}/probe1.lbdb"
shutil.rmtree(DB, ignore_errors=True)
os.path.exists(DB) and os.remove(DB)

db = ladybug.Database(DB)
conn = ladybug.Connection(db)

def run(label, q, expect_fail=False):
    try:
        r = conn.execute(q)
        rows = []
        while r.has_next():
            rows.append(r.get_next())
        print(f"  [OK]   {label}: {rows}")
        return rows
    except Exception as e:
        msg = str(e).split("\n")[0][:160]
        print(f"  [FAIL] {label}: {msg}")
        return None

print("=" * 78)
print("SETUP: group hierarchy with temporally-bounded membership edges")
print("=" * 78)

conn.execute("CREATE NODE TABLE Grp(id STRING, name STRING, PRIMARY KEY(id))")
conn.execute("CREATE NODE TABLE Person(id STRING, name STRING, PRIMARY KEY(id))")
# subgroup_of carries its own validity window -> per-hop filtering required
conn.execute("""CREATE REL TABLE subgroup_of(FROM Grp TO Grp,
                  valid_from INT64, valid_to INT64)""")
conn.execute("""CREATE REL TABLE member_of(FROM Person TO Grp,
                  valid_from INT64, valid_to INT64, role STRING)""")

for gid, name in [("g1","section-A"),("g2","cohort-26"),("g3","ECE"),("g4","SEAS"),("g5","defunct")]:
    conn.execute(f"CREATE (:Grp {{id:'{gid}', name:'{name}'}})")
conn.execute("CREATE (:Person {id:'p1', name:'adam'})")

# chain: g1 -> g2 -> g3 -> g4   (all valid in window 100..200)
# g2 -> g5 edge EXPIRED (valid_to = 50) -> must be excluded mid-traversal
edges = [("g1","g2",0,999),("g2","g3",0,999),("g3","g4",0,999),("g2","g5",0,50)]
for a,b,vf,vt in edges:
    conn.execute(f"""MATCH (a:Grp),(b:Grp) WHERE a.id='{a}' AND b.id='{b}'
                     CREATE (a)-[:subgroup_of {{valid_from:{vf}, valid_to:{vt}}}]->(b)""")
conn.execute("""MATCH (p:Person),(g:Grp) WHERE p.id='p1' AND g.id='g1'
                CREATE (p)-[:member_of {valid_from:0, valid_to:999, role:'member'}]->(g)""")

print("\nGround truth @ t=150: p1 -> g1 -> {g2, g3, g4}. g5 MUST be excluded.")

print("\n" + "=" * 78)
print("PROBE A: unfiltered variable-length baseline")
print("=" * 78)
run("var-length *1..5",
    "MATCH (p:Person)-[:member_of]->(:Grp)-[:subgroup_of*1..5]->(g:Grp) "
    "RETURN g.id ORDER BY g.id")

print("\n" + "=" * 78)
print("PROBE B: PER-HOP edge predicate  <-- THE STOP GATE")
print("=" * 78)
run("Kuzu (r,n | WHERE ...) syntax",
    "MATCH (p:Person)-[:member_of]->(:Grp)"
    "-[e:subgroup_of*1..5 (r, n | WHERE r.valid_from <= 150 AND r.valid_to >= 150)]->(g:Grp) "
    "RETURN g.id ORDER BY g.id")

run("inline WHERE in rel pattern",
    "MATCH (p:Person)-[:member_of]->(:Grp)"
    "-[e:subgroup_of*1..5 WHERE e.valid_to >= 150]->(g:Grp) "
    "RETURN g.id ORDER BY g.id")

print("\n" + "=" * 78)
print("PROBE C: whole-path post-filter (the fallback if B fails)")
print("=" * 78)
run("ALL(... IN rels(path))",
    "MATCH path = (p:Person)-[:member_of]->(:Grp)-[:subgroup_of*1..5]->(g:Grp) "
    "WHERE ALL(x IN rels(path) WHERE x.valid_from <= 150 AND x.valid_to >= 150) "
    "RETURN g.id ORDER BY g.id")

print("\n" + "=" * 78)
# Phase-0 correction (2026-08-01): this probe was mislabelled "temporal
# MONOTONICITY across hops" while only applying a CONSTANT per-hop filter —
# its own output includes the expired-edge target g5. The real cross-hop
# monotonicity tests live in probe_02_cascade.py (b) and FAIL, as
# RESEARCH-0002 records. Label fixed; behaviour unchanged.
print("PROBE D: constant per-hop filter + path length (NOT monotonicity —")
print("         see probe_02 (b) for the cross-hop tests, which fail)")
print("=" * 78)
run("constant filter with path length",
    "MATCH (p:Person)-[:member_of]->(:Grp)"
    "-[e:subgroup_of*1..5 (r, n | WHERE r.valid_from >= 0)]->(g:Grp) "
    "RETURN g.id, length(e) ORDER BY g.id")

print("\n" + "=" * 78)
print("PROBE E: path functions + shortest path")
print("=" * 78)
run("SHORTEST", "MATCH (a:Grp)-[e:subgroup_of* SHORTEST]->(b:Grp) "
                "WHERE a.id='g1' AND b.id='g4' RETURN length(e)")
run("ALL SHORTEST", "MATCH (a:Grp)-[e:subgroup_of* ALL SHORTEST]->(b:Grp) "
                    "WHERE a.id='g1' AND b.id='g4' RETURN length(e)")
run("WSHORTEST (weighted)",
    "MATCH (a:Grp)-[e:subgroup_of* WSHORTEST(valid_to)]->(b:Grp) "
    "WHERE a.id='g1' AND b.id='g4' RETURN length(e)")
