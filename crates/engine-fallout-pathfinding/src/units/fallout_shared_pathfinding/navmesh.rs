//! `fallout shared/pathfinding/navmesh.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A `NavMesh` is a form (`TESForm`, 0x18 bytes on PC) with two more bases,
//! `TESChildCell` (vtable at +0x18) and `NiRefObject` (vtable at +0x1C,
//! reference count at +0x20), followed by the mesh data: arrays of vertices
//! (12 bytes), triangles (16 bytes) and extra edge infos (12 bytes), the door
//! portals, closed doors and cover list, a point-of-view map, the search grid
//! and the obstacle bookkeeping.
//!
//! Triangles ([`NavMeshTriangle`]) hold three vertex indexes, three neighbour
//! triangle indexes (`0xFFFF` = no neighbour) and a flag word. Bit `n` of
//! the flags says that edge `n` has extra info: its neighbour slot then does
//! not hold a triangle index but an index into the mesh's `ExtraEdgeInfo`
//! array ([`EdgeExtraInfo`]: the portal's `NavMeshInfo` and triangle).
//!
//! Translated so far (address order): the whole unit (every function from
//! `0049c4c0` and `0068eb80` up to `00692740`).
//!
//! The `NavMeshTriangle` and `NavMeshInfo` methods in this unit are unnamed
//! in the engine map; they are named here by what their bodies do. Methods
//! of `NavMeshInfo` (another unit's class, whose fields are read at their
//! offsets, see the `INFO_` constants) take the `NavMeshInfo` as `this`.
//!
//! Left out everywhere: the compiler's exception-unwinding frames.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{BSSimpleArray, NiTPointerMap};

// ---------------------------------------------------------------------------
// Layouts and constants
// ---------------------------------------------------------------------------

layout! {
    /// `NavMesh` (Xbox PDB), 0x108 bytes on PC (0x118 on the Xbox: the PC
    /// `TESForm` is 0x10 smaller, so the fields after it sit 0x10 lower).
    pub struct NavMesh: 0x108 {
        /// `pParentCell` (Xbox PDB): `TESObjectCELL*`.
        0x24 pParentCell: Ptr,
        /// `Vertices` (Xbox PDB): `BSSimpleArray<NavMeshVertex>`, 12-byte
        /// elements (three floats).
        0x28 Vertices: Inline<BSSimpleArray>,
        /// `Triangles` (Xbox PDB): `BSSimpleArray<NavMeshTriangle>`.
        0x38 Triangles: Inline<BSSimpleArray>,
        /// `ExtraEdgeInfo` (Xbox PDB): `BSSimpleArray<EdgeExtraInfo>`.
        0x48 ExtraEdgeInfo: Inline<BSSimpleArray>,
        /// `DoorPortals` (Xbox PDB): `BSSimpleArray<NavMeshTriangleDoorPortal>`.
        0x58 DoorPortals: Inline<BSSimpleArray>,
        /// `ClosedDoors` (Xbox PDB): `BSSimpleArray<NavMeshClosedDoorInfo>`.
        0x68 ClosedDoors: Inline<BSSimpleArray>,
        /// `CoverArray` (Xbox PDB): `BSSimpleArray<unsigned short>` of
        /// triangle indexes.
        0x78 CoverArray: Inline<BSSimpleArray>,
        /// `POVs` (Xbox PDB): `NiTMap<unsigned short, NavMeshPOVData>`.
        0x88 POVs: Inline<NiTPointerMap>,
        /// `ClosestPOVs` (Xbox PDB): `BSSimpleArray<unsigned short>`.
        0x98 ClosestPOVs: Inline<BSSimpleArray>,
        /// `Obstacles` (Xbox PDB): `BSSimpleArray<NiPointer<ObstacleUndoData>>`.
        0xD0 Obstacles: Inline<BSSimpleArray>,
        /// `pTriangleToObstacleMap` (Xbox PDB): `NiTMap*`, 0x10 bytes,
        /// created with 0x25 buckets.
        0xE0 pTriangleToObstacleMap: Ptr,
        /// `ObstaclePOVs` (Xbox PDB): `BSSimpleArray<unsigned short>`.
        0xE4 ObstaclePOVs: Inline<BSSimpleArray>,
        /// `StaticAvoidNodes` (Xbox PDB): `BSSimpleArray<NavMeshStaticAvoidNode>`.
        0xF4 StaticAvoidNodes: Inline<BSSimpleArray>,
        /// `pNavMeshInfo` (Xbox PDB): `NavMeshInfo*`.
        0x104 pNavMeshInfo: Ptr,
    }

    /// `NavMeshTriangle` (Xbox PDB), 0x10 bytes.
    pub struct NavMeshTriangle: 0x10 {
        /// `Vertices[0]` (Xbox PDB): first of three `u16` vertex indexes.
        0x00 Vertices: u16,
        /// `Triangles[0]` (Xbox PDB): first of three `u16` neighbour
        /// triangle indexes (or extra-info indexes, see the module docs).
        0x06 Triangles: u16,
        /// `TriangleFlags` (Xbox PDB).
        0x0C TriangleFlags: u32,
    }

    /// `NavMeshGrid` (Xbox PDB), 0x28 bytes: the search grid, embedded in the
    /// mesh at [`MESH_GRID_OFFSET`]. `GridData` is an array of
    /// `iGridSize * iGridSize` 16-byte `BSSimpleArray<unsigned short>`, one
    /// per cell, listing the triangles that touch the cell.
    pub struct NavMeshGrid: 0x28 {
        /// `iGridSize` (Xbox PDB): cells per side.
        0x00 iGridSize: u32,
        /// `fColumnSectionLen` (Xbox PDB).
        0x04 fColumnSectionLen: f32,
        /// `fRowSectionLen` (Xbox PDB).
        0x08 fRowSectionLen: f32,
        /// `GridBoundsMin.x` (Xbox PDB).
        0x0C GridBoundsMin_x: f32,
        /// `GridBoundsMin.y` (Xbox PDB).
        0x10 GridBoundsMin_y: f32,
        /// `GridBoundsMin.z` (Xbox PDB).
        0x14 GridBoundsMin_z: f32,
        /// `GridBoundsMax.x` (Xbox PDB).
        0x18 GridBoundsMax_x: f32,
        /// `GridBoundsMax.y` (Xbox PDB).
        0x1C GridBoundsMax_y: f32,
        /// `GridBoundsMax.z` (Xbox PDB).
        0x20 GridBoundsMax_z: f32,
        /// `GridData` (Xbox PDB): the cell array.
        0x24 GridData: Ptr,
    }

    /// `NavMeshStaticAvoidNode` (Xbox PDB), 0x28 bytes (a `PathingAvoidNode`
    /// followed by the triangle).
    pub struct NavMeshStaticAvoidNode: 0x28 {
        /// `Point1` (Xbox PDB): first of three floats.
        0x00 Point1_x: f32,
        /// `Point2` (Xbox PDB): first of three floats.
        0x0C Point2_x: f32,
        /// `fRadius` (Xbox PDB).
        0x18 fRadius: f32,
        /// `fCost` (Xbox PDB).
        0x1C fCost: f32,
        /// `eType` (Xbox PDB): `PathingAvoidNode::AVOID_NODE_TYPE`.
        0x20 eType: u32,
        /// `usTriangle` (Xbox PDB).
        0x24 usTriangle: u16,
    }

    /// `EdgeExtraInfo` (Xbox PDB), 0x0C bytes.
    pub struct EdgeExtraInfo: 0x0C {
        /// `eType` (Xbox PDB).
        0x00 eType: u32,
        /// `Portal.pOtherMesh` (Xbox PDB): `NavMeshInfo*`.
        0x04 pOtherMesh: Ptr,
        /// `Portal.sTriangle` (Xbox PDB): triangle index in the other mesh.
        0x08 sTriangle: u16,
    }
}

/// `NavMesh` vtable, `TESChildCell` vtable (slot 0 is `0068ef50`) and
/// `NiRefObject` vtable, stored by the constructor and destructor.
const NAV_MESH_VTABLE: u32 = 0x0106_a0b4;
const CHILD_CELL_VTABLE: u32 = 0x0106_a0a8;
const REF_OBJECT_VTABLE: u32 = 0x0106_a09c;
/// Offsets of the `TESChildCell` and `NiRefObject` bases inside a `NavMesh`.
const CHILD_CELL_BASE: u32 = 0x18;
const REF_OBJECT_BASE: u32 = 0x1c;
/// `TESForm::cFormType` stored by the constructor.
const NAV_MESH_FORM_TYPE: u32 = 0x43;
/// Bucket count of the triangle-to-obstacle map and the POV map.
const OBSTACLE_MAP_BUCKETS: u32 = 0x25;
/// Size of a `NavMesh`, for the allocator's free call.
const NAV_MESH_SIZE: u32 = 0x108;
/// Offset of the `MeshGrid` (`NavMeshGrid`, 0x28 bytes) inside a `NavMesh`.
const MESH_GRID_OFFSET: u32 = 0xa8;

// `NavMeshInfo` (Xbox PDB; a class of `navmeshinfo.cpp`): offsets of the
// fields these functions read.
/// `ParentSpaceID` (`u32`).
const INFO_PARENT_SPACE_ID: u32 = 0x04;
/// `uiFlags`; bit 0x10 is tested by `0068f320`.
const INFO_FLAGS: u32 = 0x08;
/// `pParentSpace` (`TESForm*`).
const INFO_PARENT_SPACE: u32 = 0x1c;
/// `pNavMesh` (`NavMesh*`).
const INFO_NAV_MESH: u32 = 0x54;

/// Triangle flag bits 0 to 2: edge extra info.
const EDGE_INFO_FLAGS: u32 = 0x7;
/// Flag bits `00690770` tests (the cover flags).
const COVER_FLAGS: u32 = 0x0fbe_0000;
/// Flag bit `00690e70` tests.
const TRIANGLE_FLAG_0X400: u32 = 0x400;
/// "No neighbour" in a triangle's neighbour slot.
const NO_NEIGHBOUR: u16 = 0xffff;

// Values the mesh check reads from the exe's data.
/// Largest vertex coordinate magnitude (`double`, 1.00000000376832e14).
const MAX_COORDINATE: u32 = 0x0106_ab98;
/// The `double` 0.0 the triangle normal's z is compared with.
const ZERO: u32 = 0x0101_2060;
/// The `double` 4096.0 (width of an exterior cell) added to a cell origin.
const CELL_WIDTH: u32 = 0x0101_7a10;

/// Type descriptors `00690800` casts between (`.?AVTESForm@@`,
/// `.?AVTESWorldSpace@@`).
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_WORLD_SPACE: u32 = 0x0118_3fd0;
/// The `TES` instance pointer (`TES::GetNavMeshInfoMap` is called on it).
const TES_INSTANCE: u32 = 0x011d_ea10;

// Functions of other units this one calls.
/// `operator new`.
const OPERATOR_NEW: u32 = 0x0040_1000;
/// Allocator free (cdecl: block, size).
const FREE_BLOCK: u32 = 0x00aa_1460;
/// `_finite` (cdecl, one `double`).
const FINITE: u32 = 0x00ec_7595;
/// `__RTDynamicCast`.
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// `TESForm::TESForm` and `TESForm::~TESForm`.
const TES_FORM_CONSTRUCTOR: u32 = 0x0048_3370;
const TES_FORM_DESTRUCTOR: u32 = 0x0048_3630;
/// Constructor of the `TESChildCell` base; constructor and destructor of the
/// `NiRefObject` base.
const CHILD_CELL_CONSTRUCTOR: u32 = 0x0053_3240;
const REF_OBJECT_CONSTRUCTOR: u32 = 0x0049_68b0;
const REF_OBJECT_DESTRUCTOR: u32 = 0x0049_6910;
/// Stores its argument as the form type byte (`TESForm::cFormType`, +0x04).
const SET_FORM_TYPE: u32 = 0x004f_15a0;
/// `BSSimpleArray` size getter (`iSize`, +8) and clear (argument 1: free the
/// buffer).
const ARRAY_SIZE: u32 = 0x0044_ddc0;
const ARRAY_CLEAR: u32 = 0x0084_54f0;
// Constructors and destructors of the mesh's members (all in this unit,
// not yet translated).
const ARRAY_CONSTRUCTOR_VERTICES: u32 = 0x0069_1a60;
const ARRAY_DESTRUCTOR_VERTICES: u32 = 0x0069_1a90;
const ARRAY_CONSTRUCTOR_TRIANGLES: u32 = 0x0069_1b10;
const ARRAY_DESTRUCTOR_TRIANGLES: u32 = 0x0069_1b40;
const ARRAY_CONSTRUCTOR_EXTRA_INFO: u32 = 0x0069_1bc0;
const ARRAY_DESTRUCTOR_EXTRA_INFO: u32 = 0x0069_1bf0;
const ARRAY_CONSTRUCTOR_DOOR_PORTALS: u32 = 0x0069_1d40;
const ARRAY_DESTRUCTOR_DOOR_PORTALS: u32 = 0x0069_1d70;
const ARRAY_CONSTRUCTOR_CLOSED_DOORS: u32 = 0x0069_1d90;
const ARRAY_DESTRUCTOR_CLOSED_DOORS: u32 = 0x0069_1dc0;
/// Constructor and destructor shared by the `u16` arrays (cover, closest
/// POVs, obstacle POVs).
const ARRAY_CONSTRUCTOR_U16: u32 = 0x0069_17d0;
const ARRAY_DESTRUCTOR_U16: u32 = 0x0069_1800;
/// Constructor (argument: bucket count) and destructor of the POV map.
const MAP_CONSTRUCTOR: u32 = 0x0069_1720;
const MAP_DESTRUCTOR: u32 = 0x0069_1ea0;
/// Constructor and clear (`0069d2d0`) of the search grid.
const GRID_CONSTRUCTOR: u32 = 0x0069_d280;
const GRID_CLEAR: u32 = 0x0069_d2d0;
/// Constructor, clear (argument 1: free the buffer) and destructor of the
/// obstacle array.
const OBSTACLES_CONSTRUCTOR: u32 = 0x0069_1f60;
const OBSTACLES_CLEAR: u32 = 0x006c_6200;
const OBSTACLES_DESTRUCTOR: u32 = 0x0069_1f90;
/// Constructor and destructor of the static avoid node array.
const AVOID_NODES_CONSTRUCTOR: u32 = 0x0069_1fb0;
const AVOID_NODES_DESTRUCTOR: u32 = 0x0069_1fe0;
/// Constructor of the triangle-to-obstacle map object (argument: buckets).
const OBSTACLE_MAP_CONSTRUCTOR: u32 = 0x0069_16f0;
/// Stores its argument in `NavMeshInfo::pNavMesh` (+0x54).
const INFO_SET_NAV_MESH: u32 = 0x005a_8040;
/// `BSSimpleArray` element address for 12-byte elements (`buffer + index *
/// 12`): used for the vertex and the extra-info arrays.
const ELEMENT_12: u32 = 0x006a_1440;
/// `BSSimpleArray` element address for 16-byte elements (the triangles).
const TRIANGLE_ELEMENT: u32 = 0x0055_8dd0;
/// `NavMesh::GetTriangle(index)`: `Triangles[index]` (through `00558dd0`).
const NAV_MESH_GET_TRIANGLE: u32 = 0x0055_82f0;
/// `NavMeshTriangle::GetVertex(i)`: the `u16` vertex index `i`.
const TRIANGLE_VERTEX: u32 = 0x0055_82d0;
/// `NavMesh::GetTriangleCount` (`Triangles.iSize`, through `0044ddc0`).
const NAV_MESH_TRIANGLE_COUNT: u32 = 0x0055_8200;
/// `TESObjectCELL` virtual slot 0x90: `GetFormDetailedString(BSString*)`.
const GET_FORM_DETAILED_STRING_SLOT: u32 = 0x90;
/// Returns a `NavMesh`'s `pParentCell` (+0x24).
const GET_PARENT_CELL: u32 = 0x0059_bb30;
/// `TESObjectCELL::GetWorldSpace`, the cell's interior test, and
/// `GetDataX` / `GetDataY` (exterior cell coordinates).
const CELL_GET_WORLD_SPACE: u32 = 0x0054_ddd0;
const CELL_IS_INTERIOR: u32 = 0x0042_5fd0;
const CELL_GET_X: u32 = 0x0054_4c30;
const CELL_GET_Y: u32 = 0x0054_4c60;
/// `TESWorldSpace::GetCellFromCellCoord(x, y)`.
const WORLD_SPACE_GET_CELL: u32 = 0x0058_75a0;
/// Cell's navmesh array getter (`pNavMeshes`, +0x64 on PC).
const CELL_GET_NAV_MESHES: u32 = 0x0070_ec90;
/// `NavMeshArray::GetNavMeshByIndex(out NiPointer, index)` (returns `out`).
const NAV_MESH_ARRAY_GET: u32 = 0x0046_4f60;
/// Size of the cell's navmesh array.
const NAV_MESH_ARRAY_SIZE: u32 = 0x0062_0b80;
// `NiPointer` helpers: constructor from a raw pointer (`this`, pointer),
// the same as a cdecl function (`out`, pointer; returns `out`), default
// constructor, assignment (`this`, other), destructor, and the read of the
// pointer held (the word at +0, which also reads a `BSString`'s characters
// or the first word of any object).
const POINTER_CONSTRUCT_FROM: u32 = 0x0046_4ff0;
const POINTER_CONSTRUCT_FROM_CDECL: u32 = 0x0046_4fc0;
const POINTER_CONSTRUCT: u32 = 0x0042_fb00;
const POINTER_ASSIGN: u32 = 0x0042_f4c0;
const POINTER_DESTRUCT: u32 = 0x0042_fa40;
const POINTER_GET: u32 = 0x0055_9450;
/// The held pointer of an `NiPointer<NavMesh>` (calls `POINTER_GET`).
const POINTER_DEREFERENCE: u32 = 0x0045_8b50;
/// Debug-print stub the mesh check reports through (cdecl: format string,
/// then its arguments; returns 0 and prints nothing in this build).
const DEBUG_PRINT: u32 = 0x005b_5e40;
/// Debug stub called for every adjacent-cell navmesh (cdecl, five words:
/// the mesh, the neighbour mesh, a direction flag and two coordinates); it
/// always returns true.
const NEIGHBOUR_CHECK: u32 = 0x005d_4a40;
/// `BSString` constructor and destructor (the cell text).
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
const STRING_DESTRUCT: u32 = 0x0040_37d0;
/// Returns `TESForm::iFormID` (+0x0C).
const GET_FORM_ID: u32 = 0x0084_e3a0;
/// `fabs` for a `float` argument (result in ST0).
const FLOAT_ABS: u32 = 0x0040_8840;
/// Triangle normal into an out `NiPoint3` (`this`, out, triangle index).
const TRIANGLE_NORMAL: u32 = 0x0069_71e0;
/// Flips a triangle (`this`, triangle index).
const FLIP_TRIANGLE: u32 = 0x0069_5660;
/// `BSSimpleArray<unsigned short>` element address (`buffer + index * 2`).
const U16_ELEMENT: u32 = 0x005e_eda0;
/// Removes `count` elements from `index` of a 12-byte element array (the
/// array moves its last elements into the gap).
const ARRAY_REMOVE_12: u32 = 0x0069_1c90;
/// Appends a copy of a 12-byte element to its array, returns the index.
const ARRAY_ADD_12: u32 = 0x0069_1ab0;
/// Appends a copy of a 16-byte element to its array, returns the index.
const ARRAY_ADD_16: u32 = 0x0069_1b60;
/// `DoorPortals` element getter.
const DOOR_PORTAL_ELEMENT: u32 = 0x0084_4160;
/// `TES::GetNavMeshInfoMap` and `NavMeshInfoMap::DeleteNavMeshInfo(id)`.
const GET_NAV_MESH_INFO_MAP: u32 = 0x0045_af00;
const DELETE_NAV_MESH_INFO: u32 = 0x006b_6a00;
/// Looks a form up by its id (cdecl).
const LOOKUP_FORM: u32 = 0x0048_39c0;

// Functions of other units and not yet translated functions of this one,
// called by the second batch (`00691020` to `00691bf0`).
/// `_ftol2_sse`: truncates the `f64` it is given (the x87 value).
const FLOAT_TO_INT: u32 = 0x00ec_62c0;
/// Cdecl, one `float`; returns a `float` in ST0 (`floor`, through `00ec6940`).
const FLOAT_FLOOR: u32 = 0x0040_6ce0;
/// Cdecl `(int* value, int low, int high)`: clamps `*value` to the range.
const CLAMP_INT: u32 = 0x0064_7ba0;
/// Cdecl, two words: the larger / the smaller of two unsigned values.
const UNSIGNED_MAX: u32 = 0x0040_3940;
const UNSIGNED_MIN: u32 = 0x0042_f5a0;
/// `(array, unsigned short value)` membership test of one grid cell.
const GRID_CELL_CONTAINS: u32 = 0x0069_da10;
/// Appends the 4-byte value at its argument to a `BSSimpleArray` of words.
const ARRAY_PUSH_WORD: u32 = 0x007c_b2e0;
/// Appends a copy of a 40-byte avoid node to the static avoid node array.
const AVOID_NODE_ADD: u32 = 0x0069_2000;
/// Appends a copy of a 12-byte `EdgeExtraInfo` to its array.
const EXTRA_INFO_ADD: u32 = 0x0069_1c10;
/// Makes room for one more `unsigned short` element and returns its index.
const U16_ARRAY_GROW: u32 = 0x0069_2200;
/// Constructs `count` `unsigned short` elements at an address of the array.
const U16_ARRAY_CONSTRUCT_ELEMENTS: u32 = 0x005e_f650;
/// Makes room for one more 12-byte (vertex) element and returns its index,
/// and constructs elements there.
const VERTEX_ARRAY_GROW: u32 = 0x0097_8bc0;
const VERTEX_ARRAY_CONSTRUCT_ELEMENTS: u32 = 0x0069_b9d0;
/// The same for the 16-byte triangle array.
const TRIANGLE_ARRAY_GROW: u32 = 0x006f_31f0;
const TRIANGLE_ARRAY_CONSTRUCT_ELEMENTS: u32 = 0x0064_4490;
/// Initialisers of the member arrays: `(array, 0, 0)`.
const U16_ARRAY_INITIALISE: u32 = 0x0069_2290;
const VERTEX_ARRAY_INITIALISE: u32 = 0x0069_ba70;
const TRIANGLE_ARRAY_INITIALISE: u32 = 0x0064_4400;
const EXTRA_INFO_ARRAY_INITIALISE: u32 = 0x0069_23f0;
/// Constructors of the base of the two `NiTMap` instances (`NiPointer<
/// ObstacleData>` and `NavMeshPOVData` values), and their destructor bodies.
const POV_MAP_BASE_CONSTRUCTOR: u32 = 0x0069_1de0;
const OBSTACLE_MAP_DESTRUCTOR_BODY: u32 = 0x0069_19a0;
const POV_MAP_DESTRUCTOR_BODY: u32 = 0x0069_1ea0;
/// Constructor of a `NiPoint3` (does nothing, returns `this`).
const POINT_CONSTRUCTOR: u32 = 0x0068_15c0;
/// `operator delete` (cdecl, one word).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// Cdecl `(size)`: allocates `size` bytes.
const ALLOCATE: u32 = 0x00aa_1070;
/// Cdecl `(block, size)`: frees a block of an array.
const DEALLOCATE_SIZED: u32 = 0x0042_f5d0;
/// Cdecl `(destination, value, size)`: `memset`.
const MEMORY_SET: u32 = 0x0040_3d30;
/// `NiPointer::operator=(NiPointer*)` and the `NiPointer` destructor.
const POINTER_ASSIGN_FROM: u32 = 0x006e_5cc0;
const POINTER_RELEASE: u32 = 0x0045_cec0;
/// Returns the `usTriangle` field (+0x24) of a static avoid node.
const AVOID_NODE_TRIANGLE: u32 = 0x006d_f760;
/// `iSize == 0` of a `BSSimpleArray`.
const ARRAY_IS_EMPTY: u32 = 0x0076_b610;

// Constants of the third batch (`00691c10` to `00692740`).
/// Vtables stored by the door portal, closed door, map base, obstacle and
/// static avoid node array constructors and destructors.
const DOOR_PORTAL_ARRAY_VTABLE: u32 = 0x0106_ac54;
const CLOSED_DOOR_ARRAY_VTABLE: u32 = 0x0106_ac68;
const MAP_ITEM_BASE_VTABLE: u32 = 0x0106_ac7c;
const OBSTACLE_ARRAY_VTABLE: u32 = 0x0106_ac9c;
const AVOID_NODE_ARRAY_VTABLE: u32 = 0x0106_acb0;
/// Extra edge info array: makes room (`00978bc0`, as for the vertices),
/// constructs `count` elements at an address, copy-constructs one element
/// (`this` = the new element, argument = the source).
const EXTRA_INFO_ARRAY_GROW: u32 = 0x0097_8bc0;
const EXTRA_INFO_CONSTRUCT_ELEMENTS: u32 = 0x0069_2320;
const EXTRA_INFO_COPY_CONSTRUCT: u32 = 0x0069_1c60;
/// Initialiser `(array, 0, 0)` of the door portal and closed door arrays,
/// and of the obstacle array.
const DOOR_ARRAY_INITIALISE: u32 = 0x0069_2520;
const OBSTACLES_INITIALISE: u32 = 0x0082_2860;
/// Static avoid node array: makes room for one more element (returns the
/// index) and constructs `count` elements at an address.
const AVOID_NODE_ARRAY_GROW: u32 = 0x0069_25b0;
const AVOID_NODE_CONSTRUCT_ELEMENTS: u32 = 0x0069_2640;
/// Map base destructor step called twice by the POV map destructors (frees
/// the items), and the free of the bucket table (cdecl, one word).
const MAP_REMOVE_ALL: u32 = 0x0043_8af0;
const FREE_TABLE: u32 = 0x00aa_10f0;
/// Item functions of the POV map: reset the value stored at `item + 6` (one
/// argument, 0) and give an item back to the map's allocator (`this` =
/// `map + 0xc`).
const POV_ITEM_RESET_VALUE: u32 = 0x0065_de30;
const POV_ITEM_FREE: u32 = 0x006b_8310;
/// `BSSimpleArray<unsigned short>` growth step: true when the array has no
/// room left, the next capacity, and the resize `(new capacity, count)`.
const U16_ARRAY_IS_FULL: u32 = 0x0043_8b90;
const U16_ARRAY_NEXT_CAPACITY: u32 = 0x009a_3910;
const U16_ARRAY_RESIZE: u32 = 0x005e_f6a0;
/// Destructor body the obstacle map's scalar deleting destructor
/// (`00692050`) runs.
const OBSTACLE_MAP_BASE_DESTRUCTOR: u32 = 0x0069_1a00;
/// Virtual slot (byte offset) of the `BSSimpleArray` allocator
/// `(count) -> block`.
const ARRAY_ALLOCATE_SLOT: u32 = 0x04;
/// Initial capacity of a `BSSimpleArray<unsigned short>` that has none.
const U16_ARRAY_FIRST_CAPACITY: u32 = 4;

// Vtables the constructors of this batch store.
const OBSTACLE_MAP_VTABLE: u32 = 0x0106_aba4;
const POV_MAP_VTABLE: u32 = 0x0106_abc4;
const U16_ARRAY_VTABLE: u32 = 0x0106_abe4;
const MAP_BASE_VTABLE: u32 = 0x0106_abf8;
const VERTEX_ARRAY_VTABLE: u32 = 0x0106_ac18;
const TRIANGLE_ARRAY_VTABLE: u32 = 0x0106_ac2c;
const EXTRA_INFO_ARRAY_VTABLE: u32 = 0x0106_ac40;

/// Triangle flag bit `006910a0` tests before listing a triangle as cover.
const TRIANGLE_FLAG_0X20: u32 = 0x20;
/// Triangle flag bit set on the triangle of a static avoid node.
const TRIANGLE_FLAG_HAS_AVOID_NODE: u32 = 0x8000_0000;

// Format strings of the problems `CheckNavMesh` reports, in the order the
// function checks them. All start with `"PATHFINDING: Navmesh %08x Cell %s,
// "` and are printed with the navmesh's form id, the cell's text and the
// numbers listed.
/// `"has bad vertex X coordinate, Please regenerate"` and the Y, Z forms.
const FORMAT_BAD_X: u32 = 0x0106_ab40;
const FORMAT_BAD_Y: u32 = 0x0106_aae8;
const FORMAT_BAD_Z: u32 = 0x0106_aa90;
/// `"has a large number of triangles, %d, please check and optimize"`.
const FORMAT_MANY_TRIANGLES: u32 = 0x0106_aa28;
/// `"Triangle %d, edge %d has bad Triangle index, Clearing the connection"`.
const FORMAT_BAD_TRIANGLE_INDEX: u32 = 0x0106_a9c0;
/// `"Triangle %d, has a downfacing normal, flipping the triangle"`.
const FORMAT_DOWNFACING: u32 = 0x0106_a960;
/// `"Triangle %d, edge %d has bad extra info index, Clearing extra info flag"`.
const FORMAT_BAD_EXTRA_INFO_INDEX: u32 = 0x0106_a8f0;
/// `"Triangle %d, edge %d has bad Portal (Navmesh %08x does not have a
/// triangle index %d), Clearing the portal"`.
const FORMAT_BAD_PORTAL_TRIANGLE: u32 = 0x0106_a860;
/// `"Triangle %d, edge %d has a Portal to a Navmesh (%08x) in a different
/// Worldspace, Clearing the portal"`.
const FORMAT_PORTAL_OTHER_WORLD_SPACE: u32 = 0x0106_a7d8;
/// `"Triangle %d is degenerate, Vertices 0 and 1 both use vertex index %d"`
/// and the (0, 2) and (1, 2) forms.
const FORMAT_DEGENERATE_01: u32 = 0x0106_a770;
const FORMAT_DEGENERATE_02: u32 = 0x0106_a708;
const FORMAT_DEGENERATE_12: u32 = 0x0106_a6a0;
/// `"Triangle %d Edges 0 and 1 both point to the same triangle %d"` and the
/// (0, 2) and (1, 2) forms.
const FORMAT_SAME_NEIGHBOUR_01: u32 = 0x0106_a640;
const FORMAT_SAME_NEIGHBOUR_02: u32 = 0x0106_a5e0;
const FORMAT_SAME_NEIGHBOUR_12: u32 = 0x0106_a580;
/// `"Triangle %d and %d have opposite normals but are linked"`.
const FORMAT_OPPOSITE_NORMALS: u32 = 0x0106_a520;
/// `"Triangle %d Edge %d has a portal data to Triangle %d, but Triangle %d
/// doesn't have any Portal info for its matching edge %d"`.
const FORMAT_PORTAL_WITHOUT_MATCH: u32 = 0x0106_a480;
/// `"Triangle %d Edge %d Should have a link to Triangle %d, but doesn't"`.
const FORMAT_MISSING_LINK: u32 = 0x0106_a418;
/// `"Triangle %d Edge %d Should have Portal information, but doesn't"`.
const FORMAT_MISSING_PORTAL: u32 = 0x0106_a3b0;
/// `"Triangle %d has a link to Triangle %d, but Triangle %d doesn't have the
/// matching link"`.
const FORMAT_LINK_NOT_RETURNED: u32 = 0x0106_a330;
/// `"Triangles %d and %d are linked, but their vertices do not match"`.
const FORMAT_VERTICES_MISMATCH: u32 = 0x0106_a2c8;
/// `"Cover Array (index %d) referes to triangle %d, which does not exist."`
/// and the `"... which does not have cover."` form.
const FORMAT_COVER_MISSING_TRIANGLE: u32 = 0x0106_a260;
const FORMAT_COVER_WITHOUT_COVER: u32 = 0x0106_a1f0;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Address of `Triangles[edge]` (the neighbour slot) of a triangle.
fn neighbour_slot(tri: Ptr<NavMeshTriangle>, edge: i32) -> u32 {
    tri.addr()
        .wrapping_add(6)
        .wrapping_add((edge as u32).wrapping_mul(2))
}

