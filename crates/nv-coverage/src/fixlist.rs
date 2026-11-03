//! The prioritised fix list, built from the measurements: functions the
//! game's own scripts need that nv-rs does not carry out, grouped by
//! system, ranked by the quest stages they block and then by use.

use std::collections::{BTreeMap, BTreeSet};

use crate::data::Line;
use crate::funcs::FuncReport;
use crate::quests::{LineReport, Verdict};

#[derive(Debug, Clone)]
pub struct Fix {
    pub rank: usize,
    pub title: String,
    pub system: String,
    pub functions: Vec<String>,
    /// Quest stages (across all lines) whose scripts name one of the functions.
    pub stages_blocked: usize,
    /// Main-line quests whose scripts or dialogue use one of them.
    pub quests: usize,
    pub uses: usize,
    pub size: &'static str,
    /// Whether the behaviour has to come from the exe (Ghidra) or the data alone settles it.
    pub source: &'static str,
    pub files: Vec<&'static str>,
    pub brief: String,
    /// Functions of the item that already have a translated exe handler: name, address, file.
    pub translated: Vec<String>,
}

/// Stages blocked, by function name.
pub fn blocked_by(lines: &BTreeMap<Line, LineReport>) -> BTreeMap<String, BTreeSet<(String, u16)>> {
    let mut out: BTreeMap<String, BTreeSet<(String, u16)>> = BTreeMap::new();
    for rep in lines.values() {
        for s in &rep.stages {
            let names: &[String] = match &s.verdict {
                Verdict::Unhandled(n) | Verdict::StaticMissing(n) => n,
                _ => continue,
            };
            for n in names {
                out.entry(n.to_ascii_lowercase())
                    .or_default()
                    .insert((s.editor.clone(), s.stage));
            }
        }
    }
    out
}

fn files_for(system: &str) -> Vec<&'static str> {
    match system {
        "quests" => vec![
            "crates/world/src/scripting.rs",
            "crates/world/src/quest.rs",
            "crates/world/src/quest_text.rs",
        ],
        "dialogue" => vec![
            "crates/world/src/dialogue.rs",
            "crates/world/src/scripting.rs",
        ],
        "combat" => vec![
            "crates/world/src/combat.rs",
            "crates/world/src/scripting.rs",
        ],
        "inventory" => vec!["crates/world/src/items.rs", "crates/world/src/scripting.rs"],
        "UI" => vec![
            "crates/world/src/script_functions.rs",
            "viewer/src/menus.rs",
        ],
        "sound" => vec!["crates/world/src/sound.rs", "crates/world/src/music.rs"],
        "camera" => vec![
            "crates/world/src/player_camera.rs",
            "viewer/src/player_camera.rs",
        ],
        "world" => vec![
            "crates/world/src/script_functions.rs",
            "crates/world/src/more_functions/mod.rs",
        ],
        "actors" => vec![
            "crates/world/src/more_functions/actors.rs",
            "crates/world/src/living/mod.rs",
        ],
        "NVSE-only" => vec!["crates/script/src/nvse.rs", "docs/NVSE_COVERAGE.md"],
        _ => vec!["crates/world/src/script_functions.rs"],
    }
}

/// Reads (Get/Is/Has) return values from stored data; the rest change runtime state.
fn is_query(name: &str) -> bool {
    ["Get", "Is", "Has", "Can", "Was", "Count", "Eval"]
        .iter()
        .any(|p| name.starts_with(p))
}

