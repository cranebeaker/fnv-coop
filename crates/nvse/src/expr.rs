//! xNVSE's expression evaluator (ADR-0016 §6; build step I-6): the bytecode its compiler writes
//! for `Let`, `eval`, `testexpr`, a parenthesised parameter (`0xFFFF`) and the loop conditions,
//! read as tokens (`ScriptToken::ReadFrom`, `ScriptTokens.cpp` (xNVSE)) and evaluated in postfix
//! order with `&&`/`||` short-circuit (`ExpressionEvaluator::Evaluate`, `ParseShortCircuit`,
//! `ScriptUtils.cpp` (xNVSE)); each operator's rules choose a handler by the operand types
//! (`s_operators`, `kOpRule_*`, `Operator::Evaluate`).
//!
//! The evaluator computes in `f64` as xNVSE does (SSE2); conversions to integers go through
//! [`crate::args::f64_to_i32`] or the C++'s `SInt64` casts, never Rust `as` on a raw double.
//!
//! Not here yet (each a script error, as an unknown operator would be): lambdas (I-7), the `.`
//! call syntax, array slices (`MakeSlice`), the assignable substrings of a string variable or
//! element (`s[1] := ...`), and the override argument path (an argument count above `0x7FFF`).

use crate::args::{event_list_variable, f64_to_i32, resolve_ref_variable, script_ref};
use crate::table::{record_of, return_type, Returns};
use crate::vars::{self, Key, Value};
use crate::Nvse;
use engine_core::prelude::*;
use engine_core::thread_host::current_id;

/// `LookupFormByID` (`004839c0`).
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `__RTDynamicCast` and `TESGlobal`'s descriptor.
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const RTTI_TES_FORM: u32 = 0x0118_3028;
const RTTI_TES_GLOBAL: u32 = 0x0118_3ad4;
const RTTI_TES_OBJECT_REFR: u32 = 0x0118_41cc;

/// `Token_Type` (xNVSE `ScriptTokens.h`): the codes the conversion table is indexed by.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ty {
    Number = 0,
    Boolean,
    String,
    Form,
    Ref,
    Global,
    Array,
    ArrayElement,
    Slice,
    Command,
    Variable,
    NumericVar,
    RefVar,
    StringVar,
    ArrayVar,
    Ambiguous,
    Operator,
    ForEachContext,
    Byte,
    Short,
    Int,
    Pair,
    AssignableString,
    Lambda,
    LambdaScriptData,
    LeftToken,
    RightToken,
    OptionalEmpty,
}

/// `CanConvertOperand` with the `kConversions_*` lists (xNVSE `ScriptTokens.cpp`).
fn can_convert(from: Ty, to: Ty) -> bool {
    use Ty::*;
    if from == to {
        return true;
    }
    let rules: &[Ty] = match from {
        Number => &[Boolean],
        Boolean => &[Number],
        String => &[Boolean],
        Form => &[Boolean],
        Ref => &[Form, Boolean],
        Global => &[Number, Boolean],
        Array => &[Boolean],
        ArrayElement => &[Number, Form, String, Array, Boolean],
        Command => &[Ambiguous, Number, Form, Boolean],
        NumericVar => &[Number, Boolean, Variable],
        RefVar => &[Form, Boolean, Variable],
        StringVar => &[String, Variable, Boolean],
        ArrayVar => &[Array, Variable, Boolean],
        AssignableString => &[String],
        Lambda => &[Form],
        _ => &[],
    };
    rules.contains(&to)
}

/// A token as the evaluator holds it.
#[derive(Clone, Debug, PartialEq)]
pub enum Tok {
    /// A number; `form_or_number` for a command's default result that is also a form's id;
    /// `ambiguous` for the result of a command whose return type it chose.
    Number {
        v: f64,
        form_or_number: bool,
        ambiguous: bool,
    },
    Bool(bool),
    Str(Vec<u8>),
    /// A form, by id.
    Form(u32),
    /// A `TESGlobal`'s address.
    Global(u32),
    Array(u32),
    /// An element of an array (`ArrayElementToken`).
    Element {
        array: u32,
        key: Key,
    },
    /// A script local: its `ScriptLocal`'s address (0 before it is resolved, or for a string
    /// a command returned), the variable type byte, its reference index and its variable id.
    Var {
        ty: Ty,
        local: u32,
        var_type: u8,
        ref_idx: u16,
        var_idx: u16,
        /// For a string variable: the string id it held when it was resolved, or the rvalue's.
        string: u32,
    },
    /// A command call (`X`, or `x` taking its calling reference from the stack).
    Command {
        ref_idx: u16,
        opcode: u32,
        offset: u32,
        from_stack: bool,
    },
    /// An operator (`kOpType_*`).
    Op(u8),
    /// A slice of numbers (`lower:upper`) or of strings.
    Slice {
        lower: Key,
        upper: Key,
    },
    /// A pair (`a::b`).
    Pair(Box<Tok>, Box<Tok>),
    /// The `_` of an optional argument left empty.
    OptionalEmpty,
    /// A `ForEach` context (`<-`): the source (an array id, a string id or a form list's
    /// address), the iterator (an array or a string id), the iterator variable's local and
    /// its type byte.
    ForEach {
        source: u32,
        iter: u32,
        local: u32,
        var_type: u8,
    },
    /// An `F` token's script data (`LambdaScriptData`): its copy and length.
    LambdaData {
        data: u32,
        len: u32,
    },
    /// A lambda (`kTokenType_Lambda`): its script.
    Lambda(u32),
}

impl Tok {
    pub fn ty(&self) -> Ty {
        match self {
            Tok::Number { .. } => Ty::Number,
            Tok::Bool(_) => Ty::Boolean,
            Tok::Str(_) => Ty::String,
            Tok::Form(_) => Ty::Form,
            Tok::Global(_) => Ty::Global,
            Tok::Array(_) => Ty::Array,
            Tok::Element { .. } => Ty::ArrayElement,
            Tok::Var { ty, .. } => *ty,
            Tok::Command { .. } => Ty::Command,
            Tok::Op(_) => Ty::Operator,
            Tok::Slice { .. } => Ty::Slice,
            Tok::Pair(..) => Ty::Pair,
            Tok::OptionalEmpty => Ty::OptionalEmpty,
            Tok::ForEach { .. } => Ty::ForEachContext,
            Tok::LambdaData { .. } => Ty::LambdaScriptData,
            Tok::Lambda(_) => Ty::Lambda,
        }
    }

    pub fn number(v: f64) -> Tok {
        Tok::Number {
            v,
            form_or_number: false,
            ambiguous: false,
        }
    }
}

/// One token of the cached bytecode and its short-circuit information.
#[derive(Clone, Debug)]
struct Entry {
    tok: Tok,
    /// `shortCircuitParentType` (`None` for `kOpType_Max`), distance and stack offset.
    sc_parent: Option<u8>,
    sc_distance: usize,
    sc_offset: usize,
}

/// `kOpType_*` (xNVSE `ScriptTokens.h`).
pub mod op {
    pub const ASSIGNMENT: u8 = 0;
    pub const LOGICAL_OR: u8 = 1;
    pub const LOGICAL_AND: u8 = 2;
    pub const SLICE: u8 = 3;
    pub const EQUALS: u8 = 4;
    pub const NOT_EQUAL: u8 = 5;
    pub const GREATER_THAN: u8 = 6;
    pub const LESS_THAN: u8 = 7;
    pub const GREATER_OR_EQUAL: u8 = 8;
    pub const LESS_OR_EQUAL: u8 = 9;
    pub const BITWISE_OR: u8 = 10;
    pub const BITWISE_AND: u8 = 11;
    pub const LEFT_SHIFT: u8 = 12;
    pub const RIGHT_SHIFT: u8 = 13;
    pub const ADD: u8 = 14;
    pub const SUBTRACT: u8 = 15;
    pub const MULTIPLY: u8 = 16;
    pub const DIVIDE: u8 = 17;
    pub const MODULO: u8 = 18;
    pub const EXPONENT: u8 = 19;
    pub const NEGATION: u8 = 20;
    pub const LOGICAL_NOT: u8 = 21;
    pub const LEFT_BRACKET: u8 = 24;
    pub const IN: u8 = 26;
    pub const TO_STRING: u8 = 27;
    pub const PLUS_EQUALS: u8 = 28;
    pub const TIMES_EQUALS: u8 = 29;
    pub const DIVIDED_EQUALS: u8 = 30;
    pub const EXPONENT_EQUALS: u8 = 31;
    pub const MINUS_EQUALS: u8 = 32;
    pub const TO_NUMBER: u8 = 33;
    pub const DEREFERENCE: u8 = 34;
    pub const MEMBER_ACCESS: u8 = 35;
    pub const MAKE_PAIR: u8 = 36;
    pub const BOX: u8 = 37;
    pub const DOT: u8 = 40;
    pub const BITWISE_OR_EQUALS: u8 = 41;
    pub const BITWISE_AND_EQUALS: u8 = 42;
    pub const MODULO_EQUALS: u8 = 43;
    pub const BITWISE_NOT: u8 = 44;
    pub const MAX: u8 = 45;
}

/// `Operator::numOperands` (`s_operators`).
fn operand_count(o: u8) -> usize {
    match o {
        op::NEGATION
        | op::LOGICAL_NOT
        | op::TO_STRING
        | op::TO_NUMBER
        | op::DEREFERENCE
        | op::BOX
        | op::BITWISE_NOT => 1,
        22 | 23 | 25 | 38 | 39 => 0,
        _ => 2,
    }
}

fn is_logical(o: u8) -> bool {
    o == op::LOGICAL_OR || o == op::LOGICAL_AND
}

/// The evaluator's context: one `ExpressionEvaluator` (xNVSE).
pub struct Eval {
    pub params: u32,
    pub data: u32,
    pub this_obj: u32,
    pub containing_obj: u32,
    pub script: u32,
    pub event_list: u32,
    pub result: u32,
    /// The address of the offset word.
    pub offset_ptr: u32,
    /// Read position (an address in the script data).
    at: u32,
    /// Errors reported (`errorMessages`).
    pub errors: Vec<String>,
    pub suppress_errors: bool,
    /// `m_baseOffset`: the offset of the command's opcode word; `None` for an inline expression
    /// (`m_inline`, no command to name).
    pub base_offset: Option<u32>,
    /// `m_inline`: a parenthesised parameter, no command named in its errors.
    pub inline: bool,
}

