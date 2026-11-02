//! `NiCollision/run_00eaedc0` (Xbox PDB source unit), subsystem `NiCollision`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `NiIntersector` constructor (`00eae720`, thiscall: maximum time, derivative time step,
/// pseudodistance threshold, iteration limit).
const NI_INTERSECTOR_CONSTRUCTOR: u32 = 0x00ea_e720;
/// The `NiIntersector` destructor body (`00eae6d0`, thiscall).
const NI_INTERSECTOR_DESTRUCTOR: u32 = 0x00ea_e6d0;
/// `NiCapsuleCapsuleIntersector`'s vtable (`010f1abc`).
const NI_CAPSULE_CAPSULE_INTERSECTOR_VTABLE: u32 = 0x010f_1abc;
/// The `float` at `010f1ab4` (`FLT_MAX` in the executable) both parameters start at.
const INITIAL_PARAMETER_010F1AB4: u32 = 0x010f_1ab4;
/// The memory manager instance and its `Deallocate` (`00aa4060`, thiscall: the block).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;

// Translated from 00eaedc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiCapsuleCapsuleIntersector` constructor (`RET 0x18`): the base
/// constructor with the four numbers (the floats through `FLD`/`FSTP`), the capsules (+0x38,
/// +0x3c), the vtable `010f1abc`, `m_fInvRadiusSumSqr` (+0x40) `1 / (r0 + r1)^2` from the two
/// radii (+0x38 of each, the sum stored as a float first), and both parameters (+0x44, +0x48) the
/// float at `010f1ab4`. Answers `this`.
pub fn fn_00eaedc0(
    e: &mut Engine,
    this: Ptr,
    capsule0: Ptr,
    capsule1: Ptr,
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
    e.mem.set_u32(base + 0x38, capsule0.addr());
    e.mem.set_u32(base, NI_CAPSULE_CAPSULE_INTERSECTOR_VTABLE);
    e.mem.set_u32(base + 0x3c, capsule1.addr());
    let sum = x87_store32(x87_add(
        x87_load32(e.mem.u32(capsule0.addr().wrapping_add(0x38))),
        x87_m32(e.mem.u32(capsule1.addr().wrapping_add(0x38))),
    ));
    let sum = x87_load32(sum);
    e.mem
        .set_u32(base + 0x40, x87_store32(x87_div(1.0, x87_mul(sum, sum))));
    let parameter = x87_quiet_bits(e.mem.u32(INITIAL_PARAMETER_010F1AB4));
    e.mem.set_u32(base + 0x44, parameter);
    e.mem.set_u32(base + 0x48, parameter);
    this
}

// Translated from 00eaee30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the capsule intersector's scalar deleting destructor: the
/// `NiIntersector` destructor body (`00eae6d0`), then, when bit 0 of `flags` is set, the memory
/// manager frees `this` (`00aa4060`). Answers `this`.
pub fn fn_00eaee30(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(NI_INTERSECTOR_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00ea_edc0,
            fn_00eaedc0(Ptr, Ptr, Ptr, f32, f32, f32, i32) -> Ptr
        ),
        entry!(0x00ea_ee30, fn_00eaee30(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    #[test]
    fn builds_on_the_base_and_keeps_the_radius_sum() {
        let mut e = Engine::new();
        e.map(0x010f_1000, 0x1000);
        e.mem.set_u32(INITIAL_PARAMETER_010F1AB4, 0x7f7f_ffff);
        e.register(NI_INTERSECTOR_CONSTRUCTOR, |_, a| a[0].into_ret());
        let first = e.mem.alloc(0x3c);
        let second = e.mem.alloc(0x3c);
        e.mem.set_f32(first + 0x38, 1.0);
        e.mem.set_f32(second + 0x38, 3.0);
        let this = e.mem.alloc(0x4c);
        e.call_log = Some(vec![]);
        let r = e.call(
            0x00ea_edc0,
            &args![this, first, second, 2.0f32, 0.001f32, 0.0001f32, 4i32],
        );
        assert_eq!(r.u32(), this);
        assert_eq!(
            calls_to(&e, NI_INTERSECTOR_CONSTRUCTOR),
            vec![vec![
                this,
                2.0f32.to_bits(),
                0.001f32.to_bits(),
                0.0001f32.to_bits(),
                4
            ]]
        );
        assert_eq!(e.mem.u32(this), NI_CAPSULE_CAPSULE_INTERSECTOR_VTABLE);
        assert_eq!(e.mem.u32(this + 0x38), first);
        assert_eq!(e.mem.u32(this + 0x3c), second);
        assert_eq!(e.mem.f32(this + 0x40), 1.0 / 16.0);
        assert_eq!(e.mem.u32(this + 0x44), 0x7f7f_ffff);
        assert_eq!(e.mem.u32(this + 0x48), 0x7f7f_ffff);
    }

    #[test]
    fn deleting_destructor_frees_with_bit_zero() {
        let mut e = Engine::new();
        e.register(NI_INTERSECTOR_DESTRUCTOR, |_, _| Ret::default());
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        let this = e.mem.alloc(0x4c);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ea_ee30, &args![this, 0u32]).u32(), this);
        assert_eq!(calls_to(&e, NI_INTERSECTOR_DESTRUCTOR), vec![vec![this]]);
        assert!(calls_to(&e, MEMORY_DEALLOCATE).is_empty());
        e.call(0x00ea_ee30, &args![this, 1u32]);
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, this]]
        );
    }
}
