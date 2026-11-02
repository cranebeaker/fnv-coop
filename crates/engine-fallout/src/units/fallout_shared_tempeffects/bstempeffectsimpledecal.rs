//! `fallout shared/tempeffects/bstempeffectsimpledecal.cpp` (Xbox PDB source unit), subsystem `fallout shared/tempeffects`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `BSTempEffectSimpleDecal` (Xbox PDB) projects a decal onto a target
//! geometry: the constructor copies the placement from a creation-info
//! structure, `ProcessGeometry` builds the clipping planes and the texture
//! coordinates, `ClipMesh` clips the target's triangles against the six
//! planes into scrap-heap arrays, and `FinalizeGeometry` turns them into a
//! `NiTriShape` with its properties and hangs it under the target's parent.
//!
//! x87 note: the game computes in extended precision and stores `float`
//! results. The translations compute in `f64` and round to `f32` where the
//! code stores a `float`; the two can differ only in the last bit of a
//! result rounded twice.
//!
//! Callee names below follow what the callee's body shows in the exe; the
//! engine map's names are only used where marked.
//!
//! Empty constructors: `006815c0` returns `this` and does nothing; the code
//! calls it for every `NiPoint3`/`NiMatrix3` local and as the element
//! constructor of the array constructor `00401050`. The translations call it
//! the same way.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{BSSimpleArray, NiPlane, NiPoint3};

/// The class's vtable.
const VTABLE: u32 = 0x0106_9ddc;
/// The class's `NiRTTI` (`ms_RTTI`, returned by `GetRTTI`).
const RTTI: u32 = 0x011d_6bf0;
/// Size of a `BSTempEffectSimpleDecal` on PC (the `operator delete` size).
const OBJECT_SIZE: u32 = 0x144;
/// Vtable of the `BSSimpleArray<DecalVertex *, 1024>` that `+0xF8` embeds.
const DECAL_VERTEX_ARRAY_VTABLE: u32 = 0x0106_a038;

// ---- statics -----------------------------------------------------------

/// `static NiPointer<QueuedFile> spPreloadedDecalTextureSets` (Xbox PDB name;
/// the address is the one the preloading function `0068b3e0` fills).
const PRELOADED_TEXTURE_SETS: u32 = 0x011d_6c40;
/// Number of decals in [`DECAL_LIST`] (incremented by `FinalizeGeometry`,
/// decremented when the oldest is dropped).
const LISTED_DECAL_COUNT: u32 = 0x011d_6a20;
/// The list of decals (`BSSimpleList<NiPointer<BSTempEffect>>`, first item
/// word first) that `FinalizeGeometry` adds to.
const DECAL_LIST: u32 = 0x011d_6a30;
/// Counter that the destructor decrements when `bValidDecal` is set.
const VALID_DECAL_COUNT: u32 = 0x011c_57ec;
/// Decal limit: `-1` until the first constructor reads it through
/// [`SETTING_VALUE_GETTER`].
const MAX_DECAL_COUNT: u32 = 0x0119_c298;
/// The `float` the constructor computes from the creation info's angle and
/// that `ClipMesh` and the vertex loop of `fn_0068cb60` compare against.
const CLIP_ANGLE_LIMIT: u32 = 0x0119_c29c;
/// Flag word whose bit 0x20 selects the debug colouring in
/// `FinalizeGeometry` (the name `iDecalDebugFlags` is the Xbox PDB's
/// `BGSDecalManager` static; the address assignment is not confirmed).
const DECAL_DEBUG_FLAGS: u32 = 0x011c_57f4;
/// Setting object whose value `ProcessGeometry` and `FinalizeGeometry` read
/// (through [`SETTING_BYTE_GETTER`]) before printing diagnostics.
const DIAGNOSTICS_SETTING: u32 = 0x011c_5904;
/// Setting object the constructor reads the decal limit from.
const MAX_DECAL_SETTING: u32 = 0x011c_57fc;
/// `NiPointer<NiAVObject>` holding the debug decal box (the Xbox PDB's
/// `BGSDecalManager::spDebugDecalBox`; the address is the one
/// `CreateDebugBox` fills).
const DEBUG_DECAL_BOX: u32 = 0x011c_58e0;
/// A `NiPoint3` global (all callers pass it as the translation of a
/// matrix transform; its contents are not confirmed here).
const ZERO_POINT: u32 = 0x011f_426c;
/// The `NiRTTI` that `ProcessGeometry` requires of the target geometry
/// (it is the one the class at `00a71d20` returns from its `GetRTTI`).
const GEOMETRY_RTTI: u32 = 0x011f_4a20;
/// The `NiRTTI` of the node class that `FinalizeGeometry` looks for among
/// the target parent's children.
const DECAL_NODE_RTTI: u32 = 0x011c_7d34;
/// Handle word used as the name of a shape (`FinalizeGeometry`, permanent
/// decals), see [`NAME_GETTER_PERMANENT`].
const PERMANENT_NAME_WORD: u32 = 0x011c_622c;
/// The `TES` singleton pointer.
const TES_SINGLETON: u32 = 0x011d_ea10;
/// The process lists the decals are added to.
const PROCESS_LISTS: u32 = 0x011e_0e80;
/// Alignment word the scrap-heap arrays pass to `Allocate`.
const SCRAP_ALIGNMENT: u32 = 0x010a_2720;
/// Static `float` in `FinalizeGeometry`: argument of the call at `0043d410`.
const ORIGIN_X: u32 = 0x011c_3c08;

// ---- constants read from the exe's data --------------------------------

/// `0.5` (`double`).
const HALF: u32 = 0x0101_1588;
/// `2.0` (`double`).
const TWO: u32 = 0x0101_1590;
/// `0.0` (`double`).
const ZERO: u32 = 0x0101_2060;
/// `1.0` (`double`).
const ONE: u32 = 0x0101_2070;
/// `1.0001` (`double`): tolerance on the clip parameter.
const CLIP_PARAMETER_TOLERANCE: u32 = 0x0106_9ff0;
/// `pi / 180` (`double`).
const DEGREES_TO_RADIANS: u32 = 0x0102_3128;
/// `0.01` (`double`): lower bound of the clip angle limit.
const CLIP_ANGLE_FLOOR: u32 = 0x0103_1148;
/// Weld distance (`float`) for `DecalVertex` lookup in `fn_0068cb60`.
const WELD_DISTANCE: u32 = 0x0101_3ea4;
/// Plane-component tolerance (`float`) in `CreateDebugBox`.
const DEBUG_PLANE_TOLERANCE: u32 = 0x0101_7d00;
/// Lifetime (`float`) of the debug box when it is added as a debug object.
const DEBUG_BOX_LIFETIME: u32 = 0x0101_8f5c;

// ---- strings -----------------------------------------------------------

/// `"DECAL: No Decal for : %s"`.
const NO_DECAL_MESSAGE: u32 = 0x0106_9efc;
/// `"DECAL: Reached Max Decal Limit of %d! Removing oldest decal..."`.
const DECAL_LIMIT_MESSAGE: u32 = 0x0106_9f18;
/// `"TEXTURES: Decal Parent '%s' has no normals!"`.
const NO_NORMALS_MESSAGE: u32 = 0x0106_9f58;
/// `"BSTempEffectSimpleDecal: Bad t value (%f) calculated while attempting
/// to clip decal.  See Jeff S."`.
const BAD_T_MESSAGE: u32 = 0x0106_9f88;
/// `"DebugDecalBox01:0"`.
const DEBUG_BOX_SHAPE_NAME: u32 = 0x0106_9ff8;
/// `"Data\Meshes\Decals\DebugDecalBox.NIF"`.
const DEBUG_BOX_PATH: u32 = 0x0106_a00c;
/// `"Debug Decal Box"`.
const DEBUG_BOX_NAME: u32 = 0x0101_e450;
/// The source file name the preloading function passes to its scope guard.
const SOURCE_FILE_NAME: u32 = 0x0106_9ea0;

// ---- callees (identified by their bodies unless marked) ------------------

/// `NiPointer` slot reader: returns the word at `this`.
const NI_POINTER_GET: u32 = 0x0055_9450;
/// `NiPointer::operator=(T*)`: releases the old, stores, adds a reference.
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// `NiPointer` constructor from a pointer (stores it, adds a reference).
const NI_POINTER_CONSTRUCT: u32 = 0x0063_3c90;
/// `NiPointer` destructor (releases a non-null pointer).
const NI_POINTER_RELEASE: u32 = 0x0045_cec0;
/// Empty constructor: returns `this`.
const EMPTY_CONSTRUCTOR: u32 = 0x0068_15c0;
/// Constructor storing three floats (`NiPoint3`, also `NiColor`).
const POINT3_CONSTRUCT: u32 = 0x0041_6870;
/// Constructor storing four floats (`NiColorA`).
const COLOR4_CONSTRUCT: u32 = 0x0041_4430;
/// Constructor storing two words (`NiPoint2`).
const POINT2_CONSTRUCT: u32 = 0x0045_2dc0;
/// `NiPlane(const NiPoint3 &normal, float constant)`.
const PLANE_CONSTRUCT: u32 = 0x00a6_9970;
/// Plane element constructor used by the array constructor in the class
/// constructor.
const PLANE_ELEMENT_CONSTRUCTOR: u32 = 0x00a6_9940;
/// `cdecl` array constructor: `(array, element size, count, constructor)`.
const VECTOR_CONSTRUCTOR: u32 = 0x0040_1050;
/// `a - b` for `NiPoint3`: `this = a`, then `(result, b)`; returns `result`.
const POINT3_SUBTRACT: u32 = 0x0043_9ef0;
/// `a + b` for `NiPoint3`: `this = a`, then `(result, b)`; returns `result`.
const POINT3_ADD: u32 = 0x0043_9e90;
/// `a * s` for `NiPoint3`: `this = a`, then `(result, s)`; returns `result`.
const POINT3_SCALE: u32 = 0x0045_bb20;
/// `a x b`: `this = a`, then `(result, b)`; returns `result`.
const POINT3_CROSS: u32 = 0x004b_3800;
/// Dot product: `this = a`, then `(b)`, result in `ST0`.
const POINT3_DOT: u32 = 0x004b_6190;
/// Length of `this`, result in `ST0`.
const POINT3_LENGTH: u32 = 0x0045_7990;
/// Compares the three components of `this` with those at the argument
/// (`FUCOMPP` each), returns whether all are equal.
const POINT3_EQUALS: u32 = 0x0043_90c0;
/// Whether the first float equals the second within the third.
const FLOAT_NEARLY_EQUAL: u32 = 0x0040_eb10;
/// `NiMatrix3` inverse: `this`, then `(result)`.
const MATRIX_INVERSE: u32 = 0x004b_45b0;
/// `NiMatrix3 * NiPoint3`: `this`, then `(result, point)`.
const MATRIX_TRANSFORM_POINT: u32 = 0x004b_4500;
/// `NiMatrix3 * NiMatrix3`: `this`, then `(result, other)`.
const MATRIX_MULTIPLY: u32 = 0x0043_f8d0;
/// `cdecl` `(matrix, translation, count, source points, destination points)`.
const MATRIX_TRANSFORM_VERTICES: u32 = 0x00a5_82f0;
/// Stores the rotation about Z for an angle: `this`, then `(angle)`.
const MATRIX_MAKE_Z_ROTATION: u32 = 0x004a_0c90;
/// `cdecl` `(result matrix, x, y, z)`: rotation taking Z to the vector.
const ROTATION_Z_TO_VECTOR: u32 = 0x00c4_b600;
/// Same with the transpose.
const ROTATION_Z_TO_VECTOR_TRANSPOSE: u32 = 0x00c4_b8a0;
/// Copies row `index` of a matrix: `this`, then `(index, result)`.
const MATRIX_GET_ROW: u32 = 0x0047_6930;
/// `this = plane`, then `(point)`: 0 on the plane, 1 in front, 2 behind.
const PLANE_SIDE: u32 = 0x0049_da80;
/// The constant of a plane (`this + 0xC`), result in `ST0`.
const PLANE_CONSTANT: u32 = 0x0084_d030;
/// `this` and nothing else (a getter for the normal at the plane's start).
const PLANE_NORMAL: u32 = 0x0068_15c0;
/// Writes a formatted line (`cdecl`, variable arguments).
const PRINT_LINE: u32 = 0x005b_5e40;
/// Reads the byte setting at `this + 4`, or a zero byte when `this` is null.
const SETTING_BYTE_GETTER: u32 = 0x0040_8d60;
/// Reads the setting value at the argument's address.
const SETTING_VALUE_GETTER: u32 = 0x0043_d4d0;
/// `operator new` of the memory manager (`cdecl`, size).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// `operator new` of the game's runtime (`cdecl`, size).
const RUNTIME_NEW: u32 = 0x0040_1000;
/// `operator delete` (`cdecl`, pointer).
const RUNTIME_DELETE: u32 = 0x0040_1030;
/// Sized delete of the memory manager (`cdecl`, pointer, size).
const SIZED_DELETE: u32 = 0x00aa_1460;
/// Allocation with the memory manager (`cdecl`, size).
const MEMORY_ALLOCATE: u32 = 0x00aa_1070;
/// Free of such an allocation (`cdecl`, pointer).
const MEMORY_FREE: u32 = 0x00aa_10f0;
/// The `MemoryManager` singleton's address.
const MEMORY_MANAGER_GETTER: u32 = 0x0040_1020;
/// Thread scrap heap of the memory manager (`this` = manager).
const THREAD_SCRAP_HEAP: u32 = 0x00aa_42e0;
/// Scrap heap `Allocate(this = heap, size, alignment)`.
const SCRAP_ALLOCATE: u32 = 0x00aa_54a0;
/// Scrap heap `Deallocate(this = heap, pointer)`.
const SCRAP_DEALLOCATE: u32 = 0x00aa_5610;
/// `memcpy` wrapper (`cdecl`, destination, source, size).
const MEMORY_COPY: u32 = 0x0040_1460;
/// `memset` wrapper (`cdecl`, destination, value, size).
const MEMORY_FILL: u32 = 0x0040_3d30;
/// `cdecl` `(rtti, object)`: whether the object's class chain contains it.
const IS_KIND_OF: u32 = 0x0043_b300;
/// Same for a child (`cdecl`, rtti, object).
const IS_KIND_OF_CHILD: u32 = 0x0045_bad0;
/// Local rotation of a node (`this + 0x34`, the first matrix word).
const NODE_LOCAL_ROTATE: u32 = 0x006a_9540;
/// World rotation of a node (`this + 0x68`).
const NODE_WORLD_ROTATE: u32 = 0x0046_1130;
/// World translation of a node (`this + 0x8C`).
const NODE_WORLD_TRANSLATE: u32 = 0x0045_bb80;
/// World scale of a node (`this + 0x98`), result in `ST0`.
const NODE_WORLD_SCALE: u32 = 0x008d_01e0;
/// Local translation of a node (`this + 0x58`).
const NODE_LOCAL_TRANSLATE: u32 = 0x0043_c490;
/// Local scale of a node, result in `ST0` (`this + 0x34 + ...`).
const NODE_LOCAL_SCALE: u32 = 0x009c_dae0;
/// Parent of a node (`this + 0x18`).
const NODE_PARENT: u32 = 0x0096_11e0;
/// `this + 8`, the name of a node (its address, for the name setter).
const SET_NAME: u32 = 0x00a5_b950;
/// Attaches a property to a node: `this = node`, then `(property)`.
const ATTACH_PROPERTY: u32 = 0x0043_9410;
/// Sets the local translation of a node from the pointed point.
const NODE_SET_TRANSLATE: u32 = 0x0044_0460;
/// Sets the local rotation of a node from the pointed matrix.
const NODE_SET_ROTATE: u32 = 0x0043_fa80;
/// Sets the local scale of a node.
const NODE_SET_SCALE: u32 = 0x0044_0490;
/// Node-sized setter used with a point built by `0043d410`.
const NODE_SET_WORLD_POINT: u32 = 0x00a5_9c60;
/// Constructor of that point: `(x, y, z)`.
const WORLD_POINT_CONSTRUCT: u32 = 0x0043_d410;
/// `this + 0x24` of the node, the list of its properties, emptied.
const PROPERTY_LIST_CLEAR: u32 = 0x004a_4650;
/// `UpdateProperties` of a node.
const NODE_UPDATE_PROPERTIES: u32 = 0x00a5_a040;
/// Property of a given type of a node: `this`, then `(type)`.
const NODE_GET_PROPERTY: u32 = 0x00a5_9d30;
/// The property type `Update` looks up (constant function returning 2).
const MATERIAL_PROPERTY_TYPE: u32 = 0x004a_f350;
/// Float `this + 0x3C` of the property found by [`NODE_GET_PROPERTY`].
const MATERIAL_GET_ALPHA: u32 = 0x0059_8040;
/// Removes a decal from its parent: `this = parent`, then `(decal)`.
const PARENT_REMOVE_DECAL: u32 = 0x004e_e560;
/// Fade-in/out age of the base effect (`this + 0x10`), result in `ST0`.
const EFFECT_AGE: u32 = 0x0062_1b00;
/// Lifetime of the base effect (`this + 8`), result in `ST0`.
const EFFECT_LIFETIME: u32 = 0x0048_8d50;
/// Base class `Update(this, delta)` (age += delta; true while it is alive).
const BASE_UPDATE: u32 = 0x0068_28a0;
/// Base class `Initialize(this)`.
const BASE_INITIALIZE: u32 = 0x0068_2810;
/// Base class constructor: `(this, cell, lifetime)`.
const BASE_CONSTRUCT: u32 = 0x0068_27b0;
/// Base class destructor body.
const BASE_DESTRUCT: u32 = 0x0068_2860;
/// `ProcessLists::AddTempEffect` (engine map name, confirmed by the body: it
/// sorts the effect into one of two lists by its type).
const ADD_TEMP_EFFECT: u32 = 0x0097_3fd0;
/// List add: `this = list`, then `(NiPointer *)`.
const LIST_ADD: u32 = 0x0052_a9b0;
/// List remove-head: `this = list`.
const LIST_REMOVE_HEAD: u32 = 0x004e_e8a0;
/// `00bb4520`: constructor of the 0x108-byte lighting property.
const LIGHTING_PROPERTY_CONSTRUCT: u32 = 0x00bb_4520;
/// `00b6fc90`: constructor of the 0x80-byte no-lighting property.
const NO_LIGHTING_PROPERTY_CONSTRUCT: u32 = 0x00b6_fc90;
/// Shader property flag setter: `this`, then `(flag, value)`.
const SHADER_SET_FLAG: u32 = 0x0044_1130;
/// `cdecl` `(object, 0, 0)`: prepares a node for rendering.
const PREPARE_OBJECT: u32 = 0x00b5_7e30;
/// `cdecl` `(object, type)`: sets the shader object type recursively.
const SET_OBJECT_TYPE_RECURSE: u32 = 0x00b5_5360;
/// `cdecl` `(geometry)`: removes the Havok objects below a node.
const REMOVE_HAVOK: u32 = 0x0062_bc10;
/// `cdecl` `(geometry)`: removes a geometry from the property lights.
const REMOVE_FROM_PROPERTY_LIGHTS: u32 = 0x00b5_b0e0;
/// Returns the table entry for its argument (`cdecl`).
const SHADOW_SCENE_NODE_GETTER: u32 = 0x0045_0b80;
/// `this = shadow scene node`, then `(object, flag)`.
const UPDATE_OBJECT_LIGHTING: u32 = 0x00b5_d9f0;
/// `NiPointer<QueuedFile>::operator=` (engine map name, not confirmed): the
/// same shape as [`NI_POINTER_ASSIGN`] for the preloaded texture set slot.
const QUEUED_FILE_POINTER_ASSIGN: u32 = 0x006f_74f0;
/// Constructor of the `QueuedFile` (0x28 bytes) the preloading builds:
/// `this`, then `(5)`.
const QUEUED_FILE_CONSTRUCT: u32 = 0x00c3_c590;
/// Scope guard around the preloading (`this`, then `(7, 1, file, line)`) and
/// its destructor.
const SCOPE_GUARD_CONSTRUCT: u32 = 0x0040_4eb0;
const SCOPE_GUARD_DESTRUCT: u32 = 0x0040_4ee0;
/// Constructor of the 0x4C-byte material property `FinalizeGeometry`
/// builds, and the setter it calls with `fSpecular` (`this`, `(float)`).
const MATERIAL_PROPERTY_CONSTRUCT: u32 = 0x00a7_5650;
const MATERIAL_SET_SPECULAR: u32 = 0x0065_4700;
/// Constructor of the 0x1C-byte alpha property and its setters: blending
/// (`this`, `(flag)`), testing, test function and threshold.
const ALPHA_PROPERTY_CONSTRUCT: u32 = 0x0043_91c0;
const ALPHA_SET_BLENDING: u32 = 0x0049_ed90;
const ALPHA_SET_TESTING: u32 = 0x0043_93c0;
const ALPHA_SET_TEST_FUNCTION: u32 = 0x0043_93e0;
const ALPHA_SET_THRESHOLD: u32 = 0x0094_db80;
/// Constructor of the 0x24-byte property added for two-sided decals and its
/// setter (`this`, `(3)`).
const TWO_SIDED_PROPERTY_CONSTRUCT: u32 = 0x0049_ec80;
const TWO_SIDED_SET_MODE: u32 = 0x0049_ee30;
/// Setter called on the 0x108-byte lighting property right after its
/// construction (`this`, `(0x1D)`).
const LIGHTING_SET_TYPE: u32 = 0x005a_8060;
/// The shader texture set of a texture set form (`this`).
const TEXTURE_SET_SHADER_SET: u32 = 0x0059_2cf0;
/// Loads a texture set into the lighting property: `this`, then `(0, set)`.
const LIGHTING_LOAD_TEXTURE_SET: u32 = 0x00b6_6ad0;
/// Stores the parallax scale in the lighting property: `this`, `(float)`.
const LIGHTING_SET_PARALLAX_SCALE: u32 = 0x009b_d580;
/// Two getters chained on the target node before the flag test of
/// `fn_0068c870`.
const TARGET_LINK_A: u32 = 0x0050_d100;
const TARGET_LINK_B: u32 = 0x0045_8b50;
/// Sets the texture of the no-lighting property: `this`, `(texture)`.
const NO_LIGHTING_SET_TEXTURE: u32 = 0x0043_8230;
/// Vertex colour array of a geometry (`this`), and its vertex count (`u16`).
const GEOMETRY_COLORS: u32 = 0x004a_8050;
const GEOMETRY_VERTEX_COUNT: u32 = 0x0045_6650;
/// Vertex and normal arrays of a geometry, and its triangle data.
const GEOMETRY_VERTICES: u32 = 0x0049_ec60;
const GEOMETRY_NORMALS: u32 = 0x004a_8030;
const GEOMETRY_STRIP_DATA: u32 = 0x0054_95f0;
/// Number of strip triangles of a geometry (`u16`).
const GEOMETRY_TRIANGLE_COUNT: u32 = 0x004a_1ff0;
/// The index list of the strip data (`this`).
const STRIP_INDICES: u32 = 0x00a7_5f10;
/// `this + 8`, the holder of a geometry's name, and its text (`this`).
const GEOMETRY_NAME_HOLDER: u32 = 0x0041_3f40;
const NAME_TEXT: u32 = 0x0043_b1b0;
/// Word the name setter gets for a non-permanent decal's shape.
const TRANSIENT_NAME_WORD_GETTER: u32 = 0x0068_8460;
/// Child count of a node and child by index (`this`, `(index)`).
const NODE_CHILD_COUNT: u32 = 0x0043_b480;
const NODE_CHILD: u32 = 0x0043_b4a0;
/// Constructor of the 0xB4-byte decal node: `this`, then `(parent, 0)`.
const DECAL_NODE_CONSTRUCT: u32 = 0x004e_e120;
/// Adds a `NiPointer` to the list kept in the decal node's member at
/// `+0xAC`: `this = member`, then `(NiPointer *)`.
const DECAL_NODE_MEMBER_ADD: u32 = 0x0063_1540;
/// Increments the change counter at `this + 0x44`; `(1)`.
const MARK_CHANGED: u32 = 0x004b_c480;
/// Constructor of the 0xC4-byte `NiTriShape`: `this`, then `(data)`.
const TRI_SHAPE_CONSTRUCT: u32 = 0x00a7_4480;
/// Constructor of the 0x58-byte `NiTriShapeData`: `this`, then nine words.
const TRI_DATA_CONSTRUCT: u32 = 0x00a7_b630;
/// Whether two points are within an epsilon, per component (`cdecl`: `a`,
/// `b`, epsilon).
const POINTS_NEARLY_EQUAL: u32 = 0x0049_e2f0;
/// Asks an occlusion query for its result: `this = query`, then
/// `(result address, 0)`; and destroys it: `this`, `(1)`.
const OCCLUSION_QUERY_RESULT: u32 = 0x00c4_edf0;
const OCCLUSION_QUERY_DELETE: u32 = 0x004a_4410;
/// The stream used to load the debug box: constructor, `Load(path, 0)`,
/// root count, root by index and destructor.
const STREAM_CONSTRUCT: u32 = 0x0043_cfd0;
const STREAM_LOAD: u32 = 0x00c3_a8a0;
const STREAM_ROOT_COUNT: u32 = 0x0043_b1e0;
const STREAM_ROOT: u32 = 0x0043_b200;
const STREAM_DESTRUCT: u32 = 0x0043_d090;
/// `NiFixedString` constructor from text (`this`, `(text)`, returns a
/// pointer to the string handle) and its release.
const FIXED_STRING_CONSTRUCT: u32 = 0x0043_8170;
const FIXED_STRING_RELEASE: u32 = 0x0043_81b0;
/// `CreateDeepCopy`: `this = object`, then `(NiPointer * result)`.
const DEEP_COPY: u32 = 0x00a5_d510;
/// Constructor of the 0xB0-byte holder node for the debug box: `this`,
/// `(float)`.
const DEBUG_HOLDER_CONSTRUCT: u32 = 0x0045_8ef0;
/// `TES::AddTempDebugObject`-shaped call: `this = TES`, `(object, float)`.
const TES_ADD_DEBUG_OBJECT: u32 = 0x0045_8e20;
/// Element constructor of the colour array.
const COLOR_ELEMENT_CONSTRUCTOR: u32 = 0x004a_7800;

// ---- creation info (the 4th word of the constructor) ----------------------

/// Offsets in the structure the constructor copies from (class name not
/// confirmed; the offsets are the ones the constructor reads).
mod info {
    pub const ORIGIN: u32 = 0x00;
    pub const VECTOR: u32 = 0x0c;
    pub const TARGET_NODE: u32 = 0x28;
    pub const TEXTURE_SET: u32 = 0x30;
    pub const WIDTH: u32 = 0x38;
    pub const HEIGHT: u32 = 0x3c;
    pub const DEPTH: u32 = 0x40;
    pub const ROTATION: u32 = 0x44;
    pub const CELL: u32 = 0x48;
    pub const PARALLAX_SCALE: u32 = 0x4c;
    pub const SPECULAR: u32 = 0x54;
    pub const ANGLE_DEGREES: u32 = 0x58;
    pub const VERTEX_COLOR: u32 = 0x60;
    pub const SUB_TEXTURE: u32 = 0x70;
    pub const PERMANENT: u32 = 0x71;
    pub const PARALLAX_OCCLUSION: u32 = 0x73;
    pub const ALPHA_TESTING: u32 = 0x74;
    pub const ALPHA_BLENDING: u32 = 0x75;
    pub const PARALLAX_MAX_PASSES: u32 = 0x76;
    pub const MODEL_SPACE: u32 = 0x77;
    pub const FORCE_FADE: u32 = 0x78;
    pub const TWO_SIDED: u32 = 0x79;
}

