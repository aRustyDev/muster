//! Grafeo 0.5.x candidate. In-memory (no `wal`). Queries via the GQL dialect
//! (per Phase 01a: the Cypher dialect lacks inline var-length predicates).
//!
//! Q1 syntax is probed once per process: the ideal single-statement form uses
//! a quantified parenthesized pattern with per-hop WHERE (G050) and a {0,5}
//! quantifier; grafeo's own test-suite shows no {0,n} quantifier and no
//! concatenated quantified pattern, so two fallbacks exist:
//!   UnionQuant  = depth-0 statement + {1,5} quantified statement, union in Rust
//!   FixedChains = 6 fixed-depth statements (depth 0..5), union in Rust
//! The form used is reported via `notes()`. Correctness of whichever form wins
//! is enforced by the micro-fixture and the cross-engine differential check.

use crate::data::{AttendRow, Dataset, EventRow, ExpectRow, MemberRow, SubgroupRow, KINDS, OBLIGATIONS, SOURCES};
use crate::engine::Engine;
use anyhow::{anyhow, bail, Result};
use grafeo::{GrafeoDB, Value};
use std::collections::BTreeSet;
use std::sync::OnceLock;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Q1Form {
    Composite,
    UnionQuant,
    FixedChains,
}

static Q1_FORM: OnceLock<Q1Form> = OnceLock::new();
/// Why the better forms were rejected (finding for the phase doc).
static Q1_PROBE_LOG: OnceLock<String> = OnceLock::new();

pub struct GrafeoEngine {
    db: GrafeoDB,
}

impl GrafeoEngine {
    pub fn new() -> Result<Self> {
        Ok(Self { db: GrafeoDB::new_in_memory() })
    }

    fn exec(&self, q: &str) -> Result<grafeo::QueryResult> {
        self.db
            .execute(q)
            .map_err(|e| anyhow!("grafeo: {e} (query: {q})"))
    }

    fn ids_from(&self, q: &str) -> Result<Vec<i64>> {
        let r = self.exec(q)?;
        r.rows()
            .iter()
            .map(|row| match &row[0] {
                Value::Int64(v) => Ok(*v),
                other => bail!("grafeo: expected Int64, got {other:?}"),
            })
            .collect()
    }

    fn scalar_i64(&self, q: &str) -> Result<i64> {
        let r = self.exec(q)?;
        match r.rows().first().and_then(|row| row.first()) {
            Some(Value::Int64(v)) => Ok(*v),
            other => bail!("grafeo: expected scalar Int64, got {other:?}"),
        }
    }

    /// Raw pair query — used by the debug-q2 reproduction in main.rs.
    pub fn raw_pairs(&self, q: &str) -> Result<Vec<(i64, i64)>> {
        let r = self.exec(q)?;
        r.rows()
            .iter()
            .map(|row| match (&row[0], &row[1]) {
                (Value::Int64(a), Value::Int64(b)) => Ok((*a, *b)),
                other => bail!("grafeo raw_pairs: non-int row {other:?}"),
            })
            .collect()
    }

    fn q1_stmt_fixed(depth: usize, pid: i64, t: i64) -> String {
        let mut pat = format!("MATCH (p:Person {{id: {pid}}})-[m:member_of]->(g0:Grp)");
        let mut wher = format!(
            "WHERE m.valid_from <= {t} AND m.valid_to >= {t}"
        );
        for d in 1..=depth {
            pat.push_str(&format!("-[r{d}:subgroup_of]->(g{d}:Grp)"));
            wher.push_str(&format!(
                " AND r{d}.valid_from <= {t} AND r{d}.valid_to >= {t}"
            ));
        }
        pat.push_str(&format!("-[x:expects]->(e:Event)"));
        wher.push_str(&format!(" AND x.valid_from <= {t} AND x.valid_to >= {t}"));
        format!("{pat} {wher} RETURN DISTINCT e.id")
    }

    fn q1_stmt_quant(lo: usize, pid: i64, t: i64, count: bool) -> String {
        let ret = if count { "RETURN count(DISTINCT e.id)" } else { "RETURN DISTINCT e.id" };
        format!(
            "MATCH (p:Person {{id: {pid}}})-[m:member_of]->(g0:Grp)\
             ((qa:Grp)-[qr:subgroup_of]->(qb:Grp) WHERE qr.valid_from <= {t} AND qr.valid_to >= {t}){{{lo},5}}\
             (g:Grp)-[x:expects]->(e:Event) \
             WHERE m.valid_from <= {t} AND m.valid_to >= {t} \
             AND x.valid_from <= {t} AND x.valid_to >= {t} {ret}"
        )
    }

