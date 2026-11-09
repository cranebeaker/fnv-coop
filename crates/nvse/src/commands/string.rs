//! xNVSE's string commands (`Commands_String.cpp`, `StringVar.cpp` (xNVSE)): the `sv_*` commands
//! on string variables, the character tests, the number-to-text commands, and the getters and
//! setters of forms' names and paths and of the game's settings, with xNVSE's format strings
//! (`ExtractFormatStringArgs`, `ScriptFormatStringArgs`, `ExtractFormattedString`, `GameAPI.cpp`
//! (xNVSE)).
//!
//! - **Results:** a string result goes through `AssignToStringVar` ([`assign_to_string_var`]):
//!   outside an expression the `set ... to` statement's variable keeps its string
//!   ([`set_statement_var`]); inside one it is a new temporary string, as [`crate::expr`] makes
//!   for xNVSE's per-thread function-result string.
//! - **Format strings:** xNVSE's own specifiers (`%z`, `%n`, `%i`, `%c`, `%k`, `%v`, `%p`, `%q`,
//!   `%r`, `%e`, `%a`, `%{ %}`, `%x`) are expanded first; the number specifiers left go to the
//!   UCRT's `snprintf` with the 20 numbers as its argument list ([`ucrt_format`], in
//!   [`super::printf`]), whose output is kept to what `snprintf`'s buffer holds (0x3FFD bytes);
//!   xNVSE keeps the result as a C string, so a NUL a conversion writes (`%c` of 0) ends it.
//! - **The fixes preset** (docs/FIXES.md, "The script extender"): `sv_Replace` of an empty text a
//!   given number of times stops at 64 MiB with the size caps row (without a count it never ends
//!   in xNVSE: capped in every preset), which also makes a format string the C runtime refuses or
//!   has no working memory for safely; a `%s` outside the game's memory ends the text with the
//!   missing array guard row ([`crate::commands::array`] keeps the rows). Where no cap comes
//!   first, a string these commands make past 1 GiB stops the game as out of memory
//!   ([`crate::commands::array::check_bytes`]), about where xNVSE's 32-bit process runs out.
//! - **Bytes:** strings are bytes; case changes and case-blind compares go through the exe's own
//!   `toupper`/`tolower`, as xNVSE calls them; the character classes are the C locale's.
//!
//! Not here: a format string compiled by xNVSE's own compiler (the override path, an argument
//! count above `0x7FFF`), which [`crate::args`] does not read either; the `set` statement's
//! expression is not parsed again (`SetToStatement`'s `ReadExpression`, the script analyser of
//! I-14): a compiled `set` line is taken as well formed.
//!
//! `ValidateRegex` checks a pattern as `std::regex`'s constructor does in xNVSE 6.4.9 (MSVC
//! 14.50's `_Parser` for the ECMAScript grammar, derived from the MSVC STL's `<regex>`,
//! Copyright (c) Microsoft Corporation, Apache-2.0 WITH LLVM-exception, modified; licence in
//! `crates/nvse/LICENSE-msvc-stl`): only the parse, which decides every error the constructor
//! raises ([`regex_check`]).

use super::printf::ucrt_format;
use super::{
    base_form, console_print, dynamic_cast, is_console_mode, rtti, try_get_refr_parent, Read,
};
use crate::args::{self, f64_to_i32, Arg};
use crate::expr::{self, Eval, Tok, Ty};
use crate::table::{guest_text, Command, Param, Returns};
use crate::vars;
use crate::{handled, CommandArgs};
use engine_core::prelude::*;
use std::cmp::Ordering;

/// `kMaxMessageLength`.
pub(crate) const MAX_MESSAGE_LENGTH: usize = 0x4000;

/// The exe's `toupper` and `tolower` (`game_toupper`, `game_tolower` (xNVSE)).
const GAME_TOUPPER: u32 = 0x00ec_a7f4;
const GAME_TOLOWER: u32 = 0x00ec_67aa;
/// `LookupFormByID`.
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `FormHeap_Allocate`, `FormHeap_Free` (cdecl).
const FORM_HEAP_ALLOCATE: u32 = 0x0040_1000;
const FORM_HEAP_FREE: u32 = 0x0040_1030;
/// `GetActorValueName` (cdecl: the code; the name).
const GET_ACTOR_VALUE_NAME: u32 = 0x0066_eac0;
/// `NiTMapBase::Lookup` (thiscall: the map, the key, the output).
const NI_TMAP_LOOKUP: u32 = 0x0085_3130;
/// `g_GameSettingCollection`, `g_IniSettingCollection`, `g_IniPrefCollection`.
const GAME_SETTINGS: u32 = 0x011c_8048;
const INI_SETTINGS: u32 = 0x011f_96a0;
const INI_PREFS: u32 = 0x011f_35a0;
/// `g_keyNames`, `g_mouseButtonNames`: `ControlName *` arrays (the name at `+4`).
const KEY_NAMES: u32 = 0x011d_52f0;
const MOUSE_BUTTON_NAMES: u32 = 0x011d_5240;
/// `g_actorValues`: `ActorValueInfo *` by code; `eActorVal_FalloutMax`.
const ACTOR_VALUES: u32 = 0x011d_61c8;
const ACTOR_VALUE_MAX: u32 = 76;

/// RTTI descriptors (`GameRTTI_1_4_0_525.inc` (xNVSE)).
const RTTI_TES_MODEL: u32 = 0x0118_31e8;
const RTTI_TES_ICON: u32 = 0x0118_3200;
const RTTI_TES_TEXTURE: u32 = 0x0118_3218;
const RTTI_TES_FULL_NAME: u32 = 0x0118_3158;
const RTTI_TES_FACTION: u32 = 0x0118_4704;
const RTTI_TES_GLOBAL: u32 = 0x0118_3ad4;
const RTTI_BS_EXTRA_DATA: u32 = 0x0118_3b2c;
const RTTI_EXTRA_SCRIPT: u32 = 0x0118_4514;
/// `BaseExtraList::GetByType` (thiscall) and `kExtraData_Script`.
const EXTRA_GET_BY_TYPE: u32 = 0x0041_0220;
const EXTRA_SCRIPT: u32 = 0x0d;

/// Form types.
const TYPE_FACTION: u8 = 0x08;
const TYPE_AMMO: u8 = 0x29;
const TYPE_NPC: u8 = 0x2a;
const TYPE_CELL: u8 = 0x39;
const TYPE_QUEST: u8 = 0x47;

// ---- parameters ----

const STRING: u32 = 0x00;
const INT: u32 = 0x01;
const FLOAT: u32 = 0x02;
const OBJECT_ID: u32 = 0x03;
const ACTOR_VALUE: u32 = 0x05;
const FACTION: u32 = 0x11;
const ANY_FORM: u32 = 0x3d;
/// `kNVSEParamType_*`.
const NV_NUMBER: u32 = 0x8001;
const NV_BOOLEAN: u32 = 0x8002;
const NV_STRING: u32 = 0x8004;
const NV_FORM: u32 = 0x8008;
const NV_STRING_VAR: u32 = 0x2000;

const fn p(name: &'static str, kind: u32, optional: bool) -> Param {
    Param {
        name,
        kind,
        optional,
    }
}

/// `FORMAT_STRING_PARAMS` (the format string and 20 optional numbers) and what follows them.
const fn fmt_params<const N: usize>(extra: &[Param]) -> [Param; N] {
    let mut out = [p("variable", FLOAT, true); N];
    out[0] = p("format string", STRING, false);
    let mut i = 0;
    while i < extra.len() {
        out[21 + i] = extra[i];
        i += 1;
    }
    out
}

const FORMAT_STRING: [Param; 21] = fmt_params(&[]);
const SV_SET: [Param; 22] = fmt_params(&[p("stringVar", INT, true)]);
const SV_COMPARE: [Param; 23] =
    fmt_params(&[p("stringVar", INT, false), p("bCaseSensitive", INT, true)]);
const SV_FIND: [Param; 25] = fmt_params(&[
    p("stringVar", INT, false),
    p("startPos", INT, true),
    p("endPos", INT, true),
    p("bCaseSensitive", INT, true),
]);
const SV_REPLACE: [Param; 26] = fmt_params(&[
    p("stringVar", INT, false),
    p("startPos", INT, true),
    p("endPos", INT, true),
    p("bCaseSensitive", INT, true),
    p("howMany", INT, true),
]);
const SET_PATH_EX: [Param; 22] = fmt_params(&[p("object", ANY_FORM, true)]);
const SET_BIPED_PATH_EX: [Param; 23] =
    fmt_params(&[p("whichPath", INT, false), p("item", ANY_FORM, true)]);
const SET_RANK_NAME_EX: [Param; 24] = fmt_params(&[
    p("faction", FACTION, false),
    p("rank", INT, false),
    p("bFemale", INT, true),
]);
const SV_DESTRUCT: [Param; 10] = [p("string_var", NV_STRING_VAR, true); 10];
const ONE_INT: &[Param] = &[p("int", INT, false)];
const TWO_INTS: &[Param] = &[p("int", INT, false), p("int", INT, false)];
const ONE_INT_ONE_OPT_INT: &[Param] = &[p("int", INT, false), p("int", INT, true)];
const ONE_INT_TWO_OPT_INTS: &[Param] = &[
    p("int", INT, false),
    p("int", INT, true),
    p("int", INT, true),
];
const SUBSTRING: &[Param] = &[
    p("stringVar", INT, false),
    p("startPos", INT, true),
    p("howMany", INT, true),
];
const SV_TO_NUMERIC: &[Param] = &[p("stringVar", INT, false), p("startPos", INT, true)];
const ONE_STRING: &[Param] = &[p("string", STRING, false)];
const TWO_STRINGS: &[Param] = &[p("string", STRING, false), p("string", STRING, false)];
const ONE_OPT_FORM: &[Param] = &[p("form", ANY_FORM, true)];
const ONE_INT_ONE_OPT_FORM: &[Param] = &[p("path type", INT, false), p("form", ANY_FORM, true)];
const ONE_ACTOR_VALUE: &[Param] = &[p("actor value", ACTOR_VALUE, false)];
const ONE_OPT_OBJECT_ID: &[Param] = &[p("item", OBJECT_ID, true)];
const ONE_STRING_ONE_OPT_OBJECT_ID: &[Param] =
    &[p("string", STRING, false), p("item", OBJECT_ID, true)];
const RANK_NAME: &[Param] = &[
    p("faction", FACTION, false),
    p("rank", INT, false),
    p("bFemale", INT, true),
];
const NV_ONE_STRING: &[Param] = &[p("string", NV_STRING, false)];
const NV_SPLIT: &[Param] = &[
    p("string", NV_STRING, false),
    p("string", NV_STRING, false),
    p("bool", NV_BOOLEAN, true),
];
const NV_TO_NUMBER: &[Param] = &[
    p("string", NV_STRING, false),
    p("bHex", NV_NUMBER, true),
    p("bReportError", NV_NUMBER, true),
];
const NV_ONE_FORM: &[Param] = &[p("object", NV_FORM, false)];
const NV_STRING_OPT_FORM: &[Param] = &[p("string", NV_STRING, false), p("object", NV_FORM, true)];
const SV_TRIM: &[Param] = &[p("stringVar", NV_STRING_VAR, false)];
const VALIDATE_REGEX: &[Param] = &[
    p("regexToValidate", NV_STRING, false),
    p("outErrStrVar", NV_STRING_VAR, true),
];

const fn cmd(
    opcode: u32,
    name: &'static str,
    short: &'static str,
    help: &'static str,
    params: &'static [Param],
    execute: AbiFn,
    returns: Returns,
) -> Command {
    Command {
        opcode,
        name,
        short,
        help,
        needs_ref: false,
        params,
        execute,
        eval: None,
        returns,
    }
}

use Returns::{Array as RA, Default as RD, String as RS};

