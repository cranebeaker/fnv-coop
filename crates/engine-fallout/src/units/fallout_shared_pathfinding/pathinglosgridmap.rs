//! `fallout shared/pathfinding/pathinglosgridmap.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `006a62f0` (unnamed in the engine map): the base body the wrapper forwards to;
/// `thiscall`, `this` plus one stack word (`RET 4`).
const OFFSET_TARGET_006A62F0: u32 = 0x006a_62f0;

// Translated from 006e0ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (callers `SaveGame` and `LoadGame` of the pathing LOS grid map):
/// forwards `this` and one word to `006a62f0`, which returns the word at `this+4` plus the
/// argument (`ADD EAX,[EBP+8]`). The wrapper returns that address.
pub fn fn_006e0ed0(e: &mut Engine, this: Ptr, offset: u32) -> Ptr {
    e.call(OFFSET_TARGET_006A62F0, &args![this, offset]).ptr()
}

/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@D$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0106CF0C: u32 = 0x0106_cf0c;
/// `008454f0` (unnamed in the engine map): `thiscall`, `this` plus one stack word (`RET 4`). When the
/// buffer pointer (+4) is set it runs the instance's element destroy step; with the word non-zero it
/// then calls the array's virtual slot +8 on the buffer and clears the buffer pointer and the reserved
/// size (+0xc); finally it clears the size (+8).
const ARRAY_RELEASE_008454F0: u32 = 0x0084_54f0;

// Translated from 006e10a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@D$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_006e10a0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0106CF0C);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

// ---- Batch n032: the LOS grid map, its grids and cells ----------------------

layout! {
    /// `PathingLOSGridCell` (Xbox PDB), 0x1C bytes (allocated with that size
    /// by `PathingLOSGrid::LoadGame`).
    pub struct PathingLOSGridCell: 0x1C {
        /// `bAllSeen` (Xbox PDB).
        0x00 bAllSeen: u8,
        /// `SeenTriangleList` (Xbox PDB): a `BSSimpleArray<char,1024>`.
        0x04 SeenTriangleList: Inline<crate::types::BSSimpleArray>,
        /// `pNavMeshInfo` (Xbox PDB).
        0x14 pNavMeshInfo: Ptr,
        /// `iNavMeshGridCellIndex` (Xbox PDB).
        0x18 iNavMeshGridCellIndex: u32,
    }

    /// `PathingLOSGrid` (Xbox PDB), 0x14 bytes (allocated with that size by
    /// `PathingLOSGridMap::LoadGame`).
    pub struct PathingLOSGrid: 0x14 {
        /// `LOSGridCells` (Xbox PDB): a `BSSimpleArray<PathingLOSGridCell *,1024>`.
        0x00 LOSGridCells: Inline<crate::types::BSSimpleArray>,
        /// `pNavMeshInfo` (Xbox PDB).
        0x10 pNavMeshInfo: Ptr,
    }

    /// `PathingLOSGridMap` (Xbox PDB), 0x54 bytes (the size its deleting
    /// destructor frees). `Center` (+0x1C) is a `PathingLOSGridMap`'s
    /// `PathingLocation` (0x28 bytes) and `Lock` (+0x4C) a `BSReadWriteLock`.
    pub struct PathingLOSGridMap: 0x54 {
        /// `NavMeshToLOSGridMap` (Xbox PDB): `NiTMap<unsigned int,PathingLOSGrid *>`.
        0x08 NavMeshToLOSGridMap: Inline<crate::types::NiTMap>,
        /// `fPercentSeen` (Xbox PDB).
        0x18 fPercentSeen: f32,
        /// `fRadius` (Xbox PDB).
        0x44 fRadius: f32,
        /// `fSightRadius` (Xbox PDB).
        0x48 fSightRadius: f32,
    }
}

/// `PathingLOSGridMap::Center` (Xbox PDB), a `PathingLocation`.
const GRID_MAP_CENTER: u32 = 0x1c;
/// `PathingLOSGridMap::Lock` (Xbox PDB), a `BSReadWriteLock`.
const GRID_MAP_LOCK: u32 = 0x4c;

/// The `PathingLOSGridMap` vtable (by its RTTI).
const GRID_MAP_VTABLE: u32 = 0x0106_ceac;
/// `NiTMap<unsigned int,PathingLOSGrid *>`'s vtable and its
/// `NiTMapBase<DFALL<NiTMapItem<unsigned int,PathingLOSGrid *>>,...>` base's
/// (by their RTTI).
const GRID_TABLE_VTABLE: u32 = 0x0106_ceb8;
const GRID_TABLE_BASE_VTABLE: u32 = 0x0106_ced8;
/// `BSSimpleArray<PathingLOSGridCell *,1024>`'s vtable (by its RTTI).
const CELL_ARRAY_VTABLE: u32 = 0x0106_cef8;

/// `BGSSaveGameBuffer` raw save (`00865e50`, `thiscall`: data, size, 0).
const SAVE_DATA: u32 = 0x0086_5e50;
/// `BGSSaveGameBuffer::SaveFormID` (engine map), `thiscall`, `RET 8`.
const SAVE_FORM_ID: u32 = 0x0086_5db0;
/// `BGSSaveGameBuffer::SaveVariableSizedValue` (engine map), `thiscall`, `RET 4`.
const SAVE_VARIABLE_SIZED_VALUE: u32 = 0x0086_5f60;
/// `BGSSaveGameBuffer::StartVariableSizedValue` (engine map), `thiscall`:
/// returns a mark the matching `_ov2` save takes.
const START_VARIABLE_SIZED_VALUE: u32 = 0x0086_5f20;
/// `BGSSaveGameBuffer::SaveVariableSizedValue_ov2` (engine map), `thiscall`,
/// `RET 8`: (value, mark).
const SAVE_VARIABLE_SIZED_VALUE_AT: u32 = 0x0086_5ff0;
/// `BGSLoadGameBuffer` raw load (`00864980`, `thiscall`: data, size).
const LOAD_DATA: u32 = 0x0086_4980;
/// `BGSLoadGameBuffer::LoadVariableSizedValue` (engine map), `thiscall`.
const LOAD_VARIABLE_SIZED_VALUE: u32 = 0x0086_4a60;
/// `BGSLoadGameBuffer::LoadFormID_ov2` (engine map), `thiscall`, `RET 4`:
/// the form id into an out word.
const LOAD_FORM_ID: u32 = 0x0086_48e0;
/// `0044ddc0` (`thiscall`): the word at `this + 8` (a `BSSimpleArray`'s size).
const ARRAY_SIZE: u32 = 0x0044_ddc0;
/// `00877a30` (`thiscall`, `RET 4`): the address of element `i` of a
/// `BSSimpleArray` of words (`pBuffer + 4 * i`).
const WORD_ELEMENT: u32 = 0x0087_7a30;
/// `006a7ad0` (`thiscall`, `RET 4`): the address of an array element.
const ARRAY_ELEMENT: u32 = 0x006a_7ad0;
/// `00559450` (`thiscall`): the word at `this` (a pointer's target, a
/// navmesh info's first word).
const WORD_AT: u32 = 0x0055_9450;
/// The `NiPointer` constructors (`0042fb00`: null; `0042fa20`, `RET 4`: from a
/// pointer argument) and destructor (`0042fa40`).
const NI_POINTER_CONSTRUCT: u32 = 0x0042_fb00;
const NI_POINTER_CONSTRUCT_FROM: u32 = 0x0042_fa20;
const NI_POINTER_DESTRUCT: u32 = 0x0042_fa40;
/// `0069ad00` (`thiscall`, `RET 4`): the navmesh of a navmesh info into an
/// `NiPointer` out argument; answers whether there is one.
const NAV_MESH_INFO_GET_NAV_MESH: u32 = 0x0069_ad00;
/// `PathingLocation::GetNavMesh` (engine map), `thiscall`, `RET 4`.
const LOCATION_GET_NAV_MESH: u32 = 0x006d_d610;
/// `006dfba0` (named `NavMeshInfoMap::GetAllNavMeshInfo` by the engine map,
/// unconfirmed), `thiscall`, `RET 4`: called on a grid with an `NiPointer`.
const GRID_SET_NAV_MESH: u32 = 0x006d_fba0;
/// `006df230` / `006df440` (`thiscall`, `RET 4`): per-cell passes of the
/// grid's update over one argument.
const CELL_PASS_A: u32 = 0x006d_f230;
const CELL_PASS_B: u32 = 0x006d_f440;
/// `005c3420` (`thiscall`, `RET 4`): a location's point into an out
/// argument; returns its address.
const LOCATION_POINT: u32 = 0x005c_3420;
/// `00691240` (`thiscall`, `RET 4`): a navmesh's grid cell index for a point.
const NAV_MESH_CELL_INDEX: u32 = 0x0069_1240;
/// `006df5f0` (`thiscall`, `RET 4`): a cell's test for a location.
const CELL_TEST_LOCATION: u32 = 0x006d_f5f0;
/// `006df760` (`thiscall`): a location's triangle (16 bits, `0xFFFF` none).
const LOCATION_TRIANGLE: u32 = 0x006d_f760;
/// `00691350` (`thiscall`, `RET 8`): a navmesh's cell indices for a
/// triangle into an array.
const NAV_MESH_TRIANGLE_CELLS: u32 = 0x0069_1350;
/// `006df780` (`thiscall`, `RET 8`): a cell's update for a location and a flag.
const CELL_UPDATE_LOCATION: u32 = 0x006d_f780;
/// The `BSSimpleArray<unsigned int,1024>` (by its constructor's vtable) constructor (`00438660`) and destructor
/// (`00438690`), both `thiscall`.
const INDEX_ARRAY_CONSTRUCT: u32 = 0x0043_8660;
const INDEX_ARRAY_DESTRUCT: u32 = 0x0043_8690;
/// `TES::GetNavMeshInfoMap` (engine map) on the `TES` singleton (`011dea10`).
const TES_GET_NAV_MESH_INFO_MAP: u32 = 0x0045_af00;
const TES_SINGLETON: u32 = 0x011d_ea10;
/// `00693d70` (`thiscall`, `RET 4`): a navmesh info by form id.
const NAV_MESH_INFO_MAP_FIND: u32 = 0x0069_3d70;
/// `BSSimpleArray<TESObjectWEAP *,1024>::SetSize` (engine map; folded),
/// `thiscall`, `RET 8`: (size, shrink).
const WORD_ARRAY_SET_SIZE: u32 = 0x006f_2290;
/// `MemoryManager` allocation (`00401000`, `cdecl`: size).
const ALLOCATE: u32 = 0x0040_1000;
/// The game's `operator delete` (`00401030`, `cdecl`).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The heap free (`00aa1460`, `cdecl`: block, size).
const FREE_SIZED: u32 = 0x00aa_1460;
/// `00600ab0` (`thiscall`, `RET 4`): a cell's set-up with a navmesh info.
const CELL_SET_NAV_MESH_INFO: u32 = 0x0060_0ab0;
/// `NiRefObject` constructor (`004968b0`) and destructor body (`00496910`).
const REF_OBJECT_CONSTRUCT: u32 = 0x0049_68b0;
const REF_OBJECT_DESTRUCT: u32 = 0x0049_6910;
/// `PathingLocation::PathingLocation` (engine map), its destructor
/// (`004ff7e0`) and `PathingLocation::operator=` (engine map, `RET 4`).
const LOCATION_CONSTRUCT: u32 = 0x006d_cbb0;
const LOCATION_DESTRUCT: u32 = 0x004f_f7e0;
const LOCATION_ASSIGN: u32 = 0x006d_cce0;
/// The `BSReadWriteLock` constructor (`00aa5fc0`), `LockForWrite` (engine
/// map, `00aa6060`), the read lock (`00aa5ff0`) and the two unlocks
/// (`00aa62e0` after a write lock, `00aa62c0` after a read lock).
const LOCK_CONSTRUCT: u32 = 0x00aa_5fc0;
const LOCK_FOR_WRITE: u32 = 0x00aa_6060;
const LOCK_FOR_READ: u32 = 0x00aa_5ff0;
const UNLOCK_WRITE: u32 = 0x00aa_62e0;
const UNLOCK_READ: u32 = 0x00aa_62c0;
/// The `BSScrapArray<NavMeshPtr,1024>` (0x14 bytes, by its constructor's vtable) constructor (`006c6c60`) and destructor
/// (`006c6ce0`).
const NAV_MESH_ARRAY_CONSTRUCT: u32 = 0x006c_6c60;
const NAV_MESH_ARRAY_DESTRUCT: u32 = 0x006c_6ce0;
/// `006d8a20` (`cdecl`: location, radius, out array): the navmeshes within
/// a radius of a location; answers whether it found any.
const NAV_MESHES_IN_RADIUS: u32 = 0x006d_8a20;
/// `004b9ba0` (`thiscall`): an `NiTMap`'s first iteration position.
const MAP_FIRST_POSITION: u32 = 0x004b_9ba0;
/// `006b7f20` (`thiscall`, `RET 0xC`): the next item (position, key, value
/// out words).
const MAP_NEXT: u32 = 0x006b_7f20;
/// `00438af0` (`thiscall`): removes every item of an `NiTMap`.
const MAP_REMOVE_ALL: u32 = 0x0043_8af0;
/// `00853130` (`thiscall`, `RET 8`): an `NiTMap` lookup (key, value out).
const MAP_GET_AT: u32 = 0x0085_3130;
/// `NiTMapBase<...CombatThreat *>::SetAt` (engine map; folded),
/// `thiscall`, `RET 8`: (key, value).
const MAP_SET_AT: u32 = 0x0084_4700;
/// `0084e3a0` (`thiscall`): a navmesh's key in the map.
const NAV_MESH_KEY: u32 = 0x0084_e3a0;
/// `0068efb0` (`thiscall`): a navmesh's navmesh info.
const NAV_MESH_INFO_OF: u32 = 0x0068_efb0;
/// The `BSSimpleArray<NavMeshPtr,1024>` (by its constructor's vtable) constructor (`0042f800`), add (`0042f850`, `RET 4`)
/// and destructor (`0042f830`).
const NAV_MESH_POINTERS_CONSTRUCT: u32 = 0x0042_f800;
const NAV_MESH_POINTERS_ADD: u32 = 0x0042_f850;
const NAV_MESH_POINTERS_DESTRUCT: u32 = 0x0042_f830;
/// `006e1500` (`pathinglosmap.cpp`, `thiscall`, `RET 4`): the LOS map's
/// set-up from a navmesh list.
const LOS_MAP_SET_UP: u32 = 0x006e_1500;
/// `006d8990` (`thiscall`): a LOS map's navmesh count; `006d89b0`
/// (`thiscall`, `RET 4`): its navmesh `i`.
const LOS_MAP_NAV_MESH_COUNT: u32 = 0x006d_8990;
const LOS_MAP_NAV_MESH: u32 = 0x006d_89b0;
/// `0044edb0` (named `BaseProcess::GetCurrentProcedureIndex` by the engine
/// map; folded): a location's navmesh info (the word at +0x14).
const LOCATION_NAV_MESH_INFO: u32 = 0x0044_edb0;
/// `00aa1070` (`cdecl`): allocates a bucket table; `00403d30` (`cdecl`:
/// block, byte, size) fills it; `00aa10f0` frees it.
const TABLE_ALLOCATE: u32 = 0x00aa_1070;
const MEMSET: u32 = 0x0040_3d30;
const TABLE_FREE: u32 = 0x00aa_10f0;
/// `006b3eb0` / `0069b940` (`thiscall`, `RET 8`): the two arrays' base
/// set-up (size, reserved size).
const CELL_ARRAY_INIT: u32 = 0x006b_3eb0;
const CHAR_ARRAY_INIT: u32 = 0x0069_b940;
/// `006e12c0` (`thiscall`, `RET 8`): the char array's buffer reallocation
/// (new reserved size, element count to keep).
const CHAR_ARRAY_REALLOCATE: u32 = 0x006e_12c0;
/// `006e1280` (`thiscall`, `RET 8`): constructs `count` elements at an address.
const CHAR_ARRAY_CONSTRUCT_ELEMENTS: u32 = 0x006e_1280;
/// `0072ba80` (`thiscall`, `RET 8`): destroys `count` elements at an address.
const CHAR_ARRAY_DESTROY_ELEMENTS: u32 = 0x0072_ba80;

