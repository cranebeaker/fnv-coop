//! What the probes read and change in the running program. The program facts are the
//! adapter's (`coop_engine::addr`, each with its source).

use coop_engine::{addr, console, Program};
use coop_engine_glue::EngineProgram;
use engine::Engine;

/// The player's reference and position, or `None` before the game made the player.
pub fn player_place(e: &mut Engine) -> Option<(u32, [f32; 3])> {
    console::player_place(&EngineProgram(e))
}

/// A quest's state, read from its form.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Quest {
    pub at: u32,
    pub stage: u8,
    pub flags: u8,
}

/// The form with this id, or `None`. The game's own lookup finds it.
fn form(e: &mut Engine, id: u32) -> Option<u32> {
    let at = EngineProgram(e).call(addr::FORM_BY_ID, &[id]).eax;
    (at != 0).then_some(at)
}

/// The quest with form id `id`, or `None` when the game has no such form.
pub fn quest(e: &mut Engine, id: u32) -> Option<Quest> {
    let at = form(e, id)?;
    let p = EngineProgram(e);
    Some(Quest {
        at,
        stage: p.u8(at + addr::QUEST_STAGE),
        flags: p.u8(at + addr::QUEST_FLAGS),
    })
}

/// The value of the global with form id `id`, or `None` when the game has no such form.
pub fn global(e: &mut Engine, id: u32) -> Option<f32> {
    let at = form(e, id)?;
    Some(EngineProgram(e).f32(at + addr::GLOBAL_VALUE))
}

/// Sets a stage with the game's own function, which runs the stage's result scripts. `None`
/// when the game has no such quest.
pub fn set_stage(e: &mut Engine, id: u32, stage: u8) -> Option<bool> {
    let at = form(e, id)?;
    Some(
        EngineProgram(e)
            .call(addr::QUEST_SET_STAGE, &[at, u32::from(stage)])
            .bool(),
    )
}

/// The game runs `line` as it runs a line typed in its console, with no calling reference.
/// False when the game has no console manager yet.
pub fn console(e: &mut Engine, line: &str) -> bool {
    console::run_line(&mut EngineProgram(e), line, 0)
}
