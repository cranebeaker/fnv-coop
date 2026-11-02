//! `Havok SDK/run_00de1150` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00de1150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`fastcall` on `this`): answers `this + 0x60`.
pub fn fn_00de1150(_e: &mut Engine, this: u32) -> u32 {
    this.wrapping_add(0x60)
}

// Translated from 00de1160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`fastcall` on `this`): answers `this + 0x70`.
pub fn fn_00de1160(_e: &mut Engine, this: u32) -> u32 {
    this.wrapping_add(0x70)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00de1150, fn_00de1150(u32) -> u32),
        entry!(0x00de1160, fn_00de1160(u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_two_members_are_offsets_of_this() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00de_1150, &args![0x1000u32]).u32(), 0x1060);
        assert_eq!(e.call(0x00de_1160, &args![0x1000u32]).u32(), 0x1070);
    }
}
