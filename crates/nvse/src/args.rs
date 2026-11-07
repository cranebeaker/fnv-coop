//! Arguments and results (ADR-0016 §5; build step I-4).
//!
//! - [`extract_args_ex`]: xNVSE's argument reader, `vExtractArgsEx` and `v_ExtractArgsEx`
//!   (`GameAPI.cpp` (xNVSE)), the one xNVSE's own commands and JIP's read through. It is not
//!   the exe's `Script::ParseParameters` (`005accb0`): it drops the calling and containing
//!   references, refuses a parameter type above `kParamType_Region`, makes its own form type
//!   checks, and resolves a reference variable through `RefVariable::Resolve` (no cache).
//! - The wrap over `005accb0` (`OverrideWithExtractArgsEx`, `Hooks_Script.cpp` (xNVSE)): xNVSE
//!   jumps from the exe's reader to its own for every call; here that happens for an extender
//!   opcode (`0x1400` up) in every preset, and a vanilla opcode goes on to the next link (the
//!   exe's reader; xNVSE's once the planned fix row `xnvse.extract_args`, not built yet, is on):
//!   the per-opcode rule of ADR-0016 §5. Vanilla content calls no extender opcode, so under
//!   `vanilla` it reaches only the exe's reader.
//! - [`zero_extender_result`]: the result zeroed before an extender command's handler runs, as
//!   JIP's always-on patches do at `0x5ACB9D`, `0x5E16C3` and `0x5E2329` (`patches_game.h`
//!   (JIP LN NVSE)) and xNVSE's evaluator does (`cmdResult = 0`); for vanilla opcodes that is
//!   JIP's planned fix row (not built yet).
//!
//! Not here yet: an inline expression (a parameter starting `0xFFFF`) and the override path
//! (an argument count above `0x7FFF`) need xNVSE's expression evaluator (I-6), and a `$name`
//! string argument reads a string variable (I-5); until then the first two fail the read and
//! `$name` reads as the variable's text would when it holds no string.

use crate::context;
use crate::table::record_of;
use crate::{CommandArgs, Nvse};
use engine_core::extend::{Owner, WrapSlot};
use engine_core::prelude::*;

/// `Script::ParseParameters` (`005accb0`).
pub const PARSE_PARAMETERS: WrapSlot = WrapSlot::new(Owner::Nvse, 0x005a_ccb0);

/// The first extender opcode: from here up a change of xNVSE's or JIP's is part of the
/// extender in every preset (ADR-0016 §5).
pub const FIRST_EXTENDER_OPCODE: u32 = crate::names::FIRST_OPCODE;

/// `kParamType_Region` (xNVSE): the reader refuses any type above it.
const PARAM_TYPE_REGION: u32 = 0x45;

/// The exe's form lookup by id (`004839c0`: the form map at `0x011c54c0`, as xNVSE's
/// `LookupFormByID` reads it).
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `__RTDynamicCast` (`Fallout_DynamicCast` (xNVSE)) and the type descriptors it compares.
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const RTTI_TES_FORM: u32 = 0x0118_3028;
const RTTI_MAGIC_ITEM: u32 = 0x0118_3140;
const RTTI_BS_EXTRA_DATA: u32 = 0x0118_3b2c;
const RTTI_EXTRA_SCRIPT: u32 = 0x0118_4514;
/// `BaseExtraList::GetByType` (`0x410220`, `thiscall`: the list, the type).
const EXTRA_GET_BY_TYPE: u32 = 0x0041_0220;
/// `kExtraData_Script` (xNVSE).
const EXTRA_SCRIPT: u32 = 0x0d;

