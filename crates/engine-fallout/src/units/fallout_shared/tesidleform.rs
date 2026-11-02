//! `fallout shared/tesidleform.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `TESIdleForm`: an idle animation form, its record, and its place in the
//! idle manager's trees (a parent idle and, in the PC build, a previous
//! idle among its siblings).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `TESIdleForm` (Xbox PDB), 0x54 bytes on PC (`TESForm` is 0x10
    /// smaller than on the Xbox; the word at +0x48 and the string at +0x4c
    /// are PC only). The fields used here.
    pub struct TesIdleForm: 0x54 {
        /// `data.cAnimData` (Xbox PDB `IDLE_DATA`): bits 0 to 5 the anim
        /// group section, bit 6 and bit 7 flags.
        0x38 cAnimData: u8,
        /// `pChildIdles` (Xbox PDB): `NiFormArray*`.
        0x40 pChildIdles: u32,
        /// `pParentIdle` (Xbox PDB): `TESIdleForm*`; a form id until
        /// `InitItem`.
        0x44 pParentIdle: u32,
        /// PC only: the previous idle (a form id until `InitItem`; the
        /// record's `ANAM` holds it after the parent).
        0x48 pPreviousIdle: u32,
    }
}

/// `TESFile::GetTESForm` (Xbox PDB name of `00472660`; `ECX` = file).
const GET_TES_FORM: u32 = 0x0047_2660;
/// The idle form type (0x48).
const IDLE_FORM_TYPE: u32 = 0x48;
/// `TESForm::LoadForm` (Xbox PDB name of `00485110`; `ECX` = form, the file).
const TES_FORM_LOAD_FORM: u32 = 0x0048_5110;
/// `TESFile::GetTESChunk` (Xbox PDB name of `004726b0`; `ECX` = file).
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// `004726f0` (`ECX` = file): moves to the next chunk; true when there is one.
const NEXT_CHUNK: u32 = 0x0047_26f0;
/// `00401660` (`ECX` = file): the current chunk's size.
const CHUNK_SIZE: u32 = 0x0040_1660;
/// `TESFile::GetChunkData` (Xbox PDB overload `_ov3`, `00472890`; `ECX` =
/// file; buffer, size).
const GET_CHUNK_DATA: u32 = 0x0047_2890;
/// `00401680` (`ECX` = file): whether the data read needs its byte order
/// swapped.
const FILE_NEEDS_SWAP: u32 = 0x0040_1680;
/// `TESForm::LoadData` (Xbox PDB name of `004861f0`; `ECX` = form; file,
/// destination, size).
const TES_FORM_LOAD_DATA: u32 = 0x0048_61f0;
/// `TESModel::LoadModelChunk` (Xbox PDB name of `004892d0`; cdecl: model,
/// file).
const LOAD_MODEL_CHUNK: u32 = 0x0048_92d0;
/// `00436aa0` (`ECX` = idle): its conditions.
const CONDITIONS: u32 = 0x0043_6aa0;
/// `TESCondition::Load` (Xbox PDB name of `00680960`; `ECX` = conditions,
/// the file; `RET 4`).
const CONDITION_LOAD: u32 = 0x0068_0960;
/// `TESCondition::InitItem` (Xbox PDB name of `00680a20`; `ECX` =
/// conditions, the form; `RET 4`).
const CONDITION_INIT_ITEM: u32 = 0x0068_0a20;
/// `00403d30` (cdecl: destination, byte, count): fills memory.
const FILL_MEMORY: u32 = 0x0040_3d30;
/// `00462230` (`ECX` = the two `ANAM` words): their byte swap.
const SWAP_PAIR: u32 = 0x0046_2230;
/// `005ffbf0` (`ECX` = the idle manager, the idle; `RET 4`).
const MANAGER_005FFBF0: u32 = 0x005f_fbf0;
/// `005fea20` (`ECX` = the data at +0x38): its byte swap.
const SWAP_DATA: u32 = 0x005f_ea20;
/// The idle manager (`011cb6a0`, a pointer global).
const IDLE_MANAGER: u32 = 0x011c_b6a0;
/// Virtual slots: `0xe0` loads `OBND`, `0x134` takes the editor id,
/// `0x130` gives a name, `0x90` fills a string with a description.
const SLOT_LOAD_BOUNDS: u32 = 0xe0;
const SLOT_SET_EDITOR_ID: u32 = 0x134;
const SLOT_NAME: u32 = 0x130;
const SLOT_DESCRIBE: u32 = 0x90;
/// The chunk tags.
const CHUNK_OBND: u32 = 0x444e_424f;
const CHUNK_CTDA: u32 = 0x4144_5443;
const CHUNK_DATA: u32 = 0x4154_4144;
const CHUNK_EDID: u32 = 0x4449_4445;
const CHUNK_MODL: u32 = 0x4c44_4f4d;
const CHUNK_ANAM: u32 = 0x4d41_4e41;
const CHUNK_MODT: u32 = 0x5444_4f4d;
/// The most an `EDID` chunk is read (0x200 bytes).
const EDITOR_ID_MAXIMUM: u32 = 0x200;
/// `004013e0` (`ECX` = form): a byte; true skips `InitItem`.
const FORM_004013E0: u32 = 0x0040_13e0;
/// `0084e3a0` (`ECX` = form): its form id.
const FORM_ID: u32 = 0x0084_e3a0;
/// `005b5e40` (cdecl: format, arguments...): writes a warning line.
const LOG_WARNING: u32 = 0x005b_5e40;
/// `005ff190` (`ECX` = idle, the section; `RET 4`): sets the section.
const SET_SECTION: u32 = 0x005f_f190;
/// `TESForm::GetFile` (Xbox PDB name of `00484e60`; `ECX` = form, an index).
const GET_FILE: u32 = 0x0048_4e60;
/// `TESForm::AddCompileIndex` (Xbox PDB name of `00485d50`; cdecl: address
/// of a form id, file).
const ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// `004839c0` (cdecl: form id): the form with that id.
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `__RTDynamicCast` (`00ec43fb`, cdecl, five words).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The `TESForm` and `TESIdleForm` type descriptors.
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_IDLE_FORM: u32 = 0x0118_6a18;
/// `004037b0` and `004037d0` (`ECX` = string): construct and destroy.
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
const STRING_DESTRUCT: u32 = 0x0040_37d0;
/// `00559450` (`ECX` = string): its characters.
const STRING_CHARACTERS: u32 = 0x0055_9450;
/// `LowProcess::SetGenericLocation` (engine map name of `008d8040`; sets
/// +0x44) and `LowProcess::SetSecondGenericLocation` (`00673400`; sets
/// +0x48): `ECX` = idle, a word; `RET 4`.
const SET_PARENT: u32 = 0x008d_8040;
const SET_PREVIOUS: u32 = 0x0067_3400;
/// `LowProcess::GetSecondGenericLocation` (engine map name of `006733e0`;
/// `ECX` = idle): the word at +0x48.
const GET_PREVIOUS: u32 = 0x0067_33e0;
/// `00484ab0` (`ECX` = form, a word; `RET 4`).
const FORM_SET_484AB0: u32 = 0x0048_4ab0;
/// The `InitItem` warnings (`01049ed8`: a bad anim group section;
/// `01049ea0`: no parent idle; `01049e68`: no previous idle).
const FORMAT_BAD_SECTION: u32 = 0x0104_9ed8;
const FORMAT_NO_PARENT: u32 = 0x0104_9ea0;
const FORMAT_NO_PREVIOUS: u32 = 0x0104_9e68;
/// `TESIdleManager::GetLooseIdleArray_ov2` and `AddRootIdleArray_ov2` (Xbox
/// PDB names of `00600690` and `006004f0`; `ECX` = manager, the idle; `RET
/// 4`).
const LOOSE_IDLE_ARRAY: u32 = 0x0060_0690;
const ADD_ROOT_IDLE_ARRAY: u32 = 0x0060_04f0;
/// `005fe770` (`run_005fde80`; `ECX` = idle): its child array, made on
/// first use.
const CHILD_ARRAY: u32 = 0x005f_e770;
/// `NiFormArray::GetIndexForForm` (Xbox PDB name of `004d6100`; `ECX` =
/// array, the form; `RET 4`): -1 when not found.
const INDEX_FOR_FORM: u32 = 0x004d_6100;
/// `00440d80` (`ECX` = form): a byte.
const FORM_00440D80: u32 = 0x0044_0d80;
/// `0061bc10` (`ECX` = array; index, form; `RET 8`): inserts.
const ARRAY_INSERT: u32 = 0x0061_bc10;
/// `0061bd20` (`ECX` = array; index, the address of the removed form; `RET
/// 8`): removes.
const ARRAY_REMOVE_AT: u32 = 0x0061_bd20;
/// `0061bdc0` (`ECX` = array).
const ARRAY_0061BDC0: u32 = 0x0061_bdc0;
/// `00877a30` (`ECX` = array, an index; `RET 4`): the address of that
/// element.
const ARRAY_ELEMENT: u32 = 0x0087_7a30;
/// The `InitParents` and `FixNextIdles` warnings (`01049f6c`: no loose
/// idle array; `01049f3c`: no parent array; `01049f9c`: several next
/// idles).
const FORMAT_NO_LOOSE_ARRAY: u32 = 0x0104_9f6c;
const FORMAT_NO_PARENT_ARRAY: u32 = 0x0104_9f3c;
const FORMAT_SEVERAL_NEXT: u32 = 0x0104_9f9c;
/// `strrchr` (`00ec6e30`, cdecl: string, character).
const STRRCHR: u32 = 0x00ec_6e30;
/// `00404dc0` (cdecl: two strings): 0 when they compare equal.
const STRING_COMPARE: u32 = 0x0040_4dc0;
/// The extension an idle file must have (`01049fc4`).
const IDLE_FILE_EXTENSION: u32 = 0x0104_9fc4;
/// Virtual slot `0x14` of the model at +0x18: its file name.
const SLOT_MODEL_NAME: u32 = 0x14;

