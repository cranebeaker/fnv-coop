//! `NiAnimation/nianimationkey.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The key type the code treats as a three-sequence (Euler) rotation key.
const EULER_KEY: u32 = 4;
/// The per-key-type tables in `.data` (filled at start-up), indexed by
/// `type + 6 * content`: the key sizes (bytes, `011f3758`), the functions
/// `GuaranteeKeyAtStartAndEnd` calls (`011f3810`, `cdecl (time, &keys,
/// &count)`), the array makers `CopySequence` calls (`011f3930`, `cdecl
/// (count)`, answering the new array) and the key copies it calls
/// (`011f39c0`, `cdecl (&to, &from)`). Which of the Xbox PDB's
/// `NiAnimationKey` static tables each one is was not confirmed.
const KEY_SIZES: u32 = 0x011f_3758;
const START_AND_END_FUNCTIONS: u32 = 0x011f_3810;
const ARRAY_FUNCTIONS: u32 = 0x011f_3930;
const COPY_FUNCTIONS: u32 = 0x011f_39c0;
/// The memory manager (`011f6238`) and its allocate (`00aa3e40`,
/// `thiscall`, one word).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
/// `NiQuaternion`'s constructor from four floats (`00a6db50`, `thiscall`,
/// `RET 0x10`).
const QUATERNION_CONSTRUCTOR: u32 = 0x00a6_db50;
/// The base constructor `fn_00a3d540` starts with (`00a29ad0`, `thiscall`).
const ROTATION_KEY_BASE_CONSTRUCTOR: u32 = 0x00a2_9ad0;

/// `NiEulerRotKey` (Xbox PDB, 0x48 bytes; the PC code uses the same
/// offsets): `m_quat` +4 (the `NiRotKey` part), `m_uiNumKeys[3]` +0x14,
/// `m_eType[3]` +0x20, `m_ucSizes[3]` +0x2C, `m_apkKeys[3]` +0x30,
/// `m_uiLastIdx[3]` +0x3C.
const EULER_QUATERNION: u32 = 0x4;
const EULER_NUM_KEYS: u32 = 0x14;
const EULER_TYPES: u32 = 0x20;
const EULER_SIZES: u32 = 0x2c;
const EULER_KEYS: u32 = 0x30;
const EULER_LAST_INDEX: u32 = 0x3c;
const EULER_KEY_SIZE: u32 = 0x48;

/// A table word: the entry `index` of the word table at `table`.
fn table_word(e: &Engine, table: u32, index: u32) -> u32 {
    e.mem.u32(table.wrapping_add(index.wrapping_mul(4)))
}

// Translated from 00a3d540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, nine stack words, `RET 0x24`):
/// fills an `NiEulerRotKey` from three key sequences, each given as (keys,
/// count, type): the base constructor (`00a29ad0`), the arrays, counts and
/// types, the last indices 0, and each sequence's key size from the size
/// table by its type (0 for an empty sequence).
pub fn fn_00a3d540(
    e: &mut Engine,
    this: Ptr,
    keys_0: u32,
    count_0: u32,
    type_0: u32,
    keys_1: u32,
    count_1: u32,
    type_1: u32,
    keys_2: u32,
    count_2: u32,
    type_2: u32,
) {
    e.call(ROTATION_KEY_BASE_CONSTRUCTOR, &args![this]);
    let at = |offset: u32| this.addr().wrapping_add(offset);
    e.mem.set_u32(at(EULER_NUM_KEYS), count_0);
    e.mem.set_u32(at(EULER_TYPES), type_0);
    e.mem.set_u32(at(EULER_KEYS), keys_0);
    e.mem.set_u32(at(EULER_KEYS + 4), keys_1);
    e.mem.set_u32(at(EULER_NUM_KEYS + 4), count_1);
    e.mem.set_u32(at(EULER_NUM_KEYS + 8), count_2);
    e.mem.set_u32(at(EULER_TYPES + 4), type_1);
    e.mem.set_u32(at(EULER_TYPES + 8), type_2);
    e.mem.set_u32(at(EULER_KEYS + 8), keys_2);
    for i in 0..3 {
        e.mem.set_u32(at(EULER_LAST_INDEX + 4 * i), 0);
    }
    for i in 0..3 {
        let size = if e.mem.u32(at(EULER_NUM_KEYS + 4 * i)) == 0 {
            0
        } else {
            let key_type = e.mem.u32(at(EULER_TYPES + 4 * i));
            e.mem.u8(KEY_SIZES.wrapping_add(key_type))
        };
        e.mem.set_u8(at(EULER_SIZES + i), size);
    }
}