/// Form types (`kFormType_*` (xNVSE)), the byte at `TESForm +4`.
mod form_type {
    pub const BGS_MENU_ICON: u8 = 0x05;
    pub const TES_GLOBAL: u8 = 0x06;
    pub const TES_CLASS: u8 = 0x07;
    pub const TES_FACTION: u8 = 0x08;
    pub const TES_RACE: u8 = 0x0c;
    pub const TES_SOUND: u8 = 0x0d;
    pub const EFFECT_SETTING: u8 = 0x10;
    pub const SPELL_ITEM: u8 = 0x14;
    pub const TES_OBJECT_BOOK: u8 = 0x19;
    pub const TES_OBJECT_CONT: u8 = 0x1b;
    pub const TES_FURNITURE: u8 = 0x27;
    pub const TES_NPC: u8 = 0x2a;
    pub const TES_CREATURE: u8 = 0x2b;
    pub const TES_LEV_CREATURE: u8 = 0x2c;
    pub const TES_LEV_CHARACTER: u8 = 0x2d;
    pub const BGS_NOTE: u8 = 0x31;
    pub const TES_LEV_ITEM: u8 = 0x34;
    pub const TES_WEATHER: u8 = 0x35;
    pub const TES_REGION: u8 = 0x37;
    pub const TES_OBJECT_CELL: u8 = 0x39;
    pub const TES_OBJECT_REFR: u8 = 0x3a;
    pub const CHARACTER: u8 = 0x3b;
    pub const CREATURE: u8 = 0x3c;
    pub const FLAME_PROJECTILE: u8 = 0x40;
    pub const TES_WORLD_SPACE: u8 = 0x41;
    pub const TES_TOPIC: u8 = 0x45;
    pub const TES_QUEST: u8 = 0x47;
    pub const TES_IDLE_FORM: u8 = 0x48;
    pub const TES_PACKAGE: u8 = 0x49;
    pub const TES_COMBAT_STYLE: u8 = 0x4a;
    pub const TES_EFFECT_SHADER: u8 = 0x4f;
    pub const TES_IMAGE_SPACE: u8 = 0x53;
    pub const TES_IMAGE_SPACE_MODIFIER: u8 = 0x54;
    pub const BGS_LIST_FORM: u8 = 0x55;
    pub const BGS_PERK: u8 = 0x56;
    pub const BGS_ENCOUNTER_ZONE: u8 = 0x61;
    pub const BGS_MESSAGE: u8 = 0x62;
    pub const BGS_MUSIC_TYPE: u8 = 0x66;
    pub const TES_REPUTATION: u8 = 0x68;
    pub const CONTINUOUS_BEAM_PROJECTILE: u8 = 0x69;
    pub const TES_CASINO_CHIPS: u8 = 0x6c;
    pub const TES_CASINO: u8 = 0x6d;
    pub const TES_CHALLENGE: u8 = 0x71;
    pub const TES_CARAVAN_CARD: u8 = 0x73;
    pub const TES_CARAVAN_MONEY: u8 = 0x74;
    pub const TES_CARAVAN_DECK: u8 = 0x75;
}

/// `kInventoryType` (xNVSE): the form types that are inventory objects.
const INVENTORY_TYPE: [u8; 121] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 1, 1, 1,
    0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0,
];

fn is_inventory_type(kind: u8) -> bool {
    INVENTORY_TYPE.get(usize::from(kind)).copied().unwrap_or(0) != 0
}

/// How the reader takes each parameter type (`kClassifyParamExtract` (xNVSE)), by type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Extract {
    StringLiteral,
    Int,
    Short,
    Byte,
    Float,
    Double,
    Form,
    ScriptVariable,
    /// Not read (`8`): the read fails.
    None,
}

const CLASSIFY: [u8; 70] = [
    0, 1, 4, 6, 6, 2, 6, 6, 3, 6, 2, 6, 6, 6, 6, 6, 6, 6, 2, 6, 6, 6, 8, 1, 6, 6, 6, 6, 2, 6, 6, 6,
    3, 6, 6, 6, 6, 6, 6, 6, 6, 2, 6, 6, 5, 7, 8, 6, 6, 6, 6, 2, 2, 6, 6, 2, 6, 6, 6, 6, 6, 6, 6, 6,
    6, 6, 6, 6, 6, 6,
];

fn classify(kind: u32) -> Extract {
    match CLASSIFY.get(kind as usize).copied().unwrap_or(8) {
        0 => Extract::StringLiteral,
        1 => Extract::Int,
        2 => Extract::Short,
        3 => Extract::Byte,
        4 => Extract::Float,
        5 => Extract::Double,
        6 => Extract::Form,
        7 => Extract::ScriptVariable,
        _ => Extract::None,
    }
}

/// One argument as the reader stored it.
#[derive(Clone, Debug, PartialEq)]
pub enum Arg {
    /// A string literal: the bytes in the data (`raw`, written first with its NUL) and the text
    /// the command gets (`text`: `""` for `%e`, a string variable's text for `$name`).
    Str {
        raw: Vec<u8>,
        text: Vec<u8>,
    },
    /// `SInt32`.
    Int(i32),
    /// A 16-bit value stored in a `UInt32`.
    Short(u32),
    Byte(u8),
    /// A `float`.
    Float(f32),
    /// A `double`.
    Double(f64),
    /// A form's address.
    Form(u32),
    /// A `ScriptLocal`'s address.
    Var(u32),
}

impl Arg {
    /// The text of a string argument.
    pub fn text(&self) -> Option<&[u8]> {
        match self {
            Arg::Str { text, .. } => Some(text),
            _ => None,
        }
    }
    pub fn int(&self) -> Option<i32> {
        match self {
            Arg::Int(v) => Some(*v),
            _ => None,
        }
    }
    pub fn float(&self) -> Option<f32> {
        match self {
            Arg::Float(v) => Some(*v),
            _ => None,
        }
    }
    pub fn double(&self) -> Option<f64> {
        match self {
            Arg::Double(v) => Some(*v),
            _ => None,
        }
    }
    pub fn form(&self) -> Option<u32> {
        match self {
            Arg::Form(v) => Some(*v),
            _ => None,
        }
    }
}