// Translated from 005fe810 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005fe810 - the security-cookie check (00ec408c) is not translated.
/// `TESIdleForm::Load` (Xbox PDB; the file; `RET 4`): false when the
/// record is not an idle (0x48); otherwise `TESForm::LoadForm`, then chunk
/// by chunk until a zero tag or no next chunk:
/// - `EDID`: into a stack buffer of the chunk's size (the `_alloca`
///   rounds it to 16), at most 0x200 bytes, then virtual slot `0x134`;
/// - `OBND`: virtual slot `0xe0` with the file;
/// - `MODL`, `MODT`: `TESModel::LoadModelChunk` on the model at +0x18 (0
///   for a null idle);
/// - `CTDA`: `TESCondition::Load` on the conditions;
/// - `DATA`: `TESForm::LoadData` into +0x38, 8 bytes, then (still for
///   `DATA`) `005fea20` on it when the file needs swapping;
/// - `ANAM`: two words (cleared, read, swapped by `00462230` when needed)
///   into the parent (+0x44) and the previous idle (+0x48), then
///   `005ffbf0` on the idle manager with the idle.
///
/// Returns true.
pub fn tes_idle_form_load(e: &mut Engine, this: Ptr<TesIdleForm>, file: Ptr) -> bool {
    let idle = this.addr();
    if e.call(GET_TES_FORM, &args![file]).u32() != IDLE_FORM_TYPE {
        return false;
    }
    e.call(TES_FORM_LOAD_FORM, &args![this, file]);
    loop {
        let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            CHUNK_EDID => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                e.with_stack(size.wrapping_add(15) & !15, |e, buffer| {
                    e.call(GET_CHUNK_DATA, &args![file, buffer, EDITOR_ID_MAXIMUM]);
                    e.vcall(idle, SLOT_SET_EDITOR_ID, &args![buffer]);
                });
            }
            CHUNK_OBND => {
                e.vcall(idle, SLOT_LOAD_BOUNDS, &args![file]);
            }
            CHUNK_MODL | CHUNK_MODT => {
                let model = if idle == 0 { 0 } else { idle + 0x18 };
                e.call(LOAD_MODEL_CHUNK, &args![model, file]);
            }
            CHUNK_CTDA => {
                let conditions = e.call(CONDITIONS, &args![this]).u32();
                e.call(CONDITION_LOAD, &args![conditions, file]);
            }
            CHUNK_DATA => {
                e.call(TES_FORM_LOAD_DATA, &args![this, file, idle + 0x38, 8u32]);
            }
            CHUNK_ANAM => {
                e.with_stack(8, |e, pair| {
                    e.call(FILL_MEMORY, &args![pair, 0u32, 8u32]);
                    e.call(GET_CHUNK_DATA, &args![file, pair, 8u32]);
                    if e.call(FILE_NEEDS_SWAP, &args![file]).bool() {
                        e.call(SWAP_PAIR, &args![pair]);
                    }
                    let parent = e.mem.u32(pair.addr());
                    let previous = e.mem.u32(pair.addr() + 4);
                    e.set(this, TesIdleForm::pParentIdle, parent);
                    e.set(this, TesIdleForm::pPreviousIdle, previous);
                });
                let manager = e.mem.u32(IDLE_MANAGER);
                e.call(MANAGER_005FFBF0, &args![manager, this]);
            }
            _ => {}
        }
        if chunk == CHUNK_DATA && e.call(FILE_NEEDS_SWAP, &args![file]).bool() {
            e.call(SWAP_DATA, &args![idle + 0x38]);
        }
        if !e.call(NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    true
}

