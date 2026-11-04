//! `fallout shared/bgsentrypointfunction.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005e64e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `005e6490` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_005e64e0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x005e_6490, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 005e65d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSEntryPointFunctionDataActivateChoice` scalar deleting destructor (engine map name): runs the destructor body `005e6600` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn bgs_entry_point_function_data_activate_choice_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x005e_6600, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// `005b5e40`: the warning printer (cdecl, format and its arguments).
const PRINT_WARNING: u32 = 0x005b_5e40;
/// `00401000`: `operator new` (cdecl).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `00401030`: `operator delete` (cdecl).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `00401660`: the current chunk's size (`ECX` = the file).
const CHUNK_SIZE: u32 = 0x0040_1660;
/// `00401680`: whether the file's data needs its byte order converted.
const NEEDS_BYTE_SWAP: u32 = 0x0040_1680;
/// `00401080`: byte-swaps the word at the address in place (cdecl; the
/// callers pass 0 as the second word).
const SWAP_WORD_IN_PLACE: u32 = 0x0040_1080;
/// `TESFile::GetTESChunk` (`004726b0`, engine map).
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// `TESFile::GetChunkData` (`004727f0`, engine map): one word.
const GET_CHUNK_WORD: u32 = 0x0047_27f0;
/// `TESFile::GetChunkData_ov2` (`00472840`, engine map): one stack word.
const GET_CHUNK_DATA_OV2: u32 = 0x0047_2840;
/// `TESFile::GetChunkData_ov3` (`00472890`, engine map): buffer and size
/// (0: the whole chunk).
const GET_CHUNK_DATA: u32 = 0x0047_2890;
/// `0084e3a0`: the form's id.
const FORM_ID: u32 = 0x0084_e3a0;
/// The number of function arguments of each entry point function type
/// (`uint32[]`, read by `CheckFunctionArguments`).
const ARGUMENT_COUNTS: u32 = 0x0119_7400;
/// `"DEFAULT: Invalid function type %i passed to BGSEntryPointFunction::CheckFunctionArguments.  Expected %i."`.
const BAD_TYPE_WARNING: u32 = 0x0104_6250;
/// `"DEFAULT: Invalid argument count %i passed to ...  Expected %i."`.
const BAD_COUNT_WARNING: u32 = 0x0104_61e0;
/// `"DEFAULT: Invalid function argument %i passed to ...  Function arguments cannot be zero."`.
const ZERO_ARGUMENT_WARNING: u32 = 0x0104_6158;
/// `"DEFAULT: Invalid function data type %i passed to ...  Expected %i."`.
const BAD_DATA_TYPE_WARNING: u32 = 0x0104_60e8;
/// `"MASTERFILE: Unable to find leveled list (%08X) on owner object \"%s\"."`.
const NO_LEVELED_LIST_NAMED: u32 = 0x0104_63a8;
/// `"MASTERFILE: Unable to find leveled list (%08X) on owner object (%08X)."`.
const NO_LEVELED_LIST_ID: u32 = 0x0104_6360;
/// `"MASTERFILE: Errors were encountered during InitItem for result script for perk '%s' (%08X)..."`.
const RESULT_SCRIPT_ERRORS: u32 = 0x0104_63f0;
/// `BGSEntryPointFunctionData`'s vtable (RTTI).
const FUNCTION_DATA_VTABLE: u32 = 0x0104_62e0;
/// `BGSEntryPointFunctionDataOneValue`'s vtable (RTTI).
const ONE_VALUE_VTABLE: u32 = 0x0104_62c0;
/// `BGSEntryPointFunctionDataLeveledList`'s vtable (RTTI).
const LEVELED_LIST_VTABLE: u32 = 0x0104_6320;
/// `BGSEntryPointFunctionDataActivateChoice`'s vtable (RTTI).
const ACTIVATE_CHOICE_VTABLE: u32 = 0x0104_6340;
/// `TESForm`'s RTTI type descriptor (`.?AVTESForm@@`).
const TYPE_TES_FORM: u32 = 0x0118_3028;
/// `TESLevItem`'s RTTI type descriptor (`.?AVTESLevItem@@`): the target of
/// the leveled-list cast.
const TYPE_LEVELED_LIST_TARGET: u32 = 0x0118_6f7c;
/// The script variable `ActivateChoice::Load` is filling between its
/// `SLSD` and `SCVR` chunks (`ScriptVariable*`).
const CURRENT_VARIABLE: u32 = 0x011c_b0fc;

const CHUNK_SCRO: u32 = 0x4f52_4353;
const CHUNK_SCRV: u32 = 0x5652_4353;
const CHUNK_SLSD: u32 = 0x4453_4c53;
const CHUNK_SCVR: u32 = 0x5256_4353;
const CHUNK_SCHR: u32 = 0x5248_4353;
const CHUNK_SCDA: u32 = 0x4144_4353;
const CHUNK_EPF2: u32 = 0x3246_5045;
const CHUNK_EPF3: u32 = 0x3346_5045;

// Translated from 005e5b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSEntryPointFunction::CheckFunctionArguments` (Xbox PDB), cdecl, seven
/// words: whether `function_type` is `expected_type`, `argument_count`
/// (low byte) is the type's count (`01197400[expected_type]`), none of the
/// `argument_count` arguments at `arguments` is 0, and `data` (when not
/// null) reports through vtable slot 1 the data type `expected_data_type`
/// (low byte). Each failure prints its warning and returns false. The third
/// word is never read.
pub fn bgs_entry_point_function_check_function_arguments(
    e: &mut Engine,
    expected_type: u32,
    expected_data_type: u8,
    _unused_3: u32,
    function_type: u32,
    argument_count: u8,
    arguments: Ptr,
    data: Ptr,
) -> bool {
    if expected_type != function_type {
        e.call(
            PRINT_WARNING,
            &args![BAD_TYPE_WARNING, function_type, expected_type],
        );
        return false;
    }
    let count_slot = ARGUMENT_COUNTS.wrapping_add(expected_type.wrapping_mul(4));
    let expected_count = e.mem.u32(count_slot);
    if expected_count != u32::from(argument_count) {
        e.call(
            PRINT_WARNING,
            &args![BAD_COUNT_WARNING, u32::from(argument_count), expected_count],
        );
        return false;
    }
    for index in 0..u32::from(argument_count) {
        if e.mem.u32(arguments.addr().wrapping_add(index * 4)) == 0 {
            e.call(PRINT_WARNING, &args![ZERO_ARGUMENT_WARNING, index]);
            return false;
        }
    }
    if !data.is_null() {
        let data_type = e.vcall(data.addr(), 0x4, &args![]).u32();
        if data_type != u32::from(expected_data_type) {
            let reported = e.vcall(data.addr(), 0x4, &args![]).u32();
            e.call(
                PRINT_WARNING,
                &args![
                    BAD_DATA_TYPE_WARNING,
                    reported,
                    u32::from(expected_data_type)
                ],
            );
            return false;
        }
    }
    true
}

// Translated from 005e5d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The entry point function of type 0 with a one-value data (type 1), cdecl,
/// five words: when `CheckFunctionArguments(0, 1, ...)` passes, the `float`
/// the first argument points at is multiplied by the data's value
/// (`006b9130`, in `ST0`): `FMUL dword`, `FSTP dword`.
pub fn fn_005e5d20(
    e: &mut Engine,
    first: u32,
    function_type: u32,
    argument_count: u8,
    arguments: Ptr,
    data: Ptr,
) {
    if !bgs_entry_point_function_check_function_arguments(
        e,
        0,
        1,
        first,
        function_type,
        argument_count,
        arguments,
        data,
    ) {
        return;
    }
    let target = e.mem.u32(arguments.addr());
    let value = e.call(0x006b_9130, &args![data]).f64();
    let product = x87_mul(value, x87_m32(e.mem.u32(target)));
    e.mem.set_u32(target, x87_store32(product));
}

/// A function-data constructor (`ECX` = the block; returns it).
type DataConstructor = fn(&mut Engine, Ptr) -> Ptr;

// Translated from 005e6280 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005e6280 - the exception-unwinding frame (FS:[0]) and the security-cookie word it pushes are not translated.
/// Creates the function data object for a data type (cdecl, one word):
/// 1 a one-value data (8 bytes, [`fn_005e63f0`]), 2 a two-value data (12
/// bytes, `005e64b0`, vtable `01046300`, RTTI
/// `BGSEntryPointFunctionDataTwoValue`), 3 a leveled-list data (8 bytes, [`fn_005e6510`]), 4 an
/// activate-choice data (0x60 bytes, [`fn_005e6540`]); null for any other
/// type or when the allocation fails.
pub fn fn_005e6280(e: &mut Engine, data_type: u32) -> Ptr {
    let (size, construct): (u32, Option<DataConstructor>) = match data_type.wrapping_sub(1) {
        0 => (8, Some(fn_005e63f0)),
        1 => (0xc, None),
        2 => (8, Some(fn_005e6510)),
        3 => (0x60, Some(fn_005e6540)),
        _ => return Ptr::NULL,
    };
    let block = e.call(OPERATOR_NEW, &args![size]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    match construct {
        Some(construct) => construct(e, Ptr::new(block)),
        None => e.call(0x005e_64b0, &args![block]).ptr(),
    }
}

// Translated from 005e63f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSEntryPointFunctionDataOneValue` constructor (its vtable `010462c0`,
/// by RTTI): the base ([`fn_005e6420`]), the vtable, the value +0x4 = 0.0
/// (`FLDZ`). Returns `this`.
pub fn fn_005e63f0(e: &mut Engine, this: Ptr) -> Ptr {
    fn_005e6420(e, this);
    e.mem.set_u32(this.addr(), ONE_VALUE_VTABLE);
    e.mem.set_f32(this.addr() + 4, 0.0);
    this
}

// Translated from 005e6420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSEntryPointFunctionData` constructor (its vtable `010462e0`, by
/// RTTI): stores the vtable. Returns `this`.
pub fn fn_005e6420(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), FUNCTION_DATA_VTABLE);
    this
}

// Translated from 005e6470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSEntryPointFunctionData` destructor body: stores its vtable back.
pub fn fn_005e6470(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), FUNCTION_DATA_VTABLE);
}

// Translated from 005e6490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body that only runs the `BGSEntryPointFunctionData` one
/// ([`fn_005e6470`]).
pub fn fn_005e6490(e: &mut Engine, this: Ptr) {
    fn_005e6470(e, this);
}

// Translated from 005e6510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSEntryPointFunctionDataLeveledList` constructor (its vtable
/// `01046320`, by RTTI): the base, the vtable, the list +0x4 = null.
/// Returns `this`.
pub fn fn_005e6510(e: &mut Engine, this: Ptr) -> Ptr {
    fn_005e6420(e, this);
    e.mem.set_u32(this.addr(), LEVELED_LIST_VTABLE);
    e.mem.set_u32(this.addr() + 4, 0);
    this
}

// Translated from 005e6540 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005e6540 - the exception-unwinding frame (FS:[0]) and the security-cookie word it pushes are not translated.
/// `BGSEntryPointFunctionDataActivateChoice` constructor (its vtable
/// `01046340`, by RTTI): the base, the vtable, the label +0x4 = null, the
/// result script at +0x8 (`Script::Script`, `005aa0f0`), the word +0x5C =
/// 0, then [`fn_005e6c30`]. Returns `this`.
pub fn fn_005e6540(e: &mut Engine, this: Ptr) -> Ptr {
    fn_005e6420(e, this);
    e.mem.set_u32(this.addr(), ACTIVATE_CHOICE_VTABLE);
    e.mem.set_u32(this.addr() + 4, 0);
    e.call(0x005a_a0f0, &args![this.addr() + 8]);
    e.mem.set_u16(this.addr() + 0x5c, 0);
    fn_005e6c30(e, this);
    this
}

// Translated from 005e6600 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005e6600 - the exception-unwinding frame (FS:[0]) and the security-cookie word it pushes are not translated.
/// `BGSEntryPointFunctionDataActivateChoice` destructor body: the vtable
/// `01046340`, the label freed ([`bgs_entry_point_function_data_activate_choice_set_label`]
/// with null), the script's destructor `005aa1a0` (+0x8), then the base
/// one ([`fn_005e6470`]).
pub fn fn_005e6600(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ACTIVATE_CHOICE_VTABLE);
    bgs_entry_point_function_data_activate_choice_set_label(e, this, Ptr::NULL);
    e.call(0x005a_a1a0, &args![this.addr() + 8]);
    fn_005e6470(e, this);
}