layout! {
    /// `BSTempEffectSimpleDecal` (Xbox PDB), 0x144 bytes on PC and the Xbox;
    /// the offsets are the Xbox PDB's, checked against this unit's code.
    /// The `BSTempEffect` base (+0x00 to +0x17) belongs to another unit.
    pub struct BSTempEffectSimpleDecal: 0x144 {
        /// `bFinished` (Xbox PDB).
        0x18 bFinished: bool,
        /// `bFinalize` (Xbox PDB): the geometry is ready to be finalized.
        0x19 bFinalize: bool,
        /// `bOcclusionQuery` (Xbox PDB).
        0x1A bOcclusionQuery: bool,
        /// `pOcclusionQuery` (Xbox PDB).
        0x1C pOcclusionQuery: Ptr,
        /// `spQueryTexture` (Xbox PDB): a `NiPointer` slot.
        0x20 spQueryTexture: Ptr,
        /// `iCurrentOcclusionResult` (Xbox PDB).
        0x24 iCurrentOcclusionResult: u32,
        /// `sp3D` (Xbox PDB): a `NiPointer` slot.
        0x28 sp3D: Ptr,
        /// `bPermanent` (Xbox PDB).
        0x2C bPermanent: bool,
        /// `bValidDecal` (Xbox PDB).
        0x2D bValidDecal: bool,
        /// `bForceFade` (Xbox PDB).
        0x2E bForceFade: bool,
        /// `pTextureSet` (Xbox PDB).
        0x30 pTextureSet: Ptr,
        /// `pOrigin` (Xbox PDB): a `NiPoint3`.
        0x34 pOrigin: Inline<NiPoint3>,
        /// `pVector` (Xbox PDB): a `NiPoint3`.
        0x40 pVector: Inline<NiPoint3>,
        /// `pDecalCenter` (Xbox PDB): a `NiPoint3`.
        0x4C pDecalCenter: Inline<NiPoint3>,
        /// `pDecalNormal` (Xbox PDB): a `NiPoint3`.
        0x58 pDecalNormal: Inline<NiPoint3>,
        /// `pClippingPlanes` (Xbox PDB): six `NiPlane`s (left, right,
        /// bottom, top, front, back).
        0x64 pClippingPlanes: Inline<NiPlane>,
        /// `iDecalVertexCount` (Xbox PDB).
        0xC4 iDecalVertexCount: i32,
        /// `iDecalTriangleCount` (Xbox PDB).
        0xC8 iDecalTriangleCount: i32,
        /// `pTempVertexArray` (Xbox PDB).
        0xCC pTempVertexArray: Ptr,
        /// `pFinalVertexArray` (Xbox PDB).
        0xD0 pFinalVertexArray: Ptr,
        /// `pTempNormalArray` (Xbox PDB).
        0xD4 pTempNormalArray: Ptr,
        /// `pFinalNormalArray` (Xbox PDB).
        0xD8 pFinalNormalArray: Ptr,
        /// `pTempTexcoordArray` (Xbox PDB).
        0xDC pTempTexcoordArray: Ptr,
        /// `pFinalTexcoordArray` (Xbox PDB).
        0xE0 pFinalTexcoordArray: Ptr,
        /// `pTempColorArray` (Xbox PDB).
        0xE4 pTempColorArray: Ptr,
        /// `pFinalColorArray` (Xbox PDB).
        0xE8 pFinalColorArray: Ptr,
        /// `pTempIndexArray` (Xbox PDB).
        0xEC pTempIndexArray: Ptr,
        /// `pFinalIndexArray` (Xbox PDB).
        0xF0 pFinalIndexArray: Ptr,
        /// `pOldToNewIndexArray` (Xbox PDB): `i16` entries.
        0xF4 pOldToNewIndexArray: Ptr,
        /// `pNewIndexMap` (Xbox PDB): a `BSSimpleArray<DecalVertex *, 1024>`.
        0xF8 pNewIndexMap: Inline<BSSimpleArray>,
        /// `pParentNode` (Xbox PDB).
        0x108 pParentNode: Ptr,
        /// `spTargetNode` (Xbox PDB): a `NiPointer` slot.
        0x10C spTargetNode: Ptr,
        /// `fRotation` (Xbox PDB).
        0x114 fRotation: f32,
        /// `fWidth` (Xbox PDB).
        0x118 fWidth: f32,
        /// `fHeight` (Xbox PDB).
        0x11C fHeight: f32,
        /// `fDepth` (Xbox PDB).
        0x120 fDepth: f32,
        /// `cSubTexture` (Xbox PDB).
        0x124 cSubTexture: u8,
        /// `bUseParallaxOcclusion` (Xbox PDB).
        0x125 bUseParallaxOcclusion: bool,
        /// `fParallaxOcclusionScale` (Xbox PDB).
        0x128 fParallaxOcclusionScale: f32,
        /// `ucParallaxOcclusionMaxPasses` (Xbox PDB).
        0x12C ucParallaxOcclusionMaxPasses: u8,
        /// `bUseAlphaTesting` (Xbox PDB).
        0x12D bUseAlphaTesting: bool,
        /// `bUseAlphaBlending` (Xbox PDB).
        0x12E bUseAlphaBlending: bool,
        /// `fSpecular` (Xbox PDB).
        0x130 fSpecular: f32,
        /// `fVertexColor` (Xbox PDB): a `NiColor`, red first.
        0x134 fVertexColor: f32,
        /// Green of `fVertexColor`.
        0x138 fVertexColorGreen: f32,
        /// Blue of `fVertexColor`.
        0x13C fVertexColorBlue: f32,
        /// `bModelSpace` (Xbox PDB).
        0x140 bModelSpace: bool,
        /// `bTwoSided` (Xbox PDB).
        0x141 bTwoSided: bool,
    }
}

type Decal = BSTempEffectSimpleDecal;

// ---- helpers -----------------------------------------------------------

/// The `NiPointer` slot at `slot`: the pointer it holds.
fn ni_pointer_get(e: &mut Engine, slot: u32) -> Ptr {
    e.call(NI_POINTER_GET, &args![slot]).ptr()
}

fn ni_pointer_assign(e: &mut Engine, slot: u32, value: Ptr) {
    e.call(NI_POINTER_ASSIGN, &args![slot, value]);
}

fn construct_empty(e: &mut Engine, object: u32) {
    e.call(EMPTY_CONSTRUCTOR, &args![object]);
}

/// Copies `words` 32-bit words (the `MOVSD` and register copies of the code).
fn copy_words(e: &mut Engine, dest: u32, source: u32, words: u32) {
    for i in 0..words {
        let word = e.mem.u32(source + 4 * i);
        e.mem.set_u32(dest + 4 * i, word);
    }
}

/// `operator new(size)` and, when it returned memory, the constructor on it
/// with `extra` after `this`; 0 when the allocation failed.
fn new_object(e: &mut Engine, size: u32, constructor: u32, extra: &[u32]) -> u32 {
    let memory = e.call(OPERATOR_NEW, &args![size]).u32();
    if memory == 0 {
        return 0;
    }
    let mut words = vec![memory];
    words.extend_from_slice(extra);
    e.call(constructor, &words).u32()
}

/// A block standing in for the stack frame's locals; freed by [`Frame::release`].
struct Frame {
    base: u32,
    size: u32,
    used: u32,
}

impl Frame {
    fn new(e: &mut Engine, size: u32) -> Frame {
        Frame {
            base: e.mem.alloc(size),
            size,
            used: 0,
        }
    }

    /// The next `bytes` bytes of the frame (word aligned).
    fn take(&mut self, bytes: u32) -> u32 {
        let address = self.base + self.used;
        self.used += bytes.div_ceil(4) * 4;
        assert!(self.used <= self.size, "frame too small");
        address
    }

    fn release(self, e: &mut Engine) {
        e.mem.free(self.base);
    }
}

// Translated from 0068acb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTempEffectSimpleDecal::GetRTTI` (Xbox PDB): the class's `NiRTTI`.
pub fn bstempeffectsimpledecal_get_rtti(_e: &mut Engine, _this: Ptr<Decal>) -> Ptr {
    Ptr::new(RTTI)
}

// Translated from 0068acc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTempEffectSimpleDecal::Get3D` (Xbox PDB): the pointer held by `sp3D`.
pub fn bstempeffectsimpledecal_get_3d(e: &mut Engine, this: Ptr<Decal>) -> Ptr {
    ni_pointer_get(e, this.addr() + Decal::sp3D.off)
}

// Translated from 0068ace0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTempEffectSimpleDecal::_scalar_deleting_destructor_` (Xbox PDB): runs
/// the destructor and, when bit 0 of `flags` is set, frees the 0x144-byte
/// object.
pub fn bstempeffectsimpledecal_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<Decal>,
    flags: u32,
) -> Ptr<Decal> {
    bstempeffectsimpledecal_destructor(e, this);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, OBJECT_SIZE]);
    }
    this
}

// Translated from 0068ad20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor (the body stores this class's vtable `01069ddc` and
/// initializes every field; the engine map gives no name): `parent_node`
/// is stored in `pParentNode`, `lifetime` and the creation info's cell go to
/// the `BSTempEffect` constructor, and the rest of `info` (see [`info`]) is
/// copied into the fields. It also refreshes the static [`CLIP_ANGLE_LIMIT`]
/// from the info's angle (`max(0.01, f(angle in radians))`, `f` = `004e4470`)
/// and reads [`MAX_DECAL_COUNT`] once.
// Translation incomplete: 0068ad20 - the compiler's exception-unwinding frame (FS:[0], its state changes and handler) is not translated.
pub fn fn_0068ad20(
    e: &mut Engine,
    this: Ptr<Decal>,
    parent_node: Ptr,
    lifetime: f32,
    info_pointer: Ptr,
) -> Ptr<Decal> {
    let t = this.addr();
    let info_address = info_pointer.addr();
    let cell = e.mem.u32(info_address + info::CELL);
    e.call(BASE_CONSTRUCT, &args![this, cell, lifetime]);
    e.mem.set_u32(t, VTABLE);
    e.call(
        NI_POINTER_CONSTRUCT,
        &args![t + Decal::spQueryTexture.off, 0u32],
    );
    e.call(NI_POINTER_CONSTRUCT, &args![t + Decal::sp3D.off, 0u32]);
    construct_empty(e, t + Decal::pOrigin.off);
    construct_empty(e, t + Decal::pVector.off);
    construct_empty(e, t + Decal::pDecalCenter.off);
    construct_empty(e, t + Decal::pDecalNormal.off);
    e.call(
        VECTOR_CONSTRUCTOR,
        &args![
            t + Decal::pClippingPlanes.off,
            0x10u32,
            6i32,
            PLANE_ELEMENT_CONSTRUCTOR
        ],
    );
    fn_0068eb00(e, Ptr::new(t + Decal::pNewIndexMap.off));
    e.call(
        NI_POINTER_CONSTRUCT,
        &args![t + Decal::spTargetNode.off, 0u32],
    );
    e.call(
        POINT3_CONSTRUCT,
        &args![t + Decal::fVertexColor.off, 0.0f32, 0.0f32, 0.0f32],
    );
    e.set(this, Decal::bFinished, false);
    e.set(this, Decal::bValidDecal, false);
    e.set(this, Decal::bFinalize, false);
    e.set(this, Decal::bOcclusionQuery, false);
    let texture_set = e.mem.u32(info_address + info::TEXTURE_SET);
    e.set(this, Decal::pTextureSet, Ptr::new(texture_set));
    if e.global::<i32>(MAX_DECAL_COUNT) == -1 {
        let setting = e
            .call(SETTING_VALUE_GETTER, &args![MAX_DECAL_SETTING])
            .u32();
        let value = e.mem.u32(setting);
        e.set_global(MAX_DECAL_COUNT, value);
    }
    e.set(this, Decal::pParentNode, parent_node);
    let target = Ptr::new(e.mem.u32(info_address + info::TARGET_NODE));
    ni_pointer_assign(e, t + Decal::spTargetNode.off, target);
    ni_pointer_assign(e, t + Decal::sp3D.off, Ptr::NULL);
    let permanent = e.mem.u8(info_address + info::PERMANENT);
    e.mem.set_u8(t + Decal::bPermanent.off, permanent);
    let force_fade = e.mem.u8(info_address + info::FORCE_FADE);
    e.mem.set_u8(t + Decal::bForceFade.off, force_fade);
    e.set(this, Decal::iDecalVertexCount, 0);
    e.set(this, Decal::iDecalTriangleCount, 0);
    copy_words(e, t + Decal::pOrigin.off, info_address + info::ORIGIN, 3);
    copy_words(e, t + Decal::pVector.off, info_address + info::VECTOR, 3);
    let width = e.mem.f32(info_address + info::WIDTH);
    e.set(this, Decal::fWidth, width);
    let height = e.mem.f32(info_address + info::HEIGHT);
    e.set(this, Decal::fHeight, height);
    let depth = e.mem.f32(info_address + info::DEPTH);
    e.set(this, Decal::fDepth, depth);
    let rotation = e.mem.f32(info_address + info::ROTATION);
    e.set(this, Decal::fRotation, rotation);
    let sub_texture = e.mem.u8(info_address + info::SUB_TEXTURE);
    e.set(this, Decal::cSubTexture, sub_texture);
    e.set(this, Decal::pOcclusionQuery, Ptr::NULL);
    ni_pointer_assign(e, t + Decal::spQueryTexture.off, Ptr::NULL);
    let angle = e.mem.f32(info_address + info::ANGLE_DEGREES);
    let degrees_to_radians: f64 = e.global(DEGREES_TO_RADIANS);
    let radians = (angle as f64 * degrees_to_radians) as f32;
    let limit = e.call(0x004e_4470, &args![radians]).f64() as f32;
    e.set_global(CLIP_ANGLE_LIMIT, limit);
    let floor: f64 = e.global(CLIP_ANGLE_FLOOR);
    let limit = fn_0068b050(e, floor, limit) as f32;
    e.set_global(CLIP_ANGLE_LIMIT, limit);
    let parallax = e.mem.u8(info_address + info::PARALLAX_OCCLUSION);
    e.mem.set_u8(t + Decal::bUseParallaxOcclusion.off, parallax);
    let parallax_scale = e.mem.f32(info_address + info::PARALLAX_SCALE);
    e.set(this, Decal::fParallaxOcclusionScale, parallax_scale);
    let passes = e.mem.u8(info_address + info::PARALLAX_MAX_PASSES);
    e.set(this, Decal::ucParallaxOcclusionMaxPasses, passes);
    let testing = e.mem.u8(info_address + info::ALPHA_TESTING);
    e.mem.set_u8(t + Decal::bUseAlphaTesting.off, testing);
    let blending = e.mem.u8(info_address + info::ALPHA_BLENDING);
    e.mem.set_u8(t + Decal::bUseAlphaBlending.off, blending);
    let specular = e.mem.f32(info_address + info::SPECULAR);
    e.set(this, Decal::fSpecular, specular);
    copy_words(
        e,
        t + Decal::fVertexColor.off,
        info_address + info::VERTEX_COLOR,
        3,
    );
    let model_space = e.mem.u8(info_address + info::MODEL_SPACE);
    e.mem.set_u8(t + Decal::bModelSpace.off, model_space);
    let two_sided = e.mem.u8(info_address + info::TWO_SIDED);
    e.mem.set_u8(t + Decal::bTwoSided.off, two_sided);
    this
}

// Translated from 0068b050 (decompiled, FalloutNV.exe 1.4.0.525)
/// The larger of `a` and `b` (as a `double`): `a` when `b < a`, otherwise
/// `b` (so also `b` when either is a NaN). The constructor uses it to keep
/// the clip angle limit at 0.01 or more.
pub fn fn_0068b050(_e: &mut Engine, a: f64, b: f32) -> f64 {
    if (b as f64) < a {
        a
    } else {
        b as f64
    }
}

// Translated from 0068b080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTempEffectSimpleDecal::~BSTempEffectSimpleDecal` (Xbox PDB): releases
/// the target node and parent, for a non-permanent decal detaches the shape
/// from the property lights and from its parent, frees the arrays
/// ([`bstempeffectsimpledecal_clean_up_memory`]), decrements the valid-decal
/// counter and destroys the members and the base.
// Translation incomplete: 0068b080 - the compiler's exception-unwinding frame (FS:[0], its state changes and handler) is not translated.
pub fn bstempeffectsimpledecal_destructor(e: &mut Engine, this: Ptr<Decal>) {
    let t = this.addr();
    e.mem.set_u32(t, VTABLE);
    ni_pointer_assign(e, t + Decal::spTargetNode.off, Ptr::NULL);
    e.set(this, Decal::pParentNode, Ptr::NULL);
    if !e.get(this, Decal::bPermanent) && !ni_pointer_get(e, t + Decal::sp3D.off).is_null() {
        let geometry = ni_pointer_get(e, t + Decal::sp3D.off);
        e.call(REMOVE_FROM_PROPERTY_LIGHTS, &args![geometry]);
        let shape = ni_pointer_get(e, t + Decal::sp3D.off);
        if !e.call(NODE_PARENT, &args![shape]).ptr::<()>().is_null() {
            let shape = ni_pointer_get(e, t + Decal::sp3D.off);
            let parent = e.call(NODE_PARENT, &args![shape]).u32();
            let shape = ni_pointer_get(e, t + Decal::sp3D.off);
            // The parent's virtual slot 0xE8 (remove a child) with the shape.
            e.vcall(parent, 0xe8, &args![shape]);
        }
        ni_pointer_assign(e, t + Decal::sp3D.off, Ptr::NULL);
    }
    bstempeffectsimpledecal_clean_up_memory(e, this);
    if e.get(this, Decal::bValidDecal) {
        let count = e.global::<u32>(VALID_DECAL_COUNT).wrapping_sub(1);
        e.set_global(VALID_DECAL_COUNT, count);
    }
    e.call(NI_POINTER_RELEASE, &args![t + Decal::spTargetNode.off]);
    fn_0068eb30(e, Ptr::new(t + Decal::pNewIndexMap.off));
    e.call(NI_POINTER_RELEASE, &args![t + Decal::sp3D.off]);
    e.call(NI_POINTER_RELEASE, &args![t + Decal::spQueryTexture.off]);
    e.call(BASE_DESTRUCT, &args![this]);
}

// Translated from 0068b1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTempEffectSimpleDecal::Initialize` (Xbox PDB): processes the target
/// geometry and, when that succeeded, runs the base `Initialize`.
pub fn bstempeffectsimpledecal_initialize(e: &mut Engine, this: Ptr<Decal>) {
    let target = ni_pointer_get(e, this.addr() + Decal::spTargetNode.off);
    if bstempeffectsimpledecal_process_geometry(e, this, target) {
        e.call(BASE_INITIALIZE, &args![this]);
    }
}

// Translated from 0068b220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTempEffectSimpleDecal::CleanUpMemory` (Xbox PDB): unless the decal is
/// valid (its arrays now belong to the shape), gives the five temporary
/// arrays back to the thread's scrap heap and frees the old-to-new index
/// array; then deletes every `DecalVertex` of `pNewIndexMap`, nulls the
/// entries and empties the array.
pub fn bstempeffectsimpledecal_clean_up_memory(e: &mut Engine, this: Ptr<Decal>) {
    let t = this.addr();
    if !e.get(this, Decal::bValidDecal) {
        let manager = e.call(MEMORY_MANAGER_GETTER, &args![]).u32();
        let heap = e.call(THREAD_SCRAP_HEAP, &args![manager]).u32();
        for field in [
            Decal::pTempVertexArray,
            Decal::pTempNormalArray,
            Decal::pTempTexcoordArray,
            Decal::pTempColorArray,
            Decal::pTempIndexArray,
        ] {
            let array = e.get(this, field);
            if !array.is_null() {
                e.call(SCRAP_DEALLOCATE, &args![heap, array]);
            }
        }
        let map = e.get(this, Decal::pOldToNewIndexArray);
        if !map.is_null() {
            e.call(MEMORY_FREE, &args![map]);
        }
        e.set(this, Decal::pTempVertexArray, Ptr::NULL);
        e.set(this, Decal::pTempNormalArray, Ptr::NULL);
        e.set(this, Decal::pTempTexcoordArray, Ptr::NULL);
        e.set(this, Decal::pTempColorArray, Ptr::NULL);
        e.set(this, Decal::pTempIndexArray, Ptr::NULL);
        e.set(this, Decal::pOldToNewIndexArray, Ptr::NULL);
    }
    let map = t + Decal::pNewIndexMap.off;
    let mut i = 0u32;
    while i < e.call(ARRAY_SIZE, &args![map]).u32() {
        let slot = e.call(ARRAY_ELEMENT, &args![map, i]).u32();
        let vertex = e.mem.u32(slot);
        if vertex != 0 {
            // The vertex's scalar deleting destructor, flag 1 (delete).
            e.call(DECAL_VERTEX_DELETE, &args![vertex, 1u32]);
        }
        let slot = e.call(ARRAY_ELEMENT, &args![map, i]).u32();
        e.mem.set_u32(slot, 0);
        i += 1;
    }
    e.call(ARRAY_CLEAR, &args![map, 1u32]);
}

/// Number of elements of a `BSSimpleArray` (`this + 8`).
const ARRAY_SIZE: u32 = 0x0044_ddc0;
/// Address of element `index` of a `BSSimpleArray` (`this`, then `(index)`).
const ARRAY_ELEMENT: u32 = 0x0087_7a30;
/// Scalar deleting destructor of a `DecalVertex`: `this`, then `(flags)`.
const DECAL_VERTEX_DELETE: u32 = 0x007b_3fa0;
/// Empties a `BSSimpleArray` (`this`, then `(free the buffer)`).
const ARRAY_CLEAR: u32 = 0x0084_54f0;
/// Appends a pointer to a `BSSimpleArray`: `this`, then `(address of the
/// pointer to append)`.
const ARRAY_APPEND: u32 = 0x007c_b2e0;
/// Constructor of a `DecalVertex` (0x10 bytes): only calls the empty
/// constructor of its point.
const DECAL_VERTEX_CONSTRUCT: u32 = 0x0062_40d0;
/// Constructor of the `BSSimpleArray` (`this`, then `(0, 0)`).
const ARRAY_CONSTRUCT: u32 = 0x006b_3eb0;

// Translated from 0068b3e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Static function (the engine map gives no name): when the static
/// `spPreloadedDecalTextureSets` is still null, builds a `QueuedFile`
/// (0x28 bytes, constructor `00c3c590` with argument 5) inside the scope
/// guard `00404eb0`/`00404ee0` (arguments `7, 1`, this unit's source file
/// name and line 0x11F) and stores it there.
// Translation incomplete: 0068b3e0 - the compiler's exception-unwinding frame (FS:[0], its state changes and handler) is not translated.
pub fn fn_0068b3e0(e: &mut Engine) {
    if !ni_pointer_get(e, PRELOADED_TEXTURE_SETS).is_null() {
        return;
    }
    e.with_stack(4, |e, guard| {
        e.call(
            SCOPE_GUARD_CONSTRUCT,
            &args![guard, 7u32, 1u32, SOURCE_FILE_NAME, 0x11fu32],
        );
        let memory = e.call(RUNTIME_NEW, &args![0x28u32]).u32();
        let file = if memory == 0 {
            0
        } else {
            e.call(QUEUED_FILE_CONSTRUCT, &args![memory, 5u32]).u32()
        };
        e.call(
            QUEUED_FILE_POINTER_ASSIGN,
            &args![PRELOADED_TEXTURE_SETS, file],
        );
        e.call(SCOPE_GUARD_DESTRUCT, &args![guard]);
    });
}

// Translated from 0068b4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Static function: clears the static `spPreloadedDecalTextureSets`.
pub fn fn_0068b4a0(e: &mut Engine) {
    e.call(
        QUEUED_FILE_POINTER_ASSIGN,
        &args![PRELOADED_TEXTURE_SETS, 0u32],
    );
}

// Translated from 0068b4c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTempEffectSimpleDecal::ProcessGeometry` (Xbox PDB): prepares the decal
/// on `geometry`. Without a geometry of the required class it marks the decal
/// finished and returns false. Otherwise it brings the origin and direction
/// into the geometry's space (inverse rotation, and scale unless the decal
/// is model-space), scales width, height and depth by 1/world scale, builds
/// the six clipping planes from the rotation about the decal normal, clips
/// the mesh ([`bstempeffectsimpledecal_clip_mesh`]); with no triangle left
/// it marks the decal finished (printing a diagnostic when the setting is
/// on) and returns false. Otherwise it computes the texture coordinates of
/// every clipped vertex (sub-texture quarter in `cSubTexture`, mirrored in
/// u when the byte at +0x2C is set), shrinks the arrays
/// ([`bstempeffectsimpledecal_resize_data_arrays`]), creates the shape from
/// them and returns true with `bFinalize` set.
pub fn bstempeffectsimpledecal_process_geometry(
    e: &mut Engine,
    this: Ptr<Decal>,
    geometry: Ptr,
) -> bool {
    let t = this.addr();
    let mut frame = Frame::new(e, 0x300);
    let axis_u = frame.take(12);
    let axis_v = frame.take(12);
    construct_empty(e, axis_u);
    construct_empty(e, axis_v);
    let usable = !geometry.is_null() && e.call(IS_KIND_OF, &args![GEOMETRY_RTTI, geometry]).bool();
    if !usable {
        e.set(this, Decal::bFinished, true);
        frame.release(e);
        return false;
    }
    let origin = frame.take(12);
    copy_words(e, origin, t + Decal::pOrigin.off, 3);
    let vector = frame.take(12);
    copy_words(e, vector, t + Decal::pVector.off, 3);
    let center = frame.take(12);
    let normal = frame.take(12);
    let matrix = frame.take(36);
    construct_empty(e, center);
    construct_empty(e, normal);
    construct_empty(e, matrix);
    let model_space = e.get(this, Decal::bModelSpace);
    let rotation = if model_space {
        e.call(NODE_LOCAL_ROTATE, &args![geometry]).u32()
    } else {
        e.call(NODE_WORLD_ROTATE, &args![geometry]).u32()
    };
    copy_words(e, matrix, rotation, 9);
    let inverse = frame.take(36);
    e.call(MATRIX_INVERSE, &args![matrix, inverse]);
    let rotated = frame.take(12);
    let result = e
        .call(MATRIX_TRANSFORM_POINT, &args![inverse, rotated, vector])
        .u32();
    copy_words(e, normal, result, 3);
    let local_origin = frame.take(12);
    construct_empty(e, local_origin);
    if model_space {
        copy_words(e, local_origin, origin, 3);
    } else {
        let translate = e.call(NODE_WORLD_TRANSLATE, &args![geometry]).u32();
        let moved = frame.take(12);
        let result = e
            .call(POINT3_SUBTRACT, &args![origin, moved, translate])
            .u32();
        copy_words(e, local_origin, result, 3);
        let scale = e.call(NODE_WORLD_SCALE, &args![geometry]).f64();
        let inverse_scale = (1.0 / scale) as f32;
        let scaled = frame.take(12);
        let result = e
            .call(POINT3_SCALE, &args![local_origin, scaled, inverse_scale])
            .u32();
        copy_words(e, local_origin, result, 3);
    }
    e.call(
        MATRIX_TRANSFORM_VERTICES,
        &args![inverse, ZERO_POINT, 1i32, local_origin, center],
    );
    for field in [Decal::fWidth, Decal::fHeight, Decal::fDepth] {
        let scale = e.call(NODE_WORLD_SCALE, &args![geometry]).f64();
        let value = e.get(this, field);
        e.set(this, field, ((1.0 / scale) * value as f64) as f32);
    }
    copy_words(e, t + Decal::pDecalCenter.off, center, 3);
    copy_words(e, t + Decal::pDecalNormal.off, normal, 3);

    // The rotation that takes Z to the normal, turned by the decal's rotation.
    let rotation_matrix = frame.take(36);
    let (nx, ny, nz) = (
        e.mem.u32(normal),
        e.mem.u32(normal + 4),
        e.mem.u32(normal + 8),
    );
    e.call(ROTATION_Z_TO_VECTOR, &args![rotation_matrix, nx, ny, nz]);
    let turn = frame.take(36);
    construct_empty(e, turn);
    let angle = e.get(this, Decal::fRotation);
    e.call(MATRIX_MAKE_Z_ROTATION, &args![turn, angle]);
    let product = frame.take(36);
    let result = e
        .call(MATRIX_MULTIPLY, &args![turn, product, rotation_matrix])
        .u32();
    copy_words(e, rotation_matrix, result, 9);
    e.call(MATRIX_GET_ROW, &args![rotation_matrix, 0u32, axis_u]);
    e.call(MATRIX_GET_ROW, &args![rotation_matrix, 1u32, axis_v]);

    let half: f64 = e.global(HALF);
    let center_address = t + Decal::pDecalCenter.off;
    let plane_scratch = frame.take(0x20);
    let width = e.get(this, Decal::fWidth);
    let height = e.get(this, Decal::fHeight);
    let depth = e.get(this, Decal::fDepth);
    let (ux, uy, uz) = (
        e.mem.f32(axis_u),
        e.mem.f32(axis_u + 4),
        e.mem.f32(axis_u + 8),
    );
    let (vx, vy, vz) = (
        e.mem.f32(axis_v),
        e.mem.f32(axis_v + 4),
        e.mem.f32(axis_v + 8),
    );
    let planes = t + Decal::pClippingPlanes.off;

    let along_u = e.call(POINT3_DOT, &args![center_address, axis_u]).f64() as f32;
    let left = (-(width as f64 * half - along_u as f64)) as f32;
    store_plane(e, plane_scratch, planes, [ux, uy, uz], left);
    let right = (-(width as f64 * half + along_u as f64)) as f32;
    store_plane(e, plane_scratch, planes + 0x10, [-ux, -uy, -uz], right);

    let along_v = e.call(POINT3_DOT, &args![center_address, axis_v]).f64() as f32;
    let bottom = (-(height as f64 * half - along_v as f64)) as f32;
    store_plane(e, plane_scratch, planes + 0x20, [vx, vy, vz], bottom);
    let top = (-(height as f64 * half + along_v as f64)) as f32;
    store_plane(e, plane_scratch, planes + 0x30, [-vx, -vy, -vz], top);

    let normal_address = t + Decal::pDecalNormal.off;
    let along_normal = e
        .call(POINT3_DOT, &args![center_address, normal_address])
        .f64() as f32;
    let (dx, dy, dz) = (
        e.mem.f32(normal_address),
        e.mem.f32(normal_address + 4),
        e.mem.f32(normal_address + 8),
    );
    let front = (-(depth as f64 + along_normal as f64)) as f32;
    store_plane(e, plane_scratch, planes + 0x40, [-dx, -dy, -dz], front);
    let back = (-(depth as f64 - along_normal as f64)) as f32;
    store_plane(e, plane_scratch, planes + 0x50, [dx, dy, dz], back);

    bstempeffectsimpledecal_clip_mesh(e, this, geometry);

    if e.get(this, Decal::iDecalTriangleCount) == 0 {
        e.set(this, Decal::bFinished, true);
        let setting = e
            .call(SETTING_BYTE_GETTER, &args![DIAGNOSTICS_SETTING])
            .u32();
        if e.mem.u8(setting) != 0 {
            let target = ni_pointer_get(e, t + Decal::spTargetNode.off);
            let name_holder = e.call(0x0041_3f40, &args![target]).u32();
            let name = e.call(0x0043_b1b0, &args![name_holder]).u32();
            e.call(PRINT_LINE, &args![NO_DECAL_MESSAGE, name]);
        }
        frame.release(e);
        return false;
    }

    let inverse_width = (1.0 / width as f64) as f32;
    let inverse_height = (1.0 / height as f64) as f32;
    let offset = frame.take(12);
    let texcoord = frame.take(8);
    let mut i = 0i32;
    while i < e.get(this, Decal::iDecalVertexCount) {
        let vertices = e.get(this, Decal::pTempVertexArray).addr();
        e.call(
            POINT3_SUBTRACT,
            &args![vertices + 12 * i as u32, offset, center_address],
        );
        let dot_u = e.call(POINT3_DOT, &args![offset, axis_u]).f64();
        let mut u = (dot_u * inverse_width as f64 + half) as f32;
        let dot_v = e.call(POINT3_DOT, &args![offset, axis_v]).f64();
        let sub_texture = e.get(this, Decal::cSubTexture);
        let two: f64 = e.global(TWO);
        u = (u as f64 / two) as f32;
        let mut v = ((dot_v * inverse_height as f64 + half) as f32 as f64 / two) as f32;
        match sub_texture {
            0 => {}
            1 => u = (u as f64 + half) as f32,
            2 => v = (v as f64 + half) as f32,
            3 => {
                u = (u as f64 + half) as f32;
                v = (v as f64 + half) as f32;
            }
            _ => {
                u = u + u;
                v = v + v;
            }
        }
        let texcoords = e.get(this, Decal::pTempTexcoordArray).addr();
        let result = if e.get(this, Decal::bPermanent) {
            let mirrored = (1.0f64 - u as f64) as f32;
            e.call(POINT2_CONSTRUCT, &args![texcoord, mirrored, v])
                .u32()
        } else {
            e.call(POINT2_CONSTRUCT, &args![texcoord, u, v]).u32()
        };
        copy_words(e, texcoords + 8 * i as u32, result, 2);
        i += 1;
    }

    bstempeffectsimpledecal_resize_data_arrays(e, this);
    let data = bstempeffectsimpledecal_create_tri_data(e, this);
    let shape = bstempeffectsimpledecal_create_tri_shape(e, data);
    ni_pointer_assign(e, t + Decal::sp3D.off, shape);
    if !ni_pointer_get(e, t + Decal::sp3D.off).is_null() {
        e.set(this, Decal::pFinalVertexArray, Ptr::NULL);
        e.set(this, Decal::pFinalNormalArray, Ptr::NULL);
        e.set(this, Decal::pFinalTexcoordArray, Ptr::NULL);
        e.set(this, Decal::pFinalColorArray, Ptr::NULL);
        e.set(this, Decal::pFinalIndexArray, Ptr::NULL);
    }
    e.set(this, Decal::bFinalize, true);
    frame.release(e);
    true
}

