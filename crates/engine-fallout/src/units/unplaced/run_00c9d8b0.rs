//! `(unplaced)/run_00c9d8b0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c9d8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. A virtual method (it is only referenced from a vtable)
/// taking one stack word it never reads (`RET 4`); returns -1.
pub fn fn_00c9d8b0(_e: &mut Engine, _this: Ptr, _unused_1: u32) -> u32 {
    0xffff_ffff
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c9d8b0, fn_00c9d8b0(Ptr, u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00c9d8b0_returns_minus_one() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00c9_d8b0, &args![Ptr::<()>::new(0x10), 5u32])
                .i32(),
            -1
        );
    }
}
