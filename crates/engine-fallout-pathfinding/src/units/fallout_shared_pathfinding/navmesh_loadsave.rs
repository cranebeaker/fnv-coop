//! `fallout shared/pathfinding/navmesh_loadsave.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `NavMesh::Load` reads a `NAVM` record's chunks (`NVER`, `DATA`, `NVVX`,
//! `NVTR`, `NVCA`, `NVDP`, `NVGD`, `NVEX`, `EDID`, `OBND`) into the mesh's
//! arrays and then upgrades data saved by older versions. The rest of the
//! unit is the mesh's form overrides (detailed string, deletion and
//! disabling, file-group matching), the copy of one mesh into another, and
//! the template instances the unit emits: `BSSimpleArray<..>::SetSize` for
//! six element types, array copies, the obstacle map
//! (`NiTMap<unsigned short, NiPointer<ObstacleData>>`) and the
//! point-of-view map (`NiTMap<unsigned short, ..>` with a 4-byte value at
//! +6 of each item), and the `BSSimpleArray<int>` constructor and
//! destructors.
//!
//! Translated so far (address order): the whole unit, `005eee80` and
//! `00692950` to `00695230`.
//!
//! The `NavMesh` layout is the one in `navmesh.rs`; `NavMeshInfo` fields are
//! read at their offsets (`INFO_*`). Many functions here are unnamed in the
//! engine map; they are named `fn_<address>` and described by their bodies.
//!
//! Left out everywhere: the compiler's exception-unwinding frames and the
//! `/GS` cookie check (`00ec408c`); the functions that have them carry a
//! `Translation incomplete` marker.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{BSSimpleArray, NiTPointerMap};
use crate::units::fallout_shared_pathfinding::navmesh::{NavMesh, NavMeshTriangle};

// ---------------------------------------------------------------------------
// Layouts and constants
// ---------------------------------------------------------------------------

/// `BSSimpleArray<int, 1024>` vtable (RTTI `.?AV?$BSSimpleArray@H$0EAA@@`).
const INT_ARRAY_VTABLE: u32 = 0x0106_b0dc;

/// The `TES` singleton pointer.
const TES_INSTANCE: u32 = 0x011d_ea10;
/// A cell the loader uses in place of a navmesh's missing parent cell when
/// set (`DATA` chunk).
const FALLBACK_PARENT_CELL: u32 = 0x011c_3f30;
/// The word the file-group functions (`006940e0`, `006941c0`) compare a
/// group record's first word with and store into one; zero in the exe's
/// image, set at run time.
const GROUP_RECORD_TAG: u32 = 0x0118_7020;
/// The object `00408d60` is called on at the end of `NavMesh::Load`.
const LOAD_CHECK_OBJECT: u32 = 0x011d_6ca8;

/// `"D:\_Fallout3\Platforms\Common\Code\Fallout Shared\Pathfinding\NavMesh_LoadSave.cpp"`.
const SOURCE_FILE_NAME: u32 = 0x0106_af98;
/// `"NavMesh: Door reference %08X is not found but is linked to %s.  ..."`.
const DOOR_NOT_FOUND_FORMAT: u32 = 0x0106_aeb0;
/// `"MASTERFILE: Navmesh %s has a bad navmesh grid, please touch and resave"`.
const BAD_GRID_FORMAT: u32 = 0x0106_ae68;
/// `"CELL: Could not find parent cell (%08X) for NavMesh (%08X)."`.
const NO_PARENT_CELL_FORMAT: u32 = 0x0106_af5c;
/// `"CELL: Attempting to remap NavMesh parent to cell (%08X) instead."`.
const REMAP_PARENT_FORMAT: u32 = 0x0106_af18;
/// `"MASTERFILE: %s has unresolved warnings and will not be loaded."`.
const UNRESOLVED_FORMAT: u32 = 0x0106_ae24;
/// `"Depreciated NavMesh Door Portals found in %s, removing portals, ..."`.
const OLD_DOOR_PORTALS_FORMAT: u32 = 0x0106_adb0;
/// `"%s had cover that could not be converted to the new format, ..."`.
const COVER_FORMAT: u32 = 0x0106_ad58;
/// `"MASTERFILE: Did not find matching NavMeshInfo for NavMesh ID %08X, Creating a temporary one"`.
const NO_INFO_FORMAT: u32 = 0x0106_acf8;
/// `"NavMesh (%08X) with %d Triangles in Cell %s"`.
const DETAILED_STRING_FORMAT: u32 = 0x0106_afec;
/// `"DisableNavMesh Script Function: Did not find matching NavMeshInfo for NavMesh ID parameter %08X"`.
const DISABLE_NO_INFO_FORMAT: u32 = 0x0106_b078;
/// `"DisableNavMesh Script Function: Did not find matching NavMesh for NavMesh ID parameter %08X"`.
const DISABLE_NO_MESH_FORMAT: u32 = 0x0106_b018;

/// RTTI type descriptors of `TESForm` and `TESObjectCELL`.
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_OBJECT_CELL: u32 = 0x0118_3fb4;

/// Chunk types (four characters, little-endian).
const CHUNK_NVDP: u32 = 0x5044_564e;
const CHUNK_NVGD: u32 = 0x4447_564e;
const CHUNK_NVCA: u32 = 0x4143_564e;
const CHUNK_DATA: u32 = 0x4154_4144;
const CHUNK_EDID: u32 = 0x4449_4445;
const CHUNK_OBND: u32 = 0x444e_424f;
const CHUNK_NVEX: u32 = 0x5845_564e;
const CHUNK_NVER: u32 = 0x5245_564e;
const CHUNK_NVTR: u32 = 0x5254_564e;
const CHUNK_NVVX: u32 = 0x5856_564e;
/// The form-type byte `00472660` must return for a navmesh record.
const NAV_MESH_FORM_TYPE: u8 = 0x43;

/// Offsets in `NavMesh` used through other units' functions.
const MESH_REF_OBJECT_BASE: u32 = 0x1c;
const MESH_VERTICES: u32 = 0x28;
const MESH_TRIANGLES: u32 = 0x38;
const MESH_EXTRA_EDGE_INFO: u32 = 0x48;
const MESH_DOOR_PORTALS: u32 = 0x58;
const MESH_CLOSED_DOORS: u32 = 0x68;
const MESH_COVER: u32 = 0x78;
const MESH_POVS: u32 = 0x88;
const MESH_CLOSEST_POVS: u32 = 0x98;
const MESH_GRID: u32 = 0xa8;
const MESH_OBSTACLES: u32 = 0xd0;
const MESH_OBSTACLE_MAP: u32 = 0xe0;
const MESH_OBSTACLE_POVS: u32 = 0xe4;
const MESH_PARENT_CELL: u32 = 0x24;
const MESH_INFO: u32 = 0x104;

/// `NavMeshInfo` fields (read at their offsets): the parent space's form
/// id, the flag word, the float at +0x20, the two int arrays at +0x24 and
/// +0x34, and the map at +0x1c of the `NavMeshInfoMap` (`00693d70`).
const INFO_PARENT_SPACE_ID: u32 = 0x04;
const INFO_FLAGS: u32 = 0x08;
const INFO_FLOAT_20: u32 = 0x20;
const INFO_ARRAY_24: u32 = 0x24;
const INFO_ARRAY_34: u32 = 0x34;
const INFO_MAP_MAP: u32 = 0x1c;

/// `NavMeshTriangle` flag word offset (+0x0C).
const TRIANGLE_FLAGS: u32 = 0x0c;

// Callees outside this unit.
/// `TESFile`: the chunk's data size (`+0x25c`).
const FILE_CHUNK_SIZE: u32 = 0x0040_1660;
/// `TESFile`: whether the file's data must be byte-swapped (`+0x299`).
const FILE_NEEDS_SWAP: u32 = 0x0040_1680;
/// `TESFile::GetTESForm` (engine map): the form type byte of the record.
const FILE_GET_FORM_TYPE: u32 = 0x0047_2660;
/// `TESFile::GetTESChunk` (engine map): the current chunk's type.
const FILE_GET_CHUNK: u32 = 0x0047_26b0;
/// `TESFile`: moves to the next chunk, false at the end of the record.
const FILE_NEXT_CHUNK: u32 = 0x0047_26f0;
/// `TESFile::GetChunkData` (engine map, `_ov3`): `(buffer, size)`.
const FILE_GET_CHUNK_DATA: u32 = 0x0047_2890;
/// `TESFile::GetMaster` (engine map).
const FILE_GET_MASTER: u32 = 0x0047_1c20;
/// Byte-swaps the `u32` at the address (cdecl, second word unused).
const SWAP_U32: u32 = 0x0040_1080;
/// Byte-swaps the `u16` at the address (cdecl, second word unused).
const SWAP_U16: u32 = 0x0040_7a90;
/// Byte-swaps three `u32` (a point).
const SWAP_POINT: u32 = 0x005a_6b10;
/// Byte-swaps a door portal (a `u32`, then a `u16` at +4).
const SWAP_DOOR_PORTAL: u32 = 0x0041_41e0;
/// The allocator (cdecl, size) and its free (cdecl, block).
const ALLOCATE: u32 = 0x0040_1000;
const FREE: u32 = 0x0040_1030;
/// `memcpy` wrapper (cdecl, destination, source, size).
const COPY_MEMORY: u32 = 0x0040_1460;
/// A 4-byte scope object constructed with `(0x2e, 1, file name, line)` and
/// destroyed at the end of `NavMesh::Load`.
const SCOPE_CONSTRUCT: u32 = 0x0040_4eb0;
const SCOPE_DESTRUCT: u32 = 0x0040_4ee0;
/// Called on the mesh's `+0x1c` when `NavMesh::Load` finds the flag 0x20 set
/// on entry.
const REF_OBJECT_ENTRY_CALL: u32 = 0x0040_1970;
/// Called on the mesh's `+0x1c` when the flag 0x20 is set after loading.
const REF_OBJECT_DELETED_CALL: u32 = 0x0040_f6e0;
/// Tests `TESForm::iFormFlags & 0x20` (the flag `TESForm::SetDelete` sets,
/// see `00693fe0`).
const FORM_FLAG_20: u32 = 0x0044_0d80;
/// Tests `TESForm::iFormFlags & 0x800` (the flag `TESForm::SetDisabled`
/// changes, see `00694500`).
const FORM_FLAG_800: u32 = 0x0044_0da0;
/// `TESForm::LoadForm` (engine map).
const FORM_LOAD_FORM: u32 = 0x0048_5110;
/// `TESForm::SetDelete` (engine map).
const FORM_SET_DELETE: u32 = 0x0048_4530;
/// `TESForm::SetDisabled` (engine map).
const FORM_SET_DISABLED: u32 = 0x0048_4af0;
/// `TESForm::GetFile` (engine map).
const FORM_GET_FILE: u32 = 0x0048_4e60;
/// `TESForm::SetFormID` (engine map).
const FORM_SET_FORM_ID: u32 = 0x0048_5c10;
/// `TESForm::AddCompileIndex` (engine map; cdecl, form id address, file).
const FORM_ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// Looks a form up by id (cdecl).
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// The word at +0x0C (a form's `iFormID`).
const GET_FORM_ID: u32 = 0x0084_e3a0;
/// The word at +0x24 (the mesh's parent cell).
const GET_PARENT_CELL: u32 = 0x0059_bb30;
/// `__RTDynamicCast`.
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// Debug print (cdecl, format, ...).
const PRINT: u32 = 0x005b_5e40;
/// `BSStringT` constructor, destructor, `sprintf` and character pointer.
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
const STRING_DESTRUCT: u32 = 0x0040_37d0;
const STRING_PRINTF: u32 = 0x0040_6f60;
const DEREFERENCE: u32 = 0x0055_9450;
/// `TES::GetNavMeshInfoMap` (engine map).
const TES_GET_NAV_MESH_INFO_MAP: u32 = 0x0045_af00;
/// `BSSimpleArray` size (the word at +8).
const ARRAY_SIZE: u32 = 0x0044_ddc0;
/// `BSSimpleArray` element addresses: 12-byte (`006a1440`), 16-byte
/// (`00558dd0`), 2-byte (`005eeda0`) and 4-byte (`00877a30`) elements.
const ELEMENT_12: u32 = 0x006a_1440;
const ELEMENT_16: u32 = 0x0055_8dd0;
const ELEMENT_2: u32 = 0x005e_eda0;
const ELEMENT_4: u32 = 0x0087_7a30;
/// `BSSimpleArray<BGSBodyPart *>::AddUninitialized` (engine map; the code
/// is the one shared by every 4-byte element array): appends `*value`.
const ARRAY_APPEND_4: u32 = 0x007c_b2e0;
/// `BSSimpleArray<BGSBodyPart *>::_MoveItems` (engine map), here
/// `(index, 1)` on an int array: removes an element.
const ARRAY_REMOVE_4: u32 = 0x009a_4320;
/// Removes element `index` of the door portal array (`(index, 1)`).
const DOOR_PORTALS_REMOVE: u32 = 0x0084_4c90;
/// `BSSimpleArray<int>` initialisation (`(this, 0, 0)`).
const ARRAY_INIT: u32 = 0x006b_3eb0;
/// `BSSimpleArray` clear (`(this, free buffer)`).
const ARRAY_CLEAR: u32 = 0x0084_54f0;
/// Clears the given bits of a triangle's flags.
const TRIANGLE_CLEAR_FLAGS: u32 = 0x0051_9270;
/// `NavMeshTriangle`: true when any bit of the mask is set.
const TRIANGLE_HAS_FLAGS: u32 = 0x0069_1140;
/// `NavMeshTriangle`: whether any edge has extra info.
const TRIANGLE_HAS_EDGE_INFO: u32 = 0x0069_0e10;
/// `NavMeshTriangle`: whether edge `n` has extra info.
const TRIANGLE_EDGE_HAS_INFO: u32 = 0x0068_f1d0;
/// `NavMeshTriangle`: the neighbour slot of edge `n`.
const TRIANGLE_NEIGHBOUR: u32 = 0x0068_f2c0;
/// `NavMesh`: triangle `index` (16-byte record).
const MESH_GET_TRIANGLE: u32 = 0x0055_82f0;
/// `NavMesh`: `pNavMeshInfo`.
const MESH_GET_INFO: u32 = 0x0068_efb0;
/// `NavMesh::GetParentSpace` (engine map).
const MESH_GET_PARENT_SPACE: u32 = 0x0068_ef70;
/// `NavMesh::CheckNavMesh` (engine map).
const MESH_CHECK: u32 = 0x0068_f6d0;
/// `NavMesh::ClearNavMesh` (engine map).
const MESH_CLEAR: u32 = 0x0069_0d50;
/// `NavMesh::DeleteExtraEdgeInfo` (engine map).
const MESH_DELETE_EXTRA_EDGE_INFO: u32 = 0x0069_0e90;
/// `NavMesh`: rebuilds the cover array.
const MESH_REBUILD_COVER: u32 = 0x0069_10a0;
/// `NavMesh`: called when the grid chunk does not load.
const MESH_BAD_GRID: u32 = 0x0069_59b0;
/// `NavMesh`: called when no `NavMeshInfo` matches the mesh.
const MESH_CREATE_TEMPORARY_INFO: u32 = 0x0069_86d0;
/// `NavMesh`: called when the flag 0x20 is clear in `00693e60`.
const MESH_FLAG_20_CLEAR_CALL: u32 = 0x0069_9b60;
/// `NavMesh`: called after enabling a mesh (`00694500`).
const MESH_ENABLED_CALL: u32 = 0x0069_aa20;
/// `NavMesh::FixPortalsFromNavMeshDeletion` (engine map; cdecl, info).
const MESH_FIX_PORTALS: u32 = 0x0069_53b0;
/// `NavMeshGrid::Load` (engine map): `(buffer, size, file)`.
const GRID_LOAD: u32 = 0x0069_da40;
/// Copies one `NavMeshGrid` into another.
const GRID_COPY: u32 = 0x0069_dd50;
/// `EdgeExtraInfo` loaders: `(file, data)` for versions below 9 and from 9.
const EDGE_INFO_LOAD_OLD: u32 = 0x0069_d240;
const EDGE_INFO_LOAD: u32 = 0x0069_d1d0;
/// `NavMeshInfo`: the parent space (`pParentSpace` or the form found by id).
const INFO_GET_PARENT_SPACE: u32 = 0x0069_0830;
/// `NavMeshInfo`: puts its `pNavMesh` into a `NiPointer` slot; true when set.
const INFO_GET_NAV_MESH: u32 = 0x0069_0860;
/// Called with `(info, mesh)` after a mesh found its `NavMeshInfo` (the
/// engine map's `IOManager::Pause` is a folded name).
const INFO_ATTACH_MESH: u32 = 0x005a_8040;
/// `NavMeshInfoMap::DeleteNavMeshInfo` (engine map).
const INFO_MAP_DELETE: u32 = 0x006b_6a00;
/// `NavMeshInfoMap`: called with the mesh's form id when the loaded mesh
/// has the flag 0x20.
const INFO_MAP_REMOVE: u32 = 0x006b_6990;
/// A `NiTMap` lookup: `(key, out value)`, true when found.
const MAP_GET_AT: u32 = 0x0085_3130;
/// The first position of a `NiTMap`.
const MAP_GET_FIRST_POS: u32 = 0x004b_9ba0;
/// `NiPointer`: construct from a raw pointer, copy-construct, assign,
/// release.
const POINTER_CONSTRUCT: u32 = 0x0063_3c90;
const POINTER_COPY: u32 = 0x0055_9a40;
const POINTER_ASSIGN: u32 = 0x006e_5cc0;
const POINTER_RELEASE: u32 = 0x0045_cec0;
/// A `NiPointer<NavMesh>` slot's constructor and destructor (`00694500`).
const MESH_POINTER_CONSTRUCT: u32 = 0x0042_fb00;
const MESH_POINTER_DESTRUCT: u32 = 0x0042_fa40;

