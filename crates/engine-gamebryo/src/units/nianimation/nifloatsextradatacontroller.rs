//! `NiAnimation/nifloatsextradatacontroller.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiFloatsExtraDataController::ms_RTTI` (the class's `NiRTTI` record).
const NI_FLOATS_EXTRA_DATA_CONTROLLER_RTTI: u32 = 0x011f_3fcc;

// Translated from 00a4ef60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatsExtraDataController::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_floats_extra_data_controller_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_FLOATS_EXTRA_DATA_CONTROLLER_RTTI)
}

/// The base class's `CopyMembers` (`00a38ef0`, `thiscall (this, dest,
/// cloning process)`; the engine map's name for that address,
/// `NiPSysModifierCtlr::CopyMembers`, is a folded body's).
const BASE_COPY_MEMBERS: u32 = 0x00a3_8ef0;
/// `InterlockedIncrement` and `InterlockedDecrement` (import slots, one word:
/// the address).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The pooled empty `NiFixedString` handle (`0109b220`), never counted.
const FIXED_STRING_EMPTY: u32 = 0x0109_b220;
/// The controller's extra-data name (`NiFixedString`) at +0x38.
const EXTRA_DATA_NAME: u32 = 0x38;

// Translated from 00c312f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatsExtraDataController::CopyMembers` (Xbox PDB): the base class's
/// `CopyMembers`, then the extra-data name at +0x38 assigned to `dest`
/// (when it differs: the new handle counted unless empty, the old one
/// released unless empty).
pub fn ni_floats_extra_data_controller_copy_members(
    e: &mut Engine,
    this: Ptr,
    dest: Ptr,
    cloning: Ptr,
) {
    e.call(BASE_COPY_MEMBERS, &args![this, dest, cloning]);
    let name = e.mem.u32(this.addr() + EXTRA_DATA_NAME);
    let slot = dest.addr().wrapping_add(EXTRA_DATA_NAME);
    if e.mem.u32(slot) == name {
        return;
    }
    let empty = e.global::<u32>(FIXED_STRING_EMPTY);
    if name != empty {
        e.call(INTERLOCKED_INCREMENT, &args![name.wrapping_sub(8)]);
    }
    let old = e.mem.u32(slot);
    if old != e.global::<u32>(FIXED_STRING_EMPTY) {
        e.call(INTERLOCKED_DECREMENT, &args![old.wrapping_sub(8)]);
    }
    e.mem.set_u32(slot, name);
}