/// `NavMesh::GetTriangle` (`005582f0`).
fn mesh_triangle(e: &mut Engine, mesh: Ptr<NavMesh>, index: u16) -> Ptr<NavMeshTriangle> {
    e.call(NAV_MESH_GET_TRIANGLE, &args![mesh, index]).ptr()
}

/// `NavMeshTriangle::GetVertex` (`005582d0`).
fn triangle_vertex(e: &mut Engine, tri: Ptr<NavMeshTriangle>, index: i32) -> u16 {
    e.call(TRIANGLE_VERTEX, &args![tri, index]).u16()
}

/// `BSSimpleArray::iSize` (`0044ddc0`).
fn array_size<T>(e: &mut Engine, array: Ptr<T>) -> u32 {
    e.call(ARRAY_SIZE, &args![array]).u32()
}

/// `Triangles[index]` through the triangle array's element getter
/// (`00558dd0`), as `CheckNavMesh` and `DeleteExtraEdgeInfo` call it.
fn triangle_element(e: &mut Engine, mesh: Ptr<NavMesh>, index: u32) -> Ptr<NavMeshTriangle> {
    let triangles = mesh.at(NavMesh::Triangles);
    e.call(TRIANGLE_ELEMENT, &args![triangles, index]).ptr()
}

// Translated from 0049c4c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<DFALL<NiTMapItem<unsigned short, NiPointer<ObstacleData>>>,
/// unsigned short, NiPointer<ObstacleData>>::KeyToHashIndex` (Xbox PDB): the
/// bucket of a 16-bit key, `key % m_uiHashSize`.
pub fn ni_t_map_base_key_to_hash_index(e: &mut Engine, this: Ptr<NiTPointerMap>, key: u16) -> u32 {
    let hash_size = e.get(this, NiTPointerMap::m_uiHashSize);
    key as u32 % hash_size
}

// Translated from 0068eb80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMesh::NavMesh` (Xbox PDB): constructs the form and its two bases,
/// stores the three vtables, constructs every array and map member, sets the
/// form type to `0x43`, clears the parent cell and info pointers, and
/// allocates the triangle-to-obstacle map (16 bytes, 0x25 buckets; null when
/// the allocation fails). Returns `this`.
pub fn nav_mesh_nav_mesh(e: &mut Engine, this: Ptr<NavMesh>) -> Ptr<NavMesh> {
    e.call(TES_FORM_CONSTRUCTOR, &args![this]);
    e.call(
        CHILD_CELL_CONSTRUCTOR,
        &args![this.byte_add(CHILD_CELL_BASE)],
    );
    e.call(
        REF_OBJECT_CONSTRUCTOR,
        &args![this.byte_add(REF_OBJECT_BASE)],
    );
    e.mem.set_u32(this.addr(), NAV_MESH_VTABLE);
    e.mem
        .set_u32(this.addr() + CHILD_CELL_BASE, CHILD_CELL_VTABLE);
    e.mem
        .set_u32(this.addr() + REF_OBJECT_BASE, REF_OBJECT_VTABLE);
    e.call(
        ARRAY_CONSTRUCTOR_VERTICES,
        &args![this.at(NavMesh::Vertices)],
    );
    e.call(
        ARRAY_CONSTRUCTOR_TRIANGLES,
        &args![this.at(NavMesh::Triangles)],
    );
    e.call(
        ARRAY_CONSTRUCTOR_EXTRA_INFO,
        &args![this.at(NavMesh::ExtraEdgeInfo)],
    );
    e.call(
        ARRAY_CONSTRUCTOR_DOOR_PORTALS,
        &args![this.at(NavMesh::DoorPortals)],
    );
    e.call(
        ARRAY_CONSTRUCTOR_CLOSED_DOORS,
        &args![this.at(NavMesh::ClosedDoors)],
    );
    e.call(ARRAY_CONSTRUCTOR_U16, &args![this.at(NavMesh::CoverArray)]);
    e.call(
        MAP_CONSTRUCTOR,
        &args![this.at(NavMesh::POVs), OBSTACLE_MAP_BUCKETS],
    );
    e.call(ARRAY_CONSTRUCTOR_U16, &args![this.at(NavMesh::ClosestPOVs)]);
    e.call(GRID_CONSTRUCTOR, &args![this.byte_add(MESH_GRID_OFFSET)]);
    e.call(OBSTACLES_CONSTRUCTOR, &args![this.at(NavMesh::Obstacles)]);
    e.call(
        ARRAY_CONSTRUCTOR_U16,
        &args![this.at(NavMesh::ObstaclePOVs)],
    );
    e.call(
        AVOID_NODES_CONSTRUCTOR,
        &args![this.at(NavMesh::StaticAvoidNodes)],
    );
    e.call(SET_FORM_TYPE, &args![this, NAV_MESH_FORM_TYPE]);
    e.set(this, NavMesh::pParentCell, Ptr::NULL);
    e.set(this, NavMesh::pNavMeshInfo, Ptr::NULL);
    let block = e.call(OPERATOR_NEW, &args![0x10u32]).ptr::<()>();
    let map = if block.is_null() {
        Ptr::NULL
    } else {
        e.call(
            OBSTACLE_MAP_CONSTRUCTOR,
            &args![block, OBSTACLE_MAP_BUCKETS],
        )
        .ptr::<()>()
    };
    e.set(this, NavMesh::pTriangleToObstacleMap, map);
    this
}

// Translated from 0068ed30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NavMesh` scalar deleting destructor: runs the destructor `0068ed70`,
/// then frees the 0x108-byte object when bit 0 of `flags` is set. Returns
/// `this`.
pub fn fn_0068ed30(e: &mut Engine, this: Ptr<NavMesh>, flags: u32) -> Ptr<NavMesh> {
    fn_0068ed70(e, this);
    if flags & 1 != 0 {
        e.call(FREE_BLOCK, &args![this, NAV_MESH_SIZE]);
    }
    this
}

// Translated from 0068ed70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NavMesh` destructor body: restores the three vtables, empties the
/// vertex, triangle, extra-info, door-portal and cover arrays and the grid,
/// detaches itself from its `NavMeshInfo` (`pNavMesh` = null), deletes the
/// triangle-to-obstacle map through its virtual destructor, then destroys
/// the members in reverse order and finally the `NiRefObject` base and the
/// form.
pub fn fn_0068ed70(e: &mut Engine, this: Ptr<NavMesh>) {
    e.mem.set_u32(this.addr(), NAV_MESH_VTABLE);
    e.mem
        .set_u32(this.addr() + CHILD_CELL_BASE, CHILD_CELL_VTABLE);
    e.mem
        .set_u32(this.addr() + REF_OBJECT_BASE, REF_OBJECT_VTABLE);
    for array in [
        this.at(NavMesh::Vertices),
        this.at(NavMesh::Triangles),
        this.at(NavMesh::ExtraEdgeInfo),
        this.at(NavMesh::DoorPortals),
        this.at(NavMesh::CoverArray),
    ] {
        e.call(ARRAY_CLEAR, &args![array, 1u32]);
    }
    e.call(GRID_CLEAR, &args![this.byte_add(MESH_GRID_OFFSET)]);
    let info = e.get(this, NavMesh::pNavMeshInfo);
    if !info.is_null() {
        e.call(INFO_SET_NAV_MESH, &args![info, 0u32]);
    }
    let map = e.get(this, NavMesh::pTriangleToObstacleMap);
    if !map.is_null() {
        // Scalar deleting destructor (slot 0) of the map, with delete.
        e.vcall(map.addr(), 0, &args![1u32]);
    }
    e.call(
        AVOID_NODES_DESTRUCTOR,
        &args![this.at(NavMesh::StaticAvoidNodes)],
    );
    e.call(ARRAY_DESTRUCTOR_U16, &args![this.at(NavMesh::ObstaclePOVs)]);
    e.call(OBSTACLES_DESTRUCTOR, &args![this.at(NavMesh::Obstacles)]);
    e.call(ARRAY_DESTRUCTOR_U16, &args![this.at(NavMesh::ClosestPOVs)]);
    e.call(MAP_DESTRUCTOR, &args![this.at(NavMesh::POVs)]);
    e.call(ARRAY_DESTRUCTOR_U16, &args![this.at(NavMesh::CoverArray)]);
    e.call(
        ARRAY_DESTRUCTOR_CLOSED_DOORS,
        &args![this.at(NavMesh::ClosedDoors)],
    );
    e.call(
        ARRAY_DESTRUCTOR_DOOR_PORTALS,
        &args![this.at(NavMesh::DoorPortals)],
    );
    e.call(
        ARRAY_DESTRUCTOR_EXTRA_INFO,
        &args![this.at(NavMesh::ExtraEdgeInfo)],
    );
    e.call(
        ARRAY_DESTRUCTOR_TRIANGLES,
        &args![this.at(NavMesh::Triangles)],
    );
    e.call(
        ARRAY_DESTRUCTOR_VERTICES,
        &args![this.at(NavMesh::Vertices)],
    );
    e.call(
        REF_OBJECT_DESTRUCTOR,
        &args![this.byte_add(REF_OBJECT_BASE)],
    );
    e.call(TES_FORM_DESTRUCTOR, &args![this]);
}

// Translated from 0068ef50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESChildCell::GetSaveParentCell` (Xbox PDB name of slot 0 of the
/// `TESChildCell` base, whose vtable `0106a0a8` points here): `this` is the
/// base at `NavMesh + 0x18`; returns the navmesh's parent cell (`0059bb30` on
/// the `NavMesh`).
pub fn fn_0068ef50(e: &mut Engine, this: Ptr) -> u32 {
    let mesh = Ptr::<NavMesh>::new(this.addr().wrapping_sub(CHILD_CELL_BASE));
    e.call(GET_PARENT_CELL, &args![mesh]).u32()
}

// Translated from 0068ef70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMesh::GetParentSpace` (Xbox PDB): the worldspace of the parent cell
/// if it has one, else the parent cell itself; null without a parent cell.
pub fn nav_mesh_get_parent_space(e: &mut Engine, this: Ptr<NavMesh>) -> Ptr {
    let cell = e.get(this, NavMesh::pParentCell);
    if cell.is_null() {
        return Ptr::NULL;
    }
    let world_space = e.call(CELL_GET_WORLD_SPACE, &args![cell]).u32();
    if world_space != 0 {
        e.call(CELL_GET_WORLD_SPACE, &args![cell]).ptr()
    } else {
        cell
    }
}

// Translated from 0068efb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the navmesh's `NavMeshInfo` (`pNavMeshInfo`, +0x104).
pub fn fn_0068efb0(e: &mut Engine, this: Ptr<NavMesh>) -> Ptr {
    e.get(this, NavMesh::pNavMeshInfo)
}

// Translated from 0068efd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The two vertex indexes of edge `edge` of triangle `triangle`: stores
/// them as two consecutive `u16` (vertex `edge`, then vertex `(edge + 1) % 3`)
/// in the 4 bytes at `result`, and returns `result`.
pub fn fn_0068efd0(
    e: &mut Engine,
    this: Ptr<NavMesh>,
    result: Ptr,
    triangle: u16,
    edge: i32,
) -> Ptr {
    let tri = mesh_triangle(e, this, triangle);
    let first = triangle_vertex(e, tri, edge);
    let next = (edge + 1) % 3;
    let tri = mesh_triangle(e, this, triangle);
    let second = triangle_vertex(e, tri, next);
    e.mem
        .set_u32(result.addr(), first as u32 | (second as u32) << 16);
    result
}

// Translated from 0068f040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMesh::GetEdgeVertices` (Xbox PDB): stores at `result` the addresses
/// of the two vertices that bound edge `edge` of triangle `triangle`, and
/// returns `result`.
pub fn nav_mesh_get_edge_vertices(
    e: &mut Engine,
    this: Ptr<NavMesh>,
    result: Ptr,
    triangle: u16,
    edge: i32,
) -> Ptr {
    e.with_stack(4, |e, pair| {
        fn_0068efd0(e, this, pair, triangle, edge);
        let first = e.mem.u16(pair.addr());
        let second = e.mem.u16(pair.addr() + 2);
        let first_vertex = fn_0068f0a0(e, this, first);
        let second_vertex = fn_0068f0a0(e, this, second);
        e.mem.set_u32(result.addr(), first_vertex.addr());
        e.mem.set_u32(result.addr() + 4, second_vertex.addr());
    });
    result
}

// Translated from 0068f0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Address of vertex `index` of the mesh (`Vertices[index]`).
pub fn fn_0068f0a0(e: &mut Engine, this: Ptr<NavMesh>, index: u16) -> Ptr {
    let vertices = this.at(NavMesh::Vertices);
    e.call(ELEMENT_12, &args![vertices, index]).ptr()
}

// Translated from 0068f0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the coordinates (three floats each) of the two vertices bounding
/// edge `edge` of triangle `triangle` to `first` and `second`. Always
/// returns true.
pub fn fn_0068f0c0(
    e: &mut Engine,
    this: Ptr<NavMesh>,
    triangle: u16,
    edge: i32,
    first: Ptr,
    second: Ptr,
) -> bool {
    let tri = mesh_triangle(e, this, triangle);
    let first_index = triangle_vertex(e, tri, edge);
    let next = (edge + 1) % 3;
    let tri = mesh_triangle(e, this, triangle);
    let second_index = triangle_vertex(e, tri, next);
    for (index, out) in [(first_index, first), (second_index, second)] {
        let vertex = fn_0068f0a0(e, this, index);
        for word in 0..3 {
            let value = e.mem.u32(vertex.addr() + 4 * word);
            e.mem.set_u32(out.addr() + 4 * word, value);
        }
    }
    true
}

// Translated from 0068f160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the addresses of the three vertices of triangle `triangle` at
/// `first`, `second` and `third`.
pub fn fn_0068f160(
    e: &mut Engine,
    this: Ptr<NavMesh>,
    triangle: u16,
    first: Ptr,
    second: Ptr,
    third: Ptr,
) {
    let tri = mesh_triangle(e, this, triangle);
    for (corner, out) in [(0, first), (1, second), (2, third)] {
        let index = triangle_vertex(e, tri, corner);
        let vertex = fn_0068f0a0(e, this, index);
        e.mem.set_u32(out.addr(), vertex.addr());
    }
}

// Translated from 0068f1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshTriangle` method: whether bit `edge` of `TriangleFlags` is set,
/// that is whether the edge has extra info (only the low five bits of
/// `edge` count, as for an x86 shift).
pub fn fn_0068f1d0(e: &mut Engine, this: Ptr<NavMeshTriangle>, edge: u32) -> bool {
    let flags = e.get(this, NavMeshTriangle::TriangleFlags);
    (1u32 << (edge & 31)) & flags != 0
}

// Translated from 0068f200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshTriangle` method: whether neighbour slot `Triangles[edge]` holds
/// something other than `0xFFFF`.
pub fn fn_0068f200(e: &mut Engine, this: Ptr<NavMeshTriangle>, edge: i32) -> bool {
    e.mem.u16(neighbour_slot(this, edge)) != NO_NEIGHBOUR
}

// Translated from 0068f230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMesh::GetEdgeExtraInfo` (Xbox PDB): the `EdgeExtraInfo` of edge
/// `edge` of triangle `triangle`, or null if the edge has no extra info
/// (flag clear, or neighbour slot `0xFFFF`).
pub fn nav_mesh_get_edge_extra_info(
    e: &mut Engine,
    this: Ptr<NavMesh>,
    triangle: u16,
    edge: i32,
) -> Ptr<EdgeExtraInfo> {
    let triangles = this.at(NavMesh::Triangles);
    let tri: Ptr<NavMeshTriangle> = e.call(TRIANGLE_ELEMENT, &args![triangles, triangle]).ptr();
    if !fn_0068f1d0(e, tri, edge as u32) {
        return Ptr::NULL;
    }
    let tri: Ptr<NavMeshTriangle> = e.call(TRIANGLE_ELEMENT, &args![triangles, triangle]).ptr();
    if fn_0068f2c0(e, tri, edge) == NO_NEIGHBOUR {
        return Ptr::NULL;
    }
    let tri: Ptr<NavMeshTriangle> = e.call(TRIANGLE_ELEMENT, &args![triangles, triangle]).ptr();
    let slot = fn_0068f2c0(e, tri, edge);
    let infos = this.at(NavMesh::ExtraEdgeInfo);
    e.call(ELEMENT_12, &args![infos, slot]).ptr()
}

// Translated from 0068f2c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshTriangle` method: the neighbour slot `Triangles[edge]`.
pub fn fn_0068f2c0(e: &mut Engine, this: Ptr<NavMeshTriangle>, edge: i32) -> u16 {
    e.mem.u16(neighbour_slot(this, edge))
}

// Translated from 0068f2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshInfo` method: stores the info's navmesh pointer (`pNavMesh`) at
/// `result`, or null when the info is flagged (`0068f320`); returns whether
/// the stored pointer is non-null.
pub fn fn_0068f2e0(e: &mut Engine, this: Ptr, result: Ptr) -> bool {
    if fn_0068f320(e, this) {
        e.mem.set_u32(result.addr(), 0);
        false
    } else {
        let mesh = e.mem.u32(this.addr() + INFO_NAV_MESH);
        e.mem.set_u32(result.addr(), mesh);
        e.mem.u32(this.addr() + INFO_NAV_MESH) != 0
    }
}

// Translated from 0068f320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshInfo` method: whether bit 0x10 of `uiFlags` is set.
pub fn fn_0068f320(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + INFO_FLAGS) & 0x10 != 0
}

// Translated from 0068f340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMesh::GetMatchingEdge` (Xbox PDB): finds the edge on the other side
/// of the edge named by `edge_ref` (a record with the triangle index as
/// `u16` at +4 and the edge number at +8), and stores the other mesh's
/// `NavMeshInfo` at `result + 0`, the other triangle at `result + 4` (`u16`)
/// and the other edge at `result + 8`. Returns whether a match exists.
pub fn nav_mesh_get_matching_edge(
    e: &mut Engine,
    this: Ptr<NavMesh>,
    edge_ref: Ptr,
    result: Ptr,
) -> bool {
    e.with_stack(12, |e, locals| {
        // The matched mesh (+0), the matched triangle (+4), the matched edge
        // (+8, starts at -1).
        e.mem.set_u32(locals.addr() + 8, 0xffff_ffff);
        let triangle = e.mem.u16(edge_ref.addr() + 4);
        let edge = e.mem.i32(edge_ref.addr() + 8);
        let found = nav_mesh_get_matching_edge_ov3(
            e,
            this,
            triangle,
            edge,
            locals,
            locals.byte_add(4),
            locals.byte_add(8),
        );
        if found {
            let mesh = e.mem.u32(locals.addr());
            let info = fn_0068efb0(e, Ptr::new(mesh));
            e.mem.set_u32(result.addr(), info.addr());
            let other_triangle = e.mem.u16(locals.addr() + 4);
            e.mem.set_u16(result.addr() + 4, other_triangle);
            let other_edge = e.mem.u32(locals.addr() + 8);
            e.mem.set_u32(result.addr() + 8, other_edge);
        }
        found
    })
}

// Translated from 0068f3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMesh::GetMatchingEdge` overload 2 (Xbox PDB): as overload 3, but the
/// matched mesh is delivered into the `NiPointer<NavMesh>` at `mesh_out`
/// (through a temporary smart pointer), the triangle into `triangle_out`
/// (`u16`) and the edge into `edge_out`.
pub fn nav_mesh_get_matching_edge_ov2(
    e: &mut Engine,
    this: Ptr<NavMesh>,
    triangle: u16,
    edge: i32,
    mesh_out: Ptr,
    triangle_out: Ptr,
    edge_out: Ptr,
) -> bool {
    e.with_stack(4, |e, found_mesh| {
        let found = nav_mesh_get_matching_edge_ov3(
            e,
            this,
            triangle,
            edge,
            found_mesh,
            triangle_out,
            edge_out,
        );
        if found {
            let mesh = e.mem.u32(found_mesh.addr());
            e.with_stack(8, |e, temp| {
                let temp = e
                    .call(POINTER_CONSTRUCT_FROM_CDECL, &args![temp, mesh])
                    .u32();
                e.call(POINTER_ASSIGN, &args![mesh_out, temp]);
                e.call(POINTER_DESTRUCT, &args![temp]);
            });
        }
        found
    })
}

// Translated from 0068f460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMesh::GetMatchingEdge` overload 3 (Xbox PDB): finds the edge that
/// mirrors edge `edge` of triangle `triangle`. Without extra info on the
/// edge the neighbour is in this mesh and `*mesh_out` = `this`; with extra
/// info the portal's other mesh is used, and the edge must point back with a
/// portal naming this mesh's info and `triangle`. Stores the mesh, the
/// neighbour triangle (`u16`) and the neighbour's edge, and returns whether
/// a mirror edge exists.
pub fn nav_mesh_get_matching_edge_ov3(
    e: &mut Engine,
    this: Ptr<NavMesh>,
    triangle: u16,
    edge: i32,
    mesh_out: Ptr,
    triangle_out: Ptr,
    edge_out: Ptr,
) -> bool {
    let count = e.call(NAV_MESH_TRIANGLE_COUNT, &args![this]).u32();
    if triangle as u32 >= count {
        return false;
    }
    let tri = mesh_triangle(e, this, triangle);
    if !fn_0068f200(e, tri, edge) {
        return false;
    }
    if !fn_0068f1d0(e, tri, edge as u32) {
        e.mem.set_u32(mesh_out.addr(), this.addr());
        let neighbour = fn_0068f2c0(e, tri, edge);
        e.mem.set_u16(triangle_out.addr(), neighbour);
        let neighbour_index = e.mem.u16(triangle_out.addr());
        let neighbour_tri = mesh_triangle(e, this, neighbour_index);
        let mut other_edge = 0i32;
        while other_edge < 3 {
            if !fn_0068f1d0(e, neighbour_tri, other_edge as u32)
                && fn_0068f2c0(e, neighbour_tri, other_edge) == triangle
            {
                break;
            }
            other_edge += 1;
        }
        if other_edge >= 3 {
            return false;
        }
        e.mem.set_i32(edge_out.addr(), other_edge);
        return true;
    }
    let info = nav_mesh_get_edge_extra_info(e, this, triangle, edge);
    let portal_info = e.get(info, EdgeExtraInfo::pOtherMesh);
    if portal_info.is_null() {
        return false;
    }
    e.with_stack(4, |e, other_mesh| {
        if !fn_0068f2e0(e, portal_info, other_mesh) {
            return false;
        }
        let other = Ptr::<NavMesh>::new(e.mem.u32(other_mesh.addr()));
        let other_triangle = e.get(info, EdgeExtraInfo::sTriangle);
        let other_count = e.call(NAV_MESH_TRIANGLE_COUNT, &args![other]).u32();
        if other_triangle as u32 >= other_count {
            return false;
        }
        let other_tri = mesh_triangle(e, other, other_triangle);
        let mut other_edge = 0i32;
        while other_edge < 3 {
            if fn_0068f1d0(e, other_tri, other_edge as u32) {
                let back = nav_mesh_get_edge_extra_info(e, other, other_triangle, other_edge);
                if !back.is_null() {
                    let own_info = fn_0068efb0(e, this);
                    if e.get(back, EdgeExtraInfo::pOtherMesh) == own_info
                        && e.get(back, EdgeExtraInfo::sTriangle) == triangle
                    {
                        break;
                    }
                }
            }
            other_edge += 1;
        }
        if other_edge == 3 {
            return false;
        }
        e.mem.set_u32(mesh_out.addr(), other.addr());
        e.mem.set_u16(triangle_out.addr(), other_triangle);
        e.mem.set_i32(edge_out.addr(), other_edge);
        true
    })
}

// Translated from 0068f670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMesh::GetMatchingEdge` overload 4 (Xbox PDB): overload 3 on the edge
/// named by `edge_ref` (triangle `u16` at +4, edge at +8), writing the mesh,
/// triangle and edge to the first three words of `result`. Returns overload
/// 3's result.
pub fn nav_mesh_get_matching_edge_ov4(
    e: &mut Engine,
    this: Ptr<NavMesh>,
    edge_ref: Ptr,
    result: Ptr,
) -> bool {
    let triangle = e.mem.u16(edge_ref.addr() + 4);
    let edge = e.mem.i32(edge_ref.addr() + 8);
    nav_mesh_get_matching_edge_ov3(
        e,
        this,
        triangle,
        edge,
        result,
        result.byte_add(4),
        result.byte_add(8),
    )
}

// Translated from 0068f6b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DoorPortals[index]` through the array's element getter (`00844160`);
/// returns its result.
pub fn fn_0068f6b0(e: &mut Engine, this: Ptr<NavMesh>, index: u16) -> u32 {
    let portals = this.at(NavMesh::DoorPortals);
    e.call(DOOR_PORTAL_ELEMENT, &args![portals, index]).u32()
}

/// One report of `CheckNavMesh`: the format, then the navmesh's form id, the
/// cell text's characters and the numbers `extra`, to the debug print.
fn report(e: &mut Engine, this: Ptr<NavMesh>, text: Ptr, format: u32, extra: &[u32]) {
    let characters = e.call(POINTER_GET, &args![text]).u32();
    let form_id = e.call(GET_FORM_ID, &args![this]).u32();
    let mut words = vec![format, form_id, characters];
    words.extend_from_slice(extra);
    e.call(DEBUG_PRINT, &words);
}

/// Whether one coordinate (`offset` 0, 4 or 8) of vertex `index` is finite
/// and not larger than the limit in magnitude.
fn vertex_coordinate_ok(
    e: &mut Engine,
    vertices: Ptr<BSSimpleArray>,
    index: u32,
    offset: u32,
) -> bool {
    let vertex = e.call(ELEMENT_12, &args![vertices, index]).u32();
    let value = e.mem.f32(vertex + offset);
    if e.call(FINITE, &args![value as f64]).i32() == 0 {
        return false;
    }
    let vertex = e.call(ELEMENT_12, &args![vertices, index]).u32();
    let value = e.mem.f32(vertex + offset);
    let magnitude = e.call(FLOAT_ABS, &args![value]).f64();
    let limit: f64 = e.global(MAX_COORDINATE);
    !matches!(
        magnitude.partial_cmp(&limit),
        Some(std::cmp::Ordering::Greater)
    )
}

