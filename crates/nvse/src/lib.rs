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
//! - I-5 [`vars`]: string and array variables, their ids, references and temporaries; the main
//!   loop's tick (the seam at `0086B386`) cleaning the temporaries, and an event list's
//!   variables given up when the game destroys it (`0041af70`, wrapped).
//! - I-6 [`expr`]: the expression evaluator (`Let`, `eval`, `testexpr`, parenthesised
//!   parameters).
//! - I-7 [`commands::scripting`]: `while`, `ForEach`, `ForEachAlt`, `loop`, `continue`, `break`,
//!   `Label`, `Goto`; [`functions`]: user functions (`Begin Function`, `Call`,
//!   `SetFunctionValue`, `GetCallingScript`, `CallFunctionCond`) and the callers events and
//!   delayed calls use; [`lambda`]: lambdas and the event lists kept for them; [`errors`]:
//!   xNVSE's run-time error messages.
//! - I-8 (part) [`tick`]: the main loop's tick (first frame, temporaries, the lists kept for
//!   lambdas) and the delayed calls (`CallAfterSeconds`, `CallAfterFrames`, `CallForSeconds`,
//!   `CallWhile`, `CallWhen`, `CallWhilePerSeconds`).
//! - I-10 [`cosave`], [`saveload`]: the co-save (`.nvse`), written on every save and read before
//!   and after the `.fos` in xNVSE's file format and order (its own plugin: the file list, the
//!   array and string variables), the new-game, delete and rename moments.
//! - I-9 [`events`], [`event_commands`]: xNVSE's event manager (its 60 events, handlers with
//!   filters and priorities, the deprecated and newer dispatch, deferral to the tick), the game
//!   events the game marks (`005ac750`, wrapped), the save, load and exit messages, and the
//!   commands (`SetEventHandler`, `SetEventHandlerAlt`, `RemoveEventHandler`, `DispatchEvent`,
//!   `DispatchEventAlt`, `GetCurrentEventName`, `GetEventHandlers`, the priority queries).
//! - I-12 [`invref`], [`commands::inventory_ref`]: inventory references (a container's stacks
//!   as temporary references, gone each frame and before a load, with their deferred equips and
//!   removals), `ForEach` over a container, `GetSoldItemInvRef` and OnRepair's reference. Of
//!   xNVSE's form extra data only OnCellRefsLoaded's mark is used so far (a set of cells,
//!   [`call_sites`]); its script token cache comes with I-14, its plugin interface with native
//!   plugins.
//! - [`commands`]: the commands built so far, by xNVSE's `Commands_*.cpp` file.
//!
//! [`install`] runs once per engine, after the fix rows (`host::Host::start_with`); the engine
//! crates' own tests never install it, so they run the exe as it is.

pub mod args;
pub mod call_sites;
pub mod cnum;
pub mod commands;
pub mod context;
pub mod cosave;
pub mod errors;
pub mod event_commands;
pub mod events;
pub mod expr;
pub mod functions;
pub mod invref;
pub mod lambda;
pub mod mt19937;
pub mod names;
pub mod saveload;
pub mod table;
pub mod tick;
pub mod vars;

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
    /// The MT19937 generator `Rand` draws from, seeded at install from the clock, as xNVSE seeds
    /// it with `GetTickCount()` when it loads (`nvse.cpp` (xNVSE)).
    pub mt: mt19937::Mt19937,
    /// The string and array variables (I-5).
    pub vars: vars::State,
    /// The running evaluators' expected return types (I-6).
    pub expr: expr::State,
    /// The loop stacks and labels (I-7).
    pub loops: commands::scripting::State,
    /// The user function managers (I-7).
    pub functions: functions::State,
    /// The lambdas (I-7).
    pub lambdas: lambda::State,
    /// The tick and the delayed calls (I-8).
    pub tick: tick::State,
    /// The co-save's file list mapping (I-10).
    pub saveload: saveload::State,
    /// The event manager (I-9).
    pub events: events::State,
    /// The inventory references (I-12), and the stacks `GetFirstRefForItem` listed per script.
    pub invrefs: invref::State,
    pub inv_ref_arrays: commands::inventory_ref::State,
    /// The first frame recorded the main thread (`s_recordedMainThreadID`): xNVSE prints to the
    /// console from then on.
    pub main_thread_recorded: bool,
}

impl Nvse {
    /// Whether xNVSE prints to the console yet (`s_recordedMainThreadID`, set at the first
    /// frame): run-time errors are made only then.
    pub fn prints(e: &Engine) -> bool {
        e.ext_get::<Nvse>().is_some_and(|n| n.main_thread_recorded)
    }
}

/// Builds xNVSE into the engine, as xNVSE's `CommandTable::Init` and its hooks do at the game's
/// start: the command table (I-2), the script context (I-3), the argument reader and the result
/// zeroing (I-4). Runs after the fix rows, so its wraps wrap whatever is registered. Once per
/// engine; a second call does nothing.
pub fn install(e: &mut Engine) {
    if e.ext::<Nvse>().installed {
        return;
    }
    let seed = e.clock_now_ms();
    e.ext::<Nvse>().mt.init_genrand(seed);
    table::install(e);
    context::install(e);
    args::install(e);
    vars::install(e);
    functions::install(e);
    tick::install(e);
    saveload::install(e);
    events::install(e);
    call_sites::install(e);
    e.ext::<Nvse>().installed = true;
    // The extender's state is the game's, not one engine's: real threads' engines share it
    // (engine-core's shared slot, one lock), so an event raised or a variable used on another
    // game thread is the same one.
    e.share_ext::<Nvse>();
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
            0x0041_af70,
            0x005a_c750,
            0x0045_12a0,
        ] {
            e.register(addr, base);
        }
        install(&mut e);
        assert!(e.unlisted_wraps().is_empty());
        assert_eq!(e.wrapped().len(), 8);
        for site in [
            0x0048_3669,
            0x004c_0032,
            0x0052_9a37,
            0x0052_9d37,
            0x0054_9787,
            0x0055_18a5,
            0x0057_1090,
            0x0057_10ac,
            0x0057_318e,
            0x0057_515a,
            0x005c_453c,
            0x005c_47b6,
            0x0072_fe3e,
            0x0072_ff20,
            0x0076_717a,
            0x0076_74b0,
            0x0078_f8e5,
            0x0078_fe24,
            0x0079_0461,
            0x007b_5cac,
            0x007b_800e,
            0x0086_80a4,
            0x0086_fe75,
            0x0088_caeb,
            0x0088_cb97,
            0x0088_cd11,
            0x005a_cb9d,
            0x005b_6ca6,
            0x005e_16c3,
            0x005e_2329,
            0x007d_0b17,
            0x007d_0c3e,
            0x007d_33ce,
            0x0084_7c45,
            0x0084_7fd9,
            0x0084_8c3c,
            0x0084_8c91,
            0x0085_0398,
            0x0085_762b,
            0x0086_b386,
            0x0093_db69,
        ] {
            assert_eq!(e.site_owners(site), [Owner::Nvse], "{site:08x}");
        }
        let tables = e.script_tables();
        install(&mut e);
        assert_eq!(e.script_tables(), tables);
        assert_eq!(e.wraps_at(0x005e_0d20), [Owner::Nvse]);
    }
}