/// Writes `format` with `id` and the idle's description (its slot `0x90`
/// into a stack string; `00559450` of it).
fn warn_described(e: &mut Engine, this: Ptr<TesIdleForm>, format: u32, id: Option<u32>) {
    e.with_stack(8, |e, string| {
        e.call(STRING_CONSTRUCT, &args![string]);
        e.vcall(this.addr(), SLOT_DESCRIBE, &args![string]);
        let text = e.call(STRING_CHARACTERS, &args![string]).u32();
        match id {
            Some(id) => e.call(LOG_WARNING, &args![format, id, text]),
            None => e.call(LOG_WARNING, &args![format, text]),
        };
        e.call(STRING_DESTRUCT, &args![string]);
    });
}

/// `InitItem`'s resolution of the form id at `field` to an idle (0 when not
/// found, with a warning for a nonzero id), handed to `setter`.
fn resolve_idle(e: &mut Engine, this: Ptr<TesIdleForm>, field: u32, format: u32, setter: u32) {
    let id = e.mem.u32(this.addr() + field);
    if id == 0 {
        return;
    }
    let (found, id) = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), id);
        let file = e.call(GET_FILE, &args![this, 0xffff_ffffu32]).u32();
        e.call(ADD_COMPILE_INDEX, &args![slot, file]);
        let id = e.mem.u32(slot.addr());
        let form = e.call(LOOKUP_FORM, &args![id]).u32();
        let found = e
            .call(
                DYNAMIC_CAST,
                &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_IDLE_FORM, 0u32],
            )
            .u32();
        (found, id)
    });
    if found == 0 {
        warn_described(e, this, format, Some(id));
    }
    e.call(setter, &args![this, found]);
}

// Translated from 005feb90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005feb90 - the exception-unwinding frame (FS:[0]) is not translated.
/// `TESIdleForm::InitItem` (Xbox PDB): skipped when `004013e0` reports
/// true. Initialises the conditions; resets an anim group section of 8 to
/// 0x13 or above 0x15 to 7 (`005ff190`) with a warning naming the idle
/// (slot `0x130`) and its form id; resolves the parent (+0x44, set by
/// `008d8040`) and the previous idle (+0x48, set by `00673400`), each
/// with a warning and 0 when not found; then `00484ab0` with 1.
pub fn tes_idle_form_init_item(e: &mut Engine, this: Ptr<TesIdleForm>) {
    if e.call(FORM_004013E0, &args![this]).bool() {
        return;
    }
    let conditions = e.call(CONDITIONS, &args![this]).u32();
    e.call(CONDITION_INIT_ITEM, &args![conditions, this]);
    let section = tes_idle_form_get_idle_anim_group_section(e, this) as i32;
    if section >= 8 && !(0x14..=0x15).contains(&section) {
        let id = e.call(FORM_ID, &args![this]).u32();
        let name = e.vcall(this.addr(), SLOT_NAME, &[]).u32();
        e.call(LOG_WARNING, &args![FORMAT_BAD_SECTION, name, id]);
        e.call(SET_SECTION, &args![this, 7u32]);
    }
    resolve_idle(e, this, 0x44, FORMAT_NO_PARENT, SET_PARENT);
    resolve_idle(e, this, 0x48, FORMAT_NO_PREVIOUS, SET_PREVIOUS);
    e.call(FORM_SET_484AB0, &args![this, 1u32]);
}

// Translated from 005fedb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005fedb0 - the exception-unwinding frame (FS:[0]) is not translated.
/// `TESIdleForm::InitParents` (Xbox PDB): puts the idle in its array.
/// - A loose idle (`005ff200`): the manager's loose array (made through
///   `AddRootIdleArray` when missing) gets it at its end (`0061bc10` with
///   the array's count, `0084e3a0`); without an array, a warning (its
///   slot `0x130` and form id) and false.
/// - Otherwise the parent's child array (`005fe770`) or, without a
///   parent, the root array from `AddRootIdleArray`; the idle goes after
///   its previous idle (index + 1; -1 stays when the previous idle is not
///   in it), or at the end (count + 1) for an index of -1 or when
///   `00440d80` reports true, then `FixNextIdles` on the array. Without an
///   array, a warning with its description and false.
///
/// True otherwise.
pub fn tes_idle_form_init_parents(e: &mut Engine, this: Ptr<TesIdleForm>) -> bool {
    let parent = e.get(this, TesIdleForm::pParentIdle);
    let previous = e.get(this, TesIdleForm::pPreviousIdle);
    if fn_005ff200(e, this) {
        let manager = e.mem.u32(IDLE_MANAGER);
        let mut array = e.call(LOOSE_IDLE_ARRAY, &args![manager, this]).u32();
        if array == 0 {
            let manager = e.mem.u32(IDLE_MANAGER);
            e.call(ADD_ROOT_IDLE_ARRAY, &args![manager, this]);
            let manager = e.mem.u32(IDLE_MANAGER);
            array = e.call(LOOSE_IDLE_ARRAY, &args![manager, this]).u32();
        }
        if array == 0 {
            let id = e.call(FORM_ID, &args![this]).u32();
            let name = e.vcall(this.addr(), SLOT_NAME, &[]).u32();
            e.call(LOG_WARNING, &args![FORMAT_NO_LOOSE_ARRAY, name, id]);
            return false;
        }
        let count = e.call(FORM_ID, &args![array]).u32();
        e.call(ARRAY_INSERT, &args![array, count, this]);
        return true;
    }
    let array = if parent != 0 {
        e.call(CHILD_ARRAY, &args![parent]).u32()
    } else {
        let manager = e.mem.u32(IDLE_MANAGER);
        e.call(ADD_ROOT_IDLE_ARRAY, &args![manager, this]).u32()
    };
    if array == 0 {
        warn_described(e, this, FORMAT_NO_PARENT_ARRAY, None);
        return false;
    }
    let mut index = 0u32;
    if previous != 0 {
        let found = e.call(INDEX_FOR_FORM, &args![array, previous]).u32();
        index = if found == 0xffff_ffff {
            found
        } else {
            found.wrapping_add(1)
        };
    }
    if index == 0xffff_ffff || e.call(FORM_00440D80, &args![this]).bool() {
        index = e.call(FORM_ID, &args![array]).u32().wrapping_add(1);
    }
    e.call(ARRAY_INSERT, &args![array, index, this]);
    tes_idle_form_fix_next_idles(e, this, array);
    true
}

