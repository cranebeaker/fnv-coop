//! Loader and validation. A table that loads is internally consistent.

use crate::model::*;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt;

pub const SCHEMA: u32 = 2;

/// The checked-in policy file.
pub const BUNDLED: &str = include_str!("../data/policy.json");

#[derive(Debug)]
pub enum LoadError {
    Parse(String),
    Invalid(Vec<String>),
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoadError::Parse(e) => write!(f, "policy parse error: {e}"),
            LoadError::Invalid(v) => {
                write!(f, "policy invalid ({} problems): {}", v.len(), v.join("; "))
            }
        }
    }
}

impl std::error::Error for LoadError {}

/// What [`PolicyTable::remap`] did.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RemapReport {
    /// Rows now keyed by a runtime id (including those that kept theirs).
    pub resolved: usize,
    /// Of those, rows whose id changed.
    pub moved: usize,
    /// Synthetic rows kept as they are.
    pub synthetic: usize,
    /// (plugin, editor id) of rows dropped because the quest is not in the load order.
    pub unmatched: Vec<(String, String)>,
}

impl RemapReport {
    /// A one-line summary for logs, naming the plugins with unmatched rows.
    pub fn describe(&self) -> String {
        if self.unmatched.is_empty() {
            return format!(
                "policy ids: all {} rows resolved ({} moved)",
                self.resolved, self.moved
            );
        }
        let mut by: BTreeMap<&str, usize> = BTreeMap::new();
        for (p, _) in &self.unmatched {
            *by.entry(p.as_str()).or_default() += 1;
        }
        let list: Vec<String> = by.iter().map(|(p, n)| format!("{p}: {n}")).collect();
        format!(
            "policy ids: {} rows resolved, {} not in the load order ({})",
            self.resolved,
            self.unmatched.len(),
            list.join(", ")
        )
    }
}

/// Validated table with lookups by form id and EDID.
#[derive(Debug, Clone)]
pub struct PolicyTable {
    pub file: PolicyFile,
    by_id: HashMap<FormId, usize>,
    by_edid: HashMap<String, usize>,
    /// quest id -> index of its exclusive group
    group_of: HashMap<FormId, usize>,
    group_members: Vec<Vec<FormId>>,
}

impl PolicyTable {
    pub fn bundled() -> Result<PolicyTable, LoadError> {
        PolicyTable::from_json(BUNDLED)
    }

    pub fn from_json(text: &str) -> Result<PolicyTable, LoadError> {
        let file: PolicyFile =
            crate::json::from_str(text).map_err(|e| LoadError::Parse(e.to_string()))?;
        PolicyTable::from_file(file)
    }

    pub fn from_file(file: PolicyFile) -> Result<PolicyTable, LoadError> {
        let errs = validate(&file);
        if !errs.is_empty() {
            return Err(LoadError::Invalid(errs));
        }
        let mut by_id = HashMap::new();
        let mut by_edid = HashMap::new();
        for (i, q) in file.quests.iter().enumerate() {
            by_id.insert(q.id, i);
            by_edid.insert(q.edid.clone(), i);
        }
        let mut group_of = HashMap::new();
        let mut group_members = Vec::new();
        for (gi, g) in file.groups.iter().enumerate() {
            let ids: Vec<FormId> = g
                .members
                .iter()
                .map(|m| file.quests[by_edid[m]].id)
                .collect();
            for id in &ids {
                group_of.insert(*id, gi);
            }
            group_members.push(ids);
        }
        Ok(PolicyTable {
            file,
            by_id,
            by_edid,
            group_of,
            group_members,
        })
    }

