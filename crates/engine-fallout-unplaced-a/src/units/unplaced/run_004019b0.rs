//! `(unplaced)/run_004019b0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00401a10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00401a10 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x77 (0x20 bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0101_15b4`) is `BGSHungerStage`, so this is the constructor of `BGSHungerStage`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_00401a10(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.mem.set_u32(base, 0x0101_15b4);
    e.call(0x0040_19f0, &args![base + 0x18]);
    e.call(0x004f_15a0, &args![this, 0x77u32]);
    e.call(0x0040_1b20, &args![this]);
    this
}

// Translated from 004020f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004020f0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x78 (0x20 bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0101_16fc`) is `BGSSleepDeprevationStage`, so this is the constructor of `BGSSleepDeprevationStage`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_004020f0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.mem.set_u32(base, 0x0101_16fc);
    e.call(0x0040_19f0, &args![base + 0x18]);
    e.call(0x004f_15a0, &args![this, 0x78u32]);
    e.call(0x0040_1b20, &args![this]);
    this
}

// Translated from 00402820 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00402820 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x2f (0xd8 bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0101_1964`) is `AlchemyItem`, so this is the constructor of `AlchemyItem`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_00402820(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0040_2b40, &args![this]);
    e.call(0x0048_a3d0, &args![base + 0x4c]);
    e.call(0x0040_2c20, &args![base + 0x6c]);
    e.call(0x0047_a130, &args![base + 0x78]);
    e.call(0x0048_cc50, &args![base + 0x88]);
    e.call(0x0048_eac0, &args![base + 0x94]);
    e.call(0x0047_9320, &args![base + 0x9c]);
    e.call(0x0047_5350, &args![base + 0xa4]);
    e.call(0x0047_b4f0, &args![base + 0xac]);
    e.mem.set_u32(base, 0x0101_1964);
    e.mem.set_u32(base + 0x30, 0x0101_191c);
    e.mem.set_u32(base + 0x3c, 0x0101_1900);
    e.mem.set_u32(base + 0x4c, 0x0101_18dc);
    e.mem.set_u32(base + 0x6c, 0x0101_18bc);
    e.mem.set_u32(base + 0x78, 0x0101_18a8);
    e.mem.set_u32(base + 0x88, 0x0101_1894);
    e.mem.set_u32(base + 0x94, 0x0101_1880);
    e.mem.set_u32(base + 0x9c, 0x0101_186c);
    e.mem.set_u32(base + 0xa4, 0x0101_1858);
    e.mem.set_u32(base + 0xac, 0x0101_1844);
    e.call(0x0040_2c20, &args![base + 0xcc]);
    e.call(0x004f_15a0, &args![this, 0x2fu32]);
    e.call(0x0040_2ea0, &args![this]);
    this
}

/// The C runtime's `sqrt(double)` (`00ec6054`, `cdecl`, the result in
/// `ST0`).
const SQRT: u32 = 0x00ec_6054;

// Translated from 004019b0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `cdecl` (one float): passes the float through the x87 stack to
/// [`fn_004019d0`] and returns its result in `ST0`.
pub fn fn_004019b0(e: &mut Engine, value: u32) -> f64 {
    let passed = (x87_load32(value) as f32).to_bits();
    fn_004019d0(e, passed)
}

// Translated from 004019d0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `cdecl` (one float): the square root of the float as a double
/// (`sqrt`, `00ec6054`), rounded to a float (`FSTP float` then `FLD float`)
/// and returned in `ST0`. (Ghidra shows a `this`; `ECX` is never read.)
pub fn fn_004019d0(e: &mut Engine, value: u32) -> f64 {
    let root = e.call(SQRT, &args![x87_load32(value)]).f64();
    f64::from(root as f32)
}

// Translated from 00401a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `00401ac0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00401a90(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0040_1ac0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 00402170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `004021a0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00402170(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0040_21a0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// A global array of stage forms: its count and its heap array of form pointers.
#[derive(Clone, Copy)]
struct StageList {
    count: u32,
    array: u32,
}

/// The `BGSHungerStage` list (`011c3198` count, `011c319c` array).
const HUNGER_STAGES: StageList = StageList {
    count: 0x011c_3198,
    array: 0x011c_319c,
};
/// The `BGSSleepDeprevationStage` list (`011c31b0` count, `011c31b4` array).
const SLEEP_STAGES: StageList = StageList {
    count: 0x011c_31b0,
    array: 0x011c_31b4,
};

/// `00500940` (`ECX`): answers `this + 0x18` (a stage's data).
const STAGE_DATA_00500940: u32 = 0x0050_0940;
/// `00843170` (`ECX`): clears the two words at +0 and +4.
const CLEAR_PAIR_00843170: u32 = 0x0084_3170;
/// The allocator (`00401000`, `cdecl`, one word: the size).
const ALLOCATE: u32 = 0x0040_1000;
/// `operator delete` (`00401030`, `cdecl`, one word).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `009611e0` (`ECX`): the word at +0x18 (a stage's threshold).
const STAGE_THRESHOLD_009611E0: u32 = 0x0096_11e0;
/// `004013e0` (`ECX`): whether form flag 8 is set (`AL`).
const FORM_FLAG_8_SET: u32 = 0x0040_13e0;
/// `00484ab0` (`ECX`, one stack word): sets or clears form flag 8.
const SET_FORM_FLAG_8: u32 = 0x0048_4ab0;
/// `00441110` (`ECX`): the word at +0x1c (here the stage's spell, a form id until resolved).
const WORD_1C_00441110: u32 = 0x0044_1110;
/// `0050f9a0` (`ECX`, one stack word): stores the word at +0x1c.
const SET_WORD_1C_0050F9A0: u32 = 0x0050_f9a0;
/// `TESForm::GetFile` (Xbox PDB, `00484e60`, `ECX`, one stack word).
const GET_FILE: u32 = 0x0048_4e60;
/// `TESForm::AddCompileIndex` (Xbox PDB, `00485d50`, `cdecl`: address of a form id, file).
const ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// `004839c0` (`cdecl`, one word): the form of a form id, or null.
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `__RTDynamicCast` (`00ec43fb`, `cdecl`: object, vftable delta, source type, target type,
/// is-reference).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// RTTI type descriptors `.?AVTESForm@@` and `.?AVSpellItem@@`.
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_SPELL_ITEM: u32 = 0x0118_3060;
/// `TESFile::GetTESForm` (Xbox PDB, `00472660`, `ECX` = file): the record's form type (`AL`).
const GET_TES_FORM: u32 = 0x0047_2660;
/// `TESForm::LoadForm` (Xbox PDB, `00485110`, `ECX` = form, one stack word: the file).
const LOAD_FORM: u32 = 0x0048_5110;
/// `TESFile::GetTESChunk` (Xbox PDB, `004726b0`, `ECX` = file): the chunk's code, 0 at the end.
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// `00401660` (`ECX` = file): the current chunk's size.
const CHUNK_SIZE: u32 = 0x0040_1660;
/// `TESFile::GetChunkData_ov3` (Xbox PDB, `00472890`, `ECX` = file, two stack words:
/// destination, size).
const GET_CHUNK_DATA_SIZED: u32 = 0x0047_2890;
/// `TESForm::LoadData` (Xbox PDB, `004861f0`, `ECX` = form, three stack words: file,
/// destination, start).
const LOAD_DATA: u32 = 0x0048_61f0;
/// `00401680` (`ECX` = file): whether the file's data needs its byte order swapped (`AL`).
const NEEDS_SWAP: u32 = 0x0040_1680;
/// `00462230` (`ECX`): swaps the byte order of a two-word record.
const SWAP_PAIR_00462230: u32 = 0x0046_2230;
/// `004726f0` (`ECX` = file): steps to the next chunk; false at the record's end (`AL`).
const NEXT_CHUNK: u32 = 0x0047_26f0;
/// `00484730` (`ECX`, one stack word): sets or clears form flag 2.
const SET_FORM_FLAG_2: u32 = 0x0048_4730;
/// `EffectItemList::CanBePoison` (engine map, `00405da0`, `ECX`), `AL`.
const CAN_BE_POISON: u32 = 0x0040_5da0;
/// Chunk codes the stage loaders read (the four characters, first byte lowest).
const CHUNK_DATA: u32 = 0x4154_4144;
const CHUNK_EDID: u32 = 0x4449_4445;
const CHUNK_OBND: u32 = 0x444e_424f;

// Translated from 004019f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map: the stage data constructor (`ECX`, no stack words; the stage
/// constructors run it on +0x18): clears its two words (`00843170`) and answers `this`.
pub fn fn_004019f0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(CLEAR_PAIR_00843170, &args![this]);
    this
}

