//! `BSHavok/bhkcharactercontroller.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkCharacterController` (Xbox PDB, 0x650 bytes) is the Havok character controller of
//! the player and the NPCs: a `bhkCharacterProxy` (+0x000) followed by the
//! `hkpCharacterContext` (+0x3E0), the `bhkCharacterListener` (+0x410, the
//! `hkpCharacterProxyListener` whose callbacks receive `this + 0x410`) and the
//! controller's own state. The PC offsets used here agree with the Xbox PDB.
//!
//! Floats: x87 code is computed in `f64` and rounded to `f32` at each `float` store;
//! `FLD`/`FSTP` copies go through [`x87_quiet`]; SSE code (`MOVSS`, `MULPS`, `SQRTSS`,
//! `RSQRTSS`) computes in `f32`. Comparisons follow the exe's flag tests: `TEST AH,5; JP`
//! after `FCOM a, b` is `a < b`, `TEST AH,0x41; JNZ` skips unless `a > b`, and
//! `TEST AH,0x44; JNP` skips only on `==` (NaN passes).
//!
//! The exception-unwinding frames (`FS:[0]`) and the `/GS` cookie checks (`00ec408c`)
//! of the functions marked `Translation incomplete` are not modeled.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::x87::rsqrt_estimate;

/// Offset of the `bhkCharacterListener` base in the controller: the listener callbacks
/// get `controller + 0x410` as `this`.
const LISTENER: u32 = 0x410;
/// Per-thread word the Havok-allocating functions set to 0x10 (`TLS + 0x2b4`).
const SCOPE_WORD: u32 = 0x2b4;

/// `hkMemoryRouter` lookup (`00c85750`): its `+0x10` is the allocator whose slot `+4`
/// allocates `(bytes)` and slot `+8` frees `(pointer, bytes)`.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkReferencedObject` functions of `hkreferencedobject.cpp`: `00c90510` takes a
/// reference, `00c905b0` releases one (`this` only).
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// Sized delete of the game heap (`00aa1460`, cdecl `(pointer, size)`).
const FREE_SIZED: u32 = 0x00aa_1460;
/// The game heap's allocator (`00aa13e0`, cdecl `(size)`).
const HEAP_ALLOCATE: u32 = 0x00aa_13e0;
/// `MemoryManager` deallocation (`00aa4060`, `this` = the manager at `011f6238`).
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// `kernel32` `InterlockedDecrement` import slot.
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The empty fixed-string handle the string releases compare against.
const EMPTY_FIXED_STRING: u32 = 0x0109_b220;

/// `bhkCharacterProxy::UnMarkRead` (Xbox PDB, `00c89720`) and `UnMarkWrite`
/// (`00cac110`): thunks through the object's vtable slots 0x94 and 0x98.
const UN_MARK_READ: u32 = 0x00c8_9720;
const UN_MARK_WRITE: u32 = 0x00ca_c110;
/// `hkpCharacterProxy`'s `m_shapePhantom` getter (`00cac980`, `MOV EAX,[ECX+0x30]`).
const GET_SHAPE_PHANTOM: u32 = 0x00ca_c980;
/// `hkpCharacterProxy::getPosition` (Xbox PDB, `00cac910`) and `getLinearVelocity`
/// (`00cac950`).
const HKP_GET_POSITION: u32 = 0x00ca_c910;
const HKP_GET_LINEAR_VELOCITY: u32 = 0x00ca_c950;
/// `hkpCharacterProxy::integrateWithCollectors` (Xbox PDB, `00cafbd0`).
const INTEGRATE_WITH_COLLECTORS: u32 = 0x00ca_fbd0;
/// `bhkCharacterPointCollector` constructor, `handleCollisions` and destructor (Xbox PDB).
const POINT_COLLECTOR_CONSTRUCT: u32 = 0x00cd_3410;
const POINT_COLLECTOR_HANDLE_COLLISIONS: u32 = 0x00cd_32b0;
const POINT_COLLECTOR_DESTRUCT: u32 = 0x00cd_3490;
/// `bhkCharacterProxy::GethkPosition` (Xbox PDB, `00621a40`) and `GethkLinearVelocity`
/// (`00807080`): return a pointer to a vector.
const GET_HK_POSITION: u32 = 0x0062_1a40;
const GET_HK_LINEAR_VELOCITY: u32 = 0x0080_7080;
/// `bhkCharacterProxy` destructor body (`00cabf50`).
const CHARACTER_PROXY_DESTRUCT: u32 = 0x00ca_bf50;
/// Called on the proxy with the new cinfo (`00cabfd0`, one stack argument).
const SET_PROXY_CINFO: u32 = 0x00ca_bfd0;
/// `hkpWorld::removePhantom` / `addPhantom` (Xbox PDB).
const WORLD_REMOVE_PHANTOM: u32 = 0x00c9_2c70;
const WORLD_ADD_PHANTOM: u32 = 0x00c9_36a0;
/// `hkpShapePhantom::setTransform` (Xbox PDB).
const PHANTOM_SET_TRANSFORM: u32 = 0x00c9_e910;
/// `hkpWorldObject::addProperty` (Xbox PDB, `(key, value low, value high)`).
const ADD_PROPERTY: u32 = 0x00cd_2be0;
/// `hkpMotion::getMass` (Xbox PDB), result in ST0.
const GET_MASS: u32 = 0x00c8_6430;
/// `bhkRigidBody`-unit test on a motion-type byte (`00c8cce0`, cdecl, pointer to it).
const MOTION_TYPE_TEST: u32 = 0x00c8_cce0;
/// `hkVector4::setTransformedPos`, `setTransformedInversePos`, `setRotatedDir`
/// (Xbox PDB): `this` = the output vector, then the transform and the input vector.
const SET_TRANSFORMED_POS: u32 = 0x00c7_f640;
const SET_TRANSFORMED_INVERSE_POS: u32 = 0x00c7_f690;
const SET_ROTATED_DIR: u32 = 0x00c7_f700;
/// `hkRotation` set from a quaternion (`00cb2d90`, `hkrotation.cpp`) and
/// `hkMatrix3::mul` (Xbox PDB, `00cd5680`).
const ROTATION_SET: u32 = 0x00cb_2d90;
const MATRIX3_MUL: u32 = 0x00cd_5680;
/// Called on the quaternion copy before the rotation is built (`005611c0`).
const QUATERNION_PREPARE: u32 = 0x0056_11c0;
/// `bhkWorldObject::SetAVObject` (Xbox PDB), `GetAVObject`, and the
/// `bhkworldobject.obj` function `00c85ce0` (one stack argument).
const SET_AV_OBJECT: u32 = 0x00c8_5c10;
const GET_AV_OBJECT: u32 = 0x00c8_5c80;
const WORLD_OBJECT_PASS: u32 = 0x00c8_5ce0;
/// Lock call that the PC build compiled to a bare `RET` (`00a29680`).
const EMPTY_LOCK: u32 = 0x00a2_9680;
/// `bhkCollisionBox::Update` (Xbox PDB), `::Init`, and the destructor.
const COLLISION_BOX_UPDATE: u32 = 0x00cb_e8d0;
const COLLISION_BOX_INIT: u32 = 0x00cb_e9f0;
const COLLISION_BOX_DESTRUCT: u32 = 0x00cb_e880;
/// `bhkCachingShapePhantom::Init` and destructor (Xbox PDB).
const CACHING_PHANTOM_INIT: u32 = 0x00cc_2530;
const CACHING_PHANTOM_DESTRUCT: u32 = 0x00cc_2350;
/// `bhkListShape::Init` and destructor (Xbox PDB).
const LIST_SHAPE_INIT: u32 = 0x00c8_b090;
const LIST_SHAPE_DESTRUCT: u32 = 0x00c8_adf0;
/// `bhkShapeCollection` destructor (Xbox PDB).
const SHAPE_COLLECTION_DESTRUCT: u32 = 0x00cd_2de0;
/// Base constructor every `bhkRefObject`-derived constructor here starts with
/// (`00c87c30`: vtable `010c5b54`, clears `+8`).
const REF_OBJECT_CONSTRUCT: u32 = 0x00c8_7c30;
/// `bhkshape.obj` function `00c84f10` (one stack argument, returns the layer number).
const SHAPE_LAYER: u32 = 0x00c8_4f10;
/// `NiObject` cast (`00653290`, `(object, rtti)`: the object when it is of that
/// class, else 0).
const IS_KIND_OF: u32 = 0x0065_3290;
/// `NiNode::ms_RTTI` (`NiNode::GetRTTI` returns it) and `bhkCapsuleShape::ms_RTTI`.
const NODE_RTTI: u32 = 0x011f_4428;
const CAPSULE_SHAPE_RTTI: u32 = 0x0126_832c;
/// `NiFixedString` creation (`00a5b690`, cdecl), `NiObjectNET::SetName` (`00a5b950`).
const FIXED_STRING_CREATE: u32 = 0x00a5_b690;
const SET_NAME: u32 = 0x00a5_b950;
/// `"bhkColDisp"`.
const DISPLAY_NAME: u32 = 0x010c_43d8;
/// `NiNode` constructor (`00a5ecb0`, one stack argument) and the node's size.
const NODE_CONSTRUCT: u32 = 0x00a5_ecb0;
const NODE_SIZE: u32 = 0xac;
/// Child getter of the `tes.cpp` unit (`0045bc00`, one stack argument).
const NODE_CHILD_AT: u32 = 0x0045_bc00;
/// Point copy of `tes.cpp` (`00458620`, cdecl `(destination, source)`, returns the
/// destination).
const COPY_POINT: u32 = 0x0045_8620;
/// Filter-info builder for a ray of this controller (`0070c440`, writes a word through
/// its one stack argument).
const RAY_FILTER_INFO: u32 = 0x0070_c440;
/// `hkpCharacterContext` state getter (`00ca9600`, `MOV EAX,[ECX+0x10]`).
const GET_STATE: u32 = 0x00ca_9600;
/// CRT x87 helpers taking ST0: `00ec6040` (square root), `00ec9f70` and `00eca0a0`.
const X87_SQRT: u32 = 0x00ec_6040;
const X87_FIRST_TRIG: u32 = 0x00ec_9f70;
const X87_SECOND_TRIG: u32 = 0x00ec_a0a0;
/// cdecl float helpers: `0040ebd0(a, b)` returns `a` when `b > a`, else `b`;
/// `00404010(a, b)` returns `a` when `b < a`, else `b`.
const FLOAT_MIN: u32 = 0x0040_ebd0;
const FLOAT_MAX: u32 = 0x0040_4010;

/// The 16-byte vector at `01267e30` (zero in the exe).
const ZERO_VECTOR: u32 = 0x0126_7e30;
/// `float` constants: `0.0`, `1.0`, `3.0`, `0.5`, `-0.0`.
const FLOAT_ZERO: u32 = 0x0101_1d78;
const FLOAT_ONE: u32 = 0x0101_2068;
const FLOAT_THREE: u32 = 0x0101_7718;
const FLOAT_HALF: u32 = 0x0101_6248;
const FLOAT_NEGATIVE_ZERO: u32 = 0x010c_4990;
/// `double` constants: `0.5`, `-1.0`, `0.05` (as a float), `25.0`, `1000.0`, `0.9` (as a
/// float), `0.25`, `32.0`, `20.0`, `-0.25`.
const HALF: u32 = 0x0101_1588;
const MINUS_ONE: u32 = 0x0101_a6b0;
const ROT_MOD_DECAY: u32 = 0x0103_c7c8;
const PROBE_DEPTH: u32 = 0x0104_f2f0;
const RATIO_LIMIT: u32 = 0x0101_7b70;
const HEIGHT_FRACTION: u32 = 0x0106_b9e8;
const QUARTER: u32 = 0x0102_90b0;
const THIRTY_TWO: u32 = 0x0102_f070;
const LARGE_SIZE_BONUS: u32 = 0x0102_fc70;
const PUSH_SCALE: u32 = 0x0109_c9e0;
/// `float` `0x7f7fffee` the collector reset stores as its distances.
const COLLECTOR_FAR: u32 = 0x0104_da48;
/// `float` default radius when the capsule has no Havok shape (0.1).
const DEFAULT_RADIUS: u32 = 0x010c_72bc;
/// `float` impulse factor of the object callback (2.0).
const IMPULSE_DEFAULT: u32 = 0x0101_62c0;
/// `float` 0.1 (an impulse factor of the object callback, a cinfo default).
const FLOAT_TENTH: u32 = 0x0101_e2bc;
/// The mass limit of the object callback (`float`, 95.0 in the exe).
const MAX_PUSH_MASS: u32 = 0x011b_0128;
/// Mass a body must exceed for `00c6def0` (`float` global).
const BLOCKING_MASS: u32 = 0x0126_7bc0;
/// `float` global: `1 / (float at 011b0194)`, set by the static initializer `00fbccd0`.
/// The functions here multiply game positions and sizes by it.
const INVERSE_SCALE: u32 = 0x0126_7bcc;
/// The `float` global `INVERSE_SCALE` is the reciprocal of.
const SCALE: u32 = 0x011b_0194;
/// `float` velocity-modifier time of the character callback (0.01) and its rotation
/// time (0.2).
const MODIFIER_TIME: u32 = 0x0101_3ea4;
const ROT_MOD_TIME: u32 = 0x0101_df44;
/// 16-byte vector whose first lane the character callback subtracts from (0.0).
const DOT_BASE: u32 = 0x010c_7160;
/// The vector `00c6d9f0` copies three times.
const POINT_DEFAULT: u32 = 0x011f_426c;
/// Second argument of the shape's slot 0xD4 in `BuildDisplayGeometry`.
const DISPLAY_ARGUMENT: u32 = 0x011a_9b7c;
/// The layer-filter callback pointer `00c6d7e0` calls (cdecl, one argument).
const LAYER_CALLBACK: u32 = 0x0126_7bc8;
/// Globals released by the two static tear-down functions.
const STATIC_REFERENCED_OBJECT: u32 = 0x0126_7bb8;
const STATIC_NI_POINTER: u32 = 0x0126_7be0;

/// `NiRTTI` records the `GetRTTI` overrides return.
const SHAPE_COLLECTION_RTTI: u32 = 0x0126_86d4;
const CACHING_PHANTOM_RTTI: u32 = 0x0126_85a4;
const COLLISION_BOX_RTTI: u32 = 0x0126_8444;
const LIST_SHAPE_RTTI: u32 = 0x0126_81a8;

/// Vtables (class names from their RTTI type descriptors).
const VTABLE_WORLD_OBJECT: u32 = 0x0103_0394;
const VTABLE_PHANTOM: u32 = 0x0103_078c;
const VTABLE_SHAPE_PHANTOM: u32 = 0x0103_06b4;
const VTABLE_SIMPLE_SHAPE_PHANTOM: u32 = 0x0103_05dc;
const VTABLE_CACHING_SHAPE_PHANTOM: u32 = 0x010c_4694;
const VTABLE_COLLISION_BOX: u32 = 0x010c_477c;
const VTABLE_SHAPE: u32 = 0x0103_0bf4;
const VTABLE_SHAPE_COLLECTION: u32 = 0x010c_44e4;
const VTABLE_LIST_SHAPE: u32 = 0x010c_485c;
const VTABLE_CLOSEST_RAY_HIT_COLLECTOR: u32 = 0x0103_4ec4;
const VTABLE_CD_POINT_COLLECTOR: u32 = 0x0104_da3c;

/// Counter words the inlined base constructors increment (one per class).
const COUNT_WORLD_OBJECT: u32 = 0x0126_810c;
const COUNT_PHANTOM: u32 = 0x0126_8238;
const COUNT_SHAPE_PHANTOM: u32 = 0x0126_8208;
const COUNT_SIMPLE_SHAPE_PHANTOM: u32 = 0x0126_822c;
const COUNT_CACHING_SHAPE_PHANTOM: u32 = 0x0126_85a0;
const COUNT_SHAPE: u32 = 0x0126_80f8;
const COUNT_SHAPE_COLLECTION: u32 = 0x0126_86d0;
const COUNT_LIST_SHAPE: u32 = 0x0126_81a0;

/// Property key `00c6d900` looks for.
const PROPERTY_KEY: u32 = 0x1300;

/// `hkpCharacterProxy::checkSupport` (Xbox PDB), `hkpCharacterProxy::setPosition` (Xbox
/// PDB), and the `hkpcharacterproxy.cpp` function `00cad6f0` (one stack argument, the
/// listener) the destructor calls; `addCharacterProxyListener` (Xbox PDB).
const HKP_CHECK_SUPPORT: u32 = 0x00ca_fd00;
const HKP_SET_POSITION: u32 = 0x00ca_c920;
const PROXY_LISTENER_REMOVE: u32 = 0x00ca_d6f0;
const ADD_PROXY_LISTENER: u32 = 0x00ca_da30;
/// `bhkCharacterProxy::Init` (Xbox PDB, one stack argument).
const CHARACTER_PROXY_INIT: u32 = 0x00ca_c300;
/// `bhkcharacterproxy.obj` function `00cac680` (`this` only) the controller calls when it
/// leaves its world.
const PROXY_LEAVE_WORLD: u32 = 0x00ca_c680;
/// Returns its `this + 0x74` (`00cac900`).
const PROXY_PART_74: u32 = 0x00ca_c900;
/// `hkpCharacterContext` destructor body (`00ca95e0`, `this` only).
const CONTEXT_DESTRUCT: u32 = 0x00ca_95e0;
/// CRT `eh vector destructor iterator` (`00ec5fce`, stdcall `(array, size, count,
/// destructor)`) and the `NiPointer` destructor the controller hands it (`00b66f20`).
const EH_VECTOR_DESTRUCTOR: u32 = 0x00ec_5fce;
const NI_POINTER_DESTRUCT: u32 = 0x00b6_6f20;
/// List destructor of the `bsfadenode.cpp` unit (`00b4df30`, `this` only), run on the
/// `DamageImpacts` list and on its removed nodes.
const LIST_DESTRUCT: u32 = 0x00b4_df30;
/// `hkColor::rgbFromFloats` (Xbox PDB, cdecl, four floats) and
/// `bhkWorld::SetDebugObjectColor` (Xbox PDB, `(object, color)`).
const RGB_FROM_FLOATS: u32 = 0x00ca_bb30;
const SET_DEBUG_OBJECT_COLOR: u32 = 0x00c6_8e30;
/// `hkArrayUtil::_reserve` (Xbox PDB) and the `hkarrayutil.cpp` function that makes room
/// for one more element (`00c90940`), both cdecl `(allocator, array, ...)`.
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// Builds a shape from an `hkArray` of vertices (`00cd7f50`, cdecl, one argument).
const SHAPE_FROM_VERTICES: u32 = 0x00cd_7f50;
/// Called on the phantom with `(from, cast input, collector, 0)` (`00c9eba0`; the map's
/// name for it, an `hkpAllCdPointCollector` constructor, does not fit the call).
const PHANTOM_LINEAR_CAST: u32 = 0x00c9_eba0;
/// `hkpAllCdPointCollector::sortHits` and destructor (Xbox PDB).
const ALL_COLLECTOR_SORT_HITS: u32 = 0x00ca_bad0;
const ALL_COLLECTOR_DESTRUCT: u32 = 0x0062_11b0;
/// Constructs the stack cinfo `Init` gives the `bhkCachingShapePhantom` constructor
/// (`0056e0b0`, `tesobjectrefr.cpp` unit, `this` only).
const PHANTOM_CINFO_CONSTRUCT: u32 = 0x0056_e0b0;
/// Writes into `this` a vector computed from the rotation rows at its first argument and
/// the vector at its second (`004b4ab0`, forwarding to `004b4ad0`).
const TRANSFORM_VECTOR: u32 = 0x004b_4ab0;
/// `bhkutilfunctions.obj` function `00c7ffd0` (cdecl, the body): an object whose slot 0xE0
/// gives the material for a key.
const BODY_MATERIAL_SOURCE: u32 = 0x00c7_ffd0;
/// `kernel32` `InterlockedIncrement` import slot.
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
/// CRT x87 helper `00ec9050` (ST0 in, ST0 out) applied to the support slope.
const X87_SLOPE_HELPER: u32 = 0x00ec_9050;

/// `float` 1.1754944e-38 (`0x00800000`): a `SupportHeight` lane equal to it is probed.
const SUPPORT_UNSET: u32 = 0x0102_efc4;
/// `float` global, the largest pitch or roll in degrees (47.0 in the exe image).
const MAX_TILT_DEGREES: u32 = 0x011b_0148;
/// `double` degrees-to-radians factor and its negation.
const DEGREES_TO_RADIANS: u32 = 0x0102_3128;
const NEGATIVE_DEGREES_TO_RADIANS: u32 = 0x0106_4f68;
/// `float` 100.0, `double` 5.0, `double` 0.01 and `double` 0.1.
const FLOAT_HUNDRED: u32 = 0x0101_6410;
const FIVE: u32 = 0x0102_0998;
const HUNDREDTH: u32 = 0x0101_6408;
const TENTH: u32 = 0x0101_ffa0;
/// `float` 0.75, the alpha of the phantom's debug colour.
const DEBUG_ALPHA: u32 = 0x010c_44ac;
/// `float` 0.05, 10.0 and pi/2: `hkpCharacterProxyCinfo` defaults.
const FLOAT_TWENTIETH: u32 = 0x0101_81bc;
const FLOAT_TEN: u32 = 0x0101_7b78;
const FLOAT_HALF_PI: u32 = 0x0101_ff38;
/// `float` start-point tolerance of the projectile cast (written twice).
const CAST_TOLERANCE: u32 = 0x0104_da4c;
/// The 16 bytes `00c6d6b0` measures from when the reference has no vector at `+0x20`.
const POINT_FALLBACK: u32 = 0x011f_4288;
/// Eight `float` x and y factors of the vertex rings `00c70de0` builds.
const RING_X: u32 = 0x011b_0154;
const RING_Y: u32 = 0x011b_0174;

/// Vtables of the controller's three bases (RTTI: `bhkCharacterController`), the
/// `hkpCharacterProxyListener` vtable the destructor restores, `bhkCharacterProxy`'s and
/// `hkpAllCdPointCollector`'s.
const VTABLE_CONTROLLER: u32 = 0x010c_49c4;
const VTABLE_CONTROLLER_CONTEXT: u32 = 0x010c_49b4;
const VTABLE_CONTROLLER_LISTENER: u32 = 0x010c_4998;
const VTABLE_PROXY_LISTENER: u32 = 0x010c_446c;
const VTABLE_CHARACTER_PROXY: u32 = 0x010c_45cc;
const VTABLE_ALL_CD_POINT_COLLECTOR: u32 = 0x0104_da28;
/// Counter word `00c6ce80` increments.
const COUNT_CHARACTER_PROXY: u32 = 0x0126_8340;

/// `bhkcapsuleshape.obj` function `00cab710` (`this` = a 0x14-byte block, then two vertex
/// pointers and a `float` radius): makes the capsule shapes.
const CAPSULE_CONSTRUCT: u32 = 0x00ca_b710;
/// `bsbound.obj` function `00c55170` (cdecl, the object at `info + 0x80`): its bound
/// record, or null.
const BOUND_OF: u32 = 0x00c5_5170;
/// `NiPointer` assignment (`0066b0d0`, `this` = the pointer, one stack argument) and
/// copy from another `NiPointer` (`00c663d0`, `bhkworld.obj`).
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
const NI_POINTER_COPY: u32 = 0x00c6_63d0;
/// `hkpSphereShape::hkpSphereShape` (Xbox PDB, one `float` argument).
const SPHERE_CONSTRUCT: u32 = 0x00cd_8160;
/// `MemoryManager` allocation (`00aa3e40`, `this` = the manager at `011f6238`).
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
/// `double` constants of `00c72410`: 23.0, 17.5, 64.0, 2.0, 0.75, 0.0, 8.0 and 1.0.
const DEFAULT_WIDTH: u32 = 0x0103_5838;
const DEFAULT_DEPTH: u32 = 0x010c_4ab8;
const DEFAULT_HEIGHT: u32 = 0x0102_40c0;
const TWO: u32 = 0x0101_1590;
const THREE_QUARTERS: u32 = 0x0101_de30;
const DOUBLE_ZERO: u32 = 0x0101_2060;
const EIGHT: u32 = 0x0102_40b0;
const DOUBLE_ONE: u32 = 0x0101_2070;

/// `double` 31.0.
const THIRTY_ONE: u32 = 0x0103_5828;
/// `double` 3.0.
const DOUBLE_THREE: u32 = 0x0102_1928;

/// `TlsGetValue` import slot and the TLS index of the profiling record stream.
const TLS_GET_VALUE: u32 = 0x00fd_f044;
const PROFILE_SLOT: u32 = 0x0126_835c;
/// Profiling labels: `"Ttupdate character state"`, `"TtCharacter movement"`, `"Et"`.
const PROFILE_UPDATE_STATE: u32 = 0x010c_4ad8;
const PROFILE_MOVEMENT: u32 = 0x010c_4ac0;
const PROFILE_END: u32 = 0x010c_41f8;
/// `bhkCharacterController::SetPosition` (Xbox PDB, one stack argument, the position).
const SET_POSITION: u32 = 0x0056_20e0;
/// State lookup on the context's state manager (`00cbc510`, `this` = `+0x3EC`, the
/// state); the state's slot 0x20 updates the controller.
const STATE_LOOKUP: u32 = 0x00cb_c510;
/// Per-lane "equal within a tolerance" of two points (`0049e2f0`, cdecl `(a, b,
/// tolerance)`).
const POINTS_NEAR: u32 = 0x0049_e2f0;
/// `00621ad0` (`this` only): an object whose `+0x30` blocks movement while flag bit 22 is
/// set.
const MOVE_BLOCKER: u32 = 0x0062_1ad0;
/// `hkQuaternion::setAxisAngle` (Xbox PDB, `this`, axis, angle).
const SET_AXIS_ANGLE: u32 = 0x00cb_2450;
/// Converts a point to a vector with `w` 0 (`004a3e00`, cdecl `(out, point)`).
const POINT_TO_VECTOR: u32 = 0x004a_3e00;
/// `mobileobject.cpp` functions on the controller: `009306b0` (state 5 test) and
/// `00933810` (the projectile state object, whose slot 0x20 updates).
const STATE_FIVE_TEST: u32 = 0x0093_06b0;
const PROJECTILE_STATE: u32 = 0x0093_3810;
/// `0080fb00` (`this`, the velocity): run before `MoveAsProjectile`.
const PROJECTILE_VELOCITY: u32 = 0x0080_fb00;
/// `004390c0` (`this` = a point, another point): byte result.
const POINT_COMPARE: u32 = 0x0043_90c0;
/// `hkpCharacterProxy` linear velocity setter (`00cac960`, one stack argument).
const HKP_SET_LINEAR_VELOCITY: u32 = 0x00ca_c960;
/// `bhkCharacterProxy::operatorP` (Xbox PDB, `this` only): the Havok proxy.
const PROXY_OPERATOR: u32 = 0x004a_e750;
/// `hkpRootCdPoint::operator=` (Xbox PDB, one stack argument).
const ROOT_CD_POINT_ASSIGN: u32 = 0x0062_12c0;
/// `float` global dividing the speed into `fSpeedPct` (0 in the exe image).
const SPEED_DIVISOR: u32 = 0x0126_7bc4;
/// `double` 0.0001, the smallest time step `Move` works with.
const MIN_STEP: u32 = 0x0103_2980;
/// `float` 0.001 (the per-lane tolerance), the absolute-value mask, `float` -1.0, the
/// four `-0.5` of the quaternion rotation, the sign mask, and the pitch timer reset (5.0).
const LANE_TOLERANCE: u32 = 0x0101_7d00;
const ABS_MASK: u32 = 0x0101_7d10;
const FLOAT_MINUS_ONE: u32 = 0x0101_2054;
const ROTATION_BIAS: u32 = 0x010c_71e0;
const SIGN_MASK: u32 = 0x010c_44d0;
const PITCH_TIMER_RESET: u32 = 0x011b_014c;

/// `bhkcharacterlistener.obj` function `00c6c890` (`this` = the listener, then its
/// `+0x1A0` and the contact point).
const RECORD_SUPPORT_POINT: u32 = 0x00c6_c890;
/// List append (`00cbc7b0`, `bhkliquidaction.obj`; `this` = the list, one argument: the
/// address of the item pointer).
const LIST_APPEND: u32 = 0x00cb_c7b0;
/// `double` 0.333 (the foot capsule radius factor), 0.985 (the contact-normal z above
/// which the support probe runs) and 0.2.
const FOOT_RADIUS_FACTOR: u32 = 0x010c_4ab0;
const UPRIGHT_NORMAL: u32 = 0x010c_4aa8;
const DOUBLE_FIFTH: u32 = 0x0101_8a90;
/// `float` values written to a plane's `+0x28`: for layer-2 bodies of motion type 4 or 5
/// (0.577) and on flat ground (5.77).
const PLANE_SPEED_MOVING: u32 = 0x010a_5898;
const PLANE_SPEED_FLAT: u32 = 0x010c_4a98;
/// The support probe's two offsets (function statics built on first use from the zero
/// float and the floats at `010c4aa0` (36.0) and `010c4a9c` (54.0), guard bits at
/// `01267c10`), its best height (`01267bb0`, reset from `01015f5c`) and the body that
/// gave it (`01267bb4`).
const PROBE_GUARD: u32 = 0x0126_7c10;
const PROBE_RAISE: u32 = 0x0126_7c00;
const PROBE_DROP: u32 = 0x0126_7bf0;
const PROBE_RAISE_Z: u32 = 0x010c_4aa0;
const PROBE_DROP_Z: u32 = 0x010c_4a9c;
const BEST_SUPPORT_HEIGHT: u32 = 0x0126_7bb0;
const BEST_SUPPORT_RESET: u32 = 0x0101_5f5c;
const BEST_SUPPORT_BODY: u32 = 0x0126_7bb4;
/// Havok-unit function `00cd72a0` (cdecl `(triangle vertices, out)`) and
/// `hkCollideCapsuleUtilCapsVsTri` (Xbox PDB, cdecl, nine words).
const TRIANGLE_PREPARE: u32 = 0x00cd_72a0;
const CAPSULE_VS_TRIANGLE: u32 = 0x00cd_6520;
/// `00c6c720` on the listener `(index, z)`: stores `z` in `SupportHeight[index]` when that
/// still holds the float at `0102efc4`, else their mean.
const RECORD_SUPPORT_HEIGHT: u32 = 0x00c6_c720;
/// 16-bit counter `00c6da50` advances for each cinfo.
const FILTER_COUNTER: u32 = 0x011b_0808;
/// Constructor of the shared object at `01267bb8` (`00cbc460`, `this` only) and its
/// registration `00cbc4e0` (`(state, slot)`).
const STATE_MANAGER_CONSTRUCT: u32 = 0x00cb_c460;
const STATE_REGISTER: u32 = 0x00cb_c4e0;
/// The state constructors `00c6da50` runs (`this` only), by the slot they fill.
const STATE_CONSTRUCT_0: u32 = 0x00cd_47e0;
const STATE_CONSTRUCT_5: u32 = 0x00cd_44c0;
const STATE_CONSTRUCT_1: u32 = 0x00cd_4260;
const STATE_CONSTRUCT_2: u32 = 0x00cd_3f90;
const STATE_CONSTRUCT_6: u32 = 0x00cd_3c90;
/// Vtables of `bhkCharacterStateClimbing` and `bhkCharacterStateFlying` (RTTI).
const VTABLE_STATE_CLIMBING: u32 = 0x010c_4944;
const VTABLE_STATE_FLYING: u32 = 0x010c_496c;

layout! {
    /// `bhkCharacterController` (Xbox PDB), 0x650 bytes; PC offsets as this unit uses
    /// them (they agree with the PDB).
    pub struct BhkCharacterController: 0x650 {
        /// `phkObject` (Xbox PDB): the `hkpCharacterProxy`.
        0x008 phkObject: u32,
        /// `pInfo` (Xbox PDB).
        0x00c pInfo: u32,
        /// `hkpCharacterContext::m_stateManager` and `m_currentState` (Xbox PDB).
        0x3ec m_stateManager: u32,
        0x3f0 m_currentState: u32,
        /// `iFlags` (Xbox PDB) of the `bhkCharacterListener` base.
        0x414 iFlags: u32,
        /// `eSupportMaterial` (Xbox PDB).
        0x434 eSupportMaterial: u32,
        /// `eSupportLayer` (Xbox PDB).
        0x438 eSupportLayer: u32,
        /// `hkSupportVelocity` (Xbox PDB).
        0x440 hkSupportVelocity: [f32; 4],
        /// `hkSupportNorm` (Xbox PDB).
        0x450 hkSupportNorm: [f32; 4],
        /// `SupportHeight` (Xbox PDB), four floats.
        0x460 SupportHeight: [f32; 4],
        /// `fMaxSlope` (Xbox PDB).
        0x430 fMaxSlope: f32,
        /// `fStepHeight` (Xbox PDB).
        0x470 fStepHeight: f32,
        /// `fCastDepth` (Xbox PDB).
        0x474 fCastDepth: f32,
        /// `bAllSurfacesStopped` (Xbox PDB).
        0x478 bAllSurfacesStopped: u8,
        /// `hkpSurfaceInfo::m_supportedState` and `m_surfaceVelocity` (Xbox PDB).
        0x480 m_supportedState: u32,
        0x4a0 m_surfaceVelocity: [f32; 4],
        /// `UpVec` (Xbox PDB).
        0x4c0 UpVec: [f32; 4],
        /// `ForwardVec` (Xbox PDB).
        0x4d0 ForwardVec: [f32; 4],
        /// `StepInfo` (Xbox PDB `hkStepInfo` at +0x4E0): `m_deltaTime` and
        /// `m_invDeltaTime` (Havok's `hkStepInfo` layout).
        0x4e0 StepInfo: [f32; 4],
        0x4e8 StepDeltaTime: f32,
        0x4ec StepInvDeltaTime: f32,
        /// `OutVelocity` (Xbox PDB).
        0x4f0 OutVelocity: [f32; 4],
        /// `VelocityMod` (Xbox PDB).
        0x500 VelocityMod: [f32; 4],
        /// `Direction` (Xbox PDB).
        0x510 Direction: [f32; 4],
        /// `eWantState` (Xbox PDB).
        0x520 eWantState: u32,
        /// `fVelocityTime` (Xbox PDB).
        0x524 fVelocityTime: f32,
        /// `fRotMod` (Xbox PDB).
        0x528 fRotMod: f32,
        /// `fRotModTime` (Xbox PDB).
        0x52c fRotModTime: f32,
        /// `fCalculatePitchTimer` (Xbox PDB).
        0x530 fCalculatePitchTimer: f32,
        /// `fAcrobatics` (Xbox PDB).
        0x534 fAcrobatics: f32,
        /// `fCenter` (Xbox PDB).
        0x538 fCenter: f32,
        /// `fJumpHeight` (Xbox PDB).
        0x540 fJumpHeight: f32,
        /// `fFallStartHeight` (Xbox PDB).
        0x544 fFallStartHeight: f32,
        /// `fFallTime` (Xbox PDB).
        0x548 fFallTime: f32,
        /// `fGravity` (Xbox PDB).
        0x54c fGravity: f32,
        /// `fPitchAngle` (Xbox PDB).
        0x550 fPitchAngle: f32,
        /// `fRollAngle` (Xbox PDB).
        0x554 fRollAngle: f32,
        /// `fPitchMult` (Xbox PDB).
        0x558 fPitchMult: f32,
        /// `fActorHeight` (Xbox PDB).
        0x564 fActorHeight: f32,
        /// `fSpeedPct` (Xbox PDB).
        0x568 fSpeedPct: f32,
        /// `RotCenter` (Xbox PDB).
        0x570 RotCenter: [f32; 4],
        /// `PushDelta` (Xbox PDB).
        0x580 PushDelta: [f32; 4],
        /// `iPushCount` (Xbox PDB).
        0x590 iPushCount: u32,
        /// `spShapePhantom` (Xbox PDB): `bhkShapePhantom*`.
        0x594 spShapePhantom: u32,
        /// `spCollisionBox` (Xbox PDB): `bhkCollisionBox*`.
        0x598 spCollisionBox: u32,
        /// `eShapeType` (Xbox PDB).
        0x59c eShapeType: i32,
        /// `eSizedShapeType` (Xbox PDB).
        0x5a0 eSizedShapeType: i32,
        /// `spShapes` (Xbox PDB): `bhkShape*` per shape type.
        0x5a4 spShapes: u32,
        /// `FrontBack` (Xbox PDB): four `hkVector4`.
        0x5b0 FrontBack: [f32; 4],
        /// `fRadius` (Xbox PDB).
        0x5f0 fRadius: f32,
        /// `fHeight` (Xbox PDB).
        0x5f4 fHeight: f32,
        /// `fDestRadius` (Xbox PDB).
        0x5f8 fDestRadius: f32,
        /// `fLODDistance` (Xbox PDB).
        0x5fc fLODDistance: f32,
        /// `eSize` (Xbox PDB).
        0x600 eSize: u32,
        /// `iPriority` (Xbox PDB).
        0x604 iPriority: u32,
        /// `iSupportCount` (Xbox PDB).
        0x608 iSupportCount: u32,
        /// `spSupportBody` (Xbox PDB): `hkRefPtr<hkpRigidBody>`.
        0x60c spSupportBody: u32,
        /// `spBumpedBody` (Xbox PDB): `hkRefPtr<hkpRigidBody>`.
        0x630 spBumpedBody: u32,
        /// `fBumpedForce` (Xbox PDB).
        0x634 fBumpedForce: f32,
        /// `spBumpedCharProxy` (Xbox PDB): `hkRefPtr<hkpCharacterProxy>`.
        0x638 spBumpedCharProxy: u32,
        /// `LastManifold` (Xbox PDB): `hkArray<hkpRootCdPoint>` data, size and
        /// capacity-and-flags words (0x70-byte entries).
        0x63c LastManifold: u32,
        0x640 LastManifoldSize: u32,
        0x644 LastManifoldCapacity: u32,
        /// `DamageImpacts` (Xbox PDB): `BSSimpleList<DamageImpactData*>`, item then next.
        0x648 DamageImpacts: u32,
        0x64c DamageImpactsNext: u32,
    }
}

type Ctl = BhkCharacterController;

/// Four floats at `a`, bit for bit.
fn load4(e: &Engine, a: u32) -> [f32; 4] {
    [0, 4, 8, 12].map(|o| e.mem.f32(a.wrapping_add(o)))
}

fn store4(e: &mut Engine, a: u32, v: [f32; 4]) {
    for (lane, value) in v.into_iter().enumerate() {
        e.mem.set_f32(a.wrapping_add(4 * lane as u32), value);
    }
}

/// `n` words from `src` to `dst` (`MOVAPS`/`MOV` copies).
fn copy_words(e: &mut Engine, dst: u32, src: u32, n: u32) {
    for i in 0..n {
        let w = e.mem.u32(src.wrapping_add(4 * i));
        e.mem.set_u32(dst.wrapping_add(4 * i), w);
    }
}

/// `0 < v` (false for NaN), as the `FCOM`/`TEST AH,5` pairs test it.
fn is_positive(v: f32) -> bool {
    0.0 < v
}

/// `x2*x2 + (x1*x1 + x0*x0)` in single precision, the `MULPS`/`SHUFPS`/`ADDSS` sum.
fn sum_of_squares3(v: [f32; 4]) -> f32 {
    v[2] * v[2] + (v[1] * v[1] + v[0] * v[0])
}

/// The `RSQRTSS` estimate refined once, with the `CMPEQSS`/`ANDNPS` zero guard:
/// `(half * r) * (three - (sum * r) * r)`, `+0.0` when `sum == zero`.
fn inverse_length(sum: f32, zero: f32, half: f32, three: f32) -> f32 {
    let estimate = rsqrt_estimate(sum);
    let step = three - (sum * estimate) * estimate;
    let value = (half * estimate) * step;
    if sum == zero {
        0.0
    } else {
        value
    }
}

/// Sets the allocation-scope word to 0x10 and returns its address and old value.
fn enter_scope(e: &mut Engine) -> (u32, u32) {
    let word = e.tls() + SCOPE_WORD;
    let saved = e.mem.u32(word);
    e.mem.set_u32(word, 0x10);
    (word, saved)
}

/// Frees `bytes` at `block` through the current Havok allocator (slot 8).
fn havok_free(e: &mut Engine, block: u32, bytes: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    e.vcall(allocator, 8, &args![block, bytes]);
}

/// `NiPointer` release: `InterlockedDecrement(&object->m_uiRefCount)`, then
/// `DeleteThis` (slot 4) when it reached 0.
fn release_ni_object(e: &mut Engine, object: u32) {
    let count = e
        .call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .i32();
    if count == 0 {
        e.vcall(object, 4, &args![]);
    }
}

/// Releases a fixed-string handle unless it is the empty one.
fn release_fixed_string(e: &mut Engine, handle: u32) {
    if handle != e.global::<u32>(EMPTY_FIXED_STRING) {
        e.call(INTERLOCKED_DECREMENT, &args![handle.wrapping_sub(8)]);
    }
}

fn increment_count(e: &mut Engine, counter: u32) {
    let value: u32 = e.global(counter);
    e.set_global(counter, value.wrapping_add(1));
}

/// The listener `this` of a callback back to its controller.
fn controller_of(listener: Ptr) -> Ptr<Ctl> {
    Ptr::new(listener.addr().wrapping_sub(LISTENER))
}

// Translated from 00c6cba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCharacterController::UpdateState` (Xbox PDB): unless `eWantState` is 0xB, makes it
/// the `hkpCharacterContext`'s `m_currentState` and sets it to 0xB.
pub fn bhk_character_controller_update_state(e: &mut Engine, this: Ptr<Ctl>) {
    let wanted = e.get(this, Ctl::eWantState);
    if wanted != 0xb {
        e.set(this, Ctl::m_currentState, wanted);
        e.set(this, Ctl::eWantState, 0xb);
    }
}

// Translated from 00c6cbc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `fRotMod` to `rot_mod` and `fRotModTime` to `time` when `|rot_mod|` is larger than
/// the current `|fRotMod|` (ordered compare: a NaN never replaces it).
pub fn fn_00c6cbc0(e: &mut Engine, this: Ptr<Ctl>, rot_mod: f32, time: f32) {
    let rot_mod = x87_quiet(rot_mod);
    let current = x87_quiet(e.get(this, Ctl::fRotMod)).abs();
    if current < rot_mod.abs() {
        e.set(this, Ctl::fRotMod, rot_mod);
        e.set(this, Ctl::fRotModTime, x87_quiet(time));
    }
}

// Translated from 00c6cc10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCharacterController::ApplyRotationModifier` (Xbox PDB): when `fRotModTime > 0`,
/// returns `value + fRotMod * min(fRotModTime, time)` (the time when `fRotModTime >
/// time`), lowers `fRotModTime` by `time` and `fRotMod` by 0.05; otherwise clears
/// `fRotMod` and returns `value`.
#[allow(clippy::neg_cmp_op_on_partial_ord)] // the x87 compare treats NaN as "not above 0"
pub fn bhk_character_controller_apply_rotation_modifier(
    e: &mut Engine,
    this: Ptr<Ctl>,
    value: f32,
    time: f32,
) -> f32 {
    let value = x87_quiet(value);
    let remaining = e.get(this, Ctl::fRotModTime);
    if !(0.0 < remaining) {
        e.set(this, Ctl::fRotMod, 0.0);
        return value;
    }
    let remaining = x87_quiet(remaining);
    let time = x87_quiet(time);
    let used = if remaining > time { time } else { remaining };
    // `FLD fRotMod`, `FMUL` the scratch float, `FADDP` onto the value; then
    // `FSUBR fRotModTime` (the field as a memory operand) on the time, and
    // `FLD fRotMod`, `FSUB` the double: the x87 choice of NaN throughout.
    let rot_mod = x87_load32(e.get(this, Ctl::fRotMod).to_bits());
    let product = x87_mul(rot_mod, x87_m32(used.to_bits()));
    let result = f32::from_bits(x87_store32(x87_add(f64::from(value), product)));
    let remaining_field = x87_m32(e.get(this, Ctl::fRotModTime).to_bits());
    let left = x87_store32(x87_sub(remaining_field, f64::from(time)));
    e.set(this, Ctl::fRotModTime, f32::from_bits(left));
    let decay: f64 = e.global(ROT_MOD_DECAY);
    let rot_mod = x87_load32(e.get(this, Ctl::fRotMod).to_bits());
    e.set(
        this,
        Ctl::fRotMod,
        f32::from_bits(x87_store32(x87_sub(rot_mod, decay))),
    );
    result
}

// Translated from 00c6cca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkRefPtr` release: removes a reference from the object `*this` points to (when
/// there is one) and clears the pointer.
pub fn fn_00c6cca0(e: &mut Engine, this: Ptr) {
    let object = e.mem.u32(this.addr());
    if object != 0 {
        e.call(REMOVE_REFERENCE, &args![object]);
    }
    e.mem.set_u32(this.addr(), 0);
}

// Translated from 00c6ccc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// cdecl `(out, a, b)`: `out = a` with its first lane `a.x - b.x` (`SUBSS`).
pub fn fn_00c6ccc0(e: &mut Engine, out: Ptr, a: Ptr, b: Ptr) {
    let mut v = load4(e, a.addr());
    v[0] -= e.mem.f32(b.addr());
    store4(e, out.addr(), v);
}

// Translated from 00c6cce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// cdecl `(out, a, b)`: `out = a` with its first lane `a.x + b.x` (`ADDSS`).
/// Answers `out` (`MOV EAX,[EBP+8]` before the `RET`): callers read the vector through `EAX`
/// (00d8cd9f).
pub fn fn_00c6cce0(e: &mut Engine, out: Ptr, a: Ptr, b: Ptr) -> Ptr {
    let mut v = load4(e, a.addr());
    v[0] += e.mem.f32(b.addr());
    store4(e, out.addr(), v);
    out
}

// Translated from 00c6cd00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the 0x30 bytes at `source` to `this` (three `MOVAPS`).
pub fn fn_00c6cd00(e: &mut Engine, this: Ptr, source: Ptr) {
    copy_words(e, this.addr(), source.addr(), 12);
}

// Translated from 00c6cd30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the 0x30 bytes at `source` to `this`; returns `this`.
pub fn fn_00c6cd30(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    copy_words(e, this.addr(), source.addr(), 12);
    this
}

// Translated from 00c6cd60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the 0x40 bytes at `source` to `this`; returns `this`.
pub fn fn_00c6cd60(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    copy_words(e, this.addr(), source.addr(), 16);
    this
}

// Translated from 00c6cd90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkShapeCollection::GetRTTI` (Xbox PDB).
pub fn bhk_shape_collection_get_rtti(_e: &mut Engine, _this: Ptr) -> u32 {
    SHAPE_COLLECTION_RTTI
}

// Translated from 00c6cda0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkShapeCollection::_scalar_deleting_destructor_` (Xbox PDB): destroys, and frees the
/// 0x14 bytes when bit 0 of `flags` is set.
pub fn bhk_shape_collection_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u8,
) -> Ptr {
    e.call(SHAPE_COLLECTION_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE_SIZED, &args![this, 0x14u32]);
    }
    this
}

// Translated from 00c6cdc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// cdecl: frees a 0x80-byte block (`bhkCharacterProxy`'s cinfo size) through the Havok
/// allocator when `block` is not null.
pub fn fn_00c6cdc0(e: &mut Engine, block: u32) {
    if block != 0 {
        havok_free(e, block, 0x80);
    }
}

// Translated from 00c6cde0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills an `hkpCharacterProxyCinfo` (Xbox PDB layout) with defaults: position and
/// velocity the `float` zero at `01011d78` (bit copies), dynamic friction 1, static
/// friction 0, keep-contact tolerance 0.1, up `(0, 1, 0, 0)` (the zero and one constants'
/// bits), extra frictions 0, no phantom, keep distance 0.05, contact-angle sensitivity
/// 10, 4 user planes, solver speed 10, strength the collector's far value, mass 0, max
/// slope pi/2, recovery speed 1, 10 cast iterations, no manifold refresh. `+0x2C` is not
/// written.
pub fn fn_00c6cde0(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    let zero = e.global::<u32>(FLOAT_ZERO);
    let one = e.global::<u32>(FLOAT_ONE);
    e.mem.set_f32(me + 0x20, 1.0);
    e.mem.set_f32(me + 0x24, 0.0);
    let tenth = x87_quiet_bits(e.global::<u32>(FLOAT_TENTH));
    e.mem.set_u32(me + 0x28, tenth);
    e.mem.set_u32(me + 0x48, 0);
    e.mem.set_u32(me + 0x54, 4);
    e.mem.set_f32(me + 0x40, 0.0);
    e.mem.set_f32(me + 0x44, 0.0);
    e.mem.set_u32(me + 0x6c, 10);
    let twentieth = x87_quiet_bits(e.global::<u32>(FLOAT_TWENTIETH));
    e.mem.set_u32(me + 0x4c, twentieth);
    let ten = x87_quiet_bits(e.global::<u32>(FLOAT_TEN));
    e.mem.set_u32(me + 0x50, ten);
    for word in 0..8 {
        e.mem.set_u32(me + 4 * word, zero);
    }
    e.mem.set_u32(me + 0x58, ten);
    let far = x87_quiet_bits(e.global::<u32>(COLLECTOR_FAR));
    e.mem.set_u32(me + 0x5c, far);
    e.mem.set_u8(me + 0x70, 0);
    e.mem.set_f32(me + 0x60, 0.0);
    let half_pi = x87_quiet_bits(e.global::<u32>(FLOAT_HALF_PI));
    e.mem.set_u32(me + 0x64, half_pi);
    e.mem.set_f32(me + 0x68, 1.0);
    for (lane, word) in [zero, one, zero, zero].into_iter().enumerate() {
        e.mem.set_u32(me + 0x30 + 4 * lane as u32, word);
    }
}

// Translated from 00c6ce80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c6ce80 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkCharacterProxy` constructor (RTTI of vtable `010c45cc`): the base constructor,
/// `+0xC` cleared, the class vtable, the `bhkCharacterPointCollector` at `+0x10`
/// constructed with 0, and its count. Returns `this`.
pub fn fn_00c6ce80(e: &mut Engine, this: Ptr) -> Ptr {
    let me = this.addr();
    e.call(REF_OBJECT_CONSTRUCT, &args![this]);
    e.mem.set_u32(me + 0xc, 0);
    e.mem.set_u32(me, VTABLE_CHARACTER_PROXY);
    e.call(POINT_COLLECTOR_CONSTRUCT, &args![me + 0x10, 0u32]);
    increment_count(e, COUNT_CHARACTER_PROXY);
    this
}

// Translated from 00c6cee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCharacterProxy::QCinfoSize` (Xbox PDB): 0x80.
pub fn bhk_character_proxy_q_cinfo_size(_e: &mut Engine, _this: Ptr) -> u32 {
    0x80
}

// Translated from 00c6cef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCharacterProxy::CheckSupport` (Xbox PDB): when the proxy and its Havok proxy
/// exist, `hkpCharacterProxy::checkSupport(first, second)` between two `UnMarkWrite`
/// calls.
pub fn bhk_character_proxy_check_support(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    if this.addr() == 0 {
        return;
    }
    let proxy = e.mem.u32(this.addr() + 8);
    if proxy == 0 {
        return;
    }
    e.call(UN_MARK_WRITE, &args![this]);
    e.call(HKP_CHECK_SUPPORT, &args![proxy, first, second]);
    e.call(UN_MARK_WRITE, &args![this]);
}

// Translated from 00c6cf30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c6cf30 - the FS:[0] exception-unwinding frame and the /GS cookie check are not modeled.
/// `bhkCharacterProxy::Integrate` (Xbox PDB): between two `UnMarkWrite` calls, when the
/// proxy and its Havok proxy exist, builds a `bhkCharacterPointCollector` on the stack
/// (constructed with `this + 0x10`), runs `integrateWithCollectors(step_info, world + 0x10,
/// collector, this + 0x10)` with the world from vtable slot 0x94, handles the
/// collisions of `this + 0x10` and destroys the stack collector.
pub fn bhk_character_proxy_integrate(e: &mut Engine, this: Ptr, step_info: u32) {
    e.call(UN_MARK_WRITE, &args![this]);
    if this.addr() != 0 {
        let proxy = e.mem.u32(this.addr() + 8);
        if proxy != 0 {
            let owner = this.addr() + 0x10;
            e.with_stack(0x3dc, |e, collector| {
                e.call(POINT_COLLECTOR_CONSTRUCT, &args![collector, owner]);
                let world = e.vcall(this.addr(), 0x94, &args![]).u32();
                e.call(
                    INTEGRATE_WITH_COLLECTORS,
                    &args![proxy, step_info, world.wrapping_add(0x10), collector, owner],
                );
                e.call(POINT_COLLECTOR_HANDLE_COLLISIONS, &args![owner]);
                e.call(POINT_COLLECTOR_DESTRUCT, &args![collector]);
            });
        }
    }
    e.call(UN_MARK_WRITE, &args![this]);
}

// Translated from 00c6d010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCharacterProxy::_scalar_deleting_destructor_` (Xbox PDB): destroys (`00cabf50`) and
/// frees the 0x3E0 bytes when bit 0 of `flags` is set.
pub fn bhk_character_proxy_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(CHARACTER_PROXY_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE_SIZED, &args![this, 0x3e0u32]);
    }
    this
}

// Translated from 00c6d040 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c6d040 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkCachingShapePhantom::bhkCachingShapePhantom` (Xbox PDB): the inlined
/// `bhkWorldObject`, `bhkPhantom` and `bhkShapePhantom` constructors (vtables, cleared
/// `+0xC`/`+0x10`/`+0x14`, one count each), then the class vtable, `Init(info)` and its
/// own count. Returns `this`.
pub fn bhk_caching_shape_phantom_construct(e: &mut Engine, this: Ptr, info: u32) -> Ptr {
    let me = this.addr();
    construct_shape_phantom_bases(e, me);
    e.mem.set_u32(me, VTABLE_CACHING_SHAPE_PHANTOM);
    e.call(CACHING_PHANTOM_INIT, &args![this, info]);
    increment_count(e, COUNT_CACHING_SHAPE_PHANTOM);
    e.mem.set_u8(me + 0x14, 0);
    this
}

// Translated from 00c6d0e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCachingShapePhantom::GetRTTI` (Xbox PDB).
pub fn bhk_caching_shape_phantom_get_rtti(_e: &mut Engine, _this: Ptr) -> u32 {
    CACHING_PHANTOM_RTTI
}

// Translated from 00c6d0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns 0x60 (a size query; the slot is not named in the map).
pub fn fn_00c6d0f0(_e: &mut Engine, _this: Ptr) -> u32 {
    0x60
}

// Translated from 00c6d100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCachingShapePhantom::_scalar_deleting_destructor_` (Xbox PDB): destroys and frees
/// the 0x18 bytes when bit 0 of `flags` is set.
pub fn bhk_caching_shape_phantom_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u8,
) -> Ptr {
    e.call(CACHING_PHANTOM_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE_SIZED, &args![this, 0x18u32]);
    }
    this
}

// Translated from 00c6d120 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when the `hkpCharacterContext` (+0x3E0) state is 4, 5 or 6.
pub fn fn_00c6d120(e: &mut Engine, this: Ptr<Ctl>) -> bool {
    let state = e.call(GET_STATE, &args![this.addr() + 0x3e0]).u32();
    matches!(state, 4..=6)
}

// Translated from 00c6d150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears `PushDelta` and `iPushCount` when `iPushCount` is not 0.
pub fn fn_00c6d150(e: &mut Engine, this: Ptr<Ctl>) {
    if e.get(this, Ctl::iPushCount) != 0 {
        copy_zero(e, this.addr() + Ctl::PushDelta.off, 4);
        e.set(this, Ctl::iPushCount, 0);
    }
}

fn copy_zero(e: &mut Engine, dst: u32, words: u32) {
    for i in 0..words {
        e.mem.set_u32(dst + 4 * i, 0);
    }
}

// Translated from 00c6d170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpClosestCdPointCollector::reset` (Xbox PDB): clears `+0x30` and sets both distances
/// (`+0x2C`, `+4`) to the exe's far value.
pub fn hkp_closest_cd_point_collector_reset(e: &mut Engine, this: Ptr) {
    let far = x87_quiet_bits(e.global::<u32>(COLLECTOR_FAR));
    e.mem.set_u32(this.addr() + 0x30, 0);
    e.mem.set_u32(this.addr() + 0x2c, far);
    e.mem.set_u32(this.addr() + 4, far);
}

/// The inlined `bhkWorldObject`/`bhkPhantom`/`bhkShapePhantom` constructor chain.
fn construct_shape_phantom_bases(e: &mut Engine, me: u32) {
    e.call(REF_OBJECT_CONSTRUCT, &args![me]);
    e.mem.set_u32(me, VTABLE_WORLD_OBJECT);
    e.mem.set_u32(me + 0xc, 0);
    e.mem.set_u32(me + 0x10, 0);
    increment_count(e, COUNT_WORLD_OBJECT);
    e.mem.set_u32(me, VTABLE_PHANTOM);
    increment_count(e, COUNT_PHANTOM);
    e.mem.set_u32(me, VTABLE_SHAPE_PHANTOM);
    increment_count(e, COUNT_SHAPE_PHANTOM);
    e.mem.set_u8(me + 0x14, 0);
}

// Translated from 00c6d190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkSimpleShapePhantom` constructor (RTTI of vtable `010305dc`): the base chain, then
/// its vtable and count, `+0x14` cleared. Returns `this`.
pub fn fn_00c6d190(e: &mut Engine, this: Ptr) -> Ptr {
    construct_shape_phantom_bases(e, this.addr());
    e.mem.set_u32(this.addr(), VTABLE_SIMPLE_SHAPE_PHANTOM);
    increment_count(e, COUNT_SIMPLE_SHAPE_PHANTOM);
    e.mem.set_u8(this.addr() + 0x14, 0);
    this
}

// Translated from 00c6d1e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c6d1e0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkCollisionBox` constructor (RTTI of vtable `010c477c`): the
/// `bhkSimpleShapePhantom` constructor, the class vtable, then `bhkCollisionBox::Init`
/// with both arguments. Returns `this`.
pub fn fn_00c6d1e0(e: &mut Engine, this: Ptr, first: u32, second: u32) -> Ptr {
    fn_00c6d190(e, this);
    e.mem.set_u32(this.addr(), VTABLE_COLLISION_BOX);
    e.call(COLLISION_BOX_INIT, &args![this, first, second]);
    this
}

// Translated from 00c6d250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCollisionBox::GetRTTI` (Xbox PDB).
pub fn bhk_collision_box_get_rtti(_e: &mut Engine, _this: Ptr) -> u32 {
    COLLISION_BOX_RTTI
}

// Translated from 00c6d260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCollisionBox::_scalar_deleting_destructor_` (Xbox PDB): destroys, and when bit 0 of
/// `flags` is set and `this` is not null frees the aligned block (`this` minus the
/// offset byte at `this - 1`) through the `MemoryManager`.
pub fn bhk_collision_box_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(COLLISION_BOX_DESTRUCT, &args![this]);
    if flags & 1 != 0 && this.addr() != 0 {
        let offset = e.mem.u8(this.addr().wrapping_sub(1));
        let block = this.addr().wrapping_sub(u32::from(offset));
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, block]);
    }
    this
}

// Translated from 00c6d290 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c6d290 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkListShape` constructor (RTTI of vtable `010c485c`): the inlined `bhkShape` and
/// `bhkShapeCollection` constructors (vtables, cleared `+0xC`/`+0x10`, counts), the class
/// vtable, `bhkListShape::Init(info)` and its count. Returns `this`.
pub fn fn_00c6d290(e: &mut Engine, this: Ptr, info: u32) -> Ptr {
    let me = this.addr();
    e.call(REF_OBJECT_CONSTRUCT, &args![this]);
    e.mem.set_u32(me, VTABLE_SHAPE);
    e.mem.set_u32(me + 0xc, 0);
    e.mem.set_u32(me + 0x10, 0);
    increment_count(e, COUNT_SHAPE);
    e.mem.set_u32(me, VTABLE_SHAPE_COLLECTION);
    increment_count(e, COUNT_SHAPE_COLLECTION);
    e.mem.set_u32(me, VTABLE_LIST_SHAPE);
    e.call(LIST_SHAPE_INIT, &args![this, info]);
    increment_count(e, COUNT_LIST_SHAPE);
    this
}

// Translated from 00c6d310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkListShape::GetRTTI` (Xbox PDB).
pub fn bhk_list_shape_get_rtti(_e: &mut Engine, _this: Ptr) -> u32 {
    LIST_SHAPE_RTTI
}

// Translated from 00c6d320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkListShape::_scalar_deleting_destructor_` (Xbox PDB): destroys and frees the 0x14
/// bytes when bit 0 of `flags` is set.
pub fn bhk_list_shape_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(LIST_SHAPE_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE_SIZED, &args![this, 0x14u32]);
    }
    this
}

// Translated from 00c6d340 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c6d340 - the FS:[0] exception-unwinding frame is not modeled.
/// Static tear-down: with the allocation-scope word at 0x10, removes a reference from
/// the object at `01267bb8` when there is one (the pointer is left as it is).
pub fn fn_00c6d340(e: &mut Engine) {
    let (word, saved) = enter_scope(e);
    let object: u32 = e.global(STATIC_REFERENCED_OBJECT);
    if object != 0 {
        e.call(REMOVE_REFERENCE, &args![object]);
    }
    e.mem.set_u32(word, saved);
}

// Translated from 00c6d3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Static tear-down of the `NiPointer` at `01267be0`: releases the object and clears it.
pub fn fn_00c6d3c0(e: &mut Engine) {
    let object: u32 = e.global(STATIC_NI_POINTER);
    if object != 0 {
        release_ni_object(e, object);
        e.set_global(STATIC_NI_POINTER, 0u32);
    }
}

// Translated from 00c6d3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCharacterController::SetShape` (Xbox PDB): gives the Havok proxy's shape phantom
/// the `hkpShape` of `shape` (its `+8`, or none). Does nothing (returns false) without a
/// proxy or phantom, or when the phantom already has that shape. Otherwise: the
/// `UnMarkRead`/`getPosition`/`UnMarkRead` sequence (result unused), the phantom leaves
/// its world (when it has one), gets the shape (vtable slot 0xC) and is added back to the
/// same world; returns true. The second stack word is never read.
pub fn bhk_character_controller_set_shape(
    e: &mut Engine,
    this: Ptr<Ctl>,
    shape: u32,
    _unused_2: u32,
) -> bool {
    let proxy = e.get(this, Ctl::phkObject);
    if proxy == 0 {
        return false;
    }
    let phantom = e.call(GET_SHAPE_PHANTOM, &args![proxy]).u32();
    if phantom == 0 {
        return false;
    }
    let hk_shape = if shape != 0 { e.mem.u32(shape + 8) } else { 0 };
    if e.mem.u32(phantom + 0x10) == hk_shape {
        return false;
    }
    let proxy = e.get(this, Ctl::phkObject);
    let world = e.mem.u32(phantom + 8);
    if proxy != 0 {
        e.call(UN_MARK_READ, &args![this]);
        e.call(HKP_GET_POSITION, &args![proxy]);
        e.call(UN_MARK_READ, &args![this]);
    }
    if world != 0 {
        e.call(WORLD_REMOVE_PHANTOM, &args![world, phantom]);
    }
    let hk_shape = if shape != 0 { e.mem.u32(shape + 8) } else { 0 };
    e.vcall(phantom, 0xc, &args![hk_shape]);
    if world != 0 {
        e.call(WORLD_ADD_PHANTOM, &args![world, phantom]);
    }
    true
}

// Translated from 00c6d4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `bhkCollisionBox::Update` on `spCollisionBox` when there is one (tail call).
pub fn fn_00c6d4a0(e: &mut Engine, this: Ptr<Ctl>) {
    let collision_box = e.get(this, Ctl::spCollisionBox);
    if collision_box != 0 {
        e.call(COLLISION_BOX_UPDATE, &args![collision_box]);
    }
}

// Translated from 00c6d4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `eWantState` to 1 and `fJumpHeight` to `height` times the inverse scale.
pub fn fn_00c6d4b0(e: &mut Engine, this: Ptr<Ctl>, height: f32) {
    let scale = e.global::<f32>(INVERSE_SCALE);
    e.set(this, Ctl::eWantState, 1);
    e.set(
        this,
        Ctl::fJumpHeight,
        (scale as f64 * height as f64) as f32,
    );
}

// Translated from 00c6d4d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c6d4d0 - the /GS cookie check is not modeled.
/// `bhkCharacterController::SetVelocityModifier` (Xbox PDB): scales `velocity` (xyz, w 0)
/// by the inverse scale and by `1 / time`; when that is longer than `VelocityMod` (squared
/// lengths, single precision; NaN never), it becomes `VelocityMod` and `time`
/// `fVelocityTime`.
pub fn bhk_character_controller_set_velocity_modifier(
    e: &mut Engine,
    this: Ptr<Ctl>,
    velocity: Ptr,
    time: f32,
) {
    let scale = e.global::<f32>(INVERSE_SCALE) as f64;
    let v = load4(e, velocity.addr());
    let scaled = [
        (v[0] as f64 * scale) as f32,
        (v[1] as f64 * scale) as f32,
        (scale * v[2] as f64) as f32,
        0.0,
    ];
    let inverse = (1.0 / x87_quiet(time) as f64) as f32;
    let modifier = scaled.map(|lane| inverse * lane);
    let new_length = sum_of_squares3(modifier);
    let old_length = sum_of_squares3(load4(e, this.addr() + Ctl::VelocityMod.off));
    if old_length < new_length {
        e.set(this, Ctl::fVelocityTime, x87_quiet(time));
        store4(e, this.addr() + Ctl::VelocityMod.off, modifier);
    }
}

// Translated from 00c6d5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCharacterController::ApplyVelocityMod` (Xbox PDB): when `fVelocityTime > 0` and
/// neither flag 0x800 nor 0x1000 is set, `OutVelocity += fVelocityTime * VelocityMod`
/// (single precision, all four lanes); when `fVelocityTime` is not above 0 (or NaN),
/// `VelocityMod` becomes the zero vector at `01267e30`.
#[allow(clippy::neg_cmp_op_on_partial_ord)] // the x87 compare treats NaN as "not above 0"
pub fn bhk_character_controller_apply_velocity_mod(e: &mut Engine, this: Ptr<Ctl>) {
    let time = e.get(this, Ctl::fVelocityTime);
    if !(0.0 < time) {
        copy_words(e, this.addr() + Ctl::VelocityMod.off, ZERO_VECTOR, 4);
        return;
    }
    if e.get(this, Ctl::iFlags) & 0x1800 != 0 {
        return;
    }
    let modifier = load4(e, this.addr() + Ctl::VelocityMod.off);
    let velocity = load4(e, this.addr() + Ctl::OutVelocity.off);
    let out: [f32; 4] = std::array::from_fn(|lane| time * modifier[lane] + velocity[lane]);
    store4(e, this.addr() + Ctl::OutVelocity.off, out);
}

// Translated from 00c6d620 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c6d620 - the /GS cookie check is not modeled.
/// cdecl `(source, transform, out)`: `out = transform * (source[+0x20], source[+0x24],
/// source[+0x10] + source[+0x28], source[+0x2C])` through `hkVector4::setTransformedPos`.
pub fn fn_00c6d620(e: &mut Engine, source: Ptr, transform: u32, out: u32) {
    let s = source.addr();
    e.with_stack(0x10, |e, point| {
        copy_words(e, point.addr(), s + 0x20, 4);
        let z = (e.mem.f32(s + 0x10) as f64 + e.mem.f32(s + 0x28) as f64) as f32;
        e.mem.set_f32(point.addr() + 8, z);
        e.call(SET_TRANSFORMED_POS, &args![out, transform, point]);
    });
}

// Translated from 00c6d670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the 0x34 bytes at `source` to `this` (the floats at `+0x20..+0x2C` through x87
/// loads and stores); returns `this`.
pub fn fn_00c6d670(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let (t, s) = (this.addr(), source.addr());
    copy_words(e, t, s, 8);
    for o in [0x20, 0x24, 0x28, 0x2c] {
        let w = x87_quiet_bits(e.mem.u32(s + o));
        e.mem.set_u32(t + o, w);
    }
    let w = e.mem.u32(s + 0x30);
    e.mem.set_u32(t + 0x30, w);
    this
}

// Translated from 00c6d6b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// With a `reference`: `fLODDistance` = the distance from `point` (xyz) to the vector at
/// the reference's `+0x20` (`011f4288` when it has none), minus that vector's `w`, or 0
/// when that is not above 0 (NaN included). The square root is the CRT helper `00ec6040`.
pub fn fn_00c6d6b0(e: &mut Engine, this: Ptr<Ctl>, point: Ptr, reference: u32) {
    if reference == 0 {
        return;
    }
    let vector = e.mem.u32(reference + 0x20);
    let base = if vector != 0 { vector } else { POINT_FALLBACK };
    let p = load4(e, point.addr());
    let b = load4(e, base);
    let d: [f32; 3] = std::array::from_fn(|lane| (p[lane] as f64 - b[lane] as f64) as f32);
    let [dx, dy, dz] = d.map(|v| v as f64);
    let sum = (dz * dz + (dx * dx + dy * dy)) as f32;
    let root = e.call(X87_SQRT, &args![sum as f64]).f32();
    let rest = (root as f64 - e.mem.f32(base + 0xc) as f64) as f32;
    let distance = if rest > 0.0 { rest } else { 0.0 };
    e.set(this, Ctl::fLODDistance, distance);
}

// Translated from 00c6d770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Passes `value` to `00c85ce0` on the shape phantom and on the collision box, each when
/// present.
pub fn fn_00c6d770(e: &mut Engine, this: Ptr<Ctl>, value: u32) {
    let phantom = e.get(this, Ctl::spShapePhantom);
    if phantom != 0 {
        e.call(WORLD_OBJECT_PASS, &args![phantom, value]);
    }
    let collision_box = e.get(this, Ctl::spCollisionBox);
    if collision_box != 0 {
        e.call(WORLD_OBJECT_PASS, &args![collision_box, value]);
    }
}

// Translated from 00c6d7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCharacterController::SetNiObject` (Xbox PDB): `SetAVObject(object, 0)` on the
/// shape phantom and on the collision box, each when present.
pub fn bhk_character_controller_set_ni_object(e: &mut Engine, this: Ptr<Ctl>, object: u32) {
    let phantom = e.get(this, Ctl::spShapePhantom);
    if phantom != 0 {
        e.call(SET_AV_OBJECT, &args![phantom, object, 0u32]);
    }
    let collision_box = e.get(this, Ctl::spCollisionBox);
    if collision_box != 0 {
        e.call(SET_AV_OBJECT, &args![collision_box, object, 0u32]);
    }
}

// Translated from 00c6d7e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// cdecl layer filter. With `check` 0 returns 1 when `fallback` is not 0, else 0. With
/// `check` set and a `shape` whose `+8` is not null: the layer is `00c84f10(layer)` on
/// that Havok object, or its `+0x10` low five bits when `layer` is -1; the result is 1
/// when the callback at `01267bc8` is null or returns false for it, else 0. Without the
/// shape it returns `check` unchanged (its byte).
pub fn fn_00c6d7e0(e: &mut Engine, shape: u32, check: u8, fallback: u8, layer: i32) -> u8 {
    if check == 0 {
        return u8::from(fallback != 0);
    }
    if shape == 0 {
        return check;
    }
    let hk = e.mem.u32(shape + 8);
    if hk == 0 {
        return check;
    }
    let number = if layer == -1 {
        e.mem.u32(hk + 0x10) & 0x1f
    } else {
        e.call(SHAPE_LAYER, &args![hk, layer]).u32()
    };
    let callback: u32 = e.global(LAYER_CALLBACK);
    let blocked = if callback != 0 {
        e.call(callback, &args![number]).u8()
    } else {
        0
    };
    u8::from(blocked == 0)
}

// Translated from 00c6d840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldObject::getProperty` (Xbox PDB): writes the 8-byte value of the first property
/// with `key` (array `m_properties` at `+0x78`, size `+0x7C`, 16-byte entries: key, pad,
/// value) to `out`, or zeros. Returns `out`. The third stack word is never read.
pub fn hkp_world_object_get_property(
    e: &mut Engine,
    this: Ptr,
    out: Ptr,
    key: u32,
    _unused_3: u32,
) -> Ptr {
    let count = e.mem.i32(this.addr() + 0x7c);
    if count > 0 {
        let data = e.mem.u32(this.addr() + 0x78);
        let mut index = 0;
        while index < count {
            let entry = data.wrapping_add((index as u32) << 4);
            if e.mem.u32(entry) == key {
                let low = e.mem.u32(entry + 8);
                let high = e.mem.u32(entry + 0xc);
                e.mem.set_u32(out.addr() + 4, high);
                e.mem.set_u32(out.addr(), low);
                return out;
            }
            index += 1;
        }
    }
    e.mem.set_u32(out.addr(), 0);
    e.mem.set_u32(out.addr() + 4, 0);
    out
}

/// Whether the Havok world object `hk` has a property with `key`.
fn has_property(e: &Engine, hk: u32, key: u32) -> bool {
    let count = e.mem.i32(hk + 0x7c);
    let data = e.mem.u32(hk + 0x78);
    let mut index = 0;
    while index < count {
        if e.mem.u32(data.wrapping_add((index as u32) << 4)) == key {
            return true;
        }
        index += 1;
    }
    false
}

// Translated from 00c6d8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the world object's Havok object (`+8`) has a property with `key` (false
/// without one); the empty lock call brackets the search.
pub fn fn_00c6d8a0(e: &mut Engine, this: Ptr, key: u32) -> bool {
    let hk = e.mem.u32(this.addr() + 8);
    if hk == 0 {
        return false;
    }
    e.call(EMPTY_LOCK, &args![this]);
    let found = has_property(e, hk, key);
    e.call(EMPTY_LOCK, &args![this]);
    found
}

// Translated from 00c6d900 (decompiled, FalloutNV.exe 1.4.0.525)
/// stdcall `(object)`: when the `hkpWorldObject` has property 0x1300, the word at
/// `+0xC0` of the object its value points to; else 0.
pub fn fn_00c6d900(e: &mut Engine, object: u32) -> u32 {
    if object == 0 || !has_property(e, object, PROPERTY_KEY) {
        return 0;
    }
    e.with_stack(8, |e, out| {
        hkp_world_object_get_property(e, Ptr::new(object), out, PROPERTY_KEY, 0);
        let target = e.mem.u32(out.addr());
        e.mem.u32(target.wrapping_add(0xc0))
    })
}

// Translated from 00c6d960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCharacterController::GetShape` (Xbox PDB): `spShapes[eShapeType]` when the type is
/// below 2 (signed), else null. The `NiPointer` release the compiler left behind runs on
/// a null pointer only and is left out.
pub fn bhk_character_controller_get_shape(e: &mut Engine, this: Ptr<Ctl>) -> u32 {
    let kind = e.get(this, Ctl::eShapeType);
    if kind < 2 {
        let slot = (this.addr() + Ctl::spShapes.off).wrapping_add((kind as u32).wrapping_mul(4));
        e.mem.u32(slot)
    } else {
        0
    }
}

// Translated from 00c6d9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpClosestCdPointCollector::_scalar_deleting_destructor_` (Xbox PDB): sets the
/// `hkpCdPointCollector` vtable and, when bit 0 of `flags` is set, frees the 0x80 bytes
/// through the Havok allocator.
pub fn hkp_closest_cd_point_collector_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u8,
) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE_CD_POINT_COLLECTOR);
    if flags & 1 != 0 {
        havok_free(e, this.addr(), 0x80);
    }
    this
}

// Translated from 00c6d9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the first word and fills the next nine with three copies of the point at
/// `011f426c`; returns `this`.
pub fn fn_00c6d9f0(e: &mut Engine, this: Ptr) -> Ptr {
    let me = this.addr();
    e.mem.set_u32(me, 0);
    for copy in 0..3 {
        copy_words(e, me + 4 + 12 * copy, POINT_DEFAULT, 3);
    }
    this
}

/// A Havok object of `bytes` bytes from the current allocator (slot 4), its `+4`
/// half-word set to the size, as the inlined `hkReferencedObject` operator new does.
fn havok_new(e: &mut Engine, bytes: u16) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![u32::from(bytes)]).u32();
    e.mem.set_u16(block.wrapping_add(4), bytes);
    block
}

// Translated from 00c6da50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c6da50 - the FS:[0] exception-unwinding frame is not modeled.
/// Constructs the 0xC0-byte cinfo `00c6df60` makes: the `hkpCharacterProxyCinfo` defaults
/// (`00c6cde0`), then the zero vector at `01267e30` for position and velocity, the
/// frictions 1 and 0, extra frictions 0, no phantom, contact-angle sensitivity 100 (the
/// float at `01016410`), 4 user planes; `+0x80..+0x88` cleared, `+0x84` = the next
/// value of the 16-bit counter at `011b0808` (raised by one, 10 after a wrap)
/// shifted up 16 bits and ORed with 0x1E, `+0x8C` 0, `+0x90` 1, bytes `+0x95..+0x98` 0,
/// `+0x9C..+0xA4` 0, `+0xA8` 31 times the inverse scale, `+0xAC` 1, `+0xB0` 1, `+0xB4` 0.
/// With `with_states` set, `+0xA4` holds a reference to the shared object at
/// `01267bb8` (`00cbc460` on a 0x34-byte Havok object, made on first use) with seven
/// states registered (`00cbc4e0(state, slot)`, each state's own reference then dropped):
/// slots 0 (`00cd47e0`), 5 (`00cd44c0`), 1 (`00cd4260`), 2 (`00cd3f90`), 3 (a
/// `bhkCharacterStateClimbing`), 4 (a `bhkCharacterStateFlying`) and 6 (`00cd3c90`). The
/// allocation-scope word is 0x10 from the counter update on. Returns `this`.
pub fn fn_00c6da50(e: &mut Engine, this: Ptr, with_states: u8) -> Ptr {
    let me = this.addr();
    fn_00c6cde0(e, this);
    e.mem.set_u32(me + 0x80, 0);
    e.mem.set_u32(me + 0x84, 0);
    e.mem.set_f32(me + 0x8c, 0.0);
    e.mem.set_u32(me + 0x88, 0);
    e.mem.set_f32(me + 0x90, 1.0);
    for byte in 0x95..=0x98 {
        e.mem.set_u8(me + byte, 0);
    }
    for word in [0x9c, 0xa0, 0xa4] {
        e.mem.set_u32(me + word, 0);
    }
    let thirty_one: f64 = e.global(THIRTY_ONE);
    let wall = (e.global::<f32>(INVERSE_SCALE) as f64 * thirty_one) as f32;
    e.mem.set_f32(me + 0xa8, wall);
    e.mem.set_u32(me + 0xb0, 1);
    e.mem.set_u32(me + 0xb4, 0);
    e.mem.set_f32(me + 0xac, 1.0);
    let (word, saved) = enter_scope(e);
    let mut group = e.global::<u32>(FILTER_COUNTER).wrapping_add(1) & 0xffff;
    e.set_global(FILTER_COUNTER, group);
    if group == 0 {
        group = 10;
        e.set_global(FILTER_COUNTER, group);
    }
    let hundred = x87_quiet_bits(e.global::<u32>(FLOAT_HUNDRED));
    e.mem.set_u32(me + 0x58, hundred);
    e.mem.set_u32(me + 0x84, (group << 16) | 0x1e);
    e.mem.set_u32(me + 0x54, 4);
    copy_words(e, me, ZERO_VECTOR, 4);
    e.mem.set_f32(me + 0x24, 0.0);
    e.mem.set_f32(me + 0x40, 0.0);
    copy_words(e, me + 0x10, ZERO_VECTOR, 4);
    e.mem.set_f32(me + 0x44, 0.0);
    e.mem.set_u32(me + 0x48, 0);
    e.mem.set_f32(me + 0x20, 1.0);
    if with_states != 0 {
        if e.global::<u32>(STATIC_REFERENCED_OBJECT) == 0 {
            let block = havok_new(e, 0x34);
            let manager = e.call(STATE_MANAGER_CONSTRUCT, &args![block]).u32();
            e.set_global(STATIC_REFERENCED_OBJECT, manager);
            for (size, construct, slot) in [
                (0xc, STATE_CONSTRUCT_0, 0u32),
                (8, STATE_CONSTRUCT_5, 5),
                (8, STATE_CONSTRUCT_1, 1),
                (0xc, STATE_CONSTRUCT_2, 2),
            ] {
                let block = havok_new(e, size);
                let state = e.call(construct, &args![block]).u32();
                register_state(e, state, slot);
            }
            for (vtable, slot) in [(VTABLE_STATE_CLIMBING, 3u32), (VTABLE_STATE_FLYING, 4)] {
                let state = havok_new(e, 8);
                e.mem.set_u16(state.wrapping_add(6), 1);
                e.mem.set_u32(state, vtable);
                register_state(e, state, slot);
            }
            let block = havok_new(e, 8);
            let state = e.call(STATE_CONSTRUCT_6, &args![block]).u32();
            register_state(e, state, 6);
        }
        let manager: u32 = e.global(STATIC_REFERENCED_OBJECT);
        if manager != 0 {
            e.call(ADD_REFERENCE, &args![manager]);
        }
        let old = e.mem.u32(me + 0xa4);
        if old != 0 {
            e.call(REMOVE_REFERENCE, &args![old]);
        }
        e.mem.set_u32(me + 0xa4, manager);
    }
    e.mem.set_u32(word, saved);
    this
}

/// `00cbc4e0(state, slot)` on the shared state manager, then the state's own reference
/// dropped.
fn register_state(e: &mut Engine, state: u32, slot: u32) {
    let manager: u32 = e.global(STATIC_REFERENCED_OBJECT);
    e.call(STATE_REGISTER, &args![manager, state, slot]);
    e.call(REMOVE_REFERENCE, &args![state]);
}

// Translated from 00c6de10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c6de10 - the FS:[0] exception-unwinding frame is not modeled.
/// Member tear-down: releases the `hkRefPtr` at `+0xA4` twice over (the second sees the
/// cleared pointer), then the `NiPointer` at `+0xA0` (not cleared).
pub fn fn_00c6de10(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    for _ in 0..2 {
        let object = e.mem.u32(me + 0xa4);
        if object != 0 {
            e.call(REMOVE_REFERENCE, &args![object]);
        }
        e.mem.set_u32(me + 0xa4, 0);
    }
    let object = e.mem.u32(me + 0xa0);
    if object != 0 {
        release_ni_object(e, object);
    }
}

// Translated from 00c6dec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCharacterController::FindBSReference` (Xbox PDB), cdecl `(key, object)`: the low
/// word of the `hkpWorldObject`'s property `key`, 0 without an object.
pub fn bhk_character_controller_find_bs_reference(e: &mut Engine, key: u32, object: u32) -> u32 {
    if object == 0 {
        return 0;
    }
    e.with_stack(8, |e, out| {
        hkp_world_object_get_property(e, Ptr::new(object), out, key, 0);
        e.mem.u32(out.addr())
    })
}

// Translated from 00c6def0 (decompiled, FalloutNV.exe 1.4.0.525)
/// cdecl `(body)`: true when the body's motion-type byte (`+0xE8`) passes `00c8cce0` and its
/// mass (`hkpMotion::getMass` on `+0xE0`) is above the `float` at `01267bc0`, or when the
/// low seven bits of `+0x2C` are 6; false for a null body.
pub fn fn_00c6def0(e: &mut Engine, body: u32) -> bool {
    if body == 0 {
        return false;
    }
    let motion_type = u32::from(e.mem.u8(body + 0xe8));
    let passes = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), motion_type);
        e.call(MOTION_TYPE_TEST, &args![slot]).bool()
    });
    if passes {
        let mass = x87_quiet(e.call(GET_MASS, &args![body + 0xe0]).f32());
        let limit: f32 = e.global(BLOCKING_MASS);
        if limit < mass {
            return true;
        }
    }
    e.mem.u32(body + 0x2c) & 0x7f == 6
}

// Translated from 00c6df60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c6df60 - the FS:[0] exception-unwinding frame is not modeled.
/// Returns the cinfo at `+0xC`, creating it when there is none: `*created` says whether
/// it was missing; a new one is 0xC0 bytes from the Havok allocator, constructed by
/// `00c6da50(1)` (null when the allocation failed), stored, and handed to `00cabfd0` when
/// the Havok object (`+8`) exists. The allocation-scope word is 0x10 meanwhile.
pub fn fn_00c6df60(e: &mut Engine, this: Ptr, created: Ptr) -> u32 {
    let me = this.addr();
    let (word, saved) = enter_scope(e);
    let missing = e.mem.u32(me + 0xc) == 0;
    e.mem.set_u8(created.addr(), u8::from(missing));
    if missing {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let block = e.vcall(allocator, 4, &args![0xc0u32]).u32();
        let info = if block != 0 {
            fn_00c6da50(e, Ptr::new(block), 1).addr()
        } else {
            0
        };
        e.mem.set_u32(me + 0xc, info);
        if e.mem.u32(me + 8) != 0 {
            e.call(SET_PROXY_CINFO, &args![this, info]);
        }
    }
    let info = e.mem.u32(me + 0xc);
    e.mem.set_u32(word, saved);
    info
}

// Translated from 00c6e030 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the proxy and its Havok proxy exist, clears the word at `+4` of the Havok
/// proxy's part that `00cac900` returns (`+0x74`).
pub fn fn_00c6e030(e: &mut Engine, this: Ptr) {
    if this.addr() == 0 {
        return;
    }
    let proxy = e.mem.u32(this.addr() + 8);
    if proxy != 0 {
        let part = e.call(PROXY_PART_74, &args![proxy]).u32();
        e.mem.set_u32(part.wrapping_add(4), 0);
    }
}

// Translated from 00c6e050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds property `key` = `value` (sign-extended to 64 bits) to the shape phantom's Havok
/// object when the phantom exists, does not have that property yet, and has its Havok
/// object; the empty lock call brackets it.
pub fn fn_00c6e050(e: &mut Engine, this: Ptr<Ctl>, key: u32, value: i32) {
    let phantom = e.get(this, Ctl::spShapePhantom);
    if phantom == 0 || fn_00c6d8a0(e, Ptr::new(phantom), key) {
        return;
    }
    let hk = e.mem.u32(phantom + 8);
    if hk != 0 {
        e.call(EMPTY_LOCK, &args![phantom]);
        let high = (value >> 31) as u32;
        e.call(ADD_PROPERTY, &args![hk, key, value as u32, high]);
        e.call(EMPTY_LOCK, &args![phantom]);
    }
}

// Translated from 00c6e0b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCharacterController::FindDisplayGeometry` (Xbox PDB): with flag 0x8000 set, looks
/// for the child named `"bhkColDisp"` of the first child (an `NiNode`) of the phantom's
/// scene object (`GetAVObject(0)`, then its slot 0xC); returns it, or clears flag 0x8000
/// and returns null when anything along the way is missing. Without the flag returns
/// null.
pub fn bhk_character_controller_find_display_geometry(e: &mut Engine, this: Ptr<Ctl>) -> u32 {
    if e.get(this, Ctl::iFlags) & 0x8000 == 0 {
        return 0;
    }
    let found = find_display_child(e, this);
    if found == 0 {
        let flags = e.get(this, Ctl::iFlags);
        e.set(this, Ctl::iFlags, flags & 0xffff_7fff);
    }
    found
}

fn find_display_child(e: &mut Engine, this: Ptr<Ctl>) -> u32 {
    let phantom = e.get(this, Ctl::spShapePhantom);
    if phantom == 0 {
        return 0;
    }
    let object = e.call(GET_AV_OBJECT, &args![phantom, 0u32]).u32();
    if object == 0 {
        return 0;
    }
    let root = e.vcall(object, 0xc, &args![]).u32();
    if root == 0 || e.mem.u16(root + 0xa6) == 0 {
        return 0;
    }
    let first = e.mem.u32(e.mem.u32(root + 0xa0));
    if first == 0 {
        return 0;
    }
    let node = e.call(IS_KIND_OF, &args![first, NODE_RTTI]).u32();
    if node == 0 {
        return 0;
    }
    let name = e.call(FIXED_STRING_CREATE, &args![DISPLAY_NAME]).u32();
    let count = u32::from(e.mem.u16(node + 0xa6));
    let mut found = 0;
    let mut index = 0;
    while index < count && found == 0 {
        let child = e.mem.u32(e.mem.u32(node + 0xa0).wrapping_add(index * 4));
        if child != 0 && e.mem.u32(child + 8) == name {
            found = child;
        }
        index += 1;
    }
    release_fixed_string(e, name);
    found
}

// Translated from 00c6e180 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c6e180 - the /GS cookie check is not modeled.
/// `bhkCharacterController::SetRadius` (Xbox PDB): when `spShapes[0]` is a
/// `bhkCapsuleShape`, moves its Havok capsule's two vertices (`+0x20`, `+0x30`) apart along
/// z by the change of radius (`old - radius` is added to the first's z and subtracted
/// from the second's), copies the old radius into both `w` lanes and stores `radius`
/// (`+0x10`). The old radius is 0.1 (`010c72bc`) and the vertices the zero vector when the
/// capsule has no Havok shape.
pub fn bhk_character_controller_set_radius(e: &mut Engine, this: Ptr<Ctl>, radius: f32) {
    let shape = e.get(this, Ctl::spShapes);
    if shape == 0 {
        return;
    }
    let capsule = e.call(IS_KIND_OF, &args![shape, CAPSULE_SHAPE_RTTI]).u32();
    if capsule == 0 {
        return;
    }
    let hk = e.mem.u32(capsule + 8);
    let old = if hk != 0 {
        x87_quiet(e.mem.f32(hk + 0x10))
    } else {
        x87_quiet(e.global::<f32>(DEFAULT_RADIUS))
    };
    let radius = x87_quiet(radius);
    let change = (old as f64 - radius as f64) as f32;
    let first_source = if hk != 0 { hk + 0x20 } else { ZERO_VECTOR };
    let mut first = load4(e, first_source);
    let second_source = if hk != 0 { hk + 0x30 } else { ZERO_VECTOR };
    let mut second = load4(e, second_source);
    first[2] = (x87_quiet(first[2]) as f64 + change as f64) as f32;
    second[2] = (x87_quiet(second[2]) as f64 - change as f64) as f32;
    if hk != 0 {
        let w = x87_quiet(e.mem.f32(hk + 0x10));
        store4(e, hk + 0x20, first);
        e.mem.set_f32(hk + 0x2c, w);
    }
    let hk = e.mem.u32(capsule + 8);
    if hk != 0 {
        let w = x87_quiet(e.mem.f32(hk + 0x10));
        store4(e, hk + 0x30, second);
        e.mem.set_f32(hk + 0x3c, w);
    }
    let hk = e.mem.u32(capsule + 8);
    if hk != 0 {
        e.mem.set_f32(hk + 0x10, radius);
    }
}

// Translated from 00c6e280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCharacterController::GetRadius` (Xbox PDB): the radius (`+0x10`) of the Havok
/// capsule when `spShapes[0]` is a `bhkCapsuleShape` (0.1 from `010c72bc` without its Havok
/// shape), else `fRadius`.
pub fn bhk_character_controller_get_radius(e: &mut Engine, this: Ptr<Ctl>) -> f32 {
    let shape = e.get(this, Ctl::spShapes);
    if shape != 0 {
        let capsule = e.call(IS_KIND_OF, &args![shape, CAPSULE_SHAPE_RTTI]).u32();
        if capsule != 0 {
            let hk = e.mem.u32(capsule + 8);
            return if hk != 0 {
                x87_quiet(e.mem.f32(hk + 0x10))
            } else {
                x87_quiet(e.global::<f32>(DEFAULT_RADIUS))
            };
        }
    }
    x87_quiet(e.get(this, Ctl::fRadius))
}

/// The phantom's Havok object (`spShapePhantom + 8`), or 0 without a phantom.
fn phantom_object(e: &Engine, this: Ptr<Ctl>) -> u32 {
    let phantom = e.get(this, Ctl::spShapePhantom);
    if phantom != 0 {
        e.mem.u32(phantom + 8)
    } else {
        0
    }
}

// Translated from 00c6e300 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c6e300 - the /GS cookie check is not modeled.
/// `bhkCharacterController::GetPosition_ov2` (Xbox PDB): `*out` = the Havok proxy's
/// position (`getPosition` between two `UnMarkRead` calls; the zero vector at `01267e30`
/// without a proxy) minus `RotCenter` rotated by the phantom's transform (`+0xB0`), four
/// lanes in single precision.
pub fn bhk_character_controller_get_position_ov2(e: &mut Engine, this: Ptr<Ctl>, out: Ptr) {
    let hk = phantom_object(e, this);
    e.with_stack(0x10, |e, rotated| {
        e.call(
            SET_ROTATED_DIR,
            &args![
                rotated,
                hk.wrapping_add(0xb0),
                this.addr() + Ctl::RotCenter.off
            ],
        );
        let proxy = e.get(this, Ctl::phkObject);
        let mut base = ZERO_VECTOR;
        if proxy != 0 {
            e.call(UN_MARK_READ, &args![this]);
            base = e.call(HKP_GET_POSITION, &args![proxy]).u32();
            e.call(UN_MARK_READ, &args![this]);
        }
        let position = load4(e, base);
        let offset = load4(e, rotated.addr());
        let result: [f32; 4] = std::array::from_fn(|lane| position[lane] - offset[lane]);
        store4(e, out.addr(), result);
    });
}

// Translated from 00c6e390 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c6e390 - the /GS cookie check is not modeled.
/// `bhkCharacterController::SetPosition_ov2` (Xbox PDB): with a Havok proxy, sets its
/// position (`hkpCharacterProxy::setPosition`, between two `UnMarkWrite` calls) to
/// `position` plus `RotCenter` rotated by the phantom's transform (single precision) and
/// zeroes the 16 bytes at the Havok proxy's `+0x20`.
pub fn bhk_character_controller_set_position_ov2(e: &mut Engine, this: Ptr<Ctl>, position: Ptr) {
    let hk = phantom_object(e, this);
    e.with_stack(0x20, |e, frame| {
        let rotated = frame.addr();
        e.call(
            SET_ROTATED_DIR,
            &args![
                rotated,
                hk.wrapping_add(0xb0),
                this.addr() + Ctl::RotCenter.off
            ],
        );
        let p = load4(e, position.addr());
        let offset = load4(e, rotated);
        let target: [f32; 4] = std::array::from_fn(|lane| p[lane] + offset[lane]);
        store4(e, rotated + 0x10, target);
        let proxy = e.get(this, Ctl::phkObject);
        if proxy != 0 {
            e.call(UN_MARK_WRITE, &args![this]);
            e.call(HKP_SET_POSITION, &args![proxy, rotated + 0x10]);
            copy_zero(e, proxy + 0x20, 4);
            e.call(UN_MARK_WRITE, &args![this]);
        }
    });
}

// Translated from 00c6e2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds the step's delta time (`+0x4E8`) to `fFallTime`.
pub fn fn_00c6e2e0(e: &mut Engine, this: Ptr<Ctl>) {
    let sum = (e.get(this, Ctl::StepDeltaTime) as f64 + e.get(this, Ctl::fFallTime) as f64) as f32;
    e.set(this, Ctl::fFallTime, sum);
}

// Translated from 00c6e420 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c6e420 - the /GS cookie check is not modeled.
/// `bhkCharacterController::ApplyPitchAndRoll` (Xbox PDB): builds the phantom's new
/// transform from its current one (`+0xB0`, 0x40 bytes) with the rotation set from a
/// prepared copy of `rotation` (`005611c0`, `00cb2d90`); when `fPitchAngle` is not 0 (NaN
/// counts) and flag bit 25 is set, multiplies it (`hkMatrix3::mul`) by the rows
/// `(1,0,0) (0,c,-s) (0,s,c)`, and when `fRollAngle` is not 0 and bit 26 is set by
/// `(c,0,s) (0,1,0) (-s,0,c)`, with `c`/`s` the results of `00ec9f70`/`00eca0a0` on the
/// angle; then sets it on the Havok phantom (`setTransform`) when there is one.
pub fn bhk_character_controller_apply_pitch_and_roll(
    e: &mut Engine,
    this: Ptr<Ctl>,
    rotation: Ptr,
) {
    e.with_stack(0x90, |e, frame| {
        let at = |o: u32| frame.addr() + o;
        copy_words(e, at(0x10), rotation.addr(), 4);
        e.call(QUATERNION_PREPARE, &args![at(0x10)]);
        let phantom = e.get(this, Ctl::spShapePhantom);
        let hk = if phantom != 0 {
            e.mem.u32(phantom + 8)
        } else {
            0
        };
        copy_words(e, at(0x50), hk.wrapping_add(0xb0), 16);
        e.call(ROTATION_SET, &args![at(0x50), at(0x10)]);
        let zero: f32 = e.global(FLOAT_ZERO);
        let one: f32 = e.global(FLOAT_ONE);
        let negative_zero: f32 = e.global(FLOAT_NEGATIVE_ZERO);

        let pitch = e.get(this, Ctl::fPitchAngle);
        if pitch != 0.0 && (e.get(this, Ctl::iFlags) >> 0x19) & 1 != 0 {
            let angle = x87_quiet(pitch) as f64;
            let c = x87_quiet(x87_quiet(e.call(X87_FIRST_TRIG, &args![angle]).f32()));
            let s = x87_quiet(x87_quiet(e.call(X87_SECOND_TRIG, &args![angle]).f32()));
            store4(e, at(0x20), [one, zero, zero, zero]);
            store4(e, at(0x30), [zero, c, negative_zero - s, zero]);
            store4(e, at(0x40), [zero, s, c, zero]);
            e.call(MATRIX3_MUL, &args![at(0x50), at(0x20)]);
        }
        let roll = e.get(this, Ctl::fRollAngle);
        if roll != 0.0 && (e.get(this, Ctl::iFlags) >> 0x1a) & 1 != 0 {
            let angle = x87_quiet(roll) as f64;
            let c = x87_quiet(x87_quiet(e.call(X87_FIRST_TRIG, &args![angle]).f32()));
            let s = x87_quiet(x87_quiet(e.call(X87_SECOND_TRIG, &args![angle]).f32()));
            store4(e, at(0x20), [c, zero, s, zero]);
            store4(e, at(0x30), [zero, one, zero, zero]);
            store4(e, at(0x40), [negative_zero - s, zero, c, zero]);
            e.call(MATRIX3_MUL, &args![at(0x50), at(0x20)]);
        }
        if phantom != 0 {
            let hk = e.mem.u32(phantom + 8);
            if hk != 0 {
                e.call(EMPTY_LOCK, &args![phantom]);
                e.call(PHANTOM_SET_TRANSFORM, &args![hk, at(0x50)]);
                e.call(EMPTY_LOCK, &args![phantom]);
            }
        }
    });
}

// Translated from 00c6e680 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c6e680 - the FS:[0] exception-unwinding frame and the /GS cookie check are not modeled.
/// Ground probe under `FrontBack[index]`: `*out` = that point lowered by 25, then a ray
/// from the point to `*out` (filter info from `0070c440`) is cast through vtable slot 0xC8
/// of the object at `+0x350` of the phantom's world, with an `hkpClosestRayHitCollector`
/// on the stack; on a hit `out.z += (1 - hitFraction) * 25`. The stack block keeps the
/// exe's layout (input at `+0x14`, collector at `+0xC4`).
pub fn fn_00c6e680(e: &mut Engine, this: Ptr<Ctl>, index: u32, out: Ptr) {
    let me = this.addr();
    let out = out.addr();
    e.with_stack(0x130, |e, frame| {
        let at = |o: u32| frame.addr() + o;
        let one = 0x3f80_0000u32;
        e.mem.set_u32(at(0x54), one);
        e.mem.set_u32(at(0xe4), one);
        e.mem.set_u32(at(0xc8), one);
        e.mem.set_u8(at(0x34), 0);
        e.mem.set_u32(at(0x38), 0);
        e.mem.set_u32(at(0x58), u32::MAX);
        e.mem.set_u32(at(0x84), 0);
        e.mem.set_u32(at(0x64), u32::MAX);
        e.mem.set_u32(at(0x94), 0);
        e.mem.set_u32(at(0xb4), 0);
        e.mem.set_u8(at(0xc0), 0);
        e.mem.set_u32(at(0xc4), VTABLE_CLOSEST_RAY_HIT_COLLECTOR);
        e.mem.set_u32(at(0x124), 0);
        e.mem.set_u32(at(0xe8), u32::MAX);
        e.mem.set_u32(at(0x114), 0);
        e.mem.set_u32(at(0xf4), u32::MAX);
        let point = me.wrapping_add(index.wrapping_add(0x5b).wrapping_shl(4));
        copy_words(e, out, point, 4);
        copy_words(e, at(0x14), point, 4);
        let depth: f64 = e.global(PROBE_DEPTH);
        let lowered = (e.mem.f32(out + 8) as f64 - depth) as f32;
        e.mem.set_f32(out + 8, lowered);
        copy_words(e, at(0xa4), ZERO_VECTOR, 4);
        e.mem.set_u32(at(0xb8), at(0xc4));
        e.mem.set_u32(at(0xbc), 0);
        copy_words(e, at(0x24), out, 4);
        e.call(RAY_FILTER_INFO, &args![this, at(0x10)]);
        let filter = e.mem.u32(at(0x10));
        e.mem.set_u32(at(0x38), filter);
        let proxy = e.get(this, Ctl::phkObject);
        let phantom = if proxy != 0 {
            e.call(GET_SHAPE_PHANTOM, &args![proxy]).u32()
        } else {
            0
        };
        let world = e.mem.u32(phantom.wrapping_add(8));
        let caster = if world != 0 {
            e.mem.u32(world + 0x350)
        } else {
            0
        };
        if e.vcall(caster, 0xc8, &args![at(0x14)]).bool() {
            let fraction = e.mem.f32(at(0xe4));
            let rest = (1.0 - fraction as f64) as f32;
            let z = (rest as f64 * depth + e.mem.f32(out + 8) as f64) as f32;
            e.mem.set_f32(out + 8, z);
        }
    });
}

// Translated from 00c6e820 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c6e820 - the /GS cookie check is not modeled.
/// `bhkCharacterController::InitFrontBack` (Xbox PDB): with flag bit 0 set, transforms
/// four probe points by the phantom's transform (`+0xB0`) into `FrontBack[0..4]` and
/// raises each z by `fStepHeight`. With `d = fHeight * 0.5 - fRadius`, `m = d * -1` and
/// `h = fRadius` for a shape of type 9, else `-fRadius * 0.5`, the points are `(0,d,h)`,
/// `(0,m,h)`, `(m,0,h)` and `(d,0,h)` (the zero lanes are the exe's `0.0`).
pub fn bhk_character_controller_init_front_back(e: &mut Engine, this: Ptr<Ctl>) {
    if e.get(this, Ctl::iFlags) & 1 == 0 {
        return;
    }
    let me = this.addr();
    let proxy = e.get(this, Ctl::phkObject);
    let phantom = if proxy != 0 {
        e.call(GET_SHAPE_PHANTOM, &args![proxy]).u32()
    } else {
        0
    };
    let radius = x87_quiet(e.get(this, Ctl::fRadius));
    let shape_type = e.mem.u32(e.mem.u32(phantom.wrapping_add(0x10)) + 0xc);
    let half: f64 = e.global(HALF);
    let depth = (e.get(this, Ctl::fHeight) as f64 * half - radius as f64) as f32;
    let minus_one: f64 = e.global(MINUS_ONE);
    let back = (depth as f64 * minus_one) as f32;
    let side = if shape_type == 9 {
        radius
    } else {
        (-(e.get(this, Ctl::fRadius) as f64) * half) as f32
    };
    let zero: f32 = e.global(FLOAT_ZERO);
    let transform = phantom.wrapping_add(0xb0);
    e.with_stack(0x10, |e, point| {
        let p = point.addr();
        store4(e, p, [zero, depth, side, zero]);
        for i in 0..2 {
            let out = me + Ctl::FrontBack.off + 0x10 * i;
            e.call(SET_TRANSFORMED_POS, &args![out, transform, p]);
            let z = (e.get(this, Ctl::fStepHeight) as f64 + e.mem.f32(out + 8) as f64) as f32;
            e.mem.set_f32(out + 8, z);
            e.mem.set_f32(p + 4, x87_quiet(back));
        }
        store4(e, p, [back, zero, side, zero]);
        for i in 2..4 {
            let out = me + Ctl::FrontBack.off + 0x10 * i;
            e.call(SET_TRANSFORMED_POS, &args![out, transform, p]);
            let z = (e.mem.f32(out + 8) as f64 + e.get(this, Ctl::fStepHeight) as f64) as f32;
            e.mem.set_f32(out + 8, z);
            e.mem.set_f32(p, x87_quiet(depth));
        }
    });
}

// Translated from 00c6e980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Support from the body at `info + 0x48` (nothing without one, or while
/// `eSupportMaterial` is 0xF): from the root of its parent chain (`+0xC` links),
/// `eSupportLayer` = the low seven bits of `+0x1C` and `hkSupportVelocity` = the vector at
/// `root + (signed byte +0x10) + 0x1B0` when the byte at `+0x18` is 1 and that sum is not
/// null (else the zero vector). The material is 0x24, or with `info + 0x4C` = -1 the low
/// five bits of `+0x10` of the body's shape's Havok object, or when the layer is not 0x11
/// slot 0xE0 (with that key) of the object `00c7ffd0` finds for the body. When
/// `eSupportMaterial` is 0x24 or the material is 0xF, the material is stored, flag
/// 0x1000000 follows the layer callback at `01267bc8` (clear without one), and when flag
/// bit 16 is set or the material is 0xF, flag 0x20000 says whether it is 0xF.
pub fn fn_00c6e980(e: &mut Engine, this: Ptr<Ctl>, info: u32) {
    if e.get(this, Ctl::eSupportMaterial) == 0xf {
        return;
    }
    let body = e.mem.u32(info + 0x48);
    if body == 0 {
        return;
    }
    let mut material = 0x24;
    let root = root_of(e, body);
    let layer = e.mem.u32(root + 0x1c) & 0x7f;
    e.set(this, Ctl::eSupportLayer, layer);
    let root = root_of(e, body);
    let mut velocity = ZERO_VECTOR;
    if e.mem.u8(root + 0x18) == 1 {
        let offset = e.mem.u8(root + 0x10) as i8 as i32 as u32;
        let base = offset.wrapping_add(root);
        if base != 0 {
            velocity = base + 0x1b0;
        }
    }
    copy_words(e, this.addr() + Ctl::hkSupportVelocity.off, velocity, 4);
    let key = e.mem.i32(info + 0x4c);
    if key == -1 {
        let shape = e.mem.u32(body);
        if shape != 0 {
            let hk = e.mem.u32(shape + 8);
            if hk != 0 {
                material = e.mem.u32(hk + 0x10) & 0x1f;
            }
        }
    } else if layer != 0x11 {
        let source = e.call(BODY_MATERIAL_SOURCE, &args![body]).u32();
        if source != 0 {
            material = e.vcall(source, 0xe0, &args![key as u32]).u32();
        }
    }
    if e.get(this, Ctl::eSupportMaterial) != 0x24 && material != 0xf {
        return;
    }
    e.set(this, Ctl::eSupportMaterial, material);
    let callback: u32 = e.global(LAYER_CALLBACK);
    let blocked = if callback != 0 {
        e.call(callback, &args![material]).u8()
    } else {
        0
    };
    let flags = e.get(this, Ctl::iFlags);
    let flags = if blocked != 0 {
        flags | 0x0100_0000
    } else {
        flags & 0xfeff_ffff
    };
    e.set(this, Ctl::iFlags, flags);
    if (flags >> 16) & 1 != 0 || material == 0xf {
        let flags = if e.get(this, Ctl::eSupportMaterial) == 0xf {
            flags | 0x2_0000
        } else {
            flags & 0xfffd_ffff
        };
        e.set(this, Ctl::iFlags, flags);
    }
}

/// The last object of the `+0xC` parent chain starting at `object`.
fn root_of(e: &Engine, object: u32) -> u32 {
    let mut root = object;
    loop {
        let parent = e.mem.u32(root + 0xc);
        if parent == 0 {
            return root;
        }
        root = parent;
    }
}

// Translated from 00c6ead0 (decompiled, FalloutNV.exe 1.4.0.525)
/// cdecl `(shape)`: follows a type-10 shape to its `+0x34`, then returns it when its type
/// (`+0xC`) is 0x17, 0x1E or 0x1F, else null.
pub fn fn_00c6ead0(e: &mut Engine, shape: u32) -> u32 {
    if shape == 0 {
        return 0;
    }
    let mut shape = shape;
    let mut kind = e.mem.u32(shape + 0xc);
    if kind == 10 {
        shape = e.mem.u32(shape + 0x34);
        kind = e.mem.u32(shape + 0xc);
    }
    if matches!(kind, 0x17 | 0x1e | 0x1f) {
        shape
    } else {
        0
    }
}

/// The owner of an `hkpCollidable` (`collidable + (signed byte +0x10)`) when its type byte
/// (`+0x18`) is `kind`, else 0.
fn collidable_owner(e: &Engine, collidable: u32, kind: u8) -> u32 {
    if e.mem.u8(collidable + 0x18) == kind {
        let offset = e.mem.u8(collidable + 0x10) as i8 as i32 as u32;
        offset.wrapping_add(collidable)
    } else {
        0
    }
}

// Translated from 00c6eb00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c6eb00 - the /GS cookie check is not modeled.
/// `bhkCharacterController::IsStep` (Xbox PDB) for the contact `point` (position at `+0`,
/// collidables at `+0x20` (a phantom, type 2) and `+0x48` (a body, type 1), shape key at
/// `+0x4C`). True at once when the body's terrain shape (`00c6ead0` logic) has a Havok
/// object whose layer the callback at `01267bc8` reports. Otherwise, with both owners:
/// for a lying controller (flag bit 0) the two foot points `FrontBack[0..2]` are rebuilt
/// from the phantom's capsule vertices (a type-5 shape; else `(0, ±fHeight/2, radius)`)
/// raised by `fStepHeight`, the nearer one to the contact is chosen and gives the
/// position's x and y. The rise from the position (raised by `fStepHeight`) to the contact
/// must not exceed a third of `fStepHeight` (lying controllers measure from the foot and
/// may pass when that is not above 0). The step is then tested: with a shape key and a
/// triangle container, a capsule from the contact along the horizontal direction to the
/// position (`fCastDepth` long) against the triangle (`hkCollideCapsuleUtilCapsVsTri`, both
/// distances at least 0); otherwise a ray cast of the body's shape (slot 0x20, in its
/// local space) that must miss or hit a layer `00c6d7e0` lets through. For a lying
/// controller a step records the contact height (`00c6c720` on the listener).
pub fn bhk_character_controller_is_step(e: &mut Engine, this: Ptr<Ctl>, point: Ptr) -> bool {
    e.with_stack(0x370, |e, frame| {
        is_step(e, this, point.addr(), frame.addr())
    })
}

fn is_step(e: &mut Engine, this: Ptr<Ctl>, point: u32, f: u32) -> bool {
    let collidable_a = e.mem.u32(point + 0x20);
    let phantom = collidable_owner(e, collidable_a, 2);
    let collidable_b = e.mem.u32(point + 0x48);
    let body = collidable_owner(e, collidable_b, 1);
    let mut index = 0;
    let mut result = false;
    if body != 0 {
        let terrain_shape = e.mem.u32(collidable_b);
        let terrain = fn_00c6ead0(e, terrain_shape);
        let hk = hk_of(e, terrain);
        if hk != 0 {
            let key = e.mem.i32(point + 0x4c);
            let layer = if key == -1 {
                e.mem.u32(hk + 0x10) & 0x1f
            } else {
                e.call(SHAPE_LAYER, &args![hk, key as u32]).u32()
            };
            let callback: u32 = e.global(LAYER_CALLBACK);
            if callback != 0 {
                result = e.call(callback, &args![layer]).u8() != 0;
            }
        }
    }
    if !result && phantom != 0 && body != 0 {
        result = step_test(e, this, point, f, phantom, body, &mut index);
    }
    if e.get(this, Ctl::iFlags) & 1 != 0 && result {
        let z = x87_quiet(e.mem.f32(point + 8));
        e.call(
            RECORD_SUPPORT_HEIGHT,
            &args![this.addr() + LISTENER, index, z.to_bits()],
        );
    }
    result
}

fn step_test(
    e: &mut Engine,
    this: Ptr<Ctl>,
    point: u32,
    f: u32,
    phantom: u32,
    body: u32,
    index: &mut u32,
) -> bool {
    let me = this.addr();
    let shape = e.mem.u32(e.mem.u32(point + 0x48));
    let mut radius = x87_quiet(e.get(this, Ctl::fRadius));
    let phantom_shape = e.mem.u32(e.mem.u32(point + 0x20));
    let mut capsule = 0;
    if e.mem.u32(phantom_shape + 0xc) == 5 {
        radius = x87_quiet(e.mem.f32(phantom_shape + 0x10));
        capsule = phantom_shape;
    }
    e.mem.set_f32(f + 0x30, radius);
    copy_words(e, f + 0xb0, point, 4);
    let contact_z = e.mem.f32(point + 8);
    bhk_character_controller_get_position_ov2(e, this, Ptr::new(f + 0x60));
    let zero = f32::from_bits(e.global::<u32>(FLOAT_ZERO));
    if e.get(this, Ctl::iFlags) & 1 != 0 {
        let contact = load4(e, f + 0xb0);
        let mut distances = [0.0f32; 2];
        for i in 0..2u32 {
            let v = if capsule != 0 {
                load4(e, capsule + (i + 2) * 0x10)
            } else {
                let half: f64 = e.global(HALF);
                let reach = (e.get(this, Ctl::fHeight) as f64 * half) as f32;
                let mut v = [zero, reach, radius, zero];
                if i == 1 {
                    let minus_one: f64 = e.global(MINUS_ONE);
                    v[1] = (reach as f64 * minus_one) as f32;
                }
                v
            };
            store4(e, f + 0x50, v);
            let foot = me + Ctl::FrontBack.off + 0x10 * i;
            e.call(SET_TRANSFORMED_POS, &args![foot, phantom + 0xb0, f + 0x50]);
            let raised = (e.mem.f32(foot + 8) as f64 + e.get(this, Ctl::fStepHeight) as f64) as f32;
            e.mem.set_f32(foot + 8, raised);
            let at = load4(e, foot);
            let delta: [f32; 4] = std::array::from_fn(|lane| contact[lane] - at[lane]);
            store4(e, f + 0x80 + 0x10 * i, delta);
            distances[i as usize] = sum_of_squares3(delta);
        }
        *index = if distances[1] > distances[0] { 0 } else { 1 };
        let foot = me + Ctl::FrontBack.off + 0x10 * *index;
        let x = x87_quiet(e.mem.f32(foot));
        e.mem.set_f32(f + 0x60, x);
        let y = x87_quiet(e.mem.f32(foot + 4));
        e.mem.set_f32(f + 0x64, y);
    }
    // The horizontal direction from the contact to the position, and the raised position.
    let position = load4(e, f + 0x60);
    let contact = load4(e, f + 0xb0);
    let mut direction: [f32; 4] = std::array::from_fn(|lane| position[lane] - contact[lane]);
    direction[2] = 0.0;
    let raised = (position[2] as f64 + e.get(this, Ctl::fStepHeight) as f64) as f32;
    e.mem.set_f32(f + 0x68, raised);
    let three: f32 = e.global(FLOAT_THREE);
    let half: f32 = e.global(FLOAT_HALF);
    let inverse = inverse_length(sum_of_squares3(direction), zero, half, three);
    let direction = direction.map(|lane| inverse * lane);
    store4(e, f + 0x50, direction);
    e.mem.set_f32(f + 0xb8, raised);
    let rise = (contact_z as f64 - raised as f64) as f32;
    let mut stepped = rise > 0.0;
    let third = (e.get(this, Ctl::fStepHeight) as f64 / e.global::<f64>(DOUBLE_THREE)) as f32;
    if third < rise {
        if e.get(this, Ctl::iFlags) & 1 == 0 {
            return false;
        }
        let foot_z = e.mem.f32(me + Ctl::FrontBack.off + 0x10 * *index + 8);
        let above =
            ((contact_z as f64 - foot_z as f64) + e.get(this, Ctl::fStepHeight) as f64) as f32;
        if above > 0.0 {
            if third < above {
                return false;
            }
            stepped = true;
        }
    }
    let key = e.mem.i32(point + 0x4c);
    let container = if key != -1 { fn_00c6ead0(e, shape) } else { 0 };
    if container != 0 {
        capsule_against_triangle(e, this, f, body, container, key as u32)
    } else {
        ray_against_body(e, this, f, body, shape, stepped)
    }
}

/// The triangle path of `IsStep`: the triangle `key` of the container (slot 0x14 of its
/// `+0x10`, built into the frame's `+0x160`) in world space by the body's transform rows
/// (`+0xF0..+0x12C`, single precision, vertices last to first), prepared by `00cd72a0`,
/// against the capsule from the contact (raised) to it minus `fCastDepth` along the
/// direction, radius 0.05; true when both distances of the result (`+0x1C`, `+0x3C`) are
/// at least 0.
fn capsule_against_triangle(
    e: &mut Engine,
    this: Ptr<Ctl>,
    f: u32,
    body: u32,
    container: u32,
    key: u32,
) -> bool {
    let triangle = e
        .vcall(container + 0x10, 0x14, &args![key, f + 0x160])
        .u32();
    let rows = [0xf0, 0x100, 0x110, 0x120].map(|o| load4(e, body + o));
    for k in (0..3).rev() {
        let v = load4(e, triangle + 0x20 + 0x10 * k);
        let out: [f32; 4] = std::array::from_fn(|lane| {
            (v[1] * rows[1][lane] + v[0] * rows[0][lane]) + (v[2] * rows[2][lane] + rows[3][lane])
        });
        store4(e, f + 0xc0 + 0x10 * k, out);
    }
    e.call(TRIANGLE_PREPARE, &args![f + 0xc0, f + 0x3c]);
    let contact = load4(e, f + 0xb0);
    store4(e, f + 0x80, contact);
    store4(e, f + 0x90, contact);
    let direction = load4(e, f + 0x50);
    let depth = e.get(this, Ctl::fCastDepth) as f64;
    let x = (contact[0] as f64 - direction[0] as f64 * depth) as f32;
    e.mem.set_f32(f + 0x90, x);
    let y = (contact[1] as f64 - direction[1] as f64 * depth) as f32;
    e.mem.set_f32(f + 0x94, y);
    let triangle_radius = x87_quiet_bits(e.mem.u32(triangle + 0x10));
    let capsule_radius = x87_quiet_bits(e.global::<u32>(FLOAT_TWENTIETH));
    e.call(
        CAPSULE_VS_TRIANGLE,
        &args![
            f + 0x80,
            capsule_radius,
            f + 0xc0,
            triangle_radius,
            f + 0x3c,
            0u32,
            0u32,
            f + 0x100,
            0u32
        ],
    );
    let first = e.mem.f32(f + 0x11c);
    let second = e.mem.f32(f + 0x13c);
    0.0 <= first && 0.0 <= second
}

/// The ray path of `IsStep`: an `hkpShapeRayCastInput` (frame `+0x80`) from the raised
/// position to it minus `(fCastDepth + radius)` along the direction, both in the body's
/// local space (`setTransformedInversePos` with its rows copied to `+0xC0`), cast by the
/// body's shape (slot 0x20, the `hkBool` result into the frame's `+0x1B`, output at
/// `+0x100`: fraction 1, keys -1). No hit is a step; a hit is one when `00c6d7e0` (with
/// the last valid shape key) refuses its layer.
fn ray_against_body(
    e: &mut Engine,
    this: Ptr<Ctl>,
    f: u32,
    body: u32,
    shape: u32,
    stepped: bool,
) -> bool {
    let position = load4(e, f + 0x60);
    let direction = load4(e, f + 0x50);
    e.mem.set_f32(f + 0x110, 1.0);
    store4(e, f + 0x70, position);
    let reach = (e.get(this, Ctl::fCastDepth) as f64 + e.mem.f32(f + 0x30) as f64) as f32;
    e.mem.set_u32(f + 0xa0, 0);
    e.mem.set_u32(f + 0xa4, 0);
    e.mem.set_f32(f + 0x30, reach);
    e.mem.set_u32(f + 0x140, 0);
    copy_words(e, f + 0xc0, body + 0xf0, 16);
    e.mem.set_u32(f + 0x114, u32::MAX);
    e.mem.set_u32(f + 0x120, u32::MAX);
    let x = (position[0] as f64 - direction[0] as f64 * reach as f64) as f32;
    e.mem.set_f32(f + 0x70, x);
    let y = (position[1] as f64 - reach as f64 * direction[1] as f64) as f32;
    e.mem.set_f32(f + 0x74, y);
    e.call(
        SET_TRANSFORMED_INVERSE_POS,
        &args![f + 0x80, f + 0xc0, f + 0x60],
    );
    e.call(
        SET_TRANSFORMED_INVERSE_POS,
        &args![f + 0x90, f + 0xc0, f + 0x70],
    );
    e.vcall(shape, 0x20, &args![f + 0x1b, f + 0x80, f + 0x100]);
    let hit = 1.0 > e.mem.f32(f + 0x110);
    if !hit {
        return true;
    }
    let mut key = u32::MAX;
    for slot in 0..8 {
        let candidate = e.mem.u32(f + 0x120 + 4 * slot);
        if candidate == u32::MAX {
            break;
        }
        key = candidate;
    }
    fn_00c6d7e0(e, shape, 1, u8::from(stepped), key as i32) == 0
}

// Translated from 00c6f280 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c6f280 - the /GS cookie check is not modeled.
/// `bhkCharacterController::objectInteractionCallback` (Xbox PDB), `this` = the listener
/// (`controller + 0x410`). With the body (`event + 0x30`) of mass `m` and the impulse's
/// length `L`: for `0 < m < 95` (`011b0128`), the factor is 0 for motion type 6, 3.0 for
/// type 10 (0.1 when `m > 47.5`), 0.1 for 0xE and 2.0 otherwise, scaled by
/// `1000 / (L / m)` when `L / m > 1000`; when flag bit 0 is clear, bit 3 set and the event
/// point is more than 0.9 `fHeight` above the controller, the impulse becomes
/// `L * normalize(OutVelocity)`; the impulse is then scaled by the factor and the
/// reported force is `L * factor`. Otherwise the impulse is zeroed and the force is `L`.
/// A force above `fBumpedForce` replaces it and makes the body the bumped body.
pub fn bhk_character_controller_object_interaction_callback(
    e: &mut Engine,
    this: Ptr,
    _unused_1: u32,
    event: Ptr,
    impulse: Ptr,
) {
    let controller = controller_of(this);
    let impulse = impulse.addr();
    let body = e.mem.u32(event.addr() + 0x30);
    let mass = e.call(GET_MASS, &args![body + 0xe0]).f32();
    let length = sum_of_squares3(load4(e, impulse)).sqrt();
    let mut force = x87_quiet(length);
    let max_mass: f32 = e.global(MAX_PUSH_MASS);
    if mass > 0.0 && max_mass > mass {
        let motion = e.mem.u32(body + 0x2c) & 0x7f;
        let light = x87_quiet(e.global::<f32>(FLOAT_TENTH));
        let mut factor = match motion {
            6 => 0.0,
            10 => {
                let half: f64 = e.global(HALF);
                if max_mass as f64 * half < mass as f64 {
                    light
                } else {
                    x87_quiet(e.global::<f32>(FLOAT_THREE))
                }
            }
            0xe => light,
            _ => x87_quiet(e.global::<f32>(IMPULSE_DEFAULT)),
        };
        let ratio = (length as f64 / mass as f64) as f32;
        let limit: f64 = e.global(RATIO_LIMIT);
        if limit < ratio as f64 {
            factor = (limit / ratio as f64 * factor as f64) as f32;
        }
        let flags = e.get(controller, Ctl::iFlags);
        if flags & 1 == 0 && (flags >> 3) & 1 != 0 {
            e.with_stack(0x10, |e, position| {
                bhk_character_controller_get_position_ov2(e, controller, position);
                let point = load4(e, event.addr());
                let here = load4(e, position.addr());
                let rise = point[2] - here[2];
                let fraction: f64 = e.global(HEIGHT_FRACTION);
                if e.get(controller, Ctl::fHeight) as f64 * fraction < rise as f64 {
                    let velocity = load4(e, controller.addr() + Ctl::OutVelocity.off);
                    let zero: f32 = e.global(FLOAT_ZERO);
                    let three: f32 = e.global(FLOAT_THREE);
                    let half: f32 = e.global(FLOAT_HALF);
                    let inverse = inverse_length(sum_of_squares3(velocity), zero, half, three);
                    store4(e, impulse, velocity.map(|v| inverse * v * force));
                }
            });
        }
        force = (length as f64 * factor as f64) as f32;
        let scaled = load4(e, impulse).map(|v| factor * v);
        store4(e, impulse, scaled);
    } else {
        store4(e, impulse, [0.0; 4]);
    }
    let best = e.get(controller, Ctl::fBumpedForce);
    if best < force {
        e.set(controller, Ctl::fBumpedForce, force);
        let body = e.mem.u32(event.addr() + 0x30);
        if body != 0 {
            e.call(ADD_REFERENCE, &args![body]);
        }
        let old = e.get(controller, Ctl::spBumpedBody);
        if old != 0 {
            e.call(REMOVE_REFERENCE, &args![old]);
        }
        e.set(controller, Ctl::spBumpedBody, body);
    }
}

// Translated from 00c6f500 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c6f500 - the /GS cookie check is not modeled.
/// `bhkCharacterController::characterInteractionCallback` (Xbox PDB), `this` = the
/// listener. Keeps `proxy` as `spBumpedCharProxy`; when the other controller (`proxy +
/// 0xC0`) exists, has a priority not above ours and no flag 0x4000: pushes it along the
/// event's horizontal normal by `-0.25` times our horizontal speed (plus 20 when `eSize`
/// is 2) through `PushDelta`/`iPushCount` and `SetVelocityModifier(.., 0.01)`; and when
/// its flag bit 0 is set, turns it (`00c6cbc0` with `2 * c` and 0.2) by
/// `c = clamp(t * q, 0, 1)`, negated when `p.y * d.x - p.x * d.y` is above 0. Here `d` is
/// the contact point minus its position (horizontal), `p` the projection of `d` on its
/// horizontal forward vector, `q` the length of `p` over half its height, minus 0.05,
/// and `t` the float at `010c7160` (0.0) minus the dot product of our normalized
/// horizontal velocity with the event's normalized horizontal normal (0 when that
/// velocity is zero).
#[allow(clippy::neg_cmp_op_on_partial_ord)] // the x87 compare treats NaN as "not above 0"
pub fn bhk_character_controller_character_interaction_callback(
    e: &mut Engine,
    this: Ptr,
    _unused_1: u32,
    proxy: Ptr,
    event: Ptr,
) {
    let controller = controller_of(this);
    let other = e.mem.u32(proxy.addr() + 0xc0);
    e.call(ADD_REFERENCE, &args![proxy]);
    let old = e.get(controller, Ctl::spBumpedCharProxy);
    if old != 0 {
        e.call(REMOVE_REFERENCE, &args![old]);
    }
    e.set(controller, Ctl::spBumpedCharProxy, proxy.addr());
    if other == 0 {
        return;
    }
    let other = Ptr::<Ctl>::new(other);
    let size = e.get(controller, Ctl::eSize);
    if e.get(other, Ctl::iPriority) > e.get(controller, Ctl::iPriority) {
        return;
    }
    if e.get(other, Ctl::iFlags) & 0x4000 != 0 {
        return;
    }
    e.call(HKP_GET_LINEAR_VELOCITY, &args![proxy]);
    let zero: f32 = e.global(FLOAT_ZERO);
    let three: f32 = e.global(FLOAT_THREE);
    let half: f32 = e.global(FLOAT_HALF);
    let event = event.addr();

    e.with_stack(0x60, |e, frame| {
        let at = |o: u32| frame.addr() + o;
        // The event's normal, horizontal, normalized when it is not zero.
        let mut normal = load4(e, event + 0x10);
        normal[2] = 0.0;
        let sum = sum_of_squares3(normal);
        if 0.0 < sum {
            let inverse = inverse_length(sum, zero, half, three);
            normal = normal.map(|v| inverse * v);
        }
        // Our horizontal speed.
        let velocity_at = e.call(GET_HK_LINEAR_VELOCITY, &args![controller]).u32();
        let mut velocity = load4(e, velocity_at);
        velocity[2] = 0.0;
        let speed = sum_of_squares3(velocity).sqrt();
        let mut scale = x87_quiet(speed);
        if size == 2 {
            let bonus: f64 = e.global(LARGE_SIZE_BONUS);
            scale = (speed as f64 + bonus) as f32;
        }
        let push_scale: f64 = e.global(PUSH_SCALE);
        let factor = (scale as f64 * push_scale) as f32;
        store4(e, at(0x20), normal.map(|v| factor * v));
        e.call(COPY_POINT, &args![at(0x40), at(0x20)]);
        let modifier_time = x87_quiet_bits(e.global::<u32>(MODIFIER_TIME));
        let push = load4(e, other.addr() + Ctl::PushDelta.off);
        let delta = load4(e, at(0x20));
        let sum: [f32; 4] = std::array::from_fn(|lane| push[lane] + delta[lane]);
        store4(e, other.addr() + Ctl::PushDelta.off, sum);
        let count = e.get(other, Ctl::iPushCount);
        e.set(other, Ctl::iPushCount, count.wrapping_add(1));
        bhk_character_controller_set_velocity_modifier(
            e,
            other,
            Ptr::new(at(0x40)),
            f32::from_bits(modifier_time),
        );
        if e.get(other, Ctl::iFlags) & 1 == 0 {
            return;
        }

        // The contact point relative to the other character, horizontal.
        let point = load4(e, event);
        let position_at = e.call(GET_HK_POSITION, &args![other]).u32();
        let position = load4(e, position_at);
        let mut offset: [f32; 4] = std::array::from_fn(|lane| point[lane] - position[lane]);
        offset[2] = 0.0;
        // Its projection on the other's horizontal forward vector.
        let mut forward = load4(e, other.addr() + Ctl::ForwardVec.off);
        forward[2] = 0.0;
        let inverse = inverse_length(sum_of_squares3(forward), zero, half, three);
        let unit = forward.map(|v| inverse * v);
        let products = [0, 1, 2, 3].map(|lane| unit[lane] * offset[lane]);
        let along = x87_quiet(products[2] + (products[1] + products[0]));
        let projection = unit.map(|v| along * v);
        let distance = sum_of_squares3(projection).sqrt();
        let half_double: f64 = e.global(HALF);
        let decay: f64 = e.global(ROT_MOD_DECAY);
        let reach =
            (distance as f64 / (e.get(other, Ctl::fHeight) as f64 * half_double) - decay) as f32;
        // The normal again, horizontal and normalized.
        let mut normal = load4(e, event + 0x10);
        normal[2] = 0.0;
        let inverse = inverse_length(sum_of_squares3(normal), zero, half, three);
        let normal = normal.map(|v| inverse * v);
        // Minus the cosine between our horizontal velocity and the normal.
        let velocity_at = e.call(GET_HK_LINEAR_VELOCITY, &args![controller]).u32();
        let mut velocity = load4(e, velocity_at);
        velocity[2] = 0.0;
        let mut facing = 0.0f32;
        let sum = sum_of_squares3(velocity);
        if sum != 0.0 {
            let root = e.call(X87_SQRT, &args![x87_quiet(sum) as f64]).f32();
            let inverse = (1.0 / root as f64) as f32;
            let products = [0, 1, 2, 3].map(|lane| inverse * velocity[lane] * normal[lane]);
            let base: f32 = e.global(DOT_BASE);
            facing = x87_quiet(base - (products[2] + (products[1] + products[0])));
        }
        let product = x87_quiet((facing as f64 * reach as f64) as f32);
        let one = 0x3f80_0000u32;
        let upper = e
            .call(FLOAT_MIN, &args![one, x87_quiet_bits(product.to_bits())])
            .f32();
        let clamped = e
            .call(FLOAT_MAX, &args![0u32, x87_quiet_bits(upper.to_bits())])
            .f32();
        if !(clamped > 0.0) {
            return;
        }
        let side = projection[1] * offset[0] - projection[0] * offset[1];
        let mut turn = clamped;
        if 0.0 < side {
            let minus_one: f64 = e.global(MINUS_ONE);
            turn = x87_quiet((clamped as f64 * minus_one) as f32);
        }
        let time = x87_quiet(e.global::<f32>(ROT_MOD_TIME));
        let doubled = x87_quiet((turn as f64 + turn as f64) as f32);
        fn_00c6cbc0(e, other, doubled, time);
    });
}

/// Whether `CheckInsideOfObject` tests a collidable of the layer `layer - 4` (`0..=0x1a`): the
/// compiler's switch skips the other layers. Layers below 4 or above 30 are tested.
fn inside_layer_tested(layer: u32) -> bool {
    matches!(layer, 5 | 9..=11 | 14..=17 | 21..=24)
}

/// Three ray casts of `shape` (vtable slot 0x20) from the input at `frame + 0x20`, each
/// first lifting both ends (z at `+0x28` and `+0x38`) by `step`; true at the first hit
/// (hit fraction below 1).
fn cast_three_rays(e: &mut Engine, frame: u32, shape: u32, step: f32, result: u32) -> bool {
    for _ in 0..3 {
        e.mem.set_u32(frame + 0x80, 0x3f80_0000);
        let from = (e.mem.f32(frame + 0x28) as f64 + step as f64) as f32;
        e.mem.set_u32(frame + 0x84, u32::MAX);
        e.mem.set_u32(frame + 0x90, u32::MAX);
        e.mem.set_f32(frame + 0x28, from);
        let to = (step as f64 + e.mem.f32(frame + 0x38) as f64) as f32;
        e.mem.set_f32(frame + 0x38, to);
        e.mem.set_u32(frame + 0xb0, 0);
        e.vcall(shape, 0x20, &args![result, frame + 0x20, frame + 0x70]);
        if 1.0 > e.mem.f32(frame + 0x80) {
            return true;
        }
    }
    false
}

// Translated from 00c6fa20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c6fa20 - the /GS cookie check is not modeled.
/// `bhkCharacterController::CheckInsideOfObject` (Xbox PDB): from the controller's
/// position to a point `GetRadius() + 32 * (01267bcc)` along the phantom's rotated y axis,
/// casts rays against every overlapping collidable of the Havok phantom (array `+0x160`,
/// count `+0x164`) whose layer is tested and whose shape type is 6, 8 or 10: in the
/// collidable's local space, three casts outward and, without a hit, three inward, each
/// lifted by a quarter of `fActorHeight` more. True when an inward cast hits.
pub fn bhk_character_controller_check_inside_of_object(e: &mut Engine, this: Ptr<Ctl>) -> bool {
    let phantom = e.get(this, Ctl::spShapePhantom);
    let hk = if phantom != 0 {
        e.mem.u32(phantom + 8)
    } else {
        0
    };
    let quarter: f64 = e.global(QUARTER);
    let step = (e.get(this, Ctl::fActorHeight) as f64 * quarter) as f32;
    let radius = bhk_character_controller_get_radius(e, this) as f64;
    let thirty_two: f64 = e.global(THIRTY_TWO);
    let lift = (radius + e.global::<f32>(INVERSE_SCALE) as f64 * thirty_two) as f32;
    let zero: f32 = e.global(FLOAT_ZERO);
    e.with_stack(0xd0, |e, frame| {
        let f = frame.addr();
        store4(e, f + 0x50, [zero, lift, zero, zero]);
        let rotation_owner = if phantom != 0 {
            e.mem.u32(phantom + 8)
        } else {
            0
        };
        e.call(
            SET_ROTATED_DIR,
            &args![f + 0x50, rotation_owner.wrapping_add(0xb0), f + 0x50],
        );
        bhk_character_controller_get_position_ov2(e, this, Ptr::new(f + 0x60));
        let position = load4(e, f + 0x60);
        let direction = load4(e, f + 0x50);
        let end: [f32; 4] = std::array::from_fn(|lane| position[lane] + direction[lane]);
        store4(e, f + 0xc0, end);
        let mut index: i32 = 0;
        if e.mem.i32(hk + 0x164) <= 0 {
            return false;
        }
        loop {
            let entries = e.mem.u32(hk + 0x160);
            let collidable = e
                .mem
                .u32(entries.wrapping_add((index as u32).wrapping_mul(8)) + 4);
            if collidable != 0 {
                let layer = (e.mem.u32(collidable + 0x1c) & 0x7f).wrapping_sub(4);
                let tested = layer > 0x1a || inside_layer_tested(layer);
                let shape = e.mem.u32(collidable);
                if tested && matches!(e.mem.u32(shape + 0xc), 6 | 8 | 10) {
                    e.mem.set_u32(f + 0x80, 0x3f80_0000);
                    e.mem.set_u32(f + 0x84, u32::MAX);
                    e.mem.set_u32(f + 0x90, u32::MAX);
                    e.mem.set_u32(f + 0x40, 0);
                    e.mem.set_u32(f + 0x44, 0);
                    e.mem.set_u32(f + 0xb0, 0);
                    let transform = e.mem.u32(collidable + 8);
                    e.call(
                        SET_TRANSFORMED_INVERSE_POS,
                        &args![f + 0x20, transform, f + 0x60],
                    );
                    let transform = e.mem.u32(collidable + 8);
                    e.call(
                        SET_TRANSFORMED_INVERSE_POS,
                        &args![f + 0x30, transform, f + 0xc0],
                    );
                    let outward = cast_three_rays(e, f, shape, step, f + 0x1b);
                    let transform = e.mem.u32(collidable + 8);
                    e.call(
                        SET_TRANSFORMED_INVERSE_POS,
                        &args![f + 0x20, transform, f + 0xc0],
                    );
                    let transform = e.mem.u32(collidable + 8);
                    e.call(
                        SET_TRANSFORMED_INVERSE_POS,
                        &args![f + 0x30, transform, f + 0x60],
                    );
                    if !outward && cast_three_rays(e, f, shape, step, f + 0x1a) {
                        return true;
                    }
                }
            }
            index += 1;
            if index >= e.mem.i32(hk + 0x164) {
                return false;
            }
        }
    })
}

// Translated from 00c6fd20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `+0xC` of `spSupportBody`, 0 without one.
pub fn fn_00c6fd20(e: &mut Engine, this: Ptr<Ctl>) -> u32 {
    let body = e.get(this, Ctl::spSupportBody);
    if body != 0 {
        e.mem.u32(body + 0xc)
    } else {
        0
    }
}

// Translated from 00c6fd40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `+0xC` of `spBumpedBody`, 0 without one.
pub fn fn_00c6fd40(e: &mut Engine, this: Ptr<Ctl>) -> u32 {
    let body = e.get(this, Ctl::spBumpedBody);
    if body != 0 {
        e.mem.u32(body + 0xc)
    } else {
        0
    }
}

// Translated from 00c6fd60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `+0xC0` of `spBumpedCharProxy` (the word the character callback uses as
/// the other controller), 0 without one.
pub fn fn_00c6fd60(e: &mut Engine, this: Ptr<Ctl>) -> u32 {
    let proxy = e.get(this, Ctl::spBumpedCharProxy);
    if proxy != 0 {
        e.mem.u32(proxy + 0xc0)
    } else {
        0
    }
}

// Translated from 00c6fd80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c6fd80 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkCharacterController::BuildDisplayGeometry` (Xbox PDB): unless the display node
/// exists already, attaches a new `NiNode` named `"bhkColDisp"` to `node` (or, when
/// `node` is null, to the phantom's scene object, or that object's first child when it is
/// an `NiNode` and it has children), sets the node's `+0x58..+0x60` to `RotCenter` over
/// the parent's scale (`+0x98`) and `+0x64` to `|1 / scale|` when the scale is not 0,
/// hands it to the current shape's slot 0xD4 and sets flag 0x8000. Nothing without a
/// current shape. The allocation-scope word is 0x10 meanwhile.
pub fn bhk_character_controller_build_display_geometry(e: &mut Engine, this: Ptr<Ctl>, node: u32) {
    let (word, saved) = enter_scope(e);
    if bhk_character_controller_find_display_geometry(e, this) == 0 {
        build_display_node(e, this, node);
    }
    e.mem.set_u32(word, saved);
}

fn build_display_node(e: &mut Engine, this: Ptr<Ctl>, node: u32) {
    let mut node = node;
    if node == 0 {
        let phantom = e.get(this, Ctl::spShapePhantom);
        if phantom == 0 {
            return;
        }
        let object = e.call(GET_AV_OBJECT, &args![phantom, 0u32]).u32();
        if object == 0 {
            return;
        }
        node = e.vcall(object, 0xc, &args![]).u32();
        if node == 0 {
            return;
        }
        if e.mem.u16(node + 0xa6) != 0 {
            let child = e.call(NODE_CHILD_AT, &args![node, 0u32]).u32();
            if child == 0 {
                return;
            }
            node = e.call(IS_KIND_OF, &args![child, NODE_RTTI]).u32();
        }
        if node == 0 {
            return;
        }
    }
    if bhk_character_controller_get_shape(e, this) == 0 {
        return;
    }
    let block = e.call(HEAP_ALLOCATE, &args![NODE_SIZE]).u32();
    let display = if block != 0 {
        e.call(NODE_CONSTRUCT, &args![block, 0u32]).u32()
    } else {
        0
    };
    e.vcall(node, 0xdc, &args![display, 1u32]);
    let name = e.call(FIXED_STRING_CREATE, &args![DISPLAY_NAME]).u32();
    let name = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), name);
        e.call(SET_NAME, &args![display, slot]);
        e.mem.u32(slot.addr())
    });
    release_fixed_string(e, name);
    let inverse = (1.0 / e.mem.f32(node + 0x98) as f64) as f32;
    let center = e.with_stack(0x10, |e, copy| {
        let at = e
            .call(COPY_POINT, &args![copy, this.addr() + Ctl::RotCenter.off])
            .u32();
        [0, 4, 8].map(|o| e.mem.f32(at + o))
    });
    let x = (center[0] as f64 * inverse as f64) as f32;
    let y = (center[1] as f64 * inverse as f64) as f32;
    let z = (inverse as f64 * center[2] as f64) as f32;
    e.mem.set_f32(display.wrapping_add(0x58), x);
    e.mem.set_f32(display.wrapping_add(0x5c), y);
    e.mem.set_f32(display.wrapping_add(0x60), z);
    let scale = e.mem.f32(node + 0x98);
    if 0.0 != scale {
        let inverse = (1.0 / scale as f64) as f32;
        e.mem.set_f32(
            display.wrapping_add(0x64),
            x87_quiet(x87_quiet(inverse).abs()),
        );
    }
    let shape = bhk_character_controller_get_shape(e, this);
    e.vcall(shape, 0xd4, &args![display, DISPLAY_ARGUMENT]);
    let flags = e.get(this, Ctl::iFlags);
    e.set(this, Ctl::iFlags, flags | 0x8000);
}

// Translated from 00c6ffc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c6ffc0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkCharacterController::KillDisplayGeometry` (Xbox PDB): detaches the display node
/// (`FindDisplayGeometry`) from its parent (`+0x18`, slot 0xE4 with the node and an
/// `NiPointer` out slot on the stack), clears flag 0x8000 and releases what the parent
/// handed back.
pub fn bhk_character_controller_kill_display_geometry(e: &mut Engine, this: Ptr<Ctl>) {
    let node = bhk_character_controller_find_display_geometry(e, this);
    let removed = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), 0);
        if node != 0 {
            let parent = e.mem.u32(node + 0x18);
            if parent != 0 {
                e.vcall(parent, 0xe4, &args![node, slot]);
            }
        }
        e.mem.u32(slot.addr())
    });
    let flags = e.get(this, Ctl::iFlags);
    e.set(this, Ctl::iFlags, flags & 0xffff_7fff);
    if removed != 0 {
        release_ni_object(e, removed);
    }
}

/// Moves `field` (pitch or roll) towards `target`: when the difference is not 0 (NaN
/// counts), `fPitchMult` is first capped at 1.0 and the angle becomes
/// `difference * fPitchMult + angle`.
fn approach_angle(e: &mut Engine, this: Ptr<Ctl>, field: Field<Ctl, f32>, target: f32) {
    let difference = (target as f64 - e.get(this, field) as f64) as f32;
    if difference == 0.0 {
        return;
    }
    if 1.0 < e.get(this, Ctl::fPitchMult) {
        e.set(this, Ctl::fPitchMult, 1.0);
    }
    let multiplier = e.get(this, Ctl::fPitchMult);
    let value = (difference as f64 * multiplier as f64 + e.get(this, field) as f64) as f32;
    e.set(this, field, value);
}

/// `value` kept below `limit` (`limit` when not below it, NaN included), then above
/// `-limit`.
fn clamp_tilt(value: f32, limit: f32) -> f32 {
    let capped = if value < limit { value } else { limit };
    let floor = -limit;
    if floor < capped {
        capped
    } else {
        floor
    }
}

/// The tilt from two `SupportHeight` lanes (`first`, `first + 1`; a lane equal to the
/// float at `0102efc4` is probed again with `00c6e680`): 0 when they are equal, else the
/// helper `00ec9050` of `rise / fHeight`, limited to `MAX_TILT_DEGREES` (radians) on its
/// side of 0.
fn support_tilt(e: &mut Engine, this: Ptr<Ctl>, first: u32) -> f32 {
    let lanes = this.addr() + Ctl::SupportHeight.off + 4 * first;
    let mut low = x87_quiet(e.mem.f32(lanes));
    let mut high = x87_quiet(e.mem.f32(lanes + 4));
    let unset: f32 = e.global(SUPPORT_UNSET);
    if unset == low {
        low = probe_height(e, this, first);
    }
    if unset == high {
        high = probe_height(e, this, first + 1);
    }
    let rise = (high as f64 - low as f64) as f32;
    if rise == 0.0 {
        return 0.0;
    }
    let slope = (rise as f64 / e.get(this, Ctl::fHeight) as f64) as f32;
    let angle = x87_quiet(e.call(X87_SLOPE_HELPER, &args![slope as f64]).f32());
    let degrees = e.global::<f32>(MAX_TILT_DEGREES) as f64;
    if angle > 0.0 {
        let limit = degrees * e.global::<f64>(DEGREES_TO_RADIANS);
        if limit < angle as f64 {
            return limit as f32;
        }
    } else {
        let limit = degrees * e.global::<f64>(NEGATIVE_DEGREES_TO_RADIANS);
        if limit > angle as f64 {
            return limit as f32;
        }
    }
    angle
}

/// The z of the ground probe (`00c6e680`) under `FrontBack[index]`.
fn probe_height(e: &mut Engine, this: Ptr<Ctl>, index: u32) -> f32 {
    e.with_stack(0x10, |e, out| {
        fn_00c6e680(e, this, index, out);
        x87_quiet(e.mem.f32(out.addr() + 8))
    })
}

// Translated from 00c70060 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c70060 - the /GS cookie check is not modeled.
/// Pitch and roll update while `fCalculatePitchTimer` is above 0 (not NaN) and flag bit 18
/// is clear. With flag bit 0, from the support heights: pitch (bit 25) from lanes 0 and 1,
/// roll (bit 26, the flags read again) from lanes 2 and 3 (see `support_tilt`). Without
/// it, from `hkSupportNorm` brought into the phantom's frame (`004b4ab0` with its
/// transform `+0xB0`): roll (bit 26) from `-x`, pitch (bit 25) from `y`, each within
/// `±MAX_TILT_DEGREES` in radians. Each angle then moves towards its target by
/// `fPitchMult` (capped at 1).
pub fn fn_00c70060(e: &mut Engine, this: Ptr<Ctl>) {
    let flags = e.get(this, Ctl::iFlags);
    if (flags >> 0x12) & 1 != 0 {
        return;
    }
    let timer = e.get(this, Ctl::fCalculatePitchTimer);
    if !is_positive(timer) {
        return;
    }
    if flags & 1 == 0 {
        tilt_from_support_normal(e, this);
        return;
    }
    if (flags >> 0x19) & 1 != 0 {
        let target = support_tilt(e, this, 0);
        approach_angle(e, this, Ctl::fPitchAngle, target);
    }
    if (e.get(this, Ctl::iFlags) >> 0x1a) & 1 != 0 {
        let target = support_tilt(e, this, 2);
        approach_angle(e, this, Ctl::fRollAngle, target);
    }
}

fn tilt_from_support_normal(e: &mut Engine, this: Ptr<Ctl>) {
    let hk = phantom_object(e, this);
    let normal = e.with_stack(0x10, |e, out| {
        e.call(
            TRANSFORM_VECTOR,
            &args![
                out,
                hk.wrapping_add(0xb0),
                this.addr() + Ctl::hkSupportNorm.off
            ],
        );
        load4(e, out.addr())
    });
    let degrees = e.global::<f32>(MAX_TILT_DEGREES) as f64;
    let limit = (degrees * e.global::<f64>(DEGREES_TO_RADIANS)) as f32;
    let flags = e.get(this, Ctl::iFlags);
    if (flags >> 0x1a) & 1 != 0 {
        let minus_one: f64 = e.global(MINUS_ONE);
        let across = (x87_quiet(normal[0]) as f64 * minus_one) as f32;
        approach_angle(e, this, Ctl::fRollAngle, clamp_tilt(across, limit));
    }
    if (flags >> 0x19) & 1 != 0 {
        let along = x87_quiet(normal[1]);
        approach_angle(e, this, Ctl::fPitchAngle, clamp_tilt(along, limit));
    }
}

// Translated from 00c70550 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c70550 - the /GS cookie check is not modeled.
/// `bhkCharacterController::GetFallDistance` (Xbox PDB): `fFallStartHeight` minus the z of
/// the current position (`GetPosition_ov2`, copied by `00458620`), or 0 when
/// `fFallStartHeight` is 0.
pub fn bhk_character_controller_get_fall_distance(e: &mut Engine, this: Ptr<Ctl>) -> f32 {
    if e.get(this, Ctl::fFallStartHeight) == 0.0 {
        return 0.0;
    }
    e.with_stack(0x20, |e, frame| {
        let position = frame.addr();
        let copy = position + 0x10;
        bhk_character_controller_get_position_ov2(e, this, frame);
        e.call(COPY_POINT, &args![copy, position]);
        let start = e.get(this, Ctl::fFallStartHeight) as f64;
        (start - e.mem.f32(copy + 8) as f64) as f32
    })
}

/// Releases an `hkRefPtr` member at `slot` and clears it (`00c6cca0`'s body).
fn release_ref_slot(e: &mut Engine, slot: u32) {
    fn_00c6cca0(e, Ptr::new(slot));
}

/// Frees an `hkArray` (`array`: data, size, capacity-and-flags) as its inlined destructor
/// does: size 0, the buffer (`capacity * element` bytes) freed through the Havok
/// allocator's slot 0x10 unless the flags word is negative, then pointer 0 and capacity
/// word 0x80000000.
fn free_hk_array(e: &mut Engine, array: u32, element: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    e.mem.set_u32(array + 4, 0);
    let capacity = e.mem.i32(array + 8);
    if capacity >= 0 {
        let data = e.mem.u32(array);
        let bytes = (capacity as u32 & 0x3fff_ffff).wrapping_mul(element);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 8, 0x8000_0000);
}

/// The world the Havok proxy's phantom is in (`+0x350` of its `+8`), or 0.
fn proxy_world(e: &mut Engine, this: Ptr<Ctl>) -> u32 {
    let proxy = e.get(this, Ctl::phkObject);
    let phantom = if proxy != 0 {
        e.call(GET_SHAPE_PHANTOM, &args![proxy]).u32()
    } else {
        0
    };
    let world = e.mem.u32(phantom.wrapping_add(8));
    if world != 0 {
        e.mem.u32(world + 0x350)
    } else {
        0
    }
}

// Translated from 00c705c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c705c0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkCharacterController` destructor body: the class vtables; with the allocation-scope
/// word at 0x10, `LastManifold`'s size cleared, the listener removed from the Havok proxy
/// (`00cad6f0`, between two `UnMarkWrite` calls) when it is in a world, and the bumped
/// body, bumped proxy and support body released; then the `DamageImpacts` list destroyed,
/// `LastManifold` freed, the three references released again (already null), the
/// `spShapes` pointers destroyed, the collision box and phantom released, and the
/// `hkpCharacterContext` and `bhkCharacterProxy` bases destroyed.
pub fn fn_00c705c0(e: &mut Engine, this: Ptr<Ctl>) {
    let me = this.addr();
    e.mem.set_u32(me, VTABLE_CONTROLLER);
    e.mem.set_u32(me + 0x3e0, VTABLE_CONTROLLER_CONTEXT);
    e.mem.set_u32(me + LISTENER, VTABLE_CONTROLLER_LISTENER);
    let (word, saved) = enter_scope(e);
    e.set(this, Ctl::LastManifoldSize, 0);
    if proxy_world(e, this) != 0 {
        let proxy = e.get(this, Ctl::phkObject);
        if proxy != 0 {
            e.call(UN_MARK_WRITE, &args![this]);
            e.call(PROXY_LISTENER_REMOVE, &args![proxy, me + LISTENER]);
            e.call(UN_MARK_WRITE, &args![this]);
        }
    }
    for field in [
        Ctl::spBumpedBody,
        Ctl::spBumpedCharProxy,
        Ctl::spSupportBody,
    ] {
        release_ref_slot(e, me + field.off);
    }
    e.mem.set_u32(word, saved);
    e.call(LIST_DESTRUCT, &args![me + Ctl::DamageImpacts.off]);
    free_hk_array(e, me + Ctl::LastManifold.off, 0x70);
    for field in [
        Ctl::spBumpedCharProxy,
        Ctl::spBumpedBody,
        Ctl::spSupportBody,
    ] {
        release_ref_slot(e, me + field.off);
    }
    e.call(
        EH_VECTOR_DESTRUCTOR,
        &args![me + Ctl::spShapes.off, 4u32, 2u32, NI_POINTER_DESTRUCT],
    );
    for field in [Ctl::spCollisionBox, Ctl::spShapePhantom] {
        let object = e.get(this, field);
        if object != 0 {
            release_ni_object(e, object);
        }
    }
    e.mem.set_u32(me + LISTENER, VTABLE_PROXY_LISTENER);
    e.call(CONTEXT_DESTRUCT, &args![me + 0x3e0]);
    e.call(CHARACTER_PROXY_DESTRUCT, &args![this]);
}

// Translated from 00c70810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk from the listener base (`this - 0x410`) to the scalar deleting destructor
/// `00c70c60`.
pub fn fn_00c70810(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    fn_00c70c60(e, Ptr::new(this.addr().wrapping_sub(LISTENER)), flags).cast()
}

// Translated from 00c70820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk from the `hkpCharacterContext` base (`this - 0x3E0`) to the scalar deleting
/// destructor `00c70c60`.
pub fn fn_00c70820(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    fn_00c70c60(e, Ptr::new(this.addr().wrapping_sub(0x3e0)), flags).cast()
}

// Translated from 00c70830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCharacterController::SetShapeType` (Xbox PDB): switches to shape `shape_type` when
/// it differs, is below 2 (signed) and `spShapes` has it: the display geometry is killed
/// first and rebuilt after (vtable slot 0xD0 with 0) when flag 0x8000 was set, and the
/// shape is set with `SetShape(GetShape(), GethkPosition())`.
pub fn bhk_character_controller_set_shape_type(e: &mut Engine, this: Ptr<Ctl>, shape_type: i32) {
    if e.get(this, Ctl::eShapeType) == shape_type || shape_type >= 2 {
        return;
    }
    let slot = (this.addr() + Ctl::spShapes.off).wrapping_add((shape_type as u32).wrapping_mul(4));
    if e.mem.u32(slot) == 0 {
        return;
    }
    let displayed = (e.get(this, Ctl::iFlags) >> 0xf) & 1 != 0;
    if displayed {
        bhk_character_controller_kill_display_geometry(e, this);
    }
    e.set(this, Ctl::eShapeType, shape_type);
    let position = e.call(GET_HK_POSITION, &args![this]).u32();
    let shape = bhk_character_controller_get_shape(e, this);
    bhk_character_controller_set_shape(e, this, shape, position);
    if displayed {
        e.vcall(this.addr(), 0xd0, &args![0u32]);
    }
}

// Translated from 00c708a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCharacterController::SetSize` (Xbox PDB): unless flag bit 0 is set, the wanted
/// radius is `size` limited to half of `fHeight`, or `fRadius` when that is not above 0
/// (NaN stays); when it differs from `fDestRadius` (NaN counts) it becomes `fDestRadius`
/// with `eSize` 2, and a pending `eSizedShapeType` of 2 first takes `eShapeType` and the
/// shape type goes to 0.
pub fn bhk_character_controller_set_size(e: &mut Engine, this: Ptr<Ctl>, size: f32) {
    if e.get(this, Ctl::iFlags) & 1 != 0 {
        return;
    }
    let half: f64 = e.global(HALF);
    let limit = e.get(this, Ctl::fHeight) as f64 * half;
    let wanted = if size as f64 > limit {
        limit as f32
    } else {
        size
    };
    let wanted = if 0.0 < wanted || wanted.is_nan() {
        wanted
    } else {
        e.get(this, Ctl::fRadius)
    };
    let wanted = x87_quiet(wanted);
    if e.get(this, Ctl::fDestRadius) == wanted {
        return;
    }
    if e.get(this, Ctl::eSizedShapeType) == 2 {
        let current = e.get(this, Ctl::eShapeType);
        e.set(this, Ctl::eSizedShapeType, current);
        bhk_character_controller_set_shape_type(e, this, 0);
    }
    e.set(this, Ctl::eSize, 2);
    e.set(this, Ctl::fDestRadius, wanted);
}

// Translated from 00c70940 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c70940 - the /GS cookie check is not modeled.
/// `bhkCharacterController::ChangeSize` (Xbox PDB), while `eSize` is 2. With a
/// `bhkCapsuleShape` in `spShapes[0]`: its radius (0.1 without a Havok shape) steps
/// towards `fDestRadius` by `rate * 5` (`rate` 100 with flag bit 20), stopping at it; the
/// capsule's vertices move along z by the step (the first down, the second up, each `w`
/// taking the old radius) and the radius is stored. Without the capsule the radius is
/// `fDestRadius`. When the radius reached `fDestRadius`, `eSize` becomes 1 if it differs
/// from `fRadius`, else 0, after a pending `eSizedShapeType` is applied and reset to 2.
pub fn bhk_character_controller_change_size(e: &mut Engine, this: Ptr<Ctl>, rate: f32) {
    if e.get(this, Ctl::eSize) != 2 {
        return;
    }
    let shape = e.get(this, Ctl::spShapes);
    let capsule = if shape != 0 {
        e.call(IS_KIND_OF, &args![shape, CAPSULE_SHAPE_RTTI]).u32()
    } else {
        0
    };
    let target = if capsule == 0 {
        x87_quiet(e.get(this, Ctl::fDestRadius))
    } else {
        step_capsule_radius(e, this, capsule, rate)
    };
    if e.get(this, Ctl::fDestRadius) != target {
        return;
    }
    if e.get(this, Ctl::fRadius) != target {
        e.set(this, Ctl::eSize, 1);
        return;
    }
    let sized = e.get(this, Ctl::eSizedShapeType);
    if sized != 2 {
        bhk_character_controller_set_shape_type(e, this, sized);
        e.set(this, Ctl::eSizedShapeType, 2);
    }
    e.set(this, Ctl::eSize, 0);
}

fn step_capsule_radius(e: &mut Engine, this: Ptr<Ctl>, capsule: u32, rate: f32) -> f32 {
    let rate = if (e.get(this, Ctl::iFlags) >> 0x14) & 1 != 0 {
        x87_quiet(e.global::<f32>(FLOAT_HUNDRED))
    } else {
        rate
    };
    let hk = e.mem.u32(capsule + 8);
    let current = if hk != 0 {
        x87_quiet(e.mem.f32(hk + 0x10))
    } else {
        x87_quiet(e.global::<f32>(DEFAULT_RADIUS))
    };
    let five: f64 = e.global(FIVE);
    let step = (rate as f64 * five) as f32;
    let destination = e.get(this, Ctl::fDestRadius);
    let (delta, target) = if destination < current {
        let next = (current as f64 - step as f64) as f32;
        if destination < next || next.is_nan() {
            (-step, next)
        } else {
            (
                (destination as f64 - current as f64) as f32,
                x87_quiet(destination),
            )
        }
    } else {
        let next = (step as f64 + current as f64) as f32;
        if destination <= next {
            (
                (destination as f64 - current as f64) as f32,
                x87_quiet(destination),
            )
        } else {
            (step, next)
        }
    };
    let first_source = if hk != 0 { hk + 0x20 } else { ZERO_VECTOR };
    let mut first = load4(e, first_source);
    let second_source = if hk != 0 { hk + 0x30 } else { ZERO_VECTOR };
    let mut second = load4(e, second_source);
    first[2] = (x87_quiet(first[2]) as f64 - delta as f64) as f32;
    second[2] = (delta as f64 + x87_quiet(second[2]) as f64) as f32;
    if hk != 0 {
        let w = x87_quiet(e.mem.f32(hk + 0x10));
        store4(e, hk + 0x20, first);
        e.mem.set_f32(hk + 0x2c, w);
    }
    let hk = e.mem.u32(capsule + 8);
    if hk != 0 {
        let w = x87_quiet(e.mem.f32(hk + 0x10));
        store4(e, hk + 0x30, second);
        e.mem.set_f32(hk + 0x3c, w);
    }
    let hk = e.mem.u32(capsule + 8);
    if hk != 0 {
        e.mem.set_f32(hk + 0x10, target);
    }
    target
}

// Translated from 00c70b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCharacterController::MoveImmobile` (Xbox PDB): when `flag` is set and the
/// `hkpCharacterContext` state is 2, copies the point at `011f426c` to `params + 0x10` and
/// moves (vtable slot 0xC8 with `params`); otherwise `ChangeSize` with the float at
/// `params`.
pub fn bhk_character_controller_move_immobile(
    e: &mut Engine,
    this: Ptr<Ctl>,
    params: Ptr,
    flag: u8,
) {
    let state = e.call(GET_STATE, &args![this.addr() + 0x3e0]).u32();
    if flag != 0 && state == 2 {
        copy_words(e, params.addr() + 0x10, POINT_DEFAULT, 3);
        e.vcall(this.addr(), 0xc8, &args![params]);
        return;
    }
    let rate = x87_quiet(e.mem.f32(params.addr()));
    bhk_character_controller_change_size(e, this, rate);
}

// Translated from 00c70bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties `DamageImpacts`: each item's first word (an `NiPointer`) is released and the
/// item freed through the `MemoryManager`; each following node is unlinked (its item and
/// next move into the head, its next cleared), destroyed (`00b4df30`) and freed.
pub fn fn_00c70bd0(e: &mut Engine, this: Ptr<Ctl>) {
    loop {
        let next = e.get(this, Ctl::DamageImpactsNext);
        if next == 0 && e.get(this, Ctl::DamageImpacts) == 0 {
            return;
        }
        let item = e.get(this, Ctl::DamageImpacts);
        if item != 0 {
            let object = e.mem.u32(item);
            if object != 0 {
                release_ni_object(e, object);
            }
            e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, item]);
        }
        let next = e.get(this, Ctl::DamageImpactsNext);
        if next != 0 {
            let following = e.mem.u32(next + 4);
            e.set(this, Ctl::DamageImpactsNext, following);
            let moved = e.mem.u32(next);
            e.set(this, Ctl::DamageImpacts, moved);
            e.mem.set_u32(next + 4, 0);
            e.call(LIST_DESTRUCT, &args![next]);
            e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, next]);
        } else {
            e.set(this, Ctl::DamageImpacts, 0);
        }
    }
}

// Translated from 00c70c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCharacterController` scalar deleting destructor: destroys (`00c705c0`) and, when bit
/// 0 of `flags` is set and `this` is not null, frees the aligned block (`this` minus the
/// offset byte at `this - 1`) through the `MemoryManager`.
pub fn fn_00c70c60(e: &mut Engine, this: Ptr<Ctl>, flags: u8) -> Ptr<Ctl> {
    fn_00c705c0(e, this);
    if flags & 1 != 0 && this.addr() != 0 {
        let offset = e.mem.u8(this.addr().wrapping_sub(1));
        let block = this.addr().wrapping_sub(u32::from(offset));
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, block]);
    }
    this
}

/// `hkColor::rgbFromFloats(1, 1, 0, 0.75)` given to `bhkWorld::SetDebugObjectColor` for
/// the phantom.
fn set_phantom_debug_color(e: &mut Engine, world: u32, phantom: u32) {
    let alpha = x87_quiet_bits(e.global::<u32>(DEBUG_ALPHA));
    let one = 1.0f32.to_bits();
    let color = e
        .call(RGB_FROM_FLOATS, &args![one, one, 0.0f32.to_bits(), alpha])
        .u32();
    e.call(SET_DEBUG_OBJECT_COLOR, &args![world, phantom, color]);
}

// Translated from 00c70c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCharacterController::SetWorld` (Xbox PDB): when `world` differs from the proxy's
/// current world, leaves that (`00cac680`, then slot 0xA0 of the phantom and the collision
/// box), clears the word at `+4` of the Havok proxy's `+0x74` part, and with a new world:
/// sizes the phantom (unless flag bit 0 or 20: `SetSize`, `SetRadius` with 0.01 times the
/// inverse scale, `SetSize(0)`), adds it (slot 0x9C), gives it its debug colour, and adds
/// the collision box.
pub fn bhk_character_controller_set_world(e: &mut Engine, this: Ptr<Ctl>, world: u32) {
    let current = proxy_world(e, this);
    if current == world {
        return;
    }
    if current != 0 {
        e.call(PROXY_LEAVE_WORLD, &args![this]);
        for field in [Ctl::spShapePhantom, Ctl::spCollisionBox] {
            let object = e.get(this, field);
            if object != 0 {
                e.vcall(object, 0xa0, &args![]);
            }
        }
    }
    let proxy = e.get(this, Ctl::phkObject);
    if proxy != 0 {
        let part = e.call(PROXY_PART_74, &args![proxy]).u32();
        e.mem.set_u32(part.wrapping_add(4), 0);
    }
    if world == 0 {
        return;
    }
    let phantom = e.get(this, Ctl::spShapePhantom);
    if phantom != 0 {
        let flags = e.get(this, Ctl::iFlags);
        if flags & 1 == 0 && (flags >> 0x14) & 1 == 0 {
            let hundredth: f64 = e.global(HUNDREDTH);
            let size = (e.global::<f32>(INVERSE_SCALE) as f64 * hundredth) as f32;
            bhk_character_controller_set_size(e, this, size);
            let radius = (e.global::<f32>(INVERSE_SCALE) as f64 * hundredth) as f32;
            bhk_character_controller_set_radius(e, this, radius);
            bhk_character_controller_set_size(e, this, 0.0);
        }
        e.vcall(phantom, 0x9c, &args![world]);
        let phantom = e.get(this, Ctl::spShapePhantom);
        set_phantom_debug_color(e, world, phantom);
    }
    let collision_box = e.get(this, Ctl::spCollisionBox);
    if collision_box != 0 {
        e.vcall(collision_box, 0x9c, &args![world]);
    }
}

/// `hkArray<hkVector4>` push-back as inlined: room for one more through `00c90940` when
/// the size reached the capacity, then the vector stored and the size raised.
fn push_vector(e: &mut Engine, array: u32, v: [f32; 4]) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    if e.mem.u32(array + 4) == e.mem.u32(array + 8) & 0x3fff_ffff {
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 0x10u32]);
    }
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    store4(e, data.wrapping_add(size.wrapping_mul(0x10)), v);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

/// `point` times the inverse scale (the x87 order: `x*k`, `y*k`, `k*z`), `w` 0.
fn scaled_point(e: &Engine, point: u32) -> [f32; 4] {
    let k = e.global::<f32>(INVERSE_SCALE) as f64;
    let p = load4(e, point);
    [
        (p[0] as f64 * k) as f32,
        (p[1] as f64 * k) as f32,
        (k * p[2] as f64) as f32,
        0.0,
    ]
}

// Translated from 00c70de0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c70de0 - the FS:[0] exception-unwinding frame and the /GS cookie check are not modeled.
/// cdecl: builds a shape (`00cd7f50`) from 18 vertices in an `hkArray` on the stack
/// (reserved for 18 first), all in Havok units (times the inverse scale, `r` the scaled
/// `radius`): `lower` lowered by `r`; a ring of eight at `lower + (RING_X[i] * r,
/// RING_Y[i] * r)` at that height plus `height + 0.1`; a ring of eight at `(RING_X[i] *
/// r, RING_Y[i] * r)` (not offset by `upper`'s x and y) at `upper`'s height plus `r / 2`;
/// and `upper` raised by `r`. Frees the array; the allocation-scope word is 0x10 meanwhile.
pub fn fn_00c70de0(e: &mut Engine, upper: Ptr, lower: Ptr, radius: f32, height: f32) -> u32 {
    let (word, saved) = enter_scope(e);
    let r = (e.global::<f32>(INVERSE_SCALE) as f64 * radius as f64) as f32;
    let shape = e.with_stack(0xc, |e, array| {
        let array = array.addr();
        e.mem.set_u32(array, 0);
        e.mem.set_u32(array + 4, 0);
        e.mem.set_u32(array + 8, 0x8000_0000);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
        if capacity < 0x12 {
            let wanted = (capacity * 2).max(0x12);
            e.call(
                ARRAY_RESERVE,
                &args![allocator, array, wanted as u32, 0x10u32],
            );
        }
        let mut point = scaled_point(e, lower.addr());
        point[2] = (point[2] as f64 - r as f64) as f32;
        push_vector(e, array, point);
        let tenth: f64 = e.global(TENTH);
        point[2] = (x87_quiet(height) as f64 + tenth + point[2] as f64) as f32;
        for i in 0..8 {
            let x: f32 = e.global(RING_X + 4 * i);
            let y: f32 = e.global(RING_Y + 4 * i);
            let mut vertex = point;
            vertex[0] = (x as f64 * r as f64 + point[0] as f64) as f32;
            vertex[1] = (r as f64 * y as f64 + point[1] as f64) as f32;
            push_vector(e, array, vertex);
        }
        let mut top = scaled_point(e, upper.addr());
        let half: f64 = e.global(HALF);
        let half_radius = half * r as f64;
        top[2] = (half_radius + top[2] as f64) as f32;
        for i in 0..8 {
            let x: f32 = e.global(RING_X + 4 * i);
            let y: f32 = e.global(RING_Y + 4 * i);
            let mut vertex = top;
            vertex[0] = (x as f64 * r as f64) as f32;
            vertex[1] = (r as f64 * y as f64) as f32;
            push_vector(e, array, vertex);
        }
        top[2] = (top[2] as f64 + half_radius) as f32;
        push_vector(e, array, top);
        let shape = e.call(SHAPE_FROM_VERTICES, &args![array]).u32();
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        e.mem.set_u32(array + 4, 0);
        let capacity = e.mem.i32(array + 8);
        if capacity >= 0 {
            let data = e.mem.u32(array);
            let bytes = (capacity as u32 & 0x3fff_ffff) << 4;
            e.vcall(allocator, 0x10, &args![data, bytes]);
        }
        shape
    });
    e.mem.set_u32(word, saved);
    shape
}

// Translated from 00c71120 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c71120 - the FS:[0] exception-unwinding frame is not modeled.
/// Frees the two word arrays of `this` (`hkArray`s at `+0x10` and `+4`, in that order).
pub fn fn_00c71120(e: &mut Engine, this: Ptr) {
    free_hk_array(e, this.addr() + 0x10, 4);
    free_hk_array(e, this.addr() + 4, 4);
}

/// `inverse_length` with the game's zero, half and three constants, applied to all four
/// lanes of `v` (the `MULPS` by the broadcast factor).
fn normalized(e: &Engine, v: [f32; 4]) -> [f32; 4] {
    let zero = f32::from_bits(e.global::<u32>(FLOAT_ZERO));
    let half: f32 = e.global(FLOAT_HALF);
    let three: f32 = e.global(FLOAT_THREE);
    let inverse = inverse_length(sum_of_squares3(v), zero, half, three);
    v.map(|lane| inverse * lane)
}

/// `(0, 0, 1, 0)` from the zero and one float constants' bits (the `UNPCKLPS` build).
fn up_vector(e: &Engine) -> [f32; 4] {
    let zero = f32::from_bits(e.global::<u32>(FLOAT_ZERO));
    let one = f32::from_bits(e.global::<u32>(FLOAT_ONE));
    [zero, zero, one, zero]
}

/// Frame slots of `00c711d0` (from its `ESP` after the prologue).
mod constraint_frame {
    /// The contact normal's z (`+0x34`; 1 for the mass and layer-10 cases), the impact
    /// item pointer (`+0x38`, also the layer), the impact vector (`+0x50`).
    pub const NORMAL_Z: u32 = 0x34;
    pub const IMPACT_ITEM: u32 = 0x38;
    /// The `hkArray` copy of the phantom's overlapping collidables.
    pub const OVERLAPS: u32 = 0x44;
    pub const IMPACT: u32 = 0x50;
    pub const RADIUS: u32 = 0x5c;
    pub const FOOT_DISTANCE: u32 = 0x60;
    pub const HIT_BOOL: u32 = 0x6f;
    pub const VELOCITY: u32 = 0x88;
    pub const MOTION: u32 = 0xb0;
    pub const ZERO: u32 = 0xb4;
    pub const SCRATCH: u32 = 0xc4;
    pub const NORMAL: u32 = 0xd4;
    pub const POSITION: u32 = 0xe4;
    pub const FROM: u32 = 0xf4;
    pub const TO: u32 = 0x104;
    pub const FOOT_0: u32 = 0x114;
    pub const FOOT_1: u32 = 0x124;
    pub const RAY_INPUT: u32 = 0x134;
    pub const RAY_OUTPUT: u32 = 0x164;
    pub const COLLECTOR: u32 = 0x1b4;
    pub const SIZE: u32 = 0x240;
}

// Translated from 00c711d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c711d0 - the FS:[0] exception-unwinding frame and the /GS cookie check are not modeled.
/// The listener's `processConstraintsCallback` (slot 4 of vtable `010c4998`; slot name
/// from the Xbox PDB's `hkpCharacterProxyListener`), `this` = the listener; the first
/// stack word is never read. With the allocation-scope word at 0x10: flag 4 is cleared,
/// `SupportHeight` set to the float at `0102efc4`, `hkSupportNorm` to `UpVec`, the support
/// count to 0 (and with flag bit 16 the support body released). For each of the first
/// `min(input + 0x4C, manifold size)` contacts (0x70-byte points of `manifold`, 0x40-byte
/// planes at `input + 0x48`) it records a damage impact against a heavy moving body
/// (flag bits 16 and 21), runs the support probe for nearly level contacts with a
/// type-4 body, adjusts the plane by the contact's layer (with or without a body), and
/// for a body contact whose normal z is not below 0 decides support: flat ground (not
/// below `fMaxSlope`) marks flag 0x400, steeper ground is a step (`IsStep`: flag 0x404 and
/// an upright plane) or adds a horizontal plane (up to four). Supported contacts add
/// their plane normal to `hkSupportNorm`, raise the support count and flag 0x200, and
/// update the support (`00c6e980`, `spSupportBody` with flag bit 16). Then flag bit 29
/// says whether a contact had layer 2, `bAllSurfacesStopped` whether every support was
/// one, and `hkSupportNorm` is normalised (the CRT square root `00ec6040`).
pub fn fn_00c711d0(e: &mut Engine, this: Ptr, _unused_1: u32, manifold: u32, input: u32) {
    let controller = controller_of(this);
    e.with_stack(constraint_frame::SIZE, |e, frame| {
        process_constraints(e, controller, manifold, input, frame.addr());
    });
}

fn process_constraints(e: &mut Engine, ctl: Ptr<Ctl>, manifold: u32, input: u32, f: u32) {
    use constraint_frame as slot;
    let me = ctl.addr();
    let (word, saved) = enter_scope(e);
    let unset = x87_quiet_bits(e.global::<u32>(SUPPORT_UNSET));
    let flags = e.get(ctl, Ctl::iFlags);
    e.set(ctl, Ctl::iFlags, flags & 0xffff_fffb);
    for lane in (0..4).rev() {
        e.mem.set_u32(me + Ctl::SupportHeight.off + 4 * lane, unset);
    }
    copy_zero(e, me + Ctl::hkSupportNorm.off, 4);
    copy_zero(e, f + slot::ZERO, 4);
    if (e.get(ctl, Ctl::iFlags) >> 16) & 1 != 0 {
        release_ref_slot(e, me + Ctl::spSupportBody.off);
    }
    e.mem.set_f32(f + slot::FOOT_DISTANCE, 0.0);
    e.set(ctl, Ctl::iSupportCount, 0);
    let mut tally = Tally::default();
    copy_words(e, me + Ctl::hkSupportNorm.off, me + Ctl::UpVec.off, 4);
    let flags = e.get(ctl, Ctl::iFlags);
    let feet = flags & 1 == 0 && (flags >> 5) & 1 == 0 && foot_capsules(e, ctl, f);
    let manifold_size = e.mem.i32(manifold + 4);
    let planes = e.mem.i32(input + 0x4c);
    let count = if planes < manifold_size {
        planes
    } else {
        manifold_size
    };
    let mut layer_two = false;
    let mut index = 0;
    while index < count {
        let plane = e
            .mem
            .u32(input + 0x48)
            .wrapping_add((index as u32).wrapping_mul(0x40));
        let offset = (index as u32).wrapping_mul(0x70);
        let point = e.mem.u32(manifold).wrapping_add(offset);
        let collidable = e.mem.u32(point + 0x48);
        let body = collidable_owner(e, collidable, 1);
        let filter = e.mem.u32(collidable + 0x1c);
        let flags = e.get(ctl, Ctl::iFlags);
        if (flags >> 16) & 1 != 0 && flags & 0x20_0000 != 0 && fn_00c6def0(e, body) && body != 0 {
            let reference = e.mem.u32(body + 0xc);
            if reference != 0 {
                record_impact(e, ctl, f, point, reference);
            }
        }
        let layer = filter & 0x7f;
        layer_two |= layer == 2;
        let mut slide = (!(e.get(ctl, Ctl::iFlags) >> 11)) & 1 != 0;
        let normal_z = x87_quiet(e.mem.f32(point + 0x18));
        e.mem.set_f32(f + slot::NORMAL_Z, normal_z);
        e.mem.set_u32(f + slot::IMPACT_ITEM, layer);
        if body != 0 {
            let upright: f64 = e.global(UPRIGHT_NORMAL);
            if normal_z as f64 > upright
                && e.mem.u8(body + 0xe8) == 4
                && e.global::<u32>(BEST_SUPPORT_BODY) == 0
            {
                probe_support_bodies(e, ctl, f, point);
            }
        }
        copy_words(e, f + slot::NORMAL, plane, 4);
        if body == 0 {
            adjust_plane_without_body(e, ctl, f, point, plane, layer);
            index += 1;
            continue;
        }
        match layer {
            2 if matches!(e.mem.u8(body + 0xe8), 4 | 5) => {
                let speed = x87_quiet_bits(e.global::<u32>(PLANE_SPEED_MOVING));
                e.mem.set_u32(plane + 0x28, speed);
            }
            2 | 10 => {
                if (e.get(ctl, Ctl::iFlags) >> 16) & 1 == 0 {
                    e.mem.set_f32(f + slot::NORMAL_Z, 1.0);
                    e.mem.set_f32(plane + 0x2c, 0.0);
                    let motion = u32::from(e.mem.u8(body + 0xe8));
                    e.mem.set_u32(f + slot::MOTION, motion);
                    let up = up_vector(e);
                    store4(e, f + slot::NORMAL, up);
                    if e.call(MOTION_TYPE_TEST, &args![f + slot::MOTION]).bool() {
                        copy_words(e, plane + 0x10, f + slot::ZERO, 4);
                    }
                }
                slide = false;
            }
            4..=6 => {
                if (e.get(ctl, Ctl::iFlags) >> 16) & 1 == 0 {
                    let mass = e.call(GET_MASS, &args![body + 0xe0]).f32();
                    let limit: f32 = e.global(MAX_PUSH_MASS);
                    if limit > mass {
                        copy_words(e, plane + 0x10, f + slot::ZERO, 4);
                    }
                    e.mem.set_f32(f + slot::NORMAL_Z, 1.0);
                    let up = up_vector(e);
                    store4(e, f + slot::NORMAL, up);
                }
                slide = false;
            }
            _ => {
                if feet && (e.get(ctl, Ctl::iFlags) >> 5) & 1 == 0 {
                    let p = load4(e, point);
                    let limit = e.mem.f32(f + slot::FOOT_DISTANCE);
                    let near = |e: &Engine, foot: u32| {
                        let at = load4(e, f + foot);
                        let d: [f32; 4] = std::array::from_fn(|lane| p[lane] - at[lane]);
                        sum_of_squares3(d) < limit
                    };
                    if near(e, slot::FOOT_0) || near(e, slot::FOOT_1) {
                        let flags = e.get(ctl, Ctl::iFlags);
                        e.set(ctl, Ctl::iFlags, flags | 0x20);
                    }
                }
            }
        }
        let contact = Contact {
            point,
            plane,
            body,
            input,
            item: e.mem.u32(manifold).wrapping_add(offset),
        };
        decide_support(e, ctl, f, &contact, slide, &mut tally);
        index += 1;
    }
    let flags = e.get(ctl, Ctl::iFlags);
    let flags = if count > 0 && layer_two {
        flags | 0x2000_0000
    } else {
        flags & 0xdfff_ffff
    };
    e.set(ctl, Ctl::iFlags, flags);
    let all = e.get(ctl, Ctl::iSupportCount) == tally.steady;
    e.set(ctl, Ctl::bAllSurfacesStopped, u8::from(all));
    let norm = load4(e, me + Ctl::hkSupportNorm.off);
    store4(e, f + slot::SCRATCH, norm);
    let sum = sum_of_squares3(norm);
    if sum != 0.0 {
        let root = e.call(X87_SQRT, &args![sum as f64]).f32();
        let inverse = (1.0 / root as f64) as f32;
        store4(
            e,
            me + Ctl::hkSupportNorm.off,
            norm.map(|lane| inverse * lane),
        );
    }
    e.mem.set_u32(word, saved);
}

/// The two foot points of a three-part list shape whose second and third children are
/// capsules (shape type 5) in `spShapes[0]`, transformed by the phantom (`00c6d620`), and
/// the squared distance limit `(radius * 0.333)^2` of the third. False when any of that is
/// missing.
fn foot_capsules(e: &mut Engine, ctl: Ptr<Ctl>, f: u32) -> bool {
    use constraint_frame as slot;
    let phantom = e.get(ctl, Ctl::spShapePhantom);
    if phantom == 0 {
        return false;
    }
    let hk = e.mem.u32(phantom + 8);
    let shape = e.get(ctl, Ctl::spShapes);
    if hk == 0 || shape == 0 {
        return false;
    }
    let list = e.mem.u32(shape + 8);
    if list == 0 || e.mem.u32(list + 0xc) != 9 {
        return false;
    }
    if e.vcall(list + 0x10, 4, &args![]).u32() != 3 {
        return false;
    }
    let first = e.mem.u32(e.mem.u32(list + 0x18) + 0x10);
    if first == 0 || e.mem.u32(first + 0xc) != 5 {
        return false;
    }
    let transform = hk + 0xb0;
    fn_00c6d620(e, Ptr::new(first), transform, f + slot::FOOT_0);
    let second = e.mem.u32(e.mem.u32(list + 0x18) + 0x20);
    if second == 0 || e.mem.u32(second + 0xc) != 5 {
        return false;
    }
    fn_00c6d620(e, Ptr::new(second), transform, f + slot::FOOT_1);
    let radius = x87_quiet(e.mem.f32(second + 0x10));
    e.mem.set_f32(f + slot::RADIUS, radius);
    let factor: f64 = e.global(FOOT_RADIUS_FACTOR);
    let scaled = (radius as f64 * factor) as f32;
    e.mem.set_f32(
        f + slot::FOOT_DISTANCE,
        (scaled as f64 * scaled as f64) as f32,
    );
    true
}

/// A `DamageImpactData` for a contact with the body whose `+0xC` is `reference`: the
/// body's velocity (`reference + 8`'s `+0x1B0`, copied by `00458620`) less its part along
/// the proxy's own velocity (when that part is positive); when the rest is not zero, a
/// 0x28-byte item (`00c6d9f0`) holds the reference, the contact point and normal and that
/// velocity and is appended to `DamageImpacts` (`00cbc7b0`).
fn record_impact(e: &mut Engine, ctl: Ptr<Ctl>, f: u32, point: u32, reference: u32) {
    use constraint_frame as slot;
    let me = ctl.addr();
    let source = e.mem.u32(reference + 8);
    let source = if source != 0 {
        source + 0x1b0
    } else {
        ZERO_VECTOR
    };
    e.call(COPY_POINT, &args![f + slot::IMPACT, source]);
    let squared = |e: &Engine| {
        let [x, y, z] = [0, 4, 8].map(|o| e.mem.f32(f + slot::IMPACT + o) as f64);
        (z * z + (x * x + y * y)) as f32
    };
    let mut length = squared(e);
    if !is_positive(length) {
        return;
    }
    copy_zero(e, f + slot::VELOCITY, 3);
    let proxy = e.get(ctl, Ctl::phkObject);
    if proxy != 0 {
        e.call(UN_MARK_READ, &args![me]);
        let velocity = e.call(HKP_GET_LINEAR_VELOCITY, &args![proxy]).u32();
        e.call(COPY_POINT, &args![f + slot::VELOCITY, velocity]);
        e.call(UN_MARK_READ, &args![me]);
    }
    let [vx, vy, vz] = [0, 4, 8].map(|o| e.mem.f32(f + slot::IMPACT + o) as f64);
    let [wx, wy, wz] = [0, 4, 8].map(|o| e.mem.f32(f + slot::VELOCITY + o) as f64);
    let dot = ((wx * vx + wy * vy) + wz * vz) as f32 as f64;
    let a = (dot * vx) as f32 as f64;
    let b = (dot * vy) as f32 as f64;
    let c = (dot * vz) as f32 as f64;
    let inverse = (1.0 / length as f64) as f32 as f64;
    let px = (a * inverse) as f32 as f64;
    let py = (b * inverse) as f32 as f64;
    let pz = (inverse * c) as f32 as f64;
    let along = ((py * vy + px * vx) + pz * vz) as f32;
    if 0.0 < along {
        e.mem.set_f32(f + slot::IMPACT, (vx - px) as f32);
        e.mem.set_f32(f + slot::IMPACT + 4, (vy - py) as f32);
        e.mem.set_f32(f + slot::IMPACT + 8, (vz - pz) as f32);
        length = squared(e);
    }
    if !is_positive(length) {
        return;
    }
    let block = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x28u32])
        .u32();
    let item = if block != 0 {
        fn_00c6d9f0(e, Ptr::new(block)).addr()
    } else {
        0
    };
    e.mem.set_u32(f + slot::IMPACT_ITEM, item);
    e.call(NI_POINTER_ASSIGN, &args![item, reference]);
    e.call(COPY_POINT, &args![item.wrapping_add(4), point]);
    e.call(COPY_POINT, &args![item.wrapping_add(0x10), point + 0x10]);
    copy_words(e, item.wrapping_add(0x1c), f + slot::IMPACT, 3);
    e.call(
        LIST_APPEND,
        &args![me + Ctl::DamageImpacts.off, f + slot::IMPACT_ITEM],
    );
}

/// The support probe (`00c716e9`): when the position raised by `fStepHeight` is not below
/// the contact, a ray from the contact plus the first probe offset down by the second is
/// cast against every overlapping collidable of the phantom whose owner (type 1) has
/// motion type 4, in its local space; a hit (fraction below 1) sets `eSupportLayer` and,
/// when the hit's material (last shape key: `00c7ffd0`'s `+0x10`, unless the layer is
/// 0x11; no key: the shape's Havok object's `+0x10`) is 0xF, keeps the highest hit height
/// and its body in the statics. An `hkpClosestRayHitCollector` is set up on the stack and
/// never used.
fn probe_support_bodies(e: &mut Engine, ctl: Ptr<Ctl>, f: u32, point: u32) {
    use constraint_frame as slot;
    bhk_character_controller_get_position_ov2(e, ctl, Ptr::new(f + slot::POSITION));
    let raised =
        (e.get(ctl, Ctl::fStepHeight) as f64 + e.mem.f32(f + slot::POSITION + 8) as f64) as f32;
    e.mem.set_f32(f + slot::POSITION + 8, raised);
    let contact_z = e.mem.f32(point + 8);
    let above = raised >= contact_z;
    if !above {
        return;
    }
    let collector = f + slot::COLLECTOR;
    e.mem.set_f32(collector + 0x20, 1.0);
    e.mem.set_f32(collector + 4, 1.0);
    e.mem.set_u32(collector, VTABLE_CLOSEST_RAY_HIT_COLLECTOR);
    e.mem.set_u32(collector + 0x60, 0);
    e.mem.set_u32(collector + 0x24, u32::MAX);
    e.mem.set_u32(collector + 0x50, 0);
    e.mem.set_u32(collector + 0x30, u32::MAX);
    let array = f + slot::OVERLAPS;
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 4, 0);
    e.mem.set_u32(array + 8, 0x8000_0000);
    let phantom = e.mem.u32(e.get(ctl, Ctl::spShapePhantom) + 8);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let count = e.mem.i32(phantom + 0x164);
    let size = e.mem.i32(array + 4);
    let kept = if count > size { size } else { count };
    let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
    if capacity < count {
        let doubled = capacity * 2;
        let wanted = if count < doubled { doubled } else { count };
        e.call(ARRAY_RESERVE, &args![allocator, array, wanted as u32, 4u32]);
    }
    let source = e.mem.u32(phantom + 0x160);
    let data = e.mem.u32(array);
    for i in 0..count.max(0) as u32 {
        let value = e.mem.u32(source.wrapping_add(4 * i));
        let at = data.wrapping_add(4 * i);
        if i < kept.max(0) as u32 || at != 0 {
            e.mem.set_u32(at, value);
        }
    }
    e.mem.set_u32(array + 4, count as u32);
    let guard: u32 = e.global(PROBE_GUARD);
    let zero = e.global::<u32>(FLOAT_ZERO);
    if guard & 1 == 0 {
        e.set_global(PROBE_GUARD, guard | 1);
        let z = e.global::<u32>(PROBE_RAISE_Z);
        for (lane, bits) in [zero, zero, z, zero].into_iter().enumerate() {
            e.mem.set_u32(PROBE_RAISE + 4 * lane as u32, bits);
        }
    }
    let guard: u32 = e.global(PROBE_GUARD);
    if guard & 2 == 0 {
        e.set_global(PROBE_GUARD, guard | 2);
        let z = e.global::<u32>(PROBE_DROP_Z);
        for (lane, bits) in [zero, zero, z, zero].into_iter().enumerate() {
            e.mem.set_u32(PROBE_DROP + 4 * lane as u32, bits);
        }
    }
    let raise = load4(e, PROBE_RAISE);
    let drop = load4(e, PROBE_DROP);
    let reset = x87_quiet_bits(e.global::<u32>(BEST_SUPPORT_RESET));
    e.set_global(BEST_SUPPORT_HEIGHT, reset);
    let p = load4(e, point);
    let from: [f32; 4] = std::array::from_fn(|lane| p[lane] + raise[lane]);
    store4(e, f + slot::FROM, from);
    let to: [f32; 4] = std::array::from_fn(|lane| from[lane] - drop[lane]);
    store4(e, f + slot::TO, to);
    let mut j = 0;
    while j < e.mem.i32(array + 4) {
        let collidable = e.mem.u32(e.mem.u32(array).wrapping_add(4 * j as u32));
        let owner = collidable_owner(e, collidable, 1);
        if owner != 0 && e.mem.u8(owner + 0xe8) == 4 {
            probe_one_body(e, ctl, f, collidable, owner);
        }
        j += 1;
    }
    free_array_buffer(e, array, 4);
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 8, 0x8000_0000);
}

fn probe_one_body(e: &mut Engine, ctl: Ptr<Ctl>, f: u32, collidable: u32, owner: u32) {
    use constraint_frame as slot;
    let input = f + slot::RAY_INPUT;
    let output = f + slot::RAY_OUTPUT;
    e.mem.set_u32(input + 0x20, 0);
    e.mem.set_f32(output + 0x10, 1.0);
    e.mem.set_u32(input + 0x24, 0);
    e.mem.set_u32(output + 0x40, 0);
    e.mem.set_u32(output + 0x14, u32::MAX);
    e.mem.set_u32(output + 0x20, u32::MAX);
    let transform = owner + 0xf0;
    e.call(
        SET_TRANSFORMED_INVERSE_POS,
        &args![input, transform, f + slot::FROM],
    );
    e.call(
        SET_TRANSFORMED_INVERSE_POS,
        &args![input + 0x10, transform, f + slot::TO],
    );
    let shape = e.mem.u32(collidable);
    e.vcall(shape, 0x20, &args![f + slot::HIT_BOOL, input, output]);
    let fraction = e.mem.f32(output + 0x10);
    let hit = 1.0 > fraction;
    if !hit {
        return;
    }
    let layer = e.mem.u32(owner + 0x2c) & 0x7f;
    e.set(ctl, Ctl::eSupportLayer, layer);
    let mut key = u32::MAX;
    for slot_index in 0..8 {
        let candidate = e.mem.u32(output + 0x20 + 4 * slot_index);
        if candidate == u32::MAX {
            break;
        }
        key = candidate;
    }
    let material = if key == u32::MAX {
        let shape = e.mem.u32(collidable);
        if shape == 0 {
            return;
        }
        let hk = e.mem.u32(shape + 8);
        if hk == 0 {
            return;
        }
        e.mem.u32(hk + 0x10) & 0x1f
    } else {
        if layer == 0x11 {
            return;
        }
        let source = e.call(BODY_MATERIAL_SOURCE, &args![collidable]).u32();
        if source == 0 {
            return;
        }
        e.mem.u32(source + 0x10) & 0x1f
    };
    if material != 0xf {
        return;
    }
    let from_z = e.mem.f32(f + slot::FROM + 8);
    let to_z = e.mem.f32(f + slot::TO + 8);
    let height = from_z + (to_z - from_z) * fraction;
    let best: f32 = e.global(BEST_SUPPORT_HEIGHT);
    if best < height {
        e.set_global(BEST_SUPPORT_HEIGHT, x87_quiet(height));
        e.set_global(BEST_SUPPORT_BODY, owner);
    }
}

/// The plane changes for a contact without a type-1 body (`00c71e3e`): layers 12, 16,
/// 22, 23, 24 and 41 stop the plane (zero velocity at `+0x10`), layer 14 too with flag
/// bit 16; layer 30 (another character, through `00c6d900` on the type-2 owner) stops it
/// entirely when that controller has flag bit 27, else makes its normal horizontal and
/// normalises it (the CRT square root `00ec6040`) unless it is zero.
fn adjust_plane_without_body(
    e: &mut Engine,
    ctl: Ptr<Ctl>,
    f: u32,
    point: u32,
    plane: u32,
    layer: u32,
) {
    use constraint_frame as slot;
    match layer {
        12 | 16 | 22 | 23 | 24 | 41 => copy_words(e, plane + 0x10, f + slot::ZERO, 4),
        14 => {
            if (e.get(ctl, Ctl::iFlags) >> 16) & 1 != 0 {
                copy_words(e, plane + 0x10, f + slot::ZERO, 4);
            }
        }
        30 => {
            let owner = collidable_owner(e, e.mem.u32(point + 0x48), 2);
            let other = fn_00c6d900(e, owner);
            if e.mem.u32(other.wrapping_add(0x414)) & 0x0800_0000 != 0 {
                copy_words(e, plane, f + slot::ZERO, 4);
                copy_words(e, plane + 0x10, f + slot::ZERO, 4);
                return;
            }
            e.mem.set_f32(plane + 8, 0.0);
            let normal = load4(e, plane);
            store4(e, f + slot::SCRATCH, normal);
            let sum = sum_of_squares3(normal);
            if sum == 0.0 {
                return;
            }
            let root = e.call(X87_SQRT, &args![sum as f64]).f32();
            let inverse = (1.0 / root as f64) as f32;
            store4(e, plane, normal.map(|lane| inverse * lane));
        }
        _ => {}
    }
}

/// A contact of `00c711d0`: its point, plane, body, the solver input, and the point
/// read again from the manifold for `00c6e980`.
#[derive(Clone, Copy)]
struct Contact {
    point: u32,
    plane: u32,
    body: u32,
    input: u32,
    item: u32,
}

/// `00c711d0`'s counters: planes added (at most four), supports that stay supports, and
/// whether `hkSupportNorm` already holds a summed normal.
#[derive(Default)]
struct Tally {
    added: i32,
    steady: u32,
    summed: bool,
}

/// The support decision for a body contact (`00c71cc0`).
fn decide_support(
    e: &mut Engine,
    ctl: Ptr<Ctl>,
    f: u32,
    contact: &Contact,
    slide: bool,
    tally: &mut Tally,
) {
    use constraint_frame as slot;
    let me = ctl.addr();
    let Contact {
        point,
        plane,
        body,
        input,
        item,
    } = *contact;
    let normal_z = e.mem.f32(f + slot::NORMAL_Z);
    let level = normal_z >= 0.0;
    if !level {
        return;
    }
    let mut support = true;
    let max_slope = e.get(ctl, Ctl::fMaxSlope);
    if max_slope > normal_z {
        if bhk_character_controller_is_step(e, ctl, Ptr::new(point)) {
            if e.get(ctl, Ctl::iFlags) & 0x1800 == 0 {
                e.mem.set_f32(plane + 0x2c, 0.0);
            }
            e.mem.set_f32(f + slot::NORMAL + 8, 1.0);
            let flags = e.get(ctl, Ctl::iFlags);
            e.set(ctl, Ctl::iFlags, flags | 0x404);
            let normal = normalized(e, load4(e, f + slot::NORMAL));
            store4(e, f + slot::NORMAL, normal);
            store4(e, plane, normal);
        } else {
            if slide || (e.get(ctl, Ctl::iFlags) >> 11) & 1 != 0 {
                let fifth: f64 = e.global(DOUBLE_FIFTH);
                if (normal_z as f64) < fifth {
                    support = false;
                }
            }
            if tally.added < 4 && slide {
                let count = e.mem.u32(input + 0x4c);
                let extra = e
                    .mem
                    .u32(input + 0x48)
                    .wrapping_add(count.wrapping_mul(0x40));
                fn_00c6d670(e, Ptr::new(extra), Ptr::new(plane));
                copy_words(e, extra + 0x10, f + slot::ZERO, 4);
                e.mem.set_f32(extra + 8, 0.0);
                let normal = normalized(e, load4(e, extra));
                tally.added += 1;
                store4(e, extra, normal);
                e.mem.set_u32(input + 0x4c, count.wrapping_add(1));
                if support {
                    tally.steady += 1;
                }
            }
        }
    } else {
        let flags = e.get(ctl, Ctl::iFlags);
        if (flags >> 16) & 1 == 0 && flags & 1 != 0 {
            let listener = me + LISTENER;
            e.call(
                RECORD_SUPPORT_POINT,
                &args![listener, listener + 0x1a0, point],
            );
        }
        let flags = e.get(ctl, Ctl::iFlags) | 0x400;
        e.set(ctl, Ctl::iFlags, flags);
        if (flags >> 3) & 1 == 0 {
            let speed = x87_quiet_bits(e.global::<u32>(PLANE_SPEED_FLAT));
            e.mem.set_u32(plane + 0x28, speed);
        }
    }
    if e.call(GET_STATE, &args![me + 0x3e0]).u32() == 0 {
        let scaled = (e.get(ctl, Ctl::fSpeedPct) as f64 * e.mem.f32(plane + 0x2c) as f64) as f32;
        e.mem.set_f32(plane + 0x2c, scaled);
    }
    if !support {
        return;
    }
    let flags = e.get(ctl, Ctl::iFlags);
    e.set(ctl, Ctl::iFlags, flags | 0x200);
    let supports = e.get(ctl, Ctl::iSupportCount);
    e.set(ctl, Ctl::iSupportCount, supports.wrapping_add(1));
    fn_00c6e980(e, ctl, item);
    let normal = load4(e, f + slot::NORMAL);
    let total = if tally.summed {
        let sum = load4(e, me + Ctl::hkSupportNorm.off);
        std::array::from_fn(|lane| normal[lane] + sum[lane])
    } else {
        tally.summed = true;
        normal
    };
    store4(e, me + Ctl::hkSupportNorm.off, total);
    if (e.get(ctl, Ctl::iFlags) >> 16) & 1 != 0 {
        e.call(ADD_REFERENCE, &args![body]);
        let old = e.get(ctl, Ctl::spSupportBody);
        if old != 0 {
            e.call(REMOVE_REFERENCE, &args![old]);
        }
        e.set(ctl, Ctl::spSupportBody, body);
    }
}

// Translated from 00c72210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees the `hkArray` of 8-byte entries at `+0xC`.
pub fn fn_00c72210(e: &mut Engine, this: Ptr) {
    free_hk_array(e, this.addr() + 0xc, 8);
}

/// `NiPointer` assignment as inlined: when `value` differs from the pointer at `slot`, the
/// old object is released, `value` stored and its count raised (`InterlockedIncrement`).
fn assign_ni_pointer(e: &mut Engine, slot: u32, value: u32) {
    let old = e.mem.u32(slot);
    if old == value {
        return;
    }
    if old != 0 {
        release_ni_object(e, old);
    }
    e.mem.set_u32(slot, value);
    if value != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![value + 4]);
    }
}

/// The inlined `hkArray` destructor of a stack cinfo: size 0, and the buffer
/// (`capacity * element` bytes) freed through the Havok allocator's slot 0x10 unless the
/// flags word is negative.
fn free_array_buffer(e: &mut Engine, array: u32, element: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    e.mem.set_u32(array + 4, 0);
    let capacity = e.mem.i32(array + 8);
    if capacity >= 0 {
        let data = e.mem.u32(array);
        let bytes = (capacity as u32 & 0x3fff_ffff).wrapping_mul(element);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
}

/// `hkArray` push-back of a word as inlined (`00c90940` makes room when full).
fn push_word(e: &mut Engine, array: u32, value: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    if e.mem.u32(array + 4) == e.mem.u32(array + 8) & 0x3fff_ffff {
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 4u32]);
    }
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    e.mem
        .set_u32(data.wrapping_add(size.wrapping_mul(4)), value);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

/// A capsule from a fresh 0x14-byte block (`00cab710` with the two vertices and the
/// radius's bits), or null when the allocation failed.
fn new_capsule(e: &mut Engine, first: u32, second: u32, radius: u32) -> u32 {
    let block = e.call(HEAP_ALLOCATE, &args![0x14u32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(CAPSULE_CONSTRUCT, &args![block, first, second, radius])
        .u32()
}

/// The Havok object (`+8`) of a shape, or 0 for a null shape.
fn hk_of(e: &Engine, shape: u32) -> u32 {
    if shape != 0 {
        e.mem.u32(shape + 8)
    } else {
        0
    }
}

/// Slots of `00c72410`'s stack frame (offsets from its `ESP` after the prologue, which
/// this translation keeps so that the slot the exe reads uninitialised stays one).
mod frame {
    /// Twice the larger horizontal size (or the height); `fHeight` is it times the scale.
    pub const LENGTH: u32 = 0x1c;
    pub const TEMP: u32 = 0x20;
    pub const RADIUS: u32 = 0x24;
    pub const LONGER: u32 = 0x28;
    /// Capsule vertices: `A` (`+0x2C`), `B` (`+0x40`), `C` (`+0x88`), `D` (`+0x94`).
    pub const A: u32 = 0x2c;
    pub const PLUS_ONE: u32 = 0x38;
    pub const B: u32 = 0x40;
    /// The `bhkListShape` cinfo: a word, then two `hkArray`s of words.
    pub const LIST_INFO: u32 = 0x4c;
    pub const WIDTH: u32 = 0x70;
    pub const DEPTH: u32 = 0x74;
    pub const HEIGHT: u32 = 0x78;
    pub const CENTER: u32 = 0x7c;
    pub const C: u32 = 0x88;
    pub const D: u32 = 0x94;
    /// The collision box's cinfo (`0056e0b0`).
    pub const BOX_INFO: u32 = 0xa4;
    pub const SIZE: u32 = 0x110;
}

// Translated from 00c72410 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c72410 - the FS:[0] exception-unwinding frame and the /GS cookie check are not modeled.
/// Builds the controller's shapes from the cinfo `info` (called by `Init`).
/// `eSizedShapeType` becomes 2. With a shape at `info + 0xA0` it becomes `spShapes[0]` and
/// the radius is its capsule radius (0.1 without one) times the float at `011b0194`; the
/// height term is 0 and the half height is the stack slot the exe never writes on this
/// path. Otherwise the sizes come from `info + 0x80`'s bound (`00c55170`, scaled by
/// `info + 0xAC`; without it 25 times the scale and the `w` of its `+0x20` vector, or 23,
/// 17.5 and 64 times the scale or 1), flag bit 0 is set when the height is below the
/// width or the depth above the height, and capsules are made (`00cab710`): upright
/// (no bit 0) between `A` and `B`, also the convex shape `00c70de0` as `spShapes[1]`
/// (shared through `01267be0` when `info + 0x96` is set); lying (bit 0) between the
/// vertices along the longer horizontal axis, or, when 0.75 of the longer size exceeds
/// the radius, a `bhkListShape` of three capsules. `RotCenter`, `fCenter`, `info + 0x50`,
/// `fRadius`, `fDestRadius`, `fHeight`, `fActorHeight` and `fStepHeight` follow, then
/// `SetShapeType`; with `info + 0x95` a collision box (`00c6d1e0`) with a sphere shape is
/// made.
pub fn fn_00c72410(e: &mut Engine, this: Ptr<Ctl>, info: u32) {
    e.with_stack(frame::SIZE, |e, frame| {
        build_shapes(e, this, info, frame.addr());
    });
}

fn build_shapes(e: &mut Engine, this: Ptr<Ctl>, info: u32, f: u32) {
    let me = this.addr();
    e.set(this, Ctl::eSizedShapeType, 2);
    let given = e.mem.u32(info + 0xa0);
    if given != 0 {
        assign_ni_pointer(e, me + Ctl::spShapes.off, given);
        let shape = bhk_character_controller_get_shape(e, this);
        let hk = hk_of(e, shape);
        let radius = if hk != 0 {
            x87_quiet(e.mem.f32(hk + 0x10))
        } else {
            x87_quiet(e.global::<f32>(DEFAULT_RADIUS))
        };
        let scale: f32 = e.global(SCALE);
        e.mem
            .set_f32(f + frame::RADIUS, (radius as f64 * scale as f64) as f32);
        e.mem.set_f32(f + frame::LENGTH, 0.0);
    } else {
        measure_and_build(e, this, info, f);
    }
    let k: f32 = e.global(INVERSE_SCALE);
    let radius = e.mem.f32(f + frame::RADIUS);
    let half_height = e.mem.f32(f + frame::HEIGHT);
    let scaled = (radius as f64 * k as f64) as f32;
    e.set(this, Ctl::fRadius, scaled);
    e.set(this, Ctl::fDestRadius, scaled);
    let length = e.mem.f32(f + frame::LENGTH);
    e.set(this, Ctl::fHeight, (k as f64 * length as f64) as f32);
    let doubled = (half_height as f64 + half_height as f64) as f32;
    e.set(this, Ctl::fActorHeight, (k as f64 * doubled as f64) as f32);
    let kind = if e.mem.u32(info + 0xa0) != 0 {
        0
    } else {
        e.mem.i32(info + 0xb0)
    };
    bhk_character_controller_set_shape_type(e, this, kind);
    if e.mem.u8(info + 0x94) != 0 {
        e.call(MEMORY_ROUTER, &args![]);
    }
    if e.mem.u8(info + 0x95) != 0 {
        build_collision_box(e, this, info, f);
    }
}

/// Width, depth and height, and the centre's x and y words (the default point's, or the
/// bound's centre scaled, which is also written to the frame's `C`).
fn measure(
    e: &mut Engine,
    this: Ptr<Ctl>,
    info: u32,
    f: u32,
    default: [u32; 2],
) -> ([f32; 3], [u32; 2]) {
    let object = e.mem.u32(info + 0x80);
    let fixed = e.mem.u8(info + 0x96) != 0;
    if object == 0 || fixed {
        let s = if fixed {
            1.0
        } else {
            x87_quiet(e.mem.f32(info + 0xac))
        };
        let w = (e.global::<f64>(DEFAULT_WIDTH) * s as f64) as f32;
        let d = (e.global::<f64>(DEFAULT_DEPTH) * s as f64) as f32;
        let h = (s as f64 * e.global::<f64>(DEFAULT_HEIGHT)) as f32;
        return ([w, d, h], default);
    }
    let bound = e.call(BOUND_OF, &args![object]).u32();
    if bound == 0 {
        let vector = e.mem.u32(object + 0x20);
        let vector = if vector != 0 { vector } else { POINT_FALLBACK };
        let extent = x87_quiet(e.mem.f32(vector + 0xc));
        let depth: f64 = e.global(PROBE_DEPTH);
        let t = (e.mem.f32(info + 0xac) as f64 * depth) as f32;
        let sizes = if e.get(this, Ctl::iFlags) & 1 != 0 {
            [t, extent, t]
        } else {
            [t, t, extent]
        };
        return (sizes, default);
    }
    let s = x87_quiet(e.mem.f32(info + 0xac)) as f64;
    let b = |e: &Engine, o: u32| e.mem.f32(bound + o) as f64;
    let w = (s * b(e, 0x18)) as f32;
    let d = (b(e, 0x1c) * s) as f32;
    let h = (s * b(e, 0x20)) as f32;
    let s = x87_quiet(e.mem.f32(info + 0xac)) as f64;
    let center = [
        (b(e, 0xc) * s) as f32,
        (b(e, 0x10) * s) as f32,
        (s * b(e, 0x14)) as f32,
    ];
    for (lane, value) in center.into_iter().enumerate() {
        e.mem.set_f32(f + frame::C + 4 * lane as u32, value);
    }
    ([w, d, h], [center[0].to_bits(), center[1].to_bits()])
}

/// The sizes, then the shapes, when `info` brings no shape (`00c7250c` on).
fn measure_and_build(e: &mut Engine, this: Ptr<Ctl>, info: u32, f: u32) {
    let me = this.addr();
    let x0 = e.global::<u32>(POINT_DEFAULT);
    let y0 = e.global::<u32>(POINT_DEFAULT + 4);
    let z0 = e.global::<u32>(POINT_DEFAULT + 8);
    let ([w, d, h], [cx, cy]) = measure(e, this, info, f, [x0, y0]);
    e.mem.set_f32(f + frame::WIDTH, w);
    e.mem.set_f32(f + frame::DEPTH, d);
    e.mem.set_f32(f + frame::HEIGHT, h);
    e.mem.set_u32(f + frame::CENTER, cx);
    e.mem.set_u32(f + frame::CENTER + 4, cy);
    e.mem.set_u32(f + frame::A, x0);
    e.mem.set_u32(f + frame::A + 4, y0);
    e.mem.set_u32(f + frame::A + 8, z0);
    let (w, d, h) = (x87_quiet(w), x87_quiet(d), x87_quiet(h));
    if h < w || d > h {
        let flags = e.get(this, Ctl::iFlags);
        e.set(this, Ctl::iFlags, flags | 1);
    }
    if e.get(this, Ctl::iFlags) & 1 == 0 {
        standing_capsule(e, this, f, [w, d, h], [x0, y0]);
    } else {
        lying_capsules(e, this, info, f, [w, d, h], [cx, cy]);
    }
    let k: f32 = e.global(INVERSE_SCALE);
    for lane in [0, 4] {
        let value = (e.mem.f32(f + frame::CENTER + lane) as f64 * k as f64) as f32;
        e.mem.set_f32(me + Ctl::RotCenter.off + lane, value);
    }
    if e.get(this, Ctl::spShapes) != 0 {
        return;
    }
    let radius = e.mem.f32(f + frame::RADIUS);
    let z = e.mem.f32(f + frame::A + 8);
    let raised = (z as f64 + radius as f64) as f32;
    let scaled = (raised as f64 * k as f64) as f32;
    let extra = e.mem.f32(info + 0x4c) as f64;
    let center = (scaled as f64 + (extra + extra)) as f32;
    set_center(e, this, center);
    if 0.0 == e.mem.f32(info + 0x50) {
        e.mem
            .set_f32(info + 0x50, (k as f64 * radius as f64) as f32);
    }
    let radius_bits = x87_quiet_bits(e.mem.u32(f + frame::RADIUS));
    if e.get(this, Ctl::iFlags) & 1 != 0 {
        e.mem.set_u32(info + 0xb0, 0);
        let capsule = new_capsule(e, f + frame::A, f + frame::B, radius_bits);
        let index = if e.mem.u32(info + 0xa0) != 0 {
            0
        } else {
            e.mem.u32(info + 0xb0)
        };
        let slot = (me + Ctl::spShapes.off).wrapping_add(index.wrapping_mul(4));
        e.call(NI_POINTER_ASSIGN, &args![slot, capsule]);
        return;
    }
    let capsule = new_capsule(e, f + frame::A, f + frame::B, radius_bits);
    e.call(NI_POINTER_ASSIGN, &args![me + Ctl::spShapes.off, capsule]);
    let step = x87_quiet(e.get(this, Ctl::fStepHeight));
    let radius = x87_quiet(e.mem.f32(f + frame::RADIUS));
    let second = me + Ctl::spShapes.off + 4;
    if e.mem.u8(info + 0x96) != 0 {
        if e.global::<u32>(STATIC_NI_POINTER) == 0 {
            let shape = fn_00c70de0(
                e,
                Ptr::new(f + frame::A),
                Ptr::new(f + frame::B),
                radius,
                step,
            );
            e.call(NI_POINTER_ASSIGN, &args![STATIC_NI_POINTER, shape]);
        }
        e.call(NI_POINTER_COPY, &args![second, STATIC_NI_POINTER]);
    } else {
        let shape = fn_00c70de0(
            e,
            Ptr::new(f + frame::A),
            Ptr::new(f + frame::B),
            radius,
            step,
        );
        e.call(NI_POINTER_ASSIGN, &args![second, shape]);
    }
}

/// `fCenter` = `center`; `RotCenter.z += center`.
fn set_center(e: &mut Engine, this: Ptr<Ctl>, center: f32) {
    e.set(this, Ctl::fCenter, center);
    let z = this.addr() + Ctl::RotCenter.off + 8;
    let raised = (center as f64 + e.mem.f32(z) as f64) as f32;
    e.mem.set_f32(z, raised);
}

/// Without flag bit 0 (`00c72ce2`): the vertices `A` and `B` sit at `(x0, y0, ±z)` with
/// `z = (height - radius) + z0`, the radius is half of width plus depth, and
/// `fStepHeight` rises to `0.75 * radius` in Havok units when that is larger.
fn standing_capsule(
    e: &mut Engine,
    this: Ptr<Ctl>,
    f: u32,
    [w, d, h]: [f32; 3],
    [x0, y0]: [u32; 2],
) {
    e.mem
        .set_f32(f + frame::LENGTH, (h as f64 + h as f64) as f32);
    e.mem.set_u32(f + frame::B, x0);
    e.mem.set_u32(f + frame::B + 4, y0);
    let half: f64 = e.global(HALF);
    let radius = ((d as f64 + w as f64) * half) as f32;
    e.mem.set_f32(f + frame::RADIUS, radius);
    let z0 = e.mem.f32(f + frame::A + 8);
    let z = ((h as f64 - radius as f64) + z0 as f64) as f32;
    e.mem.set_f32(f + frame::A + 8, z);
    let minus_one: f64 = e.global(MINUS_ONE);
    e.mem
        .set_f32(f + frame::B + 8, (z as f64 * minus_one) as f32);
    let three_quarters: f64 = e.global(THREE_QUARTERS);
    let quarter_less = (three_quarters * radius as f64) as f32;
    let k: f32 = e.global(INVERSE_SCALE);
    let step = (quarter_less as f64 * k as f64) as f32;
    e.mem.set_f32(f + frame::TEMP, step);
    if step > e.get(this, Ctl::fStepHeight) {
        e.set(this, Ctl::fStepHeight, step);
    }
}

/// With flag bit 0 (`00c72722`): the vertices lie along the longer of width and depth
/// (`A` at `longer - radius` past the centre, `B` mirrored), the radius is the smaller
/// of the other size and the height; when `0.75 *` the larger of those exceeds the radius,
/// three capsules make a `bhkListShape` in `spShapes`.
fn lying_capsules(
    e: &mut Engine,
    this: Ptr<Ctl>,
    info: u32,
    f: u32,
    [w, d, h]: [f32; 3],
    [cx, cy]: [u32; 2],
) {
    e.mem.set_u32(f + frame::A, cx);
    e.mem.set_u32(f + frame::A + 4, cy);
    e.mem.set_f32(f + frame::A + 8, 0.0);
    let two: f64 = e.global(TWO);
    let minus_one: f64 = e.global(MINUS_ONE);
    let along_x = w > d;
    let (along, other) = if along_x { (w, d) } else { (d, w) };
    e.mem
        .set_f32(f + frame::LENGTH, (along as f64 * two) as f32);
    let (radius, longer) = if other < h { (other, h) } else { (h, other) };
    e.mem.set_f32(f + frame::RADIUS, radius);
    e.mem.set_f32(f + frame::LONGER, longer);
    let lane = if along_x { 0 } else { 4 };
    let base = f32::from_bits(if along_x { cx } else { cy });
    let end = ((along as f64 - radius as f64) + base as f64) as f32;
    e.mem.set_f32(f + frame::A + lane, end);
    e.mem.set_u32(f + frame::B, cx);
    e.mem.set_u32(f + frame::B + 4, cy);
    e.mem
        .set_f32(f + frame::B + lane, (end as f64 * minus_one) as f32);
    e.mem.set_f32(f + frame::B + 8, 0.0);
    let three_quarters: f64 = e.global(THREE_QUARTERS);
    let wide = three_quarters * longer as f64 > radius as f64;
    if !wide {
        return;
    }
    let length = (longer as f64 - radius as f64) as f32;
    e.mem.set_f32(f + frame::LONGER, length);
    let zero: f64 = e.global(DOUBLE_ZERO);
    let z = (length as f64 * two + radius as f64 + zero) as f32;
    e.mem.set_f32(f + frame::A + 8, z);
    e.mem.set_f32(f + frame::B + 8, z);
    let center = (two * e.mem.f32(info + 0x4c) as f64) as f32;
    set_center(e, this, center);
    if 0.0 == e.mem.f32(info + 0x50) {
        let k: f32 = e.global(INVERSE_SCALE);
        e.mem
            .set_f32(info + 0x50, (radius as f64 * k as f64) as f32);
    }
    let list = f + frame::LIST_INFO;
    for (offset, word) in [0, 0, 0, 0x8000_0000, 0, 0, 0x8000_0000]
        .into_iter()
        .enumerate()
    {
        e.mem.set_u32(list + 4 * offset as u32, word);
    }
    let radius_bits = x87_quiet_bits(e.mem.u32(f + frame::RADIUS));
    let first = new_capsule(e, f + frame::A, f + frame::B, radius_bits);
    // The middle capsule: `A` 8 lower in y, from `length + 1` down to `length - 1` in z.
    let length = x87_quiet(e.mem.f32(f + frame::LONGER));
    e.mem.set_f32(f + frame::A + 8, length);
    let ax = e.mem.u32(f + frame::A);
    e.mem.set_f32(f + frame::C + 8, length);
    let eight: f64 = e.global(EIGHT);
    let one: f64 = e.global(DOUBLE_ONE);
    let ay = (e.mem.f32(f + frame::A + 4) as f64 - eight) as f32;
    e.mem.set_u32(f + frame::C, ax);
    e.mem.set_f32(f + frame::A + 4, ay);
    e.mem.set_f32(f + frame::C + 4, ay);
    let plus_one = (length as f64 + 1.0) as f32;
    e.mem.set_f32(f + frame::PLUS_ONE, plus_one);
    e.mem.set_f32(f + frame::A + 8, plus_one);
    let minus = (e.mem.f32(f + frame::C + 8) as f64 - 1.0) as f32;
    e.mem.set_f32(f + frame::C + 8, minus);
    let shorter = (e.mem.f32(f + frame::LONGER) as f64 - one) as f32;
    e.mem.set_f32(f + frame::TEMP, shorter);
    let second = new_block_capsule(e, f + frame::A, f + frame::C, shorter);
    // The third: `B` 8 higher in y.
    let length = x87_quiet(e.mem.f32(f + frame::LONGER));
    let bx = e.mem.u32(f + frame::B);
    e.mem.set_f32(f + frame::B + 8, length);
    e.mem.set_f32(f + frame::D + 8, length);
    let by = (e.mem.f32(f + frame::B + 4) as f64 + eight) as f32;
    e.mem.set_u32(f + frame::D, bx);
    e.mem.set_f32(f + frame::B + 4, by);
    e.mem.set_f32(f + frame::D + 4, by);
    let plus_one = x87_quiet(e.mem.f32(f + frame::PLUS_ONE));
    e.mem.set_f32(f + frame::B + 8, plus_one);
    let minus = (e.mem.f32(f + frame::D + 8) as f64 - one) as f32;
    e.mem.set_f32(f + frame::D + 8, minus);
    let shorter = (e.mem.f32(f + frame::LONGER) as f64 - one) as f32;
    e.mem.set_f32(f + frame::TEMP, shorter);
    let third = new_block_capsule(e, f + frame::B, f + frame::D, shorter);
    for capsule in [first, second, third] {
        let hk = hk_of(e, capsule);
        push_word(e, list + 4, hk);
    }
    for _ in 0..3 {
        push_word(e, list + 0x10, 0);
    }
    e.mem.set_u32(info + 0xb0, 0);
    let block = e.call(HEAP_ALLOCATE, &args![0x14u32]).u32();
    let shape = if block != 0 {
        fn_00c6d290(e, Ptr::new(block), list).addr()
    } else {
        0
    };
    let index = if e.mem.u32(info + 0xa0) != 0 {
        0
    } else {
        e.mem.u32(info + 0xb0)
    };
    let slot = (this.addr() + Ctl::spShapes.off).wrapping_add(index.wrapping_mul(4));
    e.call(NI_POINTER_ASSIGN, &args![slot, shape]);
    fn_00c71120(e, Ptr::new(list));
}

/// A capsule whose radius argument is `radius` (computed before the allocation).
fn new_block_capsule(e: &mut Engine, first: u32, second: u32, radius: f32) -> u32 {
    new_capsule(e, first, second, radius.to_bits())
}

/// The collision box `00c72410` makes when `info + 0x95` is set: a cinfo (`0056e0b0`) with
/// the filter word `(info + 0x84) & !0x7F | 0x20` and an `hkpSphereShape` (0x20 bytes from
/// the Havok allocator, its `+4` half-word 0x20) whose radius is the `w` of the bound
/// object's `+0x20` vector when above 0, else 1, times the inverse scale; the box
/// (`00c6d1e0`, a 16-byte-aligned 0x30-byte `MemoryManager` block with the offset in the
/// byte before it) becomes `spCollisionBox`. Without a bound object the proxy's position
/// is read (between two `UnMarkRead` calls) and dropped.
fn build_collision_box(e: &mut Engine, this: Ptr<Ctl>, info: u32, f: u32) {
    let me = this.addr();
    let cinfo = f + frame::BOX_INFO;
    e.call(PHANTOM_CINFO_CONSTRUCT, &args![cinfo]);
    let filter = e.mem.u32(info + 0x84);
    let object = e.mem.u32(info + 0x80);
    e.mem.set_u32(cinfo, (filter & 0xffff_ff80) | 0x20);
    e.mem.set_u32(cinfo + 4, 0);
    let mut positive = false;
    if object != 0 {
        let vector = e.mem.u32(object + 0x20);
        let vector = if vector != 0 { vector } else { POINT_FALLBACK };
        copy_words(e, f + frame::D, vector, 4);
        let extent = x87_quiet(e.mem.f32(f + frame::D + 0xc));
        e.mem.set_f32(f + frame::LENGTH, extent);
        positive = 0.0 < extent;
    } else {
        let proxy = e.get(this, Ctl::phkObject);
        if proxy != 0 {
            e.call(UN_MARK_READ, &args![this]);
            e.call(HKP_GET_POSITION, &args![proxy]);
            e.call(UN_MARK_READ, &args![this]);
        }
    }
    if !positive {
        e.mem.set_f32(f + frame::LENGTH, 1.0);
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let sphere = e.vcall(allocator, 4, &args![0x20u32]).u32();
    e.mem.set_u16(sphere.wrapping_add(4), 0x20);
    let k: f32 = e.global(INVERSE_SCALE);
    let radius = (e.mem.f32(f + frame::LENGTH) as f64 * k as f64) as f32;
    e.mem.set_f32(f + frame::TEMP, radius);
    let shape = e
        .call(SPHERE_CONSTRUCT, &args![sphere, radius.to_bits()])
        .u32();
    e.mem.set_u32(cinfo + 4, shape);
    let block = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x30u32])
        .u32();
    let offset = 0x10u8.wrapping_sub(block as u8 & 0xf);
    let block = block.wrapping_add(u32::from(offset));
    e.mem.set_u8(block.wrapping_sub(1), offset);
    let collision_box = fn_00c6d1e0(e, Ptr::new(block), cinfo, me).addr();
    assign_ni_pointer(e, me + Ctl::spCollisionBox.off, collision_box);
    free_array_buffer(e, cinfo + 0xc, 8);
}

// Translated from 00c72250 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c72250 - the FS:[0] exception-unwinding frame and the /GS cookie check are not modeled.
/// `bhkCharacterController::MoveAsProjectile` (Xbox PDB): the target is the position
/// (`GetPosition_ov2`) plus `OutVelocity * time` (single precision). Without flag bit 19 or
/// a phantom the controller is just put there. Otherwise the phantom casts towards it
/// (`00c9eba0` with an `hkpAllCdPointCollector` of eight inline hits on the stack; the
/// cast input's two tolerances from `0104da4c`); when it hit, the hits are sorted and each
/// goes to the listener's slot 8 (`(0, hit)`, 0x70-byte hits); then the controller is put
/// at the target and the collector destroyed.
pub fn bhk_character_controller_move_as_projectile(e: &mut Engine, this: Ptr<Ctl>, time: f32) {
    let flags = e.get(this, Ctl::iFlags);
    if (flags >> 0x13) & 1 == 0 || e.get(this, Ctl::spShapePhantom) == 0 {
        e.with_stack(0x10, |e, position| {
            bhk_character_controller_get_position_ov2(e, this, position);
            let velocity = load4(e, this.addr() + Ctl::OutVelocity.off);
            let p = load4(e, position.addr());
            let target: [f32; 4] = std::array::from_fn(|lane| time * velocity[lane] + p[lane]);
            store4(e, position.addr(), target);
            bhk_character_controller_set_position_ov2(e, this, position);
        });
        return;
    }
    e.with_stack(0x3d0, |e, frame| {
        let position = frame.addr();
        let input = position + 0x10;
        let collector = position + 0x30;
        let far = x87_quiet_bits(e.global::<u32>(COLLECTOR_FAR));
        e.mem.set_u32(collector + 4, far);
        e.mem.set_u32(collector, VTABLE_ALL_CD_POINT_COLLECTOR);
        e.mem.set_u32(collector + 0x10, collector + 0x20);
        e.mem.set_u32(collector + 0x18, 0x8000_0008);
        e.mem.set_u32(collector + 0x14, 0);
        let tolerance = x87_quiet_bits(e.global::<u32>(CAST_TOLERANCE));
        e.mem.set_u32(input + 0x10, tolerance);
        e.mem.set_u32(input + 0x14, tolerance);
        bhk_character_controller_get_position_ov2(e, this, frame);
        let velocity = load4(e, this.addr() + Ctl::OutVelocity.off);
        let p = load4(e, position);
        let target: [f32; 4] = std::array::from_fn(|lane| p[lane] + velocity[lane] * time);
        store4(e, input, target);
        let phantom = e.get(this, Ctl::spShapePhantom);
        e.call(
            PHANTOM_LINEAR_CAST,
            &args![phantom, position, input, collector, 0u32],
        );
        if e.mem.i32(collector + 0x14) > 0 {
            e.call(ALL_COLLECTOR_SORT_HITS, &args![collector]);
            let mut index = 0;
            let mut offset = 0u32;
            while index < e.mem.i32(collector + 0x14) {
                let hits = e.mem.u32(collector + 0x10);
                e.vcall(
                    this.addr() + LISTENER,
                    8,
                    &args![0u32, hits.wrapping_add(offset)],
                );
                index += 1;
                offset += 0x70;
            }
        }
        bhk_character_controller_set_position_ov2(e, this, Ptr::new(input));
        e.call(ALL_COLLECTOR_DESTRUCT, &args![collector]);
    });
}

/// Low word of the time-stamp counter (`RDTSC`); the engine has no model of it and the
/// host clock stands in (only the profiling records below take it).
fn time_stamp_low() -> u32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos() as u32)
        .unwrap_or(0)
}

/// Appends a profiling record (`label`, time stamp) to the thread's record stream
/// (`TlsGetValue` of the index at `0126835c`) while its cursor (`+4`) is below its limit
/// (`+0xC`); a record is 12 bytes.
fn profile_record(e: &mut Engine, label: u32) {
    let slot: u32 = e.global(PROFILE_SLOT);
    let stream = e.call(TLS_GET_VALUE, &args![slot]).u32();
    let cursor = e.mem.u32(stream + 4);
    if cursor < e.mem.u32(stream + 0xc) {
        e.mem.set_u32(cursor, label);
        e.mem.set_u32(cursor + 4, time_stamp_low());
        e.mem.set_u32(stream + 4, cursor + 12);
    }
}

/// `v` rotated by the quaternion `q` as the inlined SSE code does, lane by lane:
/// `2 * ((b - a) * qw + ((qw * qw + bias) * v + dot * q))` with `a = (vy*qz, vz*qx,
/// vx*qy, vw*qw)`, `b = (vz*qy, vx*qz, vy*qx, vw*qw)`, `dot = vz*qz + (vy*qy + vx*qx)`
/// and `bias` the four floats at `010c71e0`.
fn rotate_by(e: &Engine, v: [f32; 4], q: [f32; 4]) -> [f32; 4] {
    let bias = load4(e, ROTATION_BIAS);
    let products: [f32; 4] = std::array::from_fn(|lane| v[lane] * q[lane]);
    let dot = products[2] + (products[1] + products[0]);
    let a = [v[1] * q[2], v[2] * q[0], v[0] * q[1], v[3] * q[3]];
    let b = [v[2] * q[1], v[0] * q[2], v[1] * q[0], v[3] * q[3]];
    let w = q[3];
    std::array::from_fn(|lane| {
        let cross = (b[lane] - a[lane]) * w;
        let rest = (w * w + bias[lane]) * v[lane] + dot * q[lane];
        let sum = cross + rest;
        sum + sum
    })
}

/// The `MOVMSKPS` bits of `|a - b| <= tolerance` per lane (`ANDPS` with the mask at
/// `01017d10`, `CMPLEPS`; NaN lanes fail).
fn near_lanes(e: &Engine, a: [f32; 4], b: [f32; 4], tolerance: [f32; 4]) -> u32 {
    let mask = [0, 4, 8, 12].map(|o| e.mem.u32(ABS_MASK + o));
    (0..4).fold(0, |bits, lane| {
        let difference = f32::from_bits((a[lane] - b[lane]).to_bits() & mask[lane]);
        if difference <= tolerance[lane] {
            bits | 1 << lane
        } else {
            bits
        }
    })
}

/// The state object of the context's current state (`00cbc510`), whose slot 0x20 then
/// updates the controller.
fn update_character_state(e: &mut Engine, this: Ptr<Ctl>) {
    let state = e.get(this, Ctl::m_currentState);
    let manager = e.get(this, Ctl::m_stateManager);
    let object = e.call(STATE_LOOKUP, &args![manager, state]).u32();
    e.vcall(object, 0x20, &args![this]);
}

// Translated from 00c73170 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c73170 - the /GS cookie check is not modeled; the RDTSC time stamps of its profiling records come from the host clock.
/// `bhkCharacterController::Move` (Xbox PDB) with the move parameters `params` (time step
/// `+0`, turn `+4`, rotation `+0xC`, velocity `+0x10`, speed `+0x20`, byte `+0x24`).
/// Returns 2 when flag 0x40 or 0x80 was set by the move, else 0. In context state 4 the
/// controller is just moved by the velocity (`SetPosition`), its state updated and
/// `OutVelocity` set to the velocity over the step (scaled). Otherwise, for a step above
/// 0.0001 (or NaN) in a world (slot 0x94), and unless flag bit 22's blocker (`00621ad0`)
/// holds: impacts, bumped body and proxy are cleared, `fSpeedPct` set, the controller is
/// "still" when the velocity and `VelocityMod` are about zero (flag 8 when not), the size
/// changes, the rotation modifier turns the heading (`setAxisAngle` about `UpVec`), the
/// direction is the velocity over the step (no z unless allowed), turned by `+4` in states
/// 4 to 6, the forward vector follows the heading (pitch and roll applied when enabled;
/// a change resets `fCalculatePitchTimer`). With flag bit 19 the controller moves as a
/// projectile; otherwise the support is checked (`CheckSupport` along `-UpVec`, flags
/// 0x100 and 0x2000), the state updated, and the proxy integrated unless a still
/// controller's manifold is unchanged (then the velocity is zeroed); a support body found
/// by the probe lifts the proxy to its height; up to five manifold points are kept in
/// `LastManifold`. Then pitch and roll, the velocity-modifier time and the pitch timer
/// run down by the step, and flag 0x1000 is cleared.
pub fn bhk_character_controller_move(e: &mut Engine, this: Ptr<Ctl>, params: Ptr) -> u32 {
    e.with_stack(0xa0, |e, frame| {
        move_controller(e, this, params.addr(), frame.addr())
    })
}

fn move_controller(e: &mut Engine, this: Ptr<Ctl>, p: u32, f: u32) -> u32 {
    let me = this.addr();
    let state = e.call(GET_STATE, &args![me + 0x3e0]).u32();
    if state == 4 {
        move_in_state_four(e, this, p, f);
        return 0;
    }
    let flags = e.get(this, Ctl::iFlags);
    e.set(this, Ctl::iFlags, flags & 0xffff_ff3f);
    fn_00c70bd0(e, this);
    release_ref_slot(e, me + Ctl::spBumpedBody.off);
    e.set(this, Ctl::fBumpedForce, 0.0);
    release_ref_slot(e, me + Ctl::spBumpedCharProxy.off);
    let divisor: f32 = e.global(SPEED_DIVISOR);
    // `FDIV dword` (`00c7333c`).
    let speed = x87_div(x87_load32(e.mem.u32(p + 0x20)), x87_m32(divisor.to_bits())) as f32;
    e.set(this, Ctl::fSpeedPct, speed);
    let step = x87_quiet(e.mem.f32(p));
    e.mem.set_f32(f + 0x38, step);
    let world = e.vcall(me, 0x94, &args![]).u32();
    let minimum: f64 = e.global(MIN_STEP);
    let time = e.mem.f32(p);
    let long_enough = time as f64 > minimum || time.is_nan();
    if !long_enough || world == 0 {
        return 0;
    }
    if (e.get(this, Ctl::iFlags) >> 0x16) & 1 != 0 {
        let blocker = e.call(MOVE_BLOCKER, &args![me]).u32();
        if e.mem.u32(blocker + 0x30) != 0 {
            return 0;
        }
        let flags = e.get(this, Ctl::iFlags);
        e.set(this, Ctl::iFlags, flags & 0xffbf_ffff);
    }
    let tolerance = x87_quiet_bits(e.global::<u32>(FLOAT_TWENTIETH));
    let mut still = e
        .call(POINTS_NEAR, &args![p + 0x10, POINT_DEFAULT, tolerance])
        .u8();
    let flags = e.get(this, Ctl::iFlags);
    let large = (flags >> 0x14) & 1 != 0;
    let projectile = (flags >> 0x13) & 1 != 0;
    if still == 0 {
        fn_00c6d150(e, this);
        e.set(this, Ctl::fRotMod, 0.0);
        e.set(this, Ctl::fRotModTime, 0.0);
    }
    let lane_tolerance = [f32::from_bits(e.global::<u32>(LANE_TOLERANCE)); 4];
    let modifier = load4(e, me + Ctl::VelocityMod.off);
    let zero_vector = load4(e, ZERO_VECTOR);
    still &= u8::from(near_lanes(e, modifier, zero_vector, lane_tolerance) & 7 == 7);
    store4(e, f + 0x70, lane_tolerance);
    let flags = e.get(this, Ctl::iFlags);
    e.set(
        this,
        Ctl::iFlags,
        if still != 0 {
            flags & 0xffff_fff7
        } else {
            flags | 8
        },
    );
    profile_record(e, PROFILE_MOVEMENT);
    bhk_character_controller_change_size(e, this, step);
    e.set(this, Ctl::eSupportMaterial, 0x24);
    e.set(this, Ctl::eSupportLayer, 0);
    let turn = e.mem.f32(p + 0xc);
    let time = e.mem.f32(p);
    let turned = bhk_character_controller_apply_rotation_modifier(e, this, turn, time);
    e.mem.set_f32(f + 0x20, turned);
    e.mem.set_f32(p + 0xc, turned);
    e.call(
        SET_AXIS_ANGLE,
        &args![f + 0x60, me + Ctl::UpVec.off, (-turned).to_bits()],
    );
    e.set(this, Ctl::StepDeltaTime, x87_quiet(e.mem.f32(p)));
    // `FLD1`, `FDIVRP` (`00c734e2`).
    let inverse = x87_div(1.0, x87_load32(e.mem.u32(p))) as f32;
    e.set(this, Ctl::StepInvDeltaTime, inverse);
    e.mem.set_u32(f + 0x48, p + 0x10);
    copy_words(e, f + 0x50, p + 0x10, 3);
    if e.call(STATE_FIVE_TEST, &args![me]).u8() == 0 && (e.get(this, Ctl::iFlags) >> 0xb) & 1 == 0 {
        e.mem.set_f32(f + 0x58, 0.0);
    }
    let inverse = x87_quiet(e.get(this, Ctl::StepInvDeltaTime));
    e.mem.set_f32(f + 0x20, inverse);
    // `FMULP ST2`, `FMUL dword` twice (`00c7353d`..`00c7354f`).
    let inverse = f64::from(inverse);
    let vx = x87_load32(e.mem.u32(f + 0x50));
    e.mem.set_f32(f + 0x50, x87_mul(vx, inverse) as f32);
    let vy = x87_m32(e.mem.u32(f + 0x54));
    e.mem.set_f32(f + 0x54, x87_mul(inverse, vy) as f32);
    let vz = x87_m32(e.mem.u32(f + 0x58));
    e.mem.set_f32(f + 0x58, x87_mul(inverse, vz) as f32);
    e.call(POINT_TO_VECTOR, &args![me + Ctl::Direction.off, f + 0x50]);
    if fn_00c6d120(e, this) && e.mem.f32(p + 4) != 0.0 {
        let zero = e.global::<u32>(FLOAT_ZERO);
        let minus_one = e.global::<u32>(FLOAT_MINUS_ONE);
        for (lane, bits) in [minus_one, zero, zero, zero].into_iter().enumerate() {
            e.mem.set_u32(f + 0x50 + 4 * lane as u32, bits);
        }
        let angle = x87_quiet_bits(e.mem.u32(p + 4));
        e.call(SET_AXIS_ANGLE, &args![f + 0x90, f + 0x50, angle]);
        let direction = load4(e, me + Ctl::Direction.off);
        let rotation = load4(e, f + 0x90);
        let turned = rotate_by(e, direction, rotation);
        store4(e, me + Ctl::Direction.off, turned);
    }
    let center = x87_quiet(e.get(this, Ctl::fCenter));
    e.mem.set_f32(me + Ctl::RotCenter.off + 8, center);
    let zero = e.global::<u32>(FLOAT_ZERO);
    let minus_one = e.global::<u32>(FLOAT_MINUS_ONE);
    for (lane, bits) in [zero, minus_one, zero, zero].into_iter().enumerate() {
        e.mem.set_u32(f + 0x90 + 4 * lane as u32, bits);
    }
    for lane in 0..4 {
        let up = e.mem.u32(me + Ctl::UpVec.off + 4 * lane);
        let sign = e.mem.u32(SIGN_MASK + 4 * lane);
        e.mem.set_u32(f + 0x80 + 4 * lane, up ^ sign);
    }
    let flags = e.get(this, Ctl::iFlags);
    if (flags >> 0x19) & 1 != 0 || (flags >> 0x1a) & 1 != 0 || flags & 1 != 0 {
        bhk_character_controller_apply_pitch_and_roll(e, this, Ptr::new(f + 0x60));
    }
    let forward = rotate_by(e, load4(e, f + 0x90), load4(e, f + 0x60));
    let current = load4(e, me + Ctl::ForwardVec.off);
    if still == 0 || near_lanes(e, forward, current, lane_tolerance) & 7 != 7 {
        let reset = x87_quiet(e.global::<f32>(PITCH_TIMER_RESET));
        e.set(this, Ctl::fCalculatePitchTimer, reset);
    }
    let proxy = e.get(this, Ctl::phkObject);
    e.mem.set_u32(f + 0x3c, proxy);
    store4(e, me + Ctl::ForwardVec.off, forward);
    if projectile {
        let velocity = e.call(GET_HK_LINEAR_VELOCITY, &args![me]).u32();
        copy_words(e, me + Ctl::OutVelocity.off, velocity, 4);
        copy_words(e, me + Ctl::m_surfaceVelocity.off, ZERO_VECTOR, 4);
        profile_record(e, PROFILE_UPDATE_STATE);
        let state_object = e.call(PROJECTILE_STATE, &args![me]).u32();
        e.vcall(state_object, 0x20, &args![me]);
        profile_record(e, PROFILE_END);
        e.call(PROJECTILE_VELOCITY, &args![me, me + Ctl::OutVelocity.off]);
        let delta = x87_quiet(e.get(this, Ctl::StepDeltaTime));
        bhk_character_controller_move_as_projectile(e, this, delta);
    } else {
        move_on_ground(e, this, p, f, still != 0, large, proxy);
    }
    let flags = e.get(this, Ctl::iFlags);
    if (flags >> 0x19) & 1 != 0 || (flags >> 0x1a) & 1 != 0 {
        fn_00c70060(e, this);
    }
    let result = if e.get(this, Ctl::iFlags) & 0xc0 != 0 {
        2
    } else {
        0
    };
    profile_record(e, PROFILE_END);
    let step = e.mem.f32(f + 0x38);
    let time = e.get(this, Ctl::fVelocityTime);
    if is_positive(time) {
        // `FSUB dword` (`00c73e62`).
        let left = x87_sub(x87_load32(time.to_bits()), x87_m32(step.to_bits())) as f32;
        e.set(this, Ctl::fVelocityTime, left);
        let over = left <= 0.0;
        if over {
            e.set(this, Ctl::fVelocityTime, 0.0);
            copy_zero(e, me + Ctl::VelocityMod.off, 4);
        }
    }
    let timer = e.get(this, Ctl::fCalculatePitchTimer);
    if is_positive(timer) {
        // `FSUB dword` (`00c73ea0`).
        let left = x87_sub(x87_load32(timer.to_bits()), x87_m32(step.to_bits())) as f32;
        e.set(this, Ctl::fCalculatePitchTimer, left);
    }
    let flags = e.get(this, Ctl::iFlags);
    e.set(this, Ctl::iFlags, flags & 0xffff_efff);
    result
}

/// `Move` in context state 4 (`00c731ac`).
fn move_in_state_four(e: &mut Engine, this: Ptr<Ctl>, p: u32, f: u32) {
    let me = this.addr();
    profile_record(e, PROFILE_UPDATE_STATE);
    bhk_character_controller_get_position_ov2(e, this, Ptr::new(f + 0x70));
    e.call(COPY_POINT, &args![f + 0x50, f + 0x70]);
    for o in [0, 4, 8] {
        // `FADD dword` (`00c731f9`, `00c7320e`, `00c73219`).
        let moved = x87_add(
            x87_load32(e.mem.u32(p + 0x10 + o)),
            x87_m32(e.mem.u32(f + 0x50 + o)),
        ) as f32;
        e.mem.set_f32(f + 0x50 + o, moved);
    }
    e.call(SET_POSITION, &args![me, f + 0x50]);
    update_character_state(e, this);
    let time = e.mem.f32(p);
    let source = if is_positive(time) {
        // `FLD1`, `FDIVRP` (`00c73255`), then `FMUL dword`, `FMUL ST1`, `FMUL dword`
        // (`00c73261`..`00c73271`).
        let inverse = x87_div(1.0, x87_load32(time.to_bits())) as f32;
        e.mem.set_f32(f + 0x44, inverse);
        let inverse = f64::from(inverse);
        let vx = x87_m32(e.mem.u32(p + 0x10));
        e.mem.set_f32(f + 0x50, x87_mul(inverse, vx) as f32);
        let vy = x87_load32(e.mem.u32(p + 0x14));
        e.mem.set_f32(f + 0x54, x87_mul(vy, inverse) as f32);
        let vz = x87_m32(e.mem.u32(p + 0x18));
        e.mem.set_f32(f + 0x58, x87_mul(inverse, vz) as f32);
        f + 0x50
    } else {
        POINT_DEFAULT
    };
    // `FMULP ST2`, `FMUL ST1`, `FMUL dword` (`00c73289`..`00c7329e`).
    let k = x87_load32(e.global::<u32>(INVERSE_SCALE));
    let x = x87_load32(e.mem.u32(source));
    let y = x87_load32(e.mem.u32(source + 4));
    let z = x87_m32(e.mem.u32(source + 8));
    store4(
        e,
        me + Ctl::OutVelocity.off,
        [
            x87_mul(x, k) as f32,
            x87_mul(y, k) as f32,
            x87_mul(k, z) as f32,
            0.0,
        ],
    );
    profile_record(e, PROFILE_END);
}

/// `Move` without flag bit 19 (`00c7383c`).
fn move_on_ground(
    e: &mut Engine,
    this: Ptr<Ctl>,
    p: u32,
    f: u32,
    still: bool,
    large: bool,
    proxy: u32,
) {
    let me = this.addr();
    e.set_global(BEST_SUPPORT_BODY, 0u32);
    let velocity = e.call(GET_HK_LINEAR_VELOCITY, &args![me]).u32();
    copy_words(e, me + Ctl::OutVelocity.off, velocity, 4);
    if still && e.call(GET_STATE, &args![me + 0x3e0]).u32() != 2 {
        e.mem.set_f32(me + Ctl::OutVelocity.off + 8, 0.0);
    }
    let tolerance = load4(e, f + 0x70);
    let check = e.mem.u8(p + 0x24) != 0
        || e.call(GET_STATE, &args![me + 0x3e0]).u32() != 0
        || e.call(POINT_COMPARE, &args![e.mem.u32(f + 0x48), POINT_DEFAULT])
            .u8()
            == 0
        || near_lanes(
            e,
            load4(e, me + Ctl::hkSupportNorm.off),
            load4(e, ZERO_VECTOR),
            tolerance,
        ) & 0xf
            == 0xf;
    if check {
        check_support(e, this, f);
    }
    profile_record(e, PROFILE_UPDATE_STATE);
    update_character_state(e, this);
    profile_record(e, PROFILE_END);
    let current = e.get(this, Ctl::phkObject);
    if current != 0 {
        e.call(
            HKP_SET_LINEAR_VELOCITY,
            &args![current, me + Ctl::OutVelocity.off],
        );
    }
    if manifold_unchanged(e, this, still, proxy, tolerance) {
        copy_words(e, me + Ctl::OutVelocity.off, ZERO_VECTOR, 4);
        let current = e.get(this, Ctl::phkObject);
        if current != 0 {
            e.call(
                HKP_SET_LINEAR_VELOCITY,
                &args![current, me + Ctl::OutVelocity.off],
            );
        }
    } else {
        e.mem
            .set_u32(proxy.wrapping_add(0xac), if large { 1 } else { 4 });
        bhk_character_proxy_integrate(e, this.cast(), me + Ctl::StepInfo.off);
        e.mem.set_u32(proxy.wrapping_add(0xac), 4);
    }
    let body: u32 = e.global(BEST_SUPPORT_BODY);
    if body != 0 && e.get(this, Ctl::iFlags) & 0x2000 == 0 {
        e.call(UN_MARK_WRITE, &args![me]);
        let hk_proxy = e.call(PROXY_OPERATOR, &args![me]).u32();
        let position = e.call(HKP_GET_POSITION, &args![hk_proxy]).u32();
        copy_words(e, f + 0x70, position, 4);
        let body: u32 = e.global(BEST_SUPPORT_BODY);
        // `FMUL dword` (`00c73ca6`).
        let lift = x87_mul(
            x87_load32(e.mem.u32(body + 0x1b8)),
            x87_m32(e.get(this, Ctl::StepDeltaTime).to_bits()),
        ) as f32;
        e.mem.set_f32(f + 0x1c, lift);
        let best: f32 = e.global(BEST_SUPPORT_HEIGHT);
        let center_z = e.mem.f32(me + Ctl::RotCenter.off + 8);
        // `FADD dword` twice (`00c73cbd`, `00c73cc3`): one chain, each sum rounded to the
        // main thread's precision.
        let z = x87_add(
            x87_add(x87_load32(center_z.to_bits()), x87_m32(best.to_bits())),
            x87_m32(lift.to_bits()),
        ) as f32;
        e.mem.set_f32(f + 0x78, z);
        e.call(HKP_SET_POSITION, &args![hk_proxy, f + 0x70]);
        let best: f32 = e.global(BEST_SUPPORT_HEIGHT);
        // `FADD dword` (`00c73cd6`).
        e.mem.set_f32(
            f + 0x78,
            x87_add(x87_load32(lift.to_bits()), x87_m32(best.to_bits())) as f32,
        );
        e.call(UN_MARK_WRITE, &args![me]);
    }
    e.set(this, Ctl::eWantState, 0xb);
    keep_manifold(e, this, proxy);
}

/// Whether a still controller's contact manifold (the Havok proxy's part `+0x74`) equals
/// `LastManifold` (positions and normals within the tolerance, words `+0x20` and `+0x48`
/// equal), in which case `Move` does not integrate. False unless still, the state is not
/// 2, in state 5 the vertical velocity is 0, and the manifold is not longer than
/// `LastManifold`.
fn manifold_unchanged(
    e: &mut Engine,
    this: Ptr<Ctl>,
    still: bool,
    proxy: u32,
    tolerance: [f32; 4],
) -> bool {
    let me = this.addr();
    if !still || e.call(GET_STATE, &args![me + 0x3e0]).u32() == 2 {
        return false;
    }
    if e.call(GET_STATE, &args![me + 0x3e0]).u32() == 5 {
        let vertical = e.mem.f32(me + Ctl::OutVelocity.off + 8);
        if 0.0 != vertical {
            return false;
        }
    }
    let part = e.call(PROXY_PART_74, &args![proxy]).u32();
    if e.mem.i32(part + 4) > e.get(this, Ctl::LastManifoldSize) as i32 {
        return false;
    }
    let mut index = 0;
    let mut offset = 0u32;
    loop {
        let part = e.call(PROXY_PART_74, &args![proxy]).u32();
        if index >= e.mem.i32(part + 4) {
            return true;
        }
        let part = e.call(PROXY_PART_74, &args![proxy]).u32();
        let current = e.mem.u32(part).wrapping_add(offset);
        let last = e.get(this, Ctl::LastManifold).wrapping_add(offset);
        let mut same = near_lanes(e, load4(e, last), load4(e, current), tolerance) & 0xf == 0xf;
        if same {
            let part = e.call(PROXY_PART_74, &args![proxy]).u32();
            let current = e.mem.u32(part).wrapping_add(offset);
            let last = e.get(this, Ctl::LastManifold).wrapping_add(offset);
            same = near_lanes(
                e,
                load4(e, last + 0x10),
                load4(e, current + 0x10),
                tolerance,
            ) & 0xf
                == 0xf;
            if same {
                let part = e.call(PROXY_PART_74, &args![proxy]).u32();
                let current = e.mem.u32(part).wrapping_add(offset);
                same = e.mem.u32(last + 0x20) == e.mem.u32(current + 0x20);
                if same {
                    let last = e.get(this, Ctl::LastManifold).wrapping_add(offset);
                    let part = e.call(PROXY_PART_74, &args![proxy]).u32();
                    let current = e.mem.u32(part).wrapping_add(offset);
                    same = e.mem.u32(last + 0x48) == e.mem.u32(current + 0x48);
                }
            }
        }
        index += 1;
        offset = offset.wrapping_add(0x70);
        if !same {
            return false;
        }
    }
}

/// Copies up to five points of the Havok proxy's manifold into `LastManifold`
/// (`hkpRootCdPoint::operator=` over the kept ones, a 0x70-byte copy appended for the
/// rest, room made by `00c90940`).
fn keep_manifold(e: &mut Engine, this: Ptr<Ctl>, proxy: u32) {
    let me = this.addr();
    let part = e.call(PROXY_PART_74, &args![proxy]).u32();
    let count = e.mem.i32(part + 4).min(5);
    let array = me + Ctl::LastManifold.off;
    let mut index = 0;
    while index < count {
        let offset = (index as u32).wrapping_mul(0x70);
        let part = e.call(PROXY_PART_74, &args![proxy]).u32();
        let source = e.mem.u32(part).wrapping_add(offset);
        if index < e.get(this, Ctl::LastManifoldSize) as i32 {
            let target = e.get(this, Ctl::LastManifold).wrapping_add(offset);
            e.call(ROOT_CD_POINT_ASSIGN, &args![target, source]);
        } else {
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            if e.mem.u32(array + 4) == e.mem.u32(array + 8) & 0x3fff_ffff {
                let allocator = e.mem.u32(router + 0x10);
                e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 0x70u32]);
            }
            let size = e.mem.u32(array + 4);
            let target = size.wrapping_mul(0x70).wrapping_add(e.mem.u32(array));
            if target != 0 {
                copy_words(e, target, source, 0x1c);
            }
            e.mem.set_u32(array + 4, size.wrapping_add(1));
        }
        index += 1;
    }
}

/// The support check of `Move` (`00c738ca`): flags 0x200/0x400 cleared and 0x10000 set
/// around `InitFrontBack` and `CheckSupport` (direction `-UpVec`, the surface info at
/// `+0x480`). Supported means state 2 and not all surfaces stopped, or flag bit 9; with
/// flag 0x2000 (a jump) support is refused, and the flag is cleared unless the upward
/// motion goes on: with no support, no rising velocity, or (supported, bit 9, ground not
/// steeper than `fMaxSlope`) a direction along the support normal of at most 0.05. Flag
/// 0x100 records the result.
fn check_support(e: &mut Engine, this: Ptr<Ctl>, f: u32) {
    let me = this.addr();
    let flags = e.get(this, Ctl::iFlags) & 0xffff_f9ff;
    e.set(this, Ctl::iFlags, flags | 0x1_0000);
    bhk_character_controller_init_front_back(e, this);
    bhk_character_proxy_check_support(e, this.cast(), f + 0x80, me + Ctl::m_supportedState.off);
    let flags = e.get(this, Ctl::iFlags);
    e.set(this, Ctl::iFlags, flags & 0xfffe_ffff);
    let mut supported =
        e.get(this, Ctl::m_supportedState) == 2 && e.get(this, Ctl::bAllSurfacesStopped) == 0;
    let flags = e.get(this, Ctl::iFlags);
    if flags & 0x2000 == 0 {
        supported |= (flags >> 9) & 1 != 0;
    } else {
        let clear = |e: &mut Engine| {
            let flags = e.get(this, Ctl::iFlags);
            e.set(this, Ctl::iFlags, flags & 0xffff_dfff);
        };
        let vertical = e.mem.f32(me + Ctl::OutVelocity.off + 8);
        // Two exe branches reach the same clear: unsupported without bit 9,
        // or a vertical speed that is not above 0 (false for NaN).
        if (!supported && (flags >> 9) & 1 == 0) || vertical <= 0.0 {
            clear(e);
        } else {
            let velocity = load4(e, me + Ctl::OutVelocity.off);
            let length = sum_of_squares3(velocity).sqrt();
            if is_positive(length) {
                // `FLD1`, `FDIVRP` (`00c73999`).
                let inverse = x87_div(1.0, x87_load32(length.to_bits())) as f32;
                let normal_z = e.mem.f32(me + Ctl::hkSupportNorm.off + 8);
                let max_slope = e.get(this, Ctl::fMaxSlope);
                if supported && (flags >> 9) & 1 != 0 && max_slope <= normal_z {
                    let normal = load4(e, me + Ctl::hkSupportNorm.off);
                    let products: [f32; 4] =
                        std::array::from_fn(|lane| inverse * velocity[lane] * normal[lane]);
                    let along = products[2] + (products[1] + products[0]);
                    let limit: f64 = e.global(ROT_MOD_DECAY);
                    if along as f64 <= limit {
                        clear(e);
                    }
                }
            }
        }
        supported = false;
    }
    let flags = e.get(this, Ctl::iFlags);
    e.set(
        this,
        Ctl::iFlags,
        if supported {
            flags | 0x100
        } else {
            flags & 0xffff_feff
        },
    );
}

// Translated from 00c73ed0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c73ed0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkCharacterController::Init` (Xbox PDB) with the cinfo `info`, the allocation-scope
/// word at 0x10 meanwhile: `fStepHeight` = `info + 0xAC` times `info + 0xA8`; the shapes
/// (`00c72410`); without a phantom, a `bhkCachingShapePhantom` (0x18 bytes) from a stack
/// cinfo (`0056e0b0`; `info + 0x84` and the current shape's Havok object) becomes
/// `spShapePhantom` and the cinfo's array is freed. With a world at `info + 0x88`, the
/// phantom joins it (slot 0x9C) with its debug colour, the display geometry is built
/// (slot 0xD0 with 0) when the world's byte `+0x16` is set, and the collision box joins
/// it. Then `eWantState`, `fAcrobatics`, `fGravity` and `iPriority` come from the cinfo,
/// `info + 0x48` gets the phantom's Havok object, `bhkCharacterProxy::Init(info)` runs,
/// the Havok proxy's byte `+0xB0` is set, flag bits 25 and 26 follow `info + 0x97` and
/// `+0x98`, and the listener is added to the Havok proxy (between two `UnMarkWrite`
/// calls).
pub fn bhk_character_controller_init(e: &mut Engine, this: Ptr<Ctl>, info: u32) {
    let me = this.addr();
    let (word, saved) = enter_scope(e);
    let step = (e.mem.f32(info + 0xac) as f64 * e.mem.f32(info + 0xa8) as f64) as f32;
    e.set(this, Ctl::fStepHeight, step);
    fn_00c72410(e, this, info);
    if e.get(this, Ctl::spShapePhantom) == 0 {
        e.with_stack(0x70, |e, cinfo| {
            let cinfo = cinfo.addr();
            e.call(PHANTOM_CINFO_CONSTRUCT, &args![cinfo]);
            let filter = e.mem.u32(info + 0x84);
            e.mem.set_u32(cinfo, filter);
            let shape = bhk_character_controller_get_shape(e, this);
            let hk = hk_of(e, shape);
            e.mem.set_u32(cinfo + 4, hk);
            let block = e.call(HEAP_ALLOCATE, &args![0x18u32]).u32();
            let phantom = if block != 0 {
                bhk_caching_shape_phantom_construct(e, Ptr::new(block), cinfo).addr()
            } else {
                0
            };
            assign_ni_pointer(e, me + Ctl::spShapePhantom.off, phantom);
            free_array_buffer(e, cinfo + 0xc, 8);
        });
    }
    let world = e.mem.u32(info + 0x88);
    if world != 0 {
        let phantom = e.get(this, Ctl::spShapePhantom);
        if phantom != 0 {
            e.vcall(phantom, 0x9c, &args![world]);
            let world = e.mem.u32(info + 0x88);
            let phantom = e.get(this, Ctl::spShapePhantom);
            set_phantom_debug_color(e, world, phantom);
            let world = e.mem.u32(info + 0x88);
            if e.mem.u8(world + 0x16) != 0 {
                e.vcall(me, 0xd0, &args![0u32]);
            }
        }
        let collision_box = e.get(this, Ctl::spCollisionBox);
        if collision_box != 0 {
            let world = e.mem.u32(info + 0x88);
            e.vcall(collision_box, 0x9c, &args![world]);
        }
    }
    let wanted = e.mem.u32(info + 0x9c);
    e.set(this, Ctl::eWantState, wanted);
    e.set(this, Ctl::fAcrobatics, x87_quiet(e.mem.f32(info + 0x8c)));
    e.set(this, Ctl::fGravity, x87_quiet(e.mem.f32(info + 0x90)));
    let priority = e.mem.u32(info + 0xb4);
    e.set(this, Ctl::iPriority, priority);
    let hk = phantom_object(e, this);
    e.mem.set_u32(info + 0x48, hk);
    e.call(CHARACTER_PROXY_INIT, &args![this, info]);
    let proxy = e.get(this, Ctl::phkObject);
    e.mem.set_u8(proxy.wrapping_add(0xb0), 1);
    let flags = e.get(this, Ctl::iFlags);
    let flags = if e.mem.u8(info + 0x97) != 0 {
        flags | 0x0200_0000
    } else {
        flags & 0xfdff_ffff
    };
    e.set(this, Ctl::iFlags, flags);
    let flags = if e.mem.u8(info + 0x98) != 0 {
        flags | 0x0400_0000
    } else {
        flags & 0xfbff_ffff
    };
    e.set(this, Ctl::iFlags, flags);
    let proxy = e.get(this, Ctl::phkObject);
    if proxy != 0 {
        e.call(UN_MARK_WRITE, &args![this]);
        e.call(ADD_PROXY_LISTENER, &args![proxy, me + LISTENER]);
        e.call(UN_MARK_WRITE, &args![this]);
    }
    e.mem.set_u32(word, saved);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c6_cba0, bhk_character_controller_update_state(Ptr<Ctl>)),
        entry!(0x00c6_cbc0, fn_00c6cbc0(Ptr<Ctl>, f32, f32)),
        entry!(
            0x00c6_cc10,
            bhk_character_controller_apply_rotation_modifier(Ptr<Ctl>, f32, f32) -> f32
        ),
        entry!(0x00c6_cca0, fn_00c6cca0(Ptr)),
        entry!(0x00c6_ccc0, fn_00c6ccc0(Ptr, Ptr, Ptr)),
        entry!(0x00c6_cce0, fn_00c6cce0(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00c6_cd00, fn_00c6cd00(Ptr, Ptr)),
        entry!(0x00c6_cd30, fn_00c6cd30(Ptr, Ptr) -> Ptr),
        entry!(0x00c6_cd60, fn_00c6cd60(Ptr, Ptr) -> Ptr),
        entry!(0x00c6_cd90, bhk_shape_collection_get_rtti(Ptr) -> u32),
        entry!(0x00c6_cda0, bhk_shape_collection_scalar_deleting_destructor(Ptr, u8) -> Ptr),
        entry!(0x00c6_cdc0, fn_00c6cdc0(u32)),
        entry!(0x00c6_cde0, fn_00c6cde0(Ptr)),
        entry!(0x00c6_ce80, fn_00c6ce80(Ptr) -> Ptr),
        entry!(0x00c6_cee0, bhk_character_proxy_q_cinfo_size(Ptr) -> u32),
        entry!(
            0x00c6_cef0,
            bhk_character_proxy_check_support(Ptr, u32, u32)
        ),
        entry!(0x00c6_cf30, bhk_character_proxy_integrate(Ptr, u32)),
        entry!(0x00c6_d010, bhk_character_proxy_scalar_deleting_destructor(Ptr, u8) -> Ptr),
        entry!(0x00c6_d040, bhk_caching_shape_phantom_construct(Ptr, u32) -> Ptr),
        entry!(0x00c6_d0e0, bhk_caching_shape_phantom_get_rtti(Ptr) -> u32),
        entry!(0x00c6_d0f0, fn_00c6d0f0(Ptr) -> u32),
        entry!(0x00c6_d100, bhk_caching_shape_phantom_scalar_deleting_destructor(Ptr, u8) -> Ptr),
        entry!(0x00c6_d120, fn_00c6d120(Ptr<Ctl>) -> bool),
        entry!(0x00c6_d150, fn_00c6d150(Ptr<Ctl>)),
        entry!(0x00c6_d170, hkp_closest_cd_point_collector_reset(Ptr)),
        entry!(0x00c6_d190, fn_00c6d190(Ptr) -> Ptr),
        entry!(0x00c6_d1e0, fn_00c6d1e0(Ptr, u32, u32) -> Ptr),
        entry!(0x00c6_d250, bhk_collision_box_get_rtti(Ptr) -> u32),
        entry!(0x00c6_d260, bhk_collision_box_scalar_deleting_destructor(Ptr, u8) -> Ptr),
        entry!(0x00c6_d290, fn_00c6d290(Ptr, u32) -> Ptr),
        entry!(0x00c6_d310, bhk_list_shape_get_rtti(Ptr) -> u32),
        entry!(0x00c6_d320, bhk_list_shape_scalar_deleting_destructor(Ptr, u8) -> Ptr),
        entry!(0x00c6_d340, fn_00c6d340()),
        entry!(0x00c6_d3c0, fn_00c6d3c0()),
        entry!(
            0x00c6_d3f0,
            bhk_character_controller_set_shape(Ptr<Ctl>, u32, u32) -> bool
        ),
        entry!(0x00c6_d4a0, fn_00c6d4a0(Ptr<Ctl>)),
        entry!(0x00c6_d4b0, fn_00c6d4b0(Ptr<Ctl>, f32)),
        entry!(
            0x00c6_d4d0,
            bhk_character_controller_set_velocity_modifier(Ptr<Ctl>, Ptr, f32)
        ),
        entry!(
            0x00c6_d5c0,
            bhk_character_controller_apply_velocity_mod(Ptr<Ctl>)
        ),
        entry!(0x00c6_d620, fn_00c6d620(Ptr, u32, u32)),
        entry!(0x00c6_d670, fn_00c6d670(Ptr, Ptr) -> Ptr),
        entry!(0x00c6_d6b0, fn_00c6d6b0(Ptr<Ctl>, Ptr, u32)),
        entry!(0x00c6_d770, fn_00c6d770(Ptr<Ctl>, u32)),
        entry!(
            0x00c6_d7a0,
            bhk_character_controller_set_ni_object(Ptr<Ctl>, u32)
        ),
        entry!(0x00c6_d7e0, fn_00c6d7e0(u32, u8, u8, i32) -> u8),
        entry!(0x00c6_d840, hkp_world_object_get_property(Ptr, Ptr, u32, u32) -> Ptr),
        entry!(0x00c6_d8a0, fn_00c6d8a0(Ptr, u32) -> bool),
        entry!(0x00c6_d900, fn_00c6d900(u32) -> u32),
        entry!(
            0x00c6_d960,
            bhk_character_controller_get_shape(Ptr<Ctl>) -> u32
        ),
        entry!(0x00c6_d9c0, hkp_closest_cd_point_collector_scalar_deleting_destructor(Ptr, u8) -> Ptr),
        entry!(0x00c6_d9f0, fn_00c6d9f0(Ptr) -> Ptr),
        entry!(0x00c6_da50, fn_00c6da50(Ptr, u8) -> Ptr),
        entry!(0x00c6_de10, fn_00c6de10(Ptr)),
        entry!(0x00c6_dec0, bhk_character_controller_find_bs_reference(u32, u32) -> u32),
        entry!(0x00c6_def0, fn_00c6def0(u32) -> bool),
        entry!(0x00c6_df60, fn_00c6df60(Ptr, Ptr) -> u32),
        entry!(0x00c6_e030, fn_00c6e030(Ptr)),
        entry!(0x00c6_e050, fn_00c6e050(Ptr<Ctl>, u32, i32)),
        entry!(
            0x00c6_e0b0,
            bhk_character_controller_find_display_geometry(Ptr<Ctl>) -> u32
        ),
        entry!(
            0x00c6_e180,
            bhk_character_controller_set_radius(Ptr<Ctl>, f32)
        ),
        entry!(
            0x00c6_e280,
            bhk_character_controller_get_radius(Ptr<Ctl>) -> f32
        ),
        entry!(0x00c6_e2e0, fn_00c6e2e0(Ptr<Ctl>)),
        entry!(
            0x00c6_e300,
            bhk_character_controller_get_position_ov2(Ptr<Ctl>, Ptr)
        ),
        entry!(
            0x00c6_e390,
            bhk_character_controller_set_position_ov2(Ptr<Ctl>, Ptr)
        ),
        entry!(
            0x00c6_e420,
            bhk_character_controller_apply_pitch_and_roll(Ptr<Ctl>, Ptr)
        ),
        entry!(0x00c6_e680, fn_00c6e680(Ptr<Ctl>, u32, Ptr)),
        entry!(
            0x00c6_e820,
            bhk_character_controller_init_front_back(Ptr<Ctl>)
        ),
        entry!(0x00c6_e980, fn_00c6e980(Ptr<Ctl>, u32)),
        entry!(0x00c6_ead0, fn_00c6ead0(u32) -> u32),
        entry!(
            0x00c6_eb00,
            bhk_character_controller_is_step(Ptr<Ctl>, Ptr) -> bool
        ),
        entry!(
            0x00c6_f280,
            bhk_character_controller_object_interaction_callback(Ptr, u32, Ptr, Ptr)
        ),
        entry!(
            0x00c6_f500,
            bhk_character_controller_character_interaction_callback(Ptr, u32, Ptr, Ptr)
        ),
        entry!(
            0x00c6_fa20,
            bhk_character_controller_check_inside_of_object(Ptr<Ctl>) -> bool
        ),
        entry!(0x00c6_fd20, fn_00c6fd20(Ptr<Ctl>) -> u32),
        entry!(0x00c6_fd40, fn_00c6fd40(Ptr<Ctl>) -> u32),
        entry!(0x00c6_fd60, fn_00c6fd60(Ptr<Ctl>) -> u32),
        entry!(
            0x00c6_fd80,
            bhk_character_controller_build_display_geometry(Ptr<Ctl>, u32)
        ),
        entry!(
            0x00c6_ffc0,
            bhk_character_controller_kill_display_geometry(Ptr<Ctl>)
        ),
        entry!(0x00c7_0060, fn_00c70060(Ptr<Ctl>)),
        entry!(
            0x00c7_0550,
            bhk_character_controller_get_fall_distance(Ptr<Ctl>) -> f32
        ),
        entry!(0x00c7_05c0, fn_00c705c0(Ptr<Ctl>)),
        entry!(0x00c7_0810, fn_00c70810(Ptr, u8) -> Ptr),
        entry!(0x00c7_0820, fn_00c70820(Ptr, u8) -> Ptr),
        entry!(
            0x00c7_0830,
            bhk_character_controller_set_shape_type(Ptr<Ctl>, i32)
        ),
        entry!(
            0x00c7_08a0,
            bhk_character_controller_set_size(Ptr<Ctl>, f32)
        ),
        entry!(
            0x00c7_0940,
            bhk_character_controller_change_size(Ptr<Ctl>, f32)
        ),
        entry!(
            0x00c7_0b60,
            bhk_character_controller_move_immobile(Ptr<Ctl>, Ptr, u8)
        ),
        entry!(0x00c7_0bd0, fn_00c70bd0(Ptr<Ctl>)),
        entry!(0x00c7_0c60, fn_00c70c60(Ptr<Ctl>, u8) -> Ptr<Ctl>),
        entry!(
            0x00c7_0c90,
            bhk_character_controller_set_world(Ptr<Ctl>, u32)
        ),
        entry!(0x00c7_0de0, fn_00c70de0(Ptr, Ptr, f32, f32) -> u32),
        entry!(0x00c7_1120, fn_00c71120(Ptr)),
        entry!(0x00c7_11d0, fn_00c711d0(Ptr, u32, u32, u32)),
        entry!(0x00c7_2210, fn_00c72210(Ptr)),
        entry!(
            0x00c7_2250,
            bhk_character_controller_move_as_projectile(Ptr<Ctl>, f32)
        ),
        entry!(0x00c7_2410, fn_00c72410(Ptr<Ctl>, u32)),
        entry!(
            0x00c7_3170,
            bhk_character_controller_move(Ptr<Ctl>, Ptr) -> u32
        ),
        entry!(0x00c7_3ed0, bhk_character_controller_init(Ptr<Ctl>, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Scratch words the doubles write their observations to (inside the mapped exe range).
    const SCRATCH: u32 = 0x012f_0000;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e.call_log = Some(vec![]);
        e
    }

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn ret_st0(st0: f64) -> Ret {
        Ret {
            st0,
            ..Ret::default()
        }
    }

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn stub(e: &mut Engine, addresses: &[u32]) {
        for a in addresses {
            e.register(*a, nothing);
        }
    }

    fn returns(e: &mut Engine, address: u32, value: u32) {
        e.register_double(address, move |_, _| ret(value));
    }

    /// The argument words of every logged call to `address`.
    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    /// The logged call addresses, in order, among `of`.
    fn order(e: &Engine, of: &[u32]) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|c| c.0)
            .filter(|a| of.contains(a))
            .collect()
    }

    /// A heap object whose fresh vtable has the given `(byte offset, target)` slots.
    fn with_vtable(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x200);
        for (offset, target) in slots {
            e.mem.set_u32(vtable + offset, *target);
        }
        let object = e.mem.alloc(0x700);
        e.mem.set_u32(object, vtable);
        object
    }

    fn controller(e: &mut Engine) -> Ptr<Ctl> {
        e.new_object::<Ctl>()
    }

    fn vec_at(e: &mut Engine, values: [f32; 4]) -> u32 {
        let a = e.mem.alloc(0x10);
        store4(e, a, values);
        a
    }

    /// A Havok allocator reached through `MEMORY_ROUTER`: slot 4 allocates (returns
    /// `block`), slots 8 and 0x10 free (logged at `0x0900_0f08` and `0x0900_0f10`).
    fn install_allocator(e: &mut Engine, block: u32) {
        e.register_double(0x0900_0f04, move |_, _| ret(block));
        e.register(0x0900_0f08, nothing);
        e.register(0x0900_0f10, nothing);
        let allocator = with_vtable(
            e,
            &[(4, 0x0900_0f04), (8, 0x0900_0f08), (0x10, 0x0900_0f10)],
        );
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        returns(e, MEMORY_ROUTER, router);
    }

    #[test]
    fn rot_mod_is_replaced_only_by_a_larger_magnitude() {
        let mut e = engine();
        let c = controller(&mut e);
        e.set(c, Ctl::fRotMod, 0.5);
        fn_00c6cbc0(&mut e, c, -0.75, f32::from_bits(0x7f80_0001));
        assert_eq!(e.get(c, Ctl::fRotMod), -0.75);
        assert_eq!(e.get(c, Ctl::fRotModTime).to_bits(), 0x7fc0_0001);
        fn_00c6cbc0(&mut e, c, 0.25, 9.0);
        fn_00c6cbc0(&mut e, c, f32::NAN, 9.0);
        assert_eq!(e.get(c, Ctl::fRotMod), -0.75);
        assert_eq!(e.get(c, Ctl::fRotModTime).to_bits(), 0x7fc0_0001);
    }

    #[test]
    fn rotation_modifier_uses_the_shorter_time_and_decays() {
        let mut e = engine();
        e.set_global(ROT_MOD_DECAY, 0.05f32 as f64);
        let c = controller(&mut e);
        e.set(c, Ctl::fRotMod, 2.0);
        e.set(c, Ctl::fRotModTime, 0.5);
        let r = bhk_character_controller_apply_rotation_modifier(&mut e, c, 1.0, 0.25);
        assert_eq!(r, 1.5);
        assert_eq!(e.get(c, Ctl::fRotModTime), 0.25);
        assert_eq!(e.get(c, Ctl::fRotMod), (2.0f64 - 0.05f32 as f64) as f32);
        e.set(c, Ctl::fRotMod, 2.0);
        e.set(c, Ctl::fRotModTime, 0.125);
        let r = bhk_character_controller_apply_rotation_modifier(&mut e, c, 1.0, 0.25);
        assert_eq!(r, 1.25);
        assert_eq!(e.get(c, Ctl::fRotModTime), -0.125);
        for remaining in [0.0, f32::NAN] {
            e.set(c, Ctl::fRotMod, 2.0);
            e.set(c, Ctl::fRotModTime, remaining);
            let r = bhk_character_controller_apply_rotation_modifier(&mut e, c, 3.0, 0.25);
            assert_eq!(r, 3.0);
            assert_eq!(e.get(c, Ctl::fRotMod), 0.0);
        }
    }

    #[test]
    fn ref_pointer_release_clears_the_slot() {
        let mut e = engine();
        stub(&mut e, &[REMOVE_REFERENCE]);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, 0x4444);
        fn_00c6cca0(&mut e, Ptr::new(slot));
        fn_00c6cca0(&mut e, Ptr::new(slot));
        assert_eq!(calls(&e, REMOVE_REFERENCE), vec![vec![0x4444]]);
        assert_eq!(e.mem.u32(slot), 0);
    }

    #[test]
    fn first_lane_difference_keeps_the_other_lanes() {
        let mut e = engine();
        let a = vec_at(&mut e, [5.0, 6.0, 7.0, 8.0]);
        let b = vec_at(&mut e, [2.0, 100.0, 100.0, 100.0]);
        let out = e.mem.alloc(0x10);
        fn_00c6ccc0(&mut e, Ptr::new(out), Ptr::new(a), Ptr::new(b));
        assert_eq!(load4(&e, out), [3.0, 6.0, 7.0, 8.0]);
    }

    #[test]
    fn first_lane_sum_keeps_the_other_lanes() {
        let mut e = engine();
        let a = vec_at(&mut e, [5.0, 6.0, 7.0, 8.0]);
        let b = vec_at(&mut e, [2.0, 100.0, 100.0, 100.0]);
        let out = e.mem.alloc(0x10);
        fn_00c6cce0(&mut e, Ptr::new(out), Ptr::new(a), Ptr::new(b));
        assert_eq!(load4(&e, out), [7.0, 6.0, 7.0, 8.0]);
    }

    #[test]
    fn first_lane_sum_answers_its_out_vector() {
        let mut e = engine();
        let a = vec_at(&mut e, [5.0, 6.0, 7.0, 8.0]);
        let b = vec_at(&mut e, [2.0, 100.0, 100.0, 100.0]);
        let out = e.mem.alloc(0x10);
        // `MOV EAX,[EBP+8]` before the RET: the callers read the vector through EAX (00d8cd9f).
        assert_eq!(e.call(0x00c6_cce0, &args![out, a, b]).u32(), out);
    }

    fn filled(e: &mut Engine, words: u32) -> u32 {
        let a = e.mem.alloc(4 * words + 8);
        for i in 0..words {
            e.mem.set_u32(a + 4 * i, 0x7f80_0001 + i);
        }
        a
    }

    #[test]
    fn copy_of_three_vectors() {
        let mut e = engine();
        let src = filled(&mut e, 13);
        let dst = e.mem.alloc(0x40);
        fn_00c6cd00(&mut e, Ptr::new(dst), Ptr::new(src));
        assert_eq!(e.mem.bytes(dst, 0x30), e.mem.bytes(src, 0x30));
        assert_eq!(e.mem.u32(dst + 0x30), 0);
    }

    #[test]
    fn copy_of_three_vectors_returns_this() {
        let mut e = engine();
        let src = filled(&mut e, 13);
        let dst = e.mem.alloc(0x40);
        assert_eq!(
            fn_00c6cd30(&mut e, Ptr::new(dst), Ptr::new(src)).addr(),
            dst
        );
        assert_eq!(e.mem.bytes(dst, 0x30), e.mem.bytes(src, 0x30));
        assert_eq!(e.mem.u32(dst + 0x30), 0);
    }

    #[test]
    fn copy_of_four_vectors_returns_this() {
        let mut e = engine();
        let src = filled(&mut e, 17);
        let dst = e.mem.alloc(0x48);
        assert_eq!(
            fn_00c6cd60(&mut e, Ptr::new(dst), Ptr::new(src)).addr(),
            dst
        );
        assert_eq!(e.mem.bytes(dst, 0x40), e.mem.bytes(src, 0x40));
        assert_eq!(e.mem.u32(dst + 0x40), 0);
    }

    #[test]
    fn shape_collection_rtti() {
        let mut e = engine();
        assert_eq!(
            bhk_shape_collection_get_rtti(&mut e, Ptr::NULL),
            0x0126_86d4
        );
    }

    #[test]
    fn shape_collection_deleting_destructor_frees_0x14_bytes() {
        let mut e = engine();
        stub(&mut e, &[SHAPE_COLLECTION_DESTRUCT, FREE_SIZED]);
        let this = Ptr::new(0x5000);
        let r = bhk_shape_collection_scalar_deleting_destructor(&mut e, this, 0);
        assert_eq!(r.addr(), 0x5000);
        assert!(calls(&e, FREE_SIZED).is_empty());
        bhk_shape_collection_scalar_deleting_destructor(&mut e, this, 1);
        assert_eq!(calls(&e, SHAPE_COLLECTION_DESTRUCT).len(), 2);
        assert_eq!(calls(&e, FREE_SIZED), vec![vec![0x5000, 0x14]]);
    }

    #[test]
    fn cinfo_block_is_freed_through_havok() {
        let mut e = engine();
        install_allocator(&mut e, 0);
        fn_00c6cdc0(&mut e, 0);
        assert!(calls(&e, 0x0900_0f08).is_empty());
        fn_00c6cdc0(&mut e, 0x7000);
        assert_eq!(calls(&e, 0x0900_0f08)[0][1..], [0x7000, 0x80]);
    }

    #[test]
    fn proxy_cinfo_size() {
        let mut e = engine();
        assert_eq!(bhk_character_proxy_q_cinfo_size(&mut e, Ptr::NULL), 0x80);
    }

    #[test]
    fn integrate_runs_the_collectors_between_unmark_writes() {
        let mut e = engine();
        stub(
            &mut e,
            &[
                UN_MARK_WRITE,
                POINT_COLLECTOR_CONSTRUCT,
                INTEGRATE_WITH_COLLECTORS,
                POINT_COLLECTOR_HANDLE_COLLISIONS,
                POINT_COLLECTOR_DESTRUCT,
            ],
        );
        returns(&mut e, 0x0900_0094, 0x6600);
        let this = with_vtable(&mut e, &[(0x94, 0x0900_0094)]);
        e.mem.set_u32(this + 8, 0x7700);
        bhk_character_proxy_integrate(&mut e, Ptr::new(this), 0x8800);
        let all = [
            UN_MARK_WRITE,
            POINT_COLLECTOR_CONSTRUCT,
            0x0900_0094,
            INTEGRATE_WITH_COLLECTORS,
            POINT_COLLECTOR_HANDLE_COLLISIONS,
            POINT_COLLECTOR_DESTRUCT,
        ];
        let mut expected = all.to_vec();
        expected.push(UN_MARK_WRITE);
        assert_eq!(order(&e, &all), expected);
        let local = calls(&e, POINT_COLLECTOR_CONSTRUCT)[0][0];
        assert_eq!(calls(&e, POINT_COLLECTOR_CONSTRUCT)[0][1], this + 0x10);
        assert_eq!(
            calls(&e, INTEGRATE_WITH_COLLECTORS)[0],
            vec![0x7700, 0x8800, 0x6610, local, this + 0x10]
        );
        assert_eq!(
            calls(&e, POINT_COLLECTOR_HANDLE_COLLISIONS)[0],
            vec![this + 0x10]
        );
        assert_eq!(calls(&e, POINT_COLLECTOR_DESTRUCT)[0], vec![local]);

        e.call_log = Some(vec![]);
        e.mem.set_u32(this + 8, 0);
        bhk_character_proxy_integrate(&mut e, Ptr::new(this), 0x8800);
        assert_eq!(order(&e, &all), vec![UN_MARK_WRITE, UN_MARK_WRITE]);
    }

    #[test]
    fn proxy_deleting_destructor_frees_0x3e0_bytes() {
        let mut e = engine();
        stub(&mut e, &[CHARACTER_PROXY_DESTRUCT, FREE_SIZED]);
        bhk_character_proxy_scalar_deleting_destructor(&mut e, Ptr::new(0x5000), 2);
        assert!(calls(&e, FREE_SIZED).is_empty());
        bhk_character_proxy_scalar_deleting_destructor(&mut e, Ptr::new(0x5000), 3);
        assert_eq!(calls(&e, FREE_SIZED), vec![vec![0x5000, 0x3e0]]);
        assert_eq!(calls(&e, CHARACTER_PROXY_DESTRUCT).len(), 2);
    }

    fn check_vtable_is(expected: u32) -> impl FnMut(&mut Engine, &[u32]) -> Ret {
        move |e, a| {
            assert_eq!(e.mem.u32(a[0]), expected);
            Ret::default()
        }
    }

    #[test]
    fn caching_shape_phantom_constructor() {
        let mut e = engine();
        stub(&mut e, &[REF_OBJECT_CONSTRUCT]);
        e.register_double(
            CACHING_PHANTOM_INIT,
            check_vtable_is(VTABLE_CACHING_SHAPE_PHANTOM),
        );
        e.set_global(COUNT_WORLD_OBJECT, 4u32);
        let this = e.mem.alloc(0x18);
        for o in [0xc, 0x10, 0x14] {
            e.mem.set_u32(this + o, 0xffff_ffff);
        }
        let r = bhk_caching_shape_phantom_construct(&mut e, Ptr::new(this), 0x99);
        assert_eq!(r.addr(), this);
        assert_eq!(e.mem.u32(this), VTABLE_CACHING_SHAPE_PHANTOM);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.mem.u8(this + 0x14), 0);
        assert_eq!(e.mem.u8(this + 0x15), 0xff);
        assert_eq!(calls(&e, CACHING_PHANTOM_INIT), vec![vec![this, 0x99]]);
        assert_eq!(e.global::<u32>(COUNT_WORLD_OBJECT), 5);
        for counter in [
            COUNT_PHANTOM,
            COUNT_SHAPE_PHANTOM,
            COUNT_CACHING_SHAPE_PHANTOM,
        ] {
            assert_eq!(e.global::<u32>(counter), 1);
        }
    }

    #[test]
    fn caching_shape_phantom_rtti() {
        let mut e = engine();
        assert_eq!(
            bhk_caching_shape_phantom_get_rtti(&mut e, Ptr::NULL),
            0x0126_85a4
        );
    }

    #[test]
    fn size_query_is_0x60() {
        let mut e = engine();
        assert_eq!(fn_00c6d0f0(&mut e, Ptr::NULL), 0x60);
    }

    #[test]
    fn caching_shape_phantom_deleting_destructor_frees_0x18_bytes() {
        let mut e = engine();
        stub(&mut e, &[CACHING_PHANTOM_DESTRUCT, FREE_SIZED]);
        bhk_caching_shape_phantom_scalar_deleting_destructor(&mut e, Ptr::new(0x5000), 1);
        assert_eq!(calls(&e, CACHING_PHANTOM_DESTRUCT), vec![vec![0x5000]]);
        assert_eq!(calls(&e, FREE_SIZED), vec![vec![0x5000, 0x18]]);
    }

    #[test]
    fn states_four_to_six_count() {
        let mut e = engine();
        let c = controller(&mut e);
        for (state, expected) in [(3, false), (4, true), (5, true), (6, true), (7, false)] {
            returns(&mut e, GET_STATE, state);
            assert_eq!(fn_00c6d120(&mut e, c), expected);
        }
        assert_eq!(calls(&e, GET_STATE)[0], vec![c.addr() + 0x3e0]);
    }

    #[test]
    fn push_delta_is_cleared_with_its_count() {
        let mut e = engine();
        let c = controller(&mut e);
        store4(&mut e, c.addr() + 0x580, [1.0, 2.0, 3.0, 4.0]);
        fn_00c6d150(&mut e, c);
        assert_eq!(load4(&e, c.addr() + 0x580), [1.0, 2.0, 3.0, 4.0]);
        e.set(c, Ctl::iPushCount, 3);
        fn_00c6d150(&mut e, c);
        assert_eq!(load4(&e, c.addr() + 0x580), [0.0; 4]);
        assert_eq!(e.get(c, Ctl::iPushCount), 0);
    }

    #[test]
    fn collector_reset_sets_the_far_distances() {
        let mut e = engine();
        e.set_global(COLLECTOR_FAR, 0x7f7f_ffeeu32);
        let this = e.mem.alloc(0x34);
        e.mem.set_u32(this + 0x30, 9);
        hkp_closest_cd_point_collector_reset(&mut e, Ptr::new(this));
        assert_eq!(e.mem.u32(this + 0x30), 0);
        assert_eq!(e.mem.u32(this + 0x2c), 0x7f7f_ffee);
        assert_eq!(e.mem.u32(this + 4), 0x7f7f_ffee);
    }

    #[test]
    fn simple_shape_phantom_constructor() {
        let mut e = engine();
        stub(&mut e, &[REF_OBJECT_CONSTRUCT]);
        let this = e.mem.alloc(0x18);
        e.mem.set_u8(this + 0x14, 7);
        assert_eq!(fn_00c6d190(&mut e, Ptr::new(this)).addr(), this);
        assert_eq!(e.mem.u32(this), VTABLE_SIMPLE_SHAPE_PHANTOM);
        assert_eq!(e.mem.u8(this + 0x14), 0);
        assert_eq!(calls(&e, REF_OBJECT_CONSTRUCT), vec![vec![this]]);
        assert_eq!(e.global::<u32>(COUNT_SIMPLE_SHAPE_PHANTOM), 1);
        assert_eq!(e.global::<u32>(COUNT_SHAPE_PHANTOM), 1);
    }

    #[test]
    fn collision_box_constructor_runs_init() {
        let mut e = engine();
        stub(&mut e, &[REF_OBJECT_CONSTRUCT]);
        e.register_double(COLLISION_BOX_INIT, check_vtable_is(VTABLE_COLLISION_BOX));
        let this = e.mem.alloc(0x20);
        assert_eq!(fn_00c6d1e0(&mut e, Ptr::new(this), 0x11, 0x22).addr(), this);
        assert_eq!(calls(&e, COLLISION_BOX_INIT), vec![vec![this, 0x11, 0x22]]);
        assert_eq!(e.global::<u32>(COUNT_SIMPLE_SHAPE_PHANTOM), 1);
    }

    #[test]
    fn collision_box_rtti() {
        let mut e = engine();
        assert_eq!(bhk_collision_box_get_rtti(&mut e, Ptr::NULL), 0x0126_8444);
    }

    #[test]
    fn collision_box_deleting_destructor_frees_the_aligned_block() {
        let mut e = engine();
        stub(&mut e, &[COLLISION_BOX_DESTRUCT, MEMORY_DEALLOCATE]);
        let block = e.mem.alloc(0x40);
        let this = block + 8;
        e.mem.set_u8(this - 1, 8);
        bhk_collision_box_scalar_deleting_destructor(&mut e, Ptr::new(this), 0);
        assert!(calls(&e, MEMORY_DEALLOCATE).is_empty());
        bhk_collision_box_scalar_deleting_destructor(&mut e, Ptr::new(this), 1);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, block]]
        );
        assert_eq!(calls(&e, COLLISION_BOX_DESTRUCT).len(), 2);
    }

    #[test]
    fn list_shape_constructor_runs_init() {
        let mut e = engine();
        stub(&mut e, &[REF_OBJECT_CONSTRUCT]);
        e.register_double(LIST_SHAPE_INIT, check_vtable_is(VTABLE_LIST_SHAPE));
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this + 0xc, 5);
        assert_eq!(fn_00c6d290(&mut e, Ptr::new(this), 0x33).addr(), this);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(calls(&e, LIST_SHAPE_INIT), vec![vec![this, 0x33]]);
        for counter in [COUNT_SHAPE, COUNT_SHAPE_COLLECTION, COUNT_LIST_SHAPE] {
            assert_eq!(e.global::<u32>(counter), 1);
        }
    }

    #[test]
    fn list_shape_rtti() {
        let mut e = engine();
        assert_eq!(bhk_list_shape_get_rtti(&mut e, Ptr::NULL), 0x0126_81a8);
    }

    #[test]
    fn list_shape_deleting_destructor_frees_0x14_bytes() {
        let mut e = engine();
        stub(&mut e, &[LIST_SHAPE_DESTRUCT, FREE_SIZED]);
        bhk_list_shape_scalar_deleting_destructor(&mut e, Ptr::new(0x5000), 1);
        assert_eq!(calls(&e, LIST_SHAPE_DESTRUCT), vec![vec![0x5000]]);
        assert_eq!(calls(&e, FREE_SIZED), vec![vec![0x5000, 0x14]]);
    }

    #[test]
    fn static_reference_release_runs_in_scope() {
        let mut e = engine();
        e.register(REMOVE_REFERENCE, |e, _| {
            let word = e.tls() + SCOPE_WORD;
            assert_eq!(e.mem.u32(word), 0x10);
            Ret::default()
        });
        let word = e.tls() + SCOPE_WORD;
        e.mem.set_u32(word, 7);
        fn_00c6d340(&mut e);
        assert!(calls(&e, REMOVE_REFERENCE).is_empty());
        e.set_global(STATIC_REFERENCED_OBJECT, 0x4321u32);
        fn_00c6d340(&mut e);
        assert_eq!(calls(&e, REMOVE_REFERENCE), vec![vec![0x4321]]);
        assert_eq!(e.mem.u32(word), 7);
        assert_eq!(e.global::<u32>(STATIC_REFERENCED_OBJECT), 0x4321);
    }

    #[test]
    fn static_ni_pointer_release() {
        let mut e = engine();
        e.register(0x0900_0004, nothing);
        let object = with_vtable(&mut e, &[(4, 0x0900_0004)]);
        returns(&mut e, INTERLOCKED_DECREMENT, 1);
        e.set_global(STATIC_NI_POINTER, object);
        fn_00c6d3c0(&mut e);
        assert_eq!(calls(&e, INTERLOCKED_DECREMENT), vec![vec![object + 4]]);
        assert!(calls(&e, 0x0900_0004).is_empty());
        assert_eq!(e.global::<u32>(STATIC_NI_POINTER), 0);
        returns(&mut e, INTERLOCKED_DECREMENT, 0);
        e.set_global(STATIC_NI_POINTER, object);
        fn_00c6d3c0(&mut e);
        assert_eq!(calls(&e, 0x0900_0004), vec![vec![object]]);
        fn_00c6d3c0(&mut e);
        assert_eq!(calls(&e, INTERLOCKED_DECREMENT).len(), 2);
    }

    #[test]
    fn set_shape_swaps_the_phantom_shape_in_its_world() {
        let mut e = engine();
        stub(
            &mut e,
            &[
                UN_MARK_READ,
                HKP_GET_POSITION,
                WORLD_REMOVE_PHANTOM,
                WORLD_ADD_PHANTOM,
            ],
        );
        e.register(0x0900_000c, nothing);
        let phantom = with_vtable(&mut e, &[(0xc, 0x0900_000c)]);
        e.mem.set_u32(phantom + 8, 0x6600);
        e.mem.set_u32(phantom + 0x10, 0x1111);
        returns(&mut e, GET_SHAPE_PHANTOM, phantom);
        let c = controller(&mut e);
        e.set(c, Ctl::phkObject, 0x5500);
        let shape = e.mem.alloc(0x10);
        e.mem.set_u32(shape + 8, 0x1111);
        assert!(!bhk_character_controller_set_shape(&mut e, c, shape, 0));
        e.mem.set_u32(shape + 8, 0x2222);
        assert!(bhk_character_controller_set_shape(&mut e, c, shape, 0));
        let all = [
            UN_MARK_READ,
            HKP_GET_POSITION,
            WORLD_REMOVE_PHANTOM,
            0x0900_000c,
            WORLD_ADD_PHANTOM,
        ];
        assert_eq!(
            order(&e, &all),
            vec![
                UN_MARK_READ,
                HKP_GET_POSITION,
                UN_MARK_READ,
                WORLD_REMOVE_PHANTOM,
                0x0900_000c,
                WORLD_ADD_PHANTOM
            ]
        );
        assert_eq!(calls(&e, UN_MARK_READ)[0], vec![c.addr()]);
        assert_eq!(calls(&e, HKP_GET_POSITION)[0], vec![0x5500]);
        assert_eq!(calls(&e, WORLD_REMOVE_PHANTOM)[0], vec![0x6600, phantom]);
        assert_eq!(calls(&e, 0x0900_000c)[0], vec![phantom, 0x2222]);
        assert_eq!(calls(&e, WORLD_ADD_PHANTOM)[0], vec![0x6600, phantom]);
        // No shape: the phantom gets none; without a proxy nothing happens.
        e.call_log = Some(vec![]);
        assert!(bhk_character_controller_set_shape(&mut e, c, 0, 0));
        assert_eq!(calls(&e, 0x0900_000c)[0], vec![phantom, 0]);
        e.set(c, Ctl::phkObject, 0);
        assert!(!bhk_character_controller_set_shape(&mut e, c, shape, 0));
    }

    #[test]
    fn collision_box_update_runs_when_there_is_one() {
        let mut e = engine();
        stub(&mut e, &[COLLISION_BOX_UPDATE]);
        let c = controller(&mut e);
        fn_00c6d4a0(&mut e, c);
        assert!(calls(&e, COLLISION_BOX_UPDATE).is_empty());
        e.set(c, Ctl::spCollisionBox, 0x4400);
        fn_00c6d4a0(&mut e, c);
        assert_eq!(calls(&e, COLLISION_BOX_UPDATE), vec![vec![0x4400]]);
    }

    #[test]
    fn velocity_mod_is_added_or_reset() {
        let mut e = engine();
        let c = controller(&mut e);
        let base = c.addr();
        e.set(c, Ctl::fVelocityTime, 0.5);
        store4(&mut e, base + 0x500, [1.0, 2.0, 3.0, 4.0]);
        store4(&mut e, base + 0x4f0, [10.0; 4]);
        bhk_character_controller_apply_velocity_mod(&mut e, c);
        assert_eq!(load4(&e, base + 0x4f0), [10.5, 11.0, 11.5, 12.0]);
        e.set(c, Ctl::iFlags, 0x1000);
        bhk_character_controller_apply_velocity_mod(&mut e, c);
        assert_eq!(load4(&e, base + 0x4f0), [10.5, 11.0, 11.5, 12.0]);
        store4(&mut e, ZERO_VECTOR, [7.0, 8.0, 9.0, 6.0]);
        for time in [0.0, f32::NAN] {
            e.set(c, Ctl::fVelocityTime, time);
            store4(&mut e, base + 0x500, [1.0; 4]);
            bhk_character_controller_apply_velocity_mod(&mut e, c);
            assert_eq!(load4(&e, base + 0x500), [7.0, 8.0, 9.0, 6.0]);
        }
    }

    /// `setTransformedPos`/`setTransformedInversePos`/`setRotatedDir` stand-in: copies the
    /// input vector to the output.
    fn copy_vector(e: &mut Engine, a: &[u32]) -> Ret {
        copy_words(e, a[0], a[2], 4);
        Ret::default()
    }

    #[test]
    fn raised_point_is_transformed() {
        let mut e = engine();
        e.register(SET_TRANSFORMED_POS, copy_vector);
        let source = e.mem.alloc(0x30);
        e.mem.set_f32(source + 0x10, 1.5);
        store4(&mut e, source + 0x20, [2.0, 3.0, 4.0, 5.0]);
        let out = e.mem.alloc(0x10);
        fn_00c6d620(&mut e, Ptr::new(source), 0x8800, out);
        assert_eq!(load4(&e, out), [2.0, 3.0, 5.5, 5.0]);
        assert_eq!(calls(&e, SET_TRANSFORMED_POS)[0][..2], [out, 0x8800]);
    }

    #[test]
    fn copy_of_0x34_bytes_quiets_the_x87_floats() {
        let mut e = engine();
        let src = filled(&mut e, 13);
        let dst = e.mem.alloc(0x34);
        assert_eq!(
            fn_00c6d670(&mut e, Ptr::new(dst), Ptr::new(src)).addr(),
            dst
        );
        assert_eq!(e.mem.bytes(dst, 0x20), e.mem.bytes(src, 0x20));
        assert_eq!(e.mem.u32(dst + 0x20), 0x7fc0_0009);
        assert_eq!(e.mem.u32(dst + 0x2c), 0x7fc0_000c);
        assert_eq!(e.mem.u32(dst + 0x30), 0x7f80_000d);
    }

    #[test]
    fn value_is_passed_to_phantom_and_box() {
        let mut e = engine();
        stub(&mut e, &[WORLD_OBJECT_PASS]);
        let c = controller(&mut e);
        e.set(c, Ctl::spCollisionBox, 0x4400);
        fn_00c6d770(&mut e, c, 9);
        e.set(c, Ctl::spShapePhantom, 0x3300);
        fn_00c6d770(&mut e, c, 8);
        assert_eq!(
            calls(&e, WORLD_OBJECT_PASS),
            vec![vec![0x4400, 9], vec![0x3300, 8], vec![0x4400, 8]]
        );
    }

    #[test]
    fn ni_object_is_set_on_phantom_and_box() {
        let mut e = engine();
        stub(&mut e, &[SET_AV_OBJECT]);
        let c = controller(&mut e);
        e.set(c, Ctl::spShapePhantom, 0x3300);
        bhk_character_controller_set_ni_object(&mut e, c, 0x77);
        e.set(c, Ctl::spCollisionBox, 0x4400);
        bhk_character_controller_set_ni_object(&mut e, c, 0x78);
        assert_eq!(
            calls(&e, SET_AV_OBJECT),
            vec![
                vec![0x3300, 0x77, 0],
                vec![0x3300, 0x78, 0],
                vec![0x4400, 0x78, 0]
            ]
        );
    }

    #[test]
    fn layer_filter_cases() {
        let mut e = engine();
        assert_eq!(fn_00c6d7e0(&mut e, 0, 0, 0, -1), 0);
        assert_eq!(fn_00c6d7e0(&mut e, 0, 0, 5, -1), 1);
        assert_eq!(fn_00c6d7e0(&mut e, 0, 3, 0, -1), 3);
        let shape = e.mem.alloc(0x10);
        assert_eq!(fn_00c6d7e0(&mut e, shape, 2, 0, -1), 2);
        let hk = e.mem.alloc(0x20);
        e.mem.set_u32(hk + 0x10, 0xe5);
        e.mem.set_u32(shape + 8, hk);
        // No callback: allowed.
        assert_eq!(fn_00c6d7e0(&mut e, shape, 1, 0, -1), 1);
        e.register(0x0900_0100, |_, a| ret(u32::from(a[0] == 5)));
        e.set_global(LAYER_CALLBACK, 0x0900_0100u32);
        assert_eq!(fn_00c6d7e0(&mut e, shape, 1, 0, -1), 0);
        assert_eq!(calls(&e, 0x0900_0100), vec![vec![5]]);
        returns(&mut e, SHAPE_LAYER, 6);
        assert_eq!(fn_00c6d7e0(&mut e, shape, 1, 0, 7), 1);
        assert_eq!(calls(&e, SHAPE_LAYER), vec![vec![hk, 7]]);
    }

    type PropertyEntry = (u32, u32, u32);

    fn property_object(e: &mut Engine, entries: &[PropertyEntry]) -> u32 {
        let object = e.mem.alloc(0x100);
        let data = e.mem.alloc(0x10 * entries.len() as u32 + 0x10);
        for (i, (key, low, high)) in entries.iter().enumerate() {
            let entry = data + 0x10 * i as u32;
            e.mem.set_u32(entry, *key);
            e.mem.set_u32(entry + 8, *low);
            e.mem.set_u32(entry + 0xc, *high);
        }
        e.mem.set_u32(object + 0x78, data);
        e.mem.set_u32(object + 0x7c, entries.len() as u32);
        object
    }

    #[test]
    fn property_lookup_returns_value_or_zero() {
        let mut e = engine();
        let object = property_object(&mut e, &[(1, 0x10, 0x11), (2, 0x20, 0x21)]);
        let out = e.mem.alloc(8);
        let r = hkp_world_object_get_property(&mut e, Ptr::new(object), Ptr::new(out), 2, 0);
        assert_eq!(r.addr(), out);
        assert_eq!((e.mem.u32(out), e.mem.u32(out + 4)), (0x20, 0x21));
        hkp_world_object_get_property(&mut e, Ptr::new(object), Ptr::new(out), 3, 0);
        assert_eq!((e.mem.u32(out), e.mem.u32(out + 4)), (0, 0));
    }

    #[test]
    fn property_presence_on_the_havok_object() {
        let mut e = engine();
        stub(&mut e, &[EMPTY_LOCK]);
        let this = e.mem.alloc(0x10);
        assert!(!fn_00c6d8a0(&mut e, Ptr::new(this), 1));
        assert!(calls(&e, EMPTY_LOCK).is_empty());
        let hk = property_object(&mut e, &[(1, 0, 0), (4, 0, 0)]);
        e.mem.set_u32(this + 8, hk);
        assert!(fn_00c6d8a0(&mut e, Ptr::new(this), 4));
        assert!(!fn_00c6d8a0(&mut e, Ptr::new(this), 5));
        assert_eq!(calls(&e, EMPTY_LOCK), vec![vec![this]; 4]);
    }

    #[test]
    fn property_0x1300_target_word() {
        let mut e = engine();
        let target = e.mem.alloc(0xd0);
        e.mem.set_u32(target + 0xc0, 0xabcd);
        let object = property_object(&mut e, &[(5, 0, 0), (0x1300, target, 0)]);
        assert_eq!(fn_00c6d900(&mut e, object), 0xabcd);
        let other = property_object(&mut e, &[(5, target, 0)]);
        assert_eq!(fn_00c6d900(&mut e, other), 0);
        assert_eq!(fn_00c6d900(&mut e, 0), 0);
    }

    #[test]
    fn shape_by_type_below_two() {
        let mut e = engine();
        let c = controller(&mut e);
        e.mem.set_u32(c.addr() + 0x5a0, 0x10);
        e.mem.set_u32(c.addr() + 0x5a4, 0x20);
        e.mem.set_u32(c.addr() + 0x5a8, 0x30);
        e.mem.set_u32(c.addr() + 0x5ac, 0x40);
        for (kind, expected) in [(-1, 0x10), (0, 0x20), (1, 0x30), (2, 0), (5, 0)] {
            e.set(c, Ctl::eShapeType, kind);
            assert_eq!(bhk_character_controller_get_shape(&mut e, c), expected);
        }
    }

    #[test]
    fn closest_collector_deleting_destructor() {
        let mut e = engine();
        install_allocator(&mut e, 0);
        let this = e.mem.alloc(0x80);
        hkp_closest_cd_point_collector_scalar_deleting_destructor(&mut e, Ptr::new(this), 0);
        assert_eq!(e.mem.u32(this), VTABLE_CD_POINT_COLLECTOR);
        assert!(calls(&e, 0x0900_0f08).is_empty());
        hkp_closest_cd_point_collector_scalar_deleting_destructor(&mut e, Ptr::new(this), 1);
        assert_eq!(calls(&e, 0x0900_0f08)[0][1..], [this, 0x80]);
    }

    #[test]
    fn three_default_points() {
        let mut e = engine();
        store4(&mut e, POINT_DEFAULT, [1.0, 2.0, 3.0, 9.0]);
        let this = e.mem.alloc(0x28);
        e.mem.set_u32(this, 5);
        assert_eq!(fn_00c6d9f0(&mut e, Ptr::new(this)).addr(), this);
        assert_eq!(e.mem.u32(this), 0);
        for copy in 0..3 {
            let at = this + 4 + 12 * copy;
            assert_eq!([0, 4, 8].map(|o| e.mem.f32(at + o)), [1.0, 2.0, 3.0]);
        }
    }

    #[test]
    fn member_tear_down() {
        let mut e = engine();
        stub(&mut e, &[REMOVE_REFERENCE]);
        e.register(0x0900_0004, nothing);
        returns(&mut e, INTERLOCKED_DECREMENT, 0);
        let pointer = with_vtable(&mut e, &[(4, 0x0900_0004)]);
        let this = e.mem.alloc(0xa8);
        e.mem.set_u32(this + 0xa4, 0x4242);
        e.mem.set_u32(this + 0xa0, pointer);
        fn_00c6de10(&mut e, Ptr::new(this));
        assert_eq!(calls(&e, REMOVE_REFERENCE), vec![vec![0x4242]]);
        assert_eq!(e.mem.u32(this + 0xa4), 0);
        assert_eq!(calls(&e, INTERLOCKED_DECREMENT), vec![vec![pointer + 4]]);
        assert_eq!(calls(&e, 0x0900_0004), vec![vec![pointer]]);
        assert_eq!(e.mem.u32(this + 0xa0), pointer);
    }

    #[test]
    fn bs_reference_is_the_property_low_word() {
        let mut e = engine();
        let object = property_object(&mut e, &[(7, 0x7777, 0x8888)]);
        assert_eq!(
            bhk_character_controller_find_bs_reference(&mut e, 7, object),
            0x7777
        );
        assert_eq!(
            bhk_character_controller_find_bs_reference(&mut e, 8, object),
            0
        );
        assert_eq!(bhk_character_controller_find_bs_reference(&mut e, 7, 0), 0);
    }

    #[test]
    fn heavy_or_type_six_bodies_block() {
        let mut e = engine();
        e.register(MOTION_TYPE_TEST, |e, a| {
            ret(u32::from(e.mem.u32(a[0]) == 2))
        });
        e.register(GET_MASS, |e, a| ret_st0(e.mem.f32(a[0] - 0xa0) as f64));
        e.set_global(BLOCKING_MASS, 10.0f32);
        let body = e.mem.alloc(0x100);
        e.mem.set_u8(body + 0xe8, 2);
        e.mem.set_f32(body + 0x40, 50.0);
        assert!(fn_00c6def0(&mut e, body));
        assert_eq!(calls(&e, GET_MASS)[0], vec![body + 0xe0]);
        e.mem.set_f32(body + 0x40, f32::NAN);
        assert!(!fn_00c6def0(&mut e, body));
        e.mem.set_u32(body + 0x2c, 0x86);
        assert!(fn_00c6def0(&mut e, body));
        e.mem.set_u8(body + 0xe8, 3);
        e.mem.set_u32(body + 0x2c, 0x7);
        assert!(!fn_00c6def0(&mut e, body));
        assert!(!fn_00c6def0(&mut e, 0));
    }

    #[test]
    fn cinfo_is_created_once() {
        let mut e = engine();
        let block = e.mem.alloc(0xc0);
        install_allocator(&mut e, block);
        // The shared state object exists: the cinfo only takes a reference to it.
        e.set_global(STATIC_REFERENCED_OBJECT, 0x7100u32);
        stub(&mut e, &[SET_PROXY_CINFO, ADD_REFERENCE, REMOVE_REFERENCE]);
        let word = e.tls() + SCOPE_WORD;
        e.mem.set_u32(word, 3);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, 1);
        let created = e.mem.alloc(4);
        assert_eq!(
            fn_00c6df60(&mut e, Ptr::new(this), Ptr::new(created)),
            block
        );
        assert_eq!(e.mem.u8(created), 1);
        assert_eq!(calls(&e, 0x0900_0f04)[0][1..], [0xc0]);
        assert_eq!(e.mem.u32(block + 0xa4), 0x7100);
        assert_eq!(calls(&e, ADD_REFERENCE), vec![vec![0x7100]]);
        assert_eq!(calls(&e, SET_PROXY_CINFO), vec![vec![this, block]]);
        assert_eq!(e.mem.u32(word), 3);
        assert_eq!(
            fn_00c6df60(&mut e, Ptr::new(this), Ptr::new(created)),
            block
        );
        assert_eq!(e.mem.u8(created), 0);
        assert_eq!(calls(&e, 0x0900_0f04).len(), 1);
    }

    #[test]
    fn property_is_added_once_sign_extended() {
        let mut e = engine();
        stub(&mut e, &[EMPTY_LOCK, ADD_PROPERTY]);
        let c = controller(&mut e);
        fn_00c6e050(&mut e, c, 9, -2);
        let phantom = e.mem.alloc(0x10);
        e.set(c, Ctl::spShapePhantom, phantom);
        let hk = property_object(&mut e, &[(3, 0, 0)]);
        e.mem.set_u32(phantom + 8, hk);
        fn_00c6e050(&mut e, c, 9, -2);
        fn_00c6e050(&mut e, c, 3, 5);
        assert_eq!(
            calls(&e, ADD_PROPERTY),
            vec![vec![hk, 9, 0xffff_fffe, 0xffff_ffff]]
        );
    }

    /// A phantom whose scene object's first child (an `NiNode` per `IS_KIND_OF`) has
    /// `children`; returns the controller.
    fn display_scene(e: &mut Engine, children: &[u32]) -> Ptr<Ctl> {
        let node = e.mem.alloc(0x100);
        let list = e.mem.alloc(0x40);
        for (i, child) in children.iter().enumerate() {
            e.mem.set_u32(list + 4 * i as u32, *child);
        }
        e.mem.set_u32(node + 0xa0, list);
        e.mem.set_u16(node + 0xa6, children.len() as u16);
        e.register_double(IS_KIND_OF, move |_, _| ret(node));
        let root = e.mem.alloc(0x100);
        let first = e.mem.alloc(4);
        let root_list = e.mem.alloc(4);
        e.mem.set_u32(root_list, first);
        e.mem.set_u32(root + 0xa0, root_list);
        e.mem.set_u16(root + 0xa6, 1);
        returns(e, 0x0900_020c, root);
        let object = with_vtable(e, &[(0xc, 0x0900_020c)]);
        returns(e, GET_AV_OBJECT, object);
        let c = controller(e);
        e.set(c, Ctl::spShapePhantom, 0x3300);
        c
    }

    #[test]
    fn display_geometry_is_found_by_name() {
        let mut e = engine();
        returns(&mut e, FIXED_STRING_CREATE, 0x5508);
        stub(&mut e, &[INTERLOCKED_DECREMENT]);
        let named = e.mem.alloc(0x10);
        e.mem.set_u32(named + 8, 0x5508);
        let other = e.mem.alloc(0x10);
        let c = display_scene(&mut e, &[0, other, named]);
        assert_eq!(bhk_character_controller_find_display_geometry(&mut e, c), 0);
        assert!(calls(&e, GET_AV_OBJECT).is_empty());
        e.set(c, Ctl::iFlags, 0x8001);
        assert_eq!(
            bhk_character_controller_find_display_geometry(&mut e, c),
            named
        );
        assert_eq!(e.get(c, Ctl::iFlags), 0x8001);
        assert_eq!(calls(&e, GET_AV_OBJECT)[0], vec![0x3300, 0]);
        assert_eq!(calls(&e, IS_KIND_OF)[0][1], NODE_RTTI);
        assert_eq!(calls(&e, FIXED_STRING_CREATE)[0], vec![DISPLAY_NAME]);
        assert_eq!(calls(&e, INTERLOCKED_DECREMENT), vec![vec![0x5500]]);
        // Not found: the flag is cleared; the empty string is not released.
        e.set_global(EMPTY_FIXED_STRING, 0x5508u32);
        e.mem.set_u32(named + 8, 0);
        assert_eq!(bhk_character_controller_find_display_geometry(&mut e, c), 0);
        assert_eq!(e.get(c, Ctl::iFlags), 1);
        assert_eq!(calls(&e, INTERLOCKED_DECREMENT).len(), 1);
    }

    #[test]
    fn radius_change_moves_the_capsule_vertices() {
        let mut e = engine();
        let hk = e.mem.alloc(0x40);
        e.mem.set_f32(hk + 0x10, 0.5);
        store4(&mut e, hk + 0x20, [1.0, 2.0, 3.0, 9.0]);
        store4(&mut e, hk + 0x30, [4.0, 5.0, 6.0, 9.0]);
        let capsule = e.mem.alloc(0x10);
        e.mem.set_u32(capsule + 8, hk);
        e.register_double(IS_KIND_OF, move |_, _| ret(capsule));
        let c = controller(&mut e);
        bhk_character_controller_set_radius(&mut e, c, 0.25);
        assert!(calls(&e, IS_KIND_OF).is_empty());
        e.set(c, Ctl::spShapes, 0x7100);
        bhk_character_controller_set_radius(&mut e, c, 0.25);
        assert_eq!(calls(&e, IS_KIND_OF)[0], vec![0x7100, CAPSULE_SHAPE_RTTI]);
        assert_eq!(load4(&e, hk + 0x20), [1.0, 2.0, 3.25, 0.5]);
        assert_eq!(load4(&e, hk + 0x30), [4.0, 5.0, 5.75, 0.5]);
        assert_eq!(e.mem.f32(hk + 0x10), 0.25);
    }

    #[test]
    fn fall_time_grows_by_the_step() {
        let mut e = engine();
        let c = controller(&mut e);
        e.set(c, Ctl::StepDeltaTime, 0.25);
        e.set(c, Ctl::fFallTime, 1.0);
        fn_00c6e2e0(&mut e, c);
        assert_eq!(e.get(c, Ctl::fFallTime), 1.25);
    }

    #[test]
    fn pitch_and_roll_rotate_the_phantom_transform() {
        let mut e = engine();
        stub(&mut e, &[QUATERNION_PREPARE, ROTATION_SET, EMPTY_LOCK]);
        e.set_global(FLOAT_ONE, 1.0f32);
        e.set_global(FLOAT_NEGATIVE_ZERO, -0.0f32);
        e.register(X87_FIRST_TRIG, |_, _| ret_st0(0.5));
        e.register(X87_SECOND_TRIG, |_, _| ret_st0(0.25));
        e.register(MATRIX3_MUL, |e, a| {
            let n = e.mem.u32(SCRATCH);
            copy_words(e, SCRATCH + 4 + 0x30 * n, a[1], 12);
            e.mem.set_u32(SCRATCH, n + 1);
            Ret::default()
        });
        e.register(PHANTOM_SET_TRANSFORM, |e, a| {
            copy_words(e, SCRATCH + 0x100, a[1], 16);
            Ret::default()
        });
        let hk = e.mem.alloc(0xf0);
        for i in 0..16 {
            e.mem.set_u32(hk + 0xb0 + 4 * i, 0x100 + i);
        }
        let phantom = e.mem.alloc(0x10);
        e.mem.set_u32(phantom + 8, hk);
        let c = controller(&mut e);
        e.set(c, Ctl::spShapePhantom, phantom);
        e.set(c, Ctl::fPitchAngle, 1.0);
        e.set(c, Ctl::fRollAngle, 2.0);
        e.set(c, Ctl::iFlags, 1 << 25);
        let rotation = vec_at(&mut e, [1.0, 2.0, 3.0, 4.0]);
        bhk_character_controller_apply_pitch_and_roll(&mut e, c, Ptr::new(rotation));
        assert_eq!(e.mem.u32(SCRATCH), 1);
        assert_eq!(load4(&e, SCRATCH + 4), [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(load4(&e, SCRATCH + 0x14), [0.0, 0.5, -0.25, 0.0]);
        assert_eq!(load4(&e, SCRATCH + 0x24), [0.0, 0.25, 0.5, 0.0]);
        assert_eq!(calls(&e, X87_FIRST_TRIG)[0], args![1.0f64]);
        let prepared = calls(&e, QUATERNION_PREPARE)[0][0];
        assert_eq!(calls(&e, ROTATION_SET)[0][1], prepared);
        for i in 0..16 {
            assert_eq!(e.mem.u32(SCRATCH + 0x100 + 4 * i), 0x100 + i);
        }
        assert_eq!(calls(&e, PHANTOM_SET_TRANSFORM)[0][0], hk);
        // Roll only.
        e.set(c, Ctl::iFlags, 1 << 26);
        bhk_character_controller_apply_pitch_and_roll(&mut e, c, Ptr::new(rotation));
        assert_eq!(e.mem.u32(SCRATCH), 2);
        assert_eq!(load4(&e, SCRATCH + 0x34), [0.5, 0.0, 0.25, 0.0]);
        assert_eq!(load4(&e, SCRATCH + 0x44), [0.0, 1.0, 0.0, 0.0]);
        assert_eq!(load4(&e, SCRATCH + 0x54), [-0.25, 0.0, 0.5, 0.0]);
        assert_eq!(calls(&e, X87_FIRST_TRIG)[1], args![2.0f64]);
    }

    #[test]
    fn ground_probe_raises_the_point_by_the_hit() {
        let mut e = engine();
        e.set_global(PROBE_DEPTH, 25.0f64);
        e.register(RAY_FILTER_INFO, |e, a| {
            e.mem.set_u32(a[1], 0x55);
            Ret::default()
        });
        e.register(0x0900_00c8, |e, a| {
            let input = a[1];
            copy_words(e, SCRATCH, input, 10);
            let collector = e.mem.u32(input + 0xa4);
            e.mem.set_u32(SCRATCH + 0x40, collector);
            e.mem.set_f32(input + 0xd0, 0.5);
            ret(e.mem.u32(SCRATCH + 0x80))
        });
        let caster = with_vtable(&mut e, &[(0xc8, 0x0900_00c8)]);
        let world = e.mem.alloc(0x360);
        e.mem.set_u32(world + 0x350, caster);
        let phantom = e.mem.alloc(0x10);
        e.mem.set_u32(phantom + 8, world);
        returns(&mut e, GET_SHAPE_PHANTOM, phantom);
        let c = controller(&mut e);
        e.set(c, Ctl::phkObject, 0x5500);
        store4(&mut e, c.addr() + 0x5c0, [1.0, 2.0, 30.0, 4.0]);
        let out = e.mem.alloc(0x10);
        fn_00c6e680(&mut e, c, 1, Ptr::new(out));
        assert_eq!(load4(&e, out), [1.0, 2.0, 5.0, 4.0]);
        assert_eq!(load4(&e, SCRATCH), [1.0, 2.0, 30.0, 4.0]);
        assert_eq!(load4(&e, SCRATCH + 0x10), [1.0, 2.0, 5.0, 4.0]);
        assert_eq!(e.mem.u32(SCRATCH + 0x20), 0);
        assert_eq!(e.mem.u32(SCRATCH + 0x24), 0x55);
        let input = calls(&e, 0x0900_00c8)[0][1];
        assert_eq!(e.mem.u32(SCRATCH + 0x40), input + 0xb0);
        assert_eq!(calls(&e, RAY_FILTER_INFO)[0][0], c.addr());
        e.mem.set_u32(SCRATCH + 0x80, 1);
        fn_00c6e680(&mut e, c, 1, Ptr::new(out));
        assert_eq!(load4(&e, out), [1.0, 2.0, 17.5, 4.0]);
    }

    #[test]
    fn front_back_points_are_placed() {
        let mut e = engine();
        e.set_global(HALF, 0.5f64);
        e.set_global(MINUS_ONE, -1.0f64);
        e.register(SET_TRANSFORMED_POS, copy_vector);
        let shape = e.mem.alloc(0x10);
        e.mem.set_u32(shape + 0xc, 9);
        let phantom = e.mem.alloc(0x20);
        e.mem.set_u32(phantom + 0x10, shape);
        returns(&mut e, GET_SHAPE_PHANTOM, phantom);
        let c = controller(&mut e);
        e.set(c, Ctl::phkObject, 0x5500);
        e.set(c, Ctl::fRadius, 0.5);
        e.set(c, Ctl::fHeight, 2.0);
        e.set(c, Ctl::fStepHeight, 0.25);
        bhk_character_controller_init_front_back(&mut e, c);
        assert!(calls(&e, SET_TRANSFORMED_POS).is_empty());
        e.set(c, Ctl::iFlags, 1);
        bhk_character_controller_init_front_back(&mut e, c);
        let points = |e: &Engine| [0, 1, 2, 3].map(|i| load4(e, c.addr() + 0x5b0 + 0x10 * i));
        assert_eq!(
            points(&e),
            [
                [0.0, 0.5, 0.75, 0.0],
                [0.0, -0.5, 0.75, 0.0],
                [-0.5, 0.0, 0.75, 0.0],
                [0.5, 0.0, 0.75, 0.0]
            ]
        );
        assert_eq!(
            calls(&e, SET_TRANSFORMED_POS)[0][..2],
            [c.addr() + 0x5b0, phantom + 0xb0]
        );
        e.mem.set_u32(shape + 0xc, 8);
        bhk_character_controller_init_front_back(&mut e, c);
        assert_eq!(points(&e)[0], [0.0, 0.5, 0.0, 0.0]);
    }

    #[test]
    fn shape_type_filter_follows_type_ten() {
        let mut e = engine();
        let inner = e.mem.alloc(0x40);
        e.mem.set_u32(inner + 0xc, 0x1e);
        let outer = e.mem.alloc(0x40);
        e.mem.set_u32(outer + 0xc, 10);
        e.mem.set_u32(outer + 0x34, inner);
        assert_eq!(fn_00c6ead0(&mut e, outer), inner);
        e.mem.set_u32(inner + 0xc, 0x20);
        assert_eq!(fn_00c6ead0(&mut e, outer), 0);
        e.mem.set_u32(outer + 0xc, 0x17);
        assert_eq!(fn_00c6ead0(&mut e, outer), outer);
        assert_eq!(fn_00c6ead0(&mut e, 0), 0);
    }

    /// The object callback's world: a body with `mass` (read by the `GET_MASS` double from
    /// the motion's `+0x20`) and `motion`, the impulse `(3,4,0,0)`. Returns the controller,
    /// the body, the event and the impulse.
    fn object_event(e: &mut Engine, mass: f32, motion: u32) -> [u32; 4] {
        e.set_global(MAX_PUSH_MASS, 95.0f32);
        e.set_global(IMPULSE_DEFAULT, 2.0f32);
        e.set_global(FLOAT_TENTH, 0.1f32);
        e.set_global(FLOAT_THREE, 3.0f32);
        e.set_global(FLOAT_HALF, 0.5f32);
        e.set_global(HALF, 0.5f64);
        e.set_global(RATIO_LIMIT, 1000.0f64);
        e.set_global(HEIGHT_FRACTION, 0.9f32 as f64);
        e.register(GET_MASS, |e, a| ret_st0(e.mem.f32(a[0] + 0x20) as f64));
        stub(e, &[ADD_REFERENCE, REMOVE_REFERENCE]);
        let body = e.mem.alloc(0x110);
        e.mem.set_f32(body + 0x100, mass);
        e.mem.set_u32(body + 0x2c, motion);
        let event = e.mem.alloc(0x40);
        e.mem.set_u32(event + 0x30, body);
        let impulse = vec_at(e, [3.0, 4.0, 0.0, 0.0]);
        let c = controller(e);
        [c.addr(), body, event, impulse]
    }

    fn object_callback(e: &mut Engine, c: u32, event: u32, impulse: u32) {
        let listener = Ptr::new(c + LISTENER);
        bhk_character_controller_object_interaction_callback(
            e,
            listener,
            0,
            Ptr::new(event),
            Ptr::new(impulse),
        );
    }

    #[test]
    fn object_interaction_scales_the_impulse() {
        let mut e = engine();
        let [c, body, event, impulse] = object_event(&mut e, 10.0, 0x81);
        let ctl = Ptr::<Ctl>::new(c);
        e.set(ctl, Ctl::spBumpedBody, 0x9900);
        object_callback(&mut e, c, event, impulse);
        assert_eq!(load4(&e, impulse), [6.0, 8.0, 0.0, 0.0]);
        assert_eq!(e.get(ctl, Ctl::fBumpedForce), 10.0);
        assert_eq!(e.get(ctl, Ctl::spBumpedBody), body);
        assert_eq!(calls(&e, ADD_REFERENCE), vec![vec![body]]);
        assert_eq!(calls(&e, REMOVE_REFERENCE), vec![vec![0x9900]]);
        // A weaker push leaves the bumped body alone.
        store4(&mut e, impulse, [0.3, 0.4, 0.0, 0.0]);
        object_callback(&mut e, c, event, impulse);
        assert_eq!(calls(&e, ADD_REFERENCE).len(), 1);
        // Type 10 above half the mass limit: 0.1.
        let [c, _, event, impulse] = object_event(&mut e, 50.0, 10);
        object_callback(&mut e, c, event, impulse);
        let force = e.get(Ptr::<Ctl>::new(c), Ctl::fBumpedForce);
        assert_eq!(force, (5.0f64 * 0.1f32 as f64) as f32);
        // Out of the mass range: zero impulse, the force is the length.
        let [c, _, event, impulse] = object_event(&mut e, 95.0, 1);
        object_callback(&mut e, c, event, impulse);
        assert_eq!(load4(&e, impulse), [0.0; 4]);
        assert_eq!(e.get(Ptr::<Ctl>::new(c), Ctl::fBumpedForce), 5.0);
    }

    #[test]
    fn object_interaction_from_above_follows_the_velocity() {
        let mut e = engine();
        let [c, _, event, impulse] = object_event(&mut e, 0.001, 1);
        let ctl = Ptr::<Ctl>::new(c);
        // `GetPosition_ov2` without a proxy: the zero vector minus the rotated centre (0).
        e.register(SET_ROTATED_DIR, copy_vector);
        store4(&mut e, event, [0.0, 0.0, 2.0, 0.0]);
        e.set(ctl, Ctl::fHeight, 1.0);
        e.set(ctl, Ctl::iFlags, 8);
        store4(&mut e, c + 0x4f0, [0.0, 2.0, 0.0, 0.0]);
        object_callback(&mut e, c, event, impulse);
        // Ratio 5000 > 1000: factor 2 * 1000 / 5000.
        let ratio = (5.0f64 / 0.001f32 as f64) as f32;
        let factor = (1000.0 / ratio as f64 * 2.0) as f32;
        let inverse = inverse_length(4.0, 0.0, 0.5, 3.0);
        let y = factor * (inverse * 2.0 * 5.0);
        assert_eq!(load4(&e, impulse), [0.0, y, 0.0, 0.0]);
        assert_eq!(calls(&e, SET_ROTATED_DIR)[0][2], c + 0x570);
    }

    fn character_world(e: &mut Engine) {
        e.set_global(FLOAT_THREE, 3.0f32);
        e.set_global(FLOAT_HALF, 0.5f32);
        e.set_global(HALF, 0.5f64);
        e.set_global(ROT_MOD_DECAY, 0.05f32 as f64);
        e.set_global(PUSH_SCALE, -0.25f64);
        e.set_global(LARGE_SIZE_BONUS, 20.0f64);
        e.set_global(MODIFIER_TIME, 0.01f32);
        e.set_global(ROT_MOD_TIME, 0.2f32);
        e.set_global(MINUS_ONE, -1.0f64);
        stub(
            e,
            &[ADD_REFERENCE, REMOVE_REFERENCE, HKP_GET_LINEAR_VELOCITY],
        );
        e.register(COPY_POINT, |e, a| {
            copy_words(e, a[0], a[1], 4);
            ret(a[0])
        });
        // `SetVelocityModifier` runs for real with an inverse scale of 1.
        e.set_global(INVERSE_SCALE, 1.0f32);
        e.register(X87_SQRT, |_, a| {
            ret_st0(f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32).sqrt())
        });
        e.register(FLOAT_MIN, |_, a| {
            let (low, high) = (f32::from_bits(a[0]), f32::from_bits(a[1]));
            ret_st0(if high > low { low } else { high } as f64)
        });
        e.register(FLOAT_MAX, |_, a| {
            let (low, high) = (f32::from_bits(a[0]), f32::from_bits(a[1]));
            ret_st0(if high < low { low } else { high } as f64)
        });
    }

    #[test]
    fn character_interaction_pushes_and_turns_the_other() {
        let mut e = engine();
        character_world(&mut e);
        let velocity = vec_at(&mut e, [0.0, -2.0, 7.0, 0.0]);
        returns(&mut e, GET_HK_LINEAR_VELOCITY, velocity);
        let position = vec_at(&mut e, [0.0, 0.0, 9.0, 0.0]);
        returns(&mut e, GET_HK_POSITION, position);
        let c = controller(&mut e);
        let other = controller(&mut e);
        e.set(c, Ctl::iPriority, 2);
        e.set(other, Ctl::iPriority, 1);
        e.set(other, Ctl::iFlags, 1);
        e.set(other, Ctl::fHeight, 4.0);
        store4(&mut e, other.addr() + 0x4d0, [1.0, 0.0, 3.0, 0.0]);
        let proxy = e.mem.alloc(0xd0);
        e.mem.set_u32(proxy + 0xc0, other.addr());
        let event = e.mem.alloc(0x20);
        store4(&mut e, event, [1.0, 0.0, 0.0, 0.0]);
        store4(&mut e, event + 0x10, [0.0, 1.0, 5.0, 0.0]);
        e.set(c, Ctl::spBumpedCharProxy, 0x9900);
        let listener = Ptr::new(c.addr() + LISTENER);
        let (proxy_ptr, event_ptr) = (Ptr::new(proxy), Ptr::new(event));
        bhk_character_controller_character_interaction_callback(
            &mut e, listener, 0, proxy_ptr, event_ptr,
        );
        assert_eq!(e.get(c, Ctl::spBumpedCharProxy), proxy);
        assert_eq!(calls(&e, REMOVE_REFERENCE), vec![vec![0x9900]]);
        assert_eq!(calls(&e, HKP_GET_LINEAR_VELOCITY), vec![vec![proxy]]);
        // Push: -0.25 * speed 2 along the normalized normal.
        let unit = inverse_length(1.0, 0.0, 0.5, 3.0);
        let push = [0.0, -0.5f32 * unit, 0.0, 0.0];
        assert_eq!(load4(&e, other.addr() + 0x580), push);
        assert_eq!(e.get(other, Ctl::iPushCount), 1);
        let inverse = (1.0 / 0.01f32 as f64) as f32;
        let modifier = push.map(|v| inverse * v);
        assert_eq!(load4(&e, other.addr() + 0x500), modifier);
        assert_eq!(e.get(other, Ctl::fVelocityTime), 0.01);
        assert_eq!(calls(&e, GET_HK_POSITION), vec![vec![other.addr()]]);
        // Turn: facing about 1, reach about 1 / 2 - 0.05, doubled.
        let turn = e.get(other, Ctl::fRotMod);
        assert!((turn - 0.9).abs() < 1e-4, "{turn}");
        assert_eq!(e.get(other, Ctl::fRotModTime), 0.2);
        // A higher-priority other is left alone.
        e.set(other, Ctl::iPriority, 3);
        bhk_character_controller_character_interaction_callback(
            &mut e, listener, 0, proxy_ptr, event_ptr,
        );
        assert_eq!(e.get(other, Ctl::iPushCount), 1);
    }

    #[test]
    fn inside_check_casts_outward_then_inward() {
        let mut e = engine();
        e.set_global(QUARTER, 0.25f64);
        e.set_global(THIRTY_TWO, 32.0f64);
        e.register(SET_ROTATED_DIR, copy_vector);
        e.register(SET_TRANSFORMED_INVERSE_POS, copy_vector);
        // Each cast logs its start z and hits on the cast number held at SCRATCH + 4.
        e.register(0x0900_0020, |e, a| {
            let n = e.mem.u32(SCRATCH) + 1;
            e.mem.set_u32(SCRATCH, n);
            let from = e.mem.f32(a[2] + 8);
            e.mem.set_f32(SCRATCH + 8 * n, from);
            let hit = n == e.mem.u32(SCRATCH + 4);
            e.mem.set_f32(a[3] + 0x10, if hit { 0.5 } else { 1.0 });
            Ret::default()
        });
        let shape = with_vtable(&mut e, &[(0x20, 0x0900_0020)]);
        e.mem.set_u32(shape + 0xc, 8);
        let skipped = e.mem.alloc(0x20);
        e.mem.set_u32(skipped + 0x1c, 0x84);
        e.mem.set_u32(skipped, shape);
        let tested = e.mem.alloc(0x20);
        e.mem.set_u32(tested + 0x1c, 0x81);
        e.mem.set_u32(tested, shape);
        e.mem.set_u32(tested + 8, 0x7700);
        let entries = e.mem.alloc(0x20);
        e.mem.set_u32(entries + 4, skipped);
        e.mem.set_u32(entries + 0xc, tested);
        let hk = e.mem.alloc(0x170);
        e.mem.set_u32(hk + 0x160, entries);
        e.mem.set_u32(hk + 0x164, 2);
        let phantom = e.mem.alloc(0x10);
        e.mem.set_u32(phantom + 8, hk);
        let c = controller(&mut e);
        e.set(c, Ctl::spShapePhantom, phantom);
        e.set(c, Ctl::fActorHeight, 4.0);
        // `GetRadius` without a capsule shape: `fRadius`.
        e.set(c, Ctl::fRadius, 0.5);
        // No hit: six casts, three outward (from the position) and three inward.
        assert!(!bhk_character_controller_check_inside_of_object(&mut e, c));
        assert_eq!(e.mem.u32(SCRATCH), 6);
        let froms = [1, 2, 3, 4, 5, 6].map(|n| e.mem.f32(SCRATCH + 8 * n));
        assert_eq!(froms, [1.0, 2.0, 3.0, 1.0, 2.0, 3.0]);
        assert_eq!(calls(&e, SET_ROTATED_DIR)[0][1], hk + 0xb0);
        assert_eq!(calls(&e, SET_TRANSFORMED_INVERSE_POS)[0][1], 0x7700);
        // An outward hit skips the inward casts; an inward hit returns true.
        e.mem.set_u32(SCRATCH, 0);
        e.mem.set_u32(SCRATCH + 4, 2);
        assert!(!bhk_character_controller_check_inside_of_object(&mut e, c));
        assert_eq!(e.mem.u32(SCRATCH), 2);
        e.mem.set_u32(SCRATCH, 0);
        e.mem.set_u32(SCRATCH + 4, 5);
        assert!(bhk_character_controller_check_inside_of_object(&mut e, c));
        assert_eq!(e.mem.u32(SCRATCH), 5);
    }

    #[test]
    fn display_geometry_is_built_under_the_node() {
        let mut e = engine();
        e.set_global(EMPTY_FIXED_STRING, 0x5508u32);
        stub(&mut e, &[SET_NAME, INTERLOCKED_DECREMENT]);
        returns(&mut e, FIXED_STRING_CREATE, 0x5508);
        returns(&mut e, HEAP_ALLOCATE, 0x6000);
        let display = e.mem.alloc(0x100);
        returns(&mut e, NODE_CONSTRUCT, display);
        e.register(COPY_POINT, |e, a| {
            copy_words(e, a[0], a[1], 4);
            ret(a[0])
        });
        e.register(0x0900_00dc, nothing);
        e.register(0x0900_00d4, nothing);
        let node = with_vtable(&mut e, &[(0xdc, 0x0900_00dc)]);
        e.mem.set_f32(node + 0x98, -2.0);
        let shape = with_vtable(&mut e, &[(0xd4, 0x0900_00d4)]);
        let c = controller(&mut e);
        e.set(c, Ctl::spShapes, shape);
        store4(&mut e, c.addr() + 0x570, [2.0, 4.0, 6.0, 0.0]);
        let word = e.tls() + SCOPE_WORD;
        e.mem.set_u32(word, 5);
        bhk_character_controller_build_display_geometry(&mut e, c, node);
        assert_eq!(calls(&e, HEAP_ALLOCATE), vec![vec![NODE_SIZE]]);
        assert_eq!(calls(&e, NODE_CONSTRUCT), vec![vec![0x6000, 0]]);
        assert_eq!(calls(&e, 0x0900_00dc), vec![vec![node, display, 1]]);
        assert_eq!(calls(&e, SET_NAME)[0][0], display);
        assert!(calls(&e, INTERLOCKED_DECREMENT).is_empty());
        let placed = [0x58, 0x5c, 0x60, 0x64].map(|o| e.mem.f32(display + o));
        assert_eq!(placed, [-1.0, -2.0, -3.0, 0.5]);
        assert_eq!(
            calls(&e, 0x0900_00d4),
            vec![vec![shape, display, DISPLAY_ARGUMENT]]
        );
        assert_eq!(e.get(c, Ctl::iFlags), 0x8000);
        assert_eq!(e.mem.u32(word), 5);
        // Without a current shape nothing is built.
        e.call_log = Some(vec![]);
        e.set(c, Ctl::iFlags, 0);
        e.set(c, Ctl::eShapeType, 2);
        bhk_character_controller_build_display_geometry(&mut e, c, node);
        assert!(calls(&e, HEAP_ALLOCATE).is_empty());
        assert_eq!(e.get(c, Ctl::iFlags), 0);
    }

    /// A Havok allocator whose slot 4 hands out fresh blocks of the asked size.
    fn install_fresh_allocator(e: &mut Engine) {
        e.register(0x0900_0f04, |e, a| ret(e.mem.alloc(a[1])));
        e.register(0x0900_0f08, nothing);
        e.register(0x0900_0f10, nothing);
        let allocator = with_vtable(
            e,
            &[(4, 0x0900_0f04), (8, 0x0900_0f08), (0x10, 0x0900_0f10)],
        );
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        returns(e, MEMORY_ROUTER, router);
    }

    fn sqrt_double(_: &mut Engine, a: &[u32]) -> Ret {
        ret_st0(f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32).sqrt())
    }

    /// A controller whose class vtable has a double at each of the given slots.
    fn controller_with_slots(e: &mut Engine, slots: &[u32]) -> Ptr<Ctl> {
        let c = controller(e);
        let vtable = e.mem.alloc(0x200);
        for slot in slots {
            e.register(0x0900_0000 + slot, nothing);
            e.mem.set_u32(vtable + slot, 0x0900_0000 + slot);
        }
        e.mem.set_u32(c.addr(), vtable);
        c
    }

    #[test]
    fn update_state_moves_the_wanted_state() {
        let mut e = engine();
        let c = controller(&mut e);
        e.set(c, Ctl::eWantState, 3);
        bhk_character_controller_update_state(&mut e, c);
        assert_eq!(e.get(c, Ctl::m_currentState), 3);
        assert_eq!(e.get(c, Ctl::eWantState), 0xb);
        e.set(c, Ctl::m_currentState, 7);
        bhk_character_controller_update_state(&mut e, c);
        assert_eq!(e.get(c, Ctl::m_currentState), 7);
    }

    #[test]
    fn proxy_cinfo_defaults() {
        let mut e = engine();
        e.set_global(FLOAT_ONE, 1.0f32);
        e.set_global(FLOAT_TENTH, 0x7f80_0001u32);
        e.set_global(FLOAT_TWENTIETH, 0.05f32);
        e.set_global(FLOAT_TEN, 10.0f32);
        e.set_global(COLLECTOR_FAR, 0x7f7f_ffeeu32);
        e.set_global(FLOAT_HALF_PI, std::f32::consts::FRAC_PI_2);
        let info = e.mem.alloc(0x80);
        store4(&mut e, info, [5.0; 4]);
        e.mem.set_u32(info + 0x2c, 0xdead);
        fn_00c6cde0(&mut e, Ptr::new(info));
        assert_eq!(load4(&e, info), [0.0; 4]);
        assert_eq!(load4(&e, info + 0x10), [0.0; 4]);
        assert_eq!([0x20, 0x24].map(|o| e.mem.f32(info + o)), [1.0, 0.0]);
        assert_eq!(e.mem.u32(info + 0x28), 0x7fc0_0001);
        assert_eq!(e.mem.u32(info + 0x2c), 0xdead);
        assert_eq!(load4(&e, info + 0x30), [0.0, 1.0, 0.0, 0.0]);
        let floats = [0x40, 0x44, 0x4c, 0x50, 0x58, 0x60, 0x64, 0x68].map(|o| e.mem.f32(info + o));
        let half_pi = std::f32::consts::FRAC_PI_2;
        assert_eq!(floats, [0.0, 0.0, 0.05, 10.0, 10.0, 0.0, half_pi, 1.0]);
        let words = [0x48, 0x54, 0x5c, 0x6c].map(|o| e.mem.u32(info + o));
        assert_eq!(words, [0, 4, 0x7f7f_ffee, 10]);
        assert_eq!(e.mem.u8(info + 0x70), 0);
    }

    #[test]
    fn character_proxy_constructor() {
        let mut e = engine();
        stub(&mut e, &[REF_OBJECT_CONSTRUCT, POINT_COLLECTOR_CONSTRUCT]);
        e.set_global(COUNT_CHARACTER_PROXY, 4u32);
        let p = e.mem.alloc(0x3e0);
        e.mem.set_u32(p + 0xc, 9);
        assert_eq!(fn_00c6ce80(&mut e, Ptr::new(p)).addr(), p);
        assert_eq!(e.mem.u32(p), VTABLE_CHARACTER_PROXY);
        assert_eq!(e.mem.u32(p + 0xc), 0);
        assert_eq!(calls(&e, REF_OBJECT_CONSTRUCT), vec![vec![p]]);
        assert_eq!(
            calls(&e, POINT_COLLECTOR_CONSTRUCT),
            vec![vec![p + 0x10, 0]]
        );
        assert_eq!(e.global::<u32>(COUNT_CHARACTER_PROXY), 5);
    }

    #[test]
    fn check_support_runs_between_unmark_writes() {
        let mut e = engine();
        stub(&mut e, &[UN_MARK_WRITE, HKP_CHECK_SUPPORT]);
        let p = e.mem.alloc(0x10);
        bhk_character_proxy_check_support(&mut e, Ptr::new(p), 1, 2);
        bhk_character_proxy_check_support(&mut e, Ptr::NULL, 1, 2);
        assert!(calls(&e, UN_MARK_WRITE).is_empty());
        e.mem.set_u32(p + 8, 0x5500);
        bhk_character_proxy_check_support(&mut e, Ptr::new(p), 1, 2);
        let seen = order(&e, &[UN_MARK_WRITE, HKP_CHECK_SUPPORT]);
        assert_eq!(seen, [UN_MARK_WRITE, HKP_CHECK_SUPPORT, UN_MARK_WRITE]);
        assert_eq!(calls(&e, HKP_CHECK_SUPPORT), vec![vec![0x5500, 1, 2]]);
    }

    #[test]
    fn jump_height_is_scaled() {
        let mut e = engine();
        e.set_global(INVERSE_SCALE, 2.0f32);
        let c = controller(&mut e);
        fn_00c6d4b0(&mut e, c, 3.0);
        assert_eq!(e.get(c, Ctl::fJumpHeight), 6.0);
        assert_eq!(e.get(c, Ctl::eWantState), 1);
    }

    #[test]
    fn velocity_modifier_keeps_the_longer_one() {
        let mut e = engine();
        e.set_global(INVERSE_SCALE, 2.0f32);
        let c = controller(&mut e);
        let velocity = vec_at(&mut e, [1.0, 2.0, 3.0, 9.0]);
        bhk_character_controller_set_velocity_modifier(&mut e, c, Ptr::new(velocity), 0.5);
        assert_eq!(load4(&e, c.addr() + 0x500), [4.0, 8.0, 12.0, 0.0]);
        assert_eq!(e.get(c, Ctl::fVelocityTime), 0.5);
        // Shorter, or NaN: kept.
        bhk_character_controller_set_velocity_modifier(&mut e, c, Ptr::new(velocity), 2.0);
        bhk_character_controller_set_velocity_modifier(&mut e, c, Ptr::new(velocity), f32::NAN);
        assert_eq!(load4(&e, c.addr() + 0x500), [4.0, 8.0, 12.0, 0.0]);
        assert_eq!(e.get(c, Ctl::fVelocityTime), 0.5);
    }

    #[test]
    fn lod_distance_from_the_reference_point() {
        let mut e = engine();
        e.register(X87_SQRT, sqrt_double);
        let c = controller(&mut e);
        let reference = e.mem.alloc(0x30);
        let vector = vec_at(&mut e, [1.0, 1.0, 1.0, 0.5]);
        e.mem.set_u32(reference + 0x20, vector);
        let point = vec_at(&mut e, [4.0, 5.0, 1.0, 0.0]);
        fn_00c6d6b0(&mut e, c, Ptr::new(point), reference);
        assert_eq!(e.get(c, Ctl::fLODDistance), 4.5);
        // The fallback point: not above 0 gives 0.
        e.mem.set_u32(reference + 0x20, 0);
        store4(&mut e, POINT_FALLBACK, [0.0, 0.0, 0.0, 10.0]);
        store4(&mut e, point, [3.0, 4.0, 0.0, 0.0]);
        fn_00c6d6b0(&mut e, c, Ptr::new(point), reference);
        assert_eq!(e.get(c, Ctl::fLODDistance), 0.0);
        e.set(c, Ctl::fLODDistance, 9.0);
        fn_00c6d6b0(&mut e, c, Ptr::new(point), 0);
        assert_eq!(e.get(c, Ctl::fLODDistance), 9.0);
    }

    #[test]
    fn proxy_part_word_is_cleared() {
        let mut e = engine();
        let part = e.mem.alloc(0x10);
        e.mem.set_u32(part + 4, 5);
        returns(&mut e, PROXY_PART_74, part);
        let p = e.mem.alloc(0x10);
        fn_00c6e030(&mut e, Ptr::new(p));
        fn_00c6e030(&mut e, Ptr::NULL);
        assert_eq!(e.mem.u32(part + 4), 5);
        e.mem.set_u32(p + 8, 0x5500);
        fn_00c6e030(&mut e, Ptr::new(p));
        assert_eq!(e.mem.u32(part + 4), 0);
        assert_eq!(calls(&e, PROXY_PART_74), vec![vec![0x5500]]);
    }

    #[test]
    fn radius_of_the_capsule_or_the_controller() {
        let mut e = engine();
        e.set_global(DEFAULT_RADIUS, 0.1f32);
        e.register(IS_KIND_OF, |e, _| ret(e.mem.u32(SCRATCH)));
        let c = controller(&mut e);
        e.set(c, Ctl::fRadius, f32::from_bits(0x7f80_0002));
        assert_eq!(
            bhk_character_controller_get_radius(&mut e, c).to_bits(),
            0x7fc0_0002
        );
        let hk = e.mem.alloc(0x20);
        e.mem.set_f32(hk + 0x10, 2.5);
        let capsule = e.mem.alloc(0x10);
        e.mem.set_u32(capsule + 8, hk);
        e.mem.set_u32(SCRATCH, capsule);
        e.set(c, Ctl::spShapes, 0x4400);
        assert_eq!(bhk_character_controller_get_radius(&mut e, c), 2.5);
        assert_eq!(calls(&e, IS_KIND_OF)[0], vec![0x4400, CAPSULE_SHAPE_RTTI]);
        e.mem.set_u32(capsule + 8, 0);
        assert_eq!(bhk_character_controller_get_radius(&mut e, c), 0.1);
        e.mem.set_u32(SCRATCH, 0);
        e.set(c, Ctl::fRadius, 0.75);
        assert_eq!(bhk_character_controller_get_radius(&mut e, c), 0.75);
    }

    #[test]
    fn position_is_the_proxy_minus_the_rotated_centre() {
        let mut e = engine();
        e.register(SET_ROTATED_DIR, |e, a| {
            store4(e, a[0], [1.0, 2.0, 3.0, 4.0]);
            Ret::default()
        });
        stub(&mut e, &[UN_MARK_READ]);
        let base = vec_at(&mut e, [10.0, 20.0, 30.0, 40.0]);
        returns(&mut e, HKP_GET_POSITION, base);
        let c = controller(&mut e);
        e.set(c, Ctl::phkObject, 0x5500);
        let out = e.mem.alloc(0x10);
        bhk_character_controller_get_position_ov2(&mut e, c, Ptr::new(out));
        assert_eq!(load4(&e, out), [9.0, 18.0, 27.0, 36.0]);
        let seen = order(&e, &[UN_MARK_READ, HKP_GET_POSITION]);
        assert_eq!(seen, [UN_MARK_READ, HKP_GET_POSITION, UN_MARK_READ]);
        assert_eq!(calls(&e, SET_ROTATED_DIR)[0][1..], [0xb0, c.addr() + 0x570]);
        // No proxy: from the zero vector.
        e.set(c, Ctl::phkObject, 0);
        bhk_character_controller_get_position_ov2(&mut e, c, Ptr::new(out));
        assert_eq!(load4(&e, out), [-1.0, -2.0, -3.0, -4.0]);
    }

    #[test]
    fn position_is_set_with_the_rotated_centre() {
        let mut e = engine();
        e.register(SET_ROTATED_DIR, |e, a| {
            store4(e, a[0], [1.0; 4]);
            Ret::default()
        });
        stub(&mut e, &[UN_MARK_WRITE]);
        e.register(HKP_SET_POSITION, |e, a| {
            copy_words(e, SCRATCH, a[1], 4);
            Ret::default()
        });
        let proxy = e.mem.alloc(0x40);
        store4(&mut e, proxy + 0x20, [7.0; 4]);
        let phantom = e.mem.alloc(0x10);
        e.mem.set_u32(phantom + 8, 0x7700);
        let c = controller(&mut e);
        e.set(c, Ctl::spShapePhantom, phantom);
        let position = vec_at(&mut e, [1.0, 2.0, 3.0, 4.0]);
        bhk_character_controller_set_position_ov2(&mut e, c, Ptr::new(position));
        assert!(calls(&e, HKP_SET_POSITION).is_empty());
        e.set(c, Ctl::phkObject, proxy);
        bhk_character_controller_set_position_ov2(&mut e, c, Ptr::new(position));
        assert_eq!(load4(&e, SCRATCH), [2.0, 3.0, 4.0, 5.0]);
        assert_eq!(load4(&e, proxy + 0x20), [0.0; 4]);
        assert_eq!(calls(&e, HKP_SET_POSITION)[0][0], proxy);
        assert_eq!(calls(&e, SET_ROTATED_DIR)[0][1], 0x77b0);
        assert_eq!(calls(&e, UN_MARK_WRITE).len(), 2);
    }

    #[test]
    fn support_comes_from_the_body() {
        let mut e = engine();
        let c = controller(&mut e);
        e.set(c, Ctl::eSupportMaterial, 0x24);
        let root = e.mem.alloc(0x200);
        e.mem.set_u32(root + 0x1c, 0x85);
        e.mem.set_u8(root + 0x18, 1);
        e.mem.set_u8(root + 0x10, 0x10);
        store4(&mut e, root + 0x10 + 0x1b0, [1.0, 2.0, 3.0, 4.0]);
        let body = e.mem.alloc(0x20);
        e.mem.set_u32(body + 0xc, root);
        let hk = e.mem.alloc(0x20);
        e.mem.set_u32(hk + 0x10, 0x2f);
        let shape = e.mem.alloc(0x10);
        e.mem.set_u32(shape + 8, hk);
        e.mem.set_u32(body, shape);
        let info = e.mem.alloc(0x50);
        e.mem.set_u32(info + 0x48, body);
        e.mem.set_u32(info + 0x4c, u32::MAX);
        e.set(c, Ctl::iFlags, 0x0100_0000);
        fn_00c6e980(&mut e, c, info);
        assert_eq!(e.get(c, Ctl::eSupportLayer), 5);
        assert_eq!(load4(&e, c.addr() + 0x440), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(e.get(c, Ctl::eSupportMaterial), 0xf);
        assert_eq!(e.get(c, Ctl::iFlags), 0x2_0000);
        // Locked while the material is 0xF.
        e.set(c, Ctl::eSupportLayer, 0);
        fn_00c6e980(&mut e, c, info);
        assert_eq!(e.get(c, Ctl::eSupportLayer), 0);
        // A shape key: the material from `00c7ffd0`'s slot 0xE0; the callback blocks it.
        e.set(c, Ctl::eSupportMaterial, 0x24);
        e.mem.set_u32(info + 0x4c, 3);
        e.register(0x0900_00e0, |_, _| ret(7));
        let source = with_vtable(&mut e, &[(0xe0, 0x0900_00e0)]);
        returns(&mut e, BODY_MATERIAL_SOURCE, source);
        e.register(0x0900_0bc8, |_, _| ret(1));
        e.set_global(LAYER_CALLBACK, 0x0900_0bc8u32);
        e.set(c, Ctl::iFlags, 0x3_0000);
        fn_00c6e980(&mut e, c, info);
        assert_eq!(e.get(c, Ctl::eSupportMaterial), 7);
        assert_eq!(e.get(c, Ctl::iFlags), 0x0101_0000);
        assert_eq!(calls(&e, BODY_MATERIAL_SOURCE), vec![vec![body]]);
        assert_eq!(calls(&e, 0x0900_00e0), vec![vec![source, 3]]);
        assert_eq!(calls(&e, 0x0900_0bc8), vec![vec![7]]);
    }

    #[test]
    fn bumped_and_support_words() {
        let mut e = engine();
        let c = controller(&mut e);
        assert_eq!(fn_00c6fd20(&mut e, c), 0);
        assert_eq!(fn_00c6fd40(&mut e, c), 0);
        assert_eq!(fn_00c6fd60(&mut e, c), 0);
        let object = e.mem.alloc(0xd0);
        e.mem.set_u32(object + 0xc, 0x11);
        e.mem.set_u32(object + 0xc0, 0x22);
        e.set(c, Ctl::spSupportBody, object);
        e.set(c, Ctl::spBumpedBody, object);
        e.set(c, Ctl::spBumpedCharProxy, object);
        assert_eq!(fn_00c6fd20(&mut e, c), 0x11);
        assert_eq!(fn_00c6fd40(&mut e, c), 0x11);
        assert_eq!(fn_00c6fd60(&mut e, c), 0x22);
    }

    #[test]
    fn display_geometry_is_detached_and_released() {
        let mut e = engine();
        returns(&mut e, FIXED_STRING_CREATE, 0x5508);
        e.register(INTERLOCKED_DECREMENT, |_, _| ret(0));
        e.register(0x0900_0004, nothing);
        let removed = with_vtable(&mut e, &[(4, 0x0900_0004)]);
        e.register_double(0x0900_00e4, move |e, a| {
            e.mem.set_u32(a[2], removed);
            Ret::default()
        });
        let parent = with_vtable(&mut e, &[(0xe4, 0x0900_00e4)]);
        let named = e.mem.alloc(0x20);
        e.mem.set_u32(named + 8, 0x5508);
        e.mem.set_u32(named + 0x18, parent);
        let c = display_scene(&mut e, &[named]);
        e.set(c, Ctl::iFlags, 0x8001);
        bhk_character_controller_kill_display_geometry(&mut e, c);
        assert_eq!(calls(&e, 0x0900_00e4)[0][..2], [parent, named]);
        assert_eq!(e.get(c, Ctl::iFlags), 1);
        assert_eq!(calls(&e, 0x0900_0004), vec![vec![removed]]);
        assert!(calls(&e, INTERLOCKED_DECREMENT).contains(&vec![removed + 4]));
        // Nothing displayed: nothing detached.
        e.call_log = Some(vec![]);
        bhk_character_controller_kill_display_geometry(&mut e, c);
        assert!(calls(&e, 0x0900_00e4).is_empty());
        assert!(calls(&e, 0x0900_0004).is_empty());
    }

    fn tilt_world(e: &mut Engine) {
        e.set_global(MAX_TILT_DEGREES, 47.0f32);
        e.set_global(DEGREES_TO_RADIANS, f64::from_bits(0x3f91_df46_a000_0000));
        e.set_global(
            NEGATIVE_DEGREES_TO_RADIANS,
            f64::from_bits(0xbf91_df46_a000_0000),
        );
        e.set_global(MINUS_ONE, -1.0f64);
        e.set_global(SUPPORT_UNSET, f32::from_bits(0x0080_0000));
    }

    #[test]
    fn pitch_and_roll_follow_the_support_heights() {
        let mut e = engine();
        tilt_world(&mut e);
        e.register(X87_SLOPE_HELPER, |_, a| {
            ret_st0(f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32).atan())
        });
        let c = controller(&mut e);
        e.set(c, Ctl::iFlags, 1 | 1 << 25 | 1 << 26);
        e.set(c, Ctl::fCalculatePitchTimer, 1.0);
        e.set(c, Ctl::fHeight, 2.0);
        e.set(c, Ctl::fPitchMult, 2.0);
        e.set(c, Ctl::fRollAngle, 0.25);
        store4(&mut e, c.addr() + 0x460, [0.0, 1.0, 2.0, 2.0]);
        fn_00c70060(&mut e, c);
        assert_eq!(e.get(c, Ctl::fPitchAngle), 0.5f64.atan() as f32);
        assert_eq!(e.get(c, Ctl::fPitchMult), 1.0);
        assert_eq!(e.get(c, Ctl::fRollAngle), 0.0);
        assert_eq!(calls(&e, X87_SLOPE_HELPER).len(), 1);
    }

    #[test]
    fn pitch_and_roll_follow_the_support_normal() {
        let mut e = engine();
        tilt_world(&mut e);
        e.register(TRANSFORM_VECTOR, |e, a| {
            store4(e, a[0], [0.5, -2.0, 0.0, 0.0]);
            Ret::default()
        });
        let c = controller(&mut e);
        e.set(c, Ctl::iFlags, 1 << 25 | 1 << 26);
        e.set(c, Ctl::fCalculatePitchTimer, 1.0);
        e.set(c, Ctl::fPitchMult, 0.5);
        fn_00c70060(&mut e, c);
        let limit = (47.0f64 * f64::from_bits(0x3f91_df46_a000_0000)) as f32;
        assert_eq!(e.get(c, Ctl::fRollAngle), -0.25);
        assert_eq!(e.get(c, Ctl::fPitchAngle), (-limit as f64 * 0.5) as f32);
        assert_eq!(
            calls(&e, TRANSFORM_VECTOR)[0][1..],
            [0xb0, c.addr() + 0x450]
        );
        // Without the timer nothing moves.
        e.set(c, Ctl::fCalculatePitchTimer, 0.0);
        e.set(c, Ctl::fRollAngle, 3.0);
        fn_00c70060(&mut e, c);
        assert_eq!(e.get(c, Ctl::fRollAngle), 3.0);
    }

    #[test]
    fn fall_distance_from_the_start_height() {
        let mut e = engine();
        e.register(SET_ROTATED_DIR, copy_vector);
        e.register(COPY_POINT, |e, a| {
            copy_words(e, a[0], a[1], 4);
            ret(a[0])
        });
        store4(&mut e, ZERO_VECTOR, [0.0, 0.0, 3.0, 0.0]);
        let c = controller(&mut e);
        assert_eq!(bhk_character_controller_get_fall_distance(&mut e, c), 0.0);
        assert!(calls(&e, COPY_POINT).is_empty());
        e.set(c, Ctl::fFallStartHeight, 10.0);
        assert_eq!(bhk_character_controller_get_fall_distance(&mut e, c), 7.0);
    }

    /// A controller in a 16-byte-aligned block (offset byte 0x10) ready for its
    /// destructor: every callee doubled, the proxy's phantom in a world.
    fn destructor_scene(e: &mut Engine) -> Ptr<Ctl> {
        install_allocator(e, 0);
        stub(
            e,
            &[
                UN_MARK_WRITE,
                PROXY_LISTENER_REMOVE,
                REMOVE_REFERENCE,
                LIST_DESTRUCT,
                EH_VECTOR_DESTRUCTOR,
                CONTEXT_DESTRUCT,
                CHARACTER_PROXY_DESTRUCT,
                MEMORY_DEALLOCATE,
            ],
        );
        e.register(INTERLOCKED_DECREMENT, |_, _| ret(1));
        let world = e.mem.alloc(0x360);
        e.mem.set_u32(world + 0x350, 1);
        let phantom = e.mem.alloc(0x10);
        e.mem.set_u32(phantom + 8, world);
        returns(e, GET_SHAPE_PHANTOM, phantom);
        let block = e.mem.alloc(0x670);
        let c = Ptr::<Ctl>::new(block + 0x10);
        e.mem.set_u8(c.addr() - 1, 0x10);
        e.set(c, Ctl::phkObject, 0x5500);
        e.set(c, Ctl::spBumpedBody, 0x6300);
        e.set(c, Ctl::spBumpedCharProxy, 0x6400);
        e.set(c, Ctl::spSupportBody, 0x6500);
        e.set(c, Ctl::LastManifold, 0x6600);
        e.set(c, Ctl::LastManifoldSize, 3);
        e.set(c, Ctl::LastManifoldCapacity, 2);
        let collision_box = e.mem.alloc(0x10);
        e.set(c, Ctl::spCollisionBox, collision_box);
        let shape_phantom = e.mem.alloc(0x10);
        e.set(c, Ctl::spShapePhantom, shape_phantom);
        c
    }

    #[test]
    fn destructor_releases_everything() {
        let mut e = engine();
        let c = destructor_scene(&mut e);
        let me = c.addr();
        let word = e.tls() + SCOPE_WORD;
        e.mem.set_u32(word, 9);
        fn_00c705c0(&mut e, c);
        assert_eq!(e.mem.u32(me), VTABLE_CONTROLLER);
        assert_eq!(e.mem.u32(me + 0x3e0), VTABLE_CONTROLLER_CONTEXT);
        assert_eq!(e.mem.u32(me + 0x410), VTABLE_PROXY_LISTENER);
        assert_eq!(
            calls(&e, PROXY_LISTENER_REMOVE),
            vec![vec![0x5500, me + 0x410]]
        );
        let released = calls(&e, REMOVE_REFERENCE);
        assert_eq!(released, vec![vec![0x6300], vec![0x6400], vec![0x6500]]);
        assert_eq!(e.get(c, Ctl::spBumpedBody), 0);
        assert_eq!(e.get(c, Ctl::spSupportBody), 0);
        assert_eq!(calls(&e, 0x0900_0f10)[0][1..], [0x6600, 2 * 0x70]);
        assert_eq!(e.get(c, Ctl::LastManifold), 0);
        assert_eq!(e.get(c, Ctl::LastManifoldSize), 0);
        assert_eq!(e.get(c, Ctl::LastManifoldCapacity), 0x8000_0000);
        assert_eq!(
            calls(&e, EH_VECTOR_DESTRUCTOR),
            vec![vec![me + 0x5a4, 4, 2, NI_POINTER_DESTRUCT]]
        );
        let box_and_phantom = calls(&e, INTERLOCKED_DECREMENT);
        let collision_box = e.get(c, Ctl::spCollisionBox);
        let shape_phantom = e.get(c, Ctl::spShapePhantom);
        assert_eq!(
            box_and_phantom,
            vec![vec![collision_box + 4], vec![shape_phantom + 4]]
        );
        let tail = [
            LIST_DESTRUCT,
            EH_VECTOR_DESTRUCTOR,
            CONTEXT_DESTRUCT,
            CHARACTER_PROXY_DESTRUCT,
        ];
        assert_eq!(order(&e, &tail), tail);
        assert_eq!(calls(&e, CONTEXT_DESTRUCT), vec![vec![me + 0x3e0]]);
        assert_eq!(calls(&e, LIST_DESTRUCT), vec![vec![me + 0x648]]);
        assert_eq!(e.mem.u32(word), 9);
    }

    #[test]
    fn deleting_destructor_frees_the_aligned_block() {
        let mut e = engine();
        let c = destructor_scene(&mut e);
        assert_eq!(fn_00c70c60(&mut e, c, 1), c);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, c.addr() - 0x10]]
        );
        let c = destructor_scene(&mut e);
        e.call_log = Some(vec![]);
        fn_00c70c60(&mut e, c, 0);
        assert!(calls(&e, MEMORY_DEALLOCATE).is_empty());
        assert_eq!(calls(&e, CHARACTER_PROXY_DESTRUCT), vec![vec![c.addr()]]);
    }

    #[test]
    fn listener_thunk_reaches_the_controller() {
        let mut e = engine();
        let c = destructor_scene(&mut e);
        let result = fn_00c70810(&mut e, Ptr::new(c.addr() + 0x410), 1);
        assert_eq!(result.addr(), c.addr());
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, c.addr() - 0x10]]
        );
    }

    #[test]
    fn context_thunk_reaches_the_controller() {
        let mut e = engine();
        let c = destructor_scene(&mut e);
        let result = fn_00c70820(&mut e, Ptr::new(c.addr() + 0x3e0), 0);
        assert_eq!(result.addr(), c.addr());
        assert!(calls(&e, MEMORY_DEALLOCATE).is_empty());
        assert_eq!(calls(&e, CHARACTER_PROXY_DESTRUCT), vec![vec![c.addr()]]);
    }

    #[test]
    fn shape_type_switch_rebuilds_the_display() {
        let mut e = engine();
        let c = controller_with_slots(&mut e, &[0xd0]);
        returns(&mut e, GET_HK_POSITION, 0x7777);
        e.mem.set_u32(c.addr() + 0x5a8, 0x4400);
        e.set(c, Ctl::iFlags, 0x8000);
        bhk_character_controller_set_shape_type(&mut e, c, 1);
        assert_eq!(e.get(c, Ctl::eShapeType), 1);
        assert_eq!(e.get(c, Ctl::iFlags), 0);
        assert_eq!(calls(&e, GET_HK_POSITION), vec![vec![c.addr()]]);
        assert_eq!(calls(&e, 0x0900_00d0), vec![vec![c.addr(), 0]]);
        // The same type, a type of 2 and an empty slot are refused.
        e.call_log = Some(vec![]);
        bhk_character_controller_set_shape_type(&mut e, c, 1);
        bhk_character_controller_set_shape_type(&mut e, c, 2);
        bhk_character_controller_set_shape_type(&mut e, c, 0);
        assert_eq!(e.get(c, Ctl::eShapeType), 1);
        assert!(calls(&e, GET_HK_POSITION).is_empty());
    }

    #[test]
    fn size_is_limited_to_half_the_height() {
        let mut e = engine();
        e.set_global(HALF, 0.5f64);
        let c = controller(&mut e);
        e.set(c, Ctl::fHeight, 4.0);
        e.set(c, Ctl::fRadius, 0.7);
        e.set(c, Ctl::eSizedShapeType, 2);
        bhk_character_controller_set_size(&mut e, c, 3.0);
        assert_eq!(e.get(c, Ctl::fDestRadius), 2.0);
        assert_eq!(e.get(c, Ctl::eSize), 2);
        assert_eq!(e.get(c, Ctl::eSizedShapeType), 0);
        e.set(c, Ctl::eSize, 0);
        bhk_character_controller_set_size(&mut e, c, 2.0);
        assert_eq!(e.get(c, Ctl::eSize), 0);
        bhk_character_controller_set_size(&mut e, c, -1.0);
        assert_eq!(e.get(c, Ctl::fDestRadius), 0.7);
        assert_eq!(e.get(c, Ctl::eSize), 2);
        e.set(c, Ctl::eSize, 0);
        bhk_character_controller_set_size(&mut e, c, f32::NAN);
        assert!(e.get(c, Ctl::fDestRadius).is_nan());
        assert_eq!(e.get(c, Ctl::eSize), 2);
        e.set(c, Ctl::iFlags, 1);
        bhk_character_controller_set_size(&mut e, c, 1.5);
        assert!(e.get(c, Ctl::fDestRadius).is_nan());
    }

    #[test]
    fn size_change_steps_the_capsule() {
        let mut e = engine();
        e.set_global(FIVE, 5.0f64);
        let c = controller(&mut e);
        e.set(c, Ctl::eSize, 2);
        e.set(c, Ctl::fDestRadius, 0.5);
        e.set(c, Ctl::fRadius, 0.5);
        e.set(c, Ctl::eSizedShapeType, 1);
        // No capsule: the radius is reached at once; the sized shape type is applied.
        bhk_character_controller_change_size(&mut e, c, 0.02);
        assert_eq!(e.get(c, Ctl::eSize), 0);
        assert_eq!(e.get(c, Ctl::eSizedShapeType), 2);
        // A capsule steps by rate * 5 towards it.
        let hk = e.mem.alloc(0x40);
        e.mem.set_f32(hk + 0x10, 1.0);
        store4(&mut e, hk + 0x20, [0.0, 0.0, 1.0, 0.0]);
        store4(&mut e, hk + 0x30, [0.0, 0.0, -1.0, 0.0]);
        let capsule = e.mem.alloc(0x10);
        e.mem.set_u32(capsule + 8, hk);
        returns(&mut e, IS_KIND_OF, capsule);
        e.set(c, Ctl::spShapes, 0x4400);
        e.set(c, Ctl::eSize, 2);
        e.set(c, Ctl::fRadius, 0.3);
        bhk_character_controller_change_size(&mut e, c, 0.02);
        let step = (0.02f32 as f64 * 5.0) as f32;
        let next = (1.0f64 - step as f64) as f32;
        let up = (1.0f64 - (-step) as f64) as f32;
        let down = ((-step) as f64 + -1.0f64) as f32;
        assert_eq!(load4(&e, hk + 0x20), [0.0, 0.0, up, 1.0]);
        assert_eq!(load4(&e, hk + 0x30), [0.0, 0.0, down, 1.0]);
        assert_eq!(e.mem.f32(hk + 0x10), next);
        assert_eq!(e.get(c, Ctl::eSize), 2);
        // The last step stops at the destination; the radius differs: size 1.
        e.mem.set_f32(hk + 0x10, 0.55);
        bhk_character_controller_change_size(&mut e, c, 0.02);
        assert_eq!(e.mem.f32(hk + 0x10), 0.5);
        assert_eq!(e.get(c, Ctl::eSize), 1);
    }

    #[test]
    fn immobile_move_or_size_change() {
        let mut e = engine();
        returns(&mut e, GET_STATE, 2);
        store4(&mut e, POINT_DEFAULT, [1.0, 2.0, 3.0, 0.0]);
        let c = controller_with_slots(&mut e, &[0xc8]);
        let params = e.mem.alloc(0x30);
        bhk_character_controller_move_immobile(&mut e, c, Ptr::new(params), 1);
        let moved = [0x10, 0x14, 0x18].map(|o| e.mem.f32(params + o));
        assert_eq!(moved, [1.0, 2.0, 3.0]);
        assert_eq!(calls(&e, 0x0900_00c8), vec![vec![c.addr(), params]]);
        assert_eq!(calls(&e, GET_STATE)[0], vec![c.addr() + 0x3e0]);
        // Without the flag: the size change instead.
        e.set(c, Ctl::eSize, 2);
        e.set(c, Ctl::fDestRadius, 0.5);
        e.mem.set_f32(params, 0.01);
        bhk_character_controller_move_immobile(&mut e, c, Ptr::new(params), 0);
        assert_eq!(calls(&e, 0x0900_00c8).len(), 1);
        assert_eq!(e.get(c, Ctl::eSize), 1);
    }

    #[test]
    fn damage_impacts_are_emptied() {
        let mut e = engine();
        stub(&mut e, &[MEMORY_DEALLOCATE, LIST_DESTRUCT]);
        e.register(INTERLOCKED_DECREMENT, |_, _| ret(0));
        e.register(0x0900_0004, nothing);
        let object = with_vtable(&mut e, &[(4, 0x0900_0004)]);
        let first = e.mem.alloc(8);
        e.mem.set_u32(first, object);
        let second = e.mem.alloc(8);
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, second);
        let c = controller(&mut e);
        e.set(c, Ctl::DamageImpacts, first);
        e.set(c, Ctl::DamageImpactsNext, node);
        fn_00c70bd0(&mut e, c);
        assert_eq!(e.get(c, Ctl::DamageImpacts), 0);
        assert_eq!(e.get(c, Ctl::DamageImpactsNext), 0);
        assert_eq!(calls(&e, 0x0900_0004), vec![vec![object]]);
        assert_eq!(calls(&e, LIST_DESTRUCT), vec![vec![node]]);
        let freed: Vec<u32> = calls(&e, MEMORY_DEALLOCATE).iter().map(|c| c[1]).collect();
        assert_eq!(freed, [first, node, second]);
    }

    #[test]
    fn world_change_moves_phantom_and_box() {
        let mut e = engine();
        e.set_global(HUNDREDTH, 0.01f64);
        e.set_global(INVERSE_SCALE, 100.0f32);
        e.set_global(HALF, 0.5f64);
        e.set_global(DEBUG_ALPHA, 0.75f32);
        stub(&mut e, &[PROXY_LEAVE_WORLD, SET_DEBUG_OBJECT_COLOR]);
        returns(&mut e, RGB_FROM_FLOATS, 0xc0102);
        let part = e.mem.alloc(0x10);
        e.mem.set_u32(part + 4, 9);
        returns(&mut e, PROXY_PART_74, part);
        let world = e.mem.alloc(0x360);
        e.mem.set_u32(world + 0x350, 0x1111);
        let hk_phantom = e.mem.alloc(0x10);
        e.mem.set_u32(hk_phantom + 8, world);
        returns(&mut e, GET_SHAPE_PHANTOM, hk_phantom);
        e.register(0x0900_00a0, nothing);
        e.register(0x0900_009c, nothing);
        let slots = [(0xa0, 0x0900_00a0), (0x9c, 0x0900_009c)];
        let phantom = with_vtable(&mut e, &slots);
        let collision_box = with_vtable(&mut e, &slots);
        let c = controller(&mut e);
        e.set(c, Ctl::phkObject, 0x5500);
        e.set(c, Ctl::spShapePhantom, phantom);
        e.set(c, Ctl::spCollisionBox, collision_box);
        e.set(c, Ctl::fHeight, 4.0);
        bhk_character_controller_set_world(&mut e, c, 0x2222);
        assert_eq!(calls(&e, PROXY_LEAVE_WORLD), vec![vec![c.addr()]]);
        assert_eq!(
            calls(&e, 0x0900_00a0),
            vec![vec![phantom], vec![collision_box]]
        );
        assert_eq!(e.mem.u32(part + 4), 0);
        assert_eq!(
            calls(&e, 0x0900_009c),
            vec![vec![phantom, 0x2222], vec![collision_box, 0x2222]]
        );
        let one = 1.0f32.to_bits();
        let color = vec![one, one, 0, 0.75f32.to_bits()];
        assert_eq!(calls(&e, RGB_FROM_FLOATS), vec![color]);
        assert_eq!(
            calls(&e, SET_DEBUG_OBJECT_COLOR),
            vec![vec![0x2222, phantom, 0xc0102]]
        );
        // Sized to 1 (100 * 0.01), then to the radius 0.
        assert_eq!(e.get(c, Ctl::fDestRadius), 0.0);
        assert_eq!(e.get(c, Ctl::eSize), 2);
        // Already in that world: nothing.
        e.call_log = Some(vec![]);
        e.mem.set_u32(world + 0x350, 0x2222);
        bhk_character_controller_set_world(&mut e, c, 0x2222);
        assert!(calls(&e, PROXY_LEAVE_WORLD).is_empty());
        assert!(calls(&e, 0x0900_009c).is_empty());
    }

    #[test]
    fn convex_vertices_are_two_rings() {
        let mut e = engine();
        install_allocator(&mut e, 0);
        e.set_global(INVERSE_SCALE, 2.0f32);
        e.set_global(TENTH, 0.1f64);
        e.set_global(HALF, 0.5f64);
        for i in 0..8 {
            e.set_global(RING_X + 4 * i, i as f32);
            e.set_global(RING_Y + 4 * i, -(i as f32));
        }
        e.register(ARRAY_RESERVE, |e, a| {
            let data = e.mem.alloc(a[2] * 0x10);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        e.register(SHAPE_FROM_VERTICES, |e, a| {
            let data = e.mem.u32(a[0]);
            let size = e.mem.u32(a[0] + 4);
            e.mem.set_u32(SCRATCH, size);
            copy_words(e, SCRATCH + 0x10, data, size * 4);
            ret(0x5150)
        });
        let word = e.tls() + SCOPE_WORD;
        e.mem.set_u32(word, 4);
        let upper = vec_at(&mut e, [1.0, 2.0, 3.0, 0.0]);
        let lower = vec_at(&mut e, [4.0, 5.0, 6.0, 0.0]);
        let shape = fn_00c70de0(&mut e, Ptr::new(upper), Ptr::new(lower), 0.5, 2.0);
        assert_eq!(shape, 0x5150);
        assert_eq!(e.mem.u32(SCRATCH), 18);
        let vertex = |e: &Engine, n: u32| load4(e, SCRATCH + 0x10 + 0x10 * n);
        assert_eq!(vertex(&e, 0), [8.0, 10.0, 11.0, 0.0]);
        let z = (2.0f64 + 0.1 + 11.0) as f32;
        for i in 0..8 {
            let x = (i as f64 + 8.0) as f32;
            let y = (-(i as f32) as f64 + 10.0) as f32;
            assert_eq!(vertex(&e, 1 + i), [x, y, z, 0.0]);
            assert_eq!(vertex(&e, 9 + i), [i as f32, -(i as f32), 6.5, 0.0]);
        }
        assert_eq!(vertex(&e, 17), [2.0, 4.0, 7.0, 0.0]);
        assert_eq!(calls(&e, ARRAY_RESERVE)[0][2..], [0x12, 0x10]);
        assert_eq!(calls(&e, 0x0900_0f10)[0][2], 0x120);
        assert_eq!(e.mem.u32(word), 4);
    }

    #[test]
    fn word_arrays_are_freed() {
        let mut e = engine();
        install_allocator(&mut e, 0);
        let this = e.mem.alloc(0x20);
        for (o, w) in [
            (4, 0x200),
            (8, 1),
            (0xc, 0x8000_0003),
            (0x10, 0x100),
            (0x14, 2),
            (0x18, 5),
        ] {
            e.mem.set_u32(this + o, w);
        }
        fn_00c71120(&mut e, Ptr::new(this));
        let freed: Vec<Vec<u32>> = calls(&e, 0x0900_0f10)
            .iter()
            .map(|c| c[1..].to_vec())
            .collect();
        assert_eq!(freed, vec![vec![0x100, 20]]);
        let words = [4, 8, 0xc, 0x10, 0x14, 0x18].map(|o| e.mem.u32(this + o));
        assert_eq!(words, [0, 0, 0x8000_0000, 0, 0, 0x8000_0000]);
    }

    #[test]
    fn entry_array_is_freed() {
        let mut e = engine();
        install_allocator(&mut e, 0);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0xc, 0x300);
        e.mem.set_u32(this + 0x10, 2);
        e.mem.set_u32(this + 0x14, 3);
        fn_00c72210(&mut e, Ptr::new(this));
        assert_eq!(calls(&e, 0x0900_0f10)[0][1..], [0x300, 24]);
        let words = [0xc, 0x10, 0x14].map(|o| e.mem.u32(this + o));
        assert_eq!(words, [0, 0, 0x8000_0000]);
    }

    #[test]
    fn projectile_moves_or_casts() {
        let mut e = engine();
        e.register(SET_ROTATED_DIR, copy_vector);
        stub(&mut e, &[UN_MARK_READ, UN_MARK_WRITE]);
        e.register(HKP_SET_POSITION, |e, a| {
            copy_words(e, SCRATCH, a[1], 4);
            Ret::default()
        });
        let base = vec_at(&mut e, [1.0, 2.0, 3.0, 4.0]);
        returns(&mut e, HKP_GET_POSITION, base);
        let proxy = e.mem.alloc(0x40);
        let c = controller(&mut e);
        e.set(c, Ctl::phkObject, proxy);
        store4(&mut e, c.addr() + 0x4f0, [2.0; 4]);
        bhk_character_controller_move_as_projectile(&mut e, c, 0.5);
        assert_eq!(load4(&e, SCRATCH), [2.0, 3.0, 4.0, 5.0]);
        assert!(calls(&e, PHANTOM_LINEAR_CAST).is_empty());
        // Flag bit 19 with a phantom: cast, then each of the two hits to the listener.
        e.set_global(COLLECTOR_FAR, 0x7f7f_ffeeu32);
        e.set_global(CAST_TOLERANCE, 0x3400_0000u32);
        e.register(PHANTOM_LINEAR_CAST, |e, a| {
            let collector = a[3];
            e.mem.set_u32(collector + 0x14, 2);
            copy_words(e, SCRATCH + 0x20, a[2], 6);
            let seen = [0, 4, 0x18].map(|o| e.mem.u32(collector + o));
            for (i, word) in seen.into_iter().enumerate() {
                e.mem.set_u32(SCRATCH + 0x40 + 4 * i as u32, word);
            }
            let inline = e.mem.u32(collector + 0x10).wrapping_sub(collector);
            e.mem.set_u32(SCRATCH + 0x4c, inline);
            Ret::default()
        });
        stub(&mut e, &[ALL_COLLECTOR_SORT_HITS, ALL_COLLECTOR_DESTRUCT]);
        e.register(0x0900_0008, nothing);
        let listener_vtable = e.mem.alloc(0x20);
        e.mem.set_u32(listener_vtable + 8, 0x0900_0008);
        e.mem.set_u32(c.addr() + 0x410, listener_vtable);
        e.set(c, Ctl::iFlags, 1 << 19);
        let phantom = e.mem.alloc(0x10);
        e.set(c, Ctl::spShapePhantom, phantom);
        bhk_character_controller_move_as_projectile(&mut e, c, 0.5);
        let cast = calls(&e, PHANTOM_LINEAR_CAST)[0].clone();
        assert_eq!([cast[0], cast[4]], [phantom, 0]);
        assert_eq!(load4(&e, SCRATCH + 0x20), [2.0, 3.0, 4.0, 5.0]);
        assert_eq!(e.mem.u32(SCRATCH + 0x30), 0x3400_0000);
        assert_eq!(e.mem.u32(SCRATCH + 0x34), 0x3400_0000);
        let collector = [0x40, 0x44, 0x48, 0x4c].map(|o| e.mem.u32(SCRATCH + o));
        assert_eq!(
            collector,
            [
                VTABLE_ALL_CD_POINT_COLLECTOR,
                0x7f7f_ffee,
                0x8000_0008,
                0x20
            ]
        );
        let hits = calls(&e, 0x0900_0008);
        let listener = c.addr() + 0x410;
        let first = cast[3] + 0x20;
        assert_eq!(
            hits,
            vec![vec![listener, 0, first], vec![listener, 0, first + 0x70]]
        );
        assert_eq!(calls(&e, ALL_COLLECTOR_SORT_HITS), vec![vec![cast[3]]]);
        assert_eq!(calls(&e, ALL_COLLECTOR_DESTRUCT), vec![vec![cast[3]]]);
        assert_eq!(load4(&e, SCRATCH), [2.0, 3.0, 4.0, 5.0]);
    }

    /// Globals and doubles for `00c72410`: the inverse scale 0.5, capsules logged at
    /// `SCRATCH + 0x10 + 0x30 * n` (two vertices and the radius word, `+8` = 0x6600 + n).
    fn shapes_world(e: &mut Engine) {
        e.set_global(INVERSE_SCALE, 0.5f32);
        e.set_global(HALF, 0.5f64);
        e.set_global(MINUS_ONE, -1.0f64);
        e.set_global(TWO, 2.0f64);
        e.set_global(THREE_QUARTERS, 0.75f64);
        e.set_global(EIGHT, 8.0f64);
        e.set_global(DOUBLE_ONE, 1.0f64);
        e.set_global(DEFAULT_WIDTH, 23.0f64);
        e.set_global(DEFAULT_DEPTH, 17.5f64);
        e.set_global(DEFAULT_HEIGHT, 64.0f64);
        e.set_global(STATIC_NI_POINTER, 0x7300u32);
        stub(e, &[INTERLOCKED_INCREMENT, NI_POINTER_COPY]);
        e.register(NI_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(HEAP_ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(CAPSULE_CONSTRUCT, |e, a| {
            let n = e.mem.u32(SCRATCH);
            e.mem.set_u32(SCRATCH, n + 1);
            let at = SCRATCH + 0x10 + 0x30 * n;
            copy_words(e, at, a[1], 4);
            copy_words(e, at + 0x10, a[2], 4);
            e.mem.set_u32(at + 0x20, a[3]);
            e.mem.set_u32(a[0] + 8, 0x6600 + n);
            ret(a[0])
        });
    }

    /// Capsule `n` logged by `shapes_world`: its two vertices (xyz) and radius.
    fn capsule_log(e: &Engine, n: u32) -> ([f32; 3], [f32; 3], f32) {
        let at = SCRATCH + 0x10 + 0x30 * n;
        let xyz = |o: u32| [0, 4, 8].map(|lane| e.mem.f32(at + o + lane));
        (xyz(0), xyz(0x10), e.mem.f32(at + 0x20))
    }

    #[test]
    fn shapes_from_a_given_shape() {
        let mut e = engine();
        shapes_world(&mut e);
        e.set_global(SCALE, 3.0f32);
        let hk = e.mem.alloc(0x20);
        e.mem.set_f32(hk + 0x10, 2.0);
        let shape = e.mem.alloc(0x10);
        e.mem.set_u32(shape + 8, hk);
        let info = e.mem.alloc(0xc0);
        e.mem.set_u32(info + 0xa0, shape);
        let c = controller(&mut e);
        fn_00c72410(&mut e, c, info);
        assert_eq!(e.get(c, Ctl::spShapes), shape);
        assert_eq!(calls(&e, INTERLOCKED_INCREMENT), vec![vec![shape + 4]]);
        assert_eq!(e.get(c, Ctl::fRadius), 3.0);
        assert_eq!(e.get(c, Ctl::fDestRadius), 3.0);
        assert_eq!(e.get(c, Ctl::fHeight), 0.0);
        assert_eq!(e.get(c, Ctl::eSizedShapeType), 2);
        assert_eq!(e.mem.u32(SCRATCH), 0);
    }

    #[test]
    fn shapes_for_a_standing_controller() {
        let mut e = engine();
        shapes_world(&mut e);
        let info = e.mem.alloc(0xc0);
        e.mem.set_u8(info + 0x96, 1);
        let c = controller(&mut e);
        fn_00c72410(&mut e, c, info);
        assert_eq!(e.get(c, Ctl::iFlags) & 1, 0);
        let radius = ((17.5f64 + 23.0) * 0.5) as f32;
        let (a, b, r) = capsule_log(&e, 0);
        assert_eq!(a, [0.0, 0.0, 43.75]);
        assert_eq!(b, [0.0, 0.0, -43.75]);
        assert_eq!(r, radius);
        assert_eq!(e.mem.u32(SCRATCH), 1);
        let step = ((0.75 * radius as f64) as f32 as f64 * 0.5) as f32;
        assert_eq!(e.get(c, Ctl::fStepHeight), step);
        assert_eq!(e.get(c, Ctl::fCenter), 32.0);
        assert_eq!(e.mem.f32(c.addr() + 0x578), 32.0);
        assert_eq!(e.mem.f32(info + 0x50), 10.125);
        let assigned = calls(&e, NI_POINTER_ASSIGN);
        assert_eq!(assigned[0][0], c.addr() + 0x5a4);
        assert_eq!(
            calls(&e, NI_POINTER_COPY),
            vec![vec![c.addr() + 0x5a8, STATIC_NI_POINTER]]
        );
        assert_eq!(e.get(c, Ctl::fRadius), 10.125);
        assert_eq!(e.get(c, Ctl::fHeight), 64.0);
        assert_eq!(e.get(c, Ctl::fActorHeight), 64.0);
    }

    #[test]
    fn shapes_for_a_lying_controller_make_a_list() {
        let mut e = engine();
        shapes_world(&mut e);
        install_allocator(&mut e, 0);
        stub(&mut e, &[REF_OBJECT_CONSTRUCT]);
        e.register(ARRAY_RESERVE_MORE, |e, a| {
            let array = a[1];
            let size = e.mem.u32(array + 4);
            let data = e.mem.alloc(4 * (size + 1));
            let old = e.mem.u32(array);
            copy_words(e, data, old, size);
            e.mem.set_u32(array, data);
            e.mem.set_u32(array + 8, size + 1);
            Ret::default()
        });
        // The list's two word arrays, as `bhkListShape::Init` sees them.
        e.register(LIST_SHAPE_INIT, |e, a| {
            for (n, array) in [a[1] + 4, a[1] + 0x10].into_iter().enumerate() {
                let at = SCRATCH + 0x100 + 0x10 * n as u32;
                let size = e.mem.u32(array + 4);
                e.mem.set_u32(at, size);
                let data = e.mem.u32(array);
                copy_words(e, at + 4, data, size);
            }
            Ret::default()
        });
        let info = e.mem.alloc(0xc0);
        e.mem.set_u8(info + 0x96, 1);
        e.mem.set_f32(info + 0x4c, 1.5);
        let c = controller(&mut e);
        e.set(c, Ctl::iFlags, 1);
        fn_00c72410(&mut e, c, info);
        assert_eq!(e.mem.u32(SCRATCH), 3);
        assert_eq!(
            capsule_log(&e, 0),
            ([5.5, 0.0, 110.5], [-5.5, 0.0, 110.5], 17.5)
        );
        assert_eq!(
            capsule_log(&e, 1),
            ([5.5, -8.0, 47.5], [5.5, -8.0, 45.5], 45.5)
        );
        assert_eq!(
            capsule_log(&e, 2),
            ([-5.5, 8.0, 47.5], [-5.5, 8.0, 45.5], 45.5)
        );
        let first = [0, 4, 8, 12].map(|o| e.mem.u32(SCRATCH + 0x100 + o));
        assert_eq!(first, [3, 0x6600, 0x6601, 0x6602]);
        let second = [0, 4, 8, 12].map(|o| e.mem.u32(SCRATCH + 0x110 + o));
        assert_eq!(second, [3, 0, 0, 0]);
        let list = e.get(c, Ctl::spShapes);
        assert_eq!(e.mem.u32(list), VTABLE_LIST_SHAPE);
        assert_eq!(calls(&e, 0x0900_0f10).len(), 2);
        assert_eq!(e.get(c, Ctl::fCenter), 3.0);
        assert_eq!(e.mem.f32(c.addr() + 0x578), 3.0);
        assert_eq!(e.mem.f32(info + 0x50), 8.75);
        assert_eq!(e.get(c, Ctl::fRadius), 8.75);
        assert_eq!(e.get(c, Ctl::fHeight), 23.0);
        assert_eq!(e.get(c, Ctl::fActorHeight), 64.0);
    }

    /// Doubles for `Move`: a profiling stream, the zero-centred position, the state
    /// object (slot 0x20 at `0x0900_0020`), and a controller with slot 0x94 (the world).
    fn move_world(e: &mut Engine) -> (Ptr<Ctl>, u32) {
        let stream = e.mem.alloc(0x10);
        let records = e.mem.alloc(0x100);
        e.mem.set_u32(stream + 4, records);
        e.mem.set_u32(stream + 0xc, records + 0x100);
        returns(e, TLS_GET_VALUE, stream);
        e.register(SET_ROTATED_DIR, copy_vector);
        e.register(COPY_POINT, |e, a| {
            copy_words(e, a[0], a[1], 4);
            ret(a[0])
        });
        e.register(0x0900_0020, nothing);
        let state_object = with_vtable(e, &[(0x20, 0x0900_0020)]);
        returns(e, STATE_LOOKUP, state_object);
        e.register(0x0900_0094, |_, _| ret(1));
        let c = controller(e);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + 0x94, 0x0900_0094);
        e.mem.set_u32(c.addr(), vtable);
        (c, records)
    }

    #[test]
    fn move_in_state_four_sets_the_position() {
        let mut e = engine();
        let (c, records) = move_world(&mut e);
        returns(&mut e, GET_STATE, 4);
        e.register(SET_POSITION, |e, a| {
            copy_words(e, SCRATCH, a[1], 3);
            Ret::default()
        });
        e.set_global(INVERSE_SCALE, 1.0f32);
        let params = e.mem.alloc(0x30);
        e.mem.set_f32(params, 0.5);
        store4(&mut e, params + 0x10, [1.0, 2.0, 3.0, 0.0]);
        e.set(c, Ctl::m_stateManager, 0x3ec0);
        e.set(c, Ctl::m_currentState, 4);
        assert_eq!(
            bhk_character_controller_move(&mut e, c, Ptr::new(params)),
            0
        );
        assert_eq!([0, 4, 8].map(|o| e.mem.f32(SCRATCH + o)), [1.0, 2.0, 3.0]);
        assert_eq!(calls(&e, STATE_LOOKUP), vec![vec![0x3ec0, 4]]);
        assert_eq!(calls(&e, 0x0900_0020)[0][1], c.addr());
        assert_eq!(load4(&e, c.addr() + 0x4f0), [2.0, 4.0, 6.0, 0.0]);
        assert_eq!(e.mem.u32(records), PROFILE_UPDATE_STATE);
        assert_eq!(e.mem.u32(records + 0xc), PROFILE_END);
    }

    #[test]
    fn move_with_a_tiny_step_only_clears() {
        let mut e = engine();
        let (c, _) = move_world(&mut e);
        returns(&mut e, GET_STATE, 0);
        stub(&mut e, &[REMOVE_REFERENCE]);
        e.set_global(SPEED_DIVISOR, 2.0f32);
        e.set_global(MIN_STEP, f64::from_bits(0x3f1a_36e2_e000_0000));
        let params = e.mem.alloc(0x30);
        e.mem.set_f32(params, 0.00001);
        e.mem.set_f32(params + 0x20, 6.0);
        e.set(c, Ctl::spBumpedBody, 0x6300);
        e.set(c, Ctl::fBumpedForce, 9.0);
        e.set(c, Ctl::spBumpedCharProxy, 0x6400);
        e.set(c, Ctl::iFlags, 0xc1);
        assert_eq!(
            bhk_character_controller_move(&mut e, c, Ptr::new(params)),
            0
        );
        assert_eq!(e.get(c, Ctl::fSpeedPct), 3.0);
        assert_eq!(e.get(c, Ctl::fBumpedForce), 0.0);
        assert_eq!(e.get(c, Ctl::spBumpedBody), 0);
        assert_eq!(e.get(c, Ctl::spBumpedCharProxy), 0);
        assert_eq!(e.get(c, Ctl::iFlags), 1);
        assert_eq!(
            calls(&e, REMOVE_REFERENCE),
            vec![vec![0x6300], vec![0x6400]]
        );
    }

    #[test]
    fn move_on_unchanged_ground_keeps_still() {
        let mut e = engine();
        let (c, records) = move_world(&mut e);
        returns(&mut e, GET_STATE, 0);
        e.set_global(SPEED_DIVISOR, 1.0f32);
        e.set_global(MIN_STEP, f64::from_bits(0x3f1a_36e2_e000_0000));
        e.set_global(LANE_TOLERANCE, 0.001f32);
        e.set_global(FLOAT_MINUS_ONE, -1.0f32);
        e.set_global(PITCH_TIMER_RESET, 5.0f32);
        for lane in 0..4 {
            e.set_global(ABS_MASK + 4 * lane, 0x7fff_ffffu32);
            e.set_global(SIGN_MASK + 4 * lane, 0x8000_0000u32);
            e.set_global(ROTATION_BIAS + 4 * lane, -0.5f32);
        }
        returns(&mut e, POINTS_NEAR, 1);
        returns(&mut e, STATE_FIVE_TEST, 0);
        returns(&mut e, POINT_COMPARE, 1);
        e.register(SET_AXIS_ANGLE, |e, a| {
            store4(e, a[0], [0.0, 0.0, 0.0, 1.0]);
            Ret::default()
        });
        e.register(POINT_TO_VECTOR, |e, a| {
            copy_words(e, a[0], a[1], 3);
            e.mem.set_u32(a[0] + 12, 0);
            ret(a[0])
        });
        let velocity = vec_at(&mut e, [1.0, 0.0, 5.0, 0.0]);
        returns(&mut e, GET_HK_LINEAR_VELOCITY, velocity);
        stub(&mut e, &[UN_MARK_WRITE, HKP_SET_LINEAR_VELOCITY]);
        e.register(HKP_CHECK_SUPPORT, |e, a| {
            e.mem.set_u32(a[2], 2);
            Ret::default()
        });
        let part = e.mem.alloc(0x10);
        returns(&mut e, PROXY_PART_74, part);
        let proxy = e.mem.alloc(0xc0);
        e.set(c, Ctl::phkObject, proxy);
        e.set(c, Ctl::fCalculatePitchTimer, 1.0);
        e.set(c, Ctl::fCenter, 2.0);
        store4(&mut e, c.addr() + 0x4d0, [0.0, -1.0, 0.0, 0.0]);
        let params = e.mem.alloc(0x30);
        e.mem.set_f32(params, 0.5);
        e.mem.set_f32(params + 0xc, 0.25);
        store4(&mut e, params + 0x10, [1.0, 2.0, 3.0, 0.0]);
        e.mem.set_f32(params + 0x20, 1.0);
        assert_eq!(
            bhk_character_controller_move(&mut e, c, Ptr::new(params)),
            0
        );
        let flags = e.get(c, Ctl::iFlags);
        assert_eq!(flags & 0x100, 0x100);
        assert_eq!(flags & (8 | 0x1000 | 0x1_0000), 0);
        assert_eq!(e.get(c, Ctl::m_supportedState), 2);
        assert_eq!(load4(&e, c.addr() + 0x4f0), [0.0; 4]);
        let set = vec![proxy, c.addr() + 0x4f0];
        assert_eq!(calls(&e, HKP_SET_LINEAR_VELOCITY), vec![set.clone(), set]);
        assert_eq!(e.get(c, Ctl::eWantState), 0xb);
        assert_eq!(e.get(c, Ctl::fCalculatePitchTimer), 0.5);
        assert_eq!(e.get(c, Ctl::StepDeltaTime), 0.5);
        assert_eq!(e.get(c, Ctl::StepInvDeltaTime), 2.0);
        assert_eq!(load4(&e, c.addr() + 0x510), [2.0, 4.0, 0.0, 0.0]);
        assert_eq!(e.mem.f32(c.addr() + 0x578), 2.0);
        assert_eq!(e.mem.f32(params + 0xc), 0.25);
        assert_eq!(
            calls(&e, SET_AXIS_ANGLE)[0][1..],
            [c.addr() + 0x4c0, (-0.25f32).to_bits()]
        );
        assert_eq!(load4(&e, c.addr() + 0x4d0), [0.0, -1.0, 0.0, 0.0]);
        let labels = [0, 0xc, 0x18, 0x24].map(|o| e.mem.u32(records + o));
        assert_eq!(
            labels,
            [
                PROFILE_MOVEMENT,
                PROFILE_UPDATE_STATE,
                PROFILE_END,
                PROFILE_END
            ]
        );
        assert!(calls(&e, POINT_COLLECTOR_CONSTRUCT).is_empty());
    }

    #[test]
    fn init_builds_the_phantom_and_joins_the_world() {
        let mut e = engine();
        shapes_world(&mut e);
        e.set_global(SCALE, 1.0f32);
        install_allocator(&mut e, 0);
        e.register(PHANTOM_CINFO_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0] + 0x14, 0x8000_0000);
            Ret::default()
        });
        stub(
            &mut e,
            &[
                REF_OBJECT_CONSTRUCT,
                SET_DEBUG_OBJECT_COLOR,
                CHARACTER_PROXY_INIT,
                UN_MARK_WRITE,
                ADD_PROXY_LISTENER,
            ],
        );
        e.register(CACHING_PHANTOM_INIT, |e, a| {
            e.mem.set_u32(a[0] + 8, 0x4800);
            Ret::default()
        });
        returns(&mut e, RGB_FROM_FLOATS, 0xc0102);
        e.register(0x0900_009c, nothing);
        e.set_global(VTABLE_CACHING_SHAPE_PHANTOM + 0x9c, 0x0900_009cu32);
        let c = controller_with_slots(&mut e, &[0xd0]);
        let hk = e.mem.alloc(0x20);
        e.mem.set_f32(hk + 0x10, 2.0);
        let shape = e.mem.alloc(0x10);
        e.mem.set_u32(shape + 8, hk);
        let world = e.mem.alloc(0x20);
        e.mem.set_u8(world + 0x16, 1);
        let proxy = e.mem.alloc(0xc0);
        e.set(c, Ctl::phkObject, proxy);
        let info = e.mem.alloc(0xc0);
        e.mem.set_f32(info + 0xac, 2.0);
        e.mem.set_f32(info + 0xa8, 3.0);
        e.mem.set_u32(info + 0xa0, shape);
        e.mem.set_u32(info + 0x84, 0x1e);
        e.mem.set_u32(info + 0x88, world);
        e.mem.set_u32(info + 0x9c, 5);
        e.mem.set_f32(info + 0x8c, 1.5);
        e.mem.set_f32(info + 0x90, -9.8);
        e.mem.set_u32(info + 0xb4, 7);
        e.mem.set_u8(info + 0x97, 1);
        e.set(c, Ctl::iFlags, 0x0400_0000);
        let word = e.tls() + SCOPE_WORD;
        e.mem.set_u32(word, 6);
        bhk_character_controller_init(&mut e, c, info);
        assert_eq!(e.get(c, Ctl::fStepHeight), 6.0);
        let phantom = e.get(c, Ctl::spShapePhantom);
        assert_eq!(e.mem.u32(phantom), VTABLE_CACHING_SHAPE_PHANTOM);
        assert_eq!(calls(&e, CACHING_PHANTOM_INIT)[0][0], phantom);
        assert_eq!(calls(&e, 0x0900_009c), vec![vec![phantom, world]]);
        assert_eq!(
            calls(&e, SET_DEBUG_OBJECT_COLOR),
            vec![vec![world, phantom, 0xc0102]]
        );
        assert_eq!(calls(&e, 0x0900_00d0), vec![vec![c.addr(), 0]]);
        assert_eq!(e.get(c, Ctl::eWantState), 5);
        assert_eq!(e.get(c, Ctl::fAcrobatics), 1.5);
        assert_eq!(e.get(c, Ctl::fGravity), -9.8);
        assert_eq!(e.get(c, Ctl::iPriority), 7);
        assert_eq!(e.mem.u32(info + 0x48), 0x4800);
        assert_eq!(calls(&e, CHARACTER_PROXY_INIT), vec![vec![c.addr(), info]]);
        assert_eq!(e.mem.u8(proxy + 0xb0), 1);
        assert_eq!(e.get(c, Ctl::iFlags), 0x0200_0000);
        assert_eq!(
            calls(&e, ADD_PROXY_LISTENER),
            vec![vec![proxy, c.addr() + 0x410]]
        );
        let increments = calls(&e, INTERLOCKED_INCREMENT);
        assert_eq!(increments, vec![vec![shape + 4], vec![phantom + 4]]);
        assert_eq!(e.mem.u32(word), 6);
    }

    /// A contact for `IsStep`: a phantom collidable (type 2) and a body collidable
    /// (type 1) whose shape casts through `0x0900_0020` (from and to logged at
    /// `SCRATCH`, the fraction taken from `SCRATCH + 0x40`, shape key 7).
    fn step_scene(e: &mut Engine) -> (Ptr<Ctl>, u32, u32) {
        e.set_global(FLOAT_THREE, 3.0f32);
        e.set_global(FLOAT_HALF, 0.5f32);
        e.set_global(DOUBLE_THREE, 3.0f64);
        e.register(SET_ROTATED_DIR, copy_vector);
        e.register(SET_TRANSFORMED_INVERSE_POS, copy_vector);
        e.register(0x0900_0020, |e, a| {
            copy_words(e, SCRATCH, a[2], 8);
            let fraction = e.mem.f32(SCRATCH + 0x40);
            e.mem.set_f32(a[3] + 0x10, fraction);
            e.mem.set_u32(a[3] + 0x20, 7);
            e.mem.set_u32(a[3] + 0x24, u32::MAX);
            Ret::default()
        });
        let phantom_collidable = e.mem.alloc(0x20);
        e.mem.set_u8(phantom_collidable + 0x18, 2);
        let phantom_shape = e.mem.alloc(0x20);
        e.mem.set_u32(phantom_collidable, phantom_shape);
        let body = e.mem.alloc(0x200);
        e.mem.set_u8(body + 0x18, 1);
        let shape = with_vtable(e, &[(0x20, 0x0900_0020)]);
        e.mem.set_u32(body, shape);
        let point = e.mem.alloc(0x50);
        store4(e, point, [1.0, 0.0, 0.35, 0.0]);
        e.mem.set_u32(point + 0x20, phantom_collidable);
        e.mem.set_u32(point + 0x48, body);
        e.mem.set_u32(point + 0x4c, u32::MAX);
        let c = controller(e);
        e.set(c, Ctl::fRadius, 0.2);
        e.set(c, Ctl::fStepHeight, 0.3);
        e.set(c, Ctl::fCastDepth, 0.5);
        (c, point, shape)
    }

    #[test]
    fn step_is_a_missed_ray_down_the_contact() {
        let mut e = engine();
        let (c, point, shape) = step_scene(&mut e);
        e.mem.set_f32(SCRATCH + 0x40, 1.0);
        assert!(bhk_character_controller_is_step(&mut e, c, Ptr::new(point)));
        let inverse = inverse_length(1.0, 0.0, 0.5, 3.0);
        let reach = (0.5f32 as f64 + 0.2f32 as f64) as f32;
        let x = (0.0f64 - (-inverse) as f64 * reach as f64) as f32;
        assert_eq!(load4(&e, SCRATCH), [0.0, 0.0, 0.3, 0.0]);
        assert_eq!(load4(&e, SCRATCH + 0x10), [x, 0.0, 0.3, 0.0]);
        assert_eq!(calls(&e, 0x0900_0020)[0][0], shape);
        assert!(calls(&e, RECORD_SUPPORT_HEIGHT).is_empty());
        // A hit on a layer `00c6d7e0` lets through is no step.
        e.mem.set_f32(SCRATCH + 0x40, 0.5);
        assert!(!bhk_character_controller_is_step(
            &mut e,
            c,
            Ptr::new(point)
        ));
        // Too high a rise is refused before any cast.
        e.call_log = Some(vec![]);
        e.mem.set_f32(point + 8, 1.0);
        assert!(!bhk_character_controller_is_step(
            &mut e,
            c,
            Ptr::new(point)
        ));
        assert!(calls(&e, 0x0900_0020).is_empty());
    }

    #[test]
    fn step_on_a_blocked_layer_records_the_height() {
        let mut e = engine();
        let (c, point, shape) = step_scene(&mut e);
        stub(&mut e, &[RECORD_SUPPORT_HEIGHT]);
        let hk = e.mem.alloc(0x20);
        e.mem.set_u32(hk + 0x10, 3);
        e.mem.set_u32(shape + 0xc, 0x17);
        e.mem.set_u32(shape + 8, hk);
        e.register(0x0900_0bc8, |_, _| ret(1));
        e.set_global(LAYER_CALLBACK, 0x0900_0bc8u32);
        e.set(c, Ctl::iFlags, 1);
        assert!(bhk_character_controller_is_step(&mut e, c, Ptr::new(point)));
        assert_eq!(calls(&e, 0x0900_0bc8), vec![vec![3]]);
        assert_eq!(
            calls(&e, RECORD_SUPPORT_HEIGHT),
            vec![vec![c.addr() + 0x410, 0, 0.35f32.to_bits()]]
        );
        assert!(calls(&e, 0x0900_0020).is_empty());
    }

    #[test]
    fn constraints_mark_flat_support_and_stop_on_layers() {
        let mut e = engine();
        e.set_global(SUPPORT_UNSET, 1.0f32);
        e.set_global(PLANE_SPEED_FLAT, 5.77f32);
        e.set_global(UPRIGHT_NORMAL, 0.985f64);
        returns(&mut e, GET_STATE, 1);
        e.register(X87_SQRT, sqrt_double);
        let c = controller(&mut e);
        e.set(c, Ctl::fMaxSlope, 0.5);
        e.set(c, Ctl::eSupportMaterial, 0x24);
        let word = e.tls() + SCOPE_WORD;
        e.mem.set_u32(word, 2);
        let body = e.mem.alloc(0x200);
        e.mem.set_u8(body + 0x18, 1);
        e.mem.set_u32(body + 0x1c, 0x83);
        let other = e.mem.alloc(0x200);
        e.mem.set_u8(other + 0x18, 2);
        e.mem.set_u32(other + 0x1c, 0x8c);
        let points = e.mem.alloc(0xe0);
        e.mem.set_f32(points + 0x18, 1.0);
        e.mem.set_u32(points + 0x48, body);
        e.mem.set_u32(points + 0x4c, u32::MAX);
        e.mem.set_f32(points + 0x70 + 0x18, 1.0);
        e.mem.set_u32(points + 0x70 + 0x48, other);
        let manifold = e.mem.alloc(8);
        e.mem.set_u32(manifold, points);
        e.mem.set_u32(manifold + 4, 2);
        let planes = e.mem.alloc(0x40 * 6);
        store4(&mut e, planes, [0.0, 0.0, 2.0, 0.0]);
        store4(&mut e, planes + 0x10, [1.0; 4]);
        store4(&mut e, planes + 0x50, [3.0; 4]);
        let input = e.mem.alloc(0x50);
        e.mem.set_u32(input + 0x48, planes);
        e.mem.set_u32(input + 0x4c, 2);
        let listener = Ptr::new(c.addr() + 0x410);
        fn_00c711d0(&mut e, listener, 0, manifold, input);
        assert_eq!(load4(&e, c.addr() + 0x460), [1.0; 4]);
        let flags = e.get(c, Ctl::iFlags);
        assert_eq!(flags & 0x600, 0x600);
        assert_eq!(flags & (4 | 0x2000_0000), 0);
        assert_eq!(e.get(c, Ctl::iSupportCount), 1);
        assert_eq!(load4(&e, c.addr() + 0x450), [0.0, 0.0, 1.0, 0.0]);
        assert_eq!(e.mem.f32(planes + 0x28), 5.77);
        assert_eq!(load4(&e, planes + 0x10), [1.0; 4]);
        assert_eq!(load4(&e, planes + 0x50), [0.0; 4]);
        assert_eq!(e.get(c, Ctl::bAllSurfacesStopped), 0);
        assert_eq!(e.get(c, Ctl::eSupportLayer), 3);
        assert_eq!(e.mem.u32(word), 2);
    }

    #[test]
    fn controller_cinfo_with_the_shared_states() {
        let mut e = engine();
        install_fresh_allocator(&mut e);
        e.set_global(INVERSE_SCALE, 2.0f32);
        e.set_global(THIRTY_ONE, 31.0f64);
        e.set_global(FLOAT_HUNDRED, 100.0f32);
        e.set_global(FILTER_COUNTER, 0xffffu32);
        returns(&mut e, STATE_MANAGER_CONSTRUCT, 0x7100);
        for construct in [
            STATE_CONSTRUCT_0,
            STATE_CONSTRUCT_5,
            STATE_CONSTRUCT_1,
            STATE_CONSTRUCT_2,
            STATE_CONSTRUCT_6,
        ] {
            e.register(construct, |_, a| ret(a[0]));
        }
        stub(&mut e, &[STATE_REGISTER, REMOVE_REFERENCE, ADD_REFERENCE]);
        let info = e.mem.alloc(0xc0);
        e.mem.set_u32(info + 0xa4, 0x7200);
        assert_eq!(fn_00c6da50(&mut e, Ptr::new(info), 1).addr(), info);
        assert_eq!(e.global::<u32>(FILTER_COUNTER), 10);
        assert_eq!(e.mem.u32(info + 0x84), 10 << 16 | 0x1e);
        assert_eq!(e.mem.f32(info + 0xa8), 62.0);
        assert_eq!(e.mem.f32(info + 0x58), 100.0);
        assert_eq!([0x90, 0xac, 0x20].map(|o| e.mem.f32(info + o)), [1.0; 3]);
        assert_eq!(e.mem.u32(info + 0xb0), 1);
        assert_eq!(e.global::<u32>(STATIC_REFERENCED_OBJECT), 0x7100);
        let registered = calls(&e, STATE_REGISTER);
        let slots: Vec<u32> = registered.iter().map(|c| c[2]).collect();
        assert_eq!(slots, [0, 5, 1, 2, 3, 4, 6]);
        assert!(registered.iter().all(|c| c[0] == 0x7100));
        let climbing = registered[4][1];
        assert_eq!(e.mem.u32(climbing), VTABLE_STATE_CLIMBING);
        assert_eq!([e.mem.u16(climbing + 4), e.mem.u16(climbing + 6)], [8, 1]);
        assert_eq!(e.mem.u32(registered[5][1]), VTABLE_STATE_FLYING);
        let released = calls(&e, REMOVE_REFERENCE);
        assert_eq!(released.len(), 7);
        assert_eq!(calls(&e, ADD_REFERENCE), vec![vec![0x7100]]);
        assert_eq!(e.mem.u32(info + 0xa4), 0x7100);
        // Without the states the cinfo keeps no reference.
        let other = e.mem.alloc(0xc0);
        fn_00c6da50(&mut e, Ptr::new(other), 0);
        assert_eq!(e.mem.u32(other + 0xa4), 0);
        assert_eq!(e.global::<u32>(FILTER_COUNTER), 11);
    }
}
