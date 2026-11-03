//! Data for the GAME checklist layer: every quest of the load order (not
//! only the main lines), who sets each stage, and a stage-by-stage run with
//! the world's `Runner`. Library calls only; the `coop-sim checklist` runner
//! and the `gamerows` generator both use this file.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::panic::{catch_unwind, AssertUnwindSafe};

use esm::{FormId, FourCC, LoadOrder};
use world::scripting::{GameState, ScriptCache};

use crate::funcs::handled_set;
use crate::quests::{panic_text, stage_verdict, Verdict};
use crate::scan::{all_calls, Kind, Source};

/// One quest record.
#[derive(Debug, Clone)]
pub struct QuestInfo {
    pub id: FormId,
    pub edid: String,
    pub name: String,
    pub plugin: String,
    /// Stage numbers, sorted.
    pub stages: Vec<u16>,
    /// The quest's own script, if any.
    pub script: Option<FormId>,
    /// `DATA` flags: bit 0 start game enabled, bit 2 allow repeated stages (as stored).
    pub flags: u8,
    pub priority: u8,
    pub objectives: usize,
}

/// Every quest in the load order, sorted by plugin load order then editor id.
pub fn quests(order: &LoadOrder) -> Vec<QuestInfo> {
    let mut out = Vec::new();
    for rr in order.records_of_type(FourCC::new(b"QUST")) {
        let Ok(r) = rr.record() else { continue };
        if rr.entry.header.is_deleted() {
            continue;
        }
        let mut stages: Vec<u16> = r
            .get_all(FourCC::new(b"INDX"))
            .filter(|x| x.data.len() >= 2)
            .map(|x| u16::from_le_bytes([x.data[0], x.data[1]]))
            .collect();
        stages.sort_unstable();
        stages.dedup();
        let script = r.get(FourCC::new(b"SCRI")).and_then(|s| {
            s.data.get(..4).map(|b| {
                rr.plugin
                    .to_global(FormId(u32::from_le_bytes([b[0], b[1], b[2], b[3]])))
            })
        });
        let (flags, priority) = r.get(FourCC::new(b"DATA")).map_or((0, 0), |d| {
            (
                d.data.first().copied().unwrap_or(0),
                d.data.get(1).copied().unwrap_or(0),
            )
        });
        let objectives = r.get_all(FourCC::new(b"QOBJ")).count();
        out.push(QuestInfo {
            id: rr.form_id,
            edid: r.editor_id().unwrap_or_default(),
            name: r.full_name().unwrap_or_default(),
            plugin: rr.plugin.name.clone(),
            stages,
            script,
            flags,
            priority,
            objectives,
        });
    }
    out
}

/// Who sets which stage, and what else touches each quest.
#[derive(Debug, Default)]
pub struct Evidence {
    /// (quest editor id lowercase, stage) -> kinds of script that set it with a literal stage.
    pub setters: BTreeMap<(String, u16), BTreeSet<Kind>>,
    /// Quests set with a stage the script computes (the stage is not a literal), or by a reference variable.
    pub dynamic: BTreeMap<String, usize>,
    /// Quest editor id lowercase -> up to eight scripts that name it: (kind, owner editor id).
    pub mentions: BTreeMap<String, Vec<(Kind, String)>>,
    /// Quests that have their own script (the script id).
    pub own_script: BTreeSet<String>,
}

