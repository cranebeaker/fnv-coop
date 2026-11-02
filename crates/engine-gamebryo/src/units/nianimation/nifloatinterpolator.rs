//! `NiAnimation/nifloatinterpolator.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `NiFloatInterpolator` (0x18 bytes, the `CreateClone` allocation; PC
//! offsets as the code uses them): `NiKeyBasedInterpolator` (the last
//! update time at +0x08), then the current value `m_fValue` +0x0c, the
//! `NiFloatData` pointer `m_spFloatData` +0x10 and the last key index
//! `m_uiLastIdx` +0x14 (Xbox PDB names).
//!
//! None of these functions has an exception frame or a `/GS` check.

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiFloatInterpolator::ms_RTTI` (the class's `NiRTTI` record).
const NI_FLOAT_INTERPOLATOR_RTTI: u32 = 0x011f_3738;

/// The vtable of `NiFloatInterpolator` (RTTI `.?AVNiFloatInterpolator@@`).
const NI_FLOAT_INTERPOLATOR_VTABLE: u32 = 0x0109_6f34;
/// `NiFloatInterpolator` size (the `CreateClone` allocation).
const NI_FLOAT_INTERPOLATOR_SIZE: u32 = 0x18;

/// The last update time (`NiInterpolator`), `m_fValue`, `m_spFloatData`,
/// `m_uiLastIdx` (see the module comment).
const LAST_TIME: u32 = 0x08;
const VALUE: u32 = 0x0c;
const FLOAT_DATA: u32 = 0x10;
const LAST_INDEX: u32 = 0x14;

/// The "no value" float (`-FLT_MAX` in the exe, read from `.rdata`).
const INVALID_FLOAT: u32 = 0x0109_a918;

/// `NiKeyBasedInterpolator::NiKeyBasedInterpolator` (`00a3ca10`).
const KEY_BASED_CONSTRUCT: u32 = 0x00a3_ca10;
/// `00aa0ab0` (unnamed in the engine map; `thiscall`, `RET 8`): the
/// `NiKeyBasedInterpolator` member copy `(this, destination, cloning)`.
const KEY_BASED_COPY_MEMBERS: u32 = 0x00aa_0ab0;
/// `NiKeyBasedInterpolator::LoadBinary` (`00aa0ac0`).
const KEY_BASED_LOAD_BINARY: u32 = 0x00aa_0ac0;
/// `NiFloatKey::GenInterp` (`00a26b40`, `cdecl`, six words: time, keys,
/// key type, key count, `&last index`, key size); returns the value in ST0.
const FLOAT_KEY_GEN_INTERP: u32 = 0x00a2_6b40;
/// `NiStream::ResolveLinkID` (`00a64620`, `thiscall` on the stream).
const RESOLVE_LINK_ID: u32 = 0x00a6_4620;
/// `00aa13e0` (`cdecl`, one word): allocates from the Ni allocator.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `NiRefObject::DeleteThis` (virtual slot 1).
const SLOT_DELETE_THIS: u32 = 0x04;

/// `NiStream::m_pkIstr` +0x24c (Xbox PDB; PC offset), and the input
/// stream's position (+4) and read function (+8, `cdecl` `(stream, buffer,
/// size, &component size, component count)`, returns the bytes read).
const STREAM_INPUT: u32 = 0x24c;
const BINARY_STREAM_POSITION: u32 = 0x4;
const BINARY_STREAM_READ: u32 = 0x8;

/// One read through the stream's read function into `buffer`: the word at
/// `slot` is set to `component` and passed as the component size; the
/// position advances by the bytes read.
fn stream_read(e: &mut Engine, stream: u32, buffer: u32, size: u32, component: u32, slot: u32) {
    let input = e.mem.u32(stream.wrapping_add(STREAM_INPUT));
    let read = e.mem.u32(input.wrapping_add(BINARY_STREAM_READ));
    e.mem.set_u32(slot, component);
    let bytes = e.call(read, &args![input, buffer, size, slot, 1u32]).u32();
    let position = e.mem.u32(input.wrapping_add(BINARY_STREAM_POSITION));
    e.mem.set_u32(
        input.wrapping_add(BINARY_STREAM_POSITION),
        position.wrapping_add(bytes),
    );
}

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

/// The inline `NiPointer::operator=` for the pointer at `slot`: when `new`
/// differs, the old one (if set) is released, `new` stored and (if set)
/// counted.
fn assign_pointer(e: &mut Engine, slot: u32, new: u32) {
    let old = e.mem.u32(slot);
    if old != new {
        if old != 0 {
            release(e, old);
        }
        e.mem.set_u32(slot, new);
        if new != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![new.wrapping_add(4)]);
        }
    }
}

// Translated from 00a3c380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatInterpolator::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_float_interpolator_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_FLOAT_INTERPOLATOR_RTTI)
}

// Translated from 00a3c390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatInterpolator::NiFloatInterpolator` (Xbox PDB), the constructor
/// taking a pose value: the `NiKeyBasedInterpolator` constructor, then the
/// value (an x87 load and store), the vtable, no float data and last index
/// 0. Returns `this`.
pub fn ni_float_interpolator_construct(e: &mut Engine, this: Ptr, value: f32) -> Ptr {
    let object = this.addr();
    e.call(KEY_BASED_CONSTRUCT, &args![this]);
    e.mem
        .set_u32(object.wrapping_add(VALUE), x87_quiet_bits(value.to_bits()));
    e.mem.set_u32(object, NI_FLOAT_INTERPOLATOR_VTABLE);
    e.mem.set_u32(object.wrapping_add(FLOAT_DATA), 0);
    e.mem.set_u32(object.wrapping_add(LAST_INDEX), 0);
    this
}

