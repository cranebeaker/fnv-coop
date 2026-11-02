//! `Havok SDK/hkpflagcdbodypaircollector.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00dafc80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpFlagCdBodyPairCollector::addCdBodyPair` (Xbox PDB; the two body arguments are ignored):
/// sets the "has hit" flag, the byte at `this + 4`.
pub fn hkp_flag_cd_body_pair_collector_add_cd_body_pair(
    e: &mut Engine,
    this: Ptr,
    _unused_1: u32,
    _unused_2: u32,
) {
    e.mem.set_u8(this.addr() + 4, 1);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00da_fc80,
        hkp_flag_cd_body_pair_collector_add_cd_body_pair(Ptr, u32, u32)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_cd_body_pair_sets_the_flag_byte() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, 0x1111_1111);
        e.call(0x00da_fc80, &args![this, 5u32, 6u32]);
        assert_eq!(e.mem.u8(this + 4), 1);
        assert_eq!(e.mem.u32(this), 0x1111_1111);
        assert_eq!(e.mem.u8(this + 5), 0);
    }
}
