//! `(unplaced)/run_00a57790` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable of `NiBoolInterpController` (RTTI
/// `.?AVNiBoolInterpController@@`).
const NI_BOOL_INTERP_CONTROLLER_VTABLE: u32 = 0x0109_ac54;
/// `00a38e00` (the engine map names it `NiBoolInterpController::CopyMembers`
/// by folding; its body is the base constructor this one runs first).
const BASE_CONSTRUCT: u32 = 0x00a3_8e00;

// Translated from 00a57790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiBoolInterpController` constructor.
/// The base constructor body (`00a38e00`), then the class vtable. Returns
/// `this`.
pub fn fn_00a57790(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), NI_BOOL_INTERP_CONTROLLER_VTABLE);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a5_7790, fn_00a57790(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00a57790_runs_the_base_constructor_then_sets_the_vtable() {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            Ret::default()
        });
        let this = e.mem.alloc(0x38);
        assert_eq!(e.call(0x00a5_7790, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_BOOL_INTERP_CONTROLLER_VTABLE);
    }
}
