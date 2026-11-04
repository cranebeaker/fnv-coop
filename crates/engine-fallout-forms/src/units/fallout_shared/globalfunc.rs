//! `fallout shared/globalfunc.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The unit is a grab bag of free functions from `004b1230` on (122
//! functions). Session 1 (b0015) covers the first 40, `004b1230` to
//! `004b51e0`: the number tests for strings, the angle helpers, the two
//! lookup functions for form type bytes, a segment against rectangle
//! clipper, the pick helpers, the debug shape builders (`MakeTriPoint`,
//! `MakeQuadBox`, `MakeTriangle`, `MakeRectangle` and a one segment line
//! builder), the small vector and matrix products, `NiMatrix3`'s inverse,
//! the geometry snapping function `004b3e60` with its helper `004b4910`,
//! and the `bhkMouseSpringAction` constructor chain. Session 2 covers the
//! next 40, `004b5210` to `004b6810`: the rest of `bhkMouseSpringAction`,
//! the collision object lookups, the collidable accessors, `NiASin`, the
//! scene graph walks (scabbard and editor marker removal, morpher test,
//! tinting, collision object counts) and the bound against frustum test.
//! Session 3 covers the next 40, `004b68a0` to `004b9d60`: the other scene
//! graph walkers (`004b6980` to `004b6c70`, with their callbacks), the
//! no lighting property recursions, the platform and language texture swap
//! (`004b71a0` to `004b7660`), `FixedStrings::InitSDM` with its shutdown
//! counterpart, and the methods of `NiTMap<unsigned int, VertexDist>`
//! (constructor to `NewItem`). Session 4 covers the last three, `004b9d80` to `004b9de0`: the unit is
//! complete.
//!
//! Conventions this file uses, so the next session finds them:
//!
//! - x87: the game computes in extended precision and stores `float`
//!   results. The translations compute in `f64` and round to `f32` where
//!   the code stores a `float` (docs/ENGINE_CRATE.md). An x87 compare
//!   followed by `TEST AH, mask` and a conditional jump is written as the
//!   ordinary comparison it implements, including what it does for NaN
//!   (`!(a > b)` is not `a <= b`), hence the `neg_cmp_op_on_partial_ord`
//!   allowance below.
//! - A `float` or `double` the code loads from the exe's data (a constant
//!   the decompiler folds into a literal) is read from memory at its
//!   address (the `const`s below, with the value they hold in the exe).
//! - `00416870` is `NiPoint3::NiPoint3(this, x, y, z)` (returns `this`),
//!   `006815c0` returns `this` (the empty `NiPoint3` constructor the array
//!   constructors call), `00439ef0` is `this - other` and `00439e90`
//!   `this + other` on `NiPoint3` (`(this, out, other)`, they return
//!   `out`), `00457990` the length of a vector (ST0), `004a0c10` the
//!   unitize. They live in other units and are called by address.
//! - The game sometimes pushes words its callee never reads; they stay on
//!   the stack until a later call reads them (`005495f0`, `0049ec60`,
//!   `006815c0`). The translations pass each call only its own words, and
//!   the doc of the function says where the next call picks them up.
//! - The compiler's exception-unwinding frames and security cookies are
//!   not translated.

#![allow(clippy::neg_cmp_op_on_partial_ord)]

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{NiAVObject, NiPoint3, NiTMap};

// ---------------------------------------------------------------------
// Constants of the exe's data read by the code (value in the exe).
// ---------------------------------------------------------------------

/// `1.0` (`double`).
const ONE: u32 = 0x0101_2070;
/// `-1.0` (`float`).
const NEGATIVE_ONE: u32 = 0x0101_2054;
/// `FLT_MAX` (`float`), `0x7f7fffff`.
const FLOAT_MAX: u32 = 0x0101_6970;
/// `4.712389` (`float`), three quarters of a turn.
const THREE_QUARTER_TURN: u32 = 0x0101_ff34;
/// `1.5707964` (`float`), a quarter turn.
const QUARTER_TURN: u32 = 0x0101_ff38;
/// `3.1415927410125732` (`double`), half a turn.
const HALF_TURN: u32 = 0x0101_ff40;
/// `6.2831854820251465` (`double`).
const FULL_TURN: u32 = 0x0101_ff48;
/// `6.2831855` (`float`).
const FULL_TURN_FLOAT: u32 = 0x0101_ff50;
/// `-3.1415927410125732` (`double`).
const NEGATIVE_HALF_TURN: u32 = 0x0101_ff58;
/// `1.4142135381698608` (`double`), the square root of two.
const SQUARE_ROOT_OF_TWO: u32 = 0x0101_ff98;
/// `9.99999974752427e-7` (`double`): a determinant at most this large in
/// magnitude makes `Inverse_ov2` fail.
const SINGULAR_DETERMINANT: u32 = 0x0101_7cf8;
/// `0.001` (`float`): the tolerance within which the snapping function
/// treats two distances as equal.
const SNAP_TOLERANCE: u32 = 0x0101_7d00;
/// `10.0` (`float`): how far the snapping function's debug mode moves a
/// point along its vector.
const DEBUG_OFFSET: u32 = 0x0101_7b78;
/// The geometry the snapping function last ran on (`011c6320`).
const LAST_SNAPPED_OBJECT: u32 = 0x011c_6320;

/// The CRT's one-argument `double` function that `004b1460` calls
/// (`00ec9010`); the angle formulas that use it show it is the arc
/// tangent (its body was not read).
const CRT_ARC_TANGENT: u32 = 0x00ec_9010;
/// The CRT `fmod(double, double)` (`00ec9130`).
const CRT_FMOD: u32 = 0x00ec_9130;

// ---------------------------------------------------------------------
// Callees in other units.
// ---------------------------------------------------------------------

/// `operator new(size)` and `operator delete(block)` (cdecl).
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `__vec_ctor(block, element size, count, constructor)`: calls
/// `constructor(element)` on each element, advancing by the size.
const VECTOR_CONSTRUCT: u32 = 0x0040_1050;
/// The Gamebryo allocators (cdecl, size): objects and arrays.
const NI_OPERATOR_NEW: u32 = 0x00aa_13e0;
const NI_ARRAY_NEW: u32 = 0x00aa_1070;
/// `NiPoint3::NiPoint3(this, x, y, z)`; returns `this`.
const POINT3_CONSTRUCT: u32 = 0x0041_6870;
/// The empty `NiPoint3` constructor (returns `this`) and the 16-byte
/// element constructor (`004a7800`) the array constructors are given.
const POINT3_EMPTY_CONSTRUCTOR: u32 = 0x0068_15c0;
const COLOR_ELEMENT_CONSTRUCTOR: u32 = 0x004a_7800;
/// `NiTriShape::NiTriShape(this, ...)` (Xbox PDB), as the callers here pass
/// it: `(vertex count, vertices, 0, colors, 0, 0, 0, triangle count,
/// triangles)`.
const NI_TRI_SHAPE_CONSTRUCT: u32 = 0x00a7_4410;
/// `NiLines::NiLines(this, count, vertices, colors, 0, 0, 0, flags)`
/// (Xbox PDB).
const NI_LINES_CONSTRUCT: u32 = 0x00a7_46e0;
/// `BSShaderNoLightingProperty::BSShaderNoLightingProperty(this)`
/// (Xbox PDB).
const NO_LIGHTING_PROPERTY_CONSTRUCT: u32 = 0x00b6_fc90;
/// `NiAVObject::AttachProperty(this, property)` (Xbox PDB).
const ATTACH_PROPERTY: u32 = 0x0043_9410;
/// `BSShaderManager::PrepareObject(object, 0, 0)` (cdecl, Xbox PDB).
const PREPARE_OBJECT: u32 = 0x00b5_7e30;
/// `NiAVObject::GetWorldBound` (Xbox PDB): a pointer to the four floats of
/// the bound (centre, then radius).
const GET_WORLD_BOUND: u32 = 0x0043_d450;
/// `float` at `this + 0xc` (ST0): the radius of a bound.
const BOUND_RADIUS: u32 = 0x0084_d030;
/// Pick helpers: `00824060(this)` and `00456610(this)` (bool results),
/// `00705fc0(this, node)` (selects what to pick against) and
/// `NiPick::PickObjects(this, origin, direction, find first)` (Xbox PDB;
/// bool).
const PICK_BLOCKED_A: u32 = 0x0082_4060;
const PICK_BLOCKED_B: u32 = 0x0045_6610;
const PICK_SET_ROOT: u32 = 0x0070_5fc0;
const PICK_OBJECTS: u32 = 0x00e9_8e20;
/// The pick state object that `004b2780` clears (`011f426c`).
const PICK_STATE: u32 = 0x011f_426c;
/// Child container accessors: the count (`0043b480(this)`) and child `i`
/// (`0043b4a0(this, i)`).
const CHILD_COUNT: u32 = 0x0043_b480;
const CHILD_AT: u32 = 0x0043_b4a0;
/// Vector helpers (`NiPoint3`): `this - other` and `this + other` as
/// `(this, out, other)`, the length (ST0), the unitize, `out = vector *
/// scale` as `(out, scale, vector)` (cdecl, returns `out`) and
/// `this += other`.
const POINT3_SUBTRACT: u32 = 0x0043_9ef0;
const POINT3_ADD: u32 = 0x0043_9e90;
const POINT3_LENGTH: u32 = 0x0045_7990;
const POINT3_UNITIZE: u32 = 0x004a_0c10;
const POINT3_SCALED: u32 = 0x004a_3760;
const POINT3_ADD_ASSIGN: u32 = 0x0063_c8a0;
/// `00408820(float)` (ST0): the absolute value of a float.
const FLOAT_ABSOLUTE_VALUE: u32 = 0x0040_8820;
/// `0040eb10(a, b, tolerance)` (cdecl): whether two floats differ by at
/// most the tolerance.
const FLOATS_NEAR: u32 = 0x0040_eb10;
/// The log function `005b5e40(format, ...)`.
const LOG_MESSAGE: u32 = 0x005b_5e40;
/// `"FORMS: Invalid FormID sent to FormIDToString().\r\n"`.
const INVALID_FORM_ID_MESSAGE: u32 = 0x0101_ff60;

// Callees of `004b3e60` and `004b4910`.
/// `(this)`: the first point array of a geometry object (12-byte
/// elements); `004a8030(this)` the second one; `00456650(this)` the point
/// count (a word); `00461130(this)` the address of the world transform
/// (`this + 0x68`).
const POINTS_FIRST: u32 = 0x0049_ec60;
const POINTS_SECOND: u32 = 0x004a_8030;
const POINT_COUNT: u32 = 0x0045_6650;
const WORLD_TRANSFORM: u32 = 0x0046_1130;
/// `004b4a10(count, source points, destination, transform)` (cdecl): a
/// thunk to the function pointer at `011aa050` that transforms `count`
/// points.
const TRANSFORM_POINTS: u32 = 0x004b_4a10;
/// The `NiTransform` constructor (`00476a80(this)`),
/// `NiTransform::InvertNonUniform` (`004b4880(this, out)`, Xbox PDB name)
/// and `NiMatrix3::operator*` (`0043f8d0(this, out, other)`, engine map).
const TRANSFORM_CONSTRUCT: u32 = 0x0047_6a80;
const TRANSFORM_INVERT_NON_UNIFORM: u32 = 0x004b_4880;
const MATRIX_PRODUCT: u32 = 0x0043_f8d0;
/// The map the snapping function builds (a hash map of 0x25 buckets keyed
/// by a point index, 12-byte values): constructor `(this, 0x25)`,
/// `Find(this, key, out value)` (bool), `SetAt(this, key, value words 0,
/// 1, 2)`, the iterator start `(this)` and next `(this, &iterator, &key,
/// &value)`, and the destructor `(this)`.
const POINT_MAP_CONSTRUCT: u32 = 0x004b_9950;
const POINT_MAP_SET_AT: u32 = 0x004b_9a20;
const POINT_MAP_FIND: u32 = 0x004b_9b20;
const POINT_MAP_FIRST: u32 = 0x004b_9ba0;
const POINT_MAP_NEXT: u32 = 0x004b_9bf0;
const POINT_MAP_DESTRUCT: u32 = 0x004b_9cd0;
/// `005495f0(this)` is `NiPointer` dereference at `+0xb8`; the call site
/// pushes the flag word `3` that the next call reads:
/// `00a67090(that object, 3)` is `NiGeometryData::MarkAsChanged` (Xbox
/// PDB).
const GEOMETRY_DATA: u32 = 0x0054_95f0;
const MARK_AS_CHANGED: u32 = 0x00a6_7090;

// Callees of the `bhk` constructors and destructors.
/// `bhkRefObject`'s constructor `(this)`, the destructors `(this)` of
/// `bhkSerializable` (`00c877b0`), `bhkAction` (`00c87970`) and
/// `bhkUnaryAction` (`00c86720`), `bhkMouseSpringAction::Init(this, arg)`
/// (`00c87260`) and `operator delete(block, size)` (`00aa1460`, cdecl).
const BHK_REF_OBJECT_CONSTRUCT: u32 = 0x00c8_7c30;
const BHK_SERIALIZABLE_DESTRUCT: u32 = 0x00c8_77b0;
const BHK_ACTION_DESTRUCT: u32 = 0x00c8_7970;
const BHK_UNARY_ACTION_DESTRUCT: u32 = 0x00c8_6720;
const BHK_MOUSE_SPRING_ACTION_INIT: u32 = 0x00c8_7260;
const SIZED_DELETE: u32 = 0x00aa_1460;
/// The vtables the constructors install and the `ms_uiObjects` counters
/// (Xbox PDB) they increment.
const BHK_SERIALIZABLE_VTABLE: u32 = 0x0102_023c;
const BHK_ACTION_VTABLE: u32 = 0x0102_016c;
const BHK_UNARY_ACTION_VTABLE: u32 = 0x0102_009c;
const BHK_MOUSE_SPRING_ACTION_VTABLE: u32 = 0x0101_ffcc;
const BHK_ACTION_OBJECT_COUNT: u32 = 0x0126_8154;
const BHK_UNARY_ACTION_OBJECT_COUNT: u32 = 0x0126_8118;
const BHK_MOUSE_SPRING_ACTION_OBJECT_COUNT: u32 = 0x0126_8124;
/// The `NiRTTI` objects the `GetRTTI` functions return.
const BHK_MOUSE_SPRING_ACTION_RTTI: u32 = 0x0126_812c;
const BHK_SERIALIZABLE_RTTI: u32 = 0x0126_8134;
const BHK_ACTION_RTTI: u32 = 0x0126_8158;
const BHK_UNARY_ACTION_RTTI: u32 = 0x0126_811c;

// Tables of the form type functions.
/// The table `004b1b60` reads when `004b1680` gives an index: name
/// pointers, 4 bytes each.
const FORM_NAME_BY_INDEX: u32 = 0x0118_a2d8;
/// The table of 12-byte entries whose first word is a name pointer,
/// indexed by form type byte (0 to 0x78).
const FORM_NAME_BY_TYPE: u32 = 0x0118_7004;

// Callees and data of the third session (`004b68a0` to `004b9d60`).
/// The walker callbacks of this file's other scene graph walks.
const SELECT_CALLBACK: u32 = 0x004b_6a00;
const HAVOK_COUNT_CALLBACK: u32 = 0x004b_6b20;
const HAVOK_FIND_CALLBACK: u32 = 0x004b_6bc0;
const HAVOK_INDEX_CALLBACK: u32 = 0x004b_6c70;
/// `006fa820(this)` reads the smart pointer at `this + 0x10`;
/// `00620b80(this)` is `0044ddc0(this)`, the word at `this + 8`. The
/// walker callbacks that count objects with something attached use them.
const POINTER_AT_0X10: u32 = 0x006f_a820;
const WORD_AT_EIGHT_WRAPPER: u32 = 0x0062_0b80;
/// `004a2c20(object)` (cdecl, bool, 0 for a null object): whether the word
/// at `object + 0x1c` (`00441110(this)`) is between 1 and 0xc.
const IN_KIND_RANGE: u32 = 0x004a_2c20;
const WORD_AT_0X1C: u32 = 0x0044_1110;
/// `004534f0(this, object)`: a function that does nothing, called on the
/// object at `011e0e80`.
const NOTIFIED_OBJECT: u32 = 0x011e_0e80;
const NOTIFY: u32 = 0x0045_34f0;
/// `0057bd60(this, reference id)`.
const SET_REFERENCE_ID: u32 = 0x0057_bd60;
/// `BSShaderProperty::SetFlag(this, flag, value)` (Xbox PDB) and
/// `NiAVObject::UpdateProperties(this)` (Xbox PDB).
const SET_SHADER_FLAG: u32 = 0x0044_1130;
const UPDATE_PROPERTIES: u32 = 0x00a5_a040;
/// `0043b230(this)` reads the smart pointer at `this + 0xc`, `004a8a90(this)`
/// the one at `this + 0x30`, and `00621b20(this, value)` stores a `float` at
/// `this + 0x10`; `011f4428` is the class key `004b70f0` filters its
/// children with.
const FIRST_ITEM: u32 = 0x0043_b230;
const NEXT_ITEM: u32 = 0x004a_8a90;
const SET_FLOAT_AT_0X10: u32 = 0x0062_1b20;
const IS_OF_CLASS_KEY_011F4428: u32 = 0x011f_4428;
/// `011f444c`: the class key of the texture objects the language swap looks
/// for.
const IS_OF_CLASS_KEY_011F444C: u32 = 0x011f_444c;
/// The language swap: the byte at `011d8a84` (whether the platform
/// choice applies), the word at `011d8a80` (read by `004b7210`), the string
/// object at `011d8c4c` that `004b9930` tests, `GetCurrentTexLanguage`'s
/// worker `00a22cb0()`, the two tables of name pointers (platforms, 4;
/// languages, 5), `strlen` through the game's wrapper (`0044a670`), the CRT
/// `_strlen` (`00ec6130`), `strcat_s(destination, size, source)` through
/// the game's wrapper (`00406d50`), `memcpy_s` (`00ec7c66`) and `tolower`
/// (`00ec67aa`).
const PLATFORM_SWAP_ENABLED: u32 = 0x011d_8a84;
const PLATFORM_SWAP_WORD: u32 = 0x011d_8a80;
const PLATFORM_SWAP_STRING: u32 = 0x011d_8c4c;
const CURRENT_LANGUAGE_WORKER: u32 = 0x00a2_2cb0;
const PLATFORM_NAMES: u32 = 0x0118_9038;
const LANGUAGE_NAMES: u32 = 0x0118_8f84;
const STRING_LENGTH: u32 = 0x0044_a670;
const CRT_STRLEN: u32 = 0x00ec_6130;
const STRING_CONCAT: u32 = 0x0040_6d50;
const CRT_MEMCPY_S: u32 = 0x00ec_7c66;
const CRT_TOLOWER: u32 = 0x00ec_67aa;
/// `NiFixedString` helpers: `0048af40(this, other)` copies a handle
/// (`this` is a 4-byte `NiFixedString`), `004381b0(this)` is its
/// destructor and `00438170(this, text)` its constructor from a C string
/// (Xbox PDB name unknown). `TES::CreateTextureImage(this, path, out
/// smart pointer, flag, flag)` (Xbox PDB, `004568c0`) is called on the TES
/// singleton read from `011dea10` (the same global [`CELL_FINDER`] names).
const FIXED_STRING_COPY: u32 = 0x0048_af40;
const FIXED_STRING_DESTRUCT: u32 = 0x0043_81b0;
const FIXED_STRING_CONSTRUCT: u32 = 0x0043_8170;
const CREATE_TEXTURE_IMAGE: u32 = 0x0045_68c0;
const TES_SINGLETON: u32 = 0x011d_ea10;
/// `NiTMap<unsigned int, VertexDist>`: the vtables of the map (`010205b4`)
/// and of its base `NiTMapBase` (`010205d4`), the base's `RemoveAll`
/// (`00438af0`), the array free `00aa10f0(block)` (cdecl), the allocator
/// call `004b9de0(this + 0xc)` that `NewItem` makes, and `memset`
/// (`00403d30(block, value, count)`).
const VERTEX_DIST_MAP_VTABLE: u32 = 0x0102_05b4;
const MAP_BASE_VTABLE: u32 = 0x0102_05d4;
const MAP_REMOVE_ALL: u32 = 0x0043_8af0;
const NI_ARRAY_DELETE: u32 = 0x00aa_10f0;
const ALLOCATOR_EMPTY_CONSTRUCTOR: u32 = 0x0065_de30;
/// `006b8310(this, block)`: `operator delete(block)`.
const ALLOCATOR_FREE: u32 = 0x006b_8310;
const MEMORY_SET: u32 = 0x0040_3d30;

/// `FixedStrings` (Xbox PDB) members, in the order `InitSDM` makes them: the
/// global that holds the `NiFixedString *` and the address of the C string
/// it is made from. The comments are the Xbox PDB member names.
const FIXED_STRINGS: [(u32, u32); 54] = [
    (0x011c_61a4, 0x0101_e460), // pBip01
    (0x011c_61a8, 0x0102_05ac), // pBip
    (0x011c_61ac, 0x0101_f63c), // pBip01Head
    (0x011c_61b0, 0x0102_05a0), // pBip01Neck
    (0x011c_61b4, 0x0102_0594), // pBip01Spine
    (0x011c_61b8, 0x0102_0584), // pBip01Spine1
    (0x011c_61bc, 0x0102_0578), // pMagicNode
    (0x011c_61c0, 0x0102_0564), // pSpecialIdle_Cast
    (0x011c_61c4, 0x0102_054c), // pSpecialIdle_AreaEffect
    (0x011c_61c8, 0x0102_0540), // pAttachSound
    (0x011c_61cc, 0x0102_0530), // pSoundMarker
    (0x011c_61d0, 0x0102_051c), // pSkinnedDecalNode
    (0x011c_61d4, 0x0101_e484), // pDecalNode
    (0x011c_61d8, 0x0102_050c), // pModelSwapNode
    (0x011c_61dc, 0x0101_1f30), // pOpenString
    (0x011c_61e0, 0x0101_abac), // pCloseString
    (0x011c_61e4, 0x0102_0504), // pDVPG
    (0x011c_61e8, 0x0102_0500), // pPrn
    (0x011c_61ec, 0x0101_3be8), // pWeapon
    (0x011c_61f0, 0x0102_030c), // pEditorMarker
    (0x011c_61f4, 0x0102_04f0), // pEditorMarker0
    (0x011c_61f8, 0x0102_04e0), // pEditorMarker1
    (0x011c_61fc, 0x0102_04d0), // pEditorMarker2
    (0x011c_6200, 0x0102_04c4), // pArrowQuiver
    (0x011c_6204, 0x0102_04b4), // pMarkerSource
    (0x011c_6208, 0x0102_04a4), // pMarkerTarget
    (0x011c_620c, 0x0102_0498), // pAttachLight
    (0x011c_6210, 0x0102_0490), // pSkin
    (0x011c_6214, 0x0102_0484), // pFaceGenEars
    (0x011c_6218, 0x0102_047c), // pUnequip
    (0x011c_621c, 0x0102_0470), // pLaserSightsName
    (0x011c_6220, 0x0102_0464), // pAimSightsName
    (0x011c_6224, 0x0102_045c), // pGrass
    (0x011c_6228, 0x0101_e47c), // pDecal
    (0x011c_622c, 0x0102_044c), // pPermanentDecal
    (0x011c_6230, 0x0102_0440), // pGrabLeft
    (0x011c_6234, 0x0102_0434), // pGrabRight
    (0x011c_6238, 0x0102_042c), // pArrow0
    (0x011c_623c, 0x0102_0420), // pArrowBone
    (0x011c_6240, 0x0102_0408), // pRaceHeadBiped
    (0x011c_6244, 0x0102_03f0), // pRaceHeadSkinned
    (0x011c_6248, 0x0102_03e4), // pHeadAnims
    (0x011c_624c, 0x0102_03d8), // pEntryPoint
    (0x011c_6250, 0x0102_03c4), // pBip01LUpperArm
    (0x011c_6254, 0x0102_03b4), // pBip01LForearm
    (0x011c_6258, 0x0102_03a0), // pBip01RUpperArm
    (0x011c_625c, 0x0102_0394), // pBip01Yaw
    (0x011c_6260, 0x0102_0388), // pBip01Pitch
    (0x011c_6264, 0x0102_0378), // pBip01Looking
    (0x011c_6268, 0x0102_0370), // pTalking
    (0x011c_626c, 0x0102_0364), // pCamera1st
    (0x011c_6270, 0x0102_0358), // pCamera3rd
    (0x011c_6274, 0x0102_0330), // pHeadMeshExp
    (0x011c_6278, 0x0102_0324), // pPinnedLimb
];
/// `pBip01Spine` and `pBip01Spine1`: the two `FixedStrings` the shutdown
/// function `004b8cf0` does not free.
const FIXED_STRING_SPINE: u32 = 0x011c_61b4;
const FIXED_STRING_SPINE1: u32 = 0x011c_61b8;

layout! {
    /// `NiTMapItem<unsigned int, VertexDist>` (Xbox PDB), 0x14 bytes. The
    /// `VertexDist` value (12 bytes) is kept as its three words, the
    /// fields of the struct being unknown.
    pub struct VertexDistMapItem: 0x14 {
        /// `m_pkNext` (Xbox PDB).
        0x00 m_pkNext: u32,
        /// `m_key` (Xbox PDB).
        0x04 m_key: u32,
        /// First word of `m_val` (Xbox PDB).
        0x08 m_val_word_0: u32,
        /// Second word of `m_val`.
        0x0C m_val_word_1: u32,
        /// Third word of `m_val`.
        0x10 m_val_word_2: u32,
    }
}

layout! {
    /// `bhkSerializable` (Xbox PDB), 0x10 bytes; `bhkAction`,
    /// `bhkUnaryAction` and `bhkMouseSpringAction` add no fields.
    pub struct BhkSerializable: 0x10 {
        /// `pInfo` (Xbox PDB): `hkSerializableCinfo*`.
        0x0C pInfo: u32,
    }
}

// ---------------------------------------------------------------------
// String number tests.
// ---------------------------------------------------------------------

fn is_digit(c: i8) -> bool {
    (0x30..=0x39).contains(&(c as i32))
}

// Translated from 004b1230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `IsAnInteger` (Xbox PDB): whether the string is an optional `-` and
/// digits (a lone `-` or an empty string is not).
pub fn is_an_integer(e: &mut Engine, text: Ptr) -> bool {
    let at = |e: &Engine, i: u32| e.mem.i8(text.addr().wrapping_add(i));
    let first = at(e, 0);
    if !is_digit(first) && first != b'-' as i8 {
        return false;
    }
    if at(e, 1) == 0 && first == b'-' as i8 {
        return false;
    }
    let mut i = 1;
    loop {
        let c = at(e, i);
        if c == 0 {
            return true;
        }
        if !is_digit(c) {
            return false;
        }
        i += 1;
    }
}

// Translated from 004b12d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `IsAFloat` (Xbox PDB): whether the string is digits with at most one
/// `.` and an optional leading `-` (a lone `-` or `.`, or an empty string,
/// is not).
pub fn is_a_float(e: &mut Engine, text: Ptr) -> bool {
    let at = |e: &Engine, i: u32| e.mem.i8(text.addr().wrapping_add(i));
    let first = at(e, 0);
    if first == 0 {
        return false;
    }
    if !is_digit(first) && first != b'-' as i8 && first != b'.' as i8 {
        return false;
    }
    if at(e, 1) == 0 && (first == b'-' as i8 || first == b'.' as i8) {
        return false;
    }
    let mut seen_point = first == b'.' as i8;
    let mut i = 1;
    loop {
        let c = at(e, i);
        if c == 0 {
            return true;
        }
        if !is_digit(c) && (c != b'.' as i8 || seen_point) {
            return false;
        }
        if c == b'.' as i8 {
            seen_point = true;
        }
        i += 1;
    }
}

// ---------------------------------------------------------------------
// Angles.
// ---------------------------------------------------------------------

// Translated from 004b13c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GetZAngleFromVector` (Xbox PDB): the angle of the vector's `x`, `y`
/// measured from the `y` axis (`atan(x / y)`, plus half a turn when
/// `y < 0`; a quarter or three quarters of a turn when `y == 0`), through
/// [`clamp_angle`] into `[0, 2 pi)`.
pub fn get_z_angle_from_vector(e: &mut Engine, vector: Ptr<NiPoint3>) -> f32 {
    let angle = get_unclamped_z_angle_from_vector(e, vector);
    clamp_angle(e, angle)
}

// Translated from 004b1460 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `float` form of the CRT function `00ec9010` (the engine map's
/// `logf` name is wrong: the angle code that calls it shows the arc
/// tangent).
pub fn fn_004b1460(e: &mut Engine, value: f32) -> f32 {
    e.call(CRT_ARC_TANGENT, &args![value as f64]).f64() as f32
}

// Translated from 004b1480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ClampAngle` (Xbox PDB): brings an angle into `[0, 2 pi)`: adds a full
/// turn to a negative angle (then takes the remainder, `fmodf`) and takes
/// the remainder of an angle at or above a full turn.
pub fn clamp_angle(e: &mut Engine, angle: f32) -> f32 {
    let full_turn: f64 = e.global(FULL_TURN);
    let full_turn_float: f32 = e.global(FULL_TURN_FLOAT);
    if angle < 0.0 {
        let shifted = (angle as f64 + full_turn) as f32;
        fn_004b1500(e, shifted, full_turn_float)
    } else if angle as f64 >= full_turn {
        fn_004b1500(e, angle, full_turn_float)
    } else {
        angle
    }
}

// Translated from 004b1500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fmodf(a, b)`: forwards to [`fn_004b1520`].
pub fn fn_004b1500(e: &mut Engine, a: f32, b: f32) -> f32 {
    fn_004b1520(e, a, b)
}

// Translated from 004b1520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fmodf(a, b)`: the CRT `fmod` (`00ec9130`) on the two floats widened to
/// `double`, stored as a `float`.
pub fn fn_004b1520(e: &mut Engine, a: f32, b: f32) -> f32 {
    e.call(CRT_FMOD, &args![a as f64, b as f64]).f64() as f32
}

// Translated from 004b1550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GetUnclampedZAngleFromVector` (Xbox PDB): [`get_z_angle_from_vector`]
/// without the final clamp.
pub fn get_unclamped_z_angle_from_vector(e: &mut Engine, vector: Ptr<NiPoint3>) -> f32 {
    let x = e.get(vector, NiPoint3::x);
    let y = e.get(vector, NiPoint3::y);
    if y == 0.0 {
        // `x <= 0` (or unordered) takes three quarters of a turn.
        let turn = if !(x > 0.0) {
            THREE_QUARTER_TURN
        } else {
            QUARTER_TURN
        };
        return e.global(turn);
    }
    let ratio = (x as f64 / y as f64) as f32;
    let mut angle = fn_004b1460(e, ratio);
    if y < 0.0 {
        let half_turn: f64 = e.global(HALF_TURN);
        angle = (angle as f64 + half_turn) as f32;
    }
    angle
}

// Translated from 004b15e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The shortest turn from angle `from` to angle `to`: returns its
/// magnitude (at most half a turn) and stores its direction in the float
/// at `direction`: `1.0`, `-1.0`, or `0.0` when the angles are equal.
pub fn fn_004b15e0(e: &mut Engine, from: f32, to: f32, direction: Ptr) -> f32 {
    let half_turn: f64 = e.global(HALF_TURN);
    let full_turn: f64 = e.global(FULL_TURN);
    let negative_half_turn: f64 = e.global(NEGATIVE_HALF_TURN);
    let negative_one: f32 = e.global(NEGATIVE_ONE);
    let mut difference = (to as f64 - from as f64) as f32;
    e.mem.set_f32(direction.addr(), 0.0);
    if difference != 0.0 {
        e.mem.set_f32(direction.addr(), 1.0);
        if !(difference < 0.0) {
            if difference as f64 > half_turn {
                e.mem.set_f32(direction.addr(), negative_one);
                difference = (full_turn - difference as f64) as f32;
            }
        } else if difference as f64 <= negative_half_turn {
            difference = (difference as f64 + full_turn) as f32;
        } else {
            e.mem.set_f32(direction.addr(), negative_one);
        }
    }
    difference
}

// ---------------------------------------------------------------------
// Form type lookups.
// ---------------------------------------------------------------------

// Translated from 004b1680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Maps a form type byte to an index (0 to 86) into the name table at
/// `0118a2d8`, or `-1` for the bytes that table does not cover (below 4,
/// above 0x75, or the gaps). A `switch` in the game, written as a `match`.
pub fn fn_004b1680(_e: &mut Engine, form_type: u8) -> i32 {
    match form_type {
        4 => 36,
        5 => 38,
        7 => 52,
        8 => 56,
        9 => 55,
        11 => 54,
        12 => 53,
        13 => 24,
        14 => 67,
        16 => 70,
        18 => 25,
        19 => 17,
        20 => 16,
        21 => 0,
        22 => 34,
        23 => 44,
        24 => 1,
        25 => 2,
        27 => 3,
        28 => 4,
        29 => 5,
        30 => 6,
        31 => 7,
        32 => 8,
        33 => 72,
        34 => 43,
        35 => 45,
        36 => 9,
        37 => 10,
        38 => 22,
        39 => 23,
        40 => 11,
        41 => 21,
        42 => 12,
        43 => 13,
        44 => 14,
        45 => 15,
        46 => 20,
        47 => 18,
        48 => 31,
        49 => 57,
        50 => 66,
        51 => 33,
        52 => 19,
        53 => 58,
        54 => 59,
        71 => 49,
        73 => 50,
        74 => 26,
        75 => 27,
        76 => 28,
        77 => 29,
        78 => 30,
        79 => 32,
        81 => 35,
        82 => 37,
        85 => 39,
        86 => 40,
        87 => 41,
        88 => 42,
        91 => 46,
        93 => 51,
        94 => 47,
        95 => 48,
        97 => 64,
        102 => 71,
        103 => 73,
        104 => 74,
        106 => 75,
        107 => 76,
        108 => 77,
        109 => 78,
        110 => 79,
        111 => 80,
        112 => 81,
        113 => 82,
        114 => 83,
        115 => 84,
        116 => 85,
        117 => 86,
        _ => -1,
    }
}

/// The form type bytes for which `004b1b60` reads a single name slot, with
/// the slot's address (`01188f98` to `01189034`): the pointer stored there
/// is the name.
const FORM_NAME_SLOTS: [(u8, u32); 40] = [
    (0x03, 0x0118_8fbc),
    (0x06, 0x0118_8fb4),
    (0x07, 0x0118_8f9c),
    (0x08, 0x0118_8fac),
    (0x09, 0x0118_8fd8),
    (0x0a, 0x0118_8fdc),
    (0x0b, 0x0118_8fe0),
    (0x0c, 0x0118_8f98),
    (0x0d, 0x0118_8fb0),
    (0x0e, 0x0118_8ffc),
    (0x0f, 0x0118_8fd0),
    (0x10, 0x0118_8fc0),
    (0x11, 0x0118_8fa0),
    (0x31, 0x0118_8ff0),
    (0x35, 0x0118_8fcc),
    (0x36, 0x0118_8fc8),
    (0x37, 0x0118_8fb8),
    (0x39, 0x0118_8fa8),
    (0x3a, 0x0118_8fe4),
    (0x41, 0x0118_8fc4),
    (0x45, 0x0118_8fa4),
    (0x47, 0x0118_8fd4),
    (0x48, 0x0118_8fe8),
    (0x49, 0x0118_8fec),
    (0x53, 0x0118_8ff4),
    (0x54, 0x0118_8ff8),
    (0x66, 0x0118_9000),
    (0x68, 0x0118_9004),
    (0x6a, 0x0118_9008),
    (0x6b, 0x0118_900c),
    (0x6c, 0x0118_9010),
    (0x6d, 0x0118_9014),
    (0x6e, 0x0118_9018),
    (0x6f, 0x0118_901c),
    (0x70, 0x0118_9020),
    (0x71, 0x0118_9024),
    (0x72, 0x0118_9028),
    (0x73, 0x0118_902c),
    (0x74, 0x0118_9030),
    (0x75, 0x0118_9034),
];

// Translated from 004b1b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The name string (its address) of a form type byte; the log message
/// below names the function `FormIDToString`. Through `004b1680`'s index
/// into the table at `0118a2d8` when it has one, else from the slot
/// listed for the type in `FORM_NAME_SLOTS`, else from the 12-byte table
/// at `01187004` (a type of 0x79 or more logs an error and uses entry 0).
pub fn fn_004b1b60(e: &mut Engine, form_type: u8) -> u32 {
    let index = fn_004b1680(e, form_type);
    if index >= 0 {
        return e.global(FORM_NAME_BY_INDEX + 4 * index as u32);
    }
    if let Some(&(_, slot)) = FORM_NAME_SLOTS.iter().find(|&&(t, _)| t == form_type) {
        return e.global(slot);
    }
    let mut form_type = form_type as u32;
    if form_type >= 0x79 {
        let name: u32 = e.global(FORM_NAME_BY_TYPE + 12 * form_type);
        e.call(LOG_MESSAGE, &args![INVALID_FORM_ID_MESSAGE, name]);
        form_type = 0;
    }
    e.global(FORM_NAME_BY_TYPE + 12 * form_type)
}

// ---------------------------------------------------------------------
// Segment against rectangle.
// ---------------------------------------------------------------------

fn copy_words(e: &mut Engine, dst: u32, src: u32, words: u32) {
    for i in 0..words {
        let word = e.mem.u32(src + 4 * i);
        e.mem.set_u32(dst + 4 * i, word);
    }
}

/// Builds `NiPoint3(x, y, z)` through the game's constructor in `scratch`
/// (12 bytes) and copies the three words it returns to `dst`.
fn store_point(e: &mut Engine, scratch: u32, dst: u32, x: f32, y: f32, z: f32) {
    let built = e.call(POINT3_CONSTRUCT, &args![scratch, x, y, z]).u32();
    copy_words(e, dst, built, 3);
}

/// Records an intersection point (`z` is 0) in the first free one of the
/// two result points.
fn record_hit(e: &mut Engine, scratch: u32, slots: [u32; 2], hits: &mut usize, x: f32, y: f32) {
    store_point(e, scratch, slots[*hits], x, y, 0.0);
    *hits += 1;
}

// Translated from 004b1e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clips the segment `from` to `to` (`NiPoint3`s; only `x` and `y` are
/// read) against the integer rectangle `rect` (`x min`, `y max`, `x max`,
/// `y min`, with `x min < x max` and `y min < y max`) and stores one
/// intersection point in `out`; returns whether there is one.
///
/// - Both ends beside the same edge, or both strictly inside: `false`,
///   `out` untouched.
/// - A segment of one point gives that point (all three words of `from`)
///   when it lies on or in the rectangle; a point strictly inside is
///   rejected by the previous rule.
/// - A vertical segment gives `(x, y max, 0)` when `to.y < from.y`, else
///   `(x, y min, 0)`; a horizontal one `(x max, y, 0)` when `to.x < from.x`,
///   else `(x min, y, 0)`, provided the line crosses the rectangle.
/// - Otherwise the line `y = m x + b` through the segment is intersected
///   with the left, top, right and bottom edges in that order, keeping the
///   first two hits. One hit is the result. With two: when `from` is
///   strictly inside the rectangle the unit vector along the segment and
///   the unit vector to the first hit are added, and the second hit is
///   chosen when the sum is at most one long, else the first; when `from`
///   is not strictly inside, the hit nearer `from` is chosen (the second
///   on a tie).
pub fn fn_004b1e80(e: &mut Engine, rect: Ptr, from: Ptr, to: Ptr, out: Ptr) -> bool {
    // The game's locals: two points, a scratch point and three vectors.
    let scratch = e.mem.alloc(0x60);
    let found = segment_against_rect(e, scratch, rect, from, to, out);
    e.mem.free(scratch);
    found
}

fn segment_against_rect(
    e: &mut Engine,
    scratch: u32,
    rect: Ptr,
    from: Ptr,
    to: Ptr,
    out: Ptr,
) -> bool {
    let x_min = e.mem.i32(rect.addr());
    let y_max = e.mem.i32(rect.addr() + 4);
    let x_max = e.mem.i32(rect.addr() + 8);
    let y_min = e.mem.i32(rect.addr() + 0xc);
    if x_min >= x_max || y_max <= y_min {
        return false;
    }
    let (x_min_f, y_max_f, x_max_f, y_min_f) =
        (x_min as f64, y_max as f64, x_max as f64, y_min as f64);
    let x1 = e.mem.f32(from.addr()) as f64;
    let y1 = e.mem.f32(from.addr() + 4) as f64;
    let x2 = e.mem.f32(to.addr()) as f64;
    let y2 = e.mem.f32(to.addr() + 4) as f64;

    // Both ends beside the same edge: no intersection.
    if (x1 < x_min_f && x2 < x_min_f)
        || (y1 < y_min_f && y2 < y_min_f)
        || (x1 > x_max_f && x2 > x_max_f)
        || (y1 > y_max_f && y2 > y_max_f)
    {
        return false;
    }
    // Both ends strictly inside: nothing to clip.
    let strictly_inside = |x: f64, y: f64| x > x_min_f && x < x_max_f && y > y_min_f && y < y_max_f;
    if strictly_inside(x1, y1) && strictly_inside(x2, y2) {
        return false;
    }

    let (x_equal, y_equal) = (x1 == x2, y1 == y2);
    if x_equal && y_equal {
        // A single point.
        if x1 <= x_max_f && x1 >= x_min_f && y1 <= y_max_f && y1 >= y_min_f {
            copy_words(e, out.addr(), from.addr(), 3);
            return true;
        }
        return false;
    }
    if x_equal {
        // Vertical.
        if x1 <= x_max_f && x1 >= x_min_f {
            let y = if y2 < y1 { y_max } else { y_min };
            store_point(e, scratch, out.addr(), x1 as f32, y as f32, 0.0);
            return true;
        }
        return false;
    }
    if y_equal {
        // Horizontal.
        if y1 <= y_max_f && y1 >= y_min_f {
            let x = if x2 < x1 { x_max } else { x_min };
            store_point(e, scratch, out.addr(), x as f32, y1 as f32, 0.0);
            return true;
        }
        return false;
    }

    // The general case: the line through the segment against the edges.
    let (first_hit, second_hit, rest) = (scratch, scratch + 0xc, scratch + 0x18);
    let slots = [first_hit, second_hit];
    let float_max: f32 = e.global(FLOAT_MAX);
    e.call(
        POINT3_CONSTRUCT,
        &args![first_hit, float_max, float_max, float_max],
    );
    e.call(
        POINT3_CONSTRUCT,
        &args![second_hit, float_max, float_max, float_max],
    );
    let slope = ((y2 - y1) / (x2 - x1)) as f32;
    let intercept = (y2 - x2 * slope as f64) as f32;
    let (m, b) = (slope as f64, intercept as f64);
    let mut hits = 0;
    // Left edge.
    let y_left = (x_min_f * m + b) as f32 as f64;
    if hits < 2 && y_left <= y_max_f && y_left >= y_min_f {
        record_hit(e, rest, slots, &mut hits, x_min as f32, y_left as f32);
    }
    // Top edge.
    let x_top = ((y_max_f - b) / m) as f32 as f64;
    if hits < 2 && x_top <= x_max_f && x_top >= x_min_f {
        record_hit(e, rest, slots, &mut hits, x_top as f32, y_max as f32);
    }
    // Right edge.
    let y_right = (x_max_f * m + b) as f32 as f64;
    if hits < 2 && y_right <= y_max_f && y_right >= y_min_f {
        record_hit(e, rest, slots, &mut hits, x_max as f32, y_right as f32);
    }
    // Bottom edge.
    let x_bottom = ((y_min_f - b) / m) as f32 as f64;
    if hits < 2 && x_bottom <= x_max_f && x_bottom >= x_min_f {
        record_hit(e, rest, slots, &mut hits, x_bottom as f32, y_min as f32);
    }
    if hits == 0 {
        return false;
    }
    if hits != 2 {
        copy_words(e, out.addr(), first_hit, 3);
        return true;
    }
    let from_not_strictly_inside = x1 <= x_min_f || x1 >= x_max_f || y1 <= y_min_f || y1 >= y_max_f;
    let chosen = if from_not_strictly_inside {
        let to_first = e.mem.alloc(12);
        let to_second = e.mem.alloc(12);
        e.call(POINT3_SUBTRACT, &args![first_hit, to_first, from]);
        let distance_first = e.call(POINT3_LENGTH, &args![to_first]).f64();
        e.call(POINT3_SUBTRACT, &args![second_hit, to_second, from]);
        let distance_second = e.call(POINT3_LENGTH, &args![to_second]).f64();
        e.mem.free(to_first);
        e.mem.free(to_second);
        if distance_second <= distance_first {
            second_hit
        } else {
            first_hit
        }
    } else {
        let vectors = e.mem.alloc(0x24);
        let (along_segment, to_first, sum) = (vectors, vectors + 0xc, vectors + 0x18);
        e.call(POINT3_SUBTRACT, &args![to, along_segment, from]);
        e.call(POINT3_SUBTRACT, &args![first_hit, to_first, from]);
        e.mem.set_f32(along_segment + 8, 0.0);
        e.mem.set_f32(to_first + 8, 0.0);
        e.call(POINT3_UNITIZE, &args![along_segment]);
        e.call(POINT3_UNITIZE, &args![to_first]);
        e.call(POINT3_ADD, &args![along_segment, sum, to_first]);
        let length = e.call(POINT3_LENGTH, &args![sum]).f64();
        e.mem.free(vectors);
        let one: f64 = e.global(ONE);
        if length <= one {
            second_hit
        } else {
            first_hit
        }
    };
    copy_words(e, out.addr(), chosen, 3);
    true
}

// ---------------------------------------------------------------------
// Pick helpers.
// ---------------------------------------------------------------------

// Translated from 004b2780 (decompiled, FalloutNV.exe 1.4.0.525)
/// Picks the ray `origin` / `direction` against `node` within `distance`
/// through [`fn_004b2800`]; with `reset`, first clears the pick state
/// (`00705fc0(object, 0)` and `PickObjects(object, 011f426c, 011f426c,
/// 0)`). False without doing anything when `object` or `node` is null or
/// `distance` is zero.
pub fn fn_004b2780(
    e: &mut Engine,
    object: Ptr,
    origin: Ptr,
    direction: Ptr,
    distance: f32,
    node: Ptr,
    reset: bool,
) -> bool {
    if object.is_null() || node.is_null() || distance == 0.0 {
        return false;
    }
    if reset {
        e.call(PICK_SET_ROOT, &args![object, 0u32]);
        e.call(PICK_OBJECTS, &args![object, PICK_STATE, PICK_STATE, 0u32]);
    }
    fn_004b2800(e, object, origin, direction, distance, node)
}

// Translated from 004b2800 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the ray hits `node` or one of its descendants within
/// `distance`. A node is skipped when both pick checks (`00824060(object)`
/// and `00456610(node)`) answer true, or when it has no bound
/// ([`fn_004b2960`]), or when the distance from its bound's centre to the
/// ray origin is not less than `radius + distance`. A node whose virtual
/// function `+0xc` gives a child container recurses over the children; any
/// other is picked with `PickObjects(object, origin, direction, 1)` after
/// selecting it with `00705fc0`.
pub fn fn_004b2800(
    e: &mut Engine,
    object: Ptr,
    origin: Ptr,
    direction: Ptr,
    distance: f32,
    node: Ptr,
) -> bool {
    let mut hit = false;
    if object.is_null() || node.is_null() {
        return false;
    }
    if e.call(PICK_BLOCKED_A, &args![object]).bool() && e.call(PICK_BLOCKED_B, &args![node]).bool()
    {
        return false;
    }
    if !fn_004b2960(e, node) {
        return false;
    }
    // The bound is copied (centre and radius) to a local; the vector from
    // its centre to the ray origin is built next to it. `006815c0` returns
    // its argument, which `00439ef0` then takes as `this`.
    let bound = e.call(GET_WORLD_BOUND, &args![node]).u32();
    let locals = e.mem.alloc(0x1c);
    let centre_to_origin = locals + 0x10;
    copy_words(e, locals, bound, 4);
    let centre = e.call(POINT3_EMPTY_CONSTRUCTOR, &args![locals]).u32();
    e.call(POINT3_SUBTRACT, &args![centre, centre_to_origin, origin]);
    let length = e.call(POINT3_LENGTH, &args![centre_to_origin]).f64();
    let radius = e.call(BOUND_RADIUS, &args![locals]).f64();
    e.mem.free(locals);
    if radius + distance as f64 > length {
        let children = e.vcall(node.addr(), 0xc, &[]).u32();
        if children != 0 {
            let count = e.call(CHILD_COUNT, &args![children]).i32();
            for i in 0..count {
                let child = e.call(CHILD_AT, &args![children, i]).u32();
                if fn_004b2800(e, object, origin, direction, distance, Ptr::new(child)) {
                    hit = true;
                }
            }
        } else {
            e.call(PICK_SET_ROOT, &args![object, node]);
            if e.call(PICK_OBJECTS, &args![object, origin, direction, 1u32])
                .bool()
            {
                hit = true;
            }
        }
    }
    hit
}

// Translated from 004b2960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the node has a world bound (`NiAVObject::m_kWorldBound`, the
/// pointer at `+0x20`) with a non-zero radius.
pub fn fn_004b2960(e: &mut Engine, node: Ptr) -> bool {
    let bound = e.get(node.cast::<NiAVObject>(), NiAVObject::m_kWorldBound);
    if bound == 0 {
        return false;
    }
    e.call(BOUND_RADIUS, &args![bound]).f32() != 0.0
}

// ---------------------------------------------------------------------
// Debug shape builders.
// ---------------------------------------------------------------------

/// An array of `count` elements of `element_size` bytes from `operator
/// new`, each constructed through the vector constructor with
/// `constructor`; null when the allocation fails.
fn new_array(e: &mut Engine, element_size: u32, count: u32, constructor: u32) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![element_size * count]).u32();
    if block != 0 {
        e.call(
            VECTOR_CONSTRUCT,
            &args![block, element_size, count, constructor],
        );
    }
    block
}

/// `count` copies of the four-word colour at `color` into `colors`.
fn fill_colors(e: &mut Engine, colors: u32, color: u32, count: u32) {
    for i in 0..count {
        copy_words(e, colors + 16 * i, color, 4);
    }
}

/// The triangle index array of a debug shape: from the Gamebryo array
/// allocator, filled with the `u16` indices.
fn new_triangle_indices(e: &mut Engine, indices: &[u16]) -> u32 {
    let block = e.call(NI_ARRAY_NEW, &args![2 * indices.len() as u32]).u32();
    for (i, &index) in indices.iter().enumerate() {
        e.mem.set_u16(block + 2 * i as u32, index);
    }
    block
}

/// A new `NiTriShape` (0xc4 bytes) over the arrays; null when the
/// allocation fails.
fn new_tri_shape(
    e: &mut Engine,
    vertex_count: u32,
    vertices: u32,
    colors: u32,
    triangle_count: u32,
    triangles: u32,
) -> u32 {
    let block = e.call(NI_OPERATOR_NEW, &args![0xc4u32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(
        NI_TRI_SHAPE_CONSTRUCT,
        &args![
            block,
            vertex_count,
            vertices,
            0u32,
            colors,
            0u32,
            0u32,
            0u32,
            triangle_count,
            triangles
        ],
    )
    .u32()
}

/// A new `NiLines` (0xc4 bytes) over the arrays; null when the allocation
/// fails.
fn new_lines(e: &mut Engine, count: u32, vertices: u32, colors: u32, flags: u32) -> u32 {
    let block = e.call(NI_OPERATOR_NEW, &args![0xc4u32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(
        NI_LINES_CONSTRUCT,
        &args![block, count, vertices, colors, 0u32, 0u32, 0u32, flags],
    )
    .u32()
}

/// With `add_property`, gives the shape a new `BSShaderNoLightingProperty`
/// and prepares it for rendering; returns the shape.
fn finish_shape(e: &mut Engine, shape: u32, add_property: bool) -> u32 {
    if add_property {
        let block = e.call(NI_OPERATOR_NEW, &args![0x80u32]).u32();
        let property = if block == 0 {
            0
        } else {
            e.call(NO_LIGHTING_PROPERTY_CONSTRUCT, &args![block]).u32()
        };
        e.call(ATTACH_PROPERTY, &args![shape, property]);
        e.call(PREPARE_OBJECT, &args![shape, 0u32, 0u32]);
    }
    shape
}

/// `MakeTriPoint`'s triangle indices (8 triangles over 6 vertices).
const TRI_POINT_INDICES: [u16; 24] = [
    0, 2, 1, 0, 3, 2, 0, 4, 3, 0, 1, 4, 1, 2, 5, 2, 3, 5, 3, 4, 5, 4, 1, 5,
];
/// `MakeQuadBox`'s indices (24 triangles over 8 vertices).
const QUAD_BOX_INDICES: [u16; 72] = [
    0, 2, 1, 0, 3, 2, 0, 7, 3, 0, 4, 7, 1, 4, 0, 1, 5, 4, 2, 5, 1, 2, 6, 5, 3, 6, 2, 3, 7, 6, 4, 7,
    6, 4, 6, 5, 0, 1, 2, 0, 2, 3, 0, 3, 7, 0, 7, 4, 1, 0, 4, 1, 4, 5, 2, 1, 5, 2, 5, 6, 3, 2, 6, 3,
    6, 7, 4, 6, 7, 4, 5, 6,
];
/// `MakeTriangle`'s indices (both faces of one triangle).
const TRIANGLE_INDICES: [u16; 6] = [0, 2, 1, 0, 1, 2];

// Translated from 004b29b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MakeTriPoint` (Xbox PDB): a `NiTriShape` of 6 vertices (`(0, 0, +-radius
/// * sqrt 2)` and the four corners `(+-radius, +-radius, 0)`) and 8
/// triangles, every vertex coloured with the `NiColorA` at `color`; with
/// `add_property` also given a no-lighting property.
pub fn make_tri_point(e: &mut Engine, radius: f32, color: Ptr, add_property: bool) -> u32 {
    let vertices = new_array(e, 12, 6, POINT3_EMPTY_CONSTRUCTOR);
    let scratch = e.mem.alloc(12);
    let square_root_of_two: f64 = e.global(SQUARE_ROOT_OF_TWO);
    let apex = (radius as f64 * square_root_of_two) as f32;
    store_point(e, scratch, vertices, 0.0, 0.0, apex);
    store_point(e, scratch, vertices + 12, -radius, radius, 0.0);
    store_point(e, scratch, vertices + 24, radius, radius, 0.0);
    store_point(e, scratch, vertices + 36, radius, -radius, 0.0);
    store_point(e, scratch, vertices + 48, -radius, -radius, 0.0);
    let apex_below = (-(radius as f64) * square_root_of_two) as f32;
    store_point(e, scratch, vertices + 60, 0.0, 0.0, apex_below);
    e.mem.free(scratch);
    let colors = new_array(e, 16, 6, COLOR_ELEMENT_CONSTRUCTOR);
    fill_colors(e, colors, color.addr(), 6);
    let triangles = new_triangle_indices(e, &TRI_POINT_INDICES);
    let shape = new_tri_shape(e, 6, vertices, colors, 8, triangles);
    finish_shape(e, shape, add_property)
}

// Translated from 004b2eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MakeQuadBox` (Xbox PDB): a `NiTriShape` of 8 vertices (copied from the
/// eight `NiPoint3`s `corner_0` to `corner_7`) and 24 triangles, every
/// vertex coloured with the `NiColorA` at `color`.
#[allow(clippy::too_many_arguments)]
pub fn make_quad_box(
    e: &mut Engine,
    corner_0: Ptr,
    corner_1: Ptr,
    corner_2: Ptr,
    corner_3: Ptr,
    corner_4: Ptr,
    corner_5: Ptr,
    corner_6: Ptr,
    corner_7: Ptr,
    color: Ptr,
    add_property: bool,
) -> u32 {
    let vertices = new_array(e, 12, 8, POINT3_EMPTY_CONSTRUCTOR);
    let corners = [
        corner_0, corner_1, corner_2, corner_3, corner_4, corner_5, corner_6, corner_7,
    ];
    for (i, corner) in corners.iter().enumerate() {
        copy_words(e, vertices + 12 * i as u32, corner.addr(), 3);
    }
    let colors = new_array(e, 16, 8, COLOR_ELEMENT_CONSTRUCTOR);
    fill_colors(e, colors, color.addr(), 8);
    let triangles = new_triangle_indices(e, &QUAD_BOX_INDICES);
    let shape = new_tri_shape(e, 8, vertices, colors, 0x18, triangles);
    finish_shape(e, shape, add_property)
}

// Translated from 004b3570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MakeTriangle` (Xbox PDB): a `NiTriShape` of one triangle (3 vertices
/// passed by value as 9 floats; the index array holds both faces),
/// coloured with the `NiColorA` at `color`.
#[allow(clippy::too_many_arguments)]
pub fn make_triangle(
    e: &mut Engine,
    x0: f32,
    y0: f32,
    z0: f32,
    x1: f32,
    y1: f32,
    z1: f32,
    x2: f32,
    y2: f32,
    z2: f32,
    color: Ptr,
    add_property: bool,
) -> u32 {
    let vertices = new_array(e, 12, 3, POINT3_EMPTY_CONSTRUCTOR);
    for (i, value) in [x0, y0, z0, x1, y1, z1, x2, y2, z2].iter().enumerate() {
        e.mem.set_f32(vertices + 4 * i as u32, *value);
    }
    let colors = new_array(e, 16, 3, COLOR_ELEMENT_CONSTRUCTOR);
    fill_colors(e, colors, color.addr(), 3);
    let triangles = new_triangle_indices(e, &TRIANGLE_INDICES);
    let shape = new_tri_shape(e, 3, vertices, colors, 2, triangles);
    finish_shape(e, shape, add_property)
}

// Translated from 004b3800 (decompiled, FalloutNV.exe 1.4.0.525)
/// The cross product `this x other` of two `NiPoint3`s, built into `out`
/// (`NiPoint3::operator%`); returns `out`.
pub fn fn_004b3800(e: &mut Engine, this: Ptr<NiPoint3>, out: Ptr, other: Ptr<NiPoint3>) -> Ptr {
    let (ax, ay, az) = (
        e.get(this, NiPoint3::x) as f64,
        e.get(this, NiPoint3::y) as f64,
        e.get(this, NiPoint3::z) as f64,
    );
    let (bx, by, bz) = (
        e.get(other, NiPoint3::x) as f64,
        e.get(other, NiPoint3::y) as f64,
        e.get(other, NiPoint3::z) as f64,
    );
    let z = (ax * by - ay * bx) as f32;
    let y = (az * bx - ax * bz) as f32;
    let x = (ay * bz - az * by) as f32;
    e.call(POINT3_CONSTRUCT, &args![out, x, y, z]);
    out
}

// Translated from 004b3890 (decompiled, FalloutNV.exe 1.4.0.525)
/// A `NiLines` of one segment: vertices copied from the `NiPoint3`s `start`
/// and `end`, colours from the `NiColorA`s `start_color` and `end_color`,
/// flag bytes `[1, 0]` (the segment connects the two vertices); optionally
/// given a no-lighting property.
pub fn fn_004b3890(
    e: &mut Engine,
    start: Ptr,
    start_color: Ptr,
    end: Ptr,
    end_color: Ptr,
    add_property: bool,
) -> u32 {
    let vertices = new_array(e, 12, 2, POINT3_EMPTY_CONSTRUCTOR);
    copy_words(e, vertices, start.addr(), 3);
    copy_words(e, vertices + 12, end.addr(), 3);
    let colors = new_array(e, 16, 2, COLOR_ELEMENT_CONSTRUCTOR);
    copy_words(e, colors, start_color.addr(), 4);
    copy_words(e, colors + 16, end_color.addr(), 4);
    let flags = e.call(OPERATOR_NEW, &args![2u32]).u32();
    e.mem.set_u8(flags, 1);
    e.mem.set_u8(flags + 1, 0);
    let lines = new_lines(e, 2, vertices, colors, flags);
    finish_shape(e, lines, add_property)
}

// Translated from 004b3ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Linear interpolation for `(a, b, t0, t1, t)`: `a + (b - a) * (t - t0) /
/// (t1 - t0)`.
pub fn fn_004b3ab0(_e: &mut Engine, a: f32, b: f32, t0: f32, t1: f32, t: f32) -> f32 {
    let ratio = (t as f64 - t0 as f64) / (t1 as f64 - t0 as f64);
    (ratio * (b as f64 - a as f64) + a as f64) as f32
}

// Translated from 004b3ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPoint3 * NiMatrix3` (the vector as a row of the nine floats by rows):
/// built into `out`, which it returns. `(out, vector, matrix)`, cdecl.
pub fn fn_004b3ae0(e: &mut Engine, out: Ptr, vector: Ptr, matrix: Ptr) -> Ptr {
    let v = |e: &Engine, i: u32| e.mem.f32(vector.addr() + 4 * i) as f64;
    let m = |e: &Engine, i: u32| e.mem.f32(matrix.addr() + 4 * i) as f64;
    let (v0, v1, v2) = (v(e, 0), v(e, 1), v(e, 2));
    let z = (v0 * m(e, 2) + v1 * m(e, 5) + v2 * m(e, 8)) as f32;
    let y = (v0 * m(e, 1) + v1 * m(e, 4) + v2 * m(e, 7)) as f32;
    let x = (v0 * m(e, 0) + v1 * m(e, 3) + v2 * m(e, 6)) as f32;
    e.call(POINT3_CONSTRUCT, &args![out, x, y, z]);
    out
}

// Translated from 004b3b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MakeRectangle` (Xbox PDB): a `NiLines` outlining the square
/// `(+-radius, +-radius, 0)` (4 vertices, each coloured with the
/// `NiColorA` at `color`, every flag byte 1).
pub fn make_rectangle(e: &mut Engine, radius: f32, color: Ptr, add_property: bool) -> u32 {
    let vertices = new_array(e, 12, 4, POINT3_EMPTY_CONSTRUCTOR);
    let colors = new_array(e, 16, 4, COLOR_ELEMENT_CONSTRUCTOR);
    let flags = e.call(OPERATOR_NEW, &args![4u32]).u32();
    let scratch = e.mem.alloc(12);
    store_point(e, scratch, vertices, -radius, -radius, 0.0);
    store_point(e, scratch, vertices + 12, -radius, radius, 0.0);
    store_point(e, scratch, vertices + 24, radius, radius, 0.0);
    store_point(e, scratch, vertices + 36, radius, -radius, 0.0);
    e.mem.free(scratch);
    for i in 0..4 {
        copy_words(e, colors + 16 * i, color.addr(), 4);
        e.mem.set_u8(flags + i, 1);
    }
    let lines = new_lines(e, 4, vertices, colors, flags);
    finish_shape(e, lines, add_property)
}

// ---------------------------------------------------------------------
// The snapping function, matrices and points.
// ---------------------------------------------------------------------

// Translated from 004b3e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Matches the points of geometry `second` to those of geometry `first`
/// and copies `first`'s vectors over `second`'s.
///
/// Both objects' first point arrays are taken to world space
/// ([`fn_004b4910`]). For each point of `second` the nearest point of
/// `first` closer than `threshold` (any distance when `threshold < 0`) is
/// found, and a map from the `first` point's index to (the `second` index,
/// the `second` count, the distance) keeps the nearest `second` point per
/// `first` point: a farther one never replaces it, one within `0.001`
/// (`0040eb10`) replaces it but keeps the earlier `second` index as the
/// entry's second word. Each pair then sets the `second` vector (second
/// array, `004a8030`) to `first`'s vector rotated by the relative rotation
/// (the inverse of `second`'s world rotation times `first`'s,
/// [`fn_004b4500`]) and unitized; when the entry's second word is a valid
/// `second` index it is copied there too. With `debug`, the points
/// (`0049ec60` arrays) are moved by 10 times their vectors as well
/// (`first`'s only when it is not the geometry of the previous run), and
/// the geometry data of `first` is marked changed too. Returns whether
/// any pair was recorded; `second`'s data (`MarkAsChanged(3)`) is marked
/// when so.
///
/// Which array is positions and which normals is not confirmed from the
/// exe, so they are called `points` (`0049ec60`) and `vectors`
/// (`004a8030`). The inverse `NiTransform` the game builds with `004b4880`
/// is not used afterwards but the call is kept; a running minimum distance
/// nobody reads is left out. The map's destructor (`004b9cd0`) runs on
/// every exit.
pub fn fn_004b3e60(
    e: &mut Engine,
    first: Ptr,
    second: Ptr,
    threshold: f32,
    _unused_3: u32,
    debug: bool,
) -> bool {
    let map = e.mem.alloc(0x10);
    e.call(POINT_MAP_CONSTRUCT, &args![map, 0x25u32]);
    // The game's locals: a rotation matrix, a transform and a point
    // (constructors that only return `this`, or leave the memory as is),
    // then the scratch for the products and vectors below.
    let scratch = e.mem.alloc(0x100);
    e.call(POINT3_EMPTY_CONSTRUCTOR, &args![scratch + 0x7c]);
    e.call(TRANSFORM_CONSTRUCT, &args![scratch]);
    e.call(POINT3_EMPTY_CONSTRUCTOR, &args![scratch + 0xf0]);
    let changed = snap_geometry(e, map, scratch, first, second, threshold, debug);
    e.mem.free(scratch);
    e.call(POINT_MAP_DESTRUCT, &args![map]);
    e.mem.free(map);
    changed
}

fn snap_geometry(
    e: &mut Engine,
    map: u32,
    scratch: u32,
    first: Ptr,
    second: Ptr,
    threshold: f32,
    debug: bool,
) -> bool {
    if first.is_null() || second.is_null() || first == second {
        return false;
    }
    let first_points = e.call(POINTS_FIRST, &args![first]).u32();
    let first_vectors = e.call(POINTS_SECOND, &args![first]).u32();
    let first_count = e.call(POINT_COUNT, &args![first]).u16() as u32;
    if first_points == 0 || first_vectors == 0 || first_count == 0 {
        return false;
    }
    let second_points = e.call(POINTS_FIRST, &args![second]).u32();
    let second_vectors = e.call(POINTS_SECOND, &args![second]).u32();
    let second_count = e.call(POINT_COUNT, &args![second]).u16() as u32;
    if second_points == 0 || second_vectors == 0 || second_count == 0 {
        return false;
    }
    let first_world = fn_004b4910(e, first);
    if first_world == 0 {
        return false;
    }
    let second_world = fn_004b4910(e, second);
    if second_world == 0 {
        e.call(OPERATOR_DELETE, &args![first_world]);
        return false;
    }

    // Locals in the scratch block: the transform (0x34 bytes at +0), two
    // matrices and the rotation, then the vectors.
    let (transform, inverse, product, matrix) =
        (scratch, scratch + 0x34, scratch + 0x58, scratch + 0x7c);
    let (difference, found, iterator, key, value) = (
        scratch + 0xa0,
        scratch + 0xac,
        scratch + 0xb8,
        scratch + 0xbc,
        scratch + 0xc0,
    );
    let (rotated, shifted_first, shifted_second) = (scratch + 0xcc, scratch + 0xd8, scratch + 0xe4);

    // The relative rotation: the inverse of `second`'s world rotation
    // times `first`'s.
    let second_transform = e.call(WORLD_TRANSFORM, &args![second]).u32();
    e.call(
        TRANSFORM_INVERT_NON_UNIFORM,
        &args![second_transform, transform],
    );
    let first_transform = e.call(WORLD_TRANSFORM, &args![first]).u32();
    let second_transform = e.call(WORLD_TRANSFORM, &args![second]).u32();
    ni_matrix3_inverse(e, Ptr::new(second_transform), Ptr::new(inverse));
    let rotation = e
        .call(MATRIX_PRODUCT, &args![inverse, product, first_transform])
        .u32();
    copy_words(e, matrix, rotation, 9);

    let float_max: f32 = e.global(FLOAT_MAX);
    let tolerance: f32 = e.global(SNAP_TOLERANCE);
    for second_index in 0..second_count {
        let mut nearest = first_count;
        let mut nearest_distance = if threshold < 0.0 {
            float_max
        } else {
            threshold
        };
        for first_index in 0..first_count {
            e.call(
                POINT3_SUBTRACT,
                &args![
                    second_world + 12 * second_index,
                    difference,
                    first_world + 12 * first_index
                ],
            );
            let distance = e.call(POINT3_LENGTH, &args![difference]).f64() as f32;
            if distance < nearest_distance {
                nearest = first_index;
                nearest_distance = distance;
            }
        }
        if nearest >= first_count {
            continue;
        }
        // The entry for this pair: (second index, second count, distance).
        let distance_bits = nearest_distance.to_bits();
        let mut partner = second_count;
        let record = if e.call(POINT_MAP_FIND, &args![map, nearest, found]).bool() {
            let kept_distance = e.mem.f32(found + 8);
            if e.call(
                FLOATS_NEAR,
                &args![nearest_distance, kept_distance, tolerance],
            )
            .bool()
            {
                partner = e.mem.u32(found);
                true
            } else {
                nearest_distance < kept_distance
            }
        } else {
            true
        };
        if record {
            e.call(
                POINT_MAP_SET_AT,
                &args![map, nearest, second_index, partner, distance_bits],
            );
        }
    }

    let mut changed = false;
    let mut position = e.call(POINT_MAP_FIRST, &args![map]).u32();
    e.mem.set_u32(iterator, position);
    while position != 0 {
        e.call(POINT_MAP_NEXT, &args![map, iterator, key, value]);
        position = e.mem.u32(iterator);
        let first_index = e.mem.u32(key);
        let second_index = e.mem.u32(value);
        let partner = e.mem.u32(value + 4);
        let moved = fn_004b4500(
            e,
            Ptr::new(matrix),
            Ptr::new(rotated),
            Ptr::new(first_vectors + 12 * first_index),
        );
        copy_words(e, second_vectors + 12 * second_index, moved.addr(), 3);
        e.call(POINT3_UNITIZE, &args![second_vectors + 12 * second_index]);
        if partner < second_count {
            copy_words(
                e,
                second_vectors + 12 * partner,
                second_vectors + 12 * second_index,
                3,
            );
        }
        if debug {
            let offset: f32 = e.global(DEBUG_OFFSET);
            if e.global::<u32>(LAST_SNAPPED_OBJECT) != first.addr() {
                let scaled = e
                    .call(
                        POINT3_SCALED,
                        &args![shifted_first, offset, first_vectors + 12 * first_index],
                    )
                    .u32();
                e.call(
                    POINT3_ADD_ASSIGN,
                    &args![first_points + 12 * first_index, scaled],
                );
            }
            let scaled = e
                .call(
                    POINT3_SCALED,
                    &args![shifted_second, offset, second_vectors + 12 * second_index],
                )
                .u32();
            e.call(
                POINT3_ADD_ASSIGN,
                &args![second_points + 12 * second_index, scaled],
            );
            if partner < second_count {
                copy_words(
                    e,
                    second_points + 12 * partner,
                    second_points + 12 * second_index,
                    3,
                );
            }
        }
        changed = true;
    }
    if changed {
        if debug {
            let data = e.call(GEOMETRY_DATA, &args![first]).u32();
            e.call(MARK_AS_CHANGED, &args![data, 3u32]);
        }
        let data = e.call(GEOMETRY_DATA, &args![second]).u32();
        e.call(MARK_AS_CHANGED, &args![data, 3u32]);
    }
    e.call(OPERATOR_DELETE, &args![first_world]);
    e.call(OPERATOR_DELETE, &args![second_world]);
    e.set_global(LAST_SNAPPED_OBJECT, first.addr());
    changed
}

// Translated from 004b4500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMatrix3 * NiPoint3` (`this` is the matrix, nine floats by rows): the
/// product is built into `out`, which it returns.
pub fn fn_004b4500(e: &mut Engine, this: Ptr, out: Ptr, vector: Ptr) -> Ptr {
    let m = |e: &Engine, i: u32| e.mem.f32(this.addr() + 4 * i) as f64;
    let v = |e: &Engine, i: u32| e.mem.f32(vector.addr() + 4 * i) as f64;
    let (v0, v1, v2) = (v(e, 0), v(e, 1), v(e, 2));
    let z = (m(e, 6) * v0 + m(e, 7) * v1 + m(e, 8) * v2) as f32;
    let y = (m(e, 3) * v0 + m(e, 4) * v1 + m(e, 5) * v2) as f32;
    let x = (m(e, 0) * v0 + m(e, 1) * v1 + m(e, 2) * v2) as f32;
    e.call(POINT3_CONSTRUCT, &args![out, x, y, z]);
    out
}

// Translated from 004b45b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMatrix3::Inverse` (Xbox PDB): the inverse of the matrix `this` stored
/// into `out`, which it returns; all zeros when [`ni_matrix3_inverse_ov2`]
/// finds `this` singular.
pub fn ni_matrix3_inverse(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let result = e.mem.alloc(0x24);
    e.call(POINT3_EMPTY_CONSTRUCTOR, &args![result]);
    if !ni_matrix3_inverse_ov2(e, this, Ptr::new(result)) {
        fn_004b4600(e, Ptr::new(result));
    }
    copy_words(e, out.addr(), result, 9);
    e.mem.free(result);
    out
}

// Translated from 004b4600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the matrix (nine floats by rows) to all zeros, column by column
/// through [`fn_004b4660`].
pub fn fn_004b4600(e: &mut Engine, this: Ptr) {
    fn_004b4660(e, this, 0, 0.0, 0.0, 0.0);
    fn_004b4660(e, this, 1, 0.0, 0.0, 0.0);
    fn_004b4660(e, this, 2, 0.0, 0.0, 0.0);
}

// Translated from 004b4660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets column `column` of the matrix (nine floats by rows) to `(x, y, z)`.
pub fn fn_004b4660(e: &mut Engine, this: Ptr, column: u32, x: f32, y: f32, z: f32) {
    let base = this.addr() + 4 * column;
    e.mem.set_f32(base, x);
    e.mem.set_f32(base + 0xc, y);
    e.mem.set_f32(base + 0x18, z);
}

// Translated from 004b46a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMatrix3::Inverse_ov2` (Xbox PDB): writes the adjugate of the matrix
/// `this` (nine floats by rows) into `out`, then divides it by the
/// determinant and returns true; when the determinant's absolute value
/// (`00408820`) is at most `1e-6` returns false with the unscaled adjugate
/// left in `out`.
pub fn ni_matrix3_inverse_ov2(e: &mut Engine, this: Ptr, out: Ptr) -> bool {
    let mut m = [0.0f64; 9];
    for (i, value) in m.iter_mut().enumerate() {
        *value = e.mem.f32(this.addr() + 4 * i as u32) as f64;
    }
    let adjugate = [
        m[4] * m[8] - m[5] * m[7],
        m[2] * m[7] - m[1] * m[8],
        m[1] * m[5] - m[2] * m[4],
        m[5] * m[6] - m[3] * m[8],
        m[0] * m[8] - m[2] * m[6],
        m[2] * m[3] - m[0] * m[5],
        m[3] * m[7] - m[4] * m[6],
        m[1] * m[6] - m[0] * m[7],
        m[0] * m[4] - m[1] * m[3],
    ];
    for (i, value) in adjugate.iter().enumerate() {
        e.mem.set_f32(out.addr() + 4 * i as u32, *value as f32);
    }
    let o = |e: &Engine, i: u32| e.mem.f32(out.addr() + 4 * i) as f64;
    let determinant = (m[0] * o(e, 0) + m[1] * o(e, 3) + m[2] * o(e, 6)) as f32;
    let magnitude = e.call(FLOAT_ABSOLUTE_VALUE, &args![determinant]).f64();
    let limit: f64 = e.global(SINGULAR_DETERMINANT);
    if magnitude <= limit {
        return false;
    }
    let inverse = (1.0 / determinant as f64) as f32;
    for i in 0..9 {
        let scaled = (o(e, i) * inverse as f64) as f32;
        e.mem.set_f32(out.addr() + 4 * i, scaled);
    }
    true
}

// Translated from 004b4910 (decompiled, FalloutNV.exe 1.4.0.525)
/// The first point array of a geometry object in world space: a new array
/// (from `operator new`; the caller frees it) of `00456650(object)`
/// 12-byte points, filled by `004b4a10(count, the object's points
/// (0049ec60), the array, the object's world transform (00461130))`. Null
/// when `object` is null or has no points.
///
/// The game pushes two words (the world transform and the array) before
/// `0049ec60(object)`, which never reads them; they stay on the stack and
/// are the last two arguments of `004b4a10`.
pub fn fn_004b4910(e: &mut Engine, object: Ptr) -> u32 {
    if object.is_null() || e.call(POINT_COUNT, &args![object]).u16() == 0 {
        return 0;
    }
    let count = e.call(POINT_COUNT, &args![object]).u16() as u32;
    let points = new_array(e, 12, count, POINT3_EMPTY_CONSTRUCTOR);
    let transform = e.call(WORLD_TRANSFORM, &args![object]).u32();
    let source = e.call(POINTS_FIRST, &args![object]).u32();
    let count = e.call(POINT_COUNT, &args![object]).u16() as u32;
    e.call(TRANSFORM_POINTS, &args![count, source, points, transform]);
    points
}

// ---------------------------------------------------------------------
// The bhk constructor chain.
// ---------------------------------------------------------------------

// Translated from 004b5040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkMouseSpringAction::bhkMouseSpringAction` (Xbox PDB): the
/// `bhkUnaryAction` constructor, then its own vtable, `Init(arg)` and a
/// count of the objects made (`ms_uiObjects`). Returns `this`.
pub fn bhk_mouse_spring_action_bhk_mouse_spring_action(
    e: &mut Engine,
    this: Ptr<BhkSerializable>,
    arg: u32,
) -> Ptr<BhkSerializable> {
    fn_004b50c0(e, this);
    e.mem.set_u32(this.addr(), BHK_MOUSE_SPRING_ACTION_VTABLE);
    e.call(BHK_MOUSE_SPRING_ACTION_INIT, &args![this, arg]);
    let count: u32 = e.global(BHK_MOUSE_SPRING_ACTION_OBJECT_COUNT);
    e.set_global(BHK_MOUSE_SPRING_ACTION_OBJECT_COUNT, count.wrapping_add(1));
    this
}

// Translated from 004b50c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `bhkUnaryAction` constructor (its vtable's destructor is `004b51e0`,
/// `bhkUnaryAction::_scalar_deleting_destructor_`): the `bhkAction`
/// constructor, the `bhkUnaryAction` vtable and its object count. Returns
/// `this`.
pub fn fn_004b50c0(e: &mut Engine, this: Ptr<BhkSerializable>) -> Ptr<BhkSerializable> {
    fn_004b50f0(e, this);
    e.mem.set_u32(this.addr(), BHK_UNARY_ACTION_VTABLE);
    let count: u32 = e.global(BHK_UNARY_ACTION_OBJECT_COUNT);
    e.set_global(BHK_UNARY_ACTION_OBJECT_COUNT, count.wrapping_add(1));
    this
}

// Translated from 004b50f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `bhkAction` constructor (its vtable's destructor is `004b51a0`,
/// `bhkAction::_scalar_deleting_destructor_`): the `bhkSerializable`
/// constructor, the `bhkAction` vtable and its object count. Returns
/// `this`.
pub fn fn_004b50f0(e: &mut Engine, this: Ptr<BhkSerializable>) -> Ptr<BhkSerializable> {
    fn_004b5120(e, this);
    e.mem.set_u32(this.addr(), BHK_ACTION_VTABLE);
    let count: u32 = e.global(BHK_ACTION_OBJECT_COUNT);
    e.set_global(BHK_ACTION_OBJECT_COUNT, count.wrapping_add(1));
    this
}

// Translated from 004b5120 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `bhkSerializable` constructor (its vtable's destructor is
/// `004b5160`): the `bhkRefObject` constructor, the `bhkSerializable`
/// vtable and a null `pInfo`. Returns `this`.
pub fn fn_004b5120(e: &mut Engine, this: Ptr<BhkSerializable>) -> Ptr<BhkSerializable> {
    e.call(BHK_REF_OBJECT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), BHK_SERIALIZABLE_VTABLE);
    e.set(this, BhkSerializable::pInfo, 0);
    this
}

// Translated from 004b5150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkSerializable::GetRTTI` (Xbox PDB): the address of its `NiRTTI`.
pub fn bhk_serializable_get_rtti(_e: &mut Engine, _this: Ptr) -> u32 {
    BHK_SERIALIZABLE_RTTI
}

// Translated from 004b5160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkSerializable::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor (`00c877b0`) and, when bit 0 of `flags` is set, frees the
/// 0x10-byte object. Returns `this`.
pub fn bhk_serializable_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(BHK_SERIALIZABLE_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x10u32]);
    }
    this
}

// Translated from 004b5190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkAction::GetRTTI` (Xbox PDB): the address of its `NiRTTI`.
pub fn bhk_action_get_rtti(_e: &mut Engine, _this: Ptr) -> u32 {
    BHK_ACTION_RTTI
}

// Translated from 004b51a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkAction::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor (`00c87970`) and, when bit 0 of `flags` is set, frees the
/// 0x10-byte object. Returns `this`.
pub fn bhk_action_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(BHK_ACTION_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x10u32]);
    }
    this
}

// Translated from 004b51d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkUnaryAction::GetRTTI` (Xbox PDB): the address of its `NiRTTI`.
pub fn bhk_unary_action_get_rtti(_e: &mut Engine, _this: Ptr) -> u32 {
    BHK_UNARY_ACTION_RTTI
}

// Translated from 004b51e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkUnaryAction::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor (`00c86720`) and, when bit 0 of `flags` is set, frees the
/// 0x10-byte object. Returns `this`.
pub fn bhk_unary_action_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(BHK_UNARY_ACTION_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x10u32]);
    }
    this
}

// ---------------------------------------------------------------------
// Second session: `bhkMouseSpringAction`'s last virtual functions, the
// collision object lookups, the collidable accessors, the scene graph
// walks and the world bound against frustum test.
// ---------------------------------------------------------------------

/// `bhkMouseSpringAction::~bhkMouseSpringAction` (Xbox PDB), `(this)`.
const BHK_MOUSE_SPRING_ACTION_DESTRUCT: u32 = 0x00c8_6cb0;
/// `bhkCollisionObject::GetbhkCollisionObject(object)` (Xbox PDB, cdecl):
/// the collision object of a scene graph object, or 0.
const GET_BHK_COLLISION_OBJECT: u32 = 0x0043_b610;
/// `006fa820(this)`: what the code uses as the owner of a collision object
/// (an object `004b5400` can measure).
const COLLISION_OBJECT_OWNER: u32 = 0x006f_a820;
/// `004ae750(this)` (the engine map names it
/// `bhkCharacterProxy::operatorP`): a pointer to the body of an object, or
/// 0. `004b4ea0(body)` (ST0) is the number `004b5400` reports for it.
const BODY_OF_OBJECT: u32 = 0x004a_e750;
const BODY_VALUE: u32 = 0x004b_4ea0;
/// `004a7290(vector)` (ST0): the squared length of a vector.
const VECTOR_LENGTH_SQUARED: u32 = 0x004a_7290;
/// `NiASin`'s body for arguments strictly between `-1` and `1`
/// (`004b5510(x)`, cdecl, result in ST0); the `-1.0` (`double`) it compares
/// with; the `float` it returns at the limits (zero in the exe file, so the
/// game writes it at run time).
const NI_ASIN_BODY: u32 = 0x004b_5510;
const NEGATIVE_ONE_DOUBLE: u32 = 0x0101_a6b0;
const ASIN_LIMIT: u32 = 0x011c_627c;
/// `0.5` (`float`): the fourth component of the tint colours.
const TINT_ALPHA: u32 = 0x0101_6248;
/// `0044ddc0(this)` reads the word at `this + 8`; `0043b540(this)` is
/// `0044ddc0(this + 0x14)`.
const WORD_AT_EIGHT: u32 = 0x0044_ddc0;
const COLLIDABLE_WORD: u32 = 0x0043_b540;
/// `008c71b0(this, value)` stores a word in a one word value (the engine
/// map's name for it is wrong) and returns `this`; `004a3a20(this)` gives
/// the high half of that word.
const STORE_WORD: u32 = 0x008c_71b0;
const HIGH_HALF: u32 = 0x004a_3a20;
/// `004b4ec0(body)`: the value `00458620(out, value)` (cdecl) turns into a
/// position; `00457620(finder, position)` finds the cell holding a
/// position (the finder is the singleton at `011dea10`); `009611e0(cell)`
/// reads the word at `cell + 0x18`.
const BODY_POSITION_SOURCE: u32 = 0x004b_4ec0;
const POSITION_FROM_VALUE: u32 = 0x0045_8620;
const CELL_AT_POSITION: u32 = 0x0045_7620;
const WORD_AT_0X18: u32 = 0x0096_11e0;
const CELL_FINDER: u32 = 0x011d_ea10;
/// `bhkWorldObject::GetAVObject(this, flags)` (Xbox PDB).
const GET_AV_OBJECT: u32 = 0x00c8_5c80;
/// The checked cast `00653270(class key, object)` (cdecl): the object when
/// it is of that class, else 0.
const DYNAMIC_CAST: u32 = 0x0065_3270;
const CAST_CLASS_012043F8: u32 = 0x0120_43f8;
const CAST_CLASS_01268168: u32 = 0x0126_8168;
/// `bhkWorldObject::GetProperty(out, object, key)` (Xbox PDB, cdecl) with
/// its key (`01267b70`); `00559450(this)` reads a smart pointer.
const WORLD_OBJECT_GET_PROPERTY: u32 = 0x00c8_5ef0;
const WORLD_OBJECT_PROPERTY_KEY: u32 = 0x0126_7b70;
const SMART_POINTER_GET: u32 = 0x0055_9450;
/// `0084e3a0(this)`.
const FN_0084E3A0: u32 = 0x0084_e3a0;
/// Names: `00413f40(object)` gives the object's name holder and
/// `0043b1b0(holder)` the text (0 when there is none). `00404dc0(a, b)`
/// (0 on a match) and `00408b20(a, b)` are string compares (cdecl),
/// `00ec7ec0` the CRT `_strnicmp(a, b, count)`.
const NAME_HOLDER: u32 = 0x0041_3f40;
const NAME_TEXT: u32 = 0x0043_b1b0;
const STRING_COMPARE: u32 = 0x0040_4dc0;
const STRING_COMPARE_SECOND: u32 = 0x0040_8b20;
const STRING_COMPARE_NO_CASE: u32 = 0x00ec_7ec0;
/// `"Scb"`, `"FadeNode "`, `"EditorMarker"` and `"Arrow"`.
const SCABBARD_NAME: u32 = 0x0101_fa04;
const FADE_NODE_PREFIX: u32 = 0x0102_0300;
const EDITOR_MARKER_NAME: u32 = 0x0102_030c;
const ARROW_NAME: u32 = 0x0102_031c;
/// `NiObjectNET::GetController(this, class key)` (Xbox PDB) with the two
/// class keys used here, and `NiObjectNET::GetExtraData(this, name)`
/// (Xbox PDB); `00448a80()` is the name asked for and `00448a60(this,
/// 0x20)` a test of what was found.
const GET_CONTROLLER: u32 = 0x00a5_c570;
const MORPHER_CONTROLLER_CLASS: u32 = 0x011f_3728;
const MARKER_CONTROLLER_CLASS: u32 = 0x011f_36ac;
const GET_EXTRA_DATA: u32 = 0x00a5_bdd0;
const EXTRA_DATA_NAME: u32 = 0x0044_8a80;
const EXTRA_DATA_TEST: u32 = 0x0044_8a60;
/// `0043b300(class key, object)` (cdecl, bool): whether the object is of
/// that class.
const IS_OF_CLASS: u32 = 0x0043_b300;
const IS_OF_CLASS_KEY_012024E0: u32 = 0x0120_24e0;
const IS_OF_CLASS_KEY_0126817C: u32 = 0x0126_817c;
const IS_OF_CLASS_KEY_011F9140: u32 = 0x011f_9140;
/// `00537bd0(controller)`.
const FN_00537BD0: u32 = 0x0053_7bd0;
/// What `RemoveEditorMarkers` remembers between calls: the root it found
/// the marker data on and a value taken from its controller; and the four
/// words `004b5f60` to `004b5f90` return.
const MARKER_ROOT: u32 = 0x011c_6328;
const MARKER_CONTROLLER: u32 = 0x011c_6324;
const MARKER_WORD_A: u32 = 0x011c_61f0;
const MARKER_WORD_B: u32 = 0x011c_61f4;
const MARKER_WORD_C: u32 = 0x011c_61f8;
const MARKER_WORD_D: u32 = 0x011c_61fc;
/// The frustum test: the six planes (an `NiFrustumPlanes`, constructor
/// `0045c620(this)`) at `011c6330`; the guard bit of the function local
/// `static` is bit 0 of `011c6394`; the camera the planes were made for
/// (`011c632c`); `00a755d0(planes, camera)` sets them; the stamp
/// `00825c00(011f6394)` of the camera state they were made for is kept at
/// `011890e0`; `004b4cf0(planes, index)` is a pointer to a plane's four
/// floats.
const FRUSTUM_PLANES: u32 = 0x011c_6330;
const FRUSTUM_GUARD: u32 = 0x011c_6394;
const FRUSTUM_CAMERA: u32 = 0x011c_632c;
const FRUSTUM_PLANES_CONSTRUCT: u32 = 0x0045_c620;
const FRUSTUM_PLANES_SET: u32 = 0x00a7_55d0;
const CAMERA_STAMP_OWNER: u32 = 0x011f_6394;
const CAMERA_STAMP: u32 = 0x0082_5c00;
const FRUSTUM_STAMP: u32 = 0x0118_90e0;
const FRUSTUM_PLANE: u32 = 0x004b_4cf0;
/// `NiPoint3::Dot(this, other)` (Xbox PDB), result in ST0.
const POINT3_DOT: u32 = 0x004b_6190;
/// Tinting: the smart pointer on the stack (`00633c90(this, pointer)` sets
/// it, `0066b0d0(this, pointer)` assigns, `0045cec0(this)` releases), the
/// 0x74-byte effect data (`00b668b0(this)` constructs it), the constructor
/// of a value of four floats `00414430(this, x, y, z, w)` (returns `this`),
/// `NiAVObject::GetProperty(this, kind)` (Xbox PDB) and
/// `BSShaderPPLightingProperty::SetTextureEffectData(this, data)`
/// (Xbox PDB).
const SMART_POINTER_CONSTRUCT: u32 = 0x0063_3c90;
const SMART_POINTER_ASSIGN: u32 = 0x0066_b0d0;
const SMART_POINTER_RELEASE: u32 = 0x0045_cec0;
const EFFECT_DATA_CONSTRUCT: u32 = 0x00b6_68b0;
const COLOUR_CONSTRUCT: u32 = 0x0041_4430;
const GET_PROPERTY: u32 = 0x00a5_9d30;
const SET_TEXTURE_EFFECT_DATA: u32 = 0x00b6_75c0;
const CAST_CLASS_011FA010: u32 = 0x011f_a010;
/// The scene graph walker `00c68900(root, context, callback)` (cdecl).
const WALK_SCENE_GRAPH: u32 = 0x00c6_8900;
/// The walker callbacks of this file: `004b6740` counts, `004b68a0` finds.
const COUNT_CALLBACK: u32 = 0x004b_6740;
const FIND_CALLBACK: u32 = 0x004b_68a0;

// Translated from 004b5210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkMouseSpringAction::GetRTTI` (Xbox PDB): the address of its `NiRTTI`.
pub fn bhk_mouse_spring_action_get_rtti(_e: &mut Engine, _this: Ptr) -> u32 {
    BHK_MOUSE_SPRING_ACTION_RTTI
}

// Translated from 004b5220 (decompiled, FalloutNV.exe 1.4.0.525)
/// A virtual function of `bhkMouseSpringAction`'s vtable (`0101ffcc`) that
/// returns the constant `0x40`; the slot is not named.
pub fn fn_004b5220(_e: &mut Engine, _this: Ptr) -> u32 {
    0x40
}

// Translated from 004b5230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkMouseSpringAction::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor (`00c86cb0`) and, when bit 0 of `flags` is set, frees the
/// 0x10-byte object. Returns `this`.
pub fn bhk_mouse_spring_action_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(BHK_MOUSE_SPRING_ACTION_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x10u32]);
    }
    this
}

/// The object's children container (virtual slot `0xc` of the scene graph
/// object, 0 for a leaf).
fn children_of(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0xc, &[]).u32()
}

/// The text of the object's name (0 when it has none).
fn name_of(e: &mut Engine, object: Ptr) -> u32 {
    let holder = e.call(NAME_HOLDER, &args![object]).u32();
    e.call(NAME_TEXT, &args![holder]).u32()
}

// Translated from 004b5260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FindFirstCollisionObject` (Xbox PDB): the object's own collision
/// object, or else the first one found by searching its children in order
/// (recursively); 0 when there is none.
pub fn find_first_collision_object(e: &mut Engine, object: Ptr) -> u32 {
    if object.is_null() {
        return 0;
    }
    let own = e.call(GET_BHK_COLLISION_OBJECT, &args![object]).u32();
    if own != 0 {
        return own;
    }
    let children = children_of(e, object);
    if children == 0 {
        return 0;
    }
    let count = e.call(CHILD_COUNT, &args![children]).u32();
    for i in 0..count {
        let child = e.call(CHILD_AT, &args![children, i]).u32();
        let found = find_first_collision_object(e, Ptr::new(child));
        if found != 0 {
            return found;
        }
    }
    0
}

// Translated from 004b52f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Among the object's own collision object (as `006fa820` turns it into
/// the object `004b5400` measures) and the ones found under its children
/// (recursively, in order), the one with the greatest `004b5400` value; the
/// first one found wins ties. 0 when there is none (or `object` is null).
pub fn fn_004b52f0(e: &mut Engine, object: Ptr) -> u32 {
    if object.is_null() {
        return 0;
    }
    let mut best = 0u32;
    let children = children_of(e, object);
    if children != 0 {
        let count = e.call(CHILD_COUNT, &args![children]).u32();
        for i in 0..count {
            let child = e.call(CHILD_AT, &args![children, i]).u32();
            let found = fn_004b52f0(e, Ptr::new(child));
            if found != 0 {
                if best != 0 {
                    let best_value = fn_004b5400(e, Ptr::new(best));
                    let found_value = fn_004b5400(e, Ptr::new(found));
                    // Not greater (or unordered): keep the one already held.
                    if !(found_value > best_value) {
                        continue;
                    }
                }
                best = found;
            }
        }
    }
    let collision = e.call(GET_BHK_COLLISION_OBJECT, &args![object]).u32();
    let own = if collision != 0 {
        e.call(COLLISION_OBJECT_OWNER, &args![collision]).u32()
    } else {
        0
    };
    if best != 0 {
        if own == 0 {
            return best;
        }
        let own_value = fn_004b5400(e, Ptr::new(own));
        let best_value = fn_004b5400(e, Ptr::new(best));
        if best_value > own_value {
            return best;
        }
    }
    own
}

// Translated from 004b5400 (decompiled, FalloutNV.exe 1.4.0.525)
/// The number `004b4ea0` reports for the body `004ae750` finds on `this`
/// (`0.0` when it has no body), as a `float`.
pub fn fn_004b5400(e: &mut Engine, this: Ptr) -> f32 {
    let body = e.call(BODY_OF_OBJECT, &args![this]).u32();
    if body == 0 {
        0.0
    } else {
        e.call(BODY_VALUE, &args![body]).f32()
    }
}

// Translated from 004b5440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Compares the distance between two points with `radius` (see
/// [`fn_004b5470`]): builds `this - other` in a local and returns that
/// function's answer (the compiler leaves it in EAX; the decompiler calls
/// the function `void`).
pub fn fn_004b5440(e: &mut Engine, this: Ptr, other: Ptr, radius: f32) -> i32 {
    e.with_stack(12, |e, difference| {
        let vector = e
            .call(POINT3_SUBTRACT, &args![this, difference, other])
            .u32();
        fn_004b5470(e, Ptr::new(vector), radius)
    })
}

// Translated from 004b5470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Compares the squared length of a vector (`004a7290`, stored as a
/// `float`) with `radius * radius` (also a `float`): `-1` when the radius
/// is the larger, `1` when the vector is, `0` when they are equal or
/// unordered.
pub fn fn_004b5470(e: &mut Engine, vector: Ptr, radius: f32) -> i32 {
    let length_squared = e.call(VECTOR_LENGTH_SQUARED, &args![vector]).f32();
    let radius_squared = (radius as f64 * radius as f64) as f32;
    if radius_squared > length_squared {
        -1
    } else if radius_squared < length_squared {
        1
    } else {
        0
    }
}

// Translated from 004b54c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiASin` (Xbox PDB): the arc sine; at or below `-1` (or unordered) the
/// negated limit constant (`011c627c`), at or above `1` the limit, in
/// between `004b5510`. The result is in ST0.
pub fn ni_a_sin(e: &mut Engine, x: f32) -> f64 {
    let negative_one: f64 = e.global(NEGATIVE_ONE_DOUBLE);
    let one: f64 = e.global(ONE);
    let limit: f32 = e.global(ASIN_LIMIT);
    if !(x as f64 > negative_one) {
        -(limit as f64)
    } else if (x as f64) < one {
        e.call(NI_ASIN_BODY, &args![x]).f64()
    } else {
        limit as f64
    }
}

// Translated from 004b5820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GetAVObjectForCollidable` (Xbox PDB): the scene graph object that a
/// Havok collidable belongs to, or 0.
///
/// The collidable is read as a body (`004b59f0`, handle type 1) or as a
/// phantom (`004b5950`, handle type 2). For a body: the object `004b5a20`
/// derives from it (`0044ddc0` of that); when there is none and the high
/// half of the collidable's word (`0043b540`) is 1, the cell holding the
/// body's position and that cell's word at `+0x18` (`009611e0`). For a
/// phantom: `004b5ad0` and then `004b5a80` give the candidates.
pub fn get_av_object_for_collidable(e: &mut Engine, collidable: Ptr) -> u32 {
    if collidable.is_null() {
        return 0;
    }
    let mut result = 0u32;
    let body = fn_004b59f0(e, collidable);
    let phantom = if body == 0 {
        fn_004b5950(e, collidable)
    } else {
        0
    };
    if body != 0 {
        let from_body = fn_004b5a20(e, Ptr::new(body));
        if from_body != 0 {
            return e.call(WORD_AT_EIGHT, &args![from_body]).u32();
        }
        let word = e.call(COLLIDABLE_WORD, &args![collidable]).u32();
        let half = e.with_stack(4, |e, value| {
            let value = e.call(STORE_WORD, &args![value, word]).u32();
            e.call(HIGH_HALF, &args![value]).u32()
        });
        if half == 1 {
            e.with_stack(12, |e, position| {
                e.call(POINT3_EMPTY_CONSTRUCTOR, &args![position]);
                let source = e.call(BODY_POSITION_SOURCE, &args![body]).u32();
                e.call(POSITION_FROM_VALUE, &args![position, source]);
                let finder: u32 = e.global(CELL_FINDER);
                let cell = e.call(CELL_AT_POSITION, &args![finder, position]).u32();
                if cell != 0 && e.call(WORD_AT_0X18, &args![cell]).u32() != 0 {
                    result = e.call(WORD_AT_0X18, &args![cell]).u32();
                }
            });
        }
        return result;
    }
    let candidate = fn_004b5ad0(e, Ptr::new(phantom));
    if candidate != 0 {
        result = e.call(WORD_AT_EIGHT, &args![candidate]).u32();
    }
    if result == 0 {
        let world_object = fn_004b5a80(e, Ptr::new(phantom));
        if world_object != 0 {
            result = e.call(GET_AV_OBJECT, &args![world_object, 0u32]).u32();
        }
    }
    result
}

// Translated from 004b5950 (decompiled, FalloutNV.exe 1.4.0.525)
/// The collidable's owner when its broad phase handle type is 2
/// (`004b5980`), else 0.
pub fn fn_004b5950(e: &mut Engine, collidable: Ptr) -> u32 {
    if fn_004b5980(e, collidable) == 2 {
        fn_004b59c0(e, collidable)
    } else {
        0
    }
}

// Translated from 004b5980 (decompiled, FalloutNV.exe 1.4.0.525)
/// The broad phase handle type of a collidable: [`fn_004b59a0`] of the
/// handle at `+0x14`.
pub fn fn_004b5980(e: &mut Engine, this: Ptr) -> i32 {
    fn_004b59a0(e, Ptr::new(this.addr().wrapping_add(0x14)))
}

// Translated from 004b59a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The signed byte at `+4` of a broad phase handle (its type).
pub fn fn_004b59a0(e: &mut Engine, this: Ptr) -> i32 {
    e.mem.i8(this.addr().wrapping_add(4)) as i32
}

// Translated from 004b59c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// [`fn_004b59d0`] on its argument.
pub fn fn_004b59c0(e: &mut Engine, collidable: Ptr) -> u32 {
    fn_004b59d0(e, collidable)
}

// Translated from 004b59d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The owner of a collidable: `this` plus the signed byte at `+0x10`.
pub fn fn_004b59d0(e: &mut Engine, this: Ptr) -> u32 {
    let offset = e.mem.i8(this.addr().wrapping_add(0x10)) as i32;
    this.addr().wrapping_add(offset as u32)
}

// Translated from 004b59f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The collidable's owner when its broad phase handle type is 1
/// (`004b5980`), else 0.
pub fn fn_004b59f0(e: &mut Engine, collidable: Ptr) -> u32 {
    if fn_004b5980(e, collidable) == 1 {
        fn_004b59c0(e, collidable)
    } else {
        0
    }
}

// Translated from 004b5a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The checked cast (`00653270`, class `012043f8`) of what [`fn_004b5a50`]
/// finds for the object.
pub fn fn_004b5a20(e: &mut Engine, object: Ptr) -> u32 {
    let property = fn_004b5a50(e, object);
    e.call(DYNAMIC_CAST, &args![CAST_CLASS_012043F8, property])
        .u32()
}

// Translated from 004b5a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorldObject::GetProperty` (Xbox PDB) with the key `01267b70`,
/// through `00559450` (the smart pointer read).
pub fn fn_004b5a50(e: &mut Engine, object: Ptr) -> u32 {
    e.with_stack(8, |e, out| {
        let property = e
            .call(
                WORLD_OBJECT_GET_PROPERTY,
                &args![out, object, WORLD_OBJECT_PROPERTY_KEY],
            )
            .u32();
        e.call(SMART_POINTER_GET, &args![property]).u32()
    })
}

// Translated from 004b5a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// [`fn_004b5aa0`] on its argument.
pub fn fn_004b5a80(e: &mut Engine, object: Ptr) -> u32 {
    fn_004b5aa0(e, object)
}

// Translated from 004b5aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `0084e3a0(object)`, or 0 for a null object.
pub fn fn_004b5aa0(e: &mut Engine, object: Ptr) -> u32 {
    if object.is_null() {
        0
    } else {
        e.call(FN_0084E3A0, &args![object]).u32()
    }
}

// Translated from 004b5ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Like [`fn_004b5a20`] with the cast class `01268168`.
pub fn fn_004b5ad0(e: &mut Engine, object: Ptr) -> u32 {
    let property = fn_004b5a50(e, object);
    e.call(DYNAMIC_CAST, &args![CAST_CLASS_01268168, property])
        .u32()
}

// Translated from 004b5b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Element `index` of the array of 0x30-byte elements at `this + 0x34`.
pub fn fn_004b5b00(e: &mut Engine, this: Ptr, index: u16) -> u32 {
    let base = e.mem.u32(this.addr().wrapping_add(0x34));
    (index as u32).wrapping_mul(0x30).wrapping_add(base)
}

// Translated from 004b5b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RemoveScabard` (Xbox PDB): looks through the children of the node for
/// one named `"Scb"` (removed through virtual slot `0xf0` of the node,
/// given its index) or, failing that, a name starting with `"FadeNode "`,
/// into whose children it descends, returning that search's answer. `true`
/// when one was removed.
pub fn remove_scabard(e: &mut Engine, node: Ptr) -> bool {
    if node.is_null() {
        return false;
    }
    let mut index = 0u32;
    while index < e.call(CHILD_COUNT, &args![node]).u32() {
        let child = e.call(CHILD_AT, &args![node, index]).u32();
        if child != 0 {
            let name = name_of(e, Ptr::new(child));
            if name != 0 {
                if e.call(STRING_COMPARE, &args![name, SCABBARD_NAME]).u32() == 0 {
                    e.vcall(node.addr(), 0xf0, &args![index]);
                    return true;
                }
                if e.call(STRING_COMPARE_NO_CASE, &args![name, FADE_NODE_PREFIX, 9u32])
                    .u32()
                    == 0
                {
                    let below = children_of(e, Ptr::new(child));
                    return remove_scabard(e, Ptr::new(below));
                }
            }
        }
        index += 1;
    }
    false
}

// Translated from 004b5bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HasMorpherController` (Xbox PDB): whether the object has a controller
/// of the morpher class, or (through its children, recursively) any object
/// below it does.
pub fn has_morpher_controller(e: &mut Engine, object: Ptr) -> bool {
    if object.is_null() {
        return false;
    }
    if e.call(GET_CONTROLLER, &args![object, MORPHER_CONTROLLER_CLASS])
        .u32()
        != 0
    {
        return true;
    }
    let children = children_of(e, object);
    if children != 0 {
        let mut index = 0u32;
        while index < e.call(CHILD_COUNT, &args![children]).u32() {
            let child = e.call(CHILD_AT, &args![children, index]).u32();
            if has_morpher_controller(e, Ptr::new(child)) {
                return true;
            }
            index += 1;
        }
    }
    false
}

// Translated from 004b5c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the object is of the class `012024e0` (`0043b300`) or any object
/// below it is (through its children, recursively).
pub fn fn_004b5c80(e: &mut Engine, object: Ptr) -> bool {
    if object.is_null() {
        return false;
    }
    if e.call(IS_OF_CLASS, &args![IS_OF_CLASS_KEY_012024E0, object])
        .bool()
    {
        return true;
    }
    let children = children_of(e, object);
    if children != 0 {
        let mut index = 0u32;
        while index < e.call(CHILD_COUNT, &args![children]).u32() {
            let child = e.call(CHILD_AT, &args![children, index]).u32();
            if fn_004b5c80(e, Ptr::new(child)) {
                return true;
            }
            index += 1;
        }
    }
    false
}

/// Forgets the remembered marker root (and its controller value) when it
/// is `node`.
fn forget_marker_root(e: &mut Engine, node: Ptr) {
    let root: u32 = e.global(MARKER_ROOT);
    if root == node.addr() {
        e.set_global(MARKER_ROOT, 0u32);
        e.set_global(MARKER_CONTROLLER, 0u32);
    }
}

// Translated from 004b5d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RemoveEditorMarkers` (Xbox PDB): while no root is remembered
/// (`011c6328`), takes the node as the root when it has the marker extra
/// data (`GetExtraData`, then `004b5fa0`), also remembering a value from
/// its controller (`011c6324`). With a root: a node named `"EditorMarker"`
/// sends the four words of [`fn_004b5f60`] to [`fn_004b5f90`] to the
/// remembered controller value (virtual slot `0x90`), asks the object at
/// `+0x18` (`009611e0`; virtual slot `0xe8`) to remove the node and returns
/// `true`; otherwise it searches the node's children (their own `+0xc`
/// containers) the same way. The remembered root is dropped when the call
/// that set it returns.
pub fn remove_editor_markers(e: &mut Engine, node: Ptr) -> bool {
    let mut found = false;
    if node.is_null() {
        return found;
    }
    if e.global::<u32>(MARKER_ROOT) == 0 {
        let name = e.call(EXTRA_DATA_NAME, &[]).u32();
        let extra = e.call(GET_EXTRA_DATA, &args![node, name]).u32();
        if extra != 0 && fn_004b5fa0(e, Ptr::new(extra)) {
            e.set_global(MARKER_ROOT, node.addr());
            let controller = e
                .call(GET_CONTROLLER, &args![node, MARKER_CONTROLLER_CLASS])
                .u32();
            if controller != 0 {
                let value = e.call(FN_00537BD0, &args![controller]).u32();
                e.set_global(MARKER_CONTROLLER, value);
            }
        }
    }
    if e.global::<u32>(MARKER_ROOT) == 0 {
        return found;
    }
    if name_of(e, node) != 0 {
        let name = name_of(e, node);
        if e.call(STRING_COMPARE, &args![name, EDITOR_MARKER_NAME])
            .u32()
            == 0
        {
            if e.global::<u32>(MARKER_CONTROLLER) != 0 {
                // 004b5de5..004b5e5b: the 0 pushed before each getter (which takes no
                // arguments) is the slot call's second word: slot 0x90 pops two (`RET 8`,
                // `NiDefaultAVObjectPalette::SetAVObject` 00a2ee40), the name the getter
                // answers and a null object, which removes that name.
                let getters: [fn(&mut Engine) -> u32; 4] =
                    [fn_004b5f60, fn_004b5f70, fn_004b5f80, fn_004b5f90];
                for getter in getters {
                    let value = getter(e);
                    let controller: u32 = e.global(MARKER_CONTROLLER);
                    e.vcall(controller, 0x90, &args![value, 0u32]);
                }
            }
            if e.call(WORD_AT_0X18, &args![node]).u32() != 0 {
                let parent = e.call(WORD_AT_0X18, &args![node]).u32();
                e.vcall(parent, 0xe8, &args![node]);
            }
            forget_marker_root(e, node);
            return true;
        }
    }
    let mut index = 0u32;
    while index < e.call(CHILD_COUNT, &args![node]).u32() {
        let child = e.call(CHILD_AT, &args![node, index]).u32();
        if child != 0 {
            let below = children_of(e, Ptr::new(child));
            if below != 0 {
                found = remove_editor_markers(e, Ptr::new(below));
                if found {
                    forget_marker_root(e, node);
                    return found;
                }
            }
        }
        index += 1;
    }
    forget_marker_root(e, node);
    found
}

// Translated from 004b5f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `011c61f0`.
pub fn fn_004b5f60(e: &mut Engine) -> u32 {
    e.global(MARKER_WORD_A)
}

// Translated from 004b5f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `011c61f4`.
pub fn fn_004b5f70(e: &mut Engine) -> u32 {
    e.global(MARKER_WORD_B)
}

// Translated from 004b5f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `011c61f8`.
pub fn fn_004b5f80(e: &mut Engine) -> u32 {
    e.global(MARKER_WORD_C)
}

// Translated from 004b5f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `011c61fc`.
pub fn fn_004b5f90(e: &mut Engine) -> u32 {
    e.global(MARKER_WORD_D)
}

// Translated from 004b5fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `00448a60(this, 0x20)` is non-zero.
pub fn fn_004b5fa0(e: &mut Engine, this: Ptr) -> bool {
    e.call(EXTRA_DATA_TEST, &args![this, 0x20u32]).u32() != 0
}

// Translated from 004b5fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the world bound of `object` is not entirely outside the frustum
/// of `camera`: [`fn_004b5ff0`] on the bound `NiAVObject::GetWorldBound`
/// gives; `false` for a null object.
pub fn fn_004b5fc0(e: &mut Engine, object: Ptr, camera: Ptr) -> bool {
    if object.is_null() {
        return false;
    }
    let bound = e.call(GET_WORLD_BOUND, &args![object]).u32();
    fn_004b5ff0(e, Ptr::new(bound), camera)
}

// Translated from 004b5ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the bound (centre and radius, four floats) is not entirely
/// outside any of the six frustum planes of `camera`. The planes are kept
/// in a global `NiFrustumPlanes` (constructed on the first call) and are
/// recomputed (`00a755d0`) when the camera differs from the one they were
/// made for or the camera state stamp (`00825c00`) changed. `false` for a
/// null camera. The compiler's exception frame is not translated.
pub fn fn_004b5ff0(e: &mut Engine, bound: Ptr, camera: Ptr) -> bool {
    if camera.is_null() {
        return false;
    }
    let guard: u32 = e.global(FRUSTUM_GUARD);
    if guard & 1 == 0 {
        e.set_global(FRUSTUM_GUARD, guard | 1);
        e.call(FRUSTUM_PLANES_CONSTRUCT, &args![FRUSTUM_PLANES]);
    }
    let mut current = false;
    if e.global::<u32>(FRUSTUM_CAMERA) == camera.addr() {
        let stamp = e.call(CAMERA_STAMP, &args![CAMERA_STAMP_OWNER]).u32();
        current = e.global::<u32>(FRUSTUM_STAMP) == stamp;
    }
    if !current {
        e.call(FRUSTUM_PLANES_SET, &args![FRUSTUM_PLANES, camera]);
        let stamp = e.call(CAMERA_STAMP, &args![CAMERA_STAMP_OWNER]).u32();
        e.set_global(FRUSTUM_STAMP, stamp);
        e.set_global(FRUSTUM_CAMERA, camera.addr());
    }
    for index in 0..6u32 {
        let plane = e.call(FRUSTUM_PLANE, &args![FRUSTUM_PLANES, index]).u32();
        let side = e.with_stack(16, |e, copy| {
            for word in 0..4 {
                let value = e.mem.u32(plane + 4 * word);
                e.mem.set_u32(copy.addr() + 4 * word, value);
            }
            ni_bound_which_side(e, bound, copy)
        });
        if side == 2 {
            return false;
        }
    }
    true
}

// Translated from 004b6100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBound::WhichSide` (Xbox PDB): the signed distance of the bound's
/// centre from the plane ([`fn_004b6160`]) against the bound's radius
/// (`+0xc`): `2` when the centre is at least the radius behind the plane,
/// `1` when it is at least the radius in front, else `0` (also for NaN).
pub fn ni_bound_which_side(e: &mut Engine, this: Ptr, plane: Ptr) -> i32 {
    let distance = fn_004b6160(e, plane, this);
    let radius = e.mem.f32(this.addr().wrapping_add(0xc));
    if distance <= -radius {
        2
    } else if distance >= radius {
        1
    } else {
        0
    }
}

// Translated from 004b6160 (decompiled, FalloutNV.exe 1.4.0.525)
/// The signed distance of a point from a plane: `NiPoint3::Dot` of the
/// plane's normal (`this`) with the point (`004b6190`) minus the plane's
/// constant (`+0xc`), stored as a `float`.
pub fn fn_004b6160(e: &mut Engine, this: Ptr, point: Ptr) -> f32 {
    let dot = e.call(POINT3_DOT, &args![this, point]).f64();
    let constant = e.mem.f32(this.addr().wrapping_add(0xc));
    (dot - constant as f64) as f32
}

// Translated from 004b6360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TintScenegraph` (Xbox PDB): gives every lighting property found in the
/// scene graph below `node` (`NiAVObject::GetProperty(3)`, then the checked
/// cast to class `011fa010`) one shared texture effect data object, set
/// with `SetTextureEffectData`. When `effect_data` is null the object is
/// made first: a 0x74-byte object whose four colours (at `+0x2c`, `+0x3c`,
/// `+0xc` and `+0x1c`, in that order) are all `colour` (three floats) with
/// alpha `0.5`, and whose words `+0x5c`, `+0x60`, `+0x64` and `+0x68` are 9,
/// 10, 1 and 4. The object is held by a smart pointer that lives for the
/// call (the children are passed the same object). The compiler's
/// exception frame is not translated.
pub fn tint_scenegraph(e: &mut Engine, node: Ptr, colour: Ptr, effect_data: Ptr) {
    e.with_stack(4, |e, holder| {
        e.call(SMART_POINTER_CONSTRUCT, &args![holder, effect_data]);
        let mut effect_data = effect_data;
        if effect_data.is_null() {
            let memory = e.call(NI_OPERATOR_NEW, &args![0x74u32]).u32();
            let object = if memory != 0 {
                e.call(EFFECT_DATA_CONSTRUCT, &args![memory]).u32()
            } else {
                0
            };
            e.call(SMART_POINTER_ASSIGN, &args![holder, object]);
            effect_data = Ptr::new(e.call(SMART_POINTER_GET, &args![holder]).u32());
            for offset in [0x2cu32, 0x3c, 0x0c, 0x1c] {
                let x = e.mem.f32(colour.addr());
                let y = e.mem.f32(colour.addr().wrapping_add(4));
                let z = e.mem.f32(colour.addr().wrapping_add(8));
                let alpha: f32 = e.global(TINT_ALPHA);
                e.with_stack(16, |e, temporary| {
                    let made = e
                        .call(COLOUR_CONSTRUCT, &args![temporary, x, y, z, alpha])
                        .u32();
                    let target = e.call(SMART_POINTER_GET, &args![holder]).u32();
                    for word in 0..4 {
                        let value = e.mem.u32(made + 4 * word);
                        e.mem.set_u32(target + offset + 4 * word, value);
                    }
                });
            }
            for (offset, value) in [(0x5cu32, 9u32), (0x60, 10), (0x64, 1), (0x68, 4)] {
                let target = e.call(SMART_POINTER_GET, &args![holder]).u32();
                e.mem.set_u32(target + offset, value);
            }
        }
        let property = e.call(GET_PROPERTY, &args![node, 3u32]).u32();
        let lighting = e
            .call(DYNAMIC_CAST, &args![CAST_CLASS_011FA010, property])
            .u32();
        if lighting != 0 {
            let data = e.call(SMART_POINTER_GET, &args![holder]).u32();
            e.call(SET_TEXTURE_EFFECT_DATA, &args![lighting, data]);
        }
        let children = children_of(e, node);
        if children != 0 {
            let mut index = 0u32;
            while index < e.call(CHILD_COUNT, &args![children]).u32() {
                if e.call(CHILD_AT, &args![children, index]).u32() != 0 {
                    let child = e.call(CHILD_AT, &args![children, index]).u32();
                    tint_scenegraph(e, Ptr::new(child), colour, effect_data);
                }
                index += 1;
            }
        }
        e.call(SMART_POINTER_RELEASE, &args![holder]);
    });
}

// Translated from 004b6630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `UnTintScenegraph` (Xbox PDB): clears the texture effect data of every
/// lighting property below `node` (the reverse of [`tint_scenegraph`]).
pub fn un_tint_scenegraph(e: &mut Engine, node: Ptr) {
    let property = e.call(GET_PROPERTY, &args![node, 3u32]).u32();
    let lighting = e
        .call(DYNAMIC_CAST, &args![CAST_CLASS_011FA010, property])
        .u32();
    if lighting != 0 {
        e.call(SET_TEXTURE_EFFECT_DATA, &args![lighting, 0u32]);
    }
    let children = children_of(e, node);
    if children != 0 {
        let mut index = 0u32;
        while index < e.call(CHILD_COUNT, &args![children]).u32() {
            if e.call(CHILD_AT, &args![children, index]).u32() != 0 {
                let child = e.call(CHILD_AT, &args![children, index]).u32();
                un_tint_scenegraph(e, Ptr::new(child));
            }
            index += 1;
        }
    }
}

/// The flags word of the context the scene graph walker hands its callbacks
/// (a 0x1c-byte block of which the callers fill `+4` with a byte 1, `+8`
/// with `0x12`, the result at `+0xc` (the count) or `+0x14` (the index
/// found), the value looked for at `+0x10`, and these flags at `+0x18`):
/// bits 0, 1 and 2 are the three flag arguments of the callers.
fn walk_flags(first: bool, second: bool, third: bool) -> u32 {
    u32::from(first) | (u32::from(second) << 1) | (u32::from(third) << 2)
}

// Translated from 004b66d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GetCollisionObjectCountInSceneGraph` (Xbox PDB): walks the scene graph
/// from `root` with `00c68900` and the callback [`fn_004b6740`], over a
/// context whose flags are the three arguments (bits 0, 1, 2), and returns
/// the count the callback built up.
pub fn get_collision_object_count_in_scene_graph(
    e: &mut Engine,
    root: Ptr,
    first_flag: bool,
    second_flag: bool,
    third_flag: bool,
) -> u32 {
    e.with_stack(0x1c, |e, context| {
        e.mem.set_u8(context.addr() + 4, 1);
        e.mem.set_u32(context.addr() + 8, 0x12);
        e.mem.set_u32(context.addr() + 0xc, 0);
        e.mem.set_u32(
            context.addr() + 0x18,
            walk_flags(first_flag, second_flag, third_flag),
        );
        e.call(WALK_SCENE_GRAPH, &args![root, context, COUNT_CALLBACK]);
        e.mem.u32(context.addr() + 0xc)
    })
}

// Translated from 004b6740 (decompiled, FalloutNV.exe 1.4.0.525)
/// The walker callback of [`get_collision_object_count_in_scene_graph`]:
/// adds one to the count (`context + 0xc`) unless the flags say to skip
/// the object. Flag bit 0 skips an object of the class `0126817c`; flag
/// bit 1 skips one whose `+8` word is of the class `011f9140` and one
/// whose `+8` word has a name that does not differ from `"Arrow"`.
pub fn fn_004b6740(e: &mut Engine, object: Ptr, context: Ptr) {
    let flags = e.mem.u32(context.addr() + 0x18);
    if flags & 1 != 0
        && e.call(IS_OF_CLASS, &args![IS_OF_CLASS_KEY_0126817C, object])
            .bool()
    {
        return;
    }
    if e.mem.u32(context.addr() + 0x18) & 2 != 0 {
        let inner = e.call(WORD_AT_EIGHT, &args![object]).u32();
        if e.call(IS_OF_CLASS, &args![IS_OF_CLASS_KEY_011F9140, inner])
            .bool()
        {
            return;
        }
    }
    if e.mem.u32(context.addr() + 0x18) & 2 != 0 && e.call(WORD_AT_EIGHT, &args![object]).u32() != 0
    {
        let inner = e.call(WORD_AT_EIGHT, &args![object]).u32();
        if name_of(e, Ptr::new(inner)) != 0 {
            let inner = e.call(WORD_AT_EIGHT, &args![object]).u32();
            let name = name_of(e, Ptr::new(inner));
            if e.call(STRING_COMPARE_SECOND, &args![name, ARROW_NAME])
                .u32()
                == 0
            {
                return;
            }
        }
    }
    let count = e.mem.u32(context.addr() + 0xc);
    e.mem.set_u32(context.addr() + 0xc, count.wrapping_add(1));
}

// Translated from 004b6810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Like [`get_collision_object_count_in_scene_graph`] but with the callback
/// `004b68a0` and the extra word `value` in the context (`+0x10`): returns
/// the word at `context + 0x14`, which starts at `-1` (the callback stores
/// the result there).
pub fn fn_004b6810(
    e: &mut Engine,
    root: Ptr,
    value: u32,
    first_flag: bool,
    second_flag: bool,
    third_flag: bool,
) -> i32 {
    e.with_stack(0x1c, |e, context| {
        e.mem.set_u8(context.addr() + 4, 1);
        e.mem.set_u32(context.addr() + 8, 0x12);
        e.mem.set_u32(context.addr() + 0xc, 0);
        e.mem.set_u32(context.addr() + 0x10, value);
        e.mem.set_u32(context.addr() + 0x14, 0xffff_ffff);
        e.mem.set_u32(
            context.addr() + 0x18,
            walk_flags(first_flag, second_flag, third_flag),
        );
        e.call(WALK_SCENE_GRAPH, &args![root, context, FIND_CALLBACK]);
        e.mem.i32(context.addr() + 0x14)
    })
}

// ---------------------------------------------------------------------
// Third session: scene graph walks (continued), the language texture
// swap, `FixedStrings` and the `NiTMap<unsigned int, VertexDist>` methods.
// ---------------------------------------------------------------------

/// Whether the walker callbacks `004b68a0` and `004b6a00` skip an object,
/// the same tests as [`fn_004b6740`]: flag bit 0 skips an object of the
/// class `0126817c`; flag bit 1 skips one whose `+8` word is of the class
/// `011f9140` and one whose `+8` word has a name that does not differ from
/// `"Arrow"`.
fn walker_skips(e: &mut Engine, object: Ptr, context: Ptr) -> bool {
    let flags = e.mem.u32(context.addr() + 0x18);
    if flags & 1 != 0
        && e.call(IS_OF_CLASS, &args![IS_OF_CLASS_KEY_0126817C, object])
            .bool()
    {
        return true;
    }
    if flags & 2 != 0 {
        let inner = e.call(WORD_AT_EIGHT, &args![object]).u32();
        if e.call(IS_OF_CLASS, &args![IS_OF_CLASS_KEY_011F9140, inner])
            .bool()
        {
            return true;
        }
    }
    if flags & 2 != 0 && e.call(WORD_AT_EIGHT, &args![object]).u32() != 0 {
        let inner = e.call(WORD_AT_EIGHT, &args![object]).u32();
        if name_of(e, Ptr::new(inner)) != 0 {
            let inner = e.call(WORD_AT_EIGHT, &args![object]).u32();
            let name = name_of(e, Ptr::new(inner));
            if e.call(STRING_COMPARE_SECOND, &args![name, ARROW_NAME])
                .u32()
                == 0
            {
                return true;
            }
        }
    }
    false
}

/// Fills the first words of a walker context (`+4` a byte 1, `+8` the
/// word `0x12`), the way every caller of the walker `00c68900` does.
fn start_walk_context(e: &mut Engine, context: Ptr) {
    e.mem.set_u8(context.addr() + 4, 1);
    e.mem.set_u32(context.addr() + 8, 0x12);
}

// Translated from 004b68a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The walker callback of [`fn_004b6810`], `cdecl(object, context)`: for
/// each object the flags do not skip ([`walker_skips`]), when the object is
/// the one in `context + 0x10` it stores the running count (`+0xc`) in
/// `context + 0x14`, then adds one to the count.
pub fn fn_004b68a0(e: &mut Engine, object: Ptr, context: Ptr) {
    if walker_skips(e, object, context) {
        return;
    }
    if object.addr() == e.mem.u32(context.addr() + 0x10) {
        let count = e.mem.u32(context.addr() + 0xc);
        e.mem.set_u32(context.addr() + 0x14, count);
    }
    let count = e.mem.u32(context.addr() + 0xc);
    e.mem.set_u32(context.addr() + 0xc, count.wrapping_add(1));
}

// Translated from 004b6980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Walks the scene graph from `root` with `00c68900` and the callback
/// [`fn_004b6a00`] over a context whose flags are the three arguments (bits
/// 0, 1, 2) and whose `+0x14` is `index`; returns the object the callback
/// stored at `context + 0x10` (0 when the walk had fewer objects).
pub fn fn_004b6980(
    e: &mut Engine,
    root: Ptr,
    index: u32,
    first_flag: bool,
    second_flag: bool,
    third_flag: bool,
) -> u32 {
    e.with_stack(0x1c, |e, context| {
        start_walk_context(e, context);
        e.mem.set_u32(context.addr() + 0xc, 0);
        e.mem.set_u32(context.addr() + 0x10, 0);
        e.mem.set_u32(context.addr() + 0x14, index);
        e.mem.set_u32(
            context.addr() + 0x18,
            walk_flags(first_flag, second_flag, third_flag),
        );
        e.call(WALK_SCENE_GRAPH, &args![root, context, SELECT_CALLBACK]);
        e.mem.u32(context.addr() + 0x10)
    })
}

// Translated from 004b6a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The walker callback of [`fn_004b6980`], `cdecl(object, context)`: for
/// each object the flags do not skip, when the running count (`+0xc`)
/// equals `context + 0x14` it stores the object in `context + 0x10`, then
/// adds one to the count.
pub fn fn_004b6a00(e: &mut Engine, object: Ptr, context: Ptr) {
    if walker_skips(e, object, context) {
        return;
    }
    if e.mem.u32(context.addr() + 0xc) == e.mem.u32(context.addr() + 0x14) {
        e.mem.set_u32(context.addr() + 0x10, object.addr());
    }
    let count = e.mem.u32(context.addr() + 0xc);
    e.mem.set_u32(context.addr() + 0xc, count.wrapping_add(1));
}

// Translated from 004b6ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GetHavokWorldObjectCountInSceneGraph` (Xbox PDB): walks the scene graph
/// from `root` with `00c68900` and the callback [`fn_004b6b20`] over a
/// context whose count (`+0xc`) starts at 0, and returns the count.
pub fn get_havok_world_object_count_in_scene_graph(e: &mut Engine, root: Ptr) -> u32 {
    e.with_stack(0x1c, |e, context| {
        start_walk_context(e, context);
        e.mem.set_u32(context.addr() + 0xc, 0);
        e.call(
            WALK_SCENE_GRAPH,
            &args![root, context, HAVOK_COUNT_CALLBACK],
        );
        e.mem.u32(context.addr() + 0xc)
    })
}

/// What the three callbacks `004b6b20`, `004b6bc0` and `004b6c70` look at:
/// the word at `+8` of the smart pointer at `object + 0x10` (0 when the
/// object has no such pointer or the word is 0).
fn attached_word(e: &mut Engine, object: Ptr) -> u32 {
    let pointed = e.call(POINTER_AT_0X10, &args![object]).u32();
    if pointed == 0 {
        return 0;
    }
    e.call(WORD_AT_EIGHT_WRAPPER, &args![pointed]).u32()
}

// Translated from 004b6b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The walker callback of [`get_havok_world_object_count_in_scene_graph`],
/// `cdecl(object, context)`: adds one to the count (`context + 0xc`) when
/// the object has an attached word ([`attached_word`]).
pub fn fn_004b6b20(e: &mut Engine, object: Ptr, context: Ptr) {
    if attached_word(e, object) != 0 {
        let count = e.mem.u32(context.addr() + 0xc);
        e.mem.set_u32(context.addr() + 0xc, count.wrapping_add(1));
    }
}

// Translated from 004b6b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Walks the scene graph from `root` with the callback [`fn_004b6bc0`],
/// over a context with the count at 0, `value` at `+0x10` and `-1` at
/// `+0x14`; returns the word at `+0x14`: the count the callback had reached
/// when it met an object whose attached word is `value`, else `-1`.
pub fn fn_004b6b60(e: &mut Engine, root: Ptr, value: u32) -> i32 {
    e.with_stack(0x1c, |e, context| {
        start_walk_context(e, context);
        e.mem.set_u32(context.addr() + 0xc, 0);
        e.mem.set_u32(context.addr() + 0x10, value);
        e.mem.set_u32(context.addr() + 0x14, 0xffff_ffff);
        e.call(WALK_SCENE_GRAPH, &args![root, context, HAVOK_FIND_CALLBACK]);
        e.mem.i32(context.addr() + 0x14)
    })
}

// Translated from 004b6bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The walker callback of [`fn_004b6b60`], `cdecl(object, context)`: for an
/// object with an attached word, when the word is `context + 0x10` it
/// stores the running count in `context + 0x14`; then adds one to the
/// count.
pub fn fn_004b6bc0(e: &mut Engine, object: Ptr, context: Ptr) {
    let word = attached_word(e, object);
    if word != 0 {
        if word == e.mem.u32(context.addr() + 0x10) {
            let count = e.mem.u32(context.addr() + 0xc);
            e.mem.set_u32(context.addr() + 0x14, count);
        }
        let count = e.mem.u32(context.addr() + 0xc);
        e.mem.set_u32(context.addr() + 0xc, count.wrapping_add(1));
    }
}

// Translated from 004b6c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Walks the scene graph from `root` with the callback [`fn_004b6c70`],
/// over a context with the count and the result (`+0x10`) at 0 and `index`
/// at `+0x14`; returns the result: the attached word of the object the
/// callback met when its count was `index`.
pub fn fn_004b6c20(e: &mut Engine, root: Ptr, index: u32) -> u32 {
    e.with_stack(0x1c, |e, context| {
        start_walk_context(e, context);
        e.mem.set_u32(context.addr() + 0xc, 0);
        e.mem.set_u32(context.addr() + 0x10, 0);
        e.mem.set_u32(context.addr() + 0x14, index);
        e.call(
            WALK_SCENE_GRAPH,
            &args![root, context, HAVOK_INDEX_CALLBACK],
        );
        e.mem.u32(context.addr() + 0x10)
    })
}

// Translated from 004b6c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The walker callback of [`fn_004b6c20`], `cdecl(object, context)`: for an
/// object with an attached word, when the running count (`+0xc`) equals
/// `context + 0x14` it stores the word in `context + 0x10`, and adds one to
/// the count. It then adds one to the count again for every object, with
/// or without a word (as the code does, so the count runs ahead by one for
/// each object that has a word).
pub fn fn_004b6c70(e: &mut Engine, object: Ptr, context: Ptr) {
    let word = attached_word(e, object);
    if word != 0 {
        if e.mem.u32(context.addr() + 0xc) == e.mem.u32(context.addr() + 0x14) {
            e.mem.set_u32(context.addr() + 0x10, word);
        }
        let count = e.mem.u32(context.addr() + 0xc);
        e.mem.set_u32(context.addr() + 0xc, count.wrapping_add(1));
    }
    let count = e.mem.u32(context.addr() + 0xc);
    e.mem.set_u32(context.addr() + 0xc, count.wrapping_add(1));
}

/// `property` when `004a2c20` accepts it (its kind word is between 1 and
/// 0xc), else 0.
fn accepted_property(e: &mut Engine, property: u32) -> u32 {
    if e.call(IN_KIND_RANGE, &args![property]).u32() != 0 {
        property
    } else {
        0
    }
}

// Translated from 004b6ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Walks the scene graph below `object`. At an object whose virtual slot
/// `0x1c` is non-zero it stops there: it takes the property of kind 3
/// (`NiAVObject::GetProperty`), and when `004a2c20` accepts it calls
/// `004534f0` (which does nothing) on the object at `011e0e80` with the
/// property. Elsewhere it recurses into the children.
pub fn fn_004b6ce0(e: &mut Engine, object: Ptr) {
    if object.addr() == 0 {
        return;
    }
    if e.vcall(object.addr(), 0x1c, &[]).u32() != 0 {
        let property = e.call(GET_PROPERTY, &args![object, 3u32]).u32();
        if property != 0 {
            let accepted = accepted_property(e, property);
            if accepted != 0 {
                // The game tests whether the kind word (`00441110`) is 3
                // here; both outcomes leave the function.
                e.call(WORD_AT_0X1C, &args![accepted]);
                e.call(NOTIFY, &args![NOTIFIED_OBJECT, accepted]);
            }
        }
        return;
    }
    let children = children_of(e, object);
    if children != 0 {
        let mut index = 0u32;
        while index < e.call(CHILD_COUNT, &args![children]).u32() {
            let child = e.call(CHILD_AT, &args![children, index]).u32();
            fn_004b6ce0(e, Ptr::new(child));
            index += 1;
        }
    }
}

// Translated from 004b6dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SetReferenceIDOnScenegraph` (Xbox PDB): below `object`, at the objects
/// whose virtual slot `0x18` is non-zero, takes the property of kind 3,
/// and when `004a2c20` accepts it calls `0057bd60(property, reference)`.
/// Stops at such an object; elsewhere recurses into the children.
pub fn set_reference_id_on_scenegraph(e: &mut Engine, object: Ptr, reference: u32) {
    if object.addr() == 0 {
        return;
    }
    if e.vcall(object.addr(), 0x18, &[]).u32() != 0 {
        let property = e.call(GET_PROPERTY, &args![object, 3u32]).u32();
        let accepted = accepted_property(e, property);
        if accepted != 0 {
            e.call(SET_REFERENCE_ID, &args![accepted, reference]);
        }
        return;
    }
    let children = children_of(e, object);
    if children != 0 {
        let mut index = 0u32;
        while index < e.call(CHILD_COUNT, &args![children]).u32() {
            let child = e.call(CHILD_AT, &args![children, index]).u32();
            set_reference_id_on_scenegraph(e, Ptr::new(child), reference);
            index += 1;
        }
    }
}

// Translated from 004b6e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the smart pointer at `this + 0x13c` (`00559450`).
pub fn fn_004b6e70(e: &mut Engine, this: Ptr) -> u32 {
    e.call(SMART_POINTER_GET, &args![this.addr().wrapping_add(0x13c)])
        .u32()
}

/// A new `BSShaderNoLightingProperty` (0x80 bytes), or 0 when the
/// allocation fails.
fn new_no_lighting_property(e: &mut Engine) -> u32 {
    let block = e.call(NI_OPERATOR_NEW, &args![0x80u32]).u32();
    if block != 0 {
        e.call(NO_LIGHTING_PROPERTY_CONSTRUCT, &args![block]).u32()
    } else {
        0
    }
}

// Translated from 004b6e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AddNoLightingPropertyRecurse` (Xbox PDB): gives every object below
/// `object` (and `object`) whose virtual slot `0x18` is non-zero and that
/// has no property of kind 3 a new `BSShaderNoLightingProperty`, attached
/// with `NiAVObject::AttachProperty`. The exception frame is not
/// translated.
pub fn add_no_lighting_property_recurse(e: &mut Engine, object: Ptr) {
    if e.vcall(object.addr(), 0x18, &[]).u32() != 0
        && e.call(GET_PROPERTY, &args![object, 3u32]).u32() == 0
    {
        let property = new_no_lighting_property(e);
        e.call(ATTACH_PROPERTY, &args![object, property]);
    }
    let children = children_of(e, object);
    if children != 0 {
        let mut index = 0u32;
        while index < e.call(CHILD_COUNT, &args![children]).u32() {
            let child = e.call(CHILD_AT, &args![children, index]).u32();
            if child != 0 {
                add_no_lighting_property_recurse(e, Ptr::new(child));
            }
            index += 1;
        }
    }
}

// Translated from 004b6f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PrepareObjectWithNoLightingRecurse` (Xbox PDB): at the first object
/// (from `object` down) whose virtual slot `0x18` is non-zero and that has
/// no property of kind 3, it makes a `BSShaderNoLightingProperty`, sets its
/// flag `0x30`, attaches it, updates the object's properties and calls
/// `BSShaderManager::PrepareObject(object, 0, 0)`, and does not look at
/// the children. Otherwise it recurses into the children, but only when
/// [`fn_004b2960`] says the container has a world bound. The exception
/// frame is not translated.
pub fn prepare_object_with_no_lighting_recurse(e: &mut Engine, object: Ptr) {
    if e.vcall(object.addr(), 0x18, &[]).u32() != 0
        && e.call(GET_PROPERTY, &args![object, 3u32]).u32() == 0
    {
        let property = new_no_lighting_property(e);
        e.call(SET_SHADER_FLAG, &args![property, 0x30u32, 1u32]);
        e.call(ATTACH_PROPERTY, &args![object, property]);
        e.call(UPDATE_PROPERTIES, &args![object]);
        e.call(PREPARE_OBJECT, &args![object, 0u32, 0u32]);
        return;
    }
    let children = children_of(e, object);
    if children != 0 && fn_004b2960(e, Ptr::new(children)) {
        let mut index = 0u32;
        while index < e.call(CHILD_COUNT, &args![children]).u32() {
            let child = e.call(CHILD_AT, &args![children, index]).u32();
            if child != 0 {
                prepare_object_with_no_lighting_recurse(e, Ptr::new(child));
            }
            index += 1;
        }
    }
}

// Translated from 004b70d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the smart pointer at `this + 0x3c` (`00559450`).
pub fn fn_004b70d0(e: &mut Engine, this: Ptr) -> u32 {
    e.call(SMART_POINTER_GET, &args![this.addr().wrapping_add(0x3c)])
        .u32()
}

// Translated from 004b70f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Below (and at) `object`, which is its own children container: walks the
/// chain that starts at `0043b230(object)` and continues with `004a8a90`,
/// storing `value` (`00621b20`) in each item, then does the same for every
/// child that is of the class `011f4428`.
pub fn fn_004b70f0(e: &mut Engine, object: Ptr, value: f32) {
    if object.addr() == 0 {
        return;
    }
    let mut item = e.call(FIRST_ITEM, &args![object]).u32();
    while item != 0 {
        e.call(SET_FLOAT_AT_0X10, &args![item, value]);
        item = e.call(NEXT_ITEM, &args![item]).u32();
    }
    let mut index = 0u32;
    while index < e.call(CHILD_COUNT, &args![object]).u32() {
        let child = e.call(CHILD_AT, &args![object, index]).u32();
        if child != 0
            && e.call(IS_OF_CLASS, &args![IS_OF_CLASS_KEY_011F4428, child])
                .bool()
        {
            fn_004b70f0(e, Ptr::new(child), value);
        }
        index += 1;
    }
}

// Translated from 004b71a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// [`fn_004b71d0`] as a `bool`.
pub fn fn_004b71a0(e: &mut Engine) -> bool {
    fn_004b71d0(e)
}

// Translated from 004b71d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the byte at `011d8a84` is set: reads [`fn_004b7210`] (the value is
/// not used) and returns [`fn_004b9930`] of the string object at
/// `011d8c4c`; else false.
pub fn fn_004b71d0(e: &mut Engine) -> bool {
    if e.mem.u8(PLATFORM_SWAP_ENABLED) == 0 {
        return false;
    }
    fn_004b7210(e);
    fn_004b9930(e, Ptr::new(PLATFORM_SWAP_STRING))
}

// Translated from 004b7210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the word at `011d8a80`.
pub fn fn_004b7210(e: &mut Engine) -> u32 {
    e.mem.u32(PLATFORM_SWAP_WORD)
}

// Translated from 004b7220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GetCurrentTexLanguage` (Xbox PDB): the index of the texture language,
/// from `00a22cb0` (which takes no arguments).
pub fn get_current_tex_language(e: &mut Engine) -> u32 {
    e.call(CURRENT_LANGUAGE_WORKER, &[]).u32()
}

/// The path separator (`\` or `/`) at `found + length`, if there is one.
fn separator_after(e: &Engine, found: u32, length: u32) -> Option<u8> {
    let c = e.mem.u8(found.wrapping_add(length));
    (c == b'\\' || c == b'/').then_some(c)
}

/// How far `swap_platform_language_texture_path` has got: the point in the
/// input it continues from and the length written to the output.
struct PathSwap {
    cursor: u32,
    written: u32,
}

/// One of the two search loops of [`swap_platform_language_texture_path`]:
/// looks, for each of `count` names of `table` in turn, for the first
/// occurrence (ignoring case) that is followed by `\` or `/`. On the first
/// name that has one, it copies the text before it to the output (when
/// there is any), appends the name `replacement` of the table, the
/// separator and a zero byte, moves the cursor past the name and its
/// separator, and returns true.
fn swap_one_name(
    e: &mut Engine,
    state: &mut PathSwap,
    out: Ptr,
    size: u32,
    table: u32,
    count: u32,
    replacement: u32,
) -> bool {
    for index in 0..count {
        let name = e.mem.u32(table.wrapping_add(index.wrapping_mul(4)));
        let mut separator = 0u8;
        let mut found = bs_stristr(e, Ptr::new(state.cursor), Ptr::new(name));
        while found != 0 {
            let length = e.call(STRING_LENGTH, &args![name]).u32();
            if let Some(c) = separator_after(e, found, length) {
                separator = c;
                break;
            }
            found = bs_stristr(e, Ptr::new(found.wrapping_add(length)), Ptr::new(name));
        }
        if found == 0 {
            continue;
        }
        let prefix = found.wrapping_sub(state.cursor);
        if prefix != 0 {
            fn_004b7580(
                e,
                Ptr::new(out.addr().wrapping_add(state.written)),
                size.wrapping_sub(state.written),
                Ptr::new(state.cursor),
                prefix,
            );
            state.written = state.written.wrapping_add(prefix);
            e.mem.set_u8(out.addr().wrapping_add(state.written), 0);
        }
        let new_name = e.mem.u32(table.wrapping_add(replacement.wrapping_mul(4)));
        e.call(STRING_CONCAT, &args![out, size, new_name]);
        let new_length = e.call(CRT_STRLEN, &args![new_name]).u32();
        state.written = state.written.wrapping_add(new_length);
        e.mem
            .set_u8(out.addr().wrapping_add(state.written), separator);
        state.written = state.written.wrapping_add(1);
        e.mem.set_u8(out.addr().wrapping_add(state.written), 0);
        let old_length = e.call(CRT_STRLEN, &args![name]).u32();
        state.cursor = found.wrapping_add(old_length).wrapping_add(1);
        return true;
    }
    false
}

// Translated from 004b7240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SwapPlatformLanguageTexturePath` (Xbox PDB): copies the texture path
/// `path` to `out` (`size` bytes) replacing the first directory named like
/// one of the four platform names (table `01189038`) by the current
/// platform's name ([`fn_004b71a0`] indexes the table) and then the first
/// directory named like one of the five language names (table `01188f84`)
/// by the current language's ([`get_current_tex_language`]); the rest of
/// the path is appended unchanged. Returns whether either was replaced.
/// False, with nothing written, when a pointer is null or `size` is 0.
pub fn swap_platform_language_texture_path(e: &mut Engine, path: Ptr, out: Ptr, size: u32) -> bool {
    if path.addr() == 0 || out.addr() == 0 || size == 0 {
        return false;
    }
    let mut swapped = false;
    e.mem.set_u8(out.addr(), 0);
    let platform = u32::from(fn_004b71a0(e));
    let language = get_current_tex_language(e);
    let mut state = PathSwap {
        cursor: path.addr(),
        written: 0,
    };
    if swap_one_name(e, &mut state, out, size, PLATFORM_NAMES, 4, platform) {
        swapped = true;
    }
    if swap_one_name(e, &mut state, out, size, LANGUAGE_NAMES, 5, language) {
        swapped = true;
    }
    e.call(STRING_CONCAT, &args![out, size, state.cursor]);
    swapped
}

// Translated from 004b7580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `memcpy_s(destination, destination size, source, count)` through the
/// CRT function `00ec7c66`; returns its result.
pub fn fn_004b7580(
    e: &mut Engine,
    destination: Ptr,
    destination_size: u32,
    source: Ptr,
    count: u32,
) -> u32 {
    e.call(
        CRT_MEMCPY_S,
        &args![destination, destination_size, source, count],
    )
    .u32()
}

// Translated from 004b75a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSstristr` (Xbox PDB): the first place in `haystack` where `needle`
/// occurs ignoring case (`tolower` of each byte as a signed `char`), or 0.
/// An empty needle matches at the start.
pub fn bs_stristr(e: &mut Engine, haystack: Ptr, needle: Ptr) -> u32 {
    let haystack_length = e.call(CRT_STRLEN, &args![haystack]).u32() as i32;
    let needle_length = e.call(CRT_STRLEN, &args![needle]).u32() as i32;
    let mut start = 0i32;
    while start < haystack_length.wrapping_sub(needle_length).wrapping_add(1) {
        let mut matched = 0i32;
        while matched < needle_length {
            let a = e.mem.i8(haystack
                .addr()
                .wrapping_add(start as u32)
                .wrapping_add(matched as u32));
            let b = e.mem.i8(needle.addr().wrapping_add(matched as u32));
            let lower_a = e.call(CRT_TOLOWER, &args![a as i32]).i32();
            let lower_b = e.call(CRT_TOLOWER, &args![b as i32]).i32();
            if lower_a != lower_b {
                break;
            }
            matched += 1;
        }
        if matched == needle_length {
            return haystack.addr().wrapping_add(start as u32);
        }
        start += 1;
    }
    0
}

// Translated from 004b7650 (decompiled, FalloutNV.exe 1.4.0.525)
/// The language name pointer `index` of the table at `01188f84`.
pub fn fn_004b7650(e: &mut Engine, index: i32) -> u32 {
    e.mem
        .u32(LANGUAGE_NAMES.wrapping_add((index as u32).wrapping_mul(4)))
}

// Translated from 004b7660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SwapPlatformLanguageTextures` (Xbox PDB): below `root`, for each object
/// that has a property of kind 3 whose kind word (`00441110`) is between 8
/// and 0xc (a lighting property with textures: virtual slot `0xe8` gives
/// the number of texture sets, slot `0xf4(slot, set)` the texture in each
/// of 6 slots of each set), looks at each texture of the class `011f444c`:
/// its name (virtual slot `0x9c`, in a `NiFixedString`) goes through
/// [`swap_platform_language_texture_path`] and when that changed it (and
/// the result is not empty) the new texture is loaded with
/// `TES::CreateTextureImage` and set with slot `0xf0(slot, set, texture)`.
/// Then recurses into the children. Returns whether any texture was
/// replaced. The exception frame is not translated.
pub fn swap_platform_language_textures(e: &mut Engine, root: Ptr) -> bool {
    let mut replaced = false;
    if root.addr() == 0 {
        return false;
    }
    let property = e.call(GET_PROPERTY, &args![root, 3u32]).u32();
    let in_range = property != 0
        && e.call(WORD_AT_0X1C, &args![property]).i32() >= 8
        && e.call(WORD_AT_0X1C, &args![property]).i32() <= 0xc;
    let lighting = if in_range { property } else { 0 };
    if lighting != 0 {
        let sets = e.vcall(lighting, 0xe8, &[]).u32() & 0xffff;
        for set in 0..sets {
            for slot in 0..6u32 {
                let texture = e.vcall(lighting, 0xf4, &args![slot, set]).u32();
                if texture != 0
                    && e.call(IS_OF_CLASS, &args![IS_OF_CLASS_KEY_011F444C, texture])
                        .bool()
                    && swap_one_texture(e, lighting, texture, slot, set)
                {
                    replaced = true;
                }
            }
        }
    }
    let children = children_of(e, root);
    if children != 0 {
        let count = e.call(CHILD_COUNT, &args![children]).u32();
        for index in 0..count {
            let child = e.call(CHILD_AT, &args![children, index]).u32();
            if swap_platform_language_textures(e, Ptr::new(child)) {
                replaced = true;
            }
        }
    }
    replaced
}

/// The body of the texture loop of [`swap_platform_language_textures`]:
/// swaps the path of one texture and loads and sets the new image.
fn swap_one_texture(e: &mut Engine, lighting: u32, texture: u32, slot: u32, set: u32) -> bool {
    // A `NiFixedString` (4 bytes), a smart pointer (4 bytes) and the 0x104
    // byte path buffer, on the stack as in the game.
    e.with_stack(0x10c, |e, block| {
        let fixed = block;
        let smart: Ptr = Ptr::new(block.addr() + 4);
        let buffer: Ptr = Ptr::new(block.addr() + 8);
        let name = e.vcall(texture, 0x9c, &[]).u32();
        e.call(FIXED_STRING_COPY, &args![fixed, name]);
        e.mem.set_u8(buffer.addr(), 0);
        let text = e.call(NAME_TEXT, &args![fixed]).u32();
        let changed = swap_platform_language_texture_path(e, Ptr::new(text), buffer, 0x104);
        let mut replaced = false;
        if changed && e.mem.i8(buffer.addr()) != 0 {
            e.call(SMART_POINTER_CONSTRUCT, &args![smart, 0u32]);
            let tes: u32 = e.global(TES_SINGLETON);
            e.call(CREATE_TEXTURE_IMAGE, &args![tes, buffer, smart, 0u32, 0u32]);
            if e.call(SMART_POINTER_GET, &args![smart]).u32() != 0 {
                let image = e.call(SMART_POINTER_GET, &args![smart]).u32();
                e.vcall(lighting, 0xf0, &args![slot, set, image]);
                replaced = true;
            }
            e.call(SMART_POINTER_RELEASE, &args![smart]);
        }
        e.call(FIXED_STRING_DESTRUCT, &args![fixed]);
        replaced
    })
}

// Translated from 004b7920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FixedStrings::InitSDM` (Xbox PDB): for each of the 54 `FixedStrings`
/// members ([`FIXED_STRINGS`]) allocates a 4-byte object with `operator
/// new` and, when that succeeds, constructs a `NiFixedString` from the C
/// string in it; stores the object (or 0) in the member's global. The
/// exception frame is not translated.
pub fn fixed_strings_init_sdm(e: &mut Engine) {
    for (global, string) in FIXED_STRINGS {
        let block = e.call(OPERATOR_NEW, &args![4u32]).u32();
        let made = if block != 0 {
            e.call(FIXED_STRING_CONSTRUCT, &args![block, string]).u32()
        } else {
            0
        };
        e.set_global(global, made);
    }
}

// Translated from 004b8cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The shutdown counterpart of [`fixed_strings_init_sdm`]: for each of the
/// `FixedStrings` globals that is not 0 calls [`fn_004b9900`] with flag 1
/// (destroy and free). `pBip01Spine` and `pBip01Spine1` are not freed (the
/// code never reads them), and no global is cleared.
pub fn fn_004b8cf0(e: &mut Engine) {
    for (global, _) in FIXED_STRINGS {
        if global == FIXED_STRING_SPINE || global == FIXED_STRING_SPINE1 {
            continue;
        }
        let object: u32 = e.global(global);
        if object != 0 {
            fn_004b9900(e, Ptr::new(object), 1);
        }
    }
}

// Translated from 004b9900 (decompiled, FalloutNV.exe 1.4.0.525)
/// The scalar deleting destructor of the 4-byte `NiFixedString` objects of
/// `FixedStrings`: runs the destructor (`004381b0`) and, when bit 0 of
/// `flags` is set, frees the block (`operator delete`). Returns `this`.
pub fn fn_004b9900(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(FIXED_STRING_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 004b9930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the byte at `this + 4` is 0.
pub fn fn_004b9930(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr().wrapping_add(4)) == 0
}

// Translated from 004b9950 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of `NiTMap<unsigned int, VertexDist>` (Xbox PDB name
/// not in the map): the base constructor [`fn_004b99b0`] with the bucket
/// count, then the map's own vtable (`010205b4`). Returns `this`.
pub fn fn_004b9950(e: &mut Engine, this: Ptr<NiTMap>, hash_size: u32) -> Ptr<NiTMap> {
    fn_004b99b0(e, this, hash_size);
    e.mem.set_u32(this.addr(), VERTEX_DIST_MAP_VTABLE);
    this
}

// Translated from 004b9980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<unsigned_int_VertexDist>::_scalar_deleting_destructor_` (Xbox
/// PDB): runs the destructor [`fn_004b9cd0`] and, when bit 0 of `flags` is
/// set, frees the block. Returns `this`.
pub fn ni_t_map_vertex_dist_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTMap>,
    flags: u32,
) -> Ptr<NiTMap> {
    fn_004b9cd0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 004b99b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of `NiTMapBase` for this map (Xbox PDB name not in the
/// map): installs the base vtable (`010205d4`), stores the bucket count
/// (`+4`), a zero item count (`+0xc`) and a new bucket array of
/// `4 * hash_size` bytes (`+8`) cleared with `memset`. Returns `this`.
pub fn fn_004b99b0(e: &mut Engine, this: Ptr<NiTMap>, hash_size: u32) -> Ptr<NiTMap> {
    e.mem.set_u32(this.addr(), MAP_BASE_VTABLE);
    e.set(this, NiTMap::m_uiHashSize, hash_size);
    e.set(this, NiTMap::m_uiCount, 0);
    let bytes = hash_size << 2;
    let table = e.call(NI_ARRAY_NEW, &args![bytes]).u32();
    e.set(this, NiTMap::m_ppkHashTable, table);
    e.call(MEMORY_SET, &args![table, 0u32, bytes]);
    this
}

// Translated from 004b9a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SetAt` of the map (Xbox PDB name not in the map): looks in the bucket
/// of `key` (virtual slot 4 hashes it, slot 8 compares two keys) for the
/// item with that key; when there is one stores the three value words in
/// it, else takes a new item (slot `0x14`), initialises it (slot `0xc`,
/// `(item, key, word 0, 1, 2)`), links it first in the bucket and counts
/// it.
pub fn fn_004b9a20(
    e: &mut Engine,
    this: Ptr<NiTMap>,
    key: u32,
    word_0: u32,
    word_1: u32,
    word_2: u32,
) {
    let bucket = e.vcall(this.addr(), 4, &args![key]).u32();
    let table = e.get(this, NiTMap::m_ppkHashTable);
    let mut item = e.mem.u32(table.wrapping_add(bucket.wrapping_mul(4)));
    while item != 0 {
        let it = Ptr::<VertexDistMapItem>::new(item);
        let item_key = e.get(it, VertexDistMapItem::m_key);
        if e.vcall(this.addr(), 8, &args![key, item_key]).bool() {
            e.set(it, VertexDistMapItem::m_val_word_0, word_0);
            e.set(it, VertexDistMapItem::m_val_word_1, word_1);
            e.set(it, VertexDistMapItem::m_val_word_2, word_2);
            return;
        }
        item = e.get(it, VertexDistMapItem::m_pkNext);
    }
    let new_item = e.vcall(this.addr(), 0x14, &[]).u32();
    e.vcall(
        this.addr(),
        0xc,
        &args![new_item, key, word_0, word_1, word_2],
    );
    let table = e.get(this, NiTMap::m_ppkHashTable);
    let slot = table.wrapping_add(bucket.wrapping_mul(4));
    let first = e.mem.u32(slot);
    let new_ptr = Ptr::<VertexDistMapItem>::new(new_item);
    e.set(new_ptr, VertexDistMapItem::m_pkNext, first);
    e.mem.set_u32(slot, new_item);
    let count = e.get(this, NiTMap::m_uiCount);
    e.set(this, NiTMap::m_uiCount, count.wrapping_add(1));
}

// Translated from 004b9b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GetAt` of the map (Xbox PDB name not in the map): finds the item of
/// `key` in its bucket (slot 8 compares keys), copies its three value words
/// to `out` and returns true; false when there is none.
pub fn fn_004b9b20(e: &mut Engine, this: Ptr<NiTMap>, key: u32, out: Ptr) -> bool {
    let bucket = e.vcall(this.addr(), 4, &args![key]).u32();
    let table = e.get(this, NiTMap::m_ppkHashTable);
    let mut item = e.mem.u32(table.wrapping_add(bucket.wrapping_mul(4)));
    while item != 0 {
        let it = Ptr::<VertexDistMapItem>::new(item);
        let item_key = e.get(it, VertexDistMapItem::m_key);
        if e.vcall(this.addr(), 8, &args![key, item_key]).bool() {
            let words = [
                e.get(it, VertexDistMapItem::m_val_word_0),
                e.get(it, VertexDistMapItem::m_val_word_1),
                e.get(it, VertexDistMapItem::m_val_word_2),
            ];
            for (i, word) in words.into_iter().enumerate() {
                e.mem.set_u32(out.addr().wrapping_add(4 * i as u32), word);
            }
            return true;
        }
        item = e.get(it, VertexDistMapItem::m_pkNext);
    }
    false
}

// Translated from 004b9ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The first item of the map: the first non-empty bucket's head, or 0.
pub fn fn_004b9ba0(e: &mut Engine, this: Ptr<NiTMap>) -> u32 {
    let size = e.get(this, NiTMap::m_uiHashSize);
    let table = e.get(this, NiTMap::m_ppkHashTable);
    for index in 0..size {
        let head = e.mem.u32(table.wrapping_add(index.wrapping_mul(4)));
        if head != 0 {
            return head;
        }
    }
    0
}

// Translated from 004b9bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The iteration step of the map: `iterator` holds an item pointer; writes
/// that item's key to `key_out` and its three value words to `value_out`,
/// then moves `iterator` to the next item of the bucket, else to the head
/// of the next non-empty bucket after the key's (slot 4 hashes), else to 0.
pub fn fn_004b9bf0(e: &mut Engine, this: Ptr<NiTMap>, iterator: Ptr, key_out: Ptr, value_out: Ptr) {
    let item = e.mem.u32(iterator.addr());
    let it = Ptr::<VertexDistMapItem>::new(item);
    let key = e.get(it, VertexDistMapItem::m_key);
    e.mem.set_u32(key_out.addr(), key);
    let words = [
        e.get(it, VertexDistMapItem::m_val_word_0),
        e.get(it, VertexDistMapItem::m_val_word_1),
        e.get(it, VertexDistMapItem::m_val_word_2),
    ];
    for (i, word) in words.into_iter().enumerate() {
        e.mem
            .set_u32(value_out.addr().wrapping_add(4 * i as u32), word);
    }
    let next = e.get(it, VertexDistMapItem::m_pkNext);
    if next != 0 {
        e.mem.set_u32(iterator.addr(), next);
        return;
    }
    let bucket = e.vcall(this.addr(), 4, &args![key]).u32();
    let mut index = bucket.wrapping_add(1);
    while index < e.get(this, NiTMap::m_uiHashSize) {
        let table = e.get(this, NiTMap::m_ppkHashTable);
        let head = e.mem.u32(table.wrapping_add(index.wrapping_mul(4)));
        if head != 0 {
            e.mem.set_u32(iterator.addr(), head);
            return;
        }
        index = index.wrapping_add(1);
    }
    e.mem.set_u32(iterator.addr(), 0);
}

// Translated from 004b9ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<DFALL<NiTMapItem<unsigned_int_VertexDist>_>_unsigned_int_VertexDist>::SetValue`
/// (Xbox PDB): stores the key and the three value words in `item`; `this`
/// is not used.
pub fn ni_t_map_base_vertex_dist_set_value(
    e: &mut Engine,
    _this: Ptr<NiTMap>,
    item: Ptr<VertexDistMapItem>,
    key: u32,
    word_0: u32,
    word_1: u32,
    word_2: u32,
) {
    e.set(item, VertexDistMapItem::m_key, key);
    e.set(item, VertexDistMapItem::m_val_word_0, word_0);
    e.set(item, VertexDistMapItem::m_val_word_1, word_1);
    e.set(item, VertexDistMapItem::m_val_word_2, word_2);
}

// Translated from 004b9cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of `NiTMap<unsigned int, VertexDist>` (Xbox PDB name not
/// in the map): sets the map's vtable, empties the map (`RemoveAll`,
/// `00438af0`) and runs the base destructor [`fn_004b9d30`]. The exception
/// frame is not translated.
pub fn fn_004b9cd0(e: &mut Engine, this: Ptr<NiTMap>) {
    e.mem.set_u32(this.addr(), VERTEX_DIST_MAP_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    fn_004b9d30(e, this);
}

// Translated from 004b9d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of `NiTMapBase` for this map: sets the base vtable,
/// empties the map and frees the bucket array (`00aa10f0`).
pub fn fn_004b9d30(e: &mut Engine, this: Ptr<NiTMap>) {
    e.mem.set_u32(this.addr(), MAP_BASE_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    let table = e.get(this, NiTMap::m_ppkHashTable);
    e.call(NI_ARRAY_DELETE, &args![table]);
}

// Translated from 004b9d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<unsigned_int_VertexDist>::NewItem` (Xbox PDB): the result of
/// `004b9de0` on the allocator at `this + 0xc`.
pub fn ni_t_map_vertex_dist_new_item(e: &mut Engine, this: Ptr<NiTMap>) -> u32 {
    fn_004b9de0(e, Ptr::new(this.addr().wrapping_add(0xc)))
}

// Translated from 004b9d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees a map item (Xbox PDB name unknown): constructs the empty
/// allocator at `item + 8` (`0065de30`, which only returns its `this`),
/// then `006b8310` on `this + 0xc` with `item`, which calls
/// `operator delete(item)`. (The decompiler hangs the arguments wrongly;
/// the disassembly passes `0` to the first call and `item` to the second.)
pub fn fn_004b9d80(e: &mut Engine, this: Ptr<NiTMap>, item: u32) {
    e.call(
        ALLOCATOR_EMPTY_CONSTRUCTOR,
        &args![item.wrapping_add(8), 0u32],
    );
    e.call(ALLOCATOR_FREE, &args![this.addr().wrapping_add(0xc), item]);
}

// Translated from 004b9db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<DFALL<NiTMapItem<unsigned_int_VertexDist>_>_unsigned_int_VertexDist>::_scalar_deleting_destructor_`
/// (Xbox PDB): runs the base destructor [`fn_004b9d30`] and, when bit 0 of
/// `flags` is set, frees the block. Returns `this`.
pub fn ni_t_map_base_vertex_dist_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTMap>,
    flags: u32,
) -> Ptr<NiTMap> {
    fn_004b9d30(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 004b9de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Allocates a 0x14 byte map item (Xbox PDB name unknown) with
/// `operator new`; `this` (the allocator member) is not used.
pub fn fn_004b9de0(e: &mut Engine, _this: Ptr) -> u32 {
    e.call(OPERATOR_NEW, &args![0x14u32]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004b1230, is_an_integer(Ptr) -> bool),
        entry!(0x004b12d0, is_a_float(Ptr) -> bool),
        entry!(0x004b13c0, get_z_angle_from_vector(Ptr<NiPoint3>) -> f32),
        entry!(0x004b1460, fn_004b1460(f32) -> f32),
        entry!(0x004b1480, clamp_angle(f32) -> f32),
        entry!(0x004b1500, fn_004b1500(f32, f32) -> f32),
        entry!(0x004b1520, fn_004b1520(f32, f32) -> f32),
        entry!(
            0x004b1550,
            get_unclamped_z_angle_from_vector(Ptr<NiPoint3>) -> f32
        ),
        entry!(0x004b15e0, fn_004b15e0(f32, f32, Ptr) -> f32),
        entry!(0x004b1680, fn_004b1680(u8) -> i32),
        entry!(0x004b1b60, fn_004b1b60(u8) -> u32),
        entry!(0x004b1e80, fn_004b1e80(Ptr, Ptr, Ptr, Ptr) -> bool),
        entry!(0x004b2780, fn_004b2780(Ptr, Ptr, Ptr, f32, Ptr, bool) -> bool),
        entry!(0x004b2800, fn_004b2800(Ptr, Ptr, Ptr, f32, Ptr) -> bool),
        entry!(0x004b2960, fn_004b2960(Ptr) -> bool),
        entry!(0x004b29b0, make_tri_point(f32, Ptr, bool) -> u32),
        entry!(
            0x004b2eb0,
            make_quad_box(Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, bool) -> u32
        ),
        entry!(
            0x004b3570,
            make_triangle(f32, f32, f32, f32, f32, f32, f32, f32, f32, Ptr, bool) -> u32
        ),
        entry!(
            0x004b3800,
            fn_004b3800(Ptr<NiPoint3>, Ptr, Ptr<NiPoint3>) -> Ptr
        ),
        entry!(0x004b3890, fn_004b3890(Ptr, Ptr, Ptr, Ptr, bool) -> u32),
        entry!(0x004b3ab0, fn_004b3ab0(f32, f32, f32, f32, f32) -> f32),
        entry!(0x004b3ae0, fn_004b3ae0(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x004b3b90, make_rectangle(f32, Ptr, bool) -> u32),
        entry!(0x004b3e60, fn_004b3e60(Ptr, Ptr, f32, u32, bool) -> bool),
        entry!(0x004b4500, fn_004b4500(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x004b45b0, ni_matrix3_inverse(Ptr, Ptr) -> Ptr),
        entry!(0x004b4600, fn_004b4600(Ptr)),
        entry!(0x004b4660, fn_004b4660(Ptr, u32, f32, f32, f32)),
        entry!(0x004b46a0, ni_matrix3_inverse_ov2(Ptr, Ptr) -> bool),
        entry!(0x004b4910, fn_004b4910(Ptr) -> u32),
        entry!(
            0x004b5040,
            bhk_mouse_spring_action_bhk_mouse_spring_action(
                Ptr<BhkSerializable>,
                u32,
            ) -> Ptr<BhkSerializable>
        ),
        entry!(
            0x004b50c0,
            fn_004b50c0(Ptr<BhkSerializable>) -> Ptr<BhkSerializable>
        ),
        entry!(
            0x004b50f0,
            fn_004b50f0(Ptr<BhkSerializable>) -> Ptr<BhkSerializable>
        ),
        entry!(
            0x004b5120,
            fn_004b5120(Ptr<BhkSerializable>) -> Ptr<BhkSerializable>
        ),
        entry!(0x004b5150, bhk_serializable_get_rtti(Ptr) -> u32),
        entry!(0x004b5160, bhk_serializable_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x004b5190, bhk_action_get_rtti(Ptr) -> u32),
        entry!(0x004b51a0, bhk_action_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x004b51d0, bhk_unary_action_get_rtti(Ptr) -> u32),
        entry!(0x004b51e0, bhk_unary_action_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x004b5210, bhk_mouse_spring_action_get_rtti(Ptr) -> u32),
        entry!(0x004b5220, fn_004b5220(Ptr) -> u32),
        entry!(
            0x004b5230,
            bhk_mouse_spring_action_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x004b5260, find_first_collision_object(Ptr) -> u32),
        entry!(0x004b52f0, fn_004b52f0(Ptr) -> u32),
        entry!(0x004b5400, fn_004b5400(Ptr) -> f32),
        entry!(0x004b5440, fn_004b5440(Ptr, Ptr, f32) -> i32),
        entry!(0x004b5470, fn_004b5470(Ptr, f32) -> i32),
        entry!(0x004b54c0, ni_a_sin(f32) -> f64),
        entry!(0x004b5820, get_av_object_for_collidable(Ptr) -> u32),
        entry!(0x004b5950, fn_004b5950(Ptr) -> u32),
        entry!(0x004b5980, fn_004b5980(Ptr) -> i32),
        entry!(0x004b59a0, fn_004b59a0(Ptr) -> i32),
        entry!(0x004b59c0, fn_004b59c0(Ptr) -> u32),
        entry!(0x004b59d0, fn_004b59d0(Ptr) -> u32),
        entry!(0x004b59f0, fn_004b59f0(Ptr) -> u32),
        entry!(0x004b5a20, fn_004b5a20(Ptr) -> u32),
        entry!(0x004b5a50, fn_004b5a50(Ptr) -> u32),
        entry!(0x004b5a80, fn_004b5a80(Ptr) -> u32),
        entry!(0x004b5aa0, fn_004b5aa0(Ptr) -> u32),
        entry!(0x004b5ad0, fn_004b5ad0(Ptr) -> u32),
        entry!(0x004b5b00, fn_004b5b00(Ptr, u16) -> u32),
        entry!(0x004b5b20, remove_scabard(Ptr) -> bool),
        entry!(0x004b5bf0, has_morpher_controller(Ptr) -> bool),
        entry!(0x004b5c80, fn_004b5c80(Ptr) -> bool),
        entry!(0x004b5d10, remove_editor_markers(Ptr) -> bool),
        entry!(0x004b5f60, fn_004b5f60() -> u32),
        entry!(0x004b5f70, fn_004b5f70() -> u32),
        entry!(0x004b5f80, fn_004b5f80() -> u32),
        entry!(0x004b5f90, fn_004b5f90() -> u32),
        entry!(0x004b5fa0, fn_004b5fa0(Ptr) -> bool),
        entry!(0x004b5fc0, fn_004b5fc0(Ptr, Ptr) -> bool),
        entry!(0x004b5ff0, fn_004b5ff0(Ptr, Ptr) -> bool),
        entry!(0x004b6100, ni_bound_which_side(Ptr, Ptr) -> i32),
        entry!(0x004b6160, fn_004b6160(Ptr, Ptr) -> f32),
        entry!(0x004b6360, tint_scenegraph(Ptr, Ptr, Ptr)),
        entry!(0x004b6630, un_tint_scenegraph(Ptr)),
        entry!(
            0x004b66d0,
            get_collision_object_count_in_scene_graph(Ptr, bool, bool, bool) -> u32
        ),
        entry!(0x004b6740, fn_004b6740(Ptr, Ptr)),
        entry!(0x004b6810, fn_004b6810(Ptr, u32, bool, bool, bool) -> i32),
        entry!(0x004b68a0, fn_004b68a0(Ptr, Ptr)),
        entry!(0x004b6980, fn_004b6980(Ptr, u32, bool, bool, bool) -> u32),
        entry!(0x004b6a00, fn_004b6a00(Ptr, Ptr)),
        entry!(
            0x004b6ae0,
            get_havok_world_object_count_in_scene_graph(Ptr) -> u32
        ),
        entry!(0x004b6b20, fn_004b6b20(Ptr, Ptr)),
        entry!(0x004b6b60, fn_004b6b60(Ptr, u32) -> i32),
        entry!(0x004b6bc0, fn_004b6bc0(Ptr, Ptr)),
        entry!(0x004b6c20, fn_004b6c20(Ptr, u32) -> u32),
        entry!(0x004b6c70, fn_004b6c70(Ptr, Ptr)),
        entry!(0x004b6ce0, fn_004b6ce0(Ptr)),
        entry!(0x004b6dc0, set_reference_id_on_scenegraph(Ptr, u32)),
        entry!(0x004b6e70, fn_004b6e70(Ptr) -> u32),
        entry!(0x004b6e90, add_no_lighting_property_recurse(Ptr)),
        entry!(0x004b6f90, prepare_object_with_no_lighting_recurse(Ptr)),
        entry!(0x004b70d0, fn_004b70d0(Ptr) -> u32),
        entry!(0x004b70f0, fn_004b70f0(Ptr, f32)),
        entry!(0x004b71a0, fn_004b71a0() -> bool),
        entry!(0x004b71d0, fn_004b71d0() -> bool),
        entry!(0x004b7210, fn_004b7210() -> u32),
        entry!(0x004b7220, get_current_tex_language() -> u32),
        entry!(
            0x004b7240,
            swap_platform_language_texture_path(Ptr, Ptr, u32) -> bool
        ),
        entry!(0x004b7580, fn_004b7580(Ptr, u32, Ptr, u32) -> u32),
        entry!(0x004b75a0, bs_stristr(Ptr, Ptr) -> u32),
        entry!(0x004b7650, fn_004b7650(i32) -> u32),
        entry!(0x004b7660, swap_platform_language_textures(Ptr) -> bool),
        entry!(0x004b7920, fixed_strings_init_sdm()),
        entry!(0x004b8cf0, fn_004b8cf0()),
        entry!(0x004b9900, fn_004b9900(Ptr, u32) -> Ptr),
        entry!(0x004b9930, fn_004b9930(Ptr) -> bool),
        entry!(0x004b9950, fn_004b9950(Ptr<NiTMap>, u32) -> Ptr<NiTMap>),
        entry!(
            0x004b9980,
            ni_t_map_vertex_dist_scalar_deleting_destructor(Ptr<NiTMap>, u32) -> Ptr<NiTMap>
        ),
        entry!(0x004b99b0, fn_004b99b0(Ptr<NiTMap>, u32) -> Ptr<NiTMap>),
        entry!(0x004b9a20, fn_004b9a20(Ptr<NiTMap>, u32, u32, u32, u32)),
        entry!(0x004b9b20, fn_004b9b20(Ptr<NiTMap>, u32, Ptr) -> bool),
        entry!(0x004b9ba0, fn_004b9ba0(Ptr<NiTMap>) -> u32),
        entry!(0x004b9bf0, fn_004b9bf0(Ptr<NiTMap>, Ptr, Ptr, Ptr)),
        entry!(
            0x004b9ca0,
            ni_t_map_base_vertex_dist_set_value(
                Ptr<NiTMap>,
                Ptr<VertexDistMapItem>,
                u32,
                u32,
                u32,
                u32,
            )
        ),
        entry!(0x004b9cd0, fn_004b9cd0(Ptr<NiTMap>)),
        entry!(0x004b9d30, fn_004b9d30(Ptr<NiTMap>)),
        entry!(
            0x004b9d60,
            ni_t_map_vertex_dist_new_item(Ptr<NiTMap>) -> u32
        ),
        entry!(0x004b9d80, fn_004b9d80(Ptr<NiTMap>, u32)),
        entry!(
            0x004b9db0,
            ni_t_map_base_vertex_dist_scalar_deleting_destructor(Ptr<NiTMap>, u32) -> Ptr<NiTMap>
        ),
        entry!(0x004b9de0, fn_004b9de0(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::BTreeMap;
    use std::rc::Rc;

    fn p(addr: u32) -> Ptr {
        Ptr::new(addr)
    }

    fn ret_u(v: u32) -> Ret {
        Ret {
            eax: v,
            ..Ret::default()
        }
    }

    fn ret_f(v: f64) -> Ret {
        Ret {
            st0: v,
            ..Ret::default()
        }
    }

    fn arg_f(a: &[u32], i: usize) -> f32 {
        f32::from_bits(a[i])
    }

    /// An engine with the pages and constants of the exe's data this file
    /// reads, as the exe holds them.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_2000,
            0x0101_6000,
            0x0101_7000,
            0x0101_a000,
            0x011d_e000,
            0x0101_f000,
            0x0118_7000,
            0x0118_8000,
            0x0118_9000,
            0x0118_a000,
            0x011c_6000,
            0x0126_8000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(ONE, 1.0f64);
        e.set_global(NEGATIVE_ONE, -1.0f32);
        e.set_global(FLOAT_MAX, f32::MAX);
        e.set_global(THREE_QUARTER_TURN, 4.712389f32);
        e.set_global(QUARTER_TURN, 1.5707964f32);
        e.set_global(HALF_TURN, 3.1415927410125732f64);
        e.set_global(FULL_TURN, 6.2831854820251465f64);
        e.set_global(FULL_TURN_FLOAT, 6.2831855f32);
        e.set_global(NEGATIVE_HALF_TURN, -3.1415927410125732f64);
        e.set_global(SQUARE_ROOT_OF_TWO, 1.4142135381698608f64);
        e.set_global(SINGULAR_DETERMINANT, 9.99999974752427e-7f64);
        e.set_global(SNAP_TOLERANCE, 0.001f32);
        e.set_global(DEBUG_OFFSET, 10.0f32);
        e.set_global(NEGATIVE_ONE_DOUBLE, -1.0f64);
        e
    }

    fn text(e: &mut Engine, s: &str) -> Ptr {
        let block = e.mem.alloc(s.len() as u32 + 1);
        e.mem.set_cstr(block, s.as_bytes());
        p(block)
    }

    /// A block holding the given floats.
    fn floats(e: &mut Engine, values: &[f32]) -> u32 {
        let p = e.mem.alloc(4 * values.len() as u32);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(p + 4 * i as u32, *v);
        }
        p
    }

    fn read_floats(e: &Engine, p: u32, n: u32) -> Vec<f32> {
        (0..n).map(|i| e.mem.f32(p + 4 * i)).collect()
    }

    fn assert_close(actual: &[f32], expected: &[f32]) {
        assert_eq!(actual.len(), expected.len());
        for (a, b) in actual.iter().zip(expected) {
            assert!((a - b).abs() < 1e-5, "{actual:?} != {expected:?}");
        }
    }

    /// Stand-ins for the `NiPoint3` helpers other units own.
    fn point_doubles(e: &mut Engine) {
        e.register(POINT3_CONSTRUCT, |e, a| {
            for i in 0..3 {
                e.mem.set_f32(a[0] + 4 * i as u32, arg_f(a, 1 + i));
            }
            ret_u(a[0])
        });
        e.register(POINT3_EMPTY_CONSTRUCTOR, |_, a| ret_u(a[0]));
        e.register(POINT3_SUBTRACT, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) - e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            ret_u(a[1])
        });
        e.register(POINT3_ADD, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) + e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            ret_u(a[1])
        });
        e.register(POINT3_LENGTH, |e, a| {
            let sum: f64 = (0..3)
                .map(|i| (e.mem.f32(a[0] + 4 * i) as f64).powi(2))
                .sum();
            ret_f(sum.sqrt())
        });
        e.register(POINT3_UNITIZE, |e, a| {
            let v: Vec<f32> = (0..3).map(|i| e.mem.f32(a[0] + 4 * i)).collect();
            let length = v.iter().map(|x| x * x).sum::<f32>().sqrt();
            for (i, x) in v.iter().enumerate() {
                e.mem.set_f32(a[0] + 4 * i as u32, x / length);
            }
            Ret::default()
        });
        e.register(POINT3_SCALED, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[2] + 4 * i) * arg_f(a, 1);
                e.mem.set_f32(a[0] + 4 * i, v);
            }
            ret_u(a[0])
        });
        e.register(POINT3_ADD_ASSIGN, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) + e.mem.f32(a[1] + 4 * i);
                e.mem.set_f32(a[0] + 4 * i, v);
            }
            ret_u(a[0])
        });
    }

    fn math_doubles(e: &mut Engine) {
        e.register(CRT_ARC_TANGENT, |_, a| ret_f(f64::take(a, &mut 0).atan()));
        e.register(CRT_FMOD, |_, a| {
            let mut i = 0;
            let x = f64::take(a, &mut i);
            let y = f64::take(a, &mut i);
            ret_f(x % y)
        });
        e.register(FLOAT_ABSOLUTE_VALUE, |_, a| ret_f(arg_f(a, 0).abs() as f64));
    }

    fn take_log(e: &mut Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log.take().unwrap_or_default()
    }

    fn calls_to(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    // ---- strings ----

    #[test]
    fn is_an_integer_accepts_digits_with_an_optional_minus() {
        let mut e = engine();
        for (s, want) in [
            ("0", true),
            ("123", true),
            ("-5", true),
            ("-", false),
            ("", false),
            ("12a", false),
            ("1.5", false),
            ("+1", false),
            ("1-", false),
        ] {
            let p = text(&mut e, s);
            assert_eq!(e.call(0x004b_1230, &args![p]).bool(), want, "{s:?}");
        }
    }

    #[test]
    fn is_a_float_accepts_one_point_and_a_leading_minus() {
        let mut e = engine();
        for (s, want) in [
            ("1.5", true),
            ("-.5", true),
            ("5.", true),
            (".5", true),
            ("-0.25", true),
            ("7", true),
            (".", false),
            ("-", false),
            ("", false),
            ("1.2.3", false),
            ("1-2", false),
            ("..", false),
            ("a", false),
        ] {
            let p = text(&mut e, s);
            assert_eq!(e.call(0x004b_12d0, &args![p]).bool(), want, "{s:?}");
        }
    }

    // ---- angles ----

    fn vector(e: &mut Engine, x: f32, y: f32) -> Ptr<NiPoint3> {
        Ptr::new(floats(e, &[x, y, 0.0]))
    }

    #[test]
    fn z_angle_uses_the_arc_tangent_and_half_a_turn_below_the_axis() {
        let mut e = engine();
        math_doubles(&mut e);
        let v = vector(&mut e, 1.0, 1.0);
        assert_close(
            &[e.call(0x004b_1550, &args![v]).f32()],
            &[std::f32::consts::FRAC_PI_4],
        );
        let v = vector(&mut e, 1.0, -1.0);
        assert_close(&[e.call(0x004b_1550, &args![v]).f32()], &[2.356194]);
        // The unclamped form keeps the negative angle.
        let v = vector(&mut e, -1.0, 1.0);
        assert_close(
            &[e.call(0x004b_1550, &args![v]).f32()],
            &[-std::f32::consts::FRAC_PI_4],
        );
    }

    #[test]
    fn z_angle_on_the_x_axis_is_a_quarter_or_three_quarters_of_a_turn() {
        let mut e = engine();
        math_doubles(&mut e);
        e.call_log = Some(vec![]);
        for (x, want) in [(2.0, 1.5707964), (-2.0, 4.712389), (0.0, 4.712389)] {
            let v = vector(&mut e, x, 0.0);
            assert_close(&[e.call(0x004b_1550, &args![v]).f32()], &[want]);
            assert_close(&[e.call(0x004b_13c0, &args![v]).f32()], &[want]);
        }
        // No arc tangent was needed.
        let log = take_log(&mut e);
        assert!(calls_to(&log, CRT_ARC_TANGENT).is_empty());
    }

    #[test]
    fn z_angle_clamps_into_a_turn() {
        let mut e = engine();
        math_doubles(&mut e);
        let v = vector(&mut e, -1.0, 1.0);
        assert_close(&[e.call(0x004b_13c0, &args![v]).f32()], &[5.497787]);
        let v = vector(&mut e, 1.0, 1.0);
        assert_close(
            &[e.call(0x004b_13c0, &args![v]).f32()],
            &[std::f32::consts::FRAC_PI_4],
        );
    }

    #[test]
    fn the_float_arc_tangent_wrapper_calls_the_crt_with_a_double() {
        let mut e = engine();
        math_doubles(&mut e);
        e.call_log = Some(vec![]);
        let r = e.call(0x004b_1460, &args![1.0f32]).f32();
        assert_close(&[r], &[std::f32::consts::FRAC_PI_4]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, CRT_ARC_TANGENT), vec![args![1.0f64]]);
    }

    #[test]
    fn clamp_angle_wraps_negative_and_large_angles_only() {
        let mut e = engine();
        math_doubles(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004b_1480, &args![1.0f32]).f32(), 1.0);
        assert_close(&[e.call(0x004b_1480, &args![-1.0f32]).f32()], &[5.2831855]);
        assert_close(&[e.call(0x004b_1480, &args![7.0f32]).f32()], &[0.7168147]);
        let log = take_log(&mut e);
        // The remainder was taken for the last two only.
        assert_eq!(calls_to(&log, CRT_FMOD).len(), 2);
        assert_eq!(
            calls_to(&log, CRT_FMOD)[1],
            args![7.0f64, 6.2831855f32 as f64]
        );
    }

    #[test]
    fn the_remainder_wrappers_forward_to_fmod() {
        let mut e = engine();
        math_doubles(&mut e);
        assert_eq!(e.call(0x004b_1500, &args![7.5f32, 2.0f32]).f32(), 1.5);
        assert_eq!(e.call(0x004b_1520, &args![-7.5f32, 2.0f32]).f32(), -1.5);
    }

    #[test]
    fn angle_difference_gives_magnitude_and_direction() {
        let mut e = engine();
        let direction = p(e.mem.alloc(4));
        let turn = |e: &mut Engine, from: f32, to: f32| {
            let r = e.call(0x004b_15e0, &args![from, to, direction]).f32();
            (r, e.mem.f32(direction.addr()))
        };
        assert_eq!(turn(&mut e, 1.0, 1.0), (0.0, 0.0));
        assert_eq!(turn(&mut e, 0.0, 1.0), (1.0, 1.0));
        // Backwards by less than half a turn: negative, direction -1.
        assert_eq!(turn(&mut e, 1.0, 0.0), (-1.0, -1.0));
        // Forwards by more than half a turn: the short way round, -1.
        let (magnitude, sign) = turn(&mut e, 0.0, 4.0);
        assert_close(&[magnitude], &[2.2831855]);
        assert_eq!(sign, -1.0);
        // Backwards by more than half a turn: wraps, direction 1.
        let (magnitude, sign) = turn(&mut e, 4.0, 0.0);
        assert_close(&[magnitude], &[2.2831855]);
        assert_eq!(sign, 1.0);
    }

    // ---- form type lookups ----

    #[test]
    fn form_type_index_follows_the_switch() {
        let mut e = engine();
        for (form_type, want) in [
            (3u8, -1),
            (4, 36),
            (5, 38),
            (6, -1),
            (7, 52),
            (0xb, 54),
            (0xc, 53),
            (0x10, 70),
            (0x75, 86),
            (0x76, -1),
            (0xff, -1),
        ] {
            assert_eq!(
                e.call(0x004b_1680, &args![form_type]).i32(),
                want,
                "{form_type:#x}"
            );
        }
    }

    #[test]
    fn form_type_name_prefers_the_index_then_the_slot_then_the_table() {
        let mut e = engine();
        // Type 4: index 36 into the name table.
        e.set_global(FORM_NAME_BY_INDEX + 4 * 36, 0xaaaa_0001u32);
        assert_eq!(e.call(0x004b_1b60, &args![4u8]).u32(), 0xaaaa_0001);
        // Type 3: no index, its own slot.
        e.set_global(0x0118_8fbcu32, 0xbbbb_0003u32);
        assert_eq!(e.call(0x004b_1b60, &args![3u8]).u32(), 0xbbbb_0003);
        // Type 0x75: index 86 wins over its slot.
        e.set_global(FORM_NAME_BY_INDEX + 4 * 86, 0xcccc_0075u32);
        e.set_global(0x0118_9034u32, 0xdddd_0075u32);
        assert_eq!(e.call(0x004b_1b60, &args![0x75u8]).u32(), 0xcccc_0075);
        // Type 1: the 12-byte table.
        e.set_global(FORM_NAME_BY_TYPE + 12, 0xeeee_0001u32);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004b_1b60, &args![1u8]).u32(), 0xeeee_0001);
        // Type 0x78 is the last table entry and logs nothing.
        e.set_global(FORM_NAME_BY_TYPE + 12 * 0x78, 0xeeee_0078u32);
        assert_eq!(e.call(0x004b_1b60, &args![0x78u8]).u32(), 0xeeee_0078);
        assert!(calls_to(&take_log(&mut e), LOG_MESSAGE).is_empty());
    }

    #[test]
    fn form_type_name_logs_an_unknown_type_and_uses_entry_zero() {
        let mut e = engine();
        e.set_global(FORM_NAME_BY_TYPE, 0x1111_0000u32);
        e.set_global(FORM_NAME_BY_TYPE + 12 * 0x7a, 0x2222_007au32);
        e.call_log = Some(vec![]);
        e.register(LOG_MESSAGE, |_, _| Ret::default());
        assert_eq!(e.call(0x004b_1b60, &args![0x7au8]).u32(), 0x1111_0000);
        let log = take_log(&mut e);
        assert_eq!(
            calls_to(&log, LOG_MESSAGE),
            vec![args![INVALID_FORM_ID_MESSAGE, 0x2222_007au32]]
        );
    }

    // ---- segment against rectangle ----

    /// Runs the clipper on `rect` = `(x min, y max, x max, y min)`.
    fn clip(e: &mut Engine, rect: [i32; 4], from: [f32; 3], to: [f32; 2]) -> (bool, Vec<f32>) {
        let r = e.mem.alloc(16);
        for (i, v) in rect.iter().enumerate() {
            e.mem.set_i32(r + 4 * i as u32, *v);
        }
        let from = floats(e, &from);
        let to = floats(e, &[to[0], to[1], 0.0]);
        let out = floats(e, &[7.0, 7.0, 7.0]);
        let found = e
            .call(0x004b_1e80, &args![p(r), p(from), p(to), p(out)])
            .bool();
        (found, read_floats(e, out, 3))
    }

    /// The rectangle `x 0..10`, `y 0..10`.
    const RECT: [i32; 4] = [0, 10, 10, 0];

    #[test]
    fn clip_rejects_an_empty_rectangle() {
        let mut e = engine();
        point_doubles(&mut e);
        let (found, out) = clip(&mut e, [10, 10, 0, 0], [5.0, 5.0, 0.0], [20.0, 5.0]);
        assert!(!found);
        assert_eq!(out, vec![7.0; 3]);
        let (found, _) = clip(&mut e, [0, 0, 10, 10], [5.0, 5.0, 0.0], [20.0, 5.0]);
        assert!(!found);
    }

    #[test]
    fn clip_rejects_segments_beside_an_edge_or_wholly_inside() {
        let mut e = engine();
        point_doubles(&mut e);
        for (from, to) in [
            ([-5.0, 5.0, 0.0], [-1.0, 5.0]),
            ([15.0, 5.0, 0.0], [11.0, 9.0]),
            ([5.0, -5.0, 0.0], [3.0, -1.0]),
            ([5.0, 15.0, 0.0], [2.0, 12.0]),
            // Both strictly inside.
            ([2.0, 2.0, 0.0], [8.0, 8.0]),
        ] {
            let (found, out) = clip(&mut e, RECT, from, to);
            assert!(!found, "{from:?} {to:?}");
            assert_eq!(out, vec![7.0; 3]);
        }
    }

    #[test]
    fn clip_of_a_single_point_gives_the_point_with_its_z() {
        let mut e = engine();
        point_doubles(&mut e);
        // A point strictly inside is "wholly inside": nothing to clip.
        let (found, out) = clip(&mut e, RECT, [3.0, 4.0, 9.0], [3.0, 4.0]);
        assert!(!found);
        assert_eq!(out, vec![7.0; 3]);
        // The border counts as inside.
        let (found, out) = clip(&mut e, RECT, [10.0, 10.0, 1.0], [10.0, 10.0]);
        assert!(found);
        assert_eq!(out, vec![10.0, 10.0, 1.0]);
    }

    #[test]
    fn clip_of_a_vertical_segment_picks_the_edge_by_direction() {
        let mut e = engine();
        point_doubles(&mut e);
        // Upwards (y grows): the y min edge.
        let (found, out) = clip(&mut e, RECT, [5.0, 5.0, 3.0], [5.0, 15.0]);
        assert!(found);
        assert_eq!(out, vec![5.0, 0.0, 0.0]);
        // Downwards: the y max edge.
        let (found, out) = clip(&mut e, RECT, [5.0, 5.0, 3.0], [5.0, -5.0]);
        assert!(found);
        assert_eq!(out, vec![5.0, 10.0, 0.0]);
        // A vertical line beside the rectangle is rejected before this.
        let (found, _) = clip(&mut e, RECT, [11.0, 5.0, 0.0], [11.0, 6.0]);
        assert!(!found);
    }

    #[test]
    fn clip_of_a_horizontal_segment_picks_the_edge_by_direction() {
        let mut e = engine();
        point_doubles(&mut e);
        let (found, out) = clip(&mut e, RECT, [5.0, 5.0, 0.0], [-5.0, 5.0]);
        assert!(found);
        assert_eq!(out, vec![10.0, 5.0, 0.0]);
        let (found, out) = clip(&mut e, RECT, [5.0, 5.0, 0.0], [15.0, 5.0]);
        assert!(found);
        assert_eq!(out, vec![0.0, 5.0, 0.0]);
    }

    #[test]
    fn clip_of_a_slanted_segment_from_inside_chooses_by_the_unit_vector_sum() {
        let mut e = engine();
        point_doubles(&mut e);
        // The line y = x / 2 + 1 crosses the left edge at (0, 1) and the right
        // edge at (10, 6). `from` is strictly inside; the unit vectors along
        // the segment and to the first hit cancel, so the second hit is chosen.
        let (found, out) = clip(&mut e, RECT, [2.0, 2.0, 0.0], [12.0, 7.0]);
        assert!(found);
        assert_close(&out, &[10.0, 6.0, 0.0]);
        // Heading for the first hit the unit vectors add up to more than one
        // long: the first hit.
        let (found, out) = clip(&mut e, RECT, [8.0, 5.0, 0.0], [-4.0, -1.0]);
        assert!(found);
        assert_close(&out, &[0.0, 1.0, 0.0]);
    }

    #[test]
    fn clip_of_a_slanted_segment_from_outside_chooses_the_nearer_hit() {
        let mut e = engine();
        point_doubles(&mut e);
        // y = x + 5 hits the left edge at (0, 5) and the top at (5, 10).
        let (found, out) = clip(&mut e, RECT, [-2.0, 3.0, 0.0], [4.0, 9.0]);
        assert!(found);
        assert_close(&out, &[0.0, 5.0, 0.0]);
        // y = 11 - x / 2 hits the top at (2, 10) and the right at (10, 6);
        // from (12, 5) the second is nearer.
        let (found, out) = clip(&mut e, RECT, [12.0, 5.0, 0.0], [0.0, 11.0]);
        assert!(found);
        assert_close(&out, &[10.0, 6.0, 0.0]);
        // A line that misses the rectangle.
        let (found, _) = clip(&mut e, RECT, [-5.0, 12.0, 0.0], [5.0, 20.0]);
        assert!(!found);
    }

    // ---- picking ----

    layout! {
        /// A test node: the vtable word, then the world bound pointer.
        struct TestNode: 0x40 {
            0x20 bound: u32,
        }
    }

    fn bound(e: &mut Engine, centre: [f32; 3], radius: f32) -> u32 {
        floats(e, &[centre[0], centre[1], centre[2], radius])
    }

    #[test]
    fn a_node_has_a_bound_when_its_radius_is_not_zero() {
        let mut e = engine();
        e.register(BOUND_RADIUS, |e, a| ret_f(e.mem.f32(a[0] + 0xc) as f64));
        let node: Ptr<TestNode> = e.new_object();
        assert!(!e.call(0x004b_2960, &args![node]).bool());
        let b = bound(&mut e, [0.0; 3], 0.0);
        e.set(node, TestNode::bound, b);
        assert!(!e.call(0x004b_2960, &args![node]).bool());
        let b = bound(&mut e, [0.0; 3], 2.5);
        e.set(node, TestNode::bound, b);
        assert!(e.call(0x004b_2960, &args![node]).bool());
    }

    struct PickWorld {
        e: Engine,
        object: Ptr,
        origin: Ptr,
        direction: Ptr,
        root: Ptr<TestNode>,
        leaves: Vec<Ptr<TestNode>>,
        picked: Rc<RefCell<Vec<u32>>>,
    }

    /// A root node with a container of two leaves, each with a bound of
    /// radius 1 at the origin; picking succeeds for the leaves in `hits`.
    fn pick_world(blocked: bool, hits: Vec<usize>) -> PickWorld {
        let mut e = engine();
        e.register(BOUND_RADIUS, |e, a| ret_f(e.mem.f32(a[0] + 0xc) as f64));
        point_doubles(&mut e);
        e.register(GET_WORLD_BOUND, |e, a| ret_u(e.mem.u32(a[0] + 0x20)));
        e.register(PICK_BLOCKED_A, |_, _| ret_u(1));
        if blocked {
            e.register(PICK_BLOCKED_B, |_, _| ret_u(1));
        } else {
            e.register(PICK_BLOCKED_B, |_, _| ret_u(0));
        }
        let container = e.mem.alloc(8);
        e.register(CHILD_COUNT, |_, _| ret_u(2));
        e.register_double(CHILD_AT, move |e, a| {
            assert_eq!(a[0], container);
            ret_u(e.mem.u32(container + 4 * a[1]))
        });
        // Virtual slot 0xc: the root returns the container, leaves null.
        let root_table = 0x0300_0000;
        let leaf_table = 0x0300_1000;
        e.put_vtable(root_table, &[0, 0, 0, 0x0300_2000]);
        e.put_vtable(leaf_table, &[0, 0, 0, 0x0300_2010]);
        e.register_double(0x0300_2000, move |_, _| ret_u(container));
        e.register(0x0300_2010, |_, _| ret_u(0));
        let make = |e: &mut Engine, table: u32, radius: f32| {
            let node: Ptr<TestNode> = e.new_object();
            e.mem.set_u32(node.addr(), table);
            let b = bound(e, [0.0; 3], radius);
            e.set(node, TestNode::bound, b);
            node
        };
        let root = make(&mut e, root_table, 1.0);
        let leaves = vec![make(&mut e, leaf_table, 1.0), make(&mut e, leaf_table, 1.0)];
        e.mem.set_u32(container, leaves[0].addr());
        e.mem.set_u32(container + 4, leaves[1].addr());
        let picked = Rc::new(RefCell::new(Vec::new()));
        let last_root = Rc::new(RefCell::new(0u32));
        {
            let last_root = last_root.clone();
            e.register_double(PICK_SET_ROOT, move |_, a| {
                *last_root.borrow_mut() = a[1];
                Ret::default()
            });
        }
        {
            let picked = picked.clone();
            let leaf_addrs: Vec<u32> = leaves.iter().map(|l| l.addr()).collect();
            e.register_double(PICK_OBJECTS, move |_, a| {
                let root = *last_root.borrow();
                picked.borrow_mut().push(root);
                let hit = hits.iter().any(|&i| leaf_addrs[i] == root) && a[3] == 1;
                ret_u(hit as u32)
            });
        }
        let object = p(e.mem.alloc(16));
        let origin = p(floats(&mut e, &[0.0, 0.0, 0.0]));
        let direction = p(floats(&mut e, &[1.0, 0.0, 0.0]));
        PickWorld {
            e,
            object,
            origin,
            direction,
            root,
            leaves,
            picked,
        }
    }

    #[test]
    fn picking_recurses_over_the_children_and_picks_the_leaves() {
        let mut w = pick_world(false, vec![1]);
        let r =
            w.e.call(
                0x004b_2800,
                &args![w.object, w.origin, w.direction, 5.0f32, w.root],
            )
            .bool();
        assert!(r);
        // Both leaves were picked against, in order.
        let expected: Vec<u32> = w.leaves.iter().map(|l| l.addr()).collect();
        assert_eq!(*w.picked.borrow(), expected);
    }

    #[test]
    fn picking_without_a_hit_is_false() {
        let mut w = pick_world(false, vec![]);
        let r =
            w.e.call(
                0x004b_2800,
                &args![w.object, w.origin, w.direction, 5.0f32, w.root],
            )
            .bool();
        assert!(!r);
        assert_eq!(w.picked.borrow().len(), 2);
    }

    #[test]
    fn picking_skips_nodes_too_far_from_the_ray_origin() {
        let mut w = pick_world(false, vec![0, 1]);
        // The bound's centre is 100 away from the ray origin; radius 1 and
        // distance 5 do not reach it.
        let far = p(floats(&mut w.e, &[100.0, 0.0, 0.0]));
        let r =
            w.e.call(
                0x004b_2800,
                &args![w.object, far, w.direction, 5.0f32, w.root],
            )
            .bool();
        assert!(!r);
        assert!(w.picked.borrow().is_empty());
    }

    #[test]
    fn picking_stops_at_a_node_both_checks_block() {
        let mut w = pick_world(true, vec![0, 1]);
        let r =
            w.e.call(
                0x004b_2800,
                &args![w.object, w.origin, w.direction, 5.0f32, w.root],
            )
            .bool();
        assert!(!r);
        assert!(w.picked.borrow().is_empty());
        // Null arguments are refused before any call.
        w.e.call_log = Some(vec![]);
        assert!(!w
            .e
            .call(
                0x004b_2800,
                &args![Ptr::<()>::NULL, w.origin, w.direction, 5.0f32, w.root]
            )
            .bool());
        assert_eq!(take_log(&mut w.e).len(), 1);
    }

    #[test]
    fn the_pick_wrapper_checks_its_arguments_and_can_reset_the_state() {
        let mut w = pick_world(false, vec![0]);
        w.e.call_log = Some(vec![]);
        for (object, distance, node) in [
            (Ptr::<()>::NULL, 5.0f32, w.root),
            (w.object, 0.0, w.root),
            (w.object, 5.0, Ptr::<TestNode>::NULL),
        ] {
            let r =
                w.e.call(
                    0x004b_2780,
                    &args![object, w.origin, w.direction, distance, node, true],
                )
                .bool();
            assert!(!r);
        }
        let log = take_log(&mut w.e);
        assert_eq!(log.len(), 3, "only the three top-level calls");
        // Without reset: straight to the recursive picker.
        let r =
            w.e.call(
                0x004b_2780,
                &args![w.object, w.origin, w.direction, 5.0f32, w.root, false],
            )
            .bool();
        assert!(r);
        // With reset: the state is cleared first.
        w.e.call_log = Some(vec![]);
        w.e.call(
            0x004b_2780,
            &args![w.object, w.origin, w.direction, 5.0f32, w.root, true],
        );
        let log = take_log(&mut w.e);
        assert_eq!(calls_to(&log, PICK_SET_ROOT)[0], args![w.object, 0u32]);
        assert_eq!(
            calls_to(&log, PICK_OBJECTS)[0],
            args![w.object, PICK_STATE, PICK_STATE, 0u32]
        );
    }

    // ---- debug shape builders ----

    fn shape_engine() -> Engine {
        let mut e = engine();
        point_doubles(&mut e);
        e.register(OPERATOR_NEW, |e, a| ret_u(e.mem.alloc(a[0])));
        e.register(NI_OPERATOR_NEW, |e, a| ret_u(e.mem.alloc(a[0])));
        e.register(NI_ARRAY_NEW, |e, a| ret_u(e.mem.alloc(a[0])));
        e.register(VECTOR_CONSTRUCT, |_, _| Ret::default());
        e.register(NI_TRI_SHAPE_CONSTRUCT, |_, a| ret_u(a[0]));
        e.register(NI_LINES_CONSTRUCT, |_, a| ret_u(a[0]));
        e.register(NO_LIGHTING_PROPERTY_CONSTRUCT, |_, a| ret_u(a[0]));
        e.register(ATTACH_PROPERTY, |_, _| Ret::default());
        e.register(PREPARE_OBJECT, |_, _| Ret::default());
        e
    }

    fn color(e: &mut Engine) -> Ptr {
        p(floats(e, &[1.0, 0.5, 0.25, 0.75]))
    }

    fn words(e: &Engine, p: u32, n: u32) -> Vec<u32> {
        (0..n).map(|i| e.mem.u32(p + 4 * i)).collect()
    }

    fn color_words() -> Vec<u32> {
        [1.0f32, 0.5, 0.25, 0.75]
            .iter()
            .map(|v| v.to_bits())
            .collect()
    }

    #[test]
    fn make_tri_point_builds_a_bipyramid_over_the_colour() {
        let mut e = shape_engine();
        let c = color(&mut e);
        e.call_log = Some(vec![]);
        let shape = e.call(0x004b_29b0, &args![2.0f32, c, false]).u32();
        let log = take_log(&mut e);
        let ctor = &calls_to(&log, NI_TRI_SHAPE_CONSTRUCT)[0];
        // (this, vertex count, vertices, 0, colors, 0, 0, 0, triangle count, triangles)
        assert_eq!(ctor[0], shape);
        assert_eq!((ctor[1], ctor[3], ctor[8]), (6, 0, 8));
        assert_eq!(&ctor[5..8], &[0, 0, 0]);
        let apex = (2.0f64 * 1.4142135381698608) as f32;
        assert_close(
            &read_floats(&e, ctor[2], 18),
            &[
                0.0, 0.0, apex, -2.0, 2.0, 0.0, 2.0, 2.0, 0.0, 2.0, -2.0, 0.0, -2.0, -2.0, 0.0,
                0.0, 0.0, -apex,
            ],
        );
        for i in 0..6 {
            assert_eq!(words(&e, ctor[4] + 16 * i, 4), color_words());
        }
        let indices: Vec<u16> = (0..24).map(|i| e.mem.u16(ctor[9] + 2 * i)).collect();
        assert_eq!(indices, TRI_POINT_INDICES);
        // No property without the flag; the arrays came from the allocators.
        assert!(calls_to(&log, ATTACH_PROPERTY).is_empty());
        assert!(calls_to(&log, PREPARE_OBJECT).is_empty());
        assert_eq!(
            calls_to(&log, OPERATOR_NEW),
            vec![args![72u32], args![96u32]]
        );
        assert_eq!(calls_to(&log, NI_ARRAY_NEW), vec![args![48u32]]);
        assert_eq!(calls_to(&log, NI_OPERATOR_NEW), vec![args![0xc4u32]]);
        assert_eq!(
            calls_to(&log, VECTOR_CONSTRUCT)
                .iter()
                .map(|a| (a[1], a[2], a[3]))
                .collect::<Vec<_>>(),
            vec![(12, 6, 0x0068_15c0), (16, 6, 0x004a_7800)]
        );
    }

    #[test]
    fn making_a_shape_with_a_property_attaches_and_prepares_it() {
        let mut e = shape_engine();
        let c = color(&mut e);
        e.call_log = Some(vec![]);
        let shape = e.call(0x004b_29b0, &args![1.0f32, c, true]).u32();
        let log = take_log(&mut e);
        let property = calls_to(&log, NO_LIGHTING_PROPERTY_CONSTRUCT)[0][0];
        assert_eq!(
            calls_to(&log, NI_OPERATOR_NEW),
            vec![args![0xc4u32], args![0x80u32]]
        );
        assert_eq!(
            calls_to(&log, ATTACH_PROPERTY),
            vec![args![shape, property]]
        );
        assert_eq!(
            calls_to(&log, PREPARE_OBJECT),
            vec![args![shape, 0u32, 0u32]]
        );
    }

    #[test]
    fn a_failed_shape_allocation_gives_a_null_shape() {
        let mut e = shape_engine();
        e.register(NI_OPERATOR_NEW, |_, _| ret_u(0));
        let c = color(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004b_29b0, &args![1.0f32, c, false]).u32(), 0);
        assert!(calls_to(&take_log(&mut e), NI_TRI_SHAPE_CONSTRUCT).is_empty());
    }

    #[test]
    fn make_quad_box_copies_the_eight_corners() {
        let mut e = shape_engine();
        let c = color(&mut e);
        let corners: Vec<Ptr> = (0..8)
            .map(|i| {
                p(floats(
                    &mut e,
                    &[i as f32, 10.0 + i as f32, 20.0 + i as f32],
                ))
            })
            .collect();
        e.call_log = Some(vec![]);
        e.call(
            0x004b_2eb0,
            &args![
                corners[0], corners[1], corners[2], corners[3], corners[4], corners[5], corners[6],
                corners[7], c, false
            ],
        );
        let log = take_log(&mut e);
        let ctor = &calls_to(&log, NI_TRI_SHAPE_CONSTRUCT)[0];
        assert_eq!((ctor[1], ctor[8]), (8, 0x18));
        let want: Vec<f32> = (0..8)
            .flat_map(|i| [i as f32, 10.0 + i as f32, 20.0 + i as f32])
            .collect();
        assert_eq!(read_floats(&e, ctor[2], 24), want);
        for i in 0..8 {
            assert_eq!(words(&e, ctor[4] + 16 * i, 4), color_words());
        }
        let indices: Vec<u16> = (0..72).map(|i| e.mem.u16(ctor[9] + 2 * i)).collect();
        assert_eq!(indices, QUAD_BOX_INDICES);
        assert_eq!(calls_to(&log, NI_ARRAY_NEW), vec![args![144u32]]);
    }

    #[test]
    fn make_triangle_takes_its_vertices_by_value() {
        let mut e = shape_engine();
        let c = color(&mut e);
        e.call_log = Some(vec![]);
        e.call(
            0x004b_3570,
            &args![1.0f32, 2.0f32, 3.0f32, 4.0f32, 5.0f32, 6.0f32, 7.0f32, 8.0f32, 9.0f32, c, true],
        );
        let log = take_log(&mut e);
        let ctor = &calls_to(&log, NI_TRI_SHAPE_CONSTRUCT)[0];
        assert_eq!((ctor[1], ctor[8]), (3, 2));
        assert_eq!(
            read_floats(&e, ctor[2], 9),
            (1..=9).map(|i| i as f32).collect::<Vec<_>>()
        );
        for i in 0..3 {
            assert_eq!(words(&e, ctor[4] + 16 * i, 4), color_words());
        }
        let indices: Vec<u16> = (0..6).map(|i| e.mem.u16(ctor[9] + 2 * i)).collect();
        assert_eq!(indices, TRIANGLE_INDICES);
        assert_eq!(calls_to(&log, ATTACH_PROPERTY).len(), 1);
    }

    #[test]
    fn the_segment_builder_makes_a_two_vertex_line() {
        let mut e = shape_engine();
        let start = p(floats(&mut e, &[1.0, 2.0, 3.0]));
        let end = p(floats(&mut e, &[4.0, 5.0, 6.0]));
        let c0 = p(floats(&mut e, &[0.1, 0.2, 0.3, 0.4]));
        let c1 = p(floats(&mut e, &[0.5, 0.6, 0.7, 0.8]));
        e.call_log = Some(vec![]);
        e.call(0x004b_3890, &args![start, c0, end, c1, false]);
        let log = take_log(&mut e);
        // (this, count, vertices, colors, 0, 0, 0, flags)
        let ctor = &calls_to(&log, NI_LINES_CONSTRUCT)[0];
        assert_eq!(ctor[1], 2);
        assert_eq!(&ctor[4..7], &[0, 0, 0]);
        assert_eq!(
            read_floats(&e, ctor[2], 6),
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]
        );
        assert_eq!(
            read_floats(&e, ctor[3], 8),
            vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8]
        );
        assert_eq!((e.mem.u8(ctor[7]), e.mem.u8(ctor[7] + 1)), (1, 0));
        assert_eq!(
            calls_to(&log, OPERATOR_NEW),
            vec![args![24u32], args![32u32], args![2u32]]
        );
    }

    #[test]
    fn make_rectangle_outlines_the_square() {
        let mut e = shape_engine();
        let c = color(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x004b_3b90, &args![3.0f32, c, true]);
        let log = take_log(&mut e);
        let ctor = &calls_to(&log, NI_LINES_CONSTRUCT)[0];
        assert_eq!(ctor[1], 4);
        assert_eq!(
            read_floats(&e, ctor[2], 12),
            vec![-3.0, -3.0, 0.0, -3.0, 3.0, 0.0, 3.0, 3.0, 0.0, 3.0, -3.0, 0.0]
        );
        for i in 0..4 {
            assert_eq!(words(&e, ctor[3] + 16 * i, 4), color_words());
            assert_eq!(e.mem.u8(ctor[7] + i), 1);
        }
        // The arrays and the flags are allocated before the points are built.
        assert_eq!(
            calls_to(&log, OPERATOR_NEW),
            vec![args![48u32], args![64u32], args![4u32]]
        );
        assert_eq!(calls_to(&log, ATTACH_PROPERTY).len(), 1);
    }

    // ---- vectors and matrices ----

    #[test]
    fn the_cross_product_is_built_into_out() {
        let mut e = engine();
        point_doubles(&mut e);
        let a = floats(&mut e, &[1.0, 2.0, 3.0]);
        let b = floats(&mut e, &[4.0, 5.0, 6.0]);
        let out = e.mem.alloc(12);
        let r = e.call(0x004b_3800, &args![p(a), p(out), p(b)]);
        assert_eq!(r.u32(), out);
        // (2*6 - 3*5, 3*4 - 1*6, 1*5 - 2*4)
        assert_eq!(read_floats(&e, out, 3), vec![-3.0, 6.0, -3.0]);
    }

    #[test]
    fn interpolation_runs_a_to_b_over_t0_to_t1() {
        let mut e = engine();
        let r = |e: &mut Engine, t: f32| {
            e.call(0x004b_3ab0, &args![10.0f32, 20.0f32, 2.0f32, 4.0f32, t])
                .f32()
        };
        assert_eq!(r(&mut e, 2.0), 10.0);
        assert_eq!(r(&mut e, 3.0), 15.0);
        assert_eq!(r(&mut e, 4.0), 20.0);
        // Not clamped.
        assert_eq!(r(&mut e, 6.0), 30.0);
    }

    #[test]
    fn row_vector_times_matrix_and_matrix_times_vector_differ_by_transposition() {
        let mut e = engine();
        point_doubles(&mut e);
        let m = floats(&mut e, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
        let v = floats(&mut e, &[1.0, 2.0, 3.0]);
        let out = e.mem.alloc(12);
        let r = e.call(0x004b_3ae0, &args![p(out), p(v), p(m)]);
        assert_eq!(r.u32(), out);
        assert_eq!(read_floats(&e, out, 3), vec![30.0, 36.0, 42.0]);
        let r = e.call(0x004b_4500, &args![p(m), p(out), p(v)]);
        assert_eq!(r.u32(), out);
        assert_eq!(read_floats(&e, out, 3), vec![14.0, 32.0, 50.0]);
    }

    #[test]
    fn the_column_setter_and_the_zeroing_function() {
        let mut e = engine();
        let m = floats(&mut e, &[9.0; 9]);
        e.call(0x004b_4660, &args![p(m), 1u32, 1.0f32, 2.0f32, 3.0f32]);
        assert_eq!(
            read_floats(&e, m, 9),
            vec![9.0, 1.0, 9.0, 9.0, 2.0, 9.0, 9.0, 3.0, 9.0]
        );
        e.call(0x004b_4600, &args![p(m)]);
        assert_eq!(read_floats(&e, m, 9), vec![0.0; 9]);
    }

    #[test]
    fn the_inverse_divides_the_adjugate_by_the_determinant() {
        let mut e = engine();
        math_doubles(&mut e);
        let m = floats(&mut e, &[1.0, 2.0, 3.0, 0.0, 1.0, 4.0, 5.0, 6.0, 0.0]);
        let out = e.mem.alloc(36);
        assert!(e.call(0x004b_46a0, &args![p(m), p(out)]).bool());
        assert_eq!(
            read_floats(&e, out, 9),
            vec![-24.0, 18.0, 5.0, 20.0, -15.0, -4.0, -5.0, 4.0, 1.0]
        );
        let diagonal = floats(&mut e, &[2.0, 0.0, 0.0, 0.0, 4.0, 0.0, 0.0, 0.0, 5.0]);
        assert!(e.call(0x004b_46a0, &args![p(diagonal), p(out)]).bool());
        assert_close(
            &read_floats(&e, out, 9),
            &[0.5, 0.0, 0.0, 0.0, 0.25, 0.0, 0.0, 0.0, 0.2],
        );
    }

    #[test]
    fn a_singular_matrix_leaves_the_adjugate_and_the_wrapper_gives_zeros() {
        let mut e = engine();
        point_doubles(&mut e);
        math_doubles(&mut e);
        let singular = floats(&mut e, &[1.0, 2.0, 3.0, 2.0, 4.0, 6.0, 7.0, 8.0, 9.0]);
        let out = e.mem.alloc(36);
        assert!(!e.call(0x004b_46a0, &args![p(singular), p(out)]).bool());
        // Unscaled: the first adjugate entry is m4 * m8 - m5 * m7 = 36 - 48.
        assert_eq!(e.mem.f32(out), -12.0);
        // The wrapper writes the zero matrix instead.
        let result = e.mem.alloc(36);
        let r = e.call(0x004b_45b0, &args![p(singular), p(result)]);
        assert_eq!(r.u32(), result);
        assert_eq!(read_floats(&e, result, 9), vec![0.0; 9]);
        // And the inverse of an invertible matrix.
        let diagonal = floats(&mut e, &[2.0, 0.0, 0.0, 0.0, 4.0, 0.0, 0.0, 0.0, 5.0]);
        e.call(0x004b_45b0, &args![p(diagonal), p(result)]);
        assert_close(
            &read_floats(&e, result, 9),
            &[0.5, 0.0, 0.0, 0.0, 0.25, 0.0, 0.0, 0.0, 0.2],
        );
    }

    // ---- world space points ----

    /// A geometry object: `+0` point count, `+4` first array, `+8` second
    /// array, `+0x68` the world transform (its rotation first).
    fn geometry(e: &mut Engine, points: &[[f32; 3]], vectors: &[[f32; 3]]) -> Ptr {
        let object = e.mem.alloc(0x68 + 0x34);
        e.mem.set_u32(object, points.len() as u32);
        let flat = |rows: &[[f32; 3]]| rows.iter().flatten().copied().collect::<Vec<f32>>();
        let point_block = floats(e, &flat(points));
        let v = floats(e, &flat(vectors));
        e.mem.set_u32(object + 4, point_block);
        e.mem.set_u32(object + 8, v);
        for (i, w) in [1.0f32, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]
            .iter()
            .enumerate()
        {
            e.mem.set_f32(object + 0x68 + 4 * i as u32, *w);
        }
        p(object)
    }

    fn geometry_doubles(e: &mut Engine) {
        e.register(POINTS_FIRST, |e, a| ret_u(e.mem.u32(a[0] + 4)));
        e.register(POINTS_SECOND, |e, a| ret_u(e.mem.u32(a[0] + 8)));
        e.register(POINT_COUNT, |e, a| ret_u(e.mem.u32(a[0])));
        e.register(WORLD_TRANSFORM, |_, a| ret_u(a[0] + 0x68));
    }

    #[test]
    fn world_points_transform_the_first_array_into_a_new_block() {
        let mut e = engine();
        point_doubles(&mut e);
        geometry_doubles(&mut e);
        e.register(OPERATOR_NEW, |e, a| ret_u(e.mem.alloc(a[0])));
        e.register(VECTOR_CONSTRUCT, |_, _| Ret::default());
        e.register(TRANSFORM_POINTS, |_, _| Ret::default());
        let object = geometry(&mut e, &[[1.0; 3], [2.0; 3]], &[[0.0; 3], [0.0; 3]]);
        e.call_log = Some(vec![]);
        let block = e.call(0x004b_4910, &args![object]).u32();
        let log = take_log(&mut e);
        assert_ne!(block, 0);
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![args![24u32]]);
        assert_eq!(
            calls_to(&log, VECTOR_CONSTRUCT),
            vec![args![block, 12u32, 2u32, 0x0068_15c0u32]]
        );
        let source = e.mem.u32(object.addr() + 4);
        assert_eq!(
            calls_to(&log, TRANSFORM_POINTS),
            vec![args![2u32, source, block, object.addr() + 0x68]]
        );
        // Order: count twice, new, vector constructor, transform, source,
        // count, transform points.
        let order: Vec<u32> = log.iter().map(|(a, _)| *a).collect();
        assert_eq!(
            order,
            vec![
                0x004b_4910,
                POINT_COUNT,
                POINT_COUNT,
                OPERATOR_NEW,
                VECTOR_CONSTRUCT,
                WORLD_TRANSFORM,
                POINTS_FIRST,
                POINT_COUNT,
                TRANSFORM_POINTS
            ]
        );
    }

    #[test]
    fn world_points_of_nothing_are_null() {
        let mut e = engine();
        geometry_doubles(&mut e);
        assert_eq!(e.call(0x004b_4910, &args![Ptr::<()>::NULL]).u32(), 0);
        let empty = geometry(&mut e, &[], &[]);
        assert_eq!(e.call(0x004b_4910, &args![empty]).u32(), 0);
    }

    // ---- the snapping function ----

    struct SnapWorld {
        e: Engine,
        first: Ptr,
        second: Ptr,
        changed: Rc<RefCell<Vec<(u32, u32)>>>,
        freed: Rc<RefCell<Vec<u32>>>,
    }

    type Arrays<'a> = (&'a [[f32; 3]], &'a [[f32; 3]]);

    /// The doubles for everything the snapping function calls outside the
    /// file, with a real in-memory map; world points are the object points
    /// (identity transform).
    fn snap_world(first: Arrays, second: Arrays) -> SnapWorld {
        let mut e = engine();
        point_doubles(&mut e);
        math_doubles(&mut e);
        geometry_doubles(&mut e);
        e.register(OPERATOR_NEW, |e, a| ret_u(e.mem.alloc(a[0])));
        e.register(VECTOR_CONSTRUCT, |_, _| Ret::default());
        e.register(TRANSFORM_POINTS, |e, a| {
            // Identity transform: a plain copy of `count` points.
            copy_words(e, a[2], a[1], 3 * a[0]);
            Ret::default()
        });
        let freed = Rc::new(RefCell::new(Vec::new()));
        {
            let freed = freed.clone();
            e.register_double(OPERATOR_DELETE, move |e, a| {
                freed.borrow_mut().push(a[0]);
                e.mem.free(a[0]);
                Ret::default()
            });
        }
        e.register(TRANSFORM_CONSTRUCT, |_, _| Ret::default());
        e.register(TRANSFORM_INVERT_NON_UNIFORM, |_, _| Ret::default());
        e.register(MATRIX_PRODUCT, |e, a| {
            // The product of the identity inverse and the identity rotation.
            for i in 0..9 {
                e.mem
                    .set_f32(a[1] + 4 * i, if i % 4 == 0 { 1.0 } else { 0.0 });
            }
            ret_u(a[1])
        });
        e.register(FLOATS_NEAR, |_, a| {
            ret_u(((arg_f(a, 0) - arg_f(a, 1)).abs() <= arg_f(a, 2)) as u32)
        });
        let map: Rc<RefCell<BTreeMap<u32, [u32; 3]>>> = Rc::default();
        e.register(POINT_MAP_CONSTRUCT, |_, a| ret_u(a[0]));
        e.register(POINT_MAP_DESTRUCT, |_, _| Ret::default());
        {
            let map = map.clone();
            e.register_double(POINT_MAP_FIND, move |e, a| match map.borrow().get(&a[1]) {
                Some(value) => {
                    for (i, w) in value.iter().enumerate() {
                        e.mem.set_u32(a[2] + 4 * i as u32, *w);
                    }
                    ret_u(1)
                }
                None => ret_u(0),
            });
        }
        {
            let map = map.clone();
            e.register_double(POINT_MAP_SET_AT, move |_, a| {
                map.borrow_mut().insert(a[1], [a[2], a[3], a[4]]);
                Ret::default()
            });
        }
        {
            let map = map.clone();
            e.register_double(POINT_MAP_FIRST, move |_, _| {
                ret_u(!map.borrow().is_empty() as u32)
            });
        }
        {
            let map = map.clone();
            e.register_double(POINT_MAP_NEXT, move |e, a| {
                // The iterator cell holds 1 + the index of the entry to give.
                let index = e.mem.u32(a[1]) - 1;
                let map = map.borrow();
                let (key, value) = map.iter().nth(index as usize).unwrap();
                e.mem.set_u32(a[2], *key);
                for (i, w) in value.iter().enumerate() {
                    e.mem.set_u32(a[3] + 4 * i as u32, *w);
                }
                let next = if (index as usize) + 1 < map.len() {
                    index + 2
                } else {
                    0
                };
                e.mem.set_u32(a[1], next);
                Ret::default()
            });
        }
        e.register(GEOMETRY_DATA, |_, a| ret_u(a[0] + 0x1000));
        let changed = Rc::new(RefCell::new(Vec::new()));
        {
            let changed = changed.clone();
            e.register_double(MARK_AS_CHANGED, move |_, a| {
                changed.borrow_mut().push((a[0], a[1]));
                Ret::default()
            });
        }
        let first = geometry(&mut e, first.0, first.1);
        let second = geometry(&mut e, second.0, second.1);
        SnapWorld {
            e,
            first,
            second,
            changed,
            freed,
        }
    }

    fn snap(w: &mut SnapWorld, threshold: f32, debug: bool) -> bool {
        w.e.call(
            0x004b_3e60,
            &args![w.first, w.second, threshold, 0u32, debug],
        )
        .bool()
    }

    fn vectors_of(w: &SnapWorld, object: Ptr, n: u32) -> Vec<f32> {
        read_floats(&w.e, w.e.mem.u32(object.addr() + 8), 3 * n)
    }

    #[test]
    fn snapping_copies_the_nearest_first_vector_onto_each_second_point() {
        let mut w = snap_world(
            (
                &[[0.0; 3], [10.0, 0.0, 0.0]],
                &[[0.0, 0.0, 2.0], [0.0, 3.0, 0.0]],
            ),
            (
                &[[0.5, 0.0, 0.0], [9.5, 0.0, 0.0], [50.0, 50.0, 50.0]],
                &[[5.0, 5.0, 5.0], [6.0, 6.0, 6.0], [7.0, 7.0, 7.0]],
            ),
        );
        w.e.call_log = Some(vec![]);
        assert!(snap(&mut w, 2.0, false));
        let log = take_log(&mut w.e);
        // The vectors are the first object's, unitized; the third second point
        // (too far) keeps its own.
        assert_close(
            &vectors_of(&w, w.second, 3),
            &[0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 7.0, 7.0, 7.0],
        );
        // Only the second object's data is marked, with flag word 3.
        assert_eq!(*w.changed.borrow(), vec![(w.second.addr() + 0x1000, 3)]);
        // The world point blocks were released and the object remembered.
        assert_eq!(w.freed.borrow().len(), 2);
        assert_eq!(w.e.global::<u32>(LAST_SNAPPED_OBJECT), w.first.addr());
        // The map was built with 0x25 buckets and destroyed.
        assert_eq!(calls_to(&log, POINT_MAP_CONSTRUCT)[0][1], 0x25);
        assert_eq!(calls_to(&log, POINT_MAP_DESTRUCT).len(), 1);
        // Two entries: first 0 -> second 0 (distance 0.5), first 1 -> second 1.
        let sets = calls_to(&log, POINT_MAP_SET_AT);
        assert_eq!(sets.len(), 2);
        assert_eq!(&sets[0][1..4], &[0, 0, 3]);
        assert_eq!(f32::from_bits(sets[0][4]), 0.5);
        assert_eq!(&sets[1][1..4], &[1, 1, 3]);
    }

    #[test]
    fn a_negative_threshold_matches_at_any_distance() {
        let mut w = snap_world(
            (&[[0.0; 3]], &[[0.0, 0.0, 4.0]]),
            (&[[1000.0, 0.0, 0.0]], &[[9.0, 9.0, 9.0]]),
        );
        assert!(!snap(&mut w, 2.0, false));
        assert!(w.changed.borrow().is_empty());
        // Nothing recorded: the geometry data is untouched, but the last
        // object is still remembered.
        assert_eq!(w.e.global::<u32>(LAST_SNAPPED_OBJECT), w.first.addr());
        assert!(snap(&mut w, -1.0, false));
        assert_close(&vectors_of(&w, w.second, 1), &[0.0, 0.0, 1.0]);
    }

    #[test]
    fn two_second_points_within_the_tolerance_share_a_first_point() {
        let mut w = snap_world(
            (&[[0.0; 3]], &[[0.0, 5.0, 0.0]]),
            (
                &[[0.5, 0.0, 0.0], [0.5002, 0.0, 0.0], [0.7, 0.0, 0.0]],
                &[[1.0, 1.0, 1.0], [2.0, 2.0, 2.0], [3.0, 3.0, 3.0]],
            ),
        );
        w.e.call_log = Some(vec![]);
        assert!(snap(&mut w, 5.0, false));
        let log = take_log(&mut w.e);
        // Second point 0 (0.5) first; 1 (0.5002) is within 0.001 of it so it
        // replaces it keeping index 0 as the partner; 2 (0.7) is farther.
        let sets = calls_to(&log, POINT_MAP_SET_AT);
        assert_eq!(sets.len(), 2);
        assert_eq!(&sets[0][1..4], &[0, 0, 3]);
        assert_eq!(&sets[1][1..4], &[0, 1, 0]);
        // The entry names second point 1, and its partner 0 gets a copy.
        assert_close(
            &vectors_of(&w, w.second, 3),
            &[0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 3.0, 3.0, 3.0],
        );
    }

    #[test]
    fn a_nearer_second_point_replaces_the_entry_and_a_farther_one_does_not() {
        let mut w = snap_world(
            (&[[0.0; 3]], &[[0.0, 0.0, 6.0]]),
            (
                &[[2.0, 0.0, 0.0], [1.0, 0.0, 0.0], [3.0, 0.0, 0.0]],
                &[[1.0, 1.0, 1.0], [2.0, 2.0, 2.0], [3.0, 3.0, 3.0]],
            ),
        );
        w.e.call_log = Some(vec![]);
        assert!(snap(&mut w, 5.0, false));
        let log = take_log(&mut w.e);
        let sets = calls_to(&log, POINT_MAP_SET_AT);
        // 2.0 first; 1.0 is nearer (replaces, partner = the second count);
        // 3.0 is farther (no call).
        assert_eq!(sets.len(), 2);
        assert_eq!(&sets[1][1..4], &[0, 1, 3]);
        assert_close(
            &vectors_of(&w, w.second, 3),
            &[1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 3.0, 3.0, 3.0],
        );
    }

    #[test]
    fn snapping_in_debug_mode_moves_the_points_and_marks_both() {
        let mut w = snap_world(
            (&[[0.0; 3]], &[[0.0, 0.0, 1.0]]),
            (&[[0.5, 0.0, 0.0]], &[[9.0, 9.0, 9.0]]),
        );
        assert!(snap(&mut w, 2.0, true));
        // Each point moved by 10 times its vector: the second's by its new
        // vector (0, 0, 1), the first's too (the previous object differs).
        let second_points = read_floats(&w.e, w.e.mem.u32(w.second.addr() + 4), 3);
        assert_close(&second_points, &[0.5, 0.0, 10.0]);
        let first_points = read_floats(&w.e, w.e.mem.u32(w.first.addr() + 4), 3);
        assert_close(&first_points, &[0.0, 0.0, 10.0]);
        assert_eq!(
            *w.changed.borrow(),
            vec![(w.first.addr() + 0x1000, 3), (w.second.addr() + 0x1000, 3)]
        );
        // Run again on the same first object: its points stay put.
        assert!(snap(&mut w, 2.0, true));
        let first_points = read_floats(&w.e, w.e.mem.u32(w.first.addr() + 4), 3);
        assert_close(&first_points, &[0.0, 0.0, 10.0]);
    }

    #[test]
    fn snapping_refuses_missing_inputs_and_still_destroys_the_map() {
        let mut w = snap_world(
            (&[[0.0; 3]], &[[0.0, 0.0, 1.0]]),
            (&[[0.5, 0.0, 0.0]], &[[9.0, 9.0, 9.0]]),
        );
        w.e.call_log = Some(vec![]);
        for (first, second) in [
            (Ptr::<()>::NULL, w.second),
            (w.first, Ptr::<()>::NULL),
            (w.first, w.first),
        ] {
            let r =
                w.e.call(0x004b_3e60, &args![first, second, 2.0f32, 0u32, false])
                    .bool();
            assert!(!r);
        }
        let log = take_log(&mut w.e);
        assert_eq!(calls_to(&log, POINT_MAP_CONSTRUCT).len(), 3);
        assert_eq!(calls_to(&log, POINT_MAP_DESTRUCT).len(), 3);
        // No point arrays were asked for.
        assert!(calls_to(&log, POINTS_FIRST).is_empty());
        // An object without points.
        let empty = geometry(&mut w.e, &[], &[]);
        let r =
            w.e.call(0x004b_3e60, &args![empty, w.second, 2.0f32, 0u32, false])
                .bool();
        assert!(!r);
        let r =
            w.e.call(0x004b_3e60, &args![w.first, empty, 2.0f32, 0u32, false])
                .bool();
        assert!(!r);
        assert!(w.freed.borrow().is_empty());
    }

    // ---- bhk objects ----

    fn bhk_engine() -> Engine {
        let mut e = engine();
        e.register(BHK_REF_OBJECT_CONSTRUCT, |_, _| Ret::default());
        e.register(BHK_MOUSE_SPRING_ACTION_INIT, |_, _| Ret::default());
        e.register(BHK_SERIALIZABLE_DESTRUCT, |_, _| Ret::default());
        e.register(BHK_ACTION_DESTRUCT, |_, _| Ret::default());
        e.register(BHK_UNARY_ACTION_DESTRUCT, |_, _| Ret::default());
        e.register(SIZED_DELETE, |_, _| Ret::default());
        e
    }

    #[test]
    fn the_serializable_constructor_installs_its_vtable_and_clears_pinfo() {
        let mut e = bhk_engine();
        let this: Ptr<BhkSerializable> = e.new_object();
        e.set(this, BhkSerializable::pInfo, 0x1234);
        e.call_log = Some(vec![]);
        let r = e.call(0x004b_5120, &args![this]);
        assert_eq!(r.u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), BHK_SERIALIZABLE_VTABLE);
        assert_eq!(e.get(this, BhkSerializable::pInfo), 0);
        assert_eq!(
            calls_to(&take_log(&mut e), BHK_REF_OBJECT_CONSTRUCT),
            vec![args![this]]
        );
    }

    #[test]
    fn each_constructor_in_the_chain_counts_its_object() {
        let mut e = bhk_engine();
        let this: Ptr<BhkSerializable> = e.new_object();
        e.call(0x004b_50f0, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), BHK_ACTION_VTABLE);
        assert_eq!(e.global::<u32>(BHK_ACTION_OBJECT_COUNT), 1);
        e.call(0x004b_50c0, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), BHK_UNARY_ACTION_VTABLE);
        assert_eq!(e.global::<u32>(BHK_UNARY_ACTION_OBJECT_COUNT), 1);
        assert_eq!(e.global::<u32>(BHK_ACTION_OBJECT_COUNT), 2);
        e.call_log = Some(vec![]);
        let r = e.call(0x004b_5040, &args![this, 0x77u32]);
        assert_eq!(r.u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), BHK_MOUSE_SPRING_ACTION_VTABLE);
        assert_eq!(e.global::<u32>(BHK_MOUSE_SPRING_ACTION_OBJECT_COUNT), 1);
        assert_eq!(e.global::<u32>(BHK_UNARY_ACTION_OBJECT_COUNT), 2);
        assert_eq!(e.global::<u32>(BHK_ACTION_OBJECT_COUNT), 3);
        let log = take_log(&mut e);
        assert_eq!(
            calls_to(&log, BHK_MOUSE_SPRING_ACTION_INIT),
            vec![args![this, 0x77u32]]
        );
        // The base constructor ran before the Init call.
        let order: Vec<u32> = log.iter().map(|(a, _)| *a).collect();
        assert_eq!(
            order,
            vec![
                0x004b_5040,
                BHK_REF_OBJECT_CONSTRUCT,
                BHK_MOUSE_SPRING_ACTION_INIT
            ]
        );
    }

    #[test]
    fn get_rtti_returns_the_class_objects() {
        let mut e = bhk_engine();
        assert_eq!(
            e.call(0x004b_5150, &args![Ptr::<()>::NULL]).u32(),
            0x0126_8134
        );
        assert_eq!(
            e.call(0x004b_5190, &args![Ptr::<()>::NULL]).u32(),
            0x0126_8158
        );
        assert_eq!(
            e.call(0x004b_51d0, &args![Ptr::<()>::NULL]).u32(),
            0x0126_811c
        );
    }

    #[test]
    fn the_scalar_deleting_destructors_free_only_when_asked() {
        let mut e = bhk_engine();
        let this = p(0x4000_0000);
        for (address, destructor) in [
            (0x004b_5160, BHK_SERIALIZABLE_DESTRUCT),
            (0x004b_51a0, BHK_ACTION_DESTRUCT),
            (0x004b_51e0, BHK_UNARY_ACTION_DESTRUCT),
        ] {
            e.call_log = Some(vec![]);
            let r = e.call(address, &args![this, 0u32]);
            assert_eq!(r.u32(), this.addr());
            let log = take_log(&mut e);
            assert_eq!(calls_to(&log, destructor), vec![args![this]]);
            assert!(calls_to(&log, SIZED_DELETE).is_empty());
            e.call_log = Some(vec![]);
            e.call(address, &args![this, 1u32]);
            let log = take_log(&mut e);
            assert_eq!(calls_to(&log, SIZED_DELETE), vec![args![this, 0x10u32]]);
        }
    }

    // ---- second session: collision objects, scene graph walks ----

    type Shared<T> = Rc<RefCell<T>>;
    /// A virtual call a test node recorded: the slot offset and the words.
    type VirtualCall = (u32, Vec<u32>);
    /// A walker call: the root, the callback and the context's seven words.
    type WalkCall = (u32, u32, Vec<u32>);

    /// A test scene graph. Every node is a block of 0x60 bytes with the
    /// vtable `GRAPH_TABLE`; its fields are whatever the doubles of the test
    /// read: `+0x10` collision object, `+0x14` value, `+0x18` parent,
    /// `+0x20` name text, `+0x24` morpher or marker controller, `+0x28` and
    /// `+0x2c` class flags, `+0x2c` extra data (in the marker test),
    /// `+0x30` property, `+0x34` and `+0x38` the results of the virtual slots
    /// `0x18` and `0x1c`. Virtual slot `0xc` gives the node itself when it
    /// has children (the node is its own container) and 0 otherwise; the
    /// slots `0x90`, `0xe8` and `0xf0` record their calls.
    struct Graph {
        children: Rc<RefCell<BTreeMap<u32, Vec<u32>>>>,
        virtuals: Shared<Vec<VirtualCall>>,
    }

    const GRAPH_TABLE: u32 = 0x0300_4000;

    fn graph(e: &mut Engine) -> Graph {
        let children: Rc<RefCell<BTreeMap<u32, Vec<u32>>>> = Rc::default();
        let virtuals: Shared<Vec<VirtualCall>> = Rc::default();
        {
            let children = children.clone();
            e.register_double(CHILD_COUNT, move |_, a| {
                ret_u(children.borrow().get(&a[0]).map_or(0, |c| c.len() as u32))
            });
        }
        {
            let children = children.clone();
            e.register_double(CHILD_AT, move |_, a| {
                ret_u(children.borrow()[&a[0]][a[1] as usize])
            });
        }
        let mut slots = vec![0u32; 61];
        slots[3] = 0x0300_5000;
        for (slot, double) in [
            (36u32, 0x0300_5010u32),
            (58, 0x0300_5020),
            (60, 0x0300_5030),
        ] {
            slots[slot as usize] = double;
            let virtuals = virtuals.clone();
            e.register_double(double, move |_, a| {
                virtuals.borrow_mut().push((slot * 4, a.to_vec()));
                Ret::default()
            });
        }
        {
            let children = children.clone();
            e.register_double(0x0300_5000, move |_, a| {
                ret_u(if children.borrow().contains_key(&a[0]) {
                    a[0]
                } else {
                    0
                })
            });
        }
        // Slots 0x18 and 0x1c (third session): the words at `+0x34` and `+0x38`.
        slots[6] = 0x0300_5040;
        e.register(0x0300_5040, |e, a| ret_u(e.mem.u32(a[0] + 0x34)));
        slots[7] = 0x0300_5050;
        e.register(0x0300_5050, |e, a| ret_u(e.mem.u32(a[0] + 0x38)));
        e.put_vtable(GRAPH_TABLE, &slots);
        Graph { children, virtuals }
    }

    fn graph_node(e: &mut Engine, g: &Graph, kids: &[u32]) -> u32 {
        let node = e.mem.alloc(0x60);
        e.mem.set_u32(node, GRAPH_TABLE);
        if !kids.is_empty() {
            g.children.borrow_mut().insert(node, kids.to_vec());
        }
        node
    }

    fn constant_text(e: &Engine, address: u32) -> String {
        match address {
            SCABBARD_NAME => "Scb".into(),
            FADE_NODE_PREFIX => "FadeNode ".into(),
            EDITOR_MARKER_NAME => "EditorMarker".into(),
            ARROW_NAME => "Arrow".into(),
            _ => (0..)
                .map(|i| e.mem.u8(address + i))
                .take_while(|&b| b != 0)
                .map(char::from)
                .collect(),
        }
    }

    /// Names: the holder is the object, its text pointer sits at `+0x20`.
    fn name_doubles(e: &mut Engine) {
        e.register(NAME_HOLDER, |_, a| ret_u(a[0]));
        e.register(NAME_TEXT, |e, a| ret_u(e.mem.u32(a[0] + 0x20)));
        fn differ(e: &mut Engine, a: &[u32]) -> Ret {
            ret_u((constant_text(e, a[0]) != constant_text(e, a[1])) as u32)
        }
        e.register(STRING_COMPARE, differ);
        e.register(STRING_COMPARE_SECOND, differ);
        e.register(STRING_COMPARE_NO_CASE, |e, a| {
            let n = a[2] as usize;
            let x: String = constant_text(e, a[0])
                .to_lowercase()
                .chars()
                .take(n)
                .collect();
            let y: String = constant_text(e, a[1])
                .to_lowercase()
                .chars()
                .take(n)
                .collect();
            ret_u((x != y) as u32)
        });
    }

    fn set_name(e: &mut Engine, node: u32, name: &str) {
        let t = text(e, name);
        e.mem.set_u32(node + 0x20, t.addr());
    }

    fn collision_engine() -> (Engine, Graph) {
        let mut e = engine();
        let g = graph(&mut e);
        e.register(GET_BHK_COLLISION_OBJECT, |e, a| {
            ret_u(e.mem.u32(a[0] + 0x10))
        });
        (e, g)
    }

    #[test]
    fn mouse_spring_action_get_rtti_returns_its_class_object() {
        let mut e = bhk_engine();
        assert_eq!(
            e.call(0x004b_5210, &args![Ptr::<()>::NULL]).u32(),
            0x0126_812c
        );
    }

    #[test]
    fn the_constant_virtual_function_returns_0x40() {
        let mut e = bhk_engine();
        assert_eq!(e.call(0x004b_5220, &args![Ptr::<()>::NULL]).u32(), 0x40);
    }

    #[test]
    fn the_mouse_spring_action_destructor_frees_only_when_asked() {
        let mut e = bhk_engine();
        e.register(BHK_MOUSE_SPRING_ACTION_DESTRUCT, |_, _| Ret::default());
        let this = p(0x4000_0000);
        e.call_log = Some(vec![]);
        let r = e.call(0x004b_5230, &args![this, 0u32]);
        assert_eq!(r.u32(), this.addr());
        let log = take_log(&mut e);
        assert_eq!(
            calls_to(&log, BHK_MOUSE_SPRING_ACTION_DESTRUCT),
            vec![args![this]]
        );
        assert!(calls_to(&log, SIZED_DELETE).is_empty());
        e.call_log = Some(vec![]);
        e.call(0x004b_5230, &args![this, 3u32]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, SIZED_DELETE), vec![args![this, 0x10u32]]);
    }

    #[test]
    fn find_first_collision_object_takes_the_own_one_or_the_first_below() {
        let (mut e, g) = collision_engine();
        let a = graph_node(&mut e, &g, &[]);
        let b = graph_node(&mut e, &g, &[]);
        let c = graph_node(&mut e, &g, &[]);
        e.mem.set_u32(b + 0x10, 0xb0);
        e.mem.set_u32(c + 0x10, 0xc0);
        let inner = graph_node(&mut e, &g, &[a]);
        let root = graph_node(&mut e, &g, &[inner, b, c]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004b_5260, &args![p(root)]).u32(), 0xb0);
        // The root, the inner node, `a`, then `b`: `c` is never asked.
        let log = take_log(&mut e);
        assert_eq!(
            calls_to(&log, GET_BHK_COLLISION_OBJECT),
            vec![args![root], args![inner], args![a], args![b]]
        );
        // An own collision object wins without looking at the children.
        e.mem.set_u32(root + 0x10, 0x99);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004b_5260, &args![p(root)]).u32(), 0x99);
        assert_eq!(calls_to(&take_log(&mut e), CHILD_COUNT).len(), 0);
        // Nothing found, a leaf, and a null object.
        assert_eq!(e.call(0x004b_5260, &args![p(a)]).u32(), 0);
        assert_eq!(e.call(0x004b_5260, &args![inner]).u32(), 0);
        assert_eq!(e.call(0x004b_5260, &args![Ptr::<()>::NULL]).u32(), 0);
    }

    fn measured_engine() -> (Engine, Graph) {
        let (mut e, g) = collision_engine();
        e.register(COLLISION_OBJECT_OWNER, |_, a| ret_u(a[0]));
        e.register(BODY_OF_OBJECT, |_, a| ret_u(a[0]));
        e.register(BODY_VALUE, |e, a| ret_f(e.mem.f32(a[0] + 0x14) as f64));
        (e, g)
    }

    /// A collision object whose measured value is `value`.
    fn measured(e: &mut Engine, value: f32) -> u32 {
        let object = e.mem.alloc(0x20);
        e.mem.set_f32(object + 0x14, value);
        object
    }

    #[test]
    fn fn_004b52f0_picks_the_greatest_value_and_the_first_on_ties() {
        let (mut e, g) = measured_engine();
        let (v1, v3, v2, v2b) = (
            measured(&mut e, 1.0),
            measured(&mut e, 3.0),
            measured(&mut e, 2.0),
            measured(&mut e, 2.0),
        );
        let leaf = |e: &mut Engine, g: &Graph, collision: u32| {
            let node = graph_node(e, g, &[]);
            e.mem.set_u32(node + 0x10, collision);
            node
        };
        let (n1, n3, n2, n2b) = (
            leaf(&mut e, &g, v1),
            leaf(&mut e, &g, v3),
            leaf(&mut e, &g, v2),
            leaf(&mut e, &g, v2b),
        );
        let root = graph_node(&mut e, &g, &[n1, n3, n2]);
        assert_eq!(e.call(0x004b_52f0, &args![p(root)]).u32(), v3);
        // Equal values: the first stays.
        let ties = graph_node(&mut e, &g, &[n2, n2b]);
        assert_eq!(e.call(0x004b_52f0, &args![p(ties)]).u32(), v2);
        // The node's own collision object against the best below: it wins
        // only when the best below is not greater.
        let own_high = measured(&mut e, 5.0);
        let own_equal = measured(&mut e, 3.0);
        let own_low = measured(&mut e, 2.0);
        let parent = graph_node(&mut e, &g, &[n3]);
        e.mem.set_u32(parent + 0x10, own_high);
        assert_eq!(e.call(0x004b_52f0, &args![p(parent)]).u32(), own_high);
        e.mem.set_u32(parent + 0x10, own_equal);
        assert_eq!(e.call(0x004b_52f0, &args![p(parent)]).u32(), own_equal);
        e.mem.set_u32(parent + 0x10, own_low);
        assert_eq!(e.call(0x004b_52f0, &args![p(parent)]).u32(), v3);
        // No own object: the one below. Nothing at all, and null: 0.
        e.mem.set_u32(parent + 0x10, 0);
        assert_eq!(e.call(0x004b_52f0, &args![p(parent)]).u32(), v3);
        let bare = graph_node(&mut e, &g, &[]);
        assert_eq!(e.call(0x004b_52f0, &args![p(bare)]).u32(), 0);
        assert_eq!(e.call(0x004b_52f0, &args![Ptr::<()>::NULL]).u32(), 0);
        // The own object alone.
        e.mem.set_u32(bare + 0x10, own_low);
        assert_eq!(e.call(0x004b_52f0, &args![p(bare)]).u32(), own_low);
    }

    #[test]
    fn fn_004b5400_reports_the_value_of_the_body_or_zero() {
        let mut e = engine();
        e.register(BODY_OF_OBJECT, |e, a| ret_u(e.mem.u32(a[0] + 0x10)));
        e.register(BODY_VALUE, |e, a| ret_f(e.mem.f32(a[0] + 0x14) as f64));
        let object = e.mem.alloc(0x20);
        assert_eq!(e.call(0x004b_5400, &args![p(object)]).f32(), 0.0);
        let body = measured(&mut e, 12.5);
        e.mem.set_u32(object + 0x10, body);
        assert_eq!(e.call(0x004b_5400, &args![p(object)]).f32(), 12.5);
    }

    fn length_squared_double(e: &mut Engine) {
        point_doubles(e);
        e.register(VECTOR_LENGTH_SQUARED, |e, a| {
            let sum: f32 = (0..3).map(|i| e.mem.f32(a[0] + 4 * i).powi(2)).sum();
            ret_f(sum as f64)
        });
    }

    #[test]
    fn fn_004b5470_compares_the_squared_length_with_the_squared_radius() {
        let mut e = engine();
        length_squared_double(&mut e);
        let vector = p(floats(&mut e, &[3.0, 4.0, 0.0]));
        // Squared length 25.
        assert_eq!(e.call(0x004b_5470, &args![vector, 6.0f32]).i32(), -1);
        assert_eq!(e.call(0x004b_5470, &args![vector, 4.0f32]).i32(), 1);
        assert_eq!(e.call(0x004b_5470, &args![vector, 5.0f32]).i32(), 0);
        assert_eq!(e.call(0x004b_5470, &args![vector, -5.0f32]).i32(), 0);
        assert_eq!(e.call(0x004b_5470, &args![vector, f32::NAN]).i32(), 0);
    }

    #[test]
    fn fn_004b5440_builds_the_difference_and_compares_it() {
        let mut e = engine();
        length_squared_double(&mut e);
        let a = p(floats(&mut e, &[4.0, 6.0, 1.0]));
        let b = p(floats(&mut e, &[1.0, 2.0, 1.0]));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004b_5440, &args![a, b, 6.0f32]).i32(), -1);
        let log = take_log(&mut e);
        let subtract = calls_to(&log, POINT3_SUBTRACT);
        assert_eq!(subtract.len(), 1);
        assert_eq!((subtract[0][0], subtract[0][2]), (a.addr(), b.addr()));
        assert_eq!(e.call(0x004b_5440, &args![a, b, 4.0f32]).i32(), 1);
        assert_eq!(e.call(0x004b_5440, &args![a, b, 5.0f32]).i32(), 0);
    }

    #[test]
    fn ni_a_sin_clamps_at_the_limits_and_calls_the_body_between() {
        let mut e = engine();
        e.set_global(NEGATIVE_ONE_DOUBLE, -1.0f64);
        e.set_global(ASIN_LIMIT, 1.5707964f32);
        e.register_double(NI_ASIN_BODY, |_, a| {
            assert_eq!(f32::from_bits(a[0]), 0.5);
            ret_f(0.25)
        });
        let limit = 1.5707964f32 as f64;
        for (x, want) in [
            (-2.0f32, -limit),
            (-1.0, -limit),
            (f32::NAN, -limit),
            (1.0, limit),
            (3.0, limit),
            (0.5, 0.25),
        ] {
            e.call_log = Some(vec![]);
            let r = e.call(0x004b_54c0, &args![x]).f64();
            assert_eq!(r, want, "x = {x}");
            let body_calls = calls_to(&take_log(&mut e), NI_ASIN_BODY).len();
            assert_eq!(body_calls, (want == 0.25) as usize);
        }
    }

    /// A block of 0x200 bytes where the collidable sits at `+0x100` with the
    /// given handle type and an owner at `-0x80` from it; returns the
    /// collidable and the owner.
    fn collidable(e: &mut Engine, handle_type: u8) -> (u32, u32) {
        let base = e.mem.alloc(0x200);
        let collidable = base + 0x100;
        e.mem.set_u8(collidable + 0x18, handle_type);
        e.mem.set_u8(collidable + 0x10, (-0x80i8) as u8);
        (collidable, base + 0x80)
    }

    fn property_doubles(e: &mut Engine) {
        e.register(WORLD_OBJECT_GET_PROPERTY, |_, a| ret_u(a[1]));
        e.register(SMART_POINTER_GET, |_, a| ret_u(a[0]));
        e.register(DYNAMIC_CAST, |e, a| {
            if a[1] == 0 {
                return ret_u(0);
            }
            match a[0] {
                CAST_CLASS_012043F8 => ret_u(e.mem.u32(a[1] + 0x30)),
                CAST_CLASS_01268168 => ret_u(e.mem.u32(a[1] + 0x34)),
                other => panic!("cast to {other:08x}"),
            }
        });
        e.register(WORD_AT_EIGHT, |e, a| ret_u(e.mem.u32(a[0] + 8)));
        e.register(WORD_AT_0X18, |e, a| ret_u(e.mem.u32(a[0] + 0x18)));
    }

    #[test]
    fn get_av_object_for_collidable_follows_bodies_and_phantoms() {
        let mut e = engine();
        property_doubles(&mut e);
        e.register(COLLIDABLE_WORD, |e, a| ret_u(e.mem.u32(a[0] + 0x20)));
        e.register(STORE_WORD, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret_u(a[0])
        });
        e.register(HIGH_HALF, |e, a| ret_u(e.mem.u32(a[0]) >> 16));
        e.register(POINT3_EMPTY_CONSTRUCTOR, |_, a| ret_u(a[0]));
        e.register(BODY_POSITION_SOURCE, |_, a| ret_u(a[0] + 1));
        e.register(POSITION_FROM_VALUE, |_, _| Ret::default());
        e.register(FN_0084E3A0, |e, a| ret_u(e.mem.u32(a[0] + 0x38)));
        e.register(GET_AV_OBJECT, |_, a| {
            assert_eq!(a[1], 0);
            ret_u(a[0] + 0x1000)
        });
        e.set_global(CELL_FINDER, 0x1234u32);
        e.register_double(CELL_AT_POSITION, |e, a| {
            assert_eq!(a[0], 0x1234);
            // The cell is found from the position the body gives.
            ret_u(e.mem.u32(a[1]))
        });
        assert_eq!(
            e.call(0x004b_5820, &args![Ptr::<()>::NULL]).u32(),
            0,
            "null"
        );

        // A body whose cast object has the scene graph object at +8.
        let (body_collidable, body) = collidable(&mut e, 1);
        let cast = e.mem.alloc(0x20);
        e.mem.set_u32(cast + 8, 0xab0);
        e.mem.set_u32(body + 0x30, cast);
        assert_eq!(e.call(0x004b_5820, &args![p(body_collidable)]).u32(), 0xab0);

        // Without a cast object: the collidable's word decides whether the
        // cell holding the position is used.
        e.mem.set_u32(body + 0x30, 0);
        let cell = e.mem.alloc(0x40);
        e.mem.set_u32(cell + 0x18, 0xce11);
        // The position block is the stack local `00457620` is given; the
        // double stores the cell there before asking.
        e.register_double(POSITION_FROM_VALUE, move |e, a| {
            e.mem.set_u32(a[0], cell);
            Ret::default()
        });
        e.mem.set_u32(body_collidable + 0x20, 0x0001_0000);
        assert_eq!(
            e.call(0x004b_5820, &args![p(body_collidable)]).u32(),
            0xce11
        );
        e.mem.set_u32(body_collidable + 0x20, 0x0002_0000);
        assert_eq!(e.call(0x004b_5820, &args![p(body_collidable)]).u32(), 0);
        e.mem.set_u32(body_collidable + 0x20, 0x0001_0000);
        e.mem.set_u32(cell + 0x18, 0);
        assert_eq!(e.call(0x004b_5820, &args![p(body_collidable)]).u32(), 0);

        // A phantom: the cast object, else the world object's AV object.
        let (phantom_collidable, phantom) = collidable(&mut e, 2);
        let cast = e.mem.alloc(0x20);
        e.mem.set_u32(cast + 8, 0xcd0);
        e.mem.set_u32(phantom + 0x34, cast);
        assert_eq!(
            e.call(0x004b_5820, &args![p(phantom_collidable)]).u32(),
            0xcd0
        );
        e.mem.set_u32(phantom + 0x34, 0);
        assert_eq!(e.call(0x004b_5820, &args![p(phantom_collidable)]).u32(), 0);
        e.mem.set_u32(phantom + 0x38, 0x7000);
        assert_eq!(
            e.call(0x004b_5820, &args![p(phantom_collidable)]).u32(),
            0x7000 + 0x1000
        );

        // Neither a body nor a phantom.
        let (other, _) = collidable(&mut e, 0);
        assert_eq!(e.call(0x004b_5820, &args![p(other)]).u32(), 0);
    }

    #[test]
    fn the_collidable_accessors_read_the_handle_type_and_the_owner() {
        let mut e = engine();
        let (body, owner) = collidable(&mut e, 1);
        let (phantom, phantom_owner) = collidable(&mut e, 2);
        // The handle type at +0x18 (= +0x14 and +4), signed.
        assert_eq!(e.call(0x004b_5980, &args![p(body)]).i32(), 1);
        assert_eq!(e.call(0x004b_5980, &args![p(phantom)]).i32(), 2);
        e.mem.set_u8(body + 0x18, 0xff);
        assert_eq!(e.call(0x004b_5980, &args![p(body)]).i32(), -1);
        e.mem.set_u8(body + 0x18, 1);
        // The handle itself (+0x14): the byte at +4.
        assert_eq!(e.call(0x004b_59a0, &args![p(body + 0x14)]).i32(), 1);
        // The owner: this plus the signed byte at +0x10.
        assert_eq!(e.call(0x004b_59d0, &args![p(body)]).u32(), owner);
        assert_eq!(e.call(0x004b_59c0, &args![p(phantom)]).u32(), phantom_owner);
        e.mem.set_u8(body + 0x10, 0x10);
        assert_eq!(e.call(0x004b_59d0, &args![p(body)]).u32(), body + 0x10);
        // The two typed owners.
        assert_eq!(e.call(0x004b_59f0, &args![p(phantom)]).u32(), 0);
        assert_eq!(e.call(0x004b_5950, &args![p(body)]).u32(), 0);
        assert_eq!(e.call(0x004b_59f0, &args![p(body)]).u32(), body + 0x10);
        assert_eq!(e.call(0x004b_5950, &args![p(phantom)]).u32(), phantom_owner);
    }

    #[test]
    fn fn_004b5950_returns_the_owner_only_for_type_2() {
        let mut e = engine();
        let (phantom, owner) = collidable(&mut e, 2);
        let (body, _) = collidable(&mut e, 1);
        assert_eq!(e.call(0x004b_5950, &args![p(phantom)]).u32(), owner);
        assert_eq!(e.call(0x004b_5950, &args![p(body)]).u32(), 0);
    }

    #[test]
    fn fn_004b5980_reads_the_type_through_the_handle() {
        let mut e = engine();
        let (c, _) = collidable(&mut e, 5);
        assert_eq!(e.call(0x004b_5980, &args![p(c)]).i32(), 5);
    }

    #[test]
    fn fn_004b59a0_reads_a_signed_byte_at_plus_4() {
        let mut e = engine();
        let handle = e.mem.alloc(8);
        e.mem.set_u8(handle + 4, 0x80);
        assert_eq!(e.call(0x004b_59a0, &args![p(handle)]).i32(), -128);
    }

    #[test]
    fn fn_004b59c0_is_the_owner_of_its_argument() {
        let mut e = engine();
        let (c, owner) = collidable(&mut e, 1);
        assert_eq!(e.call(0x004b_59c0, &args![p(c)]).u32(), owner);
    }

    #[test]
    fn fn_004b59d0_adds_the_signed_byte_at_plus_0x10() {
        let mut e = engine();
        let block = e.mem.alloc(0x40) + 0x20;
        e.mem.set_u8(block + 0x10, 0xf0);
        assert_eq!(e.call(0x004b_59d0, &args![p(block)]).u32(), block - 0x10);
        e.mem.set_u8(block + 0x10, 0x08);
        assert_eq!(e.call(0x004b_59d0, &args![p(block)]).u32(), block + 8);
    }

    #[test]
    fn fn_004b59f0_returns_the_owner_only_for_type_1() {
        let mut e = engine();
        let (body, owner) = collidable(&mut e, 1);
        let (phantom, _) = collidable(&mut e, 2);
        assert_eq!(e.call(0x004b_59f0, &args![p(body)]).u32(), owner);
        assert_eq!(e.call(0x004b_59f0, &args![p(phantom)]).u32(), 0);
    }

    #[test]
    fn fn_004b5a50_reads_the_world_object_property_through_the_smart_pointer() {
        let mut e = engine();
        let out_seen = Rc::new(RefCell::new(0u32));
        {
            let out_seen = out_seen.clone();
            e.register_double(WORLD_OBJECT_GET_PROPERTY, move |_, a| {
                assert_eq!((a[1], a[2]), (0x5000, WORLD_OBJECT_PROPERTY_KEY));
                *out_seen.borrow_mut() = a[0];
                ret_u(0x6000)
            });
        }
        e.register(SMART_POINTER_GET, |_, a| ret_u(a[0] + 4));
        assert_eq!(e.call(0x004b_5a50, &args![0x5000u32]).u32(), 0x6004);
        // The out block is a local, not null.
        assert_ne!(*out_seen.borrow(), 0);
    }

    #[test]
    fn fn_004b5a20_casts_the_property_to_the_first_class() {
        let mut e = engine();
        property_doubles(&mut e);
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object + 0x30, 0x77);
        e.mem.set_u32(object + 0x34, 0x88);
        assert_eq!(e.call(0x004b_5a20, &args![p(object)]).u32(), 0x77);
    }

    #[test]
    fn fn_004b5ad0_casts_the_property_to_the_second_class() {
        let mut e = engine();
        property_doubles(&mut e);
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object + 0x30, 0x77);
        e.mem.set_u32(object + 0x34, 0x88);
        assert_eq!(e.call(0x004b_5ad0, &args![p(object)]).u32(), 0x88);
    }

    #[test]
    fn fn_004b5a80_forwards_to_fn_004b5aa0() {
        let mut e = engine();
        e.register(FN_0084E3A0, |_, a| ret_u(a[0] * 2));
        assert_eq!(e.call(0x004b_5a80, &args![0x21u32]).u32(), 0x42);
        assert_eq!(e.call(0x004b_5a80, &args![0u32]).u32(), 0);
    }

    #[test]
    fn fn_004b5aa0_skips_null_objects() {
        let mut e = engine();
        e.register(FN_0084E3A0, |_, a| ret_u(a[0] + 1));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004b_5aa0, &args![0u32]).u32(), 0);
        assert!(calls_to(&take_log(&mut e), FN_0084E3A0).is_empty());
        assert_eq!(e.call(0x004b_5aa0, &args![0x40u32]).u32(), 0x41);
    }

    #[test]
    fn fn_004b5b00_indexes_0x30_byte_elements() {
        let mut e = engine();
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x34, 0x1000);
        assert_eq!(e.call(0x004b_5b00, &args![p(this), 0u32]).u32(), 0x1000);
        assert_eq!(e.call(0x004b_5b00, &args![p(this), 3u32]).u32(), 0x1090);
        // Only the low half-word of the stack word is the index.
        assert_eq!(
            e.call(0x004b_5b00, &args![p(this), 0x0001_0002u32]).u32(),
            0x1000 + 0x60
        );
    }

    #[test]
    fn remove_scabard_removes_the_scb_child_or_descends_into_a_fade_node() {
        let mut e = engine();
        let g = graph(&mut e);
        name_doubles(&mut e);
        let leaf = |e: &mut Engine, g: &Graph, name: &str| {
            let node = graph_node(e, g, &[]);
            if !name.is_empty() {
                set_name(e, node, name);
            }
            node
        };
        // Direct child named "Scb" (after an unnamed one and another).
        let (unnamed, other, scb) = (
            leaf(&mut e, &g, ""),
            leaf(&mut e, &g, "Weapon"),
            leaf(&mut e, &g, "Scb"),
        );
        let parent = graph_node(&mut e, &g, &[unnamed, other, scb]);
        assert!(e.call(0x004b_5b20, &args![p(parent)]).bool());
        assert_eq!(
            *g.virtuals.borrow(),
            vec![(0xf0, vec![parent, 2])],
            "slot 0xf0 of the parent with the child's index"
        );
        // A "FadeNode " child: the search goes into it and gives its answer.
        g.virtuals.borrow_mut().clear();
        let deep_scb = leaf(&mut e, &g, "Scb");
        let fade = graph_node(&mut e, &g, &[other, deep_scb]);
        set_name(&mut e, fade, "FadeNode 01");
        let top = graph_node(&mut e, &g, &[fade]);
        assert!(e.call(0x004b_5b20, &args![p(top)]).bool());
        assert_eq!(*g.virtuals.borrow(), vec![(0xf0, vec![fade, 1])]);
        // A fade node without one: the loop ends there with false (it does
        // not go on to later children).
        g.virtuals.borrow_mut().clear();
        let empty_fade = graph_node(&mut e, &g, &[other]);
        set_name(&mut e, empty_fade, "fadenode x");
        let later_scb = leaf(&mut e, &g, "Scb");
        let top = graph_node(&mut e, &g, &[empty_fade, later_scb]);
        assert!(!e.call(0x004b_5b20, &args![p(top)]).bool());
        assert!(g.virtuals.borrow().is_empty());
        // No children, and null.
        let bare = leaf(&mut e, &g, "");
        assert!(!e.call(0x004b_5b20, &args![p(bare)]).bool());
        assert!(!e.call(0x004b_5b20, &args![Ptr::<()>::NULL]).bool());
    }

    #[test]
    fn has_morpher_controller_looks_at_the_object_then_below() {
        let mut e = engine();
        let g = graph(&mut e);
        e.register(GET_CONTROLLER, |e, a| {
            assert_eq!(a[1], MORPHER_CONTROLLER_CLASS);
            ret_u(e.mem.u32(a[0] + 0x24))
        });
        let (a, b) = (graph_node(&mut e, &g, &[]), graph_node(&mut e, &g, &[]));
        let inner = graph_node(&mut e, &g, &[a, b]);
        let root = graph_node(&mut e, &g, &[inner]);
        assert!(!e.call(0x004b_5bf0, &args![p(root)]).bool());
        e.mem.set_u32(b + 0x24, 1);
        assert!(e.call(0x004b_5bf0, &args![p(root)]).bool());
        e.mem.set_u32(b + 0x24, 0);
        e.mem.set_u32(root + 0x24, 1);
        e.call_log = Some(vec![]);
        assert!(e.call(0x004b_5bf0, &args![p(root)]).bool());
        // Found on the object itself: the children are not looked at.
        assert!(calls_to(&take_log(&mut e), CHILD_COUNT).is_empty());
        assert!(!e.call(0x004b_5bf0, &args![Ptr::<()>::NULL]).bool());
    }

    #[test]
    fn fn_004b5c80_looks_for_the_class_on_the_object_then_below() {
        let mut e = engine();
        let g = graph(&mut e);
        e.register(IS_OF_CLASS, |e, a| {
            assert_eq!(a[0], IS_OF_CLASS_KEY_012024E0);
            ret_u((e.mem.u32(a[1] + 0x28) != 0) as u32)
        });
        let (a, b) = (graph_node(&mut e, &g, &[]), graph_node(&mut e, &g, &[]));
        let inner = graph_node(&mut e, &g, &[a, b]);
        let root = graph_node(&mut e, &g, &[inner]);
        assert!(!e.call(0x004b_5c80, &args![p(root)]).bool());
        e.mem.set_u32(a + 0x28, 1);
        assert!(e.call(0x004b_5c80, &args![p(root)]).bool());
        e.mem.set_u32(a + 0x28, 0);
        e.mem.set_u32(root + 0x28, 1);
        assert!(e.call(0x004b_5c80, &args![p(root)]).bool());
        assert!(!e.call(0x004b_5c80, &args![Ptr::<()>::NULL]).bool());
    }

    fn marker_engine() -> (Engine, Graph) {
        let mut e = engine();
        e.map(MARKER_ROOT & !0xfff, 0x1000);
        let g = graph(&mut e);
        name_doubles(&mut e);
        e.register(EXTRA_DATA_NAME, |_, _| ret_u(0x77));
        e.register(GET_EXTRA_DATA, |e, a| {
            assert_eq!(a[1], 0x77);
            ret_u(e.mem.u32(a[0] + 0x2c))
        });
        e.register(EXTRA_DATA_TEST, |_, a| {
            assert_eq!(a[1], 0x20);
            ret_u((a[0] == 0x2222) as u32)
        });
        e.register(GET_CONTROLLER, |e, a| {
            assert_eq!(a[1], MARKER_CONTROLLER_CLASS);
            ret_u(e.mem.u32(a[0] + 0x24))
        });
        e.register(FN_00537BD0, |e, a| ret_u(e.mem.u32(a[0])));
        e.register(WORD_AT_0X18, |e, a| ret_u(e.mem.u32(a[0] + 0x18)));
        e.set_global(MARKER_WORD_A, 0xa);
        e.set_global(MARKER_WORD_B, 0xb);
        e.set_global(MARKER_WORD_C, 0xc);
        e.set_global(MARKER_WORD_D, 0xd);
        (e, g)
    }

    #[test]
    fn remove_editor_markers_removes_the_marker_node_it_starts_on() {
        let (mut e, g) = marker_engine();
        // The marker node carries the extra data; its controller's value
        // (`00537bd0`) is an object with virtual slot 0x90; its parent has
        // slot 0xe8.
        let marker = graph_node(&mut e, &g, &[]);
        set_name(&mut e, marker, "EditorMarker");
        e.mem.set_u32(marker + 0x2c, 0x2222);
        let target = graph_node(&mut e, &g, &[]);
        let controller = e.mem.alloc(8);
        e.mem.set_u32(controller, target);
        e.mem.set_u32(marker + 0x24, controller);
        let parent = graph_node(&mut e, &g, &[]);
        e.mem.set_u32(marker + 0x18, parent);
        assert!(e.call(0x004b_5d10, &args![p(marker)]).bool());
        assert_eq!(
            *g.virtuals.borrow(),
            vec![
                // Each name with a null object (004b5de5: the 0 pushed before the getter).
                (0x90, vec![target, 0xa, 0]),
                (0x90, vec![target, 0xb, 0]),
                (0x90, vec![target, 0xc, 0]),
                (0x90, vec![target, 0xd, 0]),
                (0xe8, vec![parent, marker]),
            ]
        );
        // The root was this node: forgotten, with the controller value.
        assert_eq!(e.global::<u32>(MARKER_ROOT), 0);
        assert_eq!(e.global::<u32>(MARKER_CONTROLLER), 0);
    }

    #[test]
    fn remove_editor_markers_searches_the_children_of_a_known_root() {
        let (mut e, g) = marker_engine();
        let target = graph_node(&mut e, &g, &[]);
        e.set_global(MARKER_ROOT, 0x1234u32);
        e.set_global(MARKER_CONTROLLER, target);
        let marker = graph_node(&mut e, &g, &[]);
        let below = graph_node(&mut e, &g, &[]);
        let marker = {
            // The marker needs children to be searched as a container.
            g.children.borrow_mut().insert(marker, vec![below]);
            marker
        };
        set_name(&mut e, marker, "EditorMarker");
        let parent = graph_node(&mut e, &g, &[]);
        e.mem.set_u32(marker + 0x18, parent);
        let other = graph_node(&mut e, &g, &[]);
        let root = graph_node(&mut e, &g, &[other, marker]);
        e.call_log = Some(vec![]);
        assert!(e.call(0x004b_5d10, &args![p(root)]).bool());
        // The extra data is not looked up while a root is remembered.
        assert!(calls_to(&take_log(&mut e), GET_EXTRA_DATA).is_empty());
        let virtuals = g.virtuals.borrow().clone();
        assert_eq!(virtuals.len(), 5);
        assert_eq!(virtuals[4], (0xe8, vec![parent, marker]));
        // The remembered root is not this node: kept.
        assert_eq!(e.global::<u32>(MARKER_ROOT), 0x1234);
        // No controller value: only the removal request.
        g.virtuals.borrow_mut().clear();
        e.set_global(MARKER_CONTROLLER, 0u32);
        assert!(e.call(0x004b_5d10, &args![p(root)]).bool());
        assert_eq!(*g.virtuals.borrow(), vec![(0xe8, vec![parent, marker])]);
        // No parent: nothing to ask.
        g.virtuals.borrow_mut().clear();
        e.mem.set_u32(marker + 0x18, 0);
        assert!(e.call(0x004b_5d10, &args![p(root)]).bool());
        assert!(g.virtuals.borrow().is_empty());
    }

    #[test]
    fn remove_editor_markers_remembers_a_root_with_the_extra_data_and_drops_it() {
        let (mut e, g) = marker_engine();
        let target = graph_node(&mut e, &g, &[]);
        let controller = e.mem.alloc(8);
        e.mem.set_u32(controller, target);
        let leaf = graph_node(&mut e, &g, &[]);
        set_name(&mut e, leaf, "Plain");
        let root = graph_node(&mut e, &g, &[leaf]);
        e.mem.set_u32(root + 0x2c, 0x2222);
        e.mem.set_u32(root + 0x24, controller);
        // No marker anywhere: false, and the root it remembered is dropped.
        e.call_log = Some(vec![]);
        assert!(!e.call(0x004b_5d10, &args![p(root)]).bool());
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, FN_00537BD0), vec![args![controller]]);
        assert_eq!(e.global::<u32>(MARKER_ROOT), 0);
        assert_eq!(e.global::<u32>(MARKER_CONTROLLER), 0);
        assert!(g.virtuals.borrow().is_empty());
        // Extra data that the test rejects, or none: nothing is remembered.
        e.mem.set_u32(root + 0x2c, 0x3333);
        assert!(!e.call(0x004b_5d10, &args![p(root)]).bool());
        e.mem.set_u32(root + 0x2c, 0);
        assert!(!e.call(0x004b_5d10, &args![p(root)]).bool());
        assert_eq!(e.global::<u32>(MARKER_ROOT), 0);
        // Null node.
        assert!(!e.call(0x004b_5d10, &args![Ptr::<()>::NULL]).bool());
    }

    #[test]
    fn the_four_marker_getters_return_their_globals() {
        let mut e = engine();
        for (i, (address, global)) in [
            (0x004b_5f60u32, MARKER_WORD_A),
            (0x004b_5f70, MARKER_WORD_B),
            (0x004b_5f80, MARKER_WORD_C),
            (0x004b_5f90, MARKER_WORD_D),
        ]
        .into_iter()
        .enumerate()
        {
            e.set_global(global, 0x100 + i as u32);
            assert_eq!(e.call(address, &[]).u32(), 0x100 + i as u32);
        }
    }

    #[test]
    fn fn_004b5fa0_is_whether_the_test_with_0x20_is_non_zero() {
        let mut e = engine();
        e.register(EXTRA_DATA_TEST, |_, a| {
            assert_eq!(a[1], 0x20);
            ret_u(a[0])
        });
        assert!(e.call(0x004b_5fa0, &args![0x1234_0000u32]).bool());
        assert!(!e.call(0x004b_5fa0, &args![0u32]).bool());
    }

    /// The six planes (each four floats: a normal and the constant at
    /// `+0xc`); `dot` is the dot product of the plane's normal with the
    /// point.
    fn frustum_engine(planes: [[f32; 4]; 6]) -> Engine {
        let mut e = engine();
        e.map(FRUSTUM_GUARD & !0xfff, 0x1000);
        let block = e.mem.alloc(6 * 16);
        for (i, plane) in planes.iter().enumerate() {
            for (j, v) in plane.iter().enumerate() {
                e.mem.set_f32(block + 16 * i as u32 + 4 * j as u32, *v);
            }
        }
        e.register(FRUSTUM_PLANES_CONSTRUCT, |_, a| {
            assert_eq!(a[0], FRUSTUM_PLANES);
            Ret::default()
        });
        e.register(FRUSTUM_PLANES_SET, |_, a| {
            assert_eq!(a[0], FRUSTUM_PLANES);
            Ret::default()
        });
        e.register(CAMERA_STAMP, |_, a| {
            assert_eq!(a[0], CAMERA_STAMP_OWNER);
            ret_u(7)
        });
        e.register_double(FRUSTUM_PLANE, move |_, a| {
            assert_eq!(a[0], FRUSTUM_PLANES);
            ret_u(block + 16 * a[1])
        });
        e.register(POINT3_DOT, |e, a| {
            let sum: f64 = (0..3)
                .map(|i| e.mem.f32(a[0] + 4 * i) as f64 * e.mem.f32(a[1] + 4 * i) as f64)
                .sum();
            ret_f(sum)
        });
        e
    }

    /// A bound: centre and radius.
    fn bound_at(e: &mut Engine, centre: [f32; 3], radius: f32) -> Ptr {
        p(floats(e, &[centre[0], centre[1], centre[2], radius]))
    }

    const INSIDE: [[f32; 4]; 6] = [[0.0, 0.0, 1.0, -5.0]; 6];

    #[test]
    fn fn_004b5ff0_tests_the_bound_against_the_six_planes() {
        let mut e = frustum_engine(INSIDE);
        let bound = bound_at(&mut e, [0.0, 0.0, 0.0], 1.0);
        let camera = p(0x5000);
        // A null camera: false, nothing touched.
        e.call_log = Some(vec![]);
        assert!(!e.call(0x004b_5ff0, &args![bound, Ptr::<()>::NULL]).bool());
        assert!(take_log(&mut e).len() == 1);
        // First call: the planes are constructed and set for this camera.
        e.call_log = Some(vec![]);
        assert!(e.call(0x004b_5ff0, &args![bound, camera]).bool());
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, FRUSTUM_PLANES_CONSTRUCT).len(), 1);
        assert_eq!(
            calls_to(&log, FRUSTUM_PLANES_SET),
            vec![args![FRUSTUM_PLANES, camera]]
        );
        assert_eq!(calls_to(&log, FRUSTUM_PLANE).len(), 6);
        assert_eq!(e.global::<u32>(FRUSTUM_GUARD) & 1, 1);
        assert_eq!(e.global::<u32>(FRUSTUM_CAMERA), camera.addr());
        assert_eq!(e.global::<u32>(FRUSTUM_STAMP), 7);
        // Same camera and stamp: nothing is recomputed.
        e.call_log = Some(vec![]);
        assert!(e.call(0x004b_5ff0, &args![bound, camera]).bool());
        let log = take_log(&mut e);
        assert!(calls_to(&log, FRUSTUM_PLANES_CONSTRUCT).is_empty());
        assert!(calls_to(&log, FRUSTUM_PLANES_SET).is_empty());
        // A changed stamp, or another camera: recomputed.
        e.set_global(FRUSTUM_STAMP, 3u32);
        e.call_log = Some(vec![]);
        assert!(e.call(0x004b_5ff0, &args![bound, camera]).bool());
        assert_eq!(calls_to(&take_log(&mut e), FRUSTUM_PLANES_SET).len(), 1);
        e.call_log = Some(vec![]);
        assert!(e.call(0x004b_5ff0, &args![bound, p(0x6000)]).bool());
        assert_eq!(calls_to(&take_log(&mut e), FRUSTUM_PLANES_SET).len(), 1);
    }

    #[test]
    fn fn_004b5ff0_stops_at_the_first_plane_that_has_the_bound_behind_it() {
        let mut planes = INSIDE;
        planes[2] = [0.0, 0.0, 1.0, 5.0];
        let mut e = frustum_engine(planes);
        let bound = bound_at(&mut e, [0.0, 0.0, 0.0], 1.0);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x004b_5ff0, &args![bound, p(0x5000)]).bool());
        assert_eq!(calls_to(&take_log(&mut e), FRUSTUM_PLANE).len(), 3);
    }

    #[test]
    fn fn_004b5fc0_tests_the_world_bound_of_the_object() {
        let mut planes = INSIDE;
        planes[5] = [0.0, 0.0, 1.0, 5.0];
        let mut e = frustum_engine(INSIDE);
        e.register(GET_WORLD_BOUND, |e, a| ret_u(e.mem.u32(a[0])));
        let bound = bound_at(&mut e, [0.0, 0.0, 0.0], 1.0);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, bound.addr());
        assert!(e.call(0x004b_5fc0, &args![p(object), p(0x5000)]).bool());
        assert!(!e
            .call(0x004b_5fc0, &args![Ptr::<()>::NULL, p(0x5000)])
            .bool());
        assert!(!e
            .call(0x004b_5fc0, &args![p(object), Ptr::<()>::NULL])
            .bool());
        // Another set of planes with the last one against the bound.
        let mut e = frustum_engine(planes);
        e.register(GET_WORLD_BOUND, |e, a| ret_u(e.mem.u32(a[0])));
        let bound = bound_at(&mut e, [0.0, 0.0, 0.0], 1.0);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, bound.addr());
        assert!(!e.call(0x004b_5fc0, &args![p(object), p(0x5000)]).bool());
    }

    #[test]
    fn ni_bound_which_side_compares_the_distance_with_the_radius() {
        let mut e = engine();
        e.register(POINT3_DOT, |e, a| {
            ret_f((e.mem.f32(a[0]) * e.mem.f32(a[1])) as f64)
        });
        // Plane normal (1, 0, 0), constant `d`; the bound's centre x is 3,
        // so the distance is 3 - d. Radius 1.
        for (d, want) in [
            (6.0f32, 2),
            (4.0, 2),
            (3.5, 0),
            (3.0, 0),
            (2.5, 0),
            (2.0, 1),
            (-5.0, 1),
        ] {
            let bound = bound_at(&mut e, [3.0, 0.0, 0.0], 1.0);
            let plane = p(floats(&mut e, &[1.0, 0.0, 0.0, d]));
            assert_eq!(
                e.call(0x004b_6100, &args![bound, plane]).i32(),
                want,
                "d = {d}"
            );
        }
        // Not a number: neither side.
        let bound = bound_at(&mut e, [f32::NAN, 0.0, 0.0], 1.0);
        let plane = p(floats(&mut e, &[1.0, 0.0, 0.0, 0.0]));
        assert_eq!(e.call(0x004b_6100, &args![bound, plane]).i32(), 0);
    }

    #[test]
    fn fn_004b6160_is_the_dot_product_minus_the_plane_constant() {
        let mut e = engine();
        let plane = e.mem.alloc(16);
        e.mem.set_f32(plane + 0xc, 0.5);
        e.register_double(POINT3_DOT, move |_, a| {
            assert_eq!((a[0], a[1]), (plane, 0x2000));
            ret_f(2.0)
        });
        assert_eq!(e.call(0x004b_6160, &args![p(plane), p(0x2000)]).f32(), 1.5);
    }

    fn tint_engine() -> (Engine, Graph) {
        let mut e = engine();
        let g = graph(&mut e);
        e.set_global(TINT_ALPHA, 0.5f32);
        e.register(SMART_POINTER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret_u(a[0])
        });
        e.register(SMART_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret_u(a[0])
        });
        e.register(SMART_POINTER_GET, |e, a| ret_u(e.mem.u32(a[0])));
        e.register(SMART_POINTER_RELEASE, |_, _| Ret::default());
        e.register(NI_OPERATOR_NEW, |e, a| ret_u(e.mem.alloc(a[0])));
        e.register(EFFECT_DATA_CONSTRUCT, |_, a| ret_u(a[0]));
        e.register(COLOUR_CONSTRUCT, |e, a| {
            for i in 0..4 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
            }
            ret_u(a[0])
        });
        e.register(GET_PROPERTY, |e, a| {
            assert_eq!(a[1], 3);
            ret_u(e.mem.u32(a[0] + 0x30))
        });
        e.register(DYNAMIC_CAST, |_, a| {
            assert_eq!(a[0], CAST_CLASS_011FA010);
            ret_u(a[1])
        });
        e.register(SET_TEXTURE_EFFECT_DATA, |_, _| Ret::default());
        (e, g)
    }

    /// Three nodes: a root with two leaves; the second leaf has no property.
    fn tint_tree(e: &mut Engine, g: &Graph) -> (u32, u32, u32) {
        let first = graph_node(e, g, &[]);
        let second = graph_node(e, g, &[]);
        let root = graph_node(e, g, &[first, second]);
        e.mem.set_u32(root + 0x30, 0x1001);
        e.mem.set_u32(first + 0x30, 0x1002);
        (root, first, second)
    }

    #[test]
    fn tint_scenegraph_makes_the_effect_data_once_and_shares_it() {
        let (mut e, g) = tint_engine();
        let (root, first, _second) = tint_tree(&mut e, &g);
        let colour = p(floats(&mut e, &[0.25, 0.5, 0.75]));
        e.call_log = Some(vec![]);
        e.call(0x004b_6360, &args![p(root), colour, Ptr::<()>::NULL]);
        let log = take_log(&mut e);
        // Made once, 0x74 bytes; each node's call holds and releases it.
        assert_eq!(calls_to(&log, NI_OPERATOR_NEW), vec![args![0x74u32]]);
        assert_eq!(calls_to(&log, SMART_POINTER_CONSTRUCT).len(), 3);
        assert_eq!(calls_to(&log, SMART_POINTER_RELEASE).len(), 3);
        let sets = calls_to(&log, SET_TEXTURE_EFFECT_DATA);
        assert_eq!(sets.len(), 2);
        assert_eq!(sets[0][0], 0x1001);
        assert_eq!(sets[1][0], 0x1002);
        let data = sets[0][1];
        assert_eq!(sets[1][1], data);
        assert_ne!(data, 0);
        // The four colours and the four words.
        for offset in [0x2c, 0x3c, 0x0c, 0x1c] {
            assert_eq!(
                read_floats(&e, data + offset, 4),
                vec![0.25, 0.5, 0.75, 0.5]
            );
        }
        assert_eq!(
            [0x5c, 0x60, 0x64, 0x68].map(|o| e.mem.u32(data + o)),
            [9, 10, 1, 4]
        );
        // Children were handed the same object, not null.
        let constructs = calls_to(&log, SMART_POINTER_CONSTRUCT);
        assert_eq!(constructs[1][1], data);
        assert_eq!(constructs[2][1], data);
        let _ = first;
    }

    #[test]
    fn tint_scenegraph_uses_the_effect_data_it_is_given() {
        let (mut e, g) = tint_engine();
        let (root, _, _) = tint_tree(&mut e, &g);
        let colour = p(floats(&mut e, &[1.0, 1.0, 1.0]));
        let data = e.mem.alloc(0x74);
        e.call_log = Some(vec![]);
        e.call(0x004b_6360, &args![p(root), colour, p(data)]);
        let log = take_log(&mut e);
        assert!(calls_to(&log, NI_OPERATOR_NEW).is_empty());
        assert!(calls_to(&log, COLOUR_CONSTRUCT).is_empty());
        let sets = calls_to(&log, SET_TEXTURE_EFFECT_DATA);
        assert_eq!(sets, vec![args![0x1001u32, data], args![0x1002u32, data]]);
        assert_eq!(e.mem.u32(data + 0x5c), 0);
    }

    #[test]
    fn un_tint_scenegraph_clears_the_effect_data_below_the_node() {
        let (mut e, g) = tint_engine();
        let (root, _, _) = tint_tree(&mut e, &g);
        e.call_log = Some(vec![]);
        e.call(0x004b_6630, &args![p(root)]);
        let log = take_log(&mut e);
        assert_eq!(
            calls_to(&log, SET_TEXTURE_EFFECT_DATA),
            vec![args![0x1001u32, 0u32], args![0x1002u32, 0u32]]
        );
    }

    #[test]
    fn get_collision_object_count_in_scene_graph_walks_with_the_flags() {
        let mut e = engine();
        let seen: Shared<Vec<WalkCall>> = Rc::default();
        {
            let seen = seen.clone();
            e.register_double(WALK_SCENE_GRAPH, move |e, a| {
                let words = (0..7).map(|i| e.mem.u32(a[1] + 4 * i)).collect();
                seen.borrow_mut().push((a[0], a[2], words));
                // The callback leaves its count at +0xc.
                e.mem.set_u32(a[1] + 0xc, 9);
                Ret::default()
            });
        }
        let r = e.call(0x004b_66d0, &args![0x5000u32, true, false, true]);
        assert_eq!(r.u32(), 9);
        assert_eq!(
            *seen.borrow(),
            vec![(0x5000, COUNT_CALLBACK, vec![0, 1, 0x12, 0, 0, 0, 5])]
        );
        e.call(0x004b_66d0, &args![0x5000u32, false, true, false]);
        assert_eq!(seen.borrow()[1].2[6], 2);
        e.call(0x004b_66d0, &args![0x5000u32, false, false, false]);
        assert_eq!(seen.borrow()[2].2[6], 0);
    }

    #[test]
    fn fn_004b6740_counts_the_objects_the_flags_do_not_skip() {
        let mut e = engine();
        name_doubles(&mut e);
        e.register(WORD_AT_EIGHT, |e, a| ret_u(e.mem.u32(a[0] + 8)));
        e.register(IS_OF_CLASS, |e, a| {
            if a[1] == 0 {
                return ret_u(0);
            }
            match a[0] {
                IS_OF_CLASS_KEY_0126817C => ret_u(e.mem.u32(a[1] + 0x28)),
                IS_OF_CLASS_KEY_011F9140 => ret_u(e.mem.u32(a[1] + 0x2c)),
                other => panic!("class {other:08x}"),
            }
        });
        let object = e.mem.alloc(0x60);
        let inner = e.mem.alloc(0x60);
        e.mem.set_u32(object + 8, inner);
        let context = e.mem.alloc(0x20);
        let count = |e: &mut Engine, flags: u32| {
            e.mem.set_u32(context + 0x18, flags);
            e.mem.set_u32(context + 0xc, 10);
            e.call(0x004b_6740, &args![p(object), p(context)]);
            e.mem.u32(context + 0xc)
        };
        // No flags: counted whatever the object is.
        e.mem.set_u32(object + 0x28, 1);
        e.mem.set_u32(inner + 0x2c, 1);
        assert_eq!(count(&mut e, 0), 11);
        // Flag 1 skips the objects of the first class.
        assert_eq!(count(&mut e, 1), 10);
        e.mem.set_u32(object + 0x28, 0);
        assert_eq!(count(&mut e, 1), 11);
        // Flag 2 skips an object whose inner object is of the second class.
        assert_eq!(count(&mut e, 2), 10);
        e.mem.set_u32(inner + 0x2c, 0);
        // Without a name, or with another name, it is counted; "Arrow" is not.
        assert_eq!(count(&mut e, 2), 11);
        set_name(&mut e, inner, "Quiver");
        assert_eq!(count(&mut e, 2), 11);
        set_name(&mut e, inner, "Arrow");
        assert_eq!(count(&mut e, 2), 10);
        // No inner object at all.
        e.mem.set_u32(object + 8, 0);
        assert_eq!(count(&mut e, 2), 11);
        assert_eq!(count(&mut e, 3), 11);
    }

    #[test]
    fn fn_004b6810_returns_the_index_the_callback_stored() {
        let mut e = engine();
        let seen: Shared<Vec<WalkCall>> = Rc::default();
        {
            let seen = seen.clone();
            e.register_double(WALK_SCENE_GRAPH, move |e, a| {
                let words = (0..7).map(|i| e.mem.u32(a[1] + 4 * i)).collect();
                seen.borrow_mut().push((a[0], a[2], words));
                // The second walk finds index 5.
                if seen.borrow().len() == 2 {
                    e.mem.set_u32(a[1] + 0x14, 5);
                }
                Ret::default()
            });
        }
        // Nothing found: -1.
        let r = e.call(0x004b_6810, &args![0x5000u32, 0x77u32, false, true, true]);
        assert_eq!(r.i32(), -1);
        assert_eq!(
            seen.borrow()[0],
            (
                0x5000,
                FIND_CALLBACK,
                vec![0, 1, 0x12, 0, 0x77, 0xffff_ffff, 6]
            )
        );
        let r = e.call(0x004b_6810, &args![0x5000u32, 0x77u32, true, false, false]);
        assert_eq!(r.i32(), 5);
        assert_eq!(seen.borrow()[1].2[6], 1);
    }

    // ---- third session: more walker callbacks, language swap, maps ----

    /// A walker recorder: registers a double over the walker `00c68900`
    /// that records the root, the callback and the context's seven words,
    /// and lets `fill` write results into the context (it gets the number
    /// of the walk, from 1).
    fn record_walks(
        e: &mut Engine,
        fill: impl Fn(&mut Engine, u32, usize) + 'static,
    ) -> Shared<Vec<WalkCall>> {
        let seen: Shared<Vec<WalkCall>> = Rc::default();
        let log = seen.clone();
        e.register_double(WALK_SCENE_GRAPH, move |e, a| {
            let words = (0..7).map(|i| e.mem.u32(a[1] + 4 * i)).collect();
            log.borrow_mut().push((a[0], a[2], words));
            let n = log.borrow().len();
            fill(e, a[1], n);
            Ret::default()
        });
        seen
    }

    /// Doubles for the skip tests of `004b68a0` and `004b6a00`: the object's
    /// `+8` word is its inner object, the class tests read the flags at
    /// `+0x28` (class `0126817c`) and `+0x2c` (class `011f9140`) of what
    /// they are given. Returns the engine, an object, its inner object and
    /// a context block.
    fn skip_engine() -> (Engine, u32, u32, u32) {
        let mut e = engine();
        name_doubles(&mut e);
        e.register(WORD_AT_EIGHT, |e, a| ret_u(e.mem.u32(a[0] + 8)));
        e.register(IS_OF_CLASS, |e, a| {
            if a[1] == 0 {
                return ret_u(0);
            }
            match a[0] {
                IS_OF_CLASS_KEY_0126817C => ret_u(e.mem.u32(a[1] + 0x28)),
                IS_OF_CLASS_KEY_011F9140 => ret_u(e.mem.u32(a[1] + 0x2c)),
                other => panic!("class {other:08x}"),
            }
        });
        let object = e.mem.alloc(0x60);
        let inner = e.mem.alloc(0x60);
        e.mem.set_u32(object + 8, inner);
        let context = e.mem.alloc(0x20);
        (e, object, inner, context)
    }

    #[test]
    fn fn_004b68a0_stores_the_count_when_it_meets_the_object() {
        let (mut e, object, inner, context) = skip_engine();
        let other = e.mem.alloc(0x60);
        e.mem.set_u32(context + 0xc, 3);
        e.mem.set_u32(context + 0x10, object);
        e.mem.set_u32(context + 0x14, 0x77);
        // Another object: counted, nothing stored.
        e.call(0x004b_68a0, &args![p(other), p(context)]);
        assert_eq!(
            (e.mem.u32(context + 0xc), e.mem.u32(context + 0x14)),
            (4, 0x77)
        );
        // The object: the count before it is stored.
        e.call(0x004b_68a0, &args![p(object), p(context)]);
        assert_eq!(
            (e.mem.u32(context + 0xc), e.mem.u32(context + 0x14)),
            (5, 4)
        );
        // Flag 1 and the first class: skipped, nothing changes.
        e.mem.set_u32(context + 0x18, 1);
        e.mem.set_u32(object + 0x28, 1);
        e.mem.set_u32(context + 0x14, 0x77);
        e.call(0x004b_68a0, &args![p(object), p(context)]);
        assert_eq!(
            (e.mem.u32(context + 0xc), e.mem.u32(context + 0x14)),
            (5, 0x77)
        );
        // Flag 2: skipped when the inner object is of the second class, or
        // is named "Arrow"; counted with another name.
        e.mem.set_u32(context + 0x18, 2);
        e.mem.set_u32(inner + 0x2c, 1);
        e.call(0x004b_68a0, &args![p(object), p(context)]);
        assert_eq!(e.mem.u32(context + 0xc), 5);
        e.mem.set_u32(inner + 0x2c, 0);
        set_name(&mut e, inner, "Arrow");
        e.call(0x004b_68a0, &args![p(object), p(context)]);
        assert_eq!(e.mem.u32(context + 0xc), 5);
        set_name(&mut e, inner, "Quiver");
        e.call(0x004b_68a0, &args![p(object), p(context)]);
        assert_eq!(
            (e.mem.u32(context + 0xc), e.mem.u32(context + 0x14)),
            (6, 5)
        );
    }

    #[test]
    fn fn_004b6a00_stores_the_object_at_the_index() {
        let (mut e, object, _inner, context) = skip_engine();
        let other = e.mem.alloc(0x60);
        e.mem.set_u32(context + 0x14, 2);
        e.call(0x004b_6a00, &args![p(other), p(context)]);
        e.call(0x004b_6a00, &args![p(other), p(context)]);
        assert_eq!(
            (e.mem.u32(context + 0xc), e.mem.u32(context + 0x10)),
            (2, 0)
        );
        // A skipped object is not counted, so it does not take the index.
        e.mem.set_u32(context + 0x18, 1);
        e.mem.set_u32(other + 0x28, 1);
        e.call(0x004b_6a00, &args![p(other), p(context)]);
        assert_eq!(
            (e.mem.u32(context + 0xc), e.mem.u32(context + 0x10)),
            (2, 0)
        );
        e.call(0x004b_6a00, &args![p(object), p(context)]);
        assert_eq!(
            (e.mem.u32(context + 0xc), e.mem.u32(context + 0x10)),
            (3, object)
        );
    }

    #[test]
    fn fn_004b6980_returns_the_object_the_callback_stored() {
        let mut e = engine();
        let seen = record_walks(&mut e, |e, context, n| {
            if n == 2 {
                e.mem.set_u32(context + 0x10, 0xabcd);
            }
        });
        assert_eq!(
            e.call(0x004b_6980, &args![0x5000u32, 7u32, true, true, false])
                .u32(),
            0
        );
        assert_eq!(
            seen.borrow()[0],
            (0x5000, SELECT_CALLBACK, vec![0, 1, 0x12, 0, 0, 7, 3])
        );
        let r = e.call(0x004b_6980, &args![0x5000u32, 1u32, false, false, true]);
        assert_eq!(r.u32(), 0xabcd);
        assert_eq!(seen.borrow()[1].2[5..], [1, 4]);
    }

    #[test]
    fn get_havok_world_object_count_in_scene_graph_returns_the_count() {
        let mut e = engine();
        let seen = record_walks(&mut e, |e, context, _| e.mem.set_u32(context + 0xc, 6));
        assert_eq!(e.call(0x004b_6ae0, &args![0x5000u32]).u32(), 6);
        let walk = seen.borrow()[0].clone();
        assert_eq!((walk.0, walk.1), (0x5000, HAVOK_COUNT_CALLBACK));
        assert_eq!(walk.2[1..4], [1, 0x12, 0]);
    }

    /// Doubles for the callbacks that look at the word attached to an
    /// object: the smart pointer at `object + 0x10` is the pointed object,
    /// whose `+8` word is the attached word.
    fn attached_engine() -> Engine {
        let mut e = engine();
        e.register(POINTER_AT_0X10, |e, a| ret_u(e.mem.u32(a[0] + 0x10)));
        e.register(WORD_AT_EIGHT_WRAPPER, |e, a| ret_u(e.mem.u32(a[0] + 8)));
        e
    }

    /// An object with the attached word `word` (nothing attached for 0, and
    /// no pointed object at all for `None`).
    fn object_with_word(e: &mut Engine, word: Option<u32>) -> u32 {
        let object = e.mem.alloc(0x20);
        if let Some(word) = word {
            let pointed = e.mem.alloc(0x10);
            e.mem.set_u32(pointed + 8, word);
            e.mem.set_u32(object + 0x10, pointed);
        }
        object
    }

    #[test]
    fn fn_004b6b20_counts_the_objects_with_an_attached_word() {
        let mut e = attached_engine();
        let context = e.mem.alloc(0x20);
        for (word, want) in [(None, 0), (Some(0), 0), (Some(9), 1)] {
            let object = object_with_word(&mut e, word);
            e.mem.set_u32(context + 0xc, 0);
            e.call(0x004b_6b20, &args![p(object), p(context)]);
            assert_eq!(e.mem.u32(context + 0xc), want, "{word:?}");
        }
    }

    #[test]
    fn fn_004b6b60_returns_the_count_at_the_word_or_minus_one() {
        let mut e = engine();
        let seen = record_walks(&mut e, |e, context, n| {
            if n == 2 {
                e.mem.set_u32(context + 0x14, 3);
            }
        });
        assert_eq!(e.call(0x004b_6b60, &args![0x5000u32, 0x42u32]).i32(), -1);
        assert_eq!(
            seen.borrow()[0],
            (
                0x5000,
                HAVOK_FIND_CALLBACK,
                vec![0, 1, 0x12, 0, 0x42, 0xffff_ffff, 0]
            )
        );
        assert_eq!(e.call(0x004b_6b60, &args![0x5000u32, 0x42u32]).i32(), 3);
    }

    #[test]
    fn fn_004b6bc0_stores_the_count_at_the_matching_word() {
        let mut e = attached_engine();
        let context = e.mem.alloc(0x20);
        e.mem.set_u32(context + 0x10, 9);
        e.mem.set_u32(context + 0x14, 0xffff_ffff);
        // No word: not counted. A word that differs: counted.
        for word in [None, Some(0), Some(5)] {
            let object = object_with_word(&mut e, word);
            e.call(0x004b_6bc0, &args![p(object), p(context)]);
        }
        assert_eq!(
            (e.mem.u32(context + 0xc), e.mem.u32(context + 0x14)),
            (1, 0xffff_ffff)
        );
        // The word looked for: the count before it is stored.
        let object = object_with_word(&mut e, Some(9));
        e.call(0x004b_6bc0, &args![p(object), p(context)]);
        assert_eq!(
            (e.mem.u32(context + 0xc), e.mem.u32(context + 0x14)),
            (2, 1)
        );
    }

    #[test]
    fn fn_004b6c20_returns_the_word_the_callback_stored() {
        let mut e = engine();
        let seen = record_walks(&mut e, |e, context, _| e.mem.set_u32(context + 0x10, 0x31));
        assert_eq!(e.call(0x004b_6c20, &args![0x5000u32, 4u32]).u32(), 0x31);
        assert_eq!(
            seen.borrow()[0],
            (0x5000, HAVOK_INDEX_CALLBACK, vec![0, 1, 0x12, 0, 0, 4, 0])
        );
    }

    #[test]
    fn fn_004b6c70_counts_twice_for_an_object_with_a_word() {
        let mut e = attached_engine();
        let context = e.mem.alloc(0x20);
        e.mem.set_u32(context + 0x14, 2);
        // Without a word: one step. With one: two steps, so the count
        // jumps over the index and the word is not stored.
        let plain = object_with_word(&mut e, None);
        e.call(0x004b_6c70, &args![p(plain), p(context)]);
        assert_eq!(
            (e.mem.u32(context + 0xc), e.mem.u32(context + 0x10)),
            (1, 0)
        );
        let first = object_with_word(&mut e, Some(7));
        e.call(0x004b_6c70, &args![p(first), p(context)]);
        assert_eq!(
            (e.mem.u32(context + 0xc), e.mem.u32(context + 0x10)),
            (3, 0)
        );
        // A count equal to the index takes the word.
        e.mem.set_u32(context + 0xc, 2);
        let second = object_with_word(&mut e, Some(8));
        e.call(0x004b_6c70, &args![p(second), p(context)]);
        assert_eq!(
            (e.mem.u32(context + 0xc), e.mem.u32(context + 0x10)),
            (4, 8)
        );
    }

    /// Property doubles: a node's property is the word at `+0x30`; a
    /// property is a block whose kind is the word at `+0x1c`.
    fn property_engine() -> (Engine, Graph) {
        let (mut e, g) = {
            let mut e = engine();
            let g = graph(&mut e);
            (e, g)
        };
        e.register(GET_PROPERTY, |e, a| {
            assert_eq!(a[1], 3);
            ret_u(e.mem.u32(a[0] + 0x30))
        });
        e.register(IN_KIND_RANGE, |e, a| {
            ret_u((a[0] != 0 && (1..=0xc).contains(&e.mem.u32(a[0] + 0x1c))) as u32)
        });
        e.register(WORD_AT_0X1C, |e, a| ret_u(e.mem.u32(a[0] + 0x1c)));
        e.register(NOTIFY, |_, _| Ret::default());
        e.register(SET_REFERENCE_ID, |_, _| Ret::default());
        (e, g)
    }

    fn property_of_kind(e: &mut Engine, kind: u32) -> u32 {
        let property = e.mem.alloc(0x30);
        e.mem.set_u32(property + 0x1c, kind);
        property
    }

    /// A tree for `004b6ce0` (slot `0x1c`, word `+0x38`) and `004b6dc0`
    /// (slot `0x18`, word `+0x34`): a root with three children. `first`
    /// has the slot set, an accepted property (kind 3) and a child that
    /// would also qualify; `second` has the slot set and a property that
    /// is not accepted (kind 0xd); `third` has the slot set and no
    /// property. Returns the root and the accepted property.
    fn slot_tree(e: &mut Engine, g: &Graph, slot_word: u32) -> (u32, u32) {
        let accepted = property_of_kind(e, 3);
        let rejected = property_of_kind(e, 0xd);
        let deep = graph_node(e, g, &[]);
        e.mem.set_u32(deep + slot_word, 1);
        e.mem.set_u32(deep + 0x30, accepted);
        let first = graph_node(e, g, &[deep]);
        e.mem.set_u32(first + slot_word, 1);
        e.mem.set_u32(first + 0x30, accepted);
        let second = graph_node(e, g, &[]);
        e.mem.set_u32(second + slot_word, 1);
        e.mem.set_u32(second + 0x30, rejected);
        let third = graph_node(e, g, &[]);
        e.mem.set_u32(third + slot_word, 1);
        let root = graph_node(e, g, &[first, 0, second, third]);
        (root, accepted)
    }

    #[test]
    fn fn_004b6ce0_notifies_for_the_first_qualifying_objects_only() {
        let (mut e, g) = property_engine();
        let (root, accepted) = slot_tree(&mut e, &g, 0x38);
        e.call_log = Some(vec![]);
        e.call(0x004b_6ce0, &args![p(root)]);
        let log = take_log(&mut e);
        // One call: `first`'s (its child is not visited); the rejected
        // property and the missing one do nothing.
        assert_eq!(
            calls_to(&log, NOTIFY),
            vec![args![NOTIFIED_OBJECT, accepted]]
        );
        assert_eq!(calls_to(&log, WORD_AT_0X1C), vec![args![accepted]]);
        // A null object does nothing.
        e.call(0x004b_6ce0, &args![Ptr::<()>::NULL]);
    }

    #[test]
    fn set_reference_id_on_scenegraph_sets_it_on_the_accepted_properties() {
        let (mut e, g) = property_engine();
        let (root, accepted) = slot_tree(&mut e, &g, 0x34);
        e.call_log = Some(vec![]);
        e.call(0x004b_6dc0, &args![p(root), 0x00ff_1234u32]);
        let log = take_log(&mut e);
        assert_eq!(
            calls_to(&log, SET_REFERENCE_ID),
            vec![args![accepted, 0x00ff_1234u32]]
        );
        e.call(0x004b_6dc0, &args![Ptr::<()>::NULL, 1u32]);
    }

    #[test]
    fn fn_004b6e70_and_fn_004b70d0_read_the_smart_pointers_at_their_offsets() {
        let mut e = engine();
        e.register(SMART_POINTER_GET, |_, a| ret_u(a[0] + 1));
        assert_eq!(
            e.call(0x004b_6e70, &args![0x1000u32]).u32(),
            0x1000 + 0x13c + 1
        );
        assert_eq!(
            e.call(0x004b_70d0, &args![0x1000u32]).u32(),
            0x1000 + 0x3c + 1
        );
    }

    fn lighting_engine() -> (Engine, Graph) {
        let (mut e, g) = property_engine();
        e.register(NI_OPERATOR_NEW, |e, a| ret_u(e.mem.alloc(a[0])));
        e.register(NO_LIGHTING_PROPERTY_CONSTRUCT, |_, a| ret_u(a[0]));
        e.register(ATTACH_PROPERTY, |_, _| Ret::default());
        e.register(SET_SHADER_FLAG, |_, _| Ret::default());
        e.register(UPDATE_PROPERTIES, |_, _| Ret::default());
        e.register(PREPARE_OBJECT, |_, _| Ret::default());
        e.register(BOUND_RADIUS, |e, a| ret_f(e.mem.f32(a[0] + 0xc) as f64));
        (e, g)
    }

    #[test]
    fn add_no_lighting_property_recurse_attaches_to_the_objects_without_one() {
        let (mut e, g) = lighting_engine();
        let property = property_of_kind(&mut e, 3);
        // root: slot set, no property -> gets one. Its children: null
        // (skipped), `plain` (slot clear, has a child that gets one) and
        // `held` (slot set, already has a property).
        let deep = graph_node(&mut e, &g, &[]);
        e.mem.set_u32(deep + 0x34, 1);
        let plain = graph_node(&mut e, &g, &[deep]);
        let held = graph_node(&mut e, &g, &[]);
        e.mem.set_u32(held + 0x34, 1);
        e.mem.set_u32(held + 0x30, property);
        let root = graph_node(&mut e, &g, &[0, plain, held]);
        e.mem.set_u32(root + 0x34, 1);
        e.call_log = Some(vec![]);
        e.call(0x004b_6e90, &args![p(root)]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, NI_OPERATOR_NEW), vec![args![0x80u32]; 2]);
        let attached = calls_to(&log, ATTACH_PROPERTY);
        assert_eq!(attached.len(), 2);
        assert_eq!((attached[0][0], attached[1][0]), (root, deep));
        assert_ne!(attached[0][1], 0);
    }

    #[test]
    fn add_no_lighting_property_recurse_attaches_null_when_allocation_fails() {
        let (mut e, g) = lighting_engine();
        e.register(NI_OPERATOR_NEW, |_, _| Ret::default());
        let root = graph_node(&mut e, &g, &[]);
        e.mem.set_u32(root + 0x34, 1);
        e.call_log = Some(vec![]);
        e.call(0x004b_6e90, &args![p(root)]);
        let log = take_log(&mut e);
        assert!(calls_to(&log, NO_LIGHTING_PROPERTY_CONSTRUCT).is_empty());
        assert_eq!(calls_to(&log, ATTACH_PROPERTY), vec![args![root, 0u32]]);
    }

    #[test]
    fn prepare_object_with_no_lighting_recurse_prepares_the_first_one_it_finds() {
        let (mut e, g) = lighting_engine();
        let child = graph_node(&mut e, &g, &[]);
        let root = graph_node(&mut e, &g, &[child]);
        e.mem.set_u32(root + 0x34, 1);
        e.mem.set_u32(child + 0x34, 1);
        e.call_log = Some(vec![]);
        e.call(0x004b_6f90, &args![p(root)]);
        let log = take_log(&mut e);
        let order: Vec<u32> = log.iter().map(|(a, _)| *a).collect();
        assert_eq!(
            order,
            vec![
                0x004b_6f90,
                0x0300_5040,
                GET_PROPERTY,
                NI_OPERATOR_NEW,
                NO_LIGHTING_PROPERTY_CONSTRUCT,
                SET_SHADER_FLAG,
                ATTACH_PROPERTY,
                UPDATE_PROPERTIES,
                PREPARE_OBJECT
            ]
        );
        let property = calls_to(&log, ATTACH_PROPERTY)[0][1];
        assert_eq!(
            calls_to(&log, SET_SHADER_FLAG),
            vec![args![property, 0x30u32, 1u32]]
        );
        assert_eq!(
            calls_to(&log, PREPARE_OBJECT),
            vec![args![root, 0u32, 0u32]]
        );
        assert!(calls_to(&log, CHILD_COUNT).is_empty());
    }

    #[test]
    fn prepare_object_with_no_lighting_recurse_needs_a_world_bound_to_go_down() {
        let (mut e, g) = lighting_engine();
        let held = property_of_kind(&mut e, 3);
        let leaf = graph_node(&mut e, &g, &[]);
        e.mem.set_u32(leaf + 0x34, 1);
        let root = graph_node(&mut e, &g, &[0, leaf]);
        // The root's property exists, so it goes to the children, but its
        // world bound has radius 0: stops.
        e.mem.set_u32(root + 0x34, 1);
        e.mem.set_u32(root + 0x30, held);
        let bound = floats(&mut e, &[0.0, 0.0, 0.0, 0.0]);
        e.mem.set_u32(root + 0x20, bound);
        e.call_log = Some(vec![]);
        e.call(0x004b_6f90, &args![p(root)]);
        assert!(calls_to(&take_log(&mut e), PREPARE_OBJECT).is_empty());
        // A bound with a radius: the leaf gets prepared.
        let bound = floats(&mut e, &[0.0, 0.0, 0.0, 2.0]);
        e.mem.set_u32(root + 0x20, bound);
        e.call_log = Some(vec![]);
        e.call(0x004b_6f90, &args![p(root)]);
        assert_eq!(
            calls_to(&take_log(&mut e), PREPARE_OBJECT),
            vec![args![leaf, 0u32, 0u32]]
        );
        // No bound at all: stops.
        e.mem.set_u32(root + 0x20, 0);
        e.call_log = Some(vec![]);
        e.call(0x004b_6f90, &args![p(root)]);
        assert!(calls_to(&take_log(&mut e), PREPARE_OBJECT).is_empty());
    }

    #[test]
    fn fn_004b70f0_sets_the_value_on_the_items_and_the_children_of_the_class() {
        let mut e = engine();
        let g = graph(&mut e);
        // Items chain through `+0x30`; a container's first item is at `+0xc`.
        e.register(FIRST_ITEM, |e, a| ret_u(e.mem.u32(a[0] + 0xc)));
        e.register(NEXT_ITEM, |e, a| ret_u(e.mem.u32(a[0] + 0x30)));
        e.register(SET_FLOAT_AT_0X10, |_, _| Ret::default());
        e.register(IS_OF_CLASS, |e, a| {
            assert_eq!(a[0], IS_OF_CLASS_KEY_011F4428);
            ret_u(e.mem.u32(a[1] + 0x28))
        });
        let item = |e: &mut Engine, next: u32| {
            let item = e.mem.alloc(0x40);
            e.mem.set_u32(item + 0x30, next);
            item
        };
        let b = item(&mut e, 0);
        let a = item(&mut e, b);
        let c = item(&mut e, 0);
        let d = item(&mut e, 0);
        let classed = graph_node(&mut e, &g, &[]);
        e.mem.set_u32(classed + 0x28, 1);
        e.mem.set_u32(classed + 0xc, c);
        let other = graph_node(&mut e, &g, &[]);
        e.mem.set_u32(other + 0xc, d);
        let root = graph_node(&mut e, &g, &[0, classed, other]);
        e.mem.set_u32(root + 0xc, a);
        e.call_log = Some(vec![]);
        e.call(0x004b_70f0, &args![p(root), 2.5f32]);
        let log = take_log(&mut e);
        let value = 2.5f32.to_bits();
        assert_eq!(
            calls_to(&log, SET_FLOAT_AT_0X10),
            vec![args![a, value], args![b, value], args![c, value]]
        );
        e.call(0x004b_70f0, &args![Ptr::<()>::NULL, 1.0f32]);
    }

    fn language_state_engine() -> Engine {
        let mut e = engine();
        e.map(0x011d_8000, 0x1000);
        e
    }

    #[test]
    fn fn_004b71d0_and_fn_004b71a0_need_the_enabled_byte_and_an_empty_string() {
        let mut e = language_state_engine();
        // Disabled.
        assert!(!e.call(0x004b_71d0, &[]).bool());
        assert!(!e.call(0x004b_71a0, &[]).bool());
        // Enabled: the byte at +4 of the string object decides.
        e.mem.set_u8(PLATFORM_SWAP_ENABLED, 1);
        assert!(e.call(0x004b_71d0, &[]).bool());
        assert!(e.call(0x004b_71a0, &[]).bool());
        e.mem.set_u8(PLATFORM_SWAP_STRING + 4, b'x');
        assert!(!e.call(0x004b_71d0, &[]).bool());
        assert!(!e.call(0x004b_71a0, &[]).bool());
    }

    #[test]
    fn fn_004b7210_reads_the_word() {
        let mut e = language_state_engine();
        e.mem.set_u32(PLATFORM_SWAP_WORD, 0x5a5a);
        assert_eq!(e.call(0x004b_7210, &[]).u32(), 0x5a5a);
    }

    #[test]
    fn get_current_tex_language_returns_what_the_worker_gives() {
        let mut e = engine();
        e.register(CURRENT_LANGUAGE_WORKER, |_, a| {
            assert!(a.is_empty());
            ret_u(3)
        });
        assert_eq!(e.call(0x004b_7220, &[]).u32(), 3);
    }

    /// The tables of the language swap (platforms `PC`, `X360`, `PS3`,
    /// `Other`; languages `ENGLISH`, `FRENCH`, `GERMAN`, `ITALIAN`,
    /// `SPANISH`), the current language (`GERMAN`, 2), the state page, and
    /// doubles for the string functions and `memcpy_s`.
    fn swap_engine() -> Engine {
        let mut e = language_state_engine();
        for (i, name) in ["PC", "X360", "PS3", "Other"].iter().enumerate() {
            let t = text(&mut e, name);
            e.mem.set_u32(PLATFORM_NAMES + 4 * i as u32, t.addr());
        }
        for (i, name) in ["ENGLISH", "FRENCH", "GERMAN", "ITALIAN", "SPANISH"]
            .iter()
            .enumerate()
        {
            let t = text(&mut e, name);
            e.mem.set_u32(LANGUAGE_NAMES + 4 * i as u32, t.addr());
        }
        e.register(CURRENT_LANGUAGE_WORKER, |_, _| ret_u(2));
        e.register(STRING_LENGTH, |e, a| ret_u(e.mem.cstr(a[0]).len() as u32));
        e.register(STRING_CONCAT, |e, a| {
            let used = e.mem.cstr(a[0]).len() as u32;
            let source = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0] + used, &source);
            ret_u(0)
        });
        e.register(CRT_MEMCPY_S, |e, a| {
            let bytes = e.mem.bytes(a[2], a[3]);
            e.mem.write(a[0], &bytes);
            ret_u(0)
        });
        e
    }

    fn swapped(e: &mut Engine, path: &str) -> (bool, String) {
        let input = text(e, path);
        let out = p(e.mem.alloc(0x100));
        let r = e.call(0x004b_7240, &args![input, out, 0x100u32]).bool();
        (r, String::from_utf8(e.mem.cstr(out.addr())).unwrap())
    }

    #[test]
    fn swap_platform_language_texture_path_replaces_the_platform_and_language_directories() {
        let mut e = swap_engine();
        // Platform 1 (X360), language 2 (GERMAN).
        e.mem.set_u8(PLATFORM_SWAP_ENABLED, 1);
        assert_eq!(
            swapped(&mut e, "data/textures/pc/english/x.dds"),
            (true, "data/textures/X360/GERMAN/x.dds".into())
        );
        // Backslashes are kept as the separator found.
        assert_eq!(swapped(&mut e, "pc\\x.dds"), (true, "X360\\x.dds".into()));
        // Only the language: the text before it is copied.
        assert_eq!(
            swapped(&mut e, "a/French/b.dds"),
            (true, "a/GERMAN/b.dds".into())
        );
        // Platform 0 when the swap is disabled.
        e.mem.set_u8(PLATFORM_SWAP_ENABLED, 0);
        assert_eq!(swapped(&mut e, "t/ps3/x.dds"), (true, "t/PC/x.dds".into()));
    }

    #[test]
    fn swap_platform_language_texture_path_leaves_other_paths_alone() {
        let mut e = swap_engine();
        // A name not followed by a separator is not a directory.
        assert_eq!(
            swapped(&mut e, "data/pcx/file.dds"),
            (false, "data/pcx/file.dds".into())
        );
        assert_eq!(swapped(&mut e, "plain.dds"), (false, "plain.dds".into()));
        assert_eq!(swapped(&mut e, ""), (false, String::new()));
        // Null pointers or no room: false, nothing written.
        let input = text(&mut e, "pc/x");
        let out = p(e.mem.alloc(0x10));
        e.mem.set_cstr(out.addr(), b"keep");
        assert!(!e
            .call(0x004b_7240, &args![Ptr::<()>::NULL, out, 8u32])
            .bool());
        assert!(!e
            .call(0x004b_7240, &args![input, Ptr::<()>::NULL, 8u32])
            .bool());
        assert!(!e.call(0x004b_7240, &args![input, out, 0u32]).bool());
        assert_eq!(e.mem.cstr(out.addr()), b"keep");
    }

    #[test]
    fn fn_004b7580_is_memcpy_s() {
        let mut e = engine();
        e.register(CRT_MEMCPY_S, |_, a| {
            assert_eq!(a, [0x10, 0x20, 0x30, 4]);
            ret_u(0x55)
        });
        assert_eq!(
            e.call(0x004b_7580, &args![0x10u32, 0x20u32, 0x30u32, 4u32])
                .u32(),
            0x55
        );
    }

    #[test]
    fn bs_stristr_finds_the_needle_ignoring_case() {
        let mut e = engine();
        let hay = text(&mut e, "Data\\Textures\\PC\\x");
        let find = |e: &mut Engine, needle: &str| {
            let n = text(e, needle);
            e.call(0x004b_75a0, &args![hay, n]).u32()
        };
        assert_eq!(find(&mut e, "textures"), hay.addr() + 5);
        assert_eq!(find(&mut e, "pc\\"), hay.addr() + 14);
        assert_eq!(find(&mut e, "x"), hay.addr() + 7);
        assert_eq!(find(&mut e, "\\X"), hay.addr() + 16);
        assert_eq!(find(&mut e, "data"), hay.addr());
        assert_eq!(find(&mut e, "missing"), 0);
        // An empty needle matches at the start; a longer one never.
        assert_eq!(find(&mut e, ""), hay.addr());
        assert_eq!(find(&mut e, "Data\\Textures\\PC\\xyz"), 0);
    }

    #[test]
    fn fn_004b7650_indexes_the_language_table() {
        let mut e = engine();
        e.mem.set_u32(LANGUAGE_NAMES + 8, 0x1234);
        assert_eq!(e.call(0x004b_7650, &args![2i32]).u32(), 0x1234);
    }

    /// A lighting property for the texture swap test: `sets` texture sets
    /// of 6 slots, `textures` in them, and the calls to its slot `0xf0`
    /// recorded.
    fn lighting_property(
        e: &mut Engine,
        base: u32,
        sets: u32,
        textures: Vec<(u32, u32, u32)>,
    ) -> (u32, Shared<Vec<Vec<u32>>>) {
        let calls: Shared<Vec<Vec<u32>>> = Rc::default();
        let table = base;
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, table);
        e.mem.set_u32(object + 0x1c, 9);
        e.register_double(base + 0x10, move |_, _| ret_u(sets));
        e.register_double(base + 0x20, move |_, a| {
            ret_u(
                textures
                    .iter()
                    .find(|(slot, set, _)| (*slot, *set) == (a[1], a[2]))
                    .map_or(0, |t| t.2),
            )
        });
        let log = calls.clone();
        e.register_double(base + 0x30, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let mut slots = vec![0u32; 62];
        slots[58] = base + 0x10;
        slots[61] = base + 0x20;
        slots[60] = base + 0x30;
        e.put_vtable(table, &slots);
        (object, calls)
    }

    /// A texture whose name (the virtual slot `0x9c` result) points at a
    /// word holding the text; `class` is the flag the class test reads.
    fn texture_named(e: &mut Engine, name: &str, class: bool) -> u32 {
        let table = 0x0300_6200;
        if !e.mem.is_mapped(table) {
            e.put_vtable(table, &[0u32; 40]);
            e.mem.set_u32(table + 0x9c, 0x0300_6210);
            e.register(0x0300_6210, |e, a| ret_u(e.mem.u32(a[0] + 8)));
        }
        let texture = e.mem.alloc(0x20);
        e.mem.set_u32(texture, table);
        let holder = e.mem.alloc(8);
        let t = text(e, name);
        e.mem.set_u32(holder, t.addr());
        e.mem.set_u32(texture + 8, holder);
        e.mem.set_u32(texture + 0x10, class as u32);
        texture
    }

    #[test]
    fn swap_platform_language_textures_replaces_the_textures_whose_paths_change() {
        let mut e = swap_engine();
        let g = graph(&mut e);
        e.mem.set_u8(PLATFORM_SWAP_ENABLED, 1);
        e.register(GET_PROPERTY, |e, a| ret_u(e.mem.u32(a[0] + 0x30)));
        e.register(WORD_AT_0X1C, |e, a| ret_u(e.mem.u32(a[0] + 0x1c)));
        e.register(IS_OF_CLASS, |e, a| {
            assert_eq!(a[0], IS_OF_CLASS_KEY_011F444C);
            ret_u(e.mem.u32(a[1] + 0x10))
        });
        e.register(NAME_TEXT, |e, a| ret_u(e.mem.u32(a[0])));
        e.register(FIXED_STRING_COPY, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            ret_u(a[0])
        });
        let destroyed: Shared<u32> = Rc::default();
        {
            let destroyed = destroyed.clone();
            e.register_double(FIXED_STRING_DESTRUCT, move |_, _| {
                *destroyed.borrow_mut() += 1;
                Ret::default()
            });
        }
        e.register(SMART_POINTER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret_u(a[0])
        });
        e.register(SMART_POINTER_GET, |e, a| ret_u(e.mem.u32(a[0])));
        e.register(SMART_POINTER_RELEASE, |_, _| Ret::default());
        let loaded: Shared<Vec<String>> = Rc::default();
        {
            let loaded = loaded.clone();
            e.map(TES_SINGLETON & !0xfff, 0x1000);
            e.set_global(TES_SINGLETON, 0x7e5u32);
            e.register_double(CREATE_TEXTURE_IMAGE, move |e, a| {
                assert_eq!(a[0], 0x7e5);
                loaded
                    .borrow_mut()
                    .push(String::from_utf8(e.mem.cstr(a[1])).unwrap());
                e.mem.set_u32(a[2], 0x7700 + loaded.borrow().len() as u32);
                Ret::default()
            });
        }
        // Slot 2 of set 0: a texture whose path changes; slot 3: not of the
        // class; slot 4: of the class but unchanged; set 1 slot 0: changes.
        let changes = texture_named(&mut e, "t/pc/a.dds", true);
        let not_texture = texture_named(&mut e, "t/pc/b.dds", false);
        let unchanged = texture_named(&mut e, "t/plain/c.dds", true);
        let second_set = texture_named(&mut e, "u/PC/d.dds", true);
        let (lighting, set_calls) = lighting_property(
            &mut e,
            0x0300_6100,
            2,
            vec![
                (2, 0, changes),
                (3, 0, not_texture),
                (4, 0, unchanged),
                (0, 1, second_set),
            ],
        );
        let (low_kind, _) = lighting_property(&mut e, 0x0300_6400, 1, vec![(0, 0, changes)]);
        e.mem.set_u32(low_kind + 0x1c, 7);
        let child = graph_node(&mut e, &g, &[]);
        e.mem.set_u32(child + 0x30, low_kind);
        let root = graph_node(&mut e, &g, &[child]);
        e.mem.set_u32(root + 0x30, lighting);
        assert!(e.call(0x004b_7660, &args![p(root)]).bool());
        assert_eq!(*loaded.borrow(), vec!["t/X360/a.dds", "u/X360/d.dds"]);
        assert_eq!(
            *set_calls.borrow(),
            vec![vec![lighting, 2, 0, 0x7701], vec![lighting, 0, 1, 0x7702]]
        );
        // Every texture of the class had its name copy destroyed.
        assert_eq!(*destroyed.borrow(), 3);
        // Nothing to swap: false.
        let root = graph_node(&mut e, &g, &[child]);
        assert!(!e.call(0x004b_7660, &args![p(root)]).bool());
        assert!(!e.call(0x004b_7660, &args![Ptr::<()>::NULL]).bool());
    }

    #[test]
    fn fixed_strings_init_sdm_makes_the_54_strings() {
        let mut e = engine();
        let made: Shared<Vec<(u32, u32)>> = Rc::default();
        e.register(OPERATOR_NEW, |e, a| {
            assert_eq!(a[0], 4);
            ret_u(e.mem.alloc(4))
        });
        {
            let made = made.clone();
            e.register_double(FIXED_STRING_CONSTRUCT, move |_, a| {
                made.borrow_mut().push((a[0], a[1]));
                ret_u(a[0])
            });
        }
        e.call(0x004b_7920, &[]);
        let made = made.borrow();
        assert_eq!(made.len(), 54);
        for ((global, string), (block, made_from)) in FIXED_STRINGS.iter().zip(made.iter()) {
            assert_eq!(e.global::<u32>(*global), *block);
            assert_eq!(made_from, string);
        }
        assert_eq!(made[0].1, 0x0101_e460);
        assert_eq!(made[53].1, 0x0102_0324);
        // A failing allocation leaves 0 in the global and constructs nothing.
        let mut e = engine();
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        e.register(FIXED_STRING_CONSTRUCT, |_, _| panic!("constructed"));
        e.set_global(FIXED_STRINGS[0].0, 5u32);
        e.call(0x004b_7920, &[]);
        assert_eq!(e.global::<u32>(FIXED_STRINGS[0].0), 0);
    }

    #[test]
    fn fn_004b8cf0_frees_the_fixed_strings_but_not_the_spine_ones() {
        let mut e = engine();
        let mut expected = vec![];
        for (i, (global, _)) in FIXED_STRINGS.iter().enumerate() {
            // Every third one is already 0.
            if i % 3 != 2 {
                e.set_global(*global, 0x4000 + 16 * i as u32);
                if *global != FIXED_STRING_SPINE && *global != FIXED_STRING_SPINE1 {
                    expected.push(args![0x4000 + 16 * i as u32]);
                }
            }
        }
        e.register(FIXED_STRING_DESTRUCT, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x004b_8cf0, &[]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, FIXED_STRING_DESTRUCT), expected);
        assert_eq!(calls_to(&log, OPERATOR_DELETE), expected);
        // Nothing is cleared.
        assert_eq!(e.global::<u32>(FIXED_STRINGS[0].0), 0x4000);
        assert_eq!(e.global::<u32>(FIXED_STRING_SPINE), 0x4000 + 16 * 4);
    }

    #[test]
    fn fn_004b9900_destroys_and_frees_on_bit_zero() {
        let mut e = engine();
        e.register(FIXED_STRING_DESTRUCT, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004b_9900, &args![0x2000u32, 0u32]).u32(), 0x2000);
        let log = take_log(&mut e);
        assert_eq!(
            calls_to(&log, FIXED_STRING_DESTRUCT),
            vec![args![0x2000u32]]
        );
        assert!(calls_to(&log, OPERATOR_DELETE).is_empty());
        e.call_log = Some(vec![]);
        e.call(0x004b_9900, &args![0x2000u32, 3u32]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![args![0x2000u32]]);
    }

    #[test]
    fn fn_004b9930_tests_the_byte_at_four() {
        let mut e = engine();
        let object = e.mem.alloc(8);
        assert!(e.call(0x004b_9930, &args![p(object)]).bool());
        e.mem.set_u8(object + 4, 1);
        assert!(!e.call(0x004b_9930, &args![p(object)]).bool());
    }

    #[test]
    fn fn_004b99b0_builds_the_empty_bucket_array() {
        let mut e = engine();
        e.map(0x0102_0000, 0x1000);
        e.register(NI_ARRAY_NEW, |e, a| {
            let block = e.mem.alloc(a[0]);
            e.mem.write(block, &vec![0xff; a[0] as usize]);
            ret_u(block)
        });
        let map = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004b_99b0, &args![p(map), 5u32]).u32(), map);
        let log = take_log(&mut e);
        let table = e.mem.u32(map + 8);
        assert_eq!(
            (e.mem.u32(map), e.mem.u32(map + 4), e.mem.u32(map + 0xc)),
            (MAP_BASE_VTABLE, 5, 0)
        );
        assert_eq!(calls_to(&log, NI_ARRAY_NEW), vec![args![20u32]]);
        assert_eq!(calls_to(&log, MEMORY_SET), vec![args![table, 0u32, 20u32]]);
        assert_eq!(e.mem.bytes(table, 20), vec![0; 20]);
    }

    #[test]
    fn fn_004b9950_installs_the_map_vtable_after_the_base_constructor() {
        let mut e = engine();
        e.map(0x0102_0000, 0x1000);
        e.register(NI_ARRAY_NEW, |e, a| ret_u(e.mem.alloc(a[0])));
        let map = e.mem.alloc(0x10);
        assert_eq!(e.call(0x004b_9950, &args![p(map), 3u32]).u32(), map);
        assert_eq!(
            (e.mem.u32(map), e.mem.u32(map + 4)),
            (VERTEX_DIST_MAP_VTABLE, 3)
        );
    }

    /// A hash map of 4 buckets with the virtual slots a test needs: the
    /// hash (slot 4) is `key % 4`, the key comparison (slot 8) equality,
    /// the item initialiser (slot `0xc`) the real `SetValue`, and the
    /// allocation (slot `0x14`) a 0x14 byte block.
    fn map_engine() -> (Engine, u32) {
        let mut e = engine();
        e.register(0x0300_6300, |_, a| ret_u(a[1] % 4));
        e.register(0x0300_6310, |_, a| ret_u((a[1] == a[2]) as u32));
        e.register(0x0300_6320, |e, _| ret_u(e.mem.alloc(0x14)));
        e.put_vtable(
            0x0300_6340,
            &[0, 0x0300_6300, 0x0300_6310, 0x004b_9ca0, 0, 0x0300_6320],
        );
        let map = e.mem.alloc(0x10);
        let table = e.mem.alloc(16);
        e.mem.set_u32(map, 0x0300_6340);
        e.mem.set_u32(map + 4, 4);
        e.mem.set_u32(map + 8, table);
        (e, map)
    }

    fn set_at(e: &mut Engine, map: u32, key: u32, words: [u32; 3]) {
        e.call(
            0x004b_9a20,
            &args![p(map), key, words[0], words[1], words[2]],
        );
    }

    #[test]
    fn fn_004b9a20_inserts_new_keys_and_updates_existing_ones() {
        let (mut e, map) = map_engine();
        set_at(&mut e, map, 5, [1, 2, 3]);
        assert_eq!(e.mem.u32(map + 0xc), 1);
        let table = e.mem.u32(map + 8);
        let first = e.mem.u32(table + 4);
        assert_eq!(e.mem.bytes(first, 0x14)[4..].to_vec(), {
            let mut v = vec![];
            for w in [5u32, 1, 2, 3] {
                v.extend_from_slice(&w.to_le_bytes());
            }
            v
        });
        assert_eq!(e.mem.u32(first), 0);
        // The same key: updated in place.
        set_at(&mut e, map, 5, [7, 8, 9]);
        assert_eq!(e.mem.u32(map + 0xc), 1);
        assert_eq!([8, 0xc, 0x10].map(|o| e.mem.u32(first + o)), [7, 8, 9]);
        // Another key in the same bucket goes first and links to the old one.
        set_at(&mut e, map, 9, [4, 5, 6]);
        assert_eq!(e.mem.u32(map + 0xc), 2);
        let second = e.mem.u32(table + 4);
        assert_ne!(second, first);
        assert_eq!((e.mem.u32(second), e.mem.u32(second + 4)), (first, 9));
        // And updating the older key still finds it behind the new one.
        set_at(&mut e, map, 5, [1, 1, 1]);
        assert_eq!(e.mem.u32(first + 8), 1);
        assert_eq!(e.mem.u32(map + 0xc), 2);
    }

    #[test]
    fn fn_004b9b20_copies_the_value_words_of_the_key() {
        let (mut e, map) = map_engine();
        set_at(&mut e, map, 5, [1, 2, 3]);
        set_at(&mut e, map, 9, [4, 5, 6]);
        let out = e.mem.alloc(16);
        e.mem.write(out, &[0xee; 16]);
        assert!(e.call(0x004b_9b20, &args![p(map), 5u32, p(out)]).bool());
        assert_eq!([0, 4, 8].map(|o| e.mem.u32(out + o)), [1, 2, 3]);
        assert!(e.call(0x004b_9b20, &args![p(map), 9u32, p(out)]).bool());
        assert_eq!([0, 4, 8].map(|o| e.mem.u32(out + o)), [4, 5, 6]);
        // A key in an occupied bucket that is not there, and an empty bucket.
        e.mem.set_u32(out, 0x1111);
        assert!(!e.call(0x004b_9b20, &args![p(map), 13u32, p(out)]).bool());
        assert!(!e.call(0x004b_9b20, &args![p(map), 2u32, p(out)]).bool());
        assert_eq!(e.mem.u32(out), 0x1111);
    }

    #[test]
    fn fn_004b9ba0_returns_the_head_of_the_first_used_bucket() {
        let (mut e, map) = map_engine();
        assert_eq!(e.call(0x004b_9ba0, &args![p(map)]).u32(), 0);
        set_at(&mut e, map, 6, [0, 0, 0]);
        let table = e.mem.u32(map + 8);
        assert_eq!(
            e.call(0x004b_9ba0, &args![p(map)]).u32(),
            e.mem.u32(table + 8)
        );
        set_at(&mut e, map, 3, [0, 0, 0]);
        assert_eq!(
            e.call(0x004b_9ba0, &args![p(map)]).u32(),
            e.mem.u32(table + 8)
        );
        set_at(&mut e, map, 1, [0, 0, 0]);
        assert_eq!(
            e.call(0x004b_9ba0, &args![p(map)]).u32(),
            e.mem.u32(table + 4)
        );
    }

    #[test]
    fn fn_004b9bf0_walks_every_item_once() {
        let (mut e, map) = map_engine();
        for (key, value) in [(1u32, 10u32), (5, 50), (2, 20), (7, 70)] {
            set_at(&mut e, map, key, [value, value + 1, value + 2]);
        }
        let iterator = e.mem.alloc(8);
        let key = e.mem.alloc(8);
        let value = e.mem.alloc(16);
        let first = e.call(0x004b_9ba0, &args![p(map)]).u32();
        e.mem.set_u32(iterator, first);
        let mut seen = vec![];
        while e.mem.u32(iterator) != 0 {
            e.call(0x004b_9bf0, &args![p(map), p(iterator), p(key), p(value)]);
            seen.push((e.mem.u32(key), [0, 4, 8].map(|o| e.mem.u32(value + o))));
        }
        // Bucket 1 holds 5 then 1 (newest first), bucket 2 holds 2, bucket 3
        // holds 7.
        assert_eq!(
            seen,
            vec![
                (5, [50, 51, 52]),
                (1, [10, 11, 12]),
                (2, [20, 21, 22]),
                (7, [70, 71, 72])
            ]
        );
    }

    #[test]
    fn ni_t_map_base_vertex_dist_set_value_fills_the_item() {
        let mut e = engine();
        let item = e.mem.alloc(0x14);
        e.mem.set_u32(item, 0x99);
        e.call(
            0x004b_9ca0,
            &args![0x1234u32, p(item), 7u32, 1u32, 2u32, 3u32],
        );
        assert_eq!(
            [0, 4, 8, 0xc, 0x10].map(|o| e.mem.u32(item + o)),
            [0x99, 7, 1, 2, 3]
        );
    }

    #[test]
    fn fn_004b9d30_and_fn_004b9cd0_empty_the_map_and_free_the_table() {
        let mut e = engine();
        e.map(0x0102_0000, 0x1000);
        e.register(MAP_REMOVE_ALL, |_, _| Ret::default());
        e.register(NI_ARRAY_DELETE, |_, _| Ret::default());
        let map = e.mem.alloc(0x10);
        e.mem.set_u32(map + 8, 0x5550);
        e.call_log = Some(vec![]);
        e.call(0x004b_9d30, &args![p(map)]);
        let log = take_log(&mut e);
        assert_eq!(e.mem.u32(map), MAP_BASE_VTABLE);
        assert_eq!(
            log.iter().map(|(a, _)| *a).collect::<Vec<_>>(),
            vec![0x004b_9d30, MAP_REMOVE_ALL, NI_ARRAY_DELETE]
        );
        assert_eq!(calls_to(&log, NI_ARRAY_DELETE), vec![args![0x5550u32]]);
        // The map's own destructor empties it with its own vtable set, then
        // runs the base one.
        let seen_vtable: Shared<Vec<u32>> = Rc::default();
        {
            let seen_vtable = seen_vtable.clone();
            e.register_double(MAP_REMOVE_ALL, move |e, a| {
                seen_vtable.borrow_mut().push(e.mem.u32(a[0]));
                Ret::default()
            });
        }
        e.call(0x004b_9cd0, &args![p(map)]);
        assert_eq!(
            *seen_vtable.borrow(),
            vec![VERTEX_DIST_MAP_VTABLE, MAP_BASE_VTABLE]
        );
        assert_eq!(e.mem.u32(map), MAP_BASE_VTABLE);
    }

    #[test]
    fn ni_t_map_vertex_dist_scalar_deleting_destructor_frees_on_bit_zero() {
        let mut e = engine();
        e.map(0x0102_0000, 0x1000);
        e.register(MAP_REMOVE_ALL, |_, _| Ret::default());
        e.register(NI_ARRAY_DELETE, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        let map = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004b_9980, &args![p(map), 0u32]).u32(), map);
        assert!(calls_to(&take_log(&mut e), OPERATOR_DELETE).is_empty());
        e.call_log = Some(vec![]);
        e.call(0x004b_9980, &args![p(map), 1u32]);
        assert_eq!(
            calls_to(&take_log(&mut e), OPERATOR_DELETE),
            vec![args![map]]
        );
    }

    #[test]
    fn ni_t_map_vertex_dist_new_item_allocates_through_the_allocator_member() {
        let mut e = engine();
        e.register(OPERATOR_NEW, |_, a| ret_u(a[0] + 0x100));
        assert_eq!(e.call(0x004b_9d60, &args![0x3000u32]).u32(), 0x14 + 0x100);
    }

    #[test]
    fn fn_004b9de0_allocates_0x14_bytes() {
        let mut e = engine();
        e.register(OPERATOR_NEW, |_, a| ret_u(a[0] + 0x100));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004b_9de0, &args![0x3000u32]).u32(), 0x114);
        assert_eq!(
            calls_to(&take_log(&mut e), OPERATOR_NEW),
            vec![args![0x14u32]]
        );
    }

    #[test]
    fn fn_004b9d80_builds_the_allocator_then_frees_the_item() {
        let mut e = engine();
        e.register(ALLOCATOR_EMPTY_CONSTRUCTOR, |_, a| ret_u(a[0]));
        e.register(ALLOCATOR_FREE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x004b_9d80, &args![0x3000u32, 0x5000u32]);
        let log = take_log(&mut e);
        assert_eq!(
            calls_to(&log, ALLOCATOR_EMPTY_CONSTRUCTOR),
            vec![args![0x5008u32, 0u32]]
        );
        assert_eq!(
            calls_to(&log, ALLOCATOR_FREE),
            vec![args![0x300cu32, 0x5000u32]]
        );
    }

    #[test]
    fn ni_t_map_base_vertex_dist_scalar_deleting_destructor_frees_on_bit_zero() {
        let mut e = engine();
        e.map(0x0102_0000, 0x1000);
        e.register(MAP_REMOVE_ALL, |_, _| Ret::default());
        e.register(NI_ARRAY_DELETE, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        let map = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004b_9db0, &args![p(map), 0u32]).u32(), map);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, MAP_REMOVE_ALL).len(), 1);
        assert!(calls_to(&log, OPERATOR_DELETE).is_empty());
        e.call_log = Some(vec![]);
        e.call(0x004b_9db0, &args![p(map), 1u32]);
        assert_eq!(
            calls_to(&take_log(&mut e), OPERATOR_DELETE),
            vec![args![map]]
        );
    }
}