// Translated from 0068f6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMesh::CheckNavMesh` (Xbox PDB): validates the mesh, repairs what it
/// can and reports each problem through the debug print (format, form id,
/// cell text, numbers). Checks, in order: vertex coordinates (finite, below
/// 1e14; stops at the first bad one), a large triangle count (2000 or
/// more), triangle neighbour indexes (bad ones are cleared), downfacing
/// triangles (flipped), extra-info indexes and portals (bad ones are
/// deleted; a portal must reach a triangle that exists and a mesh in the same
/// worldspace), degenerate triangles, two edges pointing to the same
/// neighbour, opposite normals and missing portal data between neighbouring
/// triangles, links the neighbour does not return and shared vertices
/// that do not match, and the cover list. It ends with the cross-cell portal
/// check `006908f0` (its result is not used). Returns true if no problem was
/// found. The word argument is not read.
pub fn nav_mesh_check_nav_mesh(e: &mut Engine, this: Ptr<NavMesh>, _unused_1: u32) -> bool {
    let mut problems = false;
    let text = Ptr::<()>::new(e.mem.alloc(8));
    e.call(STRING_CONSTRUCT, &args![text]);
    let cell = e.call(GET_PARENT_CELL, &args![this]).u32();
    e.vcall(cell, GET_FORM_DETAILED_STRING_SLOT, &args![text]);

    let vertices = this.at(NavMesh::Vertices);
    let triangles = this.at(NavMesh::Triangles);

    // Vertex coordinates; the first bad one ends this check.
    let mut index = 0u32;
    'vertices: while index < array_size(e, vertices) {
        for (offset, format) in [(0u32, FORMAT_BAD_X), (4, FORMAT_BAD_Y), (8, FORMAT_BAD_Z)] {
            if !vertex_coordinate_ok(e, vertices, index, offset) {
                report(e, this, text, format, &[]);
                problems = true;
                break 'vertices;
            }
        }
        index += 1;
    }

    if array_size(e, triangles) >= 2000 {
        let count = array_size(e, triangles);
        report(e, this, text, FORMAT_MANY_TRIANGLES, &[count]);
    }

    // Neighbour indexes out of range, and downfacing triangles.
    let mut t = 0u32;
    while t < array_size(e, triangles) {
        let tri = triangle_element(e, this, t);
        for edge in 0..3i32 {
            if !fn_0068f1d0(e, tri, edge as u32) && fn_0068f200(e, tri, edge) {
                let neighbour = fn_0068f2c0(e, tri, edge);
                if neighbour as u32 >= array_size(e, triangles) {
                    report(e, this, text, FORMAT_BAD_TRIANGLE_INDEX, &[t, edge as u32]);
                    fn_006907d0(e, tri, edge as u32, 0xffff);
                    problems = true;
                }
            }
        }
        e.with_stack(0x10, |e, normal| {
            e.call(TRIANGLE_NORMAL, &args![this, normal, t as u16]);
            let z = e.mem.f32(normal.addr() + 8);
            let zero: f64 = e.global(ZERO);
            if matches!(
                (z as f64).partial_cmp(&zero),
                Some(std::cmp::Ordering::Less)
            ) {
                report(e, this, text, FORMAT_DOWNFACING, &[t]);
                e.call(FLIP_TRIANGLE, &args![this, t as u16]);
                problems = true;
            }
        });
        t += 1;
    }

    // Extra edge info: indexes and portals.
    let mut t = 0u32;
    while t < array_size(e, triangles) {
        let tri = mesh_triangle(e, this, t as u16);
        for edge in 0..3i32 {
            if !fn_0068f1d0(e, tri, edge as u32) {
                continue;
            }
            let slot = fn_0068f2c0(e, tri, edge);
            let infos = this.at(NavMesh::ExtraEdgeInfo);
            if slot as u32 >= array_size(e, infos) {
                report(
                    e,
                    this,
                    text,
                    FORMAT_BAD_EXTRA_INFO_INDEX,
                    &[t, edge as u32],
                );
                fn_00690790(e, tri, edge as u32);
                problems = true;
                continue;
            }
            let info = nav_mesh_get_edge_extra_info(e, this, t as u16, edge);
            if e.get(info, EdgeExtraInfo::pOtherMesh).is_null() {
                let slot = fn_0068f2c0(e, tri, edge);
                nav_mesh_delete_extra_edge_info(e, this, slot);
                problems = true;
                continue;
            }
            if e.get(info, EdgeExtraInfo::sTriangle) == NO_NEIGHBOUR {
                let slot = fn_0068f2c0(e, tri, edge);
                nav_mesh_delete_extra_edge_info(e, this, slot);
                problems = true;
                continue;
            }
            let portal_info = e.get(info, EdgeExtraInfo::pOtherMesh);
            let other_triangle = e.get(info, EdgeExtraInfo::sTriangle);
            let holder = Ptr::<()>::new(e.mem.alloc(8));
            e.call(POINTER_CONSTRUCT, &args![holder]);
            let mut cleared = false;
            if fn_00690860(e, portal_info, holder) {
                let other = e.call(POINTER_GET, &args![holder]).u32();
                let other_count = e.call(NAV_MESH_TRIANGLE_COUNT, &args![other]).u32();
                if other_triangle as u32 >= other_count {
                    let other = e.call(POINTER_GET, &args![holder]).u32();
                    let other_id = e.call(GET_FORM_ID, &args![other]).u32();
                    report(
                        e,
                        this,
                        text,
                        FORMAT_BAD_PORTAL_TRIANGLE,
                        &[t, edge as u32, other_id, other_triangle as u32],
                    );
                    let slot = fn_0068f2c0(e, tri, edge);
                    nav_mesh_delete_extra_edge_info(e, this, slot);
                    problems = true;
                    cleared = true;
                }
            }
            if !cleared {
                let cell = e.call(GET_PARENT_CELL, &args![this]).u32();
                let own_space = e.call(CELL_GET_WORLD_SPACE, &args![cell]).u32();
                let portal_space = fn_00690800(e, portal_info).addr();
                if own_space != portal_space {
                    let portal_id = e.call(POINTER_GET, &args![portal_info]).u32();
                    report(
                        e,
                        this,
                        text,
                        FORMAT_PORTAL_OTHER_WORLD_SPACE,
                        &[t, edge as u32, portal_id],
                    );
                    let slot = fn_0068f2c0(e, tri, edge);
                    nav_mesh_delete_extra_edge_info(e, this, slot);
                    problems = true;
                }
            }
            e.call(POINTER_DESTRUCT, &args![holder]);
            e.mem.free(holder.addr());
        }
        t += 1;
    }

    // Degenerate triangles: two corners share a vertex.
    let mut t = 0u32;
    while t < array_size(e, triangles) {
        let tri = mesh_triangle(e, this, t as u16);
        for (a, b, format) in [
            (0, 1, FORMAT_DEGENERATE_01),
            (0, 2, FORMAT_DEGENERATE_02),
            (1, 2, FORMAT_DEGENERATE_12),
        ] {
            if triangle_vertex(e, tri, a) == triangle_vertex(e, tri, b) {
                let shared = triangle_vertex(e, tri, a);
                report(e, this, text, format, &[t, shared as u32]);
                problems = true;
            }
        }
        t += 1;
    }

    // Two edges leading to the same neighbour.
    let mut t = 0u32;
    while t < array_size(e, triangles) {
        let tri = mesh_triangle(e, this, t as u16);
        for (a, b, format) in [
            (0, 1, FORMAT_SAME_NEIGHBOUR_01),
            (0, 2, FORMAT_SAME_NEIGHBOUR_02),
            (1, 2, FORMAT_SAME_NEIGHBOUR_12),
        ] {
            if fn_0068f2c0(e, tri, a) == fn_0068f2c0(e, tri, b)
                && fn_0068f2c0(e, tri, a) != NO_NEIGHBOUR
                && fn_0068f1d0(e, tri, a as u32) == fn_0068f1d0(e, tri, b as u32)
            {
                let shared = fn_0068f2c0(e, tri, a);
                report(e, this, text, format, &[t, shared as u32]);
                problems = true;
            }
        }
        t += 1;
    }

    // Triangles sharing an edge: orientation and portal data.
    let mut t = 0u32;
    while t < array_size(e, triangles) {
        let tri = mesh_triangle(e, this, t as u16);
        for edge in 0..3i32 {
            let next_edge = (edge + 1) % 3;
            let edge_start = triangle_vertex(e, tri, edge);
            let edge_end = triangle_vertex(e, tri, next_edge);
            let mut other = 0u32;
            while other < array_size(e, triangles) {
                if other != t {
                    let other_tri = mesh_triangle(e, this, other as u16);
                    for corner in 0..3i32 {
                        if triangle_vertex(e, other_tri, corner) != edge_end {
                            continue;
                        }
                        let corner_next = (corner + 1) % 3;
                        for probe in 0..3i32 {
                            if probe == corner || triangle_vertex(e, other_tri, probe) != edge_start
                            {
                                continue;
                            }
                            if probe != corner_next {
                                report(e, this, text, FORMAT_OPPOSITE_NORMALS, &[t, other]);
                                problems = true;
                            }
                            if !fn_0068f1d0(e, tri, edge as u32) {
                                if fn_0068f2c0(e, tri, edge) != NO_NEIGHBOUR {
                                    if fn_0068f1d0(e, other_tri, corner as u32) {
                                        report(
                                            e,
                                            this,
                                            text,
                                            FORMAT_PORTAL_WITHOUT_MATCH,
                                            &[t, edge as u32, other, other, corner as u32],
                                        );
                                        problems = true;
                                    }
                                    if fn_0068f2c0(e, tri, edge) as u32 != other {
                                        report(
                                            e,
                                            this,
                                            text,
                                            FORMAT_MISSING_LINK,
                                            &[t, edge as u32, other],
                                        );
                                        problems = true;
                                    }
                                    if fn_0068f2c0(e, other_tri, corner) as u32 != t {
                                        report(
                                            e,
                                            this,
                                            text,
                                            FORMAT_MISSING_LINK,
                                            &[other, corner as u32, t],
                                        );
                                        problems = true;
                                    }
                                }
                            } else if !fn_0068f1d0(e, other_tri, corner as u32) {
                                report(
                                    e,
                                    this,
                                    text,
                                    FORMAT_MISSING_PORTAL,
                                    &[other, corner as u32],
                                );
                                problems = true;
                            }
                        }
                    }
                }
                other += 1;
            }
        }
        t += 1;
    }

    // Every link must be returned by the neighbour, with matching vertices.
    let mut t = 0u32;
    while t < array_size(e, triangles) {
        let tri = mesh_triangle(e, this, t as u16);
        for edge in 0..3i32 {
            if fn_0068f1d0(e, tri, edge as u32) || fn_0068f2c0(e, tri, edge) == NO_NEIGHBOUR {
                continue;
            }
            let neighbour = fn_0068f2c0(e, tri, edge);
            let neighbour_tri = mesh_triangle(e, this, neighbour);
            let mut found = false;
            let mut back = 0i32;
            while back < 3 {
                if !fn_0068f1d0(e, neighbour_tri, back as u32)
                    && fn_0068f2c0(e, neighbour_tri, back) != NO_NEIGHBOUR
                    && fn_0068f2c0(e, neighbour_tri, back) as u32 == t
                {
                    found = true;
                    break;
                }
                back += 1;
            }
            if !found {
                let first = fn_0068f2c0(e, tri, edge);
                let second = fn_0068f2c0(e, tri, edge);
                report(
                    e,
                    this,
                    text,
                    FORMAT_LINK_NOT_RETURNED,
                    &[t, second as u32, first as u32],
                );
                problems = true;
            }
            if back < 3 {
                let next_edge = (edge + 1) % 3;
                let back_next = (back + 1) % 3;
                let matching = triangle_vertex(e, tri, edge)
                    == triangle_vertex(e, neighbour_tri, back_next)
                    && triangle_vertex(e, tri, next_edge)
                        == triangle_vertex(e, neighbour_tri, back);
                if !matching {
                    let linked = fn_0068f2c0(e, tri, edge);
                    report(e, this, text, FORMAT_VERTICES_MISMATCH, &[t, linked as u32]);
                    problems = true;
                }
            }
        }
        t += 1;
    }

    // The cover list names existing triangles that have cover.
    let cover = this.at(NavMesh::CoverArray);
    let mut cover_index = 0u32;
    while cover_index < array_size(e, cover) {
        let slot = e.call(U16_ELEMENT, &args![cover, cover_index]).u32();
        let cover_triangle = e.mem.u16(slot);
        let tri = mesh_triangle(e, this, cover_triangle);
        if tri.is_null() {
            report(
                e,
                this,
                text,
                FORMAT_COVER_MISSING_TRIANGLE,
                &[cover_index, cover_triangle as u32],
            );
            problems = true;
        } else if !fn_00690770(e, tri) {
            report(
                e,
                this,
                text,
                FORMAT_COVER_WITHOUT_COVER,
                &[cover_index, cover_triangle as u32],
            );
            problems = true;
        }
        cover_index += 1;
    }

    fn_006908f0(e, this);
    let ok = !problems;
    e.call(STRING_DESTRUCT, &args![text]);
    e.mem.free(text.addr());
    ok
}

// Translated from 00690770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshTriangle` method: whether any cover flag (mask `0x0FBE0000`) of
/// `TriangleFlags` is set.
pub fn fn_00690770(e: &mut Engine, this: Ptr<NavMeshTriangle>) -> bool {
    e.get(this, NavMeshTriangle::TriangleFlags) & COVER_FLAGS != 0
}

// Translated from 00690790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshTriangle` method: clears edge info flag `edge` and sets the
/// neighbour slot `Triangles[edge]` to `0xFFFF` (no neighbour).
pub fn fn_00690790(e: &mut Engine, this: Ptr<NavMeshTriangle>, edge: u32) {
    let flags = e.get(this, NavMeshTriangle::TriangleFlags);
    e.set(
        this,
        NavMeshTriangle::TriangleFlags,
        !(1u32 << (edge & 31)) & flags,
    );
    e.mem
        .set_u16(neighbour_slot(this, edge as i32), NO_NEIGHBOUR);
}

// Translated from 006907d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshTriangle` method: clears edge info flag `edge` (as `00690790`),
/// then stores `value` in the neighbour slot `Triangles[edge]`.
pub fn fn_006907d0(e: &mut Engine, this: Ptr<NavMeshTriangle>, edge: u32, value: u16) {
    fn_00690790(e, this, edge);
    e.mem.set_u16(neighbour_slot(this, edge as i32), value);
}

// Translated from 00690800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshInfo` method: the info's parent space (`00690830`) cast with
/// `__RTDynamicCast` from `TESForm` to `TESWorldSpace`; null if it is not a
/// worldspace.
pub fn fn_00690800(e: &mut Engine, this: Ptr) -> Ptr {
    let space = fn_00690830(e, this);
    e.call(
        RT_DYNAMIC_CAST,
        &args![space, 0i32, TYPE_TES_FORM, TYPE_TES_WORLD_SPACE, 0i32],
    )
    .ptr()
}

// Translated from 00690830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshInfo` method: `pParentSpace` if set, else the form found by
/// `ParentSpaceID` (`004839c0`).
pub fn fn_00690830(e: &mut Engine, this: Ptr) -> u32 {
    let space = e.mem.u32(this.addr() + INFO_PARENT_SPACE);
    if space != 0 {
        space
    } else {
        let id = e.mem.u32(this.addr() + INFO_PARENT_SPACE_ID);
        e.call(LOOKUP_FORM, &args![id]).u32()
    }
}

// Translated from 00690860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshInfo` method: assigns the info's `pNavMesh` to the
/// `NiPointer<NavMesh>` at `result` (`00690890`); returns whether `pNavMesh`
/// is non-null.
pub fn fn_00690860(e: &mut Engine, this: Ptr, result: Ptr) -> bool {
    let mesh = e.mem.u32(this.addr() + INFO_NAV_MESH);
    fn_00690890(e, result, Ptr::new(mesh));
    e.mem.u32(this.addr() + INFO_NAV_MESH) != 0
}

// Translated from 00690890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assigns the raw navmesh pointer `mesh` to the `NiPointer` at `result`,
/// through a temporary smart pointer (cdecl; the temporary's unwinding frame
/// is not translated).
pub fn fn_00690890(e: &mut Engine, result: Ptr, mesh: Ptr) {
    e.with_stack(8, |e, temp| {
        e.call(POINTER_CONSTRUCT_FROM, &args![temp, mesh]);
        e.call(POINTER_ASSIGN, &args![result, temp]);
        e.call(POINTER_DESTRUCT, &args![temp]);
    });
}

// Translated from 006908f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Checks the mesh against the four neighbouring exterior cells: for every
/// navmesh the cell to the west, east, south and north holds, asks
/// `005d4a40` (a debug stub that always accepts) with the mesh, the
/// neighbour, a direction flag (1 for west/east, 0 for south/north) and the
/// border coordinate (the west or south border as `float`, the east or north
/// border 4096 further); false if the check rejects any. Interior cells
/// always pass. Cells without a navmesh array are skipped.
pub fn fn_006908f0(e: &mut Engine, this: Ptr<NavMesh>) -> bool {
    let cell = e.call(GET_PARENT_CELL, &args![this]).u32();
    if e.call(CELL_IS_INTERIOR, &args![cell]).bool() {
        return true;
    }
    let mut ok = true;
    let cell = e.call(GET_PARENT_CELL, &args![this]).u32();
    let x = e.call(CELL_GET_X, &args![cell]).i32();
    let cell = e.call(GET_PARENT_CELL, &args![this]).u32();
    let y = e.call(CELL_GET_Y, &args![cell]).i32();
    let cell_width: f64 = e.global(CELL_WIDTH);

    // (x offset, y offset, direction flag, far border).
    let neighbours: [(i32, i32, u32, bool); 4] = [
        (-1, 0, 1, false),
        (1, 0, 1, true),
        (0, -1, 0, false),
        (0, 1, 0, true),
    ];
    for (dx, dy, horizontal, far) in neighbours {
        let origin = if horizontal == 1 { x } else { y };
        let mut border = (origin << 12) as f32;
        if far {
            border = (((origin << 12) as f64) + cell_width) as f32;
        }
        let (x_coordinate, y_coordinate) = if horizontal == 1 {
            (border, 0.0f32)
        } else {
            (0.0f32, border)
        };
        let own_cell = e.call(GET_PARENT_CELL, &args![this]).u32();
        let world_space = e.call(CELL_GET_WORLD_SPACE, &args![own_cell]).u32();
        let neighbour_cell = e
            .call(WORLD_SPACE_GET_CELL, &args![world_space, x + dx, y + dy])
            .u32();
        if neighbour_cell == 0 {
            continue;
        }
        if e.call(CELL_GET_NAV_MESHES, &args![neighbour_cell]).u32() == 0 {
            continue;
        }
        let mut n = 0u32;
        loop {
            let array = e.call(CELL_GET_NAV_MESHES, &args![neighbour_cell]).u32();
            let count = e.call(NAV_MESH_ARRAY_SIZE, &args![array]).u32();
            if n >= count {
                break;
            }
            let array = e.call(CELL_GET_NAV_MESHES, &args![neighbour_cell]).u32();
            let rejected = e.with_stack(8, |e, holder| {
                let held = e.call(NAV_MESH_ARRAY_GET, &args![array, holder, n]).u32();
                let neighbour_mesh = e.call(POINTER_DEREFERENCE, &args![held]).u32();
                let accepted = e
                    .call(
                        NEIGHBOUR_CHECK,
                        &args![this, neighbour_mesh, horizontal, x_coordinate, y_coordinate],
                    )
                    .bool();
                e.call(POINTER_DESTRUCT, &args![held]);
                !accepted
            });
            if rejected {
                ok = false;
            }
            n += 1;
        }
    }
    ok
}

// Translated from 00690d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMesh::ClearNavMesh` (Xbox PDB): empties the vertex, triangle,
/// extra-info, cover, door-portal and obstacle arrays and the grid, and, if
/// the mesh has a `NavMeshInfo`, deletes that info from the `TES` navmesh
/// info map by the mesh's form id. The word argument is not read.
pub fn nav_mesh_clear_nav_mesh(e: &mut Engine, this: Ptr<NavMesh>, _unused_1: u32) {
    for array in [
        this.at(NavMesh::Vertices),
        this.at(NavMesh::Triangles),
        this.at(NavMesh::ExtraEdgeInfo),
    ] {
        e.call(ARRAY_CLEAR, &args![array, 1u32]);
    }
    e.call(GRID_CLEAR, &args![this.byte_add(MESH_GRID_OFFSET)]);
    e.call(ARRAY_CLEAR, &args![this.at(NavMesh::CoverArray), 1u32]);
    e.call(OBSTACLES_CLEAR, &args![this.at(NavMesh::Obstacles), 1u32]);
    e.call(ARRAY_CLEAR, &args![this.at(NavMesh::DoorPortals), 1u32]);
    if !e.get(this, NavMesh::pNavMeshInfo).is_null() {
        let form_id = e.call(GET_FORM_ID, &args![this]).u32();
        let tes: u32 = e.global(TES_INSTANCE);
        let map = e.call(GET_NAV_MESH_INFO_MAP, &args![tes]).u32();
        e.call(DELETE_NAV_MESH_INFO, &args![map, form_id]);
    }
}

// Translated from 00690df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshTriangle` method: stores vertex index `value` at
/// `Vertices[index]`.
pub fn fn_00690df0(e: &mut Engine, this: Ptr<NavMeshTriangle>, index: u32, value: u16) {
    e.mem
        .set_u16(this.addr().wrapping_add(index.wrapping_mul(2)), value);
}

// Translated from 00690e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshTriangle` method: whether any edge has extra info (flags 0 to 2).
pub fn fn_00690e10(e: &mut Engine, this: Ptr<NavMeshTriangle>) -> bool {
    e.get(this, NavMeshTriangle::TriangleFlags) & EDGE_INFO_FLAGS != 0
}

// Translated from 00690e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshTriangle` method: sets edge info flag `edge` and stores `value`
/// (an `ExtraEdgeInfo` index) in the neighbour slot `Triangles[edge]`.
pub fn fn_00690e30(e: &mut Engine, this: Ptr<NavMeshTriangle>, edge: u32, value: u16) {
    let flags = e.get(this, NavMeshTriangle::TriangleFlags);
    e.set(
        this,
        NavMeshTriangle::TriangleFlags,
        (1u32 << (edge & 31)) | flags,
    );
    e.mem.set_u16(neighbour_slot(this, edge as i32), value);
}

// Translated from 00690e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshTriangle` method: whether flag bit `0x400` is set.
pub fn fn_00690e70(e: &mut Engine, this: Ptr<NavMeshTriangle>) -> bool {
    e.get(this, NavMeshTriangle::TriangleFlags) & TRIANGLE_FLAG_0X400 != 0
}

// Translated from 00690e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMesh::DeleteExtraEdgeInfo` (Xbox PDB): removes `ExtraEdgeInfo[index]`
/// (the array moves its last element into the gap) and repairs the
/// triangles: an edge that used `index` loses its extra info flag, and an
/// edge that used the moved last element now points at `index`.
pub fn nav_mesh_delete_extra_edge_info(e: &mut Engine, this: Ptr<NavMesh>, index: u16) {
    let infos = this.at(NavMesh::ExtraEdgeInfo);
    e.call(ARRAY_REMOVE_12, &args![infos, index, 1u32]);
    let triangles = this.at(NavMesh::Triangles);
    let count = array_size(e, triangles);
    let mut t = 0u32;
    while t < count {
        let tri = triangle_element(e, this, t);
        if fn_00690e10(e, tri) {
            let mut edge = 0u32;
            while edge < 3 {
                let tri = triangle_element(e, this, t);
                if fn_0068f1d0(e, tri, edge) {
                    let tri = triangle_element(e, this, t);
                    if fn_0068f2c0(e, tri, edge as i32) == index {
                        let tri = triangle_element(e, this, t);
                        fn_00690790(e, tri, edge);
                    } else {
                        let tri = triangle_element(e, this, t);
                        let slot = fn_0068f2c0(e, tri, edge as i32);
                        let size = array_size(e, infos);
                        if slot as u32 == size {
                            let tri = triangle_element(e, this, t);
                            fn_00690e30(e, tri, edge, index);
                        }
                    }
                }
                edge += 1;
            }
        }
        t += 1;
    }
}

// Translated from 00690fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends a copy of the 12-byte vertex at `vertex` to the mesh's vertex
/// array; returns the array's result (the new vertex's index).
pub fn fn_00690fe0(e: &mut Engine, this: Ptr<NavMesh>, vertex: Ptr) -> u32 {
    let vertices = this.at(NavMesh::Vertices);
    e.call(ARRAY_ADD_12, &args![vertices, vertex]).u32()
}

// Translated from 00691000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends a copy of the 16-byte triangle at `triangle` to the mesh's
/// triangle array; returns the array's result (the new triangle's index).
pub fn fn_00691000(e: &mut Engine, this: Ptr<NavMesh>, triangle: Ptr) -> u32 {
    let triangles = this.at(NavMesh::Triangles);
    e.call(ARRAY_ADD_16, &args![triangles, triangle]).u32()
}

// Translated from 00691020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends the `EdgeExtraInfo` at `info` to the mesh's `ExtraEdgeInfo`
/// array (`00691c10`); returns what that returns (the new index).
pub fn fn_00691020(e: &mut Engine, this: Ptr<NavMesh>, info: Ptr) -> u32 {
    let infos = this.at(NavMesh::ExtraEdgeInfo);
    e.call(EXTRA_INFO_ADD, &args![infos, info]).u32()
}

// Translated from 00691040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unpacks the six bits that the triangle flags hold for edge `edge` (from
/// bit `16 + 6 * edge` upward): the low four bits go to the `u16` at `value`,
/// bit 4 to the byte at `flag_a` and bit 5 to the byte at `flag_b` (both 0 or
/// 1). The shift count wraps at 32 like the processor's.
pub fn fn_00691040(
    e: &mut Engine,
    this: Ptr<NavMeshTriangle>,
    edge: u16,
    value: Ptr,
    flag_a: Ptr,
    flag_b: Ptr,
) {
    let flags = e.get(this, NavMeshTriangle::TriangleFlags);
    let shift = (edge as u32).wrapping_mul(6).wrapping_add(0x10);
    let bits = flags >> (shift & 31);
    e.mem.set_u16(value.addr(), (bits & 0xf) as u16);
    e.mem.set_u8(flag_a.addr(), (bits & 0x10 != 0) as u8);
    e.mem.set_u8(flag_b.addr(), (bits & 0x20 != 0) as u8);
}

// Translated from 006910a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Rebuilds the cover array: clears it (keeping the buffer), then appends the
/// index of every triangle that does not have flag `0x20` set and for which
/// `00690770` (the cover flags test) holds.
pub fn fn_006910a0(e: &mut Engine, this: Ptr<NavMesh>) {
    e.call(ARRAY_CLEAR, &args![this.at(NavMesh::CoverArray), 0u32]);
    let count = array_size(e, this.at(NavMesh::Triangles));
    for index in 0..count {
        let tri = mesh_triangle(e, this, index as u16);
        if fn_00691140(e, tri, TRIANGLE_FLAG_0X20) {
            continue;
        }
        let tri = mesh_triangle(e, this, index as u16);
        if fn_00690770(e, tri) {
            e.with_stack(4, |e, slot| {
                e.mem.set_u16(slot.addr(), index as u16);
                fn_00691820(e, this.at(NavMesh::CoverArray), slot);
            });
        }
    }
}

// Translated from 00691140 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when any bit of `mask` is set in the triangle's flags.
pub fn fn_00691140(e: &mut Engine, this: Ptr<NavMeshTriangle>, mask: u32) -> bool {
    e.get(this, NavMeshTriangle::TriangleFlags) & mask != 0
}

// Translated from 00691160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshTriangle::NavMeshTriangle`, by its body: sets the three vertex
/// indexes and the three neighbour slots to `0xFFFF` and the flags to 0.
/// Returns `this`.
pub fn fn_00691160(e: &mut Engine, this: Ptr<NavMeshTriangle>) -> Ptr<NavMeshTriangle> {
    for slot in 0..6 {
        e.mem.set_u16(this.addr() + 2 * slot, NO_NEIGHBOUR);
    }
    e.set(this, NavMeshTriangle::TriangleFlags, 0);
    this
}

// Translated from 006911c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Number of cells of the mesh's search grid: its side length squared (the
/// side is read through `00559450`, the first word of the grid).
pub fn fn_006911c0(e: &mut Engine, this: Ptr<NavMesh>) -> u32 {
    let grid = this.byte_add(MESH_GRID_OFFSET);
    let rows = e.call(POINTER_GET, &args![grid]).u32();
    let columns = e.call(POINTER_GET, &args![grid]).u32();
    columns.wrapping_mul(rows)
}

// Translated from 006911f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards `index` to `00691210` on the mesh's search grid; returns its
/// result, the address of the cell.
pub fn fn_006911f0(e: &mut Engine, this: Ptr<NavMesh>, index: u32) -> Ptr {
    fn_00691210(e, this.byte_add(MESH_GRID_OFFSET).cast(), index)
}

// Translated from 00691210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Address of cell `index` (a 16-byte array) of the grid, null when `index`
/// is not below `iGridSize * iGridSize`.
pub fn fn_00691210(e: &mut Engine, this: Ptr<NavMeshGrid>, index: u32) -> Ptr {
    let size = e.get(this, NavMeshGrid::iGridSize);
    if index >= size.wrapping_mul(size) {
        return Ptr::NULL;
    }
    let data = e.get(this, NavMeshGrid::GridData);
    Ptr::new(index.wrapping_shl(4).wrapping_add(data.addr()))
}

// Translated from 00691240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards `point` to `00691260` on the mesh's search grid; returns its
/// result, the cell number.
pub fn fn_00691240(e: &mut Engine, this: Ptr<NavMesh>, point: Ptr) -> u32 {
    fn_00691260(e, this.byte_add(MESH_GRID_OFFSET).cast(), point)
}

// Translated from 00691260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cell number (`row * iGridSize + column`) of the grid cell holding the
/// point at `point` (two floats are read).
pub fn fn_00691260(e: &mut Engine, this: Ptr<NavMeshGrid>, point: Ptr) -> u32 {
    let size = e.get(this, NavMeshGrid::iGridSize);
    e.with_stack(8, |e, cell| {
        fn_006912a0(e, this, point, cell, cell.byte_add(4));
        let row = e.mem.u32(cell.addr());
        let column = e.mem.u32(cell.addr() + 4);
        row.wrapping_mul(size).wrapping_add(column)
    })
}

/// `floor`, truncation and clamping to `0..size - 1` of one grid coordinate
/// (the sequence `006912a0` runs for its row and its column).
fn grid_coordinate(e: &mut Engine, position: f32, size: u32) -> u32 {
    let rounded = e.call(FLOAT_FLOOR, &args![position]).f64();
    let cell = e.call(FLOAT_TO_INT, &args![rounded]).i32();
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), cell as u32);
        e.call(CLAMP_INT, &args![slot, 0i32, size.wrapping_sub(1)]);
        e.mem.u32(slot.addr())
    })
}

// Translated from 006912a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Grid cell of the point at `point` (x, y floats): the offset from the
/// grid's minimum corner divided by the section lengths, floored, truncated
/// and clamped to `0..iGridSize - 1`. The column (from x) is stored at
/// `out_column`, the row (from y) at `out_row`.
pub fn fn_006912a0(
    e: &mut Engine,
    this: Ptr<NavMeshGrid>,
    point: Ptr,
    out_row: Ptr,
    out_column: Ptr,
) {
    let x = e.mem.f32(point.addr()) as f64;
    let y = e.mem.f32(point.addr() + 4) as f64;
    let min_x = e.get(this, NavMeshGrid::GridBoundsMin_x) as f64;
    let min_y = e.get(this, NavMeshGrid::GridBoundsMin_y) as f64;
    let offset_x = (x - min_x) as f32;
    let offset_y = (y - min_y) as f32;
    let column_length = e.get(this, NavMeshGrid::fColumnSectionLen) as f64;
    let row_length = e.get(this, NavMeshGrid::fRowSectionLen) as f64;
    let column_position = (offset_x as f64 / column_length) as f32;
    let row_position = (offset_y as f64 / row_length) as f32;
    let size = e.get(this, NavMeshGrid::iGridSize);
    let column = grid_coordinate(e, column_position, size);
    e.mem.set_u32(out_column.addr(), column);
    let row = grid_coordinate(e, row_position, size);
    e.mem.set_u32(out_row.addr(), row);
}

