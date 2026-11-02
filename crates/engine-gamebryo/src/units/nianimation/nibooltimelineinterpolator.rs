//! `NiAnimation/nibooltimelineinterpolator.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiBoolTimelineInterpolator::ms_RTTI` (the class's `NiRTTI` record).
const NI_BOOL_TIMELINE_INTERPOLATOR_RTTI: u32 = 0x011f_4044;

// Translated from 00a52d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolTimelineInterpolator::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_bool_timeline_interpolator_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_BOOL_TIMELINE_INTERPOLATOR_RTTI)
}

/// `00a43330`: the jump target of `00a514d0`.
const JUMP_TARGET_00A514D0: u32 = 0x00a4_3330;

/// `00a53600`: the jump target of `00a52ce0`.
const JUMP_TARGET_00A52CE0: u32 = 0x00a5_3600;

/// `00a3c5b0`: the jump target of `00a52cf0`.
const JUMP_TARGET_00A52CF0: u32 = 0x00a3_c5b0;

/// `00a53680`: the jump target of `00a52d00`.
const JUMP_TARGET_00A52D00: u32 = 0x00a5_3680;

/// `00a53220`: the jump target of `00a52d10`.
const JUMP_TARGET_00A52D10: u32 = 0x00a5_3220;

// Translated from 00a514d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolTimelineInterpolator::LinkObject` (Xbox PDB): a 5-byte jump to `00a43330` (engine map: `BSBlendTreadTransfInterpolator::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@NiBoolTimelineInterpolator@@UAAXAAVNiStream@@@Z`.
pub fn ni_bool_timeline_interpolator_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A514D0, &args![this, stream]);
}

// Translated from 00a52ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolTimelineInterpolator::LoadBinary` (Xbox PDB): a 5-byte jump to `00a53600` (engine map: `NiBoolInterpolator::LoadBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LoadBinary@NiBoolTimelineInterpolator@@UAAXAAVNiStream@@@Z`.
pub fn ni_bool_timeline_interpolator_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A52CE0, &args![this, stream]);
}

// Translated from 00a52cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolTimelineInterpolator::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00a3c5b0` (engine map: `NiBoolInterpolator::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@NiBoolTimelineInterpolator@@UAA_NAAVNiStream@@@Z`.
pub fn ni_bool_timeline_interpolator_register_streamables(
    e: &mut Engine,
    this: Ptr,
    stream: Ptr,
) -> bool {
    e.call(JUMP_TARGET_00A52CF0, &args![this, stream]).bool()
}

// Translated from 00a52d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolTimelineInterpolator::SaveBinary` (Xbox PDB): a 5-byte jump to `00a53680` (engine map: `NiBoolInterpolator::SaveBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?SaveBinary@NiBoolTimelineInterpolator@@UAAXAAVNiStream@@@Z`.
pub fn ni_bool_timeline_interpolator_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A52D00, &args![this, stream]);
}

// Translated from 00a52d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolTimelineInterpolator::IsEqual` (Xbox PDB): a 5-byte jump to `00a53220` (engine map: `NiBoolInterpolator::IsEqual`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?IsEqual@NiBoolTimelineInterpolator@@UAA_NPAVNiObject@@@Z`.
pub fn ni_bool_timeline_interpolator_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.call(JUMP_TARGET_00A52D10, &args![this, other]).bool()
}

/// The vtable of `NiBoolTimelineInterpolator` (RTTI
/// `.?AVNiBoolTimelineInterpolator@@`).
const NI_BOOL_TIMELINE_INTERPOLATOR_VTABLE: u32 = 0x0109_9fb4;
/// `NiBoolTimelineInterpolator` size (the `CreateObject` allocation and the
/// sized delete).
const NI_BOOL_TIMELINE_INTERPOLATOR_SIZE: u32 = 0x20;

