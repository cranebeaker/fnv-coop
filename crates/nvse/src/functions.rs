//! User-defined functions (ADR-0016 §7; build step I-7): `Begin Function` blocks, `Call`,
//! `SetFunctionValue`, `GetCallingScript`, `CallFunctionCond`, and the Rust callers events and
//! delayed calls use (`InternalFunctionCaller` and its kinds), translated from xNVSE's
//! `FunctionScripts.cpp` (xNVSE).
//!
//! - **The function's header** (`FunctionInfo`): past the script name line and any
//!   `PluginVersion` lines, a `Begin` line of block type `0x0D`; ten bytes on, a version byte (0
//!   or 1), the parameter count and each parameter as its local's id (`u16`) and its type
//!   (`u8`: float, int, string, array, ref). Read once per script per game thread and kept, with
//!   an event list made for it then (the first call's list; nested calls of the same function get
//!   their own).
//! - **A call** (`UserFunctionManager::Call`): at most 30 deep per thread; the arguments are
//!   evaluated by the caller's evaluator into the function's locals, the script runs through the
//!   game's `Script::Execute` (`005ac1e0`), and `SetFunctionValue` leaves the result on the
//!   innermost call of the same script. A list made for a nested call is destroyed after it; the
//!   kept one has its variables reset.
//! - **`Begin Function`** runs because xNVSE writes its `Function` record over the block type
//!   table's unused entry `0x0D` (`0x0118E4F8`, keeping its opcode): the block's handler answers
//!   1 when the header was read, so `EvaluateLine` runs the block.
//!
//! The managers are per game thread, as xNVSE's thread-local `UserFunctionManager`.

use crate::args::{event_list_variable, f64_to_i32};
use crate::expr::{self, Eval, Tok, Ty};
use crate::table::{Command, Param as CmdParam, Returns};
use crate::vars::{self, VarType};
use crate::{handled, lambda, CommandArgs, EvalArgs, Nvse};
use engine_core::prelude::*;
use engine_core::thread_host::current_id;
use std::collections::HashMap;

/// `Script::Execute` (`thiscall`: calling reference, event list, containing reference, flag).
pub const SCRIPT_EXECUTE: u32 = 0x005a_c1e0;
/// `Script::CreateEventList` (`thiscall`).
pub const CREATE_EVENT_LIST: u32 = 0x005a_bf60;
/// `ScriptEventList`'s deleting destructor (`thiscall`, `doFree`): wrapped by [`vars`], which
/// gives up the list's strings and arrays first, as xNVSE's `DeleteEventList` does.
pub const EVENT_LIST_DESTROY: u32 = 0x0041_af70;
/// `UserFunctionManager::kMaxNestDepth`.
pub const MAX_NEST_DEPTH: u32 = 30;
/// `kMaxUdfParams`.
pub const MAX_PARAMS: usize = 15;
/// `UserFunctionManager::kVersion`: the caller version of internal calls.
pub const CALLER_VERSION: u8 = 1;
/// The block type of `Begin Function` and its record in the block type table (`Unused_1`).
pub const FUNCTION_BLOCK: u8 = 0x0d;
pub const FUNCTION_BLOCK_RECORD: u32 = 0x0118_e4f8;
/// `PluginVersion`, skipped before the `Begin` line.
const PLUGIN_VERSION: u16 = 0x1676;
/// The opcodes of the commands here.
pub const OP_FUNCTION: u32 = 0x1544;
pub const OP_CALL: u32 = 0x1545;
pub const OP_SET_FUNCTION_VALUE: u32 = 0x1546;
const OP_GET_CALLING_SCRIPT: u32 = 0x1538;
const OP_CALL_FUNCTION_COND: u32 = 0x1636;

// ---- the script's own fields (`Script`, `GameScript.h` (xNVSE)) ----

/// `info.dataLength`, `info.type`, `data`, `quest`.
const SCRIPT_DATA_LENGTH: u32 = 0x20;
const SCRIPT_TYPE: u32 = 0x28;
const SCRIPT_DATA: u32 = 0x30;
const SCRIPT_QUEST: u32 = 0x40;

/// The script's compiled data.
pub fn script_data(e: &Engine, script: u32) -> u32 {
    e.mem.u32(script + SCRIPT_DATA)
}

/// `Script::IsUserDefinedFunction`: the block type byte of its first `Begin` line.
pub fn is_user_function(e: &Engine, script: u32) -> bool {
    let data = script_data(e, script);
    data != 0 && e.mem.u8(data + 8) == FUNCTION_BLOCK
}

