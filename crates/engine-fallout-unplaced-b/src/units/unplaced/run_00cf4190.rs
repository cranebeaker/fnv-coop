//! `(unplaced)/run_00cf4190` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The base constructor `00cdf560` (Havok SDK; thiscall: one float).
const BASE_CTOR: u32 = 0x00cd_f560;
/// `bhkPositionConstraintMotor`'s vtable (`010cc064`; RTTI `.?AVbhkPositionConstraintMotor@@`).
const VTABLE_POSITION_CONSTRAINT_MOTOR: u32 = 0x010c_c064;

// Translated from 00cf4190 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `bhkPositionConstraintMotor` constructor (unnamed in the engine map; class from the
/// vtable it sets): the base constructor `00cdf560` with the float 0.0 (`FLDZ`/`FSTP`),
/// then the vtable `010cc064`. Returns `this`.
pub fn bhk_position_constraint_motor_ctor(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_CTOR, &args![this, 0.0f32.to_bits()]);
    e.mem.set_u32(this.addr(), VTABLE_POSITION_CONSTRAINT_MOTOR);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00cf_4190, bhk_position_constraint_motor_ctor(Ptr) -> Ptr)]
}

#[cfg(test)]
mod n081_tests {
    use super::*;

    #[test]
    fn calls_the_base_with_zero_and_sets_the_vtable() {
        let mut e = Engine::new();
        e.register(BASE_CTOR, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            Ret::default()
        });
        let this = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00cf_4190, &args![this]).u32(), this);
        assert_eq!(e.call_log.take().unwrap()[1], (BASE_CTOR, vec![this, 0]));
        assert_eq!(e.mem.u32(this), VTABLE_POSITION_CONSTRAINT_MOTOR);
    }
}
