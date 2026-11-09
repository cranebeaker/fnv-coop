//! xNVSE's array commands (`Commands_Array.cpp`, with `ArrayVar.cpp` and
//! `ArrayVarElementContainer.cpp` (xNVSE) for what they call): making, reading, searching,
//! copying, sorting, editing and dumping array variables ([`crate::vars`]), and the commands that
//! call a user function on each element.
//!
//! - **Elements:** an element is set as `ArrayElement::Set` does: a string copied, a form by id,
//!   an array held by a reference from the holding array's owning mod.
//! - **Iteration:** the commands that call a function on each element walk the live array by
//!   position, as xNVSE's iterator does, so elements the function adds or erases count as they
//!   do there; the element is read afresh at each step (xNVSE's iterator can be left pointing at
//!   freed memory when the function grows the array).
//! - **Files:** `ar_DumpF` writes through the game's own `CreateFileA`, as every file the game
//!   makes ([`crate::saveload`] does the same): the platform's file layer decides where it goes.
//! - **The fixes preset** (docs/FIXES.md, "The script extender"; [`set_fix`]): vanilla is
//!   xNVSE's, its crashes included, except an outright hang or an allocation without end, which
//!   is capped in every preset: an `ar_Range` that never ends (a step of 0, or one that wraps
//!   past the end) stops at [`MAX_MADE`] elements (about 335 MB), as does `ar_Cat` of an array
//!   with itself. The rows: [`SIZE_CAPS`] caps every array these commands make or grow at
//!   [`MAX_MADE`] elements (xNVSE runs out of memory); [`NULL_GUARD`] answers 0 where xNVSE reads
//!   through a missing array (`ar_Count`, `ar_GetNth`) or a format string reads a string outside
//!   the game's memory; [`CYCLE_GUARD`] ends the walk of an array that holds itself (a deep copy,
//!   a deep compare, a dump), which overflows xNVSE's stack.
//! - **Out of memory:** where no cap comes first (vanilla's uncapped growth, or growth no row
//!   caps), an array these commands make or grow past [`OUT_OF_MEMORY_ELEMENTS`] (33,554,432
//!   elements) or a string past [`OUT_OF_MEMORY_BYTES`] (1 GiB) stops the game as out of memory
//!   ([`out_of_memory`]), about where xNVSE's 32-bit process runs out, so the host is never
//!   asked for more.
//!
//! Not here: lambdas' saved variables when an element takes or loses a lambda (as
//! [`crate::vars`]).

use super::console_print;
use super::string::{assign_to_string_var, dtoui3, form_representation, the_name, with_eval};
use crate::args::f64_to_i32;
use crate::expr::{self, Eval, Tok, Ty};
use crate::functions::{self, Caller};
use crate::table::{Command, Param, Returns};
use crate::vars::{self, Container, Data, Element, Key, Value, KEY_NUMBER, KEY_STRING};
use crate::{handled, CommandArgs, Nvse};
use engine_core::prelude::*;
use std::cmp::Ordering;
use std::sync::atomic::{AtomicBool, Ordering as Atomic};

/// The most elements an array made or grown here gets when it is capped.
pub const MAX_MADE: usize = 1 << 23;

/// The fixes preset's rows for these commands, set by the host's rows before the first frame
/// (`crates/host/src/fixes.rs`); never set (vanilla), they are off. Lock-free: every game thread
/// reads them.
#[derive(Default)]
pub struct CommandFixes {
    rows: [AtomicBool; 3],
}

/// `NV_NVSE_SIZE_CAPS`: arrays made or grown here stop at [`MAX_MADE`] elements, an empty text
/// replaced a given number of times at 64 MiB.
pub const SIZE_CAPS: usize = 0;
/// `NV_NVSE_NULL_GUARD`: a missing array or a string outside the game's memory answers 0 or ends
/// the text instead of stopping the game.
pub const NULL_GUARD: usize = 1;
/// `NV_NVSE_CYCLE_GUARD`: an array met again inside itself ends the walk.
pub const CYCLE_GUARD: usize = 2;

/// Turns one of the rows on or off (the host's fix rows; before the game's threads start).
pub fn set_fix(e: &mut Engine, row: usize, on: bool) {
    e.ext_free::<CommandFixes>().rows[row].store(on, Atomic::Relaxed);
}

/// Whether a row is on.
pub fn fix_on(e: &Engine, row: usize) -> bool {
    e.ext_get::<CommandFixes>()
        .is_some_and(|f| f.rows[row].load(Atomic::Relaxed))
}

/// The most elements an array these commands make or grow holds before the game stops as out
/// of memory (where no cap comes first: every preset, the size caps row's caps being lower).
/// xNVSE's array is a vector of 24-byte elements that doubles: past 2^25 elements it would
/// next take a 1.6 GB block, which a 32-bit process does not have, and the game stops there.
/// Ours is 1.3 GB at that point.
pub const OUT_OF_MEMORY_ELEMENTS: usize = 1 << 25;

/// The longest string these commands make before the game stops as out of memory: xNVSE's
/// `std::string` grows by half again, so past 1 GiB it would next take about 1.6 GB.
pub const OUT_OF_MEMORY_BYTES: usize = 1 << 30;

/// The out-of-memory stop: the game stops, as xNVSE's process does when an allocation fails.
pub(crate) fn out_of_memory(what: &str) -> ! {
    panic!("out of memory: {what}, more than xNVSE's 32-bit process can hold")
}

/// Stops the game when an array would hold more than [`OUT_OF_MEMORY_ELEMENTS`].
pub(crate) fn check_elements(count: usize) {
    if count > OUT_OF_MEMORY_ELEMENTS {
        out_of_memory(&format!("an array of {count} elements"));
    }
}

/// Stops the game when a string would be longer than [`OUT_OF_MEMORY_BYTES`].
pub(crate) fn check_bytes(count: usize) {
    if count > OUT_OF_MEMORY_BYTES {
        out_of_memory(&format!("a string of {count} bytes"));
    }
}

/// `s_arrayErrorCodeNum`, the bad numeric key.
const ERROR_NUM: f64 = -99999.0;

/// `LookupFormByID`.
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `kFormType_Script`.
const TYPE_SCRIPT: u8 = 0x11;
/// `Script::eVarType_Array`.
const VAR_TYPE_ARRAY: u8 = 3;

// ---- parameters ----

const STRING: u32 = 0x00;
const INT: u32 = 0x01;
const NV_NUMBER: u32 = 0x8001;
const NV_BOOLEAN: u32 = 0x8002;
const NV_STRING: u32 = 0x8004;
const NV_FORM: u32 = 0x8008;
const NV_ARRAY: u32 = 0x8040;
const NV_BASIC: u32 = 0x804d;
const NV_INDEX: u32 = 0x8005;
const NV_SLICE: u32 = 0x100;
const NV_PAIR: u32 = 0x20_0000;

const fn p(name: &'static str, kind: u32, optional: bool) -> Param {
    Param {
        name,
        kind,
        optional,
    }
}

