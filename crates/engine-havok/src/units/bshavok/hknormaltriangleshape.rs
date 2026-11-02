//! `BSHavok/hknormaltriangleshape.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::x87::rsqrt_estimate;

/// `0.0f`, `3.0f` and `0.5f` in `.rdata`: the constants the code loads.
const ZERO_01011D78: u32 = 0x0101_1d78;
const THREE_01017718: u32 = 0x0101_7718;
const HALF_01016248: u32 = 0x0101_6248;

// Translated from 00d212d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkNormalTriangleShape::castRay` (map name, taken from a vtable).
/// Thiscall, three words (`RET 0xc`); returns `result`. SSE in `f32`.
/// The ray is `input[4..7] - input[0..3]` (`from` at `+0`, `to` at `+0x10`).
/// It is normalised with `RSQRTSS` plus one Newton step
/// (`0.5 * r * (3 - s*r*r)`, zero when `s == 0`) and dotted with the
/// triangle's normal at `this + 0x60` (`p2 + (p1 + p0)`). When that dot
/// is below zero (an ordered compare, so a NaN counts as not below) the
/// shape's virtual slot `+0x20` is called with a one-byte output, `input`
/// and `third`, and `result` gets whether the byte is non-zero; otherwise
/// `result` gets 0 and nothing is called.
///
/// `RSQRTSS` is the host CPU's estimate (`rsqrt_estimate`, as elsewhere in
/// the crate); its exact bits depend on the CPU, but after the Newton step
/// the sign of the dot, the only thing used, does not.
pub fn hk_normal_triangle_shape_cast_ray(
    e: &mut Engine,
    this: Ptr,
    result: Ptr,
    input: Ptr,
    third: u32,
) -> Ptr {
    let lane = |e: &Engine, base: u32, i: u32| e.mem.f32(base + 4 * i);
    let ray = [0, 1, 2].map(|i| lane(e, input.addr() + 0x10, i) - lane(e, input.addr(), i));
    let squares = ray.map(|v| v * v);
    let sum = squares[2] + (squares[1] + squares[0]);
    let reciprocal = rsqrt_estimate(sum);
    let zero = e.mem.f32(ZERO_01011D78);
    let is_zero = sum == zero;
    let three = e.mem.f32(THREE_01017718);
    let half = e.mem.f32(HALF_01016248);
    let refined = (half * reciprocal) * (three - (sum * reciprocal) * reciprocal);
    let factor = if is_zero { 0.0 } else { refined };
    let normal = [0, 1, 2].map(|i| lane(e, this.addr() + 0x60, i));
    let products = [0, 1, 2].map(|i| (factor * ray[i]) * normal[i]);
    let dot = products[2] + (products[1] + products[0]);
    let hit = if 0.0 > dot {
        e.with_stack(4, |e, local| {
            e.vcall(this.addr(), 0x20, &args![local, input, third]);
            e.mem.u8(local.addr()) != 0
        })
    } else {
        false
    };
    e.mem.set_u8(result.addr(), hit as u8);
    result
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d212d0, hk_normal_triangle_shape_cast_ray(Ptr, Ptr, Ptr, u32) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// A triangle shape with normal (0, 0, 1) whose slot 0x20 writes
    /// `hit` and a ray from the origin to `to`.
    fn cast_00d212d0(to: [f32; 3], hit: u8) -> (u8, usize) {
        let mut e = Engine::new();
        for page in [0x0101_1000, 0x0101_6000, 0x0101_7000] {
            e.map(page, 0x1000);
        }
        e.set_global(ZERO_01011D78, 0.0f32);
        e.set_global(THREE_01017718, 3.0f32);
        e.set_global(HALF_01016248, 0.5f32);
        let mut slots = vec![0u32; 9];
        slots[8] = 0x0090_5000;
        e.put_vtable(0x0090_2000, &slots);
        let calls = Rc::new(RefCell::new(0usize));
        let counter = Rc::clone(&calls);
        e.register_double(0x0090_5000, move |e, a| {
            *counter.borrow_mut() += 1;
            assert_eq!(a[3], 0x77);
            e.mem.set_u8(a[1], hit);
            Ret::default()
        });
        let this = e.mem.alloc(0x70);
        e.mem.set_u32(this, 0x0090_2000);
        e.mem.set_f32(this + 0x68, 1.0);
        let input = e.mem.alloc(0x20);
        for (i, v) in to.iter().enumerate() {
            e.mem.set_f32(input + 0x10 + 4 * i as u32, *v);
        }
        let result = e.mem.alloc(4);
        e.mem.set_u8(result, 0xee);
        let back = e.call(
            0x00d2_12d0,
            &args![
                Ptr::<()>::new(this),
                Ptr::<()>::new(result),
                Ptr::<()>::new(input),
                0x77u32
            ],
        );
        assert_eq!(back.u32(), result);
        let count = *calls.borrow();
        (e.mem.u8(result), count)
    }

    #[test]
    fn hk_normal_triangle_shape_cast_ray_casts_only_against_the_front_face() {
        // Against the normal (dot < 0): the shape's cast is asked.
        assert_eq!(cast_00d212d0([0.0, 0.0, -2.0], 1), (1, 1));
        assert_eq!(cast_00d212d0([0.0, 0.0, -2.0], 0), (0, 1));
        // With the normal, sideways, zero-length or NaN: not asked, no hit.
        assert_eq!(cast_00d212d0([0.0, 0.0, 2.0], 1), (0, 0));
        assert_eq!(cast_00d212d0([3.0, 0.0, 0.0], 1), (0, 0));
        assert_eq!(cast_00d212d0([0.0, 0.0, 0.0], 1), (0, 0));
        assert_eq!(cast_00d212d0([0.0, 0.0, f32::NAN], 1), (0, 0));
    }
}
