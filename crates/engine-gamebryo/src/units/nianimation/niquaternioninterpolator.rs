//! `NiAnimation/niquaternioninterpolator.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiQuaternionInterpolator::ms_RTTI` (the class's `NiRTTI` record).
const NI_QUATERNION_INTERPOLATOR_RTTI: u32 = 0x011f_3f60;

// Translated from 00a48ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiQuaternionInterpolator::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_quaternion_interpolator_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_QUATERNION_INTERPOLATOR_RTTI)
}

layout! {
    /// `NiQuaternionInterpolator` (Xbox PDB), 0x24 bytes on PC (the same).
    pub struct NiQuaternionInterpolator: 0x24 {
        /// `NiInterpolator::m_fLastTime` (Xbox PDB), float bits.
        0x08 m_fLastTime: u32,
        /// `m_kQuaternionValue` (Xbox PDB): `NiQuaternion`, four floats.
        0x0C m_kQuaternionValue: u32,
        /// `m_spQuaternionData` (Xbox PDB): `NiPointer<NiRotData>`.
        0x1C m_spQuaternionData: u32,
        /// `m_uiLastIdx` (Xbox PDB).
        0x20 m_uiLastIdx: u32,
    }
}

/// `NiQuaternionInterpolator`'s vtable.
const NI_QUATERNION_INTERPOLATOR_VTABLE: u32 = 0x0109_86f4;
/// The four floats at `011f4208`: the value `Collapse` gives an interpolator without keys,
/// the one a new interpolator starts with and the one `GetChannelPosed` and `Update`
/// compare the value with (through `00a92660`).
const QUATERNION_011F4208: u32 = 0x011f_4208;
/// `FLT_MAX` (`010986e8`) and `-FLT_MAX` (`01015f5c`) as floats.
const HIGHEST_FLOAT: u32 = 0x0109_86e8;
const LOWEST_FLOAT: u32 = 0x0101_5f5c;
/// `00a92660` (`thiscall` on a quaternion, the other's address; a bool) and `0064a290`
/// (the same form; nonzero when the two differ, as `IsEqual` and `Collapse` use it).
const QUATERNION_COMPARE_00A92660: u32 = 0x00a9_2660;
const QUATERNION_DIFFERS: u32 = 0x0064_a290;
/// `NiPosData::GuaranteeKeysAtStartAndEnd` (`00a48f70` in `niposdata.cpp`, which this
/// class calls on its rotation data; `thiscall`, (start, end)) and
/// `NiRotData::GetSequenceData` (`00a48280`, `thiscall`, (start, end, &data pointer)).
const GUARANTEE_KEYS: u32 = 0x00a4_8f70;
const ROT_DATA_GET_SEQUENCE_DATA: u32 = 0x00a4_8280;
/// `NiRotKey::GenInterp` (`00a28740`, `cdecl`, (result, time, keys, key type, count, &last
/// index, key size); answers the result; it reads the low byte of the key size word).
const ROT_KEY_GEN_INTERP: u32 = 0x00a2_8740;
/// The base `SaveBinary` `00a95ca0`, `NiQuaternion::SaveBinary` (`00a6e4a0`), the base
/// `IsEqual` (`00c85800`), `NiInterpolator::GetSequenceInterpolator` (`00a57050`, (start,
/// end)), `NiKeyBasedInterpolator`'s constructor (`00a3ca10`), its member copy (`00a3c0a0`,
/// (clone, cloning)) and `GetViewerStrings` (`00a3cbf0`): all `thiscall`.
const KEY_BASED_SAVE_BINARY: u32 = 0x00a9_5ca0;
const QUATERNION_SAVE_BINARY: u32 = 0x00a6_e4a0;
const INTERPOLATOR_IS_EQUAL: u32 = 0x00c8_5800;
const INTERPOLATOR_GET_SEQUENCE_INTERPOLATOR: u32 = 0x00a5_7050;
const KEY_BASED_CONSTRUCT: u32 = 0x00a3_ca10;
const KEY_BASED_COPY_MEMBERS: u32 = 0x00a3_c0a0;
const KEY_BASED_GET_VIEWER_STRINGS: u32 = 0x00a3_cbf0;
/// `operator new` (`00aa13e0`, `cdecl`, size).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// `InterlockedIncrement` and `InterlockedDecrement` (import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `NiGetViewerString` (`cdecl`, the text), its pointer overload (`00a7d5a0`, `cdecl`,
/// (name, pointer)), `NiQuaternion::GetViewerString` (`00a6e190`, `thiscall` on the value,
/// the name), the viewer-string array's `SetSize` (`0096ad30`) and `SetAt` (`0096ae90`);
/// `"m_kQuaternionValue"` and `"m_spQuaternionData"`.
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const NI_GET_VIEWER_STRING_POINTER: u32 = 0x00a7_d5a0;
const QUATERNION_GET_VIEWER_STRING: u32 = 0x00a6_e190;
const STRINGS_SET_SIZE: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
const VALUE_NAME_TEXT: u32 = 0x0109_8800;
const DATA_NAME_TEXT: u32 = 0x0109_87ec;

