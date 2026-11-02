//! `fallout shared/pathfinding/run_006abf60` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Vtable of `BSSimpleArray<NavMeshSearchNode,1024>` (RTTI
/// `.?AV?$BSSimpleArray@UNavMeshSearchNode@@$0EAA@@@`).
const SIMPLE_ARRAY_SEARCH_NODE_VTABLE: u32 = 0x0106_b8a0;
/// Vtable of `BSSimpleArray<NavMeshSearchLink,1024>` (RTTI
/// `.?AV?$BSSimpleArray@UNavMeshSearchLink@@$0EAA@@@`).
const SIMPLE_ARRAY_SEARCH_LINK_VTABLE: u32 = 0x0106_b8c8;
/// Vtable of `BSSimpleArray<PathingLocation,1024>` (RTTI
/// `.?AV?$BSSimpleArray@VPathingLocation@@$0EAA@@@`).
const SIMPLE_ARRAY_PATHING_LOCATION_VTABLE: u32 = 0x0106_b91c;
/// The node array's set-up (`009a3d00`, thiscall, two words); unnamed in
/// the engine map.
const SEARCH_NODE_ARRAY_INIT_009A3D00: u32 = 0x009a_3d00;
/// The link array's set-up (`006b3f40`, thiscall, two words; engine map
/// unit `pathsmootherpov.cpp`, unnamed).
const SEARCH_LINK_ARRAY_INIT_006B3F40: u32 = 0x006b_3f40;
/// The pathing-location array's release (`006ac010`, thiscall, one word);
/// unnamed in the engine map.
const PATHING_LOCATION_ARRAY_RELEASE_006AC010: u32 = 0x006a_c010;

// Translated from 006abf60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NavMeshSearchNode,1024>`'s constructor (by the vtable it
/// stores; unnamed in the engine map): the vtable, then
/// `009a3d00(this, 0, 0)`. Returns `this`.
pub fn fn_006abf60(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), SIMPLE_ARRAY_SEARCH_NODE_VTABLE);
    e.call(SEARCH_NODE_ARRAY_INIT_009A3D00, &args![this, 0u32, 0u32]);
    this
}

// Translated from 006abf90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NavMeshSearchLink,1024>`'s constructor (by the vtable it
/// stores): the vtable, then `006b3f40(this, 0, 0)`. Returns `this`.
pub fn fn_006abf90(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), SIMPLE_ARRAY_SEARCH_LINK_VTABLE);
    e.call(SEARCH_LINK_ARRAY_INIT_006B3F40, &args![this, 0u32, 0u32]);
    this
}

// Translated from 006abfc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<PathingLocation,1024>`'s destructor body (by the vtable
/// it stores): the vtable, then `006ac010(this, 1)`. Returns nothing.
pub fn fn_006abfc0(e: &mut Engine, this: Ptr) {
    e.mem
        .set_u32(this.addr(), SIMPLE_ARRAY_PATHING_LOCATION_VTABLE);
    e.call(PATHING_LOCATION_ARRAY_RELEASE_006AC010, &args![this, 1u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006a_bf60, fn_006abf60(Ptr) -> Ptr),
        entry!(0x006a_bf90, fn_006abf90(Ptr) -> Ptr),
        entry!(0x006a_bfc0, fn_006abfc0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Doubles `callee` so it copies the vtable it sees to `+4`.
    fn record_vtable(e: &mut Engine, callee: u32) {
        e.register(callee, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4, seen);
            Ret::default()
        });
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn search_node_array_constructor_sets_the_vtable_then_initialises() {
        let mut e = Engine::new();
        record_vtable(&mut e, SEARCH_NODE_ARRAY_INIT_009A3D00);
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006a_bf60, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), SIMPLE_ARRAY_SEARCH_NODE_VTABLE);
        assert_eq!(e.mem.u32(this + 4), SIMPLE_ARRAY_SEARCH_NODE_VTABLE);
        assert_eq!(
            calls_to(&e, SEARCH_NODE_ARRAY_INIT_009A3D00),
            vec![vec![this, 0, 0]]
        );
    }

    #[test]
    fn search_link_array_constructor_sets_the_vtable_then_initialises() {
        let mut e = Engine::new();
        record_vtable(&mut e, SEARCH_LINK_ARRAY_INIT_006B3F40);
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006a_bf90, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), SIMPLE_ARRAY_SEARCH_LINK_VTABLE);
        assert_eq!(e.mem.u32(this + 4), SIMPLE_ARRAY_SEARCH_LINK_VTABLE);
        assert_eq!(
            calls_to(&e, SEARCH_LINK_ARRAY_INIT_006B3F40),
            vec![vec![this, 0, 0]]
        );
    }

    #[test]
    fn pathing_location_array_destructor_sets_the_vtable_then_releases() {
        let mut e = Engine::new();
        record_vtable(&mut e, PATHING_LOCATION_ARRAY_RELEASE_006AC010);
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x006a_bfc0, &args![this]);
        assert_eq!(e.mem.u32(this), SIMPLE_ARRAY_PATHING_LOCATION_VTABLE);
        assert_eq!(e.mem.u32(this + 4), SIMPLE_ARRAY_PATHING_LOCATION_VTABLE);
        assert_eq!(
            calls_to(&e, PATHING_LOCATION_ARRAY_RELEASE_006AC010),
            vec![vec![this, 1]]
        );
    }
}
