//! `Havok SDK/run_00d22160` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

type Lanes = [f32; 4];

fn lanes_00d22160(e: &Engine, at: u32) -> Lanes {
    [0u32, 1, 2, 3].map(|lane| e.mem.f32(at.wrapping_add(lane * 4)))
}

fn store_lanes_00d22160(e: &mut Engine, at: u32, value: Lanes) {
    for (lane, word) in value.into_iter().enumerate() {
        e.mem
            .set_u32(at.wrapping_add(lane as u32 * 4), word.to_bits());
    }
}

fn mul_00d22160(a: Lanes, b: Lanes) -> Lanes {
    [0usize, 1, 2, 3].map(|lane| a[lane] * b[lane])
}

fn add_00d22160(a: Lanes, b: Lanes) -> Lanes {
    [0usize, 1, 2, 3].map(|lane| a[lane] + b[lane])
}

fn sub_00d22160(a: Lanes, b: Lanes) -> Lanes {
    [0usize, 1, 2, 3].map(|lane| a[lane] - b[lane])
}

/// `SHUFPS v, v, 0x55` / `0x00` / `0xaa`: the lane repeated.
fn splat_00d22160(v: Lanes, lane: usize) -> Lanes {
    [v[lane]; 4]
}

/// `SHUFPS v, v, 0xc9`: lanes `(y, z, x, w)`.
fn yzx_00d22160(v: Lanes) -> Lanes {
    [v[1], v[2], v[0], v[3]]
}

/// `SHUFPS v, v, 0xd2`: lanes `(z, x, y, w)`.
fn zxy_00d22160(v: Lanes) -> Lanes {
    [v[2], v[0], v[1], v[3]]
}

/// `a x b` as the exe orders it: `a.yzx * b.zxy - a.zxy * b.yzx`.
fn cross_00d22160(a: Lanes, b: Lanes) -> Lanes {
    sub_00d22160(
        mul_00d22160(yzx_00d22160(a), zxy_00d22160(b)),
        mul_00d22160(zxy_00d22160(a), yzx_00d22160(b)),
    )
}

/// The three rows of the matrix whose address is the word at `body + 0x50`.
fn matrix_rows_00d22160(e: &Engine, body: u32) -> [Lanes; 3] {
    let matrix = e.mem.u32(body.wrapping_add(0x50));
    [0u32, 0x10, 0x20].map(|offset| lanes_00d22160(e, matrix.wrapping_add(offset)))
}

/// `vector * matrix` as `(y * row1 + x * row0) + z * row2`, the order the `MULPS`/`ADDPS`
/// sequence adds in.
fn times_matrix_00d22160(vector: Lanes, rows: [Lanes; 3]) -> Lanes {
    let by_y = mul_00d22160(splat_00d22160(vector, 1), rows[1]);
    let by_x = mul_00d22160(splat_00d22160(vector, 0), rows[0]);
    let by_z = mul_00d22160(splat_00d22160(vector, 2), rows[2]);
    add_00d22160(add_00d22160(by_y, by_x), by_z)
}