/// Releases a counted object (count at +4; `DeleteThis`, slot +4, at zero).
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32()
        == 0
    {
        e.vcall(object, 4, &args![]);
    }
}

/// Copies four words.
fn copy_quaternion(e: &mut Engine, destination: u32, source: u32) {
    for word in 0..4 {
        let value = e.mem.u32(source.wrapping_add(4 * word));
        e.mem.set_u32(destination.wrapping_add(4 * word), value);
    }
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

// Translated from 00a487e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiQuaternionInterpolator::GetKeyContent` (Xbox PDB; `thiscall`, the channel it does not
/// read, `RET 4`): 2.
pub fn ni_quaternion_interpolator_get_key_content(
    _e: &mut Engine,
    _this: Ptr,
    _unused_1: u32,
) -> u32 {
    2
}

// Translated from 00a48800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiQuaternionInterpolator::GetChannelPosed` (Xbox PDB; `thiscall`, the channel it does
/// not read): false with rotation data; otherwise true when `00a92660` on the value and the
/// quaternion at `011f4208` answers 0.
pub fn ni_quaternion_interpolator_get_channel_posed(
    e: &mut Engine,
    this: Ptr<NiQuaternionInterpolator>,
    _unused_1: u32,
) -> bool {
    if e.get(this, NiQuaternionInterpolator::m_spQuaternionData) != 0 {
        return false;
    }
    !e.call(
        QUATERNION_COMPARE_00A92660,
        &args![this.addr() + 0xc, QUATERNION_011F4208],
    )
    .bool()
}

// Translated from 00a48820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiQuaternionInterpolator::Collapse` (Xbox PDB; `thiscall`): with rotation data, no keys
/// release the data and set the value to the quaternion at `011f4208`; otherwise the first
/// key's quaternion (key +4) becomes the value and the data is released when there is one
/// key not of type 4 (Euler), or when the type is 1 or 5 and every other key's quaternion
/// equals the first (`0064a290` says none differs).
pub fn ni_quaternion_interpolator_collapse(e: &mut Engine, this: Ptr<NiQuaternionInterpolator>) {
    let data = e.get(this, NiQuaternionInterpolator::m_spQuaternionData);
    if data == 0 {
        return;
    }
    let count = e.mem.u32(data + 8);
    let size = u32::from(e.mem.u8(data + 0x14));
    let keys = e.mem.u32(data + 0xc);
    let key_type = e.mem.u32(data + 0x10);
    if count == 0 {
        release(e, data);
        e.set(this, NiQuaternionInterpolator::m_spQuaternionData, 0);
        copy_quaternion(e, this.addr() + 0xc, QUATERNION_011F4208);
        return;
    }
    // The first key's quaternion, kept in a local.
    e.with_stack(0x10, |e, first| {
        let first = first.addr();
        copy_quaternion(e, first, keys.wrapping_add(4));
        if count != 1 || key_type == 4 {
            if key_type != 1 && key_type != 5 {
                return;
            }
            let mut same = true;
            let mut index = 1u32;
            while index < count {
                let key = size.wrapping_mul(index).wrapping_add(keys).wrapping_add(4);
                if e.call(QUATERNION_DIFFERS, &args![key, first]).bool() {
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
        let data = e.get(this, NiQuaternionInterpolator::m_spQuaternionData);
        if data != 0 {
            release(e, data);
            e.set(this, NiQuaternionInterpolator::m_spQuaternionData, 0);
        }
        copy_quaternion(e, this.addr() + 0xc, first);
    });
}

// Translated from 00a48960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiQuaternionInterpolator::GuaranteeTimeRange` (Xbox PDB; `thiscall`, (start, end), `RET
/// 8`): with rotation data, `00a48f70` on it with the two times (through the x87 stack).
pub fn ni_quaternion_interpolator_guarantee_time_range(
    e: &mut Engine,
    this: Ptr<NiQuaternionInterpolator>,
    start: f32,
    end: f32,
) {
    let data = e.get(this, NiQuaternionInterpolator::m_spQuaternionData);
    if data != 0 {
        e.call(
            GUARANTEE_KEYS,
            &args![
                data,
                x87_quiet_bits(start.to_bits()),
                x87_quiet_bits(end.to_bits())
            ],
        );
    }
}

// Translated from 00a48a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiQuaternionInterpolator::SaveBinary` (Xbox PDB; `thiscall`, the stream): the base
/// `SaveBinary` (`00a95ca0`), the value (`NiQuaternion::SaveBinary`), then the stream's
/// `SaveLinkID` (slot +0x30) of the rotation data.
pub fn ni_quaternion_interpolator_save_binary(
    e: &mut Engine,
    this: Ptr<NiQuaternionInterpolator>,
    stream: u32,
) {
    e.call(KEY_BASED_SAVE_BINARY, &args![this, stream]);
    e.call(QUATERNION_SAVE_BINARY, &args![this.addr() + 0xc, stream]);
    let data = e.get(this, NiQuaternionInterpolator::m_spQuaternionData);
    e.vcall(stream, 0x30, &args![data]);
}

// Translated from 00a48a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiQuaternionInterpolator::IsEqual` (Xbox PDB; `thiscall`, the other object): the base
/// `IsEqual` (`00c85800`), the values not differing (`0064a290`), and the rotation data in
/// both or neither, with both equal by their own `IsEqual` (slot +0x5C).
pub fn ni_quaternion_interpolator_is_equal(
    e: &mut Engine,
    this: Ptr<NiQuaternionInterpolator>,
    other: Ptr<NiQuaternionInterpolator>,
) -> bool {
    if !e.call(INTERPOLATOR_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    if e.call(
        QUATERNION_DIFFERS,
        &args![this.addr() + 0xc, other.addr() + 0xc],
    )
    .bool()
    {
        return false;
    }
    let own = e.get(this, NiQuaternionInterpolator::m_spQuaternionData);
    if (own == 0) != (e.get(other, NiQuaternionInterpolator::m_spQuaternionData) == 0) {
        return false;
    }
    if own == 0 {
        return true;
    }
    let theirs = e.get(other, NiQuaternionInterpolator::m_spQuaternionData);
    e.vcall(own, 0x5c, &args![theirs]).bool()
}

// Translated from 00a48ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiQuaternionInterpolator::GetActiveTimeRange` (Xbox PDB; `thiscall`, the addresses of
/// the begin and end times, `RET 8`): (0, 0) without rotation data or keys; for keys of a
/// type other than 4 the first key's time and the last key's (by the key size); for Euler
/// keys (type 4) the lowest first time and the highest last time of the three axes (the
/// axis key arrays at key +0x30, their counts at +0x14 and key sizes at +0x2C, one each),
/// from (`FLT_MAX`, `-FLT_MAX`).
pub fn ni_quaternion_interpolator_get_active_time_range(
    e: &mut Engine,
    this: Ptr<NiQuaternionInterpolator>,
    begin: Ptr,
    end: Ptr,
) {
    let data = e.get(this, NiQuaternionInterpolator::m_spQuaternionData);
    let (count, key_type, size, keys) = if data == 0 {
        (0, 0, 0, 0)
    } else {
        (
            e.mem.u32(data + 8),
            e.mem.u32(data + 0x10),
            u32::from(e.mem.u8(data + 0x14)),
            e.mem.u32(data + 0xc),
        )
    };
    if data == 0 || count == 0 {
        let zero = x87_store32(0.0);
        e.mem.set_u32(begin.addr(), zero);
        e.mem.set_u32(end.addr(), zero);
        return;
    }
    if key_type != 4 {
        let first = x87_quiet_bits(e.mem.u32(keys));
        e.mem.set_u32(begin.addr(), first);
        let last = count.wrapping_sub(1).wrapping_mul(size).wrapping_add(keys);
        let last = x87_quiet_bits(e.mem.u32(last));
        e.mem.set_u32(end.addr(), last);
        return;
    }
    let mut lowest = x87_quiet_bits(e.mem.u32(HIGHEST_FLOAT));
    let mut highest = x87_quiet_bits(e.mem.u32(LOWEST_FLOAT));
    for axis in 0..3u32 {
        let axis_keys = e.mem.u32(keys + 0x30 + 4 * axis);
        let axis_count = e.mem.u32(keys + 0x14 + 4 * axis);
        let axis_size = u32::from(e.mem.u8(keys + 0x2c + axis));
        let first = x87_store32(x87_load32(e.mem.u32(axis_keys)));
        if x87_load32(lowest) > x87_load32(first) {
            lowest = first;
        }
        let last = axis_count
            .wrapping_sub(1)
            .wrapping_mul(axis_size)
            .wrapping_add(axis_keys);
        let last = x87_store32(x87_load32(e.mem.u32(last)));
        if x87_load32(highest) < x87_load32(last) {
            highest = last;
        }
    }
    e.mem.set_u32(begin.addr(), x87_quiet_bits(lowest));
    e.mem.set_u32(end.addr(), x87_quiet_bits(highest));
}

// Translated from 00a48bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiQuaternionInterpolator::GetSequenceInterpolator` (Xbox PDB; `thiscall`, (start, end),
/// `RET 8`): `NiInterpolator::GetSequenceInterpolator` makes the new interpolator; with
/// rotation data, `NiRotData::GetSequenceData` for the times goes into a local smart
/// pointer, which is assigned (counted) to the new interpolator's data, its `m_uiLastIdx`
/// cleared, and the local released. Answers the new one.
pub fn ni_quaternion_interpolator_get_sequence_interpolator(
    e: &mut Engine,
    this: Ptr<NiQuaternionInterpolator>,
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
    let data = e.get(this, NiQuaternionInterpolator::m_spQuaternionData);
    if data == 0 {
        return sequence;
    }
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), 0);
        e.call(ROT_DATA_GET_SEQUENCE_DATA, &args![data, start, end, local]);
        let copy = e.mem.u32(local.addr());
        let slot = sequence.wrapping_add(0x1c);
        let old = e.mem.u32(slot);
        if old != copy {
            if old != 0 {
                release(e, old);
            }
            e.mem.set_u32(slot, copy);
            if copy != 0 {
                e.call(INTERLOCKED_INCREMENT, &args![copy.wrapping_add(4)]);
            }
        }
        e.mem.set_u32(sequence.wrapping_add(0x20), 0);
        let copy = e.mem.u32(local.addr());
        if copy != 0 {
            release(e, copy);
        }
    });
    sequence
}

// Translated from 00a48c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiQuaternionInterpolator::Update` (Xbox PDB; `thiscall`, (time, target it does not read,
/// &quaternion), `RET 0xC`): at the last time (+0x08, `FUCOMP`) the value is copied out and
/// the answer is whether `00a92660` on it and the quaternion at `011f4208` answers 0.
/// Otherwise, with rotation data holding keys, `NiRotKey::GenInterp` at the time (into a
/// local) becomes the value; then when `00a92660` answers 0 the value is copied out, the
/// time kept as the last time, and true answered; else false.
pub fn ni_quaternion_interpolator_update(
    e: &mut Engine,
    this: Ptr<NiQuaternionInterpolator>,
    time: f32,
    _unused_2: u32,
    value: Ptr,
) -> bool {
    let t = this.addr();
    let at = x87_load32(time.to_bits());
    let value_address = t + 0xc;
    if at == x87_load32(e.get(this, NiQuaternionInterpolator::m_fLastTime)) {
        copy_quaternion(e, value.addr(), value_address);
        return !e
            .call(
                QUATERNION_COMPARE_00A92660,
                &args![value_address, QUATERNION_011F4208],
            )
            .bool();
    }
    let data = e.get(this, NiQuaternionInterpolator::m_spQuaternionData);
    if data != 0 {
        let count = e.mem.u32(data + 8);
        let size = u32::from(e.mem.u8(data + 0x14));
        let key_type = e.mem.u32(data + 0x10);
        let keys = e.mem.u32(data + 0xc);
        if count != 0 {
            e.with_stack(0x10, |e, local| {
                let result = e
                    .call(
                        ROT_KEY_GEN_INTERP,
                        &args![
                            local,
                            x87_quiet_bits(time.to_bits()),
                            keys,
                            key_type,
                            count,
                            t + 0x20,
                            size
                        ],
                    )
                    .u32();
                copy_quaternion(e, value_address, result);
            });
        }
    }
    if e.call(
        QUATERNION_COMPARE_00A92660,
        &args![value_address, QUATERNION_011F4208],
    )
    .bool()
    {
        return false;
    }
    copy_quaternion(e, value.addr(), value_address);
    e.set(
        this,
        NiQuaternionInterpolator::m_fLastTime,
        x87_quiet_bits(time.to_bits()),
    );
    true
}

// Translated from 00a48e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiQuaternionInterpolator::CreateClone` (Xbox PDB; `thiscall`, the cloning process): a
/// new 0x24-byte interpolator (`operator new`, `NiKeyBasedInterpolator`'s constructor, the
/// vtable `010986f4`, the value from `011f4208`, no data, index 0) given this one's members
/// (`00a3c0a0`, also run with a null clone when the allocation fails). Answers the clone or
/// null.
pub fn ni_quaternion_interpolator_create_clone(e: &mut Engine, this: Ptr, cloning: u32) -> Ptr {
    let object = e.call(OPERATOR_NEW, &args![0x24u32]).u32();
    if object != 0 {
        e.call(KEY_BASED_CONSTRUCT, &args![object]);
        e.mem.set_u32(object, NI_QUATERNION_INTERPOLATOR_VTABLE);
        copy_quaternion(e, object + 0xc, QUATERNION_011F4208);
        e.mem.set_u32(object + 0x1c, 0);
        e.mem.set_u32(object + 0x20, 0);
    }
    e.call(KEY_BASED_COPY_MEMBERS, &args![this, object, cloning]);
    Ptr::new(object)
}

// Translated from 00a48ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiQuaternionInterpolator::GetViewerStrings` (Xbox PDB; `thiscall`, the viewer-string
/// array): `NiKeyBasedInterpolator::GetViewerStrings`, then the class name (the first word
/// of the `NiRTTI` record at `011f3f60`), the value's string (`"m_kQuaternionValue"`) and
/// the data pointer's (`"m_spQuaternionData"`) appended.
pub fn ni_quaternion_interpolator_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<NiQuaternionInterpolator>,
    strings: u32,
) {
    e.call(KEY_BASED_GET_VIEWER_STRINGS, &args![this, strings]);
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        let class_name = e.global::<u32>(NI_QUATERNION_INTERPOLATOR_RTTI);
        let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
        add_viewer_string(e, strings, slot, text);
        let text = e
            .call(
                QUATERNION_GET_VIEWER_STRING,
                &args![this.addr() + 0xc, VALUE_NAME_TEXT],
            )
            .u32();
        add_viewer_string(e, strings, slot, text);
        let data = e.get(this, NiQuaternionInterpolator::m_spQuaternionData);
        let text = e
            .call(NI_GET_VIEWER_STRING_POINTER, &args![DATA_NAME_TEXT, data])
            .u32();
        add_viewer_string(e, strings, slot, text);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    type Q = NiQuaternionInterpolator;
    vec![
        entry!(0x00a48ab0, ni_quaternion_interpolator_get_rtti() -> Ptr),
        entry!(
            0x00a4_87e0,
            ni_quaternion_interpolator_get_key_content(Ptr, u32) -> u32
        ),
        entry!(
            0x00a4_8800,
            ni_quaternion_interpolator_get_channel_posed(Ptr<Q>, u32) -> bool
        ),
        entry!(0x00a4_8820, ni_quaternion_interpolator_collapse(Ptr<Q>)),
        entry!(
            0x00a4_8960,
            ni_quaternion_interpolator_guarantee_time_range(Ptr<Q>, f32, f32)
        ),
        entry!(
            0x00a4_8a20,
            ni_quaternion_interpolator_save_binary(Ptr<Q>, u32)
        ),
        entry!(
            0x00a4_8a50,
            ni_quaternion_interpolator_is_equal(Ptr<Q>, Ptr<Q>) -> bool
        ),
        entry!(
            0x00a4_8ac0,
            ni_quaternion_interpolator_get_active_time_range(Ptr<Q>, Ptr, Ptr)
        ),
        entry!(
            0x00a4_8bc0,
            ni_quaternion_interpolator_get_sequence_interpolator(Ptr<Q>, f32, f32) -> u32
        ),
        entry!(
            0x00a4_8c80,
            ni_quaternion_interpolator_update(Ptr<Q>, f32, u32, Ptr) -> bool
        ),
        entry!(
            0x00a4_8e20,
            ni_quaternion_interpolator_create_clone(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00a4_8ea0,
            ni_quaternion_interpolator_get_viewer_strings(Ptr<Q>, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_quaternion_interpolator_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a48ab0, &args![]).u32(),
            NI_QUATERNION_INTERPOLATOR_RTTI
        );
    }
}

#[cfg(test)]
mod n005_tests {
    use super::*;

    type Q = NiQuaternionInterpolator;

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
        e.register(0x0099_005c, |_, a| (a[1] & 1 == 0).into_ret());
    }

    /// Rotation data (counted once, vtable slots +4 and +0x5C doubled) with the given keys.
    fn rot_data(e: &mut Engine, count: u32, key_type: u32, keys: u32, size: u8) -> u32 {
        let data = e.mem.alloc(0x18);
        let table = e.mem.alloc(0x60);
        e.mem.set_u32(table + 4, 0x0099_0004);
        e.mem.set_u32(table + 0x5c, 0x0099_005c);
        e.mem.set_u32(data, table);
        e.mem.set_u32(data + 4, 1);
        e.mem.set_u32(data + 8, count);
        e.mem.set_u32(data + 0xc, keys);
        e.mem.set_u32(data + 0x10, key_type);
        e.mem.set_u8(data + 0x14, size);
        data
    }

    /// The quaternion at `011f4208` set to (9, 8, 7, 6).
    fn constants(e: &mut Engine) {
        e.map(0x011f_4000, 0x1000);
        for (i, v) in [9.0f32, 8.0, 7.0, 6.0].iter().enumerate() {
            e.mem.set_f32(QUATERNION_011F4208 + 4 * i as u32, *v);
        }
    }

    fn quaternion(e: &Engine, at: u32) -> Vec<f32> {
        (0..4).map(|i| e.mem.f32(at + 4 * i)).collect()
    }

    #[test]
    fn key_content_is_2() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a4_87e0, &[0x1000, 7]).u32(), 2);
    }

    #[test]
    fn channel_posed_without_data_asks_00a92660() {
        let mut e = Engine::new();
        e.register(QUATERNION_COMPARE_00A92660, |e, a| {
            (e.mem.u32(a[0]) == 1).into_ret()
        });
        let this = e.new_object::<Q>();
        e.mem.set_u32(this.addr() + 0xc, 2);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00a4_8800, &[this.addr(), 0]).bool());
        assert_eq!(
            calls_to(&e, QUATERNION_COMPARE_00A92660),
            vec![vec![this.addr() + 0xc, QUATERNION_011F4208]]
        );
        e.mem.set_u32(this.addr() + 0xc, 1);
        assert!(!e.call(0x00a4_8800, &[this.addr(), 0]).bool());
        e.set(this, Q::m_spQuaternionData, 0x5000);
        e.mem.set_u32(this.addr() + 0xc, 2);
        assert!(!e.call(0x00a4_8800, &[this.addr(), 0]).bool());
    }

    /// Keys of 0x14 bytes whose quaternions are (i, i, i, i) for the given values.
    fn keys(e: &mut Engine, values: &[f32]) -> u32 {
        let keys = e.mem.alloc(0x14 * values.len() as u32);
        for (i, v) in values.iter().enumerate() {
            for word in 0..4 {
                e.mem.set_f32(keys + 0x14 * i as u32 + 4 + 4 * word, *v);
            }
        }
        keys
    }

    #[test]
    fn collapse_takes_the_constant_key() {
        let mut e = Engine::new();
        counting(&mut e);
        constants(&mut e);
        e.register(QUATERNION_DIFFERS, |e, a| {
            (e.mem.u32(a[0]) != e.mem.u32(a[1])).into_ret()
        });
        // No keys: the data goes and the value is the quaternion at 011f4208.
        let this = e.new_object::<Q>();
        let data = rot_data(&mut e, 0, 1, 0, 0x14);
        e.set(this, Q::m_spQuaternionData, data);
        e.call_log = Some(vec![]);
        e.call(0x00a4_8820, &[this.addr()]);
        assert_eq!(e.get(this, Q::m_spQuaternionData), 0);
        assert_eq!(quaternion(&e, this.addr() + 0xc), [9.0, 8.0, 7.0, 6.0]);
        assert_eq!(calls_to(&e, 0x0099_0004), vec![vec![data]]);
        // One key: collapsed to it.
        let one = keys(&mut e, &[2.0]);
        let data = rot_data(&mut e, 1, 2, one, 0x14);
        e.set(this, Q::m_spQuaternionData, data);
        e.call(0x00a4_8820, &[this.addr()]);
        assert_eq!(e.get(this, Q::m_spQuaternionData), 0);
        assert_eq!(quaternion(&e, this.addr() + 0xc), [2.0; 4]);
        // One Euler key: kept.
        let data = rot_data(&mut e, 1, 4, one, 0x14);
        e.set(this, Q::m_spQuaternionData, data);
        e.call(0x00a4_8820, &[this.addr()]);
        assert_eq!(e.get(this, Q::m_spQuaternionData), data);
        // Linear keys all equal: collapsed; one different: kept.
        let same = keys(&mut e, &[3.0, 3.0, 3.0]);
        let data = rot_data(&mut e, 3, 1, same, 0x14);
        e.set(this, Q::m_spQuaternionData, data);
        e.call(0x00a4_8820, &[this.addr()]);
        assert_eq!(e.get(this, Q::m_spQuaternionData), 0);
        assert_eq!(quaternion(&e, this.addr() + 0xc), [3.0; 4]);
        let mixed = keys(&mut e, &[3.0, 4.0, 3.0]);
        let data = rot_data(&mut e, 3, 5, mixed, 0x14);
        e.set(this, Q::m_spQuaternionData, data);
        e.call_log = Some(vec![]);
        e.call(0x00a4_8820, &[this.addr()]);
        assert_eq!(e.get(this, Q::m_spQuaternionData), data);
        // The first differing key stops the scan.
        assert_eq!(calls_to(&e, QUATERNION_DIFFERS).len(), 1);
        // Several keys of another type: kept.
        let data = rot_data(&mut e, 3, 2, same, 0x14);
        e.set(this, Q::m_spQuaternionData, data);
        e.call(0x00a4_8820, &[this.addr()]);
        assert_eq!(e.get(this, Q::m_spQuaternionData), data);
    }

    #[test]
    fn guarantee_time_range_forwards_to_the_data() {
        let mut e = Engine::new();
        e.register(GUARANTEE_KEYS, |_, _| Ret::default());
        let this = e.new_object::<Q>();
        e.call_log = Some(vec![]);
        e.call(0x00a4_8960, &[this.addr(), 0, 0]);
        assert!(calls_to(&e, GUARANTEE_KEYS).is_empty());
        e.set(this, Q::m_spQuaternionData, 0x5000);
        e.call(0x00a4_8960, &[this.addr(), 0x7f80_0001, 2.0f32.to_bits()]);
        assert_eq!(
            calls_to(&e, GUARANTEE_KEYS),
            vec![vec![0x5000, 0x7fc0_0001, 2.0f32.to_bits()]]
        );
    }

    #[test]
    fn save_binary_saves_value_and_data_link() {
        let mut e = Engine::new();
        e.register(KEY_BASED_SAVE_BINARY, |_, _| Ret::default());
        e.register(QUATERNION_SAVE_BINARY, |_, _| Ret::default());
        e.register(0x0099_0030, |_, _| Ret::default());
        let stream = e.mem.alloc(0x300);
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(table + 0x30, 0x0099_0030);
        e.mem.set_u32(stream, table);
        let this = e.new_object::<Q>();
        e.set(this, Q::m_spQuaternionData, 0x5000);
        e.call_log = Some(vec![]);
        e.call(0x00a4_8a20, &[this.addr(), stream]);
        assert_eq!(
            calls_to(&e, KEY_BASED_SAVE_BINARY),
            vec![vec![this.addr(), stream]]
        );
        assert_eq!(
            calls_to(&e, QUATERNION_SAVE_BINARY),
            vec![vec![this.addr() + 0xc, stream]]
        );
        assert_eq!(calls_to(&e, 0x0099_0030), vec![vec![stream, 0x5000]]);
    }

    #[test]
    fn is_equal_compares_value_and_data() {
        let mut e = Engine::new();
        counting(&mut e);
        e.register(INTERPOLATOR_IS_EQUAL, |_, _| true.into_ret());
        e.register(QUATERNION_DIFFERS, |e, a| {
            (e.mem.u32(a[0]) != e.mem.u32(a[1])).into_ret()
        });
        let a = e.new_object::<Q>();
        let b = e.new_object::<Q>();
        let equal = |e: &mut Engine| e.call(0x00a4_8a50, &[a.addr(), b.addr()]).bool();
        assert!(equal(&mut e));
        e.mem.set_u32(b.addr() + 0xc, 1);
        assert!(!equal(&mut e));
        e.mem.set_u32(b.addr() + 0xc, 0);
        let data = rot_data(&mut e, 0, 0, 0, 0);
        e.set(a, Q::m_spQuaternionData, data);
        assert!(!equal(&mut e));
        // Both with data: the data's IsEqual (here: an even address is equal).
        e.set(b, Q::m_spQuaternionData, 0x5000);
        assert!(equal(&mut e));
        e.set(b, Q::m_spQuaternionData, 0x5001);
        assert!(!equal(&mut e));
        e.set(a, Q::m_spQuaternionData, 0);
        assert!(!equal(&mut e));
        e.register(INTERPOLATOR_IS_EQUAL, |_, _| false.into_ret());
        e.set(b, Q::m_spQuaternionData, 0);
        assert!(!equal(&mut e));
    }

    fn range_constants(e: &mut Engine) {
        e.map(0x0109_8000, 0x1000);
        e.map(0x0101_5000, 0x1000);
        e.mem.set_f32(HIGHEST_FLOAT, f32::MAX);
        e.mem.set_f32(LOWEST_FLOAT, f32::MIN);
    }

    #[test]
    fn active_time_range_of_plain_and_euler_keys() {
        let mut e = Engine::new();
        range_constants(&mut e);
        let this = e.new_object::<Q>();
        let (begin, end) = (e.mem.alloc(4), e.mem.alloc(4));
        e.mem.set_u32(begin, 0x1234);
        e.call(0x00a4_8ac0, &[this.addr(), begin, end]);
        assert_eq!((e.mem.u32(begin), e.mem.u32(end)), (0, 0));
        // Plain keys: first and last time by the key size.
        let plain = e.mem.alloc(0x40);
        e.mem.set_f32(plain, 0.25);
        e.mem.set_f32(plain + 0x28, 3.0);
        let data = rot_data(&mut e, 3, 1, plain, 0x14);
        e.set(this, Q::m_spQuaternionData, data);
        e.call(0x00a4_8ac0, &[this.addr(), begin, end]);
        assert_eq!((e.mem.f32(begin), e.mem.f32(end)), (0.25, 3.0));
        // Euler keys: the widest range of the three axes.
        let euler = e.mem.alloc(0x40);
        let axes = [
            (2u32, 8u8, [1.0f32, 2.0]),
            (1, 4, [0.5, 0.5]),
            (3, 8, [1.5, 4.0]),
        ];
        for (axis, (count, size, [first, last])) in axes.iter().enumerate() {
            let a = axis as u32;
            let array = e.mem.alloc(0x40);
            e.mem.set_f32(array, *first);
            e.mem.set_f32(array + (count - 1) * u32::from(*size), *last);
            e.mem.set_u32(euler + 0x30 + 4 * a, array);
            e.mem.set_u32(euler + 0x14 + 4 * a, *count);
            e.mem.set_u8(euler + 0x2c + a, *size);
        }
        let data = rot_data(&mut e, 1, 4, euler, 0x3c);
        e.set(this, Q::m_spQuaternionData, data);
        e.call(0x00a4_8ac0, &[this.addr(), begin, end]);
        assert_eq!((e.mem.f32(begin), e.mem.f32(end)), (0.5, 4.0));
        // Data without keys: zero.
        let data = rot_data(&mut e, 0, 4, euler, 0x3c);
        e.set(this, Q::m_spQuaternionData, data);
        e.call(0x00a4_8ac0, &[this.addr(), begin, end]);
        assert_eq!((e.mem.u32(begin), e.mem.u32(end)), (0, 0));
    }

    #[test]
    fn sequence_interpolator_takes_the_sequence_rotation_data() {
        let mut e = Engine::new();
        counting(&mut e);
        let sequence = e.new_object::<Q>();
        e.set(sequence, Q::m_uiLastIdx, 4);
        let new = rot_data(&mut e, 0, 0, 0, 0);
        e.register_double(INTERPOLATOR_GET_SEQUENCE_INTERPOLATOR, move |_, _| {
            sequence.into_ret()
        });
        e.register_double(ROT_DATA_GET_SEQUENCE_DATA, move |e, a| {
            assert_eq!(e.mem.u32(a[3]), 0);
            e.mem.set_u32(a[3], new);
            Ret::default()
        });
        let this = e.new_object::<Q>();
        e.call_log = Some(vec![]);
        let start = 1.0f32.to_bits();
        let end = 2.0f32.to_bits();
        assert_eq!(
            e.call(0x00a4_8bc0, &[this.addr(), start, end]).u32(),
            sequence.addr()
        );
        assert!(calls_to(&e, ROT_DATA_GET_SEQUENCE_DATA).is_empty());
        e.set(this, Q::m_spQuaternionData, 0x6000);
        e.call(0x00a4_8bc0, &[this.addr(), start, end]);
        assert_eq!(
            calls_to(&e, ROT_DATA_GET_SEQUENCE_DATA)[0][..3],
            [0x6000, start, end]
        );
        assert_eq!(e.get(sequence, Q::m_spQuaternionData), new);
        assert_eq!(e.get(sequence, Q::m_uiLastIdx), 0);
        assert_eq!(e.mem.u32(new + 4), 1);
    }

    #[test]
    fn update_interpolates_and_keeps_the_time() {
        let mut e = Engine::new();
        e.register(QUATERNION_COMPARE_00A92660, |e, a| {
            (e.mem.f32(a[0]) == 9.0).into_ret()
        });
        e.register(ROT_KEY_GEN_INTERP, |e, a| {
            for word in 0..4 {
                e.mem.set_f32(a[0] + 4 * word, f32::from_bits(a[1]) * 2.0);
            }
            a[0].into_ret()
        });
        let this = e.new_object::<Q>();
        let data = rot_data(&mut e, 2, 1, 0x7000, 0x14);
        e.set(this, Q::m_spQuaternionData, data);
        e.set(this, Q::m_fLastTime, 1.0f32.to_bits());
        let out = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x00a4_8c80, &[this.addr(), 1.5f32.to_bits(), 0, out])
            .bool());
        assert_eq!(
            calls_to(&e, ROT_KEY_GEN_INTERP)[0][1..],
            [1.5f32.to_bits(), 0x7000, 1, 2, this.addr() + 0x20, 0x14]
        );
        assert_eq!(quaternion(&e, out), [3.0; 4]);
        assert_eq!(e.get(this, Q::m_fLastTime), 1.5f32.to_bits());
        // At the last time: the value copied, no interpolation.
        e.call_log = Some(vec![]);
        e.mem.set_f32(this.addr() + 0xc, 5.0);
        assert!(e
            .call(0x00a4_8c80, &[this.addr(), 1.5f32.to_bits(), 0, out])
            .bool());
        assert!(calls_to(&e, ROT_KEY_GEN_INTERP).is_empty());
        assert_eq!(e.mem.f32(out), 5.0);
        e.mem.set_f32(this.addr() + 0xc, 9.0);
        assert!(!e
            .call(0x00a4_8c80, &[this.addr(), 1.5f32.to_bits(), 0, out])
            .bool());
        // A value 00a92660 recognises after the interpolation: false, time not kept.
        let out2 = e.mem.alloc(0x10);
        assert!(!e
            .call(0x00a4_8c80, &[this.addr(), 4.5f32.to_bits(), 0, out2])
            .bool());
        assert_eq!(e.get(this, Q::m_fLastTime), 1.5f32.to_bits());
        assert_eq!(e.mem.u32(out2), 0);
        // Without data: the value as it is.
        e.set(this, Q::m_spQuaternionData, 0);
        e.mem.set_f32(this.addr() + 0xc, 1.0);
        assert!(e
            .call(0x00a4_8c80, &[this.addr(), 7.0f32.to_bits(), 0, out2])
            .bool());
        assert_eq!(e.mem.f32(out2), 1.0);
    }

    #[test]
    fn create_clone_builds_and_copies() {
        let mut e = Engine::new();
        constants(&mut e);
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(KEY_BASED_CONSTRUCT, |_, _| Ret::default());
        e.register(KEY_BASED_COPY_MEMBERS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let clone = e.call(0x00a4_8e20, &[0x5000, 0x6000]).u32();
        assert_eq!(e.mem.u32(clone), NI_QUATERNION_INTERPOLATOR_VTABLE);
        assert_eq!(quaternion(&e, clone + 0xc), [9.0, 8.0, 7.0, 6.0]);
        assert_eq!(
            calls_to(&e, KEY_BASED_COPY_MEMBERS),
            vec![vec![0x5000, clone, 0x6000]]
        );
        e.register(OPERATOR_NEW, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a4_8e20, &[0x5000, 0x6000]).u32(), 0);
        assert_eq!(
            calls_to(&e, KEY_BASED_COPY_MEMBERS)[1],
            vec![0x5000, 0, 0x6000]
        );
    }

    #[test]
    fn get_viewer_strings_lists_class_value_and_data() {
        let mut e = Engine::new();
        e.map(NI_QUATERNION_INTERPOLATOR_RTTI & !0xfff, 0x1000);
        e.set_global(NI_QUATERNION_INTERPOLATOR_RTTI, 0x0444_0000u32);
        e.register(KEY_BASED_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(NI_GET_VIEWER_STRING, |_, _| 0x0600_0001u32.into_ret());
        e.register(QUATERNION_GET_VIEWER_STRING, |_, _| {
            0x0600_0002u32.into_ret()
        });
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
        let this = e.new_object::<Q>();
        e.set(this, Q::m_spQuaternionData, 0x5000);
        e.call_log = Some(vec![]);
        e.call(0x00a4_8ea0, &[this.addr(), strings]);
        let listed: Vec<u32> = (0..3).map(|i| e.mem.u32(base + 4 * i)).collect();
        assert_eq!(listed, [0x0600_0001, 0x0600_0002, 0x0600_0003]);
        assert_eq!(
            calls_to(&e, QUATERNION_GET_VIEWER_STRING),
            vec![vec![this.addr() + 0xc, VALUE_NAME_TEXT]]
        );
        assert_eq!(
            calls_to(&e, NI_GET_VIEWER_STRING_POINTER),
            vec![vec![DATA_NAME_TEXT, 0x5000]]
        );
    }
}
