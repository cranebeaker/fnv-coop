//! `NiAnimation/niflipcontroller.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00a4f880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFlipController::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a4f570`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x54 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_flip_controller_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a4_f570;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x54u32]);
    }
    this
}

/// `NiFlipController` (Xbox PDB, 0x54 bytes; the PC code uses the same
/// offsets): from the base classes `m_uFlags` +0x08, `m_fLoKeyTime` +0x14,
/// `m_fHiKeyTime` +0x18, `m_fScaledTime` +0x28, `m_pkTarget` +0x2C and
/// `m_spInterpolator` +0x34; then `m_kTextures`
/// (`NiTObjectArray<NiPointer<NiTexture>>`, +0x38: vtable, `m_pBase` +0x3C,
/// `m_usMaxSize` +0x40, `m_usSize` +0x42, `m_usESize` +0x44, `m_usGrowBy`
/// +0x46), `m_uiIndex` +0x48, `m_uiAffectedMap` +0x4C and `m_pcCtlrID`
/// +0x50.
const FLAGS: u32 = 0x08;
const LO_KEY_TIME: u32 = 0x14;
const HI_KEY_TIME: u32 = 0x18;
const SCALED_TIME: u32 = 0x28;
const TARGET: u32 = 0x2c;
const INTERPOLATOR: u32 = 0x34;
const TEXTURES: u32 = 0x38;
const TEXTURES_BASE: u32 = 0x3c;
const TEXTURES_MAX_SIZE: u32 = 0x40;
const TEXTURES_SIZE: u32 = 0x42;
const TEXTURES_ESIZE: u32 = 0x44;
const TEXTURES_GROW_BY: u32 = 0x46;
const INDEX: u32 = 0x48;
const AFFECTED_MAP: u32 = 0x4c;
const CONTROLLER_ID: u32 = 0x50;
/// The class's size and vtable (RTTI `.?AVNiFlipController@@`), and the
/// texture array's vtables: `NiTArray<NiPointer<NiTexture>>` (`0103d3e8`,
/// stored first by the constructor and again by the destructor) and
/// `NiTObjectArray<NiPointer<NiTexture>>` (`0103d3f0`).
const NI_FLIP_CONTROLLER_SIZE: u32 = 0x54;
const NI_FLIP_CONTROLLER_VTABLE: u32 = 0x0109_9384;
const TEXTURE_ARRAY_VTABLE: u32 = 0x0103_d3e8;
const TEXTURE_OBJECT_ARRAY_VTABLE: u32 = 0x0103_d3f0;
/// `NiFlipController::ms_RTTI` and `NiTexturingProperty::ms_RTTI`.
const NI_FLIP_CONTROLLER_RTTI: u32 = 0x011f_3fd4;
const NI_TEXTURING_PROPERTY_RTTI: u32 = 0x011f_49a4;
/// `4294967296.0f` (`01099378`): added after an `FILD` of a negative word,
/// the unsigned conversion.
const TWO_POW_32: u32 = 0x0109_9378;
/// The `double` `01016408` (`0.01`, as a float widened) added before the
/// texture index is truncated.
const INDEX_ROUNDING: u32 = 0x0101_6408;
/// The float the scaled time is set to when flag 0x20 is set (`01096324`).
const SCALED_TIME_RESET: u32 = 0x0109_6324;
/// `"%d"`.
const FORMAT_DECIMAL: u32 = 0x0102_0764;
/// `00aa13e0` (`cdecl`, one word): allocates from the Ni allocator.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `00aa1070` (`cdecl`: size) and `00aa10f0` (`cdecl`: block) allocate and
/// free through the allocator at `011f6080`.
const NI_MALLOC: u32 = 0x00aa_1070;
const NI_FREE: u32 = 0x00aa_10f0;
/// `NiSprintf` (`00a305b0`, `cdecl`: buffer, size, format, ...).
const NI_SPRINTF: u32 = 0x00a3_05b0;
/// `005e0ba0` (`cdecl`: count): a pointer array of that many elements (the
/// engine map's `NiTNewInterface<NiPointer<NiAVObject>>::Allocate`).
const POINTER_ARRAY_ALLOCATE: u32 = 0x005e_0ba0;
/// `004b0220` (`cdecl`: the elements): deletes a pointer array when
/// non-null (the engine map's name for it is a folded body's).
const POINTER_ARRAY_DELETE: u32 = 0x004b_0220;
/// The texture array's `SetSize` (`0060bef0`; `thiscall` on the array: new
/// size) and `SetAt` (`0060c120`; `thiscall` on the array: index, address
/// of an `NiPointer`).
const TEXTURE_ARRAY_SET_SIZE: u32 = 0x0060_bef0;
const TEXTURE_ARRAY_SET_AT: u32 = 0x0060_c120;
/// Base-class functions this class calls (`thiscall`): the constructor
/// (`00a57640`, no stack word), the member copy (`00a57620`: destination,
/// cloning process), `IsEqual` (`00a57c10`), `RegisterStreamables`
/// (`00a57780`), `SaveBinary` (`00a57c00`), `LinkObject` (`00a57630`) and
/// `NiFloatInterpController::LoadBinary` (`00a57c70`); the engine map
/// gives some of these addresses names of folded bodies.
const BASE_CONSTRUCT: u32 = 0x00a5_7640;
const BASE_COPY_MEMBERS: u32 = 0x00a5_7620;
const BASE_IS_EQUAL: u32 = 0x00a5_7c10;
const BASE_REGISTER_STREAMABLES: u32 = 0x00a5_7780;
const BASE_SAVE_BINARY: u32 = 0x00a5_7c00;
const BASE_LINK_OBJECT: u32 = 0x00a5_7630;
const FLOAT_INTERP_CONTROLLER_LOAD_BINARY: u32 = 0x00a5_7c70;
/// The base destructor the destructor ends in (`00a6d290`, a jump).
const BASE_DESTRUCT: u32 = 0x00a6_d290;
/// `NiBlendFloatInterpolator::NiBlendFloatInterpolator` (`00a40520`;
/// `thiscall`: flag, threshold, count).
const BLEND_FLOAT_INTERPOLATOR_CONSTRUCT: u32 = 0x00a4_0520;
/// `NiTimeController::DontDoUpdate` (`00a36250`; `thiscall`: time).
const DONT_DO_UPDATE: u32 = 0x00a3_6250;
/// `004dc540` (`thiscall` on a texturing-property map, `RET 4`: the
/// texture): the map's texture assignment (forwards to `0066b0d0`).
const MAP_SET_TEXTURE: u32 = 0x004d_c540;
/// `00a463e0` (`thiscall` on an `NiTexturingProperty`, `RET 4`: index): the
/// shader map at that index of the array at +0x2C, or 0.
const TEXTURING_PROPERTY_SHADER_MAP: u32 = 0x00a4_63e0;
/// `NiStream::GetObjectFromLinkID` (`00a63430`),
/// `NiStream::GetNumberOfLinkIDs` (`00a63460`) and
/// `NiStream::ReadMultipleLinkIDs` (`00a64670`), `thiscall` on the stream
/// with no stack word.
const STREAM_GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
const STREAM_GET_NUMBER_OF_LINK_IDS: u32 = 0x00a6_3460;
const STREAM_READ_MULTIPLE_LINK_IDS: u32 = 0x00a6_4670;
/// `00aa3e40` (`thiscall` on the memory manager `011f6238`: size).
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// `NiObject::NiObject` (`00a5d3a0`), the `NiFloatData` vtable and
/// `NiFloatData::ReplaceAnim` (`00a3cdc0`).
const NI_OBJECT_CONSTRUCT: u32 = 0x00a5_d3a0;
const NI_FLOAT_DATA_VTABLE: u32 = 0x0102_dabc;
const FLOAT_DATA_REPLACE_ANIM: u32 = 0x00a3_cdc0;
/// `00a3c340` (`thiscall`: an `NiFloatData*`): the `NiFloatInterpolator`
/// constructor taking float data (by the vtable it stores).
const FLOAT_INTERPOLATOR_CONSTRUCT_WITH_DATA: u32 = 0x00a3_c340;
/// `NiStream::m_uiNifFileVersion` (Xbox PDB) +0xD8; files before 10.1.0.104
/// (`0x0a010068`) hold the flip timing instead of an interpolator link.
const STREAM_FILE_VERSION: u32 = 0xd8;
const VERSION_10_1_0_104: u32 = 0x0a01_0068;
/// `NiStream::m_pkIstr` +0x24c and `m_pkOstr` +0x250 (Xbox PDB), and the
/// binary stream's position (+4), read (+8) and write (+0xc) functions.
const STREAM_INPUT: u32 = 0x24c;
const STREAM_OUTPUT: u32 = 0x250;
const BINARY_STREAM_POSITION: u32 = 0x4;
const BINARY_STREAM_READ: u32 = 0x8;
const BINARY_STREAM_WRITE: u32 = 0xc;
/// `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// Virtual slots (Xbox PDB names): `NiRefObject::DeleteThis` +0x04,
/// `NiObject::GetRTTI` +0x08, `NiObject::RegisterStreamables` +0x54,
/// `NiStream::SaveLinkID` +0x30, the interpolator's `AlwaysUpdate` +0xD4,
/// its float `Update` +0x9C (`(time, target, &value)`) and `Collapse` +0xBC.
const SLOT_DELETE_THIS: u32 = 0x04;
const SLOT_GET_RTTI: u32 = 0x08;
const SLOT_REGISTER_STREAMABLES: u32 = 0x54;
const SLOT_SAVE_LINK_ID: u32 = 0x30;
const SLOT_ALWAYS_UPDATE: u32 = 0xd4;
const SLOT_UPDATE_FLOAT: u32 = 0x9c;
const SLOT_COLLAPSE: u32 = 0xbc;