// Translated from 00401b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `InitializeData` slot (+0x14) of both stage vtables (`010115b4`, `010116fc`; `ECX`):
/// clears the two words of the stage data `00500940` answers (`00843170`).
pub fn fn_00401b20(e: &mut Engine, this: Ptr) {
    let data = e.call(STAGE_DATA_00500940, &args![this]).u32();
    e.call(CLEAR_PAIR_00843170, &args![data]);
}

// Translated from 00401b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSHungerStage`'s `ClearData` slot (+0x18; `ECX`): takes the stage out of the hunger
/// stage list ([`fn_00401f20`]).
pub fn fn_00401b40(e: &mut Engine, this: Ptr) {
    fn_00401f20(e, this);
}

// Translated from 00402200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSleepDeprevationStage`'s `ClearData` slot (+0x18; `ECX`): takes the stage out of the
/// sleep stage list ([`fn_00402650`]).
pub fn fn_00402200(e: &mut Engine, this: Ptr) {
    fn_00402650(e, this);
}

/// The destructor body shared by the two stage classes: its vtable, the `ClearData` body,
/// then `TESForm::~TESForm` (`00483630`).
fn stage_destructor(e: &mut Engine, this: Ptr, vtable: u32, clear: fn(&mut Engine, Ptr)) {
    e.mem.set_u32(this.addr(), vtable);
    clear(e, this);
    e.call(0x0048_3630, &args![this]);
}

// Translated from 00401ac0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00401ac0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00ef8ae8, state variable) is not translated
/// The `BGSHungerStage` destructor body (`ECX`; the scalar deleting destructor `00401a90`
/// runs it): the vtable `010115b4`, [`fn_00401b40`], then `TESForm::~TESForm`.
pub fn fn_00401ac0(e: &mut Engine, this: Ptr) {
    stage_destructor(e, this, 0x0101_15b4, fn_00401b40);
}

// Translated from 004021a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004021a0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00ef8ae8, state variable) is not translated
/// The `BGSSleepDeprevationStage` destructor body (`ECX`; the scalar deleting destructor
/// `00402170` runs it): the vtable `010116fc`, [`fn_00402200`], then `TESForm::~TESForm`.
pub fn fn_004021a0(e: &mut Engine, this: Ptr) {
    stage_destructor(e, this, 0x0101_16fc, fn_00402200);
}

/// A stage list's entry `index`, or 0 past its count (the exe's lower bound test,
/// `index < 0` unsigned, never holds).
fn stage_at(e: &mut Engine, list: StageList, index: u32) -> u32 {
    if index < e.global::<u32>(list.count) {
        let array = e.global::<u32>(list.array);
        e.mem.u32(array.wrapping_add(index.wrapping_mul(4)))
    } else {
        0
    }
}

// Translated from 004020b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`cdecl`, one word): the hunger stage list's entry `index`, or null.
pub fn fn_004020b0(e: &mut Engine, index: u32) -> u32 {
    stage_at(e, HUNGER_STAGES, index)
}

// Translated from 004027e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`cdecl`, one word): the sleep stage list's entry `index`, or null.
pub fn fn_004027e0(e: &mut Engine, index: u32) -> u32 {
    stage_at(e, SLEEP_STAGES, index)
}

// Translated from 004024e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`cdecl`, one word): null for a count of 0; otherwise a new array of
/// `count` words (`00401000` with `count * 4`, all ones when that overflows), each cleared
/// (the allocation is not checked).
pub fn fn_004024e0(e: &mut Engine, count: u32) -> u32 {
    if count == 0 {
        return 0;
    }
    let size = u64::from(count) * 4;
    let size = if size > u64::from(u32::MAX) {
        u32::MAX
    } else {
        size as u32
    };
    let array = e.call(ALLOCATE, &args![size]).u32();
    for index in 0..count {
        e.mem.set_u32(array.wrapping_add(index.wrapping_mul(4)), 0);
    }
    array
}

/// Puts `stage` into a stage list, before the first entry whose data word (`00500940`'s
/// answer, read for the stage and the entry each step) is below the stage's; at the end when
/// none is. The new array has room for the count + 1 as a byte; the count becomes that byte.
fn stage_insert(e: &mut Engine, list: StageList, stage: Ptr) {
    let count = e.global::<u32>(list.count);
    let new_count = count.wrapping_add(1) as u8;
    let new_array = fn_004024e0(e, u32::from(new_count));
    let mut next = 0u32;
    let mut inserted = false;
    let mut index = 0u32;
    while index < e.global::<u32>(list.count) {
        let stage_data = e.call(STAGE_DATA_00500940, &args![stage]).u32();
        let entry = e
            .mem
            .u32(e.global::<u32>(list.array).wrapping_add(index * 4));
        let entry_data = e.call(STAGE_DATA_00500940, &args![entry]).u32();
        if e.mem.u32(stage_data) > e.mem.u32(entry_data) && !inserted {
            e.mem
                .set_u32(new_array.wrapping_add(next * 4), stage.addr());
            next += 1;
            inserted = true;
        }
        let entry = e
            .mem
            .u32(e.global::<u32>(list.array).wrapping_add(index * 4));
        e.mem.set_u32(new_array.wrapping_add(next * 4), entry);
        next += 1;
        index += 1;
    }
    if !inserted {
        e.mem
            .set_u32(new_array.wrapping_add(next * 4), stage.addr());
    }
    let old_array = e.global::<u32>(list.array);
    e.call(OPERATOR_DELETE, &args![old_array]);
    e.set_global::<u32>(list.count, u32::from(new_count));
    e.set_global::<u32>(list.array, new_array);
}

// Translated from 00401e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`cdecl`, one word, a hunger stage; `00401b60` runs it): inserts the
/// stage into the hunger stage list in order of its data word, highest first (see
/// `stage_insert`), replacing the list's array (the old one deleted).
pub fn fn_00401e20(e: &mut Engine, stage: Ptr) {
    stage_insert(e, HUNGER_STAGES, stage);
}