/// `NiFloatsExtraDataController` (Xbox PDB, 0x48 bytes; the PC code uses the
/// same offsets): `m_iFloatsExtraDataIndex` +0x40 and `m_kCtlrID`
/// (`NiFixedString`) +0x44; from the base classes `m_uFlags` +0x08,
/// `m_fScaledTime` +0x28, `m_pkTarget` +0x2C, `m_spInterpolator` +0x34 and
/// `m_spExtraData` +0x3C.
const FLOATS_INDEX: u32 = 0x40;
const CONTROLLER_ID: u32 = 0x44;
const FLAGS: u32 = 0x08;
const SCALED_TIME: u32 = 0x28;
const TARGET: u32 = 0x2c;
const INTERPOLATOR: u32 = 0x34;
const EXTRA_DATA: u32 = 0x3c;
/// The class's size and vtable (RTTI `.?AVNiFloatsExtraDataController@@`).
const NI_FLOATS_EXTRA_DATA_CONTROLLER_SIZE: u32 = 0x48;
const NI_FLOATS_EXTRA_DATA_CONTROLLER_VTABLE: u32 = 0x0109_928c;
/// `00aa13e0` (`cdecl`, one word): allocates from the Ni allocator; and the
/// sized delete `00aa1460` (`cdecl`: block, size).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const SIZED_DELETE: u32 = 0x00aa_1460;
/// `NiExtraDataController`'s constructor (`00a572a0`) and destructor
/// (`00a572d0`), `thiscall` with no stack word, and its `IsEqual`
/// (`00a571e0`) and `GetViewerStrings` (`00a57420`).
const EXTRA_DATA_CONTROLLER_CONSTRUCT: u32 = 0x00a5_72a0;
const EXTRA_DATA_CONTROLLER_DESTRUCT: u32 = 0x00a5_72d0;
const EXTRA_DATA_CONTROLLER_IS_EQUAL: u32 = 0x00a5_71e0;
const EXTRA_DATA_CONTROLLER_GET_VIEWER_STRINGS: u32 = 0x00a5_7420;
/// The base `LoadBinary` (`00a571a0`) and `SaveBinary` (`00a57130`) this
/// class calls (`thiscall`: stream; the engine map's names for those
/// addresses, `NiPSysModifierCtlr::...`, are folded bodies').
const BASE_LOAD_BINARY: u32 = 0x00a5_71a0;
const BASE_SAVE_BINARY: u32 = 0x00a5_7130;
/// `NiFloatsExtraData::GetValue` (`00a89020`; `thiscall`: index; the float
/// in ST0) and `NiFloatsExtraData::SetValue` (`00a89000`; `thiscall`:
/// index, float).
const FLOATS_EXTRA_DATA_GET_VALUE: u32 = 0x00a8_9020;
const FLOATS_EXTRA_DATA_SET_VALUE: u32 = 0x00a8_9000;
/// `NiFloatInterpolator`'s constructor (`00a3c390`; `thiscall`: the float
/// value), `NiFloatInterpolator::SetPoseValue` (`00a3c500`; `thiscall`: the
/// float) and `NiFloatInterpolator::ms_RTTI` (`011f3738`).
const FLOAT_INTERPOLATOR_CONSTRUCT: u32 = 0x00a3_c390;
const FLOAT_INTERPOLATOR_SET_POSE_VALUE: u32 = 0x00a3_c500;
const NI_FLOAT_INTERPOLATOR_RTTI: u32 = 0x011f_3738;
/// `00653290` (`thiscall` on an object, `RET 4`: an `NiRTTI` record): the
/// object when it is of that class (or derived), else 0.
const OBJECT_CAST: u32 = 0x0065_3290;
/// `NiTimeController::DontDoUpdate` (`00a36250`; `thiscall`: time).
const DONT_DO_UPDATE: u32 = 0x00a3_6250;
/// The float the scaled time is set to when flag 0x20 is set (`01096324`).
const SCALED_TIME_RESET: u32 = 0x0109_6324;
/// Interpolator virtual slots (Xbox PDB names): `AlwaysUpdate` +0xD4 and the
/// float `Update` +0x9C (`(time, target, &value)`).
const SLOT_ALWAYS_UPDATE: u32 = 0xd4;
const SLOT_UPDATE_FLOAT: u32 = 0x9c;
/// `00a645b0` (`thiscall` on the stream, no stack word): reads one link ID
/// (4 bytes) and appends it to the stream's link-ID list.
const STREAM_READ_LINK_ID: u32 = 0x00a6_45b0;
/// `NiStream::m_uiNifFileVersion` (Xbox PDB) +0xD8; files before 10.1.0.104
/// (`0x0a010068`) carry an extra link ID here.
const STREAM_FILE_VERSION: u32 = 0xd8;
const VERSION_10_1_0_104: u32 = 0x0a01_0068;
/// `NiStream::m_pkIstr` +0x24c and `m_pkOstr` +0x250 (Xbox PDB), and the
/// binary stream's position (+4), read (+8) and write (+0xc) functions.
const STREAM_INPUT: u32 = 0x24c;
const STREAM_OUTPUT: u32 = 0x250;
const BINARY_STREAM_POSITION: u32 = 0x4;
const BINARY_STREAM_READ: u32 = 0x8;
const BINARY_STREAM_WRITE: u32 = 0xc;
/// `NiGetViewerString` (`00a7d330`, `cdecl`: the class name) and
/// `NiGetViewerString_ov7` (engine map; `00a7d4d0`, `cdecl`: label, value).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const NI_GET_VIEWER_STRING_INT: u32 = 0x00a7_d4d0;
/// `"m_iFloatsExtraDataIndex"`.
const LABEL_FLOATS_INDEX: u32 = 0x0109_9270;
/// The viewer-string array's grow (`0096ad30`) and store (`0096ae90`).
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;

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

/// Appends `text` to the viewer-string array at `strings` (grown by its
/// step when full), through a stack word holding it.
fn add_viewer_string(e: &mut Engine, strings: u32, text: u32) {
    let size = u32::from(e.mem.u16(strings.wrapping_add(0xa)));
    if size >= u32::from(e.mem.u16(strings.wrapping_add(8))) {
        let step = u32::from(e.mem.u16(strings.wrapping_add(0xe)));
        e.call(STRINGS_GROW, &args![strings, step + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRINGS_SET_AT, &args![strings, size, slot]);
    });
}

/// The constructor `CreateObject` and `CreateClone` inline:
/// `NiExtraDataController`'s constructor, the vtable, index -1 and the
/// controller ID set to the empty string's handle (read, not counted).
fn construct_inline(e: &mut Engine, object: u32) {
    e.call(EXTRA_DATA_CONTROLLER_CONSTRUCT, &args![object]);
    e.mem
        .set_u32(object, NI_FLOATS_EXTRA_DATA_CONTROLLER_VTABLE);
    e.mem
        .set_u32(object.wrapping_add(FLOATS_INDEX), 0xffff_ffff);
    let empty = e.global::<u32>(FIXED_STRING_EMPTY);
    e.mem.set_u32(object.wrapping_add(CONTROLLER_ID), empty);
}

// Translated from 00a4eb10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatsExtraDataController::IsEqual` (Xbox PDB):
/// `NiExtraDataController::IsEqual` and the same floats index.
pub fn ni_floats_extra_data_controller_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e
        .call(EXTRA_DATA_CONTROLLER_IS_EQUAL, &args![this, other])
        .bool()
    {
        return false;
    }
    e.mem.u32(this.addr().wrapping_add(FLOATS_INDEX))
        == e.mem.u32(other.addr().wrapping_add(FLOATS_INDEX))
}

