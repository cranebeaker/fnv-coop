//! `fallout shared/pathfinding/run_006dc9f0` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0.0` (`double`).
const ZERO: u32 = 0x0101_2060;
/// The zero point (`011f426c`, three words).
const POINT_011F426C: u32 = 0x011f_426c;
/// `0044ddc0` (`thiscall`): the element count of the array at `this`.
const COUNT: u32 = 0x0044_ddc0;
/// `006a1400` (`thiscall`, `RET 4`: index): the address of an element.
const ELEMENT: u32 = 0x006a_1400;
/// `00439ef0` (`thiscall`, `RET 8`: out, other): `this - other` into `out`.
const POINT_SUBTRACT: u32 = 0x0043_9ef0;
/// `00439e90` (`thiscall`, `RET 8`: out, other): `this + other` into `out`.
const POINT_ADD: u32 = 0x0043_9e90;
/// `NiPoint3::Dot` (engine map, `004b6190`; `thiscall`, `RET 4`, ST0).
const POINT_DOT: u32 = 0x004b_6190;
/// `004a7290` (`thiscall`, ST0): a point's squared length.
const POINT_LENGTH_SQUARED: u32 = 0x004a_7290;
/// `NiPoint3::UnitizeGetLength` (engine map, `00457910`; `thiscall`, ST0).
const POINT_UNITIZE: u32 = 0x0045_7910;
/// `0045bb20` (`thiscall`, `RET 8`: out, a float): `this * scale` into
/// `out`; returns `out`.
const POINT_SCALED: u32 = 0x0045_bb20;
/// `0053d280` (`thiscall`, `RET 8`: out, a float): `this / value` into
/// `out`; returns `out`.
const POINT_DIVIDED: u32 = 0x0053_d280;
/// `0063c8a0` (`thiscall`, `RET 4`): `this += other`.
const POINT_ADD_IN_PLACE: u32 = 0x0063_c8a0;
/// `004941c0` (`thiscall`, `RET 4`: a float): `this /= value`.
const POINT_DIVIDE_IN_PLACE: u32 = 0x0049_41c0;

