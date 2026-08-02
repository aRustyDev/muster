//! agdb 0.13.x candidate. In-memory storage (`DbMemory`).
//!
//! agdb has no text query language: queries are QueryBuilder searches with
//! per-element conditions; joins the builder cannot express (Q2's pair
//! overlap; Q7b's per-event fan-in union) are completed in Rust — that is the
//! store's documented usage shape, and the timings include that Rust work.
//!
//! Distance semantics (Phase 01a carry-forward): Distance counts nodes AND
//! edges. From a person origin: member_of edge = 1, direct group = 2, each
//! subgroup hop adds 2 (edge+node), expects edge + event adds 2. Subgroup
//! depth <= 5 therefore = distance <= 14 to the event.

use crate::data::{Dataset, SOURCES};
use crate::engine::Engine;
use anyhow::{anyhow, bail, Result};
use agdb::{
    Comparison, CountComparison, DbId, DbKeyValue, DbMemory, DbValue, QueryBuilder,
};
use std::collections::HashSet;

pub struct AgdbEngine {
    db: DbMemory,
    person_dbid: Vec<DbId>,
    event_dbid: Vec<DbId>,
}

impl AgdbEngine {
    pub fn new() -> Result<Self> {
        Ok(Self {
            db: DbMemory::new("screening").map_err(|e| anyhow!("agdb new: {e}"))?,
            person_dbid: Vec::new(),
            event_dbid: Vec::new(),
        })
    }

    fn insert_nodes(&mut self, values: Vec<Vec<DbKeyValue>>) -> Result<Vec<DbId>> {
        let mut ids = Vec::with_capacity(values.len());
        for chunk in values.chunks(10_000) {
            let r = self
                .db
                .exec_mut(QueryBuilder::insert().nodes().values(chunk.to_vec()).query())
                .map_err(|e| anyhow!("agdb insert nodes: {e}"))?;
            ids.extend(r.ids());
        }
        Ok(ids)
    }

    fn insert_edges(
        &mut self,
        from: &[DbId],
        to: &[DbId],
        values: Vec<Vec<DbKeyValue>>,
    ) -> Result<()> {
        assert_eq!(from.len(), to.len());
        assert_eq!(from.len(), values.len());
        let chunk = 10_000;
        for i in (0..from.len()).step_by(chunk) {
            let end = (i + chunk).min(from.len());
            self.db
                .exec_mut(
                    QueryBuilder::insert()
                        .edges()
                        .from(from[i..end].to_vec())
                        .to(to[i..end].to_vec())
                        .values(values[i..end].to_vec())
                        .query(),
                )
                .map_err(|e| anyhow!("agdb insert edges: {e}"))?;
        }
        Ok(())
    }

    /// Search returning p's derived-expansion event nodes (selection: nodes
    /// with a start_ts key within distance 14; traversal: only through
    /// temporally-valid edges bearing valid_from/valid_to — attends edges
    /// have neither key, so the window test fails and they are not crossed).
    fn q1_search(&self, pid: i64, t: i64) -> agdb::SearchQuery {
        QueryBuilder::search()
            .from(self.person_dbid[pid as usize])
            .where_()
            .distance(CountComparison::LessThanOrEqual(14))
            .and()
            .node()
            .and()
            .keys(vec![DbValue::from("start_ts")])
            .and()
            .beyond()
            .where_()
            .node()
            .or()
            .where_()
            .key("valid_from")
            .value(Comparison::LessThanOrEqual(t.into()))
            .and()
            .key("valid_to")
            .value(Comparison::GreaterThanOrEqual(t.into()))
            .end_where()
            .end_where()
            .query()
    }

    /// p's attends edges: (event ext id, start_ts, end_ts) per edge.
    fn attends_of(&self, pid: i64) -> Result<Vec<(i64, i64, i64)>> {
        let search = QueryBuilder::search()
            .from(self.person_dbid[pid as usize])
            .where_()
            .distance(CountComparison::LessThanOrEqual(1))
            .and()
            .edge()
            .and()
            .keys(vec![DbValue::from("start_ts")])
            .query();
        let q = QueryBuilder::select()
            .values(vec![DbValue::from("event"), DbValue::from("start_ts"), DbValue::from("end_ts")])
            .ids(search)
            .query();
        let r = self.db.exec(q).map_err(|e| anyhow!("agdb attends_of: {e}"))?;
        r.elements
            .iter()
            .map(|el| {
                let get = |i: usize| -> Result<i64> {
                    match &el.values[i].value {
                        DbValue::I64(v) => Ok(*v),
                        other => bail!("agdb attends_of: non-i64 {other:?}"),
                    }
                };
                Ok((get(0)?, get(1)?, get(2)?))
            })
            .collect()
    }