pub fn build(
    funcs: &FuncReport,
    lines: &BTreeMap<Line, LineReport>,
    parse_errors: usize,
    trans: &std::collections::HashMap<String, (String, String)>,
) -> Vec<Fix> {
    let blocked = blocked_by(lines);
    let mut by_group: BTreeMap<&str, Vec<&crate::funcs::FnStat>> = BTreeMap::new();
    for s in funcs.missing_by_use() {
        by_group.entry(s.group).or_default().push(s);
    }
    let mut fixes = Vec::new();
    for (group, list) in by_group {
        // A fix item is at most eight functions, so one agent brief fits one item.
        let mut ranked = list.clone();
        ranked.sort_by_key(|s| {
            std::cmp::Reverse((
                blocked
                    .get(&s.name.to_ascii_lowercase())
                    .map_or(0, |b| b.len()),
                s.quests.len(),
                s.calls + s.conditions,
            ))
        });
        for (n, chunk) in ranked.chunks(8).enumerate() {
            let mut stages = BTreeSet::new();
            let mut quests = BTreeSet::new();
            let mut uses = 0;
            for s in chunk {
                if let Some(b) = blocked.get(&s.name.to_ascii_lowercase()) {
                    stages.extend(b.iter().cloned());
                }
                quests.extend(s.quests.iter().cloned());
                uses += s.calls + s.conditions;
            }
            let queries = chunk.iter().filter(|s| is_query(&s.name)).count();
            let names: Vec<String> = chunk.iter().map(|s| s.name.clone()).collect();
            let source = if group == "NVSE-only" {
                "xNVSE source plus exe (these are extender functions, not in the exe)"
            } else if queries * 2 >= chunk.len() {
                "mostly data-only: read the stored field; cite the exe address of the command handler for the exact rule"
            } else {
                "needs exe decompilation: runtime state changes; cite the command handler address"
            };
            let size = match chunk.len() {
                0..=2 => "S (under half a day)",
                3..=5 => "M (about a day)",
                _ => "L (one to two days)",
            };
            let title = format!(
                "{group}: {}{}",
                names.iter().take(3).cloned().collect::<Vec<_>>().join(", "),
                if names.len() > 3 { ", ..." } else { "" }
            );
            let translated: Vec<String> = chunk
                .iter()
                .filter_map(|s| {
                    trans
                        .get(&s.name.to_ascii_lowercase())
                        .map(|(a, f)| format!("{} {a} {f}", s.name))
                })
                .collect();
            let brief = format!(
                "Implement these script functions in the world host with provenance (exe address of each command handler from Ghidra, or the record field they read): {}. \
                 Add each name to the HANDLED list of the file that carries it out, add a unit test per function on real records, \
                 and re-run `nv-coverage functions` to confirm the count drops. Do not invent behaviour: a function you cannot cite stays on the list. Batch {} of the {group} group.",
                names.join(", "),
                n + 1
            );
            fixes.push(Fix {
                rank: 0,
                title,
                system: group.to_string(),
                functions: names,
                stages_blocked: stages.len(),
                quests: quests.len(),
                uses,
                size,
                source,
                files: files_for(group),
                brief,
                translated,
            });
        }
    }
    if parse_errors > 0 {
        fixes.push(Fix {
            rank: 0,
            title: "Script parser: sources that do not parse".into(),
            system: "parser".into(),
            functions: Vec::new(),
            stages_blocked: lines.values().flat_map(|l| &l.stages).filter(|s| matches!(s.verdict, Verdict::ParseError(_))).count(),
            quests: 0,
            uses: parse_errors,
            size: "M (about a day)",
            source: "data-only: the failing source text and the compiled form (SCDA) settle the grammar",
            files: vec!["crates/script/src/parser.rs", "crates/script/src/lexer.rs"],
            translated: Vec::new(),
            brief: "Run `nv-coverage functions`, read the parse error table, and extend the parser for each message in order of count; check each fix against the compiled form the way `nvinspect scripts` does.".into(),
        });
    }
    fixes.sort_by_key(|f| std::cmp::Reverse((f.stages_blocked, f.quests, f.uses)));
    for (i, f) in fixes.iter_mut().enumerate() {
        f.rank = i + 1;
    }
    fixes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queries_are_recognised() {
        assert!(is_query("GetStage"));
        assert!(!is_query("SetStage"));
    }

    #[test]
    fn empty_input_gives_no_fixes() {
        assert!(build(
            &FuncReport::default(),
            &BTreeMap::new(),
            0,
            &Default::default()
        )
        .is_empty());
    }
}
