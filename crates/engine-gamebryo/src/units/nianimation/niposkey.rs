//! `NiAnimation/niposkey.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `-FLT_MAX` (`double`, `010241b0`): the time the `GenInterp` functions
/// answer the first key's value for.
pub(crate) const LOWEST_TIME: u32 = 0x0102_41b0;
/// The position keys' interpolation functions, one word per key type
/// (`011f3ca8`; each `cdecl (ratio, key, next key, &result)`).
const POS_KEY_INTERPOLATIONS: u32 = 0x011f_3ca8;

/// Where the key search of the `GenInterp` functions stopped.
pub(crate) struct KeySearch {
    /// The key before the time (the exe's `EDI`).
    pub(crate) index: u32,
    /// The key after it, `index + 1` (`ECX`); `num_keys` or more when the
    /// search ran off the end.
    pub(crate) next_index: u32,
    /// The time of the key before (kept as a float on the stack).
    pub(crate) previous_time: f64,
    /// The last key time the search loaded, or 0 when it loaded none (the
    /// value left on the x87 stack).
    pub(crate) next_time: f64,
}

/// The key search the `GenInterp` functions share (`NiFloatKey`, `NiPosKey`,
/// `NiRotKey`, the same code inlined in each). Keys are `size` bytes apart,
/// each starting with its float time. It starts at the last index, back at
/// key 0 when that key's time is after `time`, then walks forward (four keys
/// per round while at least four remain, counted signed, then one by one up
/// to `num_keys - 1`, unsigned) to the first key whose time is not before
/// `time` (`FCOM`: a NaN on either side counts as before).
pub(crate) fn search_keys(
    e: &mut Engine,
    time: f64,
    keys: u32,
    num_keys: u32,
    last_index: u32,
    size: u32,
) -> KeySearch {
    let key_time =
        |e: &Engine, k: u32| x87_load32(e.mem.u32(keys.wrapping_add(k.wrapping_mul(size))));
    let mut index = last_index;
    let mut previous_time = key_time(e, index);
    if previous_time > time {
        previous_time = key_time(e, 0);
        index = 0;
    }
    let mut next_index = index.wrapping_add(1);
    let mut next_time = 0.0;
    let remaining = num_keys
        .wrapping_sub(1)
        .wrapping_sub(next_index)
        .wrapping_add(1) as i32;
    if remaining >= 4 {
        loop {
            for step in 0..4u32 {
                let k = key_time(e, next_index.wrapping_add(step));
                next_time = k;
                if k >= time {
                    return KeySearch {
                        index: index.wrapping_add(step),
                        next_index: next_index.wrapping_add(step),
                        previous_time,
                        next_time,
                    };
                }
                previous_time = k;
            }
            index = index.wrapping_add(4);
            next_index = next_index.wrapping_add(4);
            if next_index > num_keys.wrapping_sub(4) {
                break;
            }
        }
    }
    while next_index <= num_keys.wrapping_sub(1) {
        let k = key_time(e, next_index);
        next_time = k;
        if k >= time {
            break;
        }
        next_index = next_index.wrapping_add(1);
        previous_time = k;
        index = index.wrapping_add(1);
    }
    KeySearch {
        index,
        next_index,
        previous_time,
        next_time,
    }
}

/// The ratio `(time - previous) / (next - previous)` the `GenInterp`
/// functions give their interpolation function, as the float they store.
pub(crate) fn key_ratio(time: f64, search: &KeySearch) -> u32 {
    let elapsed = x87_sub(time, search.previous_time);
    let span = x87_sub(search.next_time, search.previous_time);
    x87_quiet_bits(x87_store32(x87_div(elapsed, span)))
}

