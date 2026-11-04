//! `fallout shared/pathfinding/navmeshrender.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `NavMeshRender` (0x214 bytes) is the debug drawing of navmeshes (the
//! console's navmesh view). It owns a scene graph node per cell
//! (`NavMeshCellMap`, +0x200, keyed by the cell), 17 "style" slots that each
//! hold a vertex-point shape (+0x0C), an edge material (+0x50), a triangle
//! material (+0x94) and a colour (+0xD8), and the alpha and local-map
//! materials (+0x1E8, +0x1EC). The singleton is the global `011d6e2c`
//! (`NavMeshRender::TheRender`), the node everything hangs under is the
//! static `spRootDrawNode` (`011d6e5c`, a `NiPointer<NiNode>`), and the
//! statics `bDrawCover` (`011d6e28`) and `bDrawConnections` (`011d6e29`)
//! switch the cover boxes and the connection lines on.
//!
//! How the translations read the game code:
//! - A `NiPointer<T>` is one word; `00559450` is its `operator*` (returns the
//!   word), `00633c90` its constructor from a pointer, `0066b0d0` its
//!   assignment, `0045cec0` its destructor. They are called by address like
//!   every callee outside this file.
//! - Arguments the decompiler hung on the wrong call were re-read from the
//!   disassembly. The compiler pushes the arguments of a call before the
//!   getter calls that produce its receiver: in `fn_006a5ca0`,
//!   `AddDrawOnlyNavMeshes` runs on the result of `00552ba0` (the getter of
//!   the singleton, which takes no argument), and the two pushed words are
//!   `AddDrawOnlyNavMeshes`' own arguments.
//! - C++ exception unwinding states and the stack cookie are not translated.
//! - `AddDrawOnlyNavMeshes` keeps its locals in one zeroed block (the game's
//!   stack frame) so the helpers can be passed their addresses; see the
//!   `F_*` offsets, named by what the game keeps there.
//!
//! The navmesh and triangle objects are read at their offsets (the classes
//! are another unit's): `NavMesh` +0x08 flags (bit 0x800 is tested by
//! `00440da0`), +0x28 the vertex array (`BSSimpleArray<NiPoint3>`), +0x38 the
//! triangle array (`BSSimpleArray` of 16-byte triangles, indexed by
//! `005582f0`), +0xF4 the sphere array; a triangle holds its three vertex
//! indices (+0x00, `fn_006a5b30`), its three neighbour triangle indices
//! (+0x06, `fn_006a5b70`) and flag bits at +0x0C.

#[allow(unused_imports)]
use crate::prelude::*;

// ---------------------------------------------------------------------------
// Layout and constants
// ---------------------------------------------------------------------------

layout! {
    /// `NavMeshRender` (Xbox PDB), 0x214 bytes on the Xbox and on PC. The
    /// arrays of `NiPointer`s and colours are addressed by the offsets below.
    pub struct NavMeshRender: 0x214 {
        /// `sp3DNode` (Xbox PDB): `NiPointer<NiNode>`.
        0x000 sp3DNode: u32,
        /// `spSelectedPathGeometry` (Xbox PDB): `NiPointer<NiNode>`.
        0x004 spSelectedPathGeometry: u32,
        /// `spLOSMapGeometry` (Xbox PDB): `NiPointer<NiNode>`.
        0x008 spLOSMapGeometry: u32,
        /// `spAlphaProp` (Xbox PDB): `NiPointer<NiAlphaProperty>`.
        0x1e8 spAlphaProp: u32,
        /// `spLocalMapMaterial` (Xbox PDB): `NiPointer<NiMaterialProperty>`.
        0x1ec spLocalMapMaterial: u32,
        /// `bDraw` (Xbox PDB).
        0x210 bDraw: bool,
    }
}

/// `spVertPoints` (Xbox PDB) at +0x0C: 17 `NiPointer<NiTriShape>`.
pub(crate) const VERT_POINTS: u32 = 0x0c;
/// `spEdgeAndVertexMaterialArray` (Xbox PDB) at +0x50: 17
/// `NiPointer<NiMaterialProperty>`.
pub(crate) const EDGE_AND_VERTEX_MATERIALS: u32 = 0x50;
/// `spTriangleMaterialArray` (Xbox PDB) at +0x94: 17
/// `NiPointer<NiMaterialProperty>`.
pub(crate) const TRIANGLE_MATERIALS: u32 = 0x94;
/// `ColorValues` (Xbox PDB) at +0xD8: 17 `NiColorA` (16 bytes each).
pub(crate) const COLOUR_VALUES: u32 = 0xd8;
/// `NavMeshNodes` (Xbox PDB) at +0x1F0: `NiTMap<NavMesh *, NiPointer<NiNode>>`.
pub(crate) const NAV_MESH_NODES: u32 = 0x1f0;
/// `NavMeshCellMap` (Xbox PDB) at +0x200: `NiTMap<TESObjectCELL *,
/// NiPointer<NiNode>>`.
pub(crate) const NAV_MESH_CELL_MAP: u32 = 0x200;
/// Number of style slots in each array.
const STYLE_COUNT: u32 = 0x11;

/// Style (colour) slots the drawing code picks, by what they colour.
/// The colour of an ordinary triangle's vertices and of its edges.
const STYLE_ORDINARY: u32 = 0;
/// The colour of the first line of a connection edge.
const STYLE_CONNECTION_START: u32 = 2;
/// The colour of the second line of a connection edge.
const STYLE_CONNECTION_END: u32 = 9;
/// Black: the vertices of a special mesh, the cover end boxes, and the
/// border edges of kind 1.
const STYLE_BLACK: u32 = 10;
/// The colours of the border edges of kind 0, 2, 15 and every other kind.
const STYLE_BORDER_KIND_0: u32 = 6;
const STYLE_BORDER_KIND_2: u32 = 5;
const STYLE_BORDER_KIND_15: u32 = 9;
const STYLE_BORDER_OTHER: u32 = 7;

/// `NavMeshRender::TheRender` (Xbox PDB static), the singleton.
const THE_RENDER: u32 = 0x011d_6e2c;
/// `NavMeshRender::spRootDrawNode` (Xbox PDB static), a `NiPointer<NiNode>`.
const ROOT_DRAW_NODE: u32 = 0x011d_6e5c;
/// `NavMeshRender::bDrawCover` (Xbox PDB static), a byte.
const DRAW_COVER: u32 = 0x011d_6e28;
/// `NavMeshRender::bDrawConnections` (Xbox PDB static), a byte.
const DRAW_CONNECTIONS: u32 = 0x011d_6e29;
/// The `TES` singleton pointer.
const TES_SINGLETON: u32 = 0x011d_ea10;
/// The `INI` setting whose value is the number of cells loaded around the
/// player (`0043d4d0` returns the address of its integer value).
const GRID_SETTING: u32 = 0x011c_63cc;

/// `NiColor` white (1, 1, 1): the ambient colour given to every material.
const COLOUR_WHITE: u32 = 0x011a_9b7c;
/// `float` read as the size of the vertex-point shapes.
const POINT_SHAPE_SIZE: u32 = 0x0119_cb84;
/// `float` constants of the cover boxes: width/depth of the main box
/// (`5.0`), width/depth of the end boxes (`7.0`), the extra height of the end
/// boxes (`2.0`) and the length of the end boxes' direction step (`10.0`).
const COVER_BOX_SIZE: u32 = 0x0119_cb9c;
const COVER_END_BOX_SIZE: u32 = 0x0119_cb90;
const COVER_END_BOX_EXTRA_HEIGHT: u32 = 0x0119_cb94;
const COVER_STEP: u32 = 0x0119_cb98;
/// The `z` of the translation of the second set of lines (`3.0`).
const LINES_C_HEIGHT: u32 = 0x0101_7718;
/// Weights of the three vertices in the point drawn for a connection edge.
const CONNECTION_WEIGHT_A: u32 = 0x0106_b524;
const CONNECTION_WEIGHT_B: u32 = 0x0102_6994;
const CONNECTION_WEIGHT_C: u32 = 0x0101_7870;
/// `0.5`, `0.25`, `0.75` as `float`s.
const HALF: u32 = 0x0101_6248;
const QUARTER: u32 = 0x0101_622c;
const THREE_QUARTERS: u32 = 0x0101_6264;
/// `16.0`: the factor of `fn_006a5af0`.
const BOX_HEIGHT_STEP: u32 = 0x0106_b4b8;

// Callees outside this file.
/// `operator new(size)` (`00401000`) and `operator delete(p)` (`00401030`).
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `vector_constructor_iterator(p, size, count, ctor)` (stdcall).
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x0040_1050;
/// `eh_vector_constructor_iterator(p, size, count, ctor, dtor)` and
/// `eh_vector_destructor_iterator(p, size, count, dtor)`.
const EH_VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x00ec_782f;
const EH_VECTOR_DESTRUCTOR_ITERATOR: u32 = 0x00ec_5fce;
/// `NiAlloc(size)` (`00aa13e0`, objects) and the raw array version
/// (`00aa1070`).
const NI_ALLOC: u32 = 0x00aa_13e0;
const NI_ALLOC_ARRAY: u32 = 0x00aa_1070;
/// `memcpy(dst, src, n)` (`00401460`) and `memset(p, value, n)`
/// (`00403d30`), cdecl.
const MEMCPY: u32 = 0x0040_1460;
const MEMSET: u32 = 0x0040_3d30;
/// `NiPointer<T>`: `NiPointer(T *)` (`00633c90`), `operator=(T *)`
/// (`0066b0d0`), `~NiPointer` (`0045cec0`), `operator*` (`00559450`), the
/// copy constructor (`00559a40`).
const NI_POINTER_CONSTRUCT: u32 = 0x0063_3c90;
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
const NI_POINTER_DESTROY: u32 = 0x0045_cec0;
const NI_POINTER_DEREF: u32 = 0x0055_9450;
const NI_POINTER_COPY: u32 = 0x0055_9a40;
/// The default constructors the vector helpers run on the `NiPointer<T>`
/// slots (`006694e0`, which stores a null pointer), `NiPoint3` (`006815c0`,
/// which does nothing) and `NiColorA` (`004a7800`, which zeroes all four).
const NI_POINTER_DEFAULT_CONSTRUCT: u32 = 0x0066_94e0;
const NI_POINT3_CONSTRUCT: u32 = 0x0068_15c0;
const NI_COLOR_A_DEFAULT_CONSTRUCT: u32 = 0x004a_7800;
/// `NiColorA(r, g, b, a)` (`00414430`) and `NiPoint3(x, y, z)` (`00416870`).
const NI_COLOR_A_CONSTRUCT: u32 = 0x0041_4430;
const NI_POINT3_CONSTRUCT_XYZ: u32 = 0x0041_6870;
/// `NiPoint3` arithmetic: `this * factor` (`0045bb20(this, out, factor)`),
/// `this + other` (`00439e90(this, out, other)`), `this - other`
/// (`00439ef0`), normalise in place (`004a0c10`) and scale in place
/// (`00439180(this, factor)`).
const NI_POINT3_SCALE: u32 = 0x0045_bb20;
const NI_POINT3_ADD: u32 = 0x0043_9e90;
const NI_POINT3_SUBTRACT: u32 = 0x0043_9ef0;
const NI_POINT3_NORMALISE: u32 = 0x004a_0c10;
const NI_POINT3_SCALE_IN_PLACE: u32 = 0x0043_9180;
/// Element accessors: `BSSimpleArray<NiPoint3>::operator[]` (`006a1440`,
/// 12-byte elements) and `BSSimpleArray<NiColorA>::operator[]` (`00558dd0`,
/// 16-byte elements).
const POINT_ARRAY_AT: u32 = 0x006a_1440;
const COLOUR_ARRAY_AT: u32 = 0x0055_8dd0;
/// Scene graph and property calls.
const NI_NODE_CONSTRUCT: u32 = 0x00a5_ecb0;
const NI_ALPHA_PROPERTY_CONSTRUCT: u32 = 0x0043_91c0;
const NI_ALPHA_PROPERTY_SET_BLENDING: u32 = 0x0049_ed90;
const NI_MATERIAL_PROPERTY_CONSTRUCT: u32 = 0x00a7_5650;
const NI_MATERIAL_SET_AMBIENT: u32 = 0x004b_c450;
const NI_MATERIAL_SET_ALPHA: u32 = 0x0068_c9f0;
const BS_SHADER_NO_LIGHTING_CONSTRUCT: u32 = 0x00b6_fc90;
const BS_SHADER_PROPERTY_SET_FLAG: u32 = 0x0044_1130;
const NI_AV_OBJECT_ATTACH_PROPERTY: u32 = 0x0043_9410;
const NI_AV_OBJECT_REMOVE_PROPERTY: u32 = 0x00a5_b230;
const NI_AV_OBJECT_SET_TRANSLATE: u32 = 0x0044_0460;
const NI_AV_OBJECT_SET_NAME: u32 = 0x00a5_b950;
const NI_AV_OBJECT_UPDATE: u32 = 0x00a5_9c60;
const NI_AV_OBJECT_UPDATE_PROPERTIES: u32 = 0x00a5_a040;
/// `NiNode`'s child clearing (`004def90`, called on the root before a
/// redraw).
const NI_NODE_CLEAR_CHILDREN: u32 = 0x004d_ef90;
const NI_UPDATE_DATA_CONSTRUCT: u32 = 0x0043_d410;
const NI_FIXED_STRING_CONSTRUCT: u32 = 0x0043_8170;
const NI_FIXED_STRING_DESTROY: u32 = 0x0043_81b0;
const NI_LINES_CONSTRUCT: u32 = 0x00a7_46e0;
const NI_TRI_SHAPE_CONSTRUCT: u32 = 0x00a7_4410;
const MAKE_TRI_POINT: u32 = 0x004b_29b0;
const SHADER_PROPERTY_SET_FLAG_RECURSE: u32 = 0x00ba_91a0;
const ADD_NO_LIGHTING_PROPERTY_RECURSE: u32 = 0x004b_6e90;
const SHADER_MANAGER_PREPARE_OBJECT: u32 = 0x00b5_7e30;
const TEST_OBJECTS_CREATE_BOX: u32 = 0x00c5_5d50;
const TEST_OBJECTS_CREATE_SPHERE: u32 = 0x00c5_6680;
/// The slot of `NiNode::AttachChild(child, firstAvailable)` and of
/// `DetachChild(child)` in the `NiNode` vtable.
const SLOT_ATTACH_CHILD: u32 = 0xdc;
const SLOT_DETACH_CHILD: u32 = 0xe8;
/// `NiAVObject`'s parent accessor used by `fn_006a61b0` (`009611e0`, returns
/// the word at +0x18).
const NODE_PARENT: u32 = 0x0096_11e0;
/// `NiTMap` operations: `GetAt(key, &out)` (`006c62d0`), `SetAt(key, value)`
/// (`006c6920`), `RemoveAt(key)` (`00405430`), `RemoveAll` (`00438af0`).
const NI_T_MAP_GET_AT: u32 = 0x006c_62d0;
const NI_T_MAP_SET_AT: u32 = 0x006c_6920;
const NI_T_MAP_REMOVE_AT: u32 = 0x0040_5430;
const NI_T_MAP_REMOVE_ALL: u32 = 0x0043_8af0;
/// The `NiTMapBase` destructor bodies of the two maps.
const NAV_MESH_NODE_MAP_BASE_DESTROY: u32 = 0x006a_64b0;
const NAV_MESH_CELL_MAP_BASE_DESTROY: u32 = 0x006a_65b0;
/// `BSSimpleArray` helpers: `BSSimpleArray<NiPoint3>` constructor body
/// (`0049f990`), the destructor body shared by all instances (`008454f0`),
/// the `BSSimpleArray<unsigned char>` constructor body (`0069b940`),
/// `BSSimpleArray<NiPoint3>::SetSize` (`0069cea0`), the reallocation
/// (`009f6540`), the element construction of `BSSimpleArray<NiColorA>`
/// (`006a68f0`) and its destruction (`0072ba80`), and the destructor of
/// `BSSimpleArray<NiPoint3>` (`0049f1f0`).
const POINT_ARRAY_CONSTRUCT_BODY: u32 = 0x0049_f990;
const SIMPLE_ARRAY_DESTROY_BODY: u32 = 0x0084_54f0;
const BYTE_ARRAY_CONSTRUCT_BODY: u32 = 0x0069_b940;
const POINT_ARRAY_SET_SIZE: u32 = 0x0069_cea0;
const SIMPLE_ARRAY_REALLOCATE: u32 = 0x009f_6540;
const COLOUR_ARRAY_CONSTRUCT_ELEMENTS: u32 = 0x006a_68f0;
const COLOUR_ARRAY_DESTROY_ELEMENTS: u32 = 0x0072_ba80;
const POINT_ARRAY_DESTRUCTOR: u32 = 0x0049_f1f0;
/// The vtables of the classes built here.
const VTABLE_NAV_MESH_NODE_MAP: u32 = 0x0106_b52c;
const VTABLE_NAV_MESH_CELL_MAP: u32 = 0x0106_b54c;
const VTABLE_NAV_MESH_NODE_MAP_BASE: u32 = 0x0106_b56c;
const VTABLE_NAV_MESH_CELL_MAP_BASE: u32 = 0x0106_b58c;
const VTABLE_POINT_ARRAY: u32 = 0x0101_e114;
const VTABLE_COLOUR_ARRAY: u32 = 0x0106_b5ac;
const VTABLE_BYTE_ARRAY: u32 = 0x0106_b5c0;
/// The name given to the triangle shape (a string literal).
const TRI_SHAPE_NAME: u32 = 0x0106_b510;
/// Hash size of the two maps.
const MAP_HASH_SIZE: u32 = 0x25;

