//! `BSHavok/run_00cf45d0` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The base constructor (`00d86220`, `this` only).
const BASE_CONSTRUCT: u32 = 0x00d8_6220;
/// The vtable stored after it.
const VTABLE: u32 = 0x010c_c164;

// Translated from 00cf45d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00cf45d0` (unnamed in the Xbox PDB): a constructor with no argument besides `this`.
/// Runs the base constructor `00d86220` on it, stores the vtable 0x010cc164 and answers
/// `this`. The class is not named by the exe data read so far.
pub fn fn_00cf45d0(e: &mut Engine, this: u32) -> u32 {
    e.call(BASE_CONSTRUCT, &args![this]);
    e.mem.set_u32(this, VTABLE);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00cf45d0, fn_00cf45d0(u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bshavok::bhkentity::testing::{calls_to, logging_engine};

    #[test]
    fn runs_the_base_constructor_then_stores_the_vtable() {
        let mut e = logging_engine();
        e.register(BASE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            Ret::default()
        });
        let this = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00cf45d0, &args![this]).u32(), this);
        assert_eq!(calls_to(&e, BASE_CONSTRUCT), [[this]]);
        assert_eq!(e.mem.u32(this), VTABLE);
    }
}