/// `ObstacleData` and `NavMeshPOVData` map item fields: next item, key and
/// value (the point-of-view map stores its 4-byte value at +6).
const ITEM_KEY: u32 = 0x04;
const OBSTACLE_ITEM_VALUE: u32 = 0x08;
const POV_ITEM_VALUE: u32 = 0x06;

type Map = Ptr<NiTPointerMap>;
type Array = Ptr<BSSimpleArray>;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn word(e: &Engine, address: u32) -> u32 {
    e.mem.u32(address)
}

fn flag_20(e: &mut Engine, form: u32) -> bool {
    e.call(FORM_FLAG_20, &args![form]).bool()
}

fn array_size(e: &mut Engine, array: u32) -> u32 {
    e.call(ARRAY_SIZE, &args![array]).u32()
}

fn nav_mesh_info_map(e: &mut Engine) -> u32 {
    let tes: u32 = e.global(TES_INSTANCE);
    e.call(TES_GET_NAV_MESH_INFO_MAP, &args![tes]).u32()
}

fn form_id(e: &mut Engine, form: u32) -> u32 {
    e.call(GET_FORM_ID, &args![form]).u32()
}

/// A temporary `BSStringT` filled by the mesh's virtual slot `+0x90`
/// (`GetFormDetailedString`), passed as its character pointer to `f`.
fn with_detailed_string(e: &mut Engine, mesh: u32, f: impl FnOnce(&mut Engine, u32)) {
    e.with_stack(8, |e, text| {
        let text = text.addr();
        e.call(STRING_CONSTRUCT, &args![text]);
        e.vcall(mesh, 0x90, &args![text]);
        let chars = e.call(DEREFERENCE, &args![text]).u32();
        f(e, chars);
        e.call(STRING_DESTRUCT, &args![text]);
    });
}

/// The callees of one `BSSimpleArray<T, 1024>::SetSize` instance.
struct SetSizeCallees {
    element_size: u32,
    clear: u32,
    reallocate: u32,
    construct: u32,
    destruct: u32,
}

/// `BSSimpleArray<T, 1024>::SetSize`: sets the number of elements to
/// `count`. Zero clears (freeing the buffer with `shrink`); more than the
/// capacity allocates (virtual slot `+4`) or reallocates to exactly `count`
/// and constructs the new elements; fewer than now destroys the dropped
/// ones (and, with `shrink`, reallocates when `count` is at most a quarter
/// of the capacity); otherwise constructs the new elements.
fn simple_array_set_size(
    e: &mut Engine,
    this: Array,
    count: u32,
    shrink: bool,
    callees: &SetSizeCallees,
) {
    let n = callees.element_size;
    if count == 0 {
        e.call(callees.clear, &args![this, shrink]);
        return;
    }
    let reserved = e.get(this, BSSimpleArray::iReservedSize);
    if count > reserved {
        if reserved == 0 {
            let buffer = e.vcall(this.addr(), 0x04, &args![count]).u32();
            e.set(this, BSSimpleArray::pBuffer, buffer);
        } else {
            let size = e.get(this, BSSimpleArray::iSize);
            e.call(callees.reallocate, &args![this, count, size]);
        }
        e.set(this, BSSimpleArray::iReservedSize, count);
        let size = e.get(this, BSSimpleArray::iSize);
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        let first = buffer.wrapping_add(size.wrapping_mul(n));
        e.call(
            callees.construct,
            &args![this, first, count.wrapping_sub(size)],
        );
        e.set(this, BSSimpleArray::iSize, count);
    } else if count < e.get(this, BSSimpleArray::iSize) {
        let size = e.get(this, BSSimpleArray::iSize);
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        let first = buffer.wrapping_add(count.wrapping_mul(n));
        e.call(
            callees.destruct,
            &args![this, first, size.wrapping_sub(count)],
        );
        e.set(this, BSSimpleArray::iSize, count);
        if shrink && count <= e.get(this, BSSimpleArray::iReservedSize) >> 2 {
            e.call(callees.reallocate, &args![this, count, count]);
            e.set(this, BSSimpleArray::iReservedSize, count);
        }
    } else {
        let size = e.get(this, BSSimpleArray::iSize);
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        let first = buffer.wrapping_add(size.wrapping_mul(n));
        e.call(
            callees.construct,
            &args![this, first, count.wrapping_sub(size)],
        );
        e.set(this, BSSimpleArray::iSize, count);
    }
}

/// The shape of the array copies (`00694e30` and the five after it):
/// `destination` gets the source's size (`set_size(.., true)`), then each
/// element `i` of `destination` (`element`) is passed with the source's
/// element `i` to `copy`.
fn copy_array(
    e: &mut Engine,
    this: Array,
    destination: Array,
    set_size: fn(&mut Engine, Array, u32, bool),
    mut copy: impl FnMut(&mut Engine, u32, u32),
) {
    let count = e.get(this, BSSimpleArray::iSize);
    set_size(e, destination, count, true);
    let mut index = 0u32;
    while index < e.get(this, BSSimpleArray::iSize) {
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        copy(e, index, buffer);
        index += 1;
    }
}

/// Moves the map position at `position` to the item after `item` (the next
/// in its bucket, else the first item of a later bucket, else 0), as the
/// `GetNext` functions of the two maps do.
fn map_advance(e: &mut Engine, this: Map, position: Ptr, item: u32) {
    let next = word(e, item);
    if next != 0 {
        e.mem.set_u32(position.addr(), next);
        return;
    }
    let key = e.mem.u16(item + ITEM_KEY) as u32;
    let mut bucket = e.vcall(this.addr(), 0x04, &args![key]).u32();
    loop {
        bucket = bucket.wrapping_add(1);
        if bucket >= e.get(this, NiTPointerMap::m_uiHashSize) {
            e.mem.set_u32(position.addr(), 0);
            return;
        }
        let table = e.get(this, NiTPointerMap::m_ppkHashTable);
        let head = word(e, table.wrapping_add(bucket.wrapping_mul(4)));
        if head != 0 {
            e.mem.set_u32(position.addr(), head);
            return;
        }
    }
}

/// Links a new item at the head of bucket `bucket` and counts it.
fn map_link_item(e: &mut Engine, this: Map, bucket: u32, item: u32) {
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    let head = word(e, table.wrapping_add(bucket.wrapping_mul(4)));
    e.mem.set_u32(item, head);
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    e.mem
        .set_u32(table.wrapping_add(bucket.wrapping_mul(4)), item);
    let count = e.get(this, NiTPointerMap::m_uiCount);
    e.set(this, NiTPointerMap::m_uiCount, count.wrapping_add(1));
}

// ---------------------------------------------------------------------------
// Functions
// ---------------------------------------------------------------------------

// Translated from 005eee80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<unsigned short, 1024>::SetSize` (Xbox PDB): see
/// [`simple_array_set_size`]; 2-byte elements, reallocated by `005ef6a0`,
/// constructed by `005ef650`, destroyed by `0072ba80`.
pub fn bs_simple_array_unsigned_short_set_size(
    e: &mut Engine,
    this: Array,
    count: u32,
    shrink: bool,
) {
    simple_array_set_size(
        e,
        this,
        count,
        shrink,
        &SetSizeCallees {
            element_size: 2,
            clear: ARRAY_CLEAR,
            reallocate: 0x005e_f6a0,
            construct: 0x005e_f650,
            destruct: 0x0072_ba80,
        },
    );
}

/// The `DATA` chunk: 0x14 bytes before version 5 (no door portal count),
/// 0x18 from 5: parent cell id and the counts of vertices, triangles, extra
/// edge infos, cover triangles and door portals. Sizes the arrays, resolves
/// the parent cell (or falls back to [`FALLBACK_PARENT_CELL`]). A triangle
/// count of `0x10000000` or more is a marker: it is reduced by `0x10000000`
/// and the mesh is later rejected (`unresolved`).
fn load_data_chunk(
    e: &mut Engine,
    this: Ptr<NavMesh>,
    file: Ptr,
    version: i32,
    data: u32,
    unresolved: &mut bool,
) {
    let mesh = this.addr();
    if version < 5 {
        e.call(FILE_GET_CHUNK_DATA, &args![file, data, 0x14u32]);
        e.mem.set_u32(data + 0x14, 0);
    } else {
        e.call(FILE_GET_CHUNK_DATA, &args![file, data, 0x18u32]);
    }
    if e.call(FILE_NEEDS_SWAP, &args![file]).bool() {
        fn_00693dc0(e, Ptr::new(data));
    }
    let triangles = word(e, data + 8);
    if triangles >= 0x1000_0000 {
        e.mem.set_u32(data + 8, triangles.wrapping_sub(0x1000_0000));
        *unresolved = true;
    }
    let count = word(e, data + 4);
    fn_00694890(e, Ptr::new(mesh + MESH_VERTICES), count, true);
    let count = word(e, data + 8);
    fn_006949d0(e, Ptr::new(mesh + MESH_TRIANGLES), count, true);
    let count = word(e, data + 0x0c);
    fn_00694b10(e, Ptr::new(mesh + MESH_EXTRA_EDGE_INFO), count, true);
    let count = word(e, data + 0x10);
    bs_simple_array_unsigned_short_set_size(e, Ptr::new(mesh + MESH_COVER), count, true);
    let count = word(e, data + 0x14);
    bs_simple_array_nav_mesh_triangle_door_portal_set_size(
        e,
        Ptr::new(mesh + MESH_DOOR_PORTALS),
        count,
        true,
    );
    e.call(FORM_ADD_COMPILE_INDEX, &args![data, file]);
    let cell_id = word(e, data);
    let form = e.call(LOOKUP_FORM, &args![cell_id]).u32();
    let cell = e
        .call(
            RT_DYNAMIC_CAST,
            &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_OBJECT_CELL, 0u32],
        )
        .u32();
    e.set(this, NavMesh::pParentCell, Ptr::new(cell));
    if e.get(this, NavMesh::pParentCell).addr() == 0 {
        let id = form_id(e, mesh);
        let cell_id = word(e, data);
        e.call(PRINT, &args![NO_PARENT_CELL_FORMAT, cell_id, id]);
        let fallback: u32 = e.global(FALLBACK_PARENT_CELL);
        if fallback != 0 {
            let fallback_id = form_id(e, fallback);
            e.call(PRINT, &args![REMAP_PARENT_FORMAT, fallback_id]);
            let fallback: u32 = e.global(FALLBACK_PARENT_CELL);
            e.set(this, NavMesh::pParentCell, Ptr::new(fallback));
        }
    }
}

/// The `NVDP` chunk: the door portals (8 bytes: door reference id, then the
/// triangle at +4). Each id is resolved to its form; a portal whose door is
/// not found is reported, its triangle (when in range) loses flag `0x400`,
/// and it is removed.
fn load_door_portals_chunk(e: &mut Engine, this: Ptr<NavMesh>, file: Ptr) {
    let mesh = this.addr();
    let portals = Ptr::new(mesh + MESH_DOOR_PORTALS);
    let mut count = array_size(e, portals.addr()) as i32;
    let first = fn_006950d0(e, portals, 0);
    e.call(
        FILE_GET_CHUNK_DATA,
        &args![file, first, (count as u32).wrapping_shl(3)],
    );
    let mut index = 0i32;
    while index < count {
        if e.call(FILE_NEEDS_SWAP, &args![file]).bool() {
            let portal = fn_006950d0(e, portals, index as u32);
            e.call(SWAP_DOOR_PORTAL, &args![portal]);
        }
        let portal = fn_006950d0(e, portals, index as u32);
        let door_id = word(e, portal);
        let (door_id, door) = e.with_stack(4, |e, id_slot| {
            e.mem.set_u32(id_slot.addr(), door_id);
            e.call(FORM_ADD_COMPILE_INDEX, &args![id_slot, file]);
            let door_id = word(e, id_slot.addr());
            let door = e.call(LOOKUP_FORM, &args![door_id]).u32();
            (door_id, door)
        });
        if door != 0 {
            let portal = fn_006950d0(e, portals, index as u32);
            e.mem.set_u32(portal, door);
        } else {
            with_detailed_string(e, mesh, |e, chars| {
                e.call(PRINT, &args![DOOR_NOT_FOUND_FORMAT, door_id, chars]);
                let portal = fn_006950d0(e, portals, index as u32);
                let triangle = e.mem.u16(portal + 4) as u32;
                if triangle < array_size(e, mesh + MESH_TRIANGLES) {
                    let record = e
                        .call(ELEMENT_16, &args![mesh + MESH_TRIANGLES, triangle])
                        .u32();
                    e.call(TRIANGLE_CLEAR_FLAGS, &args![record, 0x400u32]);
                }
                e.call(DOOR_PORTALS_REMOVE, &args![portals, index, true]);
            });
            index -= 1;
            count -= 1;
        }
        index += 1;
    }
}

/// The `NVEX` chunk: the extra edge infos, from a heap copy of the chunk.
/// Before version 5 each entry is a 0x30-byte record read 0x2e bytes apart
/// whose word at +8 marks an old door portal (the entry's index goes into
/// `old_portals`) and whose word at +4 and `u16` at +0x2c are passed on;
/// before 9 the data goes to the old loader (`0069d240`), from 9 to
/// `0069d1d0`, each followed by the entry's size.
fn load_extra_edge_info_chunk(
    e: &mut Engine,
    this: Ptr<NavMesh>,
    file: Ptr,
    version: i32,
    old_portals: Array,
) {
    let infos = this.addr() + MESH_EXTRA_EDGE_INFO;
    let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
    let buffer = e.call(ALLOCATE, &args![size]).u32();
    e.call(FILE_GET_CHUNK_DATA, &args![file, buffer, size]);
    let mut cursor = buffer;
    if version < 5 {
        let count = array_size(e, infos);
        let mut index = 0u32;
        while index < count {
            e.with_stack(0x30, |e, record| {
                let record = record.addr();
                e.call(COPY_MEMORY, &args![record, cursor, 0x30u32]);
                if word(e, record + 8) != 0 {
                    e.with_stack(4, |e, value| {
                        e.mem.set_u32(value.addr(), index);
                        e.call(ARRAY_APPEND_4, &args![old_portals, value]);
                    });
                }
                e.with_stack(8, |e, converted| {
                    let converted = converted.addr();
                    let portal = word(e, record + 4);
                    e.mem.set_u32(converted, portal);
                    let triangle = e.mem.u16(record + 0x2c);
                    e.mem.set_u16(converted + 4, triangle);
                    let info = e.call(ELEMENT_12, &args![infos, index]).u32();
                    e.call(EDGE_INFO_LOAD_OLD, &args![info, file, converted]);
                });
            });
            cursor = fn_00693db0(e).wrapping_add(cursor);
            index += 1;
        }
    } else if version < 9 {
        let count = array_size(e, infos);
        for index in 0..count {
            let info = e.call(ELEMENT_12, &args![infos, index]).u32();
            e.call(EDGE_INFO_LOAD_OLD, &args![info, file, cursor]);
            let info = e.call(ELEMENT_12, &args![infos, index]).u32();
            cursor = fn_00693d10(e, Ptr::new(info)).wrapping_add(cursor);
        }
    } else {
        let count = array_size(e, infos);
        for index in 0..count {
            let info = e.call(ELEMENT_12, &args![infos, index]).u32();
            e.call(EDGE_INFO_LOAD, &args![info, file, cursor]);
            let info = e.call(ELEMENT_12, &args![infos, index]).u32();
            cursor = fn_00693cd0(e, Ptr::new(info)).wrapping_add(cursor);
        }
    }
    e.call(FREE, &args![buffer]);
}

