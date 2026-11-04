//! `fallout shared/bgsragdoll.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005ef560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<char_const_P_1024>` scalar deleting destructor (engine map name): runs the destructor body `005eee10` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn bs_simple_array_char_const_p_1024_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x005e_ee10, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 005ef590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<short_1024>` scalar deleting destructor (engine map name): runs the destructor body `005eee60` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn bs_simple_array_short_1024_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x005e_ee60, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// The vtable of `BSSimpleArray<LookIK_DATA::LookIK_Head_DATA,1024>` (RTTI
/// `.?AV?$BSSimpleArray@ULookIK_Head_DATA@LookIK_DATA@@$0EAA@@`).
const LOOK_IK_ARRAY_VTABLE: u32 = 0x0104_730c;
/// The vtable of `BSSimpleArray<GrabIK_DATA::GrabIK_Arm_DATA,1024>` (RTTI
/// `.?AV?$BSSimpleArray@UGrabIK_Arm_DATA@GrabIK_DATA@@$0EAA@@`).
const GRAB_IK_ARRAY_VTABLE: u32 = 0x0104_7320;
/// `008454f0` (unnamed in the engine map; `thiscall`, `RET 4`): the
/// arrays' release with its byte (free the buffer when set).
const ARRAY_RELEASE_008454F0: u32 = 0x0084_54f0;
/// `MemoryManager` allocation (`00401000`, cdecl, the size).
const ALLOCATE: u32 = 0x0040_1000;
/// The virtual slot (byte offset) of the arrays' buffer allocation (the
/// `005ef330` / `005ef510` overrides).
const SLOT_ALLOCATE: u32 = 0x04;
/// `005ef9a0` (unnamed in the engine map; `thiscall`, `RET 8`: first,
/// count): constructs `LookIK_Head_DATA` elements in place.
const LOOK_IK_CONSTRUCT_005EF9A0: u32 = 0x005e_f9a0;
/// `005efa80` (unnamed in the engine map; `thiscall`, `RET 8`: capacity,
/// count): moves the `LookIK_Head_DATA` buffer to a new capacity.
const LOOK_IK_RESERVE_005EFA80: u32 = 0x005e_fa80;
/// `005efbc0` (unnamed in the engine map; `thiscall`, `RET 8`: first,
/// count): constructs `GrabIK_Arm_DATA` elements in place.
const GRAB_IK_CONSTRUCT_005EFBC0: u32 = 0x005e_fbc0;
/// `005efc90` (unnamed in the engine map; `thiscall`, `RET 8`: capacity,
/// count): moves the `GrabIK_Arm_DATA` buffer to a new capacity.
const GRAB_IK_RESERVE_005EFC90: u32 = 0x005e_fc90;
/// `0072ba80` (unnamed in the engine map; `thiscall`, `RET 8`: first,
/// count): the step run on the elements dropped by a shrink.
const ARRAY_DROP_0072BA80: u32 = 0x0072_ba80;

/// One `BSSimpleArray` instance's element size and its two element helpers.
struct ArrayKind {
    element: u32,
    construct: u32,
    reserve: u32,
}

const LOOK_IK: ArrayKind = ArrayKind {
    element: 0x28,
    construct: LOOK_IK_CONSTRUCT_005EF9A0,
    reserve: LOOK_IK_RESERVE_005EFA80,
};

const GRAB_IK: ArrayKind = ArrayKind {
    element: 6,
    construct: GRAB_IK_CONSTRUCT_005EFBC0,
    reserve: GRAB_IK_RESERVE_005EFC90,
};

/// The shared body of `005efb30` and `005efd40` (`BSSimpleArray`'s
/// initialization with a reserve and a size; buffer +4, size +8,
/// capacity +0xC).
fn array_initialize(e: &mut Engine, this: Ptr, reserve: u32, size: u32, kind: &ArrayKind) {
    let base = this.addr();
    e.mem.set_u32(base + 4, 0);
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u32(base + 0xc, 0);
    let mut reserve = reserve;
    if size > reserve {
        reserve = size;
    }
    if reserve > 0 {
        let buffer = e.vcall(base, SLOT_ALLOCATE, &args![reserve]).u32();
        e.mem.set_u32(base + 4, buffer);
        e.mem.set_u32(base + 0xc, reserve);
    }
    if size > 0 {
        let buffer = e.mem.u32(base + 4);
        e.call(kind.construct, &args![this, buffer, size]);
        e.mem.set_u32(base + 8, size);
    }
}

/// The shared body of `005ef1f0` and `005ef3d0` (`BSSimpleArray::SetSize`).
fn array_set_size(e: &mut Engine, this: Ptr, size: u32, free_extra: u8, kind: &ArrayKind) {
    let base = this.addr();
    if size == 0 {
        e.call(ARRAY_RELEASE_008454F0, &args![this, u32::from(free_extra)]);
        return;
    }
    if size > e.mem.u32(base + 0xc) {
        if e.mem.u32(base + 0xc) == 0 {
            let buffer = e.vcall(base, SLOT_ALLOCATE, &args![size]).u32();
            e.mem.set_u32(base + 4, buffer);
            e.mem.set_u32(base + 0xc, size);
        } else {
            let count = e.mem.u32(base + 8);
            e.call(kind.reserve, &args![this, size, count]);
            e.mem.set_u32(base + 0xc, size);
        }
        let count = e.mem.u32(base + 8);
        let first = count
            .wrapping_mul(kind.element)
            .wrapping_add(e.mem.u32(base + 4));
        e.call(
            kind.construct,
            &args![this, first, size.wrapping_sub(count)],
        );
        e.mem.set_u32(base + 8, size);
    } else if size < e.mem.u32(base + 8) {
        let count = e.mem.u32(base + 8);
        let first = size
            .wrapping_mul(kind.element)
            .wrapping_add(e.mem.u32(base + 4));
        e.call(
            ARRAY_DROP_0072BA80,
            &args![this, first, count.wrapping_sub(size)],
        );
        e.mem.set_u32(base + 8, size);
        if free_extra != 0 && size <= e.mem.u32(base + 0xc) >> 2 {
            e.call(kind.reserve, &args![this, size, size]);
            e.mem.set_u32(base + 0xc, size);
        }
    } else {
        let count = e.mem.u32(base + 8);
        let first = count
            .wrapping_mul(kind.element)
            .wrapping_add(e.mem.u32(base + 4));
        e.call(
            kind.construct,
            &args![this, first, size.wrapping_sub(count)],
        );
        e.mem.set_u32(base + 8, size);
    }
}

// Translated from 005ef1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<LookIK_Head_DATA,1024>` constructor (RTTI of the vtable;
/// unnamed in the engine map): the vtable, then `005efb30(this, 0, 0)`.
/// Returns `this`.
pub fn fn_005ef1a0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), LOOK_IK_ARRAY_VTABLE);
    fn_005efb30(e, this, 0, 0);
    this
}

// Translated from 005ef1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<LookIK_Head_DATA,1024>` destructor (unnamed in the
/// engine map): the vtable, then `008454f0(this, 1)`.
pub fn fn_005ef1d0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), LOOK_IK_ARRAY_VTABLE);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

// Translated from 005ef380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<GrabIK_Arm_DATA,1024>` constructor (RTTI of the vtable;
/// unnamed in the engine map): the vtable, then `005efd40(this, 0, 0)`.
/// Returns `this`.
pub fn fn_005ef380(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), GRAB_IK_ARRAY_VTABLE);
    fn_005efd40(e, this, 0, 0);
    this
}

// Translated from 005ef3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<GrabIK_Arm_DATA,1024>` destructor (unnamed in the engine
/// map): the vtable, then `008454f0(this, 1)`.
pub fn fn_005ef3b0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), GRAB_IK_ARRAY_VTABLE);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

// Translated from 005ef330 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `LookIK_Head_DATA` array's buffer allocation (its vtable slot +4;
/// unnamed in the engine map; `RET 4`): `count * 0x28` bytes (wrapping
/// `IMUL`) from `00401000`.
pub fn fn_005ef330(e: &mut Engine, _this: Ptr, count: u32) -> u32 {
    e.call(ALLOCATE, &args![count.wrapping_mul(0x28)]).u32()
}

// Translated from 005ef510 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `GrabIK_Arm_DATA` array's buffer allocation (its vtable slot +4;
/// unnamed in the engine map; `RET 4`): `count * 6` bytes from `00401000`.
pub fn fn_005ef510(e: &mut Engine, _this: Ptr, count: u32) -> u32 {
    e.call(ALLOCATE, &args![count.wrapping_mul(6)]).u32()
}