/// The inline `NiPointer` release: `InterlockedDecrement` of the count at
/// `+4` and, when it reaches zero, the object's `DeleteThis`.
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32()
        == 0
    {
        e.vcall(object, SLOT_DELETE_THIS, &args![]);
    }
}

/// One 4-byte transfer of `buffer` through the binary stream at `slot` of
/// the `NiStream` and its function at `function`, the component size word
/// at `component_size` set to 4 first; advances the position by the bytes
/// moved.
fn transfer_word(
    e: &mut Engine,
    stream: u32,
    slot: u32,
    function: u32,
    buffer: u32,
    component_size: u32,
) {
    let binary = e.mem.u32(stream.wrapping_add(slot));
    let transfer = e.mem.u32(binary.wrapping_add(function));
    e.mem.set_u32(component_size, 4);
    let bytes = e
        .call(transfer, &args![binary, buffer, 4u32, component_size, 1u32])
        .u32();
    let position = e.mem.u32(binary.wrapping_add(BINARY_STREAM_POSITION));
    e.mem.set_u32(
        binary.wrapping_add(BINARY_STREAM_POSITION),
        position.wrapping_add(bytes),
    );
}

/// `FILD dword` of `word` and, when it is negative, `FADD` of the float
/// 2^32 (the compiler's unsigned-to-float conversion), stored as a float.
fn unsigned_to_float(e: &mut Engine, word: u32) -> u32 {
    let signed = word as i32;
    let mut value = f64::from(signed);
    if signed < 0 {
        value = x87_add(value, x87_m32(e.mem.u32(TWO_POW_32)));
    }
    x87_store32(value)
}

/// The texture at `index` of `m_kTextures` (no bounds check).
fn texture_at(e: &mut Engine, this: u32, index: u32) -> u32 {
    let base = e.mem.u32(this.wrapping_add(TEXTURES_BASE));
    e.mem.u32(base.wrapping_add(index.wrapping_mul(4)))
}

// Translated from 00a4f270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFlipController::GetTargetFloatValue` (Xbox PDB): `m_uiIndex` as an
/// unsigned float into `out`.
pub fn ni_flip_controller_get_target_float_value(e: &mut Engine, this: Ptr, out: Ptr) {
    let index = e.mem.u32(this.addr().wrapping_add(INDEX));
    let value = unsigned_to_float(e, index);
    e.mem.set_u32(out.addr(), value);
}

// Translated from 00a4f290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFlipController::IsEqual` (Xbox PDB): the base `IsEqual` (`00a57c10`)
/// and the same affected map.
pub fn ni_flip_controller_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e.call(BASE_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    e.mem.u32(this.addr().wrapping_add(AFFECTED_MAP))
        == e.mem.u32(other.addr().wrapping_add(AFFECTED_MAP))
}

// Translated from 00a4f2c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFlipController::CreateBlendInterpolator` (Xbox PDB; `thiscall`, five
/// words, `RET 0x14`; the first and third are not read): a 0x34-byte
/// `NiBlendFloatInterpolator` built from `(flag, threshold, count)` (the
/// float copied through the x87 stack), with bit 1 of its byte +0xC set.
/// When the allocation fails the bit is still set, at address 0xC, and 0
/// is returned.
pub fn ni_flip_controller_create_blend_interpolator(
    e: &mut Engine,
    _this: Ptr,
    _unused_1: u32,
    flag: u32,
    _unused_3: u32,
    threshold: u32,
    count: u32,
) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![0x34u32]).u32();
    let interpolator = if block == 0 {
        0
    } else {
        e.call(
            BLEND_FLOAT_INTERPOLATOR_CONSTRUCT,
            &args![block, flag, x87_quiet_bits(threshold), count],
        )
        .u32()
    };
    let flags = e.mem.u8(interpolator.wrapping_add(0xc));
    e.mem.set_u8(interpolator.wrapping_add(0xc), flags | 2);
    Ptr::new(interpolator)
}

// Translated from 00a4f300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: index): the texture at
/// `index` of `m_kTextures`, 0 when `index` is not below the array's 16-bit
/// size (the Xbox name is not confirmed).
pub fn fn_00a4f300(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let this = this.addr();
    let size = u32::from(e.mem.u16(this.wrapping_add(TEXTURES_SIZE)));
    if index >= size {
        return 0;
    }
    texture_at(e, this, index)
}

// Translated from 00a4f320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFlipController::GetCtlrID` (Xbox PDB): `m_pcCtlrID`, made on the first
/// call: a 15-byte buffer (`00aa1070`), stored, then filled with the
/// affected map in decimal (`NiSprintf(buffer, 15, "%d", map)`).
pub fn ni_flip_controller_get_ctlr_id(e: &mut Engine, this: Ptr) -> u32 {
    let this = this.addr();
    let id = e.mem.u32(this.wrapping_add(CONTROLLER_ID));
    if id != 0 {
        return id;
    }
    let buffer = e.call(NI_MALLOC, &args![0xfu32]).u32();
    let map = e.mem.u32(this.wrapping_add(AFFECTED_MAP));
    e.mem.set_u32(this.wrapping_add(CONTROLLER_ID), buffer);
    e.call(NI_SPRINTF, &args![buffer, 0xfu32, FORMAT_DECIMAL, map]);
    e.mem.u32(this.wrapping_add(CONTROLLER_ID))
}

// Translated from 00a4f350 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFlipController::RegisterStreamables` (Xbox PDB): false when the base
/// `RegisterStreamables` (`00a57780`) answers false; otherwise every
/// non-null texture's `RegisterStreamables` (slot +0x54; the count read
/// once) and true.
pub fn ni_flip_controller_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    if !e
        .call(BASE_REGISTER_STREAMABLES, &args![this, stream])
        .bool()
    {
        return false;
    }
    let this = this.addr();
    let count = u32::from(e.mem.u16(this.wrapping_add(TEXTURES_SIZE)));
    for index in 0..count {
        let texture = texture_at(e, this, index);
        if texture != 0 {
            e.vcall(texture, SLOT_REGISTER_STREAMABLES, &args![stream]);
        }
    }
    true
}