/// `CopySequence`'s counting test of one key time against `begin` and
/// `end` (x87 compares): `None` when the key is after `end` (the count
/// stops), otherwise whether it is counted (not before `begin`; a NaN time
/// is counted).
fn in_range(time: f64, begin: f64, end: f64) -> Option<bool> {
    if time < begin {
        return Some(false);
    }
    if time > end {
        return None;
    }
    Some(true)
}

// Translated from 00a3d630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAnimationKey::CopySequence` (Xbox PDB; `cdecl (content, type, keys,
/// count, begin, end, &new keys, &new count)`): the keys from `begin` to
/// `end` copied into a new array with their times made relative to `begin`.
///
/// For an Euler rotation key (type 4), each of the three sequences of the
/// key at `keys` is copied by itself (content 0) and the results go into a
/// new one-element `NiEulerRotKey` array from the memory manager (0x4C
/// bytes: the count word 1, then the key: its quaternion constructed from
/// four zeros, every count, type, array and last index 0 and the sizes 0;
/// the time is not set), which `fn_00a3d540` then fills (the exe runs it on
/// address 0 when the allocation failed); the new count is 1.
///
/// Otherwise, with the key size from the size table (`type + 6 *
/// content`), the keys are counted first (four at a time while more than
/// three are left, the count compared as signed for the start; a key before
/// `begin` is skipped, the first after `end` stops the count, a NaN time is
/// counted). No key: no array. Else the array maker makes the array, and
/// each key in range is copied (the copy function) to the next slot and its
/// time set to `time - begin`, stopping at the first after `end`.
pub fn ni_animation_key_copy_sequence(
    e: &mut Engine,
    content: u32,
    key_type: u32,
    keys: Ptr,
    count: u32,
    begin: f32,
    end: f32,
    new_keys: Ptr,
    new_count: Ptr,
) {
    let begin_bits = x87_quiet_bits(begin.to_bits());
    let end_bits = x87_quiet_bits(end.to_bits());
    if key_type == EULER_KEY {
        let euler = keys.addr();
        // The exe's locals: the three types, counts and arrays.
        e.with_stack(0x24, |e, frame| {
            let types = frame.addr();
            let counts = types + 0xc;
            let arrays = types + 0x18;
            for i in 0..3 {
                let sub_type = e.mem.u32(euler + EULER_TYPES + 4 * i);
                let sub_count = e.mem.u32(euler + EULER_NUM_KEYS + 4 * i);
                let sub_keys = e.mem.u32(euler + EULER_KEYS + 4 * i);
                e.mem.set_u32(types + 4 * i, sub_type);
                ni_animation_key_copy_sequence(
                    e,
                    0,
                    sub_type,
                    Ptr::new(sub_keys),
                    sub_count,
                    f32::from_bits(begin_bits),
                    f32::from_bits(end_bits),
                    Ptr::new(arrays + 4 * i),
                    Ptr::new(counts + 4 * i),
                );
            }
            let block = e
                .call(
                    MEMORY_MANAGER_ALLOCATE,
                    &args![MEMORY_MANAGER, EULER_KEY_SIZE + 4],
                )
                .u32();
            let key = if block == 0 {
                0
            } else {
                let key = block + 4;
                e.mem.set_u32(block, 1);
                e.call(
                    QUATERNION_CONSTRUCTOR,
                    &args![key + EULER_QUATERNION, 0u32, 0u32, 0u32, 0u32],
                );
                for offset in [
                    EULER_NUM_KEYS,
                    EULER_NUM_KEYS + 4,
                    EULER_NUM_KEYS + 8,
                    EULER_KEYS,
                    EULER_KEYS + 4,
                    EULER_KEYS + 8,
                    EULER_TYPES,
                    EULER_TYPES + 4,
                    EULER_TYPES + 8,
                    EULER_LAST_INDEX,
                    EULER_LAST_INDEX + 4,
                    EULER_LAST_INDEX + 8,
                ] {
                    e.mem.set_u32(key + offset, 0);
                }
                for i in 0..3 {
                    e.mem.set_u8(key + EULER_SIZES + i, 0);
                }
                key
            };
            e.mem.set_u32(new_keys.addr(), key);
            e.mem.set_u32(new_count.addr(), 1);
            let word = |e: &Engine, at: u32| e.mem.u32(at);
            let (k0, n0, t0) = (word(e, arrays), word(e, counts), word(e, types));
            let (k1, n1, t1) = (word(e, arrays + 4), word(e, counts + 4), word(e, types + 4));
            let (k2, n2, t2) = (word(e, arrays + 8), word(e, counts + 8), word(e, types + 8));
            let key = e.mem.u32(new_keys.addr());
            fn_00a3d540(e, Ptr::new(key), k0, n0, t0, k1, n1, t1, k2, n2, t2);
        });
        return;
    }
    let index = key_type.wrapping_add(content.wrapping_mul(6));
    let begin = x87_load32(begin_bits);
    let end = x87_load32(end_bits);
    e.mem.set_u32(new_count.addr(), 0);
    let size = u32::from(e.mem.u8(KEY_SIZES.wrapping_add(index)));
    let time = |e: &Engine, key: u32| x87_load32(e.mem.u32(key));
    let counted = |e: &mut Engine| {
        let n = e.mem.u32(new_count.addr()).wrapping_add(1);
        e.mem.set_u32(new_count.addr(), n);
    };
    let mut i = 0u32;
    let mut stopped = false;
    if count as i32 >= 4 {
        let mut key = keys.addr();
        loop {
            for k in 0..4 {
                let at = key.wrapping_add(k * size);
                match in_range(time(e, at), begin, end) {
                    None => {
                        stopped = true;
                        break;
                    }
                    Some(true) => counted(e),
                    Some(false) => {}
                }
            }
            if stopped {
                break;
            }
            key = key.wrapping_add(4 * size);
            i = i.wrapping_add(4);
            if i >= count.wrapping_sub(3) {
                break;
            }
        }
    }
    if !stopped && i < count {
        let mut key = size.wrapping_mul(i).wrapping_add(keys.addr());
        loop {
            match in_range(time(e, key), begin, end) {
                None => break,
                Some(true) => counted(e),
                Some(false) => {}
            }
            i = i.wrapping_add(1);
            key = key.wrapping_add(size);
            if i >= count {
                break;
            }
        }
    }
    let total = e.mem.u32(new_count.addr());
    if total == 0 {
        e.mem.set_u32(new_keys.addr(), 0);
        return;
    }
    let maker = table_word(e, ARRAY_FUNCTIONS, index);
    let array = e.call(maker, &args![total]).u32();
    e.mem.set_u32(new_keys.addr(), array);
    let copy = table_word(e, COPY_FUNCTIONS, index);
    e.mem.set_u32(new_count.addr(), 0);
    let mut key = keys.addr();
    let mut i = 0u32;
    while i < count {
        let key_time = time(e, key);
        if !(key_time < begin) {
            if end < key_time {
                return;
            }
            let slot = e
                .mem
                .u32(new_count.addr())
                .wrapping_mul(size)
                .wrapping_add(e.mem.u32(new_keys.addr()));
            e.call(copy, &args![slot, key]);
            let relative = x87_sub(time(e, key), begin);
            e.mem.set_u32(slot, x87_store32(relative));
            counted(e);
        }
        i = i.wrapping_add(1);
        key = key.wrapping_add(size);
    }
}

