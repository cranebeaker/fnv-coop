//! xNVSE's event commands (ADR-0016 §8; build step I-9), translated from `Commands_Script.cpp`
//! (xNVSE): `SetEventHandler`, `SetEventHandlerAlt`, `RemoveEventHandler`,
//! `GetCurrentEventName`, `DispatchEvent`, `DispatchEventAlt`, `GetEventHandlers`,
//! `IsEventHandlerFirst`/`Last`, `GetHigher`/`LowerPriorityEventHandlers`, on
//! [`crate::events`].
//!
//! Not here yet: `DumpEventHandlers` (a console listing; it needs the forms' and arrays' text
//! forms), JIP's LN events behind `SetEventHandler` (`GetLNEventMask`, with JIP, I-13), and
//! `GetCurrentEventName`'s `set ... to` target (`ExtractSetStatementVar`, with the script
//! analyser, I-14): in a `set` statement its string is a temporary, as in xNVSE when that read
//! fails; in an expression (`let`, `eval`) it is xNVSE's.

use crate::events::{self, Callback, Dispatch, PRIORITY_DEFAULT, PRIORITY_INVALID};
use crate::expr::{Eval, Tok, Ty};
use crate::functions;
use crate::table::{Command, Param, Returns};
use crate::vars::{self, Data, Key, Value};
use crate::{handled, CommandArgs};
use engine_core::prelude::*;

/// `kNVSEParamType_*`.
const NUMBER: u32 = 0x8001;
const STRING: u32 = 0x8004;
const FORM: u32 = 0x8008;
const ARRAY: u32 = 0x8040;
const BASIC: u32 = 0x804d;
const PAIR: u32 = 0x20_0000;

/// `kMaxMessageLength`.
const MAX_MESSAGE_LENGTH: usize = 0x4000;

const fn p(name: &'static str, kind: u32, optional: bool) -> Param {
    Param {
        name,
        kind,
        optional,
    }
}

const FILTER: Param = p("filter", PAIR, true);
/// `kNVSEParams_SetEventHandler`: name, script, three filters (one for the priority).
const SET_EVENT_HANDLER: &[Param] = &[
    p("event name", STRING, false),
    p("function script", FORM, false),
    FILTER,
    FILTER,
    FILTER,
];
/// `kNVSEParams_SetEventHandlerAlt`: name, script, 17 filters (the calling reference's, the
/// priority, 15 arguments).
const SET_EVENT_HANDLER_ALT: &[Param] = &[
    p("event name", STRING, false),
    p("function script", FORM, false),
    FILTER,
    FILTER,
    FILTER,
    FILTER,
    FILTER,
    FILTER,
    FILTER,
    FILTER,
    FILTER,
    FILTER,
    FILTER,
    FILTER,
    FILTER,
    FILTER,
    FILTER,
    FILTER,
    FILTER,
];
/// `kNVSEParams_DispatchEvent`.
const DISPATCH_EVENT: &[Param] = &[
    p("eventName", STRING, false),
    p("args", ARRAY, true),
    p("sender", STRING, true),
];
const fn arg(name: &'static str) -> Param {
    p(name, BASIC, true)
}
/// `kNVSEParams_DispatchEventAlt`: the name and 15 arguments.
const DISPATCH_EVENT_ALT: &[Param] = &[
    p("eventName", STRING, false),
    arg("arg1"),
    arg("arg2"),
    arg("arg3"),
    arg("arg4"),
    arg("arg5"),
    arg("arg6"),
    arg("arg7"),
    arg("arg8"),
    arg("arg9"),
    arg("arg10"),
    arg("arg11"),
    arg("arg12"),
    arg("arg13"),
    arg("arg14"),
    arg("arg15"),
];
/// `kNVSEParams_DumpEventHandlers` (`GetEventHandlers` takes the same).
const DUMP_EVENT_HANDLERS: &[Param] = &[
    p("eventName", STRING, true),
    p("script", FORM, true),
    p("priority", NUMBER, true),
    arg("arg1"),
    arg("arg2"),
    arg("arg3"),
    arg("arg4"),
    arg("arg5"),
    arg("arg6"),
    arg("arg7"),
    arg("arg8"),
    arg("arg9"),
    arg("arg10"),
    arg("arg11"),
    arg("arg12"),
    arg("arg13"),
    arg("arg14"),
    arg("arg15"),
];
/// `kNVSEParams_GetHigherOrLowerPriorityEventHandlers`.
const HIGHER_OR_LOWER: &[Param] = &[
    p("eventName", STRING, false),
    p("script", FORM, false),
    p("priority", NUMBER, false),
    p("scriptsToIgnore", FORM, true),
    p("pluginsToIgnore", ARRAY, true),
    p("pluginHandlersToIgnore", ARRAY, true),
];

