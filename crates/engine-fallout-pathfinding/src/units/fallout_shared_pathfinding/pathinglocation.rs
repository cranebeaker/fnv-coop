//! `fallout shared/pathfinding/pathinglocation.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A `PathingLocation` (0x28 bytes, vtable `0102583c` with `SaveGame` and
//! `LoadGame`) is a point in the world plus what the pathing code knows
//! about where it lies: the navmesh info that holds it (`pNavMesh`, +0x10),
//! or, while that is not decided, the list of candidate navmesh infos
//! (`pNavMeshes`, +0x14), the cell or worldspace with the cell key, the
//! triangle of the navmesh and a flag byte whose bit 0 says the navmesh info
//! was chosen by position (set by `path_location_set_chosen_by_position`).
//! Bit 1 of that byte is tested by `006a9520` (another unit's code), which
//! makes the resolving functions give up.
//!
//! Layout: the fields `Location`, `pCell`, `pWorldSpace`, `usTriangle` and
//! `uiFlags` are in the shared [`PathingLocation`] layout of `pathfind.rs`;
//! the three fields it does not declare are read here at their offsets
//! ([`NAV_MESH`], [`NAV_MESHES`], [`CELL_KEY`]).
//!
//! `NavMeshInfo` (another unit) is read at its offsets: `+0x00` id, `+0x08`
//! flags (bit `0x10` is tested by `0068f320`), `+0x0C` cell key, `+0x24`
//! adjacent navmesh infos, `+0x34` preferred adjacent navmesh infos (both
//! `BSSimpleArray`s of `NavMeshInfo*`), `+0x54` the loaded `NavMesh*`
//! (`0069dfb0` tests it), `+0x58` the `NavMeshBounds*` (`006286d0` returns
//! it; the engine map calls that body `LowProcess::GetNumberOfItemsActivated`
//! because the linker folded identical code).
//!
//! How the translations read the game code:
//! - A `NiPointer<NavMesh>` on the game's stack is a four byte block built
//!   by `0042fb00` and released by `0042fa40`; `00559450` (the pointer's
//!   `operator*`, which for a `NavMeshInfo*` is its first word, the id) gives
//!   the mesh.
//! - `006a7ad0` / `00877a30` index a `BSSimpleArray` of pointers (they give
//!   the address of the element); `0044ddc0` is its element count.
//! - The compiler's exception-unwinding frames are not translated, nor the
//!   stack cookie of `PrintDebugText`.
//! - Pushed arguments the decompiler attached to the wrong call were re-read
//!   from the disassembly: the arguments written before `00559450` (which
//!   takes none) belong to the call after it, and the stack words pushed
//!   before the virtual call of `PathingLocation(refr)` belong to
//!   `SetupData` (the slot takes none).

#[allow(unused_imports)]
use crate::prelude::*;

use super::pathfind::PathingLocation;

// ---------------------------------------------------------------------------
// Layout and constants
// ---------------------------------------------------------------------------

/// `pNavMesh` (Xbox PDB), `const NavMeshInfo*`, at +0x10 (not in the shared
/// layout).
const NAV_MESH: u32 = 0x10;
/// `pNavMeshes` (Xbox PDB), `const BSSimpleArray<NavMeshInfo const *,1024>*`,
/// at +0x14.
const NAV_MESHES: u32 = 0x14;
/// `pCell` (Xbox PDB) and `pWorldSpace` (Xbox PDB), as raw offsets for the
/// word helpers.
const CELL: u32 = 0x18;
const WORLD_SPACE: u32 = 0x1c;
/// `iCellKey` (Xbox PDB), at +0x20: the key of the cell inside the
/// worldspace, [`NO_CELL_KEY`] when there is no worldspace.
const CELL_KEY: u32 = 0x20;
/// Offset of the position (`Location`, a `NiPoint3`) inside a
/// `PathingLocation`.
const LOCATION: u32 = 0x04;
/// `usTriangle` and `uiFlags`, as raw offsets (the save and load buffers
/// take their addresses).
const TRIANGLE: u32 = 0x24;
const FLAGS: u32 = 0x26;
/// The `PathingLocation` vtable (`SaveGame` at +0, `LoadGame` at +4).
const VTABLE: u32 = 0x0102_583c;
/// "No cell" in `iCellKey`.
const NO_CELL_KEY: u32 = 0xdead_dead;
/// "No triangle" in `usTriangle`.
const NO_TRIANGLE: u16 = 0xffff;
/// Flag bit 0 of `uiFlags`: the navmesh info was chosen by position.
const FLAG_CHOSEN_BY_POSITION: u32 = 0x01;
/// "Not found" from the array search, and the index of a candidate that was
/// not found in the closest-candidate scans.
const NOT_FOUND: u32 = u32::MAX;

/// `float` `FLT_MAX`, loaded from the exe's data.
const FLOAT_MAX: u32 = 0x0101_6970;
/// `0.0` (`double`).
const ZERO: u32 = 0x0101_2060;
/// `16384.0` (`double`): the nearest-bounds distance (squared) from which
/// `ResolveNavMeshInfo` falls back to a navmesh without bounds.
const FAR_DISTANCE: u32 = 0x0106_cd60;
/// `25.0` (`double`): how far above the ground height a position may be and
/// still count as on the ground.
const GROUND_MARGIN: u32 = 0x0104_f2f0;
/// `180.0` (`double`): the largest height above a triangle's centre that a
/// position may have to be accepted (unless it is on the ground).
const MAX_HEIGHT_ABOVE: u32 = 0x0104_ed58;
/// `-64.0` (`double`): the lowest height relative to a triangle's centre that
/// a position may have to be accepted.
const MIN_HEIGHT_BELOW: u32 = 0x0106_b9f0;
/// `0.1f`: the weight of the triangle centre when a position is pulled in
/// from an edge.
const CENTER_WEIGHT: u32 = 0x0101_e2bc;
/// `0.9f`: the weight of the edge crossing point in that move.
const EDGE_WEIGHT: u32 = 0x0101_77e8;

/// `TES` singleton pointer and the player character pointer.
const TES_SINGLETON: u32 = 0x011d_ea10;
const PLAYER_CHARACTER: u32 = 0x011d_ea3c;

// Callees outside this file.
/// `NiPoint3`'s constructor (`ECX` is returned, nothing is written).
const NI_POINT3_CONSTRUCT: u32 = 0x0068_15c0;
/// `TESObjectREFR` helpers: the parent cell (`008d6f30`) and
/// `GetWorldSpace` (`00575d70`).
const REFERENCE_CELL: u32 = 0x008d_6f30;
const REFERENCE_WORLD_SPACE: u32 = 0x0057_5d70;
/// `TES` helpers: `005f36f0` returns the interior cell (the map's name for it,
/// `ActorMover::GetPreferredMoveMode`, is a folded-code mistake), `004fd3e0`
/// is `TES::GetWorldSpace`.
const TES_INTERIOR_CELL: u32 = 0x005f_36f0;
const TES_WORLD_SPACE: u32 = 0x004f_d3e0;
/// `BGSWorldLocation::GetCellOrWorld(&cell, &worldspace)`.
const WORLD_LOCATION_CELL_OR_WORLD: u32 = 0x0052_7990;
/// `TESWorldSpace::GetKeyForWorldCoord(position)` (cdecl): the cell key of a
/// position.
const WORLD_SPACE_KEY_FOR_COORD: u32 = 0x0058_7440;
/// `TESWorldSpace::GetCellFromKey(key)`.
const WORLD_SPACE_CELL_FROM_KEY: u32 = 0x0058_7630;
/// `TESWorldSpace::GetCellFromWorldCoord(position)`.
const WORLD_SPACE_CELL_FROM_COORD: u32 = 0x0058_7550;
/// `Pathing::GetPotentialNavMeshInfoForLocation(cell)` (cdecl) and the
/// `_ov2(worldspace, key)` version.
const POTENTIAL_NAV_MESHES_FOR_CELL: u32 = 0x006d_6f40;
const POTENTIAL_NAV_MESHES_FOR_KEY: u32 = 0x006d_6f60;
/// `TESObjectCELL` accessors: `00425fd0` (tested before the worldspace is
/// asked for), `GetWorldSpace`, `GetDataX`, `GetDataY`; and `00587410(x, y)`
/// (cdecl), which builds the cell key from the grid coordinates.
const CELL_TEST: u32 = 0x0042_5fd0;
const CELL_WORLD_SPACE: u32 = 0x0054_ddd0;
const CELL_DATA_X: u32 = 0x0054_4c30;
const CELL_DATA_Y: u32 = 0x0054_4c60;
const CELL_KEY_FROM_GRID: u32 = 0x0058_7410;
/// Navmesh info helpers: the world space (`00690800`) and the cell
/// (`006b77b0`) the info belongs to.
const INFO_WORLD_SPACE: u32 = 0x0069_0800;
const INFO_CELL: u32 = 0x006b_77b0;
/// `NavMeshInfo` helpers: `0068f320` (flag `0x10`), `0069ad00(out)` (loads
/// the navmesh into a `NiPointer`, true on success), `0069dfb0` (a navmesh is
/// loaded), `006286d0` (the bounds pointer).
const INFO_IS_DISABLED: u32 = 0x0068_f320;
const INFO_LOAD_NAV_MESH: u32 = 0x0069_ad00;
const INFO_HAS_NAV_MESH: u32 = 0x0069_dfb0;
const INFO_BOUNDS: u32 = 0x0062_86d0;
/// `NavMeshBounds` helpers: `0069bea0(position)` (the bounds contain the
/// position) and `0069c070(out, position)` (the offset from the position to
/// the bounds).
const BOUNDS_CONTAIN: u32 = 0x0069_bea0;
const BOUNDS_OFFSET_TO: u32 = 0x0069_c070;
/// `NavMesh` helpers: `00696a50(position, arg)` (the triangle holding the
/// position, 0xffff if none), `00696cf0(position, &triangle, &exact)` (the
/// `FindClosestTriangleForLocation` of the Xbox PDB), `00558220(out,
/// triangle)` (`GetCenter`), `00698160(out, triangle, -1, centre,
/// position)` (`FindInterectingEdge2`), `00440d80`/`00440da0` (tests of the
/// mesh's form flags), `00450ff0` (a cell test).
const NAV_MESH_FIND_TRIANGLE: u32 = 0x0069_6a50;
const NAV_MESH_FIND_CLOSEST_TRIANGLE: u32 = 0x0069_6cf0;
const NAV_MESH_GET_CENTER: u32 = 0x0055_8220;
const NAV_MESH_FIND_INTERSECTING_EDGE: u32 = 0x0069_8160;
const FORM_TEST_FIRST: u32 = 0x0044_0d80;
const FORM_TEST_SECOND: u32 = 0x0044_0da0;
const CELL_IS_KIND: u32 = 0x0045_0ff0;
/// `NiPointer<NavMesh>`: construct, release, `operator*`, and the test that
/// it holds a mesh (`00458b50`, which calls `operator*`).
const NI_POINTER_CONSTRUCT: u32 = 0x0042_fb00;
const NI_POINTER_RELEASE: u32 = 0x0042_fa40;
const NI_POINTER_GET: u32 = 0x0055_9450;
const NI_POINTER_HAS_MESH: u32 = 0x0045_8b50;
/// `TES` height query: `0045cbc0(position, cell)` leaves the ground height
/// in `ST0`.
const TES_GROUND_HEIGHT: u32 = 0x0045_cbc0;
/// `NiPoint3` helpers (`this` is the left operand): `00439ef0(out, rhs)` is
/// `this - rhs`, `00439e90(out, rhs)` is `this + rhs`, `0045bb20(out,
/// scale)` is `this * scale`; `004a7290` is the squared length (`ST0`),
/// `00595c80` the squared length of the x and y parts (`ST0`).
const POINT_SUBTRACT: u32 = 0x0043_9ef0;
const POINT_ADD: u32 = 0x0043_9e90;
const POINT_SCALE: u32 = 0x0045_bb20;
const POINT_SQUARED_LENGTH: u32 = 0x004a_7290;
const POINT_SQUARED_LENGTH_2D: u32 = 0x0059_5c80;
/// `_ftol2_sse` (`ST0` as a `double` argument).
const FTOL: u32 = 0x00ec_62c0;
/// `BSSimpleArray`s: count (`0044ddc0`), address of the pointer at an index
/// (`006a7ad0`, `00877a30`), add a value by address (`007cb2e0`), copy into
/// another array (`006e5330`), remove `count` items at an index
/// (`009a4320`), empty the array (`008454f0(array, free_storage)`), find
/// with a comparison function (`00719b20(array, &value, start,
/// compare)`) and construct/destroy a plain array (`005e0510`,
/// `005e0540`).
const ARRAY_COUNT: u32 = 0x0044_ddc0;
const ARRAY_ELEMENT: u32 = 0x006a_7ad0;
const LOCAL_ARRAY_ELEMENT: u32 = 0x0087_7a30;
const ARRAY_ADD: u32 = 0x007c_b2e0;
const ARRAY_COPY_TO: u32 = 0x006e_5330;
const ARRAY_REMOVE: u32 = 0x009a_4320;
const ARRAY_EMPTY: u32 = 0x0084_54f0;
const ARRAY_FIND: u32 = 0x0071_9b20;
const ARRAY_CONSTRUCT: u32 = 0x005e_0510;
const ARRAY_DESTROY: u32 = 0x005e_0540;
/// The comparison `ARRAY_FIND` is given: whether two pointers are equal.
const POINTER_EQUAL: u32 = 0x009a_3830;
/// `006df160(array)`: the address of the first element.
const ARRAY_FIRST: u32 = 0x006d_f160;
/// `0076b610(list)`: a test of the candidate list that rules the
/// connectivity check out.
const LIST_TEST: u32 = 0x0076_b610;
/// Scrap-heap array: destructor, vtable, and the pieces of its constructor
/// (`MemoryManager::GetThreadScrapHeap` and the set-up call `006b3eb0`).
const SCRAP_ARRAY_DESTROY: u32 = 0x006c_f340;
const SCRAP_ARRAY_VTABLE: u32 = 0x0106_c968;
const MEMORY_MANAGER: u32 = 0x0040_1020;
const THREAD_SCRAP_HEAP: u32 = 0x00aa_42e0;
const SCRAP_ARRAY_SET_UP: u32 = 0x006b_3eb0;
/// `006a9520`: bit 1 of `uiFlags` (the resolving functions give up).
const FLAGS_BLOCK_RESOLVING: u32 = 0x006a_9520;
/// `Pathing::CheckLineOfSight_ov2(location, &point, 0, 0)` (cdecl).
const CHECK_LINE_OF_SIGHT: u32 = 0x006d_7490;
/// `005c3420(location, out)`: copies the position into `out`.
const GET_POSITION: u32 = 0x005c_3420;
/// Variable-argument formatting: `sprintf_s(buffer, size, format, ...)`.
const FORMAT_TEXT: u32 = 0x0040_6d00;
/// Save and load buffers: `SaveGame(buffer, data, size, 0)` (`00865e50`),
/// `SaveFormID(form id, 0)` (`00865db0`), `SaveFormID_ov2(form, 0)`
/// (`00865df0`); `LoadGame(buffer, data, size)` (`00864980`),
/// `LoadFormID_ov2(buffer, &id)` (`008648e0`) and `LoadFormID(buffer)`
/// (`008648a0`, the id is the result).
const SAVE_BYTES: u32 = 0x0086_5e50;
const SAVE_FORM_ID: u32 = 0x0086_5db0;
const SAVE_FORM: u32 = 0x0086_5df0;
const LOAD_BYTES: u32 = 0x0086_4980;
const LOAD_FORM_ID: u32 = 0x0086_48e0;
const LOAD_FORM: u32 = 0x0086_48a0;
/// `TES::GetNavMeshInfoMap` and the map's lookup by form id (`00693d70`).
const TES_NAV_MESH_INFO_MAP: u32 = 0x0045_af00;
const NAV_MESH_INFO_MAP_FIND: u32 = 0x0069_3d70;
/// `TESForm` lookup by id, and `__RTDynamicCast`.
const LOOK_UP_FORM: u32 = 0x0048_39c0;
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// Run-time type information descriptors for the load's dynamic casts
/// (the source `TESForm`, then the targets `TESObjectCELL` and
/// `TESWorldSpace`).
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_CELL: u32 = 0x0118_3fb4;
const TYPE_WORLD_SPACE: u32 = 0x0118_3fd0;

/// Strings and formats of `PrintDebugText`.
const TEXT_NONE: u32 = 0x0101_a6c0;
const FORMAT_CELL: u32 = 0x0106_ce4c;
const FORMAT_WORLD_CELL_NAMED: u32 = 0x0106_ce28;
const FORMAT_WORLD_CELL: u32 = 0x0106_ce08;
const FORMAT_TRIANGLE: u32 = 0x0106_cdec;
const FORMAT_NAV_MESH: u32 = 0x0106_cdd8;
/// The virtual slot of `TESForm` that returns the name `PrintDebugText`
/// shows for a cell and a worldspace (byte offset).
const SLOT_NAME: u32 = 0x130;
/// The virtual slot of `TESObjectREFR` that returns the position's address
/// (the same slot other units call).
const SLOT_POSITION: u32 = 0x1f4;

// ---------------------------------------------------------------------------
// Small helpers
// ---------------------------------------------------------------------------

fn word(e: &Engine, this: Ptr<PathingLocation>, offset: u32) -> u32 {
    e.mem.u32(this.addr() + offset)
}

fn set_word(e: &mut Engine, this: Ptr<PathingLocation>, offset: u32, value: u32) {
    e.mem.set_u32(this.addr() + offset, value);
}

/// The address of the position inside the location.
fn position(this: Ptr<PathingLocation>) -> u32 {
    this.addr() + LOCATION
}

fn copy_point(e: &mut Engine, destination: u32, source: u32) {
    for i in 0..3 {
        let value = e.mem.u32(source + 4 * i);
        e.mem.set_u32(destination + 4 * i, value);
    }
}

/// The constructors' common start: the vtable and the position's
/// construction.
fn start_construction(e: &mut Engine, this: Ptr<PathingLocation>) {
    e.mem.set_u32(this.addr(), VTABLE);
    e.call(NI_POINT3_CONSTRUCT, &args![position(this)]);
}

fn array_count(e: &mut Engine, array: u32) -> u32 {
    e.call(ARRAY_COUNT, &args![array]).u32()
}

/// The pointer stored at `index` of the array of navmesh infos at `array`.
fn array_pointer(e: &mut Engine, array: u32, index: u32) -> u32 {
    let element = e.call(ARRAY_ELEMENT, &args![array, index]).u32();
    e.mem.u32(element)
}

/// The same for the arrays inside a navmesh info and the ones the functions
/// build on their stack.
fn stack_array_pointer(e: &mut Engine, array: u32, index: u32) -> u32 {
    let element = e.call(LOCAL_ARRAY_ELEMENT, &args![array, index]).u32();
    e.mem.u32(element)
}

/// A `NiPointer<NavMesh>` the game keeps on its stack for the duration of
/// `body`.
fn with_mesh_pointer<R>(e: &mut Engine, body: impl FnOnce(&mut Engine, u32) -> R) -> R {
    e.with_stack(4, |e, slot| {
        e.call(NI_POINTER_CONSTRUCT, &args![slot]);
        let result = body(e, slot.addr());
        e.call(NI_POINTER_RELEASE, &args![slot]);
        result
    })
}

/// The mesh in the `NiPointer` at `slot`.
fn pointed_mesh(e: &mut Engine, slot: u32) -> u32 {
    e.call(NI_POINTER_GET, &args![slot]).u32()
}

/// The triangle of `position` in the mesh held by `slot` (`00696a50`).
fn find_triangle(e: &mut Engine, slot: u32, point: u32, arg: u32) -> u16 {
    let mesh = pointed_mesh(e, slot);
    e.call(NAV_MESH_FIND_TRIANGLE, &args![mesh, point, arg])
        .u16()
}

/// `ST0` after `_ftol2_sse`, as the integer the game computes.
fn float_to_int(e: &mut Engine, value: f32) -> i32 {
    e.call(FTOL, &args![value as f64]).i32()
}

