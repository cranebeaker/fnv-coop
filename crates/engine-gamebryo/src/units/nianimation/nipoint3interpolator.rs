//! `NiAnimation/nipoint3interpolator.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiPoint3Interpolator::ms_RTTI` (the class's `NiRTTI` record).
const NI_POINT3_INTERPOLATOR_RTTI: u32 = 0x011f_3f70;

// Translated from 00a49640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPoint3Interpolator::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_point3_interpolator_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_POINT3_INTERPOLATOR_RTTI)
}

layout! {
    /// `NiPoint3Interpolator` (Xbox PDB), 0x20 bytes on PC (the same).
    pub struct NiPoint3Interpolator: 0x20 {
        /// `m_fLastTime` (Xbox PDB, from `NiInterpolator`).
        0x08 m_fLastTime: f32,
        /// `m_kPoint3Value` (Xbox PDB): an `NiPoint3`.
        0x0C m_kPoint3Value: Inline<NiPoint3>,
        /// `m_spPoint3Data` (Xbox PDB): `NiPointer<NiPosData>`.
        0x18 m_spPoint3Data: Ptr,
        /// `m_uiLastIdx` (Xbox PDB).
        0x1C m_uiLastIdx: u32,
    }
}

use crate::types::NiPoint3;

/// `NiPoint3Interpolator`'s vtable (RTTI `.?AVNiPoint3Interpolator@@`).
const NI_POINT3_INTERPOLATOR_VTABLE: u32 = 0x0109_881c;
/// The point the constructor stores as "no value" and `Update` tests for
/// (three floats at `011a9424`).
const NO_POINT: u32 = 0x011a_9424;
/// `00aa13e0` (cdecl, byte count): `NiNew`'s allocator.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `operator delete` with a size (`00aa1460`, cdecl: pointer, size).
const NI_DELETE: u32 = 0x00aa_1460;
/// `NiKeyBasedInterpolator::NiKeyBasedInterpolator` (`00a3ca10`, thiscall).
const KEY_BASED_CONSTRUCT: u32 = 0x00a3_ca10;
/// The `NiKeyBasedInterpolator` destructor (`00a3ca30`, thiscall).
const KEY_BASED_DESTRUCT: u32 = 0x00a3_ca30;
/// `NiKeyBasedInterpolator::LoadBinary` (`00aa0ac0`, thiscall, stream).
const KEY_BASED_LOAD_BINARY: u32 = 0x00aa_0ac0;
/// The `NiKeyBasedInterpolator` copy of the members (`00aa0ab0`, thiscall:
/// clone, cloning process).
const KEY_BASED_COPY_MEMBERS: u32 = 0x00aa_0ab0;
/// `00a59be0` (`NiColor::LoadBinary` in the engine map, thiscall, stream):
/// reads three floats.
const POINT_LOAD_BINARY: u32 = 0x00a5_9be0;
/// `NiStream::ResolveLinkID` (`00a64620`, thiscall).
const RESOLVE_LINK_ID: u32 = 0x00a6_4620;
/// `NiPosKey::GenInterp` (`00a27490`, cdecl: result, time, keys, key type,
/// key count, last index, key size).
const POS_KEY_GEN_INTERP: u32 = 0x00a2_7490;
/// `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

/// Releases a reference to `object` (non-null): `DeleteThis` (vtable +4)
/// when the count at +4 drops to zero.
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object + 4]).u32() == 0 {
        e.vcall(object, 4, &[]);
    }
}

/// Assigns `value` to the `NiPointer` at `slot` when it differs.
fn assign_pointer(e: &mut Engine, slot: u32, value: u32) {
    let old = e.mem.u32(slot);
    if old == value {
        return;
    }
    if old != 0 {
        release(e, old);
    }
    e.mem.set_u32(slot, value);
    if value != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![value + 4]);
    }
}

/// The member values a new interpolator gets after the base constructor:
/// the class vtable, the "no value" point, no data, index 0.
fn init_members(e: &mut Engine, object: u32) {
    e.mem.set_u32(object, NI_POINT3_INTERPOLATOR_VTABLE);
    for lane in 0..3 {
        let word = e.mem.u32(NO_POINT + 4 * lane);
        e.mem.set_u32(object + 0xc + 4 * lane, word);
    }
    e.mem.set_u32(object + 0x18, 0);
    e.mem.set_u32(object + 0x1c, 0);
}

/// Whether the point at `point` equals the "no value" point, lane by lane
/// as the exe's `FUCOMPP` tests (NaN never equal).
fn is_no_point(e: &mut Engine, point: u32) -> bool {
    (0..3).all(|lane| {
        let value = x87_load32(e.mem.u32(point + 4 * lane));
        let none = x87_load32(e.mem.u32(NO_POINT + 4 * lane));
        value == none
    })
}

/// Copies the three words of a point.
fn copy_point(e: &mut Engine, dest: u32, source: u32) {
    for lane in 0..3 {
        let word = e.mem.u32(source + 4 * lane);
        e.mem.set_u32(dest + 4 * lane, word);
    }
}

// Translated from 00a49a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPoint3Interpolator`'s create function (no function in Ghidra; read
/// from the bytes): allocates 0x20 bytes, runs `NiKeyBasedInterpolator`'s
/// constructor and sets the members (vtable, the "no value" point, no
/// data, index 0); null when the allocation fails.
pub fn fn_00a49a30(e: &mut Engine) -> Ptr<NiPoint3Interpolator> {
    let object = e.call(NI_ALLOCATE, &args![0x20u32]).u32();
    if object == 0 {
        return Ptr::new(0);
    }
    e.call(KEY_BASED_CONSTRUCT, &args![object]);
    init_members(e, object);
    Ptr::new(object)
}

// Translated from 00a49530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPoint3Interpolator::LoadBinary` (Xbox PDB name in the engine map):
/// the base's `LoadBinary`, the point value (`00a59be0`), then the data
/// pointer from the stream's next link (`NiStream::ResolveLinkID`),
/// assigned as an `NiPointer`.
pub fn ni_point3_interpolator_load_binary(
    e: &mut Engine,
    this: Ptr<NiPoint3Interpolator>,
    stream: Ptr,
) {
    e.call(KEY_BASED_LOAD_BINARY, &args![this, stream]);
    e.call(
        POINT_LOAD_BINARY,
        &args![
            this.byte_add(NiPoint3Interpolator::m_kPoint3Value.off),
            stream
        ],
    );
    let data = e.call(RESOLVE_LINK_ID, &args![stream]).u32();
    assign_pointer(
        e,
        this.addr() + NiPoint3Interpolator::m_spPoint3Data.off,
        data,
    );
}