/// Fields as the PC code uses them: the last update time (`NiInterpolator`)
/// +0x08, the current value byte (`NiBoolInterpolator::m_bBool`, Xbox PDB)
/// +0x0c, the `NiBoolData` pointer +0x10, the last key index +0x14, then
/// this class's previous key index +0x18 and previous key value byte +0x1c.
const LAST_TIME: u32 = 0x08;
const VALUE: u32 = 0x0c;
const BOOL_DATA: u32 = 0x10;
const LAST_INDEX: u32 = 0x14;
const PREVIOUS_INDEX: u32 = 0x18;
const PREVIOUS_VALUE: u32 = 0x1c;

/// The "no value" bool byte (read from `.rdata`; the code compares the value
/// byte with it).
const INVALID_BOOL: u32 = 0x0109_a914;

/// `NiBoolKey::GenInterp` (`00a29250`, `cdecl`, six words: time, keys, key
/// type, key count, `&last index`, key size); returns the value in `AL`.
const BOOL_KEY_GEN_INTERP: u32 = 0x00a2_9250;
/// `00a53280` (unit `niboolinterpolator.cpp`, `thiscall`, one word): the
/// `NiBoolInterpolator` constructor taking a bool data pointer.
const BOOL_INTERPOLATOR_CONSTRUCT: u32 = 0x00a5_3280;
/// `00a53480` (unit `niboolinterpolator.cpp`, `thiscall`, `(destination,
/// cloning)`): the `NiBoolInterpolator` member copy.
const BOOL_INTERPOLATOR_COPY_MEMBERS: u32 = 0x00a5_3480;
/// `00a3ca30` (unit `nikeybasedinterpolator.cpp`): the destructor body the
/// deleting destructor runs after releasing the bool data.
const KEY_BASED_DESTRUCT: u32 = 0x00a3_ca30;
/// `00aa13e0` (`cdecl`, one word): allocates from the Ni allocator.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `00aa1460` (`cdecl`, `(block, size)`): the sized delete.
const NI_SIZED_DELETE: u32 = 0x00aa_1460;
/// `InterlockedDecrement` (import slot) and `NiRefObject::DeleteThis`
/// (virtual slot 1).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
const SLOT_DELETE_THIS: u32 = 0x04;

// Translated from 00a52d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nibooltimelineinterpolator.cpp`): the
/// bool data's keys. With bool data, its key count, key type and key size
/// byte go to the three out parameters and its key array is returned;
/// without, the three are cleared and 0 is returned.
pub fn fn_00a52d20(
    e: &mut Engine,
    this: Ptr,
    num_keys_out: Ptr,
    key_type_out: Ptr,
    key_size_out: Ptr,
) -> u32 {
    let data = e.mem.u32(this.addr().wrapping_add(BOOL_DATA));
    if data != 0 {
        let num_keys = e.mem.u32(data.wrapping_add(8));
        e.mem.set_u32(num_keys_out.addr(), num_keys);
        let key_type = e.mem.u32(data.wrapping_add(0x10));
        e.mem.set_u32(key_type_out.addr(), key_type);
        let key_size = e.mem.u8(data.wrapping_add(0x14));
        e.mem.set_u8(key_size_out.addr(), key_size);
        return e.mem.u32(data.wrapping_add(0xc));
    }
    e.mem.set_u32(num_keys_out.addr(), 0);
    e.mem.set_u32(key_type_out.addr(), 0);
    e.mem.set_u8(key_size_out.addr(), 0);
    0
}

