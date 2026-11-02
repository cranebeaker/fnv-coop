//! `NiAnimation/niboolinterpolator.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiBoolInterpolator::ms_RTTI` (the class's `NiRTTI` record).
const NI_BOOL_INTERPOLATOR_RTTI: u32 = 0x011f_404c;

// Translated from 00a532c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolInterpolator::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_bool_interpolator_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_BOOL_INTERPOLATOR_RTTI)
}

layout! {
    /// `NiBoolInterpolator` (Xbox PDB, 0x18 bytes; the same offsets on PC).
    pub struct NiBoolInterpolator: 0x18 {
        /// `m_fLastTime` (Xbox PDB, `NiInterpolator`).
        0x08 m_fLastTime: u32,
        /// `m_bBoolValue` (Xbox PDB).
        0x0C m_bBoolValue: u8,
        /// `m_spBoolData` (Xbox PDB): `NiPointer<NiBoolData>`.
        0x10 m_spBoolData: u32,
        /// `m_uiLastIdx` (Xbox PDB).
        0x14 m_uiLastIdx: u32,
    }
}

/// The vtable of `NiBoolInterpolator` (RTTI `.?AVNiBoolInterpolator@@`).
const VTABLE: u32 = 0x0109_a0b4;
/// The byte that marks an unset value (`NiBoolInterpolator`'s invalid
/// bool, `0109a914`).
const INVALID_BOOL: u32 = 0x0109_a914;
/// `NiKeyBasedInterpolator::NiKeyBasedInterpolator` (`00a3ca10`, engine
/// map; `thiscall`).
const KEY_BASED_INTERPOLATOR_CONSTRUCTOR: u32 = 0x00a3_ca10;
/// The base destructor `00a3ca30` (`thiscall`; stores the
/// `NiKeyBasedInterpolator` vtable and continues in `00a57010`).
const KEY_BASED_INTERPOLATOR_DESTRUCTOR: u32 = 0x00a3_ca30;
/// `NiNew` allocation (`00aa13e0`, `cdecl`, the size).
const NI_NEW: u32 = 0x00aa_13e0;
/// `NiDelete` of a sized block (`00aa1460`, `cdecl`: block, size).
const NI_DELETE: u32 = 0x00aa_1460;
/// `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `NiBoolKey::GenInterp` (`00a29250`, engine map; `cdecl`, six words:
/// time, keys, key type, key count, `&last_index`, key size).
const BOOL_KEY_GEN_INTERP: u32 = 0x00a2_9250;
/// The base `CopyMembers` (`00aa0ab0`, a jump to `00aa1000`; `thiscall`,
/// two words).
const BASE_COPY_MEMBERS: u32 = 0x00aa_0ab0;
/// `NiKeyBasedInterpolator::LoadBinary` (`00aa0ac0`, engine map; a jump;
/// `thiscall`, one word).
const BASE_LOAD_BINARY: u32 = 0x00aa_0ac0;
/// `NiStream::ResolveLinkID` (`00a64620`, engine map; `thiscall`).
const STREAM_RESOLVE_LINK_ID: u32 = 0x00a6_4620;

/// Releases a reference-counted object (`InterlockedDecrement` of +4, its
/// vtable +4 deleting it at zero).
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object + 4]).u32() == 0 {
        e.vcall(object, 4, &args![]);
    }
}

/// `m_spBoolData = data` as `NiPointer` assignment does: when it changes,
/// the old object is released and the new one gains a reference.
fn set_bool_data(e: &mut Engine, this: Ptr<NiBoolInterpolator>, data: u32) {
    let old = e.get(this, NiBoolInterpolator::m_spBoolData);
    if old == data {
        return;
    }
    if old != 0 {
        release(e, old);
    }
    e.set(this, NiBoolInterpolator::m_spBoolData, data);
    if data != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![data + 4]);
    }
}

// Translated from 00a53500 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The class's `CreateObject` (`cdecl`, no words; Ghidra has no function
/// here and the engine map no name): a 0x18-byte block (`00aa13e0`) set up
/// as a `NiBoolInterpolator` (base constructor, vtable, the invalid value,
/// no data, index 0), or null when the allocation fails.
pub fn fn_00a53500(e: &mut Engine) -> Ptr<NiBoolInterpolator> {
    let block = e.call(NI_NEW, &args![0x18u32]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    let this = Ptr::<NiBoolInterpolator>::new(block);
    e.call(KEY_BASED_INTERPOLATOR_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(block, VTABLE);
    let invalid: u8 = e.global(INVALID_BOOL);
    e.set(this, NiBoolInterpolator::m_bBoolValue, invalid);
    e.set(this, NiBoolInterpolator::m_spBoolData, 0);
    e.set(this, NiBoolInterpolator::m_uiLastIdx, 0);
    this
}

// Translated from 00a530a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolInterpolator::GetChannelPosed` (Xbox PDB; `thiscall`, one word: the
/// channel, not read): true when the interpolator has no data and its value
/// is set (not the invalid byte).
pub fn ni_bool_interpolator_get_channel_posed(
    e: &mut Engine,
    this: Ptr<NiBoolInterpolator>,
    _unused_channel: u32,
) -> bool {
    if e.get(this, NiBoolInterpolator::m_spBoolData) != 0 {
        return false;
    }
    let invalid: u8 = e.global(INVALID_BOOL);
    e.get(this, NiBoolInterpolator::m_bBoolValue) != invalid
}

// Translated from 00a53280 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiBoolInterpolator(NiBoolData*)` constructor (`thiscall`, one word;
/// no Xbox PDB name): base constructor, vtable, the invalid value, the data
/// (a reference added) and index 0. Returns `this`.
pub fn fn_00a53280(
    e: &mut Engine,
    this: Ptr<NiBoolInterpolator>,
    data: u32,
) -> Ptr<NiBoolInterpolator> {
    e.call(KEY_BASED_INTERPOLATOR_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE);
    let invalid: u8 = e.global(INVALID_BOOL);
    e.set(this, NiBoolInterpolator::m_bBoolValue, invalid);
    e.set(this, NiBoolInterpolator::m_spBoolData, data);
    if data != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![data + 4]);
    }
    e.set(this, NiBoolInterpolator::m_uiLastIdx, 0);
    this
}

// Translated from 00a532d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolInterpolator::NiBoolInterpolator(bool)` (Xbox PDB, matched by the
/// engine map; `thiscall`, one word): base constructor, the value, vtable,
/// no data, index 0. Returns `this`.
pub fn ni_bool_interpolator_constructor(
    e: &mut Engine,
    this: Ptr<NiBoolInterpolator>,
    value: u8,
) -> Ptr<NiBoolInterpolator> {
    e.call(KEY_BASED_INTERPOLATOR_CONSTRUCTOR, &args![this]);
    e.set(this, NiBoolInterpolator::m_bBoolValue, value);
    e.mem.set_u32(this.addr(), VTABLE);
    e.set(this, NiBoolInterpolator::m_spBoolData, 0);
    e.set(this, NiBoolInterpolator::m_uiLastIdx, 0);
    this
}