const fn cmd(
    opcode: u32,
    name: &'static str,
    help: &'static str,
    params: &'static [Param],
    execute: AbiFn,
    returns: Returns,
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
        returns,
    }
}

pub static COMMANDS: &[Command] = &[
    cmd(
        0x15e9,
        "SetEventHandler",
        "defines a function script to serve as a callback for game events",
        SET_EVENT_HANDLER,
        set_event_handler,
        Returns::Default,
    ),
    cmd(
        0x15ea,
        "RemoveEventHandler",
        "removes event handlers matching the event, script, and optional filters specified",
        SET_EVENT_HANDLER,
        remove_event_handler,
        Returns::Default,
    ),
    cmd(
        0x15eb,
        "GetCurrentEventName",
        "returns the name of the event currently being processed by an event handler",
        &[],
        get_current_event_name,
        Returns::String,
    ),
    cmd(
        0x15ec,
        "DispatchEvent",
        "dispatches a user - defined event to any registered listeners",
        DISPATCH_EVENT,
        dispatch_event,
        Returns::Default,
    ),
    cmd(
        0x1647,
        "DispatchEventAlt",
        "dispatches a user - defined event to any registered listeners",
        DISPATCH_EVENT_ALT,
        dispatch_event_alt,
        Returns::Default,
    ),
    cmd(
        0x1649,
        "GetEventHandlers",
        "returns a multi-dimensional array of event handlers, optionally filtered by eventName, script and args.",
        DUMP_EVENT_HANDLERS,
        get_event_handlers,
        Returns::Array,
    ),
    cmd(
        0x164b,
        "SetEventHandlerAlt",
        "Uses the new event filtering system.",
        SET_EVENT_HANDLER_ALT,
        set_event_handler_alt,
        Returns::Default,
    ),
    cmd(
        0x1650,
        "IsEventHandlerFirst",
        "",
        HIGHER_OR_LOWER,
        is_event_handler_first,
        Returns::Default,
    ),
    cmd(
        0x1651,
        "IsEventHandlerLast",
        "",
        HIGHER_OR_LOWER,
        is_event_handler_last,
        Returns::Default,
    ),
    cmd(
        0x1652,
        "GetHigherPriorityEventHandlers",
        "",
        HIGHER_OR_LOWER,
        get_higher_priority_event_handlers,
        Returns::Array,
    ),
    cmd(
        0x1653,
        "GetLowerPriorityEventHandlers",
        "",
        HIGHER_OR_LOWER,
        get_lower_priority_event_handlers,
        Returns::Array,
    ),
];

/// An expression command's run: the result zeroed, its evaluator on the stack for the whole
/// command (`ExpressionEvaluator`'s lifetime), the arguments extracted, `f`'s answer as the
/// result when they were, the evaluator's errors reported at the end.
fn run(
    e: &mut Engine,
    w: &[u32],
    f: impl FnOnce(&mut Engine, &CommandArgs, &mut Eval, Vec<Tok>) -> Option<f64>,
) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    let mut ev = Eval::new(e, &a);
    crate::expr::with_evaluator(e, |e| {
        if let Some(toks) = ev.extract_args(e) {
            if let Some(r) = f(e, &a, &mut ev, toks) {
                a.set_result(e, r);
            }
        }
    });
    crate::expr::report(e, &ev);
    handled(true)
}

/// `BasicTokenToElem` into a `SelfOwningArrayElement`: an array is held.
fn basic_to_data(e: &mut Engine, ev: &Eval, t: &Tok) -> Option<Data> {
    let mut data = match functions::to_basic(e, ev, t)? {
        Tok::Number { v, .. } => Data::Number(v),
        Tok::Str(s) => Data::Str(s),
        Tok::Form(id) => Data::Form(id),
        Tok::Array(id) => Data::Array(id),
        _ => return None,
    };
    events::hold_data(e, &mut data);
    Some(data)
}