// Translated from 00a3d930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAnimationKey::GuaranteeKeyAtStartAndEnd` (Xbox PDB; `cdecl (content,
/// type, &keys, &count, begin, end)`): for an Euler rotation key (type 4),
/// each non-empty sequence of the key at `*keys` goes through this function
/// itself (content 0) on a copy of its array and count, which are stored
/// back with the type, the last index 0 and the size from the size table;
/// otherwise, unless there is exactly one key and it is at `begin`, the
/// table function for `type + 6 * content` runs at `begin` and then at
/// `end`.
pub fn ni_animation_key_guarantee_key_at_start_and_end(
    e: &mut Engine,
    content: u32,
    key_type: u32,
    keys: Ptr,
    count: Ptr,
    begin: f32,
    end: f32,
) {
    let begin_bits = x87_quiet_bits(begin.to_bits());
    let end_bits = x87_quiet_bits(end.to_bits());
    if key_type == EULER_KEY {
        let euler = e.mem.u32(keys.addr());
        // The exe's local (the sequence's array) and the type argument's
        // stack word (its count).
        e.with_stack(8, |e, frame| {
            let sequence_keys = frame.addr();
            let sequence_count = sequence_keys + 4;
            for i in 0..3u32 {
                let sub_count = e.mem.u32(euler + EULER_NUM_KEYS + 4 * i);
                let sub_keys = e.mem.u32(euler + EULER_KEYS + 4 * i);
                let sub_type = e.mem.u32(euler + EULER_TYPES + 4 * i);
                e.mem.set_u32(sequence_keys, sub_keys);
                e.mem.set_u32(sequence_count, sub_count);
                if sub_count != 0 {
                    ni_animation_key_guarantee_key_at_start_and_end(
                        e,
                        0,
                        sub_type,
                        Ptr::new(sequence_keys),
                        Ptr::new(sequence_count),
                        f32::from_bits(begin_bits),
                        f32::from_bits(end_bits),
                    );
                }
                let sub_count = e.mem.u32(sequence_count);
                let sub_keys = e.mem.u32(sequence_keys);
                e.mem.set_u32(euler + EULER_KEYS + 4 * i, sub_keys);
                e.mem.set_u32(euler + EULER_NUM_KEYS + 4 * i, sub_count);
                e.mem.set_u32(euler + EULER_TYPES + 4 * i, sub_type);
                e.mem.set_u32(euler + EULER_LAST_INDEX + 4 * i, 0);
                let size = e.mem.u8(KEY_SIZES.wrapping_add(sub_type));
                e.mem.set_u8(euler + EULER_SIZES + i, size);
            }
        });
        return;
    }
    let index = key_type.wrapping_add(content.wrapping_mul(6));
    let function = table_word(e, START_AND_END_FUNCTIONS, index);
    if e.mem.u32(count.addr()) == 1 {
        let first = e.mem.u32(keys.addr());
        if x87_load32(begin_bits) == x87_load32(e.mem.u32(first)) {
            return;
        }
    }
    e.call(function, &args![begin_bits, keys, count]);
    e.call(function, &args![end_bits, keys, count]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a3_d540,
            fn_00a3d540(Ptr, u32, u32, u32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00a3_d630,
            ni_animation_key_copy_sequence(u32, u32, Ptr, u32, f32, f32, Ptr, Ptr)
        ),
        entry!(
            0x00a3_d930,
            ni_animation_key_guarantee_key_at_start_and_end(u32, u32, Ptr, Ptr, f32, f32)
        ),
    ]
}