// Translated from 00a533c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolInterpolator::Update` (Xbox PDB, matched by the engine map;
/// `thiscall (this; time, interp_target, value)`, `value` a `bool*`): at an
/// unchanged time (`FUCOMP` equal) the stored value is reported; otherwise,
/// with data and keys, the value becomes `NiBoolKey::GenInterp(time, keys,
/// type, count, &m_uiLastIdx, key size)` as a `bool` (the key-size word
/// carries the upper bytes of the stack slot the exe reuses, which held
/// `this`). An invalid value reports false (and leaves `m_fLastTime`); a
/// valid one is written to `*value`, answers true and, after a new
/// evaluation, records the time.
pub fn ni_bool_interpolator_update(
    e: &mut Engine,
    this: Ptr<NiBoolInterpolator>,
    time: f32,
    _unused_target: u32,
    value: Ptr,
) -> bool {
    let invalid: u8 = e.global(INVALID_BOOL);
    let last = x87_load32(e.get(this, NiBoolInterpolator::m_fLastTime));
    let now = x87_load32(time.to_bits());
    // FUCOMP, TEST AH,0x44 / JP: only an equal time reuses the value.
    if now == last {
        let stored = e.get(this, NiBoolInterpolator::m_bBoolValue);
        if stored == invalid {
            e.mem.set_u8(value.addr(), 0);
            return false;
        }
        e.mem.set_u8(value.addr(), u8::from(stored != 0));
        return true;
    }
    let data = e.get(this, NiBoolInterpolator::m_spBoolData);
    if data != 0 {
        let count = e.mem.u32(data + 8);
        let key_size = e.mem.u8(data + 0x14);
        let key_type = e.mem.u32(data + 0x10);
        let keys = e.mem.u32(data + 0xc);
        if count != 0 {
            let size_word = (this.addr() & 0xffff_ff00) | u32::from(key_size);
            let result = e
                .call(
                    BOOL_KEY_GEN_INTERP,
                    &args![
                        x87_quiet_bits(time.to_bits()),
                        keys,
                        key_type,
                        count,
                        this.addr() + 0x14,
                        size_word
                    ],
                )
                .bool();
            e.set(this, NiBoolInterpolator::m_bBoolValue, u8::from(result));
        }
    }
    let stored = e.get(this, NiBoolInterpolator::m_bBoolValue);
    if stored == invalid {
        e.mem.set_u8(value.addr(), 0);
        return false;
    }
    e.mem.set_u8(value.addr(), u8::from(stored != 0));
    e.set(
        this,
        NiBoolInterpolator::m_fLastTime,
        x87_quiet_bits(time.to_bits()),
    );
    true
}

// Translated from 00a53480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolInterpolator::CopyMembers` (`thiscall (this; dest, cloning)`; the
/// engine map gives this body no name): the base `CopyMembers`, then the
/// value, the data (as a `NiPointer` assignment) and the last index into
/// `dest`.
pub fn fn_00a53480(
    e: &mut Engine,
    this: Ptr<NiBoolInterpolator>,
    dest: Ptr<NiBoolInterpolator>,
    cloning: u32,
) {
    e.call(BASE_COPY_MEMBERS, &args![this, dest, cloning]);
    let value = e.get(this, NiBoolInterpolator::m_bBoolValue);
    e.set(dest, NiBoolInterpolator::m_bBoolValue, value);
    let data = e.get(this, NiBoolInterpolator::m_spBoolData);
    set_bool_data(e, dest, data);
    let index = e.get(this, NiBoolInterpolator::m_uiLastIdx);
    e.set(dest, NiBoolInterpolator::m_uiLastIdx, index);
}

// Translated from 00a53540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolInterpolator::_scalar_deleting_destructor_` (Xbox PDB, matched by
/// the engine map; `thiscall`, one word: flags): releases the data, runs
/// the base destructor (`00a3ca30`) and with flag bit 0 frees the 0x18-byte
/// block. Returns `this`.
pub fn ni_bool_interpolator_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiBoolInterpolator>,
    flags: u32,
) -> Ptr<NiBoolInterpolator> {
    let data = e.get(this, NiBoolInterpolator::m_spBoolData);
    if data != 0 {
        release(e, data);
    }
    e.call(KEY_BASED_INTERPOLATOR_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_DELETE, &args![this, 0x18u32]);
    }
    this
}

// Translated from 00a53590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolInterpolator::CreateClone` (Xbox PDB, matched by the engine map;
/// `thiscall`, one word: the cloning process): a new interpolator set up as
/// [`fn_00a53500`] does, with this one's members copied in
/// ([`fn_00a53480`]; also called, on null, when the allocation fails).
pub fn ni_bool_interpolator_create_clone(
    e: &mut Engine,
    this: Ptr<NiBoolInterpolator>,
    cloning: u32,
) -> Ptr<NiBoolInterpolator> {
    let block = e.call(NI_NEW, &args![0x18u32]).u32();
    let clone = Ptr::<NiBoolInterpolator>::new(block);
    if block != 0 {
        e.call(KEY_BASED_INTERPOLATOR_CONSTRUCTOR, &args![clone]);
        e.mem.set_u32(block, VTABLE);
        let invalid: u8 = e.global(INVALID_BOOL);
        e.set(clone, NiBoolInterpolator::m_bBoolValue, invalid);
        e.set(clone, NiBoolInterpolator::m_spBoolData, 0);
        e.set(clone, NiBoolInterpolator::m_uiLastIdx, 0);
    }
    fn_00a53480(e, this, clone, cloning);
    clone
}

// Translated from 00a53600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolInterpolator::LoadBinary` (Xbox PDB, matched by the engine map;
/// `thiscall`, one word: the stream): the base `LoadBinary`, one byte into
/// the value through the stream's file (+0x24C: its read function at +8,
/// `cdecl (file, buffer, 1, &count, 1)` with the count word set to 1; the
/// result added to the file's +4), then the data from the stream's next
/// link id (`NiStream::ResolveLinkID`, a `NiPointer` assignment).
pub fn ni_bool_interpolator_load_binary(
    e: &mut Engine,
    this: Ptr<NiBoolInterpolator>,
    stream: Ptr,
) {
    e.call(BASE_LOAD_BINARY, &args![this, stream]);
    let file = e.mem.u32(stream.addr() + 0x24c);
    let read = e.mem.u32(file + 8);
    let got = e.with_stack(4, |e, count| {
        e.mem.set_u32(count.addr(), 1);
        e.call(read, &args![file, this.addr() + 0xc, 1u32, count, 1u32])
            .u32()
    });
    let position = e.mem.u32(file + 4);
    e.mem.set_u32(file + 4, position.wrapping_add(got));
    let data = e.call(STREAM_RESOLVE_LINK_ID, &args![stream]).u32();
    set_bool_data(e, this, data);
}

