//! `fallout shared/pathfinding/run_006c8d00` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! An unnamed run of code between `navmeshobstacletask.cpp` and
//! `pathbuilder.cpp`. Its bodies work on a `PathBuilder` (Xbox PDB, 0x34
//! bytes: `pRequest` +0x00, `pSolution` +0x04, `pScrapHeap` +0x08,
//! `WaterFilter` +0x0C, `bPathUpdate` +0x10, `DetailedPathInfos` +0x14,
//! `DetailedPathMeshes` +0x24; the offsets match the PC code below) and on
//! its `PathingSolution`. `PathBuilder` belongs to `pathbuilder.cpp`, so its
//! fields are read at their offsets here; the solution uses `pathfind.rs`'s
//! layout.
//!
//! The filter vtables the constructors store (RTTI read from the exe):
//! `0106c92c` `.?AUFindTriangleForLocationWaterFilter@PathBuilder@@`,
//! `0106c934` `.?AUFindTriangleForLocationFilter@@`.
//!
//! The compiler's exception-unwinding frames (`FS:[0]`) are not translated;
//! the functions that have one are marked incomplete.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_shared_pathfinding::pathfind::PathingSolution;

/// `PathBuilder::pRequest` (Xbox PDB) +0x00: `PathingRequest*`.
const BUILDER_REQUEST: u32 = 0x00;
/// `PathBuilder::pSolution` (Xbox PDB) +0x04: `PathingSolution*`.
const BUILDER_SOLUTION: u32 = 0x04;
/// `PathBuilder::pScrapHeap` (Xbox PDB) +0x08.
const BUILDER_SCRAP_HEAP: u32 = 0x08;
/// `PathBuilder::WaterFilter` (Xbox PDB) +0x0C: a
/// `PathBuilder::FindTriangleForLocationWaterFilter` (one vtable word).
const BUILDER_WATER_FILTER: u32 = 0x0c;
/// `PathBuilder::bPathUpdate` (Xbox PDB) +0x10.
const BUILDER_PATH_UPDATE: u32 = 0x10;
/// `PathBuilder::DetailedPathInfos` (Xbox PDB) +0x14: a `BSSimpleArray`.
const BUILDER_DETAILED_INFOS: u32 = 0x14;
/// `PathBuilder::DetailedPathMeshes` (Xbox PDB) +0x24: a `BSSimpleArray`.
const BUILDER_DETAILED_MESHES: u32 = 0x24;

/// `PathBuilder::FindTriangleForLocationWaterFilter`'s vtable (RTTI).
const WATER_FILTER_VTABLE: u32 = 0x0106_c92c;
/// `FindTriangleForLocationFilter`'s vtable (RTTI).
const FILTER_VTABLE: u32 = 0x0106_c934;