// Translated from 005e6790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSEntryPointFunctionDataOneValue::Load` (Xbox PDB): false unless the
/// chunk of `file` (one stack word) is 4 bytes; reads it into +0x4,
/// byte-swapped when the file needs it. Returns true.
pub fn bgs_entry_point_function_data_one_value_load(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    if e.call(CHUNK_SIZE, &args![file]).u32() != 4 {
        return false;
    }
    e.call(GET_CHUNK_DATA, &args![file, this.addr() + 4, 0u32]);
    if e.call(NEEDS_BYTE_SWAP, &args![file]).bool() {
        e.call(SWAP_WORD_IN_PLACE, &args![this.addr() + 4, 0u32]);
    }
    true
}

// Translated from 005e6b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSEntryPointFunctionDataLeveledList::Load` (Xbox PDB): false unless the
/// chunk is 4 bytes; reads the form id into a stack word (byte-swapped when
/// the file needs it) and stores it at +0x4. Returns true.
pub fn bgs_entry_point_function_data_leveled_list_load(
    e: &mut Engine,
    this: Ptr,
    file: Ptr,
) -> bool {
    if e.call(CHUNK_SIZE, &args![file]).u32() != 4 {
        return false;
    }
    let id = e.with_stack(4, |e, local| {
        e.call(GET_CHUNK_DATA, &args![file, local, 0u32]);
        if e.call(NEEDS_BYTE_SWAP, &args![file]).bool() {
            e.call(SWAP_WORD_IN_PLACE, &args![local, 0u32]);
        }
        e.mem.u32(local.addr())
    });
    e.mem.set_u32(this.addr() + 4, id);
    true
}

// Translated from 005e6b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSEntryPointFunctionDataLeveledList::InitItem` (Xbox PDB): resolves
/// the form id at +0x4 (when not 0) against `owner`'s file
/// (`TESForm::GetFile(-1)`, `TESForm::AddCompileIndex`), looks it up
/// (`004839c0`) and casts it from `TESForm` to the type at `01186f7c`
/// (`__RTDynamicCast`) into +0x4. When that gives null it warns with the
/// id and the owner's editor id (slot 0x130) when `00474cb0` reports one,
/// else with the owner's form id.
pub fn bgs_entry_point_function_data_leveled_list_init_item(e: &mut Engine, this: Ptr, owner: Ptr) {
    if e.mem.u32(this.addr() + 4) == 0 {
        return;
    }
    let id = e.with_stack(4, |e, local| {
        let id = e.mem.u32(this.addr() + 4);
        e.mem.set_u32(local.addr(), id);
        let file = e.call(0x0048_4e60, &args![owner, 0xffff_ffffu32]).u32();
        e.call(0x0048_5d50, &args![local, file]);
        e.mem.u32(local.addr())
    });
    let found = e.call(0x0048_39c0, &args![id]).u32();
    let list = e
        .call(
            0x00ec_43fb,
            &args![found, 0u32, TYPE_TES_FORM, TYPE_LEVELED_LIST_TARGET, 0u32],
        )
        .u32();
    e.mem.set_u32(this.addr() + 4, list);
    if list != 0 {
        return;
    }
    if e.call(0x0047_4cb0, &args![owner]).u32() != 0 {
        let editor_id = e.vcall(owner.addr(), 0x130, &args![]).u32();
        e.call(PRINT_WARNING, &args![NO_LEVELED_LIST_NAMED, id, editor_id]);
    } else {
        let owner_id = e.call(FORM_ID, &args![owner]).u32();
        e.call(PRINT_WARNING, &args![NO_LEVELED_LIST_ID, id, owner_id]);
    }
}

// Translated from 005e6c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the word at +0x5C and prepares the result script at +0x8:
/// `00484ab0(0)` and `TESForm::SetTemporary` (`00484490`, engine map).
pub fn fn_005e6c30(e: &mut Engine, this: Ptr) {
    e.mem.set_u16(this.addr() + 0x5c, 0);
    e.call(0x0048_4ab0, &args![this.addr() + 8, 0u32]);
    e.call(0x0048_4490, &args![this.addr() + 8]);
}

// Translated from 005e6c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSEntryPointFunctionDataActivateChoice::SetLabel` (Xbox PDB): frees
/// the label at +0x4 and clears it; with a `label` (one stack word), stores
/// a new copy of it (`00401000` of its length + 1, then `004046f0`).
pub fn bgs_entry_point_function_data_activate_choice_set_label(
    e: &mut Engine,
    this: Ptr,
    label: Ptr,
) {
    let old = e.mem.u32(this.addr() + 4);
    e.call(OPERATOR_DELETE, &args![old]);
    e.mem.set_u32(this.addr() + 4, 0);
    if label.is_null() {
        return;
    }
    let length = e.call(0x0044_a670, &args![label]).u32();
    let copy = e.call(OPERATOR_NEW, &args![length.wrapping_add(1)]).u32();
    e.mem.set_u32(this.addr() + 4, copy);
    e.call(0x0040_46f0, &args![copy, label]);
}

// Translated from 005e6ee0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005e6ee0 - the exception-unwinding frame (FS:[0]) and the /GS security-cookie check (00ec408c) are not translated; the __alloca_probe_16 (00ec5ec0) buffers are with_stack blocks of the chunk size rounded up to 16 bytes.
/// `BGSEntryPointFunctionDataActivateChoice::Load` (Xbox PDB): reads the
/// current chunk of `file` (one stack word) into the result script at
/// +0x8 or the label, and returns true:
/// - `SCRO`/`SCRV`: a new referenced-object entry (0x10 bytes,
///   `00500910`) gets the form id at +0x8 (`SCRO`) or the variable index at
///   +0xC (`SCRV`) and goes onto the script's reference list (`005d43c0`,
///   `00905820`);
/// - `SLSD`: a new script variable (0x20 bytes, `005e28b0`) becomes the
///   current one (`011cb0fc`), loads (`ScriptVariable::Load`) and goes onto
///   the script's variable list (`005d43e0`, `00905820`); `SCVR` (when the
///   chunk is not also `SLSD`) names the current variable (+0x18,
///   `004037f0`, at most 0x200 bytes read) when there is one;
/// - `SCHR`: the script header (`00500940`), byte-swapped (`00414220`)
///   when the file needs it; `SCDA`: the compiled data, zeroed then read
///   into a stack buffer, given to `Script::SetCompileData`;
/// - `EPF2`: the label ([`bgs_entry_point_function_data_activate_choice_set_label`]);
///   `EPF3`: one word into +0x5C (`GetChunkData_ov2`).
pub fn bgs_entry_point_function_data_activate_choice_load(
    e: &mut Engine,
    this: Ptr,
    file: Ptr,
) -> bool {
    let script = this.addr() + 8;
    let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
    match chunk {
        CHUNK_SCRO | CHUNK_SCRV => {
            let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
            let entry = if block == 0 {
                0
            } else {
                e.call(0x0050_0910, &args![block]).u32()
            };
            e.with_stack(8, |e, locals| {
                // `locals + 0`: the entry pointer; `+4`: the word read.
                e.mem.set_u32(locals.addr(), entry);
                e.call(GET_CHUNK_WORD, &args![file, locals.addr() + 4]);
                let value = e.mem.u32(locals.addr() + 4);
                let field = if chunk == CHUNK_SCRO { 8 } else { 0xc };
                e.mem.set_u32(entry.wrapping_add(field), value);
                let list = e.call(0x005d_43c0, &args![script]).u32();
                e.call(0x0090_5820, &args![list, locals]);
            });
        }
        CHUNK_SLSD | CHUNK_SCVR => {
            if e.call(GET_TES_CHUNK, &args![file]).u32() == CHUNK_SLSD {
                let block = e.call(OPERATOR_NEW, &args![0x20u32]).u32();
                let variable = if block == 0 {
                    0
                } else {
                    e.call(0x005e_28b0, &args![block]).u32()
                };
                e.mem.set_u32(CURRENT_VARIABLE, variable);
                e.call(0x005e_2a20, &args![variable, file]);
                let list = e.call(0x005d_43e0, &args![script]).u32();
                e.call(0x0090_5820, &args![list, CURRENT_VARIABLE]);
            } else if e.mem.u32(CURRENT_VARIABLE) != 0 {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                let reserved = size.wrapping_add(15) & !15;
                e.with_stack(reserved, |e, buffer| {
                    e.call(GET_CHUNK_DATA, &args![file, buffer, 0x200u32]);
                    let variable = e.mem.u32(CURRENT_VARIABLE);
                    e.call(0x0040_37f0, &args![variable + 0x18, buffer, 0u32]);
                });
            }
        }
        CHUNK_SCHR => {
            let header = e.call(0x0050_0940, &args![script]).u32();
            e.call(GET_CHUNK_DATA, &args![file, header, 0u32]);
            if e.call(NEEDS_BYTE_SWAP, &args![file]).bool() {
                let header = e.call(0x0050_0940, &args![script]).u32();
                e.call(0x0041_4220, &args![header]);
            }
        }
        CHUNK_SCDA => {
            let size = e.call(CHUNK_SIZE, &args![file]).u32();
            let reserved = size.wrapping_add(15) & !15;
            e.with_stack(reserved, |e, buffer| {
                e.call(0x0040_3d30, &args![buffer, 0u32, size]);
                e.call(GET_CHUNK_DATA, &args![file, buffer, 0u32]);
                e.call(0x005a_ae70, &args![script, size, buffer]);
            });
        }
        CHUNK_EPF2 => {
            let size = e.call(CHUNK_SIZE, &args![file]).u32();
            let reserved = size.wrapping_add(15) & !15;
            e.with_stack(reserved, |e, buffer| {
                e.call(GET_CHUNK_DATA, &args![file, buffer, 0u32]);
                bgs_entry_point_function_data_activate_choice_set_label(e, this, buffer);
            });
        }
        CHUNK_EPF3 => {
            e.call(GET_CHUNK_DATA_OV2, &args![file, this.addr() + 0x5c]);
        }
        _ => {}
    }
    true
}

// Translated from 005e7230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSEntryPointFunctionDataActivateChoice::InitItem` (Xbox PDB): resolves
/// the result script (`Script::InitItem`, `005ab400`, with `owner`) with
/// warnings off (`MessageHandler::IncDisableWarningCount` 1, then 0) and the
/// thread's error count (`0046e8a0`/`004fffe0`) cleared around it, then
/// restored; when that count is not 0 afterwards it warns with the owner's
/// editor id (slot 0x130) and form id.
pub fn bgs_entry_point_function_data_activate_choice_init_item(
    e: &mut Engine,
    this: Ptr,
    owner: Ptr,
) {
    let saved = e.call(0x0046_e8a0, &args![]).u32();
    e.call(0x0043_b2b0, &args![1u32]);
    e.call(0x004f_ffe0, &args![0u32]);
    e.call(0x005a_b400, &args![this.addr() + 8, owner]);
    e.call(0x0043_b2b0, &args![0u32]);
    let errors = e.call(0x0046_e8a0, &args![]).u32();
    e.call(0x004f_ffe0, &args![saved]);
    if errors != 0 {
        let owner_id = e.call(FORM_ID, &args![owner]).u32();
        let editor_id = e.vcall(owner.addr(), 0x130, &args![]).u32();
        e.call(
            PRINT_WARNING,
            &args![RESULT_SCRIPT_ERRORS, editor_id, owner_id],
        );
    }
}