/// `IsQuestScript() && quest`: the quest, or 0.
pub fn quest_of(e: &Engine, script: u32) -> u32 {
    if e.mem.u16(script + SCRIPT_TYPE) == 1 {
        e.mem.u32(script + SCRIPT_QUEST)
    } else {
        0
    }
}

/// A parameter (`UserFunctionParam`, a `VariableStorage`): its local's id and its type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Param {
    pub idx: u16,
    pub var_type: u8,
}

impl Param {
    /// `VariableStorage::ResolveVariable`: the local, or 0 (a stack variable index of 0 is
    /// invalid).
    fn resolve(&self, e: &Engine, event_list: u32) -> u32 {
        if self.var_type >= 5 || self.idx == 0 {
            return 0;
        }
        event_list_variable(e, event_list, u32::from(self.idx))
    }
}

/// `FunctionInfo`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Info {
    pub script: u32,
    /// The bytecode version of the `Function` statement (`0xFF` when not read).
    pub version: u8,
    pub bad: bool,
    pub instances: u8,
    /// The kept event list (none for a lambda).
    pub event_list: u32,
    pub params: Vec<Param>,
    /// A `{} => ...` lambda's one `SetFunctionValue` line: the address of its data, or 0.
    pub single_line: u32,
    pub is_lambda: bool,
}

/// `FunctionContext`: one call on the stack.
#[derive(Clone, Debug, PartialEq)]
struct Frame {
    script: u32,
    event_list: u32,
    result: Option<Tok>,
    invoking: u32,
    lambda_backup: bool,
}

/// `UserFunctionManager`.
#[derive(Default)]
struct Manager {
    depth: u32,
    stack: Vec<Frame>,
    infos: HashMap<u32, Info>,
}

/// The managers, per game thread.
#[derive(Default)]
pub struct State {
    managers: HashMap<u32, Manager>,
}

fn manager(e: &mut Engine) -> &mut Manager {
    let thread = current_id(e);
    e.ext::<Nvse>()
        .functions
        .managers
        .entry(thread)
        .or_default()
}

/// xNVSE's `ShowRuntimeError`: `Error in script <id> (<name>) in mod <mod>` and the message,
/// printed to the console (xNVSE prints nothing before it has recorded the main thread; the
/// port's console prints only once the game made it). For a lambda, its parent's id and name.
pub fn runtime_error(e: &mut Engine, script: u32, message: &str) {
    crate::errors::show(e, script, message);
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), FunctionScripts.cpp:
// FunctionInfo::FunctionInfo, IsSingleLineLambda.
/// Reads a function's header.
pub fn read_info(e: &mut Engine, script: u32) -> Info {
    let mut info = Info {
        script,
        version: 0xff,
        is_lambda: lambda::is_lambda(e, script),
        ..Info::default()
    };
    if script == 0 || script_data(e, script) == 0 {
        return info;
    }
    if e.mem.u32(script + SCRIPT_DATA_LENGTH) < 15 {
        return info;
    }
    let mut data = script_data(e, script) + 4;
    while e.mem.u16(data) == PLUGIN_VERSION {
        data += u32::from(e.mem.u16(data + 2)) + 4;
    }
    if e.mem.u8(data + 4) != FUNCTION_BLOCK {
        runtime_error(
            e,
            script,
            "Begin Function block not found in compiled script data",
        );
        return info;
    }
    data += 10;
    info.version = e.mem.u8(data);
    data += 1;
    if info.version > 1 {
        let text = format!("Unknown function version {:02X}.", info.version);
        runtime_error(e, script, &text);
        return info;
    }
    let count = e.mem.u8(data);
    data += 1;
    for _ in 0..count {
        let idx = e.mem.u16(data);
        let var_type = e.mem.u8(data + 2);
        data += 3;
        info.params.push(Param { idx, var_type });
    }
    if !info.is_lambda {
        info.event_list = e.call(CREATE_EVENT_LIST, &args![script]).u32();
        if info.event_list == 0 {
            runtime_error(
                e,
                script,
                "Cannot create variable list for function script.",
            );
            info.bad = true;
        }
    } else {
        info.single_line = single_line_lambda(e, script);
    }
    info
}