/// One chunk of the record (the body of `NavMesh::Load`'s loop).
fn load_chunk(
    e: &mut Engine,
    this: Ptr<NavMesh>,
    file: Ptr,
    frame: &LoadFrame,
    unresolved: &mut bool,
) {
    let mesh = this.addr();
    let chunk = e.call(FILE_GET_CHUNK, &args![file]).u32();
    let version = e.mem.i32(frame.version);
    let swap = |e: &mut Engine| e.call(FILE_NEEDS_SWAP, &args![file]).bool();
    match chunk {
        CHUNK_EDID => {
            let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
            // `_alloca_probe_16`: the block is padded so that its size is 4
            // modulo 16 (the return address completes the 16).
            let padded = size.wrapping_add(4u32.wrapping_sub(size) & 0xf);
            e.with_stack(padded, |e, buffer| {
                e.call(FILE_GET_CHUNK_DATA, &args![file, buffer, 0x200u32]);
                e.vcall(mesh, 0x134, &args![buffer]);
            });
        }
        CHUNK_OBND => {
            e.vcall(mesh, 0xe0, &args![file]);
        }
        CHUNK_NVER => {
            e.call(FILE_GET_CHUNK_DATA, &args![file, frame.version, 4u32]);
            if swap(e) {
                e.call(SWAP_U32, &args![frame.version, 0u32]);
            }
        }
        CHUNK_DATA => {
            load_data_chunk(e, this, file, version, frame.data, unresolved);
        }
        CHUNK_NVVX => {
            let vertices = mesh + MESH_VERTICES;
            let bytes = array_size(e, vertices).wrapping_mul(12);
            let first = e.call(ELEMENT_12, &args![vertices, 0u32]).u32();
            e.call(FILE_GET_CHUNK_DATA, &args![file, first, bytes]);
            if swap(e) {
                let count = array_size(e, vertices);
                for index in 0..count {
                    let vertex = e.call(ELEMENT_12, &args![vertices, index]).u32();
                    e.call(SWAP_POINT, &args![vertex]);
                }
            }
        }
        CHUNK_NVCA => {
            let cover = mesh + MESH_COVER;
            let bytes = array_size(e, cover).wrapping_shl(1);
            let first = e.call(ELEMENT_2, &args![cover, 0u32]).u32();
            e.call(FILE_GET_CHUNK_DATA, &args![file, first, bytes]);
            if swap(e) {
                let count = array_size(e, cover);
                for index in 0..count {
                    let item = e.call(ELEMENT_2, &args![cover, index]).u32();
                    e.call(SWAP_U16, &args![item, 0u32]);
                }
            }
        }
        CHUNK_NVDP => load_door_portals_chunk(e, this, file),
        CHUNK_NVTR => {
            let triangles = mesh + MESH_TRIANGLES;
            let bytes = array_size(e, triangles).wrapping_shl(4);
            let first = e.call(ELEMENT_16, &args![triangles, 0u32]).u32();
            e.call(FILE_GET_CHUNK_DATA, &args![file, first, bytes]);
            let count = array_size(e, triangles);
            for index in 0..count {
                if swap(e) {
                    let triangle = e.call(ELEMENT_16, &args![triangles, index]).u32();
                    fn_00693c10(e, Ptr::new(triangle));
                }
                let triangle = e.call(ELEMENT_16, &args![triangles, index]).u32();
                fn_00693bd0(e, Ptr::new(triangle), 0);
            }
        }
        CHUNK_NVGD => {
            let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
            let buffer = e.call(ALLOCATE, &args![size]).u32();
            e.call(FILE_GET_CHUNK_DATA, &args![file, buffer, size]);
            let loaded = e
                .call(GRID_LOAD, &args![mesh + MESH_GRID, buffer, size, file])
                .bool();
            if !loaded {
                with_detailed_string(e, mesh, |e, chars| {
                    e.call(PRINT, &args![BAD_GRID_FORMAT, chars]);
                    e.call(MESH_BAD_GRID, &args![mesh]);
                });
            }
            e.call(FREE, &args![buffer]);
        }
        CHUNK_NVEX => {
            load_extra_edge_info_chunk(e, this, file, version, Ptr::new(frame.old_portals));
        }
        _ => {}
    }
}

/// `NavMesh::Load`'s locals that the game keeps in memory: the scope
/// object, the old-door-portal index array (`BSSimpleArray<int>`), the
/// record version and the `DATA` chunk.
struct LoadFrame {
    scope: u32,
    old_portals: u32,
    version: u32,
    data: u32,
}

/// The upgrades after loading for a mesh without the flag 0x20; false when
/// there is no `NavMeshInfoMap`.
fn load_upgrade(e: &mut Engine, this: Ptr<NavMesh>, version: i32, old_portals: u32) -> bool {
    let mesh = this.addr();
    let triangles = mesh + MESH_TRIANGLES;
    if version < 5 {
        if array_size(e, old_portals) > 0 {
            with_detailed_string(e, mesh, |e, chars| {
                e.call(PRINT, &args![OLD_DOOR_PORTALS_FORMAT, chars]);
            });
        }
        let count = array_size(e, old_portals) as i32;
        let mut index = count.wrapping_sub(1);
        while index >= 0 {
            let item = e.call(ELEMENT_4, &args![old_portals, index]).u32();
            let info = word(e, item) & 0xffff;
            e.call(MESH_DELETE_EXTRA_EDGE_INFO, &args![mesh, info]);
            index -= 1;
        }
    }
    if version < 6 {
        // The flag is never set: the conversion it reports was removed.
        let cover_failed = false;
        let count = array_size(e, triangles);
        for index in 0..count {
            e.call(MESH_GET_TRIANGLE, &args![mesh, index & 0xffff]);
        }
        e.call(MESH_REBUILD_COVER, &args![mesh]);
        if cover_failed {
            with_detailed_string(e, mesh, |e, chars| {
                let id = form_id(e, mesh);
                e.call(PRINT, &args![COVER_FORMAT, id, chars]);
            });
        }
    }
    if version < 8 {
        let count = array_size(e, triangles);
        for index in 0..count {
            let triangle = e
                .call(MESH_GET_TRIANGLE, &args![mesh, index & 0xffff])
                .ptr::<NavMeshTriangle>();
            if fn_00693b90(e, triangle) {
                fn_00693bb0(e, triangle);
            }
        }
    }
    let map = nav_mesh_info_map(e);
    if map == 0 {
        return false;
    }
    let id = form_id(e, mesh);
    let map = nav_mesh_info_map(e);
    let info = fn_00693d70(e, Ptr::new(map), id);
    e.set(this, NavMesh::pNavMeshInfo, info);
    let info = e.get(this, NavMesh::pNavMeshInfo).addr();
    if info == 0 {
        let id = form_id(e, mesh);
        e.call(PRINT, &args![NO_INFO_FORMAT, id]);
        e.call(MESH_CREATE_TEMPORARY_INFO, &args![mesh]);
    } else {
        if e.call(INFO_GET_PARENT_SPACE, &args![info]).u32() == 0 {
            let space = e.call(MESH_GET_PARENT_SPACE, &args![mesh]).u32();
            let space_id = form_id(e, space);
            let info = e.get(this, NavMesh::pNavMeshInfo).addr();
            e.mem.set_u32(info + INFO_PARENT_SPACE_ID, space_id);
        }
        let info = e.get(this, NavMesh::pNavMeshInfo).addr();
        e.call(INFO_ATTACH_MESH, &args![info, mesh]);
    }
    if version < 10 {
        let count = array_size(e, triangles);
        let mut flagged = 0u32;
        for index in 0..count {
            let triangle = e
                .call(MESH_GET_TRIANGLE, &args![mesh, index & 0xffff])
                .ptr::<NavMeshTriangle>();
            if fn_00693b70(e, triangle) {
                flagged += 1;
            }
        }
        let info = e.get(this, NavMesh::pNavMeshInfo).addr();
        if info != 0 && count != 0 {
            // FILD/FILD/FDIVP at the default 53-bit precision, then FSTP.
            let ratio = (flagged as f64 / count as f64) as f32;
            fn_00693d50(e, Ptr::new(info), ratio);
        }
    }
    if version < 11 {
        let count = array_size(e, triangles);
        for index in 0..count {
            let triangle = e
                .call(MESH_GET_TRIANGLE, &args![mesh, index & 0xffff])
                .u32();
            if !fn_00693b70(e, Ptr::new(triangle))
                || !e.call(TRIANGLE_HAS_EDGE_INFO, &args![triangle]).bool()
            {
                continue;
            }
            for edge in 0..3i32 {
                if !e
                    .call(TRIANGLE_EDGE_HAS_INFO, &args![triangle, edge])
                    .bool()
                {
                    continue;
                }
                let slot = e.call(TRIANGLE_NEIGHBOUR, &args![triangle, edge]).u16();
                let edge_info = fn_00693d30(e, this, slot);
                if e.get(this, NavMesh::pNavMeshInfo).addr() == 0 {
                    continue;
                }
                let mut position = 0u32;
                loop {
                    let info = e.get(this, NavMesh::pNavMeshInfo).addr();
                    if position >= array_size(e, info + INFO_ARRAY_24) {
                        break;
                    }
                    let info = e.get(this, NavMesh::pNavMeshInfo).addr();
                    let item = e
                        .call(ELEMENT_4, &args![info + INFO_ARRAY_24, position])
                        .u32();
                    if word(e, item) == word(e, edge_info + 4) {
                        let info = e.get(this, NavMesh::pNavMeshInfo).addr();
                        e.call(ARRAY_REMOVE_4, &args![info + INFO_ARRAY_24, position, 1u32]);
                        let info = e.get(this, NavMesh::pNavMeshInfo).addr();
                        e.call(ARRAY_APPEND_4, &args![info + INFO_ARRAY_34, edge_info + 4]);
                        break;
                    }
                    position += 1;
                }
            }
        }
    }
    true
}

/// `NavMesh::Load`'s body after the scope object and the early returns,
/// with the old-door-portal array constructed.
fn load_record(e: &mut Engine, this: Ptr<NavMesh>, file: Ptr, frame: &LoadFrame) -> bool {
    let mesh = this.addr();
    e.call(FORM_LOAD_FORM, &args![mesh, file]);
    fn_00694de0(e, Ptr::new(frame.old_portals));
    e.mem.set_i32(frame.version, 1);
    let mut unresolved = false;
    if !flag_20(e, mesh) {
        loop {
            load_chunk(e, this, file, frame, &mut unresolved);
            if !e.call(FILE_NEXT_CHUNK, &args![file]).bool() {
                break;
            }
        }
    }
    if unresolved {
        with_detailed_string(e, mesh, |e, chars| {
            e.call(PRINT, &args![UNRESOLVED_FORMAT, chars]);
            e.call(MESH_CLEAR, &args![mesh, true]);
            e.set(this, NavMesh::pNavMeshInfo, Ptr::new(0));
            e.set(this, NavMesh::pParentCell, Ptr::new(0));
            e.call(FORM_SET_DELETE, &args![mesh, true]);
        });
    }
    let result = if flag_20(e, mesh) {
        e.call(REF_OBJECT_DELETED_CALL, &args![mesh + MESH_REF_OBJECT_BASE]);
        let id = form_id(e, mesh);
        let map = nav_mesh_info_map(e);
        e.call(INFO_MAP_REMOVE, &args![map, id]);
        false
    } else {
        if e.get(this, NavMesh::pNavMeshInfo).addr() != 0 {
            e.set(this, NavMesh::pNavMeshInfo, Ptr::new(0));
        }
        let version = e.mem.i32(frame.version);
        if !flag_20(e, mesh) && !load_upgrade(e, this, version, frame.old_portals) {
            fn_00694e10(e, Ptr::new(frame.old_portals));
            return false;
        }
        let check = e.call(0x0040_8d60, &args![LOAD_CHECK_OBJECT]).u32();
        let checked = e.mem.u8(check) != 0 || !e.call(FILE_GET_MASTER, &args![file]).bool();
        if checked && !flag_20(e, mesh) {
            e.call(MESH_CHECK, &args![mesh, false]);
        }
        true
    };
    fn_00694e10(e, Ptr::new(frame.old_portals));
    result
}

// Translated from 00692950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMesh::Load` (Xbox PDB): loads the mesh from the record `file` is on.
/// Returns false when the mesh has the flag 0x20 on entry (after `00401970`
/// on its `+0x1c`), when the record's form type is not `0x43`, when the
/// loaded mesh has the flag 0x20 (it is then removed from the
/// `NavMeshInfoMap`) or when there is no `NavMeshInfoMap`; true otherwise.
///
/// The chunks: `NVER` (version, default 1), `DATA` (counts and parent cell),
/// `NVVX` (vertices), `NVTR` (triangles, whose bits 13 to 15 are cleared),
/// `NVCA` (cover), `NVDP` (door portals), `NVGD` (grid), `NVEX` (extra edge
/// infos), `EDID` (editor id, virtual slot `+0x134`) and `OBND` (slot
/// `+0xe0`); data is byte-swapped when the file says so. A mesh with the
/// unresolved marker is reported, cleared and deleted. Then, by version:
/// below 5 the old door portals' extra edge infos are deleted; below 6 the
/// cover array is rebuilt; below 8 flag `0x10` of the triangles that have it
/// is toggled off; the mesh's `NavMeshInfo` is found (or a temporary one
/// made) and attached; below 10 the info's float at +0x20 becomes the share
/// of triangles with flag `0x40`; below 11 the portals of such triangles
/// are moved from the info's array at +0x24 to the one at +0x34. Finally the
/// mesh is checked (`NavMesh::CheckNavMesh(0)`) unless the file is a master
/// and the byte `00408d60` returns is zero.
// Translation incomplete: 00692950 - the exception-unwinding frame and the /GS cookie check (00ec408c) are not translated.
pub fn nav_mesh_load(e: &mut Engine, this: Ptr<NavMesh>, file: Ptr) -> bool {
    e.with_stack(0x30, |e, block| {
        let block = block.addr();
        let frame = LoadFrame {
            scope: block,
            old_portals: block + 0x04,
            version: block + 0x14,
            data: block + 0x18,
        };
        let mesh = this.addr();
        e.call(
            SCOPE_CONSTRUCT,
            &args![frame.scope, 0x2eu32, 1u32, SOURCE_FILE_NAME, 0x113u32],
        );
        let result = if flag_20(e, mesh) {
            e.call(REF_OBJECT_ENTRY_CALL, &args![mesh + MESH_REF_OBJECT_BASE]);
            false
        } else if e.call(FILE_GET_FORM_TYPE, &args![file]).u8() != NAV_MESH_FORM_TYPE {
            false
        } else {
            load_record(e, this, file, &frame)
        };
        e.call(SCOPE_DESTRUCT, &args![frame.scope]);
        result
    })
}

// Translated from 00693b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshTriangle` method: whether flag `0x40` is set.
pub fn fn_00693b70(e: &mut Engine, this: Ptr<NavMeshTriangle>) -> bool {
    e.get(this, NavMeshTriangle::TriangleFlags) & 0x40 != 0
}

// Translated from 00693b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshTriangle` method: whether flag `0x10` is set (`00691140`).
pub fn fn_00693b90(e: &mut Engine, this: Ptr<NavMeshTriangle>) -> bool {
    e.call(TRIANGLE_HAS_FLAGS, &args![this, 0x10u32]).bool()
}

// Translated from 00693bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshTriangle` method: toggles flag `0x10`.
pub fn fn_00693bb0(e: &mut Engine, this: Ptr<NavMeshTriangle>) {
    let flags = e.get(this, NavMeshTriangle::TriangleFlags);
    e.set(this, NavMeshTriangle::TriangleFlags, flags ^ 0x10);
}

// Translated from 00693bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshTriangle` method: stores `value`, clamped to `0..=7`
/// ([`fn_00694660`] on the argument's stack slot), in bits 13 to 15 of the
/// flags.
pub fn fn_00693bd0(e: &mut Engine, this: Ptr<NavMeshTriangle>, value: u16) {
    let value = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), value as u32);
        fn_00694660(e, slot, 0, 7);
        e.mem.u16(slot.addr())
    });
    let flags = e.get(this, NavMeshTriangle::TriangleFlags);
    e.set(
        this,
        NavMeshTriangle::TriangleFlags,
        flags & 0xffff_1fff | (value as u32) << 13,
    );
}

// Translated from 00693c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshTriangle` method: byte-swaps the flags and the six `u16` slots
/// (three vertices, three neighbours).
pub fn fn_00693c10(e: &mut Engine, this: Ptr<NavMeshTriangle>) {
    let triangle = this.addr();
    e.call(SWAP_U32, &args![triangle + TRIANGLE_FLAGS, 0u32]);
    for offset in [0u32, 2, 4, 6, 8, 10] {
        e.call(SWAP_U16, &args![triangle + offset, 0u32]);
    }
}

// Translated from 00693cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Size of an `NVEX` entry from version 9: [`fn_00693cf0`] on `this + 4`,
/// plus 4 (10).
pub fn fn_00693cd0(e: &mut Engine, this: Ptr) -> u32 {
    fn_00693cf0(e, Ptr::new(this.addr().wrapping_add(4))).wrapping_add(4)
}

// Translated from 00693cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns 6 (`4 + 2`); `this` is not read.
pub fn fn_00693cf0(_e: &mut Engine, _unused_0: Ptr) -> u32 {
    6
}

// Translated from 00693d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Size of an `NVEX` entry from version 5 to 8: [`fn_00693cf0`] on
/// `this + 4` (6; the result stays in `EAX`).
pub fn fn_00693d10(e: &mut Engine, this: Ptr) -> u32 {
    fn_00693cf0(e, Ptr::new(this.addr().wrapping_add(4)))
}

// Translated from 00693d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMesh` method: the address of `ExtraEdgeInfo[index]` (`006a1440`;
/// the result stays in `EAX`).
pub fn fn_00693d30(e: &mut Engine, this: Ptr<NavMesh>, index: u16) -> u32 {
    e.call(
        ELEMENT_12,
        &args![this.addr() + MESH_EXTRA_EDGE_INFO, index as u32],
    )
    .u32()
}

// Translated from 00693d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshInfo` method: stores `value` in the float at +0x20 (FLD/FSTP).
pub fn fn_00693d50(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_f32(this.addr() + INFO_FLOAT_20, x87_quiet(value));
}

// Translated from 00693d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshInfoMap` method: the `NavMeshInfo` stored under form id `id` in
/// the map at +0x1c (`00853130`), or null.
pub fn fn_00693d70(e: &mut Engine, this: Ptr, id: u32) -> Ptr {
    e.with_stack(4, |e, value| {
        let map = this.addr() + INFO_MAP_MAP;
        if e.call(MAP_GET_AT, &args![map, id, value]).bool() {
            Ptr::new(word(e, value.addr()))
        } else {
            Ptr::new(0)
        }
    })
}

// Translated from 00693db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Size of an `NVEX` entry before version 5: 0x2e.
pub fn fn_00693db0(_e: &mut Engine) -> u32 {
    0x2e
}

// Translated from 00693dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Byte-swaps the six words of a `DATA` chunk.
pub fn fn_00693dc0(e: &mut Engine, this: Ptr) {
    for offset in [0u32, 4, 8, 0xc, 0x10, 0x14] {
        e.call(SWAP_U32, &args![this.addr() + offset, 0u32]);
    }
}