pub static COMMANDS: &[Command] = &[
    cmd(0x156a, "sv_Destruct", "", "destroys one or more string variables", &SV_DESTRUCT, sv_destruct, RD),
    cmd(0x156b, "sv_Construct", "", "returns a formatted string", &FORMAT_STRING, sv_construct, RS),
    cmd(0x156c, "sv_Set", "", "sets the contents of a string variable", &SV_SET, sv_set, RD),
    cmd(0x156d, "sv_Compare", "", "compares two strings", &SV_COMPARE, sv_compare, RD),
    cmd(0x156e, "sv_Length", "", "returns the number of characters in a string", ONE_INT, sv_length, RD),
    cmd(0x156f, "sv_Erase", "", "erases a portion of a string variable", SUBSTRING, sv_erase, RD),
    cmd(0x1570, "sv_SubString", "", "returns a substring of a string variable", SUBSTRING, sv_substring, RS),
    cmd(0x1571, "sv_ToNumeric", "", "converts a string variable to a numeric type", SV_TO_NUMERIC, sv_to_numeric, RD),
    cmd(0x1572, "sv_Insert", "", "inserts a substring in a string variable", &SV_COMPARE, sv_insert, RD),
    cmd(0x1573, "sv_Count", "", "returns the number of occurences of a substring within a string variable", &SV_FIND, sv_count, RD),
    cmd(0x1574, "sv_Find", "", "returns the position of a substring within a string variable or -1 if not found", &SV_FIND, sv_find, RD),
    cmd(0x1575, "sv_Replace", "", "replaces 1 or more occurences of a substring within a string variable", &SV_REPLACE, sv_replace, RD),
    cmd(0x1576, "sv_GetChar", "", "returns the ASCII code of the character at the specified position in a string variable", TWO_INTS, sv_get_char, RD),
    cmd(0x1577, "sv_Split", "", "split a string into substrings returning an array", NV_SPLIT, sv_split, RA),
    cmd(0x1578, "sv_Percentify", "", "converts all percent signs in a string to double percent signs", NV_ONE_STRING, sv_percentify, RS),
    cmd(0x1579, "sv_ToUpper", "", "converts all characters in the string to uppercase", NV_ONE_STRING, sv_to_upper, RS),
    cmd(0x157a, "sv_ToLower", "", "converts all characters in the string to lowercase", NV_ONE_STRING, sv_to_lower, RS),
    cmd(0x157b, "IsLetter", "", "returns 1 if the character is an alphabetic character", ONE_INT, is_letter, RD),
    cmd(0x157c, "IsDigit", "", "returns 1 if the character is a numeric character", ONE_INT, is_digit, RD),
    cmd(0x157d, "IsPrintable", "", "returns 1 if the character is a printable character", ONE_INT, is_printable, RD),
    cmd(0x157e, "IsPunctuation", "", "returns 1 if the character is a punctuation character", ONE_INT, is_punctuation, RD),
    cmd(0x157f, "IsUpperCase", "", "returns 1 if the character is an uppercase letter", ONE_INT, is_upper_case, RD),
    cmd(0x1580, "CharToAscii", "", "converts a single character to its equivalent ASCII code", ONE_STRING, char_to_ascii, RD),
    cmd(0x1581, "ToUpper", "", "converts a character to uppercase", ONE_INT, to_upper, RD),
    cmd(0x1582, "ToLower", "", "converts a character to lowercase", ONE_INT, to_lower, RD),
    cmd(0x1583, "AsciiToChar", "", "returns a single character string given an ASCII code", ONE_INT, ascii_to_char, RS),
    cmd(0x1584, "NumToHex_OLD", "", "returns an int as a hex string of the specified width", ONE_INT_ONE_OPT_INT, num_to_hex_old, RS),
    cmd(0x1585, "ToNumber", "", "translates a string to a number", NV_TO_NUMBER, to_number, RD),
    cmd(0x1586, "GetNthModName", "", "returns the name of the nth active mod", ONE_INT, get_nth_mod_name, RS),
    cmd(0x1587, "GetName", "", "returns the name of an object", ONE_OPT_FORM, get_name, RS),
    cmd(0x1588, "GetKeyName", "", "returns the name of a key given a scan code", ONE_INT, get_key_name, RS),
    cmd(0x1589, "GetFormIDString", "", "returns a formID of a form as a hex string", ONE_OPT_FORM, get_form_id_string, RS),
    cmd(0x158a, "GetRawFormIDString", "GetFormIDString2", "returns the form ID stored in an array element or ref variable as a string regardless of whether or not the formID is valid", NV_ONE_FORM, get_raw_form_id_string, RS),
    cmd(0x158b, "GetFalloutDirectory", "GetFalloutDir", "returns the path to the Fallout directory", &[], get_fallout_directory, RS),
    cmd(0x158c, "ActorValueToString", "AVString", "returns the localized string corresponding to an actor value", ONE_ACTOR_VALUE, actor_value_to_string, RS),
    cmd(0x158d, "ActorValueToStringC", "AVStringC", "returns the localized string corresponding to an actor value code", ONE_INT_ONE_OPT_INT, actor_value_to_string_c, RS),
    cmd(0x158e, "GetModelPath", "", "returns the model path of an object", ONE_OPT_FORM, get_model_path, RS),
    cmd(0x158f, "GetIconPath", "", "returns the icon path of an object", ONE_OPT_FORM, get_icon_path, RS),
    cmd(0x1590, "GetBipedModelPath", "", "returns a model path", ONE_INT_ONE_OPT_FORM, get_biped_model_path, RS),
    cmd(0x1591, "GetBipedIconPath", "", "returns an icon path", ONE_INT_ONE_OPT_FORM, get_biped_icon_path, RS),
    cmd(0x1592, "GetTexturePath", "", "returns the texture path of an object. This command is identical to GetIconPath, but also works for other object types such as skills, classes, and miscellaneous objects.", ONE_OPT_FORM, get_texture_path, RS),
    cmd(0x1593, "SetModelPathEX", "", "sets a simple model path", &SET_PATH_EX, set_model_path_ex, RD),
    cmd(0x1594, "SetIconPathEX", "", "sets a simple icon path", &SET_PATH_EX, set_icon_path_ex, RD),
    cmd(0x1595, "SetBipedIconPathEX", "", "sets a biped icon path", &SET_BIPED_PATH_EX, set_biped_icon_path_ex, RD),
    cmd(0x1596, "SetBipedModelPathEX", "", "sets a biped model path", &SET_BIPED_PATH_EX, set_biped_model_path_ex, RD),
    cmd(0x1597, "SetTexturePath", "", "sets the texture path of an object. This command works for a broader set of objects than SetIconPathEX.", NV_STRING_OPT_FORM, set_texture_path, RD),
    cmd(0x1598, "GetNthFactionRankName", "", "returns the name of the faction rank", RANK_NAME, get_nth_faction_rank_name, RS),
    cmd(0x1599, "SetNthFactionRankNameEX", "", "sets the name of the nth faction rank", &SET_RANK_NAME_EX, set_nth_faction_rank_name_ex, RD),
    cmd(0x159a, "GetStringGameSetting", "", "returns the value of a string game setting", ONE_STRING, get_string_game_setting, RS),
    cmd(0x159b, "SetStringGameSettingEX_DEPRECATED", "", "sets a string game setting", &FORMAT_STRING, set_string_game_setting_ex_deprecated, RD),
    cmd(0x15b3, "GetScopeModelPath", "", "Get the path to the scope model of a weapon", ONE_OPT_OBJECT_ID, get_scope_model_path, RS),
    cmd(0x15b4, "SetScopeModelPath", "", "Set the path to the scope model of a weapon", ONE_STRING_ONE_OPT_OBJECT_ID, set_scope_model_path, RD),
    cmd(0x15c1, "GetStringIniSetting", "", "returns the value of a string ini setting", ONE_STRING, get_string_ini_setting, RS),
    cmd(0x15c2, "SetStringIniSetting_DEPRECATED", "", "sets a string ini setting", &FORMAT_STRING, set_string_ini_setting_deprecated, RD),
    cmd(0x1640, "sv_Trim", "", "Trims the beginning and the end of a string variable", SV_TRIM, sv_trim, RD),
    cmd(0x1654, "ValidateRegex", "", "Returns 0 for no errors, otherwise returns int codes indicating the error cause.", VALIDATE_REGEX, validate_regex, RD),
    cmd(0x1655, "IntToBin", "", "returns an int as a binary string of the specified width", ONE_INT_TWO_OPT_INTS, int_to_bin, RS),
    cmd(0x1656, "NumToHex", "IntToHex", "returns an int as a hex string of the specified width", ONE_INT_TWO_OPT_INTS, num_to_hex, RS),
    cmd(0x165d, "SetStringIniSetting", "", "sets a string ini setting", TWO_STRINGS, set_string_ini_setting, RD),
    cmd(0x166d, "SetModelPath", "", "sets the model path of an object", NV_STRING_OPT_FORM, set_model_path, RD),
];

// ---- shared helpers ----

/// MSVC's `double` to `UInt32` (`__dtoui3`): below 2^31 as `cvttsd2si`, above it less 2^32.
pub(crate) fn dtoui3(v: f64) -> u32 {
    if v < 2_147_483_648.0 {
        f64_to_i32(v) as u32
    } else {
        f64_to_i32(v - 4_294_967_296.0) as u32
    }
}

/// `game_toupper(c)` / `game_tolower(c)` of the exe, with `c` as xNVSE passes it.
fn game_case(e: &mut Engine, upper: bool, c: u32) -> u32 {
    e.call(if upper { GAME_TOUPPER } else { GAME_TOLOWER }, &args![c])
        .u32()
}

/// `std::transform(s, game_toupper/game_tolower)`: each `char` (sign-extended) through the exe.
fn change_case(e: &mut Engine, s: &[u8], upper: bool) -> Vec<u8> {
    s.iter()
        .map(|&b| game_case(e, upper, b as i8 as i32 as u32) as u8)
        .collect()
}

fn text_at(e: &Engine, at: u32) -> Vec<u8> {
    guest_text(e, at)
}

/// A `String` (`m_data` `+0`, `m_dataLen` `+4`, `m_bufLen` `+6`): its text, `None` for a null
/// `m_data`.
fn bs_string(e: &Engine, at: u32) -> Option<Vec<u8>> {
    let data = e.mem.u32(at);
    (data != 0).then(|| guest_text(e, data))
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), GameTypes.cpp: String::Set.
fn bs_string_set(e: &mut Engine, at: u32, text: &[u8]) {
    let len = text.len() as u32;
    if len > u32::from(e.mem.u16(at + 6)) {
        let old = e.mem.u32(at);
        e.call(FORM_HEAP_FREE, &args![old]);
        let data = e.call(FORM_HEAP_ALLOCATE, &args![len + 1]).u32();
        e.mem.set_u32(at, data);
        e.mem
            .set_u16(at + 6, if data != 0 { len as u16 } else { 0 });
    }
    let data = e.mem.u32(at);
    if data != 0 {
        e.mem.write(data, text);
        e.mem.set_u8(data + len, 0);
        e.mem.set_u16(at + 4, len as u16);
    } else {
        e.mem.set_u16(at + 4, 0);
    }
}

fn mod_index(e: &Engine, form: u32) -> u8 {
    vars::script_mod_index(e, form)
}

/// `TESForm::GetFullName` (xNVSE `GameForms.cpp`): a cell without a name takes its worldspace's;
/// a reference (`GetIsReference`, here the cast to `TESObjectREFR`) its base form's.
pub(crate) fn tes_full_name(e: &mut Engine, form: u32) -> u32 {
    if e.mem.u8(form + 4) == TYPE_CELL {
        let full = form + 0x18;
        let world = e.mem.u32(form + 0xc0);
        if (e.mem.u32(full + 4) == 0 || e.mem.u16(full + 8) == 0) && world != 0 {
            return world + 0x18;
        }
        return full;
    }
    let is_ref = dynamic_cast(e, form, rtti::TES_FORM, rtti::TES_OBJECT_REFR) != 0;
    let base = if is_ref { base_form(e, form) } else { form };
    dynamic_cast(e, base, rtti::TES_FORM, RTTI_TES_FULL_NAME)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), GameForms.cpp: GetFullName.
/// `"<no name>"` for a form without one, `"<NULL>"` for no form.
pub(crate) fn full_name(e: &mut Engine, form: u32) -> Vec<u8> {
    if form == 0 {
        return b"<NULL>".to_vec();
    }
    let name = tes_full_name(e, form);
    if name != 0 && e.mem.u32(name + 4) != 0 && e.mem.u16(name + 8) != 0 {
        return guest_text(e, e.mem.u32(name + 4));
    }
    b"<no name>".to_vec()
}

/// `TESForm::GetTheName`: the full name's text, `""` for a form without the component.
pub(crate) fn the_name(e: &mut Engine, form: u32) -> Vec<u8> {
    let name = tes_full_name(e, form);
    if name == 0 {
        Vec::new()
    } else {
        bs_string(e, name + 4).unwrap_or_default()
    }
}