/// The cell's navmesh array (`0070ec90`, +0x64 of the cell; the map names it
/// `DetailedActorPathHandler::GetCurrentNodeIndex` because the linker folded
/// identical code), its length (`00620b80`) and its element accessor
/// (`NavMeshArray::GetNavMeshByIndex`, `00464f60(array, &out, index)`).
const CELL_NAV_MESH_ARRAY: u32 = 0x0070_ec90;
const NAV_MESH_ARRAY_COUNT: u32 = 0x0062_0b80;
const NAV_MESH_ARRAY_GET: u32 = 0x0046_4f60;
/// `NiPointer<NavMesh>` helpers: default construct (`0042fb00`), copy
/// construct (`0042fa20`), assign (`0042f4c0`), destroy (`0042fa40`), equal
/// (`0042ff30`).
const NAV_MESH_POINTER_CONSTRUCT: u32 = 0x0042_fb00;
const NAV_MESH_POINTER_COPY: u32 = 0x0042_fa20;
const NAV_MESH_POINTER_ASSIGN: u32 = 0x0042_f4c0;
const NAV_MESH_POINTER_DESTROY: u32 = 0x0042_fa40;
const NAV_MESH_POINTER_EQUAL: u32 = 0x0042_ff30;
/// `NavMesh` queries: vertex count (`00990a60`), triangle count (`00558200`),
/// "special mesh" test (`00440da0`), triangle by index (`005582f0`), the
/// extra info of an edge (`NavMesh::GetEdgeExtraInfo`, `0068f230(mesh,
/// triangle, edge)`), sphere count (`00691620`) and sphere by index
/// (`00691640`).
const NAV_MESH_VERTEX_COUNT: u32 = 0x0099_0a60;
const NAV_MESH_TRIANGLE_COUNT: u32 = 0x0055_8200;
const NAV_MESH_IS_SPECIAL: u32 = 0x0044_0da0;
const NAV_MESH_TRIANGLE_AT: u32 = 0x0055_82f0;
const NAV_MESH_EDGE_EXTRA_INFO: u32 = 0x0068_f230;
const NAV_MESH_SPHERE_COUNT: u32 = 0x0069_1620;
const NAV_MESH_SPHERE_AT: u32 = 0x0069_1640;
/// Offset of the vertex array in a `NavMesh`.
const NAV_MESH_VERTICES: u32 = 0x28;
/// Triangle queries: flag test (`00691140(triangle, mask)`), edge flag test
/// (`0068f1d0(triangle, edge)`), the border information of an edge
/// (`00691040(triangle, edge, &kind, &flagA, &flagB)`).
const TRIANGLE_TEST_FLAGS: u32 = 0x0069_1140;
const TRIANGLE_EDGE_FLAG: u32 = 0x0068_f1d0;
const TRIANGLE_BORDER_INFO: u32 = 0x0069_1040;
/// Edge extra info queries: "has a loaded navmesh" (`0069dfb0`) and "get the
/// navmesh" (`00690860(info, &out)`), both on the word at +4 of the extra
/// info; the radius of a sphere (`004a7bd0`, the float at +0x18).
const EXTRA_INFO_HAS_NAV_MESH: u32 = 0x0069_dfb0;
const EXTRA_INFO_GET_NAV_MESH: u32 = 0x0069_0860;
const SPHERE_RADIUS: u32 = 0x004a_7bd0;
/// The current cell of the `TES` singleton (`00457070`), the cell's
/// "interior" test (`00425fd0`), its world space (`0054ddd0`), its grid
/// coordinates (`00544c30`, `00544c60`), the world space's cell by grid
/// coordinates (`005875a0(worldspace, x, y)`), and the getter of the
/// singleton (`00552ba0`).
const TES_GET_CURRENT_CELL: u32 = 0x0045_7070;
const CELL_IS_INTERIOR: u32 = 0x0042_5fd0;
const CELL_WORLD_SPACE: u32 = 0x0054_ddd0;
const CELL_GRID_X: u32 = 0x0054_4c30;
const CELL_GRID_Y: u32 = 0x0054_4c60;
const WORLD_SPACE_CELL_BY_GRID: u32 = 0x0058_75a0;
const THE_RENDER_GETTER: u32 = 0x0055_2ba0;
/// `INI` setting lookup helper: `0043d4d0` on the setting collection.
const INI_SETTING_VALUE_ADDRESS: u32 = 0x0043_d4d0;
/// Triangle/edge classification helpers of `fn_006a5da0`.
const TRIANGLE_FLAGS_SPECIAL_KIND: u32 = 0x0069_9790;
const TRIANGLE_HAS_FLAG_0X40: u32 = 0x0069_3b70;
const TRIANGLE_HAS_FLAG_0X10: u32 = 0x0069_3b90;
const TRIANGLE_HAS_FLAG_0X400: u32 = 0x0069_0e70;

// ---------------------------------------------------------------------------
// Small helpers
// ---------------------------------------------------------------------------

/// `NiPointer<T>::operator*` of the slot at `slot`.
fn deref(e: &mut Engine, slot: u32) -> u32 {
    e.call(NI_POINTER_DEREF, &args![slot]).u32()
}

/// `NiPointer(T *)` constructed at `slot` from `value`.
fn pointer_construct(e: &mut Engine, slot: u32, value: u32) {
    e.call(NI_POINTER_CONSTRUCT, &args![slot, value]);
}

/// `NiPointer::operator=(T *)` on the slot at `slot`.
fn pointer_assign(e: &mut Engine, slot: u32, value: u32) {
    e.call(NI_POINTER_ASSIGN, &args![slot, value]);
}

/// `~NiPointer` on the slot at `slot`.
fn pointer_destroy(e: &mut Engine, slot: u32) {
    e.call(NI_POINTER_DESTROY, &args![slot]);
}

/// Copies `words` words from `src` to `dst`.
fn copy_words(e: &mut Engine, dst: u32, src: u32, words: u32) {
    for i in 0..words {
        let word = e.mem.u32(src + 4 * i);
        e.mem.set_u32(dst + 4 * i, word);
    }
}

/// `*dst = *src` for a `NiPoint3`.
fn copy_point(e: &mut Engine, dst: u32, src: u32) {
    copy_words(e, dst, src, 3);
}

/// `*dst = *src` for a `NiColorA`.
fn copy_colour(e: &mut Engine, dst: u32, src: u32) {
    copy_words(e, dst, src, 4);
}

/// `array[index]` of a `BSSimpleArray<NiPoint3>`.
fn point_at(e: &mut Engine, array: u32, index: u32) -> u32 {
    e.call(POINT_ARRAY_AT, &args![array, index]).u32()
}

/// `array[index]` of a `BSSimpleArray<NiColorA>`.
fn colour_at(e: &mut Engine, array: u32, index: u32) -> u32 {
    e.call(COLOUR_ARRAY_AT, &args![array, index]).u32()
}

/// `out = point * factor`.
fn scale_point(e: &mut Engine, point: u32, out: u32, factor: f32) -> u32 {
    e.call(NI_POINT3_SCALE, &args![point, out, factor]).u32()
}

/// `out = a + b`.
fn add_points(e: &mut Engine, a: u32, out: u32, b: u32) -> u32 {
    e.call(NI_POINT3_ADD, &args![a, out, b]).u32()
}

/// `out = a - b`.
fn subtract_points(e: &mut Engine, a: u32, out: u32, b: u32) -> u32 {
    e.call(NI_POINT3_SUBTRACT, &args![a, out, b]).u32()
}

/// `new T[count]` for an element type with a constructor: the allocation size
/// saturates to `0xFFFFFFFF` when `count * element` overflows (the
/// compiler's array-new check); null when the allocation failed, else the
/// elements are constructed by `ctor`.
fn new_array_with_constructor(e: &mut Engine, count: u32, element: u32, ctor: u32) -> u32 {
    let size = count.saturating_mul(element);
    let memory = e.call(OPERATOR_NEW, &args![size]).u32();
    if memory == 0 {
        return 0;
    }
    e.call(
        VECTOR_CONSTRUCTOR_ITERATOR,
        &args![memory, element, count, ctor],
    );
    memory
}

/// A style colour of the render, by index.
fn style_colour(this: u32, index: u32) -> u32 {
    this + COLOUR_VALUES + 0x10 * index
}

// ---------------------------------------------------------------------------
// Construction and destruction
// ---------------------------------------------------------------------------

/// Which value a component of a style colour has.
#[derive(Clone, Copy)]
enum Component {
    Zero,
    One,
    /// A `float` constant of the exe's data.
    Global(u32),
}

use Component::{Global as G, One as O, Zero as Z};

/// The 17 style colours as `NiColorA(r, g, b, a)` arguments, in slot order.
const STYLE_COLOURS: [[Component; 4]; 17] = [
    [O, Z, Z, O],
    [O, Z, Z, O],
    [Z, O, Z, O],
    [Z, O, Z, O],
    [Z, O, G(HALF), O],
    [Z, G(HALF), O, O],
    [O, O, Z, O],
    [Z, Z, O, O],
    [G(HALF), G(HALF), O, O],
    [O, O, O, O],
    [Z, Z, Z, O],
    [O, Z, O, O],
    [G(THREE_QUARTERS), G(QUARTER), O, O],
    [O, G(HALF), Z, O],
    [O, G(HALF), G(HALF), O],
    [O, Z, O, O],
    [G(THREE_QUARTERS), Z, Z, O],
];

/// Alpha of the edge-and-vertex material of each style slot: the `float`
/// constants `0106b4fc` (0.5), `0106b500` (1.0), `0106b508` (0.75) and
/// `0106b50c` (0.25).
const EDGE_ALPHA_SOURCES: [u32; 17] = [
    0x0106_b4fc,
    0x0106_b500,
    0x0106_b4fc,
    0x0106_b500,
    0x0106_b500,
    0x0106_b500,
    0x0106_b4fc,
    0x0106_b4fc,
    0x0106_b4fc,
    0x0106_b4fc,
    0x0106_b508,
    0x0106_b50c,
    0x0106_b50c,
    0x0106_b4fc,
    0x0106_b4fc,
    0x0106_b4fc,
    0x0106_b4fc,
];

/// Alpha of the triangle material of each style slot; slot 5 is the one the
/// game never initialises (`None`: its stack slot is read as it is).
const TRIANGLE_ALPHA_SOURCES: [Option<u32>; 17] = [
    Some(0x0106_b4f0),
    Some(0x0106_b4f4),
    Some(0x0106_b4f0),
    Some(0x0106_b4f4),
    Some(0x0106_b4f0),
    None,
    Some(0x0106_b4f0),
    Some(0x0106_b4f0),
    Some(0x0106_b4f0),
    Some(0x0106_b500),
    Some(0x0106_b4f0),
    Some(0x0106_b50c),
    Some(0x0106_b50c),
    Some(0x0106_b4f0),
    Some(0x0106_b4f0),
    Some(0x0106_b4f0),
    Some(0x0106_b4f0),
];

/// A new `NiMaterialProperty`; null when the allocation failed.
fn new_material(e: &mut Engine) -> u32 {
    let memory = e.call(NI_ALLOC, &args![0x4cu32]).u32();
    if memory == 0 {
        return 0;
    }
    e.call(NI_MATERIAL_PROPERTY_CONSTRUCT, &args![memory]).u32()
}

/// Sets the `NiPointer<NiMaterialProperty>` at `slot` to a new material whose
/// ambient colour is white and whose alpha is `alpha`.
fn set_up_material(e: &mut Engine, slot: u32, alpha: f32) {
    let material = new_material(e);
    pointer_assign(e, slot, material);
    let target = deref(e, slot);
    e.call(NI_MATERIAL_SET_AMBIENT, &args![target, COLOUR_WHITE]);
    let target = deref(e, slot);
    e.call(NI_MATERIAL_SET_ALPHA, &args![target, alpha]);
}

/// A new `BSShaderNoLightingProperty`; null when the allocation failed.
fn new_no_lighting_property(e: &mut Engine) -> u32 {
    let memory = e.call(NI_ALLOC, &args![0x80u32]).u32();
    if memory == 0 {
        return 0;
    }
    e.call(BS_SHADER_NO_LIGHTING_CONSTRUCT, &args![memory])
        .u32()
}

// Translated from 006a2a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshRender::NavMeshRender` (Xbox PDB): sets up the empty node and
/// geometry slots, the two maps (hash size 0x25), the 17 style colours and
/// the 17 vertex shapes with their materials, the alpha property and the
/// local-map material.
///
/// The triangle-material alpha of style slot 5 is never written by the game
/// (its local table has a hole there); it is read from the zeroed stack block
/// as it stands.
pub fn nav_mesh_render_nav_mesh_render(
    e: &mut Engine,
    this: Ptr<NavMeshRender>,
) -> Ptr<NavMeshRender> {
    let this = this.addr();
    // Local tables: 17 edge alphas, 17 triangle alphas, a colour scratch
    // block, a `NiPointer` slot.
    e.with_stack(0xa0, |e, locals| {
        let locals = locals.addr();
        let edge_alphas = locals;
        let triangle_alphas = locals + 0x44;
        let colour_scratch = locals + 0x88;
        let shader_slot = locals + 0x98;

        pointer_construct(e, this, 0);
        pointer_construct(e, this + 4, 0);
        pointer_construct(e, this + 8, 0);
        for base in [VERT_POINTS, EDGE_AND_VERTEX_MATERIALS, TRIANGLE_MATERIALS] {
            e.call(
                EH_VECTOR_CONSTRUCTOR_ITERATOR,
                &args![
                    this + base,
                    4u32,
                    STYLE_COUNT,
                    NI_POINTER_DEFAULT_CONSTRUCT,
                    NI_POINTER_DESTROY
                ],
            );
        }
        e.call(
            VECTOR_CONSTRUCTOR_ITERATOR,
            &args![
                this + COLOUR_VALUES,
                0x10u32,
                STYLE_COUNT,
                NI_COLOR_A_DEFAULT_CONSTRUCT
            ],
        );
        pointer_construct(e, this + 0x1e8, 0);
        pointer_construct(e, this + 0x1ec, 0);
        fn_006a6290(e, Ptr::new(this + NAV_MESH_NODES), MAP_HASH_SIZE);
        fn_006a62c0(e, Ptr::new(this + NAV_MESH_CELL_MAP), MAP_HASH_SIZE);
        e.mem.set_u8(this + 0x210, 0);

        for (index, components) in STYLE_COLOURS.iter().enumerate() {
            let mut rgba = [0f32; 4];
            for (value, component) in rgba.iter_mut().zip(components) {
                *value = match *component {
                    Component::Zero => 0.0,
                    Component::One => 1.0,
                    Component::Global(address) => e.global::<f32>(address),
                };
            }
            let made = e
                .call(
                    NI_COLOR_A_CONSTRUCT,
                    &args![colour_scratch, rgba[0], rgba[1], rgba[2], rgba[3]],
                )
                .u32();
            copy_colour(e, style_colour(this, index as u32), made);
        }

        for (index, source) in EDGE_ALPHA_SOURCES.iter().enumerate() {
            let alpha = e.global::<f32>(*source);
            e.mem.set_f32(edge_alphas + 4 * index as u32, alpha);
        }
        for (index, source) in TRIANGLE_ALPHA_SOURCES.iter().enumerate() {
            if let Some(source) = source {
                let alpha = e.global::<f32>(*source);
                e.mem.set_f32(triangle_alphas + 4 * index as u32, alpha);
            }
        }

        // The alpha property.
        let memory = e.call(NI_ALLOC, &args![0x1cu32]).u32();
        let alpha_property = if memory == 0 {
            0
        } else {
            e.call(NI_ALPHA_PROPERTY_CONSTRUCT, &args![memory]).u32()
        };
        pointer_assign(e, this + 0x1e8, alpha_property);
        let alpha_property = deref(e, this + 0x1e8);
        e.call(NI_ALPHA_PROPERTY_SET_BLENDING, &args![alpha_property, 1u32]);

        for index in 0..STYLE_COUNT {
            let edge_slot = this + EDGE_AND_VERTEX_MATERIALS + 4 * index;
            let edge_alpha = e.mem.f32(edge_alphas + 4 * index);
            set_up_material(e, edge_slot, edge_alpha);
            let triangle_slot = this + TRIANGLE_MATERIALS + 4 * index;
            let triangle_alpha = e.mem.f32(triangle_alphas + 4 * index);
            set_up_material(e, triangle_slot, triangle_alpha);

            let point_size = e.global::<f32>(POINT_SHAPE_SIZE);
            let shape = e
                .call(
                    MAKE_TRI_POINT,
                    &args![point_size, style_colour(this, index), 1u32],
                )
                .u32();
            let shape_slot = this + VERT_POINTS + 4 * index;
            pointer_assign(e, shape_slot, shape);
            let material = deref(e, edge_slot);
            let target = deref(e, shape_slot);
            e.call(NI_AV_OBJECT_ATTACH_PROPERTY, &args![target, material]);

            let property = new_no_lighting_property(e);
            pointer_construct(e, shader_slot, property);
            let property = deref(e, shader_slot);
            e.call(BS_SHADER_PROPERTY_SET_FLAG, &args![property, 0x20u32, 0u32]);
            let target = deref(e, shape_slot);
            e.call(NI_AV_OBJECT_REMOVE_PROPERTY, &args![target, 3u32]);
            let property = deref(e, shader_slot);
            let target = deref(e, shape_slot);
            e.call(NI_AV_OBJECT_ATTACH_PROPERTY, &args![target, property]);
            pointer_destroy(e, shader_slot);
        }

        let alpha = e.global::<f32>(HALF);
        set_up_material(e, this + 0x1ec, alpha);
    });
    Ptr::new(this)
}

// Translated from 006a2920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshRender::InitSDM` (Xbox PDB): creates the singleton (`TheRender`).
pub fn nav_mesh_render_init_sdm(e: &mut Engine) {
    let memory = e.call(OPERATOR_NEW, &args![0x214u32]).u32();
    let render = if memory == 0 {
        0
    } else {
        nav_mesh_render_nav_mesh_render(e, Ptr::new(memory)).addr()
    };
    e.set_global(THE_RENDER, render);
}

// Translated from 006a29a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Shuts the singleton down (unnamed in the map; the counterpart of
/// `InitSDM`): runs the scalar deleting destructor on it and clears the
/// global.
pub fn fn_006a29a0(e: &mut Engine) {
    let render: u32 = e.global(THE_RENDER);
    if render != 0 {
        fn_006a29f0(e, Ptr::new(render), 1);
    }
    e.set_global(THE_RENDER, 0u32);
}