// Translated from 00691350 (decompiled, FalloutNV.exe 1.4.0.525)
/// Lists the grid cells that triangle `triangle` can touch: clears the word
/// array `out`, finds the smallest and largest row and column of the three
/// vertices (`006912a0`), and for every cell in that rectangle whose list
/// contains the triangle (`006914e0`) appends the cell number. Returns the
/// number of cells found.
pub fn fn_00691350(
    e: &mut Engine,
    this: Ptr<NavMesh>,
    triangle: u16,
    out: Ptr<BSSimpleArray>,
) -> u32 {
    let tri = triangle_element(e, this, triangle as u32);
    e.call(ARRAY_CLEAR, &args![out, 0u32]);
    let grid: Ptr<NavMeshGrid> = this.byte_add(MESH_GRID_OFFSET).cast();
    let (mut max_row, mut min_row) = (0u32, u32::MAX);
    let (mut max_column, mut min_column) = (0u32, u32::MAX);
    for corner in 0..3i32 {
        let vertex_index = triangle_vertex(e, tri, corner);
        let vertex = fn_0068f0a0(e, this, vertex_index);
        let (row, column) = e.with_stack(8, |e, cell| {
            fn_006912a0(e, grid, vertex, cell, cell.byte_add(4));
            (e.mem.u32(cell.addr()), e.mem.u32(cell.addr() + 4))
        });
        max_row = e.call(UNSIGNED_MAX, &args![max_row, row]).u32();
        min_row = e.call(UNSIGNED_MIN, &args![min_row, row]).u32();
        max_column = e.call(UNSIGNED_MAX, &args![max_column, column]).u32();
        min_column = e.call(UNSIGNED_MIN, &args![min_column, column]).u32();
    }
    let mut row = min_row;
    while row < max_row.wrapping_add(1) {
        let mut column = min_column;
        while column < max_column.wrapping_add(1) {
            if fn_006914e0(e, grid, triangle, column, row) {
                let size = e.call(POINTER_GET, &args![grid]).u32();
                let cell_number = size.wrapping_mul(row).wrapping_add(column);
                e.with_stack(4, |e, slot| {
                    e.mem.set_u32(slot.addr(), cell_number);
                    e.call(ARRAY_PUSH_WORD, &args![out, slot]);
                });
            }
            column = column.wrapping_add(1);
        }
        row = row.wrapping_add(1);
    }
    array_size(e, out)
}

// Translated from 006914e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the list of cell `(column, row)` of the grid (`row * iGridSize +
/// column`) contains triangle `triangle` (`0069da10`).
pub fn fn_006914e0(
    e: &mut Engine,
    this: Ptr<NavMeshGrid>,
    triangle: u16,
    column: u32,
    row: u32,
) -> bool {
    let size = e.get(this, NavMeshGrid::iGridSize);
    let cell = row.wrapping_mul(size).wrapping_add(column);
    e.call(GRID_CELL_CONTAINS, &args![this, triangle, cell])
        .bool()
}

// Translated from 00691510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds a static avoid node: builds a `NavMeshStaticAvoidNode` for triangle
/// `triangle` from the point at `point`, `radius` and `cost`
/// (`00691590`), sets flag `0x80000000` on the triangle (`00691570`) and
/// appends the node to the mesh's static avoid nodes (`00692000`); returns
/// what that returns (the new index).
pub fn fn_00691510(
    e: &mut Engine,
    this: Ptr<NavMesh>,
    point: Ptr,
    triangle: u16,
    radius: f32,
    cost: f32,
) -> u32 {
    e.with_stack(0x28, |e, node| {
        fn_00691590(e, node.cast(), point, triangle, radius, cost);
        let tri = triangle_element(e, this, triangle as u32);
        fn_00691570(e, tri, TRIANGLE_FLAG_HAS_AVOID_NODE);
        let nodes = this.at(NavMesh::StaticAvoidNodes);
        e.call(AVOID_NODE_ADD, &args![nodes, node]).u32()
    })
}

// Translated from 00691570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the bits of `mask` in the triangle's flags.
pub fn fn_00691570(e: &mut Engine, this: Ptr<NavMeshTriangle>, mask: u32) {
    let flags = e.get(this, NavMeshTriangle::TriangleFlags);
    e.set(this, NavMeshTriangle::TriangleFlags, flags | mask);
}

// Translated from 00691590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshStaticAvoidNode::NavMeshStaticAvoidNode`, by its body: builds the
/// base (`006915d0`) from `point`, `radius` and `cost` and stores the
/// triangle index. Returns `this`.
pub fn fn_00691590(
    e: &mut Engine,
    this: Ptr<NavMeshStaticAvoidNode>,
    point: Ptr,
    triangle: u16,
    radius: f32,
    cost: f32,
) -> Ptr<NavMeshStaticAvoidNode> {
    fn_006915d0(e, this, point, radius, cost);
    e.set(this, NavMeshStaticAvoidNode::usTriangle, triangle);
    this
}

// Translated from 006915d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingAvoidNode::PathingAvoidNode`, by its body: copies the three floats
/// at `point` into `Point1`, leaves `Point2` to its (empty) constructor
/// `006815c0`, and stores `radius`, `cost` and type 0. Returns `this`.
pub fn fn_006915d0(
    e: &mut Engine,
    this: Ptr<NavMeshStaticAvoidNode>,
    point: Ptr,
    radius: f32,
    cost: f32,
) -> Ptr<NavMeshStaticAvoidNode> {
    for word in 0..3 {
        let value = e.mem.u32(point.addr() + 4 * word);
        e.mem.set_u32(this.addr() + 4 * word, value);
    }
    e.call(POINT_CONSTRUCTOR, &args![this.byte_add(0xc)]);
    e.set(this, NavMeshStaticAvoidNode::fRadius, x87_quiet(radius));
    e.set(this, NavMeshStaticAvoidNode::fCost, x87_quiet(cost));
    e.set(this, NavMeshStaticAvoidNode::eType, 0);
    this
}

// Translated from 00691620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Number of static avoid nodes of the mesh.
pub fn fn_00691620(e: &mut Engine, this: Ptr<NavMesh>) -> u32 {
    array_size(e, this.at(NavMesh::StaticAvoidNodes))
}

// Translated from 00691640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards `index` to `00691750` on the mesh's static avoid nodes; returns
/// the address of that node.
pub fn fn_00691640(e: &mut Engine, this: Ptr<NavMesh>, index: u32) -> Ptr {
    fn_00691750(e, this.at(NavMesh::StaticAvoidNodes), index)
}

// Translated from 00691660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMesh::GetStaticAvoidNodesForTri` (Xbox PDB): appends the address of
/// every static avoid node whose triangle is `triangle` to the word array
/// `out`; true when `out` is not empty afterwards.
pub fn nav_mesh_get_static_avoid_nodes_for_tri(
    e: &mut Engine,
    this: Ptr<NavMesh>,
    triangle: u16,
    out: Ptr<BSSimpleArray>,
) -> bool {
    let nodes = this.at(NavMesh::StaticAvoidNodes);
    let count = array_size(e, nodes);
    for index in 0..count {
        let node = fn_00691750(e, nodes, index);
        if e.call(AVOID_NODE_TRIANGLE, &args![node]).u16() == triangle {
            let node = fn_00691750(e, nodes, index);
            e.with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), node.addr());
                e.call(ARRAY_PUSH_WORD, &args![out, slot]);
            });
        }
    }
    !e.call(ARRAY_IS_EMPTY, &args![out]).bool()
}

// Translated from 006916f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the `NiTMap<unsigned short, NiPointer<ObstacleData>>`
/// (its scalar deleting destructor is `00691770`): the base constructor
/// `006918c0` with `buckets`, then the map's vtable. Returns `this`.
pub fn fn_006916f0(e: &mut Engine, this: Ptr<NiTPointerMap>, buckets: u32) -> Ptr<NiTPointerMap> {
    fn_006918c0(e, this, buckets);
    e.mem.set_u32(this.addr(), OBSTACLE_MAP_VTABLE);
    this
}

// Translated from 00691720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the `NiTMap<unsigned short, NavMeshPOVData>` (its scalar
/// deleting destructor is `006917a0`): the base constructor `00691de0` with
/// `buckets`, then the map's vtable. Returns `this`.
pub fn fn_00691720(e: &mut Engine, this: Ptr<NiTPointerMap>, buckets: u32) -> Ptr<NiTPointerMap> {
    e.call(POV_MAP_BASE_CONSTRUCTOR, &args![this, buckets]);
    e.mem.set_u32(this.addr(), POV_MAP_VTABLE);
    this
}

// Translated from 00691750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Address of element `index` of a `BSSimpleArray` of 40-byte elements (the
/// static avoid nodes): `pBuffer + index * 0x28`.
pub fn fn_00691750(e: &mut Engine, this: Ptr<BSSimpleArray>, index: u32) -> Ptr {
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    Ptr::new(index.wrapping_mul(0x28).wrapping_add(buffer))
}

