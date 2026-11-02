//! `NiAnimation/nicolorinterpolator.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiColorInterpolator::ms_RTTI` (the class's `NiRTTI` record).
const NI_COLOR_INTERPOLATOR_RTTI: u32 = 0x011f_3730;

// Translated from 00a3bd70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColorInterpolator::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_color_interpolator_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_COLOR_INTERPOLATOR_RTTI)
}

/// The vtable of `NiColorInterpolator` (RTTI `.?AVNiColorInterpolator@@`).
const NI_COLOR_INTERPOLATOR_VTABLE: u32 = 0x0109_6e14;
/// The colour the constructor starts from and `Update` compares with
/// (`011a9430`, four `float`s; the Xbox PDB's
/// `NiInterpolator::INVALID_COLORA` by its use, not confirmed).
const INVALID_COLOR: u32 = 0x011a_9430;
/// `NiInterpolator::m_fLastTime` (Xbox PDB) +0x08.
const LAST_TIME: u32 = 0x08;
/// `NiColorInterpolator` (Xbox PDB): `m_kColorValue` +0x0c (`NiColorA`,
/// 16 bytes), `m_spColorData` +0x1c (`NiPointer<NiColorData>`),
/// `m_uiLastIdx` +0x20.
const COLOR_VALUE: u32 = 0x0c;
const COLOR_DATA: u32 = 0x1c;
const LAST_INDEX: u32 = 0x20;
/// `NiColorData` (Xbox PDB): `m_uiNumKeys` +0x08, `m_pkKeys` +0x0c,
/// `m_eType` +0x10, `m_ucKeySize` +0x14.
const DATA_NUM_KEYS: u32 = 0x08;
const DATA_KEYS: u32 = 0x0c;
const DATA_KEY_TYPE: u32 = 0x10;
const DATA_KEY_SIZE: u32 = 0x14;
/// `NiKeyBasedInterpolator::NiKeyBasedInterpolator` (`00a3ca10`).
const NI_KEY_BASED_INTERPOLATOR_CONSTRUCT: u32 = 0x00a3_ca10;
/// Import slot of `InterlockedIncrement`.
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
/// `004a4cc0` (`thiscall` on a four-`float` value, `RET 4`): whether it
/// equals the four floats at its argument (true in `AL`).
const COLOR_EQUALS: u32 = 0x004a_4cc0;
/// `NiColorKey::GenInterp` (engine map, `00a296c0`, `cdecl`, 7 words): the
/// colour at a time (result pointer, time, keys, key type, key count,
/// address of the last index, key size); returns the result pointer.
const COLOR_KEY_GEN_INTERP: u32 = 0x00a2_96c0;

/// Copies the four words of a colour.
fn copy_color(e: &mut Engine, to: u32, from: u32) {
    for offset in [0u32, 4, 8, 12] {
        let word = e.mem.u32(from.wrapping_add(offset));
        e.mem.set_u32(to.wrapping_add(offset), word);
    }
}

// Translated from 00a3bd10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColorInterpolator::NiColorInterpolator(NiColorData*)` (by the
/// vtable it stores; unnamed in the engine map): the
/// `NiKeyBasedInterpolator` constructor, the vtable, `m_kColorValue` copied
/// (as words) from `011a9430`, `m_spColorData = data` (its count
/// incremented when non-null) and `m_uiLastIdx = 0`. Returns `this`.
pub fn ni_color_interpolator_construct(e: &mut Engine, this: Ptr, data: Ptr) -> Ptr {
    let base = this.addr();
    e.call(NI_KEY_BASED_INTERPOLATOR_CONSTRUCT, &args![this]);
    e.mem.set_u32(base, NI_COLOR_INTERPOLATOR_VTABLE);
    copy_color(e, base.wrapping_add(COLOR_VALUE), INVALID_COLOR);
    e.mem.set_u32(base.wrapping_add(COLOR_DATA), data.addr());
    if data.addr() != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![data.addr().wrapping_add(4)]);
    }
    e.mem.set_u32(base.wrapping_add(LAST_INDEX), 0);
    this
}

// Translated from 00a3bfb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColorInterpolator::Update` (Xbox PDB, `bool Update(float fTime,
/// NiObjectNET*, NiColorA& kValue)`). At the time of the last update
/// (`time == m_fLastTime`, an x87 `FUCOMP`: NaN is not equal) it copies
/// `m_kColorValue` into `value` and returns whether that differs from the
/// colour at `011a9430`. Otherwise, with colour data holding keys, the
/// colour at `time` (`NiColorKey::GenInterp`, the time passed as the
/// `float` an x87 `FSTP` stores) becomes `m_kColorValue`; then, when
/// `m_kColorValue` equals the colour at `011a9430` it returns false,
/// else copies it into `value`, stores `time` as `m_fLastTime` (an x87
/// copy) and returns true. The key-size word pushed for `GenInterp` holds
/// the size byte in its low byte; the exe leaves the upper three bytes of
/// that stack word as they were (not modelled: they are passed as zero).
pub fn ni_color_interpolator_update(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    _target: Ptr,
    value: Ptr,
) -> bool {
    let base = this.addr();
    let color = base.wrapping_add(COLOR_VALUE);
    let last_time = e.mem.f32(base.wrapping_add(LAST_TIME));
    if time == last_time {
        copy_color(e, value.addr(), color);
        return !e.call(COLOR_EQUALS, &args![color, INVALID_COLOR]).bool();
    }
    let data = e.mem.u32(base.wrapping_add(COLOR_DATA));
    if data != 0 {
        let num_keys = e.mem.u32(data.wrapping_add(DATA_NUM_KEYS));
        let key_size = e.mem.u8(data.wrapping_add(DATA_KEY_SIZE));
        let key_type = e.mem.u32(data.wrapping_add(DATA_KEY_TYPE));
        let keys = e.mem.u32(data.wrapping_add(DATA_KEYS));
        if num_keys != 0 {
            e.with_stack(16, |e, result| {
                let interpolated = e
                    .call(
                        COLOR_KEY_GEN_INTERP,
                        &args![
                            result,
                            x87_quiet_bits(time.to_bits()),
                            keys,
                            key_type,
                            num_keys,
                            base.wrapping_add(LAST_INDEX),
                            u32::from(key_size)
                        ],
                    )
                    .u32();
                copy_color(e, color, interpolated);
            });
        }
    }
    if e.call(COLOR_EQUALS, &args![color, INVALID_COLOR]).bool() {
        return false;
    }
    copy_color(e, value.addr(), color);
    e.mem
        .set_u32(base.wrapping_add(LAST_TIME), x87_quiet_bits(time.to_bits()));
    true
}

