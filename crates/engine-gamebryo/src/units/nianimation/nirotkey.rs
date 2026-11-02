//! `NiAnimation/nirotkey.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::nianimation::niposkey::{key_ratio, search_keys, LOWEST_TIME};

/// The rotation keys' interpolation functions, one word per key type
/// (`011f3cc0`; each `cdecl (ratio or time, key, next key, &result)`).
const ROT_KEY_INTERPOLATIONS: u32 = 0x011f_3cc0;
/// The key type the exe hands the whole key array and the time (Euler
/// rotation keys in Gamebryo's numbering; the name is not confirmed here).
const WHOLE_ARRAY_TYPE: u32 = 4;

// Translated from 00a28740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRotKey::GenInterp` (Xbox PDB, `cdecl`, the quaternion returned through
/// `result`): for key type 4, always the type's function (`011f3cc0`) with
/// the time itself, the key array and a null next key, into a local
/// quaternion copied to `result` (the last index untouched). Otherwise,
/// with one key or at time `-FLT_MAX`, the first key's quaternion (+4);
/// else the key search (`search_keys`) and the type's function between the
/// keys `index` and `next_index` with the ratio, the last index becoming
/// `index` (the search running off the end still interpolates, towards the
/// key past the array). Answers `result`.
pub fn ni_rot_key_gen_interp(
    e: &mut Engine,
    result: Ptr,
    time: f32,
    keys: Ptr,
    key_type: u32,
    num_keys: u32,
    last_index: Ptr,
    key_size: u8,
) -> Ptr {
    let time_value = x87_load32(time.to_bits());
    let function = |e: &Engine| {
        e.mem
            .u32(ROT_KEY_INTERPOLATIONS.wrapping_add(key_type.wrapping_mul(4)))
    };
    let copy = |e: &mut Engine, from: u32| {
        for word in 0..4u32 {
            let value = e.mem.u32(from.wrapping_add(4 * word));
            e.mem.set_u32(result.addr().wrapping_add(4 * word), value);
        }
    };
    let lowest = num_keys == 1 || time_value == f64::from_bits(e.global(LOWEST_TIME));
    if key_type == WHOLE_ARRAY_TYPE {
        let function = function(e);
        let time_bits = x87_store32(time_value);
        e.with_stack(16, |e, rotation| {
            e.call(function, &args![time_bits, keys, 0u32, rotation]);
            copy(e, rotation.addr());
        });
        return result;
    }
    if lowest {
        copy(e, keys.addr().wrapping_add(4));
        return result;
    }
    let size = u32::from(key_size);
    let last = e.mem.u32(last_index.addr());
    let search = search_keys(e, time_value, keys.addr(), num_keys, last, size);
    let ratio = key_ratio(time_value, &search);
    let key = keys.addr().wrapping_add(size.wrapping_mul(search.index));
    let next_key = keys
        .addr()
        .wrapping_add(size.wrapping_mul(search.next_index));
    let function = function(e);
    e.with_stack(16, |e, rotation| {
        e.call(function, &args![ratio, key, next_key, rotation]);
        e.mem.set_u32(last_index.addr(), search.index);
        copy(e, rotation.addr());
    });
    result
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00a28740,
        ni_rot_key_gen_interp(Ptr, f32, Ptr, u32, u32, Ptr, u8) -> Ptr
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const INTERPOLATE: u32 = 0x7100_0000;

    type Log = Rc<RefCell<Vec<Vec<u32>>>>;

    /// An engine with the lowest-time constant, type `key_type`'s function a
    /// double that logs its words and writes (1, 2, 3, 4), and 6 keys of 20
    /// bytes at times 0..5 (the first key's quaternion (5, 6, 7, 8)).
    fn setup(key_type: u32) -> (Engine, u32, Log) {
        let mut e = Engine::new();
        e.map(0x0102_4000, 0x1000);
        e.set_global(LOWEST_TIME, -3.4028234663852886e38f64);
        e.map(0x011f_3000, 0x1000);
        e.mem
            .set_u32(ROT_KEY_INTERPOLATIONS + 4 * key_type, INTERPOLATE);
        let seen: Log = Rc::default();
        let log = seen.clone();
        e.register_double(INTERPOLATE, move |e, a| {
            log.borrow_mut().push(a.to_vec());
            for i in 0..4 {
                e.mem.set_f32(a[3] + 4 * i, (i + 1) as f32);
            }
            Ret::default()
        });
        let keys = e.mem.alloc(20 * 7);
        for k in 0..6u32 {
            e.mem.set_f32(keys + 20 * k, k as f32);
        }
        for i in 0..4u32 {
            e.mem.set_f32(keys + 4 + 4 * i, (i + 5) as f32);
        }
        (e, keys, seen)
    }

    fn rotation(e: &Engine, at: u32) -> Vec<f32> {
        (0..4).map(|i| e.mem.f32(at + 4 * i)).collect()
    }

    #[test]
    fn interpolates_between_the_found_keys() {
        let (mut e, keys, seen) = setup(1);
        let last = e.mem.alloc(4);
        let result = e.mem.alloc(16);
        let r = e
            .call(
                0x00a28740,
                &args![result, 1.5f32, keys, 1u32, 6u32, last, 20u32],
            )
            .u32();
        assert_eq!(r, result);
        assert_eq!(e.mem.u32(last), 1);
        let calls = seen.borrow();
        assert_eq!(f32::from_bits(calls[0][0]), 0.5);
        assert_eq!(&calls[0][1..3], [keys + 20, keys + 40]);
        assert_eq!(rotation(&e, result), [1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn first_key_for_one_key_or_the_lowest_time() {
        for (count, time) in [(1u32, 3.0f32), (6, f32::MIN)] {
            let (mut e, keys, seen) = setup(1);
            let last = e.mem.alloc(4);
            let result = e.mem.alloc(16);
            e.call(
                0x00a28740,
                &args![result, time, keys, 1u32, count, last, 20u32],
            );
            assert_eq!(rotation(&e, result), [5.0, 6.0, 7.0, 8.0]);
            assert!(seen.borrow().is_empty());
        }
    }

    #[test]
    fn type_4_hands_the_function_the_time_and_the_whole_array() {
        for count in [1u32, 6] {
            let (mut e, keys, seen) = setup(4);
            let last = e.mem.alloc(4);
            e.mem.set_u32(last, 3);
            let result = e.mem.alloc(16);
            e.call(
                0x00a28740,
                &args![result, 2.5f32, keys, 4u32, count, last, 20u32],
            );
            let calls = seen.borrow();
            assert_eq!(calls.len(), 1);
            assert_eq!(&calls[0][..3], [2.5f32.to_bits(), keys, 0]);
            assert_eq!(rotation(&e, result), [1.0, 2.0, 3.0, 4.0]);
            assert_eq!(e.mem.u32(last), 3);
        }
    }
}
