//! `Havok SDK/hkpremoveterminalsmoppmodifierclass.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! None of the functions has a /GS cookie or an exception frame.

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable of `hkpMoppModifier` (confirmed through its RTTI locator).
const MOPP_MODIFIER_VTABLE: u32 = 0x010e_8894;
/// The block release (`00401030`, cdecl: the block).
const RELEASE_BLOCK: u32 = 0x0040_1030;

// Translated from 00e00910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppModifier::_scalar_deleting_destructor_` (Xbox PDB name): sets the vtable of
/// `hkpMoppModifier` and, when bit 0 of `flags` (a byte) is set, releases the block through
/// `00401030`. Answers `this`.
pub fn hkp_mopp_modifier_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.mem.set_u32(this.addr(), MOPP_MODIFIER_VTABLE);
    if flags & 1 != 0 {
        e.call(RELEASE_BLOCK, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00e0_0910,
        hkp_mopp_modifier_scalar_deleting_destructor(Ptr, u8) -> Ptr
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{calls, log_double, new_log};

    #[test]
    fn the_block_is_released_only_when_bit_zero_is_set_00e00910() {
        for (flags, released) in [(0u32, false), (1, true), (0xfe, false), (0xffff_ff03, true)] {
            let mut e = Engine::new();
            let log = new_log();
            log_double(&mut e, &log, RELEASE_BLOCK, 0);
            let this = e.mem.alloc(0x10);
            assert_eq!(e.call(0x00e0_0910, &args![this, flags]).u32(), this);
            assert_eq!(e.mem.u32(this), MOPP_MODIFIER_VTABLE);
            let seen = calls(&log, RELEASE_BLOCK);
            assert_eq!(seen, if released { vec![vec![this]] } else { vec![] });
        }
    }
}