    fn q2_pairs(&self, pid: i64) -> Result<Vec<(i64, i64)>> {
        let atts = self.attends_of(pid)?;
        let mut out = Vec::new();
        for i in 0..atts.len() {
            for j in 0..atts.len() {
                let (e1, s1, t1) = atts[i];
                let (e2, s2, t2) = atts[j];
                if e1 < e2 && s1 < t2 && s2 < t1 {
                    out.push((e1, e2));
                }
            }
        }
        Ok(out)
    }

    /// Distinct attendee DbIds (excluding pid) of pid's events whose event
    /// start_ts is in [lo, hi].
    fn q7b_dbids(&self, pid: i64, lo: i64, hi: i64) -> Result<HashSet<DbId>> {
        let p = self.person_dbid[pid as usize];
        let ev_search = QueryBuilder::search()
            .from(p)
            .where_()
            .distance(CountComparison::LessThanOrEqual(2))
            .and()
            .node()
            .and()
            .key("start_ts")
            .value(Comparison::GreaterThanOrEqual(lo.into()))
            .and()
            .key("start_ts")
            .value(Comparison::LessThanOrEqual(hi.into()))
            .query();
        let evs = self
            .db
            .exec(ev_search)
            .map_err(|e| anyhow!("agdb q7b events: {e}"))?
            .ids();
        let mut people: HashSet<DbId> = HashSet::new();
        for ev in evs {
            // reverse search from the event; traversal gated to attends edges
            // (they carry start_ts; expects edges do not)
            let s = QueryBuilder::search()
                .to(ev)
                .where_()
                .distance(CountComparison::Equal(2))
                .and()
                .node()
                .and()
                .not()
                .ids(p)
                .and()
                .beyond()
                .where_()
                .node()
                .or()
                .keys(vec![DbValue::from("start_ts")])
                .end_where()
                .query();
            people.extend(
                self.db
                    .exec(s)
                    .map_err(|e| anyhow!("agdb q7b attendees: {e}"))?
                    .ids(),
            );
        }
        Ok(people)
    }
}

fn kv(k: &str, v: impl Into<DbValue>) -> DbKeyValue {
    DbKeyValue { key: k.into(), value: v.into() }
}