// Translated from 00693e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMesh` method: with the flag 0x800 set, sets flag `0x10` of the
/// mesh's `NavMeshInfo` ([`fn_00693ef0`]). Then, with the flag 0x20 set,
/// clears the mesh (`NavMesh::ClearNavMesh(0)`) and calls the
/// `NavMeshInfoMap`'s virtual slot `+0xc8` with the byte `00460340` returned
/// for it beforehand; otherwise calls `00699b60`.
pub fn fn_00693e60(e: &mut Engine, this: Ptr<NavMesh>) {
    let mesh = this.addr();
    if e.call(FORM_FLAG_800, &args![mesh]).bool() {
        let info = e.call(MESH_GET_INFO, &args![mesh]).ptr();
        fn_00693ef0(e, info, true);
    }
    if flag_20(e, mesh) {
        let map = nav_mesh_info_map(e);
        let flag = e.call(0x0046_0340, &args![map]).u8();
        e.call(MESH_CLEAR, &args![mesh, false]);
        let map = nav_mesh_info_map(e);
        e.vcall(map, 0xc8, &args![flag]);
    } else {
        e.call(MESH_FLAG_20_CLEAR_CALL, &args![mesh]);
    }
}

// Translated from 00693ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`set`) or clears flag `0x10` of the word at +8 (a `NavMeshInfo`'s
/// flags here).
pub fn fn_00693ef0(e: &mut Engine, this: Ptr, set: bool) {
    let flags = word(e, this.addr() + INFO_FLAGS);
    let flags = if set {
        flags | 0x10
    } else {
        flags & 0xffff_ffef
    };
    e.mem.set_u32(this.addr() + INFO_FLAGS, flags);
}

// Translated from 00693f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMesh::GetFormDetailedString` (Xbox PDB): writes
/// `"NavMesh (%08X) with %d Triangles in Cell %s"` (form id, triangle
/// count, the parent cell's detailed string from its virtual slot `+0x90`,
/// empty without a cell) into the string at `result`.
// Translation incomplete: 00693f30 - the exception-unwinding frame is not translated.
pub fn nav_mesh_get_form_detailed_string(e: &mut Engine, this: Ptr<NavMesh>, result: Ptr) {
    let mesh = this.addr();
    e.with_stack(8, |e, text| {
        let text = text.addr();
        e.call(STRING_CONSTRUCT, &args![text]);
        let cell = e.call(GET_PARENT_CELL, &args![mesh]).u32();
        if cell != 0 {
            e.vcall(cell, 0x90, &args![text]);
        }
        let chars = e.call(DEREFERENCE, &args![text]).u32();
        let triangles = array_size(e, mesh + MESH_TRIANGLES);
        let id = form_id(e, mesh);
        e.call(
            STRING_PRINTF,
            &args![result, DETAILED_STRING_FORMAT, id, triangles, chars],
        );
        e.call(STRING_DESTRUCT, &args![text]);
    });
}

// Translated from 00693fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMesh` method: sets the flag 0x20 to `value`. When it changes:
/// `TESForm::SetDelete(value)`, then virtual slot `+0xc8` with 1, or with 0
/// when setting it on a mesh whose file is missing or not a master. When
/// `value` is set, the portals into the mesh are fixed
/// (`NavMesh::FixPortalsFromNavMeshDeletion` on its `NavMeshInfo`) and the
/// info is deleted from the `NavMeshInfoMap`.
pub fn fn_00693fe0(e: &mut Engine, this: Ptr<NavMesh>, value: u8) {
    let mesh = this.addr();
    let current = e.call(FORM_FLAG_20, &args![mesh]).u8();
    if value != current {
        let mut altered = 1u8;
        let source = e.call(FORM_GET_FILE, &args![mesh, 0u32]).u32();
        if value != 0 && (source == 0 || !e.call(FILE_GET_MASTER, &args![source]).bool()) {
            altered = 0;
        }
        e.call(FORM_SET_DELETE, &args![mesh, value]);
        e.vcall(mesh, 0xc8, &args![altered]);
    }
    if value != 0 {
        let info = e.call(MESH_GET_INFO, &args![mesh]).u32();
        e.call(MESH_FIX_PORTALS, &args![info]);
        let id = form_id(e, mesh);
        let map = nav_mesh_info_map(e);
        e.call(INFO_MAP_DELETE, &args![map, id]);
    }
}

// Translated from 00694090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMesh` method: `00484730(value)` on the form, then, when `value` is
/// set and the mesh has a parent cell, the cell's virtual slot `+0xc8`
/// with 1.
pub fn fn_00694090(e: &mut Engine, this: Ptr<NavMesh>, value: u8) {
    let mesh = this.addr();
    e.call(0x0048_4730, &args![mesh, value]);
    if value != 0 {
        let cell = word(e, mesh + MESH_PARENT_CELL);
        if cell != 0 {
            e.vcall(cell, 0xc8, &args![1u32]);
        }
    }
}

// Translated from 006940e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMesh` method on a file group record (`record[0]` must equal the word
/// at [`GROUP_RECORD_TAG`], group type at +0x0C, label at +8): type 6 needs
/// `accept`; types 6, 8 and 9 match when `00485be0` on the parent cell
/// accepts the label (true except for type 8); other types are passed to
/// the parent cell's virtual slot `+0x110` when `accept` is set. False
/// otherwise.
pub fn fn_006940e0(e: &mut Engine, this: Ptr<NavMesh>, record: Ptr, accept: u8, extra: u8) -> bool {
    let record = record.addr();
    if record == 0 || word(e, record) != e.global::<u32>(GROUP_RECORD_TAG) {
        return false;
    }
    let cell = e.call(GET_PARENT_CELL, &args![this]).u32();
    let kind = word(e, record + 0x0c);
    let check_label = if kind == 6 {
        if accept == 0 {
            return false;
        }
        true
    } else {
        kind > 7 && kind <= 9
    };
    if !check_label {
        if accept == 0 {
            return false;
        }
        return e.vcall(cell, 0x110, &args![record, accept, extra]).bool();
    }
    let label = word(e, record + 8);
    if e.call(0x0048_5be0, &args![cell, label]).bool() {
        word(e, record + 0x0c) != 8
    } else {
        false
    }
}

// Translated from 006941c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMesh` method: fills the 0x14-byte group record at `result` (cleared
/// first) for the group `record` belongs to: for a group of type 3 or 5
/// whose label is the parent cell's key (`00544210`) a type 6 group labelled
/// with the cell's form id; for a type 6 group of the cell a type 9 group
/// labelled likewise. Nothing for a null `result` or `record`.
pub fn fn_006941c0(e: &mut Engine, this: Ptr<NavMesh>, result: Ptr, record: Ptr) {
    let (result, record) = (result.addr(), record.addr());
    if result == 0 {
        return;
    }
    e.mem.set_u32(result, 0);
    if record == 0 {
        return;
    }
    let cell = e.call(GET_PARENT_CELL, &args![this]).u32();
    let cell_again = e.call(GET_PARENT_CELL, &args![this]).u32();
    let cell_id = form_id(e, cell_again);
    let kind = word(e, record + 0x0c);
    let fill = |e: &mut Engine, kind: u32, label: u32| {
        let tag: u32 = e.global(GROUP_RECORD_TAG);
        e.mem.set_u32(result, tag);
        e.mem.set_u32(result + 0x0c, kind);
        e.mem.set_u32(result + 8, label);
        e.mem.set_u32(result + 4, 0);
        e.mem.set_u32(result + 0x10, 0);
    };
    if kind == 3 || kind == 5 {
        let key = e.call(0x0054_4210, &args![cell]).u32();
        if word(e, record + 8) == key {
            fill(e, 6, cell_id);
        }
    } else if kind == 6 && word(e, record + 8) == cell_id {
        let tag: u32 = e.global(GROUP_RECORD_TAG);
        e.mem.set_u32(result, tag);
        e.mem.set_u32(result + 0x0c, 9);
        let cell = e.call(GET_PARENT_CELL, &args![this]).u32();
        let label = form_id(e, cell);
        e.mem.set_u32(result + 8, label);
        e.mem.set_u32(result + 4, 0);
        e.mem.set_u32(result + 0x10, 0);
    }
}

// Translated from 006942c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMesh` method: `TESForm::SetFormID(id, 0)`.
pub fn fn_006942c0(e: &mut Engine, this: Ptr<NavMesh>, id: u32) {
    e.call(FORM_SET_FORM_ID, &args![this, id, 0u32]);
}

// Translated from 006942e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMesh` method: copies this mesh into the mesh the `NiPointer` at
/// `target` holds: parent cell, vertices, triangles, extra edge infos,
/// cover, grid (`0069dd50`), obstacles, closed doors, every obstacle map
/// entry (into the target's map, through a `NiPointer` passed by value),
/// every point-of-view map entry, the closest and obstacle point-of-view
/// arrays; the target's `pNavMeshInfo` is cleared.
// Translation incomplete: 006942e0 - the exception-unwinding frame is not translated.
pub fn fn_006942e0(e: &mut Engine, this: Ptr<NavMesh>, target: Ptr) {
    let mesh = this.addr();
    let other = |e: &mut Engine| e.call(DEREFERENCE, &args![target]).u32();
    let target_mesh = other(e);
    let cell = word(e, mesh + MESH_PARENT_CELL);
    e.mem.set_u32(target_mesh + MESH_PARENT_CELL, cell);
    type ArrayCopy = fn(&mut Engine, Array, Array);
    let pairs: [(u32, ArrayCopy); 3] = [
        (MESH_VERTICES, fn_00694e30),
        (MESH_TRIANGLES, fn_00694ea0),
        (MESH_EXTRA_EDGE_INFO, fn_00694f20),
    ];
    for (offset, copy) in pairs {
        let target_mesh = other(e);
        copy(e, Ptr::new(mesh + offset), Ptr::new(target_mesh + offset));
    }
    let target_mesh = other(e);
    fn_00694f80(
        e,
        Ptr::new(mesh + MESH_COVER),
        Ptr::new(target_mesh + MESH_COVER),
    );
    let target_mesh = other(e);
    e.call(GRID_COPY, &args![mesh + MESH_GRID, target_mesh + MESH_GRID]);
    let target_mesh = other(e);
    fn_00694fe0(
        e,
        Ptr::new(mesh + MESH_OBSTACLES),
        Ptr::new(target_mesh + MESH_OBSTACLES),
    );
    let target_mesh = other(e);
    fn_00695040(
        e,
        Ptr::new(mesh + MESH_CLOSED_DOORS),
        Ptr::new(target_mesh + MESH_CLOSED_DOORS),
    );

    // Locals: the position (+0), the key (+4), the `NiPointer` (+8) and the
    // by-value copy (+0xc).
    e.with_stack(0x10, |e, locals| {
        let position = locals.addr();
        let key_slot = position + 4;
        let pointer = position + 8;
        let copy = position + 0x0c;
        let map = word(e, mesh + MESH_OBSTACLE_MAP);
        let first = e.call(MAP_GET_FIRST_POS, &args![map]).u32();
        e.mem.set_u32(position, first);
        while word(e, position) != 0 {
            e.call(POINTER_CONSTRUCT, &args![pointer, 0u32]);
            let map = word(e, mesh + MESH_OBSTACLE_MAP);
            fn_006947e0(
                e,
                Ptr::new(map),
                Ptr::new(position),
                Ptr::new(key_slot),
                Ptr::new(pointer),
            );
            e.call(POINTER_COPY, &args![copy, pointer]);
            let value = word(e, copy);
            let key = e.mem.u16(key_slot);
            let target_mesh = other(e);
            let target_map = word(e, target_mesh + MESH_OBSTACLE_MAP);
            fn_006946a0(e, Ptr::new(target_map), key, value);
            e.call(POINTER_RELEASE, &args![pointer]);
        }

        let povs = mesh + MESH_POVS;
        let first = e.call(MAP_GET_FIRST_POS, &args![povs]).u32();
        e.mem.set_u32(position, first);
        while word(e, position) != 0 {
            fn_00694d30(
                e,
                Ptr::new(povs),
                Ptr::new(position),
                Ptr::new(key_slot),
                Ptr::new(pointer),
            );
            let value = word(e, pointer);
            let key = e.mem.u16(key_slot);
            let target_mesh = other(e);
            fn_00694c50(e, Ptr::new(target_mesh + MESH_POVS), key, value);
        }
    });

    let target_mesh = other(e);
    fn_00694f80(
        e,
        Ptr::new(mesh + MESH_CLOSEST_POVS),
        Ptr::new(target_mesh + MESH_CLOSEST_POVS),
    );
    let target_mesh = other(e);
    fn_00694f80(
        e,
        Ptr::new(mesh + MESH_OBSTACLE_POVS),
        Ptr::new(target_mesh + MESH_OBSTACLE_POVS),
    );
    let target_mesh = other(e);
    e.mem.set_u32(target_mesh + MESH_INFO, 0);
}

// Translated from 00694500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMesh::SetDisableNavMesh` (Xbox PDB): finds the `NavMeshInfo` of the
/// mesh with form id `id` and its loaded mesh (reporting and returning false
/// when either is missing); when the mesh's flag 0x800 differs from
/// `disable`, calls `TESForm::SetDisabled(disable)`, sets the info's flag
/// `0x10` likewise and, when enabling, calls `0069aa20` on the mesh. True.
// Translation incomplete: 00694500 - the exception-unwinding frame is not translated.
pub fn nav_mesh_set_disable_nav_mesh(e: &mut Engine, id: u32, disable: u8) -> bool {
    let map = nav_mesh_info_map(e);
    let info = fn_00693d70(e, Ptr::new(map), id);
    if info.addr() == 0 {
        e.call(PRINT, &args![DISABLE_NO_INFO_FORMAT, id]);
        return false;
    }
    e.with_stack(4, |e, slot| {
        e.call(MESH_POINTER_CONSTRUCT, &args![slot]);
        if !e.call(INFO_GET_NAV_MESH, &args![info, slot]).bool() {
            e.call(PRINT, &args![DISABLE_NO_MESH_FORMAT, id]);
            e.call(MESH_POINTER_DESTRUCT, &args![slot]);
            return false;
        }
        let mesh = e.call(DEREFERENCE, &args![slot]).u32();
        let current = e.call(FORM_FLAG_800, &args![mesh]).u8();
        if current != disable {
            let mesh = e.call(DEREFERENCE, &args![slot]).u32();
            e.call(FORM_SET_DISABLED, &args![mesh, disable]);
            fn_00693ef0(e, info, disable != 0);
            if disable == 0 {
                let mesh = e.call(DEREFERENCE, &args![slot]).u32();
                e.call(MESH_ENABLED_CALL, &args![mesh]);
            }
        }
        e.call(MESH_POINTER_DESTRUCT, &args![slot]);
        true
    })
}

// Translated from 00694620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMesh` method: `004534f0(value)` on the form, then, with a
/// `NavMeshInfo`, sets its flag `0x10` to the mesh's flag 0x800.
pub fn fn_00694620(e: &mut Engine, this: Ptr<NavMesh>, value: u32) {
    let mesh = this.addr();
    e.call(0x0045_34f0, &args![mesh, value]);
    let info = e.get(this, NavMesh::pNavMeshInfo);
    if info.addr() != 0 {
        let flag = e.call(FORM_FLAG_800, &args![mesh]).bool();
        let info = e.get(this, NavMesh::pNavMeshInfo);
        fn_00693ef0(e, info, flag);
    }
}

// Translated from 00694660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clamps the `u16` at `value` to `minimum..=maximum` (the maximum is
/// tested first).
pub fn fn_00694660(e: &mut Engine, value: Ptr, minimum: u16, maximum: u16) {
    let current = e.mem.u16(value.addr());
    if current > maximum {
        e.mem.set_u16(value.addr(), maximum);
    } else if current < minimum {
        e.mem.set_u16(value.addr(), minimum);
    }
}

// Translated from 006946a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<..unsigned short, NiPointer<ObstacleData>>::SetAt` (Xbox
/// PDB). `value` is an `NiPointer` passed by value, which the function
/// destroys. The bucket of `key` is found through virtual slot `+0x04`, the
/// chain searched with slot `+0x08`; a matching item gets the value
/// assigned. Otherwise a new item (slot `+0x14`) gets the key and a copy of
/// the value (slot `+0x0c`, taking the copy by value), is linked at the head
/// of the bucket and counted.
// Translation incomplete: 006946a0 - the exception-unwinding frame is not translated.
pub fn ni_t_map_base_obstacle_data_set_at(e: &mut Engine, this: Map, key: u16, value: u32) {
    let map = this.addr();
    e.with_stack(4, |e, value_slot| {
        let value_slot = value_slot.addr();
        e.mem.set_u32(value_slot, value);
        let bucket = e.vcall(map, 0x04, &args![key as u32]).u32();
        let table = e.get(this, NiTPointerMap::m_ppkHashTable);
        let mut item = word(e, table.wrapping_add(bucket.wrapping_mul(4)));
        while item != 0 {
            let item_key = e.mem.u16(item + ITEM_KEY) as u32;
            if e.vcall(map, 0x08, &args![key as u32, item_key]).bool() {
                e.call(
                    POINTER_ASSIGN,
                    &args![item + OBSTACLE_ITEM_VALUE, value_slot],
                );
                e.call(POINTER_RELEASE, &args![value_slot]);
                return;
            }
            item = word(e, item);
        }
        let item = e.vcall(map, 0x14, &args![]).u32();
        e.with_stack(4, |e, copy| {
            e.call(POINTER_COPY, &args![copy, value_slot]);
            let copied = word(e, copy.addr());
            e.vcall(map, 0x0c, &args![item, key as u32, copied]);
        });
        map_link_item(e, this, bucket, item);
        e.call(POINTER_RELEASE, &args![value_slot]);
    });
}