// Translated from 00a3c6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatInterpolator::Update` (Xbox PDB): at the time of the last update
/// the stored value goes to `out` and the result is whether it is valid
/// (not the "no value" float at `0109a918`). At another time, with float
/// data that has keys, `NiFloatKey::GenInterp` computes the value (stored
/// as a float); an invalid value returns false and leaves `out` and the
/// time alone, a valid one goes to `out` and the time is kept. The second
/// argument word is not read.
///
/// The key size word pushed for `GenInterp` is the stack word that first
/// held `this` (`PUSH ECX`) with its low byte replaced by the key size.
pub fn ni_float_interpolator_update(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    _unused_2: u32,
    out: Ptr,
) -> bool {
    let this = this.addr();
    let time_bits = x87_quiet_bits(time.to_bits());
    let last_time = x87_load32(e.mem.u32(this.wrapping_add(LAST_TIME)));
    if x87_load32(time_bits) == last_time {
        let value = e.mem.u32(this.wrapping_add(VALUE));
        e.mem.set_u32(out.addr(), x87_quiet_bits(value));
        let value = x87_load32(e.mem.u32(this.wrapping_add(VALUE)));
        let invalid = x87_load32(e.mem.u32(INVALID_FLOAT));
        return invalid != value;
    }
    let data = e.mem.u32(this.wrapping_add(FLOAT_DATA));
    if data != 0 {
        let num_keys = e.mem.u32(data.wrapping_add(8));
        let key_size = e.mem.u8(data.wrapping_add(0x14));
        let key_type = e.mem.u32(data.wrapping_add(0x10));
        let keys = e.mem.u32(data.wrapping_add(0xc));
        if num_keys != 0 {
            let key_size_word = (this & 0xffff_ff00) | u32::from(key_size);
            let value = e
                .call(
                    FLOAT_KEY_GEN_INTERP,
                    &args![
                        time_bits,
                        keys,
                        key_type,
                        num_keys,
                        this.wrapping_add(LAST_INDEX),
                        key_size_word
                    ],
                )
                .f64();
            e.mem.set_u32(this.wrapping_add(VALUE), x87_store32(value));
        }
    }
    let value = x87_load32(e.mem.u32(this.wrapping_add(VALUE)));
    let invalid = x87_load32(e.mem.u32(INVALID_FLOAT));
    if invalid == value {
        return false;
    }
    let value = e.mem.u32(this.wrapping_add(VALUE));
    e.mem.set_u32(out.addr(), x87_quiet_bits(value));
    e.mem.set_u32(this.wrapping_add(LAST_TIME), time_bits);
    true
}

// Translated from 00a3c780 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `NiFloatInterpolator`'s member copy into
/// `destination` for a clone made by `cloning` (the function `CreateClone`
/// runs). The `NiKeyBasedInterpolator` copy (`00aa0ab0`), then the value
/// (an x87 load and store), the float data pointer (counted; the old one
/// released) and the last index.
pub fn ni_float_interpolator_copy_members(
    e: &mut Engine,
    this: Ptr,
    destination: Ptr,
    cloning: u32,
) {
    e.call(KEY_BASED_COPY_MEMBERS, &args![this, destination, cloning]);
    let this = this.addr();
    let destination = destination.addr();
    let value = e.mem.u32(this.wrapping_add(VALUE));
    e.mem
        .set_u32(destination.wrapping_add(VALUE), x87_quiet_bits(value));
    let old = e.mem.u32(destination.wrapping_add(FLOAT_DATA));
    if old != e.mem.u32(this.wrapping_add(FLOAT_DATA)) {
        if old != 0 {
            release(e, old);
        }
        let new = e.mem.u32(this.wrapping_add(FLOAT_DATA));
        e.mem.set_u32(destination.wrapping_add(FLOAT_DATA), new);
        if new != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![new.wrapping_add(4)]);
        }
    }
    let last_index = e.mem.u32(this.wrapping_add(LAST_INDEX));
    e.mem
        .set_u32(destination.wrapping_add(LAST_INDEX), last_index);
}

// Translated from 00a3c800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatInterpolator::LoadBinary` (Xbox PDB): the
/// `NiKeyBasedInterpolator` load, the value (4 bytes, the component size in
/// the argument's own stack word), then the float data resolved from its
/// link ID (`NiStream::ResolveLinkID`) and stored counted.
pub fn ni_float_interpolator_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(KEY_BASED_LOAD_BINARY, &args![this, stream]);
    let this = this.addr();
    let stream = stream.addr();
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), stream);
        stream_read(e, stream, this.wrapping_add(VALUE), 4, 4, slot.addr());
    });
    let data = e.call(RESOLVE_LINK_ID, &args![stream]).u32();
    assign_pointer(e, this.wrapping_add(FLOAT_DATA), data);
}

// Translated from 00a3c8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatInterpolator::CreateClone` (Xbox PDB): allocates 0x18 bytes;
/// when that succeeds runs the `NiKeyBasedInterpolator` constructor, stores
/// the vtable, the "no value" float, no float data and last index 0. Then
/// the member copy into it, and returns it. When the allocation fails the
/// member copy still runs, into address 0, and 0 is returned.
pub fn ni_float_interpolator_create_clone(e: &mut Engine, this: Ptr, cloning: u32) -> Ptr {
    let object = e
        .call(NI_ALLOCATE, &args![NI_FLOAT_INTERPOLATOR_SIZE])
        .u32();
    if object == 0 {
        ni_float_interpolator_copy_members(e, this, Ptr::new(0), cloning);
        return Ptr::new(0);
    }
    e.call(KEY_BASED_CONSTRUCT, &args![object]);
    e.mem.set_u32(object, NI_FLOAT_INTERPOLATOR_VTABLE);
    let invalid = e.mem.u32(INVALID_FLOAT);
    e.mem
        .set_u32(object.wrapping_add(VALUE), x87_quiet_bits(invalid));
    e.mem.set_u32(object.wrapping_add(FLOAT_DATA), 0);
    e.mem.set_u32(object.wrapping_add(LAST_INDEX), 0);
    ni_float_interpolator_copy_members(e, this, Ptr::new(object), cloning);
    Ptr::new(object)
}

