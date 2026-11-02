//! `(unplaced)/run_00cac900` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00cac900 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. `__fastcall`: the address of the member at `this + 0x74`.
pub fn fn_00cac900(_e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(this.addr().wrapping_add(0x74))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00cac900, fn_00cac900(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00cac900_adds_0x74() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00ca_c900, &args![Ptr::<()>::new(0x1000)])
                .ptr::<()>(),
            Ptr::<()>::new(0x1074)
        );
    }
}
