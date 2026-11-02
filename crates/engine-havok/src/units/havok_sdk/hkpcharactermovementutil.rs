//! `Havok SDK/hkpcharactermovementutil.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::x87::rsqrt_estimate;

/// `0.0`, `3.0` and `0.5` (`float`) in the exe image: the `RSQRTSS` refinement constants.
const ZERO: u32 = 0x0101_1d78;
const THREE: u32 = 0x0101_7718;
const HALF: u32 = 0x0101_6248;
/// A small `float` (`0104da4c`): below this squared length the cross product is ignored.
const EPSILON: u32 = 0x0104_da4c;
/// The CRT square root taking its argument in ST0 (`00ec6040`).
const X87_SQRT: u32 = 0x00ec_6040;
/// `(out, matrix, vector)` thiscall forwarder (`004b4ab0`, `RET 8`) that rotates `vector`
/// by the three rows at `matrix` into `out`, and the cdecl-style `00c7f700` (`RET 8`,
/// ECX = out, `(matrix, vector)`) that rotates the other way. Both are outside this unit.
const MATRIX_APPLY: u32 = 0x004b_4ab0;
const MATRIX_APPLY_TRANSPOSED: u32 = 0x00c7_f700;

fn lanes(e: &Engine, at: u32) -> [f32; 4] {
    [0, 1, 2, 3].map(|i| e.mem.f32(at + 4 * i))
}

fn set_lanes(e: &mut Engine, at: u32, value: [f32; 4]) {
    for (i, lane) in value.into_iter().enumerate() {
        e.mem.set_f32(at + 4 * i as u32, lane);
    }
}

/// `SHUFPS dst, src, imm`: lanes 0 and 1 come from `dst`, lanes 2 and 3 from `src`.
fn shufps(dst: [f32; 4], src: [f32; 4], imm: u32) -> [f32; 4] {
    [
        dst[(imm & 3) as usize],
        dst[((imm >> 2) & 3) as usize],
        src[((imm >> 4) & 3) as usize],
        src[((imm >> 6) & 3) as usize],
    ]
}

/// The `SHUFPS 0xC9 / 0xD2` cross product of `p` and `q` (`p` times `q`).
fn cross(p: [f32; 4], q: [f32; 4]) -> [f32; 4] {
    let a = shufps(p, p, 0xc9);
    let b = shufps(q, q, 0xd2);
    let c = shufps(p, p, 0xd2);
    let d = shufps(q, q, 0xc9);
    [0, 1, 2, 3].map(|i| a[i] * b[i] - c[i] * d[i])
}

/// `z*z + (y*y + x*x)`, the `MULPS` / `ADDSS` sum of squares.
fn sum_of_squares(v: [f32; 4]) -> f32 {
    let squares = v.map(|lane| lane * lane);
    squares[2] + (squares[1] + squares[0])
}

/// The `RSQRTSS` estimate refined once with the `CMPEQSS` / `ANDNPS` zero guard:
/// `(half * r) * (three - (sum * r) * r)`, `0.0` when `sum` is `zero`.
fn inverse_length(sum: f32, zero: f32, half: f32, three: f32) -> f32 {
    let estimate = rsqrt_estimate(sum);
    let step = three - (sum * estimate) * estimate;
    let value = (half * estimate) * step;
    if sum == zero {
        0.0
    } else {
        value
    }
}

