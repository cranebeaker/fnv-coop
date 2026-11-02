//! `NiMain/nigeometrygroupmanager.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiGeometryGroupManager`'s vtable.
const NI_GEOMETRY_GROUP_MANAGER_VTABLE: u32 = 0x0109_df2c;
/// The manager singleton's pointer (`011f4ab4`) the destructor clears.
const MANAGER_SINGLETON: u32 = 0x011f_4ab4;
/// The sized delete (`00aa1460`, `cdecl`: block, size).
const SIZED_DELETE: u32 = 0x00aa_1460;

// Translated from 00a7f840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometryGroupManager::_scalar_deleting_destructor_` (Xbox PDB): stores the vtable
/// `0109df2c`, clears the singleton pointer `011f4ab4` and, when bit 0 of `flags` is set,
/// runs the sized delete of 4 bytes. Answers `this`.
pub fn ni_geometry_group_manager_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.mem.set_u32(this.addr(), NI_GEOMETRY_GROUP_MANAGER_VTABLE);
    e.mem.set_u32(MANAGER_SINGLETON, 0);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 4u32]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00a7_f840,
        ni_geometry_group_manager_scalar_deleting_destructor(Ptr, u32) -> Ptr
    )]
}

#[cfg(test)]
mod n016_tests {
    use super::*;

    #[test]
    fn scalar_deleting_destructor_clears_the_singleton() {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x1000);
        e.register(SIZED_DELETE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(4);
        e.mem.set_u32(MANAGER_SINGLETON, this);
        assert_eq!(e.call(0x00a7_f840, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_GEOMETRY_GROUP_MANAGER_VTABLE);
        assert_eq!(e.mem.u32(MANAGER_SINGLETON), 0);
        e.call(0x00a7_f840, &args![this, 0u32]);
        let deletes: Vec<Vec<u32>> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == SIZED_DELETE)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(deletes, vec![vec![this, 4]]);
    }
}
