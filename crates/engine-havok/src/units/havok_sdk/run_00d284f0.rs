//! `Havok SDK/run_00d284f0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d284f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (input in a register, then one stack word; cdecl `RET`, result in `ST0`; the
/// register is `EAX`, which the exe's callers set, here the first argument `base`): with `a` the
/// first three lanes of the vector at `base + 0xd0` and `b` those at `base + 0xe0`, answers
/// `float(x * x * (bz*bz + (by*by + bx*bx)) + (az*az + (ay*ay + ax*ax)))` where `x` is the stack
/// float (`x * x` and the product in x87 double precision, each lane sum in `SSE` single
/// precision the way the `SHUFPS`/`ADDSS` pairs order it).
pub fn fn_00d284f0(e: &mut Engine, base: Ptr, x: f32) -> f32 {
    let square_sum = |e: &Engine, at: u32| {
        let squares = [0u32, 1, 2].map(|lane| {
            let value = e.mem.f32(at + lane * 4);
            value * value
        });
        squares[2] + (squares[1] + squares[0])
    };
    let loaded = x87_load32(x.to_bits());
    let squared = x87_mul(loaded, loaded);
    let first = square_sum(e, base.addr() + 0xd0);
    let second = square_sum(e, base.addr() + 0xe0);
    let product = x87_mul(squared, x87_m32(second.to_bits()));
    let total = x87_add(product, x87_m32(first.to_bits()));
    f32::from_bits(x87_quiet_bits(x87_store32(total)))
}

/// `0.25` (`double`) and `0.010000001` (`double`): the constants the check multiplies and
/// subtracts.
const CHECK_SCALE: u32 = 0x0102_90b0;
const CHECK_OFFSET: u32 = 0x010c_e490;

/// The sum of the squares of the first three lanes at `address`, in `float` lanes: the
/// `MULPS` / `SHUFPS 0x55` / `0xaa` / `ADDSS` ladder gives `l2 + (l1 + l0)`.
fn lane_square_sum(e: &Engine, address: u32) -> f32 {
    let squares = [0u32, 1, 2].map(|lane| {
        let value = e.mem.f32(address + lane * 4);
        value * value
    });
    squares[2] + (squares[1] + squares[0])
}

// Translated from 00d28560 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (class unconfirmed). Cdecl `(_unused_0, entities, count, offset)` answering a
/// byte; the first stack word is never read. For each of `count` pointers in `entities` (the
/// object plus `offset` is a motion block): the float at +0xb0 is limited to 1.0 (a NaN stays),
/// `v * v * (sum of squares of the lanes at +0xe0) + (sum of squares of the lanes at +0xd0)` is
/// stored as a float, scaled by 0.25 and reduced by 0.010000001 (x87 double arithmetic, stored as
/// a float), and compared with the float at +0x10c when `(byte at +0xc & 0x7f) >=
/// (byte at +0xa & 0x7f)` or at +0xfc otherwise: when that float is strictly less (ordered) the
/// answer is false at once. True when none is, and for `count <= 0`.
pub fn fn_00d28560(e: &mut Engine, _unused_0: u32, entities: Ptr, count: i32, offset: u32) -> bool {
    let scale: f64 = e.global(CHECK_SCALE);
    let reduction: f64 = e.global(CHECK_OFFSET);
    let mut remaining = count.wrapping_sub(1);
    let mut cursor = entities.addr();
    while remaining >= 0 {
        let motion = e.mem.u32(cursor).wrapping_add(offset);
        // `FLD` / `FSTP` / `FLD` of the float at +0xb0, then the comparison with 1.0.
        let stored = x87_store32(x87_load32(e.mem.u32(motion + 0xb0)));
        let loaded = x87_load32(stored);
        let limited = if loaded > 1.0 {
            1.0f32.to_bits()
        } else {
            stored
        };
        let sum_second = lane_square_sum(e, motion + 0xe0);
        let sum_first = lane_square_sum(e, motion + 0xd0);
        let square = x87_mul(x87_load32(limited), x87_load32(limited));
        let product = x87_mul(square, x87_m32(sum_second.to_bits()));
        let total = x87_add(product, x87_m32(sum_first.to_bits()));
        let energy = x87_load32(x87_store32(total));
        let reduced = x87_sub(x87_mul(energy, scale), reduction);
        let level = x87_load32(x87_store32(reduced));
        let first_flags = e.mem.u8(motion + 0xa) & 0x7f;
        let second_flags = e.mem.u8(motion + 0xc) & 0x7f;
        let threshold_at = if first_flags <= second_flags {
            motion + 0x10c
        } else {
            motion + 0xfc
        };
        let threshold = x87_load32(e.mem.u32(threshold_at));
        // `FCOMPP` then `TEST AH,5` / `JNP`: only an ordered "threshold below level" leaves.
        if threshold < level {
            return false;
        }
        cursor = cursor.wrapping_add(4);
        remaining = remaining.wrapping_sub(1);
    }
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d2_84f0, fn_00d284f0(Ptr, f32) -> f32),
        entry!(0x00d2_8560, fn_00d28560(u32, Ptr, i32, u32) -> bool),
    ]
}

