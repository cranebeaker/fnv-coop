//! The quest stage graph from the data: who sets each stage (which kind of
//! script), and which main-line quests depend on functions nv-rs lacks.

use std::collections::{BTreeMap, BTreeSet};

use crate::data::{line_of, Line};
use crate::funcs::FuncReport;
use crate::scan::{all_calls, Kind, Source};

#[derive(Debug, Default)]
pub struct QuestGraph {
    /// (quest editor id, stage) → kinds of script that set it (`SetStage`).
    pub setters: BTreeMap<(String, u16), BTreeSet<Kind>>,
    /// Quest → stage list from the record.
    pub stages: BTreeMap<String, Vec<u16>>,
    /// Quests (main-line) → missing function names their scripts and dialogue use.
    pub deps: BTreeMap<String, BTreeSet<String>>,
    /// Main-line stages no script, dialogue result or terminal sets.
    pub never_set: Vec<(String, u16)>,
}

pub fn build(order: &esm::LoadOrder, sources: &[Source], funcs: &FuncReport) -> QuestGraph {
    let mut g = QuestGraph::default();
    let set_stage = script::function("SetStage").map(|(i, _)| i);
    for s in sources {
        let Ok(p) = &s.parsed else { continue };
        for c in all_calls(p) {
            if Some(c.function) != set_stage {
                continue;
            }
            // `SetStage Quest 10` or `Quest.SetStage 10`: the quest is the first word argument
            // or the reference the call is on; the stage is the first number.
            let quest = match (&c.on, c.args.first()) {
                (_, Some(script::Arg::Word(w))) => Some(w.clone()),
                (Some(on), _) => Some(on.clone()),
                _ => None,
            };
            let stage = c.args.iter().find_map(|a| {
                if let script::Arg::Number(n) = a {
                    Some(*n as u16)
                } else {
                    None
                }
            });
            if let (Some(q), Some(st)) = (quest, stage) {
                g.setters
                    .entry((q.to_ascii_lowercase(), st))
                    .or_default()
                    .insert(s.kind);
            }
        }
    }
    let mut main: BTreeSet<String> = BTreeSet::new();
    for (id, editor, plugin) in crate::data::quests(order) {
        let Some(line) = line_of(&plugin, &editor) else {
            continue;
        };
        main.insert(editor.clone());
        if let Some(rr) = order.get(id) {
            if let Ok(r) = rr.record() {
                let mut list: Vec<u16> = r
                    .get_all(esm::FourCC::new(b"INDX"))
                    .filter(|x| x.data.len() >= 2)
                    .map(|x| u16::from_le_bytes([x.data[0], x.data[1]]))
                    .collect();
                list.sort_unstable();
                list.dedup();
                let lowest = list.first().copied();
                for st in &list {
                    // The first stage is how a quest starts (often StartQuest or START_GAME_ENABLED).
                    if Some(*st) != lowest
                        && !g.setters.contains_key(&(editor.to_ascii_lowercase(), *st))
                        && line != Line::Vms
                    {
                        g.never_set.push((editor.clone(), *st));
                    }
                }
                g.stages.insert(editor, list);
            }
        }
    }
    for s in funcs
        .stats
        .iter()
        .filter(|s| !s.handled && !s.quests.is_empty())
    {
        for q in &s.quests {
            g.deps.entry(q.clone()).or_default().insert(s.name.clone());
        }
    }
    g
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_graph_has_no_deps() {
        let g = QuestGraph::default();
        assert!(g.deps.is_empty() && g.never_set.is_empty());
    }
}
