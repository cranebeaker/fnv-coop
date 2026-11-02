//! The game's own interpreter as the world's quest runner
//! ([`world::scripting::QuestVm`]).
//!
//! For the quests it takes ([`takes`]), a quest's `GameMode` run goes
//! through the game's interpreter in the host ([`QuestForm::run`]) instead
//! of `crates/script`: the quest and its script are made in the game's
//! memory on their first run ([`make_quest`]), the world's variables are
//! copied into the quest's `ScriptLocals` before each run and back after,
//! and the world's `GetSecondsPassed` is what the game's reads.
//!
//! `GetSecondsPassed` (`005c0dd0` → `Script::GetSecondsPassedConditionFunction`
//! `0059c430`) gives a quest script's get-seconds buffer (`Script` +0x3C)
//! when it's above 0, else the frame's elapsed time, the float at +0xC of
//! `011f6394` (`0084d030`). The world keeps the quests' run schedule (their
//! delays, [`world::scripting::Runner::update`]) and passes the time since a
//! quest's last run; so before a run the script's delay (`Script` +0x38,
//! which `Script::Run` `005ac1e0` counts down and, while above 0, doesn't
//! run) and buffer are cleared and the elapsed time is set to that time.
//!
//! What it takes: quests whose script has `GameMode` blocks only, whose
//! variables are all numbers (no `ref`), whose lines use only the script's
//! own variables, and which call no function but `GetSecondsPassed`. The
//! game's command handlers aren't connected to the world yet, so a script
//! that calls one runs through `crates/script` as before. A quest whose
//! making or run stops (an untranslated function, a missing object) goes
//! back to `crates/script` for good, and is counted.

use crate::game_file::GameFile;
use crate::quest_form::{QuestForm, QuestState};
use crate::script_form::{read_locals, write_locals, ScriptForm};
use engine::Engine;
use esm::{FormId, LoadOrder};
use script::interp::Locals;
use script::{Expr, Item, Script, Stmt, VarKind};
use std::collections::HashMap;

/// The frame's elapsed time that `GetSecondsPassed` reads when a quest
/// script's buffer is 0: the float at +0xC of `011f6394` (`0084d030`).
const ELAPSED_TIME: u32 = 0x011f_6394 + 0xc;
/// `Script` (Xbox PDB) `fQuestScriptDelay` (+0x38) and
/// `fQuestScriptGetSecondsBuffer` (+0x3C).
const SCRIPT_DELAY: u32 = 0x38;
const SCRIPT_SECONDS_BUFFER: u32 = 0x3c;

/// Whether the game's interpreter takes the quest whose script is `parsed`
/// (see the module).
pub fn takes(parsed: &Script) -> bool {
    let seconds = script::function("GetSecondsPassed").map(|(n, _)| n);
    let only_seconds = |call: &script::Call| {
        Some(call.function) == seconds && call.on.is_none() && call.args.is_empty()
    };
    let expr_ok = |e: &Expr| {
        e.0.iter().all(|item| match item {
            Item::Number(_) | Item::Op(_) => true,
            Item::Var(path) => path.len() == 1,
            Item::Call(call) => only_seconds(call),
            Item::Str(_) => false,
        })
    };
    fn stmts_ok(
        body: &[Stmt],
        expr_ok: &dyn Fn(&Expr) -> bool,
        call_ok: &dyn Fn(&script::Call) -> bool,
    ) -> bool {
        body.iter().all(|s| match s {
            Stmt::Set { target, value } => target.len() == 1 && expr_ok(value),
            Stmt::If {
                branches,
                otherwise,
            } => {
                branches
                    .iter()
                    .all(|(c, b)| expr_ok(c) && stmts_ok(b, expr_ok, call_ok))
                    && match otherwise {
                        Some(b) => stmts_ok(b, expr_ok, call_ok),
                        None => true,
                    }
            }
            Stmt::Return => true,
            Stmt::Call(call) => call_ok(call),
        })
    }
    !parsed.blocks.is_empty()
        && parsed.body.is_empty()
        && parsed.variables.iter().all(|(k, _)| *k != VarKind::Ref)
        && parsed.blocks.iter().all(|b| {
            b.kind == "gamemode" && b.args.is_empty() && stmts_ok(&b.body, &expr_ok, &only_seconds)
        })
}

/// How many of the load order's quests with a script the game's
/// interpreter takes ([`takes`]), and how many have a script.
pub fn eligible(order: &LoadOrder, scripts: &world::scripting::ScriptCache) -> (usize, usize) {
    let (mut taken, mut scripted) = (0, 0);
    for rr in order.records_of_type(esm::FourCC(*b"QUST")) {
        let Some(id) = world::scripting::script_of(order, rr.form_id) else {
            continue;
        };
        let Some(parsed) = scripts.script(order, id) else {
            continue;
        };
        scripted += 1;
        if takes(&parsed) {
            taken += 1;
        }
    }
    (taken, scripted)
}

