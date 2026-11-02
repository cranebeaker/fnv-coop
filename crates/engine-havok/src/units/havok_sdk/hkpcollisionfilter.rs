//! `Havok SDK/hkpcollisionfilter.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00cbfb90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCollisionFilter::numShapeKeyHitsLimitBreached` (Xbox PDB): a method with seven stack
/// words (`RET 0x1C`) that returns the last of them unchanged and reads nothing else
/// (`this` in `ECX` included).
pub fn hkp_collision_filter_num_shape_key_hits_limit_breached(
    e: &mut Engine,
    _unused_this: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
    seventh: u32,
) -> u32 {
    let _ = e;
    seventh
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00cb_fb90,
        hkp_collision_filter_num_shape_key_hits_limit_breached(
            Ptr, u32, u32, u32, u32, u32, u32, u32
        ) -> u32
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_the_seventh_stack_word_00cbfb90() {
        let mut e = Engine::new();
        let result = e.call(0x00cb_fb90, &[0x1000, 1, 2, 3, 4, 5, 6, 0xdead_beef]);
        assert_eq!(result.u32(), 0xdead_beef);
        let result = e.call(0x00cb_fb90, &[0x1000, 1, 2, 3, 4, 5, 6, 0]);
        assert_eq!(result.u32(), 0);
    }
}