/// `ArrayFind(array, &value, 0, pointer equality)`: the index of `value`,
/// [`NOT_FOUND`] if it is not in the array.
fn find_pointer(e: &mut Engine, array: u32, value: u32) -> u32 {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), value);
        e.call(ARRAY_FIND, &args![array, slot.addr(), 0u32, POINTER_EQUAL])
            .u32()
    })
}

/// `ArrayAdd(array, &value)` with the value on the stack.
fn add_pointer(e: &mut Engine, array: u32, value: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), value);
        e.call(ARRAY_ADD, &args![array, slot.addr()]);
    });
}

// ---------------------------------------------------------------------------
// Construction
// ---------------------------------------------------------------------------

// Translated from 00441110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingLocation::GetWorldspace` (Xbox PDB): the worldspace.
pub fn path_location_get_world_space(e: &mut Engine, this: Ptr<PathingLocation>) -> Ptr {
    e.get(this, PathingLocation::pWorldSpace)
}

// Translated from 006dcbb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingLocation::PathingLocation()` (Xbox PDB): a location at
/// `(FLT_MAX, FLT_MAX, FLT_MAX)` with no navmesh, cell, worldspace or
/// triangle.
pub fn path_location_path_location(
    e: &mut Engine,
    this: Ptr<PathingLocation>,
) -> Ptr<PathingLocation> {
    start_construction(e, this);
    let max: f32 = e.global(FLOAT_MAX);
    for i in 0..3 {
        e.mem.set_f32(position(this) + 4 * i, max);
    }
    set_word(e, this, CELL, 0);
    set_word(e, this, WORLD_SPACE, 0);
    e.set(this, PathingLocation::usTriangle, NO_TRIANGLE);
    set_word(e, this, CELL_KEY, NO_CELL_KEY);
    set_word(e, this, NAV_MESH, 0);
    set_word(e, this, NAV_MESHES, 0);
    e.set(this, PathingLocation::uiFlags, 0);
    this
}

/// The field copy of the copy constructor and of the assignment: the eight
/// words at +0x04 to +0x20, the triangle and the flags.
fn copy_fields(e: &mut Engine, this: Ptr<PathingLocation>, other: Ptr<PathingLocation>) {
    for offset in (LOCATION..=CELL_KEY).step_by(4) {
        let value = word(e, other, offset);
        set_word(e, this, offset, value);
    }
    let triangle = e.get(other, PathingLocation::usTriangle);
    e.set(this, PathingLocation::usTriangle, triangle);
    let flags = e.get(other, PathingLocation::uiFlags);
    e.set(this, PathingLocation::uiFlags, flags);
}

// Translated from 006dcc40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingLocation::PathingLocation(const PathingLocation&)` (Xbox PDB, the
/// map's `_ov2`): copies every field but the vtable.
pub fn path_location_path_location_ov2(
    e: &mut Engine,
    this: Ptr<PathingLocation>,
    other: Ptr<PathingLocation>,
) -> Ptr<PathingLocation> {
    start_construction(e, this);
    copy_fields(e, this, other);
    this
}

// Translated from 006dcce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingLocation::operator=` (Xbox PDB): copies every field but the
/// vtable and returns `this`.
pub fn path_location_operator_assign(
    e: &mut Engine,
    this: Ptr<PathingLocation>,
    other: Ptr<PathingLocation>,
) -> Ptr<PathingLocation> {
    copy_fields(e, this, other);
    this
}

// Translated from 006dcd70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingLocation::PathingLocation(TESObjectREFR*)` (Xbox PDB, the map's
/// `_ov3`): the location of a reference, in its cell and worldspace; for the
/// player character without either, the `TES` singleton's interior cell or,
/// if none, its worldspace. The position comes from the reference's virtual
/// slot `+0x1f4`.
pub fn path_location_path_location_ov3(
    e: &mut Engine,
    this: Ptr<PathingLocation>,
    reference: Ptr,
) -> Ptr<PathingLocation> {
    start_construction(e, this);
    let mut cell = e.call(REFERENCE_CELL, &args![reference]).u32();
    let mut world_space = e.call(REFERENCE_WORLD_SPACE, &args![reference]).u32();
    if cell == 0 && world_space == 0 && reference.addr() == e.global::<u32>(PLAYER_CHARACTER) {
        let tes: u32 = e.global(TES_SINGLETON);
        cell = e.call(TES_INTERIOR_CELL, &args![tes]).u32();
        if cell == 0 {
            world_space = e.call(TES_WORLD_SPACE, &args![tes]).u32();
        }
    }
    let point = e.vcall(reference.addr(), SLOT_POSITION, &args![]).u32();
    path_location_setup_data(e, this, point, cell, world_space);
    this
}

// Translated from 006dce10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingLocation::PathingLocation(const NiPoint3&, TESObjectREFR*)`
/// (Xbox PDB, the map's `_ov4`): the given position in the reference's cell
/// and worldspace.
pub fn path_location_path_location_ov4(
    e: &mut Engine,
    this: Ptr<PathingLocation>,
    point: u32,
    reference: Ptr,
) -> Ptr<PathingLocation> {
    start_construction(e, this);
    let world_space = e.call(REFERENCE_WORLD_SPACE, &args![reference]).u32();
    let cell = e.call(REFERENCE_CELL, &args![reference]).u32();
    path_location_setup_data(e, this, point, cell, world_space);
    this
}

// Translated from 006dce60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingLocation::PathingLocation(const NiPoint3&, TESObjectCELL*)`
/// (Xbox PDB, the map's `_ov5`).
pub fn path_location_path_location_ov5(
    e: &mut Engine,
    this: Ptr<PathingLocation>,
    point: u32,
    cell: u32,
) -> Ptr<PathingLocation> {
    start_construction(e, this);
    path_location_setup_data(e, this, point, cell, 0);
    this
}

// Translated from 006dcea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingLocation::PathingLocation(const NiPoint3&, TESWorldSpace*)`
/// (Xbox PDB, the map's `_ov6`).
pub fn path_location_path_location_ov6(
    e: &mut Engine,
    this: Ptr<PathingLocation>,
    point: u32,
    world_space: u32,
) -> Ptr<PathingLocation> {
    start_construction(e, this);
    path_location_setup_data(e, this, point, 0, world_space);
    this
}

// Translated from 006dcee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingLocation::PathingLocation(const NiPoint3&, TESObjectCELL*,
/// TESWorldSpace*)` (Xbox PDB, the map's `_ov7`).
pub fn path_location_path_location_ov7(
    e: &mut Engine,
    this: Ptr<PathingLocation>,
    point: u32,
    cell: u32,
    world_space: u32,
) -> Ptr<PathingLocation> {
    start_construction(e, this);
    path_location_setup_data(e, this, point, cell, world_space);
    this
}

/// Sets the navmesh info, no candidate list, the info's worldspace (or, if
/// it has none, its cell) and the key, as the two constructors from a
/// position and a navmesh info do; then flag 1 (`006dcfe0(1)`).
fn adopt_nav_mesh_info(e: &mut Engine, this: Ptr<PathingLocation>, info: u32) {
    set_word(e, this, NAV_MESH, info);
    set_word(e, this, NAV_MESHES, 0);
    let world_space = e.call(INFO_WORLD_SPACE, &args![info]).u32();
    set_word(e, this, WORLD_SPACE, world_space);
    if world_space == 0 {
        let cell = e.call(INFO_CELL, &args![info]).u32();
        set_word(e, this, CELL, cell);
        set_word(e, this, CELL_KEY, NO_CELL_KEY);
    } else {
        set_word(e, this, CELL, 0);
        // NavMeshInfo::iCellKey (Xbox PDB) +0x0C
        let key = e.mem.u32(info + 0x0c);
        set_word(e, this, CELL_KEY, key);
    }
    path_location_set_chosen_by_position(e, this, 1);
}

// Translated from 006dcf20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs a location at `point` inside the navmesh info `info`: the
/// info's worldspace (or cell), no triangle, flag 1 set.
pub fn fn_006dcf20(
    e: &mut Engine,
    this: Ptr<PathingLocation>,
    point: u32,
    info: u32,
) -> Ptr<PathingLocation> {
    start_construction(e, this);
    copy_point(e, position(this), point);
    e.set(this, PathingLocation::uiFlags, 0);
    adopt_nav_mesh_info(e, this, info);
    e.set(this, PathingLocation::usTriangle, NO_TRIANGLE);
    this
}

// Translated from 006dcfe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets bit 0 of `uiFlags` (the navmesh info was chosen by position) to the
/// low byte of `value` (the callers pass 0 or 1).
pub fn path_location_set_chosen_by_position(
    e: &mut Engine,
    this: Ptr<PathingLocation>,
    value: u32,
) {
    let flags = e.get(this, PathingLocation::uiFlags);
    let cleared = flags & 0xfe;
    e.set(this, PathingLocation::uiFlags, cleared);
    e.set(this, PathingLocation::uiFlags, cleared | value as u8);
}

// Translated from 006dd010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Like `fn_006dcf20` with the triangle given.
pub fn fn_006dd010(
    e: &mut Engine,
    this: Ptr<PathingLocation>,
    point: u32,
    info: u32,
    triangle: u16,
) -> Ptr<PathingLocation> {
    start_construction(e, this);
    copy_point(e, position(this), point);
    e.set(this, PathingLocation::uiFlags, 0);
    adopt_nav_mesh_info(e, this, info);
    e.set(this, PathingLocation::usTriangle, triangle);
    this
}

// Translated from 006dd0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs a location at `point` that takes `other`'s navmesh info,
/// worldspace, cell and key if `other`'s navmesh has a triangle for the
/// position; otherwise it is set up from `other`'s cell and worldspace like
/// a plain position. The compiler's unwinding frame is not translated.
pub fn fn_006dd0d0(
    e: &mut Engine,
    this: Ptr<PathingLocation>,
    point: u32,
    other: Ptr<PathingLocation>,
) -> Ptr<PathingLocation> {
    start_construction(e, this);
    copy_point(e, position(this), point);
    e.set(this, PathingLocation::uiFlags, 0);
    e.set(this, PathingLocation::usTriangle, NO_TRIANGLE);
    let adopted = with_mesh_pointer(e, |e, slot| {
        if !path_location_get_nav_mesh(e, other, slot) {
            return false;
        }
        let triangle = find_triangle(e, slot, point, 0);
        e.set(this, PathingLocation::usTriangle, triangle);
        if triangle == NO_TRIANGLE {
            return false;
        }
        let info = word(e, other, NAV_MESH);
        set_word(e, this, NAV_MESH, info);
        set_word(e, this, NAV_MESHES, 0);
        for offset in [CELL, WORLD_SPACE, CELL_KEY] {
            let value = word(e, other, offset);
            set_word(e, this, offset, value);
        }
        true
    });
    if !adopted {
        let world_space = path_location_get_world_space(e, other).addr();
        let cell = path_location_get_cell(e, other);
        path_location_setup_data(e, this, point, cell.addr(), world_space);
    }
    this
}

// Translated from 006dd220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingLocation::PathingLocation(const BGSWorldLocation&)` (Xbox PDB,
/// the map's `_ov8`): the location of a world location, in the cell or
/// worldspace `BGSWorldLocation::GetCellOrWorld` gives; the position is the
/// world location's own address (`006815c0` returns its `this`).
pub fn path_location_path_location_ov8(
    e: &mut Engine,
    this: Ptr<PathingLocation>,
    world_location: u32,
) -> Ptr<PathingLocation> {
    start_construction(e, this);
    e.with_stack(8, |e, out| {
        e.call(
            WORLD_LOCATION_CELL_OR_WORLD,
            &args![world_location, out.addr(), out.addr() + 4],
        );
        let cell = e.mem.u32(out.addr());
        let world_space = e.mem.u32(out.addr() + 4);
        let point = e.call(NI_POINT3_CONSTRUCT, &args![world_location]).u32();
        path_location_setup_data(e, this, point, cell, world_space);
    });
    this
}

// Translated from 006dd280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingLocation::SetupData` (Xbox PDB): stores the position and works
/// out where it lies. With a worldspace: the worldspace, its cell key
/// for the position and the potential navmesh infos for that key. Without:
/// the potential navmesh infos of the cell, and the cell itself stored
/// unless `00425fd0` rejects it, in which case the cell's worldspace and the
/// key of its grid coordinates are stored instead. Exactly one potential
/// navmesh info becomes `pNavMesh` (flag 1 set), otherwise the list is kept
/// and the flag cleared. The triangle is cleared.
pub fn path_location_setup_data(
    e: &mut Engine,
    this: Ptr<PathingLocation>,
    point: u32,
    cell: u32,
    world_space: u32,
) {
    copy_point(e, position(this), point);
    e.set(this, PathingLocation::uiFlags, 0);
    if world_space != 0 {
        set_word(e, this, WORLD_SPACE, world_space);
        let key = e
            .call(WORLD_SPACE_KEY_FOR_COORD, &args![position(this)])
            .u32();
        set_word(e, this, CELL_KEY, key);
        set_word(e, this, CELL, 0);
        let list = e
            .call(POTENTIAL_NAV_MESHES_FOR_KEY, &args![world_space, key])
            .u32();
        set_word(e, this, NAV_MESHES, list);
    } else {
        let list = e.call(POTENTIAL_NAV_MESHES_FOR_CELL, &args![cell]).u32();
        set_word(e, this, NAV_MESHES, list);
        if cell != 0 && !e.call(CELL_TEST, &args![cell]).bool() {
            let cell_world_space = e.call(CELL_WORLD_SPACE, &args![cell]).u32();
            set_word(e, this, WORLD_SPACE, cell_world_space);
            let grid_y = e.call(CELL_DATA_Y, &args![cell]).u32();
            let grid_x = e.call(CELL_DATA_X, &args![cell]).u32();
            let key = e.call(CELL_KEY_FROM_GRID, &args![grid_x, grid_y]).u32();
            set_word(e, this, CELL_KEY, key);
            set_word(e, this, CELL, 0);
        } else {
            set_word(e, this, CELL, cell);
            set_word(e, this, WORLD_SPACE, 0);
            set_word(e, this, CELL_KEY, NO_CELL_KEY);
        }
    }
    let list = word(e, this, NAV_MESHES);
    if list != 0 && array_count(e, list) == 1 {
        path_location_set_chosen_by_position(e, this, 1);
        let only = array_pointer(e, list, 0);
        set_word(e, this, NAV_MESH, only);
        set_word(e, this, NAV_MESHES, 0);
    } else {
        path_location_set_chosen_by_position(e, this, 0);
        set_word(e, this, NAV_MESH, 0);
    }
    e.set(this, PathingLocation::usTriangle, NO_TRIANGLE);
}

// Translated from 006dd3e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Makes `info` the location's navmesh info (no candidate list), with the
/// info's worldspace (or cell) and key, flag 1 set and no triangle.
pub fn fn_006dd3e0(e: &mut Engine, this: Ptr<PathingLocation>, info: u32) {
    adopt_nav_mesh_info(e, this, info);
    e.set(this, PathingLocation::usTriangle, NO_TRIANGLE);
}

// Translated from 006dd460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingLocation::SetLocation` (Xbox PDB): moves the location to `point`
/// and clears the triangle; with a worldspace whose cell key for the new
/// position differs from the stored one, it is set up again from scratch
/// (`SetupData(point, no cell, worldspace)`).
pub fn path_location_set_location(e: &mut Engine, this: Ptr<PathingLocation>, point: u32) {
    copy_point(e, position(this), point);
    e.set(this, PathingLocation::usTriangle, NO_TRIANGLE);
    let world_space = word(e, this, WORLD_SPACE);
    if world_space != 0 {
        let key = e
            .call(WORLD_SPACE_KEY_FOR_COORD, &args![position(this)])
            .u32();
        if key != word(e, this, CELL_KEY) {
            path_location_setup_data(e, this, point, 0, world_space);
        }
    }
}

// Translated from 006dd4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the triangle.
pub fn fn_006dd4d0(e: &mut Engine, this: Ptr<PathingLocation>, triangle: u16) {
    e.set(this, PathingLocation::usTriangle, triangle);
}

// ---------------------------------------------------------------------------
// Queries
// ---------------------------------------------------------------------------

// Translated from 006dd4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingLocation::GetCell` (Xbox PDB): the stored cell, else the
/// worldspace's cell for the key, else null.
pub fn path_location_get_cell(e: &mut Engine, this: Ptr<PathingLocation>) -> Ptr {
    let cell = e.get(this, PathingLocation::pCell);
    if !cell.is_null() {
        return cell;
    }
    let world_space = word(e, this, WORLD_SPACE);
    if world_space != 0 {
        let key = word(e, this, CELL_KEY);
        let found = e.call(WORLD_SPACE_CELL_FROM_KEY, &args![world_space, key]);
        if found.u32() != 0 {
            return found.ptr();
        }
    }
    Ptr::NULL
}

// Translated from 006dd540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingLocation::GetBestNavMeshInfo` (Xbox PDB): `pNavMesh`; else, if
/// there is a candidate list and the flag says the info was chosen by
/// position, the list's first entry; else null.
pub fn path_location_get_best_nav_mesh_info(e: &mut Engine, this: Ptr<PathingLocation>) -> u32 {
    let info = word(e, this, NAV_MESH);
    if info != 0 {
        return info;
    }
    let list = word(e, this, NAV_MESHES);
    if list == 0 {
        return 0;
    }
    if fn_006dd5a0(e, this) == 0 {
        return 0;
    }
    array_pointer(e, list, 0)
}

// Translated from 006dd5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bit 0 of `uiFlags` (the navmesh info was chosen by position).
pub fn fn_006dd5a0(e: &mut Engine, this: Ptr<PathingLocation>) -> u32 {
    e.get(this, PathingLocation::uiFlags) as u32 & FLAG_CHOSEN_BY_POSITION
}

// Translated from 006dd5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingLocation::GetNavMeshInfos` (Xbox PDB): fills `out` with the
/// navmesh info (added by `007cb2e0`) or, if there is only a candidate
/// list, a copy of the list (`006e5330`); false if there is neither.
pub fn path_location_get_nav_mesh_infos(
    e: &mut Engine,
    this: Ptr<PathingLocation>,
    out: u32,
) -> bool {
    if word(e, this, NAV_MESH) != 0 {
        e.call(ARRAY_ADD, &args![out, this.addr() + NAV_MESH]);
        return true;
    }
    let list = word(e, this, NAV_MESHES);
    if list != 0 {
        e.call(ARRAY_COPY_TO, &args![list, out]);
        return true;
    }
    false
}

// Translated from 006dd610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingLocation::GetNavMesh` (Xbox PDB): loads the navmesh of
/// `pNavMesh` into the `NiPointer` at `out`; false without a navmesh info.
pub fn path_location_get_nav_mesh(e: &mut Engine, this: Ptr<PathingLocation>, out: u32) -> bool {
    let info = word(e, this, NAV_MESH);
    if info == 0 {
        return false;
    }
    e.call(INFO_LOAD_NAV_MESH, &args![info, out]).bool()
}

// Translated from 006dd640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingLocation::GetNavMeshAndTriangle` (Xbox PDB): loads the navmesh
/// into the `NiPointer` at `out_mesh` and writes the triangle (a `u16`) to
/// `out_triangle`; false without a navmesh info, without a triangle or if
/// the navmesh does not load.
pub fn path_location_get_nav_mesh_and_triangle(
    e: &mut Engine,
    this: Ptr<PathingLocation>,
    out_mesh: u32,
    out_triangle: u32,
) -> bool {
    let info = word(e, this, NAV_MESH);
    if info == 0 {
        return false;
    }
    let triangle = e.get(this, PathingLocation::usTriangle);
    if triangle == NO_TRIANGLE {
        return false;
    }
    if !e.call(INFO_LOAD_NAV_MESH, &args![info, out_mesh]).bool() {
        return false;
    }
    e.mem.set_u16(out_triangle, triangle);
    true
}

// Translated from 006dd6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when the location has a navmesh info whose navmesh is loaded and a
/// triangle.
pub fn fn_006dd6a0(e: &mut Engine, this: Ptr<PathingLocation>) -> bool {
    let info = word(e, this, NAV_MESH);
    if info == 0 {
        return false;
    }
    if !e.call(INFO_HAS_NAV_MESH, &args![info]).bool() {
        return false;
    }
    e.get(this, PathingLocation::usTriangle) != NO_TRIANGLE
}

