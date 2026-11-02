//! `fallout shared/pathfinding/virtualpathingnode.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// `VirtualPathingNode` (Xbox PDB, 0x30 bytes): `iFlags` +0, `Location`
// +4 (a `PathingLocation`, whose position is at its +4) and `pDoorRef` +0x2c.

/// `PathingLocation::PathingLocation` (engine map), `thiscall`.
const LOCATION_CONSTRUCT: u32 = 0x006d_cbb0;
/// `PathingLocation::operator=` (engine map), `thiscall`, `RET 4`.
const LOCATION_ASSIGN: u32 = 0x006d_cce0;
/// `006dd3e0` (`thiscall`, `RET 4`): sets the location from the given
/// object (the code passes a navmesh triangle or node).
const LOCATION_SET_FROM: u32 = 0x006d_d3e0;
/// `PathingLocation::ResolveNavMeshInfo` (engine map), `thiscall`, `RET 4`.
const LOCATION_RESOLVE: u32 = 0x006d_d6f0;

// Translated from 006f4da0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006f4da0 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// `VirtualPathingNode::VirtualPathingNode` (Xbox PDB): the location
/// (constructed, then assigned from `location`), the door reference, flags
/// 6, and flag 1 when the location resolves its navmesh information
/// (`006dd6f0` with 0). Returns `this`.
pub fn virtual_pathing_node_virtual_pathing_node(
    e: &mut Engine,
    this: Ptr,
    location: u32,
    door: u32,
) -> Ptr {
    let base = this.addr();
    e.call(LOCATION_CONSTRUCT, &args![base + 4]);
    e.call(LOCATION_ASSIGN, &args![base + 4, location]);
    e.mem.set_u32(base + 0x2c, door);
    e.mem.set_u32(base, 6);
    if e.call(LOCATION_RESOLVE, &args![base + 4, 0u32]).bool() {
        let flags = e.mem.u32(base);
        e.mem.set_u32(base, flags | 1);
    }
    this
}

// Translated from 006f4e40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006f4e40 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// A `VirtualPathingNode` constructor from one object (no name in the
/// engine map): the location set from `source` (`006dd3e0`), no door, the
/// position copied from `source + 0x10` (`006f4ed0`), flags 1. Returns
/// `this`.
pub fn fn_006f4e40(e: &mut Engine, this: Ptr, source: u32) -> Ptr {
    let base = this.addr();
    e.call(LOCATION_CONSTRUCT, &args![base + 4]);
    e.call(LOCATION_SET_FROM, &args![base + 4, source]);
    e.mem.set_u32(base + 0x2c, 0);
    fn_006f4ed0(e, Ptr::new(base + 4), source.wrapping_add(0x10));
    e.mem.set_u32(base, 1);
    this
}

// Translated from 006f4ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the three words at `point` to +4 of `this` (a `PathingLocation`'s
/// position; no name in the engine map).
pub fn fn_006f4ed0(e: &mut Engine, this: Ptr, point: u32) {
    for i in 0..3 {
        let word = e.mem.u32(point + 4 * i);
        e.mem.set_u32(this.addr() + 4 + 4 * i, word);
    }
}

// Translated from 006f4f00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006f4f00 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// A `VirtualPathingNode` constructor (no name in the engine map): the
/// location set from `source` (`006dd3e0`), the door reference, the
/// position copied from `point` (`006f4ed0`), flags 7. Returns `this`.
pub fn fn_006f4f00(e: &mut Engine, this: Ptr, source: u32, point: u32, door: u32) -> Ptr {
    let base = this.addr();
    e.call(LOCATION_CONSTRUCT, &args![base + 4]);
    e.call(LOCATION_SET_FROM, &args![base + 4, source]);
    e.mem.set_u32(base + 0x2c, door);
    fn_006f4ed0(e, Ptr::new(base + 4), point);
    e.mem.set_u32(base, 7);
    this
}