// Translated from 005efb30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<LookIK_Head_DATA,1024>` initialization (unnamed in the
/// engine map; `RET 8`): clears buffer, size and capacity; the reserve is
/// raised to `size` when smaller (unsigned); a non-zero reserve allocates
/// the buffer through vtable slot +4 and sets the capacity; a non-zero
/// size constructs that many elements (`005ef9a0`) and sets the size.
pub fn fn_005efb30(e: &mut Engine, this: Ptr, reserve: u32, size: u32) {
    array_initialize(e, this, reserve, size, &LOOK_IK);
}

// Translated from 005efd40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<GrabIK_Arm_DATA,1024>` initialization (unnamed in the
/// engine map; `RET 8`), as `005efb30` with the element helper `005efbc0`.
pub fn fn_005efd40(e: &mut Engine, this: Ptr, reserve: u32, size: u32) {
    array_initialize(e, this, reserve, size, &GRAB_IK);
}

// Translated from 005ef1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<LookIK_Head_DATA,1024>::SetSize` (unnamed in the engine
/// map; `RET 8`): size 0 releases (`008454f0` with `free_extra`). Above the
/// capacity, the buffer is allocated (no capacity yet, vtable slot +4) or
/// moved (`005efa80(size, count)`), the capacity becomes `size`, and the
/// new elements are constructed (`005ef9a0`). Below the current size, the
/// dropped elements go through `0072ba80`, and with `free_extra` set and
/// the new size at most a quarter of the capacity the buffer shrinks to it
/// (`005efa80(size, size)`). Otherwise the elements from the current size
/// up are constructed. The size becomes `size` in every non-zero case.
pub fn fn_005ef1f0(e: &mut Engine, this: Ptr, size: u32, free_extra: u8) {
    array_set_size(e, this, size, free_extra, &LOOK_IK);
}

// Translated from 005ef3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<GrabIK_Arm_DATA,1024>::SetSize` (unnamed in the engine
/// map; `RET 8`), as `005ef1f0` with 6-byte elements and the helpers
/// `005efbc0` / `005efc90`.
pub fn fn_005ef3d0(e: &mut Engine, this: Ptr, size: u32, free_extra: u8) {
    array_set_size(e, this, size, free_extra, &GRAB_IK);
}

/// `004013e0` (unnamed in the engine map): the form's byte that makes
/// `InitItem` return at once.
const FORM_SKIP_004013E0: u32 = 0x0040_13e0;
/// `00483710` (unnamed in the engine map): the `TESForm` step `InitItem` runs first.
const FORM_INIT_00483710: u32 = 0x0048_3710;
/// `TESForm::GetFile` (`00484e60`, Xbox PDB; `thiscall`, one word).
const TES_FORM_GET_FILE: u32 = 0x0048_4e60;
/// `TESForm::AddCompileIndex` (`00485d50`, Xbox PDB; cdecl, the address of
/// a form id and a file).
const TES_FORM_ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// `004839c0` (unnamed in the engine map; cdecl, one word): the form of an id.
const LOOKUP_FORM_004839C0: u32 = 0x0048_39c0;
/// `__RTDynamicCast` (`00ec43fb`, cdecl, five words).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The type descriptor of `TESForm` (`.?AVTESForm@@`).
const TYPE_TES_FORM: u32 = 0x0118_3028;
/// The type descriptor of `BGSBodyPartData` (`.?AVBGSBodyPartData@@`).
const TYPE_BGS_BODY_PART_DATA: u32 = 0x0118_61bc;
/// The type descriptor of `TESActorBase` (`.?AVTESActorBase@@`).
const TYPE_TES_ACTOR_BASE: u32 = 0x0118_46e8;
/// `0084e3a0` (unnamed in the engine map): the form's id word.
const FORM_ID_0084E3A0: u32 = 0x0084_e3a0;
/// The virtual slot (byte offset) of the form's text the warnings print.
const SLOT_FORM_TEXT: u32 = 0x130;
/// `005b5e40` (unnamed in the engine map; cdecl, a format and its words): prints a warning.
const PRINT_WARNING_005B5E40: u32 = 0x005b_5e40;
/// `"MASTERFILE: Could not find body part data (%08X) for ragdoll '%s' (%08X)."`.
const NO_BODY_PART_DATA_FORMAT: u32 = 0x0104_7280;
/// `"MASTERFILE: Could not find preview actor (%08X) for ragdoll '%s' (%08X)."`.
const NO_PREVIEW_ACTOR_FORMAT: u32 = 0x0104_7230;
/// `00484ab0` (unnamed in the engine map; `thiscall`, `RET 4`): the
/// `TESForm` step `InitItem` ends with (with 1).
const FORM_DONE_00484AB0: u32 = 0x0048_4ab0;

/// Resolves the form id at `this + offset` to a form of the type
/// `target`, as `BGSRagdoll::InitItem` does for both of its references.
fn resolve(e: &mut Engine, this: Ptr, offset: u32, target: u32, format: u32) {
    let id = e.mem.u32(this.addr() + offset);
    if id == 0 {
        return;
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), id);
        let file = e
            .call(TES_FORM_GET_FILE, &args![this, 0xffff_ffffu32])
            .u32();
        e.call(TES_FORM_ADD_COMPILE_INDEX, &args![slot, file]);
        let id = e.mem.u32(slot.addr());
        let form = e.call(LOOKUP_FORM_004839C0, &args![id]).u32();
        let cast = e
            .call(
                RT_DYNAMIC_CAST,
                &args![form, 0u32, TYPE_TES_FORM, target, 0u32],
            )
            .u32();
        e.mem.set_u32(this.addr() + offset, cast);
        if e.mem.u32(this.addr() + offset) == 0 {
            let own_id = e.call(FORM_ID_0084E3A0, &args![this]).u32();
            let text = e.vcall(this.addr(), SLOT_FORM_TEXT, &args![]).u32();
            let id = e.mem.u32(slot.addr());
            e.call(PRINT_WARNING_005B5E40, &args![format, id, text, own_id]);
        }
    });
}

// Translated from 005eea20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSRagdoll::InitItem` (Xbox PDB): returns at once when the byte
/// `004013e0` is set. Otherwise runs `00483710`, then resolves the body
/// part data id at +0x140 (to `BGSBodyPartData`) and the preview actor id
/// at +0x144 (to `TESActorBase`): each non-zero id gets its compile index
/// from the form's file (`GetFile(-1)`), is looked up and dynamically cast
/// in place; a failed cast prints the matching "Could not find" warning
/// with the id, the form's text (slot +0x130) and its own id. Ends with
/// `00484ab0(this, 1)`.
pub fn bgs_ragdoll_init_item(e: &mut Engine, this: Ptr) {
    if e.call(FORM_SKIP_004013E0, &args![this]).u8() != 0 {
        return;
    }
    e.call(FORM_INIT_00483710, &args![this]);
    resolve(
        e,
        this,
        0x140,
        TYPE_BGS_BODY_PART_DATA,
        NO_BODY_PART_DATA_FORMAT,
    );
    resolve(e, this, 0x144, TYPE_TES_ACTOR_BASE, NO_PREVIEW_ACTOR_FORMAT);
    e.call(FORM_DONE_00484AB0, &args![this, 1u32]);
}

