//! `NiCollision/run_00eb3750` (Xbox PDB source unit), subsystem `NiCollision`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `NiIntersector` constructor (`00eae720`, thiscall: maximum time, derivative time step,
/// pseudodistance threshold, iteration limit).
const NI_INTERSECTOR_CONSTRUCTOR: u32 = 0x00ea_e720;
/// `NiCapsuleTriIntersector`'s vtable (`010f1ad8`, by its RTTI).
const NI_CAPSULE_TRI_INTERSECTOR_VTABLE: u32 = 0x010f_1ad8;
/// The `float` at `010f1ad0` (`FLT_MAX` in the executable) both parameters start at.
const INITIAL_PARAMETER_010F1AD0: u32 = 0x010f_1ad0;

// Translated from 00eb3750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiCapsuleTriIntersector` constructor (vtable `010f1ad8` by its
/// RTTI, `RET 0x20`): the base constructor with the four numbers (the floats through
/// `FLD`/`FSTP`), the capsule volume (+0x38), the triangle `m_kTri` (+0x3c: the first vertex, then
/// the edges `v1 - v0` and `v2 - v0` stored as floats), `m_fInvRadiusSqr` (+0x60) `1 / r^2` of the
/// capsule's radius (+0x38) and both parameters (+0x64, +0x68) the float at `010f1ad0`. Answers
/// `this`.
pub fn fn_00eb3750(
    e: &mut Engine,
    this: Ptr,
    capsule: Ptr,
    vertex0: Ptr,
    vertex1: Ptr,
    vertex2: Ptr,
    max_time: f32,
    derivative_time_step: f32,
    pseudodistance_threshold: f32,
    max_iterations: i32,
) -> Ptr {
    let base = this.addr();
    let quiet = |value: f32| x87_quiet_bits(value.to_bits());
    e.call(
        NI_INTERSECTOR_CONSTRUCTOR,
        &args![
            this,
            quiet(max_time),
            quiet(derivative_time_step),
            quiet(pseudodistance_threshold),
            max_iterations
        ],
    );
    e.mem.set_u32(base, NI_CAPSULE_TRI_INTERSECTOR_VTABLE);
    e.mem.set_u32(base + 0x38, capsule.addr());
    let origin = vertex0.addr();
    for offset in [0u32, 4, 8] {
        let word = e.mem.u32(origin.wrapping_add(offset));
        e.mem.set_u32(base + 0x3c + offset, word);
    }
    for (edge, vertex) in [(0x48u32, vertex1.addr()), (0x54, vertex2.addr())] {
        for offset in [0u32, 4, 8] {
            let word = x87_store32(x87_sub(
                x87_load32(e.mem.u32(vertex.wrapping_add(offset))),
                x87_m32(e.mem.u32(origin.wrapping_add(offset))),
            ));
            e.mem.set_u32(base + edge + offset, word);
        }
    }
    let radius = x87_load32(e.mem.u32(capsule.addr().wrapping_add(0x38)));
    e.mem.set_u32(
        base + 0x60,
        x87_store32(x87_div(1.0, x87_mul(radius, radius))),
    );
    let parameter = x87_quiet_bits(e.mem.u32(INITIAL_PARAMETER_010F1AD0));
    e.mem.set_u32(base + 0x64, parameter);
    e.mem.set_u32(base + 0x68, parameter);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00eb_3750,
        fn_00eb3750(Ptr, Ptr, Ptr, Ptr, Ptr, f32, f32, f32, i32) -> Ptr
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn point(e: &mut Engine, v: [f32; 3]) -> u32 {
        let p = e.mem.alloc(12);
        for (i, c) in v.iter().enumerate() {
            e.mem.set_f32(p + 4 * i as u32, *c);
        }
        p
    }

    #[test]
    fn keeps_the_triangle_as_origin_and_edges() {
        let mut e = Engine::new();
        e.map(0x010f_1000, 0x1000);
        e.mem.set_u32(INITIAL_PARAMETER_010F1AD0, 0x7f7f_ffff);
        e.register(NI_INTERSECTOR_CONSTRUCTOR, |_, a| a[0].into_ret());
        let capsule = e.mem.alloc(0x3c);
        e.mem.set_f32(capsule + 0x38, 2.0);
        let v0 = point(&mut e, [1.0, 1.0, 1.0]);
        let v1 = point(&mut e, [3.0, 1.0, 1.0]);
        let v2 = point(&mut e, [1.0, 4.0, 0.0]);
        let this = e.mem.alloc(0x6c);
        e.call_log = Some(vec![]);
        let r = e.call(
            0x00eb_3750,
            &args![this, capsule, v0, v1, v2, 1.0f32, 0.001f32, 0.0001f32, 5i32],
        );
        assert_eq!(r.u32(), this);
        assert_eq!(
            e.call_log.as_ref().unwrap()[1].1,
            vec![
                this,
                1.0f32.to_bits(),
                0.001f32.to_bits(),
                0.0001f32.to_bits(),
                5
            ]
        );
        let words: Vec<f32> = (0..9).map(|i| e.mem.f32(this + 0x3c + 4 * i)).collect();
        assert_eq!(words, vec![1.0, 1.0, 1.0, 2.0, 0.0, 0.0, 0.0, 3.0, -1.0]);
        assert_eq!(e.mem.u32(this), NI_CAPSULE_TRI_INTERSECTOR_VTABLE);
        assert_eq!(e.mem.u32(this + 0x38), capsule);
        assert_eq!(e.mem.f32(this + 0x60), 0.25);
        assert_eq!(e.mem.u32(this + 0x64), 0x7f7f_ffff);
        assert_eq!(e.mem.u32(this + 0x68), 0x7f7f_ffff);
    }
}
