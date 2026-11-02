//! `fallout shared/tesparameters.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The parameters of a script function as a condition item stores them
//! (`Data.FunctionData`, Xbox PDB: the function number as a 16-bit word at
//! +0, the parameter words from +4): their count, which are forms, and the
//! resolution of the form ids after loading (`TESParameters::InitItem`).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function table's parameter count (`01190922`: the 16-bit word at
/// +0x12 of the 0x28-byte entry per function number).
const FUNCTION_PARAM_COUNT: u32 = 0x0119_0922;
/// The function table's parameter list (`01190924`: the word at +0x14 of
/// the entry, the address of 0xc bytes per parameter; +4 is the
/// parameter's type).
const FUNCTION_PARAM_LIST: u32 = 0x0119_0924;
const FUNCTION_ENTRY_SIZE: u32 = 0x28;
const PARAM_ENTRY_SIZE: u32 = 0xc;
const PARAM_ENTRY_TYPE: u32 = 4;
/// The parameter type table (`0118cdd0`, 8 bytes per type: +0 a word that
/// `InitItem` compares with 0xb, +5 a byte that is nonzero for a type that
/// is a form).
const PARAM_TYPE_TABLE: u32 = 0x0118_cdd0;
const PARAM_TYPE_ENTRY_SIZE: u32 = 8;
const PARAM_TYPE_IS_FORM: u32 = 5;
/// The parameter type whose form `InitItem` casts to `MagicItem`.
const PARAM_TYPE_MAGIC_ITEM: u32 = 0xb;
/// The parameter type `00682380` tests for (0x16).
const PARAM_TYPE_0X16: u32 = 0x16;
/// The function number (0x198, compared as `number + 0x1000 == 0x1198`)
/// whose form parameters `00594d60` decides.
const FUNCTION_0X198_PLUS_0X1000: u32 = 0x1198;
/// `00594d60` (cdecl: function data, index): whether that parameter of
/// function 0x198 is a form.
const PARAM_IS_FORM_0X198: u32 = 0x0059_4d60;
/// `__RTDynamicCast` (`00ec43fb`, cdecl: object, vfptr offset, source type,
/// target type, reference flag).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// `TESForm`'s type descriptor (`01183028`, `.?AVTESForm@@`).
const TYPE_TES_FORM: u32 = 0x0118_3028;
/// `TESScriptableForm`'s type descriptor (`01183254`,
/// `.?AVTESScriptableForm@@`).
const TYPE_TES_SCRIPTABLE_FORM: u32 = 0x0118_3254;
/// `TESObjectREFR`'s type descriptor (`011841cc`, `.?AVTESObjectREFR@@`).
const TYPE_TES_OBJECT_REFR: u32 = 0x0118_41cc;
/// `MagicItem`'s type descriptor (`01183140`, `.?AVMagicItem@@`).
const TYPE_MAGIC_ITEM: u32 = 0x0118_3140;
/// `00726070` (`ECX` = object): the word at +4 (on a `TESScriptableForm`
/// part, its script).
const WORD_AT_FOUR: u32 = 0x0072_6070;
/// `007af430` (`ECX` = object): the word at +0x20 (on a reference, its base
/// object).
const WORD_AT_0X20: u32 = 0x007a_f430;
/// `TESScriptableForm::GetFormScript` (`004826d0`, cdecl: form).
const GET_FORM_SCRIPT: u32 = 0x0048_26d0;
/// `TESForm::GetFile` (`00484e60`, `ECX` = form, one word: an index).
const GET_FILE: u32 = 0x0048_4e60;
/// The `GetFile` index word the exe passes (-1).
const ANY_FILE: u32 = 0xffff_ffff;
/// `TESForm::AddCompileIndex` (`00485d50`, cdecl: address of a form id,
/// file).
const ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// `004839c0` (cdecl: form id): the form with that id.
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `Script::GetVariableName` (`005abed0`, `ECX` = script, one word: the
/// variable id; `RET 4`).
const GET_VARIABLE_NAME: u32 = 0x005a_bed0;
/// The `BSString` constructor (`004037b0`, `ECX` = string).
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
/// The `BSString` destructor (`004037d0`, `ECX` = string).
const STRING_DESTRUCT: u32 = 0x0040_37d0;
/// The size of a `BSString` on the stack.
const STRING_SIZE: u32 = 8;
/// `00559450` (`ECX` = string): the string's characters.
const STRING_CHARS: u32 = 0x0055_9450;
/// The owner's virtual slot 0x90 (one word: a string to describe it in).
const SLOT_DESCRIBE_0X90: u32 = 0x90;
/// The script's virtual slot 0x130: a string (its name in the warning).
const SLOT_NAME_0X130: u32 = 0x130;
/// `005b5e40` (cdecl: format, arguments...): writes a warning line.
const LOG_WARNING: u32 = 0x005b_5e40;
/// `"MASTERFILE: Unable to find Function Info TESForm (%08X) in
/// TESConditionItem Parameter Init for %s."` (`01069608`).
const FORMAT_NO_FORM: u32 = 0x0106_9608;
/// `"MASTERFILE: Unable to find Function Info MagicItem in TESConditionItem
/// Parameter Init for %s."` (`010695a8`).
const FORMAT_NO_MAGIC_ITEM: u32 = 0x0106_95a8;
/// `"MASTERFILE: Unable to find variableID %d on script '%s' in
/// TESConditionItem Parameter Init for %s."` (`01069540`).
const FORMAT_NO_VARIABLE: u32 = 0x0106_9540;

/// The address of parameter `index` of the function data.
fn param_slot(data: Ptr, index: u32) -> u32 {
    data.addr()
        .wrapping_add(4)
        .wrapping_add(index.wrapping_mul(4))
}

/// The type word of parameter `index` of the function data's function.
fn param_type(e: &Engine, data: Ptr, index: u32) -> u32 {
    let number = e.mem.u16(data.addr()) as u32;
    let list = e
        .mem
        .u32(FUNCTION_PARAM_LIST.wrapping_add(number.wrapping_mul(FUNCTION_ENTRY_SIZE)));
    e.mem.u32(
        list.wrapping_add(index.wrapping_mul(PARAM_ENTRY_SIZE))
            .wrapping_add(PARAM_ENTRY_TYPE),
    )
}

/// Writes one `InitItem` warning: a string the owner describes itself in
/// (virtual slot 0x90), the words `before` the description, then the
/// description's characters.
fn warn_with_owner(e: &mut Engine, owner: Ptr, format: u32, before: &[u32]) {
    e.with_stack(STRING_SIZE, |e, string| {
        e.call(STRING_CONSTRUCT, &args![string]);
        e.vcall(owner.addr(), SLOT_DESCRIBE_0X90, &args![string]);
        let chars = e.call(STRING_CHARS, &args![string]).u32();
        let mut words = vec![format];
        words.extend_from_slice(before);
        words.push(chars);
        e.call(LOG_WARNING, &words);
        e.call(STRING_DESTRUCT, &args![string]);
    });
}

// Translated from 00681fb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00681fb0 - the exception-unwinding frame (FS:[0]) that destroys the warning strings is not translated.
/// `TESParameters::InitItem` (Xbox PDB; cdecl: the owner form, the function
/// data). With both non-null, for each parameter (`00682270`): a form
/// parameter (`GetParamIsTESForm`) with a nonzero id is resolved: the id
/// gets the owner's file's compile index (`AddCompileIndex` on a stack copy,
/// the file from `GetFile(-1)`), the form is looked up (`004839c0`) and
/// stored over the id; not found, a warning with the id; found and of
/// parameter type 0xb, the form is cast to `MagicItem` and stored, and a
/// failed cast warns. Any other parameter of type 0x16 (`00682380`) after
/// the first: the script of the parameter before it (`006823d0`) must have
/// the variable with this parameter's id (`Script::GetVariableName`), or a
/// warning names the id and the script (its slot 0x130). Each warning
/// describes the owner in a stack `BSString` (slot 0x90). The
/// exception-unwinding frame is not translated.
pub fn tes_parameters_init_item(e: &mut Engine, owner: Ptr, data: Ptr) {
    if owner.is_null() || data.is_null() {
        return;
    }
    let count = fn_00682270(e, data);
    let file = if owner.is_null() {
        0
    } else {
        e.call(GET_FILE, &args![owner, ANY_FILE]).u32()
    };
    let mut index = 0u32;
    while index < count {
        if tes_parameters_get_param_is_tes_form(e, data, index) != 0 {
            let slot = param_slot(data, index);
            e.with_stack(4, |e, id_slot| {
                e.mem.set_u32(id_slot.addr(), e.mem.u32(slot));
                if e.mem.u32(id_slot.addr()) == 0 {
                    return;
                }
                e.call(ADD_COMPILE_INDEX, &args![id_slot, file]);
                let id = e.mem.u32(id_slot.addr());
                let form = e.call(LOOKUP_FORM, &args![id]).u32();
                e.mem.set_u32(slot, form);
                if e.mem.u32(slot) == 0 {
                    let id = e.mem.u32(id_slot.addr());
                    warn_with_owner(e, owner, FORMAT_NO_FORM, &[id]);
                } else {
                    let kind = param_type(e, data, index);
                    let table =
                        PARAM_TYPE_TABLE.wrapping_add(kind.wrapping_mul(PARAM_TYPE_ENTRY_SIZE));
                    if e.mem.u32(table) == PARAM_TYPE_MAGIC_ITEM {
                        let cast = e
                            .call(
                                DYNAMIC_CAST,
                                &args![e.mem.u32(slot), 0u32, TYPE_TES_FORM, TYPE_MAGIC_ITEM, 0u32],
                            )
                            .u32();
                        e.mem.set_u32(slot, cast);
                        if e.mem.u32(slot) == 0 {
                            let id = e.mem.u32(id_slot.addr());
                            warn_with_owner(e, owner, FORMAT_NO_MAGIC_ITEM, &[id]);
                        }
                    }
                }
            });
        } else if fn_00682380(e, data, index) != 0 && index != 0 {
            let script = fn_006823d0(e, index - 1, data);
            if !script.is_null() {
                let variable = e.mem.u32(param_slot(data, index));
                if e.call(GET_VARIABLE_NAME, &args![script, variable]).u32() == 0 {
                    e.with_stack(STRING_SIZE, |e, string| {
                        e.call(STRING_CONSTRUCT, &args![string]);
                        e.vcall(owner.addr(), SLOT_DESCRIBE_0X90, &args![string]);
                        let chars = e.call(STRING_CHARS, &args![string]).u32();
                        let name = e.vcall(script.addr(), SLOT_NAME_0X130, &args![]).u32();
                        let variable = e.mem.u32(param_slot(data, index));
                        e.call(
                            LOG_WARNING,
                            &args![FORMAT_NO_VARIABLE, variable, name, chars],
                        );
                        e.call(STRING_DESTRUCT, &args![string]);
                    });
                }
            }
        }
        index += 1;
    }
}

// Translated from 00682270 (decompiled, FalloutNV.exe 1.4.0.525)
/// The parameter count of the function data's function (cdecl): the 16-bit
/// count of its function table entry (`01190922`), zero-extended; 0 for
/// null function data.
pub fn fn_00682270(e: &mut Engine, data: Ptr) -> u32 {
    if data.is_null() {
        return 0;
    }
    let number = e.mem.u16(data.addr()) as u32;
    e.mem
        .u16(FUNCTION_PARAM_COUNT.wrapping_add(number.wrapping_mul(FUNCTION_ENTRY_SIZE))) as u32
}

// Translated from 006822a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESParameters::GetParamIsTESForm` (Xbox PDB; cdecl: function data,
/// index): for function 0x198, `00594d60`'s answer; otherwise, for an index
/// below the count (unsigned), the is-form byte (+5) of the parameter's
/// type in the type table (`0118cdd0`); 0 for null function data or an
/// index past the count. The byte is returned as stored.
pub fn tes_parameters_get_param_is_tes_form(e: &mut Engine, data: Ptr, index: u32) -> u8 {
    if !data.is_null()
        && (e.mem.u16(data.addr()) as u32).wrapping_add(0x1000) == FUNCTION_0X198_PLUS_0X1000
    {
        return e.call(PARAM_IS_FORM_0X198, &args![data, index]).u8();
    }
    if data.is_null() || index >= fn_00682270(e, data) {
        return 0;
    }
    let kind = param_type(e, data, index);
    e.mem.u8(PARAM_TYPE_TABLE
        .wrapping_add(kind.wrapping_mul(PARAM_TYPE_ENTRY_SIZE))
        .wrapping_add(PARAM_TYPE_IS_FORM))
}

// Translated from 00682380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether parameter `index` of the function data (cdecl: function data,
/// index) has type 0x16: 1 if so, 0 for null data, an index past the count
/// (unsigned) or another type. The engine map gives it no name.
pub fn fn_00682380(e: &mut Engine, data: Ptr, index: u32) -> u8 {
    if data.is_null() || index >= fn_00682270(e, data) {
        return 0;
    }
    (param_type(e, data, index) == PARAM_TYPE_0X16) as u8
}

// Translated from 006823d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The script of form parameter `index` (cdecl: index, function data; note
/// the order): 0 unless `GetParamIsTESForm` says it is a form; a
/// `TESScriptableForm` (dynamic cast from `TESForm`) gives its word at +4
/// (`00726070`); otherwise a `TESObjectREFR` gives its base object's script
/// (`007af430`, then `TESScriptableForm::GetFormScript` `004826d0`); else
/// 0. The engine map gives it no name.
pub fn fn_006823d0(e: &mut Engine, index: u32, data: Ptr) -> Ptr {
    if tes_parameters_get_param_is_tes_form(e, data, index) == 0 {
        return Ptr::NULL;
    }
    let param = e.mem.u32(param_slot(data, index));
    let scriptable = e
        .call(
            DYNAMIC_CAST,
            &args![param, 0u32, TYPE_TES_FORM, TYPE_TES_SCRIPTABLE_FORM, 0u32],
        )
        .u32();
    if scriptable != 0 {
        return e.call(WORD_AT_FOUR, &args![scriptable]).ptr();
    }
    let reference = e
        .call(
            DYNAMIC_CAST,
            &args![param, 0u32, TYPE_TES_FORM, TYPE_TES_OBJECT_REFR, 0u32],
        )
        .u32();
    if reference == 0 {
        return Ptr::NULL;
    }
    let base = e.call(WORD_AT_0X20, &args![reference]).u32();
    e.call(GET_FORM_SCRIPT, &args![base]).ptr()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0068_1fb0, tes_parameters_init_item(Ptr, Ptr)),
        entry!(0x0068_2270, fn_00682270(Ptr) -> u32),
        entry!(0x0068_22a0, tes_parameters_get_param_is_tes_form(Ptr, u32) -> u8),
        entry!(0x0068_2380, fn_00682380(Ptr, u32) -> u8),
        entry!(0x0068_23d0, fn_006823d0(u32, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// The owner's describe slot and the script's name slot.
    const DESCRIBE: u32 = 0x7fff_0090;
    const NAME: u32 = 0x7fff_0130;

    /// Function `number` with parameters of the given types.
    fn function(e: &mut Engine, number: u32, types: &[u32]) {
        let entry = FUNCTION_PARAM_COUNT + number * FUNCTION_ENTRY_SIZE;
        e.map(entry, 8);
        e.mem.set_u16(entry, types.len() as u16);
        let list = e.mem.alloc(PARAM_ENTRY_SIZE * types.len().max(1) as u32);
        for (i, kind) in types.iter().enumerate() {
            e.mem.set_u32(list + PARAM_ENTRY_SIZE * i as u32 + 4, *kind);
        }
        e.mem
            .set_u32(FUNCTION_PARAM_LIST + number * FUNCTION_ENTRY_SIZE, list);
    }

    /// Parameter type `kind`: its compared word and its is-form byte.
    fn param_kind(e: &mut Engine, kind: u32, is_form: u8) {
        let entry = PARAM_TYPE_TABLE + kind * PARAM_TYPE_ENTRY_SIZE;
        e.map(entry, 8);
        e.mem.set_u32(entry, kind);
        e.mem.set_u8(entry + PARAM_TYPE_IS_FORM, is_form);
    }

    fn data(e: &mut Engine, number: u16, params: &[u32]) -> Ptr {
        let p = e.mem.alloc(12);
        e.mem.set_u16(p, number);
        for (i, w) in params.iter().enumerate() {
            e.mem.set_u32(p + 4 + 4 * i as u32, *w);
        }
        Ptr::new(p)
    }

    /// An object with a vtable holding the describe and name slots.
    fn object(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x140);
        e.mem.set_u32(vtable + 0x90, DESCRIBE);
        e.mem.set_u32(vtable + 0x130, NAME);
        let p = e.mem.alloc(0x30);
        e.mem.set_u32(p, vtable);
        p
    }

    fn setup() -> Engine {
        let mut e = Engine::new();
        for addr in [
            STRING_CONSTRUCT,
            STRING_DESTRUCT,
            LOG_WARNING,
            ADD_COMPILE_INDEX,
            DESCRIBE,
        ] {
            e.register(addr, |_, _| ret(0));
        }
        e.register(NAME, |_, a| ret(a[0] + 0x130));
        e.register(STRING_CHARS, |_, a| ret(a[0] + 1));
        e.register(GET_FILE, |_, _| ret(0x5000));
        // Ids below 0x100 are found as id + 0x1000.
        e.register(LOOKUP_FORM, |_, a| {
            ret(if a[0] < 0x100 { a[0] + 0x1000 } else { 0 })
        });
        e.register(PARAM_IS_FORM_0X198, |_, a| ret(0x40 + a[1]));
        e.register(WORD_AT_FOUR, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(WORD_AT_0X20, |e, a| ret(e.mem.u32(a[0] + 0x20)));
        e.register(GET_FORM_SCRIPT, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(GET_VARIABLE_NAME, |_, a| ret((a[1] == 7) as u32));
        e
    }

    #[test]
    fn param_count_reads_the_function_table() {
        let mut e = setup();
        function(&mut e, 0x12, &[1, 2, 3]);
        let d = data(&mut e, 0x12, &[]);
        assert_eq!(e.call(0x0068_2270, &args![d]).u32(), 3);
        assert_eq!(e.call(0x0068_2270, &args![0u32]).u32(), 0);
    }

    #[test]
    fn param_is_form_by_type_table_and_function_0x198() {
        let mut e = setup();
        function(&mut e, 0x12, &[4, 9]);
        param_kind(&mut e, 4, 1);
        param_kind(&mut e, 9, 0);
        let d = data(&mut e, 0x12, &[]);
        assert_eq!(e.call(0x0068_22a0, &args![d, 0u32]).u8(), 1);
        assert_eq!(e.call(0x0068_22a0, &args![d, 1u32]).u8(), 0);
        assert_eq!(e.call(0x0068_22a0, &args![d, 2u32]).u8(), 0);
        assert_eq!(e.call(0x0068_22a0, &args![0u32, 0u32]).u8(), 0);
        let special = data(&mut e, 0x198, &[]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0068_22a0, &args![special, 1u32]).u8(), 0x41);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, PARAM_IS_FORM_0X198), [vec![special.addr(), 1]]);
    }

    #[test]
    fn param_type_0x16() {
        let mut e = setup();
        function(&mut e, 0x12, &[0x16, 0x15]);
        let d = data(&mut e, 0x12, &[]);
        assert_eq!(e.call(0x0068_2380, &args![d, 0u32]).u8(), 1);
        assert_eq!(e.call(0x0068_2380, &args![d, 1u32]).u8(), 0);
        assert_eq!(e.call(0x0068_2380, &args![d, 2u32]).u8(), 0);
        assert_eq!(e.call(0x0068_2380, &args![0u32, 0u32]).u8(), 0);
    }

    #[test]
    fn script_of_a_form_parameter() {
        let mut e = setup();
        function(&mut e, 0x12, &[4, 9]);
        param_kind(&mut e, 4, 1);
        param_kind(&mut e, 9, 0);
        let scriptable = object(&mut e);
        e.mem.set_u32(scriptable + 4, 0x7777);
        let reference = object(&mut e);
        let base = e.mem.alloc(0x10);
        e.mem.set_u32(base + 8, 0x8888);
        e.mem.set_u32(reference + 0x20, base);
        e.register_double(DYNAMIC_CAST, move |_, a| {
            ret(match (a[0], a[3]) {
                (p, TYPE_TES_SCRIPTABLE_FORM) if p == scriptable => p,
                (p, TYPE_TES_OBJECT_REFR) if p == reference => p,
                _ => 0,
            })
        });
        let d = data(&mut e, 0x12, &[scriptable, 0]);
        assert_eq!(e.call(0x0068_23d0, &args![0u32, d]).u32(), 0x7777);
        let d = data(&mut e, 0x12, &[reference, 0]);
        assert_eq!(e.call(0x0068_23d0, &args![0u32, d]).u32(), 0x8888);
        let other = object(&mut e);
        let d = data(&mut e, 0x12, &[other, 0]);
        assert_eq!(e.call(0x0068_23d0, &args![0u32, d]).u32(), 0);
        // Not a form parameter.
        let d = data(&mut e, 0x12, &[scriptable, scriptable]);
        assert_eq!(e.call(0x0068_23d0, &args![1u32, d]).u32(), 0);
    }

    #[test]
    fn init_item_resolves_forms_and_warns() {
        let mut e = setup();
        // Parameter 0: a form (type 4); parameter 1: a magic item (type 0xb).
        function(&mut e, 0x12, &[4, 0xb]);
        param_kind(&mut e, 4, 1);
        param_kind(&mut e, 0xb, 1);
        e.register_double(DYNAMIC_CAST, |_, a| {
            ret(if a[0] == 0x1020 { a[0] } else { 0 })
        });
        let owner = object(&mut e);
        let d = data(&mut e, 0x12, &[0x10, 0x20]);
        e.call_log = Some(vec![]);
        e.call(0x0068_1fb0, &args![owner, d]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(d.addr() + 4), 0x1010);
        assert_eq!(e.mem.u32(d.addr() + 8), 0x1020);
        assert_eq!(calls(&log, GET_FILE), [vec![owner, ANY_FILE]]);
        assert_eq!(calls(&log, ADD_COMPILE_INDEX).len(), 2);
        assert_eq!(calls(&log, ADD_COMPILE_INDEX)[0][1], 0x5000);
        assert_eq!(
            calls(&log, DYNAMIC_CAST),
            [vec![0x1020, 0, TYPE_TES_FORM, TYPE_MAGIC_ITEM, 0]]
        );
        assert!(calls(&log, LOG_WARNING).is_empty());

        // A missing form and a failed magic item cast warn; a zero id is left.
        let d = data(&mut e, 0x12, &[0x200, 0x30]);
        e.call_log = Some(vec![]);
        e.call(0x0068_1fb0, &args![owner, d]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(d.addr() + 4), 0);
        assert_eq!(e.mem.u32(d.addr() + 8), 0);
        let warnings = calls(&log, LOG_WARNING);
        assert_eq!(warnings.len(), 2);
        assert_eq!(warnings[0][..2], [FORMAT_NO_FORM, 0x200]);
        assert_eq!(warnings[1][..2], [FORMAT_NO_MAGIC_ITEM, 0x30]);
        assert_eq!(calls(&log, DESCRIBE).len(), 2);
        assert_eq!(calls(&log, STRING_DESTRUCT).len(), 2);
        let string = warnings[0][2] - 1;
        assert_eq!(calls(&log, DESCRIBE)[0], [owner, string]);

        let d = data(&mut e, 0x12, &[0, 0]);
        e.call_log = Some(vec![]);
        e.call(0x0068_1fb0, &args![owner, d]);
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, ADD_COMPILE_INDEX).is_empty());

        // A null owner or null data does nothing.
        e.call_log = Some(vec![]);
        e.call(0x0068_1fb0, &args![0u32, d]);
        e.call(0x0068_1fb0, &args![owner, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 2);
    }

    #[test]
    fn init_item_checks_script_variables() {
        let mut e = setup();
        // Parameter 0: a scriptable form; parameter 1: a variable (0x16).
        function(&mut e, 0x13, &[4, 0x16]);
        param_kind(&mut e, 4, 1);
        param_kind(&mut e, 0x16, 0);
        let script = object(&mut e);
        // The looked-up form 0x1010 casts to a scriptable form whose +4 is
        // the script.
        let scriptable = e.mem.alloc(8);
        e.mem.set_u32(scriptable + 4, script);
        e.register_double(DYNAMIC_CAST, move |_, a| {
            ret(if a[0] == 0x1010 && a[3] == TYPE_TES_SCRIPTABLE_FORM {
                scriptable
            } else {
                0
            })
        });
        let owner = object(&mut e);
        let d = data(&mut e, 0x13, &[0x10, 7]);
        e.call_log = Some(vec![]);
        e.call(0x0068_1fb0, &args![owner, d]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, GET_VARIABLE_NAME), [vec![script, 7]]);
        assert!(calls(&log, LOG_WARNING).is_empty());

        let d = data(&mut e, 0x13, &[0x10, 9]);
        e.call_log = Some(vec![]);
        e.call(0x0068_1fb0, &args![owner, d]);
        let log = e.call_log.take().unwrap();
        let warnings = calls(&log, LOG_WARNING);
        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0][..3], [FORMAT_NO_VARIABLE, 9, script + 0x130]);
        assert_eq!(calls(&log, NAME), [vec![script]]);
    }
}