// Translated from 00a4ef70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatsExtraDataController::CreatePoseInterpolator` (Xbox PDB;
/// `thiscall`, `RET 4`; the index word is not read): allocates 0x18 bytes
/// and constructs an `NiFloatInterpolator` (`00a3c390`) holding the extra
/// data's float at the floats index (`NiFloatsExtraData::GetValue`, stored
/// as a float). Answers the constructor's result, 0 when the allocation
/// fails.
pub fn ni_floats_extra_data_controller_create_pose_interpolator(
    e: &mut Engine,
    this: Ptr,
    _unused_1: u32,
) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![0x18u32]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    let index = e.mem.u32(this.addr().wrapping_add(FLOATS_INDEX));
    let extra = e.mem.u32(this.addr().wrapping_add(EXTRA_DATA));
    let value = e
        .call(FLOATS_EXTRA_DATA_GET_VALUE, &args![extra, index])
        .f64();
    e.call(
        FLOAT_INTERPOLATOR_CONSTRUCT,
        &args![block, x87_store32(value)],
    )
    .ptr()
}

// Translated from 00a4efb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatsExtraDataController::CreateClone` (Xbox PDB; `RET 4`: cloning
/// process): a new 0x48-byte controller built as `CreateObject` builds it
/// (0 when the allocation fails), the member copy (`00c312f0`) into it, then
/// the floats index copied (also through a null clone, at address 0x40).
pub fn ni_floats_extra_data_controller_create_clone(
    e: &mut Engine,
    this: Ptr,
    cloning: Ptr,
) -> Ptr {
    let block = e
        .call(NI_ALLOCATE, &args![NI_FLOATS_EXTRA_DATA_CONTROLLER_SIZE])
        .u32();
    if block != 0 {
        construct_inline(e, block);
    }
    ni_floats_extra_data_controller_copy_members(e, this, Ptr::new(block), cloning);
    let index = e.mem.u32(this.addr().wrapping_add(FLOATS_INDEX));
    e.mem.set_u32(block.wrapping_add(FLOATS_INDEX), index);
    Ptr::new(block)
}

// Translated from 00a4f000 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `NiFloatsExtraDataController::CreateObject` (Ghidra has no function
/// here; the loader's factory by its body): allocates 0x48 bytes and, when
/// that succeeds, runs `NiExtraDataController`'s constructor, stores the
/// vtable, index -1 and the empty controller ID. Returns the object, or 0.
pub fn ni_floats_extra_data_controller_create_object(e: &mut Engine) -> Ptr {
    let block = e
        .call(NI_ALLOCATE, &args![NI_FLOATS_EXTRA_DATA_CONTROLLER_SIZE])
        .u32();
    if block == 0 {
        return Ptr::new(0);
    }
    construct_inline(e, block);
    Ptr::new(block)
}

// Translated from 00a4f040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatsExtraDataController::_scalar_deleting_destructor_` (Xbox PDB):
/// the vtable, the controller ID released unless it is the empty string,
/// `NiExtraDataController`'s destructor (`00a572d0`), then the sized delete
/// of 0x48 bytes when bit 0 of `flags` is set. Returns `this`.
pub fn ni_floats_extra_data_controller_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    let object = this.addr();
    e.mem
        .set_u32(object, NI_FLOATS_EXTRA_DATA_CONTROLLER_VTABLE);
    let id = e.mem.u32(object.wrapping_add(CONTROLLER_ID));
    if id != e.global::<u32>(FIXED_STRING_EMPTY) {
        e.call(INTERLOCKED_DECREMENT, &args![id.wrapping_sub(8)]);
    }
    e.call(EXTRA_DATA_CONTROLLER_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(
            SIZED_DELETE,
            &args![this, NI_FLOATS_EXTRA_DATA_CONTROLLER_SIZE],
        );
    }
    this
}

// Translated from 00a4f080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatsExtraDataController::SynchronizePoseInterpolator` (Xbox PDB;
/// `RET 8`: interpolator, index; the index word is not read): the
/// interpolator cast to `NiFloatInterpolator` (`00653290`; a null
/// interpolator gives 0 without the call) gets the extra data's float at
/// the floats index (`NiFloatsExtraData::GetValue`, stored as a float) as
/// its pose value (`SetPoseValue`, called on the cast result even when it
/// is 0).
pub fn ni_floats_extra_data_controller_synchronize_pose_interpolator(
    e: &mut Engine,
    this: Ptr,
    interpolator: Ptr,
    _unused_2: u32,
) {
    let cast = if interpolator.addr() == 0 {
        0
    } else {
        e.call(
            OBJECT_CAST,
            &args![interpolator, NI_FLOAT_INTERPOLATOR_RTTI],
        )
        .u32()
    };
    let index = e.mem.u32(this.addr().wrapping_add(FLOATS_INDEX));
    let extra = e.mem.u32(this.addr().wrapping_add(EXTRA_DATA));
    let value = e
        .call(FLOATS_EXTRA_DATA_GET_VALUE, &args![extra, index])
        .f64();
    e.call(
        FLOAT_INTERPOLATOR_SET_POSE_VALUE,
        &args![cast, x87_store32(value)],
    );
}