// Translated from 006dc9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The weighted mean of the elements' points nearest to `point`, into
/// `out`, for the array at `this` (`thiscall`; point, out; `RET 8`; no
/// name in the engine map); returns the sum of the elements' floats at
/// `+0x18` (in ST0). Each element (`006a1400`) gives a point: its first
/// three words, or, when its word at `+0x20` is 1 (a segment from `+0` to
/// `+0xc`), the start when the projection of `point - start` on the
/// direction is below 0, the end when it is above the squared length,
/// else `start + unit direction * (projection / length)`
/// (`UnitizeGetLength`, `0045bb20`, `0053d280`, `00439e90`). `out` (first
/// the zero point at `011f426c`) gets each point times the element's
/// weight at `+0x1c` added (`0045bb20`, `0063c8a0`), then is divided by the
/// sum of the weights (`004941c0`) when the array is not empty.
pub fn fn_006dc9f0(e: &mut Engine, this: Ptr, point: Ptr, out: Ptr) -> f32 {
    let (point, out) = (point.addr(), out.addr());
    e.with_stack(0x78, |e, frame| {
        let l = |x: u32| frame.addr() + 0x78 - x;
        let ld = |e: &Engine, address: u32| x87_load32(e.mem.u32(address));
        let m32 = |e: &Engine, address: u32| x87_m32(e.mem.u32(address));
        let pushed = |e: &Engine, address: u32| x87_quiet_bits(e.mem.u32(address));
        let (total, weights) = (l(0x4), l(0x8));
        e.mem.set_u32(total, x87_store32(0.0));
        for offset in [0, 4, 8] {
            let word = e.global::<u32>(POINT_011F426C + offset);
            e.mem.set_u32(out + offset, word);
        }
        e.mem.set_u32(weights, x87_store32(0.0));
        let count = e.call(COUNT, &args![this]).u32();
        if count > 0 {
            let chosen = l(0x1c);
            let weight = l(0x24);
            let mut index = 0u32;
            while index < count {
                let element = e.call(ELEMENT, &args![this, index]).u32();
                let value = pushed(e, element + 0x1c);
                e.mem.set_u32(weight, value);
                for offset in [0, 4, 8] {
                    let word = e.mem.u32(element + offset);
                    e.mem.set_u32(chosen + offset, word);
                }
                if e.mem.u32(element + 0x20) == 1 {
                    let (direction, to_point) = (l(0x40), l(0x34));
                    e.call(POINT_SUBTRACT, &args![element + 0xc, direction, element]);
                    e.call(POINT_SUBTRACT, &args![point, to_point, element]);
                    let dot = e.call(POINT_DOT, &args![to_point, direction]).f64();
                    e.mem.set_u32(l(0x28), x87_store32(dot));
                    let zero: f64 = e.global(ZERO);
                    // `TEST AH,0x5; JP`: on when not below 0 (NaN included).
                    if ld(e, l(0x28)) < zero {
                        for offset in [0, 4, 8] {
                            let word = e.mem.u32(element + offset);
                            e.mem.set_u32(chosen + offset, word);
                        }
                    } else {
                        let squared = e.call(POINT_LENGTH_SQUARED, &args![direction]).f64();
                        // `TEST AH,0x41; JNZ`: on when not above (NaN
                        // included).
                        if ld(e, l(0x28)) > squared {
                            for offset in [0, 4, 8] {
                                let word = e.mem.u32(element + 0xc + offset);
                                e.mem.set_u32(chosen + offset, word);
                            }
                        } else {
                            let length = e.call(POINT_UNITIZE, &args![direction]).f64();
                            e.mem.set_u32(l(0x44), x87_store32(length));
                            let length = pushed(e, l(0x44));
                            let scale = pushed(e, l(0x28));
                            let scaled = e
                                .call(POINT_SCALED, &args![direction, l(0x50), scale])
                                .u32();
                            let divided =
                                e.call(POINT_DIVIDED, &args![scaled, l(0x5c), length]).u32();
                            let sum = e.call(POINT_ADD, &args![element, l(0x68), divided]).u32();
                            for offset in [0, 4, 8] {
                                let word = e.mem.u32(sum + offset);
                                e.mem.set_u32(chosen + offset, word);
                            }
                        }
                    }
                }
                let scale = pushed(e, weight);
                let weighted = e.call(POINT_SCALED, &args![chosen, l(0x74), scale]).u32();
                e.call(POINT_ADD_IN_PLACE, &args![out, weighted]);
                let value = x87_store32(x87_add(ld(e, total), m32(e, element + 0x18)));
                e.mem.set_u32(total, value);
                let value = x87_store32(x87_add(ld(e, weights), m32(e, weight)));
                e.mem.set_u32(weights, value);
                index += 1;
            }
            let divisor = pushed(e, weights);
            e.call(POINT_DIVIDE_IN_PLACE, &args![out, divisor]);
        }
        f32::from_bits(x87_quiet_bits(e.mem.u32(total)))
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006d_c9f0, fn_006dc9f0(Ptr, Ptr, Ptr) -> f32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read(e: &Engine, p: u32) -> [f32; 3] {
        [e.mem.f32(p), e.mem.f32(p + 4), e.mem.f32(p + 8)]
    }

    fn write(e: &mut Engine, p: u32, v: [f32; 3]) {
        for (i, x) in v.iter().enumerate() {
            e.mem.set_f32(p + 4 * i as u32, *x);
        }
    }

    fn st0(value: f32) -> Ret {
        Ret {
            st0: f64::from(value),
            ..Ret::default()
        }
    }

    /// Point doubles over plain floats, and an array whose elements are
    /// the given 0x24-byte records.
    fn mean_engine(elements: &[([f32; 3], [f32; 3], f32, f32, u32)]) -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(ZERO & !0xfff, 0x1000);
        e.set_global(ZERO, 0.0f64);
        e.map(POINT_011F426C & !0xfff, 0x1000);
        e.register(POINT_SUBTRACT, |e, a| {
            let (p, q) = (read(e, a[0]), read(e, a[2]));
            write(e, a[1], [p[0] - q[0], p[1] - q[1], p[2] - q[2]]);
            a[1].into_ret()
        });
        e.register(POINT_ADD, |e, a| {
            let (p, q) = (read(e, a[0]), read(e, a[2]));
            write(e, a[1], [p[0] + q[0], p[1] + q[1], p[2] + q[2]]);
            a[1].into_ret()
        });
        e.register(POINT_DOT, |e, a| {
            let (p, q) = (read(e, a[0]), read(e, a[1]));
            st0(p[0] * q[0] + p[1] * q[1] + p[2] * q[2])
        });
        e.register(POINT_LENGTH_SQUARED, |e, a| {
            let p = read(e, a[0]);
            st0(p[0] * p[0] + p[1] * p[1] + p[2] * p[2])
        });
        e.register(POINT_UNITIZE, |e, a| {
            let p = read(e, a[0]);
            let length = (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt();
            write(e, a[0], [p[0] / length, p[1] / length, p[2] / length]);
            st0(length)
        });
        e.register(POINT_SCALED, |e, a| {
            let (p, s) = (read(e, a[0]), f32::from_bits(a[2]));
            write(e, a[1], [p[0] * s, p[1] * s, p[2] * s]);
            a[1].into_ret()
        });
        e.register(POINT_DIVIDED, |e, a| {
            let (p, s) = (read(e, a[0]), f32::from_bits(a[2]));
            write(e, a[1], [p[0] / s, p[1] / s, p[2] / s]);
            a[1].into_ret()
        });
        e.register(POINT_ADD_IN_PLACE, |e, a| {
            let (p, q) = (read(e, a[0]), read(e, a[1]));
            write(e, a[0], [p[0] + q[0], p[1] + q[1], p[2] + q[2]]);
            a[0].into_ret()
        });
        e.register(POINT_DIVIDE_IN_PLACE, |e, a| {
            let (p, s) = (read(e, a[0]), f32::from_bits(a[1]));
            write(e, a[0], [p[0] / s, p[1] / s, p[2] / s]);
            a[0].into_ret()
        });
        let records: Vec<u32> = elements
            .iter()
            .map(|(start, end, total, weight, kind)| {
                let record = e.mem.alloc(0x24);
                write(&mut e, record, *start);
                write(&mut e, record + 0xc, *end);
                e.mem.set_f32(record + 0x18, *total);
                e.mem.set_f32(record + 0x1c, *weight);
                e.mem.set_u32(record + 0x20, *kind);
                record
            })
            .collect();
        let count = records.len() as u32;
        e.register_double(COUNT, move |_, _| count.into_ret());
        e.register_double(ELEMENT, move |_, a| records[a[1] as usize].into_ret());
        let this = e.mem.alloc(4);
        (e, this)
    }

    #[test]
    fn fn_006dc9f0_weights_the_nearest_points() {
        // A plain point (kind 0), weight 1; a segment from (0,0,0) to
        // (10,0,0) with the point at (4, 3, 0): nearest (4,0,0), weight 3.
        let (mut e, this) = mean_engine(&[
            ([8.0, 8.0, 0.0], [99.0; 3], 0.5, 1.0, 0),
            ([0.0; 3], [10.0, 0.0, 0.0], 0.25, 3.0, 1),
        ]);
        let point = e.mem.alloc(0xc);
        write(&mut e, point, [4.0, 3.0, 0.0]);
        let out = e.mem.alloc(0xc);
        let total = e.call(0x006d_c9f0, &args![this, point, out]).f32();
        assert_eq!(total, 0.75);
        // ((8,8,0) * 1 + (4,0,0) * 3) / 4.
        assert_eq!(read(&e, out), [5.0, 2.0, 0.0]);
    }

    #[test]
    fn fn_006dc9f0_clamps_to_the_segment_ends() {
        for (point, nearest) in [
            ([-5.0f32, 1.0, 0.0], [0.0f32, 0.0, 0.0]),
            ([15.0, 1.0, 0.0], [10.0, 0.0, 0.0]),
        ] {
            let (mut e, this) = mean_engine(&[([0.0; 3], [10.0, 0.0, 0.0], 2.0, 2.0, 1)]);
            let at = e.mem.alloc(0xc);
            write(&mut e, at, point);
            let out = e.mem.alloc(0xc);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x006d_c9f0, &args![this, at, out]).f32(), 2.0);
            assert_eq!(read(&e, out), nearest);
            let log = e.call_log.take().unwrap();
            assert!(!log.iter().any(|(a, _)| *a == POINT_UNITIZE));
        }
    }

    #[test]
    fn fn_006dc9f0_empty_array_gives_the_zero_point() {
        let (mut e, this) = mean_engine(&[]);
        let point = e.mem.alloc(0xc);
        let out = e.mem.alloc(0xc);
        write(&mut e, out, [1.0, 2.0, 3.0]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006d_c9f0, &args![this, point, out]).f32(), 0.0);
        assert_eq!(read(&e, out), [0.0; 3]);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|(a, _)| *a == POINT_DIVIDE_IN_PLACE));
    }
}