// Translated from 00402550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`cdecl`, one word, a sleep stage; `00402220` runs it): inserts the
/// stage into the sleep stage list in order of its data word, highest first (see
/// `stage_insert`), replacing the list's array (the old one deleted).
pub fn fn_00402550(e: &mut Engine, stage: Ptr) {
    stage_insert(e, SLEEP_STAGES, stage);
}

/// Takes `stage` out of a stage list: nothing for an empty list or one without it;
/// otherwise a new array one shorter ([`fn_004024e0`]; when it is null nothing is copied)
/// gets every other entry, the old array is deleted and the list takes the new one.
fn stage_remove(e: &mut Engine, list: StageList, stage: Ptr) {
    if e.global::<u32>(list.count) == 0 {
        return;
    }
    let mut found = false;
    let mut index = 0u32;
    while !found && index < e.global::<u32>(list.count) {
        let entry = e
            .mem
            .u32(e.global::<u32>(list.array).wrapping_add(index * 4));
        if entry == stage.addr() {
            found = true;
        }
        index += 1;
    }
    if !found {
        return;
    }
    let new_count = e.global::<u32>(list.count).wrapping_sub(1);
    let new_array = fn_004024e0(e, new_count);
    if new_array != 0 {
        let mut next = 0u32;
        let mut index = 0u32;
        while index < e.global::<u32>(list.count) {
            let entry = e
                .mem
                .u32(e.global::<u32>(list.array).wrapping_add(index * 4));
            if entry != stage.addr() {
                let entry = e
                    .mem
                    .u32(e.global::<u32>(list.array).wrapping_add(index * 4));
                e.mem.set_u32(new_array.wrapping_add(next * 4), entry);
                next += 1;
            }
            index += 1;
        }
    }
    let old_array = e.global::<u32>(list.array);
    e.call(OPERATOR_DELETE, &args![old_array]);
    e.set_global::<u32>(list.count, new_count);
    e.set_global::<u32>(list.array, new_array);
}

// Translated from 00401f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`cdecl`, one word): takes a hunger stage out of the hunger stage list
/// (see `stage_remove`).
pub fn fn_00401f20(e: &mut Engine, stage: Ptr) {
    stage_remove(e, HUNGER_STAGES, stage);
}

// Translated from 00402650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`cdecl`, one word): takes a sleep stage out of the sleep stage list
/// (see `stage_remove`).
pub fn fn_00402650(e: &mut Engine, stage: Ptr) {
    stage_remove(e, SLEEP_STAGES, stage);
}

/// The first stage of a list, in list order, whose threshold (`009611e0`, as an unsigned
/// 64-bit `FILD`) is not above `value` (`FCOMPP`, `C0` clear: a NaN `value` takes none); null
/// when none.
fn stage_for_value(
    e: &mut Engine,
    list: StageList,
    at: fn(&mut Engine, u32) -> u32,
    value: u32,
) -> u32 {
    let mut found = 0u32;
    let mut index = 0u32;
    while index < e.global::<u32>(list.count) && found == 0 {
        let stage = at(e, index);
        if stage != 0 {
            let threshold = e.call(STAGE_THRESHOLD_009611E0, &args![stage]).u32();
            if x87_load32(value) >= f64::from(threshold) {
                found = stage;
            }
        }
        index += 1;
    }
    found
}

// Translated from 00402030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`cdecl`, one float word): the hunger stage for a value (see
/// `stage_for_value`).
pub fn fn_00402030(e: &mut Engine, value: u32) -> u32 {
    stage_for_value(e, HUNGER_STAGES, fn_004020b0, value)
}

// Translated from 00402760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`cdecl`, one float word): the sleep stage for a value (see
/// `stage_for_value`).
pub fn fn_00402760(e: &mut Engine, value: u32) -> u32 {
    stage_for_value(e, SLEEP_STAGES, fn_004027e0, value)
}

/// The `InitItem` body shared by the two stage classes: unless form flag 8 is set, the spell
/// word (`00441110`), when nonzero, gets the compile index of the form's file and is replaced
/// by its form cast to `SpellItem` (`0050f9a0`); then form flag 8 is set and the stage put
/// into its list.
fn stage_init_item(e: &mut Engine, this: Ptr, insert: fn(&mut Engine, Ptr)) {
    if e.call(FORM_FLAG_8_SET, &args![this]).bool() {
        return;
    }
    e.with_stack(4, |e, id_at| {
        let id = e.call(WORD_1C_00441110, &args![this]).u32();
        e.mem.set_u32(id_at.addr(), id);
        if id != 0 {
            let file = e.call(GET_FILE, &args![this, u32::MAX]).u32();
            e.call(ADD_COMPILE_INDEX, &args![id_at, file]);
            let id = e.mem.u32(id_at.addr());
            let form = e.call(LOOKUP_FORM, &args![id]).u32();
            let spell = e
                .call(
                    DYNAMIC_CAST,
                    &args![form, 0u32, TYPE_TES_FORM, TYPE_SPELL_ITEM, 0u32],
                )
                .u32();
            e.call(SET_WORD_1C_0050F9A0, &args![this, spell]);
        }
    });
    e.call(SET_FORM_FLAG_8, &args![this, 1u32]);
    insert(e, this);
}

// Translated from 00401b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSHungerStage`'s `InitItem` slot (+0x88; `ECX`): see `stage_init_item`, into the hunger
/// stage list ([`fn_00401e20`]).
pub fn fn_00401b60(e: &mut Engine, this: Ptr) {
    stage_init_item(e, this, fn_00401e20);
}

// Translated from 00402220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSleepDeprevationStage`'s `InitItem` slot (+0x88; `ECX`): see `stage_init_item`, into
/// the sleep stage list ([`fn_00402550`]).
pub fn fn_00402220(e: &mut Engine, this: Ptr) {
    stage_init_item(e, this, fn_00402550);
}

/// The `Load` body shared by the two stage classes (`ECX` and one stack word, the file):
/// false for another form type; otherwise vtable slot +0x18 (`ClearData`),
/// `TESForm::LoadForm`, form flag 8 cleared, then each chunk: `DATA` (8 bytes of stage data
/// through `TESForm::LoadData`), `EDID` (a stack buffer of the chunk's size, read with at most
/// 0x200 bytes, given to vtable slot +0x134), `OBND` (vtable slot +0xe0 with the file); after a
/// `DATA` chunk the data's two words are byte-swapped (`00462230` on +0x18) when the file
/// needs it. Then true.
fn stage_load(e: &mut Engine, this: Ptr, file: Ptr, form_type: u8) -> bool {
    if e.call(GET_TES_FORM, &args![file]).u32() as u8 != form_type {
        return false;
    }
    let base = this.addr();
    e.vcall(base, 0x18, &args![]);
    e.call(LOAD_FORM, &args![this, file]);
    e.call(SET_FORM_FLAG_8, &args![this, 0u32]);
    loop {
        let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            CHUNK_DATA => {
                let data = e.call(STAGE_DATA_00500940, &args![this]).u32();
                e.call(LOAD_DATA, &args![this, file, data, 8u32]);
            }
            CHUNK_EDID => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                e.with_stack(size, |e, name| {
                    e.call(GET_CHUNK_DATA_SIZED, &args![file, name, 0x200u32]);
                    e.vcall(base, 0x134, &args![name]);
                });
            }
            CHUNK_OBND => {
                e.vcall(base, 0xe0, &args![file]);
            }
            _ => {}
        }
        if chunk == CHUNK_DATA && e.call(NEEDS_SWAP, &args![file]).bool() {
            e.call(SWAP_PAIR_00462230, &args![base + 0x18]);
        }
        if !e.call(NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    true
}

