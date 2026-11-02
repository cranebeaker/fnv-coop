//! `NiCollision/run_00eb3c40` (Xbox PDB source unit), subsystem `NiCollision`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `NiIntersector` constructor (`00eae720`, thiscall: maximum time, derivative time step,
/// pseudodistance threshold, iteration limit).
const NI_INTERSECTOR_CONSTRUCTOR: u32 = 0x00ea_e720;
/// `NiBoxSphereIntersector`'s vtable (`010f1b10`, by its RTTI).
const NI_BOX_SPHERE_INTERSECTOR_VTABLE: u32 = 0x010f_1b10;
/// The `float` at `010f1b08` (`FLT_MAX` in the executable) the three parameters start at.
const INITIAL_PARAMETER_010F1B08: u32 = 0x010f_1b08;

// Translated from 00eb3c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiBoxSphereIntersector` constructor (vtable `010f1b10` by its
/// RTTI, `RET 0x18`): the base constructor with the four numbers (the floats through
/// `FLD`/`FSTP`), the box volume (+0x38) and the sphere volume (+0x3c), `m_fInvRadiusSqr`
/// (+0x40) `1 / r^2` of the sphere's radius (+0x10) and the three parameters (+0x44, +0x48,
/// +0x4c) the float at `010f1b08`. Answers `this`.
pub fn fn_00eb3c40(
    e: &mut Engine,
    this: Ptr,
    box_volume: Ptr,
    sphere: Ptr,
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
    e.mem.set_u32(base + 0x3c, sphere.addr());
    e.mem.set_u32(base, NI_BOX_SPHERE_INTERSECTOR_VTABLE);
    e.mem.set_u32(base + 0x38, box_volume.addr());
    let radius = x87_load32(e.mem.u32(sphere.addr().wrapping_add(0x10)));
    e.mem.set_u32(
        base + 0x40,
        x87_store32(x87_div(1.0, x87_mul(radius, radius))),
    );
    let parameter = x87_quiet_bits(e.mem.u32(INITIAL_PARAMETER_010F1B08));
    for offset in [0x44u32, 0x48, 0x4c] {
        e.mem.set_u32(base + offset, parameter);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00eb_3c40,
        fn_00eb3c40(Ptr, Ptr, Ptr, f32, f32, f32, i32) -> Ptr
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_both_volumes_and_the_inverse_square_radius() {
        let mut e = Engine::new();
        e.map(0x010f_1000, 0x1000);
        e.mem.set_u32(INITIAL_PARAMETER_010F1B08, 0x7f7f_ffff);
        e.register(NI_INTERSECTOR_CONSTRUCTOR, |_, a| a[0].into_ret());
        let box_volume = e.mem.alloc(0x40);
        let sphere = e.mem.alloc(0x14);
        e.mem.set_f32(sphere + 0x10, 2.0);
        let this = e.mem.alloc(0x50);
        let r = e.call(
            0x00eb_3c40,
            &args![this, box_volume, sphere, 1.0f32, 0.001f32, 0.0001f32, 3i32],
        );
        assert_eq!(r.u32(), this);
        assert_eq!(e.mem.u32(this), NI_BOX_SPHERE_INTERSECTOR_VTABLE);
        assert_eq!(e.mem.u32(this + 0x38), box_volume);
        assert_eq!(e.mem.u32(this + 0x3c), sphere);
        assert_eq!(e.mem.f32(this + 0x40), 0.25);
        for offset in [0x44u32, 0x48, 0x4c] {
            assert_eq!(e.mem.u32(this + offset), 0x7f7f_ffff);
        }
    }
}
