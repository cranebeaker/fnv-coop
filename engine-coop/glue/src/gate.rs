//! The guest gate on the upstream engine: six overrides by address.
//!
//! The decision is the adapter's (`coop_engine::gate`). This module puts it in front of the
//! program's script runner and its five functions that change a quest's state. An override is
//! a plain function, so the gate's state is one static. The overrides run on the game's own
//! threads: the state is behind a lock, and the lock is never held across a call into the
//! program (a script that runs can reach a guard).
//!
//! A chained override runs the original translation, from the engine's table of
//! translations. An address with a wrap is refused: an override would remove the wrap.

use std::sync::Mutex;

use coop_engine::gate::{Counters, Gate, Intent, Rules, Verdict};
use coop_engine::{addr, console, script, Program};
use engine::{AbiFn, Engine, Ret};

use crate::EngineProgram;

extern "Rust" {
    /// The engine's translations by address (the upstream `engine` crate defines it).
    fn nv_engine_assembled_funcs(out: &mut Vec<(u32, AbiFn)>);
}

const RUNNER: usize = 0;
const STAGE: usize = 1;
const RUNNING: usize = 2;
const COMPLETED: usize = 3;
const FAILED: usize = 4;
const OBJECTIVE: usize = 5;

/// The addresses that the gate overrides, in the order of the slots above.
pub const ADDRESSES: [u32; 6] = [
    addr::SCRIPT_RUNNER,
    addr::QUEST_SET_STAGE,
    addr::QUEST_SET_RUNNING,
    addr::QUEST_SET_COMPLETED,
    addr::QUEST_SET_FAILED,
    addr::OBJECTIVE_SET_STATE,
];
const OVERRIDES: [AbiFn; 6] = [
    runner,
    set_stage,
    set_running,
    set_completed,
    set_failed,
    objective,
];

struct Installed {
    gate: Gate,
    originals: [AbiFn; 6],
}

static GATE: Mutex<Option<Installed>> = Mutex::new(None);

/// Runs `f` on the installed gate. `None` when no gate is installed.
fn with<R>(f: impl FnOnce(&mut Installed) -> R) -> Option<R> {
    // A panic in game code while the lock is held cannot happen: the lock is never held
    // across a call into the program. A poisoned lock still gives its state.
    GATE.lock()
        .unwrap_or_else(|e| e.into_inner())
        .as_mut()
        .map(f)
}

/// Puts the gate on this game: a guest's game from now on. `Err` names the address that the
/// gate cannot take (it has a wrap, or the engine has no translation for it), and nothing is
/// overridden then.
pub fn install(e: &mut Engine, rules: Rules) -> Result<(), String> {
    let mut table = Vec::new();
    // SAFETY: a plain Rust function that only appends to `table`.
    unsafe { nv_engine_assembled_funcs(&mut table) };
    let mut originals: Vec<AbiFn> = Vec::new();
    for address in ADDRESSES {
        if !e.wraps_at(address).is_empty() {
            return Err(format!(
                "{address:08x} has a wrap: an override would remove it"
            ));
        }
        let original = table
            .iter()
            .find(|(a, _)| *a == address)
            .map(|(_, f)| *f)
            .ok_or(format!("{address:08x} has no translation"))?;
        originals.push(original);
    }
    let originals: [AbiFn; 6] = originals
        .try_into()
        .map_err(|_| "the gate's address list changed".to_string())?;
    *GATE.lock().unwrap_or_else(|e| e.into_inner()) = Some(Installed {
        gate: Gate::new(rules),
        originals,
    });
    for (address, function) in ADDRESSES.iter().zip(OVERRIDES) {
        e.register(*address, function);
    }
    Ok(())
}

/// How many bytes of a quest the start-up check compares: the quest's own fields, to its
/// stage word.
const QUEST_BYTES: usize = addr::QUEST_STAGE as usize + 4;

/// The start-up checks of a gate that is installed. `Err` gives the reason, and the join
/// must not go on then.
/// - The adapter's own console line gives one script at the gate, and it runs.
/// - A change of a shared quest from the program's own code is held, and no byte of the
///   quest changes (`shared_quest`: the address of a quest that the rules do not make local).
///
/// The counters and the intent queue start again after the checks.
pub fn self_check(e: &mut Engine, shared_quest: Option<u32>) -> Result<(), String> {
    let before = counters().ok_or("no gate is installed")?;
    // The line reads a value and changes nothing.
    if !own(|| console::run_line(&mut EngineProgram(e), "GetSecondsPassed", 0)) {
        return Err("the game has no console manager".into());
    }
    let after = counters().unwrap_or_default();
    if after.runs == before.runs {
        return Err("the adapter's console line did not reach the gate".into());
    }
    if after.stopped != before.stopped {
        return Err("the gate stopped the adapter's own console line".into());
    }
    if let Some(quest) = shared_quest {
        let mut was = [0u8; QUEST_BYTES];
        e.mem.read(quest, &mut was);
        let stage = u32::from(was[addr::QUEST_STAGE as usize]);
        let answer = e.call(
            addr::QUEST_SET_STAGE,
            &[quest, stage.wrapping_add(1) & 0xff],
        );
        let mut now = [0u8; QUEST_BYTES];
        e.mem.read(quest, &mut now);
        let guarded = counters().unwrap_or_default().guarded;
        if guarded != after.guarded + 1 {
            return Err("a stage change of a shared quest did not reach the guard".into());
        }
        if now != was {
            return Err("the guard let a stage change of a shared quest through".into());
        }
        if answer.eax & 0xff != 1 {
            return Err("the guard did not answer success".into());
        }
    }
    with(|g| g.gate.clear_counts());
    Ok(())
}