// Translated from 006a29f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The scalar deleting destructor of `NavMeshRender` (unnamed in the map):
/// the destructor body, then `operator delete` when bit 0 of `flags` is set.
pub fn fn_006a29f0(e: &mut Engine, this: Ptr<NavMeshRender>, flags: u32) -> Ptr<NavMeshRender> {
    fn_006a3580(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 006a3580 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor body of `NavMeshRender` (unnamed in the map): releases the
/// vertex shapes and both material arrays, destroys the two maps and
/// releases the remaining pointers.
pub fn fn_006a3580(e: &mut Engine, this: Ptr<NavMeshRender>) {
    let this = this.addr();
    for index in 0..STYLE_COUNT {
        pointer_assign(e, this + EDGE_AND_VERTEX_MATERIALS + 4 * index, 0);
        pointer_assign(e, this + TRIANGLE_MATERIALS + 4 * index, 0);
        pointer_assign(e, this + VERT_POINTS + 4 * index, 0);
    }
    fn_006a6550(e, Ptr::new(this + NAV_MESH_CELL_MAP));
    fn_006a6450(e, Ptr::new(this + NAV_MESH_NODES));
    pointer_destroy(e, this + 0x1ec);
    pointer_destroy(e, this + 0x1e8);
    for base in [TRIANGLE_MATERIALS, EDGE_AND_VERTEX_MATERIALS, VERT_POINTS] {
        e.call(
            EH_VECTOR_DESTRUCTOR_ITERATOR,
            &args![this + base, 4u32, STYLE_COUNT, NI_POINTER_DESTROY],
        );
    }
    pointer_destroy(e, this + 8);
    pointer_destroy(e, this + 4);
    pointer_destroy(e, this);
}

// ---------------------------------------------------------------------------
// Drawing switches
// ---------------------------------------------------------------------------

/// Updates `spRootDrawNode` after a change: `NiUpdateData(0, 0, 0)` built in
/// `update_data`, then the node's update through it.
fn update_root(e: &mut Engine, update_data: u32) {
    e.call(
        NI_UPDATE_DATA_CONSTRUCT,
        &args![update_data, 0.0f32, 0u32, 0u32],
    );
    let root = deref(e, ROOT_DRAW_NODE);
    e.call(NI_AV_OBJECT_UPDATE, &args![root, update_data]);
}

// Translated from 006a36e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshRender::SetDraw` (Xbox PDB): with `draw` the root node's
/// properties are updated, then the node is updated through an
/// `NiUpdateData`; without it the root node's children are first cleared
/// (`004def90`), it is updated the same way, and the cell map is emptied
/// (`RemoveAll`). Stores the flag in `bDraw`.
pub fn nav_mesh_render_set_draw(e: &mut Engine, this: Ptr<NavMeshRender>, draw: bool) {
    e.with_stack(0x20, |e, update_data| {
        let update_data = update_data.addr();
        if draw {
            let root = deref(e, ROOT_DRAW_NODE);
            e.call(NI_AV_OBJECT_UPDATE_PROPERTIES, &args![root]);
            update_root(e, update_data);
        } else {
            let root = deref(e, ROOT_DRAW_NODE);
            e.call(NI_NODE_CLEAR_CHILDREN, &args![root]);
            let root = deref(e, ROOT_DRAW_NODE);
            e.call(NI_AV_OBJECT_UPDATE_PROPERTIES, &args![root]);
            update_root(e, update_data);
            e.call(NI_T_MAP_REMOVE_ALL, &args![this.addr() + NAV_MESH_CELL_MAP]);
        }
    });
    e.set(this, NavMeshRender::bDraw, draw);
}

// Translated from 006a5ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshRender::SetTransparentOverLayMode` (Xbox PDB): sets the alpha of
/// eleven of the style materials to the "transparent" values (`0106b4ec`,
/// `0106b4f0`, `0106b4f8`, `0106b504`) or back to the opaque ones
/// (`0106b4f0`, `0106b500`, `0106b4fc`, `0106b508`), then sets shader flag
/// 0x1F below the root node to `!enable` (`SetFlagRecurse`).
pub fn nav_mesh_render_set_transparent_over_lay_mode(
    e: &mut Engine,
    this: Ptr<NavMeshRender>,
    enable: bool,
) {
    let this = this.addr();
    // (slot offset in the render, constant when on, constant when off)
    const SLOTS: [(u32, u32, u32); 11] = [
        (0x94, 0x0106_b4ec, 0x0106_b4f0),
        (0xb0, 0x0106_b4ec, 0x0106_b4f0),
        (0xb4, 0x0106_b4ec, 0x0106_b4f0),
        (0xa4, 0x0106_b4f0, 0x0106_b500),
        (0xa8, 0x0106_b4f0, 0x0106_b500),
        (0x50, 0x0106_b4f8, 0x0106_b4fc),
        (0x68, 0x0106_b4f8, 0x0106_b4fc),
        (0x58, 0x0106_b4f8, 0x0106_b4fc),
        (0x6c, 0x0106_b4f8, 0x0106_b4fc),
        (0x74, 0x0106_b4f8, 0x0106_b4fc),
        (0x78, 0x0106_b504, 0x0106_b508),
    ];
    for (slot, on, off) in SLOTS {
        let alpha = e.global::<f32>(if enable { on } else { off });
        let material = deref(e, this + slot);
        e.call(NI_MATERIAL_SET_ALPHA, &args![material, alpha]);
    }
    let root = deref(e, ROOT_DRAW_NODE);
    e.call(
        SHADER_PROPERTY_SET_FLAG_RECURSE,
        &args![root, 0x1fu32, !enable],
    );
}

// Translated from 006a5bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshRender::RemoveDrawOnlyNavMeshes` (Xbox PDB): if the cell has a
/// node in the cell map, detaches it from the root node (`DetachChild`),
/// updates the root, and removes the cell from the map.
pub fn nav_mesh_render_remove_draw_only_nav_meshes(
    e: &mut Engine,
    this: Ptr<NavMeshRender>,
    cell: Ptr,
) {
    let cell_map = this.addr() + NAV_MESH_CELL_MAP;
    e.with_stack(0x20, |e, slots| {
        let node_slot = slots.addr();
        let update_data = slots.addr() + 0x10;
        pointer_construct(e, node_slot, 0);
        let found = e
            .call(NI_T_MAP_GET_AT, &args![cell_map, cell, node_slot])
            .bool();
        if found {
            let root = deref(e, ROOT_DRAW_NODE);
            let node = deref(e, node_slot);
            e.vcall(root, SLOT_DETACH_CHILD, &args![node]);
            update_root(e, update_data);
            let root = deref(e, ROOT_DRAW_NODE);
            e.call(NI_AV_OBJECT_UPDATE_PROPERTIES, &args![root]);
            e.call(NI_T_MAP_REMOVE_AT, &args![cell_map, cell]);
        }
        pointer_destroy(e, node_slot);
    });
}

// Translated from 006a61b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Detaches the selected-path and line-of-sight geometry (+0x04 and +0x08,
/// unnamed in the map) from their parents, then releases both.
pub fn fn_006a61b0(e: &mut Engine, this: Ptr<NavMeshRender>) {
    let this = this.addr();
    for slot in [this + 4, this + 8] {
        if deref(e, slot) != 0 {
            let geometry = deref(e, slot);
            if e.call(NODE_PARENT, &args![geometry]).u32() != 0 {
                let geometry = deref(e, slot);
                let parent = e.call(NODE_PARENT, &args![geometry]).u32();
                let geometry = deref(e, slot);
                e.vcall(parent, SLOT_DETACH_CHILD, &args![geometry]);
            }
        }
    }
    pointer_assign(e, this + 4, 0);
    pointer_assign(e, this + 8, 0);
}

// Translated from 006a5ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Redraws the navmeshes around the player (unnamed in the map): switches the
/// drawing off, then in the current cell draws, for an interior cell, that
/// cell, and for an exterior one every cell of the loaded grid (the
/// `(grid - 1) / 2` cells around it, through the world space), and switches
/// the drawing on again.
pub fn fn_006a5ca0(e: &mut Engine, this: Ptr<NavMeshRender>) {
    nav_mesh_render_set_draw(e, this, false);
    let tes: u32 = e.global(TES_SINGLETON);
    let cell = e.call(TES_GET_CURRENT_CELL, &args![tes]).u32();
    if cell != 0 {
        if !e.call(CELL_IS_INTERIOR, &args![cell]).bool() {
            let setting = e
                .call(INI_SETTING_VALUE_ADDRESS, &args![GRID_SETTING])
                .u32();
            let grid = e.mem.i32(setting);
            let radius = grid.wrapping_sub(1) / 2;
            let mut x = e.call(CELL_GRID_X, &args![cell]).i32().wrapping_sub(radius);
            while x <= e.call(CELL_GRID_X, &args![cell]).i32().wrapping_add(radius) {
                let mut y = e.call(CELL_GRID_Y, &args![cell]).i32().wrapping_sub(radius);
                while y <= e.call(CELL_GRID_Y, &args![cell]).i32().wrapping_add(radius) {
                    let world_space = e.call(CELL_WORLD_SPACE, &args![cell]).u32();
                    let grid_cell = e
                        .call(WORLD_SPACE_CELL_BY_GRID, &args![world_space, x, y])
                        .u32();
                    nav_mesh_render_add_draw_only_nav_meshes(e, this, Ptr::new(grid_cell), false);
                    y = y.wrapping_add(1);
                }
                x = x.wrapping_add(1);
            }
        } else {
            let render = e.call(THE_RENDER_GETTER, &args![]).u32();
            nav_mesh_render_add_draw_only_nav_meshes(e, Ptr::new(render), Ptr::new(cell), false);
        }
    }
    nav_mesh_render_set_draw(e, this, true);
}

// ---------------------------------------------------------------------------
// Triangle helpers
// ---------------------------------------------------------------------------

// Translated from 006a5af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Height step of a cover box (unnamed in the map): `min(kind, 15) * 16.0`,
/// returned in `ST0` as a `float`.
pub fn fn_006a5af0(e: &mut Engine, kind: u32) -> f32 {
    let kind = kind.min(15);
    let step = e.global::<f32>(BOX_HEIGHT_STEP);
    (f64::from(kind) * f64::from(step)) as f32
}

// Translated from 006a5b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the three vertex indices of a navmesh triangle (`this` +0x00, +0x02,
/// +0x04) into the three `u16` outputs (unnamed in the map).
pub fn fn_006a5b30(e: &mut Engine, this: Ptr, first: Ptr, second: Ptr, third: Ptr) {
    let this = this.addr();
    let value = e.mem.u16(this);
    e.mem.set_u16(first.addr(), value);
    let value = e.mem.u16(this + 2);
    e.mem.set_u16(second.addr(), value);
    let value = e.mem.u16(this + 4);
    e.mem.set_u16(third.addr(), value);
}

// Translated from 006a5b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the three neighbour triangle indices of a navmesh triangle (`this`
/// +0x06, +0x08, +0x0A) into the three `u16` outputs (unnamed in the map).
pub fn fn_006a5b70(e: &mut Engine, this: Ptr, first: Ptr, second: Ptr, third: Ptr) {
    let this = this.addr();
    let value = e.mem.u16(this + 6);
    e.mem.set_u16(first.addr(), value);
    let value = e.mem.u16(this + 8);
    e.mem.set_u16(second.addr(), value);
    let value = e.mem.u16(this + 0xa);
    e.mem.set_u16(third.addr(), value);
}

// Translated from 006a5ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tests flag 0x200 of a triangle (unnamed in the map).
pub fn fn_006a5ec0(e: &mut Engine, triangle: Ptr) -> bool {
    e.call(TRIANGLE_TEST_FLAGS, &args![triangle, 0x200u32])
        .bool()
}

// Translated from 006a5da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The style slot (colour index) a triangle is drawn in, 0 for an ordinary
/// triangle (unnamed in the map; `this` is not read). Reads the triangle's
/// flag word (+0x0C) through its flag tests: a non-zero kind (bits 0xE000)
/// gives slot 12 with flag 0x100 and slot 11 without it; flag 0x40 gives 5
/// (with flag 0x400), 8 (with flag 0x200) or 13; without flag 0x40 the first
/// of flag 0x200 (7), flag 0x10 (14), flag 0x400 (4), flag 0x1000 (9) and flag
/// 0x80000000 (16) that is set decides.
pub fn fn_006a5da0(e: &mut Engine, _this: Ptr<NavMeshRender>, triangle: Ptr) -> u32 {
    let mut style = 0;
    let kind = e.call(TRIANGLE_FLAGS_SPECIAL_KIND, &args![triangle]).u16();
    if kind != 0 {
        style = if e
            .call(TRIANGLE_TEST_FLAGS, &args![triangle, 0x100u32])
            .bool()
        {
            12
        } else {
            11
        };
    } else if e.call(TRIANGLE_HAS_FLAG_0X40, &args![triangle]).bool() {
        style = if e.call(TRIANGLE_HAS_FLAG_0X400, &args![triangle]).bool() {
            5
        } else if fn_006a5ec0(e, triangle) {
            8
        } else {
            13
        };
    } else if fn_006a5ec0(e, triangle) {
        style = 7;
    } else if e.call(TRIANGLE_HAS_FLAG_0X10, &args![triangle]).bool() {
        style = 14;
    } else if e.call(TRIANGLE_HAS_FLAG_0X400, &args![triangle]).bool() {
        style = 4;
    } else if e
        .call(TRIANGLE_TEST_FLAGS, &args![triangle, 0x1000u32])
        .bool()
    {
        style = 9;
    } else if e
        .call(TRIANGLE_TEST_FLAGS, &args![triangle, 0x8000_0000u32])
        .bool()
    {
        style = 16;
    }
    style
}

// ---------------------------------------------------------------------------
// Containers
// ---------------------------------------------------------------------------

// Translated from 006a6370 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMapBase<NavMesh *, NiPointer<NiNode>>` constructor (unnamed in the
/// map): vtable, hash size, no items, and a zeroed bucket array of
/// `hash_size` words.
pub fn fn_006a6370(e: &mut Engine, this: Ptr, hash_size: u32) -> Ptr {
    let this_address = this.addr();
    e.mem.set_u32(this_address, VTABLE_NAV_MESH_NODE_MAP_BASE);
    e.mem.set_u32(this_address + 4, hash_size);
    e.mem.set_u32(this_address + 0xc, 0);
    let buckets = e
        .call(NI_ALLOC_ARRAY, &args![hash_size.wrapping_shl(2)])
        .u32();
    e.mem.set_u32(this_address + 8, buckets);
    e.call(MEMSET, &args![buckets, 0u32, hash_size.wrapping_shl(2)]);
    this
}

// Translated from 006a64e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMapBase<TESObjectCELL *, NiPointer<NiNode>>` constructor (unnamed
/// in the map): same as [`fn_006a6370`] with its own vtable.
pub fn fn_006a64e0(e: &mut Engine, this: Ptr, hash_size: u32) -> Ptr {
    let this_address = this.addr();
    e.mem.set_u32(this_address, VTABLE_NAV_MESH_CELL_MAP_BASE);
    e.mem.set_u32(this_address + 4, hash_size);
    e.mem.set_u32(this_address + 0xc, 0);
    let buckets = e
        .call(NI_ALLOC_ARRAY, &args![hash_size.wrapping_shl(2)])
        .u32();
    e.mem.set_u32(this_address + 8, buckets);
    e.call(MEMSET, &args![buckets, 0u32, hash_size.wrapping_shl(2)]);
    this
}

// Translated from 006a6290 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMap<NavMesh *, NiPointer<NiNode>>` constructor (unnamed in the
/// map): the base constructor, then the `NiTMap` vtable.
pub fn fn_006a6290(e: &mut Engine, this: Ptr, hash_size: u32) -> Ptr {
    fn_006a6370(e, this, hash_size);
    e.mem.set_u32(this.addr(), VTABLE_NAV_MESH_NODE_MAP);
    this
}

// Translated from 006a62c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMap<TESObjectCELL *, NiPointer<NiNode>>` constructor (unnamed in
/// the map).
pub fn fn_006a62c0(e: &mut Engine, this: Ptr, hash_size: u32) -> Ptr {
    fn_006a64e0(e, this, hash_size);
    e.mem.set_u32(this.addr(), VTABLE_NAV_MESH_CELL_MAP);
    this
}

// Translated from 006a6450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<NavMesh *, NiPointer<NiNode>>`'s destructor body (unnamed in the
/// map): vtable, `RemoveAll`, then the `NiTMapBase` destructor.
pub fn fn_006a6450(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_NAV_MESH_NODE_MAP);
    e.call(NI_T_MAP_REMOVE_ALL, &args![this]);
    e.call(NAV_MESH_NODE_MAP_BASE_DESTROY, &args![this]);
}

// Translated from 006a6550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<TESObjectCELL *, NiPointer<NiNode>>`'s destructor body (unnamed in
/// the map).
pub fn fn_006a6550(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_NAV_MESH_CELL_MAP);
    e.call(NI_T_MAP_REMOVE_ALL, &args![this]);
    e.call(NAV_MESH_CELL_MAP_BASE_DESTROY, &args![this]);
}

// Translated from 006a6310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<NavMesh *, NiPointer<NiNode>>::scalar deleting destructor`
/// (Xbox PDB): the destructor body, then `operator delete` when bit 0 of
/// `flags` is set.
pub fn ni_t_map_nav_mesh_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006a6450(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 006a6340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<TESObjectCELL *, NiPointer<NiNode>>::scalar deleting destructor`
/// (Xbox PDB).
pub fn ni_t_map_cell_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006a6550(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 006a67f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<..NavMesh *, NiPointer<NiNode>..>::scalar deleting destructor`
/// (Xbox PDB): the `NiTMapBase` destructor body (`006a64b0`), then
/// `operator delete` when bit 0 of `flags` is set.
pub fn ni_t_map_base_nav_mesh_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(NAV_MESH_NODE_MAP_BASE_DESTROY, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 006a6820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<..TESObjectCELL *, NiPointer<NiNode>..>::scalar deleting
/// destructor` (Xbox PDB).
pub fn ni_t_map_base_cell_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(NAV_MESH_CELL_MAP_BASE_DESTROY, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 006a62f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Address of element `index` of a `BSSimpleArray<unsigned char>` (unnamed in
/// the map): the buffer pointer at +4 plus the index.
pub fn fn_006a62f0(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    e.mem.u32(this.addr() + 4).wrapping_add(index)
}

// Translated from 006a65e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NiPoint3,1024>::BSSimpleArray<NiPoint3,1024>` (Xbox PDB):
/// vtable, then the shared constructor body with size and capacity `count`.
pub fn bs_simple_array_ni_point3_construct(e: &mut Engine, this: Ptr, count: u32) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE_POINT_ARRAY);
    e.call(POINT_ARRAY_CONSTRUCT_BODY, &args![this, count, count]);
    this
}

// Translated from 006a69b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor body of `BSSimpleArray<NiColorA,1024>` (unnamed in the
/// map): empty buffer, then room for `max(capacity, size)` elements (the
/// allocation is the vtable's slot 1) of which `size` are constructed.
pub fn fn_006a69b0(e: &mut Engine, this: Ptr, capacity: u32, size: u32) {
    let this_address = this.addr();
    e.mem.set_u32(this_address + 4, 0);
    e.mem.set_u32(this_address + 8, 0);
    e.mem.set_u32(this_address + 0xc, 0);
    let capacity = capacity.max(size);
    if capacity > 0 {
        let buffer = e.vcall(this_address, 4, &args![capacity]).u32();
        e.mem.set_u32(this_address + 4, buffer);
        e.mem.set_u32(this_address + 0xc, capacity);
    }
    if size > 0 {
        let buffer = e.mem.u32(this_address + 4);
        e.call(COLOUR_ARRAY_CONSTRUCT_ELEMENTS, &args![this, buffer, size]);
        e.mem.set_u32(this_address + 8, size);
    }
}

// Translated from 006a6610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NiColorA,1024>::BSSimpleArray<NiColorA,1024>`-style
/// constructor (unnamed in the map for this instance; the map names the
/// neighbour `BSSimpleArray<NiPoint3_1024>`): vtable, then `006a69b0` with
/// size and capacity `count`.
pub fn bs_simple_array_ni_color_a_construct(e: &mut Engine, this: Ptr, count: u32) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE_COLOUR_ARRAY);
    fn_006a69b0(e, this, count, count);
    this
}