/// Builds the evidence from every script source. A literal stage on a call with no quest
/// word (`SetStage 20`) belongs to the quest that owns the script or result.
pub fn evidence(order: &LoadOrder, sources: &[Source], list: &[QuestInfo]) -> Evidence {
    let set_stage = script::function("SetStage").map(|(i, _)| i);
    let by_edid: HashMap<String, &QuestInfo> = list
        .iter()
        .map(|q| (q.edid.to_ascii_lowercase(), q))
        .collect();
    let script_owner: HashMap<FormId, &QuestInfo> = list
        .iter()
        .filter_map(|q| q.script.map(|s| (s, q)))
        .collect();
    let id_to_edid: HashMap<FormId, &str> = list.iter().map(|q| (q.id, q.edid.as_str())).collect();
    let mut ev = Evidence::default();
    for q in list {
        if q.script.is_some() {
            ev.own_script.insert(q.edid.to_ascii_lowercase());
        }
    }
    for s in sources {
        // Mentions: identifier tokens of the text that name a quest.
        let mut seen: BTreeSet<String> = BTreeSet::new();
        for tok in s
            .text
            .split(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
        {
            if tok.len() < 3 {
                continue;
            }
            let k = tok.to_ascii_lowercase();
            if by_edid.contains_key(&k) && seen.insert(k.clone()) {
                let v = ev.mentions.entry(k).or_default();
                if v.len() < 8 {
                    v.push((
                        s.kind,
                        if s.editor.is_empty() {
                            s.form.to_string()
                        } else {
                            s.editor.clone()
                        },
                    ));
                }
            }
        }
        let Ok(p) = &s.parsed else { continue };
        let owner: Option<&str> = match s.kind {
            Kind::QuestScript => script_owner.get(&s.form).map(|q| q.edid.as_str()),
            Kind::StageResult | Kind::DialogueResult => {
                s.quest.and_then(|q| id_to_edid.get(&q).copied())
            }
            _ => None,
        };
        for c in all_calls(p) {
            if Some(c.function) != set_stage {
                continue;
            }
            let stage_lit = c.args.iter().find_map(|a| {
                if let script::Arg::Number(n) = a {
                    Some(*n as u16)
                } else {
                    None
                }
            });
            let quest: Option<String> = match (&c.on, c.args.first()) {
                (_, Some(script::Arg::Word(w)))
                    if by_edid.contains_key(&w.to_ascii_lowercase()) =>
                {
                    Some(w.to_ascii_lowercase())
                }
                (Some(on), _) if by_edid.contains_key(&on.to_ascii_lowercase()) => {
                    Some(on.to_ascii_lowercase())
                }
                (None, Some(script::Arg::Number(_))) => owner.map(|o| o.to_ascii_lowercase()),
                _ => None,
            };
            match (quest, stage_lit) {
                (Some(q), Some(st)) => {
                    ev.setters.entry((q, st)).or_default().insert(s.kind);
                }
                (Some(q), None) => *ev.dynamic.entry(q).or_default() += 1,
                (None, _) => {
                    // A reference variable or an unknown name: counts for no particular quest.
                    *ev.dynamic.entry("*".into()).or_default() += 1;
                }
            }
        }
    }
    let _ = order;
    ev
}

/// Stages (other than the lowest, how a quest starts) that no script sets with a literal.
pub fn never_set(list: &[QuestInfo], ev: &Evidence) -> Vec<(String, u16)> {
    let mut v = Vec::new();
    for q in list {
        let low = q.stages.first().copied();
        for st in &q.stages {
            if Some(*st) != low && !ev.setters.contains_key(&(q.edid.to_ascii_lowercase(), *st)) {
                v.push((q.edid.clone(), *st));
            }
        }
    }
    v
}

/// One stage's run result.
#[derive(Debug, Clone)]
pub struct StageRun {
    pub stage: u16,
    pub has_script: bool,
    pub verdict: Verdict,
}

/// Runs every stage of the quests whose editor id (lowercase) is in `only` (all when `None`),
/// with the world's `Runner`, one game state per plugin. Panics are findings.
pub fn walk(
    order: &LoadOrder,
    list: &[QuestInfo],
    only: Option<&HashSet<String>>,
) -> BTreeMap<String, Vec<StageRun>> {
    let handled = handled_set();
    let scripts = ScriptCache::default();
    let mut out: BTreeMap<String, Vec<StageRun>> = BTreeMap::new();
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let mut states: BTreeMap<String, GameState> = BTreeMap::new();
    for q in list {
        if only.is_some_and(|o| !o.contains(&q.edid.to_ascii_lowercase())) {
            continue;
        }
        let state = states
            .entry(q.plugin.clone())
            .or_insert_with(|| GameState::new(order));
        let runs = out.entry(q.edid.to_ascii_lowercase()).or_default();
        let Some(quest) = catch_unwind(AssertUnwindSafe(|| scripts.quest(order, q.id)))
            .ok()
            .flatten()
        else {
            for st in &q.stages {
                runs.push(StageRun {
                    stage: *st,
                    has_script: false,
                    verdict: Verdict::Panic("quest record did not load".into()),
                });
            }
            continue;
        };
        for st in &q.stages {
            let Some(stage) = quest.stage(*st) else {
                continue;
            };
            let has_script = stage.entries.iter().any(|e| e.script.is_some());
            let verdict = catch_unwind(AssertUnwindSafe(|| {
                stage_verdict(order, &scripts, state, q.id, *st, stage, &handled)
            }))
            .unwrap_or_else(|p| Verdict::Panic(panic_text(p)));
            runs.push(StageRun {
                stage: *st,
                has_script,
                verdict,
            });
        }
    }
    std::panic::set_hook(hook);
    out
}

/// Interior cells grouped into places a player knows, from the editor id: the leading word
/// (letters up to the first digit, underscore or second capital run), lowercased.
pub fn place_of(edid: &str) -> String {
    let mut word = String::new();
    for (i, ch) in edid.chars().enumerate() {
        if ch.is_ascii_digit() || ch == '_' || ch == ' ' || ch == '-' {
            break;
        }
        if i > 0
            && ch.is_ascii_uppercase()
            && word.chars().last().is_some_and(|l| l.is_ascii_lowercase())
        {
            break;
        }
        word.push(ch);
    }
    if word.is_empty() {
        "misc".into()
    } else {
        word.to_ascii_lowercase()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn places_from_editor_ids() {
        assert_eq!(place_of("GSDocMitchellHouse"), "gsdoc");
        assert_eq!(place_of("Novac01"), "novac");
        assert_eq!(place_of("Vault21Entrance"), "vault");
        assert_eq!(place_of(""), "misc");
    }
}