// Translated from 005fefa0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005fefa0 - the exception-unwinding frame (FS:[0]) is not translated.
/// `TESIdleForm::FixNextIdles` (Xbox PDB; the array; `RET 4`): finds the
/// idle's index and the index of the one idle whose previous idle it is
/// (the next idle; a second one warns), counting the elements that are not
/// idles (the `TESIdleForm` cast fails). When both are found and the next
/// idle is not right after this one, it is taken out (`0061bd20`,
/// `0061bdc0`) and put back right after the idle (the idle's index less
/// the non-idles, one more when the next idle was before it; `0061bc10`,
/// `0061bdc0`), and the next idle's own next idle is fixed in turn.
pub fn tes_idle_form_fix_next_idles(e: &mut Engine, this: Ptr<TesIdleForm>, array: u32) {
    if array == 0 {
        return;
    }
    let mut skipped = 0u32;
    let mut own = 0xffff_ffffu32;
    let count = e.call(FORM_ID, &args![array]).u32();
    let mut next_index = 0xffff_ffffu32;
    let mut next = 0u32;
    let mut index = 0u32;
    while index < count {
        let element = e.call(ARRAY_ELEMENT, &args![array, index]).u32();
        let form = e.mem.u32(element);
        let idle = e
            .call(
                DYNAMIC_CAST,
                &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_IDLE_FORM, 0u32],
            )
            .u32();
        if idle == 0 {
            skipped = skipped.wrapping_add(1);
        } else if idle == this.addr() {
            own = index;
        } else if e.call(GET_PREVIOUS, &args![idle]).u32() == this.addr() {
            if next == 0 {
                next = idle;
                next_index = index;
            } else {
                warn_described(e, this, FORMAT_SEVERAL_NEXT, None);
            }
        }
        index += 1;
    }
    if own == 0xffff_ffff || next_index == 0xffff_ffff || next_index == own.wrapping_add(1) {
        return;
    }
    e.with_stack(4, |e, removed| {
        e.mem.set_u32(removed.addr(), 0);
        e.call(ARRAY_REMOVE_AT, &args![array, next_index, removed]);
    });
    e.call(ARRAY_0061BDC0, &args![array]);
    if next_index < own {
        skipped = skipped.wrapping_add(1);
    }
    let own = own.wrapping_sub(skipped);
    e.call(ARRAY_INSERT, &args![array, own.wrapping_add(1), next]);
    e.call(ARRAY_0061BDC0, &args![array]);
    tes_idle_form_fix_next_idles(e, Ptr::new(next), array);
}

// Translated from 005ff160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESIdleForm::GetIdleAnimGroupSection` (Xbox PDB): bits 0 to 5 of
/// `cAnimData`.
pub fn tes_idle_form_get_idle_anim_group_section(e: &mut Engine, this: Ptr<TesIdleForm>) -> u32 {
    u32::from(e.get(this, TesIdleForm::cAnimData) & 0x3f)
}

// Translated from 005ff1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 7 of `cAnimData` is set. The engine map gives it no name.
pub fn fn_005ff1d0(e: &mut Engine, this: Ptr<TesIdleForm>) -> bool {
    e.get(this, TesIdleForm::cAnimData) & 0x80 != 0
}

// Translated from 005ff200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 6 of `cAnimData` is set (the idle goes in the loose array,
/// `InitParents`). The engine map gives it no name.
pub fn fn_005ff200(e: &mut Engine, this: Ptr<TesIdleForm>) -> bool {
    e.get(this, TesIdleForm::cAnimData) & 0x40 != 0
}

// Translated from 005ff280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESIdleForm::CheckIdleFile` (Xbox PDB): whether the model's file name
/// (slot `0x14` of the model at +0x18) has an extension (`strrchr` with
/// '.') equal (`00404dc0`) to the one at `01049fc4`.
pub fn tes_idle_form_check_idle_file(e: &mut Engine, this: Ptr<TesIdleForm>) -> bool {
    let name = e.vcall(this.addr() + 0x18, SLOT_MODEL_NAME, &[]).u32();
    let dot = e.call(STRRCHR, &args![name, 0x2eu32]).u32();
    if dot == 0 {
        return false;
    }
    e.call(STRING_COMPARE, &args![dot, IDLE_FILE_EXTENSION])
        .u32()
        == 0
}

/// The 16-bit swap helper (`00407a90`, `cdecl (address, flag)`).
const SWAP_SHORT: u32 = 0x0040_7a90;
/// The savers `TESIdleForm::Save` calls: `TESForm::StartForm` (`004855a0`),
/// `TESModel::Save` (`004891d0`, two chunk words), `TESCondition::Save`
/// (`00680910`), `TESForm::__AddChunkData` (`00485990`, `cdecl`),
/// `TESForm::__SaveData` (`00485e20`, `thiscall (form, data, size)`) and
/// `TESForm::CloseForm` (`00485680`); the byte `00401500` answers (whether
/// saving swaps bytes).
const SAVE_FORM_START: u32 = 0x0048_55a0;
const SAVE_MODEL: u32 = 0x0048_91d0;
const SAVE_CONDITIONS: u32 = 0x0068_0910;
const SAVE_CHUNK_DATA: u32 = 0x0048_5990;
const SAVE_FORM_DATA: u32 = 0x0048_5e20;
const SAVE_FORM_CLOSE: u32 = 0x0048_5680;
const SAVE_SWAPS: u32 = 0x0040_1500;
/// The idle getters the save asks for the `ANAM` pair: the parent
/// (`008041a0`), whether an idle is only a grouping one (`00440d80`,
/// answers a byte) and the previous idle (`006733e0`); all `thiscall`
/// without words.
const IDLE_PARENT: u32 = 0x0080_41a0;
const IDLE_IS_GROUPING: u32 = 0x0044_0d80;
const IDLE_PREVIOUS: u32 = 0x0067_33e0;

// Translated from 005fea20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`; the byte swap of `IDLE_DATA`):
/// the 16-bit word at +4 (`00407a90` with flag 0).
pub fn fn_005fea20(e: &mut Engine, this: Ptr) {
    e.call(SWAP_SHORT, &args![this.addr() + 4, 0u32]);
}