/// `006c7e00` (thiscall, no stack words): the destructor [`fn_006c8d00`]
/// runs (engine map unit `navmeshobstacletask.cpp`).
const DESTRUCTOR_006C7E00: u32 = 0x006c_7e00;
/// `operator delete` (`00401030`, cdecl `(block)`).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `005e0510` (thiscall, no stack words): constructs the
/// `DetailedPathInfos` array.
const INFOS_CONSTRUCT: u32 = 0x005e_0510;
/// `0042f800` (thiscall, no stack words): constructs the
/// `DetailedPathMeshes` array.
const MESHES_CONSTRUCT: u32 = 0x0042_f800;
/// `00401020` (no arguments): the object whose
/// `MemoryManager::GetThreadScrapHeap` is called.
const MEMORY_MANAGER: u32 = 0x0040_1020;
/// `MemoryManager::GetThreadScrapHeap` (`00aa42e0`, thiscall).
const GET_THREAD_SCRAP_HEAP: u32 = 0x00aa_42e0;
/// `0048d150` (thiscall): `this + 0x0C`, here the request's origin
/// `PathingLocation` (`PathingRequest::Origin`).
const REQUEST_ORIGIN: u32 = 0x0048_d150;
/// `006a9540` (thiscall): `this + 0x34`, here the request's destination
/// `PathingLocation` (`PathingRequest::Destination`).
const REQUEST_DESTINATION: u32 = 0x006a_9540;
/// `PathingLocation::ClearTriangleData` (`006ddbe0`, thiscall).
const CLEAR_TRIANGLE_DATA: u32 = 0x006d_dbe0;
/// `005a4320` (thiscall): a word at +8 of `this` through `0044ddc0`; here
/// the solution's virtual node count (`VirtualPathingNodes.iSize`).
const SOLUTION_COUNT_005A4320: u32 = 0x005a_4320;
/// `008b6800` (thiscall): the same word through `0044ddc0`.
const SOLUTION_COUNT_008B6800: u32 = 0x008b_6800;
/// `0044ddc0` (thiscall): the word at +8 of `this` (a `BSSimpleArray`'s
/// `iSize`).
const ARRAY_SIZE: u32 = 0x0044_ddc0;
/// `PathingLocation::ResolveNavMeshInfo` (`006dd6f0`, thiscall
/// `(location, filter)`, byte result).
const RESOLVE_NAV_MESH_INFO: u32 = 0x006d_d6f0;
/// `PathingLocation::GetBestNavMeshInfo` (`006dd540`, thiscall).
const GET_BEST_NAV_MESH_INFO: u32 = 0x006d_d540;
/// `PathingLocation::GetCell` (`006dd4f0`, thiscall).
const GET_CELL: u32 = 0x006d_d4f0;
/// `00441110` (thiscall): the word at +0x1C of `this`
/// (`PathingLocation::GetWorldspace` in the engine map; on a
/// `PathingSolution` it reads `iLastLoadedVirtualNodeIndex`).
const WORD_AT_1C: u32 = 0x0044_1110;
/// `006ca0e0` (thiscall `(builder, a, b)`, `RET 8`, byte result; engine map
/// unit `pathbuilder.cpp`).
const BUILDER_006CA0E0: u32 = 0x006c_a0e0;
/// `PathBuilder::BuildNavMeshInfoPath` (`006c94c0`, thiscall
/// `(builder, from, to)`, byte result).
const BUILD_NAV_MESH_INFO_PATH: u32 = 0x006c_94c0;
/// `PathBuilder::BuildTeleportDoorPath` (`006c9b20`, thiscall, byte
/// result).
const BUILD_TELEPORT_DOOR_PATH: u32 = 0x006c_9b20;
/// `00408d60` (thiscall): returns `this + 4`, or a zeroed static byte
/// (`01202800`) for a null `this`; called with `011d7450` and the result
/// dropped.
const FN_00408D60: u32 = 0x0040_8d60;
/// The `this` [`fn_006c8f50`] passes to `00408d60`.
const OBJECT_011D7450: u32 = 0x011d_7450;
/// `006cefa0` (thiscall `(array, word)`, `RET 4`; engine map unit
/// `pathbuilder.cpp`), run on the solution's `VirtualPathingNodes`.
const VIRTUAL_NODES_006CEFA0: u32 = 0x006c_efa0;
/// `006cef10` (thiscall): reserves one more virtual node and returns its
/// index.
const VIRTUAL_NODES_ADD_SLOT: u32 = 0x006c_ef10;
/// `006e8f40` (thiscall `(array, index)`, `RET 4`): the address of a
/// virtual node.
const VIRTUAL_NODES_AT: u32 = 0x006e_8f40;
/// `006e6da0(size, address)` (cdecl): placement allocation; returns the
/// address.
const PLACEMENT: u32 = 0x006e_6da0;
/// `006f4d00` (thiscall `(node, location)`, `RET 4`): the 0x30-byte virtual
/// node constructor.
const VIRTUAL_NODE_CONSTRUCT: u32 = 0x006f_4d00;
/// `006c9fc0` (thiscall `(builder, index, first*, last*)`, `RET 0xC`, byte
/// result; engine map unit `pathbuilder.cpp`).
const BUILDER_006C9FC0: u32 = 0x006c_9fc0;
/// `006e78b0` (thiscall `(solution, index)`, `RET 4`): the address of a
/// virtual node (through `006e8f40`).
const SOLUTION_VIRTUAL_NODE: u32 = 0x006e_78b0;
/// `00717e50` (thiscall): `this + 4` (named
/// `BaseProcess::GetActorPackageThatIsRunning` in the engine map; folded).
const PLUS_4: u32 = 0x0071_7e50;
/// `006dd0d0` (thiscall `(location, point, navmesh info)`, `RET 8`): a
/// `PathingLocation` constructor.
const LOCATION_CONSTRUCT: u32 = 0x006d_d0d0;
/// `VirtualPathingNode::SetLocation` (`006f5060`, thiscall
/// `(node, location)`, `RET 4`).
const VIRTUAL_NODE_SET_LOCATION: u32 = 0x006f_5060;
/// `006ca850` (thiscall `(builder, word)`, `RET 4`, byte result).
const BUILDER_006CA850: u32 = 0x006c_a850;
/// `006b05d0` (thiscall, byte result; engine map unit `pathsmootherpov.cpp`).
const SOLUTION_006B05D0: u32 = 0x006b_05d0;
/// `004ff7e0` (thiscall): the `PathingLocation` destructor.
const LOCATION_DESTROY: u32 = 0x004f_f7e0;
/// `006f5020` (thiscall; engine map unit `virtualpathingnode.cpp`).
const VIRTUAL_NODE_006F5020: u32 = 0x006f_5020;
/// `006cf000` (thiscall `(array, index, word)`, `RET 8`), run on
/// `CurrentPathingNodes`.
const PATHING_NODES_006CF000: u32 = 0x006c_f000;
/// `006cf120` (thiscall `(array, word)`, `RET 4`), run on
/// `CurrentPathingNodes`.
const PATHING_NODES_006CF120: u32 = 0x006c_f120;

/// Size of a `PathingLocation` (the local [`fn_006c9170`] builds).
const LOCATION_SIZE: u32 = 0x28;

fn builder_word(e: &Engine, this: Ptr, off: u32) -> u32 {
    e.mem.u32(this.addr() + off)
}

fn origin(e: &mut Engine, this: Ptr) -> u32 {
    let request = builder_word(e, this, BUILDER_REQUEST);
    e.call(REQUEST_ORIGIN, &args![request]).u32()
}

fn destination(e: &mut Engine, this: Ptr) -> u32 {
    let request = builder_word(e, this, BUILDER_REQUEST);
    e.call(REQUEST_DESTINATION, &args![request]).u32()
}