/// The per-thread stack of running evaluators' expected return types (`ExpectReturnType`,
/// `localData.expressionEvaluator`): commands with an ambiguous return type set the top one.
#[derive(Default)]
pub struct State {
    expected: std::collections::HashMap<u32, Vec<Returns>>,
}

/// Sets what the running evaluator expects the command's result to be
/// (`ExpressionEvaluator::Get().ExpectReturnType`); nothing without one.
pub fn expect_return_type(e: &mut Engine, ty: Returns) {
    let thread = current_id(e);
    if let Some(top) = e
        .ext::<Nvse>()
        .expr
        .expected
        .get_mut(&thread)
        .and_then(|s| s.last_mut())
    {
        *top = ty;
    }
}

/// Whether an evaluator runs on this thread (`ExpressionEvaluator::Active`).
pub fn active(e: &Engine) -> bool {
    let thread = current_id(e);
    e.ext_get::<Nvse>()
        .and_then(|n| n.expr.expected.get(&thread))
        .is_some_and(|s| !s.is_empty())
}

fn push_frame(e: &mut Engine) {
    let thread = current_id(e);
    e.ext::<Nvse>()
        .expr
        .expected
        .entry(thread)
        .or_default()
        .push(Returns::Default);
}

/// `PopFromStack`: the popped evaluator's expected return type passes to its parent, as a
/// command running its own evaluator (`Call`, or any expression-format command) reports its
/// result's type to the expression that called it.
fn pop_frame(e: &mut Engine) {
    let thread = current_id(e);
    if let Some(s) = e.ext::<Nvse>().expr.expected.get_mut(&thread) {
        if let Some(ty) = s.pop() {
            if let Some(parent) = s.last_mut() {
                *parent = ty;
            }
        }
    }
}

fn expected(e: &Engine) -> Returns {
    let thread = current_id(e);
    e.ext_get::<Nvse>()
        .and_then(|n| n.expr.expected.get(&thread))
        .and_then(|s| s.last().copied())
        .unwrap_or_default()
}

fn lookup_form(e: &mut Engine, id: u32) -> u32 {
    e.call(LOOKUP_FORM, &args![id]).u32()
}

fn cast(e: &mut Engine, object: u32, from: u32, to: u32) -> u32 {
    if object == 0 {
        return 0;
    }
    e.call(DYNAMIC_CAST, &args![object, 0u32, from, to, 0u32])
        .u32()
}

/// `TESForm::GetModIndex` of the running script.
fn script_mod(e: &Engine, script: u32) -> u8 {
    vars::script_mod_index(e, script)
}

/// `FloatEqual` (xNVSE `common/ITypes.h`): both as `float`, closer than `0.0001f`.
fn float_equal(a: f64, b: f64) -> bool {
    let mut m = a as f32 - b as f32;
    if m < 0.0 {
        m = -m;
    }
    m < 0.0001
}

/// `%g` of the C runtime, as `snprintf(buf, 0x20, "%g", x)` gives it.
pub fn format_g(v: f64) -> String {
    if v.is_nan() {
        return if v.is_sign_negative() {
            "-nan(ind)"
        } else {
            "nan"
        }
        .into();
    }
    if v.is_infinite() {
        return if v < 0.0 { "-inf" } else { "inf" }.into();
    }
    if v == 0.0 {
        return if v.is_sign_negative() { "-0" } else { "0" }.into();
    }
    let exp = v.abs().log10().floor() as i32;
    // `%g` with precision 6: the exponent of the value rounded to 6 significant digits.
    let sci = format!("{v:.5e}");
    let (mant, e10) = sci.split_once('e').unwrap_or((&sci, "0"));
    let e10: i32 = e10.parse().unwrap_or(exp);
    if !(-4..6).contains(&e10) {
        let mant = trim_zeros(mant);
        let sign = if e10 < 0 { '-' } else { '+' };
        format!("{mant}e{sign}{:02}", e10.abs())
    } else {
        let decimals = (5 - e10).max(0) as usize;
        trim_zeros(&format!("{v:.decimals$}")).to_string()
    }
}

fn trim_zeros(s: &str) -> &str {
    if s.contains('.') {
        s.trim_end_matches('0').trim_end_matches('.')
    } else {
        s
    }
}

impl Eval {
    /// An evaluator for a command's arguments (`ExpressionEvaluator(COMMAND_ARGS)`): reading
    /// from the offset word's position.
    pub fn new(e: &Engine, a: &crate::CommandArgs) -> Eval {
        let offset = e.mem.u32(a.opcode_offset);
        Eval {
            params: a.params,
            data: a.data,
            this_obj: a.this_obj,
            containing_obj: a.containing_obj,
            script: a.script,
            event_list: a.event_list,
            result: a.result,
            offset_ptr: a.opcode_offset,
            at: a.data.wrapping_add(offset),
            errors: Vec::new(),
            suppress_errors: false,
            base_offset: Some(offset.wrapping_sub(4)),
            inline: false,
        }
    }

    fn error(&mut self, text: impl Into<String>) {
        self.errors.push(text.into());
    }

    /// `ExpressionEvaluator::Error`.
    pub fn error_text(&mut self, text: String) {
        self.errors.push(text);
    }

    /// `ReadByte`: the next byte (the read position only moves).
    pub fn read_byte(&mut self, e: &Engine) -> u8 {
        self.read8(e)
    }

    /// `ScriptToken::Read`: one token as it is stored, not evaluated (`Call`'s version 0
    /// script argument).
    pub fn read_one_token(&mut self, e: &mut Engine) -> Option<Tok> {
        self.read_token(e)
    }

    fn read8(&mut self, e: &Engine) -> u8 {
        let v = e.mem.u8(self.at);
        self.at += 1;
        v
    }

    fn read16(&mut self, e: &Engine) -> u16 {
        let v = e.mem.u16(self.at);
        self.at += 2;
        v
    }

    fn read32(&mut self, e: &Engine) -> u32 {
        let v = e.mem.u32(self.at);
        self.at += 4;
        v
    }

    // Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), ScriptTokens.cpp:
    // ScriptToken::ReadFrom.
    fn read_token(&mut self, e: &mut Engine) -> Option<Tok> {
        let code = self.read8(e);
        Some(match code {
            b'B' | b'b' => Tok::number(f64::from(self.read8(e))),
            b'I' | b'i' => Tok::number(f64::from(self.read16(e))),
            b'L' | b'l' => Tok::number(f64::from(self.read32(e))),
            b'Z' => {
                let v = e.mem.f64(self.at);
                self.at += 8;
                Tok::number(v)
            }
            b'S' => {
                let len = u32::from(self.read16(e));
                let s = e.mem.bytes(self.at, len);
                self.at += len;
                Tok::Str(s)
            }
            b'R' => {
                let idx = self.read16(e);
                let ref_var = script_ref(e, self.script, u32::from(idx));
                if ref_var == 0 {
                    self.error(format!("Could not find ref {idx:X} in ref list!"));
                    return None;
                }
                resolve_ref_variable(e, ref_var, self.event_list);
                let form = e.mem.u32(ref_var + 8);
                Tok::Form(if form != 0 { e.mem.u32(form + 0xc) } else { 0 })
            }
            b'G' => {
                let idx = self.read16(e);
                let ref_var = script_ref(e, self.script, u32::from(idx));
                if ref_var == 0 {
                    self.error(format!("Could not resolve global {idx:X}"));
                    return None;
                }
                resolve_ref_variable(e, ref_var, self.event_list);
                let form = e.mem.u32(ref_var + 8);
                let global = cast(e, form, RTTI_TES_FORM, RTTI_TES_GLOBAL);
                if global == 0 {
                    self.error("Failed to resolve global");
                    return None;
                }
                Tok::Global(global)
            }
            b'X' | b'x' => {
                let ref_idx = self.read16(e);
                let opcode = u32::from(self.read16(e));
                if record_of(e, opcode) == 0 {
                    self.error(format!("Failed to resolve command with opcode {opcode:X}"));
                    return None;
                }
                let args_len = u32::from(self.read16(e));
                let offset = self.at.wrapping_sub(self.data);
                self.at = self.at.wrapping_add(args_len).wrapping_sub(2);
                Tok::Command {
                    ref_idx,
                    opcode,
                    offset,
                    from_stack: code == b'x',
                }
            }
            b'V' => {
                let var_type = self.read8(e);
                let ty = match var_type {
                    3 => Ty::ArrayVar,
                    0 | 1 => Ty::NumericVar,
                    4 => Ty::RefVar,
                    2 => Ty::StringVar,
                    _ => {
                        self.error(format!("Unsupported variable type {var_type:X}"));
                        return None;
                    }
                };
                let ref_idx = self.read16(e);
                let var_idx = self.read16(e);
                Tok::Var {
                    ty,
                    local: 0,
                    var_type,
                    ref_idx,
                    var_idx,
                    string: 0,
                }
            }
            b'F' => {
                // A lambda's script data, copied once (xNVSE copies it when it caches the
                // expression's tokens).
                let len = self.read32(e);
                let data = crate::lambda::token_data(e, self.at, len);
                self.at += len;
                Tok::LambdaData { data, len }
            }
            b'K' => Tok::OptionalEmpty,
            c if c < op::MAX => Tok::Op(c),
            c => {
                self.error(format!("Unexpected token type {c} ({c:02x}) encountered"));
                return None;
            }
        })
    }

    // Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), ScriptUtils.cpp:
    // ExpressionEvaluator::ParseBytecode, ParseShortCircuit, GetOperatorParent.
    fn parse(&mut self, e: &mut Engine) -> Option<Vec<Entry>> {
        let before = self.at;
        let len = u32::from(self.read16(e));
        let end = self.at.wrapping_add(len).wrapping_sub(2);
        let mut out = Vec::new();
        while self.at < end {
            let tok = self.read_token(e)?;
            out.push(Entry {
                tok,
                sc_parent: None,
                sc_distance: 0,
                sc_offset: 0,
            });
        }
        let increment = self.at - before;
        let now = e.mem.u32(self.offset_ptr);
        e.mem.set_u32(self.offset_ptr, now.wrapping_add(increment));
        let parent = |list: &[Entry], from: usize| -> usize {
            let mut i = from + 1;
            let mut count: i32 = 0;
            while i < list.len() {
                if let Tok::Op(o) = list[i].tok {
                    if count == 0 {
                        return i;
                    }
                    count -= operand_count(o) as i32 - 1;
                } else {
                    count += 1;
                }
                if count == 0 {
                    return i;
                }
                i += 1;
            }
            list.len()
        };
        let op_of = |list: &[Entry], i: usize| match list[i].tok {
            Tok::Op(o) => Some(o),
            _ => None,
        };
        let end = out.len();
        for cur in 0..end {
            let mut grand = cur;
            let mut furthest;
            loop {
                furthest = grand;
                grand = parent(&out, grand);
                let next = furthest + 1;
                let go_on = grand < end
                    && op_of(&out, grand).is_some_and(is_logical)
                    && (furthest == cur
                        || op_of(&out, grand) == op_of(&out, furthest)
                            && (next == end || op_of(&out, next).is_none()));
                if !go_on {
                    break;
                }
            }
            if furthest != cur && op_of(&out, furthest).is_some_and(is_logical) {
                out[cur].sc_parent = op_of(&out, furthest);
                out[cur].sc_distance = furthest - cur;
                let p = parent(&out, cur);
                out[cur].sc_offset = if cur + 1 == p { 2 } else { 1 };
            }
        }
        Some(out)
    }

    // Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), ScriptUtils.cpp:
    // ExpressionEvaluator::Evaluate.
    /// Evaluates one expression from the read position; `None` on an error.
    pub fn evaluate(&mut self, e: &mut Engine) -> Option<Tok> {
        let list = self.parse(e)?;
        let mut operands: Vec<Tok> = Vec::new();
        let mut sc_info: Vec<(Option<u8>, usize, usize)> = Vec::new();
        let mut i = 0;
        while i < list.len() {
            let entry = &list[i];
            let info = (entry.sc_parent, entry.sc_distance, entry.sc_offset);
            match &entry.tok {
                Tok::Op(o) => {
                    let o = *o;
                    let n = operand_count(o);
                    if n > operands.len() {
                        self.error("Too few operands for operator");
                        return None;
                    }
                    let rh = if n == 2 { operands.pop() } else { None };
                    let lh = operands.pop()?;
                    sc_info.truncate(operands.len());
                    let Some(res) = self.operate(e, o, lh, rh) else {
                        self.error("Operator failed to evaluate to a valid result");
                        return None;
                    };
                    operands.push(res);
                    sc_info.push(info);
                }
                tok => {
                    let tok = match tok {
                        Tok::Command {
                            from_stack: false, ..
                        } => self.execute_command(e, tok, 0)?,
                        Tok::Var { .. } => self.resolve_variable(e, tok.clone())?,
                        // One lambda per event list, so it sees that list's locals.
                        Tok::LambdaData { data, len } => {
                            let position = crate::functions::script_data(e, self.script)
                                .wrapping_add(e.mem.u32(self.offset_ptr));
                            let lambda = crate::lambda::create(
                                e,
                                position,
                                *data,
                                *len,
                                self.script,
                                self.event_list,
                            );
                            if lambda == 0 {
                                self.error("Failed to create lambda script");
                                return None;
                            }
                            Tok::Lambda(lambda)
                        }
                        _ => tok.clone(),
                    };
                    operands.push(tok);
                    sc_info.push(info);
                }
            }
            // `ShortCircuit`.
            let (parent, distance, offset) = *sc_info.last()?;
            if let Some(parent) = parent {
                let last = operands.last()?;
                let value = self.get_bool(e, last);
                if (parent == op::LOGICAL_AND && !value) || (parent == op::LOGICAL_OR && value) {
                    i += distance;
                    let last = operands.pop()?;
                    sc_info.pop();
                    for _ in 1..offset {
                        if operands.pop().is_none() {
                            self.error(
                                "An internal NVSE error occurred (short circuit stack mismatch)",
                            );
                            return None;
                        }
                        sc_info.pop();
                    }
                    operands.push(last);
                    sc_info.push((None, 0, 0));
                }
            }
            i += 1;
        }
        if operands.len() != 1 || (!self.errors.is_empty() && !self.suppress_errors) {
            self.error("An expression failed to evaluate to a valid result");
            return None;
        }
        operands.pop()
    }

    // Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), ScriptUtils.cpp:
    // ExpressionEvaluator::ExtractArgs.
    /// The number of arguments, then each evaluated; `None` unless all were.
    pub fn extract_args(&mut self, e: &mut Engine) -> Option<Vec<Tok>> {
        // The count byte moves the read position only: the offset word moves by each
        // expression's own length (`GetTokens`), as in xNVSE.
        let count = self.read8(e);
        let mut out = Vec::new();
        for _ in 0..count {
            out.push(self.evaluate(e)?);
        }
        Some(out)
    }

    // Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), ScriptTokens.cpp:
    // ScriptToken::ResolveVariable, ResolveScriptLocal.
    fn resolve_variable(&mut self, e: &mut Engine, tok: Tok) -> Option<Tok> {
        let Tok::Var {
            ty,
            var_type,
            ref_idx,
            var_idx,
            ..
        } = tok
        else {
            return Some(tok);
        };
        let mut list = self.event_list;
        if ref_idx != 0 {
            let ref_var = script_ref(e, self.script, u32::from(ref_idx));
            if ref_var != 0 {
                resolve_ref_variable(e, ref_var, list);
                let form = e.mem.u32(ref_var + 8);
                if form != 0 {
                    list = event_list_from_form(e, form);
                }
            }
        }
        let local = if list != 0 {
            event_list_variable(e, list, u32::from(var_idx))
        } else {
            0
        };
        if local == 0 {
            self.error("Failed to resolve variable");
            return None;
        }
        if (ty == Ty::ArrayVar || ty == Ty::StringVar) && ref_idx == 0 {
            let kind = if ty == Ty::StringVar {
                vars::VarType::String
            } else {
                vars::VarType::Array
            };
            vars::collect(e, self.event_list, local, kind);
        }
        let string = if ty == Ty::StringVar {
            f64_to_i32(e.mem.f64(local + 8)) as u32
        } else {
            0
        };
        Some(Tok::Var {
            ty,
            local,
            var_type,
            ref_idx,
            var_idx,
            string,
        })
    }

    // Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), ScriptUtils.cpp:
    // ExpressionEvaluator::ExecuteCommandToken.
    fn execute_command(&mut self, e: &mut Engine, tok: &Tok, stack_ref: u32) -> Option<Tok> {
        let Tok::Command {
            ref_idx,
            opcode,
            offset,
            ..
        } = *tok
        else {
            return None;
        };
        let record = record_of(e, opcode);
        let mut calling = self.this_obj;
        let mut from_ref = false;
        if ref_idx != 0 {
            let ref_var = script_ref(e, self.script, u32::from(ref_idx));
            if ref_var != 0 {
                from_ref = true;
                resolve_ref_variable(e, ref_var, self.event_list);
                let form = e.mem.u32(ref_var + 8);
                if form == 0 {
                    self.error("Attempting to call a function on a NULL reference");
                    return None;
                }
                calling = cast(e, form, RTTI_TES_FORM, RTTI_TES_OBJECT_REFR);
                if calling == 0 {
                    self.error(
                        "Attempting to call a function on a base object (this must be a reference)",
                    );
                    return None;
                }
            }
        } else if stack_ref != 0 {
            calling = stack_ref;
        }
        if e.mem.u16(record + 0x10) != 0 && calling == 0 {
            self.error("Function requires a calling reference but none or NULL was provided");
            return None;
        }
        let containing = if from_ref { 0 } else { self.containing_obj };
        let mut ret = return_type(e, opcode);
        expect_return_type(e, Returns::Default);
        let params = e.mem.u32(record + 0x14);
        let execute = e.mem.u32(record + 0x18);
        let (ok, result) = e.with_stack(16, |e, frame| {
            let res = frame.addr();
            let off = res + 8;
            e.mem.set_f64(res, 0.0);
            e.mem.set_u32(off, offset);
            let ok = e
                .call(
                    execute,
                    &[
                        params,
                        self.data,
                        calling,
                        containing,
                        self.script,
                        self.event_list,
                        res,
                        off,
                    ],
                )
                .u32()
                & 0xff;
            (ok, e.mem.f64(res))
        });
        if ok == 0 {
            self.error("Command failed to execute");
            return None;
        }
        let ambiguous = matches!(ret, Returns::Ambiguous | Returns::ArrayIndex);
        if ambiguous {
            ret = expected(e);
        }
        let bits = result.to_bits();
        let tok = match ret {
            Returns::Default => {
                let form_or_number = (bits >> 32) == 0 && lookup_form(e, bits as u32) != 0;
                Tok::Number {
                    v: result,
                    form_or_number,
                    ambiguous,
                }
            }
            Returns::Form => Tok::Form(bits as u32),
            Returns::String => {
                let id = f64_to_i32(result) as u32;
                if vars::string(e, id).is_none() {
                    self.error("Failed to resolve string return result");
                }
                Tok::Var {
                    ty: Ty::StringVar,
                    local: 0,
                    var_type: 2,
                    ref_idx: 0,
                    var_idx: 0,
                    string: id,
                }
            }
            Returns::Array => {
                let id = f64_to_i32(result) as u32;
                if id != 0 && !vars::array_exists(e, id) {
                    self.error("A command returned an invalid array");
                    return None;
                }
                Tok::Array(id)
            }
            _ => {
                self.error("Unknown command return type");
                return None;
            }
        };
        Some(tok)
    }

    // ---- token values (`ScriptToken::Get*`, `ArrayElementToken::Get*`) ----

    fn element(&self, e: &mut Engine, array: u32, key: &Key) -> Option<vars::Element> {
        vars::get_element(e, array, key)
    }

    /// `CanConvertTo`, with an array element's own type check.
    pub fn can_convert(&self, e: &mut Engine, t: &Tok, to: Ty) -> bool {
        if let Tok::Element { array, key } = t {
            if to == Ty::ArrayElement {
                return true;
            }
            let Some(el) = self.element(e, *array, key) else {
                return false;
            };
            let code = el.data.type_code();
            return match to {
                Ty::Boolean => code == 2 || code == 1,
                Ty::String => code == 3,
                Ty::Number => code == 1,
                Ty::Array => code == 4,
                Ty::Form => code == 2,
                _ => false,
            };
        }
        can_convert(t.ty(), to)
    }