// ---------------------------------------------------------------------------
// Resolving
// ---------------------------------------------------------------------------

// Translated from 006dd6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingLocation::ResolveNavMeshInfo` (Xbox PDB): decides the location's
/// navmesh info (and triangle) from its candidate list; `arg` goes to the
/// triangle search (`00696a50`). False if `006a9520` says to give up, there
/// is nothing to resolve, or the navmesh info is disabled (flag `0x10`).
///
/// With a navmesh info already: only the triangle is searched, if missing.
/// Otherwise, if the cell is the kind `00450ff0` accepts, each candidate
/// whose navmesh loads, is not flagged by `00440da0` and has a triangle for
/// the position is taken; when none is, the closest navmesh and triangle are
/// resolved (`ResolveToClosestNavmeshAndTriangle`, the position restored
/// afterwards). In every other case the first candidate that is not disabled
/// and has bounds that contain the position is taken, else the one with the
/// closest bounds within 16384 (squared distance); failing both, the
/// candidate without bounds is taken and `fn_006de490` decides the flag.
pub fn path_location_resolve_nav_mesh_info(
    e: &mut Engine,
    this: Ptr<PathingLocation>,
    arg: u32,
) -> bool {
    if e.call(FLAGS_BLOCK_RESOLVING, &args![this]).bool() {
        return false;
    }
    let info = word(e, this, NAV_MESH);
    let list = word(e, this, NAV_MESHES);
    if info == 0 && list == 0 {
        return false;
    }
    if info != 0 {
        if e.call(INFO_IS_DISABLED, &args![info]).bool() {
            return false;
        }
        if e.get(this, PathingLocation::usTriangle) == NO_TRIANGLE {
            with_mesh_pointer(e, |e, slot| {
                if e.call(INFO_LOAD_NAV_MESH, &args![info, slot]).bool() {
                    let triangle = find_triangle(e, slot, position(this), arg);
                    e.set(this, PathingLocation::usTriangle, triangle);
                }
            });
        }
        return true;
    }

    if !path_location_get_cell(e, this).is_null() {
        let cell = path_location_get_cell(e, this);
        if e.call(CELL_IS_KIND, &args![cell]).bool() {
            for index in 0..array_count(e, list) {
                let taken = with_mesh_pointer(e, |e, slot| {
                    let candidate = array_pointer(e, list, index);
                    if !e.call(INFO_LOAD_NAV_MESH, &args![candidate, slot]).bool() {
                        return false;
                    }
                    let mesh = pointed_mesh(e, slot);
                    if e.call(FORM_TEST_SECOND, &args![mesh]).bool() {
                        return false;
                    }
                    let triangle = find_triangle(e, slot, position(this), arg);
                    if triangle == NO_TRIANGLE {
                        return false;
                    }
                    let chosen = array_pointer(e, list, index);
                    set_word(e, this, NAV_MESH, chosen);
                    set_word(e, this, NAV_MESHES, 0);
                    e.set(this, PathingLocation::usTriangle, triangle);
                    path_location_set_chosen_by_position(e, this, 1);
                    true
                });
                if taken {
                    return true;
                }
            }
            let saved = [
                word(e, this, LOCATION),
                word(e, this, LOCATION + 4),
                word(e, this, LOCATION + 8),
            ];
            let resolved = path_location_resolve_to_closest_nav_mesh_and_triangle(e, this);
            e.set(this, PathingLocation::usTriangle, NO_TRIANGLE);
            for (i, value) in saved.into_iter().enumerate() {
                set_word(e, this, LOCATION + 4 * i as u32, value);
            }
            if resolved {
                path_location_set_chosen_by_position(e, this, 1);
            } else {
                let flag = fn_006dd5a0(e, this);
                path_location_set_chosen_by_position(e, this, flag);
            }
            return resolved;
        }
    }

    // The first candidate whose bounds contain the position.
    for index in 0..array_count(e, list) {
        let candidate = array_pointer(e, list, index);
        if e.call(INFO_IS_DISABLED, &args![candidate]).bool() {
            continue;
        }
        let bounds = e.call(INFO_BOUNDS, &args![candidate]).u32();
        if bounds != 0
            && e.call(BOUNDS_CONTAIN, &args![bounds, position(this)])
                .bool()
        {
            let chosen = array_pointer(e, list, index);
            set_word(e, this, NAV_MESH, chosen);
            set_word(e, this, NAV_MESHES, 0);
            with_mesh_pointer(e, |e, slot| {
                if e.call(INFO_LOAD_NAV_MESH, &args![chosen, slot]).bool() {
                    let triangle = find_triangle(e, slot, position(this), arg);
                    e.set(this, PathingLocation::usTriangle, triangle);
                }
            });
            path_location_set_chosen_by_position(e, this, 1);
            return true;
        }
    }

    // The closest bounds, and the candidate without bounds.
    let mut without_bounds = NOT_FOUND;
    let mut closest_distance: f32 = e.global(FLOAT_MAX);
    let mut closest = NOT_FOUND;
    for index in 0..array_count(e, list) {
        let candidate = array_pointer(e, list, index);
        if e.call(INFO_IS_DISABLED, &args![candidate]).bool() {
            continue;
        }
        let bounds = e.call(INFO_BOUNDS, &args![candidate]).u32();
        if bounds == 0 {
            without_bounds = index;
        } else {
            let distance = e.with_stack(12, |e, offset| {
                e.call(
                    BOUNDS_OFFSET_TO,
                    &args![bounds, offset.addr(), position(this)],
                );
                e.call(POINT_SQUARED_LENGTH, &args![offset.addr()]).f32()
            });
            if distance < closest_distance {
                closest = index;
                closest_distance = distance;
            }
        }
    }
    let far: f64 = e.global(FAR_DISTANCE);
    if (closest_distance as f64) < far {
        let chosen = array_pointer(e, list, closest);
        set_word(e, this, NAV_MESH, chosen);
        set_word(e, this, NAV_MESHES, 0);
        e.set(this, PathingLocation::usTriangle, NO_TRIANGLE);
        path_location_set_chosen_by_position(e, this, 1);
        true
    } else {
        let chosen = array_pointer(e, list, without_bounds);
        set_word(e, this, NAV_MESH, chosen);
        set_word(e, this, NAV_MESHES, 0);
        let neighbours_reach_all = fn_006de490(e, this);
        path_location_set_chosen_by_position(e, this, neighbours_reach_all as u32);
        fn_006dd5a0(e, this) != 0
    }
}

// Translated from 006ddbe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingLocation::ClearTriangleData` (Xbox PDB): no triangle.
pub fn path_location_clear_triangle_data(e: &mut Engine, this: Ptr<PathingLocation>) {
    e.set(this, PathingLocation::usTriangle, NO_TRIANGLE);
}

/// After the closest triangle was found: when the position is not inside it,
/// the triangle centre (`00558220`) and the edge crossing (`00698160`)
/// decide a new position, `edge point * 0.9 + centre * 0.1`; `slot` is the
/// `NiPointer<NavMesh>` and the triangle is `usTriangle`.
fn pull_position_to_triangle_centre(e: &mut Engine, this: Ptr<PathingLocation>, slot: u32) {
    let triangle = e.get(this, PathingLocation::usTriangle) as u32;
    e.with_stack(0x80, |e, temp| {
        let centre_out = temp.addr();
        let edge = temp.addr() + 0x10;
        let centre_part = temp.addr() + 0x30;
        let edge_part = temp.addr() + 0x40;
        let sum = temp.addr() + 0x50;
        let mesh = pointed_mesh(e, slot);
        let centre = e
            .call(NAV_MESH_GET_CENTER, &args![mesh, centre_out, triangle])
            .u32();
        let mesh = pointed_mesh(e, slot);
        e.call(
            NAV_MESH_FIND_INTERSECTING_EDGE,
            &args![mesh, edge, triangle, u32::MAX, centre, position(this)],
        );
        if e.mem.u32(edge) == 0 {
            return;
        }
        let centre_weight: f32 = e.global(CENTER_WEIGHT);
        let edge_weight: f32 = e.global(EDGE_WEIGHT);
        // The crossing point is at +8 of the record `00698160` fills.
        let scaled_centre = e
            .call(POINT_SCALE, &args![centre, centre_part, centre_weight])
            .u32();
        let scaled_edge = e
            .call(POINT_SCALE, &args![edge + 8, edge_part, edge_weight])
            .u32();
        let result = e
            .call(POINT_ADD, &args![scaled_edge, sum, scaled_centre])
            .u32();
        copy_point(e, position(this), result);
    });
}

// Translated from 006ddc00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingLocation::ResolveToClosestNavmeshAndTriangle` (Xbox PDB): sets
/// the location up again from its own cell and worldspace, then finds the
/// navmesh and triangle.
///
/// With a navmesh info and no triangle: the closest triangle of that
/// navmesh (`00696cf0`); when the position is not inside it, the position
/// is pulled in towards the triangle centre (`0.9` edge point, `0.1`
/// centre). Without a navmesh info: each candidate that is not disabled
/// and whose navmesh loads is searched the same way; a candidate that holds
/// the position exactly is taken at once; otherwise the best is the one
/// with the smallest squared horizontal distance to its triangle centre,
/// those whose centre is within 25 units of the ground height or between
/// `-64` and `180` units below the position being preferred. False if
/// nothing is found.
pub fn path_location_resolve_to_closest_nav_mesh_and_triangle(
    e: &mut Engine,
    this: Ptr<PathingLocation>,
) -> bool {
    let cell = word(e, this, CELL);
    let world_space = word(e, this, WORLD_SPACE);
    path_location_setup_data(e, this, position(this), cell, world_space);
    let info = word(e, this, NAV_MESH);
    let list = word(e, this, NAV_MESHES);
    if info == 0 && list == 0 {
        return false;
    }
    if info != 0 && e.get(this, PathingLocation::usTriangle) != NO_TRIANGLE {
        return true;
    }

    if info != 0 {
        return with_mesh_pointer(e, |e, slot| {
            if !e.call(INFO_LOAD_NAV_MESH, &args![info, slot]).bool() {
                return false;
            }
            let (found, exact) = e.with_stack(4, |e, exact_slot| {
                let mesh = pointed_mesh(e, slot);
                let found = e
                    .call(
                        NAV_MESH_FIND_CLOSEST_TRIANGLE,
                        &args![
                            mesh,
                            position(this),
                            this.addr() + TRIANGLE,
                            exact_slot.addr()
                        ],
                    )
                    .bool();
                (found, e.mem.u8(exact_slot.addr()) != 0)
            });
            if !found {
                return false;
            }
            if !exact {
                pull_position_to_triangle_centre(e, this, slot);
            }
            true
        });
    }

    let mut best_distance: f32 = e.global(FLOAT_MAX);
    let mut best_index = NOT_FOUND;
    let mut best_triangle = NO_TRIANGLE;
    let mut have_inside = false;
    let count = array_count(e, list);
    let tes: u32 = e.global(TES_SINGLETON);
    let ground_cell = word(e, this, CELL);
    let ground = e
        .call(TES_GROUND_HEIGHT, &args![tes, position(this), ground_cell])
        .f64();
    let margin: f64 = e.global(GROUND_MARGIN);
    let height = e.mem.f32(position(this) + 8);
    let near_ground = (height as f64) < ground + margin;
    let max_above: f64 = e.global(MAX_HEIGHT_ABOVE);
    let min_below: f64 = e.global(MIN_HEIGHT_BELOW);
    for index in 0..count {
        let exact_hit = with_mesh_pointer(e, |e, slot| {
            // `triangle` (u16) and `exact` (u8) are the block the
            // closest-triangle search fills in.
            let (triangle, exact) = e.with_stack(4, |e, block| {
                e.mem.set_u16(block.addr(), NO_TRIANGLE);
                e.mem.set_u8(block.addr() + 2, 0);
                let candidate = array_pointer(e, list, index);
                let disabled = e.call(INFO_IS_DISABLED, &args![candidate]).bool();
                // A disabled candidate, or one whose navmesh does not load,
                // goes on to the processing below with the defaults.
                if !disabled && e.call(INFO_LOAD_NAV_MESH, &args![candidate, slot]).bool() {
                    let mesh = pointed_mesh(e, slot);
                    let found = e
                        .call(
                            NAV_MESH_FIND_CLOSEST_TRIANGLE,
                            &args![mesh, position(this), block.addr(), block.addr() + 2],
                        )
                        .bool();
                    if !found {
                        return None;
                    }
                }
                Some((e.mem.u16(block.addr()), e.mem.u8(block.addr() + 2) != 0))
            })?;
            if exact {
                let chosen = array_pointer(e, list, index);
                set_word(e, this, NAV_MESH, chosen);
                set_word(e, this, NAV_MESHES, 0);
                e.set(this, PathingLocation::usTriangle, triangle);
                return Some(true);
            }
            if e.call(NI_POINTER_HAS_MESH, &args![slot]).u32() == 0 {
                return Some(false);
            }
            let (distance, height_difference) = e.with_stack(0x30, |e, temp| {
                let centre_out = temp.addr();
                let difference_out = temp.addr() + 0x10;
                let mesh = pointed_mesh(e, slot);
                let centre = e
                    .call(
                        NAV_MESH_GET_CENTER,
                        &args![mesh, centre_out, triangle as u32],
                    )
                    .u32();
                let difference = e
                    .call(
                        POINT_SUBTRACT,
                        &args![position(this), difference_out, centre],
                    )
                    .u32();
                let distance = e.call(POINT_SQUARED_LENGTH_2D, &args![difference]).f32();
                (distance, e.mem.f32(difference + 8))
            });
            let height_difference = height_difference as f64;
            let acceptable =
                (near_ground || height_difference < max_above) && height_difference > min_below;
            let nearer = distance < best_distance;
            if have_inside {
                if acceptable && nearer {
                    best_triangle = triangle;
                    best_index = index;
                    best_distance = distance;
                }
            } else if acceptable {
                best_triangle = triangle;
                best_index = index;
                have_inside = true;
                best_distance = distance;
            } else if nearer {
                best_triangle = triangle;
                best_index = index;
                best_distance = distance;
            }
            Some(false)
        });
        if exact_hit == Some(true) {
            return true;
        }
    }
    if best_index == NOT_FOUND {
        return false;
    }
    let chosen = array_pointer(e, list, best_index);
    set_word(e, this, NAV_MESH, chosen);
    set_word(e, this, NAV_MESHES, 0);
    e.set(this, PathingLocation::usTriangle, best_triangle);
    with_mesh_pointer(e, |e, slot| {
        let info = word(e, this, NAV_MESH);
        if !e.call(INFO_LOAD_NAV_MESH, &args![info, slot]).bool() {
            return false;
        }
        pull_position_to_triangle_centre(e, this, slot);
        true
    })
}

// Translated from 006de250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Decides the navmesh info among the candidates by position: false if
/// `006a9520` says to give up or there is nothing; with a navmesh info,
/// true unless it is disabled. Otherwise a copy of the list is pruned: a
/// candidate that has bounds is kept alone (and the pruning ends) when the
/// bounds contain the position, else dropped; one without bounds is dropped
/// when its navmesh loads and `00440d80` holds for it. A single survivor
/// becomes `pNavMesh` (flag 1); otherwise `fn_006de490` decides the flag.
pub fn fn_006de250(e: &mut Engine, this: Ptr<PathingLocation>) -> bool {
    if e.call(FLAGS_BLOCK_RESOLVING, &args![this]).bool() {
        return false;
    }
    let info = word(e, this, NAV_MESH);
    let list = word(e, this, NAV_MESHES);
    if info == 0 && list == 0 {
        return false;
    }
    if info != 0 {
        return !e.call(INFO_IS_DISABLED, &args![info]).bool();
    }
    e.with_stack(0x10, |e, copy| {
        let copy = copy.addr();
        e.call(ARRAY_CONSTRUCT, &args![copy]);
        e.call(ARRAY_COPY_TO, &args![list, copy]);
        let mut index: u32 = 0;
        while index < array_count(e, copy) {
            let candidate = stack_array_pointer(e, copy, index);
            let bounds = e.call(INFO_BOUNDS, &args![candidate]).u32();
            if bounds != 0 {
                if e.call(BOUNDS_CONTAIN, &args![bounds, position(this)])
                    .bool()
                {
                    e.call(ARRAY_EMPTY, &args![copy, 0u32]);
                    add_pointer(e, copy, candidate);
                    break;
                }
                e.call(ARRAY_REMOVE, &args![copy, index, 1u32]);
                index = index.wrapping_sub(1);
            } else {
                let drop = with_mesh_pointer(e, |e, slot| {
                    if e.call(INFO_LOAD_NAV_MESH, &args![candidate, slot]).bool() {
                        let mesh = pointed_mesh(e, slot);
                        e.call(FORM_TEST_FIRST, &args![mesh]).bool()
                    } else {
                        false
                    }
                });
                if drop {
                    e.call(ARRAY_REMOVE, &args![copy, index, 1u32]);
                    index = index.wrapping_sub(1);
                }
            }
            index = index.wrapping_add(1);
        }
        let result = if array_count(e, copy) == 1 {
            let first = e.call(ARRAY_FIRST, &args![copy]).u32();
            let only = e.mem.u32(first);
            set_word(e, this, NAV_MESH, only);
            set_word(e, this, NAV_MESHES, 0);
            path_location_set_chosen_by_position(e, this, 1);
            true
        } else {
            let reaches_all = fn_006de490(e, this);
            path_location_set_chosen_by_position(e, this, reaches_all as u32);
            fn_006dd5a0(e, this) != 0
        };
        e.call(ARRAY_DESTROY, &args![copy]);
        result
    })
}

/// Constructs the scrap-heap array the game keeps on its stack (0x14
/// bytes), runs `body` and destroys it with `006cf340`.
fn with_scrap_array<R>(e: &mut Engine, body: impl FnOnce(&mut Engine, u32) -> R) -> R {
    e.with_stack(0x14, |e, array| {
        fn_006df180(e, array);
        let result = body(e, array.addr());
        e.call(SCRAP_ARRAY_DESTROY, &args![array]);
        result
    })
}

// Translated from 006de490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the candidates are all reachable from the first one through the
/// adjacent navmesh infos (`+0x24` and `+0x34` of a `NavMeshInfo`). True at
/// once when there already is a navmesh info; false when there is no
/// candidate list or `0076b610` holds for it. A scrap-heap array starts with
/// the first candidate, takes the first candidate's neighbours that are
/// candidates, then keeps adding the not yet taken candidates among the
/// (adjacent only) neighbours of the taken ones until nothing is added. True
/// when it then holds as many entries as the candidate list (and, early,
/// when the first round already does).
///
/// The inner loop of the second phase is bounded by the adjacent array's
/// count only, so its "preferred" half (indexes at or above the adjacent
/// count) never runs and is left out.
pub fn fn_006de490(e: &mut Engine, this: Ptr<PathingLocation>) -> bool {
    with_scrap_array(e, |e, reached| {
        if word(e, this, NAV_MESH) != 0 {
            return true;
        }
        let list = word(e, this, NAV_MESHES);
        if list == 0 || e.call(LIST_TEST, &args![list]).bool() {
            return false;
        }
        let first_element = e.call(ARRAY_ELEMENT, &args![list, 0u32]).u32();
        e.call(ARRAY_ADD, &args![reached, first_element]);
        let first = array_pointer(e, list, 0);
        let adjacent = array_count(e, first + 0x24);
        let preferred = array_count(e, first + 0x34);
        for index in 0..adjacent + preferred {
            let neighbour = if index < adjacent {
                stack_array_pointer(e, first + 0x24, index)
            } else {
                stack_array_pointer(e, first + 0x34, index - adjacent)
            };
            if find_pointer(e, list, neighbour) != NOT_FOUND {
                add_pointer(e, reached, neighbour);
            }
        }
        if array_count(e, reached) == array_count(e, list) {
            return true;
        }
        loop {
            let mut added = false;
            let mut index = 1;
            while index < array_count(e, reached) {
                let taken = stack_array_pointer(e, reached, index);
                let mut neighbour_index = 0;
                while neighbour_index < array_count(e, taken + 0x24) {
                    let neighbour = stack_array_pointer(e, taken + 0x24, neighbour_index);
                    if find_pointer(e, list, neighbour) != NOT_FOUND
                        && find_pointer(e, reached, neighbour) == NOT_FOUND
                    {
                        add_pointer(e, reached, neighbour);
                        added = true;
                    }
                    neighbour_index += 1;
                }
                index += 1;
            }
            if !added {
                break;
            }
        }
        array_count(e, reached) == array_count(e, list)
    })
}