/// `NiKeyBasedInterpolator::RegisterStreamables` (`00aa0ad0`, engine map;
/// `thiscall`, one word: the stream; answers a `bool`).
const BASE_REGISTER_STREAMABLES: u32 = 0x00aa_0ad0;
/// `NiObject::IsEqual` (`00c85800`, engine map; `thiscall`, one word).
const OBJECT_IS_EQUAL: u32 = 0x00c8_5800;
/// `NiInterpolator::GetSequenceInterpolator` (`00a57050`, engine map;
/// `thiscall (this; start, end)`, answers the new interpolator).
const BASE_GET_SEQUENCE_INTERPOLATOR: u32 = 0x00a5_7050;
/// `NiBoolData::GuaranteeKeysAtStartAndEnd` (`00a537a0`, engine map;
/// `thiscall (data; start, end)`).
const BOOL_DATA_GUARANTEE_KEYS: u32 = 0x00a5_37a0;
/// `NiBoolData::GetSequenceData` (`00a53910`, engine map; `thiscall (data;
/// start, end, &new data)`).
const BOOL_DATA_GET_SEQUENCE_DATA: u32 = 0x00a5_3910;
/// The base `SaveBinary` (`00a95ca0`, the engine map's
/// `BSGrabIKNote::LinkObject` on that folded body; `thiscall`, one word).
const BASE_SAVE_BINARY: u32 = 0x00a9_5ca0;
/// `NiKeyBasedInterpolator::GetViewerStrings` (`00a3cbf0`, engine map;
/// `thiscall`, one word: the strings array).
const BASE_GET_VIEWER_STRINGS: u32 = 0x00a3_cbf0;
/// `NiGetViewerString` (`00a7d330`, engine map; `cdecl (text)`), its `bool`
/// overload (`00a7d410`, `cdecl (name, value)`) and its pointer overload
/// (`00a7d5a0`, `cdecl (name, pointer)`).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const NI_GET_VIEWER_STRING_BOOL: u32 = 0x00a7_d410;
const NI_GET_VIEWER_STRING_POINTER: u32 = 0x00a7_d5a0;
/// The viewer-string array's grow (`0096ad30`, `thiscall (array; new
/// capacity)`) and store (`0096ae90`, `thiscall (array; index, &string)`).
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
/// `"m_bBoolValue"` and `"m_spBoolData"`.
const NAME_BOOL_VALUE: u32 = 0x0109_a1bc;
const NAME_BOOL_DATA: u32 = 0x0109_a1ac;

/// Appends `text` to a viewer-string array as the exe's inlined `Add`
/// does: when the 16-bit size (+0xA) has reached the capacity (+0x8) the
/// array grows to the step (+0xE) plus the size, then the string is stored
/// at the old size (through a stack slot holding it).
fn push_viewer_string(e: &mut Engine, strings: Ptr, text: u32) {
    let size = u32::from(e.mem.u16(strings.addr() + 0xa));
    if size >= u32::from(e.mem.u16(strings.addr() + 8)) {
        let step = u32::from(e.mem.u16(strings.addr() + 0xe));
        e.call(STRINGS_GROW, &args![strings, step + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRINGS_SET_AT, &args![strings, size, slot]);
    });
}

// Translated from 00a3c3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolInterpolator::GetKeyType` (Xbox PDB, matched by the engine map;
/// `thiscall`, one word: the channel, not read): the data's key type
/// (`NiBoolData` +0x10), 0 without data.
pub fn ni_bool_interpolator_get_key_type(
    e: &mut Engine,
    this: Ptr<NiBoolInterpolator>,
    _unused_channel: u32,
) -> u32 {
    let data = e.get(this, NiBoolInterpolator::m_spBoolData);
    if data == 0 {
        return 0;
    }
    e.mem.u32(data + 0x10)
}

// Translated from 00a3c3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolInterpolator::GetKeyStride` (Xbox PDB, matched by the engine
/// map; `thiscall`, one word: the channel, not read): the data's key size
/// byte (`NiBoolData` +0x14), 0 without data.
pub fn ni_bool_interpolator_get_key_stride(
    e: &mut Engine,
    this: Ptr<NiBoolInterpolator>,
    _unused_channel: u32,
) -> u8 {
    let data = e.get(this, NiBoolInterpolator::m_spBoolData);
    if data == 0 {
        return 0;
    }
    e.mem.u8(data + 0x14)
}

// Translated from 00a3c5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolInterpolator::RegisterStreamables` (Xbox PDB, matched by the
/// engine map; `thiscall`, one word: the stream): false when the base
/// registration (`00aa0ad0`) answers false; otherwise the data registers
/// itself (its vtable +0x54) and the answer is true.
pub fn ni_bool_interpolator_register_streamables(
    e: &mut Engine,
    this: Ptr<NiBoolInterpolator>,
    stream: Ptr,
) -> bool {
    if !e
        .call(BASE_REGISTER_STREAMABLES, &args![this, stream])
        .bool()
    {
        return false;
    }
    let data = e.get(this, NiBoolInterpolator::m_spBoolData);
    if data != 0 {
        e.vcall(data, 0x54, &args![stream]);
    }
    true
}

// Translated from 00a53070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolInterpolator::GetKeyContent` (Xbox PDB, matched by the engine
/// map; `thiscall`, one word: the channel, not read): always 5 (the key
/// content the exe returns for bool keys).
pub fn ni_bool_interpolator_get_key_content(
    _e: &mut Engine,
    _this: Ptr<NiBoolInterpolator>,
    _unused_channel: u32,
) -> u32 {
    5
}

// Translated from 00a53080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolInterpolator::GetKeyCount` (Xbox PDB, matched by the engine map;
/// `thiscall`, one word: the channel, not read): the data's key count
/// (`NiBoolData` +0x8), 0 without data.
pub fn ni_bool_interpolator_get_key_count(
    e: &mut Engine,
    this: Ptr<NiBoolInterpolator>,
    _unused_channel: u32,
) -> u32 {
    let data = e.get(this, NiBoolInterpolator::m_spBoolData);
    if data == 0 {
        return 0;
    }
    e.mem.u32(data + 8)
}

// Translated from 00a53090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolInterpolator::GetKeyArray` (Xbox PDB, matched by the engine map;
/// `thiscall`, one word: the channel, not read): the data's keys
/// (`NiBoolData` +0xC), null without data.
pub fn ni_bool_interpolator_get_key_array(
    e: &mut Engine,
    this: Ptr<NiBoolInterpolator>,
    _unused_channel: u32,
) -> u32 {
    let data = e.get(this, NiBoolInterpolator::m_spBoolData);
    if data == 0 {
        return 0;
    }
    e.mem.u32(data + 0xc)
}

// Translated from 00a530c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolInterpolator::Collapse` (Xbox PDB, matched by the engine map;
/// `thiscall`, no words). With data: no keys drops the data and sets the
/// invalid value (`0109a914`); one key, or several of key type 1 or 5 that
/// all hold the first key's value (the byte at +4 of each key, keys the
/// data's key size apart), drops the data and keeps that value. Several
/// keys of another type, or differing values, leave everything as it is.
pub fn ni_bool_interpolator_collapse(e: &mut Engine, this: Ptr<NiBoolInterpolator>) {
    let data = e.get(this, NiBoolInterpolator::m_spBoolData);
    if data == 0 {
        return;
    }
    let key_type = e.mem.u32(data + 0x10);
    let key_size = e.mem.u8(data + 0x14);
    let count = e.mem.u32(data + 8);
    let keys = e.mem.u32(data + 0xc);
    if count == 0 {
        release(e, data);
        e.set(this, NiBoolInterpolator::m_spBoolData, 0);
        let invalid: u8 = e.global(INVALID_BOOL);
        e.set(this, NiBoolInterpolator::m_bBoolValue, invalid);
        return;
    }
    let first = e.mem.u8(keys + 4);
    if count != 1 {
        if key_type != 1 && key_type != 5 {
            return;
        }
        let mut index = 1u32;
        while index < count {
            let offset = u32::from(key_size).wrapping_mul(index);
            let same = e.mem.u8(keys.wrapping_add(offset).wrapping_add(4)) == first;
            index += 1;
            if !same {
                return;
            }
        }
    }
    let data = e.get(this, NiBoolInterpolator::m_spBoolData);
    if data != 0 {
        release(e, data);
        e.set(this, NiBoolInterpolator::m_spBoolData, 0);
    }
    e.set(this, NiBoolInterpolator::m_bBoolValue, first);
}