/// `005e5b80` [`bgs_entry_point_function_check_function_arguments`] and the
/// value getter of a one- or two-value data (`006b9130`, `ECX` = the data,
/// result in `ST0`).
const DATA_VALUE: u32 = 0x006b_9130;
/// `00488d50` (engine map: `BSFaceGenKeyframe::GetTime`, a folded getter):
/// called on a two-value data, result in `ST0`.
const DATA_SECOND_VALUE: u32 = 0x0048_8d50;
/// `00413f40`: `this + 8` (the activate-choice data's result script).
const RESULT_SCRIPT_OF: u32 = 0x0041_3f40;
/// `00ec43fb`: `__RTDynamicCast` (cdecl, five words).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// `004858f0`: adds a one-byte chunk to the form being saved (cdecl).
const ADD_BYTE_CHUNK: u32 = 0x0048_58f0;
/// `00485910`: adds a four-byte chunk to the form being saved (cdecl).
const ADD_WORD_CHUNK: u32 = 0x0048_5910;
/// `0x54465045`, chunk `EPFT` (the data type byte).
const CHUNK_EPFT: u32 = 0x5446_5045;
/// `0x44465045`, chunk `EPFD` (the data's values).
const CHUNK_EPFD: u32 = 0x4446_5045;
/// `BGSEntryPointFunctionDataTwoValue`'s vtable (RTTI).
const TWO_VALUE_VTABLE: u32 = 0x0104_6300;
/// RTTI type descriptors (`.?AVBGSEntryPointFunctionData...@@`): the cast
/// source of the `Compare` methods and their targets.
const TYPE_FUNCTION_DATA: u32 = 0x0119_7440;
const TYPE_ONE_VALUE: u32 = 0x0119_7410;
const TYPE_TWO_VALUE: u32 = 0x0119_7468;
const TYPE_LEVELED_LIST: u32 = 0x0119_7498;
const TYPE_ACTIVATE_CHOICE: u32 = 0x0119_74cc;

// Translated from 005e5c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The entry point function of type 0 with a one-value data (type 1), cdecl,
/// five words: when `CheckFunctionArguments(0, 1, ...)` passes, the `float`
/// the first argument points at becomes the data's value (`006b9130`, in
/// `ST0`; `FSTP dword`). The first word only goes to the check.
pub fn fn_005e5c70(
    e: &mut Engine,
    subject: u32,
    function_type: u32,
    argument_count: u8,
    arguments: Ptr,
    data: Ptr,
) {
    if !bgs_entry_point_function_check_function_arguments(
        e,
        0,
        1,
        subject,
        function_type,
        argument_count,
        arguments,
        data,
    ) {
        return;
    }
    let target = e.mem.u32(arguments.addr());
    let value = e.call(DATA_VALUE, &args![data]).f64();
    e.mem.set_u32(target, x87_store32(value));
}

// Translated from 005e5cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The entry point function of type 0 with a one-value data (type 1), cdecl,
/// five words: when the check passes, the data's value (`006b9130`) is added
/// to the `float` the first argument points at (`FADD dword`, `FSTP dword`).
pub fn fn_005e5cc0(
    e: &mut Engine,
    subject: u32,
    function_type: u32,
    argument_count: u8,
    arguments: Ptr,
    data: Ptr,
) {
    if !bgs_entry_point_function_check_function_arguments(
        e,
        0,
        1,
        subject,
        function_type,
        argument_count,
        arguments,
        data,
    ) {
        return;
    }
    let target = e.mem.u32(arguments.addr());
    let value = e.call(DATA_VALUE, &args![data]).f64();
    let sum = x87_add(value, x87_m32(e.mem.u32(target)));
    e.mem.set_u32(target, x87_store32(sum));
}

// Translated from 005e5d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The entry point function of type 0 with a two-value data (type 2), cdecl,
/// five words: when `CheckFunctionArguments(0, 2, ...)` passes, adds
/// `RandomFloat(value, second)` (`00476b70`, cdecl, both pushed as floats;
/// `value` from `006b9130`, `second` from `00488d50`, read first) to the
/// `float` the first argument points at.
pub fn fn_005e5d80(
    e: &mut Engine,
    subject: u32,
    function_type: u32,
    argument_count: u8,
    arguments: Ptr,
    data: Ptr,
) {
    if !bgs_entry_point_function_check_function_arguments(
        e,
        0,
        2,
        subject,
        function_type,
        argument_count,
        arguments,
        data,
    ) {
        return;
    }
    let target = e.mem.u32(arguments.addr());
    let second = e.call(DATA_SECOND_VALUE, &args![data]).f64();
    let second = x87_store32(second);
    let value = e.call(DATA_VALUE, &args![data]).f64();
    let value = x87_store32(value);
    let random = e.call(0x0047_6b70, &args![value, second]).f64();
    let sum = x87_add(random, x87_m32(e.mem.u32(target)));
    e.mem.set_u32(target, x87_store32(sum));
}

// Translated from 005e5df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The entry point function of type 0 with a two-value data (type 2), cdecl,
/// five words: when the check passes, the data's value (`006b9130`, stored
/// as a `float`) times the integer slot 2 (+0x8) of the object at
/// `subject + 0xA4` answers for the data's second value (`00488d50`,
/// truncated with `FISTP qword` under the compiler's chop sequence, low
/// word passed) is added to the `float` the first argument points at
/// (`FILD`, `FMUL dword`, `FADD dword`, `FSTP dword`).
pub fn fn_005e5df0(
    e: &mut Engine,
    subject: u32,
    function_type: u32,
    argument_count: u8,
    arguments: Ptr,
    data: Ptr,
) {
    if !bgs_entry_point_function_check_function_arguments(
        e,
        0,
        2,
        subject,
        function_type,
        argument_count,
        arguments,
        data,
    ) {
        return;
    }
    let target = e.mem.u32(arguments.addr());
    let value = e.call(DATA_VALUE, &args![data]).f64();
    let scale = x87_store32(value);
    let second = e.call(DATA_SECOND_VALUE, &args![data]).f64();
    let whole = x87_fistp64_chop(X87Ext::from(second)) as u32;
    let count = e
        .vcall(subject.wrapping_add(0xa4), 0x8, &args![whole])
        .i32();
    let product = x87_mul(f64::from(count), x87_m32(scale));
    let sum = x87_add(product, x87_m32(e.mem.u32(target)));
    e.mem.set_u32(target, x87_store32(sum));
}

// Translated from 005e5ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The entry point function of type 0 without data (data type 0), cdecl,
/// five words: when `CheckFunctionArguments(0, 0, ...)` passes, the `float`
/// the first argument points at becomes `00408840` of itself (cdecl, the
/// float pushed through `FLD`/`FSTP`; result in `ST0`).
pub fn fn_005e5ea0(
    e: &mut Engine,
    subject: u32,
    function_type: u32,
    argument_count: u8,
    arguments: Ptr,
    data: Ptr,
) {
    if !bgs_entry_point_function_check_function_arguments(
        e,
        0,
        0,
        subject,
        function_type,
        argument_count,
        arguments,
        data,
    ) {
        return;
    }
    let target = e.mem.u32(arguments.addr());
    let pushed = x87_quiet_bits(e.mem.u32(target));
    let result = e.call(0x0040_8840, &args![pushed]).f64();
    e.mem.set_u32(target, x87_store32(result));
}

// Translated from 005e5ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// As [`fn_005e5ea0`], with the result negated (`FCHS`) before it is stored.
pub fn fn_005e5ef0(
    e: &mut Engine,
    subject: u32,
    function_type: u32,
    argument_count: u8,
    arguments: Ptr,
    data: Ptr,
) {
    if !bgs_entry_point_function_check_function_arguments(
        e,
        0,
        0,
        subject,
        function_type,
        argument_count,
        arguments,
        data,
    ) {
        return;
    }
    let target = e.mem.u32(arguments.addr());
    let pushed = x87_quiet_bits(e.mem.u32(target));
    let result = e.call(0x0040_8840, &args![pushed]).f64();
    e.mem.set_u32(target, x87_store32(-result));
}

// Translated from 005e5f50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005e5f50 - the exception-unwinding frame (FS:[0]) and the security-cookie word it pushes are not translated.
/// The entry point function of type 1 with a leveled-list data (type 3),
/// cdecl, five words: when `CheckFunctionArguments(1, 3, ...)` passes, a
/// stack `TESContainer` (12 bytes, `00481610`/`00481680`) is filled from
/// the data's list (`00726070` of the data, +0x30; `00488350` with
/// `subject` and the container). When the first argument (`target`) has
/// inventory changes (`004bf220`), each container entry (list nodes from
/// `00717e50`, next `00726070`, until a node is empty (`008256d0`) or holds
/// no entry) whose item (+0x4) answers true to slot 57 (+0xE4) is added to
/// `target` through its slot 100 (+0x190): with the item's script
/// (`004826d0`) as one item per count, each with a new `ExtraDataList`
/// (0x20 bytes, `00410360`) counted 1 (`00419ad0`) that gets the script
/// (`00419ed0`, `00419f80` of `005abf60`) when it has none, and the last
/// list passed with count 1; without a script with no list and the entry's
/// count (+0x0).
pub fn fn_005e5f50(
    e: &mut Engine,
    subject: u32,
    function_type: u32,
    argument_count: u8,
    arguments: Ptr,
    data: Ptr,
) {
    if !bgs_entry_point_function_check_function_arguments(
        e,
        1,
        3,
        subject,
        function_type,
        argument_count,
        arguments,
        data,
    ) {
        return;
    }
    let target = e.mem.u32(arguments.addr());
    e.with_stack(0xc, |e, container| {
        e.call(0x0048_1610, &args![container]);
        let holder = e.call(0x0072_6070, &args![data]).u32();
        e.call(
            0x0048_8350,
            &args![holder.wrapping_add(0x30), subject, container],
        );
        let changes = e.call(0x004b_f220, &args![target]).u32();
        let mut node = e.call(0x0071_7e50, &args![container]).u32();
        if changes != 0 {
            while node != 0 {
                if e.call(0x0082_56d0, &args![node]).bool() {
                    break;
                }
                let slot = e.call(0x0068_15c0, &args![node]).u32();
                if e.mem.u32(slot) == 0 {
                    break;
                }
                let slot = e.call(0x0068_15c0, &args![node]).u32();
                let entry = e.mem.u32(slot);
                let mut form = 0;
                let item = e.mem.u32(entry.wrapping_add(4));
                if item != 0 && e.vcall(item, 0xe4, &args![]).bool() {
                    form = e.mem.u32(entry.wrapping_add(4));
                }
                if form != 0 {
                    let item = e.mem.u32(entry.wrapping_add(4));
                    let script = e.call(0x0048_26d0, &args![item]).u32();
                    if script != 0 {
                        let mut extra = 0u32;
                        let mut index: i32 = 0;
                        while index < e.mem.u32(entry) as i32 {
                            let block = e.call(OPERATOR_NEW, &args![0x20u32]).u32();
                            extra = if block == 0 {
                                0
                            } else {
                                e.call(0x0041_0360, &args![block]).u32()
                            };
                            e.call(0x0041_9ad0, &args![extra, 1u32]);
                            if extra != 0 && e.call(0x0041_8800, &args![extra]).u32() == 0 {
                                e.call(0x0041_9ed0, &args![extra, script]);
                                let own = e.call(0x0041_8800, &args![extra]).u32();
                                let prepared = e.call(0x005a_bf60, &args![own]).u32();
                                e.call(0x0041_9f80, &args![extra, prepared]);
                            }
                            index += 1;
                        }
                        e.vcall(target, 0x190, &args![form, extra, 1u32]);
                    } else {
                        let count = e.mem.u32(entry);
                        e.vcall(target, 0x190, &args![form, 0u32, count]);
                    }
                }
                node = e.call(0x0072_6070, &args![node]).u32();
            }
        }
        e.call(0x0048_1680, &args![container]);
    });
}

// Translated from 005e61b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSEntryPointFunction::AddActivateChoiceFunction` (engine map), cdecl,
/// five words: when `CheckFunctionArguments(2, 4, ...)` passes and the
/// first argument and `data` are not null: when `005f5930` of the data is
/// true, runs its result script (`00413f40`, when not null) through
/// `005ac1e0` with the second argument and three zeros; otherwise gives
/// the first argument `005ae3d0` with the address of a stack word holding
/// `data`.
pub fn bgs_entry_point_function_add_activate_choice_function(
    e: &mut Engine,
    subject: u32,
    function_type: u32,
    argument_count: u8,
    arguments: Ptr,
    data: Ptr,
) {
    if !bgs_entry_point_function_check_function_arguments(
        e,
        2,
        4,
        subject,
        function_type,
        argument_count,
        arguments,
        data,
    ) {
        return;
    }
    let first = e.mem.u32(arguments.addr());
    let second = e.mem.u32(arguments.addr() + 4);
    if first == 0 || data.is_null() {
        return;
    }
    if e.call(0x005f_5930, &args![data]).bool() {
        let script = e.call(RESULT_SCRIPT_OF, &args![data]).u32();
        if script != 0 {
            e.call(0x005a_c1e0, &args![script, second, 0u32, 0u32, 0u32]);
        }
    } else {
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), data.addr());
            e.call(0x005a_e3d0, &args![first, slot]);
        });
    }
}