// Translated from 006c8d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a scalar deleting destructor: runs
/// `006c7e00` on `this` and frees it with `operator delete` when bit 0 of
/// `flags` is set. Returns `this`. `RET 4`.
pub fn fn_006c8d00(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(DESTRUCTOR_006C7E00, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 006c8d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; by its body the `PathBuilder` constructor:
/// builds the water filter ([`fn_006c8dd0`]) and the two detailed-path
/// arrays (`005e0510`, `0042f800`), takes the thread's scrap heap
/// (`MemoryManager::GetThreadScrapHeap` on `00401020()`), stores the
/// request and solution and clears `bPathUpdate`. Returns `this`. `RET 8`.
// Translation incomplete: 006c8d30 - FS:[0] exception frame (unwinding of the members) is not translated.
pub fn fn_006c8d30(e: &mut Engine, this: Ptr, request: Ptr, solution: Ptr) -> Ptr {
    fn_006c8dd0(e, this.byte_add(BUILDER_WATER_FILTER));
    e.call(
        INFOS_CONSTRUCT,
        &args![this.addr() + BUILDER_DETAILED_INFOS],
    );
    e.call(
        MESHES_CONSTRUCT,
        &args![this.addr() + BUILDER_DETAILED_MESHES],
    );
    let manager = e.call(MEMORY_MANAGER, &args![]).u32();
    let heap = e.call(GET_THREAD_SCRAP_HEAP, &args![manager]).u32();
    e.mem.set_u32(this.addr() + BUILDER_SCRAP_HEAP, heap);
    e.mem.set_u32(this.addr() + BUILDER_REQUEST, request.addr());
    e.mem
        .set_u32(this.addr() + BUILDER_SOLUTION, solution.addr());
    e.mem.set_u8(this.addr() + BUILDER_PATH_UPDATE, 0);
    this
}

// Translated from 006c8dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `PathBuilder::FindTriangleForLocationWaterFilter`
/// constructor: the base constructor ([`fn_006c8df0`]), then its own
/// vtable (`0106c92c`). Returns `this`.
pub fn fn_006c8dd0(e: &mut Engine, this: Ptr) -> Ptr {
    fn_006c8df0(e, this);
    e.mem.set_u32(this.addr(), WATER_FILTER_VTABLE);
    this
}

// Translated from 006c8df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `FindTriangleForLocationFilter`
/// constructor: stores its vtable (`0106c934`, a pure virtual slot).
/// Returns `this`.
pub fn fn_006c8df0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), FILTER_VTABLE);
    this
}

// Translated from 006c8e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: clears the triangle data of the request's
/// origin and destination. When the solution has no virtual nodes yet
/// (`005a4320`), it resolves both locations' navmesh info through the
/// water filter, takes each one's best navmesh info when that succeeded
/// (else 0) and builds the path between them ([`fn_006c8f50`]); it fails
/// when that fails and the solution is still empty. With virtual nodes it
/// only resolves both locations again. Then the result is
/// `006ca0e0(this, 0, 0)`.
pub fn fn_006c8e10(e: &mut Engine, this: Ptr) -> bool {
    let from = origin(e, this);
    e.call(CLEAR_TRIANGLE_DATA, &args![from]);
    let to = destination(e, this);
    e.call(CLEAR_TRIANGLE_DATA, &args![to]);
    let filter = this.addr() + BUILDER_WATER_FILTER;
    let solution = builder_word(e, this, BUILDER_SOLUTION);
    if e.call(SOLUTION_COUNT_005A4320, &args![solution]).u32() == 0 {
        let mut from_info = 0u32;
        let mut to_info = 0u32;
        let from = origin(e, this);
        if e.call(RESOLVE_NAV_MESH_INFO, &args![from, filter]).bool() {
            let from = origin(e, this);
            from_info = e.call(GET_BEST_NAV_MESH_INFO, &args![from]).u32();
        }
        let to = destination(e, this);
        if e.call(RESOLVE_NAV_MESH_INFO, &args![to, filter]).bool() {
            let to = destination(e, this);
            to_info = e.call(GET_BEST_NAV_MESH_INFO, &args![to]).u32();
        }
        if !fn_006c8f50(e, this, from_info, to_info) {
            let solution = builder_word(e, this, BUILDER_SOLUTION);
            if e.call(SOLUTION_COUNT_005A4320, &args![solution]).u32() == 0 {
                return false;
            }
        }
    } else {
        let from = origin(e, this);
        e.call(RESOLVE_NAV_MESH_INFO, &args![from, filter]);
        let to = destination(e, this);
        e.call(RESOLVE_NAV_MESH_INFO, &args![to, filter]);
    }
    e.call(BUILDER_006CA0E0, &args![this, 0u32, 0u32]).bool()
}

/// Both ends have a best navmesh info.
fn both_have_nav_mesh_info(e: &mut Engine, this: Ptr) -> bool {
    let from = origin(e, this);
    if e.call(GET_BEST_NAV_MESH_INFO, &args![from]).u32() == 0 {
        return false;
    }
    let to = destination(e, this);
    e.call(GET_BEST_NAV_MESH_INFO, &args![to]).u32() != 0
}

// Translated from 006c8f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: builds the path between two navmesh infos.
/// With a null one it only calls `00408d60(011d7450)`; with the same one
/// it adds the request's origin and destination as virtual nodes
/// ([`fn_006c90d0`]); otherwise `PathBuilder::BuildNavMeshInfoPath`. When
/// that did not build a path, it fails if both ends are in the same
/// worldspace (or, outside worldspaces, the same cell) and both have a
/// navmesh info; otherwise it runs `006cefa0(VirtualPathingNodes, 0)` and
/// returns `PathBuilder::BuildTeleportDoorPath`. `RET 8`.
pub fn fn_006c8f50(e: &mut Engine, this: Ptr, from_info: u32, to_info: u32) -> bool {
    let mut built = false;
    if from_info == 0 || to_info == 0 {
        e.call(FN_00408D60, &args![OBJECT_011D7450]);
    } else if from_info == to_info {
        let solution = Ptr::<PathingSolution>::new(builder_word(e, this, BUILDER_SOLUTION));
        let from = origin(e, this);
        fn_006c90d0(e, solution, Ptr::new(from));
        let solution = Ptr::<PathingSolution>::new(builder_word(e, this, BUILDER_SOLUTION));
        let to = destination(e, this);
        fn_006c90d0(e, solution, Ptr::new(to));
        built = true;
    } else if e
        .call(BUILD_NAV_MESH_INFO_PATH, &args![this, from_info, to_info])
        .bool()
    {
        built = true;
    }
    if built {
        return true;
    }

    let from = origin(e, this);
    if e.call(WORD_AT_1C, &args![from]).u32() != 0 {
        let from = origin(e, this);
        let from_space = e.call(WORD_AT_1C, &args![from]).u32();
        let to = destination(e, this);
        let to_space = e.call(WORD_AT_1C, &args![to]).u32();
        if from_space == to_space && both_have_nav_mesh_info(e, this) {
            return false;
        }
    } else {
        let from = origin(e, this);
        let from_cell = e.call(GET_CELL, &args![from]).u32();
        let to = destination(e, this);
        let to_cell = e.call(GET_CELL, &args![to]).u32();
        if from_cell == to_cell && both_have_nav_mesh_info(e, this) {
            return false;
        }
    }
    let solution = builder_word(e, this, BUILDER_SOLUTION);
    let virtual_nodes = solution + PathingSolution::VirtualPathingNodes.off;
    e.call(VIRTUAL_NODES_006CEFA0, &args![virtual_nodes, 0u32]);
    e.call(BUILD_TELEPORT_DOOR_PATH, &args![this]).bool()
}