/// `InterlockedDecrement` (import slot).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// Virtual slots (Xbox PDB names): `NiRefObject::DeleteThis` +0x04,
/// `NiObject::RegisterStreamables` +0x54, `NiObject::IsEqual` +0x5c and
/// `NiStream::SaveLinkID` +0x30.
const SLOT_DELETE_THIS: u32 = 0x04;
const SLOT_REGISTER_STREAMABLES: u32 = 0x54;
const SLOT_IS_EQUAL: u32 = 0x5c;
const SLOT_SAVE_LINK_ID: u32 = 0x30;
/// `NiColorInterpolator`'s size (the `CreateClone` and `CreateObject`
/// allocations, and the scalar deleting destructor's delete).
const NI_COLOR_INTERPOLATOR_SIZE: u32 = 0x24;
/// `00aa13e0` (`cdecl`, one word): allocates from the Ni allocator.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `00aa1460` (`cdecl`: block, size): the sized delete.
const SIZED_DELETE: u32 = 0x00aa_1460;
/// `00a3ca30` (unnamed in the engine map, `nikeybasedinterpolator.cpp`):
/// the base destructor the scalar deleting destructor runs.
const KEY_BASED_DESTRUCT: u32 = 0x00a3_ca30;
/// `NiKeyBasedInterpolator::CopyMembers` (Ghidra's name for `00aa0ab0`;
/// `thiscall`, `RET 8`: destination, cloning process).
const KEY_BASED_COPY_MEMBERS: u32 = 0x00aa_0ab0;
/// `00aa0ad0` (Ghidra's `NiObject::RegisterStreamables`; `thiscall`, `RET
/// 4`: stream): the base `RegisterStreamables`.
const BASE_REGISTER_STREAMABLES: u32 = 0x00aa_0ad0;
/// `00c85800` (Ghidra's `NiObject::IsEqual`; `thiscall`, `RET 4`): the base
/// `IsEqual`.
const BASE_IS_EQUAL: u32 = 0x00c8_5800;
/// `00a95ca0` (`thiscall`, `RET 4`: stream; an empty body the engine map
/// names `BSGrabIKNote::LinkObject`): the base `SaveBinary` call.
const BASE_SAVE_BINARY: u32 = 0x00a9_5ca0;
/// `NiColorA::SaveBinary` (`00a698a0`; `thiscall` on the colour: stream).
const COLOR_SAVE_BINARY: u32 = 0x00a6_98a0;
/// `004a4c90` (`thiscall` on a colour, `RET 4`): true in `AL` when the four
/// floats differ from those at its argument (the negation of `004a4cc0`).
const COLOR_DIFFERS: u32 = 0x004a_4c90;
/// `NiColorData::GuaranteeKeysAtStartAndEnd` (`00a3d0e0`; `thiscall`:
/// start, end).
const COLOR_DATA_GUARANTEE_KEYS: u32 = 0x00a3_d0e0;
/// `NiColorData::GetSequenceData` (`00a3d270`; `thiscall`: start, end,
/// address of the result pointer).
const COLOR_DATA_GET_SEQUENCE_DATA: u32 = 0x00a3_d270;
/// `NiInterpolator::GetSequenceInterpolator` (`00a57050`; `thiscall`:
/// start, end; answers the new interpolator).
const INTERPOLATOR_GET_SEQUENCE_INTERPOLATOR: u32 = 0x00a5_7050;
/// `NiKeyBasedInterpolator::GetViewerStrings` (`00a3cbf0`).
const KEY_BASED_GET_VIEWER_STRINGS: u32 = 0x00a3_cbf0;
/// `NiGetViewerString` (`00a7d330`, `cdecl`: the class name).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
/// `NiColorA::GetViewerString` (`00a69720`; `thiscall` on the colour: the
/// label).
const COLOR_GET_VIEWER_STRING: u32 = 0x00a6_9720;
/// `NiGetViewerString_ov10` (engine map; `00a7d5a0`, `cdecl`: label,
/// pointer).
const NI_GET_VIEWER_STRING_POINTER: u32 = 0x00a7_d5a0;
/// The viewer-string array's grow (`0096ad30`; `thiscall`: new size) and
/// store (`0096ae90`; `thiscall`: index, address of the string).
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
/// `"m_kColorValue"` and `"m_spColorData"`.
const LABEL_COLOR_VALUE: u32 = 0x0109_6f1c;
const LABEL_COLOR_DATA: u32 = 0x0109_6f0c;
/// The class name: the first word of the `NiRTTI` record.
const CLASS_NAME: u32 = NI_COLOR_INTERPOLATOR_RTTI;

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