// Translated from 005e6250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves the function data's type: slot 1 (+0x4, low byte) added as chunk
/// `EPFT` (`004858f0`, the byte zero-extended).
pub fn fn_005e6250(e: &mut Engine, this: Ptr) {
    let data_type = e.vcall(this.addr(), 0x4, &args![]).u8();
    e.call(ADD_BYTE_CHUNK, &args![CHUNK_EPFT, u32::from(data_type)]);
}

// Translated from 005e64b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSEntryPointFunctionDataTwoValue` constructor (its vtable `01046300`,
/// by RTTI): the base ([`fn_005e6420`]), the vtable, both values (+0x4,
/// +0x8) = 0.0 (`FLDZ`). Returns `this`.
pub fn fn_005e64b0(e: &mut Engine, this: Ptr) -> Ptr {
    fn_005e6420(e, this);
    e.mem.set_u32(this.addr(), TWO_VALUE_VTABLE);
    e.mem.set_f32(this.addr() + 4, 0.0);
    e.mem.set_f32(this.addr() + 8, 0.0);
    this
}

// Translated from 005e6670 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005e6670 - the exception-unwinding frame (FS:[0]) and the security-cookie word it pushes are not translated.
/// `BGSEntryPointFunctionDataOneValue::Duplicate` (engine map): a new
/// one-value data (8 bytes, [`fn_005e63f0`]; null when the allocation
/// fails) with this one's value (+0x4, `FLD`/`FSTP`). The value is stored
/// without a null test, as the exe does.
pub fn bgs_entry_point_function_data_one_value_duplicate(e: &mut Engine, this: Ptr) -> Ptr {
    let block = e.call(OPERATOR_NEW, &args![8u32]).u32();
    let copy = if block == 0 {
        Ptr::NULL
    } else {
        fn_005e63f0(e, Ptr::new(block))
    };
    let value = x87_quiet_bits(e.mem.u32(this.addr() + 4));
    e.mem.set_u32(copy.addr().wrapping_add(4), value);
    copy
}

// Translated from 005e6700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSEntryPointFunctionDataOneValue::Compare` (engine map): whether
/// `other` differs: true when it is not a one-value data
/// (`__RTDynamicCast`) or its value (+0x4) is not equal (`FUCOMPP`; NaN
/// differs).
pub fn bgs_entry_point_function_data_one_value_compare(
    e: &mut Engine,
    this: Ptr,
    other: Ptr,
) -> bool {
    let cast = e
        .call(
            DYNAMIC_CAST,
            &args![other, 0u32, TYPE_FUNCTION_DATA, TYPE_ONE_VALUE, 0u32],
        )
        .u32();
    if cast == 0 {
        return true;
    }
    let mine = x87_load32(e.mem.u32(this.addr() + 4));
    let theirs = x87_load32(e.mem.u32(cast + 4));
    theirs != mine
}

// Translated from 005e6760 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSEntryPointFunctionDataOneValue::Save` (engine map): the type
/// ([`fn_005e6250`]), then the value (+0x4) as chunk `EPFD` (`00485910`,
/// the float pushed through `FLD`/`FSTP`).
pub fn bgs_entry_point_function_data_one_value_save(e: &mut Engine, this: Ptr) {
    fn_005e6250(e, this);
    let value = x87_quiet_bits(e.mem.u32(this.addr() + 4));
    e.call(ADD_WORD_CHUNK, &args![CHUNK_EPFD, value]);
}

// Translated from 005e67f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005e67f0 - the exception-unwinding frame (FS:[0]) and the security-cookie word it pushes are not translated.
/// `BGSEntryPointFunctionDataTwoValue::Duplicate` (engine map): a new
/// two-value data (12 bytes, [`fn_005e64b0`]; null when the allocation
/// fails) with this one's values (+0x4, +0x8, `FLD`/`FSTP`), stored without
/// a null test.
pub fn bgs_entry_point_function_data_two_value_duplicate(e: &mut Engine, this: Ptr) -> Ptr {
    let block = e.call(OPERATOR_NEW, &args![0xcu32]).u32();
    let copy = if block == 0 {
        Ptr::NULL
    } else {
        fn_005e64b0(e, Ptr::new(block))
    };
    let first = x87_quiet_bits(e.mem.u32(this.addr() + 4));
    e.mem.set_u32(copy.addr().wrapping_add(4), first);
    let second = x87_quiet_bits(e.mem.u32(this.addr() + 8));
    e.mem.set_u32(copy.addr().wrapping_add(8), second);
    copy
}

// Translated from 005e6890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSEntryPointFunctionDataTwoValue::Compare` (engine map): true when
/// `other` is not a two-value data or either value (+0x4, then +0x8)
/// is not equal (`FUCOMPP`; NaN differs).
pub fn bgs_entry_point_function_data_two_value_compare(
    e: &mut Engine,
    this: Ptr,
    other: Ptr,
) -> bool {
    let cast = e
        .call(
            DYNAMIC_CAST,
            &args![other, 0u32, TYPE_FUNCTION_DATA, TYPE_TWO_VALUE, 0u32],
        )
        .u32();
    if cast == 0 {
        return true;
    }
    for offset in [4, 8] {
        let mine = x87_load32(e.mem.u32(this.addr() + offset));
        let theirs = x87_load32(e.mem.u32(cast + offset));
        if theirs != mine {
            return true;
        }
    }
    false
}

// Translated from 005e6900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSEntryPointFunctionDataTwoValue::Save` (engine map): the type
/// ([`fn_005e6250`]), then both values copied to an 8-byte stack buffer
/// (`FLD`/`FSTP`), swapped by `00462230` when `00401500` is true, added as
/// chunk `EPFD` (`00485990`, 8 bytes), and swapped back the same way.
pub fn bgs_entry_point_function_data_two_value_save(e: &mut Engine, this: Ptr) {
    fn_005e6250(e, this);
    e.with_stack(8, |e, buffer| {
        let first = x87_quiet_bits(e.mem.u32(this.addr() + 4));
        e.mem.set_u32(buffer.addr(), first);
        let second = x87_quiet_bits(e.mem.u32(this.addr() + 8));
        e.mem.set_u32(buffer.addr() + 4, second);
        if e.call(0x0040_1500, &args![]).bool() {
            e.call(0x0046_2230, &args![buffer]);
        }
        e.call(0x0048_5990, &args![CHUNK_EPFD, buffer, 8u32]);
        if e.call(0x0040_1500, &args![]).bool() {
            e.call(0x0046_2230, &args![buffer]);
        }
    });
}

// Translated from 005e6970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSEntryPointFunctionDataTwoValue::Load` (engine map): false unless the
/// chunk of `file` is 8 bytes; reads it into a stack buffer, swapped by
/// `00462230` when the file needs it, and copies its two floats to +0x4
/// and +0x8 (`FLD`/`FSTP`). Returns true.
pub fn bgs_entry_point_function_data_two_value_load(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let size = e.call(CHUNK_SIZE, &args![file]).u32();
    if size != 8 {
        return false;
    }
    e.with_stack(8, |e, buffer| {
        e.call(GET_CHUNK_DATA, &args![file, buffer, size]);
        if e.call(NEEDS_BYTE_SWAP, &args![file]).bool() {
            e.call(0x0046_2230, &args![buffer]);
        }
        let first = x87_quiet_bits(e.mem.u32(buffer.addr()));
        e.mem.set_u32(this.addr() + 4, first);
        let second = x87_quiet_bits(e.mem.u32(buffer.addr() + 4));
        e.mem.set_u32(this.addr() + 8, second);
    });
    true
}

// Translated from 005e69d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005e69d0 - the exception-unwinding frame (FS:[0]) and the security-cookie word it pushes are not translated.
/// `BGSEntryPointFunctionDataLeveledList::Duplicate` (engine map): a new
/// leveled-list data (8 bytes, [`fn_005e6510`]; null when the allocation
/// fails) with this one's list (+0x4), stored without a null test.
pub fn bgs_entry_point_function_data_leveled_list_duplicate(e: &mut Engine, this: Ptr) -> Ptr {
    let block = e.call(OPERATOR_NEW, &args![8u32]).u32();
    let copy = if block == 0 {
        Ptr::NULL
    } else {
        fn_005e6510(e, Ptr::new(block))
    };
    let list = e.mem.u32(this.addr() + 4);
    e.mem.set_u32(copy.addr().wrapping_add(4), list);
    copy
}

// Translated from 005e6a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSEntryPointFunctionDataLeveledList::Compare` (engine map): true when
/// `other` is not a leveled-list data or its list (+0x4) is another one.
pub fn bgs_entry_point_function_data_leveled_list_compare(
    e: &mut Engine,
    this: Ptr,
    other: Ptr,
) -> bool {
    let cast = e
        .call(
            DYNAMIC_CAST,
            &args![other, 0u32, TYPE_FUNCTION_DATA, TYPE_LEVELED_LIST, 0u32],
        )
        .u32();
    if cast == 0 {
        return true;
    }
    e.mem.u32(this.addr() + 4) != e.mem.u32(cast + 4)
}

// Translated from 005e6ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSEntryPointFunctionDataLeveledList::Save` (engine map): the type
/// ([`fn_005e6250`]), then, when there is a list (+0x4), its form id
/// (`0084e3a0`) as chunk `EPFD` (`00485910`).
pub fn bgs_entry_point_function_data_leveled_list_save(e: &mut Engine, this: Ptr) {
    fn_005e6250(e, this);
    let list = e.mem.u32(this.addr() + 4);
    if list != 0 {
        let id = e.call(FORM_ID, &args![list]).u32();
        e.call(ADD_WORD_CHUNK, &args![CHUNK_EPFD, id]);
    }
}

// Translated from 005e6cd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005e6cd0 - the exception-unwinding frame (FS:[0]) and the security-cookie word it pushes are not translated.
/// `BGSEntryPointFunctionDataActivateChoice::Duplicate` (engine map): a new
/// activate-choice data (0x60 bytes, [`fn_005e6540`]; null when the
/// allocation fails) given this one's label
/// ([`bgs_entry_point_function_data_activate_choice_set_label`]), a copy of
/// its result script (`005aa420` on the copy's `00413f40`, with this one's,
/// which is read first) and its word +0x5C; used without a null test.
pub fn bgs_entry_point_function_data_activate_choice_duplicate(e: &mut Engine, this: Ptr) -> Ptr {
    let block = e.call(OPERATOR_NEW, &args![0x60u32]).u32();
    let copy = if block == 0 {
        Ptr::NULL
    } else {
        fn_005e6540(e, Ptr::new(block))
    };
    let label = Ptr::new(e.mem.u32(this.addr() + 4));
    bgs_entry_point_function_data_activate_choice_set_label(e, copy, label);
    let source = e.call(RESULT_SCRIPT_OF, &args![this]).u32();
    let destination = e.call(RESULT_SCRIPT_OF, &args![copy]).u32();
    e.call(0x005a_a420, &args![destination, source]);
    let word = e.mem.u16(this.addr() + 0x5c);
    e.mem.set_u16(copy.addr().wrapping_add(0x5c), word);
    copy
}

// Translated from 005e6d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSEntryPointFunctionDataActivateChoice::Compare` (engine map): true
/// when `other` is not an activate-choice data, only one of the two has a
/// label (+0x4), both labels differ (`00408b20` not 0), the result scripts
/// differ (`005ab240` on this one's `00413f40` with `other`'s, read first)
/// or the words +0x5C differ.
pub fn bgs_entry_point_function_data_activate_choice_compare(
    e: &mut Engine,
    this: Ptr,
    other: Ptr,
) -> bool {
    let cast = e
        .call(
            DYNAMIC_CAST,
            &args![other, 0u32, TYPE_FUNCTION_DATA, TYPE_ACTIVATE_CHOICE, 0u32],
        )
        .u32();
    if cast == 0 {
        return true;
    }
    let mine = e.mem.u32(this.addr() + 4);
    let theirs = e.mem.u32(cast + 4);
    if (mine == 0) != (theirs == 0) {
        return true;
    }
    if mine != 0 && theirs != 0 && e.call(0x0040_8b20, &args![mine, theirs]).u32() != 0 {
        return true;
    }
    let their_script = e.call(RESULT_SCRIPT_OF, &args![cast]).u32();
    let my_script = e.call(RESULT_SCRIPT_OF, &args![this]).u32();
    if e.call(0x005a_b240, &args![my_script, their_script]).bool() {
        return true;
    }
    e.mem.u16(this.addr() + 0x5c) != e.mem.u16(cast + 0x5c)
}