// ---------------------------------------------------------------------------
// Debug text, validity, comparison
// ---------------------------------------------------------------------------

// Translated from 006de790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingLocation::PrintDebugText` (Xbox PDB): writes a description of
/// the location into `buffer` (`size` bytes) with `sprintf_s`: the cell
/// (its name from virtual slot `+0x130`, or `"NONE"`), or the worldspace
/// with the grid coordinates (position / 4096) and the cell found there, and
/// the position; with a navmesh info, a text buffer that the function never
/// fills (it is uninitialised stack in the game, zeros here), the triangle
/// if there is one and the position.
///
/// The branch that lists the candidate navmesh infos is behind a test that
/// can never hold (it is reached only with `pNavMesh != 0`, which the
/// branch itself needs to be 0), so it is not translated. The stack cookie
/// check is not translated either.
pub fn path_location_print_debug_text(
    e: &mut Engine,
    this: Ptr<PathingLocation>,
    buffer: u32,
    size: u32,
) {
    let info = word(e, this, NAV_MESH);
    let list = word(e, this, NAV_MESHES);
    let x = e.mem.f32(position(this)) as f64;
    let y = e.mem.f32(position(this) + 4) as f64;
    let z = e.mem.f32(position(this) + 8) as f64;
    let cell_text = info == 0 || (list != 0 && array_count(e, list) == 0);
    if !cell_text {
        // A navmesh info: the text buffer is never written.
        e.with_stack(0x100, |e, text| {
            let triangle = e.get(this, PathingLocation::usTriangle);
            if triangle != NO_TRIANGLE {
                e.call(
                    FORMAT_TEXT,
                    &args![
                        buffer,
                        size,
                        FORMAT_TRIANGLE,
                        text,
                        triangle as u32,
                        x,
                        y,
                        z
                    ],
                );
            } else {
                e.call(
                    FORMAT_TEXT,
                    &args![buffer, size, FORMAT_NAV_MESH, text, x, y, z],
                );
            }
        });
        return;
    }
    let world_space = word(e, this, WORLD_SPACE);
    if world_space == 0 {
        let cell = word(e, this, CELL);
        let name = if cell != 0 {
            e.vcall(cell, SLOT_NAME, &args![]).u32()
        } else {
            TEXT_NONE
        };
        e.call(
            FORMAT_TEXT,
            &args![buffer, size, FORMAT_CELL, name, x, y, z],
        );
        return;
    }
    let grid_x = float_to_int(e, x as f32) >> 12;
    let grid_y = float_to_int(e, y as f32) >> 12;
    let found = e
        .call(
            WORLD_SPACE_CELL_FROM_COORD,
            &args![world_space, position(this)],
        )
        .u32();
    if found != 0 {
        let cell_name = e.vcall(found, SLOT_NAME, &args![]).u32();
        let world_name = e.vcall(world_space, SLOT_NAME, &args![]).u32();
        e.call(
            FORMAT_TEXT,
            &args![
                buffer,
                size,
                FORMAT_WORLD_CELL_NAMED,
                world_name,
                grid_x,
                grid_y,
                cell_name,
                x,
                y,
                z
            ],
        );
    } else {
        let world_name = e.vcall(world_space, SLOT_NAME, &args![]).u32();
        e.call(
            FORMAT_TEXT,
            &args![
                buffer,
                size,
                FORMAT_WORLD_CELL,
                world_name,
                grid_x,
                grid_y,
                x,
                y,
                z
            ],
        );
    }
}

// Translated from 006ded70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingLocation::IsValidLocation` (Xbox PDB): resolves the navmesh info
/// and returns whether the location has a loaded navmesh and a triangle and,
/// for a positive `radius`, whether four line-of-sight checks hold (from the
/// location to the points `radius` away in -x, +x, -y and +y).
pub fn path_location_is_valid_location(
    e: &mut Engine,
    this: Ptr<PathingLocation>,
    radius: f32,
) -> bool {
    path_location_resolve_nav_mesh_info(e, this, 0);
    if !fn_006dd6a0(e, this) {
        return false;
    }
    let zero: f64 = e.global(ZERO);
    if (radius as f64) <= zero {
        return true;
    }
    e.with_stack(0x20, |e, block| {
        let here = block.addr();
        let point = block.addr() + 0x10;
        e.call(GET_POSITION, &args![this, here]);
        // (axis, away): the x axis first, then y, each below and above.
        for (axis, below) in [(0u32, true), (0, false), (1, true), (1, false)] {
            copy_point(e, point, here);
            let value = e.mem.f32(point + 4 * axis);
            let shifted = if below {
                value - radius
            } else {
                value + radius
            };
            e.mem.set_f32(point + 4 * axis, shifted);
            if !e
                .call(CHECK_LINE_OF_SIGHT, &args![this, point, 0u32, 0u32])
                .bool()
            {
                return false;
            }
        }
        true
    })
}

// Translated from 006deeb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `other` is in the same worldspace (and with a worldspace, the
/// same cell key; without, the same cell) and within `radius` of this
/// location (the squared distance, `004a7290`, at most `radius * radius`).
pub fn fn_006deeb0(
    e: &mut Engine,
    this: Ptr<PathingLocation>,
    other: Ptr<PathingLocation>,
    radius: f32,
) -> bool {
    if word(e, other, WORLD_SPACE) != word(e, this, WORLD_SPACE) {
        return false;
    }
    if word(e, this, WORLD_SPACE) != 0 {
        if word(e, other, CELL_KEY) != word(e, this, CELL_KEY) {
            return false;
        }
    } else if word(e, other, CELL) != word(e, this, CELL) {
        return false;
    }
    e.with_stack(0x10, |e, difference| {
        let result = e
            .call(
                POINT_SUBTRACT,
                &args![position(other), difference.addr(), position(this)],
            )
            .u32();
        let squared = e.call(POINT_SQUARED_LENGTH, &args![result]).f64();
        let limit = radius as f64 * radius as f64;
        // `FCOMPP` of the limit against the squared distance: false only
        // when the limit is below it (a NaN counts as within).
        limit.partial_cmp(&squared) != Some(std::cmp::Ordering::Less)
    })
}

// ---------------------------------------------------------------------------
// Save and load
// ---------------------------------------------------------------------------

// Translated from 006def40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingLocation::SaveGame` (Xbox PDB): writes the position (12 bytes), the
/// navmesh info's id (0 if none), the cell and worldspace as form ids, the
/// key (4 bytes), the triangle (2), the flags (1) and a byte that says
/// whether a candidate list is held.
pub fn path_location_save_game(e: &mut Engine, this: Ptr<PathingLocation>, buffer: u32) {
    e.call(SAVE_BYTES, &args![buffer, position(this), 0x0cu32, 0u32]);
    let info = word(e, this, NAV_MESH);
    let id = if info != 0 {
        // `NavMeshInfo::NavMeshID` (Xbox PDB) +0x00, read by `00559450`.
        e.call(NI_POINTER_GET, &args![info]).u32()
    } else {
        0
    };
    e.call(SAVE_FORM_ID, &args![buffer, id, 0u32]);
    let cell = word(e, this, CELL);
    e.call(SAVE_FORM, &args![buffer, cell, 0u32]);
    let world_space = word(e, this, WORLD_SPACE);
    e.call(SAVE_FORM, &args![buffer, world_space, 0u32]);
    e.call(
        SAVE_BYTES,
        &args![buffer, this.addr() + CELL_KEY, 4u32, 0u32],
    );
    e.call(
        SAVE_BYTES,
        &args![buffer, this.addr() + TRIANGLE, 2u32, 0u32],
    );
    e.call(SAVE_BYTES, &args![buffer, this.addr() + FLAGS, 1u32, 0u32]);
    let has_list = word(e, this, NAV_MESHES) != 0;
    e.with_stack(4, |e, flag| {
        e.mem.set_u8(flag.addr(), has_list as u8);
        e.call(SAVE_BYTES, &args![buffer, flag.addr(), 1u32, 0u32]);
    });
}

// Translated from 006df010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingLocation::LoadGame` (Xbox PDB): reads what `SaveGame` wrote. The
/// navmesh info is looked up by id in the `TES` navmesh info map, the cell
/// and worldspace by form id and `__RTDynamicCast`; the triangle is read and
/// then set to none. If the candidate-list byte is set, the candidate list is
/// recomputed from the worldspace and key, or from the cell if there is one.
pub fn path_location_load_game(e: &mut Engine, this: Ptr<PathingLocation>, buffer: u32) {
    e.call(LOAD_BYTES, &args![buffer, position(this), 0x0cu32]);
    let id = e.with_stack(4, |e, out| {
        e.call(LOAD_FORM_ID, &args![buffer, out.addr()]);
        e.mem.u32(out.addr())
    });
    let tes: u32 = e.global(TES_SINGLETON);
    let map = e.call(TES_NAV_MESH_INFO_MAP, &args![tes]).u32();
    let info = e.call(NAV_MESH_INFO_MAP_FIND, &args![map, id]).u32();
    set_word(e, this, NAV_MESH, info);
    for (offset, target) in [(CELL, TYPE_CELL), (WORLD_SPACE, TYPE_WORLD_SPACE)] {
        let form_id = e.call(LOAD_FORM, &args![buffer]).u32();
        let form = e.call(LOOK_UP_FORM, &args![form_id]).u32();
        let cast = e
            .call(
                DYNAMIC_CAST,
                &args![form, 0u32, TYPE_TES_FORM, target, 0u32],
            )
            .u32();
        set_word(e, this, offset, cast);
    }
    e.call(LOAD_BYTES, &args![buffer, this.addr() + CELL_KEY, 4u32]);
    e.call(LOAD_BYTES, &args![buffer, this.addr() + TRIANGLE, 2u32]);
    e.set(this, PathingLocation::usTriangle, NO_TRIANGLE);
    e.call(LOAD_BYTES, &args![buffer, this.addr() + FLAGS, 1u32]);
    let has_list = e.with_stack(4, |e, flag| {
        e.call(LOAD_BYTES, &args![buffer, flag.addr(), 1u32]);
        e.mem.u8(flag.addr()) != 0
    });
    if has_list {
        let world_space = word(e, this, WORLD_SPACE);
        if world_space != 0 {
            let key = word(e, this, CELL_KEY);
            let list = e
                .call(POTENTIAL_NAV_MESHES_FOR_KEY, &args![world_space, key])
                .u32();
            set_word(e, this, NAV_MESHES, list);
        } else {
            let cell = word(e, this, CELL);
            if cell != 0 {
                let list = e.call(POTENTIAL_NAV_MESHES_FOR_CELL, &args![cell]).u32();
                set_word(e, this, NAV_MESHES, list);
            }
        }
    }
}