// Translated from 006a67a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSSimpleArray<unsigned char,1024>` constructor (unnamed in the map):
/// vtable, then the shared constructor body with size and capacity `count`.
pub fn bs_simple_array_u8_construct(e: &mut Engine, this: Ptr, count: u32) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE_BYTE_ARRAY);
    e.call(BYTE_ARRAY_CONSTRUCT_BODY, &args![this, count, count]);
    this
}

// Translated from 006a6640 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor body of `BSSimpleArray<NiColorA,1024>` (unnamed in the
/// map): vtable, then the shared destructor body (frees the buffer).
pub fn fn_006a6640(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_COLOUR_ARRAY);
    e.call(SIMPLE_ARRAY_DESTROY_BODY, &args![this, 1u32]);
}

// Translated from 006a67d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor body of `BSSimpleArray<unsigned char,1024>` (unnamed in the
/// map).
pub fn fn_006a67d0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_BYTE_ARRAY);
    e.call(SIMPLE_ARRAY_DESTROY_BODY, &args![this, 1u32]);
}

// Translated from 006a6850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NiColorA,1024>::scalar deleting destructor` (Xbox PDB).
pub fn bs_simple_array_ni_color_a_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_006a6640(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 006a6880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<unsigned char,1024>::scalar deleting destructor` (Xbox
/// PDB).
pub fn bs_simple_array_u8_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006a67d0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 006a6660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NiColorA,1024>::SetSize`-style resize (unnamed in the map;
/// 16-byte elements): `new_size == 0` empties the array (`008454f0`); a
/// larger size reallocates (the first allocation through vtable slot 1, later
/// ones through `009f6540`) and constructs the new elements; a smaller one
/// destroys the tail and, with `shrink` and a size at most a quarter of the
/// capacity, reallocates to fit; an equal one constructs nothing.
pub fn fn_006a6660(e: &mut Engine, this: Ptr, new_size: u32, shrink: bool) {
    let this_address = this.addr();
    if new_size == 0 {
        e.call(SIMPLE_ARRAY_DESTROY_BODY, &args![this, shrink]);
        return;
    }
    let capacity = e.mem.u32(this_address + 0xc);
    let size = e.mem.u32(this_address + 8);
    if new_size > capacity {
        if capacity == 0 {
            let buffer = e.vcall(this_address, 4, &args![new_size]).u32();
            e.mem.set_u32(this_address + 4, buffer);
            e.mem.set_u32(this_address + 0xc, new_size);
        } else {
            e.call(SIMPLE_ARRAY_REALLOCATE, &args![this, new_size, size]);
            e.mem.set_u32(this_address + 0xc, new_size);
        }
        let buffer = e.mem.u32(this_address + 4);
        let size = e.mem.u32(this_address + 8);
        e.call(
            COLOUR_ARRAY_CONSTRUCT_ELEMENTS,
            &args![
                this,
                size.wrapping_shl(4).wrapping_add(buffer),
                new_size.wrapping_sub(size)
            ],
        );
        e.mem.set_u32(this_address + 8, new_size);
    } else if new_size < size {
        let buffer = e.mem.u32(this_address + 4);
        e.call(
            COLOUR_ARRAY_DESTROY_ELEMENTS,
            &args![
                this,
                new_size.wrapping_shl(4).wrapping_add(buffer),
                size - new_size
            ],
        );
        e.mem.set_u32(this_address + 8, new_size);
        if shrink && new_size <= e.mem.u32(this_address + 0xc) >> 2 {
            e.call(SIMPLE_ARRAY_REALLOCATE, &args![this, new_size, new_size]);
            e.mem.set_u32(this_address + 0xc, new_size);
        }
    } else {
        let buffer = e.mem.u32(this_address + 4);
        e.call(
            COLOUR_ARRAY_CONSTRUCT_ELEMENTS,
            &args![
                this,
                size.wrapping_shl(4).wrapping_add(buffer),
                new_size - size
            ],
        );
        e.mem.set_u32(this_address + 8, new_size);
    }
}

// ---------------------------------------------------------------------------
// AddDrawOnlyNavMeshes
// ---------------------------------------------------------------------------

/// Size of the block that stands for the locals of `AddDrawOnlyNavMeshes`.
const FRAME_SIZE: u32 = 0x300;
// `NiPointer` slots (4 bytes each).
/// The cell's `NiPointer<NiNode>`.
const F_NODE: u32 = 0x000;
/// The current `NiPointer<NavMesh>` and the temporary the array getter fills.
const F_MESH: u32 = 0x004;
const F_MESH_TEMP: u32 = 0x008;
/// The navmesh on the other side of an edge, and the one found while
/// searching back across it.
const F_OTHER_MESH: u32 = 0x00c;
const F_BACK_EDGE_MESH: u32 = 0x010;
/// The first set of lines, its shader property, the second set of lines, the
/// triangle shape and its shader property.
const F_LINES_B: u32 = 0x014;
const F_SHADER_B: u32 = 0x018;
const F_LINES_C: u32 = 0x01c;
const F_TRI_SHAPE: u32 = 0x020;
const F_SHADER_A: u32 = 0x024;
/// A sphere and its alpha property.
const F_SPHERE: u32 = 0x028;
const F_SPHERE_ALPHA: u32 = 0x02c;
/// The `NiFixedString` temporary for the triangle shape's name.
const F_NAME: u32 = 0x030;
/// The by-value `NiPointer<NiNode>` argument of `NiTMap::SetAt`.
const F_NODE_COPY: u32 = 0x034;
// Arrays (0x10 bytes each): vertices and colours of the triangle shape (A),
// points, flags and colours of the first set of lines (B) and of the second
// set (C).
const F_POINTS_A: u32 = 0x040;
const F_COLOURS_A: u32 = 0x050;
const F_POINTS_B: u32 = 0x060;
const F_FLAGS_B: u32 = 0x070;
const F_COLOURS_B: u32 = 0x080;
const F_POINTS_C: u32 = 0x090;
const F_FLAGS_C: u32 = 0x0a0;
const F_COLOURS_C: u32 = 0x0b0;
// Three `u16` each: the vertex indices of the triangle, its neighbour
// triangles, the neighbours of the triangle across a navmesh edge (read and
// not used), the neighbours of the neighbour triangle, and the vertex indices
// of the triangle found across an edge.
const F_CORNERS: u32 = 0x0c0;
const F_NEIGHBOURS: u32 = 0x0c8;
const F_OTHER_NEIGHBOURS: u32 = 0x0d0;
const F_BACK_NEIGHBOURS: u32 = 0x0d8;
const F_OTHER_CORNERS: u32 = 0x0e0;
/// The border information: a `u16` kind, then two bytes (flags).
const F_BORDER: u32 = 0x0e8;
// `NiPoint3`s (0x10 bytes each): the corners of the triangle (three), the
// corners of the triangle across (three), the two ends of a connection line,
// eight scratch points, the direction step, four box ends, the translation
// being built.
const F_CORNER_POINTS: u32 = 0x0f0;
const F_OTHER_CORNER_POINTS: u32 = 0x120;
const F_LINE_START: u32 = 0x150;
const F_LINE_END: u32 = 0x160;
const F_SCRATCH: u32 = 0x170;
const F_DIRECTION: u32 = 0x1f0;
const F_BOX_POINTS: u32 = 0x200;
const F_TRANSLATE: u32 = 0x240;
/// The colour of a sphere, and the `NiUpdateData` built at the end.
const F_SPHERE_COLOUR: u32 = 0x250;
const F_UPDATE: u32 = 0x260;

/// The counters and buffers `AddDrawOnlyNavMeshes` keeps while it walks the
/// triangles of one navmesh.
struct Build {
    this: u32,
    frame: u32,
    /// Number of vertices of the triangle shape (grows with every triangle
    /// drawn in a style colour).
    a_count: u32,
    /// Number of points used in the first and second sets of lines.
    b_count: u32,
    c_count: u32,
    /// The triangle shape's index buffer and how much of it is written.
    index_buffer: u32,
    index_cursor: u32,
}

impl Build {
    fn push_index(&mut self, e: &mut Engine, value: u32) {
        e.mem
            .set_u16(self.index_buffer + 2 * self.index_cursor, value as u16);
        self.index_cursor += 1;
    }
}

/// `node->AttachChild(child, true)`: virtual slot 0xDC.
fn attach_child(e: &mut Engine, node: u32, child: u32) {
    e.vcall(node, SLOT_ATTACH_CHILD, &args![child, 1u32]);
}

/// `target->AttachProperty(property)` for the object in the pointer slot
/// `target_slot`.
fn attach_property(e: &mut Engine, target_slot: u32, property: u32) {
    let target = deref(e, target_slot);
    e.call(NI_AV_OBJECT_ATTACH_PROPERTY, &args![target, property]);
}

/// Builds the translation `(x, y, z)` and sets it on the object in
/// `target_slot`.
fn set_translation(e: &mut Engine, frame: u32, target_slot: u32, x: f32, y: f32, z: f32) {
    let translation = e
        .call(
            NI_POINT3_CONSTRUCT_XYZ,
            &args![frame + F_TRANSLATE, x, y, z],
        )
        .u32();
    let target = deref(e, target_slot);
    e.call(NI_AV_OBJECT_SET_TRANSLATE, &args![target, translation]);
}

/// A new line-set object over copies of the points, colours and flags of
/// three parallel arrays (`count` entries): the copies are allocated first
/// (12, 16 and 1 bytes per entry), filled from the arrays, and handed to
/// `NiLines`; null when its allocation failed.
fn new_lines(
    e: &mut Engine,
    points_array: u32,
    colours_array: u32,
    flags_array: u32,
    count: u32,
) -> u32 {
    let points = new_array_with_constructor(e, count, 12, NI_POINT3_CONSTRUCT);
    let colours = new_array_with_constructor(e, count, 16, NI_COLOR_A_DEFAULT_CONSTRUCT);
    let flags = e.call(OPERATOR_NEW, &args![count]).u32();
    let source = point_at(e, points_array, 0);
    e.call(MEMCPY, &args![points, source, count.wrapping_mul(12)]);
    let source = colour_at(e, colours_array, 0);
    e.call(MEMCPY, &args![colours, source, count.wrapping_shl(4)]);
    let source = fn_006a62f0(e, Ptr::new(flags_array), 0);
    e.call(MEMCPY, &args![flags, source, count]);
    let memory = e.call(NI_ALLOC, &args![0xc4u32]).u32();
    if memory == 0 {
        return 0;
    }
    e.call(
        NI_LINES_CONSTRUCT,
        &args![
            memory,
            count & 0xffff,
            points,
            colours,
            0u32,
            0u32,
            0u32,
            flags
        ],
    )
    .u32()
}

/// A shader property in the slot: new `BSShaderNoLightingProperty` (built into
/// `construct` or assigned), then flag 0x20 cleared.
fn set_up_shader_property(e: &mut Engine, slot: u32, construct: bool) {
    let property = new_no_lighting_property(e);
    if construct {
        pointer_construct(e, slot, property);
    } else {
        pointer_assign(e, slot, property);
    }
    let property = deref(e, slot);
    e.call(BS_SHADER_PROPERTY_SET_FLAG, &args![property, 0x20u32, 0u32]);
}

// Translated from 006a37a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshRender::AddDrawOnlyNavMeshes` (Xbox PDB): draws the navmeshes of
/// `cell` unless the cell already has a node in the cell map. A new node is
/// made (capacity: twice the number of navmeshes), attached to the root draw
/// node and entered in the cell map; then every navmesh of the cell becomes
/// a set of scene objects under it (see [`draw_nav_mesh`]). With `update` the
/// root node is updated at the end.
///
/// The compiler's exception states and the stack cookie check are not
/// translated.
pub fn nav_mesh_render_add_draw_only_nav_meshes(
    e: &mut Engine,
    this: Ptr<NavMeshRender>,
    cell: Ptr,
    update: bool,
) {
    if cell.is_null() {
        return;
    }
    let frame = e.mem.alloc(FRAME_SIZE);
    add_cell_nodes(e, this.addr(), cell.addr(), update, frame);
    e.mem.free(frame);
}

/// The body of `AddDrawOnlyNavMeshes` after the null check, on the locals
/// block `frame`.
fn add_cell_nodes(e: &mut Engine, this: u32, cell: u32, update: bool, frame: u32) {
    let nav_meshes = e.call(CELL_NAV_MESH_ARRAY, &args![cell]).u32();
    if nav_meshes == 0 {
        return;
    }
    let mesh_count = e.call(NAV_MESH_ARRAY_COUNT, &args![nav_meshes]).u32();
    pointer_construct(e, frame + F_NODE, 0);
    let cell_map = this + NAV_MESH_CELL_MAP;
    if e.call(NI_T_MAP_GET_AT, &args![cell_map, cell, frame + F_NODE])
        .bool()
    {
        pointer_destroy(e, frame + F_NODE);
        return;
    }
    let memory = e.call(NI_ALLOC, &args![0xacu32]).u32();
    let node = if memory == 0 {
        0
    } else {
        e.call(
            NI_NODE_CONSTRUCT,
            &args![memory, mesh_count.wrapping_shl(1)],
        )
        .u32()
    };
    pointer_assign(e, frame + F_NODE, node);
    let root = deref(e, ROOT_DRAW_NODE);
    let new_node = deref(e, frame + F_NODE);
    e.vcall(root, SLOT_ATTACH_CHILD, &args![new_node, 1u32]);
    e.call(NI_POINTER_COPY, &args![frame + F_NODE_COPY, frame + F_NODE]);
    let copy = e.mem.u32(frame + F_NODE_COPY);
    e.call(NI_T_MAP_SET_AT, &args![cell_map, cell, copy]);

    e.call(NAV_MESH_POINTER_CONSTRUCT, &args![frame + F_MESH]);
    for index in 0..mesh_count {
        let made = e
            .call(
                NAV_MESH_ARRAY_GET,
                &args![nav_meshes, frame + F_MESH_TEMP, index],
            )
            .u32();
        e.call(NAV_MESH_POINTER_ASSIGN, &args![frame + F_MESH, made]);
        e.call(NAV_MESH_POINTER_DESTROY, &args![frame + F_MESH_TEMP]);
        draw_nav_mesh(e, this, frame);
    }
    if update {
        update_root(e, frame + F_UPDATE);
        let root = deref(e, ROOT_DRAW_NODE);
        e.call(NI_AV_OBJECT_UPDATE_PROPERTIES, &args![root]);
    }
    e.call(NAV_MESH_POINTER_DESTROY, &args![frame + F_MESH]);
    pointer_destroy(e, frame + F_NODE);
}

/// Builds the scene objects of the navmesh in `F_MESH` under the cell's node:
/// a triangle shape (vertices, a colour per vertex, an index per triangle
/// corner; triangles in a style colour get vertices of their own), up to two
/// sets of lines (borders, connections), the boxes of the cover edges, and a
/// sphere for each of the mesh's spheres.
///
/// A mesh without triangles is skipped.
fn draw_nav_mesh(e: &mut Engine, this: u32, frame: u32) {
    let mesh_slot = frame + F_MESH;
    let mesh = deref(e, mesh_slot);
    let triangle_count = e.call(NAV_MESH_TRIANGLE_COUNT, &args![mesh]).u32();
    let mesh = deref(e, mesh_slot);
    let vertex_count = e.call(NAV_MESH_VERTEX_COUNT, &args![mesh]).u32();
    if triangle_count == 0 {
        return;
    }
    let mesh = deref(e, mesh_slot);
    let is_special = e.call(NAV_MESH_IS_SPECIAL, &args![mesh]).bool();

    bs_simple_array_ni_point3_construct(e, Ptr::new(frame + F_POINTS_A), vertex_count);
    bs_simple_array_ni_color_a_construct(e, Ptr::new(frame + F_COLOURS_A), vertex_count);
    let vertex_colour = style_colour(
        this,
        if is_special {
            STYLE_BLACK
        } else {
            STYLE_ORDINARY
        },
    );
    for index in 0..vertex_count {
        let mesh = deref(e, mesh_slot);
        let source = point_at(e, mesh + NAV_MESH_VERTICES, index);
        let target = point_at(e, frame + F_POINTS_A, index);
        copy_point(e, target, source);
        let target = colour_at(e, frame + F_COLOURS_A, index);
        copy_colour(e, target, vertex_colour);
    }

    let index_buffer = e
        .call(
            NI_ALLOC_ARRAY,
            &args![triangle_count.wrapping_mul(3).wrapping_shl(1)],
        )
        .u32();
    let line_capacity = triangle_count.wrapping_mul(6);
    bs_simple_array_ni_point3_construct(e, Ptr::new(frame + F_POINTS_B), line_capacity);
    bs_simple_array_u8_construct(e, Ptr::new(frame + F_FLAGS_B), line_capacity);
    bs_simple_array_ni_color_a_construct(e, Ptr::new(frame + F_COLOURS_B), line_capacity);
    bs_simple_array_ni_point3_construct(e, Ptr::new(frame + F_POINTS_C), line_capacity);
    bs_simple_array_u8_construct(e, Ptr::new(frame + F_FLAGS_C), line_capacity);
    bs_simple_array_ni_color_a_construct(e, Ptr::new(frame + F_COLOURS_C), line_capacity);
    let mut build = Build {
        this,
        frame,
        a_count: vertex_count,
        b_count: 0,
        c_count: 0,
        index_buffer,
        index_cursor: 0,
    };

    for triangle_index in 0..triangle_count {
        let mesh = deref(e, mesh_slot);
        let triangle = e
            .call(NAV_MESH_TRIANGLE_AT, &args![mesh, triangle_index & 0xffff])
            .u32();
        if e.call(TRIANGLE_TEST_FLAGS, &args![triangle, 0x20u32])
            .bool()
        {
            continue;
        }
        draw_triangle(e, &mut build, is_special, triangle_index, triangle);
    }

    finish_nav_mesh(e, &mut build, triangle_count);
}

/// One triangle of the navmesh: its vertex indices go into the index buffer
/// (a triangle in a style colour first gets three vertices of its own, in that
/// colour), then each of its three edges may add lines.
fn draw_triangle(
    e: &mut Engine,
    build: &mut Build,
    is_special: bool,
    triangle_index: u32,
    triangle: u32,
) {
    let (this, frame) = (build.this, build.frame);
    let mesh_slot = frame + F_MESH;
    let corners = frame + F_CORNERS;
    fn_006a5b30(
        e,
        Ptr::new(triangle),
        Ptr::new(corners),
        Ptr::new(corners + 2),
        Ptr::new(corners + 4),
    );
    let corner = |e: &Engine, i: u32| u32::from(e.mem.u16(corners + 2 * i));
    if is_special {
        for i in 0..3 {
            let value = corner(e, i);
            build.push_index(e, value);
        }
    } else {
        let style = fn_006a5da0(e, Ptr::new(this), Ptr::new(triangle));
        if style != 0 {
            let new_size = build.a_count + 3;
            e.call(
                POINT_ARRAY_SET_SIZE,
                &args![frame + F_POINTS_A, new_size, 1u32],
            );
            fn_006a6660(e, Ptr::new(frame + F_COLOURS_A), new_size, true);
            for i in 0..3 {
                let vertex = corner(e, i);
                let source = point_at(e, frame + F_POINTS_A, vertex);
                let target = point_at(e, frame + F_POINTS_A, build.a_count);
                copy_point(e, target, source);
                let target = colour_at(e, frame + F_COLOURS_A, build.a_count);
                copy_colour(e, target, style_colour(this, style));
                let value = build.a_count;
                build.push_index(e, value);
                build.a_count += 1;
            }
        } else {
            for i in 0..3 {
                let value = corner(e, i);
                build.push_index(e, value);
            }
        }
    }

    fn_006a5b70(
        e,
        Ptr::new(triangle),
        Ptr::new(frame + F_NEIGHBOURS),
        Ptr::new(frame + F_NEIGHBOURS + 2),
        Ptr::new(frame + F_NEIGHBOURS + 4),
    );
    for edge in 0..3u32 {
        let next_b = build.b_count + 1;
        let edge_flag = e.call(TRIANGLE_EDGE_FLAG, &args![triangle, edge]).bool();
        let neighbour = u32::from(e.mem.u16(frame + F_NEIGHBOURS + 2 * edge));
        if e.mem.u8(DRAW_CONNECTIONS) != 0 && neighbour != 0xffff {
            add_connection_line(e, build, triangle_index, edge, edge_flag);
        }
        if neighbour > triangle_index || edge_flag {
            let source = {
                let vertex = corner(e, edge);
                let mesh = deref(e, mesh_slot);
                point_at(e, mesh + NAV_MESH_VERTICES, vertex)
            };
            let target = point_at(e, frame + F_POINTS_B, build.b_count);
            copy_point(e, target, source);
            let source = {
                let vertex = corner(e, (edge + 1) % 3);
                let mesh = deref(e, mesh_slot);
                point_at(e, mesh + NAV_MESH_VERTICES, vertex)
            };
            let target = point_at(e, frame + F_POINTS_B, next_b);
            copy_point(e, target, source);
            let flag = fn_006a62f0(e, Ptr::new(frame + F_FLAGS_B), build.b_count);
            e.mem.set_u8(flag, 1);
            let flag = fn_006a62f0(e, Ptr::new(frame + F_FLAGS_B), next_b);
            e.mem.set_u8(flag, 0);
            if neighbour != 0xffff {
                // The colour of the line to a neighbouring triangle (the
                // game's two branches, for an edge to another navmesh and for
                // one inside this mesh, write the same colour).
                let ordinary = style_colour(this, STYLE_ORDINARY);
                let target = colour_at(e, frame + F_COLOURS_B, build.b_count);
                copy_colour(e, target, ordinary);
                let target = colour_at(e, frame + F_COLOURS_B, next_b);
                copy_colour(e, target, ordinary);
            } else {
                add_border_edge(e, build, triangle, edge, next_b);
            }
            build.b_count += 2;
        }
    }
}

/// For a triangle edge that has a neighbour, with the connection lines on:
/// finds the edge of the neighbour that leads back to this triangle (across
/// a navmesh border through the edge extra info, else inside the mesh) and,
/// if there is one, adds a line between two points weighted from the corners
/// of the two triangles to the second set of lines.
fn add_connection_line(
    e: &mut Engine,
    build: &mut Build,
    triangle_index: u32,
    edge: u32,
    edge_flag: bool,
) {
    let (this, frame) = (build.this, build.frame);
    let mesh_slot = frame + F_MESH;
    let other_mesh_slot = frame + F_OTHER_MESH;
    e.call(NAV_MESH_POINTER_COPY, &args![other_mesh_slot, mesh_slot]);
    let mut other_triangle = 0u32;
    let mut back_edge: Option<u32> = None;
    if edge_flag {
        let mesh = deref(e, mesh_slot);
        let info = e
            .call(
                NAV_MESH_EDGE_EXTRA_INFO,
                &args![mesh, triangle_index & 0xffff, edge],
            )
            .u32();
        let target = if info != 0 { e.mem.u32(info + 4) } else { 0 };
        if info != 0 && target != 0 && e.call(EXTRA_INFO_HAS_NAV_MESH, &args![target]).bool() {
            e.call(EXTRA_INFO_GET_NAV_MESH, &args![target, other_mesh_slot]);
            let other_index = u32::from(e.mem.u16(info + 8));
            let other_mesh = deref(e, other_mesh_slot);
            other_triangle = e
                .call(NAV_MESH_TRIANGLE_AT, &args![other_mesh, other_index])
                .u32();
            fn_006a5b70(
                e,
                Ptr::new(other_triangle),
                Ptr::new(frame + F_OTHER_NEIGHBOURS),
                Ptr::new(frame + F_OTHER_NEIGHBOURS + 2),
                Ptr::new(frame + F_OTHER_NEIGHBOURS + 4),
            );
            for other_edge in 0..3u32 {
                if e.call(TRIANGLE_EDGE_FLAG, &args![other_triangle, other_edge])
                    .bool()
                {
                    let other_mesh = deref(e, other_mesh_slot);
                    let back_info = e
                        .call(
                            NAV_MESH_EDGE_EXTRA_INFO,
                            &args![other_mesh, other_index, other_edge],
                        )
                        .u32();
                    if back_info != 0 {
                        e.call(NAV_MESH_POINTER_CONSTRUCT, &args![frame + F_BACK_EDGE_MESH]);
                        let back_target = e.mem.u32(back_info + 4);
                        e.call(
                            EXTRA_INFO_GET_NAV_MESH,
                            &args![back_target, frame + F_BACK_EDGE_MESH],
                        );
                        let same = e
                            .call(
                                NAV_MESH_POINTER_EQUAL,
                                &args![frame + F_BACK_EDGE_MESH, mesh_slot],
                            )
                            .bool();
                        if same && u32::from(e.mem.u16(back_info + 8)) == triangle_index {
                            back_edge = Some(other_edge);
                        }
                        e.call(NAV_MESH_POINTER_DESTROY, &args![frame + F_BACK_EDGE_MESH]);
                    }
                }
            }
        }
    } else {
        let neighbour = u32::from(e.mem.u16(frame + F_NEIGHBOURS + 2 * edge));
        let mesh = deref(e, mesh_slot);
        other_triangle = e.call(NAV_MESH_TRIANGLE_AT, &args![mesh, neighbour]).u32();
        fn_006a5b70(
            e,
            Ptr::new(other_triangle),
            Ptr::new(frame + F_BACK_NEIGHBOURS),
            Ptr::new(frame + F_BACK_NEIGHBOURS + 2),
            Ptr::new(frame + F_BACK_NEIGHBOURS + 4),
        );
        for other_edge in 0..3u32 {
            if u32::from(e.mem.u16(frame + F_BACK_NEIGHBOURS + 2 * other_edge)) == triangle_index {
                back_edge = Some(other_edge);
            }
        }
    }

    if let Some(back_edge) = back_edge {
        let corners = frame + F_CORNERS;
        let line_next = build.c_count + 1;
        let weight_a = e.global::<f32>(CONNECTION_WEIGHT_A);
        let weight_b = e.global::<f32>(CONNECTION_WEIGHT_B);
        let weight_c = e.global::<f32>(CONNECTION_WEIGHT_C);
        // The corners of this triangle, starting at the edge's own vertex.
        for slot in 0..3u32 {
            let vertex = u32::from(e.mem.u16(corners + 2 * ((edge + slot) % 3)));
            let mesh = deref(e, mesh_slot);
            let source = point_at(e, mesh + NAV_MESH_VERTICES, vertex);
            copy_point(e, frame + F_CORNER_POINTS + 0x10 * slot, source);
        }
        let (p1, p2, p3) = (
            frame + F_CORNER_POINTS,
            frame + F_CORNER_POINTS + 0x10,
            frame + F_CORNER_POINTS + 0x20,
        );
        // (P2 * a) + (P1 * b) + (P3 * c), in the order the game evaluates it.
        let third = scale_point(e, p3, frame + F_SCRATCH, weight_c);
        let first = scale_point(e, p1, frame + F_SCRATCH + 0x10, weight_b);
        let second = scale_point(e, p2, frame + F_SCRATCH + 0x20, weight_a);
        let sum = add_points(e, second, frame + F_SCRATCH + 0x30, first);
        add_points(e, sum, frame + F_LINE_START, third);

        fn_006a5b30(
            e,
            Ptr::new(other_triangle),
            Ptr::new(frame + F_OTHER_CORNERS),
            Ptr::new(frame + F_OTHER_CORNERS + 2),
            Ptr::new(frame + F_OTHER_CORNERS + 4),
        );
        for slot in 0..3u32 {
            let vertex = u32::from(
                e.mem
                    .u16(frame + F_OTHER_CORNERS + 2 * ((back_edge + slot) % 3)),
            );
            let other_mesh = deref(e, other_mesh_slot);
            let source = point_at(e, other_mesh + NAV_MESH_VERTICES, vertex);
            copy_point(e, frame + F_OTHER_CORNER_POINTS + 0x10 * slot, source);
        }
        let (q1, q2, q3) = (
            frame + F_OTHER_CORNER_POINTS,
            frame + F_OTHER_CORNER_POINTS + 0x10,
            frame + F_OTHER_CORNER_POINTS + 0x20,
        );
        let third = scale_point(e, q3, frame + F_SCRATCH + 0x40, weight_c);
        let second = scale_point(e, q2, frame + F_SCRATCH + 0x50, weight_b);
        let first = scale_point(e, q1, frame + F_SCRATCH + 0x60, weight_a);
        let sum = add_points(e, first, frame + F_SCRATCH + 0x70, second);
        add_points(e, sum, frame + F_LINE_END, third);

        let target = point_at(e, frame + F_POINTS_C, build.c_count);
        copy_point(e, target, frame + F_LINE_START);
        let target = point_at(e, frame + F_POINTS_C, line_next);
        copy_point(e, target, frame + F_LINE_END);
        let flag = fn_006a62f0(e, Ptr::new(frame + F_FLAGS_C), build.c_count);
        e.mem.set_u8(flag, 1);
        let flag = fn_006a62f0(e, Ptr::new(frame + F_FLAGS_C), line_next);
        e.mem.set_u8(flag, 0);
        let target = colour_at(e, frame + F_COLOURS_C, build.c_count);
        copy_colour(e, target, style_colour(this, STYLE_CONNECTION_START));
        let target = colour_at(e, frame + F_COLOURS_C, line_next);
        copy_colour(e, target, style_colour(this, STYLE_CONNECTION_END));
        build.c_count += 2;
    }
    e.call(NAV_MESH_POINTER_DESTROY, &args![other_mesh_slot]);
}

/// The border edge of a triangle (no neighbour): both ends of the line get the
/// colour of the border kind (0, 1, 2, 15, other), and with the cover boxes on
/// and a kind above 1 the edge gets a box of a height by its kind (and
/// smaller boxes at its ends where the border information says so).
fn add_border_edge(e: &mut Engine, build: &mut Build, triangle: u32, edge: u32, next_b: u32) {
    let (this, frame) = (build.this, build.frame);
    let border = frame + F_BORDER;
    e.mem.set_u16(border, 0);
    e.mem.set_u8(border + 2, 0);
    e.mem.set_u8(border + 3, 0);
    if edge < 2 {
        e.call(
            TRIANGLE_BORDER_INFO,
            &args![triangle, edge, border, border + 2, border + 3],
        );
    }
    let kind = u32::from(e.mem.u16(border));
    let style = match kind {
        0 => STYLE_BORDER_KIND_0,
        1 => STYLE_BLACK,
        2 => STYLE_BORDER_KIND_2,
        15 => STYLE_BORDER_KIND_15,
        _ => STYLE_BORDER_OTHER,
    };
    let target = colour_at(e, frame + F_COLOURS_B, build.b_count);
    copy_colour(e, target, style_colour(this, style));
    let target = colour_at(e, frame + F_COLOURS_B, next_b);
    copy_colour(e, target, style_colour(this, style));
    if e.mem.u8(DRAW_COVER) != 0 && kind > 1 {
        add_cover_boxes(e, build, next_b, kind);
    }
}

/// The boxes drawn on a cover edge: one along the edge, and (by the two
/// border flags) one at each end.
fn add_cover_boxes(e: &mut Engine, build: &mut Build, next_b: u32, kind: u32) {
    let (this, frame) = (build.this, build.frame);
    let node_slot = frame + F_NODE;
    let border = frame + F_BORDER;
    let height = fn_006a5af0(e, kind);
    let box_size = e.global::<f32>(COVER_BOX_SIZE);
    let end_box_size = e.global::<f32>(COVER_END_BOX_SIZE);
    let end_height = height + e.global::<f32>(COVER_END_BOX_EXTRA_HEIGHT);

    let colour = colour_at(e, frame + F_COLOURS_B, build.b_count);
    let end = point_at(e, frame + F_POINTS_B, next_b);
    let start = point_at(e, frame + F_POINTS_B, build.b_count);
    let created = e
        .call(
            TEST_OBJECTS_CREATE_BOX,
            &args![start, end, box_size, box_size, height, 0.0f32, colour, 0u32],
        )
        .u32();
    let node = deref(e, node_slot);
    attach_child(e, node, created);

    let end = point_at(e, frame + F_POINTS_B, next_b);
    let start = point_at(e, frame + F_POINTS_B, build.b_count);
    subtract_points(e, start, frame + F_DIRECTION, end);
    e.call(NI_POINT3_NORMALISE, &args![frame + F_DIRECTION]);
    let step = e.global::<f32>(COVER_STEP);
    e.call(NI_POINT3_SCALE_IN_PLACE, &args![frame + F_DIRECTION, step]);
    let black = style_colour(this, STYLE_BLACK);
    if e.mem.u8(border + 2) != 0 {
        let end = point_at(e, frame + F_POINTS_B, next_b);
        subtract_points(e, end, frame + F_BOX_POINTS, frame + F_DIRECTION);
        let end = point_at(e, frame + F_POINTS_B, next_b);
        add_points(e, end, frame + F_BOX_POINTS + 0x10, frame + F_DIRECTION);
        let created = e
            .call(
                TEST_OBJECTS_CREATE_BOX,
                &args![
                    frame + F_BOX_POINTS,
                    frame + F_BOX_POINTS + 0x10,
                    end_box_size,
                    end_box_size,
                    end_height,
                    0.0f32,
                    black,
                    0u32
                ],
            )
            .u32();
        let node = deref(e, node_slot);
        attach_child(e, node, created);
    }
    if e.mem.u8(border + 3) != 0 {
        let start = point_at(e, frame + F_POINTS_B, build.b_count);
        subtract_points(e, start, frame + F_BOX_POINTS + 0x20, frame + F_DIRECTION);
        let start = point_at(e, frame + F_POINTS_B, build.b_count);
        add_points(e, start, frame + F_BOX_POINTS + 0x30, frame + F_DIRECTION);
        let created = e
            .call(
                TEST_OBJECTS_CREATE_BOX,
                &args![
                    frame + F_BOX_POINTS + 0x20,
                    frame + F_BOX_POINTS + 0x30,
                    end_box_size,
                    end_box_size,
                    end_height,
                    0.0f32,
                    black,
                    0u32
                ],
            )
            .u32();
        let node = deref(e, node_slot);
        attach_child(e, node, created);
    }
}

/// After the triangles of one navmesh: makes the objects from the collected
/// arrays and attaches them under the cell's node, then destroys the arrays.
fn finish_nav_mesh(e: &mut Engine, build: &mut Build, triangle_count: u32) {
    let (this, frame) = (build.this, build.frame);
    let mesh_slot = frame + F_MESH;
    let node_slot = frame + F_NODE;

    // The first set of lines: borders.
    let lines = new_lines(
        e,
        frame + F_POINTS_B,
        frame + F_COLOURS_B,
        frame + F_FLAGS_B,
        build.b_count,
    );
    pointer_construct(e, frame + F_LINES_B, lines);
    set_up_shader_property(e, frame + F_SHADER_B, true);
    let shader = deref(e, frame + F_SHADER_B);
    attach_property(e, frame + F_LINES_B, shader);
    let material = deref(e, this + EDGE_AND_VERTEX_MATERIALS);
    attach_property(e, frame + F_LINES_B, material);
    let alpha = deref(e, this + 0x1e8);
    attach_property(e, frame + F_LINES_B, alpha);
    set_translation(e, frame, frame + F_LINES_B, 0.0, 0.0, 1.0);

    // The second set of lines: connections.
    pointer_construct(e, frame + F_LINES_C, 0);
    if build.c_count != 0 {
        let lines = new_lines(
            e,
            frame + F_POINTS_C,
            frame + F_COLOURS_C,
            frame + F_FLAGS_C,
            build.c_count,
        );
        pointer_assign(e, frame + F_LINES_C, lines);
        set_up_shader_property(e, frame + F_SHADER_B, false);
        let shader = deref(e, frame + F_SHADER_B);
        attach_property(e, frame + F_LINES_C, shader);
        let alpha = deref(e, this + 0x1e8);
        attach_property(e, frame + F_LINES_C, alpha);
        let height = e.global::<f32>(LINES_C_HEIGHT);
        set_translation(e, frame, frame + F_LINES_C, 0.0, 0.0, height);
    }

    // The triangle shape.
    let a_count = build.a_count;
    let points = new_array_with_constructor(e, a_count, 12, NI_POINT3_CONSTRUCT);
    let colours = new_array_with_constructor(e, a_count, 16, NI_COLOR_A_DEFAULT_CONSTRUCT);
    let source = point_at(e, frame + F_POINTS_A, 0);
    e.call(MEMCPY, &args![points, source, a_count.wrapping_mul(12)]);
    let source = colour_at(e, frame + F_COLOURS_A, 0);
    e.call(MEMCPY, &args![colours, source, a_count.wrapping_shl(4)]);
    let memory = e.call(NI_ALLOC, &args![0xc4u32]).u32();
    let shape = if memory == 0 {
        0
    } else {
        e.call(
            NI_TRI_SHAPE_CONSTRUCT,
            &args![
                memory,
                a_count & 0xffff,
                points,
                0u32,
                colours,
                0u32,
                0u32,
                0u32,
                triangle_count & 0xffff,
                build.index_buffer
            ],
        )
        .u32()
    };
    pointer_construct(e, frame + F_TRI_SHAPE, shape);
    e.call(
        NI_FIXED_STRING_CONSTRUCT,
        &args![frame + F_NAME, TRI_SHAPE_NAME],
    );
    let target = deref(e, frame + F_TRI_SHAPE);
    e.call(NI_AV_OBJECT_SET_NAME, &args![target, frame + F_NAME]);
    e.call(NI_FIXED_STRING_DESTROY, &args![frame + F_NAME]);
    set_up_shader_property(e, frame + F_SHADER_A, true);
    let material = deref(e, this + TRIANGLE_MATERIALS);
    attach_property(e, frame + F_TRI_SHAPE, material);
    let alpha = deref(e, this + 0x1e8);
    attach_property(e, frame + F_TRI_SHAPE, alpha);
    let shader = deref(e, frame + F_SHADER_A);
    attach_property(e, frame + F_TRI_SHAPE, shader);
    set_translation(e, frame, frame + F_TRI_SHAPE, 0.0, 0.0, 1.0);

    // The spheres of the mesh.
    let mut sphere_index = 0u32;
    loop {
        let mesh = deref(e, mesh_slot);
        let sphere_count = e.call(NAV_MESH_SPHERE_COUNT, &args![mesh]).u32();
        if sphere_index >= sphere_count {
            break;
        }
        let mesh = deref(e, mesh_slot);
        let sphere = e.call(NAV_MESH_SPHERE_AT, &args![mesh, sphere_index]).u32();
        let red = e.global::<f32>(QUARTER);
        let alpha = e.global::<f32>(HALF);
        let colour = e
            .call(
                NI_COLOR_A_CONSTRUCT,
                &args![frame + F_SPHERE_COLOUR, red, 0.0f32, 0.0f32, alpha],
            )
            .u32();
        let radius = e.call(SPHERE_RADIUS, &args![sphere]).f32();
        let made = e
            .call(
                TEST_OBJECTS_CREATE_SPHERE,
                &args![radius, 10u32, 10u32, colour],
            )
            .u32();
        pointer_construct(e, frame + F_SPHERE, made);
        let memory = e.call(NI_ALLOC, &args![0x1cu32]).u32();
        let alpha_property = if memory == 0 {
            0
        } else {
            e.call(NI_ALPHA_PROPERTY_CONSTRUCT, &args![memory]).u32()
        };
        pointer_construct(e, frame + F_SPHERE_ALPHA, alpha_property);
        let alpha_property = deref(e, frame + F_SPHERE_ALPHA);
        e.call(NI_ALPHA_PROPERTY_SET_BLENDING, &args![alpha_property, 1u32]);
        let alpha_property = deref(e, frame + F_SPHERE_ALPHA);
        attach_property(e, frame + F_SPHERE, alpha_property);
        let position = e.call(NI_POINT3_CONSTRUCT, &args![sphere]).u32();
        let target = deref(e, frame + F_SPHERE);
        e.call(NI_AV_OBJECT_SET_TRANSLATE, &args![target, position]);
        let node = deref(e, node_slot);
        let child = deref(e, frame + F_SPHERE);
        attach_child(e, node, child);
        pointer_destroy(e, frame + F_SPHERE_ALPHA);
        pointer_destroy(e, frame + F_SPHERE);
        sphere_index += 1;
    }

    let node = deref(e, node_slot);
    let child = deref(e, frame + F_LINES_B);
    attach_child(e, node, child);
    let node = deref(e, node_slot);
    let child = deref(e, frame + F_TRI_SHAPE);
    attach_child(e, node, child);
    if build.c_count != 0 {
        let node = deref(e, node_slot);
        let child = deref(e, frame + F_LINES_C);
        attach_child(e, node, child);
    }
    let node = deref(e, node_slot);
    e.call(ADD_NO_LIGHTING_PROPERTY_RECURSE, &args![node]);
    let node = deref(e, node_slot);
    e.call(SHADER_MANAGER_PREPARE_OBJECT, &args![node, 0u32, 0u32]);

    pointer_destroy(e, frame + F_SHADER_A);
    pointer_destroy(e, frame + F_TRI_SHAPE);
    pointer_destroy(e, frame + F_LINES_C);
    pointer_destroy(e, frame + F_SHADER_B);
    pointer_destroy(e, frame + F_LINES_B);
    fn_006a6640(e, Ptr::new(frame + F_COLOURS_C));
    fn_006a67d0(e, Ptr::new(frame + F_FLAGS_C));
    e.call(POINT_ARRAY_DESTRUCTOR, &args![frame + F_POINTS_C]);
    fn_006a6640(e, Ptr::new(frame + F_COLOURS_B));
    fn_006a67d0(e, Ptr::new(frame + F_FLAGS_B));
    e.call(POINT_ARRAY_DESTRUCTOR, &args![frame + F_POINTS_B]);
    fn_006a6640(e, Ptr::new(frame + F_COLOURS_A));
    e.call(POINT_ARRAY_DESTRUCTOR, &args![frame + F_POINTS_A]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006a2920, nav_mesh_render_init_sdm()),
        entry!(0x006a29a0, fn_006a29a0()),
        entry!(
            0x006a29f0,
            fn_006a29f0(Ptr<NavMeshRender>, u32) -> Ptr<NavMeshRender>
        ),
        entry!(
            0x006a2a20,
            nav_mesh_render_nav_mesh_render(Ptr<NavMeshRender>) -> Ptr<NavMeshRender>
        ),
        entry!(0x006a3580, fn_006a3580(Ptr<NavMeshRender>)),
        entry!(
            0x006a36e0,
            nav_mesh_render_set_draw(Ptr<NavMeshRender>, bool)
        ),
        entry!(
            0x006a37a0,
            nav_mesh_render_add_draw_only_nav_meshes(Ptr<NavMeshRender>, Ptr, bool)
        ),
        entry!(0x006a5af0, fn_006a5af0(u32) -> f32),
        entry!(0x006a5b30, fn_006a5b30(Ptr, Ptr, Ptr, Ptr)),
        entry!(0x006a5b70, fn_006a5b70(Ptr, Ptr, Ptr, Ptr)),
        entry!(
            0x006a5bb0,
            nav_mesh_render_remove_draw_only_nav_meshes(Ptr<NavMeshRender>, Ptr)
        ),
        entry!(0x006a5ca0, fn_006a5ca0(Ptr<NavMeshRender>)),
        entry!(0x006a5da0, fn_006a5da0(Ptr<NavMeshRender>, Ptr) -> u32),
        entry!(0x006a5ec0, fn_006a5ec0(Ptr) -> bool),
        entry!(
            0x006a5ee0,
            nav_mesh_render_set_transparent_over_lay_mode(Ptr<NavMeshRender>, bool)
        ),
        entry!(0x006a61b0, fn_006a61b0(Ptr<NavMeshRender>)),
        entry!(0x006a6290, fn_006a6290(Ptr, u32) -> Ptr),
        entry!(0x006a62c0, fn_006a62c0(Ptr, u32) -> Ptr),
        entry!(0x006a62f0, fn_006a62f0(Ptr, u32) -> u32),
        entry!(0x006a6310, ni_t_map_nav_mesh_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x006a6340, ni_t_map_cell_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x006a6370, fn_006a6370(Ptr, u32) -> Ptr),
        entry!(0x006a6450, fn_006a6450(Ptr)),
        entry!(0x006a64e0, fn_006a64e0(Ptr, u32) -> Ptr),
        entry!(0x006a6550, fn_006a6550(Ptr)),
        entry!(0x006a65e0, bs_simple_array_ni_point3_construct(Ptr, u32) -> Ptr),
        entry!(0x006a6610, bs_simple_array_ni_color_a_construct(Ptr, u32) -> Ptr),
        entry!(0x006a6640, fn_006a6640(Ptr)),
        entry!(0x006a6660, fn_006a6660(Ptr, u32, bool)),
        entry!(0x006a67a0, bs_simple_array_u8_construct(Ptr, u32) -> Ptr),
        entry!(0x006a67d0, fn_006a67d0(Ptr)),
        entry!(0x006a67f0, ni_t_map_base_nav_mesh_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x006a6820, ni_t_map_base_cell_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x006a6850, bs_simple_array_ni_color_a_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x006a6880, bs_simple_array_u8_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x006a69b0, fn_006a69b0(Ptr, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{cell::RefCell, rc::Rc};

    /// Expected results captured from FalloutNV.exe 1.4.0.525 with nv-call.
    /// Entry `i` uses the set of flags whose bits correspond to `i` below.
    /// The 512 vectors cover all combinations of 0x10, 0x40, 0x100, 0x200,
    /// 0x400, 0x1000, 0x2000, 0x4000 and 0x80000000. Executable SHA-256:
    /// 19406942e48724d797300c4ea6be9ac69a32f670b8c35db09279a2258422739f.
    const NATIVE_STYLES: [u8; 512] = [
        0, 14, 13, 13, 0, 14, 13, 13, 7, 7, 8, 8, 7, 7, 8, 8, 4, 14, 5, 5, 4, 14, 5, 5, 7, 7, 5, 5,
        7, 7, 5, 5, 9, 14, 13, 13, 9, 14, 13, 13, 7, 7, 8, 8, 7, 7, 8, 8, 4, 14, 5, 5, 4, 14, 5, 5,
        7, 7, 5, 5, 7, 7, 5, 5, 11, 11, 11, 11, 12, 12, 12, 12, 11, 11, 11, 11, 12, 12, 12, 12, 11,
        11, 11, 11, 12, 12, 12, 12, 11, 11, 11, 11, 12, 12, 12, 12, 11, 11, 11, 11, 12, 12, 12, 12,
        11, 11, 11, 11, 12, 12, 12, 12, 11, 11, 11, 11, 12, 12, 12, 12, 11, 11, 11, 11, 12, 12, 12,
        12, 11, 11, 11, 11, 12, 12, 12, 12, 11, 11, 11, 11, 12, 12, 12, 12, 11, 11, 11, 11, 12, 12,
        12, 12, 11, 11, 11, 11, 12, 12, 12, 12, 11, 11, 11, 11, 12, 12, 12, 12, 11, 11, 11, 11, 12,
        12, 12, 12, 11, 11, 11, 11, 12, 12, 12, 12, 11, 11, 11, 11, 12, 12, 12, 12, 11, 11, 11, 11,
        12, 12, 12, 12, 11, 11, 11, 11, 12, 12, 12, 12, 11, 11, 11, 11, 12, 12, 12, 12, 11, 11, 11,
        11, 12, 12, 12, 12, 11, 11, 11, 11, 12, 12, 12, 12, 11, 11, 11, 11, 12, 12, 12, 12, 11, 11,
        11, 11, 12, 12, 12, 12, 11, 11, 11, 11, 12, 12, 12, 12, 16, 14, 13, 13, 16, 14, 13, 13, 7,
        7, 8, 8, 7, 7, 8, 8, 4, 14, 5, 5, 4, 14, 5, 5, 7, 7, 5, 5, 7, 7, 5, 5, 9, 14, 13, 13, 9,
        14, 13, 13, 7, 7, 8, 8, 7, 7, 8, 8, 4, 14, 5, 5, 4, 14, 5, 5, 7, 7, 5, 5, 7, 7, 5, 5, 11,
        11, 11, 11, 12, 12, 12, 12, 11, 11, 11, 11, 12, 12, 12, 12, 11, 11, 11, 11, 12, 12, 12, 12,
        11, 11, 11, 11, 12, 12, 12, 12, 11, 11, 11, 11, 12, 12, 12, 12, 11, 11, 11, 11, 12, 12, 12,
        12, 11, 11, 11, 11, 12, 12, 12, 12, 11, 11, 11, 11, 12, 12, 12, 12, 11, 11, 11, 11, 12, 12,
        12, 12, 11, 11, 11, 11, 12, 12, 12, 12, 11, 11, 11, 11, 12, 12, 12, 12, 11, 11, 11, 11, 12,
        12, 12, 12, 11, 11, 11, 11, 12, 12, 12, 12, 11, 11, 11, 11, 12, 12, 12, 12, 11, 11, 11, 11,
        12, 12, 12, 12, 11, 11, 11, 11, 12, 12, 12, 12, 11, 11, 11, 11, 12, 12, 12, 12, 11, 11, 11,
        11, 12, 12, 12, 12, 11, 11, 11, 11, 12, 12, 12, 12, 11, 11, 11, 11, 12, 12, 12, 12, 11, 11,
        11, 11, 12, 12, 12, 12, 11, 11, 11, 11, 12, 12, 12, 12, 11, 11, 11, 11, 12, 12, 12, 12, 11,
        11, 11, 11, 12, 12, 12, 12,
    ];

    const STYLE_FLAG_BITS: [u32; 9] = [
        0x10,
        0x40,
        0x100,
        0x200,
        0x400,
        0x1000,
        0x2000,
        0x4000,
        0x8000_0000,
    ];

    fn engine_with_triangle_flag_doubles() -> Engine {
        let mut e = Engine::new();
        e.register(TRIANGLE_FLAGS_SPECIAL_KIND, |e, a| {
            (e.mem.u32(a[0] + 0x0c) & 0xe000).into_ret()
        });
        e.register(TRIANGLE_TEST_FLAGS, |e, a| {
            (e.mem.u32(a[0] + 0x0c) & a[1] != 0).into_ret()
        });
        e.register(TRIANGLE_HAS_FLAG_0X40, |e, a| {
            (e.mem.u32(a[0] + 0x0c) & 0x40 != 0).into_ret()
        });
        e.register(TRIANGLE_HAS_FLAG_0X10, |e, a| {
            (e.mem.u32(a[0] + 0x0c) & 0x10 != 0).into_ret()
        });
        e.register(TRIANGLE_HAS_FLAG_0X400, |e, a| {
            (e.mem.u32(a[0] + 0x0c) & 0x400 != 0).into_ret()
        });
        e
    }

    #[test]
    fn edge_style_matches_all_512_native_flag_combinations_and_short_circuits() {
        let mut e = engine_with_triangle_flag_doubles();
        let triangle: Ptr = Ptr::new(e.mem.alloc(16));
        for (combination, expected) in NATIVE_STYLES.iter().copied().enumerate() {
            let mut flags = 0u32;
            for (bit, mask) in STYLE_FLAG_BITS.iter().copied().enumerate() {
                if combination & (1 << bit) != 0 {
                    flags |= mask;
                }
            }
            e.mem.set_u32(triangle.addr() + 0x0c, flags);
            e.call_log = Some(vec![]);

            let style = fn_006a5da0(&mut e, Ptr::NULL, triangle);
            assert_eq!(style, u32::from(expected), "flags {flags:#010x}");

            let calls = e.call_log.take().unwrap();
            let addresses: Vec<u32> = calls.iter().map(|(address, _)| *address).collect();
            let mut expected_calls = vec![TRIANGLE_FLAGS_SPECIAL_KIND];
            if flags & 0xe000 != 0 {
                expected_calls.push(TRIANGLE_TEST_FLAGS);
            } else {
                expected_calls.push(TRIANGLE_HAS_FLAG_0X40);
                if flags & 0x40 != 0 {
                    expected_calls.push(TRIANGLE_HAS_FLAG_0X400);
                    if flags & 0x400 == 0 {
                        expected_calls.push(TRIANGLE_TEST_FLAGS);
                    }
                } else {
                    expected_calls.push(TRIANGLE_TEST_FLAGS);
                    if flags & 0x200 == 0 {
                        expected_calls.push(TRIANGLE_HAS_FLAG_0X10);
                        if flags & 0x10 == 0 {
                            expected_calls.push(TRIANGLE_HAS_FLAG_0X400);
                            if flags & 0x400 == 0 {
                                expected_calls.push(TRIANGLE_TEST_FLAGS);
                                if flags & 0x1000 == 0 {
                                    expected_calls.push(TRIANGLE_TEST_FLAGS);
                                }
                            }
                        }
                    }
                }
            }
            assert_eq!(
                addresses, expected_calls,
                "call path for flags {flags:#010x}"
            );

            let test_masks: Vec<u32> = calls
                .iter()
                .filter(|(address, _)| *address == TRIANGLE_TEST_FLAGS)
                .map(|(_, words)| words[1])
                .collect();
            let expected_masks: Vec<u32> = if flags & 0xe000 != 0 {
                vec![0x100]
            } else if flags & 0x40 != 0 {
                if flags & 0x400 != 0 {
                    vec![]
                } else {
                    vec![0x200]
                }
            } else {
                let mut masks = vec![0x200];
                if flags & 0x200 == 0 && flags & 0x10 == 0 && flags & 0x400 == 0 {
                    masks.push(0x1000);
                    if flags & 0x1000 == 0 {
                        masks.push(0x8000_0000);
                    }
                }
                masks
            };
            assert_eq!(
                test_masks, expected_masks,
                "flag-test order for {flags:#010x}"
            );
        }
    }

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn map_game_globals(e: &mut Engine) {
        for page in [0x0101_6000, 0x0106_b000, 0x011d_6000, 0x011d_e000] {
            e.map(page, 0x1000);
        }
    }

    #[test]
    fn renderer_constructor_initialises_style_slots_maps_and_draw_flag() {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0020_0000);
        let renderer: Ptr<NavMeshRender> = e.new_object();
        e.register(NI_POINTER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(NI_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(NI_POINTER_DEREF, |e, a| ret(e.mem.u32(a[0])));
        e.register(NI_POINTER_DESTROY, |_, _| Ret::default());
        // No allocations are needed to verify the constructor's slot and
        // table setup; failed object allocations are valid constructor paths.
        e.register(NI_ALLOC, |_, _| ret(0));
        e.register(NI_ALLOC_ARRAY, |_, _| ret(0));
        for address in [
            EH_VECTOR_CONSTRUCTOR_ITERATOR,
            EH_VECTOR_DESTRUCTOR_ITERATOR,
            VECTOR_CONSTRUCTOR_ITERATOR,
            MEMSET,
            NI_ALPHA_PROPERTY_SET_BLENDING,
            NI_MATERIAL_SET_AMBIENT,
            NI_MATERIAL_SET_ALPHA,
            BS_SHADER_PROPERTY_SET_FLAG,
            NI_AV_OBJECT_ATTACH_PROPERTY,
            NI_AV_OBJECT_REMOVE_PROPERTY,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(NI_COLOR_A_CONSTRUCT, |_, a| ret(a[0]));
        e.register(MAKE_TRI_POINT, |_, _| ret(0));
        e.set_global(POINT_SHAPE_SIZE, 1.0f32);
        e.set_global(HALF, 0.5f32);

        assert_eq!(nav_mesh_render_nav_mesh_render(&mut e, renderer), renderer);
        assert_eq!(e.mem.u32(renderer.addr()), 0);
        assert_eq!(e.mem.u32(renderer.addr() + 0x1e8), 0);
        assert_eq!(e.mem.u32(renderer.addr() + 0x1ec), 0);
        assert!(!e.get(renderer, NavMeshRender::bDraw));
        for index in 0..STYLE_COUNT {
            assert_eq!(e.mem.u32(renderer.addr() + VERT_POINTS + 4 * index), 0);
            assert_eq!(
                e.mem
                    .u32(renderer.addr() + EDGE_AND_VERTEX_MATERIALS + 4 * index),
                0
            );
            assert_eq!(
                e.mem.u32(renderer.addr() + TRIANGLE_MATERIALS + 4 * index),
                0
            );
        }
        assert_eq!(
            e.mem.u32(renderer.addr() + NAV_MESH_NODES),
            VTABLE_NAV_MESH_NODE_MAP
        );
        assert_eq!(
            e.mem.u32(renderer.addr() + NAV_MESH_CELL_MAP),
            VTABLE_NAV_MESH_CELL_MAP
        );
    }

    fn pointer_dereference_double(e: &mut Engine) {
        e.register(NI_POINTER_DEREF, |e, a| ret(e.mem.u32(a[0])));
    }

    #[test]
    fn init_sdm_clears_the_singleton_when_allocation_fails() {
        let mut e = Engine::new();
        map_game_globals(&mut e);
        e.set_global(THE_RENDER, 0x1234u32);
        e.register(OPERATOR_NEW, |_, _| ret(0));
        nav_mesh_render_init_sdm(&mut e);
        assert_eq!(e.global::<u32>(THE_RENDER), 0);
    }

    #[test]
    fn shutdown_clears_an_already_null_singleton() {
        let mut e = Engine::new();
        map_game_globals(&mut e);
        e.set_global(THE_RENDER, 0u32);
        fn_006a29a0(&mut e);
        assert_eq!(e.global::<u32>(THE_RENDER), 0);
    }

    #[test]
    fn deleting_destructor_frees_only_when_flag_bit_zero_is_set() {
        let mut e = Engine::new();
        let object: Ptr<NavMeshRender> = e.new_object();
        e.call_log = Some(vec![]);
        e.register(OPERATOR_DELETE, |_, a| ret(a[0]));
        e.register(NI_POINTER_ASSIGN, |_, _| Ret::default());
        e.register(NI_POINTER_DESTROY, |_, _| Ret::default());
        e.register(EH_VECTOR_DESTRUCTOR_ITERATOR, |_, _| Ret::default());
        e.register(NI_T_MAP_REMOVE_ALL, |_, _| Ret::default());
        e.register(NAV_MESH_NODE_MAP_BASE_DESTROY, |_, _| Ret::default());
        e.register(NAV_MESH_CELL_MAP_BASE_DESTROY, |_, _| Ret::default());
        e.register(NAV_MESH_POINTER_DESTROY, |_, _| Ret::default());
        assert_eq!(fn_006a29f0(&mut e, object, 0), object);
        assert!(!e
            .call_log
            .as_ref()
            .is_some_and(|log| log.iter().any(|(a, _)| *a == OPERATOR_DELETE)));
        fn_006a29f0(&mut e, object, 1);
    }

    #[test]
    fn destructor_releases_all_style_slots_and_embedded_pointers() {
        let mut e = Engine::new();
        let object: Ptr<NavMeshRender> = e.new_object();
        e.call_log = Some(vec![]);
        e.register(NI_POINTER_ASSIGN, |_, _| Ret::default());
        pointer_dereference_double(&mut e);
        e.register(NI_POINTER_DESTROY, |_, _| Ret::default());
        e.register(EH_VECTOR_DESTRUCTOR_ITERATOR, |_, _| Ret::default());
        e.register(NI_T_MAP_REMOVE_ALL, |_, _| Ret::default());
        e.register(NAV_MESH_NODE_MAP_BASE_DESTROY, |_, _| Ret::default());
        e.register(NAV_MESH_CELL_MAP_BASE_DESTROY, |_, _| Ret::default());
        fn_006a3580(&mut e, object);
        assert_eq!(
            e.call_log
                .unwrap()
                .iter()
                .filter(|(address, _)| *address == NI_POINTER_ASSIGN)
                .count(),
            51
        );
    }

    #[test]
    fn set_draw_updates_the_root_and_stores_the_flag_for_both_states() {
        let mut e = Engine::new();
        map_game_globals(&mut e);
        let object: Ptr<NavMeshRender> = e.new_object();
        let root: Ptr = Ptr::new(e.mem.alloc(4));
        e.mem.set_u32(ROOT_DRAW_NODE, root.addr());
        pointer_dereference_double(&mut e);
        e.register(NI_NODE_CLEAR_CHILDREN, |_, _| Ret::default());
        e.register(NI_AV_OBJECT_UPDATE_PROPERTIES, |_, _| Ret::default());
        e.register(NI_UPDATE_DATA_CONSTRUCT, |_, _| Ret::default());
        e.register(NI_AV_OBJECT_UPDATE, |_, _| Ret::default());
        e.register(NI_T_MAP_REMOVE_ALL, |_, _| Ret::default());
        nav_mesh_render_set_draw(&mut e, object, false);
        assert!(!e.get(object, NavMeshRender::bDraw));
        nav_mesh_render_set_draw(&mut e, object, true);
        assert!(e.get(object, NavMeshRender::bDraw));
    }

    #[test]
    fn transparent_overlay_sets_each_material_alpha_and_recurse_flag() {
        let mut e = Engine::new();
        map_game_globals(&mut e);
        e.call_log = Some(vec![]);
        let object: Ptr<NavMeshRender> = e.new_object();
        e.map(0x0106_b000, 0x1000);
        e.set_global(0x0106_b4ec, 0.1f32);
        e.set_global(0x0106_b4f0, 0.2f32);
        e.set_global(0x0106_b4f8, 0.3f32);
        e.set_global(0x0106_b504, 0.4f32);
        e.set_global(0x0106_b500, 0.5f32);
        e.set_global(0x0106_b4fc, 0.6f32);
        e.set_global(0x0106_b508, 0.7f32);
        for slot in [
            0x94, 0xb0, 0xb4, 0xa4, 0xa8, 0x50, 0x68, 0x58, 0x6c, 0x74, 0x78,
        ] {
            e.mem.set_u32(object.addr() + slot, 0x1000 + slot);
        }
        let root: Ptr = Ptr::new(e.mem.alloc(4));
        e.mem.set_u32(ROOT_DRAW_NODE, root.addr());
        pointer_dereference_double(&mut e);
        e.register(NI_MATERIAL_SET_ALPHA, |_, _| Ret::default());
        e.register(SHADER_PROPERTY_SET_FLAG_RECURSE, |_, _| Ret::default());
        nav_mesh_render_set_transparent_over_lay_mode(&mut e, object, true);
        let recurse = e.call_log.as_ref().unwrap().last().unwrap();
        assert_eq!(recurse.0, SHADER_PROPERTY_SET_FLAG_RECURSE);
        assert_eq!(recurse.1[1..], [0x1f, 0]);
    }

    #[test]
    fn remove_draw_only_nodes_does_nothing_for_a_missing_cell() {
        let mut e = Engine::new();
        let object: Ptr<NavMeshRender> = e.new_object();
        let cell = Ptr::new(0x1234);
        e.call_log = Some(vec![]);
        e.register(NI_POINTER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(NI_T_MAP_GET_AT, |_, _| ret(0));
        e.register(NI_POINTER_DESTROY, |_, _| Ret::default());
        nav_mesh_render_remove_draw_only_nav_meshes(&mut e, object, cell);
        assert!(!e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .any(|(a, _)| *a == NI_T_MAP_REMOVE_AT));
    }

    #[test]
    fn detach_selected_geometry_releases_both_slots() {
        let mut e = Engine::new();
        let object: Ptr<NavMeshRender> = e.new_object();
        e.call_log = Some(vec![]);
        e.mem.set_u32(object.addr() + 4, 1);
        e.mem.set_u32(object.addr() + 8, 2);
        pointer_dereference_double(&mut e);
        e.register(NODE_PARENT, |_, _| ret(0));
        e.register(NI_POINTER_ASSIGN, |_, _| Ret::default());
        fn_006a61b0(&mut e, object);
        let cleared: Vec<u32> = e
            .call_log
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == NI_POINTER_ASSIGN)
            .map(|(_, args)| args[1])
            .collect();
        assert_eq!(cleared, [0, 0]);
    }

    #[test]
    fn redraw_with_no_current_cell_only_toggles_the_draw_state() {
        let mut e = Engine::new();
        map_game_globals(&mut e);
        let object: Ptr<NavMeshRender> = e.new_object();
        let root: Ptr = Ptr::new(e.mem.alloc(4));
        e.mem.set_u32(ROOT_DRAW_NODE, root.addr());
        e.set_global(TES_SINGLETON, 0u32);
        pointer_dereference_double(&mut e);
        e.register(NI_NODE_CLEAR_CHILDREN, |_, _| Ret::default());
        e.register(NI_AV_OBJECT_UPDATE_PROPERTIES, |_, _| Ret::default());
        e.register(NI_UPDATE_DATA_CONSTRUCT, |_, _| Ret::default());
        e.register(NI_AV_OBJECT_UPDATE, |_, _| Ret::default());
        e.register(NI_T_MAP_REMOVE_ALL, |_, _| Ret::default());
        e.register(TES_GET_CURRENT_CELL, |_, _| ret(0));
        fn_006a5ca0(&mut e, object);
        assert!(e.get(object, NavMeshRender::bDraw));
    }

    #[test]
    fn cover_height_saturates_at_fifteen_steps() {
        let mut e = Engine::new();
        map_game_globals(&mut e);
        e.set_global(BOX_HEIGHT_STEP, 16.0f32);
        assert_eq!(fn_006a5af0(&mut e, 0), 0.0);
        assert_eq!(fn_006a5af0(&mut e, 3), 48.0);
        assert_eq!(fn_006a5af0(&mut e, 99), 240.0);
    }

    #[test]
    fn triangle_vertex_and_neighbour_helpers_copy_all_three_indices() {
        let mut e = Engine::new();
        let triangle: Ptr = Ptr::new(e.mem.alloc(16));
        let outputs: [Ptr; 6] = std::array::from_fn(|_| Ptr::new(e.mem.alloc(2)));
        for (i, value) in [3u16, 7, 11, 13, 17, 19].into_iter().enumerate() {
            e.mem.set_u16(triangle.addr() + i as u32 * 2, value);
        }
        fn_006a5b30(&mut e, triangle, outputs[0], outputs[1], outputs[2]);
        fn_006a5b70(&mut e, triangle, outputs[3], outputs[4], outputs[5]);
        let got: Vec<u16> = outputs.iter().map(|p| e.mem.u16(p.addr())).collect();
        assert_eq!(got, [3, 7, 11, 13, 17, 19]);
    }

    #[test]
    fn triangle_flag_200_helper_passes_its_mask() {
        let mut e = Engine::new();
        let triangle = Ptr::new(0x1234);
        e.register(TRIANGLE_TEST_FLAGS, |_, a| ret(u32::from(a[1] == 0x200)));
        assert!(fn_006a5ec0(&mut e, triangle));
    }

    #[test]
    fn map_base_constructors_allocate_and_clear_the_bucket_table() {
        let mut e = Engine::new();
        let memory = Ptr::new(e.mem.alloc(16));
        let buckets = e.mem.alloc(12);
        e.register(NI_ALLOC_ARRAY, |_, a| ret(a[0]));
        e.register(MEMSET, |e, a| {
            e.mem.write(a[0], &vec![a[1] as u8; a[2] as usize]);
            Ret::default()
        });
        // The allocator result is a stable game pointer encoded by the size;
        // run each map constructor with its own real bucket allocation below.
        e.register_double(NI_ALLOC_ARRAY, move |_, _| ret(buckets));
        fn_006a6370(&mut e, memory, 3);
        assert_eq!(e.mem.u32(memory.addr()), VTABLE_NAV_MESH_NODE_MAP_BASE);
        assert_eq!(e.mem.u32(memory.addr() + 4), 3);
        assert_eq!(e.mem.u32(memory.addr() + 8), buckets);
        assert_eq!(e.mem.u32(memory.addr() + 0xc), 0);
        assert_eq!(e.mem.bytes(buckets, 12), vec![0; 12]);
    }

    #[test]
    fn map_constructors_install_the_derived_vtables() {
        let mut e = Engine::new();
        let first: Ptr = Ptr::new(e.mem.alloc(16));
        let second: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(NI_ALLOC_ARRAY, |e, a| ret(e.mem.alloc(a[0])));
        e.register(MEMSET, |e, a| {
            e.mem.write(a[0], &vec![a[1] as u8; a[2] as usize]);
            Ret::default()
        });
        fn_006a6290(&mut e, first, 2);
        fn_006a62c0(&mut e, second, 2);
        assert_eq!(e.mem.u32(first.addr()), VTABLE_NAV_MESH_NODE_MAP);
        assert_eq!(e.mem.u32(second.addr()), VTABLE_NAV_MESH_CELL_MAP);
    }

    #[test]
    fn map_destructors_empty_the_map_before_destroying_the_base() {
        let mut e = Engine::new();
        let map: Ptr = Ptr::new(e.mem.alloc(16));
        e.call_log = Some(vec![]);
        e.register(NAV_MESH_NODE_MAP_BASE_DESTROY, |_, _| Ret::default());
        fn_006a6450(&mut e, map);
        assert_eq!(e.mem.u32(map.addr()), VTABLE_NAV_MESH_NODE_MAP);
        let calls: Vec<u32> = e.call_log.unwrap().iter().map(|(a, _)| *a).collect();
        assert_eq!(calls, [NI_T_MAP_REMOVE_ALL, NAV_MESH_NODE_MAP_BASE_DESTROY]);
    }

    #[test]
    fn cell_map_destructor_installs_its_vtable_and_calls_both_bodies() {
        let mut e = Engine::new();
        let map: Ptr = Ptr::new(e.mem.alloc(16));
        e.call_log = Some(vec![]);
        e.register(NAV_MESH_CELL_MAP_BASE_DESTROY, |_, _| Ret::default());
        fn_006a6550(&mut e, map);
        assert_eq!(e.mem.u32(map.addr()), VTABLE_NAV_MESH_CELL_MAP);
        let calls: Vec<u32> = e.call_log.unwrap().iter().map(|(a, _)| *a).collect();
        assert_eq!(calls, [NI_T_MAP_REMOVE_ALL, NAV_MESH_CELL_MAP_BASE_DESTROY]);
    }

    #[test]
    fn scalar_deleting_destructor_for_map_only_frees_with_flag_bit_zero() {
        let mut e = Engine::new();
        let map: Ptr = Ptr::new(e.mem.alloc(16));
        e.call_log = Some(vec![]);
        e.register(NAV_MESH_NODE_MAP_BASE_DESTROY, |_, _| Ret::default());
        e.register(NAV_MESH_CELL_MAP_BASE_DESTROY, |_, _| Ret::default());
        e.register(NI_T_MAP_REMOVE_ALL, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, a| ret(a[0]));
        ni_t_map_nav_mesh_scalar_deleting_destructor(&mut e, map, 0);
        ni_t_map_cell_scalar_deleting_destructor(&mut e, map, 1);
        assert!(e
            .call_log
            .unwrap()
            .iter()
            .any(|(a, _)| *a == OPERATOR_DELETE));
    }

    #[test]
    fn scalar_deleting_destructors_for_base_maps_call_the_base_then_free() {
        let mut e = Engine::new();
        let map: Ptr = Ptr::new(e.mem.alloc(16));
        e.call_log = Some(vec![]);
        e.register(NAV_MESH_NODE_MAP_BASE_DESTROY, |_, _| Ret::default());
        e.register(NAV_MESH_CELL_MAP_BASE_DESTROY, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, a| ret(a[0]));
        ni_t_map_base_nav_mesh_scalar_deleting_destructor(&mut e, map, 1);
        ni_t_map_base_cell_scalar_deleting_destructor(&mut e, map, 0);
        assert_eq!(
            e.call_log
                .unwrap()
                .iter()
                .filter(|(a, _)| *a == OPERATOR_DELETE)
                .count(),
            1
        );
    }

    #[test]
    fn byte_array_element_address_adds_its_index_to_the_buffer() {
        let mut e = Engine::new();
        let array: Ptr = Ptr::new(e.mem.alloc(16));
        e.mem.set_u32(array.addr() + 4, 0x1234);
        assert_eq!(fn_006a62f0(&mut e, array, 7), 0x123b);
    }

    #[test]
    fn point_array_constructor_installs_its_vtable_and_forwards_count() {
        let mut e = Engine::new();
        let array: Ptr = Ptr::new(e.mem.alloc(16));
        e.call_log = Some(vec![]);
        e.register(POINT_ARRAY_CONSTRUCT_BODY, |_, a| ret(a[1]));
        assert_eq!(bs_simple_array_ni_point3_construct(&mut e, array, 9), array);
        assert_eq!(e.mem.u32(array.addr()), VTABLE_POINT_ARRAY);
        assert_eq!(e.call_log.unwrap()[0].1, [array.addr(), 9, 9]);
    }

    #[test]
    fn colour_array_constructor_allocates_and_constructs_its_elements() {
        let mut e = Engine::new();
        let array: Ptr = Ptr::new(e.mem.alloc(16));
        e.map(0x0106_b000, 0x1000);
        let allocator = 0x00f0_0100;
        e.mem.set_u32(VTABLE_COLOUR_ARRAY + 4, allocator);
        e.register(allocator, |e, a| ret(e.mem.alloc(a[1] * 16)));
        e.register(COLOUR_ARRAY_CONSTRUCT_ELEMENTS, |_, _| Ret::default());
        assert_eq!(
            bs_simple_array_ni_color_a_construct(&mut e, array, 3),
            array
        );
        assert_eq!(e.mem.u32(array.addr()), VTABLE_COLOUR_ARRAY);
        assert_eq!(e.mem.u32(array.addr() + 8), 3);
        assert_eq!(e.mem.u32(array.addr() + 0xc), 3);
    }

    #[test]
    fn byte_array_constructor_sets_its_vtable_and_calls_the_shared_body() {
        let mut e = Engine::new();
        let array: Ptr = Ptr::new(e.mem.alloc(16));
        e.call_log = Some(vec![]);
        e.register(BYTE_ARRAY_CONSTRUCT_BODY, |_, _| Ret::default());
        bs_simple_array_u8_construct(&mut e, array, 4);
        assert_eq!(e.mem.u32(array.addr()), VTABLE_BYTE_ARRAY);
        assert_eq!(e.call_log.unwrap()[0].1, [array.addr(), 4, 4]);
    }

    #[test]
    fn colour_array_destructor_restores_vtable_and_frees_its_buffer() {
        let mut e = Engine::new();
        let array: Ptr = Ptr::new(e.mem.alloc(16));
        e.call_log = Some(vec![]);
        e.register(SIMPLE_ARRAY_DESTROY_BODY, |_, _| Ret::default());
        fn_006a6640(&mut e, array);
        assert_eq!(e.mem.u32(array.addr()), VTABLE_COLOUR_ARRAY);
        assert_eq!(e.call_log.unwrap()[0].1, [array.addr(), 1]);
    }

    #[test]
    fn byte_array_destructor_restores_vtable_and_frees_its_buffer() {
        let mut e = Engine::new();
        let array: Ptr = Ptr::new(e.mem.alloc(16));
        e.call_log = Some(vec![]);
        e.register(SIMPLE_ARRAY_DESTROY_BODY, |_, _| Ret::default());
        fn_006a67d0(&mut e, array);
        assert_eq!(e.mem.u32(array.addr()), VTABLE_BYTE_ARRAY);
        assert_eq!(e.call_log.unwrap()[0].1, [array.addr(), 1]);
    }

    #[test]
    fn scalar_deleting_array_destructors_respect_the_delete_flag() {
        let mut e = Engine::new();
        let array: Ptr = Ptr::new(e.mem.alloc(16));
        e.call_log = Some(vec![]);
        e.register(OPERATOR_DELETE, |_, a| ret(a[0]));
        e.register(SIMPLE_ARRAY_DESTROY_BODY, |_, _| Ret::default());
        bs_simple_array_ni_color_a_scalar_deleting_destructor(&mut e, array, 0);
        bs_simple_array_u8_scalar_deleting_destructor(&mut e, array, 1);
        assert_eq!(
            e.call_log
                .unwrap()
                .iter()
                .filter(|(a, _)| *a == OPERATOR_DELETE)
                .count(),
            1
        );
    }

    #[test]
    fn colour_array_resize_to_zero_delegates_to_the_shared_destroyer() {
        let mut e = Engine::new();
        let array: Ptr = Ptr::new(e.mem.alloc(16));
        e.call_log = Some(vec![]);
        e.register(SIMPLE_ARRAY_DESTROY_BODY, |_, _| Ret::default());
        fn_006a6660(&mut e, array, 0, true);
        assert_eq!(e.call_log.unwrap()[0].1, [array.addr(), 1]);
    }

    #[test]
    fn add_draw_only_navmeshes_returns_for_null_cells_missing_arrays_and_existing_nodes() {
        let mut e = Engine::new();
        let renderer: Ptr<NavMeshRender> = e.new_object();
        let cell = Ptr::new(0x1234);
        e.call_log = Some(vec![]);
        nav_mesh_render_add_draw_only_nav_meshes(&mut e, renderer, Ptr::NULL, false);
        assert!(e.call_log.as_ref().unwrap().is_empty());

        e.register(CELL_NAV_MESH_ARRAY, |_, _| ret(0));
        nav_mesh_render_add_draw_only_nav_meshes(&mut e, renderer, cell, false);
        assert_eq!(e.call_log.as_ref().unwrap()[0].0, CELL_NAV_MESH_ARRAY);

        let array = 0x2000;
        e.register_double(CELL_NAV_MESH_ARRAY, move |_, _| ret(array));
        e.register(NAV_MESH_ARRAY_COUNT, |_, _| ret(1));
        e.register(NI_POINTER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(NI_T_MAP_GET_AT, |_, _| ret(1));
        e.register(NI_POINTER_DESTROY, |_, _| Ret::default());
        nav_mesh_render_add_draw_only_nav_meshes(&mut e, renderer, cell, false);
        assert!(e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .any(|(a, _)| *a == NI_T_MAP_GET_AT));
        assert!(!e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .any(|(a, _)| *a == NI_ALLOC));
    }

    #[test]
    fn add_draw_only_navmeshes_creates_a_cell_node_and_updates_when_requested() {
        let mut e = Engine::new();
        map_game_globals(&mut e);
        e.call_log = Some(vec![]);
        let renderer: Ptr<NavMeshRender> = e.new_object();
        let cell = Ptr::new(0x1234);
        let root: Ptr = Ptr::new(e.mem.alloc(4));
        let vtable = e.mem.alloc(0x100);
        let attach = 0x00f0_0200;
        e.mem.set_u32(ROOT_DRAW_NODE, root.addr());
        e.mem.set_u32(root.addr(), vtable);
        e.mem.set_u32(vtable + SLOT_ATTACH_CHILD, attach);
        e.register(CELL_NAV_MESH_ARRAY, |_, _| ret(0x2000));
        e.register(NAV_MESH_ARRAY_COUNT, |_, _| ret(0));
        e.register(NI_POINTER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(NI_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(NI_POINTER_DEREF, |e, a| ret(e.mem.u32(a[0])));
        e.register(NI_POINTER_COPY, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            Ret::default()
        });
        e.register(NI_POINTER_DESTROY, |_, _| Ret::default());
        e.register(NAV_MESH_POINTER_CONSTRUCT, |_, _| Ret::default());
        e.register(NAV_MESH_POINTER_DESTROY, |_, _| Ret::default());
        e.register(NI_T_MAP_GET_AT, |_, _| ret(0));
        e.register(NI_T_MAP_SET_AT, |_, _| Ret::default());
        e.register(NI_ALLOC, |e, a| ret(e.mem.alloc(a[0])));
        e.register(NI_NODE_CONSTRUCT, |_, a| ret(a[0]));
        e.register(attach, |_, _| Ret::default());
        e.register(NI_UPDATE_DATA_CONSTRUCT, |_, _| Ret::default());
        e.register(NI_AV_OBJECT_UPDATE, |_, _| Ret::default());
        e.register(NI_AV_OBJECT_UPDATE_PROPERTIES, |_, _| Ret::default());

        nav_mesh_render_add_draw_only_nav_meshes(&mut e, renderer, cell, true);
        let calls = e.call_log.unwrap();
        assert!(calls.iter().any(|(address, _)| *address == attach));
        assert!(calls.iter().any(|(address, _)| *address == NI_T_MAP_SET_AT));
        assert!(calls
            .iter()
            .any(|(address, _)| *address == NI_AV_OBJECT_UPDATE));
        assert!(calls
            .iter()
            .any(|(address, _)| *address == NI_AV_OBJECT_UPDATE_PROPERTIES));
    }

    #[test]
    fn add_draw_only_navmeshes_builds_triangle_indices_and_line_endpoints() {
        let mut e = Engine::new();
        map_game_globals(&mut e);
        e.map(0x0106_b000, 0x1000);
        e.call_log = Some(vec![]);

        let renderer: Ptr<NavMeshRender> = e.new_object();
        let cell = Ptr::new(0x1234);
        let mesh: Ptr = Ptr::new(e.mem.alloc(0x100));
        let triangle: Ptr = Ptr::new(e.mem.alloc(16));
        let vertices = e.mem.alloc(3 * 12);
        let root: Ptr = Ptr::new(e.mem.alloc(4));
        let root_vtable = e.mem.alloc(0x100);
        let node_vtable = e.mem.alloc(0x100);
        let attach = 0x00f0_0300;
        e.mem.set_u32(ROOT_DRAW_NODE, root.addr());
        e.mem.set_u32(root.addr(), root_vtable);
        e.mem.set_u32(root_vtable + SLOT_ATTACH_CHILD, attach);
        e.mem.set_u32(node_vtable + SLOT_ATTACH_CHILD, attach);
        e.mem.set_u32(mesh.addr() + NAV_MESH_VERTICES + 4, vertices);
        e.mem.set_u32(mesh.addr() + NAV_MESH_VERTICES + 8, 3);
        e.mem.set_u32(mesh.addr() + NAV_MESH_VERTICES + 0xc, 3);
        for (i, point) in [[0.0f32, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]]
            .into_iter()
            .enumerate()
        {
            for (axis, coordinate) in point.into_iter().enumerate() {
                e.mem
                    .set_f32(vertices + (i * 12 + axis * 4) as u32, coordinate);
            }
            e.mem.set_u16(triangle.addr() + (i * 2) as u32, i as u16);
            e.mem
                .set_u16(triangle.addr() + 6 + (i * 2) as u32, u16::MAX);
        }
        e.mem.set_u32(triangle.addr() + 0xc, 0);
        e.mem.set_u32(VTABLE_COLOUR_ARRAY + 4, 0x00f0_0310);

        e.register(CELL_NAV_MESH_ARRAY, |_, _| ret(0x2000));
        e.register(NAV_MESH_ARRAY_COUNT, |_, _| ret(1));
        e.register_double(NAV_MESH_ARRAY_GET, move |_, _| ret(mesh.addr()));
        e.register(NAV_MESH_TRIANGLE_COUNT, |_, _| ret(1));
        e.register(NAV_MESH_VERTEX_COUNT, |_, _| ret(3));
        e.register(NAV_MESH_IS_SPECIAL, |_, _| ret(0));
        e.register_double(NAV_MESH_TRIANGLE_AT, move |_, _| ret(triangle.addr()));
        e.register(TRIANGLE_FLAGS_SPECIAL_KIND, |_, _| ret(0));
        e.register(TRIANGLE_TEST_FLAGS, |_, _| ret(0));
        e.register(TRIANGLE_HAS_FLAG_0X40, |_, _| ret(0));
        e.register(TRIANGLE_HAS_FLAG_0X10, |_, _| ret(0));
        e.register(TRIANGLE_HAS_FLAG_0X400, |_, _| ret(0));
        e.register(TRIANGLE_EDGE_FLAG, |_, _| ret(0));
        e.register(TRIANGLE_BORDER_INFO, |_, _| Ret::default());
        e.register(POINT_ARRAY_CONSTRUCT_BODY, |e, a| {
            let buffer = e.mem.alloc(a[1] * 12);
            e.mem.set_u32(a[0] + 4, buffer);
            e.mem.set_u32(a[0] + 8, a[2]);
            e.mem.set_u32(a[0] + 0xc, a[1]);
            ret(a[0])
        });
        e.register(BYTE_ARRAY_CONSTRUCT_BODY, |e, a| {
            let buffer = e.mem.alloc(a[1]);
            e.mem.set_u32(a[0] + 4, buffer);
            e.mem.set_u32(a[0] + 8, a[2]);
            e.mem.set_u32(a[0] + 0xc, a[1]);
            ret(a[0])
        });
        e.register(COLOUR_ARRAY_CONSTRUCT_ELEMENTS, |_, _| Ret::default());
        e.register(0x00f0_0310, |e, a| ret(e.mem.alloc(a[1] * 16)));
        e.register(POINT_ARRAY_AT, |e, a| ret(e.mem.u32(a[0] + 4) + a[1] * 12));
        e.register(COLOUR_ARRAY_AT, |e, a| ret(e.mem.u32(a[0] + 4) + a[1] * 16));
        e.register(NI_ALLOC_ARRAY, |e, a| ret(e.mem.alloc(a[0])));
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        e.register(NI_ALLOC, |e, a| ret(e.mem.alloc(a[0])));
        e.register(MEMCPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            ret(a[0])
        });
        e.register_double(NI_NODE_CONSTRUCT, move |e, a| {
            e.mem.set_u32(a[0], node_vtable);
            ret(a[0])
        });
        let shape_indices = Rc::new(RefCell::new(None));
        let captured_indices = Rc::clone(&shape_indices);
        e.register_double(NI_TRI_SHAPE_CONSTRUCT, move |e, a| {
            let indices = [e.mem.u16(a[9]), e.mem.u16(a[9] + 2), e.mem.u16(a[9] + 4)];
            *captured_indices.borrow_mut() = Some(indices);
            ret(a[0])
        });
        let line_endpoints = Rc::new(RefCell::new(None));
        let captured_endpoints = Rc::clone(&line_endpoints);
        e.register_double(NI_LINES_CONSTRUCT, move |e, a| {
            let coordinates = (0..6)
                .map(|index| {
                    let point = a[2] + index * 12;
                    [e.mem.f32(point), e.mem.f32(point + 4), e.mem.f32(point + 8)]
                })
                .collect::<Vec<_>>();
            *captured_endpoints.borrow_mut() = Some(coordinates);
            ret(a[0])
        });
        e.register(VECTOR_CONSTRUCTOR_ITERATOR, |_, _| Ret::default());
        e.register(NI_POINT3_CONSTRUCT, |_, a| ret(a[0]));
        e.register(NI_POINT3_CONSTRUCT_XYZ, |_, a| ret(a[0]));
        e.register(NI_COLOR_A_CONSTRUCT, |_, a| ret(a[0]));
        e.register(NI_FIXED_STRING_CONSTRUCT, |_, _| Ret::default());
        e.register(NI_FIXED_STRING_DESTROY, |_, _| Ret::default());
        e.register(NI_AV_OBJECT_SET_NAME, |_, _| Ret::default());
        e.register(NI_AV_OBJECT_SET_TRANSLATE, |_, _| Ret::default());
        e.register(NI_AV_OBJECT_ATTACH_PROPERTY, |_, _| Ret::default());
        e.register(NI_AV_OBJECT_REMOVE_PROPERTY, |_, _| Ret::default());
        e.register(NI_AV_OBJECT_UPDATE, |_, _| Ret::default());
        e.register(NI_AV_OBJECT_UPDATE_PROPERTIES, |_, _| Ret::default());
        e.register(BS_SHADER_NO_LIGHTING_CONSTRUCT, |_, a| ret(a[0]));
        e.register(BS_SHADER_PROPERTY_SET_FLAG, |_, _| Ret::default());
        e.register(ADD_NO_LIGHTING_PROPERTY_RECURSE, |_, _| Ret::default());
        e.register(SHADER_MANAGER_PREPARE_OBJECT, |_, _| Ret::default());
        e.register(NI_POINTER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(NI_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(NI_POINTER_DEREF, |e, a| ret(e.mem.u32(a[0])));
        e.register(NI_POINTER_COPY, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            Ret::default()
        });
        e.register(NI_POINTER_DESTROY, |_, _| Ret::default());
        e.register(NAV_MESH_POINTER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a.get(1).copied().unwrap_or(0));
            Ret::default()
        });
        e.register(NAV_MESH_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(NAV_MESH_POINTER_DESTROY, |_, _| Ret::default());
        e.register(POINT_ARRAY_DESTRUCTOR, |_, _| Ret::default());
        e.register(SIMPLE_ARRAY_DESTROY_BODY, |_, _| Ret::default());
        e.register(NI_T_MAP_GET_AT, |_, _| ret(0));
        e.register(NI_T_MAP_SET_AT, |_, _| Ret::default());
        e.register(NI_UPDATE_DATA_CONSTRUCT, |_, _| Ret::default());
        e.register(attach, |_, _| Ret::default());

        nav_mesh_render_add_draw_only_nav_meshes(&mut e, renderer, cell, false);
        let calls = e.call_log.unwrap();
        let shape = calls
            .iter()
            .find(|(address, _)| *address == NI_TRI_SHAPE_CONSTRUCT)
            .expect("one triangle shape is constructed");
        assert_eq!(shape.1[1], 3, "three source vertices");
        assert_eq!(shape.1[8], 1, "one source triangle");
        assert_eq!(*shape_indices.borrow(), Some([0, 1, 2]));
        assert_eq!(
            calls
                .iter()
                .filter(|(address, _)| *address == NI_LINES_CONSTRUCT)
                .count(),
            1,
            "the border line set contains the three triangle edges"
        );
        let lines = calls
            .iter()
            .find(|(address, _)| *address == NI_LINES_CONSTRUCT)
            .unwrap();
        assert_eq!(lines.1[1], 6, "two endpoints per triangle edge");
        assert_eq!(
            *line_endpoints.borrow(),
            Some(vec![
                [0.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0],
            ]),
            "border lines connect the three source triangle edges in order"
        );
    }
}