// Translated from 00a52d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolTimelineInterpolator::Update` (Xbox PDB). At the time of the last
/// update the stored value byte decides: the "no value" byte (`0109a914`)
/// clears `out` and returns false, any other value stores whether it is
/// non-zero and returns true. At another time, with bool data that has
/// keys, the last key index is kept as the previous one and
/// `NiBoolKey::GenInterp` gives the new value. When it equals the previous
/// value, the keys passed over since the previous index are searched for
/// the opposite value (a toggle the frame stepped over): from the previous
/// index + 1 up to the new index when the index moved forward (or stayed),
/// otherwise up to the last key and then, from 0, up to the new index; a
/// key found makes the stored value the opposite one. The value byte
/// becomes `found value != 0`, the previous value the one `GenInterp` gave.
/// Then the value byte decides as above, and a valid one keeps the time.
/// The second argument word is not read.
///
/// The key size is pushed as a stack word whose low byte is the size; the
/// three bytes above it are a stack slot this function never writes, so
/// they are passed as zero here.
pub fn ni_bool_timeline_interpolator_update(
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
        let value = e.mem.u8(this.wrapping_add(VALUE));
        if value == e.mem.u8(INVALID_BOOL) {
            e.mem.set_u8(out.addr(), 0);
            return false;
        }
        e.mem.set_u8(out.addr(), u8::from(value != 0));
        return true;
    }
    let data = e.mem.u32(this.wrapping_add(BOOL_DATA));
    if data != 0 {
        let key_size = e.mem.u8(data.wrapping_add(0x14));
        let key_type = e.mem.u32(data.wrapping_add(0x10));
        let keys = e.mem.u32(data.wrapping_add(0xc));
        let num_keys = e.mem.u32(data.wrapping_add(8));
        if num_keys != 0 {
            let last_index = e.mem.u32(this.wrapping_add(LAST_INDEX));
            e.mem.set_u32(this.wrapping_add(PREVIOUS_INDEX), last_index);
            let value = e
                .call(
                    BOOL_KEY_GEN_INTERP,
                    &args![
                        time_bits,
                        keys,
                        key_type,
                        num_keys,
                        this.wrapping_add(LAST_INDEX),
                        u32::from(key_size)
                    ],
                )
                .u8();
            let index = e.mem.u32(this.wrapping_add(LAST_INDEX));
            let mut result = value;
            if value == e.mem.u8(this.wrapping_add(PREVIOUS_VALUE)) {
                let previous = e.mem.u32(this.wrapping_add(PREVIOUS_INDEX));
                let target = u8::from(value == 0);
                let key_value = |e: &Engine, i: u32| {
                    e.mem.u8(u32::from(key_size)
                        .wrapping_mul(i)
                        .wrapping_add(keys)
                        .wrapping_add(4))
                };
                let next = previous.wrapping_add(1);
                if previous == index || next <= index {
                    let mut i = next;
                    while i < index && i < num_keys {
                        if key_value(e, i) == target {
                            result = target;
                            break;
                        }
                        i = i.wrapping_add(1);
                    }
                } else {
                    let mut found = false;
                    let mut i = next;
                    while i < num_keys {
                        if key_value(e, i) == target {
                            result = target;
                            found = true;
                            break;
                        }
                        i = i.wrapping_add(1);
                    }
                    if !found && value != target {
                        let mut i = 0u32;
                        while i < index {
                            if key_value(e, i) == target {
                                result = target;
                                break;
                            }
                            i = i.wrapping_add(1);
                        }
                    }
                }
            }
            e.mem
                .set_u8(this.wrapping_add(VALUE), u8::from(result != 0));
            e.mem.set_u8(this.wrapping_add(PREVIOUS_VALUE), value);
        }
    }
    let value = e.mem.u8(this.wrapping_add(VALUE));
    if value == e.mem.u8(INVALID_BOOL) {
        e.mem.set_u8(out.addr(), 0);
        return false;
    }
    e.mem.set_u8(out.addr(), u8::from(value != 0));
    e.mem.set_u32(this.wrapping_add(LAST_TIME), time_bits);
    true
}

// Translated from 00a52f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolTimelineInterpolator::_scalar_deleting_destructor_` (Xbox PDB):
/// releases the bool data (`InterlockedDecrement`, `DeleteThis` at zero),
/// runs the destructor body `00a3ca30`, then the sized delete of 0x20 bytes
/// when bit 0 of `flags` is set. Returns `this`.
pub fn ni_bool_timeline_interpolator_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    let data = e.mem.u32(this.addr().wrapping_add(BOOL_DATA));
    if data != 0
        && e.call(INTERLOCKED_DECREMENT, &args![data.wrapping_add(4)])
            .u32()
            == 0
    {
        e.vcall(data, SLOT_DELETE_THIS, &args![]);
    }
    e.call(KEY_BASED_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(
            NI_SIZED_DELETE,
            &args![this, NI_BOOL_TIMELINE_INTERPOLATOR_SIZE],
        );
    }
    this
}