// Translated from 005fea50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`; the save of a `TESIdleForm`):
/// starts the form, saves the model (+0x18, `MODL`, `MODT`) and the
/// conditions (`00436aa0`), then the `ANAM` pair (8 stack bytes, zeroed):
/// the parent's form id when there is a parent, and, unless this idle is
/// a grouping one (`00440d80`), the form id of the first previous idle
/// (`006733e0`, stepping over grouping ones) that exists; the pair is
/// swapped (`00462230`) before and after when saving swaps bytes. Then the
/// eight data bytes at +0x38 (`__SaveData`, swapped by [`fn_005fea20`]
/// before and after) and closes the form.
pub fn fn_005fea50(e: &mut Engine, this: Ptr<TesIdleForm>) {
    let at = this.addr();
    e.call(SAVE_FORM_START, &args![this]);
    e.call(SAVE_MODEL, &args![at + 0x18, CHUNK_MODL, CHUNK_MODT]);
    let conditions = e.call(CONDITIONS, &args![this]).u32();
    e.call(SAVE_CONDITIONS, &args![conditions]);
    e.with_stack(8, |e, pair| {
        e.call(FILL_MEMORY, &args![pair, 0u32, 8u32]);
        if e.call(IDLE_PARENT, &args![this]).u32() != 0 {
            let parent = e.call(IDLE_PARENT, &args![this]).u32();
            let id = e.call(FORM_ID, &args![parent]).u32();
            e.mem.set_u32(pair.addr(), id);
        }
        if e.call(IDLE_IS_GROUPING, &args![this]).u8() == 0 {
            let mut previous = e.call(IDLE_PREVIOUS, &args![this]).u32();
            while previous != 0 && e.call(IDLE_IS_GROUPING, &args![previous]).u8() != 0 {
                previous = e.call(IDLE_PREVIOUS, &args![previous]).u32();
            }
            if previous != 0 {
                let id = e.call(FORM_ID, &args![previous]).u32();
                e.mem.set_u32(pair.addr() + 4, id);
            }
        }
        if e.call(SAVE_SWAPS, &args![]).u8() != 0 {
            e.call(SWAP_PAIR, &args![pair]);
        }
        e.call(SAVE_CHUNK_DATA, &args![CHUNK_ANAM, pair, 8u32]);
        if e.call(SAVE_SWAPS, &args![]).u8() != 0 {
            e.call(SWAP_PAIR, &args![pair]);
        }
    });
    if e.call(SAVE_SWAPS, &args![]).u8() != 0 {
        fn_005fea20(e, Ptr::new(at + 0x38));
    }
    e.call(SAVE_FORM_DATA, &args![this, at + 0x38, 8u32]);
    if e.call(SAVE_SWAPS, &args![]).u8() != 0 {
        fn_005fea20(e, Ptr::new(at + 0x38));
    }
    e.call(SAVE_FORM_CLOSE, &args![this]);
}

// Translated from 005ff190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one byte word; the anim group
/// section setter): keeps bits 6 and 7 of `cAnimData` and ors in `section`.
pub fn fn_005ff190(e: &mut Engine, this: Ptr<TesIdleForm>, section: u32) {
    let kept = e.get(this, TesIdleForm::cAnimData) & 0xc0;
    e.set(this, TesIdleForm::cAnimData, kept);
    let value = e.get(this, TesIdleForm::cAnimData) | section as u8;
    e.set(this, TesIdleForm::cAnimData, value);
}

// Translated from 005ff220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one byte word): sets bit 6 of
/// `cAnimData` when the byte is non-zero, clears it otherwise.
pub fn fn_005ff220(e: &mut Engine, this: Ptr<TesIdleForm>, on: u32) {
    let value = e.get(this, TesIdleForm::cAnimData);
    let value = if on & 0xff != 0 {
        value | 0x40
    } else {
        value & 0xbf
    };
    e.set(this, TesIdleForm::cAnimData, value);
}

