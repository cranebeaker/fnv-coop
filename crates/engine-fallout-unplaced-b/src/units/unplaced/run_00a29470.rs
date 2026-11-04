//! `(unplaced)/run_00a29470` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00a294f0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f367e` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a294f0(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_367e) != 0 {
        return 0;
    }
    e.mem.set_u8(0x011f_367e, 0x1);
    e.mem.set_u32(0x011f_3e28, 0xe7cd40);
    e.mem.set_u32(0x011f_3ac8, 0xa29680);
    e.mem.set_u32(0x011f_3a38, 0xa29680);
    e.mem.set_u32(0x011f_39a8, 0xe7cd40);
    e.mem.set_u32(0x011f_3918, 0xa29680);
    e.mem.set_u8(0x011f_3776, 0x8);
    e.call(0x00a2_91f0, &args![0x5u32, 0x0u32]);
    1
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a294f0, fn_00a294f0() -> u32),
        entry!(0x00a2_9470, fn_00a29470(f32, u32, u32, u32, u32) -> u8),
    ]
}

/// `NiBoolKey::GenInterp` (engine map), `00a29250` (cdecl: time, keys, key
/// type, key count, &last index, key size; answers the value in AL).
const BOOL_KEY_GEN_INTERP: u32 = 0x00a2_9250;
/// The bool value answered when there are no keys.
const DEFAULT_BOOL_011F367D: u32 = 0x011f_367d;

// Translated from 00a29470 (decompiled, FalloutNV.exe 1.4.0.525)
/// The bool-key value at `time` (cdecl; no Xbox PDB name; called by
/// `00a2cd60`'s code with key type 5 and key size 8): the byte at `011f367d`
/// without keys; the first key's value (byte +4) when `time` is before the
/// first key's time (`FCOMP`: false when unordered); the last key's value
/// when the last key's time is before `time` (`TEST AH,5`: false when
/// unordered); otherwise `NiBoolKey::GenInterp` (`00a29250`) from index 0,
/// whose answer it passes on. Keys are `key_size` (its low byte) bytes apart.
pub fn fn_00a29470(
    e: &mut Engine,
    time: f32,
    keys: u32,
    key_type: u32,
    count: u32,
    key_size: u32,
) -> u8 {
    if count == 0 {
        return e.mem.u8(DEFAULT_BOOL_011F367D);
    }
    let at = x87_load32(time.to_bits());
    if at < x87_load32(e.mem.u32(keys)) {
        return e.mem.u8(keys + 4);
    }
    let last = keys.wrapping_add(count.wrapping_sub(1).wrapping_mul(key_size & 0xff));
    if x87_load32(e.mem.u32(last)) < at {
        return e.mem.u8(last + 4);
    }
    e.with_stack(4, |e, last_index| {
        e.mem.set_u32(last_index.addr(), 0);
        let time_bits = x87_quiet_bits(time.to_bits());
        e.call(
            BOOL_KEY_GEN_INTERP,
            &[
                time_bits,
                keys,
                key_type,
                count,
                last_index.addr(),
                key_size,
            ],
        )
        .u8()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bool_keys_clamp_then_interpolate() {
        let mut e = Engine::new();
        e.map(0x011f_3000, 0x1000);
        e.mem.set_u8(DEFAULT_BOOL_011F367D, 0x42);
        e.register(BOOL_KEY_GEN_INTERP, |e, a| {
            assert_eq!(e.mem.u32(a[4]), 0);
            (0x100 | a[3] | (a[2] << 4)).into_ret()
        });
        let keys = e.mem.alloc(24);
        for (i, (t, v)) in [(1.0f32, 1u8), (2.0, 0), (3.0, 7)].into_iter().enumerate() {
            e.mem.set_u32(keys + 8 * i as u32, t.to_bits());
            e.mem.set_u8(keys + 8 * i as u32 + 4, v);
        }
        let at = |e: &mut Engine, t: f32, count: u32| {
            e.call(0x00a2_9470, &args![t, keys, 5u32, count, 0xff08u32])
                .u8()
        };
        assert_eq!(at(&mut e, 0.5, 0), 0x42);
        assert_eq!(at(&mut e, 0.5, 3), 1);
        assert_eq!(at(&mut e, 3.5, 3), 7);
        assert_eq!(at(&mut e, 2.5, 3), 0x53);
        // Equal to the ends and NaN go to the interpolation.
        assert_eq!(at(&mut e, 1.0, 3), 0x53);
        assert_eq!(at(&mut e, 3.0, 3), 0x53);
        assert_eq!(at(&mut e, f32::NAN, 3), 0x53);
        // With a count of 2 the last key is the second.
        assert_eq!(at(&mut e, 2.5, 2), 0);
    }
}