// Translated from 00d22160 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (thiscall, `RET 0x14`: five stack words, the first never read; the roles are
/// descriptions, not confirmed): with `p` the vector at `this + 0x20` and `q` the vector at
/// `this + 0x30`, `r1 = p * M(body_a)` and `r2 = q * M(body_b)` (`M(b)` is the matrix of three
/// rows at the address in the word at `b + 0x50`, `vector * matrix` summed as `(y * row1 +
/// x * row0) + z * row2`). Subtracts: `r1` from the vector at `body_a`, `r2` from the one at
/// `body_b`, `r1 x (pair_a + 0x10)` from the vector at `pair_a` and `r2 x (pair_b + 0x10)` from
/// the one at `pair_b` (`a x b` as `a.yzx * b.zxy - a.zxy * b.yzx`, four lanes).
pub fn fn_00d22160(
    e: &mut Engine,
    this: Ptr,
    _unused_1: u32,
    body_a: Ptr,
    pair_a: Ptr,
    body_b: Ptr,
    pair_b: Ptr,
) {
    let base = this.addr();
    let p = lanes_00d22160(e, base + 0x20);
    let rows_a = matrix_rows_00d22160(e, body_a.addr());
    let r1 = times_matrix_00d22160(p, rows_a);
    let q = lanes_00d22160(e, base + 0x30);
    let rows_b = matrix_rows_00d22160(e, body_b.addr());
    let moved_a = sub_00d22160(lanes_00d22160(e, body_a.addr()), r1);
    store_lanes_00d22160(e, body_a.addr(), moved_a);
    let r2 = times_matrix_00d22160(q, rows_b);
    let moved_b = sub_00d22160(lanes_00d22160(e, body_b.addr()), r2);
    store_lanes_00d22160(e, body_b.addr(), moved_b);
    let first_arm = lanes_00d22160(e, pair_a.addr() + 0x10);
    let second_arm = lanes_00d22160(e, pair_b.addr() + 0x10);
    let torque_a = cross_00d22160(r1, first_arm);
    let torque_b = cross_00d22160(r2, second_arm);
    let value_a = sub_00d22160(lanes_00d22160(e, pair_a.addr()), torque_a);
    store_lanes_00d22160(e, pair_a.addr(), value_a);
    let value_b = sub_00d22160(lanes_00d22160(e, pair_b.addr()), torque_b);
    store_lanes_00d22160(e, pair_b.addr(), value_b);
}

