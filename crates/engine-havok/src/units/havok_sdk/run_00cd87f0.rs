//! `Havok SDK/run_00cd87f0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00cd87f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. A virtual method (it is only referenced from a vtable)
/// taking two stack words it never reads (`RET 8`); returns 0x20.
pub fn fn_00cd87f0(_e: &mut Engine, _this: Ptr, _unused_1: u32, _unused_2: u32) -> u32 {
    0x20
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00cd87f0, fn_00cd87f0(Ptr, u32, u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00cd87f0_returns_0x20() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00cd_87f0, &args![Ptr::<()>::new(0x10), 1u32, 2u32])
                .u32(),
            0x20
        );
    }
}