/// `TESFile::GetTESForm` (`00472660`, engine map): the current record's type.
const GET_TES_FORM: u32 = 0x0047_2660;
/// `TESForm::LoadForm` (`00485110`, engine map; `thiscall`, `RET 4`).
const TES_FORM_LOAD_FORM: u32 = 0x0048_5110;
/// `TESFile::GetTESChunk` (`004726b0`, Xbox PDB).
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// `00401660` (unnamed in the engine map): the file's current chunk size.
const CHUNK_SIZE_00401660: u32 = 0x0040_1660;
/// `TESFile::GetChunkData_ov3` (`00472890`, Xbox PDB; buffer and size).
const GET_CHUNK_DATA: u32 = 0x0047_2890;
/// `TESFile::GetChunkData` (`004727f0`, engine map; `thiscall`, `RET 4`).
const GET_CHUNK_DATA_WORD: u32 = 0x0047_27f0;
/// `004726f0` (unnamed in the engine map): moves to the next chunk (false at the end).
const ADVANCE_004726F0: u32 = 0x0047_26f0;
/// `00401680` (unnamed in the engine map): the file's byte at +0x299.
const FILE_FLAG_00401680: u32 = 0x0040_1680;
/// The form's virtual slot (byte offset) given the editor id.
const SLOT_SET_EDITOR_ID: u32 = 0x134;
/// The form's virtual slot (byte offset) that reads the bounds chunk.
const SLOT_LOAD_BOUNDS: u32 = 0xe0;
/// `TESModel::LoadModelChunk` (`004892d0`, engine map; cdecl, the model and the file).
const LOAD_MODEL_CHUNK: u32 = 0x0048_92d0;
/// `005ee510` (unnamed in the engine map): fixes up the `DATA` record read
/// when the file's byte is set.
const FIX_DATA_005EE510: u32 = 0x005e_e510;
/// `BSSimpleArray<unsigned short,1024>::SetSize` (`005eee80`, engine map; `RET 8`).
const SHORT_ARRAY_SET_SIZE: u32 = 0x005e_ee80;
/// `005ef010` (unnamed in the engine map; `thiscall`, `RET 8`): the
/// `SetSize` of the array at +0xC0.
const ARRAY_SET_SIZE_005EF010: u32 = 0x005e_f010;
/// `005ee380` (unnamed in the engine map): fixes up the `RAFD` block.
const FIX_FEEDBACK_005EE380: u32 = 0x005e_e380;
/// `005ee580` (unnamed in the engine map): fixes up the `RAPS` block.
const FIX_POSE_005EE580: u32 = 0x005e_e580;
/// `0044ddc0` (unnamed in the engine map): an array's size.
const ARRAY_SIZE_0044DDC0: u32 = 0x0044_ddc0;
/// `005eeda0` (unnamed in the engine map; `thiscall`, `RET 4`): the address
/// of an element of the array at +0x80.
const SHORT_ARRAY_AT_005EEDA0: u32 = 0x005e_eda0;
/// `00407a90` (unnamed in the engine map; cdecl, an address and a byte):
/// fixes up one 16-bit element.
const FIX_SHORT_00407A90: u32 = 0x0040_7a90;
/// `004037f0` (unnamed in the engine map; `thiscall`, `RET 8`): sets a
/// string from a C string and a length.
const STRING_SET: u32 = 0x0040_37f0;

// Translated from 005ee640 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005ee640 - the /GS cookie check (00ec408c) of its stack frame is not translated
/// `BGSRagdoll::Load` (Xbox PDB; `thiscall`, `RET 4`): false unless the
/// record's type (`TESFile::GetTESForm`) is 0x63. Then `TESForm::LoadForm`
/// and, chunk by chunk until a zero chunk type or the last chunk:
/// `EDID` (read 0x200 bytes into a stack buffer of the chunk's size, the
/// exe's `_alloca`; here the engine's stack block; given to slot +0x134),
/// `OBND` (slot +0xE0 with the file), `MODL`/`MODT` (`LoadModelChunk` on
/// the model at +0x18, or null for a null `this`), `DATA` (14 bytes, fixed
/// up by `005ee510` when the file's byte is set: four 16-bit sizes for the
/// arrays at +0x80, +0xC0, +0xE4 and +0x108, each `SetSize(n, 1)`, and five
/// bytes to +0x40, +0x90, +0xD0, +0xF4 and +0x118), `XNAM` (a word to +0x144),
/// `TNAM` (a word to +0x140), `RAFD` (0x3C bytes to +0x44, fixed up by
/// `005ee380`), `RAFB` (the 16-bit elements of the array at +0x80, each
/// fixed up by `00407a90`), `RAPS` (0x18 bytes to +0x124, fixed up by
/// `005ee580`) and `ANAM` (a stack buffer of the chunk's size, read with
/// size 0 into the string at +0x11C). Returns true.
pub fn bgs_ragdoll_load(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    if e.call(GET_TES_FORM, &args![file]).u32() != 0x63 {
        return false;
    }
    let base = this.addr();
    e.call(TES_FORM_LOAD_FORM, &args![this, file]);
    loop {
        let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            // EDID
            0x4449_4445 => {
                let size = e.call(CHUNK_SIZE_00401660, &args![file]).u32();
                e.with_stack(size, |e, buffer| {
                    e.call(GET_CHUNK_DATA, &args![file, buffer, 0x200u32]);
                    e.vcall(base, SLOT_SET_EDITOR_ID, &args![buffer]);
                });
            }
            // OBND
            0x444e_424f => {
                e.vcall(base, SLOT_LOAD_BOUNDS, &args![file]);
            }
            // MODL, MODT
            0x4c44_4f4d | 0x5444_4f4d => {
                let model = if this.is_null() { 0 } else { base + 0x18 };
                e.call(LOAD_MODEL_CHUNK, &args![model, file]);
            }
            // DATA
            0x4154_4144 => {
                e.with_stack(0x10, |e, data| {
                    e.call(GET_CHUNK_DATA, &args![file, data, 0xeu32]);
                    if e.call(FILE_FLAG_00401680, &args![file]).u8() != 0 {
                        e.call(FIX_DATA_005EE510, &args![data]);
                    }
                    let d = data.addr();
                    let count = u32::from(e.mem.u16(d));
                    e.call(SHORT_ARRAY_SET_SIZE, &args![base + 0x80, count, 1u32]);
                    let count = u32::from(e.mem.u16(d + 2));
                    e.call(ARRAY_SET_SIZE_005EF010, &args![base + 0xc0, count, 1u32]);
                    let count = u32::from(e.mem.u16(d + 4));
                    fn_005ef1f0(e, Ptr::new(base + 0xe4), count, 1);
                    let count = u32::from(e.mem.u16(d + 6));
                    fn_005ef3d0(e, Ptr::new(base + 0x108), count, 1);
                    for (index, offset) in [0x40u32, 0x90, 0xd0, 0xf4, 0x118].iter().enumerate() {
                        let byte = e.mem.u8(d + 8 + index as u32);
                        e.mem.set_u8(base + offset, byte);
                    }
                });
            }
            // XNAM, TNAM
            0x4d41_4e58 | 0x4d41_4e54 => {
                let offset = if chunk == 0x4d41_4e58 { 0x144 } else { 0x140 };
                let word = e.with_stack(4, |e, slot| {
                    e.mem.set_u32(slot.addr(), 0);
                    e.call(GET_CHUNK_DATA_WORD, &args![file, slot]);
                    e.mem.u32(slot.addr())
                });
                e.mem.set_u32(base + offset, word);
            }
            // RAFD
            0x4446_4152 => {
                e.call(GET_CHUNK_DATA, &args![file, base + 0x44, 0x3cu32]);
                if e.call(FILE_FLAG_00401680, &args![file]).u8() != 0 {
                    e.call(FIX_FEEDBACK_005EE380, &args![base + 0x44]);
                }
            }
            // RAFB
            0x4246_4152 => {
                let bytes = e.call(ARRAY_SIZE_0044DDC0, &args![base + 0x80]).u32() << 1;
                let first = e
                    .call(SHORT_ARRAY_AT_005EEDA0, &args![base + 0x80, 0u32])
                    .u32();
                e.call(GET_CHUNK_DATA, &args![file, first, bytes]);
                let count = e.call(ARRAY_SIZE_0044DDC0, &args![base + 0x80]).u32();
                if e.call(FILE_FLAG_00401680, &args![file]).u8() != 0 {
                    let mut index: u32 = 0;
                    while index < count {
                        let element = e
                            .call(SHORT_ARRAY_AT_005EEDA0, &args![base + 0x80, index])
                            .u32();
                        e.call(FIX_SHORT_00407A90, &args![element, 0u32]);
                        index = index.wrapping_add(1);
                    }
                }
            }
            // RAPS
            0x5350_4152 => {
                e.call(GET_CHUNK_DATA, &args![file, base + 0x124, 0x18u32]);
                if e.call(FILE_FLAG_00401680, &args![file]).u8() != 0 {
                    e.call(FIX_POSE_005EE580, &args![base + 0x124]);
                }
            }
            // ANAM
            0x4d41_4e41 => {
                let size = e.call(CHUNK_SIZE_00401660, &args![file]).u32();
                e.with_stack(size, |e, buffer| {
                    e.call(GET_CHUNK_DATA, &args![file, buffer, 0u32]);
                    e.call(STRING_SET, &args![base + 0x11c, buffer, 0u32]);
                });
            }
            _ => {}
        }
        if e.call(ADVANCE_004726F0, &args![file]).u8() == 0 {
            break;
        }
    }
    true
}