// Translated from 006df9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingLOSGridCell::SaveGame` (Xbox PDB): saves `bAllSeen` (1 byte) and
/// `iNavMeshGridCellIndex` (4 bytes); when not all seen, the seen-triangle
/// count and each byte of the list.
pub fn pathing_los_grid_cell_save_game(e: &mut Engine, this: Ptr<PathingLOSGridCell>, buffer: Ptr) {
    e.call(SAVE_DATA, &args![buffer, this, 1u32, 0u32]);
    e.call(SAVE_DATA, &args![buffer, this.byte_add(0x18), 4u32, 0u32]);
    if e.get(this, PathingLOSGridCell::bAllSeen) == 0 {
        let list = this.byte_add(4);
        // The exe tests the list's address (`this + 4`) for null.
        let count = if list.addr() != 0 {
            e.call(ARRAY_SIZE, &args![list]).u32()
        } else {
            0
        };
        e.call(SAVE_VARIABLE_SIZED_VALUE, &args![buffer, count]);
        for i in 0..count {
            let item = fn_006e0ed0(e, list, i);
            e.call(SAVE_DATA, &args![buffer, item, 1u32, 0u32]);
        }
    }
}

// Translated from 006dfa70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingLOSGridCell::LoadGame` (Xbox PDB): the reverse of
/// [`pathing_los_grid_cell_save_game`]; sizes the list with
/// [`fn_006e10c0`] (shrinking) before reading its bytes.
pub fn pathing_los_grid_cell_load_game(e: &mut Engine, this: Ptr<PathingLOSGridCell>, buffer: Ptr) {
    e.call(LOAD_DATA, &args![buffer, this, 1u32]);
    e.call(LOAD_DATA, &args![buffer, this.byte_add(0x18), 4u32]);
    if e.get(this, PathingLOSGridCell::bAllSeen) == 0 {
        let count = e.call(LOAD_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
        let list = this.byte_add(4);
        fn_006e10c0(e, list, count, 1);
        for i in 0..count {
            let item = fn_006e0ed0(e, list, i);
            e.call(LOAD_DATA, &args![buffer, item, 1u32]);
        }
    }
}

// Translated from 006dfb00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006dfb00 - the C++ exception frame (FS:[0]) that destroys the NiPointer on unwinding is not translated
/// Unnamed (`PathingLOSGrid`): stores the navmesh info at `+0x10`, takes
/// its navmesh into a local `NiPointer` (`0069ad00`); without one it clears
/// `+0x10`, with one it hands it to `006dfba0`.
pub fn fn_006dfb00(e: &mut Engine, this: Ptr<PathingLOSGrid>, nav_mesh_info: Ptr) {
    e.set(this, PathingLOSGrid::pNavMeshInfo, nav_mesh_info);
    e.with_stack(4, |e, nav_mesh| {
        e.call(NI_POINTER_CONSTRUCT, &args![nav_mesh]);
        let info = e.get(this, PathingLOSGrid::pNavMeshInfo);
        if !e
            .call(NAV_MESH_INFO_GET_NAV_MESH, &args![info, nav_mesh])
            .bool()
        {
            e.set(this, PathingLOSGrid::pNavMeshInfo, Ptr::NULL);
            e.call(NI_POINTER_DESTRUCT, &args![nav_mesh]);
        } else {
            e.call(GRID_SET_NAV_MESH, &args![this, nav_mesh]);
            e.call(NI_POINTER_DESTRUCT, &args![nav_mesh]);
        }
    });
}

/// Runs `pass(cell, argument)` on every cell of the grid; returns true.
fn each_cell(e: &mut Engine, this: Ptr<PathingLOSGrid>, argument: u32, pass: u32) -> bool {
    let mut i = 0u32;
    while i < e.call(ARRAY_SIZE, &args![this]).u32() {
        let slot = e.call(WORD_ELEMENT, &args![this, i]).u32();
        let cell = e.mem.u32(slot);
        e.call(pass, &args![cell, argument]);
        i = i.wrapping_add(1);
    }
    true
}

// Translated from 006dfd20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`PathingLOSGrid`): `006df230(cell, argument)` on every cell
/// (the size is reread each step); returns true.
pub fn fn_006dfd20(e: &mut Engine, this: Ptr<PathingLOSGrid>, argument: u32) -> bool {
    each_cell(e, this, argument, CELL_PASS_A)
}

// Translated from 006dfd70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`PathingLOSGrid`): `006df440(cell, argument)` on every cell;
/// returns true.
pub fn fn_006dfd70(e: &mut Engine, this: Ptr<PathingLOSGrid>, argument: u32) -> bool {
    each_cell(e, this, argument, CELL_PASS_B)
}

// Translated from 006dfdc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006dfdc0 - the C++ exception frame (FS:[0]) that destroys the NiPointer on unwinding is not translated
/// Unnamed (`PathingLOSGrid`): the cell holding a location. With the
/// location's navmesh (`PathingLocation::GetNavMesh`), the navmesh's cell
/// index for the location's point (`005c3420`, `00691240`); when it is
/// inside the grid, the cell's answer `006df5f0(cell, location)`, else false.
pub fn fn_006dfdc0(e: &mut Engine, this: Ptr<PathingLOSGrid>, location: Ptr) -> bool {
    e.with_stack(4, |e, nav_mesh| {
        e.call(NI_POINTER_CONSTRUCT, &args![nav_mesh]);
        if e.call(LOCATION_GET_NAV_MESH, &args![location, nav_mesh])
            .bool()
        {
            let index = e.with_stack(0xc, |e, point| {
                let point = e.call(LOCATION_POINT, &args![location, point]).u32();
                let mesh = e.call(WORD_AT, &args![nav_mesh]).u32();
                e.call(NAV_MESH_CELL_INDEX, &args![mesh, point]).u32()
            });
            if index < e.call(ARRAY_SIZE, &args![this]).u32() {
                let slot = e.call(WORD_ELEMENT, &args![this, index]).u32();
                let cell = e.mem.u32(slot);
                let result = e.call(CELL_TEST_LOCATION, &args![cell, location]).u8();
                e.call(NI_POINTER_DESTRUCT, &args![nav_mesh]);
                return result != 0;
            }
        }
        e.call(NI_POINTER_DESTRUCT, &args![nav_mesh]);
        false
    })
}

// Translated from 006dfe90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006dfe90 - the C++ exception frame (FS:[0]) that destroys the locals on unwinding is not translated
/// Unnamed (`PathingLOSGrid`): with the location's navmesh, collects the
/// navmesh's cell indices for the location's triangle (`006df760`,
/// `00691350`) into a local array and runs `006df780(cell, location, flag)`
/// on each cell inside the grid; answers whether any of them answered true
/// (false without a navmesh).
pub fn fn_006dfe90(e: &mut Engine, this: Ptr<PathingLOSGrid>, location: Ptr, flag: u8) -> bool {
    e.with_stack(4, |e, nav_mesh| {
        e.call(NI_POINTER_CONSTRUCT, &args![nav_mesh]);
        if !e
            .call(LOCATION_GET_NAV_MESH, &args![location, nav_mesh])
            .bool()
        {
            e.call(NI_POINTER_DESTRUCT, &args![nav_mesh]);
            return false;
        }
        let any = e.with_stack(0x10, |e, cells| {
            e.call(INDEX_ARRAY_CONSTRUCT, &args![cells]);
            let triangle = e.call(LOCATION_TRIANGLE, &args![location]).u16();
            let mesh = e.call(WORD_AT, &args![nav_mesh]).u32();
            e.call(
                NAV_MESH_TRIANGLE_CELLS,
                &args![mesh, u32::from(triangle), cells],
            );
            let mut any = false;
            let mut i = 0u32;
            while i < e.call(ARRAY_SIZE, &args![cells]).u32() {
                let slot = e.call(ARRAY_ELEMENT, &args![cells, i]).u32();
                let index = e.mem.u32(slot);
                if index < e.call(ARRAY_SIZE, &args![this]).u32() {
                    let slot = e.call(WORD_ELEMENT, &args![this, index]).u32();
                    let cell = e.mem.u32(slot);
                    if e.call(
                        CELL_UPDATE_LOCATION,
                        &args![cell, location, u32::from(flag)],
                    )
                    .u8()
                        != 0
                    {
                        any = true;
                    }
                }
                i = i.wrapping_add(1);
            }
            e.call(INDEX_ARRAY_DESTRUCT, &args![cells]);
            any
        });
        e.call(NI_POINTER_DESTRUCT, &args![nav_mesh]);
        any
    })
}

// Translated from 006dffc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingLOSGrid::SaveGame` (Xbox PDB): the navmesh info's form id (the
/// first word of the info, 0 without one), the cell count and each cell
/// ([`pathing_los_grid_cell_save_game`]).
pub fn pathing_los_grid_save_game(e: &mut Engine, this: Ptr<PathingLOSGrid>, buffer: Ptr) {
    let info = e.get(this, PathingLOSGrid::pNavMeshInfo);
    let form_id = if !info.is_null() {
        e.call(WORD_AT, &args![info]).u32()
    } else {
        0
    };
    e.call(SAVE_FORM_ID, &args![buffer, form_id, 0u32]);
    let count = if !this.is_null() {
        e.call(ARRAY_SIZE, &args![this]).u32()
    } else {
        0
    };
    e.call(SAVE_VARIABLE_SIZED_VALUE, &args![buffer, count]);
    for i in 0..count {
        let slot = e.call(ARRAY_ELEMENT, &args![this, i]).u32();
        let cell = Ptr::new(e.mem.u32(slot));
        pathing_los_grid_cell_save_game(e, cell, buffer);
    }
}

// Translated from 006e0070 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006e0070 - the C++ exception frame (FS:[0]) that frees a cell whose constructor throws is not translated
/// `PathingLOSGrid::LoadGame` (Xbox PDB): the navmesh info by its saved
/// form id (`TES::GetNavMeshInfoMap`, `00693d70`) into `+0x10`, then the
/// cell count (`SetSize`, shrinking) and each cell: allocated (0x1C bytes),
/// constructed ([`fn_006e0190`], or null when the allocation fails), loaded
/// ([`pathing_los_grid_cell_load_game`]) and given the navmesh info
/// (`00600ab0`).
pub fn pathing_los_grid_load_game(e: &mut Engine, this: Ptr<PathingLOSGrid>, buffer: Ptr) {
    let form_id = e.with_stack(4, |e, form_id| {
        e.call(LOAD_FORM_ID, &args![buffer, form_id]);
        e.mem.u32(form_id.addr())
    });
    let tes: u32 = e.global(TES_SINGLETON);
    let map = e.call(TES_GET_NAV_MESH_INFO_MAP, &args![tes]).u32();
    let info = e.call(NAV_MESH_INFO_MAP_FIND, &args![map, form_id]).ptr();
    e.set(this, PathingLOSGrid::pNavMeshInfo, info);
    let count = e.call(LOAD_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
    e.call(WORD_ARRAY_SET_SIZE, &args![this, count, 1u32]);
    for i in 0..count {
        let slot = e.call(ARRAY_ELEMENT, &args![this, i]).u32();
        let block = e.call(ALLOCATE, &args![0x1cu32]).ptr();
        let cell = if !block.is_null() {
            fn_006e0190(e, block)
        } else {
            Ptr::NULL
        };
        e.mem.set_u32(slot, cell.addr());
        let cell = Ptr::new(e.mem.u32(slot));
        pathing_los_grid_cell_load_game(e, cell, buffer);
        let info = e.get(this, PathingLOSGrid::pNavMeshInfo);
        let cell = e.mem.u32(slot);
        e.call(CELL_SET_NAV_MESH_INFO, &args![cell, info]);
    }
}

// Translated from 006e0190 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `PathingLOSGridCell` constructor (by the fields it sets): the seen
/// list ([`fn_006e1070`]), `bAllSeen` false, no navmesh info, cell index -1.
pub fn fn_006e0190(e: &mut Engine, this: Ptr<PathingLOSGridCell>) -> Ptr<PathingLOSGridCell> {
    fn_006e1070(e, this.byte_add(4));
    e.set(this, PathingLOSGridCell::bAllSeen, 0);
    e.set(this, PathingLOSGridCell::pNavMeshInfo, Ptr::NULL);
    e.set(this, PathingLOSGridCell::iNavMeshGridCellIndex, 0xffff_ffff);
    this
}

// Translated from 006e01d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006e01d0 - the C++ exception frame (FS:[0]) that destroys the constructed members on unwinding is not translated
/// `PathingLOSGridMap::PathingLOSGridMap` (Xbox PDB): the `NiRefObject`
/// base, the vtable, the grid table (0x25 buckets, [`fn_006e0ea0`]),
/// `fPercentSeen` 0, the center location, `fRadius` and `fSightRadius` 0,
/// and the lock.
pub fn pathing_los_grid_map_pathing_los_grid_map(
    e: &mut Engine,
    this: Ptr<PathingLOSGridMap>,
) -> Ptr<PathingLOSGridMap> {
    e.call(REF_OBJECT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), GRID_MAP_VTABLE);
    fn_006e0ea0(e, this.byte_add(8), 0x25);
    e.set(this, PathingLOSGridMap::fPercentSeen, 0.0);
    e.call(LOCATION_CONSTRUCT, &args![this.byte_add(GRID_MAP_CENTER)]);
    e.set(this, PathingLOSGridMap::fRadius, 0.0);
    e.set(this, PathingLOSGridMap::fSightRadius, 0.0);
    e.call(LOCK_CONSTRUCT, &args![this.byte_add(GRID_MAP_LOCK)]);
    this
}

// Translated from 006e0270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingLOSGridMap::_scalar_deleting_destructor_` (Xbox PDB): the
/// destructor ([`fn_006e02a0`]), then frees 0x54 bytes when bit 0 of
/// `flags` is set. Returns `this`.
pub fn pathing_los_grid_map_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<PathingLOSGridMap>,
    flags: u32,
) -> Ptr<PathingLOSGridMap> {
    fn_006e02a0(e, this);
    if flags & 1 != 0 {
        e.call(FREE_SIZED, &args![this, 0x54u32]);
    }
    this
}