// Translated from 00a4f0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatsExtraDataController::Update` (Xbox PDB; `RET 4`: the update
/// data, whose first word is the time). With flag 0x20 of the flags byte
/// the scaled time becomes the float at `01096324`; otherwise
/// `NiTimeController::DontDoUpdate` decides, and when it says skip the
/// update still runs if the interpolator's `AlwaysUpdate` (slot +0xD4)
/// answers true. The interpolator's float `Update` (slot +0x9C: scaled
/// time, target, address of the value, which is the argument's own word)
/// then runs; on success, with extra data, `NiFloatsExtraData::SetValue`
/// stores the value at the floats index (x87 copy).
pub fn ni_floats_extra_data_controller_update(e: &mut Engine, this: Ptr, update: Ptr) {
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
    e.with_stack(4, |e, value| {
        e.mem.set_u32(value.addr(), update.addr());
        let scaled_time = x87_quiet_bits(e.mem.u32(object.wrapping_add(SCALED_TIME)));
        let target = e.mem.u32(object.wrapping_add(TARGET));
        if !e
            .vcall(
                interpolator,
                SLOT_UPDATE_FLOAT,
                &args![scaled_time, target, value],
            )
            .bool()
        {
            return;
        }
        let extra = e.mem.u32(object.wrapping_add(EXTRA_DATA));
        if extra == 0 {
            return;
        }
        let stored = x87_quiet_bits(e.mem.u32(value.addr()));
        let index = e.mem.u32(object.wrapping_add(FLOATS_INDEX));
        e.call(FLOATS_EXTRA_DATA_SET_VALUE, &args![extra, index, stored]);
    });
}

// Translated from 00a4f150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatsExtraDataController::LoadBinary` (Xbox PDB): the base
/// `LoadBinary` (`00a571a0`), the floats index (4 bytes, the component
/// size in the argument's own stack word), then, for files before
/// 10.1.0.104, one more link ID (`00a645b0`).
pub fn ni_floats_extra_data_controller_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(BASE_LOAD_BINARY, &args![this, stream]);
    let stream_addr = stream.addr();
    e.with_stack(4, |e, component_size| {
        transfer_word(
            e,
            stream_addr,
            STREAM_INPUT,
            BINARY_STREAM_READ,
            this.addr().wrapping_add(FLOATS_INDEX),
            component_size.addr(),
        );
    });
    if e.mem.u32(stream_addr.wrapping_add(STREAM_FILE_VERSION)) < VERSION_10_1_0_104 {
        e.call(STREAM_READ_LINK_ID, &args![stream]);
    }
}

// Translated from 00a4f1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatsExtraDataController::SaveBinary` (Xbox PDB): the base
/// `SaveBinary` (`00a57130`), then the floats index (4 bytes, the component
/// size in the argument's own stack word).
pub fn ni_floats_extra_data_controller_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(BASE_SAVE_BINARY, &args![this, stream]);
    e.with_stack(4, |e, component_size| {
        transfer_word(
            e,
            stream.addr(),
            STREAM_OUTPUT,
            BINARY_STREAM_WRITE,
            this.addr().wrapping_add(FLOATS_INDEX),
            component_size.addr(),
        );
    });
}