/// What a read gave: the arguments stored before it ended (in xNVSE each is written to its
/// output as it is read, so a failed read leaves the earlier ones written), and whether it
/// succeeded.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Extracted {
    pub ok: bool,
    pub args: Vec<Arg>,
}

/// MSVC's SSE2 `double` to `int` conversion (`cvttsd2si`): truncation, and `0x80000000` for
/// NaN and out of range. xNVSE was built for SSE2 (`*out = data` for an integer parameter).
pub fn f64_to_i32(v: f64) -> i32 {
    if v.is_nan() || v >= 2_147_483_648.0 || v < -2_147_483_648.0 {
        i32::MIN
    } else {
        v as i32
    }
}

/// `ScriptEventList::GetVariable` (xNVSE): the first local of the list with id `id`, or 0.
/// The list (`+0xC`, `m_pLocalList`) is the game's singly linked list of `ScriptLocal`
/// (`id +0`, `data +8`); empty nodes are passed over.
pub fn event_list_variable(e: &Engine, event_list: u32, id: u32) -> u32 {
    if event_list == 0 {
        return 0;
    }
    let mut node = e.mem.u32(event_list + 0xc);
    while node != 0 {
        let local = e.mem.u32(node);
        if local != 0 && e.mem.u32(local) == id {
            return local;
        }
        node = e.mem.u32(node + 4);
    }
    0
}

/// `Script::GetRefFromRefList` (xNVSE): reference `index` (from 1) of the script's list of
/// referenced objects (`refList`, `Script +0x44`), each a `RefVariable` (`name +0`,
/// `form +8`, `varIdx +0xC`); 0 for none.
pub fn script_ref(e: &Engine, script: u32, index: u32) -> u32 {
    if index == 0 {
        return 0;
    }
    let mut node = script + 0x44;
    let mut at = 1;
    loop {
        if at == index {
            return e.mem.u32(node);
        }
        at += 1;
        node = e.mem.u32(node + 4);
        if node == 0 {
            return 0;
        }
    }
}

/// `RefVariable::Resolve` (xNVSE, `GameScript.cpp`): a reference variable (a nonzero `varIdx`)
/// takes the form whose id its local holds, looked up afresh.
pub fn resolve_ref_variable(e: &mut Engine, ref_var: u32, event_list: u32) {
    let var_idx = e.mem.u32(ref_var + 0xc);
    if var_idx != 0 && event_list != 0 {
        let local = event_list_variable(e, event_list, var_idx);
        if local != 0 {
            let id = e.mem.u32(local + 8);
            let form = e.call(LOOKUP_FORM, &args![id]).u32();
            e.mem.set_u32(ref_var + 8, form);
        }
    }
}

fn type_of(e: &Engine, form: u32) -> u8 {
    e.mem.u8(form + 4)
}

/// `TESObjectREFR::GetEventList` (xNVSE): the event list of the reference's `ExtraScript`.
fn reference_event_list(e: &mut Engine, reference: u32) -> u32 {
    let extra = e
        .call(EXTRA_GET_BY_TYPE, &args![reference + 0x44, EXTRA_SCRIPT])
        .u32();
    if extra == 0 {
        return 0;
    }
    let script = e
        .call(
            DYNAMIC_CAST,
            &args![extra, 0u32, RTTI_BS_EXTRA_DATA, RTTI_EXTRA_SCRIPT, 0u32],
        )
        .u32();
    if script == 0 {
        0
    } else {
        e.mem.u32(script + 0x10)
    }
}

/// `ExtractScriptVar` (xNVSE): the local named at `at` (`f`/`s` and a 16-bit id, after an `r`
/// prefix for another script's local: a quest's or a reference's). Answers the local and the
/// new position (past the `r` prefix; the caller steps past the local's three bytes).
fn extract_script_var(
    e: &mut Engine,
    mut at: u32,
    script: u32,
    mut event_list: u32,
) -> Option<(u32, u32)> {
    if e.mem.u8(at) == b'r' {
        let ref_var = script_ref(e, script, u32::from(e.mem.u16(at + 1)));
        if ref_var == 0 {
            return None;
        }
        resolve_ref_variable(e, ref_var, event_list);
        let form = e.mem.u32(ref_var + 8);
        if form == 0 {
            return None;
        }
        let kind = type_of(e, form);
        event_list = if kind == form_type::TES_QUEST {
            e.mem.u32(form + 0x5c)
        } else if (form_type::TES_OBJECT_REFR..=form_type::FLAME_PROJECTILE).contains(&kind)
            || kind == form_type::CONTINUOUS_BEAM_PROJECTILE
        {
            reference_event_list(e, form)
        } else {
            0
        };
        if event_list == 0 {
            return None;
        }
        at += 3;
    }
    match e.mem.u8(at) {
        b'f' | b's' => {
            let local = event_list_variable(e, event_list, u32::from(e.mem.u16(at + 1)));
            (local != 0).then_some((local, at))
        }
        _ => None,
    }
}

