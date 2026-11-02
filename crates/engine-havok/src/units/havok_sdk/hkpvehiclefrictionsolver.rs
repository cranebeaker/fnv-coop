//! `Havok SDK/hkpvehiclefrictionsolver.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `pow` (LIBCMT, `00ec9b00`): the exe passes the base in `ST1` and the exponent in `ST0`; the
/// engine's replacement takes both as doubles.
const POW: u32 = 0x00ec_9b00;
/// The exponent the exe passes to `pow` for the first table entry: the double at `010ec9e0`
/// (`0x3fb1111120000000`, the float `1 / 15` widened).
const FIRST_EXPONENT_BITS: u64 = 0x3fb1_1111_2000_0000;
/// The double `0.0625` at `010b7e98`.
const SIXTEENTH: f64 = 0.0625;
/// Entries of each friction table the exe computes before the closing `1.0`.
const TABLE_ENTRIES: u32 = 15;

type Vec4 = [f32; 4];

fn read_vec4(e: &Engine, address: u32) -> Vec4 {
    std::array::from_fn(|i| f32::from_bits(e.mem.u32(address + 4 * i as u32)))
}

/// `FLD m32`: a float widened to the x87 stack.
fn load(e: &Engine, address: u32) -> f64 {
    f64::from(x87_quiet(e.mem.f32(address)))
}

/// `MULPS`.
fn mul(a: Vec4, b: Vec4) -> Vec4 {
    std::array::from_fn(|i| a[i] * b[i])
}

/// `ADDPS`.
fn add(a: Vec4, b: Vec4) -> Vec4 {
    std::array::from_fn(|i| a[i] + b[i])
}

/// `SUBPS`.
fn sub(a: Vec4, b: Vec4) -> Vec4 {
    std::array::from_fn(|i| a[i] - b[i])
}

/// The scalar sum the exe builds from a four-lane vector: `ADDSS` of lane 1 and lane 0, then
/// lane 2 plus that.
fn sum_xyz(a: Vec4) -> f32 {
    a[2] + (a[1] + a[0])
}