    pub fn get_number(&self, e: &mut Engine, t: &Tok) -> f64 {
        match t {
            Tok::Number { v, .. } => *v,
            Tok::Bool(b) => f64::from(u8::from(*b)),
            Tok::Global(g) => f64::from(e.mem.f32(g + 0x24)),
            Tok::Var {
                ty: Ty::NumericVar | Ty::StringVar,
                local,
                ..
            } if *local != 0 => e.mem.f64(local + 8),
            Tok::Element { array, key } => match self.element(e, *array, key) {
                Some(vars::Element {
                    data: vars::Data::Number(n),
                    ..
                }) => n,
                _ => 0.0,
            },
            _ => 0.0,
        }
    }

    pub fn get_string(&self, e: &mut Engine, t: &Tok) -> Vec<u8> {
        match t {
            Tok::Str(s) => s.clone(),
            Tok::Var {
                ty: Ty::StringVar,
                local,
                string,
                ..
            } => {
                let id = if *local != 0 {
                    f64_to_i32(e.mem.f64(local + 8)) as u32
                } else {
                    *string
                };
                vars::string(e, id).unwrap_or_default()
            }
            Tok::Element { array, key } => match self.element(e, *array, key) {
                Some(vars::Element {
                    data: vars::Data::Str(s),
                    ..
                }) => s,
                _ => Vec::new(),
            },
            _ => Vec::new(),
        }
    }

    pub fn get_form_id(&self, e: &mut Engine, t: &Tok) -> u32 {
        match t {
            Tok::Form(id) => *id,
            Tok::Lambda(script) => e.mem.u32(script + 0xc),
            Tok::Var {
                ty: Ty::RefVar,
                local,
                ..
            } => {
                if *local == 0 {
                    0
                } else {
                    e.mem.u32(local + 8)
                }
            }
            Tok::Number { v, .. } => v.to_bits() as u32,
            Tok::Element { array, key } => match self.element(e, *array, key) {
                Some(vars::Element {
                    data: vars::Data::Form(f),
                    ..
                }) => f,
                _ => 0,
            },
            _ => 0,
        }
    }

    /// `GetTESForm`: the form's address, looked up by id.
    pub fn get_form(&self, e: &mut Engine, t: &Tok) -> u32 {
        match t {
            Tok::Form(_) | Tok::Var { ty: Ty::RefVar, .. } | Tok::Element { .. } => {
                let id = self.get_form_id(e, t);
                lookup_form(e, id)
            }
            Tok::Lambda(script) => *script,
            Tok::Number {
                form_or_number: true,
                v,
                ..
            } => lookup_form(e, v.to_bits() as u32),
            _ => 0,
        }
    }

    pub fn get_array_id(&self, e: &mut Engine, t: &Tok) -> u32 {
        match t {
            Tok::Array(id) => *id,
            Tok::Var {
                ty: Ty::ArrayVar,
                local,
                ..
            } => {
                if *local == 0 {
                    0
                } else {
                    f64_to_i32(e.mem.f64(local + 8)) as u32
                }
            }
            Tok::Element { array, key } => match self.element(e, *array, key) {
                Some(vars::Element {
                    data: vars::Data::Array(a),
                    ..
                }) if a == 0 || vars::array_exists(e, a) => a,
                _ => 0,
            },
            _ => 0,
        }
    }

    pub fn get_bool(&self, e: &mut Engine, t: &Tok) -> bool {
        match t {
            Tok::Bool(b) => *b,
            Tok::Number { v, .. } => *v != 0.0,
            Tok::Form(id) => *id != 0,
            Tok::Global(g) => e.mem.f32(g + 0x24) != 0.0,
            Tok::Array(id) => *id != 0,
            Tok::Var { local, .. } => *local != 0 && e.mem.f64(local + 8) != 0.0,
            // A string token is true when it has a text pointer: a literal with no characters
            // reads as none (`ReadString` gives null for length 0).
            Tok::Str(s) => !s.is_empty(),
            Tok::Element { array, key } => match self.element(e, *array, key) {
                Some(vars::Element {
                    data: vars::Data::Number(n),
                    ..
                }) => n != 0.0,
                Some(vars::Element {
                    data: vars::Data::Form(f),
                    ..
                }) => f != 0,
                _ => false,
            },
            _ => false,
        }
    }

    // ---- operators ----

    /// `Operator::Evaluate`: the first rule with a handler that both operands convert to (in
    /// either order for a symmetric rule).
    fn operate(&mut self, e: &mut Engine, o: u8, lh: Tok, rh: Option<Tok>) -> Option<Tok> {
        use Ty::*;
        type R = (Ty, Ty, bool);
        let rules: &[R] = match o {
            op::ASSIGNMENT => &[
                (AssignableString, String, true),
                (NumericVar, Number, true),
                (StringVar, String, true),
                (RefVar, Form, true),
                (RefVar, Number, true),
                (Global, Number, true),
                (ArrayVar, Array, true),
                (ArrayElement, Number, true),
                (ArrayElement, String, true),
                (ArrayElement, Form, true),
                (ArrayElement, Array, true),
            ],
            op::LOGICAL_OR | op::LOGICAL_AND => &[(Boolean, Boolean, false)],
            op::SLICE => &[(String, String, false), (Number, Number, false)],
            op::EQUALS | op::NOT_EQUAL => &[
                (Number, Number, false),
                (String, String, false),
                (Form, Form, false),
                (Form, Number, true),
                (Array, Array, false),
            ],
            op::GREATER_THAN | op::LESS_THAN | op::GREATER_OR_EQUAL | op::LESS_OR_EQUAL => {
                &[(Number, Number, false), (String, String, false)]
            }
            op::BITWISE_OR | op::BITWISE_AND | op::LEFT_SHIFT | op::RIGHT_SHIFT | op::MODULO => {
                &[(Number, Number, false)]
            }
            op::ADD => &[(Number, Number, false), (String, String, false)],
            op::SUBTRACT | op::DIVIDE | op::EXPONENT => &[(Number, Number, false)],
            op::MULTIPLY => &[(Number, Number, false), (String, Number, false)],
            op::NEGATION | op::BITWISE_NOT => &[(Number, Number, true)],
            op::LOGICAL_NOT => &[(Boolean, Boolean, true)],
            op::LEFT_BRACKET => &[
                (Array, Number, true),
                (Array, String, true),
                (ArrayElement, Number, true),
                (StringVar, Number, true),
                (ArrayElement, Slice, true),
                (StringVar, Slice, true),
                (String, Number, true),
                (Array, Slice, true),
                (String, Slice, true),
            ],
            op::MEMBER_ACCESS => &[(Array, String, true)],
            op::IN => &[
                (ArrayVar, Array, true),
                (StringVar, String, true),
                (RefVar, Form, true),
            ],
            op::TO_STRING => &[
                (String, String, true),
                (Number, Number, true),
                (Form, Form, true),
                (Array, Array, true),
            ],
            op::TO_NUMBER => &[(String, String, true), (Number, Number, true)],
            op::DEREFERENCE => &[(Array, Array, true)],
            op::BOX => &[
                (Number, Number, true),
                (String, String, true),
                (Form, Form, true),
                (Array, Array, true),
            ],
            op::MAKE_PAIR => &[
                (String, Number, true),
                (String, String, true),
                (String, Form, true),
                (String, Array, true),
                (Number, Number, true),
                (Number, String, true),
                (Number, Form, true),
                (Number, Array, true),
            ],
            op::PLUS_EQUALS => &[
                (NumericVar, Number, true),
                (ArrayElement, Number, true),
                (StringVar, String, true),
                (ArrayElement, String, true),
                (Global, Number, true),
            ],
            op::MINUS_EQUALS
            | op::TIMES_EQUALS
            | op::DIVIDED_EQUALS
            | op::EXPONENT_EQUALS
            | op::BITWISE_OR_EQUALS
            | op::BITWISE_AND_EQUALS
            | op::MODULO_EQUALS => &[
                (NumericVar, Number, true),
                (ArrayElement, Number, true),
                (Global, Number, true),
            ],
            _ => {
                self.error("Unhandled operator");
                return None;
            }
        };
        let unary = operand_count(o) == 1;
        for (i, &(l, r, asym)) in rules.iter().enumerate() {
            let (matches, swap) = if unary {
                (self.can_convert(e, &lh, l), false)
            } else {
                let rt = rh.as_ref()?;
                if self.can_convert(e, &lh, l) && self.can_convert(e, rt, r) {
                    (true, false)
                } else if !asym && self.can_convert(e, rt, l) && self.can_convert(e, &lh, r) {
                    (true, true)
                } else {
                    (false, false)
                }
            };
            if matches {
                let (a, b) = if swap {
                    (rh.clone().unwrap_or(Tok::OptionalEmpty), lh.clone())
                } else {
                    (lh.clone(), rh.clone().unwrap_or(Tok::OptionalEmpty))
                };
                return self.apply(e, o, i, a, b);
            }
        }
        if let Tok::Element { array, key } = &lh {
            if self.element(e, *array, key).is_none() {
                self.error("Array does not contain key");
                return None;
            }
        }
        self.error("Cannot evaluate operator on these operand types");
        None
    }