/// [`ni_t_map_base_obstacle_data_set_at`] under its address name, for the
/// calls inside this unit.
fn fn_006946a0(e: &mut Engine, this: Map, key: u16, value: u32) {
    ni_t_map_base_obstacle_data_set_at(e, this, key, value);
}

// Translated from 006947e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<..unsigned short, NiPointer<ObstacleData>>::GetNext` (Xbox
/// PDB): stores the item at `*position`'s key at `key` and assigns its value
/// to the `NiPointer` at `value`, then moves `*position` to the next item.
pub fn ni_t_map_base_obstacle_data_get_next(
    e: &mut Engine,
    this: Map,
    position: Ptr,
    key: Ptr,
    value: Ptr,
) {
    let item = word(e, position.addr());
    let item_key = e.mem.u16(item + ITEM_KEY);
    e.mem.set_u16(key.addr(), item_key);
    e.call(POINTER_ASSIGN, &args![value, item + OBSTACLE_ITEM_VALUE]);
    map_advance(e, this, position, item);
}

/// [`ni_t_map_base_obstacle_data_get_next`] under its address name.
fn fn_006947e0(e: &mut Engine, this: Map, position: Ptr, key: Ptr, value: Ptr) {
    ni_t_map_base_obstacle_data_get_next(e, this, position, key, value);
}

// Translated from 00694890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<T, 1024>::SetSize` of the vertex array (12-byte elements;
/// reallocated by `006dc590`, constructed by `0069b9d0`, destroyed by
/// `0072ba80`); see [`simple_array_set_size`].
pub fn fn_00694890(e: &mut Engine, this: Array, count: u32, shrink: bool) {
    simple_array_set_size(
        e,
        this,
        count,
        shrink,
        &SetSizeCallees {
            element_size: 12,
            clear: ARRAY_CLEAR,
            reallocate: 0x006d_c590,
            construct: 0x0069_b9d0,
            destruct: 0x0072_ba80,
        },
    );
}

// Translated from 006949d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<T, 1024>::SetSize` of the triangle array (16-byte
/// elements; reallocated by `009f6540`, constructed by `00644490`,
/// destroyed by `0072ba80`); see [`simple_array_set_size`].
pub fn fn_006949d0(e: &mut Engine, this: Array, count: u32, shrink: bool) {
    simple_array_set_size(
        e,
        this,
        count,
        shrink,
        &SetSizeCallees {
            element_size: 16,
            clear: ARRAY_CLEAR,
            reallocate: 0x009f_6540,
            construct: 0x0064_4490,
            destruct: 0x0072_ba80,
        },
    );
}

// Translated from 00694b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<T, 1024>::SetSize` of the extra edge info array (12-byte
/// elements; reallocated by `006dc590`, constructed by `00692320`,
/// destroyed by `0072ba80`); see [`simple_array_set_size`].
pub fn fn_00694b10(e: &mut Engine, this: Array, count: u32, shrink: bool) {
    simple_array_set_size(
        e,
        this,
        count,
        shrink,
        &SetSizeCallees {
            element_size: 12,
            clear: ARRAY_CLEAR,
            reallocate: 0x006d_c590,
            construct: 0x0069_2320,
            destruct: 0x0072_ba80,
        },
    );
}

// Translated from 00694c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase::SetAt` of the point-of-view map (4-byte value at +6 of each
/// item): assigns `value` to the item with `key`, or adds a new item (slots
/// `+0x14` and `+0x0c`) at the head of the bucket and counts it.
pub fn fn_00694c50(e: &mut Engine, this: Map, key: u16, value: u32) {
    let map = this.addr();
    let bucket = e.vcall(map, 0x04, &args![key as u32]).u32();
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    let mut item = word(e, table.wrapping_add(bucket.wrapping_mul(4)));
    while item != 0 {
        let item_key = e.mem.u16(item + ITEM_KEY) as u32;
        if e.vcall(map, 0x08, &args![key as u32, item_key]).bool() {
            e.mem.set_u32(item + POV_ITEM_VALUE, value);
            return;
        }
        item = word(e, item);
    }
    let item = e.vcall(map, 0x14, &args![]).u32();
    e.vcall(map, 0x0c, &args![item, key as u32, value]);
    map_link_item(e, this, bucket, item);
}

// Translated from 00694d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase::GetNext` of the point-of-view map: stores the key and the
/// 4-byte value of the item at `*position` at `key` and `value`, then moves
/// `*position` to the next item.
pub fn fn_00694d30(e: &mut Engine, this: Map, position: Ptr, key: Ptr, value: Ptr) {
    let item = word(e, position.addr());
    let item_key = e.mem.u16(item + ITEM_KEY);
    e.mem.set_u16(key.addr(), item_key);
    let item_value = word(e, item + POV_ITEM_VALUE);
    e.mem.set_u32(value.addr(), item_value);
    map_advance(e, this, position, item);
}

// Translated from 00694de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<int, 1024>` constructor: the vtable, then the empty
/// initialisation `006b3eb0(0, 0)`. Returns `this`.
pub fn fn_00694de0(e: &mut Engine, this: Array) -> Array {
    e.mem.set_u32(this.addr(), INT_ARRAY_VTABLE);
    e.call(ARRAY_INIT, &args![this, 0u32, 0u32]);
    this
}

// Translated from 00694e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<int, 1024>` destructor: the vtable, then the clear that
/// frees the buffer (`008454f0(1)`).
pub fn fn_00694e10(e: &mut Engine, this: Array) {
    e.mem.set_u32(this.addr(), INT_ARRAY_VTABLE);
    e.call(ARRAY_CLEAR, &args![this, true]);
}

// Translated from 00694e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the vertex array into `destination` (12-byte elements, word by
/// word, through `006a1440`).
pub fn fn_00694e30(e: &mut Engine, this: Array, destination: Array) {
    copy_array(e, this, destination, fn_00694890, |e, index, buffer| {
        let source = buffer.wrapping_add(index.wrapping_mul(12));
        let target = e.call(ELEMENT_12, &args![destination, index]).u32();
        for offset in [0u32, 4, 8] {
            let value = word(e, source + offset);
            e.mem.set_u32(target + offset, value);
        }
    });
}

// Translated from 00694ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the triangle array into `destination` (16-byte elements, word by
/// word, through `00558dd0`).
pub fn fn_00694ea0(e: &mut Engine, this: Array, destination: Array) {
    copy_array(e, this, destination, fn_006949d0, |e, index, buffer| {
        let source = buffer.wrapping_add(index.wrapping_shl(4));
        let target = e.call(ELEMENT_16, &args![destination, index]).u32();
        for offset in [0u32, 4, 8, 0xc] {
            let value = word(e, source + offset);
            e.mem.set_u32(target + offset, value);
        }
    });
}

// Translated from 00694f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the extra edge info array into `destination` (12-byte elements,
/// each assigned by `00691c60`).
pub fn fn_00694f20(e: &mut Engine, this: Array, destination: Array) {
    copy_array(e, this, destination, fn_00694b10, |e, index, buffer| {
        let source = index.wrapping_mul(12).wrapping_add(buffer);
        let target = e.call(ELEMENT_12, &args![destination, index]).u32();
        e.call(0x0069_1c60, &args![target, source]);
    });
}

// Translated from 00694f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies a `BSSimpleArray<unsigned short>` into `destination` (through
/// `005eeda0`).
pub fn fn_00694f80(e: &mut Engine, this: Array, destination: Array) {
    copy_array(
        e,
        this,
        destination,
        bs_simple_array_unsigned_short_set_size,
        |e, index, buffer| {
            let target = e.call(ELEMENT_2, &args![destination, index]).u32();
            let value = e.mem.u16(buffer.wrapping_add(index.wrapping_mul(2)));
            e.mem.set_u16(target, value);
        },
    );
}

// Translated from 00694fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the obstacle array (`NiPointer`s) into `destination`, each
/// assigned through `006e5cc0`.
pub fn fn_00694fe0(e: &mut Engine, this: Array, destination: Array) {
    copy_array(e, this, destination, fn_00695230, |e, index, buffer| {
        let source = buffer.wrapping_add(index.wrapping_mul(4));
        let target = e.call(ELEMENT_4, &args![destination, index]).u32();
        e.call(POINTER_ASSIGN, &args![target, source]);
    });
}

// Translated from 00695040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies an 8-byte element array (the closed doors) into `destination`,
/// each assigned by `0069a690`.
pub fn fn_00695040(e: &mut Engine, this: Array, destination: Array) {
    copy_array(
        e,
        this,
        destination,
        bs_simple_array_nav_mesh_triangle_door_portal_set_size,
        |e, index, buffer| {
            let source = buffer.wrapping_add(index.wrapping_mul(8));
            let target = fn_006950d0(e, destination, index);
            e.call(0x0069_a690, &args![target, source]);
        },
    );
}