/// `ExtractFloat` (xNVSE): a number at `*at`: an immediate double (`z`) or integer (`n`), a
/// global (`G`), or a local; the position moves past it. `None` when it is not one (an inline
/// expression, `0xFFFF`, needs the evaluator: I-6).
fn extract_float(e: &mut Engine, at: &mut u32, script: u32, event_list: u32) -> Option<f64> {
    if e.mem.u16(*at) == 0xffff {
        return None;
    }
    match e.mem.u8(*at) {
        b'z' => {
            let v = e.mem.f64(*at + 1);
            *at += 9;
            Some(v)
        }
        b'n' => {
            let v = e.mem.i32(*at + 1);
            *at += 5;
            Some(f64::from(v))
        }
        b'G' => {
            let global = script_ref(e, script, u32::from(e.mem.u16(*at + 1)));
            if global == 0 {
                return None;
            }
            let form = e.mem.u32(global + 8);
            if form == 0 || type_of(e, form) != form_type::TES_GLOBAL {
                return None;
            }
            // `TESGlobal::data`, a float at +0x24.
            let v = e.mem.f32(form + 0x24);
            *at += 3;
            Some(f64::from(v))
        }
        _ => {
            let (local, var_at) = extract_script_var(e, *at, script, event_list)?;
            *at = var_at + 3;
            Some(e.mem.f64(local + 8))
        }
    }
}

/// xNVSE's form type check for a form parameter of `kind` (`v_ExtractArgsEx`); `None` refuses
/// the form, otherwise the form to store (a magic item is cast to `MagicItem`).
fn check_form(e: &mut Engine, kind: u32, form: u32) -> Option<u32> {
    use form_type::*;
    let t = type_of(e, form);
    let is = |x: u8| t == x;
    let bound = |e: &mut Engine| e.vcall(form, 0xe8, &args![]).bool();
    let ok = match kind {
        0x03 => is_inventory_type(t),
        0x04 => !(t < TES_OBJECT_REFR || (t > FLAME_PROJECTILE && t != CONTINUOUS_BEAM_PROJECTILE)),
        0x06 => is(CHARACTER) || is(CREATURE),
        0x07 => is(SPELL_ITEM) || is(TES_OBJECT_BOOK),
        0x09 => is(TES_OBJECT_CELL),
        0x0b => {
            let item = e
                .call(
                    DYNAMIC_CAST,
                    &args![form, 0u32, RTTI_TES_FORM, RTTI_MAGIC_ITEM, 0u32],
                )
                .u32();
            return (item != 0).then_some(item);
        }
        0x0c => is(TES_SOUND),
        0x0d => is(TES_TOPIC),
        0x0e => is(TES_QUEST),
        0x0f => is(TES_RACE),
        0x10 => is(TES_CLASS),
        0x11 => is(TES_FACTION),
        0x13 => is(TES_GLOBAL),
        0x14 => is(TES_FURNITURE) || is(BGS_LIST_FORM),
        0x15 => bound(e),
        0x18 => is(TES_OBJECT_REFR) && e.mem.u32(e.mem.u32(form + 0x20) + 0xc) == 0x10,
        0x19 => is(TES_NPC) || is(TES_CREATURE),
        0x1a => {
            is(CHARACTER)
                || is(CREATURE)
                || (is(TES_OBJECT_REFR) && type_of(e, e.mem.u32(form + 0x20)) == TES_OBJECT_CONT)
        }
        0x1b => is(TES_WORLD_SPACE),
        0x1d => is(TES_PACKAGE),
        0x1e => is(TES_COMBAT_STYLE),
        0x1f => is(EFFECT_SETTING),
        0x21 => is(TES_WEATHER),
        0x22 => is(TES_NPC),
        0x23 => is(TES_FACTION) || is(TES_NPC),
        0x24 => is(TES_EFFECT_SHADER),
        0x25 => is(BGS_LIST_FORM),
        0x26 => is(BGS_MENU_ICON),
        0x27 => is(BGS_PERK),
        0x28 => is(BGS_NOTE),
        0x2a => is(TES_IMAGE_SPACE_MODIFIER),
        0x2b => is(TES_IMAGE_SPACE),
        0x2f => is(BGS_ENCOUNTER_ZONE),
        0x30 => is(TES_IDLE_FORM),
        0x31 => is(BGS_MESSAGE),
        0x32 => is(BGS_LIST_FORM) || is_inventory_type(t),
        0x35 => !(is(BGS_LIST_FORM) || !bound(e)),
        0x36 => is(BGS_MUSIC_TYPE),
        0x38 => is(TES_NPC) || is(TES_LEV_CHARACTER),
        0x39 => is(TES_CREATURE) || is(TES_LEV_CREATURE),
        0x3a => is(TES_LEV_CHARACTER),
        0x3b => is(TES_LEV_CREATURE),
        0x3c => is(TES_LEV_ITEM),
        0x3e => is(TES_REPUTATION),
        0x3f => is(TES_CASINO),
        0x40 => is(TES_CASINO_CHIPS),
        0x41 => is(TES_CHALLENGE),
        0x42 => is(TES_CARAVAN_MONEY),
        0x43 => is(TES_CARAVAN_CARD),
        0x44 => is(TES_CARAVAN_DECK),
        0x45 => is(TES_REGION),
        _ => true,
    };
    ok.then_some(form)
}