// Translated from 005e6e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSEntryPointFunctionDataActivateChoice::Save` (engine map): the type
/// ([`fn_005e6250`]); the label (+0x4), when there is one and `strlen`
/// (`00ec6130`) is not 0, as chunk `EPF2` with its terminator (`004856f0`,
/// length from `0044a670` + 1); the word +0x5C (zero-extended) as chunk
/// `EPF3` (`00485950`); then the result script's save (`005abe00` on
/// `00413f40`).
pub fn bgs_entry_point_function_data_activate_choice_save(e: &mut Engine, this: Ptr) {
    fn_005e6250(e, this);
    let label = e.mem.u32(this.addr() + 4);
    if label != 0 && e.call(0x00ec_6130, &args![label]).u32() != 0 {
        let label = e.mem.u32(this.addr() + 4);
        let length = e.call(0x0044_a670, &args![label]).u32();
        let label = e.mem.u32(this.addr() + 4);
        e.call(
            0x0048_56f0,
            &args![CHUNK_EPF2, label, length.wrapping_add(1)],
        );
    }
    let word = u32::from(e.mem.u16(this.addr() + 0x5c));
    e.call(0x0048_5950, &args![CHUNK_EPF3, word]);
    let script = e.call(RESULT_SCRIPT_OF, &args![this]).u32();
    e.call(0x005a_be00, &args![script]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005e_64e0, fn_005e64e0(Ptr, u32) -> Ptr),
        entry!(0x005e_65d0, bgs_entry_point_function_data_activate_choice_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x005e_5b80,
            bgs_entry_point_function_check_function_arguments(u32, u8, u32, u32, u8, Ptr, Ptr)
                -> bool
        ),
        entry!(0x005e_5d20, fn_005e5d20(u32, u32, u8, Ptr, Ptr)),
        entry!(0x005e_6280, fn_005e6280(u32) -> Ptr),
        entry!(0x005e_63f0, fn_005e63f0(Ptr) -> Ptr),
        entry!(0x005e_6420, fn_005e6420(Ptr) -> Ptr),
        entry!(0x005e_6470, fn_005e6470(Ptr)),
        entry!(0x005e_6490, fn_005e6490(Ptr)),
        entry!(0x005e_6510, fn_005e6510(Ptr) -> Ptr),
        entry!(0x005e_6540, fn_005e6540(Ptr) -> Ptr),
        entry!(0x005e_6600, fn_005e6600(Ptr)),
        entry!(
            0x005e_6790,
            bgs_entry_point_function_data_one_value_load(Ptr, Ptr) -> bool
        ),
        entry!(
            0x005e_6b00,
            bgs_entry_point_function_data_leveled_list_load(Ptr, Ptr) -> bool
        ),
        entry!(
            0x005e_6b70,
            bgs_entry_point_function_data_leveled_list_init_item(Ptr, Ptr)
        ),
        entry!(0x005e_6c30, fn_005e6c30(Ptr)),
        entry!(
            0x005e_6c60,
            bgs_entry_point_function_data_activate_choice_set_label(Ptr, Ptr)
        ),
        entry!(
            0x005e_6ee0,
            bgs_entry_point_function_data_activate_choice_load(Ptr, Ptr) -> bool
        ),
        entry!(
            0x005e_7230,
            bgs_entry_point_function_data_activate_choice_init_item(Ptr, Ptr)
        ),
        entry!(0x005e_5c70, fn_005e5c70(u32, u32, u8, Ptr, Ptr)),
        entry!(0x005e_5cc0, fn_005e5cc0(u32, u32, u8, Ptr, Ptr)),
        entry!(0x005e_5d80, fn_005e5d80(u32, u32, u8, Ptr, Ptr)),
        entry!(0x005e_5df0, fn_005e5df0(u32, u32, u8, Ptr, Ptr)),
        entry!(0x005e_5ea0, fn_005e5ea0(u32, u32, u8, Ptr, Ptr)),
        entry!(0x005e_5ef0, fn_005e5ef0(u32, u32, u8, Ptr, Ptr)),
        entry!(0x005e_5f50, fn_005e5f50(u32, u32, u8, Ptr, Ptr)),
        entry!(
            0x005e_61b0,
            bgs_entry_point_function_add_activate_choice_function(u32, u32, u8, Ptr, Ptr)
        ),
        entry!(0x005e_6250, fn_005e6250(Ptr)),
        entry!(0x005e_64b0, fn_005e64b0(Ptr) -> Ptr),
        entry!(
            0x005e_6670,
            bgs_entry_point_function_data_one_value_duplicate(Ptr) -> Ptr
        ),
        entry!(
            0x005e_6700,
            bgs_entry_point_function_data_one_value_compare(Ptr, Ptr) -> bool
        ),
        entry!(
            0x005e_6760,
            bgs_entry_point_function_data_one_value_save(Ptr)
        ),
        entry!(
            0x005e_67f0,
            bgs_entry_point_function_data_two_value_duplicate(Ptr) -> Ptr
        ),
        entry!(
            0x005e_6890,
            bgs_entry_point_function_data_two_value_compare(Ptr, Ptr) -> bool
        ),
        entry!(
            0x005e_6900,
            bgs_entry_point_function_data_two_value_save(Ptr)
        ),
        entry!(
            0x005e_6970,
            bgs_entry_point_function_data_two_value_load(Ptr, Ptr) -> bool
        ),
        entry!(
            0x005e_69d0,
            bgs_entry_point_function_data_leveled_list_duplicate(Ptr) -> Ptr
        ),
        entry!(
            0x005e_6a60,
            bgs_entry_point_function_data_leveled_list_compare(Ptr, Ptr) -> bool
        ),
        entry!(
            0x005e_6ab0,
            bgs_entry_point_function_data_leveled_list_save(Ptr)
        ),
        entry!(
            0x005e_6cd0,
            bgs_entry_point_function_data_activate_choice_duplicate(Ptr) -> Ptr
        ),
        entry!(
            0x005e_6d80,
            bgs_entry_point_function_data_activate_choice_compare(Ptr, Ptr) -> bool
        ),
        entry!(
            0x005e_6e50,
            bgs_entry_point_function_data_activate_choice_save(Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_005e64e0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x005e_6490, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x005e_64e0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x005e_64e0)
                .collect();
            let mut expected = vec![(0x005e_6490, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_005e65d0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x005e_6600, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x005e_65d0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x005e_65d0)
                .collect();
            let mut expected = vec![(0x005e_6600, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod b028_tests {
    use super::*;

    fn quiet(e: &mut Engine, callees: &[u32]) {
        for callee in callees {
            e.register(*callee, |_, _| Ret::default());
        }
    }

    fn calls_to(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// An engine with the argument-count table (type 0: 1, type 1: 1,
    /// type 2: 2) and a data object whose slot 1 returns the word at +4.
    fn check_engine() -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x0119_7000, 0x1000);
        e.mem.set_u32(ARGUMENT_COUNTS, 1);
        e.mem.set_u32(ARGUMENT_COUNTS + 4, 1);
        e.mem.set_u32(ARGUMENT_COUNTS + 8, 2);
        quiet(&mut e, &[PRINT_WARNING]);
        e.put_vtable(0x0200_0000, &[0, 0x0090_0000]);
        e.register(0x0090_0000, |e, a| e.mem.u32(a[0] + 4).into_ret());
        let data = e.mem.alloc(8);
        e.mem.set_u32(data, 0x0200_0000);
        e.mem.set_u32(data + 4, 1);
        (e, data)
    }

    #[test]
    fn check_function_arguments_tests_each_condition() {
        let (mut e, data) = check_engine();
        let arguments = e.mem.alloc(8);
        e.mem.set_u32(arguments, 0x40);
        e.mem.set_u32(arguments + 4, 0x50);
        let check = |e: &mut Engine, words: [u32; 7]| e.call(0x005e_5b80, &words).bool();
        e.call_log = Some(vec![]);
        assert!(check(&mut e, [2, 1, 9, 2, 2, arguments, data]));
        assert!(check(&mut e, [2, 7, 9, 2, 2, arguments, 0]));
        assert!(e
            .call_log
            .take()
            .unwrap()
            .iter()
            .all(|c| c.0 != PRINT_WARNING));

        e.call_log = Some(vec![]);
        assert!(!check(&mut e, [2, 1, 9, 1, 2, arguments, data]));
        assert!(!check(&mut e, [2, 1, 9, 2, 0x103, arguments, data]));
        e.mem.set_u32(arguments + 4, 0);
        assert!(!check(&mut e, [2, 1, 9, 2, 2, arguments, data]));
        e.mem.set_u32(arguments + 4, 0x50);
        assert!(!check(&mut e, [2, 0x102, 9, 2, 2, arguments, data]));
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, PRINT_WARNING),
            vec![
                vec![BAD_TYPE_WARNING, 1, 2],
                vec![BAD_COUNT_WARNING, 3, 2],
                vec![ZERO_ARGUMENT_WARNING, 1],
                vec![BAD_DATA_TYPE_WARNING, 1, 2],
            ]
        );
    }

    #[test]
    fn fn_005e5d20_multiplies_the_target_by_the_data_value() {
        let (mut e, data) = check_engine();
        e.register(0x006b_9130, |_, _| 2.5f64.into_ret());
        let target = e.mem.alloc(4);
        e.mem.set_f32(target, 3.0);
        let arguments = e.mem.alloc(4);
        e.mem.set_u32(arguments, target);
        e.call(0x005e_5d20, &args![9u32, 0u32, 1u32, arguments, data]);
        assert_eq!(e.mem.f32(target), 7.5);
        // A failed check (wrong type) leaves it.
        e.call(0x005e_5d20, &args![9u32, 1u32, 1u32, arguments, data]);
        assert_eq!(e.mem.f32(target), 7.5);
    }

    #[test]
    fn the_data_constructors_and_destructors_store_their_vtables() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[
                0x005a_a0f0,
                0x005a_a1a0,
                0x0048_4ab0,
                0x0048_4490,
                OPERATOR_DELETE,
            ],
        );
        let this = e.mem.alloc(0x60);
        e.mem.write(this, &[0xcc; 0x60]);
        assert_eq!(e.call(0x005e_6420, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), FUNCTION_DATA_VTABLE);
        assert_eq!(e.call(0x005e_63f0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), ONE_VALUE_VTABLE);
        assert_eq!(e.mem.f32(this + 4), 0.0);
        e.mem.set_u32(this + 4, 0xcccc_cccc);
        assert_eq!(e.call(0x005e_6510, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), LEVELED_LIST_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0);
        e.call(0x005e_6470, &args![this]);
        assert_eq!(e.mem.u32(this), FUNCTION_DATA_VTABLE);
        e.mem.set_u32(this, 0);
        e.call(0x005e_6490, &args![this]);
        assert_eq!(e.mem.u32(this), FUNCTION_DATA_VTABLE);

        e.mem.write(this, &[0xcc; 0x60]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x005e_6540, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), ACTIVATE_CHOICE_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u16(this + 0x5c), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (0x005a_a0f0, vec![this + 8]),
                (0x0048_4ab0, vec![this + 8, 0]),
                (0x0048_4490, vec![this + 8]),
            ]
        );
        e.mem.set_u32(this + 4, 0x7000);
        e.call_log = Some(vec![]);
        e.call(0x005e_6600, &args![this]);
        assert_eq!(e.mem.u32(this), FUNCTION_DATA_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (OPERATOR_DELETE, vec![0x7000]),
                (0x005a_a1a0, vec![this + 8])
            ]
        );
    }

    #[test]
    fn fn_005e6280_makes_the_data_of_each_type() {
        let mut e = Engine::new();
        quiet(&mut e, &[0x005a_a0f0, 0x0048_4ab0, 0x0048_4490]);
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(0x005e_64b0, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        let one = e.call(0x005e_6280, &args![1u32]).u32();
        assert_eq!(e.mem.u32(one), ONE_VALUE_VTABLE);
        let two = e.call(0x005e_6280, &args![2u32]).u32();
        let three = e.call(0x005e_6280, &args![3u32]).u32();
        assert_eq!(e.mem.u32(three), LEVELED_LIST_VTABLE);
        let four = e.call(0x005e_6280, &args![4u32]).u32();
        assert_eq!(e.mem.u32(four), ACTIVATE_CHOICE_VTABLE);
        assert_eq!(e.call(0x005e_6280, &args![0u32]).u32(), 0);
        assert_eq!(e.call(0x005e_6280, &args![5u32]).u32(), 0);
        let log = e.call_log.take().unwrap();
        let sizes: Vec<u32> = calls_to(&log, OPERATOR_NEW).iter().map(|w| w[0]).collect();
        assert_eq!(sizes, vec![8, 0xc, 8, 0x60]);
        assert_eq!(calls_to(&log, 0x005e_64b0), vec![vec![two]]);

        // A failed allocation gives null without constructing.
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x005e_6280, &args![2u32]).u32(), 0);
        assert!(calls_to(&e.call_log.take().unwrap(), 0x005e_64b0).is_empty());
    }

    /// Doubles for the chunk size, reads (the chunk's bytes) and the swap.
    fn chunk_engine(chunk: u32, data: Vec<u8>, swap: bool) -> Engine {
        let mut e = Engine::new();
        let size = data.len() as u32;
        e.register_double(CHUNK_SIZE, move |_, _| size.into_ret());
        e.register_double(GET_TES_CHUNK, move |_, _| chunk.into_ret());
        let bytes = data.clone();
        e.register_double(GET_CHUNK_DATA, move |e, a| {
            let n = if a[2] == 0 {
                bytes.len()
            } else {
                (a[2] as usize).min(bytes.len())
            };
            e.mem.write(a[1], &bytes[..n]);
            Ret::default()
        });
        let bytes = data;
        e.register_double(GET_CHUNK_WORD, move |e, a| {
            e.mem.write(a[1], &bytes[..4]);
            Ret::default()
        });
        e.register_double(NEEDS_BYTE_SWAP, move |_, _| u32::from(swap).into_ret());
        e.register(SWAP_WORD_IN_PLACE, |e, a| {
            let v = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], v.swap_bytes());
            Ret::default()
        });
        e
    }

    #[test]
    fn one_value_and_leveled_list_load_four_bytes() {
        for swap in [false, true] {
            let mut e = chunk_engine(0, vec![0x11, 0x22, 0x33, 0x44], swap);
            let this = e.mem.alloc(8);
            let want = if swap { 0x1122_3344 } else { 0x4433_2211 };
            assert!(e.call(0x005e_6790, &args![this, 0x7000u32]).bool());
            assert_eq!(e.mem.u32(this + 4), want);
            e.mem.set_u32(this + 4, 0);
            assert!(e.call(0x005e_6b00, &args![this, 0x7000u32]).bool());
            assert_eq!(e.mem.u32(this + 4), want);
        }
        let mut e = chunk_engine(0, vec![1, 2, 3], false);
        let this = e.mem.alloc(8);
        assert!(!e.call(0x005e_6790, &args![this, 0x7000u32]).bool());
        assert!(!e.call(0x005e_6b00, &args![this, 0x7000u32]).bool());
        assert_eq!(e.mem.u32(this + 4), 0);
    }

    fn init_engine(cast_result: u32, has_name: bool) -> Engine {
        let mut e = Engine::new();
        quiet(&mut e, &[PRINT_WARNING]);
        e.register(0x0048_4e60, |_, _| 0x0900_0000u32.into_ret());
        e.register(0x0048_5d50, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            Ret::default()
        });
        e.register(0x0048_39c0, |_, a| (a[0] + 0x10).into_ret());
        e.register_double(0x00ec_43fb, move |_, _| cast_result.into_ret());
        e.register_double(0x0047_4cb0, move |_, _| u32::from(has_name).into_ret());
        e.register(FORM_ID, |_, _| 0x0002_0000u32.into_ret());
        e.put_vtable(0x0200_0000, &[0; 0x80]);
        e.mem.set_u32(0x0200_0000 + 0x130, 0x0090_0000);
        e.register(0x0090_0000, |_, _| 0x6000u32.into_ret());
        e
    }

    #[test]
    fn leveled_list_init_item_resolves_or_warns() {
        let mut e = init_engine(0x8000, true);
        let owner = e.mem.alloc(8);
        e.mem.set_u32(owner, 0x0200_0000);
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 0x55);
        e.call_log = Some(vec![]);
        e.call(0x005e_6b70, &args![this, owner]);
        assert_eq!(e.mem.u32(this + 4), 0x8000);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, 0x00ec_43fb),
            vec![vec![
                0x0100_0065,
                0,
                TYPE_TES_FORM,
                TYPE_LEVELED_LIST_TARGET,
                0
            ]]
        );
        assert!(calls_to(&log, PRINT_WARNING).is_empty());

        // Not found, with and without an editor id.
        for (has_name, want) in [
            (true, vec![NO_LEVELED_LIST_NAMED, 0x0100_0055, 0x6000]),
            (false, vec![NO_LEVELED_LIST_ID, 0x0100_0055, 0x0002_0000]),
        ] {
            let mut e = init_engine(0, has_name);
            let owner = e.mem.alloc(8);
            e.mem.set_u32(owner, 0x0200_0000);
            let this = e.mem.alloc(8);
            e.mem.set_u32(this + 4, 0x55);
            e.call_log = Some(vec![]);
            e.call(0x005e_6b70, &args![this, owner]);
            assert_eq!(e.mem.u32(this + 4), 0);
            let log = e.call_log.take().unwrap();
            assert_eq!(calls_to(&log, PRINT_WARNING), vec![want]);
        }

        // No id: nothing happens.
        let mut e = init_engine(0, false);
        let this = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        e.call(0x005e_6b70, &args![this, 0x7000u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn set_label_replaces_the_copy() {
        let mut e = Engine::new();
        quiet(&mut e, &[OPERATOR_DELETE]);
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(0x0044_a670, |e, a| {
            (e.mem.cstr(a[0]).len() as u32).into_ret()
        });
        e.register(0x0040_46f0, |e, a| {
            let text = e.mem.cstr(a[1]);
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 0x7000);
        let label = e.mem.alloc(8);
        e.mem.set_cstr(label, b"Take");
        e.call_log = Some(vec![]);
        e.call(0x005e_6c60, &args![this, label]);
        let copy = e.mem.u32(this + 4);
        assert_eq!(e.mem.cstr(copy), b"Take".to_vec());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![0x7000]]);
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![5]]);
        e.call(0x005e_6c60, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this + 4), 0);
    }

    #[test]
    fn fn_005e6c30_clears_the_word_and_prepares_the_script() {
        let mut e = Engine::new();
        quiet(&mut e, &[0x0048_4ab0, 0x0048_4490]);
        let this = e.mem.alloc(0x60);
        e.mem.set_u16(this + 0x5c, 0xffff);
        e.call_log = Some(vec![]);
        e.call(0x005e_6c30, &args![this]);
        assert_eq!(e.mem.u16(this + 0x5c), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (0x0048_4ab0, vec![this + 8, 0]),
                (0x0048_4490, vec![this + 8])
            ]
        );
    }

    fn choice_engine(chunk: u32, data: Vec<u8>, swap: bool) -> Engine {
        let mut e = chunk_engine(chunk, data, swap);
        e.map(0x011c_b000, 0x1000);
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(0x0050_0910, |_, a| a[0].into_ret());
        e.register(0x005e_28b0, |_, a| a[0].into_ret());
        e.register(0x005d_43c0, |_, a| (a[0] + 0x40).into_ret());
        e.register(0x005d_43e0, |_, a| (a[0] + 0x44).into_ret());
        e.register(0x0050_0940, |_, a| (a[0] + 0x10).into_ret());
        quiet(
            &mut e,
            &[
                0x0090_5820,
                0x005e_2a20,
                0x0040_37f0,
                0x0041_4220,
                0x0040_3d30,
                0x005a_ae70,
                GET_CHUNK_DATA_OV2,
                OPERATOR_DELETE,
                0x0044_a670,
                0x0040_46f0,
            ],
        );
        e
    }

    #[test]
    fn activate_choice_load_adds_referenced_objects() {
        for (chunk, field) in [(CHUNK_SCRO, 8u32), (CHUNK_SCRV, 0xc)] {
            let mut e = choice_engine(chunk, vec![0x78, 0x56, 0x34, 0x12], false);
            let this = e.mem.alloc(0x60);
            e.call_log = Some(vec![]);
            assert!(e.call(0x005e_6ee0, &args![this, 0x7000u32]).bool());
            let log = e.call_log.take().unwrap();
            let entry = calls_to(&log, 0x0050_0910)[0][0];
            assert_eq!(e.mem.u32(entry + field), 0x1234_5678);
            let added = calls_to(&log, 0x0090_5820);
            assert_eq!(added[0][0], this + 8 + 0x40);
            // The pushed word is the address of a stack slot holding the entry.
            assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x10]]);
        }
    }

    #[test]
    fn activate_choice_load_reads_variables() {
        let mut e = choice_engine(CHUNK_SLSD, vec![0; 24], false);
        let this = e.mem.alloc(0x60);
        e.call_log = Some(vec![]);
        assert!(e.call(0x005e_6ee0, &args![this, 0x7000u32]).bool());
        let variable = e.mem.u32(CURRENT_VARIABLE);
        assert_ne!(variable, 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x005e_2a20), vec![vec![variable, 0x7000]]);
        assert_eq!(
            calls_to(&log, 0x0090_5820),
            vec![vec![this + 8 + 0x44, CURRENT_VARIABLE]]
        );

        // SCVR names the current variable; without one it is skipped.
        let mut e = choice_engine(CHUNK_SCVR, b"fVar\0".to_vec(), false);
        e.mem.set_u32(CURRENT_VARIABLE, 0x0050_0000);
        let this = e.mem.alloc(0x60);
        e.call_log = Some(vec![]);
        e.call(0x005e_6ee0, &args![this, 0x7000u32]);
        let log = e.call_log.take().unwrap();
        let named = calls_to(&log, 0x0040_37f0);
        assert_eq!(named.len(), 1);
        assert_eq!((named[0][0], named[0][2]), (0x0050_0018, 0));
        assert_eq!(calls_to(&log, GET_CHUNK_DATA)[0][2], 0x200);
        e.mem.set_u32(CURRENT_VARIABLE, 0);
        e.call_log = Some(vec![]);
        e.call(0x005e_6ee0, &args![this, 0x7000u32]);
        assert!(calls_to(&e.call_log.take().unwrap(), 0x0040_37f0).is_empty());
    }

    #[test]
    fn activate_choice_load_reads_the_script_data_and_the_label() {
        for swap in [false, true] {
            let mut e = choice_engine(CHUNK_SCHR, vec![0; 20], swap);
            let this = e.mem.alloc(0x60);
            e.call_log = Some(vec![]);
            e.call(0x005e_6ee0, &args![this, 0x7000u32]);
            let log = e.call_log.take().unwrap();
            assert_eq!(calls_to(&log, GET_CHUNK_DATA)[0][1..], [this + 0x18, 0]);
            assert_eq!(calls_to(&log, 0x0041_4220).len(), usize::from(swap));
        }
        let mut e = choice_engine(CHUNK_SCDA, vec![1, 2, 3], false);
        let this = e.mem.alloc(0x60);
        e.call_log = Some(vec![]);
        e.call(0x005e_6ee0, &args![this, 0x7000u32]);
        let log = e.call_log.take().unwrap();
        let buffer = calls_to(&log, 0x0040_3d30)[0][0];
        assert_eq!(calls_to(&log, 0x0040_3d30), vec![vec![buffer, 0, 3]]);
        assert_eq!(calls_to(&log, 0x005a_ae70), vec![vec![this + 8, 3, buffer]]);

        let mut e = choice_engine(CHUNK_EPF2, b"Open\0".to_vec(), false);
        let this = e.mem.alloc(0x60);
        e.call_log = Some(vec![]);
        e.call(0x005e_6ee0, &args![this, 0x7000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0044_a670).len(), 1);

        let mut e = choice_engine(CHUNK_EPF3, vec![2, 0], false);
        let this = e.mem.alloc(0x60);
        e.call_log = Some(vec![]);
        e.call(0x005e_6ee0, &args![this, 0x7000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, GET_CHUNK_DATA_OV2),
            vec![vec![0x7000, this + 0x5c]]
        );

        // Any other chunk does nothing and still returns true.
        let mut e = choice_engine(0x1234_5678, vec![], false);
        let this = e.mem.alloc(0x60);
        e.call_log = Some(vec![]);
        assert!(e.call(0x005e_6ee0, &args![this, 0x7000u32]).bool());
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }

    #[test]
    fn activate_choice_init_item_warns_after_errors() {
        for errors in [0u32, 3] {
            let mut e = Engine::new();
            let reads = std::rc::Rc::new(std::cell::Cell::new(0u32));
            let r = reads.clone();
            e.register_double(0x0046_e8a0, move |_, _| {
                r.set(r.get() + 1);
                let value = if r.get() == 1 { 7u32 } else { errors };
                value.into_ret()
            });
            quiet(
                &mut e,
                &[0x0043_b2b0, 0x004f_ffe0, 0x005a_b400, PRINT_WARNING],
            );
            e.register(FORM_ID, |_, _| 0x0003_0000u32.into_ret());
            e.put_vtable(0x0200_0000, &[0; 0x80]);
            e.mem.set_u32(0x0200_0000 + 0x130, 0x0090_0000);
            e.register(0x0090_0000, |_, _| 0x6000u32.into_ret());
            let owner = e.mem.alloc(8);
            e.mem.set_u32(owner, 0x0200_0000);
            let this = e.mem.alloc(0x60);
            e.call_log = Some(vec![]);
            e.call(0x005e_7230, &args![this, owner]);
            let log = e.call_log.take().unwrap();
            assert_eq!(calls_to(&log, 0x0043_b2b0), vec![vec![1], vec![0]]);
            assert_eq!(calls_to(&log, 0x004f_ffe0), vec![vec![0], vec![7]]);
            assert_eq!(calls_to(&log, 0x005a_b400), vec![vec![this + 8, owner]]);
            let warnings = calls_to(&log, PRINT_WARNING);
            if errors == 0 {
                assert!(warnings.is_empty());
            } else {
                assert_eq!(
                    warnings,
                    vec![vec![RESULT_SCRIPT_ERRORS, 0x6000, 0x0003_0000]]
                );
            }
        }
    }
}