// Translated from 006df180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs the scrap-heap `BSSimpleArray` (0x14 bytes): the plain array's
/// constructor (`005e0510`), then the scrap-array vtable (`0106c968`), the
/// thread's scrap heap (`MemoryManager::GetThreadScrapHeap`) at +0x10 and the
/// set-up call `006b3eb0(0, 0)`. Returns `this`. The compiler's unwinding
/// frame is not translated.
pub fn fn_006df180(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(ARRAY_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), SCRAP_ARRAY_VTABLE);
    let manager = e.call(MEMORY_MANAGER, &args![]).u32();
    let heap = e.call(THREAD_SCRAP_HEAP, &args![manager]).u32();
    e.mem.set_u32(this.addr() + 0x10, heap);
    e.call(SCRAP_ARRAY_SET_UP, &args![this, 0u32, 0u32]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00441110,
            path_location_get_world_space(Ptr<PathingLocation>) -> Ptr
        ),
        entry!(
            0x006dcbb0,
            path_location_path_location(Ptr<PathingLocation>) -> Ptr<PathingLocation>
        ),
        entry!(
            0x006dcc40,
            path_location_path_location_ov2(
                Ptr<PathingLocation>,
                Ptr<PathingLocation>,
            ) -> Ptr<PathingLocation>
        ),
        entry!(
            0x006dcce0,
            path_location_operator_assign(
                Ptr<PathingLocation>,
                Ptr<PathingLocation>,
            ) -> Ptr<PathingLocation>
        ),
        entry!(
            0x006dcd70,
            path_location_path_location_ov3(Ptr<PathingLocation>, Ptr) -> Ptr<PathingLocation>
        ),
        entry!(
            0x006dce10,
            path_location_path_location_ov4(Ptr<PathingLocation>, u32, Ptr) -> Ptr<PathingLocation>
        ),
        entry!(
            0x006dce60,
            path_location_path_location_ov5(Ptr<PathingLocation>, u32, u32) -> Ptr<PathingLocation>
        ),
        entry!(
            0x006dcea0,
            path_location_path_location_ov6(Ptr<PathingLocation>, u32, u32) -> Ptr<PathingLocation>
        ),
        entry!(
            0x006dcee0,
            path_location_path_location_ov7(
                Ptr<PathingLocation>,
                u32,
                u32,
                u32,
            ) -> Ptr<PathingLocation>
        ),
        entry!(
            0x006dcf20,
            fn_006dcf20(Ptr<PathingLocation>, u32, u32) -> Ptr<PathingLocation>
        ),
        entry!(
            0x006dcfe0,
            path_location_set_chosen_by_position(Ptr<PathingLocation>, u32)
        ),
        entry!(
            0x006dd010,
            fn_006dd010(Ptr<PathingLocation>, u32, u32, u16) -> Ptr<PathingLocation>
        ),
        entry!(
            0x006dd0d0,
            fn_006dd0d0(Ptr<PathingLocation>, u32, Ptr<PathingLocation>) -> Ptr<PathingLocation>
        ),
        entry!(
            0x006dd220,
            path_location_path_location_ov8(Ptr<PathingLocation>, u32) -> Ptr<PathingLocation>
        ),
        entry!(
            0x006dd280,
            path_location_setup_data(Ptr<PathingLocation>, u32, u32, u32)
        ),
        entry!(0x006dd3e0, fn_006dd3e0(Ptr<PathingLocation>, u32)),
        entry!(
            0x006dd460,
            path_location_set_location(Ptr<PathingLocation>, u32)
        ),
        entry!(0x006dd4d0, fn_006dd4d0(Ptr<PathingLocation>, u16)),
        entry!(
            0x006dd4f0,
            path_location_get_cell(Ptr<PathingLocation>) -> Ptr
        ),
        entry!(
            0x006dd540,
            path_location_get_best_nav_mesh_info(Ptr<PathingLocation>) -> u32
        ),
        entry!(0x006dd5a0, fn_006dd5a0(Ptr<PathingLocation>) -> u32),
        entry!(
            0x006dd5c0,
            path_location_get_nav_mesh_infos(Ptr<PathingLocation>, u32) -> bool
        ),
        entry!(
            0x006dd610,
            path_location_get_nav_mesh(Ptr<PathingLocation>, u32) -> bool
        ),
        entry!(
            0x006dd640,
            path_location_get_nav_mesh_and_triangle(Ptr<PathingLocation>, u32, u32) -> bool
        ),
        entry!(0x006dd6a0, fn_006dd6a0(Ptr<PathingLocation>) -> bool),
        entry!(
            0x006dd6f0,
            path_location_resolve_nav_mesh_info(Ptr<PathingLocation>, u32) -> bool
        ),
        entry!(
            0x006ddbe0,
            path_location_clear_triangle_data(Ptr<PathingLocation>)
        ),
        entry!(
            0x006ddc00,
            path_location_resolve_to_closest_nav_mesh_and_triangle(Ptr<PathingLocation>) -> bool
        ),
        entry!(0x006de250, fn_006de250(Ptr<PathingLocation>) -> bool),
        entry!(0x006de490, fn_006de490(Ptr<PathingLocation>) -> bool),
        entry!(
            0x006de790,
            path_location_print_debug_text(Ptr<PathingLocation>, u32, u32)
        ),
        entry!(
            0x006ded70,
            path_location_is_valid_location(Ptr<PathingLocation>, f32) -> bool
        ),
        entry!(
            0x006deeb0,
            fn_006deeb0(Ptr<PathingLocation>, Ptr<PathingLocation>, f32) -> bool
        ),
        entry!(
            0x006def40,
            path_location_save_game(Ptr<PathingLocation>, u32)
        ),
        entry!(
            0x006df010,
            path_location_load_game(Ptr<PathingLocation>, u32)
        ),
        entry!(0x006df180, fn_006df180(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    // -- Test doubles -------------------------------------------------------

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    /// A double that always answers `eax`.
    fn answer(e: &mut Engine, addr: u32, eax: u32) {
        e.register_double(addr, move |_, _| ret(eax));
    }

    /// A double that records its argument words and answers `eax`.
    fn recorder(e: &mut Engine, addr: u32, eax: u32) -> Rc<RefCell<Vec<Vec<u32>>>> {
        let log: Rc<RefCell<Vec<Vec<u32>>>> = Rc::default();
        let sink = log.clone();
        e.register_double(addr, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            ret(eax)
        });
        log
    }

    // `BSSimpleArray` of pointers: vtable, data, count, capacity.
    fn array(e: &mut Engine, items: &[u32]) -> u32 {
        let array = e.mem.alloc(0x14);
        let data = e.mem.alloc(0x100);
        e.mem.set_u32(array + 4, data);
        e.mem.set_u32(array + 8, items.len() as u32);
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(data + 4 * i as u32, *item);
        }
        array
    }

    fn items(e: &Engine, array: u32) -> Vec<u32> {
        let data = e.mem.u32(array + 4);
        (0..e.mem.u32(array + 8))
            .map(|i| e.mem.u32(data + 4 * i))
            .collect()
    }

    fn count_double(e: &mut Engine, a: &[u32]) -> Ret {
        ret(e.mem.u32(a[0] + 8))
    }
    fn element_double(e: &mut Engine, a: &[u32]) -> Ret {
        ret(e.mem.u32(a[0] + 4) + 4 * a[1])
    }
    fn first_element_double(e: &mut Engine, a: &[u32]) -> Ret {
        ret(e.mem.u32(a[0] + 4))
    }
    fn add_double(e: &mut Engine, a: &[u32]) -> Ret {
        let count = e.mem.u32(a[0] + 8);
        let data = e.mem.u32(a[0] + 4);
        let value = e.mem.u32(a[1]);
        e.mem.set_u32(data + 4 * count, value);
        e.mem.set_u32(a[0] + 8, count + 1);
        ret(count)
    }
    fn copy_to_double(e: &mut Engine, a: &[u32]) -> Ret {
        let source = items(e, a[0]);
        if e.mem.u32(a[1] + 4) == 0 {
            let data = e.mem.alloc(0x100);
            e.mem.set_u32(a[1] + 4, data);
        }
        let data = e.mem.u32(a[1] + 4);
        for (i, item) in source.iter().enumerate() {
            e.mem.set_u32(data + 4 * i as u32, *item);
        }
        e.mem.set_u32(a[1] + 8, source.len() as u32);
        Ret::default()
    }
    fn remove_double(e: &mut Engine, a: &[u32]) -> Ret {
        let mut all = items(e, a[0]);
        all.drain(a[1] as usize..(a[1] + a[2]) as usize);
        let data = e.mem.u32(a[0] + 4);
        for (i, item) in all.iter().enumerate() {
            e.mem.set_u32(data + 4 * i as u32, *item);
        }
        e.mem.set_u32(a[0] + 8, all.len() as u32);
        Ret::default()
    }
    fn empty_double(e: &mut Engine, a: &[u32]) -> Ret {
        e.mem.set_u32(a[0] + 8, 0);
        Ret::default()
    }
    fn find_double(e: &mut Engine, a: &[u32]) -> Ret {
        let value = e.mem.u32(a[1]);
        let found = items(e, a[0]).iter().position(|item| *item == value);
        ret(found.map_or(u32::MAX, |i| i as u32))
    }
    fn array_construct_double(e: &mut Engine, a: &[u32]) -> Ret {
        let data = e.mem.alloc(0x100);
        e.mem.set_u32(a[0] + 4, data);
        ret(a[0])
    }

    // `NiPoint3` arithmetic.
    fn point(e: &Engine, p: u32) -> [f32; 3] {
        [e.mem.f32(p), e.mem.f32(p + 4), e.mem.f32(p + 8)]
    }
    fn put_point(e: &mut Engine, p: u32, v: [f32; 3]) {
        for (i, value) in v.iter().enumerate() {
            e.mem.set_f32(p + 4 * i as u32, *value);
        }
    }
    fn subtract_double(e: &mut Engine, a: &[u32]) -> Ret {
        let (l, r) = (point(e, a[0]), point(e, a[2]));
        put_point(e, a[1], [l[0] - r[0], l[1] - r[1], l[2] - r[2]]);
        ret(a[1])
    }
    fn add_point_double(e: &mut Engine, a: &[u32]) -> Ret {
        let (l, r) = (point(e, a[0]), point(e, a[2]));
        put_point(e, a[1], [l[0] + r[0], l[1] + r[1], l[2] + r[2]]);
        ret(a[1])
    }
    fn scale_double(e: &mut Engine, a: &[u32]) -> Ret {
        let (v, f) = (point(e, a[0]), f32::from_bits(a[2]));
        put_point(e, a[1], [v[0] * f, v[1] * f, v[2] * f]);
        ret(a[1])
    }
    fn squared_length_double(e: &mut Engine, a: &[u32]) -> Ret {
        let v = point(e, a[0]);
        st0((v[0] * v[0] + v[1] * v[1] + v[2] * v[2]) as f64)
    }
    fn squared_length_2d_double(e: &mut Engine, a: &[u32]) -> Ret {
        let v = point(e, a[0]);
        st0((v[0] * v[0] + v[1] * v[1]) as f64)
    }

    // Navmesh info (0x5c bytes): flags at +8, mesh at +0x54, bounds at +0x58.
    fn info(e: &mut Engine, flags: u32, mesh: u32, bounds: u32) -> u32 {
        let info = e.mem.alloc(0x5c);
        e.mem.set_u32(info + 8, flags);
        e.mem.set_u32(info + 0x54, mesh);
        e.mem.set_u32(info + 0x58, bounds);
        info
    }
    // Fake navmesh (0x40 bytes), what the doubles answer:
    // +0x00 triangle for a position, +0x04 closest triangle, +0x08 exact flag,
    // +0x0c closest-search result, +0x10 centre, +0x20 edge found, +0x24 edge point.
    fn mesh(e: &mut Engine) -> u32 {
        e.mem.alloc(0x40)
    }
    // Fake bounds: +0 contains the position, +4 offset to it.
    fn bounds(e: &mut Engine, contains: bool, offset: [f32; 3]) -> u32 {
        let bounds = e.mem.alloc(0x10);
        e.mem.set_u32(bounds, contains as u32);
        put_point(e, bounds + 4, offset);
        bounds
    }

    fn deref_double(e: &mut Engine, a: &[u32]) -> Ret {
        ret(e.mem.u32(a[0]))
    }
    fn zero_slot_double(e: &mut Engine, a: &[u32]) -> Ret {
        e.mem.set_u32(a[0], 0);
        ret(a[0])
    }
    fn load_mesh_double(e: &mut Engine, a: &[u32]) -> Ret {
        let mesh = e.mem.u32(a[0] + 0x54);
        e.mem.set_u32(a[1], mesh);
        ret((mesh != 0) as u32)
    }
    fn is_disabled_double(e: &mut Engine, a: &[u32]) -> Ret {
        ret((e.mem.u32(a[0] + 8) & 0x10 != 0) as u32)
    }
    fn has_mesh_double(e: &mut Engine, a: &[u32]) -> Ret {
        ret((e.mem.u32(a[0] + 0x54) != 0) as u32)
    }
    fn bounds_of_double(e: &mut Engine, a: &[u32]) -> Ret {
        ret(e.mem.u32(a[0] + 0x58))
    }
    fn bounds_contain_double(e: &mut Engine, a: &[u32]) -> Ret {
        ret((e.mem.u32(a[0]) != 0) as u32)
    }
    fn bounds_offset_double(e: &mut Engine, a: &[u32]) -> Ret {
        let offset = point(e, a[0] + 4);
        put_point(e, a[1], offset);
        ret(a[1])
    }
    fn find_triangle_double(e: &mut Engine, a: &[u32]) -> Ret {
        ret(e.mem.u32(a[0]))
    }
    fn find_closest_double(e: &mut Engine, a: &[u32]) -> Ret {
        let triangle = e.mem.u32(a[0] + 4) as u16;
        let exact = e.mem.u32(a[0] + 8) as u8;
        e.mem.set_u16(a[2], triangle);
        e.mem.set_u8(a[3], exact);
        ret(e.mem.u32(a[0] + 0x0c))
    }
    fn centre_double(e: &mut Engine, a: &[u32]) -> Ret {
        let centre = point(e, a[0] + 0x10);
        put_point(e, a[1], centre);
        ret(a[1])
    }
    fn edge_double(e: &mut Engine, a: &[u32]) -> Ret {
        let found = e.mem.u32(a[0] + 0x20);
        e.mem.set_u32(a[1], found);
        let edge_point = point(e, a[0] + 0x24);
        put_point(e, a[1] + 8, edge_point);
        ret(a[1])
    }
    fn ftol_double(_: &mut Engine, a: &[u32]) -> Ret {
        ret(f64::from_bits(a[0] as u64 | (a[1] as u64) << 32) as i32 as u32)
    }

    /// An engine whose data the functions read is mapped, with every callee
    /// outside this file doubled (answering 0 unless stated).
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_2000u32,
            0x0101_6000,
            0x0101_7000,
            0x0101_e000,
            0x0104_e000,
            0x0104_f000,
            0x0106_b000,
            0x0106_c000,
            0x011d_e000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(FLOAT_MAX, f32::MAX);
        e.set_global(ZERO, 0.0f64);
        e.set_global(FAR_DISTANCE, 16384.0f64);
        e.set_global(GROUND_MARGIN, 25.0f64);
        e.set_global(MAX_HEIGHT_ABOVE, 180.0f64);
        e.set_global(MIN_HEIGHT_BELOW, -64.0f64);
        e.set_global(CENTER_WEIGHT, 0.1f32);
        e.set_global(EDGE_WEIGHT, 0.9f32);
        for addr in [
            REFERENCE_CELL,
            REFERENCE_WORLD_SPACE,
            TES_INTERIOR_CELL,
            TES_WORLD_SPACE,
            WORLD_LOCATION_CELL_OR_WORLD,
            WORLD_SPACE_CELL_FROM_KEY,
            WORLD_SPACE_CELL_FROM_COORD,
            POTENTIAL_NAV_MESHES_FOR_CELL,
            POTENTIAL_NAV_MESHES_FOR_KEY,
            CELL_WORLD_SPACE,
            CELL_DATA_X,
            CELL_DATA_Y,
            CELL_KEY_FROM_GRID,
            INFO_WORLD_SPACE,
            INFO_CELL,
            FORM_TEST_FIRST,
            FORM_TEST_SECOND,
            CELL_IS_KIND,
            TES_GROUND_HEIGHT,
            LIST_TEST,
            FLAGS_BLOCK_RESOLVING,
            CHECK_LINE_OF_SIGHT,
            GET_POSITION,
            FORMAT_TEXT,
            SAVE_BYTES,
            SAVE_FORM_ID,
            SAVE_FORM,
            LOAD_BYTES,
            LOAD_FORM_ID,
            LOAD_FORM,
            TES_NAV_MESH_INFO_MAP,
            NAV_MESH_INFO_MAP_FIND,
            LOOK_UP_FORM,
            DYNAMIC_CAST,
            NI_POINTER_RELEASE,
            SCRAP_ARRAY_DESTROY,
            ARRAY_DESTROY,
            SCRAP_ARRAY_SET_UP,
            MEMORY_MANAGER,
            THREAD_SCRAP_HEAP,
        ] {
            answer(&mut e, addr, 0);
        }
        answer(&mut e, CELL_TEST, 1);
        answer(&mut e, WORLD_SPACE_KEY_FOR_COORD, 0x42);
        e.register(NI_POINT3_CONSTRUCT, |_, a| ret(a[0]));
        e.register(NI_POINTER_CONSTRUCT, zero_slot_double);
        e.register(NI_POINTER_GET, deref_double);
        e.register(NI_POINTER_HAS_MESH, deref_double);
        e.register(INFO_LOAD_NAV_MESH, load_mesh_double);
        e.register(INFO_IS_DISABLED, is_disabled_double);
        e.register(INFO_HAS_NAV_MESH, has_mesh_double);
        e.register(INFO_BOUNDS, bounds_of_double);
        e.register(BOUNDS_CONTAIN, bounds_contain_double);
        e.register(BOUNDS_OFFSET_TO, bounds_offset_double);
        e.register(NAV_MESH_FIND_TRIANGLE, find_triangle_double);
        e.register(NAV_MESH_FIND_CLOSEST_TRIANGLE, find_closest_double);
        e.register(NAV_MESH_GET_CENTER, centre_double);
        e.register(NAV_MESH_FIND_INTERSECTING_EDGE, edge_double);
        e.register(POINT_SUBTRACT, subtract_double);
        e.register(POINT_ADD, add_point_double);
        e.register(POINT_SCALE, scale_double);
        e.register(POINT_SQUARED_LENGTH, squared_length_double);
        e.register(POINT_SQUARED_LENGTH_2D, squared_length_2d_double);
        e.register(FTOL, ftol_double);
        e.register(ARRAY_COUNT, count_double);
        e.register(ARRAY_ELEMENT, element_double);
        e.register(LOCAL_ARRAY_ELEMENT, element_double);
        e.register(ARRAY_ADD, add_double);
        e.register(ARRAY_COPY_TO, copy_to_double);
        e.register(ARRAY_REMOVE, remove_double);
        e.register(ARRAY_EMPTY, empty_double);
        e.register(ARRAY_FIND, find_double);
        e.register(ARRAY_CONSTRUCT, array_construct_double);
        e.register(ARRAY_FIRST, first_element_double);
        e
    }

    fn location(e: &mut Engine) -> Ptr<PathingLocation> {
        let location: Ptr<PathingLocation> = e.new_object();
        e.set(location, PathingLocation::usTriangle, NO_TRIANGLE);
        location
    }

    fn set_position(e: &mut Engine, l: Ptr<PathingLocation>, v: [f32; 3]) {
        put_point(e, position(l), v);
    }

    /// An object whose virtual slot at byte offset `slot` is the function at
    /// `target`.
    fn object_with_slot(e: &mut Engine, slot: u32, target: u32) -> u32 {
        let vtable = e.mem.alloc(0x400);
        e.mem.set_u32(vtable + slot, target);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        object
    }

    // -- Tests: construction ------------------------------------------------

    #[test]
    fn get_world_space_reads_the_field() {
        let mut e = engine();
        let l = location(&mut e);
        set_word(&mut e, l, WORLD_SPACE, 0x1234);
        assert_eq!(e.call(0x0044_1110, &args![l]).u32(), 0x1234);
    }

    #[test]
    fn default_constructor_clears_everything_and_puts_the_point_at_float_max() {
        let mut e = engine();
        let l = location(&mut e);
        for offset in (0x10..0x28).step_by(4) {
            set_word(&mut e, l, offset, 0x5555_5555);
        }
        assert_eq!(e.call(0x006d_cbb0, &args![l]).u32(), l.addr());
        assert_eq!(word(&e, l, 0), VTABLE);
        assert_eq!(point(&e, position(l)), [f32::MAX; 3]);
        assert_eq!(word(&e, l, NAV_MESH), 0);
        assert_eq!(word(&e, l, NAV_MESHES), 0);
        assert_eq!(word(&e, l, CELL), 0);
        assert_eq!(word(&e, l, WORLD_SPACE), 0);
        assert_eq!(word(&e, l, CELL_KEY), NO_CELL_KEY);
        assert_eq!(e.get(l, PathingLocation::usTriangle), NO_TRIANGLE);
        assert_eq!(e.get(l, PathingLocation::uiFlags), 0);
    }

    fn filled_location(e: &mut Engine) -> Ptr<PathingLocation> {
        let l = location(e);
        set_position(e, l, [1.0, 2.0, 3.0]);
        for (i, offset) in (0x10..=0x20).step_by(4).enumerate() {
            set_word(e, l, offset, 0x100 + i as u32);
        }
        e.set(l, PathingLocation::usTriangle, 9);
        e.set(l, PathingLocation::uiFlags, 3);
        l
    }

    #[test]
    fn copy_constructor_copies_every_field_and_sets_the_vtable() {
        let mut e = engine();
        let source = filled_location(&mut e);
        let l = location(&mut e);
        e.call(0x006d_cc40, &args![l, source]);
        assert_eq!(word(&e, l, 0), VTABLE);
        assert_eq!(point(&e, position(l)), [1.0, 2.0, 3.0]);
        for (i, offset) in (0x10..=0x20).step_by(4).enumerate() {
            assert_eq!(word(&e, l, offset), 0x100 + i as u32);
        }
        assert_eq!(e.get(l, PathingLocation::usTriangle), 9);
        assert_eq!(e.get(l, PathingLocation::uiFlags), 3);
    }

    #[test]
    fn assignment_copies_every_field_but_the_vtable_and_returns_this() {
        let mut e = engine();
        let source = filled_location(&mut e);
        let l = location(&mut e);
        e.mem.set_u32(l.addr(), 0x7777);
        let result = e.call(0x006d_cce0, &args![l, source]);
        assert_eq!(result.u32(), l.addr());
        assert_eq!(word(&e, l, 0), 0x7777);
        assert_eq!(point(&e, position(l)), [1.0, 2.0, 3.0]);
        assert_eq!(word(&e, l, CELL_KEY), 0x104);
        assert_eq!(e.get(l, PathingLocation::usTriangle), 9);
        assert_eq!(e.get(l, PathingLocation::uiFlags), 3);
    }

    /// A reference whose position slot answers the address of `at`.
    fn reference_at(e: &mut Engine, at: [f32; 3]) -> u32 {
        let point_block = e.mem.alloc(12);
        put_point(e, point_block, at);
        answer(e, 0x0060_0000, point_block);
        object_with_slot(e, SLOT_POSITION, 0x0060_0000)
    }

    #[test]
    fn constructor_from_a_reference_uses_its_cell_and_position() {
        let mut e = engine();
        let reference = reference_at(&mut e, [5.0, 6.0, 7.0]);
        answer(&mut e, REFERENCE_CELL, 0xce11);
        e.set_global(PLAYER_CHARACTER, 0u32);
        let l = location(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x006d_cd70, &args![l, reference]);
        assert_eq!(point(&e, position(l)), [5.0, 6.0, 7.0]);
        assert_eq!(word(&e, l, CELL), 0xce11);
        let calls: Vec<u32> = e.call_log.take().unwrap().iter().map(|c| c.0).collect();
        assert!(!calls.contains(&TES_INTERIOR_CELL));
    }

    #[test]
    fn constructor_from_the_player_without_a_cell_asks_tes_for_its_cell_then_world() {
        let mut e = engine();
        let player = reference_at(&mut e, [1.0, 1.0, 1.0]);
        e.set_global(PLAYER_CHARACTER, player);
        e.set_global(TES_SINGLETON, 0xfe5u32);
        let interior = recorder(&mut e, TES_INTERIOR_CELL, 0xce11);
        let l = location(&mut e);
        e.call(0x006d_cd70, &args![l, player]);
        assert_eq!(interior.borrow()[0], vec![0xfe5]);
        assert_eq!(word(&e, l, CELL), 0xce11);
        // Without an interior cell the world space comes from TES.
        answer(&mut e, TES_INTERIOR_CELL, 0);
        let world = recorder(&mut e, TES_WORLD_SPACE, 0x3a5);
        let l = location(&mut e);
        e.call(0x006d_cd70, &args![l, player]);
        assert_eq!(world.borrow()[0], vec![0xfe5]);
        assert_eq!(word(&e, l, WORLD_SPACE), 0x3a5);
        assert_eq!(word(&e, l, CELL_KEY), 0x42);
    }

    #[test]
    fn constructor_from_a_point_and_a_reference_uses_the_reference_cell_and_world() {
        let mut e = engine();
        let reference = e.mem.alloc(16);
        answer(&mut e, REFERENCE_CELL, 0xce11);
        answer(&mut e, REFERENCE_WORLD_SPACE, 0);
        let at = e.mem.alloc(12);
        put_point(&mut e, at, [4.0, 5.0, 6.0]);
        let l = location(&mut e);
        e.call(0x006d_ce10, &args![l, at, reference]);
        assert_eq!(point(&e, position(l)), [4.0, 5.0, 6.0]);
        assert_eq!(word(&e, l, CELL), 0xce11);
        assert_eq!(word(&e, l, 0), VTABLE);
    }

    #[test]
    fn constructors_from_a_point_with_a_cell_or_a_world_space() {
        let mut e = engine();
        let at = e.mem.alloc(12);
        put_point(&mut e, at, [4.0, 5.0, 6.0]);
        let with_cell = location(&mut e);
        e.call(0x006d_ce60, &args![with_cell, at, 0xce11u32]);
        assert_eq!(word(&e, with_cell, CELL), 0xce11);
        assert_eq!(word(&e, with_cell, WORLD_SPACE), 0);
        let with_world = location(&mut e);
        e.call(0x006d_cea0, &args![with_world, at, 0x3a5u32]);
        assert_eq!(word(&e, with_world, WORLD_SPACE), 0x3a5);
        assert_eq!(word(&e, with_world, CELL), 0);
        let with_both = location(&mut e);
        e.call(0x006d_cee0, &args![with_both, at, 0xce11u32, 0x3a5u32]);
        assert_eq!(word(&e, with_both, WORLD_SPACE), 0x3a5);
        assert_eq!(word(&e, with_both, CELL_KEY), 0x42);
        assert_eq!(word(&e, with_both, 0), VTABLE);
    }

    #[test]
    fn constructor_in_a_navmesh_info_takes_its_world_space_and_key() {
        let mut e = engine();
        let navmesh_info = info(&mut e, 0, 0, 0);
        e.mem.set_u32(navmesh_info + 0x0c, 0xabc);
        answer(&mut e, INFO_WORLD_SPACE, 0x3a5);
        let at = e.mem.alloc(12);
        put_point(&mut e, at, [4.0, 5.0, 6.0]);
        let l = location(&mut e);
        e.set(l, PathingLocation::uiFlags, 2);
        e.set(l, PathingLocation::usTriangle, 3);
        e.call(0x006d_cf20, &args![l, at, navmesh_info]);
        assert_eq!(word(&e, l, NAV_MESH), navmesh_info);
        assert_eq!(word(&e, l, NAV_MESHES), 0);
        assert_eq!(word(&e, l, WORLD_SPACE), 0x3a5);
        assert_eq!(word(&e, l, CELL), 0);
        assert_eq!(word(&e, l, CELL_KEY), 0xabc);
        // Flags were cleared, then bit 0 set.
        assert_eq!(e.get(l, PathingLocation::uiFlags), 1);
        assert_eq!(e.get(l, PathingLocation::usTriangle), NO_TRIANGLE);
        assert_eq!(point(&e, position(l)), [4.0, 5.0, 6.0]);
    }

    #[test]
    fn constructor_in_a_navmesh_info_without_world_space_takes_its_cell() {
        let mut e = engine();
        let navmesh_info = info(&mut e, 0, 0, 0);
        answer(&mut e, INFO_CELL, 0xce11);
        let at = e.mem.alloc(12);
        let l = location(&mut e);
        e.call(0x006d_cf20, &args![l, at, navmesh_info]);
        assert_eq!(word(&e, l, CELL), 0xce11);
        assert_eq!(word(&e, l, WORLD_SPACE), 0);
        assert_eq!(word(&e, l, CELL_KEY), NO_CELL_KEY);
    }

    #[test]
    fn set_chosen_by_position_replaces_bit_zero_only() {
        let mut e = engine();
        let l = location(&mut e);
        e.set(l, PathingLocation::uiFlags, 0xff);
        e.call(0x006d_cfe0, &args![l, 0u32]);
        assert_eq!(e.get(l, PathingLocation::uiFlags), 0xfe);
        e.call(0x006d_cfe0, &args![l, 1u32]);
        assert_eq!(e.get(l, PathingLocation::uiFlags), 0xff);
        e.set(l, PathingLocation::uiFlags, 2);
        e.call(0x006d_cfe0, &args![l, 1u32]);
        assert_eq!(e.get(l, PathingLocation::uiFlags), 3);
    }

    #[test]
    fn constructor_in_a_navmesh_info_with_a_triangle_keeps_the_triangle() {
        let mut e = engine();
        let navmesh_info = info(&mut e, 0, 0, 0);
        let at = e.mem.alloc(12);
        let l = location(&mut e);
        e.call(0x006d_d010, &args![l, at, navmesh_info, 0x15u32]);
        assert_eq!(word(&e, l, NAV_MESH), navmesh_info);
        assert_eq!(e.get(l, PathingLocation::usTriangle), 0x15);
        assert_eq!(e.get(l, PathingLocation::uiFlags), 1);
        assert_eq!(word(&e, l, 0), VTABLE);
    }

    #[test]
    fn constructor_from_another_location_adopts_its_navmesh_when_it_has_a_triangle() {
        let mut e = engine();
        let navmesh = mesh(&mut e);
        e.mem.set_u32(navmesh, 7); // the triangle for any position
        let navmesh_info = info(&mut e, 0, navmesh, 0);
        let other = location(&mut e);
        set_word(&mut e, other, NAV_MESH, navmesh_info);
        set_word(&mut e, other, CELL, 0xce11);
        set_word(&mut e, other, WORLD_SPACE, 0x3a5);
        set_word(&mut e, other, CELL_KEY, 0x99);
        let at = e.mem.alloc(12);
        put_point(&mut e, at, [8.0, 9.0, 10.0]);
        let l = location(&mut e);
        e.call(0x006d_d0d0, &args![l, at, other]);
        assert_eq!(word(&e, l, NAV_MESH), navmesh_info);
        assert_eq!(word(&e, l, NAV_MESHES), 0);
        assert_eq!(word(&e, l, CELL), 0xce11);
        assert_eq!(word(&e, l, WORLD_SPACE), 0x3a5);
        assert_eq!(word(&e, l, CELL_KEY), 0x99);
        assert_eq!(e.get(l, PathingLocation::usTriangle), 7);
        assert_eq!(point(&e, position(l)), [8.0, 9.0, 10.0]);
    }

    #[test]
    fn constructor_from_another_location_without_a_triangle_sets_up_from_its_cell() {
        let mut e = engine();
        let navmesh = mesh(&mut e);
        e.mem.set_u32(navmesh, 0xffff); // no triangle for the position
        let navmesh_info = info(&mut e, 0, navmesh, 0);
        let other = location(&mut e);
        set_word(&mut e, other, NAV_MESH, navmesh_info);
        set_word(&mut e, other, CELL, 0xce11);
        let at = e.mem.alloc(12);
        let l = location(&mut e);
        e.call(0x006d_d0d0, &args![l, at, other]);
        // `SetupData(point, other's cell, other's world space)`.
        assert_eq!(word(&e, l, CELL), 0xce11);
        assert_eq!(word(&e, l, WORLD_SPACE), 0);
        assert_eq!(word(&e, l, NAV_MESH), 0);
        assert_eq!(e.get(l, PathingLocation::usTriangle), NO_TRIANGLE);
    }

    #[test]
    fn constructor_from_another_location_without_a_navmesh_sets_up_from_its_cell() {
        let mut e = engine();
        let other = location(&mut e);
        set_word(&mut e, other, WORLD_SPACE, 0x3a5);
        let at = e.mem.alloc(12);
        let l = location(&mut e);
        e.call(0x006d_d0d0, &args![l, at, other]);
        assert_eq!(word(&e, l, WORLD_SPACE), 0x3a5);
        assert_eq!(word(&e, l, CELL_KEY), 0x42);
    }

    #[test]
    fn constructor_from_a_world_location_uses_its_cell_and_world() {
        let mut e = engine();
        let world_location = e.mem.alloc(16);
        put_point(&mut e, world_location, [3.0, 2.0, 1.0]);
        e.register(WORLD_LOCATION_CELL_OR_WORLD, |e, a| {
            e.mem.set_u32(a[1], 0xce11);
            e.mem.set_u32(a[2], 0);
            Ret::default()
        });
        let l = location(&mut e);
        e.call(0x006d_d220, &args![l, world_location]);
        assert_eq!(point(&e, position(l)), [3.0, 2.0, 1.0]);
        assert_eq!(word(&e, l, CELL), 0xce11);
        assert_eq!(word(&e, l, 0), VTABLE);
    }

    // -- Tests: SetupData and small queries --------------------------------

    #[test]
    fn setup_data_with_a_world_space_takes_the_key_and_a_single_candidate() {
        let mut e = engine();
        let candidate = info(&mut e, 0, 0, 0);
        let list = array(&mut e, &[candidate]);
        let potential = recorder(&mut e, POTENTIAL_NAV_MESHES_FOR_KEY, list);
        let at = e.mem.alloc(12);
        put_point(&mut e, at, [1.0, 2.0, 3.0]);
        let l = location(&mut e);
        e.set(l, PathingLocation::uiFlags, 0xf0);
        set_word(&mut e, l, CELL, 0xce11);
        e.call(0x006d_d280, &args![l, at, 0xce11u32, 0x3a5u32]);
        assert_eq!(point(&e, position(l)), [1.0, 2.0, 3.0]);
        assert_eq!(word(&e, l, WORLD_SPACE), 0x3a5);
        assert_eq!(word(&e, l, CELL_KEY), 0x42);
        assert_eq!(word(&e, l, CELL), 0);
        assert_eq!(potential.borrow()[0], vec![0x3a5, 0x42]);
        assert_eq!(word(&e, l, NAV_MESH), candidate);
        assert_eq!(word(&e, l, NAV_MESHES), 0);
        // The flags were cleared (to 0), then bit 0 set.
        assert_eq!(e.get(l, PathingLocation::uiFlags), 1);
        assert_eq!(e.get(l, PathingLocation::usTriangle), NO_TRIANGLE);
    }

    #[test]
    fn setup_data_keeps_a_list_of_several_candidates() {
        let mut e = engine();
        let (a, b) = (info(&mut e, 0, 0, 0), info(&mut e, 0, 0, 0));
        let list = array(&mut e, &[a, b]);
        answer(&mut e, POTENTIAL_NAV_MESHES_FOR_KEY, list);
        let at = e.mem.alloc(12);
        let l = location(&mut e);
        set_word(&mut e, l, NAV_MESH, 0x9999);
        e.call(0x006d_d280, &args![l, at, 0u32, 0x3a5u32]);
        assert_eq!(word(&e, l, NAV_MESH), 0);
        assert_eq!(word(&e, l, NAV_MESHES), list);
        assert_eq!(e.get(l, PathingLocation::uiFlags), 0);
    }

    #[test]
    fn setup_data_for_a_cell_that_the_test_accepts_stores_the_cell() {
        let mut e = engine();
        let potential = recorder(&mut e, POTENTIAL_NAV_MESHES_FOR_CELL, 0);
        let at = e.mem.alloc(12);
        let l = location(&mut e);
        e.call(0x006d_d280, &args![l, at, 0xce11u32, 0u32]);
        assert_eq!(potential.borrow()[0], vec![0xce11]);
        assert_eq!(word(&e, l, CELL), 0xce11);
        assert_eq!(word(&e, l, WORLD_SPACE), 0);
        assert_eq!(word(&e, l, CELL_KEY), NO_CELL_KEY);
        assert_eq!(word(&e, l, NAV_MESH), 0);
    }

    #[test]
    fn setup_data_for_an_outdoor_cell_stores_its_world_space_and_grid_key() {
        let mut e = engine();
        answer(&mut e, CELL_TEST, 0);
        answer(&mut e, CELL_WORLD_SPACE, 0x3a5);
        answer(&mut e, CELL_DATA_X, 3);
        answer(&mut e, CELL_DATA_Y, 4);
        let grid = recorder(&mut e, CELL_KEY_FROM_GRID, 0x77);
        let at = e.mem.alloc(12);
        let l = location(&mut e);
        e.call(0x006d_d280, &args![l, at, 0xce11u32, 0u32]);
        assert_eq!(grid.borrow()[0], vec![3, 4]);
        assert_eq!(word(&e, l, WORLD_SPACE), 0x3a5);
        assert_eq!(word(&e, l, CELL_KEY), 0x77);
        assert_eq!(word(&e, l, CELL), 0);
    }

    #[test]
    fn setup_data_without_a_cell_stores_no_cell() {
        let mut e = engine();
        let at = e.mem.alloc(12);
        let l = location(&mut e);
        set_word(&mut e, l, CELL, 5);
        e.call(0x006d_d280, &args![l, at, 0u32, 0u32]);
        assert_eq!(word(&e, l, CELL), 0);
        assert_eq!(word(&e, l, CELL_KEY), NO_CELL_KEY);
    }

    #[test]
    fn adopting_a_navmesh_info_resets_the_triangle() {
        let mut e = engine();
        let navmesh_info = info(&mut e, 0, 0, 0);
        answer(&mut e, INFO_CELL, 0xce11);
        let l = location(&mut e);
        e.set(l, PathingLocation::usTriangle, 4);
        set_word(&mut e, l, NAV_MESHES, 0x1234);
        e.call(0x006d_d3e0, &args![l, navmesh_info]);
        assert_eq!(word(&e, l, NAV_MESH), navmesh_info);
        assert_eq!(word(&e, l, NAV_MESHES), 0);
        assert_eq!(word(&e, l, CELL), 0xce11);
        assert_eq!(e.get(l, PathingLocation::usTriangle), NO_TRIANGLE);
        assert_eq!(e.get(l, PathingLocation::uiFlags), 1);
    }

    #[test]
    fn set_location_moves_and_sets_up_again_only_when_the_cell_key_changed() {
        let mut e = engine();
        let at = e.mem.alloc(12);
        put_point(&mut e, at, [7.0, 8.0, 9.0]);
        let l = location(&mut e);
        e.set(l, PathingLocation::usTriangle, 4);
        // Without a world space nothing but the move happens.
        e.call(0x006d_d460, &args![l, at]);
        assert_eq!(point(&e, position(l)), [7.0, 8.0, 9.0]);
        assert_eq!(e.get(l, PathingLocation::usTriangle), NO_TRIANGLE);
        // The same key: no set-up.
        set_word(&mut e, l, WORLD_SPACE, 0x3a5);
        set_word(&mut e, l, CELL_KEY, 0x42);
        let potential = recorder(&mut e, POTENTIAL_NAV_MESHES_FOR_KEY, 0);
        e.call(0x006d_d460, &args![l, at]);
        assert!(potential.borrow().is_empty());
        // Another key: `SetupData(point, no cell, world space)`.
        set_word(&mut e, l, CELL_KEY, 0x43);
        e.call(0x006d_d460, &args![l, at]);
        assert_eq!(potential.borrow()[0], vec![0x3a5, 0x42]);
        assert_eq!(word(&e, l, CELL_KEY), 0x42);
    }

    #[test]
    fn set_triangle_stores_the_triangle() {
        let mut e = engine();
        let l = location(&mut e);
        e.call(0x006d_d4d0, &args![l, 0x1234u32]);
        assert_eq!(e.get(l, PathingLocation::usTriangle), 0x1234);
    }

    #[test]
    fn get_cell_prefers_the_stored_cell_then_the_key_lookup() {
        let mut e = engine();
        let l = location(&mut e);
        assert_eq!(e.call(0x006d_d4f0, &args![l]).u32(), 0);
        set_word(&mut e, l, CELL, 0xce11);
        assert_eq!(e.call(0x006d_d4f0, &args![l]).u32(), 0xce11);
        set_word(&mut e, l, CELL, 0);
        set_word(&mut e, l, WORLD_SPACE, 0x3a5);
        set_word(&mut e, l, CELL_KEY, 0x42);
        let lookup = recorder(&mut e, WORLD_SPACE_CELL_FROM_KEY, 0xce22);
        assert_eq!(e.call(0x006d_d4f0, &args![l]).u32(), 0xce22);
        assert_eq!(lookup.borrow()[0], vec![0x3a5, 0x42]);
        answer(&mut e, WORLD_SPACE_CELL_FROM_KEY, 0);
        assert_eq!(e.call(0x006d_d4f0, &args![l]).u32(), 0);
    }

    #[test]
    fn best_navmesh_info_is_the_info_or_the_first_candidate_when_chosen_by_position() {
        let mut e = engine();
        let l = location(&mut e);
        assert_eq!(e.call(0x006d_d540, &args![l]).u32(), 0);
        let (a, b) = (info(&mut e, 0, 0, 0), info(&mut e, 0, 0, 0));
        let list = array(&mut e, &[a, b]);
        set_word(&mut e, l, NAV_MESHES, list);
        // Not chosen by position: nothing.
        assert_eq!(e.call(0x006d_d540, &args![l]).u32(), 0);
        e.set(l, PathingLocation::uiFlags, 1);
        assert_eq!(e.call(0x006d_d540, &args![l]).u32(), a);
        set_word(&mut e, l, NAV_MESH, b);
        assert_eq!(e.call(0x006d_d540, &args![l]).u32(), b);
    }

    #[test]
    fn chosen_by_position_reads_bit_zero() {
        let mut e = engine();
        let l = location(&mut e);
        e.set(l, PathingLocation::uiFlags, 3);
        assert_eq!(e.call(0x006d_d5a0, &args![l]).u32(), 1);
        e.set(l, PathingLocation::uiFlags, 2);
        assert_eq!(e.call(0x006d_d5a0, &args![l]).u32(), 0);
    }

    #[test]
    fn get_nav_mesh_infos_adds_the_info_or_copies_the_list() {
        let mut e = engine();
        let out = array(&mut e, &[]);
        let l = location(&mut e);
        assert!(!e.call(0x006d_d5c0, &args![l, out]).bool());
        let (a, b) = (info(&mut e, 0, 0, 0), info(&mut e, 0, 0, 0));
        let list = array(&mut e, &[a, b]);
        set_word(&mut e, l, NAV_MESHES, list);
        assert!(e.call(0x006d_d5c0, &args![l, out]).bool());
        assert_eq!(items(&e, out), vec![a, b]);
        let out = array(&mut e, &[]);
        set_word(&mut e, l, NAV_MESH, b);
        assert!(e.call(0x006d_d5c0, &args![l, out]).bool());
        assert_eq!(items(&e, out), vec![b]);
    }

    #[test]
    fn get_nav_mesh_loads_the_navmesh_of_the_info() {
        let mut e = engine();
        let l = location(&mut e);
        let out = e.mem.alloc(4);
        assert!(!e.call(0x006d_d610, &args![l, out]).bool());
        let navmesh = mesh(&mut e);
        let navmesh_info = info(&mut e, 0, navmesh, 0);
        set_word(&mut e, l, NAV_MESH, navmesh_info);
        assert!(e.call(0x006d_d610, &args![l, out]).bool());
        assert_eq!(e.mem.u32(out), navmesh);
        // A navmesh that does not load.
        let unloaded = info(&mut e, 0, 0, 0);
        set_word(&mut e, l, NAV_MESH, unloaded);
        assert!(!e.call(0x006d_d610, &args![l, out]).bool());
    }

    #[test]
    fn get_nav_mesh_and_triangle_needs_a_triangle() {
        let mut e = engine();
        let l = location(&mut e);
        let (out, triangle) = (e.mem.alloc(4), e.mem.alloc(4));
        assert!(!e.call(0x006d_d640, &args![l, out, triangle]).bool());
        let navmesh = mesh(&mut e);
        let navmesh_info = info(&mut e, 0, navmesh, 0);
        set_word(&mut e, l, NAV_MESH, navmesh_info);
        assert!(!e.call(0x006d_d640, &args![l, out, triangle]).bool());
        e.set(l, PathingLocation::usTriangle, 0x21);
        assert!(e.call(0x006d_d640, &args![l, out, triangle]).bool());
        assert_eq!(e.mem.u16(triangle), 0x21);
        assert_eq!(e.mem.u32(out), navmesh);
        // The navmesh does not load: the triangle is not written.
        let unloaded = info(&mut e, 0, 0, 0);
        set_word(&mut e, l, NAV_MESH, unloaded);
        e.mem.set_u16(triangle, 0);
        assert!(!e.call(0x006d_d640, &args![l, out, triangle]).bool());
        assert_eq!(e.mem.u16(triangle), 0);
    }

    #[test]
    fn has_loaded_navmesh_and_triangle_needs_all_three() {
        let mut e = engine();
        let l = location(&mut e);
        assert!(!e.call(0x006d_d6a0, &args![l]).bool());
        let unloaded = info(&mut e, 0, 0, 0);
        set_word(&mut e, l, NAV_MESH, unloaded);
        e.set(l, PathingLocation::usTriangle, 3);
        assert!(!e.call(0x006d_d6a0, &args![l]).bool());
        let navmesh = mesh(&mut e);
        let loaded = info(&mut e, 0, navmesh, 0);
        set_word(&mut e, l, NAV_MESH, loaded);
        assert!(e.call(0x006d_d6a0, &args![l]).bool());
        e.set(l, PathingLocation::usTriangle, NO_TRIANGLE);
        assert!(!e.call(0x006d_d6a0, &args![l]).bool());
    }

    #[test]
    fn clear_triangle_data_clears_the_triangle() {
        let mut e = engine();
        let l = location(&mut e);
        e.set(l, PathingLocation::usTriangle, 3);
        e.call(0x006d_dbe0, &args![l]);
        assert_eq!(e.get(l, PathingLocation::usTriangle), NO_TRIANGLE);
    }

    // -- Tests: ResolveNavMeshInfo -----------------------------------------

    #[test]
    fn resolve_gives_up_when_blocked_or_when_there_is_nothing() {
        let mut e = engine();
        let l = location(&mut e);
        assert!(!e.call(0x006d_d6f0, &args![l, 0u32]).bool());
        answer(&mut e, FLAGS_BLOCK_RESOLVING, 1);
        let navmesh_info = info(&mut e, 0, 0, 0);
        set_word(&mut e, l, NAV_MESH, navmesh_info);
        assert!(!e.call(0x006d_d6f0, &args![l, 0u32]).bool());
    }

    #[test]
    fn resolve_with_a_navmesh_info_only_searches_the_missing_triangle() {
        let mut e = engine();
        let navmesh = mesh(&mut e);
        e.mem.set_u32(navmesh, 7);
        let navmesh_info = info(&mut e, 0, navmesh, 0);
        let l = location(&mut e);
        set_word(&mut e, l, NAV_MESH, navmesh_info);
        let search = recorder(&mut e, NAV_MESH_FIND_TRIANGLE, 7);
        assert!(e.call(0x006d_d6f0, &args![l, 0x55u32]).bool());
        assert_eq!(e.get(l, PathingLocation::usTriangle), 7);
        assert_eq!(search.borrow()[0], vec![navmesh, position(l), 0x55]);
        // A triangle already there: no search.
        e.set(l, PathingLocation::usTriangle, 9);
        assert!(e.call(0x006d_d6f0, &args![l, 0u32]).bool());
        assert_eq!(e.get(l, PathingLocation::usTriangle), 9);
        assert_eq!(search.borrow().len(), 1);
        // A disabled info.
        let disabled = info(&mut e, 0x10, navmesh, 0);
        set_word(&mut e, l, NAV_MESH, disabled);
        assert!(!e.call(0x006d_d6f0, &args![l, 0u32]).bool());
    }

    #[test]
    fn resolve_takes_the_first_candidate_whose_bounds_contain_the_position() {
        let mut e = engine();
        let navmesh = mesh(&mut e);
        e.mem.set_u32(navmesh, 6);
        let (far, near) = (
            bounds(&mut e, false, [100.0, 0.0, 0.0]),
            bounds(&mut e, true, [0.0; 3]),
        );
        let disabled = info(&mut e, 0x10, 0, near);
        let first = info(&mut e, 0, 0, far);
        let second = info(&mut e, 0, navmesh, near);
        let list = array(&mut e, &[disabled, first, second]);
        let l = location(&mut e);
        set_word(&mut e, l, NAV_MESHES, list);
        assert!(e.call(0x006d_d6f0, &args![l, 0u32]).bool());
        assert_eq!(word(&e, l, NAV_MESH), second);
        assert_eq!(word(&e, l, NAV_MESHES), 0);
        assert_eq!(e.get(l, PathingLocation::usTriangle), 6);
        assert_eq!(e.get(l, PathingLocation::uiFlags), 1);
    }

    #[test]
    fn resolve_takes_the_closest_bounds_within_the_limit() {
        let mut e = engine();
        let (far, near) = (
            bounds(&mut e, false, [100.0, 0.0, 0.0]),
            bounds(&mut e, false, [50.0, 0.0, 0.0]),
        );
        let first = info(&mut e, 0, 0, far);
        let second = info(&mut e, 0, 0, near);
        let list = array(&mut e, &[first, second]);
        let l = location(&mut e);
        e.set(l, PathingLocation::usTriangle, 3);
        set_word(&mut e, l, NAV_MESHES, list);
        assert!(e.call(0x006d_d6f0, &args![l, 0u32]).bool());
        assert_eq!(word(&e, l, NAV_MESH), second);
        assert_eq!(e.get(l, PathingLocation::usTriangle), NO_TRIANGLE);
        assert_eq!(e.get(l, PathingLocation::uiFlags), 1);
    }

    #[test]
    fn resolve_falls_back_to_the_candidate_without_bounds_when_all_are_far() {
        let mut e = engine();
        let far = bounds(&mut e, false, [200.0, 0.0, 0.0]);
        let with_bounds = info(&mut e, 0, 0, far);
        let without_bounds = info(&mut e, 0, 0, 0);
        let list = array(&mut e, &[with_bounds, without_bounds]);
        let l = location(&mut e);
        set_word(&mut e, l, NAV_MESHES, list);
        // `fn_006de490` sees the navmesh info just stored and answers true.
        assert!(e.call(0x006d_d6f0, &args![l, 0u32]).bool());
        assert_eq!(word(&e, l, NAV_MESH), without_bounds);
        assert_eq!(e.get(l, PathingLocation::uiFlags), 1);
    }

    #[test]
    fn resolve_in_a_cell_of_the_special_kind_takes_a_candidate_with_a_triangle() {
        let mut e = engine();
        answer(&mut e, CELL_IS_KIND, 1);
        let (no_triangle, with_triangle) = (mesh(&mut e), mesh(&mut e));
        e.mem.set_u32(no_triangle, 0xffff);
        e.mem.set_u32(with_triangle, 12);
        let unloaded = info(&mut e, 0, 0, 0);
        let first = info(&mut e, 0, no_triangle, 0);
        let second = info(&mut e, 0, with_triangle, 0);
        let list = array(&mut e, &[unloaded, first, second]);
        let l = location(&mut e);
        set_word(&mut e, l, NAV_MESHES, list);
        set_word(&mut e, l, CELL, 0xce11);
        assert!(e.call(0x006d_d6f0, &args![l, 0u32]).bool());
        assert_eq!(word(&e, l, NAV_MESH), second);
        assert_eq!(word(&e, l, NAV_MESHES), 0);
        assert_eq!(e.get(l, PathingLocation::usTriangle), 12);
        assert_eq!(e.get(l, PathingLocation::uiFlags), 1);
    }

    #[test]
    fn resolve_in_a_cell_of_the_special_kind_skips_candidates_the_form_test_flags() {
        let mut e = engine();
        answer(&mut e, CELL_IS_KIND, 1);
        answer(&mut e, FORM_TEST_SECOND, 1);
        // The candidate is flagged, so the closest navmesh is resolved: the
        // set-up inside it leaves this one candidate as the navmesh info and
        // its closest triangle is an exact hit.
        let navmesh = searchable_mesh(&mut e, 5, true, [0.0; 3]);
        let candidate = info(&mut e, 0, navmesh, 0);
        let list = array(&mut e, &[candidate]);
        answer(&mut e, POTENTIAL_NAV_MESHES_FOR_CELL, list);
        let l = location(&mut e);
        set_position(&mut e, l, [1.0, 2.0, 3.0]);
        set_word(&mut e, l, NAV_MESHES, list);
        set_word(&mut e, l, CELL, 0xce11);
        let triangle_search = recorder(&mut e, NAV_MESH_FIND_TRIANGLE, 0);
        assert!(e.call(0x006d_d6f0, &args![l, 0u32]).bool());
        assert!(triangle_search.borrow().is_empty());
        assert_eq!(word(&e, l, NAV_MESH), candidate);
        // The triangle is cleared after the resolve, the position restored
        // and the flag set.
        assert_eq!(e.get(l, PathingLocation::usTriangle), NO_TRIANGLE);
        assert_eq!(point(&e, position(l)), [1.0, 2.0, 3.0]);
        assert_eq!(e.get(l, PathingLocation::uiFlags), 1);
    }

    #[test]
    fn resolve_without_bounds_in_the_cell_branch_fails_when_nothing_resolves() {
        let mut e = engine();
        answer(&mut e, CELL_IS_KIND, 1);
        let unloaded = info(&mut e, 0, 0, 0);
        let other = info(&mut e, 0, 0, 0);
        let list = array(&mut e, &[unloaded, other]);
        answer(&mut e, POTENTIAL_NAV_MESHES_FOR_CELL, list);
        let l = location(&mut e);
        set_position(&mut e, l, [1.0, 2.0, 3.0]);
        set_word(&mut e, l, NAV_MESHES, list);
        set_word(&mut e, l, CELL, 0xce11);
        e.set(l, PathingLocation::uiFlags, 1);
        e.set(l, PathingLocation::usTriangle, 4);
        assert!(!e.call(0x006d_d6f0, &args![l, 0u32]).bool());
        assert_eq!(e.get(l, PathingLocation::usTriangle), NO_TRIANGLE);
        assert_eq!(point(&e, position(l)), [1.0, 2.0, 3.0]);
        // The flag is rewritten with its own value (`fn_006dd5a0`).
        assert_eq!(e.get(l, PathingLocation::uiFlags) & 1, 0);
    }

    // -- Tests: ResolveToClosestNavmeshAndTriangle ---------------------------

    /// A candidate whose closest-triangle search finds `triangle`, `exact` or
    /// not, with the triangle centre at `centre`.
    fn searchable_mesh(e: &mut Engine, triangle: u32, exact: bool, centre: [f32; 3]) -> u32 {
        let navmesh = mesh(e);
        e.mem.set_u32(navmesh + 4, triangle);
        e.mem.set_u32(navmesh + 8, exact as u32);
        e.mem.set_u32(navmesh + 0x0c, 1);
        put_point(e, navmesh + 0x10, centre);
        navmesh
    }

    #[test]
    fn closest_with_nothing_resolved_is_false() {
        let mut e = engine();
        let l = location(&mut e);
        assert!(!e.call(0x006d_dc00, &args![l]).bool());
    }

    #[test]
    fn closest_in_one_navmesh_keeps_the_position_when_it_is_inside_the_triangle() {
        let mut e = engine();
        let navmesh = searchable_mesh(&mut e, 9, true, [1.0, 1.0, 1.0]);
        let candidate = info(&mut e, 0, navmesh, 0);
        let list = array(&mut e, &[candidate]);
        answer(&mut e, POTENTIAL_NAV_MESHES_FOR_CELL, list);
        let l = location(&mut e);
        set_position(&mut e, l, [4.0, 5.0, 6.0]);
        set_word(&mut e, l, CELL, 0xce11);
        assert!(e.call(0x006d_dc00, &args![l]).bool());
        assert_eq!(word(&e, l, NAV_MESH), candidate);
        assert_eq!(e.get(l, PathingLocation::usTriangle), 9);
        assert_eq!(point(&e, position(l)), [4.0, 5.0, 6.0]);
    }

    #[test]
    fn closest_in_one_navmesh_pulls_the_position_towards_the_triangle_centre() {
        let mut e = engine();
        let navmesh = searchable_mesh(&mut e, 9, false, [1.0, 2.0, 3.0]);
        e.mem.set_u32(navmesh + 0x20, 1);
        put_point(&mut e, navmesh + 0x24, [10.0, 20.0, 30.0]);
        let candidate = info(&mut e, 0, navmesh, 0);
        let list = array(&mut e, &[candidate]);
        answer(&mut e, POTENTIAL_NAV_MESHES_FOR_CELL, list);
        let l = location(&mut e);
        set_position(&mut e, l, [4.0, 5.0, 6.0]);
        set_word(&mut e, l, CELL, 0xce11);
        assert!(e.call(0x006d_dc00, &args![l]).bool());
        let (edge, centre) = (0.9f32, 0.1f32);
        assert_eq!(
            point(&e, position(l)),
            [
                10.0 * edge + 1.0 * centre,
                20.0 * edge + 2.0 * centre,
                30.0 * edge + 3.0 * centre
            ]
        );
        assert_eq!(e.get(l, PathingLocation::usTriangle), 9);
    }

    #[test]
    fn closest_in_one_navmesh_without_a_crossing_edge_keeps_the_position() {
        let mut e = engine();
        let navmesh = searchable_mesh(&mut e, 9, false, [1.0, 2.0, 3.0]);
        let candidate = info(&mut e, 0, navmesh, 0);
        let list = array(&mut e, &[candidate]);
        answer(&mut e, POTENTIAL_NAV_MESHES_FOR_CELL, list);
        let l = location(&mut e);
        set_position(&mut e, l, [4.0, 5.0, 6.0]);
        set_word(&mut e, l, CELL, 0xce11);
        assert!(e.call(0x006d_dc00, &args![l]).bool());
        assert_eq!(point(&e, position(l)), [4.0, 5.0, 6.0]);
    }

    #[test]
    fn closest_in_one_navmesh_fails_when_the_search_or_the_load_fails() {
        let mut e = engine();
        let navmesh = searchable_mesh(&mut e, 9, true, [0.0; 3]);
        e.mem.set_u32(navmesh + 0x0c, 0);
        let candidate = info(&mut e, 0, navmesh, 0);
        let list = array(&mut e, &[candidate]);
        answer(&mut e, POTENTIAL_NAV_MESHES_FOR_CELL, list);
        let l = location(&mut e);
        set_word(&mut e, l, CELL, 0xce11);
        assert!(!e.call(0x006d_dc00, &args![l]).bool());
        let unloaded = info(&mut e, 0, 0, 0);
        let list = array(&mut e, &[unloaded]);
        answer(&mut e, POTENTIAL_NAV_MESHES_FOR_CELL, list);
        assert!(!e.call(0x006d_dc00, &args![l]).bool());
    }

    fn two_candidates(e: &mut Engine, meshes: [u32; 2]) -> (u32, [u32; 2], Ptr<PathingLocation>) {
        let first = info(e, 0, meshes[0], 0);
        let second = info(e, 0, meshes[1], 0);
        let list = array(e, &[first, second]);
        answer(e, POTENTIAL_NAV_MESHES_FOR_CELL, list);
        let l = location(e);
        set_word(e, l, CELL, 0xce11);
        (list, [first, second], l)
    }

    #[test]
    fn closest_among_candidates_takes_an_exact_hit_at_once() {
        let mut e = engine();
        let miss = searchable_mesh(&mut e, 3, false, [50.0, 50.0, 0.0]);
        let hit = searchable_mesh(&mut e, 8, true, [60.0, 60.0, 0.0]);
        let (_, [_, second], l) = two_candidates(&mut e, [miss, hit]);
        assert!(e.call(0x006d_dc00, &args![l]).bool());
        assert_eq!(word(&e, l, NAV_MESH), second);
        assert_eq!(word(&e, l, NAV_MESHES), 0);
        assert_eq!(e.get(l, PathingLocation::usTriangle), 8);
    }

    #[test]
    fn closest_among_candidates_takes_the_nearest_triangle_centre() {
        let mut e = engine();
        let far = searchable_mesh(&mut e, 3, false, [3.0, 4.0, 0.0]);
        let near = searchable_mesh(&mut e, 4, false, [1.0, 1.0, 0.0]);
        let (_, [_, second], l) = two_candidates(&mut e, [far, near]);
        set_position(&mut e, l, [0.0, 0.0, 10.0]);
        let ground = recorder(&mut e, TES_GROUND_HEIGHT, 0);
        e.set_global(TES_SINGLETON, 0x7e5u32);
        assert!(e.call(0x006d_dc00, &args![l]).bool());
        assert_eq!(word(&e, l, NAV_MESH), second);
        assert_eq!(e.get(l, PathingLocation::usTriangle), 4);
        // The ground height was asked of TES for the position and the cell.
        assert_eq!(ground.borrow()[0], vec![0x7e5, position(l), 0xce11]);
    }

    #[test]
    fn closest_among_candidates_prefers_a_centre_at_an_acceptable_height() {
        let mut e = engine();
        // The position is 500 above the ground at 0: out of reach of 180.
        let low = searchable_mesh(&mut e, 3, false, [1.0, 0.0, 0.0]);
        let high = searchable_mesh(&mut e, 4, false, [5.0, 0.0, 400.0]);
        let (_, [_, second], l) = two_candidates(&mut e, [low, high]);
        set_position(&mut e, l, [0.0, 0.0, 500.0]);
        assert!(e.call(0x006d_dc00, &args![l]).bool());
        assert_eq!(word(&e, l, NAV_MESH), second);
        assert_eq!(e.get(l, PathingLocation::usTriangle), 4);
    }

    #[test]
    fn closest_among_candidates_near_the_ground_ignores_the_height_limit() {
        let mut e = engine();
        e.register(TES_GROUND_HEIGHT, |_, _| st0(480.0));
        let low = searchable_mesh(&mut e, 3, false, [1.0, 0.0, 0.0]);
        let high = searchable_mesh(&mut e, 4, false, [5.0, 0.0, 400.0]);
        let (_, [first, _], l) = two_candidates(&mut e, [low, high]);
        set_position(&mut e, l, [0.0, 0.0, 500.0]);
        assert!(e.call(0x006d_dc00, &args![l]).bool());
        // 500 < 480 + 25: the nearer first candidate is acceptable.
        assert_eq!(word(&e, l, NAV_MESH), first);
    }

    #[test]
    fn closest_among_candidates_fails_without_any_navmesh() {
        let mut e = engine();
        let (_, _, l) = two_candidates(&mut e, [0, 0]);
        assert!(!e.call(0x006d_dc00, &args![l]).bool());
    }

    // -- Tests: fn_006de250 and fn_006de490 ---------------------------------

    /// Sets the adjacent (+0x24) and preferred (+0x34) arrays of a navmesh
    /// info in place.
    fn neighbours(e: &mut Engine, navmesh_info: u32, adjacent: &[u32], preferred: &[u32]) {
        for (offset, list) in [(0x24, adjacent), (0x34, preferred)] {
            let data = e.mem.alloc(0x40);
            for (i, item) in list.iter().enumerate() {
                e.mem.set_u32(data + 4 * i as u32, *item);
            }
            e.mem.set_u32(navmesh_info + offset + 4, data);
            e.mem.set_u32(navmesh_info + offset + 8, list.len() as u32);
        }
    }

    #[test]
    fn prune_gives_up_when_blocked_empty_or_disabled() {
        let mut e = engine();
        let l = location(&mut e);
        assert!(!e.call(0x006d_e250, &args![l]).bool());
        let disabled = info(&mut e, 0x10, 0, 0);
        set_word(&mut e, l, NAV_MESH, disabled);
        assert!(!e.call(0x006d_e250, &args![l]).bool());
        let enabled = info(&mut e, 0, 0, 0);
        set_word(&mut e, l, NAV_MESH, enabled);
        assert!(e.call(0x006d_e250, &args![l]).bool());
        answer(&mut e, FLAGS_BLOCK_RESOLVING, 1);
        assert!(!e.call(0x006d_e250, &args![l]).bool());
    }

    #[test]
    fn prune_keeps_only_the_candidate_whose_bounds_contain_the_position() {
        let mut e = engine();
        let (outside, inside) = (
            bounds(&mut e, false, [0.0; 3]),
            bounds(&mut e, true, [0.0; 3]),
        );
        let (a, b, c) = (
            info(&mut e, 0, 0, outside),
            info(&mut e, 0, 0, inside),
            info(&mut e, 0, 0, inside),
        );
        let list = array(&mut e, &[a, b, c]);
        let l = location(&mut e);
        set_word(&mut e, l, NAV_MESHES, list);
        assert!(e.call(0x006d_e250, &args![l]).bool());
        assert_eq!(word(&e, l, NAV_MESH), b);
        assert_eq!(word(&e, l, NAV_MESHES), 0);
        assert_eq!(e.get(l, PathingLocation::uiFlags), 1);
        // The list itself was not changed (the copy was).
        assert_eq!(items(&e, list), vec![a, b, c]);
    }

    #[test]
    fn prune_drops_candidates_without_bounds_that_the_form_test_flags() {
        let mut e = engine();
        e.register(FORM_TEST_FIRST, |e, a| ret((e.mem.u32(a[0]) == 1) as u32));
        let (flagged_mesh, other_mesh) = (mesh(&mut e), mesh(&mut e));
        e.mem.set_u32(flagged_mesh, 1);
        let flagged = info(&mut e, 0, flagged_mesh, 0);
        let kept = info(&mut e, 0, other_mesh, 0);
        let unloaded = info(&mut e, 0, 0, 0);
        let list = array(&mut e, &[flagged, kept, unloaded]);
        let l = location(&mut e);
        set_word(&mut e, l, NAV_MESHES, list);
        // Two survivors: `fn_006de490` decides; with `kept` and `unloaded`
        // unconnected it says no, so the flag stays clear.
        neighbours(&mut e, kept, &[], &[]);
        assert!(!e.call(0x006d_e250, &args![l]).bool());
        assert_eq!(word(&e, l, NAV_MESH), 0);
        assert_eq!(e.get(l, PathingLocation::uiFlags), 0);
    }

    #[test]
    fn prune_with_one_survivor_chooses_it() {
        let mut e = engine();
        e.register(FORM_TEST_FIRST, |_, _| ret(1));
        let flagged_mesh = mesh(&mut e);
        let flagged = info(&mut e, 0, flagged_mesh, 0);
        let kept = info(&mut e, 0, 0, 0);
        let list = array(&mut e, &[flagged, kept]);
        let l = location(&mut e);
        set_word(&mut e, l, NAV_MESHES, list);
        assert!(e.call(0x006d_e250, &args![l]).bool());
        assert_eq!(word(&e, l, NAV_MESH), kept);
        assert_eq!(word(&e, l, NAV_MESHES), 0);
        assert_eq!(e.get(l, PathingLocation::uiFlags), 1);
    }

    #[test]
    fn prune_with_connected_survivors_sets_the_flag_from_the_connectivity() {
        let mut e = engine();
        let (a, b) = (info(&mut e, 0, 0, 0), info(&mut e, 0, 0, 0));
        neighbours(&mut e, a, &[b], &[]);
        neighbours(&mut e, b, &[], &[]);
        let list = array(&mut e, &[a, b]);
        let l = location(&mut e);
        set_word(&mut e, l, NAV_MESHES, list);
        assert!(e.call(0x006d_e250, &args![l]).bool());
        assert_eq!(e.get(l, PathingLocation::uiFlags), 1);
        // Still a list of candidates, with the flag saying it is settled.
        assert_eq!(word(&e, l, NAV_MESHES), list);
    }

    #[test]
    fn connectivity_is_true_with_a_navmesh_info_and_false_without_a_list() {
        let mut e = engine();
        let l = location(&mut e);
        assert!(!e.call(0x006d_e490, &args![l]).bool());
        let navmesh_info = info(&mut e, 0, 0, 0);
        set_word(&mut e, l, NAV_MESH, navmesh_info);
        assert!(e.call(0x006d_e490, &args![l]).bool());
    }

    #[test]
    fn connectivity_is_false_when_the_list_test_holds() {
        let mut e = engine();
        let a = info(&mut e, 0, 0, 0);
        neighbours(&mut e, a, &[], &[]);
        let list = array(&mut e, &[a]);
        let l = location(&mut e);
        set_word(&mut e, l, NAV_MESHES, list);
        answer(&mut e, LIST_TEST, 1);
        assert!(!e.call(0x006d_e490, &args![l]).bool());
    }

    #[test]
    fn connectivity_follows_adjacent_and_preferred_neighbours_of_the_first_candidate() {
        let mut e = engine();
        let (a, b, c) = (
            info(&mut e, 0, 0, 0),
            info(&mut e, 0, 0, 0),
            info(&mut e, 0, 0, 0),
        );
        // `outside` is a neighbour that is not a candidate.
        let outside = info(&mut e, 0, 0, 0);
        neighbours(&mut e, a, &[b, outside], &[c]);
        neighbours(&mut e, b, &[], &[]);
        neighbours(&mut e, c, &[], &[]);
        let list = array(&mut e, &[a, b, c]);
        let l = location(&mut e);
        set_word(&mut e, l, NAV_MESHES, list);
        assert!(e.call(0x006d_e490, &args![l]).bool());
    }

    #[test]
    fn connectivity_spreads_through_the_adjacent_neighbours_of_the_taken_ones() {
        let mut e = engine();
        let (a, b, c) = (
            info(&mut e, 0, 0, 0),
            info(&mut e, 0, 0, 0),
            info(&mut e, 0, 0, 0),
        );
        neighbours(&mut e, a, &[b], &[]);
        neighbours(&mut e, b, &[c], &[]);
        neighbours(&mut e, c, &[], &[]);
        let list = array(&mut e, &[a, b, c]);
        let l = location(&mut e);
        set_word(&mut e, l, NAV_MESHES, list);
        assert!(e.call(0x006d_e490, &args![l]).bool());
        // A preferred-only link is not followed in the second phase.
        neighbours(&mut e, b, &[], &[c]);
        assert!(!e.call(0x006d_e490, &args![l]).bool());
    }

    #[test]
    fn connectivity_is_false_for_a_disconnected_candidate() {
        let mut e = engine();
        let (a, b) = (info(&mut e, 0, 0, 0), info(&mut e, 0, 0, 0));
        neighbours(&mut e, a, &[], &[]);
        neighbours(&mut e, b, &[a], &[]);
        let list = array(&mut e, &[a, b]);
        let l = location(&mut e);
        set_word(&mut e, l, NAV_MESHES, list);
        assert!(!e.call(0x006d_e490, &args![l]).bool());
    }

    // -- Tests: PrintDebugText ----------------------------------------------

    fn double_words(values: &[f64]) -> Vec<u32> {
        values
            .iter()
            .flat_map(|v| [v.to_bits() as u32, (v.to_bits() >> 32) as u32])
            .collect()
    }

    /// An object whose name slot answers `name`.
    fn named(e: &mut Engine, name: u32) -> u32 {
        let target = 0x0061_0000 + name;
        answer(e, target, name);
        object_with_slot(e, SLOT_NAME, target)
    }

    #[test]
    fn debug_text_names_the_cell_or_none() {
        let mut e = engine();
        let text = recorder(&mut e, FORMAT_TEXT, 0);
        let l = location(&mut e);
        set_position(&mut e, l, [1.5, 2.5, 3.5]);
        e.call(0x006d_e790, &args![l, 0x1000u32, 0x80u32]);
        let mut expected = vec![0x1000, 0x80, FORMAT_CELL, TEXT_NONE];
        expected.extend(double_words(&[1.5, 2.5, 3.5]));
        assert_eq!(text.borrow()[0], expected);
        let cell = named(&mut e, 0x5150);
        set_word(&mut e, l, CELL, cell);
        e.call(0x006d_e790, &args![l, 0x1000u32, 0x80u32]);
        assert_eq!(text.borrow()[1][3], 0x5150);
    }

    #[test]
    fn debug_text_with_an_empty_candidate_list_prints_the_cell() {
        let mut e = engine();
        let text = recorder(&mut e, FORMAT_TEXT, 0);
        let l = location(&mut e);
        let navmesh_info = info(&mut e, 0, 0, 0);
        set_word(&mut e, l, NAV_MESH, navmesh_info);
        let empty = array(&mut e, &[]);
        set_word(&mut e, l, NAV_MESHES, empty);
        e.call(0x006d_e790, &args![l, 0x1000u32, 0x80u32]);
        assert_eq!(text.borrow()[0][2], FORMAT_CELL);
    }

    #[test]
    fn debug_text_names_the_world_space_grid_cell_and_cell() {
        let mut e = engine();
        let text = recorder(&mut e, FORMAT_TEXT, 0);
        let world = named(&mut e, 0x5ac0);
        let found = named(&mut e, 0x5ce1);
        let lookup = recorder(&mut e, WORLD_SPACE_CELL_FROM_COORD, found);
        let l = location(&mut e);
        set_position(&mut e, l, [8192.5, -4096.0, 7.0]);
        set_word(&mut e, l, WORLD_SPACE, world);
        e.call(0x006d_e790, &args![l, 0x1000u32, 0x80u32]);
        assert_eq!(lookup.borrow()[0], vec![world, position(l)]);
        let mut expected = vec![
            0x1000,
            0x80,
            FORMAT_WORLD_CELL_NAMED,
            0x5ac0,
            2,
            (-1i32) as u32,
            0x5ce1,
        ];
        expected.extend(double_words(&[8192.5, -4096.0, 7.0]));
        assert_eq!(text.borrow()[0], expected);
    }

    #[test]
    fn debug_text_without_a_cell_at_the_position_prints_the_world_space_only() {
        let mut e = engine();
        let text = recorder(&mut e, FORMAT_TEXT, 0);
        let world = named(&mut e, 0x5ac0);
        let l = location(&mut e);
        set_position(&mut e, l, [4096.0, 8192.0, 7.0]);
        set_word(&mut e, l, WORLD_SPACE, world);
        e.call(0x006d_e790, &args![l, 0x1000u32, 0x80u32]);
        let mut expected = vec![0x1000, 0x80, FORMAT_WORLD_CELL, 0x5ac0, 1, 2];
        expected.extend(double_words(&[4096.0, 8192.0, 7.0]));
        assert_eq!(text.borrow()[0], expected);
    }

    #[test]
    fn debug_text_with_a_navmesh_info_prints_the_triangle_when_there_is_one() {
        let mut e = engine();
        let text = recorder(&mut e, FORMAT_TEXT, 0);
        let l = location(&mut e);
        set_position(&mut e, l, [1.0, 2.0, 3.0]);
        let navmesh_info = info(&mut e, 0, 0, 0);
        set_word(&mut e, l, NAV_MESH, navmesh_info);
        e.call(0x006d_e790, &args![l, 0x1000u32, 0x80u32]);
        let first = text.borrow()[0].clone();
        assert_eq!(first[..3], [0x1000, 0x80, FORMAT_NAV_MESH]);
        assert_eq!(first[4..], double_words(&[1.0, 2.0, 3.0]));
        e.set(l, PathingLocation::usTriangle, 0x21);
        e.call(0x006d_e790, &args![l, 0x1000u32, 0x80u32]);
        let second = text.borrow()[1].clone();
        assert_eq!(second[..3], [0x1000, 0x80, FORMAT_TRIANGLE]);
        assert_eq!(second[4], 0x21);
        assert_eq!(second[5..], double_words(&[1.0, 2.0, 3.0]));
    }

    // -- Tests: IsValidLocation and fn_006deeb0 --------------------------------

    /// A location that is settled: enabled navmesh info with a loaded mesh and
    /// a triangle.
    fn settled_location(e: &mut Engine) -> Ptr<PathingLocation> {
        let navmesh = mesh(e);
        let navmesh_info = info(e, 0, navmesh, 0);
        let l = location(e);
        set_word(e, l, NAV_MESH, navmesh_info);
        e.set(l, PathingLocation::usTriangle, 3);
        set_position(e, l, [10.0, 20.0, 30.0]);
        e.register(GET_POSITION, |e, a| {
            let here = point(e, a[0] + 4);
            put_point(e, a[1], here);
            ret(a[1])
        });
        l
    }

    /// Records the points the line-of-sight checks are asked about; the
    /// check number `failing` (from 1) answers false.
    fn watch_sight(e: &mut Engine, failing: usize) -> Rc<RefCell<Vec<[f32; 3]>>> {
        let seen: Rc<RefCell<Vec<[f32; 3]>>> = Rc::default();
        let sink = seen.clone();
        e.register_double(CHECK_LINE_OF_SIGHT, move |e, a| {
            sink.borrow_mut().push(point(e, a[1]));
            assert_eq!(a[2..], [0, 0]);
            ret((sink.borrow().len() != failing) as u32)
        });
        seen
    }

    #[test]
    fn valid_location_needs_a_resolved_navmesh_and_triangle() {
        let mut e = engine();
        let l = location(&mut e);
        assert!(!e.call(0x006d_ed70, &args![l, 5.0f32]).bool());
    }

    #[test]
    fn valid_location_without_a_radius_skips_the_sight_checks() {
        let mut e = engine();
        let l = settled_location(&mut e);
        let seen = watch_sight(&mut e, 0);
        assert!(e.call(0x006d_ed70, &args![l, 0.0f32]).bool());
        assert!(e.call(0x006d_ed70, &args![l, -3.0f32]).bool());
        assert!(seen.borrow().is_empty());
    }

    #[test]
    fn valid_location_checks_four_points_around_it() {
        let mut e = engine();
        let l = settled_location(&mut e);
        let seen = watch_sight(&mut e, 0);
        assert!(e.call(0x006d_ed70, &args![l, 5.0f32]).bool());
        assert_eq!(
            *seen.borrow(),
            vec![
                [5.0, 20.0, 30.0],
                [15.0, 20.0, 30.0],
                [10.0, 15.0, 30.0],
                [10.0, 25.0, 30.0]
            ]
        );
    }

    #[test]
    fn valid_location_stops_at_the_first_failing_check() {
        let mut e = engine();
        let l = settled_location(&mut e);
        let seen = watch_sight(&mut e, 2);
        assert!(!e.call(0x006d_ed70, &args![l, 5.0f32]).bool());
        assert_eq!(seen.borrow().len(), 2);
    }

    #[test]
    fn near_requires_the_same_world_space_and_cell_or_key() {
        let mut e = engine();
        let (a, b) = (location(&mut e), location(&mut e));
        assert!(e.call(0x006d_eeb0, &args![a, b, 1.0f32]).bool());
        set_word(&mut e, b, WORLD_SPACE, 0x3a5);
        assert!(!e.call(0x006d_eeb0, &args![a, b, 1.0f32]).bool());
        // The same world space: the keys decide.
        set_word(&mut e, a, WORLD_SPACE, 0x3a5);
        set_word(&mut e, a, CELL_KEY, 1);
        set_word(&mut e, b, CELL_KEY, 2);
        assert!(!e.call(0x006d_eeb0, &args![a, b, 1.0f32]).bool());
        set_word(&mut e, b, CELL_KEY, 1);
        assert!(e.call(0x006d_eeb0, &args![a, b, 1.0f32]).bool());
        // No world space: the cells decide.
        set_word(&mut e, a, WORLD_SPACE, 0);
        set_word(&mut e, b, WORLD_SPACE, 0);
        set_word(&mut e, a, CELL, 7);
        assert!(!e.call(0x006d_eeb0, &args![a, b, 1.0f32]).bool());
    }

    #[test]
    fn near_compares_the_squared_distance_with_the_squared_radius() {
        let mut e = engine();
        let (a, b) = (location(&mut e), location(&mut e));
        set_position(&mut e, a, [0.0, 0.0, 0.0]);
        set_position(&mut e, b, [3.0, 4.0, 0.0]);
        // 25 against 5 * 5: the boundary counts as near.
        assert!(e.call(0x006d_eeb0, &args![a, b, 5.0f32]).bool());
        assert!(!e.call(0x006d_eeb0, &args![a, b, 4.9f32]).bool());
        assert!(e.call(0x006d_eeb0, &args![a, b, -5.0f32]).bool());
    }

    // -- Tests: save, load and the scrap array --------------------------------

    #[test]
    fn save_game_writes_the_fields_in_order() {
        let mut e = engine();
        let navmesh_info = info(&mut e, 0, 0, 0);
        e.mem.set_u32(navmesh_info, 0xabcd);
        let l = location(&mut e);
        set_word(&mut e, l, NAV_MESH, navmesh_info);
        set_word(&mut e, l, CELL, 0xce11);
        set_word(&mut e, l, WORLD_SPACE, 0x3a5);
        set_word(&mut e, l, CELL_KEY, 0x77);
        let flag_seen: Rc<RefCell<Vec<u8>>> = Rc::default();
        let sink = flag_seen.clone();
        e.register_double(SAVE_BYTES, move |e, a| {
            sink.borrow_mut().push(e.mem.u8(a[1]));
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let buffer = 0xb0f0;
        e.call(0x006d_ef40, &args![l, buffer]);
        let log = e.call_log.take().unwrap();
        let p = |off: u32| l.addr() + off;
        assert_eq!(
            log[1..],
            [
                (SAVE_BYTES, vec![buffer, p(4), 12, 0]),
                (NI_POINTER_GET, vec![navmesh_info]),
                (SAVE_FORM_ID, vec![buffer, 0xabcd, 0]),
                (SAVE_FORM, vec![buffer, 0xce11, 0]),
                (SAVE_FORM, vec![buffer, 0x3a5, 0]),
                (SAVE_BYTES, vec![buffer, p(0x20), 4, 0]),
                (SAVE_BYTES, vec![buffer, p(0x24), 2, 0]),
                (SAVE_BYTES, vec![buffer, p(0x26), 1, 0]),
                (SAVE_BYTES, vec![buffer, log[log.len() - 1].1[1], 1, 0]),
            ]
        );
        // No candidate list: the last byte is 0.
        assert_eq!(flag_seen.borrow().last(), Some(&0));
    }

    #[test]
    fn save_game_writes_the_list_flag_and_a_zero_id_without_a_navmesh_info() {
        let mut e = engine();
        let list = array(&mut e, &[]);
        let l = location(&mut e);
        set_word(&mut e, l, NAV_MESHES, list);
        let flag_seen: Rc<RefCell<Vec<u8>>> = Rc::default();
        let sink = flag_seen.clone();
        e.register_double(SAVE_BYTES, move |e, a| {
            sink.borrow_mut().push(e.mem.u8(a[1]));
            Ret::default()
        });
        let ids = recorder(&mut e, SAVE_FORM_ID, 0);
        e.call(0x006d_ef40, &args![l, 0xb0f0u32]);
        assert_eq!(ids.borrow()[0], vec![0xb0f0, 0, 0]);
        assert_eq!(flag_seen.borrow().last(), Some(&1));
    }

    #[test]
    fn load_game_reads_the_fields_and_recomputes_the_candidate_list() {
        let mut e = engine();
        let l = location(&mut e);
        let this = l.addr();
        e.register_double(LOAD_BYTES, move |e, a| {
            match a[2] {
                0x0c => put_point(e, a[1], [1.0, 2.0, 3.0]),
                4 => e.mem.set_u32(a[1], 0x77),
                2 => e.mem.set_u16(a[1], 5),
                1 if a[1] == this + 0x26 => e.mem.set_u8(a[1], 3),
                1 => e.mem.set_u8(a[1], 1),
                _ => panic!("unexpected size"),
            }
            Ret::default()
        });
        e.register(LOAD_FORM_ID, |e, a| {
            e.mem.set_u32(a[1], 0xabcd);
            Ret::default()
        });
        answer(&mut e, TES_NAV_MESH_INFO_MAP, 0x5151);
        e.set_global(TES_SINGLETON, 0x7e5u32);
        let found = recorder(&mut e, NAV_MESH_INFO_MAP_FIND, 0x1f0);
        let ids = Rc::new(RefCell::new(vec![0x10u32, 0x20]));
        let pending = ids.clone();
        e.register_double(LOAD_FORM, move |_, _| ret(pending.borrow_mut().remove(0)));
        let lookups = recorder(&mut e, LOOK_UP_FORM, 0xf0);
        let casts = Rc::new(RefCell::new(Vec::new()));
        let sink = casts.clone();
        e.register_double(DYNAMIC_CAST, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            ret(0xc000 + sink.borrow().len() as u32)
        });
        let potential = recorder(&mut e, POTENTIAL_NAV_MESHES_FOR_KEY, 0x1157);
        e.call(0x006d_f010, &args![l, 0xb0f0u32]);
        assert_eq!(point(&e, position(l)), [1.0, 2.0, 3.0]);
        assert_eq!(found.borrow()[0], vec![0x5151, 0xabcd]);
        assert_eq!(word(&e, l, NAV_MESH), 0x1f0);
        assert_eq!(lookups.borrow().len(), 2);
        assert_eq!(
            *casts.borrow(),
            vec![
                vec![0xf0, 0, TYPE_TES_FORM, TYPE_CELL, 0],
                vec![0xf0, 0, TYPE_TES_FORM, TYPE_WORLD_SPACE, 0]
            ]
        );
        assert_eq!(word(&e, l, CELL), 0xc001);
        assert_eq!(word(&e, l, WORLD_SPACE), 0xc002);
        assert_eq!(word(&e, l, CELL_KEY), 0x77);
        assert_eq!(e.get(l, PathingLocation::usTriangle), NO_TRIANGLE);
        assert_eq!(e.get(l, PathingLocation::uiFlags), 3);
        // The list flag is set and there is a world space.
        assert_eq!(potential.borrow()[0], vec![0xc002, 0x77]);
        assert_eq!(word(&e, l, NAV_MESHES), 0x1157);
    }

    #[test]
    fn load_game_uses_the_cell_for_the_list_without_a_world_space_and_skips_it_without_the_flag() {
        let mut e = engine();
        let l = location(&mut e);
        let this = l.addr();
        let list_flag = Rc::new(RefCell::new(1u8));
        let flag = list_flag.clone();
        e.register_double(LOAD_BYTES, move |e, a| {
            if a[2] == 1 && a[1] != this + 0x26 {
                e.mem.set_u8(a[1], *flag.borrow());
            }
            Ret::default()
        });
        e.register_double(DYNAMIC_CAST, {
            let mut calls = 0;
            move |_, _| {
                calls += 1;
                ret(if calls == 1 { 0xce11 } else { 0 })
            }
        });
        let potential = recorder(&mut e, POTENTIAL_NAV_MESHES_FOR_CELL, 0x1157);
        e.call(0x006d_f010, &args![l, 0xb0f0u32]);
        assert_eq!(potential.borrow()[0], vec![0xce11]);
        assert_eq!(word(&e, l, NAV_MESHES), 0x1157);
        // Without the flag the list is not touched.
        *list_flag.borrow_mut() = 0;
        set_word(&mut e, l, NAV_MESHES, 0);
        e.register_double(DYNAMIC_CAST, {
            let mut calls = 0;
            move |_, _| {
                calls += 1;
                ret(if calls == 1 { 0xce11 } else { 0 })
            }
        });
        e.call(0x006d_f010, &args![l, 0xb0f0u32]);
        assert_eq!(word(&e, l, NAV_MESHES), 0);
    }

    #[test]
    fn scrap_array_constructor_sets_the_vtable_and_the_thread_heap() {
        let mut e = engine();
        let array = e.mem.alloc(0x14);
        answer(&mut e, MEMORY_MANAGER, 0x1000);
        let heap = recorder(&mut e, THREAD_SCRAP_HEAP, 0x2000);
        let set_up = recorder(&mut e, SCRAP_ARRAY_SET_UP, 0);
        assert_eq!(e.call(0x006d_f180, &args![array]).u32(), array);
        assert_eq!(e.mem.u32(array), SCRAP_ARRAY_VTABLE);
        assert_eq!(e.mem.u32(array + 0x10), 0x2000);
        assert_eq!(heap.borrow()[0], vec![0x1000]);
        assert_eq!(set_up.borrow()[0], vec![array, 0, 0]);
    }
}