/// A string variable's text, by id (`g_StringMap.Get`): string variables come with I-5; none
/// exists before, so every id answers none, as xNVSE does for an id it does not hold.
fn string_variable(_e: &Engine, _id: u32) -> Option<Vec<u8>> {
    None
}

/// `Script::GetVariableByName` (xNVSE): the variable list (`varList`, `Script +0x4C`; each
/// `VariableInfo` has its id at `+0` and its name, a `String`, at `+0x18`), compared without case
/// (`StrCompare`, upper-casing each byte).
fn script_variable_by_name(e: &Engine, script: u32, name: &[u8]) -> Option<u32> {
    let mut node = script + 0x4c;
    while node != 0 {
        let info = e.mem.u32(node);
        if info != 0 {
            let text = crate::table::guest_text(e, e.mem.u32(info + 0x18));
            if text.eq_ignore_ascii_case(name) {
                return Some(e.mem.u32(info));
            }
        }
        node = e.mem.u32(node + 4);
    }
    None
}

/// `v_ExtractArgsEx` (xNVSE): reads `count` arguments from `*at`.
fn read_args(
    e: &mut Engine,
    count: u32,
    params: u32,
    at: &mut u32,
    script: u32,
    event_list: u32,
) -> Extracted {
    let mut out = Extracted::default();
    for i in 0..count {
        let kind = e.mem.u32(params + 12 * i + 4);
        if kind > PARAM_TYPE_REGION {
            return out;
        }
        if e.mem.u16(*at) == 0xffff {
            // An inline expression (`vExtractExpression`): the evaluator, I-6.
            return out;
        }
        let arg = match classify(kind) {
            Extract::StringLiteral => {
                let length = u32::from(e.mem.u16(*at));
                *at += 2;
                let raw = e.mem.bytes(*at, length);
                let mut text = raw.clone();
                if length != 0 {
                    *at += length;
                    if length > 1 && raw[0] == b'$' {
                        if let Some(id) = script_variable_by_name(e, script, &raw[1..]) {
                            let local = event_list_variable(e, event_list, id);
                            if local != 0 {
                                let string = f64_to_i32(e.mem.f64(local + 8)) as u32;
                                if let Some(s) = string_variable(e, string) {
                                    text = s;
                                }
                            }
                        }
                    }
                    if text.len() == 2 && text[0] == b'%' && (text[1] | 0x20) == b'e' {
                        text.clear();
                    }
                }
                Arg::Str { raw, text }
            }
            Extract::Int => {
                if e.mem.u8(*at) == b'n' {
                    let v = e.mem.i32(*at + 1);
                    *at += 5;
                    Arg::Int(v)
                } else {
                    match extract_float(e, at, script, event_list) {
                        Some(v) => Arg::Int(f64_to_i32(v)),
                        None => return out,
                    }
                }
            }
            Extract::Short => {
                let v = u32::from(e.mem.u16(*at));
                *at += 2;
                Arg::Short(v)
            }
            Extract::Byte => {
                let v = e.mem.u8(*at);
                *at += 1;
                Arg::Byte(v)
            }
            Extract::Float => match extract_float(e, at, script, event_list) {
                Some(v) => Arg::Float(v as f32),
                None => return out,
            },
            Extract::Double => match extract_float(e, at, script, event_list) {
                Some(v) => Arg::Double(v),
                None => return out,
            },
            Extract::Form => {
                let mut form = 0;
                match e.mem.u8(*at) {
                    b'r' => {
                        let ref_var = script_ref(e, script, u32::from(e.mem.u16(*at + 1)));
                        if ref_var != 0 {
                            resolve_ref_variable(e, ref_var, event_list);
                            form = e.mem.u32(ref_var + 8);
                        }
                    }
                    b'f' => {
                        let local =
                            event_list_variable(e, event_list, u32::from(e.mem.u16(*at + 1)));
                        if local != 0 {
                            let id = e.mem.u32(local + 8);
                            form = e.call(LOOKUP_FORM, &args![id]).u32();
                        }
                    }
                    _ => {}
                }
                if form == 0 {
                    return out;
                }
                let Some(form) = check_form(e, kind, form) else {
                    return out;
                };
                *at += 3;
                Arg::Form(form)
            }
            Extract::ScriptVariable => {
                let Some((local, var_at)) = extract_script_var(e, *at, script, event_list) else {
                    return out;
                };
                *at = var_at + 3;
                Arg::Var(local)
            }
            Extract::None => return out,
        };
        out.args.push(arg);
    }
    out.ok = true;
    out
}