// Translated from 00a4f1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatsExtraDataController::GetViewerStrings` (Xbox PDB):
/// `NiExtraDataController::GetViewerStrings`, then the class name (the
/// `NiRTTI` record's first word through `NiGetViewerString`) and the floats
/// index (`NiGetViewerString_ov7("m_iFloatsExtraDataIndex", index)`).
pub fn ni_floats_extra_data_controller_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    let strings = strings.addr();
    e.call(
        EXTRA_DATA_CONTROLLER_GET_VIEWER_STRINGS,
        &args![this, strings],
    );
    let name = e.mem.u32(NI_FLOATS_EXTRA_DATA_CONTROLLER_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![name]).u32();
    add_viewer_string(e, strings, text);
    let index = e.mem.u32(this.addr().wrapping_add(FLOATS_INDEX));
    let text = e
        .call(NI_GET_VIEWER_STRING_INT, &args![LABEL_FLOATS_INDEX, index])
        .u32();
    add_viewer_string(e, strings, text);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a4ef60, ni_floats_extra_data_controller_get_rtti() -> Ptr),
        entry!(
            0x00c3_12f0,
            ni_floats_extra_data_controller_copy_members(Ptr, Ptr, Ptr)
        ),
        entry!(0x00a4_eb10, ni_floats_extra_data_controller_is_equal(Ptr, Ptr) -> bool),
        entry!(
            0x00a4_ef70,
            ni_floats_extra_data_controller_create_pose_interpolator(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00a4_efb0,
            ni_floats_extra_data_controller_create_clone(Ptr, Ptr) -> Ptr
        ),
        entry!(0x00a4_f000, ni_floats_extra_data_controller_create_object() -> Ptr),
        entry!(
            0x00a4_f040,
            ni_floats_extra_data_controller_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00a4_f080,
            ni_floats_extra_data_controller_synchronize_pose_interpolator(Ptr, Ptr, u32)
        ),
        entry!(
            0x00a4_f0c0,
            ni_floats_extra_data_controller_update(Ptr, Ptr)
        ),
        entry!(
            0x00a4_f150,
            ni_floats_extra_data_controller_load_binary(Ptr, Ptr)
        ),
        entry!(
            0x00a4_f1a0,
            ni_floats_extra_data_controller_save_binary(Ptr, Ptr)
        ),
        entry!(
            0x00a4_f1e0,
            ni_floats_extra_data_controller_get_viewer_strings(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_floats_extra_data_controller_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a4ef60, &args![]).u32(),
            NI_FLOATS_EXTRA_DATA_CONTROLLER_RTTI
        );
    }

    #[test]
    fn floats_extra_data_copy_members_assigns_the_counted_name() {
        let mut e = Engine::new();
        e.map(FIXED_STRING_EMPTY, 4);
        e.set_global(FIXED_STRING_EMPTY, 0x0500_0008u32);
        e.register(BASE_COPY_MEMBERS, |_, _| Ret::default());
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
        e.call_log = Some(vec![]);
        let name = e.mem.alloc(0x10) + 8;
        e.mem.set_u32(name - 8, 1);
        let old = e.mem.alloc(0x10) + 8;
        e.mem.set_u32(old - 8, 2);
        let this = e.mem.alloc(0x40);
        let dest = e.mem.alloc(0x40);
        e.mem.set_u32(this + EXTRA_DATA_NAME, name);
        e.mem.set_u32(dest + EXTRA_DATA_NAME, old);
        e.call(0x00c3_12f0, &args![this, dest, 0x7000u32]);
        assert_eq!(e.mem.u32(dest + EXTRA_DATA_NAME), name);
        assert_eq!(e.mem.u32(name - 8), 2);
        assert_eq!(e.mem.u32(old - 8), 1);
        // The same handle again: no counting.
        e.call(0x00c3_12f0, &args![this, dest, 0x7000u32]);
        assert_eq!(e.mem.u32(name - 8), 2);
        // The empty string replaces it without being counted.
        e.mem.set_u32(this + EXTRA_DATA_NAME, 0x0500_0008);
        e.call(0x00c3_12f0, &args![this, dest, 0x7000u32]);
        assert_eq!(e.mem.u32(dest + EXTRA_DATA_NAME), 0x0500_0008);
        assert_eq!(e.mem.u32(name - 8), 1);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log.iter()
                .filter(|call| call.0 == BASE_COPY_MEMBERS)
                .count(),
            3
        );
    }
}

#[cfg(test)]
mod n003_tests {
    use super::*;

    const INTERPOLATOR_UPDATE: u32 = 0x0320_0000;
    const INTERPOLATOR_ALWAYS: u32 = 0x0320_0010;
    const TRANSFER: u32 = 0x0320_0020;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    #[test]
    fn is_equal_needs_the_base_and_the_same_index() {
        let mut e = Engine::new();
        e.register(EXTRA_DATA_CONTROLLER_IS_EQUAL, |e, a| {
            e.mem.u32(a[0] + 4).into_ret()
        });
        let this = e.mem.alloc(0x48);
        let other = e.mem.alloc(0x48);
        e.mem.set_u32(this + 4, 1);
        e.mem.set_u32(this + FLOATS_INDEX, 2);
        e.mem.set_u32(other + FLOATS_INDEX, 2);
        assert!(e.call(0x00a4_eb10, &args![this, other]).bool());
        e.mem.set_u32(other + FLOATS_INDEX, 3);
        assert!(!e.call(0x00a4_eb10, &args![this, other]).bool());
        e.mem.set_u32(other + FLOATS_INDEX, 2);
        e.mem.set_u32(this + 4, 0);
        assert!(!e.call(0x00a4_eb10, &args![this, other]).bool());
    }