// Translated from 005ff260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`): bit 0 of the byte at +0x3E
/// (in `IDLE_DATA`).
pub fn fn_005ff260(e: &mut Engine, this: Ptr<TesIdleForm>) -> bool {
    e.mem.u8(this.addr() + 0x3e) & 1 != 0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x005f_e810,
            tes_idle_form_load(Ptr<TesIdleForm>, Ptr) -> bool
        ),
        entry!(0x005f_eb90, tes_idle_form_init_item(Ptr<TesIdleForm>)),
        entry!(
            0x005f_edb0,
            tes_idle_form_init_parents(Ptr<TesIdleForm>) -> bool
        ),
        entry!(
            0x005f_efa0,
            tes_idle_form_fix_next_idles(Ptr<TesIdleForm>, u32)
        ),
        entry!(
            0x005f_f160,
            tes_idle_form_get_idle_anim_group_section(Ptr<TesIdleForm>) -> u32
        ),
        entry!(0x005f_f1d0, fn_005ff1d0(Ptr<TesIdleForm>) -> bool),
        entry!(0x005f_f200, fn_005ff200(Ptr<TesIdleForm>) -> bool),
        entry!(
            0x005f_f280,
            tes_idle_form_check_idle_file(Ptr<TesIdleForm>) -> bool
        ),
        entry!(0x005f_ea20, fn_005fea20(Ptr)),
        entry!(0x005f_ea50, fn_005fea50(Ptr<TesIdleForm>)),
        entry!(0x005f_f190, fn_005ff190(Ptr<TesIdleForm>, u32)),
        entry!(0x005f_f220, fn_005ff220(Ptr<TesIdleForm>, u32)),
        entry!(0x005f_f260, fn_005ff260(Ptr<TesIdleForm>) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    type Log = Vec<(u32, Vec<u32>)>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn run(e: &mut Engine, addr: u32, words: &[u32]) -> (Ret, Log) {
        e.call_log = Some(vec![]);
        let result = e.call(addr, words);
        let log = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != addr)
            .collect();
        (result, log)
    }

    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn slot(offset: u32) -> u32 {
        0x0f10_0000 + offset
    }

    /// An engine whose idles share the vtable at `0f00_0000` (every slot a
    /// double returning 0 at `slot(offset)`), with zero doubles for `zero`.
    fn engine(zero: &[u32]) -> Engine {
        let mut e = Engine::new();
        for addr in zero {
            e.register(*addr, |_, _| ret(0));
        }
        let slots: Vec<u32> = (0..0x50).map(|i| slot(4 * i)).collect();
        for s in &slots {
            e.register(*s, |_, _| ret(0));
        }
        e.put_vtable(0x0f00_0000, &slots);
        e.map(IDLE_MANAGER, 4);
        e.mem.set_u32(IDLE_MANAGER, 0x6200);
        e
    }

    fn idle(e: &mut Engine) -> u32 {
        let idle = e.mem.alloc(0x54);
        e.mem.set_u32(idle, 0x0f00_0000);
        e.mem.set_u32(idle + 0x18, 0x0f00_0000);
        idle
    }

    #[test]
    fn flags_and_section() {
        let mut e = engine(&[]);
        let idle = idle(&mut e);
        e.mem.set_u8(idle + 0x38, 0xc5);
        assert_eq!(e.call(0x005f_f160, &args![idle]).u32(), 5);
        assert!(e.call(0x005f_f1d0, &args![idle]).bool());
        assert!(e.call(0x005f_f200, &args![idle]).bool());
        e.mem.set_u8(idle + 0x38, 0x3f);
        assert_eq!(e.call(0x005f_f160, &args![idle]).u32(), 0x3f);
        assert!(!e.call(0x005f_f1d0, &args![idle]).bool());
        assert!(!e.call(0x005f_f200, &args![idle]).bool());
    }

    #[test]
    fn check_idle_file_compares_the_extension() {
        let mut e = engine(&[STRRCHR, STRING_COMPARE]);
        let idle = idle(&mut e);
        e.register(slot(SLOT_MODEL_NAME), |_, _| ret(0x5000));
        let (result, log) = run(&mut e, 0x005f_f280, &[idle]);
        assert!(!result.bool());
        assert_eq!(calls(&log, slot(SLOT_MODEL_NAME)), vec![vec![idle + 0x18]]);
        assert_eq!(calls(&log, STRRCHR), vec![vec![0x5000, 0x2e]]);
        assert!(calls(&log, STRING_COMPARE).is_empty());
        e.register(STRRCHR, |_, _| ret(0x5008));
        let (result, log) = run(&mut e, 0x005f_f280, &[idle]);
        assert!(result.bool());
        assert_eq!(
            calls(&log, STRING_COMPARE),
            vec![vec![0x5008, IDLE_FILE_EXTENSION]]
        );
        e.register(STRING_COMPARE, |_, _| ret(1));
        assert!(!e.call(0x005f_f280, &args![idle]).bool());
    }

    fn load_engine(form_type: u32, chunks: Vec<u32>) -> Engine {
        let mut e = engine(&[
            TES_FORM_LOAD_FORM,
            GET_CHUNK_DATA,
            LOAD_MODEL_CHUNK,
            CONDITION_LOAD,
            TES_FORM_LOAD_DATA,
            FILL_MEMORY,
            SWAP_PAIR,
            MANAGER_005FFBF0,
            SWAP_DATA,
        ]);
        e.register_double(GET_TES_FORM, move |_, _| ret(form_type));
        e.register(CHUNK_SIZE, |_, _| ret(0x11));
        e.register(FILE_NEEDS_SWAP, |_, _| ret(1));
        e.register(CONDITIONS, |_, a| ret(a[0] + 0x30));
        let position = Rc::new(Cell::new(0usize));
        let (list, at) = (chunks.clone(), position.clone());
        e.register_double(GET_TES_CHUNK, move |_, _| {
            ret(list.get(at.get()).copied().unwrap_or(0))
        });
        let total = chunks.len();
        e.register_double(NEXT_CHUNK, move |_, _| {
            position.set(position.get() + 1);
            ret(u32::from(position.get() < total))
        });
        e.register(GET_CHUNK_DATA, |e, a| {
            if a[2] == 8 {
                e.mem.set_u32(a[1], 0x0100_0011);
                e.mem.set_u32(a[1] + 4, 0x0100_0022);
            }
            ret(0)
        });
        e
    }

    #[test]
    fn load_reads_each_chunk() {
        let chunks = vec![
            CHUNK_EDID, CHUNK_OBND, CHUNK_MODL, CHUNK_MODT, CHUNK_CTDA, CHUNK_DATA, CHUNK_ANAM,
        ];
        let mut e = load_engine(0x48, chunks);
        let idle = idle(&mut e);
        let (result, log) = run(&mut e, 0x005f_e810, &[idle, 0x7000]);
        assert!(result.bool());
        assert_eq!(calls(&log, TES_FORM_LOAD_FORM), vec![vec![idle, 0x7000]]);
        let reads = calls(&log, GET_CHUNK_DATA);
        assert_eq!(reads[0][2], EDITOR_ID_MAXIMUM);
        assert_eq!(
            calls(&log, slot(SLOT_SET_EDITOR_ID)),
            vec![vec![idle, reads[0][1]]]
        );
        assert_eq!(
            calls(&log, slot(SLOT_LOAD_BOUNDS)),
            vec![vec![idle, 0x7000]]
        );
        assert_eq!(
            calls(&log, LOAD_MODEL_CHUNK),
            vec![vec![idle + 0x18, 0x7000]; 2]
        );
        assert_eq!(calls(&log, CONDITION_LOAD), vec![vec![idle + 0x30, 0x7000]]);
        assert_eq!(
            calls(&log, TES_FORM_LOAD_DATA),
            vec![vec![idle, 0x7000, idle + 0x38, 8]]
        );
        assert_eq!(calls(&log, SWAP_DATA), vec![vec![idle + 0x38]]);
        let pair = reads[1][1];
        assert_eq!(calls(&log, FILL_MEMORY), vec![vec![pair, 0, 8]]);
        assert_eq!(calls(&log, SWAP_PAIR), vec![vec![pair]]);
        assert_eq!(
            (e.mem.u32(idle + 0x44), e.mem.u32(idle + 0x48)),
            (0x0100_0011, 0x0100_0022)
        );
        assert_eq!(calls(&log, MANAGER_005FFBF0), vec![vec![0x6200, idle]]);
    }

    #[test]
    fn load_refuses_another_record() {
        let mut e = load_engine(0x47, vec![CHUNK_DATA]);
        let idle = idle(&mut e);
        let (result, log) = run(&mut e, 0x005f_e810, &[idle, 0x7000]);
        assert!(!result.bool());
        assert_eq!(log, vec![(GET_TES_FORM, vec![0x7000])]);
    }

    const INIT_CALLEES: [u32; 12] = [
        CONDITIONS,
        CONDITION_INIT_ITEM,
        FORM_ID,
        LOG_WARNING,
        SET_SECTION,
        STRING_CONSTRUCT,
        STRING_DESTRUCT,
        STRING_CHARACTERS,
        SET_PARENT,
        SET_PREVIOUS,
        FORM_SET_484AB0,
        FORM_004013E0,
    ];

    fn init_engine() -> Engine {
        let mut e = engine(&INIT_CALLEES);
        e.register(GET_FILE, |_, _| ret(0x7100));
        e.register(ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            ret(0)
        });
        e.register(LOOKUP_FORM, |_, a| ret(a[0]));
        e.register(DYNAMIC_CAST, |_, a| {
            ret(if a[0] & 0xf == 1 { a[0] + 0x1000 } else { 0 })
        });
        e.register(FORM_ID, |_, _| ret(0x00ab_cdef));
        e.register(slot(SLOT_NAME), |_, _| ret(0x6600));
        e.register(STRING_CHARACTERS, |_, _| ret(0x6700));
        e
    }

    #[test]
    fn init_item_resets_a_bad_section_and_resolves_the_links() {
        let mut e = init_engine();
        let idle = idle(&mut e);
        e.mem.set_u8(idle + 0x38, 0xd6);
        e.mem.set_u32(idle + 0x44, 0x31);
        e.mem.set_u32(idle + 0x48, 0x42);
        let (_, log) = run(&mut e, 0x005f_eb90, &[idle]);
        assert_eq!(calls(&log, CONDITION_INIT_ITEM), vec![vec![0, idle]]);
        assert_eq!(calls(&log, SET_SECTION), vec![vec![idle, 7]]);
        assert_eq!(calls(&log, SET_PARENT), vec![vec![idle, 0x0100_1031]]);
        assert_eq!(calls(&log, SET_PREVIOUS), vec![vec![idle, 0]]);
        let describe = calls(&log, slot(SLOT_DESCRIBE));
        assert_eq!(describe.len(), 1);
        assert_eq!(
            calls(&log, LOG_WARNING),
            vec![
                vec![FORMAT_BAD_SECTION, 0x6600, 0x00ab_cdef],
                vec![FORMAT_NO_PREVIOUS, 0x0100_0042, 0x6700],
            ]
        );
        assert_eq!(calls(&log, FORM_SET_484AB0), vec![vec![idle, 1]]);
    }

    #[test]
    fn init_item_keeps_good_sections_and_skips_empty_links() {
        let mut e = init_engine();
        let idle = idle(&mut e);
        for section in [0u8, 7, 0x14, 0x15] {
            e.mem.set_u8(idle + 0x38, section);
            let (_, log) = run(&mut e, 0x005f_eb90, &[idle]);
            assert!(calls(&log, SET_SECTION).is_empty());
            assert!(calls(&log, SET_PARENT).is_empty());
            assert!(calls(&log, LOG_WARNING).is_empty());
        }
        for section in [8u8, 0x13, 0x16] {
            e.mem.set_u8(idle + 0x38, section);
            let (_, log) = run(&mut e, 0x005f_eb90, &[idle]);
            assert_eq!(calls(&log, SET_SECTION).len(), 1);
        }
        e.register(FORM_004013E0, |_, _| ret(1));
        let (_, log) = run(&mut e, 0x005f_eb90, &[idle]);
        assert_eq!(log, vec![(FORM_004013E0, vec![idle])]);
    }

    const PARENTS_CALLEES: [u32; 13] = [
        LOOSE_IDLE_ARRAY,
        ADD_ROOT_IDLE_ARRAY,
        FORM_ID,
        LOG_WARNING,
        ARRAY_INSERT,
        CHILD_ARRAY,
        INDEX_FOR_FORM,
        FORM_00440D80,
        STRING_CONSTRUCT,
        STRING_DESTRUCT,
        STRING_CHARACTERS,
        ARRAY_ELEMENT,
        DYNAMIC_CAST,
    ];

    #[test]
    fn init_parents_puts_a_loose_idle_at_the_end() {
        let mut e = engine(&PARENTS_CALLEES);
        let idle = idle(&mut e);
        e.mem.set_u8(idle + 0x38, 0x40);
        let (result, log) = run(&mut e, 0x005f_edb0, &[idle]);
        assert!(!result.bool());
        assert_eq!(calls(&log, LOOSE_IDLE_ARRAY), vec![vec![0x6200, idle]; 2]);
        assert_eq!(calls(&log, ADD_ROOT_IDLE_ARRAY), vec![vec![0x6200, idle]]);
        assert_eq!(
            calls(&log, LOG_WARNING),
            vec![vec![FORMAT_NO_LOOSE_ARRAY, 0, 0]]
        );
        e.register(LOOSE_IDLE_ARRAY, |_, _| ret(0x7000));
        e.register(FORM_ID, |_, a| ret(if a[0] == 0x7000 { 5 } else { 0 }));
        let (result, log) = run(&mut e, 0x005f_edb0, &[idle]);
        assert!(result.bool());
        assert_eq!(calls(&log, ARRAY_INSERT), vec![vec![0x7000, 5, idle]]);
    }

    #[test]
    fn init_parents_puts_an_idle_after_its_previous_one() {
        let mut e = engine(&PARENTS_CALLEES);
        let idle = idle(&mut e);
        e.mem.set_u32(idle + 0x44, 0x5100);
        e.mem.set_u32(idle + 0x48, 0x5200);
        e.register(CHILD_ARRAY, |_, _| ret(0x7000));
        e.register(INDEX_FOR_FORM, |_, _| ret(2));
        e.register(FORM_ID, |_, a| ret(if a[0] == 0x7000 { 5 } else { 0 }));
        let empty = e.mem.alloc(4);
        e.register_double(ARRAY_ELEMENT, move |_, _| ret(empty));
        let (result, log) = run(&mut e, 0x005f_edb0, &[idle]);
        assert!(result.bool());
        assert_eq!(calls(&log, CHILD_ARRAY), vec![vec![0x5100]]);
        assert_eq!(calls(&log, INDEX_FOR_FORM), vec![vec![0x7000, 0x5200]]);
        assert_eq!(calls(&log, ARRAY_INSERT), vec![vec![0x7000, 3, idle]]);
        // `FixNextIdles` walked the array.
        assert_eq!(calls(&log, ARRAY_ELEMENT).len(), 5);
        // Not found (-1), or `00440d80` true: at the end, count + 1.
        e.register(INDEX_FOR_FORM, |_, _| ret(0xffff_ffff));
        let (_, log) = run(&mut e, 0x005f_edb0, &[idle]);
        assert_eq!(calls(&log, ARRAY_INSERT), vec![vec![0x7000, 6, idle]]);
        e.register(INDEX_FOR_FORM, |_, _| ret(2));
        e.register(FORM_00440D80, |_, _| ret(1));
        let (_, log) = run(&mut e, 0x005f_edb0, &[idle]);
        assert_eq!(calls(&log, ARRAY_INSERT), vec![vec![0x7000, 6, idle]]);
        // Without a parent the root array is made; without any array it warns.
        e.mem.set_u32(idle + 0x44, 0);
        let (result, log) = run(&mut e, 0x005f_edb0, &[idle]);
        assert!(!result.bool());
        assert_eq!(calls(&log, ADD_ROOT_IDLE_ARRAY), vec![vec![0x6200, idle]]);
        assert_eq!(calls(&log, slot(SLOT_DESCRIBE)).len(), 1);
        assert_eq!(
            calls(&log, LOG_WARNING),
            vec![vec![FORMAT_NO_PARENT_ARRAY, 0]]
        );
    }

    #[test]
    fn fix_next_idles_moves_the_next_idle_after_this_one() {
        let mut e = engine(&[
            ARRAY_REMOVE_AT,
            ARRAY_0061BDC0,
            ARRAY_INSERT,
            LOG_WARNING,
            STRING_CONSTRUCT,
            STRING_DESTRUCT,
            STRING_CHARACTERS,
        ]);
        let this = idle(&mut e);
        let other = idle(&mut e);
        let next = idle(&mut e);
        // The array: [next, a non-idle, this, other]; next's previous is this.
        let elements = e.mem.alloc(0x10);
        for (i, form) in [next, 0x5000, this, other].iter().enumerate() {
            e.mem.set_u32(elements + 4 * i as u32, *form);
        }
        e.register(FORM_ID, |_, _| ret(4));
        e.register_double(ARRAY_ELEMENT, move |_, a| ret(elements + 4 * a[1]));
        e.register(DYNAMIC_CAST, |_, a| {
            ret(if a[0] == 0x5000 { 0 } else { a[0] })
        });
        e.register_double(GET_PREVIOUS, move |_, a| {
            ret(if a[0] == next { this } else { 0 })
        });
        let (_, log) = run(&mut e, 0x005f_efa0, &[this, 0x7000]);
        let removes = calls(&log, ARRAY_REMOVE_AT);
        assert_eq!(removes.len(), 1);
        assert_eq!(removes[0][..2], [0x7000, 0]);
        // Own index 2, less one non-idle, less one more for the next idle
        // being before it: inserted at 0 + 1.
        assert_eq!(calls(&log, ARRAY_INSERT), vec![vec![0x7000, 1, next]]);
        assert_eq!(calls(&log, ARRAY_0061BDC0), vec![vec![0x7000]; 2]);
        // The next idle's own next idle is looked for in turn.
        assert_eq!(calls(&log, FORM_ID), vec![vec![0x7000]; 2]);
        assert!(calls(&log, LOG_WARNING).is_empty());
    }

    #[test]
    fn fix_next_idles_leaves_a_next_idle_in_place_and_warns_of_two() {
        let mut e = engine(&[
            ARRAY_REMOVE_AT,
            ARRAY_0061BDC0,
            ARRAY_INSERT,
            LOG_WARNING,
            STRING_CONSTRUCT,
            STRING_DESTRUCT,
        ]);
        let this = idle(&mut e);
        let next = idle(&mut e);
        let second = idle(&mut e);
        let elements = e.mem.alloc(0xc);
        for (i, form) in [this, next, second].iter().enumerate() {
            e.mem.set_u32(elements + 4 * i as u32, *form);
        }
        e.register(FORM_ID, |_, _| ret(3));
        e.register(STRING_CHARACTERS, |_, _| ret(0x6700));
        e.register_double(ARRAY_ELEMENT, move |_, a| ret(elements + 4 * a[1]));
        e.register(DYNAMIC_CAST, |_, a| ret(a[0]));
        e.register_double(GET_PREVIOUS, move |_, a| {
            ret(if a[0] == next || a[0] == second {
                this
            } else {
                0
            })
        });
        let (_, log) = run(&mut e, 0x005f_efa0, &[this, 0x7000]);
        assert!(calls(&log, ARRAY_REMOVE_AT).is_empty());
        assert_eq!(
            calls(&log, LOG_WARNING),
            vec![vec![FORMAT_SEVERAL_NEXT, 0x6700]]
        );
        let (_, log) = run(&mut e, 0x005f_efa0, &[this, 0]);
        assert!(log.is_empty());
    }
}

