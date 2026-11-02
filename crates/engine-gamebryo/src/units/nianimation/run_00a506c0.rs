//! `NiAnimation/run_00a506c0` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `NiBSplineInterpolator` constructor (`00a57f20`, `thiscall (this,
/// data, basis data)`; the engine map has no name) and
/// `NiBSplinePoint3Interpolator`'s vtable (`010996dc`).
const BASE_CONSTRUCTOR: u32 = 0x00a5_7f20;
const POINT3_INTERPOLATOR_VTABLE: u32 = 0x0109_96dc;

// Translated from 00a506c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiBSplinePoint3Interpolator` constructor (the engine map has no
/// name; it writes that class's vtable): the base constructor with the
/// first and third arguments, the vtable, and the second argument (the
/// control-point handle) at +0x28. Answers `this`.
pub fn fn_00a506c0(e: &mut Engine, this: Ptr, data: u32, handle: u32, basis_data: u32) -> Ptr {
    e.call(BASE_CONSTRUCTOR, &args![this, data, basis_data]);
    e.mem.set_u32(this.addr(), POINT3_INTERPOLATOR_VTABLE);
    e.mem.set_u32(this.addr() + 0x28, handle);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a5_06c0, fn_00a506c0(Ptr, u32, u32, u32) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn b_spline_point3_constructor_passes_data_and_basis_and_keeps_the_handle() {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCTOR, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x2c);
        let answer = e.call(0x00a5_06c0, &args![this, 0x10u32, 0xffffu32, 0x30u32]);
        assert_eq!(answer.u32(), this);
        assert_eq!(e.mem.u32(this), POINT3_INTERPOLATOR_VTABLE);
        assert_eq!(e.mem.u32(this + 0x28), 0xffff);
        let log = e.call_log.take().unwrap();
        let base: Vec<_> = log
            .iter()
            .filter(|call| call.0 == BASE_CONSTRUCTOR)
            .collect();
        assert_eq!(base.len(), 1);
        assert_eq!(base[0].1, vec![this, 0x10, 0x30]);
    }
}