/// Virtual slot `NiObject::IsEqual` (Xbox PDB) +0x5c and `NiStream::SaveLinkID`
/// (Xbox PDB) +0x30.
const SLOT_IS_EQUAL: u32 = 0x5c;
const SLOT_SAVE_LINK_ID: u32 = 0x30;
/// `00c85800` (Ghidra's `NiObject::IsEqual`; `thiscall`, `RET 4`): the base
/// `IsEqual`.
const BASE_IS_EQUAL: u32 = 0x00c8_5800;
/// `00a95ca0` (`thiscall`, `RET 4`: stream; an empty body the engine map
/// names `BSGrabIKNote::LinkObject`): the base `SaveBinary` call.
const BASE_SAVE_BINARY: u32 = 0x00a9_5ca0;
/// `NiFloatData::GuaranteeKeysAtStartAndEnd` (`00a3cc40`; `thiscall`: start,
/// end).
const FLOAT_DATA_GUARANTEE_KEYS: u32 = 0x00a3_cc40;
/// `NiFloatData::GetSequenceData` (`00a3ce20`; `thiscall`: start, end,
/// address of the result pointer).
const FLOAT_DATA_GET_SEQUENCE_DATA: u32 = 0x00a3_ce20;
/// `NiInterpolator::GetSequenceInterpolator` (`00a57050`; `thiscall`:
/// start, end; answers the new interpolator).
const INTERPOLATOR_GET_SEQUENCE_INTERPOLATOR: u32 = 0x00a5_7050;
/// `007fc2b0` (`thiscall`, `RET 4`): assigns its argument to the `NiPointer`
/// at +0x10 of `this` (through `0066b0d0`); here the copy's float data.
const SET_POINTER_AT_0X10: u32 = 0x007f_c2b0;
/// `NiKeyBasedInterpolator::GetViewerStrings` (`00a3cbf0`).
const KEY_BASED_GET_VIEWER_STRINGS: u32 = 0x00a3_cbf0;
/// `NiGetViewerString` (`00a7d330`, `cdecl`: the class name),
/// `NiGetViewerString_ov9` (`00a7d550`, `cdecl`: label, float) and
/// `NiGetViewerString_ov10` (`00a7d5a0`, `cdecl`: label, pointer), engine
/// map names.
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const NI_GET_VIEWER_STRING_FLOAT: u32 = 0x00a7_d550;
const NI_GET_VIEWER_STRING_POINTER: u32 = 0x00a7_d5a0;
/// The viewer-string array's grow (`0096ad30`; `thiscall`: new size) and
/// store (`0096ae90`; `thiscall`: index, address of the string).
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
/// `"m_fFloatValue"` and `"m_spFloatData"`.
const LABEL_FLOAT_VALUE: u32 = 0x0109_703c;
const LABEL_FLOAT_DATA: u32 = 0x0109_702c;
/// `NiStream::m_pkOstr` (Xbox PDB) +0x250 and the output stream's
/// `m_pfnWrite` (+0xc, `cdecl` like the read function).
const STREAM_OUTPUT: u32 = 0x250;
const BINARY_STREAM_WRITE: u32 = 0xc;

/// Releases the float data (when set) and clears the pointer.
fn clear_float_data(e: &mut Engine, this: u32) {
    let data = e.mem.u32(this.wrapping_add(FLOAT_DATA));
    if data != 0 {
        release(e, data);
        e.mem.set_u32(this.wrapping_add(FLOAT_DATA), 0);
    }
}

/// Appends `text` to the viewer-string array at `strings`: when the 16-bit
/// size (+0xA) has reached the capacity (+0x8) the array grows to the size
/// plus the step (+0xE) first; the string goes in at the old size through a
/// stack word holding it.
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

// Translated from 00a3c3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatInterpolator::GetChannelPosed` (Xbox PDB): false with float
/// data, otherwise whether the value differs from the "no value" float
/// (`FUCOMPP`: a NaN counts as different). The channel word is not read.
pub fn ni_float_interpolator_get_channel_posed(e: &mut Engine, this: Ptr, _unused_1: u32) -> bool {
    let this = this.addr();
    if e.mem.u32(this.wrapping_add(FLOAT_DATA)) != 0 {
        return false;
    }
    let value = x87_load32(e.mem.u32(this.wrapping_add(VALUE)));
    let invalid = x87_load32(e.mem.u32(INVALID_FLOAT));
    invalid != value
}

// Translated from 00a3c420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatInterpolator::Collapse` (Xbox PDB). Without float data nothing
/// happens. With no keys the data is released and the value becomes the
/// "no value" float. With one key, or with linear (1) or step (5) keys that
/// all hold the first key's value (`FUCOMPP`, so a NaN counts as
/// different; the stride `m_ucKeySize`, the value at +4 of a key), the data
/// is released and the first key's value (stored once as a float) becomes
/// the value; any other key type, or a differing key, changes nothing.
pub fn ni_float_interpolator_collapse(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    let data = e.mem.u32(this.wrapping_add(FLOAT_DATA));
    if data == 0 {
        return;
    }
    let key_type = e.mem.u32(data.wrapping_add(0x10));
    let key_size = e.mem.u8(data.wrapping_add(0x14));
    let num_keys = e.mem.u32(data.wrapping_add(8));
    let keys = e.mem.u32(data.wrapping_add(0xc));
    if num_keys == 0 {
        // `ESI` is the data pointer here, never null.
        release(e, data);
        e.mem.set_u32(this.wrapping_add(FLOAT_DATA), 0);
        let invalid = x87_quiet_bits(e.mem.u32(INVALID_FLOAT));
        e.mem.set_u32(this.wrapping_add(VALUE), invalid);
        return;
    }
    let first = x87_quiet_bits(e.mem.u32(keys.wrapping_add(4)));
    if num_keys != 1 {
        if key_type != 1 && key_type != 5 {
            return;
        }
        let mut same = true;
        let mut index = 1u32;
        while index < num_keys {
            let key = u32::from(key_size)
                .wrapping_mul(index)
                .wrapping_add(keys)
                .wrapping_add(4);
            let value = x87_load32(e.mem.u32(key));
            if x87_load32(first) != value {
                same = false;
            }
            index = index.wrapping_add(1);
            if !same {
                return;
            }
        }
        // Always set here: a difference returns inside the loop.
        if !same {
            return;
        }
    }
    clear_float_data(e, this);
    e.mem.set_u32(this.wrapping_add(VALUE), first);
}

// Translated from 00a3c500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatInterpolator::SetPoseValue` (Xbox PDB): releases and clears the
/// float data, then stores `value` (an x87 copy).
pub fn ni_float_interpolator_set_pose_value(e: &mut Engine, this: Ptr, value: f32) {
    let this = this.addr();
    clear_float_data(e, this);
    e.mem
        .set_u32(this.wrapping_add(VALUE), x87_quiet_bits(value.to_bits()));
}

// Translated from 00a3c540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatInterpolator::GuaranteeTimeRange` (Xbox PDB): with float data,
/// `NiFloatData::GuaranteeKeysAtStartAndEnd(start, end)` (both floats
/// copied through the x87 stack).
pub fn ni_float_interpolator_guarantee_time_range(e: &mut Engine, this: Ptr, start: f32, end: f32) {
    let data = e.mem.u32(this.addr().wrapping_add(FLOAT_DATA));
    if data == 0 {
        return;
    }
    e.call(
        FLOAT_DATA_GUARANTEE_KEYS,
        &args![
            data,
            x87_quiet_bits(start.to_bits()),
            x87_quiet_bits(end.to_bits())
        ],
    );
}