/// A quest made in the game's memory, and its script.
struct Made {
    script: ScriptForm,
    quest: QuestForm,
}

/// The plugins' `TESFile`s in the game's memory, made on first need, by
/// load-order index ([`GameFile::open`]).
pub type GameFiles = HashMap<u8, GameFile>;

/// Makes the quest `quest` with its script `script` in the game's memory,
/// the one place quests are made: the script with [`ScriptForm::load`]
/// (`Script::Load` is still bridged there) into the game's form map, then
/// the quest from its record by the game's own loaders ([`QuestForm::load`]:
/// `TESFile::SetOffset`, `TESQuest::Load`, `InitItem`, which resolves the
/// script). `files` keeps the plugins' `TESFile`s.
pub fn make_quest(
    e: &mut Engine,
    files: &mut GameFiles,
    order: &LoadOrder,
    quest: FormId,
    script: FormId,
) -> Result<(ScriptForm, QuestForm), String> {
    let script_ref = order
        .get(script)
        .ok_or_else(|| format!("script {script}: not in the load order"))?;
    let script_record = script_ref.record().map_err(|err| err.to_string())?;
    let plugin = script_ref.plugin;
    let script_form = ScriptForm::load(e, script, &script_record, |id| plugin.to_global(id))?;
    let quest_ref = order
        .get(quest)
        .ok_or_else(|| format!("quest {quest}: not in the load order"))?;
    let index = quest_ref.plugin.load_index;
    let file = match files.get(&index) {
        Some(file) => *file,
        None => {
            let file = GameFile::open(e, quest_ref.plugin)?;
            files.insert(index, file);
            file
        }
    };
    let quest_form = QuestForm::load(e, &file, quest_ref.plugin, quest_ref.entry, quest)?;
    if quest_form.script != script_form.form {
        return Err(format!(
            "quest {quest}: InitItem resolved its script to {:08x}, not {script}'s {:08x}",
            quest_form.script, script_form.form
        ));
    }
    Ok((script_form, quest_form))
}

/// The quests run through the game's interpreter, and what happened.
pub struct QuestVm {
    /// Which quests it takes, by their script: [`takes`] (tests may take
    /// more, to run a script's command-free part).
    pub accept: fn(&Script) -> bool,
    /// Each quest offered: made (or not taken: `None`).
    quests: HashMap<FormId, Option<Made>>,
    /// The plugins' `TESFile`s ([`make_quest`]).
    files: GameFiles,
    /// Runs through the game's interpreter.
    pub runs: u64,
    /// Quests that stopped while made or run, with why; they run through
    /// `crates/script` from then on.
    pub stopped: Vec<(FormId, String)>,
}

impl Default for QuestVm {
    fn default() -> Self {
        QuestVm {
            accept: takes,
            quests: HashMap::new(),
            files: HashMap::new(),
            runs: 0,
            stopped: Vec::new(),
        }
    }
}

impl QuestVm {
    /// How many quests have run through the game's interpreter.
    pub fn quests_run(&self) -> usize {
        self.quests.values().filter(|m| m.is_some()).count()
    }

    /// The diagnostic line: how many quests ran through the game's
    /// interpreter, how many runs, and how many stopped.
    pub fn summary(&self) -> String {
        format!(
            "{} quests ran through the game's interpreter ({} runs); {} went back to \
             crates/script after stopping.",
            self.quests_run(),
            self.runs,
            self.stopped.len()
        )
    }

    /// The quest's state in the game's memory, when it was made.
    pub fn state(&self, e: &mut Engine, quest: FormId) -> Option<QuestState> {
        let made = self.quests.get(&quest)?.as_ref()?;
        Some(made.quest.snapshot(e, &made.script))
    }

    /// Makes the quest `quest` (with its script) in the game's memory now
    /// ([`make_quest`]), whatever [`QuestVm::accept`] says, and remembers
    /// it: later runs of it use this one.
    pub fn make(
        &mut self,
        e: &mut Engine,
        order: &LoadOrder,
        quest: FormId,
    ) -> Result<(ScriptForm, QuestForm), String> {
        let script = world::scripting::script_of(order, quest)
            .ok_or_else(|| format!("quest {quest}: no script"))?;
        let (script_form, quest_form) = make_quest(e, &mut self.files, order, quest, script)?;
        self.quests.insert(
            quest,
            Some(Made {
                script: script_form.clone(),
                quest: quest_form,
            }),
        );
        Ok((script_form, quest_form))
    }

