"""ORRERY spike — Phases 3 & 4. Generates synthetic data, loads via COPY FROM,
runs the 7 canonical queries, reports latency."""
import os as _os, pathlib as _pl, sys as _sys
_HERE = _pl.Path(__file__).resolve().parent
WORK = _os.environ.get("ORRERY_WORK", str(_HERE / "_work"))
_os.makedirs(WORK, exist_ok=True)
_sys.path.insert(0, str(_HERE))

import ladybug, shutil, os, random, time, csv, sys, statistics
from orrery_schema import build, verify_tier_constraints

HOUR = 3600
DAY = 24 * HOUR
T_MID = 7 * DAY

SCALES = {
    "S": dict(persons=100,   events=200,   rooms=20,  structs=5,  campus=2, groups=10,  per_person=10),
    "M": dict(persons=2000,  events=2000,  rooms=200, structs=20, campus=3, groups=60,  per_person=50),
    "L": dict(persons=10000, events=10000, rooms=500, structs=40, campus=4, groups=200, per_person=100),
}


def w(path, rows):
    with open(path, "w", newline="") as f:
        csv.writer(f).writerows(rows)


def generate(cfg, d):
    rnd = random.Random(42)
    os.makedirs(d, exist_ok=True)
    P, E, R, S, C, G = (cfg[k] for k in ("persons", "events", "rooms", "structs", "campus", "groups"))

    w(f"{d}/person.csv",    [[i, f"person-{i}"] for i in range(P)])
    w(f"{d}/campus.csv",    [[i, f"campus-{i}"] for i in range(C)])
    w(f"{d}/structure.csv", [[i, f"bldg-{i}", 38.9 + i * 0.01, -77.3 + i * 0.01,
                              rnd.choice(["none", "pedestrian", "vehicle", "rail"])] for i in range(S)])
    w(f"{d}/room.csv",      [[i, f"room-{i}", rnd.choice([20, 40, 80, 150, 300])] for i in range(R)])
    w(f"{d}/group.csv",     [[i, f"group-{i}"] for i in range(G)])

    events = []
    for i in range(E):
        st = rnd.randrange(0, 14 * DAY, 1800)
        events.append([i, f"event-{i}", st, st + rnd.choice([HOUR, 2 * HOUR, 3 * HOUR]),
                       rnd.choice(["lecture", "talk", "lab", "seminar"])])
    w(f"{d}/event.csv", events)

    # containment
    room_of = {i: i % S for i in range(R)}
    w(f"{d}/within_rs.csv", [[i, room_of[i]] for i in range(R)])
    w(f"{d}/within_sc.csv", [[i, i % C] for i in range(S)])

    # traverse: sibling-tier only, directed (two rows per pair)
    trav = []
    by_struct = {}
    for r, s in room_of.items():
        by_struct.setdefault(s, []).append(r)
    for s, rooms in by_struct.items():
        for a in rooms:
            for b in rooms:
                if a != b:
                    dur = rnd.randrange(60, 400)
                    trav.append([a, b, "walk", dur, dur * 1.2, "estimated", 0])
    w(f"{d}/traverse_rr.csv", trav)
    tss = []
    for a in range(S):
        for b in range(S):
            if a != b:
                dur = rnd.randrange(300, 2400)
                tss.append([a, b, "walk", dur, dur * 1.3, "estimated", 0])
    w(f"{d}/traverse_ss.csv", tss)

    # held: every event in a room; 8% get an overflow room
    held = []
    for i in range(E):
        r = rnd.randrange(R)
        held.append([r, i, events[i][2], events[i][3], "on-site", -1, -1])
        if rnd.random() < 0.08:
            held.append([(r + 1) % R, i, events[i][2], events[i][3], "on-site", r, -1])
    w(f"{d}/held.csv", held)

    # group hierarchy (tree, depth ~4) + membership
    sub = [[g, (g - 1) // 3, 0, 10 ** 9] for g in range(1, G)]
    w(f"{d}/subgroup_of.csv", sub)
    mem = []
    for p in range(P):
        for g in rnd.sample(range(G), rnd.randrange(1, 4)):
            expired = rnd.random() < 0.1
            mem.append([p, g, 0, T_MID - DAY if expired else 10 ** 9, "member"])
    w(f"{d}/member_of.csv", mem)

    exp = []
    for g in range(G):
        for e in rnd.sample(range(E), min(E, rnd.randrange(2, 8))):
            exp.append([g, e, rnd.choice(["mandatory", "expected", "recommended"]),
                        round(rnd.uniform(0.2, 1.0), 2), 0, 10 ** 9, "true"])
    w(f"{d}/expects.csv", exp)

    # attends — deliberately seeded with conflicts and travel violations
    att = []
    for p in range(P):
        for e in rnd.sample(range(E), min(E, cfg["per_person"])):
            att.append([p, e, events[e][2], events[e][3],
                        round(rnd.uniform(-1, 1), 2), round(rnd.uniform(-1, 1), 2),
                        "false", rnd.choice(["self", "group", "coordinator"]), "false"])
    w(f"{d}/attends.csv", att)

    return dict(person=P, event=E, room=R, attends=len(att), held=len(held),
                member_of=len(mem), expects=len(exp), traverse=len(trav) + len(tss))


COPIES = [
    ("Person", "person.csv", ""), ("Campus", "campus.csv", ""),
    ("Structure", "structure.csv", ""), ("Room", "room.csv", ""),
    ("Grp", "group.csv", ""), ("Event", "event.csv", ""),
    ("within", "within_rs.csv", "(from='Room', to='Structure')"),
    ("within", "within_sc.csv", "(from='Structure', to='Campus')"),
    ("traverse", "traverse_rr.csv", "(from='Room', to='Room')"),
    ("traverse", "traverse_ss.csv", "(from='Structure', to='Structure')"),
    ("held", "held.csv", ""), ("subgroup_of", "subgroup_of.csv", ""),
    ("member_of", "member_of.csv", ""), ("expects", "expects.csv", ""),
    ("attends", "attends.csv", ""),
]

# ---------------------------------------------------------------- the 7 queries
# Phase-0 correction (2026-08-01, phases/00-grounding.md): *0..5, not *1..5.
# The handoff shipped *1..5, which excludes expectations on the person's
# DIRECT groups (depth 0) — the domain-wrong semantics per ADR-0003/0004, and
# the source of the 44-vs-58 result mismatch against SQLite. *0..5 parses and
# runs on ladybug 0.19.0 despite RESEARCH-0002's contrary claim (see its
# addendum); both engines now return identical results (58 @ L, person 7).
Q1_DERIVED = """
MATCH (p:Person)-[m:member_of]->(g0:Grp)
WHERE p.id = $pid AND m.valid_from <= $t AND m.valid_to >= $t
MATCH (g0)-[r:subgroup_of*0..5 (rel, n | WHERE rel.valid_from <= $t AND rel.valid_to >= $t)]->(g:Grp)
MATCH (g)-[x:expects]->(e:Event) WHERE x.valid_from <= $t AND x.valid_to >= $t
RETURN count(DISTINCT e.id)
"""

Q2_PERSON_CONFLICT = """
MATCH (p:Person)-[a1:attends]->(e1:Event), (p)-[a2:attends]->(e2:Event)
WHERE p.id = $pid AND e1.id < e2.id
  AND a1.start_ts < a2.end_ts AND a2.start_ts < a1.end_ts
RETURN count(*)
"""

Q3_GLOBAL_CONFLICT = """
MATCH (p:Person)-[a1:attends]->(e1:Event), (p)-[a2:attends]->(e2:Event)
WHERE e1.id < e2.id AND a1.start_ts < a2.end_ts AND a2.start_ts < a1.end_ts
RETURN count(*)
"""

Q4_TRAVEL = """
MATCH (p:Person)-[a1:attends]->(e1:Event)<-[:held]-(r1:Room),
      (p)-[a2:attends]->(e2:Event)<-[:held]-(r2:Room)
WHERE p.id = $pid AND a1.end_ts <= a2.start_ts AND r1.id <> r2.id
MATCH (r1)-[t:traverse]->(r2)
WHERE a2.start_ts - a1.end_ts < t.duration_s
RETURN count(*)
"""

Q5_EXCLUSIVITY = """
MATCH (r:Room)-[h1:held]->(e1:Event), (r)-[h2:held]->(e2:Event)
WHERE e1.id < e2.id AND h1.start_ts < h2.end_ts AND h2.start_ts < h1.end_ts
  AND h2.overflow_for <> r.id
RETURN count(*)
"""

Q6_PATH = """
MATCH (a:Room)-[e:traverse* SHORTEST 1..4]->(b:Room)
WHERE a.id = 0 AND b.id = $target
RETURN length(e)
"""

Q6B_DECOMP = """
MATCH (a:Room)-[:within]->(sa:Structure), (b:Room)-[:within]->(sb:Structure)
WHERE a.id = 0 AND b.id = $target
MATCH (sa)-[t:traverse]->(sb)
RETURN min(t.duration_s)
"""

Q7_CASCADE = """
MATCH (p:Person)-[e:attends*1..3]-(reached)
WHERE p.id = $pid
RETURN count(DISTINCT reached)
"""

Q7B_BOUNDED = """
MATCH (p:Person)-[a1:attends]->(e:Event)<-[a2:attends]-(q:Person)
WHERE p.id = $pid AND e.start_ts >= 300000 AND e.start_ts <= 600000
RETURN count(DISTINCT q.id)
"""


def bench(conn, q, params, n=3):
    out, last = [], None
    for _ in range(n):
        t0 = time.perf_counter()
        res = conn.execute(q, parameters=params) if params else conn.execute(q)
        rows = []
        while res.has_next():
            rows.append(res.get_next())
        out.append((time.perf_counter() - t0) * 1000)
        last = rows
    return statistics.median(out), max(out), last


def run_scale(name):
    cfg = SCALES[name]
    d = f"{WORK}/data_{name}"
    dbp = f"{WORK}/orrery_{name}.lbdb"
    shutil.rmtree(dbp, ignore_errors=True)
    for x in (dbp, dbp + ".wal"):
        os.path.isfile(x) and os.remove(x)

    print(f"\n{'='*78}\nSCALE {name}  {cfg}\n{'='*78}")
    t0 = time.perf_counter()
    counts = generate(cfg, d)
    print(f"generate: {time.perf_counter()-t0:.1f}s  {counts}")

    db = ladybug.Database(dbp)
    conn = ladybug.Connection(db)
    build(conn)

    t0 = time.perf_counter()
    for table, fn, opt in COPIES:
        conn.execute(f"COPY {table} FROM '{d}/{fn}' {opt}")
    load = time.perf_counter() - t0
    size = sum(os.path.getsize(os.path.join(dp, f))
               for dp, _, fs in os.walk(dbp) for f in fs) / 1e6 if os.path.isdir(dbp) \
        else os.path.getsize(dbp) / 1e6
    print(f"\nload: {load:.1f}s   on-disk: {size:.0f} MB   "
          f"({counts['attends']:,} attends edges)")

    pid = 7
    plan = [
        ("Q1 derived expansion  (per-person)", Q1_DERIVED,        {"pid": pid, "t": T_MID}),
        ("Q2 conflict detect    (per-person)", Q2_PERSON_CONFLICT, {"pid": pid}),
        ("Q3 conflict sweep     (GLOBAL)",     Q3_GLOBAL_CONFLICT, None),
        ("Q4 impossible travel  (per-person)", Q4_TRAVEL,          {"pid": pid}),
        ("Q5 room exclusivity   (GLOBAL)",     Q5_EXCLUSIVITY,     None),
        ("Q6 travel path        (shortest)",   Q6_PATH,            {"target": cfg["structs"]}),
        ("Q6b cross-bldg decomposed",          Q6B_DECOMP,        {"target": cfg["rooms"] - 1}),
        ("Q7 cascade 3-hop      (per-person)", Q7_CASCADE,         {"pid": pid}),
        ("Q7b co-attend 2-hop   (windowed)",   Q7B_BOUNDED,        {"pid": pid}),
    ]
    print(f"\n{'query':38s} {'median':>10s} {'max':>10s}   result")
    print("-" * 78)
    for label, q, params in plan:
        try:
            med, mx, rows = bench(conn, q, params)
            r = str(rows[0][0]) if rows else "-"
            print(f"{label:38s} {med:8.1f}ms {mx:8.1f}ms   {r}")
        except Exception as e:
            print(f"{label:38s} {'ERROR':>10s}            {str(e).splitlines()[0][:60]}")
    conn.close(); db.close()


def tier_check():
    p = f"{WORK}/tier.lbdb"
    shutil.rmtree(p, ignore_errors=True)
    for x in (p, p + ".wal"):
        os.path.isfile(x) and os.remove(x)
    d2 = ladybug.Database(p); c2 = ladybug.Connection(d2)
    build(c2)
    print("-- tier constraint verification (isolated db) --")
    for label, outcome, ok in verify_tier_constraints(c2):
        print(f"  {'PASS' if ok else 'FAIL'}  {label:46s} -> {outcome}")
    c2.close(); d2.close()


if __name__ == "__main__":
    tier_check()
    for s in sys.argv[1:] or ["S"]:
        run_scale(s)