// Translated from 006f4f90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006f4f90 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// A `VirtualPathingNode` constructor (no name in the engine map): the
/// location set from `source` (`006dd3e0`), no door, the position copied
/// from `point` (`006f4ed0`), flags 3. Returns `this`.
pub fn fn_006f4f90(e: &mut Engine, this: Ptr, point: u32, source: u32) -> Ptr {
    let base = this.addr();
    e.call(LOCATION_CONSTRUCT, &args![base + 4]);
    e.call(LOCATION_SET_FROM, &args![base + 4, source]);
    e.mem.set_u32(base + 0x2c, 0);
    fn_006f4ed0(e, Ptr::new(base + 4), point);
    e.mem.set_u32(base, 3);
    this
}

// Translated from 006f5020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resolves the node's navmesh information once (no name in the engine
/// map): when flag 1 is clear and `006dd6f0` (with 0) succeeds on the
/// location, sets flag 1.
pub fn fn_006f5020(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.u32(base) & 1 == 0 && e.call(LOCATION_RESOLVE, &args![base + 4, 0u32]).bool() {
        let flags = e.mem.u32(base);
        e.mem.set_u32(base, flags | 1);
    }
}

// Translated from 006f5060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VirtualPathingNode::SetLocation` (Xbox PDB; `thiscall`; the location;
/// `RET 4`): the location at +4 is assigned (`PathingLocation::operator=`),
/// the flags become 2, and flag 1 is added when the location resolves its
/// navmesh information (`006dd6f0` with 0).
pub fn virtual_pathing_node_set_location(e: &mut Engine, this: Ptr, location: u32) {
    let base = this.addr();
    e.call(LOCATION_ASSIGN, &args![base + 4, location]);
    e.mem.set_u32(base, 2);
    if e.call(LOCATION_RESOLVE, &args![base + 4, 0u32]).bool() {
        let flags = e.mem.u32(base);
        e.mem.set_u32(base, flags | 1);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x006f_4da0,
            virtual_pathing_node_virtual_pathing_node(Ptr, u32, u32) -> Ptr
        ),
        entry!(0x006f_4e40, fn_006f4e40(Ptr, u32) -> Ptr),
        entry!(0x006f_4ed0, fn_006f4ed0(Ptr, u32)),
        entry!(0x006f_4f00, fn_006f4f00(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x006f_4f90, fn_006f4f90(Ptr, u32, u32) -> Ptr),
        entry!(0x006f_5020, fn_006f5020(Ptr)),
        entry!(0x006f_5060, virtual_pathing_node_set_location(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn engine(resolves: bool) -> Engine {
        let mut e = Engine::new();
        e.register(LOCATION_CONSTRUCT, |_, _| Ret::default());
        e.register(LOCATION_ASSIGN, |_, _| Ret::default());
        e.register(LOCATION_SET_FROM, |_, _| Ret::default());
        e.register_double(LOCATION_RESOLVE, move |_, _| Ret {
            eax: u32::from(resolves),
            ..Ret::default()
        });
        e
    }

    fn point(e: &mut Engine, at: u32) -> u32 {
        let block = e.mem.alloc(at + 0xc);
        e.mem.set_u32(block + at, 11);
        e.mem.set_u32(block + at + 4, 22);
        e.mem.set_u32(block + at + 8, 33);
        block
    }

    fn check_position(e: &Engine, node: u32) {
        assert_eq!(e.mem.u32(node + 8), 11);
        assert_eq!(e.mem.u32(node + 0xc), 22);
        assert_eq!(e.mem.u32(node + 0x10), 33);
    }

    #[test]
    fn constructor_from_a_location_resolves_the_navmesh() {
        let mut e = engine(true);
        let node = e.mem.alloc(0x30);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x006f_4da0, &args![node, 0x5000u32, 0x6000u32])
                .u32(),
            node
        );
        assert_eq!(calls_to(&e, LOCATION_ASSIGN), vec![vec![node + 4, 0x5000]]);
        assert_eq!(calls_to(&e, LOCATION_RESOLVE), vec![vec![node + 4, 0]]);
        assert_eq!(e.mem.u32(node), 7);
        assert_eq!(e.mem.u32(node + 0x2c), 0x6000);
        let mut e = engine(false);
        let node = e.mem.alloc(0x30);
        e.call(0x006f_4da0, &args![node, 0x5000u32, 0u32]);
        assert_eq!(e.mem.u32(node), 6);
    }

    #[test]
    fn constructor_from_one_object_copies_its_position() {
        let mut e = engine(true);
        let node = e.mem.alloc(0x30);
        e.mem.set_u32(node + 0x2c, 9);
        let source = point(&mut e, 0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006f_4e40, &args![node, source]).u32(), node);
        assert_eq!(
            calls_to(&e, LOCATION_SET_FROM),
            vec![vec![node + 4, source]]
        );
        check_position(&e, node);
        assert_eq!(e.mem.u32(node), 1);
        assert_eq!(e.mem.u32(node + 0x2c), 0);
    }

    #[test]
    fn fn_006f4ed0_copies_three_words_to_plus_4() {
        let mut e = Engine::new();
        let location = e.mem.alloc(0x28);
        let source = point(&mut e, 0);
        e.call(0x006f_4ed0, &args![location, source]);
        assert_eq!(e.mem.u32(location + 4), 11);
        assert_eq!(e.mem.u32(location + 8), 22);
        assert_eq!(e.mem.u32(location + 0xc), 33);
    }

    #[test]
    fn constructor_with_a_door_sets_flags_7() {
        let mut e = engine(true);
        let node = e.mem.alloc(0x30);
        let at = point(&mut e, 0);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x006f_4f00, &args![node, 0x5000u32, at, 0x6000u32])
                .u32(),
            node
        );
        assert_eq!(
            calls_to(&e, LOCATION_SET_FROM),
            vec![vec![node + 4, 0x5000]]
        );
        check_position(&e, node);
        assert_eq!(e.mem.u32(node), 7);
        assert_eq!(e.mem.u32(node + 0x2c), 0x6000);
    }

    #[test]
    fn constructor_with_a_point_sets_flags_3() {
        let mut e = engine(true);
        let node = e.mem.alloc(0x30);
        let at = point(&mut e, 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006f_4f90, &args![node, at, 0x5000u32]).u32(), node);
        assert_eq!(
            calls_to(&e, LOCATION_SET_FROM),
            vec![vec![node + 4, 0x5000]]
        );
        check_position(&e, node);
        assert_eq!(e.mem.u32(node), 3);
        assert_eq!(e.mem.u32(node + 0x2c), 0);
    }

    #[test]
    fn fn_006f5020_resolves_only_once() {
        let mut e = engine(true);
        let node = e.mem.alloc(0x30);
        e.mem.set_u32(node, 6);
        e.call_log = Some(vec![]);
        e.call(0x006f_5020, &args![node]);
        assert_eq!(e.mem.u32(node), 7);
        e.call(0x006f_5020, &args![node]);
        assert_eq!(calls_to(&e, LOCATION_RESOLVE).len(), 1);
        let mut e = engine(false);
        let node = e.mem.alloc(0x30);
        e.mem.set_u32(node, 2);
        e.call(0x006f_5020, &args![node]);
        assert_eq!(e.mem.u32(node), 2);
    }

    #[test]
    fn set_location_assigns_and_resolves() {
        for (resolves, flags) in [(true, 3u32), (false, 2)] {
            let mut e = engine(resolves);
            let node = e.mem.alloc(0x30);
            e.mem.set_u32(node, 0xf0);
            e.mem.set_u32(node + 0x2c, 0x77);
            e.call_log = Some(vec![]);
            e.call(0x006f_5060, &args![node, 0x4000u32]);
            assert_eq!(calls_to(&e, LOCATION_ASSIGN), vec![vec![node + 4, 0x4000]]);
            assert_eq!(calls_to(&e, LOCATION_RESOLVE), vec![vec![node + 4, 0]]);
            assert!(calls_to(&e, LOCATION_CONSTRUCT).is_empty());
            assert_eq!(e.mem.u32(node), flags);
            assert_eq!(e.mem.u32(node + 0x2c), 0x77);
        }
    }
}