/// `TESForm::StartForm`, `TESForm::CloseForm` (Xbox PDB, `004855a0`,
/// `00485680`).
const START_FORM: u32 = 0x0048_55a0;
const CLOSE_FORM: u32 = 0x0048_5680;
/// `TESModel::Save` (Xbox PDB, `004891d0`; `RET 8`: two signatures).
const MODEL_SAVE: u32 = 0x0048_91d0;
/// Chunk writers (cdecl): a word (`00485910`), data
/// (`TESForm::__AddChunkData`, `00485990`: signature, data, size), 16-bit
/// values (`00485730`: signature, data, count), bytes
/// (`TESForm::AddChunkArray`, `004856f0`: signature, data, size).
const ADD_CHUNK_WORD: u32 = 0x0048_5910;
const ADD_CHUNK_DATA: u32 = 0x0048_5990;
const ADD_CHUNK_SHORTS: u32 = 0x0048_5730;
const ADD_CHUNK_ARRAY: u32 = 0x0048_56f0;
/// Whether the data written needs byte swapping (`00401500`, no arguments).
const SAVE_SWAPS: u32 = 0x0040_1500;
/// Swaps a 32-bit word in place (`00401080`, cdecl: address, a byte 0).
const WORD_SWAP_00401080: u32 = 0x0040_1080;
/// A `BSStringT`'s length (`004048e0`) and characters (`00559450`).
const STRING_LENGTH: u32 = 0x0040_48e0;
const STRING_CHARS: u32 = 0x0055_9450;
/// The reallocation wrapper (`0042f5d0`, cdecl: buffer, bytes; the new
/// buffer in `EAX`).
const REALLOCATE: u32 = 0x0042_f5d0;
/// The `FootIK_Leg_DATA` array's destructor body (`005eeff0`).
const FOOT_IK_ARRAY_DESTRUCTOR: u32 = 0x005e_eff0;
/// The deallocation wrapper (`00401030`, cdecl).
const DEALLOCATE: u32 = 0x0040_1030;
/// The chunk signatures as the words the exe pushes: `MODL`, `MODT`, `NVER`,
/// `DATA`, `XNAM`, `TNAM`, `RAFD`, `RAFB`, `RAPS`, `ANAM`.
const SIG_MODL: u32 = 0x4c44_4f4d;
const SIG_MODT: u32 = 0x5444_4f4d;
const SIG_NVER: u32 = 0x5245_564e;
const SIG_DATA: u32 = 0x4154_4144;
const SIG_XNAM: u32 = 0x4d41_4e58;
const SIG_TNAM: u32 = 0x4d41_4e54;
const SIG_RAFD: u32 = 0x4446_4152;
const SIG_RAFB: u32 = 0x4246_4152;
const SIG_RAPS: u32 = 0x5350_4152;
const SIG_ANAM: u32 = 0x4d41_4e41;

/// `00407a90` (cdecl: address, a byte 0) on each 16-bit value, `00401080`
/// on each word, as the swaps below call them.
fn swap_shorts(e: &mut Engine, at: u32, count: u32) {
    for i in 0..count {
        e.call(FIX_SHORT_00407A90, &args![at + 2 * i, 0u32]);
    }
}

fn swap_words(e: &mut Engine, at: u32, count: u32) {
    for i in 0..count {
        e.call(WORD_SWAP_00401080, &args![at + 4 * i, 0u32]);
    }
}

// Translated from 005ee110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSRagdoll::Save` (Xbox PDB): starts the form, saves the model at
/// +0x18 (`MODL`/`MODT`), `NVER` 1, then the 14-byte `DATA` block from the
/// stack: the sizes (low 16 bits of `0044ddc0`) of the arrays at +0x80,
/// +0xC0, +0xE4 and +0x108 and the bytes at +0x40, +0x90, +0xD0, +0xF4 and
/// +0x118 (its last byte is never written by the exe and holds whatever
/// the stack held; here the stack block's 0), swapped by `005ee510` before
/// and after when `00401500` says so. Then `XNAM` and `TNAM` with the form
/// ids at +0x144 and +0x140 when set, `RAFD` (the 0x3C bytes at +0x44,
/// `005ee380` around it), `RAFB` (the 16-bit values of the array at +0x80,
/// from its element 0, when it has any), `RAPS` (the 0x18 bytes at
/// +0x124, `005ee580` around it), `ANAM` (the string at +0x11C with its
/// zero, when not empty), and closes the form.
pub fn bgs_ragdoll_save(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(START_FORM, &args![this]);
    e.call(MODEL_SAVE, &args![base + 0x18, SIG_MODL, SIG_MODT]);
    e.call(ADD_CHUNK_WORD, &args![SIG_NVER, 1u32]);
    e.with_stack(0xe, |e, data| {
        for (i, array) in [0x80u32, 0xc0, 0xe4, 0x108].into_iter().enumerate() {
            let size = e.call(ARRAY_SIZE_0044DDC0, &args![base + array]).u16();
            e.mem.set_u16(data.addr() + 2 * i as u32, size);
        }
        for (i, field) in [0x40u32, 0x90, 0xd0, 0xf4, 0x118].into_iter().enumerate() {
            let byte = e.mem.u8(base + field);
            e.mem.set_u8(data.addr() + 8 + i as u32, byte);
        }
        if e.call(SAVE_SWAPS, &args![]).bool() {
            fn_005ee510(e, data);
        }
        e.call(ADD_CHUNK_DATA, &args![SIG_DATA, data, 0xeu32]);
        if e.call(SAVE_SWAPS, &args![]).bool() {
            fn_005ee510(e, data);
        }
    });
    for (field, chunk) in [(0x144, SIG_XNAM), (0x140, SIG_TNAM)] {
        let form = e.mem.u32(base + field);
        if form != 0 {
            let id = e.call(FORM_ID_0084E3A0, &args![form]).u32();
            e.call(ADD_CHUNK_WORD, &args![chunk, id]);
        }
    }
    let feedback = Ptr::new(base + 0x44);
    if e.call(SAVE_SWAPS, &args![]).bool() {
        fn_005ee380(e, feedback);
    }
    e.call(ADD_CHUNK_DATA, &args![SIG_RAFD, feedback, 0x3cu32]);
    if e.call(SAVE_SWAPS, &args![]).bool() {
        fn_005ee380(e, feedback);
    }
    if e.call(ARRAY_SIZE_0044DDC0, &args![base + 0x80]).u32() != 0 {
        let count = e.call(ARRAY_SIZE_0044DDC0, &args![base + 0x80]).u32();
        let first = e
            .call(SHORT_ARRAY_AT_005EEDA0, &args![base + 0x80, 0u32])
            .u32();
        e.call(ADD_CHUNK_SHORTS, &args![SIG_RAFB, first, count]);
    }
    let pose = Ptr::new(base + 0x124);
    if e.call(SAVE_SWAPS, &args![]).bool() {
        fn_005ee580(e, pose);
    }
    e.call(ADD_CHUNK_DATA, &args![SIG_RAPS, pose, 0x18u32]);
    if e.call(SAVE_SWAPS, &args![]).bool() {
        fn_005ee580(e, pose);
    }
    if e.call(STRING_LENGTH, &args![base + 0x11c]).u32() != 0 {
        let length = e.call(STRING_LENGTH, &args![base + 0x11c]).u32();
        let chars = e.call(STRING_CHARS, &args![base + 0x11c]).u32();
        e.call(
            ADD_CHUNK_ARRAY,
            &args![SIG_ANAM, chars, length.wrapping_add(1)],
        );
    }
    e.call(CLOSE_FORM, &args![this]);
}

// Translated from 005ee380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Swaps the 15 words of the 0x3C-byte `RAFD` block (`00401080` on each,
/// with 0).
pub fn fn_005ee380(e: &mut Engine, this: Ptr) {
    swap_words(e, this.addr(), 15);
}

// Translated from 005ee510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Swaps the first four 16-bit values of the `DATA` block (`00407a90` on
/// each, with 0); its bytes are left alone.
pub fn fn_005ee510(e: &mut Engine, this: Ptr) {
    swap_shorts(e, this.addr(), 4);
}