    /// A copy of the table with every row keyed by the id `resolve(row)` gives it (its runtime
    /// form id in the actual load order). Rows `resolve` returns `None` for are quests that
    /// are not loaded: they are dropped and listed in the report, never an error. Synthetic
    /// rows (no record) keep the id they have.
    pub fn remap(
        &self,
        mut resolve: impl FnMut(&QuestRow) -> Option<FormId>,
    ) -> Result<(PolicyTable, RemapReport), LoadError> {
        let mut file = self.file.clone();
        let mut rep = RemapReport::default();
        let mut kept = Vec::with_capacity(file.quests.len());
        let mut gone: BTreeSet<String> = BTreeSet::new();
        for mut q in std::mem::take(&mut file.quests) {
            if q.synthetic {
                rep.synthetic += 1;
                kept.push(q);
                continue;
            }
            match resolve(&q) {
                Some(id) => {
                    if id != q.id {
                        rep.moved += 1;
                    }
                    rep.resolved += 1;
                    q.id = id;
                    kept.push(q);
                }
                None => {
                    rep.unmatched.push((q.plugin.clone(), q.edid.clone()));
                    gone.insert(q.edid.clone());
                }
            }
        }
        file.quests = kept;
        // an exclusive group keeps only the members that are loaded
        for g in &mut file.groups {
            g.members.retain(|m| !gone.contains(m));
        }
        file.groups.retain(|g| g.members.len() > 1);
        Ok((PolicyTable::from_file(file)?, rep))
    }

    pub fn row(&self, id: FormId) -> Option<&QuestRow> {
        self.by_id.get(&id).map(|&i| &self.file.quests[i])
    }

    pub fn row_by_edid(&self, edid: &str) -> Option<&QuestRow> {
        self.by_edid.get(edid).map(|&i| &self.file.quests[i])
    }

    pub fn rows(&self) -> &[QuestRow] {
        &self.file.quests
    }

    pub fn policy(&self) -> &GlobalPolicy {
        &self.file.policy
    }

    /// Other members of the exclusive group `id` belongs to (excluding `id`).
    pub fn group_rivals(&self, id: FormId) -> Vec<FormId> {
        match self.group_of.get(&id) {
            Some(&g) => self.group_members[g]
                .iter()
                .copied()
                .filter(|m| *m != id)
                .collect(),
            None => vec![],
        }
    }

    pub fn residence(&self, class: ShadowClass) -> Residence {
        self.file
            .shadow_classes
            .get(&class)
            .copied()
            .unwrap_or(Residence::Shared)
    }

    /// Rows still marked `default` (never reviewed by hand).
    pub fn unreviewed(&self) -> impl Iterator<Item = &QuestRow> {
        self.file
            .quests
            .iter()
            .filter(|q| q.status == Status::Default)
    }

    pub fn counts(&self) -> (usize, usize) {
        let c = self
            .file
            .quests
            .iter()
            .filter(|q| q.status == Status::Confirmed)
            .count();
        (c, self.file.quests.len() - c)
    }
}