// Translated from 00e593f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkVehicleFrictionDescriptionInitValues` (Xbox PDB name; cdecl `(description, params)`). Both
/// are arrays of floats; the vectors of the description (four floats each) are at the offsets
/// named below and every `float` store rounds, the arithmetic in between being double
/// precision. With `d[n]` the description's float number `n` and `p[n]` the output's:
/// - `p[1] = d[0x24]`, `p[0x17] = d[0x27]`, `p[0x30] = d[0x28]` (copies through the x87);
///   `p[0x18] = 1 / d[0x27]`, `p[0x31] = 1 / d[0x28]`, `p[0x14] = d[0x25] * p[0x18]`,
///   `p[0x2d] = d[0x26] * p[0x31]`, `p[0x15] = 1 / p[0x14]`, `p[0x2e] = 1 / p[0x2d]`;
/// - with `a` = the dot product (`xyz`, four-lane products summed as lane 2 + (lane 1 +
///   lane 0)) of the vectors at `+0x40` and `+0x00` and `b` that of the vectors at `+0x40`
///   and `+0x10`: `p[0] = |a - b|`, `inv = 1 / p[0]`, `p[0x19] = p[0x32] = 1 - d[0x29]`,
///   `p[0x1a] = |-b * inv| * d[0x29]` and `p[0x33] = |a * inv| * d[0x29]`;
/// - for `i` in 0 and 1: `g = (-d[0x27 + i]) * vec(+0x50) + (vec(+0x10 * i) - vec(+0x20))`, `h` =
///   the cross product of `g` and the vector at `+0x30`, `s` = the sum (as above) of `h *
///   (h.x * vec(+0x60) + h.y * vec(+0x70) + h.z * vec(+0x80))`, and `t[i] = float(p[1] + s)`;
/// - for `j` in 0 and 1 (`base` = `p + 0x64 * j`): with `u = float(t[j] * p[0x14 + 0x19 j])`
///   (stored at `p[0x16 + 0x19 j]`), `v = float(u * 0.0625)` (at `p[0x13 + 0x19 j]`), `w =
///   float(1 / v)`, `x = float(pow(w, 0.0666666..))` (at `p[0x12 + 0x19 j]`), `y = float(1 / u)`:
///   fifteen entries `p[2 + 0x19 j + k] = float(1 - float(pow(float(1 - v_k), y)))` with
///   `v_0 = v` and `v_(k+1) = float(x * v_k)`, then `p[0x11 + 0x19 j] = 1.0`.
pub fn hk_vehicle_friction_description_init_values(e: &mut Engine, description: Ptr, params: Ptr) {
    let d = description.addr();
    let p = params.addr();
    for (from, to) in [(0x90, 0x04), (0x9c, 0x5c), (0xa0, 0xc0)] {
        let bits = x87_quiet_bits(e.mem.u32(d + from));
        e.mem.set_u32(p + to, bits);
    }
    e.mem.set_f32(p + 0x60, (1.0 / load(e, d + 0x9c)) as f32);
    e.mem.set_f32(p + 0xc4, (1.0 / load(e, d + 0xa0)) as f32);
    e.mem
        .set_f32(p + 0x50, (load(e, d + 0x94) * load(e, p + 0x60)) as f32);
    e.mem
        .set_f32(p + 0xb4, (load(e, d + 0x98) * load(e, p + 0xc4)) as f32);
    e.mem.set_f32(p + 0x54, (1.0 / load(e, p + 0x50)) as f32);
    e.mem.set_f32(p + 0xb8, (1.0 / load(e, p + 0xb4)) as f32);

    let first = read_vec4(e, d);
    let second = read_vec4(e, d + 0x10);
    let reference = read_vec4(e, d + 0x40);
    let a = sum_xyz(mul(reference, first));
    let b = sum_xyz(mul(second, reference));
    let difference = x87_quiet((f64::from(a) - f64::from(b)) as f32).abs();
    e.mem.set_f32(p, difference);
    let inverse = (1.0 / f64::from(difference)) as f32;
    let damping = load(e, d + 0xa4);
    e.mem.set_f32(p + 0x64, (1.0 - damping) as f32);
    let first_ratio = x87_quiet((-f64::from(b) * f64::from(inverse)) as f32).abs();
    e.mem
        .set_f32(p + 0x68, (f64::from(first_ratio) * damping) as f32);
    e.mem.set_f32(p + 0xc8, (1.0 - damping) as f32);
    let second_ratio = x87_quiet((f64::from(a) * f64::from(inverse)) as f32).abs();
    e.mem
        .set_f32(p + 0xcc, (f64::from(second_ratio) * damping) as f32);

    let cross_a = read_vec4(e, d + 0x30);
    let weight_axis = read_vec4(e, d + 0x50);
    let offset = read_vec4(e, d + 0x20);
    let cross_a_yzx = [cross_a[1], cross_a[2], cross_a[0], cross_a[3]];
    let cross_a_zxy = [cross_a[2], cross_a[0], cross_a[1], cross_a[3]];
    let rows = [
        read_vec4(e, d + 0x60),
        read_vec4(e, d + 0x70),
        read_vec4(e, d + 0x80),
    ];
    let mut totals = [0.0f32; 2];
    for (i, total) in totals.iter_mut().enumerate() {
        let i = i as u32;
        let vector = read_vec4(e, d + 0x10 * i);
        let negated = -x87_quiet(e.mem.f32(d + 0x9c + 4 * i));
        let g = add(mul([negated; 4], weight_axis), sub(vector, offset));
        let g_yzx = [g[1], g[2], g[0], g[3]];
        let g_zxy = [g[2], g[0], g[1], g[3]];
        let h = sub(mul(g_yzx, cross_a_zxy), mul(g_zxy, cross_a_yzx));
        let transformed = add(
            add(mul([h[0]; 4], rows[0]), mul([h[1]; 4], rows[1])),
            mul([h[2]; 4], rows[2]),
        );
        let s = sum_xyz(mul(transformed, h));
        *total = (load(e, p + 4) + f64::from(s)) as f32;
    }

    for (j, total) in totals.iter().enumerate() {
        let base = p + 0x64 * j as u32;
        let u = (f64::from(*total) * load(e, base + 0x50)) as f32;
        e.mem.set_f32(base + 0x58, u);
        let v = (f64::from(u) * SIXTEENTH) as f32;
        e.mem.set_f32(base + 0x4c, v);
        let w = (1.0 / f64::from(v)) as f32;
        let exponent = f64::from_bits(FIRST_EXPONENT_BITS);
        let x = e.call(POW, &args![f64::from(w), exponent]).f64() as f32;
        e.mem.set_f32(base + 0x48, x);
        let y = (1.0 / f64::from(u)) as f32;
        let mut v_k = v;
        for k in 0..TABLE_ENTRIES {
            let z = (1.0 - f64::from(v_k)) as f32;
            let q = e.call(POW, &args![f64::from(z), f64::from(y)]).f64() as f32;
            e.mem.set_f32(base + 8 + 4 * k, (1.0 - f64::from(q)) as f32);
            v_k = (f64::from(x) * f64::from(v_k)) as f32;
        }
        e.mem.set_f32(base + 0x44, 1.0);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00e5_93f0,
        hk_vehicle_friction_description_init_values(Ptr, Ptr)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::engine;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// A description with `a = 3`, `b = 1`, `d[0x24] = 1.5`, `d[0x25] = 2`, `d[0x26] = 4`,
    /// `d[0x27] = 4`, `d[0x28] = 8`, `d[0x29] = 0.5`; the cross vector is `(0, 1, 0, 0)` and
    /// the matrix row 2 `(0, 0, 2, 0)`, so `h` is `(0, 0, 3)` and `(0, 0, 1)` for the two
    /// vectors, `s` 18 and 2 and the totals 19.5 and 3.5.
    fn description(e: &mut Engine) -> u32 {
        let d = e.mem.alloc(0xb0);
        let set = |e: &mut Engine, index: u32, value: f32| e.mem.set_f32(d + 4 * index, value);
        set(e, 0, 3.0);
        set(e, 4, 1.0);
        set(e, 0x10, 1.0);
        set(e, 13, 1.0);
        set(e, 0x22, 2.0);
        set(e, 0x24, 1.5);
        set(e, 0x25, 2.0);
        set(e, 0x26, 4.0);
        set(e, 0x27, 4.0);
        set(e, 0x28, 8.0);
        set(e, 0x29, 0.5);
        d
    }

    /// Runs the function with `pow` replaced by a double that logs its arguments.
    fn run() -> (Engine, u32, Vec<(f64, f64)>) {
        let mut e = engine();
        let d = description(&mut e);
        let p = e.mem.alloc(0x100);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(POW, move |_, a| {
            let x = f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32));
            let y = f64::from_bits(u64::from(a[2]) | (u64::from(a[3]) << 32));
            sink.borrow_mut().push((x, y));
            Ret {
                st0: x.powf(y),
                ..Ret::default()
            }
        });
        e.call(0x00e5_93f0, &args![d, p]);
        let seen = seen.borrow().clone();
        (e, p, seen)
    }

    #[test]
    fn the_scalar_parameters_come_from_the_description() {
        let (e, p, _) = run();
        let f = |index: u32| e.mem.f32(p + 4 * index);
        assert_eq!(f(1), 1.5);
        assert_eq!([f(0x17), f(0x30)], [4.0, 8.0]);
        assert_eq!([f(0x18), f(0x31)], [0.25, 0.125]);
        assert_eq!([f(0x14), f(0x2d)], [0.5, 0.5]);
        assert_eq!([f(0x15), f(0x2e)], [2.0, 2.0]);
        // |3 - 1| = 2; 1 - 0.5; |-1 * 0.5| * 0.5; |3 * 0.5| * 0.5.
        assert_eq!(f(0), 2.0);
        assert_eq!([f(0x19), f(0x32)], [0.5, 0.5]);
        assert_eq!([f(0x1a), f(0x33)], [0.25, 0.75]);
    }

    #[test]
    fn each_table_has_fifteen_powers_and_a_closing_one() {
        let (e, p, seen) = run();
        let f = |index: u32| e.mem.f32(p + 4 * index);
        for (j, (total, entry)) in [(19.5f64, 0.5f64), (3.5, 0.5)].iter().enumerate() {
            let at = 0x19 * j as u32;
            let u = (total * entry) as f32;
            let v = (f64::from(u) * 0.0625) as f32;
            assert_eq!(f(0x16 + at), u);
            assert_eq!(f(0x13 + at), v);
            let x = f(0x12 + at);
            let expected_x = (1.0 / f64::from(v)).powf(f64::from_bits(FIRST_EXPONENT_BITS)) as f32;
            assert!((x - expected_x).abs() <= expected_x * 1e-6);
            let y = 1.0 / f64::from(u);
            let mut v_k = f64::from(v);
            for k in 0..15 {
                let expected = 1.0 - (1.0 - v_k).powf(y);
                let got = f64::from(f(2 + at + k));
                assert!(
                    (got - expected).abs() < 1e-5,
                    "table {j} entry {k}: {got} vs {expected}"
                );
                v_k *= f64::from(x);
            }
            assert_eq!(f(0x11 + at), 1.0);
        }
        // Thirty-two `pow` calls: two tables of one start exponent and fifteen entries.
        assert_eq!(seen.len(), 32);
        assert_eq!(seen[0].1, f64::from_bits(FIRST_EXPONENT_BITS));
    }
}