/// `vExtractArgsEx` (xNVSE): the arguments of the command whose argument offset is the word at
/// `offset_ptr` in `data`. With `increment` (the override of the exe's reader) the offset word
/// moves past them when the read succeeds; xNVSE's commands read with it off.
pub fn v_extract_args_ex(
    e: &mut Engine,
    params: u32,
    data: u32,
    offset_ptr: u32,
    script: u32,
    event_list: u32,
    increment: bool,
) -> Extracted {
    if params == 0 {
        return Extracted::default();
    }
    let offset = e.mem.u32(offset_ptr);
    let start = data.wrapping_add(offset);
    let count = u32::from(e.mem.u16(start));
    let mut at = start + 2;
    let opcode = u32::from(e.mem.u16(start.wrapping_sub(4)));
    let record = record_of(e, opcode);
    context::set_command(e, record);
    let read = if count > 0x7fff {
        // The override path (`ExtractArgsOverride`): commands compiled with xNVSE's expression
        // parser; it needs the evaluator (I-6). The offset word moves past the count first.
        e.mem.set_u32(offset_ptr, offset.wrapping_add(2));
        return Extracted::default();
    } else {
        read_args(e, count, params, &mut at, script, event_list)
    };
    if increment && read.ok {
        let now = e.mem.u32(offset_ptr);
        e.mem.set_u32(offset_ptr, now.wrapping_add(at - start));
    }
    read
}

/// `ExtractArgsEx` (xNVSE): the reader xNVSE's commands use (`ExtractArgs` is it in xNVSE's
/// build): the offset word does not move.
pub fn extract_args_ex(e: &mut Engine, a: &CommandArgs) -> Extracted {
    v_extract_args_ex(
        e,
        a.params,
        a.data,
        a.opcode_offset,
        a.script,
        a.event_list,
        false,
    )
}

/// Stores the arguments through the output pointers as xNVSE writes them (a string's bytes
/// with its NUL, then the text over them when it differs; an integer, a float, a double, a
/// form or a local's address in its own width).
fn store(e: &mut Engine, args: &[Arg], outputs: &[u32]) {
    for (arg, &to) in args.iter().zip(outputs) {
        match arg {
            Arg::Str { raw, text } => {
                e.mem.set_u8(to + raw.len() as u32, 0);
                for (i, b) in raw.iter().enumerate() {
                    e.mem.set_u8(to + i as u32, *b);
                }
                if text != raw {
                    for (i, b) in text.iter().enumerate() {
                        e.mem.set_u8(to + i as u32, *b);
                    }
                    e.mem.set_u8(to + text.len() as u32, 0);
                }
            }
            Arg::Int(v) => e.mem.set_i32(to, *v),
            Arg::Short(v) => e.mem.set_u32(to, *v),
            Arg::Byte(v) => e.mem.set_u8(to, *v),
            Arg::Float(v) => e.mem.set_f32(to, *v),
            Arg::Double(v) => e.mem.set_f64(to, *v),
            Arg::Form(v) | Arg::Var(v) => e.mem.set_u32(to, *v),
        }
    }
}

/// The opcode of the command whose arguments a call of `005accb0` reads: the word four bytes
/// before them, where xNVSE reads it, except for the parameters of a `Begin` line, which
/// `EvaluateLine` reads after the block type and its span (`position + 6`): that call reads for
/// the vanilla opcode `0x10`.
fn opcode_of_call(e: &Engine, data: u32, offset: u32) -> u32 {
    if let Some(line) = context::current_line(e) {
        if line.command == 0x10 && offset == line.position.wrapping_add(6) {
            return 0x10;
        }
    }
    u32::from(e.mem.u16(data.wrapping_add(offset).wrapping_sub(4)))
}

/// `Script::ParseParameters` (`005accb0`; `cdecl`: parameters, data, offset pointer, reference,
/// container, script, event list, then the outputs), wrapped: an extender opcode is read by
/// xNVSE's reader (`OverrideWithExtractArgsEx`, the offset moving past the arguments); a vanilla
/// opcode goes on to the next link.
fn parse_parameters(e: &mut Engine, w: &[u32]) -> Ret {
    let (params, data, offset_ptr) = (w[0], w[1], w[2]);
    let opcode = opcode_of_call(e, data, e.mem.u32(offset_ptr));
    if opcode < FIRST_EXTENDER_OPCODE {
        return e.call_next(PARSE_PARAMETERS.addr(), w);
    }
    let (script, event_list) = (w[5], w[6]);
    let read = v_extract_args_ex(e, params, data, offset_ptr, script, event_list, true);
    store(e, &read.args, w.get(7..).unwrap_or(&[]));
    crate::handled(read.ok)
}