    fn q1_fixed_chains(&self, pid: i64, t: i64) -> Result<Vec<i64>> {
        let mut out: BTreeSet<i64> = BTreeSet::new();
        for d in 0..=5 {
            out.extend(self.ids_from(&Self::q1_stmt_fixed(d, pid, t))?);
        }
        Ok(out.into_iter().collect())
    }

    fn q1_union_quant(&self, pid: i64, t: i64) -> Result<Vec<i64>> {
        let mut out: BTreeSet<i64> =
            self.ids_from(&Self::q1_stmt_fixed(0, pid, t))?.into_iter().collect();
        out.extend(self.ids_from(&Self::q1_stmt_quant(1, pid, t, false))?);
        Ok(out.into_iter().collect())
    }

    fn q1_composite(&self, pid: i64, t: i64) -> Result<Vec<i64>> {
        let mut out: BTreeSet<i64> =
            self.ids_from(&Self::q1_stmt_quant(0, pid, t, false))?.into_iter().collect();
        Ok(std::mem::take(&mut out).into_iter().collect())
    }

    /// Decide which Q1 form this grafeo build actually supports, comparing
    /// against the FixedChains baseline on the currently-loaded data. Runs
    /// once per process (first q1 call — main calls it first on the
    /// micro-fixture, which contains expired edges, so temporal pruning is
    /// part of the comparison).
    fn q1_form(&self, pid: i64, t: i64) -> Result<Q1Form> {
        if let Some(f) = Q1_FORM.get() {
            return Ok(*f);
        }
        let baseline: BTreeSet<i64> = self.q1_fixed_chains(pid, t)?.into_iter().collect();
        let mut log = String::new();
        let form = match self.q1_composite(pid, t) {
            Ok(v) if v.iter().copied().collect::<BTreeSet<i64>>() == baseline => Q1Form::Composite,
            Ok(v) => {
                log.push_str(&format!(
                    "composite {{0,5}} parsed but WRONG: got {:?}, want {:?}; ",
                    v, baseline
                ));
                match self.q1_union_quant(pid, t) {
                    Ok(v) if v.iter().copied().collect::<BTreeSet<i64>>() == baseline => {
                        Q1Form::UnionQuant
                    }
                    Ok(v) => {
                        log.push_str(&format!(
                            "union-quant {{1,5}} parsed but WRONG: got {:?}, want {:?}",
                            v, baseline
                        ));
                        Q1Form::FixedChains
                    }
                    Err(e) => {
                        log.push_str(&format!("union-quant {{1,5}} ERROR: {e}"));
                        Q1Form::FixedChains
                    }
                }
            }
            Err(e) => {
                log.push_str(&format!("composite {{0,5}} ERROR: {e}; "));
                match self.q1_union_quant(pid, t) {
                    Ok(v) if v.iter().copied().collect::<BTreeSet<i64>>() == baseline => {
                        Q1Form::UnionQuant
                    }
                    Ok(v) => {
                        log.push_str(&format!(
                            "union-quant {{1,5}} parsed but WRONG: got {:?}, want {:?}",
                            v, baseline
                        ));
                        Q1Form::FixedChains
                    }
                    Err(e) => {
                        log.push_str(&format!("union-quant {{1,5}} ERROR: {e}"));
                        Q1Form::FixedChains
                    }
                }
            }
        };
        let _ = Q1_PROBE_LOG.set(log);
        let _ = Q1_FORM.set(form);
        Ok(form)
    }
}

fn node_props_person(id: i64) -> Vec<(&'static str, Value)> {
    vec![
        ("id", Value::Int64(id)),
        ("name", Value::String(format!("person-{id}").into())),
    ]
}

