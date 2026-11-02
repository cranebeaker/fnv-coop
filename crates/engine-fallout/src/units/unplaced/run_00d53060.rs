//! `(unplaced)/run_00d53060` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `004b4ab0` (unnamed; thiscall on a destination vector: a transform, a vector; `RET 8`).
const TRANSFORM_VECTOR: u32 = 0x004b_4ab0;
/// The normalizer's constants: the length² that gives a zero scale (`01011d78`), and the
/// Newton step's 3 and 0.5 (`01017718`, `01016248`).
const ZERO_LENGTH: u32 = 0x0101_1d78;
const NEWTON_THREE: u32 = 0x0101_7718;
const NEWTON_HALF: u32 = 0x0101_6248;

/// Four lanes of the vector at `addr`.
fn load_vector(e: &Engine, addr: u32) -> crate::sse::Xmm {
    std::array::from_fn(|lane| e.mem.f32(addr + 4 * lane as u32))
}

/// Stores four lanes at `addr` (bits kept).
fn store_vector(e: &mut Engine, addr: u32, v: crate::sse::Xmm) {
    for (lane, x) in v.iter().enumerate() {
        e.mem.set_u32(addr + 4 * lane as u32, x.to_bits());
    }
}

/// `SHUFPS v,v,imm` with the same register as both operands.
fn shuffle(v: crate::sse::Xmm, imm: u8) -> crate::sse::Xmm {
    std::array::from_fn(|lane| v[usize::from((imm >> (2 * lane)) & 3)])
}

// Translated from 00d53060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, three words): builds a frame around `axis` and
/// transforms it into both bodies' spaces. A perpendicular `v` (zero vector, then
/// `v[i] = axis[j]`, `v[j] = -axis[i]` (x87 `FLD`/`FCHS`/`FSTP`), with `i`, `j` picked by
/// comparing the components' magnitudes as the exe's two x87 compares do) is normalized
/// (SSE: `RSQRTSS` estimate and one Newton step `0.5 * r * (3 - l²r²)`, scale 0 when `l²` is
/// the float at `01011d78`), `w = axis × n` (SSE shuffles), and `004b4ab0` writes `axis`,
/// `n`, `w` through `first` into the vectors at +0x20, +0x30, +0x40, and through `second`
/// into +0x50, +0x60, +0x70. Passes on the last call's `EAX`.
pub fn fn_00d53060(e: &mut Engine, this: Ptr, first: Ptr, second: Ptr, axis: Ptr) -> u32 {
    use crate::sse::{add, compare, mul, mulps, sub, subps};
    let this = this.addr();
    let axis = axis.addr();
    let magnitude = |e: &Engine, lane: u32| x87_quiet(e.mem.f32(axis + 4 * lane)).abs();
    let x = magnitude(e, 0);
    let y = magnitude(e, 1);
    let z = magnitude(e, 2);
    // ECX, EDX and EBX of the exe: the slot that takes the other component, the slot that
    // takes the negated one, and the fallback for the second.
    let (mut smallest, mut kept, mut fallback) = (x, 1u32, 0u32);
    if x > y {
        smallest = y;
        kept = 0;
        fallback = 1;
    }
    let negated = if smallest > z { fallback } else { 2 };
    e.with_stack(0x20, |e, locals| {
        // +0 the normalized perpendicular, +0x10 the cross product.
        let normal_at = locals.addr();
        let cross_at = normal_at + 0x10;
        let mut v = [0.0f32; 4];
        v[kept as usize] = x87_quiet(e.mem.f32(axis + 4 * negated));
        let flipped = x87_quiet(e.mem.f32(axis + 4 * kept)).to_bits() ^ 0x8000_0000;
        v[negated as usize] = f32::from_bits(flipped);
        let zero_length = e.mem.f32(ZERO_LENGTH);
        let three = e.mem.f32(NEWTON_THREE);
        let half = e.mem.f32(NEWTON_HALF);
        let squares = mulps(v, v);
        let length2 = add(squares[2], add(squares[1], squares[0]));
        let estimate = crate::x87::rsqrt_estimate(length2);
        let scaled = mul(mul(length2, estimate), estimate);
        let step = mul(mul(half, estimate), sub(three, scaled));
        let scale = if compare(length2, zero_length, 0) {
            0.0
        } else {
            step
        };
        let normal = mulps([scale; 4], v);
        let a = load_vector(e, axis);
        let cross = subps(
            mulps(shuffle(a, 0xc9), shuffle(normal, 0xd2)),
            mulps(shuffle(a, 0xd2), shuffle(normal, 0xc9)),
        );
        store_vector(e, normal_at, normal);
        store_vector(e, cross_at, cross);
        let mut last = 0;
        for (body, base) in [(first.addr(), 0x20u32), (second.addr(), 0x50)] {
            for (offset, source) in [(0u32, axis), (0x10, normal_at), (0x20, cross_at)] {
                last = e
                    .call(TRANSFORM_VECTOR, &args![this + base + offset, body, source])
                    .u32();
            }
        }
        last
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d5_3060, fn_00d53060(Ptr, Ptr, Ptr, Ptr) -> u32)]
}