// Translated from 005ee580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Swaps the `RAPS` block: three 16-bit values at +0 (`00407a90`), then
/// the four words from +8 (`00401080`), each with 0.
pub fn fn_005ee580(e: &mut Engine, this: Ptr) {
    swap_shorts(e, this.addr(), 3);
    swap_words(e, this.addr() + 8, 4);
}

// Translated from 005ef150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<FootIK_DATA::FootIK_Leg_DATA,1024>::_Allocate` (Xbox
/// PDB; `RET 4`): `count * 0x38` bytes (wrapping `IMUL`) from `00401000`.
pub fn bs_simple_array_foot_ik_leg_data_1024_allocate(
    e: &mut Engine,
    _this: Ptr,
    count: u32,
) -> u32 {
    e.call(ALLOCATE, &args![count.wrapping_mul(0x38)]).u32()
}

// Translated from 005ef180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<FootIK_DATA::FootIK_Leg_DATA,1024>::_Reallocate` (Xbox
/// PDB; `RET 8`): `0042f5d0(buffer, count * 0x38)`, its `EAX` passed on.
pub fn bs_simple_array_foot_ik_leg_data_1024_reallocate(
    e: &mut Engine,
    _this: Ptr,
    buffer: u32,
    count: u32,
) -> u32 {
    e.call(REALLOCATE, &args![buffer, count.wrapping_mul(0x38)])
        .u32()
}

// Translated from 005ef360 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `LookIK_Head_DATA` array's reallocation (unnamed in the engine map;
/// `RET 8`): `0042f5d0(buffer, count * 0x28)`, its `EAX` passed on.
pub fn fn_005ef360(e: &mut Engine, _this: Ptr, buffer: u32, count: u32) -> u32 {
    e.call(REALLOCATE, &args![buffer, count.wrapping_mul(0x28)])
        .u32()
}

// Translated from 005ef540 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `GrabIK_Arm_DATA` array's reallocation (unnamed in the engine map;
/// `RET 8`): `0042f5d0(buffer, count * 6)`, its `EAX` passed on.
pub fn fn_005ef540(e: &mut Engine, _this: Ptr, buffer: u32, count: u32) -> u32 {
    e.call(REALLOCATE, &args![buffer, count.wrapping_mul(6)])
        .u32()
}

// Translated from 005ef5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<FootIK_DATA::FootIK_Leg_DATA,1024>` scalar deleting
/// destructor (Xbox PDB; `RET 4`): the destructor body `005eeff0`, then
/// `00401030(this)` when bit 0 of `flags` is set. Returns `this`.
pub fn bs_simple_array_foot_ik_leg_data_1024_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(FOOT_IK_ARRAY_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

// Translated from 005ef5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<LookIK_DATA::LookIK_Head_DATA,1024>` scalar deleting
/// destructor (Xbox PDB; `RET 4`): the destructor `fn_005ef1d0`, then
/// `00401030(this)` when bit 0 of `flags` is set. Returns `this`.
pub fn bs_simple_array_look_ik_head_data_1024_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_005ef1d0(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

// Translated from 005ef620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<GrabIK_DATA::GrabIK_Arm_DATA,1024>` scalar deleting
/// destructor (Xbox PDB; `RET 4`): the destructor `fn_005ef3b0`, then
/// `00401030(this)` when bit 0 of `flags` is set. Returns `this`.
pub fn bs_simple_array_grab_ik_arm_data_1024_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_005ef3b0(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005e_e640, bgs_ragdoll_load(Ptr, Ptr) -> bool),
        entry!(0x005e_f560, bs_simple_array_char_const_p_1024_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x005e_f590, bs_simple_array_short_1024_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x005e_f1a0, fn_005ef1a0(Ptr) -> Ptr),
        entry!(0x005e_f1d0, fn_005ef1d0(Ptr)),
        entry!(0x005e_f380, fn_005ef380(Ptr) -> Ptr),
        entry!(0x005e_f3b0, fn_005ef3b0(Ptr)),
        entry!(0x005e_f330, fn_005ef330(Ptr, u32) -> u32),
        entry!(0x005e_f510, fn_005ef510(Ptr, u32) -> u32),
        entry!(0x005e_fb30, fn_005efb30(Ptr, u32, u32)),
        entry!(0x005e_fd40, fn_005efd40(Ptr, u32, u32)),
        entry!(0x005e_f1f0, fn_005ef1f0(Ptr, u32, u8)),
        entry!(0x005e_f3d0, fn_005ef3d0(Ptr, u32, u8)),
        entry!(0x005e_ea20, bgs_ragdoll_init_item(Ptr)),
        entry!(0x005e_e110, bgs_ragdoll_save(Ptr)),
        entry!(0x005e_e380, fn_005ee380(Ptr)),
        entry!(0x005e_e510, fn_005ee510(Ptr)),
        entry!(0x005e_e580, fn_005ee580(Ptr)),
        entry!(
            0x005e_f150,
            bs_simple_array_foot_ik_leg_data_1024_allocate(Ptr, u32) -> u32
        ),
        entry!(
            0x005e_f180,
            bs_simple_array_foot_ik_leg_data_1024_reallocate(Ptr, u32, u32) -> u32
        ),
        entry!(0x005e_f360, fn_005ef360(Ptr, u32, u32) -> u32),
        entry!(0x005e_f540, fn_005ef540(Ptr, u32, u32) -> u32),
        entry!(
            0x005e_f5c0,
            bs_simple_array_foot_ik_leg_data_1024_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x005e_f5f0,
            bs_simple_array_look_ik_head_data_1024_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x005e_f620,
            bs_simple_array_grab_ik_arm_data_1024_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod ragdoll_load_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    /// A file double: record type at +0x10, chunk index at +0, chunk count
    /// at +4, chunk types from +0x20, the flag byte at +0x299 and the chunk
    /// size at +0x25C (24 for every chunk).
    fn file(e: &mut Engine, record: u32, chunks: &[u32], flag: u8) -> u32 {
        let file = e.mem.alloc(0x300);
        e.mem.set_u32(file + 4, chunks.len() as u32);
        e.mem.set_u32(file + 0x10, record);
        for (i, chunk) in chunks.iter().enumerate() {
            e.mem.set_u32(file + 0x20 + 4 * i as u32, *chunk);
        }
        e.mem.set_u8(file + 0x299, flag);
        e.mem.set_u32(file + 0x25c, 24);
        e.register(GET_TES_FORM, |e, a| e.mem.u32(a[0] + 0x10).into_ret());
        e.register(GET_TES_CHUNK, |e, a| {
            let index = e.mem.u32(a[0]);
            e.mem.u32(a[0] + 0x20 + 4 * index).into_ret()
        });
        e.register(ADVANCE_004726F0, |e, a| {
            let index = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], index);
            u32::from(index < e.mem.u32(a[0] + 4)).into_ret()
        });
        e.register(CHUNK_SIZE_00401660, |e, a| {
            e.mem.u32(a[0] + 0x25c).into_ret()
        });
        e.register(FILE_FLAG_00401680, |e, a| {
            u32::from(e.mem.u8(a[0] + 0x299)).into_ret()
        });
        // DATA: sizes 2, 3, 0, 0 and bytes 0x11..0x15; a word chunk: 0x77.
        e.register(GET_CHUNK_DATA, |e, a| {
            let index = e.mem.u32(a[0]);
            if e.mem.u32(a[0] + 0x20 + 4 * index) == 0x4154_4144 {
                e.mem.write(
                    a[1],
                    &[2, 0, 3, 0, 0, 0, 0, 0, 0x11, 0x12, 0x13, 0x14, 0x15, 0],
                );
            }
            Ret::default()
        });
        e.register(GET_CHUNK_DATA_WORD, |e, a| {
            e.mem.set_u32(a[1], 0x77);
            Ret::default()
        });
        file
    }

    fn quiet(e: &mut Engine) {
        for address in [
            TES_FORM_LOAD_FORM,
            LOAD_MODEL_CHUNK,
            FIX_DATA_005EE510,
            SHORT_ARRAY_SET_SIZE,
            ARRAY_SET_SIZE_005EF010,
            FIX_FEEDBACK_005EE380,
            FIX_POSE_005EE580,
            FIX_SHORT_00407A90,
            STRING_SET,
            ARRAY_RELEASE_008454F0,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(ARRAY_SIZE_0044DDC0, |_, _| 2u32.into_ret());
        e.register(SHORT_ARRAY_AT_005EEDA0, |_, a| {
            (0x6000 + 2 * a[1]).into_ret()
        });
    }

    #[test]
    fn at_005ee640_refuses_other_records() {
        let mut e = Engine::new();
        quiet(&mut e);
        let file = file(&mut e, 0x62, &[0x4154_4144], 0);
        let this = e.mem.alloc(0x150);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x005e_e640, &args![this, file]).bool());
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, TES_FORM_LOAD_FORM).is_empty());
    }

    #[test]
    fn at_005ee640_reads_every_chunk() {
        let mut e = Engine::new();
        quiet(&mut e);
        let chunks = [
            0x4449_4445, // EDID
            0x444e_424f, // OBND
            0x4c44_4f4d, // MODL
            0x4154_4144, // DATA
            0x4d41_4e58, // XNAM
            0x4d41_4e54, // TNAM
            0x4446_4152, // RAFD
            0x4246_4152, // RAFB
            0x5350_4152, // RAPS
            0x4d41_4e41, // ANAM
            0x1234_5678, // unknown
        ];
        let file = file(&mut e, 0x63, &chunks, 1);
        let mut slots = vec![0u32; 0x4e];
        slots[0x38] = 0x0f06_1000;
        slots[0x4d] = 0x0f06_2000;
        e.put_vtable(0x0f06_0000, &slots);
        e.register(0x0f06_1000, |_, _| Ret::default());
        e.register(0x0f06_2000, |_, _| Ret::default());
        let this = e.mem.alloc(0x150);
        e.mem.set_u32(this, 0x0f06_0000);
        // The two IK arrays at +0xE4 and +0x108 have no capacity: size 0
        // releases them.
        e.call_log = Some(vec![]);
        assert!(e.call(0x005e_e640, &args![this, file]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, TES_FORM_LOAD_FORM), vec![vec![this, file]]);
        let edid = calls_to(&log, 0x0f06_2000);
        assert_eq!(edid.len(), 1);
        let reads = calls_to(&log, GET_CHUNK_DATA);
        assert_eq!(reads[0][0], file);
        assert_eq!(reads[0][2], 0x200);
        assert_eq!(edid[0], vec![this, reads[0][1]]);
        assert_eq!(calls_to(&log, 0x0f06_1000), vec![vec![this, file]]);
        assert_eq!(
            calls_to(&log, LOAD_MODEL_CHUNK),
            vec![vec![this + 0x18, file]]
        );
        assert_eq!(reads[1][2], 0xe);
        assert_eq!(calls_to(&log, FIX_DATA_005EE510), vec![vec![reads[1][1]]]);
        assert_eq!(
            calls_to(&log, SHORT_ARRAY_SET_SIZE),
            vec![vec![this + 0x80, 2, 1]]
        );
        assert_eq!(
            calls_to(&log, ARRAY_SET_SIZE_005EF010),
            vec![vec![this + 0xc0, 3, 1]]
        );
        assert_eq!(
            calls_to(&log, ARRAY_RELEASE_008454F0),
            vec![vec![this + 0xe4, 1], vec![this + 0x108, 1]]
        );
        for (offset, byte) in [
            (0x40u32, 0x11u8),
            (0x90, 0x12),
            (0xd0, 0x13),
            (0xf4, 0x14),
            (0x118, 0x15),
        ] {
            assert_eq!(e.mem.u8(this + offset), byte);
        }
        assert_eq!(e.mem.u32(this + 0x144), 0x77);
        assert_eq!(e.mem.u32(this + 0x140), 0x77);
        assert_eq!(reads[2], vec![file, this + 0x44, 0x3c]);
        assert_eq!(
            calls_to(&log, FIX_FEEDBACK_005EE380),
            vec![vec![this + 0x44]]
        );
        assert_eq!(reads[3], vec![file, 0x6000, 4]);
        assert_eq!(
            calls_to(&log, FIX_SHORT_00407A90),
            vec![vec![0x6000, 0], vec![0x6002, 0]]
        );
        assert_eq!(reads[4], vec![file, this + 0x124, 0x18]);
        assert_eq!(calls_to(&log, FIX_POSE_005EE580), vec![vec![this + 0x124]]);
        assert_eq!(reads[5][2], 0);
        assert_eq!(
            calls_to(&log, STRING_SET),
            vec![vec![this + 0x11c, reads[5][1], 0]]
        );
        assert_eq!(reads.len(), 6);
    }

    #[test]
    fn at_005ee640_stops_at_a_zero_chunk_and_passes_no_model_for_null() {
        let mut e = Engine::new();
        quiet(&mut e);
        let file = file(&mut e, 0x63, &[0x5444_4f4d, 0, 0x4c44_4f4d], 0);
        e.call_log = Some(vec![]);
        assert!(e.call(0x005e_e640, &args![0u32, file]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, LOAD_MODEL_CHUNK), vec![vec![0, file]]);
    }
}