// Translated from 00d6aef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCharacterMovementUtil::calculateMovement` (named by the exe's symbol table), cdecl
/// `(input, output)`. `input` holds a float at +0x00 (`speed`), vectors at +0x10, +0x20 and
/// +0x30 (the three that define a frame), +0x40 and +0x70, +0x50, and a float at +0x60 (the
/// speed limit). Builds an orthonormal frame from the cross products of the first two
/// vectors (the first normalised cross product is skipped, leaving `output` untouched, when
/// its squared length is below the epsilon at `0104da4c`), turns `input[+0x40] - input[+0x70]`
/// through it (`004b4ab0`), takes the difference to `input[+0x50]`, scales it by `speed` (or,
/// when `speed * speed * |difference|^2` exceeds `limit * limit`, to length `limit`),
/// adds the turned vector back, turns the sum the other way (`00c7f700`) into `output` and
/// adds `input[+0x70]`. The exe keeps the local matrix rows on the stack; they live in a
/// scratch block here. `speed` and `limit` are descriptive labels, not names from the PDB.
pub fn hkp_character_movement_util_calculate_movement(e: &mut Engine, input: Ptr, output: Ptr) {
    let s = input.addr();
    let epsilon = e.mem.f32(EPSILON);
    let (zero, three, half) = (e.mem.f32(ZERO), e.mem.f32(THREE), e.mem.f32(HALF));
    let first = lanes(e, s + 0x10);
    let second = lanes(e, s + 0x20);
    let cross_a = cross(first, second);
    let length_a = sum_of_squares(cross_a);
    if epsilon > length_a {
        return;
    }
    let third = lanes(e, s + 0x30);
    let scale_a = inverse_length(length_a, zero, half, three);
    let unit_a = cross_a.map(|lane| scale_a * lane);
    let cross_b = cross(unit_a, third);
    let scale_b = inverse_length(sum_of_squares(cross_b), zero, half, three);
    let unit_b = cross_b.map(|lane| scale_b * lane);
    let cross_c = cross(unit_b, third);
    let scale_c = inverse_length(sum_of_squares(cross_c), zero, half, three);
    let unit_c = cross_c.map(|lane| scale_c * lane);
    let target = lanes(e, s + 0x40);
    let origin = lanes(e, s + 0x70);
    e.with_stack(0x60, |e, local| {
        let b = local.addr();
        set_lanes(e, b + 0x30, unit_b);
        set_lanes(e, b + 0x40, unit_c);
        set_lanes(e, b + 0x50, third);
        let relative = [0, 1, 2, 3].map(|i| target[i] - origin[i]);
        set_lanes(e, b + 0x10, relative);
        e.call(MATRIX_APPLY, &args![b + 0x10, b + 0x30, b + 0x10]);
        let turned = lanes(e, b + 0x10);
        let desired = lanes(e, s + 0x50);
        let difference = [0, 1, 2, 3].map(|i| desired[i] - turned[i]);
        let length = sum_of_squares(difference);
        set_lanes(e, b + 0x20, difference);
        let speed = e.mem.f32(s);
        let limit = e.mem.f32(s + 0x60);
        let over = (f64::from(speed) * f64::from(length)) * f64::from(speed);
        let allowed = f64::from(limit) * f64::from(limit);
        let step = if over > allowed {
            let root = e.call(X87_SQRT, &args![f64::from(length)]).f32();
            let inverse = (1.0 / f64::from(root)) as f32;
            let factor = (f64::from(inverse) * f64::from(limit)) as f32;
            difference.map(|lane| factor * lane)
        } else {
            difference.map(|lane| speed * lane)
        };
        let moved = [0, 1, 2, 3].map(|i| step[i] + turned[i]);
        set_lanes(e, b + 0x10, moved);
        e.call(MATRIX_APPLY_TRANSPOSED, &args![output, b + 0x30, b + 0x10]);
    });
    let current = lanes(e, output.addr());
    set_lanes(
        e,
        output.addr(),
        [0, 1, 2, 3].map(|i| origin[i] + current[i]),
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00d6_aef0,
        hkp_character_movement_util_calculate_movement(Ptr, Ptr)
    )]
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// An engine with the exe's data range and the import page mapped, as the tests run without the exe.
    fn test_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    fn put(e: &mut Engine, at: u32, v: [f32; 4]) {
        set_lanes(e, at, v);
    }

    /// Doubles for the two matrix helpers (identity transforms that log their calls) and
    /// the CRT square root, plus the exe constants.
    fn rig() -> (Engine, Rc<RefCell<Vec<u32>>>) {
        let mut e = test_engine();
        e.mem.set_f32(EPSILON, 1.0e-6);
        e.mem.set_f32(ZERO, 0.0);
        e.mem.set_f32(THREE, 3.0);
        e.mem.set_f32(HALF, 0.5);
        let calls = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&calls);
        e.register_double(MATRIX_APPLY, move |e, a| {
            sink.borrow_mut().push(MATRIX_APPLY);
            let v = lanes(e, a[2]);
            set_lanes(e, a[0], v);
            Ret::default()
        });
        let sink = Rc::clone(&calls);
        e.register_double(MATRIX_APPLY_TRANSPOSED, move |e, a| {
            sink.borrow_mut().push(MATRIX_APPLY_TRANSPOSED);
            let v = lanes(e, a[2]);
            set_lanes(e, a[0], v);
            Ret::default()
        });
        e.register_double(X87_SQRT, |_, a| {
            let x = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            Ret {
                st0: x.sqrt(),
                ..Ret::default()
            }
        });
        (e, calls)
    }

    fn input(e: &mut Engine, speed: f32, limit: f32) -> u32 {
        let s = e.mem.alloc(0x80);
        e.mem.set_f32(s, speed);
        put(e, s + 0x10, [1.0, 0.0, 0.0, 0.0]);
        put(e, s + 0x20, [0.0, 1.0, 0.0, 0.0]);
        put(e, s + 0x30, [0.0, 0.0, 1.0, 0.0]);
        put(e, s + 0x40, [1.0, 2.0, 3.0, 0.0]);
        put(e, s + 0x50, [4.0, 2.0, 1.0, 0.0]);
        e.mem.set_f32(s + 0x60, limit);
        put(e, s + 0x70, [1.0, 1.0, 1.0, 0.0]);
        s
    }

    #[test]
    fn scales_the_difference_by_the_speed() {
        let (mut e, calls) = rig();
        let s = input(&mut e, 0.5, 100.0);
        let out = e.mem.alloc(0x10);
        e.call(0x00d6_aef0, &args![s, out]);
        assert_eq!(lanes(&e, out), [3.0, 2.5, 2.5, 0.0]);
        assert_eq!(*calls.borrow(), vec![MATRIX_APPLY, MATRIX_APPLY_TRANSPOSED]);
    }

    #[test]
    fn limits_the_step_to_the_limit_length() {
        let (mut e, _) = rig();
        let s = input(&mut e, 0.5, 1.0);
        let out = e.mem.alloc(0x10);
        e.call(0x00d6_aef0, &args![s, out]);
        let root = 18.0f64.sqrt();
        let inverse = (1.0 / f64::from(root as f32)) as f32;
        let expected = [4.0f32, 1.0, -1.0].map(|d| inverse * d);
        let got = lanes(&e, out);
        assert_eq!(got[0], 1.0 + (expected[0] + 0.0));
        assert_eq!(got[1], 1.0 + (expected[1] + 1.0));
        assert_eq!(got[2], 1.0 + (expected[2] + 2.0));
    }

    #[test]
    fn a_degenerate_frame_leaves_the_output_alone() {
        let (mut e, calls) = rig();
        let s = input(&mut e, 0.5, 100.0);
        put(&mut e, s + 0x20, [1.0, 0.0, 0.0, 0.0]);
        let out = e.mem.alloc(0x10);
        put(&mut e, out, [9.0, 9.0, 9.0, 9.0]);
        e.call(0x00d6_aef0, &args![s, out]);
        assert_eq!(lanes(&e, out), [9.0, 9.0, 9.0, 9.0]);
        assert!(calls.borrow().is_empty());
    }
}