/// A `NiPlane` built the way the code does it (a `NiPoint3` of the three
/// components, then the plane from that point and `constant`), copied to
/// `destination`. `scratch` is 0x20 bytes of locals.
fn store_plane(e: &mut Engine, scratch: u32, destination: u32, normal: [f32; 3], constant: f32) {
    let point = scratch;
    let plane = scratch + 0x10;
    e.call(
        POINT3_CONSTRUCT,
        &args![point, normal[0], normal[1], normal[2]],
    );
    let result = e
        .call(PLANE_CONSTRUCT, &args![plane, point, constant])
        .u32();
    copy_words(e, destination, result, 4);
}

// Translated from 0068be90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTempEffectSimpleDecal::FinalizeGeometry` (Xbox PDB): builds the
/// properties of the decal's shape and hangs it in the scene. Without a shape
/// it marks the decal finished and returns false. It attaches a material
/// property (specular from `fSpecular`), an alpha property (blending and
/// testing flags, test function 4, threshold 0x80) and, for a two-sided
/// decal, a third property (argument 3); then a lighting property built
/// from the texture set (flag 5, and 0xB with the parallax settings when the
/// shader accepts it) or, when the shader refuses it, a no-lighting
/// property with the texture set's first texture. Both get flag 0x1B and
/// flag 0x35 when the target's flag test passes. With the debug flag the
/// shape's vertex colours are set to yellow. The shape is named, the Havok
/// objects below it are removed and its shader object type is set. A
/// non-permanent decal is added to the process lists when `bForceFade` is
/// set; otherwise it joins the static decal list and the oldest decal is
/// handed to the process lists when the limit is exceeded. The decal then
/// finds or creates a decal node among the children of its target's parent,
/// attaches itself to it, copies the target's transform onto the shape,
/// updates properties and lighting and releases the temporary arrays
/// ([`bstempeffectsimpledecal_clean_up_memory`]). It sets `bValidDecal` and
/// `bFinished`, clears `bFinalize` and returns true. When the target has no
/// parent it marks the decal finished and returns false.
///
/// The stack byte at `-0xD` (which would force the no-lighting path when
/// set) is initialized to 0 and never written, so it is a constant false
/// here.
// Translation incomplete: 0068be90 - the compiler's exception-unwinding frame (FS:[0], its state changes and handler) is not translated.
pub fn bstempeffectsimpledecal_finalize_geometry(e: &mut Engine, this: Ptr<Decal>) -> bool {
    let t = this.addr();
    let forced_no_lighting = false;
    if ni_pointer_get(e, t + Decal::sp3D.off).is_null() {
        e.set(this, Decal::bFinished, true);
        return false;
    }

    // Material property.
    let material = new_object(e, 0x4c, MATERIAL_PROPERTY_CONSTRUCT, &[]);
    let specular = e.get(this, Decal::fSpecular);
    e.call(MATERIAL_SET_SPECULAR, &args![material, specular]);
    let shape = ni_pointer_get(e, t + Decal::sp3D.off);
    e.call(ATTACH_PROPERTY, &args![shape, material]);

    // Alpha property.
    let alpha = new_object(e, 0x1c, ALPHA_PROPERTY_CONSTRUCT, &[]);
    let blending = e.get(this, Decal::bUseAlphaBlending) as u32;
    e.call(ALPHA_SET_BLENDING, &args![alpha, blending]);
    let testing = e.get(this, Decal::bUseAlphaTesting) as u32;
    e.call(ALPHA_SET_TESTING, &args![alpha, testing]);
    e.call(ALPHA_SET_TEST_FUNCTION, &args![alpha, 4u32]);
    e.call(ALPHA_SET_THRESHOLD, &args![alpha, 0x80u32]);
    let shape = ni_pointer_get(e, t + Decal::sp3D.off);
    e.call(ATTACH_PROPERTY, &args![shape, alpha]);

    if e.get(this, Decal::bTwoSided) {
        let two_sided = new_object(e, 0x24, TWO_SIDED_PROPERTY_CONSTRUCT, &[]);
        e.call(TWO_SIDED_SET_MODE, &args![two_sided, 3u32]);
        let shape = ni_pointer_get(e, t + Decal::sp3D.off);
        e.call(ATTACH_PROPERTY, &args![shape, two_sided]);
    }

    // Shader property.
    let lighting = new_object(e, 0x108, LIGHTING_PROPERTY_CONSTRUCT, &[]);
    e.call(LIGHTING_SET_TYPE, &args![lighting, 0x1du32]);
    e.call(SHADER_SET_FLAG, &args![lighting, 5u32, 1u32]);
    let texture_set = e.get(this, Decal::pTextureSet);
    let shader_texture_set = e.call(TEXTURE_SET_SHADER_SET, &args![texture_set]).u32();
    e.call(
        LIGHTING_LOAD_TEXTURE_SET,
        &args![lighting, 0u32, shader_texture_set],
    );
    let supported = e.vcall(lighting, 0xf4, &args![1u32, 0u32]).u32();
    if supported != 0 && !forced_no_lighting {
        if e.vcall(lighting, 0xf4, &args![3u32, 0u32]).u32() != 0 {
            e.call(SHADER_SET_FLAG, &args![lighting, 0xbu32, 1u32]);
            if e.get(this, Decal::bUseParallaxOcclusion) {
                e.call(SHADER_SET_FLAG, &args![lighting, 0x1cu32, 1u32]);
                let scale = e.get(this, Decal::fParallaxOcclusionScale);
                e.call(LIGHTING_SET_PARALLAX_SCALE, &args![lighting, scale]);
                let passes = e.get(this, Decal::ucParallaxOcclusionMaxPasses) as f32;
                fn_0068c8b0(e, Ptr::new(lighting), passes);
            }
        }
        e.call(SHADER_SET_FLAG, &args![lighting, 0x1bu32, 1u32]);
        let target = ni_pointer_get(e, t + Decal::spTargetNode.off);
        let first = e.call(TARGET_LINK_A, &args![target]).u32();
        let second = e.call(TARGET_LINK_B, &args![first]).u32();
        if fn_0068c870(e, Ptr::new(second)) {
            e.call(SHADER_SET_FLAG, &args![lighting, 0x35u32, 1u32]);
        }
        let shape = ni_pointer_get(e, t + Decal::sp3D.off);
        e.call(ATTACH_PROPERTY, &args![shape, lighting]);
        let shape = ni_pointer_get(e, t + Decal::sp3D.off);
        e.call(PREPARE_OBJECT, &args![shape, 0u32, 0u32]);
    } else {
        let no_lighting = new_object(e, 0x80, NO_LIGHTING_PROPERTY_CONSTRUCT, &[]);
        let texture_set = e.get(this, Decal::pTextureSet);
        if !forced_no_lighting
            && !texture_set.is_null()
            && e.global::<u32>(DECAL_DEBUG_FLAGS) & 0x20 == 0
        {
            e.with_stack(4, |e, texture| {
                e.call(NI_POINTER_CONSTRUCT, &args![texture, 0u32]);
                // The texture set's second base (+0x30), virtual slot 0x90.
                let base = texture_set.addr() + 0x30;
                e.vcall(base, 0x90, &args![0u32, texture]);
                let loaded = ni_pointer_get(e, texture.addr());
                e.call(NO_LIGHTING_SET_TEXTURE, &args![no_lighting, loaded]);
                e.call(NI_POINTER_RELEASE, &args![texture]);
            });
        }
        e.call(SHADER_SET_FLAG, &args![no_lighting, 0x1bu32, 1u32]);
        let target = ni_pointer_get(e, t + Decal::spTargetNode.off);
        let first = e.call(TARGET_LINK_A, &args![target]).u32();
        let second = e.call(TARGET_LINK_B, &args![first]).u32();
        if fn_0068c870(e, Ptr::new(second)) {
            e.call(SHADER_SET_FLAG, &args![no_lighting, 0x35u32, 1u32]);
        }
        let shape = ni_pointer_get(e, t + Decal::sp3D.off);
        e.call(ATTACH_PROPERTY, &args![shape, no_lighting]);
        if e.global::<u32>(DECAL_DEBUG_FLAGS) & 0x20 != 0 {
            let shape = ni_pointer_get(e, t + Decal::sp3D.off);
            let colors = e.call(GEOMETRY_COLORS, &args![shape]).u32();
            let color = e.mem.alloc(16);
            let mut i = 0i32;
            while i < e.call(GEOMETRY_VERTEX_COUNT, &args![shape]).u16() as i32 {
                let yellow = e
                    .call(
                        COLOR4_CONSTRUCT,
                        &args![color, 1.0f32, 1.0f32, 0.0f32, 1.0f32],
                    )
                    .u32();
                copy_words(e, colors + 16 * i as u32, yellow, 4);
                i += 1;
            }
            e.mem.free(color);
        }
        let shape = ni_pointer_get(e, t + Decal::sp3D.off);
        e.call(PREPARE_OBJECT, &args![shape, 0u32, 0u32]);
    }

    // Name the shape.
    let name = if e.get(this, Decal::bPermanent) {
        fn_0068c8d0(e)
    } else {
        e.call(TRANSIENT_NAME_WORD_GETTER, &args![]).u32()
    };
    let shape = ni_pointer_get(e, t + Decal::sp3D.off);
    e.call(SET_NAME, &args![shape, name]);
    let shape = ni_pointer_get(e, t + Decal::sp3D.off);
    e.call(REMOVE_HAVOK, &args![shape]);
    let shape = ni_pointer_get(e, t + Decal::sp3D.off);
    e.call(SET_OBJECT_TYPE_RECURSE, &args![shape, 5u32]);

    if !e.get(this, Decal::bPermanent) {
        if e.get(this, Decal::bForceFade) {
            e.call(ADD_TEMP_EFFECT, &args![PROCESS_LISTS, this]);
        } else {
            e.with_stack(4, |e, holder| {
                e.call(NI_POINTER_CONSTRUCT, &args![holder, this]);
                e.call(LIST_ADD, &args![DECAL_LIST, holder]);
                e.call(NI_POINTER_RELEASE, &args![holder]);
            });
            let count = e.global::<i32>(LISTED_DECAL_COUNT).wrapping_add(1);
            e.set_global(LISTED_DECAL_COUNT, count);
            if count > e.global::<i32>(MAX_DECAL_COUNT) {
                let list = e.call(EMPTY_CONSTRUCTOR, &args![DECAL_LIST]).u32();
                let oldest = e.call(NI_POINTER_GET, &args![list]).u32();
                e.call(ADD_TEMP_EFFECT, &args![PROCESS_LISTS, oldest]);
                e.call(LIST_REMOVE_HEAD, &args![DECAL_LIST]);
                let count = e.global::<i32>(LISTED_DECAL_COUNT).wrapping_sub(1);
                e.set_global(LISTED_DECAL_COUNT, count);
                let setting = e
                    .call(SETTING_BYTE_GETTER, &args![DIAGNOSTICS_SETTING])
                    .u32();
                if e.mem.u8(setting) != 0 {
                    let limit = e.global::<i32>(MAX_DECAL_COUNT);
                    e.call(PRINT_LINE, &args![DECAL_LIMIT_MESSAGE, limit]);
                }
            }
        }
    }

    // The node the decal hangs under: a child of the target's parent.
    let target = ni_pointer_get(e, t + Decal::spTargetNode.off);
    let parent = e.call(NODE_PARENT, &args![target]).u32();
    if parent == 0 {
        e.set(this, Decal::bFinished, true);
        return false;
    }
    let mut found = 0u32;
    let mut i = 0u32;
    while i < e.call(NODE_CHILD_COUNT, &args![parent]).u32() {
        let child = e.call(NODE_CHILD, &args![parent, i]).u32();
        if child != 0
            && e.call(IS_KIND_OF_CHILD, &args![DECAL_NODE_RTTI, child])
                .bool()
        {
            found = child;
            break;
        }
        i += 1;
    }
    let decal_node = if found == 0 {
        let target = ni_pointer_get(e, t + Decal::spTargetNode.off);
        let target_parent = e.call(NODE_PARENT, &args![target]).u32();
        new_object(e, 0xb4, DECAL_NODE_CONSTRUCT, &[target_parent, 0u32])
    } else {
        found
    };
    e.set(this, Decal::pParentNode, Ptr::new(decal_node));
    let decal_node = e.get(this, Decal::pParentNode).addr();
    // Slot 0x104 of the decal node: add this decal (argument 1).
    e.vcall(decal_node, 0x104, &args![this, 1u32]);

    let target = ni_pointer_get(e, t + Decal::spTargetNode.off);
    let translate = e.call(NODE_LOCAL_TRANSLATE, &args![target]).u32();
    let shape = ni_pointer_get(e, t + Decal::sp3D.off);
    e.call(NODE_SET_TRANSLATE, &args![shape, translate]);
    let target = ni_pointer_get(e, t + Decal::spTargetNode.off);
    let rotate = e.call(NODE_LOCAL_ROTATE, &args![target]).u32();
    let shape = ni_pointer_get(e, t + Decal::sp3D.off);
    e.call(NODE_SET_ROTATE, &args![shape, rotate]);
    let target = ni_pointer_get(e, t + Decal::spTargetNode.off);
    let scale = e.call(NODE_LOCAL_SCALE, &args![target]).f32();
    let shape = ni_pointer_get(e, t + Decal::sp3D.off);
    e.call(NODE_SET_SCALE, &args![shape, scale]);
    let shape = ni_pointer_get(e, t + Decal::sp3D.off);
    e.call(NODE_UPDATE_PROPERTIES, &args![shape]);

    e.with_stack(12, |e, point| {
        let origin_x = e.global::<f32>(ORIGIN_X);
        e.call(WORLD_POINT_CONSTRUCT, &args![point, origin_x, 0u32, 0u32]);
        let shape = ni_pointer_get(e, t + Decal::sp3D.off);
        e.call(NODE_SET_WORLD_POINT, &args![shape, point]);
    });
    let object = e.vcall(t, 0x98, &args![]).u32();
    let scene_node = e.call(SHADOW_SCENE_NODE_GETTER, &args![0u32]).u32();
    e.call(UPDATE_OBJECT_LIGHTING, &args![scene_node, object, 0u32]);

    e.with_stack(4, |e, holder| {
        e.call(NI_POINTER_CONSTRUCT, &args![holder, this]);
        let node = e.get(this, Decal::pParentNode).addr();
        let member = e.mem.u32(node + 0xac);
        e.call(DECAL_NODE_MEMBER_ADD, &args![member, holder]);
        e.call(NI_POINTER_RELEASE, &args![holder]);
    });
    bstempeffectsimpledecal_clean_up_memory(e, this);
    e.set(this, Decal::bFinalize, false);
    e.set(this, Decal::bValidDecal, true);
    e.set(this, Decal::bFinished, true);
    true
}

// Translated from 0068c870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tests bit 0 of the 16-bit word at `+0x18` of `this` (calls `0068c890`
/// with mask 1).
pub fn fn_0068c870(e: &mut Engine, this: Ptr) -> bool {
    fn_0068c890(e, this, 1)
}

// Translated from 0068c890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether any bit of `mask` is set in the 16-bit word at `+0x18` of `this`.
pub fn fn_0068c890(e: &mut Engine, this: Ptr, mask: u16) -> bool {
    e.mem.u16(this.addr() + 0x18) & mask != 0
}

// Translated from 0068c8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the `float` at `+0xE8` of `this` (`FinalizeGeometry`
/// calls it on the lighting property it creates, with the parallax pass
/// count).
pub fn fn_0068c8b0(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_f32(this.addr() + 0xe8, x87_quiet(value));
}

// Translated from 0068c8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `011c622c` (`FinalizeGeometry` passes it to the name
/// setter for permanent decals).
pub fn fn_0068c8d0(e: &mut Engine) -> u32 {
    e.global(PERMANENT_NAME_WORD)
}

// Translated from 0068c8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTempEffectSimpleDecal::Update` (Xbox PDB): runs the base update, which
/// returns true while the effect is alive. When it has expired and the decal
/// has a shape, the time since expiry decides: within one second the
/// material property's alpha is set to `1 - elapsed` (when it differs from
/// the current alpha) and the decal stays alive (returns true); later the
/// decal is removed from the shape's parent and the base result (false) is
/// returned. Without a shape the base result is returned.
pub fn bstempeffectsimpledecal_update(e: &mut Engine, this: Ptr<Decal>, delta: f32) -> bool {
    let t = this.addr();
    let alive = e.call(BASE_UPDATE, &args![this, delta]).bool();
    if alive {
        return alive;
    }
    let shape = ni_pointer_get(e, t + Decal::sp3D.off);
    if shape.is_null() {
        return alive;
    }
    let age = e.call(EFFECT_AGE, &args![this]).f64();
    let lifetime = e.call(EFFECT_LIFETIME, &args![this]).f64();
    let overshoot = (age - lifetime) as f32;
    let zero: f64 = e.global(ZERO);
    let one: f64 = e.global(ONE);
    if overshoot as f64 >= zero && overshoot as f64 <= one {
        let alpha = ((1.0f64 - overshoot as f64) / one) as f32;
        let property_type = e.call(MATERIAL_PROPERTY_TYPE, &args![]).u32();
        let shape = ni_pointer_get(e, t + Decal::sp3D.off);
        let property = e
            .call(NODE_GET_PROPERTY, &args![shape, property_type])
            .u32();
        if property != 0 {
            let current = e.call(MATERIAL_GET_ALPHA, &args![property]).f64();
            if alpha as f64 != current {
                fn_0068c9f0(e, Ptr::new(property), alpha);
            }
        }
        return true;
    }
    let shape = ni_pointer_get(e, t + Decal::sp3D.off);
    let parent = e.call(NODE_PARENT, &args![shape]).u32();
    if parent != 0 {
        e.call(PARENT_REMOVE_DECAL, &args![parent, this]);
    }
    alive
}

// Translated from 0068c9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the `float` at `+0x3C` of `this` (the material
/// property's alpha, from `Update`) and calls `004bc480` with 1 (which
/// increments the word at `+0x44`).
pub fn fn_0068c9f0(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_f32(this.addr() + 0x3c, value);
    e.call(MARK_CHANGED, &args![this, 1u32]);
}

// Translated from 0068ca10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTempEffectSimpleDecal::CreateTriShape` (Xbox PDB name of this
/// address, shared by identical code with other `Create` functions): a new
/// 0xC4-byte `NiTriShape` built from `data` by `00a74480`, or null.
// Translation incomplete: 0068ca10 - the compiler's exception-unwinding frame (FS:[0], its state changes and handler) is not translated.
pub fn bstempeffectsimpledecal_create_tri_shape(e: &mut Engine, data: Ptr) -> Ptr {
    Ptr::new(new_object(e, 0xc4, TRI_SHAPE_CONSTRUCT, &[data.addr()]))
}

// Translated from 0068ca90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTempEffectSimpleDecal::CreateTriData` (Xbox PDB): a new 0x58-byte
/// `NiTriShapeData` from the final arrays: vertex count, vertices, normals,
/// colours, texture coordinates, `1`, `0`, triangle count and indices
/// (constructor `00a7b630`), or null.
// Translation incomplete: 0068ca90 - the compiler's exception-unwinding frame (FS:[0], its state changes and handler) is not translated.
pub fn bstempeffectsimpledecal_create_tri_data(e: &mut Engine, this: Ptr<Decal>) -> Ptr {
    let memory = e.call(OPERATOR_NEW, &args![0x58u32]).u32();
    if memory == 0 {
        return Ptr::NULL;
    }
    let vertex_count = e.get(this, Decal::iDecalVertexCount) as u16 as u32;
    let vertices = e.get(this, Decal::pFinalVertexArray);
    let normals = e.get(this, Decal::pFinalNormalArray);
    let colors = e.get(this, Decal::pFinalColorArray);
    let texcoords = e.get(this, Decal::pFinalTexcoordArray);
    let triangle_count = e.get(this, Decal::iDecalTriangleCount) as u16 as u32;
    let indices = e.get(this, Decal::pFinalIndexArray);
    e.call(
        TRI_DATA_CONSTRUCT,
        &args![
            memory,
            vertex_count,
            vertices,
            normals,
            colors,
            texcoords,
            1u32,
            0u32,
            triangle_count,
            indices
        ],
    )
    .ptr()
}

// Translated from 0068cb60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds one clipped polygon to the decal (no name in the engine map; called
/// by `ClipMesh`). `indices` holds `count` 32-bit entries, `-1` for a vertex
/// the clipping created and an index of the target's vertex otherwise;
/// `vertices` and `normals` are the polygon's points. Returns false when the
/// decal would reach 0x200 vertices.
///
/// The first time it allocates the five scrap-heap arrays (0x1800, 0x1800,
/// 0x1000, 0x2000 and 0x1800 bytes) and the old-to-new index array (`i16`,
/// three per triangle of the target, filled with -1). Each entry is mapped to
/// a decal vertex: an existing mapping is reused; a new clipping vertex is
/// first looked up among the created `DecalVertex`es by position (within
/// the weld distance), then appended with a new `DecalVertex`; either way a
/// new vertex gets position, normal and a colour whose alpha is
/// `max(0, (dot(decal normal, normal) / |normal| - limit) / (1 - limit))`.
/// Finally the polygon is added as a triangle fan.
// Translation incomplete: 0068cb60 - the compiler's exception-unwinding frame (FS:[0], its state changes and handler) is not translated.
pub fn fn_0068cb60(
    e: &mut Engine,
    this: Ptr<Decal>,
    count: i32,
    indices: Ptr,
    vertices: Ptr,
    normals: Ptr,
) -> bool {
    let t = this.addr();
    let mut next = e.get(this, Decal::iDecalVertexCount);
    if next + count >= 0x200 {
        return false;
    }
    if e.get(this, Decal::pTempVertexArray).is_null() {
        let manager = e.call(MEMORY_MANAGER_GETTER, &args![]).u32();
        let heap = e.call(THREAD_SCRAP_HEAP, &args![manager]).u32();
        let alignment = e.global::<u32>(SCRAP_ALIGNMENT);
        let array = e
            .call(SCRAP_ALLOCATE, &args![heap, 0x1800u32, alignment])
            .ptr();
        e.set(this, Decal::pTempVertexArray, array);
        let array = e
            .call(SCRAP_ALLOCATE, &args![heap, 0x1800u32, alignment])
            .ptr();
        e.set(this, Decal::pTempNormalArray, array);
        let array = e
            .call(SCRAP_ALLOCATE, &args![heap, 0x1000u32, alignment])
            .ptr();
        e.set(this, Decal::pTempTexcoordArray, array);
        let array = e
            .call(SCRAP_ALLOCATE, &args![heap, 0x2000u32, alignment])
            .ptr();
        e.set(this, Decal::pTempColorArray, array);
        let array = e
            .call(SCRAP_ALLOCATE, &args![heap, 0x1800u32, alignment])
            .ptr();
        e.set(this, Decal::pTempIndexArray, array);
        let target = ni_pointer_get(e, t + Decal::spTargetNode.off);
        let triangles = e.call(GEOMETRY_TRIANGLE_COUNT, &args![target]).u16() as u32;
        let map = e.call(MEMORY_ALLOCATE, &args![triangles * 3 * 2]).ptr();
        e.set(this, Decal::pOldToNewIndexArray, map);
        e.call(MEMORY_FILL, &args![map, 0xffff_ffffu32, triangles * 3 * 2]);
    }
    let limit = e.global::<f32>(CLIP_ANGLE_LIMIT);
    let reciprocal = (1.0f64 / (1.0f64 - limit as f64)) as f32;

    let mut j = 0i32;
    while j < count {
        let entry = e.mem.i32(indices.addr() + 4 * j as u32);
        let source_vertex = vertices.addr() + 12 * j as u32;
        let source_normal = normals.addr() + 12 * j as u32;
        if entry == -1 {
            // Look for a vertex created before at the same position.
            let map = t + Decal::pNewIndexMap.off;
            let mut found = -1i32;
            let mut k = 0u32;
            while k < e.call(ARRAY_SIZE, &args![map]).u32() {
                let weld = e.global::<f32>(WELD_DISTANCE);
                let slot = e.call(ARRAY_ELEMENT, &args![map, k]).u32();
                let existing = e.mem.u32(slot);
                if e.call(POINTS_NEARLY_EQUAL, &args![source_vertex, existing, weld])
                    .bool()
                {
                    let slot = e.call(ARRAY_ELEMENT, &args![map, k]).u32();
                    let existing = e.mem.u32(slot);
                    found = e.mem.i16(existing + 0xc) as i32;
                    break;
                }
                k += 1;
            }
            if found != -1 {
                e.mem.set_i32(indices.addr() + 4 * j as u32, found);
            } else {
                let temp_vertices = e.get(this, Decal::pTempVertexArray).addr();
                copy_words(e, temp_vertices + 12 * next as u32, source_vertex, 3);
                let created = new_object(e, 0x10, DECAL_VERTEX_CONSTRUCT, &[]);
                let temp_vertices = e.get(this, Decal::pTempVertexArray).addr();
                copy_words(e, created, temp_vertices + 12 * next as u32, 3);
                e.mem.set_i16(created + 0xc, next as i16);
                e.with_stack(4, |e, holder| {
                    e.mem.set_u32(holder.addr(), created);
                    e.call(ARRAY_APPEND, &args![map, holder]);
                });
                add_vertex_data(e, this, next, source_normal, limit, reciprocal);
                e.mem.set_i32(indices.addr() + 4 * j as u32, next);
                next += 1;
            }
        } else {
            let map = e.get(this, Decal::pOldToNewIndexArray).addr();
            let mapped = e.mem.i16(map + 2 * entry as u32);
            if mapped == -1 {
                let temp_vertices = e.get(this, Decal::pTempVertexArray).addr();
                copy_words(e, temp_vertices + 12 * next as u32, source_vertex, 3);
                e.mem.set_i16(map + 2 * entry as u32, next as i16);
                add_vertex_data(e, this, next, source_normal, limit, reciprocal);
                e.mem.set_i32(indices.addr() + 4 * j as u32, next);
                next += 1;
            } else {
                e.mem.set_i32(indices.addr() + 4 * j as u32, mapped as i32);
            }
        }
        j += 1;
    }

    let triangle_count = e.get(this, Decal::iDecalTriangleCount);
    let out = e.get(this, Decal::pTempIndexArray).addr() + 6 * triangle_count as u32;
    e.set(this, Decal::iDecalTriangleCount, count - 2 + triangle_count);
    let mut written = 0u32;
    let mut k = 2i32;
    while k < count {
        let first = e.mem.u16(indices.addr());
        e.mem.set_u16(out + 2 * written, first);
        written += 1;
        let previous = e.mem.u16(indices.addr() + 4 * (k as u32 - 1));
        e.mem.set_u16(out + 2 * written, previous);
        written += 1;
        let current = e.mem.u16(indices.addr() + 4 * k as u32);
        e.mem.set_u16(out + 2 * written, current);
        written += 1;
        k += 1;
    }
    e.set(this, Decal::iDecalVertexCount, next);
    true
}

