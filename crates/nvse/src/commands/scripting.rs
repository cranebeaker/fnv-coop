//! xNVSE's scripting core (`Commands_Scripting.cpp`, `Loops.cpp` (xNVSE)): `Let`, `eval` and
//! `testexpr` on the expression evaluator (I-6), and the loops and labels (I-7): `while`,
//! `ForEach`, `ForEachAlt`, `loop`, `continue`, `break`, `Label`, `Goto`.
//!
//! The loops move the script runner as xNVSE does: through the running line's `length` word
//! (`GetCalculatedOpLength`, here the line record of [`crate::context`]), and they save and
//! restore the runner's if/else stack (`ifStackDepth` `+0x20`, `ifStack` `+0x24`, ten deep;
//! `LoopManager::RestoreStack`). The loop stack is per game thread, as xNVSE's thread-local
//! `LoopManager`.
//!
//! `ForEach` over a container goes through its stacks as inventory references (I-12). The
//! user functions and lambdas are [`crate::functions`] and [`crate::lambda`].

use crate::expr::{self, Eval, Tok};
use crate::table::{Command, Param, Returns};
use crate::vars::{self, Key};
use crate::{handled, CommandArgs, Nvse};
use engine_core::prelude::*;
use engine_core::thread_host::current_id;
use std::collections::HashMap;

/// `kNVSEParamType_BasicType`, `_Boolean`, `_NoTypeCheck` (xNVSE `ScriptUtils.h`): bits of
/// token types, with `Ambiguous` (bit 15).
const BASIC_TYPE: u32 = 0x804d;
const BOOLEAN: u32 = 0x8002;
const NO_TYPE_CHECK: u32 = 0;

const fn p(name: &'static str, kind: u32) -> Param {
    Param {
        name,
        kind,
        optional: false,
    }
}
const ONE_BASIC_TYPE: &[Param] = &[p("expression", BASIC_TYPE)];
const ONE_BOOLEAN: &[Param] = &[p("boolean expression", BOOLEAN)];
const NO_TYPE_CHECKING: &[Param] = &[p("expression", NO_TYPE_CHECK)];
const FOREACH_CONTEXT: &[Param] = &[p("ForEach expression", 1 << 17)];
/// `kParams_ForEachAlt`: the array (`kNVSEParamType_Array`), the value variable
/// (`_Variable | _OptionalEmpty`), the optional key variable (`_StringVar | _NumericVar |
/// _OptionalEmpty`).
const FOREACH_ALT: &[Param] = &[
    p("sourceArray", 0x8040),
    p("valueVariable", 0x0800_0400),
    Param {
        name: "keyVariable",
        kind: 0x0800_2800,
        optional: true,
    },
];
const ONE_INT: &[Param] = &[Param {
    name: "int",
    kind: 0x01,
    optional: true,
}];

const fn cmd(
    opcode: u32,
    name: &'static str,
    help: &'static str,
    params: &'static [Param],
    execute: AbiFn,
) -> Command {
    Command {
        opcode,
        name,
        short: "",
        help,
        needs_ref: false,
        params,
        execute,
        eval: None,
        returns: Returns::Default,
    }
}

pub static COMMANDS: &[Command] = &[
    cmd(0x149e, "Label", "set a label", ONE_INT, label),
    cmd(0x149f, "Goto", "branch to a label", ONE_INT, goto),
    cmd(
        0x1539,
        "Let",
        "assigns the value of an expression to a variable",
        ONE_BASIC_TYPE,
        let_execute,
    ),
    cmd(
        0x153a,
        "eval",
        "evaluates an expression and returns a boolean result.",
        ONE_BOOLEAN,
        eval_execute,
    ),
    cmd(
        0x153b,
        "while",
        "loops until the given condition evaluates to false",
        ONE_BOOLEAN,
        while_execute,
    ),
    cmd(
        0x153c,
        "loop",
        "marks the end of a While or ForEach loop",
        &[],
        loop_execute,
    ),
    cmd(
        0x153d,
        "ForEach",
        "iterates over the elements of an array",
        FOREACH_CONTEXT,
        foreach_execute,
    ),
    cmd(
        0x153e,
        "continue",
        "returns control to the top of a loop",
        &[],
        continue_execute,
    ),
    cmd(0x153f, "break", "exits a loop", &[], break_execute),
    cmd(
        0x1542,
        "testexpr",
        "returns false if errors occur while evaluating expression, true otherwise",
        NO_TYPE_CHECKING,
        testexpr_execute,
    ),
    cmd(
        0x1670,
        "ForEachAlt",
        "iterates over the elements of an array, passing values directly into variables",
        FOREACH_ALT,
        foreach_alt_execute,
    ),
];