// Translated from 00a49870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPoint3Interpolator::Update` (Xbox PDB name in the engine map): at the
/// last time, writes the cached point to `value` and answers whether it is a
/// real value (not the "no value" point). Otherwise interpolates the keys
/// of the data (`NiPosKey::GenInterp`, when it has keys) into the cached
/// point; when that is the "no value" point answers false, else writes it
/// to `value`, stores the time as the last time and answers true.
/// `_interp_target` is not read.
pub fn ni_point3_interpolator_update(
    e: &mut Engine,
    this: Ptr<NiPoint3Interpolator>,
    time: f32,
    _interp_target: u32,
    value: Ptr,
) -> bool {
    let t = this.addr();
    let point = t + NiPoint3Interpolator::m_kPoint3Value.off;
    let last_time = x87_load32(e.mem.u32(t + 8));
    let time_bits = x87_quiet_bits(time.to_bits());
    if x87_load32(time.to_bits()) == last_time {
        copy_point(e, value.addr(), point);
        return !is_no_point(e, point);
    }
    let data = e.get(this, NiPoint3Interpolator::m_spPoint3Data);
    if data.addr() != 0 {
        let d = data.addr();
        let count = e.mem.u32(d + 8);
        let key_size = e.mem.u8(d + 0x14) as u32;
        let key_type = e.mem.u32(d + 0x10);
        let keys = e.mem.u32(d + 0xc);
        if count != 0 {
            e.with_stack(0xc, |e, result| {
                let interpolated = e
                    .call(
                        POS_KEY_GEN_INTERP,
                        &args![
                            result,
                            time_bits,
                            keys,
                            key_type,
                            count,
                            t + NiPoint3Interpolator::m_uiLastIdx.off,
                            key_size
                        ],
                    )
                    .u32();
                copy_point(e, point, interpolated);
            });
        }
    }
    if is_no_point(e, point) {
        return false;
    }
    copy_point(e, value.addr(), point);
    e.mem.set_u32(t + 8, time_bits);
    true
}

// Translated from 00a499a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`NiPoint3Interpolator::CopyMembers` by its
/// body and callers): the base's copy (`00aa0ab0`), then the point, the
/// data pointer (as an `NiPointer`) and the last index into `clone`.
pub fn fn_00a499a0(
    e: &mut Engine,
    this: Ptr<NiPoint3Interpolator>,
    clone: Ptr<NiPoint3Interpolator>,
    cloning: Ptr,
) {
    let (t, c) = (this.addr(), clone.addr());
    e.call(KEY_BASED_COPY_MEMBERS, &args![this, clone, cloning]);
    copy_point(e, c + 0xc, t + 0xc);
    let data = e.mem.u32(t + 0x18);
    assign_pointer(e, c + 0x18, data);
    let index = e.mem.u32(t + 0x1c);
    e.mem.set_u32(c + 0x1c, index);
}

// Translated from 00a49ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPoint3Interpolator`'s scalar deleting destructor (Xbox PDB name in the
/// engine map): releases the data pointer, runs the base destructor
/// (`00a3ca30`) and frees the 0x20 bytes when bit 0 of `flags` is set.
/// Returns `this`.
pub fn ni_point3_interpolator_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiPoint3Interpolator>,
    flags: u32,
) -> Ptr<NiPoint3Interpolator> {
    let data = e.get(this, NiPoint3Interpolator::m_spPoint3Data).addr();
    if data != 0 {
        release(e, data);
    }
    e.call(KEY_BASED_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_DELETE, &args![this, 0x20u32]);
    }
    this
}

// Translated from 00a49b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPoint3Interpolator::CreateClone` (Xbox PDB name in the engine map): a
/// new interpolator (as `00a49a30` makes it) with the members copied
/// (`00a499a0`). When the allocation fails, the copy still runs with a null
/// clone (as the exe does) and null is returned.
pub fn ni_point3_interpolator_create_clone(
    e: &mut Engine,
    this: Ptr<NiPoint3Interpolator>,
    cloning: Ptr,
) -> Ptr<NiPoint3Interpolator> {
    let object = e.call(NI_ALLOCATE, &args![0x20u32]).u32();
    if object != 0 {
        e.call(KEY_BASED_CONSTRUCT, &args![object]);
        init_members(e, object);
    }
    fn_00a499a0(e, this, Ptr::new(object), cloning);
    Ptr::new(object)
}

/// `NiPosData::GuaranteeKeysAtStartAndEnd` (`00a48f70`, `thiscall` on the data, (start,
/// end)) and `NiPosData::GetSequenceData` (`00a49140`, `thiscall`, (start, end, &data
/// pointer)).
const POS_DATA_GUARANTEE_KEYS: u32 = 0x00a4_8f70;
const POS_DATA_GET_SEQUENCE_DATA: u32 = 0x00a4_9140;
/// `NiKeyBasedInterpolator::RegisterStreamables` (`00aa0ad0`; a bool), the base `SaveBinary`
/// `00a95ca0`, `NiPoint3`'s `SaveBinary` (`00a69780`, the engine map's `NiColor::SaveBinary`)
/// and the base `IsEqual` (`00c85800`; a bool), `NiInterpolator::GetSequenceInterpolator`
/// (`00a57050`, (start, end)) and `NiKeyBasedInterpolator::GetViewerStrings` (`00a3cbf0`):
/// all `thiscall`.
const KEY_BASED_REGISTER_STREAMABLES: u32 = 0x00aa_0ad0;
const KEY_BASED_SAVE_BINARY: u32 = 0x00a9_5ca0;
const POINT_SAVE_BINARY: u32 = 0x00a6_9780;
const INTERPOLATOR_IS_EQUAL: u32 = 0x00c8_5800;
const INTERPOLATOR_GET_SEQUENCE_INTERPOLATOR: u32 = 0x00a5_7050;
const KEY_BASED_GET_VIEWER_STRINGS: u32 = 0x00a3_cbf0;
/// `00439090` (`thiscall` on a point, the other's address): nonzero when the two differ.
const POINT_DIFFERS: u32 = 0x0043_9090;
/// `NiGetViewerString` (`cdecl`, the text), its pointer overload (`00a7d5a0`, `cdecl`,
/// (name, pointer)), `NiPoint3::GetViewerString` (`00a59af0`, `thiscall` on the point, the
/// name), the viewer-string array's `SetSize` (`0096ad30`) and `SetAt` (`0096ae90`);
/// `"m_kPoint3Value"` and `"m_spPoint3Data"`.
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const NI_GET_VIEWER_STRING_POINTER: u32 = 0x00a7_d5a0;
const POINT_GET_VIEWER_STRING: u32 = 0x00a5_9af0;
const STRINGS_SET_SIZE: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
const VALUE_NAME_TEXT: u32 = 0x0109_8924;
const DATA_NAME_TEXT: u32 = 0x0109_8914;

/// The data's word at `offset` (count +0x08, keys +0x0C, type +0x10), 0 without data.
fn data_word(e: &Engine, this: Ptr<NiPoint3Interpolator>, offset: u32) -> u32 {
    let data = e.get(this, NiPoint3Interpolator::m_spPoint3Data).addr();
    if data == 0 {
        return 0;
    }
    e.mem.u32(data + offset)
}