// Translated from 00a531a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolInterpolator::SetPoseValue` (Xbox PDB, matched by the engine
/// map; `thiscall`, one word: the value): releases and clears the data
/// (when there is any), then stores the value byte.
pub fn ni_bool_interpolator_set_pose_value(
    e: &mut Engine,
    this: Ptr<NiBoolInterpolator>,
    value: u8,
) {
    let data = e.get(this, NiBoolInterpolator::m_spBoolData);
    if data != 0 {
        release(e, data);
        e.set(this, NiBoolInterpolator::m_spBoolData, 0);
    }
    e.set(this, NiBoolInterpolator::m_bBoolValue, value);
}

// Translated from 00a531f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolInterpolator::GuaranteeTimeRange` (Xbox PDB, matched by the
/// engine map; `thiscall (this; start, end)`): with data, its
/// `GuaranteeKeysAtStartAndEnd` (`00a537a0`) on both times (copied
/// through the FPU).
pub fn ni_bool_interpolator_guarantee_time_range(
    e: &mut Engine,
    this: Ptr<NiBoolInterpolator>,
    start: f32,
    end: f32,
) {
    let data = e.get(this, NiBoolInterpolator::m_spBoolData);
    if data == 0 {
        return;
    }
    let end = x87_quiet_bits(end.to_bits());
    let start = x87_quiet_bits(start.to_bits());
    e.call(BOOL_DATA_GUARANTEE_KEYS, &args![data, start, end]);
}

// Translated from 00a53220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolInterpolator::IsEqual` (Xbox PDB, matched by the engine map;
/// `thiscall`, one word: the other object): `NiObject::IsEqual`, the same
/// value byte, and either both without data or both with data that the
/// data's own `IsEqual` (vtable +0x5C) finds equal.
pub fn ni_bool_interpolator_is_equal(
    e: &mut Engine,
    this: Ptr<NiBoolInterpolator>,
    other: Ptr<NiBoolInterpolator>,
) -> bool {
    if !e.call(OBJECT_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    if e.get(this, NiBoolInterpolator::m_bBoolValue)
        != e.get(other, NiBoolInterpolator::m_bBoolValue)
    {
        return false;
    }
    let data = e.get(this, NiBoolInterpolator::m_spBoolData);
    let other_data = e.get(other, NiBoolInterpolator::m_spBoolData);
    if data != 0 {
        if other_data == 0 {
            return false;
        }
    } else {
        return other_data == 0;
    }
    e.vcall(data, 0x5c, &args![other_data]).bool()
}

// Translated from 00a53300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolInterpolator::GetSequenceInterpolator` (Xbox PDB, matched by the
/// engine map; `thiscall (this; start, end)`): the base's new interpolator
/// (`00a57050`); with data, the data's sequence data for the range
/// (`NiBoolData::GetSequenceData`, `00a53910`, into a stack `NiPointer`
/// that starts null) becomes the new interpolator's data (a `NiPointer`
/// assignment), its last index 0, and the stack pointer is released.
/// Answers the new interpolator.
pub fn ni_bool_interpolator_get_sequence_interpolator(
    e: &mut Engine,
    this: Ptr<NiBoolInterpolator>,
    start: f32,
    end: f32,
) -> Ptr<NiBoolInterpolator> {
    let start_bits = x87_quiet_bits(start.to_bits());
    let end_bits = x87_quiet_bits(end.to_bits());
    let sequence = e
        .call(
            BASE_GET_SEQUENCE_INTERPOLATOR,
            &args![this, start_bits, end_bits],
        )
        .ptr::<NiBoolInterpolator>();
    let data = e.get(this, NiBoolInterpolator::m_spBoolData);
    if data == 0 {
        return sequence;
    }
    let new_data = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), 0);
        e.call(
            BOOL_DATA_GET_SEQUENCE_DATA,
            &args![data, start_bits, end_bits, slot],
        );
        e.mem.u32(slot.addr())
    });
    set_bool_data(e, sequence, new_data);
    e.set(sequence, NiBoolInterpolator::m_uiLastIdx, 0);
    if new_data != 0 {
        release(e, new_data);
    }
    sequence
}

// Translated from 00a53680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolInterpolator::SaveBinary` (Xbox PDB, matched by the engine map;
/// `thiscall`, one word: the stream): the base `SaveBinary` (`00a95ca0`),
/// the value byte through the stream's output file (+0x250: its write
/// function at +0xC, `cdecl (file, source, 1, &count, 1)` with the count
/// word set to 1; the result added to the file's +4), then the data's link
/// id (the stream's vtable +0x30).
pub fn ni_bool_interpolator_save_binary(
    e: &mut Engine,
    this: Ptr<NiBoolInterpolator>,
    stream: Ptr,
) {
    e.call(BASE_SAVE_BINARY, &args![this, stream]);
    let file = e.mem.u32(stream.addr() + 0x250);
    let write = e.mem.u32(file + 0xc);
    let written = e.with_stack(4, |e, count| {
        e.mem.set_u32(count.addr(), 1);
        e.call(write, &args![file, this.addr() + 0xc, 1u32, count, 1u32])
            .u32()
    });
    let position = e.mem.u32(file + 4);
    e.mem.set_u32(file + 4, position.wrapping_add(written));
    let data = e.get(this, NiBoolInterpolator::m_spBoolData);
    e.vcall(stream.addr(), 0x30, &args![data]);
}