    #[test]
    fn create_pose_interpolator_holds_the_indexed_value() {
        let mut e = Engine::new();
        let block = e.mem.alloc(0x18);
        e.register_double(NI_ALLOCATE, move |_, _| block.into_ret());
        e.register(FLOATS_EXTRA_DATA_GET_VALUE, |_, _| st0(1.0 / 3.0));
        e.register(FLOAT_INTERPOLATOR_CONSTRUCT, |_, a| a[0].into_ret());
        let this = e.mem.alloc(0x48);
        e.mem.set_u32(this + FLOATS_INDEX, 2);
        e.mem.set_u32(this + EXTRA_DATA, 0x6000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a4_ef70, &args![this, 9u32]).u32(), block);
        assert_eq!(calls(&e, NI_ALLOCATE), vec![vec![0x18]]);
        assert_eq!(
            calls(&e, FLOATS_EXTRA_DATA_GET_VALUE),
            vec![vec![0x6000, 2]]
        );
        assert_eq!(
            calls(&e, FLOAT_INTERPOLATOR_CONSTRUCT),
            vec![vec![block, ((1.0f64 / 3.0) as f32).to_bits()]]
        );
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00a4_ef70, &args![this, 9u32]).u32(), 0);
        assert_eq!(calls(&e, FLOATS_EXTRA_DATA_GET_VALUE).len(), 1);
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(FIXED_STRING_EMPTY, 4);
        e.set_global(FIXED_STRING_EMPTY, 0x0800u32);
        // The base constructor leaves the empty name, as the real one does.
        e.register(EXTRA_DATA_CONTROLLER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0] + EXTRA_DATA_NAME, 0x0800);
            Ret::default()
        });
        e
    }

    #[test]
    fn create_object_builds_an_empty_controller_or_answers_null() {
        let mut e = engine();
        let block = e.mem.alloc(0x48);
        e.register_double(NI_ALLOCATE, move |_, _| block.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a4_f000, &args![]).u32(), block);
        assert_eq!(calls(&e, NI_ALLOCATE), vec![vec![0x48]]);
        assert_eq!(
            calls(&e, EXTRA_DATA_CONTROLLER_CONSTRUCT),
            vec![vec![block]]
        );
        assert_eq!(e.mem.u32(block), NI_FLOATS_EXTRA_DATA_CONTROLLER_VTABLE);
        assert_eq!(e.mem.u32(block + FLOATS_INDEX), 0xffff_ffff);
        assert_eq!(e.mem.u32(block + CONTROLLER_ID), 0x0800);
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00a4_f000, &args![]).u32(), 0);
        assert_eq!(calls(&e, EXTRA_DATA_CONTROLLER_CONSTRUCT).len(), 1);
    }

    #[test]
    fn create_clone_builds_copies_and_takes_the_index() {
        let mut e = engine();
        e.register(BASE_COPY_MEMBERS, |_, _| Ret::default());
        let block = e.mem.alloc(0x48);
        e.register_double(NI_ALLOCATE, move |_, _| block.into_ret());
        let this = e.mem.alloc(0x48);
        e.mem.set_u32(this + FLOATS_INDEX, 5);
        e.mem.set_u32(this + EXTRA_DATA_NAME, 0x0800);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a4_efb0, &args![this, 0x4000u32]).u32(), block);
        assert_eq!(e.mem.u32(block), NI_FLOATS_EXTRA_DATA_CONTROLLER_VTABLE);
        assert_eq!(e.mem.u32(block + FLOATS_INDEX), 5);
        assert_eq!(
            calls(&e, BASE_COPY_MEMBERS),
            vec![vec![this, block, 0x4000]]
        );
        // A failed allocation still copies, into address 0.
        e.map(0, 0x100);
        e.mem.set_u32(EXTRA_DATA_NAME, 0x0800);
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00a4_efb0, &args![this, 0x4000u32]).u32(), 0);
        assert_eq!(calls(&e, BASE_COPY_MEMBERS)[1], vec![this, 0, 0x4000]);
        assert_eq!(e.mem.u32(FLOATS_INDEX), 5);
    }

    #[test]
    fn scalar_deleting_destructor_releases_the_id_and_deletes_on_bit_0() {
        let mut e = engine();
        e.register(INTERLOCKED_DECREMENT, |_, _| Ret::default());
        e.register(EXTRA_DATA_CONTROLLER_DESTRUCT, |_, _| Ret::default());
        e.register(SIZED_DELETE, |_, _| Ret::default());
        let this = e.mem.alloc(0x48);
        e.mem.set_u32(this + CONTROLLER_ID, 0x5008);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a4_f040, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_FLOATS_EXTRA_DATA_CONTROLLER_VTABLE);
        assert_eq!(calls(&e, INTERLOCKED_DECREMENT), vec![vec![0x5000]]);
        assert_eq!(calls(&e, EXTRA_DATA_CONTROLLER_DESTRUCT), vec![vec![this]]);
        assert_eq!(calls(&e, SIZED_DELETE), vec![vec![this, 0x48]]);
        // The empty ID is not released; no delete without bit 0.
        e.mem.set_u32(this + CONTROLLER_ID, 0x0800);
        e.call(0x00a4_f040, &args![this, 2u32]);
        assert_eq!(calls(&e, INTERLOCKED_DECREMENT).len(), 1);
        assert_eq!(calls(&e, SIZED_DELETE).len(), 1);
    }

    #[test]
    fn synchronize_pose_interpolator_sets_the_indexed_value() {
        let mut e = Engine::new();
        e.register(OBJECT_CAST, |_, a| (a[0] + 1).into_ret());
        e.register(FLOATS_EXTRA_DATA_GET_VALUE, |_, _| st0(2.5));
        e.register(FLOAT_INTERPOLATOR_SET_POSE_VALUE, |_, _| Ret::default());
        let this = e.mem.alloc(0x48);
        e.mem.set_u32(this + FLOATS_INDEX, 1);
        e.mem.set_u32(this + EXTRA_DATA, 0x6000);
        e.call_log = Some(vec![]);
        e.call(0x00a4_f080, &args![this, 0x5000u32, 0u32]);
        assert_eq!(
            calls(&e, OBJECT_CAST),
            vec![vec![0x5000, NI_FLOAT_INTERPOLATOR_RTTI]]
        );
        assert_eq!(
            calls(&e, FLOATS_EXTRA_DATA_GET_VALUE),
            vec![vec![0x6000, 1]]
        );
        assert_eq!(
            calls(&e, FLOAT_INTERPOLATOR_SET_POSE_VALUE),
            vec![vec![0x5001, 2.5f32.to_bits()]]
        );
        e.call(0x00a4_f080, &args![this, 0u32, 0u32]);
        assert_eq!(calls(&e, OBJECT_CAST).len(), 1);
        assert_eq!(calls(&e, FLOAT_INTERPOLATOR_SET_POSE_VALUE)[1][0], 0);
    }

    fn controller(e: &mut Engine, flags: u8, always: bool) -> u32 {
        let vtable = e.mem.alloc(0x100);
        e.mem
            .set_u32(vtable + SLOT_UPDATE_FLOAT, INTERPOLATOR_UPDATE);
        e.mem
            .set_u32(vtable + SLOT_ALWAYS_UPDATE, INTERPOLATOR_ALWAYS);
        let interpolator = e.mem.alloc(8);
        e.mem.set_u32(interpolator, vtable);
        e.register(INTERPOLATOR_UPDATE, |e, a| {
            e.mem.set_u32(a[3], 0x7f80_0001);
            1u32.into_ret()
        });
        if always {
            e.register(INTERPOLATOR_ALWAYS, |_, _| 1u32.into_ret());
        } else {
            e.register(INTERPOLATOR_ALWAYS, |_, _| Ret::default());
        }
        e.register(FLOATS_EXTRA_DATA_SET_VALUE, |_, _| Ret::default());
        let this = e.mem.alloc(0x48);
        e.mem.set_u8(this + FLAGS, flags);
        e.mem.set_u32(this + SCALED_TIME, 1.25f32.to_bits());
        e.mem.set_u32(this + TARGET, 0x6000);
        e.mem.set_u32(this + INTERPOLATOR, interpolator);
        e.mem.set_u32(this + EXTRA_DATA, 0x6100);
        e.mem.set_u32(this + FLOATS_INDEX, 3);
        this
    }

    #[test]
    fn update_stores_the_interpolated_value_at_the_index() {
        let mut e = Engine::new();
        e.register(DONT_DO_UPDATE, |_, _| Ret::default());
        let this = controller(&mut e, 0, false);
        let update = e.mem.alloc(4);
        e.mem.set_f32(update, 4.0);
        e.call_log = Some(vec![]);
        e.call(0x00a4_f0c0, &args![this, update]);
        assert_eq!(
            calls(&e, DONT_DO_UPDATE),
            vec![vec![this, 4.0f32.to_bits()]]
        );
        assert_eq!(
            calls(&e, INTERPOLATOR_UPDATE)[0][1..3],
            [1.25f32.to_bits(), 0x6000]
        );
        assert_eq!(
            calls(&e, FLOATS_EXTRA_DATA_SET_VALUE),
            vec![vec![0x6100, 3, 0x7fc0_0001]]
        );
        // No extra data: nothing stored.
        e.mem.set_u32(this + EXTRA_DATA, 0);
        e.call(0x00a4_f0c0, &args![this, update]);
        assert_eq!(calls(&e, FLOATS_EXTRA_DATA_SET_VALUE).len(), 1);
    }

    #[test]
    fn update_skips_unless_the_interpolator_always_updates_or_flag_0x20() {
        let mut e = Engine::new();
        e.register(DONT_DO_UPDATE, |_, _| 1u32.into_ret());
        let this = controller(&mut e, 0, false);
        let update = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        e.call(0x00a4_f0c0, &args![this, update]);
        assert!(calls(&e, INTERPOLATOR_UPDATE).is_empty());
        let this = controller(&mut e, 0, true);
        e.call(0x00a4_f0c0, &args![this, update]);
        assert_eq!(calls(&e, INTERPOLATOR_UPDATE).len(), 1);
        // Flag 0x20: the reset time, no DontDoUpdate.
        e.map(SCALED_TIME_RESET, 4);
        e.set_global(SCALED_TIME_RESET, f32::MIN.to_bits());
        let this = controller(&mut e, 0x20, false);
        e.call(0x00a4_f0c0, &args![this, update]);
        assert_eq!(calls(&e, DONT_DO_UPDATE).len(), 2);
        assert_eq!(e.mem.f32(this + SCALED_TIME), f32::MIN);
        assert_eq!(calls(&e, INTERPOLATOR_UPDATE)[1][1], f32::MIN.to_bits());
        // No interpolator at all.
        e.mem.set_u32(this + INTERPOLATOR, 0);
        e.call(0x00a4_f0c0, &args![this, update]);
        assert_eq!(calls(&e, INTERPOLATOR_UPDATE).len(), 2);
    }

    /// A stream whose binary streams transfer through `TRANSFER` (4 bytes,
    /// writing 7 into a read buffer), file version `version`.
    fn new_stream(e: &mut Engine, version: u32) -> (u32, u32) {
        e.register(TRANSFER, |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (4, 4, 1));
            e.mem.set_u32(a[1], 7);
            4u32.into_ret()
        });
        let binary = e.mem.alloc(0x10);
        e.mem.set_u32(binary + BINARY_STREAM_READ, TRANSFER);
        e.mem.set_u32(binary + BINARY_STREAM_WRITE, TRANSFER);
        let stream = e.mem.alloc(0x260);
        e.mem.set_u32(stream + STREAM_INPUT, binary);
        e.mem.set_u32(stream + STREAM_OUTPUT, binary);
        e.mem.set_u32(stream + STREAM_FILE_VERSION, version);
        (stream, binary)
    }

    #[test]
    fn load_binary_reads_the_index_and_an_old_link() {
        let mut e = Engine::new();
        e.register(BASE_LOAD_BINARY, |_, _| Ret::default());
        e.register(STREAM_READ_LINK_ID, |_, _| Ret::default());
        let (stream, binary) = new_stream(&mut e, 0x0a01_0067);
        let this = e.mem.alloc(0x48);
        e.call_log = Some(vec![]);
        e.call(0x00a4_f150, &args![this, stream]);
        assert_eq!(calls(&e, BASE_LOAD_BINARY), vec![vec![this, stream]]);
        assert_eq!(e.mem.u32(this + FLOATS_INDEX), 7);
        assert_eq!(e.mem.u32(binary + BINARY_STREAM_POSITION), 4);
        assert_eq!(calls(&e, STREAM_READ_LINK_ID), vec![vec![stream]]);
        e.mem
            .set_u32(stream + STREAM_FILE_VERSION, VERSION_10_1_0_104);
        e.call(0x00a4_f150, &args![this, stream]);
        assert_eq!(calls(&e, STREAM_READ_LINK_ID).len(), 1);
    }

    #[test]
    fn save_binary_writes_the_index() {
        let mut e = Engine::new();
        e.register(BASE_SAVE_BINARY, |_, _| Ret::default());
        let (stream, binary) = new_stream(&mut e, 0);
        let this = e.mem.alloc(0x48);
        e.call_log = Some(vec![]);
        e.call(0x00a4_f1a0, &args![this, stream]);
        assert_eq!(calls(&e, BASE_SAVE_BINARY), vec![vec![this, stream]]);
        let transfers = calls(&e, TRANSFER);
        assert_eq!(
            (transfers.len(), transfers[0][0], transfers[0][1]),
            (1, binary, this + FLOATS_INDEX)
        );
        assert_eq!(e.mem.u32(binary + BINARY_STREAM_POSITION), 4);
    }

    #[test]
    fn get_viewer_strings_adds_the_name_and_the_index() {
        let mut e = Engine::new();
        e.register(EXTRA_DATA_CONTROLLER_GET_VIEWER_STRINGS, |_, _| {
            Ret::default()
        });
        e.register(NI_GET_VIEWER_STRING, |_, _| 0x7100u32.into_ret());
        e.register(NI_GET_VIEWER_STRING_INT, |_, _| 0x7200u32.into_ret());
        e.register(STRINGS_GROW, |_, _| Ret::default());
        e.register(STRINGS_SET_AT, |e, a| {
            assert_eq!(e.mem.u32(a[2]), 0x7100 + 0x100 * a[1]);
            let size = e.mem.u16(a[0] + 0xa) + 1;
            e.mem.set_u16(a[0] + 0xa, size);
            Ret::default()
        });
        e.map(NI_FLOATS_EXTRA_DATA_CONTROLLER_RTTI, 4);
        e.mem.set_u32(NI_FLOATS_EXTRA_DATA_CONTROLLER_RTTI, 0x5151);
        let this = e.mem.alloc(0x48);
        e.mem.set_u32(this + FLOATS_INDEX, 0xffff_ffff);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 1);
        e.mem.set_u16(strings + 0xe, 2);
        e.call_log = Some(vec![]);
        e.call(0x00a4_f1e0, &args![this, strings]);
        assert_eq!(
            calls(&e, EXTRA_DATA_CONTROLLER_GET_VIEWER_STRINGS),
            vec![vec![this, strings]]
        );
        assert_eq!(calls(&e, NI_GET_VIEWER_STRING), vec![vec![0x5151]]);
        assert_eq!(
            calls(&e, NI_GET_VIEWER_STRING_INT),
            vec![vec![LABEL_FLOATS_INDEX, 0xffff_ffff]]
        );
        assert_eq!(calls(&e, STRINGS_GROW), vec![vec![strings, 3]]);
        assert_eq!(calls(&e, STRINGS_SET_AT).len(), 2);
    }
}
