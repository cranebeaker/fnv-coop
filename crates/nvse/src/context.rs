//! The script context (ADR-0016 §6; build step I-3): what xNVSE reads from the exe's stack frames
//! while a script runs, kept explicitly, since the port's guest frames do not have the exe's
//! layout.
//!
//! - **The context stack, per game thread** (`CurrentScriptContext`, `Hooks_Other.cpp`
//!   (xNVSE)): pushed and popped by wraps at `ScriptRunner::Run` (`005e0d20`) and
//!   `InitActionList` (`005e1160`) exactly where xNVSE's hooks do. In `Run` the push
//!   (`0x005E0D51`, before the call to `00702fc0`) is the first thing after the prologue,
//!   whatever the script; the pop (`0x005E1137`, the epilogue) only when the script word is
//!   non-null (`PostScriptExecute`). So a `Run` with no script leaves its entry on the stack, as
//!   in the original with xNVSE. In `InitActionList` the push (`0x005E119A`) comes after the
//!   test of the script word, the pop (`0x005E1392`) as in `Run`. An entry holds the script, the
//!   runner, the calling reference (`[ebp+0xC]`), the command being read (set by the argument
//!   reader), the address of the runner's data cursor word (`ebp-0x3C`, here from `ParseLine`
//!   `005e13a0`, which gets it as a parameter) and the line number (`ebp-0x40`, here from
//!   `EvaluateLine`).
//! - **The line being evaluated**, per game thread: a wrap at `EvaluateLine` (`005e1550`)
//!   records the runner, the command, the address of `Run`'s `length` word, the skip word,
//!   the position and the line number; xNVSE finds the runner and the length in the exe's
//!   frames from the command's offset pointer (`GetScriptRunner`, `Commands_Scripting.cpp`
//!   (xNVSE)). The seams before the exe's handler calls note which call is running.
//!
//! The thread is the thread host's current game thread (`thread_host::current_id`).

use crate::Nvse;
use engine_core::extend::{Owner, SiteSlot, WrapSlot};
use engine_core::prelude::*;
use engine_core::thread_host::current_id;
use std::collections::HashMap;

/// `ScriptRunner::Run`.
pub const RUN: WrapSlot = WrapSlot::new(Owner::Nvse, 0x005e_0d20);
/// `ScriptRunner` `InitActionList`.
pub const INIT_ACTION_LIST: WrapSlot = WrapSlot::new(Owner::Nvse, 0x005e_1160);
/// `ScriptRunner::ParseLine`.
pub const PARSE_LINE: WrapSlot = WrapSlot::new(Owner::Nvse, 0x005e_13a0);
/// `ScriptRunner::EvaluateLine`.
pub const EVALUATE_LINE: WrapSlot = WrapSlot::new(Owner::Nvse, 0x005e_1550);
/// The seams before `EvaluateLine`'s block-type handler call (at `005E16C3`, the call at
/// `005E16DC`) and its command handler call (at `005E2329`, the call at `005E2345`), and before
/// the operand reader's handler call (`005ac7a0`, at `005ACB9D`).
pub const BLOCK_HANDLER_CALL: SiteSlot = SiteSlot::new(Owner::Nvse, 0x005e_16c3);
pub const COMMAND_HANDLER_CALL: SiteSlot = SiteSlot::new(Owner::Nvse, 0x005e_2329);
pub const OPERAND_HANDLER_CALL: SiteSlot = SiteSlot::new(Owner::Nvse, 0x005a_cb9d);

/// One entry of the context stack (`CurrentScriptContext` (xNVSE)).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Context {
    pub script: u32,
    pub runner: u32,
    /// The calling reference (`scriptOwnerRef`).
    pub owner_ref: u32,
    /// The record of the command whose arguments were read last (`command`).
    pub command: u32,
    /// The address of the runner's data cursor word (`curDataPtr`); 0 before the first line.
    pub cursor: u32,
    /// The line number of the line being evaluated (`*lineNumberPtr`).
    pub line: u32,
}

/// Which handler call of the exe is running.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HandlerCall {
    #[default]
    None,
    /// `EvaluateLine`'s block-type handler call (`005E16DC`).
    Block,
    /// `EvaluateLine`'s command handler call (`005E2345`, returning to `0x5E234B`).
    Command,
    /// The operand reader's handler call (`005ac7a0`): a command inside an expression.
    Operand,
}

