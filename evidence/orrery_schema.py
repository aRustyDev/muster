"""ORRERY — Ladybug schema (Phase 2) + tier-constraint verification."""

DDL = [
    # ---- Nodes -------------------------------------------------------------
    "CREATE NODE TABLE Person(id INT64, name STRING, PRIMARY KEY(id))",
    "CREATE NODE TABLE Grp(id INT64, name STRING, PRIMARY KEY(id))",
    """CREATE NODE TABLE Event(id INT64, name STRING, start_ts INT64, end_ts INT64,
                              kind STRING, PRIMARY KEY(id))""",
    # Location tiers as SEPARATE node tables -> tier rules become schema
    "CREATE NODE TABLE Room(id INT64, name STRING, capacity INT64, PRIMARY KEY(id))",
    """CREATE NODE TABLE Structure(id INT64, name STRING, lat DOUBLE, lon DOUBLE,
                                  portal STRING, PRIMARY KEY(id))""",
    "CREATE NODE TABLE Campus(id INT64, name STRING, PRIMARY KEY(id))",
    "CREATE NODE TABLE Violation(id INT64, kind STRING, severity STRING, "
    "detected_at INT64, resolved_at INT64, PRIMARY KEY(id))",

    # ---- Relations ---------------------------------------------------------
    "CREATE REL TABLE member_of(FROM Person TO Grp, valid_from INT64, valid_to INT64, role STRING)",
    "CREATE REL TABLE subgroup_of(FROM Grp TO Grp, valid_from INT64, valid_to INT64)",
    """CREATE REL TABLE expects(FROM Grp TO Event, obligation STRING, default_priority DOUBLE,
                               valid_from INT64, valid_to INT64, cascades BOOLEAN)""",
    """CREATE REL TABLE attends(FROM Person TO Event, start_ts INT64, end_ts INT64,
                               priority_person DOUBLE, priority_coord DOUBLE,
                               coord_binding BOOLEAN, source STRING, pinned BOOLEAN)""",
    """CREATE REL TABLE held(FROM Room TO Event, start_ts INT64, end_ts INT64,
                            posture STRING, overflow_for INT64, capacity_override INT64)""",
    # containment: strictly tier-ascending
    "CREATE REL TABLE within(FROM Room TO Structure, FROM Structure TO Campus)",
    # continuous travel: SIBLING TIERS ONLY (no Room->Structure pair declared)
    """CREATE REL TABLE traverse(FROM Room TO Room, FROM Structure TO Structure,
                                FROM Campus TO Campus,
                                mode STRING, duration_s INT64, distance_m DOUBLE,
                                provenance STRING, computed_at INT64)""",
    # scheduled travel: STRUCTURE TIER ONLY (portals) -> "no train between rooms"
    """CREATE REL TABLE transit(FROM Structure TO Structure, line STRING,
                               headway_s INT64, ride_s INT64,
                               service_from INT64, service_to INT64)""",
    "CREATE REL TABLE anchors(FROM Person TO Structure, label STRING, valid_from INT64, valid_to INT64)",
]


def build(conn):
    for stmt in DDL:
        conn.execute(stmt)


def verify_tier_constraints(conn):
    """The Part-2 claim: polymorphic FROM/TO makes illegal tier edges
    UNREPRESENTABLE rather than merely invalid. Verify empirically."""
    conn.execute("CREATE (:Room {id:9001, name:'probe-room-a', capacity:10})")
    conn.execute("CREATE (:Room {id:9002, name:'probe-room-b', capacity:10})")
    conn.execute("CREATE (:Structure {id:9101, name:'probe-bldg-a', lat:0.0, lon:0.0, portal:'rail'})")
    conn.execute("CREATE (:Structure {id:9102, name:'probe-bldg-b', lat:0.0, lon:0.0, portal:'rail'})")

    cases = [
        ("traverse Room->Room            (legal)", True,
         "MATCH (a:Room),(b:Room) WHERE a.id=9001 AND b.id=9002 "
         "CREATE (a)-[:traverse {mode:'walk',duration_s:60,distance_m:50.0,"
         "provenance:'measured',computed_at:0}]->(b)"),
        ("transit  Structure->Structure  (legal)", True,
         "MATCH (a:Structure),(b:Structure) WHERE a.id=9101 AND b.id=9102 "
         "CREATE (a)-[:transit {line:'red',headway_s:600,ride_s:900,"
         "service_from:0,service_to:999}]->(b)"),
        ("transit  Room->Room            (MUST REJECT)", False,
         "MATCH (a:Room),(b:Room) WHERE a.id=9001 AND b.id=9002 "
         "CREATE (a)-[:transit {line:'red',headway_s:600,ride_s:900,"
         "service_from:0,service_to:999}]->(b)"),
        ("traverse Room->Structure       (MUST REJECT)", False,
         "MATCH (a:Room),(b:Structure) WHERE a.id=9001 AND b.id=9101 "
         "CREATE (a)-[:traverse {mode:'walk',duration_s:60,distance_m:50.0,"
         "provenance:'measured',computed_at:0}]->(b)"),
        ("within   Structure->Room       (MUST REJECT: inverted tier)", False,
         "MATCH (a:Structure),(b:Room) WHERE a.id=9101 AND b.id=9001 "
         "CREATE (a)-[:within]->(b)"),
    ]

    results = []
    for label, should_succeed, q in cases:
        try:
            conn.execute(q)
            ok = should_succeed
            outcome = "accepted"
        except Exception:
            ok = not should_succeed
            outcome = "rejected"
        results.append((label, outcome, ok))
    return results