// Translated from 006e02a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006e02a0 - the C++ exception frame (FS:[0]) that finishes the member destructors on unwinding is not translated
/// The `PathingLOSGridMap` destructor (by the vtable it sets): empties the
/// grids ([`fn_006e03e0`]), then destroys the center location, the grid
/// table ([`fn_006e0f90`]) and the `NiRefObject` base.
pub fn fn_006e02a0(e: &mut Engine, this: Ptr<PathingLOSGridMap>) {
    e.mem.set_u32(this.addr(), GRID_MAP_VTABLE);
    fn_006e03e0(e, this);
    e.call(LOCATION_DESTRUCT, &args![this.byte_add(GRID_MAP_CENTER)]);
    fn_006e0f90(e, this.byte_add(8));
    e.call(REF_OBJECT_DESTRUCT, &args![this]);
}

// Translated from 006e0320 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006e0320 - the C++ exception frame (FS:[0]) that destroys the navmesh array on unwinding is not translated
/// Unnamed (`PathingLOSGridMap`): under the write lock, copies the location
/// into the center and the radius into `fRadius`, gathers the navmeshes
/// within the radius (`006d8a20`) into a local array and, when it found
/// any, adds their grids ([`fn_006e05f0`]); otherwise `fRadius` becomes 0.
/// The radius goes through the x87 stack (a signalling NaN is stored and
/// passed quiet).
pub fn fn_006e0320(e: &mut Engine, this: Ptr<PathingLOSGridMap>, location: Ptr, radius: f32) {
    e.call(LOCK_FOR_WRITE, &args![this.byte_add(GRID_MAP_LOCK)]);
    let center = this.byte_add(GRID_MAP_CENTER);
    e.call(LOCATION_ASSIGN, &args![center, location]);
    let radius = x87_quiet_bits(radius.to_bits());
    e.mem.set_u32(this.addr().wrapping_add(0x44), radius);
    e.with_stack(0x14, |e, nav_meshes| {
        e.call(NAV_MESH_ARRAY_CONSTRUCT, &args![nav_meshes]);
        if e.call(NAV_MESHES_IN_RADIUS, &args![center, radius, nav_meshes])
            .bool()
        {
            fn_006e05f0(e, this, nav_meshes);
        } else {
            e.set(this, PathingLOSGridMap::fRadius, 0.0);
        }
        e.call(UNLOCK_WRITE, &args![this.byte_add(GRID_MAP_LOCK)]);
        e.call(NAV_MESH_ARRAY_DESTRUCT, &args![nav_meshes]);
    });
}

/// Iterates an `NiTMap` the way the exe does: the first position, then
/// `006b7f20` for each item (key, value), until the position is null.
fn for_each_item(e: &mut Engine, map: Ptr, mut f: impl FnMut(&mut Engine, u32, u32)) {
    let position = e.call(MAP_FIRST_POSITION, &args![map]).u32();
    e.with_stack(0xc, |e, words| {
        let (position_slot, key_slot, value_slot) =
            (words.addr(), words.addr() + 4, words.addr() + 8);
        e.mem.set_u32(position_slot, position);
        while e.mem.u32(position_slot) != 0 {
            e.call(MAP_NEXT, &args![map, position_slot, key_slot, value_slot]);
            let (key, value) = (e.mem.u32(key_slot), e.mem.u32(value_slot));
            f(e, key, value);
        }
    });
}

// Translated from 006e03e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`PathingLOSGridMap`): under the write lock, deletes every grid
/// of the table ([`fn_006e0480`] with 1) and removes all its items.
pub fn fn_006e03e0(e: &mut Engine, this: Ptr<PathingLOSGridMap>) {
    e.call(LOCK_FOR_WRITE, &args![this.byte_add(GRID_MAP_LOCK)]);
    let table = this.byte_add(8);
    let position = e.call(MAP_FIRST_POSITION, &args![table]).u32();
    e.with_stack(0xc, |e, words| {
        let (position_slot, key_slot, value_slot) =
            (words.addr(), words.addr() + 4, words.addr() + 8);
        e.mem.set_u32(position_slot, position);
        while e.mem.u32(position_slot) != 0 {
            // Key and value are cleared before each step.
            e.mem.set_u32(key_slot, 0);
            e.mem.set_u32(value_slot, 0);
            e.call(MAP_NEXT, &args![table, position_slot, key_slot, value_slot]);
            let grid = Ptr::new(e.mem.u32(value_slot));
            if !grid.is_null() {
                fn_006e0480(e, grid, 1);
            }
        }
    });
    e.call(MAP_REMOVE_ALL, &args![table]);
    e.call(UNLOCK_WRITE, &args![this.byte_add(GRID_MAP_LOCK)]);
}

