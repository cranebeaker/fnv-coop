//! Script function coverage: every function the game's scripts call or its
//! conditions ask, ranked by use, against what the interpreter and the world
//! host carry out.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use esm::{FormId, FourCC, LoadOrder};

use crate::data::{line_of, Line};
use crate::scan::{calls_of, Kind, Source};
use crate::systems::group_of;

/// The functions nv-rs carries out, by lower case full name.
pub fn handled_set() -> HashSet<String> {
    world::scripting::HANDLED
        .iter()
        .copied()
        .chain(world::script_functions::handled())
        .map(|s| s.to_ascii_lowercase())
        .collect()
}

#[derive(Debug, Default, Clone)]
pub struct FnStat {
    pub index: u16,
    pub name: String,
    pub group: &'static str,
    pub handled: bool,
    pub calls: usize,
    pub scripts: usize,
    pub by_kind: BTreeMap<Kind, usize>,
    pub conditions: usize,
    /// Editor ids of the main-line quests (main, VMS, DLC main) whose scripts or dialogue use it.
    pub quests: BTreeSet<String>,
}

#[derive(Debug, Default)]
pub struct Totals {
    pub sources: usize,
    pub parsed: usize,
}

#[derive(Debug, Default)]
pub struct FuncReport {
    pub table: usize,
    pub named: usize,
    pub named_handled: usize,
    pub stats: Vec<FnStat>,
    pub by_kind: BTreeMap<Kind, Totals>,
    /// Parse failure message (line number removed) → (count, examples).
    pub parse_errors: BTreeMap<String, (usize, Vec<String>)>,
    /// Block kinds (`gamemode`, `onactivate`, ...) → scripts using them.
    pub events: BTreeMap<String, usize>,
}

impl FuncReport {
    pub fn used(&self) -> impl Iterator<Item = &FnStat> {
        self.stats.iter().filter(|s| s.calls + s.conditions > 0)
    }
    pub fn used_count(&self) -> usize {
        self.used().count()
    }
    pub fn used_handled(&self) -> usize {
        self.used().filter(|s| s.handled).count()
    }
    /// Missing functions, most used first (calls plus conditions).
    pub fn missing(&self) -> Vec<&FnStat> {
        let mut v: Vec<&FnStat> = self.used().filter(|s| !s.handled).collect();
        v.sort_by_key(|s| std::cmp::Reverse((s.quests.len(), s.calls + s.conditions)));
        v
    }
    /// Missing functions ranked by use alone.
    pub fn missing_by_use(&self) -> Vec<&FnStat> {
        let mut v: Vec<&FnStat> = self.used().filter(|s| !s.handled).collect();
        v.sort_by_key(|s| std::cmp::Reverse(s.calls + s.conditions));
        v
    }
    pub fn weight(&self, handled_only: bool) -> usize {
        self.used()
            .filter(|s| !handled_only || s.handled)
            .map(|s| s.calls + s.conditions)
            .sum()
    }
}

/// Which quest a quest script (`SCRI` of a `QUST`) belongs to.
fn quest_scripts(order: &LoadOrder) -> HashMap<FormId, FormId> {
    let mut map = HashMap::new();
    for rr in order.records_of_type(FourCC::new(b"QUST")) {
        if let Ok(r) = rr.record() {
            if let Some(s) = r.get(FourCC::new(b"SCRI")) {
                if let Some(raw) = s.data.get(..4) {
                    let id = rr
                        .plugin
                        .to_global(FormId(u32::from_le_bytes([raw[0], raw[1], raw[2], raw[3]])));
                    map.insert(id, rr.form_id);
                }
            }
        }
    }
    map
}

/// Function condition uses over every record type.
fn condition_total(order: &LoadOrder) -> BTreeMap<u16, usize> {
    let mut all = BTreeMap::new();
    for (kind, _) in order.type_counts() {
        for (f, n) in crate::scan::condition_uses(order, kind.as_bytes()) {
            *all.entry(f).or_default() += n;
        }
    }
    all
}

pub fn analyse(order: &LoadOrder, sources: &[Source]) -> FuncReport {
    let handled = handled_set();
    let table = &script::functions::FUNCTIONS;
    let mut stats: Vec<FnStat> = table
        .iter()
        .enumerate()
        .map(|(i, sig)| FnStat {
            index: i as u16,
            name: sig.name.to_string(),
            group: group_of(sig.name, 0x1000 + i as u16),
            handled: handled.contains(&sig.name.to_ascii_lowercase()),
            ..FnStat::default()
        })
        .collect();
    let quest_names: HashMap<FormId, (String, Line)> = crate::data::quests(order)
        .into_iter()
        .filter_map(|(id, e, p)| line_of(&p, &e).map(|l| (id, (e, l))))
        .collect();
    let by_script = quest_scripts(order);
    let mut rep = FuncReport::default();
    for s in sources {
        let t = rep.by_kind.entry(s.kind).or_default();
        t.sources += 1;
        match &s.parsed {
            Err(e) => {
                let msg = e
                    .split_once(": ")
                    .map_or(e.as_str(), |(_, m)| m)
                    .to_string();
                let slot = rep.parse_errors.entry(msg).or_default();
                slot.0 += 1;
                if slot.1.len() < 5 {
                    slot.1
                        .push(format!("{} {} ({})", s.record, s.editor, s.plugin));
                }
            }
            Ok(parsed) => {
                t.parsed += 1;
                for b in &parsed.blocks {
                    *rep.events.entry(b.kind.clone()).or_default() += 1;
                }
                let quest = s.quest.or_else(|| by_script.get(&s.form).copied());
                let named = quest
                    .and_then(|q| quest_names.get(&q))
                    .map(|(e, _)| e.clone());
                let mut seen = BTreeSet::new();
                for f in calls_of(parsed) {
                    let Some(st) = stats.get_mut(usize::from(f)) else {
                        continue;
                    };
                    st.calls += 1;
                    *st.by_kind.entry(s.kind).or_default() += 1;
                    if seen.insert(f) {
                        st.scripts += 1;
                    }
                    if let Some(n) = &named {
                        st.quests.insert(n.clone());
                    }
                }
            }
        }
    }
    for (f, n) in condition_total(order) {
        if let Some(st) = stats.get_mut(usize::from(f)) {
            st.conditions += n;
        }
    }
    rep.table = table.len();
    rep.named = stats
        .iter()
        .filter(|s| !s.name.starts_with("UnusedFunction"))
        .count();
    rep.named_handled = stats
        .iter()
        .filter(|s| !s.name.starts_with("UnusedFunction") && s.handled)
        .count();
    rep.stats = stats;
    rep
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handled_set_knows_core_functions() {
        let h = handled_set();
        assert!(h.contains("setstage"));
        assert!(h.contains("getstage"));
    }
}
