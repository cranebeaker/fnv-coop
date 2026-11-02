//! `Havok SDK/hkpconvexverticesconnectivity.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00e48de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexVerticesConnectivity::clear` (engine map): `ECX` = object. Sets the sizes of its
/// two arrays (`+0xC` and `+0x18`) to 0; the storage is kept.
pub fn hkp_convex_vertices_connectivity_clear(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr() + 0xc, 0);
    e.mem.set_u32(this.addr() + 0x18, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00e4_8de0,
        hkp_convex_vertices_connectivity_clear(Ptr)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clear_zeroes_both_sizes_and_nothing_else() {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        let this = e.mem.alloc(0x20);
        for offset in (0..0x20).step_by(4) {
            e.mem.set_u32(this + offset, 0x1111_1111);
        }
        e.call(0x00e4_8de0, &args![this]);
        for offset in (0..0x20).step_by(4) {
            let expected = if offset == 0xc || offset == 0x18 {
                0
            } else {
                0x1111_1111
            };
            assert_eq!(e.mem.u32(this + offset), expected);
        }
    }
}