const ONE_STRING: &[Param] = &[p("string", STRING, false)];
const ONE_INT: &[Param] = &[p("int", INT, false)];
const ONE_ARRAY: &[Param] = &[p("array", NV_ARRAY, false)];
const DUMP_F: &[Param] = &[
    p("array", NV_ARRAY, false),
    p("string", NV_STRING, false),
    p("bool", NV_BOOLEAN, true),
];
const ERASE: &[Param] = &[
    p("array", NV_ARRAY, false),
    p("index or range", NV_INDEX | NV_SLICE, true),
];
const SORT: &[Param] = &[
    p("array", NV_ARRAY, false),
    p("bDescending", NV_NUMBER, true),
];
const CUSTOM_SORT: &[Param] = &[
    p("array", NV_ARRAY, false),
    p("comparator", NV_FORM, false),
    p("bDescending", NV_NUMBER, true),
];
const FIND: &[Param] = &[
    p("valueToFind", NV_BASIC, false),
    p("array", NV_ARRAY, false),
    p("range", NV_SLICE, true),
];
const ARRAY_AND_KEY: &[Param] = &[
    p("array", NV_ARRAY, false),
    p("array index", NV_INDEX, false),
];
const RESIZE: &[Param] = &[
    p("array", NV_ARRAY, false),
    p("newSize", NV_NUMBER, false),
    p("padding", NV_BASIC, true),
];
const INSERT: &[Param] = &[
    p("array", NV_ARRAY, false),
    p("index", NV_NUMBER, false),
    p("toInsert", NV_BASIC, false),
];
const INSERT_RANGE: &[Param] = &[
    p("array", NV_ARRAY, false),
    p("index", NV_NUMBER, false),
    p("range", NV_ARRAY, false),
];
const APPEND: &[Param] = &[p("array", NV_ARRAY, false), p("toInsert", NV_BASIC, false)];
const LIST_PARAMS: [Param; 20] = [p("element", NV_BASIC, true); 20];
const PAIR_LIST: [Param; 20] = {
    let mut a = [p("key-value pair", NV_PAIR, true); 20];
    a[0] = p("key-value pair", NV_PAIR, false);
    a
};
const RANGE: &[Param] = &[
    p("start", NV_NUMBER, false),
    p("end", NV_NUMBER, false),
    p("step", NV_NUMBER, true),
];
const ARRAY_AND_FUNCTION: &[Param] = &[
    p("array", NV_ARRAY, false),
    p("condition user defined function", NV_FORM, false),
];
const GENERATE: &[Param] = &[
    p("int", NV_NUMBER, false),
    p("condition user defined function", NV_FORM, false),
    p("condition user defined function", NV_FORM, true),
];
const INIT: &[Param] = &[p("int", NV_NUMBER, false), p("element", NV_BASIC, false)];
const TWO_ARRAYS: &[Param] = &[p("array", NV_ARRAY, false), p("array", NV_ARRAY, false)];
const EXISTS: &[Param] = &[
    p("array", NV_ARRAY, false),
    p("toFind", NV_BASIC | NV_PAIR, false),
    p("range", NV_SLICE, true),
];
const COUNT: &[Param] = &[
    p("array", NV_ARRAY, false),
    p("toFind", NV_BASIC | NV_PAIR, false),
];
const GET_NTH: &[Param] = &[p("array", NV_ARRAY, false), p("index", NV_NUMBER, false)];
const CAT: &[Param] = &[
    p("array", NV_ARRAY, false),
    p("array", NV_ARRAY, false),
    p("int", NV_NUMBER, true),
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

use Returns::{Array as RA, ArrayIndex as RI, Default as RD, String as RS};

pub static COMMANDS: &[Command] = &[
    cmd(0x154f, "ar_Construct", "", "creates a new array", ONE_STRING, ar_construct, RA),
    cmd(0x1550, "ar_Size", "", "returns the size of an array or -1 if the array does not exist", ONE_ARRAY, ar_size, RD),
    cmd(0x1551, "ar_Dump", "", "dumps the contents of an array for debugging purposes.", ONE_ARRAY, ar_dump, RD),
    cmd(0x1552, "ar_DumpID", "", "dumps an array given an integer ID", ONE_INT, ar_dump_id, RD),
    cmd(0x1553, "ar_Erase", "", "erases an element or range of elements from an array", ERASE, ar_erase, RD),
    cmd(0x1554, "ar_Sort", "", "returns an array containing the source array elements in sorted order", SORT, ar_sort, RA),
    cmd(0x1555, "ar_CustomSort", "", "returns an array containing the source array elements sorted using the passed function script", CUSTOM_SORT, ar_custom_sort, RA),
    cmd(0x1556, "ar_SortAlpha", "", "returns an array containing the source array elements in alphabetical order", SORT, ar_sort_alpha, RA),
    cmd(0x1557, "ar_Find", "", "finds the first occurence of a value within an array", FIND, ar_find, RI),
    cmd(0x1558, "ar_First", "", "returns the key of the first element in an array", ONE_ARRAY, ar_first, RI),
    cmd(0x1559, "ar_Last", "", "returns the key of the last element in an array", ONE_ARRAY, ar_last, RI),
    cmd(0x155a, "ar_Next", "", "returns the key of the element immediately following the specified key", ARRAY_AND_KEY, ar_next, RI),
    cmd(0x155b, "ar_Prev", "", "returns the key of the element immediately preceding the specified key", ARRAY_AND_KEY, ar_prev, RI),
    cmd(0x155c, "ar_Keys", "", "returns an Array containing the keys of the specified array", ONE_ARRAY, ar_keys, RA),
    cmd(0x155d, "ar_HasKey", "", "returns 1 if the array contains an element with the specified key", ARRAY_AND_KEY, ar_has_key, RD),
    cmd(0x155e, "ar_BadStringIndex", "", "returns the return value of a bad string array index", &[], ar_bad_string_index, RS),
    cmd(0x155f, "ar_BadNumericIndex", "", "returns the return value of a bad numeric array index", &[], ar_bad_numeric_index, RD),
    cmd(0x1560, "ar_Copy", "", "returns a shallow copy of an array", ONE_ARRAY, ar_copy, RA),
    cmd(0x1561, "ar_DeepCopy", "", "returns a deep copy of an array copying any arrays contained in that array", ONE_ARRAY, ar_deep_copy, RA),
    cmd(0x1562, "ar_Null", "", "makes an array variable refer to no array", &[], ar_null, RA),
    cmd(0x1563, "ar_Resize", "", "resizes an array padding additional elements with zero or a supplied value", RESIZE, ar_resize, RD),
    cmd(0x1564, "ar_Insert", "", "inserts a single element in an array", INSERT, ar_insert, RD),
    cmd(0x1565, "ar_InsertRange", "", "inserts a range of elements into an array", INSERT_RANGE, ar_insert_range, RD),
    cmd(0x1566, "ar_Append", "", "appends an element to an Array", APPEND, ar_append, RD),
    cmd(0x1567, "ar_List", "", "creates an array given a list of elements", &LIST_PARAMS, ar_list, RA),
    cmd(0x1568, "ar_Map", "", "creates a Map or Stringmap given a list of key-value pairs", &PAIR_LIST, ar_map, RA),
    cmd(0x1569, "ar_Range", "", "returns an array containing numbers beginning with 'start' up to and including 'end' in intervals 'step'", RANGE, ar_range, RA),
    cmd(0x1604, "ar_Packed", "", "returns if the array is packed (array) or not (map)", ONE_ARRAY, ar_packed, RD),
    cmd(0x1625, "ar_DumpF", "", "dumps the contents of an array to a file", DUMP_F, ar_dump_f, RD),
    cmd(0x1627, "ar_FindWhere", "", "finds first element in array which satisfies a condition", ARRAY_AND_FUNCTION, ar_find_where, RI),
    cmd(0x1628, "ar_Filter", "", "filters an array on a condition", ARRAY_AND_FUNCTION, ar_filter, RA),
    cmd(0x1629, "ar_MapTo", "", "transforms an array into a new array from a script", ARRAY_AND_FUNCTION, ar_map_to, RA),
    cmd(0x162d, "ar_Generate", "", "creates a new array from a function called each time for each element", GENERATE, ar_generate, RA),
    cmd(0x162e, "ar_Init", "ar_Initialize", "creates a new array with x amount of elements, all set to a single value", INIT, ar_init, RA),
    cmd(0x162f, "ar_DeepEquals", "ar_DeepCompare", "checks if every element and sub-elements are identical between two arrays.", TWO_ARRAYS, ar_deep_equals, RD),
    cmd(0x1630, "ar_ForEach", "", "calls a function on each element of array", ARRAY_AND_FUNCTION, ar_for_each, RD),
    cmd(0x1631, "ar_Any", "", "checks if any of the elements in array satisfies a function script", ARRAY_AND_FUNCTION, ar_any, RD),
    cmd(0x1632, "ar_All", "", "checks if all of the elements in array satisfy a function script", ARRAY_AND_FUNCTION, ar_all, RD),
    cmd(0x1635, "ar_Unique", "", "returns a new array with no duplicate elements from the source array", ONE_ARRAY, ar_unique, RA),
    cmd(0x1671, "ar_Exists", "", "For packed arrays, check if value exists. Else (map), check if key exists.", EXISTS, ar_exists, RD),
    cmd(0x1672, "ar_Count", "", "Count how many times a given value is found inside of an array.", COUNT, ar_count, RD),
    cmd(0x1673, "ar_CountWhere", "", "Count how many times a value matches via a specified UDF.", ARRAY_AND_FUNCTION, ar_count_where, RD),
    cmd(0x1675, "ar_GetNth", "", "Get the nth element of an array, if it exists", GET_NTH, ar_get_nth, RD),
    cmd(0x167e, "ar_Cat", "pinto_Cat", "Concatenate two arrays", CAT, ar_cat, RD),
];

// ---- array helpers (`ArrayVar.cpp` (xNVSE)) ----

fn arrays(e: &mut Engine) -> &mut std::collections::HashMap<u32, vars::ArrayVar> {
    &mut e.ext::<Nvse>().vars.arrays
}

fn exists(e: &Engine, id: u32) -> bool {
    vars::array(e, id).is_some()
}

/// The array's keys and elements, in order, as they are now.
fn entries(e: &Engine, id: u32) -> Vec<(Key, Element)> {
    let Some(a) = vars::array(e, id) else {
        return Vec::new();
    };
    match &a.elements {
        Container::Array(v) => v
            .iter()
            .enumerate()
            .map(|(i, x)| (Key::Number(i as f64), x.clone()))
            .collect(),
        Container::NumMap(v) => v
            .iter()
            .map(|(k, x)| (Key::Number(*k), x.clone()))
            .collect(),
        Container::StrMap(v) => v
            .iter()
            .map(|(k, x)| (Key::Str(k.clone()), x.clone()))
            .collect(),
    }
}

/// The key and element at position `i` of the array as it is now (`None` past its end, or when
/// it is gone): the live walk of xNVSE's iterator.
fn nth_entry(e: &Engine, id: u32, i: usize) -> Option<(Key, Element)> {
    let a = vars::array(e, id)?;
    match &a.elements {
        Container::Array(v) => v.get(i).map(|x| (Key::Number(i as f64), x.clone())),
        Container::NumMap(v) => v.get(i).map(|(k, x)| (Key::Number(*k), x.clone())),
        Container::StrMap(v) => v.get(i).map(|(k, x)| (Key::Str(k.clone()), x.clone())),
    }
}

fn size(e: &Engine, id: u32) -> usize {
    vars::array(e, id).map_or(0, |a| a.len())
}

fn key_type(e: &Engine, id: u32) -> u8 {
    vars::array(e, id).map_or(0, |a| a.key_type)
}

fn is_packed(e: &Engine, id: u32) -> bool {
    vars::array(e, id).is_some_and(|a| a.packed)
}

/// `GetContainerType`: 0 array, 1 number map, 2 string map.
fn container_type(e: &Engine, id: u32) -> Option<u8> {
    vars::array(e, id).map(|a| match a.elements {
        Container::Array(_) => 0,
        Container::NumMap(_) => 1,
        Container::StrMap(_) => 2,
    })
}

fn script_mod(e: &Engine, script: u32) -> u8 {
    vars::script_mod_index(e, script)
}

fn create(e: &mut Engine, key_type: u8, packed: bool, modi: u8) -> u32 {
    vars::create_array(e, key_type, packed, modi)
}

fn value_of(data: &Data) -> Option<Value> {
    match data {
        Data::Invalid => None,
        Data::Number(n) => Some(Value::Number(*n)),
        Data::Form(f) => Some(Value::Form(*f)),
        Data::Str(s) => Some(Value::Str(s.clone())),
        Data::Array(a) => Some(Value::Array(*a)),
    }
}

/// The element at `key` of a container, for changing it in place.
fn element_mut<'a>(
    a: &'a mut vars::ArrayVar,
    upper: &[u8; 256],
    key: &Key,
) -> Option<&'a mut Element> {
    match (&mut a.elements, key) {
        (Container::Array(v), Key::Number(n)) => {
            let mut idx = f64_to_i32(*n);
            if idx < 0 {
                idx = idx.wrapping_add(v.len() as i32);
            }
            v.get_mut(idx as u32 as usize)
        }
        (Container::NumMap(v), Key::Number(n)) => {
            v.iter_mut().find(|(k, _)| k == n).map(|x| &mut x.1)
        }
        (Container::StrMap(v), Key::Str(s)) => v
            .iter_mut()
            .find(|(k, _)| vars::str_compare(upper, k, s) == Ordering::Equal)
            .map(|x| &mut x.1),
        _ => None,
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), ArrayVar.cpp: ArrayVar::SetElement and
// ArrayElement::Set.
/// The element at `key` (made when missing: a packed array appends) takes `data`; an invalid
/// value leaves the element there, invalid. False when the key does not fit the array.
fn set_data(e: &mut Engine, id: u32, key: &Key, data: &Data) -> bool {
    let len = size(e, id);
    if len >= OUT_OF_MEMORY_ELEMENTS && !has_key(e, id, key) {
        check_elements(len + 1);
    }
    match value_of(data) {
        Some(v) => vars::set_element(e, id, key, v),
        None => {
            if !vars::set_element(e, id, key, Value::Number(0.0)) {
                return false;
            }
            let upper = vars::upper_table(e);
            if let Some(el) = arrays(e)
                .get_mut(&id)
                .and_then(|a| element_mut(a, &upper, key))
            {
                el.data = Data::Invalid;
            }
            true
        }
    }
}