/// Releases `m_spColorData` (when set) and clears it.
fn clear_color_data(e: &mut Engine, this: u32) {
    let data = e.mem.u32(this.wrapping_add(COLOR_DATA));
    if data != 0 {
        release(e, data);
        e.mem.set_u32(this.wrapping_add(COLOR_DATA), 0);
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

// Translated from 00a3bbf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColorInterpolator::GetKeyContent` (Xbox PDB): 3 for any channel (the
/// channel word is not read).
pub fn ni_color_interpolator_get_key_content(_e: &mut Engine, _this: Ptr, _unused_1: u32) -> u32 {
    3
}

// Translated from 00a3bc00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColorInterpolator::GetKeyCount` (Xbox PDB): the colour data's
/// `m_uiNumKeys`, 0 without colour data (the channel word is not read).
pub fn ni_color_interpolator_get_key_count(e: &mut Engine, this: Ptr, _unused_1: u32) -> u32 {
    let data = e.mem.u32(this.addr().wrapping_add(COLOR_DATA));
    if data == 0 {
        return 0;
    }
    e.mem.u32(data.wrapping_add(DATA_NUM_KEYS))
}

// Translated from 00a3bc10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColorInterpolator::GetKeyType` (Xbox PDB): the colour data's
/// `m_eType`, 0 without colour data (the channel word is not read).
pub fn ni_color_interpolator_get_key_type(e: &mut Engine, this: Ptr, _unused_1: u32) -> u32 {
    let data = e.mem.u32(this.addr().wrapping_add(COLOR_DATA));
    if data == 0 {
        return 0;
    }
    e.mem.u32(data.wrapping_add(DATA_KEY_TYPE))
}

// Translated from 00a3bc20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColorInterpolator::GetKeyStride` (Xbox PDB): the colour data's
/// `m_ucKeySize`, 0 without colour data (the channel word is not read).
pub fn ni_color_interpolator_get_key_stride(e: &mut Engine, this: Ptr, _unused_1: u32) -> u8 {
    let data = e.mem.u32(this.addr().wrapping_add(COLOR_DATA));
    if data == 0 {
        return 0;
    }
    e.mem.u8(data.wrapping_add(DATA_KEY_SIZE))
}

// Translated from 00a487f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColorInterpolator::GetKeyArray` (Xbox PDB): the colour data's
/// `m_pkKeys`, 0 without colour data (the channel word is not read).
pub fn ni_color_interpolator_get_key_array(e: &mut Engine, this: Ptr, _unused_1: u32) -> u32 {
    let data = e.mem.u32(this.addr().wrapping_add(COLOR_DATA));
    if data == 0 {
        return 0;
    }
    e.mem.u32(data.wrapping_add(DATA_KEYS))
}

// Translated from 00a3bc40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColorInterpolator::GetChannelPosed` (Xbox PDB): false with colour
/// data, otherwise whether `m_kColorValue` differs from the colour at
/// `011a9430` (`004a4cc0`'s answer inverted). The channel word is not read.
pub fn ni_color_interpolator_get_channel_posed(e: &mut Engine, this: Ptr, _unused_1: u32) -> bool {
    let base = this.addr();
    if e.mem.u32(base.wrapping_add(COLOR_DATA)) != 0 {
        return false;
    }
    let color = base.wrapping_add(COLOR_VALUE);
    !e.call(COLOR_EQUALS, &args![color, INVALID_COLOR]).bool()
}

// Translated from 00a3bc60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0x10`: a colour by value, four
/// words): releases and clears the colour data, then stores the four words
/// as `m_kColorValue` (the colour pose value; the Xbox name is not
/// confirmed).
pub fn fn_00a3bc60(e: &mut Engine, this: Ptr, red: u32, green: u32, blue: u32, alpha: u32) {
    let base = this.addr();
    clear_color_data(e, base);
    for (offset, word) in [(0u32, red), (4, green), (8, blue), (12, alpha)] {
        e.mem.set_u32(base.wrapping_add(COLOR_VALUE + offset), word);
    }
}

// Translated from 00a3bcb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColorInterpolator::GuaranteeTimeRange` (Xbox PDB): with colour data,
/// `NiColorData::GuaranteeKeysAtStartAndEnd(start, end)` (both floats
/// copied through the x87 stack).
pub fn ni_color_interpolator_guarantee_time_range(e: &mut Engine, this: Ptr, start: f32, end: f32) {
    let data = e.mem.u32(this.addr().wrapping_add(COLOR_DATA));
    if data == 0 {
        return;
    }
    e.call(
        COLOR_DATA_GUARANTEE_KEYS,
        &args![
            data,
            x87_quiet_bits(start.to_bits()),
            x87_quiet_bits(end.to_bits())
        ],
    );
}

// Translated from 00a3bce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColorInterpolator::SaveBinary` (Xbox PDB): the base call (`00a95ca0`),
/// `NiColorA::SaveBinary` of `m_kColorValue`, then the stream's `SaveLinkID`
/// (virtual slot +0x30) of the colour data.
pub fn ni_color_interpolator_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let base = this.addr();
    e.call(BASE_SAVE_BINARY, &args![this, stream]);
    e.call(
        COLOR_SAVE_BINARY,
        &args![base.wrapping_add(COLOR_VALUE), stream],
    );
    let data = e.mem.u32(base.wrapping_add(COLOR_DATA));
    e.vcall(stream.addr(), SLOT_SAVE_LINK_ID, &args![data]);
}

// Translated from 00a3bd80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColorInterpolator::NiColorInterpolator` (Xbox PDB), the constructor
/// taking a colour by value (four words, `RET 0x10`): the
/// `NiKeyBasedInterpolator` constructor, the colour words as
/// `m_kColorValue`, the vtable, no colour data and last index 0. Returns
/// `this`.
pub fn ni_color_interpolator_construct_with_color(
    e: &mut Engine,
    this: Ptr,
    red: u32,
    green: u32,
    blue: u32,
    alpha: u32,
) -> Ptr {
    let base = this.addr();
    e.call(NI_KEY_BASED_INTERPOLATOR_CONSTRUCT, &args![this]);
    for (offset, word) in [(0u32, red), (4, green), (8, blue), (12, alpha)] {
        e.mem.set_u32(base.wrapping_add(COLOR_VALUE + offset), word);
    }
    e.mem.set_u32(base, NI_COLOR_INTERPOLATOR_VTABLE);
    e.mem.set_u32(base.wrapping_add(COLOR_DATA), 0);
    e.mem.set_u32(base.wrapping_add(LAST_INDEX), 0);
    this
}

// Translated from 00a3bdc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColorInterpolator::Collapse` (Xbox PDB). Without colour data nothing
/// happens. With no keys the data is released and `m_kColorValue` becomes
/// the colour at `011a9430`. With one key, or with linear (1) or step (5)
/// keys that all hold the first key's colour (`004a4c90` on each later key,
/// the stride `m_ucKeySize`, the colour at +4 of a key), the data is
/// released and the first key's colour becomes `m_kColorValue`; any other
/// key type, or a differing key, leaves everything as it is.
pub fn ni_color_interpolator_collapse(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let data = e.mem.u32(base.wrapping_add(COLOR_DATA));
    if data == 0 {
        return;
    }
    let num_keys = e.mem.u32(data.wrapping_add(DATA_NUM_KEYS));
    let key_size = e.mem.u8(data.wrapping_add(DATA_KEY_SIZE));
    let keys = e.mem.u32(data.wrapping_add(DATA_KEYS));
    let key_type = e.mem.u32(data.wrapping_add(DATA_KEY_TYPE));
    if num_keys == 0 {
        // `ESI` is the data pointer here, never null.
        release(e, data);
        e.mem.set_u32(base.wrapping_add(COLOR_DATA), 0);
        copy_color(e, base.wrapping_add(COLOR_VALUE), INVALID_COLOR);
        return;
    }
    e.with_stack(16, |e, first| {
        let first = first.addr();
        copy_color(e, first, keys.wrapping_add(4));
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
                if e.call(COLOR_DIFFERS, &args![key, first]).bool() {
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
        clear_color_data(e, base);
        copy_color(e, base.wrapping_add(COLOR_VALUE), first);
    });
}

// Translated from 00a3bef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColorInterpolator::GetSequenceInterpolator` (Xbox PDB): the base
/// class's copy (`NiInterpolator::GetSequenceInterpolator`, `00a57050`,
/// with `start` and `end` copied through the x87 stack). With colour data,
/// `NiColorData::GetSequenceData(start, end, &sequence)` fills a pointer
/// starting at 0, which becomes the copy's colour data (the old one
/// released, the new one counted, when they differ); the copy's last index
/// becomes 0 and the local pointer is released. Returns the copy.
pub fn ni_color_interpolator_get_sequence_interpolator(
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
    let data = e.mem.u32(this.addr().wrapping_add(COLOR_DATA));
    if data == 0 {
        return Ptr::new(copy);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), 0);
        e.call(COLOR_DATA_GET_SEQUENCE_DATA, &args![data, start, end, slot]);
        let sequence = e.mem.u32(slot.addr());
        let old = e.mem.u32(copy.wrapping_add(COLOR_DATA));
        if old != sequence {
            if old != 0 {
                release(e, old);
            }
            e.mem.set_u32(copy.wrapping_add(COLOR_DATA), sequence);
            if sequence != 0 {
                e.call(INTERLOCKED_INCREMENT, &args![sequence.wrapping_add(4)]);
            }
        }
        e.mem.set_u32(copy.wrapping_add(LAST_INDEX), 0);
        let sequence = e.mem.u32(slot.addr());
        if sequence != 0 {
            release(e, sequence);
        }
    });
    Ptr::new(copy)
}

// Translated from 00a3c0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColorInterpolator::CopyMembers` (Xbox PDB; `thiscall`, `RET 8`:
/// destination, cloning process): the `NiKeyBasedInterpolator` copy
/// (`00aa0ab0`), `m_kColorValue` (as words), the colour data pointer (when
/// it differs: the old one released, the new one counted) and the last
/// index.
pub fn ni_color_interpolator_copy_members(
    e: &mut Engine,
    this: Ptr,
    destination: Ptr,
    cloning: u32,
) {
    e.call(KEY_BASED_COPY_MEMBERS, &args![this, destination, cloning]);
    let source = this.addr();
    let destination = destination.addr();
    copy_color(
        e,
        destination.wrapping_add(COLOR_VALUE),
        source.wrapping_add(COLOR_VALUE),
    );
    let old = e.mem.u32(destination.wrapping_add(COLOR_DATA));
    if old != e.mem.u32(source.wrapping_add(COLOR_DATA)) {
        if old != 0 {
            release(e, old);
        }
        let data = e.mem.u32(source.wrapping_add(COLOR_DATA));
        e.mem.set_u32(destination.wrapping_add(COLOR_DATA), data);
        if data != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![data.wrapping_add(4)]);
        }
    }
    let last_index = e.mem.u32(source.wrapping_add(LAST_INDEX));
    e.mem
        .set_u32(destination.wrapping_add(LAST_INDEX), last_index);
}

/// The constructor `CreateObject` and `CreateClone` inline: the
/// `NiKeyBasedInterpolator` constructor, the vtable, `m_kColorValue` from
/// `011a9430`, no colour data, last index 0.
fn construct_inline(e: &mut Engine, object: u32) {
    e.call(NI_KEY_BASED_INTERPOLATOR_CONSTRUCT, &args![object]);
    e.mem.set_u32(object, NI_COLOR_INTERPOLATOR_VTABLE);
    copy_color(e, object.wrapping_add(COLOR_VALUE), INVALID_COLOR);
    e.mem.set_u32(object.wrapping_add(COLOR_DATA), 0);
    e.mem.set_u32(object.wrapping_add(LAST_INDEX), 0);
}

// Translated from 00a3c130 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `NiColorInterpolator::CreateObject` (Ghidra has no function here; the
/// loader's factory by its body): allocates 0x24 bytes and, when that
/// succeeds, constructs a default `NiColorInterpolator` in them (no colour
/// data, the colour at `011a9430`). Returns the object, or 0.
pub fn ni_color_interpolator_create_object(e: &mut Engine) -> Ptr {
    let object = e
        .call(NI_ALLOCATE, &args![NI_COLOR_INTERPOLATOR_SIZE])
        .u32();
    if object == 0 {
        return Ptr::new(0);
    }
    construct_inline(e, object);
    Ptr::new(object)
}