// Translated from 00a536d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolInterpolator::GetViewerStrings` (Xbox PDB, matched by the engine
/// map; `thiscall`, one word: the strings array): the base's strings
/// (`00a3cbf0`), then the class name (the first word of the `NiRTTI`
/// record), `m_bBoolValue` and `m_spBoolData`, each appended.
pub fn ni_bool_interpolator_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<NiBoolInterpolator>,
    strings: Ptr,
) {
    e.call(BASE_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name: u32 = e.global(NI_BOOL_INTERPOLATOR_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    push_viewer_string(e, strings, text);
    let value = u32::from(e.get(this, NiBoolInterpolator::m_bBoolValue));
    let text = e
        .call(NI_GET_VIEWER_STRING_BOOL, &args![NAME_BOOL_VALUE, value])
        .u32();
    push_viewer_string(e, strings, text);
    let data = e.get(this, NiBoolInterpolator::m_spBoolData);
    let text = e
        .call(NI_GET_VIEWER_STRING_POINTER, &args![NAME_BOOL_DATA, data])
        .u32();
    push_viewer_string(e, strings, text);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a3_c3c0,
            ni_bool_interpolator_get_key_type(Ptr<NiBoolInterpolator>, u32) -> u32
        ),
        entry!(
            0x00a3_c3d0,
            ni_bool_interpolator_get_key_stride(Ptr<NiBoolInterpolator>, u32) -> u8
        ),
        entry!(
            0x00a3_c5b0,
            ni_bool_interpolator_register_streamables(Ptr<NiBoolInterpolator>, Ptr) -> bool
        ),
        entry!(
            0x00a5_3070,
            ni_bool_interpolator_get_key_content(Ptr<NiBoolInterpolator>, u32) -> u32
        ),
        entry!(
            0x00a5_3080,
            ni_bool_interpolator_get_key_count(Ptr<NiBoolInterpolator>, u32) -> u32
        ),
        entry!(
            0x00a5_3090,
            ni_bool_interpolator_get_key_array(Ptr<NiBoolInterpolator>, u32) -> u32
        ),
        entry!(
            0x00a5_30c0,
            ni_bool_interpolator_collapse(Ptr<NiBoolInterpolator>)
        ),
        entry!(
            0x00a5_31a0,
            ni_bool_interpolator_set_pose_value(Ptr<NiBoolInterpolator>, u8)
        ),
        entry!(
            0x00a5_31f0,
            ni_bool_interpolator_guarantee_time_range(Ptr<NiBoolInterpolator>, f32, f32)
        ),
        entry!(
            0x00a5_3220,
            ni_bool_interpolator_is_equal(Ptr<NiBoolInterpolator>, Ptr<NiBoolInterpolator>) -> bool
        ),
        entry!(
            0x00a5_3300,
            ni_bool_interpolator_get_sequence_interpolator(
                Ptr<NiBoolInterpolator>,
                f32,
                f32,
            ) -> Ptr<NiBoolInterpolator>
        ),
        entry!(
            0x00a5_3680,
            ni_bool_interpolator_save_binary(Ptr<NiBoolInterpolator>, Ptr)
        ),
        entry!(
            0x00a5_36d0,
            ni_bool_interpolator_get_viewer_strings(Ptr<NiBoolInterpolator>, Ptr)
        ),
        entry!(0x00a532c0, ni_bool_interpolator_get_rtti() -> Ptr),
        entry!(0x00a5_3500, fn_00a53500() -> Ptr<NiBoolInterpolator>),
        entry!(
            0x00a5_30a0,
            ni_bool_interpolator_get_channel_posed(Ptr<NiBoolInterpolator>, u32) -> bool
        ),
        entry!(
            0x00a5_3280,
            fn_00a53280(Ptr<NiBoolInterpolator>, u32) -> Ptr<NiBoolInterpolator>
        ),
        entry!(
            0x00a5_32d0,
            ni_bool_interpolator_constructor(
                Ptr<NiBoolInterpolator>,
                u8,
            ) -> Ptr<NiBoolInterpolator>
        ),
        entry!(
            0x00a5_33c0,
            ni_bool_interpolator_update(Ptr<NiBoolInterpolator>, f32, u32, Ptr) -> bool
        ),
        entry!(
            0x00a5_3480,
            fn_00a53480(Ptr<NiBoolInterpolator>, Ptr<NiBoolInterpolator>, u32)
        ),
        entry!(
            0x00a5_3540,
            ni_bool_interpolator_scalar_deleting_destructor(
                Ptr<NiBoolInterpolator>,
                u32,
            ) -> Ptr<NiBoolInterpolator>
        ),
        entry!(
            0x00a5_3590,
            ni_bool_interpolator_create_clone(
                Ptr<NiBoolInterpolator>,
                u32,
            ) -> Ptr<NiBoolInterpolator>
        ),
        entry!(
            0x00a5_3600,
            ni_bool_interpolator_load_binary(Ptr<NiBoolInterpolator>, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_bool_interpolator_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a532c0, &args![]).u32(),
            NI_BOOL_INTERPOLATOR_RTTI
        );
    }
}

#[cfg(test)]
mod interpolator_tests {
    use super::*;

    type B = NiBoolInterpolator;