impl Engine for AgdbEngine {
    fn name(&self) -> &'static str {
        "agdb"
    }

    fn load(&mut self, ds: &Dataset) -> Result<()> {
        let person_vals: Vec<Vec<DbKeyValue>> = (0..ds.persons)
            .map(|i| vec![kv("id", i), kv("name", format!("person-{i}"))])
            .collect();
        self.person_dbid = self.insert_nodes(person_vals)?;

        let group_vals: Vec<Vec<DbKeyValue>> = (0..ds.groups)
            .map(|i| vec![kv("id", i), kv("name", format!("group-{i}"))])
            .collect();
        let group_dbid = self.insert_nodes(group_vals)?;

        let event_vals: Vec<Vec<DbKeyValue>> = ds
            .events
            .iter()
            .map(|e| {
                vec![
                    kv("id", e.id),
                    kv("name", format!("event-{}", e.id)),
                    kv("start_ts", e.start_ts),
                    kv("end_ts", e.end_ts),
                    kv("kind", crate::data::KINDS[e.kind as usize]),
                ]
            })
            .collect();
        self.event_dbid = self.insert_nodes(event_vals)?;

        // member_of
        let (mut from, mut to, mut vals) = (Vec::new(), Vec::new(), Vec::new());
        for m in &ds.member_of {
            from.push(self.person_dbid[m.person as usize]);
            to.push(group_dbid[m.group as usize]);
            vals.push(vec![
                kv("valid_from", m.valid_from),
                kv("valid_to", m.valid_to),
                kv("role", "member"),
            ]);
        }
        self.insert_edges(&from, &to, vals)?;

        // subgroup_of
        let (mut from, mut to, mut vals) = (Vec::new(), Vec::new(), Vec::new());
        for s in &ds.subgroup_of {
            from.push(group_dbid[s.child as usize]);
            to.push(group_dbid[s.parent as usize]);
            vals.push(vec![kv("valid_from", s.valid_from), kv("valid_to", s.valid_to)]);
        }
        self.insert_edges(&from, &to, vals)?;

        // expects
        let (mut from, mut to, mut vals) = (Vec::new(), Vec::new(), Vec::new());
        for x in &ds.expects {
            from.push(group_dbid[x.group as usize]);
            to.push(self.event_dbid[x.event as usize]);
            vals.push(vec![
                kv("obligation", crate::data::OBLIGATIONS[x.obligation as usize]),
                kv("default_priority", x.default_priority),
                kv("valid_from", x.valid_from),
                kv("valid_to", x.valid_to),
                kv("cascades", true),
            ]);
        }
        self.insert_edges(&from, &to, vals)?;

        // attends (carries the event ext id so Q2 needs no endpoint decode)
        let (mut from, mut to, mut vals) = (Vec::new(), Vec::new(), Vec::new());
        for a in &ds.attends {
            from.push(self.person_dbid[a.person as usize]);
            to.push(self.event_dbid[a.event as usize]);
            vals.push(vec![
                kv("event", a.event),
                kv("start_ts", a.start_ts),
                kv("end_ts", a.end_ts),
                kv("priority_person", a.priority_person),
                kv("priority_coord", a.priority_coord),
                kv("coord_binding", false),
                kv("source", SOURCES[a.source as usize]),
                kv("pinned", false),
            ]);
        }
        self.insert_edges(&from, &to, vals)?;
        Ok(())
    }

    fn crud_smoke(&mut self) -> Result<()> {
        let r = self
            .db
            .exec_mut(
                QueryBuilder::insert()
                    .nodes()
                    .values(vec![vec![kv("pk", 1), kv("name", "alpha")]])
                    .query(),
            )
            .map_err(|e| anyhow!("agdb crud insert: {e}"))?;
        let id = r.ids()[0];
        let read = |db: &DbMemory| -> Result<String> {
            let r = db
                .exec(QueryBuilder::select().values(vec![DbValue::from("name")]).ids(id).query())
                .map_err(|e| anyhow!("agdb crud read: {e}"))?;
            match &r.elements[0].values[0].value {
                DbValue::String(s) => Ok(s.clone()),
                other => bail!("agdb crud read type: {other:?}"),
            }
        };
        if read(&self.db)? != "alpha" {
            bail!("agdb crud: create/read mismatch");
        }
        self.db
            .exec_mut(
                QueryBuilder::insert()
                    .values(vec![vec![kv("name", "beta")]])
                    .ids(id)
                    .query(),
            )
            .map_err(|e| anyhow!("agdb crud update: {e}"))?;
        if read(&self.db)? != "beta" {
            bail!("agdb crud: update/read mismatch");
        }
        self.db
            .exec_mut(QueryBuilder::remove().ids(id).query())
            .map_err(|e| anyhow!("agdb crud delete: {e}"))?;
        if self
            .db
            .exec(QueryBuilder::select().ids(id).query())
            .is_ok()
        {
            bail!("agdb crud: element readable after delete");
        }
        Ok(())
    }

    fn q1(&self, pid: i64, t: i64) -> Result<Vec<i64>> {
        let q = QueryBuilder::select()
            .values(vec![DbValue::from("id")])
            .ids(self.q1_search(pid, t))
            .query();
        let r = self.db.exec(q).map_err(|e| anyhow!("agdb q1: {e}"))?;
        r.elements
            .iter()
            .map(|el| match &el.values[0].value {
                DbValue::I64(v) => Ok(*v),
                other => bail!("agdb q1: non-i64 id {other:?}"),
            })
            .collect()
    }

    fn q1_count(&self, pid: i64, t: i64) -> Result<i64> {
        let r = self
            .db
            .exec(self.q1_search(pid, t))
            .map_err(|e| anyhow!("agdb q1_count: {e}"))?;
        Ok(r.result as i64)
    }

    fn q2(&self, pid: i64) -> Result<Vec<(i64, i64)>> {
        self.q2_pairs(pid)
    }

    fn q2_count(&self, pid: i64) -> Result<i64> {
        Ok(self.q2_pairs(pid)?.len() as i64)
    }

    fn q7b(&self, pid: i64, lo: i64, hi: i64) -> Result<Vec<i64>> {
        let dbids = self.q7b_dbids(pid, lo, hi)?;
        if dbids.is_empty() {
            return Ok(Vec::new());
        }
        let ids: Vec<DbId> = dbids.into_iter().collect();
        let q = QueryBuilder::select().values(vec![DbValue::from("id")]).ids(ids).query();
        let r = self.db.exec(q).map_err(|e| anyhow!("agdb q7b select: {e}"))?;
        r.elements
            .iter()
            .map(|el| match &el.values[0].value {
                DbValue::I64(v) => Ok(*v),
                other => bail!("agdb q7b: non-i64 id {other:?}"),
            })
            .collect()
    }

    fn q7b_count(&self, pid: i64, lo: i64, hi: i64) -> Result<i64> {
        Ok(self.q7b_dbids(pid, lo, hi)?.len() as i64)
    }

    fn notes(&self) -> String {
        "q2 pair-join and q7b per-event union computed in Rust (builder has no join surface); \
         q2 count-only ~= materialised by construction"
            .into()
    }
}