// Translated from 00a27490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPosKey::GenInterp` (Xbox PDB, `cdecl`, the point returned through
/// `result`): with one key, or at time `-FLT_MAX`, the first key's point
/// (+4). Otherwise the key search (`search_keys`) and the key type's
/// interpolation function (`011f3ca8`) between the keys `index` and
/// `next_index` with the ratio, into a local point copied to `result`; the
/// last index becomes `index`. The search running off the end still
/// interpolates (towards the key at `next_index`, past the array). Answers
/// `result`.
pub fn ni_pos_key_gen_interp(
    e: &mut Engine,
    result: Ptr,
    time: f32,
    keys: Ptr,
    key_type: u32,
    num_keys: u32,
    last_index: Ptr,
    key_size: u8,
) -> Ptr {
    let first = keys.addr().wrapping_add(4);
    if num_keys == 1 || x87_load32(time.to_bits()) == f64::from_bits(e.global(LOWEST_TIME)) {
        for word in 0..3u32 {
            let value = e.mem.u32(first.wrapping_add(4 * word));
            e.mem.set_u32(result.addr().wrapping_add(4 * word), value);
        }
        return result;
    }
    let time = x87_load32(time.to_bits());
    let size = u32::from(key_size);
    let last = e.mem.u32(last_index.addr());
    let search = search_keys(e, time, keys.addr(), num_keys, last, size);
    let ratio = key_ratio(time, &search);
    let key = keys.addr().wrapping_add(size.wrapping_mul(search.index));
    let next_key = keys
        .addr()
        .wrapping_add(size.wrapping_mul(search.next_index));
    let function = e
        .mem
        .u32(POS_KEY_INTERPOLATIONS.wrapping_add(key_type.wrapping_mul(4)));
    e.with_stack(12, |e, point| {
        e.call(function, &args![ratio, key, next_key, point]);
        e.mem.set_u32(last_index.addr(), search.index);
        for word in 0..3u32 {
            let value = e.mem.u32(point.addr() + 4 * word);
            e.mem.set_u32(result.addr().wrapping_add(4 * word), value);
        }
    });
    result
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00a27490,
        ni_pos_key_gen_interp(Ptr, f32, Ptr, u32, u32, Ptr, u8) -> Ptr
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const INTERPOLATE: u32 = 0x7100_0000;

    /// Keys of 16 bytes (time, x, y, z) at times 0, 1, 2, ... (`count` of
    /// them), with x = 10 * index.
    fn keys(e: &mut Engine, count: u32) -> u32 {
        let keys = e.mem.alloc(16 * (count + 1));
        for k in 0..count {
            e.mem.set_f32(keys + 16 * k, k as f32);
            e.mem.set_f32(keys + 16 * k + 4, 10.0 * k as f32);
        }
        keys
    }

    #[test]
    fn search_keys_walks_by_fours_then_by_ones() {
        let mut e = Engine::new();
        let keys = keys(&mut e, 12);
        // (time, last index) -> (index, next index, previous, next)
        for (time, last, expected) in [
            (0.5f64, 0u32, (0u32, 1u32, 0.0f64, 1.0f64)),
            (6.5, 0, (6, 7, 6.0, 7.0)),
            (10.5, 0, (10, 11, 10.0, 11.0)),
            (3.0, 3, (3, 4, 3.0, 4.0)),
            (2.5, 9, (2, 3, 2.0, 3.0)),
            (20.0, 0, (11, 12, 11.0, 11.0)),
            (20.0, 11, (11, 12, 11.0, 0.0)),
        ] {
            let s = search_keys(&mut e, time, keys, 12, last, 16);
            assert_eq!(
                (s.index, s.next_index, s.previous_time, s.next_time),
                expected,
                "time {time}, last {last}"
            );
        }
    }

    #[test]
    fn gen_interp_interpolates_between_the_found_keys() {
        let mut e = Engine::new();
        e.map(0x0102_4000, 0x1000);
        e.set_global(LOWEST_TIME, -3.4028234663852886e38f64);
        e.map(0x011f_3000, 0x1000);
        e.mem.set_u32(POS_KEY_INTERPOLATIONS + 8, INTERPOLATE);
        let seen: Rc<RefCell<Vec<Vec<u32>>>> = Rc::default();
        let log = seen.clone();
        e.register_double(INTERPOLATE, move |e, a| {
            log.borrow_mut().push(a.to_vec());
            e.mem.set_f32(a[3], 1.0);
            e.mem.set_f32(a[3] + 4, 2.0);
            e.mem.set_f32(a[3] + 8, 3.0);
            Ret::default()
        });
        let keys = keys(&mut e, 6);
        let last = e.mem.alloc(4);
        let result = e.mem.alloc(12);
        let r = e
            .call(
                0x00a27490,
                &args![result, 3.25f32, keys, 2u32, 6u32, last, 16u32],
            )
            .u32();
        assert_eq!(r, result);
        assert_eq!(e.mem.u32(last), 3);
        let calls = seen.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(f32::from_bits(calls[0][0]), 0.25);
        assert_eq!(&calls[0][1..3], [keys + 48, keys + 64]);
        let point: Vec<f32> = (0..3).map(|i| e.mem.f32(result + 4 * i)).collect();
        assert_eq!(point, [1.0, 2.0, 3.0]);
    }

    #[test]
    fn gen_interp_answers_the_first_key_for_one_key_or_the_lowest_time() {
        for (count, time) in [(1u32, 5.0f32), (4, f32::MIN)] {
            let mut e = Engine::new();
            e.map(0x0102_4000, 0x1000);
            e.set_global(LOWEST_TIME, -3.4028234663852886e38f64);
            let keys = keys(&mut e, count);
            e.mem.set_f32(keys + 8, 7.0);
            e.mem.set_f32(keys + 12, 8.0);
            let last = e.mem.alloc(4);
            e.mem.set_u32(last, 2);
            let result = e.mem.alloc(12);
            e.call(
                0x00a27490,
                &args![result, time, keys, 2u32, count, last, 16u32],
            );
            let point: Vec<f32> = (0..3).map(|i| e.mem.f32(result + 4 * i)).collect();
            assert_eq!(point, [0.0, 7.0, 8.0]);
            assert_eq!(e.mem.u32(last), 2);
        }
    }
}