    /// The handler of rule `rule` of operator `o` (`Eval_*`, `ScriptUtils.cpp` (xNVSE)).
    fn apply(&mut self, e: &mut Engine, o: u8, rule: usize, lh: Tok, rh: Tok) -> Option<Tok> {
        let modi = script_mod(e, self.script);
        match o {
            op::ASSIGNMENT => self.assign(e, rule, lh, rh),
            // `Eval_Logical`: the right operand forwarded when true (`ForwardEvalResult`).
            op::LOGICAL_AND | op::LOGICAL_OR => {
                let (l, r) = (self.get_bool(e, &lh), self.get_bool(e, &rh));
                let v = if o == op::LOGICAL_AND { l && r } else { l || r };
                Some(if v {
                    self.forward(e, rh)
                } else {
                    Tok::Bool(false)
                })
            }
            op::SLICE => {
                let (lower, upper) = if rule == 0 {
                    (
                        Key::Str(self.get_string(e, &lh)),
                        Key::Str(self.get_string(e, &rh)),
                    )
                } else {
                    (
                        Key::Number(self.get_number(e, &lh)),
                        Key::Number(self.get_number(e, &rh)),
                    )
                };
                Some(Tok::Slice { lower, upper })
            }
            op::EQUALS | op::NOT_EQUAL => {
                let eq = match rule {
                    0 => {
                        let (l, r) = (self.get_number(e, &lh), self.get_number(e, &rh));
                        float_equal(l, r)
                    }
                    1 => {
                        let upper = vars::upper_table(e);
                        let (l, r) = (self.get_string(e, &lh), self.get_string(e, &rh));
                        vars::str_compare(&upper, &l, &r) == std::cmp::Ordering::Equal
                    }
                    2 => {
                        let (l, r) = (self.get_form(e, &lh), self.get_form(e, &rh));
                        (l == 0 && r == 0)
                            || (l != 0 && r != 0 && e.mem.u32(l + 0xc) == e.mem.u32(r + 0xc))
                    }
                    3 => self.get_number(e, &rh) == 0.0 && self.get_form_id(e, &lh) == 0,
                    _ => {
                        let (l, r) = (self.get_array_id(e, &lh), self.get_array_id(e, &rh));
                        match (vars::array(e, l).cloned(), vars::array(e, r).cloned()) {
                            (Some(a), Some(b)) => {
                                a.elements == b.elements && a.key_type == b.key_type
                            }
                            (None, None) => true,
                            _ => false,
                        }
                    }
                };
                Some(Tok::Bool(if o == op::EQUALS { eq } else { !eq }))
            }
            op::GREATER_THAN | op::LESS_THAN | op::GREATER_OR_EQUAL | op::LESS_OR_EQUAL => {
                if rule == 0 {
                    let (l, r) = (self.get_number(e, &lh), self.get_number(e, &rh));
                    Some(Tok::Bool(match o {
                        op::GREATER_THAN => l - r > f64::EPSILON,
                        op::LESS_THAN => r - l > f64::EPSILON,
                        op::GREATER_OR_EQUAL => l >= r,
                        _ => l <= r,
                    }))
                } else {
                    let upper = vars::upper_table(e);
                    let (l, r) = (self.get_string(e, &lh), self.get_string(e, &rh));
                    let c = vars::str_compare(&upper, &l, &r);
                    Some(Tok::Bool(match o {
                        op::GREATER_THAN => c.is_gt(),
                        op::LESS_THAN => c.is_lt(),
                        op::GREATER_OR_EQUAL => c.is_ge(),
                        _ => c.is_le(),
                    }))
                }
            }
            op::BITWISE_OR | op::BITWISE_AND | op::LEFT_SHIFT | op::RIGHT_SHIFT | op::MODULO => {
                // `Eval_Integer`: `SInt64` operands (`cvttsd2si` to 64 bits).
                let l = to_i64(self.get_number(e, &lh));
                let r = to_i64(self.get_number(e, &rh));
                let v = match o {
                    op::MODULO => {
                        if r == 0 {
                            self.error("Division by zero");
                            return None;
                        }
                        l.wrapping_rem(r)
                    }
                    op::BITWISE_OR => l | r,
                    op::BITWISE_AND => l & r,
                    op::LEFT_SHIFT => shift_left(l, r),
                    _ => shift_right(l, r),
                };
                Some(Tok::number(v as f64))
            }
            op::ADD => {
                if rule == 0 {
                    let v = self.get_number(e, &lh) + self.get_number(e, &rh);
                    Some(Tok::number(v))
                } else {
                    let mut s = self.get_string(e, &lh);
                    s.extend(self.get_string(e, &rh));
                    Some(Tok::Str(s))
                }
            }
            op::SUBTRACT | op::DIVIDE | op::EXPONENT | op::MULTIPLY
                if o != op::MULTIPLY || rule == 0 =>
            {
                let (l, r) = (self.get_number(e, &lh), self.get_number(e, &rh));
                let v = match o {
                    op::SUBTRACT => l - r,
                    op::MULTIPLY => l * r,
                    op::DIVIDE => {
                        if r == 0.0 {
                            self.error("Division by zero");
                            return None;
                        }
                        l / r
                    }
                    _ => l.powf(r),
                };
                Some(Tok::number(v))
            }
            op::MULTIPLY => {
                // `Eval_Multiply_String_Number`: the string repeated `int` times (`cvttsd2si`:
                // NaN and out of range give `INT_MIN`, so nothing). xNVSE grows the string until
                // memory runs out for a huge count; here a result past `MAX_REPEATED` bytes is a
                // script error instead (a fix: the original stops the game).
                let s = self.get_string(e, &lh);
                let n = f64_to_i32(self.get_number(e, &rh));
                if n > 0 && s.len().saturating_mul(n as usize) > MAX_REPEATED {
                    self.error("String too long");
                    return None;
                }
                Some(Tok::Str(s.repeat(n.max(0) as usize)))
            }
            op::NEGATION => Some(Tok::number(-self.get_number(e, &lh))),
            op::BITWISE_NOT => {
                let v = !(to_i64(self.get_number(e, &lh)) as u32);
                Some(Tok::number(f64::from(v)))
            }
            op::LOGICAL_NOT => Some(Tok::Bool(!self.get_bool(e, &lh))),
            op::LEFT_BRACKET | op::MEMBER_ACCESS => self.subscript(e, o, rule, lh, rh),
            op::TO_STRING => {
                let s = match rule {
                    0 => self.get_string(e, &lh),
                    1 => format_g(self.get_number(e, &lh)).into_bytes(),
                    2 => {
                        let form = self.get_form(e, &lh);
                        full_name(e, form)
                    }
                    _ => {
                        let id = self.get_array_id(e, &lh);
                        if vars::array_exists(e, id) {
                            format!("array ID {id}").into_bytes()
                        } else {
                            format!("array ID {id} (invalid)").into_bytes()
                        }
                    }
                };
                Some(Tok::Str(s))
            }
            op::TO_NUMBER => {
                let v = if rule == 1 {
                    self.get_number(e, &lh)
                } else {
                    numeric_representation(&self.get_string(e, &lh), false).0
                };
                Some(Tok::number(v))
            }
            op::DEREFERENCE => {
                let id = self.get_array_id(e, &lh);
                let Some(a) = vars::array(e, id).cloned() else {
                    self.error("Invalid array access - the array was not initialized.");
                    return None;
                };
                let upper = vars::upper_table(e);
                let value = Key::Str(b"value".to_vec());
                if a.len() == 2
                    && a.get(&upper, &Key::Str(b"key".to_vec())).is_some()
                    && a.get(&upper, &value).is_some()
                {
                    return Some(Tok::Element {
                        array: id,
                        key: value,
                    });
                }
                match a.keys().into_iter().next() {
                    Some(key) => Some(Tok::Element { array: id, key }),
                    None => {
                        self.error("Invalid array access - the array was not initialized.");
                        None
                    }
                }
            }
            op::BOX => {
                let arr = vars::create_array(e, vars::KEY_NUMBER, true, modi);
                let value = match rule {
                    0 => Value::Number(self.get_number(e, &lh)),
                    1 => Value::Str(self.get_string(e, &lh)),
                    2 => {
                        let form = self.get_form(e, &lh);
                        Value::Form(if form != 0 { e.mem.u32(form + 0xc) } else { 0 })
                    }
                    _ => Value::Array(self.get_array_id(e, &lh)),
                };
                vars::set_element(e, arr, &Key::Number(0.0), value);
                Some(Tok::Array(arr))
            }
            op::IN => match crate::commands::scripting::foreach_context(e, self, &lh, &rh) {
                Ok(t) => Some(t),
                Err(text) => {
                    self.error(text);
                    None
                }
            },
            op::MAKE_PAIR => Some(Tok::Pair(
                Box::new(self.forward(e, lh)),
                Box::new(self.forward(e, rh)),
            )),
            _ => self.compound_assign(e, o, rule, lh, rh),
        }
    }

    /// `ForwardEvalResult`: a variable's or element's value as a plain token.
    fn forward(&self, e: &mut Engine, t: Tok) -> Tok {
        match &t {
            Tok::Var {
                ty: Ty::NumericVar, ..
            } => Tok::number(self.get_number(e, &t)),
            Tok::Var {
                ty: Ty::StringVar, ..
            } => Tok::Str(self.get_string(e, &t)),
            Tok::Var { ty: Ty::RefVar, .. } => Tok::Form(self.get_form_id(e, &t)),
            Tok::Var {
                ty: Ty::ArrayVar, ..
            } => Tok::Array(self.get_array_id(e, &t)),
            Tok::Global(_) => Tok::number(self.get_number(e, &t)),
            _ => t,
        }
    }

    /// `[` and `->`: an element of an array, a character of a string.
    fn subscript(&mut self, e: &mut Engine, o: u8, rule: usize, lh: Tok, rh: Tok) -> Option<Tok> {
        if o == op::MEMBER_ACCESS || rule == 1 {
            // `Eval_Subscript_Array_String` / `Eval_MemberAccess`.
            let id = self.get_array_id(e, &lh);
            let Some(a) = vars::array(e, id) else {
                self.error("Invalid array access - the array was not initialized. 1");
                return None;
            };
            if a.key_type != vars::KEY_STRING {
                self.error("Invalid array access - expected numeric index, received string");
                return None;
            }
            let key = Key::Str(self.get_string(e, &rh));
            return Some(Tok::Element { array: id, key });
        }
        match rule {
            0 => {
                let id = self.get_array_id(e, &lh);
                let Some(a) = vars::array(e, id) else {
                    self.error("Invalid array access - the array was not initialized. 0");
                    return None;
                };
                if a.key_type != vars::KEY_NUMBER {
                    self.error("Invalid array access - expected string index, received numeric.");
                    return None;
                }
                let key = Key::Number(self.get_number(e, &rh));
                Some(Tok::Element { array: id, key })
            }
            6 => {
                // `Eval_Subscript_String`: one character, empty past the end.
                let s = self.get_string(e, &lh);
                let idx = f64_to_i32(self.get_number(e, &rh)) as u32 as usize;
                Some(Tok::Str(s.get(idx).map(|c| vec![*c]).unwrap_or_default()))
            }
            _ => {
                self.error("This subscript is not supported yet");
                None
            }
        }
    }

