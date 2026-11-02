//! `NiCollision/nicollision.cpp` (Xbox PDB source unit), subsystem `NiCollision`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00e9c7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoundingVolume::Type` (Xbox PDB): the base class's type, -1 (`OR EAX,-1`; `this` is not
/// read).
pub fn ni_bounding_volume_type(_e: &mut Engine, _this: Ptr) -> i32 {
    -1
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e9_c7c0, ni_bounding_volume_type(Ptr) -> i32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_base_type_is_minus_one() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e9_c7c0, &args![0x5000u32]).u32(), 0xffff_ffff);
    }
}
