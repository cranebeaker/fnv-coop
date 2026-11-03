//! Quest walk: sets every stage of the main questlines in order with the
//! world's `Runner`, and records which stages fail to run.

use std::collections::{BTreeMap, BTreeSet};
use std::panic::{catch_unwind, AssertUnwindSafe};

use esm::{FormId, LoadOrder};
use world::scripting::{GameState, Runner, ScriptCache};

use crate::data::{line_of, Line};
use crate::funcs::handled_set;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verdict {
    /// Ran with nothing missing (or had no script).
    Clean,
    /// The stage's script did not parse.
    ParseError(String),
    /// The host does not carry out these functions (they are not in the handled lists).
    Unhandled(Vec<String>),
    /// Handled functions that returned no value because the stub world has no placed
    /// references or live actors (`GetDistance`, `GetInCell`, ...). Not an nv-rs gap:
    /// the stage runs in the viewer. Counted as runnable.
    StubLimit(Vec<String>),
    /// The script calls functions nv-rs does not carry out on a path the stub world did not take.
    StaticMissing(Vec<String>),
    /// The host panicked.
    Panic(String),
    /// `set_stage` refused (stage unknown to the quest or already done).
    Refused,
}

impl Verdict {
    pub fn is_clean(&self) -> bool {
        matches!(self, Verdict::Clean)
    }
    /// Runs, or only stops on questions the stub world cannot answer.
    pub fn runnable(&self) -> bool {
        matches!(self, Verdict::Clean | Verdict::StubLimit(_))
    }
    pub fn tag(&self) -> &'static str {
        match self {
            Verdict::Clean => "clean",
            Verdict::ParseError(_) => "parse error",
            Verdict::Unhandled(_) => "unhandled function",
            Verdict::StubLimit(_) => "needs live world (stub limit)",
            Verdict::StaticMissing(_) => "missing function (static)",
            Verdict::Panic(_) => "panic",
            Verdict::Refused => "refused",
        }
    }
}

#[derive(Debug, Clone)]
pub struct StageResult {
    pub quest: FormId,
    pub editor: String,
    pub stage: u16,
    pub has_script: bool,
    pub verdict: Verdict,
}

#[derive(Debug, Clone)]
pub struct ObjectiveGap {
    pub editor: String,
    pub objective: i32,
    pub target: FormId,
    pub why: String,
}

#[derive(Debug, Default)]
pub struct LineReport {
    pub quests: usize,
    pub stages: Vec<StageResult>,
    pub objectives: usize,
    pub targets: usize,
    pub gaps: Vec<ObjectiveGap>,
    /// Quest editor id → stages total / clean.
    pub per_quest: BTreeMap<String, (usize, usize)>,
}

impl LineReport {
    /// Stages that run (clean or stopped only by the stub world).
    pub fn clean(&self) -> usize {
        self.stages.iter().filter(|s| s.verdict.runnable()).count()
    }
    pub fn strictly_clean(&self) -> usize {
        self.stages.iter().filter(|s| s.verdict.is_clean()).count()
    }
    pub fn percent(&self) -> f64 {
        if self.stages.is_empty() {
            100.0
        } else {
            100.0 * self.clean() as f64 / self.stages.len() as f64
        }
    }
}

const PLACED: [&[u8; 4]; 7] = [
    b"REFR", b"ACHR", b"ACRE", b"PGRE", b"PMIS", b"PBEA", b"PFLA",
];

/// Walks every quest of every line.
pub fn walk(order: &LoadOrder) -> BTreeMap<Line, LineReport> {
    let handled = handled_set();
    let scripts = ScriptCache::default();
    let mut out: BTreeMap<Line, LineReport> = BTreeMap::new();
    let mut by_line: BTreeMap<Line, Vec<(FormId, String)>> = BTreeMap::new();
    for (id, editor, plugin) in crate::data::quests(order) {
        if let Some(l) = line_of(&plugin, &editor) {
            by_line.entry(l).or_default().push((id, editor));
        }
    }
    // Panics are findings, not noise.
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    for (line, mut quests) in by_line {
        quests.sort_by(|a, b| a.1.cmp(&b.1));
        let mut state = GameState::new(order);
        let rep = out.entry(line).or_default();
        rep.quests = quests.len();
        for (id, editor) in quests {
            let Some(quest) = scripts.quest(order, id) else {
                continue;
            };
            let mut stages: Vec<u16> = quest.stages.iter().map(|s| s.index as u16).collect();
            stages.sort_unstable();
            stages.dedup();
            let entry = rep.per_quest.entry(editor.clone()).or_default();
            for stage in stages {
                let st = quest.stage(stage).expect("listed stage");
                let has_script = st.entries.iter().any(|e| e.script.is_some());
                let verdict = stage_verdict(order, &scripts, &mut state, id, stage, st, &handled);
                entry.0 += 1;
                if verdict.runnable() {
                    entry.1 += 1;
                }
                rep.stages.push(StageResult {
                    quest: id,
                    editor: editor.clone(),
                    stage,
                    has_script,
                    verdict,
                });
            }
            for o in &quest.objectives {
                rep.objectives += 1;
                for t in &o.targets {
                    rep.targets += 1;
                    if let Some(why) = target_problem(order, t.reference) {
                        rep.gaps.push(ObjectiveGap {
                            editor: editor.clone(),
                            objective: o.index,
                            target: t.reference,
                            why,
                        });
                    }
                }
            }
        }
    }
    std::panic::set_hook(hook);
    out
}