/// The inlined `Add` of the viewer-string array: `text` into the stack word at `slot`; a
/// full array grown to size + step first; then `SetAt` at the old size with the word's
/// address.
fn add_viewer_string(e: &mut Engine, strings: u32, slot: u32, text: u32) {
    let size = u32::from(e.mem.u16(strings + 0xa));
    e.mem.set_u32(slot, text);
    if size >= u32::from(e.mem.u16(strings + 8)) {
        let step = u32::from(e.mem.u16(strings + 0xe));
        e.call(STRINGS_SET_SIZE, &args![strings, step + size]);
    }
    e.call(STRINGS_SET_AT, &args![strings, size, slot]);
}

// Translated from 00a49410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPoint3Interpolator::GetKeyCount` (Xbox PDB; `thiscall`, the channel it does not read):
/// the data's key count, 0 without data.
pub fn ni_point3_interpolator_get_key_count(
    e: &mut Engine,
    this: Ptr<NiPoint3Interpolator>,
    _unused_1: u32,
) -> u32 {
    data_word(e, this, 8)
}

// Translated from 00a49420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPoint3Interpolator::GetKeyType` (Xbox PDB; `thiscall`, the channel it does not read):
/// the data's key type, 0 without data.
pub fn ni_point3_interpolator_get_key_type(
    e: &mut Engine,
    this: Ptr<NiPoint3Interpolator>,
    _unused_1: u32,
) -> u32 {
    data_word(e, this, 0x10)
}

// Translated from 00a49430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPoint3Interpolator::GetKeyArray` (Xbox PDB; `thiscall`, the channel it does not read):
/// the data's keys, 0 without data.
pub fn ni_point3_interpolator_get_key_array(
    e: &mut Engine,
    this: Ptr<NiPoint3Interpolator>,
    _unused_1: u32,
) -> u32 {
    data_word(e, this, 0xc)
}

// Translated from 00a49440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPoint3Interpolator::GetKeyStride` (Xbox PDB; `thiscall`, the channel it does not read;
/// the answer is the low byte): the data's key size, 0 without data.
pub fn ni_point3_interpolator_get_key_stride(
    e: &mut Engine,
    this: Ptr<NiPoint3Interpolator>,
    _unused_1: u32,
) -> u8 {
    let data = e.get(this, NiPoint3Interpolator::m_spPoint3Data).addr();
    if data == 0 {
        return 0;
    }
    e.mem.u8(data + 0x14)
}

// Translated from 00a49460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPoint3Interpolator::GetChannelPosed` (Xbox PDB; `thiscall`, the channel it does not
/// read): false with data; otherwise true unless the value is the "no value" point
/// (`011a9424`, lane by lane by `FUCOMPP`: a NaN is unequal).
pub fn ni_point3_interpolator_get_channel_posed(
    e: &mut Engine,
    this: Ptr<NiPoint3Interpolator>,
    _unused_1: u32,
) -> bool {
    if e.get(this, NiPoint3Interpolator::m_spPoint3Data).addr() != 0 {
        return false;
    }
    !is_no_point(e, this.addr() + 0xc)
}

// Translated from 00a494b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPoint3Interpolator::SetPoseValue` (Xbox PDB; `thiscall`, the point by value, `RET
/// 0xC`): the data released and cleared, the three words stored as the value.
pub fn ni_point3_interpolator_set_pose_value(
    e: &mut Engine,
    this: Ptr<NiPoint3Interpolator>,
    x: u32,
    y: u32,
    z: u32,
) {
    let data = e.get(this, NiPoint3Interpolator::m_spPoint3Data).addr();
    if data != 0 {
        release(e, data);
        e.set(this, NiPoint3Interpolator::m_spPoint3Data, Ptr::new(0));
    }
    let t = this.addr();
    e.mem.set_u32(t + 0xc, x);
    e.mem.set_u32(t + 0x10, y);
    e.mem.set_u32(t + 0x14, z);
}

// Translated from 00a49500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPoint3Interpolator::GuaranteeTimeRange` (Xbox PDB; `thiscall`, (start, end), `RET
/// 8`): with data, `NiPosData::GuaranteeKeysAtStartAndEnd` on it with the two times
/// (through the x87 stack).
pub fn ni_point3_interpolator_guarantee_time_range(
    e: &mut Engine,
    this: Ptr<NiPoint3Interpolator>,
    start: f32,
    end: f32,
) {
    let data = e.get(this, NiPoint3Interpolator::m_spPoint3Data).addr();
    if data != 0 {
        e.call(
            POS_DATA_GUARANTEE_KEYS,
            &args![
                data,
                x87_quiet_bits(start.to_bits()),
                x87_quiet_bits(end.to_bits())
            ],
        );
    }
}

// Translated from 00a49590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPoint3Interpolator::RegisterStreamables` (Xbox PDB; `thiscall`, the stream): false when
/// `NiKeyBasedInterpolator::RegisterStreamables` says so; otherwise the data's own (slot
/// +0x54) when present, and true.
pub fn ni_point3_interpolator_register_streamables(
    e: &mut Engine,
    this: Ptr<NiPoint3Interpolator>,
    stream: u32,
) -> bool {
    if !e
        .call(KEY_BASED_REGISTER_STREAMABLES, &args![this, stream])
        .bool()
    {
        return false;
    }
    let data = e.get(this, NiPoint3Interpolator::m_spPoint3Data).addr();
    if data != 0 {
        e.vcall(data, 0x54, &args![stream]);
    }
    true
}

// Translated from 00a495c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPoint3Interpolator::SaveBinary` (Xbox PDB; `thiscall`, the stream): the base
/// `SaveBinary` (`00a95ca0`), the value (`00a69780`), then the stream's `SaveLinkID` (slot
/// +0x30) of the data.
pub fn ni_point3_interpolator_save_binary(
    e: &mut Engine,
    this: Ptr<NiPoint3Interpolator>,
    stream: u32,
) {
    e.call(KEY_BASED_SAVE_BINARY, &args![this, stream]);
    e.call(POINT_SAVE_BINARY, &args![this.addr() + 0xc, stream]);
    let data = e.get(this, NiPoint3Interpolator::m_spPoint3Data).addr();
    e.vcall(stream, 0x30, &args![data]);
}

// Translated from 00a495f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiPoint3Interpolator(NiPosData*)` constructor (unnamed in the map; `thiscall`, the
/// data): `NiKeyBasedInterpolator`'s constructor, the vtable, the "no value" point, the data
/// stored and counted, index 0. Answers `this`.
pub fn fn_00a495f0(
    e: &mut Engine,
    this: Ptr<NiPoint3Interpolator>,
    data: u32,
) -> Ptr<NiPoint3Interpolator> {
    let t = this.addr();
    e.call(KEY_BASED_CONSTRUCT, &args![this]);
    e.mem.set_u32(t, NI_POINT3_INTERPOLATOR_VTABLE);
    copy_point(e, t + 0xc, NO_POINT);
    e.set(this, NiPoint3Interpolator::m_spPoint3Data, Ptr::new(data));
    if data != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![data + 4]);
    }
    e.set(this, NiPoint3Interpolator::m_uiLastIdx, 0);
    this
}