    /// `:=` (`Eval_Assign_*`).
    fn assign(&mut self, e: &mut Engine, rule: usize, lh: Tok, rh: Tok) -> Option<Tok> {
        let modi = script_mod(e, self.script);
        match (rule, &lh) {
            (
                1,
                Tok::Var {
                    local, var_type, ..
                },
            ) => {
                let mut v = self.get_number(e, &rh);
                if *var_type == 1 {
                    v = v.floor();
                }
                e.mem.set_f64(local + 8, v);
                Some(Tok::number(v))
            }
            (
                2,
                Tok::Var {
                    local: 0, string, ..
                },
            ) => {
                // A command's string result on the left (`Eval_Assign_String` with no local):
                // xNVSE sets that string when it exists and dereferences null when it does not;
                // here the latter is the script error xNVSE gives an unusable variable.
                let text = self.get_string(e, &rh);
                if vars::string(e, *string).is_none() {
                    self.error("Invalid variable");
                    return None;
                }
                vars::set_string(e, *string, &text);
                // Only the move from another command's result sets the owning mod (as for a
                // variable on the left, below).
                if matches!(
                    rh,
                    Tok::Var {
                        ty: Ty::StringVar,
                        local: 0,
                        ..
                    }
                ) {
                    set_owner(e, *string, modi);
                }
                Some(lh.clone())
            }
            (2, Tok::Var { local, ref_idx, .. }) => {
                let text = self.get_string(e, &rh);
                let current = f64_to_i32(e.mem.f64(local + 8)) as u32;
                if vars::string(e, current).is_some() {
                    vars::set_string(e, current, &text);
                    if matches!(
                        rh,
                        Tok::Var {
                            ty: Ty::StringVar,
                            local: 0,
                            ..
                        }
                    ) {
                        set_owner(e, current, modi);
                    }
                } else {
                    if *ref_idx == 0 {
                        vars::collect(e, self.event_list, *local, vars::VarType::String);
                    }
                    let id = vars::add_string(e, modi, &text, false);
                    e.mem.set_f64(local + 8, f64::from(id));
                }
                let id = f64_to_i32(e.mem.f64(local + 8)) as u32;
                Some(Tok::Var {
                    ty: Ty::StringVar,
                    local: *local,
                    var_type: 2,
                    ref_idx: *ref_idx,
                    var_idx: 0,
                    string: id,
                })
            }
            (3 | 4, Tok::Var { local, .. }) => {
                let id = self.get_form_id(e, &rh);
                if *local != 0 {
                    // The whole double: the id in the low word, 0 in the high one.
                    e.mem.set_u32(local + 8, id);
                    e.mem.set_u32(local + 12, 0);
                }
                Some(Tok::Form(id))
            }
            (5, Tok::Global(g)) => {
                let v = self.get_number(e, &rh);
                e.mem.set_f32(g + 0x24, v as f32);
                Some(Tok::number(v))
            }
            (6, Tok::Var { local, ref_idx, .. }) => {
                let to = self.get_array_id(e, &rh);
                vars::add_reference_double(e, local + 8, to, modi);
                if *ref_idx == 0 {
                    vars::collect(e, self.event_list, *local, vars::VarType::Array);
                }
                let id = f64_to_i32(e.mem.f64(local + 8)) as u32;
                Some(Tok::Array(id))
            }
            (7..=10, Tok::Element { array, key }) => {
                if !vars::array_exists(e, *array) {
                    self.error("Invalid array access - the array was not initialized.");
                    return None;
                }
                let (value, out) = match rule {
                    7 => {
                        let v = self.get_number(e, &rh);
                        (Value::Number(v), Tok::number(v))
                    }
                    8 => {
                        let s = self.get_string(e, &rh);
                        (Value::Str(s.clone()), Tok::Str(s))
                    }
                    9 => {
                        let f = self.get_form_id(e, &rh);
                        (Value::Form(f), Tok::Form(f))
                    }
                    _ => {
                        let a = self.get_array_id(e, &rh);
                        (Value::Array(a), Tok::Array(a))
                    }
                };
                if !vars::set_element(e, *array, key, value) {
                    self.error("Element with key not found");
                    return None;
                }
                Some(out)
            }
            _ => {
                self.error("This assignment is not supported yet");
                None
            }
        }
    }

    /// `+=`, `-=`, `*=`, `/=`, `^=`, `|=`, `&=`, `%=` (`Eval_*Equals*`).
    fn compound_assign(
        &mut self,
        e: &mut Engine,
        o: u8,
        rule: usize,
        lh: Tok,
        rh: Tok,
    ) -> Option<Tok> {
        // `+=` has string rules at 2 and 3; the numeric rules are 0 (variable), 1 (element) and
        // 4 (global) for `+=`, 0, 1, 2 for the others.
        let (var, elem, global) = if o == op::PLUS_EQUALS {
            (0, 1, 4)
        } else {
            (0, 1, 2)
        };
        if o == op::PLUS_EQUALS && (rule == 2 || rule == 3) {
            let add = self.get_string(e, &rh);
            if rule == 2 {
                // `Eval_PlusEquals_String`: a variable without a string gets a new one. A
                // command's string result has no variable (xNVSE dereferences null there): the
                // script error xNVSE gives an unusable variable instead.
                let Tok::Var { local, ref_idx, .. } = lh else {
                    return None;
                };
                if local == 0 {
                    self.error("Invalid variable");
                    return None;
                }
                let mut id = f64_to_i32(e.mem.f64(local + 8)) as u32;
                let mut s = match vars::string(e, id) {
                    Some(s) => s,
                    None => {
                        if ref_idx == 0 {
                            vars::collect(e, self.event_list, local, vars::VarType::String);
                        }
                        let modi = script_mod(e, self.script);
                        id = vars::add_string(e, modi, b"", false);
                        e.mem.set_f64(local + 8, f64::from(id));
                        Vec::new()
                    }
                };
                s.extend(add);
                vars::set_string(e, id, &s);
                return Some(Tok::Str(s));
            }
            let Tok::Element { array, key } = &lh else {
                return None;
            };
            let mut s = self.get_string(e, &lh);
            s.extend(add);
            vars::set_element(e, *array, key, Value::Str(s.clone()));
            return Some(Tok::Str(s));
        }
        let r = self.get_number(e, &rh);
        let l = self.get_number(e, &lh);
        let mut v = match o {
            op::PLUS_EQUALS => l + r,
            op::MINUS_EQUALS => l - r,
            op::TIMES_EQUALS => l * r,
            op::DIVIDED_EQUALS => {
                if r == 0.0 {
                    self.error("Division by zero");
                    return None;
                }
                l / r
            }
            op::EXPONENT_EQUALS => l.powf(r),
            op::BITWISE_OR_EQUALS => (to_i64(l) | to_i64(r)) as f64,
            op::BITWISE_AND_EQUALS => (to_i64(l) & to_i64(r)) as f64,
            _ => {
                if to_i64(r) == 0 {
                    self.error("Division by zero");
                    return None;
                }
                to_i64(l).wrapping_rem(to_i64(r)) as f64
            }
        };
        let handle_op = matches!(
            o,
            op::BITWISE_OR_EQUALS | op::BITWISE_AND_EQUALS | op::MODULO_EQUALS
        );
        match (&lh, rule) {
            (
                Tok::Var {
                    local, var_type, ..
                },
                r,
            ) if r == var => {
                if *var_type == 1 && !handle_op {
                    v = v.floor();
                }
                e.mem.set_f64(local + 8, v);
            }
            (Tok::Element { array, key }, r) if r == elem => {
                vars::set_element(e, *array, key, Value::Number(v));
            }
            (Tok::Global(g), r) if r == global => {
                // `TESGlobal::type` (`+0x20`): not float (`'f'`) truncates.
                if e.mem.u8(g + 0x20) != b'f' && !handle_op {
                    v = v.floor();
                }
                e.mem.set_f32(g + 0x24, v as f32);
                v = f64::from(e.mem.f32(g + 0x24));
            }
            _ => return None,
        }
        Some(Tok::number(v))
    }

    // Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), ScriptUtils.h:
    // ExpressionEvaluator::AssignAmbiguousResult.
    /// Writes a result as `eval`/`Let`-style commands return it, by the token's own type.
    pub fn assign_result(&mut self, e: &mut Engine, t: &Tok) {
        if self.can_convert(e, t, Ty::Number) {
            let v = self.get_number(e, t);
            e.mem.set_f64(self.result, v);
        } else if self.can_convert(e, t, Ty::String) {
            let s = self.get_string(e, t);
            let id = assign_string_result(e, self.script, &s);
            e.mem.set_f64(self.result, f64::from(id));
            expect_return_type(e, Returns::String);
        } else if self.can_convert(e, t, Ty::Form) {
            let id = self.get_form_id(e, t);
            e.mem.set_u32(self.result, id);
            expect_return_type(e, Returns::Form);
        } else if self.can_convert(e, t, Ty::Array) {
            let id = self.get_array_id(e, t);
            e.mem.set_f64(self.result, f64::from(id));
            expect_return_type(e, Returns::Array);
        } else {
            self.error("Function call returned unexpected return type");
        }
    }
}

/// A string result inside an expression: a temporary string (the per-thread function result
/// cache of `AssignToStringVarLong` is an optimisation of the same).
fn assign_string_result(e: &mut Engine, script: u32, text: &[u8]) -> u32 {
    let modi = vars::script_mod_index(e, script);
    vars::add_string(e, modi, text, true)
}

fn set_owner(e: &mut Engine, id: u32, modi: u8) {
    if let Some(s) = e.ext::<Nvse>().vars.strings.get_mut(&id) {
        s.owning_mod = modi;
    }
}

/// A double to `SInt64` as MSVC converts it (`cvttsd2si` to 64 bits): truncation,
/// `0x8000000000000000` for NaN and out of range.
fn to_i64(v: f64) -> i64 {
    if v.is_nan() || v >= 9_223_372_036_854_775_808.0 || v < -9_223_372_036_854_775_808.0 {
        i64::MIN
    } else {
        v as i64
    }
}

/// The longest string `*` makes (64 MiB; see `Eval_Multiply_String_Number`).
const MAX_REPEATED: usize = 64 << 20;