// Translated from 006e0480 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `PathingLOSGrid` scalar deleting destructor (by its body): the
/// destructor ([`fn_006e04b0`]), then `operator delete` when bit 0 of
/// `flags` is set. Returns `this`.
pub fn fn_006e0480(e: &mut Engine, this: Ptr<PathingLOSGrid>, flags: u32) -> Ptr<PathingLOSGrid> {
    fn_006e04b0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 006e04b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006e04b0 - the C++ exception frame (FS:[0]) that destroys the cell array on unwinding is not translated
/// The `PathingLOSGrid` destructor (by its body): deletes every cell
/// ([`fn_006e0560`] with 1), then destroys the cell array ([`fn_006e1050`]).
pub fn fn_006e04b0(e: &mut Engine, this: Ptr<PathingLOSGrid>) {
    let mut i = 0u32;
    while i < e.call(ARRAY_SIZE, &args![this]).u32() {
        let slot = e.call(WORD_ELEMENT, &args![this, i]).u32();
        let cell = Ptr::new(e.mem.u32(slot));
        if !cell.is_null() {
            fn_006e0560(e, cell, 1);
        }
        i = i.wrapping_add(1);
    }
    fn_006e1050(e, this.cast());
}

// Translated from 006e0560 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `PathingLOSGridCell` scalar deleting destructor (by its body): the
/// destructor ([`fn_006e0590`]), then `operator delete` when bit 0 of
/// `flags` is set. Returns `this`.
pub fn fn_006e0560(
    e: &mut Engine,
    this: Ptr<PathingLOSGridCell>,
    flags: u32,
) -> Ptr<PathingLOSGridCell> {
    fn_006e0590(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 006e0590 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `PathingLOSGridCell` destructor (by its body): destroys the seen
/// list ([`fn_006e10a0`]).
pub fn fn_006e0590(e: &mut Engine, this: Ptr<PathingLOSGridCell>) {
    fn_006e10a0(e, this.byte_add(4));
}

// Translated from 006e05b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`PathingLOSGridMap`): resets `fPercentSeen` to 0, empties the
/// grids ([`fn_006e03e0`]) and sets the map up again around its own center
/// and radius ([`fn_006e0320`]; the radius goes through the x87 stack).
pub fn fn_006e05b0(e: &mut Engine, this: Ptr<PathingLOSGridMap>) {
    e.set(this, PathingLOSGridMap::fPercentSeen, 0.0);
    fn_006e03e0(e, this);
    let radius = x87_quiet(e.get(this, PathingLOSGridMap::fRadius));
    fn_006e0320(e, this, this.byte_add(GRID_MAP_CENTER), radius);
}

// Translated from 006e05f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006e05f0 - the C++ exception frame (FS:[0]) that destroys the NiPointer and frees a grid whose constructor throws is not translated
/// Unnamed (`PathingLOSGridMap`): under the write lock, for each navmesh of
/// the array: looks its grid up by the navmesh's key (`0084e3a0`); a new
/// navmesh gets a new grid (0x14 bytes, [`fn_006e0730`], null when the
/// allocation fails) set in the table, a known one is handed the navmesh
/// again (`006dfba0`). Returns true.
pub fn fn_006e05f0(e: &mut Engine, this: Ptr<PathingLOSGridMap>, nav_meshes: Ptr) -> bool {
    e.call(LOCK_FOR_WRITE, &args![this.byte_add(GRID_MAP_LOCK)]);
    let table = this.byte_add(8);
    let mut i = 0u32;
    while i < e.call(ARRAY_SIZE, &args![nav_meshes]).u32() {
        let element = e.call(WORD_ELEMENT, &args![nav_meshes, i]).u32();
        e.with_stack(8, |e, locals| {
            let (nav_mesh, grid_slot) = (locals, locals.byte_add(4));
            e.call(NI_POINTER_CONSTRUCT_FROM, &args![nav_mesh, element]);
            e.mem.set_u32(grid_slot.addr(), 0);
            let mesh = e.call(WORD_AT, &args![nav_mesh]).u32();
            let key = e.call(NAV_MESH_KEY, &args![mesh]).u32();
            if !e.call(MAP_GET_AT, &args![table, key, grid_slot]).bool() {
                let block = e.call(ALLOCATE, &args![0x14u32]).ptr();
                let grid = if !block.is_null() {
                    fn_006e0730(e, block, nav_mesh)
                } else {
                    Ptr::NULL
                };
                e.mem.set_u32(grid_slot.addr(), grid.addr());
                let grid = e.mem.u32(grid_slot.addr());
                let mesh = e.call(WORD_AT, &args![nav_mesh]).u32();
                let key = e.call(NAV_MESH_KEY, &args![mesh]).u32();
                e.call(MAP_SET_AT, &args![table, key, grid]);
            } else {
                let grid = e.mem.u32(grid_slot.addr());
                e.call(GRID_SET_NAV_MESH, &args![grid, nav_mesh]);
            }
            e.call(NI_POINTER_DESTRUCT, &args![nav_mesh]);
        });
        i = i.wrapping_add(1);
    }
    e.call(UNLOCK_WRITE, &args![this.byte_add(GRID_MAP_LOCK)]);
    true
}

// Translated from 006e0730 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006e0730 - the C++ exception frame (FS:[0]) that destroys the cell array on unwinding is not translated
/// A `PathingLOSGrid` constructor from a navmesh (by its body): the cell
/// array ([`fn_006e1020`]), no navmesh info, then `006dfba0(this,
/// nav_mesh)`. Returns `this`.
pub fn fn_006e0730(
    e: &mut Engine,
    this: Ptr<PathingLOSGrid>,
    nav_mesh: Ptr,
) -> Ptr<PathingLOSGrid> {
    fn_006e1020(e, this.cast());
    e.set(this, PathingLOSGrid::pNavMeshInfo, Ptr::NULL);
    e.call(GRID_SET_NAV_MESH, &args![this, nav_mesh]);
    this
}

// Translated from 006e07a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006e07a0 - the C++ exception frame (FS:[0]) that destroys the local list and NiPointers on unwinding is not translated
/// Unnamed (`PathingLOSGridMap`): under the read lock, collects the
/// navmesh of every grid that has one ([`fn_006e08f0`]) into a local list
/// (`0042f850`), sets the LOS map the argument points to up from it
/// (`006e1500`), then runs [`fn_006dfd20`] with the argument on every grid
/// that has a navmesh. Returns true.
pub fn fn_006e07a0(e: &mut Engine, this: Ptr<PathingLOSGridMap>, los_map: Ptr) -> bool {
    e.call(LOCK_FOR_READ, &args![this.byte_add(GRID_MAP_LOCK)]);
    let table = this.byte_add(8);
    e.with_stack(0x10, |e, list| {
        e.call(NAV_MESH_POINTERS_CONSTRUCT, &args![list]);
        for_each_item(e, table, |e, _, grid| {
            e.with_stack(4, |e, nav_mesh| {
                e.call(NI_POINTER_CONSTRUCT, &args![nav_mesh]);
                if fn_006e08f0(e, Ptr::new(grid), nav_mesh) {
                    e.call(NAV_MESH_POINTERS_ADD, &args![list, nav_mesh]);
                }
                e.call(NI_POINTER_DESTRUCT, &args![nav_mesh]);
            });
        });
        let map = e.call(WORD_AT, &args![los_map]).u32();
        e.call(LOS_MAP_SET_UP, &args![map, list]);
        for_each_item(e, table, |e, _, grid| {
            e.with_stack(4, |e, nav_mesh| {
                e.call(NI_POINTER_CONSTRUCT, &args![nav_mesh]);
                if fn_006e08f0(e, Ptr::new(grid), nav_mesh) {
                    fn_006dfd20(e, Ptr::new(grid), los_map.addr());
                }
                e.call(NI_POINTER_DESTRUCT, &args![nav_mesh]);
            });
        });
        e.call(UNLOCK_READ, &args![this.byte_add(GRID_MAP_LOCK)]);
        e.call(NAV_MESH_POINTERS_DESTRUCT, &args![list]);
    });
    true
}

// Translated from 006e08f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`PathingLOSGrid`): false without a navmesh info, else the
/// info's navmesh into the `NiPointer` out argument (`0069ad00`) and its
/// answer.
pub fn fn_006e08f0(e: &mut Engine, this: Ptr<PathingLOSGrid>, nav_mesh: Ptr) -> bool {
    let info = e.get(this, PathingLOSGrid::pNavMeshInfo);
    if info.is_null() {
        return false;
    }
    e.call(NAV_MESH_INFO_GET_NAV_MESH, &args![info, nav_mesh])
        .bool()
}

// Translated from 006e0920 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006e0920 - the C++ exception frame (FS:[0]) that destroys the NiPointer and frees a grid whose constructor throws is not translated
/// Unnamed (`PathingLOSGridMap`): under the write lock, for each navmesh of
/// the LOS map the argument points to (`006d8990`, `006d89b0`): looks its
/// grid up by key; a new one gets a grid built from the navmesh's info
/// (0x14 bytes, [`fn_006e0a90`], null when the allocation fails) set in
/// the table; then [`fn_006dfd70`] runs on the grid with the argument.
/// Finally `fPercentSeen` becomes the LOS map's seen fraction
/// ([`fn_006e0b00`]). Returns true.
pub fn fn_006e0920(e: &mut Engine, this: Ptr<PathingLOSGridMap>, los_map: Ptr) -> bool {
    e.call(LOCK_FOR_WRITE, &args![this.byte_add(GRID_MAP_LOCK)]);
    let table = this.byte_add(8);
    let mut i = 0u32;
    loop {
        let map = e.call(WORD_AT, &args![los_map]).u32();
        if i >= e.call(LOS_MAP_NAV_MESH_COUNT, &args![map]).u32() {
            break;
        }
        let map = e.call(WORD_AT, &args![los_map]).u32();
        let element = e.call(LOS_MAP_NAV_MESH, &args![map, i]).u32();
        e.with_stack(8, |e, locals| {
            let (nav_mesh, grid_slot) = (locals, locals.byte_add(4));
            e.call(NI_POINTER_CONSTRUCT_FROM, &args![nav_mesh, element]);
            e.mem.set_u32(grid_slot.addr(), 0);
            let mesh = e.call(WORD_AT, &args![nav_mesh]).u32();
            let key = e.call(NAV_MESH_KEY, &args![mesh]).u32();
            if !e.call(MAP_GET_AT, &args![table, key, grid_slot]).bool() {
                let block = e.call(ALLOCATE, &args![0x14u32]).ptr();
                let grid = if !block.is_null() {
                    let mesh = e.call(WORD_AT, &args![nav_mesh]).u32();
                    let info = e.call(NAV_MESH_INFO_OF, &args![mesh]).ptr();
                    fn_006e0a90(e, block, info)
                } else {
                    Ptr::NULL
                };
                e.mem.set_u32(grid_slot.addr(), grid.addr());
                let grid = e.mem.u32(grid_slot.addr());
                let mesh = e.call(WORD_AT, &args![nav_mesh]).u32();
                let key = e.call(NAV_MESH_KEY, &args![mesh]).u32();
                e.call(MAP_SET_AT, &args![table, key, grid]);
            }
            let grid = Ptr::new(e.mem.u32(grid_slot.addr()));
            fn_006dfd70(e, grid, los_map.addr());
            e.call(NI_POINTER_DESTRUCT, &args![nav_mesh]);
        });
        i = i.wrapping_add(1);
    }
    let map = e.call(WORD_AT, &args![los_map]).ptr();
    let seen = fn_006e0b00(e, map);
    e.set(this, PathingLOSGridMap::fPercentSeen, seen);
    e.call(UNLOCK_WRITE, &args![this.byte_add(GRID_MAP_LOCK)]);
    true
}

// Translated from 006e0a90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006e0a90 - the C++ exception frame (FS:[0]) that destroys the cell array on unwinding is not translated
/// A `PathingLOSGrid` constructor from a navmesh info (by its body): the
/// cell array ([`fn_006e1020`]), no navmesh info, then
/// [`fn_006dfb00`]`(this, nav_mesh_info)`. Returns `this`.
pub fn fn_006e0a90(
    e: &mut Engine,
    this: Ptr<PathingLOSGrid>,
    nav_mesh_info: Ptr,
) -> Ptr<PathingLOSGrid> {
    fn_006e1020(e, this.cast());
    e.set(this, PathingLOSGrid::pNavMeshInfo, Ptr::NULL);
    fn_006dfb00(e, this, nav_mesh_info);
    this
}

// Translated from 006e0b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (on a `PathingLOSMap`, by the fields it reads): the fraction of
/// triangles seen, `iTotalTrianglesSeen / (iTotalTrianglesSeen +
/// iTotalTrianglesUnseen)` (+0x30, +0x34; the sum wraps as the exe's
/// 32-bit add), computed on the x87 stack from the two unsigned words and
/// stored as a `float`; 1.0 when the sum is 0.
pub fn fn_006e0b00(e: &mut Engine, this: Ptr) -> f32 {
    let seen = e.mem.u32(this.addr().wrapping_add(0x30));
    let total = seen.wrapping_add(e.mem.u32(this.addr().wrapping_add(0x34)));
    if total == 0 {
        return 1.0;
    }
    // FILD qword (zero-extended) twice, FDIVP ST(1),ST(0) (DE F9), FSTP float.
    f32::from_bits(x87_store32(x87_div(f64::from(seen), f64::from(total))))
}

// Translated from 006e0b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`PathingLOSGridMap`): false when the location has no navmesh
/// info (`0044edb0`) or no triangle (`0xFFFF`); else the grid of the
/// info's key (its first word) and that grid's answer ([`fn_006dfdc0`]),
/// false when the table has none.
pub fn fn_006e0b50(e: &mut Engine, this: Ptr<PathingLOSGridMap>, location: Ptr) -> bool {
    let info = e.call(LOCATION_NAV_MESH_INFO, &args![location]).u32();
    if info == 0 || e.call(LOCATION_TRIANGLE, &args![location]).u16() == 0xffff {
        return false;
    }
    let grid = e.with_stack(4, |e, grid_slot| {
        let key = e.call(WORD_AT, &args![info]).u32();
        let table = this.byte_add(8);
        if e.call(MAP_GET_AT, &args![table, key, grid_slot]).bool() {
            Some(Ptr::new(e.mem.u32(grid_slot.addr())))
        } else {
            None
        }
    });
    match grid {
        Some(grid) => fn_006dfdc0(e, grid, location),
        None => false,
    }
}

// Translated from 006e0bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`PathingLOSGridMap`): [`fn_006e0b50`]'s lookup, then the grid's
/// [`fn_006dfe90`]`(grid, location, flag)`.
pub fn fn_006e0bc0(e: &mut Engine, this: Ptr<PathingLOSGridMap>, location: Ptr, flag: u8) -> bool {
    let info = e.call(LOCATION_NAV_MESH_INFO, &args![location]).u32();
    if info == 0 || e.call(LOCATION_TRIANGLE, &args![location]).u16() == 0xffff {
        return false;
    }
    let grid = e.with_stack(4, |e, grid_slot| {
        let key = e.call(WORD_AT, &args![info]).u32();
        let table = this.byte_add(8);
        if e.call(MAP_GET_AT, &args![table, key, grid_slot]).bool() {
            Some(Ptr::new(e.mem.u32(grid_slot.addr())))
        } else {
            None
        }
    });
    match grid {
        Some(grid) => fn_006dfe90(e, grid, location, flag),
        None => false,
    }
}

// Translated from 006e0c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingLOSGridMap::SaveGame` (Xbox PDB): a variable-sized count
/// started before the grids (`StartVariableSizedValue`), each grid that is
/// not null as its key (`SaveFormID`) and contents
/// ([`pathing_los_grid_save_game`]), then the count written at the mark;
/// then `fPercentSeen`, `fRadius`, `fSightRadius` (4 bytes each) and the
/// center location (its vtable slot 0).
pub fn pathing_los_grid_map_save_game(e: &mut Engine, this: Ptr<PathingLOSGridMap>, buffer: Ptr) {
    let mut count = 0u32;
    let mark = e.call(START_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
    let table = this.byte_add(8);
    let position = e.call(MAP_FIRST_POSITION, &args![table]).u32();
    e.with_stack(0xc, |e, words| {
        let (position_slot, key_slot, value_slot) =
            (words.addr(), words.addr() + 4, words.addr() + 8);
        e.mem.set_u32(position_slot, position);
        while e.mem.u32(position_slot) != 0 {
            e.mem.set_u32(value_slot, 0);
            e.call(MAP_NEXT, &args![table, position_slot, key_slot, value_slot]);
            let grid = Ptr::new(e.mem.u32(value_slot));
            if !grid.is_null() {
                let key = e.mem.u32(key_slot);
                e.call(SAVE_FORM_ID, &args![buffer, key, 0u32]);
                pathing_los_grid_save_game(e, grid, buffer);
                count = count.wrapping_add(1);
            }
        }
    });
    e.call(SAVE_VARIABLE_SIZED_VALUE_AT, &args![buffer, count, mark]);
    e.call(SAVE_DATA, &args![buffer, this.byte_add(0x18), 4u32, 0u32]);
    e.call(SAVE_DATA, &args![buffer, this.byte_add(0x44), 4u32, 0u32]);
    e.call(SAVE_DATA, &args![buffer, this.byte_add(0x48), 4u32, 0u32]);
    e.vcall(this.addr().wrapping_add(GRID_MAP_CENTER), 0, &args![buffer]);
}

// Translated from 006e0d20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006e0d20 - the C++ exception frame (FS:[0]) that frees a grid whose constructor throws is not translated
/// `PathingLOSGridMap::LoadGame` (Xbox PDB): the grid count, then for each
/// grid: allocated (0x14 bytes, [`fn_006e0e80`], null when the allocation
/// fails), its key (`LoadFormID_ov2`), its contents
/// ([`pathing_los_grid_load_game`]); a grid with key 0 is deleted
/// ([`fn_006e0480`] with 1, when not null), the others are set in the table.
/// Then `fPercentSeen`, `fRadius`, `fSightRadius` and the center location
/// (its vtable slot 4).
pub fn pathing_los_grid_map_load_game(e: &mut Engine, this: Ptr<PathingLOSGridMap>, buffer: Ptr) {
    let count = e.call(LOAD_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
    for _ in 0..count {
        let block = e.call(ALLOCATE, &args![0x14u32]).ptr();
        let grid = if !block.is_null() {
            fn_006e0e80(e, block)
        } else {
            Ptr::NULL
        };
        let key = e.with_stack(4, |e, key| {
            e.call(LOAD_FORM_ID, &args![buffer, key]);
            e.mem.u32(key.addr())
        });
        pathing_los_grid_load_game(e, grid, buffer);
        let skip = key == 0;
        if !skip {
            e.call(MAP_SET_AT, &args![this.byte_add(8), key, grid]);
        } else if !grid.is_null() {
            fn_006e0480(e, grid, 1);
        }
    }
    e.call(LOAD_DATA, &args![buffer, this.byte_add(0x18), 4u32]);
    e.call(LOAD_DATA, &args![buffer, this.byte_add(0x44), 4u32]);
    e.call(LOAD_DATA, &args![buffer, this.byte_add(0x48), 4u32]);
    e.vcall(this.addr().wrapping_add(GRID_MAP_CENTER), 4, &args![buffer]);
}

// Translated from 006e0e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `PathingLOSGrid` default constructor (by its body): the cell array
/// ([`fn_006e1020`]) and no navmesh info. Returns `this`.
pub fn fn_006e0e80(e: &mut Engine, this: Ptr<PathingLOSGrid>) -> Ptr<PathingLOSGrid> {
    fn_006e1020(e, this.cast());
    e.set(this, PathingLOSGrid::pNavMeshInfo, Ptr::NULL);
    this
}

// Translated from 006e0ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMap<unsigned int,PathingLOSGrid *>` constructor (by the vtable
/// it sets): the base ([`fn_006e0f20`]) with `buckets`, then its vtable.
pub fn fn_006e0ea0(e: &mut Engine, this: Ptr, buckets: u32) -> Ptr {
    fn_006e0f20(e, this, buckets);
    e.mem.set_u32(this.addr(), GRID_TABLE_VTABLE);
    this
}

// Translated from 006e0ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<unsigned int,PathingLOSGrid *>::_scalar_deleting_destructor_`
/// (engine map): the destructor ([`fn_006e0f90`]), then `operator delete`
/// when bit 0 of `flags` is set. Returns `this`.
pub fn ni_t_map_pathing_los_grid_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_006e0f90(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 006e0f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMapBase<...PathingLOSGrid *>` constructor (by the vtable it
/// sets): `buckets` buckets, no items, the bucket table allocated
/// (`00aa1070`, `4 * buckets` bytes, wrapping) and zeroed; the bucket
/// count is reread after the allocation.
pub fn fn_006e0f20(e: &mut Engine, this: Ptr, buckets: u32) -> Ptr {
    let map = this.cast::<crate::types::NiTMap>();
    e.mem.set_u32(this.addr(), GRID_TABLE_BASE_VTABLE);
    e.set(map, crate::types::NiTMap::m_uiHashSize, buckets);
    e.set(map, crate::types::NiTMap::m_uiCount, 0);
    let size = e
        .get(map, crate::types::NiTMap::m_uiHashSize)
        .wrapping_shl(2);
    let table = e.call(TABLE_ALLOCATE, &args![size]).u32();
    e.set(map, crate::types::NiTMap::m_ppkHashTable, table);
    let size = e
        .get(map, crate::types::NiTMap::m_uiHashSize)
        .wrapping_shl(2);
    let table = e.get(map, crate::types::NiTMap::m_ppkHashTable);
    e.call(MEMSET, &args![table, 0u32, size]);
    this
}

// Translated from 006e0f90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006e0f90 - the C++ exception frame (FS:[0]) that runs the base destructor on unwinding is not translated
/// The `NiTMap<unsigned int,PathingLOSGrid *>` destructor (by the vtable
/// it sets): removes every item (`00438af0`), then the base destructor
/// ([`fn_006e0ff0`]).
pub fn fn_006e0f90(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), GRID_TABLE_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    fn_006e0ff0(e, this);
}

// Translated from 006e0ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMapBase<...PathingLOSGrid *>` destructor (by the vtable it
/// sets): removes every item, then frees the bucket table (`00aa10f0`).
pub fn fn_006e0ff0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), GRID_TABLE_BASE_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    let table = e.mem.u32(this.addr().wrapping_add(8));
    e.call(TABLE_FREE, &args![table]);
}

// Translated from 006e1020 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSSimpleArray<PathingLOSGridCell *,1024>` constructor (by the
/// vtable it sets): its vtable, then `006b3eb0(this, 0, 0)`.
pub fn fn_006e1020(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), CELL_ARRAY_VTABLE);
    e.call(CELL_ARRAY_INIT, &args![this, 0u32, 0u32]);
    this
}

// Translated from 006e1050 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSSimpleArray<PathingLOSGridCell *,1024>` destructor (by the
/// vtable it sets): its vtable, then `008454f0(this, 1)`.
pub fn fn_006e1050(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), CELL_ARRAY_VTABLE);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

// Translated from 006e1070 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSSimpleArray<char,1024>` constructor (by the vtable it sets): its
/// vtable, then `0069b940(this, 0, 0)`.
pub fn fn_006e1070(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0106CF0C);
    e.call(CHAR_ARRAY_INIT, &args![this, 0u32, 0u32]);
    this
}

// Translated from 006e10c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<char,1024>::SetSize` (by its body; 1-byte elements):
/// size 0 releases the buffer (`008454f0` with `shrink`). Growing past the
/// reserved size allocates (vtable slot +4) when nothing is reserved, else
/// reallocates (`006e12c0` keeping the old size), then constructs the new
/// elements (`006e1280`). Shrinking destroys the elements past `size`
/// (`0072ba80`) and, with `shrink` and `size` at most a quarter of the
/// reserved size, reallocates to `size`. Growing within the reservation
/// constructs the new elements.
pub fn fn_006e10c0(e: &mut Engine, this: Ptr, size: u32, shrink: u8) {
    use crate::types::BSSimpleArray as A;
    let array = this.cast::<A>();
    if size == 0 {
        e.call(ARRAY_RELEASE_008454F0, &args![this, u32::from(shrink)]);
        return;
    }
    if size > e.get(array, A::iReservedSize) {
        if e.get(array, A::iReservedSize) == 0 {
            let buffer = e.vcall(this.addr(), 4, &args![size]).u32();
            e.set(array, A::pBuffer, buffer);
            e.set(array, A::iReservedSize, size);
        } else {
            let used = e.get(array, A::iSize);
            e.call(CHAR_ARRAY_REALLOCATE, &args![this, size, used]);
            e.set(array, A::iReservedSize, size);
        }
        let used = e.get(array, A::iSize);
        let at = e.get(array, A::pBuffer).wrapping_add(used);
        e.call(
            CHAR_ARRAY_CONSTRUCT_ELEMENTS,
            &args![this, at, size.wrapping_sub(used)],
        );
        e.set(array, A::iSize, size);
    } else if size < e.get(array, A::iSize) {
        let used = e.get(array, A::iSize);
        let at = e.get(array, A::pBuffer).wrapping_add(size);
        e.call(
            CHAR_ARRAY_DESTROY_ELEMENTS,
            &args![this, at, used.wrapping_sub(size)],
        );
        e.set(array, A::iSize, size);
        if shrink != 0 && size <= e.get(array, A::iReservedSize) >> 2 {
            e.call(CHAR_ARRAY_REALLOCATE, &args![this, size, size]);
            e.set(array, A::iReservedSize, size);
        }
    } else {
        let used = e.get(array, A::iSize);
        let at = e.get(array, A::pBuffer).wrapping_add(used);
        e.call(
            CHAR_ARRAY_CONSTRUCT_ELEMENTS,
            &args![this, at, size.wrapping_sub(used)],
        );
        e.set(array, A::iSize, size);
    }
}

// Translated from 006e11f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<DFALL<NiTMapItem<unsigned int,PathingLOSGrid *>>,...>::_scalar_deleting_destructor_`
/// (engine map): the base destructor ([`fn_006e0ff0`]), then `operator
/// delete` when bit 0 of `flags` is set. Returns `this`.
pub fn ni_t_map_base_pathing_los_grid_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_006e0ff0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 006e1220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<PathingLOSGridCell *,1024>::_scalar_deleting_destructor_`
/// (engine map): the destructor ([`fn_006e1050`]), then `operator delete`
/// when bit 0 of `flags` is set. Returns `this`.
pub fn bs_simple_array_pathing_los_grid_cell_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_006e1050(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 006e1250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<char,1024>::_scalar_deleting_destructor_` (engine map):
/// the destructor ([`fn_006e10a0`]), then `operator delete` when bit 0 of
/// `flags` is set. Returns `this`.
pub fn bs_simple_array_char_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_006e10a0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006e_0ed0, fn_006e0ed0(Ptr, u32) -> Ptr),
        entry!(0x006e_10a0, fn_006e10a0(Ptr)),
        entry!(
            0x006d_f9c0,
            pathing_los_grid_cell_save_game(Ptr<PathingLOSGridCell>, Ptr)
        ),
        entry!(
            0x006d_fa70,
            pathing_los_grid_cell_load_game(Ptr<PathingLOSGridCell>, Ptr)
        ),
        entry!(0x006d_fb00, fn_006dfb00(Ptr<PathingLOSGrid>, Ptr)),
        entry!(0x006d_fd20, fn_006dfd20(Ptr<PathingLOSGrid>, u32) -> bool),
        entry!(0x006d_fd70, fn_006dfd70(Ptr<PathingLOSGrid>, u32) -> bool),
        entry!(0x006d_fdc0, fn_006dfdc0(Ptr<PathingLOSGrid>, Ptr) -> bool),
        entry!(
            0x006d_fe90,
            fn_006dfe90(Ptr<PathingLOSGrid>, Ptr, u8) -> bool
        ),
        entry!(
            0x006d_ffc0,
            pathing_los_grid_save_game(Ptr<PathingLOSGrid>, Ptr)
        ),
        entry!(
            0x006e_0070,
            pathing_los_grid_load_game(Ptr<PathingLOSGrid>, Ptr)
        ),
        entry!(
            0x006e_0190,
            fn_006e0190(Ptr<PathingLOSGridCell>) -> Ptr<PathingLOSGridCell>
        ),
        entry!(
            0x006e_01d0,
            pathing_los_grid_map_pathing_los_grid_map(
                Ptr<PathingLOSGridMap>,
            ) -> Ptr<PathingLOSGridMap>
        ),
        entry!(
            0x006e_0270,
            pathing_los_grid_map_scalar_deleting_destructor(
                Ptr<PathingLOSGridMap>,
                u32,
            ) -> Ptr<PathingLOSGridMap>
        ),
        entry!(0x006e_02a0, fn_006e02a0(Ptr<PathingLOSGridMap>)),
        entry!(0x006e_0320, fn_006e0320(Ptr<PathingLOSGridMap>, Ptr, f32)),
        entry!(0x006e_03e0, fn_006e03e0(Ptr<PathingLOSGridMap>)),
        entry!(
            0x006e_0480,
            fn_006e0480(Ptr<PathingLOSGrid>, u32) -> Ptr<PathingLOSGrid>
        ),
        entry!(0x006e_04b0, fn_006e04b0(Ptr<PathingLOSGrid>)),
        entry!(
            0x006e_0560,
            fn_006e0560(Ptr<PathingLOSGridCell>, u32) -> Ptr<PathingLOSGridCell>
        ),
        entry!(0x006e_0590, fn_006e0590(Ptr<PathingLOSGridCell>)),
        entry!(0x006e_05b0, fn_006e05b0(Ptr<PathingLOSGridMap>)),
        entry!(
            0x006e_05f0,
            fn_006e05f0(Ptr<PathingLOSGridMap>, Ptr) -> bool
        ),
        entry!(
            0x006e_0730,
            fn_006e0730(Ptr<PathingLOSGrid>, Ptr) -> Ptr<PathingLOSGrid>
        ),
        entry!(
            0x006e_07a0,
            fn_006e07a0(Ptr<PathingLOSGridMap>, Ptr) -> bool
        ),
        entry!(0x006e_08f0, fn_006e08f0(Ptr<PathingLOSGrid>, Ptr) -> bool),
        entry!(
            0x006e_0920,
            fn_006e0920(Ptr<PathingLOSGridMap>, Ptr) -> bool
        ),
        entry!(
            0x006e_0a90,
            fn_006e0a90(Ptr<PathingLOSGrid>, Ptr) -> Ptr<PathingLOSGrid>
        ),
        entry!(0x006e_0b00, fn_006e0b00(Ptr) -> f32),
        entry!(
            0x006e_0b50,
            fn_006e0b50(Ptr<PathingLOSGridMap>, Ptr) -> bool
        ),
        entry!(
            0x006e_0bc0,
            fn_006e0bc0(Ptr<PathingLOSGridMap>, Ptr, u8) -> bool
        ),
        entry!(
            0x006e_0c40,
            pathing_los_grid_map_save_game(Ptr<PathingLOSGridMap>, Ptr)
        ),
        entry!(
            0x006e_0d20,
            pathing_los_grid_map_load_game(Ptr<PathingLOSGridMap>, Ptr)
        ),
        entry!(
            0x006e_0e80,
            fn_006e0e80(Ptr<PathingLOSGrid>) -> Ptr<PathingLOSGrid>
        ),
        entry!(0x006e_0ea0, fn_006e0ea0(Ptr, u32) -> Ptr),
        entry!(0x006e_0ef0, ni_t_map_pathing_los_grid_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x006e_0f20, fn_006e0f20(Ptr, u32) -> Ptr),
        entry!(0x006e_0f90, fn_006e0f90(Ptr)),
        entry!(0x006e_0ff0, fn_006e0ff0(Ptr)),
        entry!(0x006e_1020, fn_006e1020(Ptr) -> Ptr),
        entry!(0x006e_1050, fn_006e1050(Ptr)),
        entry!(0x006e_1070, fn_006e1070(Ptr) -> Ptr),
        entry!(0x006e_10c0, fn_006e10c0(Ptr, u32, u8)),
        entry!(0x006e_11f0, ni_t_map_base_pathing_los_grid_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x006e_1220, bs_simple_array_pathing_los_grid_cell_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x006e_1250, bs_simple_array_char_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_fn_006e0ed0_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(OFFSET_TARGET_006A62F0, |_, a| {
            (0x4000_0000 | a[1]).into_ret()
        });
        e.call_log = Some(vec![]);
        let result = e.call(0x006e_0ed0, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == OFFSET_TARGET_006A62F0)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
        assert_eq!(result.u32(), 0x4000_2000);
    }
}

