//! `NiCollision/run_00eae9e0` (Xbox PDB source unit), subsystem `NiCollision`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `NiIntersector` constructor (`00eae720`, thiscall: maximum time, derivative time step,
/// pseudodistance threshold, iteration limit).
const NI_INTERSECTOR_CONSTRUCTOR: u32 = 0x00ea_e720;
/// `NiCapsuleSphereIntersector`'s vtable (`010f1aa0`).
const NI_CAPSULE_SPHERE_INTERSECTOR_VTABLE: u32 = 0x010f_1aa0;
/// The `float` at `010f1a98` (`FLT_MAX` in the executable) the parameter starts at.
const INITIAL_PARAMETER_010F1A98: u32 = 0x010f_1a98;

// Translated from 00eae9e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiCapsuleSphereIntersector` constructor (`RET 0x18`): the base
/// constructor with the four numbers (the floats through `FLD`/`FSTP`), the capsule (+0x38) and the
/// sphere (+0x3c), the vtable `010f1aa0`, `m_fInvRadiusSumSqr` (+0x40) `1 / (r1 + r2)^2` with the
/// capsule's radius (+0x38) plus the sphere's (+0x10) stored as a float first, and `m_fParameter`
/// (+0x44) the float at `010f1a98`. Answers `this`.
pub fn fn_00eae9e0(
    e: &mut Engine,
    this: Ptr,
    capsule: Ptr,
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
    e.mem.set_u32(base + 0x38, capsule.addr());
    e.mem.set_u32(base, NI_CAPSULE_SPHERE_INTERSECTOR_VTABLE);
    e.mem.set_u32(base + 0x3c, sphere.addr());
    let sum = x87_store32(x87_add(
        x87_load32(e.mem.u32(capsule.addr().wrapping_add(0x38))),
        x87_m32(e.mem.u32(sphere.addr().wrapping_add(0x10))),
    ));
    let sum = x87_load32(sum);
    e.mem
        .set_u32(base + 0x40, x87_store32(x87_div(1.0, x87_mul(sum, sum))));
    let parameter = x87_quiet_bits(e.mem.u32(INITIAL_PARAMETER_010F1A98));
    e.mem.set_u32(base + 0x44, parameter);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00ea_e9e0,
        fn_00eae9e0(Ptr, Ptr, Ptr, f32, f32, f32, i32) -> Ptr
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_on_the_base_and_keeps_the_radius_sum() {
        let mut e = Engine::new();
        e.map(0x010f_1000, 0x1000);
        e.mem.set_u32(INITIAL_PARAMETER_010F1A98, 0x7f7f_ffff);
        e.register(NI_INTERSECTOR_CONSTRUCTOR, |_, a| a[0].into_ret());
        let capsule = e.mem.alloc(0x3c);
        let sphere = e.mem.alloc(0x14);
        e.mem.set_f32(capsule + 0x38, 1.5);
        e.mem.set_f32(sphere + 0x10, 0.5);
        let this = e.mem.alloc(0x48);
        e.call_log = Some(vec![]);
        let r = e.call(
            0x00ea_e9e0,
            &args![
                this,
                capsule,
                sphere,
                0x7f80_0001u32,
                0.001f32,
                0.0001f32,
                9i32
            ],
        );
        assert_eq!(r.u32(), this);
        assert_eq!(
            e.call_log.as_ref().unwrap()[1],
            (
                NI_INTERSECTOR_CONSTRUCTOR,
                vec![
                    this,
                    0x7fc0_0001,
                    0.001f32.to_bits(),
                    0.0001f32.to_bits(),
                    9
                ]
            )
        );
        assert_eq!(e.mem.u32(this), NI_CAPSULE_SPHERE_INTERSECTOR_VTABLE);
        assert_eq!(e.mem.u32(this + 0x38), capsule);
        assert_eq!(e.mem.u32(this + 0x3c), sphere);
        assert_eq!(e.mem.f32(this + 0x40), 0.25);
        assert_eq!(e.mem.u32(this + 0x44), 0x7f7f_ffff);
    }
}