// Translated from 006c90d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: adds a virtual node for `location` to the
/// solution: reserves a slot (`006cef10`), takes its address (`006e8f40`)
/// and, unless placement returns 0, constructs the 0x30-byte node there
/// (`006f4d00(node, location)`). Returns the new node's index. `RET 4`.
// Translation incomplete: 006c90d0 - FS:[0] exception frame (unwinding of the placement allocation) is not translated.
pub fn fn_006c90d0(e: &mut Engine, this: Ptr<PathingSolution>, location: Ptr) -> u32 {
    let nodes = this.at(PathingSolution::VirtualPathingNodes);
    let index = e.call(VIRTUAL_NODES_ADD_SLOT, &args![nodes]).u32();
    let slot = e.call(VIRTUAL_NODES_AT, &args![nodes, index]).u32();
    let node = e.call(PLACEMENT, &args![0x30u32, slot]).u32();
    if node != 0 {
        e.call(VIRTUAL_NODE_CONSTRUCT, &args![node, location]);
    }
    index
}

// Translated from 006c9170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: updates the solution from virtual node
/// `index`. `006c9fc0(this, index, &first, &last)` (both -1 first) finds the
/// range; when it fails the result is 0 for a solution without virtual
/// nodes, else the solution's loaded range is cleared ([`fn_006c93a0`]) and
/// the result is 1. Without virtual nodes the result is 2 when
/// `006ca0e0(this, 0, index)` succeeds, else -1. Otherwise, when the
/// solution's `iLastLoadedVirtualNodeIndex` is already at or past `last`
/// the result is 0; else node `index` gets a location built from `point`
/// and the node's navmesh info (+4), the loaded range becomes
/// `index..=last`, `bPathUpdate` is set, and the result is -1 when
/// `006ca850(this, 0)` fails or `006b05d0(solution)` succeeds, else 2 after
/// `006c9380(solution, old count - 1)`. `RET 8`.
// Translation incomplete: 006c9170 - FS:[0] exception frame (unwinding of the local PathingLocation) is not translated.
pub fn fn_006c9170(e: &mut Engine, this: Ptr, index: u32, point: Ptr) -> i32 {
    e.with_stack(8, |e, range| {
        let first = range.addr();
        let last = range.addr() + 4;
        e.mem.set_u32(first, u32::MAX);
        e.mem.set_u32(last, u32::MAX);
        let found = e
            .call(BUILDER_006C9FC0, &args![this, index, first, last])
            .bool();
        let solution = builder_word(e, this, BUILDER_SOLUTION);
        if !found {
            if e.call(SOLUTION_COUNT_008B6800, &args![solution]).u32() == 0 {
                return 0;
            }
            fn_006c93a0(e, Ptr::new(solution));
            return 1;
        }
        if e.call(SOLUTION_COUNT_008B6800, &args![solution]).u32() == 0 {
            return if e.call(BUILDER_006CA0E0, &args![this, 0u32, index]).bool() {
                2
            } else {
                -1
            };
        }
        let last_index = e.mem.u32(last) as i32;
        let solution = builder_word(e, this, BUILDER_SOLUTION);
        if e.call(WORD_AT_1C, &args![solution]).i32() >= last_index {
            return 0;
        }
        let solution = Ptr::<PathingSolution>::new(builder_word(e, this, BUILDER_SOLUTION));
        e.set(
            solution,
            PathingSolution::iFirstLoadedVirtualNodeIndex,
            index as i32,
        );
        e.with_stack(LOCATION_SIZE, |e, location| {
            let node = e.call(SOLUTION_VIRTUAL_NODE, &args![solution, index]).u32();
            let info = e.call(PLUS_4, &args![node]).u32();
            e.call(LOCATION_CONSTRUCT, &args![location, point, info]);
            let node = e.call(SOLUTION_VIRTUAL_NODE, &args![solution, index]).u32();
            e.call(VIRTUAL_NODE_SET_LOCATION, &args![node, location]);
            let count = e.call(SOLUTION_COUNT_008B6800, &args![solution]).u32();
            let last_old = count.wrapping_sub(1);
            fn_006c9350(e, solution.cast());
            e.set(
                solution,
                PathingSolution::iLastLoadedVirtualNodeIndex,
                last_index,
            );
            e.mem.set_u8(this.addr() + BUILDER_PATH_UPDATE, 1);
            if !e.call(BUILDER_006CA850, &args![this, 0u32]).bool() {
                e.call(LOCATION_DESTROY, &args![location]);
                return -1;
            }
            if e.call(SOLUTION_006B05D0, &args![solution]).bool() {
                e.call(LOCATION_DESTROY, &args![location]);
                return -1;
            }
            fn_006c9380(e, solution.cast(), last_old);
            e.call(LOCATION_DESTROY, &args![location]);
            2
        })
    })
}

// Translated from 006c9350 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: on a `PathingSolution`:
/// `006cf000(CurrentPathingNodes, size - 1, 1)` with the size from
/// `0044ddc0`.
pub fn fn_006c9350(e: &mut Engine, this: Ptr) {
    let nodes = this.addr() + PathingSolution::CurrentPathingNodes.off;
    let size = e.call(ARRAY_SIZE, &args![nodes]).u32();
    e.call(
        PATHING_NODES_006CF000,
        &args![nodes, size.wrapping_sub(1), 1u32],
    );
}