/// The normal and colour of decal vertex `next` (the common tail of the
/// three vertex-creating paths of [`fn_0068cb60`]): the normal is copied,
/// the colour is the decal's `fVertexColor` with alpha
/// `max(0, (dot(decal normal, normal) / length - limit) * reciprocal)`.
fn add_vertex_data(
    e: &mut Engine,
    this: Ptr<Decal>,
    next: i32,
    source_normal: u32,
    limit: f32,
    reciprocal: f32,
) {
    let t = this.addr();
    let normals = e.get(this, Decal::pTempNormalArray).addr();
    let stored = normals + 12 * next as u32;
    copy_words(e, stored, source_normal, 3);
    let local = e.mem.alloc(12);
    copy_words(e, local, stored, 3);
    let dot = e
        .call(POINT3_DOT, &args![t + Decal::pDecalNormal.off, local])
        .f64();
    let length = e.call(POINT3_LENGTH, &args![local]).f64();
    let alpha = (((dot / length) - limit as f64) * reciprocal as f64) as f32;
    e.mem.free(local);
    let alpha = if alpha > 0.0 { alpha } else { 0.0 };
    let (red, green, blue) = (
        e.get(this, Decal::fVertexColor),
        e.get(this, Decal::fVertexColorGreen),
        e.get(this, Decal::fVertexColorBlue),
    );
    let color = e.mem.alloc(16);
    let result = e
        .call(COLOR4_CONSTRUCT, &args![color, red, green, blue, alpha])
        .u32();
    let colors = e.get(this, Decal::pTempColorArray).addr();
    copy_words(e, colors + 16 * next as u32, result, 4);
    e.mem.free(color);
}

// Translated from 0068d230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTempEffectSimpleDecal::ClipMesh` (Xbox PDB): clips the triangles of
/// `geometry` (the geometry's vertices, normals and its strip data) against
/// the six planes. A geometry with vertices but no normals only prints a
/// diagnostic; one without strip data does nothing. Each strip triangle
/// (consecutive index triples, the winding flipped for odd positions;
/// degenerate ones skipped) whose normal faces the decal enough
/// (`|normal| * limit < dot(decal normal, normal)`) is clipped
/// ([`bstempeffectsimpledecal_clip_polygon`]) and the result added by
/// [`fn_0068cb60`]; the loop stops when that refuses a polygon.
pub fn bstempeffectsimpledecal_clip_mesh(e: &mut Engine, this: Ptr<Decal>, geometry: Ptr) {
    let t = this.addr();
    let vertices = e.call(GEOMETRY_VERTICES, &args![geometry]).u32();
    let normals = e.call(GEOMETRY_NORMALS, &args![geometry]).u32();
    let strip_data = e.call(GEOMETRY_STRIP_DATA, &args![geometry]).u32();
    if vertices != 0 && normals == 0 {
        let holder = e.call(GEOMETRY_NAME_HOLDER, &args![geometry]).u32();
        let name = e.call(NAME_TEXT, &args![holder]).u32();
        e.call(PRINT_LINE, &args![NO_NORMALS_MESSAGE, name]);
        return;
    }
    if strip_data == 0 {
        return;
    }
    let mut frame = Frame::new(e, 0x300);
    let polygon_vertices = frame.take(0x6c);
    let polygon_normals = frame.take(0x6c);
    e.call(
        VECTOR_CONSTRUCTOR,
        &args![polygon_vertices, 12u32, 9i32, EMPTY_CONSTRUCTOR],
    );
    e.call(
        VECTOR_CONSTRUCTOR,
        &args![polygon_normals, 12u32, 9i32, EMPTY_CONSTRUCTOR],
    );
    let polygon_indices = frame.take(12);
    let edge_first = frame.take(12);
    let edge_second = frame.take(12);
    let cross = frame.take(12);
    let count = e.call(GEOMETRY_TRIANGLE_COUNT, &args![geometry]).u16() as i32;
    let strip = e.call(STRIP_INDICES, &args![strip_data]).u32();
    let mut i = 0i32;
    while i < count {
        let first = e.mem.u16(strip + 2 * i as u32) as u32;
        let second = e.mem.u16(strip + 2 * i as u32 + 2) as u32;
        let third = e.mem.u16(strip + 2 * i as u32 + 4) as u32;
        if first != second && first != third && second != third {
            let (a, b, c) = if i % 2 == 0 {
                (first, second, third)
            } else {
                (second, first, third)
            };
            let point_a = vertices + a * 12;
            let point_b = vertices + b * 12;
            let point_c = vertices + c * 12;
            e.call(POINT3_SUBTRACT, &args![point_b, edge_first, point_a]);
            e.call(POINT3_SUBTRACT, &args![point_c, edge_second, point_a]);
            e.call(POINT3_CROSS, &args![edge_first, cross, edge_second]);
            let facing = e
                .call(POINT3_DOT, &args![t + Decal::pDecalNormal.off, cross])
                .f64();
            let length = e.call(POINT3_LENGTH, &args![cross]).f64();
            let limit = e.global::<f32>(CLIP_ANGLE_LIMIT);
            if length * (limit as f64) < facing {
                e.mem.set_u32(polygon_indices, a);
                e.mem.set_u32(polygon_indices + 4, b);
                e.mem.set_u32(polygon_indices + 8, c);
                copy_words(e, polygon_vertices, point_a, 3);
                copy_words(e, polygon_vertices + 12, point_b, 3);
                copy_words(e, polygon_vertices + 24, point_c, 3);
                copy_words(e, polygon_normals, normals + a * 12, 3);
                copy_words(e, polygon_normals + 12, normals + b * 12, 3);
                copy_words(e, polygon_normals + 24, normals + c * 12, 3);
                let clipped = bstempeffectsimpledecal_clip_polygon(
                    e,
                    this,
                    3,
                    Ptr::new(polygon_indices),
                    Ptr::new(polygon_vertices),
                    Ptr::new(polygon_normals),
                    Ptr::new(polygon_indices),
                    Ptr::new(polygon_vertices),
                    Ptr::new(polygon_normals),
                );
                if clipped != 0
                    && !fn_0068cb60(
                        e,
                        this,
                        clipped,
                        Ptr::new(polygon_indices),
                        Ptr::new(polygon_vertices),
                        Ptr::new(polygon_normals),
                    )
                {
                    break;
                }
            }
        }
        i += 1;
    }
    frame.release(e);
}

// Translated from 0068d660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTempEffectSimpleDecal::ClipPolygon` (Xbox PDB): clips the polygon of
/// `count` points (`indices`, `vertices`, `normals`) against the six planes
/// in the order left, right, bottom, top, back, front, alternating between
/// the result arrays and temporaries; the final result is in the `*_out`
/// arrays. Returns the number of points left, 0 as soon as a plane leaves
/// none.
#[allow(clippy::too_many_arguments)]
pub fn bstempeffectsimpledecal_clip_polygon(
    e: &mut Engine,
    this: Ptr<Decal>,
    count: i32,
    indices_in: Ptr,
    vertices_in: Ptr,
    normals_in: Ptr,
    indices_out: Ptr,
    vertices_out: Ptr,
    normals_out: Ptr,
) -> i32 {
    let planes = this.addr() + Decal::pClippingPlanes.off;
    let mut frame = Frame::new(e, 0x200);
    let temp_vertices = frame.take(0x6c);
    let temp_normals = frame.take(0x6c);
    let temp_indices = frame.take(0x24);
    e.call(
        VECTOR_CONSTRUCTOR,
        &args![temp_vertices, 12u32, 9i32, EMPTY_CONSTRUCTOR],
    );
    e.call(
        VECTOR_CONSTRUCTOR,
        &args![temp_normals, 12u32, 9i32, EMPTY_CONSTRUCTOR],
    );
    let (ti, tv, tn) = (
        Ptr::new(temp_indices),
        Ptr::new(temp_vertices),
        Ptr::new(temp_normals),
    );
    let mut result = bstempeffectsimpledecal_clip_polygon_against_plane(
        e,
        Ptr::new(planes),
        count,
        indices_in,
        vertices_in,
        normals_in,
        ti,
        tv,
        tn,
    );
    if result != 0 {
        result = bstempeffectsimpledecal_clip_polygon_against_plane(
            e,
            Ptr::new(planes + 0x10),
            result,
            ti,
            tv,
            tn,
            indices_out,
            vertices_out,
            normals_out,
        );
        if result != 0 {
            result = bstempeffectsimpledecal_clip_polygon_against_plane(
                e,
                Ptr::new(planes + 0x20),
                result,
                indices_out,
                vertices_out,
                normals_out,
                ti,
                tv,
                tn,
            );
            if result != 0 {
                result = bstempeffectsimpledecal_clip_polygon_against_plane(
                    e,
                    Ptr::new(planes + 0x30),
                    result,
                    ti,
                    tv,
                    tn,
                    indices_out,
                    vertices_out,
                    normals_out,
                );
                if result != 0 {
                    result = bstempeffectsimpledecal_clip_polygon_against_plane(
                        e,
                        Ptr::new(planes + 0x50),
                        result,
                        indices_out,
                        vertices_out,
                        normals_out,
                        ti,
                        tv,
                        tn,
                    );
                    if result != 0 {
                        result = bstempeffectsimpledecal_clip_polygon_against_plane(
                            e,
                            Ptr::new(planes + 0x40),
                            result,
                            ti,
                            tv,
                            tn,
                            indices_out,
                            vertices_out,
                            normals_out,
                        );
                    }
                }
            }
        }
    }
    frame.release(e);
    result
}

// Translated from 0068d820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTempEffectSimpleDecal::ClipPolygonAgainstPlane` (Xbox PDB), a `cdecl`
/// function taking the plane first: clips the polygon of `count` points
/// against `plane`, keeping the points in front of or on it. Every point
/// is classified by `0049da80` (2 means behind); when all are behind it
/// returns 0. Walking the polygon, for each point and its predecessor: an
/// edge that crosses the plane adds a new point at parameter
/// `t = (n . inside - c) / (n . (inside - outside))` (index -1, position
/// `inside * (1 - t) + outside * t`, normal `outside_normal * (1 - t) +
/// inside_normal * t`), `t` being corrected or reported ("Bad t value")
/// when outside 0..1 (an edge entering the plane from behind allows 1.0001
/// before correcting to 1.0); a point in front is copied after that. Returns
/// the number of points written.
// Translation incomplete: 0068d820 - the /GS security-cookie check (00ec408c) at the end of the function is not translated.
#[allow(clippy::too_many_arguments)]
pub fn bstempeffectsimpledecal_clip_polygon_against_plane(
    e: &mut Engine,
    plane: Ptr,
    count: i32,
    indices_in: Ptr,
    vertices_in: Ptr,
    normals_in: Ptr,
    indices_out: Ptr,
    vertices_out: Ptr,
    normals_out: Ptr,
) -> i32 {
    let mut behind = vec![false; count.max(0) as usize];
    let mut behind_count = 0;
    for (i, flag) in behind.iter_mut().enumerate() {
        let side = e
            .call(
                PLANE_SIDE,
                &args![plane, vertices_in.addr() + 12 * i as u32],
            )
            .i32();
        if side == 2 {
            *flag = true;
            behind_count += 1;
        }
    }
    if behind_count == count {
        return 0;
    }
    let mut written = 0u32;
    for i in 0..count {
        let previous = if i != 0 { i - 1 } else { count - 1 };
        let current_behind = behind[i as usize];
        let previous_behind = behind[previous as usize];
        if current_behind {
            if !previous_behind {
                // The edge leaves the front: previous is inside, current outside.
                let tolerant = true;
                add_crossing_point(
                    e,
                    plane,
                    previous as u32,
                    i as u32,
                    tolerant,
                    (vertices_in, normals_in),
                    (indices_out, vertices_out, normals_out),
                    written,
                );
                written += 1;
            }
        } else {
            if previous_behind {
                // The edge enters the front: current is inside, previous outside.
                add_crossing_point(
                    e,
                    plane,
                    i as u32,
                    previous as u32,
                    false,
                    (vertices_in, normals_in),
                    (indices_out, vertices_out, normals_out),
                    written,
                );
                written += 1;
            }
            let index = e.mem.u32(indices_in.addr() + 4 * i as u32);
            e.mem.set_u32(indices_out.addr() + 4 * written, index);
            copy_words(
                e,
                vertices_out.addr() + 12 * written,
                vertices_in.addr() + 12 * i as u32,
                3,
            );
            copy_words(
                e,
                normals_out.addr() + 12 * written,
                normals_in.addr() + 12 * i as u32,
                3,
            );
            written += 1;
        }
    }
    written as i32
}

/// The point where the edge between the inside point `inside` and the outside
/// point `outside` (indices into the input arrays) crosses `plane`, written
/// as output point `slot`. The parameter is the plane equation solved along
/// `inside - outside`; `tolerant` allows the 1.0001 correction.
#[allow(clippy::too_many_arguments)]
fn add_crossing_point(
    e: &mut Engine,
    plane: Ptr,
    inside: u32,
    outside: u32,
    tolerant: bool,
    inputs: (Ptr, Ptr),
    outputs: (Ptr, Ptr, Ptr),
    slot: u32,
) {
    let (vertices_in, normals_in) = inputs;
    let (indices_out, vertices_out, normals_out) = outputs;
    let mut frame = Frame::new(e, 0x100);
    let inside_point = frame.take(12);
    copy_words(e, inside_point, vertices_in.addr() + 12 * inside, 3);
    let outside_point = frame.take(12);
    copy_words(e, outside_point, vertices_in.addr() + 12 * outside, 3);
    let difference = frame.take(12);
    e.call(
        POINT3_SUBTRACT,
        &args![inside_point, difference, outside_point],
    );
    let normal_holder = frame.take(12);
    let normal_source = e.call(PLANE_NORMAL, &args![plane]).u32();
    copy_words(e, normal_holder, normal_source, 3);
    let along_inside = e
        .call(POINT3_DOT, &args![normal_holder, inside_point])
        .f64();
    let constant = e.call(PLANE_CONSTANT, &args![plane]).f64();
    let numerator = -constant + along_inside;
    let along_difference = e.call(POINT3_DOT, &args![normal_holder, difference]).f64();
    let mut t = (numerator / along_difference) as f32;
    if tolerant {
        let tolerance: f64 = e.global(CLIP_PARAMETER_TOLERANCE);
        let one: f64 = e.global(ONE);
        if t as f64 > one && t as f64 <= tolerance {
            t = 1.0;
        }
    }
    let zero: f64 = e.global(ZERO);
    let one: f64 = e.global(ONE);
    if (t as f64) < zero {
        e.call(PRINT_LINE, &args![BAD_T_MESSAGE, t as f64]);
        t = 0.0;
    } else if t as f64 > one {
        e.call(PRINT_LINE, &args![BAD_T_MESSAGE, t as f64]);
        t = 1.0;
    }
    e.mem.set_u32(indices_out.addr() + 4 * slot, 0xffff_ffff);

    // Position: inside * (1 - t) + outside * t.
    let outside_scaled = frame.take(12);
    let first = e
        .call(POINT3_SCALE, &args![outside_point, outside_scaled, t])
        .u32();
    let remainder = (1.0f64 - t as f64) as f32;
    let inside_scaled = frame.take(12);
    let second = e
        .call(POINT3_SCALE, &args![inside_point, inside_scaled, remainder])
        .u32();
    let position = frame.take(12);
    let sum = e.call(POINT3_ADD, &args![second, position, first]).u32();
    copy_words(e, vertices_out.addr() + 12 * slot, sum, 3);

    // Normal: outside normal * (1 - t) + inside normal * t.
    let inside_normal = frame.take(12);
    copy_words(e, inside_normal, normals_in.addr() + 12 * inside, 3);
    let outside_normal = frame.take(12);
    copy_words(e, outside_normal, normals_in.addr() + 12 * outside, 3);
    let inside_weighted = frame.take(12);
    let first = e
        .call(POINT3_SCALE, &args![inside_normal, inside_weighted, t])
        .u32();
    let remainder = (1.0f64 - t as f64) as f32;
    let outside_weighted = frame.take(12);
    let second = e
        .call(
            POINT3_SCALE,
            &args![outside_normal, outside_weighted, remainder],
        )
        .u32();
    let normal = frame.take(12);
    let sum = e.call(POINT3_ADD, &args![second, normal, first]).u32();
    copy_words(e, normals_out.addr() + 12 * slot, sum, 3);
    frame.release(e);
}

// Translated from 0068de30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Occlusion-query check (no name in the engine map; the fields are the Xbox
/// PDB's `pOcclusionQuery` at +0x1C and `iCurrentOcclusionResult` at +0x24):
/// asks the query (`00c4edf0` with the result field and 0) and, when it
/// answers, destroys the query (`004a4410` with 1), clears the field and
/// returns true; otherwise false.
pub fn fn_0068de30(e: &mut Engine, this: Ptr<Decal>) -> bool {
    let query = e.get(this, Decal::pOcclusionQuery);
    let result = this.addr() + Decal::iCurrentOcclusionResult.off;
    if !e
        .call(OCCLUSION_QUERY_RESULT, &args![query, result, 0u32])
        .bool()
    {
        return false;
    }
    let query = e.get(this, Decal::pOcclusionQuery);
    if !query.is_null() {
        e.call(OCCLUSION_QUERY_DELETE, &args![query, 1u32]);
    }
    e.set(this, Decal::pOcclusionQuery, Ptr::NULL);
    true
}

// Translated from 0068dea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTempEffectSimpleDecal::CreateDebugBox` (Xbox PDB): shows the decal's
/// volume as a box. The first call loads `DebugDecalBox.NIF` (when it has
/// exactly one root) into the static `spDebugDecalBox` and names it. Each call
/// makes a deep copy of it, finds its shape named `DebugDecalBox01:0`,
/// removes Havok, scales the shape's vertices by width, height and twice the
/// depth, sets every vertex colour to `*color`, zeroes near-zero components
/// of the decal normal and, when the normal equals the static point at
/// `011f426c`, takes the decal's vector (brought into the target's space)
/// instead. It orients the copy from the target (model space: the copy takes
/// the target's local transform and the shape the rotation about that vector
/// and the decal centre; otherwise the shape gets the world rotation of the
/// target turned by the same rotations and the decal origin), clears the
/// copy's properties, attaches an alpha property and a no-lighting property
/// (flag 0x30 when `flag` is set) to the shape, and adds the copy as a
/// debug object (model space: under a new node attached to the target's
/// parent; otherwise as a temporary debug object of the `TES` singleton).
// Translation incomplete: 0068dea0 - the compiler's exception-unwinding frame (FS:[0], its state changes and handler) and the /GS security-cookie check (00ec408c) are not translated.
pub fn bstempeffectsimpledecal_create_debug_box(
    e: &mut Engine,
    this: Ptr<Decal>,
    color: Ptr,
    flag: bool,
) {
    let t = this.addr();
    if ni_pointer_get(e, DEBUG_DECAL_BOX).is_null() {
        e.with_stack(0x5d4, |e, stream| {
            e.call(STREAM_CONSTRUCT, &args![stream]);
            let loaded = e
                .call(STREAM_LOAD, &args![stream, DEBUG_BOX_PATH, 0u32])
                .bool();
            if loaded {
                if e.call(STREAM_ROOT_COUNT, &args![stream]).u32() == 1 {
                    let root = e.call(STREAM_ROOT, &args![stream, 0u32]).u32();
                    ni_pointer_assign(e, DEBUG_DECAL_BOX, Ptr::new(root));
                }
                e.with_stack(4, |e, name| {
                    let text = e
                        .call(FIXED_STRING_CONSTRUCT, &args![name, DEBUG_BOX_NAME])
                        .u32();
                    let debug_box = ni_pointer_get(e, DEBUG_DECAL_BOX);
                    e.call(SET_NAME, &args![debug_box, text]);
                    e.call(FIXED_STRING_RELEASE, &args![name]);
                });
            }
            e.call(STREAM_DESTRUCT, &args![stream]);
        });
    }
    if ni_pointer_get(e, DEBUG_DECAL_BOX).is_null() {
        return;
    }
    let mut frame = Frame::new(e, 0x400);
    let copy = frame.take(4);
    e.call(NI_POINTER_CONSTRUCT, &args![copy, 0u32]);
    let debug_box = ni_pointer_get(e, DEBUG_DECAL_BOX);
    e.call(DEEP_COPY, &args![debug_box, copy]);
    let root = ni_pointer_get(e, copy).addr();
    let name = frame.take(4);
    let text = e
        .call(FIXED_STRING_CONSTRUCT, &args![name, DEBUG_BOX_SHAPE_NAME])
        .u32();
    // Slot 0x9C of the copy: the child with that name.
    let shape = e.vcall(root, 0x9c, &args![text]).u32();
    e.call(FIXED_STRING_RELEASE, &args![name]);
    e.call(REMOVE_HAVOK, &args![root]);
    let vertices = e.call(GEOMETRY_VERTICES, &args![shape]).u32();
    let colors = e.call(GEOMETRY_COLORS, &args![shape]).u32();
    let width = e.get(this, Decal::fWidth);
    let height = e.get(this, Decal::fHeight);
    let depth = e.get(this, Decal::fDepth);
    let mut i = 0u32;
    while (i as i32) < e.call(GEOMETRY_VERTEX_COUNT, &args![shape]).u16() as i32 {
        let x = e.mem.f32(vertices + 12 * i);
        e.mem
            .set_f32(vertices + 12 * i, (x as f64 * width as f64) as f32);
        let y = e.mem.f32(vertices + 12 * i + 4);
        e.mem
            .set_f32(vertices + 12 * i + 4, (y as f64 * height as f64) as f32);
        let z = e.mem.f32(vertices + 12 * i + 8);
        e.mem.set_f32(
            vertices + 12 * i + 8,
            ((depth as f64 + depth as f64) * z as f64) as f32,
        );
        copy_words(e, colors + 16 * i, color.addr(), 4);
        i += 1;
    }
    let tolerance = e.global::<f32>(DEBUG_PLANE_TOLERANCE);
    let normal = t + Decal::pDecalNormal.off;
    for component in 0..3u32 {
        let value = e.mem.f32(normal + 4 * component);
        if e.call(FLOAT_NEARLY_EQUAL, &args![value, 0.0f32, tolerance])
            .bool()
        {
            e.mem.set_f32(normal + 4 * component, 0.0);
        }
    }
    let direction = frame.take(12);
    copy_words(e, direction, normal, 3);
    if e.call(POINT3_EQUALS, &args![direction, ZERO_POINT]).bool() {
        copy_words(e, direction, t + Decal::pVector.off, 3);
        let matrix = frame.take(36);
        construct_empty(e, matrix);
        let target = ni_pointer_get(e, t + Decal::spTargetNode.off);
        let rotation = if e.get(this, Decal::bModelSpace) {
            e.call(NODE_LOCAL_ROTATE, &args![target]).u32()
        } else {
            e.call(NODE_WORLD_ROTATE, &args![target]).u32()
        };
        copy_words(e, matrix, rotation, 9);
        let inverse = frame.take(36);
        e.call(MATRIX_INVERSE, &args![matrix, inverse]);
        let rotated = frame.take(12);
        let result = e
            .call(MATRIX_TRANSFORM_POINT, &args![inverse, rotated, direction])
            .u32();
        copy_words(e, direction, result, 3);
    }
    let rotation = frame.take(36);
    let (dx, dy, dz) = (
        e.mem.u32(direction),
        e.mem.u32(direction + 4),
        e.mem.u32(direction + 8),
    );
    e.call(ROTATION_Z_TO_VECTOR_TRANSPOSE, &args![rotation, dx, dy, dz]);
    let turn = frame.take(36);
    construct_empty(e, turn);
    let angle = e.get(this, Decal::fRotation);
    e.call(MATRIX_MAKE_Z_ROTATION, &args![turn, -angle]);
    if e.get(this, Decal::bModelSpace) {
        let target = ni_pointer_get(e, t + Decal::spTargetNode.off);
        let translate = e.call(NODE_LOCAL_TRANSLATE, &args![target]).u32();
        e.call(NODE_SET_TRANSLATE, &args![root, translate]);
        let target = ni_pointer_get(e, t + Decal::spTargetNode.off);
        let rotate = e.call(NODE_LOCAL_ROTATE, &args![target]).u32();
        e.call(NODE_SET_ROTATE, &args![root, rotate]);
        let target = ni_pointer_get(e, t + Decal::spTargetNode.off);
        let scale = e.call(NODE_LOCAL_SCALE, &args![target]).f32();
        e.call(NODE_SET_SCALE, &args![root, scale]);
        let turned = frame.take(36);
        let result = e
            .call(MATRIX_MULTIPLY, &args![rotation, turned, turn])
            .u32();
        e.call(NODE_SET_ROTATE, &args![shape, result]);
        e.call(
            NODE_SET_TRANSLATE,
            &args![shape, t + Decal::pDecalCenter.off],
        );
    } else {
        let target = ni_pointer_get(e, t + Decal::spTargetNode.off);
        let world = e.call(NODE_WORLD_ROTATE, &args![target]).u32();
        let first = frame.take(36);
        let step = e
            .call(MATRIX_MULTIPLY, &args![world, first, rotation])
            .u32();
        let second = frame.take(36);
        let result = e.call(MATRIX_MULTIPLY, &args![step, second, turn]).u32();
        e.call(NODE_SET_ROTATE, &args![shape, result]);
        e.call(NODE_SET_TRANSLATE, &args![shape, t + Decal::pOrigin.off]);
    }
    fn_0068e740(e, Ptr::new(root));

    let alpha = new_object(e, 0x1c, ALPHA_PROPERTY_CONSTRUCT, &[]);
    e.call(ALPHA_SET_BLENDING, &args![alpha, 1u32]);
    e.call(ATTACH_PROPERTY, &args![root, alpha]);
    let no_lighting = new_object(e, 0x80, NO_LIGHTING_PROPERTY_CONSTRUCT, &[]);
    if flag {
        e.call(SHADER_SET_FLAG, &args![no_lighting, 0x30u32, 1u32]);
    }
    e.call(ATTACH_PROPERTY, &args![shape, no_lighting]);
    e.call(PREPARE_OBJECT, &args![shape, 0u32, 0u32]);
    let lifetime = e.global::<f32>(DEBUG_BOX_LIFETIME);
    if e.get(this, Decal::bModelSpace) {
        let holder = new_object(e, 0xb0, DEBUG_HOLDER_CONSTRUCT, &[lifetime.to_bits()]);
        e.vcall(holder, 0xdc, &args![root, 1u32]);
        let target = ni_pointer_get(e, t + Decal::spTargetNode.off);
        let parent = e.call(NODE_PARENT, &args![target]).u32();
        e.vcall(parent, 0xdc, &args![holder, 1u32]);
    } else {
        let tes = e.global::<u32>(TES_SINGLETON);
        e.call(TES_ADD_DEBUG_OBJECT, &args![tes, root, lifetime]);
    }
    let parent = e.call(NODE_PARENT, &args![root]).u32();
    e.vcall(parent, 0xbc, &args![]);
    let point = frame.take(12);
    e.call(WORLD_POINT_CONSTRUCT, &args![point, 0.0f32, 0u32, 0u32]);
    e.call(NODE_SET_WORLD_POINT, &args![root, point]);
    e.call(NI_POINTER_RELEASE, &args![copy]);
    frame.release(e);
}

// Translated from 0068e740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties the list at `+0x24` of `this` (calls `004a4650`; on a node that
/// is its property list).
pub fn fn_0068e740(e: &mut Engine, this: Ptr) {
    e.call(PROPERTY_LIST_CLEAR, &args![this.addr() + 0x24]);
}

// Translated from 0068e760 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTempEffectSimpleDecal::ResizeDataArrays` (Xbox PDB): moves the
/// temporary arrays into right-sized permanent ones. For vertices, normals,
/// texture coordinates (8 bytes each) and colours (16 bytes each) it
/// allocates `iDecalVertexCount` elements (constructing each), copies the
/// data over and gives the scrap-heap array back; the indices (three
/// `u16` per triangle) go into a block from the memory manager the same
/// way. The temporary pointers are nulled.
// Translation incomplete: 0068e760 - the compiler's exception-unwinding frame (FS:[0], its state changes and handler) is not translated.
pub fn bstempeffectsimpledecal_resize_data_arrays(e: &mut Engine, this: Ptr<Decal>) {
    let manager = e.call(MEMORY_MANAGER_GETTER, &args![]).u32();
    let heap = e.call(THREAD_SCRAP_HEAP, &args![manager]).u32();
    let vertex_count = e.get(this, Decal::iDecalVertexCount) as u32;
    for (temporary, permanent, element_size, constructor) in [
        (
            Decal::pTempVertexArray,
            Decal::pFinalVertexArray,
            12u32,
            EMPTY_CONSTRUCTOR,
        ),
        (
            Decal::pTempNormalArray,
            Decal::pFinalNormalArray,
            12,
            EMPTY_CONSTRUCTOR,
        ),
        (
            Decal::pTempTexcoordArray,
            Decal::pFinalTexcoordArray,
            8,
            EMPTY_CONSTRUCTOR,
        ),
        (
            Decal::pTempColorArray,
            Decal::pFinalColorArray,
            16,
            COLOR_ELEMENT_CONSTRUCTOR,
        ),
    ] {
        // `new T[n]`: the size saturates to 0xFFFFFFFF when n * size overflows.
        let bytes = vertex_count.saturating_mul(element_size);
        let block = e.call(RUNTIME_NEW, &args![bytes]).u32();
        let block = if block == 0 {
            0
        } else {
            e.call(
                VECTOR_CONSTRUCTOR,
                &args![block, element_size, vertex_count, constructor],
            );
            block
        };
        e.set(this, permanent, Ptr::new(block));
        let source = e.get(this, temporary);
        e.call(
            MEMORY_COPY,
            &args![block, source, vertex_count.wrapping_mul(element_size)],
        );
        e.call(SCRAP_DEALLOCATE, &args![heap, source]);
        e.set(this, temporary, Ptr::NULL);
    }
    let triangle_count = e.get(this, Decal::iDecalTriangleCount) as u32;
    let indices = e
        .call(MEMORY_ALLOCATE, &args![triangle_count.wrapping_mul(6)])
        .ptr();
    e.set(this, Decal::pFinalIndexArray, indices);
    let source = e.get(this, Decal::pTempIndexArray);
    e.call(
        MEMORY_COPY,
        &args![indices, source, triangle_count.wrapping_mul(6)],
    );
    e.call(SCRAP_DEALLOCATE, &args![heap, source]);
    e.set(this, Decal::pTempIndexArray, Ptr::NULL);
}