/// The seams before a handler call (`005ACB9D`, `005E16C3`, `005E2329`; words: the opcode, the
/// record, the result's address): an extender command's result starts at 0.
pub fn zero_extender_result(e: &mut Engine, a: &[u32]) {
    if a[0] >= FIRST_EXTENDER_OPCODE && a[2] != 0 {
        e.mem.set_f64(a[2], 0.0);
    }
}

/// Installs the wrap over the exe's reader.
pub fn install(e: &mut Engine) {
    e.wrap(PARSE_PARAMETERS, parse_parameters);
    let _ = e.ext::<Nvse>();
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: u32 = 0x0200_0000;
    const OFFSET: u32 = 0x0200_1000;
    const PARAMS: u32 = 0x0200_2000;
    const SCRIPT: u32 = 0x0200_3000;
    const LIST: u32 = 0x0200_4000;
    const FORM: u32 = 0x0200_5000;
    const OUT: u32 = 0x0200_6000;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0200_0000, 0x8000);
        e.register(LOOKUP_FORM, |_, a| Ret {
            eax: if a[0] == 0x1234 { FORM } else { 0 },
            ..Ret::default()
        });
        e
    }

    /// Parameters of the given types at PARAMS.
    fn params(e: &mut Engine, kinds: &[(u32, bool)]) {
        for (i, (k, opt)) in kinds.iter().enumerate() {
            e.mem.set_u32(PARAMS + 12 * i as u32 + 4, *k);
            e.mem.set_u32(PARAMS + 12 * i as u32 + 8, u32::from(*opt));
        }
    }

    /// Compiled data: opcode, length, count, then `bytes`; the offset word points at the count.
    fn data(e: &mut Engine, opcode: u16, count: u16, bytes: &[u8]) {
        e.mem.set_u16(DATA, opcode);
        e.mem.set_u16(DATA + 2, bytes.len() as u16 + 2);
        e.mem.set_u16(DATA + 4, count);
        for (i, b) in bytes.iter().enumerate() {
            e.mem.set_u8(DATA + 6 + i as u32, *b);
        }
        e.mem.set_u32(OFFSET, 4);
    }

    fn read(e: &mut Engine) -> Extracted {
        v_extract_args_ex(e, PARAMS, DATA, OFFSET, SCRIPT, LIST, false)
    }

    /// Immediate numbers, a short, a byte and a string, each in its own width.
    #[test]
    fn numbers_strings_shorts_and_bytes() {
        let mut e = engine();
        // Integer, Float, ActorValue (short), Axis (byte), String, Double.
        params(
            &mut e,
            &[
                (1, false),
                (2, false),
                (5, false),
                (8, false),
                (0, false),
                (0x2c, false),
            ],
        );
        let mut bytes = vec![b'n'];
        bytes.extend_from_slice(&(-7i32).to_le_bytes());
        bytes.push(b'z');
        bytes.extend_from_slice(&2.5f64.to_le_bytes());
        bytes.extend_from_slice(&0x0123u16.to_le_bytes());
        bytes.push(b'X');
        bytes.extend_from_slice(&3u16.to_le_bytes());
        bytes.extend_from_slice(b"abc");
        bytes.push(b'n');
        bytes.extend_from_slice(&9i32.to_le_bytes());
        data(&mut e, 0x1404, 6, &bytes);
        let r = read(&mut e);
        assert!(r.ok);
        assert_eq!(
            r.args,
            vec![
                Arg::Int(-7),
                Arg::Float(2.5),
                Arg::Short(0x123),
                Arg::Byte(b'X'),
                Arg::Str {
                    raw: b"abc".to_vec(),
                    text: b"abc".to_vec()
                },
                Arg::Double(9.0),
            ]
        );
        // ExtractArgsEx leaves the offset where it was.
        assert_eq!(e.mem.u32(OFFSET), 4);
    }

    /// `%e` is the empty string; a double out of range for an integer is `0x80000000`.
    #[test]
    fn percent_e_and_the_sse2_conversion() {
        let mut e = engine();
        params(&mut e, &[(0, false), (1, false)]);
        let mut bytes = vec![2, 0, b'%', b'E', b'z'];
        bytes.extend_from_slice(&3e10f64.to_le_bytes());
        data(&mut e, 0x1404, 2, &bytes);
        let r = read(&mut e);
        assert!(r.ok);
        assert_eq!(r.args[0].text(), Some(&b""[..]));
        assert_eq!(r.args[1], Arg::Int(i32::MIN));
        assert_eq!(f64_to_i32(-1.9), -1);
        assert_eq!(f64_to_i32(f64::NAN), i32::MIN);
    }

    /// A local and a reference variable: the local by id in the event list, the reference
    /// variable resolved afresh from its local (no cache), and the form type checked.
    #[test]
    fn locals_and_reference_variables() {
        let mut e = engine();
        // Event list: one node holding local 2 (value 4.0) then local 3 (form id 0x1234).
        let (node1, node2, local2, local3) =
            (LIST + 0x100, LIST + 0x110, LIST + 0x200, LIST + 0x220);
        e.mem.set_u32(LIST + 0xc, node1);
        e.mem.set_u32(node1, local2);
        e.mem.set_u32(node1 + 4, node2);
        e.mem.set_u32(node2, local3);
        e.mem.set_u32(local2, 2);
        e.mem.set_f64(local2 + 8, 4.0);
        e.mem.set_u32(local3, 3);
        e.mem.set_u32(local3 + 8, 0x1234);
        // The script's reference list: reference 1 is a reference variable on local 3.
        let ref_var = SCRIPT + 0x400;
        e.mem.set_u32(SCRIPT + 0x44, ref_var);
        e.mem.set_u32(ref_var + 0xc, 3);
        // The form is a Character (0x3b).
        e.mem.set_u8(FORM + 4, 0x3b);
        // Float from a local, Actor from the reference variable, Actor from a form local.
        params(&mut e, &[(2, false), (6, false), (6, false)]);
        data(&mut e, 0x1404, 3, &[b'f', 2, 0, b'r', 1, 0, b'f', 3, 0]);
        let r = read(&mut e);
        assert!(r.ok, "{r:?}");
        assert_eq!(
            r.args,
            vec![Arg::Float(4.0), Arg::Form(FORM), Arg::Form(FORM)]
        );
        assert_eq!(e.mem.u32(ref_var + 8), FORM);
        // A Cell parameter refuses the Character: the read stops with what it had.
        params(&mut e, &[(2, false), (9, false)]);
        data(&mut e, 0x1404, 2, &[b'f', 2, 0, b'r', 1, 0]);
        let r = read(&mut e);
        assert!(!r.ok);
        assert_eq!(r.args, vec![Arg::Float(4.0)]);
    }

    /// Types above `kParamType_Region` and inline expressions fail the read.
    #[test]
    fn unknown_types_and_inline_expressions_fail() {
        let mut e = engine();
        params(&mut e, &[(0x46, false)]);
        data(&mut e, 0x1404, 1, &[b'n', 1, 0, 0, 0]);
        assert!(!read(&mut e).ok);
        params(&mut e, &[(1, false)]);
        data(&mut e, 0x1404, 1, &[0xff, 0xff, 0, 0]);
        assert!(!read(&mut e).ok);
    }

    /// The wrap over the exe's reader: an extender opcode is read by xNVSE's reader into the
    /// outputs and the offset moves past the arguments; a vanilla opcode reaches the exe's.
    #[test]
    fn the_exes_reader_is_wrapped_per_opcode() {
        fn exe_reader(e: &mut Engine, _: &[u32]) -> Ret {
            e.mem.set_u32(OUT + 0x100, 0xe8e);
            crate::handled(true)
        }
        let mut e = engine();
        e.register(PARSE_PARAMETERS.addr(), exe_reader);
        install(&mut e);
        params(&mut e, &[(1, false), (0, false)]);
        let mut bytes = vec![b'n'];
        bytes.extend_from_slice(&5i32.to_le_bytes());
        bytes.extend_from_slice(&[2, 0, b'h', b'i']);
        data(&mut e, 0x143e, 2, &bytes);
        let w = [PARAMS, DATA, OFFSET, 0, 0, SCRIPT, LIST, OUT, OUT + 0x10];
        assert_eq!(e.call(PARSE_PARAMETERS.addr(), &w).eax, 1);
        assert_eq!(e.mem.i32(OUT), 5);
        assert_eq!(e.mem.bytes(OUT + 0x10, 3), b"hi\0");
        assert_eq!(e.mem.u32(OFFSET), 4 + 2 + 9);
        assert_eq!(e.mem.u32(OUT + 0x100), 0);
        // A vanilla opcode: the exe's reader.
        data(&mut e, 0x1004, 2, &bytes);
        e.call(PARSE_PARAMETERS.addr(), &w);
        assert_eq!(e.mem.u32(OUT + 0x100), 0xe8e);
        assert_eq!(e.mem.u32(OFFSET), 4);
    }

    /// The result is zeroed for extender opcodes only.
    #[test]
    fn the_result_is_zeroed_for_extender_opcodes_only() {
        let mut e = engine();
        e.mem.set_f64(OUT, 3.0);
        zero_extender_result(&mut e, &[0x1004, 0, OUT]);
        assert_eq!(e.mem.f64(OUT), 3.0);
        zero_extender_result(&mut e, &[0x1400, 0, OUT]);
        assert_eq!(e.mem.f64(OUT), 0.0);
    }
}
