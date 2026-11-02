//! `Havok SDK/hkmeshprimitiveutil.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00e60cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMeshPrimitiveUtil::calculateNumIndices` (Xbox PDB), cdecl (`kind`, `count`): the number
/// of indices of `count` primitives, by the primitive kind through the code's four-entry
/// jump table (`00e60ce0`, read from the exe): kind 1 gives `count`, kind 2 `count * 2`,
/// kind 3 `count * 3`, kind 4 `count + 2`; every other kind (the table index is
/// `kind - 1`, unsigned, above 3) gives 0. The kind names are not confirmed here.
pub fn hk_mesh_primitive_util_calculate_num_indices(_e: &mut Engine, kind: u32, count: u32) -> u32 {
    match kind.wrapping_sub(1) {
        0 => count,
        1 => count.wrapping_add(count),
        2 => count.wrapping_add(count.wrapping_mul(2)),
        3 => count.wrapping_add(2),
        _ => 0,
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e6_0cb0, hk_mesh_primitive_util_calculate_num_indices(u32, u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_number_of_indices_follows_the_kind() {
        let mut e = Engine::new();
        let indices =
            |e: &mut Engine, kind: u32, count: u32| e.call(0x00e6_0cb0, &args![kind, count]).u32();
        assert_eq!(indices(&mut e, 1, 7), 7);
        assert_eq!(indices(&mut e, 2, 7), 14);
        assert_eq!(indices(&mut e, 3, 7), 21);
        assert_eq!(indices(&mut e, 4, 7), 9);
        assert_eq!(indices(&mut e, 0, 7), 0);
        assert_eq!(indices(&mut e, 5, 7), 0);
        assert_eq!(indices(&mut e, u32::MAX, 7), 0);
    }
}