// Translated from 00401cd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00401cd0 - the /GS cookie check (00ec408c) is not translated; the EDID buffer from __alloca_probe_16 (00ec5ec0) is a with_stack block of the chunk size (its 16-byte alignment padding, and the stack growing with each EDID chunk, are not modelled)
/// `BGSHungerStage`'s `Load` slot (+0x20; `RET 4`): see `stage_load`, form type 0x77.
pub fn fn_00401cd0(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    stage_load(e, this, file, 0x77)
}

// Translated from 00402390 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00402390 - the /GS cookie check (00ec408c) is not translated; the EDID buffer from __alloca_probe_16 (00ec5ec0) is a with_stack block of the chunk size (its 16-byte alignment padding, and the stack growing with each EDID chunk, are not modelled)
/// `BGSSleepDeprevationStage`'s `Load` slot (+0x20; `RET 4`): see `stage_load`, form type
/// 0x78.
pub fn fn_00402390(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    stage_load(e, this, file, 0x78)
}

// Translated from 00402990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map, a slot of several form vtables (`01011a2c`, `01011c14`, ...; `ECX` and
/// one byte word, `RET 4`): form flag 2 set or cleared (`00484730`) with the byte.
pub fn fn_00402990(e: &mut Engine, this: Ptr, on: u8) {
    e.call(SET_FORM_FLAG_2, &args![this, u32::from(on)]);
}

// Translated from 004029d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`ECX` and one stack word, `RET 4`): 1 when the `AlchemyItem` flag
/// byte at +0xbc has a bit of `mask`, else 0.
pub fn fn_004029d0(e: &mut Engine, this: Ptr, mask: u32) -> u32 {
    u32::from(u32::from(e.mem.u8(this.addr() + 0xbc)) & mask != 0)
}

// Translated from 00402ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +0 of `AlchemyItem`'s vtable at +0x3c (`01011900`; `ECX` is that +0x3c): flag 4 of
/// the item ([`fn_004029d0`] on `this - 0x3c`), 0 or 1.
pub fn fn_00402ae0(e: &mut Engine, this: Ptr) -> u32 {
    fn_004029d0(e, Ptr::new(this.addr().wrapping_sub(0x3c)), 4)
}

// Translated from 00402a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +4 of `AlchemyItem`'s vtable at +0x3c (`01011900`; `ECX` is that +0x3c): true when
/// the item has flag 2 ([`fn_004029d0`] on `this - 0x3c`) and `EffectItemList::CanBePoison`
/// (on `this`) answers false.
pub fn fn_00402a90(e: &mut Engine, this: Ptr) -> bool {
    if fn_004029d0(e, Ptr::new(this.addr().wrapping_sub(0x3c)), 2) as u8 == 0 {
        return false;
    }
    e.call(CAN_BE_POISON, &args![this]).u8() == 0
}

/// RTTI type descriptors `.?AVBGSHungerStage@@` and `.?AVBGSSleepDeprevationStage@@`.
const TYPE_HUNGER_STAGE: u32 = 0x0118_307c;
const TYPE_SLEEP_STAGE: u32 = 0x0118_30a0;
/// `TESForm::CopyAllComponents` (Xbox PDB, `004851b0`, `ECX`, one stack word: the source).
const COPY_ALL_COMPONENTS: u32 = 0x0048_51b0;
/// `TESForm::CompareAllComponents` (Xbox PDB, `00485270`, `ECX`, one stack word), `AL`.
const COMPARE_ALL_COMPONENTS: u32 = 0x0048_5270;
/// `00401460` (`cdecl`, three words: destination, source, size).
const COPY_BYTES_00401460: u32 = 0x0040_1460;
/// The CRT's `memcmp` (`00ec4835`, `cdecl`).
const MEMCMP: u32 = 0x00ec_4835;

/// The `Copy` body shared by the two stage classes: cast `source` from `TESForm` to the
/// class; when that answers a stage, copy its components and the 8 bytes of its stage data.
fn stage_copy(e: &mut Engine, this: Ptr, source: Ptr, class: u32) {
    let stage = e
        .call(
            DYNAMIC_CAST,
            &args![source, 0u32, TYPE_TES_FORM, class, 0u32],
        )
        .u32();
    if stage == 0 {
        return;
    }
    e.call(COPY_ALL_COMPONENTS, &args![this, stage]);
    let from = e.call(STAGE_DATA_00500940, &args![stage]).u32();
    let to = e.call(STAGE_DATA_00500940, &args![this]).u32();
    e.call(COPY_BYTES_00401460, &args![to, from, 8u32]);
}

/// The `Compare` body shared by the two stage classes: true (different) unless `other` is
/// of the class, its components compare equal and the 8 bytes of the stage datas match.
fn stage_compare(e: &mut Engine, this: Ptr, other: Ptr, class: u32) -> bool {
    let stage = e
        .call(
            DYNAMIC_CAST,
            &args![other, 0u32, TYPE_TES_FORM, class, 0u32],
        )
        .u32();
    if stage == 0 {
        return true;
    }
    if e.call(COMPARE_ALL_COMPONENTS, &args![this, stage]).u8() != 0 {
        return true;
    }
    let theirs = e.call(STAGE_DATA_00500940, &args![stage]).u32();
    let mine = e.call(STAGE_DATA_00500940, &args![this]).u32();
    e.call(MEMCMP, &args![mine, theirs, 8u32]).u32() != 0
}

// Translated from 00401bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSHungerStage`'s `Copy` slot (+0x108 of its vtable `010115b4`; unnamed in the engine
/// map; `ECX` and one stack word, `RET 4`): when `source` casts (`__RTDynamicCast` from
/// `TESForm`) to a `BGSHungerStage`, `TESForm::CopyAllComponents` from it, then `00401460`
/// copies the 8 bytes of its stage data (`00500940`) into this one's.
pub fn fn_00401bf0(e: &mut Engine, this: Ptr, source: Ptr) {
    stage_copy(e, this, source, TYPE_HUNGER_STAGE);
}

// Translated from 00401c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSHungerStage`'s `Compare` slot (+0x10c of its vtable `010115b4`; unnamed in the engine
/// map; `ECX` and one stack word, `RET 4`): true (different) when `other` is not a
/// `BGSHungerStage`, when `TESForm::CompareAllComponents` answers true, or when `memcmp`
/// finds the 8 bytes of the two stage datas (this one's first) different; else false.
pub fn fn_00401c50(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    stage_compare(e, this, other, TYPE_HUNGER_STAGE)
}

