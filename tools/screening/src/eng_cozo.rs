//! Cozo 0.7.x candidate — `mem` storage engine (single writer, multi reader;
//! the harness is single-threaded so the restriction never binds).
//! Queries are CozoScript Datalog; recursion is native, so Q1's bounded depth
//! is expressed with an explicit depth counter (d <= 5), depth 0 = the direct
//! groups (the Phase-0 `*0..5` correction).

use crate::data::{Dataset, KINDS, OBLIGATIONS, SOURCES};
use crate::engine::Engine;
use anyhow::{anyhow, bail, Result};
use cozo::{DataValue, DbInstance, NamedRows, ScriptMutability};
use std::collections::BTreeMap;

pub struct CozoEngine {
    db: DbInstance,
}

// Key-prefix forms: the bound value sits in the key-column position so the
// stored-relation (or index) prefix is used, mirroring "any competent
// repository indexes its lookups" (Phase-0 ix_att_e lesson). The reverse
// lookup in Q7b explicitly uses the attends:by_event index created at load.
const Q1_RULES: &str = "
g0[g] := *member_of{person: $pid, grp: g, valid_from: vf, valid_to: vt}, vf <= $t, vt >= $t
anc[g, d] := g0[g], d = 0
anc[gp, d] := anc[gc, d0], d0 < 5, *subgroup_of{child: gc, parent: gp, valid_from: vf, valid_to: vt}, vf <= $t, vt >= $t, d = d0 + 1
evts[e] := anc[g, dd], *expects{grp: g, event: e, valid_from: vf, valid_to: vt}, vf <= $t, vt >= $t
";

const Q2_RULES: &str = "
pr[e1, e2] := *attends{person: $pid, event: e1, start_ts: s1, end_ts: t1}, \
              *attends{person: $pid, event: e2, start_ts: s2, end_ts: t2}, \
              e1 < e2, s1 < t2, s2 < t1
";

const Q7B_RULES: &str = "
we[e] := *attends{person: $pid, event: e}, *event{id: e, start_ts: s}, s >= $lo, s <= $hi
qs[q] := we[e], *attends:by_event{event: e, person: q}, q != $pid
";

impl CozoEngine {
    pub fn new() -> Result<Self> {
        let db = DbInstance::new("mem", "", "")
            .map_err(|e| anyhow!("cozo new(mem): {e:?}"))?;
        Ok(Self { db })
    }

    fn run(
        &self,
        script: &str,
        params: BTreeMap<String, DataValue>,
        mutability: ScriptMutability,
    ) -> Result<NamedRows> {
        self.db
            .run_script(script, params, mutability)
            .map_err(|e| anyhow!("cozo: {e:?}\nscript: {script}"))
    }

    fn put(&self, script: &str, rows: Vec<DataValue>) -> Result<()> {
        let mut params = BTreeMap::new();
        params.insert("rows".to_string(), DataValue::List(rows));
        self.run(script, params, ScriptMutability::Mutable)?;
        Ok(())
    }

    fn params_pt(pid: i64, t: i64) -> BTreeMap<String, DataValue> {
        BTreeMap::from([
            ("pid".to_string(), DataValue::from(pid)),
            ("t".to_string(), DataValue::from(t)),
        ])
    }

    fn params_window(pid: i64, lo: i64, hi: i64) -> BTreeMap<String, DataValue> {
        BTreeMap::from([
            ("pid".to_string(), DataValue::from(pid)),
            ("lo".to_string(), DataValue::from(lo)),
            ("hi".to_string(), DataValue::from(hi)),
        ])
    }
}

fn dv_i64(v: &DataValue) -> Result<i64> {
    v.get_int().ok_or_else(|| anyhow!("cozo: non-int value {v:?}"))
}