#[cfg(test)]
mod n043_tests {
    use super::*;

    const STATE: u32 = 0x7300_0000;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// Idles at 0x1000-aligned addresses: the word at +0x100 is the parent,
    /// +0x104 the previous idle, +0x108 the grouping flag.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(STATE, 0x40);
        for addr in [
            SWAP_SHORT,
            SAVE_FORM_START,
            SAVE_MODEL,
            SAVE_CONDITIONS,
            SAVE_FORM_DATA,
            SAVE_FORM_CLOSE,
            SWAP_PAIR,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(SAVE_SWAPS, |e, _| e.mem.u32(STATE).into_ret());
        e.register(FILL_MEMORY, |e, a| {
            for offset in 0..a[2] {
                e.mem.set_u8(a[0] + offset, a[1] as u8);
            }
            Ret::default()
        });
        e.register(CONDITIONS, |_, a| (a[0] + 0x50).into_ret());
        e.register(IDLE_PARENT, |e, a| e.mem.u32(a[0] + 0x100).into_ret());
        e.register(IDLE_PREVIOUS, |e, a| e.mem.u32(a[0] + 0x104).into_ret());
        e.register(IDLE_IS_GROUPING, |e, a| e.mem.u32(a[0] + 0x108).into_ret());
        e.register(FORM_ID, |_, a| (a[0] + 1).into_ret());
        e.register_double(SAVE_CHUNK_DATA, |e, a| {
            let first = e.mem.u32(a[1]);
            let second = e.mem.u32(a[1] + 4);
            e.mem.set_u32(STATE + 4, first);
            e.mem.set_u32(STATE + 8, second);
            Ret::default()
        });
        e.map(0x7400_0000, 0x4000);
        e
    }