// Translated from 004022b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSleepDeprevationStage`'s `Copy` slot (+0x108 of its vtable `010116fc`; unnamed in
/// the engine map; `ECX` and one stack word, `RET 4`): the same as [`fn_00401bf0`] with the
/// `BGSSleepDeprevationStage` type.
pub fn fn_004022b0(e: &mut Engine, this: Ptr, source: Ptr) {
    stage_copy(e, this, source, TYPE_SLEEP_STAGE);
}

// Translated from 00402310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSleepDeprevationStage`'s `Compare` slot (+0x10c of its vtable `010116fc`; unnamed in
/// the engine map; `ECX` and one stack word, `RET 4`): the same as [`fn_00401c50`] with the
/// `BGSSleepDeprevationStage` type.
pub fn fn_00402310(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    stage_compare(e, this, other, TYPE_SLEEP_STAGE)
}

// Translated from 004029b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +0x144 of `AlchemyItem`'s vtable (`01011964`; also called by `00403a50`; unnamed in
/// the engine map; `ECX` only, plain `RET`): 1 when flag 1 of the item's flag byte at +0xbc
/// is clear ([`fn_004029d0`]), else 0 (`NEG`/`SBB`/`ADD 1` on its `AL`).
pub fn fn_004029b0(e: &mut Engine, this: Ptr) -> u32 {
    let set = fn_004029d0(e, this, 1) as u8;
    u32::from(set == 0)
}

// Translated from 004029f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +0x148 of `AlchemyItem`'s vtable (`01011964`; also called by `00403a40`; unnamed in
/// the engine map; `ECX` and one byte word, `RET 4`): sets flag 1 of the flag byte at +0xbc
/// when `on` is 0 and clears it otherwise ([`fn_00402a20`] with mask 1).
pub fn fn_004029f0(e: &mut Engine, this: Ptr, on: u8) {
    fn_00402a20(e, this, 1, u8::from(on == 0));
}