// Translated from 00a49650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPoint3Interpolator::NiPoint3Interpolator` (Xbox PDB; `thiscall`, the point by value,
/// `RET 0xC`): `NiKeyBasedInterpolator`'s constructor, the three words as the value, the
/// vtable, no data, index 0. Answers `this`.
pub fn ni_point3_interpolator_constructor(
    e: &mut Engine,
    this: Ptr<NiPoint3Interpolator>,
    x: u32,
    y: u32,
    z: u32,
) -> Ptr<NiPoint3Interpolator> {
    let t = this.addr();
    e.call(KEY_BASED_CONSTRUCT, &args![this]);
    e.mem.set_u32(t + 0xc, x);
    e.mem.set_u32(t + 0x10, y);
    e.mem.set_u32(t, NI_POINT3_INTERPOLATOR_VTABLE);
    e.mem.set_u32(t + 0x14, z);
    e.set(this, NiPoint3Interpolator::m_spPoint3Data, Ptr::new(0));
    e.set(this, NiPoint3Interpolator::m_uiLastIdx, 0);
    this
}

// Translated from 00a49690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPoint3Interpolator::Collapse` (Xbox PDB; `thiscall`): with data, no keys release the
/// data and set the "no value" point; otherwise the first key's point (key +4) becomes the
/// value and the data is released when there is one key, or when the type is 1 or 5 and
/// every other key's point equals the first (`00439090` says none differs).
pub fn ni_point3_interpolator_collapse(e: &mut Engine, this: Ptr<NiPoint3Interpolator>) {
    let t = this.addr();
    let data = e.get(this, NiPoint3Interpolator::m_spPoint3Data).addr();
    if data == 0 {
        return;
    }
    let size = u32::from(e.mem.u8(data + 0x14));
    let key_type = e.mem.u32(data + 0x10);
    let count = e.mem.u32(data + 8);
    let keys = e.mem.u32(data + 0xc);
    if count == 0 {
        release(e, data);
        e.set(this, NiPoint3Interpolator::m_spPoint3Data, Ptr::new(0));
        copy_point(e, t + 0xc, NO_POINT);
        return;
    }
    // The first key's point, kept in a local.
    e.with_stack(0xc, |e, first| {
        let first = first.addr();
        copy_point(e, first, keys.wrapping_add(4));
        if count != 1 {
            if key_type != 1 && key_type != 5 {
                return;
            }
            let mut same = true;
            let mut index = 1u32;
            while index < count {
                let key = size.wrapping_mul(index).wrapping_add(keys).wrapping_add(4);
                if e.call(POINT_DIFFERS, &args![key, first]).bool() {
                    same = false;
                }
                index += 1;
                if !same {
                    return;
                }
            }
            if !same {
                return;
            }
        }
        let data = e.get(this, NiPoint3Interpolator::m_spPoint3Data).addr();
        if data != 0 {
            release(e, data);
            e.set(this, NiPoint3Interpolator::m_spPoint3Data, Ptr::new(0));
        }
        copy_point(e, t + 0xc, first);
    });
}

// Translated from 00a497b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPoint3Interpolator::GetSequenceInterpolator` (Xbox PDB; `thiscall`, (start, end), `RET
/// 8`): `NiInterpolator::GetSequenceInterpolator` makes the new interpolator; with data,
/// `NiPosData::GetSequenceData` for the times goes into a local smart pointer, which is
/// assigned (counted) to the new interpolator's data, its `m_uiLastIdx` cleared, and the
/// local released. Answers the new one.
pub fn ni_point3_interpolator_get_sequence_interpolator(
    e: &mut Engine,
    this: Ptr<NiPoint3Interpolator>,
    start: f32,
    end: f32,
) -> u32 {
    let start = x87_quiet_bits(start.to_bits());
    let end = x87_quiet_bits(end.to_bits());
    let sequence = e
        .call(
            INTERPOLATOR_GET_SEQUENCE_INTERPOLATOR,
            &args![this, start, end],
        )
        .u32();
    let data = e.get(this, NiPoint3Interpolator::m_spPoint3Data).addr();
    if data == 0 {
        return sequence;
    }
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), 0);
        e.call(POS_DATA_GET_SEQUENCE_DATA, &args![data, start, end, local]);
        let copy = e.mem.u32(local.addr());
        assign_pointer(e, sequence.wrapping_add(0x18), copy);
        e.mem.set_u32(sequence.wrapping_add(0x1c), 0);
        let copy = e.mem.u32(local.addr());
        if copy != 0 {
            release(e, copy);
        }
    });
    sequence
}