/// `ArrayElement::Set` into a new slot of array `id`: an array value is held from the array's
/// owning mod.
fn held(e: &mut Engine, id: u32, data: &Data) -> Data {
    match data {
        Data::Array(to) => {
            let owner = vars::array_owning_mod(e, id);
            let mut reference = 0;
            vars::add_reference(e, &mut reference, *to, owner);
            Data::Array(reference)
        }
        other => other.clone(),
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), ArrayVar.cpp: ArrayVar::Insert (one
// element).
fn insert_at(e: &mut Engine, id: u32, at: u32, data: &Data) -> bool {
    let len = match vars::array(e, id) {
        Some(a) if a.packed => a.len(),
        _ => return false,
    };
    if at as usize > len {
        return false;
    }
    check_elements(len + 1);
    let data = held(e, id, data);
    if let Some(Container::Array(v)) = arrays(e).get_mut(&id).map(|a| &mut a.elements) {
        v.insert(
            at as usize,
            Element {
                data,
                owning_array: id,
            },
        );
    }
    true
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), ArrayVar.cpp: ArrayVar::Insert (a range).
fn insert_range(e: &mut Engine, id: u32, at: u32, from: u32) -> bool {
    if !is_packed(e, id) || !is_packed(e, from) {
        return false;
    }
    if at as usize > size(e, id) {
        return false;
    }
    let source: Vec<Data> = entries(e, from).into_iter().map(|(_, x)| x.data).collect();
    check_elements(size(e, id) + source.len());
    let made: Vec<Element> = source
        .iter()
        .map(|d| Element {
            data: held(e, id, d),
            owning_array: id,
        })
        .collect();
    if let Some(Container::Array(v)) = arrays(e).get_mut(&id).map(|a| &mut a.elements) {
        let tail = v.split_off(at as usize);
        v.extend(made);
        v.extend(tail);
    }
    true
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), ArrayVarElementContainer.cpp:
// ArrayVarElementContainer::erase(UInt32, UInt32).
/// A packed array's elements `low` to `high` (both kept within it); a number map's keys from
/// `low` up to, not including, `high`. How many were erased.
fn erase_range(e: &mut Engine, id: u32, low: u32, high: u32) -> u32 {
    let Some(a) = arrays(e).get_mut(&id) else {
        return 0;
    };
    if a.is_empty() {
        return 0;
    }
    let removed: Vec<Element> = match &mut a.elements {
        Container::Array(v) => {
            let size = v.len() as u32;
            let high = high.min(size - 1);
            if low >= size || low > high {
                return 0;
            }
            v.drain(low as usize..=high as usize).collect()
        }
        Container::NumMap(v) => {
            let mut out = Vec::new();
            v.retain(|(k, x)| {
                let gone = k.fract() == 0.0 && *k >= f64::from(low) && *k < f64::from(high);
                if gone {
                    out.push(x.clone());
                }
                !gone
            });
            out
        }
        Container::StrMap(_) => return 0,
    };
    for x in &removed {
        vars::release_element(e, x);
    }
    removed.len() as u32
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), ArrayVar.cpp: ArrayElement::operator==.
fn data_eq(upper: &[u8; 256], a: &Data, b: &Data) -> bool {
    match (a, b) {
        (Data::Form(x), Data::Form(y)) => x == y,
        (Data::Str(x), Data::Str(y)) => vars::str_compare(upper, x, y) == Ordering::Equal,
        (Data::Array(x), Data::Array(y)) => x == y,
        (Data::Number(x), Data::Number(y)) => x == y,
        (Data::Invalid, Data::Invalid) => true,
        _ => false,
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), ArrayVar.cpp: ArrayElement::Equals.
fn data_equals(e: &mut Engine, a: &Data, b: &Data, deep: bool) -> bool {
    match (a, b) {
        (Data::Array(x), Data::Array(y)) => {
            let (l, r) = (exists(e, *x), exists(e, *y));
            if (!l && !r) || (l && r && x == y) {
                return true;
            }
            if !l || !r {
                return false;
            }
            compare_arrays(e, *x, *y, deep)
        }
        _ => {
            let upper = vars::upper_table(e);
            data_eq(&upper, a, b)
        }
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), ArrayVar.cpp: ArrayVar::CompareArrays.
fn compare_arrays(e: &mut Engine, a: u32, b: u32, deep: bool) -> bool {
    let guard = fix_on(e, CYCLE_GUARD);
    compare_in(e, a, b, deep, guard.then(Vec::new).as_mut())
}

/// [`compare_arrays`], with the pairs being compared when the cycle guard is on: a pair met again
/// inside itself counts as equal (without it the walk goes on until the stack runs out, as in
/// xNVSE).
fn compare_in(
    e: &mut Engine,
    a: u32,
    b: u32,
    deep: bool,
    path: Option<&mut Vec<(u32, u32)>>,
) -> bool {
    if size(e, a) != size(e, b) {
        return false;
    }
    if a == b {
        return true;
    }
    match path {
        Some(path) => {
            if path.contains(&(a, b)) {
                return true;
            }
            path.push((a, b));
            let equal = compare_entries(e, a, b, deep, Some(&mut *path));
            path.pop();
            equal
        }
        None => compare_entries(e, a, b, deep, None),
    }
}

fn compare_entries(
    e: &mut Engine,
    a: u32,
    b: u32,
    deep: bool,
    mut path: Option<&mut Vec<(u32, u32)>>,
) -> bool {
    let upper = vars::upper_table(e);
    for ((k1, x1), (k2, x2)) in entries(e, a).into_iter().zip(entries(e, b)) {
        let keys_equal = match (&k1, &k2) {
            (Key::Number(l), Key::Number(r)) => l == r,
            (Key::Str(l), Key::Str(r)) => vars::str_compare(&upper, l, r) == Ordering::Equal,
            _ => false,
        };
        if !keys_equal || x1.data.type_code() != x2.data.type_code() {
            return false;
        }
        if let (Data::Array(i1), Data::Array(i2), true) = (&x1.data, &x2.data, deep) {
            let (l, r) = (exists(e, *i1), exists(e, *i2));
            if l && r {
                if !compare_in(e, *i1, *i2, true, path.as_deref_mut()) {
                    return false;
                }
            } else if l || r {
                return false;
            }
        } else if !data_eq(&upper, &x1.data, &x2.data) {
            return false;
        }
    }
    true
}

/// `Slice`: numbers or strings, from its two keys.
fn slice_of(t: &Tok) -> Option<(Key, Key)> {
    match t {
        Tok::Slice { lower, upper } => Some((lower.clone(), upper.clone())),
        _ => None,
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), ArrayVar.cpp: ArrayVar::Find.
fn find(e: &mut Engine, id: u32, to_find: &Data, range: Option<&(Key, Key)>) -> Option<Key> {
    let upper = vars::upper_table(e);
    let a = vars::array(e, id)?;
    if a.is_empty() {
        return None;
    }
    let is_str = |r: &(Key, Key)| matches!(r.0, Key::Str(_));
    let num = |k: &Key| match k {
        Key::Number(n) => *n,
        Key::Str(_) => 0.0,
    };
    let text = |k: &Key| match k {
        Key::Str(s) => s.clone(),
        Key::Number(_) => Vec::new(),
    };
    match &a.elements {
        Container::Array(v) => {
            let size = v.len() as u32;
            let (low, mut high) = match range {
                Some(r) if is_str(r) => return None,
                Some(r) => (f64_to_i32(num(&r.0)) as u32, f64_to_i32(num(&r.1)) as u32),
                None => (0, size - 1),
            };
            if range.is_some() {
                if high >= size {
                    high = size - 1;
                }
                if low >= size || low > high {
                    return None;
                }
            }
            (low..=high)
                .find(|&i| data_eq(&upper, &v[i as usize].data, to_find))
                .map(|i| Key::Number(f64::from(i)))
        }
        Container::NumMap(v) => {
            if let Some(r) = range {
                if is_str(r) {
                    return None;
                }
                let (lo, hi) = (num(&r.0), num(&r.1));
                for (k, x) in v.iter().filter(|(k, _)| *k >= lo) {
                    if *k > hi {
                        return None;
                    }
                    if data_eq(&upper, &x.data, to_find) {
                        return Some(Key::Number(*k));
                    }
                }
                None
            } else {
                v.iter()
                    .find(|(_, x)| data_eq(&upper, &x.data, to_find))
                    .map(|(k, _)| Key::Number(*k))
            }
        }
        Container::StrMap(v) => {
            if let Some(r) = range {
                if !is_str(r) {
                    return None;
                }
                let (lo, hi) = (text(&r.0), text(&r.1));
                for (k, x) in v
                    .iter()
                    .filter(|(k, _)| vars::str_compare(&upper, k, &lo) != Ordering::Less)
                {
                    if vars::str_compare(&upper, k, &hi) == Ordering::Greater {
                        return None;
                    }
                    if data_eq(&upper, &x.data, to_find) {
                        return Some(Key::Str(k.clone()));
                    }
                }
                None
            } else {
                v.iter()
                    .find(|(_, x)| data_eq(&upper, &x.data, to_find))
                    .map(|(k, _)| Key::Str(k.clone()))
            }
        }
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), ArrayVar.cpp: ArrayVar::GetKeys.
fn keys_of(e: &mut Engine, id: u32, modi: u8) -> u32 {
    let out = create(e, KEY_NUMBER, true, modi);
    for (i, (k, _)) in entries(e, id).into_iter().enumerate() {
        let value = match k {
            Key::Number(n) => Value::Number(n),
            Key::Str(s) => Value::Str(s),
        };
        vars::set_element(e, out, &Key::Number(i as f64), value);
    }
    out
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), ArrayVar.cpp: ArrayVar::Copy.
fn copy(e: &mut Engine, id: u32, modi: u8, deep: bool) -> u32 {
    let guard = fix_on(e, CYCLE_GUARD);
    copy_in(e, id, modi, deep, guard.then(Vec::new).as_mut())
}

/// [`copy`], with the arrays being copied and their copies when the cycle guard is on: an array
/// met again inside itself is held as its copy (without it the copy goes on until the stack runs
/// out, as in xNVSE).
fn copy_in(
    e: &mut Engine,
    id: u32,
    modi: u8,
    deep: bool,
    mut path: Option<&mut Vec<(u32, u32)>>,
) -> u32 {
    let (kt, packed) = vars::array(e, id).map_or((KEY_NUMBER, true), |a| (a.key_type, a.packed));
    let out = create(e, kt, packed, modi);
    if let Some(p) = path.as_deref_mut() {
        p.push((id, out));
    }
    for (k, x) in entries(e, id) {
        if let (Data::Array(inner), true) = (&x.data, deep) {
            if exists(e, *inner) {
                let made = path
                    .as_deref()
                    .and_then(|p| p.iter().find(|(from, _)| from == inner).map(|&(_, to)| to));
                let inner_copy = match made {
                    Some(to) => to,
                    None => copy_in(e, *inner, modi, true, path.as_deref_mut()),
                };
                vars::set_element(e, out, &k, Value::Array(inner_copy));
            }
        } else {
            set_data(e, out, &k, &x.data);
        }
    }
    if let Some(p) = path {
        p.pop();
    }
    out
}

/// `BasicTokenToElem`: the token as a number, a string, a form or an array, in that order, after
/// `ToBasicToken`; `None` when it is none of them.
fn basic_elem(e: &mut Engine, ev: &Eval, t: &Tok) -> Option<Data> {
    let b = functions::to_basic(e, ev, t)?;
    as_elem(e, ev, &b)
}

/// `ScriptToken::GetAsArrayElement`: number, string, form, array; else invalid (`None`).
fn as_elem(e: &mut Engine, ev: &Eval, t: &Tok) -> Option<Data> {
    if ev.can_convert(e, t, Ty::Number) {
        Some(Data::Number(ev.get_number(e, t)))
    } else if ev.can_convert(e, t, Ty::String) {
        Some(Data::Str(ev.get_string(e, t)))
    } else if ev.can_convert(e, t, Ty::Form) {
        Some(Data::Form(ev.get_form_id(e, t)))
    } else if ev.can_convert(e, t, Ty::Array) {
        Some(Data::Array(ev.get_array_id(e, t)))
    } else {
        None
    }
}

/// `ScriptToken::GetArrayVar`: the token's array, if it exists.
fn array_arg(e: &mut Engine, ev: &Eval, t: &Tok) -> Option<u32> {
    let id = ev.get_array_id(e, t);
    exists(e, id).then_some(id)
}

/// `ScriptToken::GetUserFunction`: the token's form when it is a script.
fn user_function(e: &mut Engine, ev: &Eval, t: &Tok) -> Option<u32> {
    let form = ev.get_form(e, t);
    (form != 0 && e.mem.u8(form + 4) == TYPE_SCRIPT).then_some(form)
}

fn number_key(e: &mut Engine, ev: &Eval, t: &Tok) -> Option<f64> {
    ev.can_convert(e, t, Ty::Number)
        .then(|| ev.get_number(e, t))
}

fn string_key(e: &mut Engine, ev: &Eval, t: &Tok) -> Option<Vec<u8>> {
    ev.can_convert(e, t, Ty::String)
        .then(|| ev.get_string(e, t))
}

/// An expression command's run: the result zeroed, then `f` with the evaluator and the
/// arguments (`None` when they could not be read).
fn run(
    e: &mut Engine,
    w: &[u32],
    f: impl FnOnce(&mut Engine, &CommandArgs, &mut Eval, Option<Vec<Tok>>),
) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    with_eval(e, &a, |e, ev, toks| f(e, &a, ev, toks));
    handled(true)
}

// ---- commands ----

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Array.cpp:
// Cmd_ar_Construct_Execute.
fn ar_construct(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    if !expr::active(e) {
        let id = if a.script == 0 {
            0
        } else {
            e.mem.u32(a.script + 0xc)
        };
        say(
            e,
            "Error in script %08X: ar_Construct called outside of a valid context.",
            &[id],
        );
        return handled(true);
    }
    let read = super::Read::of(e, &a);
    if !read.ok {
        return handled(true);
    }
    let kind = read.text(0);
    let upper = vars::upper_table(e);
    let (mut key, mut packed) = (KEY_NUMBER, false);
    if vars::str_compare(&upper, &kind, b"StringMap") == Ordering::Equal {
        key = KEY_STRING;
    } else if vars::str_compare(&upper, &kind, b"Map") != Ordering::Equal {
        packed = true;
    }
    let modi = script_mod(e, a.script);
    let id = create(e, key, packed, modi);
    a.set_result(e, f64::from(id as i32));
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Array.cpp: Cmd_ar_Size_Execute,
// Cmd_ar_Packed_Execute.
fn ar_size(e: &mut Engine, w: &[u32]) -> Ret {
    run(e, w, |e, a, ev, toks| {
        a.set_result(e, -1.0);
        if let Some(t) = toks.as_ref().and_then(|t| t.first()) {
            if ev.can_convert(e, t, Ty::Array) {
                if let Some(id) = array_arg(e, ev, t) {
                    a.set_result(e, f64::from(size(e, id) as i32));
                }
            }
        }
    })
}

fn ar_packed(e: &mut Engine, w: &[u32]) -> Ret {
    run(e, w, |e, a, ev, toks| {
        if let Some(t) = toks.as_ref().and_then(|t| t.first()) {
            if ev.can_convert(e, t, Ty::Array) {
                if let Some(id) = array_arg(e, ev, t) {
                    if is_packed(e, id) {
                        a.set_result(e, 1.0);
                    }
                }
            }
        }
    })
}

/// `%f` of the C runtime.
fn fixed(e: &Engine, v: f64) -> Vec<u8> {
    super::string::ucrt_format(e, b"%f", &v.to_le_bytes())
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), ArrayVar.cpp: ArrayElement::
// GetStringRepresentation, ArrayVar::GetStringRepresentation.
fn element_repr(e: &mut Engine, data: &Data, path: Option<&mut Vec<u32>>) -> Vec<u8> {
    match data {
        Data::Invalid => b"invalid".to_vec(),
        Data::Number(n) => expr::format_g(*n).into_bytes(),
        Data::Form(id) => {
            let form = e.call(LOOKUP_FORM, &args![*id]).u32();
            if form == 0 {
                b"null".to_vec()
            } else {
                form_representation(e, form)
            }
        }
        Data::Str(s) => {
            let mut out = vec![b'"'];
            out.extend_from_slice(s);
            out.push(b'"');
            out
        }
        Data::Array(id) => {
            if exists(e, *id) {
                array_repr_in(e, *id, path)
            } else {
                b"invalid array".to_vec()
            }
        }
    }
}

fn array_repr(e: &mut Engine, id: u32) -> Vec<u8> {
    let guard = fix_on(e, CYCLE_GUARD);
    array_repr_in(e, id, guard.then(Vec::new).as_mut())
}

/// [`array_repr`], with the arrays being written when the cycle guard is on: an array met again
/// inside itself is written `[...]` (without it the text goes on until the stack runs out, as in
/// xNVSE).
fn array_repr_in(e: &mut Engine, id: u32, mut path: Option<&mut Vec<u32>>) -> Vec<u8> {
    if let Some(p) = path.as_deref_mut() {
        if p.contains(&id) {
            return b"[...]".to_vec();
        }
        p.push(id);
    }
    let list = entries(e, id);
    let numbered = matches!(
        vars::array(e, id).map(|a| &a.elements),
        Some(Container::NumMap(_))
    );
    let mut out = vec![b'['];
    let n = list.len();
    for (i, (k, x)) in list.into_iter().enumerate() {
        match k {
            Key::Number(v) if numbered => {
                out.extend(fixed(e, v));
                out.extend_from_slice(b": ");
            }
            Key::Str(s) => {
                out.push(b'"');
                out.extend(s);
                out.extend_from_slice(b"\": ");
            }
            Key::Number(_) => {}
        }
        out.extend(element_repr(e, &x.data, path.as_deref_mut()));
        if i != n - 1 {
            out.extend_from_slice(b", ");
        }
    }
    out.push(b']');
    if let Some(p) = path {
        p.pop();
    }
    out
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), ArrayVar.cpp: ArrayVar::Dump.
/// The lines `ar_Dump` prints.
fn dump_lines(e: &mut Engine, id: u32) -> Vec<Vec<u8>> {
    let Some((owner, refs, kt)) =
        vars::array(e, id).map(|a| (a.owning_mod, a.refs.len(), a.key_type))
    else {
        return Vec::new();
    };
    let mod_name = crate::errors::nth_mod_name(e, u32::from(owner));
    let mut head =
        format!("** Dumping Array #{id} **\nRefs: {refs} Owner {owner:02X}: ").into_bytes();
    head.extend(mod_name);
    let mut lines = vec![head];
    for (k, x) in entries(e, id) {
        let mut line = b"[ ".to_vec();
        match (kt, k) {
            (KEY_NUMBER, Key::Number(v)) => line.extend(fixed(e, v)),
            (KEY_STRING, Key::Str(s)) => line.extend(s),
            _ => line.extend_from_slice(b"?Unknown Key Type?"),
        }
        line.extend_from_slice(b" ] : ");
        match &x.data {
            Data::Number(v) => line.extend(fixed(e, *v)),
            Data::Str(s) => line.extend_from_slice(s),
            Data::Array(inner) => {
                if exists(e, *inner) {
                    line.extend(array_repr(e, *inner));
                } else {
                    line.extend_from_slice(b"NULL Array");
                }
            }
            Data::Form(f) => {
                let form = e.call(LOOKUP_FORM, &args![*f]).u32();
                if form != 0 {
                    line.extend(super::string::full_name(e, form));
                } else {
                    line.extend_from_slice(b"<?NAME?>");
                }
                line.extend_from_slice(format!(" ({f:08X})").as_bytes());
            }
            Data::Invalid => line.extend_from_slice(b"?Unknown Element Type?"),
        }
        lines.push(line);
    }
    lines
}

/// `Console_Print_Str`: a line of up to 511 bytes in one print, a longer one in pieces.
fn print_line(e: &mut Engine, text: &[u8]) {
    if Nvse::prints(e) {
        crate::errors::print_long(e, text);
    }
}

/// `Console_Print`: to the console once xNVSE prints (from the first frame on).
fn say(e: &mut Engine, format: &str, words: &[u32]) {
    if Nvse::prints(e) {
        console_print(e, format, words);
    }
}

fn dump(e: &mut Engine, id: u32) {
    for line in dump_lines(e, id) {
        print_line(e, &line);
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Array.cpp: Cmd_ar_Dump_Execute.
fn ar_dump(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    with_eval(e, &a, |e, ev, toks| {
        if let Some(t) = toks.as_ref().and_then(|t| t.first()) {
            if ev.can_convert(e, t, Ty::Array) {
                if let Some(id) = array_arg(e, ev, t) {
                    dump(e, id);
                }
            }
        }
    });
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Array.cpp: Cmd_ar_DumpID_Execute.
fn ar_dump_id(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    let read = super::Read::of(e, &a);
    if read.ok {
        let id = read.int(0, 0);
        if !exists(e, id) {
            say(e, "Array does not exist.", &[]);
        } else if size(e, id) == 0 {
            say(e, "Array is empty.", &[]);
        } else {
            dump(e, id);
        }
    }
    handled(true)
}

/// KERNEL32's import slots the game's own file code calls (`CreateFileA`, `SetFilePointer`,
/// `WriteFile`, `CloseHandle`): what goes through them reaches only where the platform's file
/// layer lets it.
const CREATE_FILE_A: u32 = 0x00fd_f088;
const SET_FILE_POINTER: u32 = 0x00fd_f240;
const WRITE_FILE: u32 = 0x00fd_f0dc;
const CLOSE_HANDLE: u32 = 0x00fd_f07c;
const GENERIC_READ_WRITE: u32 = 0xc000_0000;
const CREATE_ALWAYS: u32 = 2;
const OPEN_ALWAYS: u32 = 4;
const FILE_ATTRIBUTE_NORMAL: u32 = 0x80;
const FILE_END: u32 = 2;
const INVALID_HANDLE_VALUE: u32 = 0xffff_ffff;

/// `fopen_s(path, "a+"/"w+")`, the text and `fclose`, through the game's file calls: `"w+"`
/// makes the file anew, `"a+"` opens or makes it and writes at its end. False when it cannot be
/// opened.
fn write_text_file(e: &mut Engine, path: &[u8], append: bool, text: &[u8]) -> bool {
    let disposition = if append { OPEN_ALWAYS } else { CREATE_ALWAYS };
    let handle = crate::events::with_guest_text(e, path, |e, p| {
        e.call(
            CREATE_FILE_A,
            &args![
                p,
                GENERIC_READ_WRITE,
                0u32,
                0u32,
                disposition,
                FILE_ATTRIBUTE_NORMAL,
                0u32
            ],
        )
        .u32()
    });
    if handle == INVALID_HANDLE_VALUE {
        return false;
    }
    if append {
        e.call(SET_FILE_POINTER, &args![handle, 0u32, 0u32, FILE_END]);
    }
    e.with_stack(0x1004, |e, frame| {
        let (buf, written) = (frame.addr(), frame.addr() + 0x1000);
        for piece in text.chunks(0x1000) {
            e.mem.write(buf, piece);
            e.call(
                WRITE_FILE,
                &args![handle, buf, piece.len() as u32, written, 0u32],
            );
        }
    });
    e.call(CLOSE_HANDLE, &args![handle]);
    true
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Array.cpp: Cmd_ar_DumpF_Execute,
// ArrayVar.cpp: ArrayVar::DumpToFile.
/// The dump's lines to a text file (`fopen_s` `"a+"` or `"w+"`, text mode: a new line is
/// `\r\n`), through the game's own file calls; a file that cannot be opened is passed over.
fn ar_dump_f(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    with_eval(e, &a, |e, ev, toks| {
        let Some(toks) = toks else {
            return;
        };
        let (Some(arr_tok), Some(path_tok)) = (toks.first(), toks.get(1)) else {
            return;
        };
        if !ev.can_convert(e, arr_tok, Ty::Array) || !ev.can_convert(e, path_tok, Ty::String) {
            return;
        }
        let id = array_arg(e, ev, arr_tok);
        let append = match toks.get(2) {
            Some(t) if ev.can_convert(e, t, Ty::Boolean) => ev.get_bool(e, t),
            _ => true,
        };
        let path = ev.get_string(e, path_tok);
        let Some(id) = id else {
            return;
        };
        let mut text = Vec::new();
        for line in dump_lines(e, id) {
            for b in line {
                if b == b'\n' {
                    text.extend_from_slice(b"\r\n");
                } else {
                    text.push(b);
                }
            }
            text.extend_from_slice(b"\r\n");
        }
        let path = super::string::c_str(&path).to_vec();
        write_text_file(e, &path, append, &text);
    });
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Array.cpp: Cmd_ar_Erase_Execute.
/// How many were erased, or `(UInt32)-1` (4294967295) for an error.
fn ar_erase(e: &mut Engine, w: &[u32]) -> Ret {
    run(e, w, |e, a, ev, toks| {
        let mut erased = u32::MAX;
        if let Some(toks) = toks {
            if let Some(t) = toks.first().filter(|t| ev.can_convert(e, t, Ty::Array)) {
                if let Some(id) = array_arg(e, ev, t) {
                    match toks.get(1) {
                        Some(which) => {
                            if let Some((lo, hi)) = slice_of(which) {
                                let empty = size(e, id) == 0;
                                erased = match (lo, hi) {
                                    (Key::Number(l), Key::Number(h)) if !empty => erase_range(
                                        e,
                                        id,
                                        f64_to_i32(l) as u32,
                                        f64_to_i32(h) as u32,
                                    ),
                                    _ => u32::MAX,
                                };
                            } else if let Some(s) = string_key(e, ev, which) {
                                erased = vars::erase_element(e, id, &Key::Str(s));
                            } else if let Some(n) = number_key(e, ev, which) {
                                erased = vars::erase_element(e, id, &Key::Number(n));
                            }
                        }
                        None => erased = vars::erase_all(e, id),
                    }
                }
            }
        }
        a.set_result(e, f64::from(erased));
    })
}

#[derive(Clone, Copy, PartialEq)]
enum SortKind {
    Default,
    Alpha,
    Function,
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), ArrayVar.cpp: ArrayElement::operator<.
fn data_less(upper: &[u8; 256], a: &Data, b: &Data) -> bool {
    match (a, b) {
        (Data::Form(x), Data::Form(y)) | (Data::Array(x), Data::Array(y)) => x < y,
        (Data::Str(x), Data::Str(y)) => vars::str_compare(upper, x, y) == Ordering::Less,
        (Data::Number(x), Data::Number(y)) => x < y,
        _ => false,
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), ArrayVar.cpp: ArrayElement::CompareNames.
fn names_less(e: &mut Engine, upper: &[u8; 256], a: &Data, b: &Data) -> bool {
    let (Data::Form(l), Data::Form(r)) = (a, b) else {
        return false;
    };
    let lform = e.call(LOOKUP_FORM, &args![*l]).u32();
    if lform != 0 {
        let rform = e.call(LOOKUP_FORM, &args![*r]).u32();
        if rform != 0 {
            let lname = the_name(e, lform);
            if !lname.is_empty() {
                let rname = the_name(e, rform);
                if !rname.is_empty() {
                    return vars::str_compare(upper, &lname, &rname) == Ordering::Less;
                }
            }
        }
    }
    l < r
}

/// `Vector::InsertSorted`: after the last element `item` is not "less" than, by halving.
fn insert_sorted(
    e: &mut Engine,
    out: &mut Vec<Data>,
    item: Data,
    less: &mut dyn FnMut(&mut Engine, &Data, &Data) -> bool,
) {
    let (mut lo, mut hi) = (0usize, out.len());
    while lo != hi {
        let mid = (lo + hi) >> 1;
        if less(e, &item, &out[mid]) {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    out.insert(lo, item);
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), ArrayVar.cpp: ArrayVar::Sort and
// SortFunctionCaller.
/// Fills `result` (a packed array) with `src`'s elements of the first element's type, sorted.
fn sort(
    e: &mut Engine,
    src: u32,
    result: u32,
    descending: bool,
    mut kind: SortKind,
    comparator: u32,
) {
    let list = entries(e, src);
    let Some(first) = list.first() else {
        return;
    };
    let ty = first.1.data.type_code();
    if ty == 0 || ty == 4 {
        return;
    }
    if kind == SortKind::Alpha && ty != 2 {
        kind = SortKind::Default;
    }
    let upper = vars::upper_table(e);
    let mut out: Vec<Data> = Vec::new();
    match kind {
        SortKind::Default => {
            let mut less =
                |_: &mut Engine, a: &Data, b: &Data| data_less(&upper, a, b) != descending;
            for (_, x) in list.iter().filter(|(_, x)| x.data.type_code() == ty) {
                insert_sorted(e, &mut out, x.data.clone(), &mut less);
            }
        }
        SortKind::Alpha => {
            let mut less =
                |e: &mut Engine, a: &Data, b: &Data| names_less(e, &upper, a, b) != descending;
            for (_, x) in list.iter().filter(|(_, x)| matches!(x.data, Data::Form(_))) {
                insert_sorted(e, &mut out, x.data.clone(), &mut less);
            }
        }
        SortKind::Function => {
            if comparator == 0 {
                return;
            }
            let modi = script_mod(e, comparator);
            let lhs = create(e, KEY_NUMBER, true, modi);
            let rhs = create(e, KEY_NUMBER, true, modi);
            let takes_two_arrays = functions::param_types(e, comparator)
                .is_some_and(|t| t == [VAR_TYPE_ARRAY, VAR_TYPE_ARRAY]);
            let mut less = |e: &mut Engine, a: &Data, b: &Data| {
                set_data(e, lhs, &Key::Number(0.0), a);
                set_data(e, rhs, &Key::Number(0.0), b);
                if !takes_two_arrays {
                    return false;
                }
                let caller = Caller::internal(comparator, 0, 0, vec![lhs, rhs]);
                match functions::call(e, caller) {
                    Some(t) => truth(&t) != descending,
                    None => false,
                }
            };
            for (_, x) in list.iter().filter(|(_, x)| x.data.type_code() == ty) {
                insert_sorted(e, &mut out, x.data.clone(), &mut less);
            }
        }
    }
    for (i, d) in out.iter().enumerate() {
        set_data(e, result, &Key::Number(i as f64), d);
    }
}

fn sort_execute(e: &mut Engine, w: &[u32], kind: SortKind) -> Ret {
    let a = CommandArgs::new(w);
    let modi = script_mod(e, a.script);
    let sorted = create(e, KEY_NUMBER, true, modi);
    a.set_result(e, f64::from(sorted));
    with_eval(e, &a, |e, ev, toks| {
        let Some(toks) = toks else {
            return;
        };
        if kind == SortKind::Function {
            if toks.len() < 2 {
                return;
            }
        } else if !toks
            .first()
            .is_some_and(|t| ev.can_convert(e, t, Ty::Array))
        {
            return;
        }
        let Some(src) = array_arg(e, ev, &toks[0]).filter(|&id| size(e, id) > 0) else {
            return;
        };
        let mut comparator = 0;
        let descending = if kind == SortKind::Function {
            let form = ev.get_form(e, &toks[1]);
            comparator = if form != 0 && e.mem.u8(form + 4) == TYPE_SCRIPT {
                form
            } else {
                0
            };
            if comparator == 0 {
                return;
            }
            toks.get(2).is_some_and(|t| ev.get_bool(e, t))
        } else {
            toks.get(1).is_some_and(|t| ev.get_number(e, t) != 0.0)
        };
        sort(e, src, sorted, descending, kind, comparator);
    });
    handled(true)
}

fn ar_sort(e: &mut Engine, w: &[u32]) -> Ret {
    sort_execute(e, w, SortKind::Default)
}
fn ar_custom_sort(e: &mut Engine, w: &[u32]) -> Ret {
    sort_execute(e, w, SortKind::Function)
}
fn ar_sort_alpha(e: &mut Engine, w: &[u32]) -> Ret {
    sort_execute(e, w, SortKind::Alpha)
}

/// A string result of the running script's file, temporary (`g_StringMap.Add(..., true)`).
fn temp_string(e: &mut Engine, a: &CommandArgs, text: &[u8]) -> f64 {
    let modi = script_mod(e, a.script);
    f64::from(vars::add_string(e, modi, text, true))
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Array.cpp: Cmd_ar_Find_Execute.
fn ar_find(e: &mut Engine, w: &[u32]) -> Ret {
    run(e, w, |e, a, ev, toks| {
        let Some(toks) = toks.filter(|t| t.len() >= 2) else {
            return;
        };
        if !ev.can_convert(e, &toks[1], Ty::Array) {
            return;
        }
        let to_find = basic_elem(e, ev, &toks[0]);
        let arr = array_arg(e, ev, &toks[1]);
        let kt = arr.map_or(0, |id| key_type(e, id));
        if kt == KEY_STRING {
            expr::expect_return_type(e, Returns::String);
            let id = temp_string(e, a, b"");
            a.set_result(e, id);
        } else {
            expr::expect_return_type(e, Returns::Default);
            a.set_result(e, ERROR_NUM);
        }
        let (Some(arr), Some(to_find)) = (arr, to_find) else {
            return;
        };
        let range = toks.get(2).and_then(slice_of);
        match find(e, arr, &to_find, range.as_ref()) {
            Some(Key::Number(n)) if kt == KEY_NUMBER => a.set_result(e, n),
            Some(Key::Str(s)) if kt != KEY_NUMBER => {
                let id = temp_string(e, a, &s);
                a.set_result(e, id);
            }
            Some(Key::Number(n)) => {
                let id = temp_string(e, a, format!("{n}").as_bytes());
                a.set_result(e, id);
            }
            _ => {}
        }
    })
}

#[derive(Clone, Copy, PartialEq)]
enum Iter {
    First,
    Last,
    Next,
    Prev,
}

/// The key next to (`step` 1) or before (`-1`) `key`, as the container's iterator finds it.
fn neighbour(e: &mut Engine, id: u32, key: &Key, step: isize) -> Option<Key> {
    let upper = vars::upper_table(e);
    let list = entries(e, id);
    let at = match (vars::array(e, id).map(|a| &a.elements)?, key) {
        (Container::Array(v), Key::Number(n)) => {
            let i = f64_to_i32(*n) as u32 as usize;
            (i < v.len()).then_some(i)?
        }
        (Container::NumMap(_), Key::Number(n)) => {
            list.iter().position(|(k, _)| *k == Key::Number(*n))?
        }
        (Container::StrMap(_), Key::Str(s)) => list.iter().position(|(k, _)| match k {
            Key::Str(t) => vars::str_compare(&upper, t, s) == Ordering::Equal,
            Key::Number(_) => false,
        })?,
        _ => return None,
    };
    let to = at.checked_add_signed(step)?;
    list.get(to).map(|(k, _)| k.clone())
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Array.cpp: ArrayIterCommand.
fn iter_command(e: &mut Engine, w: &[u32], mode: Iter) -> Ret {
    run(e, w, |e, a, ev, toks| {
        let toks = toks.filter(|t| !t.is_empty() && ev.can_convert(e, &t[0], Ty::Array));
        let Some(toks) = toks else {
            ev.error_text(
                "Invalid array access in ar_First/Last. Is the array initialized?".into(),
            );
            return;
        };
        let Some(id) = array_arg(e, ev, &toks[0]) else {
            return;
        };
        let list = entries(e, id);
        let found = |e: &mut Engine, ev: &Eval, numeric: bool| -> Option<Key> {
            match mode {
                Iter::First => list.first().map(|(k, _)| k.clone()),
                Iter::Last => list.last().map(|(k, _)| k.clone()),
                Iter::Next | Iter::Prev => {
                    let t = toks.get(1).filter(|_| toks.len() == 2)?;
                    let key = if numeric {
                        Key::Number(number_key(e, ev, t)?)
                    } else {
                        Key::Str(string_key(e, ev, t)?)
                    };
                    neighbour(e, id, &key, if mode == Iter::Next { 1 } else { -1 })
                }
            }
        };
        match key_type(e, id) {
            KEY_NUMBER => {
                a.set_result(e, ERROR_NUM);
                expr::expect_return_type(e, Returns::Default);
                if let Some(Key::Number(n)) = found(e, ev, true) {
                    a.set_result(e, n);
                }
            }
            KEY_STRING => {
                expr::expect_return_type(e, Returns::String);
                let key = match found(e, ev, false) {
                    Some(Key::Str(s)) => s,
                    _ => Vec::new(),
                };
                assign_to_string_var(e, a, &key);
            }
            _ => ev.error_text(
                "Invalid array key type in ar_First/Last. Check that the array has been initialized."
                    .into(),
            ),
        }
    })
}

fn ar_first(e: &mut Engine, w: &[u32]) -> Ret {
    iter_command(e, w, Iter::First)
}
fn ar_last(e: &mut Engine, w: &[u32]) -> Ret {
    iter_command(e, w, Iter::Last)
}
fn ar_next(e: &mut Engine, w: &[u32]) -> Ret {
    iter_command(e, w, Iter::Next)
}
fn ar_prev(e: &mut Engine, w: &[u32]) -> Ret {
    iter_command(e, w, Iter::Prev)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Array.cpp: Cmd_ar_Keys_Execute.
fn ar_keys(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    if !expr::active(e) {
        crate::errors::show(
            e,
            a.script,
            "ar_Keys must be called within an NVSE expression.",
        );
        return handled(true);
    }
    with_eval(e, &a, |e, ev, toks| {
        if let Some(t) = toks.as_ref().and_then(|t| t.first()) {
            if ev.can_convert(e, t, Ty::Array) {
                if let Some(id) = array_arg(e, ev, t) {
                    let modi = script_mod(e, a.script);
                    let keys = keys_of(e, id, modi);
                    a.set_result(e, f64::from(keys));
                }
            }
        }
    });
    handled(true)
}

/// `ArrayVar::HasKey`: whether `Get(key, false)` finds it (a packed array counts negative indexes
/// from the end).
fn has_key(e: &mut Engine, id: u32, key: &Key) -> bool {
    vars::get_element(e, id, key).is_some()
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Array.cpp: Cmd_ar_HasKey_Execute.
fn ar_has_key(e: &mut Engine, w: &[u32]) -> Ret {
    run(e, w, |e, a, ev, toks| {
        let Some(toks) = toks.filter(|t| t.len() == 2) else {
            return;
        };
        if !ev.can_convert(e, &toks[0], Ty::Array) {
            return;
        }
        let Some(id) = array_arg(e, ev, &toks[0]) else {
            return;
        };
        let found = if key_type(e, id) == KEY_NUMBER {
            number_key(e, ev, &toks[1]).map(|n| has_key(e, id, &Key::Number(n)))
        } else {
            string_key(e, ev, &toks[1]).map(|s| has_key(e, id, &Key::Str(s)))
        };
        if let Some(found) = found {
            a.set_result(e, f64::from(u8::from(found)));
        }
    })
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Array.cpp:
// Cmd_ar_BadStringIndex_Execute, Cmd_ar_BadNumericIndex_Execute, Cmd_ar_Null_Execute.
fn ar_bad_string_index(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    assign_to_string_var(e, &a, b"");
    handled(true)
}

fn ar_bad_numeric_index(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, ERROR_NUM);
    handled(true)
}

fn ar_null(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Array.cpp: ArrayCopyCommand.
fn copy_command(e: &mut Engine, w: &[u32], deep: bool) -> Ret {
    run(e, w, |e, a, ev, toks| {
        if let Some([t]) = toks.as_deref() {
            if ev.can_convert(e, t, Ty::Array) {
                if let Some(id) = array_arg(e, ev, t) {
                    let modi = script_mod(e, a.script);
                    let out = copy(e, id, modi, deep);
                    a.set_result(e, f64::from(out));
                }
            }
        }
    })
}

fn ar_copy(e: &mut Engine, w: &[u32]) -> Ret {
    copy_command(e, w, false)
}
fn ar_deep_copy(e: &mut Engine, w: &[u32]) -> Ret {
    copy_command(e, w, true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), ArrayVar.cpp: ArrayVar::SetSize.
fn set_size(e: &mut Engine, id: u32, new_size: u32, pad: &Data) -> bool {
    if !is_packed(e, id) {
        return false;
    }
    let len = size(e, id) as u32;
    if len < new_size {
        let mut grow = (new_size - len) as usize;
        if fix_on(e, SIZE_CAPS) {
            grow = grow.min(MAX_MADE.saturating_sub(len as usize));
        }
        // xNVSE resizes its vector at once.
        check_elements(len as usize + grow);
        for i in 0..grow {
            set_data(e, id, &Key::Number(f64::from(len) + i as f64), pad);
        }
        true
    } else if len > new_size {
        erase_range(e, id, new_size, len - 1) > 0
    } else {
        true
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Array.cpp: Cmd_ar_Resize_Execute.
fn ar_resize(e: &mut Engine, w: &[u32]) -> Ret {
    run(e, w, |e, a, ev, toks| {
        let Some(toks) = toks.filter(|t| t.len() >= 2) else {
            return;
        };
        if !ev.can_convert(e, &toks[0], Ty::Array) || !ev.can_convert(e, &toks[1], Ty::Number) {
            return;
        }
        let Some(id) = array_arg(e, ev, &toks[0]) else {
            return;
        };
        let pad = if toks.len() == 3 {
            basic_elem(e, ev, &toks[2]).unwrap_or_default()
        } else {
            Data::Number(0.0)
        };
        let new_size = f64_to_i32(ev.get_number(e, &toks[1])) as u32;
        let ok = set_size(e, id, new_size, &pad);
        a.set_result(e, f64::from(u8::from(ok)));
    })
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Array.cpp: Cmd_ar_Append_Execute.
fn ar_append(e: &mut Engine, w: &[u32]) -> Ret {
    run(e, w, |e, a, ev, toks| {
        let Some(toks) = toks.filter(|t| t.len() == 2) else {
            return;
        };
        if !ev.can_convert(e, &toks[0], Ty::Array) {
            return;
        }
        let Some(id) = array_arg(e, ev, &toks[0]) else {
            return;
        };
        if let Some(d) = basic_elem(e, ev, &toks[1]) {
            let at = size(e, id) as u32;
            let ok = insert_at(e, id, at, &d);
            a.set_result(e, f64::from(u8::from(ok)));
        }
    })
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Array.cpp: ar_Insert_Execute.
fn insert_command(e: &mut Engine, w: &[u32], range: bool) -> Ret {
    run(e, w, |e, a, ev, toks| {
        let Some(toks) = toks.filter(|t| t.len() == 3) else {
            return;
        };
        if !ev.can_convert(e, &toks[0], Ty::Array) || !ev.can_convert(e, &toks[1], Ty::Number) {
            return;
        }
        let Some(id) = array_arg(e, ev, &toks[0]) else {
            return;
        };
        let at = f64_to_i32(ev.get_number(e, &toks[1])) as u32;
        let ok = if range {
            if !ev.can_convert(e, &toks[2], Ty::Array) {
                return;
            }
            let from = ev.get_array_id(e, &toks[2]);
            insert_range(e, id, at, from)
        } else {
            let Some(d) = basic_elem(e, ev, &toks[2]) else {
                return;
            };
            insert_at(e, id, at, &d)
        };
        a.set_result(e, f64::from(u8::from(ok)));
    })
}

fn ar_insert(e: &mut Engine, w: &[u32]) -> Ret {
    insert_command(e, w, false)
}
fn ar_insert_range(e: &mut Engine, w: &[u32]) -> Ret {
    insert_command(e, w, true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Array.cpp: Cmd_ar_List_Execute.
fn ar_list(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    let modi = script_mod(e, a.script);
    let arr = create(e, KEY_NUMBER, true, modi);
    a.set_result(e, f64::from(arr as i32));
    with_eval(e, &a, |e, ev, toks| {
        for (i, t) in toks.unwrap_or_default().iter().enumerate() {
            match basic_elem(e, ev, t) {
                Some(d) => {
                    set_data(e, arr, &Key::Number(i as f64), &d);
                }
                None => break,
            }
        }
    });
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Array.cpp: Cmd_ar_Map_Execute.
fn ar_map(e: &mut Engine, w: &[u32]) -> Ret {
    run(e, w, |e, a, ev, toks| {
        let Some(toks) = toks.filter(|t| !t.is_empty()) else {
            return;
        };
        let Tok::Pair(left, _) = &toks[0] else {
            return;
        };
        let kt = if ev.can_convert(e, left, Ty::Number) {
            KEY_NUMBER
        } else if ev.can_convert(e, left, Ty::String) {
            KEY_STRING
        } else {
            return;
        };
        let modi = script_mod(e, a.script);
        let arr = create(e, kt, false, modi);
        a.set_result(e, f64::from(arr as i32));
        for t in &toks {
            let Tok::Pair(left, right) = t else {
                continue;
            };
            let Some(d) = basic_elem(e, ev, right) else {
                continue;
            };
            if kt == KEY_STRING {
                let key = ev.get_string(e, left);
                set_data(e, arr, &Key::Str(key), &d);
            } else if let Some(n) = number_key(e, ev, left) {
                set_data(e, arr, &Key::Number(n), &d);
            }
        }
    })
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Array.cpp: Cmd_ar_Range_Execute.
fn ar_range(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    let modi = script_mod(e, a.script);
    let arr = create(e, KEY_NUMBER, true, modi);
    a.set_result(e, f64::from(arr as i32));
    with_eval(e, &a, |e, ev, toks| {
        let Some(toks) = toks.filter(|t| t.len() >= 2) else {
            return;
        };
        let mut start = f64_to_i32(ev.get_number(e, &toks[0]));
        let end = f64_to_i32(ev.get_number(e, &toks[1]));
        let step = if toks.len() == 3 {
            f64_to_i32(ev.get_number(e, &toks[2]))
        } else {
            1
        };
        // A step of 0, or one that wraps past the end, never ends in xNVSE: capped in every
        // preset. Any other range is capped with the fixes preset's row only.
        let mut endless = step == 0;
        let caps = fix_on(e, SIZE_CAPS);
        let mut made = Vec::new();
        while !((step > 0 && start > end) || (step < 0 && start < end)) {
            if (endless || caps) && made.len() >= MAX_MADE {
                break;
            }
            check_elements(made.len() + 1);
            made.push(Element {
                data: Data::Number(f64::from(start)),
                owning_array: arr,
            });
            endless |= start.checked_add(step).is_none();
            start = start.wrapping_add(step);
        }
        if let Some(Container::Array(v)) = arrays(e).get_mut(&arr).map(|a| &mut a.elements) {
            v.extend(made);
        }
    });
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Array.cpp: ElementToIterator.
/// The `{"key": key, "value": value}` string map a function gets for one element.
fn element_iterator(e: &mut Engine, script: u32, key: &Key, x: &Element) -> u32 {
    let modi = script_mod(e, script);
    let it = create(e, KEY_STRING, false, modi);
    let k = match key {
        Key::Str(s) => Value::Str(s.clone()),
        Key::Number(n) => Value::Number(*n),
    };
    vars::set_element(e, it, &Key::Str(b"key".to_vec()), k);
    set_data(e, it, &Key::Str(b"value".to_vec()), &x.data);
    it
}

/// `InternalFunctionCaller` with the element's iterator array as the one argument.
fn call_on(e: &mut Engine, a: &CommandArgs, function: u32, key: &Key, x: &Element) -> Option<Tok> {
    let it = element_iterator(e, a.script, key, x);
    functions::call(
        e,
        Caller::internal(function, a.this_obj, a.containing_obj, vec![it]),
    )
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Array.cpp: ExtractArrayUDF.
fn array_and_function(e: &mut Engine, ev: &Eval, toks: Option<Vec<Tok>>) -> Option<(u32, u32)> {
    let toks = toks.filter(|t| t.len() == 2)?;
    let arr = array_arg(e, ev, &toks[0]);
    let function = user_function(e, ev, &toks[1]);
    Some((arr?, function?))
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Array.cpp: ReturnElement.
fn return_element(e: &mut Engine, a: &CommandArgs, data: &Data) {
    match data {
        Data::Number(n) => {
            expr::expect_return_type(e, Returns::Default);
            a.set_result(e, *n);
        }
        Data::Form(f) => {
            expr::expect_return_type(e, Returns::Form);
            a.set_form_result(e, *f);
        }
        Data::Str(s) => {
            expr::expect_return_type(e, Returns::String);
            assign_to_string_var(e, a, s);
        }
        Data::Array(id) => {
            expr::expect_return_type(e, Returns::Array);
            let id = if *id == 0 || exists(e, *id) { *id } else { 0 };
            a.set_result(e, f64::from(id));
        }
        Data::Invalid => a.set_result(e, 0.0),
    }
}

/// A function's result (a plain token, `ToBasicToken`): `GetBool`.
fn truth(t: &Tok) -> bool {
    match t {
        Tok::Number { v, .. } => *v != 0.0,
        Tok::Bool(b) => *b,
        Tok::Str(s) => !s.is_empty(),
        Tok::Form(f) => *f != 0,
        Tok::Array(a) => *a != 0,
        _ => false,
    }
}

/// A function's result: `GetNumber` (0 for anything but a number).
fn truth_number(t: &Tok) -> bool {
    match t {
        Tok::Number { v, .. } => *v != 0.0,
        Tok::Bool(b) => *b,
        _ => false,
    }
}

/// `BasicTokenToElem` of a plain token.
fn basic_data(t: &Tok) -> Option<Data> {
    match t {
        Tok::Number { v, .. } => Some(Data::Number(*v)),
        Tok::Bool(b) => Some(Data::Number(f64::from(u8::from(*b)))),
        Tok::Str(s) => Some(Data::Str(s.clone())),
        Tok::Form(f) => Some(Data::Form(*f)),
        Tok::Array(a) => Some(Data::Array(*a)),
        _ => None,
    }
}

/// xNVSE's walk of an array (`ArrayIterator`, `++iter` until `End()`): position by position
/// against its size as it is now, each element read afresh, so what the function adds or erases
/// counts as it does there; `f` answers false to stop.
fn walk(e: &mut Engine, arr: u32, mut f: impl FnMut(&mut Engine, Key, Element) -> bool) {
    let mut i = 0;
    while let Some((k, x)) = nth_entry(e, arr, i) {
        if !f(e, k, x) {
            return;
        }
        i += 1;
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Array.cpp:
// Cmd_ar_FindWhere_Execute.
fn ar_find_where(e: &mut Engine, w: &[u32]) -> Ret {
    run(e, w, |e, a, ev, toks| {
        let Some((arr, function)) = array_and_function(e, ev, toks) else {
            return;
        };
        walk(e, arr, |e, k, x| {
            let Some(t) = call_on(e, a, function, &k, &x) else {
                return true;
            };
            if truth(&t) {
                return_element(e, a, &x.data);
                return false;
            }
            true
        });
    })
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Array.cpp: Cmd_ar_Filter_Execute,
// Cmd_ar_MapTo_Execute.
fn filter_or_map(e: &mut Engine, w: &[u32], map: bool) -> Ret {
    run(e, w, |e, a, ev, toks| {
        let Some((arr, function)) = array_and_function(e, ev, toks) else {
            return;
        };
        let (kt, packed) = (key_type(e, arr), is_packed(e, arr));
        let modi = script_mod(e, a.script);
        let out = create(e, kt, packed, modi);
        walk(e, arr, |e, k, x| {
            let Some(t) = call_on(e, a, function, &k, &x) else {
                return true;
            };
            if map {
                if let Some(d) = basic_data(&t) {
                    set_data(e, out, &k, &d);
                }
            } else if truth(&t) {
                set_data(e, out, &k, &x.data);
            }
            true
        });
        a.set_result(e, f64::from(out));
    })
}

fn ar_filter(e: &mut Engine, w: &[u32]) -> Ret {
    filter_or_map(e, w, false)
}
fn ar_map_to(e: &mut Engine, w: &[u32]) -> Ret {
    filter_or_map(e, w, true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Array.cpp:
// Cmd_ar_ForEach_Execute.
fn ar_for_each(e: &mut Engine, w: &[u32]) -> Ret {
    run(e, w, |e, a, ev, toks| {
        let Some((arr, function)) = array_and_function(e, ev, toks) else {
            return;
        };
        walk(e, arr, |e, k, x| {
            call_on(e, a, function, &k, &x);
            true
        });
        a.set_result(e, 1.0);
    })
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Array.cpp: Cmd_ar_Any_Execute,
// Cmd_ar_All_Execute.
fn any_or_all(e: &mut Engine, w: &[u32], all: bool) -> Ret {
    run(e, w, |e, a, ev, toks| {
        let Some((arr, function)) = array_and_function(e, ev, toks) else {
            return;
        };
        let mut answer = all;
        walk(e, arr, |e, k, x| {
            let t = call_on(e, a, function, &k, &x);
            match (t, all) {
                (None, false) => true,
                (None, true) => {
                    answer = false;
                    false
                }
                (Some(t), false) => {
                    if truth_number(&t) {
                        answer = true;
                        return false;
                    }
                    true
                }
                (Some(t), true) => {
                    if !truth_number(&t) {
                        answer = false;
                        return false;
                    }
                    true
                }
            }
        });
        if answer {
            a.set_result(e, 1.0);
        }
    })
}

fn ar_any(e: &mut Engine, w: &[u32]) -> Ret {
    any_or_all(e, w, false)
}
fn ar_all(e: &mut Engine, w: &[u32]) -> Ret {
    any_or_all(e, w, true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Array.cpp:
// Cmd_ar_CountWhere_Execute (it also makes an array it never fills, as xNVSE does).
fn ar_count_where(e: &mut Engine, w: &[u32]) -> Ret {
    run(e, w, |e, a, ev, toks| {
        let Some((arr, function)) = array_and_function(e, ev, toks) else {
            return;
        };
        let (kt, packed) = (key_type(e, arr), is_packed(e, arr));
        let modi = script_mod(e, a.script);
        create(e, kt, packed, modi);
        let mut count = 0.0;
        walk(e, arr, |e, k, x| {
            if let Some(t) = call_on(e, a, function, &k, &x) {
                if truth(&t) {
                    count += 1.0;
                }
            }
            true
        });
        a.set_result(e, count);
    })
}

/// A function called with no arguments, its result a basic element.
fn generated(e: &mut Engine, a: &CommandArgs, function: u32) -> Option<Option<Data>> {
    let t = functions::call(
        e,
        Caller::internal(function, a.this_obj, a.containing_obj, Vec::new()),
    )?;
    Some(basic_data(&t))
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Array.cpp:
// Cmd_ar_Generate_Execute.
fn ar_generate(e: &mut Engine, w: &[u32]) -> Ret {
    run(e, w, |e, a, ev, toks| {
        let Some(toks) = toks.filter(|t| t.len() >= 2) else {
            return;
        };
        let count = dtoui3(ev.get_number(e, &toks[0]));
        let value_fn = user_function(e, ev, &toks[1]);
        let key_fn = if toks.len() == 3 {
            user_function(e, ev, &toks[2])
        } else {
            None
        };
        let Some(value_fn) = value_fn.filter(|_| count != 0) else {
            return;
        };
        let modi = script_mod(e, a.script);
        let mut out = match key_fn {
            None => Some(create(e, KEY_NUMBER, true, modi)),
            Some(_) => None,
        };
        let limit = if fix_on(e, SIZE_CAPS) {
            count.min(MAX_MADE as u32)
        } else {
            count
        };
        for i in 0..limit {
            let Some(value) = generated(e, a, value_fn) else {
                continue;
            };
            let Some(value) = value else {
                continue;
            };
            match key_fn {
                Some(key_fn) => {
                    let Some(key) = generated(e, a, key_fn) else {
                        continue;
                    };
                    let Some(key) = key else {
                        continue;
                    };
                    let kt = match key {
                        Data::Number(_) => KEY_NUMBER,
                        Data::Str(_) => KEY_STRING,
                        _ => return,
                    };
                    if i == 0 && out.is_none() {
                        out = Some(create(e, kt, false, modi));
                    }
                    let Some(arr) = out else {
                        return;
                    };
                    match key {
                        Data::Str(s) => {
                            set_data(e, arr, &Key::Str(s), &value);
                        }
                        Data::Number(n) => {
                            set_data(e, arr, &Key::Number(n), &value);
                        }
                        _ => {}
                    }
                }
                None => {
                    if let Some(arr) = out {
                        insert_at(e, arr, i, &value);
                    }
                }
            }
        }
        if let Some(arr) = out {
            a.set_result(e, f64::from(arr));
        }
    })
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Array.cpp: Cmd_ar_Init_Execute.
fn ar_init(e: &mut Engine, w: &[u32]) -> Ret {
    run(e, w, |e, a, ev, toks| {
        let Some(toks) = toks.filter(|t| t.len() == 2) else {
            return;
        };
        let count = dtoui3(ev.get_number(e, &toks[0]));
        if count == 0 {
            return;
        }
        let Some(d) = basic_elem(e, ev, &toks[1]) else {
            return;
        };
        let modi = script_mod(e, a.script);
        let out = create(e, KEY_NUMBER, true, modi);
        let limit = if fix_on(e, SIZE_CAPS) {
            count.min(MAX_MADE as u32)
        } else {
            count
        };
        for i in 0..limit {
            insert_at(e, out, i, &d);
        }
        a.set_result(e, f64::from(out));
    })
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Array.cpp:
// Cmd_ar_DeepEquals_Execute.
fn ar_deep_equals(e: &mut Engine, w: &[u32]) -> Ret {
    run(e, w, |e, a, ev, toks| {
        let Some(toks) = toks.filter(|t| t.len() == 2) else {
            return;
        };
        let (l, r) = (
            ev.can_convert(e, &toks[0], Ty::Array),
            ev.can_convert(e, &toks[1], Ty::Array),
        );
        if l && r {
            let (x, y) = (array_arg(e, ev, &toks[0]), array_arg(e, ev, &toks[1]));
            match (x, y) {
                (Some(x), Some(y)) => {
                    let eq = compare_arrays(e, x, y, true);
                    a.set_result(e, f64::from(u8::from(eq)));
                }
                (None, None) => a.set_result(e, 1.0),
                _ => {}
            }
        } else if !l && !r {
            a.set_result(e, 1.0);
        }
    })
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Array.cpp: Cmd_ar_Unique_Execute.
fn ar_unique(e: &mut Engine, w: &[u32]) -> Ret {
    run(e, w, |e, a, ev, toks| {
        let Some([t]) = toks.as_deref() else {
            return;
        };
        if !ev.can_convert(e, t, Ty::Array) {
            return;
        }
        let Some(src) = array_arg(e, ev, t) else {
            return;
        };
        let (kt, packed) = (key_type(e, src), is_packed(e, src));
        let modi = script_mod(e, a.script);
        let out = create(e, kt, packed, modi);
        for (k, x) in entries(e, src) {
            if find(e, out, &x.data, None).is_none() {
                set_data(e, out, &k, &x.data);
            }
        }
        a.set_result(e, f64::from(out));
    })
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Array.cpp: Cmd_ar_Exists_Execute.
fn ar_exists(e: &mut Engine, w: &[u32]) -> Ret {
    run(e, w, |e, a, ev, toks| {
        let Some(toks) = toks.filter(|t| !t.is_empty()) else {
            return;
        };
        if !ev.can_convert(e, &toks[0], Ty::Array) {
            return;
        }
        let Some(id) = array_arg(e, ev, &toks[0]) else {
            return;
        };
        let Some(check) = toks.get(1) else {
            return;
        };
        let upper = vars::upper_table(e);
        if let Tok::Pair(left, right) = check {
            let key = if key_type(e, id) == KEY_NUMBER && ev.can_convert(e, left, Ty::Number) {
                Some(Key::Number(ev.get_number(e, left)))
            } else if ev.can_convert(e, left, Ty::String) {
                Some(Key::Str(ev.get_string(e, left)))
            } else {
                None
            };
            match key {
                Some(key) => {
                    let value = as_elem(e, ev, right).unwrap_or_default();
                    let at = vars::get_element(e, id, &key);
                    let found = at.is_some_and(|x| data_eq(&upper, &x.data, &value));
                    a.set_result(e, f64::from(u8::from(found)));
                }
                None => ev.error_text(
                    "ar_Exists >> Invalid type of key from passed pair (doesn't match key type for array)."
                        .into(),
                ),
            }
        } else if is_packed(e, id) {
            let Some(value) = as_elem(e, ev, check) else {
                return;
            };
            let range = toks.get(2).and_then(slice_of);
            let found = find(e, id, &value, range.as_ref()).is_some();
            a.set_result(e, f64::from(u8::from(found)));
        } else if let Some(s) = string_key(e, ev, check) {
            let found = has_key(e, id, &Key::Str(s));
            a.set_result(e, f64::from(u8::from(found)));
        } else if let Some(n) = number_key(e, ev, check) {
            let found = has_key(e, id, &Key::Number(n));
            a.set_result(e, f64::from(u8::from(found)));
        } else {
            ev.error_text("ar_Exists >> Invalid type of key to find for the map array".into());
        }
    })
}

/// What xNVSE does with a missing array it goes on to use: it reads the null `ArrayVar`'s word at
/// `offset` (an access violation: the game stops), which the port does too; under the fixes
/// preset's row ([`NULL_GUARD`]) the command answers 0 instead.
fn missing_array(e: &mut Engine, offset: u32) {
    if !fix_on(e, NULL_GUARD) {
        let _ = e.mem.u32(offset);
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Array.cpp: Cmd_ar_Count_Execute.
fn ar_count(e: &mut Engine, w: &[u32]) -> Ret {
    run(e, w, |e, a, ev, toks| {
        let Some(toks) = toks.filter(|t| t.len() == 2) else {
            return;
        };
        if !ev.can_convert(e, &toks[0], Ty::Array) {
            return;
        }
        let Some(id) = array_arg(e, ev, &toks[0]) else {
            // `arr->Begin()`: the container's type word.
            missing_array(e, 0xc);
            return;
        };
        let value = as_elem(e, ev, &toks[1]).unwrap_or_default();
        let mut count = 0.0;
        for (_, x) in entries(e, id) {
            if data_equals(e, &x.data, &value, false) {
                count += 1.0;
            }
        }
        a.set_result(e, count);
    })
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Array.cpp: Cmd_ar_GetNth_Execute.
fn ar_get_nth(e: &mut Engine, w: &[u32]) -> Ret {
    run(e, w, |e, a, ev, toks| {
        let Some(toks) = toks.filter(|t| t.len() == 2) else {
            return;
        };
        let Some(id) = array_arg(e, ev, &toks[0]) else {
            // `arr->Size()`: the container's item count.
            missing_array(e, 4);
            return;
        };
        let idx = f64_to_i32(ev.get_number(e, &toks[1]));
        let list = entries(e, id);
        if idx as u32 as usize >= list.len() {
            return;
        }
        return_element(e, a, &list[idx as usize].1.data);
    })
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Array.cpp: Cmd_ar_Cat_Execute.
fn ar_cat(e: &mut Engine, w: &[u32]) -> Ret {
    run(e, w, |e, _, ev, toks| {
        let Some(toks) = toks.filter(|t| t.len() >= 2) else {
            return;
        };
        let (Some(x), Some(y)) = (array_arg(e, ev, &toks[0]), array_arg(e, ev, &toks[1])) else {
            return;
        };
        let overwrite = toks.len() >= 3 && ev.get_bool(e, &toks[2]);
        let kind = container_type(e, x);
        if kind != container_type(e, y) {
            return;
        }
        // The live walk of the second array; an array added to itself grows under its own walk,
        // which never ends in xNVSE: capped at [`MAX_MADE`] elements in every preset.
        walk(e, y, |e, k, el| {
            if kind == Some(0) {
                let at = size(e, x);
                if x == y && at >= MAX_MADE {
                    return false;
                }
                insert_at(e, x, at as u32, &el.data);
            } else if !has_key(e, x, &k) || overwrite {
                set_data(e, x, &k, &el.data);
            }
            true
        });
    })
}

#[cfg(test)]
#[path = "array_tests.rs"]
mod tests;