/// The line `EvaluateLine` is evaluating.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Line {
    pub runner: u32,
    pub script: u32,
    /// The line's opcode.
    pub command: u32,
    pub reference: u32,
    /// The address of `Run`'s (or `InitActionList`'s) `length` word: the next line is the cursor
    /// plus this word.
    pub length: u32,
    /// The offset of the line's data, past its opcode and length words.
    pub position: u32,
    /// The address of the skip count word.
    pub skip: u32,
    pub line: u32,
    /// The handler call running, and the record it called through.
    pub call: HandlerCall,
    pub definition: u32,
}

/// The context state, per game thread.
#[derive(Default)]
pub struct State {
    contexts: HashMap<u32, Vec<Context>>,
    lines: HashMap<u32, Vec<Line>>,
}

fn contexts(e: &mut Engine) -> &mut Vec<Context> {
    let thread = current_id(e);
    e.ext::<Nvse>().context.contexts.entry(thread).or_default()
}

fn lines(e: &mut Engine) -> &mut Vec<Line> {
    let thread = current_id(e);
    e.ext::<Nvse>().context.lines.entry(thread).or_default()
}

/// The running script's context on this thread (xNVSE's `GetExecutingScriptContext`).
pub fn current(e: &Engine) -> Option<Context> {
    let thread = current_id(e);
    e.ext_get::<Nvse>()?
        .context
        .contexts
        .get(&thread)?
        .last()
        .copied()
}

/// The contexts on this thread, innermost last.
pub fn stack(e: &Engine) -> Vec<Context> {
    let thread = current_id(e);
    e.ext_get::<Nvse>()
        .and_then(|n| n.context.contexts.get(&thread).cloned())
        .unwrap_or_default()
}

/// The line `EvaluateLine` is evaluating on this thread.
pub fn current_line(e: &Engine) -> Option<Line> {
    let thread = current_id(e);
    e.ext_get::<Nvse>()?
        .context
        .lines
        .get(&thread)?
        .last()
        .copied()
}

/// Notes the command whose arguments are read (`vExtractArgsEx` sets the context's `command`).
pub fn set_command(e: &mut Engine, record: u32) {
    if let Some(top) = contexts(e).last_mut() {
        top.command = record;
    }
}

/// How many contexts this thread has (the running call's depth).
pub fn depth(e: &mut Engine) -> usize {
    contexts(e).len()
}

/// `OtherHooks::PushScriptContext`: a context of xNVSE's own (a single-line lambda's run).
pub fn push(e: &mut Engine, context: Context) {
    contexts(e).push(context);
}

/// `OtherHooks::PopScriptContext`.
pub fn pop(e: &mut Engine) {
    contexts(e).pop();
}

/// The depths of this thread's context and line stacks.
fn depths(e: &mut Engine) -> (usize, usize) {
    (contexts(e).len(), lines(e).len())
}

/// Calls the next link; when it stops (a panic unwinding through it), this thread's context and
/// line stacks go back to the depths `saved` before the stop goes on, as `Engine::call_next`
/// does for its own record, so no entry of the stopped call is left for a later one.
fn call_restoring(e: &mut Engine, addr: u32, a: &[u32], saved: (usize, usize)) -> Ret {
    let ret = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| e.call_next(addr, a)));
    match ret {
        Ok(ret) => ret,
        Err(stop) => {
            contexts(e).truncate(saved.0);
            lines(e).truncate(saved.1);
            std::panic::resume_unwind(stop)
        }
    }
}

/// `ScriptRunner::Run` (`005e0d20`), wrapped: xNVSE's `PreScriptExecute` before it,
/// `PostScriptExecute` after (`Hooks_Other.cpp` (xNVSE)).
fn run(e: &mut Engine, a: &[u32]) -> Ret {
    let (runner, script, reference) = (a[0], a[1], a[2]);
    let saved = depths(e);
    contexts(e).push(Context {
        script,
        runner,
        owner_ref: reference,
        ..Context::default()
    });
    let ret = call_restoring(e, RUN.addr(), a, saved);
    crate::commands::scripting::unwind(e, saved.0);
    if script != 0 {
        contexts(e).pop();
    }
    ret
}