/// `TESForm::GetStringRepresentation`: `[id: %X, edid: "%s", name: "%s"]` (the editor id from
/// `GetName`, the `+0x130` virtual function).
pub(crate) fn form_representation(e: &mut Engine, form: u32) -> Vec<u8> {
    let edid_at = e.vcall(form, 0x130, &args![]).u32();
    let edid = text_at(e, edid_at);
    let name = tes_full_name(e, form);
    let name = if name == 0 {
        b"<no name>".to_vec()
    } else {
        bs_string(e, name + 4).unwrap_or_default()
    };
    let mut out = format!("[id: {:X}, edid: \"", e.mem.u32(form + 0xc)).into_bytes();
    out.extend(edid);
    out.extend_from_slice(b"\", name: \"");
    out.extend(name);
    out.extend_from_slice(b"\"]");
    out
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Utilities.cpp: GetDXDescription.
fn dx_description(e: &Engine, keycode: u32) -> Vec<u8> {
    let control = |e: &Engine, at: u32| -> Option<Vec<u8>> {
        let c = e.mem.u32(at);
        (c != 0).then(|| text_at(e, e.mem.u32(c + 4)))
    };
    if keycode <= 220 {
        control(e, KEY_NAMES + 4 * keycode)
    } else if (255..=263).contains(&keycode) {
        let k = if keycode == 255 { 256 } else { keycode };
        control(e, MOUSE_BUTTON_NAMES + 4 * (k - 256))
    } else if keycode == 264 {
        Some(b"WheelUp".to_vec())
    } else if keycode == 265 {
        Some(b"WheelDown".to_vec())
    } else {
        None
    }
    .unwrap_or_else(|| b"<no key>".to_vec())
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), GameAPI.cpp: GetActorValueString.
fn actor_value_string(e: &mut Engine, code: u32) -> Vec<u8> {
    let mut name = 0;
    if code <= ACTOR_VALUE_MAX {
        name = e.call(GET_ACTOR_VALUE_NAME, &args![code]).u32();
    }
    if name == 0 {
        b"unknown".to_vec()
    } else {
        text_at(e, name)
    }
}

/// `TESFaction::ranks.GetNthItem(n)` (`tList` at `+0x3C`): the rank, 0 for none.
fn nth_rank(e: &Engine, faction: u32, which: u32) -> u32 {
    if (which as i32) < 0 {
        return 0;
    }
    let mut node = faction + 0x3c;
    let mut index = which;
    loop {
        if index == 0 {
            return e.mem.u32(node);
        }
        index -= 1;
        node = e.mem.u32(node + 4);
        if node == 0 {
            return 0;
        }
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), GameForms.cpp: TESFaction::GetNthRankName.
/// `None` for no such rank; a rank's name with no text reads `""` (`CStr`).
fn nth_rank_name(e: &Engine, faction: u32, which: u32, female: bool) -> Option<Vec<u8>> {
    let rank = nth_rank(e, faction, which);
    (rank != 0).then(|| bs_string(e, rank + if female { 8 } else { 0 }).unwrap_or_default())
}

// ---- settings (`GameSettings.cpp` (xNVSE)) ----

/// `Setting::GetType`: by the name's first letter; 6 is a string.
fn setting_type(e: &Engine, setting: u32) -> u32 {
    let name = e.mem.u32(setting + 8);
    if name == 0 {
        return 9;
    }
    match e.mem.u8(name) {
        b'b' => 0,
        b'c' => 1,
        b'i' => 3,
        b'u' => 4,
        b'f' => 5,
        b's' | b'S' => 6,
        b'r' => 7,
        b'a' => 8,
        _ => 9,
    }
}

/// `Setting::Get()`: a string setting's text pointer, else 0.
fn setting_str(e: &Engine, setting: u32) -> u32 {
    if setting_type(e, setting) == 6 {
        e.mem.u32(setting + 4)
    } else {
        0
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), GameSettings.cpp: Setting::Set(const char*).
fn setting_set_str(e: &mut Engine, setting: u32, text: &[u8]) -> bool {
    if setting_type(e, setting) != 6 {
        return false;
    }
    let at = e
        .call(FORM_HEAP_ALLOCATE, &args![text.len() as u32 + 1])
        .u32();
    if at == 0 {
        return false;
    }
    e.mem.write(at, text);
    e.mem.set_u8(at + text.len() as u32, 0);
    e.mem.set_u32(setting + 4, at);
    true
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), GameSettings.cpp:
// GameSettingCollection::GetGameSetting.
fn game_setting(e: &mut Engine, name: &[u8]) -> u32 {
    let collection = e.mem.u32(GAME_SETTINGS);
    if collection == 0 {
        return 0;
    }
    crate::events::with_guest_text(e, name, |e, at| {
        e.with_stack(4, |e, out| {
            e.mem.set_u32(out.addr(), 0);
            let found = e
                .call(NI_TMAP_LOOKUP, &args![collection + 0x10c, at, out.addr()])
                .u32()
                & 0xff;
            if found != 0 {
                e.mem.u32(out.addr())
            } else {
                0
            }
        })
    })
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), GameSettings.cpp: GetIniSetting.
/// The preferences' settings first, then the INI's (each list at `+0x10C`), by name without
/// case.
fn ini_setting(e: &mut Engine, name: &[u8]) -> u32 {
    let upper = vars::upper_table(e);
    for collection in [INI_PREFS, INI_SETTINGS] {
        let coll = e.mem.u32(collection);
        if coll == 0 {
            continue;
        }
        let mut node = coll + 0x10c;
        while node != 0 {
            let setting = e.mem.u32(node);
            if setting != 0 {
                let n = e.mem.u32(setting + 8);
                if n != 0 && vars::str_compare(&upper, name, &text_at(e, n)) == Ordering::Equal {
                    return setting;
                }
            }
            node = e.mem.u32(node + 4);
        }
    }
    0
}

/// `GetSeparatorChar`: `|`, or `@` for a console line of no mod.
fn separator(e: &mut Engine, script: u32) -> u8 {
    if is_console_mode(e) && !(script != 0 && mod_index(e, script) != 0xff) {
        b'@'
    } else {
        b'|'
    }
}

fn print_text(e: &mut Engine, format: &str, text: &[u8]) {
    crate::events::with_guest_text(e, text, |e, at| console_print(e, format, &[at]));
}

// ---- results ----

/// `TESQuest`'s script (`scriptable.script`, `+0x1C`) or a reference's (`ExtraScript +0xC`):
/// `Script::RefVariable::GetReferencedScript`.
fn referenced_script(e: &mut Engine, form: u32) -> u32 {
    if e.mem.u8(form + 4) == TYPE_QUEST {
        return e.mem.u32(form + 0x1c);
    }
    let refr = dynamic_cast(e, form, rtti::TES_FORM, rtti::TES_OBJECT_REFR);
    if refr == 0 {
        return 0;
    }
    let extra = e
        .call(EXTRA_GET_BY_TYPE, &args![refr + 0x44, EXTRA_SCRIPT])
        .u32();
    let script = dynamic_cast(e, extra, RTTI_BS_EXTRA_DATA, RTTI_EXTRA_SCRIPT);
    if script == 0 {
        0
    } else {
        e.mem.u32(script + 0xc)
    }
}

/// `Script::GetVariableInfo(idx)`: the variable list (`+0x4C`) entry with that id.
fn variable_info(e: &Engine, script: u32, idx: u32) -> u32 {
    let mut node = script + 0x4c;
    while node != 0 {
        let info = e.mem.u32(node);
        if info != 0 && e.mem.u32(info) == idx {
            return info;
        }
        node = e.mem.u32(node + 4);
    }
    0
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), GameAPI.cpp: ExtractSetStatementVar, with
// ScriptAnalyzer.cpp: SetToStatement and ScriptIterator::ReadVariable for the variable.
/// A command run from a `set ... to` statement's expression (its data starts at its `X` token):
/// the statement's variable's value and the mod that owns it, the local recorded as holding a
/// string. `None` for any other line.
pub(crate) fn set_statement_var(e: &mut Engine, a: &CommandArgs) -> Option<(f64, u8)> {
    let byte = |e: &Engine, at: u32| e.mem.is_mapped(at).then(|| e.mem.u8(at));
    let args_at = a.data.wrapping_add(e.mem.u32(a.opcode_offset));
    let mut back = args_at.wrapping_sub(5);
    if byte(e, back)? != b'X' {
        return None;
    }
    if byte(e, back.wrapping_sub(3))? == b'r' {
        back = back.wrapping_sub(3);
    }
    back = if byte(e, back.wrapping_sub(9))? == b'r' {
        back.wrapping_sub(13)
    } else {
        back.wrapping_sub(10)
    };
    if byte(e, back)? != 0x15 || e.mem.u16(back) != 0x15 {
        return None;
    }
    let mut at = back + 4;
    match e.mem.u8(at) {
        b'r' | b's' | b'f' => {
            let mut referenced = 0;
            let mut var_script = a.script;
            if e.mem.u8(at) == b'r' {
                let ref_var = args::script_ref(e, a.script, u32::from(e.mem.u16(at + 1)));
                at += 3;
                if ref_var == 0 {
                    return None;
                }
                let form = e.mem.u32(ref_var + 8);
                if form == 0 {
                    return None;
                }
                var_script = referenced_script(e, form);
                if var_script == 0 {
                    return None;
                }
                referenced = form;
            }
            let var_type = e.mem.u8(at);
            if var_type != b'f' && var_type != b's' {
                return None;
            }
            let info = variable_info(e, var_script, u32::from(e.mem.u16(at + 1)));
            if info == 0 {
                return None;
            }
            let local = args::event_list_variable(e, a.event_list, e.mem.u32(info));
            if local == 0 {
                return None;
            }
            let data = e.mem.f64(local + 8);
            let quest = crate::functions::quest_of(e, a.script);
            if referenced == 0 || referenced == quest || referenced == a.this_obj {
                vars::collect(e, a.event_list, local, vars::VarType::String);
            }
            Some((data, mod_index(e, a.script)))
        }
        b'G' => {
            let ref_var = args::script_ref(e, a.script, u32::from(e.mem.u16(at + 1)));
            let form = if ref_var == 0 {
                0
            } else {
                e.mem.u32(ref_var + 8)
            };
            let global = dynamic_cast(e, form, rtti::TES_FORM, RTTI_TES_GLOBAL);
            if global == 0 {
                return None;
            }
            Some((f64::from(e.mem.f32(global + 0x24)), mod_index(e, global)))
        }
        _ => None,
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), StringVar.cpp: AssignToStringVarLong.
/// A string command's result: the `set` statement's string set (no longer temporary) or a new
/// string; inside an expression a new temporary string. Text of `kMaxMessageLength` or more is
/// assigned as `""`.
pub(crate) fn assign_to_string_var(e: &mut Engine, a: &CommandArgs, text: &[u8]) {
    let text = if text.len() >= MAX_MESSAGE_LENGTH {
        &[][..]
    } else {
        text
    };
    let in_expression = expr::active(e);
    let target = if in_expression {
        None
    } else {
        set_statement_var(e, a)
    };
    let mut modi = target.map_or(0, |t| t.1);
    if modi == 0 {
        modi = mod_index(e, a.script);
    }
    let id = if in_expression {
        f64::from(vars::add_string(e, modi, text, true))
    } else if let Some((data, _)) = target.filter(|t| vars::string(e, dtoui3(t.0)).is_some()) {
        let id = dtoui3(data);
        vars::set_string(e, id, text);
        vars::mark_string_temporary(e, id, false);
        data
    } else {
        f64::from(vars::add_string(e, modi, text, target.is_none()) as i32)
    };
    a.set_result(e, id);
}

/// The string variable `id` holds, if any (`g_StringMap.Get`).
fn string_of(e: &Engine, id: u32) -> Option<Vec<u8>> {
    vars::string(e, id)
}

// ---- format strings ----

/// `ScriptFormatStringArgs`: the arguments after the format string, read as its specifiers ask.
struct FmtArgs {
    /// `numArgs`, a `UInt32` that skipping can wrap.
    num_args: u32,
    at: u32,
    data: u32,
    script: u32,
    event_list: u32,
}

/// `EventListFromForm` as `ResolveExternalVar` reads it: a quest's list (`+0x5C`) or a
/// reference's script's.
fn event_list_of(e: &mut Engine, form: u32) -> u32 {
    let kind = e.mem.u8(form + 4);
    if kind == TYPE_QUEST {
        return e.mem.u32(form + 0x5c);
    }
    if (0x3a..=0x40).contains(&kind) || kind == 0x69 {
        let extra = e
            .call(EXTRA_GET_BY_TYPE, &args![form + 0x44, EXTRA_SCRIPT])
            .u32();
        let script = dynamic_cast(e, extra, RTTI_BS_EXTRA_DATA, RTTI_EXTRA_SCRIPT);
        return if script == 0 {
            0
        } else {
            e.mem.u32(script + 0x10)
        };
    }
    0
}

impl FmtArgs {
    // Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), GameAPI.cpp:
    // ScriptFormatStringArgs::Arg (kArgType_Float) and ExtractFloat.
    fn float(&mut self, e: &mut Engine) -> Option<f64> {
        if self.num_args == 0 {
            return None;
        }
        self.num_args -= 1;
        if e.mem.u16(self.at) == 0xffff {
            return match expr::inline_argument(
                e,
                0x2c,
                &mut self.at,
                self.data,
                self.script,
                self.event_list,
            )? {
                Arg::Double(v) => Some(v),
                _ => None,
            };
        }
        args::extract_float_at(e, &mut self.at, self.script, self.event_list)
    }

    // Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), GameAPI.cpp:
    // ScriptFormatStringArgs::Arg (kArgType_Form), ExtractFormFromFloat, ResolveExternalVar.
    /// A form (0 for none; only too few arguments fail).
    fn form(&mut self, e: &mut Engine) -> Option<u32> {
        if self.num_args == 0 {
            return None;
        }
        self.num_args -= 1;
        if e.mem.u16(self.at) == 0xffff {
            let arg = expr::inline_argument(
                e,
                0x3d,
                &mut self.at,
                self.data,
                self.script,
                self.event_list,
            );
            return Some(match arg {
                Some(Arg::Form(f)) => f,
                _ => 0,
            });
        }
        let mut list = self.event_list;
        if e.mem.u8(self.at) == b'r' {
            let ref_var = args::script_ref(e, self.script, u32::from(e.mem.u16(self.at + 1)));
            if ref_var == 0 {
                return Some(0);
            }
            args::resolve_ref_variable(e, ref_var, self.event_list);
            let form = e.mem.u32(ref_var + 8);
            if form == 0 {
                return Some(0);
            }
            self.at += 3;
            list = event_list_of(e, form);
            if list == 0 {
                return Some(0);
            }
        }
        let idx = u32::from(e.mem.u16(self.at + 1));
        self.at += 3;
        let local = args::event_list_variable(e, list, idx);
        if local == 0 {
            return Some(0);
        }
        let id = e.mem.u32(local + 8);
        Some(e.call(LOOKUP_FORM, &args![id]).u32())
    }

    // Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), GameAPI.cpp:
    // ScriptFormatStringArgs::SkipArgs.
    fn skip(&mut self, e: &Engine, count: u32) {
        for _ in 0..count {
            self.at += match e.mem.u8(self.at) {
                b'z' => 9,
                b'r' => 6,
                _ => 3,
            };
            self.num_args = self.num_args.wrapping_sub(1);
        }
    }
}

/// `ConvertLiteralPercents`: every `%` of text copied in is doubled.
fn literal_percents(text: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(text.len());
    for &b in text {
        out.push(b);
        if b == b'%' {
            out.push(b'%');
        }
    }
    out
}

/// `StrCopy` of a C string: up to its first NUL.
pub(crate) fn c_str(text: &[u8]) -> &[u8] {
    match text.iter().position(|&b| b == 0) {
        Some(n) => &text[..n],
        None => text,
    }
}

/// MSVC's `double` to `UInt64` (`__dtoul3`): truncation, negative values through `SInt64`.
fn dtoul3(v: f64) -> u64 {
    if v.is_nan() {
        return 0x8000_0000_0000_0000;
    }
    if v < 0.0 {
        if v < -9_223_372_036_854_775_808.0 {
            0x8000_0000_0000_0000
        } else {
            (v as i64) as u64
        }
    } else if v >= 18_446_744_073_709_551_616.0 {
        0x8000_0000_0000_0000
    } else {
        v as u64
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), GameAPI.cpp: ExtractFormattedString.
/// The text a format string makes; `None` when an argument it asks for is missing or unreadable.
fn formatted_string(e: &mut Engine, args: &mut FmtArgs, format: &[u8]) -> Option<Vec<u8>> {
    let mut fmt = c_str(format).to_vec();
    let mut f = [0u64; 20];
    let mut arg_idx = 0usize;
    let mut no_arg_format = false;
    let mut res: Vec<u8> = Vec::new();
    let mut src = 0usize;
    let mut push_double = |f: &mut [u64; 20], bits: u64| {
        if arg_idx < 20 {
            f[arg_idx] = bits;
        }
        arg_idx += 1;
    };
    while let Some(rel) = fmt[src..].iter().position(|&b| b == b'%') {
        let percent = src + rel;
        res.extend_from_slice(&fmt[src..percent]);
        let mut at = percent + 1;
        let c = fmt.get(at).copied().unwrap_or(0);
        match c {
            b'%' => {
                res.extend_from_slice(b"%%");
                no_arg_format = true;
            }
            b'z' | b'Z' => {
                let d = args.float(e)?;
                if let Some(s) = string_of(e, dtoui3(d)) {
                    res.extend_from_slice(c_str(&s));
                }
            }
            b'r' | b'R' => res.push(b'\n'),
            b'e' | b'E' => {}
            b'a' | b'A' => {
                let d = args.float(e)?;
                res.push(f64_to_i32(d) as u8);
            }
            b'n' | b'N' => {
                let form = args.form(e)?;
                let name = full_name(e, form);
                res.extend(literal_percents(c_str(&name)));
            }
            b'i' | b'I' => {
                let form = args.form(e)?;
                let id = if form == 0 { 0 } else { e.mem.u32(form + 0xc) };
                res.extend_from_slice(format!("{id:08X}").as_bytes());
            }
            b'c' | b'C' => {
                let form = args.form(e)?;
                if form != 0 {
                    let d = args.float(e)?;
                    match e.mem.u8(form + 4) {
                        TYPE_AMMO => {
                            let text = match f64_to_i32(d) {
                                1 => bs_string(e, form + 0xc4).unwrap_or_default(),
                                2 => bs_string(e, form + 0xcc).unwrap_or_default(),
                                _ => full_name(e, form),
                            };
                            res.extend(literal_percents(c_str(&text)));
                        }
                        TYPE_FACTION => {
                            let name = nth_rank_name(e, form, dtoui3(d), false);
                            res.extend(literal_percents(c_str(&name.unwrap_or_default())));
                        }
                        _ => {}
                    }
                }
            }
            b'k' | b'K' => {
                let d = args.float(e)?;
                res.extend(dx_description(e, dtoui3(d)));
            }
            b'v' | b'V' => {
                let d = args.float(e)?;
                res.extend(actor_value_string(e, dtoui3(d)));
            }
            b'p' | b'P' => {
                at += 1;
                let mut form = args.form(e)?;
                if form != 0 {
                    if dynamic_cast(e, form, rtti::TES_FORM, rtti::TES_OBJECT_REFR) != 0 {
                        form = base_form(e, form);
                    }
                    let kind = if form != 0 && e.mem.u8(form + 4) == TYPE_NPC {
                        if e.mem.u32(form + 0x34) & 1 != 0 {
                            2
                        } else {
                            1
                        }
                    } else {
                        0
                    };
                    let word: &[u8] = match (fmt.get(at).copied().unwrap_or(0), kind) {
                        (b'o' | b'O', 1) => b"him",
                        (b'o' | b'O', 2) => b"her",
                        (b'o' | b'O', _) => b"it",
                        (b's' | b'S', 1) => b"he",
                        (b's' | b'S', 2) => b"she",
                        (b's' | b'S', _) => b"it",
                        (b'p' | b'P', 1) => b"his",
                        (b'p' | b'P', 2) => b"her",
                        (b'p' | b'P', _) => b"its",
                        _ => b"",
                    };
                    res.extend_from_slice(word);
                }
            }
            b'q' | b'Q' => res.push(b'"'),
            b'{' => {
                let close = fmt[at + 1..]
                    .windows(2)
                    .position(|w| w == b"%}")
                    .map(|i| at + 1 + i);
                if let Some(close) = close {
                    let d = args.float(e)?;
                    if d != 0.0 {
                        fmt[close + 1] = b'e';
                    } else {
                        let mut pos = at + 1;
                        while let Some(rel) = fmt[pos..].iter().position(|&b| b == b'%') {
                            let sp = pos + rel;
                            if sp >= close || args.num_args == 0 {
                                break;
                            }
                            let spec = fmt.get(sp + 1).copied().unwrap_or(0);
                            match spec {
                                b'%' | b'q' | b'Q' | b'r' | b'R' => {}
                                b'c' | b'C' => args.skip(e, 2),
                                _ => args.skip(e, 1),
                            }
                            pos = sp + 2;
                        }
                        at = close + 1;
                    }
                }
            }
            b'}' => {}
            b'x' | b'X' => {
                let d = args.float(e)?;
                push_double(&mut f, dtoul3(d));
                res.extend_from_slice(b"%0");
                if let Some(&w) = fmt.get(at + 1).filter(|w| w.is_ascii_digit()) {
                    res.push(w);
                    at += 1;
                }
                res.extend_from_slice(b"llX");
            }
            0 => {
                // A `%` that ends the text: xNVSE reads on past the string's end; here it ends.
                args.float(e)?;
                res.push(b'%');
                src = fmt.len();
                break;
            }
            _ => {
                let d = args.float(e)?;
                push_double(&mut f, d.to_bits());
                res.push(b'%');
                res.push(c);
            }
        }
        src = (at + 1).min(fmt.len());
    }
    res.extend_from_slice(&fmt[src.min(fmt.len())..]);
    let res = c_str(&res).to_vec();
    if res.is_empty() {
        return Some(Vec::new());
    }
    if arg_idx > 0 || no_arg_format {
        let mut list = Vec::with_capacity(160);
        for bits in f {
            list.extend_from_slice(&bits.to_le_bytes());
        }
        let mut out = ucrt_format(e, &res, &list);
        out.truncate(MAX_MESSAGE_LENGTH - 3);
        // The buffer is a C string: a NUL a conversion wrote (`%c` of 0) ends it.
        let end = c_str(&out).len();
        out.truncate(end);
        Some(out)
    } else {
        Some(res)
    }
}

/// What `ExtractFormatStringArgs` gave: whether it succeeded, the formatted text (empty when the
/// format failed) and the parameters after the format string that it read.
struct Formatted {
    ok: bool,
    text: Vec<u8>,
    args: Vec<Arg>,
}

impl Formatted {
    fn int(&self, i: usize, default: u32) -> u32 {
        self.args
            .get(i)
            .and_then(Arg::int)
            .map_or(default, |v| v as u32)
    }
    fn form(&self, i: usize) -> u32 {
        self.args.get(i).and_then(Arg::form).unwrap_or(0)
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), GameAPI.cpp: ExtractFormatStringArgs
// (format string first, `fmtStringPos` 0 for every caller) and the ScriptFormatStringArgs
// constructor.
fn format_string_args(e: &mut Engine, a: &CommandArgs, max_params: u32) -> Formatted {
    let fail = Formatted {
        ok: false,
        text: Vec::new(),
        args: Vec::new(),
    };
    let start = a.data.wrapping_add(e.mem.u32(a.opcode_offset));
    let count = e.mem.u16(start) as i16;
    if count < 0 {
        return fail;
    }
    let mut at = start + 2;
    let format = if e.mem.u16(at) == 0xffff {
        match expr::inline_argument(e, 0, &mut at, a.data, a.script, a.event_list) {
            Some(Arg::Str { text, .. }) => text,
            _ => return fail,
        }
    } else {
        let len = u32::from(e.mem.u16(at));
        let raw = e.mem.bytes(at + 2, len);
        at += 2 + len;
        let mut text = c_str(&raw).to_vec();
        if text.first() == Some(&b'$') && a.event_list != 0 {
            let script = e.mem.u32(a.event_list);
            let upper = vars::upper_table(e);
            let mut node = script + 0x4c;
            while node != 0 {
                let info = e.mem.u32(node);
                if info != 0 {
                    let name = text_at(e, e.mem.u32(info + 0x18));
                    if vars::str_compare(&upper, &name, &text[1..]) == Ordering::Equal {
                        let local = args::event_list_variable(e, a.event_list, e.mem.u32(info));
                        if local != 0 {
                            if let Some(s) = string_of(e, dtoui3(e.mem.f64(local + 8))) {
                                text = c_str(&s).to_vec();
                            }
                        }
                        break;
                    }
                }
                node = e.mem.u32(node + 4);
            }
        }
        text
    };
    let mut fa = FmtArgs {
        num_args: i32::from(count - 1) as u32,
        at,
        data: a.data,
        script: a.script,
        event_list: a.event_list,
    };
    let Some(text) = formatted_string(e, &mut fa, &format) else {
        return fail;
    };
    let mut num = fa.num_args as i16;
    if num > 0 {
        if i32::from(num) + 21 > max_params as i32 {
            num = (max_params as i32 - 21) as i16;
        }
        if num <= 0 {
            return Formatted {
                ok: num == 0,
                text,
                args: Vec::new(),
            };
        }
        let mut pos = fa.at;
        let read = args::read_args_at(
            e,
            num as u32,
            a.params + 12 * 21,
            &mut pos,
            a.data,
            a.script,
            a.event_list,
        );
        return Formatted {
            ok: read.ok,
            text,
            args: read.args,
        };
    }
    Formatted {
        ok: true,
        text,
        args: Vec::new(),
    }
}

// ---- StringVar (`StringVar.cpp` (xNVSE)) ----

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), StringVar.cpp: StringVar::Insert.
fn sv_insert_at(data: &mut Vec<u8>, sub: &[u8], pos: u32) {
    let len = data.len() as u32;
    if pos <= len {
        super::array::check_bytes(data.len() + sub.len());
    }
    if pos < len {
        let tail = data.split_off(pos as usize);
        data.extend_from_slice(sub);
        data.extend(tail);
    } else if pos == len {
        data.extend_from_slice(sub);
    }
}

/// `std::string::find(sub, from)`: the first position at or after `from`, `None` for none (an
/// empty `sub` is found at `from` when `from` is not past the end).
fn find_bytes(hay: &[u8], sub: &[u8], from: usize) -> Option<usize> {
    if from > hay.len() {
        return None;
    }
    if sub.is_empty() {
        return Some(from);
    }
    hay[from..]
        .windows(sub.len())
        .position(|w| w == sub)
        .map(|i| i + from)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), StringVar.cpp: StringVar::Find.
fn sv_find_in(
    e: &mut Engine,
    data: &[u8],
    sub: &[u8],
    start: u32,
    mut num: u32,
    case: bool,
) -> u32 {
    let len = data.len() as u32;
    if num.wrapping_add(start) >= len {
        num = len.wrapping_sub(start);
    }
    if start >= len {
        return u32::MAX;
    }
    let end = (start as usize)
        .saturating_add(num as usize)
        .min(data.len());
    let mut source = data[start as usize..end].to_vec();
    let mut sub = sub.to_vec();
    if !case {
        source = change_case(e, &source, false);
        sub = change_case(e, &sub, false);
    }
    match find_bytes(&source, &sub, 0) {
        Some(pos) => pos as u32 + start,
        None => u32::MAX,
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), StringVar.cpp: StringVar::Count.
fn sv_count_in(
    e: &mut Engine,
    data: &[u8],
    sub: &[u8],
    start: u32,
    mut num: u32,
    case: bool,
) -> u32 {
    let len = data.len() as u32;
    if num.wrapping_add(start) >= len {
        num = len.wrapping_sub(start);
    }
    if start >= len {
        return 0;
    }
    let end = (start as usize)
        .saturating_add(num as usize)
        .min(data.len());
    let mut source = data[start as usize..end].to_vec();
    if sub.is_empty() {
        return 0;
    }
    let mut sub = sub.to_vec();
    if !case {
        source = change_case(e, &source, false);
        sub = change_case(e, &sub, false);
    }
    let mut idx = 0usize;
    let mut count = 0;
    while (idx as u32) < len {
        match find_bytes(&source, &sub, idx) {
            Some(at) => {
                count += 1;
                idx = at + sub.len();
            }
            None => break,
        }
    }
    count
}

/// The longest string a replacement of an empty text makes when it is capped: always for a
/// replace-all (xNVSE never stops), with the fixes preset's row for a given count (xNVSE runs
/// out of memory).
const MAX_REPLACED: usize = 64 << 20;

/// The exe's `tolower` of every byte (each passed as xNVSE's `char`, sign-extended).
fn lower_table(e: &mut Engine) -> [u8; 256] {
    let mut t = [0u8; 256];
    for (b, slot) in t.iter_mut().enumerate() {
        *slot = game_case(e, false, b as u8 as i8 as i32 as u32) as u8;
    }
    t
}

/// `std::search` with `ci_equal`: the first place at or after `from` where `needle` matches
/// without case (both through the exe's `tolower`).
fn find_ci(lower: &[u8; 256], hay: &[u8], needle: &[u8], from: usize) -> Option<usize> {
    if from > hay.len() {
        return None;
    }
    if needle.is_empty() {
        return Some(from);
    }
    hay[from..]
        .windows(needle.len())
        .position(|w| {
            w.iter()
                .zip(needle)
                .all(|(&a, &b)| lower[usize::from(a)] == lower[usize::from(b)])
        })
        .map(|i| i + from)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), StringVar.cpp: StringVar::Replace.
#[allow(clippy::too_many_arguments)]
fn sv_replace_in(
    e: &mut Engine,
    data: &mut Vec<u8>,
    find: &[u8],
    with: &[u8],
    start: u32,
    mut num: u32,
    case: bool,
    how_many: u32,
) -> u32 {
    let len = data.len() as u32;
    if start >= len {
        return 0;
    } else if num.wrapping_add(start) > len {
        num = len - start;
    }
    let end = (start as usize)
        .saturating_add(num as usize)
        .min(data.len());
    let mut src: Vec<u8> = data.drain(start as usize..end).collect();
    let lower = if case { [0u8; 256] } else { lower_table(e) };
    let capped = how_many == u32::MAX || super::array::fix_on(e, super::array::SIZE_CAPS);
    let mut replaced = 0u32;
    let mut idx = 0usize;
    while replaced < how_many {
        let found = if case {
            find_bytes(&src, find, idx)
        } else {
            find_ci(&lower, &src, find, idx)
        };
        let Some(at) = found else {
            break;
        };
        idx = at;
        if find.is_empty() {
            // Nothing is ever removed: the replacement goes in at the same place each time.
            if idx == src.len() {
                src.extend_from_slice(with);
                replaced += 1;
                break;
            }
            let left = u64::from(how_many - replaced);
            let room = if with.is_empty() || !capped {
                left
            } else {
                left.min((MAX_REPLACED.saturating_sub(src.len()) / with.len()) as u64)
            };
            super::array::check_bytes(
                (data.len() + src.len()).saturating_add((room as usize).saturating_mul(with.len())),
            );
            let insert = with.repeat(room as usize);
            let tail = src.split_off(idx);
            src.extend(insert);
            src.extend(tail);
            replaced += room as u32;
            if room < left {
                break;
            }
            continue;
        }
        replaced += 1;
        let cut = (idx + find.len()).min(src.len());
        src.drain(idx..cut);
        super::array::check_bytes(data.len() + src.len() + with.len());
        if idx == src.len() {
            src.extend_from_slice(with);
            break;
        }
        let tail = src.split_off(idx);
        src.extend_from_slice(with);
        src.extend(tail);
        idx += with.len();
    }
    if start as usize == data.len() {
        data.extend(src);
    } else {
        let tail = data.split_off(start as usize);
        data.extend(src);
        data.extend(tail);
    }
    replaced
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), StringVar.cpp: StringVar::Erase.
fn sv_erase_in(data: &mut Vec<u8>, start: u32, mut num: u32) {
    let len = data.len() as u32;
    if num.wrapping_add(start) >= len {
        num = len.wrapping_sub(start);
    }
    if start < len {
        let end = (start as usize)
            .saturating_add(num as usize)
            .min(data.len());
        data.drain(start as usize..end);
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), StringVar.cpp: StringVar::SubString.
fn sv_substring_of(data: &[u8], start: u32, mut num: u32) -> Vec<u8> {
    let len = data.len() as u32;
    if num.wrapping_add(start) >= len {
        num = len.wrapping_sub(start);
    }
    if start < len {
        let end = (start as usize)
            .saturating_add(num as usize)
            .min(data.len());
        data[start as usize..end].to_vec()
    } else {
        Vec::new()
    }
}

/// `isspace` of the "C" locale, as `Trim` asks it of each `unsigned char`.
fn is_space(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\n' | 0x0b | 0x0c | b'\r')
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), StringVar.cpp: StringVar::Trim.
fn trimmed(data: &[u8]) -> Vec<u8> {
    let start = data
        .iter()
        .position(|&b| !is_space(b))
        .unwrap_or(data.len());
    let end = data
        .iter()
        .rposition(|&b| !is_space(b))
        .map_or(start, |i| i + 1);
    data[start..end.max(start)].to_vec()
}

/// `kCharType_*`.
const ALPHABETIC: u32 = 1;
const DIGIT: u32 = 2;
const PUNCTUATION: u32 = 4;
const PRINTABLE: u32 = 8;
const UPPERCASE: u32 = 16;

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), StringVar.cpp: StringVar::GetCharType.
/// The C locale's classes of a `char` (a code from 0x80 up is a negative `char`, of no class).
fn char_type(code: u32) -> u32 {
    let ch = code as u8;
    if ch >= 0x80 {
        return 0;
    }
    let mut t = 0;
    if ch.is_ascii_alphabetic() {
        t |= ALPHABETIC;
    }
    if ch.is_ascii_digit() {
        t |= DIGIT;
    }
    if ch.is_ascii_punctuation() {
        t |= PUNCTUATION;
    }
    if (0x20..0x7f).contains(&ch) {
        t |= PRINTABLE;
    }
    if ch.is_ascii_uppercase() {
        t |= UPPERCASE;
    }
    t
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Utilities.cpp: Tokenizer::NextToken.
/// The tokens between the delimiters (any of `delims`), as `sv_Split` collects them.
fn tokens(data: &[u8], delims: &[u8]) -> Vec<Vec<u8>> {
    let mut out = Vec::new();
    let mut offset = 0;
    while offset != data.len() {
        let Some(start) = data[offset..]
            .iter()
            .position(|b| !delims.contains(b))
            .map(|i| i + offset)
        else {
            break;
        };
        let end = data[start..]
            .iter()
            .position(|b| delims.contains(b))
            .map_or(data.len(), |i| i + start);
        offset = end;
        out.push(data[start..end].to_vec());
    }
    out
}

/// `std::views::split(data, delim)`: the pieces between each whole `delim` (an empty one splits
/// every character apart); an empty text has none.
fn split_on(data: &[u8], delim: &[u8]) -> Vec<Vec<u8>> {
    if data.is_empty() {
        return Vec::new();
    }
    if delim.is_empty() {
        return data.iter().map(|&b| vec![b]).collect();
    }
    let mut out = Vec::new();
    let mut start = 0;
    while let Some(at) = find_bytes(data, delim, start) {
        out.push(data[start..at].to_vec());
        start = at + delim.len();
    }
    out.push(data[start..].to_vec());
    out
}

// ---- commands ----

/// A form command's form: the given one's base form when it is a reference
/// (`TryGetREFRParent`), else the calling reference's base form.
fn form_or_base(e: &mut Engine, form: u32, this_obj: u32, parent: bool) -> u32 {
    let form = if parent && form != 0 {
        try_get_refr_parent(e, form)
    } else {
        form
    };
    if form == 0 && this_obj != 0 {
        base_form(e, this_obj)
    } else {
        form
    }
}

/// An integer argument read as `UInt32` (an integer, a 16-bit or an 8-bit parameter).
fn word(read: &Read, i: usize, default: u32) -> u32 {
    match read.args.get(i) {
        Some(Arg::Int(v)) => *v as u32,
        Some(Arg::Short(v)) => *v,
        Some(Arg::Byte(v)) => u32::from(*v),
        _ => default,
    }
}

/// An expression command's run (`ExpressionEvaluator` for the command's whole run): its
/// arguments, or `None` when they could not be read; the evaluator's errors reported at the end.
pub(crate) fn with_eval(
    e: &mut Engine,
    a: &CommandArgs,
    f: impl FnOnce(&mut Engine, &mut Eval, Option<Vec<Tok>>),
) {
    let mut ev = Eval::new(e, a);
    expr::with_evaluator(e, |e| {
        let toks = ev.extract_args(e);
        f(e, &mut ev, toks);
    });
    expr::report(e, &ev);
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_String.cpp:
// Cmd_sv_Destruct_Execute.
fn sv_destruct(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    let first = e.mem.u8(a.data);
    if first == b'X' || first == b'r' {
        a.set_result(e, 0.0);
        if let Some((data, _)) = set_statement_var(e, &a) {
            vars::delete_string(e, dtoui3(data));
        }
        return handled(true);
    }
    with_eval(e, &a, |e, _, toks| {
        for t in toks.unwrap_or_default() {
            if let Tok::Var {
                ty: Ty::StringVar,
                local,
                ..
            } = t
            {
                if local != 0 {
                    let id = dtoui3(e.mem.f64(local + 8));
                    vars::delete_string(e, id);
                    e.mem.set_f64(local + 8, 0.0);
                }
            }
        }
    });
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_String.cpp:
// Cmd_sv_Construct_Execute.
fn sv_construct(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    let f = format_string_args(e, &a, FORMAT_STRING.len() as u32);
    assign_to_string_var(e, &a, &f.text);
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_String.cpp: Cmd_sv_Set_Execute.
fn sv_set(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    let f = format_string_args(e, &a, SV_SET.len() as u32);
    if f.ok {
        vars::set_string(e, f.int(0, 0), &f.text);
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_String.cpp:
// Cmd_sv_Compare_Execute, StringVar::Compare.
fn sv_compare(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, -2.0);
    let f = format_string_args(e, &a, SV_COMPARE.len() as u32);
    if !f.ok {
        return handled(true);
    }
    let Some(lhs) = string_of(e, f.int(0, 0)) else {
        return handled(true);
    };
    let ord = if f.int(1, 0) != 0 {
        f.text.as_slice().cmp(c_str(&lhs))
    } else {
        let upper = vars::upper_table(e);
        vars::str_compare(&upper, &f.text, c_str(&lhs))
    };
    a.set_result(e, f64::from(ord as i8));
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_String.cpp: Cmd_sv_Length_Execute.
fn sv_length(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, -1.0);
    let read = Read::of(e, &a);
    if read.ok {
        if let Some(s) = string_of(e, read.int(0, 0)) {
            a.set_result(e, f64::from(s.len() as u32));
        }
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_String.cpp: Cmd_sv_Erase_Execute.
fn sv_erase(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    let read = Read::of(e, &a);
    if !read.ok {
        return handled(true);
    }
    let (id, start, mut many) = (read.int(0, 0), read.int(1, 0), read.int(2, u32::MAX));
    if let Some(mut s) = string_of(e, id) {
        if many == u32::MAX {
            many = (s.len() as u32).wrapping_sub(start);
        }
        sv_erase_in(&mut s, start, many);
        vars::set_string(e, id, &s);
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_String.cpp:
// Cmd_sv_SubString_Execute.
fn sv_substring(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    let read = Read::of(e, &a);
    let mut sub = Vec::new();
    if read.ok {
        let (start, mut many) = (read.int(1, 0), read.int(2, u32::MAX));
        let Some(s) = string_of(e, read.int(0, 0)) else {
            return handled(true);
        };
        if many == u32::MAX {
            many = (s.len() as u32).wrapping_sub(start);
        }
        sub = sv_substring_of(&s, start, many);
    }
    assign_to_string_var(e, &a, &sub);
    handled(true)
}

#[derive(Clone, Copy, PartialEq)]
enum FindMode {
    Find,
    Count,
    Replace,
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_String.cpp:
// StringVar_Find_Execute.
fn find_execute(e: &mut Engine, w: &[u32], mode: FindMode, max: u32) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, -1.0);
    let f = format_string_args(e, &a, max);
    if !f.ok {
        return handled(true);
    }
    let (id, start, mut num, case, how_many) = (
        f.int(0, 0),
        f.int(1, 0),
        f.int(2, u32::MAX),
        f.int(3, 0) != 0,
        f.int(4, u32::MAX),
    );
    let mut result = u32::MAX;
    if let Some(mut s) = string_of(e, id) {
        if num == u32::MAX {
            num = (s.len() as u32).wrapping_sub(start);
        }
        match mode {
            FindMode::Find => result = sv_find_in(e, &s, &f.text, start, num, case),
            FindMode::Count => result = sv_count_in(e, &s, &f.text, start, num, case),
            FindMode::Replace => {
                let sep = separator(e, a.script);
                if let Some(split) = f.text.iter().position(|&b| b == sep) {
                    let (find, with) = (&f.text[..split], &f.text[split + 1..]);
                    result = sv_replace_in(e, &mut s, find, with, start, num, case, how_many);
                    vars::set_string(e, id, &s);
                }
            }
        }
    }
    if result != u32::MAX {
        a.set_result(e, f64::from(result));
    }
    handled(true)
}

fn sv_find(e: &mut Engine, w: &[u32]) -> Ret {
    find_execute(e, w, FindMode::Find, SV_FIND.len() as u32)
}

fn sv_count(e: &mut Engine, w: &[u32]) -> Ret {
    find_execute(e, w, FindMode::Count, SV_FIND.len() as u32)
}

fn sv_replace(e: &mut Engine, w: &[u32]) -> Ret {
    find_execute(e, w, FindMode::Replace, SV_REPLACE.len() as u32)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_String.cpp:
// Cmd_sv_ToNumeric_Execute.
fn sv_to_numeric(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    let read = Read::of(e, &a);
    if !read.ok {
        return handled(true);
    }
    if let Some(s) = string_of(e, read.int(0, 0)) {
        let from = (read.int(1, 0) as usize).min(s.len());
        a.set_result(e, crate::cnum::strtod(c_str(&s[from..])).0);
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_String.cpp: Cmd_sv_Insert_Execute.
fn sv_insert(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    let f = format_string_args(e, &a, SV_COMPARE.len() as u32);
    if !f.ok {
        return handled(true);
    }
    let id = f.int(0, 0);
    if let Some(mut s) = string_of(e, id) {
        sv_insert_at(&mut s, &f.text, f.int(1, 0));
        vars::set_string(e, id, &s);
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_String.cpp:
// Cmd_sv_GetChar_Execute, StringVar::At.
fn sv_get_char(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, -1.0);
    let read = Read::of(e, &a);
    if !read.ok {
        return handled(true);
    }
    if let Some(s) = string_of(e, read.int(0, 0)) {
        let at = read.int(1, 0) as usize;
        let ch = s.get(at).map_or(-1, |&b| b as i8);
        a.set_result(e, f64::from(ch));
    }
    handled(true)
}

/// `ScriptToken::GetStringVar`: a string variable token's string id.
fn string_var_id(e: &Engine, t: &Tok) -> Option<u32> {
    match t {
        Tok::Var {
            ty: Ty::StringVar,
            local,
            string,
            ..
        } => Some(if *local != 0 {
            f64_to_i32(e.mem.f64(local + 8)) as u32
        } else {
            *string
        }),
        _ => None,
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_String.cpp: Cmd_sv_Trim_Execute.
fn sv_trim(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    with_eval(e, &a, |e, _, toks| {
        let Some(t) = toks.and_then(|t| t.into_iter().next()) else {
            return;
        };
        let Some(id) = string_var_id(e, &t) else {
            return;
        };
        let Some(s) = string_of(e, id) else {
            return;
        };
        vars::set_string(e, id, &trimmed(&s));
        a.set_result(e, 1.0);
    });
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_String.cpp:
// MatchCharType_Execute.
fn match_char_type(e: &mut Engine, w: &[u32], mask: u32) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    let read = Read::of(e, &a);
    if read.ok && char_type(read.int(0, 0)) & mask == mask {
        a.set_result(e, 1.0);
    }
    handled(true)
}

fn is_letter(e: &mut Engine, w: &[u32]) -> Ret {
    match_char_type(e, w, ALPHABETIC)
}
fn is_digit(e: &mut Engine, w: &[u32]) -> Ret {
    match_char_type(e, w, DIGIT)
}
fn is_punctuation(e: &mut Engine, w: &[u32]) -> Ret {
    match_char_type(e, w, PUNCTUATION)
}
fn is_printable(e: &mut Engine, w: &[u32]) -> Ret {
    match_char_type(e, w, PRINTABLE)
}
fn is_upper_case(e: &mut Engine, w: &[u32]) -> Ret {
    match_char_type(e, w, UPPERCASE)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_String.cpp: Cmd_ToUpper_Execute,
// Cmd_ToLower_Execute.
fn to_case(e: &mut Engine, w: &[u32], upper: bool) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    let read = Read::of(e, &a);
    if read.ok {
        let c = game_case(e, upper, read.int(0, 0));
        a.set_result(e, f64::from(c as i32));
    }
    handled(true)
}

fn to_upper(e: &mut Engine, w: &[u32]) -> Ret {
    to_case(e, w, true)
}
fn to_lower(e: &mut Engine, w: &[u32]) -> Ret {
    to_case(e, w, false)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_String.cpp:
// Cmd_CharToAscii_Execute.
fn char_to_ascii(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, -1.0);
    let read = Read::of(e, &a);
    if read.ok {
        let text = read.text(0);
        let text = c_str(&text);
        if text.len() == 1 {
            a.set_result(e, f64::from(text[0] as i8));
        }
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_String.cpp:
// Cmd_GetNthModName_Execute.
fn get_nth_mod_name(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    let read = Read::of(e, &a);
    let name = if read.ok {
        crate::errors::nth_mod_name(e, read.int(0, 0xff))
    } else {
        Vec::new()
    };
    assign_to_string_var(e, &a, &name);
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_String.cpp: Cmd_GetName_Execute.
fn get_name(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    let read = Read::of(e, &a);
    let mut name = Vec::new();
    if read.ok {
        let mut form = read.form(0);
        if form == 0 {
            form = a.this_obj;
        }
        if form != 0 {
            name = full_name(e, form);
        }
    }
    assign_to_string_var(e, &a, &name);
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_String.cpp:
// Cmd_GetStringGameSetting_Execute.
fn get_string_game_setting(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    let read = Read::of(e, &a);
    let mut value = Vec::new();
    if read.ok {
        let setting = game_setting(e, c_str(&read.text(0)));
        if setting != 0 && setting_type(e, setting) == 6 {
            value = text_at(e, setting_str(e, setting));
        }
    }
    assign_to_string_var(e, &a, &value);
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_String.cpp:
// Cmd_GetStringIniSetting_Execute.
fn get_string_ini_setting(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, -1.0);
    let read = Read::of(e, &a);
    if !read.ok {
        return handled(true);
    }
    let setting = ini_setting(e, c_str(&read.text(0)));
    if setting != 0 {
        let at = setting_str(e, setting);
        if at != 0 {
            let val = text_at(e, at);
            assign_to_string_var(e, &a, &val);
            if is_console_mode(e) {
                print_text(e, "GetStringIniSetting >> %s", &val);
            }
        }
    } else if is_console_mode(e) {
        console_print(e, "GetStringIniSetting >> SETTING NOT FOUND", &[]);
    }
    handled(true)
}

/// The `name|value` of the deprecated setters, split at the separator.
fn name_and_value(e: &mut Engine, script: u32, text: &[u8]) -> Option<(Vec<u8>, Vec<u8>)> {
    let sep = separator(e, script);
    let at = text.iter().position(|&b| b == sep)?;
    Some((text[..at].to_vec(), text[at + 1..].to_vec()))
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_String.cpp:
// Cmd_SetStringGameSettingEX_DEPRECATED_Execute (it sets an INI setting).
fn set_string_game_setting_ex_deprecated(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    let f = format_string_args(e, &a, FORMAT_STRING.len() as u32);
    if f.ok {
        if let Some((name, value)) = name_and_value(e, a.script, &f.text) {
            let setting = ini_setting(e, &name);
            if setting != 0 {
                setting_set_str(e, setting, &value);
                a.set_result(e, 1.0);
            }
        }
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_String.cpp:
// Cmd_SetStringIniSetting_DEPRECATED_Execute (it sets a game setting).
fn set_string_ini_setting_deprecated(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    let f = format_string_args(e, &a, FORMAT_STRING.len() as u32);
    if f.ok {
        if let Some((name, value)) = name_and_value(e, a.script, &f.text) {
            let setting = game_setting(e, &name);
            if setting != 0 && setting_type(e, setting) == 6 {
                setting_set_str(e, setting, &value);
                a.set_result(e, 1.0);
            }
        }
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_String.cpp:
// Cmd_SetStringIniSetting_Execute.
fn set_string_ini_setting(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    let read = Read::of(e, &a);
    if read.ok {
        let setting = ini_setting(e, c_str(&read.text(0)));
        if setting != 0 {
            if setting_type(e, setting) == 6 {
                setting_set_str(e, setting, c_str(&read.text(1)));
                a.set_result(e, 1.0);
            }
        } else if is_console_mode(e) {
            console_print(e, "SetStringIniSetting >> SETTING NOT FOUND", &[]);
        }
    }
    handled(true)
}

/// The path `String` of a form's component: `TESModel::nifPath` (`+4`), `TESIcon`'s or
/// `TESTexture`'s `ddsPath` (`+4`); 0 when the form has no such component.
fn path_string(e: &mut Engine, form: u32, rtti_to: u32) -> u32 {
    let part = dynamic_cast(e, form, rtti::TES_FORM, rtti_to);
    if part == 0 {
        0
    } else {
        part + 4
    }
}

/// A getter's path: `String::m_data`'s text, `""` for none.
fn path_text(e: &Engine, at: u32) -> Vec<u8> {
    if at == 0 {
        Vec::new()
    } else {
        bs_string(e, at).unwrap_or_default()
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_String.cpp:
// Cmd_GetModelPath_Execute, GetPath_Execute.
fn get_path(e: &mut Engine, w: &[u32], rtti_to: u32) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    let read = Read::of(e, &a);
    let mut path = Vec::new();
    if read.ok {
        let form = form_or_base(e, read.form(0), a.this_obj, true);
        let at = path_string(e, form, rtti_to);
        path = path_text(e, at);
    }
    assign_to_string_var(e, &a, &path);
    handled(true)
}

fn get_model_path(e: &mut Engine, w: &[u32]) -> Ret {
    get_path(e, w, RTTI_TES_MODEL)
}
fn get_icon_path(e: &mut Engine, w: &[u32]) -> Ret {
    get_path(e, w, RTTI_TES_ICON)
}
fn get_texture_path(e: &mut Engine, w: &[u32]) -> Ret {
    get_path(e, w, RTTI_TES_TEXTURE)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_String.cpp:
// Cmd_SetModelPath_Execute, Cmd_SetTexturePath_Execute.
fn set_path_expr(e: &mut Engine, w: &[u32], rtti_to: u32) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    with_eval(e, &a, |e, ev, toks| {
        let Some(toks) = toks.filter(|t| !t.is_empty()) else {
            return;
        };
        let form = if toks.len() == 2 {
            ev.get_form(e, &toks[1])
        } else if a.this_obj != 0 {
            base_form(e, a.this_obj)
        } else {
            0
        };
        let at = path_string(e, form, rtti_to);
        if at != 0 {
            let path = ev.get_string(e, &toks[0]);
            bs_string_set(e, at, c_str(&path));
            a.set_result(e, 1.0);
        }
    });
    handled(true)
}

fn set_model_path(e: &mut Engine, w: &[u32]) -> Ret {
    set_path_expr(e, w, RTTI_TES_MODEL)
}
fn set_texture_path(e: &mut Engine, w: &[u32]) -> Ret {
    set_path_expr(e, w, RTTI_TES_TEXTURE)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_String.cpp:
// Cmd_SetModelPathEX_Execute, Cmd_SetIconPathEX_Execute.
fn set_path_ex(e: &mut Engine, w: &[u32], rtti_to: u32, parent: bool) -> Ret {
    let a = CommandArgs::new(w);
    let f = format_string_args(e, &a, SET_PATH_EX.len() as u32);
    if f.ok {
        let form = form_or_base(e, f.form(0), a.this_obj, parent);
        let at = path_string(e, form, rtti_to);
        if at != 0 {
            bs_string_set(e, at, &f.text);
        }
    }
    handled(true)
}

fn set_model_path_ex(e: &mut Engine, w: &[u32]) -> Ret {
    set_path_ex(e, w, RTTI_TES_MODEL, true)
}
fn set_icon_path_ex(e: &mut Engine, w: &[u32]) -> Ret {
    set_path_ex(e, w, RTTI_TES_ICON, false)
}

/// `TESBipedModelForm::GetPath`/`SetPath`'s `String`: the biped model's (`+0xC`, 0x20 each), the
/// ground model's (`+0x4C`) or the icon's (`+0x8C`, 0xC each) path; 0 for another path.
fn biped_path(biped: u32, which: u32, female: bool) -> u32 {
    let f = u32::from(female);
    match which {
        0 => biped + 0xc + 0x20 * f + 4,
        1 => biped + 0x4c + 0x20 * f + 4,
        2 => biped + 0x8c + 0xc * f + 4,
        _ => 0,
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_String.cpp:
// BipedPathFunc_Execute.
fn biped_path_execute(e: &mut Engine, w: &[u32], set: bool, icon: bool) -> Ret {
    let a = CommandArgs::new(w);
    let (ok, which, form, text) = if set {
        let f = format_string_args(e, &a, SET_BIPED_PATH_EX.len() as u32);
        (f.ok, f.int(0, 0), f.form(1), f.text)
    } else {
        let read = Read::of(e, &a);
        (read.ok, read.int(0, 0), read.form(1), Vec::new())
    };
    let mut path = Vec::new();
    if ok {
        let form = form_or_base(e, form, a.this_obj, false);
        let biped = dynamic_cast(e, form, rtti::TES_FORM, rtti::TES_BIPED_MODEL_FORM);
        if biped != 0 {
            let female = which % 2 != 0;
            let which = if icon { 2 } else { which / 2 };
            let at = biped_path(biped, which, female);
            if set {
                if at != 0 {
                    bs_string_set(e, at, &text);
                }
            } else {
                path = path_text(e, at);
            }
        }
    }
    if !set {
        assign_to_string_var(e, &a, &path);
    }
    handled(true)
}

fn get_biped_model_path(e: &mut Engine, w: &[u32]) -> Ret {
    biped_path_execute(e, w, false, false)
}
fn set_biped_model_path_ex(e: &mut Engine, w: &[u32]) -> Ret {
    biped_path_execute(e, w, true, false)
}
fn get_biped_icon_path(e: &mut Engine, w: &[u32]) -> Ret {
    biped_path_execute(e, w, false, true)
}
fn set_biped_icon_path_ex(e: &mut Engine, w: &[u32]) -> Ret {
    biped_path_execute(e, w, true, true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_String.cpp:
// Cmd_GetNthFactionRankName_Execute.
fn get_nth_faction_rank_name(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    let read = Read::of(e, &a);
    let mut name = Vec::new();
    if read.ok {
        let faction = read.form(0);
        name = nth_rank_name(e, faction, read.int(1, 0), read.int(2, 0) != 0).unwrap_or_default();
    }
    assign_to_string_var(e, &a, &name);
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_String.cpp:
// Cmd_SetNthFactionRankNameEX_Execute, TESFaction::SetNthRankName.
fn set_nth_faction_rank_name_ex(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    let f = format_string_args(e, &a, SET_RANK_NAME_EX.len() as u32);
    if f.ok {
        let faction = dynamic_cast(e, f.form(0), rtti::TES_FORM, RTTI_TES_FACTION);
        if faction != 0 {
            let rank = nth_rank(e, faction, f.int(1, 0));
            if rank != 0 {
                let at = rank + if f.int(2, 0) != 0 { 8 } else { 0 };
                bs_string_set(e, at, &f.text);
            }
        }
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_String.cpp:
// Cmd_ActorValueToString_Execute, Cmd_ActorValueToStringC_Execute.
fn actor_value_name(e: &mut Engine, w: &[u32], by_kind: bool) -> Ret {
    let a = CommandArgs::new(w);
    let read = Read::of(e, &a);
    let code = word(&read, 0, 0);
    let mut name = None;
    if read.ok && code < ACTOR_VALUE_MAX {
        let info = e.mem.u32(ACTOR_VALUES + 4 * code);
        if info != 0 {
            let at = match if by_kind { read.int(1, 0) } else { 0 } {
                0 => e.mem.u32(info + 0x38),
                1 => e.mem.u32(info + 0x1c),
                2 => e.mem.u32(info + 0x3c),
                _ => 0,
            };
            if at != 0 {
                name = Some(text_at(e, at));
            }
        }
    }
    let name = name.unwrap_or_else(|| b"Unknown".to_vec());
    assign_to_string_var(e, &a, &name);
    handled(true)
}

fn actor_value_to_string(e: &mut Engine, w: &[u32]) -> Ret {
    actor_value_name(e, w, false)
}
fn actor_value_to_string_c(e: &mut Engine, w: &[u32]) -> Ret {
    actor_value_name(e, w, true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_String.cpp:
// Cmd_GetKeyName_Execute.
fn get_key_name(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    let read = Read::of(e, &a);
    let name = if read.ok {
        dx_description(e, read.int(0, 0))
    } else {
        Vec::new()
    };
    assign_to_string_var(e, &a, &name);
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_String.cpp:
// Cmd_AsciiToChar_Execute.
fn ascii_to_char(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    let read = Read::of(e, &a);
    let code = read.int(0, 0);
    let text = if read.ok && code < 256 && code != 0 {
        vec![code as u8]
    } else {
        Vec::new()
    };
    assign_to_string_var(e, &a, &text);
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_String.cpp:
// Cmd_GetFormIDString_Execute.
fn get_form_id_string(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    let read = Read::of(e, &a);
    let mut form = read.form(0);
    if form == 0 {
        form = a.this_obj;
    }
    let id = if form == 0 { 0 } else { e.mem.u32(form + 0xc) };
    assign_to_string_var(e, &a, format!("{id:08X}").as_bytes());
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_String.cpp:
// Cmd_GetRawFormIDString_Execute.
fn get_raw_form_id_string(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    let mut id = 0;
    with_eval(e, &a, |e, ev, toks| {
        if let Some([t]) = toks.as_deref() {
            if ev.can_convert(e, t, Ty::ArrayElement) {
                id = ev.get_form_id(e, t);
            } else if let Tok::Var {
                ty: Ty::RefVar,
                local,
                ..
            } = t
            {
                if *local != 0 {
                    id = e.mem.u32(local + 8);
                }
            }
        }
        assign_to_string_var(e, &a, format!("{id:08X}").as_bytes());
    });
    handled(true)
}

/// `NumToHex`/`IntToBin`'s arguments: what was read is kept (the read's result is not looked at).
fn read_ints(e: &mut Engine, a: &CommandArgs, defaults: [u32; 3]) -> [u32; 3] {
    let read = Read::of(e, a);
    [
        read.int(0, defaults[0]),
        read.int(1, defaults[1]),
        read.int(2, defaults[2]),
    ]
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_String.cpp:
// Cmd_NumToHex_OLD_Execute.
fn num_to_hex_old(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    let [num, width, _] = read_ints(e, &a, [0, 8, 0]);
    let width = width.min(8) as usize;
    assign_to_string_var(e, &a, format!("{num:0width$X}").as_bytes());
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_String.cpp:
// Cmd_NumToHex_Execute.
fn num_to_hex(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    let [num, width, prefix] = read_ints(e, &a, [0, 8, 0]);
    let width = width.min(8) as usize;
    let mut text = format!("{num:0width$X}");
    if prefix != 0 {
        text.insert_str(0, "0x");
    }
    assign_to_string_var(e, &a, text.as_bytes());
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_String.cpp:
// Cmd_IntToBin_Execute.
fn int_to_bin(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    let [num, width, prefix] = read_ints(e, &a, [0, 32, 0]);
    let width = width.min(32) as usize;
    let mut text = format!("{num:0width$b}");
    if prefix != 0 {
        text.insert_str(0, "0b");
    }
    assign_to_string_var(e, &a, text.as_bytes());
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_String.cpp:
// Cmd_ToNumber_Execute, ScriptTokens.cpp: ScriptToken::GetNumericRepresentation.
fn to_number(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    with_eval(e, &a, |e, ev, toks| {
        let Some(toks) = toks.filter(|t| !t.is_empty()) else {
            return;
        };
        let hex = toks
            .get(1)
            .filter(|t| ev.can_convert(e, t, Ty::Number))
            .is_some_and(|t| ev.get_number(e, t) != 0.0);
        let (value, error) = if ev.can_convert(e, &toks[0], Ty::Number) {
            (ev.get_number(e, &toks[0]), false)
        } else if ev.can_convert(e, &toks[0], Ty::String) {
            let s = ev.get_string(e, &toks[0]);
            expr::numeric_representation(c_str(&s), hex)
        } else {
            (0.0, true)
        };
        a.set_result(e, value);
        if let Some(report) = toks.get(2).filter(|t| ev.can_convert(e, t, Ty::Number)) {
            if ev.get_number(e, report) != 0.0 && error {
                ev.error_text("Error converting string to number.".into());
            }
        }
    });
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_String.cpp:
// Cmd_sv_Split_Execute.
fn sv_split(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    let modi = mod_index(e, a.script);
    let arr = vars::create_array(e, vars::KEY_NUMBER, true, modi);
    a.set_result(e, f64::from(arr));
    with_eval(e, &a, |e, ev, toks| {
        let Some(toks) = toks else {
            return;
        };
        if toks.len() < 2
            || !ev.can_convert(e, &toks[0], Ty::String)
            || !ev.can_convert(e, &toks[1], Ty::String)
        {
            return;
        }
        let single = toks.len() >= 3 && ev.get_bool(e, &toks[2]);
        let base = ev.get_string(e, &toks[0]);
        let delims = ev.get_string(e, &toks[1]);
        let (base, delims) = (c_str(&base), c_str(&delims));
        let pieces = if single {
            split_on(base, delims)
        } else {
            tokens(base, delims)
        };
        for (i, piece) in pieces.into_iter().enumerate() {
            vars::set_element(
                e,
                arr,
                &vars::Key::Number(i as f64),
                vars::Value::Str(piece),
            );
        }
    });
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_String.cpp:
// Cmd_GetFalloutDirectory_Execute, Utilities.cpp: GetFalloutDirectory.
/// The folder of the exe's path (`GetModuleFileName`: the folder the game started in and
/// `FalloutNV.exe`), with its last backslash; `""` for a path of `MAX_PATH` or more.
fn get_fallout_directory(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    let mut path = e.kernel.current_directory.clone();
    if !path.ends_with(b"\\") && !path.is_empty() {
        path.push(b'\\');
    }
    path.extend_from_slice(b"FalloutNV.exe");
    let dir = if path.len() < 260 {
        match path.iter().rposition(|&b| b == b'\\') {
            Some(i) => path[..=i].to_vec(),
            None => Vec::new(),
        }
    } else {
        Vec::new()
    };
    assign_to_string_var(e, &a, &dir);
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_String.cpp:
// Cmd_sv_Percentify_Execute.
fn sv_percentify(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    with_eval(e, &a, |e, ev, toks| {
        let mut converted = Vec::new();
        if let Some([t]) = toks.as_deref() {
            let s = ev.get_string(e, t);
            let text = c_str(&s);
            // Each `%` doubles: past 1 GiB the game stops as out of memory.
            super::array::check_bytes(text.len() + text.iter().filter(|&&b| b == b'%').count());
            converted = literal_percents(text);
        }
        assign_to_string_var(e, &a, &converted);
    });
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_String.cpp: ChangeCase_Execute.
fn change_case_execute(e: &mut Engine, w: &[u32], upper: bool) -> Ret {
    let a = CommandArgs::new(w);
    with_eval(e, &a, |e, ev, toks| {
        let mut converted = Vec::new();
        if let Some([t]) = toks.as_deref() {
            let s = ev.get_string(e, t);
            converted = change_case(e, c_str(&s), upper);
        }
        assign_to_string_var(e, &a, &converted);
    });
    handled(true)
}

fn sv_to_upper(e: &mut Engine, w: &[u32]) -> Ret {
    change_case_execute(e, w, true)
}
fn sv_to_lower(e: &mut Engine, w: &[u32]) -> Ret {
    change_case_execute(e, w, false)
}

/// `TESObjectWEAP::targetNIF.nifPath` (`+0x1EC`) of a weapon with a scope (`weaponFlags1`
/// `+0x100`, bit `0x4`); 0 otherwise.
fn scope_path(e: &mut Engine, form: u32) -> u32 {
    let weapon = dynamic_cast(e, form, rtti::TES_FORM, rtti::TES_OBJECT_WEAP);
    if weapon != 0 && e.mem.u8(weapon + 0x100) & 4 != 0 {
        weapon + 0x1ec
    } else {
        0
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_String.cpp:
// Cmd_GetScopeModelPath_Execute.
fn get_scope_model_path(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    let read = Read::of(e, &a);
    let mut path = Vec::new();
    if read.ok {
        let form = form_or_base(e, read.form(0), a.this_obj, false);
        let at = scope_path(e, form);
        path = path_text(e, at);
    }
    assign_to_string_var(e, &a, &path);
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_String.cpp:
// Cmd_SetScopeModelPath_Execute.
fn set_scope_model_path(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    let read = Read::of(e, &a);
    if read.ok {
        let form = form_or_base(e, read.form(1), a.this_obj, false);
        let at = scope_path(e, form);
        if at != 0 {
            bs_string_set(e, at, c_str(&read.text(0)));
        }
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_String.cpp:
// Cmd_ValidateRegex_Execute.
/// 0 for a pattern `std::regex` takes, else xNVSE's code for the error (`RegexError`), its
/// `what()` text into the optional string variable; -1 when the arguments cannot be read.
fn validate_regex(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, -1.0);
    with_eval(e, &a, |e, ev, toks| {
        let Some(toks) = toks else {
            return;
        };
        let pattern = toks
            .first()
            .map(|t| ev.get_string(e, t))
            .unwrap_or_default();
        match regex_check::check(c_str(&pattern)) {
            Ok(()) => a.set_result(e, 0.0),
            Err(error) => {
                a.set_result(e, f64::from(error.code()));
                if let Some(id) = toks.get(1).and_then(|t| string_var_id(e, t)) {
                    vars::set_string(e, id, error.what().as_bytes());
                }
            }
        }
    });
    handled(true)
}

/// The MSVC STL's `std::regex` parser for the ECMAScript grammar (`_Parser` in `<regex>`:
/// `_Compile`, `_Disjunction`, `_Alternative`, `_Quantifier`, `_Wrapped_disjunction`,
/// `_AtomEscape`, `_CharacterEscape`, `_IdentityEscape`, `_CharacterClassEscape`,
/// `_CharacterClass`, `_ClassRanges`, `_ClassAtom`, `_ClassEscape2`, `_Do_ex_class`,
/// `_Do_digits`, `_Trans`, `_Next`, `_Expect`), keeping only what decides an error: the NFA it
/// builds raises none for a `char` pattern.
///
/// Derived from the Microsoft C++ Standard Library, `<regex>`:
/// Copyright (c) Microsoft Corporation.
/// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception (`crates/nvse/LICENSE-msvc-stl`).
/// Modified: translated to Rust, reduced to the parse, and made to answer as MSVC 14.50 (the
/// version compiled into xNVSE 6.4.9's `nvse_1_4.dll`) where the 14.44 source read here
/// differs: the open-group limit, `(?` with another character, `\0` before a digit, `\b` in a
/// class, the `[. .]`/`[= =]` names (each case measured on that DLL, `regex_cases.txt`).
pub(crate) mod regex_check {
    /// `regex_constants::error_type` as xNVSE numbers it (`RegexError`), `Syntax` its
    /// `Unknown`.
    /// `Space` and `Complexity` are in xNVSE's list; the parse raises neither for a pattern a
    /// script can give.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    #[allow(dead_code)]
    pub enum RegexError {
        Collate = 1,
        Ctype,
        Escape,
        Backref,
        Brack,
        Paren,
        Brace,
        Badbrace,
        Range,
        Space,
        Badrepeat,
        Complexity,
        Stack,
        Syntax,
    }

    impl RegexError {
        pub fn code(self) -> u8 {
            self as u8
        }

        /// `regex_error::what()` (`_Stringify`).
        pub fn what(self) -> &'static str {
            match self {
                RegexError::Collate => "regex_error(error_collate): The expression contained an invalid collating element name.",
                RegexError::Ctype => "regex_error(error_ctype): The expression contained an invalid character class name.",
                RegexError::Escape => "regex_error(error_escape): The expression contained an invalid escaped character, or a trailing escape.",
                RegexError::Backref => "regex_error(error_backref): The expression contained an invalid back reference.",
                RegexError::Brack => "regex_error(error_brack): The expression contained mismatched [ and ].",
                RegexError::Paren => "regex_error(error_paren): The expression contained mismatched ( and ).",
                RegexError::Brace => "regex_error(error_brace): The expression contained mismatched { and }.",
                RegexError::Badbrace => "regex_error(error_badbrace): The expression contained an invalid range in a {} expression.",
                RegexError::Range => "regex_error(error_range): The expression contained an invalid character range, such as [b-a] in most encodings.",
                RegexError::Space => "regex_error(error_space): There was insufficient memory to convert the expression into a finite state machine.",
                RegexError::Badrepeat => "regex_error(error_badrepeat): One of *?+{ was not preceded by a valid regular expression.",
                RegexError::Complexity => "regex_error(error_complexity): The complexity of an attempted match against a regular expression exceeded a pre-set level.",
                RegexError::Stack => "regex_error(error_stack): There was insufficient memory to determine whether the regular expression could match the specified character sequence.",
                RegexError::Syntax => "regex_error(error_syntax)",
            }
        }
    }

    type R<T = ()> = Result<T, RegexError>;

    /// `_Meta_eos`, `_Meta_chr`.
    const EOS: i32 = -1;
    const CHR: i32 = 0;
    /// `_Meta_map`: the characters that are meta-characters.
    const META: &[u8] = b"()$^.*+?[]|\\-{},:=!\n\r\x08";
    /// `regex_traits::lookup_classname`'s names (compared without case).
    const CLASS_NAMES: [&[u8]; 15] = [
        b"alnum", b"alpha", b"blank", b"cntrl", b"d", b"digit", b"graph", b"lower", b"print",
        b"punct", b"space", b"s", b"upper", b"w", b"xdigit",
    ];

    #[derive(Clone, Copy, PartialEq)]
    enum Atom {
        None,
        Chr,
        Set,
    }

    struct Parser<'a> {
        pat: &'a [u8],
        pos: usize,
        mchar: i32,
        ch: i32,
        val: i32,
        grp_idx: usize,
        finished: Vec<bool>,
        disj_count: u32,
    }

    fn is_class_name(name: &[u8]) -> bool {
        CLASS_NAMES.iter().any(|n| n.eq_ignore_ascii_case(name))
    }

    /// `regex_traits::value`.
    fn value(ch: i32, base: i32) -> i32 {
        let Ok(c) = u8::try_from(ch) else {
            return -1;
        };
        if (base != 8 && c.is_ascii_digit()) || (base == 8 && (b'0'..=b'7').contains(&c)) {
            return i32::from(c - b'0');
        }
        if base != 16 {
            return -1;
        }
        match c {
            b'a'..=b'f' => i32::from(c - b'a') + 10,
            b'A'..=b'F' => i32::from(c - b'A') + 10,
            _ => -1,
        }
    }

    impl Parser<'_> {
        fn trans(&mut self) {
            match self.pat.get(self.pos) {
                None => {
                    self.mchar = EOS;
                    self.ch = EOS;
                }
                Some(&c) => {
                    self.ch = i32::from(c);
                    self.mchar = if META.contains(&c) { self.ch } else { CHR };
                }
            }
        }

        fn next(&mut self) {
            if self.pos != self.pat.len() {
                self.pos += 1;
            }
            self.trans();
        }

        fn is(&self, c: u8) -> bool {
            self.mchar == i32::from(c)
        }

        fn expect(&mut self, c: u8, code: RegexError) -> R {
            if !self.is(c) {
                return Err(code);
            }
            self.next();
            Ok(())
        }

        fn do_digits(&mut self, base: i32, mut count: i32, error: RegexError) -> R<i32> {
            self.val = 0;
            while count != 0 {
                let chv = value(self.ch, base);
                if chv == -1 {
                    break;
                }
                if self.val > (i32::MAX - chv) / base {
                    return Err(error);
                }
                count -= 1;
                self.val = self.val * base + chv;
                self.next();
            }
            Ok(count)
        }

        fn decimal_digits(&mut self, error: RegexError) -> R<bool> {
            Ok(self.do_digits(10, i32::MAX, error)? != i32::MAX)
        }

        fn hex_digits(&mut self, count: i32) -> R {
            if self.do_digits(16, count, RegexError::Escape)? != 0 {
                return Err(RegexError::Escape);
            }
            Ok(())
        }

        fn class_escape(&mut self) -> bool {
            match self.pat.get(self.pos) {
                Some(&c) if is_class_name(&[c]) => {
                    self.next();
                    true
                }
                _ => false,
            }
        }

        fn class_escape2(&mut self) -> R<Atom> {
            if self.ch == i32::from(b'\\') {
                self.val = self.ch;
                self.next();
                return Ok(Atom::Chr);
            }
            // 14.50: `\b` in a class is the backspace.
            if self.ch == i32::from(b'b') {
                self.val = 8;
                self.next();
                return Ok(Atom::Chr);
            }
            if self.class_escape() {
                return Ok(Atom::Set);
            }
            // 14.50: `\0` is the NUL character unless a digit follows it.
            if self.ch == i32::from(b'0') {
                self.next();
                if value(self.ch, 10) != -1 {
                    return Err(RegexError::Escape);
                }
                self.val = 0;
                return Ok(Atom::Chr);
            }
            if self.decimal_digits(RegexError::Escape)? {
                return Err(RegexError::Escape);
            }
            Ok(if self.character_escape()? {
                Atom::Chr
            } else {
                Atom::None
            })
        }

        fn class_atom(&mut self) -> R<Atom> {
            if self.is(b'\\') {
                self.next();
                return self.class_escape2();
            }
            if self.is(b'[') {
                self.next();
                if self.is(b':') || self.is(b'=') || self.is(b'.') {
                    let end = self.mchar;
                    self.next();
                    return self.ex_class(end);
                }
                self.val = i32::from(b'[');
                return Ok(Atom::Chr);
            }
            if self.is(b']') || self.mchar == EOS {
                return Ok(Atom::None);
            }
            self.val = self.ch;
            self.next();
            Ok(Atom::Chr)
        }

        /// A `[: :]` class (a set), a `[= =]` equivalence class (a set) or a `[. .]` collating
        /// element (one character): in 14.50 the last two name exactly one character.
        fn ex_class(&mut self, end: i32) -> R<Atom> {
            let error = if end == i32::from(b':') {
                RegexError::Ctype
            } else if end == i32::from(b'=') || end == i32::from(b'.') {
                RegexError::Collate
            } else {
                RegexError::Syntax
            };
            let beg = self.pos;
            while !(self.is(b':') || self.is(b'=') || self.is(b'.') || self.mchar == EOS) {
                self.next();
            }
            if self.mchar != end {
                return Err(error);
            }
            let atom = if end == i32::from(b':') {
                if !is_class_name(&self.pat[beg..self.pos]) {
                    return Err(RegexError::Ctype);
                }
                Atom::Set
            } else if self.pos - beg != 1 {
                return Err(RegexError::Collate);
            } else if end == i32::from(b'.') {
                self.val = i32::from(self.pat[beg]);
                Atom::Chr
            } else {
                Atom::Set
            };
            self.next();
            self.expect(b']', error)?;
            Ok(atom)
        }

        fn class_ranges(&mut self) -> R {
            loop {
                let ret = self.class_atom()?;
                if ret == Atom::None {
                    return Ok(());
                }
                if self.is(b'-') {
                    self.next();
                    let first = self.val as u8;
                    let set_before = ret == Atom::Set;
                    let ret = self.class_atom()?;
                    if ret == Atom::None {
                        return Ok(());
                    }
                    if set_before || ret == Atom::Set {
                        return Err(RegexError::Range);
                    }
                    if (self.val as u8) < first {
                        return Err(RegexError::Range);
                    }
                }
            }
        }

        fn character_class(&mut self) -> R {
            if self.is(b'^') {
                self.next();
            }
            self.class_ranges()
        }

        fn wrapped_disjunction(&mut self) -> R<bool> {
            self.disj_count += 1;
            // 14.50: at most 999 groups open at once.
            if self.disj_count >= 1000 {
                return Err(RegexError::Stack);
            }
            // 14.50: `(?` followed by anything but `:`, `=` or `!` is error_badrepeat at once,
            // before a capture group is counted (so even past 999 groups it is not error_stack).
            let kind = if self.is(b'?') {
                self.pat.get(self.pos + 1).copied()
            } else {
                None
            };
            if self.is(b'?') && !matches!(kind, Some(b':' | b'=' | b'!')) {
                return Err(RegexError::Badrepeat);
            }
            if kind == Some(b':') {
                self.next();
                self.next();
                self.disjunction()?;
            } else if kind == Some(b'!') || kind == Some(b'=') {
                self.next();
                self.next();
                self.disjunction()?;
                self.disj_count -= 1;
                return Ok(false);
            } else {
                self.grp_idx += 1;
                if self.grp_idx >= 1000 {
                    return Err(RegexError::Stack);
                }
                let idx = self.grp_idx;
                self.disjunction()?;
                self.finished.resize(self.grp_idx + 1, false);
                self.finished[idx] = true;
            }
            self.disj_count -= 1;
            Ok(true)
        }

        fn identity_escape(&mut self) -> bool {
            if matches!(
                u8::try_from(self.ch),
                Ok(b'c' | b'd' | b'D' | b's' | b'S' | b'w' | b'W')
            ) {
                return false;
            }
            self.val = self.ch;
            self.next();
            true
        }

        fn character_escape(&mut self) -> R<bool> {
            if self.mchar == EOS {
                return Err(RegexError::Escape);
            }
            match u8::try_from(self.ch).unwrap_or(0) {
                c @ (b'f' | b'n' | b'r' | b't' | b'v') => {
                    // `_Do_ffn`: the control character is the value.
                    self.val = match c {
                        b'f' => 0x0c,
                        b'n' => 0x0a,
                        b'r' => 0x0d,
                        b't' => 0x09,
                        _ => 0x0b,
                    };
                    self.next();
                }
                b'c' => {
                    self.next();
                    let alpha = u8::try_from(self.ch).is_ok_and(|c| c.is_ascii_alphabetic());
                    if !alpha {
                        return Err(RegexError::Escape);
                    }
                    self.val = self.ch % 32;
                    self.next();
                }
                b'x' => {
                    self.next();
                    self.hex_digits(2)?;
                }
                b'u' => {
                    self.next();
                    self.hex_digits(4)?;
                }
                _ => return Ok(self.identity_escape()),
            }
            if self.val as u32 > 255 {
                return Err(RegexError::Escape);
            }
            Ok(true)
        }

        fn atom_escape(&mut self) -> R {
            // 14.50: `\0` is the NUL character unless a digit follows it.
            if self.ch == i32::from(b'0') {
                self.next();
                if value(self.ch, 10) != -1 {
                    return Err(RegexError::Escape);
                }
                return Ok(());
            }
            if self.decimal_digits(RegexError::Backref)? {
                let v = self.val as usize;
                if self.grp_idx < v || self.finished.len() <= v || !self.finished[v] {
                    return Err(RegexError::Backref);
                }
            } else if self.character_escape()? {
            } else if !self.class_escape() {
                return Err(RegexError::Escape);
            }
            Ok(())
        }

        fn quantifier(&mut self) -> R {
            if !self.is(b'*') {
                if self.is(b'+') || self.is(b'?') {
                } else if self.is(b'{') {
                    self.next();
                    if !self.decimal_digits(RegexError::Badbrace)? {
                        return Err(RegexError::Badbrace);
                    }
                    let min = self.val;
                    let mut max = -1;
                    if !self.is(b',') {
                        max = min;
                    } else {
                        self.next();
                        if !self.is(b'}') {
                            if !self.decimal_digits(RegexError::Badbrace)? {
                                return Err(RegexError::Badbrace);
                            }
                            max = self.val;
                        }
                    }
                    if !self.is(b'}') || (max != -1 && max < min) {
                        return Err(RegexError::Badbrace);
                    }
                } else {
                    return Ok(());
                }
            }
            self.next();
            if self.is(b'?') {
                self.next();
            }
            Ok(())
        }

        fn alternative(&mut self) -> R<bool> {
            let mut found = false;
            loop {
                let mut quant = true;
                if self.mchar == EOS || self.is(b'|') || (self.is(b')') && self.disj_count != 0) {
                    return Ok(found);
                } else if self.is(b')') {
                    return Err(RegexError::Paren);
                } else if self.is(b'.') {
                    self.next();
                } else if self.is(b'\\') {
                    self.next();
                    if self.ch == i32::from(b'b') || self.ch == i32::from(b'B') {
                        self.next();
                        quant = false;
                    } else {
                        self.atom_escape()?;
                    }
                } else if self.is(b'[') {
                    self.next();
                    self.character_class()?;
                    self.expect(b']', RegexError::Brack)?;
                } else if self.is(b'(') {
                    self.next();
                    quant = self.wrapped_disjunction()?;
                    self.expect(b')', RegexError::Paren)?;
                } else if self.is(b'^') || self.is(b'$') {
                    self.next();
                    quant = false;
                } else if self.is(b'*') || self.is(b'+') || self.is(b'?') || self.is(b'{') {
                    return Err(RegexError::Badrepeat);
                } else if self.is(b'}') {
                    return Err(RegexError::Brace);
                } else if self.is(b']') {
                    return Err(RegexError::Brack);
                } else {
                    self.next();
                }
                if quant {
                    self.quantifier()?;
                }
                found = true;
            }
        }

        fn disjunction(&mut self) -> R {
            if !self.alternative()? && !self.is(b'|') {
                return Ok(());
            }
            while self.is(b'|') {
                self.next();
                self.alternative()?;
            }
            Ok(())
        }
    }

    /// Whether `std::regex(pattern)` (ECMAScript) constructs, or the error it raises.
    pub fn check(pattern: &[u8]) -> Result<(), RegexError> {
        let mut p = Parser {
            pat: pattern,
            pos: 0,
            mchar: EOS,
            ch: EOS,
            val: 0,
            grp_idx: 0,
            finished: Vec::new(),
            disj_count: 0,
        };
        p.trans();
        p.disjunction()?;
        if p.pos != p.pat.len() {
            return Err(RegexError::Syntax);
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "string_tests.rs"]
mod tests;