/// `IsSingleLineLambda`: the script is `scn`, `Begin`, `SetFunctionValue`, `End` and nothing
/// else in that order; the address of the `SetFunctionValue` line's data, or 0.
fn single_line_lambda(e: &Engine, script: u32) -> u32 {
    let mut at = script_data(e, script);
    let mut position = 0;
    for opcode in [0x1d, 0x10, OP_SET_FUNCTION_VALUE, 0x11] {
        // `ScriptIterator::ReadLine`: a reference call reads its index first.
        let mut code = u32::from(e.mem.u16(at));
        let mut data = at + 2;
        if code == 0x1c {
            code = u32::from(e.mem.u16(data + 2));
            data += 4;
        }
        let length = u32::from(e.mem.u16(data));
        data += 2;
        if code != opcode {
            return 0;
        }
        if opcode == OP_SET_FUNCTION_VALUE {
            position = data;
        }
        at = data + length;
    }
    position
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), FunctionScripts.cpp:
// UserFunctionManager::GetFunctionInfo.
/// The function's header on this thread, read the first time (`m_functionInfos.Emplace`); `None`
/// when it is bad.
fn function_info(e: &mut Engine, script: u32) -> Option<Info> {
    if let Some(info) = manager(e).infos.get(&script) {
        return (!info.bad).then(|| info.clone());
    }
    let info = read_info(e, script);
    manager(e).infos.insert(script, info.clone());
    (!info.bad).then_some(info)
}

fn set_instances(e: &mut Engine, script: u32, delta: i32) {
    if let Some(info) = manager(e).infos.get_mut(&script) {
        info.instances = (i32::from(info.instances) + delta) as u8;
    }
}

/// `UserFunctionManager::Enter`: the `Begin Function` block runs when its header reads.
pub fn enter(e: &mut Engine, script: u32) -> bool {
    function_info(e, script).is_some()
}

/// `UserFunctionManager::ClearInfos`: forgets this thread's headers (xNVSE does it when the
/// script data cache is cleared). Their kept event lists are destroyed (`~FunctionInfo`).
pub fn clear_infos(e: &mut Engine) {
    let infos: Vec<Info> = manager(e).infos.drain().map(|(_, i)| i).collect();
    for info in infos {
        if info.event_list != 0 {
            delete_event_list(e, info.event_list);
        }
    }
}

/// `OtherHooks::DeleteEventList`: lambdas lose it as their parent, its strings and arrays are
/// given up and the game destroys and frees it (the wrapped `0041af70` does both).
pub fn delete_event_list(e: &mut Engine, event_list: u32) {
    e.call(EVENT_LIST_DESTROY, &args![event_list, 1u32]);
}

/// `ScriptEventList::ResetAllVariables`: its strings and arrays given up, every local 0.
pub fn reset_all_variables(e: &mut Engine, event_list: u32) -> u32 {
    vars::clean_up_event_list(e, event_list);
    let mut count = 0;
    let mut node = e.mem.u32(event_list + 0xc);
    while node != 0 {
        let local = e.mem.u32(node);
        if local != 0 {
            e.mem.set_f64(local + 8, 0.0);
            count += 1;
        }
        node = e.mem.u32(node + 4);
    }
    count
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), FunctionScripts.cpp:
// FunctionContext::FunctionContext.
fn create_frame(e: &mut Engine, info: &Info, version: u8, invoking: u32) -> Option<Frame> {
    if info.bad {
        return None;
    }
    if version > 2 {
        let text = format!("Unknown function version {version:02X}");
        runtime_error(e, info.script, &text);
        return None;
    }
    let mut lambda_backup = false;
    let event_list = if info.is_lambda {
        let mut list = lambda::parent_event_list(e, info.script);
        if list == 0 {
            lambda_backup = true;
            list = e.call(CREATE_EVENT_LIST, &args![info.script]).u32();
        }
        list
    } else if info.instances != 0 {
        e.call(CREATE_EVENT_LIST, &args![info.script]).u32()
    } else if info.event_list != 0 {
        info.event_list
    } else {
        e.call(CREATE_EVENT_LIST, &args![info.script]).u32()
    };
    if event_list == 0 {
        let text = if info.is_lambda {
            "Failed to retrieve parent variable list for lambda script"
        } else if info.instances != 0 {
            "Couldn't create variable list for function script"
        } else {
            "Couldn't recreate variable list for function script"
        };
        runtime_error(e, info.script, text);
        return None;
    }
    Some(Frame {
        script: info.script,
        event_list,
        result: None,
        invoking,
        lambda_backup,
    })
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), FunctionScripts.cpp:
// FunctionContext::~FunctionContext.
fn destroy_frame(e: &mut Engine, frame: &Frame) {
    let (is_lambda, kept) = manager(e)
        .infos
        .get(&frame.script)
        .map_or((false, 0), |i| (i.is_lambda, i.event_list));
    if frame.event_list != 0 && (!is_lambda || frame.lambda_backup) {
        lambda::mark_parent_deleted(e, frame.event_list);
        if frame.event_list != kept {
            delete_event_list(e, frame.event_list);
        } else {
            reset_all_variables(e, frame.event_list);
        }
    }
}