// Translated from 006c9380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: on a `PathingSolution`:
/// `006cf000(CurrentPathingNodes, index, 1)`. `RET 4`.
pub fn fn_006c9380(e: &mut Engine, this: Ptr, index: u32) {
    let nodes = this.addr() + PathingSolution::CurrentPathingNodes.off;
    e.call(PATHING_NODES_006CF000, &args![nodes, index, 1u32]);
}

// Translated from 006c93a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: on a `PathingSolution`:
/// `006cf120(CurrentPathingNodes, 1)`, then both loaded virtual node
/// indices become -1.
pub fn fn_006c93a0(e: &mut Engine, this: Ptr) {
    let solution = Ptr::<PathingSolution>::new(this.addr());
    let nodes = solution.at(PathingSolution::CurrentPathingNodes);
    e.call(PATHING_NODES_006CF120, &args![nodes, 1u32]);
    e.set(solution, PathingSolution::iFirstLoadedVirtualNodeIndex, -1);
    e.set(solution, PathingSolution::iLastLoadedVirtualNodeIndex, -1);
}

// Translated from 006c93d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: clears the builder's solution's loaded range
/// ([`fn_006c93a0`]) and returns 1.
pub fn fn_006c93d0(e: &mut Engine, this: Ptr) -> u32 {
    let solution = builder_word(e, this, BUILDER_SOLUTION);
    fn_006c93a0(e, Ptr::new(solution));
    1
}