/// The constructor body `CreateClone` and `CreateObject` share: the
/// `NiBoolInterpolator` constructor with no data, the vtable, previous
/// index 0 and previous value 0.
fn construct_in(e: &mut Engine, object: u32) {
    e.call(BOOL_INTERPOLATOR_CONSTRUCT, &args![object, 0u32]);
    e.mem.set_u32(object, NI_BOOL_TIMELINE_INTERPOLATOR_VTABLE);
    e.mem.set_u32(object.wrapping_add(PREVIOUS_INDEX), 0);
    e.mem.set_u8(object.wrapping_add(PREVIOUS_VALUE), 0);
}

// Translated from 00a52f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolTimelineInterpolator::CreateClone` (Xbox PDB): allocates 0x20
/// bytes and, when that succeeds, constructs it (see `CreateObject`) and
/// runs the `NiBoolInterpolator` member copy (`00a53480`) into it; returns
/// it. When the allocation fails the member copy still runs, into address
/// 0, and 0 is returned.
pub fn ni_bool_timeline_interpolator_create_clone(e: &mut Engine, this: Ptr, cloning: u32) -> Ptr {
    let object = e
        .call(NI_ALLOCATE, &args![NI_BOOL_TIMELINE_INTERPOLATOR_SIZE])
        .u32();
    if object == 0 {
        e.call(BOOL_INTERPOLATOR_COPY_MEMBERS, &args![this, 0u32, cloning]);
        return Ptr::new(0);
    }
    construct_in(e, object);
    e.call(
        BOOL_INTERPOLATOR_COPY_MEMBERS,
        &args![this, object, cloning],
    );
    Ptr::new(object)
}

// Translated from 00a52fe0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `NiBoolTimelineInterpolator::CreateObject` (registered through
/// `NiStream::RegisterLoader` for `"NiBoolTimelineInterpolator"` at
/// `00a2d95c`; Ghidra has no function here): allocates 0x20 bytes and, when that
/// succeeds, runs the `NiBoolInterpolator` constructor with no data
/// (`00a53280`), stores the vtable and clears the previous index and value.
/// Returns the object, or 0.
pub fn ni_bool_timeline_interpolator_create_object(e: &mut Engine) -> Ptr {
    let object = e
        .call(NI_ALLOCATE, &args![NI_BOOL_TIMELINE_INTERPOLATOR_SIZE])
        .u32();
    if object == 0 {
        return Ptr::new(0);
    }
    construct_in(e, object);
    Ptr::new(object)
}

/// `NiBoolInterpolator::GetViewerStrings` (`00a536d0`, engine map;
/// `thiscall`, one word: the strings array).
const BOOL_INTERPOLATOR_GET_VIEWER_STRINGS: u32 = 0x00a5_36d0;
/// `NiGetViewerString` (`00a7d330`, `cdecl (text)`); the viewer-string
/// array's grow (`0096ad30`, `thiscall (array; new capacity)`) and store
/// (`0096ae90`, `thiscall (array; index, &string)`).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;

// Translated from 00a52f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiBoolTimelineInterpolator(NiBoolData*)` constructor (`thiscall`,
/// one word: the data; the engine map gives it no name): the
/// `NiBoolInterpolator` constructor on the data (`00a53280`), previous
/// index 0, previous value 0, then the vtable. Returns `this`.
pub fn fn_00a52f00(e: &mut Engine, this: Ptr, data: u32) -> Ptr {
    e.call(BOOL_INTERPOLATOR_CONSTRUCT, &args![this, data]);
    let object = this.addr();
    e.mem.set_u32(object.wrapping_add(PREVIOUS_INDEX), 0);
    e.mem.set_u8(object.wrapping_add(PREVIOUS_VALUE), 0);
    e.mem.set_u32(object, NI_BOOL_TIMELINE_INTERPOLATOR_VTABLE);
    this
}