/// Who calls a function (`FunctionCaller` and its kinds).
pub enum Caller<'a> {
    /// `ScriptFunctionCaller`: `Call`'s evaluator, which reads the caller version byte, the
    /// script and the arguments from the line.
    Script { eval: &'a mut Eval, version: u8 },
    /// `InternalFunctionCaller` (with `alt`, `InternalFunctionCallerAlt`: numbers always passed
    /// as `float` bits): up to 15 raw words (an `int`, a `float`'s bits, a form's address, a
    /// guest string's address or an array id, by the parameter's type).
    Internal {
        script: u32,
        this_obj: u32,
        containing: u32,
        args: Vec<u32>,
        alt: bool,
        allow_surplus: bool,
    },
    /// `ArrayElementArgFunctionCaller`: values of array elements (`None`: no arguments).
    Elements {
        script: u32,
        this_obj: u32,
        containing: u32,
        args: Option<Vec<vars::Data>>,
    },
}

impl Caller<'_> {
    /// `InternalFunctionCaller(script, callingObj, container)` with `SetArgs`.
    pub fn internal(
        script: u32,
        this_obj: u32,
        containing: u32,
        args: Vec<u32>,
    ) -> Caller<'static> {
        Caller::Internal {
            script,
            this_obj,
            containing,
            args,
            alt: false,
            allow_surplus: false,
        }
    }

    fn this_obj(&self) -> u32 {
        match self {
            Caller::Script { eval, .. } => eval.this_obj,
            Caller::Internal { this_obj, .. } | Caller::Elements { this_obj, .. } => *this_obj,
        }
    }

    fn containing(&self) -> u32 {
        match self {
            Caller::Script { eval, .. } => eval.containing_obj,
            Caller::Internal { containing, .. } | Caller::Elements { containing, .. } => {
                *containing
            }
        }
    }

    fn invoking_script(&self) -> u32 {
        match self {
            Caller::Script { eval, .. } => eval.script,
            _ => 0,
        }
    }

    fn version(&mut self, e: &mut Engine) -> u8 {
        match self {
            Caller::Script { eval, version } => {
                *version = eval.read_byte(e);
                *version
            }
            _ => CALLER_VERSION,
        }
    }

    // Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), FunctionScripts.cpp:
    // ScriptFunctionCaller::ReadScript.
    fn script(&mut self, e: &mut Engine) -> u32 {
        let (eval, version) = match self {
            Caller::Script { eval, version } => (eval, *version),
            Caller::Internal { script, .. } | Caller::Elements { script, .. } => return *script,
        };
        let tok = match version {
            0 => eval.read_one_token(e),
            1 => eval.evaluate(e),
            v => {
                eval.error_text(format!(
                    "Unknown bytecode version {v} encountered in Call statement"
                ));
                return 0;
            }
        };
        let Some(tok) = tok else {
            return 0;
        };
        let form = eval.get_form(e, &tok);
        let script = if form != 0 && e.mem.u8(form + 4) == 0x11 {
            form
        } else {
            0
        };
        if script == 0 && form != 0 {
            let text = format!(
                "Call statement received invalid form for the script arg. Said form {:08X} has type {}.",
                e.mem.u32(form + 0xc),
                e.mem.u8(form + 4)
            );
            eval.error_text(text);
        }
        // xNVSE quotes the line decompiled by its script analyser (I-14); without it, its own
        // text for a line it cannot decompile.
        let in_line = crate::context::current(e).is_some_and(|c| c.script != 0 && c.cursor != 0);
        if form == 0 && in_line {
            eval.error_text(
                "Call failed! Script is NULL: '<failed to decompile line>'".to_string(),
            );
        }
        script
    }

    // Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), FunctionScripts.cpp:
    // ScriptFunctionCaller::PopulateArgs, InternalFunctionCaller::PopulateArgs,
    // InternalFunctionCallerAlt::PopulateArgs, ArrayElementArgFunctionCaller::PopulateArgs.
    fn populate(&mut self, e: &mut Engine, event_list: u32, info: &Info) -> bool {
        let modi = vars::script_mod_index(e, e.mem.u32(event_list));
        match self {
            Caller::Script { eval, .. } => {
                let Some(args) = eval.extract_args(e) else {
                    return false;
                };
                for (i, arg) in args.iter().enumerate() {
                    let Some(param) = info.params.get(i).copied() else {
                        let text = format!("Param index {i:02X} out of bounds");
                        runtime_error(e, info.script, &text);
                        return false;
                    };
                    let local = param.resolve(e, event_list);
                    if local == 0 {
                        runtime_error(
                            e,
                            info.script,
                            "Could not look up argument variable for function script",
                        );
                        return false;
                    }
                    vars::clean_up_local(e, event_list, local);
                    match param.var_type {
                        3 => {
                            if eval.can_convert(e, arg, Ty::Array) {
                                let id = eval.get_array_id(e, arg);
                                assign_to_array(e, event_list, local, id, modi);
                            }
                        }
                        2 => {
                            if eval.can_convert(e, arg, Ty::String) {
                                let text = eval.get_string(e, arg);
                                assign_to_string(e, event_list, local, &text, modi);
                            }
                        }
                        4 => {
                            if eval.can_convert(e, arg, Ty::Form) {
                                let id = eval.get_form_id(e, arg);
                                e.mem.set_u32(local + 8, id);
                            }
                        }
                        0 | 1 => {
                            if eval.can_convert(e, arg, Ty::Number) {
                                let v = eval.get_number(e, arg);
                                let v = if param.var_type == 1 { v.floor() } else { v };
                                e.mem.set_f64(local + 8, v);
                            }
                        }
                        t => {
                            let text = format!(
                                "Unknown variable type {t:02X} encountered for parameter {i}"
                            );
                            runtime_error(e, info.script, &text);
                            return false;
                        }
                    }
                }
                true
            }
            Caller::Internal {
                script,
                args,
                alt,
                allow_surplus,
                ..
            } => {
                if info.params.len() > MAX_PARAMS {
                    return false;
                }
                for (i, word) in args.iter().copied().enumerate() {
                    let Some(param) = info.params.get(i).copied() else {
                        if !*allow_surplus {
                            let text = format!("Failed to extract parameter {i}. Please verify the number of parameters in function script match those required for event.");
                            runtime_error(e, *script, &text);
                            return false;
                        }
                        return true;
                    };
                    let local = param.resolve(e, event_list);
                    if local == 0 {
                        runtime_error(
                            e,
                            *script,
                            "Could not look up argument variable for function script",
                        );
                        return false;
                    }
                    match param.var_type {
                        1 => {
                            let v = if *alt {
                                f64::from(f32::from_bits(word)).floor()
                            } else {
                                f64::from(word as i32)
                            };
                            e.mem.set_f64(local + 8, v);
                        }
                        0 => e.mem.set_f64(local + 8, f64::from(f32::from_bits(word))),
                        4 => {
                            let id = if word != 0 { e.mem.u32(word + 0xc) } else { 0 };
                            e.mem.set_u32(local + 8, id);
                        }
                        2 => {
                            let text = crate::table::guest_text(e, word);
                            assign_to_string(e, event_list, local, &text, modi);
                        }
                        3 => {
                            if vars::array_exists(e, word) {
                                assign_to_array(e, event_list, local, word, modi);
                            } else {
                                e.mem.set_f64(local + 8, 0.0);
                            }
                        }
                        t => {
                            let text =
                                format!("Unexpected param type {t:02X} in internal function call");
                            runtime_error(e, *script, &text);
                            return false;
                        }
                    }
                }
                true
            }
            Caller::Elements { script, args, .. } => {
                let Some(args) = args else {
                    return true;
                };
                if args.len() > MAX_PARAMS {
                    return false;
                }
                for (i, arg) in args.iter().enumerate() {
                    let Some(param) = info.params.get(i).copied() else {
                        let text = format!("Failed to extract parameter {i}. Please verify the number of parameters in function script match those required for event.");
                        runtime_error(e, *script, &text);
                        return false;
                    };
                    let local = param.resolve(e, event_list);
                    if local == 0 {
                        runtime_error(
                            e,
                            *script,
                            "Could not look up argument variable for function script",
                        );
                        return false;
                    }
                    let wanted = match param.var_type {
                        0 | 1 => 1,
                        4 => 2,
                        2 => 3,
                        3 => 4,
                        _ => 0,
                    };
                    if arg.type_code() != wanted {
                        let name = variable_name(e, event_list, local);
                        let text = format!(
                            "Wrong type passed for parameter {i} ({}). Cannot assign {} to {}.",
                            String::from_utf8_lossy(&name),
                            variable_type_name(param.var_type),
                            data_type_name(arg.type_code())
                        );
                        runtime_error(e, *script, &text);
                        return false;
                    }
                    match (param.var_type, arg) {
                        (1, vars::Data::Number(n)) => {
                            e.mem.set_f64(local + 8, f64::from(f64_to_i32(*n)))
                        }
                        (0, vars::Data::Number(n)) => e.mem.set_f64(local + 8, *n),
                        (4, vars::Data::Form(id)) => e.mem.set_u32(local + 8, *id),
                        (2, vars::Data::Str(s)) => {
                            let s = s.clone();
                            assign_to_string(e, event_list, local, &s, modi);
                        }
                        (3, vars::Data::Array(id)) => {
                            if vars::array_exists(e, *id) {
                                assign_to_array(e, event_list, local, *id, modi);
                            } else {
                                e.mem.set_f64(local + 8, 0.0);
                            }
                        }
                        (t, _) => {
                            let text =
                                format!("Unexpected param type {t:02X} in internal function call");
                            runtime_error(e, *script, &text);
                            return false;
                        }
                    }
                }
                true
            }
        }
    }
}