#[cfg(test)]
mod ragdoll_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    const ARRAY_SLOTS: u32 = 0x0f04_0000;
    const ALLOCATE_SLOT: u32 = 0x0f04_1000;

    /// An array object whose slot +4 allocates `0x1000 + count` (a marker).
    fn array(e: &mut Engine, buffer: u32, size: u32, capacity: u32) -> u32 {
        e.put_vtable(ARRAY_SLOTS, &[0, ALLOCATE_SLOT]);
        e.register(ALLOCATE_SLOT, |_, a| (0x1000 + a[1]).into_ret());
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, ARRAY_SLOTS);
        e.mem.set_u32(this + 4, buffer);
        e.mem.set_u32(this + 8, size);
        e.mem.set_u32(this + 0xc, capacity);
        this
    }

    fn quiet(e: &mut Engine) {
        for address in [
            LOOK_IK_CONSTRUCT_005EF9A0,
            LOOK_IK_RESERVE_005EFA80,
            GRAB_IK_CONSTRUCT_005EFBC0,
            GRAB_IK_RESERVE_005EFC90,
            ARRAY_DROP_0072BA80,
            ARRAY_RELEASE_008454F0,
        ] {
            e.register(address, |_, _| Ret::default());
        }
    }

    #[test]
    fn at_005ef1a0_and_005ef380_construct_empty_arrays() {
        for (address, vtable) in [
            (0x005e_f1a0u32, LOOK_IK_ARRAY_VTABLE),
            (0x005e_f380, GRAB_IK_ARRAY_VTABLE),
        ] {
            let mut e = Engine::new();
            quiet(&mut e);
            let this = e.mem.alloc(0x10);
            e.mem.write(this, &[0xcd; 0x10]);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(address, &args![this]).u32(), this);
            let log = e.call_log.take().unwrap();
            assert_eq!(e.mem.u32(this), vtable);
            for offset in [4u32, 8, 0xc] {
                assert_eq!(e.mem.u32(this + offset), 0);
            }
            assert_eq!(log.len(), 1);
        }
    }

    #[test]
    fn at_005ef1d0_and_005ef3b0_release_with_one() {
        for (address, vtable) in [
            (0x005e_f1d0u32, LOOK_IK_ARRAY_VTABLE),
            (0x005e_f3b0, GRAB_IK_ARRAY_VTABLE),
        ] {
            let mut e = Engine::new();
            quiet(&mut e);
            let this = e.mem.alloc(0x10);
            e.call_log = Some(vec![]);
            e.call(address, &args![this]);
            let log = e.call_log.take().unwrap();
            assert_eq!(e.mem.u32(this), vtable);
            assert_eq!(calls_to(&log, ARRAY_RELEASE_008454F0), vec![vec![this, 1]]);
        }
    }

    #[test]
    fn at_005ef330_and_005ef510_allocate_element_sized_buffers() {
        let mut e = Engine::new();
        e.register(ALLOCATE, |_, a| a[0].into_ret());
        assert_eq!(e.call(0x005e_f330, &args![0u32, 3u32]).u32(), 0x78);
        assert_eq!(e.call(0x005e_f510, &args![0u32, 3u32]).u32(), 18);
        // The product wraps as the exe's IMUL does.
        assert_eq!(
            e.call(0x005e_f330, &args![0u32, 0x0800_0000u32]).u32(),
            0x4000_0000
        );
    }

    #[test]
    fn at_005efb30_reserves_and_constructs() {
        let mut e = Engine::new();
        quiet(&mut e);
        let this = array(&mut e, 0xdead, 0xdead, 0xdead);
        e.call_log = Some(vec![]);
        e.call(0x005e_fb30, &args![this, 2u32, 5u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, ALLOCATE_SLOT), vec![vec![this, 5]]);
        assert_eq!(e.mem.u32(this + 4), 0x1005);
        assert_eq!(e.mem.u32(this + 0xc), 5);
        assert_eq!(e.mem.u32(this + 8), 5);
        assert_eq!(
            calls_to(&log, LOOK_IK_CONSTRUCT_005EF9A0),
            vec![vec![this, 0x1005, 5]]
        );

        e.call_log = Some(vec![]);
        e.call(0x005e_fd40, &args![this, 0u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 1);
        for offset in [4u32, 8, 0xc] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }

        e.call_log = Some(vec![]);
        e.call(0x005e_fd40, &args![this, 4u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this + 0xc), 4);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert!(calls_to(&log, GRAB_IK_CONSTRUCT_005EFBC0).is_empty());
    }

    #[test]
    fn at_005ef1f0_set_size_branches() {
        let mut e = Engine::new();
        quiet(&mut e);
        // Size 0: release with the byte.
        let this = array(&mut e, 0x5000, 3, 8);
        e.call_log = Some(vec![]);
        e.call(0x005e_f1f0, &args![this, 0u32, 0x0101u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, ARRAY_RELEASE_008454F0), vec![vec![this, 1]]);
        assert_eq!(e.mem.u32(this + 8), 3);

        // Growing without a buffer: allocate through slot +4.
        let this = array(&mut e, 0, 0, 0);
        e.call_log = Some(vec![]);
        e.call(0x005e_f1f0, &args![this, 4u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, ALLOCATE_SLOT), vec![vec![this, 4]]);
        assert_eq!(
            calls_to(&log, LOOK_IK_CONSTRUCT_005EF9A0),
            vec![vec![this, 0x1004, 4]]
        );
        assert_eq!((e.mem.u32(this + 8), e.mem.u32(this + 0xc)), (4, 4));

        // Growing past the capacity: move, then construct the new tail.
        let this = array(&mut e, 0x5000, 2, 3);
        e.call_log = Some(vec![]);
        e.call(0x005e_f1f0, &args![this, 6u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, LOOK_IK_RESERVE_005EFA80),
            vec![vec![this, 6, 2]]
        );
        assert_eq!(
            calls_to(&log, LOOK_IK_CONSTRUCT_005EF9A0),
            vec![vec![this, 0x5000 + 2 * 0x28, 4]]
        );
        assert_eq!((e.mem.u32(this + 8), e.mem.u32(this + 0xc)), (6, 6));

        // Growing within the capacity.
        let this = array(&mut e, 0x5000, 2, 8);
        e.call_log = Some(vec![]);
        e.call(0x005e_f1f0, &args![this, 5u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, LOOK_IK_CONSTRUCT_005EF9A0),
            vec![vec![this, 0x5000 + 2 * 0x28, 3]]
        );
        assert_eq!((e.mem.u32(this + 8), e.mem.u32(this + 0xc)), (5, 8));

        // Shrinking, with and without freeing the extra.
        let this = array(&mut e, 0x5000, 6, 8);
        e.call_log = Some(vec![]);
        e.call(0x005e_f1f0, &args![this, 2u32, 1u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, ARRAY_DROP_0072BA80),
            vec![vec![this, 0x5000 + 2 * 0x28, 4]]
        );
        assert_eq!(
            calls_to(&log, LOOK_IK_RESERVE_005EFA80),
            vec![vec![this, 2, 2]]
        );
        assert_eq!((e.mem.u32(this + 8), e.mem.u32(this + 0xc)), (2, 2));
        let this = array(&mut e, 0x5000, 6, 8);
        e.call_log = Some(vec![]);
        e.call(0x005e_f1f0, &args![this, 3u32, 1u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, LOOK_IK_RESERVE_005EFA80).is_empty());
        assert_eq!((e.mem.u32(this + 8), e.mem.u32(this + 0xc)), (3, 8));
    }

    #[test]
    fn at_005ef3d0_uses_six_byte_elements() {
        let mut e = Engine::new();
        quiet(&mut e);
        let this = array(&mut e, 0x5000, 2, 3);
        e.call_log = Some(vec![]);
        e.call(0x005e_f3d0, &args![this, 6u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, GRAB_IK_RESERVE_005EFC90),
            vec![vec![this, 6, 2]]
        );
        assert_eq!(
            calls_to(&log, GRAB_IK_CONSTRUCT_005EFBC0),
            vec![vec![this, 0x5000 + 12, 4]]
        );
        let this = array(&mut e, 0x5000, 8, 8);
        e.call_log = Some(vec![]);
        e.call(0x005e_f3d0, &args![this, 1u32, 1u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, ARRAY_DROP_0072BA80),
            vec![vec![this, 0x5000 + 6, 7]]
        );
        assert_eq!(
            calls_to(&log, GRAB_IK_RESERVE_005EFC90),
            vec![vec![this, 1, 1]]
        );
    }

    fn init_engine() -> Engine {
        let mut e = Engine::new();
        e.register(FORM_SKIP_004013E0, |e, a| {
            u32::from(e.mem.u8(a[0] + 0x150)).into_ret()
        });
        e.register(FORM_INIT_00483710, |_, _| Ret::default());
        e.register(TES_FORM_GET_FILE, |_, _| 0x00f1_1e00u32.into_ret());
        e.register(TES_FORM_ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            Ret::default()
        });
        e.register(LOOKUP_FORM_004839C0, |_, a| (a[0] & 0xffff).into_ret());
        // The cast succeeds for forms below 0x8000.
        e.register(RT_DYNAMIC_CAST, |_, a| {
            let cast = if a[0] < 0x8000 { a[0] } else { 0 };
            cast.into_ret()
        });
        e.register(FORM_ID_0084E3A0, |_, _| 0x00ab_cdefu32.into_ret());
        let mut slots = vec![0u32; 0x4d];
        slots[0x4c] = 0x0f04_2000;
        e.put_vtable(0x0f04_3000, &slots);
        e.register(0x0f04_2000, |_, _| 0x0f04_4000u32.into_ret());
        e.register(PRINT_WARNING_005B5E40, |_, _| Ret::default());
        e.register(FORM_DONE_00484AB0, |_, _| Ret::default());
        e
    }

    #[test]
    fn at_005eea20_resolves_both_references() {
        let mut e = init_engine();
        let this = e.mem.alloc(0x154);
        e.mem.set_u32(this, 0x0f04_3000);
        e.mem.set_u32(this + 0x140, 0x1234);
        e.mem.set_u32(this + 0x144, 0x9000);
        e.call_log = Some(vec![]);
        e.call(0x005e_ea20, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this + 0x140), 0x1234);
        assert_eq!(e.mem.u32(this + 0x144), 0);
        let casts = calls_to(&log, RT_DYNAMIC_CAST);
        assert_eq!(
            casts,
            vec![
                vec![0x1234, 0, TYPE_TES_FORM, TYPE_BGS_BODY_PART_DATA, 0],
                vec![0x9000, 0, TYPE_TES_FORM, TYPE_TES_ACTOR_BASE, 0],
            ]
        );
        assert_eq!(
            calls_to(&log, TES_FORM_GET_FILE)[0],
            vec![this, 0xffff_ffff]
        );
        assert_eq!(
            calls_to(&log, PRINT_WARNING_005B5E40),
            vec![vec![
                NO_PREVIEW_ACTOR_FORMAT,
                0x0100_9000,
                0x0f04_4000,
                0x00ab_cdef
            ]]
        );
        assert_eq!(calls_to(&log, FORM_DONE_00484AB0), vec![vec![this, 1]]);
    }

    #[test]
    fn at_005eea20_skips_null_ids_and_returns_early_on_the_byte() {
        let mut e = init_engine();
        let this = e.mem.alloc(0x154);
        e.mem.set_u32(this, 0x0f04_3000);
        e.mem.set_u32(this + 0x140, 0x9999);
        e.call_log = Some(vec![]);
        e.call(0x005e_ea20, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, PRINT_WARNING_005B5E40),
            vec![vec![
                NO_BODY_PART_DATA_FORMAT,
                0x0100_9999,
                0x0f04_4000,
                0x00ab_cdef
            ]]
        );
        assert_eq!(calls_to(&log, RT_DYNAMIC_CAST).len(), 1);

        e.mem.set_u8(this + 0x150, 1);
        e.call_log = Some(vec![]);
        e.call(0x005e_ea20, &args![this]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, FORM_INIT_00483710).is_empty());
        assert!(calls_to(&log, FORM_DONE_00484AB0).is_empty());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_005ef560_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x005e_ee10, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x005e_f560, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x005e_f560)
                .collect();
            let mut expected = vec![(0x005e_ee10, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_005ef590_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x005e_ee60, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x005e_f590, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x005e_f590)
                .collect();
            let mut expected = vec![(0x005e_ee60, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod n037_tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    type Chunks = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u8>)>>>;

    /// Save doubles: array sizes are the word at +8 of the array, the
    /// swaps add 1 to each byte they touch (the low byte for words and
    /// 16-bit values), form ids the form plus 1; returns the data chunks.
    fn engine(swaps: bool) -> (Engine, Chunks) {
        let mut e = Engine::new();
        for addr in [
            START_FORM,
            CLOSE_FORM,
            MODEL_SAVE,
            ADD_CHUNK_WORD,
            ADD_CHUNK_SHORTS,
            ADD_CHUNK_ARRAY,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        if swaps {
            e.register(SAVE_SWAPS, |_, _| ret(1));
        } else {
            e.register(SAVE_SWAPS, |_, _| ret(0));
        }
        e.register(FIX_SHORT_00407A90, |e, a| {
            let b = e.mem.u8(a[0]);
            e.mem.set_u8(a[0], b + 1);
            Ret::default()
        });
        e.register(WORD_SWAP_00401080, |e, a| {
            let b = e.mem.u8(a[0]);
            e.mem.set_u8(a[0], b + 1);
            Ret::default()
        });
        e.register(ARRAY_SIZE_0044DDC0, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(SHORT_ARRAY_AT_005EEDA0, |e, a| {
            ret(e.mem.u32(a[0] + 4) + 2 * a[1])
        });
        e.register(FORM_ID_0084E3A0, |_, a| ret(a[0] + 1));
        e.register(STRING_LENGTH, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(STRING_CHARS, |e, a| ret(e.mem.u32(a[0])));
        let chunks = Chunks::default();
        let seen = chunks.clone();
        e.register_double(ADD_CHUNK_DATA, move |e, a| {
            let bytes = (0..a[2]).map(|i| e.mem.u8(a[1] + i)).collect();
            seen.borrow_mut().push((a[0], bytes));
            Ret::default()
        });
        e.call_log = Some(vec![]);
        (e, chunks)
    }

    #[test]
    fn save_writes_every_chunk() {
        let (mut e, chunks) = engine(false);
        let this = e.mem.alloc(0x148);
        for (array, size) in [(0x80u32, 2u32), (0xc0, 3), (0xe4, 4), (0x108, 0x10005)] {
            e.mem.set_u32(this + array + 8, size);
        }
        let shorts = e.mem.alloc(4);
        e.mem.set_u32(this + 0x84, shorts);
        for (i, field) in [0x40u32, 0x90, 0xd0, 0xf4, 0x118].into_iter().enumerate() {
            e.mem.set_u8(this + field, 0x10 + i as u8);
        }
        e.mem.set_u32(this + 0x144, 0x500);
        e.mem.set_u32(this + 0x11c, 0x9000);
        e.mem.set_u32(this + 0x120, 4);
        e.call(0x005e_e110, &[this]);
        let chunks = chunks.borrow();
        assert_eq!(chunks.len(), 3);
        assert_eq!(
            chunks[0],
            (
                SIG_DATA,
                vec![2, 0, 3, 0, 4, 0, 5, 0, 0x10, 0x11, 0x12, 0x13, 0x14, 0]
            )
        );
        assert_eq!(chunks[1].0, SIG_RAFD);
        assert_eq!(chunks[1].1.len(), 0x3c);
        assert_eq!(chunks[2].0, SIG_RAPS);
        assert_eq!(chunks[2].1.len(), 0x18);
        assert_eq!(
            calls_to(&e, ADD_CHUNK_WORD),
            vec![vec![SIG_NVER, 1], vec![SIG_XNAM, 0x501]]
        );
        assert_eq!(
            calls_to(&e, ADD_CHUNK_SHORTS),
            vec![vec![SIG_RAFB, shorts, 2]]
        );
        assert_eq!(
            calls_to(&e, ADD_CHUNK_ARRAY),
            vec![vec![SIG_ANAM, 0x9000, 5]]
        );
        assert_eq!(
            calls_to(&e, MODEL_SAVE),
            vec![vec![this + 0x18, SIG_MODL, SIG_MODT]]
        );
    }

    #[test]
    fn save_swaps_around_each_block() {
        let (mut e, chunks) = engine(true);
        let this = e.mem.alloc(0x148);
        e.mem.set_u32(this + 0x140, 0x700);
        e.call(0x005e_e110, &[this]);
        let chunks = chunks.borrow();
        // DATA: the four 16-bit values swapped, the bytes not.
        assert_eq!(chunks[0].1[..9], [1, 0, 1, 0, 1, 0, 1, 0, 0]);
        // RAFD: each of the 15 words.
        assert!((0..15).all(|i| chunks[1].1[4 * i] == 1));
        // RAPS: three 16-bit values, then four words from +8.
        assert_eq!(chunks[2].1[..9], [1, 0, 1, 0, 1, 0, 0, 0, 1]);
        assert_eq!(chunks[2].1[0x14], 1);
        // Swapped back afterwards.
        assert_eq!(e.mem.u8(this + 0x44), 2);
        assert_eq!(e.mem.u8(this + 0x124), 2);
        assert!(calls_to(&e, ADD_CHUNK_SHORTS).is_empty());
        assert!(calls_to(&e, ADD_CHUNK_ARRAY).is_empty());
        assert_eq!(calls_to(&e, ADD_CHUNK_WORD)[1], vec![SIG_TNAM, 0x701]);
    }

    #[test]
    fn swaps_touch_their_fields_with_zero() {
        let (mut e, _) = engine(false);
        let block = e.mem.alloc(0x40);
        e.call(0x005e_e380, &[block]);
        e.call(0x005e_e510, &[block]);
        e.call(0x005e_e580, &[block]);
        let words = calls_to(&e, WORD_SWAP_00401080);
        let shorts = calls_to(&e, FIX_SHORT_00407A90);
        assert_eq!(words.len(), 19);
        assert_eq!(words[14], vec![block + 0x38, 0]);
        assert_eq!(words[15], vec![block + 8, 0]);
        assert_eq!(shorts.len(), 7);
        assert_eq!(shorts[3], vec![block + 6, 0]);
        assert_eq!(shorts[6], vec![block + 4, 0]);
    }

    #[test]
    fn allocations_scale_by_the_element_size() {
        let (mut e, _) = engine(false);
        e.register(ALLOCATE, |_, a| ret(a[0]));
        e.register(REALLOCATE, |_, a| ret(a[0] + a[1]));
        assert_eq!(e.call(0x005e_f150, &[0, 3]).u32(), 0xa8);
        assert_eq!(e.call(0x005e_f180, &[0, 0x1000, 2]).u32(), 0x1070);
        assert_eq!(e.call(0x005e_f360, &[0, 0x1000, 2]).u32(), 0x1050);
        assert_eq!(e.call(0x005e_f540, &[0, 0x1000, 2]).u32(), 0x100c);
    }

    #[test]
    fn scalar_deleting_destructors() {
        let (mut e, _) = engine(false);
        e.register(FOOT_IK_ARRAY_DESTRUCTOR, |_, _| Ret::default());
        e.register(DEALLOCATE, |_, _| Ret::default());
        e.register(ARRAY_RELEASE_008454F0, |_, _| Ret::default());
        let array = e.mem.alloc(0x10);
        assert_eq!(e.call(0x005e_f5c0, &[array, 0]).u32(), array);
        assert_eq!(e.call(0x005e_f5f0, &[array, 1]).u32(), array);
        assert_eq!(e.mem.u32(array), LOOK_IK_ARRAY_VTABLE);
        assert_eq!(e.call(0x005e_f620, &[array, 3]).u32(), array);
        assert_eq!(e.mem.u32(array), GRAB_IK_ARRAY_VTABLE);
        assert_eq!(calls_to(&e, FOOT_IK_ARRAY_DESTRUCTOR), vec![vec![array]]);
        assert_eq!(calls_to(&e, DEALLOCATE), vec![vec![array]; 2]);
        assert_eq!(
            calls_to(&e, ARRAY_RELEASE_008454F0),
            vec![vec![array, 1]; 2]
        );
    }
}