// Translated from 00a4f3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFlipController::Update` (Xbox PDB; `RET 4`: the update data, whose
/// first word is the time). With flag 0x20 of the flags byte the scaled
/// time becomes the float at `01096324`; otherwise
/// `NiTimeController::DontDoUpdate` decides, and when it says skip the
/// update still runs if the interpolator's `AlwaysUpdate` (slot +0xD4)
/// answers true. With an interpolator its float `Update` (slot +0x9C:
/// scaled time, target, address of a stack float) runs; on success the
/// float plus the double 0.01 (`01016408`), truncated (`FISTP qword` under
/// the chop mode, low word), becomes `m_uiIndex`, limited to the texture
/// count minus one (unsigned). Then, with a target (an
/// `NiTexturingProperty`): when the affected map is below the property's
/// map count (16 bits at +0x26) and that map (+0x20 array) is set, the
/// texture at `m_uiIndex` goes to it (`004dc540`); otherwise the affected
/// map minus 0x400 is a shader map index, and when the property has a
/// shader map array (+0x2C) whose 16-bit size (+0xA) is above it and the
/// shader map (`00a463e0`) exists, the texture goes to that map.
pub fn ni_flip_controller_update(e: &mut Engine, this: Ptr, update: Ptr) {
    let object = this.addr();
    if (e.mem.u8(object.wrapping_add(FLAGS)) >> 5) & 1 != 0 {
        let reset = x87_quiet_bits(e.global::<u32>(SCALED_TIME_RESET));
        e.mem.set_u32(object.wrapping_add(SCALED_TIME), reset);
    } else {
        let time = x87_quiet_bits(e.mem.u32(update.addr()));
        if e.call(DONT_DO_UPDATE, &args![this, time]).bool() {
            let interpolator = e.mem.u32(object.wrapping_add(INTERPOLATOR));
            if interpolator == 0 {
                return;
            }
            if !e.vcall(interpolator, SLOT_ALWAYS_UPDATE, &args![]).bool() {
                return;
            }
        }
    }
    let interpolator = e.mem.u32(object.wrapping_add(INTERPOLATOR));
    if interpolator == 0 {
        return;
    }
    let value = e.with_stack(4, |e, value| {
        let scaled_time = x87_quiet_bits(e.mem.u32(object.wrapping_add(SCALED_TIME)));
        let target = e.mem.u32(object.wrapping_add(TARGET));
        let updated = e
            .vcall(
                interpolator,
                SLOT_UPDATE_FLOAT,
                &args![scaled_time, target, value],
            )
            .bool();
        updated.then(|| e.mem.u32(value.addr()))
    });
    let Some(value) = value else {
        return;
    };
    let count = u32::from(e.mem.u16(object.wrapping_add(TEXTURES_SIZE)));
    let rounding = f64::from_bits(e.mem.u64(INDEX_ROUNDING));
    let sum = x87_add(x87_load32(value), rounding);
    let index = x87_fistp64_chop(X87Ext::from(sum)) as u32;
    e.mem.set_u32(object.wrapping_add(INDEX), index);
    if index >= count {
        e.mem
            .set_u32(object.wrapping_add(INDEX), count.wrapping_sub(1));
    }
    let target = e.mem.u32(object.wrapping_add(TARGET));
    if target == 0 {
        return;
    }
    // NiTexturingProperty: the maps' array at +0x20 and its 16-bit count at
    // +0x26 (by this code), the shader maps' array pointer at +0x2C.
    let map_count = u32::from(e.mem.u16(target.wrapping_add(0x26)));
    let affected = e.mem.u32(object.wrapping_add(AFFECTED_MAP));
    if affected < map_count {
        let maps = e.mem.u32(target.wrapping_add(0x20));
        let map = e.mem.u32(maps.wrapping_add(affected.wrapping_mul(4)));
        if map != 0 {
            let index = e.mem.u32(object.wrapping_add(INDEX));
            let texture = texture_at(e, object, index);
            e.call(MAP_SET_TEXTURE, &args![map, texture]);
            return;
        }
    }
    let shader_index = affected.wrapping_add(0xffff_fc00);
    let shader_maps = e.mem.u32(target.wrapping_add(0x2c));
    if shader_maps == 0 {
        return;
    }
    if shader_index >= u32::from(e.mem.u16(shader_maps.wrapping_add(0xa))) {
        return;
    }
    if e.call(TEXTURING_PROPERTY_SHADER_MAP, &args![target, shader_index])
        .u32()
        == 0
    {
        return;
    }
    let index = e.mem.u32(object.wrapping_add(INDEX));
    let texture = texture_at(e, object, index);
    let map = e
        .call(TEXTURING_PROPERTY_SHADER_MAP, &args![target, shader_index])
        .u32();
    e.call(MAP_SET_TEXTURE, &args![map, texture]);
}

// Translated from 00a4f4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFlipController::SaveBinary` (Xbox PDB): the base `SaveBinary`
/// (`00a57c00`), the affected map and the texture count (4 bytes each; the
/// count goes through the argument's own stack word, the component size
/// through a local), then each texture's link ID (`NiStream::SaveLinkID`,
/// slot +0x30).
pub fn ni_flip_controller_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(BASE_SAVE_BINARY, &args![this, stream]);
    let object = this.addr();
    let stream = stream.addr();
    // The frame: the component size word +0, the argument's own word +4.
    e.with_stack(8, |e, frame| {
        let component_size = frame.addr();
        let count_word = component_size + 4;
        transfer_word(
            e,
            stream,
            STREAM_OUTPUT,
            BINARY_STREAM_WRITE,
            object.wrapping_add(AFFECTED_MAP),
            component_size,
        );
        let count = u32::from(e.mem.u16(object.wrapping_add(TEXTURES_SIZE)));
        e.mem.set_u32(count_word, count);
        transfer_word(
            e,
            stream,
            STREAM_OUTPUT,
            BINARY_STREAM_WRITE,
            count_word,
            component_size,
        );
        let mut index = 0u32;
        while index < e.mem.u32(count_word) {
            let texture = texture_at(e, object, index);
            e.vcall(stream, SLOT_SAVE_LINK_ID, &args![texture]);
            index = index.wrapping_add(1);
        }
    });
}

// Translated from 00a4f570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFlipController::~NiFlipController` (Xbox PDB): the vtable, the
/// controller ID freed (`00aa10f0`) and cleared, the texture array's vtable
/// and its elements deleted (`004b0220`), the interpolator released, then
/// the base destructor (`00a6d290`, a jump).
pub fn ni_flip_controller_destructor(e: &mut Engine, this: Ptr) {
    let object = this.addr();
    let id = e.mem.u32(object.wrapping_add(CONTROLLER_ID));
    e.mem.set_u32(object, NI_FLIP_CONTROLLER_VTABLE);
    e.call(NI_FREE, &args![id]);
    e.mem.set_u32(object.wrapping_add(CONTROLLER_ID), 0);
    let base = e.mem.u32(object.wrapping_add(TEXTURES_BASE));
    e.mem
        .set_u32(object.wrapping_add(TEXTURES), TEXTURE_ARRAY_VTABLE);
    e.call(POINTER_ARRAY_DELETE, &args![base]);
    let interpolator = e.mem.u32(object.wrapping_add(INTERPOLATOR));
    if interpolator != 0 {
        release(e, interpolator);
    }
    e.call(BASE_DESTRUCT, &args![this]);
}

// Translated from 00a4f5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFlipController::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_flip_controller_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_FLIP_CONTROLLER_RTTI)
}

// Translated from 00a4f5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFlipController::InterpTargetIsCorrectType` (Xbox PDB; `this` is not
/// read): whether `target` is an `NiTexturingProperty`: its `GetRTTI`
/// (slot +0x08) and the base records (+4) walked until `011f49a4`; false
/// for a null target or the end of the chain.
pub fn ni_flip_controller_interp_target_is_correct_type(
    e: &mut Engine,
    _this: Ptr,
    target: Ptr,
) -> bool {
    if target.addr() == 0 {
        return false;
    }
    let mut rtti = e.vcall(target.addr(), SLOT_GET_RTTI, &args![]).u32();
    while rtti != 0 {
        if rtti == NI_TEXTURING_PROPERTY_RTTI {
            return true;
        }
        rtti = e.mem.u32(rtti.wrapping_add(4));
    }
    false
}