// Translated from 00a3c5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatInterpolator::IsEqual` (Xbox PDB): the base `IsEqual`
/// (`00c85800`), equal values (`FUCOMPP`: a NaN is never equal), and either
/// both without float data or both with float data that the first's
/// `IsEqual` (virtual slot +0x5c) finds equal.
pub fn ni_float_interpolator_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e.call(BASE_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let this = this.addr();
    let other = other.addr();
    let value = x87_load32(e.mem.u32(this.wrapping_add(VALUE)));
    let other_value = x87_load32(e.mem.u32(other.wrapping_add(VALUE)));
    if other_value != value {
        return false;
    }
    let data = e.mem.u32(this.wrapping_add(FLOAT_DATA));
    let other_data = e.mem.u32(other.wrapping_add(FLOAT_DATA));
    if data != 0 {
        if other_data == 0 {
            return false;
        }
    } else if other_data != 0 {
        return false;
    }
    if data == 0 {
        return true;
    }
    let other_data = e.mem.u32(other.wrapping_add(FLOAT_DATA));
    e.vcall(data, SLOT_IS_EQUAL, &args![other_data]).bool()
}

// Translated from 00a3c640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatInterpolator::GetSequenceInterpolator` (Xbox PDB): the base
/// class's copy (`NiInterpolator::GetSequenceInterpolator`, `00a57050`,
/// with `start` and `end` copied through the x87 stack). With float data
/// and a copy, `NiFloatData::GetSequenceData(start, end, &sequence)` fills
/// a pointer starting at 0, which `007fc2b0` assigns to the copy's float
/// data; the copy's last index becomes 0 and the local pointer is
/// released. Returns the copy.
pub fn ni_float_interpolator_get_sequence_interpolator(
    e: &mut Engine,
    this: Ptr,
    start: f32,
    end: f32,
) -> Ptr {
    let start = x87_quiet_bits(start.to_bits());
    let end = x87_quiet_bits(end.to_bits());
    let copy = e
        .call(
            INTERPOLATOR_GET_SEQUENCE_INTERPOLATOR,
            &args![this, start, end],
        )
        .u32();
    let data = e.mem.u32(this.addr().wrapping_add(FLOAT_DATA));
    if data == 0 || copy == 0 {
        return Ptr::new(copy);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), 0);
        e.call(FLOAT_DATA_GET_SEQUENCE_DATA, &args![data, start, end, slot]);
        let sequence = e.mem.u32(slot.addr());
        e.call(SET_POINTER_AT_0X10, &args![copy, sequence]);
        let sequence = e.mem.u32(slot.addr());
        e.mem.set_u32(copy.wrapping_add(LAST_INDEX), 0);
        if sequence != 0 {
            release(e, sequence);
        }
    });
    Ptr::new(copy)
}

// Translated from 00a3c880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatInterpolator::SaveBinary` (Xbox PDB): the base call
/// (`00a95ca0`), the value (4 bytes through the output stream's write
/// function, the component size in the argument's own stack word), then the
/// stream's `SaveLinkID` (virtual slot +0x30) of the float data.
pub fn ni_float_interpolator_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(BASE_SAVE_BINARY, &args![this, stream]);
    let this = this.addr();
    let stream = stream.addr();
    e.with_stack(4, |e, slot| {
        let output = e.mem.u32(stream.wrapping_add(STREAM_OUTPUT));
        let write = e.mem.u32(output.wrapping_add(BINARY_STREAM_WRITE));
        e.mem.set_u32(slot.addr(), 4);
        let bytes = e
            .call(
                write,
                &args![output, this.wrapping_add(VALUE), 4u32, slot, 1u32],
            )
            .u32();
        let position = e.mem.u32(output.wrapping_add(BINARY_STREAM_POSITION));
        e.mem.set_u32(
            output.wrapping_add(BINARY_STREAM_POSITION),
            position.wrapping_add(bytes),
        );
    });
    let data = e.mem.u32(this.wrapping_add(FLOAT_DATA));
    e.vcall(stream, SLOT_SAVE_LINK_ID, &args![data]);
}

