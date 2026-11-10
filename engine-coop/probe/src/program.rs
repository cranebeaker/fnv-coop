//! The running game program by address (FalloutNV.exe 1.4.0.525): the few facts the probes use.
//!
//! Each constant names its source. `legacy:` is the project's own plugin (branch `legacy-xnvse`,
//! MIT). `ghidra:` is our analysis of the program.

use engine::Engine;

/// The player (`PlayerCharacter**`, reference 0x14).
// legacy: plugin/avatar/engine/engine_pins.h (`kThePlayerPtr`).
const PLAYER: u32 = 0x011d_ea3c;
/// Where a reference keeps its position (`TESObjectREFR` posX/Y/Z, three floats).
// legacy: docs/engine-addresses.md, section 3.
const POSITION: u32 = 0x30;
/// `LookupFormByID`: cdecl (form id), the form or 0.
// legacy: docs/engine-addresses.md, section 2.
const FORM_BY_ID: u32 = 0x0048_39c0;
/// `TESQuest::SetStage`: method (quest, stage), true when the stage exists.
// legacy: docs/engine-addresses.md, section 2.
const QUEST_SET_STAGE: u32 = 0x0060_d510;
/// A quest's flags: bit 0 running, bit 1 completed.
// ghidra: 0060ca30 (`TESQuest::SetCompleted` tests and sets bit 1 of the byte at +0x3c).
const QUEST_FLAGS: u32 = 0x3c;
/// A quest's current stage, one byte.
// legacy: docs/engine-addresses.md, section 2 (the note of `TESQuest::SetStage`).
const QUEST_STAGE: u32 = 0x60;
/// A global's value (`TESGlobal`, form type 6): one float.
// legacy: docs/engine-addresses.md, section 3 (the getter 0x526AC0 reads the float at +0x24).
const GLOBAL_VALUE: u32 = 0x24;
/// The console manager: cdecl (create), the manager or 0. Its first word is the context that
/// the console's scripts compile in.
// ghidra: 0071b160.
const CONSOLE_MANAGER: u32 = 0x0071_b160;
// How the game runs a line typed in its console (ghidra: 0071b210, the console's key handler,
// at its two calls of `Script::Run`): it makes a `Script` on the stack, marks the form
// temporary, sets its text, runs it in the manager's context, and destroys it.
/// `Script`'s constructor: method (script).
// ghidra: 005aa0f0 (sets `Script::vftable`, then two members at +0x44 and +0x4c).
const SCRIPT_NEW: u32 = 0x005a_a0f0;
/// Bytes for a `Script`: the constructor's last member is 8 bytes at +0x4c.
const SCRIPT_BYTES: u32 = 0x54;
/// Marks a form temporary (flag 0x4000 at +8): method (form).
// ghidra: 00484490.
const FORM_SET_TEMPORARY: u32 = 0x0048_4490;
/// Sets a script's text (a copy, kept at +0x2c): method (script, text).
// ghidra: 005abe50.
const SCRIPT_SET_TEXT: u32 = 0x005a_be50;
/// Compiles the script's text and executes it: method (script, context, 1, calling reference).
// legacy: docs/engine-addresses.md, section 2 (`Script::Run`). ghidra: 005ac400.
const SCRIPT_RUN: u32 = 0x005a_c400;
/// `Script`'s destructor (it frees the text and the compiled data, not the object): method
/// (script).
// ghidra: 005aa1a0 (the console's key handler calls it on its stack object).
const SCRIPT_DROP: u32 = 0x005a_a1a0;

/// The player's reference and position, or `None` before the game made the player.
pub fn player_place(e: &Engine) -> Option<(u32, [f32; 3])> {
    let player = e.mem.u32(PLAYER);
    (player != 0).then(|| (player, [0, 4, 8].map(|o| e.mem.f32(player + POSITION + o))))
}

/// A quest's state, read from its form.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Quest {
    pub at: u32,
    pub stage: u8,
    pub flags: u8,
}

/// The form with this id, or `None`. It calls the game's own lookup, so it needs `&mut`.
pub fn form(e: &mut Engine, id: u32) -> Option<u32> {
    let at = e.call(FORM_BY_ID, &[id]).u32();
    (at != 0).then_some(at)
}

/// The state of the quest whose form is at `at`. It reads only memory.
pub fn quest_at(e: &Engine, at: u32) -> Quest {
    Quest {
        at,
        stage: e.mem.u8(at + QUEST_STAGE),
        flags: e.mem.u8(at + QUEST_FLAGS),
    }
}

/// The quest with form id `id`, or `None` when the game has no such form. The game's own
/// lookup finds the form, so this needs `&mut`.
pub fn quest(e: &mut Engine, id: u32) -> Option<Quest> {
    form(e, id).map(|at| quest_at(e, at))
}

/// The value of the global with form id `id`, or `None` when the game has no such form.
pub fn global(e: &mut Engine, id: u32) -> Option<f32> {
    form(e, id).map(|at| e.mem.f32(at + GLOBAL_VALUE))
}

/// Sets a stage with the game's own function, which runs the stage's result scripts. `None`
/// when the game has no such quest.
pub fn set_stage(e: &mut Engine, id: u32, stage: u8) -> Option<bool> {
    let at = form(e, id)?;
    Some(e.call(QUEST_SET_STAGE, &[at, u32::from(stage)]).bool())
}

/// The game runs `line` as it runs a line typed in its console, with no calling reference.
/// False when the game has no console manager yet.
pub fn console(e: &mut Engine, line: &str) -> bool {
    let manager = e.call(CONSOLE_MANAGER, &[1]).u32();
    if manager == 0 {
        return false;
    }
    let context = e.mem.u32(manager);
    let bytes = line.as_bytes();
    let text = e.mem.alloc(bytes.len() as u32 + 1);
    e.mem.set_cstr(text, bytes);
    let script = e.mem.alloc(SCRIPT_BYTES);
    e.mem.zero(script, SCRIPT_BYTES);
    e.call(SCRIPT_NEW, &[script]);
    e.call(FORM_SET_TEMPORARY, &[script]);
    e.call(SCRIPT_SET_TEXT, &[script, text]);
    e.call(SCRIPT_RUN, &[script, context, 1, 0]);
    e.call(SCRIPT_DROP, &[script]);
    e.mem.free(script);
    e.mem.free(text);
    true
}