impl Engine for CozoEngine {
    fn name(&self) -> &'static str {
        "cozo"
    }

    fn load(&mut self, ds: &Dataset) -> Result<()> {
        for create in [
            ":create person {id: Int => name: String}",
            ":create grp {id: Int => name: String}",
            ":create event {id: Int => name: String, start_ts: Int, end_ts: Int, kind: String}",
            ":create member_of {person: Int, grp: Int => valid_from: Int, valid_to: Int, role: String}",
            ":create subgroup_of {child: Int, parent: Int => valid_from: Int, valid_to: Int}",
            ":create expects {grp: Int, event: Int => obligation: String, default_priority: Float, valid_from: Int, valid_to: Int, cascades: Bool}",
            ":create attends {person: Int, event: Int => start_ts: Int, end_ts: Int, priority_person: Float, priority_coord: Float, coord_binding: Bool, source: String, pinned: Bool}",
        ] {
            self.run(create, BTreeMap::new(), ScriptMutability::Mutable)?;
        }
        // reverse-adjacency index for Q7b (graph stores get this structurally;
        // relational representation needs it — Phase-0 ix_att_e parity)
        self.run(
            "::index create attends:by_event {event, person}",
            BTreeMap::new(),
            ScriptMutability::Mutable,
        )?;

        let lst = |vals: Vec<DataValue>| DataValue::List(vals);

        let persons: Vec<DataValue> = (0..ds.persons)
            .map(|i| lst(vec![i.into(), format!("person-{i}").into()]))
            .collect();
        self.put("?[id, name] <- $rows\n:put person {id => name}", persons)?;

        let groups: Vec<DataValue> = (0..ds.groups)
            .map(|i| lst(vec![i.into(), format!("group-{i}").into()]))
            .collect();
        self.put("?[id, name] <- $rows\n:put grp {id => name}", groups)?;

        let events: Vec<DataValue> = ds
            .events
            .iter()
            .map(|e| {
                lst(vec![
                    e.id.into(),
                    format!("event-{}", e.id).into(),
                    e.start_ts.into(),
                    e.end_ts.into(),
                    KINDS[e.kind as usize].into(),
                ])
            })
            .collect();
        self.put(
            "?[id, name, start_ts, end_ts, kind] <- $rows\n:put event {id => name, start_ts, end_ts, kind}",
            events,
        )?;

        let members: Vec<DataValue> = ds
            .member_of
            .iter()
            .map(|m| {
                lst(vec![
                    m.person.into(),
                    m.group.into(),
                    m.valid_from.into(),
                    m.valid_to.into(),
                    "member".into(),
                ])
            })
            .collect();
        self.put(
            "?[person, grp, valid_from, valid_to, role] <- $rows\n:put member_of {person, grp => valid_from, valid_to, role}",
            members,
        )?;

        let subs: Vec<DataValue> = ds
            .subgroup_of
            .iter()
            .map(|s| lst(vec![s.child.into(), s.parent.into(), s.valid_from.into(), s.valid_to.into()]))
            .collect();
        self.put(
            "?[child, parent, valid_from, valid_to] <- $rows\n:put subgroup_of {child, parent => valid_from, valid_to}",
            subs,
        )?;

        let exps: Vec<DataValue> = ds
            .expects
            .iter()
            .map(|x| {
                lst(vec![
                    x.group.into(),
                    x.event.into(),
                    OBLIGATIONS[x.obligation as usize].into(),
                    x.default_priority.into(),
                    x.valid_from.into(),
                    x.valid_to.into(),
                    true.into(),
                ])
            })
            .collect();
        self.put(
            "?[grp, event, obligation, default_priority, valid_from, valid_to, cascades] <- $rows\n\
             :put expects {grp, event => obligation, default_priority, valid_from, valid_to, cascades}",
            exps,
        )?;

        // attends in chunks (largest relation)
        let att_script = "?[person, event, start_ts, end_ts, priority_person, priority_coord, coord_binding, source, pinned] <- $rows\n\
             :put attends {person, event => start_ts, end_ts, priority_person, priority_coord, coord_binding, source, pinned}";
        for chunk in ds.attends.chunks(25_000) {
            let rows: Vec<DataValue> = chunk
                .iter()
                .map(|a| {
                    lst(vec![
                        a.person.into(),
                        a.event.into(),
                        a.start_ts.into(),
                        a.end_ts.into(),
                        a.priority_person.into(),
                        a.priority_coord.into(),
                        false.into(),
                        SOURCES[a.source as usize].into(),
                        false.into(),
                    ])
                })
                .collect();
            self.put(att_script, rows)?;
        }
        Ok(())
    }

    fn crud_smoke(&mut self) -> Result<()> {
        self.run(":create crud {k: Int => v: String}", BTreeMap::new(), ScriptMutability::Mutable)?;
        self.put("?[k, v] <- $rows\n:put crud {k => v}", vec![DataValue::List(vec![1.into(), "alpha".into()])])?;
        let read = |me: &Self| -> Result<Vec<String>> {
            let r = me.run(
                "?[v] := *crud{k, v}, k == 1",
                BTreeMap::new(),
                ScriptMutability::Immutable,
            )?;
            r.rows
                .iter()
                .map(|row| {
                    row[0]
                        .get_str()
                        .map(str::to_string)
                        .ok_or_else(|| anyhow!("cozo crud: non-str {row:?}"))
                })
                .collect()
        };
        if read(self)? != vec!["alpha".to_string()] {
            bail!("cozo crud: create/read mismatch");
        }
        self.put("?[k, v] <- $rows\n:put crud {k => v}", vec![DataValue::List(vec![1.into(), "beta".into()])])?;
        if read(self)? != vec!["beta".to_string()] {
            bail!("cozo crud: update/read mismatch");
        }
        self.run("?[k] <- [[1]]\n:rm crud {k}", BTreeMap::new(), ScriptMutability::Mutable)?;
        if !read(self)?.is_empty() {
            bail!("cozo crud: row visible after delete");
        }
        Ok(())
    }

    fn q1(&self, pid: i64, t: i64) -> Result<Vec<i64>> {
        let script = format!("{Q1_RULES}\n?[e] := evts[e]");
        let r = self.run(&script, Self::params_pt(pid, t), ScriptMutability::Immutable)?;
        r.rows.iter().map(|row| dv_i64(&row[0])).collect()
    }

    fn q1_count(&self, pid: i64, t: i64) -> Result<i64> {
        let script = format!("{Q1_RULES}\n?[count(e)] := evts[e]");
        let r = self.run(&script, Self::params_pt(pid, t), ScriptMutability::Immutable)?;
        dv_i64(&r.rows[0][0])
    }

    fn q2(&self, pid: i64) -> Result<Vec<(i64, i64)>> {
        let script = format!("{Q2_RULES}\n?[e1, e2] := pr[e1, e2]");
        let params = BTreeMap::from([("pid".to_string(), DataValue::from(pid))]);
        let r = self.run(&script, params, ScriptMutability::Immutable)?;
        r.rows
            .iter()
            .map(|row| Ok((dv_i64(&row[0])?, dv_i64(&row[1])?)))
            .collect()
    }

    fn q2_count(&self, pid: i64) -> Result<i64> {
        let script = format!("{Q2_RULES}\n?[count(pair)] := pr[e1, e2], pair = [e1, e2]");
        let params = BTreeMap::from([("pid".to_string(), DataValue::from(pid))]);
        let r = self.run(&script, params, ScriptMutability::Immutable)?;
        dv_i64(&r.rows[0][0])
    }

    fn q7b(&self, pid: i64, lo: i64, hi: i64) -> Result<Vec<i64>> {
        let script = format!("{Q7B_RULES}\n?[q] := qs[q]");
        let r = self.run(&script, Self::params_window(pid, lo, hi), ScriptMutability::Immutable)?;
        r.rows.iter().map(|row| dv_i64(&row[0])).collect()
    }

    fn q7b_count(&self, pid: i64, lo: i64, hi: i64) -> Result<i64> {
        let script = format!("{Q7B_RULES}\n?[count(q)] := qs[q]");
        let r = self.run(&script, Self::params_window(pid, lo, hi), ScriptMutability::Immutable)?;
        dv_i64(&r.rows[0][0])
    }

    fn notes(&self) -> String {
        "mem engine; q1 depth bound via explicit counter d<=5; \
         attends:by_event index used explicitly in q7b (ix_att_e parity)"
            .into()
    }
}