/// `InitActionList` (`005e1160`), wrapped: the push only past the exe's test of the script word.
fn init_action_list(e: &mut Engine, a: &[u32]) -> Ret {
    let (runner, script, reference) = (a[0], a[1], a[2]);
    let saved = depths(e);
    if script != 0 {
        contexts(e).push(Context {
            script,
            runner,
            owner_ref: reference,
            ..Context::default()
        });
    }
    let ret = call_restoring(e, INIT_ACTION_LIST.addr(), a, saved);
    crate::commands::scripting::unwind(e, saved.0);
    if script != 0 {
        contexts(e).pop();
    }
    ret
}

/// `ParseLine` (`005e13a0`; `this, script, opcode, cursor, length, reference, skip`), wrapped:
/// the running context gets the address of its runner's cursor word.
fn parse_line(e: &mut Engine, a: &[u32]) -> Ret {
    let (runner, cursor) = (a[0], a[3]);
    if let Some(top) = contexts(e).last_mut() {
        if top.runner == runner {
            top.cursor = cursor;
        }
    }
    e.call_next(PARSE_LINE.addr(), a)
}

/// `EvaluateLine` (`005e1550`; `this, script, command, reference, length, position, skip,
/// line, parse_only, action_list`), wrapped: the line is recorded while it runs.
fn evaluate_line(e: &mut Engine, a: &[u32]) -> Ret {
    let line = Line {
        runner: a[0],
        script: a[1],
        command: a[2],
        reference: a[3],
        length: a[4],
        position: a[5],
        skip: a[6],
        line: a[7],
        ..Line::default()
    };
    if let Some(top) = contexts(e).last_mut() {
        if top.runner == line.runner {
            top.line = line.line;
        }
    }
    let saved = depths(e);
    lines(e).push(line);
    let ret = call_restoring(e, EVALUATE_LINE.addr(), a, saved);
    lines(e).pop();
    ret
}

/// Notes which handler call runs (the seams' words: the opcode, the record, the result).
fn note_call(e: &mut Engine, call: HandlerCall, definition: u32) {
    if let Some(top) = lines(e).last_mut() {
        top.call = call;
        top.definition = definition;
    }
}

fn block_handler_call(e: &mut Engine, a: &[u32]) -> Option<u32> {
    note_call(e, HandlerCall::Block, a[1]);
    None
}

fn command_handler_call(e: &mut Engine, a: &[u32]) -> Option<u32> {
    note_call(e, HandlerCall::Command, a[1]);
    crate::args::zero_extender_result(e, a);
    None
}

fn operand_handler_call(e: &mut Engine, a: &[u32]) -> Option<u32> {
    note_call(e, HandlerCall::Operand, a[1]);
    crate::args::zero_extender_result(e, a);
    None
}

/// Installs the wraps and the seams' handlers.
pub fn install(e: &mut Engine) {
    e.wrap(RUN, run);
    e.wrap(INIT_ACTION_LIST, init_action_list);
    e.wrap(PARSE_LINE, parse_line);
    e.wrap(EVALUATE_LINE, evaluate_line);
    e.on_site(BLOCK_HANDLER_CALL, block_handler_call);
    e.on_site(COMMAND_HANDLER_CALL, command_handler_call);
    e.on_site(OPERAND_HANDLER_CALL, operand_handler_call);
}

#[cfg(test)]
mod tests {
    use super::*;

    const SCRIPT: u32 = 0x0200_0000;
    const RUNNER: u32 = 0x0200_1000;

