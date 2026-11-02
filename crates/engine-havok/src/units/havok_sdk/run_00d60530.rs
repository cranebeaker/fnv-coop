//! `Havok SDK/run_00d60530` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d60530 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (a vtable slot, `RET 0x14`: five stack words; the register `this` is not read):
/// stores a zero byte at the address in the first stack word `out`; the other four are never read.
pub fn fn_00d60530(
    e: &mut Engine,
    _this: Ptr,
    out: Ptr,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) {
    e.mem.set_u8(out.addr(), 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00d6_0530,
        fn_00d60530(Ptr, Ptr, u32, u32, u32, u32)
    )]
}

#[cfg(test)]
mod havok_a04_tests {
    use super::*;

    #[test]
    fn fn_00d60530_clears_the_byte() {
        let mut e = Engine::new();
        let out = e.mem.alloc(8);
        e.mem.set_u32(out, 0xffff_ffff);
        e.call(0x00d6_0530, &args![0u32, out, 1u32, 2u32, 3u32, 4u32]);
        assert_eq!(e.mem.u32(out), 0xffff_ff00);
    }
}