pub fn stage_verdict(
    order: &LoadOrder,
    scripts: &ScriptCache,
    state: &mut GameState,
    quest: FormId,
    stage: u16,
    st: &world::quest::Stage,
    handled: &std::collections::HashSet<String>,
) -> Verdict {
    // Static: parse every entry's script and list the functions it calls.
    let mut missing = BTreeSet::new();
    for e in &st.entries {
        let Some(src) = &e.script else { continue };
        match script::parse(src) {
            Err(err) => return Verdict::ParseError(err.to_string()),
            Ok(p) => {
                for f in crate::scan::calls_of(&p) {
                    let name = script::function_name(f);
                    if !handled.contains(&name.to_ascii_lowercase()) {
                        missing.insert(name);
                    }
                }
            }
        }
    }
    let before = state.unhandled.clone();
    let done_before = state.stages_done.contains(&(quest, stage));
    let ran = catch_unwind(AssertUnwindSafe(|| {
        let mut runner = Runner::new(order, scripts, state);
        runner.set_stage(quest, stage)
    }));
    let delta = state_delta(state, &before);
    // One second of game time so the quest scripts and timers run; only a panic counts,
    // as what they ask of the stub world is not this stage's doing.
    let ticked = catch_unwind(AssertUnwindSafe(|| {
        Runner::new(order, scripts, state).update(1.0)
    }));
    if let Err(p) = ticked {
        return Verdict::Panic(panic_text(p));
    }
    match ran {
        Err(p) => Verdict::Panic(panic_text(p)),
        Ok(false) if done_before => Verdict::Clean,
        Ok(false) => Verdict::Refused,
        Ok(true) => {
            let (stub, new): (Vec<String>, Vec<String>) = delta
                .into_iter()
                .partition(|n| handled.contains(&n.to_ascii_lowercase()));
            if !new.is_empty() {
                Verdict::Unhandled(new)
            } else if !stub.is_empty() {
                Verdict::StubLimit(stub)
            } else if !missing.is_empty() {
                Verdict::StaticMissing(missing.into_iter().collect())
            } else {
                Verdict::Clean
            }
        }
    }
}

pub fn panic_text(p: Box<dyn std::any::Any + Send>) -> String {
    p.downcast_ref::<String>()
        .cloned()
        .or_else(|| p.downcast_ref::<&str>().map(|s| s.to_string()))
        .unwrap_or_else(|| "panic".into())
}

/// Functions the host reported unhandled since `before`.
fn state_delta(state: &GameState, before: &BTreeMap<&'static str, usize>) -> Vec<String> {
    state
        .unhandled
        .iter()
        .filter(|(k, n)| before.get(*k).copied().unwrap_or(0) < **n)
        .map(|(k, _)| k.to_string())
        .collect()
}

/// Why an objective target cannot be resolved, if it cannot.
fn target_problem(order: &LoadOrder, reference: FormId) -> Option<String> {
    if reference.0 == 0 {
        return Some("null reference".into());
    }
    let Some(rr) = order.get(reference) else {
        return Some("reference is not in the load order".into());
    };
    let kind = rr.entry.header.kind;
    if !PLACED.iter().any(|p| kind.as_bytes() == *p) {
        return Some(format!("target is a {kind} record, not a placed reference"));
    }
    if order.cell_of(&rr).is_none() {
        return Some("placed reference has no cell".into());
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percent_of_empty_is_full() {
        assert_eq!(LineReport::default().percent(), 100.0);
    }

    #[test]
    fn verdict_tags() {
        assert!(Verdict::Clean.is_clean());
        assert_eq!(Verdict::Refused.tag(), "refused");
    }
}