// Translated from 00a3c940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatInterpolator::GetViewerStrings` (Xbox PDB): the
/// `NiKeyBasedInterpolator` strings, then three more: the class name (the
/// `NiRTTI` record's first word through `NiGetViewerString`), the value
/// (`NiGetViewerString_ov9("m_fFloatValue", value)`, the float copied
/// through the x87 stack) and the float data pointer
/// (`NiGetViewerString_ov10("m_spFloatData", data)`).
pub fn ni_float_interpolator_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    let this_addr = this.addr();
    let strings = strings.addr();
    e.call(KEY_BASED_GET_VIEWER_STRINGS, &args![this, strings]);
    let name = e.mem.u32(NI_FLOAT_INTERPOLATOR_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![name]).u32();
    add_viewer_string(e, strings, text);
    let value = x87_quiet_bits(e.mem.u32(this_addr.wrapping_add(VALUE)));
    let text = e
        .call(NI_GET_VIEWER_STRING_FLOAT, &args![LABEL_FLOAT_VALUE, value])
        .u32();
    add_viewer_string(e, strings, text);
    let data = e.mem.u32(this_addr.wrapping_add(FLOAT_DATA));
    let text = e
        .call(NI_GET_VIEWER_STRING_POINTER, &args![LABEL_FLOAT_DATA, data])
        .u32();
    add_viewer_string(e, strings, text);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a3c380, ni_float_interpolator_get_rtti() -> Ptr),
        entry!(0x00a3_c390, ni_float_interpolator_construct(Ptr, f32) -> Ptr),
        entry!(0x00a3_c6d0, ni_float_interpolator_update(Ptr, f32, u32, Ptr) -> bool),
        entry!(
            0x00a3_c780,
            ni_float_interpolator_copy_members(Ptr, Ptr, u32)
        ),
        entry!(0x00a3_c800, ni_float_interpolator_load_binary(Ptr, Ptr)),
        entry!(0x00a3_c8d0, ni_float_interpolator_create_clone(Ptr, u32) -> Ptr),
        entry!(0x00a3_c3f0, ni_float_interpolator_get_channel_posed(Ptr, u32) -> bool),
        entry!(0x00a3_c420, ni_float_interpolator_collapse(Ptr)),
        entry!(0x00a3_c500, ni_float_interpolator_set_pose_value(Ptr, f32)),
        entry!(
            0x00a3_c540,
            ni_float_interpolator_guarantee_time_range(Ptr, f32, f32)
        ),
        entry!(0x00a3_c5e0, ni_float_interpolator_is_equal(Ptr, Ptr) -> bool),
        entry!(
            0x00a3_c640,
            ni_float_interpolator_get_sequence_interpolator(Ptr, f32, f32) -> Ptr
        ),
        entry!(0x00a3_c880, ni_float_interpolator_save_binary(Ptr, Ptr)),
        entry!(
            0x00a3_c940,
            ni_float_interpolator_get_viewer_strings(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

    const READ: u32 = 0x00ee_0400;
    const DELETE_THIS: u32 = 0x00ee_0500;
    const INVALID: f32 = -f32::MAX;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn logged(e: &mut Engine, log: &Log, address: u32, eax: u32) {
        let log = log.clone();
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            ret(eax)
        });
    }

    /// The `Interlocked` import slots, counting in memory.
    fn interlocked(e: &mut Engine) {
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            ret(count)
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            ret(count)
        });
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        interlocked(&mut e);
        e.map(INVALID_FLOAT, 4);
        e.mem.set_f32(INVALID_FLOAT, INVALID);
        e
    }

    /// A reference-counted object with count `count` whose `DeleteThis` is
    /// `DELETE_THIS`.
    fn counted(e: &mut Engine, count: u32) -> u32 {
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, DELETE_THIS);
        let object = e.mem.alloc(0x18);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, count);
        object
    }

    #[test]
    fn ni_float_interpolator_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a3c380, &args![]).u32(),
            NI_FLOAT_INTERPOLATOR_RTTI
        );
    }

    #[test]
    fn construct_sets_the_value_vtable_and_clears_the_data() {
        let mut e = engine();
        let log = Log::default();
        logged(&mut e, &log, KEY_BASED_CONSTRUCT, 0);
        let this = e.mem.alloc(0x18);
        e.mem.write(this, &[0xaa; 0x18]);
        let back = e.call(0x00a3_c390, &args![this, 2.5f32]).u32();
        assert_eq!(back, this);
        assert_eq!(*log.borrow(), [(KEY_BASED_CONSTRUCT, vec![this])]);
        assert_eq!(e.mem.u32(this), NI_FLOAT_INTERPOLATOR_VTABLE);
        assert_eq!(e.mem.f32(this + VALUE), 2.5);
        assert_eq!(e.mem.u32(this + FLOAT_DATA), 0);
        assert_eq!(e.mem.u32(this + LAST_INDEX), 0);
        // A signalling NaN pose value is quieted by the x87 load.
        e.call(0x00a3_c390, &args![this, 0x7f80_0001u32]);
        assert_eq!(e.mem.u32(this + VALUE), 0x7fc0_0001);
    }

    #[test]
    fn update_at_the_last_time_reports_the_stored_value() {
        let mut e = engine();
        let this = e.mem.alloc(0x18);
        let out = e.mem.alloc(4);
        e.mem.set_f32(this + LAST_TIME, 1.5);
        e.mem.set_f32(this + VALUE, 7.0);
        assert!(e.call(0x00a3_c6d0, &args![this, 1.5f32, 0u32, out]).bool());
        assert_eq!(e.mem.f32(out), 7.0);
        // The invalid value still goes to `out`, but the result is false.
        e.mem.set_f32(this + VALUE, INVALID);
        assert!(!e.call(0x00a3_c6d0, &args![this, 1.5f32, 0u32, out]).bool());
        assert_eq!(e.mem.f32(out), INVALID);
    }

    #[test]
    fn update_at_a_new_time_interpolates_the_keys() {
        let mut e = engine();
        let log = Log::default();
        let l = log.clone();
        e.register_double(FLOAT_KEY_GEN_INTERP, move |_, a| {
            l.borrow_mut().push((FLOAT_KEY_GEN_INTERP, a.to_vec()));
            Ret {
                st0: 3.25,
                ..Ret::default()
            }
        });
        let this = e.mem.alloc(0x18);
        let data = e.mem.alloc(0x18);
        let out = e.mem.alloc(4);
        e.mem.set_u32(data + 8, 4);
        e.mem.set_u32(data + 0xc, 0x6000);
        e.mem.set_u32(data + 0x10, 2);
        e.mem.set_u8(data + 0x14, 0x14);
        e.mem.set_u32(this + FLOAT_DATA, data);
        e.mem.set_f32(this + LAST_TIME, 1.0);
        assert!(e.call(0x00a3_c6d0, &args![this, 2.0f32, 0u32, out]).bool());
        assert_eq!(
            *log.borrow(),
            [(
                FLOAT_KEY_GEN_INTERP,
                vec![
                    2.0f32.to_bits(),
                    0x6000,
                    2,
                    4,
                    this + LAST_INDEX,
                    (this & 0xffff_ff00) | 0x14
                ]
            )]
        );
        assert_eq!(e.mem.f32(this + VALUE), 3.25);
        assert_eq!(e.mem.f32(out), 3.25);
        assert_eq!(e.mem.f32(this + LAST_TIME), 2.0);

        // No keys: the stored value is reported unchanged; an invalid one
        // leaves `out` and the time alone.
        e.mem.set_u32(data + 8, 0);
        e.mem.set_f32(this + VALUE, INVALID);
        e.mem.set_f32(out, 9.0);
        assert!(!e.call(0x00a3_c6d0, &args![this, 5.0f32, 0u32, out]).bool());
        assert_eq!(e.mem.f32(out), 9.0);
        assert_eq!(e.mem.f32(this + LAST_TIME), 2.0);
        assert_eq!(log.borrow().len(), 1);
        // No data at all, a valid value.
        e.mem.set_u32(this + FLOAT_DATA, 0);
        e.mem.set_f32(this + VALUE, 6.0);
        assert!(e.call(0x00a3_c6d0, &args![this, 5.0f32, 0u32, out]).bool());
        assert_eq!(e.mem.f32(out), 6.0);
        assert_eq!(e.mem.f32(this + LAST_TIME), 5.0);
    }

    #[test]
    fn copy_members_copies_value_index_and_counts_the_data() {
        let mut e = engine();
        let log = Log::default();
        logged(&mut e, &log, KEY_BASED_COPY_MEMBERS, 0);
        logged(&mut e, &log, DELETE_THIS, 0);
        let this = e.mem.alloc(0x18);
        let destination = e.mem.alloc(0x18);
        let data = counted(&mut e, 1);
        let old = counted(&mut e, 1);
        e.mem.set_f32(this + VALUE, 4.0);
        e.mem.set_u32(this + FLOAT_DATA, data);
        e.mem.set_u32(this + LAST_INDEX, 3);
        e.mem.set_u32(destination + FLOAT_DATA, old);
        e.call(0x00a3_c780, &args![this, destination, 0x9000u32]);
        assert_eq!(
            *log.borrow(),
            [
                (KEY_BASED_COPY_MEMBERS, vec![this, destination, 0x9000]),
                (DELETE_THIS, vec![old]),
            ]
        );
        assert_eq!(e.mem.f32(destination + VALUE), 4.0);
        assert_eq!(e.mem.u32(destination + FLOAT_DATA), data);
        assert_eq!(e.mem.u32(data + 4), 2);
        assert_eq!(e.mem.u32(destination + LAST_INDEX), 3);
        // Same data already: no count change.
        e.call(0x00a3_c780, &args![this, destination, 0x9000u32]);
        assert_eq!(e.mem.u32(data + 4), 2);
    }

    #[test]
    fn load_binary_reads_the_value_and_resolves_the_data() {
        let mut e = engine();
        let log = Log::default();
        logged(&mut e, &log, KEY_BASED_LOAD_BINARY, 0);
        let data = counted(&mut e, 0);
        logged(&mut e, &log, RESOLVE_LINK_ID, data);
        let stream = e.mem.alloc(0x5c4);
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(stream + STREAM_INPUT, input);
        e.mem.set_u32(input + BINARY_STREAM_READ, READ);
        let l = log.clone();
        e.register_double(READ, move |e, a| {
            l.borrow_mut()
                .push((READ, vec![a[0], a[1], a[2], e.mem.u32(a[3]), a[4]]));
            e.mem.set_f32(a[1], 0.75);
            ret(4)
        });
        let this = e.mem.alloc(0x18);
        e.call(0x00a3_c800, &args![this, stream]);
        assert_eq!(
            *log.borrow(),
            [
                (KEY_BASED_LOAD_BINARY, vec![this, stream]),
                (READ, vec![input, this + VALUE, 4, 4, 1]),
                (RESOLVE_LINK_ID, vec![stream]),
            ]
        );
        assert_eq!(e.mem.f32(this + VALUE), 0.75);
        assert_eq!(e.mem.u32(input + BINARY_STREAM_POSITION), 4);
        assert_eq!(e.mem.u32(this + FLOAT_DATA), data);
        assert_eq!(e.mem.u32(data + 4), 1);
    }

    #[test]
    fn create_clone_builds_a_copy_or_copies_into_null() {
        let mut e = engine();
        let log = Log::default();
        let block = e.mem.alloc(0x18);
        logged(&mut e, &log, NI_ALLOCATE, block);
        logged(&mut e, &log, KEY_BASED_CONSTRUCT, 0);
        logged(&mut e, &log, KEY_BASED_COPY_MEMBERS, 0);
        let this = e.mem.alloc(0x18);
        e.mem.set_f32(this + VALUE, 1.25);
        e.mem.set_u32(this + LAST_INDEX, 7);
        assert_eq!(e.call(0x00a3_c8d0, &args![this, 0x9000u32]).u32(), block);
        assert_eq!(e.mem.u32(block), NI_FLOAT_INTERPOLATOR_VTABLE);
        assert_eq!(e.mem.f32(block + VALUE), 1.25);
        assert_eq!(e.mem.u32(block + LAST_INDEX), 7);
        assert_eq!(
            *log.borrow(),
            [
                (NI_ALLOCATE, vec![0x18]),
                (KEY_BASED_CONSTRUCT, vec![block]),
                (KEY_BASED_COPY_MEMBERS, vec![this, block, 0x9000]),
            ]
        );
        // A failed allocation copies into address 0.
        e.map(0, 0x100);
        logged(&mut e, &log, NI_ALLOCATE, 0);
        log.borrow_mut().clear();
        assert_eq!(e.call(0x00a3_c8d0, &args![this, 0x9000u32]).u32(), 0);
        assert_eq!(
            *log.borrow(),
            [
                (NI_ALLOCATE, vec![0x18]),
                (KEY_BASED_COPY_MEMBERS, vec![this, 0, 0x9000]),
            ]
        );
        assert_eq!(e.mem.f32(VALUE), 1.25);
    }
}