    /// [`world::scripting::QuestVm::run_quest`] on the engine `e`.
    #[allow(clippy::too_many_arguments)]
    pub fn run_quest(
        &mut self,
        e: &mut Engine,
        order: &LoadOrder,
        quest: FormId,
        script: FormId,
        parsed: &Script,
        locals: &mut Locals,
        seconds_passed: f32,
    ) -> bool {
        if !self.quests.contains_key(&quest) {
            let made = if (self.accept)(parsed) {
                let mark = e.guest_stack_mark();
                match guarded(|| make_quest(e, &mut self.files, order, quest, script)) {
                    Ok(Ok((script, quest))) => Some(Made { script, quest }),
                    Ok(Err(why)) => {
                        self.stopped.push((quest, why));
                        None
                    }
                    Err(why) => {
                        // The engine goes on with other quests: the stop's locals go back.
                        e.guest_stack_restore(mark);
                        self.stopped.push((quest, why));
                        None
                    }
                }
            } else {
                None
            };
            self.quests.insert(quest, made);
        }
        let Some(Some(made)) = self.quests.get(&quest) else {
            return false;
        };
        let (script_form, quest_form) = (made.script.clone(), made.quest);
        let mark = e.guest_stack_mark();
        let run = guarded(|| {
            let game_locals = quest_form.locals(e);
            write_locals(e, &script_form, game_locals, locals);
            e.mem.set_u32(script_form.form + SCRIPT_DELAY, 0);
            e.mem.set_u32(script_form.form + SCRIPT_SECONDS_BUFFER, 0);
            e.mem.set_u32(ELAPSED_TIME, seconds_passed.to_bits());
            quest_form.run(e);
            let mut back = locals.clone();
            read_locals(e, &script_form, game_locals, &mut back);
            back
        });
        match run {
            Ok(back) => {
                *locals = back;
                self.runs += 1;
                true
            }
            Err(why) => {
                e.guest_stack_restore(mark);
                self.stopped.push((quest, why));
                self.quests.insert(quest, None);
                false
            }
        }
    }
}

/// Runs `f`; a stop in the game's code (the engine panics: an untranslated
/// function, an unmapped read) comes back as its first line.
fn guarded<R>(f: impl FnOnce() -> R) -> Result<R, String> {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let out = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)).map_err(|payload| {
        let text = payload
            .downcast_ref::<String>()
            .cloned()
            .or_else(|| payload.downcast_ref::<&str>().map(|s| s.to_string()))
            .unwrap_or_default();
        text.lines().next().unwrap_or("").to_string()
    });
    std::panic::set_hook(hook);
    out
}

impl world::scripting::QuestVm for crate::Host {
    fn run_quest(
        &mut self,
        order: &LoadOrder,
        quest: FormId,
        script: FormId,
        parsed: &Script,
        locals: &mut Locals,
        seconds_passed: f32,
    ) -> bool {
        // The engine is main's while a frame of it is in flight.
        self.join();
        self.quests.run_quest(
            &mut self.engine,
            order,
            quest,
            script,
            parsed,
            locals,
            seconds_passed,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(source: &str) -> Script {
        script::parse(source).unwrap()
    }

    #[test]
    fn takes_command_free_quests_and_get_seconds_passed_only() {
        assert!(takes(&parse(
            "scn A\nfloat fTimer\nshort bInit\nBegin GameMode\nif (bInit == 0)\nset bInit to 1\nset fTimer to 1.0\nendif\nif (fTimer > 0)\nset fTimer to fTimer - GetSecondsPassed\nendif\nEnd"
        )));
        // A command, a reference variable, another script's variable, a
        // block other than GameMode, or nothing to run: not taken.
        assert!(!takes(&parse(
            "scn A\nshort b\nBegin GameMode\nset b to 1\nStopQuest A\nEnd"
        )));
        assert!(!takes(&parse(
            "scn A\nref r\nBegin GameMode\nset r to player\nEnd"
        )));
        assert!(!takes(&parse(
            "scn A\nshort b\nBegin GameMode\nset b to OtherQuest.x\nEnd"
        )));
        assert!(!takes(&parse(
            "scn A\nshort b\nBegin GameMode\nset OtherQuest.x to 1\nEnd"
        )));
        assert!(!takes(&parse(
            "scn A\nshort b\nBegin MenuMode\nset b to 1\nEnd"
        )));
        assert!(!takes(&parse(
            "scn A\nshort b\nBegin GameMode\nif GetStage A == 10\nset b to 1\nendif\nEnd"
        )));
        assert!(!takes(&parse("scn A\nshort b")));
    }
}