// Translated from 00a53020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolTimelineInterpolator::GetViewerStrings` (Xbox PDB, matched by
/// the engine map; `thiscall`, one word: the strings array):
/// `NiBoolInterpolator::GetViewerStrings` (`00a536d0`), then the class
/// name (the first word of the `NiRTTI` record) appended: when the 16-bit
/// size (+0xA) has reached the capacity (+0x8) the array grows to the step
/// (+0xE) plus the size first, then the string is stored at the old size.
pub fn ni_bool_timeline_interpolator_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(BOOL_INTERPOLATOR_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name: u32 = e.global(NI_BOOL_TIMELINE_INTERPOLATOR_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
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

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a5_2f00, fn_00a52f00(Ptr, u32) -> Ptr),
        entry!(
            0x00a5_3020,
            ni_bool_timeline_interpolator_get_viewer_strings(Ptr, Ptr)
        ),
        entry!(0x00a52d70, ni_bool_timeline_interpolator_get_rtti() -> Ptr),
        entry!(
            0x00a5_14d0,
            ni_bool_timeline_interpolator_link_object(Ptr, Ptr)
        ),
        entry!(
            0x00a5_2ce0,
            ni_bool_timeline_interpolator_load_binary(Ptr, Ptr)
        ),
        entry!(0x00a5_2cf0, ni_bool_timeline_interpolator_register_streamables(Ptr, Ptr) -> bool),
        entry!(
            0x00a5_2d00,
            ni_bool_timeline_interpolator_save_binary(Ptr, Ptr)
        ),
        entry!(0x00a5_2d10, ni_bool_timeline_interpolator_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00a5_2d20, fn_00a52d20(Ptr, Ptr, Ptr, Ptr) -> u32),
        entry!(0x00a5_2d80, ni_bool_timeline_interpolator_update(Ptr, f32, u32, Ptr) -> bool),
        entry!(0x00a5_2f30, ni_bool_timeline_interpolator_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00a5_2f80, ni_bool_timeline_interpolator_create_clone(Ptr, u32) -> Ptr),
        entry!(0x00a5_2fe0, ni_bool_timeline_interpolator_create_object() -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_bool_timeline_interpolator_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a52d70, &args![]).u32(),
            NI_BOOL_TIMELINE_INTERPOLATOR_RTTI
        );
    }

    #[test]
    fn thunk_00a514d0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a5_14d0;
        const TARGET: u32 = 0x00a4_3330;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }

    #[test]
    fn thunk_00a52ce0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a5_2ce0;
        const TARGET: u32 = 0x00a5_3600;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }

    #[test]
    fn thunk_00a52cf0_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a5_2cf0;
        const TARGET: u32 = 0x00a3_c5b0;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
        e.register(TARGET, |_, _| Ret::default());
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 0);
    }

    #[test]
    fn thunk_00a52d00_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a5_2d00;
        const TARGET: u32 = 0x00a5_3680;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }

    #[test]
    fn thunk_00a52d10_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a5_2d10;
        const TARGET: u32 = 0x00a5_3220;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
        e.register(TARGET, |_, _| Ret::default());
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 0);
    }

    type Log = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

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

    #[test]
    fn fn_00a52d20_hands_out_the_data_keys_or_clears_the_outputs() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        let data = e.mem.alloc(0x18);
        let outs = e.mem.alloc(12);
        e.mem.set_u32(data + 8, 6);
        e.mem.set_u32(data + 0xc, 0x6000);
        e.mem.set_u32(data + 0x10, 2);
        e.mem.set_u8(data + 0x14, 8);
        e.mem.set_u32(this + BOOL_DATA, data);
        let keys = e
            .call(0x00a5_2d20, &args![this, outs, outs + 4, outs + 8])
            .u32();
        assert_eq!(keys, 0x6000);
        assert_eq!(
            (e.mem.u32(outs), e.mem.u32(outs + 4), e.mem.u8(outs + 8)),
            (6, 2, 8)
        );
        e.mem.set_u32(this + BOOL_DATA, 0);
        let keys = e
            .call(0x00a5_2d20, &args![this, outs, outs + 4, outs + 8])
            .u32();
        assert_eq!(keys, 0);
        assert_eq!(e.mem.bytes(outs, 9), vec![0; 9]);
    }

    /// An interpolator over five bool keys (8 bytes each, the value at +4:
    /// 0 1 0 0 1) whose `GenInterp` double sets the last index to `index`
    /// and returns `value`.
    fn timeline(index: u32, value: u8) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.map(INVALID_BOOL, 4);
        e.mem.set_u8(INVALID_BOOL, 2);
        let keys = e.mem.alloc(0x28);
        for (i, v) in [0u8, 1, 0, 0, 1].iter().enumerate() {
            e.mem.set_u8(keys + 8 * i as u32 + 4, *v);
        }
        let data = e.mem.alloc(0x18);
        e.mem.set_u32(data + 8, 5);
        e.mem.set_u32(data + 0xc, keys);
        e.mem.set_u32(data + 0x10, 1);
        e.mem.set_u8(data + 0x14, 8);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + BOOL_DATA, data);
        e.register_double(BOOL_KEY_GEN_INTERP, move |e, a| {
            e.mem.set_u32(a[4], index);
            ret(u32::from(value))
        });
        let out = e.mem.alloc(4);
        (e, this, out)
    }

    #[test]
    fn update_at_the_last_time_reports_the_stored_value() {
        let (mut e, this, out) = timeline(0, 0);
        e.mem.set_f32(this + LAST_TIME, 2.0);
        e.mem.set_u8(this + VALUE, 5);
        assert!(e.call(0x00a5_2d80, &args![this, 2.0f32, 0u32, out]).bool());
        assert_eq!(e.mem.u8(out), 1);
        e.mem.set_u8(this + VALUE, 2);
        assert!(!e.call(0x00a5_2d80, &args![this, 2.0f32, 0u32, out]).bool());
        assert_eq!(e.mem.u8(out), 0);
    }

    #[test]
    fn update_takes_a_changed_value_and_keeps_the_time() {
        let (mut e, this, out) = timeline(3, 1);
        e.call_log = Some(vec![]);
        e.mem.set_u32(this + LAST_INDEX, 1);
        assert!(e.call(0x00a5_2d80, &args![this, 3.0f32, 9u32, out]).bool());
        let log = e.call_log.take().unwrap();
        let data = e.mem.u32(this + BOOL_DATA);
        let keys = e.mem.u32(data + 0xc);
        let call = log.iter().find(|c| c.0 == BOOL_KEY_GEN_INTERP).unwrap();
        assert_eq!(call.1, [3.0f32.to_bits(), keys, 1, 5, this + LAST_INDEX, 8]);
        assert_eq!(e.mem.u32(this + PREVIOUS_INDEX), 1);
        assert_eq!(e.mem.u8(this + VALUE), 1);
        assert_eq!(e.mem.u8(this + PREVIOUS_VALUE), 1);
        assert_eq!(e.mem.u8(out), 1);
        assert_eq!(e.mem.f32(this + LAST_TIME), 3.0);
    }

    #[test]
    fn update_finds_a_toggle_stepped_over_forward() {
        // Previous index 0, new index 3, value 0 as before: key 1 is a 1.
        let (mut e, this, out) = timeline(3, 0);
        assert!(e.call(0x00a5_2d80, &args![this, 3.0f32, 0u32, out]).bool());
        assert_eq!(e.mem.u8(this + VALUE), 1);
        assert_eq!(e.mem.u8(this + PREVIOUS_VALUE), 0);
        assert_eq!(e.mem.u8(out), 1);
        // From index 2 to 3: key 2 is not a 1, the value stays 0.
        let (mut e, this, out) = timeline(3, 0);
        e.mem.set_u32(this + LAST_INDEX, 2);
        assert!(e.call(0x00a5_2d80, &args![this, 3.0f32, 0u32, out]).bool());
        assert_eq!(e.mem.u8(this + VALUE), 0);
    }

    #[test]
    fn update_searches_around_the_end_when_the_index_went_back() {
        // Previous index 2, new index 1, value 0: keys 3 and 4 are searched
        // first (key 4 is a 1).
        let (mut e, this, out) = timeline(1, 0);
        e.mem.set_u32(this + LAST_INDEX, 2);
        assert!(e.call(0x00a5_2d80, &args![this, 3.0f32, 0u32, out]).bool());
        assert_eq!(e.mem.u8(this + VALUE), 1);
        // Previous index 3, new index 2, previous value 1, value 1: keys
        // 4.. hold no 0, then keys 0 and 1 from the start: key 0 is a 0.
        let (mut e, this, out) = timeline(2, 1);
        e.mem.set_u32(this + LAST_INDEX, 3);
        e.mem.set_u8(this + PREVIOUS_VALUE, 1);
        assert!(e.call(0x00a5_2d80, &args![this, 3.0f32, 0u32, out]).bool());
        assert_eq!(e.mem.u8(this + VALUE), 0);
        assert_eq!(e.mem.u8(out), 0);
    }

    #[test]
    fn update_without_data_reports_the_stored_value_or_fails() {
        let (mut e, this, out) = timeline(0, 0);
        e.mem.set_u32(this + BOOL_DATA, 0);
        e.mem.set_u8(this + VALUE, 2);
        assert!(!e.call(0x00a5_2d80, &args![this, 4.0f32, 0u32, out]).bool());
        assert_eq!(e.mem.u8(out), 0);
        assert_eq!(e.mem.f32(this + LAST_TIME), 0.0);
        e.mem.set_u8(this + VALUE, 1);
        assert!(e.call(0x00a5_2d80, &args![this, 4.0f32, 0u32, out]).bool());
        assert_eq!(e.mem.f32(this + LAST_TIME), 4.0);
    }

    #[test]
    fn scalar_deleting_destructor_releases_the_data_and_deletes_on_bit_0() {
        const DELETE_THIS: u32 = 0x00ee_0500;
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, KEY_BASED_DESTRUCT, 0);
        logged(&mut e, &log, NI_SIZED_DELETE, 0);
        logged(&mut e, &log, DELETE_THIS, 0);
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            ret(count)
        });
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, DELETE_THIS);
        let data = e.mem.alloc(0x18);
        e.mem.set_u32(data, vtable);
        e.mem.set_u32(data + 4, 1);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + BOOL_DATA, data);
        assert_eq!(e.call(0x00a5_2f30, &args![this, 1u32]).u32(), this);
        assert_eq!(
            *log.borrow(),
            [
                (DELETE_THIS, vec![data]),
                (KEY_BASED_DESTRUCT, vec![this]),
                (NI_SIZED_DELETE, vec![this, 0x20]),
            ]
        );
        log.borrow_mut().clear();
        e.mem.set_u32(this + BOOL_DATA, 0);
        e.call(0x00a5_2f30, &args![this, 0u32]);
        assert_eq!(*log.borrow(), [(KEY_BASED_DESTRUCT, vec![this])]);
    }

    #[test]
    fn create_object_and_create_clone_construct_the_interpolator() {
        let mut e = Engine::new();
        let log = Log::default();
        let block = e.mem.alloc(0x20);
        e.mem.write(block, &[0xaa; 0x20]);
        logged(&mut e, &log, NI_ALLOCATE, block);
        logged(&mut e, &log, BOOL_INTERPOLATOR_CONSTRUCT, block);
        logged(&mut e, &log, BOOL_INTERPOLATOR_COPY_MEMBERS, 0);
        assert_eq!(e.call(0x00a5_2fe0, &args![]).u32(), block);
        assert_eq!(e.mem.u32(block), NI_BOOL_TIMELINE_INTERPOLATOR_VTABLE);
        assert_eq!(e.mem.u32(block + PREVIOUS_INDEX), 0);
        assert_eq!(e.mem.u8(block + PREVIOUS_VALUE), 0);
        assert_eq!(e.mem.u8(block + PREVIOUS_VALUE + 1), 0xaa);
        assert_eq!(
            *log.borrow(),
            [
                (NI_ALLOCATE, vec![0x20]),
                (BOOL_INTERPOLATOR_CONSTRUCT, vec![block, 0]),
            ]
        );
        log.borrow_mut().clear();
        let this = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00a5_2f80, &args![this, 0x9000u32]).u32(), block);
        assert_eq!(
            log.borrow()[2],
            (BOOL_INTERPOLATOR_COPY_MEMBERS, vec![this, block, 0x9000])
        );
        logged(&mut e, &log, NI_ALLOCATE, 0);
        log.borrow_mut().clear();
        assert_eq!(e.call(0x00a5_2fe0, &args![]).u32(), 0);
        assert_eq!(e.call(0x00a5_2f80, &args![this, 0x9000u32]).u32(), 0);
        assert_eq!(
            *log.borrow(),
            [
                (NI_ALLOCATE, vec![0x20]),
                (NI_ALLOCATE, vec![0x20]),
                (BOOL_INTERPOLATOR_COPY_MEMBERS, vec![this, 0, 0x9000]),
            ]
        );
    }
}