/// `VariableStorage::AssignToArray`: the local refers to the array (from the list's script's
/// mod) and is recorded as an array local of the list.
fn assign_to_array(e: &mut Engine, event_list: u32, local: u32, id: u32, modi: u8) {
    vars::add_reference_double(e, local + 8, id, modi);
    vars::collect(e, event_list, local, VarType::Array);
}

/// `VariableStorage::AssignToString`: a new temporary string for the local.
fn assign_to_string(e: &mut Engine, event_list: u32, local: u32, text: &[u8], modi: u8) {
    let id = vars::add_string(e, modi, text, true);
    e.mem.set_f64(local + 8, f64::from(id));
    vars::collect(e, event_list, local, VarType::String);
}

/// `GetVariableName`: the name of the local's variable in its list's script (`varList`,
/// `+0x4C`: `VariableInfo` with its id at `+0` and its name's text at `+0x18`).
fn variable_name(e: &Engine, event_list: u32, local: u32) -> Vec<u8> {
    let id = e.mem.u32(local);
    let script = e.mem.u32(event_list);
    let mut node = script + 0x4c;
    while node != 0 {
        let info = e.mem.u32(node);
        if info != 0 && e.mem.u32(info) == id {
            return crate::table::guest_text(e, e.mem.u32(info + 0x18));
        }
        node = e.mem.u32(node + 4);
    }
    format!("<failed to get var name {id}>").into_bytes()
}