// Translated from 00a3c190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColorInterpolator::IsEqual` (Xbox PDB): the base `IsEqual`
/// (`00c85800`), equal colours (`004a4c90` false), and either both without
/// colour data or both with colour data that the first's `IsEqual`
/// (virtual slot +0x5c) finds equal.
pub fn ni_color_interpolator_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e.call(BASE_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let base = this.addr();
    let other = other.addr();
    if e.call(
        COLOR_DIFFERS,
        &args![
            base.wrapping_add(COLOR_VALUE),
            other.wrapping_add(COLOR_VALUE)
        ],
    )
    .bool()
    {
        return false;
    }
    let data = e.mem.u32(base.wrapping_add(COLOR_DATA));
    let other_data = e.mem.u32(other.wrapping_add(COLOR_DATA));
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
    let other_data = e.mem.u32(other.wrapping_add(COLOR_DATA));
    e.vcall(data, SLOT_IS_EQUAL, &args![other_data]).bool()
}

// Translated from 00a3c1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColorInterpolator::CreateClone` (Xbox PDB; `RET 4`: cloning process):
/// allocates 0x24 bytes and constructs a default `NiColorInterpolator` in
/// them, then `CopyMembers` into it; when the allocation fails the copy
/// still runs, into address 0. Returns the clone, or 0.
pub fn ni_color_interpolator_create_clone(e: &mut Engine, this: Ptr, cloning: u32) -> Ptr {
    let object = e
        .call(NI_ALLOCATE, &args![NI_COLOR_INTERPOLATOR_SIZE])
        .u32();
    if object == 0 {
        ni_color_interpolator_copy_members(e, this, Ptr::new(0), cloning);
        return Ptr::new(0);
    }
    construct_inline(e, object);
    ni_color_interpolator_copy_members(e, this, Ptr::new(object), cloning);
    Ptr::new(object)
}

// Translated from 00a3c270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColorInterpolator::GetViewerStrings` (Xbox PDB): the
/// `NiKeyBasedInterpolator` strings, then three more: the class name (the
/// `NiRTTI` record's first word through `NiGetViewerString`),
/// `m_kColorValue` (`NiColorA::GetViewerString("m_kColorValue")`) and the
/// colour data pointer (`NiGetViewerString_ov10("m_spColorData", data)`).
pub fn ni_color_interpolator_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    let base = this.addr();
    let strings = strings.addr();
    e.call(KEY_BASED_GET_VIEWER_STRINGS, &args![this, strings]);
    let name = e.mem.u32(CLASS_NAME);
    let text = e.call(NI_GET_VIEWER_STRING, &args![name]).u32();
    add_viewer_string(e, strings, text);
    let text = e
        .call(
            COLOR_GET_VIEWER_STRING,
            &args![base.wrapping_add(COLOR_VALUE), LABEL_COLOR_VALUE],
        )
        .u32();
    add_viewer_string(e, strings, text);
    let data = e.mem.u32(base.wrapping_add(COLOR_DATA));
    let text = e
        .call(NI_GET_VIEWER_STRING_POINTER, &args![LABEL_COLOR_DATA, data])
        .u32();
    add_viewer_string(e, strings, text);
}

// Translated from 00a489f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColorInterpolator::RegisterStreamables` (Xbox PDB): false when the
/// base `RegisterStreamables` (`00aa0ad0`) answers false; otherwise the
/// colour data's `RegisterStreamables` (virtual slot +0x54, when there is
/// data) and true.
pub fn ni_color_interpolator_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    if !e
        .call(BASE_REGISTER_STREAMABLES, &args![this, stream])
        .bool()
    {
        return false;
    }
    let data = e.mem.u32(this.addr().wrapping_add(COLOR_DATA));
    if data != 0 {
        e.vcall(data, SLOT_REGISTER_STREAMABLES, &args![stream]);
    }
    true
}