    #[test]
    fn swap_short_at_4() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        e.call(0x005f_ea20, &args![0x1000u32]);
        assert_eq!(calls(&e, SWAP_SHORT), vec![vec![0x1004, 0]]);
    }

    #[test]
    fn save_writes_model_conditions_pair_and_data() {
        let mut e = engine();
        let this = 0x7400_0000u32;
        let grouping = 0x7400_1000u32;
        let previous = 0x7400_2000u32;
        e.mem.set_u32(this + 0x100, 0x5000);
        e.mem.set_u32(this + 0x104, grouping);
        e.mem.set_u32(grouping + 0x108, 1);
        e.mem.set_u32(grouping + 0x104, previous);
        e.mem.set_u32(STATE, 1);
        e.call_log = Some(vec![]);
        e.call(0x005f_ea50, &args![this]);
        assert_eq!(
            calls(&e, SAVE_MODEL),
            vec![vec![this + 0x18, CHUNK_MODL, CHUNK_MODT]]
        );
        assert_eq!(calls(&e, SAVE_CONDITIONS), vec![vec![this + 0x50]]);
        assert_eq!(e.mem.u32(STATE + 4), 0x5001);
        assert_eq!(e.mem.u32(STATE + 8), previous + 1);
        assert_eq!(calls(&e, SAVE_CHUNK_DATA)[0][0], CHUNK_ANAM);
        assert_eq!(calls(&e, SWAP_PAIR).len(), 2);
        assert_eq!(calls(&e, SWAP_SHORT), vec![vec![this + 0x3c, 0]; 2]);
        assert_eq!(calls(&e, SAVE_FORM_DATA), vec![vec![this, this + 0x38, 8]]);
        assert_eq!(calls(&e, SAVE_FORM_CLOSE), vec![vec![this]]);
        // A grouping idle keeps no previous one; nothing is swapped.
        e.mem.set_u32(this + 0x108, 1);
        e.mem.set_u32(this + 0x100, 0);
        e.mem.set_u32(STATE, 0);
        e.call_log = Some(vec![]);
        e.call(0x005f_ea50, &args![this]);
        assert_eq!((e.mem.u32(STATE + 4), e.mem.u32(STATE + 8)), (0, 0));
        assert!(calls(&e, IDLE_PREVIOUS).is_empty());
        assert!(calls(&e, SWAP_PAIR).is_empty());
    }

    #[test]
    fn anim_data_bits() {
        let mut e = engine();
        let this = e.mem.alloc(0x54);
        e.mem.set_u8(this + 0x38, 0xff);
        e.call(0x005f_f190, &args![this, 0x1234_5605u32]);
        assert_eq!(e.mem.u8(this + 0x38), 0xc5);
        e.call(0x005f_f220, &args![this, 0x100u32]);
        assert_eq!(e.mem.u8(this + 0x38), 0x85);
        e.call(0x005f_f220, &args![this, 1u32]);
        assert_eq!(e.mem.u8(this + 0x38), 0xc5);
        assert!(!e.call(0x005f_f260, &args![this]).bool());
        e.mem.set_u8(this + 0x3e, 3);
        assert!(e.call(0x005f_f260, &args![this]).bool());
    }
}