/// `VariableTypeToName`.
pub fn variable_type_name(t: u8) -> &'static str {
    match t {
        0 => "float",
        1 => "int",
        2 => "string",
        3 => "array",
        4 => "ref",
        _ => "invalid",
    }
}

/// `DataTypeToString`.
fn data_type_name(t: u8) -> &'static str {
    match t {
        0 => "Invalid",
        1 => "Numeric",
        2 => "Form",
        3 => "String",
        4 => "Array",
        _ => "Unknown",
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), FunctionScripts.cpp:
// UserFunctionManager::Call (both), FunctionInfo::Execute, FunctionContext::Execute.
/// Calls a function script; its result as a plain token (`ToBasicToken`), if it set one.
pub fn call(e: &mut Engine, mut caller: Caller) -> Option<Tok> {
    if manager(e).depth >= MAX_NEST_DEPTH {
        let text = format!("Max nest depth {MAX_NEST_DEPTH} exceeded in function call.");
        runtime_error(e, 0, &text);
        return None;
    }
    let version = caller.version(e);
    let script = caller.script(e);
    if script == 0 {
        runtime_error(e, 0, "Could not extract function script.");
        return None;
    }
    if script_data(e, script) == 0 {
        return None;
    }
    let Some(info) = function_info(e, script) else {
        runtime_error(
            e,
            script,
            "Could not parse function info for function script",
        );
        return None;
    };
    let invoking = caller.invoking_script();
    let Some(frame) = create_frame(e, &info, version, invoking) else {
        runtime_error(
            e,
            script,
            "Could not create function context for function script",
        );
        return None;
    };
    let event_list = frame.event_list;
    manager(e).stack.push(frame);
    manager(e).depth += 1;
    set_instances(e, script, 1);
    let ran = if caller.populate(e, event_list, &info) {
        let (this_obj, containing) = (caller.this_obj(), caller.containing());
        if info.single_line != 0 {
            run_single_line(e, &info, this_obj, containing, event_list);
        } else {
            e.call(
                SCRIPT_EXECUTE,
                &args![script, this_obj, event_list, containing, 0u32],
            );
        }
        true
    } else {
        false
    };
    set_instances(e, script, -1);
    let result = if ran {
        manager(e)
            .stack
            .last()
            .filter(|f| f.script == script)
            .and_then(|f| f.result.clone())
    } else {
        None
    };
    manager(e).depth -= 1;
    let top_is_ours = manager(e).stack.last().is_some_and(|f| f.script == script);
    if top_is_ours {
        let frame = manager(e).stack.pop().expect("top frame");
        destroy_frame(e, &frame);
    } else {
        runtime_error(
            e,
            script,
            "Call stack is corrupted on return from call to function script",
        );
    }
    result
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), FunctionScripts.cpp:
// ExecuteSingleLineLambda.
/// A `{} => ...` lambda: its `SetFunctionValue` runs directly, inside a script context of its
/// own.
fn run_single_line(e: &mut Engine, info: &Info, this_obj: u32, containing: u32, list: u32) {
    let data = script_data(e, info.script);
    let record = crate::table::record_of(e, OP_SET_FUNCTION_VALUE);
    let (params, execute) = (e.mem.u32(record + 0x14), e.mem.u32(record + 0x18));
    crate::context::push(
        e,
        crate::context::Context {
            script: info.script,
            owner_ref: this_obj,
            command: record,
            ..Default::default()
        },
    );
    e.with_stack(12, |e, frame| {
        let (result, offset) = (frame.addr(), frame.addr() + 8);
        e.mem.set_f64(result, 0.0);
        e.mem.set_u32(offset, info.single_line - data);
        e.call(
            execute,
            &[
                params,
                data,
                this_obj,
                containing,
                info.script,
                list,
                result,
                offset,
            ],
        );
    });
    crate::context::pop(e);
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), FunctionScripts.cpp:
// UserFunctionManager::Return, FunctionContext::Return.
/// `SetFunctionValue`: the innermost call of the running script takes the value.
fn set_return(e: &mut Engine, ev: &mut Eval) -> bool {
    let top_is_ours = manager(e)
        .stack
        .last()
        .is_some_and(|f| f.script == ev.script);
    if !top_is_ours {
        return false;
    }
    let args = ev.extract_args(e);
    let value = match args.as_deref() {
        Some([one]) => to_basic(e, ev, one),
        _ => None,
    };
    let ok = value.is_some();
    if let Some(top) = manager(e).stack.last_mut() {
        top.result = value;
    }
    ok
}

