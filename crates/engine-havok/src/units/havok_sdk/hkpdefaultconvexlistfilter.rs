//! `Havok SDK/hkpdefaultconvexlistfilter.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00cf6dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpDefaultConvexListFilter::getConvexListCollisionType` (Ghidra name): `ECX` = filter
/// (unused), three stack words: one never read, then two bodies. Each body's first word is its
/// shape. The shape's type (`+0xC`) of the first body indexes the table of 32-bit words at
/// `+0x110` of the second body's shape; the result is bit 8 of that word (0 or 1).
pub fn hkp_default_convex_list_filter_get_convex_list_collision_type(
    e: &mut Engine,
    _this: Ptr,
    _unused_1: u32,
    first: Ptr,
    second: Ptr,
) -> u32 {
    let first_shape = e.mem.u32(first.addr());
    let shape_type = e.mem.u32(first_shape + 0xc);
    let second_shape = e.mem.u32(second.addr());
    let entry = e.mem.u32(second_shape + 0x110 + shape_type.wrapping_mul(4));
    (entry >> 8) & 1
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00cf6dc0,
        hkp_default_convex_list_filter_get_convex_list_collision_type(Ptr, u32, Ptr, Ptr) -> u32
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collision_type_is_bit_eight_of_the_table_entry_for_the_first_shape_type() {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        let first_shape = e.mem.alloc(0x20);
        e.mem.set_u32(first_shape + 0xc, 3);
        let second_shape = e.mem.alloc(0x200);
        let first = e.mem.alloc(0x10);
        let second = e.mem.alloc(0x10);
        e.mem.set_u32(first, first_shape);
        e.mem.set_u32(second, second_shape);
        e.mem.set_u32(second_shape + 0x110 + 3 * 4, 0x0000_0100);
        e.mem.set_u32(second_shape + 0x110 + 2 * 4, 0xffff_feff);
        let call = |e: &mut Engine| e.call(0x00cf_6dc0, &args![0u32, 0u32, first, second]).u32();
        assert_eq!(call(&mut e), 1);
        e.mem.set_u32(second_shape + 0x110 + 3 * 4, 0xffff_feff);
        assert_eq!(call(&mut e), 0);
    }
}
