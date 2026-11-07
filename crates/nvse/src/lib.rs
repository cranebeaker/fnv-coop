//! xNVSE, the New Vegas Script Extender, built in (ADR-0016): its script commands, run from the
//! exe's own command table, and what they run on. Translated from xNVSE 6.4.9 (`0ccd23a`) with
//! its authors' permission; credits and notices in `NOTICE`.
//!
//! What is here, by build step of ADR-0016:
//! - I-2 [`table`]: the script command table in guest memory, as xNVSE builds it (the exe's 0x280
//!   records, xNVSE's padding to `0x1400`, its 654 commands), and the engine's
//!   [`ScriptTables`](engine_core::extend::ScriptTables) moved onto it, the analogue of xNVSE
//!   patching the exe's 19 immediates.
//! - I-3 [`context`]: the script context xNVSE keeps per thread around `ScriptRunner::Run` and
//!   `InitActionList`, and the line `EvaluateLine` is running.
//! - I-4 [`args`]: xNVSE's argument reader (`ExtractArgsEx`), the one its own commands use and,
//!   wrapped over `Script::ParseParameters`, the one vanilla handlers use for extender opcodes; the
//!   result zeroed before an extender command runs.
//! - [`commands`]: the commands built so far, by xNVSE's `Commands_*.cpp` file.
//!
//! [`install`] runs once per engine, after the fix rows (`host::Host::start_with`); the engine
//! crates' own tests never install it, so they run the exe as it is.

pub mod args;
pub mod commands;
pub mod context;
pub mod names;
pub mod table;

use engine_core::prelude::*;

/// The xNVSE version nv-rs reproduces, 6.4.9 (ADR-0016, question 6): `NVSE_VERSION_INTEGER`,
/// `_MINOR`, `_BETA` (`nvse_version.h:6-8` (xNVSE)).
pub const VERSION: (u32, u32, u32) = (6, 4, 9);

/// The eight words a command handler gets (`COMMAND_ARGS` (xNVSE)): the exe's `EvaluateLine`
/// (`005e1550`) and the operand reader (`005ac7a0`) call the handler word of a `CommandInfo`
/// with these, in this order.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CommandArgs {
    /// `ParamInfo *paramInfo`: the command's parameter list (12 bytes each).
    pub params: u32,
    /// `void *scriptData`: the script's compiled data.
    pub data: u32,
    /// `TESObjectREFR *thisObj`: the calling reference, or 0.
    pub this_obj: u32,
    /// `TESObjectREFR *containingObj`.
    pub containing_obj: u32,
    /// `Script *scriptObj`.
    pub script: u32,
    /// `ScriptEventList *eventList`: the script's locals.
    pub event_list: u32,
    /// `double *result`.
    pub result: u32,
    /// `UInt32 *opcodeOffsetPtr`: the address of the word holding the offset of the command's
    /// arguments in `data` (just past its opcode and length words).
    pub opcode_offset: u32,
}

impl CommandArgs {
    /// From a handler's words (missing ones read 0).
    pub fn new(words: &[u32]) -> CommandArgs {
        let w = |i: usize| words.get(i).copied().unwrap_or(0);
        CommandArgs {
            params: w(0),
            data: w(1),
            this_obj: w(2),
            containing_obj: w(3),
            script: w(4),
            event_list: w(5),
            result: w(6),
            opcode_offset: w(7),
        }
    }

    /// The command's opcode: the word four bytes before its arguments, where xNVSE reads it
    /// (`vExtractArgsEx`, `GameAPI.cpp` (xNVSE)).
    pub fn opcode(&self, e: &Engine) -> u32 {
        let offset = e.mem.u32(self.opcode_offset);
        u32::from(e.mem.u16(self.data.wrapping_add(offset).wrapping_sub(4)))
    }

    /// Writes the result double.
    pub fn set_result(&self, e: &mut Engine, value: f64) {
        e.mem.set_f64(self.result, value);
    }

    /// Writes a form result as xNVSE does (`REFR_RES`, `*(UInt32 *)result = refID`): the form's
    /// id in the double's low word, the high word untouched.
    pub fn set_form_result(&self, e: &mut Engine, form_id: u32) {
        e.mem.set_u32(self.result, form_id);
    }
}

/// The four words of a condition's eval (`COMMAND_ARGS_EVAL` (xNVSE)): the exe's condition code
/// (`00681600`) calls the eval word of a `CommandInfo` with these.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct EvalArgs {
    /// `TESObjectREFR *thisObj`.
    pub this_obj: u32,
    /// `void *arg1`, `void *arg2`: the condition's two parameters.
    pub arg1: u32,
    pub arg2: u32,
    /// `double *result`.
    pub result: u32,
}

impl EvalArgs {
    pub fn new(words: &[u32]) -> EvalArgs {
        let w = |i: usize| words.get(i).copied().unwrap_or(0);
        EvalArgs {
            this_obj: w(0),
            arg1: w(1),
            arg2: w(2),
            result: w(3),
        }
    }
}

/// A handler's answer: `true` (the line goes on) in `al`.
pub fn handled(ok: bool) -> Ret {
    Ret {
        eax: u32::from(ok),
        ..Ret::default()
    }
}

/// xNVSE built in on this engine: its state, one slot per engine ([`Engine::ext`]).
#[derive(Default)]
pub struct Nvse {
    /// [`install`] ran.
    pub installed: bool,
    /// The table and the guest data the built-ins keep (I-2).
    pub table: table::State,
    /// The script contexts and the lines being evaluated, per game thread (I-3).
    pub context: context::State,
}

/// Builds xNVSE into the engine, as xNVSE's `CommandTable::Init` and its hooks do at the game's
/// start: the command table (I-2), the script context (I-3), the argument reader and the result
/// zeroing (I-4). Runs after the fix rows, so its wraps wrap whatever is registered. Once per
/// engine; a second call does nothing.
pub fn install(e: &mut Engine) {
    if e.ext::<Nvse>().installed {
        return;
    }
    table::install(e);
    context::install(e);
    args::install(e);
    e.ext::<Nvse>().installed = true;
}

// The unit tests' own program: no other crate's functions are linked in.
#[cfg(test)]
engine_core::assemble!();

#[cfg(test)]
mod tests {
    use super::*;
    use engine_core::extend::Owner;

    fn base(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    /// Installed over the exe's functions, xNVSE's wraps are all listed in the wrap table (the
    /// dispatch check of ADR-0016), its seam handlers sit at their seams, and a second install
    /// changes nothing.
    #[test]
    fn install_wraps_only_listed_addresses_once() {
        let mut e = crate::table::tests::engine_with_exe_tables();
        for addr in [
            0x005e_0d20,
            0x005e_1160,
            0x005e_13a0,
            0x005e_1550,
            0x005a_ccb0,
        ] {
            e.register(addr, base);
        }
        install(&mut e);
        assert!(e.unlisted_wraps().is_empty());
        assert_eq!(e.wrapped().len(), 5);
        for site in [0x005a_cb9d, 0x005e_16c3, 0x005e_2329] {
            assert_eq!(e.site_owners(site), [Owner::Nvse], "{site:08x}");
        }
        let tables = e.script_tables();
        install(&mut e);
        assert_eq!(e.script_tables(), tables);
        assert_eq!(e.wraps_at(0x005e_0d20), [Owner::Nvse]);
    }
}