#[cfg(test)]
mod n003_tests {
    use super::*;

    type Log = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

    const DELETE_THIS: u32 = 0x00ee_0500;
    const DATA_IS_EQUAL: u32 = 0x00ee_0600;
    const SAVE_LINK_ID: u32 = 0x00ee_0800;
    const WRITE: u32 = 0x00ee_0900;
    const INVALID: u32 = 0xff7f_ffff;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn logged(e: &mut Engine, log: &Log, address: u32, eax: u32) {
        let log = log.clone();
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            ret(eax)
        });
    }

    fn calls(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    fn engine(log: &Log) -> Engine {
        let mut e = Engine::new();
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            ret(count)
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            ret(count)
        });
        logged(&mut e, log, DELETE_THIS, 0);
        e.map(INVALID_FLOAT, 4);
        e.mem.set_u32(INVALID_FLOAT, INVALID);
        e
    }

    /// A counted object whose vtable has `DeleteThis` and `IsEqual` doubles.
    fn counted(e: &mut Engine, count: u32) -> u32 {
        let vtable = e.mem.alloc(0x60);
        e.mem.set_u32(vtable + SLOT_DELETE_THIS, DELETE_THIS);
        e.mem.set_u32(vtable + SLOT_IS_EQUAL, DATA_IS_EQUAL);
        let object = e.mem.alloc(0x18);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, count);
        object
    }

    #[test]
    fn get_channel_posed_needs_no_data_and_a_valid_value() {
        let log = Log::default();
        let mut e = engine(&log);
        let this = e.mem.alloc(0x18);
        e.mem.set_f32(this + VALUE, 2.0);
        assert!(e.call(0x00a3_c3f0, &args![this, 0u32]).bool());
        e.mem.set_u32(this + VALUE, 0x7fc0_0000);
        assert!(e.call(0x00a3_c3f0, &args![this, 0u32]).bool());
        e.mem.set_u32(this + VALUE, INVALID);
        assert!(!e.call(0x00a3_c3f0, &args![this, 0u32]).bool());
        e.mem.set_f32(this + VALUE, 2.0);
        e.mem.set_u32(this + FLOAT_DATA, 0x6000);
        assert!(!e.call(0x00a3_c3f0, &args![this, 0u32]).bool());
    }

    /// An interpolator whose data holds keys (stride 8, value at +4) of type
    /// `key_type` with the value bits `values`.
    fn keyed(e: &mut Engine, key_type: u32, values: &[u32]) -> (u32, u32) {
        let this = e.mem.alloc(0x18);
        let data = counted(e, 1);
        let keys = e.mem.alloc(8 * values.len() as u32 + 4);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_u32(keys + 8 * i as u32 + 4, *v);
        }
        e.mem.set_u32(data + 8, values.len() as u32);
        e.mem.set_u32(data + 0xc, keys);
        e.mem.set_u32(data + 0x10, key_type);
        e.mem.set_u8(data + 0x14, 8);
        e.mem.set_u32(this + FLOAT_DATA, data);
        e.mem.set_f32(this + VALUE, 9.0);
        (this, data)
    }

    #[test]
    fn collapse_keeps_a_constant_value_and_drops_the_data() {
        let log = Log::default();
        let mut e = engine(&log);
        let one = 1.0f32.to_bits();
        let (this, data) = keyed(&mut e, 5, &[one, one, one]);
        e.call(0x00a3_c420, &args![this]);
        assert_eq!(e.mem.u32(this + FLOAT_DATA), 0);
        assert_eq!(calls(&log, DELETE_THIS), vec![vec![data]]);
        assert_eq!(e.mem.f32(this + VALUE), 1.0);
        // A different key, or a NaN, keeps the data.
        for values in [[one, 2.0f32.to_bits()], [0x7fc0_0000, 0x7fc0_0000]] {
            let (this, data) = keyed(&mut e, 1, &values);
            e.call(0x00a3_c420, &args![this]);
            assert_eq!(e.mem.u32(this + FLOAT_DATA), data);
            assert_eq!(e.mem.f32(this + VALUE), 9.0);
        }
        // Another key type: unchanged.
        let (this, data) = keyed(&mut e, 2, &[one, one]);
        e.call(0x00a3_c420, &args![this]);
        assert_eq!(e.mem.u32(this + FLOAT_DATA), data);
        // One key: its value, quieted by the x87 copy.
        let (this, _) = keyed(&mut e, 3, &[0x7f80_0001]);
        e.call(0x00a3_c420, &args![this]);
        assert_eq!(e.mem.u32(this + FLOAT_DATA), 0);
        assert_eq!(e.mem.u32(this + VALUE), 0x7fc0_0001);
        // No keys: the invalid value.
        let (this, _) = keyed(&mut e, 1, &[]);
        e.call(0x00a3_c420, &args![this]);
        assert_eq!(e.mem.u32(this + FLOAT_DATA), 0);
        assert_eq!(e.mem.u32(this + VALUE), INVALID);
        // No data: nothing.
        e.call(0x00a3_c420, &args![this]);
        assert_eq!(calls(&log, DELETE_THIS).len(), 3);
    }

    #[test]
    fn set_pose_value_releases_the_data() {
        let log = Log::default();
        let mut e = engine(&log);
        let this = e.mem.alloc(0x18);
        let data = counted(&mut e, 1);
        e.mem.set_u32(this + FLOAT_DATA, data);
        e.call(0x00a3_c500, &args![this, 0x7f80_0002u32]);
        assert_eq!(calls(&log, DELETE_THIS), vec![vec![data]]);
        assert_eq!(e.mem.u32(this + FLOAT_DATA), 0);
        assert_eq!(e.mem.u32(this + VALUE), 0x7fc0_0002);
        e.call(0x00a3_c500, &args![this, 3.5f32]);
        assert_eq!(e.mem.f32(this + VALUE), 3.5);
        assert_eq!(calls(&log, DELETE_THIS).len(), 1);
    }

    #[test]
    fn guarantee_time_range_forwards_to_the_data() {
        let log = Log::default();
        let mut e = engine(&log);
        logged(&mut e, &log, FLOAT_DATA_GUARANTEE_KEYS, 0);
        let this = e.mem.alloc(0x18);
        e.call(0x00a3_c540, &args![this, 1.0f32, 2.0f32]);
        assert!(calls(&log, FLOAT_DATA_GUARANTEE_KEYS).is_empty());
        e.mem.set_u32(this + FLOAT_DATA, 0x6000);
        e.call(0x00a3_c540, &args![this, 1.0f32, 0xff80_0001u32]);
        assert_eq!(
            calls(&log, FLOAT_DATA_GUARANTEE_KEYS),
            vec![vec![0x6000, 1.0f32.to_bits(), 0xffc0_0001]]
        );
    }

    #[test]
    fn is_equal_compares_base_value_and_data() {
        let log = Log::default();
        let mut e = engine(&log);
        e.register(BASE_IS_EQUAL, |e, a| ret(e.mem.u32(a[0] + 8)));
        let mut answers = vec![1u32, 0];
        let l = log.clone();
        e.register_double(DATA_IS_EQUAL, move |_, a| {
            l.borrow_mut().push((DATA_IS_EQUAL, a.to_vec()));
            ret(answers.remove(0))
        });
        let this = e.mem.alloc(0x18);
        let other = e.mem.alloc(0x18);
        e.mem.set_u32(this + 8, 1);
        e.mem.set_f32(this + VALUE, 2.0);
        e.mem.set_f32(other + VALUE, 2.0);
        assert!(e.call(0x00a3_c5e0, &args![this, other]).bool());
        e.mem.set_u32(this + 8, 0);
        assert!(!e.call(0x00a3_c5e0, &args![this, other]).bool());
        e.mem.set_u32(this + 8, 1);
        // Different values, and a NaN never equal to itself.
        e.mem.set_f32(other + VALUE, 3.0);
        assert!(!e.call(0x00a3_c5e0, &args![this, other]).bool());
        e.mem.set_u32(this + VALUE, 0x7fc0_0000);
        e.mem.set_u32(other + VALUE, 0x7fc0_0000);
        assert!(!e.call(0x00a3_c5e0, &args![this, other]).bool());
        e.mem.set_f32(this + VALUE, 2.0);
        e.mem.set_f32(other + VALUE, 2.0);
        let data = counted(&mut e, 1);
        let other_data = counted(&mut e, 1);
        e.mem.set_u32(other + FLOAT_DATA, other_data);
        assert!(!e.call(0x00a3_c5e0, &args![this, other]).bool());
        e.mem.set_u32(this + FLOAT_DATA, data);
        e.mem.set_u32(other + FLOAT_DATA, 0);
        assert!(!e.call(0x00a3_c5e0, &args![this, other]).bool());
        e.mem.set_u32(other + FLOAT_DATA, other_data);
        assert!(e.call(0x00a3_c5e0, &args![this, other]).bool());
        assert!(!e.call(0x00a3_c5e0, &args![this, other]).bool());
        assert_eq!(calls(&log, DATA_IS_EQUAL), vec![vec![data, other_data]; 2]);
    }

    #[test]
    fn get_sequence_interpolator_assigns_the_sequence_data() {
        let log = Log::default();
        let mut e = engine(&log);
        let copy = e.mem.alloc(0x18);
        e.mem.set_u32(copy + LAST_INDEX, 3);
        let sequence = counted(&mut e, 1);
        logged(&mut e, &log, INTERPOLATOR_GET_SEQUENCE_INTERPOLATOR, copy);
        logged(&mut e, &log, SET_POINTER_AT_0X10, 0);
        let l = log.clone();
        e.register_double(FLOAT_DATA_GET_SEQUENCE_DATA, move |e, a| {
            l.borrow_mut().push((
                FLOAT_DATA_GET_SEQUENCE_DATA,
                vec![a[0], a[1], a[2], e.mem.u32(a[3])],
            ));
            e.mem.set_u32(a[3], sequence);
            ret(0)
        });
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this + FLOAT_DATA, 0x6000);
        let answer = e
            .call(0x00a3_c640, &args![this, 1.0f32, 0x7f80_0003u32])
            .u32();
        assert_eq!(answer, copy);
        assert_eq!(
            calls(&log, INTERPOLATOR_GET_SEQUENCE_INTERPOLATOR),
            vec![vec![this, 1.0f32.to_bits(), 0x7fc0_0003]]
        );
        assert_eq!(
            calls(&log, FLOAT_DATA_GET_SEQUENCE_DATA),
            vec![vec![0x6000, 1.0f32.to_bits(), 0x7fc0_0003, 0]]
        );
        assert_eq!(calls(&log, SET_POINTER_AT_0X10), vec![vec![copy, sequence]]);
        assert_eq!(e.mem.u32(copy + LAST_INDEX), 0);
        // The local pointer's release.
        assert_eq!(calls(&log, DELETE_THIS), vec![vec![sequence]]);
        // No data, or no copy: only the base copy.
        e.mem.set_u32(this + FLOAT_DATA, 0);
        e.call(0x00a3_c640, &args![this, 1.0f32, 2.0f32]);
        e.mem.set_u32(this + FLOAT_DATA, 0x6000);
        logged(&mut e, &log, INTERPOLATOR_GET_SEQUENCE_INTERPOLATOR, 0);
        assert_eq!(e.call(0x00a3_c640, &args![this, 1.0f32, 2.0f32]).u32(), 0);
        assert_eq!(calls(&log, FLOAT_DATA_GET_SEQUENCE_DATA).len(), 1);
    }

    #[test]
    fn save_binary_writes_the_value_and_links_the_data() {
        let log = Log::default();
        let mut e = engine(&log);
        logged(&mut e, &log, BASE_SAVE_BINARY, 0);
        logged(&mut e, &log, SAVE_LINK_ID, 0);
        let l = log.clone();
        e.register_double(WRITE, move |e, a| {
            l.borrow_mut()
                .push((WRITE, vec![a[0], a[1], a[2], e.mem.u32(a[3]), a[4]]));
            ret(4)
        });
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + SLOT_SAVE_LINK_ID, SAVE_LINK_ID);
        let output = e.mem.alloc(0x10);
        e.mem.set_u32(output + BINARY_STREAM_WRITE, WRITE);
        e.mem.set_u32(output + BINARY_STREAM_POSITION, 8);
        let stream = e.mem.alloc(0x260);
        e.mem.set_u32(stream, vtable);
        e.mem.set_u32(stream + STREAM_OUTPUT, output);
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this + FLOAT_DATA, 0x6000);
        e.call(0x00a3_c880, &args![this, stream]);
        assert_eq!(
            *log.borrow(),
            [
                (BASE_SAVE_BINARY, vec![this, stream]),
                (WRITE, vec![output, this + VALUE, 4, 4, 1]),
                (SAVE_LINK_ID, vec![stream, 0x6000]),
            ]
        );
        assert_eq!(e.mem.u32(output + BINARY_STREAM_POSITION), 12);
    }

    #[test]
    fn get_viewer_strings_adds_name_value_and_data() {
        let log = Log::default();
        let mut e = engine(&log);
        logged(&mut e, &log, KEY_BASED_GET_VIEWER_STRINGS, 0);
        logged(&mut e, &log, NI_GET_VIEWER_STRING, 0x7100);
        logged(&mut e, &log, NI_GET_VIEWER_STRING_FLOAT, 0x7200);
        logged(&mut e, &log, NI_GET_VIEWER_STRING_POINTER, 0x7300);
        logged(&mut e, &log, STRINGS_GROW, 0);
        let l = log.clone();
        e.register_double(STRINGS_SET_AT, move |e, a| {
            l.borrow_mut()
                .push((STRINGS_SET_AT, vec![a[0], a[1], e.mem.u32(a[2])]));
            let size = e.mem.u16(a[0] + 0xa) + 1;
            e.mem.set_u16(a[0] + 0xa, size);
            ret(0)
        });
        e.map(NI_FLOAT_INTERPOLATOR_RTTI, 4);
        e.mem.set_u32(NI_FLOAT_INTERPOLATOR_RTTI, 0x5151);
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this + VALUE, 0x7f80_0001);
        e.mem.set_u32(this + FLOAT_DATA, 0x6000);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 2);
        e.mem.set_u16(strings + 0xa, 1);
        e.mem.set_u16(strings + 0xe, 4);
        e.call(0x00a3_c940, &args![this, strings]);
        assert_eq!(
            *log.borrow(),
            [
                (KEY_BASED_GET_VIEWER_STRINGS, vec![this, strings]),
                (NI_GET_VIEWER_STRING, vec![0x5151]),
                (STRINGS_SET_AT, vec![strings, 1, 0x7100]),
                (
                    NI_GET_VIEWER_STRING_FLOAT,
                    vec![LABEL_FLOAT_VALUE, 0x7fc0_0001]
                ),
                (STRINGS_GROW, vec![strings, 6]),
                (STRINGS_SET_AT, vec![strings, 2, 0x7200]),
                (NI_GET_VIEWER_STRING_POINTER, vec![LABEL_FLOAT_DATA, 0x6000]),
                (STRINGS_GROW, vec![strings, 7]),
                (STRINGS_SET_AT, vec![strings, 3, 0x7300]),
            ]
        );
    }
}