    /// What the wrapped functions saw: the context stack inside them.
    fn seen(e: &mut Engine) -> Ret {
        let depth = stack(e).len() as u32;
        let top = current(e).unwrap_or_default();
        e.mem.set_u32(0x0300_0000, depth);
        e.mem.set_u32(0x0300_0004, top.script);
        e.mem.set_u32(0x0300_0008, top.owner_ref);
        e.mem.set_u32(0x0300_000c, top.cursor);
        Ret::default()
    }
    fn base_run(e: &mut Engine, _: &[u32]) -> Ret {
        seen(e)
    }
    fn base_parse(e: &mut Engine, _: &[u32]) -> Ret {
        seen(e)
    }
    fn base_evaluate(e: &mut Engine, _: &[u32]) -> Ret {
        let line = current_line(e).unwrap_or_default();
        e.mem.set_u32(0x0300_0010, line.command);
        e.mem.set_u32(0x0300_0014, line.length);
        e.mem.set_u32(0x0300_0018, line.line);
        Ret::default()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0300_0000, 0x1000);
        e.register(RUN.addr(), base_run);
        e.register(INIT_ACTION_LIST.addr(), base_run);
        e.register(PARSE_LINE.addr(), base_parse);
        e.register(EVALUATE_LINE.addr(), base_evaluate);
        install(&mut e);
        e
    }

    /// `Run` pushes whatever the script and pops only with one, as xNVSE's two hooks do.
    #[test]
    fn run_pushes_always_and_pops_only_with_a_script() {
        let mut e = engine();
        e.call(RUN.addr(), &[RUNNER, SCRIPT, 0x77, 0, 0, 0, 0, 0, 0]);
        assert_eq!(e.mem.u32(0x0300_0000), 1);
        assert_eq!(e.mem.u32(0x0300_0004), SCRIPT);
        assert_eq!(e.mem.u32(0x0300_0008), 0x77);
        assert!(stack(&e).is_empty());
        // No script: the entry stays, as in the original with xNVSE.
        e.call(RUN.addr(), &[RUNNER, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(stack(&e).len(), 1);
    }

    /// `InitActionList` pushes and pops only with a script.
    #[test]
    fn init_action_list_pushes_only_with_a_script() {
        let mut e = engine();
        e.call(INIT_ACTION_LIST.addr(), &[RUNNER, 0, 0, 0]);
        assert_eq!(e.mem.u32(0x0300_0000), 0);
        e.call(INIT_ACTION_LIST.addr(), &[RUNNER, SCRIPT, 5, 0]);
        assert_eq!(e.mem.u32(0x0300_0000), 1);
        assert!(stack(&e).is_empty());
    }

    /// `ParseLine` gives the running context its cursor word; `EvaluateLine` records its line
    /// while it runs.
    #[test]
    fn the_cursor_and_the_line_are_recorded() {
        fn run_one_line(e: &mut Engine, a: &[u32]) -> Ret {
            e.call(PARSE_LINE.addr(), &[a[0], a[1], 0, 0x0300_0100, 0, 0, 0]);
            e.call(
                EVALUATE_LINE.addr(),
                &[a[0], a[1], 0x1400, 0, 0x0300_0108, 4, 0, 7, 0, 0],
            );
            seen(e)
        }
        let mut e = engine();
        e.register(RUN.addr(), run_one_line);
        e.wrap(RUN, run);
        e.call(RUN.addr(), &[RUNNER, SCRIPT, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(e.mem.u32(0x0300_000c), 0x0300_0100);
        assert_eq!(
            (
                e.mem.u32(0x0300_0010),
                e.mem.u32(0x0300_0014),
                e.mem.u32(0x0300_0018)
            ),
            (0x1400, 0x0300_0108, 7)
        );
        assert!(current_line(&e).is_none());
    }

    /// A stop unwinding through `Run` and `EvaluateLine` leaves neither stack deeper than before
    /// the call: the wraps put them back.
    #[test]
    fn a_stop_leaves_the_stacks_as_they_were() {
        fn stopping_evaluate(_: &mut Engine, _: &[u32]) -> Ret {
            panic!("stop");
        }
        fn run_one_line(e: &mut Engine, a: &[u32]) -> Ret {
            e.call(
                EVALUATE_LINE.addr(),
                &[a[0], a[1], 0x1400, 0, 0, 4, 0, 1, 0, 0],
            )
        }
        let mut e = engine();
        e.register(EVALUATE_LINE.addr(), stopping_evaluate);
        e.wrap(EVALUATE_LINE, evaluate_line);
        e.register(RUN.addr(), run_one_line);
        e.wrap(RUN, run);
        let stopped = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            e.call(RUN.addr(), &[RUNNER, SCRIPT, 0, 0, 0, 0, 0, 0, 0]);
        }));
        assert!(stopped.is_err());
        e.call_stack.clear();
        assert!(stack(&e).is_empty());
        assert!(current_line(&e).is_none());
    }
}
