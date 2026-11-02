//! `NiAnimation/nifloatkey.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::nianimation::niposkey::{key_ratio, search_keys, LOWEST_TIME};

/// The float keys' interpolation functions, one word per key type
/// (`011f3c90`; each `cdecl (ratio, key, next key, &result)`).
const FLOAT_KEY_INTERPOLATIONS: u32 = 0x011f_3c90;

// Translated from 00a26b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatKey::GenInterp` (Xbox PDB, `cdecl`, the value in ST0): with one
/// key, or at time `-FLT_MAX`, the first key's value (+4). Otherwise the key
/// search (`search_keys`); when it ran off the end (`next_index >=
/// num_keys`) the value of the key at `index`, else the key type's
/// interpolation function (`011f3c90`) between the keys `index` and
/// `next_index` with the ratio (its result is written into the stack word
/// of the key-count argument, which held the ratio). Either way the last
/// index becomes `index`.
pub fn ni_float_key_gen_interp(
    e: &mut Engine,
    time: f32,
    keys: Ptr,
    key_type: u32,
    num_keys: u32,
    last_index: Ptr,
    key_size: u8,
) -> f64 {
    let first_value = keys.addr().wrapping_add(4);
    if num_keys == 1 {
        return x87_load32(e.mem.u32(first_value));
    }
    let time = x87_load32(time.to_bits());
    if time == f64::from_bits(e.global(LOWEST_TIME)) {
        return x87_load32(e.mem.u32(first_value));
    }
    let size = u32::from(key_size);
    let last = e.mem.u32(last_index.addr());
    let search = search_keys(e, time, keys.addr(), num_keys, last, size);
    let key = keys.addr().wrapping_add(size.wrapping_mul(search.index));
    if search.next_index >= num_keys {
        let value = e.mem.u32(key.wrapping_add(4));
        e.mem.set_u32(last_index.addr(), search.index);
        return x87_load32(value);
    }
    let ratio = key_ratio(time, &search);
    let next_key = keys
        .addr()
        .wrapping_add(size.wrapping_mul(search.next_index));
    let function = e
        .mem
        .u32(FLOAT_KEY_INTERPOLATIONS.wrapping_add(key_type.wrapping_mul(4)));
    let value = e.with_stack(4, |e, out| {
        e.mem.set_u32(out.addr(), ratio);
        e.call(function, &args![ratio, key, next_key, out]);
        e.mem.u32(out.addr())
    });
    e.mem.set_u32(last_index.addr(), search.index);
    x87_load32(value)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00a26b40,
        ni_float_key_gen_interp(f32, Ptr, u32, u32, Ptr, u8) -> f64
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const INTERPOLATE: u32 = 0x7100_0000;

    type Log = Rc<RefCell<Vec<Vec<u32>>>>;

    /// 8-byte keys (time, value) at times 0..count-1 with value 10 * index;
    /// type 3's function logs its words and answers ratio + 100.
    fn setup(count: u32) -> (Engine, u32, Log) {
        let mut e = Engine::new();
        e.map(0x0102_4000, 0x1000);
        e.set_global(LOWEST_TIME, -3.4028234663852886e38f64);
        e.map(0x011f_3000, 0x1000);
        e.mem.set_u32(FLOAT_KEY_INTERPOLATIONS + 12, INTERPOLATE);
        let seen: Log = Rc::default();
        let log = seen.clone();
        e.register_double(INTERPOLATE, move |e, a| {
            log.borrow_mut().push(a.to_vec());
            e.mem.set_f32(a[3], f32::from_bits(a[0]) + 100.0);
            Ret::default()
        });
        let keys = e.mem.alloc(8 * (count + 1));
        for k in 0..count {
            e.mem.set_f32(keys + 8 * k, k as f32);
            e.mem.set_f32(keys + 8 * k + 4, 10.0 * k as f32);
        }
        (e, keys, seen)
    }

    #[test]
    fn interpolates_inside_and_holds_the_last_value_past_the_end() {
        let (mut e, keys, seen) = setup(5);
        let last = e.mem.alloc(4);
        let v = e
            .call(0x00a26b40, &args![2.75f32, keys, 3u32, 5u32, last, 8u32])
            .f64();
        assert_eq!(v, 100.75);
        assert_eq!(e.mem.u32(last), 2);
        assert_eq!(&seen.borrow()[0][1..3], [keys + 16, keys + 24]);
        // Past the last key: the last key's value, no interpolation.
        let v = e
            .call(0x00a26b40, &args![9.0f32, keys, 3u32, 5u32, last, 8u32])
            .f64();
        assert_eq!(v, 40.0);
        assert_eq!(e.mem.u32(last), 4);
        assert_eq!(seen.borrow().len(), 1);
    }

    #[test]
    fn first_value_for_one_key_or_the_lowest_time() {
        for (count, time) in [(1u32, 3.0f32), (5, f32::MIN)] {
            let (mut e, keys, seen) = setup(count);
            e.mem.set_f32(keys + 4, 7.5);
            let last = e.mem.alloc(4);
            e.mem.set_u32(last, 2);
            let v = e
                .call(0x00a26b40, &args![time, keys, 3u32, count, last, 8u32])
                .f64();
            assert_eq!(v, 7.5);
            assert_eq!(e.mem.u32(last), 2);
            assert!(seen.borrow().is_empty());
        }
    }
}