// Translated from 00a48dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColorInterpolator::_scalar_deleting_destructor_` (Xbox PDB): releases
/// the colour data (the pointer is left as it is), runs the base destructor
/// `00a3ca30`, then the sized delete of 0x24 bytes when bit 0 of `flags` is
/// set. Returns `this`.
pub fn ni_color_interpolator_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    let data = e.mem.u32(this.addr().wrapping_add(COLOR_DATA));
    if data != 0 {
        release(e, data);
    }
    e.call(KEY_BASED_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, NI_COLOR_INTERPOLATOR_SIZE]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a3bd70, ni_color_interpolator_get_rtti() -> Ptr),
        entry!(
            0x00a3_bd10,
            ni_color_interpolator_construct(Ptr, Ptr) -> Ptr
        ),
        entry!(
            0x00a3_bfb0,
            ni_color_interpolator_update(Ptr, f32, Ptr, Ptr) -> bool
        ),
        entry!(0x00a3_bbf0, ni_color_interpolator_get_key_content(Ptr, u32) -> u32),
        entry!(0x00a3_bc00, ni_color_interpolator_get_key_count(Ptr, u32) -> u32),
        entry!(0x00a3_bc10, ni_color_interpolator_get_key_type(Ptr, u32) -> u32),
        entry!(0x00a3_bc20, ni_color_interpolator_get_key_stride(Ptr, u32) -> u8),
        entry!(0x00a4_87f0, ni_color_interpolator_get_key_array(Ptr, u32) -> u32),
        entry!(0x00a3_bc40, ni_color_interpolator_get_channel_posed(Ptr, u32) -> bool),
        entry!(0x00a3_bc60, fn_00a3bc60(Ptr, u32, u32, u32, u32)),
        entry!(
            0x00a3_bcb0,
            ni_color_interpolator_guarantee_time_range(Ptr, f32, f32)
        ),
        entry!(0x00a3_bce0, ni_color_interpolator_save_binary(Ptr, Ptr)),
        entry!(
            0x00a3_bd80,
            ni_color_interpolator_construct_with_color(Ptr, u32, u32, u32, u32) -> Ptr
        ),
        entry!(0x00a3_bdc0, ni_color_interpolator_collapse(Ptr)),
        entry!(
            0x00a3_bef0,
            ni_color_interpolator_get_sequence_interpolator(Ptr, f32, f32) -> Ptr
        ),
        entry!(
            0x00a3_c0a0,
            ni_color_interpolator_copy_members(Ptr, Ptr, u32)
        ),
        entry!(0x00a3_c130, ni_color_interpolator_create_object() -> Ptr),
        entry!(0x00a3_c190, ni_color_interpolator_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00a3_c1f0, ni_color_interpolator_create_clone(Ptr, u32) -> Ptr),
        entry!(
            0x00a3_c270,
            ni_color_interpolator_get_viewer_strings(Ptr, Ptr)
        ),
        entry!(0x00a4_89f0, ni_color_interpolator_register_streamables(Ptr, Ptr) -> bool),
        entry!(
            0x00a4_8dd0,
            ni_color_interpolator_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_color_interpolator_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a3bd70, &args![]).u32(),
            NI_COLOR_INTERPOLATOR_RTTI
        );
    }

    type Log = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

    const INVALID: [f32; 4] = [-f32::MAX, -f32::MAX, -f32::MAX, -f32::MAX];

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn colors(e: &Engine, at: u32) -> [f32; 4] {
        [0, 4, 8, 12].map(|offset| e.mem.f32(at + offset))
    }

    fn set_colors(e: &mut Engine, at: u32, values: [f32; 4]) {
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(at + 4 * i as u32, *v);
        }
    }

    /// An interpolator (last time 1.0, colour `start`) with data holding
    /// `num_keys` keys; `GenInterp` writes `interpolated`.
    fn fixture(num_keys: u32, start: [f32; 4], interpolated: [f32; 4]) -> (Engine, Log, u32, u32) {
        let mut e = Engine::new();
        let log = Log::default();
        e.map(INVALID_COLOR, 16);
        set_colors(&mut e, INVALID_COLOR, INVALID);
        e.register(COLOR_EQUALS, |e, a| {
            ret((colors(e, a[0]) == colors(e, a[1])) as u32)
        });
        let l = log.clone();
        e.register_double(COLOR_KEY_GEN_INTERP, move |e, a| {
            l.borrow_mut().push((COLOR_KEY_GEN_INTERP, a[1..].to_vec()));
            set_colors(e, a[0], interpolated);
            ret(a[0])
        });
        let this = e.mem.alloc(0x24);
        e.mem.set_f32(this + LAST_TIME, 1.0);
        set_colors(&mut e, this + COLOR_VALUE, start);
        let data = e.mem.alloc(0x18);
        e.mem.set_u32(data + DATA_NUM_KEYS, num_keys);
        e.mem.set_u32(data + DATA_KEYS, 0x7000);
        e.mem.set_u32(data + DATA_KEY_TYPE, 2);
        e.mem.set_u8(data + DATA_KEY_SIZE, 0x14);
        e.mem.set_u32(this + COLOR_DATA, data);
        let value = e.mem.alloc(16);
        (e, log, this, value)
    }

    #[test]
    fn constructor_starts_from_the_invalid_colour_and_holds_the_data() {
        let mut e = Engine::new();
        e.register(NI_KEY_BASED_INTERPOLATOR_CONSTRUCT, |_, _| Ret::default());
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let value = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], value);
            ret(value)
        });
        e.map(INVALID_COLOR, 16);
        // Word copies: a signalling NaN stays as it is.
        e.mem.set_u32(INVALID_COLOR, 0x7fa0_0001);
        e.mem.set_u32(INVALID_COLOR + 12, 0x1234_5678);
        let this = e.mem.alloc(0x24);
        e.mem.set_u32(this + LAST_INDEX, 3);
        let data = e.mem.alloc(0x18);
        assert_eq!(e.call(0x00a3_bd10, &args![this, data]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_COLOR_INTERPOLATOR_VTABLE);
        assert_eq!(e.mem.u32(this + COLOR_VALUE), 0x7fa0_0001);
        assert_eq!(e.mem.u32(this + COLOR_VALUE + 12), 0x1234_5678);
        assert_eq!(e.mem.u32(this + COLOR_DATA), data);
        assert_eq!(e.mem.u32(data + 4), 1);
        assert_eq!(e.mem.u32(this + LAST_INDEX), 0);
        e.call(0x00a3_bd10, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this + COLOR_DATA), 0);
        assert_eq!(e.mem.u32(data + 4), 1);
    }

    #[test]
    fn update_at_the_last_time_copies_the_current_colour() {
        let current = [0.5, 0.25, 1.0, 1.0];
        let (mut e, log, this, value) = fixture(3, current, [0.0; 4]);
        assert!(e
            .call(0x00a3_bfb0, &args![this, 1.0f32, 0u32, value])
            .bool());
        assert_eq!(colors(&e, value), current);
        assert!(log.borrow().is_empty());
        // The same time with the invalid colour answers false, still copied.
        set_colors(&mut e, this + COLOR_VALUE, INVALID);
        assert!(!e
            .call(0x00a3_bfb0, &args![this, 1.0f32, 0u32, value])
            .bool());
        assert_eq!(colors(&e, value), INVALID);
    }

    #[test]
    fn update_at_a_new_time_interpolates_the_keys() {
        let new = [0.1, 0.2, 0.3, 0.4];
        let (mut e, log, this, value) = fixture(3, [0.0; 4], new);
        assert!(e
            .call(0x00a3_bfb0, &args![this, 2.5f32, 0u32, value])
            .bool());
        assert_eq!(
            *log.borrow(),
            [(
                COLOR_KEY_GEN_INTERP,
                vec![2.5f32.to_bits(), 0x7000, 2, 3, this + LAST_INDEX, 0x14]
            )]
        );
        assert_eq!(colors(&e, this + COLOR_VALUE), new);
        assert_eq!(colors(&e, value), new);
        assert_eq!(e.mem.f32(this + LAST_TIME), 2.5);
    }

    #[test]
    fn update_with_a_nan_time_interpolates_with_the_quieted_time() {
        let new = [0.1, 0.2, 0.3, 0.4];
        let (mut e, log, this, value) = fixture(3, [0.0; 4], new);
        e.mem.set_u32(this + LAST_TIME, 0x7fa0_0000);
        let time = f32::from_bits(0x7fa0_0000);
        assert!(e.call(0x00a3_bfb0, &args![this, time, 0u32, value]).bool());
        assert_eq!(log.borrow()[0].1[0], 0x7fe0_0000);
        assert_eq!(e.mem.u32(this + LAST_TIME), 0x7fe0_0000);
    }

    #[test]
    fn update_without_keys_keeps_the_colour() {
        let current = [0.5, 0.5, 0.5, 1.0];
        let (mut e, log, this, value) = fixture(0, current, [0.0; 4]);
        assert!(e
            .call(0x00a3_bfb0, &args![this, 2.0f32, 0u32, value])
            .bool());
        assert!(log.borrow().is_empty());
        assert_eq!(colors(&e, value), current);
        assert_eq!(e.mem.f32(this + LAST_TIME), 2.0);
        // No data at all, and the invalid colour: false, nothing stored.
        e.mem.set_u32(this + COLOR_DATA, 0);
        set_colors(&mut e, this + COLOR_VALUE, INVALID);
        set_colors(&mut e, value, [9.0; 4]);
        assert!(!e
            .call(0x00a3_bfb0, &args![this, 3.0f32, 0u32, value])
            .bool());
        assert_eq!(colors(&e, value), [9.0; 4]);
        assert_eq!(e.mem.f32(this + LAST_TIME), 2.0);
    }
}

#[cfg(test)]
mod n003_tests {
    use super::*;

    type Log = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

    const DELETE_THIS: u32 = 0x00ee_0500;
    const DATA_IS_EQUAL: u32 = 0x00ee_0600;
    const DATA_REGISTER: u32 = 0x00ee_0700;
    const SAVE_LINK_ID: u32 = 0x00ee_0800;

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

    fn colors(e: &Engine, at: u32) -> [u32; 4] {
        [0, 4, 8, 12].map(|offset| e.mem.u32(at + offset))
    }

    fn set_colors(e: &mut Engine, at: u32, values: [u32; 4]) {
        for (i, v) in values.iter().enumerate() {
            e.mem.set_u32(at + 4 * i as u32, *v);
        }
    }