fn validate(f: &PolicyFile) -> Vec<String> {
    let mut e: Vec<String> = Vec::new();
    if f.schema != SCHEMA {
        e.push(format!("schema {} not supported (want {SCHEMA})", f.schema));
    }
    let p = &f.policy;
    if p.barrier_timeout_s == 0 || p.barrier_timeout_s > 3600 {
        e.push("policy.barrier_timeout_s out of range 1..=3600".into());
    }
    if p.tether_timeout_s == 0 || p.tether_timeout_s > 3600 {
        e.push("policy.tether_timeout_s out of range 1..=3600".into());
    }
    if p.party_cap == 0 || p.party_cap > 8 {
        e.push("policy.party_cap out of range 1..=8".into());
    }
    let mut ids = BTreeSet::new();
    let mut edids = BTreeSet::new();
    for q in &f.quests {
        let n = &q.edid;
        if n.is_empty() {
            e.push(format!("row with id 0x{:08X} has an empty edid", q.id));
            continue;
        }
        if q.id == 0 {
            e.push(format!("{n}: form id 0"));
        }
        if !ids.insert(q.id) {
            e.push(format!("{n}: duplicate form id 0x{:08X}", q.id));
        }
        if !edids.insert(n.clone()) {
            e.push(format!("{n}: duplicate edid"));
        }
        if q.local > 0x00FF_FFFF {
            e.push(format!(
                "{n}: local id 0x{:08X} has a load index byte",
                q.local
            ));
        }
        if q.plugin.is_empty() {
            e.push(format!("{n}: empty plugin"));
        }
        if let Some(t) = &q.tether {
            if t.cells.is_empty() && t.anchor.is_none() {
                e.push(format!("{n}: tether needs cells or an anchor"));
            }
            if t.anchor.is_some() && t.radius.unwrap_or(0) == 0 {
                e.push(format!("{n}: tether anchor needs a radius"));
            }
            if t.radius.is_some() && t.anchor.is_none() {
                e.push(format!("{n}: tether radius without anchor"));
            }
            if matches!(t.timeout_s, Some(0)) || t.timeout_s.unwrap_or(0) > 3600 {
                e.push(format!("{n}: tether timeout out of range"));
            }
        }
        if let Some(b) = &q.barrier {
            // An empty barrier gates the quest's host scenes (ending, cinematic) without naming a stage.
            if let Some(w) = &b.before {
                if w != "slideshow" {
                    e.push(format!("{n}: unknown barrier gate {w:?}"));
                }
            }
        }
        if let Some(r) = &q.ready {
            if r.timeout_s == 0 || r.timeout_s > 3600 {
                e.push(format!("{n}: ready timeout out of range"));
            }
        }
        if let Some(x) = &q.exclusive {
            let s: BTreeSet<_> = x.stages.iter().collect();
            if s.len() < 2 {
                e.push(format!("{n}: exclusive needs at least two distinct stages"));
            }
        }
        if let Some(c) = &q.consent {
            if c.timeout_s == 0 || c.timeout_s > 3600 {
                e.push(format!("{n}: consent timeout out of range"));
            }
            if c.topics.is_empty() && c.boxes.is_empty() {
                e.push(format!("{n}: consent names no topic or box"));
            }
        }
        if let Some(w) = &q.warp {
            if w.markers.is_empty() {
                e.push(format!("{n}: warp without markers"));
            }
        }
        if let Some(c) = &q.controls {
            if c.timeout_s == 0 || c.timeout_s > 3600 {
                e.push(format!("{n}: controls timeout out of range"));
            }
        }
        if let Some(s) = &q.shadow {
            if s.vars.is_empty() && s.objvars.is_empty() && s.scripts.is_empty() {
                e.push(format!("{n}: shadow names nothing"));
            }
            if s.down.is_some() && s.checkpoint.is_empty() {
                e.push(format!("{n}: shadow.down needs checkpoints"));
            }
        }
        if let Some(c) = &q.companion {
            if c.owner == CompanionOwner::Recruiter && !c.npcs.is_empty() {
                // allowed: names the recruitable set
            }
        }
        if let Some(ff) = &q.friendly_fire {
            if ff.deliberate_hits == 0 || ff.actors.is_empty() {
                e.push(format!("{n}: friendly_fire needs actors and hits >= 1"));
            }
        }
        if q.reward == Reward::Split && q.pooled_items.is_empty() && q.status == Status::Confirmed {
            // a split reward without a pool is legal; nothing to check
        }
        if q.gate_exempt && (q.tether.is_some() || q.barrier.is_some() || q.ready.is_some()) {
            e.push(format!("{n}: gate_exempt row carries gates"));
        }
    }
    let mut seen_members = BTreeSet::new();
    for g in &f.groups {
        if g.members.len() < 2 {
            e.push(format!("group {}: needs two members", g.name));
        }
        for m in &g.members {
            if !edids.contains(m) {
                e.push(format!("group {}: unknown quest {m}", g.name));
            }
            if !seen_members.insert(m.clone()) {
                e.push(format!("group {}: quest {m} is in two groups", g.name));
            }
        }
    }
    e
}
