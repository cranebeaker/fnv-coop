//! `(unplaced)/run_00d213d0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d213d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. `__fastcall`: `this + 0x10`, or 0 when `this` is null.
pub fn fn_00d213d0(_e: &mut Engine, this: Ptr) -> Ptr {
    if this.addr() != 0 {
        Ptr::new(this.addr().wrapping_add(0x10))
    } else {
        Ptr::NULL
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d213d0, fn_00d213d0(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00d213d0_adds_0x10_unless_null() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00d2_13d0, &args![Ptr::<()>::new(0x1000)]).u32(),
            0x1010
        );
        assert_eq!(e.call(0x00d2_13d0, &args![Ptr::<()>::NULL]).u32(), 0);
    }
}