/// `ScriptToken::ToBasicToken`: a string, an array, a form or a number, in that order.
pub fn to_basic(e: &mut Engine, ev: &Eval, t: &Tok) -> Option<Tok> {
    if ev.can_convert(e, t, Ty::String) {
        Some(Tok::Str(ev.get_string(e, t)))
    } else if ev.can_convert(e, t, Ty::Array) {
        Some(Tok::Array(ev.get_array_id(e, t)))
    } else if ev.can_convert(e, t, Ty::Form) {
        Some(Tok::Form(ev.get_form_id(e, t)))
    } else if ev.can_convert(e, t, Ty::Number) {
        Some(Tok::number(ev.get_number(e, t)))
    } else {
        None
    }
}

/// `UserFunctionManager::GetInvokingScript`: the script that called the innermost call of
/// `script`, or 0.
pub fn invoking_script(e: &mut Engine, script: u32) -> u32 {
    manager(e)
        .stack
        .last()
        .filter(|f| f.script == script)
        .map_or(0, |f| f.invoking)
}

/// `UserFunctionManager::GetFunctionParamTypes`: the parameters' types, or `None`.
pub fn param_types(e: &mut Engine, script: u32) -> Option<Vec<u8>> {
    function_info(e, script).map(|i| i.params.iter().map(|p| p.var_type).collect())
}

// ---- the commands ----

const ONE_OPTIONAL_INT: &[CmdParam] = &[CmdParam {
    name: "int",
    kind: 0x01,
    optional: true,
}];
const ONE_STRING: &[CmdParam] = &[CmdParam {
    name: "string",
    kind: 0x00,
    optional: false,
}];
/// `kNVSEParamType_BasicType`.
const ONE_BASIC_TYPE: &[CmdParam] = &[CmdParam {
    name: "expression",
    kind: 0x804d,
    optional: false,
}];
/// `kParams_CallFunctionCond`: a form list and an optional integer.
const CALL_FUNCTION_COND: &[CmdParam] = &[
    CmdParam {
        name: "UDF FormList",
        kind: 0x25,
        optional: false,
    },
    CmdParam {
        name: "bBreakIfFalse",
        kind: 0x01,
        optional: true,
    },
];

