//! `Havok SDK/run_00d584f0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d584f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. A virtual method (it is only referenced from vtables)
/// taking two stack words it never reads (`RET 8`); returns 0x60.
pub fn fn_00d584f0(_e: &mut Engine, _this: Ptr, _unused_1: u32, _unused_2: u32) -> u32 {
    0x60
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d584f0, fn_00d584f0(Ptr, u32, u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00d584f0_returns_0x60() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00d5_84f0, &args![Ptr::<()>::new(0x10), 1u32, 2u32])
                .u32(),
            0x60
        );
    }
}