// Translated from 00691770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<unsigned short, NiPointer<ObstacleData>>::scalar deleting
/// destructor` (Xbox PDB): runs the destructor body (`006919a0`) and, when bit
/// 0 of `flags` is set, frees the object. Returns `this`.
pub fn ni_t_map_obstacle_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTPointerMap>,
    flags: u32,
) -> Ptr<NiTPointerMap> {
    e.call(OBSTACLE_MAP_DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 006917a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<unsigned short, NavMeshPOVData>::scalar deleting destructor`
/// (Xbox PDB): runs the destructor body (`00691ea0`) and, when bit 0 of
/// `flags` is set, frees the object. Returns `this`.
pub fn ni_t_map_pov_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTPointerMap>,
    flags: u32,
) -> Ptr<NiTPointerMap> {
    e.call(POV_MAP_DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 006917d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of a `BSSimpleArray<unsigned short>`: stores the vtable and
/// initialises the array with `(0, 0)` (`00692290`). Returns `this`.
pub fn fn_006917d0(e: &mut Engine, this: Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray> {
    e.mem.set_u32(this.addr(), U16_ARRAY_VTABLE);
    e.call(U16_ARRAY_INITIALISE, &args![this, 0u32, 0u32]);
    this
}

// Translated from 00691800 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of a `BSSimpleArray<unsigned short>`: stores the vtable and
/// clears the array, freeing its buffer.
pub fn fn_00691800(e: &mut Engine, this: Ptr<BSSimpleArray>) {
    e.mem.set_u32(this.addr(), U16_ARRAY_VTABLE);
    e.call(ARRAY_CLEAR, &args![this, 1u32]);
}

// Translated from 00691820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends the `unsigned short` at `value` to a `BSSimpleArray<unsigned
/// short>`: makes room (`00692200`), constructs the element (`005ef650`) and
/// stores the value. Returns the new element's index.
pub fn fn_00691820(e: &mut Engine, this: Ptr<BSSimpleArray>, value: Ptr) -> u32 {
    let index = e.call(U16_ARRAY_GROW, &args![this]).u32();
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    let element = buffer.wrapping_add(index.wrapping_mul(2));
    e.call(U16_ARRAY_CONSTRUCT_ELEMENTS, &args![this, element, 1u32]);
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    let stored = e.mem.u16(value.addr());
    e.mem
        .set_u16(buffer.wrapping_add(index.wrapping_mul(2)), stored);
    index
}

// Translated from 00691870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Allocates storage for `count` `unsigned short` elements (`operator new`
/// of `count * 2` bytes). `this` is not used.
pub fn fn_00691870(e: &mut Engine, _this: Ptr<BSSimpleArray>, count: u32) -> Ptr {
    e.call(OPERATOR_NEW, &args![count.wrapping_shl(1)]).ptr()
}

// Translated from 006918a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The array's `_Reallocate` (its vtable slot `+0xC`; `BSSimpleArray`'s
/// `_Reallocate` in the Xbox PDB) for `unsigned short` elements: `0042f5d0`
/// with `block` and `count * 2`, answering the block it gives (the exe ends
/// on that call and leaves its `EAX`). `this` is not used.
pub fn fn_006918a0(e: &mut Engine, _this: Ptr<BSSimpleArray>, block: Ptr, count: u32) -> u32 {
    e.call(DEALLOCATE_SIZED, &args![block, count.wrapping_shl(1)])
        .u32()
}

// Translated from 006918c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Base constructor of the `NiTMap` instances: stores the base vtable and the
/// bucket count, sets the item count to 0, allocates `buckets * 4` bytes for
/// the bucket table (`00aa1070`) and zeroes them. Returns `this`.
pub fn fn_006918c0(e: &mut Engine, this: Ptr<NiTPointerMap>, buckets: u32) -> Ptr<NiTPointerMap> {
    e.mem.set_u32(this.addr(), MAP_BASE_VTABLE);
    e.set(this, NiTPointerMap::m_uiHashSize, buckets);
    e.set(this, NiTPointerMap::m_uiCount, 0);
    let size = buckets.wrapping_shl(2);
    let table = e.call(ALLOCATE, &args![size]).u32();
    e.set(this, NiTPointerMap::m_ppkHashTable, table);
    let size = buckets.wrapping_shl(2);
    e.call(MEMORY_SET, &args![table, 0u32, size]);
    this
}

// Translated from 00691930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<DFALL<NiTMapItem<unsigned short, NiPointer<ObstacleData>>>,
/// unsigned short, NiPointer<ObstacleData>>::SetValue` (Xbox PDB): stores
/// `key` in the item and assigns `value` (an `NiPointer` passed by value) to
/// the item's value (`006e5cc0`), then destroys the passed `NiPointer`
/// (`0045cec0`). `this` is not used. Left out: the exception-unwinding frame.
pub fn ni_t_map_base_obstacle_data_set_value(
    e: &mut Engine,
    _this: Ptr<NiTPointerMap>,
    item: Ptr,
    key: u16,
    value: Ptr,
) {
    e.mem.set_u16(item.addr() + 4, key);
    e.with_stack(4, |e, argument| {
        e.mem.set_u32(argument.addr(), value.addr());
        e.call(POINTER_ASSIGN_FROM, &args![item.byte_add(8), argument]);
        e.call(POINTER_RELEASE, &args![argument]);
    });
}

// Translated from 00691a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the vertex array (`BSSimpleArray<NavMeshVertex>`): stores
/// the vtable and initialises the array with `(0, 0)` (`0069ba70`). Returns
/// `this`.
pub fn fn_00691a60(e: &mut Engine, this: Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray> {
    e.mem.set_u32(this.addr(), VERTEX_ARRAY_VTABLE);
    e.call(VERTEX_ARRAY_INITIALISE, &args![this, 0u32, 0u32]);
    this
}

// Translated from 00691a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the vertex array: stores the vtable and clears the array,
/// freeing its buffer.
pub fn fn_00691a90(e: &mut Engine, this: Ptr<BSSimpleArray>) {
    e.mem.set_u32(this.addr(), VERTEX_ARRAY_VTABLE);
    e.call(ARRAY_CLEAR, &args![this, 1u32]);
}

// Translated from 00691ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends a copy of the 12-byte element at `value` to a `BSSimpleArray` of
/// 12-byte elements (the vertices): makes room (`00978bc0`), constructs the
/// element (`0069b9d0`) and copies the three words. Returns the new
/// element's index.
pub fn fn_00691ab0(e: &mut Engine, this: Ptr<BSSimpleArray>, value: Ptr) -> u32 {
    let index = e.call(VERTEX_ARRAY_GROW, &args![this]).u32();
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    let element = index.wrapping_mul(12).wrapping_add(buffer);
    e.call(VERTEX_ARRAY_CONSTRUCT_ELEMENTS, &args![this, element, 1u32]);
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    let element = index.wrapping_mul(12).wrapping_add(buffer);
    for word in 0..3 {
        let copied = e.mem.u32(value.addr() + 4 * word);
        e.mem.set_u32(element + 4 * word, copied);
    }
    index
}

// Translated from 00691b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the triangle array (`BSSimpleArray<NavMeshTriangle>`):
/// stores the vtable and initialises the array with `(0, 0)` (`00644400`).
/// Returns `this`.
pub fn fn_00691b10(e: &mut Engine, this: Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray> {
    e.mem.set_u32(this.addr(), TRIANGLE_ARRAY_VTABLE);
    e.call(TRIANGLE_ARRAY_INITIALISE, &args![this, 0u32, 0u32]);
    this
}

// Translated from 00691b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the triangle array: stores the vtable and clears the array,
/// freeing its buffer.
pub fn fn_00691b40(e: &mut Engine, this: Ptr<BSSimpleArray>) {
    e.mem.set_u32(this.addr(), TRIANGLE_ARRAY_VTABLE);
    e.call(ARRAY_CLEAR, &args![this, 1u32]);
}

// Translated from 00691b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends a copy of the 16-byte element at `value` to a `BSSimpleArray` of
/// 16-byte elements (the triangles): makes room (`006f31f0`), constructs the
/// element (`00644490`) and copies the four words. Returns the new element's
/// index.
pub fn fn_00691b60(e: &mut Engine, this: Ptr<BSSimpleArray>, value: Ptr) -> u32 {
    let index = e.call(TRIANGLE_ARRAY_GROW, &args![this]).u32();
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    let element = index.wrapping_shl(4).wrapping_add(buffer);
    e.call(
        TRIANGLE_ARRAY_CONSTRUCT_ELEMENTS,
        &args![this, element, 1u32],
    );
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    let element = index.wrapping_shl(4).wrapping_add(buffer);
    for word in 0..4 {
        let copied = e.mem.u32(value.addr() + 4 * word);
        e.mem.set_u32(element + 4 * word, copied);
    }
    index
}

// Translated from 00691bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the extra edge info array (`BSSimpleArray<EdgeExtraInfo>`):
/// stores the vtable and initialises the array with `(0, 0)` (`006923f0`).
/// Returns `this`.
pub fn fn_00691bc0(e: &mut Engine, this: Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray> {
    e.mem.set_u32(this.addr(), EXTRA_INFO_ARRAY_VTABLE);
    e.call(EXTRA_INFO_ARRAY_INITIALISE, &args![this, 0u32, 0u32]);
    this
}

// Translated from 00691bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the extra edge info array: stores the vtable and clears the
/// array, freeing its buffer.
pub fn fn_00691bf0(e: &mut Engine, this: Ptr<BSSimpleArray>) {
    e.mem.set_u32(this.addr(), EXTRA_INFO_ARRAY_VTABLE);
    e.call(ARRAY_CLEAR, &args![this, 1u32]);
}

// Translated from 00691c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends a copy of the `EdgeExtraInfo` at `value` to the extra edge info
/// array: makes room (`00978bc0`), constructs the element (`00692320`) and
/// copy-constructs it from `value` (`00691c60`). Returns the new element's
/// index.
pub fn fn_00691c10(e: &mut Engine, this: Ptr<BSSimpleArray>, value: Ptr) -> u32 {
    let index = e.call(EXTRA_INFO_ARRAY_GROW, &args![this]).u32();
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    let element = index.wrapping_mul(12).wrapping_add(buffer);
    e.call(EXTRA_INFO_CONSTRUCT_ELEMENTS, &args![this, element, 1u32]);
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    let element = index.wrapping_mul(12).wrapping_add(buffer);
    e.call(EXTRA_INFO_COPY_CONSTRUCT, &args![element, value]);
    index
}

// Translated from 00691d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the door portal array
/// (`BSSimpleArray<NavMeshTriangleDoorPortal>`): stores the vtable and
/// initialises the array with `(0, 0)` (`00692520`). Returns `this`.
pub fn fn_00691d40(e: &mut Engine, this: Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray> {
    e.mem.set_u32(this.addr(), DOOR_PORTAL_ARRAY_VTABLE);
    e.call(DOOR_ARRAY_INITIALISE, &args![this, 0u32, 0u32]);
    this
}

// Translated from 00691d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the door portal array: stores the vtable and clears the
/// array, freeing its buffer.
pub fn fn_00691d70(e: &mut Engine, this: Ptr<BSSimpleArray>) {
    e.mem.set_u32(this.addr(), DOOR_PORTAL_ARRAY_VTABLE);
    e.call(ARRAY_CLEAR, &args![this, 1u32]);
}

// Translated from 00691d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the closed door array
/// (`BSSimpleArray<NavMeshClosedDoorInfo>`): stores the vtable and
/// initialises the array with `(0, 0)` (`00692520`). Returns `this`.
pub fn fn_00691d90(e: &mut Engine, this: Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray> {
    e.mem.set_u32(this.addr(), CLOSED_DOOR_ARRAY_VTABLE);
    e.call(DOOR_ARRAY_INITIALISE, &args![this, 0u32, 0u32]);
    this
}

// Translated from 00691dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the closed door array: stores the vtable and clears the
/// array, freeing its buffer.
pub fn fn_00691dc0(e: &mut Engine, this: Ptr<BSSimpleArray>) {
    e.mem.set_u32(this.addr(), CLOSED_DOOR_ARRAY_VTABLE);
    e.call(ARRAY_CLEAR, &args![this, 1u32]);
}

// Translated from 00691de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Base constructor of the `NavMeshPOVData` `NiTMap`: like `006918c0` but
/// with its own base vtable. Stores the vtable and the bucket count, sets
/// the item count to 0, allocates `buckets * 4` bytes (`00aa1070`) for the
/// bucket table and zeroes them. Returns `this`.
pub fn fn_00691de0(e: &mut Engine, this: Ptr<NiTPointerMap>, buckets: u32) -> Ptr<NiTPointerMap> {
    e.mem.set_u32(this.addr(), MAP_ITEM_BASE_VTABLE);
    e.set(this, NiTPointerMap::m_uiHashSize, buckets);
    e.set(this, NiTPointerMap::m_uiCount, 0);
    let size = buckets.wrapping_shl(2);
    let table = e.call(ALLOCATE, &args![size]).u32();
    e.set(this, NiTPointerMap::m_ppkHashTable, table);
    let size = buckets.wrapping_shl(2);
    e.call(MEMORY_SET, &args![table, 0u32, size]);
    this
}

// Translated from 00691e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<DFALL<NiTMapItem<unsigned short, NiPointer<ObstacleData>>>,
/// unsigned short, NiPointer<ObstacleData>>::IsKeysEqual` (Xbox PDB): whether
/// the two `unsigned short` keys are equal. `this` is not used.
pub fn ni_t_map_base_obstacle_data_is_keys_equal(
    _e: &mut Engine,
    _this: Ptr<NiTPointerMap>,
    key_1: u16,
    key_2: u16,
) -> bool {
    key_1 == key_2
}

// Translated from 00691e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<DFALL<NiTMapItem<unsigned short, NavMeshPOVData>>, unsigned
/// short, NavMeshPOVData>::SetValue` (Xbox PDB): stores `key` at `item + 4`
/// and the 4-byte `value` at `item + 6` (the item is packed). `this` is not
/// used.
pub fn ni_t_map_base_pov_data_set_value(
    e: &mut Engine,
    _this: Ptr<NiTPointerMap>,
    item: Ptr,
    key: u16,
    value: u32,
) {
    e.mem.set_u16(item.addr() + 4, key);
    e.mem.set_u32(item.addr() + 6, value);
}

// Translated from 00691ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `NavMeshPOVData` `NiTMap`: stores the map's vtable,
/// removes all items (`00438af0`) and runs the base destructor (`00691f00`).
/// Left out: the exception-unwinding frame.
pub fn fn_00691ea0(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    e.mem.set_u32(this.addr(), POV_MAP_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    fn_00691f00(e, this);
}

// Translated from 00691f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Base destructor of the `NavMeshPOVData` `NiTMap`: stores the base vtable,
/// removes all items (`00438af0`) and frees the bucket table (`00aa10f0`).
pub fn fn_00691f00(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    e.mem.set_u32(this.addr(), MAP_ITEM_BASE_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    e.call(FREE_TABLE, &args![table]);
}

// Translated from 00691f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases an item of the `NavMeshPOVData` `NiTMap`: resets the value at
/// `item + 6` (`0065de30`, argument 0) and hands the item back to the map's
/// allocator (`006b8310` on `this + 0xc`).
pub fn fn_00691f30(e: &mut Engine, this: Ptr<NiTPointerMap>, item: Ptr) {
    e.call(POV_ITEM_RESET_VALUE, &args![item.byte_add(6), 0u32]);
    e.call(POV_ITEM_FREE, &args![this.byte_add(0xc), item]);
}

// Translated from 00691f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the obstacle array
/// (`BSSimpleArray<NiPointer<ObstacleUndoData>>`): stores the vtable and
/// initialises the array with `(0, 0)` (`00822860`). Returns `this`.
pub fn fn_00691f60(e: &mut Engine, this: Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray> {
    e.mem.set_u32(this.addr(), OBSTACLE_ARRAY_VTABLE);
    e.call(OBSTACLES_INITIALISE, &args![this, 0u32, 0u32]);
    this
}

// Translated from 00691f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NiPointer<ObstacleUndoData>, 1024>::~BSSimpleArray`
/// (Xbox PDB): stores the vtable and clears the array (`006c6200`, argument
/// 1: free the buffer).
pub fn fn_00691f90(e: &mut Engine, this: Ptr<BSSimpleArray>) {
    e.mem.set_u32(this.addr(), OBSTACLE_ARRAY_VTABLE);
    e.call(OBSTACLES_CLEAR, &args![this, 1u32]);
}

// Translated from 00691fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the static avoid node array: stores the vtable and
/// initialises the array with `(0, 0)` (`00692740`). Returns `this`.
pub fn fn_00691fb0(e: &mut Engine, this: Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray> {
    e.mem.set_u32(this.addr(), AVOID_NODE_ARRAY_VTABLE);
    fn_00692740(e, this, 0, 0);
    this
}

// Translated from 00691fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the static avoid node array: stores the vtable and clears
/// the array, freeing its buffer.
pub fn fn_00691fe0(e: &mut Engine, this: Ptr<BSSimpleArray>) {
    e.mem.set_u32(this.addr(), AVOID_NODE_ARRAY_VTABLE);
    e.call(ARRAY_CLEAR, &args![this, 1u32]);
}

// Translated from 00692000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends a copy of the 40-byte avoid node at `value` to the static avoid
/// node array: makes room (`006925b0`), constructs the element (`00692640`)
/// and copies the ten words. Returns the new element's index.
pub fn fn_00692000(e: &mut Engine, this: Ptr<BSSimpleArray>, value: Ptr) -> u32 {
    let index = e.call(AVOID_NODE_ARRAY_GROW, &args![this]).u32();
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    let element = index.wrapping_mul(0x28).wrapping_add(buffer);
    e.call(AVOID_NODE_CONSTRUCT_ELEMENTS, &args![this, element, 1u32]);
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    let element = index.wrapping_mul(0x28).wrapping_add(buffer);
    for word in 0..10 {
        let copied = e.mem.u32(value.addr() + 4 * word);
        e.mem.set_u32(element + 4 * word, copied);
    }
    index
}

// Translated from 00692050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<DFALL<NiTMapItem<unsigned short, NiPointer<ObstacleData>>>,
/// unsigned short, NiPointer<ObstacleData>>::scalar deleting destructor`
/// (Xbox PDB): runs the destructor body (`00691a00`) and, when bit 0 of
/// `flags` is set, frees the object. Returns `this`.
pub fn ni_t_map_base_obstacle_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTPointerMap>,
    flags: u32,
) -> Ptr<NiTPointerMap> {
    e.call(OBSTACLE_MAP_BASE_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// Shared tail of the scalar deleting destructors below: frees the object
/// when bit 0 of `flags` is set.
fn delete_if_flagged(e: &mut Engine, this: Ptr<BSSimpleArray>, flags: u32) -> Ptr<BSSimpleArray> {
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00692080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NavMeshVertex, 1024>::scalar deleting destructor` (Xbox
/// PDB): runs the destructor (`00691a90`) and, when bit 0 of `flags` is set,
/// frees the object. Returns `this`.
pub fn bs_simple_array_vertex_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BSSimpleArray>,
    flags: u32,
) -> Ptr<BSSimpleArray> {
    fn_00691a90(e, this);
    delete_if_flagged(e, this, flags)
}

// Translated from 006920b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NavMeshTriangle, 1024>::scalar deleting destructor` (Xbox
/// PDB): runs the destructor (`00691b40`) and, when bit 0 of `flags` is set,
/// frees the object. Returns `this`.
pub fn bs_simple_array_triangle_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BSSimpleArray>,
    flags: u32,
) -> Ptr<BSSimpleArray> {
    fn_00691b40(e, this);
    delete_if_flagged(e, this, flags)
}

// Translated from 006920e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<EdgeExtraInfo, 1024>::scalar deleting destructor` (Xbox
/// PDB): runs the destructor (`00691bf0`) and, when bit 0 of `flags` is set,
/// frees the object. Returns `this`.
pub fn bs_simple_array_edge_extra_info_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BSSimpleArray>,
    flags: u32,
) -> Ptr<BSSimpleArray> {
    fn_00691bf0(e, this);
    delete_if_flagged(e, this, flags)
}

// Translated from 00692110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NavMeshTriangleDoorPortal, 1024>::scalar deleting
/// destructor` (Xbox PDB): runs the destructor (`00691d70`) and, when bit 0
/// of `flags` is set, frees the object. Returns `this`.
pub fn bs_simple_array_door_portal_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BSSimpleArray>,
    flags: u32,
) -> Ptr<BSSimpleArray> {
    fn_00691d70(e, this);
    delete_if_flagged(e, this, flags)
}

// Translated from 00692140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NavMeshClosedDoorInfo, 1024>::scalar deleting destructor`
/// (Xbox PDB): runs the destructor (`00691dc0`) and, when bit 0 of `flags` is
/// set, frees the object. Returns `this`.
pub fn bs_simple_array_closed_door_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BSSimpleArray>,
    flags: u32,
) -> Ptr<BSSimpleArray> {
    fn_00691dc0(e, this);
    delete_if_flagged(e, this, flags)
}

// Translated from 00692170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<DFALL<NiTMapItem<unsigned short, NavMeshPOVData>>, unsigned
/// short, NavMeshPOVData>::scalar deleting destructor` (Xbox PDB): runs the
/// base destructor (`00691f00`) and, when bit 0 of `flags` is set, frees the
/// object. Returns `this`.
pub fn ni_t_map_base_pov_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTPointerMap>,
    flags: u32,
) -> Ptr<NiTPointerMap> {
    fn_00691f00(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 006921a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NiPointer<ObstacleUndoData>, 1024>::scalar deleting
/// destructor` (Xbox PDB): runs the destructor (`00691f90`) and, when bit 0
/// of `flags` is set, frees the object. Returns `this`.
pub fn bs_simple_array_obstacle_undo_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BSSimpleArray>,
    flags: u32,
) -> Ptr<BSSimpleArray> {
    fn_00691f90(e, this);
    delete_if_flagged(e, this, flags)
}

// Translated from 006921d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NavMeshStaticAvoidNode, 1024>::scalar deleting destructor`
/// (Xbox PDB): runs the destructor (`00691fe0`) and, when bit 0 of `flags` is
/// set, frees the object. Returns `this`.
pub fn bs_simple_array_static_avoid_node_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BSSimpleArray>,
    flags: u32,
) -> Ptr<BSSimpleArray> {
    fn_00691fe0(e, this);
    delete_if_flagged(e, this, flags)
}

// Translated from 00692200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Makes room for one more `unsigned short` element and returns its index
/// (the old size). When the array is full (`00438b90`): with no capacity
/// yet, allocates 4 elements through the array's virtual allocator (slot 4);
/// otherwise takes the next capacity (`009a3910`) and resizes (`005ef6a0`).
/// Then increments the size.
pub fn fn_00692200(e: &mut Engine, this: Ptr<BSSimpleArray>) -> u32 {
    let full = e.call(U16_ARRAY_IS_FULL, &args![this]).u32() & 0xff != 0;
    if full {
        if e.get(this, BSSimpleArray::iReservedSize) == 0 {
            let capacity = U16_ARRAY_FIRST_CAPACITY;
            let block = e
                .vcall(this.addr(), ARRAY_ALLOCATE_SLOT, &args![capacity])
                .u32();
            e.set(this, BSSimpleArray::pBuffer, block);
            e.set(this, BSSimpleArray::iReservedSize, capacity);
        } else {
            let capacity = e.call(U16_ARRAY_NEXT_CAPACITY, &args![this]).u32();
            let size = e.get(this, BSSimpleArray::iSize);
            e.call(U16_ARRAY_RESIZE, &args![this, capacity, size]);
            e.set(this, BSSimpleArray::iReservedSize, capacity);
        }
    }
    let size = e.get(this, BSSimpleArray::iSize).wrapping_add(1);
    e.set(this, BSSimpleArray::iSize, size);
    size.wrapping_sub(1)
}

// Translated from 00692740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initialiser of the static avoid node array: empties it, raises `capacity`
/// to at least `count`, allocates that many 40-byte elements through the
/// array's virtual allocator (slot 4) and constructs `count` of them
/// (`00692640`), setting the size to `count`.
pub fn fn_00692740(e: &mut Engine, this: Ptr<BSSimpleArray>, capacity: u32, count: u32) {
    e.set(this, BSSimpleArray::pBuffer, 0);
    e.set(this, BSSimpleArray::iSize, 0);
    e.set(this, BSSimpleArray::iReservedSize, 0);
    let capacity = capacity.max(count);
    if capacity > 0 {
        let block = e
            .vcall(this.addr(), ARRAY_ALLOCATE_SLOT, &args![capacity])
            .u32();
        e.set(this, BSSimpleArray::pBuffer, block);
        e.set(this, BSSimpleArray::iReservedSize, capacity);
    }
    if count > 0 {
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        e.call(AVOID_NODE_CONSTRUCT_ELEMENTS, &args![this, buffer, count]);
        e.set(this, BSSimpleArray::iSize, count);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0049c4c0,
            ni_t_map_base_key_to_hash_index(Ptr<NiTPointerMap>, u16) -> u32
        ),
        entry!(0x0068eb80, nav_mesh_nav_mesh(Ptr<NavMesh>) -> Ptr<NavMesh>),
        entry!(0x0068ed30, fn_0068ed30(Ptr<NavMesh>, u32) -> Ptr<NavMesh>),
        entry!(0x0068ed70, fn_0068ed70(Ptr<NavMesh>)),
        entry!(0x0068ef50, fn_0068ef50(Ptr) -> u32),
        entry!(0x0068ef70, nav_mesh_get_parent_space(Ptr<NavMesh>) -> Ptr),
        entry!(0x0068efb0, fn_0068efb0(Ptr<NavMesh>) -> Ptr),
        entry!(0x0068efd0, fn_0068efd0(Ptr<NavMesh>, Ptr, u16, i32) -> Ptr),
        entry!(
            0x0068f040,
            nav_mesh_get_edge_vertices(Ptr<NavMesh>, Ptr, u16, i32) -> Ptr
        ),
        entry!(0x0068f0a0, fn_0068f0a0(Ptr<NavMesh>, u16) -> Ptr),
        entry!(
            0x0068f0c0,
            fn_0068f0c0(Ptr<NavMesh>, u16, i32, Ptr, Ptr) -> bool
        ),
        entry!(0x0068f160, fn_0068f160(Ptr<NavMesh>, u16, Ptr, Ptr, Ptr)),
        entry!(0x0068f1d0, fn_0068f1d0(Ptr<NavMeshTriangle>, u32) -> bool),
        entry!(0x0068f200, fn_0068f200(Ptr<NavMeshTriangle>, i32) -> bool),
        entry!(
            0x0068f230,
            nav_mesh_get_edge_extra_info(Ptr<NavMesh>, u16, i32) -> Ptr<EdgeExtraInfo>
        ),
        entry!(0x0068f2c0, fn_0068f2c0(Ptr<NavMeshTriangle>, i32) -> u16),
        entry!(0x0068f2e0, fn_0068f2e0(Ptr, Ptr) -> bool),
        entry!(0x0068f320, fn_0068f320(Ptr) -> bool),
        entry!(
            0x0068f340,
            nav_mesh_get_matching_edge(Ptr<NavMesh>, Ptr, Ptr) -> bool
        ),
        entry!(
            0x0068f3b0,
            nav_mesh_get_matching_edge_ov2(Ptr<NavMesh>, u16, i32, Ptr, Ptr, Ptr) -> bool
        ),
        entry!(
            0x0068f460,
            nav_mesh_get_matching_edge_ov3(Ptr<NavMesh>, u16, i32, Ptr, Ptr, Ptr) -> bool
        ),
        entry!(
            0x0068f670,
            nav_mesh_get_matching_edge_ov4(Ptr<NavMesh>, Ptr, Ptr) -> bool
        ),
        entry!(0x0068f6b0, fn_0068f6b0(Ptr<NavMesh>, u16) -> u32),
        entry!(
            0x0068f6d0,
            nav_mesh_check_nav_mesh(Ptr<NavMesh>, u32) -> bool
        ),
        entry!(0x00690770, fn_00690770(Ptr<NavMeshTriangle>) -> bool),
        entry!(0x00690790, fn_00690790(Ptr<NavMeshTriangle>, u32)),
        entry!(0x006907d0, fn_006907d0(Ptr<NavMeshTriangle>, u32, u16)),
        entry!(0x00690800, fn_00690800(Ptr) -> Ptr),
        entry!(0x00690830, fn_00690830(Ptr) -> u32),
        entry!(0x00690860, fn_00690860(Ptr, Ptr) -> bool),
        entry!(0x00690890, fn_00690890(Ptr, Ptr)),
        entry!(0x006908f0, fn_006908f0(Ptr<NavMesh>) -> bool),
        entry!(0x00690d50, nav_mesh_clear_nav_mesh(Ptr<NavMesh>, u32)),
        entry!(0x00690df0, fn_00690df0(Ptr<NavMeshTriangle>, u32, u16)),
        entry!(0x00690e10, fn_00690e10(Ptr<NavMeshTriangle>) -> bool),
        entry!(0x00690e30, fn_00690e30(Ptr<NavMeshTriangle>, u32, u16)),
        entry!(0x00690e70, fn_00690e70(Ptr<NavMeshTriangle>) -> bool),
        entry!(
            0x00690e90,
            nav_mesh_delete_extra_edge_info(Ptr<NavMesh>, u16)
        ),
        entry!(0x00690fe0, fn_00690fe0(Ptr<NavMesh>, Ptr) -> u32),
        entry!(0x00691000, fn_00691000(Ptr<NavMesh>, Ptr) -> u32),
        entry!(0x00691020, fn_00691020(Ptr<NavMesh>, Ptr) -> u32),
        entry!(
            0x00691040,
            fn_00691040(Ptr<NavMeshTriangle>, u16, Ptr, Ptr, Ptr)
        ),
        entry!(0x006910a0, fn_006910a0(Ptr<NavMesh>)),
        entry!(0x00691140, fn_00691140(Ptr<NavMeshTriangle>, u32) -> bool),
        entry!(
            0x00691160,
            fn_00691160(Ptr<NavMeshTriangle>) -> Ptr<NavMeshTriangle>
        ),
        entry!(0x006911c0, fn_006911c0(Ptr<NavMesh>) -> u32),
        entry!(0x006911f0, fn_006911f0(Ptr<NavMesh>, u32) -> Ptr),
        entry!(0x00691210, fn_00691210(Ptr<NavMeshGrid>, u32) -> Ptr),
        entry!(0x00691240, fn_00691240(Ptr<NavMesh>, Ptr) -> u32),
        entry!(0x00691260, fn_00691260(Ptr<NavMeshGrid>, Ptr) -> u32),
        entry!(0x006912a0, fn_006912a0(Ptr<NavMeshGrid>, Ptr, Ptr, Ptr)),
        entry!(
            0x00691350,
            fn_00691350(Ptr<NavMesh>, u16, Ptr<BSSimpleArray>) -> u32
        ),
        entry!(
            0x006914e0,
            fn_006914e0(Ptr<NavMeshGrid>, u16, u32, u32) -> bool
        ),
        entry!(
            0x00691510,
            fn_00691510(Ptr<NavMesh>, Ptr, u16, f32, f32) -> u32
        ),
        entry!(0x00691570, fn_00691570(Ptr<NavMeshTriangle>, u32)),
        entry!(
            0x00691590,
            fn_00691590(
                Ptr<NavMeshStaticAvoidNode>,
                Ptr,
                u16,
                f32,
                f32,
            ) -> Ptr<NavMeshStaticAvoidNode>
        ),
        entry!(
            0x006915d0,
            fn_006915d0(Ptr<NavMeshStaticAvoidNode>, Ptr, f32, f32) -> Ptr<NavMeshStaticAvoidNode>
        ),
        entry!(0x00691620, fn_00691620(Ptr<NavMesh>) -> u32),
        entry!(0x00691640, fn_00691640(Ptr<NavMesh>, u32) -> Ptr),
        entry!(
            0x00691660,
            nav_mesh_get_static_avoid_nodes_for_tri(Ptr<NavMesh>, u16, Ptr<BSSimpleArray>) -> bool
        ),
        entry!(
            0x006916f0,
            fn_006916f0(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x00691720,
            fn_00691720(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(0x00691750, fn_00691750(Ptr<BSSimpleArray>, u32) -> Ptr),
        entry!(
            0x00691770,
            ni_t_map_obstacle_data_scalar_deleting_destructor(
                Ptr<NiTPointerMap>,
                u32,
            ) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x006917a0,
            ni_t_map_pov_data_scalar_deleting_destructor(
                Ptr<NiTPointerMap>,
                u32,
            ) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x006917d0,
            fn_006917d0(Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray>
        ),
        entry!(0x00691800, fn_00691800(Ptr<BSSimpleArray>)),
        entry!(0x00691820, fn_00691820(Ptr<BSSimpleArray>, Ptr) -> u32),
        entry!(0x00691870, fn_00691870(Ptr<BSSimpleArray>, u32) -> Ptr),
        entry!(0x006918a0, fn_006918a0(Ptr<BSSimpleArray>, Ptr, u32) -> u32),
        entry!(
            0x006918c0,
            fn_006918c0(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x00691930,
            ni_t_map_base_obstacle_data_set_value(Ptr<NiTPointerMap>, Ptr, u16, Ptr)
        ),
        entry!(
            0x00691a60,
            fn_00691a60(Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray>
        ),
        entry!(0x00691a90, fn_00691a90(Ptr<BSSimpleArray>)),
        entry!(0x00691ab0, fn_00691ab0(Ptr<BSSimpleArray>, Ptr) -> u32),
        entry!(
            0x00691b10,
            fn_00691b10(Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray>
        ),
        entry!(0x00691b40, fn_00691b40(Ptr<BSSimpleArray>)),
        entry!(0x00691b60, fn_00691b60(Ptr<BSSimpleArray>, Ptr) -> u32),
        entry!(
            0x00691bc0,
            fn_00691bc0(Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray>
        ),
        entry!(0x00691bf0, fn_00691bf0(Ptr<BSSimpleArray>)),
        entry!(0x00691c10, fn_00691c10(Ptr<BSSimpleArray>, Ptr) -> u32),
        entry!(
            0x00691d40,
            fn_00691d40(Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray>
        ),
        entry!(0x00691d70, fn_00691d70(Ptr<BSSimpleArray>)),
        entry!(
            0x00691d90,
            fn_00691d90(Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray>
        ),
        entry!(0x00691dc0, fn_00691dc0(Ptr<BSSimpleArray>)),
        entry!(
            0x00691de0,
            fn_00691de0(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x00691e50,
            ni_t_map_base_obstacle_data_is_keys_equal(Ptr<NiTPointerMap>, u16, u16) -> bool
        ),
        entry!(
            0x00691e70,
            ni_t_map_base_pov_data_set_value(Ptr<NiTPointerMap>, Ptr, u16, u32)
        ),
        entry!(0x00691ea0, fn_00691ea0(Ptr<NiTPointerMap>)),
        entry!(0x00691f00, fn_00691f00(Ptr<NiTPointerMap>)),
        entry!(0x00691f30, fn_00691f30(Ptr<NiTPointerMap>, Ptr)),
        entry!(
            0x00691f60,
            fn_00691f60(Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray>
        ),
        entry!(0x00691f90, fn_00691f90(Ptr<BSSimpleArray>)),
        entry!(
            0x00691fb0,
            fn_00691fb0(Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray>
        ),
        entry!(0x00691fe0, fn_00691fe0(Ptr<BSSimpleArray>)),
        entry!(0x00692000, fn_00692000(Ptr<BSSimpleArray>, Ptr) -> u32),
        entry!(
            0x00692050,
            ni_t_map_base_obstacle_data_scalar_deleting_destructor(
                Ptr<NiTPointerMap>,
                u32,
            )
                -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x00692080,
            bs_simple_array_vertex_scalar_deleting_destructor(
                Ptr<BSSimpleArray>,
                u32,
            ) -> Ptr<BSSimpleArray>
        ),
        entry!(
            0x006920b0,
            bs_simple_array_triangle_scalar_deleting_destructor(
                Ptr<BSSimpleArray>,
                u32,
            ) -> Ptr<BSSimpleArray>
        ),
        entry!(
            0x006920e0,
            bs_simple_array_edge_extra_info_scalar_deleting_destructor(
                Ptr<BSSimpleArray>,
                u32,
            )
                -> Ptr<BSSimpleArray>
        ),
        entry!(
            0x00692110,
            bs_simple_array_door_portal_scalar_deleting_destructor(
                Ptr<BSSimpleArray>,
                u32,
            )
                -> Ptr<BSSimpleArray>
        ),
        entry!(
            0x00692140,
            bs_simple_array_closed_door_scalar_deleting_destructor(
                Ptr<BSSimpleArray>,
                u32,
            )
                -> Ptr<BSSimpleArray>
        ),
        entry!(
            0x00692170,
            ni_t_map_base_pov_data_scalar_deleting_destructor(
                Ptr<NiTPointerMap>,
                u32,
            ) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x006921a0,
            bs_simple_array_obstacle_undo_scalar_deleting_destructor(
                Ptr<BSSimpleArray>,
                u32,
            )
                -> Ptr<BSSimpleArray>
        ),
        entry!(
            0x006921d0,
            bs_simple_array_static_avoid_node_scalar_deleting_destructor(
                Ptr<BSSimpleArray>,
                u32,
            )
                -> Ptr<BSSimpleArray>
        ),
        entry!(0x00692200, fn_00692200(Ptr<BSSimpleArray>) -> u32),
        entry!(0x00692740, fn_00692740(Ptr<BSSimpleArray>, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type CallLog = Vec<(u32, Vec<u32>)>;

    fn result(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// Registers `do nothing, return 0` doubles.
    fn noops(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, |_, _| Ret::default());
        }
    }

    /// The arguments of every call to `address` in the log.
    fn calls_to(log: &CallLog, address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// An engine whose array and triangle getters read the real memory
    /// layout of a `BSSimpleArray` (buffer +4, size +8), the way the game's
    /// own getters do.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(ARRAY_SIZE, |e, a| result(e.mem.u32(a[0] + 8)));
        e.register(ELEMENT_12, |e, a| result(e.mem.u32(a[0] + 4) + a[1] * 12));
        e.register(TRIANGLE_ELEMENT, |e, a| {
            result(e.mem.u32(a[0] + 4) + a[1] * 16)
        });
        e.register(NAV_MESH_GET_TRIANGLE, |e, a| {
            result(e.mem.u32(a[0] + 0x38 + 4) + (a[1] & 0xffff) * 16)
        });
        e.register(TRIANGLE_VERTEX, |e, a| {
            result(e.mem.u16(a[0] + 2 * a[1]) as u32)
        });
        e.register(NAV_MESH_TRIANGLE_COUNT, |e, a| {
            result(e.mem.u32(a[0] + 0x38 + 8))
        });
        e
    }

    fn put_array(e: &mut Engine, array: Ptr<BSSimpleArray>, data: &[u8], count: u32) {
        let buffer = e.mem.alloc(data.len().max(8) as u32);
        e.mem.write(buffer, data);
        e.mem.set_u32(array.addr() + 4, buffer);
        e.mem.set_u32(array.addr() + 8, count);
    }

    /// A triangle record: vertex indexes, neighbour slots, flags.
    fn triangle_bytes(vertices: [u16; 3], neighbours: [u16; 3], flags: u32) -> Vec<u8> {
        let mut bytes = Vec::new();
        for word in vertices.iter().chain(neighbours.iter()) {
            bytes.extend_from_slice(&word.to_le_bytes());
        }
        bytes.extend_from_slice(&flags.to_le_bytes());
        bytes
    }

    fn vertex_bytes(vertices: &[[f32; 3]]) -> Vec<u8> {
        vertices
            .iter()
            .flat_map(|v| v.iter().flat_map(|c| c.to_le_bytes()))
            .collect()
    }

    type TriangleSpec = ([u16; 3], [u16; 3], u32);

    /// A mesh with the given vertices and `(vertices, neighbours, flags)`
    /// triangles.
    fn mesh_with(
        e: &mut Engine,
        vertices: &[[f32; 3]],
        triangles: &[TriangleSpec],
    ) -> Ptr<NavMesh> {
        let mesh: Ptr<NavMesh> = e.new_object();
        put_array(
            e,
            mesh.at(NavMesh::Vertices),
            &vertex_bytes(vertices),
            vertices.len() as u32,
        );
        let bytes: Vec<u8> = triangles
            .iter()
            .flat_map(|(v, n, f)| triangle_bytes(*v, *n, *f))
            .collect();
        put_array(
            e,
            mesh.at(NavMesh::Triangles),
            &bytes,
            triangles.len() as u32,
        );
        mesh
    }

    fn triangle_ptr(e: &Engine, mesh: Ptr<NavMesh>, index: u32) -> Ptr<NavMeshTriangle> {
        let base = e.mem.u32(mesh.addr() + 0x38 + 4);
        Ptr::new(base + index * 16)
    }

    /// Two triangles `(0, 1, 2)` and `(1, 3, 2)` sharing the edge 1-2, the
    /// first one's edge 1 is the second one's edge 2.
    fn two_triangles(e: &mut Engine) -> Ptr<NavMesh> {
        mesh_with(
            e,
            &[
                [0.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [1.0, 1.0, 0.0],
            ],
            &[
                ([0, 1, 2], [0xffff, 1, 0xffff], 0),
                ([1, 3, 2], [0xffff, 0xffff, 0], 0),
            ],
        )
    }

    #[test]
    fn key_to_hash_index_is_the_key_modulo_the_bucket_count() {
        let mut e = Engine::new();
        let map: Ptr<NiTPointerMap> = e.new_object();
        e.set(map, NiTPointerMap::m_uiHashSize, 37);
        assert_eq!(e.call(0x0049_c4c0, &args![map, 100u16]).u32(), 100 % 37);
        assert_eq!(
            e.call(0x0049_c4c0, &args![map, 0xffffu16]).u32(),
            0xffff % 37
        );
    }

    /// Doubles for everything the constructor and destructor call.
    fn member_doubles(e: &mut Engine) {
        noops(
            e,
            &[
                TES_FORM_CONSTRUCTOR,
                TES_FORM_DESTRUCTOR,
                CHILD_CELL_CONSTRUCTOR,
                REF_OBJECT_CONSTRUCTOR,
                REF_OBJECT_DESTRUCTOR,
                ARRAY_CONSTRUCTOR_VERTICES,
                ARRAY_DESTRUCTOR_VERTICES,
                ARRAY_CONSTRUCTOR_TRIANGLES,
                ARRAY_DESTRUCTOR_TRIANGLES,
                ARRAY_CONSTRUCTOR_EXTRA_INFO,
                ARRAY_DESTRUCTOR_EXTRA_INFO,
                ARRAY_CONSTRUCTOR_DOOR_PORTALS,
                ARRAY_DESTRUCTOR_DOOR_PORTALS,
                ARRAY_CONSTRUCTOR_CLOSED_DOORS,
                ARRAY_DESTRUCTOR_CLOSED_DOORS,
                ARRAY_CONSTRUCTOR_U16,
                ARRAY_DESTRUCTOR_U16,
                MAP_CONSTRUCTOR,
                MAP_DESTRUCTOR,
                GRID_CONSTRUCTOR,
                GRID_CLEAR,
                OBSTACLES_CONSTRUCTOR,
                OBSTACLES_DESTRUCTOR,
                AVOID_NODES_CONSTRUCTOR,
                AVOID_NODES_DESTRUCTOR,
                SET_FORM_TYPE,
                ARRAY_CLEAR,
                INFO_SET_NAV_MESH,
                FREE_BLOCK,
            ],
        );
    }

    #[test]
    fn constructor_builds_the_bases_and_members() {
        let mut e = Engine::new();
        member_doubles(&mut e);
        e.register(OPERATOR_NEW, |e, a| result(e.mem.alloc(a[0])));
        e.register(OBSTACLE_MAP_CONSTRUCTOR, |_, a| result(a[0]));
        let mesh: Ptr<NavMesh> = e.new_object();
        e.set(mesh, NavMesh::pParentCell, Ptr::new(0x77));
        e.set(mesh, NavMesh::pNavMeshInfo, Ptr::new(0x88));
        e.call_log = Some(vec![]);
        let back = e.call(0x0068_eb80, &args![mesh]).ptr::<NavMesh>();
        assert_eq!(back, mesh);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(mesh.addr()), NAV_MESH_VTABLE);
        assert_eq!(e.mem.u32(mesh.addr() + 0x18), CHILD_CELL_VTABLE);
        assert_eq!(e.mem.u32(mesh.addr() + 0x1c), REF_OBJECT_VTABLE);
        assert!(e.get(mesh, NavMesh::pParentCell).is_null());
        assert!(e.get(mesh, NavMesh::pNavMeshInfo).is_null());
        // The form type 0x43 is stored; the obstacle map is the 16-byte block.
        assert_eq!(calls_to(&log, SET_FORM_TYPE), vec![vec![mesh.addr(), 0x43]]);
        let map = e.get(mesh, NavMesh::pTriangleToObstacleMap);
        assert!(!map.is_null());
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x10]]);
        assert_eq!(
            calls_to(&log, OBSTACLE_MAP_CONSTRUCTOR),
            vec![vec![map.addr(), 0x25]]
        );
        // Bases first (form, child cell, ref object), then the vertex array.
        let order: Vec<u32> = log.iter().map(|(a, _)| *a).collect();
        assert_eq!(
            &order[..5],
            &[
                0x0068_eb80,
                TES_FORM_CONSTRUCTOR,
                CHILD_CELL_CONSTRUCTOR,
                REF_OBJECT_CONSTRUCTOR,
                ARRAY_CONSTRUCTOR_VERTICES
            ]
        );
        assert_eq!(
            calls_to(&log, MAP_CONSTRUCTOR),
            vec![vec![mesh.addr() + 0x88, 0x25]]
        );
    }

    #[test]
    fn constructor_leaves_the_map_null_when_allocation_fails() {
        let mut e = Engine::new();
        member_doubles(&mut e);
        e.register(OPERATOR_NEW, |_, _| result(0));
        let mesh: Ptr<NavMesh> = e.new_object();
        e.set(mesh, NavMesh::pTriangleToObstacleMap, Ptr::new(0x99));
        e.call_log = Some(vec![]);
        e.call(0x0068_eb80, &args![mesh]);
        let log = e.call_log.take().unwrap();
        assert!(e.get(mesh, NavMesh::pTriangleToObstacleMap).is_null());
        assert!(calls_to(&log, OBSTACLE_MAP_CONSTRUCTOR).is_empty());
    }

    #[test]
    fn scalar_deleting_destructor_frees_only_when_asked() {
        let mut e = Engine::new();
        member_doubles(&mut e);
        let mesh: Ptr<NavMesh> = e.new_object();
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0068_ed30, &args![mesh, 1u32]).ptr::<NavMesh>(),
            mesh
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, FREE_BLOCK), vec![vec![mesh.addr(), 0x108]]);
        assert_eq!(calls_to(&log, TES_FORM_DESTRUCTOR).len(), 1);
        e.call_log = Some(vec![]);
        e.call(0x0068_ed30, &args![mesh, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, FREE_BLOCK).is_empty());
        assert_eq!(calls_to(&log, TES_FORM_DESTRUCTOR).len(), 1);
    }

    #[test]
    fn destructor_detaches_from_the_info_and_deletes_the_map() {
        let mut e = Engine::new();
        member_doubles(&mut e);
        let mesh: Ptr<NavMesh> = e.new_object();
        let info = Ptr::<()>::new(e.mem.alloc(0x60));
        e.set(mesh, NavMesh::pNavMeshInfo, info);
        // A map object whose virtual destructor (slot 0) is a double.
        e.put_vtable(0x0e10_0000, &[0x00f0_0100]);
        e.register(0x00f0_0100, |_, _| Ret::default());
        let map = e.mem.alloc(16);
        e.mem.set_u32(map, 0x0e10_0000);
        e.set(mesh, NavMesh::pTriangleToObstacleMap, Ptr::new(map));
        e.call_log = Some(vec![]);
        e.call(0x0068_ed70, &args![mesh]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(mesh.addr()), NAV_MESH_VTABLE);
        assert_eq!(
            calls_to(&log, INFO_SET_NAV_MESH),
            vec![vec![info.addr(), 0]]
        );
        assert_eq!(calls_to(&log, 0x00f0_0100), vec![vec![map, 1]]);
        // The five arrays emptied first, members destroyed last to first,
        // the form last.
        assert_eq!(calls_to(&log, ARRAY_CLEAR).len(), 5);
        let order: Vec<u32> = log.iter().map(|(a, _)| *a).collect();
        let position = |address: u32| order.iter().position(|a| *a == address).unwrap();
        assert!(position(AVOID_NODES_DESTRUCTOR) < position(OBSTACLES_DESTRUCTOR));
        assert!(position(ARRAY_DESTRUCTOR_TRIANGLES) < position(ARRAY_DESTRUCTOR_VERTICES));
        assert_eq!(*order.last().unwrap(), TES_FORM_DESTRUCTOR);
    }

    #[test]
    fn destructor_without_info_or_map_skips_them() {
        let mut e = Engine::new();
        member_doubles(&mut e);
        let mesh: Ptr<NavMesh> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x0068_ed70, &args![mesh]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, INFO_SET_NAV_MESH).is_empty());
    }

    #[test]
    fn child_cell_thunk_passes_the_navmesh() {
        let mut e = Engine::new();
        e.register(GET_PARENT_CELL, |_, a| result(a[0] + 1));
        // `this` is the TESChildCell base at +0x18.
        assert_eq!(e.call(0x0068_ef50, &args![0x1018u32]).u32(), 0x1001);
    }

    #[test]
    fn parent_space_prefers_the_worldspace() {
        let mut e = Engine::new();
        e.register(CELL_GET_WORLD_SPACE, |e, a| result(e.mem.u32(a[0] + 0x40)));
        let mesh: Ptr<NavMesh> = e.new_object();
        assert!(e.call(0x0068_ef70, &args![mesh]).ptr::<()>().is_null());
        let cell = e.mem.alloc(0x50);
        e.set(mesh, NavMesh::pParentCell, Ptr::new(cell));
        assert_eq!(e.call(0x0068_ef70, &args![mesh]).u32(), cell);
        e.mem.set_u32(cell + 0x40, 0xabc0);
        assert_eq!(e.call(0x0068_ef70, &args![mesh]).u32(), 0xabc0);
    }

    #[test]
    fn info_getter_returns_the_navmesh_info() {
        let mut e = Engine::new();
        let mesh: Ptr<NavMesh> = e.new_object();
        e.set(mesh, NavMesh::pNavMeshInfo, Ptr::new(0x1234));
        assert_eq!(e.call(0x0068_efb0, &args![mesh]).u32(), 0x1234);
    }

    #[test]
    fn edge_vertex_indexes_wrap_around_the_triangle() {
        let mut e = engine();
        let mesh = mesh_with(&mut e, &[], &[([10, 20, 30], [0xffff; 3], 0)]);
        let out = Ptr::<()>::new(e.mem.alloc(8));
        assert_eq!(
            e.call(0x0068_efd0, &args![mesh, out, 0u16, 2i32])
                .ptr::<()>(),
            out
        );
        // Edge 2 runs from vertex 30 back to vertex 10.
        assert_eq!(e.mem.u16(out.addr()), 30);
        assert_eq!(e.mem.u16(out.addr() + 2), 10);
        e.call(0x0068_efd0, &args![mesh, out, 0u16, 0i32]);
        assert_eq!((e.mem.u16(out.addr()), e.mem.u16(out.addr() + 2)), (10, 20));
    }

    #[test]
    fn edge_vertices_are_the_addresses_of_the_vertices() {
        let mut e = engine();
        let mesh = two_triangles(&mut e);
        let out = Ptr::<()>::new(e.mem.alloc(8));
        let back = e
            .call(0x0068_f040, &args![mesh, out, 1u16, 1i32])
            .ptr::<()>();
        assert_eq!(back, out);
        let base = e.mem.u32(mesh.addr() + 0x28 + 4);
        // Triangle 1 is (1, 3, 2): edge 1 runs from vertex 3 to vertex 2.
        assert_eq!(e.mem.u32(out.addr()), base + 3 * 12);
        assert_eq!(e.mem.u32(out.addr() + 4), base + 2 * 12);
    }

    #[test]
    fn vertex_getter_indexes_the_vertex_array() {
        let mut e = engine();
        let mesh = two_triangles(&mut e);
        let base = e.mem.u32(mesh.addr() + 0x28 + 4);
        assert_eq!(e.call(0x0068_f0a0, &args![mesh, 2u16]).u32(), base + 24);
    }

    #[test]
    fn edge_coordinates_are_copied() {
        let mut e = engine();
        let mesh = two_triangles(&mut e);
        let first = Ptr::<()>::new(e.mem.alloc(12));
        let second = Ptr::<()>::new(e.mem.alloc(12));
        // Triangle 0, edge 1: vertex 1 (1, 0, 0) to vertex 2 (0, 1, 0).
        assert!(e
            .call(0x0068_f0c0, &args![mesh, 0u16, 1i32, first, second])
            .bool());
        assert_eq!(
            [e.mem.f32(first.addr()), e.mem.f32(first.addr() + 4)],
            [1.0, 0.0]
        );
        assert_eq!(
            [e.mem.f32(second.addr()), e.mem.f32(second.addr() + 4)],
            [0.0, 1.0]
        );
    }

    #[test]
    fn triangle_vertex_addresses_are_stored() {
        let mut e = engine();
        let mesh = two_triangles(&mut e);
        let outs: Vec<Ptr> = (0..3).map(|_| Ptr::new(e.mem.alloc(4))).collect();
        e.call(0x0068_f160, &args![mesh, 1u16, outs[0], outs[1], outs[2]]);
        let base = e.mem.u32(mesh.addr() + 0x28 + 4);
        let stored: Vec<u32> = outs.iter().map(|p| e.mem.u32(p.addr())).collect();
        assert_eq!(stored, vec![base + 12, base + 36, base + 24]);
    }

    #[test]
    fn triangle_flag_tests() {
        let mut e = Engine::new();
        let tri: Ptr<NavMeshTriangle> = e.new_object();
        e.set(tri, NavMeshTriangle::TriangleFlags, 0b101);
        assert!(e.call(0x0068_f1d0, &args![tri, 0u32]).bool());
        assert!(!e.call(0x0068_f1d0, &args![tri, 1u32]).bool());
        assert!(e.call(0x0068_f1d0, &args![tri, 2u32]).bool());
        // Only the low five bits of the edge number count.
        assert!(e.call(0x0068_f1d0, &args![tri, 32u32]).bool());
    }

    #[test]
    fn neighbour_tests_and_getter() {
        let mut e = Engine::new();
        let tri: Ptr<NavMeshTriangle> = e.new_object();
        e.mem.write(tri.addr() + 6, &[7, 0, 0xff, 0xff, 9, 0]);
        assert!(e.call(0x0068_f200, &args![tri, 0i32]).bool());
        assert!(!e.call(0x0068_f200, &args![tri, 1i32]).bool());
        assert_eq!(e.call(0x0068_f2c0, &args![tri, 2i32]).u16(), 9);
        assert_eq!(e.call(0x0068_f2c0, &args![tri, 1i32]).u16(), 0xffff);
    }

    #[test]
    fn edge_extra_info_needs_the_flag_and_a_neighbour_slot() {
        let mut e = engine();
        let mesh = mesh_with(
            &mut e,
            &[],
            &[
                ([0, 1, 2], [1, 0xffff, 0xffff], 0b001),
                ([0, 1, 2], [0xffff, 0xffff, 0xffff], 0b010),
                ([0, 1, 2], [0xffff, 0xffff, 0], 0),
            ],
        );
        put_array(&mut e, mesh.at(NavMesh::ExtraEdgeInfo), &[0u8; 36], 3);
        let infos = e.mem.u32(mesh.addr() + 0x48 + 4);
        // Flag set, slot 1: the second info record.
        assert_eq!(
            e.call(0x0068_f230, &args![mesh, 0u16, 0i32]).u32(),
            infos + 12
        );
        // Flag set but slot 0xFFFF.
        assert_eq!(e.call(0x0068_f230, &args![mesh, 1u16, 1i32]).u32(), 0);
        // Flag clear.
        assert_eq!(e.call(0x0068_f230, &args![mesh, 2u16, 2i32]).u32(), 0);
    }

    #[test]
    fn info_navmesh_getter_honours_the_flag() {
        let mut e = Engine::new();
        let info = Ptr::<()>::new(e.mem.alloc(0x60));
        let out = Ptr::<()>::new(e.mem.alloc(4));
        e.mem.set_u32(out.addr(), 0xdead);
        e.mem.set_u32(info.addr() + 0x54, 0x4321);
        assert!(e.call(0x0068_f2e0, &args![info, out]).bool());
        assert_eq!(e.mem.u32(out.addr()), 0x4321);
        // A null mesh: stored, but false.
        e.mem.set_u32(info.addr() + 0x54, 0);
        assert!(!e.call(0x0068_f2e0, &args![info, out]).bool());
        // Flagged: null and false even with a mesh.
        e.mem.set_u32(info.addr() + 0x54, 0x4321);
        e.mem.set_u32(info.addr() + 8, 0x10);
        assert!(!e.call(0x0068_f2e0, &args![info, out]).bool());
        assert_eq!(e.mem.u32(out.addr()), 0);
    }

    #[test]
    fn info_flag_bit_0x10() {
        let mut e = Engine::new();
        let info = Ptr::<()>::new(e.mem.alloc(0x60));
        assert!(!e.call(0x0068_f320, &args![info]).bool());
        e.mem.set_u32(info.addr() + 8, 0x10);
        assert!(e.call(0x0068_f320, &args![info]).bool());
        e.mem.set_u32(info.addr() + 8, 0xffff_ffef);
        assert!(!e.call(0x0068_f320, &args![info]).bool());
    }

    fn matching_edge_engine() -> (Engine, Ptr<NavMesh>) {
        let mut e = engine();
        let mesh = two_triangles(&mut e);
        let info = e.mem.alloc(0x60);
        e.set(mesh, NavMesh::pNavMeshInfo, Ptr::new(info));
        (e, mesh)
    }

    #[test]
    fn matching_edge_inside_one_mesh() {
        let (mut e, mesh) = matching_edge_engine();
        let mesh_out = e.mem.alloc(4);
        let triangle_out = e.mem.alloc(4);
        let edge_out = e.mem.alloc(4);
        // Triangle 0 edge 1 mirrors triangle 1 edge 2.
        assert!(e
            .call(
                0x0068_f460,
                &args![mesh, 0u16, 1i32, mesh_out, triangle_out, edge_out]
            )
            .bool());
        assert_eq!(e.mem.u32(mesh_out), mesh.addr());
        assert_eq!(e.mem.u16(triangle_out), 1);
        assert_eq!(e.mem.i32(edge_out), 2);
        // And back.
        assert!(e
            .call(
                0x0068_f460,
                &args![mesh, 1u16, 2i32, mesh_out, triangle_out, edge_out]
            )
            .bool());
        assert_eq!((e.mem.u16(triangle_out), e.mem.i32(edge_out)), (0, 1));
    }

    #[test]
    fn matching_edge_fails_without_a_neighbour_or_return_link() {
        let (mut e, mesh) = matching_edge_engine();
        let outs: Vec<u32> = (0..3).map(|_| e.mem.alloc(4)).collect();
        let call = |e: &mut Engine, triangle: u16, edge: i32| {
            e.call(
                0x0068_f460,
                &args![mesh, triangle, edge, outs[0], outs[1], outs[2]],
            )
            .bool()
        };
        // Triangle index past the end, an edge without a neighbour.
        assert!(!call(&mut e, 2, 0));
        assert!(!call(&mut e, 0, 0));
        // The neighbour does not link back.
        let neighbour = triangle_ptr(&e, mesh, 1);
        e.mem.set_u16(neighbour.addr() + 6 + 4, 0xffff);
        assert!(!call(&mut e, 0, 1));
    }

    type PortalPair = (Ptr<NavMesh>, Ptr<NavMesh>, Ptr, Ptr);

    /// Mesh A (triangle 0, edge 0) has a portal to mesh B (triangle 0,
    /// edge 1) which has one back.
    fn portal_pair(e: &mut Engine) -> PortalPair {
        let a = mesh_with(e, &[], &[([0, 1, 2], [0, 0xffff, 0xffff], 0b001)]);
        let b = mesh_with(e, &[], &[([0, 1, 2], [0xffff, 0, 0xffff], 0b010)]);
        let info_a = Ptr::<()>::new(e.mem.alloc(0x60));
        let info_b = Ptr::<()>::new(e.mem.alloc(0x60));
        e.set(a, NavMesh::pNavMeshInfo, info_a);
        e.set(b, NavMesh::pNavMeshInfo, info_b);
        e.mem.set_u32(info_a.addr() + 0x54, a.addr());
        e.mem.set_u32(info_b.addr() + 0x54, b.addr());
        // A's info record: portal to B's info, triangle 0. B's back to A.
        let mut record_a = vec![0u8; 12];
        record_a[4..8].copy_from_slice(&info_b.addr().to_le_bytes());
        put_array(e, a.at(NavMesh::ExtraEdgeInfo), &record_a, 1);
        let mut record_b = vec![0u8; 12];
        record_b[4..8].copy_from_slice(&info_a.addr().to_le_bytes());
        put_array(e, b.at(NavMesh::ExtraEdgeInfo), &record_b, 1);
        (a, b, info_a, info_b)
    }

    #[test]
    fn matching_edge_through_a_portal() {
        let mut e = engine();
        let (a, b, _, _) = portal_pair(&mut e);
        let outs: Vec<u32> = (0..3).map(|_| e.mem.alloc(4)).collect();
        assert!(e
            .call(
                0x0068_f460,
                &args![a, 0u16, 0i32, outs[0], outs[1], outs[2]]
            )
            .bool());
        assert_eq!(e.mem.u32(outs[0]), b.addr());
        assert_eq!(e.mem.u16(outs[1]), 0);
        assert_eq!(e.mem.i32(outs[2]), 1);
    }

    #[test]
    fn matching_edge_through_a_portal_needs_a_back_portal() {
        let mut e = engine();
        let (a, b, _, info_b) = portal_pair(&mut e);
        let outs: Vec<u32> = (0..3).map(|_| e.mem.alloc(4)).collect();
        // B's portal points to the wrong mesh.
        let record = e.mem.u32(b.addr() + 0x48 + 4);
        e.mem.set_u32(record + 4, info_b.addr());
        assert!(!e
            .call(
                0x0068_f460,
                &args![a, 0u16, 0i32, outs[0], outs[1], outs[2]]
            )
            .bool());
        // A flagged (unloaded) other info ends the search early.
        let record = e.mem.u32(b.addr() + 0x48 + 4);
        let info_a = e.get(a, NavMesh::pNavMeshInfo);
        e.mem.set_u32(record + 4, info_a.addr());
        e.mem.set_u32(info_b.addr() + 8, 0x10);
        assert!(!e
            .call(
                0x0068_f460,
                &args![a, 0u16, 0i32, outs[0], outs[1], outs[2]]
            )
            .bool());
    }

    #[test]
    fn matching_edge_with_a_null_portal_info_fails() {
        let mut e = engine();
        let (a, _, _, _) = portal_pair(&mut e);
        let record = e.mem.u32(a.addr() + 0x48 + 4);
        e.mem.set_u32(record + 4, 0);
        let outs: Vec<u32> = (0..3).map(|_| e.mem.alloc(4)).collect();
        assert!(!e
            .call(
                0x0068_f460,
                &args![a, 0u16, 0i32, outs[0], outs[1], outs[2]]
            )
            .bool());
    }

    #[test]
    fn matching_edge_record_form_stores_the_info() {
        let (mut e, mesh) = matching_edge_engine();
        let edge_ref = e.mem.alloc(12);
        e.mem.set_u16(edge_ref + 4, 0);
        e.mem.set_i32(edge_ref + 8, 1);
        let result_record = e.mem.alloc(12);
        assert!(e
            .call(0x0068_f340, &args![mesh, edge_ref, result_record])
            .bool());
        let info = e.get(mesh, NavMesh::pNavMeshInfo);
        assert_eq!(e.mem.u32(result_record), info.addr());
        assert_eq!(e.mem.u16(result_record + 4), 1);
        assert_eq!(e.mem.i32(result_record + 8), 2);
        // No match: the record is left alone.
        e.mem.set_i32(edge_ref + 8, 0);
        e.mem.set_u32(result_record, 0x55);
        assert!(!e
            .call(0x0068_f340, &args![mesh, edge_ref, result_record])
            .bool());
        assert_eq!(e.mem.u32(result_record), 0x55);
    }

    #[test]
    fn matching_edge_overload_2_hands_the_mesh_to_a_smart_pointer() {
        let (mut e, mesh) = matching_edge_engine();
        e.register(POINTER_CONSTRUCT_FROM_CDECL, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            result(a[0])
        });
        e.register(POINTER_ASSIGN, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            result(a[0])
        });
        e.register(POINTER_DESTRUCT, |_, _| Ret::default());
        let pointer = e.mem.alloc(4);
        let triangle_out = e.mem.alloc(4);
        let edge_out = e.mem.alloc(4);
        assert!(e
            .call(
                0x0068_f3b0,
                &args![mesh, 0u16, 1i32, pointer, triangle_out, edge_out]
            )
            .bool());
        assert_eq!(e.mem.u32(pointer), mesh.addr());
        assert_eq!((e.mem.u16(triangle_out), e.mem.i32(edge_out)), (1, 2));
        // No match: the pointer is untouched.
        e.mem.set_u32(pointer, 0);
        assert!(!e
            .call(
                0x0068_f3b0,
                &args![mesh, 0u16, 0i32, pointer, triangle_out, edge_out]
            )
            .bool());
        assert_eq!(e.mem.u32(pointer), 0);
    }

    #[test]
    fn matching_edge_overload_4_writes_three_words() {
        let (mut e, mesh) = matching_edge_engine();
        let edge_ref = e.mem.alloc(12);
        e.mem.set_u16(edge_ref + 4, 1);
        e.mem.set_i32(edge_ref + 8, 2);
        let out = e.mem.alloc(12);
        assert!(e.call(0x0068_f670, &args![mesh, edge_ref, out]).bool());
        assert_eq!(e.mem.u32(out), mesh.addr());
        assert_eq!((e.mem.u16(out + 4), e.mem.i32(out + 8)), (0, 1));
    }

    #[test]
    fn door_portal_getter_forwards_the_array() {
        let mut e = Engine::new();
        e.register(DOOR_PORTAL_ELEMENT, |_, a| result(a[0] * 1000 + a[1]));
        let mesh: Ptr<NavMesh> = Ptr::new(0x2000);
        assert_eq!(
            e.call(0x0068_f6b0, &args![mesh, 3u16]).u32(),
            (0x2000 + 0x58) * 1000 + 3
        );
    }

    // ---- CheckNavMesh ------------------------------------------------------

    /// Characters the cell text double stores, and the navmesh's form id.
    const CELL_TEXT: u32 = 0xcafe_0001;
    const FORM_ID: u32 = 0x0001_2345;
    const WORLD_SPACE: u32 = 0x0e20_0000;

    /// Doubles for everything `CheckNavMesh` and the functions it runs call.
    fn check_engine() -> Engine {
        let mut e = engine();
        e.register(CELL_IS_INTERIOR, |_, _| result(1));
        noops(
            &mut e,
            &[
                STRING_CONSTRUCT,
                STRING_DESTRUCT,
                DEBUG_PRINT,
                FLIP_TRIANGLE,
                POINTER_CONSTRUCT,
                POINTER_DESTRUCT,
            ],
        );
        e.register(GET_PARENT_CELL, |e, a| result(e.mem.u32(a[0] + 0x24)));
        e.register(POINTER_GET, |e, a| result(e.mem.u32(a[0])));
        e.register(GET_FORM_ID, |e, a| result(e.mem.u32(a[0] + 0xc)));
        e.register(FINITE, |_, a| {
            let value = f64::from_bits(a[0] as u64 | (a[1] as u64) << 32);
            result(value.is_finite() as u32)
        });
        e.register(FLOAT_ABS, |_, a| Ret {
            st0: f32::from_bits(a[0]).abs() as f64,
            ..Ret::default()
        });
        e.register(U16_ELEMENT, |e, a| result(e.mem.u32(a[0] + 4) + a[1] * 2));
        e.register(CELL_GET_WORLD_SPACE, |_, _| result(WORLD_SPACE));
        e.register(RT_DYNAMIC_CAST, |_, a| result(a[0]));
        e.register(POINTER_CONSTRUCT_FROM, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(POINTER_ASSIGN, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            Ret::default()
        });
        e.register(ARRAY_REMOVE_12, |e, a| {
            let size = e.mem.u32(a[0] + 8);
            e.mem.set_u32(a[0] + 8, size - a[2]);
            Ret::default()
        });
        e.map(0x0106_a000, 0x1000);
        e.map(0x0101_2000, 0x1000);
        e.set_global(MAX_COORDINATE, 1.00000000376832e14f64);
        e.set_global(ZERO, 0.0f64);
        // A cell whose detailed-string virtual stores the cell text.
        e.put_vtable(0x0e30_0000, &[0x00f0_0200; 37]);
        e.register(0x00f0_0200, |e, a| {
            e.mem.set_u32(a[1], CELL_TEXT);
            Ret::default()
        });
        e
    }

    /// Gives the mesh its cell and form id, and makes the normals point up
    /// unless `down_triangle` names one.
    fn checked(e: &mut Engine, mesh: Ptr<NavMesh>, down_triangle: Option<u32>) {
        let cell = e.mem.alloc(0x80);
        e.mem.set_u32(cell, 0x0e30_0000);
        e.set(mesh, NavMesh::pParentCell, Ptr::new(cell));
        e.mem.set_u32(mesh.addr() + 0xc, FORM_ID);
        e.register_double(TRIANGLE_NORMAL, move |e, a| {
            let z = if Some(a[2]) == down_triangle {
                -1.0
            } else {
                1.0
            };
            e.mem.set_f32(a[1] + 8, z);
            Ret::default()
        });
    }

    /// Runs the check and returns its result and the debug prints (format,
    /// then the numbers after the form id and cell text).
    fn run_check(e: &mut Engine, mesh: Ptr<NavMesh>) -> (bool, Vec<Vec<u32>>) {
        e.call_log = Some(vec![]);
        let ok = e.call(0x0068_f6d0, &args![mesh, 0u32]).bool();
        let log = e.call_log.take().unwrap();
        let prints = calls_to(&log, DEBUG_PRINT)
            .into_iter()
            .map(|words| {
                assert_eq!(words[1], FORM_ID, "form id");
                assert_eq!(words[2], CELL_TEXT, "cell text");
                let mut report = vec![words[0]];
                report.extend_from_slice(&words[3..]);
                report
            })
            .collect();
        (ok, prints)
    }

    #[test]
    fn check_accepts_a_consistent_mesh() {
        let mut e = check_engine();
        let mesh = two_triangles(&mut e);
        checked(&mut e, mesh, None);
        let (ok, prints) = run_check(&mut e, mesh);
        assert!(ok);
        assert!(prints.is_empty(), "{prints:?}");
    }

    #[test]
    fn check_reports_the_first_bad_vertex_coordinate() {
        let mut e = check_engine();
        let mesh = mesh_with(
            &mut e,
            &[[0.0, 0.0, 0.0], [f32::NAN, 0.0, 0.0], [0.0, 3.0e14, 0.0]],
            &[],
        );
        checked(&mut e, mesh, None);
        let (ok, prints) = run_check(&mut e, mesh);
        assert!(!ok);
        // The NaN stops the vertex scan; the huge y coordinate is not seen.
        assert_eq!(prints, vec![vec![FORMAT_BAD_X]]);
        let mesh = mesh_with(&mut e, &[[0.0, 3.0e14, 0.0]], &[]);
        checked(&mut e, mesh, None);
        assert_eq!(run_check(&mut e, mesh).1, vec![vec![FORMAT_BAD_Y]]);
        let mesh = mesh_with(&mut e, &[[0.0, 0.0, -3.0e14]], &[]);
        checked(&mut e, mesh, None);
        assert_eq!(run_check(&mut e, mesh).1, vec![vec![FORMAT_BAD_Z]]);
    }

    #[test]
    fn check_warns_about_many_triangles() {
        let mut e = check_engine();
        let mesh = mesh_with(&mut e, &[], &[]);
        // The mesh reports 2000 triangles on the second and third size query
        // (the check and the warning); every other query sees an empty array,
        // so the O(n^2) loops stay short. A warning is not a problem.
        let mut queries = 0;
        e.register_double(ARRAY_SIZE, move |_, _| {
            queries += 1;
            result(if queries == 2 || queries == 3 {
                2000
            } else {
                0
            })
        });
        checked(&mut e, mesh, None);
        let (ok, prints) = run_check(&mut e, mesh);
        assert!(ok);
        assert_eq!(prints, vec![vec![FORMAT_MANY_TRIANGLES, 2000]]);
    }

    #[test]
    fn check_clears_a_neighbour_index_past_the_end() {
        let mut e = check_engine();
        let mesh = mesh_with(
            &mut e,
            &[[0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
            &[([0, 1, 2], [0xffff, 7, 0xffff], 0)],
        );
        checked(&mut e, mesh, None);
        let (ok, prints) = run_check(&mut e, mesh);
        assert!(!ok);
        assert_eq!(prints, vec![vec![FORMAT_BAD_TRIANGLE_INDEX, 0, 1]]);
        let tri = triangle_ptr(&e, mesh, 0);
        assert_eq!(e.mem.u16(tri.addr() + 8), 0xffff);
    }

    #[test]
    fn check_flips_a_downfacing_triangle() {
        let mut e = check_engine();
        let mesh = two_triangles(&mut e);
        checked(&mut e, mesh, Some(1));
        e.call_log = Some(vec![]);
        let ok = e.call(0x0068_f6d0, &args![mesh, 0u32]).bool();
        let log = e.call_log.take().unwrap();
        assert!(!ok);
        assert_eq!(
            calls_to(&log, DEBUG_PRINT)
                .iter()
                .map(|w| (w[0], w[3]))
                .collect::<Vec<_>>(),
            vec![(FORMAT_DOWNFACING, 1)]
        );
        assert_eq!(calls_to(&log, FLIP_TRIANGLE), vec![vec![mesh.addr(), 1]]);
    }

    #[test]
    fn check_reports_degenerate_triangles() {
        let mut e = check_engine();
        let mesh = mesh_with(
            &mut e,
            &[[0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
            &[
                ([0, 0, 2], [0xffff; 3], 0),
                ([0, 1, 1], [0xffff; 3], 0),
                ([2, 1, 2], [0xffff; 3], 0),
            ],
        );
        checked(&mut e, mesh, None);
        let (ok, prints) = run_check(&mut e, mesh);
        assert!(!ok);
        assert!(prints.contains(&vec![FORMAT_DEGENERATE_01, 0, 0]));
        assert!(prints.contains(&vec![FORMAT_DEGENERATE_12, 1, 1]));
        assert!(prints.contains(&vec![FORMAT_DEGENERATE_02, 2, 2]));
    }

    #[test]
    fn check_reports_two_edges_to_the_same_neighbour() {
        let mut e = check_engine();
        let mesh = two_triangles(&mut e);
        checked(&mut e, mesh, None);
        // Triangle 0: edges 0 and 1 both lead to triangle 1.
        let tri = triangle_ptr(&e, mesh, 0);
        e.mem.set_u16(tri.addr() + 6, 1);
        let (ok, prints) = run_check(&mut e, mesh);
        assert!(!ok);
        assert!(prints.contains(&vec![FORMAT_SAME_NEIGHBOUR_01, 0, 1]));
    }

    #[test]
    fn check_reports_a_link_the_neighbour_does_not_return() {
        let mut e = check_engine();
        let mesh = two_triangles(&mut e);
        checked(&mut e, mesh, None);
        let neighbour = triangle_ptr(&e, mesh, 1);
        e.mem.set_u16(neighbour.addr() + 6 + 4, 0xffff);
        let (ok, prints) = run_check(&mut e, mesh);
        assert!(!ok);
        assert!(prints.contains(&vec![FORMAT_LINK_NOT_RETURNED, 0, 1, 1]));
        assert!(prints.contains(&vec![FORMAT_MISSING_LINK, 1, 2, 0]));
    }

    #[test]
    fn check_reports_linked_triangles_with_opposite_normals() {
        let mut e = check_engine();
        // Triangle 1 runs the shared edge in the same direction as triangle 0.
        let mesh = mesh_with(
            &mut e,
            &[[0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0, 0.0]],
            &[
                ([0, 1, 2], [0xffff, 1, 0xffff], 0),
                ([1, 2, 3], [0, 0xffff, 0xffff], 0),
            ],
        );
        checked(&mut e, mesh, None);
        let (ok, prints) = run_check(&mut e, mesh);
        assert!(!ok);
        assert!(prints.contains(&vec![FORMAT_OPPOSITE_NORMALS, 0, 1]));
    }

    #[test]
    fn check_reports_mismatched_shared_vertices() {
        let mut e = check_engine();
        let mesh = two_triangles(&mut e);
        checked(&mut e, mesh, None);
        // Triangle 1 keeps the link but moves its vertex 0 away from the edge.
        let neighbour = triangle_ptr(&e, mesh, 1);
        e.mem.set_u16(neighbour.addr(), 3);
        let (ok, prints) = run_check(&mut e, mesh);
        assert!(!ok);
        assert!(prints.contains(&vec![FORMAT_VERTICES_MISMATCH, 0, 1]));
    }

    #[test]
    fn check_reports_missing_portal_data_on_the_other_side() {
        let mut e = check_engine();
        let mesh = two_triangles(&mut e);
        checked(&mut e, mesh, None);
        // Triangle 0 claims extra info on the shared edge (valid record),
        // triangle 1 has none.
        let record = portal_record(&mut e, 0, WORLD_SPACE, None);
        put_array(&mut e, mesh.at(NavMesh::ExtraEdgeInfo), &record, 1);
        let tri = triangle_ptr(&e, mesh, 0);
        e.mem.set_u32(tri.addr() + 0xc, 0b010);
        e.mem.set_u16(tri.addr() + 8, 0);
        let (ok, prints) = run_check(&mut e, mesh);
        assert!(!ok);
        assert!(prints.contains(&vec![FORMAT_MISSING_PORTAL, 1, 2]));
    }

    #[test]
    fn check_reports_portal_data_the_other_side_lacks_a_match_for() {
        let mut e = check_engine();
        let mesh = two_triangles(&mut e);
        checked(&mut e, mesh, None);
        // Triangle 1 has extra info on its shared edge, triangle 0 does not.
        let record = portal_record(&mut e, 0, WORLD_SPACE, None);
        put_array(&mut e, mesh.at(NavMesh::ExtraEdgeInfo), &record, 1);
        let neighbour = triangle_ptr(&e, mesh, 1);
        e.mem.set_u32(neighbour.addr() + 0xc, 0b100);
        e.mem.set_u16(neighbour.addr() + 6 + 4, 0);
        let (ok, prints) = run_check(&mut e, mesh);
        assert!(!ok);
        assert!(prints.contains(&vec![FORMAT_PORTAL_WITHOUT_MATCH, 0, 1, 1, 1, 2]));
    }

    #[test]
    fn check_clears_a_bad_extra_info_index() {
        let mut e = check_engine();
        let mesh = mesh_with(
            &mut e,
            &[[0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
            &[([0, 1, 2], [4, 0xffff, 0xffff], 0b001)],
        );
        checked(&mut e, mesh, None);
        let (ok, prints) = run_check(&mut e, mesh);
        assert!(!ok);
        assert_eq!(prints, vec![vec![FORMAT_BAD_EXTRA_INFO_INDEX, 0, 0]]);
        let tri = triangle_ptr(&e, mesh, 0);
        assert_eq!(e.mem.u32(tri.addr() + 0xc), 0);
        assert_eq!(e.mem.u16(tri.addr() + 6), 0xffff);
    }

    /// An extra-info record with a portal to triangle `portal_triangle` of
    /// another mesh whose info lives in `world_space` (or to `other_info`).
    fn portal_record(
        e: &mut Engine,
        portal_triangle: u16,
        world_space: u32,
        other_info: Option<u32>,
    ) -> Vec<u8> {
        let other = mesh_with(e, &[], &[([0, 1, 2], [0xffff; 3], 0)]);
        e.mem.set_u32(other.addr() + 0xc, 0x0bad_0001);
        let info = e.mem.alloc(0x60);
        e.mem.set_u32(info, 0xbeef);
        e.mem.set_u32(info + 0x1c, world_space);
        e.mem.set_u32(info + 0x54, other.addr());
        let mut record = vec![0u8; 12];
        record[4..8].copy_from_slice(&other_info.unwrap_or(info).to_le_bytes());
        record[8..10].copy_from_slice(&portal_triangle.to_le_bytes());
        record
    }

    /// A mesh with one triangle whose edge 0 has extra info (record 0, see
    /// [`portal_record`]).
    fn portal_check(
        e: &mut Engine,
        portal_triangle: u16,
        world_space: u32,
        other_info: Option<u32>,
    ) -> Ptr<NavMesh> {
        let record = portal_record(e, portal_triangle, world_space, other_info);
        let mesh = mesh_with(
            e,
            &[[0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
            &[([0, 1, 2], [0, 0xffff, 0xffff], 0b001)],
        );
        put_array(e, mesh.at(NavMesh::ExtraEdgeInfo), &record, 1);
        checked(e, mesh, None);
        mesh
    }

    #[test]
    fn check_accepts_a_valid_portal() {
        let mut e = check_engine();
        let mesh = portal_check(&mut e, 0, WORLD_SPACE, None);
        let (ok, prints) = run_check(&mut e, mesh);
        assert!(ok, "{prints:?}");
        assert!(prints.is_empty());
    }

    #[test]
    fn check_deletes_a_portal_to_a_missing_triangle() {
        let mut e = check_engine();
        let mesh = portal_check(&mut e, 5, WORLD_SPACE, None);
        e.call_log = Some(vec![]);
        let ok = e.call(0x0068_f6d0, &args![mesh, 0u32]).bool();
        let log = e.call_log.take().unwrap();
        assert!(!ok);
        let prints = calls_to(&log, DEBUG_PRINT);
        assert_eq!(prints.len(), 1);
        // Triangle 0, edge 0, the other mesh's form id, the bad index 5.
        assert_eq!(prints[0][0], FORMAT_BAD_PORTAL_TRIANGLE);
        assert_eq!(&prints[0][3..], &[0, 0, 0x0bad_0001, 5]);
        assert_eq!(calls_to(&log, ARRAY_REMOVE_12).len(), 1);
    }

    #[test]
    fn check_deletes_a_portal_into_another_worldspace() {
        let mut e = check_engine();
        let mesh = portal_check(&mut e, 0, 0x0e20_1000, None);
        e.call_log = Some(vec![]);
        let ok = e.call(0x0068_f6d0, &args![mesh, 0u32]).bool();
        let log = e.call_log.take().unwrap();
        assert!(!ok);
        let prints = calls_to(&log, DEBUG_PRINT);
        assert_eq!(prints.len(), 1);
        assert_eq!(prints[0][0], FORMAT_PORTAL_OTHER_WORLD_SPACE);
        assert_eq!(&prints[0][3..], &[0, 0, 0xbeef]);
        assert_eq!(calls_to(&log, ARRAY_REMOVE_12).len(), 1);
    }

    #[test]
    fn check_deletes_a_portal_without_a_target_or_triangle() {
        let mut e = check_engine();
        // Portal info pointer is null.
        let mesh = portal_check(&mut e, 0, WORLD_SPACE, Some(0));
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0068_f6d0, &args![mesh, 0u32]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, ARRAY_REMOVE_12).len(), 1);
        assert!(calls_to(&log, DEBUG_PRINT).is_empty());
        // Portal triangle is 0xFFFF.
        let mesh = portal_check(&mut e, 0xffff, WORLD_SPACE, None);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0068_f6d0, &args![mesh, 0u32]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, ARRAY_REMOVE_12).len(), 1);
    }

    #[test]
    fn check_reports_cover_entries() {
        let mut e = check_engine();
        let mesh = two_triangles(&mut e);
        checked(&mut e, mesh, None);
        // Cover entry 0: triangle 0 without cover flags; entry 1: triangle 1
        // with a cover flag.
        put_array(&mut e, mesh.at(NavMesh::CoverArray), &[0, 0, 1, 0], 2);
        let covered = triangle_ptr(&e, mesh, 1);
        e.mem.set_u32(covered.addr() + 0xc, 0x0002_0000);
        let (ok, prints) = run_check(&mut e, mesh);
        assert!(!ok);
        assert_eq!(prints, vec![vec![FORMAT_COVER_WITHOUT_COVER, 0, 0]]);
    }

    #[test]
    fn check_reports_a_cover_entry_without_a_triangle() {
        let mut e = check_engine();
        let mesh = mesh_with(&mut e, &[], &[]);
        checked(&mut e, mesh, None);
        // No triangle buffer: triangle 0 resolves to null.
        e.mem.set_u32(mesh.addr() + 0x38 + 4, 0);
        put_array(&mut e, mesh.at(NavMesh::CoverArray), &[0, 0], 1);
        let (ok, prints) = run_check(&mut e, mesh);
        assert!(!ok);
        assert_eq!(prints, vec![vec![FORMAT_COVER_MISSING_TRIANGLE, 0, 0]]);
    }

    // ---- triangle and info methods -----------------------------------------

    #[test]
    fn cover_flags_mask() {
        let mut e = Engine::new();
        let tri: Ptr<NavMeshTriangle> = e.new_object();
        assert!(!e.call(0x0069_0770, &args![tri]).bool());
        e.set(tri, NavMeshTriangle::TriangleFlags, 0x0002_0000);
        assert!(e.call(0x0069_0770, &args![tri]).bool());
        // Bits outside the mask (0x0FBE0000) do not count.
        e.set(tri, NavMeshTriangle::TriangleFlags, 0x0041_ffff);
        assert!(!e.call(0x0069_0770, &args![tri]).bool());
    }

    #[test]
    fn clearing_edge_info_resets_flag_and_neighbour() {
        let mut e = Engine::new();
        let tri: Ptr<NavMeshTriangle> = e.new_object();
        e.set(tri, NavMeshTriangle::TriangleFlags, 0b111);
        e.mem.write(tri.addr() + 6, &[1, 0, 2, 0, 3, 0]);
        e.call(0x0069_0790, &args![tri, 1u32]);
        assert_eq!(e.get(tri, NavMeshTriangle::TriangleFlags), 0b101);
        assert_eq!(e.mem.u16(tri.addr() + 8), 0xffff);
        assert_eq!(e.mem.u16(tri.addr() + 6), 1);
        assert_eq!(e.mem.u16(tri.addr() + 10), 3);
    }

    #[test]
    fn setting_a_plain_neighbour_clears_the_flag_first() {
        let mut e = Engine::new();
        let tri: Ptr<NavMeshTriangle> = e.new_object();
        e.set(tri, NavMeshTriangle::TriangleFlags, 0b010);
        e.call(0x0069_07d0, &args![tri, 1u32, 9u16]);
        assert_eq!(e.get(tri, NavMeshTriangle::TriangleFlags), 0);
        assert_eq!(e.mem.u16(tri.addr() + 8), 9);
    }

    #[test]
    fn parent_space_is_cast_to_a_worldspace() {
        let mut e = Engine::new();
        e.register(LOOKUP_FORM, |_, a| result(a[0] + 0x100));
        e.register_double(RT_DYNAMIC_CAST, |_, a| result(a[0] + 1));
        let info = Ptr::<()>::new(e.mem.alloc(0x60));
        e.mem.set_u32(info.addr() + 0x1c, 0x5000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0069_0800, &args![info]).u32(), 0x5001);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, RT_DYNAMIC_CAST),
            vec![vec![0x5000, 0, TYPE_TES_FORM, TYPE_TES_WORLD_SPACE, 0]]
        );
    }

    #[test]
    fn parent_space_falls_back_to_the_form_lookup() {
        let mut e = Engine::new();
        e.register(LOOKUP_FORM, |_, a| result(a[0] + 0x100));
        let info = Ptr::<()>::new(e.mem.alloc(0x60));
        assert_eq!(e.call(0x0069_0830, &args![info]).u32(), 0x100);
        e.mem.set_u32(info.addr() + 4, 0x3c);
        assert_eq!(e.call(0x0069_0830, &args![info]).u32(), 0x13c);
        e.mem.set_u32(info.addr() + 0x1c, 0x7777);
        assert_eq!(e.call(0x0069_0830, &args![info]).u32(), 0x7777);
    }

    #[test]
    fn info_mesh_assignment_reports_a_loaded_mesh() {
        let mut e = Engine::new();
        e.register(POINTER_CONSTRUCT_FROM, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(POINTER_ASSIGN, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            Ret::default()
        });
        e.register(POINTER_DESTRUCT, |_, _| Ret::default());
        let info = Ptr::<()>::new(e.mem.alloc(0x60));
        let out = Ptr::<()>::new(e.mem.alloc(4));
        e.mem.set_u32(info.addr() + 0x54, 0x6000);
        assert!(e.call(0x0069_0860, &args![info, out]).bool());
        assert_eq!(e.mem.u32(out.addr()), 0x6000);
        e.mem.set_u32(info.addr() + 0x54, 0);
        assert!(!e.call(0x0069_0860, &args![info, out]).bool());
        assert_eq!(e.mem.u32(out.addr()), 0);
    }

    #[test]
    fn smart_pointer_assignment_uses_a_temporary() {
        let mut e = Engine::new();
        noops(
            &mut e,
            &[POINTER_CONSTRUCT_FROM, POINTER_ASSIGN, POINTER_DESTRUCT],
        );
        e.call_log = Some(vec![]);
        e.call(0x0069_0890, &args![0x100u32, 0x200u32]);
        let log = e.call_log.take().unwrap();
        let construct = calls_to(&log, POINTER_CONSTRUCT_FROM);
        let assign = calls_to(&log, POINTER_ASSIGN);
        let destruct = calls_to(&log, POINTER_DESTRUCT);
        let temp = construct[0][0];
        assert_eq!(construct[0][1], 0x200);
        assert_eq!(assign, vec![vec![0x100, temp]]);
        assert_eq!(destruct, vec![vec![temp]]);
    }

    /// Doubles for the neighbour-cell walk of `006908f0`.
    fn neighbour_engine() -> (Engine, Ptr<NavMesh>) {
        let mut e = engine();
        e.map(0x0101_7000, 0x1000);
        e.register(GET_PARENT_CELL, |e, a| result(e.mem.u32(a[0] + 0x24)));
        e.register(CELL_IS_INTERIOR, |e, a| {
            result((e.mem.u8(a[0] + 0x24) & 1) as u32)
        });
        e.register(CELL_GET_X, |e, a| result(e.mem.u32(a[0] + 0x48)));
        e.register(CELL_GET_Y, |e, a| result(e.mem.u32(a[0] + 0x4c)));
        e.register(CELL_GET_WORLD_SPACE, |_, _| result(0x7000));
        e.register(CELL_GET_NAV_MESHES, |e, a| result(e.mem.u32(a[0] + 0x64)));
        e.register(NAV_MESH_ARRAY_SIZE, |e, a| result(e.mem.u32(a[0] + 8)));
        e.register(NAV_MESH_ARRAY_GET, |e, a| {
            let element = e.mem.u32(e.mem.u32(a[0] + 4) + 4 * a[2]);
            e.mem.set_u32(a[1], element);
            result(a[1])
        });
        e.register(POINTER_DEREFERENCE, |e, a| result(e.mem.u32(a[0])));
        noops(&mut e, &[POINTER_DESTRUCT]);
        let mesh: Ptr<NavMesh> = e.new_object();
        let cell = e.mem.alloc(0x80);
        e.mem.set_u32(cell + 0x48, 10);
        e.mem.set_u32(cell + 0x4c, 20);
        e.set(mesh, NavMesh::pParentCell, Ptr::new(cell));
        (e, mesh)
    }

    /// A cell holding the given navmeshes as its navmesh array.
    fn cell_with_meshes(e: &mut Engine, meshes: &[u32]) -> u32 {
        let cell = e.mem.alloc(0x80);
        let array = e.mem.alloc(0x10);
        let buffer = e.mem.alloc(8 + 4 * meshes.len() as u32);
        for (i, mesh) in meshes.iter().enumerate() {
            e.mem.set_u32(buffer + 4 * i as u32, *mesh);
        }
        e.mem.set_u32(array + 4, buffer);
        e.mem.set_u32(array + 8, meshes.len() as u32);
        e.mem.set_u32(cell + 0x64, array);
        cell
    }

    #[test]
    fn interior_cells_pass_the_neighbour_check() {
        let (mut e, mesh) = neighbour_engine();
        let cell = e.get(mesh, NavMesh::pParentCell);
        e.mem.set_u8(cell.addr() + 0x24, 1);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0069_08f0, &args![mesh]).bool());
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, WORLD_SPACE_GET_CELL).is_empty());
    }

    #[test]
    fn exterior_cells_are_checked_against_their_neighbours() {
        let (mut e, mesh) = neighbour_engine();
        e.set_global(CELL_WIDTH, 4096.0f64);
        let east = cell_with_meshes(&mut e, &[0x111, 0x222]);
        let north = cell_with_meshes(&mut e, &[0x333]);
        // West of the cell there is no cell; south has a cell without an
        // array.
        let south = e.mem.alloc(0x80);
        e.register_double(WORLD_SPACE_GET_CELL, move |_, a| {
            result(match (a[1] as i32, a[2] as i32) {
                (11, 20) => east,
                (10, 21) => north,
                (10, 19) => south,
                _ => 0,
            })
        });
        e.register_double(NEIGHBOUR_CHECK, |_, _| result(1));
        e.call_log = Some(vec![]);
        assert!(e.call(0x0069_08f0, &args![mesh]).bool());
        let log = e.call_log.take().unwrap();
        let checks = calls_to(&log, NEIGHBOUR_CHECK);
        let far_x = (10.0f32 * 4096.0 + 4096.0).to_bits();
        let far_y = (20.0f32 * 4096.0 + 4096.0).to_bits();
        assert_eq!(
            checks,
            vec![
                vec![mesh.addr(), 0x111, 1, far_x, 0],
                vec![mesh.addr(), 0x222, 1, far_x, 0],
                vec![mesh.addr(), 0x333, 0, 0, far_y],
            ]
        );
        // The west neighbour is asked for first, as cell (x - 1, y).
        let lookups = calls_to(&log, WORLD_SPACE_GET_CELL);
        assert_eq!(lookups[0], vec![0x7000, 9u32, 20]);
    }

    #[test]
    fn a_rejecting_neighbour_check_fails_the_mesh_check() {
        let (mut e, mesh) = neighbour_engine();
        e.set_global(CELL_WIDTH, 4096.0f64);
        let west = cell_with_meshes(&mut e, &[0x444]);
        e.register_double(WORLD_SPACE_GET_CELL, move |_, a| {
            result(if (a[1] as i32, a[2] as i32) == (9, 20) {
                west
            } else {
                0
            })
        });
        e.register_double(NEIGHBOUR_CHECK, |_, _| result(0));
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0069_08f0, &args![mesh]).bool());
        let log = e.call_log.take().unwrap();
        let near_x = (10.0f32 * 4096.0).to_bits();
        assert_eq!(
            calls_to(&log, NEIGHBOUR_CHECK),
            vec![vec![mesh.addr(), 0x444, 1, near_x, 0]]
        );
    }

    #[test]
    fn clearing_the_mesh_empties_everything_and_deletes_the_info() {
        let mut e = Engine::new();
        noops(
            &mut e,
            &[
                ARRAY_CLEAR,
                GRID_CLEAR,
                OBSTACLES_CLEAR,
                DELETE_NAV_MESH_INFO,
            ],
        );
        e.register(GET_FORM_ID, |e, a| result(e.mem.u32(a[0] + 0xc)));
        e.register(GET_NAV_MESH_INFO_MAP, |_, a| result(a[0] + 1));
        e.map(0x011d_e000, 0x1000);
        e.set_global(TES_INSTANCE, 0x5000u32);
        let mesh: Ptr<NavMesh> = e.new_object();
        e.mem.set_u32(mesh.addr() + 0xc, 0xabcd);
        e.call_log = Some(vec![]);
        e.call(0x0069_0d50, &args![mesh, 0u32]);
        let log = e.call_log.take().unwrap();
        // No info yet: nothing is deleted.
        assert!(calls_to(&log, DELETE_NAV_MESH_INFO).is_empty());
        assert_eq!(calls_to(&log, ARRAY_CLEAR).len(), 5);
        assert_eq!(calls_to(&log, OBSTACLES_CLEAR).len(), 1);
        assert_eq!(calls_to(&log, GRID_CLEAR), vec![vec![mesh.addr() + 0xa8]]);
        e.set(mesh, NavMesh::pNavMeshInfo, Ptr::new(0x4444));
        e.call_log = Some(vec![]);
        e.call(0x0069_0d50, &args![mesh, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, DELETE_NAV_MESH_INFO),
            vec![vec![0x5001, 0xabcd]]
        );
    }

    #[test]
    fn triangle_vertex_setter() {
        let mut e = Engine::new();
        let tri: Ptr<NavMeshTriangle> = e.new_object();
        e.call(0x0069_0df0, &args![tri, 2u32, 0x1234u16]);
        assert_eq!(e.mem.u16(tri.addr() + 4), 0x1234);
        assert_eq!(e.mem.u16(tri.addr()), 0);
    }

    #[test]
    fn edge_info_presence_flags() {
        let mut e = Engine::new();
        let tri: Ptr<NavMeshTriangle> = e.new_object();
        assert!(!e.call(0x0069_0e10, &args![tri]).bool());
        assert!(!e.call(0x0069_0e70, &args![tri]).bool());
        e.set(tri, NavMeshTriangle::TriangleFlags, 0x4);
        assert!(e.call(0x0069_0e10, &args![tri]).bool());
        assert!(!e.call(0x0069_0e70, &args![tri]).bool());
        e.set(tri, NavMeshTriangle::TriangleFlags, 0x408);
        assert!(!e.call(0x0069_0e10, &args![tri]).bool());
        assert!(e.call(0x0069_0e70, &args![tri]).bool());
    }

    #[test]
    fn setting_edge_info_sets_flag_and_slot() {
        let mut e = Engine::new();
        let tri: Ptr<NavMeshTriangle> = e.new_object();
        e.set(tri, NavMeshTriangle::TriangleFlags, 0b001);
        e.call(0x0069_0e30, &args![tri, 2u32, 6u16]);
        assert_eq!(e.get(tri, NavMeshTriangle::TriangleFlags), 0b101);
        assert_eq!(e.mem.u16(tri.addr() + 10), 6);
    }

    #[test]
    fn deleting_extra_edge_info_repairs_the_triangles() {
        let mut e = engine();
        e.register(ARRAY_REMOVE_12, |e, a| {
            let size = e.mem.u32(a[0] + 8);
            e.mem.set_u32(a[0] + 8, size - a[2]);
            Ret::default()
        });
        // Three infos; info 0 is deleted and info 2 (the last) moves to 0.
        // Triangle 0: edge 0 uses info 0, edge 1 uses info 2, edge 2 uses a
        // plain neighbour. Triangle 1 has no extra info flags at all.
        let mesh = mesh_with(
            &mut e,
            &[],
            &[([0, 1, 2], [0, 2, 1], 0b011), ([0, 1, 2], [5, 5, 5], 0)],
        );
        put_array(&mut e, mesh.at(NavMesh::ExtraEdgeInfo), &[0u8; 36], 3);
        e.call_log = Some(vec![]);
        e.call(0x0069_0e90, &args![mesh, 0u16]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, ARRAY_REMOVE_12),
            vec![vec![mesh.addr() + 0x48, 0, 1]]
        );
        let tri = triangle_ptr(&e, mesh, 0);
        // Edge 0 lost its flag and neighbour; edge 1 now points at info 0.
        assert_eq!(e.mem.u32(tri.addr() + 0xc), 0b010);
        assert_eq!(e.mem.u16(tri.addr() + 6), 0xffff);
        assert_eq!(e.mem.u16(tri.addr() + 8), 0);
        assert_eq!(e.mem.u16(tri.addr() + 10), 1);
        let other = triangle_ptr(&e, mesh, 1);
        assert_eq!(e.mem.u16(other.addr() + 6), 5);
    }

    #[test]
    fn deleting_extra_edge_info_leaves_other_infos_alone() {
        let mut e = engine();
        e.register(ARRAY_REMOVE_12, |e, a| {
            let size = e.mem.u32(a[0] + 8);
            e.mem.set_u32(a[0] + 8, size - a[2]);
            Ret::default()
        });
        let mesh = mesh_with(&mut e, &[], &[([0, 1, 2], [1, 0xffff, 0xffff], 0b001)]);
        put_array(&mut e, mesh.at(NavMesh::ExtraEdgeInfo), &[0u8; 36], 3);
        e.call(0x0069_0e90, &args![mesh, 0u16]);
        let tri = triangle_ptr(&e, mesh, 0);
        assert_eq!(e.mem.u32(tri.addr() + 0xc), 0b001);
        assert_eq!(e.mem.u16(tri.addr() + 6), 1);
    }

    #[test]
    fn appending_vertices_and_triangles_forwards_to_the_arrays() {
        let mut e = Engine::new();
        e.register(ARRAY_ADD_12, |_, a| result(a[0] * 10 + a[1]));
        e.register(ARRAY_ADD_16, |_, a| result(a[0] * 100 + a[1]));
        let mesh: Ptr<NavMesh> = Ptr::new(0x1000);
        assert_eq!(
            e.call(0x0069_0fe0, &args![mesh, 7u32]).u32(),
            (0x1000 + 0x28) * 10 + 7
        );
        assert_eq!(
            e.call(0x0069_1000, &args![mesh, 9u32]).u32(),
            (0x1000 + 0x38) * 100 + 9
        );
    }

    // -----------------------------------------------------------------
    // Second batch: 00691020 to 00691bf0
    // -----------------------------------------------------------------

    /// Double for `BSSimpleArray::Clear` (keeps the buffer, size 0).
    fn clear_array(e: &mut Engine, a: &[u32]) -> Ret {
        e.mem.set_u32(a[0] + 8, 0);
        Ret::default()
    }

    /// Double for the word-array append: writes `*a[1]` after the last element
    /// (the buffer is allocated on first use, 64 bytes) and bumps the size.
    fn push_word(e: &mut Engine, a: &[u32]) -> Ret {
        let mut buffer = e.mem.u32(a[0] + 4);
        if buffer == 0 {
            buffer = e.mem.alloc(64);
            e.mem.set_u32(a[0] + 4, buffer);
        }
        let size = e.mem.u32(a[0] + 8);
        let value = e.mem.u32(a[1]);
        e.mem.set_u32(buffer + 4 * size, value);
        e.mem.set_u32(a[0] + 8, size + 1);
        result(size)
    }

    /// Double for "make room for one more element": allocates a 256-byte
    /// buffer on first use, bumps the size and returns the new index.
    fn grow_array(e: &mut Engine, a: &[u32]) -> Ret {
        if e.mem.u32(a[0] + 4) == 0 {
            let buffer = e.mem.alloc(256);
            e.mem.set_u32(a[0] + 4, buffer);
        }
        let size = e.mem.u32(a[0] + 8);
        e.mem.set_u32(a[0] + 8, size + 1);
        result(size)
    }

    /// An engine with the doubles the grid functions call (`floor`, the
    /// clamp, unsigned minimum and maximum, the first word of an object).
    fn grid_engine() -> Engine {
        let mut e = engine();
        e.register(FLOAT_FLOOR, |_, a| Ret {
            st0: (f32::from_bits(a[0]) as f64).floor(),
            ..Ret::default()
        });
        e.register(CLAMP_INT, |e, a| {
            let value = e.mem.u32(a[0]) as i32;
            e.mem
                .set_u32(a[0], value.clamp(a[1] as i32, a[2] as i32) as u32);
            Ret::default()
        });
        e.register(UNSIGNED_MAX, |_, a| result(a[0].max(a[1])));
        e.register(UNSIGNED_MIN, |_, a| result(a[0].min(a[1])));
        e.register(POINTER_GET, |e, a| result(e.mem.u32(a[0])));
        e.register(ARRAY_CLEAR, clear_array);
        e.register(ARRAY_PUSH_WORD, push_word);
        e
    }

    /// A grid at the mesh's grid offset: `size` cells per side, minimum
    /// corner (0, 0), sections `length` long.
    fn set_grid(e: &mut Engine, mesh: Ptr<NavMesh>, size: u32, length: f32) -> Ptr<NavMeshGrid> {
        let grid: Ptr<NavMeshGrid> = mesh.byte_add(MESH_GRID_OFFSET).cast();
        e.set(grid, NavMeshGrid::iGridSize, size);
        e.set(grid, NavMeshGrid::fColumnSectionLen, length);
        e.set(grid, NavMeshGrid::fRowSectionLen, length);
        grid
    }

    fn point(e: &mut Engine, x: f32, y: f32) -> Ptr {
        let p = Ptr::new(e.mem.alloc(12));
        e.mem.set_f32(p.addr(), x);
        e.mem.set_f32(p.addr() + 4, y);
        p
    }

    fn words(e: &Engine, array: Ptr<BSSimpleArray>) -> Vec<u32> {
        let buffer = e.mem.u32(array.addr() + 4);
        (0..e.mem.u32(array.addr() + 8))
            .map(|i| e.mem.u32(buffer + 4 * i))
            .collect()
    }

    #[test]
    fn extra_info_is_appended_to_the_extra_edge_info_array() {
        let mut e = Engine::new();
        e.register(EXTRA_INFO_ADD, |_, a| result(a[0] * 3 + a[1]));
        let mesh: Ptr<NavMesh> = Ptr::new(0x1000);
        assert_eq!(
            e.call(0x0069_1020, &args![mesh, 5u32]).u32(),
            (0x1000 + 0x48) * 3 + 5
        );
    }

    #[test]
    fn edge_bits_are_unpacked_from_the_triangle_flags() {
        let mut e = Engine::new();
        let tri: Ptr<NavMeshTriangle> = e.new_object();
        let out = Ptr::<()>::new(e.mem.alloc(8));
        let (flag_a, flag_b) = (out.byte_add(2), out.byte_add(3));
        e.set(tri, NavMeshTriangle::TriangleFlags, 0x2b << 22);
        e.call(0x0069_1040, &args![tri, 1u16, out, flag_a, flag_b]);
        assert_eq!(e.mem.u16(out.addr()), 0xb);
        assert_eq!(e.mem.u8(flag_a.addr()), 0);
        assert_eq!(e.mem.u8(flag_b.addr()), 1);
        e.set(tri, NavMeshTriangle::TriangleFlags, 0x1f << 16);
        e.call(0x0069_1040, &args![tri, 0u16, out, flag_a, flag_b]);
        assert_eq!(e.mem.u16(out.addr()), 0xf);
        assert_eq!(e.mem.u8(flag_a.addr()), 1);
        assert_eq!(e.mem.u8(flag_b.addr()), 0);
    }

    #[test]
    fn cover_array_lists_cover_triangles_without_flag_0x20() {
        let mut e = engine();
        e.register(ARRAY_CLEAR, clear_array);
        e.register(U16_ARRAY_GROW, grow_array);
        noops(&mut e, &[U16_ARRAY_CONSTRUCT_ELEMENTS]);
        // Triangle 0: cover; 1: cover but flag 0x20; 2: no cover; 3: cover.
        let mesh = mesh_with(
            &mut e,
            &[],
            &[
                ([0, 0, 0], [0; 3], 0x0002_0000),
                ([0, 0, 0], [0; 3], 0x0002_0020),
                ([0, 0, 0], [0; 3], 0),
                ([0, 0, 0], [0; 3], 0x0400_0000),
            ],
        );
        // A stale entry is dropped by the clear.
        e.mem.set_u32(mesh.addr() + 0x78 + 8, 9);
        e.call_log = Some(vec![]);
        e.call(0x0069_10a0, &args![mesh]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, ARRAY_CLEAR),
            vec![vec![mesh.addr() + 0x78, 0]]
        );
        let cover = e.mem.u32(mesh.addr() + 0x78 + 4);
        assert_eq!(e.mem.u32(mesh.addr() + 0x78 + 8), 2);
        assert_eq!(e.mem.u16(cover), 0);
        assert_eq!(e.mem.u16(cover + 2), 3);
    }

    #[test]
    fn flag_test_is_true_when_any_masked_bit_is_set() {
        let mut e = Engine::new();
        let tri: Ptr<NavMeshTriangle> = e.new_object();
        e.set(tri, NavMeshTriangle::TriangleFlags, 0b0110);
        assert!(e.call(0x0069_1140, &args![tri, 0b0100u32]).bool());
        assert!(!e.call(0x0069_1140, &args![tri, 0b1001u32]).bool());
    }

    #[test]
    fn a_new_triangle_has_no_vertices_neighbours_or_flags() {
        let mut e = Engine::new();
        let tri: Ptr<NavMeshTriangle> = e.new_object();
        e.mem.write(tri.addr(), &[7u8; 16]);
        assert_eq!(e.call(0x0069_1160, &args![tri]).u32(), tri.addr());
        for slot in 0..6 {
            assert_eq!(e.mem.u16(tri.addr() + 2 * slot), 0xffff);
        }
        assert_eq!(e.mem.u32(tri.addr() + 0xc), 0);
    }

    #[test]
    fn grid_cell_count_is_the_side_squared() {
        let mut e = Engine::new();
        e.register(POINTER_GET, |e, a| result(e.mem.u32(a[0])));
        let mesh: Ptr<NavMesh> = e.new_object();
        set_grid(&mut e, mesh, 5, 1.0);
        assert_eq!(e.call(0x0069_11c0, &args![mesh]).u32(), 25);
    }

    #[test]
    fn grid_cell_address_is_null_past_the_last_cell() {
        let mut e = Engine::new();
        let mesh: Ptr<NavMesh> = e.new_object();
        let grid = set_grid(&mut e, mesh, 2, 1.0);
        e.set(grid, NavMeshGrid::GridData, Ptr::new(0x5000));
        assert_eq!(e.call(0x0069_1210, &args![grid, 3u32]).u32(), 0x5000 + 48);
        assert_eq!(e.call(0x0069_1210, &args![grid, 4u32]).u32(), 0);
        // The mesh's forwarding version asks the grid inside the mesh.
        assert_eq!(e.call(0x0069_11f0, &args![mesh, 1u32]).u32(), 0x5000 + 16);
        assert_eq!(e.call(0x0069_11f0, &args![mesh, 9u32]).u32(), 0);
    }

    #[test]
    fn point_to_grid_cell_clamps_and_numbers_rows_first() {
        let mut e = grid_engine();
        let mesh: Ptr<NavMesh> = e.new_object();
        let grid = set_grid(&mut e, mesh, 4, 10.0);
        let inside = point(&mut e, 25.0, 35.0);
        // Column 2, row 3: 3 * 4 + 2.
        assert_eq!(e.call(0x0069_1260, &args![grid, inside]).u32(), 14);
        assert_eq!(e.call(0x0069_1240, &args![mesh, inside]).u32(), 14);
        // Far outside on both sides: clamped to column 0 and row 3.
        let outside = point(&mut e, -5.0, 1000.0);
        let out = Ptr::<()>::new(e.mem.alloc(8));
        e.call(0x0069_12a0, &args![grid, outside, out, out.byte_add(4)]);
        assert_eq!(e.mem.u32(out.addr()), 3);
        assert_eq!(e.mem.u32(out.addr() + 4), 0);
    }

    #[test]
    fn grid_cells_of_a_triangle_are_those_whose_list_has_it() {
        let mut e = grid_engine();
        // Corners in cells (column, row): (0, 0), (1, 0), (1, 1).
        let mesh = mesh_with(
            &mut e,
            &[[5.0, 5.0, 0.0], [15.0, 5.0, 0.0], [15.0, 15.0, 0.0]],
            &[([0, 1, 2], [0xffff; 3], 0)],
        );
        set_grid(&mut e, mesh, 4, 10.0);
        // Cell (column, row) lists the triangle except (0, 1): cell number
        // row * 4 + column.
        e.register(GRID_CELL_CONTAINS, |_, a| {
            assert_eq!(a[1], 0);
            result((a[2] != 4) as u32)
        });
        let out: Ptr<BSSimpleArray> = e.new_object();
        e.mem.set_u32(out.addr() + 8, 7);
        let count = e.call(0x0069_1350, &args![mesh, 0u16, out]).u32();
        assert_eq!(count, 3);
        // Rows are the outer loop: row 0 (cells 0, 1), row 1 (cell 5).
        assert_eq!(words(&e, out), vec![0, 1, 5]);
    }

    #[test]
    fn grid_cell_membership_asks_the_cell_by_row_and_column() {
        let mut e = Engine::new();
        e.register(GRID_CELL_CONTAINS, |_, a| result((a[2] == 11) as u32));
        let grid: Ptr<NavMeshGrid> = e.new_object();
        e.set(grid, NavMeshGrid::iGridSize, 4);
        // Column 3, row 2 is cell 2 * 4 + 3.
        assert!(e.call(0x0069_14e0, &args![grid, 6u16, 3u32, 2u32]).bool());
        assert!(!e.call(0x0069_14e0, &args![grid, 6u16, 2u32, 3u32]).bool());
    }

    #[test]
    fn a_static_avoid_node_is_built_flagged_and_appended() {
        use std::{cell::RefCell, rc::Rc};
        let mut e = engine();
        noops(&mut e, &[POINT_CONSTRUCTOR]);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = seen.clone();
        e.register_double(AVOID_NODE_ADD, move |e, a| {
            let bytes: Vec<u8> = (0..0x28).map(|i| e.mem.u8(a[1] + i)).collect();
            log.borrow_mut().push((a[0], bytes));
            result(6)
        });
        let mesh = mesh_with(&mut e, &[], &[([0; 3], [0; 3], 1), ([0; 3], [0; 3], 2)]);
        let at = Ptr::<()>::new(e.mem.alloc(12));
        for (i, v) in [1.5f32, 2.5, 3.5].iter().enumerate() {
            e.mem.set_f32(at.addr() + 4 * i as u32, *v);
        }
        let index = e
            .call(0x0069_1510, &args![mesh, at, 1u16, 4.0f32, 0.5f32])
            .u32();
        assert_eq!(index, 6);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].0, mesh.addr() + 0xf4);
        let node = &seen[0].1;
        assert_eq!(f32::from_le_bytes(node[0..4].try_into().unwrap()), 1.5);
        assert_eq!(f32::from_le_bytes(node[8..12].try_into().unwrap()), 3.5);
        assert_eq!(
            f32::from_le_bytes(node[0x18..0x1c].try_into().unwrap()),
            4.0
        );
        assert_eq!(
            f32::from_le_bytes(node[0x1c..0x20].try_into().unwrap()),
            0.5
        );
        assert_eq!(node[0x24], 1);
        assert_eq!(triangle_ptr_flags(&e, mesh, 1), 0x8000_0002);
        assert_eq!(triangle_ptr_flags(&e, mesh, 0), 1);
    }

    fn triangle_ptr_flags(e: &Engine, mesh: Ptr<NavMesh>, index: u32) -> u32 {
        e.mem.u32(triangle_ptr(e, mesh, index).addr() + 0xc)
    }

    #[test]
    fn triangle_flags_are_or_ed() {
        let mut e = Engine::new();
        let tri: Ptr<NavMeshTriangle> = e.new_object();
        e.set(tri, NavMeshTriangle::TriangleFlags, 0b0101);
        e.call(0x0069_1570, &args![tri, 0b0011u32]);
        assert_eq!(e.mem.u32(tri.addr() + 0xc), 0b0111);
    }

    #[test]
    fn avoid_node_constructor_stores_the_triangle_after_the_base() {
        let mut e = Engine::new();
        noops(&mut e, &[POINT_CONSTRUCTOR]);
        let node: Ptr<NavMeshStaticAvoidNode> = e.new_object();
        let at = point(&mut e, 9.0, 8.0);
        e.call_log = Some(vec![]);
        let result = e.call(0x0069_1590, &args![node, at, 42u16, 2.0f32, 3.0f32]);
        assert_eq!(result.u32(), node.addr());
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, POINT_CONSTRUCTOR),
            vec![vec![node.addr() + 0xc]]
        );
        assert_eq!(e.mem.f32(node.addr()), 9.0);
        assert_eq!(e.mem.f32(node.addr() + 0x18), 2.0);
        assert_eq!(e.mem.f32(node.addr() + 0x1c), 3.0);
        assert_eq!(e.mem.u16(node.addr() + 0x24), 42);
    }

    #[test]
    fn avoid_node_base_copies_the_point_and_clears_the_type() {
        let mut e = Engine::new();
        noops(&mut e, &[POINT_CONSTRUCTOR]);
        let node: Ptr<NavMeshStaticAvoidNode> = e.new_object();
        e.mem.set_u32(node.addr() + 0x20, 99);
        let at = Ptr::<()>::new(e.mem.alloc(12));
        for (i, v) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            e.mem.set_f32(at.addr() + 4 * i as u32, *v);
        }
        assert_eq!(
            e.call(0x0069_15d0, &args![node, at, 7.0f32, 8.0f32]).u32(),
            node.addr()
        );
        assert_eq!(e.mem.f32(node.addr() + 8), 3.0);
        assert_eq!(e.mem.f32(node.addr() + 0x18), 7.0);
        assert_eq!(e.mem.f32(node.addr() + 0x1c), 8.0);
        assert_eq!(e.mem.u32(node.addr() + 0x20), 0);
    }

    #[test]
    fn avoid_node_count_and_element_address() {
        let mut e = engine();
        let mesh: Ptr<NavMesh> = e.new_object();
        e.mem.set_u32(mesh.addr() + 0xf4 + 4, 0x7000);
        e.mem.set_u32(mesh.addr() + 0xf4 + 8, 3);
        assert_eq!(e.call(0x0069_1620, &args![mesh]).u32(), 3);
        assert_eq!(e.call(0x0069_1640, &args![mesh, 2u32]).u32(), 0x7000 + 80);
        assert_eq!(
            e.call(
                0x0069_1750,
                &args![mesh.at(NavMesh::StaticAvoidNodes), 1u32]
            )
            .u32(),
            0x7000 + 40
        );
    }

    #[test]
    fn avoid_nodes_of_a_triangle_are_collected() {
        let mut e = engine();
        e.register(ARRAY_PUSH_WORD, push_word);
        e.register(AVOID_NODE_TRIANGLE, |e, a| {
            result(e.mem.u16(a[0] + 0x24) as u32)
        });
        e.register(ARRAY_IS_EMPTY, |e, a| {
            result((e.mem.u32(a[0] + 8) == 0) as u32)
        });
        let mesh: Ptr<NavMesh> = e.new_object();
        let nodes = e.mem.alloc(0x28 * 3);
        for (i, triangle) in [3u16, 5, 3].iter().enumerate() {
            e.mem.set_u16(nodes + 0x28 * i as u32 + 0x24, *triangle);
        }
        e.mem.set_u32(mesh.addr() + 0xf4 + 4, nodes);
        e.mem.set_u32(mesh.addr() + 0xf4 + 8, 3);
        let out: Ptr<BSSimpleArray> = e.new_object();
        assert!(e.call(0x0069_1660, &args![mesh, 3u16, out]).bool());
        assert_eq!(words(&e, out), vec![nodes, nodes + 0x50]);
        let none: Ptr<BSSimpleArray> = e.new_object();
        assert!(!e.call(0x0069_1660, &args![mesh, 9u16, none]).bool());
        assert!(words(&e, none).is_empty());
    }

    #[test]
    fn map_base_constructor_allocates_and_clears_the_buckets() {
        let mut e = Engine::new();
        e.register(ALLOCATE, |e, a| result(e.mem.alloc(a[0])));
        noops(&mut e, &[MEMORY_SET]);
        let map: Ptr<NiTPointerMap> = e.new_object();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0069_18c0, &args![map, 0x25u32]).u32(), map.addr());
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(map.addr()), 0x0106_abf8);
        assert_eq!(e.get(map, NiTPointerMap::m_uiHashSize), 0x25);
        assert_eq!(e.get(map, NiTPointerMap::m_uiCount), 0);
        let table = e.get(map, NiTPointerMap::m_ppkHashTable);
        assert_eq!(calls_to(&log, ALLOCATE), vec![vec![0x94]]);
        assert_eq!(calls_to(&log, MEMORY_SET), vec![vec![table, 0, 0x94]]);
    }

    #[test]
    fn obstacle_map_constructor_stores_its_own_vtable_over_the_base() {
        let mut e = Engine::new();
        e.register(ALLOCATE, |e, a| result(e.mem.alloc(a[0])));
        noops(&mut e, &[MEMORY_SET]);
        let map: Ptr<NiTPointerMap> = e.new_object();
        assert_eq!(e.call(0x0069_16f0, &args![map, 7u32]).u32(), map.addr());
        assert_eq!(e.mem.u32(map.addr()), 0x0106_aba4);
        assert_eq!(e.get(map, NiTPointerMap::m_uiHashSize), 7);
    }

    #[test]
    fn pov_map_constructor_calls_its_base_then_stores_the_vtable() {
        let mut e = Engine::new();
        noops(&mut e, &[POV_MAP_BASE_CONSTRUCTOR]);
        let map: Ptr<NiTPointerMap> = e.new_object();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0069_1720, &args![map, 0x25u32]).u32(), map.addr());
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, POV_MAP_BASE_CONSTRUCTOR),
            vec![vec![map.addr(), 0x25]]
        );
        assert_eq!(e.mem.u32(map.addr()), 0x0106_abc4);
    }

    #[test]
    fn map_scalar_deleting_destructors_free_only_when_asked() {
        let mut e = Engine::new();
        noops(
            &mut e,
            &[
                OBSTACLE_MAP_DESTRUCTOR_BODY,
                POV_MAP_DESTRUCTOR_BODY,
                OPERATOR_DELETE,
            ],
        );
        let map: Ptr<NiTPointerMap> = Ptr::new(0x3000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0069_1770, &args![map, 1u32]).u32(), 0x3000);
        assert_eq!(e.call(0x0069_1770, &args![map, 0u32]).u32(), 0x3000);
        assert_eq!(e.call(0x0069_17a0, &args![map, 3u32]).u32(), 0x3000);
        assert_eq!(e.call(0x0069_17a0, &args![map, 2u32]).u32(), 0x3000);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, OBSTACLE_MAP_DESTRUCTOR_BODY).len(), 2);
        assert_eq!(calls_to(&log, POV_MAP_DESTRUCTOR_BODY).len(), 2);
        // Only the calls with bit 0 set free the block.
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![0x3000]; 2]);
    }

    #[test]
    fn member_array_constructors_store_vtable_then_initialise() {
        let mut e = Engine::new();
        noops(
            &mut e,
            &[
                U16_ARRAY_INITIALISE,
                VERTEX_ARRAY_INITIALISE,
                TRIANGLE_ARRAY_INITIALISE,
                EXTRA_INFO_ARRAY_INITIALISE,
            ],
        );
        let array: Ptr<BSSimpleArray> = e.new_object();
        e.call_log = Some(vec![]);
        for (constructor, initialise, vtable) in [
            (0x0069_17d0, U16_ARRAY_INITIALISE, 0x0106_abe4),
            (0x0069_1a60, VERTEX_ARRAY_INITIALISE, 0x0106_ac18),
            (0x0069_1b10, TRIANGLE_ARRAY_INITIALISE, 0x0106_ac2c),
            (0x0069_1bc0, EXTRA_INFO_ARRAY_INITIALISE, 0x0106_ac40),
        ] {
            e.mem.set_u32(array.addr(), 0);
            assert_eq!(e.call(constructor, &args![array]).u32(), array.addr());
            assert_eq!(e.mem.u32(array.addr()), vtable);
            let log = e.call_log.as_ref().unwrap().clone();
            assert_eq!(calls_to(&log, initialise), vec![vec![array.addr(), 0, 0]]);
        }
    }

    #[test]
    fn member_array_destructors_store_vtable_then_free_the_buffer() {
        let mut e = Engine::new();
        noops(&mut e, &[ARRAY_CLEAR]);
        let array: Ptr<BSSimpleArray> = e.new_object();
        for (destructor, vtable) in [
            (0x0069_1800, 0x0106_abe4),
            (0x0069_1a90, 0x0106_ac18),
            (0x0069_1b40, 0x0106_ac2c),
            (0x0069_1bf0, 0x0106_ac40),
        ] {
            e.call_log = Some(vec![]);
            e.mem.set_u32(array.addr(), 0);
            e.call(destructor, &args![array]);
            let log = e.call_log.take().unwrap();
            assert_eq!(e.mem.u32(array.addr()), vtable);
            assert_eq!(calls_to(&log, ARRAY_CLEAR), vec![vec![array.addr(), 1]]);
        }
    }

    #[test]
    fn a_word_is_appended_to_the_u16_array_after_construction() {
        let mut e = Engine::new();
        e.register(U16_ARRAY_GROW, grow_array);
        e.register_double(U16_ARRAY_CONSTRUCT_ELEMENTS, |e, a| {
            // The element is constructed before the value is stored.
            e.mem.set_u16(a[1], 0xdead);
            Ret::default()
        });
        let array: Ptr<BSSimpleArray> = e.new_object();
        let value = Ptr::<()>::new(e.mem.alloc(4));
        e.mem.set_u16(value.addr(), 0x1234);
        assert_eq!(e.call(0x0069_1820, &args![array, value]).u32(), 0);
        e.mem.set_u16(value.addr(), 0x5678);
        assert_eq!(e.call(0x0069_1820, &args![array, value]).u32(), 1);
        let buffer = e.mem.u32(array.addr() + 4);
        assert_eq!(e.mem.u16(buffer), 0x1234);
        assert_eq!(e.mem.u16(buffer + 2), 0x5678);
    }

    #[test]
    fn u16_array_storage_sizes_are_twice_the_element_count() {
        let mut e = Engine::new();
        e.register(OPERATOR_NEW, |_, a| result(a[0] + 1));
        noops(&mut e, &[DEALLOCATE_SIZED]);
        let array: Ptr<BSSimpleArray> = Ptr::new(0x100);
        assert_eq!(e.call(0x0069_1870, &args![array, 10u32]).u32(), 21);
        e.call_log = Some(vec![]);
        e.call(0x0069_18a0, &args![array, Ptr::<()>::new(0x400), 10u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, DEALLOCATE_SIZED), vec![vec![0x400, 20]]);
    }

    #[test]
    fn obstacle_item_value_is_assigned_then_the_argument_released() {
        let mut e = Engine::new();
        e.register_double(POINTER_ASSIGN_FROM, |e, a| {
            // The argument is the address of a word holding the pointer.
            let held = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], held);
            Ret::default()
        });
        noops(&mut e, &[POINTER_RELEASE]);
        let item = Ptr::<()>::new(e.mem.alloc(12));
        let map: Ptr<NiTPointerMap> = Ptr::new(0x100);
        e.call_log = Some(vec![]);
        e.call(
            0x0069_1930,
            &args![map, item, 0x1234u16, Ptr::<()>::new(0x9000)],
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u16(item.addr() + 4), 0x1234);
        assert_eq!(e.mem.u32(item.addr() + 8), 0x9000);
        let release = calls_to(&log, POINTER_RELEASE);
        assert_eq!(release.len(), 1);
        assert_eq!(
            calls_to(&log, POINTER_ASSIGN_FROM)[0][1],
            release[0][0],
            "the same temporary is assigned from and released"
        );
    }

    #[test]
    fn vertex_and_triangle_arrays_append_copies_after_construction() {
        let mut e = Engine::new();
        e.register(VERTEX_ARRAY_GROW, grow_array);
        e.register(TRIANGLE_ARRAY_GROW, grow_array);
        noops(
            &mut e,
            &[
                VERTEX_ARRAY_CONSTRUCT_ELEMENTS,
                TRIANGLE_ARRAY_CONSTRUCT_ELEMENTS,
            ],
        );
        let vertices: Ptr<BSSimpleArray> = e.new_object();
        let triangles: Ptr<BSSimpleArray> = e.new_object();
        let source = Ptr::<()>::new(e.mem.alloc(16));
        for word in 0..4 {
            e.mem.set_u32(source.addr() + 4 * word, 0x10 + word);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0069_1ab0, &args![vertices, source]).u32(), 0);
        assert_eq!(e.call(0x0069_1ab0, &args![vertices, source]).u32(), 1);
        assert_eq!(e.call(0x0069_1b60, &args![triangles, source]).u32(), 0);
        assert_eq!(e.call(0x0069_1b60, &args![triangles, source]).u32(), 1);
        let log = e.call_log.take().unwrap();
        let vertex_buffer = e.mem.u32(vertices.addr() + 4);
        // Three words per vertex, 12 bytes apart; the fourth word is not copied.
        assert_eq!(e.mem.u32(vertex_buffer + 12 + 8), 0x12);
        assert_eq!(e.mem.u32(vertex_buffer + 12 + 12), 0);
        let triangle_buffer = e.mem.u32(triangles.addr() + 4);
        assert_eq!(e.mem.u32(triangle_buffer + 16 + 12), 0x13);
        assert_eq!(
            calls_to(&log, VERTEX_ARRAY_CONSTRUCT_ELEMENTS)[1],
            vec![vertices.addr(), vertex_buffer + 12, 1]
        );
        assert_eq!(
            calls_to(&log, TRIANGLE_ARRAY_CONSTRUCT_ELEMENTS)[1],
            vec![triangles.addr(), triangle_buffer + 16, 1]
        );
    }

    /// Address the fake allocator double (virtual slot 4) is registered at.
    const FAKE_ALLOCATE: u32 = 0x00ff_0001;

    /// An array object whose vtable slot 4 is a double that returns a fresh
    /// 256-byte block and logs nothing else.
    fn array_with_allocator(e: &mut Engine) -> Ptr<BSSimpleArray> {
        e.register(FAKE_ALLOCATE, |e, _| result(e.mem.alloc(256)));
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, FAKE_ALLOCATE);
        let array: Ptr<BSSimpleArray> = e.new_object();
        e.mem.set_u32(array.addr(), vtable);
        array
    }

    #[test]
    fn extra_info_add_constructs_then_copy_constructs() {
        let mut e = Engine::new();
        e.register(EXTRA_INFO_ARRAY_GROW, grow_array);
        noops(
            &mut e,
            &[EXTRA_INFO_CONSTRUCT_ELEMENTS, EXTRA_INFO_COPY_CONSTRUCT],
        );
        let array: Ptr<BSSimpleArray> = e.new_object();
        let source = Ptr::<()>::new(e.mem.alloc(12));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0069_1c10, &args![array, source]).u32(), 0);
        assert_eq!(e.call(0x0069_1c10, &args![array, source]).u32(), 1);
        let log = e.call_log.take().unwrap();
        let buffer = e.mem.u32(array.addr() + 4);
        assert_eq!(
            calls_to(&log, EXTRA_INFO_CONSTRUCT_ELEMENTS)[1],
            vec![array.addr(), buffer + 12, 1]
        );
        assert_eq!(
            calls_to(&log, EXTRA_INFO_COPY_CONSTRUCT)[1],
            vec![buffer + 12, source.addr()]
        );
    }

    #[test]
    fn door_and_closed_door_arrays_construct_and_destroy() {
        let mut e = Engine::new();
        noops(&mut e, &[DOOR_ARRAY_INITIALISE, ARRAY_CLEAR]);
        let array: Ptr<BSSimpleArray> = e.new_object();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0069_1d40, &args![array]).u32(), array.addr());
        assert_eq!(e.mem.u32(array.addr()), DOOR_PORTAL_ARRAY_VTABLE);
        e.call(0x0069_1d70, &args![array]);
        assert_eq!(e.call(0x0069_1d90, &args![array]).u32(), array.addr());
        assert_eq!(e.mem.u32(array.addr()), CLOSED_DOOR_ARRAY_VTABLE);
        e.call(0x0069_1dc0, &args![array]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, DOOR_ARRAY_INITIALISE),
            vec![vec![array.addr(), 0, 0]; 2]
        );
        assert_eq!(calls_to(&log, ARRAY_CLEAR), vec![vec![array.addr(), 1]; 2]);
    }

    #[test]
    fn pov_map_base_constructor_allocates_and_clears_the_bucket_table() {
        let mut e = Engine::new();
        e.register(ALLOCATE, |e, _| result(e.mem.alloc(0x100)));
        noops(&mut e, &[MEMORY_SET]);
        let map: Ptr<NiTPointerMap> = e.new_object();
        e.mem.set_u32(map.addr() + 0xc, 7);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0069_1de0, &args![map, 0x25u32]).u32(), map.addr());
        let log = e.call_log.take().unwrap();
        let table = e.mem.u32(map.addr() + 8);
        assert_ne!(table, 0);
        assert_eq!(e.mem.u32(map.addr()), MAP_ITEM_BASE_VTABLE);
        assert_eq!(e.mem.u32(map.addr() + 4), 0x25);
        assert_eq!(e.mem.u32(map.addr() + 0xc), 0);
        assert_eq!(calls_to(&log, ALLOCATE), vec![vec![0x94]]);
        assert_eq!(calls_to(&log, MEMORY_SET), vec![vec![table, 0, 0x94]]);
    }

    #[test]
    fn obstacle_map_keys_compare_as_unsigned_shorts() {
        let mut e = Engine::new();
        let map: Ptr<NiTPointerMap> = e.new_object();
        assert!(e.call(0x0069_1e50, &args![map, 5u16, 5u16]).bool());
        assert!(!e.call(0x0069_1e50, &args![map, 5u16, 6u16]).bool());
    }

    #[test]
    fn pov_map_set_value_stores_key_and_packed_value() {
        let mut e = Engine::new();
        let map: Ptr<NiTPointerMap> = e.new_object();
        let item = Ptr::<()>::new(e.mem.alloc(16));
        e.call(0x0069_1e70, &args![map, item, 0xbeefu16, 0x1122_3344u32]);
        assert_eq!(e.mem.u16(item.addr() + 4), 0xbeef);
        assert_eq!(e.mem.u32(item.addr() + 6), 0x1122_3344);
    }

    #[test]
    fn pov_map_destructors_remove_items_and_free_the_table() {
        let mut e = Engine::new();
        noops(&mut e, &[MAP_REMOVE_ALL, FREE_TABLE]);
        let map: Ptr<NiTPointerMap> = e.new_object();
        e.mem.set_u32(map.addr() + 8, 0x4444);
        e.call_log = Some(vec![]);
        e.call(0x0069_1ea0, &args![map]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(map.addr()), MAP_ITEM_BASE_VTABLE);
        assert_eq!(calls_to(&log, MAP_REMOVE_ALL), vec![vec![map.addr()]; 2]);
        assert_eq!(calls_to(&log, FREE_TABLE), vec![vec![0x4444]]);
        e.call_log = Some(vec![]);
        e.call(0x0069_1f00, &args![map]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, MAP_REMOVE_ALL).len(), 1);
        assert_eq!(calls_to(&log, FREE_TABLE), vec![vec![0x4444]]);
    }

    #[test]
    fn pov_map_item_release_resets_value_then_frees_item() {
        let mut e = Engine::new();
        noops(&mut e, &[POV_ITEM_RESET_VALUE, POV_ITEM_FREE]);
        let map: Ptr<NiTPointerMap> = e.new_object();
        let item = Ptr::<()>::new(e.mem.alloc(16));
        e.call_log = Some(vec![]);
        e.call(0x0069_1f30, &args![map, item]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, POV_ITEM_RESET_VALUE),
            vec![vec![item.addr() + 6, 0]]
        );
        assert_eq!(
            calls_to(&log, POV_ITEM_FREE),
            vec![vec![map.addr() + 0xc, item.addr()]]
        );
    }

    #[test]
    fn obstacle_array_constructor_and_destructor() {
        let mut e = Engine::new();
        noops(&mut e, &[OBSTACLES_INITIALISE, OBSTACLES_CLEAR]);
        let array: Ptr<BSSimpleArray> = e.new_object();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0069_1f60, &args![array]).u32(), array.addr());
        assert_eq!(e.mem.u32(array.addr()), OBSTACLE_ARRAY_VTABLE);
        e.call(0x0069_1f90, &args![array]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, OBSTACLES_INITIALISE),
            vec![vec![array.addr(), 0, 0]]
        );
        assert_eq!(calls_to(&log, OBSTACLES_CLEAR), vec![vec![array.addr(), 1]]);
    }

    #[test]
    fn avoid_node_array_constructor_and_destructor() {
        let mut e = Engine::new();
        noops(&mut e, &[AVOID_NODE_CONSTRUCT_ELEMENTS, ARRAY_CLEAR]);
        let array: Ptr<BSSimpleArray> = e.new_object();
        e.mem.set_u32(array.addr() + 4, 0x1234);
        e.mem.set_u32(array.addr() + 8, 9);
        e.mem.set_u32(array.addr() + 12, 9);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0069_1fb0, &args![array]).u32(), array.addr());
        assert_eq!(e.mem.u32(array.addr()), AVOID_NODE_ARRAY_VTABLE);
        assert_eq!(e.mem.u32(array.addr() + 4), 0);
        assert_eq!(e.mem.u32(array.addr() + 8), 0);
        assert_eq!(e.mem.u32(array.addr() + 12), 0);
        e.call(0x0069_1fe0, &args![array]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, AVOID_NODE_CONSTRUCT_ELEMENTS).is_empty());
        assert_eq!(calls_to(&log, ARRAY_CLEAR), vec![vec![array.addr(), 1]]);
    }

    #[test]
    fn avoid_node_add_copies_ten_words() {
        let mut e = Engine::new();
        e.register(AVOID_NODE_ARRAY_GROW, grow_array);
        noops(&mut e, &[AVOID_NODE_CONSTRUCT_ELEMENTS]);
        let array: Ptr<BSSimpleArray> = e.new_object();
        let source = Ptr::<()>::new(e.mem.alloc(48));
        for word in 0..11 {
            e.mem.set_u32(source.addr() + 4 * word, 0x100 + word);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0069_2000, &args![array, source]).u32(), 0);
        assert_eq!(e.call(0x0069_2000, &args![array, source]).u32(), 1);
        let log = e.call_log.take().unwrap();
        let buffer = e.mem.u32(array.addr() + 4);
        assert_eq!(e.mem.u32(buffer + 0x28), 0x100);
        assert_eq!(e.mem.u32(buffer + 0x28 + 36), 0x109);
        assert_eq!(e.mem.u32(buffer + 0x28 + 40), 0, "only ten words");
        assert_eq!(
            calls_to(&log, AVOID_NODE_CONSTRUCT_ELEMENTS)[1],
            vec![array.addr(), buffer + 0x28, 1]
        );
    }

    #[test]
    fn scalar_deleting_destructors_run_the_destructor_and_free_on_bit_zero() {
        let mut e = Engine::new();
        noops(
            &mut e,
            &[
                ARRAY_CLEAR,
                OBSTACLES_CLEAR,
                MAP_REMOVE_ALL,
                FREE_TABLE,
                OBSTACLE_MAP_BASE_DESTRUCTOR,
                OPERATOR_DELETE,
            ],
        );
        let object: Ptr<BSSimpleArray> = e.new_object();
        // (entry, the call the destructor makes first).
        let cases = [
            (0x0069_2050, OBSTACLE_MAP_BASE_DESTRUCTOR),
            (0x0069_2080, ARRAY_CLEAR),
            (0x0069_20b0, ARRAY_CLEAR),
            (0x0069_20e0, ARRAY_CLEAR),
            (0x0069_2110, ARRAY_CLEAR),
            (0x0069_2140, ARRAY_CLEAR),
            (0x0069_2170, MAP_REMOVE_ALL),
            (0x0069_21a0, OBSTACLES_CLEAR),
            (0x0069_21d0, ARRAY_CLEAR),
        ];
        for (entry, inner) in cases {
            for flags in [0u32, 1, 2, 3] {
                e.call_log = Some(vec![]);
                let returned = e.call(entry, &args![object, flags]).u32();
                let log = e.call_log.take().unwrap();
                assert_eq!(returned, object.addr(), "{entry:08x}");
                assert!(!calls_to(&log, inner).is_empty(), "{entry:08x}");
                let deleted = calls_to(&log, OPERATOR_DELETE);
                if flags & 1 != 0 {
                    assert_eq!(deleted, vec![vec![object.addr()]], "{entry:08x}");
                } else {
                    assert!(deleted.is_empty(), "{entry:08x}");
                }
            }
        }
    }

    #[test]
    fn unsigned_short_array_grow_branches() {
        let mut e = Engine::new();
        let full = std::rc::Rc::new(std::cell::Cell::new(false));
        let flag = full.clone();
        e.register_double(U16_ARRAY_IS_FULL, move |_, _| result(flag.get() as u32));
        e.register(U16_ARRAY_NEXT_CAPACITY, |_, _| result(8));
        noops(&mut e, &[U16_ARRAY_RESIZE]);
        let array = array_with_allocator(&mut e);
        // Not full: only the size changes.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0069_2200, &args![array]).u32(), 0);
        assert_eq!(e.mem.u32(array.addr() + 8), 1);
        assert_eq!(e.mem.u32(array.addr() + 4), 0);
        // Full with no capacity: 4 elements from the allocator.
        full.set(true);
        assert_eq!(e.call(0x0069_2200, &args![array]).u32(), 1);
        assert_eq!(e.mem.u32(array.addr() + 12), 4);
        assert_ne!(e.mem.u32(array.addr() + 4), 0);
        // Full with a capacity: next capacity and resize with the old size.
        assert_eq!(e.call(0x0069_2200, &args![array]).u32(), 2);
        assert_eq!(e.mem.u32(array.addr() + 12), 8);
        assert_eq!(e.mem.u32(array.addr() + 8), 3);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, U16_ARRAY_RESIZE),
            vec![vec![array.addr(), 8, 2]]
        );
    }

    #[test]
    fn avoid_node_array_initialiser_raises_capacity_to_count() {
        let mut e = Engine::new();
        noops(&mut e, &[AVOID_NODE_CONSTRUCT_ELEMENTS]);
        let array = array_with_allocator(&mut e);
        e.mem.set_u32(array.addr() + 8, 5);
        e.call_log = Some(vec![]);
        e.call(0x0069_2740, &args![array, 2u32, 6u32]);
        let log = e.call_log.take().unwrap();
        let buffer = e.mem.u32(array.addr() + 4);
        assert_ne!(buffer, 0);
        assert_eq!(e.mem.u32(array.addr() + 12), 6, "capacity raised to count");
        assert_eq!(e.mem.u32(array.addr() + 8), 6);
        assert_eq!(
            calls_to(&log, AVOID_NODE_CONSTRUCT_ELEMENTS),
            vec![vec![array.addr(), buffer, 6]]
        );
        // Capacity only: allocates, constructs nothing.
        e.call_log = Some(vec![]);
        e.call(0x0069_2740, &args![array, 3u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(array.addr() + 12), 3);
        assert_eq!(e.mem.u32(array.addr() + 8), 0);
        assert!(calls_to(&log, AVOID_NODE_CONSTRUCT_ELEMENTS).is_empty());
        // Nothing: no allocation.
        e.call(0x0069_2740, &args![array, 0u32, 0u32]);
        assert_eq!(e.mem.u32(array.addr() + 4), 0);
    }

    #[test]
    fn u16_array_reallocation_answers_the_new_block() {
        let mut e = Engine::new();
        e.register(DEALLOCATE_SIZED, |_, a| result(a[0] + a[1]));
        let array: Ptr<BSSimpleArray> = Ptr::new(0x100);
        assert_eq!(
            e.call(0x0069_18a0, &args![array, Ptr::<()>::new(0x400), 10u32])
                .u32(),
            0x414
        );
    }
}