// Translated from 00402a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` and two stack words, `RET 8`): ORs the low byte of
/// `mask` into the `AlchemyItem` flag byte at +0xbc when `set` is nonzero, else ANDs the
/// byte with `!mask`.
pub fn fn_00402a20(e: &mut Engine, this: Ptr, mask: u32, set: u8) {
    let address = this.addr() + 0xbc;
    let flags = u32::from(e.mem.u8(address));
    let flags = if set != 0 {
        flags | mask
    } else {
        flags & !mask
    };
    e.mem.set_u8(address, flags as u8);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0040_1bf0, fn_00401bf0(Ptr, Ptr)),
        entry!(0x0040_1c50, fn_00401c50(Ptr, Ptr) -> bool),
        entry!(0x0040_22b0, fn_004022b0(Ptr, Ptr)),
        entry!(0x0040_2310, fn_00402310(Ptr, Ptr) -> bool),
        entry!(0x0040_29b0, fn_004029b0(Ptr) -> u32),
        entry!(0x0040_29f0, fn_004029f0(Ptr, u8)),
        entry!(0x0040_2a20, fn_00402a20(Ptr, u32, u8)),
        entry!(0x0040_1a10, fn_00401a10(Ptr) -> Ptr),
        entry!(0x0040_20f0, fn_004020f0(Ptr) -> Ptr),
        entry!(0x0040_2820, fn_00402820(Ptr) -> Ptr),
        entry!(0x004019b0, fn_004019b0(u32) -> f64),
        entry!(0x004019d0, fn_004019d0(u32) -> f64),
        entry!(0x0040_1a90, fn_00401a90(Ptr, u32) -> Ptr),
        entry!(0x0040_2170, fn_00402170(Ptr, u32) -> Ptr),
        entry!(0x0040_19f0, fn_004019f0(Ptr) -> Ptr),
        entry!(0x0040_1b20, fn_00401b20(Ptr)),
        entry!(0x0040_1b40, fn_00401b40(Ptr)),
        entry!(0x0040_2200, fn_00402200(Ptr)),
        entry!(0x0040_1ac0, fn_00401ac0(Ptr)),
        entry!(0x0040_21a0, fn_004021a0(Ptr)),
        entry!(0x0040_20b0, fn_004020b0(u32) -> u32),
        entry!(0x0040_27e0, fn_004027e0(u32) -> u32),
        entry!(0x0040_24e0, fn_004024e0(u32) -> u32),
        entry!(0x0040_1e20, fn_00401e20(Ptr)),
        entry!(0x0040_2550, fn_00402550(Ptr)),
        entry!(0x0040_1f20, fn_00401f20(Ptr)),
        entry!(0x0040_2650, fn_00402650(Ptr)),
        entry!(0x0040_2030, fn_00402030(u32) -> u32),
        entry!(0x0040_2760, fn_00402760(u32) -> u32),
        entry!(0x0040_1b60, fn_00401b60(Ptr)),
        entry!(0x0040_2220, fn_00402220(Ptr)),
        entry!(0x0040_1cd0, fn_00401cd0(Ptr, Ptr) -> bool),
        entry!(0x0040_2390, fn_00402390(Ptr, Ptr) -> bool),
        entry!(0x0040_2990, fn_00402990(Ptr, u8)),
        entry!(0x0040_29d0, fn_004029d0(Ptr, u32) -> u32),
        entry!(0x0040_2ae0, fn_00402ae0(Ptr) -> u32),
        entry!(0x0040_2a90, fn_00402a90(Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_00401a10_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0040_1a10,
            size: 0x20,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0040_19f0, &[A::T(0x18)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x77)]),
                (0x0040_1b20, &[A::T(0x0)]),
            ],
            stores: &[(0x0, 4, 0x0101_15b4)],
        });
    }

    #[test]
    fn fn_004020f0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0040_20f0,
            size: 0x20,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0040_19f0, &[A::T(0x18)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x78)]),
                (0x0040_1b20, &[A::T(0x0)]),
            ],
            stores: &[(0x0, 4, 0x0101_16fc)],
        });
    }

    #[test]
    fn fn_00402820_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0040_2820,
            size: 0xd8,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0040_2b40, &[A::T(0x0)]),
                (0x0048_a3d0, &[A::T(0x4c)]),
                (0x0040_2c20, &[A::T(0x6c)]),
                (0x0047_a130, &[A::T(0x78)]),
                (0x0048_cc50, &[A::T(0x88)]),
                (0x0048_eac0, &[A::T(0x94)]),
                (0x0047_9320, &[A::T(0x9c)]),
                (0x0047_5350, &[A::T(0xa4)]),
                (0x0047_b4f0, &[A::T(0xac)]),
                (0x0040_2c20, &[A::T(0xcc)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x2f)]),
                (0x0040_2ea0, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0101_1964),
                (0x30, 4, 0x0101_191c),
                (0x3c, 4, 0x0101_1900),
                (0x4c, 4, 0x0101_18dc),
                (0x6c, 4, 0x0101_18bc),
                (0x78, 4, 0x0101_18a8),
                (0x88, 4, 0x0101_1894),
                (0x94, 4, 0x0101_1880),
                (0x9c, 4, 0x0101_186c),
                (0xa4, 4, 0x0101_1858),
                (0xac, 4, 0x0101_1844),
            ],
        });
    }

    #[test]
    fn the_square_root_of_a_float_rounded_to_a_float() {
        let mut e = Engine::new();
        e.register(SQRT, |_, a| {
            let v = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            v.sqrt().into_ret()
        });
        e.call_log = Some(vec![]);
        let r = e.call(0x004019b0, &[2.0f32.to_bits()]).f64();
        assert_eq!(r, f64::from(2.0f64.sqrt() as f32));
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1].0, SQRT);
        assert_eq!(log[1].1, args![2.0f64]);
    }
    #[test]
    fn at_00401a90_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0040_1ac0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0040_1a90, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0040_1a90)
                .collect();
            let mut expected = vec![(0x0040_1ac0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_00402170_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0040_21a0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0040_2170, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0040_2170)
                .collect();
            let mut expected = vec![(0x0040_21a0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod stage_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn order(e: &Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .skip(1)
            .cloned()
            .collect()
    }

    /// An engine with the stage globals mapped, a heap allocator double and the stage data
    /// getter (`+0x18`).
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011c_3000, 0x1000);
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e.register(STAGE_DATA_00500940, |_, a| (a[0] + 0x18).into_ret());
        e.call_log = Some(vec![]);
        e
    }

    /// A stage whose data word (+0x18) and threshold are `value`.
    fn stage(e: &mut Engine, value: u32) -> u32 {
        let stage = e.mem.alloc(0x20);
        e.mem.set_u32(stage + 0x18, value);
        stage
    }

    fn set_list(e: &mut Engine, list: StageList, stages: &[u32]) {
        let array = e.mem.alloc(4 * stages.len() as u32 + 4);
        for (i, stage) in stages.iter().enumerate() {
            e.mem.set_u32(array + 4 * i as u32, *stage);
        }
        e.set_global(list.count, stages.len() as u32);
        e.set_global(list.array, array);
    }

    fn list(e: &Engine, list: StageList) -> Vec<u32> {
        let count = e.global::<u32>(list.count);
        let array = e.global::<u32>(list.array);
        (0..count).map(|i| e.mem.u32(array + i * 4)).collect()
    }

    #[test]
    fn fn_004019f0_and_00401b20_clear_the_data() {
        let mut e = engine();
        e.register(CLEAR_PAIR_00843170, |_, _| Ret::default());
        assert_eq!(e.call(0x0040_19f0, &args![0x5018u32]).u32(), 0x5018);
        e.call(0x0040_1b20, &args![0x5000u32]);
        assert_eq!(
            calls(&e, CLEAR_PAIR_00843170),
            vec![vec![0x5018], vec![0x5018]]
        );
    }

    #[test]
    fn fn_004020b0_and_004027e0_read_their_lists() {
        let mut e = engine();
        set_list(&mut e, HUNGER_STAGES, &[0x11, 0x22]);
        set_list(&mut e, SLEEP_STAGES, &[0x33]);
        assert_eq!(e.call(0x0040_20b0, &args![1u32]).u32(), 0x22);
        assert_eq!(e.call(0x0040_20b0, &args![2u32]).u32(), 0);
        assert_eq!(e.call(0x0040_27e0, &args![0u32]).u32(), 0x33);
        assert_eq!(e.call(0x0040_27e0, &args![u32::MAX]).u32(), 0);
    }

    #[test]
    fn fn_004024e0_makes_a_cleared_array() {
        let mut e = engine();
        assert_eq!(e.call(0x0040_24e0, &args![0u32]).u32(), 0);
        assert!(calls(&e, ALLOCATE).is_empty());
        let array = e.call(0x0040_24e0, &args![3u32]).u32();
        assert_eq!(calls(&e, ALLOCATE), vec![vec![0xc]]);
        assert_eq!(e.mem.bytes(array, 0xc), vec![0; 0xc]);
    }

    #[test]
    fn fn_004024e0_overflowing_size_is_all_ones() {
        let mut e = engine();
        let seen = Rc::new(RefCell::new(0u32));
        let s = seen.clone();
        e.register_double(ALLOCATE, move |_, a| {
            *s.borrow_mut() = a[0];
            // Panics on the first clear, after the allocator saw its size.
            0u32.into_ret()
        });
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            e.call(0x0040_24e0, &args![0x4000_0001u32]);
        }));
        assert!(result.is_err());
        assert_eq!(*seen.borrow(), u32::MAX);
    }

    #[test]
    fn fn_00401e20_inserts_by_data_word() {
        let mut e = engine();
        let a = stage(&mut e, 30);
        let b = stage(&mut e, 20);
        let c = stage(&mut e, 10);
        set_list(&mut e, HUNGER_STAGES, &[a, b, c]);
        let old = e.global::<u32>(HUNGER_STAGES.array);
        let new = stage(&mut e, 15);
        e.call(0x0040_1e20, &args![new]);
        assert_eq!(list(&e, HUNGER_STAGES), vec![a, b, new, c]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![old]]);
        // Below every entry: appended.
        let low = stage(&mut e, 1);
        e.call(0x0040_1e20, &args![low]);
        assert_eq!(list(&e, HUNGER_STAGES), vec![a, b, new, c, low]);
        // Above every entry: first.
        let high = stage(&mut e, 99);
        e.call(0x0040_2550, &args![high]);
        assert_eq!(list(&e, SLEEP_STAGES), vec![high]);
    }

    #[test]
    fn fn_00401f20_removes_a_listed_stage() {
        let mut e = engine();
        set_list(&mut e, HUNGER_STAGES, &[0x11, 0x22, 0x33]);
        let old = e.global::<u32>(HUNGER_STAGES.array);
        e.call(0x0040_1f20, &args![0x44u32]);
        assert_eq!(list(&e, HUNGER_STAGES), vec![0x11, 0x22, 0x33]);
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        e.call(0x0040_1f20, &args![0x22u32]);
        assert_eq!(list(&e, HUNGER_STAGES), vec![0x11, 0x33]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![old]]);
        // The last one: a null array of count 0.
        set_list(&mut e, SLEEP_STAGES, &[0x55]);
        e.call(0x0040_2650, &args![0x55u32]);
        assert_eq!(e.global::<u32>(SLEEP_STAGES.count), 0);
        assert_eq!(e.global::<u32>(SLEEP_STAGES.array), 0);
        // An empty list: nothing.
        e.call(0x0040_2650, &args![0x55u32]);
        assert_eq!(calls(&e, OPERATOR_DELETE).len(), 2);
    }

    #[test]
    fn fn_00402030_and_00402760_find_the_stage_for_a_value() {
        let mut e = engine();
        e.register(STAGE_THRESHOLD_009611E0, |e, a| {
            e.mem.u32(a[0] + 0x18).into_ret()
        });
        let a = stage(&mut e, 300);
        let b = stage(&mut e, 200);
        set_list(&mut e, HUNGER_STAGES, &[a, 0, b]);
        set_list(&mut e, SLEEP_STAGES, &[b]);
        let call = |e: &mut Engine, addr: u32, value: f32| e.call(addr, &args![value]).u32();
        assert_eq!(call(&mut e, 0x0040_2030, 350.0), a);
        assert_eq!(call(&mut e, 0x0040_2030, 300.0), a);
        assert_eq!(call(&mut e, 0x0040_2030, 250.0), b);
        assert_eq!(call(&mut e, 0x0040_2030, 50.0), 0);
        assert_eq!(call(&mut e, 0x0040_2030, f32::NAN), 0);
        assert_eq!(call(&mut e, 0x0040_2760, 200.0), b);
        assert_eq!(call(&mut e, 0x0040_2760, 199.5), 0);
    }

    fn init_rig(flag_set: bool, spell_id: u32) -> (Engine, u32) {
        let mut e = engine();
        e.register_double(FORM_FLAG_8_SET, move |_, _| flag_set.into_ret());
        e.register(SET_FORM_FLAG_8, |_, _| Ret::default());
        e.register(WORD_1C_00441110, |e, a| e.mem.u32(a[0] + 0x1c).into_ret());
        e.register(SET_WORD_1C_0050F9A0, |e, a| {
            e.mem.set_u32(a[0] + 0x1c, a[1]);
            Ret::default()
        });
        e.register(GET_FILE, |_, _| 0x0777u32.into_ret());
        e.register(ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            Ret::default()
        });
        e.register(LOOKUP_FORM, |_, a| (a[0] + 0x0900_0000).into_ret());
        e.register(DYNAMIC_CAST, |_, a| (a[0] + 4).into_ret());
        let this = stage(&mut e, 5);
        e.mem.set_u32(this + 0x1c, spell_id);
        set_list(&mut e, HUNGER_STAGES, &[]);
        set_list(&mut e, SLEEP_STAGES, &[]);
        (e, this)
    }

    #[test]
    fn fn_00401b60_resolves_the_spell_and_lists_the_stage() {
        let (mut e, this) = init_rig(false, 0x42);
        e.call(0x0040_1b60, &args![this]);
        assert_eq!(calls(&e, GET_FILE), vec![vec![this, u32::MAX]]);
        assert_eq!(
            calls(&e, DYNAMIC_CAST),
            vec![vec![0x0a00_0042, 0, TYPE_TES_FORM, TYPE_SPELL_ITEM, 0]]
        );
        assert_eq!(e.mem.u32(this + 0x1c), 0x0a00_0046);
        assert_eq!(calls(&e, SET_FORM_FLAG_8), vec![vec![this, 1]]);
        assert_eq!(list(&e, HUNGER_STAGES), vec![this]);
    }

    #[test]
    fn fn_00402220_without_a_spell_only_lists_and_once() {
        let (mut e, this) = init_rig(false, 0);
        e.call(0x0040_2220, &args![this]);
        assert!(calls(&e, GET_FILE).is_empty());
        assert!(calls(&e, SET_WORD_1C_0050F9A0).is_empty());
        assert_eq!(list(&e, SLEEP_STAGES), vec![this]);
        let (mut e, this) = init_rig(true, 0x42);
        e.call(0x0040_2220, &args![this]);
        assert!(calls(&e, SET_FORM_FLAG_8).is_empty());
        assert!(list(&e, SLEEP_STAGES).is_empty());
    }

    #[test]
    fn fn_00401b40_00402200_and_the_destructors() {
        let mut e = engine();
        e.register(0x0048_3630, |_, _| Ret::default());
        let this = stage(&mut e, 1);
        set_list(&mut e, HUNGER_STAGES, &[this]);
        set_list(&mut e, SLEEP_STAGES, &[this, 0x99]);
        e.call(0x0040_1ac0, &args![this]);
        assert_eq!(e.mem.u32(this), 0x0101_15b4);
        assert!(list(&e, HUNGER_STAGES).is_empty());
        e.call(0x0040_21a0, &args![this]);
        assert_eq!(e.mem.u32(this), 0x0101_16fc);
        assert_eq!(list(&e, SLEEP_STAGES), vec![0x99]);
        assert_eq!(calls(&e, 0x0048_3630), vec![vec![this]; 2]);
        e.call(0x0040_1b40, &args![0x99u32]);
        e.call(0x0040_2200, &args![0x99u32]);
        assert!(list(&e, SLEEP_STAGES).is_empty());
    }

    type Script = Rc<RefCell<(Vec<(u32, u32)>, usize)>>;

    const FILE: u32 = 0x0e00_0000;

    fn load_rig(chunks: Vec<(u32, u32)>, swap: bool, form_type: u32) -> (Engine, u32) {
        let mut e = engine();
        let script: Script = Rc::new(RefCell::new((chunks, 0)));
        e.register_double(GET_TES_FORM, move |_, _| form_type.into_ret());
        e.register(LOAD_FORM, |_, _| Ret::default());
        e.register(SET_FORM_FLAG_8, |_, _| Ret::default());
        let s = script.clone();
        e.register_double(GET_TES_CHUNK, move |_, _| {
            let s = s.borrow();
            s.0.get(s.1).map_or(0, |c| c.0).into_ret()
        });
        let s = script.clone();
        e.register_double(CHUNK_SIZE, move |_, _| {
            let s = s.borrow();
            s.0[s.1].1.into_ret()
        });
        let s = script;
        e.register_double(NEXT_CHUNK, move |_, _| {
            let mut s = s.borrow_mut();
            s.1 += 1;
            (s.1 < s.0.len()).into_ret()
        });
        for addr in [GET_CHUNK_DATA_SIZED, LOAD_DATA, SWAP_PAIR_00462230] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register_double(NEEDS_SWAP, move |_, _| swap.into_ret());
        e.put_vtable(0x0f00_e000, &[0; 0x4e]);
        e.mem.set_u32(0x0f00_e018, 0x0f00_e300);
        e.mem.set_u32(0x0f00_e0e0, 0x0f00_e100);
        e.mem.set_u32(0x0f00_e134, 0x0f00_e200);
        for addr in [0x0f00_e100u32, 0x0f00_e200, 0x0f00_e300] {
            e.register(addr, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this, 0x0f00_e000);
        e.call_log = Some(vec![]);
        (e, this)
    }

    #[test]
    fn fn_00401cd0_and_00402390_check_their_form_type() {
        let (mut e, this) = load_rig(vec![], false, 0x78);
        assert!(!e.call(0x0040_1cd0, &args![this, FILE]).bool());
        assert!(e.call(0x0040_2390, &args![this, FILE]).bool());
        let (mut e, this) = load_rig(vec![], false, 0x77);
        assert!(!e.call(0x0040_2390, &args![this, FILE]).bool());
        assert!(e.call(0x0040_1cd0, &args![this, FILE]).bool());
    }

    #[test]
    fn fn_00401cd0_reads_each_chunk() {
        let chunks = vec![
            (CHUNK_EDID, 6),
            (CHUNK_OBND, 0xc),
            (CHUNK_DATA, 8),
            (0x1234_5678, 4),
        ];
        let (mut e, this) = load_rig(chunks, true, 0x77);
        assert!(e.call(0x0040_1cd0, &args![this, FILE]).bool());
        let first: Vec<u32> = order(&e).iter().take(4).map(|c| c.0).collect();
        assert_eq!(
            first,
            vec![GET_TES_FORM, 0x0f00_e300, LOAD_FORM, SET_FORM_FLAG_8]
        );
        let edid = calls(&e, 0x0f00_e200);
        assert_eq!(
            calls(&e, GET_CHUNK_DATA_SIZED),
            vec![vec![FILE, edid[0][1], 0x200]]
        );
        assert_eq!(calls(&e, 0x0f00_e100), vec![vec![this, FILE]]);
        assert_eq!(calls(&e, LOAD_DATA), vec![vec![this, FILE, this + 0x18, 8]]);
        // Swapped only after the DATA chunk.
        assert_eq!(calls(&e, SWAP_PAIR_00462230), vec![vec![this + 0x18]]);
        assert_eq!(calls(&e, NEEDS_SWAP).len(), 1);
        assert_eq!(calls(&e, NEXT_CHUNK).len(), 4);
    }

    #[test]
    fn fn_00402990_sets_form_flag_2_with_the_byte() {
        let mut e = engine();
        e.register(SET_FORM_FLAG_2, |_, _| Ret::default());
        e.call(0x0040_2990, &args![0x5000u32, 0x101u32]);
        assert_eq!(calls(&e, SET_FORM_FLAG_2), vec![vec![0x5000, 1]]);
    }

    #[test]
    fn fn_004029d0_00402ae0_00402a90_test_the_flags() {
        let mut e = engine();
        let item = e.mem.alloc(0xd8);
        e.mem.set_u8(item + 0xbc, 0x06);
        assert_eq!(e.call(0x0040_29d0, &args![item, 2u32]).u32(), 1);
        assert_eq!(e.call(0x0040_29d0, &args![item, 0x109u32]).u32(), 0);
        assert_eq!(e.call(0x0040_2ae0, &args![item + 0x3c]).u32(), 1);
        e.register(CAN_BE_POISON, |_, _| 0x100u32.into_ret());
        assert!(e.call(0x0040_2a90, &args![item + 0x3c]).bool());
        assert_eq!(calls(&e, CAN_BE_POISON), vec![vec![item + 0x3c]]);
        e.register(CAN_BE_POISON, |_, _| 1u32.into_ret());
        assert!(!e.call(0x0040_2a90, &args![item + 0x3c]).bool());
        e.mem.set_u8(item + 0xbc, 0x04);
        assert!(!e.call(0x0040_2a90, &args![item + 0x3c]).bool());
        assert_eq!(e.call(0x0040_2ae0, &args![item + 0x3c]).u32(), 1);
        e.mem.set_u8(item + 0xbc, 0);
        assert_eq!(e.call(0x0040_2ae0, &args![item + 0x3c]).u32(), 0);
    }
}