// Translated from 00a4f610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFlipController::LoadBinary` (Xbox PDB): `NiFloatInterpController::
/// LoadBinary`, then the affected map (4 bytes, the component size in the
/// argument's own stack word). Files from 10.1.0.104 on only read the
/// texture links (`NiStream::ReadMultipleLinkIDs`). Older files hold the
/// start time and the time per texture (4 bytes each) before the links;
/// from those the controller makes its interpolator: an `NiFloatData`
/// (built inline in 0x18 bytes, or 0 when the allocation fails) with, for
/// `n` textures, `n + 1` step keys (type 5, 8 bytes each in an array whose
/// count word precedes it, allocated through the memory manager with the
/// size saturating on overflow): key `i` at the start time plus `i` times
/// the step (each sum stored as a float) holding the float `i`, the last
/// key at the final time holding `n - 1`; `ReplaceAnim` takes them, the
/// start and final times become `m_fLoKeyTime` and `m_fHiKeyTime`. Then an
/// `NiFloatInterpolator` on that data (`00a3c340`, 0 when its allocation
/// fails) becomes the interpolator (counted, the old one released) and its
/// `Collapse` (slot +0xBC) runs. With no textures only the interpolator
/// part runs, on null data.
pub fn ni_flip_controller_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(FLOAT_INTERP_CONTROLLER_LOAD_BINARY, &args![this, stream]);
    let object = this.addr();
    let stream_addr = stream.addr();
    // The frame: the argument's own word +0 (component sizes, then the
    // running time), the start time +4, the step +8.
    e.with_stack(12, |e, frame| {
        let argument_word = frame.addr();
        let start_word = argument_word + 4;
        let step_word = argument_word + 8;
        transfer_word(
            e,
            stream_addr,
            STREAM_INPUT,
            BINARY_STREAM_READ,
            object.wrapping_add(AFFECTED_MAP),
            argument_word,
        );
        if e.mem.u32(stream_addr.wrapping_add(STREAM_FILE_VERSION)) >= VERSION_10_1_0_104 {
            e.call(STREAM_READ_MULTIPLE_LINK_IDS, &args![stream]);
            return;
        }
        transfer_word(
            e,
            stream_addr,
            STREAM_INPUT,
            BINARY_STREAM_READ,
            start_word,
            argument_word,
        );
        transfer_word(
            e,
            stream_addr,
            STREAM_INPUT,
            BINARY_STREAM_READ,
            step_word,
            argument_word,
        );
        let count = e.call(STREAM_READ_MULTIPLE_LINK_IDS, &args![stream]).u32();
        let data = e.call(NI_ALLOCATE, &args![0x18u32]).u32();
        let data = if data == 0 {
            0
        } else {
            e.call(NI_OBJECT_CONSTRUCT, &args![data]);
            e.mem.set_u32(data, NI_FLOAT_DATA_VTABLE);
            e.mem.set_u32(data.wrapping_add(8), 0);
            e.mem.set_u32(data.wrapping_add(0xc), 0);
            e.mem.set_u32(data.wrapping_add(0x10), 0);
            e.mem.set_u8(data.wrapping_add(0x14), 0);
            data
        };
        if count != 0 {
            let key_count = count.wrapping_add(1);
            // `MUL` by 8 with `SETO`/`NEG`/`OR`, then `+ 4` with `SETC`/`NEG`/`OR`.
            let bytes = key_count.checked_mul(8).unwrap_or(u32::MAX);
            let bytes = bytes.checked_add(4).unwrap_or(u32::MAX);
            let block = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, bytes]).u32();
            let keys = if block == 0 {
                0
            } else {
                e.mem.set_u32(block, key_count);
                block.wrapping_add(4)
            };
            let start = x87_quiet_bits(e.mem.u32(start_word));
            e.mem.set_u32(argument_word, start);
            let mut index = 0u32;
            while index < count {
                let time = x87_load32(e.mem.u32(argument_word));
                let key = keys.wrapping_add(index.wrapping_mul(8));
                e.mem.set_u32(key, x87_store32(time));
                e.mem.set_u32(argument_word, index);
                let value = unsigned_to_float(e, index);
                e.mem.set_u32(key.wrapping_add(4), value);
                index = index.wrapping_add(1);
                let next = x87_add(time, x87_m32(e.mem.u32(step_word)));
                e.mem.set_u32(argument_word, x87_store32(next));
            }
            let end = x87_quiet_bits(e.mem.u32(argument_word));
            let last = keys.wrapping_add(count.wrapping_mul(8));
            e.mem.set_u32(last, end);
            let value = unsigned_to_float(e, count.wrapping_sub(1));
            e.mem.set_u32(last.wrapping_add(4), value);
            e.call(FLOAT_DATA_REPLACE_ANIM, &args![data, keys, key_count, 5u32]);
            let start = x87_quiet_bits(e.mem.u32(start_word));
            e.mem.set_u32(object.wrapping_add(LO_KEY_TIME), start);
            let end = x87_quiet_bits(e.mem.u32(argument_word));
            e.mem.set_u32(object.wrapping_add(HI_KEY_TIME), end);
        }
        let block = e.call(NI_ALLOCATE, &args![0x18u32]).u32();
        let interpolator = if block == 0 {
            0
        } else {
            e.call(FLOAT_INTERPOLATOR_CONSTRUCT_WITH_DATA, &args![block, data])
                .u32()
        };
        let old = e.mem.u32(object.wrapping_add(INTERPOLATOR));
        if old != interpolator {
            if old != 0 {
                release(e, old);
            }
            e.mem
                .set_u32(object.wrapping_add(INTERPOLATOR), interpolator);
            if interpolator != 0 {
                e.call(INTERLOCKED_INCREMENT, &args![interpolator.wrapping_add(4)]);
            }
        }
        let interpolator = e.mem.u32(object.wrapping_add(INTERPOLATOR));
        e.vcall(interpolator, SLOT_COLLAPSE, &args![]);
    });
}

// Translated from 00a4f820 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiFlipController` constructor (unnamed in the engine map; by the
/// vtable it stores; `thiscall`, no stack word): the base constructor
/// (`00a57640`), the vtable, an empty texture array (capacity 1, grow-by 1,
/// a one-element pointer array from `005e0ba0`; its vtable first
/// `0103d3e8`, then `0103d3f0`), index, affected map and controller ID 0.
/// Returns `this`.
pub fn fn_00a4f820(e: &mut Engine, this: Ptr) -> Ptr {
    let object = this.addr();
    e.call(BASE_CONSTRUCT, &args![this]);
    e.mem.set_u32(object, NI_FLIP_CONTROLLER_VTABLE);
    e.mem.set_u16(object.wrapping_add(TEXTURES_MAX_SIZE), 1);
    e.mem
        .set_u32(object.wrapping_add(TEXTURES), TEXTURE_ARRAY_VTABLE);
    e.mem.set_u16(object.wrapping_add(TEXTURES_GROW_BY), 1);
    e.mem.set_u16(object.wrapping_add(TEXTURES_SIZE), 0);
    e.mem.set_u16(object.wrapping_add(TEXTURES_ESIZE), 0);
    let base = e.call(POINTER_ARRAY_ALLOCATE, &args![1u32]).u32();
    e.mem.set_u32(object.wrapping_add(TEXTURES_BASE), base);
    e.mem
        .set_u32(object.wrapping_add(TEXTURES), TEXTURE_OBJECT_ARRAY_VTABLE);
    e.mem.set_u32(object.wrapping_add(INDEX), 0);
    e.mem.set_u32(object.wrapping_add(AFFECTED_MAP), 0);
    e.mem.set_u32(object.wrapping_add(CONTROLLER_ID), 0);
    this
}

// Translated from 00a4f8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`: texture, index): stores
/// `texture` at `index` of `m_kTextures` (the Xbox name is not confirmed).
/// When `index` is not below the size the array's size becomes `index + 1`
/// (`SetSize`); the texture is held counted in the second argument's own
/// word; when `index` is not below the capacity the size grows by the
/// grow-by step first, then `SetAt(index, &held)`, and the hold is
/// released.
pub fn fn_00a4f8a0(e: &mut Engine, this: Ptr, texture: Ptr, index: u32) {
    let object = this.addr();
    let array = object.wrapping_add(TEXTURES);
    let size = u32::from(e.mem.u16(object.wrapping_add(TEXTURES_SIZE)));
    if size <= index {
        e.call(TEXTURE_ARRAY_SET_SIZE, &args![array, index.wrapping_add(1)]);
    }
    let texture = texture.addr();
    e.with_stack(4, |e, held| {
        e.mem.set_u32(held.addr(), texture);
        if texture != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![texture.wrapping_add(4)]);
        }
        let capacity = u32::from(e.mem.u16(object.wrapping_add(TEXTURES_MAX_SIZE)));
        if index >= capacity {
            let grow_by = u32::from(e.mem.u16(object.wrapping_add(TEXTURES_GROW_BY)));
            e.call(
                TEXTURE_ARRAY_SET_SIZE,
                &args![array, grow_by.wrapping_add(index)],
            );
        }
        e.call(TEXTURE_ARRAY_SET_AT, &args![array, index, held]);
        if texture != 0 {
            release(e, texture);
        }
    });
}

// Translated from 00a4f920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`: destination, cloning
/// process): `NiFlipController`'s member copy. The base copy (`00a57620`),
/// the affected map, then every texture stored into the destination at the
/// same index ([`fn_00a4f8a0`]; the size reread after each).
pub fn fn_00a4f920(e: &mut Engine, this: Ptr, destination: Ptr, cloning: u32) {
    e.call(BASE_COPY_MEMBERS, &args![this, destination, cloning]);
    let object = this.addr();
    let map = e.mem.u32(object.wrapping_add(AFFECTED_MAP));
    e.mem
        .set_u32(destination.addr().wrapping_add(AFFECTED_MAP), map);
    let size = u32::from(e.mem.u16(object.wrapping_add(TEXTURES_SIZE)));
    if size == 0 {
        return;
    }
    let mut index = 0u32;
    // The first texture's bounds check (the loop's later ones are skipped).
    let mut texture = if index < size {
        texture_at(e, object, index)
    } else {
        0
    };
    loop {
        fn_00a4f8a0(e, destination, Ptr::new(texture), index);
        let size = u32::from(e.mem.u16(object.wrapping_add(TEXTURES_SIZE)));
        index = index.wrapping_add(1);
        if index >= size {
            break;
        }
        texture = texture_at(e, object, index);
    }
}