/// Runs the evaluator over a command's arguments (`ExpressionEvaluator(PASS_COMMAND_ARGS)` and
/// `ExtractArgs`), reporting its errors when it ends.
fn evaluate_args(e: &mut Engine, a: &CommandArgs, suppress: bool) -> (Option<Vec<Tok>>, Eval) {
    let mut ev = Eval::new(e, a);
    ev.suppress_errors = suppress;
    let toks = expr::with_evaluator(e, |e| ev.extract_args(e));
    expr::report(e, &ev);
    (toks, ev)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Scripting.cpp: Cmd_Let_Execute.
fn let_execute(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    evaluate_args(e, &a, false);
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Scripting.cpp: Cmd_eval_Execute.
fn eval_execute(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    let (toks, ev) = evaluate_args(e, &a, false);
    if let Some(first) = toks.as_ref().and_then(|t| t.first()) {
        let v = ev.get_bool(e, first);
        a.set_result(e, f64::from(u8::from(v)));
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Scripting.cpp:
// Cmd_testexpr_Execute.
fn testexpr_execute(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    let (toks, ev) = evaluate_args(e, &a, true);
    if let Some(first) = toks.as_ref().and_then(|t| t.first()) {
        if ev.errors.is_empty() {
            let v = match first {
                Tok::Element { array, key } => {
                    let upper = vars::upper_table(e);
                    vars::array(e, *array).is_some_and(|arr| arr.get(&upper, key).is_some())
                }
                _ => true,
            };
            a.set_result(e, f64::from(u8::from(v)));
        }
    }
    handled(true)
}

/// The running script line's runner and `length` word (`GetScriptRunner`,
/// `GetCalculatedOpLength`).
fn runner_and_length(e: &Engine) -> Option<(u32, u32)> {
    crate::context::current_line(e).map(|l| (l.runner, l.length))
}

/// The runner's if/else stack (`SavedIPInfo`): its depth and the words up to it.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SavedIp {
    pub ip: u32,
    pub depth: u32,
    pub stack: Vec<u32>,
}

fn save_stack(e: &Engine, runner: u32, ip: u32) -> SavedIp {
    let depth = e.mem.u32(runner + 0x20);
    let stack = (0..=depth.min(10))
        .map(|i| e.mem.u32(runner + 0x24 + 4 * i))
        .collect();
    SavedIp { ip, depth, stack }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Loops.cpp: LoopManager::RestoreStack.
fn restore_stack(e: &mut Engine, runner: u32, info: &SavedIp) {
    e.mem.set_u32(runner + 0x20, info.depth);
    for (i, w) in info.stack.iter().enumerate() {
        e.mem.set_u32(runner + 0x24 + 4 * i as u32, *w);
    }
}

/// Moves the runner to `target`: `*calculatedOpLength += target - *opcodeOffsetPtr`.
fn jump(e: &mut Engine, length: u32, offset_ptr: u32, target: u32) {
    let now = e.mem.u32(offset_ptr);
    let v = e.mem.u32(length).wrapping_add(target.wrapping_sub(now));
    e.mem.set_u32(length, v);
}

/// A loop (`Loop` and its kinds, xNVSE `Loops.h`).
#[derive(Clone, Debug, PartialEq)]
pub enum Loop {
    /// `WhileLoop`: the offset of its condition.
    While { expr_offset: u32 },
    /// `ArrayIterLoop` with an iterator array (`iter["key"]`, `iter["value"]`): the source,
    /// the iterator, the iterator variable's local and the current key.
    Array {
        source: u32,
        iter: u32,
        local: u32,
        key: Option<Key>,
        owner_mod: u8,
    },
    /// `StringIterLoop`: the source text, the iterator string, the index.
    Str {
        text: Vec<u8>,
        iter: u32,
        index: usize,
    },
    /// `ArrayIterLoop` of `ForEachAlt`: the source and the current key, the value variable (its
    /// local, 0 when invalid, and type) and the key variable when one was given, and the
    /// script (whose mod owns the strings made for them).
    Alt {
        source: u32,
        key: Option<Key>,
        value_var: (u32, u8),
        key_var: Option<(u32, u8)>,
        script: u32,
    },
    /// `FormListIterLoop`: the list node (0 at the end) and the reference variable's local.
    FormList {
        node: u32,
        local: u32,
        /// The nodes walked so far ([`crate::invref::list_step`]).
        walked: usize,
    },
    /// `ContainerIterLoop`: its inventory reference (by the temporary reference's id), the
    /// reference variable's local, the stacks (changes entries of its own) and the index.
    Container {
        invref: u32,
        local: u32,
        elements: Vec<u32>,
        index: usize,
    },
}

/// `LoopInfo`.
#[derive(Clone, Debug, PartialEq)]
pub struct LoopInfo {
    pub kind: Loop,
    pub saved: SavedIp,
    pub end_ip: u32,
    /// The depth of the script context stack when the loop began: the call it belongs to.
    pub depth: usize,
}

/// The loop managers (per thread) and the labels (`s_savedIPTable`, by script form id).
#[derive(Default)]
pub struct State {
    loops: HashMap<u32, Vec<LoopInfo>>,
    labels: HashMap<u32, HashMap<u32, SavedIp>>,
}

fn loops(e: &mut Engine) -> &mut Vec<LoopInfo> {
    let thread = current_id(e);
    e.ext::<Nvse>().loops.loops.entry(thread).or_default()
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Loops.cpp: LoopManager::Add.
fn add_loop(e: &mut Engine, kind: Loop, runner: u32, start: u32, end: u32) {
    let saved = save_stack(e, runner, start);
    let depth = crate::context::depth(e);
    loops(e).push(LoopInfo {
        kind,
        saved,
        end_ip: end,
        depth,
    });
}

/// A script run ended (the `Run` and `InitActionList` wraps, back at context depth `depth`):
/// loops it began and never ended (`return` inside a loop, or the script's end) go with it.
///
/// xNVSE 6.4.9 never takes such loops off its per-thread stack (nothing but `Break` pops
/// `LoopManager::m_loops`), so a `loop` or `break` in the calling script would act on the
/// callee's loop with the callee's offsets. A fix: the loops are tied to the call they began
/// in.
pub fn unwind(e: &mut Engine, depth: usize) {
    while loops(e).last().is_some_and(|l| l.depth > depth) {
        if let Some(info) = loops(e).pop() {
            drop_loop(e, &info);
        }
    }
}

/// `PlayerCharacter::GetSingleton` (`0x011DEA3C`).
const PLAYER_SINGLETON: u32 = 0x011d_ea3c;

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Loops.cpp:
// ContainerIterLoop::ContainerIterLoop.
/// A container's loop: its inventory reference, and a stack (an entry of its own) for each
/// part of each item: the parts with their own extra data first (their counts, at most what is
/// left), then the rest; items in the order of xNVSE's map of the container's items.
fn container_loop(e: &mut Engine, container: u32, local: u32) -> (Loop, bool) {
    let refr = crate::invref::create(e, container, crate::invref::Data::default(), false);
    let invref = e.mem.u32(refr + 0xc);
    let mut elements = Vec::new();
    for (item, count, entry) in crate::invref::inventory_items(e, container) {
        let mut count = count;
        let extend = if entry != 0 {
            e.mem.u32(entry + crate::invref::ENTRY_EXTEND_DATA)
        } else {
            0
        };
        if extend != 0 {
            for xdata in crate::invref::list_items(e, extend) {
                let mut n = crate::invref::count_for_extra_list(e, xdata);
                if n < 1 {
                    continue;
                }
                if n > count {
                    n = count;
                }
                count -= n;
                elements.push(crate::invref::create_temp_entry(e, item, n, xdata));
                if count == 0 {
                    break;
                }
            }
        }
        if count > 0 {
            elements.push(crate::invref::create_temp_entry(e, item, count, 0));
        }
    }
    let empty = !set_container_iterator(e, invref, local, &elements, 0);
    (
        Loop::Container {
            invref,
            local,
            elements,
            index: 0,
        },
        empty,
    )
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Loops.cpp: ContainerIterLoop::SetIterator.
/// The inventory reference stands for stack `index` and the variable holds it; past the end
/// both are cleared.
fn set_container_iterator(
    e: &mut Engine,
    invref: u32,
    local: u32,
    elements: &[u32],
    index: usize,
) -> bool {
    let refr = crate::invref::get_ref(e, invref);
    if index < elements.len() && refr != 0 {
        let entry = elements[index];
        let extend = e.mem.u32(entry + crate::invref::ENTRY_EXTEND_DATA);
        let xdata = if extend != 0 { e.mem.u32(extend) } else { 0 };
        let item = e.mem.u32(entry + crate::invref::ENTRY_TYPE);
        crate::invref::set_data(e, invref, crate::invref::Data { item, entry, xdata });
        e.mem.set_u32(local + 8, e.mem.u32(refr + 0xc));
        e.mem.set_u32(local + 12, 0);
        true
    } else {
        e.mem.set_f64(local + 8, 0.0);
        crate::invref::set_data(e, invref, crate::invref::Data::default());
        false
    }
}

/// Ends a loop: `ArrayIterLoop`'s destructor gives up the iterator variable's reference;
/// `ContainerIterLoop`'s releases its inventory reference and frees its stacks.
///
/// xNVSE frees the stacks first, and the release's deferred actions then read the count of the
/// stack the reference stands for from its freed entry. Here the release comes first: the same
/// actions with the same counts, read before the entry is freed.
pub(crate) fn drop_loop(e: &mut Engine, info: &LoopInfo) {
    match info.kind {
        Loop::Container {
            invref,
            local,
            ref elements,
            ..
        } => {
            crate::invref::release(e, invref);
            for &entry in elements {
                crate::invref::free_temp_entry(e, entry);
            }
            e.mem.set_f64(local + 8, 0.0);
        }
        Loop::Array {
            iter,
            local,
            owner_mod,
            ..
        } if iter != 0 => vars::remove_reference_double(e, local + 8, owner_mod),
        // `ForEachAlt`'s variables are zeroed.
        Loop::Alt {
            value_var, key_var, ..
        } => {
            if value_var.0 != 0 {
                e.mem.set_f64(value_var.0 + 8, 0.0);
            }
            if let Some((local, _)) = key_var {
                if local != 0 {
                    e.mem.set_f64(local + 8, 0.0);
                }
            }
        }
        _ => {}
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Loops.cpp: LoopManager::Break.
fn break_loop(e: &mut Engine, a: &CommandArgs) -> bool {
    let Some(info) = loops(e).pop() else {
        return false;
    };
    if let Some((runner, length)) = runner_and_length(e) {
        restore_stack(e, runner, &info.saved);
        jump(e, length, a.opcode_offset, info.end_ip);
    }
    drop_loop(e, &info);
    true
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Loops.cpp: LoopManager::Continue.
fn continue_loop(e: &mut Engine, a: &CommandArgs) -> bool {
    let Some(mut info) = loops(e).last().cloned() else {
        return false;
    };
    if !update(e, a, &mut info.kind) {
        break_loop(e, a);
        return true;
    }
    if let Some(top) = loops(e).last_mut() {
        top.kind = info.kind.clone();
    }
    if let Some((runner, length)) = runner_and_length(e) {
        restore_stack(e, runner, &info.saved);
        jump(e, length, a.opcode_offset, info.saved.ip);
    }
    true
}

/// `Loop::Update`: the next pass, false at the end.
fn update(e: &mut Engine, a: &CommandArgs, kind: &mut Loop) -> bool {
    match kind {
        // `WhileLoop::Update`: the condition at its offset, the offset word put back.
        Loop::While { expr_offset } => {
            let original = e.mem.u32(a.opcode_offset);
            e.mem.set_u32(a.opcode_offset, *expr_offset);
            let (toks, ev) = evaluate_args(e, a, false);
            e.mem.set_u32(a.opcode_offset, original);
            match toks.as_ref().and_then(|t| t.first()) {
                Some(first) => ev.get_bool(e, first),
                None => false,
            }
        }
        Loop::Array {
            source, iter, key, ..
        } => {
            let Some(cur) = key.clone() else {
                return false;
            };
            let Some(arr) = vars::array(e, *source) else {
                return false;
            };
            let keys = arr.keys();
            // `GetNextElement`: the first key after the current one.
            let upper = vars::upper_table(e);
            let next = keys.into_iter().find(|k| key_less(&upper, &cur, k));
            let Some(next) = next else {
                return false;
            };
            *key = Some(next.clone());
            update_iterator(e, *source, *iter, &next)
        }
        Loop::Alt {
            source,
            key,
            value_var,
            key_var,
            script,
        } => {
            let Some(cur) = key.clone() else {
                return false;
            };
            if *source == 0 {
                return false;
            }
            let Some(arr) = vars::array(e, *source) else {
                return false;
            };
            let keys = arr.keys();
            let upper = vars::upper_table(e);
            let Some(next) = keys.into_iter().find(|k| key_less(&upper, &cur, k)) else {
                return false;
            };
            *key = Some(next.clone());
            update_alt(e, *source, &next, *value_var, *key_var, *script)
        }
        Loop::Str { text, iter, index } => {
            if vars::string(e, *iter).is_none() {
                return false;
            }
            *index += 1;
            if *index < text.len() {
                vars::set_string(e, *iter, &text[*index..*index + 1]);
                true
            } else {
                false
            }
        }
        Loop::Container {
            invref,
            local,
            elements,
            index,
        } => {
            // `UnsetIterator`, then the next stack.
            crate::invref::write_ref_data_to_container(e, *invref);
            *index += 1;
            set_container_iterator(e, *invref, *local, elements, *index)
        }
        Loop::FormList {
            node,
            local,
            walked,
        } => {
            if *node == 0 {
                return false;
            }
            *node = e.mem.u32(*node + 4);
            while *node != 0 {
                if !crate::invref::list_step(e, walked) {
                    *node = 0;
                    return false;
                }
                let form = e.mem.u32(*node);
                if form != 0 {
                    e.mem.set_u32(*local + 8, e.mem.u32(form + 0xc));
                    e.mem.set_u32(*local + 12, 0);
                    return true;
                }
                *node = e.mem.u32(*node + 4);
            }
            false
        }
    }
}

/// `ArrayKey::operator<` for keys of one type.
fn key_less(upper: &[u8; 256], a: &Key, b: &Key) -> bool {
    match (a, b) {
        (Key::Number(x), Key::Number(y)) => x < y,
        (Key::Str(x), Key::Str(y)) => vars::str_compare(upper, x, y).is_lt(),
        _ => true,
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Loops.cpp: ArrayIterLoop::UpdateIterator
// (the iterator-array form).
fn update_iterator(e: &mut Engine, source: u32, iter: u32, key: &Key) -> bool {
    let Some(elem) = vars::get_element(e, source, key) else {
        return false;
    };
    let key_value = match key {
        Key::Str(s) => vars::Value::Str(s.clone()),
        Key::Number(n) => vars::Value::Number(*n),
    };
    if !vars::set_element(e, iter, &Key::Str(b"key".to_vec()), key_value) {
        return false;
    }
    let value = match elem.data {
        vars::Data::Number(n) => vars::Value::Number(n),
        vars::Data::Form(f) => vars::Value::Form(f),
        vars::Data::Str(s) => vars::Value::Str(s),
        vars::Data::Array(a) => vars::Value::Array(a),
        vars::Data::Invalid => return false,
    };
    vars::set_element(e, iter, &Key::Str(b"value".to_vec()), value)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Scripting.cpp:
// Cmd_While_Execute.
/// The data: a `UInt32` offset to the loop's end, the argument count, the condition.
fn while_execute(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    let Some((runner, _)) = runner_and_length(e) else {
        return handled(true);
    };
    let offset = e.mem.u32(a.opcode_offset);
    let at = a.data.wrapping_add(offset);
    let end = e.mem.u32(at);
    let expr_len = u32::from(e.mem.u16(at + 5));
    let start = offset + 5 + expr_len;
    add_loop(
        e,
        Loop::While {
            expr_offset: offset + 4,
        },
        runner,
        start,
        end,
    );
    e.mem.set_u32(a.opcode_offset, start);
    let mut kind = Loop::While {
        expr_offset: offset + 4,
    };
    if !update(e, &a, &mut kind) {
        break_loop(e, &a);
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Scripting.cpp:
// Cmd_ForEach_Execute, with the ArrayIterLoop, StringIterLoop and FormListIterLoop constructors
// (Loops.cpp).
fn foreach_execute(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    let Some((runner, _)) = runner_and_length(e) else {
        return handled(true);
    };
    let offset = e.mem.u32(a.opcode_offset);
    let at = a.data.wrapping_add(offset);
    let end = e.mem.u32(at);
    let expr_len = u32::from(e.mem.u16(at + 5));
    let start = offset + 5 + expr_len;
    e.mem.set_u32(a.opcode_offset, offset + 4);
    let (toks, _) = evaluate_args(e, &a, false);
    let now = e.mem.u32(a.opcode_offset);
    e.mem.set_u32(a.opcode_offset, now.wrapping_sub(4));
    let Some(Tok::ForEach {
        source,
        iter,
        local,
        var_type,
    }) = toks.and_then(|t| t.into_iter().next())
    else {
        // `ForEach expression failed to return a value`: the line fails.
        return handled(false);
    };
    let modi = vars::script_mod_index(e, a.script);
    let (kind, empty) = match var_type {
        3 => {
            // Clear the iterator variable, then refer it to the iterator array.
            vars::remove_reference_double(e, local + 8, modi);
            vars::add_reference_double(e, local + 8, iter, modi);
            let mut key = None;
            let mut ok_source = source;
            if source != 0 {
                match vars::array(e, source).map(|arr| arr.keys()) {
                    Some(keys) => {
                        if let Some(first) = keys.into_iter().next() {
                            if !update_iterator(e, source, iter, &first) {
                                ok_source = 0;
                            }
                            key = Some(first);
                        }
                    }
                    None => ok_source = 0,
                }
            }
            vars::collect(e, a.event_list, local, vars::VarType::Array);
            let empty =
                ok_source == 0 || !vars::array(e, ok_source).is_some_and(|arr| !arr.is_empty());
            (
                Loop::Array {
                    source: ok_source,
                    iter,
                    local,
                    key,
                    owner_mod: modi,
                },
                empty,
            )
        }
        2 => {
            let text = vars::string(e, source).unwrap_or_default();
            if !text.is_empty() {
                vars::set_string(e, iter, &text[..1]);
            }
            let empty = text.is_empty();
            (
                Loop::Str {
                    text,
                    iter,
                    index: 0,
                },
                empty,
            )
        }
        _ if e.mem.u8(source + 4) != 0x55 => {
            let (kind, empty) = container_loop(e, source, local);
            (kind, empty)
        }
        _ => {
            // A form list.
            let mut node = source + 0x18;
            let mut walked = 0usize;
            while node != 0 && e.mem.u32(node) == 0 {
                if !crate::invref::list_step(e, &mut walked) {
                    node = 0;
                    break;
                }
                node = e.mem.u32(node + 4);
            }
            if node != 0 {
                let form = e.mem.u32(node);
                e.mem.set_u32(local + 8, e.mem.u32(form + 0xc));
                e.mem.set_u32(local + 12, 0);
            }
            (
                Loop::FormList {
                    node,
                    local,
                    walked,
                },
                node == 0,
            )
        }
    };
    add_loop(e, kind, runner, start, end);
    if empty {
        e.mem.set_u32(a.opcode_offset, start);
        break_loop(e, &a);
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Loops.cpp: UpdateForEachAltIterator, and
// the `ForEachAlt` half of ArrayIterLoop::UpdateIterator.
/// The element at `key` into `ForEachAlt`'s variables: with one variable over a map, that one
/// takes the key.
fn update_alt(
    e: &mut Engine,
    source: u32,
    key: &Key,
    value_var: (u32, u8),
    key_var: Option<(u32, u8)>,
    script: u32,
) -> bool {
    let Some(elem) = vars::get_element(e, source, key) else {
        return false;
    };
    let is_map =
        vars::array(e, source).is_some_and(|a| !matches!(a.elements, vars::Container::Array(_)));
    let (value, key_target) = match key_var {
        None if is_map => ((0, 5), value_var),
        None => (value_var, (0, 5)),
        Some(k) => (value_var, k),
    };
    let modi = vars::script_mod_index(e, script);
    if key_target.0 != 0 {
        match key {
            Key::Str(s) => {
                let id = vars::add_string(e, modi, s, true);
                e.mem.set_f64(key_target.0 + 8, f64::from(id));
            }
            Key::Number(n) => {
                let n = if key_target.1 == 1 { n.floor() } else { *n };
                e.mem.set_f64(key_target.0 + 8, n);
            }
        }
    }
    if value.0 == 0 {
        return true;
    }
    let (local, ty) = value;
    let wrong = |e: &mut Engine, what: &str| {
        let text = format!(
            "ForEachAlt >> Received {what} value, but the value iter variable has type {}.",
            crate::functions::variable_type_name(ty)
        );
        crate::functions::runtime_error(e, script, &text);
        false
    };
    match elem.data {
        vars::Data::Array(id) => {
            if ty != 3 {
                return wrong(e, "an array-type");
            }
            e.mem.set_f64(local + 8, f64::from(id));
        }
        vars::Data::Str(s) => {
            if ty != 2 {
                return wrong(e, "a string-type");
            }
            let id = vars::add_string(e, modi, &s, true);
            e.mem.set_f64(local + 8, f64::from(id));
        }
        vars::Data::Number(n) => {
            if ty != 0 && ty != 1 {
                return wrong(e, "a numeric-type");
            }
            let n = if ty == 1 { n.floor() } else { n };
            e.mem.set_f64(local + 8, n);
        }
        vars::Data::Form(id) => {
            if ty != 4 {
                return wrong(e, "a form-type");
            }
            e.mem.set_u32(local + 8, id);
        }
        vars::Data::Invalid => return false,
    }
    true
}

/// `ForEachAlt::ValidateKeyVariableType`: numbers for an array or a number map, a string for a
/// string map.
fn valid_key_variable(e: &mut Engine, ev: &mut Eval, ty: u8, source: u32) -> bool {
    let numeric = vars::array(e, source).is_some_and(|a| {
        matches!(
            a.elements,
            vars::Container::Array(_) | vars::Container::NumMap(_)
        )
    });
    if numeric {
        if ty != 1 && ty != 0 {
            ev.error_text("ForEachAlt >> Invalid variable type to receive array keys (keys were numeric-type, var was not)".to_string());
            return false;
        }
    } else if ty != 2 {
        ev.error_text("ForEachAlt >> Invalid variable type to receive array keys (keys were string-type, var was not)".to_string());
        return false;
    }
    true
}

/// A variable argument's local and type (`GetScriptLocal`, `variableType`); `(0, 5)` for
/// anything else (an empty optional argument).
fn variable_of(t: &Tok) -> (u32, u8) {
    match t {
        Tok::Var {
            local, var_type, ..
        } => (*local, *var_type),
        _ => (0, 5),
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Scripting.cpp:
// Cmd_ForEachAlt_Execute, with ArrayIterLoop's ForEachAlt constructor and Init (Loops.cpp).
fn foreach_alt_execute(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    let Some((runner, _)) = runner_and_length(e) else {
        return handled(true);
    };
    let offset = e.mem.u32(a.opcode_offset);
    let end = e.mem.u32(a.data.wrapping_add(offset));
    e.mem.set_u32(a.opcode_offset, offset + 4);
    let mut ev = Eval::new(e, &a);
    let toks = expr::with_evaluator(e, |e| ev.extract_args(e));
    let start = e.mem.u32(a.opcode_offset) + 1;
    let now = e.mem.u32(a.opcode_offset);
    e.mem.set_u32(a.opcode_offset, now.wrapping_sub(4));
    let toks = match toks {
        Some(t) if t.len() >= 2 => t,
        _ => {
            crate::functions::runtime_error(e, a.script, "ForEachAlt >> Failed to extract args.");
            expr::report(e, &ev);
            return handled(false);
        }
    };
    let source_id = ev.get_array_id(e, &toks[0]);
    if !vars::array_exists(e, source_id) {
        ev.error_text("ForEachAlt >> Invalid source array.".to_string());
        expr::report(e, &ev);
        return handled(true);
    }
    let mut source = source_id;
    let value_var = variable_of(&toks[1]);
    let mut key_var = None;
    if toks.len() >= 3 {
        let k = variable_of(&toks[2]);
        key_var = Some(k);
        if k.0 != 0 && !valid_key_variable(e, &mut ev, k.1, source) {
            source = 0;
        }
    } else {
        let packed = vars::array(e, source)
            .is_some_and(|arr| matches!(arr.elements, vars::Container::Array(_)));
        if !packed && !valid_key_variable(e, &mut ev, value_var.1, source) {
            source = 0;
        }
    }
    if value_var.0 == 0 && !key_var.is_some_and(|k| k.0 != 0) {
        source = 0;
    }
    expr::report(e, &ev);
    // `ArrayIterLoop::Init`.
    let mut key = None;
    if source != 0 {
        if let Some(first) = vars::array(e, source).and_then(|arr| arr.keys().into_iter().next()) {
            if !update_alt(e, source, &first, value_var, key_var, a.script) {
                source = 0;
            }
            key = Some(first);
        }
    }
    let empty = source == 0 || !vars::array(e, source).is_some_and(|arr| !arr.is_empty());
    add_loop(
        e,
        Loop::Alt {
            source,
            key,
            value_var,
            key_var,
            script: a.script,
        },
        runner,
        start,
        end,
    );
    if empty {
        e.mem.set_u32(a.opcode_offset, start);
        break_loop(e, &a);
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Scripting.cpp:
// Cmd_Loop_Execute, Cmd_Continue_Execute, Cmd_Break_Execute. Outside a loop each is a runtime
// error and the line fails.
fn loop_execute(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    handled(continue_loop(e, &a))
}

fn continue_execute(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    if continue_loop(e, &a) {
        let now = e.mem.u32(a.opcode_offset);
        e.mem.set_u32(a.opcode_offset, now.wrapping_add(4));
        return handled(true);
    }
    handled(false)
}

fn break_execute(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    handled(break_loop(e, &a))
}

/// `Label`'s and `Goto`'s index (`ExtractArgs(&idx)`), and the offset word moved past the
/// arguments by their length (the word just before them).
fn label_index(e: &mut Engine, a: &CommandArgs) -> Option<u32> {
    let read = super::Read::of(e, a);
    if !read.ok {
        return None;
    }
    let offset = e.mem.u32(a.opcode_offset);
    let len = u32::from(e.mem.u16(a.data.wrapping_add(offset).wrapping_sub(2)));
    e.mem.set_u32(a.opcode_offset, offset.wrapping_add(len));
    Some(read.int(0, 0))
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Scripting.cpp: Cmd_Label_Execute.
fn label(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    let Some(idx) = label_index(e, &a) else {
        return handled(true);
    };
    let Some((runner, _)) = runner_and_length(e) else {
        return handled(true);
    };
    let ip = e.mem.u32(a.opcode_offset);
    let saved = save_stack(e, runner, ip);
    let script_id = e.mem.u32(a.script + 0xc);
    e.ext::<Nvse>()
        .loops
        .labels
        .entry(script_id)
        .or_default()
        .insert(idx, saved);
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Scripting.cpp: Cmd_Goto_Execute.
fn goto(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    let read = super::Read::of(e, &a);
    if !read.ok {
        return handled(true);
    }
    let script_id = e.mem.u32(a.script + 0xc);
    let Some(info) = e
        .ext::<Nvse>()
        .loops
        .labels
        .get(&script_id)
        .and_then(|m| m.get(&read.int(0, 0)))
        .cloned()
    else {
        return handled(true);
    };
    let offset = e.mem.u32(a.opcode_offset);
    let len = u32::from(e.mem.u16(a.data.wrapping_add(offset).wrapping_sub(2)));
    e.mem.set_u32(a.opcode_offset, offset.wrapping_add(len));
    if let Some((runner, length)) = runner_and_length(e) {
        jump(e, length, a.opcode_offset, info.ip);
        restore_stack(e, runner, &info);
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), ScriptUtils.cpp: Eval_In.
/// The `<-` operator's result (a `ForEachContext`), made by [`crate::expr`]. A left side with no
/// local (a command's string result) falls through to the unsupported-type error, as in xNVSE
/// (each case tests `GetScriptLocal()` after making what it makes first).
pub fn foreach_context(e: &mut Engine, ev: &Eval, lh: &Tok, rh: &Tok) -> Result<Tok, &'static str> {
    const UNSUPPORTED: &str = "Unsupported variable type to iterate over (only array, string and ref variables supported for non-alt ForEach)";
    let Tok::Var {
        local, var_type, ..
    } = lh
    else {
        return Err(UNSUPPORTED);
    };
    let local = *local;
    let modi = vars::script_mod_index(e, ev.script);
    match *var_type {
        3 => {
            let iter = vars::create_array(e, vars::KEY_STRING, false, modi);
            if local == 0 {
                return Err(UNSUPPORTED);
            }
            let source = ev.get_array_id(e, rh);
            Ok(Tok::ForEach {
                source,
                iter,
                local,
                var_type: 3,
            })
        }
        2 => {
            let text = ev.get_string(e, rh);
            let source = vars::add_string(e, modi, &text, true);
            if local == 0 {
                return Err(UNSUPPORTED);
            }
            let mut iter = crate::args::f64_to_i32(e.mem.f64(local + 8)) as u32;
            if vars::string(e, iter).is_none() {
                vars::collect(e, ev.event_list, local, vars::VarType::String);
                iter = vars::add_string(e, modi, b"", false);
                e.mem.set_f64(local + 8, f64::from(iter));
            }
            Ok(Tok::ForEach {
                source,
                iter,
                local,
                var_type: 2,
            })
        }
        4 => {
            // A form list stays one; the player's id is the player; another form must be a
            // reference (`DYNAMIC_CAST` to `TESObjectREFR`).
            let mut form = ev.get_form(e, rh);
            if form != 0 && e.mem.u8(form + 4) != 0x55 {
                form = if e.mem.u32(form + 0xc) == 0x14 {
                    e.global::<u32>(PLAYER_SINGLETON)
                } else {
                    super::dynamic_cast(
                        e,
                        form,
                        super::rtti::TES_FORM,
                        super::rtti::TES_OBJECT_REFR,
                    )
                };
            }
            if form == 0 || local == 0 {
                return Err("Source is a base form (must be a reference)");
            }
            Ok(Tok::ForEach {
                source: form,
                iter: 0,
                local,
                var_type: 4,
            })
        }
        _ => Err(UNSUPPORTED),
    }
}

#[cfg(test)]
#[path = "scripting_tests.rs"]
mod tests;