    const DELETE_SLOT: u32 = 0x0001_0010;
    const READ: u32 = 0x0001_0020;
    const INVALID: u8 = 2;

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0109_a000, 0x1000);
        e.set_global(INVALID_BOOL, INVALID);
        e.register(KEY_BASED_INTERPOLATOR_CONSTRUCTOR, |_, _| Ret::default());
        e.register(KEY_BASED_INTERPOLATOR_DESTRUCTOR, |_, _| Ret::default());
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let value = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], value);
            word(value)
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let value = e.mem.u32(a[0]) - 1;
            e.mem.set_u32(a[0], value);
            word(value)
        });
        e.register(DELETE_SLOT, |_, _| Ret::default());
        e
    }

    /// A reference-counted object (count at +4) whose vtable +4 is
    /// `DELETE_SLOT`.
    fn counted(e: &mut Engine, references: u32) -> u32 {
        let object = e.mem.alloc(0x18);
        e.put_vtable(0x0002_0000, &[0, DELETE_SLOT]);
        e.mem.set_u32(object, 0x0002_0000);
        e.mem.set_u32(object + 4, references);
        object
    }

    fn deleted(log: &[(u32, Vec<u32>)]) -> Vec<u32> {
        log.iter()
            .filter(|c| c.0 == DELETE_SLOT)
            .map(|c| c.1[0])
            .collect()
    }

    #[test]
    fn create_object_sets_up_an_empty_interpolator() {
        let mut e = engine();
        let block = e.mem.alloc(0x18);
        e.mem.write(block, &[0xcd; 0x18]);
        e.register_double(NI_NEW, move |_, a| {
            assert_eq!(a[0], 0x18);
            word(block)
        });
        assert_eq!(e.call(0x00a5_3500, &args![]).u32(), block);
        let this = Ptr::<B>::new(block);
        assert_eq!(e.mem.u32(block), VTABLE);
        assert_eq!(e.get(this, B::m_bBoolValue), INVALID);
        assert_eq!(e.get(this, B::m_spBoolData), 0);
        assert_eq!(e.get(this, B::m_uiLastIdx), 0);
        e.register(NI_NEW, |_, _| word(0));
        assert_eq!(e.call(0x00a5_3500, &args![]).u32(), 0);
    }

    #[test]
    fn channel_is_posed_with_a_value_and_no_data() {
        let mut e = engine();
        let this = e.new_object::<B>();
        e.set(this, B::m_bBoolValue, 1);
        assert!(e.call(0x00a5_30a0, &args![this, 0u32]).bool());
        e.set(this, B::m_bBoolValue, INVALID);
        assert!(!e.call(0x00a5_30a0, &args![this, 0u32]).bool());
        e.set(this, B::m_bBoolValue, 0);
        e.set(this, B::m_spBoolData, 0x500);
        assert!(!e.call(0x00a5_30a0, &args![this, 0u32]).bool());
    }

    #[test]
    fn constructors_set_the_members() {
        let mut e = engine();
        let data = counted(&mut e, 1);
        let this = e.new_object::<B>();
        assert_eq!(e.call(0x00a5_3280, &args![this, data]).u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), VTABLE);
        assert_eq!(e.get(this, B::m_bBoolValue), INVALID);
        assert_eq!(e.get(this, B::m_spBoolData), data);
        assert_eq!(e.mem.u32(data + 4), 2);
        assert_eq!(e.get(this, B::m_uiLastIdx), 0);

        let other = e.new_object::<B>();
        e.mem.write(other.addr(), &[0xcd; 0x18]);
        assert_eq!(
            e.call(0x00a5_32d0, &args![other, 0x101u32]).u32(),
            other.addr()
        );
        assert_eq!(e.get(other, B::m_bBoolValue), 1);
        assert_eq!(e.mem.u32(other.addr()), VTABLE);
        assert_eq!(e.get(other, B::m_spBoolData), 0);
        assert_eq!(e.get(other, B::m_uiLastIdx), 0);
    }

    fn bool_data(e: &mut Engine, count: u32) -> u32 {
        let data = counted(e, 1);
        e.mem.set_u32(data + 8, count);
        e.mem.set_u32(data + 0xc, 0x7000);
        e.mem.set_u32(data + 0x10, 3);
        e.mem.set_u8(data + 0x14, 2);
        data
    }

    #[test]
    fn update_evaluates_the_keys_at_a_new_time() {
        let mut e = engine();
        let this = e.new_object::<B>();
        let data = bool_data(&mut e, 4);
        e.set(this, B::m_spBoolData, data);
        e.set(this, B::m_fLastTime, 1.0f32.to_bits());
        e.register(BOOL_KEY_GEN_INTERP, |_, _| word(0x0000_0100));
        let value = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x00a5_33c0, &args![this, 2.0f32, 0u32, value])
            .bool());
        let log = e.call_log.take().unwrap();
        let gen: Vec<_> = log.iter().filter(|c| c.0 == BOOL_KEY_GEN_INTERP).collect();
        assert_eq!(
            gen[0].1,
            vec![
                2.0f32.to_bits(),
                0x7000,
                3,
                4,
                this.addr() + 0x14,
                (this.addr() & 0xffff_ff00) | 2
            ]
        );
        // The key answered false (AL = 0): the value is false.
        assert_eq!(e.get(this, B::m_bBoolValue), 0);
        assert_eq!(e.mem.u8(value), 0);
        assert_eq!(e.get(this, B::m_fLastTime), 2.0f32.to_bits());

        e.register(BOOL_KEY_GEN_INTERP, |_, _| word(1));
        assert!(e
            .call(0x00a5_33c0, &args![this, 3.0f32, 0u32, value])
            .bool());
        assert_eq!(e.mem.u8(value), 1);
        assert_eq!(e.get(this, B::m_fLastTime), 3.0f32.to_bits());
    }

    #[test]
    fn update_reuses_the_value_at_the_same_time() {
        let mut e = engine();
        let this = e.new_object::<B>();
        e.set(this, B::m_fLastTime, 1.0f32.to_bits());
        e.set(this, B::m_bBoolValue, 5);
        let value = e.mem.alloc(4);
        assert!(e
            .call(0x00a5_33c0, &args![this, 1.0f32, 0u32, value])
            .bool());
        assert_eq!(e.mem.u8(value), 1);
        e.set(this, B::m_bBoolValue, INVALID);
        e.mem.set_u8(value, 9);
        assert!(!e
            .call(0x00a5_33c0, &args![this, 1.0f32, 0u32, value])
            .bool());
        assert_eq!(e.mem.u8(value), 0);
        // A new time without data keeps the value but records the time;
        // an invalid value records nothing.
        e.set(this, B::m_bBoolValue, 0);
        assert!(e
            .call(0x00a5_33c0, &args![this, 2.0f32, 0u32, value])
            .bool());
        assert_eq!(e.get(this, B::m_fLastTime), 2.0f32.to_bits());
        e.set(this, B::m_bBoolValue, INVALID);
        assert!(!e
            .call(0x00a5_33c0, &args![this, 4.0f32, 0u32, value])
            .bool());
        assert_eq!(e.get(this, B::m_fLastTime), 2.0f32.to_bits());
        // Data without keys is not evaluated.
        let data = bool_data(&mut e, 0);
        e.set(this, B::m_spBoolData, data);
        e.call_log = Some(vec![]);
        e.call(0x00a5_33c0, &args![this, 5.0f32, 0u32, value]);
        assert!(e
            .call_log
            .take()
            .unwrap()
            .iter()
            .all(|c| c.0 != BOOL_KEY_GEN_INTERP));
    }

    #[test]
    fn copy_members_shares_the_data() {
        let mut e = engine();
        e.register(BASE_COPY_MEMBERS, |_, _| Ret::default());
        let source = e.new_object::<B>();
        let dest = e.new_object::<B>();
        let data = counted(&mut e, 1);
        let old = counted(&mut e, 1);
        e.set(source, B::m_bBoolValue, 1);
        e.set(source, B::m_spBoolData, data);
        e.set(source, B::m_uiLastIdx, 7);
        e.set(dest, B::m_spBoolData, old);
        e.call_log = Some(vec![]);
        e.call(0x00a5_3480, &args![source, dest, 0x900u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1],
            (BASE_COPY_MEMBERS, vec![source.addr(), dest.addr(), 0x900])
        );
        assert_eq!(e.get(dest, B::m_bBoolValue), 1);
        assert_eq!(e.get(dest, B::m_spBoolData), data);
        assert_eq!(e.get(dest, B::m_uiLastIdx), 7);
        assert_eq!(e.mem.u32(data + 4), 2);
        assert_eq!(deleted(&log), vec![old]);
        // The same data: no reference change.
        e.call(0x00a5_3480, &args![source, dest, 0x900u32]);
        assert_eq!(e.mem.u32(data + 4), 2);
    }

    #[test]
    fn scalar_deleting_destructor_releases_the_data() {
        let mut e = engine();
        e.register(NI_DELETE, |_, _| Ret::default());
        let this = e.new_object::<B>();
        let data = counted(&mut e, 1);
        e.set(this, B::m_spBoolData, data);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a5_3540, &args![this, 1u32]).u32(), this.addr());
        let log = e.call_log.take().unwrap();
        assert_eq!(deleted(&log), vec![data]);
        assert_eq!(log.last().unwrap(), &(NI_DELETE, vec![this.addr(), 0x18]));
        e.set(this, B::m_spBoolData, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a5_3540, &args![this, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [(KEY_BASED_INTERPOLATOR_DESTRUCTOR, vec![this.addr()])]
        );
    }

    #[test]
    fn create_clone_copies_into_a_new_interpolator() {
        let mut e = engine();
        e.register(BASE_COPY_MEMBERS, |_, _| Ret::default());
        let block = e.mem.alloc(0x18);
        e.register_double(NI_NEW, move |_, _| word(block));
        let this = e.new_object::<B>();
        e.set(this, B::m_bBoolValue, 1);
        e.set(this, B::m_uiLastIdx, 3);
        assert_eq!(e.call(0x00a5_3590, &args![this, 0x900u32]).u32(), block);
        let clone = Ptr::<B>::new(block);
        assert_eq!(e.mem.u32(block), VTABLE);
        assert_eq!(e.get(clone, B::m_bBoolValue), 1);
        assert_eq!(e.get(clone, B::m_uiLastIdx), 3);
    }

    #[test]
    fn load_binary_reads_the_value_and_links_the_data() {
        let mut e = engine();
        e.register(BASE_LOAD_BINARY, |_, _| Ret::default());
        let this = e.new_object::<B>();
        let file = e.mem.alloc(0x10);
        e.mem.set_u32(file + 4, 0x40);
        e.mem.set_u32(file + 8, READ);
        e.register(READ, |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (1, 1, 1));
            e.mem.set_u8(a[1], 1);
            word(1)
        });
        let stream = e.mem.alloc(0x250);
        e.mem.set_u32(stream + 0x24c, file);
        let data = counted(&mut e, 1);
        e.register_double(STREAM_RESOLVE_LINK_ID, move |_, _| word(data));
        e.call(0x00a5_3600, &args![this, stream]);
        assert_eq!(e.get(this, B::m_bBoolValue), 1);
        assert_eq!(e.mem.u32(file + 4), 0x41);
        assert_eq!(e.get(this, B::m_spBoolData), data);
        assert_eq!(e.mem.u32(data + 4), 2);
    }
}