// Translated from 006950a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<int, 1024>::scalar deleting destructor` (Xbox PDB):
/// [`fn_00694e10`], then frees the object when bit 0 of `flags` is set.
/// Returns `this`.
pub fn bs_simple_array_int_scalar_deleting_destructor(
    e: &mut Engine,
    this: Array,
    flags: u32,
) -> Array {
    fn_00694e10(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 006950d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of element `index` of an 8-byte element array.
pub fn fn_006950d0(e: &mut Engine, this: Array, index: u32) -> u32 {
    e.get(this, BSSimpleArray::pBuffer)
        .wrapping_add(index.wrapping_mul(8))
}

// Translated from 006950f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NavMeshTriangleDoorPortal, 1024>::SetSize` (Xbox PDB):
/// 8-byte elements, reallocated by `006a2770`, constructed by `006927d0`,
/// destroyed by `0072ba80`; see [`simple_array_set_size`].
pub fn bs_simple_array_nav_mesh_triangle_door_portal_set_size(
    e: &mut Engine,
    this: Array,
    count: u32,
    shrink: bool,
) {
    simple_array_set_size(
        e,
        this,
        count,
        shrink,
        &SetSizeCallees {
            element_size: 8,
            clear: ARRAY_CLEAR,
            reallocate: 0x006a_2770,
            construct: 0x0069_27d0,
            destruct: 0x0072_ba80,
        },
    );
}

// Translated from 00695230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<T, 1024>::SetSize` of the obstacle array (`NiPointer`s,
/// 4 bytes; cleared by `006c6200`, reallocated by `0042fc00`, constructed by
/// `00822780`, destroyed by `00822820`); see [`simple_array_set_size`].
pub fn fn_00695230(e: &mut Engine, this: Array, count: u32, shrink: bool) {
    simple_array_set_size(
        e,
        this,
        count,
        shrink,
        &SetSizeCallees {
            element_size: 4,
            clear: 0x006c_6200,
            reallocate: 0x0042_fc00,
            construct: 0x0082_2780,
            destruct: 0x0082_2820,
        },
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x005eee80,
            bs_simple_array_unsigned_short_set_size(Array, u32, bool)
        ),
        entry!(0x00692950, nav_mesh_load(Ptr<NavMesh>, Ptr) -> bool),
        entry!(0x00693b70, fn_00693b70(Ptr<NavMeshTriangle>) -> bool),
        entry!(0x00693b90, fn_00693b90(Ptr<NavMeshTriangle>) -> bool),
        entry!(0x00693bb0, fn_00693bb0(Ptr<NavMeshTriangle>)),
        entry!(0x00693bd0, fn_00693bd0(Ptr<NavMeshTriangle>, u16)),
        entry!(0x00693c10, fn_00693c10(Ptr<NavMeshTriangle>)),
        entry!(0x00693cd0, fn_00693cd0(Ptr) -> u32),
        entry!(0x00693cf0, fn_00693cf0(Ptr) -> u32),
        entry!(0x00693d10, fn_00693d10(Ptr) -> u32),
        entry!(0x00693d30, fn_00693d30(Ptr<NavMesh>, u16) -> u32),
        entry!(0x00693d50, fn_00693d50(Ptr, f32)),
        entry!(0x00693d70, fn_00693d70(Ptr, u32) -> Ptr),
        entry!(0x00693db0, fn_00693db0() -> u32),
        entry!(0x00693dc0, fn_00693dc0(Ptr)),
        entry!(0x00693e60, fn_00693e60(Ptr<NavMesh>)),
        entry!(0x00693ef0, fn_00693ef0(Ptr, bool)),
        entry!(
            0x00693f30,
            nav_mesh_get_form_detailed_string(Ptr<NavMesh>, Ptr)
        ),
        entry!(0x00693fe0, fn_00693fe0(Ptr<NavMesh>, u8)),
        entry!(0x00694090, fn_00694090(Ptr<NavMesh>, u8)),
        entry!(0x006940e0, fn_006940e0(Ptr<NavMesh>, Ptr, u8, u8) -> bool),
        entry!(0x006941c0, fn_006941c0(Ptr<NavMesh>, Ptr, Ptr)),
        entry!(0x006942c0, fn_006942c0(Ptr<NavMesh>, u32)),
        entry!(0x006942e0, fn_006942e0(Ptr<NavMesh>, Ptr)),
        entry!(0x00694500, nav_mesh_set_disable_nav_mesh(u32, u8) -> bool),
        entry!(0x00694620, fn_00694620(Ptr<NavMesh>, u32)),
        entry!(0x00694660, fn_00694660(Ptr, u16, u16)),
        entry!(
            0x006946a0,
            ni_t_map_base_obstacle_data_set_at(Map, u16, u32)
        ),
        entry!(
            0x006947e0,
            ni_t_map_base_obstacle_data_get_next(Map, Ptr, Ptr, Ptr)
        ),
        entry!(0x00694890, fn_00694890(Array, u32, bool)),
        entry!(0x006949d0, fn_006949d0(Array, u32, bool)),
        entry!(0x00694b10, fn_00694b10(Array, u32, bool)),
        entry!(0x00694c50, fn_00694c50(Map, u16, u32)),
        entry!(0x00694d30, fn_00694d30(Map, Ptr, Ptr, Ptr)),
        entry!(0x00694de0, fn_00694de0(Array) -> Array),
        entry!(0x00694e10, fn_00694e10(Array)),
        entry!(0x00694e30, fn_00694e30(Array, Array)),
        entry!(0x00694ea0, fn_00694ea0(Array, Array)),
        entry!(0x00694f20, fn_00694f20(Array, Array)),
        entry!(0x00694f80, fn_00694f80(Array, Array)),
        entry!(0x00694fe0, fn_00694fe0(Array, Array)),
        entry!(0x00695040, fn_00695040(Array, Array)),
        entry!(0x006950a0, bs_simple_array_int_scalar_deleting_destructor(Array, u32) -> Array),
        entry!(0x006950d0, fn_006950d0(Array, u32) -> u32),
        entry!(
            0x006950f0,
            bs_simple_array_nav_mesh_triangle_door_portal_set_size(Array, u32, bool)
        ),
        entry!(0x00695230, fn_00695230(Array, u32, bool)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type CallLog = Vec<(u32, Vec<u32>)>;

    /// Vtable slot targets used by the doubles (not exe addresses).
    const ARRAY_VTABLE: u32 = 0x0100_0000;
    const ARRAY_ALLOCATE_SLOT: u32 = 0x7000_0004;
    const MAP_VTABLE: u32 = 0x0100_0100;
    const MAP_HASH_SLOT: u32 = 0x7000_0104;
    const MAP_EQUAL_SLOT: u32 = 0x7000_0108;
    const MAP_SET_ITEM_SLOT: u32 = 0x7000_010c;
    const MAP_NEW_ITEM_SLOT: u32 = 0x7000_0114;
    const MESH_VTABLE: u32 = 0x0100_0200;
    const MESH_DETAILED_STRING_SLOT: u32 = 0x7000_0290;
    const MESH_ALTERED_SLOT: u32 = 0x7000_02c8;
    const CELL_VTABLE: u32 = 0x0100_0400;
    const CELL_DETAILED_STRING_SLOT: u32 = 0x7000_0490;
    const CELL_ALTERED_SLOT: u32 = 0x7000_04c8;
    const CELL_GROUP_SLOT: u32 = 0x7000_0510;
    /// Test memory for globals the doubles use: the byte `00408d60` points
    /// at (+0), an info (+4), the `TES` value (+8), the info map (+0xc).
    const TEST_GLOBALS: u32 = 0x0120_0000;

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn noops(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, |_, _| Ret::default());
        }
    }

    fn logged(e: &mut Engine) {
        e.call_log = Some(vec![]);
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        let log: &CallLog = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// An array with a vtable whose slot +4 allocates `count * 16` bytes.
    fn array(e: &mut Engine) -> Array {
        e.put_vtable(ARRAY_VTABLE, &[0, ARRAY_ALLOCATE_SLOT]);
        e.register(ARRAY_ALLOCATE_SLOT, |e, a| ret(e.mem.alloc(a[1] * 16)));
        let array: Array = e.new_object();
        e.mem.set_u32(array.addr(), ARRAY_VTABLE);
        array
    }

    fn fill_array(e: &mut Engine, array: Array, data: &[u8], count: u32) {
        let buffer = e.mem.alloc(data.len().max(16) as u32 * 2);
        e.mem.write(buffer, data);
        e.set(array, BSSimpleArray::pBuffer, buffer);
        e.set(array, BSSimpleArray::iSize, count);
        e.set(array, BSSimpleArray::iReservedSize, count);
    }

    /// Element getters and the size getter reading the real layout.
    fn element_doubles(e: &mut Engine) {
        e.register(ARRAY_SIZE, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(ELEMENT_12, |e, a| ret(e.mem.u32(a[0] + 4) + a[1] * 12));
        e.register(ELEMENT_16, |e, a| ret(e.mem.u32(a[0] + 4) + a[1] * 16));
        e.register(ELEMENT_2, |e, a| ret(e.mem.u32(a[0] + 4) + a[1] * 2));
        e.register(ELEMENT_4, |e, a| ret(e.mem.u32(a[0] + 4) + a[1] * 4));
    }

    fn check_set_size(set_size: fn(&mut Engine, Array, u32, bool), callees: SetSizeCallees) {
        let mut e = Engine::new();
        noops(
            &mut e,
            &[
                callees.clear,
                callees.reallocate,
                callees.construct,
                callees.destruct,
            ],
        );
        let this = array(&mut e);
        let n = callees.element_size;
        logged(&mut e);

        // Zero clears.
        set_size(&mut e, this, 0, true);
        assert_eq!(calls_to(&e, callees.clear), vec![vec![this.addr(), 1]]);

        // Growing an empty array allocates exactly `count` elements.
        set_size(&mut e, this, 4, false);
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        assert_ne!(buffer, 0);
        assert_eq!(
            calls_to(&e, ARRAY_ALLOCATE_SLOT),
            vec![vec![this.addr(), 4]]
        );
        assert_eq!(
            calls_to(&e, callees.construct),
            vec![vec![this.addr(), buffer, 4]]
        );
        assert_eq!(e.get(this, BSSimpleArray::iReservedSize), 4);
        assert_eq!(e.get(this, BSSimpleArray::iSize), 4);

        // Growing past the capacity reallocates.
        set_size(&mut e, this, 9, false);
        assert_eq!(
            calls_to(&e, callees.reallocate),
            vec![vec![this.addr(), 9, 4]]
        );
        assert_eq!(
            calls_to(&e, callees.construct)[1],
            vec![this.addr(), buffer + 4 * n, 5]
        );
        assert_eq!(e.get(this, BSSimpleArray::iReservedSize), 9);

        // Shrinking without `shrink` only destroys.
        set_size(&mut e, this, 3, false);
        assert_eq!(
            calls_to(&e, callees.destruct),
            vec![vec![this.addr(), buffer + 3 * n, 6]]
        );
        assert_eq!(calls_to(&e, callees.reallocate).len(), 1);
        assert_eq!(e.get(this, BSSimpleArray::iSize), 3);

        // Growing within the capacity constructs.
        set_size(&mut e, this, 5, true);
        assert_eq!(
            calls_to(&e, callees.construct)[2],
            vec![this.addr(), buffer + 3 * n, 2]
        );

        // Shrinking to a quarter with `shrink` reallocates.
        set_size(&mut e, this, 2, true);
        assert_eq!(calls_to(&e, callees.reallocate)[1], vec![this.addr(), 2, 2]);
        assert_eq!(e.get(this, BSSimpleArray::iReservedSize), 2);
        assert_eq!(e.get(this, BSSimpleArray::iSize), 2);
    }

    #[test]
    fn unsigned_short_set_size() {
        check_set_size(
            bs_simple_array_unsigned_short_set_size,
            SetSizeCallees {
                element_size: 2,
                clear: ARRAY_CLEAR,
                reallocate: 0x005e_f6a0,
                construct: 0x005e_f650,
                destruct: 0x0072_ba80,
            },
        );
    }

    #[test]
    fn vertex_set_size() {
        check_set_size(
            fn_00694890,
            SetSizeCallees {
                element_size: 12,
                clear: ARRAY_CLEAR,
                reallocate: 0x006d_c590,
                construct: 0x0069_b9d0,
                destruct: 0x0072_ba80,
            },
        );
    }

    #[test]
    fn triangle_set_size() {
        check_set_size(
            fn_006949d0,
            SetSizeCallees {
                element_size: 16,
                clear: ARRAY_CLEAR,
                reallocate: 0x009f_6540,
                construct: 0x0064_4490,
                destruct: 0x0072_ba80,
            },
        );
    }

    #[test]
    fn extra_edge_info_set_size() {
        check_set_size(
            fn_00694b10,
            SetSizeCallees {
                element_size: 12,
                clear: ARRAY_CLEAR,
                reallocate: 0x006d_c590,
                construct: 0x0069_2320,
                destruct: 0x0072_ba80,
            },
        );
    }

    #[test]
    fn door_portal_set_size() {
        check_set_size(
            bs_simple_array_nav_mesh_triangle_door_portal_set_size,
            SetSizeCallees {
                element_size: 8,
                clear: ARRAY_CLEAR,
                reallocate: 0x006a_2770,
                construct: 0x0069_27d0,
                destruct: 0x0072_ba80,
            },
        );
    }

    #[test]
    fn obstacle_set_size() {
        check_set_size(
            fn_00695230,
            SetSizeCallees {
                element_size: 4,
                clear: 0x006c_6200,
                reallocate: 0x0042_fc00,
                construct: 0x0082_2780,
                destruct: 0x0082_2820,
            },
        );
    }

    // -- NavMesh::Load ------------------------------------------------------

    /// A record reader: chunk index at +0, chunk count at +4, chunk types
    /// from +8, chunk size at +0x80, the master flag at +0x90, the data of
    /// chunk `i` at `+0x100 + 0x40 * i`.
    fn file_with(e: &mut Engine, chunks: &[(u32, &[u8])]) -> Ptr {
        let file = e.mem.alloc(0x400);
        e.mem.set_u32(file + 4, chunks.len() as u32);
        for (i, (kind, data)) in chunks.iter().enumerate() {
            let i = i as u32;
            e.mem.set_u32(file + 8 + 4 * i, *kind);
            e.mem.write(file + 0x100 + 0x40 * i, data);
        }
        Ptr::new(file)
    }

    fn words(values: &[u32]) -> Vec<u8> {
        values.iter().flat_map(|v| v.to_le_bytes()).collect()
    }

    /// The flags at +8 of `form`.
    fn form_flags(e: &Engine, form: u32) -> u32 {
        e.mem.u32(form + 8)
    }

    /// The `TES` value, the info map and the doubles that reach them.
    fn tes_with_info_map(e: &mut Engine) -> u32 {
        e.map(TES_INSTANCE, 4);
        e.set_global(TES_INSTANCE, TEST_GLOBALS + 8);
        e.map(TEST_GLOBALS, 0x10);
        let info_map = e.mem.alloc(0x40);
        e.mem.set_u32(TEST_GLOBALS + 0xc, info_map);
        e.register(TES_GET_NAV_MESH_INFO_MAP, |e, a| {
            assert_eq!(a[0], TEST_GLOBALS + 8);
            ret(e.mem.u32(TEST_GLOBALS + 0xc))
        });
        info_map
    }

    struct LoadWorld {
        e: Engine,
        mesh: Ptr<NavMesh>,
        info: u32,
        info_map: u32,
    }

    fn load_world() -> LoadWorld {
        let mut e = Engine::new();
        element_doubles(&mut e);
        noops(
            &mut e,
            &[
                SCOPE_CONSTRUCT,
                SCOPE_DESTRUCT,
                REF_OBJECT_ENTRY_CALL,
                REF_OBJECT_DELETED_CALL,
                FORM_LOAD_FORM,
                ARRAY_INIT,
                ARRAY_CLEAR,
                FORM_ADD_COMPILE_INDEX,
                PRINT,
                STRING_CONSTRUCT,
                STRING_DESTRUCT,
                MESH_DETAILED_STRING_SLOT,
                MESH_CLEAR,
                MESH_REBUILD_COVER,
                MESH_CHECK,
                MESH_CREATE_TEMPORARY_INFO,
                INFO_ATTACH_MESH,
                INFO_MAP_REMOVE,
                ARRAY_REMOVE_4,
                ARRAY_APPEND_4,
                0x0069_b9d0,
                0x0064_4490,
                0x0069_2320,
                0x0069_27d0,
                0x005e_f650,
            ],
        );
        e.register(FORM_FLAG_20, |e, a| {
            ret((form_flags(e, a[0]) & 0x20 != 0) as u32)
        });
        e.register(FORM_SET_DELETE, |e, a| {
            let flags = form_flags(e, a[0]);
            e.mem.set_u32(a[0] + 8, flags | 0x20);
            Ret::default()
        });
        e.register(GET_FORM_ID, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        e.register(DEREFERENCE, |e, a| ret(e.mem.u32(a[0])));
        e.register(FILE_GET_FORM_TYPE, |_, _| ret(0x43));
        e.register(FILE_NEEDS_SWAP, |_, _| ret(0));
        e.register(FILE_CHUNK_SIZE, |e, a| ret(e.mem.u32(a[0] + 0x80)));
        e.register(FILE_GET_CHUNK, |e, a| {
            let index = e.mem.u32(a[0]);
            ret(e.mem.u32(a[0] + 8 + 4 * index))
        });
        e.register(FILE_NEXT_CHUNK, |e, a| {
            let index = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], index);
            ret((index < e.mem.u32(a[0] + 4)) as u32)
        });
        e.register(FILE_GET_CHUNK_DATA, |e, a| {
            let index = e.mem.u32(a[0]);
            let data = e.mem.bytes(a[0] + 0x100 + 0x40 * index, a[2].min(0x40));
            e.mem.write(a[1], &data);
            Ret::default()
        });
        e.register(FILE_GET_MASTER, |e, a| ret(e.mem.u32(a[0] + 0x90)));
        e.register(LOOKUP_FORM, |_, a| ret(a[0] + 0x10_0000));
        e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0]));
        e.register(0x0040_8d60, |_, _| ret(TEST_GLOBALS));

        // The mesh, its vtable and its arrays.
        e.put_vtable(MESH_VTABLE, &[0; 0x50]);
        e.mem.set_u32(MESH_VTABLE + 0x90, MESH_DETAILED_STRING_SLOT);
        let mesh: Ptr<NavMesh> = e.new_object();
        e.mem.set_u32(mesh.addr(), MESH_VTABLE);
        e.mem.set_u32(mesh.addr() + 0x0c, 0x0001_2345);
        e.put_vtable(ARRAY_VTABLE, &[0, ARRAY_ALLOCATE_SLOT]);
        e.register(ARRAY_ALLOCATE_SLOT, |e, a| ret(e.mem.alloc(a[1] * 16)));
        for offset in [
            MESH_VERTICES,
            MESH_TRIANGLES,
            MESH_EXTRA_EDGE_INFO,
            MESH_DOOR_PORTALS,
            MESH_COVER,
        ] {
            e.mem.set_u32(mesh.addr() + offset, ARRAY_VTABLE);
        }

        // The `NavMeshInfoMap` and the mesh's info (kept at +0x30 of the map).
        let info_map = tes_with_info_map(&mut e);
        let info = e.mem.alloc(0x60);
        e.mem.set_u32(info_map + 0x30, info);
        e.register(MAP_GET_AT, |e, a| {
            let info = e.mem.u32(a[0] - INFO_MAP_MAP + 0x30);
            e.mem.set_u32(a[2], info);
            ret(1)
        });
        e.register(INFO_GET_PARENT_SPACE, |_, _| ret(1));
        LoadWorld {
            e,
            mesh,
            info,
            info_map,
        }
    }

    #[test]
    fn load_reads_chunks() {
        let mut w = load_world();
        let vertices = words(&[1, 2, 3, 4, 5, 6]);
        let data = words(&[0x777, 2, 0, 0, 0, 0]);
        let version = words(&[11]);
        let file = file_with(
            &mut w.e,
            &[
                (CHUNK_NVER, &version),
                (CHUNK_DATA, &data),
                (CHUNK_NVVX, &vertices),
            ],
        );
        w.e.mem.set_u32(TEST_GLOBALS, 1);
        logged(&mut w.e);
        assert!(nav_mesh_load(&mut w.e, w.mesh, file));
        let e = &w.e;
        assert_eq!(
            e.get(w.mesh, NavMesh::pParentCell).addr(),
            0x777 + 0x10_0000
        );
        assert_eq!(e.get(w.mesh, NavMesh::pNavMeshInfo).addr(), w.info);
        let buffer = e.mem.u32(w.mesh.addr() + MESH_VERTICES + 4);
        assert_eq!(e.mem.bytes(buffer, 24), vertices);
        assert_eq!(e.mem.u32(w.mesh.addr() + MESH_VERTICES + 8), 2);
        let scope = calls_to(e, SCOPE_DESTRUCT)[0][0];
        assert_eq!(
            calls_to(e, SCOPE_CONSTRUCT),
            vec![vec![scope, 0x2e, 1, SOURCE_FILE_NAME, 0x113]]
        );
        assert_eq!(calls_to(e, MAP_GET_AT)[0][1], 0x0001_2345);
        assert_eq!(
            calls_to(e, INFO_ATTACH_MESH),
            vec![vec![w.info, w.mesh.addr()]]
        );
        assert_eq!(calls_to(e, MESH_CHECK), vec![vec![w.mesh.addr(), 0]]);
        // Four empty arrays cleared, then the old-portal array destroyed.
        assert_eq!(calls_to(e, ARRAY_CLEAR).len(), 5);
        assert!(calls_to(e, MESH_CREATE_TEMPORARY_INFO).is_empty());
    }

    #[test]
    fn load_rejects_unresolved_and_early_cases() {
        // The flag 0x20 on entry.
        let mut w = load_world();
        w.e.mem.set_u32(w.mesh.addr() + 8, 0x20);
        let file = file_with(&mut w.e, &[]);
        logged(&mut w.e);
        assert!(!nav_mesh_load(&mut w.e, w.mesh, file));
        assert_eq!(
            calls_to(&w.e, REF_OBJECT_ENTRY_CALL),
            vec![vec![w.mesh.addr() + 0x1c]]
        );
        assert!(calls_to(&w.e, FORM_LOAD_FORM).is_empty());

        // Another form type.
        let mut w = load_world();
        w.e.register(FILE_GET_FORM_TYPE, |_, _| ret(0x42));
        let file = file_with(&mut w.e, &[]);
        logged(&mut w.e);
        assert!(!nav_mesh_load(&mut w.e, w.mesh, file));
        assert!(calls_to(&w.e, FORM_LOAD_FORM).is_empty());
        assert_eq!(calls_to(&w.e, SCOPE_DESTRUCT).len(), 1);

        // The unresolved marker in the triangle count.
        let mut w = load_world();
        let data = words(&[0x777, 0, 0x1000_0000, 0, 0, 0]);
        let version = words(&[11]);
        let file = file_with(&mut w.e, &[(CHUNK_NVER, &version), (CHUNK_DATA, &data)]);
        logged(&mut w.e);
        assert!(!nav_mesh_load(&mut w.e, w.mesh, file));
        let e = &w.e;
        assert_eq!(calls_to(e, PRINT)[0][0], UNRESOLVED_FORMAT);
        assert_eq!(calls_to(e, MESH_CLEAR), vec![vec![w.mesh.addr(), 1]]);
        assert_eq!(e.get(w.mesh, NavMesh::pParentCell).addr(), 0);
        assert_eq!(
            calls_to(e, INFO_MAP_REMOVE),
            vec![vec![w.info_map, 0x0001_2345]]
        );
        assert_eq!(
            calls_to(e, REF_OBJECT_DELETED_CALL),
            vec![vec![w.mesh.addr() + 0x1c]]
        );
        assert!(calls_to(e, MESH_CHECK).is_empty());
    }

    #[test]
    fn load_upgrades_old_versions() {
        let mut w = load_world();
        let e = &mut w.e;
        // One triangle with flags 0x40 | 0x10 | edge 1 info, its edge 1 slot
        // pointing at extra edge info 0, whose portal word is 0x99.
        let triangle = [
            0u8, 0, 1, 0, 2, 0, 0xff, 0xff, 0, 0, 0xff, 0xff, 0x52, 0, 0, 0,
        ];
        let triangles = Ptr::new(w.mesh.addr() + MESH_TRIANGLES);
        fill_array(e, triangles, &triangle, 1);
        let edges = Ptr::new(w.mesh.addr() + MESH_EXTRA_EDGE_INFO);
        fill_array(e, edges, &words(&[0, 0x99, 0]), 1);
        e.register(MESH_GET_TRIANGLE, |e, a| {
            ret(e.mem.u32(a[0] + MESH_TRIANGLES + 4) + (a[1] & 0xffff) * 16)
        });
        e.register(TRIANGLE_HAS_FLAGS, |e, a| {
            ret((e.mem.u32(a[0] + 0xc) & a[1] != 0) as u32)
        });
        e.register(TRIANGLE_HAS_EDGE_INFO, |e, a| {
            ret((e.mem.u32(a[0] + 0xc) & 7 != 0) as u32)
        });
        e.register(TRIANGLE_EDGE_HAS_INFO, |e, a| {
            ret((e.mem.u32(a[0] + 0xc) >> a[1]) & 1)
        });
        e.register(TRIANGLE_NEIGHBOUR, |e, a| {
            ret(e.mem.u16(a[0] + 6 + 2 * a[1]) as u32)
        });
        let info_portals = Ptr::new(w.info + INFO_ARRAY_24);
        fill_array(e, info_portals, &words(&[0x55, 0x99]), 2);
        e.mem.set_u32(TEST_GLOBALS, 0);
        let file = file_with(e, &[(0x1234_5678, &[])]);
        e.mem.set_u32(file.addr() + 0x90, 1);
        logged(e);
        assert!(nav_mesh_load(e, w.mesh, file));

        let triangle = e.mem.u32(triangles.addr() + 4);
        // Flag 0x10 toggled off (version below 8).
        assert_eq!(e.mem.u32(triangle + 0xc), 0x42);
        // The share of 0x40 triangles (version below 10).
        assert_eq!(e.mem.f32(w.info + INFO_FLOAT_20), 1.0);
        // The portal moved from +0x24 to +0x34 (version below 11).
        assert_eq!(
            calls_to(e, ARRAY_REMOVE_4),
            vec![vec![w.info + INFO_ARRAY_24, 1, 1]]
        );
        let edge_info = e.mem.u32(edges.addr() + 4);
        assert_eq!(
            calls_to(e, ARRAY_APPEND_4),
            vec![vec![w.info + INFO_ARRAY_34, edge_info + 4]]
        );
        assert_eq!(calls_to(e, MESH_REBUILD_COVER), vec![vec![w.mesh.addr()]]);
        // A master file and a zero byte from `00408d60`: no check.
        assert!(calls_to(e, MESH_CHECK).is_empty());
    }

    // -- Triangle and small methods -----------------------------------------

    fn triangle(e: &mut Engine, flags: u32) -> Ptr<NavMeshTriangle> {
        let t: Ptr<NavMeshTriangle> = e.new_object();
        e.set(t, NavMeshTriangle::TriangleFlags, flags);
        t
    }

    #[test]
    fn triangle_flag_40() {
        let mut e = Engine::new();
        let t = triangle(&mut e, 0x40);
        assert!(fn_00693b70(&mut e, t));
        let t = triangle(&mut e, 0xbf);
        assert!(!fn_00693b70(&mut e, t));
    }

    #[test]
    fn triangle_flag_10() {
        let mut e = Engine::new();
        e.register(TRIANGLE_HAS_FLAGS, |_, a| ret((a[1] == 0x10) as u32));
        let t = triangle(&mut e, 0);
        logged(&mut e);
        assert!(fn_00693b90(&mut e, t));
        assert_eq!(calls_to(&e, TRIANGLE_HAS_FLAGS), vec![vec![t.addr(), 0x10]]);
    }

    #[test]
    fn triangle_toggle_flag_10() {
        let mut e = Engine::new();
        let t = triangle(&mut e, 0x41);
        fn_00693bb0(&mut e, t);
        assert_eq!(e.get(t, NavMeshTriangle::TriangleFlags), 0x51);
        fn_00693bb0(&mut e, t);
        assert_eq!(e.get(t, NavMeshTriangle::TriangleFlags), 0x41);
    }

    #[test]
    fn triangle_set_bits_13_to_15() {
        let mut e = Engine::new();
        let t = triangle(&mut e, 0xffff_ffff);
        fn_00693bd0(&mut e, t, 3);
        assert_eq!(e.get(t, NavMeshTriangle::TriangleFlags), 0xffff_7fff);
        fn_00693bd0(&mut e, t, 0x20);
        assert_eq!(e.get(t, NavMeshTriangle::TriangleFlags), 0xffff_ffff);
        fn_00693bd0(&mut e, t, 0);
        assert_eq!(e.get(t, NavMeshTriangle::TriangleFlags), 0xffff_1fff);
    }

    #[test]
    fn triangle_byte_swap() {
        let mut e = Engine::new();
        noops(&mut e, &[SWAP_U32, SWAP_U16]);
        let t = triangle(&mut e, 0);
        logged(&mut e);
        fn_00693c10(&mut e, t);
        assert_eq!(calls_to(&e, SWAP_U32), vec![vec![t.addr() + 0xc, 0]]);
        let swapped: Vec<u32> = calls_to(&e, SWAP_U16).iter().map(|a| a[0]).collect();
        let expected: Vec<u32> = (0..6).map(|i| t.addr() + 2 * i).collect();
        assert_eq!(swapped, expected);
    }

    #[test]
    fn entry_size_from_version_9() {
        let mut e = Engine::new();
        assert_eq!(fn_00693cd0(&mut e, Ptr::new(0x1000)), 10);
    }

    #[test]
    fn six() {
        let mut e = Engine::new();
        assert_eq!(fn_00693cf0(&mut e, Ptr::new(0)), 6);
    }

    #[test]
    fn entry_size_from_version_5() {
        let mut e = Engine::new();
        assert_eq!(fn_00693d10(&mut e, Ptr::new(0x1000)), 6);
    }

    #[test]
    fn extra_edge_info_address() {
        let mut e = Engine::new();
        e.register(ELEMENT_12, |_, a| ret(a[0] + a[1]));
        let mesh: Ptr<NavMesh> = e.new_object();
        assert_eq!(
            fn_00693d30(&mut e, mesh, 0xfffe),
            mesh.addr() + 0x48 + 0xfffe
        );
    }

    #[test]
    fn info_float_store_quiets() {
        let mut e = Engine::new();
        let info = Ptr::new(e.mem.alloc(0x40));
        fn_00693d50(&mut e, info, 0.25);
        assert_eq!(e.mem.f32(info.addr() + 0x20), 0.25);
        fn_00693d50(&mut e, info, f32::from_bits(0x7f80_0001));
        assert_eq!(e.mem.u32(info.addr() + 0x20), 0x7fc0_0001);
    }

    #[test]
    fn info_map_lookup() {
        let mut e = Engine::new();
        e.register(MAP_GET_AT, |e, a| {
            if a[1] == 7 {
                e.mem.set_u32(a[2], 0x4444);
                ret(1)
            } else {
                ret(0x1234_5600)
            }
        });
        let map = Ptr::new(e.mem.alloc(0x40));
        logged(&mut e);
        assert_eq!(fn_00693d70(&mut e, map, 7).addr(), 0x4444);
        assert_eq!(fn_00693d70(&mut e, map, 8).addr(), 0);
        assert_eq!(calls_to(&e, MAP_GET_AT)[0][0], map.addr() + 0x1c);
    }

    #[test]
    fn old_entry_size() {
        let mut e = Engine::new();
        assert_eq!(fn_00693db0(&mut e), 0x2e);
    }

    #[test]
    fn data_chunk_byte_swap() {
        let mut e = Engine::new();
        noops(&mut e, &[SWAP_U32]);
        logged(&mut e);
        fn_00693dc0(&mut e, Ptr::new(0x1000));
        let swapped: Vec<u32> = calls_to(&e, SWAP_U32).iter().map(|a| a[0]).collect();
        assert_eq!(
            swapped,
            vec![0x1000, 0x1004, 0x1008, 0x100c, 0x1010, 0x1014]
        );
    }

    fn mesh_world() -> (Engine, Ptr<NavMesh>, u32) {
        let mut e = Engine::new();
        e.register(FORM_FLAG_20, |e, a| {
            ret((form_flags(e, a[0]) & 0x20 != 0) as u32)
        });
        e.register(FORM_FLAG_800, |e, a| {
            ret((form_flags(e, a[0]) & 0x800 != 0) as u32)
        });
        e.register(GET_FORM_ID, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        e.register(MESH_GET_INFO, |e, a| ret(e.mem.u32(a[0] + 0x104)));
        e.register(GET_PARENT_CELL, |e, a| ret(e.mem.u32(a[0] + 0x24)));
        e.register(DEREFERENCE, |e, a| ret(e.mem.u32(a[0])));
        let info_map = tes_with_info_map(&mut e);
        e.put_vtable(MESH_VTABLE, &[0; 0x50]);
        e.mem.set_u32(MESH_VTABLE + 0xc8, MESH_ALTERED_SLOT);
        e.mem.set_u32(MESH_VTABLE + 0x90, MESH_DETAILED_STRING_SLOT);
        e.put_vtable(CELL_VTABLE, &[0; 0x50]);
        e.mem.set_u32(CELL_VTABLE + 0x90, CELL_DETAILED_STRING_SLOT);
        e.mem.set_u32(CELL_VTABLE + 0xc8, CELL_ALTERED_SLOT);
        e.mem.set_u32(CELL_VTABLE + 0x110, CELL_GROUP_SLOT);
        let mesh: Ptr<NavMesh> = e.new_object();
        e.mem.set_u32(mesh.addr(), MESH_VTABLE);
        e.mem.set_u32(mesh.addr() + 0xc, 0x0005_0505);
        (e, mesh, info_map)
    }

    #[test]
    fn flag_800_and_20_follow_up() {
        let (mut e, mesh, info_map) = mesh_world();
        noops(
            &mut e,
            &[MESH_CLEAR, MESH_FLAG_20_CLEAR_CALL, MESH_ALTERED_SLOT],
        );
        e.register(0x0046_0340, |_, _| ret(0x1234_5601));
        e.mem.set_u32(info_map, MESH_VTABLE);
        let info = e.mem.alloc(0x60);
        e.mem.set_u32(mesh.addr() + 0x104, info);
        e.mem.set_u32(mesh.addr() + 8, 0x820);
        logged(&mut e);
        fn_00693e60(&mut e, mesh);
        assert_eq!(e.mem.u32(info + 8), 0x10);
        assert_eq!(calls_to(&e, MESH_CLEAR), vec![vec![mesh.addr(), 0]]);
        assert_eq!(calls_to(&e, MESH_ALTERED_SLOT), vec![vec![info_map, 1]]);

        e.mem.set_u32(mesh.addr() + 8, 0);
        e.mem.set_u32(info + 8, 0);
        fn_00693e60(&mut e, mesh);
        assert_eq!(e.mem.u32(info + 8), 0);
        assert_eq!(
            calls_to(&e, MESH_FLAG_20_CLEAR_CALL),
            vec![vec![mesh.addr()]]
        );
    }

    #[test]
    fn info_flag_10() {
        let mut e = Engine::new();
        let info = Ptr::new(e.mem.alloc(0x40));
        e.mem.set_u32(info.addr() + 8, 0x101);
        fn_00693ef0(&mut e, info, true);
        assert_eq!(e.mem.u32(info.addr() + 8), 0x111);
        fn_00693ef0(&mut e, info, false);
        assert_eq!(e.mem.u32(info.addr() + 8), 0x101);
    }

    #[test]
    fn detailed_string() {
        let (mut e, mesh, _) = mesh_world();
        noops(
            &mut e,
            &[
                STRING_CONSTRUCT,
                STRING_DESTRUCT,
                STRING_PRINTF,
                CELL_DETAILED_STRING_SLOT,
            ],
        );
        e.register(ARRAY_SIZE, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.mem.set_u32(mesh.addr() + 0x38 + 8, 12);
        logged(&mut e);
        nav_mesh_get_form_detailed_string(&mut e, mesh, Ptr::new(0x5000));
        let printed = calls_to(&e, STRING_PRINTF);
        assert_eq!(
            printed[0][..4],
            [0x5000, DETAILED_STRING_FORMAT, 0x0005_0505, 12]
        );
        assert!(calls_to(&e, CELL_DETAILED_STRING_SLOT).is_empty());

        let cell = e.mem.alloc(0x40);
        e.mem.set_u32(cell, CELL_VTABLE);
        e.mem.set_u32(mesh.addr() + 0x24, cell);
        nav_mesh_get_form_detailed_string(&mut e, mesh, Ptr::new(0x5000));
        let text = calls_to(&e, STRING_CONSTRUCT)[1][0];
        assert_eq!(
            calls_to(&e, CELL_DETAILED_STRING_SLOT),
            vec![vec![cell, text]]
        );
    }

    #[test]
    fn set_flag_20() {
        let (mut e, mesh, info_map) = mesh_world();
        noops(
            &mut e,
            &[
                FORM_SET_DELETE,
                MESH_ALTERED_SLOT,
                MESH_FIX_PORTALS,
                INFO_MAP_DELETE,
            ],
        );
        e.register(FORM_GET_FILE, |_, _| ret(0x3000));
        e.register(FILE_GET_MASTER, |_, _| ret(0));
        e.mem.set_u32(mesh.addr() + 0x104, 0x6000);
        logged(&mut e);
        // Setting it on a mesh from a non-master file: altered with 0.
        fn_00693fe0(&mut e, mesh, 1);
        assert_eq!(calls_to(&e, FORM_SET_DELETE), vec![vec![mesh.addr(), 1]]);
        assert_eq!(calls_to(&e, MESH_ALTERED_SLOT), vec![vec![mesh.addr(), 0]]);
        assert_eq!(calls_to(&e, MESH_FIX_PORTALS), vec![vec![0x6000]]);
        assert_eq!(
            calls_to(&e, INFO_MAP_DELETE),
            vec![vec![info_map, 0x0005_0505]]
        );
        // Clearing it when set: altered with 1, no portal fix.
        e.mem.set_u32(mesh.addr() + 8, 0x20);
        fn_00693fe0(&mut e, mesh, 0);
        assert_eq!(calls_to(&e, MESH_ALTERED_SLOT)[1], vec![mesh.addr(), 1]);
        assert_eq!(calls_to(&e, MESH_FIX_PORTALS).len(), 1);
        // No change and not set: nothing.
        e.mem.set_u32(mesh.addr() + 8, 0);
        fn_00693fe0(&mut e, mesh, 0);
        assert_eq!(calls_to(&e, FORM_SET_DELETE).len(), 2);
    }

    #[test]
    fn set_value_and_notify_cell() {
        let (mut e, mesh, _) = mesh_world();
        noops(&mut e, &[0x0048_4730, CELL_ALTERED_SLOT]);
        logged(&mut e);
        fn_00694090(&mut e, mesh, 1);
        assert!(calls_to(&e, CELL_ALTERED_SLOT).is_empty());
        let cell = e.mem.alloc(0x40);
        e.mem.set_u32(cell, CELL_VTABLE);
        e.mem.set_u32(mesh.addr() + 0x24, cell);
        fn_00694090(&mut e, mesh, 0);
        assert!(calls_to(&e, CELL_ALTERED_SLOT).is_empty());
        fn_00694090(&mut e, mesh, 1);
        assert_eq!(calls_to(&e, CELL_ALTERED_SLOT), vec![vec![cell, 1]]);
        assert_eq!(calls_to(&e, 0x0048_4730).len(), 3);
    }

    fn group(e: &mut Engine, tag: u32, label: u32, kind: u32) -> Ptr {
        let record = e.mem.alloc(0x14);
        e.mem.set_u32(record, tag);
        e.mem.set_u32(record + 8, label);
        e.mem.set_u32(record + 0x0c, kind);
        Ptr::new(record)
    }

    #[test]
    fn group_matching() {
        let (mut e, mesh, _) = mesh_world();
        let tag = 0x5055_5247u32;
        e.map(GROUP_RECORD_TAG, 4);
        e.set_global(GROUP_RECORD_TAG, tag);
        let cell = e.mem.alloc(0x40);
        e.mem.set_u32(cell, CELL_VTABLE);
        e.mem.set_u32(mesh.addr() + 0x24, cell);
        e.register(0x0048_5be0, |_, a| ret((a[1] == 0x77) as u32));
        e.register(CELL_GROUP_SLOT, |_, _| ret(1));
        let r = group(&mut e, 0x1111, 0x77, 8);
        assert!(!fn_006940e0(&mut e, mesh, r, 1, 0));
        assert!(!fn_006940e0(&mut e, mesh, Ptr::new(0), 1, 0));
        let r = group(&mut e, tag, 0x77, 6);
        assert!(!fn_006940e0(&mut e, mesh, r, 0, 0));
        assert!(fn_006940e0(&mut e, mesh, r, 1, 0));
        let r = group(&mut e, tag, 0x77, 8);
        assert!(!fn_006940e0(&mut e, mesh, r, 0, 0));
        let r = group(&mut e, tag, 0x77, 9);
        assert!(fn_006940e0(&mut e, mesh, r, 0, 0));
        let r = group(&mut e, tag, 0x78, 9);
        assert!(!fn_006940e0(&mut e, mesh, r, 0, 0));
        let r = group(&mut e, tag, 0x78, 2);
        assert!(!fn_006940e0(&mut e, mesh, r, 0, 5));
        logged(&mut e);
        assert!(fn_006940e0(&mut e, mesh, r, 3, 5));
        assert_eq!(
            calls_to(&e, CELL_GROUP_SLOT),
            vec![vec![cell, r.addr(), 3, 5]]
        );
    }

    #[test]
    fn group_for_record() {
        let (mut e, mesh, _) = mesh_world();
        e.map(GROUP_RECORD_TAG, 4);
        e.set_global(GROUP_RECORD_TAG, 0x5055_5247u32);
        let cell = e.mem.alloc(0x40);
        e.mem.set_u32(cell + 0xc, 0x0000_3c3c);
        e.mem.set_u32(mesh.addr() + 0x24, cell);
        e.register(0x0054_4210, |_, _| ret(0x21));
        let result = e.mem.alloc(0x14);
        e.mem.set_u32(result, 0xdead);
        fn_006941c0(&mut e, mesh, Ptr::new(result), Ptr::new(0));
        assert_eq!(e.mem.u32(result), 0);

        let r = group(&mut e, 0, 0x21, 5);
        fn_006941c0(&mut e, mesh, Ptr::new(result), r);
        assert_eq!(e.mem.u32(result), 0x5055_5247);
        assert_eq!(e.mem.u32(result + 8), 0x3c3c);
        assert_eq!(e.mem.u32(result + 0x0c), 6);

        let r = group(&mut e, 0, 0x3c3c, 6);
        fn_006941c0(&mut e, mesh, Ptr::new(result), r);
        assert_eq!(e.mem.u32(result + 0x0c), 9);

        let r = group(&mut e, 0, 0x22, 3);
        e.mem.set_u32(result + 0x0c, 0x55);
        fn_006941c0(&mut e, mesh, Ptr::new(result), r);
        assert_eq!(e.mem.u32(result), 0);
        assert_eq!(e.mem.u32(result + 0x0c), 0x55);
    }

    #[test]
    fn set_form_id() {
        let mut e = Engine::new();
        noops(&mut e, &[FORM_SET_FORM_ID]);
        logged(&mut e);
        fn_006942c0(&mut e, Ptr::new(0x3000), 0x99);
        assert_eq!(calls_to(&e, FORM_SET_FORM_ID), vec![vec![0x3000, 0x99, 0]]);
    }

    #[test]
    fn copy_into_other_mesh() {
        let mut e = Engine::new();
        element_doubles(&mut e);
        noops(
            &mut e,
            &[
                GRID_COPY,
                POINTER_CONSTRUCT,
                POINTER_COPY,
                POINTER_RELEASE,
                0x0069_b9d0,
                ARRAY_CLEAR,
                0x006c_6200,
            ],
        );
        e.register(DEREFERENCE, |e, a| ret(e.mem.u32(a[0])));
        e.register(MAP_GET_FIRST_POS, |_, _| ret(0));
        let source: Ptr<NavMesh> = e.new_object();
        let target: Ptr<NavMesh> = e.new_object();
        e.mem.set_u32(source.addr() + 0x24, 0x4242);
        e.mem.set_u32(target.addr() + 0x104, 0x9999);
        let vertices = Ptr::new(source.addr() + MESH_VERTICES);
        fill_array(&mut e, vertices, &words(&[1, 2, 3]), 1);
        let target_vertices: Array = Ptr::new(target.addr() + MESH_VERTICES);
        let buffer = e.mem.alloc(0x40);
        e.set(target_vertices, BSSimpleArray::pBuffer, buffer);
        e.set(target_vertices, BSSimpleArray::iReservedSize, 4);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, target.addr());
        logged(&mut e);
        fn_006942e0(&mut e, source, Ptr::new(slot));
        assert_eq!(e.mem.u32(target.addr() + 0x24), 0x4242);
        assert_eq!(e.mem.u32(target.addr() + 0x104), 0);
        assert_eq!(e.mem.bytes(buffer, 12), words(&[1, 2, 3]));
        assert_eq!(e.get(target_vertices, BSSimpleArray::iSize), 1);
        assert_eq!(
            calls_to(&e, GRID_COPY),
            vec![vec![source.addr() + 0xa8, target.addr() + 0xa8]]
        );
        assert_eq!(
            calls_to(&e, MAP_GET_FIRST_POS),
            vec![vec![0], vec![source.addr() + 0x88]]
        );
    }

    #[test]
    fn set_disable_nav_mesh() {
        let (mut e, mesh, _) = mesh_world();
        noops(
            &mut e,
            &[
                PRINT,
                MESH_POINTER_CONSTRUCT,
                MESH_POINTER_DESTRUCT,
                MESH_ENABLED_CALL,
            ],
        );
        e.register(MAP_GET_AT, |e, a| {
            if a[1] == 0x55 {
                let info = e.mem.u32(TEST_GLOBALS + 4);
                e.mem.set_u32(a[2], info);
                ret(1)
            } else {
                ret(0)
            }
        });
        e.register(FORM_SET_DISABLED, |e, a| {
            let flags = e.mem.u32(a[0] + 8) & !0x800;
            e.mem.set_u32(a[0] + 8, flags | (a[1] & 1) << 11);
            Ret::default()
        });
        e.register(INFO_GET_NAV_MESH, |e, a| {
            let mesh = e.mem.u32(a[0] + 0x54);
            e.mem.set_u32(a[1], mesh);
            ret((mesh != 0) as u32)
        });
        let info = e.mem.alloc(0x60);
        e.mem.set_u32(TEST_GLOBALS + 4, info);
        logged(&mut e);
        // No info.
        assert!(!nav_mesh_set_disable_nav_mesh(&mut e, 0x56, 1));
        assert_eq!(
            calls_to(&e, PRINT),
            vec![vec![DISABLE_NO_INFO_FORMAT, 0x56]]
        );
        // No loaded mesh.
        assert!(!nav_mesh_set_disable_nav_mesh(&mut e, 0x55, 1));
        assert_eq!(calls_to(&e, PRINT)[1], vec![DISABLE_NO_MESH_FORMAT, 0x55]);
        assert_eq!(calls_to(&e, MESH_POINTER_DESTRUCT).len(), 1);
        // Disable, then enable.
        e.mem.set_u32(info + 0x54, mesh.addr());
        assert!(nav_mesh_set_disable_nav_mesh(&mut e, 0x55, 1));
        assert_eq!(e.mem.u32(mesh.addr() + 8), 0x800);
        assert_eq!(e.mem.u32(info + 8), 0x10);
        assert!(calls_to(&e, MESH_ENABLED_CALL).is_empty());
        assert!(nav_mesh_set_disable_nav_mesh(&mut e, 0x55, 0));
        assert_eq!(e.mem.u32(info + 8), 0);
        assert_eq!(calls_to(&e, MESH_ENABLED_CALL), vec![vec![mesh.addr()]]);
        // Unchanged: no call.
        assert!(nav_mesh_set_disable_nav_mesh(&mut e, 0x55, 0));
        assert_eq!(calls_to(&e, FORM_SET_DISABLED).len(), 2);
    }

    #[test]
    fn copy_flag_800_to_info() {
        let (mut e, mesh, _) = mesh_world();
        noops(&mut e, &[0x0045_34f0]);
        logged(&mut e);
        fn_00694620(&mut e, mesh, 7);
        assert_eq!(calls_to(&e, 0x0045_34f0), vec![vec![mesh.addr(), 7]]);
        assert!(calls_to(&e, FORM_FLAG_800).is_empty());
        let info = e.mem.alloc(0x60);
        e.mem.set_u32(mesh.addr() + 0x104, info);
        e.mem.set_u32(mesh.addr() + 8, 0x800);
        fn_00694620(&mut e, mesh, 7);
        assert_eq!(e.mem.u32(info + 8), 0x10);
    }

    #[test]
    fn clamp_u16() {
        let mut e = Engine::new();
        let value = Ptr::new(e.mem.alloc(4));
        for (start, expected) in [(9u16, 7u16), (7, 7), (2, 2), (0, 1), (0xffff, 7)] {
            e.mem.set_u16(value.addr(), start);
            fn_00694660(&mut e, value, 1, 7);
            assert_eq!(e.mem.u16(value.addr()), expected);
        }
    }

    /// A map with `buckets` buckets whose slots hash by `key % buckets`,
    /// compare keys and make 12-byte items.
    fn map(e: &mut Engine, buckets: u32) -> Map {
        e.put_vtable(
            MAP_VTABLE,
            &[
                0,
                MAP_HASH_SLOT,
                MAP_EQUAL_SLOT,
                MAP_SET_ITEM_SLOT,
                0,
                MAP_NEW_ITEM_SLOT,
            ],
        );
        e.register(MAP_HASH_SLOT, |e, a| ret(a[1] % e.mem.u32(a[0] + 4)));
        e.register(MAP_EQUAL_SLOT, |_, a| ret((a[1] == a[2]) as u32));
        e.register(MAP_NEW_ITEM_SLOT, |e, _| ret(e.mem.alloc(12)));
        let map: Map = e.new_object();
        e.mem.set_u32(map.addr(), MAP_VTABLE);
        e.set(map, NiTPointerMap::m_uiHashSize, buckets);
        let table = e.mem.alloc(4 * buckets);
        e.set(map, NiTPointerMap::m_ppkHashTable, table);
        map
    }

    fn bucket_head(e: &Engine, map: Map, bucket: u32) -> u32 {
        e.mem
            .u32(e.get(map, NiTPointerMap::m_ppkHashTable) + 4 * bucket)
    }

    fn copy_word(e: &mut Engine, a: &[u32]) -> Ret {
        let value = e.mem.u32(a[1]);
        e.mem.set_u32(a[0], value);
        Ret::default()
    }

    #[test]
    fn obstacle_map_set_at() {
        let mut e = Engine::new();
        let map = map(&mut e, 5);
        e.register(MAP_SET_ITEM_SLOT, |e, a| {
            e.mem.set_u16(a[1] + 4, a[2] as u16);
            e.mem.set_u32(a[1] + 8, a[3]);
            Ret::default()
        });
        e.register(POINTER_COPY, copy_word);
        e.register(POINTER_ASSIGN, copy_word);
        noops(&mut e, &[POINTER_RELEASE]);
        logged(&mut e);
        ni_t_map_base_obstacle_data_set_at(&mut e, map, 7, 0xaaaa);
        let item = bucket_head(&e, map, 2);
        assert_eq!(e.mem.u16(item + 4), 7);
        assert_eq!(e.mem.u32(item + 8), 0xaaaa);
        assert_eq!(e.get(map, NiTPointerMap::m_uiCount), 1);
        assert_eq!(calls_to(&e, MAP_SET_ITEM_SLOT)[0][1..], [item, 7, 0xaaaa]);
        // The same key: assigned, not added; the by-value pointer released.
        ni_t_map_base_obstacle_data_set_at(&mut e, map, 7, 0xbbbb);
        assert_eq!(e.mem.u32(item + 8), 0xbbbb);
        assert_eq!(e.get(map, NiTPointerMap::m_uiCount), 1);
        assert_eq!(calls_to(&e, POINTER_RELEASE).len(), 2);
        // Another key in the same bucket goes to the head.
        ni_t_map_base_obstacle_data_set_at(&mut e, map, 12, 0xcccc);
        let head = bucket_head(&e, map, 2);
        assert_eq!(e.mem.u32(head), item);
        assert_eq!(e.get(map, NiTPointerMap::m_uiCount), 2);
    }

    #[test]
    fn obstacle_map_get_next() {
        let mut e = Engine::new();
        let map = map(&mut e, 4);
        e.register(POINTER_ASSIGN, copy_word);
        let table = e.get(map, NiTPointerMap::m_ppkHashTable);
        let first = e.mem.alloc(12);
        let second = e.mem.alloc(12);
        let third = e.mem.alloc(12);
        e.mem.set_u32(first, second);
        e.mem.set_u16(first + 4, 1);
        e.mem.set_u32(first + 8, 0x11);
        e.mem.set_u16(second + 4, 5);
        e.mem.set_u32(second + 8, 0x22);
        e.mem.set_u16(third + 4, 3);
        e.mem.set_u32(third + 8, 0x33);
        e.mem.set_u32(table + 4, first);
        e.mem.set_u32(table + 12, third);
        let slots = e.mem.alloc(12);
        let (position, key, value) = (Ptr::new(slots), Ptr::new(slots + 4), Ptr::new(slots + 8));
        e.mem.set_u32(slots, first);
        let mut seen = vec![];
        while e.mem.u32(slots) != 0 {
            ni_t_map_base_obstacle_data_get_next(&mut e, map, position, key, value);
            seen.push((e.mem.u16(slots + 4), e.mem.u32(slots + 8)));
        }
        assert_eq!(seen, vec![(1, 0x11), (5, 0x22), (3, 0x33)]);
    }

    #[test]
    fn pov_map_set_at() {
        let mut e = Engine::new();
        let map = map(&mut e, 3);
        e.register(MAP_SET_ITEM_SLOT, |e, a| {
            e.mem.set_u16(a[1] + 4, a[2] as u16);
            e.mem.set_u32(a[1] + 6, a[3]);
            Ret::default()
        });
        fn_00694c50(&mut e, map, 4, 0x1234_5678);
        let item = bucket_head(&e, map, 1);
        assert_eq!(e.mem.u32(item + 6), 0x1234_5678);
        fn_00694c50(&mut e, map, 4, 0x0bad_f00d);
        assert_eq!(e.mem.u32(item + 6), 0x0bad_f00d);
        assert_eq!(e.get(map, NiTPointerMap::m_uiCount), 1);
    }

    #[test]
    fn pov_map_get_next() {
        let mut e = Engine::new();
        let map = map(&mut e, 3);
        let table = e.get(map, NiTPointerMap::m_ppkHashTable);
        let item = e.mem.alloc(12);
        e.mem.set_u16(item + 4, 9);
        e.mem.set_u32(item + 6, 0xfeed_beef);
        e.mem.set_u32(table, item);
        let slots = e.mem.alloc(12);
        e.mem.set_u32(slots, item);
        fn_00694d30(
            &mut e,
            map,
            Ptr::new(slots),
            Ptr::new(slots + 4),
            Ptr::new(slots + 8),
        );
        assert_eq!(e.mem.u16(slots + 4), 9);
        assert_eq!(e.mem.u32(slots + 8), 0xfeed_beef);
        assert_eq!(e.mem.u32(slots), 0);
    }

    #[test]
    fn int_array_constructor() {
        let mut e = Engine::new();
        noops(&mut e, &[ARRAY_INIT]);
        let array: Array = e.new_object();
        logged(&mut e);
        assert_eq!(fn_00694de0(&mut e, array), array);
        assert_eq!(e.mem.u32(array.addr()), INT_ARRAY_VTABLE);
        assert_eq!(calls_to(&e, ARRAY_INIT), vec![vec![array.addr(), 0, 0]]);
    }

    #[test]
    fn int_array_destructor() {
        let mut e = Engine::new();
        noops(&mut e, &[ARRAY_CLEAR]);
        let array: Array = e.new_object();
        logged(&mut e);
        fn_00694e10(&mut e, array);
        assert_eq!(e.mem.u32(array.addr()), INT_ARRAY_VTABLE);
        assert_eq!(calls_to(&e, ARRAY_CLEAR), vec![vec![array.addr(), 1]]);
    }

    #[test]
    fn int_array_scalar_deleting_destructor() {
        let mut e = Engine::new();
        noops(&mut e, &[ARRAY_CLEAR, FREE]);
        let array: Array = e.new_object();
        logged(&mut e);
        bs_simple_array_int_scalar_deleting_destructor(&mut e, array, 0);
        assert!(calls_to(&e, FREE).is_empty());
        assert_eq!(
            bs_simple_array_int_scalar_deleting_destructor(&mut e, array, 3),
            array
        );
        assert_eq!(calls_to(&e, FREE), vec![vec![array.addr()]]);
        assert_eq!(calls_to(&e, ARRAY_CLEAR).len(), 2);
    }

    /// A source array with `data` and an empty destination with room.
    fn copy_pair(e: &mut Engine, data: &[u8], count: u32) -> (Array, Array) {
        element_doubles(e);
        let source: Array = e.new_object();
        fill_array(e, source, data, count);
        let destination: Array = e.new_object();
        let buffer = e.mem.alloc(0x100);
        e.set(destination, BSSimpleArray::pBuffer, buffer);
        e.set(destination, BSSimpleArray::iReservedSize, 8);
        (source, destination)
    }

    fn buffers(e: &Engine, source: Array, destination: Array) -> (u32, u32) {
        (
            e.get(source, BSSimpleArray::pBuffer),
            e.get(destination, BSSimpleArray::pBuffer),
        )
    }

    #[test]
    fn copy_vertices() {
        let mut e = Engine::new();
        noops(&mut e, &[0x0069_b9d0]);
        let data = words(&[1, 2, 3, 4, 5, 6]);
        let (source, destination) = copy_pair(&mut e, &data, 2);
        fn_00694e30(&mut e, source, destination);
        let (_, dst) = buffers(&e, source, destination);
        assert_eq!(e.get(destination, BSSimpleArray::iSize), 2);
        assert_eq!(e.mem.bytes(dst, 24), data);
    }

    #[test]
    fn copy_triangles() {
        let mut e = Engine::new();
        noops(&mut e, &[0x0064_4490]);
        let data = words(&[1, 2, 3, 4, 5, 6, 7, 8]);
        let (source, destination) = copy_pair(&mut e, &data, 2);
        fn_00694ea0(&mut e, source, destination);
        let (_, dst) = buffers(&e, source, destination);
        assert_eq!(e.mem.bytes(dst, 32), data);
    }

    #[test]
    fn copy_extra_edge_infos() {
        let mut e = Engine::new();
        noops(&mut e, &[0x0069_2320, 0x0069_1c60]);
        let (source, destination) = copy_pair(&mut e, &[0; 24], 2);
        logged(&mut e);
        fn_00694f20(&mut e, source, destination);
        let (src, dst) = buffers(&e, source, destination);
        assert_eq!(
            calls_to(&e, 0x0069_1c60),
            vec![vec![dst, src], vec![dst + 12, src + 12]]
        );
    }

    #[test]
    fn copy_unsigned_shorts() {
        let mut e = Engine::new();
        noops(&mut e, &[0x005e_f650]);
        let data = [1u8, 2, 3, 4, 5, 6];
        let (source, destination) = copy_pair(&mut e, &data, 3);
        fn_00694f80(&mut e, source, destination);
        let (_, dst) = buffers(&e, source, destination);
        assert_eq!(e.mem.bytes(dst, 6), data);
    }

    #[test]
    fn copy_obstacles() {
        let mut e = Engine::new();
        noops(&mut e, &[0x0082_2780, POINTER_ASSIGN]);
        let (source, destination) = copy_pair(&mut e, &words(&[7, 8]), 2);
        logged(&mut e);
        fn_00694fe0(&mut e, source, destination);
        let (src, dst) = buffers(&e, source, destination);
        assert_eq!(
            calls_to(&e, POINTER_ASSIGN),
            vec![vec![dst, src], vec![dst + 4, src + 4]]
        );
    }

    #[test]
    fn copy_closed_doors() {
        let mut e = Engine::new();
        noops(&mut e, &[0x0069_27d0, 0x0069_a690]);
        let (source, destination) = copy_pair(&mut e, &[0; 16], 2);
        logged(&mut e);
        fn_00695040(&mut e, source, destination);
        let (src, dst) = buffers(&e, source, destination);
        assert_eq!(
            calls_to(&e, 0x0069_a690),
            vec![vec![dst, src], vec![dst + 8, src + 8]]
        );
    }

    #[test]
    fn eight_byte_element() {
        let mut e = Engine::new();
        let array: Array = e.new_object();
        e.set(array, BSSimpleArray::pBuffer, 0x1000);
        assert_eq!(fn_006950d0(&mut e, array, 3), 0x1018);
    }
}