#[cfg(test)]
mod n075_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn engine(cast: bool, differ: bool, bytes_differ: bool) -> Engine {
        let mut e = Engine::new();
        e.register(
            DYNAMIC_CAST,
            if cast {
                |_, a| a[0].into_ret()
            } else {
                |_, _| 0u32.into_ret()
            },
        );
        e.register(COPY_ALL_COMPONENTS, |_, _| Ret::default());
        e.register(
            COMPARE_ALL_COMPONENTS,
            if differ {
                |_, _| 0x0101u32.into_ret()
            } else {
                |_, _| 0x0100u32.into_ret()
            },
        );
        e.register(STAGE_DATA_00500940, |_, a| (a[0] + 0x18).into_ret());
        e.register(COPY_BYTES_00401460, |_, _| Ret::default());
        e.register(
            MEMCMP,
            if bytes_differ {
                |_, _| 1u32.into_ret()
            } else {
                |_, _| 0u32.into_ret()
            },
        );
        e.call_log = Some(vec![]);
        e
    }

    fn log_without(e: &mut Engine, top: u32) -> Log {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != top)
            .collect()
    }

    fn check_copy(address: u32, class: u32) {
        let mut e = engine(true, false, false);
        e.call(address, &args![0x1000u32, 0x2000u32]);
        assert_eq!(
            log_without(&mut e, address),
            vec![
                (DYNAMIC_CAST, vec![0x2000, 0, TYPE_TES_FORM, class, 0]),
                (COPY_ALL_COMPONENTS, vec![0x1000, 0x2000]),
                (STAGE_DATA_00500940, vec![0x2000]),
                (STAGE_DATA_00500940, vec![0x1000]),
                (COPY_BYTES_00401460, vec![0x1018, 0x2018, 8]),
            ]
        );
        let mut e = engine(false, false, false);
        e.call(address, &args![0x1000u32, 0x2000u32]);
        assert_eq!(log_without(&mut e, address).len(), 1);
    }

    fn check_compare(address: u32, class: u32) {
        let compare = |cast, differ, bytes| {
            let mut e = engine(cast, differ, bytes);
            let r = e.call(address, &args![0x1000u32, 0x2000u32]).bool();
            (r, log_without(&mut e, address))
        };
        let (r, log) = compare(false, false, false);
        assert!(r);
        assert_eq!(
            log,
            vec![(DYNAMIC_CAST, vec![0x2000, 0, TYPE_TES_FORM, class, 0])]
        );
        let (r, log) = compare(true, true, false);
        assert!(r);
        assert!(log.iter().all(|(a, _)| *a != MEMCMP));
        assert!(compare(true, false, true).0);
        let (r, log) = compare(true, false, false);
        assert!(!r);
        assert_eq!(log.last().unwrap(), &(MEMCMP, vec![0x1018, 0x2018, 8]));
    }

    #[test]
    fn fn_00401bf0_copies_a_hunger_stage() {
        check_copy(0x0040_1bf0, TYPE_HUNGER_STAGE);
    }

    #[test]
    fn fn_00401c50_compares_a_hunger_stage() {
        check_compare(0x0040_1c50, TYPE_HUNGER_STAGE);
    }

    #[test]
    fn fn_004022b0_copies_a_sleep_stage() {
        check_copy(0x0040_22b0, TYPE_SLEEP_STAGE);
    }

    #[test]
    fn fn_00402310_compares_a_sleep_stage() {
        check_compare(0x0040_2310, TYPE_SLEEP_STAGE);
    }

    #[test]
    fn fn_004029b0_answers_one_while_flag_one_is_clear() {
        let mut e = Engine::new();
        let item = e.mem.alloc(0xc0);
        e.mem.set_u8(item + 0xbc, 0xfe);
        assert_eq!(e.call(0x0040_29b0, &args![item]).u32(), 1);
        e.mem.set_u8(item + 0xbc, 0x01);
        assert_eq!(e.call(0x0040_29b0, &args![item]).u32(), 0);
    }

    #[test]
    fn fn_004029f0_sets_flag_one_for_zero_and_clears_it_otherwise() {
        let mut e = Engine::new();
        let item = e.mem.alloc(0xc0);
        e.mem.set_u8(item + 0xbc, 0x80);
        e.call(0x0040_29f0, &args![item, 0x100u32]);
        assert_eq!(e.mem.u8(item + 0xbc), 0x81);
        e.call(0x0040_29f0, &args![item, 0x102u32]);
        assert_eq!(e.mem.u8(item + 0xbc), 0x80);
    }

    #[test]
    fn fn_00402a20_ors_or_masks_the_flag_byte() {
        let mut e = Engine::new();
        let item = e.mem.alloc(0xc0);
        e.mem.set_u8(item + 0xbc, 0x11);
        e.call(0x0040_2a20, &args![item, 0x0f06u32, 0x100u32]);
        assert_eq!(e.mem.u8(item + 0xbc), 0x11 & !0x06);
        e.call(0x0040_2a20, &args![item, 0x0f06u32, 1u32]);
        assert_eq!(e.mem.u8(item + 0xbc), 0x17);
        assert_eq!(e.mem.u8(item + 0xbd), 0);
    }
}
