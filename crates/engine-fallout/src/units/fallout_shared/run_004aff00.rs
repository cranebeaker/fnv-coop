//! `fallout shared/run_004aff00` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The base constructor `004b0260` (`this` plus two stack words, `RET 8`).
const BASE_CONSTRUCTOR_004B0260: u32 = 0x004b_0260;
/// The vtable stored at `+0` (RTTI class `NiTObjectArray<NiPointer<NiAVObject>>`, read from the
/// locator before the vtable; the engine map has no name for the constructor).
const VTABLE_004AFF00: u32 = 0x0101_fbd4;

// Translated from 004aff00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: runs the base constructor `004b0260` on `this` with its two
/// stack words, then stores the vtable `0101fbd4` at `+0`. Returns `this`.
pub fn fn_004aff00(e: &mut Engine, this: Ptr, first: u32, second: u32) -> Ptr {
    e.call(BASE_CONSTRUCTOR_004B0260, &args![this, first, second]);
    e.mem.set_u32(this.addr(), VTABLE_004AFF00);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x004a_ff00, fn_004aff00(Ptr, u32, u32) -> Ptr)]
}

#[cfg(test)]
mod static_init_callee_tests {
    use super::*;

    fn calls_to(e: &mut Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    #[test]
    fn runs_the_base_constructor_then_stores_the_vtable() {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCTOR_004B0260, |e, a| {
            e.mem.set_u32(a[0], 0x1234);
            Ret::default()
        });
        let this = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        let result = e.call(0x004a_ff00, &args![this, 7u32, 9u32]);
        assert_eq!(result.u32(), this);
        assert_eq!(
            calls_to(&mut e, BASE_CONSTRUCTOR_004B0260),
            vec![vec![this, 7, 9]]
        );
        assert_eq!(e.mem.u32(this), VTABLE_004AFF00);
    }
}
