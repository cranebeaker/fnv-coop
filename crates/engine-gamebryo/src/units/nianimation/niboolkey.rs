//! `NiAnimation/niboolkey.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The invalid time (`010241b0`, -FLT_MAX as a `double`).
const INVALID_TIME: u32 = 0x0102_41b0;
/// The bool interpolation functions by key type (`011f3d08`, one word per
/// type; filled at run time).
const INTERPOLATORS: u32 = 0x011f_3d08;
/// A key's `bool` value (+4; the time is the `float` at +0).
const KEY_VALUE: u32 = 4;

/// The time of key `index` (`FLD`/`FSTP`/`FLD` through a `float` local).
fn key_time(e: &Engine, keys: u32, size: u32, index: u32) -> f64 {
    let address = keys.wrapping_add(size.wrapping_mul(index));
    x87_load32(e.mem.u32(address))
}

// Translated from 00a29250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolKey::GenInterp` (engine map; cdecl): the `bool` value at `time`
/// of the `num_keys` keys of `key_size` bytes at `keys`. With one key, or
/// at the invalid time (`FCOM` with -FLT_MAX), the first key's value. The
/// search starts at `*last_index` (back at key 0 when that key is later
/// than `time`) and walks forward (four keys at a time while at least four
/// remain, then one by one) to the first key not earlier than `time`;
/// the type's interpolation function (`011f3d08[key_type]`) gets the ratio
/// `(time - t0) / (t1 - t0)` (rounded to a `float`), the two keys and the
/// address of a stack word holding the ratio, into which it writes the
/// value. `*last_index` becomes the earlier key's index; the result is the
/// low byte of that word.
pub fn ni_bool_key_gen_interp(
    e: &mut Engine,
    time: f32,
    keys: Ptr,
    key_type: u32,
    num_keys: u32,
    last_index: Ptr,
    key_size: u8,
) -> u8 {
    let base = keys.addr();
    if num_keys == 1 {
        return e.mem.u8(base + KEY_VALUE);
    }
    let t = x87_load32(time.to_bits());
    if t == e.global::<f64>(INVALID_TIME) {
        return e.mem.u8(base + KEY_VALUE);
    }
    let size = u32::from(key_size);
    let last = num_keys.wrapping_sub(1);
    let mut first = e.mem.u32(last_index.addr());
    let mut earlier = key_time(e, base, size, first);
    if earlier > t {
        earlier = key_time(e, base, size, 0);
        first = 0;
    }
    let mut index = first.wrapping_add(1);
    let later = 'search: {
        // The value on the x87 stack when the walk ends without a find.
        let mut current = 0.0f64;
        if (last.wrapping_sub(index).wrapping_add(1) as i32) >= 4 {
            loop {
                let n0 = key_time(e, base, size, index);
                if n0 >= t {
                    break 'search n0;
                }
                earlier = n0;
                let n1 = key_time(e, base, size, index.wrapping_add(1));
                if n1 >= t {
                    index = index.wrapping_add(1);
                    first = first.wrapping_add(1);
                    break 'search n1;
                }
                earlier = n1;
                let n2 = key_time(e, base, size, index.wrapping_add(2));
                if n2 >= t {
                    index = index.wrapping_add(2);
                    first = first.wrapping_add(2);
                    break 'search n2;
                }
                earlier = n2;
                let n3 = key_time(e, base, size, index.wrapping_add(3));
                if n3 >= t {
                    index = index.wrapping_add(3);
                    first = first.wrapping_add(3);
                    break 'search n3;
                }
                earlier = n3;
                current = n3;
                first = first.wrapping_add(4);
                index = index.wrapping_add(4);
                if index > last.wrapping_sub(3) {
                    break;
                }
            }
        }
        while index <= last {
            let n = key_time(e, base, size, index);
            current = n;
            if n >= t {
                break;
            }
            index = index.wrapping_add(1);
            earlier = n;
            first = first.wrapping_add(1);
        }
        current
    };
    let ratio = x87_store32(x87_div(x87_sub(t, earlier), x87_sub(later, earlier)));
    let key_a = base.wrapping_add(size.wrapping_mul(first));
    let key_b = base.wrapping_add(size.wrapping_mul(index));
    let interpolate = e
        .mem
        .u32(INTERPOLATORS.wrapping_add(key_type.wrapping_mul(4)));
    let value = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), ratio);
        e.call(interpolate, &args![ratio, key_a, key_b, slot]);
        e.mem.u8(slot.addr())
    });
    e.mem.set_u32(last_index.addr(), first);
    value
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00a2_9250,
        ni_bool_key_gen_interp(f32, Ptr, u32, u32, Ptr, u8) -> u8
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    const INTERPOLATE: u32 = 0x0059_0000;

    /// Keys of 8 bytes: the time, then the value byte (the index + 1).
    fn keys(e: &mut Engine, times: &[f32]) -> u32 {
        let keys = e.mem.alloc(8 * times.len() as u32);
        for (i, time) in times.iter().enumerate() {
            e.mem.set_f32(keys + 8 * i as u32, *time);
            e.mem.set_u8(keys + 8 * i as u32 + 4, i as u8 + 1);
        }
        keys
    }

    fn setup(e: &mut Engine) {
        e.map(INVALID_TIME, 8);
        e.mem.set_f64(INVALID_TIME, -f64::from(f32::MAX));
        e.map(INTERPOLATORS, 0x10);
        e.mem.set_u32(INTERPOLATORS + 8, INTERPOLATE);
        // Writes 0x5a into the slot's low byte; the slot held the ratio.
        e.register(INTERPOLATE, |e, a| {
            assert_eq!(e.mem.u32(a[3]), a[0]);
            e.mem.set_u8(a[3], 0x5a);
            Ret::default()
        });
    }

    /// Runs the search; answers the interpolation's (ratio, key a, key b)
    /// and the new last index.
    fn search(
        e: &mut Engine,
        keys: u32,
        count: u32,
        time: f32,
        start: u32,
    ) -> (f32, u32, u32, u32) {
        let last = e.mem.alloc(4);
        e.mem.set_u32(last, start);
        e.call_log = Some(vec![]);
        let value = e
            .call(0x00a2_9250, &args![time, keys, 2u32, count, last, 8u32])
            .u32();
        assert_eq!(value & 0xff, 0x5a);
        let log = e.call_log.take().unwrap();
        let call = log.iter().find(|(a, _)| *a == INTERPOLATE).unwrap();
        (
            f32::from_bits(call.1[0]),
            (call.1[1] - keys) / 8,
            (call.1[2] - keys) / 8,
            e.mem.u32(last),
        )
    }

    #[test]
    fn one_key_or_the_invalid_time_gives_the_first_value() {
        let mut e = Engine::new();
        setup(&mut e);
        let k = keys(&mut e, &[0.0, 1.0]);
        let last = e.mem.alloc(4);
        assert_eq!(
            e.call(0x00a2_9250, &args![0.5f32, k, 2u32, 1u32, last, 8u32])
                .u32()
                & 0xff,
            1
        );
        assert_eq!(
            e.call(0x00a2_9250, &args![-f32::MAX, k, 2u32, 2u32, last, 8u32])
                .u32()
                & 0xff,
            1
        );
    }

    #[test]
    fn finds_the_bracketing_keys_one_by_one() {
        let mut e = Engine::new();
        setup(&mut e);
        let k = keys(&mut e, &[0.0, 1.0, 2.0]);
        assert_eq!(search(&mut e, k, 3, 1.5, 0), (0.5, 1, 2, 1));
        // A start after the time goes back to key 0.
        assert_eq!(search(&mut e, k, 3, 0.25, 2), (0.25, 0, 1, 0));
        // Past the last key: the walk ends at index 3 with the last time.
        let (ratio, a, b, last) = search(&mut e, k, 3, 5.0, 0);
        assert_eq!((a, b, last), (2, 3, 2));
        assert!(ratio.is_infinite());
    }

    #[test]
    fn finds_the_bracketing_keys_four_at_a_time() {
        let mut e = Engine::new();
        setup(&mut e);
        let times: Vec<f32> = (0..12).map(|i| i as f32).collect();
        let k = keys(&mut e, &times);
        for (time, a) in [
            (1.5f32, 1u32),
            (2.5, 2),
            (3.5, 3),
            (4.5, 4),
            (8.25, 8),
            (10.5, 10),
        ] {
            let (ratio, ka, kb, last) = search(&mut e, k, 12, time, 0);
            assert_eq!((ka, kb, last), (a, a + 1, a), "time {time}");
            assert_eq!(ratio, time - a as f32);
        }
        // Exactly on a key: that key is the later one.
        assert_eq!(search(&mut e, k, 12, 6.0, 0), (1.0, 5, 6, 5));
        // Fewer than four left from the start: the one-by-one walk only,
        // the later time starting as 0.0.
        assert_eq!(search(&mut e, k, 12, 11.0, 9), (1.0, 10, 11, 10));
    }
}