// Translated from 00d22260 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (thiscall, `RET 0x18`: six stack words, the first two never read; the roles are
/// descriptions, not confirmed): the same products `r1 = p * M(body_a)` and `r2 = q * M(body_b)`
/// as `00d22160`, but adds instead of subtracts and leaves the bodies' vectors alone: the vector
/// at `pair_a` becomes `r1 x (pair_a + 0x10) + itself` and the one at `pair_b`
/// `r2 x (pair_b + 0x10) + itself`.
pub fn fn_00d22260(
    e: &mut Engine,
    this: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    body_a: Ptr,
    pair_a: Ptr,
    body_b: Ptr,
    pair_b: Ptr,
) {
    let base = this.addr();
    let p = lanes_00d22160(e, base + 0x20);
    let rows_a = matrix_rows_00d22160(e, body_a.addr());
    let rows_b = matrix_rows_00d22160(e, body_b.addr());
    let r1 = times_matrix_00d22160(p, rows_a);
    let q = lanes_00d22160(e, base + 0x30);
    let r2 = times_matrix_00d22160(q, rows_b);
    let first_arm = lanes_00d22160(e, pair_a.addr() + 0x10);
    let second_arm = lanes_00d22160(e, pair_b.addr() + 0x10);
    let torque_a = cross_00d22160(r1, first_arm);
    let torque_b = cross_00d22160(r2, second_arm);
    let value_a = add_00d22160(torque_a, lanes_00d22160(e, pair_a.addr()));
    store_lanes_00d22160(e, pair_a.addr(), value_a);
    let value_b = add_00d22160(torque_b, lanes_00d22160(e, pair_b.addr()));
    store_lanes_00d22160(e, pair_b.addr(), value_b);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d2_2160, fn_00d22160(Ptr, u32, Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00d2_2260, fn_00d22260(Ptr, u32, u32, Ptr, Ptr, Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod havok_a04_tests {
    use super::*;

    struct Scene {
        this: Ptr,
        body_a: Ptr,
        pair_a: Ptr,
        body_b: Ptr,
        pair_b: Ptr,
    }

    fn put(e: &mut Engine, at: u32, v: [f32; 4]) {
        store_lanes_00d22160(e, at, v);
    }

    /// `p = (1, 2, 3)`, `q = (0, 1, 0)`; matrix A rows `(1 2 3) (4 5 6) (7 8 10)`, matrix B
    /// `diag(1, 2, 3)`; so `r1 = (30, 36, 45)` and `r2 = (0, 2, 0)`.
    fn scene(e: &mut Engine) -> Scene {
        let this = e.mem.alloc(0x40);
        put(e, this + 0x20, [1.0, 2.0, 3.0, 0.0]);
        put(e, this + 0x30, [0.0, 1.0, 0.0, 0.0]);
        let matrix_a = e.mem.alloc(0x30);
        for (row, v) in [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 10.0]]
            .iter()
            .enumerate()
        {
            put(e, matrix_a + row as u32 * 0x10, [v[0], v[1], v[2], 0.0]);
        }
        let matrix_b = e.mem.alloc(0x30);
        for (row, v) in [[1.0, 0.0, 0.0], [0.0, 2.0, 0.0], [0.0, 0.0, 3.0]]
            .iter()
            .enumerate()
        {
            put(e, matrix_b + row as u32 * 0x10, [v[0], v[1], v[2], 0.0]);
        }
        let body_a = e.mem.alloc(0x60);
        e.mem.set_u32(body_a + 0x50, matrix_a);
        put(e, body_a, [100.0, 100.0, 100.0, 0.0]);
        let body_b = e.mem.alloc(0x60);
        e.mem.set_u32(body_b + 0x50, matrix_b);
        put(e, body_b, [10.0, 10.0, 10.0, 0.0]);
        let pair_a = e.mem.alloc(0x20);
        put(e, pair_a, [50.0, 50.0, 50.0, 0.0]);
        put(e, pair_a + 0x10, [1.0, 0.0, 0.0, 0.0]);
        let pair_b = e.mem.alloc(0x20);
        put(e, pair_b, [9.0, 9.0, 9.0, 0.0]);
        put(e, pair_b + 0x10, [0.0, 0.0, 1.0, 0.0]);
        Scene {
            this: Ptr::new(this),
            body_a: Ptr::new(body_a),
            pair_a: Ptr::new(pair_a),
            body_b: Ptr::new(body_b),
            pair_b: Ptr::new(pair_b),
        }
    }

    fn at(e: &Engine, p: Ptr) -> Lanes {
        lanes_00d22160(e, p.addr())
    }

    #[test]
    fn fn_00d22160_subtracts_the_products_and_the_torques() {
        let mut e = Engine::new();
        let s = scene(&mut e);
        e.call(
            0x00d2_2160,
            &args![s.this, 0u32, s.body_a, s.pair_a, s.body_b, s.pair_b],
        );
        // r1 = (30, 36, 45), r2 = (0, 2, 0); r1 x (1, 0, 0) = (0, 45, -36); r2 x (0, 0, 1) = (2, 0, 0).
        assert_eq!(at(&e, s.body_a), [70.0, 64.0, 55.0, 0.0]);
        assert_eq!(at(&e, s.body_b), [10.0, 8.0, 10.0, 0.0]);
        assert_eq!(at(&e, s.pair_a), [50.0, 5.0, 86.0, 0.0]);
        assert_eq!(at(&e, s.pair_b), [7.0, 9.0, 9.0, 0.0]);
    }

    #[test]
    fn fn_00d22260_adds_the_torques_and_leaves_the_bodies() {
        let mut e = Engine::new();
        let s = scene(&mut e);
        e.call(
            0x00d2_2260,
            &args![s.this, 0u32, 0u32, s.body_a, s.pair_a, s.body_b, s.pair_b],
        );
        assert_eq!(at(&e, s.body_a), [100.0, 100.0, 100.0, 0.0]);
        assert_eq!(at(&e, s.body_b), [10.0, 10.0, 10.0, 0.0]);
        assert_eq!(at(&e, s.pair_a), [50.0, 95.0, 14.0, 0.0]);
        assert_eq!(at(&e, s.pair_b), [11.0, 9.0, 9.0, 0.0]);
    }
}