fn str_eq(e: &mut Engine, a: &[u8], b: &[u8]) -> bool {
    let upper = vars::upper_table(e);
    vars::str_compare(&upper, a, b) == std::cmp::Ordering::Equal
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Script.cpp:
// ExtractEventCallback.
/// The event's name, the handler and its priority (left as given unless a `"priority"` pair
/// sets it); `None` when it fails (what was held is given up).
fn extract_callback(
    e: &mut Engine,
    ev: &mut Eval,
    toks: &[Tok],
    old_filters: bool,
    new_filters: bool,
    priority: &mut i32,
) -> Option<(Vec<u8>, Callback)> {
    if toks.len() < 2 {
        return None;
    }
    let name = ev.get_string(e, &toks[0]);
    let form = ev.get_form(e, &toks[1]);
    if form == 0 || e.mem.u8(form + 4) != 0x11 {
        return None;
    }
    let mut cb = Callback::new(form);
    // Handlers with the new filters go when a game is loaded: an array filter from a save
    // could be stale after loading back to before it was set.
    cb.flush_on_load = new_filters;
    for (i, t) in toks.iter().enumerate().skip(2) {
        let Tok::Pair(left, right) = t else {
            ev.error_text(format!("Received invalid pair for arg {i} somehow."));
            continue;
        };
        if ev.can_convert(e, left, Ty::String) {
            let key = ev.get_string(e, left);
            if str_eq(e, &key, b"priority") {
                *priority = crate::args::f64_to_i32(ev.get_number(e, right));
                continue;
            }
            if old_filters {
                if str_eq(e, &key, b"ref") || str_eq(e, &key, b"first") {
                    cb.source = ev.get_form(e, right);
                    continue;
                }
                if str_eq(e, &key, b"object") || str_eq(e, &key, b"second") {
                    cb.object = ev.get_form(e, right);
                    continue;
                }
                ev.error_text(format!(
                    "Invalid string filter key \"{}\" passed, ignoring it.",
                    String::from_utf8_lossy(&key)
                ));
                continue;
            }
            ev.error_text(
                "String filter keys are not allowed for this function; use int codes.".to_string(),
            );
            events::release(e, &cb);
            return None;
        }
        if new_filters {
            let index = crate::args::f64_to_i32(ev.get_number(e, left));
            if index < 0 {
                ev.error_text(format!("Invalid index {index} passed (arg indices start from 1, and callingReference is filter #0)."));
                events::release(e, &cb);
                return None;
            }
            if index > events::MAX_FILTERS as i32 {
                ev.error_text(format!(
                    "Invalid index {index} passed (max filter index is {}).",
                    events::MAX_FILTERS
                ));
                events::release(e, &cb);
                return None;
            }
            if let Some(data) = basic_to_data(e, ev, right) {
                let index = index as u32;
                match cb.filters.entry(index) {
                    std::collections::btree_map::Entry::Occupied(_) => {
                        ev.error_text(format!(
                            "Event filter index {index} appears more than once."
                        ));
                        events::release_data(e, &data);
                    }
                    std::collections::btree_map::Entry::Vacant(slot) => {
                        slot.insert(data);
                    }
                }
            }
        } else {
            ev.error_text(
                "Int filter keys are not allowed for this function; use string.".to_string(),
            );
        }
    }
    Some((name, cb))
}

fn set_with(e: &mut Engine, ev: &mut Eval, toks: &[Tok], old: bool, new: bool) -> Option<f64> {
    let mut priority = PRIORITY_DEFAULT;
    let ok = match extract_callback(e, ev, toks, old, new, &mut priority) {
        Some((name, cb)) => events::set_handler(e, &name, cb, priority, Some(ev)),
        None => false,
    };
    Some(f64::from(u8::from(ok)))
}

fn set_event_handler(e: &mut Engine, w: &[u32]) -> Ret {
    run(e, w, |e, _, ev, toks| set_with(e, ev, &toks, true, false))
}

fn set_event_handler_alt(e: &mut Engine, w: &[u32]) -> Ret {
    run(e, w, |e, _, ev, toks| set_with(e, ev, &toks, false, true))
}

fn remove_event_handler(e: &mut Engine, w: &[u32]) -> Ret {
    run(e, w, |e, _, ev, toks| {
        let mut priority = PRIORITY_INVALID;
        let ok = match extract_callback(e, ev, &toks, true, true, &mut priority) {
            Some((name, cb)) => {
                let ok = events::remove_handler(e, &name, &cb, priority, Some(ev));
                events::release(e, &cb);
                ok
            }
            None => false,
        };
        Some(f64::from(u8::from(ok)))
    })
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), StringVar.cpp: AssignToStringVarLong.
/// A string command's result: a new temporary string of the script's file, its id the result.
fn assign_to_string_var(e: &mut Engine, a: &CommandArgs, text: &[u8]) {
    let text = if text.len() >= MAX_MESSAGE_LENGTH {
        &[][..]
    } else {
        text
    };
    let modi = vars::script_mod_index(e, a.script);
    let id = vars::add_string(e, modi, text, true);
    a.set_result(e, f64::from(id));
}

fn get_current_event_name(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    let name = events::current_event_name(e);
    assign_to_string_var(e, &a, &name);
    handled(true)
}

fn dispatch_event(e: &mut Engine, w: &[u32]) -> Ret {
    run(e, w, |e, a, ev, toks| {
        let first = toks.first()?;
        let name = ev.get_string(e, first);
        let mut args_array = 0;
        let mut sender = None;
        if toks.len() > 1 {
            if !ev.can_convert(e, &toks[1], Ty::Array) {
                return None;
            }
            args_array = ev.get_array_id(e, &toks[1]);
            if toks.len() > 2 {
                sender = Some(ev.get_string(e, &toks[2]));
            }
        }
        let ok = events::dispatch_user_defined(
            e,
            &name,
            a.script,
            args_array,
            sender.as_deref(),
            Some(ev),
        );
        Some(f64::from(u8::from(ok)))
    })
}

/// An argument as a dispatch takes it from a script (`GetAsVoidArgAndVarType`): a number's
/// `float` bits, a string (put on the guest stack for the dispatch), a form's address, an
/// array's id.
enum Raw {
    Word(u32),
    Text(Vec<u8>),
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Script.cpp:
// ExtractArgsAndArgTypes.
fn raw_args(e: &mut Engine, ev: &Eval, toks: &[Tok], from: usize) -> (Vec<Raw>, Vec<u8>) {
    let mut raws = Vec::new();
    let mut types = Vec::new();
    for t in toks.iter().skip(from) {
        let (raw, var_type) = if ev.can_convert(e, t, Ty::Number) {
            let v = ev.get_number(e, t) as f32;
            (Raw::Word(v.to_bits()), 0)
        } else if ev.can_convert(e, t, Ty::String) {
            (Raw::Text(ev.get_string(e, t)), 2)
        } else if ev.can_convert(e, t, Ty::Form) {
            let id = ev.get_form_id(e, t);
            let form = e.call(0x0048_39c0, &args![id]).u32();
            (Raw::Word(form), 4)
        } else if ev.can_convert(e, t, Ty::Array) {
            (Raw::Word(ev.get_array_id(e, t)), 3)
        } else {
            (Raw::Word(0), 5)
        };
        raws.push(raw);
        types.push(events::var_to_param(var_type));
    }
    (raws, types)
}

/// Runs `f` with the arguments as words, their strings on the guest stack meanwhile.
fn with_words<T>(e: &mut Engine, raws: &[Raw], f: impl FnOnce(&mut Engine, Vec<u32>) -> T) -> T {
    let size: u32 = raws
        .iter()
        .map(|r| match r {
            Raw::Text(s) => (s.len() as u32 + 1).next_multiple_of(4),
            Raw::Word(_) => 0,
        })
        .sum();
    e.with_stack(size.max(4), |e, base| {
        let mut at = base.addr();
        let mut words = Vec::new();
        for r in raws {
            match r {
                Raw::Word(w) => words.push(*w),
                Raw::Text(s) => {
                    e.mem.write(at, s);
                    e.mem.set_u8(at + s.len() as u32, 0);
                    words.push(at);
                    at += (s.len() as u32 + 1).next_multiple_of(4);
                }
            }
        }
        f(e, words)
    })
}

fn dispatch_event_alt(e: &mut Engine, w: &[u32]) -> Ret {
    run(e, w, |e, a, ev, toks| {
        let first = toks.first()?;
        let name = ev.get_string(e, first);
        let Some(at) = events::find(e, &name) else {
            // The event may just have no handlers yet.
            return Some(1.0);
        };
        if events::state_info(e, at).flags & events::FLAG_ALLOW_SCRIPT_DISPATCH == 0 {
            ev.error_text(format!(
                "Event {} is set up to not allow script dispatch.",
                String::from_utf8_lossy(&name)
            ));
            return None;
        }
        let (raws, types) = raw_args(e, ev, &toks, 1);
        let this_obj = a.this_obj;
        let r = with_words(e, &raws, |e, words| {
            if !events::validate_dispatched_types(e, at, &types, Some(ev)) {
                ev.error_text(format!(
                    "Caught attempt to dispatch event {} with invalid args.",
                    String::from_utf8_lossy(&name)
                ));
                return Dispatch::GenericError;
            }
            events::dispatch_raw(e, at, this_obj, &words, &types, false, true, Some(ev))
        });
        Some(f64::from(u8::from(r > Dispatch::GenericError)))
    })
}

/// A form's id (`refID`).
fn ref_id(e: &Engine, form: u32) -> u32 {
    if form == 0 {
        0
    } else {
        e.mem.u32(form + 0xc)
    }
}

/// `ArrayElement` to the value it sets (filters are never invalid).
fn value_of(d: &Data) -> Option<Value> {
    match d {
        Data::Number(n) => Some(Value::Number(*n)),
        Data::Form(id) => Some(Value::Form(*id)),
        Data::Str(s) => Some(Value::Str(s.clone())),
        Data::Array(id) => Some(Value::Array(*id)),
        Data::Invalid => None,
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), EventManager.cpp:
// EventCallback::GetFiltersAsArray, EventCallback::GetAsArray.
/// A handler as an array: `[0]` its script, `[1]` its filters by `"first"`, `"second"` and
/// index.
fn callback_array(e: &mut Engine, cb: &Callback, modi: u8) -> u32 {
    let filters = vars::create_array(e, vars::KEY_STRING, false, modi);
    if cb.source != 0 {
        let id = ref_id(e, cb.source);
        vars::set_element(e, filters, &Key::Str(b"first".to_vec()), Value::Form(id));
    }
    if cb.object != 0 {
        let id = ref_id(e, cb.object);
        vars::set_element(e, filters, &Key::Str(b"second".to_vec()), Value::Form(id));
    }
    for (index, data) in &cb.filters {
        if let Some(v) = value_of(data) {
            let key = Key::Str(index.to_string().into_bytes());
            vars::set_element(e, filters, &key, v);
        }
    }
    let arr = vars::create_array(e, vars::KEY_NUMBER, true, modi);
    let script = ref_id(e, cb.script);
    vars::set_element(e, arr, &Key::Number(0.0), Value::Form(script));
    vars::set_element(e, arr, &Key::Number(1.0), Value::Array(filters));
    arr
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Script.cpp:
// Cmd_GetEventHandlers_Execute.
fn get_event_handlers(e: &mut Engine, w: &[u32]) -> Ret {
    run(e, w, |e, a, ev, toks| {
        let modi = vars::script_mod_index(e, a.script);
        let mut only = None;
        let mut script_filter = 0;
        let mut priority_filter = PRIORITY_INVALID;
        if let Some(first) = toks.first() {
            let name = ev.get_string(e, first);
            if !name.is_empty() {
                only = Some(events::find(e, &name)?);
            }
            if let Some(t) = toks.get(1) {
                script_filter = crate::tick::user_function(e, ev, t);
                if let Some(t) = toks.get(2) {
                    priority_filter = crate::args::f64_to_i32(ev.get_number(e, t));
                }
            }
        }
        let (raws, types) = raw_args(e, ev, &toks, 3);
        let this_obj = a.this_obj;
        with_words(e, &raws, |e, words| {
            let handlers = |e: &mut Engine, ev: &mut Eval, at: usize| -> u32 {
                if !types.is_empty() && !events::validate_dispatched_types(e, at, &types, Some(ev))
                {
                    return vars::create_array(e, vars::KEY_NUMBER, true, modi);
                }
                let info = events::state_info(e, at).clone();
                let accurate = if info.flags & events::FLAG_UNKNOWN_ARG_TYPES != 0 {
                    types.clone()
                } else {
                    info.arg_types()
                };
                let add =
                    |e: &mut Engine, ev: &mut Eval, x: &events::Entry, pos: usize, arr: u32| {
                        let cb = &x.callback;
                        let matches = (script_filter == 0 || script_filter == cb.script)
                            && !cb.removed
                            && (words.is_empty()
                                || events::filters_match_alt(
                                    e,
                                    cb,
                                    this_obj,
                                    &words,
                                    &accurate,
                                    &info.name,
                                    Some(ev),
                                ));
                        if matches {
                            let one = callback_array(e, cb, modi);
                            vars::set_element(e, arr, &Key::Number(pos as f64), Value::Array(one));
                        }
                    };
                if priority_filter != PRIORITY_INVALID {
                    let arr = vars::create_array(e, vars::KEY_NUMBER, true, modi);
                    let range: Vec<&events::Entry> = info
                        .callbacks
                        .iter()
                        .filter(|x| x.priority == priority_filter)
                        .collect();
                    for (pos, x) in range.into_iter().enumerate() {
                        add(e, ev, x, pos, arr);
                    }
                    return arr;
                }
                let all = vars::create_array(e, vars::KEY_NUMBER, false, modi);
                let mut i = 0;
                while i < info.callbacks.len() {
                    let priority = info.callbacks[i].priority;
                    let arr = vars::create_array(e, vars::KEY_NUMBER, true, modi);
                    let mut pos = 0;
                    while i < info.callbacks.len() && info.callbacks[i].priority == priority {
                        add(e, ev, &info.callbacks[i], pos, arr);
                        pos += 1;
                        i += 1;
                    }
                    if vars::array(e, arr).is_some_and(|x| !x.is_empty()) {
                        vars::set_element(
                            e,
                            all,
                            &Key::Number(f64::from(priority)),
                            Value::Array(arr),
                        );
                    }
                }
                all
            };
            match only {
                Some(at) => Some(f64::from(handlers(e, ev, at))),
                None => {
                    if !words.is_empty() {
                        ev.error_text("Args were passed to get filtered, but no eventName was passed; halting the function to prevent potential error message spam due to passing invalidly typed args for some events.".to_string());
                        return None;
                    }
                    let map = vars::create_array(e, vars::KEY_STRING, false, modi);
                    for at in 0..events::count(e) {
                        let name = events::state_info(e, at).name.clone();
                        let arr = handlers(e, ev, at);
                        vars::set_element(e, map, &Key::Str(name), Value::Array(arr));
                    }
                    Some(f64::from(map))
                }
            }
        })
    })
}

/// `ScriptHandlerFilters::ShouldIgnore`: a removed handler, or one whose script the form (a
/// script or a form list of scripts) names.
fn should_ignore(e: &Engine, cb: &Callback, scripts: u32) -> bool {
    if cb.removed {
        return true;
    }
    if scripts == 0 {
        return false;
    }
    if scripts == cb.script {
        return true;
    }
    if e.mem.u8(scripts + 4) == 0x55 {
        let mut node = scripts + 0x18;
        while node != 0 {
            let (data, next) = (e.mem.u32(node), e.mem.u32(node + 4));
            if data == 0 && next == 0 {
                break;
            }
            if data == cb.script {
                return true;
            }
            node = next;
        }
    }
    false
}

/// The arguments of the four priority queries: the event, the function, the start priority,
/// the scripts to ignore; `None` (and the error) for priority 0.
fn priority_query(e: &mut Engine, ev: &mut Eval, toks: &[Tok]) -> Option<(Vec<u8>, u32, i32, u32)> {
    let name = ev.get_string(e, toks.first()?);
    let func = crate::tick::user_function(e, ev, toks.get(1)?);
    let start = crate::args::f64_to_i32(ev.get_number(e, toks.get(2)?));
    if start == PRIORITY_INVALID {
        ev.error_text("Cannot use reserved priority 0 for this function.".to_string());
        return None;
    }
    let ignore = toks.get(3).map_or(0, |t| ev.get_form(e, t));
    Some((name, func, start, ignore))
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), EventManager.h: IsEventHandlerFirstOrLast.
fn first_or_last(
    e: &mut Engine,
    name: &[u8],
    func: u32,
    start: i32,
    ignore: u32,
    first: bool,
) -> bool {
    let Some(at) = events::find(e, name) else {
        return false;
    };
    let list = events::state_info(e, at).callbacks.clone();
    if list.is_empty() {
        return false;
    }
    let Some(begin) = list.iter().position(|x| x.priority == start) else {
        return false;
    };
    let mut found = false;
    let walk: Vec<&events::Entry> = if first {
        let end = list
            .iter()
            .rposition(|x| x.priority == start)
            .unwrap_or(begin);
        list[..=end].iter().rev().collect()
    } else {
        list[begin..].iter().collect()
    };
    for x in walk {
        if should_ignore(e, &x.callback, ignore) {
            continue;
        }
        if x.callback.script != func {
            return false;
        }
        if x.priority == start {
            found = true;
        }
    }
    found
}

fn is_event_handler_first(e: &mut Engine, w: &[u32]) -> Ret {
    run(e, w, |e, _, ev, toks| {
        let (name, func, start, ignore) = priority_query(e, ev, &toks)?;
        Some(f64::from(u8::from(first_or_last(
            e, &name, func, start, ignore, true,
        ))))
    })
}

fn is_event_handler_last(e: &mut Engine, w: &[u32]) -> Ret {
    run(e, w, |e, _, ev, toks| {
        let (name, func, start, ignore) = priority_query(e, ev, &toks)?;
        Some(f64::from(u8::from(first_or_last(
            e, &name, func, start, ignore, false,
        ))))
    })
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), EventManager.h:
// GetHigherOrLowerPriorityEventHandlers.
/// The other handlers at or above (`higher`) or at or below the function's start priority, by
/// priority; 0 when the function is not at the start priority.
#[allow(clippy::too_many_arguments)]
fn higher_or_lower(
    e: &mut Engine,
    name: &[u8],
    func: u32,
    start: i32,
    ignore: u32,
    modi: u8,
    higher: bool,
) -> u32 {
    let result = vars::create_array(e, vars::KEY_NUMBER, false, modi);
    let Some(at) = events::find(e, name) else {
        return result;
    };
    let list = events::state_info(e, at).callbacks.clone();
    if list.is_empty() {
        return result;
    }
    let begin = list.iter().position(|x| x.priority == start);
    let walk: Vec<&events::Entry> = match (higher, begin) {
        (true, None) => return 0,
        (true, Some(b)) => {
            let end = list.iter().rposition(|x| x.priority == start).unwrap_or(b);
            list[..=end].iter().rev().collect()
        }
        (false, None) => Vec::new(),
        (false, Some(b)) => list[b..].iter().collect(),
    };
    let mut found = false;
    for x in walk {
        if should_ignore(e, &x.callback, ignore) {
            continue;
        }
        if x.callback.script != func {
            let one = callback_array(e, &x.callback, modi);
            vars::set_element(
                e,
                result,
                &Key::Number(f64::from(x.priority)),
                Value::Array(one),
            );
        } else if x.priority == start {
            found = true;
        }
    }
    if found {
        result
    } else {
        0
    }
}

fn get_higher_priority_event_handlers(e: &mut Engine, w: &[u32]) -> Ret {
    run(e, w, |e, a, ev, toks| {
        let (name, func, start, ignore) = priority_query(e, ev, &toks)?;
        let modi = vars::script_mod_index(e, a.script);
        Some(f64::from(higher_or_lower(
            e, &name, func, start, ignore, modi, true,
        )))
    })
}

fn get_lower_priority_event_handlers(e: &mut Engine, w: &[u32]) -> Ret {
    run(e, w, |e, a, ev, toks| {
        let (name, func, start, ignore) = priority_query(e, ev, &toks)?;
        let modi = vars::script_mod_index(e, a.script);
        Some(f64::from(higher_or_lower(
            e, &name, func, start, ignore, modi, false,
        )))
    })
}
