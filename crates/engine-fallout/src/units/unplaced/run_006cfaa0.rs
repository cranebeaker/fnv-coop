//! `(unplaced)/run_006cfaa0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BSSimpleArray<FastNavMeshTriHandle,1024>`'s vtable (RTTI at `0106c9dc`) and the
/// shared `BSSimpleArray` buffer release `008454f0` (thiscall, one word: 1 here).
const ARRAY_VTABLE: u32 = 0x0106_c9e0;
const ARRAY_RELEASE: u32 = 0x0084_54f0;

// Translated from 006cfaa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<FastNavMeshTriHandle,1024>::~BSSimpleArray` (the class from the
/// vtable's RTTI; called from its scalar deleting destructor and `006cfac0`): puts back the
/// class's vtable and runs `008454f0(1)`.
pub fn bs_simple_array_fast_nav_mesh_tri_handle_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE);
    e.call(ARRAY_RELEASE, &args![this, 1u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x006c_faa0,
        bs_simple_array_fast_nav_mesh_tri_handle_destructor(Ptr)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn destructor_puts_the_vtable_back_and_releases() {
        let mut e = Engine::new();
        e.register(ARRAY_RELEASE, |_, _| Ret::default());
        let array = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x006c_faa0, &args![array]);
        assert_eq!(e.mem.u32(array), ARRAY_VTABLE);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(ARRAY_RELEASE, vec![array, 1])]);
    }
}