#[cfg(test)]
mod array_destructor_tests {
    use super::*;

    fn check_array_destructor(address: u32, vtable: u32, release: u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.register(release, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(address, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), vtable);
        let hits: Vec<_> = log.iter().filter(|c| c.0 == release).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![this, 1]);
    }

    #[test]
    fn fn_006e10a0_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x006e_10a0, ARRAY_VTABLE_0106CF0C, ARRAY_RELEASE_008454F0);
    }
}

#[cfg(test)]
mod n032_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn stub(e: &mut Engine, addresses: &[u32]) {
        for &address in addresses {
            e.register(address, |_, _| Ret::default());
        }
    }

    /// The array doubles: size at +8, element `i` at `pBuffer + 4 * i`, the
    /// word at a pointer.
    fn array_doubles(e: &mut Engine) {
        e.register(ARRAY_SIZE, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(WORD_ELEMENT, |e, a| {
            (e.mem.u32(a[0] + 4) + 4 * a[1]).into_ret()
        });
        e.register(ARRAY_ELEMENT, |e, a| {
            (e.mem.u32(a[0] + 4) + 4 * a[1]).into_ret()
        });
        e.register(WORD_AT, |e, a| e.mem.u32(a[0]).into_ret());
    }

    /// An array of words at `at` (vtable word, buffer, size, reserved).
    fn word_array(e: &mut Engine, at: u32, words: &[u32]) {
        let buffer = e.mem.alloc(4 * words.len() as u32 + 4);
        for (i, w) in words.iter().enumerate() {
            e.mem.set_u32(buffer + 4 * i as u32, *w);
        }
        e.mem.set_u32(at + 4, buffer);
        e.mem.set_u32(at + 8, words.len() as u32);
        e.mem.set_u32(at + 12, words.len() as u32);
    }

    fn calls(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    fn take(e: &mut Engine) -> Log {
        e.call_log.take().unwrap()
    }

    #[test]
    fn cell_save_game_saves_the_list_only_when_not_all_seen() {
        let mut e = Engine::new();
        stub(&mut e, &[SAVE_DATA, SAVE_VARIABLE_SIZED_VALUE]);
        e.register(ARRAY_SIZE, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(OFFSET_TARGET_006A62F0, |e, a| {
            (e.mem.u32(a[0] + 4) + a[1]).into_ret()
        });
        let cell = e.mem.alloc(0x1c);
        e.mem.set_u32(cell + 8, 0x500);
        e.mem.set_u32(cell + 12, 2);
        e.call_log = Some(vec![]);
        e.call(0x006d_f9c0, &args![cell, 0x77u32]);
        let log = take(&mut e);
        assert_eq!(
            calls(&log, SAVE_DATA),
            vec![
                vec![0x77, cell, 1, 0],
                vec![0x77, cell + 0x18, 4, 0],
                vec![0x77, 0x500, 1, 0],
                vec![0x77, 0x501, 1, 0],
            ]
        );
        assert_eq!(calls(&log, SAVE_VARIABLE_SIZED_VALUE), vec![vec![0x77, 2]]);
        e.mem.set_u8(cell, 1);
        e.call_log = Some(vec![]);
        e.call(0x006d_f9c0, &args![cell, 0x77u32]);
        let log = take(&mut e);
        assert_eq!(calls(&log, SAVE_DATA).len(), 2);
        assert!(calls(&log, SAVE_VARIABLE_SIZED_VALUE).is_empty());
    }

    #[test]
    fn cell_load_game_sizes_the_list_then_reads_each_byte() {
        let mut e = Engine::new();
        stub(
            &mut e,
            &[
                LOAD_DATA,
                ARRAY_RELEASE_008454F0,
                CHAR_ARRAY_CONSTRUCT_ELEMENTS,
            ],
        );
        e.register(LOAD_VARIABLE_SIZED_VALUE, |_, _| 2u32.into_ret());
        e.register(OFFSET_TARGET_006A62F0, |e, a| {
            (e.mem.u32(a[0] + 4) + a[1]).into_ret()
        });
        let cell = e.mem.alloc(0x1c);
        // A reserved buffer of 4 bytes, none used.
        e.mem.set_u32(cell + 8, 0x600);
        e.mem.set_u32(cell + 16, 4);
        e.call_log = Some(vec![]);
        e.call(0x006d_fa70, &args![cell, 0x77u32]);
        let log = take(&mut e);
        assert_eq!(
            calls(&log, LOAD_DATA),
            vec![
                vec![0x77, cell, 1],
                vec![0x77, cell + 0x18, 4],
                vec![0x77, 0x600, 1],
                vec![0x77, 0x601, 1],
            ]
        );
        assert_eq!(
            calls(&log, CHAR_ARRAY_CONSTRUCT_ELEMENTS),
            vec![vec![cell + 4, 0x600, 2]]
        );
        assert_eq!(e.mem.u32(cell + 12), 2);
    }

    #[test]
    fn grid_init_keeps_the_info_only_with_a_nav_mesh() {
        for found in [false, true] {
            let mut e = Engine::new();
            stub(
                &mut e,
                &[NI_POINTER_CONSTRUCT, NI_POINTER_DESTRUCT, GRID_SET_NAV_MESH],
            );
            e.register_double(NAV_MESH_INFO_GET_NAV_MESH, move |_, _| found.into_ret());
            let grid = e.mem.alloc(0x14);
            e.call_log = Some(vec![]);
            e.call(0x006d_fb00, &args![grid, 0x900u32]);
            let log = take(&mut e);
            let get = calls(&log, NAV_MESH_INFO_GET_NAV_MESH);
            assert_eq!(get[0][0], 0x900);
            assert_eq!(e.mem.u32(grid + 0x10), if found { 0x900 } else { 0 });
            assert_eq!(calls(&log, GRID_SET_NAV_MESH).len(), usize::from(found));
            assert_eq!(calls(&log, NI_POINTER_DESTRUCT).len(), 1);
        }
    }

    #[test]
    fn cell_passes_run_on_every_cell() {
        for (address, pass) in [(0x006d_fd20, CELL_PASS_A), (0x006d_fd70, CELL_PASS_B)] {
            let mut e = Engine::new();
            array_doubles(&mut e);
            stub(&mut e, &[pass]);
            let grid = e.mem.alloc(0x14);
            word_array(&mut e, grid, &[0x10, 0x20]);
            e.call_log = Some(vec![]);
            assert!(e.call(address, &args![grid, 0x55u32]).bool());
            let log = take(&mut e);
            assert_eq!(calls(&log, pass), vec![vec![0x10, 0x55], vec![0x20, 0x55]]);
        }
    }

    #[test]
    fn grid_location_test_needs_a_nav_mesh_and_an_index_inside() {
        for (found, index, expected) in [(false, 0u32, false), (true, 2, false), (true, 1, true)] {
            let mut e = Engine::new();
            array_doubles(&mut e);
            stub(&mut e, &[NI_POINTER_CONSTRUCT, NI_POINTER_DESTRUCT]);
            e.register_double(LOCATION_GET_NAV_MESH, move |e, a| {
                e.mem.set_u32(a[1], 0x4444);
                found.into_ret()
            });
            e.register(LOCATION_POINT, |_, a| a[1].into_ret());
            e.register_double(NAV_MESH_CELL_INDEX, move |_, _| index.into_ret());
            e.register(CELL_TEST_LOCATION, |_, _| 1u32.into_ret());
            let grid = e.mem.alloc(0x14);
            word_array(&mut e, grid, &[0x10, 0x20]);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x006d_fdc0, &args![grid, 0x700u32]).bool(), expected);
            let log = take(&mut e);
            if found {
                assert_eq!(calls(&log, NAV_MESH_CELL_INDEX)[0][0], 0x4444);
            }
            if expected {
                assert_eq!(calls(&log, CELL_TEST_LOCATION), vec![vec![0x20, 0x700]]);
            }
            assert_eq!(calls(&log, NI_POINTER_DESTRUCT).len(), 1);
        }
    }

    #[test]
    fn grid_update_runs_on_the_triangle_cells_inside_the_grid() {
        let mut e = Engine::new();
        array_doubles(&mut e);
        stub(
            &mut e,
            &[
                NI_POINTER_CONSTRUCT,
                NI_POINTER_DESTRUCT,
                INDEX_ARRAY_DESTRUCT,
                INDEX_ARRAY_CONSTRUCT,
            ],
        );
        e.register(LOCATION_GET_NAV_MESH, |e, a| {
            e.mem.set_u32(a[1], 0x4444);
            true.into_ret()
        });
        e.register(LOCATION_TRIANGLE, |_, _| 0x1_0007u32.into_ret());
        // The navmesh's cells for the triangle: 1, 5 (outside), 0.
        e.register(NAV_MESH_TRIANGLE_CELLS, |e, a| {
            let buffer = e.mem.alloc(12);
            e.mem.set_u32(buffer, 1);
            e.mem.set_u32(buffer + 4, 5);
            e.mem.set_u32(buffer + 8, 0);
            e.mem.set_u32(a[2] + 4, buffer);
            e.mem.set_u32(a[2] + 8, 3);
            Ret::default()
        });
        e.register(CELL_UPDATE_LOCATION, |_, a| {
            u32::from(a[0] == 0x20).into_ret()
        });
        let grid = e.mem.alloc(0x14);
        word_array(&mut e, grid, &[0x10, 0x20]);
        e.call_log = Some(vec![]);
        assert!(e.call(0x006d_fe90, &args![grid, 0x700u32, 0x101u32]).bool());
        let log = take(&mut e);
        let cells = calls(&log, NAV_MESH_TRIANGLE_CELLS);
        assert_eq!((cells[0][0], cells[0][1]), (0x4444, 7));
        assert_eq!(
            calls(&log, CELL_UPDATE_LOCATION),
            vec![vec![0x20, 0x700, 1], vec![0x10, 0x700, 1]]
        );
        assert_eq!(calls(&log, INDEX_ARRAY_DESTRUCT).len(), 1);
        // Without a navmesh: false, nothing collected.
        e.register(LOCATION_GET_NAV_MESH, |_, _| false.into_ret());
        e.call_log = Some(vec![]);
        assert!(!e.call(0x006d_fe90, &args![grid, 0x700u32, 0u32]).bool());
        assert!(calls(&take(&mut e), INDEX_ARRAY_CONSTRUCT).is_empty());
    }

    #[test]
    fn grid_save_game_saves_the_info_id_and_each_cell() {
        let mut e = Engine::new();
        array_doubles(&mut e);
        stub(
            &mut e,
            &[SAVE_FORM_ID, SAVE_VARIABLE_SIZED_VALUE, SAVE_DATA],
        );
        let info = e.mem.alloc(4);
        e.mem.set_u32(info, 0x00ab_cdef);
        let cell = e.mem.alloc(0x1c);
        e.mem.set_u8(cell, 1);
        let grid = e.mem.alloc(0x14);
        word_array(&mut e, grid, &[cell]);
        e.mem.set_u32(grid + 0x10, info);
        e.call_log = Some(vec![]);
        e.call(0x006d_ffc0, &args![grid, 0x77u32]);
        let log = take(&mut e);
        assert_eq!(calls(&log, SAVE_FORM_ID), vec![vec![0x77, 0x00ab_cdef, 0]]);
        assert_eq!(calls(&log, SAVE_VARIABLE_SIZED_VALUE), vec![vec![0x77, 1]]);
        assert_eq!(calls(&log, SAVE_DATA)[0], vec![0x77, cell, 1, 0]);
        // Without an info the id is 0.
        e.mem.set_u32(grid + 0x10, 0);
        e.call_log = Some(vec![]);
        e.call(0x006d_ffc0, &args![grid, 0x77u32]);
        assert_eq!(calls(&take(&mut e), SAVE_FORM_ID), vec![vec![0x77, 0, 0]]);
    }

    #[test]
    fn grid_load_game_finds_the_info_and_builds_each_cell() {
        let mut e = Engine::new();
        array_doubles(&mut e);
        stub(
            &mut e,
            &[LOAD_DATA, CELL_SET_NAV_MESH_INFO, CHAR_ARRAY_INIT],
        );
        e.map(TES_SINGLETON, 4);
        e.set_global(TES_SINGLETON, 0x5150u32);
        e.register(LOAD_FORM_ID, |e, a| {
            e.mem.set_u32(a[1], 0x0012_3456);
            Ret::default()
        });
        e.register(TES_GET_NAV_MESH_INFO_MAP, |_, _| 0x6000u32.into_ret());
        e.register(NAV_MESH_INFO_MAP_FIND, |_, _| 0x6100u32.into_ret());
        // One cell, whose seen list is empty.
        let mut counts = vec![1u32, 0];
        e.register_double(LOAD_VARIABLE_SIZED_VALUE, move |_, _| {
            counts.remove(0).into_ret()
        });
        e.register(ARRAY_RELEASE_008454F0, |_, _| Ret::default());
        e.register(WORD_ARRAY_SET_SIZE, |e, a| {
            let buffer = e.mem.alloc(4 * a[1]);
            e.mem.set_u32(a[0] + 4, buffer);
            e.mem.set_u32(a[0] + 8, a[1]);
            Ret::default()
        });
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        let grid = e.mem.alloc(0x14);
        e.call_log = Some(vec![]);
        e.call(0x006e_0070, &args![grid, 0x77u32]);
        let log = take(&mut e);
        assert_eq!(calls(&log, TES_GET_NAV_MESH_INFO_MAP), vec![vec![0x5150]]);
        assert_eq!(
            calls(&log, NAV_MESH_INFO_MAP_FIND),
            vec![vec![0x6000, 0x0012_3456]]
        );
        assert_eq!(e.mem.u32(grid + 0x10), 0x6100);
        assert_eq!(calls(&log, WORD_ARRAY_SET_SIZE), vec![vec![grid, 1, 1]]);
        assert_eq!(calls(&log, ALLOCATE), vec![vec![0x1c]]);
        let cell = e.mem.u32(e.mem.u32(grid + 4));
        assert_eq!(e.mem.u32(cell + 4), ARRAY_VTABLE_0106CF0C);
        assert_eq!(e.mem.u32(cell + 0x18), 0xffff_ffff);
        assert_eq!(
            calls(&log, CELL_SET_NAV_MESH_INFO),
            vec![vec![cell, 0x6100]]
        );
    }

    #[test]
    fn cell_constructor_sets_the_defaults() {
        let mut e = Engine::new();
        stub(&mut e, &[CHAR_ARRAY_INIT]);
        let cell = e.mem.alloc(0x1c);
        e.mem.write(cell, &[0xaa; 0x1c]);
        assert_eq!(e.call(0x006e_0190, &args![cell]).u32(), cell);
        assert_eq!(e.mem.u8(cell), 0);
        assert_eq!(e.mem.u32(cell + 4), ARRAY_VTABLE_0106CF0C);
        assert_eq!(e.mem.u32(cell + 0x14), 0);
        assert_eq!(e.mem.u32(cell + 0x18), 0xffff_ffff);
    }

    #[test]
    fn grid_map_constructor_builds_every_member() {
        let mut e = Engine::new();
        stub(
            &mut e,
            &[
                REF_OBJECT_CONSTRUCT,
                LOCATION_CONSTRUCT,
                LOCK_CONSTRUCT,
                MEMSET,
            ],
        );
        e.register(TABLE_ALLOCATE, |_, _| 0x7000u32.into_ret());
        let map = e.mem.alloc(0x54);
        e.mem.write(map, &[0xaa; 0x54]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006e_01d0, &args![map]).u32(), map);
        let log = take(&mut e);
        assert_eq!(e.mem.u32(map), GRID_MAP_VTABLE);
        assert_eq!(e.mem.u32(map + 8), GRID_TABLE_VTABLE);
        assert_eq!(e.mem.u32(map + 0xc), 0x25);
        assert_eq!(calls(&log, TABLE_ALLOCATE), vec![vec![0x94]]);
        for off in [0x18, 0x44, 0x48] {
            assert_eq!(e.mem.f32(map + off), 0.0);
        }
        assert_eq!(calls(&log, LOCATION_CONSTRUCT), vec![vec![map + 0x1c]]);
        assert_eq!(calls(&log, LOCK_CONSTRUCT), vec![vec![map + 0x4c]]);
    }

    fn destructor_doubles(e: &mut Engine) {
        stub(
            e,
            &[
                LOCK_FOR_WRITE,
                UNLOCK_WRITE,
                MAP_REMOVE_ALL,
                LOCATION_DESTRUCT,
                TABLE_FREE,
                REF_OBJECT_DESTRUCT,
                FREE_SIZED,
                MAP_FIRST_POSITION,
            ],
        );
    }

    #[test]
    fn grid_map_deleting_destructor_frees_with_bit_zero() {
        for flags in [0u32, 1] {
            let mut e = Engine::new();
            destructor_doubles(&mut e);
            let map = e.mem.alloc(0x54);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x006e_0270, &args![map, flags]).u32(), map);
            let log = take(&mut e);
            assert_eq!(calls(&log, REF_OBJECT_DESTRUCT), vec![vec![map]]);
            assert_eq!(calls(&log, FREE_SIZED).len(), flags as usize);
        }
    }

    #[test]
    fn grid_map_destructor_runs_the_members_in_order() {
        let mut e = Engine::new();
        destructor_doubles(&mut e);
        let map = e.mem.alloc(0x54);
        e.mem.set_u32(map + 0x10, 0x7000);
        e.call_log = Some(vec![]);
        e.call(0x006e_02a0, &args![map]);
        let log = take(&mut e);
        let order: Vec<u32> = log[1..].iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            vec![
                LOCK_FOR_WRITE,
                MAP_FIRST_POSITION,
                MAP_REMOVE_ALL,
                UNLOCK_WRITE,
                LOCATION_DESTRUCT,
                MAP_REMOVE_ALL,
                MAP_REMOVE_ALL,
                TABLE_FREE,
                REF_OBJECT_DESTRUCT,
            ]
        );
        assert_eq!(calls(&log, TABLE_FREE), vec![vec![0x7000]]);
        assert_eq!(e.mem.u32(map), GRID_MAP_VTABLE);
        assert_eq!(e.mem.u32(map + 8), GRID_TABLE_BASE_VTABLE);
    }

    fn set_up_doubles(e: &mut Engine, found: bool) {
        stub(
            e,
            &[
                LOCK_FOR_WRITE,
                UNLOCK_WRITE,
                LOCATION_ASSIGN,
                NAV_MESH_ARRAY_CONSTRUCT,
                NAV_MESH_ARRAY_DESTRUCT,
                MAP_REMOVE_ALL,
                ARRAY_SIZE,
                MAP_FIRST_POSITION,
            ],
        );
        e.register_double(NAV_MESHES_IN_RADIUS, move |_, _| found.into_ret());
    }

    #[test]
    fn set_up_stores_the_center_and_radius_and_keeps_it_only_with_nav_meshes() {
        for found in [false, true] {
            let mut e = Engine::new();
            set_up_doubles(&mut e, found);
            let map = e.mem.alloc(0x54);
            let signalling = 0x7f80_0001u32;
            e.call_log = Some(vec![]);
            e.call(0x006e_0320, &args![map, 0x880u32, signalling]);
            let log = take(&mut e);
            assert_eq!(calls(&log, LOCATION_ASSIGN), vec![vec![map + 0x1c, 0x880]]);
            let near = calls(&log, NAV_MESHES_IN_RADIUS);
            assert_eq!((near[0][0], near[0][1]), (map + 0x1c, 0x7fc0_0001));
            let radius = e.mem.u32(map + 0x44);
            assert_eq!(radius, if found { 0x7fc0_0001 } else { 0 });
            // With navmeshes, the grids are added under a second write lock.
            assert_eq!(calls(&log, LOCK_FOR_WRITE).len(), 1 + usize::from(found));
            assert_eq!(calls(&log, NAV_MESH_ARRAY_DESTRUCT).len(), 1);
        }
    }

    #[test]
    fn clear_deletes_every_grid_then_empties_the_table() {
        let mut e = Engine::new();
        array_doubles(&mut e);
        stub(
            &mut e,
            &[
                LOCK_FOR_WRITE,
                UNLOCK_WRITE,
                MAP_REMOVE_ALL,
                OPERATOR_DELETE,
                ARRAY_RELEASE_008454F0,
            ],
        );
        e.register(MAP_FIRST_POSITION, |_, _| 1u32.into_ret());
        let grid = e.mem.alloc(0x14);
        // Two items: a grid, then a null value; the position ends at 0.
        e.register_double(MAP_NEXT, move |e, a| {
            let step = e.mem.u32(a[1]);
            assert_eq!((e.mem.u32(a[2]), e.mem.u32(a[3])), (0, 0));
            e.mem.set_u32(a[3], if step == 1 { grid } else { 0 });
            e.mem.set_u32(a[1], if step == 1 { 2 } else { 0 });
            Ret::default()
        });
        let map = e.mem.alloc(0x54);
        e.call_log = Some(vec![]);
        e.call(0x006e_03e0, &args![map]);
        let log = take(&mut e);
        assert_eq!(calls(&log, MAP_NEXT).len(), 2);
        assert_eq!(calls(&log, OPERATOR_DELETE), vec![vec![grid]]);
        assert_eq!(calls(&log, MAP_REMOVE_ALL), vec![vec![map + 8]]);
        assert_eq!(e.mem.u32(grid), CELL_ARRAY_VTABLE);
    }

    #[test]
    fn grid_deleting_destructor_deletes_its_cells() {
        for flags in [0u32, 1] {
            let mut e = Engine::new();
            array_doubles(&mut e);
            stub(&mut e, &[OPERATOR_DELETE, ARRAY_RELEASE_008454F0]);
            let cell = e.mem.alloc(0x1c);
            let grid = e.mem.alloc(0x14);
            word_array(&mut e, grid, &[cell, 0]);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x006e_0480, &args![grid, flags]).u32(), grid);
            let log = take(&mut e);
            let mut expected = vec![vec![cell]];
            if flags == 1 {
                expected.push(vec![grid]);
            }
            assert_eq!(calls(&log, OPERATOR_DELETE), expected);
            assert_eq!(
                calls(&log, ARRAY_RELEASE_008454F0),
                vec![vec![cell + 4, 1], vec![grid, 1]]
            );
        }
    }

    #[test]
    fn grid_destructor_skips_null_cells() {
        let mut e = Engine::new();
        array_doubles(&mut e);
        stub(&mut e, &[OPERATOR_DELETE, ARRAY_RELEASE_008454F0]);
        let grid = e.mem.alloc(0x14);
        word_array(&mut e, grid, &[0, 0]);
        e.call_log = Some(vec![]);
        e.call(0x006e_04b0, &args![grid]);
        let log = take(&mut e);
        assert!(calls(&log, OPERATOR_DELETE).is_empty());
        assert_eq!(e.mem.u32(grid), CELL_ARRAY_VTABLE);
    }

    #[test]
    fn cell_deleting_destructor_and_destructor() {
        for flags in [0u32, 1] {
            let mut e = Engine::new();
            stub(&mut e, &[OPERATOR_DELETE, ARRAY_RELEASE_008454F0]);
            let cell = e.mem.alloc(0x1c);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x006e_0560, &args![cell, flags]).u32(), cell);
            let log = take(&mut e);
            assert_eq!(calls(&log, OPERATOR_DELETE).len(), flags as usize);
            assert_eq!(calls(&log, ARRAY_RELEASE_008454F0), vec![vec![cell + 4, 1]]);
        }
        let mut e = Engine::new();
        stub(&mut e, &[ARRAY_RELEASE_008454F0]);
        let cell = e.mem.alloc(0x1c);
        e.call(0x006e_0590, &args![cell]);
        assert_eq!(e.mem.u32(cell + 4), ARRAY_VTABLE_0106CF0C);
    }

    #[test]
    fn reset_clears_then_sets_up_around_its_own_center() {
        let mut e = Engine::new();
        set_up_doubles(&mut e, true);
        let map = e.mem.alloc(0x54);
        e.mem.set_f32(map + 0x18, 0.5);
        e.mem.set_f32(map + 0x44, 300.0);
        e.call_log = Some(vec![]);
        e.call(0x006e_05b0, &args![map]);
        let log = take(&mut e);
        assert_eq!(e.mem.f32(map + 0x18), 0.0);
        assert_eq!(calls(&log, MAP_REMOVE_ALL), vec![vec![map + 8]]);
        assert_eq!(
            calls(&log, LOCATION_ASSIGN),
            vec![vec![map + 0x1c, map + 0x1c]]
        );
        assert_eq!(calls(&log, NAV_MESHES_IN_RADIUS)[0][1], 300.0f32.to_bits());
    }

    fn add_doubles(e: &mut Engine, known: u32) -> u32 {
        let known_grid = e.mem.alloc(0x14);
        array_doubles(e);
        stub(
            e,
            &[
                LOCK_FOR_WRITE,
                UNLOCK_WRITE,
                NI_POINTER_DESTRUCT,
                MAP_SET_AT,
                GRID_SET_NAV_MESH,
                CELL_ARRAY_INIT,
                CELL_PASS_B,
            ],
        );
        // A copy of the `NiPointer` at the argument.
        e.register(NI_POINTER_CONSTRUCT_FROM, |e, a| {
            let target = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], target);
            Ret::default()
        });
        e.register(NAV_MESH_KEY, |_, a| (a[0] + 1).into_ret());
        // Navmesh `known` already has a grid.
        e.register_double(MAP_GET_AT, move |e, a| {
            if a[1] == known + 1 {
                e.mem.set_u32(a[2], known_grid);
                true.into_ret()
            } else {
                false.into_ret()
            }
        });
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        known_grid
    }

    #[test]
    fn add_grids_makes_new_grids_and_refreshes_known_ones() {
        let mut e = Engine::new();
        let known_grid = add_doubles(&mut e, 0x200);
        let meshes = e.mem.alloc(0x10);
        word_array(&mut e, meshes, &[0x100, 0x200]);
        let map = e.mem.alloc(0x54);
        e.call_log = Some(vec![]);
        assert!(e.call(0x006e_05f0, &args![map, meshes]).bool());
        let log = take(&mut e);
        let set = calls(&log, MAP_SET_AT);
        assert_eq!(set.len(), 1);
        assert_eq!((set[0][0], set[0][1]), (map + 8, 0x101));
        let grid = set[0][2];
        assert_eq!(e.mem.u32(grid), CELL_ARRAY_VTABLE);
        let handed = calls(&log, GRID_SET_NAV_MESH);
        assert_eq!(handed.len(), 2);
        assert_eq!(handed[0][0], grid);
        assert_eq!(handed[1][0], known_grid);
        assert_eq!(calls(&log, NI_POINTER_DESTRUCT).len(), 2);
    }

    #[test]
    fn grid_from_nav_mesh_constructor() {
        let mut e = Engine::new();
        stub(&mut e, &[CELL_ARRAY_INIT, GRID_SET_NAV_MESH]);
        let grid = e.mem.alloc(0x14);
        e.mem.set_u32(grid + 0x10, 9);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006e_0730, &args![grid, 0x880u32]).u32(), grid);
        let log = take(&mut e);
        assert_eq!(e.mem.u32(grid + 0x10), 0);
        assert_eq!(calls(&log, GRID_SET_NAV_MESH), vec![vec![grid, 0x880]]);
        assert_eq!(calls(&log, CELL_ARRAY_INIT), vec![vec![grid, 0, 0]]);
    }

    #[test]
    fn collect_hands_the_nav_meshes_to_the_los_map_then_updates_the_grids() {
        let mut e = Engine::new();
        stub(
            &mut e,
            &[
                LOCK_FOR_READ,
                UNLOCK_READ,
                NAV_MESH_POINTERS_CONSTRUCT,
                NAV_MESH_POINTERS_DESTRUCT,
                NAV_MESH_POINTERS_ADD,
                NI_POINTER_CONSTRUCT,
                NI_POINTER_DESTRUCT,
                LOS_MAP_SET_UP,
                CELL_PASS_A,
            ],
        );
        e.register(WORD_AT, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(ARRAY_SIZE, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(MAP_FIRST_POSITION, |_, _| 1u32.into_ret());
        let with_info = e.mem.alloc(0x14);
        e.mem.set_u32(with_info + 0x10, 0x900);
        let without = e.mem.alloc(0x14);
        e.register_double(MAP_NEXT, move |e, a| {
            let step = e.mem.u32(a[1]);
            e.mem
                .set_u32(a[3], if step == 1 { with_info } else { without });
            e.mem.set_u32(a[1], if step == 1 { 2 } else { 0 });
            Ret::default()
        });
        e.register(NAV_MESH_INFO_GET_NAV_MESH, |_, _| true.into_ret());
        let los_map_pointer = e.mem.alloc(4);
        e.mem.set_u32(los_map_pointer, 0x5500);
        let map = e.mem.alloc(0x54);
        e.call_log = Some(vec![]);
        assert!(e.call(0x006e_07a0, &args![map, los_map_pointer]).bool());
        let log = take(&mut e);
        assert_eq!(calls(&log, NAV_MESH_POINTERS_ADD).len(), 1);
        let set_up = calls(&log, LOS_MAP_SET_UP);
        assert_eq!(set_up[0][0], 0x5500);
        assert_eq!(calls(&log, NAV_MESH_INFO_GET_NAV_MESH).len(), 2);
        assert_eq!(calls(&log, UNLOCK_READ), vec![vec![map + 0x4c]]);
        assert_eq!(calls(&log, NI_POINTER_DESTRUCT).len(), 4);
    }

    #[test]
    fn grid_nav_mesh_needs_an_info() {
        let mut e = Engine::new();
        e.register(NAV_MESH_INFO_GET_NAV_MESH, |_, a| {
            (a[0] == 0x900).into_ret()
        });
        let grid = e.mem.alloc(0x14);
        assert!(!e.call(0x006e_08f0, &args![grid, 0x40u32]).bool());
        e.mem.set_u32(grid + 0x10, 0x900);
        e.call_log = Some(vec![]);
        assert!(e.call(0x006e_08f0, &args![grid, 0x40u32]).bool());
        assert_eq!(
            calls(&take(&mut e), NAV_MESH_INFO_GET_NAV_MESH),
            vec![vec![0x900, 0x40]]
        );
    }

    #[test]
    fn update_from_los_map_builds_grids_and_sets_the_percent_seen() {
        let mut e = Engine::new();
        add_doubles(&mut e, 0x200);
        stub(&mut e, &[NI_POINTER_CONSTRUCT]);
        e.register(NAV_MESH_INFO_GET_NAV_MESH, |_, _| false.into_ret());
        e.register(NAV_MESH_INFO_OF, |_, a| (a[0] + 0x10).into_ret());
        let los_map = e.mem.alloc(0x50);
        e.mem.set_u32(los_map + 0x30, 1);
        e.mem.set_u32(los_map + 0x34, 3);
        e.register_double(LOS_MAP_NAV_MESH_COUNT, move |_, a| {
            assert_eq!(a[0], los_map);
            2u32.into_ret()
        });
        let meshes = e.mem.alloc(0x10);
        word_array(&mut e, meshes, &[0x100, 0x200]);
        e.register_double(LOS_MAP_NAV_MESH, move |e, a| {
            (e.mem.u32(meshes + 4) + 4 * a[1]).into_ret()
        });
        let pointer = e.mem.alloc(4);
        e.mem.set_u32(pointer, los_map);
        let map = e.mem.alloc(0x54);
        e.call_log = Some(vec![]);
        assert!(e.call(0x006e_0920, &args![map, pointer]).bool());
        let log = take(&mut e);
        let set = calls(&log, MAP_SET_AT);
        assert_eq!(set.len(), 1);
        let grid = set[0][2];
        // The new grid took the navmesh's info (then dropped it: no navmesh).
        assert_eq!(calls(&log, NAV_MESH_INFO_GET_NAV_MESH)[0][0], 0x110);
        assert_eq!(e.mem.u32(grid + 0x10), 0);
        assert_eq!(e.mem.f32(map + 0x18), 0.25);
        assert_eq!(calls(&log, UNLOCK_WRITE).len(), 1);
    }

    #[test]
    fn grid_from_info_constructor_runs_the_init() {
        let mut e = Engine::new();
        stub(
            &mut e,
            &[CELL_ARRAY_INIT, NI_POINTER_CONSTRUCT, NI_POINTER_DESTRUCT],
        );
        e.register(NAV_MESH_INFO_GET_NAV_MESH, |_, _| false.into_ret());
        let grid = e.mem.alloc(0x14);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006e_0a90, &args![grid, 0x900u32]).u32(), grid);
        let log = take(&mut e);
        assert_eq!(calls(&log, NAV_MESH_INFO_GET_NAV_MESH)[0][0], 0x900);
        assert_eq!(e.mem.u32(grid), CELL_ARRAY_VTABLE);
    }

    #[test]
    fn percent_seen_divides_and_is_one_without_triangles() {
        let mut e = Engine::new();
        let map = e.mem.alloc(0x50);
        assert_eq!(e.call(0x006e_0b00, &args![map]).f32(), 1.0);
        e.mem.set_u32(map + 0x30, 1);
        e.mem.set_u32(map + 0x34, 2);
        assert_eq!(
            e.call(0x006e_0b00, &args![map]).f32(),
            (1.0f64 / 3.0) as f32
        );
        // The sum wraps to 0.
        e.mem.set_u32(map + 0x30, 0xffff_ffff);
        e.mem.set_u32(map + 0x34, 1);
        assert_eq!(e.call(0x006e_0b00, &args![map]).f32(), 1.0);
        // Large counts stay unsigned.
        e.mem.set_u32(map + 0x30, 0x8000_0000);
        e.mem.set_u32(map + 0x34, 0x7fff_ffff);
        assert_eq!(
            e.call(0x006e_0b00, &args![map]).f32(),
            (2_147_483_648.0f64 / 4_294_967_295.0) as f32
        );
    }

    fn lookup_doubles(e: &mut Engine, info: u32, triangle: u32, found: bool) {
        e.register(WORD_AT, |e, a| e.mem.u32(a[0]).into_ret());
        e.register_double(LOCATION_NAV_MESH_INFO, move |_, _| info.into_ret());
        e.register_double(LOCATION_TRIANGLE, move |_, _| triangle.into_ret());
        e.register_double(MAP_GET_AT, move |e, a| {
            e.mem.set_u32(a[2], 0x3000);
            found.into_ret()
        });
        stub(e, &[NI_POINTER_CONSTRUCT, NI_POINTER_DESTRUCT]);
        // The grid's own test then finds no navmesh.
        e.register(LOCATION_GET_NAV_MESH, |_, _| false.into_ret());
    }

    #[test]
    fn location_test_looks_the_grid_up_by_the_info_id() {
        for (has_info, triangle, found) in [
            (false, 1u32, true),
            (true, 0xffff, true),
            (true, 1, false),
            (true, 1, true),
        ] {
            let mut e = Engine::new();
            let info = if has_info { e.mem.alloc(4) } else { 0 };
            if has_info {
                e.mem.set_u32(info, 0x0042_0042);
            }
            lookup_doubles(&mut e, info, triangle, found);
            let map = e.mem.alloc(0x54);
            e.call_log = Some(vec![]);
            assert!(!e.call(0x006e_0b50, &args![map, 0x700u32]).bool());
            let log = take(&mut e);
            let get = calls(&log, MAP_GET_AT);
            if has_info && triangle != 0xffff {
                assert_eq!((get[0][0], get[0][1]), (map + 8, 0x0042_0042));
            } else {
                assert!(get.is_empty());
            }
            let reached_grid = has_info && triangle != 0xffff && found;
            assert_eq!(!calls(&log, LOCATION_GET_NAV_MESH).is_empty(), reached_grid);
        }
    }

    #[test]
    fn location_update_looks_the_grid_up_and_passes_the_location() {
        let mut e = Engine::new();
        let info = e.mem.alloc(4);
        e.mem.set_u32(info, 0x0042_0042);
        lookup_doubles(&mut e, info, 1, true);
        let map = e.mem.alloc(0x54);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x006e_0bc0, &args![map, 0x700u32, 1u32]).bool());
        let log = take(&mut e);
        assert_eq!(calls(&log, LOCATION_GET_NAV_MESH)[0][0], 0x700);
        e.register(LOCATION_TRIANGLE, |_, _| 0xffffu32.into_ret());
        e.call_log = Some(vec![]);
        assert!(!e.call(0x006e_0bc0, &args![map, 0x700u32, 1u32]).bool());
        assert!(calls(&take(&mut e), MAP_GET_AT).is_empty());
    }

    #[test]
    fn grid_map_save_game_counts_the_saved_grids() {
        let mut e = Engine::new();
        array_doubles(&mut e);
        stub(
            &mut e,
            &[
                SAVE_DATA,
                SAVE_FORM_ID,
                SAVE_VARIABLE_SIZED_VALUE_AT,
                SAVE_VARIABLE_SIZED_VALUE,
            ],
        );
        e.register(START_VARIABLE_SIZED_VALUE, |_, _| 0x99u32.into_ret());
        e.register(MAP_FIRST_POSITION, |_, _| 1u32.into_ret());
        let grid = e.mem.alloc(0x14);
        e.register_double(MAP_NEXT, move |e, a| {
            let step = e.mem.u32(a[1]);
            assert_eq!(e.mem.u32(a[3]), 0);
            e.mem.set_u32(a[2], 0x40 + step);
            e.mem.set_u32(a[3], if step == 1 { grid } else { 0 });
            e.mem.set_u32(a[1], if step == 1 { 2 } else { 0 });
            Ret::default()
        });
        e.put_vtable(0x5000, &[0x5100]);
        e.register(0x5100, |_, _| Ret::default());
        let map = e.mem.alloc(0x54);
        e.mem.set_u32(map + 0x1c, 0x5000);
        e.call_log = Some(vec![]);
        e.call(0x006e_0c40, &args![map, 0x77u32]);
        let log = take(&mut e);
        // The grid's key, then the grid's own (null) info id.
        assert_eq!(
            calls(&log, SAVE_FORM_ID),
            vec![vec![0x77, 0x41, 0], vec![0x77, 0, 0]]
        );
        assert_eq!(
            calls(&log, SAVE_VARIABLE_SIZED_VALUE_AT),
            vec![vec![0x77, 1, 0x99]]
        );
        let data = calls(&log, SAVE_DATA);
        assert_eq!(
            data,
            vec![
                vec![0x77, map + 0x18, 4, 0],
                vec![0x77, map + 0x44, 4, 0],
                vec![0x77, map + 0x48, 4, 0],
            ]
        );
        assert_eq!(calls(&log, 0x5100), vec![vec![map + 0x1c, 0x77]]);
    }

    #[test]
    fn grid_map_load_game_keeps_grids_with_a_key() {
        let mut e = Engine::new();
        array_doubles(&mut e);
        stub(
            &mut e,
            &[
                LOAD_DATA,
                MAP_SET_AT,
                CELL_ARRAY_INIT,
                OPERATOR_DELETE,
                ARRAY_RELEASE_008454F0,
                TES_GET_NAV_MESH_INFO_MAP,
                NAV_MESH_INFO_MAP_FIND,
                WORD_ARRAY_SET_SIZE,
            ],
        );
        e.map(TES_SINGLETON, 4);
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        // The map's grid count (2), then each grid's cell count (0).
        let mut counts = vec![2u32, 0, 0];
        e.register_double(LOAD_VARIABLE_SIZED_VALUE, move |_, _| {
            counts.remove(0).into_ret()
        });
        // Each grid's key, then its own info id: 0x51 and 0x7, then 0 and 0x8.
        let mut ids = vec![0x51u32, 0x7, 0, 0x8];
        e.register_double(LOAD_FORM_ID, move |e, a| {
            e.mem.set_u32(a[1], ids.remove(0));
            Ret::default()
        });
        e.put_vtable(0x5000, &[0x5100, 0x5104]);
        e.register(0x5104, |_, _| Ret::default());
        let map = e.mem.alloc(0x54);
        e.mem.set_u32(map + 0x1c, 0x5000);
        e.call_log = Some(vec![]);
        e.call(0x006e_0d20, &args![map, 0x77u32]);
        let log = take(&mut e);
        let set = calls(&log, MAP_SET_AT);
        assert_eq!(set.len(), 1);
        assert_eq!((set[0][0], set[0][1]), (map + 8, 0x51));
        assert_eq!(calls(&log, ALLOCATE).len(), 2);
        assert_eq!(calls(&log, OPERATOR_DELETE).len(), 1);
        assert_eq!(
            calls(&log, LOAD_DATA),
            vec![
                vec![0x77, map + 0x18, 4],
                vec![0x77, map + 0x44, 4],
                vec![0x77, map + 0x48, 4],
            ]
        );
        assert_eq!(calls(&log, 0x5104), vec![vec![map + 0x1c, 0x77]]);
    }

    #[test]
    fn grid_default_constructor() {
        let mut e = Engine::new();
        stub(&mut e, &[CELL_ARRAY_INIT]);
        let grid = e.mem.alloc(0x14);
        e.mem.set_u32(grid + 0x10, 5);
        assert_eq!(e.call(0x006e_0e80, &args![grid]).u32(), grid);
        assert_eq!(e.mem.u32(grid), CELL_ARRAY_VTABLE);
        assert_eq!(e.mem.u32(grid + 0x10), 0);
    }

    #[test]
    fn grid_table_constructors_allocate_and_zero_the_buckets() {
        for (address, vtable) in [
            (0x006e_0ea0, GRID_TABLE_VTABLE),
            (0x006e_0f20, GRID_TABLE_BASE_VTABLE),
        ] {
            let mut e = Engine::new();
            let table = e.mem.alloc(8);
            let map = e.mem.alloc(0x10);
            e.register_double(TABLE_ALLOCATE, move |e, a| {
                assert_eq!(a, &[4]); // 0x40000001 * 4 wraps.
                e.mem.set_u32(map + 4, 2);
                table.into_ret()
            });
            e.register(MEMSET, |_, _| Ret::default());
            e.mem.set_u32(map + 12, 9);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(address, &args![map, 0x4000_0001u32]).u32(), map);
            let log = take(&mut e);
            assert_eq!(e.mem.u32(map), vtable);
            assert_eq!(e.mem.u32(map + 8), table);
            assert_eq!(e.mem.u32(map + 12), 0);
            assert_eq!(calls(&log, MEMSET), vec![vec![table, 0, 8]]);
        }
    }

    #[test]
    fn grid_table_deleting_destructors() {
        for (address, flags, deletes) in [
            (0x006e_0ef0, 1u32, true),
            (0x006e_0ef0, 0, false),
            (0x006e_11f0, 1, true),
            (0x006e_11f0, 2, false),
        ] {
            let mut e = Engine::new();
            stub(&mut e, &[MAP_REMOVE_ALL, TABLE_FREE, OPERATOR_DELETE]);
            let map = e.mem.alloc(0x10);
            e.mem.set_u32(map + 8, 0x4240);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(address, &args![map, flags]).u32(), map);
            let log = take(&mut e);
            let removes = if address == 0x006e_0ef0 { 2 } else { 1 };
            assert_eq!(calls(&log, MAP_REMOVE_ALL).len(), removes);
            assert_eq!(calls(&log, TABLE_FREE), vec![vec![0x4240]]);
            assert_eq!(calls(&log, OPERATOR_DELETE).len(), usize::from(deletes));
            assert_eq!(e.mem.u32(map), GRID_TABLE_BASE_VTABLE);
        }
    }

    #[test]
    fn grid_table_destructor_bodies() {
        let mut e = Engine::new();
        // A vtable in place while the items go.
        e.register(MAP_REMOVE_ALL, |e, a| {
            let vtable = e.mem.u32(a[0]);
            assert!(vtable == GRID_TABLE_VTABLE || vtable == GRID_TABLE_BASE_VTABLE);
            Ret::default()
        });
        stub(&mut e, &[TABLE_FREE]);
        let map = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x006e_0f90, &args![map]);
        let log = take(&mut e);
        assert_eq!(calls(&log, MAP_REMOVE_ALL).len(), 2);
        e.mem.set_u32(map, 0);
        e.call_log = Some(vec![]);
        e.call(0x006e_0ff0, &args![map]);
        let log = take(&mut e);
        assert_eq!(calls(&log, MAP_REMOVE_ALL).len(), 1);
        assert_eq!(calls(&log, TABLE_FREE).len(), 1);
        assert_eq!(e.mem.u32(map), GRID_TABLE_BASE_VTABLE);
    }

    #[test]
    fn array_constructors_and_destructors() {
        let mut e = Engine::new();
        stub(
            &mut e,
            &[
                CELL_ARRAY_INIT,
                CHAR_ARRAY_INIT,
                ARRAY_RELEASE_008454F0,
                OPERATOR_DELETE,
            ],
        );
        let array = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006e_1020, &args![array]).u32(), array);
        assert_eq!(e.mem.u32(array), CELL_ARRAY_VTABLE);
        assert_eq!(e.call(0x006e_1070, &args![array]).u32(), array);
        assert_eq!(e.mem.u32(array), ARRAY_VTABLE_0106CF0C);
        e.call(0x006e_1050, &args![array]);
        assert_eq!(e.mem.u32(array), CELL_ARRAY_VTABLE);
        let log = take(&mut e);
        assert_eq!(calls(&log, CELL_ARRAY_INIT), vec![vec![array, 0, 0]]);
        assert_eq!(calls(&log, CHAR_ARRAY_INIT), vec![vec![array, 0, 0]]);
        assert_eq!(calls(&log, ARRAY_RELEASE_008454F0), vec![vec![array, 1]]);
        for (address, vtable) in [
            (0x006e_1220, CELL_ARRAY_VTABLE),
            (0x006e_1250, ARRAY_VTABLE_0106CF0C),
        ] {
            for flags in [0u32, 1] {
                e.call_log = Some(vec![]);
                assert_eq!(e.call(address, &args![array, flags]).u32(), array);
                let log = take(&mut e);
                assert_eq!(e.mem.u32(array), vtable);
                assert_eq!(calls(&log, OPERATOR_DELETE).len(), flags as usize);
            }
        }
    }

    #[test]
    fn char_array_set_size_branches() {
        let mut e = Engine::new();
        stub(
            &mut e,
            &[
                ARRAY_RELEASE_008454F0,
                CHAR_ARRAY_REALLOCATE,
                CHAR_ARRAY_CONSTRUCT_ELEMENTS,
                CHAR_ARRAY_DESTROY_ELEMENTS,
            ],
        );
        e.put_vtable(0x5000, &[0x5100, 0x5104]);
        e.register(0x5104, |_, _| 0x8000u32.into_ret());
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array, 0x5000);
        let state = |e: &Engine| {
            (
                e.mem.u32(array + 4),
                e.mem.u32(array + 8),
                e.mem.u32(array + 12),
            )
        };
        // Size 0: release with the shrink byte.
        e.call_log = Some(vec![]);
        e.call(0x006e_10c0, &args![array, 0u32, 0x100u32]);
        assert_eq!(
            calls(&take(&mut e), ARRAY_RELEASE_008454F0),
            vec![vec![array, 0]]
        );
        // Nothing reserved: allocate through the vtable, construct all.
        e.call_log = Some(vec![]);
        e.call(0x006e_10c0, &args![array, 8u32, 1u32]);
        let log = take(&mut e);
        assert_eq!(calls(&log, 0x5104), vec![vec![array, 8]]);
        assert_eq!(
            calls(&log, CHAR_ARRAY_CONSTRUCT_ELEMENTS),
            vec![vec![array, 0x8000, 8]]
        );
        assert_eq!(state(&e), (0x8000, 8, 8));
        // Past the reservation: reallocate keeping the size, construct the rest.
        e.call_log = Some(vec![]);
        e.call(0x006e_10c0, &args![array, 12u32, 1u32]);
        let log = take(&mut e);
        assert_eq!(calls(&log, CHAR_ARRAY_REALLOCATE), vec![vec![array, 12, 8]]);
        assert_eq!(
            calls(&log, CHAR_ARRAY_CONSTRUCT_ELEMENTS),
            vec![vec![array, 0x8008, 4]]
        );
        assert_eq!(state(&e), (0x8000, 12, 12));
        // Shrinking, not below a quarter: destroy only.
        e.call_log = Some(vec![]);
        e.call(0x006e_10c0, &args![array, 4u32, 1u32]);
        let log = take(&mut e);
        assert_eq!(
            calls(&log, CHAR_ARRAY_DESTROY_ELEMENTS),
            vec![vec![array, 0x8004, 8]]
        );
        assert!(calls(&log, CHAR_ARRAY_REALLOCATE).is_empty());
        assert_eq!(state(&e), (0x8000, 4, 12));
        // Shrinking to a quarter with shrink set: reallocate to the size.
        e.call_log = Some(vec![]);
        e.call(0x006e_10c0, &args![array, 3u32, 1u32]);
        let log = take(&mut e);
        assert_eq!(calls(&log, CHAR_ARRAY_REALLOCATE), vec![vec![array, 3, 3]]);
        assert_eq!(state(&e), (0x8000, 3, 3));
        // Shrinking without shrink keeps the reservation.
        e.mem.set_u32(array + 12, 100);
        e.call_log = Some(vec![]);
        e.call(0x006e_10c0, &args![array, 1u32, 0u32]);
        assert!(calls(&take(&mut e), CHAR_ARRAY_REALLOCATE).is_empty());
        assert_eq!(state(&e), (0x8000, 1, 100));
        // Growing within the reservation: construct the new elements.
        e.call_log = Some(vec![]);
        e.call(0x006e_10c0, &args![array, 5u32, 0u32]);
        let log = take(&mut e);
        assert_eq!(
            calls(&log, CHAR_ARRAY_CONSTRUCT_ELEMENTS),
            vec![vec![array, 0x8001, 4]]
        );
        assert_eq!(state(&e), (0x8000, 5, 100));
    }
}