/// `SInt64 << SInt64` as 32-bit MSVC compiles it (`_allshl`): the count is the low byte of the
/// right operand (`cl`), and 64 or more gives 0, so a negative count (its low byte 0x80 and
/// up) gives 0 as well.
fn shift_left(l: i64, r: i64) -> i64 {
    let count = (r as u64 & 0xff) as u32;
    if count >= 64 {
        0
    } else {
        l << count
    }
}

/// `SInt64 >> SInt64` (`_allshr`): the low byte as the count; 64 or more fills with the sign.
fn shift_right(l: i64, r: i64) -> i64 {
    let count = (r as u64 & 0xff) as u32;
    if count >= 64 {
        l >> 63
    } else {
        l >> count
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), ScriptTokens.cpp:
// ScriptToken::GetNumericRepresentation (the string half).
/// A string's number: unless `from_hex`, the first token (split at space, tab, CR, LF) decides:
/// `0x` (any case) reads hex, `0b` binary from two bytes past the string's start, else
/// `strtod`. On raw bytes ([`crate::cnum`]): a script string need not be UTF-8. Also whether
/// xNVSE counts it an error (`hasErrorOut`).
pub fn numeric_representation(s: &[u8], from_hex: bool) -> (f64, bool) {
    let mut hex = from_hex;
    let mut bin = false;
    if !hex {
        let delim = |b: &u8| matches!(*b, b' ' | b'\t' | b'\r' | b'\n');
        if let Some(start) = s.iter().position(|b| !delim(b)) {
            let token = &s[start..];
            let len = token.iter().position(delim).unwrap_or(token.len());
            if len >= 2 {
                match token[..2].to_ascii_uppercase().as_slice() {
                    b"0X" => hex = true,
                    b"0B" => bin = true,
                    _ => {}
                }
            }
        }
    }
    if hex {
        let (v, ok) = crate::cnum::scan_hex(s);
        (f64::from(v), !ok)
    } else if bin {
        // `str += 2`: the string's first two bytes, whatever they are.
        let (v, read, range) = crate::cnum::strtol_binary(s.get(2..).unwrap_or(&[]));
        (f64::from(v), read == 0 || range)
    } else {
        let (v, read, range) = crate::cnum::strtod(s);
        (v, read == 0 || range)
    }
}

/// `GetFullName(form)`'s text (`"<no name>"` for none).
fn full_name(e: &mut Engine, form: u32) -> Vec<u8> {
    const TES_FULL_NAME: u32 = 0x0118_3158;
    let name = cast(e, form, RTTI_TES_FORM, TES_FULL_NAME);
    if name != 0 {
        let (data, len) = (e.mem.u32(name + 4), e.mem.u16(name + 8));
        if data != 0 && len != 0 {
            return crate::table::guest_text(e, data);
        }
    }
    b"<no name>".to_vec()
}

/// `EventListFromForm`: a quest's event list (`+0x5C`) or a reference's script's.
fn event_list_from_form(e: &mut Engine, form: u32) -> u32 {
    let kind = e.mem.u8(form + 4);
    if kind == 0x47 {
        return e.mem.u32(form + 0x5c);
    }
    if (0x3a..=0x40).contains(&kind) || kind == 0x69 {
        const EXTRA_GET_BY_TYPE: u32 = 0x0041_0220;
        const RTTI_BS_EXTRA_DATA: u32 = 0x0118_3b2c;
        const RTTI_EXTRA_SCRIPT: u32 = 0x0118_4514;
        let extra = e
            .call(EXTRA_GET_BY_TYPE, &args![form + 0x44, 0x0du32])
            .u32();
        let script = cast(e, extra, RTTI_BS_EXTRA_DATA, RTTI_EXTRA_SCRIPT);
        return if script == 0 {
            0
        } else {
            e.mem.u32(script + 0x10)
        };
    }
    0
}

/// MSVC's `double` to `UInt32` (`__dtoui3`): below 2^31 as `cvttsd2si`, above it less 2^32.
fn to_u32(v: f64) -> u32 {
    if v < 2_147_483_648.0 {
        f64_to_i32(v) as u32
    } else {
        f64_to_i32(v - 4_294_967_296.0) as u32
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), GameAPI.cpp: vExtractExpression, and
// ScriptUtils.cpp: ExpressionEvaluator::ConvertDefaultArg (with bConvertTESForms).
/// A parenthesised parameter (`0xFFFF`, then the expression) at `*at`: evaluated with the running
/// script context's calling reference and converted to the parameter's type; the position moves
/// past it. `None` when it fails or its value does not fit the parameter.
pub fn inline_argument(
    e: &mut Engine,
    kind: u32,
    at: &mut u32,
    data: u32,
    script: u32,
    event_list: u32,
) -> Option<crate::args::Arg> {
    use crate::args::Arg;
    let this_obj = crate::context::current(e).map_or(0, |c| c.owner_ref);
    let start = *at + 2;
    let (tok, ev, end) = e.with_stack(16, |e, frame| {
        let off = frame.addr();
        let result = off + 8;
        e.mem.set_u32(off, start.wrapping_sub(data));
        e.mem.set_f64(result, 0.0);
        let mut ev = Eval {
            params: 0,
            data,
            this_obj,
            containing_obj: 0,
            script,
            event_list,
            result,
            offset_ptr: off,
            at: start,
            errors: Vec::new(),
            suppress_errors: false,
            base_offset: Some(start.wrapping_sub(data).wrapping_sub(4)),
            inline: true,
        };
        let tok = with_evaluator(e, |e| ev.evaluate(e));
        let end = data.wrapping_add(e.mem.u32(off));
        (tok, ev, end)
    });
    report(e, &ev);
    *at = end;
    let tok = tok?;
    let modi = script_mod(e, script);
    // `handlePrimitiveStringVar`: a string variable gives its id, a string a new temporary id.
    let string_as_number = |e: &mut Engine, ev: &Eval, tok: &Tok| -> Option<f64> {
        if let Tok::Var {
            ty: Ty::StringVar,
            local,
            ..
        } = tok
        {
            if *local != 0 {
                return Some(e.mem.f64(local + 8));
            }
        }
        if ev.can_convert(e, tok, Ty::String) {
            let s = ev.get_string(e, tok);
            return Some(f64::from(vars::add_string(e, modi, &s, true)));
        }
        None
    };
    Some(match kind {
        0x01 => {
            if let Some(id) = string_as_number(e, &ev, &tok) {
                Arg::Int(to_u32(id) as i32)
            } else if ev.can_convert(e, &tok, Ty::Number) {
                Arg::Int(to_u32(ev.get_number(e, &tok)) as i32)
            } else {
                return None;
            }
        }
        0x17 | 0x1c | 0x29 | 0x20 | 0x33 | 0x34 | 0x37 => {
            if !ev.can_convert(e, &tok, Ty::Number) {
                return None;
            }
            Arg::Int(to_u32(ev.get_number(e, &tok)) as i32)
        }
        0x02 => {
            if let Some(id) = string_as_number(e, &ev, &tok) {
                Arg::Float(id as f32)
            } else if ev.can_convert(e, &tok, Ty::Number) {
                Arg::Float(ev.get_number(e, &tok) as f32)
            } else {
                return None;
            }
        }
        0x2c => {
            if let Some(id) = string_as_number(e, &ev, &tok) {
                Arg::Double(id)
            } else if ev.can_convert(e, &tok, Ty::Number) {
                Arg::Double(ev.get_number(e, &tok))
            } else {
                return None;
            }
        }
        0x2d => match tok {
            // xNVSE passes a command's string result on as a null `ScriptLocal *`, which the
            // command then dereferences; here the argument does not convert (the read fails).
            Tok::Var { local, .. } if local != 0 && ev.can_convert(e, &tok, Ty::Variable) => {
                Arg::Var(local)
            }
            _ => return None,
        },
        0x00 => {
            let s = ev.get_string(e, &tok);
            Arg::Str {
                raw: s.clone(),
                text: s,
            }
        }
        // Axis, actor value, animation group, sex, magic effect: the token's names for them
        // come later (a magic effect always fails in xNVSE too).
        0x05 | 0x08 | 0x0a | 0x12 | 0x1f => return None,
        _ => {
            let number_form = matches!(
                tok,
                Tok::Number {
                    form_or_number: true,
                    ..
                }
            );
            if !ev.can_convert(e, &tok, Ty::Form) && !number_form {
                return None;
            }
            let form = ev.get_form(e, &tok);
            if form == 0 {
                return None;
            }
            Arg::Form(convert_form(e, kind, form)?)
        }
    })
}

/// `ConvertDefaultArg`'s form checks: the form (or the cast pointer) a parameter of `kind` takes.
fn convert_form(e: &mut Engine, kind: u32, form: u32) -> Option<u32> {
    const RTTI_ACTOR: u32 = 0x0118_46d4;
    const RTTI_SPELL_ITEM: u32 = 0x0118_3060;
    const RTTI_CELL: u32 = 0x0118_3fb4;
    const RTTI_MAGIC_ITEM: u32 = 0x0118_3140;
    const RTTI_TES_OBJECT: u32 = 0x0118_3128;
    const RTTI_ACTOR_BASE: u32 = 0x0118_46e8;
    const RTTI_WORLD_SPACE: u32 = 0x0118_3fd0;
    const RTTI_PACKAGE: u32 = 0x0118_46a0;
    const RTTI_COMBAT_STYLE: u32 = 0x0118_627c;
    const RTTI_NPC: u32 = 0x0118_3a1c;
    const RTTI_LEV_CHARACTER: u32 = 0x0118_a280;
    const RTTI_CREATURE: u32 = 0x0118_3a00;
    const RTTI_LEV_CREATURE: u32 = 0x0118_9c10;
    const IS_INVENTORY_OBJECT_TYPE: u32 = 0x0048_1f30;
    let t = e.mem.u8(form + 4);
    let c = |e: &mut Engine, to: u32| cast(e, form, RTTI_TES_FORM, to);
    let some = |p: u32| (p != 0).then_some(p);
    let ok = |b: bool| b.then_some(form);
    match kind {
        0x03 => ok(e
            .call(IS_INVENTORY_OBJECT_TYPE, &args![u32::from(t)])
            .bool()),
        0x04 | 0x18 => {
            let refr = c(e, RTTI_TES_OBJECT_REFR);
            if refr == 0 {
                return None;
            }
            // `kParamType_MapMarker`: `IsMapMarker`, the base form's id is 0x10.
            if kind == 0x18 {
                let base = e.mem.u32(refr + 0x20);
                if base == 0 || e.mem.u32(base + 0xc) != 0x10 {
                    return None;
                }
            }
            Some(refr)
        }
        0x06 => some(c(e, RTTI_ACTOR)),
        0x07 => ok(c(e, RTTI_SPELL_ITEM) != 0 || t == 0x19),
        0x09 => some(c(e, RTTI_CELL)),
        0x0b => some(c(e, RTTI_MAGIC_ITEM)),
        0x15 => some(c(e, RTTI_TES_OBJECT)),
        0x19 => some(c(e, RTTI_ACTOR_BASE)),
        0x1a => {
            let refr = c(e, RTTI_TES_OBJECT_REFR);
            if refr == 0 {
                return None;
            }
            // `TESObjectREFR::GetContainer`: an actor's base, or a container base.
            let is_actor = e.vcall(refr, 0x10c, &args![]).bool();
            let base = e.mem.u32(refr + 0x20);
            (is_actor || (base != 0 && e.mem.u8(base + 4) == 0x1b)).then_some(refr)
        }
        0x1b => some(c(e, RTTI_WORLD_SPACE)),
        0x1d => some(c(e, RTTI_PACKAGE)),
        0x1e => some(c(e, RTTI_COMBAT_STYLE)),
        0x38 => ok(c(e, RTTI_NPC) != 0 || c(e, RTTI_LEV_CHARACTER) != 0),
        0x39 => ok(c(e, RTTI_CREATURE) != 0 || c(e, RTTI_LEV_CREATURE) != 0),
        0x23 | 0x3d => Some(form),
        0x32 => ok(e
            .call(IS_INVENTORY_OBJECT_TYPE, &args![u32::from(t)])
            .bool()
            || t == 0x55),
        0x35 => ok(e.vcall(form, 0xec, &args![]).bool() && t != 0x55),
        _ => {
            let want: u8 = match kind {
                0x0c => 0x0d,
                0x0d => 0x45,
                0x0e => 0x47,
                0x0f => 0x0c,
                0x10 => 0x07,
                0x11 => 0x08,
                0x13 => 0x06,
                0x14 => 0x27,
                0x21 => 0x35,
                0x22 => 0x2a,
                0x24 => 0x4f,
                0x25 => 0x55,
                0x26 => 0x05,
                0x27 => 0x56,
                0x28 => 0x31,
                0x2a => 0x54,
                0x2b => 0x53,
                0x2f => 0x61,
                0x30 => 0x48,
                0x31 => 0x62,
                0x36 => 0x66,
                0x3a => 0x2d,
                0x3b => 0x2c,
                0x3c => 0x34,
                0x3e => 0x68,
                0x3f => 0x6d,
                0x40 => 0x6c,
                0x41 => 0x71,
                0x42 => 0x74,
                0x43 => 0x73,
                0x44 => 0x75,
                0x45 => 0x37,
                _ => 0xff,
            };
            ok(t == want)
        }
    }
}

/// Runs `f` with an evaluator pushed on this thread's stack (`PushOnStack`/`PopFromStack`).
pub fn with_evaluator<T>(e: &mut Engine, f: impl FnOnce(&mut Engine) -> T) -> T {
    push_frame(e);
    let out = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(e)));
    pop_frame(e);
    match out {
        Ok(v) => v,
        Err(stop) => std::panic::resume_unwind(stop),
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), ScriptUtils.cpp:
// ExpressionEvaluator::~ExpressionEvaluator.
/// Reports an evaluator's errors as xNVSE's destructor does: one `ShowRuntimeError` with the
/// messages, the mod, the command's offset and its name.
pub fn report(e: &mut Engine, ev: &Eval) {
    if ev.errors.is_empty() || ev.suppress_errors || !Nvse::prints(e) {
        return;
    }
    let error = ev.errors.join("\n");
    let modname = String::from_utf8_lossy(&crate::errors::mod_name(e, ev.script)).into_owned();
    let base = ev.base_offset.unwrap_or(0);
    let command = match ev.base_offset.filter(|_| !ev.inline) {
        Some(b) => {
            let opcode = u32::from(e.mem.u16(ev.data.wrapping_add(b)));
            let record = record_of(e, opcode);
            if record != 0 {
                let name = crate::table::guest_text(e, e.mem.u32(record));
                String::from_utf8_lossy(&name).into_owned()
            } else {
                "<unknown>".to_string()
            }
        }
        None => "<unknown>".to_string(),
    };
    let text = format!("{error}\n    File: {modname} Offset: 0x{base:04X} Command: {command}");
    crate::errors::show(e, ev.script, &text);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_g_matches_the_c_runtime() {
        assert_eq!(format_g(1.0), "1");
        assert_eq!(format_g(0.5), "0.5");
        assert_eq!(format_g(123456.0), "123456");
        assert_eq!(format_g(1234567.0), "1.23457e+06");
        assert_eq!(format_g(0.0001), "0.0001");
        assert_eq!(format_g(0.00001), "1e-05");
        assert_eq!(format_g(-2.5), "-2.5");
        assert_eq!(format_g(1.0 / 3.0), "0.333333");
    }

    #[test]
    fn float_equality_is_in_float_with_an_epsilon() {
        assert!(float_equal(1.0, 1.00005));
        assert!(!float_equal(1.0, 1.0002));
    }

    /// Review item 1: `#` of a string reads raw bytes; a name with an umlaut (byte 0xFC in the
    /// game's code page, invalid UTF-8) gives 0 and an error, it does not stop the game.
    #[test]
    fn numbers_from_strings() {
        let n = |s: &[u8]| numeric_representation(s, false);
        assert_eq!(n(b"0x1F"), (31.0, false));
        assert_eq!(n(b"  0X1f"), (31.0, false));
        assert_eq!(n(b"0b101"), (5.0, false));
        assert_eq!(n(b"2.5abc"), (2.5, false));
        assert_eq!(n(b"x"), (0.0, true));
        assert_eq!(n(b"M\xfcller"), (0.0, true));
        assert_eq!(n(b"12 M\xfcller"), (12.0, false));
        assert_eq!(n("Jürgen 3".as_bytes()), (0.0, true));
        // `0b` skips the string's first two bytes, not the token's.
        assert_eq!(n(b" 0b11"), (0.0, true));
        assert_eq!(numeric_representation(b"ff", true), (255.0, false));
    }
}

#[cfg(test)]
mod review_tests {
    use super::*;

    /// Review item 8: shifts as `_allshl`/`_allshr` give them: the count's low byte, 64 and up
    /// 0 (left) or the sign (right), negative counts the same way.
    #[test]
    fn shifts_follow_the_crt_helpers() {
        assert_eq!(shift_left(1, 3), 8);
        assert_eq!(shift_left(1, 63), i64::MIN);
        assert_eq!(shift_left(1, 64), 0);
        assert_eq!(shift_left(1, 200), 0);
        assert_eq!(shift_left(1, -1), 0);
        assert_eq!(shift_left(1, 256 + 2), 4);
        assert_eq!(shift_right(-8, 1), -4);
        assert_eq!(shift_right(-8, 100), -1);
        assert_eq!(shift_right(8, 64), 0);
        assert_eq!(shift_right(8, -1), 0);
        assert_eq!(shift_right(-8, -1), -1);
    }
}

#[cfg(test)]
mod command_string_tests {
    use super::*;

    fn engine_and_eval() -> (Engine, Eval) {
        let mut e = Engine::new();
        e.map(0x0200_0000, 0x1_0000);
        let a = crate::CommandArgs {
            data: 0x0200_0000,
            script: 0x0200_1000,
            event_list: 0x0200_2000,
            result: 0x0200_0f08,
            opcode_offset: 0x0200_0f00,
            ..Default::default()
        };
        e.mem.set_u32(a.opcode_offset, 4);
        let ev = Eval::new(&e, &a);
        (e, ev)
    }

    fn command_string(id: u32) -> Tok {
        Tok::Var {
            ty: Ty::StringVar,
            local: 0,
            var_type: 2,
            ref_idx: 0,
            var_idx: 0,
            string: id,
        }
    }

    /// Review item 5: a command's string result (no local) on the left of `:=`, `+=` and `<-`
    /// is a script error (or, for `:=` on a live string, xNVSE's setting of that string), never
    /// a read of address 8.
    #[test]
    fn command_strings_without_a_local_are_script_errors() {
        let (mut e, mut ev) = engine_and_eval();
        let id = vars::add_string(&mut e, 5, b"old", true);
        let set = ev.operate(
            &mut e,
            op::ASSIGNMENT,
            command_string(id),
            Some(Tok::Str(b"new".to_vec())),
        );
        assert!(set.is_some());
        assert_eq!(vars::string(&e, id), Some(b"new".to_vec()));
        // A plain string on the right keeps the owning mod; another command's result moves
        // it to the script's (mod 0 here), as `Eval_Assign_String`'s move path does.
        let owner = |e: &mut Engine| e.ext::<Nvse>().vars.strings[&id].owning_mod;
        assert_eq!(owner(&mut e), 5);
        let other = vars::add_string(&mut e, 7, b"other", true);
        ev.operate(
            &mut e,
            op::ASSIGNMENT,
            command_string(id),
            Some(command_string(other)),
        );
        assert_eq!(vars::string(&e, id), Some(b"other".to_vec()));
        assert_eq!(owner(&mut e), 0);
        assert!(ev
            .operate(
                &mut e,
                op::ASSIGNMENT,
                command_string(999),
                Some(Tok::Str(b"x".to_vec()))
            )
            .is_none());
        assert!(ev.errors.iter().any(|m| m == "Invalid variable"));
        ev.errors.clear();
        assert!(ev
            .operate(
                &mut e,
                op::PLUS_EQUALS,
                command_string(id),
                Some(Tok::Str(b"x".to_vec()))
            )
            .is_none());
        assert!(ev.errors.iter().any(|m| m == "Invalid variable"));
        let foreach = crate::commands::scripting::foreach_context(
            &mut e,
            &ev,
            &command_string(id),
            &Tok::Str(b"abc".to_vec()),
        );
        assert!(foreach
            .unwrap_err()
            .starts_with("Unsupported variable type"));
    }
}
