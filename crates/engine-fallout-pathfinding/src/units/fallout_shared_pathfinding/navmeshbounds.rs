//! `fallout shared/pathfinding/navmeshbounds.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0069cd40` (`unplaced/run_0069b920.rs`): the triangle array's destructor
/// body (its vtable, then the buffer released).
const TRIANGLE_ARRAY_DESTRUCT: u32 = 0x0069_cd40;
/// `MemoryManager` deallocation (`00401030`, cdecl, the block).
const DEALLOCATE: u32 = 0x0040_1030;

// Translated from 0069d0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NavMeshBounds::NavMeshBoundsTriangle,1024>` scalar deleting
/// destructor (engine map, from the vtable; `RET 4`): the destructor body
/// `0069cd40`, then `00401030(this)` when bit 0 of `flags` is set. Returns
/// `this`.
pub fn bs_simple_array_nav_mesh_bounds_triangle_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(TRIANGLE_ARRAY_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x0069_d0c0,
        bs_simple_array_nav_mesh_bounds_triangle_scalar_deleting_destructor(Ptr, u32) -> Ptr
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_0069d0c0_destroys_then_frees_only_with_bit_zero() {
        for (flags, frees) in [(0u32, false), (1, true), (2, false), (3, true)] {
            let mut e = Engine::new();
            e.register(TRIANGLE_ARRAY_DESTRUCT, |_, _| Ret::default());
            e.register(DEALLOCATE, |_, _| Ret::default());
            let this = e.mem.alloc(0x10);
            e.call_log = Some(vec![]);
            let result = e.call(0x0069_d0c0, &args![this, flags]).u32();
            let log = e.call_log.take().unwrap();
            assert_eq!(result, this);
            let destroyed: Vec<_> = log
                .iter()
                .filter(|c| c.0 == TRIANGLE_ARRAY_DESTRUCT)
                .collect();
            assert_eq!(destroyed.len(), 1);
            assert_eq!(destroyed[0].1, vec![this]);
            let freed: Vec<_> = log.iter().filter(|c| c.0 == DEALLOCATE).collect();
            assert_eq!(freed.len(), usize::from(frees));
            if frees {
                assert_eq!(freed[0].1, vec![this]);
            }
        }
    }
}