/// One frame of play passed, this many milliseconds long. True when the gate is blind: it
/// saw no script for five seconds of play.
pub fn frame(play_ms: u32) -> bool {
    with(|g| {
        g.gate.frame(play_ms);
        g.gate.blind()
    })
    .unwrap_or(false)
}

/// True when this game has the gate.
pub fn installed() -> bool {
    with(|_| ()).is_some()
}

/// Runs the adapter's own call into the program: what the program runs inside is not gated.
pub fn own<R>(f: impl FnOnce() -> R) -> R {
    with(|g| g.gate.enter_own());
    let result = f();
    with(|g| g.gate.leave_own());
    result
}

/// The intents that wait for the host. The queue is empty after.
pub fn take_intents() -> Vec<Intent> {
    with(|g| g.gate.intents.take()).unwrap_or_default()
}

/// What the gate counted. `None` when no gate is installed.
pub fn counters() -> Option<Counters> {
    with(|g| g.gate.counters)
}

/// The script runner: a script that the rules leave to the host does not run, and the call
/// answers success.
fn runner(e: &mut Engine, args: &[u32]) -> Ret {
    let script = args
        .get(addr::RUNNER_ARG_SCRIPT)
        .map(|at| script::read(&EngineProgram(e), *at));
    let decided = with(|g| {
        let verdict = script.map_or(Verdict::Here, |s| g.gate.script(&s));
        (verdict, g.originals[RUNNER])
    });
    match decided {
        Some((Verdict::Here, original)) => original(e, args),
        // The original answers 1 for a script that ran. A 0 would be a script error.
        Some((Verdict::Host, _)) => Ret {
            eax: 1,
            ..Ret::default()
        },
        // The override is registered only after the gate is installed.
        None => Ret::default(),
    }
}

/// A function that changes a quest's state, with the quest as its first argument. On a
/// shared quest it does nothing and answers `blocked`, and `intent` waits for the host.
fn quest_guard(
    e: &mut Engine,
    args: &[u32],
    slot: usize,
    blocked: u32,
    intent: impl Fn(u32, &[u32]) -> Intent,
) -> Ret {
    let quest = args
        .first()
        .map(|at| EngineProgram(e).u32(at + addr::FORM_ID));
    let decided = with(|g| {
        let verdict = quest.map_or(Verdict::Here, |q| g.gate.quest_change(q, intent(q, args)));
        (verdict, g.originals[slot])
    });
    match decided {
        Some((Verdict::Here, original)) => original(e, args),
        Some((Verdict::Host, _)) => Ret {
            eax: blocked,
            ..Ret::default()
        },
        None => Ret::default(),
    }
}

fn flag(args: &[u32]) -> bool {
    args.get(1).is_some_and(|w| *w as u8 != 0)
}

fn set_stage(e: &mut Engine, args: &[u32]) -> Ret {
    // `SetStage` answers true for a stage that the quest has: the script goes on.
    quest_guard(e, args, STAGE, 1, |quest, a| Intent::Stage {
        quest,
        stage: a.get(1).map_or(0, |w| *w as u8),
    })
}

fn set_running(e: &mut Engine, args: &[u32]) -> Ret {
    quest_guard(e, args, RUNNING, 0, |quest, a| Intent::Running {
        quest,
        on: flag(a),
    })
}

fn set_completed(e: &mut Engine, args: &[u32]) -> Ret {
    quest_guard(e, args, COMPLETED, 0, |quest, a| Intent::Completed {
        quest,
        on: flag(a),
    })
}

fn set_failed(e: &mut Engine, args: &[u32]) -> Ret {
    quest_guard(e, args, FAILED, 0, |quest, a| Intent::Failed {
        quest,
        on: flag(a),
    })
}

/// `BGSQuestObjective::SetState`: the quest is the objective's.
fn objective(e: &mut Engine, args: &[u32]) -> Ret {
    let read = args.first().map(|at| {
        let p = EngineProgram(e);
        let quest = match p.u32(at + addr::OBJECTIVE_QUEST) {
            0 => 0,
            quest => p.u32(quest + addr::FORM_ID),
        };
        (quest, p.u32(at + addr::OBJECTIVE_ID))
    });
    let state = args.get(1).map_or(0, |w| (*w & 3) as u8);
    let decided = with(|g| {
        let verdict = read.map_or(Verdict::Here, |(quest, objective)| {
            g.gate.quest_change(
                quest,
                Intent::Objective {
                    quest,
                    objective,
                    state,
                },
            )
        });
        (verdict, g.originals[OBJECTIVE])
    });
    match decided {
        Some((Verdict::Here, original)) => original(e, args),
        Some((Verdict::Host, _)) | None => Ret::default(),
    }
}