impl Engine for GrafeoEngine {
    fn name(&self) -> &'static str {
        "grafeo"
    }

    fn load(&mut self, ds: &Dataset) -> Result<()> {
        let db = &self.db;
        let persons: Vec<_> = (0..ds.persons)
            .map(|i| db.create_node_with_props(&["Person"], node_props_person(i)))
            .collect();
        let groups: Vec<_> = (0..ds.groups)
            .map(|i| {
                db.create_node_with_props(
                    &["Grp"],
                    vec![
                        ("id", Value::Int64(i)),
                        ("name", Value::String(format!("group-{i}").into())),
                    ],
                )
            })
            .collect();
        let events: Vec<_> = ds
            .events
            .iter()
            .map(|EventRow { id, start_ts, end_ts, kind }| {
                db.create_node_with_props(
                    &["Event"],
                    vec![
                        ("id", Value::Int64(*id)),
                        ("name", Value::String(format!("event-{id}").into())),
                        ("start_ts", Value::Int64(*start_ts)),
                        ("end_ts", Value::Int64(*end_ts)),
                        ("kind", Value::String(KINDS[*kind as usize].into())),
                    ],
                )
            })
            .collect();

        for MemberRow { person, group, valid_from, valid_to } in &ds.member_of {
            db.create_edge_with_props(
                persons[*person as usize],
                groups[*group as usize],
                "member_of",
                vec![
                    ("valid_from", Value::Int64(*valid_from)),
                    ("valid_to", Value::Int64(*valid_to)),
                    ("role", Value::String("member".into())),
                ],
            );
        }
        for SubgroupRow { child, parent, valid_from, valid_to } in &ds.subgroup_of {
            db.create_edge_with_props(
                groups[*child as usize],
                groups[*parent as usize],
                "subgroup_of",
                vec![
                    ("valid_from", Value::Int64(*valid_from)),
                    ("valid_to", Value::Int64(*valid_to)),
                ],
            );
        }
        for ExpectRow { group, event, obligation, default_priority, valid_from, valid_to } in
            &ds.expects
        {
            db.create_edge_with_props(
                groups[*group as usize],
                events[*event as usize],
                "expects",
                vec![
                    ("obligation", Value::String(OBLIGATIONS[*obligation as usize].into())),
                    ("default_priority", Value::Float64(*default_priority)),
                    ("valid_from", Value::Int64(*valid_from)),
                    ("valid_to", Value::Int64(*valid_to)),
                    ("cascades", Value::Bool(true)),
                ],
            );
        }
        for AttendRow { person, event, start_ts, end_ts, priority_person, priority_coord, source } in
            &ds.attends
        {
            db.create_edge_with_props(
                persons[*person as usize],
                events[*event as usize],
                "attends",
                vec![
                    ("start_ts", Value::Int64(*start_ts)),
                    ("end_ts", Value::Int64(*end_ts)),
                    ("priority_person", Value::Float64(*priority_person)),
                    ("priority_coord", Value::Float64(*priority_coord)),
                    ("coord_binding", Value::Bool(false)),
                    ("source", Value::String(SOURCES[*source as usize].into())),
                    ("pinned", Value::Bool(false)),
                ],
            );
        }
        // Entity-key index (mirror of the Phase-0 SQLite ix lesson: any
        // competent repository indexes its key lookups).
        db.create_property_index("id");
        Ok(())
    }

    fn crud_smoke(&mut self) -> Result<()> {
        let db = &self.db;
        let id = db.create_node_with_props(
            &["Probe"],
            vec![("pk", Value::Int64(1)), ("name", Value::String("alpha".into()))],
        );
        let r = self.exec("MATCH (n:Probe) WHERE n.pk = 1 RETURN n.name")?;
        match r.rows().first().and_then(|row| row.first()) {
            Some(Value::String(s)) if s.as_str() == "alpha" => {}
            other => bail!("grafeo crud read: {other:?}"),
        }
        self.db.set_node_property(id, "name", Value::String("beta".into()));
        let r = self.exec("MATCH (n:Probe) WHERE n.pk = 1 RETURN n.name")?;
        match r.rows().first().and_then(|row| row.first()) {
            Some(Value::String(s)) if s.as_str() == "beta" => {}
            other => bail!("grafeo crud update-read: {other:?}"),
        }
        if !self.db.delete_node(id) {
            bail!("grafeo crud delete returned false");
        }
        let r = self.exec("MATCH (n:Probe) WHERE n.pk = 1 RETURN n.name")?;
        if !r.rows().is_empty() {
            bail!("grafeo crud: node visible after delete");
        }
        // Direct G050 probe (pure form: quantified pattern IS the whole
        // MATCH — the only form grafeo parses). Chain g10-[valid]->g11
        // -[expired]->g12: per-hop WHERE must stop the walk at the expired
        // edge, so only g11 is reachable.
        let g10 = self.db.create_node_with_props(&["Qgrp"], vec![("id", Value::Int64(10))]);
        let g11 = self.db.create_node_with_props(&["Qgrp"], vec![("id", Value::Int64(11))]);
        let g12 = self.db.create_node_with_props(&["Qgrp"], vec![("id", Value::Int64(12))]);
        self.db.create_edge_with_props(
            g10, g11, "subgroup_of",
            vec![("valid_from", Value::Int64(0)), ("valid_to", Value::Int64(1_000_000_000))],
        );
        self.db.create_edge_with_props(
            g11, g12, "subgroup_of",
            vec![("valid_from", Value::Int64(0)), ("valid_to", Value::Int64(100))],
        );
        let got: BTreeSet<i64> = self
            .ids_from(
                "MATCH ((a:Qgrp)-[r:subgroup_of]->(b:Qgrp) \
                 WHERE r.valid_from <= 604800 AND r.valid_to >= 604800){1,2} \
                 RETURN DISTINCT b.id",
            )?
            .into_iter()
            .collect();
        let want: BTreeSet<i64> = [11i64].into_iter().collect();
        if got != want {
            bail!("grafeo G050 pure-form probe: got {got:?}, want {want:?} (per-hop WHERE not pruning)");
        }
        Ok(())
    }

    fn q1(&self, pid: i64, t: i64) -> Result<Vec<i64>> {
        match self.q1_form(pid, t)? {
            Q1Form::Composite => self.q1_composite(pid, t),
            Q1Form::UnionQuant => self.q1_union_quant(pid, t),
            Q1Form::FixedChains => self.q1_fixed_chains(pid, t),
        }
    }

    fn q1_count(&self, pid: i64, t: i64) -> Result<i64> {
        match self.q1_form(pid, t)? {
            Q1Form::Composite => self.scalar_i64(&Self::q1_stmt_quant(0, pid, t, true)),
            // multi-statement forms: count is the client-side union size
            _ => Ok(self.q1(pid, t)?.len() as i64),
        }
    }

    fn q2(&self, pid: i64) -> Result<Vec<(i64, i64)>> {
        let q = format!(
            "MATCH (p:Person {{id: {pid}}})-[a1:attends]->(e1:Event), (p)-[a2:attends]->(e2:Event) \
             WHERE e1.id < e2.id \
             AND a1.start_ts < a2.end_ts AND a2.start_ts < a1.end_ts \
             RETURN e1.id, e2.id"
        );
        let r = self.exec(&q)?;
        r.rows()
            .iter()
            .map(|row| match (&row[0], &row[1]) {
                (Value::Int64(a), Value::Int64(b)) => Ok((*a, *b)),
                other => bail!("grafeo q2: non-int row {other:?}"),
            })
            .collect()
    }

    fn q2_count(&self, pid: i64) -> Result<i64> {
        let q = format!(
            "MATCH (p:Person {{id: {pid}}})-[a1:attends]->(e1:Event), (p)-[a2:attends]->(e2:Event) \
             WHERE e1.id < e2.id \
             AND a1.start_ts < a2.end_ts AND a2.start_ts < a1.end_ts \
             RETURN count(*)"
        );
        self.scalar_i64(&q)
    }

    fn q7b(&self, pid: i64, lo: i64, hi: i64) -> Result<Vec<i64>> {
        let q = format!(
            "MATCH (p:Person {{id: {pid}}})-[a1:attends]->(e:Event)<-[a2:attends]-(q:Person) \
             WHERE e.start_ts >= {lo} AND e.start_ts <= {hi} AND q.id <> {pid} \
             RETURN DISTINCT q.id"
        );
        self.ids_from(&q)
    }

    fn q7b_count(&self, pid: i64, lo: i64, hi: i64) -> Result<i64> {
        let q = format!(
            "MATCH (p:Person {{id: {pid}}})-[a1:attends]->(e:Event)<-[a2:attends]-(q:Person) \
             WHERE e.start_ts >= {lo} AND e.start_ts <= {hi} AND q.id <> {pid} \
             RETURN count(DISTINCT q.id)"
        );
        self.scalar_i64(&q)
    }

    fn notes(&self) -> String {
        // Planner note: WHERE p.id = N is NOT index-anchored by grafeo 0.5.42;
        // the inline (p:Person {id: N}) form is (~40x at S on q2). All queries
        // use the inline anchor.
        let form = match Q1_FORM.get() {
            Some(Q1Form::Composite) => "q1: single-statement {0,5} quantified form".to_string(),
            Some(Q1Form::UnionQuant) => {
                "q1: FALLBACK depth-0 + {1,5} quantified union; count client-side".to_string()
            }
            Some(Q1Form::FixedChains) => {
                "q1: FALLBACK 6 fixed-depth statements; count client-side".to_string()
            }
            None => "q1 form undecided".to_string(),
        };
        match Q1_PROBE_LOG.get() {
            Some(log) if !log.is_empty() => format!("{form} [probe: {log}]"),
            _ => form,
        }
    }
}