#[cfg(test)]
mod n002_tests {
    use super::*;

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
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
    fn data_constructor_builds_on_the_bool_interpolator() {
        let mut e = Engine::new();
        e.register(BOOL_INTERPOLATOR_CONSTRUCT, |_, a| word(a[0]));
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x20);
        e.mem.write(this, &[0xcd; 0x20]);
        assert_eq!(e.call(0x00a5_2f00, &args![this, 0x5000u32]).u32(), this);
        assert_eq!(
            calls(&e, BOOL_INTERPOLATOR_CONSTRUCT),
            vec![vec![this, 0x5000]]
        );
        assert_eq!(e.mem.u32(this), NI_BOOL_TIMELINE_INTERPOLATOR_VTABLE);
        assert_eq!(e.mem.u32(this + PREVIOUS_INDEX), 0);
        assert_eq!(e.mem.u8(this + PREVIOUS_VALUE), 0);
        assert_eq!(e.mem.u8(this + PREVIOUS_VALUE + 1), 0xcd);
    }

    #[test]
    fn get_viewer_strings_appends_the_class_name() {
        for (capacity, grows) in [(4u16, false), (2, true)] {
            let mut e = Engine::new();
            e.map(NI_BOOL_TIMELINE_INTERPOLATOR_RTTI, 4);
            e.set_global(NI_BOOL_TIMELINE_INTERPOLATOR_RTTI, 0x0109_3333u32);
            e.register(BOOL_INTERPOLATOR_GET_VIEWER_STRINGS, |_, _| Ret::default());
            e.register(NI_GET_VIEWER_STRING, |_, _| word(0xc1));
            e.register(STRINGS_GROW, |_, _| Ret::default());
            e.register(STRINGS_SET_AT, |e, a| {
                assert_eq!(e.mem.u32(a[2]), 0xc1);
                Ret::default()
            });
            e.call_log = Some(vec![]);
            let this = e.mem.alloc(0x20);
            let strings = e.mem.alloc(0x10);
            e.mem.set_u16(strings + 8, capacity);
            e.mem.set_u16(strings + 0xa, 2);
            e.mem.set_u16(strings + 0xe, 3);
            e.call(0x00a5_3020, &args![this, strings]);
            assert_eq!(
                calls(&e, BOOL_INTERPOLATOR_GET_VIEWER_STRINGS),
                vec![vec![this, strings]]
            );
            assert_eq!(calls(&e, NI_GET_VIEWER_STRING), vec![vec![0x0109_3333]]);
            let grow = calls(&e, STRINGS_GROW);
            if grows {
                assert_eq!(grow, vec![vec![strings, 5]]);
            } else {
                assert!(grow.is_empty());
            }
            assert_eq!(calls(&e, STRINGS_SET_AT)[0][..2], [strings, 2]);
        }
    }
}