#[cfg(test)]
mod n036_tests {
    use super::*;

    fn quiet(e: &mut Engine, callees: &[u32]) {
        for callee in callees {
            e.register(*callee, |_, _| Ret::default());
        }
    }

    fn calls_to(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// An engine whose argument-count table gives types 0, 1 and 2 one, one
    /// and two arguments, and a data object of `data_type` (slot 1 answers
    /// the word at +4).
    fn function_engine(data_type: u32) -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x0119_7000, 0x1000);
        e.mem.set_u32(ARGUMENT_COUNTS, 1);
        e.mem.set_u32(ARGUMENT_COUNTS + 4, 1);
        e.mem.set_u32(ARGUMENT_COUNTS + 8, 2);
        quiet(&mut e, &[PRINT_WARNING]);
        e.put_vtable(0x0200_0000, &[0, 0x0090_0000]);
        e.register(0x0090_0000, |e, a| e.mem.u32(a[0] + 4).into_ret());
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(data, 0x0200_0000);
        e.mem.set_u32(data + 4, data_type);
        (e, data)
    }

    /// A float target and the one-word argument array pointing at it.
    fn float_target(e: &mut Engine, value: f32) -> (u32, u32) {
        let target = e.mem.alloc(4);
        e.mem.set_f32(target, value);
        let arguments = e.mem.alloc(8);
        e.mem.set_u32(arguments, target);
        (target, arguments)
    }

    #[test]
    fn fn_005e5c70_sets_the_target_to_the_value() {
        let (mut e, data) = function_engine(1);
        e.register(DATA_VALUE, |_, _| 2.5f64.into_ret());
        let (target, arguments) = float_target(&mut e, 3.0);
        e.call(0x005e_5c70, &args![9u32, 0u32, 1u32, arguments, data]);
        assert_eq!(e.mem.f32(target), 2.5);
        e.mem.set_f32(target, 3.0);
        e.call(0x005e_5c70, &args![9u32, 1u32, 1u32, arguments, data]);
        assert_eq!(e.mem.f32(target), 3.0);
    }

    #[test]
    fn fn_005e5cc0_adds_the_value() {
        let (mut e, data) = function_engine(1);
        e.register(DATA_VALUE, |_, _| 2.5f64.into_ret());
        let (target, arguments) = float_target(&mut e, 3.0);
        e.call(0x005e_5cc0, &args![9u32, 0u32, 1u32, arguments, data]);
        assert_eq!(e.mem.f32(target), 5.5);
        // A one-value data fails the two-value check of 005e5d80.
        e.call(0x005e_5d80, &args![9u32, 0u32, 1u32, arguments, data]);
        assert_eq!(e.mem.f32(target), 5.5);
    }

    #[test]
    fn fn_005e5d80_adds_a_random_float_between_the_values() {
        let (mut e, data) = function_engine(2);
        e.register(DATA_VALUE, |_, _| 1.0f64.into_ret());
        e.register(DATA_SECOND_VALUE, |_, _| 4.0f64.into_ret());
        e.register(0x0047_6b70, |_, a| {
            let low = f32::from_bits(a[0]);
            let high = f32::from_bits(a[1]);
            f64::from(low + high / 16.0).into_ret()
        });
        let (target, arguments) = float_target(&mut e, 3.0);
        e.call(0x005e_5d80, &args![9u32, 0u32, 1u32, arguments, data]);
        assert_eq!(e.mem.f32(target), 4.25);
    }

    #[test]
    fn fn_005e5df0_adds_the_value_times_the_subjects_count() {
        let (mut e, data) = function_engine(2);
        e.register(DATA_VALUE, |_, _| 1.5f64.into_ret());
        e.register(DATA_SECOND_VALUE, |_, _| (-2.9f64).into_ret());
        e.put_vtable(0x0201_0000, &[0, 0, 0x0090_0008]);
        e.register(0x0090_0008, |_, a| (a[1] as i32 * 3).into_ret());
        let subject = e.mem.alloc(0xb0);
        e.mem.set_u32(subject + 0xa4, 0x0201_0000);
        let (target, arguments) = float_target(&mut e, 1.0);
        e.call_log = Some(vec![]);
        e.call(0x005e_5df0, &args![subject, 0u32, 1u32, arguments, data]);
        // -2.9 truncates to -2; -2 * 3 * 1.5 + 1.
        assert_eq!(e.mem.f32(target), -8.0);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, 0x0090_0008),
            vec![vec![subject + 0xa4, (-2i32) as u32]]
        );
    }

    #[test]
    fn fn_005e5ea0_and_005e5ef0_store_the_callee_result() {
        let (mut e, _) = function_engine(0);
        e.register(0x0040_8840, |_, a| {
            f64::from(f32::from_bits(a[0]) * 2.0).into_ret()
        });
        let (target, arguments) = float_target(&mut e, 3.0);
        e.call(0x005e_5ea0, &args![9u32, 0u32, 1u32, arguments, 0u32]);
        assert_eq!(e.mem.f32(target), 6.0);
        e.call(0x005e_5ef0, &args![9u32, 0u32, 1u32, arguments, 0u32]);
        assert_eq!(e.mem.f32(target), -12.0);
        // Wrong type: nothing.
        e.call(0x005e_5ef0, &args![9u32, 2u32, 1u32, arguments, 0u32]);
        assert_eq!(e.mem.f32(target), -12.0);
    }

    #[test]
    fn fn_005e5f50_adds_the_container_items_to_the_target() {
        let (mut e, data) = function_engine(3);
        quiet(
            &mut e,
            &[
                0x0048_1610,
                0x0048_1680,
                0x0048_8350,
                0x0041_9ad0,
                0x0041_9ed0,
                0x0041_9f80,
            ],
        );
        e.register(0x0072_6070, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(0x0068_15c0, |_, a| a[0].into_ret());
        e.register(0x0082_56d0, |_, _| false.into_ret());
        e.register(0x004b_f220, |_, _| 1u32.into_ret());
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(0x0041_0360, |_, a| a[0].into_ret());
        e.register(0x0041_8800, |_, _| 0u32.into_ret());
        e.register(0x005a_bf60, |_, a| (a[0] + 1).into_ret());
        // Items: slot 57 answers true; the first has a script.
        e.put_vtable(0x0202_0000, &[0; 0x80]);
        e.mem.set_u32(0x0202_0000 + 0xe4, 0x0090_00e4);
        e.register(0x0090_00e4, |_, _| true.into_ret());
        let scripted = e.mem.alloc(8);
        e.mem.set_u32(scripted, 0x0202_0000);
        let plain = e.mem.alloc(8);
        e.mem.set_u32(plain, 0x0202_0000);
        e.register_double(0x0048_26d0, move |_, a| {
            u32::from(a[0] == scripted).wrapping_mul(0x7700).into_ret()
        });
        let first = e.mem.alloc(8);
        e.mem.set_u32(first, 2);
        e.mem.set_u32(first + 4, scripted);
        let second = e.mem.alloc(8);
        e.mem.set_u32(second, 5);
        e.mem.set_u32(second + 4, plain);
        let node2 = e.mem.alloc(8);
        e.mem.set_u32(node2, second);
        let node1 = e.mem.alloc(8);
        e.mem.set_u32(node1, first);
        e.mem.set_u32(node1 + 4, node2);
        e.register_double(0x0071_7e50, move |_, _| node1.into_ret());
        // The target: slot 100 records.
        e.put_vtable(0x0203_0000, &[0; 0x80]);
        e.mem.set_u32(0x0203_0000 + 0x190, 0x0090_0190);
        e.register(0x0090_0190, |_, _| Ret::default());
        let target = e.mem.alloc(8);
        e.mem.set_u32(target, 0x0203_0000);
        let arguments = e.mem.alloc(4);
        e.mem.set_u32(arguments, target);
        e.call_log = Some(vec![]);
        e.call(0x005e_5f50, &args![0x6000u32, 1u32, 1u32, arguments, data]);
        let log = e.call_log.take().unwrap();
        let extras = calls_to(&log, 0x0041_0360);
        assert_eq!(extras.len(), 2);
        assert_eq!(
            calls_to(&log, 0x0090_0190),
            vec![
                vec![target, scripted, extras[1][0], 1],
                vec![target, plain, 0, 5]
            ]
        );
        assert_eq!(
            calls_to(&log, 0x0041_9ed0),
            vec![vec![extras[0][0], 0x7700], vec![extras[1][0], 0x7700]]
        );
        assert_eq!(calls_to(&log, 0x0048_8350)[0][..2], [3 + 0x30, 0x6000]);
        assert_eq!(calls_to(&log, 0x0048_1680).len(), 1);

        // No inventory changes: nothing is added.
        e.register(0x004b_f220, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x005e_5f50, &args![0x6000u32, 1u32, 1u32, arguments, data]);
        assert!(calls_to(&e.call_log.take().unwrap(), 0x0090_0190).is_empty());
    }

    #[test]
    fn add_activate_choice_function_runs_or_adds_the_choice() {
        let (mut e, data) = function_engine(4);
        quiet(&mut e, &[0x005a_c1e0, 0x005a_e3d0]);
        e.register(RESULT_SCRIPT_OF, |_, a| (a[0] + 8).into_ret());
        let arguments = e.mem.alloc(8);
        e.mem.set_u32(arguments, 0x5000);
        e.mem.set_u32(arguments + 4, 0x5100);
        for runs in [true, false] {
            e.register_double(0x005f_5930, move |_, _| runs.into_ret());
            e.call_log = Some(vec![]);
            e.call(0x005e_61b0, &args![9u32, 2u32, 2u32, arguments, data]);
            let log = e.call_log.take().unwrap();
            if runs {
                assert_eq!(
                    calls_to(&log, 0x005a_c1e0),
                    vec![vec![data + 8, 0x5100, 0, 0, 0]]
                );
            } else {
                let added = calls_to(&log, 0x005a_e3d0);
                assert_eq!(added[0][0], 0x5000);
                assert!(calls_to(&log, 0x005a_c1e0).is_empty());
            }
        }
        // A null first argument: nothing.
        e.mem.set_u32(arguments, 0);
        e.call_log = Some(vec![]);
        e.call(0x005e_61b0, &args![9u32, 2u32, 2u32, arguments, data]);
        assert!(calls_to(&e.call_log.take().unwrap(), 0x005f_5930).is_empty());
    }

    #[test]
    fn fn_005e6250_saves_the_type_byte() {
        let (mut e, data) = function_engine(0x1ff);
        quiet(&mut e, &[ADD_BYTE_CHUNK]);
        e.call_log = Some(vec![]);
        e.call(0x005e_6250, &args![data]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, ADD_BYTE_CHUNK), vec![vec![CHUNK_EPFT, 0xff]]);
    }

    #[test]
    fn fn_005e64b0_builds_a_two_value_data() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xc);
        e.mem.write(this, &[0xcc; 0xc]);
        assert_eq!(e.call(0x005e_64b0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), TWO_VALUE_VTABLE);
        assert_eq!((e.mem.u32(this + 4), e.mem.u32(this + 8)), (0, 0));
    }

    fn duplicate_engine() -> Engine {
        let mut e = Engine::new();
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        quiet(&mut e, &[0x005a_a0f0, 0x0048_4ab0, 0x0048_4490]);
        e
    }

    #[test]
    fn the_duplicates_copy_their_values() {
        let mut e = duplicate_engine();
        let this = e.mem.alloc(0xc);
        e.mem.set_f32(this + 4, 1.5);
        e.mem.set_u32(this + 8, 0x7f80_0001);
        let one = e.call(0x005e_6670, &args![this]).u32();
        assert_eq!(e.mem.u32(one), ONE_VALUE_VTABLE);
        assert_eq!(e.mem.f32(one + 4), 1.5);
        let two = e.call(0x005e_67f0, &args![this]).u32();
        assert_eq!(e.mem.u32(two), TWO_VALUE_VTABLE);
        assert_eq!(e.mem.f32(two + 4), 1.5);
        // FLD/FSTP quiets the signalling NaN.
        assert_eq!(e.mem.u32(two + 8), 0x7fc0_0001);
        e.mem.set_u32(this + 4, 0x4444);
        let list = e.call(0x005e_69d0, &args![this]).u32();
        assert_eq!(e.mem.u32(list), LEVELED_LIST_VTABLE);
        assert_eq!(e.mem.u32(list + 4), 0x4444);
    }

    #[test]
    fn activate_choice_duplicate_copies_label_script_and_word() {
        let mut e = duplicate_engine();
        quiet(&mut e, &[OPERATOR_DELETE, 0x005a_a420]);
        e.register(RESULT_SCRIPT_OF, |_, a| (a[0] + 8).into_ret());
        e.register(0x0044_a670, |e, a| {
            (e.mem.cstr(a[0]).len() as u32).into_ret()
        });
        e.register(0x0040_46f0, |e, a| {
            let text = e.mem.cstr(a[1]);
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        let this = e.mem.alloc(0x60);
        let label = e.mem.alloc(8);
        e.mem.set_cstr(label, b"Use");
        e.mem.set_u32(this + 4, label);
        e.mem.set_u16(this + 0x5c, 0x1234);
        e.call_log = Some(vec![]);
        let copy = e.call(0x005e_6cd0, &args![this]).u32();
        assert_eq!(e.mem.u32(copy), ACTIVATE_CHOICE_VTABLE);
        assert_eq!(e.mem.cstr(e.mem.u32(copy + 4)), b"Use".to_vec());
        assert_eq!(e.mem.u16(copy + 0x5c), 0x1234);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x005a_a420), vec![vec![copy + 8, this + 8]]);
        assert_eq!(
            calls_to(&log, RESULT_SCRIPT_OF),
            vec![vec![this], vec![copy]]
        );
    }

    /// A cast double that answers the object when its word +0x10 is the
    /// target type, else null.
    fn compare_engine() -> Engine {
        let mut e = Engine::new();
        e.register(DYNAMIC_CAST, |e, a| {
            let kind = e.mem.u32(a[0] + 0x10);
            (if kind == a[3] { a[0] } else { 0 }).into_ret()
        });
        e
    }

    #[test]
    fn value_compares_test_type_and_values() {
        let mut e = compare_engine();
        let this = e.mem.alloc(0x14);
        let other = e.mem.alloc(0x14);
        e.mem.set_f32(this + 4, 1.0);
        e.mem.set_f32(this + 8, 2.0);
        e.mem.set_f32(other + 4, 1.0);
        e.mem.set_f32(other + 8, 2.0);
        e.mem.set_u32(other + 0x10, TYPE_ONE_VALUE);
        assert!(!e.call(0x005e_6700, &args![this, other]).bool());
        assert!(e.call(0x005e_6890, &args![this, other]).bool());
        e.mem.set_u32(other + 0x10, TYPE_TWO_VALUE);
        assert!(e.call(0x005e_6700, &args![this, other]).bool());
        assert!(!e.call(0x005e_6890, &args![this, other]).bool());
        e.mem.set_f32(other + 8, 3.0);
        assert!(e.call(0x005e_6890, &args![this, other]).bool());
        e.mem.set_u32(other + 0x10, TYPE_ONE_VALUE);
        e.mem.set_u32(this + 4, 0x7fc0_0000);
        e.mem.set_u32(other + 4, 0x7fc0_0000);
        assert!(e.call(0x005e_6700, &args![this, other]).bool());
        // Leveled list: the same list or another one.
        e.mem.set_u32(other + 0x10, TYPE_LEVELED_LIST);
        assert!(!e.call(0x005e_6a60, &args![this, other]).bool());
        e.mem.set_u32(other + 4, 9);
        assert!(e.call(0x005e_6a60, &args![this, other]).bool());
        e.mem.set_u32(other + 0x10, 0);
        assert!(e.call(0x005e_6a60, &args![this, other]).bool());
    }

    #[test]
    fn activate_choice_compare_tests_labels_scripts_and_word() {
        let mut e = compare_engine();
        e.register(RESULT_SCRIPT_OF, |_, a| (a[0] + 8).into_ret());
        e.register(0x0040_8b20, |e, a| {
            u32::from(e.mem.cstr(a[0]) != e.mem.cstr(a[1])).into_ret()
        });
        e.register(0x005a_b240, |e, a| {
            (e.mem.u32(a[0]) != e.mem.u32(a[1])).into_ret()
        });
        let this = e.mem.alloc(0x60);
        let other = e.mem.alloc(0x60);
        e.mem.set_u32(other + 0x10, TYPE_ACTIVATE_CHOICE);
        e.mem.set_u32(this + 0x10, TYPE_ACTIVATE_CHOICE);
        let compare = |e: &mut Engine| e.call(0x005e_6d80, &args![this, other]).bool();
        assert!(!compare(&mut e));
        let a = e.mem.alloc(8);
        e.mem.set_cstr(a, b"A");
        let b = e.mem.alloc(8);
        e.mem.set_cstr(b, b"B");
        e.mem.set_u32(this + 4, a);
        assert!(compare(&mut e));
        e.mem.set_u32(other + 4, b);
        assert!(compare(&mut e));
        e.mem.set_u32(other + 4, a);
        assert!(!compare(&mut e));
        e.mem.set_u32(this + 8, 1);
        assert!(compare(&mut e));
        e.mem.set_u32(this + 8, 0);
        e.mem.set_u16(other + 0x5c, 2);
        assert!(compare(&mut e));
        e.mem.set_u32(other + 0x10, 0);
        e.mem.set_u16(other + 0x5c, 0);
        assert!(compare(&mut e));
    }

    #[test]
    fn the_saves_write_type_and_values() {
        let (mut e, _) = function_engine(1);
        quiet(
            &mut e,
            &[
                ADD_BYTE_CHUNK,
                ADD_WORD_CHUNK,
                0x0048_56f0,
                0x0048_5950,
                0x005a_be00,
            ],
        );
        e.register(FORM_ID, |_, a| (a[0] | 0x0100_0000).into_ret());
        e.register(RESULT_SCRIPT_OF, |_, a| (a[0] + 8).into_ret());
        e.register(0x00ec_6130, |e, a| {
            (e.mem.cstr(a[0]).len() as u32).into_ret()
        });
        e.register(0x0044_a670, |e, a| {
            (e.mem.cstr(a[0]).len() as u32).into_ret()
        });
        // A one-value data (its vtable from the engine above).
        let one = e.mem.alloc(0x60);
        e.mem.set_u32(one, 0x0200_0000);
        e.mem.set_u32(one + 4, 0x7f80_0001);
        e.call_log = Some(vec![]);
        e.call(0x005e_6760, &args![one]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, ADD_WORD_CHUNK),
            vec![vec![CHUNK_EPFD, 0x7fc0_0001]]
        );
        assert_eq!(calls_to(&log, ADD_BYTE_CHUNK)[0][0], CHUNK_EPFT);
        // Leveled list: the form id when there is a list.
        e.mem.set_u32(one + 4, 0x55);
        e.call_log = Some(vec![]);
        e.call(0x005e_6ab0, &args![one]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, ADD_WORD_CHUNK),
            vec![vec![CHUNK_EPFD, 0x0100_0055]]
        );
        // Activate choice: label, word, script.
        let label = e.mem.alloc(8);
        e.mem.set_cstr(label, b"Go");
        e.mem.set_u32(one + 4, label);
        e.mem.set_u16(one + 0x5c, 0x8001);
        e.call_log = Some(vec![]);
        e.call(0x005e_6e50, &args![one]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, 0x0048_56f0),
            vec![vec![CHUNK_EPF2, label, 3]]
        );
        assert_eq!(calls_to(&log, 0x0048_5950), vec![vec![CHUNK_EPF3, 0x8001]]);
        assert_eq!(calls_to(&log, 0x005a_be00), vec![vec![one + 8]]);
        // An empty label is not saved.
        e.mem.set_cstr(label, b"");
        e.call_log = Some(vec![]);
        e.call(0x005e_6e50, &args![one]);
        assert!(calls_to(&e.call_log.take().unwrap(), 0x0048_56f0).is_empty());
    }

    #[test]
    fn two_value_save_and_load_swap_through_the_buffer() {
        let (mut e, _) = function_engine(2);
        quiet(&mut e, &[ADD_BYTE_CHUNK]);
        e.map(0x0300_0000, 0x1000);
        e.register(0x0040_1500, |_, _| true.into_ret());
        e.register(0x0046_2230, |e, a| {
            let first = e.mem.u32(a[0]);
            let second = e.mem.u32(a[0] + 4);
            e.mem.set_u32(a[0], second);
            e.mem.set_u32(a[0] + 4, first);
            Ret::default()
        });
        e.register(0x0048_5990, |e, a| {
            let first = e.mem.u32(a[1]);
            let second = e.mem.u32(a[1] + 4);
            e.mem.set_u32(0x0300_0000, first);
            e.mem.set_u32(0x0300_0004, second);
            Ret::default()
        });
        let this = e.mem.alloc(0xc);
        e.mem.set_u32(this, 0x0200_0000);
        e.mem.set_f32(this + 4, 1.0);
        e.mem.set_f32(this + 8, 2.0);
        e.call_log = Some(vec![]);
        e.call(0x005e_6900, &args![this]);
        assert_eq!(e.mem.f32(0x0300_0000), 2.0);
        assert_eq!(e.mem.f32(0x0300_0004), 1.0);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0046_2230).len(), 2);
        assert_eq!(calls_to(&log, 0x0048_5990)[0][2], 8);

        // Load: 8 bytes, swapped when the file needs it.
        let bytes: Vec<u8> = [3.0f32, 4.0f32]
            .iter()
            .flat_map(|v| v.to_le_bytes())
            .collect();
        for swap in [false, true] {
            let data = bytes.clone();
            e.register_double(CHUNK_SIZE, |_, _| 8u32.into_ret());
            e.register_double(GET_CHUNK_DATA, move |e, a| {
                e.mem.write(a[1], &data);
                Ret::default()
            });
            e.register_double(NEEDS_BYTE_SWAP, move |_, _| swap.into_ret());
            assert!(e.call(0x005e_6970, &args![this, 0x7000u32]).bool());
            let want = if swap { (4.0, 3.0) } else { (3.0, 4.0) };
            assert_eq!((e.mem.f32(this + 4), e.mem.f32(this + 8)), want);
        }
        e.register(CHUNK_SIZE, |_, _| 4u32.into_ret());
        assert!(!e.call(0x005e_6970, &args![this, 0x7000u32]).bool());
    }
}
