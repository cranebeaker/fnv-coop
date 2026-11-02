//! `fallout shared/tesland.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The unit holds `TESObjectLAND`, the landscape record of one exterior
//! cell (152 functions to translate). The first session translated the
//! first 40 open or traced functions by address, `00533120` to `005374f0`:
//! the constructor and destructors, `InitializeStatics` (the shared default
//! triangle list, colours, normals, texture coordinates and vertex blocks),
//! the cell accessors and the flag bits of the record data, the vertex,
//! normal and colour readers, the form-record `Load`, `LoadVertices` with
//! `LoadVerticesIntoArrays` (the loader that reads the land chunks straight
//! from the plugin file), `UnLoadVertices`, `MakeLandTriStrips` and the
//! function that builds the four quadrant meshes (`005374f0`). The second
//! session translated the next 40 by address, `00537b10` to `00539f40`: the
//! small getters and constructors the mesh builder calls, the
//! `NiAdditionalGeometryData` functions, the release of the loaded data
//! (`00537eb0`), `Save` (`00538110`), the Havok MOPP code object and its
//! base classes, the save-reference slots of the land's vtable, the data
//! allocator (`00539500`) and the material builder (`00539960`). The third
//! session translated the next 40 by address, `00539f50` to `0053e290`: the
//! binary extra data of the percent arrays, `UpdateMesh` (`0053a090`), the
//! `CoordData` record and `GetCoordData` (`0053b550`), the layer texture and
//! opacity accessors and the cleaning of the layers (`0053a940`,
//! `0053aeb0`), the grass parameter builder (`0053bc10`), the position and
//! normal at a place (`0053caf0`, `0053d2e0`), the eight-neighbour search
//! (`0053d330`) and the border normals with their helpers (`0053d8f0`,
//! `0053db20`, `0053df30`, `0053e290`). The fourth session translated the
//! last 32 functions, `0053f0e0` to `00540bc0`, which finishes the unit: the
//! mesh node, height, extents and colour at a position (`0053f0e0` to
//! `0053f570`, `0053fa10`), the border lines (`0053fa40`), the distant
//! texture blending (`005400e0`), the `LoadedLandData` constructor
//! (`005405a0`), and the small containers the file ends with: the grass
//! map (`NiTPointerMap` and its base), the `NiTArray` of the additional
//! geometry data blocks and the Havok byte array of the MOPP code. No
//! function of the queue is left.
//!
//! Two translations of the first session were corrected in the third:
//! `InitializeStatics` passed the default texture set to `00726070` (a
//! getter of the word at +4 of the data handler, which takes no argument)
//! and called `00510130` without it; the set is the argument of `00510130`.
//!
//! ## Layout (PC build)
//!
//! `TESObjectLAND` is 0x2C bytes on the PC (`TESForm` 0x18, the
//! `TESChildCell` vtable at +0x18, then the Xbox fields from `Data` on); the
//! Xbox PDB has them 0x10 higher. `LoadedLandData` (0xA4 bytes) holds the
//! four quadrants of the land: `ppMesh`, `ppVertices`, `ppNormals`,
//! `ppColorsA` and `ppNormalsSet` point to arrays of four pointers (one per
//! quadrant, 0x121 = 17 x 17 entries each); the texture, percent and grass
//! arrays have four entries on the PC where the PDB has sixteen.
//!
//! ## Chunks
//!
//! The land record chunks are `DATA` (flags), `VHGT` (heights), `VNML`
//! (normals), `VCLR` (vertex colours), `MPCD` (Havok MOPP code), `BTXT` (base
//! texture of a quadrant), `ATXT` (additional texture layer) and `VTXT`
//! (the layer's per-vertex opacity). A chunk holds a 33 x 33 grid; each
//! quadrant takes the 17 x 17 block that starts at
//! `(quadrant / 2) * 16 * 33 + (quadrant % 2) * 16`. `Load` (the record
//! loader) and `LoadVerticesIntoArrays` (the loader that finds the record
//! in the file again) have the same chunk code inlined; the translations
//! share it in the private functions below (`read_height_chunk` and the
//! others) and keep what differs (the flags that gate a chunk, the
//! messages, the opacity scale of `VTXT`, the `DATA` handling) in the two
//! loaders.
//!
//! ## Helpers the translations call by address
//!
//! `00559450` reads a smart-pointer slot (`*this`); `00401000` and `00401030`
//! are the allocator; `00401460` is `memcpy` and `00403d30` `memset`;
//! `00416870`, `00452dc0` and `00414430` construct an `NiPoint3`, `NiPoint2`
//! and `NiColorA` and return it; `00404eb0`/`00404ee0` are the scope guard
//! that tags allocations with the source line; `007af430` is the folded
//! getter of the `+0x20` field (`pParentCell` here); `0084e3a0` reads the
//! form id (`+0x0C`, and the count of an `NiTPointerMap`).
//!
//! x87: the code stores `float`s; the translations compute in `f64` where
//! the code does and round to `f32` at each store (the sum of two floats is
//! the same either way).
//!
//! Not translated: the C++ exception-unwinding frames (`FS:[0]` chains) of
//! the constructor, destructor, `InitializeStatics`, `Load`, `LoadVertices`,
//! `LoadVerticesIntoArrays`, `MakeLandTriStrips` and the `LoadedLandData`
//! destructor body, and the stack-cookie checks.

#[allow(unused_imports)]
use crate::prelude::*;

// ---- Data of the exe ------------------------------------------------------

/// `TESObjectLAND`'s vtable and the `TESChildCell` vtable at +0x18, and the
/// vtable the `TESChildCell` constructor stores before the land overwrites
/// it (a class with a pure virtual function).
pub(crate) const LAND_VTABLE: u32 = 0x0102_dcd4;
pub(crate) const CHILD_CELL_VTABLE: u32 = 0x0102_dccc;
pub(crate) const CHILD_CELL_ABSTRACT_VTABLE: u32 = 0x0102_de10;

/// `TESObjectLAND`'s statics (Xbox PDB names): the triangle list, colour,
/// normal and texture coordinate blocks every land shares, the "normal set"
/// flags, `bStaticsDefined`, `iLANDsinuse`, the default land texture and its
/// texture set, and the default texturing property.
pub(crate) const DEFAULT_TRIANGLE_LIST: u32 = 0x011c_9ee8;
pub(crate) const DEFAULT_COLORS: u32 = 0x011c_9eec;
pub(crate) const DEFAULT_NORMALS: u32 = 0x011c_9ef0;
pub(crate) const DEFAULT_TEXTURE_COORDINATES: u32 = 0x011c_9ef4;
pub(crate) const DEFAULT_NORMAL_SET: u32 = 0x011c_9ef8;
pub(crate) const STATICS_DEFINED: u32 = 0x011c_9efc;
pub(crate) const LANDS_IN_USE: u32 = 0x011c_9f00;
pub(crate) const DEFAULT_LAND_TEXTURE: u32 = 0x011c_9f04;
pub(crate) const DEFAULT_TEXTURE_SET: u32 = 0x011c_9f08;
/// `NiPointer<NiTexturingProperty>` `spDefTexProp`; the INI setting
/// `sDefaultLandDiffuseTexture` follows it, `sDefaultLandNormalTexture` is at
/// `011c9f8c`.
pub(crate) const DEFAULT_TEXTURING_PROPERTY: u32 = 0x011c_a000;
pub(crate) const DEFAULT_DIFFUSE_TEXTURE_SETTING: u32 = 0x011c_a004;
pub(crate) const DEFAULT_NORMAL_TEXTURE_SETTING: u32 = 0x011c_9f8c;
/// The INI setting `fLandTextureTilingMult` (a float setting).
pub(crate) const TEXTURE_TILING_SETTING: u32 = 0x011c_9f2c;
/// `ppDefVertexBLOCK[4]`, `pXOffsetBLOCK[4]`, `pYOffsetBLOCK[4]`.
pub(crate) const DEFAULT_VERTEX_BLOCKS: u32 = 0x011c_9ed4;
pub(crate) const X_OFFSETS: u32 = 0x011c_9ec4;
pub(crate) const Y_OFFSETS: u32 = 0x011c_9eb4;
/// `pTriStripListBLOCK`: the triangle strip of a quadrant (`u16[1021]`,
/// 0x7FA bytes).
pub(crate) const TRIANGLE_STRIP_LIST: u32 = 0x0118_ae90;
/// The data handler singleton (`011c3f2c`): `InitializeStatics` reads the word at its +4 (`00726070`) and hands it the default texture set (`00510130`); the neighbour search asks it for cells.
const DATA_HANDLER: u32 = 0x011c_3f2c;
/// The default colour (four floats) the colour reader returns when the land
/// has no vertex arrays.
const DEFAULT_COLOR_WORDS: u32 = 0x011a_9be0;

/// The source file name passed to the allocation-tag scope guard, and the
/// guard's tag and size.
const SOURCE_FILE: u32 = 0x0102_de60;
const GUARD_TAG: u32 = 0x1b;
const GUARD_SIZE: u32 = 4;

/// Strings: `"Landscape\\%s"`, `"Block (%i, %i)"`, the INI warning,
/// `"UNKNOWN"` (the file name when a land has no file) and the messages of
/// `LoadVerticesIntoArrays`.
const LANDSCAPE_PATH_FORMAT: u32 = 0x0102_de50;
const BLOCK_NAME_FORMAT: u32 = 0x0102_e1c8;
const TILING_WARNING: u32 = 0x0102_de14;
const UNKNOWN_FILE_NAME: u32 = 0x0101_5890;
/// `"CELLS: Failed to load landscape data for LAND (%08X) in Cell (%i, %i)
/// from file '%s'."`.
const LAND_LOAD_FAILED: u32 = 0x0102_e170;
/// `"CELLS: Land for cell (%i, %i) in file '%s' does not contain Normal
/// Data."`.
const NO_NORMAL_DATA: u32 = 0x0102_dfe8;

/// Constants the code reads from the exe's data: the floats `FLT_MAX`,
/// `-FLT_MAX`, `16.0`, `1024.0`, `-2048.0`, and the doubles `0.0`, `1.0`,
/// `0.5`, `2.0`, `4.0`, `100.0`, `127.0`, `255.0`, `1024.0` and `2048.0`.
const LARGEST_F32: u32 = 0x0101_6970;
const LOWEST_F32: u32 = 0x0101_5f5c;
const SIXTEEN_F32: u32 = 0x0101_e57c;
const F32_1024: u32 = 0x0102_36e0;
const F32_MINUS_2048: u32 = 0x0102_dcc4;
const F64_ZERO: u32 = 0x0101_2060;
const F64_ONE: u32 = 0x0101_2070;
const F64_HALF: u32 = 0x0101_1588;
const F64_TWO: u32 = 0x0101_1590;
const F64_FOUR: u32 = 0x0101_db80;
const F64_HUNDRED: u32 = 0x0101_7a40;
const F64_127: u32 = 0x0102_dfe0;
const F64_255: u32 = 0x0101_e568;
const F64_1024: u32 = 0x0101_ecb8;
const F64_2048: u32 = 0x0101_6968;

// ---- Callees outside this file ---------------------------------------------

/// The allocator (`MemoryManager::Allocate`) and its `Deallocate`, `memcpy`
/// (`__cdecl(destination, source, size)`), `memset`
/// (`__cdecl(destination, value, size)`) and the vector constructor
/// iterator (`(array, element size, count, constructor)`).
const ALLOCATE: u32 = 0x0040_1000;
const DEALLOCATE: u32 = 0x0040_1030;
const MEMORY_COPY: u32 = 0x0040_1460;
const MEMORY_SET: u32 = 0x0040_3d30;
const VECTOR_CONSTRUCT: u32 = 0x0040_1050;
/// The Gamebryo allocator: `NiAlloc(size)` and the object allocator.
const NI_ALLOCATE: u32 = 0x00aa_1070;
const NI_ALLOCATE_OBJECT: u32 = 0x00aa_13e0;
/// `sprintf(buffer, size, format, ...)` and the message log
/// (`(format, ...)`).
const FORMAT_STRING: u32 = 0x0040_6d00;
const LOG_MESSAGE: u32 = 0x005b_5e40;
/// The allocation-tag scope guard: `__thiscall(guard, tag, 1, file, line)`
/// and its destructor.
const SCOPE_GUARD_ENTER: u32 = 0x0040_4eb0;
const SCOPE_GUARD_LEAVE: u32 = 0x0040_4ee0;
/// `__RTDynamicCast` and the type descriptors of `TESForm`,
/// `TESObjectLAND` and `TESLandTexture`.
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const RTTI_TES_FORM: u32 = 0x0118_3028;
const RTTI_TES_OBJECT_LAND: u32 = 0x0118_ac10;
const RTTI_TES_LAND_TEXTURE: u32 = 0x0118_6350;

/// Constructors of the vector types: `NiPoint3(x, y, z)`, `NiPoint2(x, y)`,
/// `NiColorA(r, g, b, a)`, each returning its `this`, and the default
/// constructors the vector constructor iterator calls.
const NI_POINT3_CONSTRUCT: u32 = 0x0041_6870;
const NI_POINT2_CONSTRUCT: u32 = 0x0045_2dc0;
const NI_COLOR_CONSTRUCT: u32 = 0x0041_4430;
const NI_POINT3_DEFAULT_CONSTRUCT: u32 = 0x0068_15c0;
const NI_COLOR_DEFAULT_CONSTRUCT: u32 = 0x004a_7800;
/// Reads the smart-pointer slot at `this` (`*this`).
const SMART_POINTER_GET: u32 = 0x0055_9450;
/// `NiPointer<NiTexturingProperty>::operator=(pointer)`, and the
/// `NiPointer<QueuedFile>` assignment.
const TEXTURING_PROPERTY_SET: u32 = 0x0066_b0d0;
const QUEUED_FILE_POINTER_SET: u32 = 0x006f_74f0;
/// Releases the `NiPointer` at `this`, and constructs one from a pointer.
const SMART_POINTER_RELEASE: u32 = 0x0045_cec0;
const SMART_POINTER_CONSTRUCT: u32 = 0x0063_3c90;
/// `BGSTextureSet::BGSTextureSet`, `BGSTextureSet::SetTexturePath(index,
/// path)`, `TESLandTexture::TESLandTexture`, `TESLandTexture::SetTextureSet`
/// (`this` = texture, texture set), the getter of the word at +4 of the data
/// handler (`00726070`, no argument) and the call on that word that
/// receives the default texture set (`00510130`, one argument).
const TEXTURE_SET_CONSTRUCT: u32 = 0x0059_22e0;
const TEXTURE_SET_SET_PATH: u32 = 0x0059_2cc0;
const LAND_TEXTURE_CONSTRUCT: u32 = 0x0054_0c50;
const LAND_TEXTURE_SET_TEXTURE_SET: u32 = 0x0098_4f60;
const DATA_HANDLER_WORD_AT_4: u32 = 0x0072_6070;
const DATA_HANDLER_ADD_TEXTURE_SET: u32 = 0x0051_0130;
/// The string value of a string setting, and the address of the value of a
/// float setting.
const SETTING_GET_STRING: u32 = 0x0040_3df0;
const SETTING_GET_FLOAT_ADDRESS: u32 = 0x0040_3e20;
/// `NiTexturingProperty::Map::Map(this, texture, 0, 3, 5, 0)`, the
/// `NiTexturingProperty` constructor, the element address `(this, index)` and
/// element setter `(this, index, &value)` of the property's map table, and
/// `(this, value, 0xe, 1)` (`00439360`), which the default property gets with
/// value 2.
const TEXTURE_MAP_CONSTRUCT: u32 = 0x00a6_9e00;
const TEXTURING_PROPERTY_CONSTRUCT: u32 = 0x00a6_aa40;
const ARRAY_SET_AT: u32 = 0x0096_ae90;
const ARRAY_ELEMENT_ADDRESS: u32 = 0x0087_7a30;
const PROPERTY_SET_MODE: u32 = 0x0043_9360;
/// The form's constructor and destructors, `SetFormType`, `LoadForm`, the
/// form-id lookup, `AddCompileIndex(&id, file)`, `GetFile(index)`.
const FORM_CONSTRUCT: u32 = 0x0048_3370;
const FORM_DESTRUCT: u32 = 0x0048_3630;
const FORM_BASE_DESTRUCT: u32 = 0x0048_3710;
const FORM_SET_FORM_TYPE: u32 = 0x004f_15a0;
const FORM_ID: u32 = 0x0084_e3a0;
const FORM_LOAD: u32 = 0x0048_5110;
const FORM_LOOKUP_BY_ID: u32 = 0x0048_39c0;
const FORM_ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
const FORM_GET_FILE: u32 = 0x0048_4e60;
const NI_POINTER_SLOT_CONSTRUCT: u32 = 0x0052_8cb0;
const QUEUED_FILE_POINTER_DESTRUCT: u32 = 0x0044_cbf0;
/// The parent cell getter (`+0x20`) and the cell's accessors.
const PARENT_CELL: u32 = 0x007a_f430;
const CELL_GET_DATA_X: u32 = 0x0054_4c30;
const CELL_GET_DATA_Y: u32 = 0x0054_4c60;
const CELL_GET_WORLD_SPACE: u32 = 0x0054_ddd0;
const CELL_GET_3D: u32 = 0x0054_5cb0;
/// Tests bit `quadrant` of a byte of the cell's data (`005445d0` gives the
/// data, the byte is at `+8`), `(cell, quadrant) -> bool`.
const CELL_QUADRANT_TEST: u32 = 0x0054_4590;
const CELL_GET_NODE: u32 = 0x0045_c9a0;
const WORLD_SPACE_HAS_LAND: u32 = 0x0058_6390;
const WORLD_SPACE_FIND_LAND_DATA: u32 = 0x0058_5ee0;
/// `TESFile` methods.
const FILE_GET_TES_FORM: u32 = 0x0047_2660;
const FILE_GET_CHUNK: u32 = 0x0047_26b0;
const FILE_NEXT_CHUNK: u32 = 0x0047_26f0;
const FILE_GET_CHUNK_DATA: u32 = 0x0047_2890;
const FILE_GET_CHUNK_SIZE: u32 = 0x0040_1660;
const FILE_NEEDS_SWAP: u32 = 0x0040_1680;
const FILE_GET_ACTIVE: u32 = 0x0047_1d60;
const FILE_GET_NAME: u32 = 0x0089_1170;
const FILE_GET_THREAD_SAFE_FILE: u32 = 0x0047_39b0;
const FILE_FIND_FORM: u32 = 0x0047_34d0;
const FILE_SET_OFFSET: u32 = 0x0047_23a0;
const FILE_RECORD_OFFSET: u32 = 0x0046_7bb0;
/// Byte swap of the 32-bit word at `this`.
const SWAP_WORD: u32 = 0x0050_3210;
/// Whether the loading menu is up, the warning counter (`(enable)`), the
/// current thread's "loading master files" flag.
const LOADING_MENU_VISIBLE: u32 = 0x0070_5e80;
const DISABLE_WARNINGS: u32 = 0x0043_b2b0;
const LOADING_MASTER_FILE: u32 = 0x0055_1680;
/// `float` to integer conversion (`__cdecl(float) -> int`), and the
/// normalization of the `NiPoint3` at `this`.
const FLOAT_TO_INT: u32 = 0x0040_6d90;
const NI_POINT3_NORMALIZE: u32 = 0x004a_0c10;
/// Functions of the rest of this unit that later sessions translate: the
/// default height of a land without vertex data, "has vertex arrays", the
/// "vertices loaded" flag test and setter, the allocator of the loaded
/// data (`this`, source land), the setter of a vertex's layer opacity, the
/// flush of the warnings, and the Havok MOPP builder.
const LAND_DEFAULT_HEIGHT: u32 = 0x0053_a550;
const LAND_HAS_VERTEX_ARRAYS: u32 = 0x0045_cb90;
const LAND_IS_LOADED: u32 = 0x0053_94a0;
const LAND_SET_LOADED: u32 = 0x0053_94c0;
const LAND_ALLOCATE_DATA: u32 = 0x0053_9500;
const LAND_SET_VERTEX_OPACITY: u32 = 0x0053_a8a0;
const LAND_FLUSH_WARNINGS: u32 = 0x0053_a940;
const LAND_BUILD_MOPP: u32 = 0x0053_8c00;
/// What the quadrant meshes builder and the unload call in the rest of the
/// unit: the release of the border lines, the height extents of a quadrant
/// `(this, &out, quadrant)` (min, max), the three builds run after the
/// Havok bodies are made, the strips' bound (the object at `+0xb8`, which
/// `NiBound::ComputeFromData` is run on), the cell's MOPP holder (`cell +
/// 0x28` run through `0041bb10`), the check on the cell's reference list
/// (`!ListIsEnd(cell + 0xac)`), the render task of a scheduler (`+0x78`)
/// and the constructor of a local the builder keeps (`+8` = 0, `+0xc` =
/// 1.0).
const LAND_RELEASE_BORDER: u32 = 0x0053_7eb0;
const LAND_QUADRANT_EXTENTS: u32 = 0x0053_f390;
const LAND_BUILD_FOLLOW_UPS: [u32; 3] = [0x0053_bc10, 0x0053_aeb0, 0x0053_9960];
const LAND_BOUND_SOURCE: u32 = 0x0053_7b10;
const LAND_CELL_MOPP: u32 = 0x0053_7b30;
const CELL_REFERENCE_LIST_CHECK: u32 = 0x0053_7b50;
const LAND_RENDER_TASK: u32 = 0x0053_7bd0;
const LAND_MOPP_LOCAL_CONSTRUCT: u32 = 0x0053_7b80;
/// The float getter of `fBaseHeight` (`0049db00`: the loaded data's, 0 without), the renderer getter, the form lookup
/// `(this, 0, id)` and the two byte swaps `(pointer, 0)`.
const LAND_BASE_HEIGHT: u32 = 0x0049_db00;
const RENDERER_GET: u32 = 0x0043_c4b0;
const FORM_LOOKUP: u32 = 0x0048_67a0;
const SWAP_BYTES_32: u32 = 0x0040_1080;
const SWAP_BYTES_16: u32 = 0x0040_7a90;
/// `NiGeometry` and `NiGeometryData` calls. The node's data accessors:
/// `(this, 0)` gives the geometry of mesh node, then the vertex, normal and
/// colour arrays of a geometry.
const GEOMETRY_GET_MODEL_DATA: u32 = 0x0043_b4a0;
const GEOMETRY_GET_POSITIONS: u32 = 0x0049_ec60;
const GEOMETRY_GET_NORMALS: u32 = 0x004a_8030;
const GEOMETRY_GET_COLORS: u32 = 0x004a_8050;
const GEOMETRY_GET_DATA: u32 = 0x0054_95f0;
const GEOMETRY_DATA_GET_OWNER: u32 = 0x004a_8a90;
/// The lock-supported slot of the owner (`0x8c`), the geometry data's
/// `Begin(1)` and the position and colour locks, and `End`.
const OWNER_CAN_LOCK_SLOT: u32 = 0x8c;
const GEOMETRY_DATA_BEGIN: u32 = 0x00a6_7360;
const GEOMETRY_DATA_LOCK_POSITIONS: u32 = 0x00a6_75b0;
const GEOMETRY_DATA_LOCK_COLORS: u32 = 0x00a6_7660;
const GEOMETRY_DATA_END: u32 = 0x00a6_73d0;
/// The vertex lock (`{base, stride, packed}`): its constructor and the index
/// check of its readers.
const LOCK_CONSTRUCT: u32 = 0x0054_0720;
const LOCK_CHECK_INDEX: u32 = 0x0045_34f0;
/// Scene node calls of the mesh builder: the culled flag getter and setter,
/// `AttachProperty`, `AttachChild` (virtual slot `0xf8`), `Update`,
/// `UpdateProperties`, and the update data constructor.
const NODE_GET_CULLED: u32 = 0x0045_6610;
const NODE_SET_CULLED: u32 = 0x0045_0f90;
const NODE_ATTACH_PROPERTY: u32 = 0x0043_9410;
const NODE_ATTACH_CHILD_SLOT: u32 = 0xf8;
const NODE_UPDATE: u32 = 0x00a5_9c60;
const NODE_UPDATE_PROPERTIES: u32 = 0x00a5_a040;
const UPDATE_DATA_CONSTRUCT: u32 = 0x0043_d410;
/// `NiTriStrips::NiTriStrips`, its translate setter, its vertex count, its
/// bound computation (`(bound, count, positions)`), its model data
/// `MarkAsChanged`, `SetConsistency`, the two flag setters and the three
/// mesh node accessors.
const TRI_STRIPS_CONSTRUCT: u32 = 0x00a7_1c40;
const NODE_SET_TRANSLATE: u32 = 0x0044_0460;
const TRI_STRIPS_VERTEX_COUNT: u32 = 0x0045_6650;
const BOUND_COMPUTE_FROM_DATA: u32 = 0x00a7_ee30;
const GEOMETRY_DATA_MARK_CHANGED: u32 = 0x00a6_7090;
const GEOMETRY_DATA_SET_CONSISTENCY: u32 = 0x00a6_7050;
const GEOMETRY_DATA_SET_KEEP_FLAGS: u32 = 0x0044_10f0;
const GEOMETRY_DATA_SET_COMPRESS_FLAGS: u32 = 0x0044_10d0;
const FACE_GEN_NODE_GET_ANIMATION_DATA: u32 = 0x0066_29f0;
const ANIMATION_DATA_GET_ROOT: u32 = 0x0043_b230;
const ROOT_SET_LOCAL_TRANSLATE: u32 = 0x0043_9680;
const ROOT_SET_WORLD_TRANSLATE_SLOT: u32 = 0xb8;
const OBJECT_SET_NAME: u32 = 0x00a5_b950;
const FIXED_STRING_CONSTRUCT: u32 = 0x0043_8170;
const FIXED_STRING_DESTRUCT: u32 = 0x0043_81b0;
/// `CellMopp::Create(this, strips, 4, base height)`, `CellMopp` preparation,
/// the geometry's scheduler, the scheduler's saved acquire object, the
/// renderer's queue slot (`0xec`) and the call after it.
const CELL_MOPP_CREATE: u32 = 0x0062_1f60;
const CELL_MOPP_PREPARE: u32 = 0x0062_1e60;
const GEOMETRY_GET_SCHEDULER: u32 = 0x0040_30b0;
const SCHEDULER_GET_SAVED_ACQUIRE: u32 = 0x008d_8520;
const RENDERER_QUEUE_SLOT: u32 = 0xec;
const RENDERER_AFTER_QUEUE: u32 = 0x00e7_4120;
/// `~LoadedLandData`'s pieces: the vector destructor iterator, the
/// destructor of one grass map, `hkReferencedObject::removeReference` and
/// `addReference`, the `NiTPointerMap` calls (first position, next, remove
/// all).
const VECTOR_DESTRUCT_ITERATOR: u32 = 0x00ec_5fce;
const GRASS_MAP_DESTRUCT: u32 = 0x0054_09b0;
const HAVOK_REMOVE_REFERENCE: u32 = 0x00c9_05b0;
const HAVOK_ADD_REFERENCE: u32 = 0x00c9_0510;
const MAP_FIRST_POSITION: u32 = 0x004b_9ba0;
const MAP_GET_NEXT: u32 = 0x006b_7f20;
const MAP_REMOVE_ALL: u32 = 0x0043_8af0;
/// `QueuedFile::QueuedFile(this, 5)` and `TESLandTexture::QueueTexture(
/// texture, 5, queued file)`.
const QUEUED_FILE_CONSTRUCT: u32 = 0x00c3_c590;
const LAND_TEXTURE_QUEUE: u32 = 0x0054_1540;
/// The render task priority the loaders pass.
const QUEUE_PRIORITY: u32 = 5;

// ---- Second session: data and callees ---------------------------------------

/// The word every save reference record starts with (the global at
/// `01187020`), and the type descriptors of
/// `TESChildCell`, `TESObjectREFR` and `TESObjectCELL`.
const SAVE_REFERENCE_MARKER: u32 = 0x0118_7020;
const RTTI_TES_CHILD_CELL: u32 = 0x0118_ac2c;
const RTTI_TES_OBJECT_REFR: u32 = 0x0118_41cc;
const RTTI_TES_OBJECT_CELL: u32 = 0x0118_3fb4;
/// The global word `00539f40` returns (the key under which the percent
/// extra data is attached to a quadrant mesh).
const EXTRA_DATA_KEY: u32 = 0x011f_94bc;
/// `"MASTERFILE: Error saving land height Data for cell (%i, %i). Error
/// correction attempted."` plus CR LF.
const SAVE_HEIGHT_ERROR: u32 = 0x0102_e308;

/// `NiAdditionalGeometryData` (0x2C bytes) and `BSPackedAdditionalGeometryData`
/// (0x34 bytes): vtables, `GetRTTI` results, the `NiObject` constructor,
/// the destructor body of the first, `NiFree(pointer, size)` and the
/// constructors of the members at +0x1c.
const ADDITIONAL_GEOMETRY_DATA_VTABLE: u32 = 0x0102_e274;
const PACKED_ADDITIONAL_GEOMETRY_DATA_VTABLE: u32 = 0x0102_e1dc;
const ADDITIONAL_GEOMETRY_DATA_RTTI: u32 = 0x011f_4a30;
const PACKED_ADDITIONAL_GEOMETRY_DATA_RTTI: u32 = 0x011f_4aac;
const NI_OBJECT_CONSTRUCT: u32 = 0x00a5_d3a0;
const ADDITIONAL_GEOMETRY_DATA_DESTRUCT: u32 = 0x00a7_3080;
const NI_FREE: u32 = 0x00aa_1460;
/// Called with `this + 0x1c` and `(0, 1)` by the constructor; the member's
/// other constructor (`this + 0x1c`) and the destructor `00540800` are
/// later functions of this unit.
const MEMBER_CONSTRUCT: u32 = 0x0054_0830;
const MEMBER_CONSTRUCT_AGAIN: u32 = 0x005e_03d0;
const MEMBER_DESTRUCT: u32 = 0x0054_0800;

/// The Havok MOPP code object (0x30 bytes): the three vtables of its base
/// classes (a base object, a referenced object, the MOPP code), the
/// allocator it comes from, the constructors of its members at +0x10, +0x20
/// and +0x2c, the call that sizes the array at +0x20, the call that
/// finishes the member at +0x2c (with 2), the destructor of the array at
/// +0x20, and the Havok memory router calls of the release function.
const HAVOK_BASE_OBJECT_VTABLE: u32 = 0x0102_e388;
const HAVOK_REFERENCED_OBJECT_VTABLE: u32 = 0x0102_e378;
const HAVOK_MOPP_CODE_VTABLE: u32 = 0x0102_e368;
const MOPP_ALLOCATE: u32 = 0x0056_d280;
const MOPP_MEMBER_CONSTRUCT_A: u32 = 0x0062_40d0;
const MOPP_MEMBER_CONSTRUCT_B: u32 = 0x0062_99a0;
const MOPP_ARRAY_SET_SIZE: u32 = 0x0054_0750;
const MOPP_MEMBER_FINISH: u32 = 0x0054_07b0;
const MOPP_ARRAY_DESTRUCT: u32 = 0x0054_08e0;
const HAVOK_MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `BaseProcess::GetCurrentProcedureIndex` (Xbox PDB name of the folded
/// body at `0044edb0`): here it turns the router into the allocator object.
const HAVOK_ALLOCATOR_OF_ROUTER: u32 = 0x0044_edb0;

/// Calls of the release function of the loaded data (`00537eb0`): the
/// entry `index` of the table at `011f91c8` (`00450b80`), a cell's
/// physics object (`004543c0`), "the slot does not hold this value"
/// (`0052aa80`, `(slot, value)`), the call that hands the rigid body to
/// the physics object (`005380d0`), `ShadowSceneNode::RemoveObject`
/// (Xbox PDB, `(manager, object)`) and `+0x18` of an object (`009611e0`).
const TABLE_ENTRY: u32 = 0x0045_0b80;
const CELL_PHYSICS_OBJECT: u32 = 0x0045_43c0;
const SLOT_HOLDS_OTHER_THAN: u32 = 0x0052_aa80;
const PHYSICS_OBJECT_ADD_BODY: u32 = 0x0053_80d0;
const SHADOW_SCENE_NODE_REMOVE_OBJECT: u32 = 0x00b5_b1c0;
const OBJECT_FIELD_0X18: u32 = 0x0096_11e0;
/// Slots of the quadrant node (`0xf0`, called with 0) and of the border
/// object's owner (`0xe8`).
const NODE_DETACH_SLOT: u32 = 0xf0;
const BORDER_OWNER_SLOT: u32 = 0xe8;

/// The getters `00537b10` to `00537bd0` end in: `00460140`, `0041bb10`,
/// `008256d0` and `004b5020`.
const GETTER_00460140: u32 = 0x0046_0140;
const GETTER_0041BB10: u32 = 0x0041_bb10;
const LIST_CHECK: u32 = 0x0082_56d0;
const BASE_CONSTRUCT_004B5020: u32 = 0x004b_5020;

/// Form type and save helpers: the form's type byte (`00401170`), "the
/// type is one of the reference types" (`005548a0`),
/// `TESObjectREFR::GetRefPersists` (Xbox PDB), "the id belongs to the
/// cell's file" (`00485be0`, `(cell, id)`), the cell's reference id of a
/// save (`00544210`), and a form method of `tesform.cpp` that takes a flag
/// byte (`00484730`).
const FORM_TYPE: u32 = 0x0040_1170;
const TYPE_IS_REFERENCE_KIND: u32 = 0x0055_48a0;
const GET_REF_PERSISTS: u32 = 0x0056_53d0;
const FORM_ID_IN_CELL_FILE: u32 = 0x0048_5be0;
const CELL_SAVE_ID: u32 = 0x0054_4210;
const FORM_FLAG_SETTER: u32 = 0x0048_4730;
/// Slots of the cell (`0x38` and `0x3c` of its vtable, `0x110`) and of the
/// parent-cell part of the land (`0`).
const CELL_SLOT_REFERENCE_OWNED: u32 = 0x38;
const CELL_SLOT_REFERENCE_CHECK: u32 = 0x3c;
const CELL_SLOT_SAVE_CHECK: u32 = 0x110;
const CHILD_CELL_PARENT_SLOT: u32 = 0;
/// Slot `0xc8` of the parent cell, called with 1 by `fn_00539010`.
const CELL_SLOT_MARK: u32 = 0xc8;

/// The save writer: `TESForm::StartForm`, `TESForm::CloseForm`,
/// `TESForm::CompressSaveBuffer` (Xbox PDB), `TESForm::__AddChunkData(tag,
/// data, size)`, `TESForm::AddChunkArray(tag, data, size)`, the
/// big-endian flag, the load-if-needed call at the start of `Save`, the
/// opacity of layer `l` at vertex `i` of quadrant `q` (`(this, q, i, l)`,
/// a float), `abs`, `_ftol2` (the double is in ST0) and the form id getter.
const FORM_START: u32 = 0x0048_55a0;
const FORM_CLOSE: u32 = 0x0048_5680;
const FORM_COMPRESS_SAVE_BUFFER: u32 = 0x0048_3d70;
const FORM_ADD_CHUNK_DATA: u32 = 0x0048_5990;
const FORM_ADD_CHUNK_ARRAY: u32 = 0x0048_56f0;
const IS_BIG_ENDIAN: u32 = 0x0040_1500;
const LAND_LOAD_FOR_SAVE: u32 = 0x0053_db20;
const LAND_LAYER_OPACITY: u32 = 0x0053_a830;
const INT_ABS: u32 = 0x00ec_7d40;
const FTOL2: u32 = 0x00ec_62c0;

/// The constructor of `LoadedLandData` (`this`), and the shader property
/// calls of the material builder (`fn_00539960`): the property
/// (`BSShaderPPLightingProperty`, 0x104 bytes) constructor, the setter of
/// the word at +0x58, the texture-set setter `(this, slot, set)`, the
/// slot flags setter `(this, 10 words)`, `BGSTextureSet::GetAsShaderTextureSet`
/// (Xbox PDB), the flag byte of a land texture (`+0x1f`), the first child
/// of a node (`0045bc00`, `(this, 0)`), `BSShaderManager::CreateTangentSpaceSimple`
/// and `BSShaderManager::PrepareObject` (Xbox PDB), `NiObjectNET::AddExtraData`
/// (Xbox PDB, `(this, key, data)`), the percent extra data constructor
/// (`00539f50`, `(this, 0x2420, first block)`) and the geometry data's
/// setter `00a67260`.
const LOADED_LAND_DATA_CONSTRUCT: u32 = 0x0054_05a0;
const SHADER_PROPERTY_CONSTRUCT: u32 = 0x00b6_6f50;
const SHADER_PROPERTY_SET_WORD_0X58: u32 = 0x005a_8060;
const SHADER_PROPERTY_SET_TEXTURE_SET: u32 = 0x00b6_8660;
const SHADER_PROPERTY_SET_FLAGS: u32 = 0x00b6_6640;
const TEXTURE_SET_AS_SHADER_SET: u32 = 0x0059_2cf0;
const LAND_TEXTURE_FLAG_BYTE: u32 = 0x0054_1590;
const NODE_FIRST_GEOMETRY: u32 = 0x0045_bc00;
const CREATE_TANGENT_SPACE_SIMPLE: u32 = 0x00b5_4b60;
const PREPARE_OBJECT: u32 = 0x00b5_7e30;
const OBJECT_ADD_EXTRA_DATA: u32 = 0x00a5_bc40;
const PERCENT_EXTRA_DATA_CONSTRUCT: u32 = 0x0053_9f50;
const GEOMETRY_DATA_SET_SHARED: u32 = 0x00a6_7260;
/// Allocation size of the extra data and of the property, and the slot of
/// the property's tangent space query (`0xd4`).
const EXTRA_DATA_SIZE: u32 = 0x14;
const SHADER_PROPERTY_SIZE: u32 = 0x104;
const PROPERTY_SLOT_TANGENT_SPACE: u32 = 0xd4;

// ---- Layouts ---------------------------------------------------------------

layout! {
    /// `TESObjectLAND` (Xbox PDB), 0x2C bytes on the PC: `TESForm` (0x18),
    /// the `TESChildCell` vtable at +0x18, then the fields below.
    pub struct TESObjectLAND: 0x2c {
        /// `Data` (Xbox PDB, `OBJ_LAND`): the land's flags. Bit 0x01: height
        /// and normal data present; 0x02: vertex colours; 0x04: textures; 0x08: the
        /// vertex data is loaded;
        /// 0x10: set by the editor; 0x400: the data is read from the world
        /// space's land file; 0x800: the Havok MOPP code was built.
        0x1C Data: u32,
        /// `pParentCell` (Xbox PDB): the cell the land belongs to.
        0x20 pParentCell: Ptr,
        /// `spQueuedTextures` (Xbox PDB): `NiPointer<QueuedFile>` slot.
        0x24 spQueuedTextures: Ptr,
        /// `pLoadedData` (Xbox PDB): `LoadedLandData*`.
        0x28 pLoadedData: Ptr,
    }

    /// `TESObjectLAND::LoadedLandData` (Xbox PDB), 0xA4 bytes on both builds.
    pub struct LoadedLandData: 0xa4 {
        /// `ppMesh` (Xbox PDB): four `NiNode*`.
        0x00 ppMesh: Ptr,
        /// `ppVertices` (Xbox PDB): four arrays of 0x121 `NiPoint3`.
        0x04 ppVertices: Ptr,
        /// `ppNormals` (Xbox PDB): four arrays of 0x121 `NiPoint3`.
        0x08 ppNormals: Ptr,
        /// `ppColorsA` (Xbox PDB): four arrays of 0x121 `NiColorA`.
        0x0C ppColorsA: Ptr,
        /// `ppNormalsSet` (Xbox PDB).
        0x10 ppNormalsSet: Ptr,
        /// `spBorder` (Xbox PDB): `NiPointer<NiLines>` slot.
        0x14 spBorder: Ptr,
        /// `HeightExtents.x` (Xbox PDB `NiPoint2`): the lowest vertex height.
        0x18 HeightExtentsMin: f32,
        /// `HeightExtents.y` (Xbox PDB): the highest vertex height.
        0x1C HeightExtentsMax: f32,
        /// `pDefQuadTexture` (Xbox PDB): four `TESLandTexture*`.
        0x20 pDefQuadTexture: Ptr,
        /// `pQuadTextureArray` (Xbox PDB): four arrays of layer textures.
        0x30 pQuadTextureArray: Ptr,
        /// `ppPercentArrays` (Xbox PDB): four arrays of 0x121-entry opacity
        /// blocks, one per layer.
        0x40 ppPercentArrays: Ptr,
        /// `pMoppCode` (Xbox PDB): `hkpMoppCode*`.
        0x50 pMoppCode: Ptr,
        /// `pmGrassMap` (Xbox PDB): four `NiTPointerMap`s (0x10 bytes each).
        0x54 pmGrassMap: Ptr,
        /// `spLandRB` (Xbox PDB): `NiPointer<bhkRigidBody>` slot.
        0x94 spLandRB: Ptr,
        /// `iCellX` (Xbox PDB).
        0x98 iCellX: i32,
        /// `iCellY` (Xbox PDB).
        0x9C iCellY: i32,
        /// `fBaseHeight` (Xbox PDB): the mid height of the land; vertex
        /// heights are stored relative to it.
        0xA0 fBaseHeight: f32,
    }
}

/// The offset of the `TESChildCell` sub-object in the land.
pub(crate) const CHILD_CELL_OFFSET: u32 = 0x18;
/// The offset of the `pmGrassMap` maps (0x10 bytes each, four).
pub(crate) const GRASS_MAP_OFFSET: u32 = 0x54;
/// Quadrants of a land and the vertices of a quadrant (17 x 17).
pub(crate) const QUADRANTS: u32 = 4;
pub(crate) const QUADRANT_VERTICES: u32 = 0x121;
/// Vertices of a chunk's grid (33 x 33).
const CHUNK_VERTICES: usize = 0x441;

/// The four-character tag of a land chunk as the record reader compares it
/// (the bytes in file order read as a little-endian word).
pub(crate) const fn tag(name: &[u8; 4]) -> u32 {
    u32::from_le_bytes(*name)
}

pub(crate) const CHUNK_DATA: u32 = tag(b"DATA");
pub(crate) const CHUNK_HEIGHTS: u32 = tag(b"VHGT");
pub(crate) const CHUNK_NORMALS: u32 = tag(b"VNML");
pub(crate) const CHUNK_COLORS: u32 = tag(b"VCLR");
pub(crate) const CHUNK_MOPP: u32 = tag(b"MPCD");
pub(crate) const CHUNK_BASE_TEXTURE: u32 = tag(b"BTXT");
pub(crate) const CHUNK_ADDITIONAL_TEXTURE: u32 = tag(b"ATXT");
pub(crate) const CHUNK_VERTEX_TEXTURE: u32 = tag(b"VTXT");

/// The land's flag bits (see [`TESObjectLAND::Data`]).
pub(crate) const FLAG_HEIGHTS: u32 = 0x1;
pub(crate) const FLAG_COLORS: u32 = 0x2;
pub(crate) const FLAG_TEXTURES: u32 = 0x4;
pub(crate) const FLAG_LOADED: u32 = 0x8;
pub(crate) const FLAG_EDITED: u32 = 0x10;
pub(crate) const FLAG_FROM_WORLD_SPACE: u32 = 0x400;
pub(crate) const FLAG_MOPP_BUILT: u32 = 0x800;

// ---- Helpers ---------------------------------------------------------------

/// `this->pLoadedData`.
pub(crate) fn loaded_data(e: &Engine, this: Ptr<TESObjectLAND>) -> Ptr<LoadedLandData> {
    e.get(this, TESObjectLAND::pLoadedData).cast()
}

/// Element `index` of an array of pointers.
pub(crate) fn element(e: &Engine, array: Ptr, index: u32) -> u32 {
    e.mem.u32(array.addr().wrapping_add(index.wrapping_mul(4)))
}

/// Reads `count` words at `at`.
fn read_words(e: &Engine, at: u32, count: u32) -> Vec<u32> {
    (0..count)
        .map(|i| e.mem.u32(at.wrapping_add(4 * i)))
        .collect()
}

/// Writes `words` at `at`.
fn write_words(e: &mut Engine, at: u32, words: &[u32]) {
    for (i, word) in words.iter().enumerate() {
        e.mem.set_u32(at.wrapping_add(4 * i as u32), *word);
    }
}

/// Copies `count` words from `from` to `to`.
fn copy_words(e: &mut Engine, from: u32, to: u32, count: u32) {
    let words = read_words(e, from, count);
    write_words(e, to, &words);
}

/// Runs a constructor of a vector type on a temporary of `size` bytes and
/// returns the words of the value it returns a pointer to.
fn construct_words(e: &mut Engine, construct: u32, size: u32, arguments: &[u32]) -> Vec<u32> {
    e.with_stack(size, |e, temporary| {
        let mut words = vec![temporary.addr()];
        words.extend_from_slice(arguments);
        let result = e.call(construct, &words).u32();
        read_words(e, result, size / 4)
    })
}

fn ni_point3(e: &mut Engine, x: f32, y: f32, z: f32) -> Vec<u32> {
    construct_words(e, NI_POINT3_CONSTRUCT, 12, &args![x, y, z])
}

fn ni_point2(e: &mut Engine, x: f32, y: f32) -> Vec<u32> {
    construct_words(e, NI_POINT2_CONSTRUCT, 8, &args![x, y])
}

fn ni_color(e: &mut Engine, r: f32, g: f32, b: f32, a: f32) -> Vec<u32> {
    construct_words(e, NI_COLOR_CONSTRUCT, 16, &args![r, g, b, a])
}

/// The allocation-tag scope guard the loaders run under: entered with the
/// source line, left when `body` returns.
fn scoped<R>(e: &mut Engine, line: u32, body: impl FnOnce(&mut Engine) -> R) -> R {
    e.with_stack(GUARD_SIZE, |e, guard| {
        e.call(
            SCOPE_GUARD_ENTER,
            &args![guard, GUARD_TAG, 1u32, SOURCE_FILE, line],
        );
        let result = body(e);
        e.call(SCOPE_GUARD_LEAVE, &args![guard]);
        result
    })
}

/// A vector of `count` elements built by the vector constructor iterator
/// when the allocation succeeded (0 otherwise).
fn allocate_vector(
    e: &mut Engine,
    size: u32,
    element_size: u32,
    count: u32,
    construct: u32,
) -> u32 {
    let memory = e.call(ALLOCATE, &args![size]).u32();
    if memory != 0 {
        e.call(
            VECTOR_CONSTRUCT,
            &args![memory, element_size, count, construct],
        );
    }
    memory
}

/// A `__RTDynamicCast` from `TESForm`.
fn form_cast(e: &mut Engine, form: u32, target: u32) -> u32 {
    e.call(
        RT_DYNAMIC_CAST,
        &args![form, 0u32, RTTI_TES_FORM, target, 0u32],
    )
    .u32()
}

/// The cell coordinates of the land `(x, y)`, evaluated y first as the
/// message calls do.
fn cell_coordinates(e: &mut Engine, this: Ptr<TESObjectLAND>) -> (i32, i32) {
    let y = fn_00534010(e, this);
    let x = fn_00533fd0(e, this);
    (x, y)
}

// ---- Constructor, destructors and statics ---------------------------------

// Translated from 00533120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectLAND::TESObjectLAND` (Xbox PDB): builds the form and the
/// `TESChildCell` part, stores the two vtables, constructs the queued
/// textures slot, defines the statics when no land has yet, clears the
/// flags, the cell and the loaded data, sets the form type to 0x42 and
/// counts the land in `iLANDsinuse`. Returns `this`.
pub fn tes_object_land_tes_object_land(
    e: &mut Engine,
    this: Ptr<TESObjectLAND>,
) -> Ptr<TESObjectLAND> {
    e.call(FORM_CONSTRUCT, &args![this]);
    fn_00533240(e, this.byte_add(CHILD_CELL_OFFSET));
    e.mem.set_u32(this.addr(), LAND_VTABLE);
    e.mem
        .set_u32(this.addr() + CHILD_CELL_OFFSET, CHILD_CELL_VTABLE);
    e.call(
        NI_POINTER_SLOT_CONSTRUCT,
        &args![this.byte_add(TESObjectLAND::spQueuedTextures.off), 0u32],
    );
    if e.global::<u8>(STATICS_DEFINED) == 0 {
        tes_object_land_initialize_statics(e);
    }
    e.call(
        MEMORY_SET,
        &args![this.byte_add(TESObjectLAND::Data.off), 0u32, 4u32],
    );
    e.set(this, TESObjectLAND::pParentCell, Ptr::NULL);
    e.call(FORM_SET_FORM_TYPE, &args![this, 0x42u32]);
    e.set(this, TESObjectLAND::pLoadedData, Ptr::NULL);
    let in_use = e.global::<u32>(LANDS_IN_USE).wrapping_add(1);
    e.set_global(LANDS_IN_USE, in_use);
    this
}

// Translated from 005331f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESChildCell::GetSaveParentCell` for a land (slot 0 of the vtable
/// `0102dccc`; `this` is the `TESChildCell` part at +0x18): the land's
/// parent cell.
pub fn fn_005331f0(e: &mut Engine, this: Ptr) -> Ptr {
    let land = this.addr().wrapping_sub(CHILD_CELL_OFFSET);
    e.call(PARENT_CELL, &args![land]).ptr()
}

// Translated from 00533210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectLAND::__vecDelDtor` (Xbox PDB, "scalar deleting destructor",
/// slot 0x10): runs the destructor and frees the land when bit 0 of `flags`
/// is set. Returns `this`.
pub fn tes_object_land_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<TESObjectLAND>,
    flags: u32,
) -> Ptr<TESObjectLAND> {
    tes_object_land_destructor(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

// Translated from 00533240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESChildCell::TESChildCell`: stores the vtable of the abstract class.
/// Returns `this`.
pub fn fn_00533240(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), CHILD_CELL_ABSTRACT_VTABLE);
    this
}

// Translated from 00533260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectLAND::~TESObjectLAND` (Xbox PDB): resets the vtables, unloads
/// the vertices, and when this was the last land frees the shared blocks
/// (triangle list, colours, normals, texture coordinates, normal set and
/// the four default vertex blocks), clears `bStaticsDefined`, releases the
/// default texturing property and deletes the default land texture and
/// texture set. Then destroys the queued textures slot and the form.
pub fn tes_object_land_destructor(e: &mut Engine, this: Ptr<TESObjectLAND>) {
    e.mem.set_u32(this.addr(), LAND_VTABLE);
    e.mem
        .set_u32(this.addr() + CHILD_CELL_OFFSET, CHILD_CELL_VTABLE);
    tes_object_land_un_load_vertices(e, this);
    let in_use = e.global::<u32>(LANDS_IN_USE).wrapping_sub(1);
    e.set_global(LANDS_IN_USE, in_use);
    if in_use == 0 {
        for block in [
            DEFAULT_TRIANGLE_LIST,
            DEFAULT_COLORS,
            DEFAULT_NORMALS,
            DEFAULT_TEXTURE_COORDINATES,
            DEFAULT_NORMAL_SET,
        ] {
            let memory = e.global::<u32>(block);
            e.call(DEALLOCATE, &args![memory]);
        }
        for quadrant in 0..QUADRANTS {
            let memory = e.global::<u32>(DEFAULT_VERTEX_BLOCKS + 4 * quadrant);
            e.call(DEALLOCATE, &args![memory]);
        }
        e.set_global(STATICS_DEFINED, 0u8);
        e.call(
            TEXTURING_PROPERTY_SET,
            &args![DEFAULT_TEXTURING_PROPERTY, 0u32],
        );
        for slot in [DEFAULT_LAND_TEXTURE, DEFAULT_TEXTURE_SET] {
            let object = e.global::<u32>(slot);
            if object != 0 {
                // The virtual deleting destructor (slot 4).
                e.vcall(object, 0x10, &args![1u32]);
            }
            e.set_global(slot, 0u32);
        }
    }
    e.call(FORM_BASE_DESTRUCT, &args![this]);
    e.call(
        QUEUED_FILE_POINTER_DESTRUCT,
        &args![this.byte_add(TESObjectLAND::spQueuedTextures.off)],
    );
    e.call(FORM_DESTRUCT, &args![this]);
}

// Translated from 00533420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectLAND::InitializeStatics` (Xbox PDB): builds what all lands
/// share. The default texture set with the two Landscape textures from the
/// INI settings, the default land texture, the default texturing property
/// (diffuse map, mode 2) and the triangle list of one quadrant (16 x 16
/// cells of two triangles, the diagonal alternating with the parity of the
/// cell), then 0x121-entry blocks of white colours (alpha 0), up normals,
/// "set" flags and texture coordinates (scaled by `4 /
/// fLandTextureTilingMult`, 16 when the setting is 0), the offsets of the
/// four quadrants (+-0x800) and four default vertex blocks at height -2048.
pub fn tes_object_land_initialize_statics(e: &mut Engine) {
    scoped(e, 0xc6, |e| {
        // The default texture set and its two texture paths.
        let memory = e.call(NI_ALLOCATE_OBJECT, &args![0xa0u32]).u32();
        let texture_set = if memory == 0 {
            0
        } else {
            e.call(TEXTURE_SET_CONSTRUCT, &args![memory]).u32()
        };
        e.set_global(DEFAULT_TEXTURE_SET, texture_set);
        e.with_stack(0x104, |e, path| {
            for (index, setting) in [
                (0u32, DEFAULT_DIFFUSE_TEXTURE_SETTING),
                (1u32, DEFAULT_NORMAL_TEXTURE_SETTING),
            ] {
                let name = e.call(SETTING_GET_STRING, &args![setting]).u32();
                e.call(
                    FORMAT_STRING,
                    &args![path, 0x104u32, LANDSCAPE_PATH_FORMAT, name],
                );
                let texture_set = e.global::<u32>(DEFAULT_TEXTURE_SET);
                e.call(TEXTURE_SET_SET_PATH, &args![texture_set, index, path]);
            }
        });
        let texture_set = e.global::<u32>(DEFAULT_TEXTURE_SET);
        let handler = e.global::<u32>(DATA_HANDLER);
        let word = e.call(DATA_HANDLER_WORD_AT_4, &args![handler]).u32();
        e.call(DATA_HANDLER_ADD_TEXTURE_SET, &args![word, texture_set]);

        // The default land texture.
        let memory = e.call(ALLOCATE, &args![0x28u32]).u32();
        let land_texture = if memory == 0 {
            0
        } else {
            e.call(LAND_TEXTURE_CONSTRUCT, &args![memory]).u32()
        };
        e.set_global(DEFAULT_LAND_TEXTURE, land_texture);
        let texture_set = e.global::<u32>(DEFAULT_TEXTURE_SET);
        e.call(
            LAND_TEXTURE_SET_TEXTURE_SET,
            &args![land_texture, texture_set],
        );

        // The diffuse texture of the set (slot 0x90 of the base at +0x30)
        // goes into a smart pointer that lives until the end of the function.
        let diffuse = e.mem.alloc(4);
        e.call(SMART_POINTER_CONSTRUCT, &args![diffuse, 0u32]);
        let texture_set = e.global::<u32>(DEFAULT_TEXTURE_SET);
        e.vcall(texture_set + 0x30, 0x90, &args![0u32, diffuse]);
        let memory = e.call(ALLOCATE, &args![0x10u32]).u32();
        let map = if memory == 0 {
            0
        } else {
            let texture = e.call(SMART_POINTER_GET, &args![diffuse]).u32();
            e.call(
                TEXTURE_MAP_CONSTRUCT,
                &args![memory, texture, 0u32, 3u32, 5u32, 0u32],
            )
            .u32()
        };
        let memory = e.call(NI_ALLOCATE_OBJECT, &args![0x30u32]).u32();
        let property = if memory == 0 {
            0
        } else {
            e.call(TEXTURING_PROPERTY_CONSTRUCT, &args![memory]).u32()
        };
        e.call(
            TEXTURING_PROPERTY_SET,
            &args![DEFAULT_TEXTURING_PROPERTY, property],
        );
        let property = e
            .call(SMART_POINTER_GET, &args![DEFAULT_TEXTURING_PROPERTY])
            .u32();
        fn_00533fb0(e, Ptr::new(property), 2);
        let property = e
            .call(SMART_POINTER_GET, &args![DEFAULT_TEXTURING_PROPERTY])
            .u32();
        fn_00533f40(e, Ptr::new(property), Ptr::new(map));

        // The triangle list of a quadrant: 16 x 16 cells, six indices each.
        let triangles = e.call(ALLOCATE, &args![0xc00u32]).u32();
        e.set_global(DEFAULT_TRIANGLE_LIST, triangles);
        let mut count = 0u32;
        for row in 0..16u32 {
            for column in 0..16u32 {
                let next = (row + 1) * 17;
                let current = row * 17;
                let indices = if row % 2 == column % 2 {
                    [
                        next + column + 1,
                        next + column,
                        current + column,
                        current + column,
                        current + column + 1,
                        next + column + 1,
                    ]
                } else {
                    [
                        next + column,
                        current + column,
                        current + column + 1,
                        current + column + 1,
                        next + column + 1,
                        next + column,
                    ]
                };
                for index in indices {
                    e.mem.set_u16(triangles + 2 * count, index as u16);
                    count += 1;
                }
            }
        }

        // The texture coordinate scale.
        let setting = e
            .call(SETTING_GET_FLOAT_ADDRESS, &args![TEXTURE_TILING_SETTING])
            .u32();
        let mut tiling = e.mem.f32(setting);
        let zero: f64 = e.global(F64_ZERO);
        if tiling as f64 == zero {
            e.call(LOG_MESSAGE, &args![TILING_WARNING]);
            tiling = e.global(SIXTEEN_F32);
        }
        let four: f64 = e.global(F64_FOUR);
        let scale = (four / tiling as f64) as f32;

        // The blocks of 0x121 colours, normals, flags and coordinates.
        let colors = allocate_vector(
            e,
            0x1210,
            0x10,
            QUADRANT_VERTICES,
            NI_COLOR_DEFAULT_CONSTRUCT,
        );
        e.set_global(DEFAULT_COLORS, colors);
        let normals = allocate_vector(e, 0xd8c, 12, QUADRANT_VERTICES, NI_POINT3_DEFAULT_CONSTRUCT);
        e.set_global(DEFAULT_NORMALS, normals);
        let flags = e.call(ALLOCATE, &args![0x121u32]).u32();
        e.set_global(DEFAULT_NORMAL_SET, flags);
        let coordinates =
            allocate_vector(e, 0x908, 8, QUADRANT_VERTICES, NI_POINT3_DEFAULT_CONSTRUCT);
        e.set_global(DEFAULT_TEXTURE_COORDINATES, coordinates);
        let mut index = 0u32;
        for row in 0..17u32 {
            for column in 0..17u32 {
                let color = ni_color(e, 1.0, 1.0, 1.0, 0.0);
                write_words(e, colors + 0x10 * index, &color);
                let normal = ni_point3(e, 0.0, 0.0, 1.0);
                write_words(e, normals + 12 * index, &normal);
                e.mem.set_u8(flags + index, 1);
                let u = (column as f64 / scale as f64) as f32;
                let v = (row as f64 / scale as f64) as f32;
                let coordinate = ni_point2(e, u, v);
                write_words(e, coordinates + 8 * index, &coordinate);
                index += 1;
            }
        }

        // The offsets of the four quadrants and the default vertex blocks.
        for quadrant in 0..QUADRANTS {
            let x = ((quadrant % 2) << 11) as i32 - 0x800;
            e.set_global(X_OFFSETS + 4 * quadrant, x as f32);
            let y = ((quadrant / 2) << 11) as i32 - 0x800;
            e.set_global(Y_OFFSETS + 4 * quadrant, y as f32);
        }
        for quadrant in 0..QUADRANTS {
            let block =
                allocate_vector(e, 0xd8c, 12, QUADRANT_VERTICES, NI_POINT3_DEFAULT_CONSTRUCT);
            e.set_global(DEFAULT_VERTEX_BLOCKS + 4 * quadrant, block);
            let mut index = 0u32;
            for row in 0..17u32 {
                for column in 0..17u32 {
                    let z: f32 = e.global(F32_MINUS_2048);
                    let y_offset: f32 = e.global(Y_OFFSETS + 4 * quadrant);
                    let y = ((row << 7) as f32) + y_offset;
                    let x_offset: f32 = e.global(X_OFFSETS + 4 * quadrant);
                    let x = ((column << 7) as f32) + x_offset;
                    let point = ni_point3(e, x, y, z);
                    write_words(e, block + 12 * index, &point);
                    index += 1;
                }
            }
        }
        e.set_global(STATICS_DEFINED, 1u8);
        e.call(SMART_POINTER_RELEASE, &args![diffuse]);
        e.mem.free(diffuse);
    })
}

// Translated from 00533f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the map at slot 0 of the texturing property's table (`this + 0x1c`):
/// when it differs from the current one, deletes the current map (virtual
/// destructor, slot 0) and stores the new one.
pub fn fn_00533f40(e: &mut Engine, this: Ptr, map: Ptr) {
    let table = this.byte_add(0x1c);
    let slot = e.call(ARRAY_ELEMENT_ADDRESS, &args![table, 0u32]).u32();
    let current = e.mem.u32(slot);
    if map.addr() != current {
        if current != 0 {
            e.vcall(current, 0, &args![1u32]);
        }
        e.with_stack(4, |e, value| {
            e.mem.set_u32(value.addr(), map.addr());
            e.call(ARRAY_SET_AT, &args![table, 0u32, value]);
        });
    }
}

// Translated from 00533fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the 16-bit `value` of a texturing property through `00439360(value,
/// 0xe, 1)`.
pub fn fn_00533fb0(e: &mut Engine, this: Ptr, value: u16) {
    e.call(PROPERTY_SET_MODE, &args![this, value as u32, 0xeu32, 1u32]);
}

// ---- Cell accessors and flags ---------------------------------------------

// Translated from 00533fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The land's cell X: from the loaded data when there is any, else from the
/// parent cell, else 0.
pub fn fn_00533fd0(e: &mut Engine, this: Ptr<TESObjectLAND>) -> i32 {
    let data = loaded_data(e, this);
    if !data.is_null() {
        return e.get(data, LoadedLandData::iCellX);
    }
    let cell = e.get(this, TESObjectLAND::pParentCell);
    if cell.is_null() {
        return 0;
    }
    e.call(CELL_GET_DATA_X, &args![cell]).i32()
}

// Translated from 00534010 (decompiled, FalloutNV.exe 1.4.0.525)
/// The land's cell Y, like [`fn_00533fd0`].
pub fn fn_00534010(e: &mut Engine, this: Ptr<TESObjectLAND>) -> i32 {
    let data = loaded_data(e, this);
    if !data.is_null() {
        return e.get(data, LoadedLandData::iCellY);
    }
    let cell = e.get(this, TESObjectLAND::pParentCell);
    if cell.is_null() {
        return 0;
    }
    e.call(CELL_GET_DATA_Y, &args![cell]).i32()
}

// Translated from 00534050 (decompiled, FalloutNV.exe 1.4.0.525)
/// The world X of the land's cell corner: the cell X times 4096, returned
/// in `ST0` (the `FILD` of an integer, so an exact `f64`).
pub fn fn_00534050(e: &mut Engine, this: Ptr<TESObjectLAND>) -> f64 {
    let x = fn_00533fd0(e, this);
    x.wrapping_shl(12) as f64
}

// Translated from 00534080 (decompiled, FalloutNV.exe 1.4.0.525)
/// The world Y of the land's cell corner, like [`fn_00534050`].
pub fn fn_00534080(e: &mut Engine, this: Ptr<TESObjectLAND>) -> f64 {
    let y = fn_00534010(e, this);
    y.wrapping_shl(12) as f64
}

// Translated from 005340b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The world space of the parent cell, or null without a cell.
pub fn fn_005340b0(e: &mut Engine, this: Ptr<TESObjectLAND>) -> Ptr {
    let cell = e.get(this, TESObjectLAND::pParentCell);
    if cell.is_null() {
        return Ptr::NULL;
    }
    e.call(CELL_GET_WORLD_SPACE, &args![cell]).ptr()
}

// Translated from 005340e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectLAND::SetCell` (Xbox PDB): stores the parent cell; for a cell
/// whose world space has land data (`00586390(0)`) sets flag 0x400 (the data
/// comes from the world space's land file), otherwise clears it.
pub fn tes_object_land_set_cell(e: &mut Engine, this: Ptr<TESObjectLAND>, cell: Ptr) {
    e.set(this, TESObjectLAND::pParentCell, cell);
    if cell.is_null() {
        return;
    }
    let world_space = e.call(CELL_GET_WORLD_SPACE, &args![cell]).u32();
    if world_space != 0
        && e.call(WORLD_SPACE_HAS_LAND, &args![world_space, 0u32])
            .u32()
            != 0
    {
        fn_00534160(e, this, true);
        return;
    }
    fn_00534160(e, this, false);
}

/// Sets or clears `bit` in the land's flags.
fn set_flag(e: &mut Engine, this: Ptr<TESObjectLAND>, bit: u32, on: bool) {
    let flags = e.get(this, TESObjectLAND::Data);
    let flags = if on { flags | bit } else { flags & !bit };
    e.set(this, TESObjectLAND::Data, flags);
}

fn flag(e: &Engine, this: Ptr<TESObjectLAND>, bit: u32) -> bool {
    e.get(this, TESObjectLAND::Data) & bit != 0
}

// Translated from 00534140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the land's data comes from the world space's land file (flag
/// 0x400).
pub fn fn_00534140(e: &mut Engine, this: Ptr<TESObjectLAND>) -> bool {
    flag(e, this, FLAG_FROM_WORLD_SPACE)
}

// Translated from 00534160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears flag 0x400.
pub fn fn_00534160(e: &mut Engine, this: Ptr<TESObjectLAND>, on: bool) {
    set_flag(e, this, FLAG_FROM_WORLD_SPACE, on);
}

// Translated from 005341a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears flag 0x800 (the Havok MOPP code was built).
pub fn fn_005341a0(e: &mut Engine, this: Ptr<TESObjectLAND>, on: bool) {
    set_flag(e, this, FLAG_MOPP_BUILT, on);
}

// Translated from 005341e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether flag 0x10 is set.
pub fn fn_005341e0(e: &mut Engine, this: Ptr<TESObjectLAND>) -> bool {
    flag(e, this, FLAG_EDITED)
}

// Translated from 00534200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears flag 0x10.
pub fn fn_00534200(e: &mut Engine, this: Ptr<TESObjectLAND>, on: bool) {
    set_flag(e, this, FLAG_EDITED, on);
}

// ---- Vertex, normal and colour readers --------------------------------------

/// The default `NiPoint3` of a reader written to `out`.
fn write_point3(e: &mut Engine, out: Ptr, x: f32, y: f32, z: f32) {
    let point = ni_point3(e, x, y, z);
    write_words(e, out.addr(), &point);
}

/// The default colour of the colour reader: the four floats at `011a9be0`.
fn write_default_color(e: &mut Engine, out: Ptr) {
    copy_words(e, DEFAULT_COLOR_WORDS, out.addr(), 4);
}

/// The vertex arrays exist (`0045cb90`: loaded data with `ppVertices`).
fn has_vertex_arrays(e: &mut Engine, this: Ptr<TESObjectLAND>) -> bool {
    e.call(LAND_HAS_VERTEX_ARRAYS, &args![this]).bool()
}

// Translated from 00534240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the position of vertex `index` of quadrant `block` into `out` (an
/// `NiPoint3`), adding the land's base height to Z. Without vertex arrays
/// the result is `(0, 0, default height)` and no base height is added. The
/// vertex comes from `ppVertices` when that block exists, else from the
/// mesh node's geometry of the quadrant, else it is `(0, 0, default
/// height)` plus the base height.
pub fn fn_00534240(e: &mut Engine, this: Ptr<TESObjectLAND>, block: u32, index: i32, out: Ptr) {
    if !has_vertex_arrays(e, this) {
        let height = e.call(LAND_DEFAULT_HEIGHT, &args![this]).f32();
        let point = ni_point3(e, 0.0, 0.0, height);
        write_words(e, out.addr(), &point);
        return;
    }
    let data = loaded_data(e, this);
    let offset = (index as u32).wrapping_mul(12);
    let vertices = e.get(data, LoadedLandData::ppVertices);
    let direct = if vertices.is_null() {
        0
    } else {
        element(e, vertices, block)
    };
    if direct != 0 {
        copy_words(e, direct.wrapping_add(offset), out.addr(), 3);
    } else {
        let meshes = e.get(data, LoadedLandData::ppMesh);
        let mesh = if meshes.is_null() {
            0
        } else {
            element(e, meshes, block)
        };
        if mesh != 0 {
            let geometry = e.call(GEOMETRY_GET_MODEL_DATA, &args![mesh, 0u32]).u32();
            let positions = e.call(GEOMETRY_GET_POSITIONS, &args![geometry]).u32();
            copy_words(e, positions.wrapping_add(offset), out.addr(), 3);
        } else {
            let height = e.call(LAND_DEFAULT_HEIGHT, &args![this]).f32();
            let point = ni_point3(e, 0.0, 0.0, height);
            write_words(e, out.addr(), &point);
        }
    }
    let base_height = e.get(data, LoadedLandData::fBaseHeight);
    let z = e.mem.f32(out.addr() + 8);
    e.mem.set_f32(out.addr() + 8, z + base_height);
}

// Translated from 00534390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the normal of vertex `index` of quadrant `block` into `out`: from
/// `ppNormals`, else from the normal array of the quadrant's mesh geometry,
/// else (when the geometry has no normal stream) locks the geometry data
/// and reads the packed vertex through [`fn_00534570`]. When nothing gives
/// one, and without vertex arrays, the normal is `(0, 0, 1)`.
pub fn fn_00534390(e: &mut Engine, this: Ptr<TESObjectLAND>, block: u32, index: i32, out: Ptr) {
    if !has_vertex_arrays(e, this) {
        write_point3(e, out, 0.0, 0.0, 1.0);
        return;
    }
    let data = loaded_data(e, this);
    let offset = (index as u32).wrapping_mul(12);
    let mut found = false;
    let normals = e.get(data, LoadedLandData::ppNormals);
    let direct = if normals.is_null() {
        0
    } else {
        element(e, normals, block)
    };
    if direct != 0 {
        copy_words(e, direct.wrapping_add(offset), out.addr(), 3);
        found = true;
    } else {
        let meshes = e.get(data, LoadedLandData::ppMesh);
        let mesh = if meshes.is_null() {
            0
        } else {
            element(e, meshes, block)
        };
        if mesh != 0 {
            let geometry = e.call(GEOMETRY_GET_MODEL_DATA, &args![mesh, 0u32]).u32();
            if e.call(GEOMETRY_GET_NORMALS, &args![geometry]).u32() != 0 {
                let normals = e.call(GEOMETRY_GET_NORMALS, &args![geometry]).u32();
                copy_words(e, normals.wrapping_add(offset), out.addr(), 3);
                found = true;
            } else {
                let geometry_data = e.call(GEOMETRY_GET_DATA, &args![geometry]).u32();
                found = e.with_stack(12, |e, lock| {
                    fn_00534550(e, lock);
                    let owner = e.call(GEOMETRY_DATA_GET_OWNER, &args![geometry_data]).u32();
                    if owner != 0
                        && e.vcall(owner, OWNER_CAN_LOCK_SLOT, &[]).bool()
                        && e.call(GEOMETRY_DATA_BEGIN, &args![geometry_data, 1u32])
                            .bool()
                    {
                        e.call(GEOMETRY_DATA_LOCK_POSITIONS, &args![geometry_data, lock]);
                        fn_00534570(e, lock, index, out);
                        e.call(GEOMETRY_DATA_END, &args![geometry_data]);
                        true
                    } else {
                        false
                    }
                });
            }
        }
    }
    if !found {
        write_point3(e, out, 0.0, 0.0, 1.0);
    }
}

// Translated from 00534550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs a vertex lock (`{base, stride, packed}` zeroed) and returns
/// it.
pub fn fn_00534550(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(LOCK_CONSTRUCT, &args![this]);
    this
}

// Translated from 00534570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the three floats of vertex `index` of a locked vertex stream
/// (`this` = `{base, stride, packed}`) into `out`.
pub fn fn_00534570(e: &mut Engine, this: Ptr, index: i32, out: Ptr) {
    e.call(LOCK_CHECK_INDEX, &args![this, index]);
    let base = e.mem.u32(this.addr());
    let stride = e.mem.u32(this.addr() + 4);
    let vertex = (index as u32).wrapping_mul(stride).wrapping_add(base);
    copy_words(e, vertex, out.addr(), 3);
}

// Translated from 005345c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the colour of vertex `index` of quadrant `block` into `out` (an
/// `NiColorA`): from `ppColorsA`, else from the colour array of the
/// quadrant's mesh geometry, else from its locked colour stream through
/// [`fn_00534780`]. Without vertex arrays, or when nothing gives one, the
/// colour is the exe's default (four floats at `011a9be0`).
pub fn fn_005345c0(e: &mut Engine, this: Ptr<TESObjectLAND>, block: u32, index: i32, out: Ptr) {
    if !has_vertex_arrays(e, this) {
        write_default_color(e, out);
        return;
    }
    let data = loaded_data(e, this);
    let mut found = false;
    let colors = e.get(data, LoadedLandData::ppColorsA);
    let direct = if colors.is_null() {
        0
    } else {
        element(e, colors, block)
    };
    if direct != 0 {
        copy_words(e, direct.wrapping_add((index as u32) << 4), out.addr(), 4);
        found = true;
    } else {
        let meshes = e.get(data, LoadedLandData::ppMesh);
        let mesh = if meshes.is_null() {
            0
        } else {
            element(e, meshes, block)
        };
        if mesh != 0 {
            let geometry = e.call(GEOMETRY_GET_MODEL_DATA, &args![mesh, 0u32]).u32();
            if e.call(GEOMETRY_GET_NORMALS, &args![geometry]).u32() != 0 {
                let colors = e.call(GEOMETRY_GET_COLORS, &args![geometry]).u32();
                copy_words(e, colors.wrapping_add((index as u32) << 4), out.addr(), 4);
                found = true;
            } else {
                let geometry_data = e.call(GEOMETRY_GET_DATA, &args![geometry]).u32();
                found = e.with_stack(12, |e, lock| {
                    fn_00534550(e, lock);
                    let owner = e.call(GEOMETRY_DATA_GET_OWNER, &args![geometry_data]).u32();
                    if owner != 0
                        && e.vcall(owner, OWNER_CAN_LOCK_SLOT, &[]).bool()
                        && e.call(GEOMETRY_DATA_BEGIN, &args![geometry_data, 1u32])
                            .bool()
                    {
                        e.call(GEOMETRY_DATA_LOCK_COLORS, &args![geometry_data, lock]);
                        fn_00534780(e, lock, index, out);
                        e.call(GEOMETRY_DATA_END, &args![geometry_data]);
                        true
                    } else {
                        false
                    }
                });
            }
        }
    }
    if !found {
        write_default_color(e, out);
    }
}

// Translated from 00534780 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the colour of vertex `index` of a locked colour stream (`this` =
/// `{base, stride, packed}`) into `out`. A packed stream holds one 32-bit
/// `0xAARRGGBB` word per vertex, which becomes `(r, g, b, a) / 255`;
/// otherwise the four floats are copied.
pub fn fn_00534780(e: &mut Engine, this: Ptr, index: i32, out: Ptr) {
    e.call(LOCK_CHECK_INDEX, &args![this, index]);
    let base = e.mem.u32(this.addr());
    let stride = e.mem.u32(this.addr() + 4);
    let vertex = (index as u32).wrapping_mul(stride).wrapping_add(base);
    if e.mem.u8(this.addr() + 8) != 0 {
        let word = e.mem.u32(vertex);
        let red = (word >> 16) & 0xff;
        let green = (word >> 8) & 0xff;
        let blue = word & 0xff;
        let alpha = (word >> 24) & 0xff;
        let divisor: f64 = e.global(F64_255);
        for (i, channel) in [red, green, blue, alpha].into_iter().enumerate() {
            e.mem
                .set_f32(out.addr() + 4 * i as u32, (channel as f64 / divisor) as f32);
        }
    } else {
        copy_words(e, vertex, out.addr(), 4);
    }
}

// Translated from 00534880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Looks the form `id` up (`004867a0(0, id)`) and casts it to
/// `TESObjectLAND`; the first stack word is not read.
pub fn fn_00534880(e: &mut Engine, this: Ptr, _unused_1: u32, id: u32) -> Ptr {
    let form = e.call(FORM_LOOKUP, &args![this, 0u32, id]).u32();
    e.call(
        RT_DYNAMIC_CAST,
        &args![form, 0u32, RTTI_TES_FORM, RTTI_TES_OBJECT_LAND, 0u32],
    )
    .ptr()
}

// Translated from 005348c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies another land's data into this one: casts `form` to
/// `TESObjectLAND` (nothing happens when it is not one), loads the source's
/// vertices when they are not loaded, takes its parent cell, allocates this
/// land's data as a copy, and copies the quadrants' vertices (0xd8c bytes),
/// normals (0xd8c), colours (0x1210), normal-set flags (0x121), percent
/// block (0x2420), layer textures (0x18) and base texture, then the flags
/// and the height extents. The source's vertices are unloaded again if this
/// loaded them.
pub fn fn_005348c0(e: &mut Engine, this: Ptr<TESObjectLAND>, form: Ptr) {
    let source = form_cast_from(e, form.addr(), RTTI_TES_OBJECT_LAND);
    if source == 0 {
        return;
    }
    let source = Ptr::<TESObjectLAND>::new(source);
    let was_loaded = e.call(LAND_IS_LOADED, &args![source]).bool();
    if !was_loaded {
        tes_object_land_load_vertices(e, source, false);
    }
    let cell = e.call(PARENT_CELL, &args![source]).ptr::<()>();
    e.set(this, TESObjectLAND::pParentCell, cell);
    e.call(LAND_ALLOCATE_DATA, &args![this, source]);
    let to = loaded_data(e, this);
    let from = loaded_data(e, source);
    for quadrant in 0..QUADRANTS {
        for (field, size) in [
            (LoadedLandData::ppVertices.off, 0xd8cu32),
            (LoadedLandData::ppNormals.off, 0xd8c),
            (LoadedLandData::ppColorsA.off, 0x1210),
            (LoadedLandData::ppNormalsSet.off, 0x121),
        ] {
            let destination = e.mem.u32(to.addr() + field);
            let source_block = e.mem.u32(from.addr() + field);
            let destination = e.mem.u32(destination + 4 * quadrant);
            let source_block = e.mem.u32(source_block + 4 * quadrant);
            e.call(MEMORY_COPY, &args![destination, source_block, size]);
        }
        let destination = e
            .mem
            .u32(to.addr() + LoadedLandData::ppPercentArrays.off + 4 * quadrant);
        let source_block = e
            .mem
            .u32(from.addr() + LoadedLandData::ppPercentArrays.off + 4 * quadrant);
        let destination = e.mem.u32(destination);
        let source_block = e.mem.u32(source_block);
        e.call(MEMORY_COPY, &args![destination, source_block, 0x2420u32]);
        let destination = e
            .mem
            .u32(to.addr() + LoadedLandData::pQuadTextureArray.off + 4 * quadrant);
        let source_block = e
            .mem
            .u32(from.addr() + LoadedLandData::pQuadTextureArray.off + 4 * quadrant);
        e.call(MEMORY_COPY, &args![destination, source_block, 0x18u32]);
        let base_texture = e
            .mem
            .u32(from.addr() + LoadedLandData::pDefQuadTexture.off + 4 * quadrant);
        e.mem.set_u32(
            to.addr() + LoadedLandData::pDefQuadTexture.off + 4 * quadrant,
            base_texture,
        );
    }
    e.call(
        MEMORY_COPY,
        &args![
            this.byte_add(TESObjectLAND::Data.off),
            source.byte_add(TESObjectLAND::Data.off),
            4u32
        ],
    );
    e.call(
        MEMORY_COPY,
        &args![
            to.byte_add(LoadedLandData::HeightExtentsMin.off),
            from.byte_add(LoadedLandData::HeightExtentsMin.off),
            8u32
        ],
    );
    if !was_loaded {
        tes_object_land_un_load_vertices(e, source);
    }
    e.call(LAND_SET_LOADED, &args![this, 1u32]);
}

/// A `__RTDynamicCast` of `object` from `TESForm` to `target`.
fn form_cast_from(e: &mut Engine, object: u32, target: u32) -> u32 {
    form_cast(e, object, target)
}

// ---- Chunk readers shared by Load and LoadVerticesIntoArrays ---------------

/// The messages the texture chunks log; the two loaders have their own
/// ("MASTERFILE: ..." for `Load`, "TEXTURES: ..." for
/// `LoadVerticesIntoArrays`).
struct TextureMessages {
    /// `Land (%i, %i) clamping invalid index %i for block %i.`
    clamped_layer: u32,
    /// `Land (%i, %i) unable to find additional texture ID (%08X) for block
    /// %i.`
    missing_additional: u32,
    /// `Land (%i, %i) unable to find base texture ID (%08X) for block %i.`
    missing_base: u32,
    /// `Land (%i, %i) found unrecognized vertex texture data in file %s.`
    unrecognized_data: u32,
}

const MASTER_FILE_MESSAGES: TextureMessages = TextureMessages {
    clamped_layer: 0x0102_df48,
    missing_additional: 0x0102_def0,
    missing_base: 0x0102_df90,
    unrecognized_data: 0x0102_dea0,
};

const CELL_MESSAGES: TextureMessages = TextureMessages {
    clamped_layer: 0x0102_e0dc,
    missing_additional: 0x0102_e088,
    missing_base: 0x0102_e120,
    unrecognized_data: 0x0102_e038,
};

/// The state the chunk loop of a loader keeps: whether a message was logged
/// (so the warnings get flushed at the end), and the quadrant and layer of
/// the last `ATXT` chunk (`-1` when none), which the `VTXT` chunk that
/// follows belongs to.
struct ChunkState {
    warned: bool,
    quadrant: i32,
    layer: i32,
}

impl ChunkState {
    fn new() -> Self {
        ChunkState {
            warned: false,
            quadrant: -1,
            layer: -1,
        }
    }
}

fn file_needs_swap(e: &mut Engine, file: Ptr) -> bool {
    e.call(FILE_NEEDS_SWAP, &args![file]).bool()
}

/// The first index of quadrant `quadrant`'s 17 x 17 block in a 33 x 33
/// chunk grid, and the grid index of its vertex `k`.
fn quadrant_origin(quadrant: u32) -> u32 {
    (quadrant / 2) * 0x10 * 0x21 + (quadrant % 2) * 0x10
}

fn grid_index(origin: u32, k: u32) -> u32 {
    (k / 0x11) * 0x21 + origin + k % 0x11
}

/// The `VHGT` chunk at `buffer` (a float base height and 0x441 signed byte
/// steps; each step adds to the running height, which restarts from the
/// first height of the row at the end of a row): resets the height
/// extents, builds the vertices of the four quadrants (x and y from the
/// quadrant offsets and the grid position, z = the height rounded to an
/// integer, times 8), keeping the lowest and highest, sets the base height
/// to their mean and makes the heights relative to it.
fn read_height_chunk(e: &mut Engine, this: Ptr<TESObjectLAND>, buffer: u32) {
    let data = loaded_data(e, this);
    let float_max: f32 = e.global(LARGEST_F32);
    let negative_float_max: f32 = e.global(LOWEST_F32);
    let extents = ni_point2(e, float_max, negative_float_max);
    e.mem.set_u32(
        data.addr() + LoadedLandData::HeightExtentsMin.off,
        extents[0],
    );
    e.mem.set_u32(
        data.addr() + LoadedLandData::HeightExtentsMax.off,
        extents[1],
    );
    let mut offset = e.mem.f32(buffer);
    let mut heights = vec![0f32; CHUNK_VERTICES];
    for i in 0..CHUNK_VERTICES {
        let step = e.mem.i8(buffer + 4 + i as u32);
        heights[i] = step as f32 + offset;
        offset = if (i + 1) % 0x21 == 0 {
            heights[i - 0x20]
        } else {
            heights[i]
        };
    }
    let vertices = e.get(data, LoadedLandData::ppVertices);
    for quadrant in 0..QUADRANTS {
        let origin = quadrant_origin(quadrant);
        for k in 0..QUADRANT_VERTICES {
            let sample = grid_index(origin, k) as usize;
            let rounded = e.call(FLOAT_TO_INT, &args![heights[sample]]).i32();
            let z = rounded.wrapping_shl(3) as f32;
            let x_offset: f32 = e.global(X_OFFSETS + 4 * quadrant);
            let x = ((k % 0x11) << 7) as f32 + x_offset;
            let y_offset: f32 = e.global(Y_OFFSETS + 4 * quadrant);
            let y = ((k / 0x11) << 7) as f32 + y_offset;
            let vertex = element(e, vertices, quadrant) + 12 * k;
            e.mem.set_f32(vertex, x);
            e.mem.set_f32(vertex + 4, y);
            e.mem.set_f32(vertex + 8, z);
            let lowest = e.get(data, LoadedLandData::HeightExtentsMin);
            if z < lowest {
                e.set(data, LoadedLandData::HeightExtentsMin, z);
            } else {
                let highest = e.get(data, LoadedLandData::HeightExtentsMax);
                if highest < z {
                    e.set(data, LoadedLandData::HeightExtentsMax, z);
                }
            }
        }
    }
    let lowest = e.get(data, LoadedLandData::HeightExtentsMin);
    let highest = e.get(data, LoadedLandData::HeightExtentsMax);
    let half: f64 = e.global(F64_HALF);
    let base_height = ((lowest as f64 + highest as f64) * half) as f32;
    e.set(data, LoadedLandData::fBaseHeight, base_height);
    for quadrant in 0..QUADRANTS {
        for k in 0..QUADRANT_VERTICES {
            let vertex = element(e, vertices, quadrant) + 12 * k;
            let base_height = e.get(data, LoadedLandData::fBaseHeight);
            let z = e.mem.f32(vertex + 8);
            e.mem.set_f32(vertex + 8, z - base_height);
        }
    }
}

/// The `VNML` chunk at `buffer` (33 x 33 signed byte normals, three bytes
/// each): the normals of the four quadrants, each component divided by 127,
/// then normalized.
fn read_normal_chunk(e: &mut Engine, this: Ptr<TESObjectLAND>, buffer: u32) {
    let data = loaded_data(e, this);
    let normals = e.get(data, LoadedLandData::ppNormals);
    for quadrant in 0..QUADRANTS {
        let origin = quadrant_origin(quadrant);
        for k in 0..QUADRANT_VERTICES {
            let sample = buffer + grid_index(origin, k) * 3;
            let normal = element(e, normals, quadrant) + 12 * k;
            for component in 0..3u32 {
                let divisor: f64 = e.global(F64_127);
                let value = e.mem.i8(sample + component) as f64 / divisor;
                e.mem.set_f32(normal + 4 * component, value as f32);
            }
            e.call(NI_POINT3_NORMALIZE, &args![normal]);
        }
    }
}

/// The `VCLR` chunk at `buffer` (33 x 33 colours of three bytes): the
/// colours of the four quadrants, each byte divided by 255, alpha 1.
fn read_color_chunk(e: &mut Engine, this: Ptr<TESObjectLAND>, buffer: u32) {
    let data = loaded_data(e, this);
    let colors = e.get(data, LoadedLandData::ppColorsA);
    for quadrant in 0..QUADRANTS {
        let origin = quadrant_origin(quadrant);
        for k in 0..QUADRANT_VERTICES {
            let sample = buffer + grid_index(origin, k) * 3;
            let divisor: f64 = e.global(F64_255);
            let r = (e.mem.u8(sample) as f64 / divisor) as f32;
            let g = (e.mem.u8(sample + 1) as f64 / divisor) as f32;
            let b = (e.mem.u8(sample + 2) as f64 / divisor) as f32;
            let color = ni_color(e, r, g, b, 1.0);
            let at = element(e, colors, quadrant) + 0x10 * k;
            write_words(e, at, &color);
        }
    }
}

/// Queues the texture for loading: creates the land's `QueuedFile` first
/// when it has none.
fn queue_texture(e: &mut Engine, this: Ptr<TESObjectLAND>, texture: u32) {
    let slot = this.byte_add(TESObjectLAND::spQueuedTextures.off);
    if e.call(SMART_POINTER_GET, &args![slot]).u32() == 0 {
        let memory = e.call(ALLOCATE, &args![0x28u32]).u32();
        let queued = if memory == 0 {
            0
        } else {
            e.call(QUEUED_FILE_CONSTRUCT, &args![memory, QUEUE_PRIORITY])
                .u32()
        };
        e.call(QUEUED_FILE_POINTER_SET, &args![slot, queued]);
    }
    let queued = e.call(SMART_POINTER_GET, &args![slot]).u32();
    e.call(LAND_TEXTURE_QUEUE, &args![texture, QUEUE_PRIORITY, queued]);
}

/// The `ATXT` chunk (an 8-byte record: texture form id, quadrant byte, a
/// byte, layer word): looks the `TESLandTexture` up (the default texture
/// for id 0), stores it in the quadrant's layer (layers above 5 are
/// clamped to 5 with a message), queues it, and remembers quadrant and
/// layer for the `VTXT` chunk.
fn read_additional_texture_chunk(
    e: &mut Engine,
    this: Ptr<TESObjectLAND>,
    file: Ptr,
    messages: &TextureMessages,
    state: &mut ChunkState,
) {
    e.with_stack(8, |e, record| {
        let record = record.addr();
        e.call(MEMORY_SET, &args![record, 0u32, 8u32]);
        e.call(FILE_GET_CHUNK_DATA, &args![file, record, 8u32]);
        if file_needs_swap(e, file) {
            fn_00535a60(e, Ptr::new(record));
        }
        let quadrant = e.mem.u8(record + 4) as u32;
        if e.mem.u16(record + 6) > 5 {
            let layer = e.mem.u16(record + 6) as u32;
            let (x, y) = cell_coordinates(e, this);
            e.call(
                LOG_MESSAGE,
                &args![messages.clamped_layer, x, y, layer, quadrant],
            );
            e.mem.set_u16(record + 6, 5);
            state.warned = true;
        }
        let texture = if e.mem.u32(record) == 0 {
            fn_00535ae0(e).addr()
        } else {
            e.call(FORM_ADD_COMPILE_INDEX, &args![record, file]);
            let id = e.mem.u32(record);
            let form = e.call(FORM_LOOKUP_BY_ID, &args![id]).u32();
            let texture = form_cast(e, form, RTTI_TES_LAND_TEXTURE);
            if texture == 0 {
                let id = e.mem.u32(record);
                let (x, y) = cell_coordinates(e, this);
                e.call(
                    LOG_MESSAGE,
                    &args![messages.missing_additional, x, y, id, quadrant],
                );
                state.warned = true;
            }
            texture
        };
        let data = loaded_data(e, this);
        let layers = e
            .mem
            .u32(data.addr() + LoadedLandData::pQuadTextureArray.off + 4 * quadrant);
        let layer = e.mem.u16(record + 6) as u32;
        e.mem.set_u32(layers.wrapping_add(4 * layer), texture);
        if texture != 0 {
            queue_texture(e, this, texture);
        }
        state.quadrant = quadrant as i32;
        state.layer = layer as i32;
    });
}

/// The `BTXT` chunk (an 8-byte record: texture form id, quadrant byte):
/// looks the `TESLandTexture` up and makes it the quadrant's base texture
/// (a message when it is not found), queues it.
fn read_base_texture_chunk(
    e: &mut Engine,
    this: Ptr<TESObjectLAND>,
    file: Ptr,
    messages: &TextureMessages,
    state: &mut ChunkState,
) {
    e.with_stack(8, |e, record| {
        let record = record.addr();
        e.call(MEMORY_SET, &args![record, 0u32, 8u32]);
        e.call(FILE_GET_CHUNK_DATA, &args![file, record, 8u32]);
        if file_needs_swap(e, file) {
            fn_00535a60(e, Ptr::new(record));
        }
        e.call(FORM_ADD_COMPILE_INDEX, &args![record, file]);
        let id = e.mem.u32(record);
        let form = e.call(FORM_LOOKUP_BY_ID, &args![id]).u32();
        let texture = form_cast(e, form, RTTI_TES_LAND_TEXTURE);
        let quadrant = e.mem.u8(record + 4) as u32;
        let data = loaded_data(e, this);
        let slot = data.addr() + LoadedLandData::pDefQuadTexture.off + 4 * quadrant;
        e.mem.set_u32(slot, texture);
        if e.mem.u32(slot) == 0 {
            let id = e.mem.u32(record);
            let (x, y) = cell_coordinates(e, this);
            e.call(
                LOG_MESSAGE,
                &args![messages.missing_base, x, y, id, quadrant],
            );
            state.warned = true;
        } else {
            let texture = e.mem.u32(slot);
            queue_texture(e, this, texture);
        }
    });
}

/// The `VTXT` chunk (8-byte entries: position word, two bytes, opacity
/// float) of the layer the last `ATXT` chunk set up: hands every entry to
/// `SetVertexOpacity` (`0053a8a0`). `Load` divides an opacity above 1 by
/// 100 first (`scale_opacity`). A size that is not a multiple of 8 logs a
/// message. Quadrant and layer are reset afterwards.
fn read_vertex_texture_chunk(
    e: &mut Engine,
    this: Ptr<TESObjectLAND>,
    file: Ptr,
    messages: &TextureMessages,
    scale_opacity: bool,
    state: &mut ChunkState,
) {
    if state.quadrant >= 0 && state.layer >= 0 {
        let size = e.call(FILE_GET_CHUNK_SIZE, &args![file]).u32();
        if size % 8 == 0 {
            let entries = e.call(ALLOCATE, &args![size]).u32();
            e.call(FILE_GET_CHUNK_DATA, &args![file, entries, size]);
            for i in 0..size >> 3 {
                let entry = entries.wrapping_add(8 * i);
                if file_needs_swap(e, file) {
                    fn_00535aa0(e, Ptr::new(entry));
                }
                if scale_opacity {
                    let one: f64 = e.global(F64_ONE);
                    let opacity = e.mem.f32(entry + 4);
                    if opacity as f64 > one {
                        let hundred: f64 = e.global(F64_HUNDRED);
                        e.mem.set_f32(entry + 4, (opacity as f64 / hundred) as f32);
                    }
                }
                let position = e.mem.u16(entry) as u32;
                let opacity = e.mem.f32(entry + 4);
                e.call(
                    LAND_SET_VERTEX_OPACITY,
                    &args![
                        this,
                        state.quadrant as u32 & 0xff,
                        position,
                        state.layer as u32 & 0xffff,
                        opacity
                    ],
                );
            }
            e.call(DEALLOCATE, &args![entries]);
        } else {
            let name = e.call(FILE_GET_NAME, &args![file]).u32();
            let (x, y) = cell_coordinates(e, this);
            e.call(LOG_MESSAGE, &args![messages.unrecognized_data, x, y, name]);
            state.warned = true;
        }
    }
    state.quadrant = -1;
    state.layer = -1;
}

/// The `MPCD` chunk: the Havok MOPP code. Replaces the land's code (the old
/// one is released), has it built from the chunk (`00538c00`) and, when
/// that gave one, takes a reference and sets flag 0x800.
fn read_mopp_chunk(e: &mut Engine, this: Ptr<TESObjectLAND>, file: Ptr) {
    let size = e.call(FILE_GET_CHUNK_SIZE, &args![file]).u32();
    let code = e.call(ALLOCATE, &args![size]).u32();
    e.call(FILE_GET_CHUNK_DATA, &args![file, code, size]);
    let data = loaded_data(e, this);
    let old = e.get(data, LoadedLandData::pMoppCode);
    if !old.is_null() {
        e.call(HAVOK_REMOVE_REFERENCE, &args![old]);
        e.set(data, LoadedLandData::pMoppCode, Ptr::NULL);
    }
    e.call(
        LAND_BUILD_MOPP,
        &args![
            this,
            code,
            size,
            data.byte_add(LoadedLandData::pMoppCode.off)
        ],
    );
    let new = e.get(data, LoadedLandData::pMoppCode);
    if !new.is_null() {
        e.call(HAVOK_ADD_REFERENCE, &args![new]);
        fn_005341a0(e, this, true);
    }
    e.call(DEALLOCATE, &args![code]);
}

// ---- Loading ---------------------------------------------------------------

// Translated from 00534ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectLAND::Load` (Xbox PDB): loads the land record from `file`.
/// Returns false when the record is not a land record (type `B`). The
/// geometry chunks are only read when the file is the active one or a
/// master is being loaded, and the data does not come from the world
/// space's land file; then the loaded data is allocated first. `DATA` sets
/// the flags (keeping the world space bit), `VHGT`, `VNML` and `VCLR` build
/// the vertices, normals and colours of the quadrants, `MPCD` the Havok
/// code, `BTXT`, `ATXT` and `VTXT` the textures. The warnings are flushed
/// if any message was logged, and the "vertices loaded" state follows
/// whether the geometry was read.
///
/// The cell's id read at the start (`0084e3a0`) is not used.
pub fn tes_object_land_load(e: &mut Engine, this: Ptr<TESObjectLAND>, file: Ptr) -> bool {
    scoped(e, 0x2c2, |e| {
        if e.call(FILE_GET_TES_FORM, &args![file]).u8() != b'B' {
            return false;
        }
        e.call(FORM_ID, &args![this]);
        e.call(FORM_LOAD, &args![this, file]);
        let mut state = ChunkState::new();
        let active = e.call(FILE_GET_ACTIVE, &args![file]).bool();
        let load_geometry =
            (active || e.call(LOADING_MASTER_FILE, &[]).bool()) && !fn_00534140(e, this);
        if load_geometry {
            e.call(LAND_ALLOCATE_DATA, &args![this, 0u32]);
        }
        loop {
            let chunk = e.call(FILE_GET_CHUNK, &args![file]).u32();
            match chunk {
                CHUNK_DATA => {
                    let from_world_space = fn_00534140(e, this);
                    e.call(
                        FILE_GET_CHUNK_DATA,
                        &args![file, this.byte_add(TESObjectLAND::Data.off), 4u32],
                    );
                    if file_needs_swap(e, file) {
                        e.call(SWAP_WORD, &args![this.byte_add(TESObjectLAND::Data.off)]);
                    }
                    e.call(LAND_SET_LOADED, &args![this, 0u32]);
                    fn_00534160(e, this, from_world_space);
                }
                CHUNK_HEIGHTS => {
                    if load_geometry && flag(e, this, FLAG_HEIGHTS) {
                        e.with_stack(0x448, |e, buffer| {
                            e.call(FILE_GET_CHUNK_DATA, &args![file, buffer, 0u32]);
                            if file_needs_swap(e, file) {
                                e.call(SWAP_WORD, &args![buffer]);
                            }
                            read_height_chunk(e, this, buffer.addr());
                        });
                    }
                }
                CHUNK_NORMALS => {
                    if load_geometry && flag(e, this, FLAG_HEIGHTS) {
                        e.with_stack(0xcc3, |e, buffer| {
                            e.call(FILE_GET_CHUNK_DATA, &args![file, buffer, 0u32]);
                            read_normal_chunk(e, this, buffer.addr());
                        });
                    }
                }
                CHUNK_MOPP => {
                    if load_geometry {
                        read_mopp_chunk(e, this, file);
                    }
                }
                CHUNK_COLORS => {
                    if load_geometry && flag(e, this, FLAG_COLORS) {
                        e.with_stack(0xcc3, |e, buffer| {
                            e.call(FILE_GET_CHUNK_DATA, &args![file, buffer, 0u32]);
                            read_color_chunk(e, this, buffer.addr());
                        });
                    }
                }
                CHUNK_ADDITIONAL_TEXTURE => {
                    if load_geometry {
                        read_additional_texture_chunk(
                            e,
                            this,
                            file,
                            &MASTER_FILE_MESSAGES,
                            &mut state,
                        );
                    }
                }
                CHUNK_BASE_TEXTURE => {
                    if load_geometry {
                        read_base_texture_chunk(e, this, file, &MASTER_FILE_MESSAGES, &mut state);
                    }
                }
                CHUNK_VERTEX_TEXTURE if load_geometry => {
                    read_vertex_texture_chunk(
                        e,
                        this,
                        file,
                        &MASTER_FILE_MESSAGES,
                        true,
                        &mut state,
                    );
                }
                _ => {}
            }
            if !e.call(FILE_NEXT_CHUNK, &args![file]).bool() {
                break;
            }
        }
        if state.warned {
            e.call(LAND_FLUSH_WARNINGS, &args![this]);
        }
        e.call(LAND_SET_LOADED, &args![this, load_geometry as u32]);
        true
    })
}

// Translated from 00535a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Byte swaps an `ATXT`/`BTXT` record: the form id (a 32-bit word at +0)
/// and the layer (a 16-bit word at +6).
pub fn fn_00535a60(e: &mut Engine, this: Ptr) {
    e.call(SWAP_BYTES_32, &args![this, 0u32]);
    e.call(SWAP_BYTES_16, &args![this.byte_add(6), 0u32]);
}

// Translated from 00535aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Byte swaps a `VTXT` entry: the position (a 16-bit word at +0) and the
/// opacity (a 32-bit word at +4).
pub fn fn_00535aa0(e: &mut Engine, this: Ptr) {
    e.call(SWAP_BYTES_16, &args![this, 0u32]);
    e.call(SWAP_BYTES_32, &args![this.byte_add(4), 0u32]);
}

// Translated from 00535ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectLAND::pDefText` (Xbox PDB): the default land texture.
pub fn fn_00535ae0(e: &mut Engine) -> Ptr {
    Ptr::new(e.global::<u32>(DEFAULT_LAND_TEXTURE))
}

// Translated from 00535af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The scene node of quadrant `index` (0 to 3) of the parent cell's 3D
/// (`0045c9a0`), or null without a cell or for an index of 4 or more.
pub fn fn_00535af0(e: &mut Engine, this: Ptr<TESObjectLAND>, index: i32) -> Ptr {
    let cell = e.get(this, TESObjectLAND::pParentCell);
    if !cell.is_null() && index < 4 {
        return e.call(CELL_GET_NODE, &args![cell, index]).ptr();
    }
    Ptr::NULL
}

// Translated from 00535b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectLAND::GetWorldOffsetForBlock` (Xbox PDB): writes the world
/// position of the land's centre into `out` (an `NiPoint3`): the cell
/// corner plus 2048 in x and y, and the base height. The block number is
/// not used. Returns `out`.
pub fn tes_object_land_get_world_offset_for_block(
    e: &mut Engine,
    this: Ptr<TESObjectLAND>,
    out: Ptr,
    _unused_2: u32,
) -> Ptr {
    e.with_stack(12, |e, local| {
        e.call(NI_POINT3_DEFAULT_CONSTRUCT, &args![local]);
        let offset: f64 = e.global(F64_2048);
        let x = (fn_00534050(e, this) + offset) as f32;
        e.mem.set_f32(local.addr(), x);
        let offset: f64 = e.global(F64_2048);
        let y = (fn_00534080(e, this) + offset) as f32;
        e.mem.set_f32(local.addr() + 4, y);
        let z = e.call(LAND_BASE_HEIGHT, &args![this]).f32();
        e.mem.set_f32(local.addr() + 8, z);
        copy_words(e, local.addr(), out.addr(), 3);
    });
    out
}

// Translated from 00535b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectLAND::LoadVertices` (Xbox PDB): makes sure the land's vertex
/// arrays are loaded, and builds the quadrant meshes (`005374f0`) when
/// `build_meshes` is set (never while the loading menu is up). A land whose
/// vertices are already loaded only builds the meshes it lacks. Otherwise the
/// loaded data is allocated, and filled by `LoadVerticesIntoArrays` when the
/// land has a file to read it from (the world space's land data, or a
/// record with heights, colours or textures that belongs to a file).
/// Returns that function's result, true when nothing needed reading.
pub fn tes_object_land_load_vertices(
    e: &mut Engine,
    this: Ptr<TESObjectLAND>,
    build_meshes: bool,
) -> bool {
    scoped(e, 0x455, |e| {
        let build_meshes = if e.call(LOADING_MENU_VISIBLE, &[]).bool() {
            false
        } else {
            build_meshes
        };
        if e.call(LAND_IS_LOADED, &args![this]).bool() {
            let data = loaded_data(e, this);
            if e.get(data, LoadedLandData::ppMesh).is_null() && build_meshes {
                fn_005374f0(e, this);
            }
            return true;
        }
        if !fn_00534140(e, this) {
            let flags = e.get(this, TESObjectLAND::Data);
            let has_file = flags & (FLAG_HEIGHTS | FLAG_COLORS | FLAG_TEXTURES) != 0
                && e.call(FORM_GET_FILE, &args![this, 0xffff_ffffu32]).u32() != 0;
            if !has_file {
                e.call(LAND_ALLOCATE_DATA, &args![this, 0u32]);
                if build_meshes {
                    fn_005374f0(e, this);
                }
                return true;
            }
        }
        e.call(LAND_ALLOCATE_DATA, &args![this, 0u32]);
        let loaded = tes_object_land_load_vertices_into_arrays(e, this);
        e.call(LAND_SET_LOADED, &args![this, 1u32]);
        if build_meshes {
            fn_005374f0(e, this);
        }
        loaded
    })
}

// Translated from 00535d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectLAND::LoadVerticesIntoArrays` (Xbox PDB): reads the land's
/// chunks from its plugin file into the (already allocated) loaded data.
/// The file and the record offset come from the form (found in the file) or,
/// for a land of the world space's land file, from `FindLandDataInFile`;
/// without them the function returns true, with the form missing from its
/// file, or not a land record, it logs "Failed to load landscape data" and
/// returns false (warnings are switched off around the message). Chunks are
/// gated by the land's flags (heights and normals 1, colours 2, textures 4)
/// instead of a "load" flag; `DATA` only merges the low three flag bits and
/// `VTXT` does not scale the opacity. Logs "does not contain Normal Data"
/// when heights came without normals. Returns true.
pub fn tes_object_land_load_vertices_into_arrays(e: &mut Engine, this: Ptr<TESObjectLAND>) -> bool {
    if e.call(LAND_IS_LOADED, &args![this]).bool() {
        return true;
    }
    let mut state = ChunkState::new();
    let mut failed = false;
    let file;
    let offset;
    if !fn_00534140(e, this) {
        let flags = e.get(this, TESObjectLAND::Data);
        if flags & (FLAG_HEIGHTS | FLAG_COLORS | FLAG_TEXTURES) == 0 {
            return true;
        }
        let form_file = e.call(FORM_GET_FILE, &args![this, 0xffff_ffffu32]).u32();
        file = e.call(FILE_GET_THREAD_SAFE_FILE, &args![form_file]).u32();
        if !e.call(FILE_FIND_FORM, &args![file, this]).bool() {
            failed = true;
        }
        if !failed && e.call(FILE_GET_TES_FORM, &args![file]).u8() != b'B' {
            failed = true;
        }
        offset = if failed {
            0
        } else {
            e.call(FILE_RECORD_OFFSET, &args![file]).u32()
        };
    } else {
        let world_space = fn_005340b0(e, this);
        if world_space.is_null() {
            return true;
        }
        let found = e.with_stack(8, |e, found_at| {
            let y = fn_00534010(e, this);
            let x = fn_00533fd0(e, this);
            let found = e
                .call(
                    WORLD_SPACE_FIND_LAND_DATA,
                    &args![world_space, x, y, found_at, found_at.byte_add(4)],
                )
                .bool();
            (
                found,
                e.mem.u32(found_at.addr()),
                e.mem.u32(found_at.addr() + 4),
            )
        });
        if !found.0 {
            return true;
        }
        file = found.1;
        offset = found.2;
    }
    if failed {
        e.call(DISABLE_WARNINGS, &args![0u32]);
        let name = if file == 0 {
            UNKNOWN_FILE_NAME
        } else {
            e.call(FILE_GET_NAME, &args![file]).u32()
        };
        let cell = e.call(PARENT_CELL, &args![this]).u32();
        let y = e.call(CELL_GET_DATA_Y, &args![cell]).u32();
        let cell = e.call(PARENT_CELL, &args![this]).u32();
        let x = e.call(CELL_GET_DATA_X, &args![cell]).u32();
        let id = e.call(FORM_ID, &args![this]).u32();
        e.call(LOG_MESSAGE, &args![LAND_LOAD_FAILED, id, x, y, name]);
        e.call(DISABLE_WARNINGS, &args![1u32]);
        return false;
    }
    let file = Ptr::new(file);
    if !e.call(FILE_SET_OFFSET, &args![file, offset]).bool() {
        return false;
    }
    let mut got_heights = false;
    let mut got_normals = false;
    loop {
        let chunk = e.call(FILE_GET_CHUNK, &args![file]).u32();
        match chunk {
            CHUNK_DATA => {
                if fn_00534140(e, this) {
                    e.with_stack(4, |e, value| {
                        e.call(FILE_GET_CHUNK_DATA, &args![file, value, 4u32]);
                        if file_needs_swap(e, file) {
                            e.call(SWAP_WORD, &args![value]);
                        }
                        let low_bits = e.mem.u32(value.addr()) & 7;
                        e.mem.set_u32(value.addr(), low_bits);
                        let flags = e.get(this, TESObjectLAND::Data) & !7;
                        e.set(this, TESObjectLAND::Data, flags);
                        let flags = e.get(this, TESObjectLAND::Data) | low_bits;
                        e.set(this, TESObjectLAND::Data, flags);
                    });
                }
            }
            CHUNK_MOPP => read_mopp_chunk(e, this, file),
            CHUNK_HEIGHTS => {
                if flag(e, this, FLAG_HEIGHTS) {
                    e.with_stack(0x448, |e, buffer| {
                        e.call(FILE_GET_CHUNK_DATA, &args![file, buffer, 0x448u32]);
                        if file_needs_swap(e, file) {
                            e.call(SWAP_WORD, &args![buffer]);
                        }
                        read_height_chunk(e, this, buffer.addr());
                    });
                    got_heights = true;
                }
            }
            CHUNK_NORMALS => {
                if flag(e, this, FLAG_HEIGHTS) {
                    e.with_stack(0xcc3, |e, buffer| {
                        e.call(FILE_GET_CHUNK_DATA, &args![file, buffer, 0xcc3u32]);
                        read_normal_chunk(e, this, buffer.addr());
                    });
                    got_normals = true;
                }
            }
            CHUNK_COLORS => {
                if flag(e, this, FLAG_COLORS) {
                    e.with_stack(0xcc3, |e, buffer| {
                        e.call(FILE_GET_CHUNK_DATA, &args![file, buffer, 0xcc3u32]);
                        read_color_chunk(e, this, buffer.addr());
                    });
                }
            }
            CHUNK_ADDITIONAL_TEXTURE => {
                if flag(e, this, FLAG_TEXTURES) {
                    read_additional_texture_chunk(e, this, file, &CELL_MESSAGES, &mut state);
                }
            }
            CHUNK_BASE_TEXTURE => {
                if flag(e, this, FLAG_TEXTURES) {
                    read_base_texture_chunk(e, this, file, &CELL_MESSAGES, &mut state);
                }
            }
            CHUNK_VERTEX_TEXTURE if flag(e, this, FLAG_TEXTURES) => {
                read_vertex_texture_chunk(e, this, file, &CELL_MESSAGES, false, &mut state);
            }
            _ => {}
        }
        if !e.call(FILE_NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    if state.warned {
        e.call(LAND_FLUSH_WARNINGS, &args![this]);
    }
    if got_heights && !got_normals {
        let name = e.call(FILE_GET_NAME, &args![file]).u32();
        let (x, y) = cell_coordinates(e, this);
        e.call(LOG_MESSAGE, &args![NO_NORMAL_DATA, x, y, name]);
    }
    true
}

// ---- Unloading and meshes ----------------------------------------------------

// Translated from 00536d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectLAND::UnLoadVertices` (Xbox PDB): marks the vertices as not
/// loaded, releases the border lines, and frees the loaded data. For each
/// quadrant it frees the percent block (and its first array), the layer
/// textures, the vertex, normal, colour and normal-set blocks, and empties
/// the grass map (every item holds 16 pointers, each zeroed in its first
/// word and freed; the item is freed too). Then it frees the four block
/// arrays, releases the MOPP code, and deletes the loaded data. Returns
/// true.
pub fn tes_object_land_un_load_vertices(e: &mut Engine, this: Ptr<TESObjectLAND>) -> bool {
    e.call(LAND_SET_LOADED, &args![this, 0u32]);
    e.call(LAND_RELEASE_BORDER, &args![this]);
    let data = loaded_data(e, this);
    if !data.is_null() {
        let at = data.addr();
        // The iteration state the function keeps in locals: position, key
        // and value of the grass map walk.
        e.with_stack(12, |e, frame| {
            let position_cell = frame.addr();
            let key_cell = frame.addr() + 4;
            let value_cell = frame.addr() + 8;
            for quadrant in 0..QUADRANTS {
                let percent_slot = at + LoadedLandData::ppPercentArrays.off + 4 * quadrant;
                let percent = e.mem.u32(percent_slot);
                if percent != 0 {
                    let first = e.mem.u32(percent);
                    e.call(DEALLOCATE, &args![first]);
                    e.call(DEALLOCATE, &args![percent]);
                    e.mem.set_u32(percent_slot, 0);
                }
                let layers_slot = at + LoadedLandData::pQuadTextureArray.off + 4 * quadrant;
                let layers = e.mem.u32(layers_slot);
                if layers != 0 {
                    e.call(DEALLOCATE, &args![layers]);
                    e.mem.set_u32(layers_slot, 0);
                }
                for field in [
                    LoadedLandData::ppVertices.off,
                    LoadedLandData::ppNormals.off,
                    LoadedLandData::ppColorsA.off,
                    LoadedLandData::ppNormalsSet.off,
                ] {
                    let array = e.mem.u32(at + field);
                    if array != 0 {
                        let block = e.mem.u32(array + 4 * quadrant);
                        e.call(DEALLOCATE, &args![block]);
                        e.mem.set_u32(array + 4 * quadrant, 0);
                    }
                }
                let map = at + GRASS_MAP_OFFSET + 0x10 * quadrant;
                if e.call(FORM_ID, &args![map]).u32() > 0 {
                    let first = e.call(MAP_FIRST_POSITION, &args![map]).u32();
                    e.mem.set_u32(position_cell, first);
                    while e.mem.u32(position_cell) != 0 {
                        e.call(
                            MAP_GET_NEXT,
                            &args![map, position_cell, key_cell, value_cell],
                        );
                        let value = e.mem.u32(value_cell);
                        if value != 0 {
                            for item in 0..16u32 {
                                let pointer = e.mem.u32(value + 4 * item);
                                if pointer != 0 {
                                    e.mem.set_u32(pointer, 0);
                                    e.call(DEALLOCATE, &args![pointer]);
                                }
                            }
                            e.call(DEALLOCATE, &args![value]);
                        }
                    }
                }
                e.call(MAP_REMOVE_ALL, &args![map]);
            }
        });
        for field in [
            LoadedLandData::ppNormalsSet.off,
            LoadedLandData::ppVertices.off,
            LoadedLandData::ppColorsA.off,
            LoadedLandData::ppNormals.off,
        ] {
            let array = e.mem.u32(at + field);
            e.call(DEALLOCATE, &args![array]);
        }
        let mopp = e.get(data, LoadedLandData::pMoppCode);
        if !mopp.is_null() {
            e.call(HAVOK_REMOVE_REFERENCE, &args![mopp]);
            e.set(data, LoadedLandData::pMoppCode, Ptr::NULL);
        }
    }
    let data = loaded_data(e, this);
    if !data.is_null() {
        fn_00537100(e, data, 1);
    }
    e.set(this, TESObjectLAND::pLoadedData, Ptr::NULL);
    true
}

// Translated from 00537100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LoadedLandData`'s scalar deleting destructor: runs [`fn_00537130`] and
/// frees the block when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00537100(e: &mut Engine, this: Ptr<LoadedLandData>, flags: u32) -> Ptr<LoadedLandData> {
    fn_00537130(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

// Translated from 00537130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LoadedLandData`'s destructor body: releases the land rigid body, runs
/// the destructors of the four grass maps (16 bytes each), releases the
/// border lines.
pub fn fn_00537130(e: &mut Engine, this: Ptr<LoadedLandData>) {
    e.call(
        SMART_POINTER_RELEASE,
        &args![this.byte_add(LoadedLandData::spLandRB.off)],
    );
    e.call(
        VECTOR_DESTRUCT_ITERATOR,
        &args![
            this.byte_add(GRASS_MAP_OFFSET),
            0x10u32,
            QUADRANTS,
            GRASS_MAP_DESTRUCT
        ],
    );
    e.call(
        SMART_POINTER_RELEASE,
        &args![this.byte_add(LoadedLandData::spBorder.off)],
    );
}

// Translated from 005371b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectLAND::MakeLandTriStrips` (Xbox PDB): builds the `NiTriStrips`
/// of quadrant `block` (0 to 3, with loaded data, else null is returned):
/// copies of the default texture coordinates and of the quadrant's normals,
/// a one-strip list of 1021 indices (the strip length word `0x3fd`, the
/// indices from `0118ae90`), the quadrant's vertices and colours; moves the
/// mesh to the land's world offset and names it `"Block (%i, %i)"` (block
/// mod 4, block / 4).
pub fn tes_object_land_make_land_tri_strips(
    e: &mut Engine,
    this: Ptr<TESObjectLAND>,
    block: u8,
) -> Ptr {
    let data = loaded_data(e, this);
    if block >= 4 || data.is_null() {
        return Ptr::NULL;
    }
    let block = block as u32;
    let coordinates = allocate_vector(e, 0x908, 8, QUADRANT_VERTICES, NI_POINT3_DEFAULT_CONSTRUCT);
    let default_coordinates = e.global::<u32>(DEFAULT_TEXTURE_COORDINATES);
    e.call(
        MEMORY_COPY,
        &args![coordinates, default_coordinates, 0x908u32],
    );
    let lengths = e.call(NI_ALLOCATE, &args![2u32]).u32();
    let indices = e.call(NI_ALLOCATE, &args![0x7fau32]).u32();
    e.mem.set_u16(lengths, 0x3fd);
    e.call(MEMORY_COPY, &args![indices, TRIANGLE_STRIP_LIST, 0x7fau32]);
    let normals = allocate_vector(e, 0xd8c, 12, QUADRANT_VERTICES, NI_POINT3_DEFAULT_CONSTRUCT);
    let source_normals = e.get(data, LoadedLandData::ppNormals);
    let source_normals = element(e, source_normals, block);
    e.call(MEMORY_COPY, &args![normals, source_normals, 0xd8cu32]);
    let memory = e.call(NI_ALLOCATE_OBJECT, &args![0xc4u32]).u32();
    let strips = if memory == 0 {
        0
    } else {
        let vertices = e.get(data, LoadedLandData::ppVertices);
        let vertices = element(e, vertices, block);
        let colors = e.get(data, LoadedLandData::ppColorsA);
        let colors = element(e, colors, block);
        let strip_length = e.mem.u16(lengths) as u32;
        e.call(
            TRI_STRIPS_CONSTRUCT,
            &args![
                memory,
                QUADRANT_VERTICES,
                vertices,
                normals,
                colors,
                coordinates,
                1u32,
                0u32,
                strip_length.wrapping_sub(2),
                1u32,
                lengths,
                indices
            ],
        )
        .u32()
    };
    e.with_stack(12, |e, offset| {
        tes_object_land_get_world_offset_for_block(e, this, offset, block);
        e.call(NODE_SET_TRANSLATE, &args![strips, offset]);
    });
    e.with_stack(0x104, |e, name| {
        e.call(
            FORMAT_STRING,
            &args![name, 0x104u32, BLOCK_NAME_FORMAT, block & 3, block >> 2],
        );
        e.with_stack(4, |e, fixed| {
            let handle = e.call(FIXED_STRING_CONSTRUCT, &args![fixed, name]).u32();
            e.call(OBJECT_SET_NAME, &args![strips, handle]);
            e.call(FIXED_STRING_DESTRUCT, &args![fixed]);
        });
    });
    Ptr::new(strips)
}

// Translated from 005374f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds the four quadrant meshes of the land in its cell's 3D, when the
/// cell has one. Without meshes yet: for each quadrant it takes the cell's
/// node, culls it (`NiAVObject` flag bit 0, `APP_CULLED`), attaches the default texturing property, builds the
/// tri-strips ([`tes_object_land_make_land_tri_strips`]) and attaches them
/// (virtual slot `0xf8`, arguments `0` and the strips), computes the bound
/// from the vertices, marks the geometry data changed, and moves the
/// face-gen root node to the quadrant's centre (the cell corner plus 1024
/// plus the quadrant's 2048 step in x and y, the middle of its height
/// extents in z) after giving it the half height extents as its local
/// translation. Then it builds the Havok cell MOPP (`CellMopp::Create`)
/// from the four meshes and the base height, runs the three follow-up
/// builds, and for each mesh sets the data's consistency (0x4000) and
/// flags, queues the geometry with the renderer when its scheduler's saved
/// acquire object is 1 to 5, restores the node's culled flag, and updates
/// the node and its properties. With meshes: sets each node's culled flag:
/// to true for all four when the land has flag 0x400 and `00537b50(cell)` is
/// false, otherwise per quadrant to what `00544590(cell, quadrant)` says.
///
/// Locals the code constructs but never reads (the translate zero point, the
/// MOPP code pointer copy) are left out; the height extents it keeps in a
/// local are updated but not used.
pub fn fn_005374f0(e: &mut Engine, this: Ptr<TESObjectLAND>) {
    let cell = e.get(this, TESObjectLAND::pParentCell);
    if cell.is_null() || e.call(CELL_GET_3D, &args![cell]).u32() == 0 {
        return;
    }
    let data = loaded_data(e, this);
    if e.get(data, LoadedLandData::ppMesh).is_null() {
        let memory = e.call(ALLOCATE, &args![0x10u32]).ptr();
        e.set(data, LoadedLandData::ppMesh, memory);
        // Locals: the four hidden flags at +0 and the four strips at +0x10.
        e.with_stack(0x20, |e, frame| {
            let hidden = frame.addr();
            let strips = frame.addr() + 0x10;
            e.call(MEMORY_SET, &args![hidden, 0u32, 4u32]);
            e.call(MEMORY_SET, &args![strips, 0u32, 0x10u32]);
            let float_max: f32 = e.global(LARGEST_F32);
            let negative_float_max: f32 = e.global(LOWEST_F32);
            let extents = ni_point2(e, float_max, negative_float_max);
            let mut lowest = f32::from_bits(extents[0]);
            let mut highest = f32::from_bits(extents[1]);
            let meshes = e.get(data, LoadedLandData::ppMesh);
            for quadrant in 0..QUADRANTS {
                let node = fn_00535af0(e, this, quadrant as i32).addr();
                e.mem.set_u32(meshes.addr() + 4 * quadrant, node);
                if element(e, meshes, quadrant) == 0 {
                    continue;
                }
                let node = element(e, meshes, quadrant);
                let was_hidden = e.call(NODE_GET_CULLED, &args![node]).u8();
                e.mem.set_u8(hidden + quadrant, was_hidden);
                let node = element(e, meshes, quadrant);
                e.call(NODE_SET_CULLED, &args![node, 1u32]);
                let property = e
                    .call(SMART_POINTER_GET, &args![DEFAULT_TEXTURING_PROPERTY])
                    .u32();
                let node = element(e, meshes, quadrant);
                e.call(NODE_ATTACH_PROPERTY, &args![node, property]);
                let tri_strips =
                    tes_object_land_make_land_tri_strips(e, this, quadrant as u8).addr();
                e.mem.set_u32(strips + 4 * quadrant, tri_strips);
                let node = element(e, meshes, quadrant);
                e.vcall(node, NODE_ATTACH_CHILD_SLOT, &args![0u32, tri_strips]);
                let vertex_count = e.call(TRI_STRIPS_VERTEX_COUNT, &args![tri_strips]).u16();
                let positions = e.call(GEOMETRY_GET_POSITIONS, &args![tri_strips]).u32();
                let bound = e.call(LAND_BOUND_SOURCE, &args![tri_strips]).u32();
                e.call(
                    BOUND_COMPUTE_FROM_DATA,
                    &args![bound, vertex_count as u32, positions],
                );
                let geometry_data = e.call(GEOMETRY_GET_DATA, &args![tri_strips]).u32();
                e.call(GEOMETRY_DATA_MARK_CHANGED, &args![geometry_data, 0xfu32]);
                let node = element(e, meshes, quadrant);
                let animation = e.call(FACE_GEN_NODE_GET_ANIMATION_DATA, &args![node]).u32();
                let root = e.call(ANIMATION_DATA_GET_ROOT, &args![animation]).u32();
                let (low, high) = e.with_stack(8, |e, extents| {
                    e.call(LAND_QUADRANT_EXTENTS, &args![this, extents, quadrant]);
                    (e.mem.f32(extents.addr()), e.mem.f32(extents.addr() + 4))
                });
                let half_height = ((high as f64 - low as f64) / e.global::<f64>(F64_TWO)) as f32;
                let size: f32 = e.global(F32_1024);
                e.with_stack(12, |e, local| {
                    let point = e
                        .call(NI_POINT3_CONSTRUCT, &args![local, size, size, half_height])
                        .u32();
                    e.call(ROOT_SET_LOCAL_TRANSLATE, &args![root, point]);
                });
                let corner_x = fn_00534050(e, this) + e.global::<f64>(F64_1024);
                let x = (corner_x + ((quadrant % 2) << 11) as f64) as f32;
                let corner_y = fn_00534080(e, this) + e.global::<f64>(F64_1024);
                let y = (corner_y + ((quadrant / 2) << 11) as f64) as f32;
                let z = ((low as f64 + high as f64) / e.global::<f64>(F64_TWO)) as f32;
                e.with_stack(12, |e, translation| {
                    e.mem.set_f32(translation.addr(), x);
                    e.mem.set_f32(translation.addr() + 4, y);
                    e.mem.set_f32(translation.addr() + 8, z);
                    e.vcall(root, ROOT_SET_WORLD_TRANSLATE_SLOT, &args![translation]);
                });
                if low < lowest {
                    lowest = low;
                }
                if highest < high {
                    highest = high;
                }
            }
            let _ = (lowest, highest);
            let cell = e.get(this, TESObjectLAND::pParentCell);
            let cell_mopp = e.call(LAND_CELL_MOPP, &args![cell]).u32();
            e.call(CELL_MOPP_PREPARE, &args![cell_mopp]);
            e.with_stack(0x10, |e, local| {
                e.call(LAND_MOPP_LOCAL_CONSTRUCT, &args![local]);
            });
            let base_height = e.get(data, LoadedLandData::fBaseHeight);
            e.call(
                CELL_MOPP_CREATE,
                &args![cell_mopp, strips, QUADRANTS, base_height],
            );
            for follow_up in LAND_BUILD_FOLLOW_UPS {
                e.call(follow_up, &args![this]);
            }
            for quadrant in 0..QUADRANTS {
                let tri_strips = e.mem.u32(strips + 4 * quadrant);
                if tri_strips != 0 {
                    let geometry_data = e.call(GEOMETRY_GET_DATA, &args![tri_strips]).u32();
                    e.call(
                        GEOMETRY_DATA_SET_CONSISTENCY,
                        &args![geometry_data, 0x4000u32],
                    );
                    e.call(GEOMETRY_DATA_SET_KEEP_FLAGS, &args![geometry_data, 1u32]);
                    e.call(
                        GEOMETRY_DATA_SET_COMPRESS_FLAGS,
                        &args![geometry_data, 0x11u32 | 6],
                    );
                    let scheduler = e.call(GEOMETRY_GET_SCHEDULER, &args![tri_strips]).u32();
                    // Only a scheduler whose saved acquire object (asked twice, as the
                    // code does) is 1 to 5 gets the geometry queued.
                    let accepted = scheduler != 0
                        && e.call(SCHEDULER_GET_SAVED_ACQUIRE, &args![scheduler]).i32() >= 1
                        && e.call(SCHEDULER_GET_SAVED_ACQUIRE, &args![scheduler]).i32() <= 5;
                    if accepted {
                        let renderer = e.call(RENDERER_GET, &[]).u32();
                        let task = e.call(LAND_RENDER_TASK, &args![scheduler]).u32();
                        e.vcall(
                            renderer,
                            RENDERER_QUEUE_SLOT,
                            &args![tri_strips, 0u32, 0u32, task],
                        );
                        e.call(RENDERER_AFTER_QUEUE, &args![renderer]);
                    }
                }
                let node = element(e, meshes, quadrant);
                if node != 0 {
                    let was_hidden = e.mem.u8(hidden + quadrant) as u32;
                    e.call(NODE_SET_CULLED, &args![node, was_hidden]);
                    e.with_stack(0x10, |e, update| {
                        e.call(UPDATE_DATA_CONSTRUCT, &args![update, 0.0f32, 0u32, 0u32]);
                        e.call(NODE_UPDATE, &args![node, update]);
                    });
                    e.call(NODE_UPDATE_PROPERTIES, &args![node]);
                }
            }
        });
    }
    let meshes = e.get(data, LoadedLandData::ppMesh);
    if meshes.is_null() {
        return;
    }
    if e.call(PARENT_CELL, &args![this]).u32() == 0 {
        return;
    }
    let mut hide_all = false;
    if fn_00534140(e, this) {
        let cell = e.call(PARENT_CELL, &args![this]).u32();
        hide_all = !e.call(CELL_REFERENCE_LIST_CHECK, &args![cell]).bool();
    }
    for quadrant in 0..QUADRANTS {
        let hide = if hide_all {
            true
        } else {
            let cell = e.call(PARENT_CELL, &args![this]).u32();
            e.call(CELL_QUADRANT_TEST, &args![cell, quadrant]).bool()
        };
        let node = element(e, meshes, quadrant);
        e.call(NODE_SET_CULLED, &args![node, hide as u32]);
    }
}

// ---- Second session: small functions, Havok MOPP code, save ---------------

/// `FISTP` with the truncating rounding mode the compiler's cast sets: the
/// integer of `value`, or `i32::MIN` (the "integer indefinite" value) when
/// it does not fit.
fn x87_truncate(value: f64) -> i32 {
    if value.is_nan() || value >= 2_147_483_648.0 || value <= -2_147_483_649.0 {
        i32::MIN
    } else {
        value as i32
    }
}

/// Whether the save writer targets a big-endian file (`00401500`).
fn is_big_endian(e: &mut Engine) -> bool {
    e.call(IS_BIG_ENDIAN, &args![]).bool()
}

// Translated from 00537b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The object `00460140` gives for the smart pointer at `this + 0xb8` (the
/// strips' bound, see `fn_005374f0`).
pub fn fn_00537b10(e: &mut Engine, this: Ptr) -> Ptr {
    let held = e.call(SMART_POINTER_GET, &args![this.byte_add(0xb8)]).u32();
    e.call(GETTER_00460140, &args![held]).ptr()
}

// Translated from 00537b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The result of `0041bb10` on the member at `this + 0x28` (a cell's MOPP
/// holder, see `fn_005374f0`).
pub fn fn_00537b30(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(GETTER_0041BB10, &args![this.byte_add(0x28)]).ptr()
}

// Translated from 00537b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `008256d0` returns 0 for the member at `this + 0xac` of a cell
/// (the check on the cell's reference list, see `fn_005374f0`).
pub fn fn_00537b50(e: &mut Engine, this: Ptr) -> bool {
    !e.call(LIST_CHECK, &args![this.byte_add(0xac)]).bool()
}

// Translated from 00537b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the local `fn_005374f0` keeps: runs `00537bb0`, then
/// sets +8 to 0 and +0xc to 1.0. Returns `this`.
pub fn fn_00537b80(e: &mut Engine, this: Ptr) -> Ptr {
    fn_00537bb0(e, this);
    e.mem.set_u32(this.addr() + 8, 0);
    e.mem.set_f32(this.addr() + 0xc, 1.0);
    this
}

// Translated from 00537bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs the base constructor `004b5020` and clears +4. Returns `this`.
pub fn fn_00537bb0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_CONSTRUCT_004B5020, &args![this]);
    e.mem.set_u32(this.addr() + 4, 0);
    this
}

// Translated from 00537bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The value of the smart pointer at `this + 0x78` (the render task of a
/// scheduler, see `fn_005374f0`).
pub fn fn_00537bd0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(SMART_POINTER_GET, &args![this.byte_add(0x78)]).ptr()
}

// Translated from 00537bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAdditionalGeometryData::NiAdditionalGeometryData` (Xbox PDB): the
/// `NiObject` constructor, the vtable, the member at +0x1c (constructed
/// with `(0, 1)` and again with `005e03d0`), the count `value` at +0xc and
/// zeroes at +8, +0x10, +0x14 and +0x18. Returns `this`. (The unwinding
/// frame is not translated.)
pub fn ni_additional_geometry_data_ni_additional_geometry_data(
    e: &mut Engine,
    this: Ptr,
    value: u16,
) -> Ptr {
    e.call(NI_OBJECT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), ADDITIONAL_GEOMETRY_DATA_VTABLE);
    e.call(MEMBER_CONSTRUCT, &args![this.byte_add(0x1c), 0u32, 1u32]);
    e.mem.set_u32(this.addr() + 8, 0);
    e.mem.set_u16(this.addr() + 0xc, value);
    e.mem.set_u32(this.addr() + 0x10, 0);
    e.mem.set_u32(this.addr() + 0x14, 0);
    e.call(MEMBER_CONSTRUCT_AGAIN, &args![this.byte_add(0x1c)]);
    e.mem.set_u32(this.addr() + 0x18, 0);
    this
}

// Translated from 00537ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAdditionalGeometryData::GetRTTI` (Xbox PDB): the class's `NiRTTI`.
pub fn ni_additional_geometry_data_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(ADDITIONAL_GEOMETRY_DATA_RTTI)
}

// Translated from 00537cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAdditionalGeometryData::_scalar_deleting_destructor_` (Xbox PDB):
/// runs the destructor body `00a73080` and frees the 0x2C bytes when bit 0
/// of `flags` is set. Returns `this`.
pub fn ni_additional_geometry_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(ADDITIONAL_GEOMETRY_DATA_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_FREE, &args![this, 0x2cu32]);
    }
    this
}

// Translated from 00537ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `00540800` on `this` (the decompiler names it after a library
/// destructor; it is a function of this unit that a later session
/// translates).
pub fn fn_00537ce0(e: &mut Engine, this: Ptr) {
    e.call(MEMBER_DESTRUCT, &args![this]);
}

// Translated from 00537d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPackedAdditionalGeometryData::GetRTTI` (Xbox PDB): the class's
/// `NiRTTI`.
pub fn bs_packed_additional_geometry_data_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(PACKED_ADDITIONAL_GEOMETRY_DATA_RTTI)
}

// Translated from 00537d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPackedAdditionalGeometryData::_scalar_deleting_destructor_` (Xbox
/// PDB): runs the destructor `00537d40` and frees the 0x34 bytes when bit 0
/// of `flags` is set. Returns `this`.
pub fn bs_packed_additional_geometry_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00537d40(e, this);
    if flags & 1 != 0 {
        e.call(NI_FREE, &args![this, 0x34u32]);
    }
    this
}

// Translated from 00537d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of `BSPackedAdditionalGeometryData` (by its caller's Xbox PDB
/// name): stores its vtable and runs the base destructor body `00a73080`.
pub fn fn_00537d40(e: &mut Engine, this: Ptr) {
    e.mem
        .set_u32(this.addr(), PACKED_ADDITIONAL_GEOMETRY_DATA_VTABLE);
    e.call(ADDITIONAL_GEOMETRY_DATA_DESTRUCT, &args![this]);
}

// Translated from 00537eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases what the loaded data of a land holds in the scene and physics:
/// clears the queued textures, hands the rigid body back to the cell's
/// physics object (when there is one that holds it) and releases it, then
/// for each quadrant with a mesh drops the vertex, colour and normal
/// arrays, removes the node from the shadow scene node and detaches it
/// (slot `0xf0`), frees the mesh array, and finally lets the border
/// object's owner (slot `0xe8`) release the border and clears it.
pub fn fn_00537eb0(e: &mut Engine, this: Ptr<TESObjectLAND>) {
    e.call(
        QUEUED_FILE_POINTER_SET,
        &args![this.byte_add(TESObjectLAND::spQueuedTextures.off), 0u32],
    );
    let data = loaded_data(e, this);
    if data.is_null() {
        return;
    }
    let meshes = e.get(data, LoadedLandData::ppMesh);
    if !meshes.is_null() {
        let cell = e.get(this, TESObjectLAND::pParentCell);
        let physics = if cell.is_null() {
            0
        } else {
            e.call(CELL_PHYSICS_OBJECT, &args![cell]).u32()
        };
        let body_slot = data.byte_add(LoadedLandData::spLandRB.off);
        if physics != 0
            && e.call(SLOT_HOLDS_OTHER_THAN, &args![body_slot, 0u32])
                .bool()
        {
            let body = e.call(SMART_POINTER_GET, &args![body_slot]).u32();
            e.call(PHYSICS_OBJECT_ADD_BODY, &args![physics, body]);
        }
        e.call(TEXTURING_PROPERTY_SET, &args![body_slot, 0u32]);
        for quadrant in 0..QUADRANTS {
            let node = element(e, meshes, quadrant);
            if node != 0 {
                let vertices = e.get(data, LoadedLandData::ppVertices);
                e.mem.set_u32(vertices.addr() + 4 * quadrant, 0);
                let colors = e.get(data, LoadedLandData::ppColorsA);
                e.mem.set_u32(colors.addr() + 4 * quadrant, 0);
                let normals = e.get(data, LoadedLandData::ppNormals);
                let block = element(e, normals, quadrant);
                e.call(DEALLOCATE, &args![block]);
                e.mem.set_u32(normals.addr() + 4 * quadrant, 0);
                let manager = e.call(TABLE_ENTRY, &args![0u32]).u32();
                e.call(SHADOW_SCENE_NODE_REMOVE_OBJECT, &args![manager, node]);
                e.vcall(node, NODE_DETACH_SLOT, &args![0u32]);
            }
            e.mem.set_u32(meshes.addr() + 4 * quadrant, 0);
        }
        e.call(DEALLOCATE, &args![meshes]);
        e.set(data, LoadedLandData::ppMesh, Ptr::NULL);
    }
    let border_slot = data.byte_add(LoadedLandData::spBorder.off);
    if e.call(SMART_POINTER_GET, &args![border_slot]).u32() != 0 {
        let border = e.call(SMART_POINTER_GET, &args![border_slot]).u32();
        let owner = e.call(OBJECT_FIELD_0X18, &args![border]).u32();
        if owner != 0 {
            let border = e.call(SMART_POINTER_GET, &args![border_slot]).u32();
            e.vcall(owner, BORDER_OWNER_SLOT, &args![border]);
        }
        e.call(TEXTURING_PROPERTY_SET, &args![border_slot, 0u32]);
    }
}

// Translated from 00538110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectLAND::Save` (Xbox PDB): writes the land record. Needs one of
/// the data flags 1, 2 or 4 and either the "loaded" bit (8) or all of the
/// vertex, normal and colour arrays; loads the data first when the flag
/// test `fn_005341e0` fails. Opens the form and writes `DATA` (the flags).
/// With flag 1 it writes `VNML` (the normals, normalized in place, as three
/// signed bytes of a 33 x 33 grid) and `VHGT` (the base height as a float,
/// then the 33 x 33 height steps as signed bytes, clamped to +-127;
/// clamping is reported in the log and the later heights follow the
/// clamped steps). With flag 2 it writes `VCLR` (the colours as bytes).
/// With flag 4 it writes, per quadrant, `BTXT` (the quadrant's own base
/// texture) and per layer `ATXT` (layer texture, 0 for the default) and
/// `VTXT` (the position and opacity of each vertex with a non-zero
/// opacity). Each quadrant fills the 17 x 17 block that starts at
/// `(q / 2) * 16 * 33 + (q % 2) * 16` of the grid and the shared edges are
/// written once. Records are byte swapped around the write when the writer
/// is big-endian. Ends with `CloseForm` and `CompressSaveBuffer`.
///
/// The padding bytes the game leaves uninitialized on its stack (the 3
/// bytes after the 33 x 33 steps of `VHGT`, the byte at +5 of a `BTXT` or
/// `ATXT` record and the 2 bytes at +2 of a `VTXT` entry) are written as 0.
pub fn tes_object_land_save(e: &mut Engine, this: Ptr<TESObjectLAND>) {
    if !fn_00539460(e, this) {
        return;
    }
    let data = loaded_data(e, this);
    let has_arrays = !data.is_null()
        && !e.get(data, LoadedLandData::ppVertices).is_null()
        && !e.get(data, LoadedLandData::ppNormals).is_null()
        && !e.get(data, LoadedLandData::ppColorsA).is_null();
    if e.get(this, TESObjectLAND::Data) & FLAG_LOADED == 0 && !has_arrays {
        return;
    }
    if !fn_005341e0(e, this) {
        e.call(LAND_LOAD_FOR_SAVE, &args![this]);
    }
    e.call(FORM_START, &args![this]);
    let flags_word = this.byte_add(TESObjectLAND::Data.off);
    if is_big_endian(e) {
        e.call(SWAP_WORD, &args![flags_word]);
    }
    e.call(FORM_ADD_CHUNK_DATA, &args![CHUNK_DATA, flags_word, 4u32]);
    if is_big_endian(e) {
        e.call(SWAP_WORD, &args![flags_word]);
    }
    if e.get(this, TESObjectLAND::Data) & FLAG_HEIGHTS != 0 {
        save_heights_and_normals(e, this);
    }
    if e.get(this, TESObjectLAND::Data) & FLAG_COLORS != 0 {
        save_colors(e, this);
    }
    if e.get(this, TESObjectLAND::Data) & FLAG_TEXTURES != 0 {
        save_textures(e, this);
    }
    e.call(FORM_CLOSE, &args![this]);
    e.call(FORM_COMPRESS_SAVE_BUFFER, &args![]);
}

/// The cells of the 33 x 33 grid that quadrant `quadrant` writes: for each
/// of its 17 x 17 vertices `(vertex index, grid index)`, without the shared
/// edge vertices the neighbouring quadrant writes (the last row of the
/// upper quadrants and the last column of the left ones). The column test
/// runs after the row test and undoes it, so quadrant 1 also keeps its
/// corner vertex (last row, last column), which quadrant 3 writes again.
fn quadrant_grid_cells(quadrant: i32) -> Vec<(u32, usize)> {
    let mut cells = Vec::new();
    for vertex in 0..QUADRANT_VERTICES as i32 {
        let row = vertex / 0x11;
        let column = vertex % 0x11;
        let grid = row * 0x21 + (quadrant / 2) * 16 * 0x21 + (quadrant % 2) * 16 + column;
        let mut skip = false;
        if row == 0x10 {
            skip = true;
            if quadrant / 2 == 1 {
                skip = false;
            }
        }
        if column == 0x10 {
            skip = true;
            if quadrant % 2 == 1 {
                skip = false;
            }
        }
        if !skip {
            cells.push((vertex as u32, grid as usize));
        }
    }
    cells
}

/// The `VNML` and `VHGT` chunks of `Save`.
fn save_heights_and_normals(e: &mut Engine, this: Ptr<TESObjectLAND>) {
    let data = loaded_data(e, this);
    let vertices = e.get(data, LoadedLandData::ppVertices);
    let normals = e.get(data, LoadedLandData::ppNormals);
    let mut heights = vec![0i32; CHUNK_VERTICES];
    let mut normal_bytes = vec![0u8; CHUNK_VERTICES * 3];
    for quadrant in 0..QUADRANTS as i32 {
        for (vertex, grid) in quadrant_grid_cells(quadrant) {
            let position = element(e, vertices, quadrant as u32) + vertex * 12;
            let height = e.mem.f32(position + 8);
            heights[grid] = e.call(FLOAT_TO_INT, &args![height]).i32() >> 3;
            let normal = element(e, normals, quadrant as u32) + vertex * 12;
            e.call(NI_POINT3_NORMALIZE, &args![normal]);
            for axis in 0..3 {
                let scaled = e.mem.f32(normal + 4 * axis) as f64 * e.global::<f64>(F64_127);
                normal_bytes[grid * 3 + axis as usize] = e.call(FTOL2, &args![scaled]).u32() as u8;
            }
        }
    }
    e.with_stack(0xcc3, |e, buffer| {
        e.mem.write(buffer.addr(), &normal_bytes);
        e.call(
            FORM_ADD_CHUNK_ARRAY,
            &args![CHUNK_NORMALS, buffer, 0xcc3u32],
        );
    });
    let mut previous = heights[0];
    let base = heights[0] as f32;
    let mut clamped = false;
    let mut steps = vec![0u8; CHUNK_VERTICES];
    for index in 0..CHUNK_VERTICES {
        let difference = heights[index].wrapping_sub(previous);
        let magnitude = e.call(INT_ABS, &args![difference]).i32();
        if magnitude >= 0x80 {
            clamped = true;
            steps[index] = if heights[index] > previous {
                0x7f
            } else {
                0x81
            };
        } else {
            steps[index] = difference as u8;
        }
        if (index + 1) % 0x21 == 0 {
            previous = heights[index - 32];
        } else if !clamped {
            previous = heights[index];
        } else {
            previous = (steps[index] as i8 as i32).wrapping_add(previous);
        }
    }
    e.with_stack(0x448, |e, chunk| {
        e.mem.set_f32(chunk.addr(), base);
        e.mem.write(chunk.addr() + 4, &steps);
        if is_big_endian(e) {
            e.call(SWAP_WORD, &args![chunk]);
        }
        e.call(FORM_ADD_CHUNK_DATA, &args![CHUNK_HEIGHTS, chunk, 0x448u32]);
        if is_big_endian(e) {
            e.call(SWAP_WORD, &args![chunk]);
        }
    });
    if clamped {
        let (x, y) = cell_coordinates(e, this);
        e.call(LOG_MESSAGE, &args![SAVE_HEIGHT_ERROR, x, y]);
    }
}

/// The `VCLR` chunk of `Save`: the red, green and blue of each vertex as
/// bytes (the float times 255, truncated).
fn save_colors(e: &mut Engine, this: Ptr<TESObjectLAND>) {
    let data = loaded_data(e, this);
    let colors = e.get(data, LoadedLandData::ppColorsA);
    let mut bytes = vec![0u8; CHUNK_VERTICES * 3];
    for quadrant in 0..QUADRANTS as i32 {
        for (vertex, grid) in quadrant_grid_cells(quadrant) {
            let color = element(e, colors, quadrant as u32) + vertex * 16;
            for axis in 0..3 {
                let scaled = e.mem.f32(color + 4 * axis) as f64 * e.global::<f64>(F64_255);
                bytes[grid * 3 + axis as usize] = x87_truncate(scaled) as u8;
            }
        }
    }
    e.with_stack(0xcc3, |e, buffer| {
        e.mem.write(buffer.addr(), &bytes);
        e.call(FORM_ADD_CHUNK_ARRAY, &args![CHUNK_COLORS, buffer, 0xcc3u32]);
    });
}

/// Writes one 8-byte `BTXT` or `ATXT` record (form id, quadrant, layer),
/// byte swapped around the write when the writer is big-endian.
fn save_texture_record(e: &mut Engine, tag: u32, form_id: u32, quadrant: u32, layer: u16) {
    e.with_stack(8, |e, record| {
        e.mem.set_u32(record.addr(), form_id);
        e.mem.set_u8(record.addr() + 4, quadrant as u8);
        e.mem.set_u16(record.addr() + 6, layer);
        if is_big_endian(e) {
            fn_00535a60(e, record);
        }
        e.call(FORM_ADD_CHUNK_DATA, &args![tag, record, 8u32]);
        if is_big_endian(e) {
            fn_00535a60(e, record);
        }
    });
}

/// The word at `field + 4 * index` of the land's loaded data.
fn data_slot(e: &mut Engine, this: Ptr<TESObjectLAND>, field: u32, index: u32) -> u32 {
    let data = loaded_data(e, this);
    e.mem.u32(data.addr() + field + 4 * index)
}

/// The `BTXT`, `ATXT` and `VTXT` chunks of `Save`.
fn save_textures(e: &mut Engine, this: Ptr<TESObjectLAND>) {
    e.call(LAND_FLUSH_WARNINGS, &args![this]);
    for quadrant in 0..QUADRANTS {
        let texture = data_slot(e, this, LoadedLandData::pDefQuadTexture.off, quadrant);
        if texture != 0 {
            let default_texture = fn_00535ae0(e).addr();
            if texture != default_texture {
                let form_id = e.call(FORM_ID, &args![texture]).u32();
                save_texture_record(e, CHUNK_BASE_TEXTURE, form_id, quadrant, 0xffff);
            }
        }
        let layers = data_slot(e, this, LoadedLandData::pQuadTextureArray.off, quadrant);
        if layers == 0 {
            continue;
        }
        for layer in 0..6u32 {
            let texture = e.mem.u32(layers + 4 * layer);
            if texture == 0 {
                continue;
            }
            let default_texture = fn_00535ae0(e).addr();
            let form_id = if texture == default_texture {
                0
            } else {
                e.call(FORM_ID, &args![texture]).u32()
            };
            save_texture_record(e, CHUNK_ADDITIONAL_TEXTURE, form_id, quadrant, layer as u16);
            let mut entries: Vec<(u16, f32)> = Vec::new();
            for vertex in 0..QUADRANT_VERTICES {
                if data_slot(e, this, LoadedLandData::ppPercentArrays.off, quadrant) != 0 {
                    let opacity = e
                        .call(LAND_LAYER_OPACITY, &args![this, quadrant, vertex, layer])
                        .f32();
                    if opacity > 0.0 {
                        entries.push((vertex as u16, opacity));
                    }
                }
            }
            if entries.is_empty() {
                continue;
            }
            let count = entries.len() as u32;
            e.with_stack(count * 8, |e, buffer| {
                for (i, (position, opacity)) in entries.iter().enumerate() {
                    let entry = buffer.addr() + 8 * i as u32;
                    e.mem.set_u16(entry, *position);
                    e.mem.set_f32(entry + 4, *opacity);
                }
                if is_big_endian(e) {
                    for i in 0..count {
                        fn_00535aa0(e, buffer.byte_add(8 * i));
                    }
                }
                e.call(
                    FORM_ADD_CHUNK_DATA,
                    &args![CHUNK_VERTEX_TEXTURE, buffer, count * 8],
                );
                if is_big_endian(e) {
                    for i in 0..count {
                        fn_00535aa0(e, buffer.byte_add(8 * i));
                    }
                }
            });
        }
    }
}

// Translated from 00538c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds the Havok MOPP code object of a land from `size` bytes at `data`
/// and stores it at `out`: allocates the 0x30-byte object and constructs it
/// (`fn_00538d70`), copies the first 0x10 bytes to +0x10 and the rest into
/// the array at +0x20 (sized first). Needs `data`, `size` and `out`, and
/// more than 0x10 bytes; otherwise (or when nothing is left for the array)
/// the object is destroyed through its vtable and `out` is cleared.
/// Returns whether the object was built. `_unused_0` is the unused `this`.
pub fn fn_00538c00(e: &mut Engine, _unused_0: Ptr, data: u32, size: u32, out: Ptr) -> bool {
    let mut built = false;
    if data != 0 && size != 0 && !out.is_null() {
        let memory = e.call(MOPP_ALLOCATE, &args![0x30u32]).u32();
        let object = if memory == 0 {
            0
        } else {
            fn_00538d70(e, Ptr::new(memory)).addr()
        };
        e.mem.set_u32(out.addr(), object);
        if size > 0x10 {
            e.call(MEMORY_COPY, &args![object + 0x10, data, 0x10u32]);
            let remaining = size - 0x10;
            if remaining != 0 {
                e.call(MOPP_ARRAY_SET_SIZE, &args![object + 0x20, remaining]);
                let buffer = e.call(SMART_POINTER_GET, &args![object + 0x20]).u32();
                e.call(MEMORY_COPY, &args![buffer, data + 0x10, remaining]);
                built = true;
            }
        }
        if !built {
            if object != 0 {
                e.vcall(object, 0, &args![1u32]);
            }
            e.mem.set_u32(out.addr(), 0);
        }
    }
    built
}

// Translated from 00538d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the Havok MOPP code object (the class whose vtable is
/// `0102e368`; `hkpMoppCode` by the Xbox PDB name of its deleting
/// destructor `00538f80`): the base constructor, the vtable, the members at
/// +0x10, +0x20 and +0x2c, the four words at +0x10 cleared and the member at
/// +0x2c finished with 2. Returns `this`. (The unwinding frame is not
/// translated.)
pub fn fn_00538d70(e: &mut Engine, this: Ptr) -> Ptr {
    fn_00538ef0(e, this);
    e.mem.set_u32(this.addr(), HAVOK_MOPP_CODE_VTABLE);
    e.call(MOPP_MEMBER_CONSTRUCT_A, &args![this.byte_add(0x10)]);
    e.call(MOPP_MEMBER_CONSTRUCT_B, &args![this.byte_add(0x20)]);
    // `006815c0` is also the `NiPoint3` default constructor of the vector
    // constructor iterator (identical code folded).
    e.call(NI_POINT3_DEFAULT_CONSTRUCT, &args![this.byte_add(0x2c)]);
    fn_00538f40(e, this.byte_add(0x10));
    e.call(MOPP_MEMBER_FINISH, &args![this.byte_add(0x2c), 2u32]);
    this
}

// Translated from 00538e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of the referenced-object base: stores its vtable and
/// runs `fn_00538e30`.
pub fn fn_00538e10(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), HAVOK_REFERENCED_OBJECT_VTABLE);
    fn_00538e30(e, this);
}

// Translated from 00538e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of the Havok base object: stores its vtable.
pub fn fn_00538e30(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), HAVOK_BASE_OBJECT_VTABLE);
}

// Translated from 00538e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBaseObject::_scalar_deleting_destructor_` (Xbox PDB): the destructor
/// body `fn_00538e30`, then the allocator's free when bit 0 of `flags` is
/// set. Returns `this`.
pub fn hk_base_object_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00538e30(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

// Translated from 00538e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the referenced-object base (vtable
/// `0102e378`): the destructor body `fn_00538e10`, then the Havok free
/// `fn_00538eb0` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00538e80(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00538e10(e, this);
    if flags & 1 != 0 {
        fn_00538eb0(e, this);
    }
    this
}

// Translated from 00538eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Gives the object back to the Havok allocator: asks the allocator object
/// (`00c85750` through `0044edb0`) to free `object`, whose size is the
/// 16-bit word at +4 (slot 8 of the allocator's vtable, `(object, size)`).
pub fn fn_00538eb0(e: &mut Engine, object: Ptr) {
    let router = e.call(HAVOK_MEMORY_ROUTER, &args![]).u32();
    let allocator = e.call(HAVOK_ALLOCATOR_OF_ROUTER, &args![router]).u32();
    let size = e.mem.u16(object.addr() + 4) as u32;
    e.vcall(allocator, 8, &args![object, size]);
}

// Translated from 00538ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the referenced-object base: the Havok base constructor
/// `fn_00538f20`, the vtable `0102e378` and the 16-bit word at +6 (the
/// reference count) set to 1. Returns `this`.
pub fn fn_00538ef0(e: &mut Engine, this: Ptr) -> Ptr {
    fn_00538f20(e, this);
    e.mem.set_u32(this.addr(), HAVOK_REFERENCED_OBJECT_VTABLE);
    e.mem.set_u16(this.addr() + 6, 1);
    this
}

// Translated from 00538f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the Havok base object: stores its vtable. Returns `this`.
pub fn fn_00538f20(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), HAVOK_BASE_OBJECT_VTABLE);
    this
}

// Translated from 00538f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the 16 bytes at `this` (the code stores a zeroed `XMM` register).
pub fn fn_00538f40(e: &mut Engine, this: Ptr) {
    for word in 0..4 {
        e.mem.set_u32(this.addr() + 4 * word, 0);
    }
}

// Translated from 00538f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppCode::_scalar_deleting_destructor_` (Xbox PDB): the destructor
/// `fn_00538fb0`, then the Havok free `fn_00538eb0` when bit 0 of `flags`
/// is set. Returns `this`.
pub fn hkp_mopp_code_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00538fb0(e, this);
    if flags & 1 != 0 {
        fn_00538eb0(e, this);
    }
    this
}

// Translated from 00538fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the Havok MOPP code object: stores its vtable `0102e368`,
/// destroys the array at +0x20 and runs `fn_00538e10`. (The decompiler
/// names it after a library destructor; the unwinding frame is not
/// translated.)
pub fn fn_00538fb0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), HAVOK_MOPP_CODE_VTABLE);
    e.call(MOPP_ARRAY_DESTRUCT, &args![this.byte_add(0x20)]);
    fn_00538e10(e, this);
}

// Translated from 00539010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs the form method `00484730` with `flag`, and when `flag` is set and
/// the land has a parent cell, calls slot `0xc8` of that cell's vtable
/// with 1.
pub fn fn_00539010(e: &mut Engine, this: Ptr<TESObjectLAND>, flag: u8) {
    e.call(FORM_FLAG_SETTER, &args![this, flag]);
    let cell = e.get(this, TESObjectLAND::pParentCell);
    if flag != 0 && !cell.is_null() {
        e.vcall(cell.addr(), CELL_SLOT_MARK, &args![1u32]);
    }
}

// Translated from 00539060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot `0x3c` of the land's vtable: whether the land saves `form`. For
/// the reference kinds `005548a0` accepts, the form is cast to a
/// `TESChildCell`; when its parent cell (slot 0) is the land's own, the
/// answer is 1 for the types `0x3a` to `0x40` and `0x69` when the form is a
/// `TESObjectREFR` that does not persist (`GetRefPersists`), 1 for type
/// `0x43` and 0 otherwise; when it differs, slot `0x3c` of the land's
/// parent cell decides, asked about that other cell. For other types slot
/// `0x3c` of the parent cell decides, asked about `form`.
pub fn fn_00539060(e: &mut Engine, this: Ptr<TESObjectLAND>, form: u32) -> u8 {
    let child_cell = this.byte_add(CHILD_CELL_OFFSET).addr();
    let kind = e.call(FORM_TYPE, &args![form]).u32();
    if !e.call(TYPE_IS_REFERENCE_KIND, &args![kind]).bool() {
        let cell = e.vcall(child_cell, CHILD_CELL_PARENT_SLOT, &args![]).u32();
        return e.vcall(cell, CELL_SLOT_REFERENCE_CHECK, &args![form]).u8();
    }
    let owner = form_cast(e, form, RTTI_TES_CHILD_CELL);
    let other_cell = e.vcall(owner, 0, &args![]).u32();
    let own_cell = e.vcall(child_cell, CHILD_CELL_PARENT_SLOT, &args![]).u32();
    if other_cell != own_cell {
        let cell = e.vcall(child_cell, CHILD_CELL_PARENT_SLOT, &args![]).u32();
        return e
            .vcall(cell, CELL_SLOT_REFERENCE_CHECK, &args![other_cell])
            .u8();
    }
    let kind = e.call(FORM_TYPE, &args![form]).u32();
    match kind {
        0x3a..=0x40 | 0x69 => {
            let reference = form_cast(e, form, RTTI_TES_OBJECT_REFR);
            if reference != 0 && !e.call(GET_REF_PERSISTS, &args![reference]).bool() {
                1
            } else {
                0
            }
        }
        0x43 => 1,
        _ => 0,
    }
}

// Translated from 005391d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot `0x38` of the land's vtable, on a save reference record
/// `reference` (marker word at +0, id at +8, kind at +0xc). Ignores a
/// record without the marker. For kinds 8 and 9 it looks the form up by the
/// id, casts it to a `TESObjectCELL` and compares it with the land's parent
/// cell (the answer is 0 either way); for other kinds slot `0x38` of the
/// parent cell decides.
pub fn fn_005391d0(e: &mut Engine, this: Ptr<TESObjectLAND>, reference: Ptr) -> u8 {
    if reference.is_null() || e.mem.u32(reference.addr()) != e.global::<u32>(SAVE_REFERENCE_MARKER)
    {
        return 0;
    }
    let kind = e.mem.u32(reference.addr() + 0xc);
    if (8..=9).contains(&kind) {
        let id = e.mem.u32(reference.addr() + 8);
        let form = e.call(FORM_LOOKUP_BY_ID, &args![id]).u32();
        let cell = form_cast(e, form, RTTI_TES_OBJECT_CELL);
        if cell != 0 {
            let _same = e.call(PARENT_CELL, &args![this]).u32() == cell;
        }
        0
    } else {
        let cell = e.call(PARENT_CELL, &args![this]).u32();
        e.vcall(cell, CELL_SLOT_REFERENCE_OWNED, &args![reference])
            .u8()
    }
}

// Translated from 00539280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot `0x110` of the land's vtable, on a save reference record
/// `reference` and two flag bytes. Ignores a record without the marker.
/// Kind 6 needs `flag`, kinds 8 and 9 do not; for those, the answer is 1
/// when `00485be0` says the id belongs to the parent cell (0 for kind 8).
/// Other kinds need `flag` too and slot `0x110` of the parent cell decides
/// (with `reference`, `flag` and `second_flag`).
pub fn fn_00539280(
    e: &mut Engine,
    this: Ptr<TESObjectLAND>,
    reference: Ptr,
    flag: u8,
    second_flag: u8,
) -> u8 {
    if reference.is_null() || e.mem.u32(reference.addr()) != e.global::<u32>(SAVE_REFERENCE_MARKER)
    {
        return 0;
    }
    let cell = e.call(PARENT_CELL, &args![this]).u32();
    let kind = e.mem.u32(reference.addr() + 0xc);
    match kind {
        6 => {
            if flag == 0 {
                return 0;
            }
        }
        8 | 9 => {}
        _ => {
            if flag == 0 {
                return 0;
            }
            return e
                .vcall(
                    cell,
                    CELL_SLOT_SAVE_CHECK,
                    &args![reference, flag, second_flag],
                )
                .u8();
        }
    }
    let id = e.mem.u32(reference.addr() + 8);
    if e.call(FORM_ID_IN_CELL_FILE, &args![cell, id]).bool() {
        let kind = e.mem.u32(reference.addr() + 0xc);
        if kind == 8 {
            0
        } else {
            1
        }
    } else {
        0
    }
}

// Translated from 00539360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot `0x114` of the land's vtable: fills the save reference record
/// `out` for the record `reference`. Clears the marker of `out` first. For
/// kinds 3 and 5, when the id (+8) of `reference` is the cell's save id
/// (`00544210`), `out` becomes kind 6 with the parent cell's form id; for
/// kind 6, when the id is the parent cell's form id, `out` becomes kind 9
/// with the form id of the parent cell. The other words (+4 and +0x10) are
/// cleared.
pub fn fn_00539360(e: &mut Engine, this: Ptr<TESObjectLAND>, out: Ptr, reference: Ptr) {
    if out.is_null() {
        return;
    }
    e.mem.set_u32(out.addr(), 0);
    if reference.is_null() {
        return;
    }
    let cell = e.call(PARENT_CELL, &args![this]).u32();
    let parent = e.call(PARENT_CELL, &args![this]).u32();
    let form_id = e.call(FORM_ID, &args![parent]).u32();
    let kind = e.mem.u32(reference.addr() + 0xc);
    match kind {
        3 | 5 => {
            let save_id = e.call(CELL_SAVE_ID, &args![cell]).u32();
            if e.mem.u32(reference.addr() + 8) == save_id {
                let marker = e.global::<u32>(SAVE_REFERENCE_MARKER);
                e.mem.set_u32(out.addr(), marker);
                e.mem.set_u32(out.addr() + 0xc, 6);
                e.mem.set_u32(out.addr() + 8, form_id);
                e.mem.set_u32(out.addr() + 4, 0);
                e.mem.set_u32(out.addr() + 0x10, 0);
            }
        }
        6 if e.mem.u32(reference.addr() + 8) == form_id => {
            let marker = e.global::<u32>(SAVE_REFERENCE_MARKER);
            e.mem.set_u32(out.addr(), marker);
            e.mem.set_u32(out.addr() + 0xc, 9);
            let parent = e.call(PARENT_CELL, &args![this]).u32();
            let form_id = e.call(FORM_ID, &args![parent]).u32();
            e.mem.set_u32(out.addr() + 8, form_id);
            e.mem.set_u32(out.addr() + 4, 0);
            e.mem.set_u32(out.addr() + 0x10, 0);
        }
        _ => {}
    }
}

// Translated from 00539460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether any of the flag bits 1 (heights), 4 (textures) or 2 (colours) of
/// the land's data is set.
pub fn fn_00539460(e: &mut Engine, this: Ptr<TESObjectLAND>) -> bool {
    e.get(this, TESObjectLAND::Data) & (FLAG_HEIGHTS | FLAG_COLORS | FLAG_TEXTURES) != 0
}

// Translated from 005394a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether flag bit 8 of the land's data (the vertex data is loaded) is
/// set.
pub fn fn_005394a0(e: &mut Engine, this: Ptr<TESObjectLAND>) -> bool {
    e.get(this, TESObjectLAND::Data) & FLAG_LOADED != 0
}

// Translated from 005394c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears flag bit 8 of the land's data (the vertex data is
/// loaded).
pub fn fn_005394c0(e: &mut Engine, this: Ptr<TESObjectLAND>, loaded: bool) {
    let data = e.get(this, TESObjectLAND::Data);
    let data = if loaded {
        data | FLAG_LOADED
    } else {
        data & !FLAG_LOADED
    };
    e.set(this, TESObjectLAND::Data, data);
}

// Translated from 00539500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Allocates the loaded data of a land that has none (or has no vertex
/// array): returns false otherwise. Constructs the 0xA4-byte
/// `LoadedLandData`, stores the cell coordinates, allocates the arrays of
/// four pointers for vertices, colours, normals and normal-set flags and the
/// default height (`0053a550`), then for each quadrant allocates the 0x121
/// vertex, colour and normal entries, the normal-set flags, the percent
/// pointer array (0x484 bytes), the layer texture array (0x18 bytes) and sets
/// the default texture. When `source` has no loaded data the quadrant starts
/// as a copy of the shared defaults (vertices, colours, normals, flags), its
/// vertex heights set to the default height unless that is -2048, a zeroed
/// block of 0x121 percent entries of 0x20 bytes and zeroed layer textures.
pub fn fn_00539500(e: &mut Engine, this: Ptr<TESObjectLAND>, source: Ptr<TESObjectLAND>) -> bool {
    let existing = loaded_data(e, this);
    if !existing.is_null() && !e.get(existing, LoadedLandData::ppVertices).is_null() {
        return false;
    }
    let memory = e.call(ALLOCATE, &args![0xa4u32]).u32();
    let data: Ptr<LoadedLandData> = if memory == 0 {
        Ptr::NULL
    } else {
        e.call(LOADED_LAND_DATA_CONSTRUCT, &args![memory]).ptr()
    };
    e.set(this, TESObjectLAND::pLoadedData, data.cast());
    let cell = e.get(this, TESObjectLAND::pParentCell);
    let x = e.call(CELL_GET_DATA_X, &args![cell]).i32();
    e.set(data, LoadedLandData::iCellX, x);
    let y = e.call(CELL_GET_DATA_Y, &args![cell]).i32();
    e.set(data, LoadedLandData::iCellY, y);
    for field in [
        LoadedLandData::ppVertices.off,
        LoadedLandData::ppColorsA.off,
        LoadedLandData::ppNormals.off,
        LoadedLandData::ppNormalsSet.off,
    ] {
        let array = e.call(ALLOCATE, &args![0x10u32]).u32();
        e.mem.set_u32(data.addr() + field, array);
    }
    let default_height = e.call(LAND_DEFAULT_HEIGHT, &args![this]).f32();
    for quadrant in 0..QUADRANTS {
        let slot = 4 * quadrant;
        let vertices =
            allocate_vector(e, 0xd8c, 12, QUADRANT_VERTICES, NI_POINT3_DEFAULT_CONSTRUCT);
        let array = e.get(data, LoadedLandData::ppVertices);
        e.mem.set_u32(array.addr() + slot, vertices);
        let colors = allocate_vector(e, 0x1210, 16, QUADRANT_VERTICES, NI_COLOR_DEFAULT_CONSTRUCT);
        let array = e.get(data, LoadedLandData::ppColorsA);
        e.mem.set_u32(array.addr() + slot, colors);
        let normals = allocate_vector(e, 0xd8c, 12, QUADRANT_VERTICES, NI_POINT3_DEFAULT_CONSTRUCT);
        let array = e.get(data, LoadedLandData::ppNormals);
        e.mem.set_u32(array.addr() + slot, normals);
        let flags = e.call(ALLOCATE, &args![0x121u32]).u32();
        let array = e.get(data, LoadedLandData::ppNormalsSet);
        e.mem.set_u32(array.addr() + slot, flags);
        let percent = e.call(ALLOCATE, &args![0x484u32]).u32();
        e.mem.set_u32(
            data.addr() + LoadedLandData::ppPercentArrays.off + slot,
            percent,
        );
        let default_texture = fn_00535ae0(e).addr();
        e.mem.set_u32(
            data.addr() + LoadedLandData::pDefQuadTexture.off + slot,
            default_texture,
        );
        let layers = e.call(ALLOCATE, &args![0x18u32]).u32();
        e.mem.set_u32(
            data.addr() + LoadedLandData::pQuadTextureArray.off + slot,
            layers,
        );
        if !source.is_null() && !loaded_data(e, source).is_null() {
            continue;
        }
        let blocks = e.global::<u32>(DEFAULT_VERTEX_BLOCKS + slot);
        e.call(MEMORY_COPY, &args![vertices, blocks, 0xd8cu32]);
        let defaults = e.global::<u32>(DEFAULT_COLORS);
        e.call(MEMORY_COPY, &args![colors, defaults, 0x1210u32]);
        let defaults = e.global::<u32>(DEFAULT_NORMALS);
        e.call(MEMORY_COPY, &args![normals, defaults, 0xd8cu32]);
        let defaults = e.global::<u32>(DEFAULT_NORMAL_SET);
        e.call(MEMORY_COPY, &args![flags, defaults, 0x121u32]);
        if default_height != e.global::<f32>(F32_MINUS_2048) {
            for vertex in 0..QUADRANT_VERTICES {
                e.mem.set_f32(vertices + 8 + vertex * 12, default_height);
            }
        }
        let block = e.call(ALLOCATE, &args![0x2420u32]).u32();
        e.call(MEMORY_SET, &args![block, 0u32, 0x2420u32]);
        for entry in 0..QUADRANT_VERTICES {
            e.mem.set_u32(percent + 4 * entry, block + 0x20 * entry);
        }
        e.call(MEMORY_SET, &args![layers, 0u32, 0x18u32]);
    }
    true
}

// Translated from 00539960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds the materials of the quadrant meshes of a land (source line
/// 0xca4 of the allocation tag). Fails (false) without loaded data or
/// without a first mesh. Fills the colours with white (alpha 0) when the
/// land has no vertex colours, sets the "loaded" flag, and for each
/// quadrant attaches the percent extra data to the mesh, creates a
/// `BSShaderPPLightingProperty`, gives it the quadrant's default texture
/// set (or the default land texture's) in slot 0 and the layer texture sets
/// in slots 1 to 6, the flag bytes of those textures, the tangent space of
/// the mesh's first geometry and the geometry data hand-over (slot `0xd4`),
/// attaches it and prepares the mesh. Returns true.
pub fn fn_00539960(e: &mut Engine, this: Ptr<TESObjectLAND>) -> bool {
    scoped(e, 0xca4, |e| {
        let data = loaded_data(e, this);
        if data.is_null() {
            return false;
        }
        let meshes = e.get(data, LoadedLandData::ppMesh);
        if meshes.is_null() || element(e, meshes, 0) == 0 {
            return false;
        }
        let white = ni_color(e, 1.0, 1.0, 1.0, 0.0);
        if e.get(this, TESObjectLAND::Data) & FLAG_COLORS == 0 {
            let colors = e.get(data, LoadedLandData::ppColorsA);
            for quadrant in 0..QUADRANTS {
                for row in 0..0x11 {
                    for column in 0..0x11 {
                        let at = element(e, colors, quadrant) + (row * 0x11 + column) * 0x10;
                        write_words(e, at, &white);
                    }
                }
            }
        }
        fn_005394c0(e, this, true);
        for quadrant in 0..QUADRANTS {
            build_quadrant_material(e, data, meshes, quadrant);
        }
        true
    })
}

/// The per-quadrant part of `fn_00539960`.
fn build_quadrant_material(e: &mut Engine, data: Ptr<LoadedLandData>, meshes: Ptr, quadrant: u32) {
    let mesh = element(e, meshes, quadrant);
    if data
        .addr()
        .wrapping_add(LoadedLandData::ppPercentArrays.off)
        != 0
    {
        let memory = e.call(NI_ALLOCATE_OBJECT, &args![EXTRA_DATA_SIZE]).u32();
        let extra = if memory == 0 {
            0
        } else {
            let percent = e
                .mem
                .u32(data.addr() + LoadedLandData::ppPercentArrays.off + 4 * quadrant);
            let first = e.mem.u32(percent);
            e.call(
                PERCENT_EXTRA_DATA_CONSTRUCT,
                &args![memory, 0x2420u32, first],
            )
            .u32()
        };
        let key = fn_00539f40(e);
        e.call(OBJECT_ADD_EXTRA_DATA, &args![mesh, key, extra]);
    }
    let memory = e
        .call(NI_ALLOCATE_OBJECT, &args![SHADER_PROPERTY_SIZE])
        .u32();
    let property = if memory == 0 {
        0
    } else {
        e.call(SHADER_PROPERTY_CONSTRUCT, &args![memory]).u32()
    };
    e.call(SHADER_PROPERTY_SET_WORD_0X58, &args![property, 1u32]);
    if property == 0 {
        return;
    }
    let default_slot = data.addr() + LoadedLandData::pDefQuadTexture.off + 4 * quadrant;
    let layers = e
        .mem
        .u32(data.addr() + LoadedLandData::pQuadTextureArray.off + 4 * quadrant);
    // Slot 0: the quadrant's default texture's texture set, else the
    // default land texture's.
    let texture = e.mem.u32(default_slot);
    let texture_set = if texture != 0 && e.call(OBJECT_FIELD_0X18, &args![texture]).u32() != 0 {
        let set = e.call(OBJECT_FIELD_0X18, &args![texture]).u32();
        e.call(TEXTURE_SET_AS_SHADER_SET, &args![set]).u32()
    } else {
        let default_texture = fn_00535ae0(e).addr();
        let set = e.call(OBJECT_FIELD_0X18, &args![default_texture]).u32();
        e.call(TEXTURE_SET_AS_SHADER_SET, &args![set]).u32()
    };
    e.call(
        SHADER_PROPERTY_SET_TEXTURE_SET,
        &args![property, 0u32, texture_set],
    );
    for slot in 1..7u32 {
        let texture = e.mem.u32(layers + 4 * slot - 4);
        if texture != 0 && e.call(OBJECT_FIELD_0X18, &args![texture]).u32() != 0 {
            let set = e.call(OBJECT_FIELD_0X18, &args![texture]).u32();
            let shader_set = e.call(TEXTURE_SET_AS_SHADER_SET, &args![set]).u32();
            e.call(
                SHADER_PROPERTY_SET_TEXTURE_SET,
                &args![property, slot, shader_set],
            );
        } else {
            e.call(
                SHADER_PROPERTY_SET_TEXTURE_SET,
                &args![property, slot, 0u32],
            );
        }
    }
    // The flag bytes of the layer textures 5 to 0, then of the default one.
    let sources = [
        e.mem.u32(layers + 0x14),
        e.mem.u32(layers + 0x10),
        e.mem.u32(layers + 0xc),
        e.mem.u32(layers + 8),
        e.mem.u32(layers + 4),
        e.mem.u32(layers),
        e.mem.u32(default_slot),
    ];
    let mut flag_bytes = [0u32; 7];
    for (flag_byte, source) in flag_bytes.iter_mut().zip(sources) {
        *flag_byte = if source == 0 {
            0
        } else {
            e.call(LAND_TEXTURE_FLAG_BYTE, &args![source]).u8() as u32
        };
    }
    let [layer5, layer4, layer3, layer2, layer1, layer0, default_flag] = flag_bytes;
    e.call(
        SHADER_PROPERTY_SET_FLAGS,
        &args![
            property,
            default_flag,
            layer0,
            layer1,
            layer2,
            layer3,
            layer4,
            layer5,
            0u32,
            0u32,
            0u32
        ],
    );
    let geometry = e.call(NODE_FIRST_GEOMETRY, &args![mesh, 0u32]).u32();
    let tangent_space = e.call(CREATE_TANGENT_SPACE_SIMPLE, &args![geometry]).u32();
    fn_00539f20(e, Ptr::new(property), tangent_space);
    let shared = e
        .vcall(property, PROPERTY_SLOT_TANGENT_SPACE, &args![geometry])
        .u32();
    fn_00539ef0(e, Ptr::new(geometry), shared);
    e.call(NODE_ATTACH_PROPERTY, &args![geometry, property]);
    e.call(PREPARE_OBJECT, &args![mesh, 0u32, 0u32]);
}

// Translated from 00539ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Hands `value` to `00a67260` on the object held by the smart pointer at
/// `this + 0xb8`.
pub fn fn_00539ef0(e: &mut Engine, this: Ptr, value: u32) {
    let held = e.call(SMART_POINTER_GET, &args![this.byte_add(0xb8)]).u32();
    e.call(GEOMETRY_DATA_SET_SHARED, &args![held, value]);
}

// Translated from 00539f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assigns `value` to the smart pointer at `this + 0xd0`.
pub fn fn_00539f20(e: &mut Engine, this: Ptr, value: u32) {
    e.call(TEXTURING_PROPERTY_SET, &args![this.byte_add(0xd0), value]);
}

// Translated from 00539f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The global word at `011f94bc` (the key of the percent extra data).
pub fn fn_00539f40(e: &mut Engine) -> u32 {
    e.global::<u32>(EXTRA_DATA_KEY)
}

// ---- Third session: data, callees and layouts ------------------------------

/// The vtable and `NiRTTI` of the 0x14-byte binary extra data (vtable,
/// reference count, name, data pointer at +0xc, size at +0x10), its base
/// constructor and destructor body, and a block free that takes only the
/// pointer.
const BINARY_EXTRA_DATA_VTABLE: u32 = 0x0102_e394;
const BINARY_EXTRA_DATA_RTTI: u32 = 0x011f_4ab8;
const EXTRA_DATA_CONSTRUCT: u32 = 0x00a7_b2e0;
const EXTRA_DATA_DESTRUCT: u32 = 0x00a7_b300;
const FREE_BLOCK: u32 = 0x00aa_10f0;
/// The float the world space gives as the default land height (`009a1260`).
const WORLD_SPACE_DEFAULT_HEIGHT: u32 = 0x009a_1260;
/// The type descriptor of `TESTexture`, the target of the casts of the land
/// texture pointers.
const RTTI_TES_TEXTURE: u32 = 0x0118_3218;
/// `(texture, other)`: 0 when the two textures can be merged into one layer
/// (`0048e4f0`).
const TEXTURES_DIFFER: u32 = 0x0048_e4f0;
/// `max(a, b)` and `min(a, b)` of two floats (`__cdecl`, result in `ST0`),
/// and `fmod(a, b)` of two floats (`__cdecl`).
const FLOAT_MAX: u32 = 0x0040_4010;
const FLOAT_MIN: u32 = 0x0040_ebd0;
const FLOAT_MODULO: u32 = 0x004b_1520;
/// Constants read from the exe's data: the floats 128.0 and 4096.0, and the
/// doubles 4096.0, 289.0, 128.0, 9.0, 0.9, 0.1 (a float widened), 0.001 (a
/// float widened) and 1e-6 (a float widened).
const F32_128: u32 = 0x0101_e704;
const F32_4096: u32 = 0x0101_7a3c;
const F64_4096: u32 = 0x0101_7a10;
const F64_289: u32 = 0x0102_e428;
const F64_128: u32 = 0x0102_e430;
const F64_NINE: u32 = 0x0102_e438;
const F64_POINT_NINE: u32 = 0x0102_e440;
const F64_TENTH: u32 = 0x0101_ffa0;
const F64_THOUSANDTH: u32 = 0x0101_6978;
const F64_MILLIONTH: u32 = 0x0101_7cf8;
/// The default normal of a vertex (three words at `011f426c`).
const DEFAULT_NORMAL_WORDS: u32 = 0x011f_426c;
/// Vector calls (`NiPoint3` unless noted): `this + other`
/// (`__thiscall(this, result, other)`), `this - other` (same form),
/// `this += other` (`__thiscall(this, other)`), `this -= other`,
/// `scalar * vector` (`__cdecl(result, scalar, vector)`), `this * scalar`
/// (`__thiscall(this, result, scalar)`), the cross product (same form as
/// the sum), the length, the equality of x and y (`__thiscall(this,
/// other)`), the function `00525340` the face normals are passed through
/// (`__cdecl(vector)`) and `NiPoint3::UnitizeVectors` (`__cdecl(array,
/// count, stride)`).
const NI_POINT3_ADD: u32 = 0x0043_9e90;
const NI_POINT3_SUBTRACT: u32 = 0x0043_9ef0;
const NI_POINT3_ADD_IN_PLACE: u32 = 0x0063_c8a0;
const NI_POINT3_SUBTRACT_IN_PLACE: u32 = 0x0045_78c0;
const NI_POINT3_SCALE_BY: u32 = 0x004a_3760;
const NI_POINT3_TIMES_SCALAR: u32 = 0x0045_bb20;
const NI_POINT3_CROSS: u32 = 0x004b_3800;
const NI_POINT3_LENGTH: u32 = 0x0045_7990;
const NI_POINT3_SAME_XY: u32 = 0x0043_90c0;
const NI_POINT3_UNIT_VECTOR: u32 = 0x0052_5340;
const NI_POINT3_UNITIZE_VECTORS: u32 = 0x00a7_e960;
/// `NiPoint2` calls: `this - other` (`__thiscall(this, result, other)`) and
/// the length.
const NI_POINT2_SUBTRACT: u32 = 0x004e_8880;
const NI_POINT2_LENGTH: u32 = 0x0058_9850;
/// The data handler's cell lookup `(handler, x, y, world space, 0)`
/// (`00461c20`) and `TESObjectCELL::GetLand`.
const TES_GET_CELL: u32 = 0x0046_1c20;
const CELL_GET_LAND: u32 = 0x0054_6fb0;
/// The address of the value of an integer setting (`0043d4d0`; the float one
/// is [`SETTING_GET_FLOAT_ADDRESS`]).
const SETTING_GET_INT_ADDRESS: u32 = 0x0043_d4d0;
/// The settings the grass builder reads: a float (`011c9f38`), a maximum
/// count (`011c9f74`) and a step (`011c9fb8`, copied to the global
/// `011c9f6c`); and the float setting `0053ca40` returns.
const GRASS_SETTING_THRESHOLD: u32 = 0x011c_9f38;
const GRASS_SETTING_MAXIMUM: u32 = 0x011c_9f74;
const GRASS_SETTING_STEP: u32 = 0x011c_9fb8;
const GRASS_STEP: u32 = 0x011c_9f6c;
const GRASS_DENSITY_SETTING: u32 = 0x011c_8dcc;
/// The list of the grass forms of a land texture: `00891170` gives the
/// first position of the list in a texture (the object at +0x20),
/// `006815c0` returns the position it is given (the element is read
/// there) and `00726070` gives the next position.
const LIST_FIRST_POSITION: u32 = 0x0089_1170;
const LIST_ELEMENT: u32 = 0x0068_15c0;
const LIST_NEXT_POSITION: u32 = 0x0072_6070;
/// A grass parameter record is 0x44 bytes; `00b600e0` constructs it;
/// `SetAt(map, key, record table)` and `GetAt(map, key, &value)` of the
/// quadrant's grass map.
const GRASS_RECORD_SIZE: u32 = 0x44;
const GRASS_RECORD_CONSTRUCT: u32 = 0x00b6_00e0;
const GRASS_MAP_SET_AT: u32 = 0x0084_4700;
const GRASS_MAP_GET_AT: u32 = 0x0085_3130;
/// Calls on a grass form: the name (`0050a550`) and the virtual getters of
/// its parameters (the meaning of the first four is not confirmed; the
/// record keeps them at +8, +0xc, +0x10 and +0x18).
const GRASS_FORM_NAME: u32 = 0x0050_a550;
const GRASS_SLOT_PARAMETER_AT_8: u32 = 0x1b0;
const GRASS_SLOT_PARAMETER_AT_0C: u32 = 0x1b8;
const GRASS_SLOT_PARAMETER_AT_10: u32 = 0x1c0;
const GRASS_SLOT_PARAMETER_AT_18: u32 = 0x1c8;
const GRASS_SLOT_BYTE_AT_1C: u32 = 0x1d0;
const GRASS_SLOT_BYTE_AT_1D: u32 = 0x1d8;
const GRASS_SLOT_BYTE_AT_1E: u32 = 0x1e0;
const GRASS_SLOT_PERCENT: u32 = 0x180;
/// Calls of the mesh update: the vertex data pointer of a geometry data
/// (`0059bb30`, `D3DTexture_LockRect` in the Xbox PDB) and the call that
/// follows the copy (`00a66a60`, `(data, 0)`), `NiAVObject::GetProperty(
/// this, 3)`, the property's type (`00441110`), `BSShaderProperty::
/// FreeRenderPasses` (Xbox PDB), the property's update slot (0x9c), the
/// node's slot 0x18 that gives its geometry, the exterior loader global
/// (`011dea10`) with its flag getter (`00528790`) and the follow-up
/// `0053fa40(land, flag)`.
const GEOMETRY_DATA_LOCKED_POINTER: u32 = 0x0059_bb30;
const GEOMETRY_DATA_UNLOCK: u32 = 0x00a6_6a60;
const OBJECT_GET_PROPERTY: u32 = 0x00a5_9d30;
const PROPERTY_GET_TYPE: u32 = 0x0044_1110;
const SHADER_PROPERTY_FREE_RENDER_PASSES: u32 = 0x00ba_a000;
const SHADER_PROPERTY_SLOT_UPDATE: u32 = 0x9c;
const NODE_SLOT_AS_GEOMETRY: u32 = 0x18;
const EXTERIOR_LOADER: u32 = 0x011d_ea10;
const EXTERIOR_LOADER_FLAG: u32 = 0x0052_8790;
const LAND_FOLLOW_UP: u32 = 0x0053_fa40;

layout! {
    /// What `GetCoordData` (`0053b550`) works out for a world position
    /// (0x4E bytes on the PC; the Xbox PDB has no name for it): the
    /// position relative to the land's cell corner, its quadrant (2048
    /// units), its 128-unit block, the triangle of the block that holds it
    /// and the vertex nearest to it. `QuadrantOffset*` and `BlockOffset*`
    /// are relative to the quadrant and block corners. The vector at +0x30
    /// is the position of the nearest vertex plus the cell origin.
    pub struct CoordData: 0x50 {
        /// The position minus the cell corner (x).
        0x00 CellOffsetX: f32,
        /// The position minus the cell corner (y).
        0x04 CellOffsetY: f32,
        /// The position minus the quadrant corner (x).
        0x08 QuadrantOffsetX: f32,
        /// The position minus the quadrant corner (y).
        0x0C QuadrantOffsetY: f32,
        /// `trunc(CellOffsetX / 2048)`, less one on an exact boundary.
        0x10 QuadrantColumn: i32,
        /// `trunc(CellOffsetY / 2048)`, less one on an exact boundary.
        0x14 QuadrantRow: i32,
        /// `QuadrantColumn + 2 * QuadrantRow`: the quadrant index.
        0x18 Quadrant: i32,
        /// The position minus the 128-unit block corner (x).
        0x1C BlockOffsetX: f32,
        /// The position minus the 128-unit block corner (y).
        0x20 BlockOffsetY: f32,
        /// The block column inside the quadrant.
        0x24 BlockColumn: i32,
        /// The block row inside the quadrant.
        0x28 BlockRow: i32,
        /// A copy of the quadrant index.
        0x2C QuadrantCopy: i32,
        /// The vector the nearest vertex gives (see the struct doc).
        0x30 VertexPositionX: f32,
        0x34 VertexPositionY: f32,
        0x38 VertexPositionZ: f32,
        /// The index of the nearest vertex of the quadrant.
        0x3C NearestVertex: i32,
        /// The three vertices of the triangle that holds the position.
        0x40 TriangleVertex2: i32,
        0x44 TriangleVertex0: i32,
        0x48 TriangleVertex1: i32,
        /// Set when the block's diagonal runs the other way (odd block).
        0x4C OddBlock: u8,
        /// Set for the second triangle of the block.
        0x4D SecondTriangle: u8,
    }
}

/// The size of the `CoordData` the callers keep on their stack, rounded up.
const COORD_DATA_SIZE: u32 = 0x50;

/// The order the eight neighbours of a land are numbered in by
/// `fn_0053d330`: `(dx, dy)` of the cell relative to the land's cell.
const NEIGHBOUR_OFFSETS: [(i32, i32); 8] = [
    (-1, 1),
    (0, 1),
    (1, 1),
    (-1, 0),
    (1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

/// `__RTDynamicCast` of a land texture pointer to `TESTexture`.
fn texture_cast(e: &mut Engine, texture: u32) -> u32 {
    e.call(
        RT_DYNAMIC_CAST,
        &args![texture, 0u32, RTTI_TES_LAND_TEXTURE, RTTI_TES_TEXTURE, 0u32],
    )
    .u32()
}

/// The array of layer textures (`pQuadTextureArray`) of quadrant `quadrant`
/// of the loaded data.
fn layer_array(e: &Engine, data: Ptr<LoadedLandData>, quadrant: u32) -> u32 {
    e.mem.u32(
        data.addr()
            .wrapping_add(LoadedLandData::pQuadTextureArray.off)
            .wrapping_add(4u32.wrapping_mul(quadrant)),
    )
}

/// The array of the 0x121 percent rows (`ppPercentArrays`) of a quadrant.
fn percent_array(e: &Engine, data: Ptr<LoadedLandData>, quadrant: u32) -> u32 {
    e.mem.u32(
        data.addr()
            .wrapping_add(LoadedLandData::ppPercentArrays.off)
            .wrapping_add(4u32.wrapping_mul(quadrant)),
    )
}

/// The row of layer opacities (eight floats) of vertex `vertex` of a
/// quadrant's percent array.
fn percent_row(e: &Engine, percent: u32, vertex: u32) -> u32 {
    e.mem.u32(percent.wrapping_add(4u32.wrapping_mul(vertex)))
}

/// A float stored the way an `FSTP float` does after a sum computed in
/// extended precision.
fn sum_f32(a: f32, b: f32) -> f32 {
    (a as f64 + b as f64) as f32
}

/// The vector `(corner x + 2048, corner y + 2048, 0)` the coordinate code
/// adds to a vertex: `fn_00534080` (y) first, then `fn_00534050` (x), then
/// the `NiPoint3` constructor.
fn cell_centre_vector(e: &mut Engine, this: Ptr<TESObjectLAND>) -> Vec<u32> {
    let two_thousand = e.global::<f64>(F64_2048);
    let y = (fn_00534080(e, this) + two_thousand) as f32;
    let x = (fn_00534050(e, this) + two_thousand) as f32;
    ni_point3(e, x, y, 0.0)
}

/// Runs `body` with a 12-byte `NiPoint3` local holding
/// [`cell_centre_vector`], after its default constructor ran, as the
/// coordinate functions keep it.
fn with_cell_centre<R>(
    e: &mut Engine,
    this: Ptr<TESObjectLAND>,
    body: impl FnOnce(&mut Engine, Ptr) -> R,
) -> R {
    e.with_stack(12, |e, vector| {
        e.call(NI_POINT3_DEFAULT_CONSTRUCT, &args![vector]);
        let words = cell_centre_vector(e, this);
        write_words(e, vector.addr(), &words);
        body(e, vector)
    })
}

// ---- Fourth session: data and callees ---------------------------------------

/// Vtables the last functions of the unit store, named by the RTTI of each
/// (the word before the vtable): the grass map (`NiTPointerMap<unsigned
/// int, TESGrassAreaParam*>`) and its base (`NiTMapBase<NiTPointerAllocator
/// <unsigned int>, ...>`); the block array of `NiAdditionalGeometryData`
/// (`NiTArray<NiAGDDataBlock*, ...>`) and its derived
/// `NiTPrimitiveArray<NiAGDDataBlock*, ...>`.
const GRASS_MAP_VTABLE: u32 = 0x0102_e44c;
const MAP_BASE_VTABLE: u32 = 0x0102_e47c;
const BLOCK_ARRAY_VTABLE: u32 = 0x0102_e46c;
const BLOCK_PRIMITIVE_ARRAY_VTABLE: u32 = 0x0102_e474;
/// Number of hash buckets of a grass map (`0x25`), the default constructor
/// of a grass map (the vector constructor iterator's callback) and the
/// vector constructor iterator that takes a destructor as well
/// (`(array, element size, count, constructor, destructor)`).
const GRASS_MAP_HASH_SIZE: u32 = 0x25;
const GRASS_MAP_DEFAULT_CONSTRUCT: u32 = 0x0054_0700;
const EH_VECTOR_CONSTRUCT: u32 = 0x00ec_782f;
/// `NiFree`-style release of the bucket table of a map (`00aa10f0`) and of
/// the table of a block array (`004ede70`, which calls it).
const RELEASE_BLOCK: u32 = 0x00aa_10f0;
const RELEASE_TABLE: u32 = 0x004e_de70;

/// The Havok array of bytes at +0x20 of the MOPP code object: the object
/// that gives its allocator (`004a4840`, `(local byte, array)`, which turns
/// the router into the allocator), the capacity (`0062a100`: the word at
/// +8 without its two top bits), `hkArrayUtil::_reserve` (Xbox PDB,
/// `(allocator, array, capacity, element size)`, cdecl), the empty range
/// function (`0040fbe0`: `(pointer, count, byte)`, cdecl, a function whose
/// body does nothing for bytes) and the release of the array (`00540b70`,
/// `(array, allocator)`).
const HK_ARRAY_ALLOCATOR: u32 = 0x004a_4840;
const HK_ARRAY_CAPACITY: u32 = 0x0062_a100;
const HK_ARRAY_RESERVE: u32 = 0x00c9_08c0;
const BYTE_RANGE_NOTHING: u32 = 0x0040_fbe0;
const HK_ARRAY_CLEAR: u32 = 0x0054_0b70;
/// The block allocator of `NiTArray` (`0096afc0`, `(count)`).
const BLOCK_ARRAY_ALLOCATE: u32 = 0x0096_afc0;

/// `NiLines::NiLines(this, 0x80, positions, colors, 0, 0, 0, flags)`
/// (Xbox PDB), `BSShaderNoLightingProperty::BSShaderNoLightingProperty`
/// (Xbox PDB), the call on the new texturing property that follows its
/// first map (`00a6a150`, `(this, 0, 0)`), and the setter of the word at
/// +0x58 of a shader property (`005a8060`, here 0x21).
const NI_LINES_CONSTRUCT: u32 = 0x00a7_46e0;
const NO_LIGHTING_PROPERTY_CONSTRUCT: u32 = 0x00b6_fc90;
const TEXTURING_PROPERTY_AFTER_MAP: u32 = 0x00a6_a150;
const BORDER_SHADER_WORD: u32 = 0x21;
/// Slot `0xdc` of the cell's node: it takes the border lines and 1.
const NODE_ATTACH_BORDER_SLOT: u32 = 0xdc;
/// Count of vertices of the border lines (4 x 32), their indices around the
/// quadrant grid, and the height added to a border vertex (`01020998`, the
/// double 5.0).
const BORDER_VERTICES: u32 = 0x80;
const F64_FIVE: u32 = 0x0102_0998;
/// The six INI settings (integers, 0 to 255) the border colours are made of,
/// by the parity of the vertex's index: the settings' names are not
/// confirmed. The vertices with an odd index take red, green and blue from
/// the first three, the others from the last three.
const BORDER_ODD_RED: u32 = 0x011c_9f20;
const BORDER_ODD_GREEN: u32 = 0x011c_9f9c;
const BORDER_ODD_BLUE: u32 = 0x011c_9fc8;
const BORDER_EVEN_RED: u32 = 0x011c_9ff4;
const BORDER_EVEN_GREEN: u32 = 0x011c_a024;
const BORDER_EVEN_BLUE: u32 = 0x011c_9fac;

/// `NiColorA` arithmetic: `scalar * color` (`__cdecl(result, scalar,
/// color)`), `this + other` (`__thiscall(this, result, other)`) and
/// `this * scalar` (`__thiscall(this, result, scalar)`), each returning its
/// result.
const COLOR_SCALED: u32 = 0x0053_2f40;
const COLOR_ADD: u32 = 0x0053_2e40;
const COLOR_TIMES_SCALAR: u32 = 0x0053_2f60;
/// The float `0.5` (`01016248`) and the doubles the extents code compares
/// against: `FLT_MAX` and `-FLT_MAX` as doubles.
const F32_HALF: u32 = 0x0101_6248;
const F64_LARGEST: u32 = 0x0102_31b0;
const F64_LOWEST: u32 = 0x0102_41b0;

/// The distant texture blending: the exterior's world space
/// (`TES::GetWorldSpace`, `(tes)`), its terrain manager
/// (`TESWorldSpace::GetTerrainManager`), the three terrain manager calls
/// that take the address of the position (`006fce30` and `006fce70` give a
/// word each, `006fceb0` gives the object whose smart pointer is read), the
/// two 16-bit getters it calls on that object (the word getter is
/// [`DATA_HANDLER_WORD_AT_4`], the word at +4), the property getter of
/// `00441110`, and the slots `0xfc` and `0x100` of the shader property, which take
/// `(9, value)`.
const TES_GET_WORLD_SPACE: u32 = 0x004f_d3e0;
const WORLD_SPACE_GET_TERRAIN_MANAGER: u32 = 0x0058_6170;
const TERRAIN_BLEND_FIRST: u32 = 0x006f_ce30;
const TERRAIN_BLEND_SECOND: u32 = 0x006f_ce70;
const TERRAIN_BLEND_OBJECT: u32 = 0x006f_ceb0;
const TERRAIN_OBJECT_SHORT_FIRST: u32 = 0x004a_8ae0;
const TERRAIN_OBJECT_SHORT_SECOND: u32 = 0x0047_d3f0;
const PROPERTY_SLOT_BLEND_FIRST: u32 = 0xfc;
const PROPERTY_SLOT_BLEND_SECOND: u32 = 0x100;
/// The texture index the blend textures are set for, and the range (8 to
/// 12) of the value of `00441110` (a getter of an integer field of the
/// property; the map names it `PathingLocation::GetWorldspace`, a folded
/// body) for the properties that take blending.
const BLEND_TEXTURE_INDEX: i32 = 9;
const BLEND_TYPE_FIRST: i32 = 8;
const BLEND_TYPE_LAST: i32 = 12;
/// The four words copied into the property's block at +0x94 (`011a9bd0`),
/// the first two of which the code then replaces.
const BLEND_BLOCK_WORDS: u32 = 0x011a_9bd0;
/// Where the blending block is stored in the property.
const PROPERTY_BLEND_BLOCK_OFFSET: u32 = 0x94;

// ---- Binary extra data (the percent extra data of a quadrant mesh) ---------

// Translated from 00539f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the binary extra data (`NiBinaryExtraData`, by the Xbox
/// PDB name of its destructor; the decompiler names it after a library
/// constructor): runs the base constructor `00a7b2e0`, stores the vtable
/// and sets the size and the data pointer through [`fn_0053a070`]. Returns
/// `this`. (The unwinding frame is not translated.)
pub fn fn_00539f50(e: &mut Engine, this: Ptr, size: u32, data: u32) -> Ptr {
    e.call(EXTRA_DATA_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), BINARY_EXTRA_DATA_VTABLE);
    fn_0053a070(e, this, size, data);
    this
}

// Translated from 00539fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBinaryExtraData::GetRTTI` (Xbox PDB): the class's `NiRTTI`.
pub fn ni_binary_extra_data_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(BINARY_EXTRA_DATA_RTTI)
}

// Translated from 00539fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBinaryExtraData::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor body [`fn_0053a000`] and frees the 0x14 bytes when bit 0 of
/// `flags` is set. Returns `this`.
pub fn ni_binary_extra_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_0053a000(e, this);
    if flags & 1 != 0 {
        e.call(NI_FREE, &args![this, 0x14u32]);
    }
    this
}

// Translated from 0053a000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of the binary extra data: stores its vtable, frees the
/// data block at +0xc and clears the pointer, then runs the base
/// destructor body `00a7b300`. (The unwinding frame is not translated.)
pub fn fn_0053a000(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), BINARY_EXTRA_DATA_VTABLE);
    let data = e.mem.u32(this.addr() + 0xc);
    e.call(FREE_BLOCK, &args![data]);
    e.mem.set_u32(this.addr() + 0xc, 0);
    e.call(EXTRA_DATA_DESTRUCT, &args![this]);
}

// Translated from 0053a070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the size (+0x10) and the data pointer (+0xc) of the binary extra
/// data.
pub fn fn_0053a070(e: &mut Engine, this: Ptr, size: u32, data: u32) {
    e.mem.set_u32(this.addr() + 0x10, size);
    e.mem.set_u32(this.addr() + 0xc, data);
}

// ---- UpdateMesh ----------------------------------------------------------------

// Translated from 0053a090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectLAND::UpdateMesh` (Xbox PDB): a land whose loaded data has no
/// mesh array allocates its data (`fn_00539500` without a source land) and
/// builds the quadrant meshes (`fn_005374f0`). Otherwise, for each quadrant
/// with a scene node: when `update_materials` is set and the node's first
/// geometry exists, recomputes its bound, marks its data changed (0xf),
/// takes the shader property (property 3, when its type is 8 to 12), copies
/// the land's normals into the locked vertex data, hands the geometry to
/// the property's update slot (0x9c), gives it the texture sets (slot 0:
/// the quadrant's default texture's, else the default land texture's;
/// slots 1 to 6: the layer textures') and the texture flag bytes, and
/// frees its render passes. Every node then gets an update (with a zero
/// time) and, when `update_materials` is set, `UpdateProperties`. At the
/// end `update_materials` also runs `0053fa40` with the exterior loader's
/// flag. The second word of the stack arguments is not read.
pub fn tes_object_land_update_mesh(
    e: &mut Engine,
    this: Ptr<TESObjectLAND>,
    update_materials: u8,
    _unused_1: u32,
) {
    let data = loaded_data(e, this);
    if e.get(data, LoadedLandData::ppMesh).is_null() {
        fn_00539500(e, this, Ptr::NULL);
        fn_005374f0(e, this);
        return;
    }
    for quadrant in 0..4i32 {
        let mesh = fn_00535af0(e, this, quadrant).addr();
        if mesh == 0 {
            continue;
        }
        if update_materials != 0 {
            update_mesh_materials(e, data, mesh, quadrant as u32);
        }
        e.with_stack(12, |e, update_data| {
            e.call(
                UPDATE_DATA_CONSTRUCT,
                &args![update_data, 0.0f32, 0u32, 0u32],
            );
            e.call(NODE_UPDATE, &args![mesh, update_data]);
        });
        if update_materials != 0 {
            e.call(NODE_UPDATE_PROPERTIES, &args![mesh]);
        }
    }
    if update_materials != 0 {
        let loader = e.global::<u32>(EXTERIOR_LOADER);
        let flag = e.call(EXTERIOR_LOADER_FLAG, &args![loader]).u8();
        e.call(LAND_FOLLOW_UP, &args![this, flag as u32]);
    }
}

/// The part of `tes_object_land_update_mesh` that runs on a node's first
/// geometry (see there).
fn update_mesh_materials(e: &mut Engine, data: Ptr<LoadedLandData>, mesh: u32, quadrant: u32) {
    let node = e.call(NODE_FIRST_GEOMETRY, &args![mesh, 0u32]).u32();
    let geometry = if node == 0 {
        0
    } else {
        e.vcall(node, NODE_SLOT_AS_GEOMETRY, &args![]).u32()
    };
    if geometry == 0 {
        return;
    }
    let vertex_count = e.call(TRI_STRIPS_VERTEX_COUNT, &args![geometry]).u16();
    let positions = e.call(GEOMETRY_GET_POSITIONS, &args![geometry]).u32();
    let bound = fn_00537b10(e, Ptr::new(geometry)).addr();
    e.call(
        BOUND_COMPUTE_FROM_DATA,
        &args![bound, vertex_count as u32, positions],
    );
    let geometry_data = e.call(GEOMETRY_GET_DATA, &args![geometry]).u32();
    e.call(GEOMETRY_DATA_MARK_CHANGED, &args![geometry_data, 0xfu32]);
    let property = e.call(OBJECT_GET_PROPERTY, &args![geometry, 3u32]).u32();
    let is_shader = if property == 0 {
        false
    } else {
        let kind = e.call(PROPERTY_GET_TYPE, &args![property]).i32();
        kind >= 8 && e.call(PROPERTY_GET_TYPE, &args![property]).i32() <= 0xc
    };
    let shader = if is_shader { property } else { 0 };
    if shader == 0 {
        return;
    }
    let geometry_data = e.call(GEOMETRY_GET_DATA, &args![geometry]).u32();
    let locked = e
        .call(GEOMETRY_DATA_LOCKED_POINTER, &args![geometry_data])
        .u32();
    let normals = e.get(data, LoadedLandData::ppNormals);
    let source = element(e, normals, quadrant);
    e.call(MEMORY_COPY, &args![locked, source, 0xd8cu32]);
    let geometry_data = e.call(GEOMETRY_GET_DATA, &args![geometry]).u32();
    e.call(GEOMETRY_DATA_UNLOCK, &args![geometry_data, 0u32]);
    e.vcall(shader, SHADER_PROPERTY_SLOT_UPDATE, &args![geometry]);
    // Slot 0: the quadrant's default texture set, else the default land
    // texture's.
    let default_slot = data.addr() + LoadedLandData::pDefQuadTexture.off + 4 * quadrant;
    let texture = e.mem.u32(default_slot);
    let texture_set = if texture != 0 && e.call(OBJECT_FIELD_0X18, &args![texture]).u32() != 0 {
        let set = e.call(OBJECT_FIELD_0X18, &args![texture]).u32();
        e.call(TEXTURE_SET_AS_SHADER_SET, &args![set]).u32()
    } else {
        let default_texture = fn_00535ae0(e).addr();
        let set = e.call(OBJECT_FIELD_0X18, &args![default_texture]).u32();
        e.call(TEXTURE_SET_AS_SHADER_SET, &args![set]).u32()
    };
    e.call(
        SHADER_PROPERTY_SET_TEXTURE_SET,
        &args![shader, 0u32, texture_set],
    );
    let layers = layer_array(e, data, quadrant);
    for slot in 1..7u32 {
        let texture = e.mem.u32(layers + 4 * slot - 4);
        if texture != 0 && e.call(OBJECT_FIELD_0X18, &args![texture]).u32() != 0 {
            let set = e.call(OBJECT_FIELD_0X18, &args![texture]).u32();
            let shader_set = e.call(TEXTURE_SET_AS_SHADER_SET, &args![set]).u32();
            e.call(
                SHADER_PROPERTY_SET_TEXTURE_SET,
                &args![shader, slot, shader_set],
            );
        } else {
            e.call(SHADER_PROPERTY_SET_TEXTURE_SET, &args![shader, slot, 0u32]);
        }
    }
    // The flag bytes of the layer textures 5 to 0, then of the default one.
    let sources = [
        e.mem.u32(layers + 0x14),
        e.mem.u32(layers + 0x10),
        e.mem.u32(layers + 0xc),
        e.mem.u32(layers + 8),
        e.mem.u32(layers + 4),
        e.mem.u32(layers),
        e.mem.u32(default_slot),
    ];
    let mut flag_bytes = [0u32; 7];
    for (flag_byte, source) in flag_bytes.iter_mut().zip(sources) {
        *flag_byte = if source == 0 {
            0
        } else {
            e.call(LAND_TEXTURE_FLAG_BYTE, &args![source]).u8() as u32
        };
    }
    let [layer5, layer4, layer3, layer2, layer1, layer0, default_flag] = flag_bytes;
    e.call(
        SHADER_PROPERTY_SET_FLAGS,
        &args![
            shader,
            default_flag,
            layer0,
            layer1,
            layer2,
            layer3,
            layer4,
            layer5,
            0u32,
            0u32,
            0u32
        ],
    );
    e.call(SHADER_PROPERTY_FREE_RENDER_PASSES, &args![shader]);
}

// ---- Coordinate data records -----------------------------------------------

// Translated from 0053a510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the `CoordData` record the coordinate functions keep on
/// their stack (the decompiler names it after a library constructor): runs
/// the `NiPoint3` default constructor on the members at +0, +8, +0x1c and
/// +0x30. Returns `this`.
pub fn fn_0053a510(e: &mut Engine, this: Ptr) -> Ptr {
    for offset in [0u32, 0x8, 0x1c, 0x30] {
        e.call(NI_POINT3_DEFAULT_CONSTRUCT, &args![this.byte_add(offset)]);
    }
    this
}

// Translated from 0053a550 (decompiled, FalloutNV.exe 1.4.0.525)
/// The default height of the land: -2048 without a parent cell or a world
/// space, else the world space's value (`009a1260`).
pub fn fn_0053a550(e: &mut Engine, this: Ptr<TESObjectLAND>) -> f32 {
    let mut height = e.global::<f32>(F32_MINUS_2048);
    let cell = e.call(PARENT_CELL, &args![this]).u32();
    if cell != 0 {
        let world_space = e.call(CELL_GET_WORLD_SPACE, &args![cell]).u32();
        if world_space != 0 {
            height = e
                .call(WORLD_SPACE_DEFAULT_HEIGHT, &args![world_space])
                .f32();
        }
    }
    height
}

// ---- Layer textures and opacities --------------------------------------------

// Translated from 0053a5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The default texture of quadrant `quadrant` (`pDefQuadTexture`), or 0 for
/// a quadrant of 4 or more or a land without loaded data.
pub fn fn_0053a5a0(e: &mut Engine, this: Ptr<TESObjectLAND>, quadrant: u8) -> u32 {
    let data = loaded_data(e, this);
    if quadrant < 4 && !data.is_null() {
        return e
            .mem
            .u32(data.addr() + LoadedLandData::pDefQuadTexture.off + 4 * quadrant as u32);
    }
    0
}

// Translated from 0053a5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The texture that dominates the position `position` (two floats): works
/// out its `CoordData` with [`tes_object_land_get_coord_data`] (recursing
/// once) and returns [`fn_0053a630`] for its quadrant and nearest vertex,
/// or 0 when the position is outside the land.
pub fn fn_0053a5e0(e: &mut Engine, this: Ptr<TESObjectLAND>, position: Ptr) -> u32 {
    e.with_stack(COORD_DATA_SIZE, |e, info| {
        fn_0053a510(e, info);
        if tes_object_land_get_coord_data(e, this, info.cast(), position, true) {
            let quadrant = e.mem.u8(info.addr() + CoordData::Quadrant.off);
            let vertex = e.mem.u16(info.addr() + CoordData::NearestVertex.off);
            fn_0053a630(e, this, quadrant, vertex)
        } else {
            0
        }
    })
}

// Translated from 0053a630 (decompiled, FalloutNV.exe 1.4.0.525)
/// The texture that dominates vertex `vertex` of quadrant `quadrant`: the
/// layer (0 to 5) with the highest opacity there, ties going to the
/// lowest, as the layer texture ([`fn_0053a700`] of that layer minus one),
/// or the default texture ([`fn_0053a5a0`]) when the highest is layer 0.
/// 0 for a quadrant of 4 or more or a vertex of 0x121 or more.
pub fn fn_0053a630(e: &mut Engine, this: Ptr<TESObjectLAND>, quadrant: u8, vertex: u16) -> u32 {
    if quadrant >= 4 || vertex >= 0x121 {
        return 0;
    }
    let mut highest = e.global::<f32>(LOWEST_F32);
    let mut best = 0u16;
    for layer in 0..6u16 {
        let opacity = fn_0053a830(e, this, quadrant, vertex, layer);
        if highest < opacity {
            best = layer;
            highest = opacity;
        }
    }
    if best == 0 {
        fn_0053a5a0(e, this, quadrant)
    } else {
        fn_0053a700(e, this, quadrant, best - 1)
    }
}

// Translated from 0053a700 (decompiled, FalloutNV.exe 1.4.0.525)
/// The texture of layer `layer` of quadrant `quadrant`, or 0 for a quadrant
/// of 4 or more, a layer of 6 or more, no loaded data or no layer array.
pub fn fn_0053a700(e: &mut Engine, this: Ptr<TESObjectLAND>, quadrant: u8, layer: u16) -> u32 {
    let data = loaded_data(e, this);
    if quadrant < 4 && layer < 6 && !data.is_null() {
        let layers = layer_array(e, data, quadrant as u32);
        if layers != 0 {
            return e.mem.u32(layers + 4 * layer as u32);
        }
    }
    0
}

// Translated from 0053a760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the texture of layer `layer` of quadrant `quadrant` (nothing for a
/// quadrant of 4 or more, a layer of 6 or more or no loaded data).
pub fn fn_0053a760(
    e: &mut Engine,
    this: Ptr<TESObjectLAND>,
    quadrant: u8,
    layer: u16,
    texture: u32,
) {
    let data = loaded_data(e, this);
    if quadrant < 4 && layer < 6 && !data.is_null() {
        let layers = layer_array(e, data, quadrant as u32);
        e.mem.set_u32(layers + 4 * layer as u32, texture);
    }
}

// Translated from 0053a7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The opacity left to the default texture at vertex `vertex` of quadrant
/// `quadrant`: 1 minus the sum of the six layer opacities (just 1 when the
/// indices are out of range or the quadrant has no percent array).
pub fn fn_0053a7a0(e: &mut Engine, this: Ptr<TESObjectLAND>, quadrant: u8, vertex: u16) -> f32 {
    let mut sum = 0.0f32;
    let data = loaded_data(e, this);
    if quadrant < 4 && vertex < 0x121 && !data.is_null() {
        let percent = percent_array(e, data, quadrant as u32);
        if percent != 0 {
            let row = percent_row(e, percent, vertex as u32);
            for layer in 0..6u32 {
                sum = sum_f32(sum, e.mem.f32(row + 4 * layer));
            }
        }
    }
    (1.0 - sum as f64) as f32
}

// Translated from 0053a830 (decompiled, FalloutNV.exe 1.4.0.525)
/// The opacity of layer `layer` at vertex `vertex` of quadrant `quadrant`
/// (0 when the indices are out of range or the quadrant has no percent
/// array).
pub fn fn_0053a830(
    e: &mut Engine,
    this: Ptr<TESObjectLAND>,
    quadrant: u8,
    vertex: u16,
    layer: u16,
) -> f32 {
    let data = loaded_data(e, this);
    if quadrant < 4 && vertex < 0x121 && layer < 6 && !data.is_null() {
        let percent = percent_array(e, data, quadrant as u32);
        if percent != 0 {
            let row = percent_row(e, percent, vertex as u32);
            return e.mem.f32(row + 4 * layer as u32);
        }
    }
    0.0
}

// Translated from 0053a8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the opacity of layer `layer` at vertex `vertex` of quadrant
/// `quadrant`: a positive `opacity` is stored, anything else stores 0 (and
/// only when the quadrant has a percent array). Nothing for indices out of
/// range or a land without loaded data.
pub fn fn_0053a8a0(
    e: &mut Engine,
    this: Ptr<TESObjectLAND>,
    quadrant: u8,
    vertex: u16,
    layer: u16,
    opacity: f32,
) {
    let data = loaded_data(e, this);
    if quadrant >= 4 || vertex >= 0x121 || layer >= 6 || data.is_null() {
        return;
    }
    let percent = percent_array(e, data, quadrant as u32);
    if opacity as f64 > e.global::<f64>(F64_ZERO) {
        let row = percent_row(e, percent, vertex as u32);
        e.mem.set_f32(row + 4 * layer as u32, opacity);
    } else if percent != 0 {
        let row = percent_row(e, percent, vertex as u32);
        e.mem.set_f32(row + 4 * layer as u32, 0.0);
    }
}

// Translated from 0053a940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cleans the layers of every quadrant: a layer whose total opacity over the
/// 289 vertices, divided by 289, is below 0.001 loses its texture; the empty
/// layers are squeezed out ([`fn_0053abb0`]); the opacities of the
/// remaining layers are summed per vertex into a scratch array (with the
/// share the default texture would have, when the layers sum to less than
/// 1, added to the entry after the last layer); and the layer with the
/// highest total is swapped with the default texture ([`fn_0053ace0`]).
pub fn fn_0053a940(e: &mut Engine, this: Ptr<TESObjectLAND>) {
    let data = loaded_data(e, this);
    if data.is_null() {
        return;
    }
    for quadrant in 0..4u32 {
        let layers = layer_array(e, data, quadrant);
        for layer in 0..6u32 {
            if e.mem.u32(layers + 4 * layer) == 0 {
                continue;
            }
            let total = fn_0053ae30(e, this, quadrant as u8, layer as u16);
            let share = (total as f64 / e.global::<f64>(F64_289)) as f32;
            if (share as f64) < e.global::<f64>(F64_THOUSANDTH) {
                e.mem.set_u32(layers + 4 * layer, 0);
            }
        }
        let mut count = 6u32;
        let mut index = 0u32;
        while index < count {
            while e.mem.u32(layers + 4 * index) == 0 && index < count {
                fn_0053abb0(e, this, quadrant as u8, index as u16);
                count -= 1;
            }
            index += 1;
        }
        let size = count * 4 + 4;
        let sums = e.call(ALLOCATE, &args![size]).u32();
        e.call(MEMORY_SET, &args![sums, 0u32, size]);
        for vertex in 0..QUADRANT_VERTICES {
            let mut row_sum = 0.0f32;
            for layer in 0..count {
                let opacity = fn_0053a830(e, this, quadrant as u8, vertex as u16, layer as u16);
                let total = sum_f32(e.mem.f32(sums + 4 * layer), opacity);
                e.mem.set_f32(sums + 4 * layer, total);
                row_sum = sum_f32(row_sum, opacity);
            }
            if (row_sum as f64) < e.global::<f64>(F64_ONE) {
                // 0053ab06: `FLD1; FSUBRP; FADD dword [sums + 4 * count]` on
                // the x87 stack, each step rounded to the control word's
                // precision, one `FSTP dword` at the end.
                let share = x87_sub(1.0, f64::from(row_sum));
                let rest = x87_add(share, x87_m32(e.mem.u32(sums + 4 * count)));
                e.mem.set_u32(sums + 4 * count, x87_store32(rest));
            }
        }
        let mut best = None;
        let mut highest = e.mem.f32(sums + 4 * count);
        for layer in 0..count {
            let total = e.mem.f32(sums + 4 * layer);
            if highest < total {
                highest = total;
                best = Some(layer);
            }
        }
        if let Some(layer) = best {
            fn_0053ace0(e, this, quadrant as u8, layer as u16);
        }
        e.call(DEALLOCATE, &args![sums]);
    }
}

// Translated from 0053abb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes layer `layer` of quadrant `quadrant`: the following layer
/// textures move down one place, the last entry becomes 0 and the same is
/// done to the opacity rows of the 289 vertices (the last opacity becomes
/// 0). Nothing for a quadrant of 4 or more, a layer of 6 or more or a land
/// without loaded data.
pub fn fn_0053abb0(e: &mut Engine, this: Ptr<TESObjectLAND>, quadrant: u8, layer: u16) {
    let data = loaded_data(e, this);
    if quadrant >= 4 || layer >= 6 || data.is_null() {
        return;
    }
    let layers = layer_array(e, data, quadrant as u32);
    for index in layer as u32..5 {
        let next = e.mem.u32(layers + 4 * index + 4);
        e.mem.set_u32(layers + 4 * index, next);
    }
    e.mem.set_u32(layers + 0x14, 0);
    let percent = percent_array(e, data, quadrant as u32);
    if percent == 0 {
        return;
    }
    for vertex in 0..QUADRANT_VERTICES {
        let row = percent_row(e, percent, vertex);
        for index in layer as u32..5 {
            let next = e.mem.f32(row + 4 * index + 4);
            e.mem.set_f32(row + 4 * index, next);
        }
        e.mem.set_f32(row + 0x14, 0.0);
    }
}

// Translated from 0053ace0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Swaps the default texture of quadrant `quadrant` with the texture of
/// layer `layer`; then the opacity of that layer at each vertex becomes 1
/// minus the sum of the six opacities (with the old value of the layer
/// still in the sum), at least 0. Nothing for a quadrant of 4 or more, a
/// layer of 6 or more or a land without loaded data.
pub fn fn_0053ace0(e: &mut Engine, this: Ptr<TESObjectLAND>, quadrant: u8, layer: u16) {
    let data = loaded_data(e, this);
    if quadrant >= 4 || layer >= 6 || data.is_null() {
        return;
    }
    let layers = layer_array(e, data, quadrant as u32);
    let layer_texture = e.mem.u32(layers + 4 * layer as u32);
    let default_slot = data.addr() + LoadedLandData::pDefQuadTexture.off + 4 * quadrant as u32;
    let default_texture = e.mem.u32(default_slot);
    e.mem.set_u32(default_slot, layer_texture);
    e.mem.set_u32(layers + 4 * layer as u32, default_texture);
    let percent = percent_array(e, data, quadrant as u32);
    if percent == 0 {
        return;
    }
    for vertex in 0..QUADRANT_VERTICES {
        let mut sum = 0.0f32;
        for index in 0..6u16 {
            let opacity = fn_0053a830(e, this, quadrant, vertex as u16, index);
            sum = sum_f32(opacity, sum);
        }
        let mut rest = (1.0 - sum as f64) as f32;
        if (rest as f64) < e.global::<f64>(F64_ZERO) {
            rest = 0.0;
        }
        let row = percent_row(e, percent, vertex);
        e.mem.set_f32(row + 4 * layer as u32, rest);
    }
}

// Translated from 0053ae30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The total opacity of layer `layer` of quadrant `quadrant` over the 289
/// vertices: 0 for a quadrant of 4 or more, a layer of 6 or more or a layer
/// without a texture.
pub fn fn_0053ae30(e: &mut Engine, this: Ptr<TESObjectLAND>, quadrant: u8, layer: u16) -> f32 {
    let mut total = 0.0f32;
    if quadrant < 4 && layer < 6 && fn_0053a700(e, this, quadrant, layer) != 0 {
        for vertex in 0..QUADRANT_VERTICES {
            let opacity = fn_0053a830(e, this, quadrant, vertex as u16, layer);
            total = sum_f32(opacity, total);
        }
    }
    total
}

// Translated from 0053aeb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Makes the layer opacities of every quadrant consistent. For each
/// quadrant, for each layer whose texture is a `TESTexture` (a cast of the
/// layer texture): every other layer whose texture is one too, and
/// that `0048e4f0` says is not compatible with it, is merged into it (its
/// opacity added, then zeroed, and its texture cleared), and when the
/// quadrant's default texture is a `TESTexture` that is not compatible with
/// the layer's, the layer's opacities are zeroed and its texture cleared;
/// the layer's opacities are summed per vertex into a scratch array. The
/// empty layers are squeezed out ([`fn_0053abb0`]); then per vertex every
/// opacity moves up one place, opacity 0 becomes the share left over by the
/// others (`max(0, 1 - sum)` capped at 1, through `00404010` and `0040ebd0`)
/// and when the scratch total was above 1 the shifted opacities are divided
/// by their sum.
pub fn fn_0053aeb0(e: &mut Engine, this: Ptr<TESObjectLAND>) {
    for quadrant in 0..4u32 {
        let default_texture = fn_0053a5a0(e, this, quadrant as u8);
        let default_cast = texture_cast(e, default_texture);
        e.with_stack(0x484, |e, totals| {
            e.call(MEMORY_SET, &args![totals, 0u32, 0x484u32]);
            let data = loaded_data(e, this);
            let percent = percent_array(e, data, quadrant);
            for layer in 0..6u32 {
                let texture = fn_0053a700(e, this, quadrant as u8, layer as u16);
                let layer_cast = texture_cast(e, texture);
                if layer_cast != 0 {
                    for other in 0..6u32 {
                        if layer == other {
                            continue;
                        }
                        let other_texture = fn_0053a700(e, this, quadrant as u8, other as u16);
                        let other_cast = texture_cast(e, other_texture);
                        if other_cast == 0 {
                            continue;
                        }
                        if e.call(TEXTURES_DIFFER, &args![layer_cast, other_cast])
                            .bool()
                        {
                            continue;
                        }
                        for vertex in 0..QUADRANT_VERTICES {
                            let row = percent_row(e, percent, vertex);
                            let merged =
                                sum_f32(e.mem.f32(row + 4 * layer), e.mem.f32(row + 4 * other));
                            e.mem.set_f32(row + 4 * layer, merged);
                            e.mem.set_f32(row + 4 * other, 0.0);
                        }
                        fn_0053a760(e, this, quadrant as u8, other as u16, 0);
                    }
                    if default_cast != 0
                        && !e
                            .call(TEXTURES_DIFFER, &args![layer_cast, default_cast])
                            .bool()
                    {
                        for vertex in 0..QUADRANT_VERTICES {
                            let row = percent_row(e, percent, vertex);
                            e.mem.set_f32(row + 4 * layer, 0.0);
                        }
                        fn_0053a760(e, this, quadrant as u8, layer as u16, 0);
                    }
                }
                for vertex in 0..QUADRANT_VERTICES {
                    let row = percent_row(e, percent, vertex);
                    let total = sum_f32(
                        e.mem.f32(totals.addr() + 4 * vertex),
                        e.mem.f32(row + 4 * layer),
                    );
                    e.mem.set_f32(totals.addr() + 4 * vertex, total);
                }
            }
            let layers = layer_array(e, data, quadrant);
            let mut count = 6u32;
            let mut index = 0u32;
            while index < count {
                while e.mem.u32(layers + 4 * index) == 0 && index < count {
                    fn_0053abb0(e, this, quadrant as u8, index as u16);
                    count -= 1;
                }
                index += 1;
            }
            for vertex in 0..QUADRANT_VERTICES {
                let row = percent_row(e, percent, vertex);
                let mut sum = 0.0f32;
                for slot in (1..=6u32).rev() {
                    // 0053b324: the opacity moves through the x87 stack
                    // (`FLD dword; FSTP dword`), which quiets a signalling NaN.
                    let lower = e.mem.u32(row + 4 * (slot - 1));
                    e.mem.set_u32(row + 4 * slot, x87_quiet_bits(lower));
                    sum = sum_f32(sum, e.mem.f32(row + 4 * slot));
                }
                let rest = (1.0 - sum as f64) as f32;
                let at_least_zero = e.call(FLOAT_MAX, &args![0.0f32, rest]).f32();
                let share = e.call(FLOAT_MIN, &args![1.0f32, at_least_zero]).f32();
                e.mem.set_f32(row, share);
                if (e.mem.f32(totals.addr() + 4 * vertex) as f64) > e.global::<f64>(F64_ONE) {
                    for slot in (1..=6u32).rev() {
                        let scaled = e.mem.f32(row + 4 * slot) as f64 / sum as f64;
                        e.mem.set_f32(row + 4 * slot, scaled as f32);
                    }
                }
            }
        });
    }
}

// ---- Coordinates of a position in the land -------------------------------------

// Translated from 0053b450 (decompiled, FalloutNV.exe 1.4.0.525)
/// The world position of vertex `vertex` of quadrant `quadrant`, written
/// to `out` (an `NiPoint3`): the cell corner (4096 times the cell
/// coordinates, taken from `cell_x` and `cell_y` when they are given, else
/// from the land), plus 2048 for a quadrant in the right column (bit 0) or
/// the upper row (bit 1), plus 128 times the vertex's column (`vertex %
/// 17`) and row (`vertex / 17`); z is 0. Returns `out`.
pub fn fn_0053b450(
    e: &mut Engine,
    this: Ptr<TESObjectLAND>,
    out: Ptr,
    quadrant: u8,
    vertex: u16,
    cell_x: Ptr,
    cell_y: Ptr,
) -> Ptr {
    e.with_stack(12, |e, local| {
        e.call(NI_POINT3_DEFAULT_CONSTRUCT, &args![local]);
        let mut x: f32 = e.global(F32_4096);
        let mut y: f32 = e.global(F32_4096);
        let column = if cell_x.is_null() {
            fn_00533fd0(e, this)
        } else {
            e.mem.i32(cell_x.addr())
        };
        x = (column as f64 * x as f64) as f32;
        let row = if cell_y.is_null() {
            fn_00534010(e, this)
        } else {
            e.mem.i32(cell_y.addr())
        };
        y = (row as f64 * y as f64) as f32;
        let right = if quadrant & 1 != 0 { 0x800 } else { 0 };
        x = sum_f32(right as f32, x);
        let upper = if quadrant & 2 != 0 { 0x800 } else { 0 };
        y = sum_f32(upper as f32, y);
        x = sum_f32((((vertex as i32) % 0x11) << 7) as f32, x);
        y = sum_f32((((vertex as i32) / 0x11) << 7) as f32, y);
        e.mem.set_f32(out.addr(), x);
        e.mem.set_f32(out.addr() + 4, y);
        e.mem.set_f32(out.addr() + 8, 0.0);
        out
    })
}

/// `trunc(value / divisor)` through the compiler's float to integer helper
/// (`_ftol2`), with the correction `GetCoordData` applies on an exact
/// boundary: when the integer part of `value` is a non-zero multiple of
/// `modulus`, the block is the one below.
fn block_index(value: f32, divisor: f64, modulus: i32) -> i32 {
    let mut index = x87_truncate(value as f64 / divisor);
    let integer = x87_truncate(value as f64);
    if integer % modulus == 0 && integer != 0 {
        index = index.wrapping_sub(1);
    }
    index
}

// Translated from 0053b550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectLAND::GetCoordData` (Xbox PDB): works out where the position
/// `position` (two floats, world x and y) falls in the land and fills
/// `out` (a [`CoordData`]): the offsets from the cell corner, the quadrant
/// (2048 units) and its row and column, the offsets inside the quadrant,
/// the 128-unit block, the offsets inside the block, the triangle of the
/// block (two diagonals, by the block's parity) with its three vertices,
/// and the nearest vertex (the block's vertex when the position is on the
/// grid, else the nearest of the triangle's three by distance from the
/// position, through `NiPoint2`s); at +0x30 the position of that vertex plus
/// the cell centre vector. When `recurse` is set the call is repeated once
/// for that position (with `recurse` clear). Returns false without loaded
/// vertex or mesh arrays, or when the position is outside the cell.
pub fn tes_object_land_get_coord_data(
    e: &mut Engine,
    this: Ptr<TESObjectLAND>,
    out: Ptr<CoordData>,
    position: Ptr,
    recurse: bool,
) -> bool {
    let cell_size = e.global::<f64>(F64_4096);
    let corner_x = (fn_00533fd0(e, this) as f64 * cell_size) as f32;
    let corner_y = (fn_00534010(e, this) as f64 * cell_size) as f32;
    let data = loaded_data(e, this);
    if data.is_null() {
        return false;
    }
    if e.get(data, LoadedLandData::ppVertices).is_null()
        && e.get(data, LoadedLandData::ppMesh).is_null()
    {
        return false;
    }
    let x = e.mem.f32(position.addr());
    let y = e.mem.f32(position.addr() + 4);
    if corner_x > x {
        return false;
    }
    if x > (corner_x as f64 + cell_size) as f32 {
        return false;
    }
    if corner_y > y {
        return false;
    }
    if y > (corner_y as f64 + cell_size) as f32 {
        return false;
    }
    let cell_offset_x = (x as f64 - corner_x as f64) as f32;
    e.set(out, CoordData::CellOffsetX, cell_offset_x);
    let cell_offset_y = (y as f64 - corner_y as f64) as f32;
    e.set(out, CoordData::CellOffsetY, cell_offset_y);
    let two_thousand = e.global::<f64>(F64_2048);
    let column = block_index(cell_offset_x, two_thousand, 2048);
    e.set(out, CoordData::QuadrantColumn, column);
    let row = block_index(cell_offset_y, two_thousand, 2048);
    e.set(out, CoordData::QuadrantRow, row);
    let quadrant = column.wrapping_add(row.wrapping_mul(2));
    e.set(out, CoordData::Quadrant, quadrant);
    let quadrant_x = (column.wrapping_shl(11) as f64 + corner_x as f64) as f32;
    let quadrant_y = (row.wrapping_shl(11) as f64 + corner_y as f64) as f32;
    let quadrant_offset_x = (x as f64 - quadrant_x as f64) as f32;
    e.set(out, CoordData::QuadrantOffsetX, quadrant_offset_x);
    let quadrant_offset_y = (y as f64 - quadrant_y as f64) as f32;
    e.set(out, CoordData::QuadrantOffsetY, quadrant_offset_y);
    with_cell_centre(e, this, |e, centre| {
        let block_size = e.global::<f64>(F64_128);
        let block_column = block_index(quadrant_offset_x, block_size, 128);
        e.set(out, CoordData::BlockColumn, block_column);
        let block_row = block_index(quadrant_offset_y, block_size, 128);
        e.set(out, CoordData::BlockRow, block_row);
        let block_x = (block_column.wrapping_shl(7) as f64 + quadrant_x as f64) as f32;
        let block_y = (block_row.wrapping_shl(7) as f64 + quadrant_y as f64) as f32;
        let block_offset_x = (x as f64 - block_x as f64) as f32;
        e.set(out, CoordData::BlockOffsetX, block_offset_x);
        let block_offset_y = (y as f64 - block_y as f64) as f32;
        e.set(out, CoordData::BlockOffsetY, block_offset_y);
        e.set(out, CoordData::QuadrantCopy, quadrant);
        let first = block_row.wrapping_mul(0x11).wrapping_add(block_column);
        let next_row = block_row
            .wrapping_add(1)
            .wrapping_mul(0x11)
            .wrapping_add(block_column);
        if block_column.wrapping_add(block_row) % 2 == 0 {
            e.set(out, CoordData::OddBlock, 0u8);
            e.set(out, CoordData::TriangleVertex0, first);
            e.set(out, CoordData::TriangleVertex1, next_row.wrapping_add(1));
            if block_offset_y < block_offset_x {
                e.set(out, CoordData::TriangleVertex2, first.wrapping_add(1));
                e.set(out, CoordData::SecondTriangle, 0u8);
            } else {
                e.set(out, CoordData::TriangleVertex2, next_row);
                e.set(out, CoordData::SecondTriangle, 1u8);
            }
        } else {
            e.set(out, CoordData::OddBlock, 1u8);
            e.set(out, CoordData::TriangleVertex0, first.wrapping_add(1));
            e.set(out, CoordData::TriangleVertex1, next_row);
            // 0053b9c3: `FLD; FADD dword` rounds the sum to the control word's
            // precision before `FCOMP` with 128.
            let offset_sum = x87_add(f64::from(block_offset_x), f64::from(block_offset_y));
            if offset_sum > e.global::<f64>(F64_128) {
                e.set(out, CoordData::TriangleVertex2, next_row.wrapping_add(1));
                e.set(out, CoordData::SecondTriangle, 1u8);
            } else {
                e.set(out, CoordData::TriangleVertex2, first);
                e.set(out, CoordData::SecondTriangle, 0u8);
            }
        }
        let on_grid = x87_truncate(x as f64) % 128 == 0 && x87_truncate(y as f64) % 128 == 0;
        if on_grid {
            let rows = x87_truncate(quadrant_offset_y as f64) / 128;
            let columns = x87_truncate(quadrant_offset_x as f64) / 128;
            let index = rows.wrapping_mul(0x11).wrapping_add(columns);
            let vertex = e.call(FLOAT_TO_INT, &args![index as f32]).i32();
            e.set(out, CoordData::NearestVertex, vertex);
        } else {
            let mut nearest_distance = 0.0f32;
            let mut nearest = 0u32;
            e.with_stack(8, |e, target| {
                // 0053bab1: both pass through `FLD dword; FSTP dword [ESP]`.
                e.call(
                    NI_POINT2_CONSTRUCT,
                    &args![target, x87_quiet(x), x87_quiet(y)],
                );
                for candidate in 0..3u32 {
                    let index = e
                        .mem
                        .i32(out.addr() + CoordData::TriangleVertex2.off + 4 * candidate);
                    let distance = e.with_stack(12, |e, vertex| {
                        e.call(NI_POINT3_DEFAULT_CONSTRUCT, &args![vertex]);
                        fn_00534240(e, this, quadrant as u32, index, vertex);
                        let vertex_x = e.mem.f32(vertex.addr());
                        let vertex_y = e.mem.f32(vertex.addr() + 4);
                        let centre_x = e.mem.f32(centre.addr());
                        let centre_y = e.mem.f32(centre.addr() + 4);
                        let point_y = sum_f32(centre_y, vertex_y);
                        let point_x = sum_f32(centre_x, vertex_x);
                        e.with_stack(8, |e, point| {
                            e.with_stack(8, |e, difference| {
                                e.call(NI_POINT2_CONSTRUCT, &args![point, point_x, point_y]);
                                e.call(NI_POINT2_SUBTRACT, &args![point, difference, target]);
                                e.call(NI_POINT2_LENGTH, &args![difference]).f32()
                            })
                        })
                    });
                    if candidate == 0 || distance < nearest_distance {
                        nearest = candidate;
                        nearest_distance = distance;
                    }
                }
            });
            let vertex = e
                .mem
                .i32(out.addr() + CoordData::TriangleVertex2.off + 4 * nearest);
            e.set(out, CoordData::NearestVertex, vertex);
        }
        let nearest_vertex = e.get(out, CoordData::NearestVertex);
        e.with_stack(12, |e, vertex| {
            e.call(NI_POINT3_DEFAULT_CONSTRUCT, &args![vertex]);
            fn_00534240(e, this, quadrant as u32, nearest_vertex, vertex);
            e.with_stack(12, |e, sum| {
                let result = e.call(NI_POINT3_ADD, &args![centre, sum, vertex]).u32();
                copy_words(e, result, out.addr() + CoordData::VertexPositionX.off, 3);
            });
        });
        true
    });
    if recurse {
        let vertex_position = out.addr() + CoordData::VertexPositionX.off;
        return tes_object_land_get_coord_data(e, this, out, Ptr::new(vertex_position), false);
    }
    true
}

// ---- Grass parameters -------------------------------------------------------------

/// The offsets (in vertices) of the nine neighbours of a vertex in the
/// 17 x 17 grid, in the order the grass records keep their samples.
const GRASS_SAMPLE_OFFSETS: [i32; 9] = [-0x12, -0x11, -0x10, -1, 0, 1, 0x10, 0x11, 0x12];

/// Fills a new grass record (0x44 bytes, constructed by
/// [`fn_0053ca60`]) from the grass form `form`: name, form id, the
/// setting float (`0053ca40`), four floats read through the form's virtual
/// getters (slots 0x1c8, 0x1c0, 0x1b0 and 0x1b8, stored at +0x18, +0x10,
/// +8 and +0xc) and three bytes (slots 0x1d0, 0x1e0 and 0x1d8, stored at
/// +0x1c, +0x1e and +0x1d), and returns the byte from slot 0x180 divided by
/// 100 as a float (the value each sample takes when it passes).
fn fill_grass_record(e: &mut Engine, record: u32, form: u32) -> f32 {
    let name = e.call(GRASS_FORM_NAME, &args![form]).u32();
    e.mem.set_u32(record, name);
    let id = e.call(FORM_ID, &args![form]).u32();
    e.mem.set_u32(record + 4, id);
    let setting = fn_0053ca40(e);
    e.mem.set_f32(record + 0x14, setting);
    let value = e.vcall(form, GRASS_SLOT_PARAMETER_AT_18, &args![]).f32();
    e.mem.set_f32(record + 0x18, value);
    let value = e.vcall(form, GRASS_SLOT_PARAMETER_AT_10, &args![]).f32();
    e.mem.set_f32(record + 0x10, value);
    let value = e.vcall(form, GRASS_SLOT_PARAMETER_AT_8, &args![]).f32();
    e.mem.set_f32(record + 8, value);
    let value = e.vcall(form, GRASS_SLOT_PARAMETER_AT_0C, &args![]).f32();
    e.mem.set_f32(record + 0xc, value);
    let value = e.vcall(form, GRASS_SLOT_BYTE_AT_1C, &args![]).u8();
    e.mem.set_u8(record + 0x1c, value);
    let value = e.vcall(form, GRASS_SLOT_BYTE_AT_1E, &args![]).u8();
    e.mem.set_u8(record + 0x1e, value);
    let value = e.vcall(form, GRASS_SLOT_BYTE_AT_1D, &args![]).u8();
    e.mem.set_u8(record + 0x1d, value);
    let fraction = e.vcall(form, GRASS_SLOT_PERCENT, &args![]).u8();
    (fraction as f64 / e.global::<f64>(F64_HUNDRED)) as f32
}

/// After the nine samples of a grass record (floats at +0x20 to +0x40) are
/// set: when their mean is below the threshold, clears them all.
fn clear_weak_samples(e: &mut Engine, record: u32, threshold: f32) {
    let mut sum = 0.0f32;
    for sample in 0..9u32 {
        sum = sum_f32(sum, e.mem.f32(record + 0x20 + 4 * sample));
    }
    // 0053c27c / 0053c977: the mean stays on the x87 stack (`FDIV qword`,
    // rounded to the control word's precision) for the compare.
    if (threshold as f64) > x87_div(f64::from(sum), e.global::<f64>(F64_NINE)) {
        for sample in 0..9u32 {
            e.mem.set_f32(record + 0x20 + 4 * sample, 0.0);
        }
    }
}

// Translated from 0053bc10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds the grass parameter maps of the land: false without loaded data.
/// The threshold is the float setting at `011c9f38` clamped to 0 .. 1 (above
/// 1 it becomes 0.9); the maximum is the integer setting at `011c9f74`; the
/// step is the integer setting at `011c9fb8` stored at `011c9f6c` (2 unless
/// it is 1, 2, 4 or 8). For each quadrant and each vertex of the grid with
/// row and column `step, 3 * step, ...` below 16 (key `row * 17 + column`)
/// a table of 16 record pointers is allocated and cleared, then filled from
/// the grass of the quadrant's default texture and of each layer texture
/// whose opacity at the vertex or its eight neighbours exceeds 0.1: for
/// each grass form of the texture's list (at most the maximum plus one
/// forms, and at most 16 records), a 0x44-byte record gets the form's
/// parameters and nine samples, one per neighbour: the value of slot 0x180 when
/// the neighbour's opacity for the default texture (`fn_0053a7a0`) or for
/// the layer exceeds the threshold, else 0; a record whose samples average
/// below the threshold has them cleared. A table with a first record is
/// added to the quadrant's grass map under the vertex key, else freed.
pub fn fn_0053bc10(e: &mut Engine, this: Ptr<TESObjectLAND>) -> bool {
    let data = loaded_data(e, this);
    if data.is_null() {
        return false;
    }
    let setting = e
        .call(SETTING_GET_FLOAT_ADDRESS, &args![GRASS_SETTING_THRESHOLD])
        .u32();
    let mut threshold = e.mem.f32(setting);
    if threshold < 0.0 {
        threshold = 0.0;
    }
    if threshold as f64 > e.global::<f64>(F64_ONE) {
        threshold = e.global::<f64>(F64_POINT_NINE) as f32;
    }
    let setting = e
        .call(SETTING_GET_INT_ADDRESS, &args![GRASS_SETTING_MAXIMUM])
        .u32();
    let maximum = e.mem.i32(setting);
    let setting = e
        .call(SETTING_GET_INT_ADDRESS, &args![GRASS_SETTING_STEP])
        .u32();
    let step = e.mem.i32(setting);
    e.mem.set_i32(GRASS_STEP, step);
    if !matches!(step, 1 | 2 | 4 | 8) {
        e.mem.set_i32(GRASS_STEP, 2);
    }
    for quadrant in 0..4u32 {
        let step = e.mem.i32(GRASS_STEP);
        let mut column = step;
        while column < 0x10 {
            let mut row = e.mem.i32(GRASS_STEP);
            while row < 0x10 {
                grass_vertex(e, this, data, quadrant, row, column, threshold, maximum);
                row += e.mem.i32(GRASS_STEP) * 2;
            }
            column += e.mem.i32(GRASS_STEP) * 2;
        }
    }
    true
}

/// The body of [`fn_0053bc10`] for one vertex: `row` and `column` of the
/// 17 x 17 grid of quadrant `quadrant`.
#[allow(clippy::too_many_arguments)]
fn grass_vertex(
    e: &mut Engine,
    this: Ptr<TESObjectLAND>,
    data: Ptr<LoadedLandData>,
    quadrant: u32,
    row: i32,
    column: i32,
    threshold: f32,
    maximum: i32,
) {
    let table_size = 16u32;
    let allocation = e.call(ALLOCATE, &args![table_size * 4]).u32();
    let table = allocation;
    e.call(MEMORY_SET, &args![table, 0u32, table_size * 4]);
    let key = row * 0x11 + column;
    let mut count = 0u32;
    let default_texture = e
        .mem
        .u32(data.addr() + LoadedLandData::pDefQuadTexture.off + 4 * quadrant);
    if default_texture != 0 {
        let mut position = e.call(LIST_FIRST_POSITION, &args![default_texture]).u32();
        let mut used = 0i32;
        while position != 0 && count < table_size && used <= maximum {
            let element = e.call(LIST_ELEMENT, &args![position]).u32();
            let form = e.mem.u32(element);
            if form != 0 {
                used += 1;
                let record = new_grass_record(e);
                e.mem.set_u32(table + 4 * count, record);
                let fraction = fill_grass_record(e, record, form);
                for (sample, offset) in GRASS_SAMPLE_OFFSETS.iter().enumerate() {
                    let free = fn_0053a7a0(e, this, quadrant as u8, (key + offset) as u16);
                    let value = if (threshold as f64) < free as f64 {
                        fraction
                    } else {
                        0.0
                    };
                    e.mem.set_f32(record + 0x20 + 4 * sample as u32, value);
                }
                clear_weak_samples(e, record, threshold);
                count += 1;
            }
            position = e.call(LIST_NEXT_POSITION, &args![position]).u32();
        }
    }
    for layer in 0..6u32 {
        let layers = layer_array(e, data, quadrant);
        let layer_texture = e.mem.u32(layers + 4 * layer);
        if layer_texture == 0 {
            continue;
        }
        let percent = percent_array(e, data, quadrant);
        if percent == 0 {
            continue;
        }
        let mut strong = false;
        for offset in GRASS_SAMPLE_OFFSETS {
            let row_pointer = percent_row(e, percent, (key + offset) as u32);
            if (e.mem.f32(row_pointer + 4 * layer) as f64) > e.global::<f64>(F64_TENTH) {
                strong = true;
                break;
            }
        }
        if !strong {
            continue;
        }
        let mut used = 0i32;
        let mut position = e.call(LIST_FIRST_POSITION, &args![layer_texture]).u32();
        while position != 0 && count < table_size && used <= maximum {
            let element = e.call(LIST_ELEMENT, &args![position]).u32();
            let form = e.mem.u32(element);
            if form != 0 {
                let record = new_grass_record(e);
                e.mem.set_u32(table + 4 * count, record);
                used += 1;
                e.call(MEMORY_SET, &args![record + 0x20, 0u32, 0x24u32]);
                let fraction = fill_grass_record(e, record, form);
                for (sample, offset) in GRASS_SAMPLE_OFFSETS.iter().enumerate() {
                    let row_pointer = percent_row(e, percent, (key + offset) as u32);
                    let opacity = e.mem.f32(row_pointer + 4 * layer);
                    let value = if (threshold as f64) < opacity as f64 {
                        fraction
                    } else {
                        0.0
                    };
                    e.mem.set_f32(record + 0x20 + 4 * sample as u32, value);
                }
                clear_weak_samples(e, record, threshold);
                count += 1;
            }
            position = e.call(LIST_NEXT_POSITION, &args![position]).u32();
        }
    }
    if e.mem.u32(table) != 0 {
        let map = data.addr() + GRASS_MAP_OFFSET + (quadrant << 4);
        e.call(GRASS_MAP_SET_AT, &args![map, key as u32, table]);
    } else {
        e.call(DEALLOCATE, &args![table]);
    }
}

/// Allocates a grass record (0x44 bytes) and constructs it with
/// [`fn_0053ca60`]; the pointer is 0 when the allocation failed. The first
/// loop of [`fn_0053bc10`] also clears the new record with `memset`.
fn new_grass_record(e: &mut Engine) -> u32 {
    let memory = e.call(ALLOCATE, &args![GRASS_RECORD_SIZE]).u32();
    let record = if memory != 0 {
        fn_0053ca60(e, Ptr::new(memory)).addr()
    } else {
        0
    };
    e.call(MEMORY_SET, &args![record, 0u32, GRASS_RECORD_SIZE]);
    record
}

// Translated from 0053ca40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The value of the float setting at `011c8dcc` (read through
/// `00403e20`, the address of the setting's value).
pub fn fn_0053ca40(e: &mut Engine) -> f32 {
    let value = e
        .call(SETTING_GET_FLOAT_ADDRESS, &args![GRASS_DENSITY_SETTING])
        .u32();
    e.mem.f32(value)
}

// Translated from 0053ca60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of a grass parameter record: runs `00b600e0` on it. Returns
/// `this`.
pub fn fn_0053ca60(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(GRASS_RECORD_CONSTRUCT, &args![this]);
    this
}

// Translated from 0053ca80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The grass table of vertex `key` of quadrant `quadrant`: looked up in the
/// quadrant's grass map (`00853130`), or 0 without loaded data, for a
/// quadrant of 4 or more or a key outside 0x12 .. 0x10E.
pub fn fn_0053ca80(e: &mut Engine, this: Ptr<TESObjectLAND>, quadrant: i32, key: i32) -> u32 {
    let data = loaded_data(e, this);
    if data.is_null() || quadrant >= 4 || key + 0x12 >= 0x121 || key - 0x12 < 0 {
        return 0;
    }
    e.with_stack(4, |e, found| {
        let map = data.addr() + GRASS_MAP_OFFSET + ((quadrant as u32) << 4);
        if e.call(GRASS_MAP_GET_AT, &args![map, key as u32, found])
            .bool()
        {
            e.mem.u32(found.addr())
        } else {
            0
        }
    })
}

// ---- Normal at a position -----------------------------------------------------------

// Translated from 0053caf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The position and normal at the place `info` (a [`CoordData`]) describes,
/// written to `out_position` and `out_normal`; always true. When the
/// vertex's position (`vertex + cell centre`) has the same x and y
/// (`004390c0`) as the grid position of that vertex (`fn_0053b450`), the
/// position and normal are those of the nearest vertex. Otherwise the
/// fractional position inside the 128-unit block (`fmod(x, 128) / 128`
/// and the same for y of that grid position) weighs the normals of the
/// triangle's three vertices (the weighting depends on the block's two
/// flags), the result is unitized as the position, and the normal is the
/// unit cross product of two edges of the triangle (in the order that
/// depends on the flags).
pub fn fn_0053caf0(
    e: &mut Engine,
    this: Ptr<TESObjectLAND>,
    info: Ptr<CoordData>,
    out_position: Ptr,
    out_normal: Ptr,
) -> bool {
    let quadrant = e.get(info, CoordData::Quadrant);
    let nearest = e.get(info, CoordData::NearestVertex);
    e.with_stack(12, |e, vertex| {
        e.call(NI_POINT3_DEFAULT_CONSTRUCT, &args![vertex]);
        e.with_stack(12, |e, grid| {
            fn_0053b450(
                e,
                this,
                grid,
                quadrant as u8,
                nearest as u16,
                Ptr::NULL,
                Ptr::NULL,
            );
            fn_00534240(e, this, quadrant as u32, nearest, vertex);
            e.with_stack(12, |e, centre| {
                e.call(NI_POINT3_DEFAULT_CONSTRUCT, &args![centre]);
                let words = cell_centre_vector(e, this);
                write_words(e, centre.addr(), &words);
                e.with_stack(12, |e, position| {
                    e.call(NI_POINT3_ADD, &args![vertex, position, centre]);
                    e.mem.set_u32(position.addr() + 8, 0);
                    let same = e.call(NI_POINT3_SAME_XY, &args![position, grid]).bool();
                    if same {
                        fn_00534390(e, this, quadrant as u32, nearest, out_position);
                        copy_words(e, out_position.addr(), out_normal.addr(), 3);
                        return;
                    }
                    let grid_x = e.mem.f32(grid.addr());
                    let grid_y = e.mem.f32(grid.addr() + 4);
                    let block: f32 = e.global(F32_128);
                    let u = (e.call(FLOAT_MODULO, &args![grid_x, block]).f32() as f64
                        / e.global::<f64>(F64_128)) as f32;
                    let v = (e.call(FLOAT_MODULO, &args![grid_y, block]).f32() as f64
                        / e.global::<f64>(F64_128)) as f32;
                    triangle_normal(e, this, info, quadrant, u, v, out_position, out_normal);
                });
            });
        });
    });
    true
}

/// The interpolated normal and the face normal of the triangle in `info`
/// at the fractional position `(u, v)`: see [`fn_0053caf0`].
#[allow(clippy::too_many_arguments)]
fn triangle_normal(
    e: &mut Engine,
    this: Ptr<TESObjectLAND>,
    info: Ptr<CoordData>,
    quadrant: i32,
    u: f32,
    v: f32,
    out_position: Ptr,
    out_normal: Ptr,
) {
    let odd = e.get(info, CoordData::OddBlock) != 0;
    let second = e.get(info, CoordData::SecondTriangle) != 0;
    let indices = [
        e.get(info, CoordData::TriangleVertex2),
        e.get(info, CoordData::TriangleVertex0),
        e.get(info, CoordData::TriangleVertex1),
    ];
    // Twelve-byte slots of one scratch block: the three vertex normals
    // (0 to 2), the temporaries of the weighting (3 to 5), the three
    // results of `combine` (6 to 8), the edges of the face normal (9 to
    // 12) and its result (13).
    e.with_stack(12 * 14, |e, scratch| {
        let slot = |index: u32| scratch.addr() + 12 * index;
        let (a, b, c) = (slot(0), slot(1), slot(2));
        for vector in [a, b, c] {
            e.call(NI_POINT3_DEFAULT_CONSTRUCT, &args![vector]);
        }
        for (vector, index) in [a, b, c].into_iter().zip(indices) {
            fn_00534390(e, this, quadrant as u32, index, Ptr::new(vector));
        }
        let one_minus_u = (1.0 - u as f64) as f32;
        let one_minus_v = (1.0 - v as f64) as f32;
        let scale = |e: &mut Engine, target: u32, scalar: f32, vector: u32| -> u32 {
            e.call(NI_POINT3_SCALE_BY, &args![target, scalar, vector])
                .u32()
        };
        let (t1, t2, t3) = (slot(3), slot(4), slot(5));
        let results = [slot(6), slot(7), slot(8)];
        let weighted = match (odd, second) {
            (false, false) => {
                // ((1 - u) * b + u * a) * (1 - v) + v * c
                let last = scale(e, t1, v, c);
                let second_term = scale(e, t2, u, a);
                let first = scale(e, t3, one_minus_u, b);
                combine(e, [first, second_term, last], one_minus_v, results)
            }
            (false, true) => {
                // ((1 - u) * a + u * c) * v + (1 - v) * b
                let last = scale(e, t1, one_minus_v, b);
                let second_term = scale(e, t2, u, c);
                let first = scale(e, t3, one_minus_u, a);
                combine(e, [first, second_term, last], v, results)
            }
            (true, false) => {
                // ((1 - u) * a + u * b) * (1 - v) + v * c
                let last = scale(e, t1, v, c);
                let second_term = scale(e, t2, u, b);
                let first = scale(e, t3, one_minus_u, a);
                combine(e, [first, second_term, last], one_minus_v, results)
            }
            (true, true) => {
                // ((1 - u) * c + u * a) * v + (1 - v) * b
                let last = scale(e, t1, one_minus_v, b);
                let second_term = scale(e, t2, u, a);
                let first = scale(e, t3, one_minus_u, c);
                combine(e, [first, second_term, last], v, results)
            }
        };
        write_words(e, out_position.addr(), &weighted);
        e.call(NI_POINT3_NORMALIZE, &args![out_position]);
        // The face normal from two edges of the triangle.
        let (edge_a, other_a, edge_b, other_b) = (slot(9), slot(10), slot(11), slot(12));
        for vector in [edge_a, other_a, edge_b, other_b] {
            e.call(NI_POINT3_DEFAULT_CONSTRUCT, &args![vector]);
        }
        fn_00534240(e, this, quadrant as u32, indices[0], Ptr::new(edge_a));
        fn_00534240(e, this, quadrant as u32, indices[1], Ptr::new(other_a));
        e.call(NI_POINT3_SUBTRACT_IN_PLACE, &args![edge_a, other_a]);
        fn_00534240(e, this, quadrant as u32, indices[0], Ptr::new(edge_b));
        fn_00534240(e, this, quadrant as u32, indices[2], Ptr::new(other_b));
        e.call(NI_POINT3_SUBTRACT_IN_PLACE, &args![edge_b, other_b]);
        let target = Ptr::new(slot(13));
        let cross = if odd != second {
            ni_point3_unit_cross(e, Ptr::new(edge_a), target, Ptr::new(edge_b))
        } else {
            ni_point3_unit_cross(e, Ptr::new(edge_b), target, Ptr::new(edge_a))
        };
        copy_words(e, cross.addr(), out_normal.addr(), 3);
    });
}

/// `((first + second) * scalar) + last` as the code evaluates it
/// (`00439e90`, `0045bb20`, `00439e90`) with the three result slots
/// `slots`; the words of the result.
fn combine(e: &mut Engine, terms: [u32; 3], scalar: f32, slots: [u32; 3]) -> Vec<u32> {
    let [first, second, last] = terms;
    let [sum, scaled, total] = slots;
    e.call(NI_POINT3_ADD, &args![first, sum, second]);
    e.call(NI_POINT3_TIMES_SCALAR, &args![sum, scaled, scalar]);
    e.call(NI_POINT3_ADD, &args![scaled, total, last]);
    read_words(e, total, 3)
}

// Translated from 0053d1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPoint3::UnitCross` (Xbox PDB): the cross product of `this` and
/// `other` as a unit vector in `out`, or the zero vector when its length is
/// at most 1e-6. Returns `out`.
pub fn ni_point3_unit_cross(e: &mut Engine, this: Ptr, out: Ptr, other: Ptr) -> Ptr {
    let a = [
        e.mem.f32(this.addr()),
        e.mem.f32(this.addr() + 4),
        e.mem.f32(this.addr() + 8),
    ];
    let b = [
        e.mem.f32(other.addr()),
        e.mem.f32(other.addr() + 4),
        e.mem.f32(other.addr() + 8),
    ];
    let z = (a[0] as f64 * b[1] as f64 - a[1] as f64 * b[0] as f64) as f32;
    let y = (a[2] as f64 * b[0] as f64 - a[0] as f64 * b[2] as f64) as f32;
    let x = (a[1] as f64 * b[2] as f64 - a[2] as f64 * b[1] as f64) as f32;
    e.with_stack(12, |e, cross| {
        e.call(NI_POINT3_CONSTRUCT, &args![cross, x, y, z]);
        let length = e.call(NI_POINT3_LENGTH, &args![cross]).f32();
        if (length as f64) > e.global::<f64>(F64_MILLIONTH) {
            fn_0053d280(e, cross, out, length);
        } else {
            e.call(NI_POINT3_CONSTRUCT, &args![out, 0.0f32, 0.0f32, 0.0f32]);
        }
    });
    out
}

// Translated from 0053d280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scales `this` (an `NiPoint3`) by `1 / length` into `out`. Returns `out`.
pub fn fn_0053d280(e: &mut Engine, this: Ptr, out: Ptr, length: f32) -> Ptr {
    let inverse = (1.0 / length as f64) as f32;
    let x = (inverse as f64 * e.mem.f32(this.addr()) as f64) as f32;
    let y = (inverse as f64 * e.mem.f32(this.addr() + 4) as f64) as f32;
    let z = (inverse as f64 * e.mem.f32(this.addr() + 8) as f64) as f32;
    e.call(NI_POINT3_CONSTRUCT, &args![out, x, y, z]);
    out
}

// Translated from 0053d2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The position and normal at the world position `position` (two floats):
/// [`tes_object_land_get_coord_data`] (without recursion) for it, then
/// [`fn_0053caf0`]; false when the position is outside the land.
pub fn fn_0053d2e0(
    e: &mut Engine,
    this: Ptr<TESObjectLAND>,
    position: Ptr,
    out_position: Ptr,
    out_normal: Ptr,
) -> bool {
    e.with_stack(COORD_DATA_SIZE, |e, info| {
        fn_0053a510(e, info);
        if !tes_object_land_get_coord_data(e, this, info.cast(), position, false) {
            return false;
        }
        fn_0053caf0(e, this, info.cast(), out_position, out_normal)
    })
}

// ---- The eight neighbours of a land and the border normals --------------------------

// Translated from 0053d330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finds the lands of the eight cells around this land's cell and stores
/// them in `lands` (eight pointers, in the order of [`NEIGHBOUR_OFFSETS`]):
/// for each, the data handler (`011c3f2c`) finds the cell (`00461c20`) in
/// the land's world space and the cell's land (`TESObjectCELL::GetLand`)
/// is stored when it has vertex arrays. A land without them is stored
/// (after `LoadVertices` without building meshes, and the entry of `loaded`
/// set to 1) when `load_missing` is set and `loaded` (eight bytes) is given.
/// Otherwise the entry is 0. Nothing when `lands` is null.
pub fn fn_0053d330(
    e: &mut Engine,
    this: Ptr<TESObjectLAND>,
    lands: Ptr,
    load_missing: bool,
    loaded: Ptr,
) {
    if lands.is_null() {
        return;
    }
    for (index, (dx, dy)) in NEIGHBOUR_OFFSETS.into_iter().enumerate() {
        let index = index as u32;
        let world_space = fn_005340b0(e, this).addr();
        let y = fn_00534010(e, this).wrapping_add(dy);
        let x = fn_00533fd0(e, this).wrapping_add(dx);
        let handler = e.global::<u32>(DATA_HANDLER);
        let cell = e
            .call(TES_GET_CELL, &args![handler, x, y, world_space, 0u32])
            .u32();
        let slot = lands.addr() + 4 * index;
        if cell != 0 {
            let land = e.call(CELL_GET_LAND, &args![cell]).u32();
            if has_vertex_arrays(e, Ptr::new(land)) {
                let land = e.call(CELL_GET_LAND, &args![cell]).u32();
                e.mem.set_u32(slot, land);
                continue;
            }
        }
        if cell != 0 {
            let land = e.call(CELL_GET_LAND, &args![cell]).u32();
            if !has_vertex_arrays(e, Ptr::new(land)) && load_missing && !loaded.is_null() {
                let land = e.call(CELL_GET_LAND, &args![cell]).u32();
                e.mem.set_u32(slot, land);
                tes_object_land_load_vertices(e, Ptr::new(land), false);
                e.mem.set_u8(loaded.addr() + index, 1);
                continue;
            }
        }
        e.mem.set_u32(slot, 0);
    }
}

// Translated from 0053d8f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Recomputes the normals of the vertices on the border of the four
/// quadrants (vertex rows 0 and 16, columns 0 and 16 of each 17 x 17 block;
/// only those in `bounds`, see [`fn_0053da60`], when it is given) from the
/// neighbouring quadrants and lands ([`fn_0053e290`]). `lands` is the
/// neighbour array of [`fn_0053d330`] (filled here when it is null). When
/// all eight neighbours exist, sets flag 0x10 (`fn_00534200`). Nothing
/// when the vertices are not loaded.
pub fn fn_0053d8f0(e: &mut Engine, this: Ptr<TESObjectLAND>, lands: Ptr, bounds: Ptr) {
    if !fn_005394a0(e, this) {
        return;
    }
    e.with_stack(0x20, |e, local| {
        let lands = if lands.is_null() {
            fn_0053d330(e, this, local, false, Ptr::NULL);
            local
        } else {
            lands
        };
        e.with_stack(12, |e, scratch| {
            e.call(NI_POINT3_DEFAULT_CONSTRUCT, &args![scratch]);
            let data = loaded_data(e, this);
            for quadrant in 0..QUADRANTS {
                for vertex in 0..QUADRANT_VERTICES {
                    let border = !(0x11..=0x110).contains(&vertex)
                        || vertex % 0x11 == 0
                        || (vertex + 1) % 0x11 == 0;
                    if !border {
                        continue;
                    }
                    if !bounds.is_null() {
                        let vertices = e.get(data, LoadedLandData::ppVertices);
                        let at = element(e, vertices, quadrant) + vertex * 12;
                        if !fn_0053da60(e, this, bounds, Ptr::new(at)) {
                            continue;
                        }
                    }
                    let normals = e.get(data, LoadedLandData::ppNormals);
                    let at = element(e, normals, quadrant) + vertex * 12;
                    fn_0053e290(e, this, quadrant as i32, vertex as i32, Ptr::new(at), lands);
                }
            }
            let all_present = (0..8).all(|index| e.mem.u32(lands.addr() + 4 * index) != 0);
            if all_present {
                fn_00534200(e, this, true);
            }
        });
    });
}

// Translated from 0053da60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the position of `vertex` (an `NiPoint3` of the land's local
/// coordinates) lies inside `bounds` (four integers: left, top, right,
/// bottom) once the cell centre vector (cell corner + 2048) is added: x not
/// above the right edge or below the left edge, y not above the top edge
/// or below the bottom edge. Always true without `bounds`.
pub fn fn_0053da60(e: &mut Engine, this: Ptr<TESObjectLAND>, bounds: Ptr, vertex: Ptr) -> bool {
    if bounds.is_null() {
        return true;
    }
    let centre = cell_centre_vector(e, this);
    e.with_stack(12, |e, position| {
        write_words(e, position.addr(), &centre);
        e.call(NI_POINT3_ADD_IN_PLACE, &args![position, vertex]);
        let x = e.mem.f32(position.addr()) as f64;
        let y = e.mem.f32(position.addr() + 4) as f64;
        let left = e.mem.i32(bounds.addr()) as f64;
        let top = e.mem.i32(bounds.addr() + 4) as f64;
        let right = e.mem.i32(bounds.addr() + 8) as f64;
        let bottom = e.mem.i32(bounds.addr() + 0xc) as f64;
        !(x > right || y > top || x < left || y < bottom)
    })
}

// Translated from 0053db20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Recomputes the normals along the borders of the land with the
/// neighbours loaded for it: false unless the vertices are loaded and the
/// data does not come from the world space's land file (flag 0x400).
/// Loads the missing neighbours ([`fn_0053d330`] with `load_missing`),
/// saves the normals of every neighbour (8 x 4 blocks of 0xd8c bytes in a
/// 0x1b180-byte local), recomputes the normals of every neighbour
/// ([`fn_0053df30`]) and of this land, then the border normals
/// ([`fn_0053d8f0`]). Each normal of this land is then rounded to signed
/// bytes (`trunc(n * 127)`, divided by 127) and unitized again, and the
/// saved normals of the neighbours are restored; neighbours loaded here
/// are unloaded again. Returns true. (The stack probe and security cookie
/// of the 0x1b1d8-byte frame are not translated.)
pub fn fn_0053db20(e: &mut Engine, this: Ptr<TESObjectLAND>) -> bool {
    if !fn_005394a0(e, this) || fn_00534140(e, this) {
        return false;
    }
    e.with_stack(8, |e, loaded| {
        e.call(MEMORY_SET, &args![loaded, 0u32, 8u32]);
        e.with_stack(0x20, |e, lands| {
            fn_0053d330(e, this, lands, true, loaded);
            e.with_stack(0x1b180, |e, saved| {
                e.call(
                    VECTOR_CONSTRUCT,
                    &args![saved, 0xcu32, 0x2420u32, NI_POINT3_DEFAULT_CONSTRUCT],
                );
                for neighbour in 0..8u32 {
                    let land = e.mem.u32(lands.addr() + 4 * neighbour);
                    if land == 0 {
                        continue;
                    }
                    for quadrant in 0..4u32 {
                        let normals = fn_0053df00(e, Ptr::new(land));
                        let source = e.mem.u32(normals + 4 * quadrant);
                        let target = saved.addr() + neighbour * 0x3630 + quadrant * 0xd8c;
                        e.call(MEMORY_COPY, &args![target, source, 0xd8cu32]);
                    }
                    fn_0053df30(e, Ptr::new(land), Ptr::NULL);
                }
                fn_0053df30(e, this, Ptr::NULL);
                fn_0053d8f0(e, this, lands, Ptr::NULL);
                let data = loaded_data(e, this);
                let scale = e.global::<f64>(F64_127);
                for quadrant in 0..QUADRANTS {
                    for vertex in 0..QUADRANT_VERTICES {
                        let normals = e.get(data, LoadedLandData::ppNormals);
                        let normal = element(e, normals, quadrant) + vertex * 12;
                        e.call(NI_POINT3_NORMALIZE, &args![normal]);
                        let mut bytes = [0i8; 3];
                        for (axis, byte) in bytes.iter_mut().enumerate() {
                            let component = e.mem.f32(normal + 4 * axis as u32);
                            *byte = x87_truncate(component as f64 * scale) as i8;
                        }
                        for (axis, byte) in bytes.into_iter().enumerate() {
                            e.mem
                                .set_f32(normal + 4 * axis as u32, (byte as f64 / scale) as f32);
                        }
                        e.call(NI_POINT3_NORMALIZE, &args![normal]);
                    }
                }
                for neighbour in 0..8u32 {
                    let land = e.mem.u32(lands.addr() + 4 * neighbour);
                    if land == 0 {
                        continue;
                    }
                    for quadrant in 0..4u32 {
                        let normals = fn_0053df00(e, Ptr::new(land));
                        let target = e.mem.u32(normals + 4 * quadrant);
                        let source = saved.addr() + neighbour * 0x3630 + quadrant * 0xd8c;
                        e.call(MEMORY_COPY, &args![target, source, 0xd8cu32]);
                    }
                    if e.mem.u8(loaded.addr() + neighbour) != 0 {
                        tes_object_land_un_load_vertices(e, Ptr::new(land));
                    }
                }
            });
        });
    });
    true
}

// Translated from 0053df00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The land's `ppNormals` array, or 0 without loaded data.
pub fn fn_0053df00(e: &mut Engine, this: Ptr<TESObjectLAND>) -> u32 {
    let data = loaded_data(e, this);
    if data.is_null() {
        0
    } else {
        e.get(data, LoadedLandData::ppNormals).addr()
    }
}

// Translated from 0053df30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Recomputes the vertex normals of the land from its triangles (only the
/// vertices inside `bounds`, see [`fn_0053da60`], when it is given): every
/// normal in range is first reset to the default normal (`011f426c`) and
/// its "normal set" flag cleared; then for each of the 512 triangles of the
/// shared triangle list (u16 triples), the vector `(b - a) x (c - b)` is
/// passed through `00525340` and added to the normal of each of its
/// vertices that is in range; finally each quadrant's normals are
/// normalized (`NiPoint3::UnitizeVectors`, Xbox PDB). Nothing when the
/// vertices are not loaded.
pub fn fn_0053df30(e: &mut Engine, this: Ptr<TESObjectLAND>, bounds: Ptr) {
    if !fn_005394a0(e, this) {
        return;
    }
    let data = loaded_data(e, this);
    for quadrant in 0..QUADRANTS {
        for vertex in 0..QUADRANT_VERTICES {
            let in_range = bounds.is_null() || {
                let vertices = e.get(data, LoadedLandData::ppVertices);
                let at = element(e, vertices, quadrant) + vertex * 12;
                fn_0053da60(e, this, bounds, Ptr::new(at))
            };
            if in_range {
                let normals = e.get(data, LoadedLandData::ppNormals);
                let normal = element(e, normals, quadrant) + vertex * 12;
                copy_words(e, DEFAULT_NORMAL_WORDS, normal, 3);
                let set = e.get(data, LoadedLandData::ppNormalsSet);
                e.mem.set_u8(element(e, set, quadrant) + vertex, 0);
            }
        }
        let mut triangles = e.global::<u32>(DEFAULT_TRIANGLE_LIST);
        for _ in 0..0x200u32 {
            let a = e.mem.u16(triangles) as u32;
            let b = e.mem.u16(triangles + 2) as u32;
            let c = e.mem.u16(triangles + 4) as u32;
            triangles += 6;
            let vertices = e.get(data, LoadedLandData::ppVertices);
            let base = element(e, vertices, quadrant);
            e.with_stack(36, |e, scratch| {
                let first_edge = scratch.addr();
                let second_edge = scratch.addr() + 12;
                let cross = scratch.addr() + 24;
                e.call(
                    NI_POINT3_SUBTRACT,
                    &args![base + b * 12, first_edge, base + a * 12],
                );
                e.call(
                    NI_POINT3_SUBTRACT,
                    &args![base + c * 12, second_edge, base + b * 12],
                );
                e.call(NI_POINT3_CROSS, &args![first_edge, cross, second_edge]);
                e.call(NI_POINT3_UNIT_VECTOR, &args![cross]);
                for corner in [a, b, c] {
                    let in_range = bounds.is_null()
                        || fn_0053da60(e, this, bounds, Ptr::new(base + corner * 12));
                    if in_range {
                        let normals = e.get(data, LoadedLandData::ppNormals);
                        let normal = element(e, normals, quadrant) + corner * 12;
                        e.call(NI_POINT3_ADD_IN_PLACE, &args![normal, cross]);
                    }
                }
            });
        }
        let normals = e.get(data, LoadedLandData::ppNormals);
        let block = element(e, normals, quadrant);
        e.call(
            NI_POINT3_UNITIZE_VECTORS,
            &args![block, QUADRANT_VERTICES, 0xcu32],
        );
    }
}

// Translated from 0053e210 (decompiled, FalloutNV.exe 1.4.0.525)
/// The "normal set" flag of the vertex nearest to the position `position`
/// (two floats): [`tes_object_land_get_coord_data`] (without recursion)
/// then [`fn_0053e260`] for its quadrant and nearest vertex; 0 when the
/// position is outside the land.
pub fn fn_0053e210(e: &mut Engine, this: Ptr<TESObjectLAND>, position: Ptr) -> u8 {
    e.with_stack(COORD_DATA_SIZE, |e, info| {
        fn_0053a510(e, info);
        if !tes_object_land_get_coord_data(e, this, info.cast(), position, false) {
            return 0;
        }
        let quadrant = e.get(info.cast::<CoordData>(), CoordData::Quadrant);
        let vertex = e.get(info.cast::<CoordData>(), CoordData::NearestVertex);
        fn_0053e260(e, this, quadrant, vertex)
    })
}

// Translated from 0053e260 (decompiled, FalloutNV.exe 1.4.0.525)
/// The "normal set" flag byte of vertex `vertex` of quadrant `quadrant`.
pub fn fn_0053e260(e: &mut Engine, this: Ptr<TESObjectLAND>, quadrant: i32, vertex: i32) -> u8 {
    let data = loaded_data(e, this);
    let set = e.get(data, LoadedLandData::ppNormalsSet);
    let block = element(e, set, quadrant as u32);
    e.mem.u8(block.wrapping_add(vertex as u32))
}

/// What `fn_0053e290` accumulates for one vertex: the position the
/// neighbours are asked about, the normal a neighbour gave, the running sum
/// of those normals (starting as the vertex's own) and their count.
struct NormalBlend {
    position: Ptr,
    candidate: Ptr,
    sum: Ptr,
    scale: f32,
}

/// Marks the normal of vertex `vertex` of quadrant `quadrant` as set.
fn mark_normal_set(e: &mut Engine, this: Ptr<TESObjectLAND>, quadrant: i32, vertex: i32) {
    let data = loaded_data(e, this);
    let set = e.get(data, LoadedLandData::ppNormalsSet);
    let block = element(e, set, quadrant as u32);
    e.mem.set_u8(block.wrapping_add(vertex as u32), 1);
}

/// One neighbouring land of `fn_0053e290`: asks `land` for the position and
/// normal at the vertex ([`fn_0053d2e0`]; the default normal (0, 0, 1) when
/// it has no land or does not know the position); when the land's own
/// normal there is set ([`fn_0053e210`]) it is the result and the function
/// is done (true), else the normal is added to the sum.
fn blend_land(
    e: &mut Engine,
    this: Ptr<TESObjectLAND>,
    land: u32,
    blend: &mut NormalBlend,
    quadrant: i32,
    vertex: i32,
    out: Ptr,
) -> bool {
    e.with_stack(12, |e, normal| {
        e.call(NI_POINT3_DEFAULT_CONSTRUCT, &args![normal]);
        let known =
            land != 0 && fn_0053d2e0(e, Ptr::new(land), blend.position, blend.candidate, normal);
        if !known {
            write_point3(e, blend.candidate, 0.0, 0.0, 1.0);
        }
    });
    if land != 0 && fn_0053e210(e, Ptr::new(land), blend.position) != 0 {
        copy_words(e, blend.candidate.addr(), out.addr(), 3);
        mark_normal_set(e, this, quadrant, vertex);
        return true;
    }
    e.call(NI_POINT3_ADD_IN_PLACE, &args![blend.sum, blend.candidate]);
    blend.scale = sum_f32(blend.scale, 1.0);
    false
}

/// One neighbouring quadrant of the same land: the normal of vertex
/// `neighbour_vertex` of quadrant `neighbour_quadrant`; when its "normal
/// set" flag is set it is the result (true), else it is added to the sum.
#[allow(clippy::too_many_arguments)]
fn blend_quadrant(
    e: &mut Engine,
    this: Ptr<TESObjectLAND>,
    blend: &mut NormalBlend,
    quadrant: i32,
    vertex: i32,
    out: Ptr,
    neighbour_quadrant: i32,
    neighbour_vertex: i32,
) -> bool {
    let data = loaded_data(e, this);
    let normals = e.get(data, LoadedLandData::ppNormals);
    let block = element(e, normals, neighbour_quadrant as u32);
    let at = block.wrapping_add((neighbour_vertex as u32).wrapping_mul(12));
    copy_words(e, at, blend.candidate.addr(), 3);
    if fn_0053e260(e, this, neighbour_quadrant, neighbour_vertex) != 0 {
        copy_words(e, blend.candidate.addr(), out.addr(), 3);
        mark_normal_set(e, this, quadrant, vertex);
        return true;
    }
    e.call(NI_POINT3_ADD_IN_PLACE, &args![blend.sum, blend.candidate]);
    blend.scale = sum_f32(blend.scale, 1.0);
    false
}

// Translated from 0053e290 (decompiled, FalloutNV.exe 1.4.0.525)
/// Computes the normal of vertex `vertex` of quadrant `quadrant` on the
/// border of the land, into `out`, unless its "normal set" flag is already
/// set or `lands` (the eight neighbours of [`fn_0053d330`]) is null. The
/// normal starts as the vertex's own; every neighbour that shares the
/// vertex adds its normal: the neighbouring land across a quadrant edge
/// that lies on the land's border (`lands[6]` above quadrants 0 and 1,
/// `lands[1]` below quadrants 2 and 3, `lands[3]` left of the left column,
/// `lands[4]` right of the right column, the diagonal lands at the
/// corners), the neighbouring quadrant of the same land otherwise, and the
/// diagonal quadrant at the quadrant corners. A neighbour whose own normal
/// there is already set ends the search with that normal; if none does,
/// the sum is divided by the count (`fn_0053d280`). The flag is set at the
/// end.
pub fn fn_0053e290(
    e: &mut Engine,
    this: Ptr<TESObjectLAND>,
    quadrant: i32,
    vertex: i32,
    out: Ptr,
    lands: Ptr,
) {
    let data = loaded_data(e, this);
    let set = e.get(data, LoadedLandData::ppNormalsSet);
    let already = e
        .mem
        .u8(element(e, set, quadrant as u32).wrapping_add(vertex as u32));
    if already != 0 || lands.is_null() {
        return;
    }
    e.with_stack(12 * 5, |e, scratch| {
        let slot = |index: u32| scratch.addr() + 12 * index;
        let centre = slot(0);
        e.call(NI_POINT3_DEFAULT_CONSTRUCT, &args![centre]);
        let words = cell_centre_vector(e, this);
        write_words(e, centre, &words);
        let vertices = e.get(data, LoadedLandData::ppVertices);
        let own_vertex = element(e, vertices, quadrant as u32) + (vertex as u32).wrapping_mul(12);
        let mut blend = NormalBlend {
            position: Ptr::new(slot(1)),
            candidate: Ptr::new(slot(2)),
            sum: Ptr::new(slot(3)),
            scale: 1.0,
        };
        e.call(NI_POINT3_ADD, &args![own_vertex, blend.position, centre]);
        e.call(NI_POINT3_DEFAULT_CONSTRUCT, &args![blend.candidate]);
        e.call(NI_POINT3_DEFAULT_CONSTRUCT, &args![blend.sum]);
        let normals = e.get(data, LoadedLandData::ppNormals);
        let own_normal = element(e, normals, quadrant as u32) + (vertex as u32).wrapping_mul(12);
        copy_words(e, own_normal, blend.sum.addr(), 3);
        let land = |e: &Engine, index: u32| e.mem.u32(lands.addr() + 4 * index);
        let (mut top, mut bottom, mut left, mut right) = (false, false, false, false);
        if quadrant < 2 && vertex < 0x11 {
            top = true;
            let neighbour = land(e, 6);
            if blend_land(e, this, neighbour, &mut blend, quadrant, vertex, out) {
                return;
            }
        } else if quadrant >= 2 && vertex >= 0x110 {
            bottom = true;
            let neighbour = land(e, 1);
            if blend_land(e, this, neighbour, &mut blend, quadrant, vertex, out) {
                return;
            }
        }
        if quadrant % 2 == 0 && vertex % 0x11 == 0 {
            left = true;
            let neighbour = land(e, 3);
            if blend_land(e, this, neighbour, &mut blend, quadrant, vertex, out) {
                return;
            }
        } else if (quadrant + 1) % 2 == 0 && (vertex + 1) % 0x11 == 0 {
            right = true;
            let neighbour = land(e, 4);
            if blend_land(e, this, neighbour, &mut blend, quadrant, vertex, out) {
                return;
            }
        }
        let corner = if left && top {
            Some(5)
        } else if right && top {
            Some(7)
        } else if left && bottom {
            Some(0)
        } else if right && bottom {
            Some(2)
        } else {
            None
        };
        if let Some(index) = corner {
            let neighbour = land(e, index);
            if blend_land(e, this, neighbour, &mut blend, quadrant, vertex, out) {
                return;
            }
        }
        let (mut up, mut down, mut left_quadrant, mut right_quadrant) =
            (false, false, false, false);
        if vertex < 0x11 && !top {
            up = true;
            if blend_quadrant(
                e,
                this,
                &mut blend,
                quadrant,
                vertex,
                out,
                quadrant - 2,
                vertex + 0x110,
            ) {
                return;
            }
        } else if vertex >= 0x110 && !bottom {
            down = true;
            if blend_quadrant(
                e,
                this,
                &mut blend,
                quadrant,
                vertex,
                out,
                quadrant + 2,
                vertex - 0x110,
            ) {
                return;
            }
        }
        if vertex % 0x11 == 0 && !left {
            left_quadrant = true;
            if blend_quadrant(
                e,
                this,
                &mut blend,
                quadrant,
                vertex,
                out,
                quadrant - 1,
                vertex + 0x10,
            ) {
                return;
            }
        } else if (vertex + 1) % 0x11 == 0 && !right {
            right_quadrant = true;
            if blend_quadrant(
                e,
                this,
                &mut blend,
                quadrant,
                vertex,
                out,
                quadrant + 1,
                vertex - 0x10,
            ) {
                return;
            }
        }
        let diagonal = if left_quadrant && up {
            Some((quadrant - 3, 0x120))
        } else if left_quadrant && down {
            Some((quadrant + 1, 0x10))
        } else if right_quadrant && up {
            Some((quadrant - 1, 0x110))
        } else if right_quadrant && down {
            Some((quadrant + 3, 0))
        } else {
            None
        };
        if let Some((neighbour_quadrant, neighbour_vertex)) = diagonal {
            if blend_quadrant(
                e,
                this,
                &mut blend,
                quadrant,
                vertex,
                out,
                neighbour_quadrant,
                neighbour_vertex,
            ) {
                return;
            }
        }
        let averaged = Ptr::new(slot(4));
        fn_0053d280(e, blend.sum, averaged, blend.scale);
        copy_words(e, averaged.addr(), out.addr(), 3);
        mark_normal_set(e, this, quadrant, vertex);
    });
}

// ---- Fourth session: height, colour and border of a position, blending, the
// ---- loaded data and the small classes the unit ends with -----------------

/// Runs `body` with a `CoordData` record on the stack after its constructor
/// (`0053a510`), as the callers of `GetCoordData` keep it.
fn with_coord_data<R>(e: &mut Engine, body: impl FnOnce(&mut Engine, Ptr<CoordData>) -> R) -> R {
    e.with_stack(COORD_DATA_SIZE, |e, info| {
        fn_0053a510(e, info);
        body(e, info.cast())
    })
}

// Translated from 0053f0e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The mesh node of the quadrant that holds the world position `position`
/// (two floats): [`tes_object_land_get_coord_data`] (without recursion),
/// then [`fn_0053f120`]. Null when the position is outside the land.
pub fn fn_0053f0e0(e: &mut Engine, this: Ptr<TESObjectLAND>, position: Ptr) -> Ptr {
    with_coord_data(e, |e, info| {
        if !tes_object_land_get_coord_data(e, this, info, position, false) {
            return Ptr::NULL;
        }
        fn_0053f120(e, this, info)
    })
}

// Translated from 0053f120 (decompiled, FalloutNV.exe 1.4.0.525)
/// The first geometry of the mesh node of the quadrant `info` names
/// (`0045bc00(node, 0)`); null without loaded data, mesh array or node.
pub fn fn_0053f120(e: &mut Engine, this: Ptr<TESObjectLAND>, info: Ptr<CoordData>) -> Ptr {
    let data = loaded_data(e, this);
    if data.is_null() {
        return Ptr::NULL;
    }
    let meshes = e.get(data, LoadedLandData::ppMesh);
    if meshes.is_null() {
        return Ptr::NULL;
    }
    let quadrant = e.get(info, CoordData::Quadrant) as u32;
    let node = element(e, meshes, quadrant);
    if node == 0 {
        return Ptr::NULL;
    }
    e.call(NODE_FIRST_GEOMETRY, &args![node, 0u32]).ptr()
}

// Translated from 0053f180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectLAND::GetLandHeight` (Xbox PDB): stores the height of the land
/// at the world position `position` (two floats) in `out`. Returns false,
/// with the default height ([`fn_0053a550`]) stored, when the position is
/// outside the land; otherwise the result of [`fn_0053f1e0`].
pub fn tes_object_land_get_land_height(
    e: &mut Engine,
    this: Ptr<TESObjectLAND>,
    position: Ptr,
    out: Ptr,
) -> bool {
    with_coord_data(e, |e, info| {
        if !tes_object_land_get_coord_data(e, this, info, position, false) {
            let height = fn_0053a550(e, this);
            e.mem.set_f32(out.addr(), height);
            return false;
        }
        fn_0053f1e0(e, this, info, out)
    })
}

// Translated from 0053f1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The height inside the triangle `info` names: the heights of its three
/// vertices (`TriangleVertex2`, `TriangleVertex0`, `TriangleVertex1`, read
/// with [`fn_00534240`]) are `a`, `b` and `c`, `x` and `y` are the offsets
/// in the 128-unit block, and the plane through the three is evaluated:
/// for an even block, `(a - b) / 128 * x + b + (c - a) / 128 * y` (x and y
/// swapped for the second triangle); for an odd block, `(b - a) / 128 * x +
/// a + (c - a) / 128 * y` for the first triangle and `a - ((128 - x) * (a -
/// c) / 128 + (128 - y) * (a - b) / 128)` for the second. Always returns
/// true.
pub fn fn_0053f1e0(
    e: &mut Engine,
    this: Ptr<TESObjectLAND>,
    info: Ptr<CoordData>,
    out: Ptr,
) -> bool {
    // Three `NiPoint3` locals: `a` at +0, `b` at +0x0c, `c` at +0x18 (the
    // code constructs them in the order of the stack: a, b, c).
    e.with_stack(0x24, |e, frame| {
        let first = frame;
        let second = frame.byte_add(0x0c);
        let third = frame.byte_add(0x18);
        for vector in [first, second, third] {
            e.call(NI_POINT3_DEFAULT_CONSTRUCT, &args![vector]);
        }
        let quadrant = e.get(info, CoordData::Quadrant) as u32;
        let vertex_a = e.get(info, CoordData::TriangleVertex2);
        fn_00534240(e, this, quadrant, vertex_a, first);
        let vertex_b = e.get(info, CoordData::TriangleVertex0);
        fn_00534240(e, this, quadrant, vertex_b, second);
        let vertex_c = e.get(info, CoordData::TriangleVertex1);
        fn_00534240(e, this, quadrant, vertex_c, third);
        // 0053f24f: the heights are copied through the x87 stack (`FLD dword;
        // FSTP dword`); every step below runs on the stack in exe order,
        // rounded to the control word's precision, and only the slopes and
        // the result are stored as floats.
        let a = f64::from(x87_quiet(e.mem.f32(first.addr() + 8)));
        let b = f64::from(x87_quiet(e.mem.f32(second.addr() + 8)));
        let c = f64::from(x87_quiet(e.mem.f32(third.addr() + 8)));
        let block = e.global::<f64>(F64_128);
        let x_bits = e.get(info, CoordData::BlockOffsetX).to_bits();
        let y_bits = e.get(info, CoordData::BlockOffsetY).to_bits();
        // The offsets as `FMUL dword` operands.
        let (x, y) = (x87_m32(x_bits), x87_m32(y_bits));
        let odd = e.get(info, CoordData::OddBlock) != 0;
        let second_triangle = e.get(info, CoordData::SecondTriangle) != 0;
        // `FLD to; FSUB from; FDIV qword 128; FSTP dword`, loaded again.
        let slope = |from: f64, to: f64| {
            f64::from(f32::from_bits(x87_store32(x87_div(
                x87_sub(to, from),
                block,
            ))))
        };
        let height = if !odd {
            let s = slope(b, a);
            let t = slope(a, c);
            if !second_triangle {
                x87_add(x87_add(x87_mul(s, x), b), x87_mul(t, y))
            } else {
                x87_add(x87_add(x87_mul(s, y), b), x87_mul(t, x))
            }
        } else if !second_triangle {
            let s = slope(a, b);
            let t = slope(a, c);
            x87_add(x87_add(x87_mul(s, x), a), x87_mul(t, y))
        } else {
            let s = slope(c, a);
            let t = slope(b, a);
            // 0053f31e: `FLD dword x; FSUBR qword 128; FMUL dword s`, the
            // same for y and t, `FADDP; FSUBR dword a`.
            let across_x = x87_mul(x87_sub(block, x87_load32(x_bits)), s);
            let across_y = x87_mul(x87_sub(block, x87_load32(y_bits)), t);
            x87_sub(a, x87_add(across_x, across_y))
        };
        e.mem.set_u32(out.addr(), x87_store32(height));
        true
    })
}

// Translated from 0053f390 (decompiled, FalloutNV.exe 1.4.0.525)
/// The lowest and highest vertex height of quadrant `quadrant` (below 16)
/// stored in `out` as an `NiPoint2` `(lowest, highest)`, read with
/// [`fn_00534240`] over its 0x121 vertices. Without a valid quadrant the
/// extents stay `(FLT_MAX, -FLT_MAX)`. Returns `out`.
pub fn fn_0053f390(e: &mut Engine, this: Ptr<TESObjectLAND>, out: Ptr, quadrant: i32) -> Ptr {
    let largest = e.global::<f32>(LARGEST_F32);
    let lowest = e.global::<f32>(LOWEST_F32);
    let extents = ni_point2(e, largest, lowest);
    let mut low = f32::from_bits(extents[0]);
    let mut high = f32::from_bits(extents[1]);
    e.with_stack(12, |e, vertex| {
        e.call(NI_POINT3_DEFAULT_CONSTRUCT, &args![vertex]);
        if quadrant < 0x10 {
            for index in 0..QUADRANT_VERTICES as i32 {
                fn_00534240(e, this, quadrant as u32, index, vertex);
                let height = e.mem.f32(vertex.addr() + 8);
                if height < low {
                    low = height;
                }
                if high < height {
                    high = height;
                }
            }
        }
    });
    e.mem.set_f32(out.addr(), low);
    e.mem.set_f32(out.addr() + 4, high);
    out
}

// Translated from 0053f440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectLAND::GetMinMaxLandHeight` (Xbox PDB): stores the height
/// extents of the land in `out` (an `NiPoint2`, lowest then highest) and
/// returns it. Without loaded data they are `(FLT_MAX, -FLT_MAX)`. When the
/// loaded data still holds the initial extents, they are worked out first:
/// the default height twice for a land without height data, else the
/// extents of the four quadrants ([`fn_0053f390`]) folded in.
pub fn tes_object_land_get_min_max_land_height(
    e: &mut Engine,
    this: Ptr<TESObjectLAND>,
    out: Ptr,
) -> Ptr {
    let data = loaded_data(e, this);
    if data.is_null() {
        let largest = e.global::<f32>(LARGEST_F32);
        let lowest = e.global::<f32>(LOWEST_F32);
        e.call(NI_POINT2_CONSTRUCT, &args![out, largest, lowest]);
        return out;
    }
    let largest = e.global::<f64>(F64_LARGEST);
    let lowest = e.global::<f64>(F64_LOWEST);
    let low = e.get(data, LoadedLandData::HeightExtentsMin) as f64;
    let high = e.get(data, LoadedLandData::HeightExtentsMax) as f64;
    if low == largest || high == lowest {
        if e.get(this, TESObjectLAND::Data) & FLAG_HEIGHTS != 0 {
            e.with_stack(8, |e, extents| {
                for quadrant in 0..QUADRANTS as i32 {
                    fn_0053f390(e, this, extents, quadrant);
                    let data = loaded_data(e, this);
                    let new_low = e.mem.f32(extents.addr());
                    if new_low < e.get(data, LoadedLandData::HeightExtentsMin) {
                        e.set(data, LoadedLandData::HeightExtentsMin, new_low);
                    }
                    let data = loaded_data(e, this);
                    let new_high = e.mem.f32(extents.addr() + 4);
                    if e.get(data, LoadedLandData::HeightExtentsMax) < new_high {
                        e.set(data, LoadedLandData::HeightExtentsMax, new_high);
                    }
                }
            });
        } else {
            let first = fn_0053a550(e, this);
            let second = fn_0053a550(e, this);
            let words = ni_point2(e, second, first);
            let data = loaded_data(e, this);
            write_words(
                e,
                data.addr() + LoadedLandData::HeightExtentsMin.off,
                &words,
            );
        }
    }
    let data = loaded_data(e, this);
    copy_words(
        e,
        data.addr() + LoadedLandData::HeightExtentsMin.off,
        out.addr(),
        2,
    );
    out
}

// Translated from 0053f570 (decompiled, FalloutNV.exe 1.4.0.525)
/// The vertex colour at the world position `position` (two floats), stored
/// in `out` (an `NiColorA`): the colours of the three vertices of the
/// triangle [`tes_object_land_get_coord_data`] finds, read with
/// [`fn_005345c0`] and blended with the position's fractions `s` and `t`
/// inside the 128-unit block (the remainder of the position by 128, over
/// 128). Returns false when the position is outside the land. With `c1`,
/// `c2` and `c3` the colours of `TriangleVertex2`, `TriangleVertex0` and
/// `TriangleVertex1`, the result is
/// `(1 - t) * ((1 - s) * c2 + s * c1) + t * c3` for an even block's first
/// triangle, `t * ((1 - s) * c1 + s * c3) + (1 - t) * c2` for its second,
/// `(1 - t) * ((1 - s) * c1 + s * c2) + t * c3` for an odd block's first and
/// `t * ((1 - s) * c3 + s * c1) + (1 - t) * c2` for its second.
pub fn fn_0053f570(e: &mut Engine, this: Ptr<TESObjectLAND>, position: Ptr, out: Ptr) -> bool {
    with_coord_data(e, |e, info| {
        if !tes_object_land_get_coord_data(e, this, info, position, false) {
            return false;
        }
        // Three `NiColorA` locals, then the six temporaries of the blend.
        e.with_stack(0x90, |e, frame| {
            let colors = [
                frame.byte_add(0x00),
                frame.byte_add(0x10),
                frame.byte_add(0x20),
            ];
            for color in colors {
                e.call(
                    NI_COLOR_CONSTRUCT,
                    &args![color, 0.0f32, 0.0f32, 0.0f32, 0.0f32],
                );
            }
            let block = e.global::<f32>(F32_128);
            let divisor = e.global::<f64>(F64_128);
            let x = e.mem.f32(position.addr());
            let s = (e.call(FLOAT_MODULO, &args![x, block]).f32() as f64 / divisor) as f32;
            let y = e.mem.f32(position.addr() + 4);
            let t = (e.call(FLOAT_MODULO, &args![y, block]).f32() as f64 / divisor) as f32;
            let quadrant = e.get(info, CoordData::Quadrant) as u32;
            for (color, field) in colors.into_iter().zip([
                CoordData::TriangleVertex2,
                CoordData::TriangleVertex0,
                CoordData::TriangleVertex1,
            ]) {
                let vertex = e.get(info, field);
                fn_005345c0(e, this, quadrant, vertex, color);
            }
            let [c1, c2, c3] = colors;
            let odd = e.get(info, CoordData::OddBlock) != 0;
            let second_triangle = e.get(info, CoordData::SecondTriangle) != 0;
            let one_minus_s = (1.0f64 - s as f64) as f32;
            let one_minus_t = (1.0f64 - t as f64) as f32;
            // The blend `E * scale + A` where `E = (C * (1 - s) + B * s)`:
            // `first` is `(scalar, colour)` of the term added last, `b` and
            // `c` the colours scaled by `s` and `1 - s`, `scale` the factor
            // of their sum.
            let (first, b, c, scale) = match (odd, second_triangle) {
                (false, false) => ((t, c3), c1, c2, one_minus_t),
                (false, true) => ((one_minus_t, c2), c3, c1, t),
                (true, false) => ((t, c3), c2, c1, one_minus_t),
                (true, true) => ((one_minus_t, c2), c1, c3, t),
            };
            let temporaries = frame.byte_add(0x30);
            let temporary = |index: u32| temporaries.byte_add(0x10 * index);
            let a = e
                .call(COLOR_SCALED, &args![temporary(0), first.0, first.1])
                .u32();
            let b = e.call(COLOR_SCALED, &args![temporary(1), s, b]).u32();
            let c = e
                .call(COLOR_SCALED, &args![temporary(2), one_minus_s, c])
                .u32();
            let sum = e.call(COLOR_ADD, &args![c, temporary(3), b]).u32();
            let scaled = e
                .call(COLOR_TIMES_SCALAR, &args![sum, temporary(4), scale])
                .u32();
            let result = e.call(COLOR_ADD, &args![scaled, temporary(5), a]).u32();
            copy_words(e, result, out.addr(), 4);
            true
        })
    })
}

// Translated from 0053fa10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The colour of the vertex nearest to the position `info` describes
/// (`NearestVertex` of its quadrant), read with [`fn_005345c0`] into `out`.
/// Always returns true.
pub fn fn_0053fa10(
    e: &mut Engine,
    this: Ptr<TESObjectLAND>,
    info: Ptr<CoordData>,
    out: Ptr,
) -> bool {
    let quadrant = e.get(info, CoordData::Quadrant) as u32;
    let vertex = e.get(info, CoordData::NearestVertex);
    fn_005345c0(e, this, quadrant, vertex, out);
    true
}

// Translated from 0053fa40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Replaces the border lines of the land (the outline of the cell drawn in
/// the editor): with vertex arrays and a 3D node for the cell, the old
/// lines are detached (slot `0xe8` of the cell's node) and released; when
/// `build` is set it makes 128 vertices around the 33 x 33 grid (the
/// vertex index runs along the left column, the top row, the right column
/// and the bottom row; its parity picks one of two colours made of INI
/// settings), each at the land height there plus 5 and relative to the
/// cell's corner, builds the `NiLines` from them, moves it to the corner,
/// gives it a texturing property and a no-lighting shader property, and
/// attaches it to the cell's node (slot `0xdc`) and updates it. (The
/// unwinding frame is not translated.)
pub fn fn_0053fa40(e: &mut Engine, this: Ptr<TESObjectLAND>, build: bool) {
    if !has_vertex_arrays(e, this) {
        return;
    }
    let cell = e.get(this, TESObjectLAND::pParentCell);
    let node = e.call(CELL_GET_3D, &args![cell]).u32();
    if node == 0 {
        return;
    }
    let data = loaded_data(e, this);
    let slot = data.byte_add(LoadedLandData::spBorder.off);
    if e.call(SMART_POINTER_GET, &args![slot]).u32() != 0 {
        let border = e.call(SMART_POINTER_GET, &args![slot]).u32();
        e.vcall(node, BORDER_OWNER_SLOT, &args![border]);
        e.call(TEXTURING_PROPERTY_SET, &args![slot, 0u32]);
    }
    if !build {
        return;
    }
    let positions = allocate_vector(e, 0x600, 12, BORDER_VERTICES, NI_POINT3_DEFAULT_CONSTRUCT);
    let colors = allocate_vector(e, 0x800, 16, BORDER_VERTICES, NI_COLOR_DEFAULT_CONSTRUCT);
    let flags = e.call(ALLOCATE, &args![0x80u32]).u32();
    // Locals: the cell's corner, the sum, the vector built for a vertex, the
    // height and the vertex relative to the corner, the update data.
    e.with_stack(0x80, |e, frame| {
        let origin = frame;
        let sum = frame.byte_add(0x10);
        let step = frame.byte_add(0x20);
        let height = frame.byte_add(0x30);
        let relative = frame.byte_add(0x40);
        let update_data = frame.byte_add(0x50);
        let corner_y = fn_00534080(e, this) as f32;
        let corner_x = fn_00534050(e, this) as f32;
        e.call(
            NI_POINT3_CONSTRUCT,
            &args![origin, corner_x, corner_y, 0.0f32],
        );
        let step_size = e.global::<f64>(F64_128);
        let height_offset = e.global::<f64>(F64_FIVE);
        let color_scale = e.global::<f64>(F64_255);
        for column in 0..0x21u32 {
            for row in 0..0x21u32 {
                if !(column == 0 || row == 0 || column == 0x20 || row == 0x20) {
                    continue;
                }
                let index = if row == 0 {
                    column
                } else if column == 0x20 {
                    row + 0x20
                } else if row == 0x20 {
                    0x60 - column
                } else {
                    0x80 - row
                };
                let (red, green, blue) = if index & 1 == 0 {
                    (BORDER_EVEN_RED, BORDER_EVEN_GREEN, BORDER_EVEN_BLUE)
                } else {
                    (BORDER_ODD_RED, BORDER_ODD_GREEN, BORDER_ODD_BLUE)
                };
                // The blue setting is read first, the red one last.
                let mut components = [0.0f32; 3];
                for (component, setting) in components.iter_mut().zip([blue, green, red]) {
                    let address = e.call(SETTING_GET_INT_ADDRESS, &args![setting]).u32();
                    *component = (e.mem.i32(address) as f64 / color_scale) as f32;
                }
                let color = ni_color(e, components[2], components[1], components[0], 1.0);
                write_words(e, colors + index * 16, &color);
                e.mem.set_u8(flags + index, 1);
                let step_x = (column as i32 as f64 * step_size) as f32;
                let step_y = (row as i32 as f64 * step_size) as f32;
                let offset = e
                    .call(NI_POINT3_CONSTRUCT, &args![step, step_x, step_y, 0.0f32])
                    .u32();
                e.call(NI_POINT3_ADD, &args![origin, sum, offset]);
                tes_object_land_get_land_height(e, this, sum, height);
                let lifted = (e.mem.f32(height.addr()) as f64 + height_offset) as f32;
                e.mem.set_f32(sum.addr() + 8, lifted);
                let point = e
                    .call(NI_POINT3_SUBTRACT, &args![sum, relative, origin])
                    .u32();
                copy_words(e, point, positions + index * 12, 3);
            }
        }
        let memory = e.call(NI_ALLOCATE_OBJECT, &args![0xc4u32]).u32();
        let lines = if memory == 0 {
            0
        } else {
            e.call(
                NI_LINES_CONSTRUCT,
                &args![
                    memory,
                    BORDER_VERTICES,
                    positions,
                    colors,
                    0u32,
                    0u32,
                    0u32,
                    flags
                ],
            )
            .u32()
        };
        e.call(TEXTURING_PROPERTY_SET, &args![slot, lines]);
        let border = e.call(SMART_POINTER_GET, &args![slot]).u32();
        e.call(NODE_SET_TRANSLATE, &args![border, origin]);
        let memory = e.call(NI_ALLOCATE_OBJECT, &args![0x30u32]).u32();
        let texturing = if memory == 0 {
            0
        } else {
            e.call(TEXTURING_PROPERTY_CONSTRUCT, &args![memory]).u32()
        };
        fn_00533f40(e, Ptr::new(texturing), Ptr::NULL);
        e.call(TEXTURING_PROPERTY_AFTER_MAP, &args![texturing, 0u32, 0u32]);
        let border = e.call(SMART_POINTER_GET, &args![slot]).u32();
        e.call(NODE_ATTACH_PROPERTY, &args![border, texturing]);
        let memory = e.call(NI_ALLOCATE_OBJECT, &args![0x80u32]).u32();
        let shader = if memory == 0 {
            0
        } else {
            e.call(NO_LIGHTING_PROPERTY_CONSTRUCT, &args![memory]).u32()
        };
        e.call(
            SHADER_PROPERTY_SET_WORD_0X58,
            &args![shader, BORDER_SHADER_WORD],
        );
        let border = e.call(SMART_POINTER_GET, &args![slot]).u32();
        e.call(NODE_ATTACH_PROPERTY, &args![border, shader]);
        let border = e.call(SMART_POINTER_GET, &args![slot]).u32();
        e.call(PREPARE_OBJECT, &args![border, 0u32, 0u32]);
        let border = e.call(SMART_POINTER_GET, &args![slot]).u32();
        e.vcall(node, NODE_ATTACH_BORDER_SLOT, &args![border, 1u32]);
        let border = e.call(SMART_POINTER_GET, &args![slot]).u32();
        e.call(NODE_UPDATE_PROPERTIES, &args![border]);
        e.call(
            UPDATE_DATA_CONSTRUCT,
            &args![update_data, 0.0f32, 0u32, 0u32],
        );
        let border = e.call(SMART_POINTER_GET, &args![slot]).u32();
        e.call(NODE_UPDATE, &args![border, update_data]);
    });
}

// Translated from 005400e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectLAND::InitDistantTextureBlending` (Xbox PDB): for each of the
/// four quadrant meshes whose shader property is of a type that takes
/// blending (8 to 12) and whose texture at index 9 is missing in one of its
/// two tables (`fn_00540540`, `fn_00540570`), asks the exterior's terrain
/// manager for the blend textures at the land's position (the cell centre,
/// `(cell + 0.5) * 4096`) and hands them to the property (slots `0xfc` and
/// `0x100`, with 9). When the terrain manager has an object for the
/// position, it stores in the property's block at +0x94 the fractions of
/// the cell position (less the object's two 16-bit values, plus the
/// quadrant's half-cell offset) over the object's word at +4, wrapped into
/// 0 to 1, and the two constant words. Does nothing without a terrain
/// manager or the meshes.
pub fn tes_object_land_init_distant_texture_blending(e: &mut Engine, this: Ptr<TESObjectLAND>) {
    let exterior = e.global::<u32>(EXTERIOR_LOADER);
    let terrain = if e.call(TES_GET_WORLD_SPACE, &args![exterior]).u32() != 0 {
        let world_space = e.call(TES_GET_WORLD_SPACE, &args![exterior]).u32();
        e.call(WORLD_SPACE_GET_TERRAIN_MANAGER, &args![world_space])
            .u32()
    } else {
        0
    };
    let data = loaded_data(e, this);
    let cell_x = e.get(data, LoadedLandData::iCellX);
    let cell_y = e.get(data, LoadedLandData::iCellY);
    let half = e.global::<f64>(F64_HALF);
    let cell_size = e.global::<f64>(F64_4096);
    // The position handed to the terrain manager: the default normal's
    // words with x and y replaced by the cell centre. (The code also copies
    // the two words at `011f4980` into a local offset that every quadrant
    // overwrites before it is read.)
    e.with_stack(12, |e, position| {
        copy_words(e, DEFAULT_NORMAL_WORDS, position.addr(), 3);
        e.mem
            .set_f32(position.addr(), ((cell_x as f64 + half) * cell_size) as f32);
        e.mem.set_f32(
            position.addr() + 4,
            ((cell_y as f64 + half) * cell_size) as f32,
        );
        for quadrant in 0..4u32 {
            let data = loaded_data(e, this);
            let meshes = e.get(data, LoadedLandData::ppMesh);
            if meshes.is_null() || terrain == 0 {
                continue;
            }
            let node = element(e, meshes, quadrant);
            let geometry = e.call(NODE_FIRST_GEOMETRY, &args![node, 0u32]).u32();
            let property = e.call(OBJECT_GET_PROPERTY, &args![geometry, 3u32]).u32();
            let takes_blending = property != 0 && {
                e.call(PROPERTY_GET_TYPE, &args![property]).i32() >= BLEND_TYPE_FIRST
                    && e.call(PROPERTY_GET_TYPE, &args![property]).i32() <= BLEND_TYPE_LAST
            };
            let property = if takes_blending { property } else { 0 };
            if property == 0 {
                continue;
            }
            if fn_00540540(e, Ptr::new(property), BLEND_TEXTURE_INDEX) != 0
                && fn_00540570(e, Ptr::new(property), BLEND_TEXTURE_INDEX) != 0
            {
                continue;
            }
            let first = e.call(TERRAIN_BLEND_FIRST, &args![terrain, position]).u32();
            let second = e
                .call(TERRAIN_BLEND_SECOND, &args![terrain, position])
                .u32();
            e.vcall(
                property,
                PROPERTY_SLOT_BLEND_FIRST,
                &args![BLEND_TEXTURE_INDEX as u32, first],
            );
            e.vcall(
                property,
                PROPERTY_SLOT_BLEND_SECOND,
                &args![BLEND_TEXTURE_INDEX as u32, second],
            );
            let mut block = [
                e.mem.u32(BLEND_BLOCK_WORDS),
                e.mem.u32(BLEND_BLOCK_WORDS + 4),
                e.mem.u32(BLEND_BLOCK_WORDS + 8),
                e.mem.u32(BLEND_BLOCK_WORDS + 12),
            ];
            let offset = match quadrant {
                0 => {
                    let words = ni_point2(e, 0.0, 0.0);
                    [f32::from_bits(words[0]), f32::from_bits(words[1])]
                }
                1 => {
                    let half = e.global::<f32>(F32_HALF);
                    let words = ni_point2(e, half, 0.0);
                    [f32::from_bits(words[0]), f32::from_bits(words[1])]
                }
                2 => {
                    let half = e.global::<f32>(F32_HALF);
                    let words = ni_point2(e, 0.0, half);
                    [f32::from_bits(words[0]), f32::from_bits(words[1])]
                }
                _ => {
                    let half = e.global::<f32>(F32_HALF);
                    let words = ni_point2(e, half, half);
                    [f32::from_bits(words[0]), f32::from_bits(words[1])]
                }
            };
            let object = e
                .call(TERRAIN_BLEND_OBJECT, &args![terrain, position])
                .u32();
            if object == 0 {
                continue;
            }
            let target = e.call(SMART_POINTER_GET, &args![object]).u32();
            let first_short = e.call(TERRAIN_OBJECT_SHORT_FIRST, &args![target]).u16() as i16;
            let target = e.call(SMART_POINTER_GET, &args![object]).u32();
            let second_short = e.call(TERRAIN_OBJECT_SHORT_SECOND, &args![target]).u16() as i16;
            let target = e.call(SMART_POINTER_GET, &args![object]).u32();
            e.call(DATA_HANDLER_WORD_AT_4, &args![target]);
            let data = loaded_data(e, this);
            let cell_x = e.get(data, LoadedLandData::iCellX);
            let cell_y = e.get(data, LoadedLandData::iCellY);
            let along_x = (cell_x as f64 - first_short as f64) as f32;
            let along_y = (cell_y as f64 - second_short as f64) as f32;
            // 0054040f: `FLD dword; FADD dword; FSTP qword`, then the word
            // (`FILD qword`, zero-extended) divides it (`FDIVR qword`), each
            // rounded to the control word's precision; `FSTP dword`.
            let sum_x = x87_add(f64::from(along_x), x87_m32(offset[0].to_bits()));
            let target = e.call(SMART_POINTER_GET, &args![object]).u32();
            let divisor_x = e.call(DATA_HANDLER_WORD_AT_4, &args![target]).u32();
            let quotient_x = x87_div(sum_x, f64::from(divisor_x));
            let mut fraction_x = f32::from_bits(x87_store32(quotient_x));
            let sum_y = x87_add(f64::from(along_y), x87_m32(offset[1].to_bits()));
            let target = e.call(SMART_POINTER_GET, &args![object]).u32();
            let divisor_y = e.call(DATA_HANDLER_WORD_AT_4, &args![target]).u32();
            let quotient_y = x87_div(sum_y, f64::from(divisor_y));
            let mut fraction_y = f32::from_bits(x87_store32(quotient_y));
            let one = e.global::<f64>(F64_ONE);
            let zero = e.global::<f64>(F64_ZERO);
            while fraction_x as f64 > one {
                fraction_x = (fraction_x as f64 - one) as f32;
            }
            while (fraction_x as f64) < zero {
                fraction_x = (fraction_x as f64 + one) as f32;
            }
            while fraction_y as f64 > one {
                fraction_y = (fraction_y as f64 - one) as f32;
            }
            while (fraction_y as f64) < zero {
                fraction_y = (fraction_y as f64 + one) as f32;
            }
            block[0] = fraction_x.to_bits();
            block[1] = fraction_y.to_bits();
            write_words(e, property + PROPERTY_BLEND_BLOCK_OFFSET, &block);
        }
    });
}

// Translated from 00540540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the smart pointer `index` of the table at +0xac of `this`
/// (`NiPointer` slots, four bytes each).
pub fn fn_00540540(e: &mut Engine, this: Ptr, index: i32) -> u32 {
    let table = e.mem.u32(this.addr() + 0xac);
    let slot = table.wrapping_add((index as u32).wrapping_mul(4));
    e.call(SMART_POINTER_GET, &args![slot]).u32()
}

// Translated from 00540570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the smart pointer `index` of the table at +0xb0 of `this`
/// (`NiPointer` slots, four bytes each).
pub fn fn_00540570(e: &mut Engine, this: Ptr, index: i32) -> u32 {
    let table = e.mem.u32(this.addr() + 0xb0);
    let slot = table.wrapping_add((index as u32).wrapping_mul(4));
    e.call(SMART_POINTER_GET, &args![slot]).u32()
}

// Translated from 005405a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectLAND::LoadedLandData::LoadedLandData` (Xbox PDB, 0xA4 bytes):
/// constructs the border slot (+0x14) and the height extents (+0x18),
/// the four grass maps (+0x54, `0x10` bytes each, hash size 0x25), the
/// rigid body slot (+0x94); clears the mesh, vertex, normal, colour and
/// normal-set pointers, empties the border (`NiPointer = 0`), sets the
/// extents to `(FLT_MAX, -FLT_MAX)`, the four default quad textures to the
/// default land texture, clears the texture arrays (+0x30, +0x40), the Havok
/// code pointer (+0x50) and the base height (+0xA0). Returns `this`. (The
/// unwinding frame is not translated.)
pub fn loaded_land_data_loaded_land_data(
    e: &mut Engine,
    this: Ptr<LoadedLandData>,
) -> Ptr<LoadedLandData> {
    e.call(
        SMART_POINTER_CONSTRUCT,
        &args![this.byte_add(LoadedLandData::spBorder.off), 0u32],
    );
    e.call(
        NI_POINT3_DEFAULT_CONSTRUCT,
        &args![this.byte_add(LoadedLandData::HeightExtentsMin.off)],
    );
    e.call(
        EH_VECTOR_CONSTRUCT,
        &args![
            this.byte_add(GRASS_MAP_OFFSET),
            0x10u32,
            QUADRANTS,
            GRASS_MAP_DEFAULT_CONSTRUCT,
            GRASS_MAP_DESTRUCT
        ],
    );
    e.call(
        SMART_POINTER_CONSTRUCT,
        &args![this.byte_add(LoadedLandData::spLandRB.off), 0u32],
    );
    e.set(this, LoadedLandData::ppMesh, Ptr::NULL);
    e.set(this, LoadedLandData::ppVertices, Ptr::NULL);
    e.set(this, LoadedLandData::ppNormals, Ptr::NULL);
    e.set(this, LoadedLandData::ppColorsA, Ptr::NULL);
    e.set(this, LoadedLandData::ppNormalsSet, Ptr::NULL);
    e.call(
        TEXTURING_PROPERTY_SET,
        &args![this.byte_add(LoadedLandData::spBorder.off), 0u32],
    );
    let largest = e.global::<f32>(LARGEST_F32);
    let lowest = e.global::<f32>(LOWEST_F32);
    let extents = ni_point2(e, largest, lowest);
    write_words(
        e,
        this.addr() + LoadedLandData::HeightExtentsMin.off,
        &extents,
    );
    let default_texture = e.global::<u32>(DEFAULT_LAND_TEXTURE);
    for quadrant in 0..QUADRANTS {
        e.mem.set_u32(
            this.addr() + LoadedLandData::pDefQuadTexture.off + 4 * quadrant,
            default_texture,
        );
    }
    e.call(MEMORY_SET, &args![this.byte_add(0x30), 0u32, 0x10u32]);
    e.call(MEMORY_SET, &args![this.byte_add(0x40), 0u32, 0x10u32]);
    e.set(this, LoadedLandData::pMoppCode, Ptr::NULL);
    e.set(this, LoadedLandData::fBaseHeight, 0.0);
    this
}

// Translated from 00540700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Default constructor of a grass map: [`fn_00540780`] with the hash size
/// `0x25`. Returns what that does, `this`.
pub fn fn_00540700(e: &mut Engine, this: Ptr) -> Ptr {
    fn_00540780(e, this, GRASS_MAP_HASH_SIZE)
}

// Translated from 00540720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the vertex lock the readers keep (`{base, stride, packed}`:
/// two words and a byte, all zero). Returns `this`.
pub fn fn_00540720(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0);
    e.mem.set_u32(this.addr() + 4, 0);
    e.mem.set_u8(this.addr() + 8, 0);
    this
}

// Translated from 00540750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sizes the Havok byte array `this` (`{data, size, capacity}`) to
/// `new_size`: asks the router for the allocator (`004a4840`, whose stack
/// argument is `this`) and runs [`fn_00540860`] with it and the size.
pub fn fn_00540750(e: &mut Engine, this: Ptr, new_size: i32) {
    let allocator = e.with_stack(4, |e, zero| {
        e.mem.set_u8(zero.addr(), 0);
        e.call(HK_ARRAY_ALLOCATOR, &args![zero, this]).u32()
    });
    fn_00540860(e, this, allocator, new_size);
}

// Translated from 00540780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerMap<unsigned int, TESGrassAreaParam*>` constructor (by the
/// RTTI of the vtable it stores): the base constructor ([`fn_00540940`]) with
/// the hash size, then the map's vtable. Returns `this`.
pub fn fn_00540780(e: &mut Engine, this: Ptr, hash_size: u32) -> Ptr {
    fn_00540940(e, this, hash_size);
    e.mem.set_u32(this.addr(), GRASS_MAP_VTABLE);
    this
}

// Translated from 005407b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the low byte of `value` at `this` (a setter of a byte field).
pub fn fn_005407b0(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr(), value);
}

// Translated from 005407d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerMap<unsigned_int_TESGrassAreaParam_P_P>::
/// _scalar_deleting_destructor_` (Xbox PDB): [`fn_005409b0`], then the block
/// is freed when bit 0 of `flags` is set. Returns `this`.
pub fn ni_t_pointer_map_grass_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_005409b0(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

// Translated from 00540800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray<NiAGDDataBlock*, ...>` destructor body (by the RTTI of the
/// vtable): stores the array's vtable and releases the table at +4
/// (`004ede70`).
pub fn fn_00540800(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), BLOCK_ARRAY_VTABLE);
    let table = e.mem.u32(this.addr() + 4);
    e.call(RELEASE_TABLE, &args![table]);
}

// Translated from 00540830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPrimitiveArray<NiAGDDataBlock*, ...>` constructor (by the RTTI of
/// the vtable it stores): the array constructor ([`fn_00540b00`]) with the
/// maximum size and the grow-by step, then the derived vtable. Returns
/// `this`.
pub fn fn_00540830(e: &mut Engine, this: Ptr, max_size: u16, grow_by: u16) -> Ptr {
    fn_00540b00(e, this, max_size, grow_by);
    e.mem.set_u32(this.addr(), BLOCK_PRIMITIVE_ARRAY_VTABLE);
    this
}

// Translated from 00540860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the size of the Havok byte array `this` (`{data, size, capacity}`)
/// to `new_size` with `allocator`: reserves ([`fn_00540bc0`]), then runs the
/// two range functions of the construct and destruct steps (`0040fbe0`, which
/// does nothing for bytes) on the shrunk and the grown range, and stores
/// the size.
pub fn fn_00540860(e: &mut Engine, this: Ptr, allocator: u32, new_size: i32) {
    fn_00540bc0(e, this, allocator, new_size);
    let data = e.mem.u32(this.addr());
    let size = e.mem.i32(this.addr() + 4);
    e.call(
        BYTE_RANGE_NOTHING,
        &args![
            data.wrapping_add(new_size as u32),
            size.wrapping_sub(new_size),
            0u32
        ],
    );
    let data = e.mem.u32(this.addr());
    let size = e.mem.i32(this.addr() + 4);
    e.call(
        BYTE_RANGE_NOTHING,
        &args![
            data.wrapping_add(size as u32),
            new_size.wrapping_sub(size),
            0u32
        ],
    );
    e.mem.set_i32(this.addr() + 4, new_size);
}

// Translated from 005408e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the Havok byte array (the decompiler names it after a
/// library destructor): releases the array ([`fn_00540ad0`]), then runs the
/// base destructor (`00483710`, which does nothing). (The unwinding frame is
/// not translated.)
pub fn fn_005408e0(e: &mut Engine, this: Ptr) {
    fn_00540ad0(e, this);
    e.call(FORM_BASE_DESTRUCT, &args![this]);
}

// Translated from 00540940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<NiTPointerAllocator<unsigned int>, unsigned int,
/// TESGrassAreaParam*>` constructor (by the RTTI of the vtable it stores):
/// the base vtable, the hash size at +4, no entries (+0xc), a zeroed bucket
/// table of `hash_size` pointers (+8, from `NiAlloc`). Returns `this`.
pub fn fn_00540940(e: &mut Engine, this: Ptr, hash_size: u32) -> Ptr {
    e.mem.set_u32(this.addr(), MAP_BASE_VTABLE);
    e.mem.set_u32(this.addr() + 4, hash_size);
    e.mem.set_u32(this.addr() + 0xc, 0);
    let size = hash_size << 2;
    let table = e.call(NI_ALLOCATE, &args![size]).u32();
    e.mem.set_u32(this.addr() + 8, table);
    let table = e.mem.u32(this.addr() + 8);
    let size = e.mem.u32(this.addr() + 4) << 2;
    e.call(MEMORY_SET, &args![table, 0u32, size]);
    this
}

// Translated from 005409b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerMap<unsigned int, TESGrassAreaParam*>` destructor body (by the
/// RTTI of the vtable): stores the map's vtable, removes all entries
/// (`00438af0`), then runs [`fn_00540a10`]. (The unwinding frame is not
/// translated.)
pub fn fn_005409b0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), GRASS_MAP_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    fn_00540a10(e, this);
}

// Translated from 00540a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase` destructor body: stores the base vtable, removes all entries
/// (`00438af0`) and frees the bucket table at +8 (`00aa10f0`).
pub fn fn_00540a10(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), MAP_BASE_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    let table = e.mem.u32(this.addr() + 8);
    e.call(RELEASE_BLOCK, &args![table]);
}

// Translated from 00540a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray<NiAGDDataBlock*, ...>::_scalar_deleting_destructor_` (by the
/// RTTI of its vtable): [`fn_00540800`], then the block is freed when bit 0
/// of `flags` is set. Returns `this`.
pub fn fn_00540a40(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00540800(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

// Translated from 00540a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPrimitiveArray<NiAGDDataBlock*, ...>::_scalar_deleting_destructor_`
/// (by the RTTI of its vtable): [`fn_00537ce0`] (the array destructor body),
/// then the block is freed when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00540a70(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00537ce0(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

// Translated from 00540aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<NiTPointerAllocator<unsigned_int>_unsigned_int_
/// TESGrassAreaParam_P_P>::_scalar_deleting_destructor_` (Xbox PDB):
/// [`fn_00540a10`], then the block is freed when bit 0 of `flags` is set.
/// Returns `this`.
pub fn ni_t_map_base_grass_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00540a10(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

// Translated from 00540ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases the Havok byte array `this`: asks for the allocator
/// (`004a4840`, whose stack argument is `this`) and runs `00540b70(this,
/// allocator)`.
pub fn fn_00540ad0(e: &mut Engine, this: Ptr) {
    let allocator = e.with_stack(4, |e, zero| {
        e.mem.set_u8(zero.addr(), 0);
        e.call(HK_ARRAY_ALLOCATOR, &args![zero, this]).u32()
    });
    e.call(HK_ARRAY_CLEAR, &args![this, allocator]);
}

// Translated from 00540b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray<NiAGDDataBlock*, ...>::NiTArray(maxSize, growBy)` (by the RTTI
/// of the vtable it stores): the array's vtable, the maximum size at +8, the
/// grow-by step at +0xe, no elements (+0xa, +0xc) and a table of
/// `max_size` entries from `0096afc0` (null when the size is 0) at +4.
/// Returns `this`.
pub fn fn_00540b00(e: &mut Engine, this: Ptr, max_size: u16, grow_by: u16) -> Ptr {
    e.mem.set_u32(this.addr(), BLOCK_ARRAY_VTABLE);
    e.mem.set_u16(this.addr() + 8, max_size);
    e.mem.set_u16(this.addr() + 0xe, grow_by);
    e.mem.set_u16(this.addr() + 0xa, 0);
    e.mem.set_u16(this.addr() + 0xc, 0);
    let table = if e.mem.u16(this.addr() + 8) != 0 {
        let count = e.mem.u16(this.addr() + 8) as u32;
        e.call(BLOCK_ARRAY_ALLOCATE, &args![count]).u32()
    } else {
        0
    };
    e.mem.set_u32(this.addr() + 4, table);
    this
}

// Translated from 00540bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Makes room for `size` elements in the Havok byte array `this`: when its
/// capacity (`0062a100`) is smaller, reserves (`hkArrayUtil::_reserve`) the
/// larger of twice the capacity and `size`, with element size 1.
pub fn fn_00540bc0(e: &mut Engine, this: Ptr, allocator: u32, size: i32) {
    let capacity = e.call(HK_ARRAY_CAPACITY, &args![this]).i32();
    if capacity < size {
        let doubled = capacity.wrapping_shl(1);
        let wanted = if size < doubled { doubled } else { size };
        e.call(HK_ARRAY_RESERVE, &args![allocator, this, wanted, 1u32]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00533120,
            tes_object_land_tes_object_land(Ptr<TESObjectLAND>) -> Ptr<TESObjectLAND>
        ),
        entry!(0x005331f0, fn_005331f0(Ptr) -> Ptr),
        entry!(
            0x00533210,
            tes_object_land_scalar_deleting_destructor(
                Ptr<TESObjectLAND>,
                u32,
            ) -> Ptr<TESObjectLAND>
        ),
        entry!(0x00533240, fn_00533240(Ptr) -> Ptr),
        entry!(0x00533260, tes_object_land_destructor(Ptr<TESObjectLAND>)),
        entry!(0x00533420, tes_object_land_initialize_statics()),
        entry!(0x00533f40, fn_00533f40(Ptr, Ptr)),
        entry!(0x00533fb0, fn_00533fb0(Ptr, u16)),
        entry!(0x00533fd0, fn_00533fd0(Ptr<TESObjectLAND>) -> i32),
        entry!(0x00534010, fn_00534010(Ptr<TESObjectLAND>) -> i32),
        entry!(0x00534050, fn_00534050(Ptr<TESObjectLAND>) -> f64),
        entry!(0x00534080, fn_00534080(Ptr<TESObjectLAND>) -> f64),
        entry!(0x005340b0, fn_005340b0(Ptr<TESObjectLAND>) -> Ptr),
        entry!(
            0x005340e0,
            tes_object_land_set_cell(Ptr<TESObjectLAND>, Ptr)
        ),
        entry!(0x00534140, fn_00534140(Ptr<TESObjectLAND>) -> bool),
        entry!(0x00534160, fn_00534160(Ptr<TESObjectLAND>, bool)),
        entry!(0x005341a0, fn_005341a0(Ptr<TESObjectLAND>, bool)),
        entry!(0x005341e0, fn_005341e0(Ptr<TESObjectLAND>) -> bool),
        entry!(0x00534200, fn_00534200(Ptr<TESObjectLAND>, bool)),
        entry!(0x00534240, fn_00534240(Ptr<TESObjectLAND>, u32, i32, Ptr)),
        entry!(0x00534390, fn_00534390(Ptr<TESObjectLAND>, u32, i32, Ptr)),
        entry!(0x00534550, fn_00534550(Ptr) -> Ptr),
        entry!(0x00534570, fn_00534570(Ptr, i32, Ptr)),
        entry!(0x005345c0, fn_005345c0(Ptr<TESObjectLAND>, u32, i32, Ptr)),
        entry!(0x00534780, fn_00534780(Ptr, i32, Ptr)),
        entry!(0x00534880, fn_00534880(Ptr, u32, u32) -> Ptr),
        entry!(0x005348c0, fn_005348c0(Ptr<TESObjectLAND>, Ptr)),
        entry!(
            0x00534ad0,
            tes_object_land_load(Ptr<TESObjectLAND>, Ptr) -> bool
        ),
        entry!(0x00535a60, fn_00535a60(Ptr)),
        entry!(0x00535aa0, fn_00535aa0(Ptr)),
        entry!(0x00535ae0, fn_00535ae0() -> Ptr),
        entry!(0x00535af0, fn_00535af0(Ptr<TESObjectLAND>, i32) -> Ptr),
        entry!(
            0x00535b30,
            tes_object_land_get_world_offset_for_block(Ptr<TESObjectLAND>, Ptr, u32) -> Ptr
        ),
        entry!(
            0x00535b90,
            tes_object_land_load_vertices(Ptr<TESObjectLAND>, bool) -> bool
        ),
        entry!(
            0x00535d00,
            tes_object_land_load_vertices_into_arrays(Ptr<TESObjectLAND>) -> bool
        ),
        entry!(
            0x00536d80,
            tes_object_land_un_load_vertices(Ptr<TESObjectLAND>) -> bool
        ),
        entry!(
            0x00537100,
            fn_00537100(Ptr<LoadedLandData>, u32) -> Ptr<LoadedLandData>
        ),
        entry!(0x00537130, fn_00537130(Ptr<LoadedLandData>)),
        entry!(
            0x005371b0,
            tes_object_land_make_land_tri_strips(Ptr<TESObjectLAND>, u8) -> Ptr
        ),
        entry!(0x005374f0, fn_005374f0(Ptr<TESObjectLAND>)),
        entry!(0x00537b10, fn_00537b10(Ptr) -> Ptr),
        entry!(0x00537b30, fn_00537b30(Ptr) -> Ptr),
        entry!(0x00537b50, fn_00537b50(Ptr) -> bool),
        entry!(0x00537b80, fn_00537b80(Ptr) -> Ptr),
        entry!(0x00537bb0, fn_00537bb0(Ptr) -> Ptr),
        entry!(0x00537bd0, fn_00537bd0(Ptr) -> Ptr),
        entry!(
            0x00537bf0,
            ni_additional_geometry_data_ni_additional_geometry_data(Ptr, u16) -> Ptr
        ),
        entry!(0x00537ca0, ni_additional_geometry_data_get_rtti() -> Ptr),
        entry!(
            0x00537cb0,
            ni_additional_geometry_data_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00537ce0, fn_00537ce0(Ptr)),
        entry!(0x00537d00, bs_packed_additional_geometry_data_get_rtti() -> Ptr),
        entry!(
            0x00537d10,
            bs_packed_additional_geometry_data_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00537d40, fn_00537d40(Ptr)),
        entry!(0x00537eb0, fn_00537eb0(Ptr<TESObjectLAND>)),
        entry!(0x00538110, tes_object_land_save(Ptr<TESObjectLAND>)),
        entry!(0x00538c00, fn_00538c00(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x00538d70, fn_00538d70(Ptr) -> Ptr),
        entry!(0x00538e10, fn_00538e10(Ptr)),
        entry!(0x00538e30, fn_00538e30(Ptr)),
        entry!(
            0x00538e50,
            hk_base_object_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00538e80, fn_00538e80(Ptr, u32) -> Ptr),
        entry!(0x00538eb0, fn_00538eb0(Ptr)),
        entry!(0x00538ef0, fn_00538ef0(Ptr) -> Ptr),
        entry!(0x00538f20, fn_00538f20(Ptr) -> Ptr),
        entry!(0x00538f40, fn_00538f40(Ptr)),
        entry!(
            0x00538f80,
            hkp_mopp_code_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00538fb0, fn_00538fb0(Ptr)),
        entry!(0x00539010, fn_00539010(Ptr<TESObjectLAND>, u8)),
        entry!(0x00539060, fn_00539060(Ptr<TESObjectLAND>, u32) -> u8),
        entry!(0x005391d0, fn_005391d0(Ptr<TESObjectLAND>, Ptr) -> u8),
        entry!(
            0x00539280,
            fn_00539280(Ptr<TESObjectLAND>, Ptr, u8, u8) -> u8
        ),
        entry!(0x00539360, fn_00539360(Ptr<TESObjectLAND>, Ptr, Ptr)),
        entry!(0x00539460, fn_00539460(Ptr<TESObjectLAND>) -> bool),
        entry!(0x005394a0, fn_005394a0(Ptr<TESObjectLAND>) -> bool),
        entry!(0x005394c0, fn_005394c0(Ptr<TESObjectLAND>, bool)),
        entry!(
            0x00539500,
            fn_00539500(Ptr<TESObjectLAND>, Ptr<TESObjectLAND>) -> bool
        ),
        entry!(0x00539960, fn_00539960(Ptr<TESObjectLAND>) -> bool),
        entry!(0x00539ef0, fn_00539ef0(Ptr, u32)),
        entry!(0x00539f20, fn_00539f20(Ptr, u32)),
        entry!(0x00539f40, fn_00539f40() -> u32),
        entry!(0x00539f50, fn_00539f50(Ptr, u32, u32) -> Ptr),
        entry!(0x00539fc0, ni_binary_extra_data_get_rtti() -> Ptr),
        entry!(
            0x00539fd0,
            ni_binary_extra_data_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0053a000, fn_0053a000(Ptr)),
        entry!(0x0053a070, fn_0053a070(Ptr, u32, u32)),
        entry!(
            0x0053a090,
            tes_object_land_update_mesh(Ptr<TESObjectLAND>, u8, u32)
        ),
        entry!(0x0053a510, fn_0053a510(Ptr) -> Ptr),
        entry!(0x0053a550, fn_0053a550(Ptr<TESObjectLAND>) -> f32),
        entry!(0x0053a5a0, fn_0053a5a0(Ptr<TESObjectLAND>, u8) -> u32),
        entry!(0x0053a5e0, fn_0053a5e0(Ptr<TESObjectLAND>, Ptr) -> u32),
        entry!(0x0053a630, fn_0053a630(Ptr<TESObjectLAND>, u8, u16) -> u32),
        entry!(0x0053a700, fn_0053a700(Ptr<TESObjectLAND>, u8, u16) -> u32),
        entry!(0x0053a760, fn_0053a760(Ptr<TESObjectLAND>, u8, u16, u32)),
        entry!(0x0053a7a0, fn_0053a7a0(Ptr<TESObjectLAND>, u8, u16) -> f32),
        entry!(
            0x0053a830,
            fn_0053a830(Ptr<TESObjectLAND>, u8, u16, u16) -> f32
        ),
        entry!(
            0x0053a8a0,
            fn_0053a8a0(Ptr<TESObjectLAND>, u8, u16, u16, f32)
        ),
        entry!(0x0053a940, fn_0053a940(Ptr<TESObjectLAND>)),
        entry!(0x0053abb0, fn_0053abb0(Ptr<TESObjectLAND>, u8, u16)),
        entry!(0x0053ace0, fn_0053ace0(Ptr<TESObjectLAND>, u8, u16)),
        entry!(0x0053ae30, fn_0053ae30(Ptr<TESObjectLAND>, u8, u16) -> f32),
        entry!(0x0053aeb0, fn_0053aeb0(Ptr<TESObjectLAND>)),
        entry!(
            0x0053b450,
            fn_0053b450(Ptr<TESObjectLAND>, Ptr, u8, u16, Ptr, Ptr) -> Ptr
        ),
        entry!(
            0x0053b550,
            tes_object_land_get_coord_data(Ptr<TESObjectLAND>, Ptr<CoordData>, Ptr, bool) -> bool
        ),
        entry!(0x0053bc10, fn_0053bc10(Ptr<TESObjectLAND>) -> bool),
        entry!(0x0053ca40, fn_0053ca40() -> f32),
        entry!(0x0053ca60, fn_0053ca60(Ptr) -> Ptr),
        entry!(0x0053ca80, fn_0053ca80(Ptr<TESObjectLAND>, i32, i32) -> u32),
        entry!(
            0x0053caf0,
            fn_0053caf0(Ptr<TESObjectLAND>, Ptr<CoordData>, Ptr, Ptr) -> bool
        ),
        entry!(0x0053d1a0, ni_point3_unit_cross(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x0053d280, fn_0053d280(Ptr, Ptr, f32) -> Ptr),
        entry!(
            0x0053d2e0,
            fn_0053d2e0(Ptr<TESObjectLAND>, Ptr, Ptr, Ptr) -> bool
        ),
        entry!(0x0053d330, fn_0053d330(Ptr<TESObjectLAND>, Ptr, bool, Ptr)),
        entry!(0x0053d8f0, fn_0053d8f0(Ptr<TESObjectLAND>, Ptr, Ptr)),
        entry!(
            0x0053da60,
            fn_0053da60(Ptr<TESObjectLAND>, Ptr, Ptr) -> bool
        ),
        entry!(0x0053db20, fn_0053db20(Ptr<TESObjectLAND>) -> bool),
        entry!(0x0053df00, fn_0053df00(Ptr<TESObjectLAND>) -> u32),
        entry!(0x0053df30, fn_0053df30(Ptr<TESObjectLAND>, Ptr)),
        entry!(0x0053e210, fn_0053e210(Ptr<TESObjectLAND>, Ptr) -> u8),
        entry!(0x0053e260, fn_0053e260(Ptr<TESObjectLAND>, i32, i32) -> u8),
        entry!(
            0x0053e290,
            fn_0053e290(Ptr<TESObjectLAND>, i32, i32, Ptr, Ptr)
        ),
        entry!(0x0053f0e0, fn_0053f0e0(Ptr<TESObjectLAND>, Ptr) -> Ptr),
        entry!(
            0x0053f120,
            fn_0053f120(Ptr<TESObjectLAND>, Ptr<CoordData>) -> Ptr
        ),
        entry!(
            0x0053f180,
            tes_object_land_get_land_height(Ptr<TESObjectLAND>, Ptr, Ptr) -> bool
        ),
        entry!(
            0x0053f1e0,
            fn_0053f1e0(Ptr<TESObjectLAND>, Ptr<CoordData>, Ptr) -> bool
        ),
        entry!(0x0053f390, fn_0053f390(Ptr<TESObjectLAND>, Ptr, i32) -> Ptr),
        entry!(
            0x0053f440,
            tes_object_land_get_min_max_land_height(Ptr<TESObjectLAND>, Ptr) -> Ptr
        ),
        entry!(
            0x0053f570,
            fn_0053f570(Ptr<TESObjectLAND>, Ptr, Ptr) -> bool
        ),
        entry!(
            0x0053fa10,
            fn_0053fa10(Ptr<TESObjectLAND>, Ptr<CoordData>, Ptr) -> bool
        ),
        entry!(0x0053fa40, fn_0053fa40(Ptr<TESObjectLAND>, bool)),
        entry!(
            0x005400e0,
            tes_object_land_init_distant_texture_blending(Ptr<TESObjectLAND>)
        ),
        entry!(0x00540540, fn_00540540(Ptr, i32) -> u32),
        entry!(0x00540570, fn_00540570(Ptr, i32) -> u32),
        entry!(
            0x005405a0,
            loaded_land_data_loaded_land_data(Ptr<LoadedLandData>) -> Ptr<LoadedLandData>
        ),
        entry!(0x00540700, fn_00540700(Ptr) -> Ptr),
        entry!(0x00540720, fn_00540720(Ptr) -> Ptr),
        entry!(0x00540750, fn_00540750(Ptr, i32)),
        entry!(0x00540780, fn_00540780(Ptr, u32) -> Ptr),
        entry!(0x005407b0, fn_005407b0(Ptr, u8)),
        entry!(
            0x005407d0,
            ni_t_pointer_map_grass_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00540800, fn_00540800(Ptr)),
        entry!(0x00540830, fn_00540830(Ptr, u16, u16) -> Ptr),
        entry!(0x00540860, fn_00540860(Ptr, u32, i32)),
        entry!(0x005408e0, fn_005408e0(Ptr)),
        entry!(0x00540940, fn_00540940(Ptr, u32) -> Ptr),
        entry!(0x005409b0, fn_005409b0(Ptr)),
        entry!(0x00540a10, fn_00540a10(Ptr)),
        entry!(0x00540a40, fn_00540a40(Ptr, u32) -> Ptr),
        entry!(0x00540a70, fn_00540a70(Ptr, u32) -> Ptr),
        entry!(
            0x00540aa0,
            ni_t_map_base_grass_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00540ad0, fn_00540ad0(Ptr)),
        entry!(0x00540b00, fn_00540b00(Ptr, u16, u16) -> Ptr),
        entry!(0x00540bc0, fn_00540bc0(Ptr, u32, i32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    // ---- Test support ----------------------------------------------------------

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn float_ret(value: f32) -> Ret {
        Ret {
            st0: value as f64,
            ..Ret::default()
        }
    }

    /// The pages that hold the exe's constants and the statics.
    const PAGES: [u32; 24] = [
        0x0101_1000,
        0x0101_2000,
        0x0101_5000,
        0x0101_6000,
        0x0101_7000,
        0x0101_d000,
        0x0101_e000,
        0x0102_3000,
        0x0102_d000,
        0x0102_e000,
        0x0118_7000,
        0x011f_9000,
        0x0118_3000,
        0x0118_a000,
        0x0118_b000,
        0x011a_9000,
        0x011c_3000,
        0x011c_9000,
        0x011c_a000,
        0x011f_4000,
        0x011f_5000,
        0x0101_f000,
        0x011c_8000,
        0x011d_e000,
    ];

    /// An engine with the exe's constants the code reads and doubles for the
    /// helpers nearly every function goes through (allocator, `memcpy`,
    /// `memset`, scope guard, smart-pointer reader and the vector
    /// constructors).
    fn engine() -> Engine {
        let mut e = Engine::new();
        // A cast that finds the object (tests of the cast replace it).
        e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0]));
        for page in PAGES {
            e.map(page, 0x1000);
        }
        e.set_global(LARGEST_F32, f32::MAX);
        e.set_global(LOWEST_F32, -f32::MAX);
        e.set_global(SIXTEEN_F32, 16.0f32);
        e.set_global(F32_1024, 1024.0f32);
        e.set_global(F32_MINUS_2048, -2048.0f32);
        e.set_global(F64_ZERO, 0.0f64);
        e.set_global(F64_ONE, 1.0f64);
        e.set_global(F64_HALF, 0.5f64);
        e.set_global(F64_TWO, 2.0f64);
        e.set_global(F64_FOUR, 4.0f64);
        e.set_global(F64_HUNDRED, 100.0f64);
        e.set_global(F64_127, 127.0f64);
        e.set_global(F64_255, 255.0f64);
        e.set_global(F64_1024, 1024.0f64);
        e.set_global(F64_2048, 2048.0f64);
        e.set_global(F64_4096, 4096.0f64);
        e.set_global(F32_4096, 4096.0f32);
        e.set_global(F32_128, 128.0f32);
        e.set_global(F64_289, 289.0f64);
        e.set_global(F64_128, 128.0f64);
        e.set_global(F64_NINE, 9.0f64);
        e.set_global(F64_POINT_NINE, 0.9f64);
        e.set_global(F64_TENTH, 0.1f32 as f64);
        e.set_global(F64_THOUSANDTH, 0.001f32 as f64);
        e.set_global(F64_MILLIONTH, 1e-6f32 as f64);
        for (i, value) in [1.0f32, 1.0, 1.0, 1.0].into_iter().enumerate() {
            e.set_global(DEFAULT_COLOR_WORDS + 4 * i as u32, value);
        }
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(DEALLOCATE, |e, a| {
            e.mem.free(a[0]);
            Ret::default()
        });
        e.register(MEMORY_COPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            ret(a[0])
        });
        e.register(MEMORY_SET, |e, a| {
            let bytes = vec![a[1] as u8; a[2] as usize];
            e.mem.write(a[0], &bytes);
            ret(a[0])
        });
        e.register(SCOPE_GUARD_ENTER, |_, _| Ret::default());
        e.register(SCOPE_GUARD_LEAVE, |_, _| Ret::default());
        e.register(SMART_POINTER_GET, |e, a| ret(e.mem.u32(a[0])));
        e.register(NI_POINT3_CONSTRUCT, |e, a| {
            for i in 0..3 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
            }
            ret(a[0])
        });
        e.register(NI_POINT2_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            ret(a[0])
        });
        e.register(NI_COLOR_CONSTRUCT, |e, a| {
            for i in 0..4 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
            }
            ret(a[0])
        });
        e.register(NI_POINT3_DEFAULT_CONSTRUCT, |_, a| ret(a[0]));
        e.register(NI_COLOR_DEFAULT_CONSTRUCT, |_, a| ret(a[0]));
        e.register(VECTOR_CONSTRUCT, |e, a| {
            for i in 0..a[2] {
                e.call(a[3], &args![a[0] + i * a[1]]);
            }
            ret(a[0])
        });
        // `0045cb90`: loaded data with vertex arrays.
        e.register(LAND_HAS_VERTEX_ARRAYS, |e, a| {
            let data = e.mem.u32(a[0] + 0x28);
            ret((data != 0 && e.mem.u32(data + 4) != 0) as u32)
        });
        e.register(PARENT_CELL, |e, a| ret(e.mem.u32(a[0] + 0x20)));
        e.register(FORM_ID, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        e
    }

    /// Starts recording calls.
    fn start_log(e: &mut Engine) {
        e.call_log = Some(vec![]);
    }

    /// The calls recorded since `start_log`, without the first one (the call
    /// of the function under test), and stops recording.
    fn end_log(e: &mut Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log.take().unwrap().into_iter().skip(1).collect()
    }

    fn called(e: &mut Engine) -> Vec<u32> {
        end_log(e).into_iter().map(|(address, _)| address).collect()
    }

    fn arguments_of(log: &[(u32, Vec<u32>)], address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// A shared list the doubles append to.
    fn recorder<T: 'static>() -> Rc<RefCell<Vec<T>>> {
        Rc::new(RefCell::new(Vec::new()))
    }

    fn land(e: &mut Engine) -> Ptr<TESObjectLAND> {
        e.new_object()
    }

    /// An object with a vtable whose `(offset, target)` slots are given.
    fn object_with_vtable(e: &mut Engine, size: u32, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x800);
        for (offset, target) in slots {
            e.mem.set_u32(vtable + offset, *target);
        }
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, vtable);
        object
    }

    fn read_cstring(e: &Engine, address: u32) -> String {
        String::from_utf8(e.mem.cstr(address)).unwrap()
    }

    /// Floats at `address`.
    fn floats(e: &Engine, address: u32, count: u32) -> Vec<f32> {
        (0..count).map(|i| e.mem.f32(address + 4 * i)).collect()
    }

    /// A land with loaded data whose arrays are all allocated like the
    /// game's: four blocks of 0x121 vertices, normals (0xd8c bytes each),
    /// colours (0x1210), normal-set flags (0x121), the percent arrays and
    /// the layer texture arrays. Returns the land and its data.
    fn land_with_data(e: &mut Engine) -> (Ptr<TESObjectLAND>, Ptr<LoadedLandData>) {
        let this = land(e);
        let data: Ptr<LoadedLandData> = e.new_object();
        e.set(this, TESObjectLAND::pLoadedData, data.cast());
        for (field, size) in [
            (LoadedLandData::ppVertices.off, 0xd8cu32),
            (LoadedLandData::ppNormals.off, 0xd8c),
            (LoadedLandData::ppColorsA.off, 0x1210),
            (LoadedLandData::ppNormalsSet.off, 0x121),
        ] {
            let array = e.mem.alloc(16);
            for quadrant in 0..4 {
                let block = e.mem.alloc(size);
                e.mem.set_u32(array + 4 * quadrant, block);
            }
            e.mem.set_u32(data.addr() + field, array);
        }
        for quadrant in 0..4 {
            let layers = e.mem.alloc(0x18);
            e.mem.set_u32(
                data.addr() + LoadedLandData::pQuadTextureArray.off + 4 * quadrant,
                layers,
            );
            let percent = e.mem.alloc(4);
            let block = e.mem.alloc(0x2420);
            e.mem.set_u32(percent, block);
            e.mem.set_u32(
                data.addr() + LoadedLandData::ppPercentArrays.off + 4 * quadrant,
                percent,
            );
        }
        (this, data)
    }

    // ---- Constructor, destructors and statics ---------------------------------

    #[test]
    fn constructor_sets_up_the_land() {
        let mut e = engine();
        for address in [
            FORM_CONSTRUCT,
            NI_POINTER_SLOT_CONSTRUCT,
            FORM_SET_FORM_TYPE,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.set_global(STATICS_DEFINED, 1u8);
        e.set_global(LANDS_IN_USE, 2u32);
        let this = land(&mut e);
        e.set(this, TESObjectLAND::Data, 0xffff_ffff);
        e.set(this, TESObjectLAND::pParentCell, Ptr::new(0x1234));
        e.set(this, TESObjectLAND::pLoadedData, Ptr::new(0x5678));
        start_log(&mut e);
        let result = e.call(0x0053_3120, &args![this]);
        assert_eq!(result.u32(), this.addr());
        let log = end_log(&mut e);
        let addresses: Vec<u32> = log.iter().map(|(a, _)| *a).collect();
        assert_eq!(
            addresses,
            vec![
                FORM_CONSTRUCT,
                NI_POINTER_SLOT_CONSTRUCT,
                MEMORY_SET,
                FORM_SET_FORM_TYPE
            ]
        );
        assert_eq!(
            log[1].1,
            vec![this.addr() + 0x24, 0],
            "the queued textures slot"
        );
        assert_eq!(log[3].1, vec![this.addr(), 0x42]);
        assert_eq!(e.mem.u32(this.addr()), LAND_VTABLE);
        assert_eq!(e.mem.u32(this.addr() + 0x18), CHILD_CELL_VTABLE);
        assert_eq!(e.get(this, TESObjectLAND::Data), 0);
        assert!(e.get(this, TESObjectLAND::pParentCell).is_null());
        assert!(e.get(this, TESObjectLAND::pLoadedData).is_null());
        assert_eq!(e.global::<u32>(LANDS_IN_USE), 3);
    }

    #[test]
    fn child_cell_constructor_stores_the_abstract_vtable() {
        let mut e = engine();
        let object = e.mem.alloc(8);
        let result = e.call(0x0053_3240, &args![object]);
        assert_eq!(result.u32(), object);
        assert_eq!(e.mem.u32(object), CHILD_CELL_ABSTRACT_VTABLE);
    }

    #[test]
    fn save_parent_cell_is_the_lands_cell() {
        let mut e = engine();
        let this = land(&mut e);
        e.set(this, TESObjectLAND::pParentCell, Ptr::new(0x7000));
        let child = this.addr() + 0x18;
        assert_eq!(e.call(0x0053_31f0, &args![child]).u32(), 0x7000);
    }

    /// Doubles for what the destructor calls besides the unload.
    fn destructor_engine() -> Engine {
        let mut e = engine();
        for address in [
            LAND_SET_LOADED,
            LAND_RELEASE_BORDER,
            FORM_BASE_DESTRUCT,
            QUEUED_FILE_POINTER_DESTRUCT,
            FORM_DESTRUCT,
            TEXTURING_PROPERTY_SET,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e
    }

    #[test]
    fn destructor_keeps_the_statics_while_other_lands_exist() {
        let mut e = destructor_engine();
        e.set_global(LANDS_IN_USE, 2u32);
        let this = land(&mut e);
        start_log(&mut e);
        e.call(0x0053_3260, &args![this]);
        assert_eq!(
            called(&mut e),
            vec![
                LAND_SET_LOADED,
                LAND_RELEASE_BORDER,
                FORM_BASE_DESTRUCT,
                QUEUED_FILE_POINTER_DESTRUCT,
                FORM_DESTRUCT
            ]
        );
        assert_eq!(e.global::<u32>(LANDS_IN_USE), 1);
        assert_eq!(e.mem.u32(this.addr()), LAND_VTABLE);
        assert_eq!(e.mem.u32(this.addr() + 0x18), CHILD_CELL_VTABLE);
    }

    #[test]
    fn destructor_of_the_last_land_frees_the_statics() {
        let mut e = destructor_engine();
        e.set_global(LANDS_IN_USE, 1u32);
        e.set_global(STATICS_DEFINED, 1u8);
        let mut blocks = vec![];
        for global in [
            DEFAULT_TRIANGLE_LIST,
            DEFAULT_COLORS,
            DEFAULT_NORMALS,
            DEFAULT_TEXTURE_COORDINATES,
            DEFAULT_NORMAL_SET,
        ] {
            let block = e.mem.alloc(16);
            e.set_global(global, block);
            blocks.push(block);
        }
        for quadrant in 0..4 {
            let block = e.mem.alloc(16);
            e.set_global(DEFAULT_VERTEX_BLOCKS + 4 * quadrant, block);
            blocks.push(block);
        }
        // The default land texture has a destructor in slot 4; the texture
        // set is absent.
        let texture = object_with_vtable(&mut e, 0x28, &[(0x10, 0x00aa_0001)]);
        e.set_global(DEFAULT_LAND_TEXTURE, texture);
        e.set_global(DEFAULT_TEXTURE_SET, 0u32);
        let deleted = recorder::<Vec<u32>>();
        let sink = deleted.clone();
        e.register_double(0x00aa_0001, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let this = land(&mut e);
        start_log(&mut e);
        e.call(0x0053_3260, &args![this]);
        let log = end_log(&mut e);
        let freed: Vec<u32> = arguments_of(&log, DEALLOCATE)
            .into_iter()
            .map(|a| a[0])
            .collect();
        assert_eq!(freed, blocks);
        assert_eq!(e.global::<u8>(STATICS_DEFINED), 0);
        assert_eq!(
            arguments_of(&log, TEXTURING_PROPERTY_SET),
            vec![vec![DEFAULT_TEXTURING_PROPERTY, 0]]
        );
        assert_eq!(*deleted.borrow(), vec![vec![texture, 1]]);
        assert_eq!(e.global::<u32>(DEFAULT_LAND_TEXTURE), 0);
        assert_eq!(e.global::<u32>(DEFAULT_TEXTURE_SET), 0);
        assert_eq!(e.global::<u32>(LANDS_IN_USE), 0);
    }

    #[test]
    fn scalar_deleting_destructor_frees_only_with_bit_zero() {
        let mut e = destructor_engine();
        e.set_global(LANDS_IN_USE, 5u32);
        let this = land(&mut e);
        start_log(&mut e);
        let kept = e.call(0x0053_3210, &args![this, 0u32]);
        assert_eq!(kept.u32(), this.addr());
        assert!(!called(&mut e).contains(&DEALLOCATE));
        let other = land(&mut e);
        start_log(&mut e);
        let freed = e.call(0x0053_3210, &args![other, 1u32]);
        assert_eq!(freed.u32(), other.addr());
        let log = end_log(&mut e);
        assert_eq!(log.last().unwrap(), &(DEALLOCATE, vec![other.addr()]));
    }

    /// What `InitializeStatics` calls, with the texture set object it
    /// builds, the map and the texturing property recorded.
    struct StaticsRecord {
        paths: Rc<RefCell<Vec<(u32, String)>>>,
        slot_sets: Rc<RefCell<Vec<Vec<u32>>>>,
    }

    /// The engine for `InitializeStatics`: `tiling` is the value of the
    /// INI setting.
    fn statics_engine(tiling: f32) -> (Engine, StaticsRecord) {
        let mut e = engine();
        // Object allocator.
        e.register(NI_ALLOCATE_OBJECT, |e, a| ret(e.mem.alloc(a[0].max(0x200))));
        // The texture set: a vtable at +0x30 whose slot 0x90 stores a
        // texture into the smart pointer it is given.
        let texture_vtable = e.mem.alloc(0x100);
        e.mem.set_u32(texture_vtable + 0x90, 0x00aa_0002);
        e.register(0x00aa_0002, |e, a| {
            e.mem.set_u32(a[2], 0x7777_0000);
            Ret::default()
        });
        e.register_double(TEXTURE_SET_CONSTRUCT, move |e, a| {
            e.mem.set_u32(a[0] + 0x30, texture_vtable);
            ret(a[0])
        });
        e.register(SETTING_GET_STRING, |e, a| {
            let text = if a[0] == DEFAULT_DIFFUSE_TEXTURE_SETTING {
                "Diffuse.dds"
            } else {
                "Normal.dds"
            };
            let address = e.mem.alloc(16);
            e.mem.set_cstr(address, text.as_bytes());
            ret(address)
        });
        e.register(FORMAT_STRING, |e, a| {
            let name = e.mem.cstr(a[3]);
            let mut text = b"Landscape\\".to_vec();
            text.extend_from_slice(&name);
            e.mem.set_cstr(a[0], &text);
            ret(text.len() as u32)
        });
        let paths = recorder::<(u32, String)>();
        let sink = paths.clone();
        e.register_double(TEXTURE_SET_SET_PATH, move |e, a| {
            sink.borrow_mut()
                .push((a[1], String::from_utf8(e.mem.cstr(a[2])).unwrap()));
            Ret::default()
        });
        e.set_global(DATA_HANDLER, 0x0900_0000u32);
        e.register(DATA_HANDLER_WORD_AT_4, |_, a| ret(a[0] + 1));
        e.register(DATA_HANDLER_ADD_TEXTURE_SET, |_, _| Ret::default());
        e.register(LAND_TEXTURE_CONSTRUCT, |_, a| ret(a[0]));
        e.register(LAND_TEXTURE_SET_TEXTURE_SET, |_, _| Ret::default());
        e.register(SMART_POINTER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(SMART_POINTER_RELEASE, |e, a| {
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
        e.register(TEXTURE_MAP_CONSTRUCT, |_, a| ret(a[0] + 1));
        e.register(TEXTURING_PROPERTY_CONSTRUCT, |_, a| ret(a[0]));
        e.register(TEXTURING_PROPERTY_SET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(PROPERTY_SET_MODE, |_, _| Ret::default());
        // The property's map table: slot 0 lives at `table + 0x100`.
        e.register(ARRAY_ELEMENT_ADDRESS, |_, a| ret(a[0] + 0x100));
        let slot_sets = recorder::<Vec<u32>>();
        let sink = slot_sets.clone();
        e.register_double(ARRAY_SET_AT, move |e, a| {
            sink.borrow_mut().push(vec![a[0], a[1], e.mem.u32(a[2])]);
            Ret::default()
        });
        let setting = e.mem.alloc(4);
        e.mem.set_f32(setting, tiling);
        e.register_double(SETTING_GET_FLOAT_ADDRESS, move |_, _| ret(setting));
        e.register(LOG_MESSAGE, |_, _| Ret::default());
        (e, StaticsRecord { paths, slot_sets })
    }

    #[test]
    fn initialize_statics_builds_the_shared_blocks() {
        let (mut e, record) = statics_engine(8.0);
        start_log(&mut e);
        e.call(0x0053_3420, &[]);
        let log = end_log(&mut e);
        // The guard is entered with the tag, the source file and the line.
        assert_eq!(
            log[0],
            (
                SCOPE_GUARD_ENTER,
                vec![log[0].1[0], 0x1b, 1, SOURCE_FILE, 0xc6]
            )
        );
        assert_eq!(log.last().unwrap().0, SCOPE_GUARD_LEAVE);
        // The texture set and its two paths.
        let texture_set = e.global::<u32>(DEFAULT_TEXTURE_SET);
        assert_ne!(texture_set, 0);
        assert_eq!(
            *record.paths.borrow(),
            vec![
                (0, "Landscape\\Diffuse.dds".to_string()),
                (1, "Landscape\\Normal.dds".to_string())
            ]
        );
        assert_eq!(
            arguments_of(&log, DATA_HANDLER_WORD_AT_4),
            vec![vec![0x0900_0000]]
        );
        assert_eq!(
            arguments_of(&log, DATA_HANDLER_ADD_TEXTURE_SET),
            vec![vec![0x0900_0001, texture_set]]
        );
        let land_texture = e.global::<u32>(DEFAULT_LAND_TEXTURE);
        assert_eq!(
            arguments_of(&log, LAND_TEXTURE_SET_TEXTURE_SET),
            vec![vec![land_texture, texture_set]]
        );
        // The map is built from the texture the set's slot 0x90 produced
        // and becomes slot 0 of the property; the property gets mode 2.
        let maps = arguments_of(&log, TEXTURE_MAP_CONSTRUCT);
        assert_eq!(maps.len(), 1);
        assert_eq!(maps[0][1..], [0x7777_0000, 0, 3, 5, 0]);
        let map = maps[0][0] + 1;
        let property = e.mem.u32(DEFAULT_TEXTURING_PROPERTY);
        assert_ne!(property, 0);
        assert_eq!(
            arguments_of(&log, PROPERTY_SET_MODE),
            vec![vec![property, 2, 0xe, 1]]
        );
        assert_eq!(
            *record.slot_sets.borrow(),
            vec![vec![property + 0x1c, 0, map]]
        );
        // The triangle list: cell (0, 0) has the same parity in both
        // directions, cell (0, 1) does not.
        let triangles = e.global::<u32>(DEFAULT_TRIANGLE_LIST);
        let cell = |row: u32, column: u32| -> Vec<u16> {
            (0..6)
                .map(|i| e.mem.u16(triangles + 2 * ((row * 16 + column) * 6 + i)))
                .collect()
        };
        assert_eq!(cell(0, 0), vec![18, 17, 0, 0, 1, 18]);
        assert_eq!(cell(0, 1), vec![18, 1, 2, 2, 19, 18]);
        assert_eq!(cell(1, 0), vec![34, 17, 18, 18, 35, 34]);
        assert_eq!(cell(15, 15), vec![288, 287, 270, 270, 271, 288]);
        // Colours, normals, flags and texture coordinates (4 / 8 = 0.5).
        let colors = e.global::<u32>(DEFAULT_COLORS);
        let normals = e.global::<u32>(DEFAULT_NORMALS);
        let flags = e.global::<u32>(DEFAULT_NORMAL_SET);
        let coordinates = e.global::<u32>(DEFAULT_TEXTURE_COORDINATES);
        let k = 17 + 2;
        assert_eq!(floats(&e, colors + 0x10 * k, 4), vec![1.0, 1.0, 1.0, 0.0]);
        assert_eq!(floats(&e, normals + 12 * k, 3), vec![0.0, 0.0, 1.0]);
        assert_eq!(e.mem.u8(flags + k), 1);
        assert_eq!(e.mem.u8(flags + 0x120), 1);
        assert_eq!(floats(&e, coordinates + 8 * k, 2), vec![4.0, 2.0]);
        // Quadrant offsets and the default vertex blocks at -2048.
        let x_offsets = floats(&e, X_OFFSETS, 4);
        let y_offsets = floats(&e, Y_OFFSETS, 4);
        assert_eq!(x_offsets, vec![-2048.0, 0.0, -2048.0, 0.0]);
        assert_eq!(y_offsets, vec![-2048.0, -2048.0, 0.0, 0.0]);
        let block = e.global::<u32>(DEFAULT_VERTEX_BLOCKS + 4);
        assert_eq!(
            floats(&e, block + 12 * (2 * 17 + 3), 3),
            vec![384.0, -1792.0, -2048.0]
        );
        let last = e.global::<u32>(DEFAULT_VERTEX_BLOCKS + 12);
        assert_eq!(
            floats(&e, last + 12 * 0x120, 3),
            vec![2048.0, 2048.0, -2048.0]
        );
        assert_eq!(e.global::<u8>(STATICS_DEFINED), 1);
        // The smart pointer that held the texture is released at the end.
        assert!(arguments_of(&log, SMART_POINTER_RELEASE).len() == 1);
    }

    #[test]
    fn initialize_statics_replaces_a_zero_tiling_with_sixteen() {
        let (mut e, _) = statics_engine(0.0);
        start_log(&mut e);
        e.call(0x0053_3420, &[]);
        let log = end_log(&mut e);
        assert_eq!(arguments_of(&log, LOG_MESSAGE), vec![vec![TILING_WARNING]]);
        // 4 / 16 = 0.25, so column 1 is at u = 4.
        let coordinates = e.global::<u32>(DEFAULT_TEXTURE_COORDINATES);
        assert_eq!(floats(&e, coordinates + 8, 2), vec![4.0, 0.0]);
    }

    #[test]
    fn constructor_defines_the_statics_once() {
        let (mut e, _) = statics_engine(16.0);
        for address in [
            FORM_CONSTRUCT,
            NI_POINTER_SLOT_CONSTRUCT,
            FORM_SET_FORM_TYPE,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        let this = land(&mut e);
        e.call(0x0053_3120, &args![this]);
        assert_eq!(e.global::<u8>(STATICS_DEFINED), 1);
        assert_eq!(e.global::<u32>(LANDS_IN_USE), 1);
        let first = e.global::<u32>(DEFAULT_TRIANGLE_LIST);
        let other = land(&mut e);
        e.call(0x0053_3120, &args![other]);
        assert_eq!(e.global::<u32>(DEFAULT_TRIANGLE_LIST), first);
        assert_eq!(e.global::<u32>(LANDS_IN_USE), 2);
    }

    #[test]
    fn texture_map_slot_is_replaced_only_when_it_differs() {
        let mut e = engine();
        e.register(ARRAY_ELEMENT_ADDRESS, |_, a| ret(a[0] + 0x100));
        let sets = recorder::<Vec<u32>>();
        let sink = sets.clone();
        e.register_double(ARRAY_SET_AT, move |e, a| {
            sink.borrow_mut().push(vec![a[0], a[1], e.mem.u32(a[2])]);
            Ret::default()
        });
        let deleted = recorder::<Vec<u32>>();
        let sink = deleted.clone();
        e.register_double(0x00aa_0003, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let property = e.mem.alloc(0x200);
        let old = object_with_vtable(&mut e, 0x10, &[(0, 0x00aa_0003)]);
        e.mem.set_u32(property + 0x1c + 0x100, old);
        // The same map: nothing happens.
        e.call(0x0053_3f40, &args![property, old]);
        assert!(sets.borrow().is_empty() && deleted.borrow().is_empty());
        // A different map: the old one is deleted and the slot set.
        e.call(0x0053_3f40, &args![property, 0x4444u32]);
        assert_eq!(*deleted.borrow(), vec![vec![old, 1]]);
        assert_eq!(*sets.borrow(), vec![vec![property + 0x1c, 0, 0x4444]]);
        // An empty slot: no deletion.
        e.mem.set_u32(property + 0x1c + 0x100, 0);
        e.call(0x0053_3f40, &args![property, 0x5555u32]);
        assert_eq!(deleted.borrow().len(), 1);
        assert_eq!(sets.borrow().len(), 2);
    }

    #[test]
    fn property_mode_setter_passes_the_value() {
        let mut e = engine();
        e.register(PROPERTY_SET_MODE, |_, _| Ret::default());
        start_log(&mut e);
        e.call(0x0053_3fb0, &args![0x1000u32, 0xffff_0002u32]);
        let log = end_log(&mut e);
        assert_eq!(log, vec![(PROPERTY_SET_MODE, vec![0x1000, 2, 0xe, 1])]);
    }

    // ---- Cell accessors and flags ---------------------------------------------

    #[test]
    fn cell_coordinates_come_from_the_data_then_the_cell() {
        let mut e = engine();
        e.register(CELL_GET_DATA_X, |_, _| ret(-7i32 as u32));
        e.register(CELL_GET_DATA_Y, |_, _| ret(9));
        let this = land(&mut e);
        // No data and no cell: zero.
        assert_eq!(e.call(0x0053_3fd0, &args![this]).i32(), 0);
        assert_eq!(e.call(0x0053_4010, &args![this]).i32(), 0);
        // The cell answers.
        e.set(this, TESObjectLAND::pParentCell, Ptr::new(0x3000));
        assert_eq!(e.call(0x0053_3fd0, &args![this]).i32(), -7);
        assert_eq!(e.call(0x0053_4010, &args![this]).i32(), 9);
        // The data wins.
        let data: Ptr<LoadedLandData> = e.new_object();
        e.set(data, LoadedLandData::iCellX, 12);
        e.set(data, LoadedLandData::iCellY, -13);
        e.set(this, TESObjectLAND::pLoadedData, data.cast());
        assert_eq!(e.call(0x0053_3fd0, &args![this]).i32(), 12);
        assert_eq!(e.call(0x0053_4010, &args![this]).i32(), -13);
    }

    #[test]
    fn world_corner_is_the_cell_times_4096() {
        let mut e = engine();
        let this = land(&mut e);
        let data: Ptr<LoadedLandData> = e.new_object();
        e.set(data, LoadedLandData::iCellX, -3);
        e.set(data, LoadedLandData::iCellY, 5);
        e.set(this, TESObjectLAND::pLoadedData, data.cast());
        assert_eq!(e.call(0x0053_4050, &args![this]).f64(), -12288.0);
        assert_eq!(e.call(0x0053_4080, &args![this]).f64(), 20480.0);
    }

    #[test]
    fn world_space_is_asked_of_the_cell() {
        let mut e = engine();
        e.register(CELL_GET_WORLD_SPACE, |_, a| ret(a[0] + 1));
        let this = land(&mut e);
        assert_eq!(e.call(0x0053_40b0, &args![this]).u32(), 0);
        e.set(this, TESObjectLAND::pParentCell, Ptr::new(0x3000));
        assert_eq!(e.call(0x0053_40b0, &args![this]).u32(), 0x3001);
    }

    #[test]
    fn set_cell_sets_flag_400_from_the_world_space() {
        let mut e = engine();
        // The cell's world space is `cell + 1` unless the cell is 0x5000
        // (none); the world space has land data when its address is odd.
        e.register(CELL_GET_WORLD_SPACE, |_, a| {
            ret(if a[0] == 0x5000 { 0 } else { a[0] + 1 })
        });
        e.register(WORLD_SPACE_HAS_LAND, |_, a| ret(a[0] & 1));
        let this = land(&mut e);
        e.set(this, TESObjectLAND::Data, 0x401);
        // A null cell only stores.
        e.call(0x0053_40e0, &args![this, 0u32]);
        assert_eq!(e.get(this, TESObjectLAND::Data), 0x401);
        // A world space with land data sets the bit...
        e.set(this, TESObjectLAND::Data, 0);
        e.call(0x0053_40e0, &args![this, 0x3000u32]);
        assert_eq!(e.get(this, TESObjectLAND::Data), 0x400);
        assert_eq!(e.get(this, TESObjectLAND::pParentCell).addr(), 0x3000);
        // ...one without clears it, and so does a cell without a world space.
        e.call(0x0053_40e0, &args![this, 0x3001u32]);
        assert_eq!(e.get(this, TESObjectLAND::Data), 0);
        e.set(this, TESObjectLAND::Data, 0x400);
        e.call(0x0053_40e0, &args![this, 0x5000u32]);
        assert_eq!(e.get(this, TESObjectLAND::Data), 0);
    }

    #[test]
    fn flag_bits_are_tested_and_set() {
        let mut e = engine();
        let this = land(&mut e);
        e.set(this, TESObjectLAND::Data, 0x2);
        assert_eq!(e.call(0x0053_4140, &args![this]).u32() & 0xff, 0);
        e.call(0x0053_4160, &args![this, 1u32]);
        assert_eq!(e.get(this, TESObjectLAND::Data), 0x402);
        assert_eq!(e.call(0x0053_4140, &args![this]).u32() & 0xff, 1);
        e.call(0x0053_4160, &args![this, 0u32]);
        assert_eq!(e.get(this, TESObjectLAND::Data), 0x2);
        e.call(0x0053_41a0, &args![this, 7u32]);
        assert_eq!(e.get(this, TESObjectLAND::Data), 0x802);
        e.call(0x0053_41a0, &args![this, 0u32]);
        assert_eq!(e.get(this, TESObjectLAND::Data), 0x2);
        assert_eq!(e.call(0x0053_41e0, &args![this]).u32() & 0xff, 0);
        e.call(0x0053_4200, &args![this, 1u32]);
        assert_eq!(e.get(this, TESObjectLAND::Data), 0x12);
        assert_eq!(e.call(0x0053_41e0, &args![this]).u32() & 0xff, 1);
        e.call(0x0053_4200, &args![this, 0u32]);
        assert_eq!(e.get(this, TESObjectLAND::Data), 0x2);
    }

    // ---- Vertex, normal and colour readers --------------------------------------

    /// Doubles for the mesh geometry the readers fall back to: the node's
    /// geometry is at `node + 4`, its positions at `geometry + 0`, normals
    /// at `+8`, colours at `+0xc`, its geometry data at `+0x10`; the data's
    /// owner (at `data + 0`) reports whether it can be locked.
    fn geometry_engine() -> Engine {
        let mut e = engine();
        e.register(LAND_DEFAULT_HEIGHT, |_, _| float_ret(7.0));
        e.register(GEOMETRY_GET_MODEL_DATA, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(GEOMETRY_GET_POSITIONS, |e, a| ret(e.mem.u32(a[0])));
        e.register(GEOMETRY_GET_NORMALS, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(GEOMETRY_GET_COLORS, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        e.register(GEOMETRY_GET_DATA, |e, a| ret(e.mem.u32(a[0] + 0x10)));
        e.register(GEOMETRY_DATA_GET_OWNER, |e, a| ret(e.mem.u32(a[0])));
        e.register(GEOMETRY_DATA_BEGIN, |_, _| ret(1));
        e.register(GEOMETRY_DATA_END, |_, _| Ret::default());
        e.register(LOCK_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0);
            e.mem.set_u32(a[0] + 4, 0);
            e.mem.set_u8(a[0] + 8, 0);
            ret(a[0])
        });
        e.register(LOCK_CHECK_INDEX, |_, _| Ret::default());
        e
    }

    /// A byte as the code turns it into a colour channel.
    fn byte_fraction(byte: u32) -> f32 {
        (byte as f64 / 255.0) as f32
    }

    /// A mesh node whose geometry is `geometry`.
    fn mesh_node(e: &mut Engine, geometry: u32) -> u32 {
        let node = e.mem.alloc(16);
        e.mem.set_u32(node + 4, geometry);
        node
    }

    /// A geometry with its positions, normals, colours and data pointers.
    fn geometry(e: &mut Engine, positions: u32, normals: u32, colors: u32, data: u32) -> u32 {
        let geometry = e.mem.alloc(0x20);
        e.mem.set_u32(geometry, positions);
        e.mem.set_u32(geometry + 8, normals);
        e.mem.set_u32(geometry + 0xc, colors);
        e.mem.set_u32(geometry + 0x10, data);
        geometry
    }

    /// Geometry data whose owner can (or cannot) be locked; the owner's
    /// slot 0x8c answers.
    fn lockable_data(e: &mut Engine, can_lock: bool) -> u32 {
        e.register(0x00aa_0010, |_, _| ret(1));
        e.register(0x00aa_0011, |_, _| ret(0));
        let owner = object_with_vtable(
            e,
            16,
            &[(0x8c, if can_lock { 0x00aa_0010 } else { 0x00aa_0011 })],
        );
        let data = e.mem.alloc(16);
        e.mem.set_u32(data, owner);
        data
    }

    #[test]
    fn vertex_reader_follows_the_available_sources() {
        let mut e = geometry_engine();
        let out = e.mem.alloc(12);
        // No vertex arrays: (0, 0, default height), no base height added.
        let this = land(&mut e);
        e.call(0x0053_4240, &args![this, 1u32, 5u32, out]);
        assert_eq!(floats(&e, out, 3), vec![0.0, 0.0, 7.0]);

        let (this, data) = land_with_data(&mut e);
        e.set(data, LoadedLandData::fBaseHeight, 10.0);
        let vertices = e.get(data, LoadedLandData::ppVertices);
        // From ppVertices.
        let block = element(&e, vertices, 1);
        for (i, value) in [1.0f32, 2.0, 3.0].into_iter().enumerate() {
            e.mem.set_f32(block + 12 * 5 + 4 * i as u32, value);
        }
        e.call(0x0053_4240, &args![this, 1u32, 5u32, out]);
        assert_eq!(floats(&e, out, 3), vec![1.0, 2.0, 13.0]);

        // From the mesh geometry when the block is missing.
        e.mem.set_u32(vertices.addr() + 4, 0);
        let positions = e.mem.alloc(12 * 8);
        for (i, value) in [4.0f32, 5.0, 6.0].into_iter().enumerate() {
            e.mem.set_f32(positions + 12 * 6 + 4 * i as u32, value);
        }
        let g = geometry(&mut e, positions, 0, 0, 0);
        let node = mesh_node(&mut e, g);
        let meshes = e.mem.alloc(16);
        e.mem.set_u32(meshes + 4, node);
        e.set(data, LoadedLandData::ppMesh, Ptr::new(meshes));
        e.call(0x0053_4240, &args![this, 1u32, 6u32, out]);
        assert_eq!(floats(&e, out, 3), vec![4.0, 5.0, 16.0]);

        // Neither: (0, 0, default height) plus the base height.
        e.mem.set_u32(vertices.addr() + 8, 0);
        e.call(0x0053_4240, &args![this, 2u32, 6u32, out]);
        assert_eq!(floats(&e, out, 3), vec![0.0, 0.0, 17.0]);
    }

    /// Fills the vertex `index` of a locked stream: `{base, stride, packed}`.
    fn locking_geometry(e: &mut Engine, packed: bool) -> (u32, u32) {
        let stream = e.mem.alloc(64);
        let data = lockable_data(e, true);
        let geometry = geometry(e, 0, 0, 0, data);
        let node = mesh_node(e, geometry);
        let lock_to_stream = stream;
        let stride = if packed { 4 } else { 16 };
        e.register_double(GEOMETRY_DATA_LOCK_POSITIONS, move |e, a| {
            e.mem.set_u32(a[1], lock_to_stream);
            e.mem.set_u32(a[1] + 4, 12);
            Ret::default()
        });
        e.register_double(GEOMETRY_DATA_LOCK_COLORS, move |e, a| {
            e.mem.set_u32(a[1], lock_to_stream);
            e.mem.set_u32(a[1] + 4, stride);
            e.mem.set_u8(a[1] + 8, packed as u8);
            Ret::default()
        });
        (node, stream)
    }

    #[test]
    fn normal_reader_follows_the_available_sources() {
        let mut e = geometry_engine();
        let out = e.mem.alloc(12);
        let this = land(&mut e);
        e.call(0x0053_4390, &args![this, 0u32, 1u32, out]);
        assert_eq!(floats(&e, out, 3), vec![0.0, 0.0, 1.0]);

        // ppNormals.
        let (this, data) = land_with_data(&mut e);
        let normals = e.get(data, LoadedLandData::ppNormals);
        let block = element(&e, normals, 2);
        for (i, value) in [0.5f32, 0.25, 0.75].into_iter().enumerate() {
            e.mem.set_f32(block + 12 * 3 + 4 * i as u32, value);
        }
        e.call(0x0053_4390, &args![this, 2u32, 3u32, out]);
        assert_eq!(floats(&e, out, 3), vec![0.5, 0.25, 0.75]);

        // The mesh geometry's normal array.
        e.mem.set_u32(normals.addr(), 0);
        let array = e.mem.alloc(12 * 4);
        for (i, value) in [0.0f32, 1.0, 0.0].into_iter().enumerate() {
            e.mem.set_f32(array + 12 * 2 + 4 * i as u32, value);
        }
        let g = geometry(&mut e, 0, array, 0, 0);
        let node = mesh_node(&mut e, g);
        let meshes = e.mem.alloc(16);
        e.mem.set_u32(meshes, node);
        e.set(data, LoadedLandData::ppMesh, Ptr::new(meshes));
        e.call(0x0053_4390, &args![this, 0u32, 2u32, out]);
        assert_eq!(floats(&e, out, 3), vec![0.0, 1.0, 0.0]);

        // No normal stream: the locked geometry data is read at the
        // lock's base and stride.
        let (node, stream) = locking_geometry(&mut e, false);
        for (i, value) in [0.6f32, 0.0, 0.8].into_iter().enumerate() {
            e.mem.set_f32(stream + 12 * 4 + 4 * i as u32, value);
        }
        e.mem.set_u32(meshes, node);
        start_log(&mut e);
        e.call(0x0053_4390, &args![this, 0u32, 4u32, out]);
        let calls = called(&mut e);
        assert_eq!(floats(&e, out, 3), vec![0.6, 0.0, 0.8]);
        assert_eq!(
            calls,
            vec![
                LAND_HAS_VERTEX_ARRAYS,
                GEOMETRY_GET_MODEL_DATA,
                GEOMETRY_GET_NORMALS,
                GEOMETRY_GET_DATA,
                LOCK_CONSTRUCT,
                GEOMETRY_DATA_GET_OWNER,
                0x00aa_0010,
                GEOMETRY_DATA_BEGIN,
                GEOMETRY_DATA_LOCK_POSITIONS,
                LOCK_CHECK_INDEX,
                GEOMETRY_DATA_END
            ]
        );

        // A geometry that cannot be locked gives the default normal.
        let unlockable = lockable_data(&mut e, false);
        let g = geometry(&mut e, 0, 0, 0, unlockable);
        let node = mesh_node(&mut e, g);
        e.mem.set_u32(meshes, node);
        e.call(0x0053_4390, &args![this, 0u32, 4u32, out]);
        assert_eq!(floats(&e, out, 3), vec![0.0, 0.0, 1.0]);
    }

    #[test]
    fn lock_helpers_read_the_vertex_stream() {
        let mut e = geometry_engine();
        let lock = e.mem.alloc(16);
        e.mem.set_u32(lock + 8, 0xffff_ffff);
        assert_eq!(e.call(0x0053_4550, &args![lock]).u32(), lock);
        assert_eq!(e.mem.u32(lock), 0);
        assert_eq!(e.mem.u8(lock + 8), 0);
        let stream = e.mem.alloc(64);
        for (i, value) in [1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0].into_iter().enumerate() {
            e.mem.set_f32(stream + 4 * i as u32, value);
        }
        e.mem.set_u32(lock, stream);
        e.mem.set_u32(lock + 4, 12);
        let out = e.mem.alloc(12);
        start_log(&mut e);
        e.call(0x0053_4570, &args![lock, 1u32, out]);
        assert_eq!(end_log(&mut e), vec![(LOCK_CHECK_INDEX, vec![lock, 1])]);
        assert_eq!(floats(&e, out, 3), vec![4.0, 5.0, 6.0]);
    }

    #[test]
    fn color_reader_follows_the_available_sources() {
        let mut e = geometry_engine();
        let out = e.mem.alloc(16);
        // No vertex arrays: the exe's default colour.
        let this = land(&mut e);
        e.call(0x0053_45c0, &args![this, 0u32, 1u32, out]);
        assert_eq!(floats(&e, out, 4), vec![1.0, 1.0, 1.0, 1.0]);

        // ppColorsA.
        let (this, data) = land_with_data(&mut e);
        let colors = e.get(data, LoadedLandData::ppColorsA);
        let block = element(&e, colors, 3);
        for (i, value) in [0.1f32, 0.2, 0.3, 0.4].into_iter().enumerate() {
            e.mem.set_f32(block + 0x10 * 2 + 4 * i as u32, value);
        }
        e.call(0x0053_45c0, &args![this, 3u32, 2u32, out]);
        assert_eq!(floats(&e, out, 4), vec![0.1, 0.2, 0.3, 0.4]);

        // The mesh geometry's colour array.
        e.mem.set_u32(colors.addr() + 12, 0);
        let array = e.mem.alloc(0x40);
        for (i, value) in [0.9f32, 0.8, 0.7, 0.6].into_iter().enumerate() {
            e.mem.set_f32(array + 0x10 + 4 * i as u32, value);
        }
        let g = geometry(&mut e, 0, 0x1000, array, 0);
        let node = mesh_node(&mut e, g);
        let meshes = e.mem.alloc(16);
        e.mem.set_u32(meshes + 12, node);
        e.set(data, LoadedLandData::ppMesh, Ptr::new(meshes));
        e.call(0x0053_45c0, &args![this, 3u32, 1u32, out]);
        assert_eq!(floats(&e, out, 4), vec![0.9, 0.8, 0.7, 0.6]);

        // A locked float stream, and a packed 0xAARRGGBB stream.
        e.register(GEOMETRY_GET_NORMALS, |_, _| ret(0));
        let (node, stream) = locking_geometry(&mut e, false);
        for (i, value) in [0.5f32, 0.4, 0.3, 0.2].into_iter().enumerate() {
            e.mem.set_f32(stream + 0x10 * 2 + 4 * i as u32, value);
        }
        e.mem.set_u32(meshes + 12, node);
        e.call(0x0053_45c0, &args![this, 3u32, 2u32, out]);
        assert_eq!(floats(&e, out, 4), vec![0.5, 0.4, 0.3, 0.2]);
        let (node, stream) = locking_geometry(&mut e, true);
        e.mem.set_u32(stream + 4 * 2, 0x80ff_4000);
        e.mem.set_u32(meshes + 12, node);
        e.call(0x0053_45c0, &args![this, 3u32, 2u32, out]);
        assert_eq!(
            floats(&e, out, 4),
            vec![1.0, byte_fraction(64), 0.0, byte_fraction(128)]
        );

        // Nothing usable: the default colour.
        let unlockable = lockable_data(&mut e, false);
        let g = geometry(&mut e, 0, 0, 0, unlockable);
        let node = mesh_node(&mut e, g);
        e.mem.set_u32(meshes + 12, node);
        e.call(0x0053_45c0, &args![this, 3u32, 2u32, out]);
        assert_eq!(floats(&e, out, 4), vec![1.0, 1.0, 1.0, 1.0]);
    }

    #[test]
    fn packed_color_stream_unpacks_argb() {
        let mut e = geometry_engine();
        e.register(LOCK_CHECK_INDEX, |_, _| Ret::default());
        let lock = e.mem.alloc(16);
        let stream = e.mem.alloc(16);
        e.mem.set_u32(stream + 4, 0x40_ff_80_00);
        e.mem.set_u32(lock, stream);
        e.mem.set_u32(lock + 4, 4);
        e.mem.set_u8(lock + 8, 1);
        let out = e.mem.alloc(16);
        e.call(0x0053_4780, &args![lock, 1u32, out]);
        assert_eq!(
            floats(&e, out, 4),
            vec![1.0, byte_fraction(128), 0.0, byte_fraction(64)]
        );
        // Unpacked: four floats are copied.
        e.mem.set_u8(lock + 8, 0);
        e.mem.set_u32(lock + 4, 16);
        for (i, value) in [0.1f32, 0.2, 0.3, 0.4].into_iter().enumerate() {
            e.mem.set_f32(stream + 16 + 4 * i as u32, value);
        }
        e.mem.set_u32(lock, stream);
        e.call(0x0053_4780, &args![lock, 1u32, out]);
        assert_eq!(floats(&e, out, 4), vec![0.1, 0.2, 0.3, 0.4]);
    }

    #[test]
    fn form_lookup_is_cast_to_a_land() {
        let mut e = engine();
        e.register(FORM_LOOKUP, |_, a| ret(a[2] + 0x100));
        let casts = recorder::<Vec<u32>>();
        let sink = casts.clone();
        e.register_double(RT_DYNAMIC_CAST, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            ret(a[0] + 1)
        });
        let result = e.call(0x0053_4880, &args![0x1000u32, 0xdeadu32, 0x42u32]);
        assert_eq!(result.u32(), 0x143);
        assert_eq!(
            *casts.borrow(),
            vec![vec![0x142, 0, RTTI_TES_FORM, RTTI_TES_OBJECT_LAND, 0]]
        );
    }

    // ---- Copying a land ---------------------------------------------------------

    /// Fills `data`'s blocks with a pattern: block `b` of quadrant `q` gets
    /// the byte `16 * (b + 1) + q`.
    fn fill_blocks(e: &mut Engine, data: Ptr<LoadedLandData>) {
        for (index, (field, size)) in [
            (LoadedLandData::ppVertices.off, 0xd8cu32),
            (LoadedLandData::ppNormals.off, 0xd8c),
            (LoadedLandData::ppColorsA.off, 0x1210),
            (LoadedLandData::ppNormalsSet.off, 0x121),
        ]
        .into_iter()
        .enumerate()
        {
            let array = e.mem.u32(data.addr() + field);
            for quadrant in 0..4u32 {
                let block = e.mem.u32(array + 4 * quadrant);
                let bytes = vec![(16 * (index as u32 + 1) + quadrant) as u8; size as usize];
                e.mem.write(block, &bytes);
            }
        }
        for quadrant in 0..4u32 {
            let percent = e
                .mem
                .u32(data.addr() + LoadedLandData::ppPercentArrays.off + 4 * quadrant);
            let block = e.mem.u32(percent);
            e.mem.write(block, &[0xa0 + quadrant as u8; 0x2420]);
            let layers = e
                .mem
                .u32(data.addr() + LoadedLandData::pQuadTextureArray.off + 4 * quadrant);
            e.mem.write(layers, &[0xb0 + quadrant as u8; 0x18]);
            e.mem.set_u32(
                data.addr() + LoadedLandData::pDefQuadTexture.off + 4 * quadrant,
                0x7000 + quadrant,
            );
        }
    }

    /// Doubles for the unload: no grass map items, no border.
    fn unload_doubles(e: &mut Engine) {
        e.register(SMART_POINTER_RELEASE, |_, _| Ret::default());
        e.register(VECTOR_DESTRUCT_ITERATOR, |_, _| Ret::default());
        e.register(LAND_SET_LOADED, |_, _| Ret::default());
        e.register(LAND_RELEASE_BORDER, |_, _| Ret::default());
        e.register(MAP_REMOVE_ALL, |_, _| Ret::default());
        e.register(HAVOK_REMOVE_REFERENCE, |_, _| Ret::default());
    }

    /// A double for the data allocator that gives its land the arrays.
    fn allocate_data_double(e: &mut Engine) {
        e.register(LAND_ALLOCATE_DATA, |e, a| {
            let (donor, data) = land_with_data(e);
            let _ = donor;
            // The real allocator takes the cell coordinates from the cell.
            e.set(data, LoadedLandData::iCellX, -7);
            e.set(data, LoadedLandData::iCellY, 9);
            e.mem.set_u32(a[0] + 0x28, data.addr());
            Ret::default()
        });
    }

    #[test]
    fn copy_takes_the_blocks_flags_and_extents_of_another_land() {
        let mut e = geometry_engine();
        unload_doubles(&mut e);
        allocate_data_double(&mut e);
        e.register(LAND_IS_LOADED, |_, _| ret(1));
        e.register(LAND_SET_LOADED, |_, _| Ret::default());
        let (source, from) = land_with_data(&mut e);
        fill_blocks(&mut e, from);
        e.set(source, TESObjectLAND::Data, 0x1234);
        e.set(source, TESObjectLAND::pParentCell, Ptr::new(0x6000));
        e.set(from, LoadedLandData::HeightExtentsMin, -5.0);
        e.set(from, LoadedLandData::HeightExtentsMax, 9.0);
        let this = land(&mut e);
        start_log(&mut e);
        e.call(0x0053_48c0, &args![this, source]);
        let log = end_log(&mut e);
        let to = loaded_data(&e, this);
        assert!(!to.is_null());
        assert_ne!(to.addr(), from.addr());
        assert_eq!(e.get(this, TESObjectLAND::Data), 0x1234);
        assert_eq!(e.get(this, TESObjectLAND::pParentCell).addr(), 0x6000);
        assert_eq!(e.get(to, LoadedLandData::HeightExtentsMin), -5.0);
        assert_eq!(e.get(to, LoadedLandData::HeightExtentsMax), 9.0);
        for (index, (field, size)) in [
            (LoadedLandData::ppVertices.off, 0xd8cu32),
            (LoadedLandData::ppNormals.off, 0xd8c),
            (LoadedLandData::ppColorsA.off, 0x1210),
            (LoadedLandData::ppNormalsSet.off, 0x121),
        ]
        .into_iter()
        .enumerate()
        {
            let array = e.mem.u32(to.addr() + field);
            for quadrant in 0..4u32 {
                let block = e.mem.u32(array + 4 * quadrant);
                let bytes = e.mem.bytes(block, size);
                assert!(bytes
                    .iter()
                    .all(|b| *b == (16 * (index as u32 + 1) + quadrant) as u8));
            }
        }
        for quadrant in 0..4u32 {
            let percent = e
                .mem
                .u32(to.addr() + LoadedLandData::ppPercentArrays.off + 4 * quadrant);
            let block = e.mem.u32(percent);
            assert!(e
                .mem
                .bytes(block, 0x2420)
                .iter()
                .all(|b| *b == 0xa0 + quadrant as u8));
            let layers = e
                .mem
                .u32(to.addr() + LoadedLandData::pQuadTextureArray.off + 4 * quadrant);
            assert!(e
                .mem
                .bytes(layers, 0x18)
                .iter()
                .all(|b| *b == 0xb0 + quadrant as u8));
            assert_eq!(
                e.mem
                    .u32(to.addr() + LoadedLandData::pDefQuadTexture.off + 4 * quadrant),
                0x7000 + quadrant
            );
        }
        // The source was loaded, so it is neither loaded nor unloaded here.
        assert_eq!(
            arguments_of(&log, LAND_ALLOCATE_DATA),
            vec![vec![this.addr(), source.addr()]]
        );
        assert_eq!(
            arguments_of(&log, LAND_SET_LOADED),
            vec![vec![this.addr(), 1]]
        );
        assert!(!loaded_data(&e, source).is_null());
    }

    #[test]
    fn copy_loads_and_unloads_a_source_that_was_not_loaded() {
        let mut e = geometry_engine();
        unload_doubles(&mut e);
        allocate_data_double(&mut e);
        e.register(LAND_IS_LOADED, |_, _| ret(0));
        e.register(LOADING_MENU_VISIBLE, |_, _| ret(0));
        let source = land(&mut e);
        let this = land(&mut e);
        start_log(&mut e);
        e.call(0x0053_48c0, &args![this, source]);
        let log = end_log(&mut e);
        // The source got data to copy from (no flags: nothing to read), and
        // lost it again.
        assert_eq!(
            arguments_of(&log, LAND_ALLOCATE_DATA),
            vec![vec![source.addr(), 0], vec![this.addr(), source.addr()]]
        );
        assert!(loaded_data(&e, source).is_null());
        assert!(!loaded_data(&e, this).is_null());
        // A form that is not a land is ignored.
        e.register(RT_DYNAMIC_CAST, |_, _| ret(0));
        let other = land(&mut e);
        start_log(&mut e);
        e.call(0x0053_48c0, &args![other, 0x1234u32]);
        assert_eq!(called(&mut e), vec![RT_DYNAMIC_CAST]);
    }

    // ---- Chunk helpers ------------------------------------------------------------

    #[test]
    fn byte_swaps_cover_the_record_fields() {
        let mut e = engine();
        e.register(SWAP_BYTES_32, |_, _| Ret::default());
        e.register(SWAP_BYTES_16, |_, _| Ret::default());
        start_log(&mut e);
        e.call(0x0053_5a60, &args![0x1000u32]);
        assert_eq!(
            end_log(&mut e),
            vec![
                (SWAP_BYTES_32, vec![0x1000, 0]),
                (SWAP_BYTES_16, vec![0x1006, 0])
            ]
        );
        start_log(&mut e);
        e.call(0x0053_5aa0, &args![0x2000u32]);
        assert_eq!(
            end_log(&mut e),
            vec![
                (SWAP_BYTES_16, vec![0x2000, 0]),
                (SWAP_BYTES_32, vec![0x2004, 0])
            ]
        );
    }

    #[test]
    fn default_texture_getter_reads_the_global() {
        let mut e = engine();
        e.set_global(DEFAULT_LAND_TEXTURE, 0x9999u32);
        assert_eq!(e.call(0x0053_5ae0, &[]).u32(), 0x9999);
    }

    #[test]
    fn quadrant_node_comes_from_the_cell() {
        let mut e = engine();
        e.register(CELL_GET_NODE, |_, a| ret(a[0].wrapping_add(a[1])));
        let this = land(&mut e);
        assert_eq!(e.call(0x0053_5af0, &args![this, 1u32]).u32(), 0);
        e.set(this, TESObjectLAND::pParentCell, Ptr::new(0x8000));
        assert_eq!(e.call(0x0053_5af0, &args![this, 3u32]).u32(), 0x8003);
        assert_eq!(e.call(0x0053_5af0, &args![this, 4u32]).u32(), 0);
        assert_eq!(e.call(0x0053_5af0, &args![this, -1i32]).u32(), 0x7fff);
    }

    #[test]
    fn world_offset_is_the_cell_centre_at_base_height() {
        let mut e = engine();
        e.register(LAND_BASE_HEIGHT, |_, _| float_ret(33.5));
        let this = land(&mut e);
        let data: Ptr<LoadedLandData> = e.new_object();
        e.set(data, LoadedLandData::iCellX, -2);
        e.set(data, LoadedLandData::iCellY, 3);
        e.set(this, TESObjectLAND::pLoadedData, data.cast());
        let out = e.mem.alloc(12);
        let result = e.call(0x0053_5b30, &args![this, out, 77u32]);
        assert_eq!(result.u32(), out);
        assert_eq!(
            floats(&e, out, 3),
            vec![-8192.0 + 2048.0, 12288.0 + 2048.0, 33.5]
        );
    }

    // ---- Loading ---------------------------------------------------------------

    /// A plugin file scripted as a list of chunks. The doubles answer the
    /// `TESFile` calls: the record type, the current chunk's tag, size and
    /// data, the move to the next chunk, the active flag and the name.
    struct ScriptedFile {
        chunks: Vec<(u32, Vec<u8>)>,
        current: usize,
        /// The sizes the loader asked `GetChunkData` for.
        requested: Vec<u32>,
        record_type: u8,
        swap: bool,
    }

    fn script_file(
        e: &mut Engine,
        chunks: Vec<(u32, Vec<u8>)>,
    ) -> (u32, Rc<RefCell<ScriptedFile>>) {
        let file = e.mem.alloc(16);
        let script = Rc::new(RefCell::new(ScriptedFile {
            chunks,
            current: 0,
            requested: vec![],
            record_type: b'B',
            swap: false,
        }));
        let state = script.clone();
        e.register_double(FILE_GET_TES_FORM, move |_, _| {
            ret(state.borrow().record_type as u32)
        });
        let state = script.clone();
        e.register_double(FILE_GET_CHUNK, move |_, _| {
            let s = state.borrow();
            ret(s.chunks[s.current].0)
        });
        let state = script.clone();
        e.register_double(FILE_NEXT_CHUNK, move |_, _| {
            let mut s = state.borrow_mut();
            s.current += 1;
            ret((s.current < s.chunks.len()) as u32)
        });
        let state = script.clone();
        e.register_double(FILE_GET_CHUNK_SIZE, move |_, _| {
            let s = state.borrow();
            ret(s.chunks[s.current].1.len() as u32)
        });
        let state = script.clone();
        e.register_double(FILE_GET_CHUNK_DATA, move |e, a| {
            let mut s = state.borrow_mut();
            s.requested.push(a[2]);
            let bytes = s.chunks[s.current].1.clone();
            e.mem.write(a[1], &bytes);
            Ret::default()
        });
        let state = script.clone();
        e.register_double(FILE_NEEDS_SWAP, move |_, _| ret(state.borrow().swap as u32));
        e.register(FILE_GET_ACTIVE, |_, _| ret(1));
        e.register(FILE_GET_NAME, |e, _| {
            let name = e.mem.alloc(16);
            e.mem.set_cstr(name, b"Fallout.esm");
            ret(name)
        });
        (file, script)
    }

    /// A `VHGT` chunk: the base height and 0x441 signed steps, all zero
    /// except the `(index, step)` pairs given.
    fn height_chunk(base: f32, steps: &[(usize, i8)]) -> Vec<u8> {
        let mut bytes = base.to_le_bytes().to_vec();
        bytes.resize(4 + CHUNK_VERTICES, 0);
        for (index, step) in steps {
            bytes[4 + index] = *step as u8;
        }
        bytes.resize(0x448, 0);
        bytes
    }

    /// A `VNML` or `VCLR` chunk of 33 x 33 triples, zero except the
    /// `(grid index, triple)` pairs given.
    fn triple_chunk(entries: &[(usize, [u8; 3])]) -> Vec<u8> {
        let mut bytes = vec![0u8; 0xcc3];
        for (index, triple) in entries {
            bytes[3 * index..3 * index + 3].copy_from_slice(triple);
        }
        bytes
    }

    fn record(form_id: u32, quadrant: u8, layer: u16) -> Vec<u8> {
        let mut bytes = form_id.to_le_bytes().to_vec();
        bytes.push(quadrant);
        bytes.push(0);
        bytes.extend_from_slice(&layer.to_le_bytes());
        bytes
    }

    fn vertex_texture_chunk(entries: &[(u16, f32)]) -> Vec<u8> {
        let mut bytes = vec![];
        for (position, opacity) in entries {
            bytes.extend_from_slice(&position.to_le_bytes());
            bytes.extend_from_slice(&[0, 0]);
            bytes.extend_from_slice(&opacity.to_le_bytes());
        }
        bytes
    }

    /// The doubles the chunk readers call.
    fn loader_engine() -> Engine {
        let mut e = geometry_engine();
        allocate_data_double(&mut e);
        unload_doubles(&mut e);
        e.set_global(X_OFFSETS, -2048.0f32);
        e.set_global(X_OFFSETS + 4, 0.0f32);
        e.set_global(X_OFFSETS + 8, -2048.0f32);
        e.set_global(X_OFFSETS + 12, 0.0f32);
        e.set_global(Y_OFFSETS, -2048.0f32);
        e.set_global(Y_OFFSETS + 4, -2048.0f32);
        e.set_global(Y_OFFSETS + 8, 0.0f32);
        e.set_global(Y_OFFSETS + 12, 0.0f32);
        // `FISTP`: round to nearest.
        e.register(FLOAT_TO_INT, |_, a| {
            ret(f32::from_bits(a[0]).round() as i32 as u32)
        });
        e.register(NI_POINT3_NORMALIZE, |_, _| Ret::default());
        e.register(LAND_SET_LOADED, |_, _| Ret::default());
        e.register(LAND_FLUSH_WARNINGS, |_, _| Ret::default());
        e.register(LOADING_MASTER_FILE, |_, _| ret(0));
        e.register(FORM_LOAD, |_, _| Ret::default());
        e.register(CELL_GET_DATA_X, |_, _| ret(-7i32 as u32));
        e.register(CELL_GET_DATA_Y, |_, _| ret(9));
        e.register(LOG_MESSAGE, |_, _| Ret::default());
        e.register(HAVOK_ADD_REFERENCE, |_, _| Ret::default());
        e.register(FORM_ADD_COMPILE_INDEX, |_, _| Ret::default());
        e.register(QUEUED_FILE_CONSTRUCT, |_, a| ret(a[0]));
        e.register(QUEUED_FILE_POINTER_SET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(LAND_TEXTURE_QUEUE, |_, _| Ret::default());
        e.register(LAND_SET_VERTEX_OPACITY, |_, _| Ret::default());
        e.register(LAND_BUILD_MOPP, |e, a| {
            e.mem.set_u32(a[3], 0x5555);
            Ret::default()
        });
        e
    }

    /// The quadrant vertex `k` of the block of `quadrant` of `array`.
    fn block_vertex(e: &Engine, array: Ptr, quadrant: u32, k: u32, size: u32) -> u32 {
        element(e, array, quadrant) + size * k
    }

    #[test]
    fn load_reads_geometry_and_textures_from_the_record() {
        let mut e = loader_engine();
        e.set_global(DEFAULT_LAND_TEXTURE, 0x8888u32);
        // `FindForm` is unused here: the form ids resolve through lookups.
        e.register(FORM_LOOKUP_BY_ID, |_, a| {
            ret(if a[0] == 0x1234 { 0x2000 } else { 0 })
        });
        let queued = recorder::<Vec<u32>>();
        let sink = queued.clone();
        e.register_double(LAND_TEXTURE_QUEUE, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let opacities = recorder::<Vec<u32>>();
        let sink = opacities.clone();
        e.register_double(LAND_SET_VERTEX_OPACITY, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let messages = recorder::<Vec<u32>>();
        let sink = messages.clone();
        e.register_double(LOG_MESSAGE, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let mopp = recorder::<Vec<u32>>();
        let sink = mopp.clone();
        e.register_double(LAND_BUILD_MOPP, move |e, a| {
            sink.borrow_mut().push(a.to_vec());
            e.mem.set_u32(a[3], 0x5555);
            Ret::default()
        });
        let chunks = vec![
            (CHUNK_DATA, 7u32.to_le_bytes().to_vec()),
            // Row 5 from column 3 on is 4 higher than the rest.
            (
                CHUNK_HEIGHTS,
                height_chunk(100.0, &[(0, 2), (5 * 33 + 3, 4)]),
            ),
            // The normal at grid 16 (quadrant 1, vertex 0) is (127, 0, -127).
            (CHUNK_NORMALS, triple_chunk(&[(16, [127, 0, 0x81])])),
            // The colour at grid 529 (quadrant 2, vertex 1).
            (CHUNK_COLORS, triple_chunk(&[(529, [255, 0, 51])])),
            (CHUNK_MOPP, vec![1, 2, 3, 4, 5, 6, 7, 8]),
            // The base texture of quadrant 3 is unknown.
            (CHUNK_BASE_TEXTURE, record(0x4321, 3, 0)),
            // A layer of quadrant 2 with its vertex opacities.
            (CHUNK_ADDITIONAL_TEXTURE, record(0x1234, 2, 3)),
            (
                CHUNK_VERTEX_TEXTURE,
                vertex_texture_chunk(&[(7, 50.0), (8, 0.75)]),
            ),
            // A layer without texture (the default one) with an invalid
            // index, and vertex data of the wrong size.
            (CHUNK_ADDITIONAL_TEXTURE, record(0, 1, 9)),
            (CHUNK_VERTEX_TEXTURE, vec![0; 7]),
            (tag(b"ZZZZ"), vec![]),
        ];
        let (file, script) = script_file(&mut e, chunks);
        let this = land(&mut e);
        start_log(&mut e);
        let loaded = e.call(0x0053_4ad0, &args![this, file]);
        assert_eq!(loaded.u32() & 0xff, 1);
        let log = end_log(&mut e);
        let data = loaded_data(&e, this);
        // DATA replaced the flags (the world space bit stays clear), and the
        // MOPP chunk set bit 0x800.
        assert_eq!(e.get(this, TESObjectLAND::Data), 0x807);
        assert_eq!(*script.borrow().requested.first().unwrap(), 4);
        // The heights: 102 everywhere, 106 in row 5 from column 3; each
        // rounded and times 8, relative to their mean.
        assert_eq!(e.get(data, LoadedLandData::HeightExtentsMin), 816.0);
        assert_eq!(e.get(data, LoadedLandData::HeightExtentsMax), 848.0);
        assert_eq!(e.get(data, LoadedLandData::fBaseHeight), 832.0);
        let vertices = e.get(data, LoadedLandData::ppVertices);
        let vertex = block_vertex(&e, vertices, 0, 5 * 17 + 3, 12);
        assert_eq!(floats(&e, vertex, 3), vec![-1664.0, -1408.0, 16.0]);
        let vertex = block_vertex(&e, vertices, 0, 0, 12);
        assert_eq!(floats(&e, vertex, 3), vec![-2048.0, -2048.0, -16.0]);
        let vertex = block_vertex(&e, vertices, 3, 0x120, 12);
        assert_eq!(floats(&e, vertex, 3), vec![2048.0, 2048.0, -16.0]);
        // Normals and colours.
        let normals = e.get(data, LoadedLandData::ppNormals);
        let normal = block_vertex(&e, normals, 1, 0, 12);
        assert_eq!(floats(&e, normal, 3), vec![1.0, 0.0, -1.0]);
        let colors = e.get(data, LoadedLandData::ppColorsA);
        let color = block_vertex(&e, colors, 2, 1, 0x10);
        assert_eq!(floats(&e, color, 4), vec![1.0, 0.0, byte_fraction(51), 1.0]);
        // The MOPP code was built from the chunk and referenced.
        assert_eq!(e.get(data, LoadedLandData::pMoppCode).addr(), 0x5555);
        assert_eq!(mopp.borrow().len(), 1);
        assert_eq!(mopp.borrow()[0][2], 8);
        // The textures: the unknown base texture is null, the found layer
        // texture is in quadrant 2 layer 3, the clamped one in quadrant 1
        // layer 5 holds the default texture.
        assert_eq!(
            e.mem
                .u32(data.addr() + LoadedLandData::pDefQuadTexture.off + 12),
            0
        );
        let layers = e
            .mem
            .u32(data.addr() + LoadedLandData::pQuadTextureArray.off + 8);
        assert_eq!(e.mem.u32(layers + 12), 0x2000);
        let layers = e
            .mem
            .u32(data.addr() + LoadedLandData::pQuadTextureArray.off + 4);
        assert_eq!(e.mem.u32(layers + 20), 0x8888);
        let queued_file = e.mem.u32(this.addr() + 0x24);
        assert_ne!(queued_file, 0);
        assert_eq!(
            *queued.borrow(),
            vec![vec![0x2000, 5, queued_file], vec![0x8888, 5, queued_file]]
        );
        // The vertex opacities of the layer: 50 is divided by 100.
        assert_eq!(
            *opacities.borrow(),
            vec![
                vec![this.addr(), 2, 7, 3, 0.5f32.to_bits()],
                vec![this.addr(), 2, 8, 3, 0.75f32.to_bits()]
            ]
        );
        // Messages: base texture missing, clamped layer, bad vertex data.
        let (x, y) = (-7i32 as u32, 9u32);
        let messages = messages.borrow();
        assert_eq!(messages.len(), 3);
        assert_eq!(
            messages[0],
            vec![MASTER_FILE_MESSAGES.missing_base, x, y, 0x4321, 3]
        );
        assert_eq!(
            messages[1],
            vec![MASTER_FILE_MESSAGES.clamped_layer, x, y, 9, 1]
        );
        assert_eq!(
            messages[2][..3],
            [MASTER_FILE_MESSAGES.unrecognized_data, x, y]
        );
        assert_eq!(read_cstring(&e, messages[2][3]), "Fallout.esm");
        assert_eq!(
            arguments_of(&log, LAND_FLUSH_WARNINGS),
            vec![vec![this.addr()]]
        );
        // The land counts as loaded with its geometry.
        assert_eq!(
            arguments_of(&log, LAND_SET_LOADED).last().unwrap(),
            &vec![this.addr(), 1]
        );
        assert_eq!(arguments_of(&log, NI_POINT3_NORMALIZE).len(), 4 * 0x121);
    }

    #[test]
    fn load_ignores_records_of_other_types() {
        let mut e = loader_engine();
        let (file, script) = script_file(&mut e, vec![(CHUNK_DATA, vec![0; 4])]);
        script.borrow_mut().record_type = b'C';
        let this = land(&mut e);
        start_log(&mut e);
        let result = e.call(0x0053_4ad0, &args![this, file]);
        assert_eq!(result.u32() & 0xff, 0);
        let log = end_log(&mut e);
        assert_eq!(
            log.iter().map(|(a, _)| *a).collect::<Vec<_>>(),
            vec![SCOPE_GUARD_ENTER, FILE_GET_TES_FORM, SCOPE_GUARD_LEAVE]
        );
        assert_eq!(log[0].1[4], 0x2c2);
    }

    #[test]
    fn load_skips_geometry_for_a_land_of_the_world_space_file() {
        let mut e = loader_engine();
        let (file, _) = script_file(
            &mut e,
            vec![
                (CHUNK_DATA, (0x7u32).to_le_bytes().to_vec()),
                (CHUNK_HEIGHTS, height_chunk(0.0, &[])),
                (CHUNK_BASE_TEXTURE, record(0x1, 0, 0)[..8].to_vec()),
            ],
        );
        let this = land(&mut e);
        e.set(this, TESObjectLAND::Data, 0x400);
        start_log(&mut e);
        e.call(0x0053_4ad0, &args![this, file]);
        let log = end_log(&mut e);
        // Flag 0x400 is kept by DATA; no data was allocated and nothing
        // else was read.
        assert_eq!(e.get(this, TESObjectLAND::Data), 0x407 | 0x400);
        assert!(arguments_of(&log, LAND_ALLOCATE_DATA).is_empty());
        assert_eq!(arguments_of(&log, FILE_GET_CHUNK_DATA).len(), 1);
        assert_eq!(
            arguments_of(&log, LAND_SET_LOADED).last().unwrap(),
            &vec![this.addr(), 0]
        );
    }

    #[test]
    fn load_swaps_the_chunks_of_a_big_endian_file() {
        let mut e = loader_engine();
        e.register(SWAP_WORD, |e, a| {
            let bytes = e.mem.u32(a[0]).swap_bytes();
            e.mem.set_u32(a[0], bytes);
            Ret::default()
        });
        e.register(SWAP_BYTES_32, |e, a| {
            let value = e.mem.u32(a[0]).swap_bytes();
            e.mem.set_u32(a[0], value);
            Ret::default()
        });
        e.register(SWAP_BYTES_16, |e, a| {
            let value = e.mem.u16(a[0]).swap_bytes();
            e.mem.set_u16(a[0], value);
            Ret::default()
        });
        e.register(FORM_LOOKUP_BY_ID, |_, a| ret(a[0] + 1));
        let mut flags = 1u32.to_be_bytes().to_vec();
        flags.truncate(4);
        let mut heights = 100.0f32.to_be_bytes().to_vec();
        heights.resize(0x448, 0);
        let mut layer = 0x0102_0304u32.to_be_bytes().to_vec();
        layer.extend_from_slice(&[1, 0]);
        layer.extend_from_slice(&2u16.to_be_bytes());
        let (file, script) = script_file(
            &mut e,
            vec![
                (CHUNK_DATA, flags),
                (CHUNK_HEIGHTS, heights),
                (CHUNK_ADDITIONAL_TEXTURE, layer),
            ],
        );
        script.borrow_mut().swap = true;
        let this = land(&mut e);
        e.call(0x0053_4ad0, &args![this, file]);
        let data = loaded_data(&e, this);
        // The flags were swapped, and so was the base height (all heights are
        // 100, which is 800 after the scaling).
        assert_eq!(e.get(this, TESObjectLAND::Data), 1);
        assert_eq!(e.get(data, LoadedLandData::fBaseHeight), 800.0);
        // The layer record: form id swapped, layer swapped; texture 1 + id.
        let layers = e
            .mem
            .u32(data.addr() + LoadedLandData::pQuadTextureArray.off + 4);
        assert_eq!(e.mem.u32(layers + 8), 0x0102_0305);
    }

    #[test]
    fn load_stores_the_vertices_of_each_quadrant_on_its_own_block() {
        let mut e = loader_engine();
        // Steps that make the rows differ: row r is 10 * r higher than the
        // first (each row starts from the first height of the previous one,
        // so the row's first step is the difference).
        let steps: Vec<(usize, i8)> = (1..33).map(|row| (row * 33, 10)).collect();
        let (file, _) = script_file(
            &mut e,
            vec![
                (CHUNK_DATA, 1u32.to_le_bytes().to_vec()),
                (CHUNK_HEIGHTS, height_chunk(0.0, &steps)),
            ],
        );
        let this = land(&mut e);
        e.call(0x0053_4ad0, &args![this, file]);
        let data = loaded_data(&e, this);
        let vertices = e.get(data, LoadedLandData::ppVertices);
        // Row r (0..33) is at height 10 r, so z = 80 r minus the mean
        // (80 * 16 = 1280 for the full grid: min 0, max 2560).
        for (quadrant, row, column) in [(0u32, 3u32, 4u32), (1, 3, 4), (2, 3, 4), (3, 16, 16)] {
            let k = row * 17 + column;
            let vertex = block_vertex(&e, vertices, quadrant, k, 12);
            let grid_row = (quadrant / 2) * 16 + row;
            assert_eq!(e.mem.f32(vertex + 8), 80.0 * grid_row as f32 - 1280.0);
        }
    }

    // ---- LoadVerticesIntoArrays and LoadVertices ---------------------------------

    /// The doubles around the file lookups `LoadVerticesIntoArrays` makes.
    fn file_lookup_engine(file: u32) -> Engine {
        let mut e = loader_engine();
        e.register(LAND_IS_LOADED, |_, _| ret(0));
        e.register(FORM_GET_FILE, |_, _| ret(0x7700));
        e.register_double(FILE_GET_THREAD_SAFE_FILE, move |_, _| ret(file));
        e.register(FILE_FIND_FORM, |_, _| ret(1));
        e.register(FILE_RECORD_OFFSET, |_, _| ret(0x1000));
        e.register(FILE_SET_OFFSET, |_, _| ret(1));
        e.register(DISABLE_WARNINGS, |_, _| Ret::default());
        e.register(CELL_GET_WORLD_SPACE, |_, _| ret(0x4000));
        e
    }

    #[test]
    fn arrays_loader_returns_early_when_there_is_nothing_to_read() {
        let mut e = file_lookup_engine(0);
        // Already loaded.
        e.register(LAND_IS_LOADED, |_, _| ret(1));
        let this = land(&mut e);
        start_log(&mut e);
        assert_eq!(e.call(0x0053_5d00, &args![this]).u32() & 0xff, 1);
        assert_eq!(called(&mut e), vec![LAND_IS_LOADED]);
        // No flags: no file is looked up.
        e.register(LAND_IS_LOADED, |_, _| ret(0));
        start_log(&mut e);
        assert_eq!(e.call(0x0053_5d00, &args![this]).u32() & 0xff, 1);
        assert_eq!(called(&mut e), vec![LAND_IS_LOADED]);
        // A land of the world space's land file without a world space.
        e.register(CELL_GET_WORLD_SPACE, |_, _| ret(0));
        e.set(this, TESObjectLAND::Data, 0x400);
        e.set(this, TESObjectLAND::pParentCell, Ptr::new(0x6000));
        assert_eq!(e.call(0x0053_5d00, &args![this]).u32() & 0xff, 1);
        // The world space has no data for the cell.
        e.register(CELL_GET_WORLD_SPACE, |_, _| ret(0x4000));
        e.register(WORLD_SPACE_FIND_LAND_DATA, |_, _| ret(0));
        assert_eq!(e.call(0x0053_5d00, &args![this]).u32() & 0xff, 1);
    }

    #[test]
    fn arrays_loader_reports_a_form_that_is_missing_from_its_file() {
        let mut e = file_lookup_engine(0);
        let (this, _) = land_with_data(&mut e);
        e.set(this, TESObjectLAND::Data, 1);
        e.set(this, TESObjectLAND::pParentCell, Ptr::new(0x6000));
        e.mem.set_u32(this.addr() + 0xc, 0xabc);
        let messages = recorder::<Vec<u32>>();
        let sink = messages.clone();
        e.register_double(LOG_MESSAGE, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        // The form's file has no thread-safe copy: the name is "UNKNOWN".
        e.register(FILE_FIND_FORM, |_, _| ret(0));
        start_log(&mut e);
        assert_eq!(e.call(0x0053_5d00, &args![this]).u32() & 0xff, 0);
        let log = end_log(&mut e);
        let addresses: Vec<u32> = log.iter().map(|(a, _)| *a).collect();
        assert_eq!(
            addresses,
            vec![
                LAND_IS_LOADED,
                FORM_GET_FILE,
                FILE_GET_THREAD_SAFE_FILE,
                FILE_FIND_FORM,
                DISABLE_WARNINGS,
                PARENT_CELL,
                CELL_GET_DATA_Y,
                PARENT_CELL,
                CELL_GET_DATA_X,
                FORM_ID,
                LOG_MESSAGE,
                DISABLE_WARNINGS
            ]
        );
        assert_eq!(log[1].1, vec![this.addr(), 0xffff_ffff]);
        assert_eq!(log[4].1, vec![0]);
        assert_eq!(log[11].1, vec![1]);
        assert_eq!(
            *messages.borrow(),
            vec![vec![
                LAND_LOAD_FAILED,
                0xabc,
                -7i32 as u32,
                9,
                UNKNOWN_FILE_NAME
            ]]
        );
        // A file that is not a land record names the file.
        let (file, script) = script_file(&mut e, vec![(CHUNK_DATA, vec![0; 4])]);
        e.register_double(FILE_GET_THREAD_SAFE_FILE, move |_, _| ret(file));
        e.register(FILE_FIND_FORM, |_, _| ret(1));
        script.borrow_mut().record_type = b'X';
        messages.borrow_mut().clear();
        assert_eq!(e.call(0x0053_5d00, &args![this]).u32() & 0xff, 0);
        let messages = messages.borrow();
        assert_eq!(messages.len(), 1);
        assert_eq!(read_cstring(&e, messages[0][4]), "Fallout.esm");
    }

    #[test]
    fn arrays_loader_stops_when_the_offset_cannot_be_set() {
        let mut e = file_lookup_engine(0);
        let (file, _) = script_file(&mut e, vec![(CHUNK_DATA, vec![0; 4])]);
        e.register_double(FILE_GET_THREAD_SAFE_FILE, move |_, _| ret(file));
        let seen = recorder::<Vec<u32>>();
        let sink = seen.clone();
        e.register_double(FILE_SET_OFFSET, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            ret(0)
        });
        let (this, _) = land_with_data(&mut e);
        e.set(this, TESObjectLAND::Data, 4);
        assert_eq!(e.call(0x0053_5d00, &args![this]).u32() & 0xff, 0);
        assert_eq!(*seen.borrow(), vec![vec![file, 0x1000]]);
    }

    #[test]
    fn arrays_loader_reads_the_chunks_of_a_world_space_land() {
        let mut e = loader_engine();
        e.register(FORM_LOOKUP_BY_ID, |_, a| {
            ret(if a[0] == 0x1234 { 0x2000 } else { 0 })
        });
        let chunks = vec![
            // Low bits 5: heights and textures.
            (CHUNK_DATA, 0xffff_fff5u32.to_le_bytes().to_vec()),
            (CHUNK_MOPP, vec![9; 4]),
            (CHUNK_HEIGHTS, height_chunk(100.0, &[(0, 2)])),
            // Colours are not in the flags: not read.
            (CHUNK_COLORS, triple_chunk(&[(0, [255, 255, 255])])),
            (CHUNK_BASE_TEXTURE, record(0x4321, 0, 0)),
            (CHUNK_ADDITIONAL_TEXTURE, record(0x1234, 1, 2)),
            (CHUNK_VERTEX_TEXTURE, vertex_texture_chunk(&[(3, 50.0)])),
            (tag(b"ZZZZ"), vec![]),
        ];
        let (file, script) = script_file(&mut e, chunks);
        e.register(LAND_IS_LOADED, |_, _| ret(0));
        e.register(CELL_GET_WORLD_SPACE, |_, _| ret(0x4000));
        e.register(FILE_SET_OFFSET, |_, _| ret(1));
        e.register(DISABLE_WARNINGS, |_, _| Ret::default());
        let found = recorder::<Vec<u32>>();
        let sink = found.clone();
        e.register_double(WORLD_SPACE_FIND_LAND_DATA, move |e, a| {
            sink.borrow_mut().push(a.to_vec());
            e.mem.set_u32(a[3], file);
            e.mem.set_u32(a[4], 0x1000);
            ret(1)
        });
        let opacities = recorder::<Vec<u32>>();
        let sink = opacities.clone();
        e.register_double(LAND_SET_VERTEX_OPACITY, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let messages = recorder::<Vec<u32>>();
        let sink = messages.clone();
        e.register_double(LOG_MESSAGE, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let mopp = recorder::<Vec<u32>>();
        let sink = mopp.clone();
        e.register_double(LAND_BUILD_MOPP, move |e, a| {
            sink.borrow_mut().push(a.to_vec());
            e.mem.set_u32(a[3], 0x5555);
            Ret::default()
        });
        let (this, data) = land_with_data(&mut e);
        e.set(data, LoadedLandData::iCellX, -7);
        e.set(data, LoadedLandData::iCellY, 9);
        e.set(this, TESObjectLAND::Data, 0x410);
        e.set(this, TESObjectLAND::pParentCell, Ptr::new(0x6000));
        start_log(&mut e);
        assert_eq!(e.call(0x0053_5d00, &args![this]).u32() & 0xff, 1);
        let log = end_log(&mut e);
        // The cell's land data was looked up with its coordinates.
        assert_eq!(found.borrow().len(), 1);
        assert_eq!(found.borrow()[0][..3], [0x4000, -7i32 as u32, 9]);
        assert_eq!(
            arguments_of(&log, FILE_SET_OFFSET),
            vec![vec![file, 0x1000]]
        );
        // DATA only merges the low three bits; the MOPP chunk sets 0x800.
        assert_eq!(e.get(this, TESObjectLAND::Data), 0xc15);
        // The MOPP code was read without a flag.
        assert_eq!(mopp.borrow().len(), 1);
        assert_eq!(e.get(data, LoadedLandData::pMoppCode).addr(), 0x5555);
        // Heights were read (102 everywhere, 816 after the scaling).
        assert_eq!(e.get(data, LoadedLandData::fBaseHeight), 816.0);
        // The colour chunk was skipped: the data requests of the file were
        // DATA (4), MPCD (4), VHGT (0x448), BTXT (8), ATXT (8), VTXT (8).
        assert_eq!(script.borrow().requested, vec![4, 4, 0x448, 8, 8, 8]);
        // The layer texture of quadrant 1, layer 2, and its opacity (the
        // value is not divided by 100).
        let layers = e
            .mem
            .u32(data.addr() + LoadedLandData::pQuadTextureArray.off + 4);
        assert_eq!(e.mem.u32(layers + 8), 0x2000);
        assert_eq!(
            *opacities.borrow(),
            vec![vec![this.addr(), 1, 3, 2, 50.0f32.to_bits()]]
        );
        // The missing base texture is logged with the "TEXTURES" message and
        // flushes the warnings; heights without normals are logged too.
        let messages = messages.borrow();
        assert_eq!(messages.len(), 2);
        assert_eq!(
            messages[0],
            vec![CELL_MESSAGES.missing_base, -7i32 as u32, 9, 0x4321, 0]
        );
        assert_eq!(messages[1][..3], [NO_NORMAL_DATA, -7i32 as u32, 9]);
        assert_eq!(read_cstring(&e, messages[1][3]), "Fallout.esm");
        assert_eq!(
            arguments_of(&log, LAND_FLUSH_WARNINGS),
            vec![vec![this.addr()]]
        );
        // Unlike `Load`, this loader does not mark the land loaded.
        assert!(arguments_of(&log, LAND_SET_LOADED).is_empty());
    }

    #[test]
    fn load_vertices_builds_meshes_and_reads_what_is_needed() {
        let mut e = loader_engine();
        e.register(LOADING_MENU_VISIBLE, |_, _| ret(0));
        e.register(LAND_IS_LOADED, |_, _| ret(0));
        e.register(FORM_GET_FILE, |_, _| ret(0));
        e.register(CELL_GET_3D, |_, _| ret(0));
        // A land with no flags: data is allocated, nothing is read.
        let this = land(&mut e);
        e.set(this, TESObjectLAND::pParentCell, Ptr::new(0x6000));
        start_log(&mut e);
        let result = e.call(0x0053_5b90, &args![this, 1u32]);
        assert_eq!(result.u32() & 0xff, 1);
        let log = end_log(&mut e);
        let addresses: Vec<u32> = log.iter().map(|(a, _)| *a).collect();
        assert_eq!(
            addresses,
            vec![
                SCOPE_GUARD_ENTER,
                LOADING_MENU_VISIBLE,
                LAND_IS_LOADED,
                LAND_ALLOCATE_DATA,
                CELL_GET_3D,
                SCOPE_GUARD_LEAVE
            ]
        );
        assert_eq!(log[0].1[4], 0x455);
        assert_eq!(log[3].1, vec![this.addr(), 0]);
        // Without the mesh request nothing is built.
        start_log(&mut e);
        e.call(0x0053_5b90, &args![this, 0u32]);
        assert!(!called(&mut e).contains(&CELL_GET_3D));
        // With a land whose record has heights and a file, the arrays are
        // read (here the world space has no data: nothing to read) and the
        // land is marked loaded.
        e.register(FORM_GET_FILE, |_, _| ret(0x7700));
        e.set(this, TESObjectLAND::Data, 1);
        e.register(LAND_SET_LOADED, |_, _| Ret::default());
        e.register(FILE_GET_THREAD_SAFE_FILE, |_, _| ret(0));
        e.register(FILE_FIND_FORM, |_, _| ret(0));
        e.register(DISABLE_WARNINGS, |_, _| Ret::default());
        e.register(CELL_GET_DATA_X, |_, _| ret(1));
        e.register(CELL_GET_DATA_Y, |_, _| ret(2));
        start_log(&mut e);
        let result = e.call(0x0053_5b90, &args![this, 1u32]);
        assert_eq!(result.u32() & 0xff, 0, "the arrays loader failed");
        let log = end_log(&mut e);
        let addresses: Vec<u32> = log.iter().map(|(a, _)| *a).collect();
        assert!(addresses.contains(&FILE_FIND_FORM));
        assert_eq!(
            arguments_of(&log, LAND_SET_LOADED),
            vec![vec![this.addr(), 1]]
        );
        assert!(
            addresses.contains(&CELL_GET_3D),
            "the meshes are still built"
        );
    }

    #[test]
    fn load_vertices_of_a_loaded_land_only_builds_missing_meshes() {
        let mut e = loader_engine();
        e.register(LOADING_MENU_VISIBLE, |_, _| ret(0));
        e.register(LAND_IS_LOADED, |_, _| ret(1));
        e.register(CELL_GET_3D, |_, _| ret(0));
        let (this, data) = land_with_data(&mut e);
        e.set(this, TESObjectLAND::pParentCell, Ptr::new(0x6000));
        start_log(&mut e);
        assert_eq!(e.call(0x0053_5b90, &args![this, 1u32]).u32() & 0xff, 1);
        assert!(called(&mut e).contains(&CELL_GET_3D));
        // The meshes exist already.
        e.set(data, LoadedLandData::ppMesh, Ptr::new(0x1234));
        start_log(&mut e);
        e.call(0x0053_5b90, &args![this, 1u32]);
        assert!(!called(&mut e).contains(&CELL_GET_3D));
        // The loading menu forbids building them.
        e.set(data, LoadedLandData::ppMesh, Ptr::NULL);
        e.register(LOADING_MENU_VISIBLE, |_, _| ret(1));
        start_log(&mut e);
        e.call(0x0053_5b90, &args![this, 1u32]);
        assert!(!called(&mut e).contains(&CELL_GET_3D));
    }

    #[test]
    fn load_vertices_reads_a_world_space_land_through_the_arrays_loader() {
        let mut e = loader_engine();
        e.register(LOADING_MENU_VISIBLE, |_, _| ret(0));
        e.register(LAND_IS_LOADED, |_, _| ret(0));
        e.register(CELL_GET_WORLD_SPACE, |_, _| ret(0));
        e.register(CELL_GET_3D, |_, _| ret(0));
        let this = land(&mut e);
        e.set(this, TESObjectLAND::Data, 0x400);
        e.set(this, TESObjectLAND::pParentCell, Ptr::new(0x6000));
        start_log(&mut e);
        assert_eq!(e.call(0x0053_5b90, &args![this, 1u32]).u32() & 0xff, 1);
        let addresses = called(&mut e);
        assert_eq!(
            addresses,
            vec![
                SCOPE_GUARD_ENTER,
                LOADING_MENU_VISIBLE,
                LAND_IS_LOADED,
                LAND_ALLOCATE_DATA,
                LAND_IS_LOADED,
                CELL_GET_WORLD_SPACE,
                LAND_SET_LOADED,
                CELL_GET_3D,
                SCOPE_GUARD_LEAVE
            ]
        );
    }

    // ---- Unloading ----------------------------------------------------------------

    #[test]
    fn unload_frees_every_block_and_the_data() {
        let mut e = loader_engine();
        e.register(HAVOK_REMOVE_REFERENCE, |_, _| Ret::default());
        let (this, data) = land_with_data(&mut e);
        e.set(data, LoadedLandData::pMoppCode, Ptr::new(0x5555));
        // The grass map of quadrant 1 has two items of 16 pointers; the
        // first pointer of each item is a live block.
        let items: Vec<(u32, u32)> = (0..2)
            .map(|_| {
                let item = e.mem.alloc(64);
                let pointer = e.mem.alloc(8);
                e.mem.set_u32(item + 8, pointer);
                (item, pointer)
            })
            .collect();
        let map = data.addr() + GRASS_MAP_OFFSET + 0x10;
        e.mem.set_u32(map + 0xc, 2);
        e.register(MAP_FIRST_POSITION, |_, _| ret(1));
        let walk = items.clone();
        e.register_double(MAP_GET_NEXT, move |e, a| {
            // `a` = map, &position, &key, &value.
            let position = e.mem.u32(a[1]);
            e.mem.set_u32(a[3], walk[position as usize - 1].0);
            let next = if position < 2 { position + 1 } else { 0 };
            e.mem.set_u32(a[1], next);
            Ret::default()
        });
        let removed = recorder::<u32>();
        let sink = removed.clone();
        e.register_double(MAP_REMOVE_ALL, move |_, a| {
            sink.borrow_mut().push(a[0]);
            Ret::default()
        });
        let vertices_array = e.get(data, LoadedLandData::ppVertices);
        let vertices_block = element(&e, vertices_array, 2);
        let percent = e
            .mem
            .u32(data.addr() + LoadedLandData::ppPercentArrays.off + 12);
        let percent_block = e.mem.u32(percent);
        let arrays: Vec<u32> = [
            LoadedLandData::ppNormalsSet.off,
            LoadedLandData::ppVertices.off,
            LoadedLandData::ppColorsA.off,
            LoadedLandData::ppNormals.off,
        ]
        .iter()
        .map(|field| e.mem.u32(data.addr() + field))
        .collect();
        // The data's destructor body: nothing to release.
        e.register(SMART_POINTER_RELEASE, |_, _| Ret::default());
        e.register(VECTOR_DESTRUCT_ITERATOR, |_, _| Ret::default());
        start_log(&mut e);
        let result = e.call(0x0053_6d80, &args![this]);
        assert_eq!(result.u32() & 0xff, 1);
        let log = end_log(&mut e);
        assert_eq!(log[0], (LAND_SET_LOADED, vec![this.addr(), 0]));
        assert_eq!(log[1], (LAND_RELEASE_BORDER, vec![this.addr()]));
        assert!(loaded_data(&e, this).is_null());
        // The grass map items were emptied and freed.
        let freed: Vec<u32> = arguments_of(&log, DEALLOCATE)
            .into_iter()
            .map(|a| a[0])
            .collect();
        for (item, pointer) in &items {
            assert!(freed.contains(item) && freed.contains(pointer));
        }
        assert_eq!(
            *removed.borrow(),
            (0..4).map(|q| map - 0x10 + 0x10 * q).collect::<Vec<_>>()
        );
        // Percent: its first array, then the array itself.
        let at = freed.iter().position(|a| *a == percent_block).unwrap();
        assert_eq!(freed[at + 1], percent);
        assert!(freed.contains(&vertices_block));
        // The four block arrays are freed in this order after the quadrants,
        // then the data itself last.
        let tail: Vec<u32> = freed[freed.len() - 5..].to_vec();
        assert_eq!(tail[..4], arrays[..]);
        assert_eq!(tail[4], data.addr());
        // The MOPP code was released.
        assert_eq!(
            arguments_of(&log, HAVOK_REMOVE_REFERENCE),
            vec![vec![0x5555]]
        );
    }

    #[test]
    fn unload_of_a_land_without_data_only_clears_the_state() {
        let mut e = loader_engine();
        let this = land(&mut e);
        start_log(&mut e);
        assert_eq!(e.call(0x0053_6d80, &args![this]).u32() & 0xff, 1);
        assert_eq!(called(&mut e), vec![LAND_SET_LOADED, LAND_RELEASE_BORDER]);
    }

    #[test]
    fn data_destructor_releases_its_members() {
        let mut e = engine();
        e.register(SMART_POINTER_RELEASE, |_, _| Ret::default());
        e.register(VECTOR_DESTRUCT_ITERATOR, |_, _| Ret::default());
        let data: Ptr<LoadedLandData> = e.new_object();
        start_log(&mut e);
        e.call(0x0053_7130, &args![data]);
        assert_eq!(
            end_log(&mut e),
            vec![
                (SMART_POINTER_RELEASE, vec![data.addr() + 0x94]),
                (
                    VECTOR_DESTRUCT_ITERATOR,
                    vec![data.addr() + 0x54, 0x10, 4, GRASS_MAP_DESTRUCT]
                ),
                (SMART_POINTER_RELEASE, vec![data.addr() + 0x14])
            ]
        );
        // The deleting form frees the block only with bit 0.
        start_log(&mut e);
        assert_eq!(e.call(0x0053_7100, &args![data, 0u32]).u32(), data.addr());
        assert!(!called(&mut e).contains(&DEALLOCATE));
        e.register(DEALLOCATE, |_, _| Ret::default());
        start_log(&mut e);
        e.call(0x0053_7100, &args![data, 1u32]);
        let log = end_log(&mut e);
        assert_eq!(log.last().unwrap(), &(DEALLOCATE, vec![data.addr()]));
    }

    // ---- Meshes -------------------------------------------------------------------

    /// What the strips builder calls: allocators, the name formatter, the
    /// fixed string (the handle holds the address of the text), and the
    /// strips constructor, which hands out a fresh object each time.
    fn strips_engine() -> Engine {
        let mut e = loader_engine();
        e.register(NI_ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(NI_ALLOCATE_OBJECT, |e, a| ret(e.mem.alloc(a[0].max(0x100))));
        e.register(FORMAT_STRING, |e, a| {
            let text = format!("Block ({}, {})", a[3], a[4]);
            e.mem.set_cstr(a[0], text.as_bytes());
            ret(text.len() as u32)
        });
        e.register(FIXED_STRING_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(FIXED_STRING_DESTRUCT, |_, _| Ret::default());
        e.register(OBJECT_SET_NAME, |_, _| Ret::default());
        e.register(NODE_SET_TRANSLATE, |_, _| Ret::default());
        e.register(LAND_BASE_HEIGHT, |_, _| float_ret(3.0));
        e.register(TRI_STRIPS_CONSTRUCT, |e, _| ret(e.mem.alloc(0x40)));
        e
    }

    #[test]
    fn tri_strips_are_built_from_the_blocks_of_a_quadrant() {
        let mut e = strips_engine();
        let constructed = recorder::<(Vec<u32>, Vec<u8>, Vec<u8>, Vec<u8>, u16)>();
        let sink = constructed.clone();
        e.register_double(TRI_STRIPS_CONSTRUCT, move |e, a| {
            // Snapshot what the constructor is given: the normals and
            // texture coordinate copies, the strip index list and the
            // length.
            sink.borrow_mut().push((
                a.to_vec(),
                e.mem.bytes(a[3], 0xd8c),
                e.mem.bytes(a[5], 0x908),
                e.mem.bytes(a[11], 0x7fa),
                e.mem.u16(a[10]),
            ));
            ret(e.mem.alloc(0x40))
        });
        let translated = recorder::<(u32, Vec<f32>)>();
        let sink = translated.clone();
        e.register_double(NODE_SET_TRANSLATE, move |e, a| {
            sink.borrow_mut().push((a[0], floats(e, a[1], 3)));
            Ret::default()
        });
        let names = recorder::<(u32, String)>();
        let sink = names.clone();
        e.register_double(OBJECT_SET_NAME, move |e, a| {
            let text = e.mem.u32(a[1]);
            sink.borrow_mut()
                .push((a[0], String::from_utf8(e.mem.cstr(text)).unwrap()));
            Ret::default()
        });
        let defaults = e.mem.alloc(0x908);
        e.mem.write(defaults, &vec![0x5a; 0x908]);
        e.set_global(DEFAULT_TEXTURE_COORDINATES, defaults);
        let strip_list: Vec<u8> = (0..0x7fa).map(|i| (i % 251) as u8).collect();
        e.mem.write(TRIANGLE_STRIP_LIST, &strip_list);
        let (this, data) = land_with_data(&mut e);
        fill_blocks(&mut e, data);
        e.set(data, LoadedLandData::iCellX, 1);
        e.set(data, LoadedLandData::iCellY, 2);
        let vertices = e.get(data, LoadedLandData::ppVertices);
        let colors = e.get(data, LoadedLandData::ppColorsA);
        let result = e.call(0x0053_71b0, &args![this, 2u32]);
        let strips = result.u32();
        assert_ne!(strips, 0);
        let constructed = constructed.borrow();
        assert_eq!(constructed.len(), 1);
        let (arguments, normals, coordinates, indices, length) = &constructed[0];
        assert_eq!(
            arguments[1..9],
            [
                0x121,
                element(&e, vertices, 2),
                arguments[3],
                element(&e, colors, 2),
                arguments[5],
                1,
                0,
                0x3fb
            ]
        );
        assert_eq!(arguments[9], 1);
        // The normals are a copy of the quadrant's, the coordinates a copy
        // of the defaults.
        assert!(normals.iter().all(|b| *b == 32 + 2));
        assert_ne!(
            arguments[3],
            element(&e, e.get(data, LoadedLandData::ppNormals), 2)
        );
        assert!(coordinates.iter().all(|b| *b == 0x5a));
        assert_eq!(*length, 0x3fd);
        assert_eq!(*indices, strip_list);
        // Moved to the land's centre and named after the block.
        assert_eq!(
            *translated.borrow(),
            vec![(strips, vec![6144.0, 10240.0, 3.0])]
        );
        assert_eq!(*names.borrow(), vec![(strips, "Block (2, 0)".to_string())]);
        // Blocks from 4 on, and lands without data, give nothing.
        assert_eq!(e.call(0x0053_71b0, &args![this, 4u32]).u32(), 0);
        let bare = land(&mut e);
        assert_eq!(e.call(0x0053_71b0, &args![bare, 0u32]).u32(), 0);
        assert_eq!(constructed.len(), 1);
    }

    // ---- The quadrant meshes ----------------------------------------------------------

    type EventLog = Rc<RefCell<Vec<(&'static str, Vec<u32>)>>>;

    /// Everything the mesh builder touches, recorded as `(name, words)`.
    struct MeshWorld {
        events: EventLog,
        nodes: Vec<u32>,
        roots: Vec<u32>,
        strips: Rc<RefCell<Vec<u32>>>,
        renderer: u32,
    }

    fn event(world: &EventLog, name: &'static str, words: &[u32]) {
        world.borrow_mut().push((name, words.to_vec()));
    }

    /// An engine with a cell of four nodes and doubles for every scene graph
    /// and Havok call of the mesh builder.
    fn mesh_world() -> (Engine, Ptr<TESObjectLAND>, Ptr<LoadedLandData>, MeshWorld) {
        let mut e = strips_engine();
        let events = recorder::<(&'static str, Vec<u32>)>();
        let strips = recorder::<u32>();
        // Nodes with `AttachChild` in slot 0xf8, roots with the world
        // translate setter in slot 0xb8.
        let mut nodes = vec![];
        let mut roots = vec![];
        let sink = events.clone();
        e.register_double(0x00aa_0020, move |_, a| {
            event(&sink, "attach", a);
            Ret::default()
        });
        let sink = events.clone();
        e.register_double(0x00aa_0021, move |e, a| {
            let mut words = vec![a[0]];
            words.extend(floats(e, a[1], 3).into_iter().map(f32::to_bits));
            event(&sink, "world translate", &words);
            Ret::default()
        });
        for _ in 0..4 {
            nodes.push(object_with_vtable(&mut e, 0x20, &[(0xf8, 0x00aa_0020)]));
            roots.push(object_with_vtable(&mut e, 0x20, &[(0xb8, 0x00aa_0021)]));
        }
        let renderer = object_with_vtable(&mut e, 0x20, &[(0xec, 0x00aa_0022)]);
        let sink = events.clone();
        e.register_double(0x00aa_0022, move |_, a| {
            event(&sink, "queue", a);
            Ret::default()
        });
        e.register(CELL_GET_3D, |_, _| ret(1));
        let list = nodes.clone();
        e.register_double(CELL_GET_NODE, move |_, a| ret(list[a[1] as usize]));
        let list = nodes.clone();
        e.register_double(NODE_GET_CULLED, move |_, a| {
            ret(list.iter().position(|n| *n == a[0]).unwrap() as u32 % 2)
        });
        let sink = events.clone();
        e.register_double(NODE_SET_CULLED, move |_, a| {
            event(&sink, "culled", a);
            Ret::default()
        });
        let sink = events.clone();
        e.register_double(NODE_ATTACH_PROPERTY, move |_, a| {
            event(&sink, "property", a);
            Ret::default()
        });
        e.set_global(DEFAULT_TEXTURING_PROPERTY, 0x9999u32);
        let list = strips.clone();
        e.register_double(TRI_STRIPS_CONSTRUCT, move |e, _| {
            let object = e.mem.alloc(0x40);
            list.borrow_mut().push(object);
            ret(object)
        });
        e.register(TRI_STRIPS_VERTEX_COUNT, |_, _| ret(0x121));
        e.register(GEOMETRY_GET_POSITIONS, |_, a| ret(a[0] + 1));
        e.register(LAND_BOUND_SOURCE, |_, a| ret(a[0] + 2));
        e.register(GEOMETRY_GET_DATA, |_, a| ret(a[0] + 3));
        let sink = events.clone();
        e.register_double(BOUND_COMPUTE_FROM_DATA, move |_, a| {
            event(&sink, "bound", a);
            Ret::default()
        });
        let sink = events.clone();
        e.register_double(GEOMETRY_DATA_MARK_CHANGED, move |_, a| {
            event(&sink, "changed", a);
            Ret::default()
        });
        e.register(FACE_GEN_NODE_GET_ANIMATION_DATA, |_, a| ret(a[0] + 8));
        let list: Vec<(u32, u32)> = nodes.iter().map(|n| n + 8).zip(roots.clone()).collect();
        e.register_double(ANIMATION_DATA_GET_ROOT, move |_, a| {
            ret(list.iter().find(|(anim, _)| *anim == a[0]).unwrap().1)
        });
        e.register(LAND_QUADRANT_EXTENTS, |e, a| {
            e.mem.set_f32(a[1], -4.0);
            e.mem.set_f32(a[1] + 4, 12.0 + a[2] as f32);
            Ret::default()
        });
        let sink = events.clone();
        e.register_double(ROOT_SET_LOCAL_TRANSLATE, move |e, a| {
            let mut words = vec![a[0]];
            words.extend(floats(e, a[1], 3).into_iter().map(f32::to_bits));
            event(&sink, "local translate", &words);
            Ret::default()
        });
        e.register(LAND_CELL_MOPP, |_, _| ret(0x6600));
        let sink = events.clone();
        e.register_double(CELL_MOPP_PREPARE, move |_, a| {
            event(&sink, "mopp prepare", a);
            Ret::default()
        });
        e.register(LAND_MOPP_LOCAL_CONSTRUCT, |_, _| Ret::default());
        let sink = events.clone();
        e.register_double(CELL_MOPP_CREATE, move |e, a| {
            let mut words = a.to_vec();
            words.extend(read_words(e, a[1], 4));
            event(&sink, "mopp create", &words);
            Ret::default()
        });
        for address in LAND_BUILD_FOLLOW_UPS {
            let sink = events.clone();
            e.register_double(address, move |_, a| {
                event(&sink, "follow-up", a);
                Ret::default()
            });
        }
        let sink = events.clone();
        e.register_double(GEOMETRY_DATA_SET_CONSISTENCY, move |_, a| {
            event(&sink, "consistency", a);
            Ret::default()
        });
        let sink = events.clone();
        e.register_double(GEOMETRY_DATA_SET_KEEP_FLAGS, move |_, a| {
            event(&sink, "keep", a);
            Ret::default()
        });
        let sink = events.clone();
        e.register_double(GEOMETRY_DATA_SET_COMPRESS_FLAGS, move |_, a| {
            event(&sink, "compress", a);
            Ret::default()
        });
        // The scheduler of strips `n` (in construction order): none, state
        // 3 (valid), state 0 and state 6 (both invalid).
        let list = strips.clone();
        e.register_double(GEOMETRY_GET_SCHEDULER, move |_, a| {
            let index = list.borrow().iter().position(|s| *s == a[0]).unwrap();
            ret(if index == 0 { 0 } else { 0x7000 + index as u32 })
        });
        e.register(SCHEDULER_GET_SAVED_ACQUIRE, |_, a| {
            ret([0, 3, 0, 6][(a[0] - 0x7000) as usize])
        });
        e.register(LAND_RENDER_TASK, |_, a| ret(a[0] + 0x100));
        e.register_double(RENDERER_GET, move |_, _| ret(renderer));
        let sink = events.clone();
        e.register_double(RENDERER_AFTER_QUEUE, move |_, a| {
            event(&sink, "after queue", a);
            Ret::default()
        });
        let sink = events.clone();
        e.register_double(UPDATE_DATA_CONSTRUCT, move |_, a| {
            event(&sink, "update data", a);
            Ret::default()
        });
        let sink = events.clone();
        e.register_double(NODE_UPDATE, move |_, a| {
            event(&sink, "update", a);
            Ret::default()
        });
        let sink = events.clone();
        e.register_double(NODE_UPDATE_PROPERTIES, move |_, a| {
            event(&sink, "update properties", a);
            Ret::default()
        });
        e.register(CELL_QUADRANT_TEST, |_, a| ret((a[1] == 2) as u32));
        e.register(CELL_REFERENCE_LIST_CHECK, |_, _| ret(0));
        let defaults = e.mem.alloc(0x908);
        e.set_global(DEFAULT_TEXTURE_COORDINATES, defaults);
        let (this, data) = land_with_data(&mut e);
        fill_blocks(&mut e, data);
        e.set(data, LoadedLandData::iCellX, 2);
        e.set(data, LoadedLandData::iCellY, -1);
        e.set(data, LoadedLandData::fBaseHeight, 3.0);
        e.set(this, TESObjectLAND::pParentCell, Ptr::new(0x6000));
        (
            e,
            this,
            data,
            MeshWorld {
                events,
                nodes,
                roots,
                strips,
                renderer,
            },
        )
    }

    fn events_named(world: &MeshWorld, name: &str) -> Vec<Vec<u32>> {
        world
            .events
            .borrow()
            .iter()
            .filter(|(n, _)| *n == name)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn meshes_are_built_for_the_four_quadrants() {
        let (mut e, this, data, world) = mesh_world();
        e.call(0x0053_74f0, &args![this]);
        let strips = world.strips.borrow().clone();
        assert_eq!(strips.len(), 4);
        // The mesh array holds the cell's four nodes.
        let meshes = e.get(data, LoadedLandData::ppMesh);
        assert_eq!(read_words(&e, meshes.addr(), 4), world.nodes);
        // Hidden while built (node 1 and 3 were hidden before), the default
        // property attached, the strips attached with arguments (0, strips).
        let culled = events_named(&world, "culled");
        for (i, node) in world.nodes.iter().enumerate() {
            assert_eq!(culled[i], vec![*node, 1]);
            assert_eq!(events_named(&world, "property")[i], vec![*node, 0x9999]);
            assert_eq!(events_named(&world, "attach")[i], vec![*node, 0, strips[i]]);
            // The bound is computed from the strips' vertices.
            assert_eq!(
                events_named(&world, "bound")[i],
                vec![strips[i] + 2, 0x121, strips[i] + 1]
            );
            assert_eq!(events_named(&world, "changed")[i], vec![strips[i] + 3, 0xf]);
            // The root node: local translation from the height extents
            // (-4 to 12 + quadrant), world translation at the quadrant's
            // centre.
            let root = world.roots[i];
            let half = (16.0 + i as f32) / 2.0;
            assert_eq!(
                events_named(&world, "local translate")[i],
                vec![
                    root,
                    1024.0f32.to_bits(),
                    1024.0f32.to_bits(),
                    half.to_bits()
                ]
            );
            let x = 2.0 * 4096.0 + 1024.0 + (i % 2) as f32 * 2048.0;
            let y = -4096.0 + 1024.0 + (i / 2) as f32 * 2048.0;
            let z = (8.0 + i as f32) / 2.0;
            assert_eq!(
                events_named(&world, "world translate")[i],
                vec![root, x.to_bits(), y.to_bits(), z.to_bits()]
            );
        }
        // The Havok MOPP of the cell from the four strips and the base
        // height, followed by the three follow-up builds.
        assert_eq!(events_named(&world, "mopp prepare"), vec![vec![0x6600]]);
        let create = &events_named(&world, "mopp create")[0];
        assert_eq!(create[0], 0x6600);
        assert_eq!(create[2], 4);
        assert_eq!(create[3], 3.0f32.to_bits());
        assert_eq!(create[4..], strips[..]);
        assert_eq!(
            events_named(&world, "follow-up"),
            vec![vec![this.addr()]; 3]
        );
        // Geometry data flags, for all four.
        assert_eq!(events_named(&world, "consistency").len(), 4);
        assert_eq!(
            events_named(&world, "consistency")[0],
            vec![strips[0] + 3, 0x4000]
        );
        assert_eq!(events_named(&world, "keep")[0], vec![strips[0] + 3, 1]);
        assert_eq!(
            events_named(&world, "compress")[0],
            vec![strips[0] + 3, 0x17]
        );
        // Only the strips whose scheduler is in state 1 to 5 are queued with
        // the renderer.
        let queued = events_named(&world, "queue");
        assert_eq!(
            queued,
            vec![vec![world.renderer, strips[1], 0, 0, 0x7001 + 0x100]]
        );
        assert_eq!(
            events_named(&world, "after queue"),
            vec![vec![world.renderer]]
        );
        // The nodes get their hidden state back, are updated with a fresh
        // update data (time 0), and their properties updated.
        let updates = events_named(&world, "update");
        let data_objects = events_named(&world, "update data");
        assert_eq!(updates.len(), 4);
        for i in 0..4 {
            assert_eq!(updates[i][0], world.nodes[i]);
            assert_eq!(updates[i][1], data_objects[i][0]);
            assert_eq!(data_objects[i][1..], [0, 0, 0]);
            assert_eq!(
                events_named(&world, "update properties")[i],
                vec![world.nodes[i]]
            );
            // After the first four (hiding) and before the last four, the
            // restoring of the culled flag: node 1 and 3 were hidden before.
            assert_eq!(culled[4 + i], vec![world.nodes[i], (i % 2) as u32]);
        }
        // Finally the nodes are hidden as the cell says: quadrant 2 only.
        for i in 0..4 {
            assert_eq!(culled[8 + i], vec![world.nodes[i], (i == 2) as u32]);
        }
    }

    #[test]
    fn existing_meshes_only_get_their_hidden_flags() {
        let (mut e, this, _data, world) = mesh_world();
        e.call(0x0053_74f0, &args![this]);
        let before = world.events.borrow().len();
        // A land of the world space's land file whose cell fails the check:
        // everything hidden.
        e.set(this, TESObjectLAND::Data, 0x400);
        e.call(0x0053_74f0, &args![this]);
        let culled: Vec<Vec<u32>> = world.events.borrow()[before..]
            .iter()
            .filter(|(n, _)| *n == "culled")
            .map(|(_, w)| w.clone())
            .collect();
        assert_eq!(
            culled,
            world.nodes.iter().map(|n| vec![*n, 1]).collect::<Vec<_>>()
        );
        // When the cell passes the check, the quadrants are asked.
        e.register(CELL_REFERENCE_LIST_CHECK, |_, _| ret(1));
        let before = world.events.borrow().len();
        e.call(0x0053_74f0, &args![this]);
        let culled: Vec<Vec<u32>> = world.events.borrow()[before..]
            .iter()
            .filter(|(n, _)| *n == "culled")
            .map(|(_, w)| w.clone())
            .collect();
        assert_eq!(
            culled,
            world
                .nodes
                .iter()
                .enumerate()
                .map(|(i, n)| vec![*n, (i == 2) as u32])
                .collect::<Vec<_>>()
        );
        // No strips were built again.
        assert_eq!(world.strips.borrow().len(), 4);
        // Without a parent cell, or when it has no 3D, nothing happens.
        let before = world.events.borrow().len();
        e.set(this, TESObjectLAND::pParentCell, Ptr::NULL);
        e.call(0x0053_74f0, &args![this]);
        e.set(this, TESObjectLAND::pParentCell, Ptr::new(0x6000));
        e.register(CELL_GET_3D, |_, _| ret(0));
        e.call(0x0053_74f0, &args![this]);
        assert_eq!(world.events.borrow().len(), before);
    }

    // ---- Second session: small functions, Havok MOPP code, save --------------

    fn noop(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, |_, _| Ret::default());
        }
    }

    /// A block filled with 0xdd bytes, to see what a function writes.
    fn dirty_object(e: &mut Engine, size: u32) -> u32 {
        let object = e.mem.alloc(size);
        e.mem.write(object, &vec![0xdd; size as usize]);
        object
    }

    #[test]
    fn bound_getter_asks_the_callee_about_the_smart_pointer() {
        let mut e = engine();
        e.register(GETTER_00460140, |_, a| ret(a[0] + 0x10));
        let object = e.mem.alloc(0xc0);
        e.mem.set_u32(object + 0xb8, 0x4444);
        assert_eq!(e.call(0x0053_7b10, &args![object]).u32(), 0x4454);
    }

    #[test]
    fn mopp_holder_getter_passes_the_member() {
        let mut e = engine();
        e.register(GETTER_0041BB10, |_, a| ret(a[0] + 1));
        assert_eq!(e.call(0x0053_7b30, &args![0x1000u32]).u32(), 0x1029);
    }

    #[test]
    fn reference_list_check_is_true_when_the_callee_says_zero() {
        let mut e = engine();
        e.register(LIST_CHECK, |_, a| ret((a[0] != 0x2000 + 0xac) as u32));
        assert_eq!(e.call(0x0053_7b50, &args![0x2000u32]).u32() & 0xff, 1);
        e.register(LIST_CHECK, |_, _| ret(0x100 | 1));
        assert_eq!(e.call(0x0053_7b50, &args![0x2000u32]).u32() & 0xff, 0);
        e.register(LIST_CHECK, |_, _| ret(0x100));
        assert_eq!(e.call(0x0053_7b50, &args![0x2000u32]).u32() & 0xff, 1);
    }

    #[test]
    fn mesh_helper_constructors_run_the_base_and_set_their_fields() {
        let mut e = engine();
        e.register(BASE_CONSTRUCT_004B5020, |_, a| ret(a[0]));
        let object = dirty_object(&mut e, 0x10);
        start_log(&mut e);
        assert_eq!(e.call(0x0053_7bb0, &args![object]).u32(), object);
        assert_eq!(
            end_log(&mut e),
            vec![(BASE_CONSTRUCT_004B5020, vec![object])]
        );
        assert_eq!(e.mem.u32(object + 4), 0);
        assert_eq!(e.mem.u32(object + 8), 0xdddd_dddd);
        let object = dirty_object(&mut e, 0x10);
        start_log(&mut e);
        assert_eq!(e.call(0x0053_7b80, &args![object]).u32(), object);
        assert_eq!(
            end_log(&mut e),
            vec![(BASE_CONSTRUCT_004B5020, vec![object])]
        );
        assert_eq!(e.mem.u32(object + 4), 0);
        assert_eq!(e.mem.u32(object + 8), 0);
        assert_eq!(e.mem.f32(object + 0xc), 1.0);
    }

    #[test]
    fn render_task_getter_reads_the_smart_pointer() {
        let mut e = engine();
        let object = e.mem.alloc(0x80);
        e.mem.set_u32(object + 0x78, 0x3333);
        assert_eq!(e.call(0x0053_7bd0, &args![object]).u32(), 0x3333);
    }

    #[test]
    fn additional_geometry_data_constructor_sets_its_fields() {
        let mut e = engine();
        e.register(NI_OBJECT_CONSTRUCT, |_, a| ret(a[0]));
        noop(&mut e, &[MEMBER_CONSTRUCT, MEMBER_CONSTRUCT_AGAIN]);
        let object = dirty_object(&mut e, 0x2c);
        start_log(&mut e);
        let result = e.call(0x0053_7bf0, &args![object, 0x1234u32]);
        assert_eq!(result.u32(), object);
        assert_eq!(
            end_log(&mut e),
            vec![
                (NI_OBJECT_CONSTRUCT, vec![object]),
                (MEMBER_CONSTRUCT, vec![object + 0x1c, 0, 1]),
                (MEMBER_CONSTRUCT_AGAIN, vec![object + 0x1c]),
            ]
        );
        assert_eq!(e.mem.u32(object), ADDITIONAL_GEOMETRY_DATA_VTABLE);
        assert_eq!(e.mem.u32(object + 8), 0);
        assert_eq!(e.mem.u16(object + 0xc), 0x1234);
        assert_eq!(e.mem.u32(object + 0x10), 0);
        assert_eq!(e.mem.u32(object + 0x14), 0);
        assert_eq!(e.mem.u32(object + 0x18), 0);
        // The member's own bytes are left to its constructors.
        assert_eq!(e.mem.u32(object + 0x1c), 0xdddd_dddd);
    }

    #[test]
    fn additional_geometry_data_rtti_is_a_constant() {
        let mut e = engine();
        assert_eq!(e.call(0x0053_7ca0, &args![]).u32(), 0x011f_4a30);
        assert_eq!(e.call(0x0053_7d00, &args![]).u32(), 0x011f_4aac);
    }

    #[test]
    fn additional_geometry_data_deleting_destructor_frees_with_bit_zero() {
        let mut e = engine();
        noop(&mut e, &[ADDITIONAL_GEOMETRY_DATA_DESTRUCT, NI_FREE]);
        start_log(&mut e);
        assert_eq!(e.call(0x0053_7cb0, &args![0x5000u32, 2u32]).u32(), 0x5000);
        assert_eq!(
            end_log(&mut e),
            vec![(ADDITIONAL_GEOMETRY_DATA_DESTRUCT, vec![0x5000])]
        );
        start_log(&mut e);
        e.call(0x0053_7cb0, &args![0x5000u32, 3u32]);
        assert_eq!(
            end_log(&mut e),
            vec![
                (ADDITIONAL_GEOMETRY_DATA_DESTRUCT, vec![0x5000]),
                (NI_FREE, vec![0x5000, 0x2c]),
            ]
        );
    }

    #[test]
    fn member_destructor_is_called_on_this() {
        let mut e = engine();
        noop(&mut e, &[MEMBER_DESTRUCT]);
        start_log(&mut e);
        e.call(0x0053_7ce0, &args![0x6000u32]);
        assert_eq!(end_log(&mut e), vec![(MEMBER_DESTRUCT, vec![0x6000])]);
    }

    #[test]
    fn packed_additional_geometry_data_destructors() {
        let mut e = engine();
        noop(&mut e, &[ADDITIONAL_GEOMETRY_DATA_DESTRUCT, NI_FREE]);
        let object = dirty_object(&mut e, 0x34);
        start_log(&mut e);
        e.call(0x0053_7d40, &args![object]);
        assert_eq!(
            end_log(&mut e),
            vec![(ADDITIONAL_GEOMETRY_DATA_DESTRUCT, vec![object])]
        );
        assert_eq!(e.mem.u32(object), PACKED_ADDITIONAL_GEOMETRY_DATA_VTABLE);
        // The deleting form: destructor, then the free of 0x34 bytes.
        let object = dirty_object(&mut e, 0x34);
        start_log(&mut e);
        assert_eq!(e.call(0x0053_7d10, &args![object, 1u32]).u32(), object);
        assert_eq!(
            end_log(&mut e),
            vec![
                (ADDITIONAL_GEOMETRY_DATA_DESTRUCT, vec![object]),
                (NI_FREE, vec![object, 0x34]),
            ]
        );
        assert_eq!(e.mem.u32(object), PACKED_ADDITIONAL_GEOMETRY_DATA_VTABLE);
        start_log(&mut e);
        e.call(0x0053_7d10, &args![object, 0u32]);
        assert_eq!(called(&mut e), vec![ADDITIONAL_GEOMETRY_DATA_DESTRUCT]);
    }

    /// A land with loaded data whose four meshes (all but `missing`) are
    /// nodes with a detach slot, plus the doubles of the release function.
    fn release_world(
        e: &mut Engine,
        missing: Option<u32>,
    ) -> (Ptr<TESObjectLAND>, Ptr<LoadedLandData>, Vec<u32>, u32) {
        let detach = 0x0900_0001;
        let owner_slot = 0x0900_0002;
        e.register(detach, |_, _| Ret::default());
        e.register(owner_slot, |_, _| Ret::default());
        noop(
            e,
            &[
                PHYSICS_OBJECT_ADD_BODY,
                SHADOW_SCENE_NODE_REMOVE_OBJECT,
                TEXTURING_PROPERTY_SET,
            ],
        );
        e.register(QUEUED_FILE_POINTER_SET, |_, _| Ret::default());
        e.register(CELL_PHYSICS_OBJECT, |_, _| ret(0x7777));
        e.register(SLOT_HOLDS_OTHER_THAN, |_, _| ret(1));
        e.register(TABLE_ENTRY, |_, _| ret(0x8888));
        let (this, data) = land_with_data(e);
        e.set(this, TESObjectLAND::pParentCell, Ptr::new(0x6000));
        let meshes = e.mem.alloc(16);
        let mut nodes = Vec::new();
        for quadrant in 0..4 {
            let node = object_with_vtable(e, 0x20, &[(NODE_DETACH_SLOT, detach)]);
            if missing != Some(quadrant) {
                e.mem.set_u32(meshes + 4 * quadrant, node);
            }
            nodes.push(node);
        }
        e.set(data, LoadedLandData::ppMesh, Ptr::new(meshes));
        e.mem.set_u32(data.addr() + 0x94, 0x5151);
        let border = 0x6161;
        e.mem.set_u32(data.addr() + 0x14, border);
        let owner = object_with_vtable(e, 0x20, &[(BORDER_OWNER_SLOT, owner_slot)]);
        e.register_double(OBJECT_FIELD_0X18, move |_, _| ret(owner));
        (this, data, nodes, owner)
    }

    #[test]
    fn release_drops_the_quadrants_the_body_and_the_border() {
        let mut e = engine();
        let (this, data, nodes, owner) = release_world(&mut e, Some(2));
        let meshes = e.get(data, LoadedLandData::ppMesh);
        let normals = e.get(data, LoadedLandData::ppNormals);
        let blocks: Vec<u32> = (0..4).map(|q| element(&e, normals, q)).collect();
        let vertices = e.get(data, LoadedLandData::ppVertices);
        e.mem.set_u32(vertices.addr() + 8, 0x1111);
        start_log(&mut e);
        e.call(0x0053_7eb0, &args![this]);
        let log = end_log(&mut e);
        assert_eq!(
            arguments_of(&log, QUEUED_FILE_POINTER_SET),
            vec![vec![this.addr() + 0x24, 0]]
        );
        assert_eq!(
            arguments_of(&log, PHYSICS_OBJECT_ADD_BODY),
            vec![vec![0x7777, 0x5151]]
        );
        assert_eq!(
            arguments_of(&log, TEXTURING_PROPERTY_SET),
            vec![vec![data.addr() + 0x94, 0], vec![data.addr() + 0x14, 0]]
        );
        let present = [nodes[0], nodes[1], nodes[3]];
        assert_eq!(
            arguments_of(&log, SHADOW_SCENE_NODE_REMOVE_OBJECT),
            present.iter().map(|n| vec![0x8888, *n]).collect::<Vec<_>>()
        );
        assert_eq!(
            arguments_of(&log, 0x0900_0001),
            present.iter().map(|n| vec![*n, 0]).collect::<Vec<_>>()
        );
        let freed: Vec<u32> = arguments_of(&log, DEALLOCATE)
            .into_iter()
            .map(|a| a[0])
            .collect();
        assert_eq!(freed, vec![blocks[0], blocks[1], blocks[3], meshes.addr()]);
        // The slots are cleared; the quadrant without a mesh keeps its arrays.
        assert_eq!(e.mem.u32(vertices.addr() + 8), 0x1111);
        assert_eq!(element(&e, normals, 2), blocks[2]);
        assert_eq!(element(&e, normals, 0), 0);
        assert_eq!(element(&e, vertices, 0), 0);
        assert!(e.get(data, LoadedLandData::ppMesh).is_null());
        // The border owner releases the border.
        assert_eq!(arguments_of(&log, 0x0900_0002), vec![vec![owner, 0x6161]]);
    }

    #[test]
    fn release_without_meshes_or_data_does_less() {
        let mut e = engine();
        let (this, data, _, _) = release_world(&mut e, None);
        e.set(data, LoadedLandData::ppMesh, Ptr::NULL);
        start_log(&mut e);
        e.call(0x0053_7eb0, &args![this]);
        let log = end_log(&mut e);
        assert!(arguments_of(&log, CELL_PHYSICS_OBJECT).is_empty());
        assert_eq!(
            arguments_of(&log, TEXTURING_PROPERTY_SET),
            vec![vec![data.addr() + 0x14, 0]]
        );
        // A border that is not held is left alone.
        e.mem.set_u32(data.addr() + 0x14, 0);
        start_log(&mut e);
        e.call(0x0053_7eb0, &args![this]);
        assert_eq!(
            called(&mut e),
            vec![QUEUED_FILE_POINTER_SET, SMART_POINTER_GET]
        );
        // No loaded data at all: only the queued textures are cleared.
        let bare = land(&mut e);
        start_log(&mut e);
        e.call(0x0053_7eb0, &args![bare]);
        assert_eq!(called(&mut e), vec![QUEUED_FILE_POINTER_SET]);
        // Without the body in the physics object, the body is only released.
        let (this, data, _, _) = release_world(&mut e, None);
        e.register(SLOT_HOLDS_OTHER_THAN, |_, _| ret(0));
        start_log(&mut e);
        e.call(0x0053_7eb0, &args![this]);
        let log = end_log(&mut e);
        assert!(arguments_of(&log, PHYSICS_OBJECT_ADD_BODY).is_empty());
        assert_eq!(
            arguments_of(&log, TEXTURING_PROPERTY_SET)[0],
            vec![data.addr() + 0x94, 0]
        );
    }

    // ---- Save ----------------------------------------------------------------

    type Chunks = Rc<RefCell<Vec<(u32, Vec<u8>)>>>;

    /// An engine with the doubles of the save writer; every chunk written is
    /// recorded as `(tag, bytes)`.
    fn save_engine(big_endian: bool) -> (Engine, Chunks) {
        let mut e = engine();
        noop(
            &mut e,
            &[
                FORM_START,
                FORM_CLOSE,
                FORM_COMPRESS_SAVE_BUFFER,
                LAND_FLUSH_WARNINGS,
                LAND_LOAD_FOR_SAVE,
                SWAP_WORD,
                SWAP_BYTES_32,
                SWAP_BYTES_16,
                LOG_MESSAGE,
                NI_POINT3_NORMALIZE,
            ],
        );
        if big_endian {
            e.register(IS_BIG_ENDIAN, |_, _| ret(1));
        } else {
            e.register(IS_BIG_ENDIAN, |_, _| ret(0));
        }
        e.register(FLOAT_TO_INT, |_, a| ret(f32::from_bits(a[0]) as i32 as u32));
        e.register(INT_ABS, |_, a| ret((a[0] as i32).wrapping_abs() as u32));
        e.register(CELL_GET_DATA_X, |_, _| ret(-7i32 as u32));
        e.register(CELL_GET_DATA_Y, |_, _| ret(9));
        e.register(LAND_LAYER_OPACITY, |_, _| float_ret(0.0));
        let chunks: Chunks = recorder();
        for address in [FORM_ADD_CHUNK_DATA, FORM_ADD_CHUNK_ARRAY] {
            let sink = chunks.clone();
            e.register_double(address, move |e, a| {
                sink.borrow_mut().push((a[0], e.mem.bytes(a[1], a[2])));
                Ret::default()
            });
        }
        (e, chunks)
    }

    /// Sets every vertex height, normal (0, 0, 1) and colour of the data.
    fn fill_land(e: &mut Engine, data: Ptr<LoadedLandData>, height: f32, color: [f32; 4]) {
        let vertices = e.get(data, LoadedLandData::ppVertices);
        let normals = e.get(data, LoadedLandData::ppNormals);
        let colors = e.get(data, LoadedLandData::ppColorsA);
        for quadrant in 0..4 {
            for vertex in 0..0x121 {
                let at = element(e, vertices, quadrant) + vertex * 12;
                e.mem.set_f32(at + 8, height);
                let at = element(e, normals, quadrant) + vertex * 12;
                e.mem.set_f32(at + 8, 1.0);
                let at = element(e, colors, quadrant) + vertex * 16;
                for (i, value) in color.iter().enumerate() {
                    e.mem.set_f32(at + 4 * i as u32, *value);
                }
            }
        }
    }

    fn tags(chunks: &Chunks) -> Vec<u32> {
        chunks.borrow().iter().map(|(tag, _)| *tag).collect()
    }

    #[test]
    fn save_writes_flags_normals_heights_and_colors() {
        let (mut e, chunks) = save_engine(false);
        let (this, data) = land_with_data(&mut e);
        e.set(this, TESObjectLAND::Data, 0x1 | 0x2 | 0x8);
        fill_land(&mut e, data, 80.0, [1.0, 0.5, 0.0, 1.0]);
        e.set(data, LoadedLandData::iCellX, -3);
        e.set(data, LoadedLandData::iCellY, 4);
        start_log(&mut e);
        e.call(0x0053_8110, &args![this]);
        let log = end_log(&mut e);
        assert_eq!(
            tags(&chunks),
            vec![CHUNK_DATA, CHUNK_NORMALS, CHUNK_HEIGHTS, CHUNK_COLORS]
        );
        let chunks = chunks.borrow();
        assert_eq!(chunks[0].1, 0xbu32.to_le_bytes());
        // Normals: (0, 0, 127) for each of the 33 x 33 grid points.
        assert_eq!(chunks[1].1.len(), 0xcc3);
        assert!(chunks[1].1.chunks(3).all(|n| n == [0, 0, 127]));
        // Heights: 80 >> 3 = 10 everywhere, so the base is 10 and the steps 0.
        assert_eq!(chunks[2].1.len(), 0x448);
        assert_eq!(
            f32::from_le_bytes(chunks[2].1[..4].try_into().unwrap()),
            10.0
        );
        assert!(chunks[2].1[4..].iter().all(|b| *b == 0));
        // Colours: red 1.0 -> 255, green 0.5 -> 127, blue 0.
        assert!(chunks[3].1.chunks(3).all(|c| c == [255, 127, 0]));
        // Nothing was clamped, so nothing was logged, and the form was
        // opened and closed around the chunks.
        assert!(arguments_of(&log, LOG_MESSAGE).is_empty());
        let order = called_order(&log);
        assert!(order.contains(&FORM_START));
        assert_eq!(
            &order[order.len() - 2..],
            &[FORM_CLOSE, FORM_COMPRESS_SAVE_BUFFER]
        );
        // The normals were normalized in place: 4 x 289 vertices minus the
        // shared edges are 1089 grid points, and the code handles the corner
        // vertex of quadrant 1 that quadrant 3 writes again (the column test
        // undoes the row test), so one more.
        assert_eq!(arguments_of(&log, NI_POINT3_NORMALIZE).len(), 0x442);
    }

    /// The addresses of the calls of `log`, in order.
    fn called_order(log: &[(u32, Vec<u32>)]) -> Vec<u32> {
        log.iter().map(|(address, _)| *address).collect()
    }

    #[test]
    fn save_clamps_big_height_steps_and_reports_them() {
        let (mut e, chunks) = save_engine(false);
        let (this, data) = land_with_data(&mut e);
        e.set(this, TESObjectLAND::Data, 0x1 | 0x8);
        fill_land(&mut e, data, 80.0, [0.0; 4]);
        e.set(data, LoadedLandData::iCellX, -3);
        e.set(data, LoadedLandData::iCellY, 4);
        // The second grid point of the first row rises by 200 steps; the
        // third falls back to the base.
        let vertices = e.get(data, LoadedLandData::ppVertices);
        let second = element(&e, vertices, 0) + 12;
        e.mem.set_f32(second + 8, 80.0 + 8.0 * 200.0);
        start_log(&mut e);
        e.call(0x0053_8110, &args![this]);
        let log = end_log(&mut e);
        let chunks = chunks.borrow();
        let heights = &chunks[2].1;
        assert_eq!(chunks[2].0, CHUNK_HEIGHTS);
        // Step 0 is 0; step 1 is clamped to +127; the follower sees the
        // clamped previous height (80 + 127 steps), so the step back down is
        // clamped to -127 too, and the later ones stay on the clamped track.
        assert_eq!(heights[4], 0);
        assert_eq!(heights[5], 0x7f);
        assert_eq!(heights[6], 0x81);
        assert_eq!(
            arguments_of(&log, LOG_MESSAGE),
            vec![vec![SAVE_HEIGHT_ERROR, -3i32 as u32, 4]]
        );
    }

    #[test]
    fn save_steps_restart_from_the_row_start() {
        let (mut e, chunks) = save_engine(false);
        let (this, data) = land_with_data(&mut e);
        e.set(this, TESObjectLAND::Data, 0x1 | 0x8);
        fill_land(&mut e, data, 0.0, [0.0; 4]);
        // Quadrant 0's vertices 0 and 1 (grid points 0 and 1): heights 8 and
        // 24 (1 and 3 after >> 3); the first point of the second row (vertex
        // 17, grid point 33) is 40 (5).
        let vertices = e.get(data, LoadedLandData::ppVertices);
        let block = element(&e, vertices, 0);
        e.mem.set_f32(block + 8, 8.0);
        e.mem.set_f32(block + 12 + 8, 24.0);
        e.mem.set_f32(block + 17 * 12 + 8, 40.0);
        e.call(0x0053_8110, &args![this]);
        let chunks = chunks.borrow();
        let bytes = &chunks[2].1;
        assert_eq!(f32::from_le_bytes(bytes[..4].try_into().unwrap()), 1.0);
        assert_eq!(bytes[4], 0);
        assert_eq!(bytes[5], 2);
        // After the first row, the previous height is the row's first (1),
        // so the next row starts with a step of 5 - 1.
        assert_eq!(bytes[4 + 33], 4);
    }

    #[test]
    fn save_writes_the_texture_chunks() {
        let (mut e, chunks) = save_engine(false);
        let (this, data) = land_with_data(&mut e);
        e.set(this, TESObjectLAND::Data, 0x4 | 0x8);
        e.set_global(DEFAULT_LAND_TEXTURE, 0xdefu32);
        e.register(FORM_ID, |_, a| ret(a[0] + 0x1000_0000));
        // Quadrant 0 has its own texture and layer 2, quadrant 1 the default
        // texture and the default as layer 0, quadrant 2 nothing.
        let base = data.addr() + LoadedLandData::pDefQuadTexture.off;
        e.mem.set_u32(base, 0xa0);
        e.mem.set_u32(base + 4, 0xdef);
        let layers_of = |e: &Engine, q: u32| {
            e.mem
                .u32(data.addr() + LoadedLandData::pQuadTextureArray.off + 4 * q)
        };
        let layers = layers_of(&e, 0);
        e.mem.set_u32(layers + 8, 0xb0);
        let layers = layers_of(&e, 1);
        e.mem.set_u32(layers, 0xdef);
        e.register(LAND_LAYER_OPACITY, |_, a| {
            // Only vertex 3 of quadrant 0 layer 2 has an opacity.
            float_ret(if a[1] == 0 && a[2] == 3 && a[3] == 2 {
                0.5
            } else {
                0.0
            })
        });
        start_log(&mut e);
        e.call(0x0053_8110, &args![this]);
        let log = end_log(&mut e);
        assert_eq!(
            tags(&chunks),
            vec![
                CHUNK_DATA,
                CHUNK_BASE_TEXTURE,
                CHUNK_ADDITIONAL_TEXTURE,
                CHUNK_VERTEX_TEXTURE,
                CHUNK_ADDITIONAL_TEXTURE
            ]
        );
        let chunks = chunks.borrow();
        // BTXT: form id, quadrant 0, unused, layer 0xffff.
        assert_eq!(chunks[1].1, [0xa0, 0, 0, 0x10, 0, 0, 0xff, 0xff]);
        // ATXT: form id, quadrant 0, unused, layer 2.
        assert_eq!(chunks[2].1, [0xb0, 0, 0, 0x10, 0, 0, 2, 0]);
        // VTXT: one entry, position 3 and opacity 0.5.
        assert_eq!(chunks[3].1, [3, 0, 0, 0, 0, 0, 0, 0x3f]);
        // The default texture in a layer is written with form id 0, in
        // quadrant 1; the quadrant's own default texture is not written.
        assert_eq!(chunks[4].1, [0, 0, 0, 0, 1, 0, 0, 0]);
        assert_eq!(
            arguments_of(&log, LAND_FLUSH_WARNINGS),
            vec![vec![this.addr()]]
        );
        assert_eq!(arguments_of(&log, LAND_LAYER_OPACITY).len(), 2 * 0x121);
    }

    #[test]
    fn save_swaps_the_records_of_a_big_endian_writer() {
        let (mut e, chunks) = save_engine(true);
        let (this, data) = land_with_data(&mut e);
        e.set(this, TESObjectLAND::Data, 0x1 | 0x4 | 0x8);
        fill_land(&mut e, data, 0.0, [0.0; 4]);
        e.set_global(DEFAULT_LAND_TEXTURE, 0xdefu32);
        e.register(FORM_ID, |_, a| ret(a[0]));
        let base = data.addr() + LoadedLandData::pDefQuadTexture.off;
        e.mem.set_u32(base, 0xa0);
        let layers = e
            .mem
            .u32(data.addr() + LoadedLandData::pQuadTextureArray.off);
        e.mem.set_u32(layers, 0xb0);
        e.register(LAND_LAYER_OPACITY, |_, a| {
            float_ret(if a[2] < 2 { 1.0 } else { 0.0 })
        });
        start_log(&mut e);
        e.call(0x0053_8110, &args![this]);
        let log = end_log(&mut e);
        // The flags word and the height chunk are swapped before and after.
        assert_eq!(
            arguments_of(&log, SWAP_WORD).len(),
            4,
            "DATA and VHGT, twice each"
        );
        // Texture records: form id (32 bit) and layer (16 bit) twice for each
        // of BTXT and ATXT; VTXT entries: 2 entries, twice.
        assert_eq!(arguments_of(&log, SWAP_BYTES_32).len(), 2 + 2 + 2 * 2);
        assert_eq!(arguments_of(&log, SWAP_BYTES_16).len(), 2 + 2 + 2 * 2);
        assert_eq!(chunks.borrow().len(), 6);
    }

    #[test]
    fn save_does_nothing_without_data_to_write() {
        let (mut e, chunks) = save_engine(false);
        let this = land(&mut e);
        // No data flags that matter.
        e.set(this, TESObjectLAND::Data, 0x10);
        start_log(&mut e);
        e.call(0x0053_8110, &args![this]);
        assert!(called(&mut e).is_empty());
        // Heights, but neither the loaded bit nor the arrays.
        e.set(this, TESObjectLAND::Data, 0x1);
        start_log(&mut e);
        e.call(0x0053_8110, &args![this]);
        assert!(called(&mut e).is_empty());
        assert!(chunks.borrow().is_empty());
    }

    #[test]
    fn save_loads_the_data_first_when_it_is_not_loaded() {
        let (mut e, chunks) = save_engine(false);
        let (this, data) = land_with_data(&mut e);
        e.set(this, TESObjectLAND::Data, 0x2);
        fill_land(&mut e, data, 0.0, [0.0; 4]);
        start_log(&mut e);
        e.call(0x0053_8110, &args![this]);
        let log = end_log(&mut e);
        assert_eq!(
            arguments_of(&log, LAND_LOAD_FOR_SAVE),
            vec![vec![this.addr()]]
        );
        assert_eq!(tags(&chunks), vec![CHUNK_DATA, CHUNK_COLORS]);
        // A land the editor changed (0x10) does not need the load.
        let (mut e, _chunks) = save_engine(false);
        let (this, data) = land_with_data(&mut e);
        e.set(this, TESObjectLAND::Data, 0x12);
        fill_land(&mut e, data, 0.0, [0.0; 4]);
        start_log(&mut e);
        e.call(0x0053_8110, &args![this]);
        assert!(arguments_of(&end_log(&mut e), LAND_LOAD_FOR_SAVE).is_empty());
    }

    // ---- Havok MOPP code -------------------------------------------------------

    #[test]
    fn mopp_code_is_built_from_the_chunk() {
        let mut e = engine();
        e.register(MOPP_ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(MOPP_ARRAY_SET_SIZE, |e, a| {
            let buffer = e.mem.alloc(a[1]);
            e.mem.set_u32(a[0], buffer);
            Ret::default()
        });
        noop(
            &mut e,
            &[
                MOPP_MEMBER_CONSTRUCT_A,
                MOPP_MEMBER_CONSTRUCT_B,
                MOPP_MEMBER_FINISH,
            ],
        );
        let data = e.mem.alloc(0x18);
        let bytes: Vec<u8> = (1..=0x18).collect();
        e.mem.write(data, &bytes);
        let out = e.mem.alloc(8);
        let result = e.call(0x0053_8c00, &args![0u32, data, 0x18u32, out]);
        assert_eq!(result.u32() & 0xff, 1);
        let object = e.mem.u32(out);
        assert_ne!(object, 0);
        assert_eq!(e.mem.u32(object), HAVOK_MOPP_CODE_VTABLE);
        // The first 0x10 bytes are at +0x10, the rest in the array at +0x20.
        assert_eq!(e.mem.bytes(object + 0x10, 0x10), bytes[..0x10]);
        let buffer = e.mem.u32(object + 0x20);
        assert_eq!(e.mem.bytes(buffer, 8), bytes[0x10..]);
    }

    #[test]
    fn mopp_code_that_cannot_be_built_is_destroyed() {
        let mut e = engine();
        e.register(MOPP_ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        noop(
            &mut e,
            &[
                MOPP_MEMBER_CONSTRUCT_A,
                MOPP_MEMBER_CONSTRUCT_B,
                MOPP_MEMBER_FINISH,
            ],
        );
        let destroy = 0x0900_0003;
        e.register(destroy, |_, _| Ret::default());
        // The vtable the constructor stores is the exe's; put one with a
        // destructor in slot 0 where the exe would have it.
        e.mem.set_u32(HAVOK_MOPP_CODE_VTABLE, destroy);
        let data = e.mem.alloc(0x20);
        let out = e.mem.alloc(8);
        // Exactly 0x10 bytes: nothing is left for the array.
        start_log(&mut e);
        let result = e.call(0x0053_8c00, &args![0u32, data, 0x10u32, out]);
        let log = end_log(&mut e);
        assert_eq!(result.u32() & 0xff, 0);
        assert_eq!(e.mem.u32(out), 0);
        let object = arguments_of(&log, destroy);
        assert_eq!(object.len(), 1);
        assert_eq!(object[0][1], 1);
        // Missing inputs are refused without allocating.
        for (data, size, out) in [(0, 0x20, out), (data, 0, out), (data, 0x20, 0)] {
            start_log(&mut e);
            let result = e.call(0x0053_8c00, &args![0u32, data, size, out]);
            assert_eq!(result.u32() & 0xff, 0);
            assert!(called(&mut e).is_empty());
        }
    }

    #[test]
    fn havok_constructors_and_destructors_store_their_vtables() {
        let mut e = engine();
        noop(
            &mut e,
            &[
                MOPP_MEMBER_CONSTRUCT_A,
                MOPP_MEMBER_CONSTRUCT_B,
                MOPP_MEMBER_FINISH,
                MOPP_ARRAY_DESTRUCT,
            ],
        );
        e.register(NI_POINT3_DEFAULT_CONSTRUCT, |_, _| Ret::default());
        let object = dirty_object(&mut e, 0x30);
        start_log(&mut e);
        assert_eq!(e.call(0x0053_8d70, &args![object]).u32(), object);
        let log = end_log(&mut e);
        assert_eq!(
            log,
            vec![
                (MOPP_MEMBER_CONSTRUCT_A, vec![object + 0x10]),
                (MOPP_MEMBER_CONSTRUCT_B, vec![object + 0x20]),
                (NI_POINT3_DEFAULT_CONSTRUCT, vec![object + 0x2c]),
                (MOPP_MEMBER_FINISH, vec![object + 0x2c, 2]),
            ]
        );
        assert_eq!(e.mem.u32(object), HAVOK_MOPP_CODE_VTABLE);
        // The reference count is 1 and the four words at +0x10 are cleared.
        assert_eq!(e.mem.u16(object + 6), 1);
        assert_eq!(e.mem.bytes(object + 0x10, 16), vec![0; 16]);
        // The destructor goes back through the base classes.
        start_log(&mut e);
        e.call(0x0053_8fb0, &args![object]);
        assert_eq!(called(&mut e), vec![MOPP_ARRAY_DESTRUCT]);
        assert_eq!(e.mem.u32(object), HAVOK_BASE_OBJECT_VTABLE);
    }

    #[test]
    fn havok_base_object_functions() {
        let mut e = engine();
        let object = dirty_object(&mut e, 0x10);
        assert_eq!(e.call(0x0053_8f20, &args![object]).u32(), object);
        assert_eq!(e.mem.u32(object), HAVOK_BASE_OBJECT_VTABLE);
        let object = dirty_object(&mut e, 0x10);
        assert_eq!(e.call(0x0053_8ef0, &args![object]).u32(), object);
        assert_eq!(e.mem.u32(object), HAVOK_REFERENCED_OBJECT_VTABLE);
        assert_eq!(e.mem.u16(object + 6), 1);
        e.call(0x0053_8e30, &args![object]);
        assert_eq!(e.mem.u32(object), HAVOK_BASE_OBJECT_VTABLE);
        e.call(0x0053_8e10, &args![object]);
        // The referenced-object destructor ends with the base vtable.
        assert_eq!(e.mem.u32(object), HAVOK_BASE_OBJECT_VTABLE);
    }

    #[test]
    fn havok_base_object_deleting_destructor_frees_with_bit_zero() {
        let mut e = engine();
        let object = dirty_object(&mut e, 0x10);
        start_log(&mut e);
        assert_eq!(e.call(0x0053_8e50, &args![object, 0u32]).u32(), object);
        assert!(called(&mut e).is_empty());
        assert_eq!(e.mem.u32(object), HAVOK_BASE_OBJECT_VTABLE);
        start_log(&mut e);
        e.call(0x0053_8e50, &args![object, 1u32]);
        assert_eq!(end_log(&mut e), vec![(DEALLOCATE, vec![object])]);
    }

    #[test]
    fn havok_free_asks_the_allocator_with_the_stored_size() {
        let mut e = engine();
        let free = 0x0900_0004;
        e.register(free, |_, _| Ret::default());
        let allocator = object_with_vtable(&mut e, 0x10, &[(8, free)]);
        e.register(HAVOK_MEMORY_ROUTER, |_, _| ret(0x77));
        e.register_double(HAVOK_ALLOCATOR_OF_ROUTER, move |_, a| {
            assert_eq!(a[0], 0x77);
            ret(allocator)
        });
        let object = e.mem.alloc(0x10);
        e.mem.set_u16(object + 4, 0x30);
        start_log(&mut e);
        e.call(0x0053_8eb0, &args![object]);
        let log = end_log(&mut e);
        assert_eq!(
            arguments_of(&log, free),
            vec![vec![allocator, object, 0x30]]
        );
    }

    #[test]
    fn havok_deleting_destructors_free_with_bit_zero() {
        let mut e = engine();
        let free = 0x0900_0004;
        e.register(free, |_, _| Ret::default());
        let allocator = object_with_vtable(&mut e, 0x10, &[(8, free)]);
        e.register(HAVOK_MEMORY_ROUTER, |_, _| ret(0x77));
        e.register_double(HAVOK_ALLOCATOR_OF_ROUTER, move |_, _| ret(allocator));
        noop(&mut e, &[MOPP_ARRAY_DESTRUCT]);
        for (address, vtable) in [
            (0x0053_8e80, HAVOK_BASE_OBJECT_VTABLE),
            (0x0053_8f80, HAVOK_BASE_OBJECT_VTABLE),
        ] {
            let object = e.mem.alloc(0x40);
            e.mem.set_u16(object + 4, 0x40);
            start_log(&mut e);
            assert_eq!(e.call(address, &args![object, 0u32]).u32(), object);
            assert!(arguments_of(&end_log(&mut e), free).is_empty());
            assert_eq!(e.mem.u32(object), vtable);
            start_log(&mut e);
            e.call(address, &args![object, 1u32]);
            assert_eq!(
                arguments_of(&end_log(&mut e), free),
                vec![vec![allocator, object, 0x40]]
            );
        }
    }

    #[test]
    fn four_words_are_cleared() {
        let mut e = engine();
        let object = dirty_object(&mut e, 0x20);
        e.call(0x0053_8f40, &args![object]);
        assert_eq!(e.mem.bytes(object, 16), vec![0; 16]);
        assert_eq!(e.mem.u32(object + 16), 0xdddd_dddd);
    }

    // ---- Save references, flags and the data allocator -------------------------

    #[test]
    fn marking_asks_the_form_and_the_cell() {
        let mut e = engine();
        noop(&mut e, &[FORM_FLAG_SETTER]);
        let slot = 0x0900_0005;
        e.register(slot, |_, _| Ret::default());
        let cell = object_with_vtable(&mut e, 0x40, &[(CELL_SLOT_MARK, slot)]);
        let this = land(&mut e);
        e.set(this, TESObjectLAND::pParentCell, Ptr::new(cell));
        start_log(&mut e);
        e.call(0x0053_9010, &args![this, 0u32]);
        assert_eq!(
            end_log(&mut e),
            vec![(FORM_FLAG_SETTER, vec![this.addr(), 0])]
        );
        start_log(&mut e);
        e.call(0x0053_9010, &args![this, 1u32]);
        assert_eq!(
            end_log(&mut e),
            vec![
                (FORM_FLAG_SETTER, vec![this.addr(), 1]),
                (slot, vec![cell, 1]),
            ]
        );
        // Without a cell only the form method runs.
        e.set(this, TESObjectLAND::pParentCell, Ptr::NULL);
        start_log(&mut e);
        e.call(0x0053_9010, &args![this, 1u32]);
        assert_eq!(called(&mut e), vec![FORM_FLAG_SETTER]);
    }

    /// A land whose parent cell and child-cell part answer through vtables:
    /// the child-cell part's slot 0 gives `own_cell`, the cell's slots
    /// `0x38`, `0x3c` and `0x110` are doubles that return 0x11, 0x22 and
    /// 0x33.
    fn reference_world(e: &mut Engine) -> (Ptr<TESObjectLAND>, u32) {
        let (a, b, c) = (0x0900_0006, 0x0900_0007, 0x0900_0008);
        e.register(a, |_, _| ret(0x11));
        e.register(b, |_, _| ret(0x22));
        e.register(c, |_, _| ret(0x33));
        let cell = object_with_vtable(
            e,
            0x40,
            &[
                (CELL_SLOT_REFERENCE_OWNED, a),
                (CELL_SLOT_REFERENCE_CHECK, b),
                (CELL_SLOT_SAVE_CHECK, c),
            ],
        );
        let own_slot = 0x0900_0009;
        e.register_double(own_slot, move |_, _| ret(cell));
        let this = land(e);
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable, own_slot);
        e.mem.set_u32(this.addr() + 0x18, vtable);
        e.set(this, TESObjectLAND::pParentCell, Ptr::new(cell));
        e.set_global(SAVE_REFERENCE_MARKER, 0x5a5a_5a5au32);
        (this, cell)
    }

    fn reference_record(e: &mut Engine, kind: u32, id: u32) -> Ptr {
        let record = e.mem.alloc(0x14);
        e.mem.set_u32(record, 0x5a5a_5a5a);
        e.mem.set_u32(record + 8, id);
        e.mem.set_u32(record + 0xc, kind);
        Ptr::new(record)
    }

    #[test]
    fn reference_owned_check_follows_the_kind() {
        let mut e = engine();
        let (this, cell) = reference_world(&mut e);
        // Other kinds: the cell decides.
        let record = reference_record(&mut e, 3, 0x100);
        assert_eq!(e.call(0x0053_91d0, &args![this, record]).u32() & 0xff, 0x11);
        // Kinds 8 and 9: the form is looked up and cast; the answer is 0.
        e.register(FORM_LOOKUP_BY_ID, |_, a| ret(a[0] + 1));
        e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0]));
        for kind in [8, 9] {
            let record = reference_record(&mut e, kind, 0x100);
            start_log(&mut e);
            assert_eq!(e.call(0x0053_91d0, &args![this, record]).u32() & 0xff, 0);
            let log = end_log(&mut e);
            assert_eq!(arguments_of(&log, FORM_LOOKUP_BY_ID), vec![vec![0x100]]);
            assert_eq!(arguments_of(&log, PARENT_CELL), vec![vec![this.addr()]]);
        }
        // No record, or a record without the marker: 0.
        assert_eq!(e.call(0x0053_91d0, &args![this, 0u32]).u32() & 0xff, 0);
        let record = reference_record(&mut e, 3, 0x100);
        e.mem.set_u32(record.addr(), 0);
        assert_eq!(e.call(0x0053_91d0, &args![this, record]).u32() & 0xff, 0);
        let _ = cell;
    }

    #[test]
    fn save_check_follows_the_kind_and_the_flags() {
        let mut e = engine();
        let (this, cell) = reference_world(&mut e);
        e.register(FORM_ID_IN_CELL_FILE, |_, a| ret((a[1] == 0x100) as u32));
        // Kind 6 needs the flag.
        let record = reference_record(&mut e, 6, 0x100);
        assert_eq!(
            e.call(0x0053_9280, &args![this, record, 0u32, 0u32]).u32() & 0xff,
            0
        );
        assert_eq!(
            e.call(0x0053_9280, &args![this, record, 1u32, 0u32]).u32() & 0xff,
            1
        );
        // Kind 8 does not, and is answered 0 when the id belongs to the cell.
        let record = reference_record(&mut e, 8, 0x100);
        assert_eq!(
            e.call(0x0053_9280, &args![this, record, 0u32, 0u32]).u32() & 0xff,
            0
        );
        let record = reference_record(&mut e, 9, 0x100);
        assert_eq!(
            e.call(0x0053_9280, &args![this, record, 0u32, 0u32]).u32() & 0xff,
            1
        );
        // An id that is not the cell's: 0.
        let record = reference_record(&mut e, 9, 0x200);
        assert_eq!(
            e.call(0x0053_9280, &args![this, record, 0u32, 0u32]).u32() & 0xff,
            0
        );
        // Other kinds need the flag and the cell decides.
        let record = reference_record(&mut e, 3, 0x100);
        assert_eq!(
            e.call(0x0053_9280, &args![this, record, 0u32, 1u32]).u32() & 0xff,
            0
        );
        start_log(&mut e);
        assert_eq!(
            e.call(0x0053_9280, &args![this, record, 1u32, 0u32]).u32() & 0xff,
            0x33
        );
        let log = end_log(&mut e);
        assert_eq!(
            arguments_of(&log, 0x0900_0008),
            vec![vec![cell, record.addr(), 1, 0]]
        );
        // No marker: 0.
        e.mem.set_u32(record.addr(), 1);
        assert_eq!(
            e.call(0x0053_9280, &args![this, record, 1u32, 0u32]).u32() & 0xff,
            0
        );
    }

    #[test]
    fn reference_check_of_a_form_follows_its_type() {
        let mut e = engine();
        let (this, _cell) = reference_world(&mut e);
        e.register(FORM_TYPE, |e, a| ret(e.mem.u8(a[0] + 4) as u32));
        e.register(TYPE_IS_REFERENCE_KIND, |_, a| ret((a[0] != 0x20) as u32));
        // A form of an unrelated type: the cell decides, asked about the form.
        let form = e.mem.alloc(0x20);
        e.mem.set_u8(form + 4, 0x20);
        start_log(&mut e);
        assert_eq!(e.call(0x0053_9060, &args![this, form]).u32() & 0xff, 0x22);
        assert_eq!(
            arguments_of(&end_log(&mut e), 0x0900_0007),
            vec![vec![e.mem.u32(this.addr() + 0x20), form]]
        );
        // A reference kind whose child-cell owner is the land's own cell.
        let owner = 0x0900_000a;
        let own_cell = e.mem.u32(this.addr() + 0x20);
        e.register_double(owner, move |_, _| ret(own_cell));
        let owner_object = object_with_vtable(&mut e, 0x10, &[(0, owner)]);
        e.register_double(RT_DYNAMIC_CAST, move |_, a| {
            // To a TESChildCell: the owner object; to a TESObjectREFR: the form.
            ret(if a[3] == RTTI_TES_CHILD_CELL {
                owner_object
            } else {
                a[0]
            })
        });
        e.register(GET_REF_PERSISTS, |e, a| ret(e.mem.u8(a[0] + 8) as u32));
        for (kind, persists, expected) in [
            (0x3a, 0, 1),
            (0x40, 0, 1),
            (0x69, 0, 1),
            (0x3a, 1, 0),
            (0x43, 0, 1),
            (0x41, 0, 0),
            (0x6a, 0, 0),
        ] {
            let form = e.mem.alloc(0x20);
            e.mem.set_u8(form + 4, kind);
            e.mem.set_u8(form + 8, persists);
            assert_eq!(
                e.call(0x0053_9060, &args![this, form]).u32() & 0xff,
                expected,
                "type {kind:#x} persists {persists}"
            );
        }
        // Another cell owns the form: the land's cell decides, asked about
        // the other cell.
        let other_cell = 0x4242;
        e.register_double(owner, move |_, _| ret(other_cell));
        let form = e.mem.alloc(0x20);
        e.mem.set_u8(form + 4, 0x3a);
        start_log(&mut e);
        assert_eq!(e.call(0x0053_9060, &args![this, form]).u32() & 0xff, 0x22);
        assert_eq!(
            arguments_of(&end_log(&mut e), 0x0900_0007),
            vec![vec![own_cell, other_cell]]
        );
    }

    #[test]
    fn reference_translation_builds_the_save_record() {
        let mut e = engine();
        let (this, cell) = reference_world(&mut e);
        e.register(FORM_ID, |_, a| ret(a[0] + 5));
        e.register(CELL_SAVE_ID, |_, a| ret(a[0] + 7));
        let out = dirty_object(&mut e, 0x14);
        let out: Ptr = Ptr::new(out);
        // Kind 3 with the cell's save id becomes kind 6 with the form id.
        let record = reference_record(&mut e, 3, cell + 7);
        e.call(0x0053_9360, &args![this, out, record]);
        let words = |e: &Engine| {
            (0..5)
                .map(|i| e.mem.u32(out.addr() + 4 * i))
                .collect::<Vec<_>>()
        };
        assert_eq!(words(&e), vec![0x5a5a_5a5a, 0, cell + 5, 6, 0]);
        // Kind 5 with another id leaves only the cleared marker.
        let record = reference_record(&mut e, 5, 1);
        e.call(0x0053_9360, &args![this, out, record]);
        assert_eq!(e.mem.u32(out.addr()), 0);
        // Kind 6 with the form id becomes kind 9.
        let record = reference_record(&mut e, 6, cell + 5);
        e.call(0x0053_9360, &args![this, out, record]);
        assert_eq!(words(&e), vec![0x5a5a_5a5a, 0, cell + 5, 9, 0]);
        // Kind 6 with another id, or an unknown kind: nothing.
        for (kind, id) in [(6, 1), (4, cell + 5)] {
            let record = reference_record(&mut e, kind, id);
            e.call(0x0053_9360, &args![this, out, record]);
            assert_eq!(e.mem.u32(out.addr()), 0);
        }
        // No source record: the marker is cleared; no destination: nothing.
        e.mem.set_u32(out.addr(), 9);
        e.call(0x0053_9360, &args![this, out, 0u32]);
        assert_eq!(e.mem.u32(out.addr()), 0);
        start_log(&mut e);
        e.call(0x0053_9360, &args![this, 0u32, record]);
        assert!(called(&mut e).is_empty());
    }

    #[test]
    fn data_flag_tests_and_setter() {
        let mut e = engine();
        let this = land(&mut e);
        for (data, expected) in [(0u32, 0), (1, 1), (2, 1), (4, 1), (0x18, 0), (0x1c, 1)] {
            e.set(this, TESObjectLAND::Data, data);
            assert_eq!(
                e.call(0x0053_9460, &args![this]).u32() & 0xff,
                expected,
                "{data:#x}"
            );
        }
        e.set(this, TESObjectLAND::Data, 0x7);
        assert_eq!(e.call(0x0053_94a0, &args![this]).u32() & 0xff, 0);
        e.call(0x0053_94c0, &args![this, 1u32]);
        assert_eq!(e.get(this, TESObjectLAND::Data), 0xf);
        assert_eq!(e.call(0x0053_94a0, &args![this]).u32() & 0xff, 1);
        e.call(0x0053_94c0, &args![this, 0u32]);
        assert_eq!(e.get(this, TESObjectLAND::Data), 0x7);
    }

    /// The default blocks of the shared statics, each starting with a marker
    /// word.
    fn default_blocks(e: &mut Engine) {
        for quadrant in 0..4 {
            let block = e.mem.alloc(0xd8c);
            e.mem.set_u32(block, 0xa0 + quadrant);
            e.mem.set_f32(block + 8, 1.5);
            e.set_global(DEFAULT_VERTEX_BLOCKS + 4 * quadrant, block);
        }
        for (global, marker, size) in [
            (DEFAULT_COLORS, 0xc0u32, 0x1210u32),
            (DEFAULT_NORMALS, 0xd0, 0xd8c),
            (DEFAULT_NORMAL_SET, 0xe0, 0x121),
        ] {
            let block = e.mem.alloc(size);
            e.mem.set_u32(block, marker);
            e.set_global(global, block);
        }
        e.set_global(DEFAULT_LAND_TEXTURE, 0xdefu32);
    }

    fn allocation_world(e: &mut Engine) {
        default_blocks(e);
        e.register(LOADED_LAND_DATA_CONSTRUCT, |_, a| ret(a[0]));
        e.register(CELL_GET_DATA_X, |_, _| ret(-7i32 as u32));
        e.register(CELL_GET_DATA_Y, |_, _| ret(9));
        e.register(LAND_DEFAULT_HEIGHT, |_, _| float_ret(7.0));
    }

    #[test]
    fn data_allocation_copies_the_defaults_into_a_fresh_land() {
        let mut e = engine();
        allocation_world(&mut e);
        let this = land(&mut e);
        e.set(this, TESObjectLAND::pParentCell, Ptr::new(0x6000));
        assert_eq!(e.call(0x0053_9500, &args![this, 0u32]).u32() & 0xff, 1);
        let data = loaded_data(&e, this);
        assert_eq!(e.get(data, LoadedLandData::iCellX), -7);
        assert_eq!(e.get(data, LoadedLandData::iCellY), 9);
        let vertices = e.get(data, LoadedLandData::ppVertices);
        let colors = e.get(data, LoadedLandData::ppColorsA);
        let normals = e.get(data, LoadedLandData::ppNormals);
        let flags = e.get(data, LoadedLandData::ppNormalsSet);
        for quadrant in 0..4 {
            // Each quadrant has its own copy of its default vertex block,
            // with the default height in every Z.
            let block = element(&e, vertices, quadrant);
            assert_eq!(e.mem.u32(block), 0xa0 + quadrant);
            assert_eq!(e.mem.f32(block + 8), 7.0);
            assert_eq!(e.mem.f32(block + 0x120 * 12 + 8), 7.0);
            assert_eq!(e.mem.u32(element(&e, colors, quadrant)), 0xc0);
            assert_eq!(e.mem.u32(element(&e, normals, quadrant)), 0xd0);
            assert_eq!(e.mem.u32(element(&e, flags, quadrant)), 0xe0);
            // The default texture, an empty set of layer textures, and the
            // percent array pointing at 0x121 entries of 0x20 bytes.
            assert_eq!(
                e.mem
                    .u32(data.addr() + LoadedLandData::pDefQuadTexture.off + 4 * quadrant),
                0xdef
            );
            let layers = e
                .mem
                .u32(data.addr() + LoadedLandData::pQuadTextureArray.off + 4 * quadrant);
            assert_eq!(e.mem.bytes(layers, 0x18), vec![0; 0x18]);
            let percent = e
                .mem
                .u32(data.addr() + LoadedLandData::ppPercentArrays.off + 4 * quadrant);
            let first = e.mem.u32(percent);
            assert_eq!(e.mem.u32(percent + 4 * 0x120), first + 0x20 * 0x120);
            assert_eq!(e.mem.block_size(first), Some(0x2420));
        }
        // A land that already has vertex arrays is left alone.
        start_log(&mut e);
        assert_eq!(e.call(0x0053_9500, &args![this, 0u32]).u32() & 0xff, 0);
        assert!(called(&mut e).is_empty());
    }

    #[test]
    fn data_allocation_keeps_the_default_vertices_for_height_minus_2048() {
        let mut e = engine();
        allocation_world(&mut e);
        e.register(LAND_DEFAULT_HEIGHT, |_, _| float_ret(-2048.0));
        let this = land(&mut e);
        e.call(0x0053_9500, &args![this, 0u32]);
        let data = loaded_data(&e, this);
        let vertices = e.get(data, LoadedLandData::ppVertices);
        assert_eq!(e.mem.f32(element(&e, vertices, 1) + 8), 1.5);
    }

    #[test]
    fn data_allocation_for_a_copy_leaves_the_blocks_empty() {
        let mut e = engine();
        allocation_world(&mut e);
        let (source, _data) = land_with_data(&mut e);
        let this = land(&mut e);
        start_log(&mut e);
        assert_eq!(e.call(0x0053_9500, &args![this, source]).u32() & 0xff, 1);
        let log = end_log(&mut e);
        assert!(arguments_of(&log, MEMORY_COPY).is_empty());
        assert!(arguments_of(&log, MEMORY_SET).is_empty());
        let data = loaded_data(&e, this);
        let vertices = e.get(data, LoadedLandData::ppVertices);
        assert_eq!(e.mem.u32(element(&e, vertices, 0)), 0);
        // The arrays are still there for the caller to fill.
        assert_eq!(arguments_of(&log, ALLOCATE).len(), 1 + 4 + 4 * 6);
    }

    // ---- Quadrant materials ----------------------------------------------------

    /// A land with four mesh nodes and the doubles of the material builder.
    fn material_world(e: &mut Engine) -> (Ptr<TESObjectLAND>, Ptr<LoadedLandData>, Vec<u32>) {
        let (this, data) = land_with_data(e);
        e.set_global(DEFAULT_LAND_TEXTURE, 0xdefu32);
        let meshes = e.mem.alloc(16);
        let nodes: Vec<u32> = (0..4).map(|_| e.mem.alloc(0x20)).collect();
        for (i, node) in nodes.iter().enumerate() {
            e.mem.set_u32(meshes + 4 * i as u32, *node);
        }
        e.set(data, LoadedLandData::ppMesh, Ptr::new(meshes));
        let tangent_slot = 0x0900_000b;
        e.register(tangent_slot, |_, _| ret(0x7777));
        let vtable = e.mem.alloc(0x200);
        e.mem
            .set_u32(vtable + PROPERTY_SLOT_TANGENT_SPACE, tangent_slot);
        e.register(NI_ALLOCATE_OBJECT, |e, a| ret(e.mem.alloc(a[0])));
        e.register_double(SHADER_PROPERTY_CONSTRUCT, move |e, a| {
            e.mem.set_u32(a[0], vtable);
            ret(a[0])
        });
        e.register(PERCENT_EXTRA_DATA_CONSTRUCT, |_, a| ret(a[0]));
        e.set_global(EXTRA_DATA_KEY, 0x4b4bu32);
        noop(
            e,
            &[
                OBJECT_ADD_EXTRA_DATA,
                SHADER_PROPERTY_SET_WORD_0X58,
                SHADER_PROPERTY_SET_TEXTURE_SET,
                SHADER_PROPERTY_SET_FLAGS,
                TEXTURING_PROPERTY_SET,
                GEOMETRY_DATA_SET_SHARED,
                NODE_ATTACH_PROPERTY,
                PREPARE_OBJECT,
            ],
        );
        // Texture 0 has no texture set; the others have one at +0x18.
        e.register(OBJECT_FIELD_0X18, |_, a| {
            ret(if a[0] == 0x0bad { 0 } else { a[0] + 0x100 })
        });
        e.register(TEXTURE_SET_AS_SHADER_SET, |_, a| ret(a[0] + 1));
        e.register(LAND_TEXTURE_FLAG_BYTE, |_, a| ret(0x100 | (a[0] & 0xff)));
        // Each mesh's first geometry holds a geometry data at +0xb8.
        e.register(NODE_FIRST_GEOMETRY, |e, _| {
            let geometry = e.mem.alloc(0x100);
            e.mem.set_u32(geometry + 0xb8, 0x3030);
            ret(geometry)
        });
        e.register(CREATE_TANGENT_SPACE_SIMPLE, |_, a| ret(a[0] + 1));
        (this, data, nodes)
    }

    #[test]
    fn materials_are_built_for_every_quadrant() {
        let mut e = engine();
        let (this, data, nodes) = material_world(&mut e);
        e.set(this, TESObjectLAND::Data, 0x1);
        // Quadrant 0: its own default texture and layers 1 and 3; quadrant 1
        // a default texture without a texture set.
        let base = data.addr() + LoadedLandData::pDefQuadTexture.off;
        e.mem.set_u32(base, 0xa0);
        e.mem.set_u32(base + 4, 0x0bad);
        let layers = e
            .mem
            .u32(data.addr() + LoadedLandData::pQuadTextureArray.off);
        e.mem.set_u32(layers + 4, 0x31);
        e.mem.set_u32(layers + 12, 0x33);
        start_log(&mut e);
        assert_eq!(e.call(0x0053_9960, &args![this]).u32() & 0xff, 1);
        let log = end_log(&mut e);
        // The guard is entered with the source line of the function.
        assert_eq!(
            arguments_of(&log, SCOPE_GUARD_ENTER)[0][1..],
            [GUARD_TAG, 1, SOURCE_FILE, 0xca4]
        );
        assert!(e.get(this, TESObjectLAND::Data) & FLAG_LOADED != 0);
        // The vertex colours are white with alpha 0 (no colour flag).
        let colors = e.get(data, LoadedLandData::ppColorsA);
        for quadrant in 0..4 {
            let block = element(&e, colors, quadrant);
            for vertex in [0, 0x120] {
                let at = block + vertex * 16;
                assert_eq!(
                    (0..4).map(|i| e.mem.f32(at + 4 * i)).collect::<Vec<_>>(),
                    vec![1.0, 1.0, 1.0, 0.0]
                );
            }
        }
        // Extra data: one per mesh, built from the first percent block.
        let extra = arguments_of(&log, OBJECT_ADD_EXTRA_DATA);
        assert_eq!(extra.len(), 4);
        for (quadrant, call) in extra.iter().enumerate() {
            let percent = e
                .mem
                .u32(data.addr() + LoadedLandData::ppPercentArrays.off + 4 * quadrant as u32);
            let first = e.mem.u32(percent);
            assert_eq!(call[0], nodes[quadrant]);
            assert_eq!(call[1], 0x4b4b);
            assert_eq!(
                arguments_of(&log, PERCENT_EXTRA_DATA_CONSTRUCT)[quadrant][1..],
                [0x2420, first]
            );
        }
        // Texture sets of quadrant 0: its own texture in slot 0, layers 1 and
        // 3 in slots 2 and 4, nothing elsewhere.
        let sets = arguments_of(&log, SHADER_PROPERTY_SET_TEXTURE_SET);
        assert_eq!(sets.len(), 4 * 7);
        let slots: Vec<(u32, u32)> = sets[..7].iter().map(|a| (a[1], a[2])).collect();
        assert_eq!(
            slots,
            vec![
                (0, 0xa0 + 0x100 + 1),
                (1, 0),
                (2, 0x31 + 0x100 + 1),
                (3, 0),
                (4, 0x33 + 0x100 + 1),
                (5, 0),
                (6, 0)
            ]
        );
        // Quadrant 1's texture has no set, so slot 0 gets the default land
        // texture's.
        assert_eq!(sets[7][1..], [0, 0xdef + 0x100 + 1]);
        // The flags: default texture first, then layers 0 to 5 (the layer
        // order of the call is the reverse of the order they were read in).
        let flags = arguments_of(&log, SHADER_PROPERTY_SET_FLAGS);
        assert_eq!(flags.len(), 4);
        assert_eq!(flags[0][1..], [0xa0, 0, 0x31, 0, 0x33, 0, 0, 0, 0, 0]);
        assert_eq!(flags[1][1], 0x0bad & 0xff);
        // Tangent space, hand-over, attach and prepare of the first mesh.
        let geometry = arguments_of(&log, CREATE_TANGENT_SPACE_SIMPLE)[0][0];
        assert_eq!(
            arguments_of(&log, NODE_FIRST_GEOMETRY)[0],
            vec![nodes[0], 0]
        );
        let property = flags[0][0];
        assert_eq!(
            arguments_of(&log, TEXTURING_PROPERTY_SET)[0],
            vec![property + 0xd0, geometry + 1]
        );
        assert_eq!(
            arguments_of(&log, GEOMETRY_DATA_SET_SHARED)[0],
            vec![0x3030, 0x7777]
        );
        assert_eq!(
            arguments_of(&log, NODE_ATTACH_PROPERTY)[0],
            vec![geometry, property]
        );
        assert_eq!(arguments_of(&log, PREPARE_OBJECT)[0], vec![nodes[0], 0, 0]);
    }

    #[test]
    fn materials_keep_the_colors_of_a_land_with_vertex_colors() {
        let mut e = engine();
        let (this, data, _nodes) = material_world(&mut e);
        e.set(this, TESObjectLAND::Data, 0x2);
        let colors = e.get(data, LoadedLandData::ppColorsA);
        e.mem.set_f32(element(&e, colors, 0), 0.25);
        assert_eq!(e.call(0x0053_9960, &args![this]).u32() & 0xff, 1);
        assert_eq!(e.mem.f32(element(&e, colors, 0)), 0.25);
    }

    #[test]
    fn materials_need_loaded_meshes() {
        let mut e = engine();
        let (this, data, _nodes) = material_world(&mut e);
        let meshes = e.get(data, LoadedLandData::ppMesh);
        e.mem.set_u32(meshes.addr(), 0);
        start_log(&mut e);
        assert_eq!(e.call(0x0053_9960, &args![this]).u32() & 0xff, 0);
        assert_eq!(called(&mut e), vec![SCOPE_GUARD_ENTER, SCOPE_GUARD_LEAVE]);
        e.set(data, LoadedLandData::ppMesh, Ptr::NULL);
        assert_eq!(e.call(0x0053_9960, &args![this]).u32() & 0xff, 0);
        e.set(this, TESObjectLAND::pLoadedData, Ptr::NULL);
        assert_eq!(e.call(0x0053_9960, &args![this]).u32() & 0xff, 0);
    }

    #[test]
    fn geometry_and_property_helpers() {
        let mut e = engine();
        noop(&mut e, &[GEOMETRY_DATA_SET_SHARED, TEXTURING_PROPERTY_SET]);
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object + 0xb8, 0x3333);
        start_log(&mut e);
        e.call(0x0053_9ef0, &args![object, 0x99u32]);
        assert_eq!(
            end_log(&mut e),
            vec![
                (SMART_POINTER_GET, vec![object + 0xb8]),
                (GEOMETRY_DATA_SET_SHARED, vec![0x3333, 0x99])
            ]
        );
        start_log(&mut e);
        e.call(0x0053_9f20, &args![object, 0x55u32]);
        assert_eq!(
            end_log(&mut e),
            vec![(TEXTURING_PROPERTY_SET, vec![object + 0xd0, 0x55])]
        );
        e.set_global(EXTRA_DATA_KEY, 0x1234u32);
        assert_eq!(e.call(0x0053_9f40, &args![]).u32(), 0x1234);
    }

    // ---- Third session: helpers ---------------------------------------------------

    /// A land with the loaded data of `land_with_data` and, per quadrant, a
    /// full percent array (289 rows of eight floats).
    fn percent_land(e: &mut Engine) -> (Ptr<TESObjectLAND>, Ptr<LoadedLandData>) {
        let (this, data) = land_with_data(e);
        for quadrant in 0..4u32 {
            let percent = e.mem.alloc(0x484);
            let rows = e.mem.alloc(0x2420);
            for vertex in 0..0x121u32 {
                e.mem.set_u32(percent + 4 * vertex, rows + 0x20 * vertex);
            }
            e.mem.set_u32(
                data.addr() + LoadedLandData::ppPercentArrays.off + 4 * quadrant,
                percent,
            );
        }
        (this, data)
    }

    fn opacity_row(e: &Engine, data: Ptr<LoadedLandData>, quadrant: u32, vertex: u32) -> u32 {
        let percent = percent_array(e, data, quadrant);
        percent_row(e, percent, vertex)
    }

    fn set_opacities(
        e: &mut Engine,
        data: Ptr<LoadedLandData>,
        quadrant: u32,
        vertex: u32,
        values: &[f32],
    ) {
        let row = opacity_row(e, data, quadrant, vertex);
        for (i, value) in values.iter().enumerate() {
            e.mem.set_f32(row + 4 * i as u32, *value);
        }
    }

    fn opacities(e: &Engine, data: Ptr<LoadedLandData>, quadrant: u32, vertex: u32) -> Vec<f32> {
        let row = opacity_row(e, data, quadrant, vertex);
        floats(e, row, 8)
    }

    fn set_layers(e: &mut Engine, data: Ptr<LoadedLandData>, quadrant: u32, textures: &[u32]) {
        let layers = layer_array(e, data, quadrant);
        for (i, texture) in textures.iter().enumerate() {
            e.mem.set_u32(layers + 4 * i as u32, *texture);
        }
    }

    fn layers(e: &Engine, data: Ptr<LoadedLandData>, quadrant: u32) -> Vec<u32> {
        let layers = layer_array(e, data, quadrant);
        (0..6).map(|i| e.mem.u32(layers + 4 * i)).collect()
    }

    fn set_default_texture(e: &mut Engine, data: Ptr<LoadedLandData>, quadrant: u32, texture: u32) {
        e.mem.set_u32(
            data.addr() + LoadedLandData::pDefQuadTexture.off + 4 * quadrant,
            texture,
        );
    }

    fn default_texture(e: &Engine, data: Ptr<LoadedLandData>, quadrant: u32) -> u32 {
        e.mem
            .u32(data.addr() + LoadedLandData::pDefQuadTexture.off + 4 * quadrant)
    }

    // ---- Binary extra data -----------------------------------------------------------

    #[test]
    fn extra_data_constructor_stores_vtable_size_and_data() {
        let mut e = engine();
        e.register(EXTRA_DATA_CONSTRUCT, |_, a| ret(a[0]));
        let object = e.mem.alloc(0x14);
        start_log(&mut e);
        let result = e.call(0x0053_9f50, &args![object, 0x2420u32, 0x4000u32]);
        assert_eq!(result.u32(), object);
        assert_eq!(called(&mut e), vec![EXTRA_DATA_CONSTRUCT]);
        assert_eq!(e.mem.u32(object), BINARY_EXTRA_DATA_VTABLE);
        assert_eq!(e.mem.u32(object + 0x10), 0x2420);
        assert_eq!(e.mem.u32(object + 0xc), 0x4000);
    }

    #[test]
    fn extra_data_setter_sets_size_and_data() {
        let mut e = engine();
        let object = e.mem.alloc(0x14);
        e.call(0x0053_a070, &args![object, 7u32, 0x1234u32]);
        assert_eq!(e.mem.u32(object + 0x10), 7);
        assert_eq!(e.mem.u32(object + 0xc), 0x1234);
    }

    #[test]
    fn extra_data_rtti_is_the_class_descriptor() {
        let mut e = engine();
        assert_eq!(e.call(0x0053_9fc0, &args![]).u32(), 0x011f_4ab8);
    }

    #[test]
    fn extra_data_destructor_frees_the_data_and_the_object_on_request() {
        let mut e = engine();
        noop(&mut e, &[FREE_BLOCK, EXTRA_DATA_DESTRUCT, NI_FREE]);
        let object = e.mem.alloc(0x14);
        e.mem.set_u32(object + 0xc, 0x4444);
        start_log(&mut e);
        e.call(0x0053_a000, &args![object]);
        assert_eq!(
            end_log(&mut e),
            vec![
                (FREE_BLOCK, vec![0x4444]),
                (EXTRA_DATA_DESTRUCT, vec![object])
            ]
        );
        assert_eq!(e.mem.u32(object), BINARY_EXTRA_DATA_VTABLE);
        assert_eq!(e.mem.u32(object + 0xc), 0);
        // The scalar deleting destructor frees the 0x14 bytes only for bit 0.
        start_log(&mut e);
        let result = e.call(0x0053_9fd0, &args![object, 1u32]);
        assert_eq!(result.u32(), object);
        let log = end_log(&mut e);
        assert_eq!(log.last().unwrap(), &(NI_FREE, vec![object, 0x14]));
        start_log(&mut e);
        e.call(0x0053_9fd0, &args![object, 0u32]);
        assert!(!called(&mut e).contains(&NI_FREE));
    }

    // ---- UpdateMesh ------------------------------------------------------------------------

    /// A land with a parent cell, loaded data with one mesh array whose node
    /// for quadrant 1 exists; the doubles of the update. `kind` is the
    /// shader property's type. Returns the land, its data, the node, the
    /// property and the locked buffer.
    fn update_world(
        e: &mut Engine,
        kind: u32,
    ) -> (Ptr<TESObjectLAND>, Ptr<LoadedLandData>, u32, u32, u32) {
        let (this, data) = percent_land(e);
        e.set(this, TESObjectLAND::pParentCell, Ptr::new(0xce11));
        let meshes = e.mem.alloc(16);
        e.set(data, LoadedLandData::ppMesh, Ptr::new(meshes));
        e.set_global(DEFAULT_LAND_TEXTURE, 0xdefu32);
        let node = e.mem.alloc(0x20);
        let geometry = e.mem.alloc(0x100);
        e.mem.set_u32(geometry + 0xb8, 0x3030);
        let child_vtable = e.mem.alloc(0x40);
        e.mem
            .set_u32(child_vtable + NODE_SLOT_AS_GEOMETRY, 0x0900_0018);
        let child = e.mem.alloc(0x20);
        e.mem.set_u32(child, child_vtable);
        e.register_double(0x0900_0018, move |_, _| ret(geometry));
        e.register_double(CELL_GET_NODE, move |_, a| {
            ret(if a[1] == 1 { node } else { 0 })
        });
        e.register_double(NODE_FIRST_GEOMETRY, move |_, _| ret(child));
        let property_vtable = e.mem.alloc(0x100);
        e.mem
            .set_u32(property_vtable + SHADER_PROPERTY_SLOT_UPDATE, 0x0900_009c);
        let property = e.mem.alloc(0x20);
        e.mem.set_u32(property, property_vtable);
        let locked = e.mem.alloc(0xd8c);
        e.register(TRI_STRIPS_VERTEX_COUNT, |_, _| ret(0x121));
        e.register(GEOMETRY_GET_POSITIONS, |_, _| ret(0x5000));
        e.register(GETTER_00460140, |_, _| ret(0x6000));
        e.register_double(GEOMETRY_GET_DATA, |_, _| ret(0x7000));
        e.register_double(OBJECT_GET_PROPERTY, move |_, _| ret(property));
        e.register_double(PROPERTY_GET_TYPE, move |_, _| ret(kind));
        e.register_double(GEOMETRY_DATA_LOCKED_POINTER, move |_, _| ret(locked));
        e.register(OBJECT_FIELD_0X18, |_, a| {
            ret(if a[0] == 0x0bad { 0 } else { a[0] + 0x100 })
        });
        e.register(TEXTURE_SET_AS_SHADER_SET, |_, a| ret(a[0] + 1));
        e.register(LAND_TEXTURE_FLAG_BYTE, |_, a| ret(0x100 | (a[0] & 0xff)));
        e.register(EXTERIOR_LOADER_FLAG, |_, _| ret(0x1fe));
        e.set_global(EXTERIOR_LOADER, 0x0777u32);
        noop(
            e,
            &[
                BOUND_COMPUTE_FROM_DATA,
                GEOMETRY_DATA_MARK_CHANGED,
                GEOMETRY_DATA_UNLOCK,
                0x0900_009c,
                SHADER_PROPERTY_SET_TEXTURE_SET,
                SHADER_PROPERTY_SET_FLAGS,
                SHADER_PROPERTY_FREE_RENDER_PASSES,
                UPDATE_DATA_CONSTRUCT,
                NODE_UPDATE,
                NODE_UPDATE_PROPERTIES,
                LAND_FOLLOW_UP,
            ],
        );
        (this, data, node, property, locked)
    }

    #[test]
    fn update_mesh_refreshes_the_materials_of_a_quadrant() {
        let mut e = engine();
        let (this, data, node, property, locked) = update_world(&mut e, 9);
        // The normals of quadrant 1 are copied into the locked data.
        let normals = e.get(data, LoadedLandData::ppNormals);
        let block = element(&e, normals, 1);
        e.mem.set_u32(block, 0xabcd_0123);
        e.mem.set_u32(block + 0xd88, 0x7777_8888);
        set_default_texture(&mut e, data, 1, 0xa0);
        set_layers(&mut e, data, 1, &[0xb0, 0, 0x0bad, 0, 0, 0xb5]);
        start_log(&mut e);
        e.call(0x0053_a090, &args![this, 1u32, 0u32]);
        let log = end_log(&mut e);
        assert_eq!(e.mem.u32(locked), 0xabcd_0123);
        assert_eq!(e.mem.u32(locked + 0xd88), 0x7777_8888);
        assert_eq!(
            arguments_of(&log, BOUND_COMPUTE_FROM_DATA),
            vec![vec![0x6000, 0x121, 0x5000]]
        );
        assert_eq!(
            arguments_of(&log, GEOMETRY_DATA_MARK_CHANGED),
            vec![vec![0x7000, 0xf]]
        );
        assert_eq!(
            arguments_of(&log, GEOMETRY_DATA_UNLOCK),
            vec![vec![0x7000, 0]]
        );
        // Slot 0 from the default texture, slots 1 to 6 from the layers
        // (a texture without a texture set gives 0).
        let sets = arguments_of(&log, SHADER_PROPERTY_SET_TEXTURE_SET);
        assert_eq!(
            sets,
            vec![
                vec![property, 0, 0xa0 + 0x100 + 1],
                vec![property, 1, 0xb0 + 0x100 + 1],
                vec![property, 2, 0],
                vec![property, 3, 0],
                vec![property, 4, 0],
                vec![property, 5, 0],
                vec![property, 6, 0xb5 + 0x100 + 1],
            ]
        );
        assert_eq!(
            arguments_of(&log, SHADER_PROPERTY_SET_FLAGS),
            vec![vec![property, 0xa0, 0xb0, 0, 0xad, 0, 0, 0xb5, 0, 0, 0]]
        );
        assert_eq!(
            arguments_of(&log, SHADER_PROPERTY_FREE_RENDER_PASSES),
            vec![vec![property]]
        );
        assert_eq!(arguments_of(&log, NODE_UPDATE)[0][0], node);
        assert_eq!(arguments_of(&log, NODE_UPDATE_PROPERTIES), vec![vec![node]]);
        assert_eq!(
            arguments_of(&log, LAND_FOLLOW_UP),
            vec![vec![this.addr(), 0xfe]]
        );
        assert_eq!(arguments_of(&log, EXTERIOR_LOADER_FLAG), vec![vec![0x0777]]);
    }

    #[test]
    fn update_mesh_uses_the_default_land_texture_without_a_quadrant_texture() {
        let mut e = engine();
        let (this, data, _node, property, _locked) = update_world(&mut e, 12);
        set_default_texture(&mut e, data, 1, 0);
        start_log(&mut e);
        e.call(0x0053_a090, &args![this, 1u32, 0u32]);
        let log = end_log(&mut e);
        assert_eq!(
            arguments_of(&log, SHADER_PROPERTY_SET_TEXTURE_SET)[0],
            vec![property, 0, 0xdef + 0x100 + 1]
        );
    }

    #[test]
    fn update_mesh_leaves_other_property_types_alone() {
        let mut e = engine();
        let (this, _data, node, _property, locked) = update_world(&mut e, 7);
        start_log(&mut e);
        e.call(0x0053_a090, &args![this, 1u32, 0u32]);
        let log = end_log(&mut e);
        assert_eq!(e.mem.u32(locked), 0);
        assert!(arguments_of(&log, SHADER_PROPERTY_SET_TEXTURE_SET).is_empty());
        assert!(arguments_of(&log, SHADER_PROPERTY_FREE_RENDER_PASSES).is_empty());
        // The node is still updated, with its properties.
        assert_eq!(arguments_of(&log, NODE_UPDATE)[0][0], node);
        assert_eq!(arguments_of(&log, NODE_UPDATE_PROPERTIES), vec![vec![node]]);
        // A type above 12 is left alone as well.
        let mut e = engine();
        let (this, _data, _node, _property, _locked) = update_world(&mut e, 13);
        start_log(&mut e);
        e.call(0x0053_a090, &args![this, 1u32, 0u32]);
        assert!(arguments_of(&end_log(&mut e), SHADER_PROPERTY_SET_FLAGS).is_empty());
    }

    #[test]
    fn update_mesh_without_materials_only_updates_the_nodes() {
        let mut e = engine();
        let (this, _data, node, _property, _locked) = update_world(&mut e, 9);
        start_log(&mut e);
        e.call(0x0053_a090, &args![this, 0u32, 0u32]);
        let log = end_log(&mut e);
        let addresses: Vec<u32> = log.iter().map(|(a, _)| *a).collect();
        assert_eq!(
            addresses,
            vec![
                CELL_GET_NODE,
                CELL_GET_NODE,
                UPDATE_DATA_CONSTRUCT,
                NODE_UPDATE,
                CELL_GET_NODE,
                CELL_GET_NODE
            ]
        );
        assert_eq!(log[3].1[0], node);
        // The update data is constructed with zero time.
        assert_eq!(log[2].1[1..], [0, 0, 0]);
    }

    #[test]
    fn update_mesh_without_meshes_allocates_the_data_instead() {
        let mut e = engine();
        // Loaded data with vertex arrays (so the allocator returns at once)
        // but no mesh array, and no parent cell (so no meshes are built).
        let (this, data) = land_with_data(&mut e);
        assert!(e.get(data, LoadedLandData::ppMesh).is_null());
        start_log(&mut e);
        e.call(0x0053_a090, &args![this, 1u32, 0u32]);
        assert!(called(&mut e).is_empty());
    }

    // ---- Coordinate records, default height --------------------------------------------------

    #[test]
    fn coord_data_constructor_constructs_its_four_members() {
        let mut e = engine();
        let info = e.mem.alloc(0x50);
        start_log(&mut e);
        let result = e.call(0x0053_a510, &args![info]);
        assert_eq!(result.u32(), info);
        let log = end_log(&mut e);
        assert_eq!(
            log,
            vec![
                (NI_POINT3_DEFAULT_CONSTRUCT, vec![info]),
                (NI_POINT3_DEFAULT_CONSTRUCT, vec![info + 8]),
                (NI_POINT3_DEFAULT_CONSTRUCT, vec![info + 0x1c]),
                (NI_POINT3_DEFAULT_CONSTRUCT, vec![info + 0x30]),
            ]
        );
    }

    #[test]
    fn default_height_comes_from_the_world_space() {
        let mut e = engine();
        e.register(CELL_GET_WORLD_SPACE, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(WORLD_SPACE_DEFAULT_HEIGHT, |e, a| {
            float_ret(e.mem.f32(a[0] + 8))
        });
        let this = land(&mut e);
        // No cell: -2048.
        assert_eq!(e.call(0x0053_a550, &args![this]).f32(), -2048.0);
        // A cell without a world space: -2048.
        let cell = e.mem.alloc(0x20);
        e.set(this, TESObjectLAND::pParentCell, Ptr::new(cell));
        assert_eq!(e.call(0x0053_a550, &args![this]).f32(), -2048.0);
        // A world space: its value.
        let world_space = e.mem.alloc(0x20);
        e.mem.set_f32(world_space + 8, 64.5);
        e.mem.set_u32(cell + 4, world_space);
        assert_eq!(e.call(0x0053_a550, &args![this]).f32(), 64.5);
    }

    // ---- Layer textures and opacities ------------------------------------------------------------

    #[test]
    fn default_texture_getter_checks_the_quadrant_and_the_data() {
        let mut e = engine();
        let (this, data) = percent_land(&mut e);
        set_default_texture(&mut e, data, 2, 0x2222);
        assert_eq!(e.call(0x0053_a5a0, &args![this, 2u32]).u32(), 0x2222);
        assert_eq!(e.call(0x0053_a5a0, &args![this, 4u32]).u32(), 0);
        let bare = land(&mut e);
        assert_eq!(e.call(0x0053_a5a0, &args![bare, 0u32]).u32(), 0);
    }

    #[test]
    fn layer_texture_getter_and_setter() {
        let mut e = engine();
        let (this, data) = percent_land(&mut e);
        set_layers(&mut e, data, 3, &[0x10, 0x11, 0x12, 0x13, 0x14, 0x15]);
        assert_eq!(e.call(0x0053_a700, &args![this, 3u32, 4u32]).u32(), 0x14);
        assert_eq!(e.call(0x0053_a700, &args![this, 3u32, 6u32]).u32(), 0);
        assert_eq!(e.call(0x0053_a700, &args![this, 4u32, 0u32]).u32(), 0);
        // A quadrant without a layer array.
        e.mem
            .set_u32(data.addr() + LoadedLandData::pQuadTextureArray.off + 4, 0);
        assert_eq!(e.call(0x0053_a700, &args![this, 1u32, 0u32]).u32(), 0);
        e.call(0x0053_a760, &args![this, 3u32, 2u32, 0x99u32]);
        assert_eq!(layers(&e, data, 3)[2], 0x99);
        // Out of range: nothing is written.
        e.call(0x0053_a760, &args![this, 3u32, 6u32, 0x77u32]);
        e.call(0x0053_a760, &args![this, 4u32, 0u32, 0x77u32]);
        assert_eq!(
            layers(&e, data, 3),
            vec![0x10, 0x11, 0x99, 0x13, 0x14, 0x15]
        );
    }

    #[test]
    fn opacity_getters() {
        let mut e = engine();
        let (this, data) = percent_land(&mut e);
        set_opacities(&mut e, data, 2, 7, &[0.25, 0.125, 0.0, 0.0, 0.0, 0.5]);
        assert_eq!(
            e.call(0x0053_a830, &args![this, 2u32, 7u32, 5u32]).f32(),
            0.5
        );
        assert_eq!(
            e.call(0x0053_a830, &args![this, 2u32, 7u32, 6u32]).f32(),
            0.0
        );
        assert_eq!(
            e.call(0x0053_a830, &args![this, 2u32, 0x121u32, 0u32])
                .f32(),
            0.0
        );
        assert_eq!(
            e.call(0x0053_a830, &args![this, 4u32, 7u32, 0u32]).f32(),
            0.0
        );
        // The default texture's share: 1 - sum.
        assert_eq!(e.call(0x0053_a7a0, &args![this, 2u32, 7u32]).f32(), 0.125);
        assert_eq!(e.call(0x0053_a7a0, &args![this, 2u32, 0x121u32]).f32(), 1.0);
        // A quadrant without a percent array.
        e.mem
            .set_u32(data.addr() + LoadedLandData::ppPercentArrays.off + 12, 0);
        assert_eq!(
            e.call(0x0053_a830, &args![this, 3u32, 7u32, 0u32]).f32(),
            0.0
        );
        assert_eq!(e.call(0x0053_a7a0, &args![this, 3u32, 7u32]).f32(), 1.0);
    }

    #[test]
    fn opacity_setter_stores_positive_values_and_clears_the_rest() {
        let mut e = engine();
        let (this, data) = percent_land(&mut e);
        e.call(0x0053_a8a0, &args![this, 1u32, 9u32, 2u32, 0.75f32]);
        assert_eq!(opacities(&e, data, 1, 9)[2], 0.75);
        // Zero and negative values store 0.
        e.call(0x0053_a8a0, &args![this, 1u32, 9u32, 2u32, 0.0f32]);
        assert_eq!(opacities(&e, data, 1, 9)[2], 0.0);
        e.call(0x0053_a8a0, &args![this, 1u32, 9u32, 3u32, 0.5f32]);
        e.call(0x0053_a8a0, &args![this, 1u32, 9u32, 3u32, -1.0f32]);
        assert_eq!(opacities(&e, data, 1, 9)[3], 0.0);
        // Out of range: nothing changes.
        e.call(0x0053_a8a0, &args![this, 1u32, 9u32, 6u32, 0.5f32]);
        e.call(0x0053_a8a0, &args![this, 4u32, 9u32, 0u32, 0.5f32]);
        e.call(0x0053_a8a0, &args![this, 1u32, 0x121u32, 0u32, 0.5f32]);
        assert_eq!(opacities(&e, data, 1, 9)[..6], [0.0; 6]);
        // Without a percent array a non-positive value is dropped.
        e.mem
            .set_u32(data.addr() + LoadedLandData::ppPercentArrays.off + 4, 0);
        e.call(0x0053_a8a0, &args![this, 1u32, 9u32, 0u32, 0.0f32]);
    }

    #[test]
    fn dominant_texture_follows_the_highest_opacity() {
        let mut e = engine();
        let (this, data) = percent_land(&mut e);
        set_default_texture(&mut e, data, 0, 0xd0);
        set_layers(&mut e, data, 0, &[0x100, 0x101, 0x102, 0x103, 0x104, 0x105]);
        // Layer 3 is the highest at vertex 5: its texture is layer 2's.
        set_opacities(&mut e, data, 0, 5, &[0.1, 0.0, 0.5, 0.7, 0.0, 0.0]);
        assert_eq!(e.call(0x0053_a630, &args![this, 0u32, 5u32]).u32(), 0x102);
        // The first layer winning (or a tie) gives the default texture.
        set_opacities(&mut e, data, 0, 6, &[0.7, 0.0, 0.0, 0.7, 0.0, 0.0]);
        assert_eq!(e.call(0x0053_a630, &args![this, 0u32, 6u32]).u32(), 0xd0);
        // All zero: layer 0 is the first: the default texture.
        assert_eq!(e.call(0x0053_a630, &args![this, 0u32, 7u32]).u32(), 0xd0);
        // Out of range.
        assert_eq!(e.call(0x0053_a630, &args![this, 4u32, 5u32]).u32(), 0);
        assert_eq!(e.call(0x0053_a630, &args![this, 0u32, 0x121u32]).u32(), 0);
    }

    #[test]
    fn removing_a_layer_moves_the_others_down() {
        let mut e = engine();
        let (this, data) = percent_land(&mut e);
        set_layers(&mut e, data, 2, &[0xa, 0xb, 0xc, 0xd, 0xe, 0xf]);
        set_opacities(
            &mut e,
            data,
            2,
            0,
            &[0.5, 0.25, 0.125, 0.0625, 0.03125, 0.015625],
        );
        set_opacities(&mut e, data, 2, 288, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        e.call(0x0053_abb0, &args![this, 2u32, 1u32]);
        assert_eq!(layers(&e, data, 2), vec![0xa, 0xc, 0xd, 0xe, 0xf, 0]);
        assert_eq!(
            opacities(&e, data, 2, 0)[..6],
            [0.5, 0.125, 0.0625, 0.03125, 0.015625, 0.0]
        );
        assert_eq!(
            opacities(&e, data, 2, 288)[..6],
            [1.0, 3.0, 4.0, 5.0, 6.0, 0.0]
        );
        // Out of range: nothing.
        e.call(0x0053_abb0, &args![this, 2u32, 6u32]);
        e.call(0x0053_abb0, &args![this, 4u32, 0u32]);
        assert_eq!(layers(&e, data, 2), vec![0xa, 0xc, 0xd, 0xe, 0xf, 0]);
    }

    #[test]
    fn swapping_a_layer_with_the_default_texture_recomputes_its_opacity() {
        let mut e = engine();
        let (this, data) = percent_land(&mut e);
        set_default_texture(&mut e, data, 0, 0xd0);
        set_layers(&mut e, data, 0, &[0xa, 0xb, 0, 0, 0, 0]);
        set_opacities(&mut e, data, 0, 3, &[0.25, 0.5, 0.0, 0.0, 0.0, 0.0]);
        // The sum exceeds 1: the layer's opacity is 0.
        set_opacities(&mut e, data, 0, 4, &[0.75, 0.75, 0.0, 0.0, 0.0, 0.0]);
        e.call(0x0053_ace0, &args![this, 0u32, 1u32]);
        assert_eq!(default_texture(&e, data, 0), 0xb);
        assert_eq!(layers(&e, data, 0)[..2], [0xa, 0xd0]);
        // 1 - (0.25 + 0.5) with the old value of the layer in the sum.
        assert_eq!(opacities(&e, data, 0, 3)[1], 0.25);
        assert_eq!(opacities(&e, data, 0, 4)[1], 0.0);
        // Every vertex is recomputed: an empty one gets 1.
        assert_eq!(opacities(&e, data, 0, 100)[1], 1.0);
        // Out of range: nothing changes.
        e.call(0x0053_ace0, &args![this, 0u32, 6u32]);
        e.call(0x0053_ace0, &args![this, 4u32, 0u32]);
        assert_eq!(default_texture(&e, data, 0), 0xb);
    }

    #[test]
    fn layer_total_sums_the_opacity_of_a_textured_layer() {
        let mut e = engine();
        let (this, data) = percent_land(&mut e);
        set_layers(&mut e, data, 1, &[0xa, 0, 0, 0, 0, 0]);
        for vertex in 0..0x121 {
            set_opacities(&mut e, data, 1, vertex, &[0.5, 0.25]);
        }
        assert_eq!(e.call(0x0053_ae30, &args![this, 1u32, 0u32]).f32(), 144.5);
        // A layer without a texture, a layer or quadrant out of range.
        assert_eq!(e.call(0x0053_ae30, &args![this, 1u32, 1u32]).f32(), 0.0);
        assert_eq!(e.call(0x0053_ae30, &args![this, 1u32, 6u32]).f32(), 0.0);
        assert_eq!(e.call(0x0053_ae30, &args![this, 4u32, 0u32]).f32(), 0.0);
    }

    #[test]
    fn weak_layers_are_dropped_and_the_strongest_becomes_the_default() {
        let mut e = engine();
        let (this, data) = percent_land(&mut e);
        set_default_texture(&mut e, data, 0, 0x400);
        // Layer 0 holds 0.9 everywhere; layer 1 nothing; the rest are empty.
        set_layers(&mut e, data, 0, &[0x500, 0x501, 0, 0, 0, 0]);
        for vertex in 0..0x121 {
            set_opacities(&mut e, data, 0, vertex, &[0.9]);
        }
        start_log(&mut e);
        e.call(0x0053_a940, &args![this]);
        let log = end_log(&mut e);
        // The layer array is squeezed, the scratch array has 1 + 1 floats.
        assert_eq!(layers(&e, data, 0)[1], 0);
        assert_eq!(
            arguments_of(&log, ALLOCATE),
            vec![vec![8], vec![4], vec![4], vec![4]]
        );
        assert_eq!(arguments_of(&log, DEALLOCATE).len(), 4);
        // The first layer has the highest total: swapped with the default.
        assert_eq!(default_texture(&e, data, 0), 0x500);
        assert_eq!(layers(&e, data, 0)[0], 0x400);
        let values = opacities(&e, data, 0, 17);
        assert!((values[0] - 0.1).abs() < 1e-6, "{values:?}");
        // The other quadrants have nothing to do.
        assert_eq!(default_texture(&e, data, 1), 0);
    }

    #[test]
    fn dropping_weak_layers_keeps_a_stronger_default() {
        let mut e = engine();
        let (this, data) = percent_land(&mut e);
        set_default_texture(&mut e, data, 1, 0x400);
        set_layers(&mut e, data, 1, &[0x500, 0, 0, 0, 0, 0]);
        // The layer's total is below 0.001 * 289: dropped.
        set_opacities(&mut e, data, 1, 0, &[0.01]);
        e.call(0x0053_a940, &args![this]);
        assert_eq!(layers(&e, data, 1), vec![0; 6]);
        assert_eq!(default_texture(&e, data, 1), 0x400);
        // Without loaded data nothing happens.
        let bare = land(&mut e);
        e.call(0x0053_a940, &args![bare]);
    }

    #[test]
    fn default_share_of_weak_layers_rounds_at_the_control_words_precision() {
        // 0053ab06: `(1 - sum) + share` stays on the x87 stack. Vertex 0
        // leaves 0.5 + 2^-23 to the default texture, vertex 1 leaves
        // 1 - 0.75 * 2^-24: at 24-bit precision that is 1 - 2^-24 first and
        // the sum 1.5 + 2^-24 rounds to even (1.5); at 53-bit the exact
        // 1.5 + 1.25 * 2^-24 rounds up to 1.5 + 2^-23.
        fn default_share(word: u16) -> u32 {
            let mut e = engine();
            let (this, data) = percent_land(&mut e);
            set_layers(&mut e, data, 0, &[0x500, 0, 0, 0, 0, 0]);
            set_opacities(&mut e, data, 0, 0, &[0.5 - f32::EPSILON]);
            set_opacities(&mut e, data, 0, 1, &[3.0 / 67_108_864.0]);
            for vertex in 2..0x121 {
                set_opacities(&mut e, data, 0, vertex, &[1.0]);
            }
            // The scratch array of quadrant 0 is the first block freed:
            // (layer total, default share).
            let freed = recorder::<u32>();
            let sink = freed.clone();
            e.register_double(DEALLOCATE, move |e, a| {
                if sink.borrow().is_empty() {
                    sink.borrow_mut().push(e.mem.u32(a[0] + 4));
                }
                e.mem.free(a[0]);
                Ret::default()
            });
            crate::units::crt::set_x87_control_word(&mut e, word);
            e.call(0x0053_a940, &args![this]);
            crate::units::crt::set_x87_control_word(&mut e, 0x027f);
            freed.take()[0]
        }
        assert_eq!(default_share(0x007f), 1.5f32.to_bits());
        assert_eq!(default_share(0x027f), 1.5f32.to_bits() + 1);
    }

    /// Float helpers for the doubles of the layer merge.
    fn merge_doubles(e: &mut Engine) {
        // The texture groups are the top nibble of the 16-bit value: textures
        // of one group can be merged (the "differ" call returns 0).
        e.register(TEXTURES_DIFFER, |_, a| {
            ret(((a[0] & 0xf000) != (a[1] & 0xf000)) as u32)
        });
        e.register(FLOAT_MAX, |_, a| {
            float_ret(f32::from_bits(a[0]).max(f32::from_bits(a[1])))
        });
        e.register(FLOAT_MIN, |_, a| {
            float_ret(f32::from_bits(a[0]).min(f32::from_bits(a[1])))
        });
    }

    #[test]
    fn mergeable_layers_are_combined_and_the_opacities_normalized() {
        let mut e = engine();
        merge_doubles(&mut e);
        let (this, data) = percent_land(&mut e);
        // Quadrant 0: layers 0 and 1 are of one group, the default is not.
        set_default_texture(&mut e, data, 0, 0x1000);
        set_layers(&mut e, data, 0, &[0x2000, 0x2010, 0, 0, 0, 0]);
        set_opacities(&mut e, data, 0, 1, &[0.25, 0.25]);
        set_opacities(&mut e, data, 0, 2, &[0.75, 0.75]);
        // Quadrant 1: layer 0 is of the default texture's group.
        set_default_texture(&mut e, data, 1, 0x1100);
        set_layers(&mut e, data, 1, &[0x1200, 0, 0, 0, 0, 0]);
        set_opacities(&mut e, data, 1, 1, &[0.5]);
        start_log(&mut e);
        e.call(0x0053_aeb0, &args![this]);
        let log = end_log(&mut e);
        assert!(called_contains(&log, TEXTURES_DIFFER));
        // Layer 1 was merged into layer 0 and removed.
        assert_eq!(layers(&e, data, 0), vec![0x2000, 0, 0, 0, 0, 0]);
        // Vertex 1: the opacities moved up and the default share is
        // 1 - 0.5 = 0.5.
        assert_eq!(opacities(&e, data, 0, 1)[..3], [0.5, 0.5, 0.0]);
        // Vertex 2: merged 1.5, share 0 (clamped), the others divided by
        // their sum.
        assert_eq!(opacities(&e, data, 0, 2)[..3], [0.0, 1.0, 0.0]);
        // An empty vertex: all of it goes to the default texture.
        assert_eq!(opacities(&e, data, 0, 3)[..3], [1.0, 0.0, 0.0]);
        // Quadrant 1: the layer shares the default's group: dropped.
        assert_eq!(layers(&e, data, 1), vec![0; 6]);
        assert_eq!(opacities(&e, data, 1, 1)[..2], [1.0, 0.0]);
    }

    #[test]
    fn opacities_move_up_through_the_x87_stack() {
        let mut e = engine();
        merge_doubles(&mut e);
        let (this, data) = percent_land(&mut e);
        // One layer, of another group than the default texture; vertex 5
        // holds a signalling NaN in its first opacity.
        set_default_texture(&mut e, data, 0, 0x1000);
        set_layers(&mut e, data, 0, &[0x2000, 0, 0, 0, 0, 0]);
        let row = opacity_row(&e, data, 0, 5);
        e.mem.set_u32(row, 0x7f80_0001);
        e.call(0x0053_aeb0, &args![this]);
        // 0053b324 copies it with `FLD dword; FSTP dword`: quieted.
        assert_eq!(e.mem.u32(row + 4), 0x7fc0_0001);
        assert_eq!(layers(&e, data, 0), vec![0x2000, 0, 0, 0, 0, 0]);
    }

    /// Whether the call log has a call to `address`.
    fn called_contains(log: &[(u32, Vec<u32>)], address: u32) -> bool {
        log.iter().any(|(a, _)| *a == address)
    }

    // ---- Third session: coordinate and normal helpers ---------------------------------

    fn v3(e: &Engine, at: u32) -> [f32; 3] {
        [e.mem.f32(at), e.mem.f32(at + 4), e.mem.f32(at + 8)]
    }

    fn put_v3(e: &mut Engine, at: u32, value: [f32; 3]) {
        for (i, component) in value.iter().enumerate() {
            e.mem.set_f32(at + 4 * i as u32, *component);
        }
    }

    fn normalize_v3(value: [f32; 3]) -> [f32; 3] {
        let length = (value[0] * value[0] + value[1] * value[1] + value[2] * value[2]).sqrt();
        if length > 0.0 {
            [value[0] / length, value[1] / length, value[2] / length]
        } else {
            value
        }
    }

    /// Doubles for the vector arithmetic the coordinate code calls.
    fn vector_doubles(e: &mut Engine) {
        e.register(NI_POINT3_ADD, |e, a| {
            let (p, q) = (v3(e, a[0]), v3(e, a[2]));
            put_v3(e, a[1], [p[0] + q[0], p[1] + q[1], p[2] + q[2]]);
            ret(a[1])
        });
        e.register(NI_POINT3_SUBTRACT, |e, a| {
            let (p, q) = (v3(e, a[0]), v3(e, a[2]));
            put_v3(e, a[1], [p[0] - q[0], p[1] - q[1], p[2] - q[2]]);
            ret(a[1])
        });
        e.register(NI_POINT3_ADD_IN_PLACE, |e, a| {
            let (p, q) = (v3(e, a[0]), v3(e, a[1]));
            put_v3(e, a[0], [p[0] + q[0], p[1] + q[1], p[2] + q[2]]);
            Ret::default()
        });
        e.register(NI_POINT3_SUBTRACT_IN_PLACE, |e, a| {
            let (p, q) = (v3(e, a[0]), v3(e, a[1]));
            put_v3(e, a[0], [p[0] - q[0], p[1] - q[1], p[2] - q[2]]);
            Ret::default()
        });
        e.register(NI_POINT3_SCALE_BY, |e, a| {
            let (s, q) = (f32::from_bits(a[1]), v3(e, a[2]));
            put_v3(e, a[0], [s * q[0], s * q[1], s * q[2]]);
            ret(a[0])
        });
        e.register(NI_POINT3_TIMES_SCALAR, |e, a| {
            let (p, s) = (v3(e, a[0]), f32::from_bits(a[2]));
            put_v3(e, a[1], [s * p[0], s * p[1], s * p[2]]);
            ret(a[1])
        });
        e.register(NI_POINT3_CROSS, |e, a| {
            let (p, q) = (v3(e, a[0]), v3(e, a[2]));
            put_v3(
                e,
                a[1],
                [
                    p[1] * q[2] - p[2] * q[1],
                    p[2] * q[0] - p[0] * q[2],
                    p[0] * q[1] - p[1] * q[0],
                ],
            );
            ret(a[1])
        });
        e.register(NI_POINT3_LENGTH, |e, a| {
            let p = v3(e, a[0]);
            float_ret((p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt())
        });
        e.register(NI_POINT3_UNIT_VECTOR, |e, a| {
            let p = normalize_v3(v3(e, a[0]));
            put_v3(e, a[0], p);
            Ret::default()
        });
        e.register(NI_POINT3_NORMALIZE, |e, a| {
            let p = normalize_v3(v3(e, a[0]));
            put_v3(e, a[0], p);
            Ret::default()
        });
        e.register(NI_POINT3_UNITIZE_VECTORS, |e, a| {
            for i in 0..a[1] {
                let at = a[0] + i * a[2];
                let p = normalize_v3(v3(e, at));
                put_v3(e, at, p);
            }
            Ret::default()
        });
        e.register(NI_POINT2_SUBTRACT, |e, a| {
            let x = e.mem.f32(a[0]) - e.mem.f32(a[2]);
            let y = e.mem.f32(a[0] + 4) - e.mem.f32(a[2] + 4);
            e.mem.set_f32(a[1], x);
            e.mem.set_f32(a[1] + 4, y);
            ret(a[1])
        });
        e.register(NI_POINT2_LENGTH, |e, a| {
            float_ret((e.mem.f32(a[0]).powi(2) + e.mem.f32(a[0] + 4).powi(2)).sqrt())
        });
        e.register(FLOAT_MODULO, |_, a| {
            float_ret(f32::from_bits(a[0]) % f32::from_bits(a[1]))
        });
        e.register(FLOAT_TO_INT, |_, a| ret(f32::from_bits(a[0]) as i32 as u32));
        e.register(NI_POINT3_SAME_XY, |_, _| ret(0));
    }

    /// A land at cell (0, 0) with loaded data: the vertex `v` of every
    /// quadrant is at `(128 * (v % 17), 128 * (v / 17), 0)`, every normal is
    /// (0, 0, 1), the "normal set" flags are clear, the land is marked
    /// loaded.
    fn terrain(e: &mut Engine) -> (Ptr<TESObjectLAND>, Ptr<LoadedLandData>) {
        let (this, data) = percent_land(e);
        e.set(this, TESObjectLAND::Data, FLAG_LOADED);
        let vertices = e.get(data, LoadedLandData::ppVertices);
        let normals = e.get(data, LoadedLandData::ppNormals);
        for quadrant in 0..4u32 {
            let block = element(e, vertices, quadrant);
            let normal_block = element(e, normals, quadrant);
            for vertex in 0..0x121u32 {
                put_v3(
                    e,
                    block + 12 * vertex,
                    [
                        128.0 * (vertex % 17) as f32,
                        128.0 * (vertex / 17) as f32,
                        0.0,
                    ],
                );
                put_v3(e, normal_block + 12 * vertex, [0.0, 0.0, 1.0]);
            }
        }
        (this, data)
    }

    fn normal_of(e: &Engine, data: Ptr<LoadedLandData>, quadrant: u32, vertex: u32) -> [f32; 3] {
        let normals = e.get(data, LoadedLandData::ppNormals);
        v3(e, element(e, normals, quadrant) + 12 * vertex)
    }

    fn set_normal(
        e: &mut Engine,
        data: Ptr<LoadedLandData>,
        quadrant: u32,
        vertex: u32,
        value: [f32; 3],
    ) {
        let normals = e.get(data, LoadedLandData::ppNormals);
        let at = element(e, normals, quadrant) + 12 * vertex;
        put_v3(e, at, value);
    }

    fn normal_flag(e: &Engine, data: Ptr<LoadedLandData>, quadrant: u32, vertex: u32) -> u8 {
        let set = e.get(data, LoadedLandData::ppNormalsSet);
        e.mem.u8(element(e, set, quadrant) + vertex)
    }

    fn set_normal_flag(
        e: &mut Engine,
        data: Ptr<LoadedLandData>,
        quadrant: u32,
        vertex: u32,
        on: u8,
    ) {
        let set = e.get(data, LoadedLandData::ppNormalsSet);
        let at = element(e, set, quadrant) + vertex;
        e.mem.set_u8(at, on);
    }

    /// Two floats in memory.
    fn position(e: &mut Engine, x: f32, y: f32) -> Ptr {
        let at = e.mem.alloc(8);
        e.mem.set_f32(at, x);
        e.mem.set_f32(at + 4, y);
        Ptr::new(at)
    }

    /// A `CoordData` the tests fill in.
    fn coord_info(e: &mut Engine) -> Ptr<CoordData> {
        e.new_object()
    }

    // ---- World position of a vertex ----------------------------------------------------

    #[test]
    fn vertex_world_position_from_the_cell_and_quadrant() {
        let mut e = engine();
        let (this, data) = terrain(&mut e);
        e.set(data, LoadedLandData::iCellX, 2);
        e.set(data, LoadedLandData::iCellY, -1);
        let out = e.mem.alloc(12);
        let result = e.call(0x0053_b450, &args![this, out, 3u32, 20u32, 0u32, 0u32]);
        assert_eq!(result.u32(), out);
        // 2 * 4096 + 2048 + 3 * 128, -4096 + 2048 + 128.
        assert_eq!(v3(&e, out), [10624.0, -1920.0, 0.0]);
        // Cell numbers given by pointer replace the land's.
        let (cell_x, cell_y) = (e.mem.alloc(4), e.mem.alloc(4));
        e.mem.set_i32(cell_x, 5);
        e.mem.set_i32(cell_y, 6);
        e.call(0x0053_b450, &args![this, out, 0u32, 0u32, cell_x, cell_y]);
        assert_eq!(v3(&e, out), [20480.0, 24576.0, 0.0]);
    }

    // ---- GetCoordData ----------------------------------------------------------------------

    #[test]
    fn coord_data_needs_loaded_arrays_and_a_position_in_the_cell() {
        let mut e = engine();
        vector_doubles(&mut e);
        let inside = position(&mut e, 100.0, 100.0);
        let info = coord_info(&mut e);
        // No loaded data.
        let bare = land(&mut e);
        assert_eq!(
            e.call(0x0053_b550, &args![bare, info, inside, 0u32]).u32() & 0xff,
            0
        );
        // Loaded data without vertices and meshes.
        let (this, data) = terrain(&mut e);
        let vertices = e.get(data, LoadedLandData::ppVertices);
        e.set(data, LoadedLandData::ppVertices, Ptr::NULL);
        assert_eq!(
            e.call(0x0053_b550, &args![this, info, inside, 0u32]).u32() & 0xff,
            0
        );
        e.set(data, LoadedLandData::ppVertices, vertices);
        // Outside the cell on every side; the edges belong to the cell.
        for (x, y, expected) in [
            (-1.0, 100.0, 0),
            (4097.0, 100.0, 0),
            (100.0, -1.0, 0),
            (100.0, 4097.0, 0),
            (4096.0, 4096.0, 1),
            (0.0, 0.0, 1),
        ] {
            let at = position(&mut e, x, y);
            let result = e.call(0x0053_b550, &args![this, info, at, 0u32]).u32() & 0xff;
            assert_eq!(result, expected, "{x} {y}");
        }
    }

    #[test]
    fn coord_data_of_a_position_between_vertices() {
        let mut e = engine();
        vector_doubles(&mut e);
        let (this, _data) = terrain(&mut e);
        let at = position(&mut e, 300.0, 200.0);
        let info = coord_info(&mut e);
        assert_eq!(
            e.call(0x0053_b550, &args![this, info, at, 0u32]).u32() & 0xff,
            1
        );
        assert_eq!(e.get(info, CoordData::CellOffsetX), 300.0);
        assert_eq!(e.get(info, CoordData::CellOffsetY), 200.0);
        assert_eq!(e.get(info, CoordData::Quadrant), 0);
        assert_eq!(e.get(info, CoordData::QuadrantCopy), 0);
        assert_eq!(e.get(info, CoordData::QuadrantOffsetX), 300.0);
        assert_eq!(e.get(info, CoordData::BlockColumn), 2);
        assert_eq!(e.get(info, CoordData::BlockRow), 1);
        assert_eq!(e.get(info, CoordData::BlockOffsetX), 44.0);
        assert_eq!(e.get(info, CoordData::BlockOffsetY), 72.0);
        // An odd block (2 + 1): the first triangle.
        assert_eq!(e.get(info, CoordData::OddBlock), 1);
        assert_eq!(e.get(info, CoordData::SecondTriangle), 0);
        assert_eq!(e.get(info, CoordData::TriangleVertex0), 20);
        assert_eq!(e.get(info, CoordData::TriangleVertex1), 36);
        assert_eq!(e.get(info, CoordData::TriangleVertex2), 19);
        // The nearest of the three, and its position plus the cell centre.
        assert_eq!(e.get(info, CoordData::NearestVertex), 19);
        assert_eq!(
            v3(&e, info.addr() + CoordData::VertexPositionX.off),
            [2304.0, 2176.0, 0.0]
        );
    }

    #[test]
    fn coord_data_picks_the_nearest_of_the_triangle() {
        let mut e = engine();
        vector_doubles(&mut e);
        let (this, data) = terrain(&mut e);
        // The code adds the cell centre to each vertex and measures from the
        // position: vertex 36 at the origin is the closest.
        let vertices = e.get(data, LoadedLandData::ppVertices);
        let at = element(&e, vertices, 0) + 12 * 36;
        put_v3(&mut e, at, [0.0, 0.0, 0.0]);
        let at = position(&mut e, 300.0, 200.0);
        let info = coord_info(&mut e);
        e.call(0x0053_b550, &args![this, info, at, 0u32]);
        assert_eq!(e.get(info, CoordData::NearestVertex), 36);
        assert_eq!(
            v3(&e, info.addr() + CoordData::VertexPositionX.off),
            [2048.0, 2048.0, 0.0]
        );
    }

    #[test]
    fn coord_data_of_a_position_on_the_grid() {
        let mut e = engine();
        vector_doubles(&mut e);
        let (this, _data) = terrain(&mut e);
        let at = position(&mut e, 256.0, 128.0);
        let info = coord_info(&mut e);
        assert_eq!(
            e.call(0x0053_b550, &args![this, info, at, 0u32]).u32() & 0xff,
            1
        );
        // An exact multiple belongs to the block below: (2 - 1, 1 - 1).
        assert_eq!(e.get(info, CoordData::BlockColumn), 1);
        assert_eq!(e.get(info, CoordData::BlockRow), 0);
        assert_eq!(e.get(info, CoordData::BlockOffsetX), 128.0);
        assert_eq!(e.get(info, CoordData::BlockOffsetY), 128.0);
        assert_eq!(e.get(info, CoordData::OddBlock), 1);
        assert_eq!(e.get(info, CoordData::SecondTriangle), 1);
        assert_eq!(e.get(info, CoordData::TriangleVertex0), 2);
        assert_eq!(e.get(info, CoordData::TriangleVertex1), 18);
        assert_eq!(e.get(info, CoordData::TriangleVertex2), 19);
        // On the grid the vertex is computed: row 1, column 2.
        assert_eq!(e.get(info, CoordData::NearestVertex), 19);
    }

    #[test]
    fn coord_data_of_an_even_block() {
        let mut e = engine();
        vector_doubles(&mut e);
        let (this, _data) = terrain(&mut e);
        // Block (1, 1): even; x offset 30, y offset 10: the first triangle.
        let at = position(&mut e, 158.0, 138.0);
        let info = coord_info(&mut e);
        e.call(0x0053_b550, &args![this, info, at, 0u32]);
        assert_eq!(e.get(info, CoordData::OddBlock), 0);
        assert_eq!(e.get(info, CoordData::SecondTriangle), 0);
        assert_eq!(e.get(info, CoordData::TriangleVertex0), 18);
        assert_eq!(e.get(info, CoordData::TriangleVertex1), 36);
        assert_eq!(e.get(info, CoordData::TriangleVertex2), 19);
        // The same block with y above x: the second triangle.
        let at = position(&mut e, 138.0, 158.0);
        e.call(0x0053_b550, &args![this, info, at, 0u32]);
        assert_eq!(e.get(info, CoordData::SecondTriangle), 1);
        assert_eq!(e.get(info, CoordData::TriangleVertex2), 35);
        // An odd block (0, 1): offsets 10 + 42 are below 128, the first
        // triangle; 100 + 42 above, the second.
        let at = position(&mut e, 10.0, 170.0);
        e.call(0x0053_b550, &args![this, info, at, 0u32]);
        assert_eq!(e.get(info, CoordData::OddBlock), 1);
        assert_eq!(e.get(info, CoordData::SecondTriangle), 0);
        assert_eq!(e.get(info, CoordData::TriangleVertex2), 17);
        let at = position(&mut e, 100.0, 170.0);
        e.call(0x0053_b550, &args![this, info, at, 0u32]);
        assert_eq!(e.get(info, CoordData::SecondTriangle), 1);
        assert_eq!(e.get(info, CoordData::TriangleVertex2), 35);
    }

    #[test]
    fn coord_data_odd_block_sum_rounds_at_the_control_words_precision() {
        let mut e = engine();
        vector_doubles(&mut e);
        let (this, _data) = terrain(&mut e);
        // Block (0, 1), odd; offsets 64 + 2^-17 and 64. 0053b9c3 adds them
        // on the x87 stack: at 24-bit precision 128 + 2^-17 rounds to 128
        // (not above 128: the first triangle), at 53-bit it stays above.
        let at = position(&mut e, f32::from_bits(0x4280_0001), 192.0);
        let info = coord_info(&mut e);
        for (word, second, vertex) in [(0x007fu16, 0u8, 17), (0x027f, 1, 35)] {
            crate::units::crt::set_x87_control_word(&mut e, word);
            e.call(0x0053_b550, &args![this, info, at, 0u32]);
            assert_eq!(e.get(info, CoordData::OddBlock), 1);
            assert_eq!(e.get(info, CoordData::SecondTriangle), second, "{word:#x}");
            assert_eq!(e.get(info, CoordData::TriangleVertex2), vertex, "{word:#x}");
        }
        crate::units::crt::set_x87_control_word(&mut e, 0x027f);
    }

    #[test]
    fn coord_data_can_look_up_the_vertex_position_again() {
        let mut e = engine();
        vector_doubles(&mut e);
        let (this, _data) = terrain(&mut e);
        let at = position(&mut e, 256.0, 128.0);
        let info = coord_info(&mut e);
        assert_eq!(
            e.call(0x0053_b550, &args![this, info, at, 1u32]).u32() & 0xff,
            1
        );
        // The vertex position (2304, 2176) is in quadrant 1 + 2 * 1.
        assert_eq!(e.get(info, CoordData::Quadrant), 3);
        assert_eq!(e.get(info, CoordData::CellOffsetX), 2304.0);
    }

    #[test]
    fn texture_at_a_position() {
        let mut e = engine();
        vector_doubles(&mut e);
        let (this, data) = terrain(&mut e);
        // The lookup recurses once: the vertex position (2304, 2176) is in
        // quadrant 3, where the grid vertex is 19 again.
        set_default_texture(&mut e, data, 3, 0xd0);
        set_layers(&mut e, data, 3, &[0x100, 0x101, 0x102, 0, 0, 0]);
        set_opacities(&mut e, data, 3, 19, &[0.1, 0.6, 0.2]);
        let at = position(&mut e, 256.0, 128.0);
        assert_eq!(e.call(0x0053_a5e0, &args![this, at]).u32(), 0x100);
        // Outside the cell.
        let outside = position(&mut e, -5.0, 128.0);
        assert_eq!(e.call(0x0053_a5e0, &args![this, outside]).u32(), 0);
    }

    // ---- Grass parameters -----------------------------------------------------------------------

    /// Doubles for the grass builder: settings, lists, form getters.
    fn grass_doubles(e: &mut Engine) {
        e.register(SETTING_GET_FLOAT_ADDRESS, |_, a| ret(a[0] + 4));
        e.register(SETTING_GET_INT_ADDRESS, |_, a| ret(a[0] + 4));
        e.register(LIST_FIRST_POSITION, |_, a| ret(a[0] + 0x20));
        e.register(LIST_NEXT_POSITION, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(GRASS_RECORD_CONSTRUCT, |_, a| ret(a[0]));
        e.register(GRASS_FORM_NAME, |_, _| ret(0x1234));
        e.register(0x0900_01b0, |_, _| float_ret(1.5));
        e.register(0x0900_01b8, |_, _| float_ret(2.5));
        e.register(0x0900_01c0, |_, _| float_ret(3.25));
        e.register(0x0900_01c8, |_, _| float_ret(4.5));
        e.register(0x0900_01d0, |_, _| ret(1));
        e.register(0x0900_01d8, |_, _| ret(3));
        e.register(0x0900_01e0, |_, _| ret(2));
        e.register(0x0900_0180, |_, _| ret(100));
        e.set_global(GRASS_DENSITY_SETTING + 4, 3.5f32);
    }

    /// A grass form with the given id.
    fn grass_form(e: &mut Engine, id: u32) -> u32 {
        let vtable = e.mem.alloc(0x200);
        for slot in [0x1b0u32, 0x1b8, 0x1c0, 0x1c8, 0x1d0, 0x1d8, 0x1e0, 0x180] {
            e.mem.set_u32(vtable + slot, 0x0900_0000 + slot);
        }
        let form = e.mem.alloc(0x20);
        e.mem.set_u32(form, vtable);
        e.mem.set_u32(form + 0xc, id);
        form
    }

    /// A land texture whose grass list holds `forms`.
    fn grass_texture(e: &mut Engine, forms: &[u32]) -> u32 {
        let texture = e.mem.alloc(0x40);
        let mut node = texture + 0x20;
        for (i, form) in forms.iter().enumerate() {
            e.mem.set_u32(node, *form);
            if i + 1 < forms.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
        texture
    }

    #[test]
    fn grass_records_are_built_for_the_default_texture_and_the_layers() {
        let mut e = engine();
        grass_doubles(&mut e);
        // A threshold above 1 is 0.9; at most 3 forms per texture; a step of
        // 8 gives the single vertex (8, 8) = key 144.
        e.set_global(GRASS_SETTING_THRESHOLD + 4, 2.0f32);
        e.set_global(GRASS_SETTING_MAXIMUM + 4, 2i32);
        e.set_global(GRASS_SETTING_STEP + 4, 8i32);
        let (this, data) = terrain(&mut e);
        let (form1, form2, form3) = (
            grass_form(&mut e, 0x111),
            grass_form(&mut e, 0x222),
            grass_form(&mut e, 0x333),
        );
        // Quadrant 0: the default texture has two forms; quadrant 1: layer 0
        // is a texture with one form and a strong opacity around the vertex.
        let default = grass_texture(&mut e, &[form1, form2]);
        set_default_texture(&mut e, data, 0, default);
        let layer = grass_texture(&mut e, &[form3]);
        set_layers(&mut e, data, 1, &[layer]);
        for offset in GRASS_SAMPLE_OFFSETS {
            set_opacities(&mut e, data, 1, (144 + offset) as u32, &[0.95]);
        }
        let sets = recorder::<(u32, u32, u32)>();
        let sink = sets.clone();
        e.register_double(GRASS_MAP_SET_AT, move |_, a| {
            sink.borrow_mut().push((a[0], a[1], a[2]));
            Ret::default()
        });
        start_log(&mut e);
        assert_eq!(e.call(0x0053_bc10, &args![this]).u32() & 0xff, 1);
        let log = end_log(&mut e);
        assert_eq!(e.mem.i32(GRASS_STEP), 8);
        let sets = sets.borrow();
        assert_eq!(sets.len(), 2);
        assert_eq!(sets[0].0, data.addr() + 0x54);
        assert_eq!(sets[1].0, data.addr() + 0x54 + 16);
        assert_eq!((sets[0].1, sets[1].1), (144, 144));
        // The tables of the quadrants without grass are freed.
        let freed: Vec<u32> = arguments_of(&log, DEALLOCATE)
            .iter()
            .map(|a| a[0])
            .collect();
        assert_eq!(freed.len(), 2);
        assert!(!freed.contains(&sets[0].2) && !freed.contains(&sets[1].2));
        // Quadrant 0: two records, then 0.
        let table = sets[0].2;
        let (first, second) = (e.mem.u32(table), e.mem.u32(table + 4));
        assert_eq!(e.mem.u32(table + 8), 0);
        for (record, id) in [(first, 0x111), (second, 0x222)] {
            assert_eq!(e.mem.u32(record), 0x1234);
            assert_eq!(e.mem.u32(record + 4), id);
            assert_eq!(e.mem.f32(record + 0x14), 3.5);
            assert_eq!(e.mem.f32(record + 0x18), 4.5);
            assert_eq!(e.mem.f32(record + 0x10), 3.25);
            assert_eq!(e.mem.f32(record + 8), 1.5);
            assert_eq!(e.mem.f32(record + 0xc), 2.5);
            assert_eq!(
                (
                    e.mem.u8(record + 0x1c),
                    e.mem.u8(record + 0x1d),
                    e.mem.u8(record + 0x1e)
                ),
                (1, 3, 2)
            );
            // The percentage 100 gives 1.0 for the nine samples that pass.
            assert_eq!(floats(&e, record + 0x20, 9), vec![1.0; 9]);
        }
        // Quadrant 1: one record from the layer.
        let table = sets[1].2;
        let record = e.mem.u32(table);
        assert_eq!(e.mem.u32(record + 4), 0x333);
        assert_eq!(floats(&e, record + 0x20, 9), vec![1.0; 9]);
        assert_eq!(e.mem.u32(table + 4), 0);
    }

    #[test]
    fn grass_samples_that_average_below_the_threshold_are_cleared() {
        let mut e = engine();
        grass_doubles(&mut e);
        e.set_global(GRASS_SETTING_THRESHOLD + 4, 0.5f32);
        e.set_global(GRASS_SETTING_MAXIMUM + 4, 0i32);
        e.set_global(GRASS_SETTING_STEP + 4, 8i32);
        let (this, data) = terrain(&mut e);
        let form = grass_form(&mut e, 0x444);
        // Layer 0 of quadrant 2 passes only at the vertex itself: one sample
        // of nine, the mean 1/9 is below the threshold.
        let layer = grass_texture(&mut e, &[form]);
        set_layers(&mut e, data, 2, &[layer]);
        set_opacities(&mut e, data, 2, 144, &[0.9]);
        let sets = recorder::<u32>();
        let sink = sets.clone();
        e.register_double(GRASS_MAP_SET_AT, move |_, a| {
            sink.borrow_mut().push(a[2]);
            Ret::default()
        });
        e.call(0x0053_bc10, &args![this]);
        let table = sets.borrow()[0];
        let record = e.mem.u32(table);
        assert_eq!(e.mem.u32(record + 4), 0x444);
        assert_eq!(floats(&e, record + 0x20, 9), vec![0.0; 9]);
    }

    #[test]
    fn grass_sample_mean_rounds_at_the_control_words_precision() {
        // The threshold is 1/3 rounded to a float (just above 1/3); three
        // samples of nine pass. 0053c27c compares it with 3 / 9 on the x87
        // stack: at 24-bit precision the mean is that same float (kept), at
        // 53-bit it is below the threshold (cleared).
        fn samples(word: u16) -> Vec<f32> {
            let mut e = engine();
            grass_doubles(&mut e);
            e.set_global(GRASS_SETTING_THRESHOLD + 4, f32::from_bits(0x3eaa_aaab));
            e.set_global(GRASS_SETTING_MAXIMUM + 4, 0i32);
            e.set_global(GRASS_SETTING_STEP + 4, 8i32);
            let (this, data) = terrain(&mut e);
            let form = grass_form(&mut e, 0x555);
            let layer = grass_texture(&mut e, &[form]);
            set_layers(&mut e, data, 2, &[layer]);
            for vertex in [126, 127, 128] {
                set_opacities(&mut e, data, 2, vertex, &[0.9]);
            }
            let sets = recorder::<u32>();
            let sink = sets.clone();
            e.register_double(GRASS_MAP_SET_AT, move |_, a| {
                sink.borrow_mut().push(a[2]);
                Ret::default()
            });
            crate::units::crt::set_x87_control_word(&mut e, word);
            e.call(0x0053_bc10, &args![this]);
            crate::units::crt::set_x87_control_word(&mut e, 0x027f);
            let record = e.mem.u32(sets.take()[0]);
            floats(&e, record + 0x20, 9)
        }
        let mut kept = vec![1.0f32; 3];
        kept.extend([0.0; 6]);
        assert_eq!(samples(0x007f), kept);
        assert_eq!(samples(0x027f), vec![0.0; 9]);
    }

    #[test]
    fn grass_step_defaults_to_two() {
        let mut e = engine();
        grass_doubles(&mut e);
        e.set_global(GRASS_SETTING_THRESHOLD + 4, 0.5f32);
        e.set_global(GRASS_SETTING_MAXIMUM + 4, 0i32);
        e.set_global(GRASS_SETTING_STEP + 4, 3i32);
        let (this, _data) = terrain(&mut e);
        start_log(&mut e);
        assert_eq!(e.call(0x0053_bc10, &args![this]).u32() & 0xff, 1);
        let log = end_log(&mut e);
        assert_eq!(e.mem.i32(GRASS_STEP), 2);
        // Rows and columns 2, 6, 10 and 14 of four quadrants.
        assert_eq!(arguments_of(&log, ALLOCATE).len(), 64);
        assert_eq!(arguments_of(&log, DEALLOCATE).len(), 64);
        // Without loaded data nothing is built.
        let bare = land(&mut e);
        start_log(&mut e);
        assert_eq!(e.call(0x0053_bc10, &args![bare]).u32() & 0xff, 0);
        assert!(called(&mut e).is_empty());
    }

    #[test]
    fn grass_helpers() {
        let mut e = engine();
        e.register(SETTING_GET_FLOAT_ADDRESS, |_, a| ret(a[0] + 4));
        e.set_global(GRASS_DENSITY_SETTING + 4, 7.25f32);
        assert_eq!(e.call(0x0053_ca40, &args![]).f32(), 7.25);
        // The record constructor returns its record.
        e.register(GRASS_RECORD_CONSTRUCT, |_, a| ret(a[0] + 1));
        start_log(&mut e);
        assert_eq!(e.call(0x0053_ca60, &args![0x5000u32]).u32(), 0x5000);
        assert_eq!(called(&mut e), vec![GRASS_RECORD_CONSTRUCT]);
        // The grass table lookup.
        let (this, data) = terrain(&mut e);
        e.register(GRASS_MAP_GET_AT, |e, a| {
            if a[1] == 144 {
                e.mem.set_u32(a[2], 0xbeef);
                ret(1)
            } else {
                ret(0)
            }
        });
        start_log(&mut e);
        assert_eq!(
            e.call(0x0053_ca80, &args![this, 2i32, 144i32]).u32(),
            0xbeef
        );
        let log = end_log(&mut e);
        assert_eq!(log.len(), 1);
        assert_eq!(log[0].0, GRASS_MAP_GET_AT);
        assert_eq!(log[0].1[..2], [data.addr() + 0x54 + 32, 144]);
        // Not found, quadrant or key out of range, no data.
        start_log(&mut e);
        assert_eq!(e.call(0x0053_ca80, &args![this, 2i32, 145i32]).u32(), 0);
        assert_eq!(e.call(0x0053_ca80, &args![this, 4i32, 144i32]).u32(), 0);
        assert_eq!(e.call(0x0053_ca80, &args![this, 0i32, 0x11_i32]).u32(), 0);
        assert_eq!(e.call(0x0053_ca80, &args![this, 0i32, 0x10f_i32]).u32(), 0);
        assert_eq!(e.call(0x0053_ca80, &args![this, 0i32, 0x10e_i32]).u32(), 0);
        // Only 145 and 0x10e passed the range check (the top-level calls of
        // the other three are logged as well).
        let looked_up = arguments_of(&end_log(&mut e), GRASS_MAP_GET_AT);
        assert_eq!(looked_up.len(), 2);
        let bare = land(&mut e);
        assert_eq!(e.call(0x0053_ca80, &args![bare, 0i32, 144i32]).u32(), 0);
    }

    // ---- Vector helpers --------------------------------------------------------------------------

    #[test]
    fn unit_cross_product() {
        let mut e = engine();
        vector_doubles(&mut e);
        let (a, b, out) = (e.mem.alloc(12), e.mem.alloc(12), e.mem.alloc(12));
        put_v3(&mut e, a, [3.0, 0.0, 0.0]);
        put_v3(&mut e, b, [0.0, 2.0, 0.0]);
        let result = e.call(0x0053_d1a0, &args![a, out, b]);
        assert_eq!(result.u32(), out);
        assert_eq!(v3(&e, out), [0.0, 0.0, 1.0]);
        // The other order is the opposite.
        e.call(0x0053_d1a0, &args![b, out, a]);
        assert_eq!(v3(&e, out), [0.0, 0.0, -1.0]);
        // Parallel vectors give the zero vector.
        put_v3(&mut e, b, [6.0, 0.0, 0.0]);
        put_v3(&mut e, out, [9.0, 9.0, 9.0]);
        e.call(0x0053_d1a0, &args![a, out, b]);
        assert_eq!(v3(&e, out), [0.0, 0.0, 0.0]);
    }

    #[test]
    fn vector_scaling_by_the_inverse_length() {
        let mut e = engine();
        vector_doubles(&mut e);
        let (a, out) = (e.mem.alloc(12), e.mem.alloc(12));
        put_v3(&mut e, a, [2.0, 4.0, 6.0]);
        assert_eq!(e.call(0x0053_d280, &args![a, out, 2.0f32]).u32(), out);
        assert_eq!(v3(&e, out), [1.0, 2.0, 3.0]);
    }

    // ---- The normal at a position -------------------------------------------------------------

    /// A land and a `CoordData` describing the triangle (19, 20, 36) of
    /// quadrant 0 with the nearest vertex 19; the vertex normals are 19:
    /// (1, 0, 0), 20: (0, 1, 0), 36: (0, 0, 1).
    fn normal_world(e: &mut Engine, odd: u8, second: u8) -> (Ptr<TESObjectLAND>, Ptr<CoordData>) {
        vector_doubles(e);
        let (this, data) = terrain(e);
        set_normal(e, data, 0, 19, [1.0, 0.0, 0.0]);
        set_normal(e, data, 0, 20, [0.0, 1.0, 0.0]);
        set_normal(e, data, 0, 36, [0.0, 0.0, 1.0]);
        let info = coord_info(e);
        e.set(info, CoordData::Quadrant, 0);
        e.set(info, CoordData::NearestVertex, 19);
        e.set(info, CoordData::TriangleVertex2, 19);
        e.set(info, CoordData::TriangleVertex0, 20);
        e.set(info, CoordData::TriangleVertex1, 36);
        e.set(info, CoordData::OddBlock, odd);
        e.set(info, CoordData::SecondTriangle, second);
        (this, info)
    }

    #[test]
    fn position_and_normal_on_a_vertex() {
        let mut e = engine();
        let (this, info) = normal_world(&mut e, 0, 0);
        let compared = recorder::<([f32; 3], [f32; 3])>();
        let sink = compared.clone();
        e.register_double(NI_POINT3_SAME_XY, move |e, a| {
            sink.borrow_mut().push((v3(e, a[0]), v3(e, a[1])));
            ret(1)
        });
        let (position, normal) = (e.mem.alloc(12), e.mem.alloc(12));
        assert_eq!(
            e.call(0x0053_caf0, &args![this, info, position, normal])
                .u32()
                & 0xff,
            1
        );
        // The vertex's normal is both results.
        assert_eq!(v3(&e, position), [1.0, 0.0, 0.0]);
        assert_eq!(v3(&e, normal), [1.0, 0.0, 0.0]);
        // The vertex position plus the cell centre is compared with the
        // grid position.
        assert_eq!(
            *compared.borrow(),
            vec![([2304.0, 2176.0, 0.0], [256.0, 128.0, 0.0])]
        );
    }

    #[test]
    fn interpolated_normal_depends_on_the_block_flags() {
        // On the grid the weights are 0, so each case gives one vertex normal.
        for (odd, second, expected, face) in [
            (0u8, 0u8, [0.0, 1.0, 0.0], [0.0, 0.0, -1.0]),
            (0, 1, [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]),
            (1, 0, [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]),
            (1, 1, [0.0, 1.0, 0.0], [0.0, 0.0, -1.0]),
        ] {
            let mut e = engine();
            let (this, info) = normal_world(&mut e, odd, second);
            let (position, normal) = (e.mem.alloc(12), e.mem.alloc(12));
            assert_eq!(
                e.call(0x0053_caf0, &args![this, info, position, normal])
                    .u32()
                    & 0xff,
                1
            );
            assert_eq!(v3(&e, position), expected, "{odd} {second}");
            assert_eq!(v3(&e, normal), face, "{odd} {second}");
        }
    }

    #[test]
    fn fractional_position_weighs_the_vertex_normals() {
        let mut e = engine();
        let (this, info) = normal_world(&mut e, 0, 0);
        // A grid position 32 units from the block corner in x and 64 in y:
        // u = 0.25, v = 0.5. Vertex 19 sits at (256, 128).
        e.register(FLOAT_MODULO, |_, a| {
            // fmod(x, 128): the test position is 32 / 64.
            float_ret(if f32::from_bits(a[0]) == 256.0 {
                32.0
            } else {
                64.0
            })
        });
        let (position, normal) = (e.mem.alloc(12), e.mem.alloc(12));
        e.call(0x0053_caf0, &args![this, info, position, normal]);
        // ((1 - u) * b + u * a) * (1 - v) + v * c, normalized.
        let weighted = [0.25 * 0.5, 0.75 * 0.5, 0.5];
        let expected = normalize_v3(weighted);
        let actual = v3(&e, position);
        for i in 0..3 {
            assert!(
                (actual[i] - expected[i]).abs() < 1e-6,
                "{actual:?} {expected:?}"
            );
        }
    }

    #[test]
    fn position_and_normal_of_a_world_position() {
        let mut e = engine();
        vector_doubles(&mut e);
        let (this, data) = terrain(&mut e);
        set_normal(&mut e, data, 0, 19, [0.0, 1.0, 0.0]);
        e.register(NI_POINT3_SAME_XY, |_, _| ret(1));
        let (out_position, out_normal) = (e.mem.alloc(12), e.mem.alloc(12));
        let at = position(&mut e, 256.0, 128.0);
        assert_eq!(
            e.call(0x0053_d2e0, &args![this, at, out_position, out_normal])
                .u32()
                & 0xff,
            1
        );
        assert_eq!(v3(&e, out_position), [0.0, 1.0, 0.0]);
        assert_eq!(v3(&e, out_normal), [0.0, 1.0, 0.0]);
        // Outside the land: false and nothing written.
        let outside = position(&mut e, -50.0, 128.0);
        put_v3(&mut e, out_position, [9.0, 9.0, 9.0]);
        assert_eq!(
            e.call(0x0053_d2e0, &args![this, outside, out_position, out_normal])
                .u32()
                & 0xff,
            0
        );
        assert_eq!(v3(&e, out_position), [9.0, 9.0, 9.0]);
    }

    // ---- Border normals and neighbours --------------------------------------------------------------

    #[test]
    fn bounds_test_adds_the_cell_centre() {
        let mut e = engine();
        vector_doubles(&mut e);
        let (this, _data) = terrain(&mut e);
        let bounds = e.mem.alloc(16);
        for (i, value) in [0i32, 5000, 4000, 100].iter().enumerate() {
            e.mem.set_i32(bounds + 4 * i as u32, *value);
        }
        let vertex = e.mem.alloc(12);
        for (point, expected) in [
            ([0.0, 0.0, 0.0], 1),
            ([2500.0, 0.0, 0.0], 0),
            ([-2100.0, 0.0, 0.0], 0),
            ([0.0, -2000.0, 0.0], 0),
            ([0.0, 2900.0, 0.0], 1),
            ([0.0, 3500.0, 0.0], 0),
        ] {
            put_v3(&mut e, vertex, point);
            let result = e.call(0x0053_da60, &args![this, bounds, vertex]).u32() & 0xff;
            assert_eq!(result, expected, "{point:?}");
        }
        // Without bounds everything is inside.
        assert_eq!(
            e.call(0x0053_da60, &args![this, 0u32, vertex]).u32() & 0xff,
            1
        );
    }

    #[test]
    fn normals_array_and_flag_getters() {
        let mut e = engine();
        let (this, data) = terrain(&mut e);
        let normals = e.get(data, LoadedLandData::ppNormals);
        assert_eq!(e.call(0x0053_df00, &args![this]).u32(), normals.addr());
        let bare = land(&mut e);
        assert_eq!(e.call(0x0053_df00, &args![bare]).u32(), 0);
        set_normal_flag(&mut e, data, 2, 40, 1);
        assert_eq!(
            e.call(0x0053_e260, &args![this, 2i32, 40i32]).u32() & 0xff,
            1
        );
        assert_eq!(
            e.call(0x0053_e260, &args![this, 2i32, 41i32]).u32() & 0xff,
            0
        );
        // The flag of the vertex nearest to a position.
        vector_doubles(&mut e);
        set_normal_flag(&mut e, data, 0, 19, 1);
        let at = position(&mut e, 256.0, 128.0);
        assert_eq!(e.call(0x0053_e210, &args![this, at]).u32() & 0xff, 1);
        let other = position(&mut e, 640.0, 128.0);
        assert_eq!(e.call(0x0053_e210, &args![this, other]).u32() & 0xff, 0);
        let outside = position(&mut e, -3.0, 128.0);
        assert_eq!(e.call(0x0053_e210, &args![this, outside]).u32() & 0xff, 0);
    }

    /// The shared triangle list with `triangles` first and degenerate
    /// triangles for the rest.
    fn triangle_list(e: &mut Engine, triangles: &[[u16; 3]]) {
        let list = e.mem.alloc(512 * 6);
        for (i, triangle) in triangles.iter().enumerate() {
            for (j, index) in triangle.iter().enumerate() {
                e.mem.set_u16(list + 6 * i as u32 + 2 * j as u32, *index);
            }
        }
        e.set_global(DEFAULT_TRIANGLE_LIST, list);
        for (i, component) in [0.0f32, 0.0, 0.0].iter().enumerate() {
            e.set_global(DEFAULT_NORMAL_WORDS + 4 * i as u32, *component);
        }
    }

    #[test]
    fn normals_are_rebuilt_from_the_triangles() {
        let mut e = engine();
        vector_doubles(&mut e);
        let (this, data) = terrain(&mut e);
        triangle_list(&mut e, &[[0, 1, 17]]);
        let unitized = recorder::<(u32, u32, u32)>();
        let sink = unitized.clone();
        e.register_double(NI_POINT3_UNITIZE_VECTORS, move |_, a| {
            sink.borrow_mut().push((a[0], a[1], a[2]));
            Ret::default()
        });
        for quadrant in 0..4 {
            for vertex in 0..0x121 {
                set_normal(&mut e, data, quadrant, vertex, [9.0, 9.0, 9.0]);
                set_normal_flag(&mut e, data, quadrant, vertex, 1);
            }
        }
        e.call(0x0053_df30, &args![this, 0u32]);
        // The three vertices of the triangle got its normal (b - a) x (c - b);
        // the others were reset to the default normal (zero here).
        for quadrant in 0..4 {
            assert_eq!(normal_of(&e, data, quadrant, 0), [0.0, 0.0, 1.0]);
            assert_eq!(normal_of(&e, data, quadrant, 1), [0.0, 0.0, 1.0]);
            assert_eq!(normal_of(&e, data, quadrant, 17), [0.0, 0.0, 1.0]);
            assert_eq!(normal_of(&e, data, quadrant, 2), [0.0, 0.0, 0.0]);
            assert_eq!(normal_flag(&e, data, quadrant, 2), 0);
            assert_eq!(normal_flag(&e, data, quadrant, 288), 0);
        }
        // Each quadrant's block is unitized with 289 vectors of 12 bytes.
        let normals = e.get(data, LoadedLandData::ppNormals);
        let blocks: Vec<(u32, u32, u32)> = (0..4)
            .map(|quadrant| (element(&e, normals, quadrant), 0x121, 12))
            .collect();
        assert_eq!(*unitized.borrow(), blocks);
    }

    #[test]
    fn normals_in_bounds_only_are_rebuilt() {
        let mut e = engine();
        vector_doubles(&mut e);
        let (this, data) = terrain(&mut e);
        triangle_list(&mut e, &[[0, 1, 17]]);
        e.register(NI_POINT3_UNITIZE_VECTORS, |_, _| Ret::default());
        for vertex in 0..0x121 {
            set_normal(&mut e, data, 1, vertex, [9.0, 9.0, 9.0]);
            set_normal_flag(&mut e, data, 1, vertex, 1);
        }
        // x in 2048 ..= 2176 and y up to 2048: vertices 0 and 1 of the first
        // row only.
        let bounds = e.mem.alloc(16);
        for (i, value) in [2048i32, 2048, 2176, 0].iter().enumerate() {
            e.mem.set_i32(bounds + 4 * i as u32, *value);
        }
        e.call(0x0053_df30, &args![this, bounds]);
        assert_eq!(normal_of(&e, data, 1, 0), [0.0, 0.0, 1.0]);
        assert_eq!(normal_of(&e, data, 1, 1), [0.0, 0.0, 1.0]);
        assert_eq!(normal_flag(&e, data, 1, 0), 0);
        assert_eq!(normal_flag(&e, data, 1, 1), 0);
        // Vertex 17 is out of range: untouched.
        assert_eq!(normal_of(&e, data, 1, 17), [9.0, 9.0, 9.0]);
        assert_eq!(normal_flag(&e, data, 1, 17), 1);
        // A land that is not loaded is left alone.
        let (other, other_data) = terrain(&mut e);
        e.set(other, TESObjectLAND::Data, 0);
        set_normal(&mut e, other_data, 0, 0, [7.0, 7.0, 7.0]);
        e.call(0x0053_df30, &args![other, 0u32]);
        assert_eq!(normal_of(&e, other_data, 0, 0), [7.0, 7.0, 7.0]);
    }

    /// Doubles for the neighbour search of a land at cell (5, 7): cells and
    /// their lands.
    fn neighbour_world(e: &mut Engine) -> (Ptr<TESObjectLAND>, Vec<u32>) {
        let (this, data) = terrain(e);
        e.set(data, LoadedLandData::iCellX, 5);
        e.set(data, LoadedLandData::iCellY, 7);
        e.set(this, TESObjectLAND::pParentCell, Ptr::new(0xce11));
        e.register(CELL_GET_WORLD_SPACE, |_, _| ret(0x5a5a));
        e.set_global(DATA_HANDLER, 0x0900_0000u32);
        e.register(LOADING_MENU_VISIBLE, |_, _| ret(0));
        // Lands at (4, 8) and (6, 7) with vertex arrays; (5, 8) without
        // arrays; (4, 7) without arrays but already loaded.
        let (north_west, _) = terrain(e);
        let (east, _) = terrain(e);
        let (north, north_data) = terrain(e);
        e.set(north_data, LoadedLandData::ppVertices, Ptr::NULL);
        let (west, west_data) = terrain(e);
        e.set(west_data, LoadedLandData::ppVertices, Ptr::NULL);
        let lands = vec![north_west.addr(), east.addr(), north.addr(), west.addr()];
        let cells = lands.clone();
        e.register_double(TES_GET_CELL, move |_, a| {
            let (x, y) = (a[1] as i32, a[2] as i32);
            ret(match (x, y) {
                (4, 8) => 0xc000,
                (5, 8) => 0xc001,
                (4, 7) => 0xc002,
                (6, 7) => 0xc003,
                _ => 0,
            })
        });
        e.register_double(CELL_GET_LAND, move |_, a| {
            ret(match a[0] {
                0xc000 => cells[0],
                0xc001 => cells[2],
                0xc002 => cells[3],
                _ => cells[1],
            })
        });
        (this, lands)
    }

    #[test]
    fn neighbours_are_found_through_the_data_handler() {
        let mut e = engine();
        let (this, lands) = neighbour_world(&mut e);
        let (found, loaded) = (e.mem.alloc(0x20), e.mem.alloc(8));
        start_log(&mut e);
        e.call(0x0053_d330, &args![this, found, 0u32, loaded]);
        let log = end_log(&mut e);
        // The first query: the handler, x - 1, y + 1, the world space, 0.
        assert_eq!(
            arguments_of(&log, TES_GET_CELL)[0],
            vec![0x0900_0000, 4, 8, 0x5a5a, 0]
        );
        assert_eq!(arguments_of(&log, TES_GET_CELL).len(), 8);
        // (4, 8) has vertex arrays; (5, 8) and (4, 7) do not and are not
        // loaded without the load flag; (6, 7) has them.
        let entries: Vec<u32> = (0..8).map(|i| e.mem.u32(found + 4 * i)).collect();
        assert_eq!(entries, vec![lands[0], 0, 0, 0, lands[1], 0, 0, 0]);
        assert_eq!(e.mem.bytes(loaded, 8), vec![0; 8]);
    }

    #[test]
    fn neighbours_without_arrays_are_loaded_on_request() {
        let mut e = engine();
        let (this, lands) = neighbour_world(&mut e);
        let (found, loaded) = (e.mem.alloc(0x20), e.mem.alloc(8));
        e.call(0x0053_d330, &args![this, found, 1u32, loaded]);
        let entries: Vec<u32> = (0..8).map(|i| e.mem.u32(found + 4 * i)).collect();
        // (5, 8) is in the list too: loading is requested for it as well.
        assert_eq!(
            entries,
            vec![lands[0], lands[2], 0, lands[3], lands[1], 0, 0, 0]
        );
        assert_eq!(e.mem.bytes(loaded, 8), vec![0, 1, 0, 1, 0, 0, 0, 0]);
        // Without the flag array nothing is loaded.
        let found = e.mem.alloc(0x20);
        e.call(0x0053_d330, &args![this, found, 1u32, 0u32]);
        let entries: Vec<u32> = (0..8).map(|i| e.mem.u32(found + 4 * i)).collect();
        assert_eq!(entries, vec![lands[0], 0, 0, 0, lands[1], 0, 0, 0]);
        // A null array: nothing happens.
        start_log(&mut e);
        e.call(0x0053_d330, &args![this, 0u32, 1u32, loaded]);
        assert!(called(&mut e).is_empty());
    }

    // ---- Border normals of one vertex --------------------------------------------------------

    /// The call of `fn_0053e290` for vertex `vertex` of quadrant `quadrant`
    /// with the neighbour array `lands`; returns the output normal.
    fn border_normal(
        e: &mut Engine,
        this: Ptr<TESObjectLAND>,
        quadrant: i32,
        vertex: i32,
        lands: u32,
    ) -> [f32; 3] {
        let out = e.mem.alloc(12);
        put_v3(e, out, [-1.0, -1.0, -1.0]);
        e.call(0x0053_e290, &args![this, quadrant, vertex, out, lands]);
        v3(e, out)
    }

    #[test]
    fn border_normal_is_left_alone_when_set_or_without_neighbours() {
        let mut e = engine();
        vector_doubles(&mut e);
        let (this, data) = terrain(&mut e);
        let lands = e.mem.alloc(0x20);
        set_normal_flag(&mut e, data, 0, 40, 1);
        start_log(&mut e);
        assert_eq!(border_normal(&mut e, this, 0, 40, lands), [-1.0; 3]);
        assert!(!called_contains(&end_log(&mut e), NI_POINT3_ADD));
        // No neighbour array: nothing either.
        set_normal_flag(&mut e, data, 0, 41, 0);
        assert_eq!(border_normal(&mut e, this, 0, 41, 0), [-1.0; 3]);
        assert_eq!(normal_flag(&e, data, 0, 41), 0);
    }

    #[test]
    fn border_normal_of_an_interior_vertex_is_its_own() {
        let mut e = engine();
        vector_doubles(&mut e);
        let (this, data) = terrain(&mut e);
        let lands = e.mem.alloc(0x20);
        set_normal(&mut e, data, 0, 40, [4.0, 0.0, 0.0]);
        assert_eq!(border_normal(&mut e, this, 0, 40, lands), [4.0, 0.0, 0.0]);
        assert_eq!(normal_flag(&e, data, 0, 40), 1);
    }

    #[test]
    fn border_normal_averages_the_missing_neighbour_lands() {
        let mut e = engine();
        vector_doubles(&mut e);
        let (this, data) = terrain(&mut e);
        let lands = e.mem.alloc(0x20);
        // The top edge of quadrant 0 has one land neighbour (lands[6], null):
        // its answer is the default normal (0, 0, 1).
        set_normal(&mut e, data, 0, 5, [1.0, 0.0, 0.0]);
        assert_eq!(border_normal(&mut e, this, 0, 5, lands), [0.5, 0.0, 0.5]);
        assert_eq!(normal_flag(&e, data, 0, 5), 1);
        // The corner has three: above, left and the diagonal.
        set_normal(&mut e, data, 0, 0, [4.0, 0.0, 0.0]);
        assert_eq!(border_normal(&mut e, this, 0, 0, lands), [1.0, 0.0, 0.75]);
        // The bottom-right corner of quadrant 3 as well (right, below,
        // diagonal).
        set_normal(&mut e, data, 3, 288, [0.0, 8.0, 0.0]);
        assert_eq!(border_normal(&mut e, this, 3, 288, lands), [0.0, 2.0, 0.75]);
    }

    #[test]
    fn border_normal_uses_the_neighbouring_quadrant() {
        let mut e = engine();
        vector_doubles(&mut e);
        let (this, data) = terrain(&mut e);
        let lands = e.mem.alloc(0x20);
        // Vertex 276 of quadrant 0 is on the edge shared with quadrant 2
        // (vertex 4 there).
        set_normal(&mut e, data, 0, 276, [1.0, 0.0, 0.0]);
        set_normal(&mut e, data, 2, 4, [0.0, 1.0, 0.0]);
        // The neighbour's normal is not set: the two are averaged.
        assert_eq!(border_normal(&mut e, this, 0, 276, lands), [0.5, 0.5, 0.0]);
        // Set: it is taken as it is.
        set_normal_flag(&mut e, data, 2, 4, 1);
        set_normal_flag(&mut e, data, 0, 276, 0);
        assert_eq!(border_normal(&mut e, this, 0, 276, lands), [0.0, 1.0, 0.0]);
        assert_eq!(normal_flag(&e, data, 0, 276), 1);
        // The edge with the quadrant above (vertex 3 of quadrant 2 is the
        // vertex 275 of quadrant 0's...): quadrant 2, vertex 3 looks at
        // quadrant 0 vertex 275.
        set_normal(&mut e, data, 0, 275, [0.0, 0.0, 6.0]);
        set_normal(&mut e, data, 2, 3, [2.0, 0.0, 0.0]);
        assert_eq!(border_normal(&mut e, this, 2, 3, lands), [1.0, 0.0, 3.0]);
        // The left and right edges: quadrant 0's right edge vertex 33 looks at
        // quadrant 1's vertex 17; quadrant 1's vertex 17 looks at nothing
        // on its left (that is a land edge: lands[3], null).
        set_normal(&mut e, data, 0, 33, [2.0, 0.0, 0.0]);
        set_normal(&mut e, data, 1, 17, [0.0, 2.0, 0.0]);
        assert_eq!(border_normal(&mut e, this, 0, 33, lands), [1.0, 1.0, 0.0]);
    }

    #[test]
    fn border_normal_of_a_quadrant_corner_uses_three_neighbours() {
        let mut e = engine();
        vector_doubles(&mut e);
        let (this, data) = terrain(&mut e);
        let lands = e.mem.alloc(0x20);
        // The bottom-right vertex of quadrant 0 touches quadrants 2, 1 and
        // the diagonal quadrant 3.
        set_normal(&mut e, data, 0, 288, [1.0, 0.0, 0.0]);
        set_normal(&mut e, data, 2, 16, [0.0, 1.0, 0.0]);
        set_normal(&mut e, data, 1, 272, [0.0, 0.0, 1.0]);
        set_normal(&mut e, data, 3, 0, [1.0, 1.0, 1.0]);
        assert_eq!(border_normal(&mut e, this, 0, 288, lands), [0.5, 0.5, 0.5]);
        // The diagonal one set: it ends the search.
        set_normal(&mut e, data, 3, 0, [0.25, 0.5, 0.75]);
        set_normal_flag(&mut e, data, 3, 0, 1);
        set_normal_flag(&mut e, data, 0, 288, 0);
        assert_eq!(
            border_normal(&mut e, this, 0, 288, lands),
            [0.25, 0.5, 0.75]
        );
    }

    #[test]
    fn border_normal_asks_the_neighbouring_land() {
        let mut e = engine();
        vector_doubles(&mut e);
        let (this, data) = terrain(&mut e);
        // A land of the same cell whose normals are all set.
        let (neighbour, neighbour_data) = terrain(&mut e);
        for quadrant in 0..4 {
            for vertex in 0..0x121 {
                set_normal_flag(&mut e, neighbour_data, quadrant, vertex, 1);
            }
        }
        let lands = e.mem.alloc(0x20);
        e.mem.set_u32(lands + 4 * 6, neighbour.addr());
        set_normal(&mut e, data, 0, 5, [1.0, 0.0, 0.0]);
        // Its answer is its interpolated normal, (0, 0, 1) here.
        assert_eq!(border_normal(&mut e, this, 0, 5, lands), [0.0, 0.0, 1.0]);
        assert_eq!(normal_flag(&e, data, 0, 5), 1);
    }

    // ---- Border normals of the whole land --------------------------------------------------------

    #[test]
    fn border_normals_are_computed_for_the_border_vertices() {
        let mut e = engine();
        vector_doubles(&mut e);
        let (this, data) = terrain(&mut e);
        let lands = e.mem.alloc(0x20);
        e.call(0x0053_d8f0, &args![this, lands, 0u32]);
        for quadrant in 0..4 {
            let flagged: Vec<u32> = (0..0x121)
                .filter(|vertex| normal_flag(&e, data, quadrant, *vertex) != 0)
                .collect();
            let border: Vec<u32> = (0..0x121u32)
                .filter(|v| v / 17 == 0 || v / 17 == 16 || v % 17 == 0 || v % 17 == 16)
                .collect();
            assert_eq!(flagged, border);
            assert_eq!(border.len(), 64);
        }
        // Not all neighbours exist: flag 0x10 stays clear.
        assert_eq!(e.get(this, TESObjectLAND::Data), FLAG_LOADED);
    }

    #[test]
    fn border_normals_find_the_neighbours_when_none_are_given() {
        let mut e = engine();
        vector_doubles(&mut e);
        let (this, data) = terrain(&mut e);
        e.set_global(DATA_HANDLER, 0x0900_0000u32);
        e.register(TES_GET_CELL, |_, _| ret(0));
        start_log(&mut e);
        e.call(0x0053_d8f0, &args![this, 0u32, 0u32]);
        let log = end_log(&mut e);
        assert_eq!(arguments_of(&log, TES_GET_CELL).len(), 8);
        assert_eq!(normal_flag(&e, data, 2, 0), 1);
    }

    #[test]
    fn border_normals_respect_the_bounds_and_report_a_complete_ring() {
        let mut e = engine();
        vector_doubles(&mut e);
        let (this, data) = terrain(&mut e);
        // Eight neighbours that are never asked (the bounds exclude every
        // vertex): flag 0x10 is set.
        let lands = e.mem.alloc(0x20);
        for i in 0..8 {
            e.mem.set_u32(lands + 4 * i, 0xaaaa);
        }
        let bounds = e.mem.alloc(16);
        for (i, value) in [10000i32, 10000, 10001, 10000].iter().enumerate() {
            e.mem.set_i32(bounds + 4 * i as u32, *value);
        }
        e.call(0x0053_d8f0, &args![this, lands, bounds]);
        assert_eq!(normal_flag(&e, data, 0, 0), 0);
        assert_eq!(e.get(this, TESObjectLAND::Data), FLAG_LOADED | FLAG_EDITED);
        // A land that is not loaded does nothing.
        let (other, other_data) = terrain(&mut e);
        e.set(other, TESObjectLAND::Data, 0);
        e.call(0x0053_d8f0, &args![other, lands, 0u32]);
        assert_eq!(normal_flag(&e, other_data, 0, 0), 0);
    }

    // ---- Border normals with the neighbours -------------------------------------------------------

    #[test]
    fn normals_are_recomputed_for_the_neighbours_and_restored() {
        let mut e = engine();
        vector_doubles(&mut e);
        let (this, data) = terrain(&mut e);
        triangle_list(&mut e, &[[0, 1, 17]]);
        // The land to the west, with arrays: its normals are (1, 0, 0).
        e.set(data, LoadedLandData::iCellX, 5);
        e.set(data, LoadedLandData::iCellY, 7);
        e.set(this, TESObjectLAND::pParentCell, Ptr::new(0xce11));
        e.register(CELL_GET_WORLD_SPACE, |_, _| ret(0x5a5a));
        e.set_global(DATA_HANDLER, 0x0900_0000u32);
        let (west, west_data) = terrain(&mut e);
        e.set(west_data, LoadedLandData::iCellX, 4);
        e.set(west_data, LoadedLandData::iCellY, 7);
        for quadrant in 0..4 {
            for vertex in 0..0x121 {
                set_normal(&mut e, west_data, quadrant, vertex, [1.0, 0.0, 0.0]);
            }
        }
        let west_addr = west.addr();
        e.register_double(TES_GET_CELL, |_, a| {
            ret(if (a[1], a[2]) == (4, 7) { 0xc000 } else { 0 })
        });
        e.register_double(CELL_GET_LAND, move |_, _| ret(west_addr));
        start_log(&mut e);
        assert_eq!(e.call(0x0053_db20, &args![this]).u32() & 0xff, 1);
        let log = end_log(&mut e);
        assert_eq!(arguments_of(&log, TES_GET_CELL).len(), 8);
        // The neighbour's normals are back.
        for quadrant in 0..4 {
            for vertex in [0u32, 17, 100, 288] {
                assert_eq!(normal_of(&e, west_data, quadrant, vertex), [1.0, 0.0, 0.0]);
            }
        }
        // This land's normals: the triangle's, rounded to bytes and unitized;
        // the border flags are set.
        assert_eq!(normal_of(&e, data, 0, 1), [0.0, 0.0, 1.0]);
        assert_eq!(normal_flag(&e, data, 0, 1), 1);
        assert_eq!(normal_flag(&e, data, 0, 40), 0);
        assert_eq!(normal_of(&e, data, 0, 40), [0.0, 0.0, 0.0]);
        // The saved normals were copied with the right sizes.
        let copies = arguments_of(&log, MEMORY_COPY);
        assert_eq!(copies.len(), 8);
        assert!(copies.iter().all(|call| call[2] == 0xd8c));
    }

    #[test]
    fn normals_pass_is_skipped_for_unloaded_lands_and_world_space_land_files() {
        let mut e = engine();
        vector_doubles(&mut e);
        let (this, _data) = terrain(&mut e);
        e.set(this, TESObjectLAND::Data, 0);
        assert_eq!(e.call(0x0053_db20, &args![this]).u32() & 0xff, 0);
        e.set(
            this,
            TESObjectLAND::Data,
            FLAG_LOADED | FLAG_FROM_WORLD_SPACE,
        );
        start_log(&mut e);
        assert_eq!(e.call(0x0053_db20, &args![this]).u32() & 0xff, 0);
        assert!(called(&mut e).is_empty());
    }

    #[test]
    fn border_normals_round_to_signed_bytes() {
        let mut e = engine();
        vector_doubles(&mut e);
        let (this, data) = terrain(&mut e);
        e.set_global(DATA_HANDLER, 0x0900_0000u32);
        e.register(TES_GET_CELL, |_, _| ret(0));
        // A triangle list with no triangles, and a default normal that is
        // already unit: (0.6, 0, 0.8) is rounded to 76/127 and 101/127 and
        // unitized again (by the double).
        let list = e.mem.alloc(512 * 6);
        e.set_global(DEFAULT_TRIANGLE_LIST, list);
        for (i, component) in [0.6f32, 0.0, 0.8].iter().enumerate() {
            e.set_global(DEFAULT_NORMAL_WORDS + 4 * i as u32, *component);
        }
        e.call(0x0053_db20, &args![this]);
        let expected = normalize_v3([76.0 / 127.0, 0.0, 101.0 / 127.0]);
        let actual = normal_of(&e, data, 1, 40);
        for i in 0..3 {
            assert!(
                (actual[i] - expected[i]).abs() < 1e-6,
                "{actual:?} {expected:?}"
            );
        }
    }

    // ---- Fourth session: height, colour and border of a position ---------------

    /// The pages and constants the fourth session's functions read.
    fn fourth_session_data(e: &mut Engine) {
        for page in [0x0102_0000u32, 0x0102_4000] {
            e.map(page, 0x1000);
        }
        e.set_global(F64_FIVE, 5.0f64);
        e.set_global(F64_LARGEST, f32::MAX as f64);
        e.set_global(F64_LOWEST, -(f32::MAX as f64));
        e.set_global(F32_HALF, 0.5f32);
    }

    fn set_vertex_height(
        e: &mut Engine,
        data: Ptr<LoadedLandData>,
        quadrant: u32,
        vertex: u32,
        height: f32,
    ) {
        let vertices = e.get(data, LoadedLandData::ppVertices);
        let block = element(e, vertices, quadrant);
        e.mem.set_f32(block + 12 * vertex + 8, height);
    }

    fn set_vertex_color(
        e: &mut Engine,
        data: Ptr<LoadedLandData>,
        quadrant: u32,
        vertex: u32,
        color: [f32; 4],
    ) {
        let colors = e.get(data, LoadedLandData::ppColorsA);
        let block = element(e, colors, quadrant);
        for (i, component) in color.iter().enumerate() {
            e.mem
                .set_f32(block + 16 * vertex + 4 * i as u32, *component);
        }
    }

    /// Doubles for the colour arithmetic.
    fn color_doubles(e: &mut Engine) {
        fn read(e: &Engine, at: u32) -> [f32; 4] {
            floats(e, at, 4).try_into().unwrap()
        }
        fn write(e: &mut Engine, at: u32, value: [f32; 4]) {
            for (i, component) in value.iter().enumerate() {
                e.mem.set_f32(at + 4 * i as u32, *component);
            }
        }
        e.register(COLOR_SCALED, |e, a| {
            let color = read(e, a[2]);
            let scalar = f32::from_bits(a[1]);
            write(e, a[0], color.map(|c| c * scalar));
            ret(a[0])
        });
        e.register(COLOR_TIMES_SCALAR, |e, a| {
            let color = read(e, a[0]);
            let scalar = f32::from_bits(a[2]);
            write(e, a[1], color.map(|c| c * scalar));
            ret(a[1])
        });
        e.register(COLOR_ADD, |e, a| {
            let (p, q) = (read(e, a[0]), read(e, a[2]));
            write(
                e,
                a[1],
                [p[0] + q[0], p[1] + q[1], p[2] + q[2], p[3] + q[3]],
            );
            ret(a[1])
        });
    }

    /// A `CoordData` for the triangle `(vertex2, vertex0, vertex1)` of the
    /// quadrant 0, block offsets `(x, y)`.
    fn triangle_info(
        e: &mut Engine,
        vertices: (i32, i32, i32),
        offsets: (f32, f32),
        odd: bool,
        second: bool,
    ) -> Ptr<CoordData> {
        let info = coord_info(e);
        e.set(info, CoordData::Quadrant, 0);
        e.set(info, CoordData::TriangleVertex2, vertices.0);
        e.set(info, CoordData::TriangleVertex0, vertices.1);
        e.set(info, CoordData::TriangleVertex1, vertices.2);
        e.set(info, CoordData::BlockOffsetX, offsets.0);
        e.set(info, CoordData::BlockOffsetY, offsets.1);
        e.set(info, CoordData::OddBlock, odd as u8);
        e.set(info, CoordData::SecondTriangle, second as u8);
        info
    }

    #[test]
    fn mesh_of_a_position_is_the_first_geometry_of_its_quadrant_node() {
        let mut e = engine();
        vector_doubles(&mut e);
        let (this, data) = terrain(&mut e);
        let meshes = e.mem.alloc(16);
        e.mem.set_u32(meshes, 0x7000);
        e.set(data, LoadedLandData::ppMesh, Ptr::new(meshes));
        e.register(NODE_FIRST_GEOMETRY, |_, _| ret(0x7777));
        let inside = position(&mut e, 300.0, 200.0);
        start_log(&mut e);
        assert_eq!(e.call(0x0053_f0e0, &args![this, inside]).u32(), 0x7777);
        let log = end_log(&mut e);
        assert_eq!(
            arguments_of(&log, NODE_FIRST_GEOMETRY),
            vec![vec![0x7000, 0]]
        );
        // Outside the cell nothing is asked.
        let outside = position(&mut e, -5.0, 200.0);
        start_log(&mut e);
        assert_eq!(e.call(0x0053_f0e0, &args![this, outside]).u32(), 0);
        assert!(!called(&mut e).contains(&NODE_FIRST_GEOMETRY));
    }

    #[test]
    fn first_geometry_needs_data_meshes_and_a_node() {
        let mut e = engine();
        e.register(NODE_FIRST_GEOMETRY, |_, a| ret(a[0] + 1));
        let info = coord_info(&mut e);
        e.set(info, CoordData::Quadrant, 2);
        let bare = land(&mut e);
        assert_eq!(e.call(0x0053_f120, &args![bare, info]).u32(), 0);
        let (this, data) = land_with_data(&mut e);
        assert_eq!(e.call(0x0053_f120, &args![this, info]).u32(), 0);
        let meshes = e.mem.alloc(16);
        e.set(data, LoadedLandData::ppMesh, Ptr::new(meshes));
        assert_eq!(e.call(0x0053_f120, &args![this, info]).u32(), 0);
        e.mem.set_u32(meshes + 8, 0x4000);
        assert_eq!(e.call(0x0053_f120, &args![this, info]).u32(), 0x4001);
    }

    #[test]
    fn land_height_is_the_plane_of_the_triangle() {
        let mut e = engine();
        vector_doubles(&mut e);
        e.set_global(F32_MINUS_2048, -2048.0f32);
        let (this, data) = terrain(&mut e);
        // Position (300, 200): an odd block, first triangle; vertices 19,
        // 20 and 36 are `a`, `b` and `c`; the offsets in the block are
        // (44, 72).
        for (vertex, height) in [(19, 10.0), (20, 30.0), (36, 50.0)] {
            set_vertex_height(&mut e, data, 0, vertex, height);
        }
        let at = position(&mut e, 300.0, 200.0);
        let out = e.mem.alloc(4);
        assert_eq!(e.call(0x0053_f180, &args![this, at, out]).u32() & 0xff, 1);
        // 10 + (30 - 10) / 128 * 44 + (50 - 10) / 128 * 72
        assert_eq!(e.mem.f32(out), 39.375);
        // Outside the cell the default height is stored and false returned.
        let outside = position(&mut e, 5000.0, 200.0);
        assert_eq!(
            e.call(0x0053_f180, &args![this, outside, out]).u32() & 0xff,
            0
        );
        assert_eq!(e.mem.f32(out), -2048.0);
    }

    #[test]
    fn triangle_height_of_each_kind_of_block() {
        let mut e = engine();
        vector_doubles(&mut e);
        let (this, data) = terrain(&mut e);
        for (vertex, height) in [(1, 10.0), (2, 30.0), (3, 70.0)] {
            set_vertex_height(&mut e, data, 0, vertex, height);
        }
        let out = e.mem.alloc(4);
        // (a, b, c) = (10, 30, 70); the vertices are 1, 2 and 3.
        for (odd, second, offsets, expected) in [
            (false, false, (96.0, 32.0), 30.0),
            (false, true, (96.0, 32.0), 70.0),
            (true, false, (32.0, 32.0), 30.0),
            (true, true, (32.0, 32.0), 70.0),
        ] {
            let info = triangle_info(&mut e, (1, 2, 3), offsets, odd, second);
            let result = e.call(0x0053_f1e0, &args![this, info, out]);
            assert_eq!(result.u32() & 0xff, 1);
            assert_eq!(e.mem.f32(out), expected, "{odd} {second}");
        }
    }

    #[test]
    fn triangle_height_rounds_each_step_at_the_control_words_precision() {
        let mut e = engine();
        vector_doubles(&mut e);
        let (this, data) = terrain(&mut e);
        // (a, b, c) = (2^24 + 2, 2^24, 2^24 + 4): both slopes are 1/64 and
        // the offsets 48 give 0.75 each. At 24-bit precision b + 0.75 rounds
        // back to 2^24, and so does the second 0.75; at 53-bit the sum
        // 2^24 + 1.5 rounds to 2^24 + 2 when stored.
        for (vertex, height) in [(1, 16_777_218.0), (2, 16_777_216.0), (3, 16_777_220.0)] {
            set_vertex_height(&mut e, data, 0, vertex, height);
        }
        let out = e.mem.alloc(4);
        let info = triangle_info(&mut e, (1, 2, 3), (48.0, 48.0), false, false);
        for (word, expected) in [(0x007fu16, 16_777_216.0f32), (0x027f, 16_777_218.0)] {
            crate::units::crt::set_x87_control_word(&mut e, word);
            e.call(0x0053_f1e0, &args![this, info, out]);
            assert_eq!(e.mem.f32(out), expected, "{word:#x}");
        }
        crate::units::crt::set_x87_control_word(&mut e, 0x027f);
    }

    #[test]
    fn quadrant_extents_are_the_lowest_and_highest_vertex() {
        let mut e = engine();
        vector_doubles(&mut e);
        e.register(NI_POINT2_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            ret(a[0])
        });
        let (this, data) = terrain(&mut e);
        set_vertex_height(&mut e, data, 1, 17, -7.0);
        set_vertex_height(&mut e, data, 1, 200, 99.0);
        let out = e.mem.alloc(8);
        assert_eq!(e.call(0x0053_f390, &args![this, out, 1u32]).u32(), out);
        assert_eq!(floats(&e, out, 2), vec![-7.0, 99.0]);
        // A quadrant number from 16 on leaves the initial extents.
        e.call(0x0053_f390, &args![this, out, 16u32]);
        assert_eq!(floats(&e, out, 2), vec![f32::MAX, -f32::MAX]);
    }

    #[test]
    fn height_extents_of_the_land() {
        let mut e = engine();
        vector_doubles(&mut e);
        fourth_session_data(&mut e);
        e.set_global(F32_MINUS_2048, -2048.0f32);
        e.register(NI_POINT2_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            ret(a[0])
        });
        let out = e.mem.alloc(8);
        // Without loaded data: the initial extents.
        let bare = land(&mut e);
        assert_eq!(e.call(0x0053_f440, &args![bare, out]).u32(), out);
        assert_eq!(floats(&e, out, 2), vec![f32::MAX, -f32::MAX]);
        // Known extents are returned as they are.
        let (this, data) = terrain(&mut e);
        e.set(data, LoadedLandData::HeightExtentsMin, -3.0);
        e.set(data, LoadedLandData::HeightExtentsMax, 9.0);
        start_log(&mut e);
        e.call(0x0053_f440, &args![this, out]);
        assert_eq!(called(&mut e), vec![]);
        assert_eq!(floats(&e, out, 2), vec![-3.0, 9.0]);
        // Initial extents with height data: the quadrants are folded in.
        e.set(data, LoadedLandData::HeightExtentsMin, f32::MAX);
        e.set(data, LoadedLandData::HeightExtentsMax, -f32::MAX);
        e.set(this, TESObjectLAND::Data, FLAG_LOADED | FLAG_HEIGHTS);
        set_vertex_height(&mut e, data, 0, 5, -4.0);
        set_vertex_height(&mut e, data, 3, 6, 12.0);
        e.call(0x0053_f440, &args![this, out]);
        assert_eq!(floats(&e, out, 2), vec![-4.0, 12.0]);
        assert_eq!(e.get(data, LoadedLandData::HeightExtentsMin), -4.0);
        assert_eq!(e.get(data, LoadedLandData::HeightExtentsMax), 12.0);
        // Initial extents without height data: the default height twice.
        e.set(data, LoadedLandData::HeightExtentsMin, f32::MAX);
        e.set(data, LoadedLandData::HeightExtentsMax, -f32::MAX);
        e.set(this, TESObjectLAND::Data, FLAG_LOADED);
        e.call(0x0053_f440, &args![this, out]);
        assert_eq!(floats(&e, out, 2), vec![-2048.0, -2048.0]);
    }

    /// The expected colour of the blend described at [`fn_0053f570`].
    fn expected_blend(colors: [[f32; 4]; 3], s: f32, t: f32, odd: bool, second: bool) -> [f32; 4] {
        let [c1, c2, c3] = colors;
        let mix = |p: [f32; 4], q: [f32; 4], w: f32, scale: f32, last: [f32; 4], lw: f32| {
            let mut out = [0.0; 4];
            for i in 0..4 {
                out[i] = ((1.0 - w) * p[i] + w * q[i]) * scale + lw * last[i];
            }
            out
        };
        match (odd, second) {
            (false, false) => mix(c2, c1, s, 1.0 - t, c3, t),
            (false, true) => mix(c1, c3, s, t, c2, 1.0 - t),
            (true, false) => mix(c1, c2, s, 1.0 - t, c3, t),
            (true, true) => mix(c3, c1, s, t, c2, 1.0 - t),
        }
    }

    #[test]
    fn colour_at_a_position_blends_the_triangle() {
        let mut e = engine();
        vector_doubles(&mut e);
        color_doubles(&mut e);
        let (this, data) = terrain(&mut e);
        // Colours for every vertex: a different one per vertex index.
        for vertex in 0..0x121u32 {
            let value = vertex as f32 / 400.0;
            set_vertex_color(
                &mut e,
                data,
                0,
                vertex,
                [value, 1.0 - value, 0.5 * value, 1.0],
            );
        }
        let out = e.mem.alloc(16);
        // One position of each kind of block (even/odd, first/second
        // triangle); the colours of the three vertices come from `info`.
        let mut seen = vec![];
        for (x, y) in [
            (158.0, 138.0),
            (138.0, 158.0),
            (10.0, 170.0),
            (100.0, 170.0),
        ] {
            let at = position(&mut e, x, y);
            let info = coord_info(&mut e);
            e.call(0x0053_b550, &args![this, info, at, 0u32]);
            let odd = e.get(info, CoordData::OddBlock) != 0;
            let second = e.get(info, CoordData::SecondTriangle) != 0;
            seen.push((odd, second));
            let colors = [
                CoordData::TriangleVertex2,
                CoordData::TriangleVertex0,
                CoordData::TriangleVertex1,
            ]
            .map(|field| {
                let vertex = e.get(info, field) as u32;
                let value = vertex as f32 / 400.0;
                [value, 1.0 - value, 0.5 * value, 1.0]
            });
            let result = e.call(0x0053_f570, &args![this, at, out]);
            assert_eq!(result.u32() & 0xff, 1);
            let expected = expected_blend(
                colors,
                (x % 128.0) / 128.0,
                (y % 128.0) / 128.0,
                odd,
                second,
            );
            let actual = floats(&e, out, 4);
            for i in 0..4 {
                assert!(
                    (actual[i] - expected[i]).abs() < 1e-5,
                    "{x} {y}: {actual:?} {expected:?}"
                );
            }
        }
        seen.sort();
        assert_eq!(
            seen,
            vec![(false, false), (false, true), (true, false), (true, true)]
        );
        // Outside the land nothing is stored.
        let outside = position(&mut e, -1.0, 5.0);
        e.mem.set_f32(out, 77.0);
        assert_eq!(
            e.call(0x0053_f570, &args![this, outside, out]).u32() & 0xff,
            0
        );
        assert_eq!(e.mem.f32(out), 77.0);
    }

    #[test]
    fn colour_of_the_nearest_vertex() {
        let mut e = engine();
        let (this, data) = terrain(&mut e);
        set_vertex_color(&mut e, data, 1, 5, [0.1, 0.2, 0.3, 0.4]);
        let info = coord_info(&mut e);
        e.set(info, CoordData::Quadrant, 1);
        e.set(info, CoordData::NearestVertex, 5);
        let out = e.mem.alloc(16);
        assert_eq!(e.call(0x0053_fa10, &args![this, info, out]).u32() & 0xff, 1);
        assert_eq!(floats(&e, out, 4), vec![0.1, 0.2, 0.3, 0.4]);
    }

    // ---- The border lines -------------------------------------------------------

    /// A virtual call on the cell node: `(slot, arguments)`.
    type NodeCall = (u32, Vec<u32>);
    /// A virtual call on a property: `(slot, this, arguments)`.
    type PropertyCall = (u32, u32, Vec<u32>);

    /// The doubles of the border builder and the recorder of the cell
    /// node's virtual calls `(slot, arguments)`.
    fn border_doubles(e: &mut Engine) -> (u32, Rc<RefCell<Vec<NodeCall>>>) {
        let node_calls = recorder();
        let node = object_with_vtable(
            e,
            0x20,
            &[
                (BORDER_OWNER_SLOT, 0x00a0_0001),
                (NODE_ATTACH_BORDER_SLOT, 0x00a0_0002),
            ],
        );
        for (address, slot) in [
            (0x00a0_0001u32, BORDER_OWNER_SLOT),
            (0x00a0_0002, NODE_ATTACH_BORDER_SLOT),
        ] {
            let node_calls = node_calls.clone();
            e.register_double(address, move |_, a| {
                node_calls.borrow_mut().push((slot, a.to_vec()));
                Ret::default()
            });
        }
        e.register_double(CELL_GET_3D, move |_, _| ret(node));
        e.register(NI_ALLOCATE_OBJECT, |e, a| ret(e.mem.alloc(a[0].max(0x200))));
        e.register(TEXTURING_PROPERTY_SET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(NI_LINES_CONSTRUCT, |_, a| ret(a[0]));
        e.register(TEXTURING_PROPERTY_CONSTRUCT, |_, a| ret(a[0]));
        e.register(NO_LIGHTING_PROPERTY_CONSTRUCT, |_, a| ret(a[0]));
        let slot = e.mem.alloc(4);
        e.register_double(ARRAY_ELEMENT_ADDRESS, move |_, _| ret(slot));
        for address in [
            NODE_SET_TRANSLATE,
            NODE_ATTACH_PROPERTY,
            PREPARE_OBJECT,
            NODE_UPDATE_PROPERTIES,
            UPDATE_DATA_CONSTRUCT,
            NODE_UPDATE,
            TEXTURING_PROPERTY_AFTER_MAP,
            SHADER_PROPERTY_SET_WORD_0X58,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(SETTING_GET_INT_ADDRESS, |_, a| ret(a[0] + 4));
        for (setting, value) in [
            (BORDER_ODD_RED, 51),
            (BORDER_ODD_GREEN, 102),
            (BORDER_ODD_BLUE, 153),
            (BORDER_EVEN_RED, 204),
            (BORDER_EVEN_GREEN, 255),
            (BORDER_EVEN_BLUE, 0),
        ] {
            e.mem.set_i32(setting + 4, value);
        }
        (node, node_calls)
    }

    #[test]
    fn border_builder_does_nothing_without_vertex_arrays_or_a_node() {
        let mut e = engine();
        vector_doubles(&mut e);
        border_doubles(&mut e);
        let bare = land(&mut e);
        start_log(&mut e);
        e.call(0x0053_fa40, &args![bare, 1u32]);
        assert_eq!(called(&mut e), vec![LAND_HAS_VERTEX_ARRAYS]);
        let (this, _data) = terrain(&mut e);
        e.register(CELL_GET_3D, |_, _| ret(0));
        start_log(&mut e);
        e.call(0x0053_fa40, &args![this, 1u32]);
        assert_eq!(called(&mut e), vec![LAND_HAS_VERTEX_ARRAYS, CELL_GET_3D]);
    }

    #[test]
    fn border_builder_detaches_the_old_lines() {
        let mut e = engine();
        vector_doubles(&mut e);
        let (node, node_calls) = border_doubles(&mut e);
        let (this, data) = terrain(&mut e);
        let slot = data.addr() + LoadedLandData::spBorder.off;
        e.mem.set_u32(slot, 0x9000);
        e.call(0x0053_fa40, &args![this, 0u32]);
        assert_eq!(
            node_calls.borrow().clone(),
            vec![(BORDER_OWNER_SLOT, vec![node, 0x9000])]
        );
        assert_eq!(e.mem.u32(slot), 0);
        // Nothing was built.
        assert!(e.mem.u32(slot) == 0);
    }

    #[test]
    fn border_builder_makes_the_lines_around_the_cell() {
        let mut e = engine();
        vector_doubles(&mut e);
        fourth_session_data(&mut e);
        let (node, node_calls) = border_doubles(&mut e);
        let (this, data) = terrain(&mut e);
        let slot = data.addr() + LoadedLandData::spBorder.off;
        start_log(&mut e);
        e.call(0x0053_fa40, &args![this, 1u32]);
        let log = end_log(&mut e);
        // The lines: 128 vertices.
        let lines = arguments_of(&log, NI_LINES_CONSTRUCT);
        assert_eq!(lines.len(), 1);
        let call = &lines[0];
        assert_eq!(&call[1..2], &[0x80]);
        assert_eq!(&call[4..7], &[0, 0, 0]);
        let (positions, colors, flags) = (call[2], call[3], call[7]);
        assert_eq!(e.mem.u32(slot), call[0]);
        // Every index is used once; the flags are set.
        assert!((0..0x80).all(|i| e.mem.u8(flags + i) == 1));
        // Index 0 is the corner; the vertex height is 0 and 5 is added.
        assert_eq!(v3(&e, positions), [0.0, 0.0, 5.0]);
        // Index 5: the sixth vertex of the first column (x = 5 * 128).
        assert_eq!(v3(&e, positions + 12 * 5), [640.0, 0.0, 5.0]);
        // Column 0x20, row 7 gives index 7 + 0x20.
        assert_eq!(v3(&e, positions + 12 * 39), [4096.0, 896.0, 5.0]);
        // Row 0x20, column 3 gives 0x60 - 3; column 0, row 3 gives 0x80 - 3.
        assert_eq!(v3(&e, positions + 12 * 93), [384.0, 4096.0, 5.0]);
        assert_eq!(v3(&e, positions + 12 * 125), [0.0, 384.0, 5.0]);
        // Colours by the parity of the index: odd 51/102/153, even
        // 204/255/0 (of 255).
        assert_eq!(
            floats(&e, colors + 16 * 5, 4),
            vec![51.0 / 255.0, 102.0 / 255.0, 153.0 / 255.0, 1.0]
        );
        assert_eq!(
            floats(&e, colors + 16 * 6, 4),
            vec![204.0 / 255.0, 1.0, 0.0, 1.0]
        );
        // The lines are moved to the corner, given the two properties, and
        // attached to the cell's node with 1.
        let order: Vec<u32> = log
            .iter()
            .map(|(a, _)| *a)
            .filter(|a| {
                [
                    NI_LINES_CONSTRUCT,
                    NODE_SET_TRANSLATE,
                    TEXTURING_PROPERTY_CONSTRUCT,
                    TEXTURING_PROPERTY_AFTER_MAP,
                    NODE_ATTACH_PROPERTY,
                    NO_LIGHTING_PROPERTY_CONSTRUCT,
                    SHADER_PROPERTY_SET_WORD_0X58,
                    PREPARE_OBJECT,
                    NODE_UPDATE_PROPERTIES,
                    UPDATE_DATA_CONSTRUCT,
                    NODE_UPDATE,
                ]
                .contains(a)
            })
            .collect();
        assert_eq!(
            order,
            vec![
                NI_LINES_CONSTRUCT,
                NODE_SET_TRANSLATE,
                TEXTURING_PROPERTY_CONSTRUCT,
                TEXTURING_PROPERTY_AFTER_MAP,
                NODE_ATTACH_PROPERTY,
                NO_LIGHTING_PROPERTY_CONSTRUCT,
                SHADER_PROPERTY_SET_WORD_0X58,
                NODE_ATTACH_PROPERTY,
                PREPARE_OBJECT,
                NODE_UPDATE_PROPERTIES,
                UPDATE_DATA_CONSTRUCT,
                NODE_UPDATE,
            ]
        );
        assert_eq!(
            arguments_of(&log, SHADER_PROPERTY_SET_WORD_0X58)[0][1],
            0x21
        );
        assert_eq!(
            node_calls.borrow().clone(),
            vec![(NODE_ATTACH_BORDER_SLOT, vec![node, call[0], 1])]
        );
    }

    // ---- InitDistantTextureBlending ----------------------------------------------

    /// Everything the blending code asks: four nodes with a property each
    /// (a table pair for the textures), the terrain manager's answers and
    /// the object it gives.
    struct Blending {
        properties: Vec<u32>,
        calls: Rc<RefCell<Vec<PropertyCall>>>,
        property_type: Rc<std::cell::Cell<i32>>,
        object: Rc<std::cell::Cell<u32>>,
        shorts: Rc<std::cell::Cell<(i16, i16)>>,
        positions: Rc<RefCell<Vec<[f32; 3]>>>,
    }

    fn blending_setup(e: &mut Engine) -> (Ptr<TESObjectLAND>, Blending) {
        vector_doubles(e);
        fourth_session_data(e);
        let (this, data) = terrain(e);
        e.set(data, LoadedLandData::iCellX, 2);
        e.set(data, LoadedLandData::iCellY, 3);
        let nodes = [0x100u32, 0x200, 0x300, 0x400];
        let meshes = e.mem.alloc(16);
        for (i, node) in nodes.iter().enumerate() {
            e.mem.set_u32(meshes + 4 * i as u32, *node);
        }
        e.set(data, LoadedLandData::ppMesh, Ptr::new(meshes));
        e.set_global(EXTERIOR_LOADER, 0x5000u32);
        e.set_global(DEFAULT_NORMAL_WORDS + 8, 7.0f32);
        e.set_global(BLEND_BLOCK_WORDS + 8, 0x1234u32);
        e.set_global(BLEND_BLOCK_WORDS + 12, 0x5678u32);
        e.register(TES_GET_WORLD_SPACE, |_, _| ret(0x6000));
        e.register(WORLD_SPACE_GET_TERRAIN_MANAGER, |_, _| ret(0x7000));
        e.register(NODE_FIRST_GEOMETRY, |_, a| ret(a[0] + 1));
        // A property per node, its two texture tables with slot 9 empty.
        let mut properties = vec![];
        for _ in nodes {
            let property = object_with_vtable(
                e,
                0xc0,
                &[
                    (PROPERTY_SLOT_BLEND_FIRST, 0x00a0_0011),
                    (PROPERTY_SLOT_BLEND_SECOND, 0x00a0_0012),
                ],
            );
            for table_offset in [0xacu32, 0xb0] {
                let table = e.mem.alloc(0x40);
                e.mem.set_u32(property + table_offset, table);
            }
            properties.push(property);
        }
        let by_geometry = properties.clone();
        e.register_double(OBJECT_GET_PROPERTY, move |_, a| {
            ret(by_geometry[(a[0] as usize - 1) / 0x100 - 1])
        });
        let calls = recorder();
        for (address, slot) in [(0x00a0_0011u32, 0xfcu32), (0x00a0_0012, 0x100)] {
            let calls = calls.clone();
            e.register_double(address, move |_, a| {
                calls.borrow_mut().push((slot, a[0], a[1..].to_vec()));
                Ret::default()
            });
        }
        let property_type = Rc::new(std::cell::Cell::new(10));
        let shared_type = property_type.clone();
        e.register_double(PROPERTY_GET_TYPE, move |_, _| ret(shared_type.get() as u32));
        let object = Rc::new(std::cell::Cell::new(0u32));
        let shorts = Rc::new(std::cell::Cell::new((0i16, 0i16)));
        let positions = recorder();
        let recorded = positions.clone();
        e.register_double(TERRAIN_BLEND_FIRST, move |e, a| {
            recorded.borrow_mut().push(v3(e, a[1]));
            ret(0xaaa1)
        });
        e.register(TERRAIN_BLEND_SECOND, |_, _| ret(0xaaa2));
        let shared_object = object.clone();
        e.register_double(TERRAIN_BLEND_OBJECT, move |_, _| ret(shared_object.get()));
        let shared_shorts = shorts.clone();
        e.register_double(TERRAIN_OBJECT_SHORT_FIRST, move |_, _| {
            ret(shared_shorts.get().0 as u16 as u32)
        });
        let shared_shorts = shorts.clone();
        e.register_double(TERRAIN_OBJECT_SHORT_SECOND, move |_, _| {
            ret(shared_shorts.get().1 as u16 as u32)
        });
        e.register(DATA_HANDLER_WORD_AT_4, |_, _| ret(4));
        (
            this,
            Blending {
                properties,
                calls,
                property_type,
                object,
                shorts,
                positions,
            },
        )
    }

    fn blend_block(e: &Engine, property: u32) -> Vec<u32> {
        read_words(e, property + PROPERTY_BLEND_BLOCK_OFFSET, 4)
    }

    #[test]
    fn blending_without_a_terrain_manager_or_meshes_does_nothing() {
        let mut e = engine();
        let (this, blending) = blending_setup(&mut e);
        e.register(WORLD_SPACE_GET_TERRAIN_MANAGER, |_, _| ret(0));
        e.call(0x0054_00e0, &args![this]);
        assert!(blending.calls.borrow().is_empty());
        // No world space at all.
        e.register(TES_GET_WORLD_SPACE, |_, _| ret(0));
        e.call(0x0054_00e0, &args![this]);
        assert!(blending.calls.borrow().is_empty());
        // No meshes.
        e.register(TES_GET_WORLD_SPACE, |_, _| ret(0x6000));
        e.register(WORLD_SPACE_GET_TERRAIN_MANAGER, |_, _| ret(0x7000));
        let data = loaded_data(&e, this);
        e.set(data, LoadedLandData::ppMesh, Ptr::NULL);
        e.call(0x0054_00e0, &args![this]);
        assert!(blending.calls.borrow().is_empty());
    }

    #[test]
    fn blending_skips_properties_of_other_types_and_with_both_textures() {
        let mut e = engine();
        let (this, blending) = blending_setup(&mut e);
        blending.property_type.set(7);
        e.call(0x0054_00e0, &args![this]);
        assert!(blending.calls.borrow().is_empty());
        blending.property_type.set(13);
        e.call(0x0054_00e0, &args![this]);
        assert!(blending.calls.borrow().is_empty());
        // Both tables hold a texture at index 9: skipped; one empty: done.
        blending.property_type.set(8);
        for property in &blending.properties {
            for table_offset in [0xacu32, 0xb0] {
                let table = e.mem.u32(property + table_offset);
                e.mem.set_u32(table + 36, 0x1111);
            }
        }
        e.call(0x0054_00e0, &args![this]);
        assert!(blending.calls.borrow().is_empty());
        let table = e.mem.u32(blending.properties[2] + 0xb0);
        e.mem.set_u32(table + 36, 0);
        blending.property_type.set(12);
        e.call(0x0054_00e0, &args![this]);
        // Only the third property: both slots with 9 and the answers.
        assert_eq!(
            blending.calls.borrow().clone(),
            vec![
                (0xfc, blending.properties[2], vec![9, 0xaaa1]),
                (0x100, blending.properties[2], vec![9, 0xaaa2]),
            ]
        );
        // The terrain manager was asked for the cell centre and the default
        // normal's z.
        assert_eq!(
            blending.positions.borrow().clone(),
            vec![[10240.0, 14336.0, 7.0]]
        );
        // No object at the position: the block is untouched.
        assert_eq!(blend_block(&e, blending.properties[2]), vec![0, 0, 0, 0]);
    }

    #[test]
    fn blending_stores_the_fractions_of_the_cell_in_the_terrain_object() {
        let mut e = engine();
        let (this, blending) = blending_setup(&mut e);
        let object = e.mem.alloc(8);
        let target = e.mem.alloc(8);
        e.mem.set_u32(object, target);
        blending.object.set(object);
        e.call(0x0054_00e0, &args![this]);
        // (cell - 0) + quadrant offset, over the word 4: x = 2, y = 3.
        let expected = [
            (0.5f32, 0.75f32),
            (0.625, 0.75),
            (0.5, 0.875),
            (0.625, 0.875),
        ];
        for (property, (x, y)) in blending.properties.iter().zip(expected) {
            assert_eq!(
                blend_block(&e, *property),
                vec![x.to_bits(), y.to_bits(), 0x1234, 0x5678]
            );
        }
        // Fractions outside 0 to 1 are wrapped by whole steps: (2 + 10) / 4
        // is 3, stepped down to 1; (3 - 10) / 4 is -1.75, stepped up to
        // 0.25.
        blending.shorts.set((-10, 10));
        e.call(0x0054_00e0, &args![this]);
        assert_eq!(
            blend_block(&e, blending.properties[0]),
            vec![1.0f32.to_bits(), 0.25f32.to_bits(), 0x1234, 0x5678]
        );
    }

    #[test]
    fn blending_fractions_round_at_the_control_words_precision() {
        let mut e = engine();
        let (this, blending) = blending_setup(&mut e);
        let object = e.mem.alloc(8);
        let target = e.mem.alloc(8);
        e.mem.set_u32(object, target);
        blending.object.set(object);
        // Cell x 2^24 and the word 25 * 2^20. Quadrant 1 adds 0.5: at 24-bit
        // precision 0054040f rounds 2^24 + 0.5 back to 2^24 and the quotient
        // 0.64 rounds down to the float below it; at 53-bit (2^24 + 0.5) /
        // (25 * 2^20) rounds up to the float above.
        let data = loaded_data(&e, this);
        e.set(data, LoadedLandData::iCellX, 1 << 24);
        e.register(DATA_HANDLER_WORD_AT_4, |_, _| ret(25 << 20));
        for (word, expected) in [(0x007fu16, 0x3f23_d70au32), (0x027f, 0x3f23_d70b)] {
            crate::units::crt::set_x87_control_word(&mut e, word);
            e.call(0x0054_00e0, &args![this]);
            let block = blend_block(&e, blending.properties[1]);
            assert_eq!(block[0], expected, "{word:#x}");
        }
        crate::units::crt::set_x87_control_word(&mut e, 0x027f);
    }

    #[test]
    fn texture_table_slots_are_read_through_the_smart_pointer() {
        let mut e = engine();
        let object = e.mem.alloc(0xc0);
        for (offset, value) in [(0xacu32, 0x2222u32), (0xb0, 0x3333)] {
            let table = e.mem.alloc(0x40);
            e.mem.set_u32(table + 36, value);
            e.mem.set_u32(object + offset, table);
        }
        assert_eq!(e.call(0x0054_0540, &args![object, 9u32]).u32(), 0x2222);
        assert_eq!(e.call(0x0054_0570, &args![object, 9u32]).u32(), 0x3333);
    }

    // ---- LoadedLandData and the small classes -----------------------------------

    #[test]
    fn loaded_land_data_constructor_clears_the_data() {
        let mut e = engine();
        e.set_global(DEFAULT_LAND_TEXTURE, 0x1111u32);
        for address in [
            SMART_POINTER_CONSTRUCT,
            EH_VECTOR_CONSTRUCT,
            TEXTURING_PROPERTY_SET,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(NI_POINT2_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            ret(a[0])
        });
        let this: Ptr<LoadedLandData> = e.new_object();
        e.mem.write(this.addr(), &[0xff; 0xa4]);
        start_log(&mut e);
        assert_eq!(e.call(0x0054_05a0, &args![this]).u32(), this.addr());
        let log = end_log(&mut e);
        assert_eq!(
            arguments_of(&log, SMART_POINTER_CONSTRUCT),
            vec![vec![this.addr() + 0x14, 0], vec![this.addr() + 0x94, 0]]
        );
        assert_eq!(
            arguments_of(&log, EH_VECTOR_CONSTRUCT),
            vec![vec![
                this.addr() + 0x54,
                0x10,
                4,
                GRASS_MAP_DEFAULT_CONSTRUCT,
                GRASS_MAP_DESTRUCT
            ]]
        );
        assert_eq!(
            arguments_of(&log, TEXTURING_PROPERTY_SET),
            vec![vec![this.addr() + 0x14, 0]]
        );
        for field in [0u32, 4, 8, 0xc, 0x10, 0x50] {
            assert_eq!(e.mem.u32(this.addr() + field), 0, "{field:#x}");
        }
        assert_eq!(floats(&e, this.addr() + 0x18, 2), vec![f32::MAX, -f32::MAX]);
        for quadrant in 0..4 {
            assert_eq!(e.mem.u32(this.addr() + 0x20 + 4 * quadrant), 0x1111);
        }
        assert_eq!(e.mem.bytes(this.addr() + 0x30, 0x20), vec![0; 0x20]);
        assert_eq!(e.get(this, LoadedLandData::fBaseHeight), 0.0);
        // The words the constructor leaves to its callees are untouched.
        assert_eq!(e.mem.u32(this.addr() + 0x14), 0xffff_ffff);
    }

    #[test]
    fn grass_map_default_constructor_uses_the_hash_size_0x25() {
        let mut e = engine();
        let requested = Rc::new(std::cell::Cell::new(0u32));
        let seen = requested.clone();
        e.register_double(NI_ALLOCATE, move |e, a| {
            seen.set(a[0]);
            ret(e.mem.alloc(a[0]))
        });
        let map = e.mem.alloc(0x10);
        assert_eq!(e.call(0x0054_0700, &args![map]).u32(), map);
        assert_eq!(requested.get(), 0x94);
        assert_eq!(e.mem.u32(map), GRASS_MAP_VTABLE);
        assert_eq!(e.mem.u32(map + 4), 0x25);
        assert_eq!(e.mem.u32(map + 0xc), 0);
        let table = e.mem.u32(map + 8);
        assert_eq!(e.mem.bytes(table, 0x94), vec![0; 0x94]);
    }

    #[test]
    fn map_base_constructor_zeroes_a_bucket_table() {
        let mut e = engine();
        e.register(NI_ALLOCATE, |e, a| {
            let block = e.mem.alloc(a[0]);
            e.mem.write(block, &vec![0xee; a[0] as usize]);
            ret(block)
        });
        let map = e.mem.alloc(0x10);
        e.mem.set_u32(map + 0xc, 99);
        assert_eq!(e.call(0x0054_0940, &args![map, 7u32]).u32(), map);
        assert_eq!(e.mem.u32(map), MAP_BASE_VTABLE);
        assert_eq!(e.mem.u32(map + 4), 7);
        assert_eq!(e.mem.u32(map + 0xc), 0);
        let table = e.mem.u32(map + 8);
        assert_eq!(e.mem.bytes(table, 28), vec![0; 28]);
    }

    #[test]
    fn vertex_lock_constructor_clears_three_fields() {
        let mut e = engine();
        let lock = e.mem.alloc(12);
        e.mem.write(lock, &[0xff; 12]);
        assert_eq!(e.call(0x0054_0720, &args![lock]).u32(), lock);
        assert_eq!(e.mem.u32(lock), 0);
        assert_eq!(e.mem.u32(lock + 4), 0);
        assert_eq!(e.mem.u8(lock + 8), 0);
        assert_eq!(e.mem.u8(lock + 9), 0xff);
    }

    #[test]
    fn byte_setter_stores_the_low_byte() {
        let mut e = engine();
        let at = e.mem.alloc(4);
        e.mem.write(at, &[0xff; 4]);
        e.call(0x0054_07b0, &args![at, 0x1234_5678u32]);
        assert_eq!(e.mem.bytes(at, 4), vec![0x78, 0xff, 0xff, 0xff]);
    }

    #[test]
    fn grass_map_scalar_deleting_destructor_frees_when_asked() {
        let mut e = engine();
        e.register(MAP_REMOVE_ALL, |_, _| Ret::default());
        e.register(RELEASE_BLOCK, |_, _| Ret::default());
        e.register(DEALLOCATE, |_, _| Ret::default());
        let table = e.mem.alloc(8);
        let map = e.mem.alloc(0x10);
        e.mem.set_u32(map + 8, table);
        // The map class (0x005407d0): its destructor body, which runs the
        // base's, then the free.
        start_log(&mut e);
        assert_eq!(e.call(0x0054_07d0, &args![map, 0u32]).u32(), map);
        let log = end_log(&mut e);
        assert_eq!(
            log,
            vec![
                (MAP_REMOVE_ALL, vec![map]),
                (MAP_REMOVE_ALL, vec![map]),
                (RELEASE_BLOCK, vec![table])
            ]
        );
        assert_eq!(e.mem.u32(map), MAP_BASE_VTABLE);
        start_log(&mut e);
        e.call(0x0054_07d0, &args![map, 1u32]);
        assert_eq!(
            called(&mut e),
            vec![MAP_REMOVE_ALL, MAP_REMOVE_ALL, RELEASE_BLOCK, DEALLOCATE]
        );
    }

    #[test]
    fn map_base_scalar_deleting_destructor_runs_only_its_own_body() {
        let mut e = engine();
        e.register(MAP_REMOVE_ALL, |_, _| Ret::default());
        e.register(RELEASE_BLOCK, |_, _| Ret::default());
        e.register(DEALLOCATE, |_, _| Ret::default());
        let map = e.mem.alloc(0x10);
        e.mem.set_u32(map + 8, 0x4444);
        start_log(&mut e);
        assert_eq!(e.call(0x0054_0aa0, &args![map, 1u32]).u32(), map);
        assert_eq!(
            end_log(&mut e),
            vec![
                (MAP_REMOVE_ALL, vec![map]),
                (RELEASE_BLOCK, vec![0x4444]),
                (DEALLOCATE, vec![map])
            ]
        );
        assert_eq!(e.mem.u32(map), MAP_BASE_VTABLE);
        start_log(&mut e);
        e.call(0x0054_0aa0, &args![map, 0u32]);
        assert_eq!(called(&mut e), vec![MAP_REMOVE_ALL, RELEASE_BLOCK]);
    }

    #[test]
    fn grass_map_destructor_body_stores_its_vtable_first() {
        let mut e = engine();
        let vtables = recorder();
        let seen = vtables.clone();
        e.register_double(MAP_REMOVE_ALL, move |e, a| {
            seen.borrow_mut().push(e.mem.u32(a[0]));
            Ret::default()
        });
        e.register(RELEASE_BLOCK, |_, _| Ret::default());
        let map = e.mem.alloc(0x10);
        e.call(0x0054_09b0, &args![map]);
        // The map removes its entries under its own vtable, then the base
        // body does under the base vtable.
        assert_eq!(
            vtables.borrow().clone(),
            vec![GRASS_MAP_VTABLE, MAP_BASE_VTABLE]
        );
    }

    #[test]
    fn block_array_destructor_releases_its_table() {
        let mut e = engine();
        e.register(RELEASE_TABLE, |_, _| Ret::default());
        e.register(DEALLOCATE, |_, _| Ret::default());
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array + 4, 0x6666);
        start_log(&mut e);
        e.call(0x0054_0800, &args![array]);
        assert_eq!(end_log(&mut e), vec![(RELEASE_TABLE, vec![0x6666])]);
        assert_eq!(e.mem.u32(array), BLOCK_ARRAY_VTABLE);
        // Its deleting destructor frees the block when bit 0 is set.
        start_log(&mut e);
        assert_eq!(e.call(0x0054_0a40, &args![array, 1u32]).u32(), array);
        assert_eq!(called(&mut e), vec![RELEASE_TABLE, DEALLOCATE]);
        start_log(&mut e);
        e.call(0x0054_0a40, &args![array, 0u32]);
        assert_eq!(called(&mut e), vec![RELEASE_TABLE]);
        // The primitive array's goes through `00537ce0` (the same body).
        start_log(&mut e);
        e.call(0x0054_0a70, &args![array, 3u32]);
        assert_eq!(
            called(&mut e),
            vec![MEMBER_DESTRUCT, RELEASE_TABLE, DEALLOCATE]
        );
    }

    #[test]
    fn block_array_constructor_allocates_its_table() {
        let mut e = engine();
        e.register(BLOCK_ARRAY_ALLOCATE, |_, a| ret(0x5000 + a[0]));
        let array = e.mem.alloc(0x10);
        e.mem.write(array, &[0xff; 0x10]);
        assert_eq!(e.call(0x0054_0b00, &args![array, 6u32, 2u32]).u32(), array);
        assert_eq!(e.mem.u32(array), BLOCK_ARRAY_VTABLE);
        assert_eq!(e.mem.u16(array + 8), 6);
        assert_eq!(e.mem.u16(array + 0xe), 2);
        assert_eq!(e.mem.u16(array + 0xa), 0);
        assert_eq!(e.mem.u16(array + 0xc), 0);
        assert_eq!(e.mem.u32(array + 4), 0x5006);
        // A maximum size of 0 has no table.
        start_log(&mut e);
        e.call(0x0054_0b00, &args![array, 0u32, 2u32]);
        assert_eq!(called(&mut e), vec![]);
        assert_eq!(e.mem.u32(array + 4), 0);
        // The primitive array's constructor then sets its own vtable.
        e.call(0x0054_0830, &args![array, 3u32, 1u32]);
        assert_eq!(e.mem.u32(array), BLOCK_PRIMITIVE_ARRAY_VTABLE);
        assert_eq!(e.mem.u16(array + 8), 3);
    }

    // ---- The Havok byte array ----------------------------------------------------

    #[test]
    fn byte_array_reserves_the_larger_of_twice_the_capacity_and_the_size() {
        let mut e = engine();
        e.register(HK_ARRAY_CAPACITY, |_, _| ret(4));
        e.register(HK_ARRAY_RESERVE, |_, _| Ret::default());
        let array = e.mem.alloc(12);
        for (size, reserved) in [(3i32, None), (4, None), (5, Some(8u32)), (20, Some(20))] {
            start_log(&mut e);
            e.call(0x0054_0bc0, &args![array, 0xa110u32, size as u32]);
            let log = end_log(&mut e);
            let reserves = arguments_of(&log, HK_ARRAY_RESERVE);
            match reserved {
                None => assert!(reserves.is_empty(), "{size}"),
                Some(capacity) => {
                    assert_eq!(reserves, vec![vec![0xa110, array, capacity, 1]], "{size}")
                }
            }
        }
    }

    #[test]
    fn byte_array_resize_runs_the_range_functions() {
        let mut e = engine();
        e.register(HK_ARRAY_CAPACITY, |_, _| ret(100));
        e.register(BYTE_RANGE_NOTHING, |_, _| Ret::default());
        let array = e.mem.alloc(12);
        e.mem.set_u32(array, 0x1000);
        e.mem.set_i32(array + 4, 10);
        start_log(&mut e);
        e.call(0x0054_0860, &args![array, 0xa110u32, 3u32]);
        let log = end_log(&mut e);
        assert_eq!(
            arguments_of(&log, BYTE_RANGE_NOTHING),
            vec![vec![0x1003, 7, 0], vec![0x100a, (-7i32) as u32, 0]]
        );
        assert_eq!(e.mem.i32(array + 4), 3);
    }

    #[test]
    fn byte_array_functions_ask_the_router_for_the_allocator() {
        let mut e = engine();
        e.register(HK_ARRAY_ALLOCATOR, |_, _| ret(0xa110));
        e.register(HK_ARRAY_CAPACITY, |_, _| ret(100));
        e.register(BYTE_RANGE_NOTHING, |_, _| Ret::default());
        e.register(HK_ARRAY_CLEAR, |_, _| Ret::default());
        e.register(FORM_BASE_DESTRUCT, |_, _| Ret::default());
        let array = e.mem.alloc(12);
        e.mem.set_u32(array, 0x1000);
        // Sizing: the allocator, then `00540860`.
        start_log(&mut e);
        e.call(0x0054_0750, &args![array, 5u32]);
        let log = end_log(&mut e);
        assert_eq!(log[0].0, HK_ARRAY_ALLOCATOR);
        assert_eq!(log[0].1[1], array);
        assert_eq!(log[1].0, HK_ARRAY_CAPACITY);
        assert_eq!(e.mem.i32(array + 4), 5);
        // Release: the allocator, then `00540b70(array, allocator)`.
        start_log(&mut e);
        e.call(0x0054_0ad0, &args![array]);
        let log = end_log(&mut e);
        assert_eq!(log[0].0, HK_ARRAY_ALLOCATOR);
        assert_eq!(log[1], (HK_ARRAY_CLEAR, vec![array, 0xa110]));
        // The destructor does that and then the empty base destructor.
        start_log(&mut e);
        e.call(0x0054_08e0, &args![array]);
        assert_eq!(
            called(&mut e),
            vec![HK_ARRAY_ALLOCATOR, HK_ARRAY_CLEAR, FORM_BASE_DESTRUCT]
        );
    }

    #[test]
    fn grass_map_constructor_takes_the_hash_size() {
        let mut e = engine();
        e.register(NI_ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        let map = e.mem.alloc(0x10);
        assert_eq!(e.call(0x0054_0780, &args![map, 5u32]).u32(), map);
        assert_eq!(e.mem.u32(map), GRASS_MAP_VTABLE);
        assert_eq!(e.mem.u32(map + 4), 5);
    }

    #[test]
    fn map_base_destructor_body_removes_entries_and_frees_the_table() {
        let mut e = engine();
        e.register(MAP_REMOVE_ALL, |_, _| Ret::default());
        e.register(RELEASE_BLOCK, |_, _| Ret::default());
        let map = e.mem.alloc(0x10);
        e.mem.set_u32(map + 8, 0x4444);
        start_log(&mut e);
        e.call(0x0054_0a10, &args![map]);
        assert_eq!(
            end_log(&mut e),
            vec![(MAP_REMOVE_ALL, vec![map]), (RELEASE_BLOCK, vec![0x4444])]
        );
        assert_eq!(e.mem.u32(map), MAP_BASE_VTABLE);
    }
}