// Translated from 006c93f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `006c9fc0(this, index, &first, &last)` (both
/// -1 first); 0 when it fails. Otherwise clears the triangle data of every
/// virtual node's location (`PathingLocation::ClearTriangleData` on node +4,
/// for each node below the solution's count `005a4320`), runs `006f5020`
/// on the nodes `first..=last`, and returns 2 when
/// `006ca0e0(this, 0, index)` succeeds, else -1. `RET 4`.
pub fn fn_006c93f0(e: &mut Engine, this: Ptr, index: u32) -> i32 {
    e.with_stack(8, |e, range| {
        let first = range.addr();
        let last = range.addr() + 4;
        e.mem.set_u32(first, u32::MAX);
        e.mem.set_u32(last, u32::MAX);
        if !e
            .call(BUILDER_006C9FC0, &args![this, index, first, last])
            .bool()
        {
            return 0;
        }
        let mut i = 0u32;
        loop {
            let solution = builder_word(e, this, BUILDER_SOLUTION);
            if i >= e.call(SOLUTION_COUNT_005A4320, &args![solution]).u32() {
                break;
            }
            let node = e.call(SOLUTION_VIRTUAL_NODE, &args![solution, i]).u32();
            let location = e.call(PLUS_4, &args![node]).u32();
            e.call(CLEAR_TRIANGLE_DATA, &args![location]);
            i = i.wrapping_add(1);
        }
        let mut j = e.mem.u32(first) as i32;
        while j <= e.mem.u32(last) as i32 {
            let solution = builder_word(e, this, BUILDER_SOLUTION);
            let node = e
                .call(SOLUTION_VIRTUAL_NODE, &args![solution, j as u32])
                .u32();
            e.call(VIRTUAL_NODE_006F5020, &args![node]);
            j = j.wrapping_add(1);
        }
        if e.call(BUILDER_006CA0E0, &args![this, 0u32, index]).bool() {
            2
        } else {
            -1
        }
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006c8d00, fn_006c8d00(Ptr, u32) -> Ptr),
        entry!(0x006c8d30, fn_006c8d30(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x006c8dd0, fn_006c8dd0(Ptr) -> Ptr),
        entry!(0x006c8df0, fn_006c8df0(Ptr) -> Ptr),
        entry!(0x006c8e10, fn_006c8e10(Ptr) -> bool),
        entry!(0x006c8f50, fn_006c8f50(Ptr, u32, u32) -> bool),
        entry!(0x006c90d0, fn_006c90d0(Ptr<PathingSolution>, Ptr) -> u32),
        entry!(0x006c9170, fn_006c9170(Ptr, u32, Ptr) -> i32),
        entry!(0x006c9350, fn_006c9350(Ptr)),
        entry!(0x006c9380, fn_006c9380(Ptr, u32)),
        entry!(0x006c93a0, fn_006c93a0(Ptr)),
        entry!(0x006c93d0, fn_006c93d0(Ptr) -> u32),
        entry!(0x006c93f0, fn_006c93f0(Ptr, u32) -> i32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type CallLog = Vec<(u32, Vec<u32>)>;

    fn calls(log: &CallLog, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn first_word(_: &mut Engine, a: &[u32]) -> Ret {
        a[0].into_ret()
    }

    fn yes(_: &mut Engine, _: &[u32]) -> Ret {
        1u32.into_ret()
    }

    fn no(_: &mut Engine, _: &[u32]) -> Ret {
        0u32.into_ret()
    }

    /// A builder with a request at 0x5000 (origin 0x500c, destination
    /// 0x5034) and a solution object; the request getters are the real
    /// `+0x0C` / `+0x34`.
    fn builder(e: &mut Engine) -> (Ptr, Ptr) {
        let this = Ptr::new(e.mem.alloc(0x34));
        let solution = Ptr::new(e.mem.alloc(0x44));
        e.mem.set_u32(this.addr(), 0x5000);
        e.mem.set_u32(this.addr() + 4, solution.addr());
        e.register(REQUEST_ORIGIN, |_, a| (a[0] + 0xc).into_ret());
        e.register(REQUEST_DESTINATION, |_, a| (a[0] + 0x34).into_ret());
        (this, solution)
    }

    #[test]
    fn scalar_deleting_destructor_frees_only_with_bit_0() {
        let mut e = Engine::new();
        e.register(DESTRUCTOR_006C7E00, nothing);
        e.register(OPERATOR_DELETE, nothing);
        for (flags, frees) in [(0u32, 0usize), (1, 1), (2, 0), (3, 1)] {
            e.call_log = Some(vec![]);
            let back = e.call(0x006c8d00, &args![0x4440u32, flags]).u32();
            assert_eq!(back, 0x4440);
            let log = e.call_log.take().unwrap();
            assert_eq!(calls(&log, DESTRUCTOR_006C7E00), vec![vec![0x4440]]);
            assert_eq!(calls(&log, OPERATOR_DELETE).len(), frees);
        }
    }

    #[test]
    fn constructor_builds_members_and_stores_request_solution_and_heap() {
        let mut e = Engine::new();
        e.register(INFOS_CONSTRUCT, nothing);
        e.register(MESHES_CONSTRUCT, nothing);
        e.register(MEMORY_MANAGER, |_, _| 0x7700u32.into_ret());
        e.register(GET_THREAD_SCRAP_HEAP, |_, a| (a[0] + 1).into_ret());
        let this = e.mem.alloc(0x34);
        e.mem.set_u8(this + 0x10, 1);
        e.call_log = Some(vec![]);
        let back = e.call(0x006c8d30, &args![this, 0x1234u32, 0x5678u32]).u32();
        assert_eq!(back, this);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, INFOS_CONSTRUCT), vec![vec![this + 0x14]]);
        assert_eq!(calls(&log, MESHES_CONSTRUCT), vec![vec![this + 0x24]]);
        assert_eq!(calls(&log, GET_THREAD_SCRAP_HEAP), vec![vec![0x7700]]);
        assert_eq!(e.mem.u32(this), 0x1234);
        assert_eq!(e.mem.u32(this + 4), 0x5678);
        assert_eq!(e.mem.u32(this + 8), 0x7701);
        assert_eq!(e.mem.u32(this + 0xc), WATER_FILTER_VTABLE);
        assert_eq!(e.mem.u8(this + 0x10), 0);
    }

    #[test]
    fn filter_constructors_store_their_vtables() {
        let mut e = Engine::new();
        let this = e.mem.alloc(4);
        assert_eq!(e.call(0x006c8df0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), FILTER_VTABLE);
        assert_eq!(e.call(0x006c8dd0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), WATER_FILTER_VTABLE);
    }

    /// Doubles for [`fn_006c8e10`] and [`fn_006c8f50`]'s callees.
    fn path_engine() -> (Engine, Ptr, Ptr) {
        let mut e = Engine::new();
        let (this, solution) = builder(&mut e);
        for addr in [
            CLEAR_TRIANGLE_DATA,
            FN_00408D60,
            VIRTUAL_NODES_006CEFA0,
            VIRTUAL_NODE_CONSTRUCT,
        ] {
            e.register(addr, nothing);
        }
        e.register(VIRTUAL_NODES_ADD_SLOT, |_, _| 3u32.into_ret());
        e.register(VIRTUAL_NODES_AT, |_, a| (0x9000 + a[1] * 0x30).into_ret());
        e.register(PLACEMENT, |_, a| a[1].into_ret());
        e.register(WORD_AT_1C, first_word);
        e.register(BUILD_TELEPORT_DOOR_PATH, yes);
        (e, this, solution)
    }

    #[test]
    fn prepare_without_virtual_nodes_builds_between_best_infos() {
        let (mut e, this, _) = path_engine();
        e.register(SOLUTION_COUNT_005A4320, no);
        e.register(RESOLVE_NAV_MESH_INFO, |_, a| (a[0] == 0x500c).into_ret());
        e.register(GET_BEST_NAV_MESH_INFO, |_, a| (a[0] + 0x100).into_ret());
        e.register(BUILD_NAV_MESH_INFO_PATH, yes);
        e.register(BUILDER_006CA0E0, yes);
        e.call_log = Some(vec![]);
        assert!(e.call(0x006c8e10, &args![this]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, CLEAR_TRIANGLE_DATA),
            vec![vec![0x500c], vec![0x5034]]
        );
        assert_eq!(
            calls(&log, RESOLVE_NAV_MESH_INFO),
            vec![
                vec![0x500c, this.addr() + 0xc],
                vec![0x5034, this.addr() + 0xc]
            ]
        );
        // Only the origin resolved: the destination's info stays 0, so the
        // path builder is not asked and `00408d60` is; the different
        // worldspaces then go to the teleport door path, which succeeds.
        assert!(calls(&log, BUILD_NAV_MESH_INFO_PATH).is_empty());
        assert_eq!(calls(&log, FN_00408D60), vec![vec![OBJECT_011D7450]]);
        assert_eq!(calls(&log, BUILDER_006CA0E0), vec![vec![this.addr(), 0, 0]]);
        assert_eq!(
            calls(&log, BUILD_TELEPORT_DOOR_PATH),
            vec![vec![this.addr()]]
        );
    }

    #[test]
    fn prepare_fails_when_no_path_and_still_no_nodes() {
        let (mut e, this, _) = path_engine();
        e.register(SOLUTION_COUNT_005A4320, no);
        e.register(RESOLVE_NAV_MESH_INFO, yes);
        e.register(GET_BEST_NAV_MESH_INFO, |_, a| (a[0] + 0x100).into_ret());
        e.register(BUILD_NAV_MESH_INFO_PATH, no);
        e.register(WORD_AT_1C, |_, _| 0x44u32.into_ret());
        e.register(BUILDER_006CA0E0, yes);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x006c8e10, &args![this]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, BUILD_NAV_MESH_INFO_PATH),
            vec![vec![this.addr(), 0x510c, 0x5134]]
        );
        assert!(calls(&log, BUILDER_006CA0E0).is_empty());
    }

    #[test]
    fn prepare_with_virtual_nodes_only_resolves() {
        let (mut e, this, _) = path_engine();
        e.register(SOLUTION_COUNT_005A4320, yes);
        e.register(RESOLVE_NAV_MESH_INFO, no);
        e.register(BUILDER_006CA0E0, no);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x006c8e10, &args![this]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, RESOLVE_NAV_MESH_INFO).len(), 2);
        assert!(calls(&log, GET_BEST_NAV_MESH_INFO).is_empty());
        assert_eq!(calls(&log, BUILDER_006CA0E0).len(), 1);
    }

    #[test]
    fn build_with_one_info_adds_origin_and_destination_nodes() {
        let (mut e, this, solution) = path_engine();
        e.call_log = Some(vec![]);
        assert!(e.call(0x006c8f50, &args![this, 0x77u32, 0x77u32]).bool());
        let log = e.call_log.take().unwrap();
        let nodes = solution.addr() + 8;
        assert_eq!(
            calls(&log, VIRTUAL_NODES_ADD_SLOT),
            vec![vec![nodes], vec![nodes]]
        );
        assert_eq!(
            calls(&log, VIRTUAL_NODE_CONSTRUCT),
            vec![vec![0x9090, 0x500c], vec![0x9090, 0x5034]]
        );
    }

    #[test]
    fn build_falls_back_to_teleport_doors_unless_same_space_with_meshes() {
        // Different worldspaces: the teleport door path decides.
        let (mut e, this, solution) = path_engine();
        e.register(BUILD_NAV_MESH_INFO_PATH, no);
        e.register(WORD_AT_1C, |_, a| a[0].into_ret());
        e.register(BUILD_TELEPORT_DOOR_PATH, yes);
        e.call_log = Some(vec![]);
        assert!(e.call(0x006c8f50, &args![this, 1u32, 2u32]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, VIRTUAL_NODES_006CEFA0),
            vec![vec![solution.addr() + 8, 0]]
        );
        e.register(BUILD_TELEPORT_DOOR_PATH, no);
        assert!(!e.call(0x006c8f50, &args![this, 1u32, 2u32]).bool());

        // Same worldspace, both with navmesh info: fails at once.
        e.register(WORD_AT_1C, |_, _| 9u32.into_ret());
        e.register(GET_BEST_NAV_MESH_INFO, yes);
        e.register(BUILD_TELEPORT_DOOR_PATH, yes);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x006c8f50, &args![this, 1u32, 2u32]).bool());
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, BUILD_TELEPORT_DOOR_PATH).is_empty());
        // Same worldspace but the destination has none: teleport doors.
        e.register(GET_BEST_NAV_MESH_INFO, |_, a| (a[0] == 0x500c).into_ret());
        assert!(e.call(0x006c8f50, &args![this, 1u32, 2u32]).bool());

        // No worldspace: the cells are compared instead.
        e.register(WORD_AT_1C, no);
        e.register(GET_CELL, |_, _| 5u32.into_ret());
        e.register(GET_BEST_NAV_MESH_INFO, yes);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x006c8f50, &args![this, 1u32, 2u32]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, GET_CELL), vec![vec![0x500c], vec![0x5034]]);
        e.register(GET_CELL, |_, a| a[0].into_ret());
        assert!(e.call(0x006c8f50, &args![this, 1u32, 2u32]).bool());
        // The navmesh info path succeeding skips all of it.
        e.register(BUILD_NAV_MESH_INFO_PATH, yes);
        e.register(BUILD_TELEPORT_DOOR_PATH, no);
        assert!(e.call(0x006c8f50, &args![this, 1u32, 2u32]).bool());
    }

    #[test]
    fn add_virtual_node_constructs_unless_placement_fails() {
        let (mut e, _, solution) = path_engine();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006c90d0, &args![solution, 0x4321u32]).u32(), 3);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, VIRTUAL_NODES_AT),
            vec![vec![solution.addr() + 8, 3]]
        );
        assert_eq!(calls(&log, PLACEMENT), vec![vec![0x30, 0x9090]]);
        assert_eq!(
            calls(&log, VIRTUAL_NODE_CONSTRUCT),
            vec![vec![0x9090, 0x4321]]
        );
        e.register(PLACEMENT, no);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006c90d0, &args![solution, 0x4321u32]).u32(), 3);
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, VIRTUAL_NODE_CONSTRUCT).is_empty());
    }

    /// Doubles for [`fn_006c9170`] and [`fn_006c93f0`]: `006c9fc0` writes
    /// first = 2, last = 4 and answers `found`.
    fn update_engine(found: bool, count: u32) -> (Engine, Ptr, Ptr) {
        let mut e = Engine::new();
        let (this, solution) = builder(&mut e);
        e.register_double(BUILDER_006C9FC0, move |e, a| {
            e.mem.set_u32(a[2], 2);
            e.mem.set_u32(a[3], 4);
            (found as u32).into_ret()
        });
        e.register_double(SOLUTION_COUNT_008B6800, move |_, _| count.into_ret());
        e.register_double(SOLUTION_COUNT_005A4320, move |_, _| count.into_ret());
        e.register(WORD_AT_1C, |e, a| e.mem.u32(a[0] + 0x1c).into_ret());
        e.register(SOLUTION_VIRTUAL_NODE, |_, a| {
            (0x9000 + a[1] * 0x30).into_ret()
        });
        e.register(PLUS_4, |_, a| (a[0] + 4).into_ret());
        for addr in [
            LOCATION_CONSTRUCT,
            VIRTUAL_NODE_SET_LOCATION,
            LOCATION_DESTROY,
            PATHING_NODES_006CF000,
            PATHING_NODES_006CF120,
            CLEAR_TRIANGLE_DATA,
            VIRTUAL_NODE_006F5020,
        ] {
            e.register(addr, nothing);
        }
        e.register(ARRAY_SIZE, |_, _| 6u32.into_ret());
        e.register(BUILDER_006CA0E0, yes);
        e.register(BUILDER_006CA850, yes);
        e.register(SOLUTION_006B05D0, no);
        (e, this, solution)
    }

    #[test]
    fn update_without_range_clears_or_returns_0() {
        let (mut e, this, _) = update_engine(false, 0);
        assert_eq!(e.call(0x006c9170, &args![this, 1u32, 0x300u32]).i32(), 0);
        let (mut e, this, solution) = update_engine(false, 3);
        e.mem.set_u32(solution.addr() + 0x18, 7);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006c9170, &args![this, 1u32, 0x300u32]).i32(), 1);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, PATHING_NODES_006CF120),
            vec![vec![solution.addr() + 0x20, 1]]
        );
        assert_eq!(e.mem.u32(solution.addr() + 0x18), u32::MAX);
        assert_eq!(e.mem.u32(solution.addr() + 0x1c), u32::MAX);
    }

    #[test]
    fn update_with_empty_solution_asks_006ca0e0() {
        let (mut e, this, _) = update_engine(true, 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006c9170, &args![this, 1u32, 0x300u32]).i32(), 2);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, BUILDER_006CA0E0), vec![vec![this.addr(), 0, 1]]);
        e.register(BUILDER_006CA0E0, no);
        assert_eq!(e.call(0x006c9170, &args![this, 1u32, 0x300u32]).i32(), -1);
    }

    #[test]
    fn update_loads_the_range_and_reports_2() {
        let (mut e, this, solution) = update_engine(true, 5);
        // Already loaded up to 4 (= last): nothing to do.
        e.mem.set_u32(solution.addr() + 0x1c, 4);
        assert_eq!(e.call(0x006c9170, &args![this, 1u32, 0x300u32]).i32(), 0);

        e.mem.set_u32(solution.addr() + 0x1c, 3);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006c9170, &args![this, 1u32, 0x300u32]).i32(), 2);
        let log = e.call_log.take().unwrap();
        let construct = calls(&log, LOCATION_CONSTRUCT);
        assert_eq!(construct.len(), 1);
        assert_eq!(&construct[0][1..], &[0x300, 0x9034]);
        let set = calls(&log, VIRTUAL_NODE_SET_LOCATION);
        assert_eq!(set, vec![vec![0x9030, construct[0][0]]]);
        assert_eq!(
            calls(&log, PATHING_NODES_006CF000),
            vec![
                vec![solution.addr() + 0x20, 5, 1],
                vec![solution.addr() + 0x20, 4, 1]
            ]
        );
        assert_eq!(calls(&log, LOCATION_DESTROY), vec![vec![construct[0][0]]]);
        assert_eq!(e.mem.u32(solution.addr() + 0x18), 1);
        assert_eq!(e.mem.u32(solution.addr() + 0x1c), 4);
        assert_eq!(e.mem.u8(this.addr() + 0x10), 1);
        assert_eq!(calls(&log, BUILDER_006CA850), vec![vec![this.addr(), 0]]);
    }

    #[test]
    fn update_fails_when_006ca850_fails_or_006b05d0_succeeds() {
        let (mut e, this, _) = update_engine(true, 5);
        e.register(BUILDER_006CA850, no);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006c9170, &args![this, 1u32, 0x300u32]).i32(), -1);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, LOCATION_DESTROY).len(), 1);
        assert_eq!(calls(&log, PATHING_NODES_006CF000).len(), 1);

        let (mut e, this, _) = update_engine(true, 5);
        e.register(SOLUTION_006B05D0, yes);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006c9170, &args![this, 1u32, 0x300u32]).i32(), -1);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, LOCATION_DESTROY).len(), 1);
        assert_eq!(calls(&log, PATHING_NODES_006CF000).len(), 1);
    }

    #[test]
    fn current_node_helpers_call_006cf000_and_006cf120() {
        let (mut e, _, solution) = update_engine(true, 5);
        let nodes = solution.addr() + 0x20;
        e.call_log = Some(vec![]);
        e.call(0x006c9350, &args![solution]);
        e.call(0x006c9380, &args![solution, 9u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, ARRAY_SIZE), vec![vec![nodes]]);
        assert_eq!(
            calls(&log, PATHING_NODES_006CF000),
            vec![vec![nodes, 5, 1], vec![nodes, 9, 1]]
        );
    }

    #[test]
    fn clear_loaded_range_through_the_builder_returns_1() {
        let (mut e, this, solution) = update_engine(true, 5);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006c93d0, &args![this]).u32(), 1);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, PATHING_NODES_006CF120),
            vec![vec![solution.addr() + 0x20, 1]]
        );
        assert_eq!(e.mem.u32(solution.addr() + 0x18), u32::MAX);
        assert_eq!(e.mem.u32(solution.addr() + 0x1c), u32::MAX);
    }

    #[test]
    fn reset_clears_every_node_and_resets_the_range() {
        let (mut e, this, _) = update_engine(false, 3);
        assert_eq!(e.call(0x006c93f0, &args![this, 1u32]).i32(), 0);

        let (mut e, this, _) = update_engine(true, 3);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006c93f0, &args![this, 1u32]).i32(), 2);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, CLEAR_TRIANGLE_DATA),
            vec![vec![0x9004], vec![0x9034], vec![0x9064]]
        );
        assert_eq!(
            calls(&log, VIRTUAL_NODE_006F5020),
            vec![vec![0x9060], vec![0x9090], vec![0x90c0]]
        );
        assert_eq!(calls(&log, BUILDER_006CA0E0)[0][1..], [0, 1]);
        e.register(BUILDER_006CA0E0, no);
        assert_eq!(e.call(0x006c93f0, &args![this, 1u32]).i32(), -1);
    }
}