// Translated from 0068eb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the `BSSimpleArray<DecalVertex *, 1024>` (no name in the
/// engine map): stores its vtable and calls the array constructor with
/// `(0, 0)`.
pub fn fn_0068eb00(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), DECAL_VERTEX_ARRAY_VTABLE);
    e.call(ARRAY_CONSTRUCT, &args![this, 0u32, 0u32]);
    this
}

// Translated from 0068eb30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of that array: stores its vtable and empties it, freeing the
/// buffer (`008454f0` with 1).
pub fn fn_0068eb30(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), DECAL_VERTEX_ARRAY_VTABLE);
    e.call(ARRAY_CLEAR, &args![this, 1u32]);
}

// Translated from 0068eb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<DecalVertex *, 1024>::_scalar_deleting_destructor_` (Xbox
/// PDB name of this address): the destructor, then `operator delete` when
/// bit 0 of `flags` is set.
pub fn fn_0068eb50(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0068eb30(e, this);
    if flags & 1 != 0 {
        e.call(RUNTIME_DELETE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0068acb0,
            bstempeffectsimpledecal_get_rtti(Ptr<Decal>) -> Ptr
        ),
        entry!(
            0x0068acc0,
            bstempeffectsimpledecal_get_3d(Ptr<Decal>) -> Ptr
        ),
        entry!(
            0x0068ace0,
            bstempeffectsimpledecal_scalar_deleting_destructor(Ptr<Decal>, u32) -> Ptr<Decal>
        ),
        entry!(
            0x0068ad20,
            fn_0068ad20(Ptr<Decal>, Ptr, f32, Ptr) -> Ptr<Decal>
        ),
        entry!(0x0068b050, fn_0068b050(f64, f32) -> f64),
        entry!(0x0068b080, bstempeffectsimpledecal_destructor(Ptr<Decal>)),
        entry!(0x0068b1e0, bstempeffectsimpledecal_initialize(Ptr<Decal>)),
        entry!(
            0x0068b220,
            bstempeffectsimpledecal_clean_up_memory(Ptr<Decal>)
        ),
        entry!(0x0068b3e0, fn_0068b3e0()),
        entry!(0x0068b4a0, fn_0068b4a0()),
        entry!(
            0x0068b4c0,
            bstempeffectsimpledecal_process_geometry(Ptr<Decal>, Ptr) -> bool
        ),
        entry!(
            0x0068be90,
            bstempeffectsimpledecal_finalize_geometry(Ptr<Decal>) -> bool
        ),
        entry!(0x0068c870, fn_0068c870(Ptr) -> bool),
        entry!(0x0068c890, fn_0068c890(Ptr, u16) -> bool),
        entry!(0x0068c8b0, fn_0068c8b0(Ptr, f32)),
        entry!(0x0068c8d0, fn_0068c8d0() -> u32),
        entry!(
            0x0068c8e0,
            bstempeffectsimpledecal_update(Ptr<Decal>, f32) -> bool
        ),
        entry!(0x0068c9f0, fn_0068c9f0(Ptr, f32)),
        entry!(0x0068ca10, bstempeffectsimpledecal_create_tri_shape(Ptr) -> Ptr),
        entry!(
            0x0068ca90,
            bstempeffectsimpledecal_create_tri_data(Ptr<Decal>) -> Ptr
        ),
        entry!(
            0x0068cb60,
            fn_0068cb60(Ptr<Decal>, i32, Ptr, Ptr, Ptr) -> bool
        ),
        entry!(
            0x0068d230,
            bstempeffectsimpledecal_clip_mesh(Ptr<Decal>, Ptr)
        ),
        entry!(
            0x0068d660,
            bstempeffectsimpledecal_clip_polygon(
                Ptr<Decal>,
                i32,
                Ptr,
                Ptr,
                Ptr,
                Ptr,
                Ptr,
                Ptr,
            ) -> i32
        ),
        entry!(
            0x0068d820,
            bstempeffectsimpledecal_clip_polygon_against_plane(Ptr, i32, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr) -> i32
        ),
        entry!(0x0068de30, fn_0068de30(Ptr<Decal>) -> bool),
        entry!(
            0x0068dea0,
            bstempeffectsimpledecal_create_debug_box(Ptr<Decal>, Ptr, bool)
        ),
        entry!(0x0068e740, fn_0068e740(Ptr)),
        entry!(
            0x0068e760,
            bstempeffectsimpledecal_resize_data_arrays(Ptr<Decal>)
        ),
        entry!(0x0068eb00, fn_0068eb00(Ptr) -> Ptr),
        entry!(0x0068eb30, fn_0068eb30(Ptr)),
        entry!(0x0068eb50, fn_0068eb50(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn value(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn float(st0: f64) -> Ret {
        Ret {
            st0,
            ..Ret::default()
        }
    }

    fn points(e: &Engine, address: u32, count: u32) -> Vec<[f32; 3]> {
        (0..count)
            .map(|i| {
                [
                    e.mem.f32(address + 12 * i),
                    e.mem.f32(address + 12 * i + 4),
                    e.mem.f32(address + 12 * i + 8),
                ]
            })
            .collect()
    }

    fn write_points(e: &mut Engine, address: u32, list: &[[f32; 3]]) {
        for (i, point) in list.iter().enumerate() {
            for (k, component) in point.iter().enumerate() {
                e.mem
                    .set_f32(address + 12 * i as u32 + 4 * k as u32, *component);
            }
        }
    }

    fn write_identity(e: &mut Engine, address: u32) {
        let identity = [1.0f32, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
        for (i, v) in identity.iter().enumerate() {
            e.mem.set_f32(address + 4 * i as u32, *v);
        }
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .map(|log| {
                log.iter()
                    .filter(|(a, _)| *a == address)
                    .map(|(_, args)| args.clone())
                    .collect()
            })
            .unwrap_or_default()
    }

    fn noops(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, |_, _| Ret::default());
        }
    }

    /// Constructors that only return `this`.
    fn returns_this(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, |_, a| value(a[0]));
        }
    }

    fn matrix_of(e: &Engine, address: u32) -> [f32; 9] {
        let mut m = [0.0; 9];
        for (i, v) in m.iter_mut().enumerate() {
            *v = e.mem.f32(address + 4 * i as u32);
        }
        m
    }

    fn multiply_vector(m: &[f32; 9], v: [f32; 3]) -> [f32; 3] {
        [
            m[2] * v[2] + m[1] * v[1] + m[0] * v[0],
            m[5] * v[2] + m[4] * v[1] + m[3] * v[0],
            m[8] * v[2] + m[7] * v[1] + m[6] * v[0],
        ]
    }

    /// An engine with the pages of the exe's data this unit reads, the
    /// constants in them, and doubles for the small helpers with the
    /// semantics their bodies show (`NiPointer`, `NiPoint3` and `NiMatrix3`
    /// arithmetic, planes, scrap heap, `BSSimpleArray`). A test doubles
    /// anything else it reaches.
    fn decal_engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x010a_2000u32,
            0x0101_1000,
            0x0101_2000,
            0x0101_3000,
            0x0101_7000,
            0x0101_8000,
            0x0102_3000,
            0x0103_1000,
            0x0106_9000,
            0x0119_c000,
            0x011c_3000,
            0x011c_5000,
            0x011c_6000,
            0x011c_7000,
            0x011d_6000,
            0x011d_e000,
            0x011e_0000,
            0x011f_4000,
            0x011f_9000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(HALF, 0.5f64);
        e.set_global(TWO, 2.0f64);
        e.set_global(ZERO, 0.0f64);
        e.set_global(ONE, 1.0f64);
        e.set_global(CLIP_PARAMETER_TOLERANCE, 1.0001f64);
        e.set_global(DEGREES_TO_RADIANS, std::f64::consts::PI / 180.0);
        e.set_global(CLIP_ANGLE_FLOOR, 0.01f64);
        e.set_global(WELD_DISTANCE, 0.01f32);
        e.set_global(DEBUG_PLANE_TOLERANCE, 0.001f32);
        e.set_global(DEBUG_BOX_LIFETIME, 30.0f32);
        e.set_global(CLIP_ANGLE_LIMIT, 0.5f32);

        e.register(NI_POINTER_GET, |e, a| value(e.mem.u32(a[0])));
        for address in [
            NI_POINTER_ASSIGN,
            NI_POINTER_CONSTRUCT,
            QUEUED_FILE_POINTER_ASSIGN,
        ] {
            e.register(address, |e, a| {
                e.mem.set_u32(a[0], a[1]);
                value(a[0])
            });
        }
        noops(&mut e, &[NI_POINTER_RELEASE, VECTOR_CONSTRUCTOR]);
        returns_this(&mut e, &[EMPTY_CONSTRUCTOR]);
        e.register(POINT3_CONSTRUCT, |e, a| {
            for i in 0..3 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
            }
            value(a[0])
        });
        e.register(COLOR4_CONSTRUCT, |e, a| {
            for i in 0..4 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
            }
            value(a[0])
        });
        e.register(POINT2_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            value(a[0])
        });
        e.register(PLANE_CONSTRUCT, |e, a| {
            for i in 0..3 {
                let word = e.mem.u32(a[1] + 4 * i);
                e.mem.set_u32(a[0] + 4 * i, word);
            }
            e.mem.set_u32(a[0] + 12, a[2]);
            value(a[0])
        });
        e.register(POINT3_SUBTRACT, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) - e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            value(a[1])
        });
        e.register(POINT3_ADD, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) + e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            value(a[1])
        });
        e.register(POINT3_SCALE, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) * f32::from_bits(a[2]);
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            value(a[1])
        });
        e.register(POINT3_CROSS, |e, a| {
            let p: Vec<f32> = (0..3).map(|i| e.mem.f32(a[0] + 4 * i)).collect();
            let q: Vec<f32> = (0..3).map(|i| e.mem.f32(a[2] + 4 * i)).collect();
            e.mem.set_f32(a[1], p[1] * q[2] - p[2] * q[1]);
            e.mem.set_f32(a[1] + 4, p[2] * q[0] - p[0] * q[2]);
            e.mem.set_f32(a[1] + 8, p[0] * q[1] - p[1] * q[0]);
            value(a[1])
        });
        e.register(POINT3_DOT, |e, a| {
            let sum: f32 = (0..3)
                .map(|i| e.mem.f32(a[0] + 4 * i) * e.mem.f32(a[1] + 4 * i))
                .sum();
            float(sum as f64)
        });
        e.register(POINT3_LENGTH, |e, a| {
            let sum: f32 = (0..3)
                .map(|i| e.mem.f32(a[0] + 4 * i) * e.mem.f32(a[0] + 4 * i))
                .sum();
            float((sum as f64).sqrt())
        });
        e.register(PLANE_SIDE, |e, a| {
            let distance: f32 = (0..3)
                .map(|i| e.mem.f32(a[0] + 4 * i) * e.mem.f32(a[1] + 4 * i))
                .sum::<f32>()
                - e.mem.f32(a[0] + 12);
            value(if distance < 0.0 {
                2
            } else if distance > 0.0 {
                1
            } else {
                0
            })
        });
        e.register(PLANE_CONSTANT, |e, a| float(e.mem.f32(a[0] + 12) as f64));
        // The matrix helpers: row-major, `m * v` as the exe's bodies compute it.
        e.register(MATRIX_INVERSE, |e, a| {
            for i in 0..9 {
                let word = e.mem.u32(a[0] + 4 * i);
                e.mem.set_u32(a[1] + 4 * i, word);
            }
            value(a[1])
        });
        e.register(MATRIX_TRANSFORM_POINT, |e, a| {
            let m = matrix_of(e, a[0]);
            let v = [e.mem.f32(a[2]), e.mem.f32(a[2] + 4), e.mem.f32(a[2] + 8)];
            let r = multiply_vector(&m, v);
            for (i, x) in r.iter().enumerate() {
                e.mem.set_f32(a[1] + 4 * i as u32, *x);
            }
            value(a[1])
        });
        e.register(MATRIX_TRANSFORM_VERTICES, |e, a| {
            let m = matrix_of(e, a[0]);
            let t = [e.mem.f32(a[1]), e.mem.f32(a[1] + 4), e.mem.f32(a[1] + 8)];
            for n in 0..a[2] {
                let v = [
                    e.mem.f32(a[3] + 12 * n),
                    e.mem.f32(a[3] + 12 * n + 4),
                    e.mem.f32(a[3] + 12 * n + 8),
                ];
                let r = multiply_vector(&m, v);
                for k in 0..3 {
                    e.mem.set_f32(a[4] + 12 * n + 4 * k as u32, r[k] + t[k]);
                }
            }
            Ret::default()
        });
        e.register(MATRIX_MAKE_Z_ROTATION, |e, a| {
            let angle = f32::from_bits(a[1]);
            let (s, c) = angle.sin_cos();
            for (i, v) in [c, s, 0.0, -s, c, 0.0, 0.0, 0.0, 1.0].iter().enumerate() {
                e.mem.set_f32(a[0] + 4 * i as u32, *v);
            }
            Ret::default()
        });
        e.register(MATRIX_MULTIPLY, |e, a| {
            let m = matrix_of(e, a[0]);
            let n = matrix_of(e, a[2]);
            let mut r = [0.0f32; 9];
            for row in 0..3 {
                for col in 0..3 {
                    r[row * 3 + col] = m[row * 3 + 2] * n[6 + col]
                        + m[row * 3 + 1] * n[3 + col]
                        + m[row * 3] * n[col];
                }
            }
            for (i, v) in r.iter().enumerate() {
                e.mem.set_f32(a[1] + 4 * i as u32, *v);
            }
            value(a[1])
        });
        e.register(MATRIX_GET_ROW, |e, a| {
            for k in 0..3 {
                let word = e.mem.u32(a[0] + 12 * a[1] + 4 * k);
                e.mem.set_u32(a[2] + 4 * k, word);
            }
            Ret::default()
        });
        e.register(ROTATION_Z_TO_VECTOR, |e, a| {
            write_identity(e, a[0]);
            Ret::default()
        });
        e.register(ROTATION_Z_TO_VECTOR_TRANSPOSE, |e, a| {
            write_identity(e, a[0]);
            Ret::default()
        });
        // Node accessors: rotations at +0x34 and +0x68, translation +0x8C,
        // world scale +0x98 and parent +0x18.
        e.register(NODE_LOCAL_ROTATE, |_, a| value(a[0] + 0x34));
        e.register(NODE_WORLD_ROTATE, |_, a| value(a[0] + 0x68));
        e.register(NODE_WORLD_TRANSLATE, |_, a| value(a[0] + 0x8c));
        e.register(
            NODE_WORLD_SCALE,
            |e, a| float(e.mem.f32(a[0] + 0x98) as f64),
        );
        e.register(NODE_PARENT, |e, a| value(e.mem.u32(a[0] + 0x18)));
        // Memory.
        e.register(MEMORY_MANAGER_GETTER, |_, _| value(0x011f_6238));
        e.register(THREAD_SCRAP_HEAP, |_, _| value(0x5000_0000));
        e.register(SCRAP_ALLOCATE, |e, a| value(e.mem.alloc(a[1])));
        e.register(SCRAP_DEALLOCATE, |e, a| {
            e.mem.free(a[1]);
            Ret::default()
        });
        for address in [OPERATOR_NEW, RUNTIME_NEW, MEMORY_ALLOCATE] {
            e.register(address, |e, a| value(e.mem.alloc(a[0])));
        }
        for address in [MEMORY_FREE, RUNTIME_DELETE] {
            e.register(address, |e, a| {
                e.mem.free(a[0]);
                Ret::default()
            });
        }
        e.register(MEMORY_COPY, |e, a| {
            for i in 0..a[2] {
                let byte = e.mem.u8(a[1] + i);
                e.mem.set_u8(a[0] + i, byte);
            }
            value(a[0])
        });
        e.register(MEMORY_FILL, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            value(a[0])
        });
        // `BSSimpleArray`: vtable, buffer +4, size +8, capacity +0xC.
        e.register(ARRAY_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0] + 4, 0);
            e.mem.set_u32(a[0] + 8, 0);
            e.mem.set_u32(a[0] + 12, 0);
            value(a[0])
        });
        e.register(ARRAY_SIZE, |e, a| value(e.mem.u32(a[0] + 8)));
        e.register(ARRAY_ELEMENT, |e, a| value(e.mem.u32(a[0] + 4) + 4 * a[1]));
        e.register(ARRAY_APPEND, |e, a| {
            let size = e.mem.u32(a[0] + 8);
            let capacity = e.mem.u32(a[0] + 12);
            let mut buffer = e.mem.u32(a[0] + 4);
            if buffer == 0 || size >= capacity {
                let new_capacity = (capacity * 2).max(8);
                let new_buffer = e.mem.alloc(4 * new_capacity);
                for i in 0..size {
                    let word = e.mem.u32(buffer + 4 * i);
                    e.mem.set_u32(new_buffer + 4 * i, word);
                }
                if buffer != 0 {
                    e.mem.free(buffer);
                }
                buffer = new_buffer;
                e.mem.set_u32(a[0] + 4, buffer);
                e.mem.set_u32(a[0] + 12, new_capacity);
            }
            let item = e.mem.u32(a[1]);
            e.mem.set_u32(buffer + 4 * size, item);
            e.mem.set_u32(a[0] + 8, size + 1);
            value(size)
        });
        e.register(ARRAY_CLEAR, |e, a| {
            e.mem.set_u32(a[0] + 8, 0);
            if a[1] != 0 {
                let buffer = e.mem.u32(a[0] + 4);
                if buffer != 0 {
                    e.mem.free(buffer);
                }
                e.mem.set_u32(a[0] + 4, 0);
                e.mem.set_u32(a[0] + 12, 0);
            }
            Ret::default()
        });
        e.register(DECAL_VERTEX_DELETE, |e, a| {
            if a[1] & 1 != 0 {
                e.mem.free(a[0]);
            }
            Ret::default()
        });
        returns_this(&mut e, &[DECAL_VERTEX_CONSTRUCT]);
        // Diagnostics: the setting getter returns the address of a byte.
        e.register(SETTING_BYTE_GETTER, |_, a| value(a[0] + 4));
        noops(&mut e, &[PRINT_LINE]);
        e
    }

    fn new_decal(e: &mut Engine) -> Ptr<Decal> {
        e.new_object::<Decal>()
    }

    #[test]
    fn registers_every_function_of_the_unit() {
        let table = funcs();
        assert_eq!(table.len(), 31);
        let mut addresses: Vec<u32> = table.iter().map(|(a, _)| *a).collect();
        addresses.sort_unstable();
        addresses.dedup();
        assert_eq!(addresses.len(), 31);
    }

    #[test]
    fn get_rtti_returns_the_class_rtti() {
        let mut e = decal_engine();
        let decal = new_decal(&mut e);
        assert_eq!(e.call(0x0068_acb0, &args![decal]).u32(), RTTI);
    }

    #[test]
    fn get_3d_returns_the_pointer_in_sp3d() {
        let mut e = decal_engine();
        let decal = new_decal(&mut e);
        e.set(decal, Decal::sp3D, Ptr::new(0x1234));
        assert_eq!(e.call(0x0068_acc0, &args![decal]).u32(), 0x1234);
    }

    /// The destructor chain with nothing to detach: no shape, no arrays.
    fn destructor_engine() -> Engine {
        let mut e = decal_engine();
        noops(
            &mut e,
            &[BASE_DESTRUCT, REMOVE_FROM_PROPERTY_LIGHTS, SIZED_DELETE],
        );
        e
    }

    #[test]
    fn scalar_deleting_destructor_deletes_only_when_asked() {
        let mut e = destructor_engine();
        let decal = new_decal(&mut e);
        e.call_log = Some(vec![]);
        let result = e.call(0x0068_ace0, &args![decal, 0u32]);
        assert_eq!(result.u32(), decal.addr());
        assert!(calls_to(&e, SIZED_DELETE).is_empty());
        assert_eq!(calls_to(&e, BASE_DESTRUCT).len(), 1);

        let result = e.call(0x0068_ace0, &args![decal, 1u32]);
        assert_eq!(result.u32(), decal.addr());
        assert_eq!(
            calls_to(&e, SIZED_DELETE),
            vec![vec![decal.addr(), OBJECT_SIZE]]
        );
    }

    #[test]
    fn constructor_copies_the_creation_info() {
        let mut e = decal_engine();
        let decal = new_decal(&mut e);
        let info = e.mem.alloc(0x80);
        write_points(&mut e, info, &[[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
        e.mem.set_u32(info + info::TARGET_NODE, 0x2000);
        e.mem.set_u32(info + info::TEXTURE_SET, 0x3000);
        e.mem.set_f32(info + info::WIDTH, 10.0);
        e.mem.set_f32(info + info::HEIGHT, 20.0);
        e.mem.set_f32(info + info::DEPTH, 30.0);
        e.mem.set_f32(info + info::ROTATION, 0.5);
        e.mem.set_u32(info + info::CELL, 0x4000);
        e.mem.set_f32(info + info::PARALLAX_SCALE, 0.25);
        e.mem.set_f32(info + info::SPECULAR, 0.75);
        e.mem.set_f32(info + info::ANGLE_DEGREES, 60.0);
        write_points(&mut e, info + info::VERTEX_COLOR, &[[0.1, 0.2, 0.3]]);
        e.mem.set_u8(info + info::SUB_TEXTURE, 2);
        e.mem.set_u8(info + info::PERMANENT, 1);
        e.mem.set_u8(info + info::PARALLAX_OCCLUSION, 1);
        e.mem.set_u8(info + info::ALPHA_TESTING, 1);
        e.mem.set_u8(info + info::ALPHA_BLENDING, 1);
        e.mem.set_u8(info + info::PARALLAX_MAX_PASSES, 7);
        e.mem.set_u8(info + info::MODEL_SPACE, 1);
        e.mem.set_u8(info + info::FORCE_FADE, 1);
        e.mem.set_u8(info + info::TWO_SIDED, 1);
        // The base constructor (records its arguments), the setting getter
        // (a word holding 50) and the function at 004e4470 (cosine).
        noops(&mut e, &[BASE_CONSTRUCT]);
        let setting = e.mem.alloc(4);
        e.mem.set_u32(setting, 50);
        e.register_double(SETTING_VALUE_GETTER, move |_, _| value(setting));
        e.register(0x004e_4470, |_, a| {
            float((f32::from_bits(a[0]) as f64).cos())
        });
        e.set_global(MAX_DECAL_COUNT, -1i32);
        e.call_log = Some(vec![]);

        let result = e.call(0x0068_ad20, &args![decal, 0x5000u32, 7.5f32, info]);
        assert_eq!(result.u32(), decal.addr());
        assert_eq!(e.mem.u32(decal.addr()), VTABLE);
        assert_eq!(
            calls_to(&e, BASE_CONSTRUCT),
            vec![vec![decal.addr(), 0x4000, 7.5f32.to_bits()]]
        );
        assert_eq!(e.get(decal, Decal::pParentNode), Ptr::new(0x5000));
        assert_eq!(e.get(decal, Decal::spTargetNode), Ptr::new(0x2000));
        assert!(e.get(decal, Decal::sp3D).is_null());
        assert_eq!(e.get(decal, Decal::pTextureSet), Ptr::new(0x3000));
        let at = decal.addr();
        assert_eq!(
            points(&e, at + 0x34, 2),
            vec![[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]
        );
        assert_eq!(e.get(decal, Decal::fWidth), 10.0);
        assert_eq!(e.get(decal, Decal::fHeight), 20.0);
        assert_eq!(e.get(decal, Decal::fDepth), 30.0);
        assert_eq!(e.get(decal, Decal::fRotation), 0.5);
        assert_eq!(e.get(decal, Decal::cSubTexture), 2);
        assert!(e.get(decal, Decal::bPermanent));
        assert!(!e.get(decal, Decal::bValidDecal));
        assert!(e.get(decal, Decal::bForceFade));
        assert!(e.get(decal, Decal::bUseParallaxOcclusion));
        assert_eq!(e.get(decal, Decal::fParallaxOcclusionScale), 0.25);
        assert_eq!(e.get(decal, Decal::ucParallaxOcclusionMaxPasses), 7);
        assert!(e.get(decal, Decal::bUseAlphaTesting));
        assert!(e.get(decal, Decal::bUseAlphaBlending));
        assert_eq!(e.get(decal, Decal::fSpecular), 0.75);
        assert_eq!(e.get(decal, Decal::fVertexColor), 0.1);
        assert_eq!(e.get(decal, Decal::fVertexColorGreen), 0.2);
        assert_eq!(e.get(decal, Decal::fVertexColorBlue), 0.3);
        assert!(e.get(decal, Decal::bModelSpace));
        assert!(e.get(decal, Decal::bTwoSided));
        assert_eq!(e.get(decal, Decal::iDecalVertexCount), 0);
        assert_eq!(e.get(decal, Decal::iDecalTriangleCount), 0);
        assert_eq!(
            calls_to(&e, VECTOR_CONSTRUCTOR)[0],
            vec![at + 0x64, 0x10, 6, PLANE_ELEMENT_CONSTRUCTOR]
        );
        assert_eq!(e.mem.u32(at + 0xf8), DECAL_VERTEX_ARRAY_VTABLE);
        // The limit was -1, so it was read from the setting; the angle limit
        // is cos(60 degrees).
        assert_eq!(e.global::<i32>(MAX_DECAL_COUNT), 50);
        assert!((e.global::<f32>(CLIP_ANGLE_LIMIT) - 0.5).abs() < 1e-5);
        assert_eq!(calls_to(&e, SETTING_VALUE_GETTER).len(), 1);

        // With the limit set it is not read again, and a small angle limit
        // is raised to 0.01 (cos of 90 degrees is about 0).
        e.mem.set_f32(info + info::ANGLE_DEGREES, 90.0);
        let other = new_decal(&mut e);
        e.call(0x0068_ad20, &args![other, 0u32, 1.0f32, info]);
        assert_eq!(calls_to(&e, SETTING_VALUE_GETTER).len(), 1);
        assert_eq!(e.global::<f32>(CLIP_ANGLE_LIMIT), 0.01);
    }

    #[test]
    fn larger_of_two_keeps_the_float_on_ties_and_nan() {
        let mut e = decal_engine();
        assert_eq!(e.call(0x0068_b050, &args![0.01f64, 0.5f32]).f64(), 0.5);
        assert_eq!(e.call(0x0068_b050, &args![0.01f64, 0.005f32]).f64(), 0.01);
        assert_eq!(e.call(0x0068_b050, &args![0.25f64, 0.25f32]).f64(), 0.25);
        assert!(e
            .call(0x0068_b050, &args![0.01f64, f32::NAN])
            .f64()
            .is_nan());
    }

    #[test]
    fn destructor_detaches_a_non_permanent_shape() {
        let mut e = destructor_engine();
        let decal = new_decal(&mut e);
        let shape = e.mem.alloc(0x40);
        let parent = e.mem.alloc(0x40);
        e.mem.set_u32(shape + 0x18, parent);
        e.put_vtable(0x0110_0000, &[0u32; 59]);
        e.mem.set_u32(0x0110_0000 + 0xe8, 0x0060_e8e8);
        e.mem.set_u32(parent, 0x0110_0000);
        noops(&mut e, &[0x0060_e8e8]);
        e.set(decal, Decal::sp3D, Ptr::new(shape));
        e.set(decal, Decal::spTargetNode, Ptr::new(0x77));
        e.set(decal, Decal::pParentNode, Ptr::new(0x88));
        e.set(decal, Decal::bValidDecal, true);
        e.set_global(VALID_DECAL_COUNT, 5u32);
        e.call_log = Some(vec![]);

        e.call(0x0068_b080, &args![decal]);
        assert_eq!(e.mem.u32(decal.addr()), VTABLE);
        assert_eq!(calls_to(&e, REMOVE_FROM_PROPERTY_LIGHTS), vec![vec![shape]]);
        assert_eq!(calls_to(&e, 0x0060_e8e8), vec![vec![parent, shape]]);
        assert!(e.get(decal, Decal::sp3D).is_null());
        assert!(e.get(decal, Decal::spTargetNode).is_null());
        assert!(e.get(decal, Decal::pParentNode).is_null());
        assert_eq!(e.global::<u32>(VALID_DECAL_COUNT), 4);
        assert_eq!(calls_to(&e, BASE_DESTRUCT), vec![vec![decal.addr()]]);
        // Three members are released: target node, shape and query texture.
        assert_eq!(calls_to(&e, NI_POINTER_RELEASE).len(), 3);
    }

    #[test]
    fn destructor_leaves_the_shape_of_a_permanent_decal() {
        let mut e = destructor_engine();
        let decal = new_decal(&mut e);
        e.set(decal, Decal::sp3D, Ptr::new(0x9900));
        e.set(decal, Decal::bPermanent, true);
        e.set_global(VALID_DECAL_COUNT, 5u32);
        e.call_log = Some(vec![]);
        e.call(0x0068_b080, &args![decal]);
        assert!(calls_to(&e, REMOVE_FROM_PROPERTY_LIGHTS).is_empty());
        assert_eq!(e.get(decal, Decal::sp3D), Ptr::new(0x9900));
        assert_eq!(e.global::<u32>(VALID_DECAL_COUNT), 5);
        assert_eq!(calls_to(&e, BASE_DESTRUCT).len(), 1);
    }

    #[test]
    fn clean_up_memory_returns_temporary_arrays_of_an_invalid_decal() {
        let mut e = decal_engine();
        let decal = new_decal(&mut e);
        let arrays: Vec<u32> = (0..5).map(|_| e.mem.alloc(16)).collect();
        let map = e.mem.alloc(16);
        for (field, array) in [
            Decal::pTempVertexArray,
            Decal::pTempNormalArray,
            Decal::pTempTexcoordArray,
            Decal::pTempColorArray,
            Decal::pTempIndexArray,
        ]
        .iter()
        .zip(&arrays)
        {
            e.set(decal, *field, Ptr::new(*array));
        }
        e.set(decal, Decal::pOldToNewIndexArray, Ptr::new(map));
        // Two decal vertices in the index map.
        let table = decal.addr() + 0xf8;
        fn_0068eb00(&mut e, Ptr::new(table));
        let first = e.mem.alloc(16);
        let second = e.mem.alloc(16);
        for vertex in [first, second] {
            let holder = e.mem.alloc(4);
            e.mem.set_u32(holder, vertex);
            e.call(ARRAY_APPEND, &args![table, holder]);
        }
        e.call_log = Some(vec![]);

        e.call(0x0068_b220, &args![decal]);
        let released: Vec<u32> = calls_to(&e, SCRAP_DEALLOCATE)
            .iter()
            .map(|a| a[1])
            .collect();
        assert_eq!(released, arrays);
        assert_eq!(calls_to(&e, MEMORY_FREE), vec![vec![map]]);
        for field in [
            Decal::pTempVertexArray,
            Decal::pTempNormalArray,
            Decal::pTempTexcoordArray,
            Decal::pTempColorArray,
            Decal::pTempIndexArray,
            Decal::pOldToNewIndexArray,
        ] {
            assert!(e.get(decal, field).is_null());
        }
        let deleted: Vec<Vec<u32>> = calls_to(&e, DECAL_VERTEX_DELETE);
        assert_eq!(deleted, vec![vec![first, 1], vec![second, 1]]);
        assert_eq!(calls_to(&e, ARRAY_CLEAR), vec![vec![table, 1]]);
    }

    #[test]
    fn clean_up_memory_keeps_the_arrays_of_a_valid_decal() {
        let mut e = decal_engine();
        let decal = new_decal(&mut e);
        let array = e.mem.alloc(16);
        e.set(decal, Decal::pTempVertexArray, Ptr::new(array));
        e.set(decal, Decal::bValidDecal, true);
        fn_0068eb00(&mut e, Ptr::new(decal.addr() + 0xf8));
        e.call_log = Some(vec![]);
        e.call(0x0068_b220, &args![decal]);
        assert!(calls_to(&e, SCRAP_DEALLOCATE).is_empty());
        assert_eq!(e.get(decal, Decal::pTempVertexArray), Ptr::new(array));
        assert_eq!(calls_to(&e, ARRAY_CLEAR).len(), 1);
    }

    #[test]
    fn preloading_builds_the_queued_file_once() {
        let mut e = decal_engine();
        e.register(0x0040_4eb0, |_, _| Ret::default());
        e.register(0x0040_4ee0, |_, _| Ret::default());
        e.register(QUEUED_FILE_CONSTRUCT, |_, a| value(a[0]));
        e.call_log = Some(vec![]);
        e.call(0x0068_b3e0, &args![]);
        let made = e.mem.u32(PRELOADED_TEXTURE_SETS);
        assert_ne!(made, 0);
        assert_eq!(calls_to(&e, QUEUED_FILE_CONSTRUCT), vec![vec![made, 5]]);
        let guards = calls_to(&e, SCOPE_GUARD_CONSTRUCT);
        assert_eq!(guards.len(), 1);
        assert_eq!(&guards[0][1..], &[7, 1, SOURCE_FILE_NAME, 0x11f]);
        assert_eq!(calls_to(&e, SCOPE_GUARD_DESTRUCT).len(), 1);

        // A second call finds it set and does nothing.
        e.call(0x0068_b3e0, &args![]);
        assert_eq!(calls_to(&e, QUEUED_FILE_CONSTRUCT).len(), 1);
        assert_eq!(e.mem.u32(PRELOADED_TEXTURE_SETS), made);
    }

    #[test]
    fn releasing_the_preloaded_sets_clears_the_static() {
        let mut e = decal_engine();
        e.mem.set_u32(PRELOADED_TEXTURE_SETS, 0x1234);
        e.call(0x0068_b4a0, &args![]);
        assert_eq!(e.mem.u32(PRELOADED_TEXTURE_SETS), 0);
    }

    /// A decal over a geometry with one strip triangle (0,0,0), (1,0,0),
    /// (0,1,0), all normals +Z, identity rotations and world scale 1; the
    /// decal is 4 x 4 x 2 at the origin looking along +Z, so the whole
    /// triangle is inside every plane.
    fn processing_scene() -> (Engine, Ptr<Decal>, u32) {
        let mut e = decal_engine();
        let geometry = e.mem.alloc(0x100);
        write_identity(&mut e, geometry + 0x34);
        write_identity(&mut e, geometry + 0x68);
        e.mem.set_f32(geometry + 0x98, 1.0);
        let vertices = e.mem.alloc(36);
        write_points(
            &mut e,
            vertices,
            &[[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
        );
        let normals = e.mem.alloc(36);
        write_points(&mut e, normals, &[[0.0, 0.0, 1.0]; 3]);
        let strip_data = e.mem.alloc(0x60);
        let strip = e.mem.alloc(8);
        for (i, index) in [0u16, 1, 2].iter().enumerate() {
            e.mem.set_u16(strip + 2 * i as u32, *index);
        }
        e.mem.set_u32(strip_data + 0x4c, strip);
        e.mem.set_u32(geometry + 0xe0, vertices);
        e.mem.set_u32(geometry + 0xe4, normals);
        e.mem.set_u32(geometry + 0xe8, strip_data);
        e.mem.set_u16(geometry + 0xec, 1);
        e.register(GEOMETRY_VERTICES, |e, a| value(e.mem.u32(a[0] + 0xe0)));
        e.register(GEOMETRY_NORMALS, |e, a| value(e.mem.u32(a[0] + 0xe4)));
        e.register(GEOMETRY_STRIP_DATA, |e, a| value(e.mem.u32(a[0] + 0xe8)));
        e.register(GEOMETRY_TRIANGLE_COUNT, |e, a| {
            value(e.mem.u16(a[0] + 0xec) as u32)
        });
        e.register(STRIP_INDICES, |e, a| value(e.mem.u32(a[0] + 0x4c)));
        e.register(IS_KIND_OF, |_, a| {
            value((a[0] == GEOMETRY_RTTI && a[1] != 0) as u32)
        });
        e.register(GEOMETRY_NAME_HOLDER, |_, a| value(a[0] + 8));
        e.register(NAME_TEXT, |_, _| value(0x0777_0000));
        returns_this(&mut e, &[TRI_SHAPE_CONSTRUCT, TRI_DATA_CONSTRUCT]);
        e.set_global(CLIP_ANGLE_LIMIT, 0.5f32);

        let decal = new_decal(&mut e);
        write_points(
            &mut e,
            decal.addr() + 0x34,
            &[[0.0, 0.0, 0.0], [0.0, 0.0, 1.0]],
        );
        e.set(decal, Decal::fWidth, 4.0);
        e.set(decal, Decal::fHeight, 4.0);
        e.set(decal, Decal::fDepth, 2.0);
        e.set(decal, Decal::spTargetNode, Ptr::new(geometry));
        e.set(decal, Decal::fVertexColor, 1.0);
        e.set(decal, Decal::fVertexColorGreen, 0.5);
        e.set(decal, Decal::fVertexColorBlue, 0.25);
        (e, decal, geometry)
    }

    type HandedArrays = (
        u32,
        Vec<[f32; 3]>,
        Vec<[f32; 3]>,
        Vec<[f32; 4]>,
        Vec<[f32; 2]>,
        u32,
        Vec<u16>,
    );

    /// The final arrays `ProcessGeometry` handed to the shape data
    /// constructor: (vertex count, vertices, normals, colours, texture
    /// coordinates, triangle count, indices).
    fn handed_to_the_shape(e: &Engine) -> HandedArrays {
        let call = &calls_to(e, TRI_DATA_CONSTRUCT)[0];
        let count = call[1];
        let colors = (0..count)
            .map(|i| {
                let at = call[4] + 16 * i;
                [
                    e.mem.f32(at),
                    e.mem.f32(at + 4),
                    e.mem.f32(at + 8),
                    e.mem.f32(at + 12),
                ]
            })
            .collect();
        let texcoords = (0..count)
            .map(|i| [e.mem.f32(call[5] + 8 * i), e.mem.f32(call[5] + 8 * i + 4)])
            .collect();
        let indices = (0..call[8] * 3)
            .map(|i| e.mem.u16(call[9] + 2 * i))
            .collect();
        (
            count,
            points(e, call[2], count),
            points(e, call[3], count),
            colors,
            texcoords,
            call[8],
            indices,
        )
    }

    #[test]
    fn process_geometry_builds_the_planes_and_the_shape() {
        let (mut e, decal, geometry) = processing_scene();
        e.call_log = Some(vec![]);
        assert!(e.call(0x0068_b4c0, &args![decal, geometry]).bool());
        assert!(e.get(decal, Decal::bFinalize));
        assert!(!e.get(decal, Decal::bFinished));
        let at = decal.addr();
        assert_eq!(points(&e, at + 0x4c, 1), vec![[0.0, 0.0, 0.0]]);
        assert_eq!(points(&e, at + 0x58, 1), vec![[0.0, 0.0, 1.0]]);
        // Left, right, bottom, top, front and back planes: normal and constant.
        let expected = [
            ([1.0, 0.0, 0.0], -2.0),
            ([-1.0, 0.0, 0.0], -2.0),
            ([0.0, 1.0, 0.0], -2.0),
            ([0.0, -1.0, 0.0], -2.0),
            ([0.0, 0.0, -1.0], -2.0),
            ([0.0, 0.0, 1.0], -2.0),
        ];
        for (i, (normal, constant)) in expected.iter().enumerate() {
            let plane = at + 0x64 + 0x10 * i as u32;
            assert_eq!(points(&e, plane, 1)[0], *normal, "plane {i}");
            assert_eq!(e.mem.f32(plane + 12), *constant, "plane {i}");
        }
        // The shape was created from the clipped triangle.
        let (count, vertices, normals, colors, texcoords, triangles, indices) =
            handed_to_the_shape(&e);
        assert_eq!(count, 3);
        assert_eq!(
            vertices,
            vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]]
        );
        assert_eq!(normals, vec![[0.0, 0.0, 1.0]; 3]);
        // alpha = (dot / length - limit) / (1 - limit) = 1 for a facing normal.
        assert_eq!(colors, vec![[1.0, 0.5, 0.25, 1.0]; 3]);
        assert_eq!(texcoords, vec![[0.25, 0.25], [0.375, 0.25], [0.25, 0.375]]);
        assert_eq!(triangles, 1);
        assert_eq!(indices, vec![0, 1, 2]);
        let data_call = &calls_to(&e, TRI_DATA_CONSTRUCT)[0];
        assert_eq!(&data_call[6..8], &[1, 0]);
        assert!(!e.get(decal, Decal::sp3D).is_null());
        // The temporary arrays are gone and the final ones belong to the shape.
        assert!(e.get(decal, Decal::pTempVertexArray).is_null());
        assert!(e.get(decal, Decal::pFinalVertexArray).is_null());
        assert_eq!(e.get(decal, Decal::iDecalVertexCount), 3);
        assert_eq!(e.get(decal, Decal::iDecalTriangleCount), 1);
    }

    #[test]
    fn process_geometry_needs_a_geometry_of_the_right_class() {
        let (mut e, decal, geometry) = processing_scene();
        assert!(!e.call(0x0068_b4c0, &args![decal, 0u32]).bool());
        assert!(e.get(decal, Decal::bFinished));

        e.set(decal, Decal::bFinished, false);
        e.register(IS_KIND_OF, |_, _| value(0));
        assert!(!e.call(0x0068_b4c0, &args![decal, geometry]).bool());
        assert!(e.get(decal, Decal::bFinished));
        assert!(!e.get(decal, Decal::bFinalize));
    }

    #[test]
    fn process_geometry_with_nothing_left_reports_and_finishes() {
        let (mut e, decal, geometry) = processing_scene();
        // The decal looks along -Z: the +Z triangle faces away.
        write_points(&mut e, decal.addr() + 0x40, &[[0.0, 0.0, -1.0]]);
        e.mem.set_u8(DIAGNOSTICS_SETTING + 4, 1);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0068_b4c0, &args![decal, geometry]).bool());
        assert!(e.get(decal, Decal::bFinished));
        assert!(!e.get(decal, Decal::bFinalize));
        assert_eq!(e.get(decal, Decal::iDecalTriangleCount), 0);
        assert_eq!(
            calls_to(&e, PRINT_LINE),
            vec![vec![NO_DECAL_MESSAGE, 0x0777_0000]]
        );
        assert!(e.get(decal, Decal::sp3D).is_null());

        // Without the diagnostics setting nothing is printed.
        let (mut e, decal, geometry) = processing_scene();
        write_points(&mut e, decal.addr() + 0x40, &[[0.0, 0.0, -1.0]]);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0068_b4c0, &args![decal, geometry]).bool());
        assert!(calls_to(&e, PRINT_LINE).is_empty());
    }

    #[test]
    fn process_geometry_texture_coordinates_follow_the_sub_texture() {
        // (sub texture, mirrored, expected u and v of the second vertex)
        let cases = [
            (0u8, false, [0.375f32, 0.25f32]),
            (1, false, [0.875, 0.25]),
            (2, false, [0.375, 0.75]),
            (3, false, [0.875, 0.75]),
            (4, false, [0.75, 0.5]),
            (0, true, [0.625, 0.25]),
        ];
        for (sub_texture, mirrored, expected) in cases {
            let (mut e, decal, geometry) = processing_scene();
            e.set(decal, Decal::cSubTexture, sub_texture);
            e.set(decal, Decal::bPermanent, mirrored);
            e.call_log = Some(vec![]);
            assert!(e.call(0x0068_b4c0, &args![decal, geometry]).bool());
            let (_, _, _, _, texcoords, _, _) = handed_to_the_shape(&e);
            assert_eq!(
                texcoords[1], expected,
                "sub texture {sub_texture} mirrored {mirrored}"
            );
        }
    }

    #[test]
    fn process_geometry_in_model_space_uses_the_local_rotation() {
        let (mut e, decal, geometry) = processing_scene();
        e.set(decal, Decal::bModelSpace, true);
        // Turn the geometry's local rotation 90 degrees about Z; the world
        // rotation stays the identity and must not be used.
        let turned = [0.0f32, 1.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 1.0];
        for (i, v) in turned.iter().enumerate() {
            e.mem.set_f32(geometry + 0x34 + 4 * i as u32, *v);
        }
        write_points(
            &mut e,
            decal.addr() + 0x34,
            &[[2.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
        );
        e.call(0x0068_b4c0, &args![decal, geometry]);
        // normal = inverse * vector, with the (copied) matrix as the inverse
        // in this double: (1,0,0) -> row 0 dot (1,0,0) = (0, -1, 0).
        assert_eq!(points(&e, decal.addr() + 0x58, 1), vec![[0.0, -1.0, 0.0]]);
        // The model-space origin (2,0,0) is used unscaled and untranslated:
        // centre = matrix * origin.
        assert_eq!(points(&e, decal.addr() + 0x4c, 1), vec![[0.0, -2.0, 0.0]]);
    }

    #[test]
    fn process_geometry_divides_the_size_by_the_world_scale() {
        let (mut e, decal, geometry) = processing_scene();
        e.mem.set_f32(geometry + 0x98, 2.0);
        e.set(decal, Decal::fWidth, 8.0);
        e.set(decal, Decal::fHeight, 8.0);
        e.set(decal, Decal::fDepth, 4.0);
        e.call(0x0068_b4c0, &args![decal, geometry]);
        assert_eq!(e.get(decal, Decal::fWidth), 4.0);
        assert_eq!(e.get(decal, Decal::fHeight), 4.0);
        assert_eq!(e.get(decal, Decal::fDepth), 2.0);
    }

    #[test]
    fn initialize_runs_the_base_initialize_after_processing() {
        let (mut e, decal, _geometry) = processing_scene();
        noops(&mut e, &[BASE_INITIALIZE]);
        e.call_log = Some(vec![]);
        e.call(0x0068_b1e0, &args![decal]);
        assert_eq!(calls_to(&e, BASE_INITIALIZE), vec![vec![decal.addr()]]);

        // Without a target there is nothing to process.
        let (mut e, decal, _geometry) = processing_scene();
        noops(&mut e, &[BASE_INITIALIZE]);
        e.set(decal, Decal::spTargetNode, Ptr::NULL);
        e.call_log = Some(vec![]);
        e.call(0x0068_b1e0, &args![decal]);
        assert!(calls_to(&e, BASE_INITIALIZE).is_empty());
        assert!(e.get(decal, Decal::bFinished));
    }

    #[test]
    fn create_tri_shape_constructs_from_the_data() {
        let mut e = decal_engine();
        returns_this(&mut e, &[TRI_SHAPE_CONSTRUCT]);
        e.call_log = Some(vec![]);
        let shape = e.call(0x0068_ca10, &args![0x4321u32]).u32();
        assert_ne!(shape, 0);
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![0xc4]]);
        assert_eq!(calls_to(&e, TRI_SHAPE_CONSTRUCT), vec![vec![shape, 0x4321]]);

        // A failed allocation gives null and no construction.
        e.register(OPERATOR_NEW, |_, _| value(0));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0068_ca10, &args![0x4321u32]).u32(), 0);
        assert!(calls_to(&e, TRI_SHAPE_CONSTRUCT).is_empty());
    }

    #[test]
    fn create_tri_data_passes_the_final_arrays() {
        let mut e = decal_engine();
        returns_this(&mut e, &[TRI_DATA_CONSTRUCT]);
        let decal = new_decal(&mut e);
        e.set(decal, Decal::iDecalVertexCount, 0x1_0005);
        e.set(decal, Decal::iDecalTriangleCount, 7);
        e.set(decal, Decal::pFinalVertexArray, Ptr::new(0x11));
        e.set(decal, Decal::pFinalNormalArray, Ptr::new(0x22));
        e.set(decal, Decal::pFinalColorArray, Ptr::new(0x33));
        e.set(decal, Decal::pFinalTexcoordArray, Ptr::new(0x44));
        e.set(decal, Decal::pFinalIndexArray, Ptr::new(0x55));
        e.call_log = Some(vec![]);
        let data = e.call(0x0068_ca90, &args![decal]).u32();
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![0x58]]);
        // Vertex count (low 16 bits), vertices, normals, colours, texture
        // coordinates, 1, 0, triangle count, indices.
        assert_eq!(
            calls_to(&e, TRI_DATA_CONSTRUCT),
            vec![vec![data, 5, 0x11, 0x22, 0x33, 0x44, 1, 0, 7, 0x55]]
        );
        e.register(OPERATOR_NEW, |_, _| value(0));
        assert_eq!(e.call(0x0068_ca90, &args![decal]).u32(), 0);
    }

    #[test]
    fn small_helpers_read_and_write_their_fields() {
        let mut e = decal_engine();
        let object = e.mem.alloc(0x100);
        e.mem.set_u16(object + 0x18, 0b0110);
        assert!(!e.call(0x0068_c870, &args![object]).bool());
        assert!(e.call(0x0068_c890, &args![object, 2u16]).bool());
        assert!(e.call(0x0068_c890, &args![object, 6u16]).bool());
        assert!(!e.call(0x0068_c890, &args![object, 1u16]).bool());
        e.mem.set_u16(object + 0x18, 0b0111);
        assert!(e.call(0x0068_c870, &args![object]).bool());
        // Only the low 16 bits of the mask count.
        assert!(!e.call(0x0068_c890, &args![object, 0x8000u16]).bool());

        e.call(0x0068_c8b0, &args![object, 3.5f32]);
        assert_eq!(e.mem.f32(object + 0xe8), 3.5);

        e.set_global(PERMANENT_NAME_WORD, 0xabcd_u32);
        assert_eq!(e.call(0x0068_c8d0, &args![]).u32(), 0xabcd);

        e.register(MARK_CHANGED, |e, a| {
            let count = e.mem.u32(a[0] + 0x44) + a[1];
            e.mem.set_u32(a[0] + 0x44, count);
            Ret::default()
        });
        e.call(0x0068_c9f0, &args![object, 0.25f32]);
        assert_eq!(e.mem.f32(object + 0x3c), 0.25);
        assert_eq!(e.mem.u32(object + 0x44), 1);
    }

    /// An expired decal with a shape whose material property holds alpha
    /// 1.0, age 10.25 and lifetime 10.
    fn update_scene(age: f32) -> (Engine, Ptr<Decal>, u32, u32) {
        let mut e = decal_engine();
        e.map(0x0110_6000, 0x1000);
        e.register(BASE_UPDATE, |e, _| value(e.mem.u32(0x0110_6000)));
        e.register(EFFECT_AGE, |e, a| float(e.mem.f32(a[0] + 0x10) as f64));
        e.register(EFFECT_LIFETIME, |e, a| float(e.mem.f32(a[0] + 8) as f64));
        e.register(MATERIAL_PROPERTY_TYPE, |_, _| value(2));
        e.register(NODE_GET_PROPERTY, |e, a| value(e.mem.u32(a[0] + 0x30)));
        e.register(MATERIAL_GET_ALPHA, |e, a| {
            float(e.mem.f32(a[0] + 0x3c) as f64)
        });
        noops(&mut e, &[MARK_CHANGED, PARENT_REMOVE_DECAL]);
        let decal = new_decal(&mut e);
        e.mem.set_f32(decal.addr() + 8, 10.0);
        e.mem.set_f32(decal.addr() + 0x10, age);
        let shape = e.mem.alloc(0x80);
        let parent = e.mem.alloc(0x40);
        let property = e.mem.alloc(0x80);
        e.mem.set_f32(property + 0x3c, 1.0);
        e.mem.set_u32(shape + 0x18, parent);
        e.mem.set_u32(shape + 0x30, property);
        e.set(decal, Decal::sp3D, Ptr::new(shape));
        e.call_log = Some(vec![]);
        (e, decal, property, parent)
    }

    #[test]
    fn update_returns_true_while_the_effect_is_alive() {
        let (mut e, decal, property, _) = update_scene(5.0);
        e.mem.set_u32(0x0110_6000, 1);
        assert!(e.call(0x0068_c8e0, &args![decal, 0.016f32]).bool());
        assert_eq!(e.mem.f32(property + 0x3c), 1.0);
        assert!(calls_to(&e, PARENT_REMOVE_DECAL).is_empty());
    }

    #[test]
    fn update_fades_the_material_after_expiry() {
        let (mut e, decal, property, _) = update_scene(10.25);
        assert!(e.call(0x0068_c8e0, &args![decal, 0.016f32]).bool());
        assert_eq!(e.mem.f32(property + 0x3c), 0.75);
        assert_eq!(calls_to(&e, MARK_CHANGED), vec![vec![property, 1]]);

        // The alpha already matches: nothing is written.
        e.call_log = Some(vec![]);
        assert!(e.call(0x0068_c8e0, &args![decal, 0.016f32]).bool());
        assert!(calls_to(&e, MARK_CHANGED).is_empty());
    }

    #[test]
    fn update_removes_the_decal_after_the_fade() {
        let (mut e, decal, property, parent) = update_scene(11.5);
        assert!(!e.call(0x0068_c8e0, &args![decal, 0.016f32]).bool());
        assert_eq!(e.mem.f32(property + 0x3c), 1.0);
        assert_eq!(
            calls_to(&e, PARENT_REMOVE_DECAL),
            vec![vec![parent, decal.addr()]]
        );

        // Exactly one second after expiry is still inside the fade.
        let (mut e, decal, property, _) = update_scene(11.0);
        assert!(e.call(0x0068_c8e0, &args![decal, 0.016f32]).bool());
        assert_eq!(e.mem.f32(property + 0x3c), 0.0);

        // A shape without parent is only reported as expired.
        let (mut e, decal, _, _) = update_scene(12.0);
        let shape = e.get(decal, Decal::sp3D).addr();
        e.mem.set_u32(shape + 0x18, 0);
        assert!(!e.call(0x0068_c8e0, &args![decal, 0.016f32]).bool());
        assert!(calls_to(&e, PARENT_REMOVE_DECAL).is_empty());
    }

    #[test]
    fn update_without_a_shape_returns_the_base_result() {
        let (mut e, decal, _, _) = update_scene(12.0);
        e.set(decal, Decal::sp3D, Ptr::NULL);
        assert!(!e.call(0x0068_c8e0, &args![decal, 0.016f32]).bool());
        assert!(calls_to(&e, PARENT_REMOVE_DECAL).is_empty());
    }

    #[test]
    fn occlusion_query_check_destroys_the_answered_query() {
        let mut e = decal_engine();
        e.register(OCCLUSION_QUERY_RESULT, |e, a| {
            e.mem.set_u32(a[1], 77);
            value(a[2] + 1)
        });
        noops(&mut e, &[OCCLUSION_QUERY_DELETE]);
        let decal = new_decal(&mut e);
        e.set(decal, Decal::pOcclusionQuery, Ptr::new(0x4400));
        e.call_log = Some(vec![]);
        // The double answers when the last argument is 0, as it is called.
        assert!(e.call(0x0068_de30, &args![decal]).bool());
        assert_eq!(
            calls_to(&e, OCCLUSION_QUERY_RESULT),
            vec![vec![0x4400, decal.addr() + 0x24, 0]]
        );
        assert_eq!(calls_to(&e, OCCLUSION_QUERY_DELETE), vec![vec![0x4400, 1]]);
        assert_eq!(e.get(decal, Decal::iCurrentOcclusionResult), 77);
        assert!(e.get(decal, Decal::pOcclusionQuery).is_null());

        // A query that has no answer yet is kept.
        e.register(OCCLUSION_QUERY_RESULT, |_, _| value(0));
        e.set(decal, Decal::pOcclusionQuery, Ptr::new(0x4400));
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0068_de30, &args![decal]).bool());
        assert!(calls_to(&e, OCCLUSION_QUERY_DELETE).is_empty());
        assert_eq!(e.get(decal, Decal::pOcclusionQuery), Ptr::new(0x4400));
    }

    #[test]
    fn property_list_clear_is_called_on_the_member_at_0x24() {
        let mut e = decal_engine();
        noops(&mut e, &[PROPERTY_LIST_CLEAR]);
        e.call_log = Some(vec![]);
        e.call(0x0068_e740, &args![0x1000u32]);
        assert_eq!(calls_to(&e, PROPERTY_LIST_CLEAR), vec![vec![0x1024]]);
    }

    #[test]
    fn vertex_array_constructor_and_destructors() {
        let mut e = decal_engine();
        let array = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0068_eb00, &args![array]).u32(), array);
        assert_eq!(e.mem.u32(array), DECAL_VERTEX_ARRAY_VTABLE);
        assert_eq!(calls_to(&e, ARRAY_CONSTRUCT), vec![vec![array, 0, 0]]);

        e.mem.set_u32(array, 0);
        e.call(0x0068_eb30, &args![array]);
        assert_eq!(e.mem.u32(array), DECAL_VERTEX_ARRAY_VTABLE);
        assert_eq!(calls_to(&e, ARRAY_CLEAR), vec![vec![array, 1]]);

        // The scalar deleting destructor deletes only when asked.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0068_eb50, &args![array, 0u32]).u32(), array);
        assert!(calls_to(&e, RUNTIME_DELETE).is_empty());
        assert_eq!(calls_to(&e, ARRAY_CLEAR).len(), 1);
        assert_eq!(e.call(0x0068_eb50, &args![array, 1u32]).u32(), array);
        assert_eq!(calls_to(&e, RUNTIME_DELETE), vec![vec![array]]);
    }

    #[test]
    fn resize_moves_the_arrays_to_exact_blocks() {
        let mut e = decal_engine();
        let decal = new_decal(&mut e);
        e.set(decal, Decal::iDecalVertexCount, 2);
        e.set(decal, Decal::iDecalTriangleCount, 1);
        let temp_vertices = e.mem.alloc(24);
        write_points(&mut e, temp_vertices, &[[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
        let temp_normals = e.mem.alloc(24);
        write_points(&mut e, temp_normals, &[[0.0, 0.0, 1.0], [0.0, 1.0, 0.0]]);
        let temp_texcoords = e.mem.alloc(16);
        e.mem.set_f32(temp_texcoords + 12, 0.75);
        let temp_colors = e.mem.alloc(32);
        e.mem.set_f32(temp_colors + 28, 0.5);
        let temp_indices = e.mem.alloc(16);
        for (i, v) in [7u16, 8, 9].iter().enumerate() {
            e.mem.set_u16(temp_indices + 2 * i as u32, *v);
        }
        e.set(decal, Decal::pTempVertexArray, Ptr::new(temp_vertices));
        e.set(decal, Decal::pTempNormalArray, Ptr::new(temp_normals));
        e.set(decal, Decal::pTempTexcoordArray, Ptr::new(temp_texcoords));
        e.set(decal, Decal::pTempColorArray, Ptr::new(temp_colors));
        e.set(decal, Decal::pTempIndexArray, Ptr::new(temp_indices));
        e.call_log = Some(vec![]);

        e.call(0x0068_e760, &args![decal]);
        let vertices = e.get(decal, Decal::pFinalVertexArray).addr();
        assert_eq!(
            points(&e, vertices, 2),
            vec![[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]
        );
        let normals = e.get(decal, Decal::pFinalNormalArray).addr();
        assert_eq!(points(&e, normals, 2)[1], [0.0, 1.0, 0.0]);
        assert_eq!(
            e.mem
                .f32(e.get(decal, Decal::pFinalTexcoordArray).addr() + 12),
            0.75
        );
        assert_eq!(
            e.mem.f32(e.get(decal, Decal::pFinalColorArray).addr() + 28),
            0.5
        );
        let indices = e.get(decal, Decal::pFinalIndexArray).addr();
        assert_eq!(
            (0..3)
                .map(|i| e.mem.u16(indices + 2 * i))
                .collect::<Vec<_>>(),
            vec![7, 8, 9]
        );
        for field in [
            Decal::pTempVertexArray,
            Decal::pTempNormalArray,
            Decal::pTempTexcoordArray,
            Decal::pTempColorArray,
            Decal::pTempIndexArray,
        ] {
            assert!(e.get(decal, field).is_null());
        }
        // Element sizes and constructors: 12, 12, 8 and 16 bytes (the colour
        // array with its own constructor).
        let constructed: Vec<Vec<u32>> = calls_to(&e, VECTOR_CONSTRUCTOR);
        assert_eq!(constructed.len(), 4);
        assert_eq!(&constructed[0][1..], &[12, 2, EMPTY_CONSTRUCTOR]);
        assert_eq!(&constructed[2][1..], &[8, 2, EMPTY_CONSTRUCTOR]);
        assert_eq!(&constructed[3][1..], &[16, 2, COLOR_ELEMENT_CONSTRUCTOR]);
        assert_eq!(calls_to(&e, SCRAP_DEALLOCATE).len(), 5);
        let sizes: Vec<u32> = calls_to(&e, MEMORY_COPY).iter().map(|a| a[2]).collect();
        assert_eq!(sizes, vec![24, 24, 16, 32, 6]);
    }

    /// The six planes of a 4 x 4 x 2 box around the origin looking along
    /// +Z (what `ProcessGeometry` builds for the scene above).
    fn set_box_planes(e: &mut Engine, decal: Ptr<Decal>) {
        let planes = [
            ([1.0f32, 0.0, 0.0], -2.0f32),
            ([-1.0, 0.0, 0.0], -2.0),
            ([0.0, 1.0, 0.0], -2.0),
            ([0.0, -1.0, 0.0], -2.0),
            ([0.0, 0.0, -1.0], -2.0),
            ([0.0, 0.0, 1.0], -2.0),
        ];
        for (i, (normal, constant)) in planes.iter().enumerate() {
            let at = decal.addr() + 0x64 + 0x10 * i as u32;
            write_points(e, at, &[*normal]);
            e.mem.set_f32(at + 12, *constant);
        }
        write_points(e, decal.addr() + 0x58, &[[0.0, 0.0, 1.0]]);
    }

    fn polygon(e: &mut Engine, indices: &[u32], vertices: &[[f32; 3]]) -> (u32, u32, u32) {
        let index_block = e.mem.alloc(4 * indices.len() as u32 + 4);
        for (i, index) in indices.iter().enumerate() {
            e.mem.set_u32(index_block + 4 * i as u32, *index);
        }
        let vertex_block = e.mem.alloc(12 * vertices.len() as u32 + 12);
        write_points(e, vertex_block, vertices);
        let normal_block = e.mem.alloc(12 * vertices.len() as u32 + 12);
        write_points(e, normal_block, &vec![[0.0, 0.0, 1.0]; vertices.len()]);
        (index_block, vertex_block, normal_block)
    }

    fn near(actual: [f32; 3], expected: [f32; 3]) -> bool {
        actual
            .iter()
            .zip(expected.iter())
            .all(|(a, b)| (a - b).abs() < 1e-6)
    }

    #[test]
    fn clip_against_a_plane_adds_the_crossing_points() {
        let mut e = decal_engine();
        let plane = e.mem.alloc(16);
        write_points(&mut e, plane, &[[1.0, 0.0, 0.0]]);
        e.mem.set_f32(plane + 12, 0.0);
        let (indices, vertices, normals) = polygon(
            &mut e,
            &[10, 11, 12],
            &[[1.0, 0.0, 0.0], [-1.0, 0.0, 0.0], [1.0, 1.0, 0.0]],
        );
        let (out_indices, out_vertices, out_normals) = polygon(&mut e, &[0; 8], &[[0.0; 3]; 8]);
        let count = e
            .call(
                0x0068_d820,
                &args![
                    plane,
                    3i32,
                    indices,
                    vertices,
                    normals,
                    out_indices,
                    out_vertices,
                    out_normals
                ],
            )
            .i32();
        assert_eq!(count, 4);
        let kept: Vec<u32> = (0..4).map(|i| e.mem.u32(out_indices + 4 * i)).collect();
        assert_eq!(kept, vec![10, 0xffff_ffff, 0xffff_ffff, 12]);
        let result = points(&e, out_vertices, 4);
        assert!(near(result[0], [1.0, 0.0, 0.0]));
        assert!(near(result[1], [0.0, 0.0, 0.0]));
        assert!(near(result[2], [0.0, 0.5, 0.0]));
        assert!(near(result[3], [1.0, 1.0, 0.0]));
        let result = points(&e, out_normals, 4);
        assert!(result.iter().all(|n| near(*n, [0.0, 0.0, 1.0])));
    }

    #[test]
    fn clip_against_a_plane_keeps_or_drops_whole_polygons() {
        let mut e = decal_engine();
        let plane = e.mem.alloc(16);
        write_points(&mut e, plane, &[[1.0, 0.0, 0.0]]);
        let (indices, vertices, normals) = polygon(
            &mut e,
            &[1, 2, 3],
            &[[1.0, 0.0, 0.0], [2.0, 0.0, 0.0], [3.0, 1.0, 0.0]],
        );
        let (out_indices, out_vertices, out_normals) = polygon(&mut e, &[0; 4], &[[0.0; 3]; 4]);
        let args_of = |p: u32, i: u32, v: u32, n: u32| {
            args![p, 3i32, i, v, n, out_indices, out_vertices, out_normals]
        };
        assert_eq!(
            e.call(0x0068_d820, &args_of(plane, indices, vertices, normals))
                .i32(),
            3
        );
        assert_eq!(e.mem.u32(out_indices + 8), 3);
        assert_eq!(points(&e, out_vertices, 3)[2], [3.0, 1.0, 0.0]);

        // Everything behind the plane: nothing is written.
        let (back_indices, back_vertices, back_normals) = polygon(
            &mut e,
            &[1, 2, 3],
            &[[-1.0, 0.0, 0.0], [-2.0, 0.0, 0.0], [-3.0, 1.0, 0.0]],
        );
        assert_eq!(
            e.call(
                0x0068_d820,
                &args_of(plane, back_indices, back_vertices, back_normals)
            )
            .i32(),
            0
        );
    }

    #[test]
    fn clip_parameter_is_corrected_and_reported() {
        let mut e = decal_engine();
        let plane = e.mem.alloc(16);
        write_points(&mut e, plane, &[[1.0, 0.0, 0.0]]);
        // A point is "behind" when its y exceeds 5, whatever the plane says,
        // so the parameter can leave 0..1.
        e.register(PLANE_SIDE, |e, a| {
            value(if e.mem.f32(a[1] + 4) > 5.0 { 2 } else { 1 })
        });
        let (out_indices, out_vertices, out_normals) = polygon(&mut e, &[0; 8], &[[0.0; 3]; 8]);
        let run = |e: &mut Engine, inside: [f32; 3], outside: [f32; 3], other: [f32; 3]| {
            let (indices, vertices, normals) = polygon(e, &[1, 2, 3], &[inside, outside, other]);
            e.call_log = Some(vec![]);
            let count = e
                .call(
                    0x0068_d820,
                    &args![
                        plane,
                        3i32,
                        indices,
                        vertices,
                        normals,
                        out_indices,
                        out_vertices,
                        out_normals
                    ],
                )
                .i32();
            (count, calls_to(e, PRINT_LINE))
        };

        // t = 3 / (3 - 2) = 3 on both edges: reported twice, clamped to 1.
        let (count, messages) = run(&mut e, [3.0, 0.0, 0.0], [2.0, 10.0, 0.0], [3.0, 1.0, 0.0]);
        assert_eq!(count, 4);
        assert_eq!(messages.len(), 2);
        let t_words = 3.0f64.to_bits();
        assert_eq!(
            messages[0],
            vec![BAD_T_MESSAGE, t_words as u32, (t_words >> 32) as u32]
        );
        // t = 1 puts the new point on the outside point.
        assert!(near(points(&e, out_vertices, 4)[1], [2.0, 10.0, 0.0]));

        // t = 3 / (3 - 6) = -1: reported, clamped to 0 (the inside point).
        let (count, messages) = run(&mut e, [3.0, 0.0, 0.0], [6.0, 10.0, 0.0], [3.0, 1.0, 0.0]);
        assert_eq!(count, 4);
        assert_eq!(messages.len(), 2);
        assert!(near(points(&e, out_vertices, 4)[1], [3.0, 0.0, 0.0]));

        // t just above 1 (1.00005): silently corrected on the edge that
        // leaves the front, reported on the one that enters it.
        let (count, messages) = run(
            &mut e,
            [20000.0, 0.0, 0.0],
            [0.99995, 10.0, 0.0],
            [20000.0, 1.0, 0.0],
        );
        assert_eq!(count, 4);
        assert_eq!(messages.len(), 1);
        assert!(near(points(&e, out_vertices, 4)[1], [0.99995, 10.0, 0.0]));
    }

    #[test]
    fn clip_polygon_visits_the_planes_in_order() {
        let mut e = decal_engine();
        let decal = new_decal(&mut e);
        set_box_planes(&mut e, decal);
        let (indices, vertices, normals) = polygon(
            &mut e,
            &[10, 11, 12],
            &[[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
        );
        let (out_indices, out_vertices, out_normals) = polygon(&mut e, &[0; 12], &[[0.0; 3]; 12]);
        e.call_log = Some(vec![]);
        let count = e
            .call(
                0x0068_d660,
                &args![
                    decal,
                    3i32,
                    indices,
                    vertices,
                    normals,
                    out_indices,
                    out_vertices,
                    out_normals
                ],
            )
            .i32();
        assert_eq!(count, 3);
        assert_eq!(points(&e, out_vertices, 3)[1], [1.0, 0.0, 0.0]);
        assert_eq!(e.mem.u32(out_indices + 8), 12);
        // Left, right, bottom, top, back, front, three points each.
        let planes: Vec<u32> = calls_to(&e, PLANE_SIDE).iter().map(|a| a[0]).collect();
        let base = decal.addr() + 0x64;
        let expected: Vec<u32> = [0x00u32, 0x10, 0x20, 0x30, 0x50, 0x40]
            .iter()
            .flat_map(|offset| [base + offset; 3])
            .collect();
        assert_eq!(planes, expected);
    }

    #[test]
    fn clip_polygon_cuts_and_gives_up() {
        let mut e = decal_engine();
        let decal = new_decal(&mut e);
        set_box_planes(&mut e, decal);
        let (out_indices, out_vertices, out_normals) = polygon(&mut e, &[0; 12], &[[0.0; 3]; 12]);
        // One point beyond the right plane (x = 2).
        let (indices, vertices, normals) = polygon(
            &mut e,
            &[10, 11, 12],
            &[[0.0, 0.0, 0.0], [3.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
        );
        let count = e
            .call(
                0x0068_d660,
                &args![
                    decal,
                    3i32,
                    indices,
                    vertices,
                    normals,
                    out_indices,
                    out_vertices,
                    out_normals
                ],
            )
            .i32();
        assert_eq!(count, 4);
        let result = points(&e, out_vertices, 4);
        assert!(near(result[0], [0.0, 0.0, 0.0]));
        assert!(near(result[1], [2.0, 0.0, 0.0]));
        assert!(near(result[2], [2.0, 1.0 / 3.0, 0.0]));
        assert!(near(result[3], [0.0, 1.0, 0.0]));
        assert_eq!(e.mem.u32(out_indices + 4), 0xffff_ffff);

        // Wholly left of the left plane: no later plane is asked.
        let (indices, vertices, normals) = polygon(
            &mut e,
            &[10, 11, 12],
            &[[-3.0, 0.0, 0.0], [-4.0, 0.0, 0.0], [-3.0, 1.0, 0.0]],
        );
        e.call_log = Some(vec![]);
        let count = e
            .call(
                0x0068_d660,
                &args![
                    decal,
                    3i32,
                    indices,
                    vertices,
                    normals,
                    out_indices,
                    out_vertices,
                    out_normals
                ],
            )
            .i32();
        assert_eq!(count, 0);
        assert_eq!(calls_to(&e, PLANE_SIDE).len(), 3);
    }

    #[test]
    fn clip_mesh_adds_the_facing_triangle() {
        let (mut e, decal, geometry) = processing_scene();
        set_box_planes(&mut e, decal);
        e.call_log = Some(vec![]);
        e.call(0x0068_d230, &args![decal, geometry]);
        assert_eq!(e.get(decal, Decal::iDecalTriangleCount), 1);
        assert_eq!(e.get(decal, Decal::iDecalVertexCount), 3);
        let indices = e.get(decal, Decal::pTempIndexArray).addr();
        assert_eq!(
            (0..3)
                .map(|i| e.mem.u16(indices + 2 * i))
                .collect::<Vec<_>>(),
            vec![0, 1, 2]
        );
        let vertices = e.get(decal, Decal::pTempVertexArray).addr();
        assert_eq!(
            points(&e, vertices, 3),
            vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]]
        );
        // The array constructor ran for the two local arrays here and for the two
        // of the polygon clip.
        assert_eq!(calls_to(&e, VECTOR_CONSTRUCTOR).len(), 4);
    }

    #[test]
    fn clip_mesh_flips_odd_strip_triangles_and_skips_degenerate_ones() {
        let (mut e, decal, geometry) = processing_scene();
        set_box_planes(&mut e, decal);
        // The strip 0 1 2 0 0: triangle 0 faces the decal; triangle 1 is
        // (1, 2, 0), flipped to face away; triangle 2 is (2, 0, 0), degenerate.
        let strip_data = e.mem.u32(geometry + 0xe8);
        let strip = e.mem.u32(strip_data + 0x4c);
        for (i, index) in [0u16, 1, 2, 0, 0].iter().enumerate() {
            e.mem.set_u16(strip + 2 * i as u32, *index);
        }
        e.mem.set_u16(geometry + 0xec, 3);
        e.call(0x0068_d230, &args![decal, geometry]);
        assert_eq!(e.get(decal, Decal::iDecalTriangleCount), 1);

        // An odd position flips the winding back: the strip 1 1 0 2 has a
        // degenerate first triangle and (1, 0, 2) as the second, which is
        // flipped to (0, 1, 2) and faces the decal.
        let (mut e, decal, geometry) = processing_scene();
        set_box_planes(&mut e, decal);
        let strip_data = e.mem.u32(geometry + 0xe8);
        let strip = e.mem.u32(strip_data + 0x4c);
        for (i, index) in [1u16, 1, 0, 2].iter().enumerate() {
            e.mem.set_u16(strip + 2 * i as u32, *index);
        }
        e.mem.set_u16(geometry + 0xec, 2);
        e.call(0x0068_d230, &args![decal, geometry]);
        assert_eq!(e.get(decal, Decal::iDecalTriangleCount), 1);
        let indices = e.get(decal, Decal::pTempIndexArray).addr();
        assert_eq!(
            (0..3)
                .map(|i| e.mem.u16(indices + 2 * i))
                .collect::<Vec<_>>(),
            vec![0, 1, 2]
        );
    }

    #[test]
    fn clip_mesh_ignores_triangles_facing_away() {
        let (mut e, decal, geometry) = processing_scene();
        set_box_planes(&mut e, decal);
        write_points(&mut e, decal.addr() + 0x58, &[[0.0, 0.0, -1.0]]);
        e.call(0x0068_d230, &args![decal, geometry]);
        assert_eq!(e.get(decal, Decal::iDecalTriangleCount), 0);
        assert_eq!(e.get(decal, Decal::iDecalVertexCount), 0);
    }

    #[test]
    fn clip_mesh_reports_a_geometry_without_normals() {
        let (mut e, decal, geometry) = processing_scene();
        set_box_planes(&mut e, decal);
        e.mem.set_u32(geometry + 0xe4, 0);
        e.call_log = Some(vec![]);
        e.call(0x0068_d230, &args![decal, geometry]);
        assert_eq!(
            calls_to(&e, PRINT_LINE),
            vec![vec![NO_NORMALS_MESSAGE, 0x0777_0000]]
        );
        assert_eq!(e.get(decal, Decal::iDecalTriangleCount), 0);
        assert!(calls_to(&e, VECTOR_CONSTRUCTOR).is_empty());

        // No strip data at all: silently nothing.
        let (mut e, decal, geometry) = processing_scene();
        set_box_planes(&mut e, decal);
        e.mem.set_u32(geometry + 0xe8, 0);
        e.call_log = Some(vec![]);
        e.call(0x0068_d230, &args![decal, geometry]);
        assert!(calls_to(&e, PRINT_LINE).is_empty());
        assert_eq!(e.get(decal, Decal::iDecalTriangleCount), 0);
    }

    #[test]
    fn clip_mesh_stops_when_the_decal_is_full() {
        let (mut e, decal, geometry) = processing_scene();
        set_box_planes(&mut e, decal);
        e.set(decal, Decal::iDecalVertexCount, 0x1ff);
        e.call(0x0068_d230, &args![decal, geometry]);
        assert_eq!(e.get(decal, Decal::iDecalTriangleCount), 0);
        assert_eq!(e.get(decal, Decal::iDecalVertexCount), 0x1ff);
    }

    /// A decal over the scene's geometry (two target triangles, so the
    /// old-to-new map has six entries) ready for `0068cb60`.
    fn polygon_scene() -> (Engine, Ptr<Decal>) {
        let (mut e, decal, geometry) = processing_scene();
        e.mem.set_u16(geometry + 0xec, 2);
        write_points(&mut e, decal.addr() + 0x58, &[[0.0, 0.0, 1.0]]);
        e.register(POINTS_NEARLY_EQUAL, |e, a| {
            let close = (0..3).all(|i| {
                (e.mem.f32(a[0] + 4 * i) - e.mem.f32(a[1] + 4 * i)).abs() <= f32::from_bits(a[2])
            });
            value(close as u32)
        });
        (e, decal)
    }

    fn call_polygon(
        e: &mut Engine,
        decal: Ptr<Decal>,
        indices: &[u32],
        vertices: &[[f32; 3]],
        normals: &[[f32; 3]],
    ) -> (bool, u32) {
        let (index_block, vertex_block, normal_block) = polygon(e, indices, vertices);
        write_points(e, normal_block, normals);
        let ok = e
            .call(
                0x0068_cb60,
                &args![
                    decal,
                    indices.len() as i32,
                    index_block,
                    vertex_block,
                    normal_block
                ],
            )
            .bool();
        (ok, index_block)
    }

    #[test]
    fn polygon_adds_new_vertices_and_a_triangle() {
        let (mut e, decal) = polygon_scene();
        let normal = [0.0, 0.6, 0.8];
        let (ok, indices) = call_polygon(
            &mut e,
            decal,
            &[0xffff_ffff; 3],
            &[[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
            &[normal, [0.0, 0.0, -1.0], [0.0, 0.0, 1.0]],
        );
        assert!(ok);
        assert_eq!(e.get(decal, Decal::iDecalVertexCount), 3);
        assert_eq!(e.get(decal, Decal::iDecalTriangleCount), 1);
        assert_eq!(
            (0..3)
                .map(|i| e.mem.u32(indices + 4 * i))
                .collect::<Vec<_>>(),
            vec![0, 1, 2]
        );
        let temp_indices = e.get(decal, Decal::pTempIndexArray).addr();
        assert_eq!(
            (0..3)
                .map(|i| e.mem.u16(temp_indices + 2 * i))
                .collect::<Vec<_>>(),
            vec![0, 1, 2]
        );
        let vertices = e.get(decal, Decal::pTempVertexArray).addr();
        assert_eq!(points(&e, vertices, 2)[1], [1.0, 0.0, 0.0]);
        let normals = e.get(decal, Decal::pTempNormalArray).addr();
        assert_eq!(points(&e, normals, 3)[0], normal);
        // Alpha = (dot / length - 0.5) / (1 - 0.5): 0.6, then clamped to 0,
        // then 1.
        let colors = e.get(decal, Decal::pTempColorArray).addr();
        let alpha: Vec<f32> = (0..3).map(|i| e.mem.f32(colors + 16 * i + 12)).collect();
        assert!((alpha[0] - 0.6).abs() < 1e-5);
        assert_eq!(alpha[1], 0.0);
        assert_eq!(alpha[2], 1.0);
        // The decal vertices were recorded in the new index map.
        let map = decal.addr() + 0xf8;
        assert_eq!(e.mem.u32(map + 8), 3);
        // The scrap arrays and the old-to-new map exist now, the map holding -1.
        let old_to_new = e.get(decal, Decal::pOldToNewIndexArray).addr();
        assert_eq!(e.mem.i16(old_to_new), -1);
        assert_eq!(e.mem.block_size(old_to_new), Some(16));
    }

    #[test]
    fn polygon_welds_and_reuses_vertices() {
        let (mut e, decal) = polygon_scene();
        let up = [0.0, 0.0, 1.0];
        call_polygon(
            &mut e,
            decal,
            &[0xffff_ffff; 3],
            &[[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
            &[up; 3],
        );
        // A clipping vertex near an existing one is welded (index 1); the
        // target vertex 5 gets a new decal vertex (3); the far point is new (4).
        let (ok, indices) = call_polygon(
            &mut e,
            decal,
            &[0xffff_ffff, 5, 0xffff_ffff],
            &[[1.0005, 0.0, 0.0], [2.0, 2.0, 0.0], [5.0, 5.0, 0.0]],
            &[up; 3],
        );
        assert!(ok);
        assert_eq!(
            (0..3)
                .map(|i| e.mem.u32(indices + 4 * i))
                .collect::<Vec<_>>(),
            vec![1, 3, 4]
        );
        assert_eq!(e.get(decal, Decal::iDecalVertexCount), 5);
        assert_eq!(e.get(decal, Decal::iDecalTriangleCount), 2);
        let temp_indices = e.get(decal, Decal::pTempIndexArray).addr();
        assert_eq!(
            (3..6)
                .map(|i| e.mem.u16(temp_indices + 2 * i))
                .collect::<Vec<_>>(),
            vec![1, 3, 4]
        );
        // Only the clipping vertex that was not welded got a decal vertex.
        assert_eq!(e.mem.u32(decal.addr() + 0xf8 + 8), 4);

        // Target vertex 5 is mapped now: reused without a new vertex.
        let (ok, indices) =
            call_polygon(&mut e, decal, &[5, 5, 5], &[[2.0, 2.0, 0.0]; 3], &[up; 3]);
        assert!(ok);
        assert_eq!(
            (0..3)
                .map(|i| e.mem.u32(indices + 4 * i))
                .collect::<Vec<_>>(),
            vec![3, 3, 3]
        );
        assert_eq!(e.get(decal, Decal::iDecalVertexCount), 5);
        assert_eq!(e.get(decal, Decal::iDecalTriangleCount), 3);
    }

    #[test]
    fn polygon_refuses_to_overflow_the_vertex_limit() {
        let (mut e, decal) = polygon_scene();
        e.set(decal, Decal::iDecalVertexCount, 0x1fd);
        let up = [0.0, 0.0, 1.0];
        let (ok, _) = call_polygon(
            &mut e,
            decal,
            &[0xffff_ffff; 3],
            &[[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
            &[up; 3],
        );
        assert!(!ok);
        assert_eq!(e.get(decal, Decal::iDecalVertexCount), 0x1fd);
        assert!(e.get(decal, Decal::pTempVertexArray).is_null());

        // One vertex less than the limit is still accepted.
        e.set(decal, Decal::iDecalVertexCount, 0x1fc);
        let (ok, _) = call_polygon(
            &mut e,
            decal,
            &[0xffff_ffff; 3],
            &[[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
            &[up; 3],
        );
        assert!(ok);
        assert_eq!(e.get(decal, Decal::iDecalVertexCount), 0x1ff);
    }

    // ---- FinalizeGeometry ----

    const DECAL_VTABLE: u32 = 0x0110_4000;
    const LIGHTING_VTABLE: u32 = 0x0110_1000;
    const LIGHTING_CONFIG: u32 = 0x0110_2000;
    const TEXTURE_VTABLE: u32 = 0x0110_3000;
    const DECAL_NODE_VTABLE: u32 = 0x0110_5000;
    const LINK_OBJECT: u32 = 0x0110_6000;
    const LIGHTING_QUERY: u32 = 0x0060_f4f4;
    const TEXTURE_FETCH: u32 = 0x0060_9090;
    const DECAL_NODE_ADD: u32 = 0x0060_0104;
    const SCENE_NODE: u32 = 0x0abc_0000;
    const NODE_MEMBER: u32 = 0x0abc_a000;
    const LOADED_TEXTURE: u32 = 0x0abc_def0;

    struct Finalizing {
        e: Engine,
        decal: Ptr<Decal>,
        shape: u32,
        target: u32,
        parent: u32,
    }

    /// A decal with a shape, a target whose parent has no children, a
    /// lighting property that accepts both queries, a texture set and a
    /// target whose flag test passes. Constructors return `this`; every
    /// setter is a no-op the test inspects through the call log.
    fn finalize_scene() -> Finalizing {
        let mut e = decal_engine();
        for page in [
            0x0110_1000u32,
            0x0110_2000,
            0x0110_3000,
            0x0110_4000,
            0x0110_5000,
            0x0110_6000,
        ] {
            e.map(page, 0x1000);
        }
        returns_this(
            &mut e,
            &[
                MATERIAL_PROPERTY_CONSTRUCT,
                ALPHA_PROPERTY_CONSTRUCT,
                TWO_SIDED_PROPERTY_CONSTRUCT,
                NO_LIGHTING_PROPERTY_CONSTRUCT,
            ],
        );
        noops(
            &mut e,
            &[
                MATERIAL_SET_SPECULAR,
                ALPHA_SET_BLENDING,
                ALPHA_SET_TESTING,
                ALPHA_SET_TEST_FUNCTION,
                ALPHA_SET_THRESHOLD,
                TWO_SIDED_SET_MODE,
                LIGHTING_SET_TYPE,
                SHADER_SET_FLAG,
                LIGHTING_LOAD_TEXTURE_SET,
                LIGHTING_SET_PARALLAX_SCALE,
                NO_LIGHTING_SET_TEXTURE,
                ATTACH_PROPERTY,
                PREPARE_OBJECT,
                SET_NAME,
                REMOVE_HAVOK,
                SET_OBJECT_TYPE_RECURSE,
                ADD_TEMP_EFFECT,
                LIST_ADD,
                LIST_REMOVE_HEAD,
                NODE_SET_TRANSLATE,
                NODE_SET_ROTATE,
                NODE_SET_SCALE,
                NODE_UPDATE_PROPERTIES,
                WORLD_POINT_CONSTRUCT,
                NODE_SET_WORLD_POINT,
                UPDATE_OBJECT_LIGHTING,
                DECAL_NODE_MEMBER_ADD,
                DECAL_NODE_ADD,
            ],
        );
        e.register(LIGHTING_PROPERTY_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], LIGHTING_VTABLE);
            value(a[0])
        });
        e.put_vtable(LIGHTING_VTABLE, &[0u32; 62]);
        e.mem.set_u32(LIGHTING_VTABLE + 0xf4, LIGHTING_QUERY);
        // The lighting property's query answers from the config words.
        e.register(LIGHTING_QUERY, |e, a| {
            value(e.mem.u32(LIGHTING_CONFIG + 4 * a[1]))
        });
        e.mem.set_u32(LIGHTING_CONFIG + 4, 1);
        e.mem.set_u32(LIGHTING_CONFIG + 12, 1);
        e.register(TEXTURE_SET_SHADER_SET, |_, _| value(0x0abc_0abc));
        // The texture set's second base: slot 0x90 stores a texture.
        e.put_vtable(TEXTURE_VTABLE, &[0u32; 37]);
        e.mem.set_u32(TEXTURE_VTABLE + 0x90, TEXTURE_FETCH);
        e.register(TEXTURE_FETCH, |e, a| {
            e.mem.set_u32(a[2], LOADED_TEXTURE);
            Ret::default()
        });
        e.register(TARGET_LINK_A, |_, a| value(a[0]));
        e.register(TARGET_LINK_B, |_, _| value(LINK_OBJECT));
        e.mem.set_u16(LINK_OBJECT + 0x18, 1);
        e.register(TRANSIENT_NAME_WORD_GETTER, |_, _| value(0x1111));
        e.set_global(PERMANENT_NAME_WORD, 0x2222u32);
        e.register(NODE_LOCAL_TRANSLATE, |_, a| value(a[0] + 0x58));
        e.register(
            NODE_LOCAL_SCALE,
            |e, a| float(e.mem.f32(a[0] + 0x98) as f64),
        );
        // Children of a node: count at +0x100, pointer array at +0x104.
        e.register(NODE_CHILD_COUNT, |e, a| value(e.mem.u32(a[0] + 0x100)));
        e.register(NODE_CHILD, |e, a| {
            let children = e.mem.u32(a[0] + 0x104);
            value(e.mem.u32(children + 4 * a[1]))
        });
        e.register(IS_KIND_OF_CHILD, |e, a| {
            value((a[0] == DECAL_NODE_RTTI && e.mem.u32(a[1] + 0x10) == 0xdeca) as u32)
        });
        e.put_vtable(DECAL_NODE_VTABLE, &[0u32; 66]);
        e.mem.set_u32(DECAL_NODE_VTABLE + 0x104, DECAL_NODE_ADD);
        e.register(DECAL_NODE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], DECAL_NODE_VTABLE);
            e.mem.set_u32(a[0] + 0xac, NODE_MEMBER);
            value(a[0])
        });
        e.register(SHADOW_SCENE_NODE_GETTER, |_, _| value(SCENE_NODE));
        e.register(GEOMETRY_COLORS, |e, a| value(e.mem.u32(a[0] + 0xe8)));
        e.register(GEOMETRY_VERTEX_COUNT, |e, a| {
            value(e.mem.u16(a[0] + 0xec) as u32)
        });

        let parent = e.mem.alloc(0x120);
        let target = e.mem.alloc(0x100);
        e.mem.set_u32(target + 0x18, parent);
        e.mem.set_f32(target + 0x98, 2.0);
        let shape = e.mem.alloc(0x100);
        let colors = e.mem.alloc(32);
        e.mem.set_u32(shape + 0xe8, colors);
        e.mem.set_u16(shape + 0xec, 2);
        let texture_set = e.mem.alloc(0x80);
        e.mem.set_u32(texture_set + 0x30, TEXTURE_VTABLE);

        let decal = new_decal(&mut e);
        e.mem.set_u32(decal.addr(), DECAL_VTABLE);
        e.put_vtable(DECAL_VTABLE, &[0u32; 39]);
        e.mem.set_u32(DECAL_VTABLE + 0x98, 0x0068_acc0);
        e.set(decal, Decal::sp3D, Ptr::new(shape));
        e.set(decal, Decal::spTargetNode, Ptr::new(target));
        e.set(decal, Decal::pTextureSet, Ptr::new(texture_set));
        e.set(decal, Decal::fSpecular, 0.5);
        e.set(decal, Decal::fParallaxOcclusionScale, 0.25);
        e.set(decal, Decal::ucParallaxOcclusionMaxPasses, 7);
        e.set_global(MAX_DECAL_COUNT, 3i32);
        e.set_global(LISTED_DECAL_COUNT, 0i32);
        e.call_log = Some(vec![]);
        Finalizing {
            e,
            decal,
            shape,
            target,
            parent,
        }
    }

    fn flags_set(e: &Engine, object: u32) -> Vec<u32> {
        calls_to(e, SHADER_SET_FLAG)
            .iter()
            .filter(|a| a[0] == object)
            .map(|a| a[1])
            .collect()
    }

    #[test]
    fn finalize_builds_the_lighting_path_for_a_normal_decal() {
        let Finalizing {
            mut e,
            decal,
            shape,
            target,
            parent,
        } = finalize_scene();
        e.set(decal, Decal::bUseParallaxOcclusion, true);
        e.set(decal, Decal::bUseAlphaBlending, true);
        e.set_global(ORIGIN_X, 1.5f32);
        assert!(e.call(0x0068_be90, &args![decal]).bool());

        // Material, alpha and lighting properties are attached to the shape.
        let attached = calls_to(&e, ATTACH_PROPERTY);
        assert_eq!(attached.len(), 3);
        assert!(attached.iter().all(|a| a[0] == shape));
        let lighting = attached[2][1];
        assert_eq!(e.mem.u32(lighting), LIGHTING_VTABLE);
        assert_eq!(
            calls_to(&e, MATERIAL_SET_SPECULAR),
            vec![vec![attached[0][1], 0.5f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&e, ALPHA_SET_BLENDING),
            vec![vec![attached[1][1], 1]]
        );
        assert_eq!(
            calls_to(&e, ALPHA_SET_TESTING),
            vec![vec![attached[1][1], 0]]
        );
        assert_eq!(
            calls_to(&e, ALPHA_SET_TEST_FUNCTION),
            vec![vec![attached[1][1], 4]]
        );
        assert_eq!(
            calls_to(&e, ALPHA_SET_THRESHOLD),
            vec![vec![attached[1][1], 0x80]]
        );
        assert_eq!(calls_to(&e, LIGHTING_SET_TYPE), vec![vec![lighting, 0x1d]]);
        assert_eq!(
            calls_to(&e, LIGHTING_LOAD_TEXTURE_SET),
            vec![vec![lighting, 0, 0x0abc_0abc]]
        );
        // Flags: 5, then 0xB and the parallax pair, then 0x1B and 0x35.
        assert_eq!(flags_set(&e, lighting), vec![5, 0xb, 0x1c, 0x1b, 0x35]);
        assert_eq!(
            calls_to(&e, LIGHTING_SET_PARALLAX_SCALE),
            vec![vec![lighting, 0.25f32.to_bits()]]
        );
        assert_eq!(e.mem.f32(lighting + 0xe8), 7.0);
        assert_eq!(calls_to(&e, PREPARE_OBJECT), vec![vec![shape, 0, 0]]);
        // A non-permanent shape gets the transient name and the shader type.
        assert_eq!(calls_to(&e, SET_NAME), vec![vec![shape, 0x1111]]);
        assert_eq!(calls_to(&e, REMOVE_HAVOK), vec![vec![shape]]);
        assert_eq!(calls_to(&e, SET_OBJECT_TYPE_RECURSE), vec![vec![shape, 5]]);

        // The decal joins the list; no limit was reached.
        let added = calls_to(&e, LIST_ADD);
        assert_eq!(added.len(), 1);
        assert_eq!(added[0][0], DECAL_LIST);
        assert_eq!(e.global::<i32>(LISTED_DECAL_COUNT), 1);
        assert!(calls_to(&e, ADD_TEMP_EFFECT).is_empty());

        // A decal node is created under the target's parent and the decal
        // is added to it.
        let node = e.get(decal, Decal::pParentNode).addr();
        assert_eq!(
            calls_to(&e, DECAL_NODE_CONSTRUCT),
            vec![vec![node, parent, 0]]
        );
        assert_eq!(
            calls_to(&e, DECAL_NODE_ADD),
            vec![vec![node, decal.addr(), 1]]
        );
        // The target's transform is copied onto the shape.
        assert_eq!(
            calls_to(&e, NODE_SET_TRANSLATE),
            vec![vec![shape, target + 0x58]]
        );
        assert_eq!(
            calls_to(&e, NODE_SET_ROTATE),
            vec![vec![shape, target + 0x34]]
        );
        assert_eq!(
            calls_to(&e, NODE_SET_SCALE),
            vec![vec![shape, 2.0f32.to_bits()]]
        );
        assert_eq!(calls_to(&e, NODE_UPDATE_PROPERTIES), vec![vec![shape]]);
        let point_call = &calls_to(&e, WORLD_POINT_CONSTRUCT)[0];
        assert_eq!(&point_call[1..], &[1.5f32.to_bits(), 0, 0]);
        assert_eq!(
            calls_to(&e, NODE_SET_WORLD_POINT),
            vec![vec![shape, point_call[0]]]
        );
        assert_eq!(
            calls_to(&e, UPDATE_OBJECT_LIGHTING),
            vec![vec![SCENE_NODE, shape, 0]]
        );
        let member_call = &calls_to(&e, DECAL_NODE_MEMBER_ADD)[0];
        assert_eq!(member_call[0], NODE_MEMBER);

        assert!(e.get(decal, Decal::bValidDecal));
        assert!(e.get(decal, Decal::bFinished));
        assert!(!e.get(decal, Decal::bFinalize));
        // `CleanUpMemory` ran while the decal was not yet valid, so it asked
        // for the scrap heap to give the temporary arrays back.
        assert_eq!(calls_to(&e, THREAD_SCRAP_HEAP).len(), 1);
    }

    #[test]
    fn finalize_drops_the_oldest_decal_over_the_limit() {
        let Finalizing { mut e, decal, .. } = finalize_scene();
        e.set_global(LISTED_DECAL_COUNT, 3i32);
        e.mem.set_u32(DECAL_LIST, 0x0aaa_0001);
        e.mem.set_u8(DIAGNOSTICS_SETTING + 4, 1);
        assert!(e.call(0x0068_be90, &args![decal]).bool());
        // The counter went to 4, over the limit 3: the oldest is handed to the
        // process lists, the list head removed, the counter back to 3 and
        // the message printed.
        assert_eq!(
            calls_to(&e, ADD_TEMP_EFFECT),
            vec![vec![PROCESS_LISTS, 0x0aaa_0001]]
        );
        assert_eq!(calls_to(&e, LIST_REMOVE_HEAD), vec![vec![DECAL_LIST]]);
        assert_eq!(e.global::<i32>(LISTED_DECAL_COUNT), 3);
        assert_eq!(calls_to(&e, PRINT_LINE), vec![vec![DECAL_LIMIT_MESSAGE, 3]]);

        // Without the diagnostics setting nothing is printed.
        let Finalizing { mut e, decal, .. } = finalize_scene();
        e.set_global(LISTED_DECAL_COUNT, 3i32);
        e.call(0x0068_be90, &args![decal]);
        assert_eq!(calls_to(&e, ADD_TEMP_EFFECT).len(), 1);
        assert!(calls_to(&e, PRINT_LINE).is_empty());
    }

    #[test]
    fn finalize_uses_the_no_lighting_property_when_the_shader_refuses() {
        let Finalizing {
            mut e,
            decal,
            shape,
            ..
        } = finalize_scene();
        e.mem.set_u32(LIGHTING_CONFIG + 4, 0);
        assert!(e.call(0x0068_be90, &args![decal]).bool());
        let attached = calls_to(&e, ATTACH_PROPERTY);
        assert_eq!(attached.len(), 3);
        let no_lighting = attached[2][1];
        assert_ne!(e.mem.u32(no_lighting), LIGHTING_VTABLE);
        assert_eq!(
            calls_to(&e, NO_LIGHTING_SET_TEXTURE),
            vec![vec![no_lighting, LOADED_TEXTURE]]
        );
        assert_eq!(flags_set(&e, no_lighting), vec![0x1b, 0x35]);
        // The lighting property only got its first flag.
        let lighting_flags: Vec<u32> = calls_to(&e, SHADER_SET_FLAG)
            .iter()
            .filter(|a| a[0] != no_lighting)
            .map(|a| a[1])
            .collect();
        assert_eq!(lighting_flags, vec![5]);
        assert_eq!(calls_to(&e, PREPARE_OBJECT), vec![vec![shape, 0, 0]]);
    }

    #[test]
    fn finalize_colours_the_vertices_yellow_with_the_debug_flag() {
        let Finalizing {
            mut e,
            decal,
            shape,
            ..
        } = finalize_scene();
        e.mem.set_u32(LIGHTING_CONFIG + 4, 0);
        e.set_global(DECAL_DEBUG_FLAGS, 0x20u32);
        assert!(e.call(0x0068_be90, &args![decal]).bool());
        // With the flag set the texture fetch is skipped.
        assert!(calls_to(&e, NO_LIGHTING_SET_TEXTURE).is_empty());
        let colors = e.mem.u32(shape + 0xe8);
        for i in 0..2 {
            let at = colors + 16 * i;
            assert_eq!(
                [
                    e.mem.f32(at),
                    e.mem.f32(at + 4),
                    e.mem.f32(at + 8),
                    e.mem.f32(at + 12)
                ],
                [1.0, 1.0, 0.0, 1.0]
            );
        }
    }

    #[test]
    fn finalize_without_a_texture_set_skips_the_texture() {
        let Finalizing { mut e, decal, .. } = finalize_scene();
        e.mem.set_u32(LIGHTING_CONFIG + 4, 0);
        e.set(decal, Decal::pTextureSet, Ptr::NULL);
        e.call(0x0068_be90, &args![decal]);
        assert!(calls_to(&e, NO_LIGHTING_SET_TEXTURE).is_empty());
    }

    #[test]
    fn finalize_skips_the_flag_35_when_the_target_flag_test_fails() {
        let Finalizing { mut e, decal, .. } = finalize_scene();
        e.mem.set_u16(LINK_OBJECT + 0x18, 0);
        e.call(0x0068_be90, &args![decal]);
        let attached = calls_to(&e, ATTACH_PROPERTY);
        assert_eq!(flags_set(&e, attached[2][1]), vec![5, 0xb, 0x1b]);
    }

    #[test]
    fn finalize_names_a_permanent_shape_and_leaves_the_lists_alone() {
        let Finalizing {
            mut e,
            decal,
            shape,
            ..
        } = finalize_scene();
        e.set(decal, Decal::bPermanent, true);
        assert!(e.call(0x0068_be90, &args![decal]).bool());
        assert_eq!(calls_to(&e, SET_NAME), vec![vec![shape, 0x2222]]);
        assert!(calls_to(&e, LIST_ADD).is_empty());
        assert!(calls_to(&e, ADD_TEMP_EFFECT).is_empty());
        assert_eq!(e.global::<i32>(LISTED_DECAL_COUNT), 0);
    }

    #[test]
    fn finalize_hands_a_force_fade_decal_to_the_process_lists() {
        let Finalizing { mut e, decal, .. } = finalize_scene();
        e.set(decal, Decal::bForceFade, true);
        assert!(e.call(0x0068_be90, &args![decal]).bool());
        assert_eq!(
            calls_to(&e, ADD_TEMP_EFFECT),
            vec![vec![PROCESS_LISTS, decal.addr()]]
        );
        assert!(calls_to(&e, LIST_ADD).is_empty());
        assert_eq!(e.global::<i32>(LISTED_DECAL_COUNT), 0);
    }

    #[test]
    fn finalize_adds_the_two_sided_property() {
        let Finalizing { mut e, decal, .. } = finalize_scene();
        e.set(decal, Decal::bTwoSided, true);
        e.call(0x0068_be90, &args![decal]);
        let attached = calls_to(&e, ATTACH_PROPERTY);
        assert_eq!(attached.len(), 4);
        assert_eq!(
            calls_to(&e, TWO_SIDED_SET_MODE),
            vec![vec![attached[2][1], 3]]
        );
    }

    #[test]
    fn finalize_reuses_an_existing_decal_node() {
        let Finalizing {
            mut e,
            decal,
            parent,
            ..
        } = finalize_scene();
        let plain = e.mem.alloc(0x100);
        let existing = e.mem.alloc(0x100);
        e.mem.set_u32(existing, DECAL_NODE_VTABLE);
        e.mem.set_u32(existing + 0x10, 0xdeca);
        e.mem.set_u32(existing + 0xac, NODE_MEMBER);
        let children = e.mem.alloc(16);
        e.mem.set_u32(children, 0);
        e.mem.set_u32(children + 4, plain);
        e.mem.set_u32(children + 8, existing);
        e.mem.set_u32(parent + 0x100, 3);
        e.mem.set_u32(parent + 0x104, children);
        assert!(e.call(0x0068_be90, &args![decal]).bool());
        assert!(calls_to(&e, DECAL_NODE_CONSTRUCT).is_empty());
        assert_eq!(e.get(decal, Decal::pParentNode), Ptr::new(existing));
        assert_eq!(
            calls_to(&e, DECAL_NODE_ADD),
            vec![vec![existing, decal.addr(), 1]]
        );
    }

    #[test]
    fn finalize_fails_without_a_shape_or_a_parent() {
        let Finalizing { mut e, decal, .. } = finalize_scene();
        e.set(decal, Decal::sp3D, Ptr::NULL);
        assert!(!e.call(0x0068_be90, &args![decal]).bool());
        assert!(e.get(decal, Decal::bFinished));
        assert!(calls_to(&e, ATTACH_PROPERTY).is_empty());

        let Finalizing {
            mut e,
            decal,
            target,
            ..
        } = finalize_scene();
        e.mem.set_u32(target + 0x18, 0);
        assert!(!e.call(0x0068_be90, &args![decal]).bool());
        assert!(e.get(decal, Decal::bFinished));
        assert!(!e.get(decal, Decal::bValidDecal));
        assert!(calls_to(&e, DECAL_NODE_ADD).is_empty());
    }

    // ---- CreateDebugBox ----

    const BOX_STREAM_ROOT: u32 = 0x0110_7100;
    const BOX_COPY_ROOT: u32 = 0x0110_7000;
    const BOX_VTABLE: u32 = 0x0110_7200;
    const BOX_SHAPE_LOOKUP: u32 = 0x0060_9c9c;
    const ROOT_PARENT_ADD: u32 = 0x0060_bcbc;
    const HOLDER_ATTACH: u32 = 0x0060_dcdc;

    struct DebugBox {
        e: Engine,
        decal: Ptr<Decal>,
        color: u32,
        shape: u32,
        target: u32,
    }

    /// A decal 2 x 3 x 4 whose target has a parent; the debug box is not
    /// loaded yet.
    fn debug_box_scene() -> DebugBox {
        let mut e = decal_engine();
        e.map(0x0110_7000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        returns_this(
            &mut e,
            &[
                ALPHA_PROPERTY_CONSTRUCT,
                NO_LIGHTING_PROPERTY_CONSTRUCT,
                FIXED_STRING_CONSTRUCT,
            ],
        );
        noops(
            &mut e,
            &[
                ALPHA_SET_BLENDING,
                SHADER_SET_FLAG,
                ATTACH_PROPERTY,
                PREPARE_OBJECT,
                NODE_SET_TRANSLATE,
                NODE_SET_ROTATE,
                NODE_SET_SCALE,
                PROPERTY_LIST_CLEAR,
                REMOVE_HAVOK,
                SET_NAME,
                FIXED_STRING_RELEASE,
                STREAM_CONSTRUCT,
                STREAM_DESTRUCT,
                TES_ADD_DEBUG_OBJECT,
                WORLD_POINT_CONSTRUCT,
                NODE_SET_WORLD_POINT,
                ROOT_PARENT_ADD,
                HOLDER_ATTACH,
            ],
        );
        e.register(STREAM_LOAD, |_, _| value(1));
        e.register(STREAM_ROOT_COUNT, |_, _| value(1));
        e.register(STREAM_ROOT, |_, _| value(BOX_STREAM_ROOT));
        e.register(DEEP_COPY, |e, a| {
            e.mem.set_u32(a[1], BOX_COPY_ROOT);
            value(a[1])
        });
        e.register(FLOAT_NEARLY_EQUAL, |_, a| {
            value(
                ((f32::from_bits(a[0]) - f32::from_bits(a[1])).abs() <= f32::from_bits(a[2]))
                    as u32,
            )
        });
        e.register(POINT3_EQUALS, |e, a| {
            value((0..3).all(|i| e.mem.f32(a[0] + 4 * i) == e.mem.f32(a[1] + 4 * i)) as u32)
        });
        e.register(GEOMETRY_VERTICES, |e, a| value(e.mem.u32(a[0] + 0xe0)));
        e.register(GEOMETRY_COLORS, |e, a| value(e.mem.u32(a[0] + 0xe8)));
        e.register(GEOMETRY_VERTEX_COUNT, |e, a| {
            value(e.mem.u16(a[0] + 0xec) as u32)
        });
        e.register(NODE_LOCAL_TRANSLATE, |_, a| value(a[0] + 0x58));
        e.register(
            NODE_LOCAL_SCALE,
            |e, a| float(e.mem.f32(a[0] + 0x98) as f64),
        );
        e.register(DEBUG_HOLDER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], BOX_VTABLE);
            value(a[0])
        });
        e.put_vtable(BOX_VTABLE, &[0u32; 60]);
        e.mem.set_u32(BOX_VTABLE + 0xdc, HOLDER_ATTACH);
        e.mem.set_u32(BOX_VTABLE + 0x9c, BOX_SHAPE_LOOKUP);
        e.mem.set_u32(BOX_VTABLE + 0xbc, ROOT_PARENT_ADD);
        // The copied root: its shape lookup returns the shape, its parent
        // answers slot 0xBC.
        let shape = e.mem.alloc(0x100);
        let vertices = e.mem.alloc(24);
        write_points(&mut e, vertices, &[[1.0, 2.0, 3.0], [-1.0, -2.0, -3.0]]);
        let colors = e.mem.alloc(32);
        e.mem.set_u32(shape + 0xe0, vertices);
        e.mem.set_u32(shape + 0xe8, colors);
        e.mem.set_u16(shape + 0xec, 2);
        e.register_double(BOX_SHAPE_LOOKUP, move |_, _| value(shape));
        e.mem.set_u32(BOX_COPY_ROOT, BOX_VTABLE);
        let root_parent = e.mem.alloc(0x40);
        e.mem.set_u32(root_parent, BOX_VTABLE);
        e.mem.set_u32(BOX_COPY_ROOT + 0x18, root_parent);
        let tes = e.mem.alloc(0x40);
        e.set_global(TES_SINGLETON, tes);

        let target_parent = e.mem.alloc(0x40);
        e.mem.set_u32(target_parent, BOX_VTABLE);
        let target = e.mem.alloc(0x100);
        e.mem.set_u32(target + 0x18, target_parent);
        e.mem.set_f32(target + 0x98, 0.5);
        write_identity(&mut e, target + 0x34);
        write_identity(&mut e, target + 0x68);

        let decal = new_decal(&mut e);
        e.set(decal, Decal::fWidth, 2.0);
        e.set(decal, Decal::fHeight, 3.0);
        e.set(decal, Decal::fDepth, 4.0);
        e.set(decal, Decal::spTargetNode, Ptr::new(target));
        write_points(&mut e, decal.addr() + 0x34, &[[5.0, 6.0, 7.0]]);
        write_points(&mut e, decal.addr() + 0x4c, &[[8.0, 9.0, 10.0]]);
        let color = e.mem.alloc(16);
        for (i, v) in [0.25f32, 0.5, 0.75, 1.0].iter().enumerate() {
            e.mem.set_f32(color + 4 * i as u32, *v);
        }
        e.call_log = Some(vec![]);
        DebugBox {
            e,
            decal,
            color,
            shape,
            target,
        }
    }

    #[test]
    fn debug_box_is_loaded_once_and_named() {
        let DebugBox {
            mut e,
            decal,
            color,
            ..
        } = debug_box_scene();
        write_points(&mut e, decal.addr() + 0x58, &[[0.0, 0.0, 1.0]]);
        e.call(0x0068_dea0, &args![decal, color, false]);
        let loads = calls_to(&e, STREAM_LOAD);
        assert_eq!(loads.len(), 1);
        assert_eq!(&loads[0][1..], &[DEBUG_BOX_PATH, 0]);
        assert_eq!(e.mem.u32(DEBUG_DECAL_BOX), BOX_STREAM_ROOT);
        let fixed = calls_to(&e, FIXED_STRING_CONSTRUCT);
        assert_eq!(fixed[0][1], DEBUG_BOX_NAME);
        assert_eq!(fixed[1][1], DEBUG_BOX_SHAPE_NAME);
        assert_eq!(calls_to(&e, SET_NAME)[0][0], BOX_STREAM_ROOT);

        // The second call finds the box and loads nothing.
        e.call(0x0068_dea0, &args![decal, color, false]);
        assert_eq!(calls_to(&e, STREAM_LOAD).len(), 1);
    }

    #[test]
    fn debug_box_is_skipped_when_the_file_cannot_be_loaded() {
        let DebugBox {
            mut e,
            decal,
            color,
            ..
        } = debug_box_scene();
        e.register(STREAM_LOAD, |_, _| value(0));
        e.call(0x0068_dea0, &args![decal, color, false]);
        assert_eq!(e.mem.u32(DEBUG_DECAL_BOX), 0);
        assert!(calls_to(&e, DEEP_COPY).is_empty());

        // A file with two roots is not used either.
        e.register(STREAM_LOAD, |_, _| value(1));
        e.register(STREAM_ROOT_COUNT, |_, _| value(2));
        e.call(0x0068_dea0, &args![decal, color, false]);
        assert_eq!(e.mem.u32(DEBUG_DECAL_BOX), 0);
        assert!(calls_to(&e, DEEP_COPY).is_empty());
    }

    #[test]
    fn debug_box_world_space_places_the_copy_with_the_decal() {
        let DebugBox {
            mut e,
            decal,
            color,
            shape,
            target,
        } = debug_box_scene();
        // Near-zero components of the decal normal are zeroed.
        write_points(&mut e, decal.addr() + 0x58, &[[0.0005, -0.0005, 1.0]]);
        e.call(0x0068_dea0, &args![decal, color, true]);
        assert_eq!(points(&e, decal.addr() + 0x58, 1), vec![[0.0, 0.0, 1.0]]);
        // The shape's vertices are scaled by width, height and twice the
        // depth, and take the colour.
        let vertices = e.mem.u32(shape + 0xe0);
        assert_eq!(
            points(&e, vertices, 2),
            vec![[2.0, 6.0, 24.0], [-2.0, -6.0, -24.0]]
        );
        let colors = e.mem.u32(shape + 0xe8);
        for i in 0..2 {
            assert_eq!(e.mem.f32(colors + 16 * i), 0.25);
            assert_eq!(e.mem.f32(colors + 16 * i + 12), 1.0);
        }
        assert_eq!(calls_to(&e, REMOVE_HAVOK), vec![vec![BOX_COPY_ROOT]]);
        // The shape gets the target's world rotation (turned by the identity
        // rotations of the doubles) and the decal's origin.
        let rotate = calls_to(&e, NODE_SET_ROTATE);
        assert_eq!(rotate.len(), 1);
        assert_eq!(rotate[0][0], shape);
        assert_eq!(
            matrix_of(&e, rotate[0][1]),
            [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]
        );
        assert_eq!(
            calls_to(&e, NODE_SET_TRANSLATE),
            vec![vec![shape, decal.addr() + 0x34]]
        );
        let _ = target;
        // The copy's properties are cleared; an alpha property goes on the
        // copy, a no-lighting property (with flag 0x30) on the shape.
        assert_eq!(
            calls_to(&e, PROPERTY_LIST_CLEAR),
            vec![vec![BOX_COPY_ROOT + 0x24]]
        );
        let attached = calls_to(&e, ATTACH_PROPERTY);
        assert_eq!(attached.len(), 2);
        assert_eq!(attached[0][0], BOX_COPY_ROOT);
        assert_eq!(attached[1][0], shape);
        assert_eq!(flags_set(&e, attached[1][1]), vec![0x30]);
        assert_eq!(
            calls_to(&e, ALPHA_SET_BLENDING),
            vec![vec![attached[0][1], 1]]
        );
        assert_eq!(calls_to(&e, PREPARE_OBJECT), vec![vec![shape, 0, 0]]);
        // A world-space box is a debug object of the TES singleton.
        assert_eq!(
            calls_to(&e, TES_ADD_DEBUG_OBJECT),
            vec![vec![
                e.global::<u32>(TES_SINGLETON),
                BOX_COPY_ROOT,
                30.0f32.to_bits()
            ]]
        );
        // The copy's parent is told to update (slot 0xBC).
        assert_eq!(calls_to(&e, ROOT_PARENT_ADD).len(), 1);
        assert_eq!(calls_to(&e, NI_POINTER_RELEASE).len(), 1);
    }

    #[test]
    fn debug_box_without_the_flag_leaves_the_lighting_flag_alone() {
        let DebugBox {
            mut e,
            decal,
            color,
            ..
        } = debug_box_scene();
        write_points(&mut e, decal.addr() + 0x58, &[[0.0, 0.0, 1.0]]);
        e.call(0x0068_dea0, &args![decal, color, false]);
        assert!(calls_to(&e, SHADER_SET_FLAG).is_empty());
    }

    #[test]
    fn debug_box_in_model_space_hangs_under_the_target() {
        let DebugBox {
            mut e,
            decal,
            color,
            shape,
            target,
        } = debug_box_scene();
        e.set(decal, Decal::bModelSpace, true);
        // A zero normal selects the decal's vector brought into the target's
        // space.
        write_points(&mut e, decal.addr() + 0x58, &[[0.0, 0.0, 0.0]]);
        write_points(&mut e, decal.addr() + 0x40, &[[0.0, 0.0, 1.0]]);
        e.call(0x0068_dea0, &args![decal, color, false]);
        // The copy takes the target's local transform.
        assert_eq!(
            calls_to(&e, NODE_SET_TRANSLATE),
            vec![
                vec![BOX_COPY_ROOT, target + 0x58],
                vec![shape, decal.addr() + 0x4c],
            ]
        );
        assert_eq!(
            calls_to(&e, NODE_SET_SCALE),
            vec![vec![BOX_COPY_ROOT, 0.5f32.to_bits()]]
        );
        let rotate = calls_to(&e, NODE_SET_ROTATE);
        assert_eq!(rotate[0], vec![BOX_COPY_ROOT, target + 0x34]);
        assert_eq!(rotate[1][0], shape);
        // The copy hangs under a new holder, itself attached to the target's
        // parent; no TES debug object is added.
        assert!(calls_to(&e, TES_ADD_DEBUG_OBJECT).is_empty());
        let holder_attach = calls_to(&e, HOLDER_ATTACH);
        assert_eq!(holder_attach.len(), 2);
        let holder = holder_attach[0].clone();
        assert_eq!(holder[1..], [BOX_COPY_ROOT, 1]);
        assert_eq!(holder_attach[1][1], holder[0]);
        let constructed = calls_to(&e, DEBUG_HOLDER_CONSTRUCT);
        assert_eq!(constructed[0][1], 30.0f32.to_bits());
    }
}