    /// An engine with counting `Interlocked` doubles, logged `DeleteThis`
    /// and data `RegisterStreamables`, and the colour at `011a9430` set to
    /// four words 0xff7fffff.
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
        logged(&mut e, log, DATA_REGISTER, 0);
        e.map(INVALID_COLOR, 16);
        set_colors(&mut e, INVALID_COLOR, [0xff7f_ffff; 4]);
        e
    }

    /// A counted object (count `count`) whose vtable has `DeleteThis`,
    /// `RegisterStreamables` and `IsEqual` doubles.
    fn counted(e: &mut Engine, count: u32) -> u32 {
        let vtable = e.mem.alloc(0x60);
        e.mem.set_u32(vtable + SLOT_DELETE_THIS, DELETE_THIS);
        e.mem
            .set_u32(vtable + SLOT_REGISTER_STREAMABLES, DATA_REGISTER);
        e.mem.set_u32(vtable + SLOT_IS_EQUAL, DATA_IS_EQUAL);
        let object = e.mem.alloc(0x18);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, count);
        object
    }

    fn interpolator(e: &mut Engine) -> u32 {
        e.mem.alloc(NI_COLOR_INTERPOLATOR_SIZE)
    }

    #[test]
    fn get_key_content_is_always_three() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a3_bbf0, &args![0x5000u32, 7u32]).u32(), 3);
    }

    /// Data at +0x1c with the four key fields set.
    fn with_data(e: &mut Engine) -> u32 {
        let this = interpolator(e);
        let data = e.mem.alloc(0x18);
        e.mem.set_u32(data + DATA_NUM_KEYS, 6);
        e.mem.set_u32(data + DATA_KEYS, 0x7000);
        e.mem.set_u32(data + DATA_KEY_TYPE, 2);
        e.mem.set_u8(data + DATA_KEY_SIZE, 0x14);
        e.mem.set_u32(this + COLOR_DATA, data);
        this
    }

    #[test]
    fn get_key_count_reads_the_data_or_answers_zero() {
        let mut e = Engine::new();
        let this = with_data(&mut e);
        assert_eq!(e.call(0x00a3_bc00, &args![this, 0u32]).u32(), 6);
        e.mem.set_u32(this + COLOR_DATA, 0);
        assert_eq!(e.call(0x00a3_bc00, &args![this, 0u32]).u32(), 0);
    }

    #[test]
    fn get_key_type_reads_the_data_or_answers_zero() {
        let mut e = Engine::new();
        let this = with_data(&mut e);
        assert_eq!(e.call(0x00a3_bc10, &args![this, 0u32]).u32(), 2);
        e.mem.set_u32(this + COLOR_DATA, 0);
        assert_eq!(e.call(0x00a3_bc10, &args![this, 0u32]).u32(), 0);
    }

    #[test]
    fn get_key_stride_reads_the_data_or_answers_zero() {
        let mut e = Engine::new();
        let this = with_data(&mut e);
        assert_eq!(e.call(0x00a3_bc20, &args![this, 0u32]).u8(), 0x14);
        e.mem.set_u32(this + COLOR_DATA, 0);
        assert_eq!(e.call(0x00a3_bc20, &args![this, 0u32]).u8(), 0);
    }

    #[test]
    fn get_key_array_reads_the_data_or_answers_zero() {
        let mut e = Engine::new();
        let this = with_data(&mut e);
        assert_eq!(e.call(0x00a4_87f0, &args![this, 0u32]).u32(), 0x7000);
        e.mem.set_u32(this + COLOR_DATA, 0);
        assert_eq!(e.call(0x00a4_87f0, &args![this, 0u32]).u32(), 0);
    }

    #[test]
    fn get_channel_posed_needs_no_data_and_a_valid_colour() {
        let log = Log::default();
        let mut e = engine(&log);
        e.register(COLOR_EQUALS, |e, a| {
            ret((colors(e, a[0]) == colors(e, a[1])) as u32)
        });
        let this = with_data(&mut e);
        assert!(!e.call(0x00a3_bc40, &args![this, 0u32]).bool());
        e.mem.set_u32(this + COLOR_DATA, 0);
        set_colors(&mut e, this + COLOR_VALUE, [0x3f80_0000; 4]);
        assert!(e.call(0x00a3_bc40, &args![this, 0u32]).bool());
        set_colors(&mut e, this + COLOR_VALUE, [0xff7f_ffff; 4]);
        assert!(!e.call(0x00a3_bc40, &args![this, 0u32]).bool());
    }

    #[test]
    fn pose_colour_setter_releases_the_data_and_stores_the_words() {
        let log = Log::default();
        let mut e = engine(&log);
        let this = interpolator(&mut e);
        let data = counted(&mut e, 1);
        e.mem.set_u32(this + COLOR_DATA, data);
        e.call(0x00a3_bc60, &args![this, 1u32, 2u32, 0x7f80_0001u32, 4u32]);
        assert_eq!(calls(&log, DELETE_THIS), vec![vec![data]]);
        assert_eq!(e.mem.u32(this + COLOR_DATA), 0);
        assert_eq!(colors(&e, this + COLOR_VALUE), [1, 2, 0x7f80_0001, 4]);
        // Without data: only the colour changes.
        e.call(0x00a3_bc60, &args![this, 5u32, 6u32, 7u32, 8u32]);
        assert_eq!(calls(&log, DELETE_THIS).len(), 1);
        assert_eq!(colors(&e, this + COLOR_VALUE), [5, 6, 7, 8]);
    }

    #[test]
    fn guarantee_time_range_forwards_to_the_data() {
        let log = Log::default();
        let mut e = engine(&log);
        logged(&mut e, &log, COLOR_DATA_GUARANTEE_KEYS, 0);
        let this = interpolator(&mut e);
        e.call(0x00a3_bcb0, &args![this, 1.0f32, 2.0f32]);
        assert!(calls(&log, COLOR_DATA_GUARANTEE_KEYS).is_empty());
        e.mem.set_u32(this + COLOR_DATA, 0x6000);
        e.call(0x00a3_bcb0, &args![this, 0x7f80_0001u32, 2.0f32]);
        assert_eq!(
            calls(&log, COLOR_DATA_GUARANTEE_KEYS),
            vec![vec![0x6000, 0x7fc0_0001, 2.0f32.to_bits()]]
        );
    }

    #[test]
    fn save_binary_saves_the_colour_and_links_the_data() {
        let log = Log::default();
        let mut e = engine(&log);
        logged(&mut e, &log, BASE_SAVE_BINARY, 0);
        logged(&mut e, &log, COLOR_SAVE_BINARY, 0);
        logged(&mut e, &log, SAVE_LINK_ID, 0);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + SLOT_SAVE_LINK_ID, SAVE_LINK_ID);
        let stream = e.mem.alloc(0x260);
        e.mem.set_u32(stream, vtable);
        let this = interpolator(&mut e);
        e.mem.set_u32(this + COLOR_DATA, 0x6000);
        e.call(0x00a3_bce0, &args![this, stream]);
        assert_eq!(
            *log.borrow(),
            [
                (BASE_SAVE_BINARY, vec![this, stream]),
                (COLOR_SAVE_BINARY, vec![this + COLOR_VALUE, stream]),
                (SAVE_LINK_ID, vec![stream, 0x6000]),
            ]
        );
    }

    #[test]
    fn colour_constructor_stores_the_colour_words() {
        let log = Log::default();
        let mut e = engine(&log);
        logged(&mut e, &log, NI_KEY_BASED_INTERPOLATOR_CONSTRUCT, 0);
        let this = interpolator(&mut e);
        e.mem.write(this, &[0xaa; 0x24]);
        let back = e
            .call(0x00a3_bd80, &args![this, 1u32, 2u32, 0x7f80_0001u32, 4u32])
            .u32();
        assert_eq!(back, this);
        assert_eq!(
            calls(&log, NI_KEY_BASED_INTERPOLATOR_CONSTRUCT),
            vec![vec![this]]
        );
        assert_eq!(e.mem.u32(this), NI_COLOR_INTERPOLATOR_VTABLE);
        assert_eq!(colors(&e, this + COLOR_VALUE), [1, 2, 0x7f80_0001, 4]);
        assert_eq!(e.mem.u32(this + COLOR_DATA), 0);
        assert_eq!(e.mem.u32(this + LAST_INDEX), 0);
    }

    /// An interpolator whose data holds `colours.len()` keys of stride 0x14
    /// (the colour at +4 of each) of type `key_type`.
    fn keyed(e: &mut Engine, key_type: u32, colours: &[[u32; 4]]) -> (u32, u32) {
        let this = interpolator(e);
        let data = counted(e, 1);
        let keys = e.mem.alloc(0x14 * colours.len() as u32 + 4);
        for (i, c) in colours.iter().enumerate() {
            set_colors(e, keys + 0x14 * i as u32 + 4, *c);
        }
        e.mem.set_u32(data + DATA_NUM_KEYS, colours.len() as u32);
        e.mem.set_u32(data + DATA_KEYS, keys);
        e.mem.set_u32(data + DATA_KEY_TYPE, key_type);
        e.mem.set_u8(data + DATA_KEY_SIZE, 0x14);
        e.mem.set_u32(this + COLOR_DATA, data);
        set_colors(e, this + COLOR_VALUE, [9; 4]);
        (this, data)
    }

    #[test]
    fn collapse_keeps_a_constant_colour_and_drops_the_data() {
        let log = Log::default();
        let mut e = engine(&log);
        e.register(COLOR_DIFFERS, |e, a| {
            ret((colors(e, a[0]) != colors(e, a[1])) as u32)
        });
        let red = [0x3f80_0000, 0, 0, 0x3f80_0000];
        let blue = [0, 0, 0x3f80_0000, 0x3f80_0000];
        // Linear keys, all the same colour: collapsed.
        let (this, data) = keyed(&mut e, 1, &[red, red, red]);
        e.call(0x00a3_bdc0, &args![this]);
        assert_eq!(e.mem.u32(this + COLOR_DATA), 0);
        assert_eq!(calls(&log, DELETE_THIS), vec![vec![data]]);
        assert_eq!(colors(&e, this + COLOR_VALUE), red);
        // Step keys with a different colour: unchanged.
        let (this, data) = keyed(&mut e, 5, &[red, blue]);
        e.call(0x00a3_bdc0, &args![this]);
        assert_eq!(e.mem.u32(this + COLOR_DATA), data);
        assert_eq!(colors(&e, this + COLOR_VALUE), [9; 4]);
        // Another key type with equal colours: unchanged.
        let (this, data) = keyed(&mut e, 2, &[red, red]);
        e.call(0x00a3_bdc0, &args![this]);
        assert_eq!(e.mem.u32(this + COLOR_DATA), data);
        // One key of any type: collapsed to it.
        let (this, _) = keyed(&mut e, 3, &[blue]);
        e.call(0x00a3_bdc0, &args![this]);
        assert_eq!(e.mem.u32(this + COLOR_DATA), 0);
        assert_eq!(colors(&e, this + COLOR_VALUE), blue);
        // No keys: the invalid colour.
        let (this, _) = keyed(&mut e, 1, &[]);
        e.call(0x00a3_bdc0, &args![this]);
        assert_eq!(e.mem.u32(this + COLOR_DATA), 0);
        assert_eq!(colors(&e, this + COLOR_VALUE), [0xff7f_ffff; 4]);
        // No data: nothing.
        e.call(0x00a3_bdc0, &args![this]);
        assert_eq!(calls(&log, DELETE_THIS).len(), 3);
    }

    #[test]
    fn get_sequence_interpolator_gives_the_copy_the_sequence_data() {
        let log = Log::default();
        let mut e = engine(&log);
        let copy = interpolator(&mut e);
        let old = counted(&mut e, 1);
        let sequence = counted(&mut e, 1);
        e.mem.set_u32(copy + COLOR_DATA, old);
        e.mem.set_u32(copy + LAST_INDEX, 4);
        let l = log.clone();
        e.register_double(INTERPOLATOR_GET_SEQUENCE_INTERPOLATOR, move |_, a| {
            l.borrow_mut()
                .push((INTERPOLATOR_GET_SEQUENCE_INTERPOLATOR, a.to_vec()));
            ret(copy)
        });
        let l = log.clone();
        e.register_double(COLOR_DATA_GET_SEQUENCE_DATA, move |e, a| {
            l.borrow_mut().push((
                COLOR_DATA_GET_SEQUENCE_DATA,
                vec![a[0], a[1], a[2], e.mem.u32(a[3])],
            ));
            e.mem.set_u32(a[3], sequence);
            ret(0)
        });
        let this = interpolator(&mut e);
        e.mem.set_u32(this + COLOR_DATA, 0x6000);
        let answer = e
            .call(0x00a3_bef0, &args![this, 0x7f80_0001u32, 3.0f32])
            .u32();
        assert_eq!(answer, copy);
        assert_eq!(
            calls(&log, INTERPOLATOR_GET_SEQUENCE_INTERPOLATOR),
            vec![vec![this, 0x7fc0_0001, 3.0f32.to_bits()]]
        );
        assert_eq!(
            calls(&log, COLOR_DATA_GET_SEQUENCE_DATA),
            vec![vec![0x6000, 0x7fc0_0001, 3.0f32.to_bits(), 0]]
        );
        assert_eq!(e.mem.u32(copy + COLOR_DATA), sequence);
        assert_eq!(e.mem.u32(copy + LAST_INDEX), 0);
        // Counted by the copy, then released by the local pointer.
        assert_eq!(e.mem.u32(sequence + 4), 1);
        assert_eq!(calls(&log, DELETE_THIS), vec![vec![old]]);
        // Without colour data only the base copy is made.
        e.mem.set_u32(this + COLOR_DATA, 0);
        assert_eq!(
            e.call(0x00a3_bef0, &args![this, 1.0f32, 3.0f32]).u32(),
            copy
        );
        assert_eq!(calls(&log, COLOR_DATA_GET_SEQUENCE_DATA).len(), 1);
    }

    #[test]
    fn copy_members_copies_the_colour_and_counts_the_data() {
        let log = Log::default();
        let mut e = engine(&log);
        logged(&mut e, &log, KEY_BASED_COPY_MEMBERS, 0);
        let this = interpolator(&mut e);
        let destination = interpolator(&mut e);
        let data = counted(&mut e, 1);
        let old = counted(&mut e, 1);
        set_colors(&mut e, this + COLOR_VALUE, [1, 2, 0x7f80_0001, 4]);
        e.mem.set_u32(this + COLOR_DATA, data);
        e.mem.set_u32(this + LAST_INDEX, 5);
        e.mem.set_u32(destination + COLOR_DATA, old);
        e.call(0x00a3_c0a0, &args![this, destination, 0x9000u32]);
        assert_eq!(
            calls(&log, KEY_BASED_COPY_MEMBERS),
            vec![vec![this, destination, 0x9000]]
        );
        assert_eq!(
            colors(&e, destination + COLOR_VALUE),
            [1, 2, 0x7f80_0001, 4]
        );
        assert_eq!(e.mem.u32(destination + COLOR_DATA), data);
        assert_eq!(e.mem.u32(data + 4), 2);
        assert_eq!(calls(&log, DELETE_THIS), vec![vec![old]]);
        assert_eq!(e.mem.u32(destination + LAST_INDEX), 5);
        // The same data again: no counting.
        e.call(0x00a3_c0a0, &args![this, destination, 0x9000u32]);
        assert_eq!(e.mem.u32(data + 4), 2);
    }

    #[test]
    fn create_object_builds_a_default_interpolator_or_answers_null() {
        let log = Log::default();
        let mut e = engine(&log);
        logged(&mut e, &log, NI_KEY_BASED_INTERPOLATOR_CONSTRUCT, 0);
        let block = interpolator(&mut e);
        e.mem.write(block, &[0xaa; 0x24]);
        logged(&mut e, &log, NI_ALLOCATE, block);
        assert_eq!(e.call(0x00a3_c130, &args![]).u32(), block);
        assert_eq!(calls(&log, NI_ALLOCATE), vec![vec![0x24]]);
        assert_eq!(e.mem.u32(block), NI_COLOR_INTERPOLATOR_VTABLE);
        assert_eq!(colors(&e, block + COLOR_VALUE), [0xff7f_ffff; 4]);
        assert_eq!(e.mem.u32(block + COLOR_DATA), 0);
        assert_eq!(e.mem.u32(block + LAST_INDEX), 0);
        logged(&mut e, &log, NI_ALLOCATE, 0);
        assert_eq!(e.call(0x00a3_c130, &args![]).u32(), 0);
        assert_eq!(calls(&log, NI_KEY_BASED_INTERPOLATOR_CONSTRUCT).len(), 1);
    }

    #[test]
    fn is_equal_compares_base_colour_and_data() {
        let log = Log::default();
        let mut e = engine(&log);
        e.register(COLOR_DIFFERS, |e, a| {
            ret((colors(e, a[0]) != colors(e, a[1])) as u32)
        });
        e.register(BASE_IS_EQUAL, |e, a| ret(e.mem.u32(a[0] + 8)));
        let mut answers = vec![1u32, 0];
        let l = log.clone();
        e.register_double(DATA_IS_EQUAL, move |_, a| {
            l.borrow_mut().push((DATA_IS_EQUAL, a.to_vec()));
            ret(answers.remove(0))
        });
        let this = interpolator(&mut e);
        let other = interpolator(&mut e);
        e.mem.set_u32(this + 8, 1);
        // Both without data, same colour.
        assert!(e.call(0x00a3_c190, &args![this, other]).bool());
        // The base says no.
        e.mem.set_u32(this + 8, 0);
        assert!(!e.call(0x00a3_c190, &args![this, other]).bool());
        e.mem.set_u32(this + 8, 1);
        // Different colours.
        e.mem.set_u32(other + COLOR_VALUE, 1);
        assert!(!e.call(0x00a3_c190, &args![this, other]).bool());
        e.mem.set_u32(other + COLOR_VALUE, 0);
        // Only one has data.
        let data = counted(&mut e, 1);
        let other_data = counted(&mut e, 1);
        e.mem.set_u32(this + COLOR_DATA, data);
        assert!(!e.call(0x00a3_c190, &args![this, other]).bool());
        e.mem.set_u32(this + COLOR_DATA, 0);
        e.mem.set_u32(other + COLOR_DATA, other_data);
        assert!(!e.call(0x00a3_c190, &args![this, other]).bool());
        // Both: the data's IsEqual decides (true, then false).
        e.mem.set_u32(this + COLOR_DATA, data);
        assert!(e.call(0x00a3_c190, &args![this, other]).bool());
        assert!(!e.call(0x00a3_c190, &args![this, other]).bool());
        assert_eq!(calls(&log, DATA_IS_EQUAL), vec![vec![data, other_data]; 2]);
    }

    #[test]
    fn create_clone_constructs_and_copies_or_copies_into_null() {
        let log = Log::default();
        let mut e = engine(&log);
        logged(&mut e, &log, NI_KEY_BASED_INTERPOLATOR_CONSTRUCT, 0);
        logged(&mut e, &log, KEY_BASED_COPY_MEMBERS, 0);
        let block = interpolator(&mut e);
        logged(&mut e, &log, NI_ALLOCATE, block);
        let this = interpolator(&mut e);
        set_colors(&mut e, this + COLOR_VALUE, [1, 2, 3, 4]);
        e.mem.set_u32(this + LAST_INDEX, 2);
        assert_eq!(e.call(0x00a3_c1f0, &args![this, 0x9000u32]).u32(), block);
        assert_eq!(e.mem.u32(block), NI_COLOR_INTERPOLATOR_VTABLE);
        assert_eq!(colors(&e, block + COLOR_VALUE), [1, 2, 3, 4]);
        assert_eq!(e.mem.u32(block + LAST_INDEX), 2);
        assert_eq!(
            calls(&log, KEY_BASED_COPY_MEMBERS),
            vec![vec![this, block, 0x9000]]
        );
        // A failed allocation copies into address 0.
        e.map(0, 0x100);
        logged(&mut e, &log, NI_ALLOCATE, 0);
        assert_eq!(e.call(0x00a3_c1f0, &args![this, 0x9000u32]).u32(), 0);
        assert_eq!(
            calls(&log, KEY_BASED_COPY_MEMBERS)[1],
            vec![this, 0, 0x9000]
        );
        assert_eq!(colors(&e, COLOR_VALUE), [1, 2, 3, 4]);
        assert_eq!(calls(&log, NI_KEY_BASED_INTERPOLATOR_CONSTRUCT).len(), 1);
    }

    #[test]
    fn get_viewer_strings_adds_name_colour_and_data() {
        let log = Log::default();
        let mut e = engine(&log);
        logged(&mut e, &log, KEY_BASED_GET_VIEWER_STRINGS, 0);
        logged(&mut e, &log, NI_GET_VIEWER_STRING, 0x7100);
        logged(&mut e, &log, COLOR_GET_VIEWER_STRING, 0x7200);
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
        e.map(CLASS_NAME, 4);
        e.mem.set_u32(CLASS_NAME, 0x5151);
        let this = interpolator(&mut e);
        e.mem.set_u32(this + COLOR_DATA, 0x6000);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 2);
        e.mem.set_u16(strings + 0xa, 1);
        e.mem.set_u16(strings + 0xe, 4);
        e.call(0x00a3_c270, &args![this, strings]);
        assert_eq!(
            *log.borrow(),
            [
                (KEY_BASED_GET_VIEWER_STRINGS, vec![this, strings]),
                (NI_GET_VIEWER_STRING, vec![0x5151]),
                (STRINGS_SET_AT, vec![strings, 1, 0x7100]),
                (
                    COLOR_GET_VIEWER_STRING,
                    vec![this + COLOR_VALUE, LABEL_COLOR_VALUE]
                ),
                (STRINGS_GROW, vec![strings, 6]),
                (STRINGS_SET_AT, vec![strings, 2, 0x7200]),
                (NI_GET_VIEWER_STRING_POINTER, vec![LABEL_COLOR_DATA, 0x6000]),
                (STRINGS_GROW, vec![strings, 7]),
                (STRINGS_SET_AT, vec![strings, 3, 0x7300]),
            ]
        );
    }

    #[test]
    fn register_streamables_registers_the_data_after_the_base() {
        let log = Log::default();
        let mut e = engine(&log);
        let mut answers = vec![0u32, 1, 1];
        e.register_double(
            BASE_REGISTER_STREAMABLES,
            move |_, _| ret(answers.remove(0)),
        );
        let this = interpolator(&mut e);
        let data = counted(&mut e, 1);
        e.mem.set_u32(this + COLOR_DATA, data);
        assert!(!e.call(0x00a4_89f0, &args![this, 0x8000u32]).bool());
        assert!(calls(&log, DATA_REGISTER).is_empty());
        assert!(e.call(0x00a4_89f0, &args![this, 0x8000u32]).bool());
        assert_eq!(calls(&log, DATA_REGISTER), vec![vec![data, 0x8000]]);
        e.mem.set_u32(this + COLOR_DATA, 0);
        assert!(e.call(0x00a4_89f0, &args![this, 0x8000u32]).bool());
        assert_eq!(calls(&log, DATA_REGISTER).len(), 1);
    }

    #[test]
    fn scalar_deleting_destructor_releases_the_data_and_deletes_on_bit_0() {
        let log = Log::default();
        let mut e = engine(&log);
        logged(&mut e, &log, KEY_BASED_DESTRUCT, 0);
        logged(&mut e, &log, SIZED_DELETE, 0);
        let this = interpolator(&mut e);
        let data = counted(&mut e, 1);
        e.mem.set_u32(this + COLOR_DATA, data);
        assert_eq!(e.call(0x00a4_8dd0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls(&log, DELETE_THIS), vec![vec![data]]);
        assert_eq!(e.mem.u32(this + COLOR_DATA), data);
        assert_eq!(calls(&log, KEY_BASED_DESTRUCT), vec![vec![this]]);
        assert_eq!(calls(&log, SIZED_DELETE), vec![vec![this, 0x24]]);
        e.mem.set_u32(this + COLOR_DATA, 0);
        assert_eq!(e.call(0x00a4_8dd0, &args![this, 2u32]).u32(), this);
        assert_eq!(calls(&log, SIZED_DELETE).len(), 1);
        assert_eq!(calls(&log, KEY_BASED_DESTRUCT).len(), 2);
    }
}