#[cfg(test)]
mod a07_tests {
    use super::*;
    use crate::units::nianimation::niblendinterpolator::a07_test_support::calls;

    /// Test doubles for the per-type tables: index 1 (float keys of 8 bytes:
    /// time and value) has an array maker giving zeroed memory and a copy of
    /// both words; the start/end function of index 2 is a recording double.
    const MAKE: u32 = 0x7600_0000;
    const COPY: u32 = 0x7600_0010;
    const GUARANTEE: u32 = 0x7600_0020;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_3000, 0x1000);
        e.mem.set_u8(KEY_SIZES + 1, 8);
        e.mem.set_u8(KEY_SIZES + 2, 0xc);
        e.mem.set_u32(ARRAY_FUNCTIONS + 4, MAKE);
        e.mem.set_u32(COPY_FUNCTIONS + 4, COPY);
        e.mem.set_u32(START_AND_END_FUNCTIONS + 8, GUARANTEE);
        e.register(MAKE, |e, a| e.mem.alloc(8 * a[0]).into_ret());
        e.register(COPY, |e, a| {
            for i in 0..2 {
                let word = e.mem.u32(a[1] + 4 * i);
                e.mem.set_u32(a[0] + 4 * i, word);
            }
            Ret::default()
        });
        e.register(GUARANTEE, |_, _| Ret::default());
        e.register(ROTATION_KEY_BASE_CONSTRUCTOR, |_, _| Ret::default());
        e.register(QUATERNION_CONSTRUCTOR, |_, _| Ret::default());
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e
    }

    /// Float keys (time, value = 10 * time) at the given times.
    fn float_keys(e: &mut Engine, times: &[f32]) -> u32 {
        let keys = e.mem.alloc(8 * times.len() as u32);
        for (i, t) in times.iter().enumerate() {
            e.mem.set_f32(keys + 8 * i as u32, *t);
            e.mem.set_f32(keys + 8 * i as u32 + 4, 10.0 * t);
        }
        keys
    }

    #[test]
    fn euler_key_fill_sets_sizes_by_type() {
        let mut e = engine();
        let key = e.mem.alloc(EULER_KEY_SIZE);
        let words = args![key, 0x100u32, 3u32, 1u32, 0x200u32, 0u32, 1u32, 0x300u32, 2u32, 2u32];
        e.call(0x00a3_d540, &words);
        assert_eq!(e.mem.u32(key + EULER_KEYS), 0x100);
        assert_eq!(e.mem.u32(key + EULER_KEYS + 8), 0x300);
        assert_eq!(e.mem.u32(key + EULER_NUM_KEYS + 8), 2);
        assert_eq!(e.mem.u32(key + EULER_TYPES + 8), 2);
        assert_eq!(e.mem.u8(key + EULER_SIZES), 8);
        assert_eq!(e.mem.u8(key + EULER_SIZES + 1), 0, "empty sequence");
        assert_eq!(e.mem.u8(key + EULER_SIZES + 2), 0xc);
    }

    #[test]
    fn copy_sequence_keeps_the_keys_in_range_relative_to_begin() {
        let mut e = engine();
        let times = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, f32::NAN, 6.0];
        let keys = float_keys(&mut e, &times);
        let (out_keys, out_count) = (e.mem.alloc(4), e.mem.alloc(4));
        let words = args![0u32, 1u32, keys, 8u32, 1.0f32, 5.0f32, out_keys, out_count];
        e.call(0x00a3_d630, &words);
        // Counted and copied: 1 to 5 and the NaN (not before 1, not after
        // 5); 6.0 stops both passes.
        assert_eq!(e.mem.u32(out_count), 6);
        assert!(e.mem.f32(e.mem.u32(out_keys) + 8 * 5).is_nan());
        let array = e.mem.u32(out_keys);
        assert_eq!(e.mem.f32(array), 0.0);
        assert_eq!(e.mem.f32(array + 4), 10.0);
        assert_eq!(e.mem.f32(array + 8 * 4), 4.0);
        assert_eq!(e.mem.f32(array + 8 * 4 + 4), 50.0);
    }

    #[test]
    fn copy_sequence_without_keys_in_range_makes_no_array() {
        let mut e = engine();
        let keys = float_keys(&mut e, &[0.0, 1.0]);
        let (out_keys, out_count) = (e.mem.alloc(4), e.mem.alloc(4));
        e.mem.set_u32(out_keys, 0x1234);
        e.call_log = Some(vec![]);
        let words = args![0u32, 1u32, keys, 2u32, 2.0f32, 3.0f32, out_keys, out_count];
        e.call(0x00a3_d630, &words);
        assert_eq!(e.mem.u32(out_keys), 0);
        assert_eq!(e.mem.u32(out_count), 0);
        assert!(calls(&e, MAKE).is_empty());
    }

    #[test]
    fn copy_sequence_of_an_euler_key_copies_each_sequence() {
        let mut e = engine();
        let source = e.mem.alloc(EULER_KEY_SIZE);
        let a = float_keys(&mut e, &[0.0, 1.0, 2.0]);
        e.mem.set_u32(source + EULER_KEYS, a);
        e.mem.set_u32(source + EULER_NUM_KEYS, 3);
        e.mem.set_u32(source + EULER_TYPES, 1);
        e.mem.set_u32(source + EULER_TYPES + 4, 1);
        e.mem.set_u32(source + EULER_TYPES + 8, 1);
        let (out_keys, out_count) = (e.mem.alloc(4), e.mem.alloc(4));
        let words = args![0u32, EULER_KEY, source, 1u32, 1.0f32, 2.0f32, out_keys, out_count];
        e.call(0x00a3_d630, &words);
        assert_eq!(e.mem.u32(out_count), 1);
        let key = e.mem.u32(out_keys);
        assert_eq!(e.mem.u32(key - 4), 1, "the array's count word");
        assert_eq!(e.mem.u32(key + EULER_NUM_KEYS), 2);
        assert_eq!(e.mem.u32(key + EULER_NUM_KEYS + 4), 0);
        assert_eq!(e.mem.u32(key + EULER_KEYS + 4), 0);
        assert_eq!(e.mem.u8(key + EULER_SIZES), 8);
        let copied = e.mem.u32(key + EULER_KEYS);
        assert_eq!(e.mem.f32(copied + 8), 1.0);
    }

    #[test]
    fn guarantee_key_runs_the_table_function_at_both_ends() {
        let mut e = engine();
        let keys = float_keys(&mut e, &[0.5]);
        let (keys_ref, count_ref) = (e.mem.alloc(4), e.mem.alloc(4));
        e.mem.set_u32(keys_ref, keys);
        e.mem.set_u32(count_ref, 1);
        e.call_log = Some(vec![]);
        let words = args![0u32, 2u32, keys_ref, count_ref, 0.5f32, 2.0f32];
        e.call(0x00a3_d930, &words);
        assert!(calls(&e, GUARANTEE).is_empty(), "one key already at begin");
        e.mem.set_u32(count_ref, 2);
        e.call(0x00a3_d930, &words);
        assert_eq!(
            calls(&e, GUARANTEE),
            vec![
                vec![0.5f32.to_bits(), keys_ref, count_ref],
                vec![2.0f32.to_bits(), keys_ref, count_ref]
            ]
        );
    }

    #[test]
    fn guarantee_key_of_an_euler_key_reaches_each_sequence() {
        let mut e = engine();
        e.mem.set_u32(START_AND_END_FUNCTIONS + 4, GUARANTEE);
        e.register(GUARANTEE, |e, a| {
            let count = e.mem.u32(a[2]) + 1;
            e.mem.set_u32(a[2], count);
            Ret::default()
        });
        let key = e.mem.alloc(EULER_KEY_SIZE);
        e.mem.set_u32(key + EULER_NUM_KEYS + 4, 3);
        e.mem.set_u32(key + EULER_TYPES + 4, 1);
        e.mem.set_u32(key + EULER_KEYS + 4, 0x5000);
        e.mem.set_u32(key + EULER_LAST_INDEX + 4, 7);
        let keys_ref = e.mem.alloc(4);
        e.mem.set_u32(keys_ref, key);
        e.call_log = Some(vec![]);
        let words = args![0u32, EULER_KEY, keys_ref, 0u32, 0.0f32, 1.0f32];
        e.call(0x00a3_d930, &words);
        assert_eq!(calls(&e, GUARANTEE).len(), 2, "only the non-empty sequence");
        assert_eq!(e.mem.u32(key + EULER_NUM_KEYS + 4), 5);
        assert_eq!(e.mem.u32(key + EULER_LAST_INDEX + 4), 0);
        assert_eq!(e.mem.u8(key + EULER_SIZES + 1), 8);
        assert_eq!(e.mem.u32(key + EULER_KEYS + 4), 0x5000);
    }
}