// Translated from 00a4f970 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `NiFlipController::CreateObject` (Ghidra has no function here; the
/// loader's factory by its body): allocates 0x54 bytes and, when that
/// succeeds, runs the constructor ([`fn_00a4f820`], a tail jump). Returns
/// the object, or 0.
pub fn ni_flip_controller_create_object(e: &mut Engine) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![NI_FLIP_CONTROLLER_SIZE]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    fn_00a4f820(e, Ptr::new(block))
}

// Translated from 00a4f990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFlipController::LinkObject` (Xbox PDB): the base `LinkObject`
/// (`00a57630`), then, with a target, the texture array sized to the
/// stream's link count (`SetSize`) and each texture taken from the next
/// link (`NiStream::GetObjectFromLinkID`), held counted in a local and
/// stored with `SetAt(index, &held)`, then the hold released.
pub fn ni_flip_controller_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(BASE_LINK_OBJECT, &args![this, stream]);
    let object = this.addr();
    if e.mem.u32(object.wrapping_add(TARGET)) == 0 {
        return;
    }
    let count = e.call(STREAM_GET_NUMBER_OF_LINK_IDS, &args![stream]).u32();
    let array = object.wrapping_add(TEXTURES);
    e.call(TEXTURE_ARRAY_SET_SIZE, &args![array, count]);
    e.with_stack(4, |e, held| {
        for index in 0..count {
            let texture = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
            e.mem.set_u32(held.addr(), texture);
            if texture != 0 {
                e.call(INTERLOCKED_INCREMENT, &args![texture.wrapping_add(4)]);
            }
            e.call(TEXTURE_ARRAY_SET_AT, &args![array, index, held]);
            if texture != 0 {
                release(e, texture);
            }
        }
    });
}

// Translated from 00a4fa20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFlipController::CreateClone` (Xbox PDB; `RET 4`: cloning process): a
/// new 0x54-byte controller ([`fn_00a4f820`]), filled by the member copy
/// ([`fn_00a4f920`]; also called with a null clone when the allocation
/// fails). Returns the clone, or 0.
pub fn ni_flip_controller_create_clone(e: &mut Engine, this: Ptr, cloning: u32) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![NI_FLIP_CONTROLLER_SIZE]).u32();
    if block == 0 {
        fn_00a4f920(e, this, Ptr::new(0), cloning);
        return Ptr::new(0);
    }
    let clone = fn_00a4f820(e, Ptr::new(block));
    fn_00a4f920(e, this, clone, cloning);
    clone
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a4f880, ni_flip_controller_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x00a4_f270,
            ni_flip_controller_get_target_float_value(Ptr, Ptr)
        ),
        entry!(0x00a4_f290, ni_flip_controller_is_equal(Ptr, Ptr) -> bool),
        entry!(
            0x00a4_f2c0,
            ni_flip_controller_create_blend_interpolator(Ptr, u32, u32, u32, u32, u32) -> Ptr
        ),
        entry!(0x00a4_f300, fn_00a4f300(Ptr, u32) -> u32),
        entry!(0x00a4_f320, ni_flip_controller_get_ctlr_id(Ptr) -> u32),
        entry!(0x00a4_f350, ni_flip_controller_register_streamables(Ptr, Ptr) -> bool),
        entry!(0x00a4_f3a0, ni_flip_controller_update(Ptr, Ptr)),
        entry!(0x00a4_f4e0, ni_flip_controller_save_binary(Ptr, Ptr)),
        entry!(0x00a4_f570, ni_flip_controller_destructor(Ptr)),
        entry!(0x00a4_f5d0, ni_flip_controller_get_rtti() -> Ptr),
        entry!(
            0x00a4_f5e0,
            ni_flip_controller_interp_target_is_correct_type(Ptr, Ptr) -> bool
        ),
        entry!(0x00a4_f610, ni_flip_controller_load_binary(Ptr, Ptr)),
        entry!(0x00a4_f820, fn_00a4f820(Ptr) -> Ptr),
        entry!(0x00a4_f8a0, fn_00a4f8a0(Ptr, Ptr, u32)),
        entry!(0x00a4_f920, fn_00a4f920(Ptr, Ptr, u32)),
        entry!(0x00a4_f970, ni_flip_controller_create_object() -> Ptr),
        entry!(0x00a4_f990, ni_flip_controller_link_object(Ptr, Ptr)),
        entry!(0x00a4_fa20, ni_flip_controller_create_clone(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_flip_controller_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a4_f570, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00a4f880, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a4f880, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x54]]);
        assert_eq!(calls_to(&e, 0x00a4_f570), vec![vec![this]; 2]);
    }
}

#[cfg(test)]
mod n003_tests {
    use super::*;