#[cfg(test)]
mod n002_tests {
    use super::*;

    type B = NiBoolInterpolator;

    const DELETE_SLOT: u32 = 0x0001_0010;
    const DATA_SLOT: u32 = 0x0001_0030;
    const INVALID: u8 = 2;
    const DATA_VTABLE: u32 = 0x0002_0000;

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0109_a000, 0x1000);
        e.set_global(INVALID_BOOL, INVALID);
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let value = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], value);
            word(value)
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let value = e.mem.u32(a[0]) - 1;
            e.mem.set_u32(a[0], value);
            word(value)
        });
        e.register(DELETE_SLOT, |_, _| Ret::default());
        e.register(DATA_SLOT, |_, a| word(u32::from(a[1] == 0x4444)));
        let mut slots = vec![0u32; 0x18];
        slots[1] = DELETE_SLOT;
        slots[0x54 / 4] = DATA_SLOT;
        slots[0x5c / 4] = DATA_SLOT;
        e.put_vtable(DATA_VTABLE, &slots);
        e.call_log = Some(vec![]);
        e
    }

    /// A bool data object (reference count at +4) with keys of size 2
    /// holding `values`, of key type `key_type`.
    fn data(e: &mut Engine, references: u32, key_type: u32, values: &[u8]) -> u32 {
        let object = e.mem.alloc(0x18);
        e.mem.set_u32(object, DATA_VTABLE);
        e.mem.set_u32(object + 4, references);
        e.mem.set_u32(object + 8, values.len() as u32);
        let keys = e.mem.alloc(8 + 2 * values.len() as u32);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_u8(keys + 4 + 2 * i as u32, *v);
        }
        e.mem.set_u32(object + 0xc, keys);
        e.mem.set_u32(object + 0x10, key_type);
        e.mem.set_u8(object + 0x14, 2);
        object
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn key_accessors_read_the_data_or_answer_zero() {
        let mut e = engine();
        let this = e.new_object::<B>();
        for addr in [0x00a3_c3c0u32, 0x00a3_c3d0, 0x00a5_3080, 0x00a5_3090] {
            assert_eq!(e.call(addr, &args![this, 0u32]).u32(), 0);
        }
        let d = data(&mut e, 1, 3, &[1, 0, 1]);
        e.set(this, B::m_spBoolData, d);
        assert_eq!(e.call(0x00a3_c3c0, &args![this, 0u32]).u32(), 3);
        assert_eq!(e.call(0x00a3_c3d0, &args![this, 0u32]).u8(), 2);
        assert_eq!(e.call(0x00a5_3080, &args![this, 0u32]).u32(), 3);
        assert_eq!(
            e.call(0x00a5_3090, &args![this, 0u32]).u32(),
            e.mem.u32(d + 0xc)
        );
        assert_eq!(e.call(0x00a5_3070, &args![this, 0u32]).u32(), 5);
    }

    #[test]
    fn register_streamables_registers_the_data_after_the_base() {
        let mut e = engine();
        let this = e.new_object::<B>();
        let d = data(&mut e, 1, 1, &[1]);
        e.set(this, B::m_spBoolData, d);
        e.register(BASE_REGISTER_STREAMABLES, |_, _| word(0));
        assert!(!e.call(0x00a3_c5b0, &args![this, 0x700u32]).bool());
        assert!(calls(&e, DATA_SLOT).is_empty());
        e.register(BASE_REGISTER_STREAMABLES, |_, _| word(1));
        assert!(e.call(0x00a3_c5b0, &args![this, 0x700u32]).bool());
        assert_eq!(calls(&e, DATA_SLOT), vec![vec![d, 0x700]]);
        e.set(this, B::m_spBoolData, 0);
        assert!(e.call(0x00a3_c5b0, &args![this, 0x700u32]).bool());
        assert_eq!(calls(&e, DATA_SLOT).len(), 1);
    }

    #[test]
    fn collapse_drops_constant_data() {
        let mut e = engine();
        let this = e.new_object::<B>();
        // No keys: the invalid value.
        let d = data(&mut e, 1, 1, &[]);
        e.set(this, B::m_spBoolData, d);
        e.call(0x00a5_30c0, &args![this]);
        assert_eq!(e.get(this, B::m_spBoolData), 0);
        assert_eq!(e.get(this, B::m_bBoolValue), INVALID);
        assert_eq!(calls(&e, DELETE_SLOT), vec![vec![d]]);
        // One key: its value.
        let d = data(&mut e, 2, 9, &[1]);
        e.set(this, B::m_spBoolData, d);
        e.call(0x00a5_30c0, &args![this]);
        assert_eq!(e.get(this, B::m_spBoolData), 0);
        assert_eq!(e.get(this, B::m_bBoolValue), 1);
        assert_eq!(e.mem.u32(d + 4), 1);
        // Equal keys of type 5: collapsed.
        let d = data(&mut e, 2, 5, &[0, 0, 0]);
        e.set(this, B::m_spBoolData, d);
        e.call(0x00a5_30c0, &args![this]);
        assert_eq!(e.get(this, B::m_spBoolData), 0);
        assert_eq!(e.get(this, B::m_bBoolValue), 0);
        // Differing keys, or another type: unchanged.
        for (key_type, values) in [(1u32, [1u8, 1, 0]), (2, [1, 1, 1])] {
            let d = data(&mut e, 2, key_type, &values);
            e.set(this, B::m_spBoolData, d);
            e.set(this, B::m_bBoolValue, 7);
            e.call(0x00a5_30c0, &args![this]);
            assert_eq!(e.get(this, B::m_spBoolData), d);
            assert_eq!(e.get(this, B::m_bBoolValue), 7);
            assert_eq!(e.mem.u32(d + 4), 2);
        }
        // No data: nothing.
        e.set(this, B::m_spBoolData, 0);
        e.call(0x00a5_30c0, &args![this]);
        assert_eq!(e.get(this, B::m_bBoolValue), 7);
    }

    #[test]
    fn set_pose_value_drops_the_data() {
        let mut e = engine();
        let this = e.new_object::<B>();
        let d = data(&mut e, 1, 1, &[1]);
        e.set(this, B::m_spBoolData, d);
        e.call(0x00a5_31a0, &args![this, 0x301u32]);
        assert_eq!(e.get(this, B::m_spBoolData), 0);
        assert_eq!(e.get(this, B::m_bBoolValue), 1);
        assert_eq!(calls(&e, DELETE_SLOT), vec![vec![d]]);
        e.call(0x00a5_31a0, &args![this, 0u32]);
        assert_eq!(e.get(this, B::m_bBoolValue), 0);
    }

    #[test]
    fn guarantee_time_range_passes_both_times_to_the_data() {
        let mut e = engine();
        e.register(BOOL_DATA_GUARANTEE_KEYS, |_, _| Ret::default());
        let this = e.new_object::<B>();
        e.call(0x00a5_31f0, &args![this, 1.0f32, 2.0f32]);
        assert!(calls(&e, BOOL_DATA_GUARANTEE_KEYS).is_empty());
        e.set(this, B::m_spBoolData, 0x5000);
        e.call(0x00a5_31f0, &args![this, 1.0f32, 2.0f32]);
        assert_eq!(
            calls(&e, BOOL_DATA_GUARANTEE_KEYS),
            vec![vec![0x5000, 1.0f32.to_bits(), 2.0f32.to_bits()]]
        );
    }

    #[test]
    fn is_equal_compares_value_and_data() {
        let mut e = engine();
        e.register(OBJECT_IS_EQUAL, |_, _| word(1));
        let this = e.new_object::<B>();
        let other = e.new_object::<B>();
        assert!(e.call(0x00a5_3220, &args![this, other]).bool());
        e.set(other, B::m_bBoolValue, 1);
        assert!(!e.call(0x00a5_3220, &args![this, other]).bool());
        e.set(other, B::m_bBoolValue, 0);
        let d = data(&mut e, 1, 1, &[1]);
        e.set(other, B::m_spBoolData, 0x4444);
        assert!(!e.call(0x00a5_3220, &args![this, other]).bool());
        e.set(this, B::m_spBoolData, d);
        assert!(e.call(0x00a5_3220, &args![this, other]).bool());
        assert_eq!(calls(&e, DATA_SLOT), vec![vec![d, 0x4444]]);
        e.set(other, B::m_spBoolData, 0x4445);
        assert!(!e.call(0x00a5_3220, &args![this, other]).bool());
        e.set(other, B::m_spBoolData, 0);
        assert!(!e.call(0x00a5_3220, &args![this, other]).bool());
        e.register(OBJECT_IS_EQUAL, |_, _| word(0));
        e.set(this, B::m_spBoolData, 0);
        assert!(!e.call(0x00a5_3220, &args![this, other]).bool());
    }

    #[test]
    fn get_sequence_interpolator_gives_the_copy_the_sequence_data() {
        let mut e = engine();
        let this = e.new_object::<B>();
        let sequence = e.new_object::<B>();
        let old = data(&mut e, 1, 1, &[1]);
        e.set(sequence, B::m_spBoolData, old);
        e.set(sequence, B::m_uiLastIdx, 9);
        let sequence_addr = sequence.addr();
        e.register_double(BASE_GET_SEQUENCE_INTERPOLATOR, move |_, _| {
            word(sequence_addr)
        });
        // Without data: the base's copy as it is.
        assert_eq!(
            e.call(0x00a5_3300, &args![this, 1.0f32, 2.0f32]).u32(),
            sequence.addr()
        );
        assert_eq!(
            calls(&e, BASE_GET_SEQUENCE_INTERPOLATOR),
            vec![vec![this.addr(), 1.0f32.to_bits(), 2.0f32.to_bits()]]
        );
        assert_eq!(e.get(sequence, B::m_uiLastIdx), 9);
        // With data: the new data (one reference from GetSequenceData) is
        // shared, the stack pointer's reference dropped.
        let new = data(&mut e, 0, 1, &[0]);
        e.register_double(BOOL_DATA_GET_SEQUENCE_DATA, move |e, a| {
            assert_eq!(e.mem.u32(a[3]), 0);
            e.mem.set_u32(new + 4, 1);
            e.mem.set_u32(a[3], new);
            Ret::default()
        });
        e.set(this, B::m_spBoolData, 0x5000);
        e.call(0x00a5_3300, &args![this, 1.0f32, 2.0f32]);
        let call = &calls(&e, BOOL_DATA_GET_SEQUENCE_DATA)[0];
        assert_eq!(call[..3], [0x5000, 1.0f32.to_bits(), 2.0f32.to_bits()]);
        assert_eq!(e.get(sequence, B::m_spBoolData), new);
        assert_eq!(e.get(sequence, B::m_uiLastIdx), 0);
        assert_eq!(e.mem.u32(new + 4), 1);
        assert_eq!(calls(&e, DELETE_SLOT), vec![vec![old]]);
    }

    #[test]
    fn save_binary_writes_the_value_and_the_data_link() {
        const WRITE: u32 = 0x0001_0040;
        const LINK: u32 = 0x0001_0050;
        let mut e = engine();
        e.register(BASE_SAVE_BINARY, |_, _| Ret::default());
        e.register(WRITE, |e, a| {
            assert_eq!(e.mem.u32(a[3]), 1);
            word(1)
        });
        e.register(LINK, |_, _| Ret::default());
        let this = e.new_object::<B>();
        e.set(this, B::m_spBoolData, 0x5000);
        let file = e.mem.alloc(0x10);
        e.mem.set_u32(file + 4, 0x20);
        e.mem.set_u32(file + 0xc, WRITE);
        let stream = e.mem.alloc(0x254);
        e.mem.set_u32(stream + 0x250, file);
        let mut slots = vec![0u32; 0xd];
        slots[0x30 / 4] = LINK;
        e.put_vtable(0x0002_1000, &slots);
        e.mem.set_u32(stream, 0x0002_1000);
        e.call(0x00a5_3680, &args![this, stream]);
        assert_eq!(calls(&e, BASE_SAVE_BINARY), vec![vec![this.addr(), stream]]);
        let write = &calls(&e, WRITE)[0];
        assert_eq!(
            (write[0], write[1], write[2], write[4]),
            (file, this.addr() + 0xc, 1, 1)
        );
        assert_eq!(e.mem.u32(file + 4), 0x21);
        assert_eq!(calls(&e, LINK), vec![vec![stream, 0x5000]]);
    }

    #[test]
    fn get_viewer_strings_appends_name_value_and_data() {
        let mut e = engine();
        e.map(NI_BOOL_INTERPOLATOR_RTTI, 4);
        e.set_global(NI_BOOL_INTERPOLATOR_RTTI, 0x0109_1111u32);
        e.register(BASE_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(NI_GET_VIEWER_STRING, |_, _| word(0xa1));
        e.register(NI_GET_VIEWER_STRING_BOOL, |_, _| word(0xa2));
        e.register(NI_GET_VIEWER_STRING_POINTER, |_, _| word(0xa3));
        e.register(STRINGS_GROW, |e, a| {
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
        e.register(STRINGS_SET_AT, |e, a| {
            let size = e.mem.u16(a[0] + 0xa);
            e.mem.set_u16(a[0] + 0xa, size + 1);
            let text = e.mem.u32(a[2]);
            e.mem.set_u32(0x0109_a800 + 4 * a[1], text);
            Ret::default()
        });
        let this = e.new_object::<B>();
        e.set(this, B::m_bBoolValue, 1);
        e.set(this, B::m_spBoolData, 0x5000);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 2);
        e.mem.set_u16(strings + 0xa, 1);
        e.mem.set_u16(strings + 0xe, 4);
        e.call(0x00a5_36d0, &args![this, strings]);
        assert_eq!(calls(&e, NI_GET_VIEWER_STRING), vec![vec![0x0109_1111]]);
        assert_eq!(
            calls(&e, NI_GET_VIEWER_STRING_BOOL),
            vec![vec![NAME_BOOL_VALUE, 1]]
        );
        assert_eq!(
            calls(&e, NI_GET_VIEWER_STRING_POINTER),
            vec![vec![NAME_BOOL_DATA, 0x5000]]
        );
        assert_eq!(calls(&e, STRINGS_GROW), vec![vec![strings, 6]]);
        for (i, text) in [0xa1u32, 0xa2, 0xa3].iter().enumerate() {
            assert_eq!(e.mem.u32(0x0109_a804 + 4 * i as u32), *text);
        }
    }
}