#[cfg(test)]
mod n081_tests {
    use super::*;

    fn constants(e: &mut Engine) {
        e.map(0x0101_1000, 0x7000);
        e.mem.set_f32(ZERO_LENGTH, 0.0);
        e.mem.set_f32(NEWTON_THREE, 3.0);
        e.mem.set_f32(NEWTON_HALF, 0.5);
    }

    fn run(axis: [f32; 4]) -> (Vec<(u32, Vec<u32>)>, Vec<[f32; 4]>) {
        let mut e = Engine::new();
        constants(&mut e);
        e.register(TRANSFORM_VECTOR, |e, a| {
            let v = load_vector(e, a[2]);
            store_vector(e, a[0], v);
            (a[0] ^ 0x5).into_ret()
        });
        let this = e.mem.alloc(0x80);
        let at = e.mem.alloc(16);
        store_vector(&mut e, at, axis);
        e.call_log = Some(vec![]);
        let r = e
            .call(0x00d5_3060, &args![this, 0x100u32, 0x200u32, at])
            .u32();
        assert_eq!(r, (this + 0x70) ^ 5);
        let log: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 == TRANSFORM_VECTOR)
            .collect();
        let vectors = (0..6)
            .map(|i| load_vector(&e, this + 0x20 + 0x10 * i))
            .collect();
        (log, vectors)
    }

    fn close(a: [f32; 4], b: [f32; 4]) -> bool {
        a.iter().zip(b.iter()).all(|(x, y)| (x - y).abs() < 1e-3)
    }

    #[test]
    fn builds_the_frame_for_each_axis_case() {
        // z axis: |x| <= |y|, |x| <= |z| -> v = (0, z, -y) = (0, 1, -0), w = axis x n.
        let (log, v) = run([0.0, 0.0, 1.0, 0.0]);
        assert_eq!(log.len(), 6);
        assert_eq!(log[0].1[1..2], [0x100]);
        assert_eq!(log[3].1[1..2], [0x200]);
        assert_eq!(v[0], [0.0, 0.0, 1.0, 0.0]);
        assert!(close(v[1], [0.0, 1.0, 0.0, 0.0]));
        assert!(close(v[2], [-1.0, 0.0, 0.0, 0.0]));
        assert_eq!(v[3..], v[..3]);
        // x axis: |x| > |y| and |y| <= |z| -> slots 0 and 2: v = (z, 0, -x).
        let (_, v) = run([2.0, 0.0, 0.0, 0.0]);
        assert!(close(v[1], [0.0, 0.0, -1.0, 0.0]));
        // |x| > |y| > |z| -> slots 0 and 1: v = (y, -x, 0).
        let (_, v) = run([3.0, 2.0, 1.0, 0.0]);
        let s = 1.0 / 13.0f32.sqrt();
        assert!(close(v[1], [2.0 * s, -3.0 * s, 0.0, 0.0]));
        // |x| <= |y| and |x| > |z| -> slots 1 and 0: v = (-y, x, 0).
        let (_, v) = run([2.0, 3.0, 1.0, 0.0]);
        assert!(close(v[1], [-3.0 * s, 2.0 * s, 0.0, 0.0]));
        // A zero axis gives a zero scale (length² equal to the constant 0); 0 * -0 is -0.
        let (_, v) = run([0.0; 4]);
        assert_eq!(v[1].map(f32::to_bits), [0, 0, 0x8000_0000, 0]);
    }
}