    const DELETE_THIS: u32 = 0x0330_0000;
    const REGISTER: u32 = 0x0330_0010;
    const SAVE_LINK_ID: u32 = 0x0330_0020;
    const GET_RTTI: u32 = 0x0330_0030;
    const INTERPOLATOR_UPDATE: u32 = 0x0330_0040;
    const INTERPOLATOR_ALWAYS: u32 = 0x0330_0050;
    const COLLAPSE: u32 = 0x0330_0060;
    const TRANSFER: u32 = 0x0330_0070;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// An engine with counting `Interlocked` doubles, a call log, and doubles
    /// for the vtable slots the objects below use.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        for address in [DELETE_THIS, REGISTER, SAVE_LINK_ID, COLLAPSE] {
            e.register(address, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    /// An object with count `count` whose vtable holds the test doubles.
    fn object(e: &mut Engine, count: u32) -> u32 {
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + SLOT_DELETE_THIS, DELETE_THIS);
        e.mem.set_u32(vtable + SLOT_GET_RTTI, GET_RTTI);
        e.mem.set_u32(vtable + SLOT_REGISTER_STREAMABLES, REGISTER);
        e.mem.set_u32(vtable + SLOT_SAVE_LINK_ID, SAVE_LINK_ID);
        e.mem
            .set_u32(vtable + SLOT_UPDATE_FLOAT, INTERPOLATOR_UPDATE);
        e.mem
            .set_u32(vtable + SLOT_ALWAYS_UPDATE, INTERPOLATOR_ALWAYS);
        e.mem.set_u32(vtable + SLOT_COLLAPSE, COLLAPSE);
        let object = e.mem.alloc(0x60);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, count);
        object
    }

    /// A controller whose texture array holds `textures`.
    fn controller(e: &mut Engine, textures: &[u32]) -> u32 {
        let this = e.mem.alloc(NI_FLIP_CONTROLLER_SIZE);
        let base = e.mem.alloc(4 * textures.len() as u32 + 4);
        for (i, t) in textures.iter().enumerate() {
            e.mem.set_u32(base + 4 * i as u32, *t);
        }
        e.mem.set_u32(this + TEXTURES_BASE, base);
        e.mem.set_u16(this + TEXTURES_SIZE, textures.len() as u16);
        e.mem
            .set_u16(this + TEXTURES_MAX_SIZE, textures.len() as u16);
        this
    }

    #[test]
    fn get_target_float_value_is_the_unsigned_index() {
        let mut e = engine();
        e.map(TWO_POW_32, 4);
        e.mem.set_f32(TWO_POW_32, 4_294_967_296.0);
        let this = controller(&mut e, &[]);
        let out = e.mem.alloc(4);
        e.mem.set_u32(this + INDEX, 7);
        e.call(0x00a4_f270, &args![this, out]);
        assert_eq!(e.mem.f32(out), 7.0);
        e.mem.set_u32(this + INDEX, 0xffff_ffff);
        e.call(0x00a4_f270, &args![this, out]);
        assert_eq!(e.mem.f32(out), 4_294_967_296.0);
    }

    #[test]
    fn is_equal_needs_the_base_and_the_same_map() {
        let mut e = engine();
        e.register(BASE_IS_EQUAL, |e, a| e.mem.u32(a[0] + 4).into_ret());
        let this = controller(&mut e, &[]);
        let other = controller(&mut e, &[]);
        e.mem.set_u32(this + 4, 1);
        assert!(e.call(0x00a4_f290, &args![this, other]).bool());
        e.mem.set_u32(other + AFFECTED_MAP, 2);
        assert!(!e.call(0x00a4_f290, &args![this, other]).bool());
        e.mem.set_u32(other + AFFECTED_MAP, 0);
        e.mem.set_u32(this + 4, 0);
        assert!(!e.call(0x00a4_f290, &args![this, other]).bool());
    }

    #[test]
    fn create_blend_interpolator_marks_the_new_interpolator() {
        let mut e = engine();
        let block = e.mem.alloc(0x34);
        e.register_double(NI_ALLOCATE, move |_, _| block.into_ret());
        e.register(BLEND_FLOAT_INTERPOLATOR_CONSTRUCT, |_, a| a[0].into_ret());
        e.mem.set_u8(block + 0xc, 0x41);
        let answer = e
            .call(
                0x00a4_f2c0,
                &args![0x5000u32, 1u32, 1u32, 2u32, 0x7f80_0001u32, 2u32],
            )
            .u32();
        assert_eq!(answer, block);
        assert_eq!(calls(&e, NI_ALLOCATE), vec![vec![0x34]]);
        assert_eq!(
            calls(&e, BLEND_FLOAT_INTERPOLATOR_CONSTRUCT),
            vec![vec![block, 1, 0x7fc0_0001, 2]]
        );
        assert_eq!(e.mem.u8(block + 0xc), 0x43);
        // A failed allocation sets the bit at address 0xC.
        e.map(0, 0x10);
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        assert_eq!(
            e.call(0x00a4_f2c0, &args![0x5000u32, 0u32, 0u32, 0u32, 0u32, 0u32])
                .u32(),
            0
        );
        assert_eq!(e.mem.u8(0xc), 2);
    }

    #[test]
    fn texture_at_checks_the_size() {
        let mut e = engine();
        let this = controller(&mut e, &[0x6000, 0x6100]);
        assert_eq!(e.call(0x00a4_f300, &args![this, 1u32]).u32(), 0x6100);
        assert_eq!(e.call(0x00a4_f300, &args![this, 2u32]).u32(), 0);
    }

    #[test]
    fn get_ctlr_id_formats_the_map_once() {
        let mut e = engine();
        let buffer = e.mem.alloc(0x10);
        e.register_double(NI_MALLOC, move |_, _| buffer.into_ret());
        e.register(NI_SPRINTF, |_, _| Ret::default());
        let this = controller(&mut e, &[]);
        e.mem.set_u32(this + AFFECTED_MAP, 4);
        assert_eq!(e.call(0x00a4_f320, &args![this]).u32(), buffer);
        assert_eq!(calls(&e, NI_MALLOC), vec![vec![0xf]]);
        assert_eq!(
            calls(&e, NI_SPRINTF),
            vec![vec![buffer, 0xf, FORMAT_DECIMAL, 4]]
        );
        assert_eq!(e.call(0x00a4_f320, &args![this]).u32(), buffer);
        assert_eq!(calls(&e, NI_MALLOC).len(), 1);
    }

    #[test]
    fn register_streamables_registers_each_texture() {
        let mut e = engine();
        let mut answers = vec![0u32, 1];
        e.register_double(BASE_REGISTER_STREAMABLES, move |_, _| {
            answers.remove(0).into_ret()
        });
        let texture = object(&mut e, 1);
        let this = controller(&mut e, &[texture, 0, texture]);
        assert!(!e.call(0x00a4_f350, &args![this, 0x8000u32]).bool());
        assert!(calls(&e, REGISTER).is_empty());
        assert!(e.call(0x00a4_f350, &args![this, 0x8000u32]).bool());
        assert_eq!(calls(&e, REGISTER), vec![vec![texture, 0x8000]; 2]);
    }

    /// A controller over three textures with an interpolator whose float
    /// `Update` writes `value` and answers true, and the index rounding
    /// constant mapped.
    fn updating(e: &mut Engine, value: f32, always: bool) -> u32 {
        e.map(INDEX_ROUNDING, 8);
        e.mem.set_u64(INDEX_ROUNDING, 0x3f84_7ae1_4000_0000);
        let interpolator = object(e, 1);
        e.register_double(INTERPOLATOR_UPDATE, move |e, a| {
            e.mem.set_f32(a[3], value);
            1u32.into_ret()
        });
        e.register_double(INTERPOLATOR_ALWAYS, move |_, _| {
            u32::from(always).into_ret()
        });
        e.register(MAP_SET_TEXTURE, |_, _| Ret::default());
        let this = controller(e, &[0x6000, 0x6100, 0x6200]);
        e.mem.set_u32(this + INTERPOLATOR, interpolator);
        e.mem.set_f32(this + SCALED_TIME, 1.5);
        this
    }

    /// A texturing property with `maps` maps (the map at `with_map` set)
    /// and `shader_maps` shader maps.
    fn property(e: &mut Engine, maps: u16, with_map: u32, shader_maps: u16) -> u32 {
        let target = e.mem.alloc(0x40);
        let array = e.mem.alloc(0x20);
        e.mem.set_u32(array + 4 * with_map, 0x7700);
        e.mem.set_u32(target + 0x20, array);
        e.mem.set_u16(target + 0x26, maps);
        if shader_maps != 0 {
            let shaders = e.mem.alloc(0x10);
            e.mem.set_u16(shaders + 0xa, shader_maps);
            e.mem.set_u32(target + 0x2c, shaders);
        }
        target
    }

    #[test]
    fn update_picks_the_texture_for_the_affected_map() {
        let mut e = engine();
        e.register(DONT_DO_UPDATE, |_, _| Ret::default());
        let this = updating(&mut e, 0.995, false);
        let target = property(&mut e, 4, 2, 0);
        e.mem.set_u32(this + TARGET, target);
        e.mem.set_u32(this + AFFECTED_MAP, 2);
        let update = e.mem.alloc(4);
        e.mem.set_f32(update, 3.0);
        e.call(0x00a4_f3a0, &args![this, update]);
        assert_eq!(
            calls(&e, DONT_DO_UPDATE),
            vec![vec![this, 3.0f32.to_bits()]]
        );
        assert_eq!(
            calls(&e, INTERPOLATOR_UPDATE)[0][1..3],
            [1.5f32.to_bits(), target]
        );
        // 0.995 + 0.01 truncates to 1.
        assert_eq!(e.mem.u32(this + INDEX), 1);
        assert_eq!(calls(&e, MAP_SET_TEXTURE), vec![vec![0x7700, 0x6100]]);
    }

    #[test]
    fn update_limits_the_index_and_uses_shader_maps() {
        let mut e = engine();
        e.register(DONT_DO_UPDATE, |_, _| Ret::default());
        e.register(TEXTURING_PROPERTY_SHADER_MAP, |_, a| {
            (0x7800 + a[1]).into_ret()
        });
        let this = updating(&mut e, 7.5, false);
        // Map 0x401 is shader map 1; the plain maps (4) do not reach it.
        let target = property(&mut e, 4, 0, 2);
        e.mem.set_u32(this + TARGET, target);
        e.mem.set_u32(this + AFFECTED_MAP, 0x401);
        let update = e.mem.alloc(4);
        e.call(0x00a4_f3a0, &args![this, update]);
        assert_eq!(e.mem.u32(this + INDEX), 2);
        assert_eq!(
            calls(&e, TEXTURING_PROPERTY_SHADER_MAP),
            vec![vec![target, 1]; 2]
        );
        assert_eq!(calls(&e, MAP_SET_TEXTURE), vec![vec![0x7801, 0x6200]]);
        // A shader index beyond the shader maps: nothing.
        e.mem.set_u32(this + AFFECTED_MAP, 0x402);
        e.call(0x00a4_f3a0, &args![this, update]);
        assert_eq!(calls(&e, MAP_SET_TEXTURE).len(), 1);
        // An unset plain map falls through to the shader maps (index
        // 0 - 0x400 is far beyond them).
        e.mem.set_u32(this + AFFECTED_MAP, 1);
        e.call(0x00a4_f3a0, &args![this, update]);
        assert_eq!(calls(&e, TEXTURING_PROPERTY_SHADER_MAP).len(), 2);
        // No target: only the index.
        e.mem.set_u32(this + TARGET, 0);
        e.call(0x00a4_f3a0, &args![this, update]);
        assert_eq!(calls(&e, MAP_SET_TEXTURE).len(), 1);
    }

    #[test]
    fn update_skips_unless_always_and_resets_with_flag_0x20() {
        let mut e = engine();
        e.register(DONT_DO_UPDATE, |_, _| 1u32.into_ret());
        let this = updating(&mut e, 0.0, false);
        let update = e.mem.alloc(4);
        e.call(0x00a4_f3a0, &args![this, update]);
        assert!(calls(&e, INTERPOLATOR_UPDATE).is_empty());
        let this = updating(&mut e, 0.0, true);
        e.call(0x00a4_f3a0, &args![this, update]);
        assert_eq!(calls(&e, INTERPOLATOR_UPDATE).len(), 1);
        e.map(SCALED_TIME_RESET, 4);
        e.set_global(SCALED_TIME_RESET, f32::MIN.to_bits());
        let this = updating(&mut e, 0.0, false);
        e.mem.set_u8(this + FLAGS, 0x20);
        e.call(0x00a4_f3a0, &args![this, update]);
        assert_eq!(calls(&e, DONT_DO_UPDATE).len(), 2);
        assert_eq!(e.mem.f32(this + SCALED_TIME), f32::MIN);
        assert_eq!(calls(&e, INTERPOLATOR_UPDATE).len(), 2);
    }

    /// A stream whose binary streams transfer through `TRANSFER` (logging
    /// the buffer and the component size, filling reads from `words`).
    fn new_stream(e: &mut Engine, version: u32, words: &'static [u32]) -> (u32, u32) {
        let mut next = words.iter();
        e.register_double(TRANSFER, move |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (4, 4, 1));
            if let Some(word) = next.next() {
                e.mem.set_u32(a[1], *word);
            }
            4u32.into_ret()
        });
        let binary = e.mem.alloc(0x10);
        e.mem.set_u32(binary + BINARY_STREAM_READ, TRANSFER);
        e.mem.set_u32(binary + BINARY_STREAM_WRITE, TRANSFER);
        // The stream reaches past `m_pkOstr` (+0x250), beyond the 0x60 bytes `object`
        // gives: a block of its own with the doubles' vtable, so the fields stay in it.
        let doubles = object(e, 1);
        let vtable = e.mem.u32(doubles);
        let stream = e.mem.alloc(STREAM_OUTPUT + 4);
        e.mem.set_u32(stream, vtable);
        e.mem.set_u32(stream + 4, 1);
        e.mem.set_u32(stream + STREAM_INPUT, binary);
        e.mem.set_u32(stream + STREAM_OUTPUT, binary);
        e.mem.set_u32(stream + STREAM_FILE_VERSION, version);
        (stream, binary)
    }

    #[test]
    fn save_binary_writes_the_map_count_and_links() {
        let mut e = engine();
        e.register(BASE_SAVE_BINARY, |_, _| Ret::default());
        let (stream, binary) = new_stream(&mut e, 0, &[]);
        let written = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let seen = written.clone();
        e.register_double(TRANSFER, move |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (4, 4, 1));
            seen.borrow_mut().push(e.mem.u32(a[1]));
            4u32.into_ret()
        });
        let this = controller(&mut e, &[0x6000, 0x6100]);
        e.mem.set_u32(this + AFFECTED_MAP, 3);
        e.call(0x00a4_f4e0, &args![this, stream]);
        assert_eq!(calls(&e, BASE_SAVE_BINARY), vec![vec![this, stream]]);
        assert_eq!(calls(&e, TRANSFER)[0][1], this + AFFECTED_MAP);
        assert_eq!(*written.borrow(), [3, 2]);
        assert_eq!(
            calls(&e, SAVE_LINK_ID),
            vec![vec![stream, 0x6000], vec![stream, 0x6100]]
        );
        assert_eq!(e.mem.u32(binary + BINARY_STREAM_POSITION), 8);
    }

    #[test]
    fn destructor_frees_the_id_and_textures_and_releases_the_interpolator() {
        let mut e = engine();
        for address in [NI_FREE, POINTER_ARRAY_DELETE, BASE_DESTRUCT] {
            e.register(address, |_, _| Ret::default());
        }
        let this = controller(&mut e, &[0x6000]);
        let base = e.mem.u32(this + TEXTURES_BASE);
        let interpolator = object(&mut e, 1);
        e.mem.set_u32(this + INTERPOLATOR, interpolator);
        e.mem.set_u32(this + CONTROLLER_ID, 0x5500);
        e.call(0x00a4_f570, &args![this]);
        assert_eq!(e.mem.u32(this), NI_FLIP_CONTROLLER_VTABLE);
        assert_eq!(calls(&e, NI_FREE), vec![vec![0x5500]]);
        assert_eq!(e.mem.u32(this + CONTROLLER_ID), 0);
        assert_eq!(e.mem.u32(this + TEXTURES), TEXTURE_ARRAY_VTABLE);
        assert_eq!(calls(&e, POINTER_ARRAY_DELETE), vec![vec![base]]);
        assert_eq!(calls(&e, DELETE_THIS), vec![vec![interpolator]]);
        assert_eq!(calls(&e, BASE_DESTRUCT), vec![vec![this]]);
    }

    #[test]
    fn get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a4_f5d0, &args![]).u32(), NI_FLIP_CONTROLLER_RTTI);
    }

    #[test]
    fn interp_target_is_correct_type_walks_the_rtti_chain() {
        let mut e = engine();
        e.map(NI_TEXTURING_PROPERTY_RTTI, 8);
        let derived = e.mem.alloc(8);
        e.mem.set_u32(derived + 4, NI_TEXTURING_PROPERTY_RTTI);
        let unrelated = e.mem.alloc(8);
        let target = object(&mut e, 1);
        e.register_double(GET_RTTI, move |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.mem.set_u32(target + 8, derived);
        assert!(e.call(0x00a4_f5e0, &args![0u32, target]).bool());
        e.mem.set_u32(target + 8, unrelated);
        assert!(!e.call(0x00a4_f5e0, &args![0u32, target]).bool());
        assert!(!e.call(0x00a4_f5e0, &args![0u32, 0u32]).bool());
        assert_eq!(calls(&e, GET_RTTI).len(), 2);
    }

    #[test]
    fn load_binary_of_a_new_file_reads_the_map_and_links() {
        let mut e = engine();
        e.register(FLOAT_INTERP_CONTROLLER_LOAD_BINARY, |_, _| Ret::default());
        e.register(STREAM_READ_MULTIPLE_LINK_IDS, |_, _| 2u32.into_ret());
        let (stream, _) = new_stream(&mut e, VERSION_10_1_0_104, &[5]);
        let this = controller(&mut e, &[]);
        e.call(0x00a4_f610, &args![this, stream]);
        assert_eq!(
            calls(&e, FLOAT_INTERP_CONTROLLER_LOAD_BINARY),
            vec![vec![this, stream]]
        );
        assert_eq!(e.mem.u32(this + AFFECTED_MAP), 5);
        assert_eq!(calls(&e, STREAM_READ_MULTIPLE_LINK_IDS), vec![vec![stream]]);
        assert!(calls(&e, NI_ALLOCATE).is_empty());
    }

    #[test]
    fn load_binary_of_an_old_file_builds_step_keys() {
        let mut e = engine();
        e.register(FLOAT_INTERP_CONTROLLER_LOAD_BINARY, |_, _| Ret::default());
        e.register(STREAM_READ_MULTIPLE_LINK_IDS, |_, _| 2u32.into_ret());
        e.register(NI_OBJECT_CONSTRUCT, |_, _| Ret::default());
        e.register(FLOAT_DATA_REPLACE_ANIM, |_, _| Ret::default());
        let data = e.mem.alloc(0x18);
        let block = e.mem.alloc(0x18);
        let mut blocks = vec![data, block];
        e.register_double(NI_ALLOCATE, move |_, _| blocks.remove(0).into_ret());
        let keys = e.mem.alloc(0x1c);
        e.register_double(MEMORY_ALLOCATE, move |_, _| keys.into_ret());
        let interpolator = object(&mut e, 0);
        e.register_double(FLOAT_INTERPOLATOR_CONSTRUCT_WITH_DATA, move |_, _| {
            interpolator.into_ret()
        });
        let (stream, _) = new_stream(&mut e, 0x0a01_0067, &[6, 0x3f00_0000, 0x3e80_0000]);
        let this = controller(&mut e, &[]);
        let old = object(&mut e, 1);
        e.mem.set_u32(this + INTERPOLATOR, old);
        e.call(0x00a4_f610, &args![this, stream]);
        assert_eq!(e.mem.u32(this + AFFECTED_MAP), 6);
        assert_eq!(e.mem.u32(data), NI_FLOAT_DATA_VTABLE);
        assert_eq!(calls(&e, MEMORY_ALLOCATE), vec![vec![MEMORY_MANAGER, 0x1c]]);
        assert_eq!(e.mem.u32(keys), 3);
        let times: Vec<f32> = (0..3).map(|i| e.mem.f32(keys + 4 + 8 * i)).collect();
        let values: Vec<f32> = (0..3).map(|i| e.mem.f32(keys + 8 + 8 * i)).collect();
        assert_eq!(times, [0.5, 0.75, 1.0]);
        assert_eq!(values, [0.0, 1.0, 1.0]);
        assert_eq!(
            calls(&e, FLOAT_DATA_REPLACE_ANIM),
            vec![vec![data, keys + 4, 3, 5]]
        );
        assert_eq!(e.mem.f32(this + LO_KEY_TIME), 0.5);
        assert_eq!(e.mem.f32(this + HI_KEY_TIME), 1.0);
        assert_eq!(
            calls(&e, FLOAT_INTERPOLATOR_CONSTRUCT_WITH_DATA),
            vec![vec![block, data]]
        );
        assert_eq!(e.mem.u32(this + INTERPOLATOR), interpolator);
        assert_eq!(e.mem.u32(interpolator + 4), 1);
        assert_eq!(calls(&e, DELETE_THIS), vec![vec![old]]);
        assert_eq!(calls(&e, COLLAPSE), vec![vec![interpolator]]);
    }

    #[test]
    fn constructor_sets_up_an_empty_texture_array() {
        let mut e = engine();
        e.register(BASE_CONSTRUCT, |_, _| Ret::default());
        e.register(POINTER_ARRAY_ALLOCATE, |_, _| 0x6600u32.into_ret());
        let this = e.mem.alloc(NI_FLIP_CONTROLLER_SIZE);
        e.mem.write(this, &[0xaa; 0x54]);
        assert_eq!(e.call(0x00a4_f820, &args![this]).u32(), this);
        assert_eq!(calls(&e, BASE_CONSTRUCT), vec![vec![this]]);
        assert_eq!(calls(&e, POINTER_ARRAY_ALLOCATE), vec![vec![1]]);
        assert_eq!(e.mem.u32(this), NI_FLIP_CONTROLLER_VTABLE);
        assert_eq!(e.mem.u32(this + TEXTURES), TEXTURE_OBJECT_ARRAY_VTABLE);
        assert_eq!(e.mem.u32(this + TEXTURES_BASE), 0x6600);
        assert_eq!(
            [
                e.mem.u16(this + TEXTURES_MAX_SIZE),
                e.mem.u16(this + TEXTURES_SIZE),
                e.mem.u16(this + TEXTURES_ESIZE),
                e.mem.u16(this + TEXTURES_GROW_BY)
            ],
            [1, 0, 0, 1]
        );
        assert_eq!(e.mem.bytes(this + INDEX, 12), vec![0; 12]);
    }

    #[test]
    fn set_texture_at_grows_the_array_and_holds_the_texture() {
        let mut e = engine();
        e.register(TEXTURE_ARRAY_SET_SIZE, |_, _| Ret::default());
        e.register(TEXTURE_ARRAY_SET_AT, |e, a| {
            assert_eq!(e.mem.u32(e.mem.u32(a[2]) + 4), 2);
            Ret::default()
        });
        let texture = object(&mut e, 1);
        let this = controller(&mut e, &[0, 0]);
        e.mem.set_u16(this + TEXTURES_MAX_SIZE, 3);
        e.mem.set_u16(this + TEXTURES_GROW_BY, 4);
        // Inside the size: no resize.
        e.call(0x00a4_f8a0, &args![this, texture, 1u32]);
        assert!(calls(&e, TEXTURE_ARRAY_SET_SIZE).is_empty());
        assert_eq!(
            calls(&e, TEXTURE_ARRAY_SET_AT)[0][..2],
            [this + TEXTURES, 1]
        );
        assert_eq!(e.mem.u32(texture + 4), 1);
        // Past the size: size + 1; past the capacity: grown by the step.
        e.call(0x00a4_f8a0, &args![this, texture, 3u32]);
        assert_eq!(
            calls(&e, TEXTURE_ARRAY_SET_SIZE),
            vec![vec![this + TEXTURES, 4], vec![this + TEXTURES, 7]]
        );
        assert_eq!(e.mem.u32(texture + 4), 1);
    }

    #[test]
    fn copy_members_copies_the_map_and_every_texture() {
        let mut e = engine();
        e.register(BASE_COPY_MEMBERS, |_, _| Ret::default());
        e.register(TEXTURE_ARRAY_SET_SIZE, |_, _| Ret::default());
        e.register(TEXTURE_ARRAY_SET_AT, |_, _| Ret::default());
        let texture = object(&mut e, 1);
        let this = controller(&mut e, &[texture, 0]);
        e.mem.set_u32(this + AFFECTED_MAP, 9);
        let destination = controller(&mut e, &[0, 0]);
        e.call(0x00a4_f920, &args![this, destination, 0x4000u32]);
        assert_eq!(
            calls(&e, BASE_COPY_MEMBERS),
            vec![vec![this, destination, 0x4000]]
        );
        assert_eq!(e.mem.u32(destination + AFFECTED_MAP), 9);
        let set_at = calls(&e, TEXTURE_ARRAY_SET_AT);
        assert_eq!(set_at.len(), 2);
        assert_eq!((set_at[0][1], set_at[1][1]), (0, 1));
        // An empty array copies only the map.
        let empty = controller(&mut e, &[]);
        e.call(0x00a4_f920, &args![empty, destination, 0x4000u32]);
        assert_eq!(calls(&e, TEXTURE_ARRAY_SET_AT).len(), 2);
    }

    #[test]
    fn create_object_constructs_or_answers_null() {
        let mut e = engine();
        e.register(BASE_CONSTRUCT, |_, _| Ret::default());
        e.register(POINTER_ARRAY_ALLOCATE, |_, _| Ret::default());
        let block = e.mem.alloc(NI_FLIP_CONTROLLER_SIZE);
        e.register_double(NI_ALLOCATE, move |_, _| block.into_ret());
        assert_eq!(e.call(0x00a4_f970, &args![]).u32(), block);
        assert_eq!(calls(&e, NI_ALLOCATE), vec![vec![0x54]]);
        assert_eq!(e.mem.u32(block), NI_FLIP_CONTROLLER_VTABLE);
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00a4_f970, &args![]).u32(), 0);
        assert_eq!(calls(&e, BASE_CONSTRUCT).len(), 1);
    }

    #[test]
    fn link_object_fills_the_textures_from_the_links() {
        let mut e = engine();
        e.register(BASE_LINK_OBJECT, |_, _| Ret::default());
        e.register(STREAM_GET_NUMBER_OF_LINK_IDS, |_, _| 2u32.into_ret());
        e.register(TEXTURE_ARRAY_SET_SIZE, |_, _| Ret::default());
        e.register(TEXTURE_ARRAY_SET_AT, |_, _| Ret::default());
        let texture = object(&mut e, 1);
        let mut links = vec![texture, 0];
        e.register_double(STREAM_GET_OBJECT_FROM_LINK_ID, move |_, _| {
            links.remove(0).into_ret()
        });
        let this = controller(&mut e, &[]);
        // No target: only the base.
        e.call(0x00a4_f990, &args![this, 0x8000u32]);
        assert!(calls(&e, STREAM_GET_NUMBER_OF_LINK_IDS).is_empty());
        e.mem.set_u32(this + TARGET, 0x9000);
        e.call(0x00a4_f990, &args![this, 0x8000u32]);
        assert_eq!(calls(&e, BASE_LINK_OBJECT), vec![vec![this, 0x8000]; 2]);
        assert_eq!(
            calls(&e, TEXTURE_ARRAY_SET_SIZE),
            vec![vec![this + TEXTURES, 2]]
        );
        let set_at = calls(&e, TEXTURE_ARRAY_SET_AT);
        assert_eq!((set_at[0][1], set_at[1][1]), (0, 1));
        assert_eq!(e.mem.u32(texture + 4), 1);
    }

    #[test]
    fn create_clone_constructs_and_copies_or_copies_into_null() {
        let mut e = engine();
        e.register(BASE_CONSTRUCT, |_, _| Ret::default());
        e.register(POINTER_ARRAY_ALLOCATE, |_, _| Ret::default());
        e.register(BASE_COPY_MEMBERS, |_, _| Ret::default());
        let block = e.mem.alloc(NI_FLIP_CONTROLLER_SIZE);
        e.register_double(NI_ALLOCATE, move |_, _| block.into_ret());
        let this = controller(&mut e, &[]);
        e.mem.set_u32(this + AFFECTED_MAP, 2);
        assert_eq!(e.call(0x00a4_fa20, &args![this, 0x4000u32]).u32(), block);
        assert_eq!(e.mem.u32(block), NI_FLIP_CONTROLLER_VTABLE);
        assert_eq!(e.mem.u32(block + AFFECTED_MAP), 2);
        e.map(0, 0x100);
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00a4_fa20, &args![this, 0x4000u32]).u32(), 0);
        assert_eq!(calls(&e, BASE_COPY_MEMBERS)[1], vec![this, 0, 0x4000]);
        assert_eq!(e.mem.u32(AFFECTED_MAP), 2);
    }
}