#[cfg(test)]
mod havok_a04_tests {
    use super::*;

    #[test]
    fn fn_00d284f0_adds_the_scaled_square_lengths() {
        let mut e = Engine::new();
        let base = e.mem.alloc(0x100);
        for (lane, value) in [1.0f32, 2.0, 2.0, 9.0].into_iter().enumerate() {
            e.mem.set_f32(base + 0xd0 + lane as u32 * 4, value);
        }
        for (lane, value) in [3.0f32, 4.0, 0.0, 5.0].into_iter().enumerate() {
            e.mem.set_f32(base + 0xe0 + lane as u32 * 4, value);
        }
        // 2 * 2 * 25 + 9
        assert_eq!(e.call(0x00d2_84f0, &args![base, 2.0f32]).f32(), 109.0);
        // x = 0 leaves the first length alone.
        assert_eq!(e.call(0x00d2_84f0, &args![base, 0.0f32]).f32(), 9.0);
    }

    #[test]
    fn fn_00d284f0_keeps_a_nan_from_the_stack_float() {
        let mut e = Engine::new();
        let base = e.mem.alloc(0x100);
        assert!(e.call(0x00d2_84f0, &args![base, f32::NAN]).f32().is_nan());
    }
}

#[cfg(test)]
mod physics_check_tests {
    use super::*;

    /// A motion block at `offset` 0 holding the given float at +0xb0, the lanes at +0xd0 and
    /// +0xe0 and the two thresholds.
    fn motion(
        e: &mut Engine,
        limit: f32,
        first: [f32; 3],
        second: [f32; 3],
        thresholds: (f32, f32),
    ) -> u32 {
        let block = e.mem.alloc(0x120);
        e.mem.set_f32(block + 0xb0, limit);
        for (lane, value) in first.into_iter().enumerate() {
            e.mem.set_f32(block + 0xd0 + 4 * lane as u32, value);
        }
        for (lane, value) in second.into_iter().enumerate() {
            e.mem.set_f32(block + 0xe0 + 4 * lane as u32, value);
        }
        e.mem.set_f32(block + 0xfc, thresholds.0);
        e.mem.set_f32(block + 0x10c, thresholds.1);
        block
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0102_9000, 0x1000);
        e.map(0x010c_e000, 0x1000);
        e.set_global(CHECK_SCALE, 0.25f64);
        e.set_global(CHECK_OFFSET, 0.010000001f64);
        e
    }

    fn run(e: &mut Engine, blocks: &[u32]) -> bool {
        let array = e.mem.alloc(4 * blocks.len() as u32 + 4);
        for (i, block) in blocks.iter().enumerate() {
            e.mem.set_u32(array + 4 * i as u32, *block);
        }
        e.call(0x00d2_8560, &args![0u32, array, blocks.len() as u32, 0u32])
            .bool()
    }

    #[test]
    fn fn_00d28560_is_true_for_an_empty_array_and_quiet_motions() {
        let mut e = engine();
        assert!(run(&mut e, &[]));
        // Level (1 * 1 * 4 + 4) * 0.25 - 0.01 = 1.99; threshold 2.0 at +0x10c: not below.
        let quiet = motion(&mut e, 1.0, [2.0, 0.0, 0.0], [2.0, 0.0, 0.0], (0.0, 2.0));
        assert!(run(&mut e, &[quiet]));
    }

    #[test]
    fn fn_00d28560_is_false_when_a_threshold_is_below_the_level() {
        let mut e = engine();
        let quiet = motion(&mut e, 1.0, [2.0, 0.0, 0.0], [2.0, 0.0, 0.0], (0.0, 2.0));
        let busy = motion(&mut e, 1.0, [2.0, 0.0, 0.0], [2.0, 0.0, 0.0], (0.0, 1.5));
        assert!(!run(&mut e, &[quiet, busy]));
        // The float at +0xb0 is limited to 1.0: 5.0 gives the same level.
        let limited = motion(&mut e, 5.0, [2.0, 0.0, 0.0], [2.0, 0.0, 0.0], (0.0, 2.0));
        assert!(run(&mut e, &[limited]));
        // The threshold at +0xfc is the one used when byte +0xa (masked) is above byte +0xc.
        e.mem.set_u8(limited + 0xa, 0x85);
        e.mem.set_u8(limited + 0xc, 0x03);
        assert!(!run(&mut e, &[limited]));
        e.mem.set_f32(limited + 0xfc, 5.0);
        assert!(run(&mut e, &[limited]));
    }

    #[test]
    fn fn_00d28560_treats_nan_as_not_below() {
        let mut e = engine();
        let nan = motion(
            &mut e,
            f32::NAN,
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            (f32::NAN, f32::NAN),
        );
        assert!(run(&mut e, &[nan]));
    }
}