pub static COMMANDS: &[Command] = &[
    Command {
        opcode: OP_GET_CALLING_SCRIPT,
        name: "GetCallingScript",
        short: "",
        help: "returns the script that called the executing function script",
        needs_ref: false,
        params: &[],
        execute: get_calling_script,
        eval: None,
        returns: Returns::Form,
    },
    Command {
        opcode: OP_FUNCTION,
        name: "Function",
        short: "",
        help: "defines a function",
        needs_ref: false,
        params: ONE_OPTIONAL_INT,
        execute: function_execute,
        eval: None,
        returns: Returns::Default,
    },
    Command {
        opcode: OP_CALL,
        name: "Call",
        short: "",
        help: "calls a user-defined function",
        needs_ref: false,
        params: ONE_STRING,
        execute: call_execute,
        eval: None,
        returns: Returns::Ambiguous,
    },
    Command {
        opcode: OP_SET_FUNCTION_VALUE,
        name: "SetFunctionValue",
        short: "",
        help: "returns a value from a user-defined function",
        needs_ref: false,
        params: ONE_BASIC_TYPE,
        execute: set_function_value,
        eval: None,
        returns: Returns::Default,
    },
    Command {
        opcode: OP_CALL_FUNCTION_COND,
        name: "CallFunctionCond",
        short: "",
        help: "calls every UDF in a formlist and returns the result of the last one.",
        needs_ref: false,
        params: CALL_FUNCTION_COND,
        execute: call_function_cond_execute,
        eval: Some(call_function_cond_eval),
        returns: Returns::Default,
    },
];

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Scripting.cpp:
// Cmd_Function_Execute.
fn function_execute(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    let v = if enter(e, a.script) { 1.0 } else { 0.0 };
    a.set_result(e, v);
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Scripting.cpp: Cmd_Call_Execute.
fn call_execute(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    let mut ev = Eval::new(e, &a);
    expr::with_evaluator(e, |e| {
        let result = call(
            e,
            Caller::Script {
                eval: &mut ev,
                version: 0xff,
            },
        );
        if let Some(t) = result {
            ev.assign_result(e, &t);
        }
    });
    expr::report(e, &ev);
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Scripting.cpp:
// Cmd_SetFunctionValue_Execute.
fn set_function_value(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    let mut ev = Eval::new(e, &a);
    let ok = expr::with_evaluator(e, |e| set_return(e, &mut ev));
    if !ok {
        runtime_error(e, a.script, "SetFunctionValue statement failed.");
    }
    expr::report(e, &ev);
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Script.cpp:
// Cmd_GetCallingScript_Execute.
fn get_calling_script(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    e.mem.set_u32(a.result, 0);
    let caller = invoking_script(e, a.script);
    if caller != 0 {
        let id = e.mem.u32(caller + 0xc);
        e.mem.set_u32(a.result, id);
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Scripting.cpp:
// Cmd_CallFunctionCond_Execute ("don't use this in scripts").
fn call_function_cond_execute(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Scripting.cpp:
// Cmd_CallFunctionCond_Eval.
/// Each script of the form list (`BGSListForm`, its list at `+0x18`) called with no arguments;
/// with the flag, a false result stops at 0. The last call's result: its number, else its truth.
fn call_function_cond_eval(e: &mut Engine, w: &[u32]) -> Ret {
    let a = EvalArgs::new(w);
    e.mem.set_f64(a.result, 0.0);
    let mut last: Option<Tok> = None;
    if a.arg1 != 0 {
        let break_if_false = a.arg2 != 0;
        let mut node = a.arg1 + 0x18;
        while node != 0 {
            let form = e.mem.u32(node);
            node = e.mem.u32(node + 4);
            if form == 0 || e.mem.u8(form + 4) != 0x11 {
                continue;
            }
            last = call(e, Caller::internal(form, a.this_obj, 0, Vec::new()));
            if let Some(t) = &last {
                if break_if_false && !basic_bool(t) {
                    return handled(true);
                }
            }
        }
    }
    let Some(t) = last else {
        return handled(true);
    };
    let num = match t {
        Tok::Number { v, .. } => v,
        _ => 0.0,
    };
    let v = if num != 0.0 {
        num
    } else {
        f64::from(u8::from(basic_bool(&t)))
    };
    e.mem.set_f64(a.result, v);
    handled(true)
}

/// `GetBool` of a plain token.
fn basic_bool(t: &Tok) -> bool {
    match t {
        Tok::Number { v, .. } => *v != 0.0,
        Tok::Form(id) | Tok::Array(id) => *id != 0,
        Tok::Str(s) => !s.is_empty(),
        Tok::Bool(b) => *b,
        _ => false,
    }
}

/// xNVSE's `CommandTable::Init`: "blocktype `Unused_1` becomes `Function`": the `Function`
/// record copied over block type `0x0D`'s, its opcode kept.
pub fn install(e: &mut Engine) {
    let record = crate::table::record_of(e, OP_FUNCTION);
    if record == 0 {
        return;
    }
    let opcode = e.mem.u32(FUNCTION_BLOCK_RECORD + 8);
    let bytes = e.mem.bytes(record, crate::table::RECORD);
    e.mem.write(FUNCTION_BLOCK_RECORD, &bytes);
    e.mem.set_u32(FUNCTION_BLOCK_RECORD + 8, opcode);
}

#[cfg(test)]
#[path = "functions_tests.rs"]
mod tests;