// Translated from 00a49a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPoint3Interpolator::IsEqual` (Xbox PDB; `thiscall`, the other object): the base
/// `IsEqual` (`00c85800`), the values not differing (`00439090`), and the data in both or
/// neither, with both equal by their own `IsEqual` (slot +0x5C).
pub fn ni_point3_interpolator_is_equal(
    e: &mut Engine,
    this: Ptr<NiPoint3Interpolator>,
    other: Ptr<NiPoint3Interpolator>,
) -> bool {
    if !e.call(INTERPOLATOR_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    if e.call(POINT_DIFFERS, &args![this.addr() + 0xc, other.addr() + 0xc])
        .bool()
    {
        return false;
    }
    let own = e.get(this, NiPoint3Interpolator::m_spPoint3Data).addr();
    let theirs = e.get(other, NiPoint3Interpolator::m_spPoint3Data).addr();
    if (own == 0) != (theirs == 0) {
        return false;
    }
    if own == 0 {
        return true;
    }
    e.vcall(own, 0x5c, &args![theirs]).bool()
}

// Translated from 00a49bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPoint3Interpolator::GetViewerStrings` (Xbox PDB; `thiscall`, the viewer-string array):
/// `NiKeyBasedInterpolator::GetViewerStrings`, then the class name (the first word of the
/// `NiRTTI` record at `011f3f70`), the value's string (`"m_kPoint3Value"`) and the data
/// pointer's (`"m_spPoint3Data"`) appended.
pub fn ni_point3_interpolator_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<NiPoint3Interpolator>,
    strings: u32,
) {
    e.call(KEY_BASED_GET_VIEWER_STRINGS, &args![this, strings]);
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        let class_name = e.global::<u32>(NI_POINT3_INTERPOLATOR_RTTI);
        let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
        add_viewer_string(e, strings, slot, text);
        let text = e
            .call(
                POINT_GET_VIEWER_STRING,
                &args![this.addr() + 0xc, VALUE_NAME_TEXT],
            )
            .u32();
        add_viewer_string(e, strings, slot, text);
        let data = e.get(this, NiPoint3Interpolator::m_spPoint3Data).addr();
        let text = e
            .call(NI_GET_VIEWER_STRING_POINTER, &args![DATA_NAME_TEXT, data])
            .u32();
        add_viewer_string(e, strings, slot, text);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a49640, ni_point3_interpolator_get_rtti() -> Ptr),
        entry!(
            0x00a4_9410,
            ni_point3_interpolator_get_key_count(Ptr<NiPoint3Interpolator>, u32) -> u32
        ),
        entry!(
            0x00a4_9420,
            ni_point3_interpolator_get_key_type(Ptr<NiPoint3Interpolator>, u32) -> u32
        ),
        entry!(
            0x00a4_9430,
            ni_point3_interpolator_get_key_array(Ptr<NiPoint3Interpolator>, u32) -> u32
        ),
        entry!(
            0x00a4_9440,
            ni_point3_interpolator_get_key_stride(Ptr<NiPoint3Interpolator>, u32) -> u8
        ),
        entry!(
            0x00a4_9460,
            ni_point3_interpolator_get_channel_posed(Ptr<NiPoint3Interpolator>, u32) -> bool
        ),
        entry!(
            0x00a4_94b0,
            ni_point3_interpolator_set_pose_value(Ptr<NiPoint3Interpolator>, u32, u32, u32)
        ),
        entry!(
            0x00a4_9500,
            ni_point3_interpolator_guarantee_time_range(Ptr<NiPoint3Interpolator>, f32, f32)
        ),
        entry!(
            0x00a4_9590,
            ni_point3_interpolator_register_streamables(Ptr<NiPoint3Interpolator>, u32) -> bool
        ),
        entry!(
            0x00a4_95c0,
            ni_point3_interpolator_save_binary(Ptr<NiPoint3Interpolator>, u32)
        ),
        entry!(
            0x00a4_95f0,
            fn_00a495f0(Ptr<NiPoint3Interpolator>, u32) -> Ptr<NiPoint3Interpolator>
        ),
        entry!(
            0x00a4_9650,
            ni_point3_interpolator_constructor(
                Ptr<NiPoint3Interpolator>,
                u32,
                u32,
                u32,
            ) -> Ptr<NiPoint3Interpolator>
        ),
        entry!(
            0x00a4_9690,
            ni_point3_interpolator_collapse(Ptr<NiPoint3Interpolator>)
        ),
        entry!(
            0x00a4_97b0,
            ni_point3_interpolator_get_sequence_interpolator(
                Ptr<NiPoint3Interpolator>,
                f32,
                f32,
            ) -> u32
        ),
        entry!(
            0x00a4_9a80,
            ni_point3_interpolator_is_equal(
                Ptr<NiPoint3Interpolator>,
                Ptr<NiPoint3Interpolator>,
            ) -> bool
        ),
        entry!(
            0x00a4_9bb0,
            ni_point3_interpolator_get_viewer_strings(Ptr<NiPoint3Interpolator>, u32)
        ),
        entry!(0x00a4_9a30, fn_00a49a30() -> Ptr<NiPoint3Interpolator>),
        entry!(
            0x00a4_9530,
            ni_point3_interpolator_load_binary(Ptr<NiPoint3Interpolator>, Ptr)
        ),
        entry!(
            0x00a4_9870,
            ni_point3_interpolator_update(Ptr<NiPoint3Interpolator>, f32, u32, Ptr) -> bool
        ),
        entry!(
            0x00a4_99a0,
            fn_00a499a0(Ptr<NiPoint3Interpolator>, Ptr<NiPoint3Interpolator>, Ptr)
        ),
        entry!(
            0x00a4_9ae0,
            ni_point3_interpolator_scalar_deleting_destructor(
                Ptr<NiPoint3Interpolator>,
                u32,
            )
                -> Ptr<NiPoint3Interpolator>
        ),
        entry!(
            0x00a4_9b30,
            ni_point3_interpolator_create_clone(
                Ptr<NiPoint3Interpolator>,
                Ptr,
            ) -> Ptr<NiPoint3Interpolator>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_point3_interpolator_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a49640, &args![]).u32(),
            NI_POINT3_INTERPOLATOR_RTTI
        );
    }

    /// Reference counting through the import stand-ins: plain decrement and
    /// increment of the count at the address.
    fn refcount_doubles(e: &mut Engine) {
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], v);
            Ret {
                eax: v,
                ..Ret::default()
            }
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            Ret {
                eax: v,
                ..Ret::default()
            }
        });
        e.register(0x7000_0004, |_, _| Ret::default());
        e.put_vtable(0x9100_0000, &[0, 0x7000_0004]);
    }

    /// A reference-counted object with `count` references.
    fn counted(e: &mut Engine, count: u32) -> u32 {
        let object = e.mem.alloc(0x18);
        e.mem.set_u32(object, 0x9100_0000);
        e.mem.set_u32(object + 4, count);
        object
    }

    fn no_point(e: &mut Engine) {
        e.map(NO_POINT, 0xc);
        for lane in 0..3 {
            e.mem.set_f32(NO_POINT + 4 * lane, -3.0e38);
        }
    }

    fn deleted(e: &Engine) -> usize {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == 0x7000_0004)
            .count()
    }

    #[test]
    fn fn_00a49a30_creates_an_empty_interpolator() {
        let mut e = Engine::new();
        no_point(&mut e);
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        e.register(KEY_BASED_CONSTRUCT, |_, _| Ret::default());
        assert_eq!(e.call(0x00a4_9a30, &[]).u32(), 0);
        e.register(NI_ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        let object = e.call(0x00a4_9a30, &[]).u32();
        assert_eq!(e.mem.u32(object), NI_POINT3_INTERPOLATOR_VTABLE);
        assert_eq!(e.mem.f32(object + 0x10), -3.0e38);
        assert_eq!(e.mem.u32(object + 0x18), 0);
        assert_eq!(e.mem.u32(object + 0x1c), 0);
    }

    #[test]
    fn load_binary_reads_the_point_and_links_the_data() {
        let mut e = Engine::new();
        refcount_doubles(&mut e);
        e.register(KEY_BASED_LOAD_BINARY, |_, _| Ret::default());
        e.register(POINT_LOAD_BINARY, |_, _| Ret::default());
        let data = counted(&mut e, 0);
        e.register_double(RESOLVE_LINK_ID, move |_, _| Ret {
            eax: data,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x20);
        let old = counted(&mut e, 1);
        e.mem.set_u32(this + 0x18, old);
        e.call_log = Some(vec![]);
        e.call(0x00a4_9530, &args![this, 0x5000u32]);
        assert_eq!(e.mem.u32(this + 0x18), data);
        assert_eq!(e.mem.u32(data + 4), 1);
        assert_eq!(deleted(&e), 1, "the old data had its last reference");
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (KEY_BASED_LOAD_BINARY, vec![this, 0x5000]));
        assert_eq!(log[2], (POINT_LOAD_BINARY, vec![this + 0xc, 0x5000]));
        // The same data again changes nothing.
        e.call(0x00a4_9530, &args![this, 0x5000u32]);
        assert_eq!(e.mem.u32(data + 4), 1);
    }

    #[test]
    fn update_caches_and_interpolates() {
        let mut e = Engine::new();
        no_point(&mut e);
        e.register(POS_KEY_GEN_INTERP, |e, a| {
            let time = f32::from_bits(a[1]);
            for lane in 0..3 {
                e.mem.set_f32(a[0] + 4 * lane, time * (lane + 1) as f32);
            }
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let this = e.mem.alloc(0x20);
        for lane in 0..3 {
            e.mem.set_f32(this + 0xc + 4 * lane, -3.0e38);
        }
        e.mem.set_f32(this + 8, 1.0);
        let value = e.mem.alloc(0xc);
        let update =
            |e: &mut Engine, time: f32| e.call(0x00a4_9870, &args![this, time, 0u32, value]).bool();
        assert!(!update(&mut e, 1.0), "cached, no value");
        assert_eq!(e.mem.f32(value), -3.0e38);
        assert!(!update(&mut e, 2.0), "no data");
        assert_eq!(e.mem.f32(this + 8), 1.0);

        let data = e.mem.alloc(0x18);
        e.mem.set_u32(this + 0x18, data);
        assert!(!update(&mut e, 2.0), "no keys");
        e.mem.set_u32(data + 8, 3);
        e.mem.set_u32(data + 0xc, 0x4400);
        e.mem.set_u32(data + 0x10, 1);
        e.mem.set_u8(data + 0x14, 0x10);
        e.call_log = Some(vec![]);
        assert!(update(&mut e, 2.0));
        assert_eq!(e.mem.f32(value + 8), 6.0);
        assert_eq!(e.mem.f32(this + 8), 2.0);
        let log = e.call_log.take().unwrap();
        let interp: Vec<_> = log.iter().filter(|c| c.0 == POS_KEY_GEN_INTERP).collect();
        assert_eq!(
            interp[0].1[1..].to_vec(),
            vec![2.0f32.to_bits(), 0x4400, 1, 3, this + 0x1c, 0x10]
        );
        e.mem.set_f32(value + 8, 0.0);
        assert!(update(&mut e, 2.0), "cached value");
        assert_eq!(e.mem.f32(value + 8), 6.0);
    }

    #[test]
    fn fn_00a499a0_copies_the_point_data_and_index() {
        let mut e = Engine::new();
        refcount_doubles(&mut e);
        e.register(KEY_BASED_COPY_MEMBERS, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        let clone = e.mem.alloc(0x20);
        let data = counted(&mut e, 1);
        e.mem.set_u32(this + 0x18, data);
        e.mem.set_f32(this + 0x14, 4.5);
        e.mem.set_u32(this + 0x1c, 7);
        e.call_log = Some(vec![]);
        e.call(0x00a4_99a0, &args![this, clone, 0x6000u32]);
        assert_eq!(
            e.call_log.take().unwrap()[1],
            (KEY_BASED_COPY_MEMBERS, vec![this, clone, 0x6000])
        );
        assert_eq!(e.mem.f32(clone + 0x14), 4.5);
        assert_eq!(e.mem.u32(clone + 0x18), data);
        assert_eq!(e.mem.u32(data + 4), 2);
        assert_eq!(e.mem.u32(clone + 0x1c), 7);
    }

    #[test]
    fn scalar_deleting_destructor_releases_and_frees_on_bit_0() {
        let mut e = Engine::new();
        refcount_doubles(&mut e);
        e.register(KEY_BASED_DESTRUCT, |_, _| Ret::default());
        e.register(NI_DELETE, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        let data = counted(&mut e, 2);
        e.mem.set_u32(this + 0x18, data);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a4_9ae0, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(data + 4), 1);
        assert!(!e.call_log.take().unwrap().iter().any(|c| c.0 == NI_DELETE));
        e.mem.set_u32(this + 0x18, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a4_9ae0, &args![this, 1u32]);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(KEY_BASED_DESTRUCT, vec![this])));
        assert!(log.contains(&(NI_DELETE, vec![this, 0x20])));
    }

    #[test]
    fn create_clone_builds_and_copies() {
        let mut e = Engine::new();
        no_point(&mut e);
        refcount_doubles(&mut e);
        e.register(NI_ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(KEY_BASED_CONSTRUCT, |_, _| Ret::default());
        e.register(KEY_BASED_COPY_MEMBERS, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        e.mem.set_f32(this + 0xc, 1.25);
        let clone = e.call(0x00a4_9b30, &args![this, 0x6000u32]).u32();
        assert_eq!(e.mem.u32(clone), NI_POINT3_INTERPOLATOR_VTABLE);
        assert_eq!(e.mem.f32(clone + 0xc), 1.25);
        assert_eq!(e.mem.f32(clone + 0x10), 0.0);
    }
}

#[cfg(test)]
mod n005_tests {
    use super::*;

    type P = NiPoint3Interpolator;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn counting(e: &mut Engine) {
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]) - 1;
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(0x0099_0004, |_, _| Ret::default());
        e.register(0x0099_0054, |_, _| Ret::default());
        e.register(0x0099_005c, |_, a| (a[1] & 1 == 0).into_ret());
    }

    /// Position data (counted once; vtable slots +4, +0x54 and +0x5C doubled).
    fn pos_data(e: &mut Engine, count: u32, key_type: u32, keys: u32, size: u8) -> u32 {
        let data = e.mem.alloc(0x18);
        let table = e.mem.alloc(0x60);
        for slot in [4u32, 0x54, 0x5c] {
            e.mem.set_u32(table + slot, 0x0099_0000 + slot);
        }
        e.mem.set_u32(data, table);
        e.mem.set_u32(data + 4, 1);
        e.mem.set_u32(data + 8, count);
        e.mem.set_u32(data + 0xc, keys);
        e.mem.set_u32(data + 0x10, key_type);
        e.mem.set_u8(data + 0x14, size);
        data
    }

    /// The "no value" point set to (7, 8, 9).
    fn no_point(e: &mut Engine) {
        e.map(NO_POINT & !0xfff, 0x1000);
        for (i, v) in [7.0f32, 8.0, 9.0].iter().enumerate() {
            e.mem.set_f32(NO_POINT + 4 * i as u32, *v);
        }
    }

    fn point(e: &Engine, at: u32) -> Vec<f32> {
        (0..3).map(|i| e.mem.f32(at + 4 * i)).collect()
    }

    #[test]
    fn key_accessors_read_the_data() {
        let mut e = Engine::new();
        let this = e.new_object::<P>();
        for address in [0x00a4_9410u32, 0x00a4_9420, 0x00a4_9430, 0x00a4_9440] {
            assert_eq!(e.call(address, &[this.addr(), 0]).u32(), 0);
        }
        let data = pos_data(&mut e, 3, 2, 0x7000, 0x10);
        e.set(this, P::m_spPoint3Data, Ptr::new(data));
        assert_eq!(e.call(0x00a4_9410, &[this.addr(), 0]).u32(), 3);
        assert_eq!(e.call(0x00a4_9420, &[this.addr(), 0]).u32(), 2);
        assert_eq!(e.call(0x00a4_9430, &[this.addr(), 0]).u32(), 0x7000);
        assert_eq!(e.call(0x00a4_9440, &[this.addr(), 0]).u32(), 0x10);
    }

    #[test]
    fn channel_posed_unless_no_value_or_data() {
        let mut e = Engine::new();
        no_point(&mut e);
        let this = e.new_object::<P>();
        for (i, v) in [7.0f32, 8.0, 9.0].iter().enumerate() {
            e.mem.set_f32(this.addr() + 0xc + 4 * i as u32, *v);
        }
        assert!(!e.call(0x00a4_9460, &[this.addr(), 0]).bool());
        e.mem.set_f32(this.addr() + 0x14, 1.0);
        assert!(e.call(0x00a4_9460, &[this.addr(), 0]).bool());
        // A NaN lane is never equal.
        e.mem.set_f32(this.addr() + 0x14, 9.0);
        e.mem.set_u32(this.addr() + 0x10, 0x7fc0_0000);
        assert!(e.call(0x00a4_9460, &[this.addr(), 0]).bool());
        e.set(this, P::m_spPoint3Data, Ptr::new(0x5000));
        assert!(!e.call(0x00a4_9460, &[this.addr(), 0]).bool());
    }

    #[test]
    fn set_pose_value_drops_the_data() {
        let mut e = Engine::new();
        counting(&mut e);
        let this = e.new_object::<P>();
        let data = pos_data(&mut e, 0, 0, 0, 0);
        e.set(this, P::m_spPoint3Data, Ptr::new(data));
        e.call_log = Some(vec![]);
        e.call(0x00a4_94b0, &[this.addr(), 1, 2, 3]);
        assert_eq!(e.get(this, P::m_spPoint3Data).addr(), 0);
        assert_eq!(calls_to(&e, 0x0099_0004), vec![vec![data]]);
        let words: Vec<u32> = (0..3)
            .map(|i| e.mem.u32(this.addr() + 0xc + 4 * i))
            .collect();
        assert_eq!(words, [1, 2, 3]);
        // Without data only the value changes.
        e.call(0x00a4_94b0, &[this.addr(), 4, 5, 6]);
        assert_eq!(e.mem.u32(this.addr() + 0x14), 6);
        assert_eq!(calls_to(&e, 0x0099_0004).len(), 1);
    }

    #[test]
    fn guarantee_time_range_forwards_to_the_data() {
        let mut e = Engine::new();
        e.register(POS_DATA_GUARANTEE_KEYS, |_, _| Ret::default());
        let this = e.new_object::<P>();
        e.call_log = Some(vec![]);
        e.call(0x00a4_9500, &[this.addr(), 0, 0]);
        assert!(calls_to(&e, POS_DATA_GUARANTEE_KEYS).is_empty());
        e.set(this, P::m_spPoint3Data, Ptr::new(0x5000));
        e.call(0x00a4_9500, &[this.addr(), 1.0f32.to_bits(), 0xff80_0001]);
        assert_eq!(
            calls_to(&e, POS_DATA_GUARANTEE_KEYS),
            vec![vec![0x5000, 1.0f32.to_bits(), 0xffc0_0001]]
        );
    }

    #[test]
    fn register_streamables_and_save_binary_reach_the_data() {
        let mut e = Engine::new();
        counting(&mut e);
        e.register(KEY_BASED_REGISTER_STREAMABLES, |_, a| {
            (a[1] == 0x9000).into_ret()
        });
        e.register(KEY_BASED_SAVE_BINARY, |_, _| Ret::default());
        e.register(POINT_SAVE_BINARY, |_, _| Ret::default());
        let this = e.new_object::<P>();
        let data = pos_data(&mut e, 0, 0, 0, 0);
        e.set(this, P::m_spPoint3Data, Ptr::new(data));
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00a4_9590, &[this.addr(), 0x9001]).bool());
        assert!(e.call(0x00a4_9590, &[this.addr(), 0x9000]).bool());
        assert_eq!(calls_to(&e, 0x0099_0054), vec![vec![data, 0x9000]]);
        let stream = e.mem.alloc(0x300);
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(table + 0x30, 0x0099_0030);
        e.mem.set_u32(stream, table);
        e.register(0x0099_0030, |_, _| Ret::default());
        e.call(0x00a4_95c0, &[this.addr(), stream]);
        assert_eq!(
            calls_to(&e, POINT_SAVE_BINARY),
            vec![vec![this.addr() + 0xc, stream]]
        );
        assert_eq!(calls_to(&e, 0x0099_0030), vec![vec![stream, data]]);
    }

    #[test]
    fn constructors_set_the_members() {
        let mut e = Engine::new();
        counting(&mut e);
        no_point(&mut e);
        e.register(KEY_BASED_CONSTRUCT, |_, _| Ret::default());
        let this = Ptr::<P>::new(e.mem.alloc(0x20));
        e.mem.set_u32(this.addr() + 0x1c, 5);
        let data = pos_data(&mut e, 0, 0, 0, 0);
        assert_eq!(e.call(0x00a4_95f0, &[this.addr(), data]).u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), NI_POINT3_INTERPOLATOR_VTABLE);
        assert_eq!(point(&e, this.addr() + 0xc), [7.0, 8.0, 9.0]);
        assert_eq!(e.get(this, P::m_spPoint3Data).addr(), data);
        assert_eq!(e.mem.u32(data + 4), 2);
        assert_eq!(e.get(this, P::m_uiLastIdx), 0);
        let other = Ptr::<P>::new(e.mem.alloc(0x20));
        e.mem.set_u32(other.addr() + 0x18, 0x1234);
        e.call(0x00a4_9650, &[other.addr(), 1, 2, 3]);
        let words: Vec<u32> = (0..3)
            .map(|i| e.mem.u32(other.addr() + 0xc + 4 * i))
            .collect();
        assert_eq!(words, [1, 2, 3]);
        assert_eq!(e.get(other, P::m_spPoint3Data).addr(), 0);
        assert_eq!(e.mem.u32(other.addr()), NI_POINT3_INTERPOLATOR_VTABLE);
    }

    /// Keys of 0x10 bytes whose points are (v, v, v).
    fn keys(e: &mut Engine, values: &[f32]) -> u32 {
        let keys = e.mem.alloc(0x10 * values.len() as u32);
        for (i, v) in values.iter().enumerate() {
            for word in 0..3 {
                e.mem.set_f32(keys + 0x10 * i as u32 + 4 + 4 * word, *v);
            }
        }
        keys
    }

    #[test]
    fn collapse_takes_the_constant_key() {
        let mut e = Engine::new();
        counting(&mut e);
        no_point(&mut e);
        e.register(POINT_DIFFERS, |e, a| {
            (e.mem.u32(a[0]) != e.mem.u32(a[1])).into_ret()
        });
        let this = e.new_object::<P>();
        let data = pos_data(&mut e, 0, 1, 0, 0x10);
        e.set(this, P::m_spPoint3Data, Ptr::new(data));
        e.call(0x00a4_9690, &[this.addr()]);
        assert_eq!(e.get(this, P::m_spPoint3Data).addr(), 0);
        assert_eq!(point(&e, this.addr() + 0xc), [7.0, 8.0, 9.0]);
        // One key (any type): collapsed.
        let one = keys(&mut e, &[2.0]);
        let data = pos_data(&mut e, 1, 4, one, 0x10);
        e.set(this, P::m_spPoint3Data, Ptr::new(data));
        e.call(0x00a4_9690, &[this.addr()]);
        assert_eq!(e.get(this, P::m_spPoint3Data).addr(), 0);
        assert_eq!(point(&e, this.addr() + 0xc), [2.0; 3]);
        // Equal linear keys collapse; a different one, or another type, keeps the data.
        let same = keys(&mut e, &[3.0, 3.0]);
        let data = pos_data(&mut e, 2, 5, same, 0x10);
        e.set(this, P::m_spPoint3Data, Ptr::new(data));
        e.call(0x00a4_9690, &[this.addr()]);
        assert_eq!(e.get(this, P::m_spPoint3Data).addr(), 0);
        assert_eq!(point(&e, this.addr() + 0xc), [3.0; 3]);
        let mixed = keys(&mut e, &[3.0, 4.0]);
        let data = pos_data(&mut e, 2, 1, mixed, 0x10);
        e.set(this, P::m_spPoint3Data, Ptr::new(data));
        e.call(0x00a4_9690, &[this.addr()]);
        assert_eq!(e.get(this, P::m_spPoint3Data).addr(), data);
        let data = pos_data(&mut e, 2, 2, same, 0x10);
        e.set(this, P::m_spPoint3Data, Ptr::new(data));
        e.call(0x00a4_9690, &[this.addr()]);
        assert_eq!(e.get(this, P::m_spPoint3Data).addr(), data);
    }

    #[test]
    fn sequence_interpolator_takes_the_sequence_data() {
        let mut e = Engine::new();
        counting(&mut e);
        let sequence = e.new_object::<P>();
        e.set(sequence, P::m_uiLastIdx, 4);
        let new = pos_data(&mut e, 0, 0, 0, 0);
        e.register_double(INTERPOLATOR_GET_SEQUENCE_INTERPOLATOR, move |_, _| {
            sequence.into_ret()
        });
        e.register_double(POS_DATA_GET_SEQUENCE_DATA, move |e, a| {
            assert_eq!(e.mem.u32(a[3]), 0);
            e.mem.set_u32(a[3], new);
            Ret::default()
        });
        let this = e.new_object::<P>();
        let (start, end) = (1.0f32.to_bits(), 2.0f32.to_bits());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00a4_97b0, &[this.addr(), start, end]).u32(),
            sequence.addr()
        );
        assert!(calls_to(&e, POS_DATA_GET_SEQUENCE_DATA).is_empty());
        e.set(this, P::m_spPoint3Data, Ptr::new(0x6000));
        e.call(0x00a4_97b0, &[this.addr(), start, end]);
        assert_eq!(
            calls_to(&e, POS_DATA_GET_SEQUENCE_DATA)[0][..3],
            [0x6000, start, end]
        );
        assert_eq!(e.get(sequence, P::m_spPoint3Data).addr(), new);
        assert_eq!(e.get(sequence, P::m_uiLastIdx), 0);
        assert_eq!(e.mem.u32(new + 4), 1);
    }

    #[test]
    fn is_equal_compares_value_and_data() {
        let mut e = Engine::new();
        counting(&mut e);
        e.register(INTERPOLATOR_IS_EQUAL, |_, _| true.into_ret());
        e.register(POINT_DIFFERS, |e, a| {
            (e.mem.u32(a[0]) != e.mem.u32(a[1])).into_ret()
        });
        let a = e.new_object::<P>();
        let b = e.new_object::<P>();
        let equal = |e: &mut Engine| e.call(0x00a4_9a80, &[a.addr(), b.addr()]).bool();
        assert!(equal(&mut e));
        e.mem.set_u32(b.addr() + 0xc, 1);
        assert!(!equal(&mut e));
        e.mem.set_u32(b.addr() + 0xc, 0);
        let data = pos_data(&mut e, 0, 0, 0, 0);
        e.set(a, P::m_spPoint3Data, Ptr::new(data));
        assert!(!equal(&mut e));
        e.set(b, P::m_spPoint3Data, Ptr::new(0x5000));
        assert!(equal(&mut e));
        e.set(b, P::m_spPoint3Data, Ptr::new(0x5001));
        assert!(!equal(&mut e));
        e.register(INTERPOLATOR_IS_EQUAL, |_, _| false.into_ret());
        e.set(b, P::m_spPoint3Data, Ptr::new(0x5000));
        assert!(!equal(&mut e));
    }

    #[test]
    fn get_viewer_strings_lists_class_value_and_data() {
        let mut e = Engine::new();
        e.map(NI_POINT3_INTERPOLATOR_RTTI & !0xfff, 0x1000);
        e.set_global(NI_POINT3_INTERPOLATOR_RTTI, 0x0444_0000u32);
        e.register(KEY_BASED_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(NI_GET_VIEWER_STRING, |_, _| 0x0600_0001u32.into_ret());
        e.register(POINT_GET_VIEWER_STRING, |_, _| 0x0600_0002u32.into_ret());
        e.register(NI_GET_VIEWER_STRING_POINTER, |_, _| {
            0x0600_0003u32.into_ret()
        });
        e.register(STRINGS_SET_SIZE, |_, _| Ret::default());
        e.register(STRINGS_SET_AT, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            let text = e.mem.u32(a[2]);
            e.mem.set_u32(base + 4 * a[1], text);
            e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            Ret::default()
        });
        let strings = e.mem.alloc(0x10);
        let base = e.mem.alloc(0x10);
        e.mem.set_u32(strings + 4, base);
        e.mem.set_u16(strings + 8, 4);
        let this = e.new_object::<P>();
        e.set(this, P::m_spPoint3Data, Ptr::new(0x5000));
        e.call_log = Some(vec![]);
        e.call(0x00a4_9bb0, &[this.addr(), strings]);
        let listed: Vec<u32> = (0..3).map(|i| e.mem.u32(base + 4 * i)).collect();
        assert_eq!(listed, [0x0600_0001, 0x0600_0002, 0x0600_0003]);
        assert_eq!(
            calls_to(&e, POINT_GET_VIEWER_STRING),
            vec![vec![this.addr() + 0xc, VALUE_NAME_TEXT]]
        );
        assert_eq!(
            calls_to(&e, NI_GET_VIEWER_STRING_POINTER),
            vec![vec![DATA_NAME_TEXT, 0x5000]]
        );
    }
}
