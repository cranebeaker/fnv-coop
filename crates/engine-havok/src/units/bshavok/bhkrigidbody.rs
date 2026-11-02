//! `BSHavok/bhkrigidbody.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkRigidBody` wraps a Havok `hkpRigidBody` (`phkObject`), keeps the construction info
//! (`bhkRigidBodyCinfo`, the Xbox `hkpRigidBodyCinfo` plus a 0x20-byte header on the PC) in
//! `pInfo`, and owns a list of the actions and constraints that act on the body. The
//! functions that allocate, construct or tear down Havok objects raise the per-thread
//! allocation word at `TLS + 0x2b4` to 0x10 for their duration and put the old value
//! back, as `bhkWorld` does.
//!
//! The exception-unwinding frames (`FS:[0]` registrations) and `/GS` stack-cookie checks of
//! the functions marked `Translation incomplete` are not modeled.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::BSSimpleList;

/// Per-thread word the Havok-allocating functions set to 0x10 (`TLS + 0x2b4`).
const SCOPE_WORD: u32 = 0x2b4;
/// `hkMemoryRouter` lookup (`00c85750`): its `+0x10` is the allocator whose slot `+4`
/// allocates and `+0x10` frees `(pointer, bytes)`.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The game heap's allocator (`00aa13e0`, cdecl).
const HEAP_ALLOCATE: u32 = 0x00aa_13e0;
/// The memory manager (`011f6238`) with its allocate (`00aa3e40`) and free (`00aa4060`).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_MANAGER_FREE: u32 = 0x00aa_4060;
/// The empty lock call (`00a29680`) the PC build kept around the Havok accesses.
const EMPTY_LOCK: u32 = 0x00a2_9680;
/// `kernel32` `InterlockedIncrement` / `InterlockedDecrement` import slots.
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `sprintf` (`00406d00`, `(buffer, size, format, ...)`).
const FORMAT_TEXT: u32 = 0x0040_6d00;

/// `bhkWorldObject` methods this unit's overrides call.
const WORLD_OBJECT_IS_EQUAL: u32 = 0x00c8_5800;
const WORLD_OBJECT_ADD: u32 = 0x00c8_5810;
const WORLD_OBJECT_COPY_CINFO: u32 = 0x00c8_5880;
const WORLD_OBJECT_COPY_MEMBERS: u32 = 0x00c8_5900;
const WORLD_OBJECT_LINK_OBJECT: u32 = 0x00c8_59e0;
const WORLD_OBJECT_REGISTER_STREAMABLES: u32 = 0x00c8_5ac0;
const WORLD_OBJECT_CREATE_DISPLAY_GEOMETRY: u32 = 0x00c8_5d70;
/// `(out, object, key)`: the property of `key`, returned through `out`.
const WORLD_OBJECT_GET_PROPERTY: u32 = 0x00c8_5ef0;
const PROPERTY_KEY: u32 = 0x0126_7b70;
/// `bhkWorldObject::GethkWorld` and `GethkWorldRw`.
const WORLD_OBJECT_GET_WORLD: u32 = 0x00c8_7a40;
const WORLD_OBJECT_GET_WORLD_RW: u32 = 0x00c8_7a50;
/// The world a `bhkConstraint` is in (`00c89a00`).
const CONSTRAINT_GET_WORLD: u32 = 0x00c8_9a00;
/// The action/constraint list: add at the head (`00c80a30`), remove the head
/// (`00c88990`, advances the list), clear (`004ee920`) and destroy (`004ee840`).
const ACTION_LIST_ADD_HEAD: u32 = 0x00c8_0a30;
const ACTION_LIST_REMOVE_HEAD: u32 = 0x00c8_8990;
const ACTION_LIST_CLEAR: u32 = 0x004e_e920;
const ACTION_LIST_DESTRUCT: u32 = 0x004e_e840;
/// `bhkEntity` methods.
const ENTITY_LOAD_BINARY: u32 = 0x00c9_d4f0;
const ENTITY_SAVE_BINARY: u32 = 0x00c9_d530;
const ENTITY_KILL: u32 = 0x00c9_d4c0;
const ENTITY_REMOVE: u32 = 0x00c9_d590;
const ENTITY_DESTRUCT: u32 = 0x00c9_d620;
const ENTITY_GET_VIEWER_STRINGS: u32 = 0x00c9_d6b0;
/// `bhkSerializable` methods: cinfo loader (`00c87710`), clone lock and unlock.
const SERIALIZABLE_LOAD_CINFO: u32 = 0x00c8_7710;
const CLONE_LOCK: u32 = 0x00c8_76f0;
const CLONE_UNLOCK: u32 = 0x00c8_7700;
/// `bhkRefObject` constructor (`00c87c30`).
const REF_OBJECT_CONSTRUCT: u32 = 0x00c8_7c30;
/// Class vtables written by the constructors and the matching object counters.
const BHK_SERIALIZABLE_VTABLE: u32 = 0x0103_0394;
const BHK_ENTITY_VTABLE: u32 = 0x0103_02bc;
const BHK_RIGID_BODY_VTABLE: u32 = 0x0103_01b4;
const SERIALIZABLE_COUNT: u32 = 0x0126_810c;
const ENTITY_COUNT: u32 = 0x0126_81fc;
const RIGID_BODY_COUNT: u32 = 0x0126_81bc;
/// The `NiRTTI` pointers of `bhkConstraint`, `bhkFixedConstraint` and `bhkAction`, and the
/// key of `bhkCollisionObject` (`012043f8`) with the cast to it (`00653290`, thiscall).
const CONSTRAINT_RTTI: u32 = 0x0126_818c;
const FIXED_CONSTRAINT_RTTI: u32 = 0x0126_8628;
const ACTION_RTTI: u32 = 0x0126_8158;
const COLLISION_OBJECT_KEY: u32 = 0x0120_43f8;
const CAST_TO_COLLISION_OBJECT: u32 = 0x0065_3290;
/// Prepares a scene-graph node before its bodies are fixed or unfixed (`00a68bf0`).
const NODE_PREPARE: u32 = 0x00a6_8bf0;

/// `hkpRigidBody` methods: motion getter, `getCinfo`, setters, constructor, motion type.
const RIGID_BODY_GET_MOTION: u32 = 0x00ca_86c0;
const RIGID_BODY_GET_CINFO: u32 = 0x00ca_8f50;
const RIGID_BODY_SET_POSITION: u32 = 0x00ca_8800;
const RIGID_BODY_SET_ROTATION: u32 = 0x00ca_8830;
const RIGID_BODY_SET_POSITION_AND_ROTATION: u32 = 0x00ca_8860;
const RIGID_BODY_SET_MOTION_TYPE: u32 = 0x00ca_8420;
const RIGID_BODY_CONSTRUCT: u32 = 0x00ca_8c90;
/// `hkUFloat8::operator=` (`00ca9360`, byte buffer in ECX, `&float`).
const HK_UFLOAT8_ASSIGN: u32 = 0x00ca_9360;
/// `hkpMotion` mass getter (`00c86430`).
const MOTION_GET_MASS: u32 = 0x00c8_6430;
/// `hkReferencedObject::removeReference` (`00c905b0`).
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// The Havok half of the cinfo constructor (`00cf5ea0`).
const RIGID_BODY_CINFO_HK_CONSTRUCT: u32 = 0x00cf_5ea0;
/// Maximum linear velocity of `bhkRigidBody` (`00577b50`), dynamic mass (`0062ab20`) and
/// the layer fix (`00626cb0`, `(this, new filter word)`).
const GET_MAX_LINEAR_VELOCITY: u32 = 0x0057_7b50;
const GET_DYNAMIC_MASS: u32 = 0x0062_ab20;
const FIX_LAYER: u32 = 0x0062_6cb0;
/// `bhkFixedConstraint::FixConstraint` (`00cc81c0`, cdecl `(constraint, strength)`).
const FIX_CONSTRAINT: u32 = 0x00cc_81c0;
/// Velocity setters of the Havok body (`(body, vector)`) and of the object (`(this, vector)`).
const RIGID_BODY_SET_LINEAR_VELOCITY: u32 = 0x0056_16d0;
const RIGID_BODY_SET_ANGULAR_VELOCITY: u32 = 0x0056_1800;
const OBJECT_SET_LINEAR_VELOCITY: u32 = 0x0056_1690;
const OBJECT_SET_ANGULAR_VELOCITY: u32 = 0x0056_17c0;
/// Keyframe helpers: teleport notification (`005610f0`), the guard's preparation
/// (`0040fbf0`) and the position conversion (`004a3e00`, cdecl `(out, position)`).
const KEYFRAME_TELEPORT_NOTIFY: u32 = 0x0056_10f0;
const GUARD_PREPARE: u32 = 0x0040_fbf0;
const KEYFRAME_POSITION_CONVERT: u32 = 0x004a_3e00;
/// Quaternion helpers: product, normalise, angle (result in `st0`), axis, difference.
const QUATERNION_SET_MUL: u32 = 0x00c6_6320;
const QUATERNION_NORMALIZE: u32 = 0x0056_11c0;
const QUATERNION_GET_ANGLE: u32 = 0x00c7_4c20;
const QUATERNION_GET_AXIS: u32 = 0x00c7_4c90;
const QUATERNION_DIFFERENCE: u32 = 0x00c7_4ac0;
/// `hkVector4::setTransformedPos` (`00c7f640`, `(out, transform, vector)`).
const TRANSFORM_POINT: u32 = 0x00c7_f640;
/// Copies a vector into a `NiPoint3` (`00458620`, cdecl `(destination, source)`).
const NI_POINT3_FROM_VECTOR: u32 = 0x0045_8620;
/// `BSTestObjects::CreateCoordinateJack` (`00c59200`, cdecl, one float) and its size.
const CREATE_COORDINATE_JACK: u32 = 0x00c5_9200;
const COORDINATE_JACK_SIZE: u32 = 0x0105_0c48;

/// `NiStream` helpers: `NiStream::ReadMultipleLinkIDs` (`00a64670`),
/// `GetNumberOfLinkIDs` (`00a63460`) and `GetObjectFromLinkID` (`00a63430`).
const STREAM_READ_MULTIPLE_LINK_IDS: u32 = 0x00a6_4670;
const STREAM_GET_NUMBER_OF_LINK_IDS: u32 = 0x00a6_3460;
const STREAM_GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
/// Viewer-string helpers: `NiGetViewerString` (`00a7d330`), the string (`00a7d5e0`),
/// float (`00a7d550`) and int (`00a7d510`) overloads, `NiPoint3::GetViewerString`
/// (`00a59af0`) and `NiQuaternion::GetViewerString` (`00a6e190`).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const NI_GET_VIEWER_STRING_TEXT: u32 = 0x00a7_d5e0;
const NI_GET_VIEWER_STRING_FLOAT: u32 = 0x00a7_d550;
const NI_GET_VIEWER_STRING_INT: u32 = 0x00a7_d510;
const NI_POINT3_GET_VIEWER_STRING: u32 = 0x00a5_9af0;
const NI_QUATERNION_GET_VIEWER_STRING: u32 = 0x00a6_e190;
/// The viewer-string array's add (`0096a610`), grow (`0096ad30`) and set-at (`0096ae90`).
const VIEWER_STRINGS_ADD: u32 = 0x0096_a610;
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
/// Class-name pointer whose viewer string follows the entity strings.
const RIGID_BODY_CLASS_NAME: u32 = 0x0126_81c0;

/// The Havok error/reporting object (`01268360` holds a pointer to it): slot `0x10`
/// enables or disables one message id, slot `0xC` reports `(0, id, text, file, line)`.
const HAVOK_ERROR_OBJECT: u32 = 0x0126_8360;
const HAVOK_ERROR_SLOT_SET_ENABLED: u32 = 0x10;
const HAVOK_ERROR_SLOT_MESSAGE: u32 = 0xc;
/// Message text format and source file name of the old-cinfo report.
const OLD_CINFO_MESSAGE: u32 = 0x010c_6118;
const HAVOK_SOURCE_FILE: u32 = 0x010c_60dc;
/// `Init` and `Add` state: the shape query context, the property key's assert byte.
const SHAPE_QUERY_CONTEXT: u32 = 0x0126_8370;
const ASSERT_STATE_BYTE: u32 = 0x011a_fe59;

/// Tables and defaults: the 256-float quantised table (`010c7948`), the default vector
/// and rotation, one and half vectors, sign and absolute-value masks.
const QUANTISED_FLOAT_TABLE: u32 = 0x010c_7948;
const DEFAULT_VECTOR: u32 = 0x0126_7e30;
const DEFAULT_ROTATION: u32 = 0x0126_83b0;
const ONES_VECTOR: u32 = 0x010e_c7f0;
const HALF_VECTOR: u32 = 0x010c_71e0;
const SIGN_MASK: u32 = 0x010c_4c00;
const ABSOLUTE_MASK: u32 = 0x0101_7d10;
/// Fill values of an empty bounding box: -0.001, 0.001 and 0.0.
const AABB_LOW_FILL: u32 = 0x0104_ffb8;
const AABB_HIGH_FILL: u32 = 0x0101_7d00;
const AABB_W_FILL: u32 = 0x0101_1d78;
/// Clamps of `Init`: allowed penetration depth floor (f32) and ceiling (f64) with its
/// default, maximum linear velocity floor (f64) and default, restitution ceiling (f64)
/// and default.
const PENETRATION_DEPTH_FLOOR: u32 = 0x0104_da4c;
const PENETRATION_DEPTH_CEILING: u32 = 0x0101_8a90;
const DEFAULT_PENETRATION_DEPTH: u32 = 0x0103_0ff4;
const MAX_LINEAR_VELOCITY_FLOOR: u32 = 0x0101_7b70;
const DEFAULT_MAX_LINEAR_VELOCITY: u32 = 0x0101_3974;
const RESTITUTION_CEILING: u32 = 0x0102_e958;
const DEFAULT_RESTITUTION: u32 = 0x010c_608c;
/// Quality decision of `Init`: extent limit, velocity scale and velocity limit.
const QUALITY_EXTENT_LIMIT: u32 = 0x0126_77bc;
const QUALITY_VELOCITY_SCALE: u32 = 0x011b_0c48;
const QUALITY_VELOCITY_LIMIT: u32 = 0x0126_77c0;
/// Keyframe thresholds: angle (f64), position scale, squared distance, rotation tolerance,
/// velocity tolerance, step time and speed limit (f64).
const ANGLE_THRESHOLD: u32 = 0x0101_6978;
const POSITION_SCALE: u32 = 0x0126_81c8;
const NEAR_DISTANCE_SQUARED: u32 = 0x010c_6090;
const ROTATION_TOLERANCE: u32 = 0x0109_59f4;
const VELOCITY_TOLERANCE: u32 = 0x0101_7d00;
const STEP_TIME: u32 = 0x011a_fe64;
const SPEED_LIMIT: u32 = 0x0106_b798;
/// The strength `Fix` gives the fixed constraints of some collision groups (0.3).
const FIXED_CONSTRAINT_STRENGTH: u32 = 0x0101_7870;
/// Default maximum linear velocity (10000.0) and maximum angular velocity (10 pi) of a new
/// cinfo.
const DEFAULT_CINFO_MAX_LINEAR_VELOCITY: u32 = 0x0102_2958;
const DEFAULT_CINFO_MAX_ANGULAR_VELOCITY: u32 = 0x010c_60d8;

/// Viewer-string names and formats of `GetViewerStrings`.
const POSITION_NAME: u32 = 0x010c_6220;
const ROTATION_NAME: u32 = 0x0104_49cc;
const CENTER_OF_MASS_NAME: u32 = 0x010c_621c;
const MASS_NAME: u32 = 0x010c_6214;
const LINEAR_DAMPING_NAME: u32 = 0x010c_620c;
const ANGULAR_DAMPING_NAME: u32 = 0x010c_6204;
const FRICTION_NAME: u32 = 0x010c_61f8;
const RESTITUTION_NAME: u32 = 0x010c_61f0;
const MAX_LINEAR_VELOCITY_NAME: u32 = 0x010c_61e4;
const MAX_ANGULAR_VELOCITY_NAME: u32 = 0x010c_61d8;
const LINEAR_VELOCITY_NAME: u32 = 0x010c_61d0;
const ANGULAR_VELOCITY_NAME: u32 = 0x010c_61c8;
const INERTIA_TENSOR_NAME: u32 = 0x010c_61b8;
const PENETRATION_DEPTH_NAME: u32 = 0x010c_61ac;
const QUALITY_TYPE_NAME: u32 = 0x010c_61a0;
const ACTION_COUNT_NAME: u32 = 0x010c_6194;
const ACTION_ITEM_NAME: u32 = 0x010c_618c;
const ITEM_FORMAT: u32 = 0x010c_59e4;

/// Virtual slots (byte offsets) used on `bhkRigidBody` and on the list items.
const VSLOT_GET_RTTI: u32 = 0x08;
const VSLOT_IS_NODE: u32 = 0x0c;
const VSLOT_CREATE_CLONE: u32 = 0x48;
const VSLOT_SET_HK_OBJECT: u32 = 0x8c;
const VSLOT_ADD: u32 = 0x9c;
const VSLOT_REMOVE: u32 = 0xa0;
const VSLOT_QUERY_CINFO_SIZE: u32 = 0xa8;
const VSLOT_CREATE_CINFO: u32 = 0xb4;
const VSLOT_POST_INIT: u32 = 0xbc;
const VSLOT_UPDATE_COLLISION_FILTER: u32 = 0xc4;
const VSLOT_GET_POSITION_OV2: u32 = 0xd4;
const VSLOT_GET_ROTATION_OV2: u32 = 0xd8;
const VSLOT_SET_POS_ROT: u32 = 0xe8;
const VSLOT_GET_CENTER_OF_MASS_LOCAL: u32 = 0xec;
const VSLOT_GET_CENTER_OF_MASS_IN_WORLD: u32 = 0xf0;
const VSLOT_GET_TRANSFORM: u32 = 0xf4;
const VSLOT_SET_HAVOK_ASSERT_STATE: u32 = 0xfc;

layout! {
    /// `bhkRigidBody` (Xbox PDB), 0x1C bytes; the fields this unit uses.
    pub struct BhkRigidBody: 0x1c {
        /// `phkObject` (Xbox PDB): the `hkpRigidBody`.
        0x08 phkObject: Ptr,
        /// `pInfo` (Xbox PDB): the `bhkRigidBodyCinfo`.
        0x0c pInfo: Ptr,
        /// `iFlags` (Xbox PDB): bit 2 constraints fixed, bit 6 never added to a world,
        /// bit 9 motion type forced to dynamic when copied.
        0x10 iFlags: u32,
        /// `ActionConstraint` (Xbox PDB): the list of actions and constraints.
        0x14 ActionConstraint: Inline<BSSimpleList>,
    }

    /// `bhkRigidBodyCinfo` (Xbox PDB), 0xE0 bytes: the `bhkSerializable` cinfo header
    /// followed (at `+0x20`) by the PC `hkpRigidBodyCinfo`.
    pub struct BhkRigidBodyCinfo: 0xe0 {
        /// `m_collisionFilterInfo` (header copy).
        0x00 m_collisionFilterInfo: u32,
        /// `m_shape` (header copy).
        0x04 m_shape: Ptr,
        /// `m_centerOfMass` (Xbox PDB), a four-float vector.
        0xa0 m_centerOfMass: [f32; 4],
        /// `m_mass` (Xbox PDB).
        0xb0 m_mass: f32,
        /// `m_restitution` (Xbox PDB).
        0xc4 m_restitution: f32,
        /// `m_maxLinearVelocity` (Xbox PDB).
        0xc8 m_maxLinearVelocity: f32,
        /// `m_maxAngularVelocity` (Xbox PDB).
        0xcc m_maxAngularVelocity: f32,
        /// `m_allowedPenetrationDepth` (Xbox PDB).
        0xd0 m_allowedPenetrationDepth: f32,
        /// `m_qualityType` (Xbox PDB): -1 unset, otherwise the `hkpCollidableQualityType`.
        0xd7 m_qualityType: i8,
    }
}

/// The Havok allocator of the current memory router (`router + 0x10`).
fn memory_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// Appends the viewer string `text` to the array `strings` (size and capacity are
/// 16-bit fields at `+0xA` and `+8`; `+0xE` is the grow step): grows it when full,
/// then sets the element at the old size.
fn push_viewer_string(e: &mut Engine, strings: u32, text: u32) {
    let size = u32::from(e.mem.u16(strings + 0xa));
    if size >= u32::from(e.mem.u16(strings + 8)) {
        let step = u32::from(e.mem.u16(strings + 0xe));
        e.call(STRINGS_GROW, &args![strings, step + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRINGS_SET_AT, &args![strings, size, slot]);
    });
}

// Translated from 00c8ccc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRigidBody::IsEqual` (Xbox PDB): the `bhkWorldObject` comparison, then the `iFlags`
/// words must match. Returns the base result (false) when it fails.
pub fn bhk_rigid_body_is_equal(
    e: &mut Engine,
    this: Ptr<BhkRigidBody>,
    other: Ptr<BhkRigidBody>,
) -> bool {
    if !e.call(WORLD_OBJECT_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    e.get(this, BhkRigidBody::iFlags) == e.get(other, BhkRigidBody::iFlags)
}

// Translated from 00c8cce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl): true when the motion-type value `*value` is 1, 2, 3 or 6 (positive
/// and either below 4 or exactly 6). `bhkRigidBody::SetMotionType` and others test it.
pub fn fn_00c8cce0(e: &mut Engine, value: Ptr) -> bool {
    let motion = e.mem.i32(value.addr());
    motion > 0 && (motion < 4 || motion == 6)
}

// Translated from 00c8cd20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRigidBody::GetMaxAngularVelocity` (Xbox PDB): looks the quantised maximum angular
/// velocity up in the 256-float table. With a motion object it is the byte at
/// `motion + 0xBD`; without one the table value for the body's byte at `+0x19D` is
/// rounded through `hkUFloat8::operator=` and looked up again. 0.0 without a body.
pub fn bhk_rigid_body_get_max_angular_velocity(e: &mut Engine, this: Ptr<BhkRigidBody>) -> f32 {
    if this.is_null() {
        return 0.0;
    }
    let body = e.get(this, BhkRigidBody::phkObject);
    if body.is_null() {
        return 0.0;
    }
    let motion = e.call(RIGID_BODY_GET_MOTION, &args![body]).u32();
    let index = if motion != 0 {
        e.mem.u8(motion + 0xbd)
    } else {
        let quantised = e.mem.u8(body.addr() + 0x19d);
        let bits = x87_quiet_bits(e.mem.u32(QUANTISED_FLOAT_TABLE + 4 * quantised as u32));
        e.with_stack(8, |e, scratch| {
            let value = scratch.addr();
            let byte = value + 4;
            e.mem.set_u32(value, bits);
            e.call(HK_UFLOAT8_ASSIGN, &args![byte, value]);
            e.mem.u8(byte)
        })
    };
    f32::from_bits(x87_quiet_bits(
        e.mem.u32(QUANTISED_FLOAT_TABLE + 4 * index as u32),
    ))
}

/// Byte-swaps the 32-bit word at `address` (the Xbox 360 stores big-endian).
fn swap_word(e: &mut Engine, address: u32) {
    let word = e.mem.u32(address);
    e.mem.set_u32(address, word.swap_bytes());
}

// Translated from 00c8cdb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl): byte-swaps the twelve words of the three vectors at `matrix`
/// (the inertia tensor of a cinfo).
pub fn fn_00c8cdb0(e: &mut Engine, matrix: Ptr) {
    for word in 0..12 {
        swap_word(e, matrix.addr() + 4 * word);
    }
}

// Translated from 00c8cdf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRigidBodyCinfo::Scale` (Xbox PDB): scales the centre of mass (single-precision
/// SSE), the nine inertia-tensor floats that sit in the first three lanes of each
/// column by `scale` cubed (x87, `(scale * scale) * scale` rounded to `f32` first) and
/// the mass by `scale`.
pub fn bhk_rigid_body_cinfo_scale(e: &mut Engine, this: Ptr<BhkRigidBodyCinfo>, scale: f32) {
    let base = this.addr();
    for lane in 0..4 {
        let at = base + BhkRigidBodyCinfo::m_centerOfMass.off + 4 * lane;
        let value = e.mem.f32(at);
        e.mem.set_f32(at, scale * value);
    }
    let cube = ((scale as f64 * scale as f64) * scale as f64) as f32;
    for column in [0x70u32, 0x80, 0x90] {
        for lane in 0..3 {
            let at = base + column + 4 * lane;
            let value = e.mem.f32(at);
            e.mem.set_f32(at, (value as f64 * cube as f64) as f32);
        }
    }
    let at = base + BhkRigidBodyCinfo::m_mass.off;
    let mass = e.mem.f32(at);
    e.mem.set_f32(at, (scale as f64 * mass as f64) as f32);
}

// Translated from 00c8ce70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRigidBodyCinfo::Endian` (Xbox PDB): byte-swaps every multi-byte field of the cinfo
/// (the 16 words at `+0x30`, the inertia tensor, the centre of mass, the `u16` at
/// `+0x2E` and the nine words at `+0xB0`).
pub fn bhk_rigid_body_cinfo_endian(e: &mut Engine, this: Ptr<BhkRigidBodyCinfo>) {
    let base = this.addr();
    for word in 0..16 {
        swap_word(e, base + 0x30 + 4 * word);
    }
    fn_00c8cdb0(e, Ptr::new(base + 0x70));
    for word in 0..4 {
        swap_word(e, base + 0xa0 + 4 * word);
    }
    let delay = e.mem.u16(base + 0x2e);
    e.mem.set_u16(base + 0x2e, delay.swap_bytes());
    for word in 0..9 {
        swap_word(e, base + 0xb0 + 4 * word);
    }
}

/// Copies 16 bytes (one `MOVAPS`).
fn copy_vector(e: &mut Engine, destination: u32, source: u32) {
    for word in 0..4 {
        let value = e.mem.u32(source + 4 * word);
        e.mem.set_u32(destination + 4 * word, value);
    }
}

/// An `FLD dword` / `FSTP dword` copy of one float.
fn copy_float(e: &mut Engine, destination: u32, source: u32) {
    let bits = x87_quiet_bits(e.mem.u32(source));
    e.mem.set_u32(destination, bits);
}

// Translated from 00c8cfd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRigidBody::CopyCinfo` (Xbox PDB): the `bhkWorldObject` copy, then (with a Havok
/// body) `hkpRigidBody::getCinfo` into the Havok half at `cinfo + 0x20`, and from the body's
/// motion the mass, centre of mass, both dampings and the inertia tensor (virtual slot
/// `0x14` of the motion), the centre of mass copied again afterwards; finally the
/// maximum linear and angular velocities.
pub fn bhk_rigid_body_copy_cinfo(
    e: &mut Engine,
    this: Ptr<BhkRigidBody>,
    cinfo: Ptr<BhkRigidBodyCinfo>,
) {
    let info = cinfo.addr();
    e.call(WORLD_OBJECT_COPY_CINFO, &args![this, cinfo]);
    let body = e.get(this, BhkRigidBody::phkObject);
    if body.is_null() {
        return;
    }
    e.call(RIGID_BODY_GET_CINFO, &args![body, info + 0x20]);
    let body = e.get(this, BhkRigidBody::phkObject);
    if !body.is_null() {
        let motion = e.call(RIGID_BODY_GET_MOTION, &args![body]).u32();
        if motion != 0 {
            let mass = e.call(MOTION_GET_MASS, &args![motion]).f32();
            e.mem.set_f32(info + BhkRigidBodyCinfo::m_mass.off, mass);
            copy_vector(e, info + 0xa0, motion + 0x90);
            copy_float(e, info + 0xb4, motion + 0xb4);
            copy_float(e, info + 0xb8, motion + 0xb8);
            e.vcall(motion, 0x14, &args![info + 0x70]);
            copy_vector(e, info + 0xa0, motion + 0x90);
        }
    }
    if !e.get(this, BhkRigidBody::phkObject).is_null() {
        let linear = e.call(GET_MAX_LINEAR_VELOCITY, &args![this]).f32();
        e.mem
            .set_f32(info + BhkRigidBodyCinfo::m_maxLinearVelocity.off, linear);
        let angular = bhk_rigid_body_get_max_angular_velocity(e, this);
        e.mem
            .set_f32(info + BhkRigidBodyCinfo::m_maxAngularVelocity.off, angular);
    }
}

// Translated from 00c8d090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRigidBody::RegisterStreamables` (Xbox PDB): the `bhkWorldObject` registration and,
/// when it succeeds, virtual slot `0x54` (`RegisterStreamables`) of every item of the
/// action/constraint list. Returns the base result.
pub fn bhk_rigid_body_register_streamables(
    e: &mut Engine,
    this: Ptr<BhkRigidBody>,
    stream: u32,
) -> bool {
    let registered = e
        .call(WORLD_OBJECT_REGISTER_STREAMABLES, &args![this, stream])
        .bool();
    if registered {
        let mut node = this.at(BhkRigidBody::ActionConstraint);
        loop {
            let item = e.get(node, BSSimpleList::m_item);
            if item == 0 {
                break;
            }
            e.vcall(item, 0x54, &args![stream]);
            node = Ptr::new(e.get(node, BSSimpleList::m_pkNext));
            if node.is_null() {
                break;
            }
        }
    }
    registered
}

// Translated from 00c8d0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the number of items in the action/constraint list (nodes up to the first with
/// no item).
pub fn fn_00c8d0d0(e: &mut Engine, this: Ptr<BhkRigidBody>) -> u32 {
    let mut count = 0;
    let mut node = this.at(BhkRigidBody::ActionConstraint);
    while e.get(node, BSSimpleList::m_item) != 0 {
        node = Ptr::new(e.get(node, BSSimpleList::m_pkNext));
        count += 1;
        if node.is_null() {
            break;
        }
    }
    count
}

/// Copies the 16 bytes at `body + offset` of the Havok body to `out`, or the 16 bytes at
/// `fallback` when there is no body.
fn copy_body_vector(
    e: &mut Engine,
    this: Ptr<BhkRigidBody>,
    offset: u32,
    out: Ptr,
    fallback: u32,
) -> Ptr {
    let body = if this.is_null() {
        Ptr::NULL
    } else {
        e.get(this, BhkRigidBody::phkObject)
    };
    let source = if body.is_null() {
        fallback
    } else {
        body.addr() + offset
    };
    copy_vector(e, out.addr(), source);
    out
}

// Translated from 00c8d0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRigidBody::GetPosition_ov2` (Xbox PDB name): copies the body's position (`+0x120`)
/// into `out`, the default vector without a body.
pub fn bhk_rigid_body_get_position_ov2(e: &mut Engine, this: Ptr<BhkRigidBody>, out: Ptr) -> Ptr {
    copy_body_vector(e, this, 0x120, out, DEFAULT_VECTOR)
}

// Translated from 00c8d130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRigidBody::GetRotation_ov2` (Xbox PDB name): copies the body's rotation quaternion
/// (`+0x160`) into `out`, the default rotation without a body.
pub fn bhk_rigid_body_get_rotation_ov2(e: &mut Engine, this: Ptr<BhkRigidBody>, out: Ptr) -> Ptr {
    copy_body_vector(e, this, 0x160, out, DEFAULT_ROTATION)
}

// Translated from 00c8d200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRigidBody::GetCenterOfMassLocal` (Xbox PDB): copies the body's local centre of mass
/// (`+0x170`) into `out`, the default vector without a body.
pub fn bhk_rigid_body_get_center_of_mass_local(
    e: &mut Engine,
    this: Ptr<BhkRigidBody>,
    out: Ptr,
) -> Ptr {
    copy_body_vector(e, this, 0x170, out, DEFAULT_VECTOR)
}

// Translated from 00c8d240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRigidBody::GetCenterOfMassInWorld` (Xbox PDB): copies the body's world centre of
/// mass (`+0x140`) into `out`, the default vector without a body.
pub fn bhk_rigid_body_get_center_of_mass_in_world(
    e: &mut Engine,
    this: Ptr<BhkRigidBody>,
    out: Ptr,
) -> Ptr {
    copy_body_vector(e, this, 0x140, out, DEFAULT_VECTOR)
}

/// The empty lock call (`00a29680`) the PC build kept around the Havok accesses.
fn lock(e: &mut Engine, this: Ptr<BhkRigidBody>) {
    e.call(EMPTY_LOCK, &args![this]);
}

// Translated from 00c8d170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRigidBody::SetPosition` (Xbox PDB): `hkpRigidBody::setPosition(position)` between
/// two empty lock calls; nothing without a Havok body.
pub fn bhk_rigid_body_set_position(e: &mut Engine, this: Ptr<BhkRigidBody>, position: Ptr) {
    if this.is_null() {
        return;
    }
    let body = e.get(this, BhkRigidBody::phkObject);
    if !body.is_null() {
        lock(e, this);
        e.call(RIGID_BODY_SET_POSITION, &args![body, position]);
        lock(e, this);
    }
}

// Translated from 00c8d1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRigidBody::SetRotation` (Xbox PDB): `hkpRigidBody::setRotation(rotation)` between
/// two empty lock calls; nothing without a Havok body.
pub fn bhk_rigid_body_set_rotation(e: &mut Engine, this: Ptr<BhkRigidBody>, rotation: Ptr) {
    if this.is_null() {
        return;
    }
    let body = e.get(this, BhkRigidBody::phkObject);
    if !body.is_null() {
        lock(e, this);
        e.call(RIGID_BODY_SET_ROTATION, &args![body, rotation]);
        lock(e, this);
    }
}

// Translated from 00c8d1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRigidBody::SetPosRot` (Xbox PDB): `bhkWorldObject::GethkWorldRw` (result unused),
/// then `hkpRigidBody::setPositionAndRotation(position, rotation)`; nothing without a body.
pub fn bhk_rigid_body_set_pos_rot(
    e: &mut Engine,
    this: Ptr<BhkRigidBody>,
    position: Ptr,
    rotation: Ptr,
) {
    if this.is_null() {
        return;
    }
    let body = e.get(this, BhkRigidBody::phkObject);
    if !body.is_null() {
        e.call(WORLD_OBJECT_GET_WORLD_RW, &args![this]);
        e.call(
            RIGID_BODY_SET_POSITION_AND_ROTATION,
            &args![body, position, rotation],
        );
    }
}

// Translated from 00c8d280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRigidBody::GetTransform` (Xbox PDB): copies the body's 64-byte transform (`+0xF0`)
/// into `out`, or writes the identity matrix (1.0 at `+0`, `+0x14`, `+0x28`; zero
/// elsewhere) without a body.
pub fn bhk_rigid_body_get_transform(e: &mut Engine, this: Ptr<BhkRigidBody>, out: Ptr) -> Ptr {
    let body = if this.is_null() {
        Ptr::NULL
    } else {
        e.get(this, BhkRigidBody::phkObject)
    };
    if body.is_null() {
        for word in 0..16 {
            e.mem.set_u32(out.addr() + 4 * word, 0);
        }
        for diagonal in [0u32, 0x14, 0x28] {
            e.mem.set_f32(out.addr() + diagonal, 1.0);
        }
    } else {
        for row in 0..4 {
            copy_vector(e, out.addr() + 0x10 * row, body.addr() + 0xf0 + 0x10 * row);
        }
    }
    out
}

// Translated from 00c8d2f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c8d2f0 - /GS stack-cookie check (00ec408c) is not modeled
/// `bhkRigidBody::CreateDisplayGeometry` (Xbox PDB): the `bhkWorldObject` display
/// geometry node; with one, a coordinate jack (`BSTestObjects::CreateCoordinateJack`)
/// sized by the float at `01050c48` (6.0) is created at the body's local centre of mass
/// (converted to a `NiPoint3` by `00458620`) and attached to it as child (virtual slot
/// `0xDC` of the node, second argument 1). Returns the node (0 when it was not made).
pub fn bhk_rigid_body_create_display_geometry(
    e: &mut Engine,
    this: Ptr<BhkRigidBody>,
    argument: u32,
) -> u32 {
    let node = e
        .call(WORLD_OBJECT_CREATE_DISPLAY_GEOMETRY, &args![this, argument])
        .u32();
    if node == 0 {
        return 0;
    }
    e.with_stack(0x1c, |e, scratch| {
        let point = scratch.addr();
        let vector = point + 12;
        e.vcall(this.addr(), 0xec, &args![vector]);
        e.call(NI_POINT3_FROM_VECTOR, &args![point, vector]);
        let size = x87_quiet_bits(e.mem.u32(COORDINATE_JACK_SIZE));
        let jack = e.call(CREATE_COORDINATE_JACK, &args![size]).u32();
        if jack != 0 {
            for axis in 0..3 {
                let value = e.mem.u32(point + 4 * axis);
                e.mem.set_u32(jack + 0x58 + 4 * axis, value);
            }
            e.vcall(node, 0xdc, &args![jack, 1u32]);
        }
    });
    node
}

// Translated from 00c8d390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRigidBody::GetAabbLocal` (Xbox PDB): the local bounding box of the body's shape,
/// `shape->virtual 0x1C(01268370, 0.001, out)` between two empty lock calls; without a
/// body or shape `out` is set to `min = (-0.001, -0.001, -0.001, 0)` and
/// `max = (0.001, 0.001, 0.001, 0)` from the floats at `0104ffb8`, `01017d00`, `01011d78`.
pub fn bhk_rigid_body_get_aabb_local(e: &mut Engine, this: Ptr<BhkRigidBody>, out: Ptr) {
    let mut shape = 0;
    if !this.is_null() {
        let body = e.get(this, BhkRigidBody::phkObject);
        if !body.is_null() {
            lock(e, this);
            lock(e, this);
            shape = e.mem.u32(body.addr() + 0x10);
        }
    }
    if shape != 0 {
        let margin = x87_quiet_bits(e.mem.u32(AABB_HIGH_FILL));
        e.vcall(shape, 0x1c, &args![SHAPE_QUERY_CONTEXT, margin, out]);
    } else {
        let low = e.mem.u32(AABB_LOW_FILL);
        let high = e.mem.u32(AABB_HIGH_FILL);
        let w = e.mem.u32(AABB_W_FILL);
        for (lane, value) in [low, low, low, w].into_iter().enumerate() {
            e.mem.set_u32(out.addr() + 4 * lane as u32, value);
        }
        for (lane, value) in [high, high, high, w].into_iter().enumerate() {
            e.mem.set_u32(out.addr() + 0x10 + 4 * lane as u32, value);
        }
    }
}

// Translated from 00c8d440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRigidBody::SetHavokAssertState` (Xbox PDB; only reachable through vtable data):
/// sets four Havok message ids (`f043d534`, `f02e32e1`, `f02e32e2`, `f02132df`) enabled or
/// disabled on the Havok error object (`01268360`, virtual slot `0x10`). The value is the
/// low byte of the stack word (the exe leaves the other three bytes as they were).
pub fn bhk_rigid_body_set_havok_assert_state(e: &mut Engine, _unused_this: Ptr, state: u32) {
    let object = e.mem.u32(HAVOK_ERROR_OBJECT);
    let value = state & 0xff;
    for id in [0xf043_d534u32, 0xf02e_32e1, 0xf02e_32e2, 0xf021_32df] {
        e.vcall(object, HAVOK_ERROR_SLOT_SET_ENABLED, &args![id, value]);
    }
}

// Translated from 00c8d4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRigidBodyCinfo::GetQualityTypeString` (Xbox PDB): the address of the name string of
/// the cinfo's `m_qualityType` (-1..=7; 3 and anything else give 0).
pub fn bhk_rigid_body_cinfo_get_quality_type_string(
    e: &mut Engine,
    this: Ptr<BhkRigidBodyCinfo>,
) -> u32 {
    match e.get(this, BhkRigidBodyCinfo::m_qualityType) {
        -1 => 0x0104_8858,
        0 => 0x010c_6084,
        1 => 0x010c_6078,
        2 => 0x0102_9400,
        4 => 0x0109_25bc,
        5 => 0x010c_606c,
        6 => 0x010c_6064,
        7 => 0x010c_605c,
        _ => 0,
    }
}

// Translated from 00c8d510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`this` is the destination cinfo, RET 4): converts an old-format cinfo (stream
/// versions below 0x23, 0xE0 bytes) into the current layout: the filter (to `+0` and
/// `+0x20`), shape, response byte and delay, the vectors and inertia, the floats from
/// `+0xB0` (those from `+0xBC` move up by four bytes; gravity factor is set to 1.0 and
/// the local frame to 0), the motion type renumbered (`>= 5` minus 2, `>= 3` minus 1 as
/// unsigned), `enableDeactivation = (old byte == 2)`, solver deactivation and quality
/// (signed, `< 3` minus 1).
pub fn fn_00c8d510(e: &mut Engine, this: Ptr<BhkRigidBodyCinfo>, old: Ptr) {
    let dest = this.addr();
    let src = old.addr();
    e.mem.set_f32(dest + 0xbc, 1.0);
    e.mem.set_u32(dest + 0x28, 0);
    let filter = e.mem.u32(src);
    e.mem.set_u32(dest, filter);
    e.mem.set_u32(dest + 0x20, filter);
    let shape = e.mem.u32(src + 0x24);
    e.mem.set_u32(dest + 0x24, shape);
    let response = e.mem.u8(src + 0x28);
    e.mem.set_u8(dest + 0x2c, response);
    let delay = e.mem.u16(src + 0x2a);
    e.mem.set_u16(dest + 0x2e, delay);
    for block in 0..8 {
        copy_vector(e, dest + 0x30 + 0x10 * block, src + 0x30 + 0x10 * block);
    }
    for (to, from) in [
        (0xb0u32, 0xb0u32),
        (0xb4, 0xb4),
        (0xb8, 0xb8),
        (0xc0, 0xbc),
        (0xc4, 0xc0),
        (0xc8, 0xc4),
        (0xcc, 0xc8),
        (0xd0, 0xcc),
    ] {
        copy_float(e, dest + to, src + from);
    }
    let mut motion = e.mem.i8(src + 0xd0) as i32 as u32;
    if motion >= 5 {
        motion = motion.wrapping_sub(2);
    } else if motion >= 3 {
        motion -= 1;
    }
    e.mem.set_u8(dest + 0xd4, motion as u8);
    let deactivation = e.mem.u8(src + 0xd1) == 2;
    e.mem.set_u8(dest + 0xd5, deactivation as u8);
    let solver = e.mem.u8(src + 0xd2);
    e.mem.set_u8(dest + 0xd6, solver);
    let mut quality = e.mem.i8(src + 0xd3) as i32;
    if quality < 3 {
        quality -= 1;
    }
    e.mem.set_u8(dest + 0xd7, quality as u8);
}

/// Sets the allocation-scope word to 0x10 and returns its address and old value.
fn enter_scope(e: &mut Engine) -> (u32, u32) {
    let word = e.tls() + SCOPE_WORD;
    let saved = e.mem.u32(word);
    e.mem.set_u32(word, 0x10);
    (word, saved)
}

/// Whether `object`'s run-time type (the chain of `GetRTTI` nodes through the parent
/// word at `+4`) contains the `NiRTTI` at `rtti`.
fn is_kind_of(e: &mut Engine, object: u32, rtti: u32) -> bool {
    let mut node = e.vcall(object, VSLOT_GET_RTTI, &args![]).u32();
    while node != 0 {
        if node == rtti {
            return true;
        }
        node = e.mem.u32(node + 4);
    }
    false
}

// Translated from 00c8d650 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c8d650 - exception-unwinding frame (FS:[0]) and /GS stack-cookie
// check (00ec408c) are not modeled
/// `bhkRigidBody::Init` (Xbox PDB): sanitises the cinfo, builds the Havok rigid body from
/// its Havok half and attaches it. The allocation-scope word is 0x10 throughout.
///
/// Clamps: the allowed penetration depth (`+0xD0`) becomes 0.15 when it is at most
/// `2^-23` or above 0.2; the maximum linear velocity (`+0xC8`) becomes 1000.0 when below
/// 1000.0; the restitution (`+0xC4`) becomes 1.98 when at least 1.99; a solver
/// deactivation outside 1..=4 becomes 2. Zero mass (`+0xB0`) turns a fixed body's
/// quality (`+0xD7`) into 0; a non-zero mass turns motion type 4 or 5 (keyframed, fixed)
/// into dynamic (1, quality 4) and remembers the type to set on the body afterwards, and
/// an unset quality (-1) into 4. The body (0x220 bytes from the memory router) is
/// constructed from `cinfo + 0x20`, given to the object (virtual slot `0x8C`) and released.
/// A dynamic body (motion types 1..=3, 6) then gets its quality from its local bounding
/// box: the smallest extent against two globals and the maximum linear velocity decide
/// between 2 and 4 (also stored in the body's broadphase handle at `+0x2A`). Finally
/// virtual slot `0xBC` (`PostInit`) gets the cinfo.
pub fn bhk_rigid_body_init(e: &mut Engine, this: Ptr<BhkRigidBody>, cinfo: Ptr<BhkRigidBodyCinfo>) {
    let (word, saved) = enter_scope(e);
    let info = cinfo.addr();
    let floor = e.global::<f32>(PENETRATION_DEPTH_FLOOR);
    let depth = e.get(cinfo, BhkRigidBodyCinfo::m_allowedPenetrationDepth);
    if floor >= depth || depth as f64 > e.global::<f64>(PENETRATION_DEPTH_CEILING) {
        copy_float(e, info + 0xd0, DEFAULT_PENETRATION_DEPTH);
    }
    let max_velocity = e.get(cinfo, BhkRigidBodyCinfo::m_maxLinearVelocity);
    if (max_velocity as f64) < e.global::<f64>(MAX_LINEAR_VELOCITY_FLOOR) {
        copy_float(e, info + 0xc8, DEFAULT_MAX_LINEAR_VELOCITY);
    }
    let restitution = e.get(cinfo, BhkRigidBodyCinfo::m_restitution);
    if (restitution as f64) >= e.global::<f64>(RESTITUTION_CEILING) {
        copy_float(e, info + 0xc4, DEFAULT_RESTITUTION);
    }
    let mut override_motion = 8i32;
    if e.get(cinfo, BhkRigidBodyCinfo::m_mass) == 0.0 {
        if e.mem.u8(info + 0xd4) == 5 {
            e.mem.set_u8(info + 0xd7, 0);
        }
    } else {
        let motion = e.mem.i8(info + 0xd4) as i32;
        if motion == 4 || motion == 5 {
            override_motion = motion;
            e.mem.set_u8(info + 0xd4, 1);
            e.mem.set_u8(info + 0xd7, 4);
        } else if e.mem.u8(info + 0xd7) == 0xff {
            e.mem.set_u8(info + 0xd7, 4);
        }
    }
    let solver = e.mem.i8(info + 0xd6);
    if !(solver < 5 && solver > 0) {
        e.mem.set_u8(info + 0xd6, 2);
    }
    let filter = e.mem.u32(info);
    let shape = e.mem.u32(info + 4);
    e.mem.set_u8(info + 0xda, 3);
    e.mem.set_u32(info + 0x20, filter);
    e.mem.set_u32(info + 0x24, shape);
    let allocator = memory_allocator(e);
    let raw = e.vcall(allocator, 4, &args![0x220u32]).u32();
    e.mem.set_u16(raw + 4, 0x220);
    let body = e.call(RIGID_BODY_CONSTRUCT, &args![raw, info + 0x20]).u32();
    if override_motion != 8 {
        e.mem.set_u8(info + 0xd4, override_motion as u8);
        e.call(
            RIGID_BODY_SET_MOTION_TYPE,
            &args![body, override_motion as u32, 1u32, 0u32],
        );
    }
    e.vcall(this.addr(), VSLOT_SET_HK_OBJECT, &args![body]);
    e.call(REMOVE_REFERENCE, &args![body]);
    let motion = e.mem.i8(info + 0xd4) as i32;
    if motion > 0 && (motion <= 3 || motion == 6) {
        let body = e.get(this, BhkRigidBody::phkObject);
        if !body.is_null() {
            lock(e, this);
            lock(e, this);
            let handle = body.addr() + 0x10;
            let extent = e.with_stack(0x20, |e, boxes| {
                let low = boxes.addr();
                let high = low + 0x10;
                bhk_rigid_body_get_aabb_local(e, this, boxes);
                [0, 4, 8].map(|lane| e.mem.f32(high + lane) - e.mem.f32(low + lane))
            });
            let smallest = if extent[1] < extent[0] {
                if extent[2] < extent[1] {
                    2
                } else {
                    1
                }
            } else if extent[2] < extent[0] {
                2
            } else {
                0
            };
            let limit = e.global::<f32>(QUALITY_EXTENT_LIMIT);
            let set_quality = |e: &mut Engine, quality: u8| {
                e.mem.set_u8(handle + 0x1a, quality);
                e.mem.set_u8(info + 0xd7, quality);
            };
            if limit < extent[smallest] {
                set_quality(e, 2);
            } else {
                let velocity = e.call(GET_MAX_LINEAR_VELOCITY, &args![this]).f32();
                let scaled =
                    (e.global::<f32>(QUALITY_VELOCITY_SCALE) as f64 * velocity as f64) as f32;
                if e.global::<f32>(QUALITY_VELOCITY_LIMIT) > scaled {
                    set_quality(e, 2);
                } else if e.mem.u8(info + 0xd7) == 2 {
                    set_quality(e, 4);
                }
            }
        }
    }
    e.vcall(this.addr(), VSLOT_POST_INIT, &args![cinfo]);
    e.mem.set_u32(word, saved);
}

/// One write through the output stream object at `out` (a callback pointer at `+0xC`,
/// the position counter at `+4`): the callback gets `(out, data, 4, &four, 1)` with a
/// local holding 4, and its result advances the position.
fn stream_write_word(e: &mut Engine, out: u32, data: u32) {
    let write = e.mem.u32(out + 0xc);
    let written = e.with_stack(4, |e, four| {
        e.mem.set_u32(four.addr(), 4);
        e.call(write, &args![out, data, 4u32, four, 1u32]).u32()
    });
    let position = e.mem.u32(out + 4);
    e.mem.set_u32(out + 4, position.wrapping_add(written));
}

// Translated from 00c8d960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRigidBody::SaveBinary` (Xbox PDB): gets the cinfo through virtual slot `0xB4`
/// (`CreateCinfo`, with a flag byte), byte-swaps it when the stream is not the PC
/// byte order (`stream + 0x1E4 != 1`), runs `bhkEntity::SaveBinary`, writes the number
/// of action/constraint items, a link id for each (stream virtual slot `0x30`) and
/// `iFlags` to the stream's output object (`stream + 0x250`), then swaps the cinfo back.
pub fn bhk_rigid_body_save_binary(e: &mut Engine, this: Ptr<BhkRigidBody>, stream: u32) {
    let cinfo = e
        .with_stack(4, |e, flag| {
            e.vcall(this.addr(), VSLOT_CREATE_CINFO, &args![flag])
        })
        .u32();
    if e.mem.u8(stream + 0x1e4) != 1 {
        bhk_rigid_body_cinfo_endian(e, Ptr::new(cinfo));
    }
    e.call(ENTITY_SAVE_BINARY, &args![this, stream]);
    let count = fn_00c8d0d0(e, this);
    let out = e.mem.u32(stream + 0x250);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), count);
        stream_write_word(e, out, slot.addr());
    });
    let mut node = this.at(BhkRigidBody::ActionConstraint);
    loop {
        let item = e.get(node, BSSimpleList::m_item);
        if item == 0 {
            break;
        }
        e.vcall(stream, 0x30, &args![item]);
        node = Ptr::new(e.get(node, BSSimpleList::m_pkNext));
        if node.is_null() {
            break;
        }
    }
    let out = e.mem.u32(stream + 0x250);
    stream_write_word(e, out, this.addr() + BhkRigidBody::iFlags.off);
    if e.mem.u8(stream + 0x1e4) != 1 {
        bhk_rigid_body_cinfo_endian(e, Ptr::new(cinfo));
    }
}

// Translated from 00c8da40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c8da40 - exception-unwinding frame (FS:[0]) is not modeled
/// `bhkRigidBody::Kill` (Xbox PDB): with the allocation-scope word at 0x10, releases the
/// action/constraint list (`004ee920` on the embedded list head), then `bhkEntity::Kill`.
pub fn bhk_rigid_body_kill(e: &mut Engine, this: Ptr<BhkRigidBody>) {
    let (word, saved) = enter_scope(e);
    let list = this.at(BhkRigidBody::ActionConstraint);
    e.call(ACTION_LIST_CLEAR, &args![list]);
    e.call(ENTITY_KILL, &args![this]);
    e.mem.set_u32(word, saved);
}

// Translated from 00c8dac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRigidBody::AddActConToWorld` (Xbox PDB): adds every action/constraint item to
/// `world` through its virtual slot `0x9C`. When bit 2 of `iFlags` is set only items
/// that are `bhkConstraint`s and also `bhkFixedConstraint`s are added (each test reads
/// the item's `GetRTTI` chain).
pub fn bhk_rigid_body_add_act_con_to_world(e: &mut Engine, this: Ptr<BhkRigidBody>, world: u32) {
    let restricted = (e.get(this, BhkRigidBody::iFlags) >> 2) & 1 != 0;
    let mut node = this.at(BhkRigidBody::ActionConstraint);
    loop {
        let item = e.get(node, BSSimpleList::m_item);
        if item == 0 {
            break;
        }
        let add = if restricted {
            is_kind_of(e, item, CONSTRAINT_RTTI) && {
                let item = e.get(node, BSSimpleList::m_item);
                item != 0 && is_kind_of(e, item, FIXED_CONSTRAINT_RTTI)
            }
        } else {
            true
        };
        if add {
            let item = e.get(node, BSSimpleList::m_item);
            e.vcall(item, VSLOT_ADD, &args![world]);
        }
        node = Ptr::new(e.get(node, BSSimpleList::m_pkNext));
        if node.is_null() {
            break;
        }
    }
}

/// The record of the Havok property `01267b70` on `body` (virtual slot `0x3C` of the
/// property), or 0.
fn property_record(e: &mut Engine, body: u32) -> u32 {
    let property = e.with_stack(8, |e, out| {
        let result = e
            .call(WORLD_OBJECT_GET_PROPERTY, &args![out, body, PROPERTY_KEY])
            .u32();
        e.mem.u32(result)
    });
    if property == 0 {
        0
    } else {
        e.vcall(property, 0x3c, &args![]).u32()
    }
}

// Translated from 00c8db50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRigidBody::Add` (Xbox PDB): looks up the body's property `01267b70`; when its record
/// has `1.0` at `+0x18` and `+0x1C` it is "marked", and the object is only added when
/// the record's count at `+0x28` is positive or the byte `011afe59` is set; objects with
/// `iFlags` bit 6 are never added. Adding runs `bhkWorldObject::Add`; when that fails the
/// object still proceeds if its body is already in a world and its motion type is 1..=3 or
/// 6, and then (record count positive when marked, bit 6 clear) adds the
/// action/constraint items. The record's `+0x24` receives `world`. When the add is
/// skipped, a still-present world makes the object remove itself (virtual slot `0xA0`)
/// and the result is true; otherwise the result is the base add's.
pub fn bhk_rigid_body_add(e: &mut Engine, this: Ptr<BhkRigidBody>, world: u32) -> bool {
    let body = if this.is_null() {
        0
    } else {
        e.get(this, BhkRigidBody::phkObject).addr()
    };
    let record = property_record(e, body);
    let marked =
        record != 0 && 1.0f32 == e.mem.f32(record + 0x18) && 1.0f32 == e.mem.f32(record + 0x1c);
    let wanted = !marked || e.mem.i32(record + 0x28) > 0 || e.mem.u8(ASSERT_STATE_BYTE) != 0;
    if !wanted || (e.get(this, BhkRigidBody::iFlags) >> 6) & 1 != 0 {
        if e.call(WORLD_OBJECT_GET_WORLD, &args![this]).u32() != 0 {
            e.vcall(this.addr(), VSLOT_REMOVE, &args![]);
        }
        if record != 0 {
            e.mem.set_u32(record + 0x24, world);
        }
        return true;
    }
    let added = e.call(WORLD_OBJECT_ADD, &args![this, world]).bool();
    let mut proceed = true;
    if !added {
        let hk_object = e.get(this, BhkRigidBody::phkObject);
        proceed = false;
        if !hk_object.is_null() {
            lock(e, this);
            let in_world = e.mem.u32(hk_object.addr() + 8) != 0;
            lock(e, this);
            if in_world {
                let hk_object = e.get(this, BhkRigidBody::phkObject);
                if !hk_object.is_null() {
                    let motion = e.mem.u8(hk_object.addr() + 0xe8);
                    proceed = motion > 0 && (motion <= 3 || motion == 6);
                }
            }
        }
    }
    if proceed
        && !(marked && e.mem.i32(record + 0x28) <= 0)
        && (e.get(this, BhkRigidBody::iFlags) >> 6) & 1 == 0
    {
        bhk_rigid_body_add_act_con_to_world(e, this, world);
    }
    if record != 0 {
        e.mem.set_u32(record + 0x24, world);
    }
    added
}

// Translated from 00c8dcb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRigidBody::ForceAdd` (Xbox PDB): `bhkWorldObject::Add`; when it succeeds the
/// action/constraint items are added too. The property `01267b70` record, if any, gets
/// `world` at `+0x24`. Returns the base add's result.
pub fn bhk_rigid_body_force_add(e: &mut Engine, this: Ptr<BhkRigidBody>, world: u32) -> bool {
    let added = e.call(WORLD_OBJECT_ADD, &args![this, world]).bool();
    if added {
        bhk_rigid_body_add_act_con_to_world(e, this, world);
    }
    let body = if this.is_null() {
        0
    } else {
        e.get(this, BhkRigidBody::phkObject).addr()
    };
    let record = property_record(e, body);
    if record != 0 {
        e.mem.set_u32(record + 0x24, world);
    }
    added
}

// Translated from 00c8dd10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRigidBody::SetMotionType` (Xbox PDB): false without a Havok body. When the body's
/// broadphase handle word (`body + 0x2C`) has collision layer (low 7 bits) 8 or 0x1D, the
/// layer becomes 0x1D for motion types 1..=3 and 6 and 8 for the others; with an unchanged
/// motion type only `00626cb0(this, new word)` runs, otherwise the word is stored and
/// `hkpRigidBody::setMotionType(motion, 1, 0)` is called between two empty locks. Any
/// other layer: false when the type is unchanged, or when a keyframed/fixed body (4, 5)
/// with zero dynamic mass is asked for a dynamic type; otherwise `setMotionType` between
/// the locks. True when it acted.
pub fn bhk_rigid_body_set_motion_type(
    e: &mut Engine,
    this: Ptr<BhkRigidBody>,
    motion: i32,
) -> bool {
    let body = e.get(this, BhkRigidBody::phkObject);
    if body.is_null() {
        return false;
    }
    let word = body.addr() + 0x2c;
    let flags = e.mem.u32(word);
    let current = e.mem.u8(body.addr() + 0xe8) as i32;
    let layer = flags & 0x7f;
    if layer == 8 || layer == 0x1d {
        let dynamic = motion > 0 && (motion < 4 || motion == 6);
        let new_layer = if dynamic { 0x1d } else { 8 };
        let new_flags = (flags & !0x7f) | new_layer;
        if motion == current {
            e.call(FIX_LAYER, &args![this, new_flags]);
        } else {
            lock(e, this);
            e.mem.set_u32(word, new_flags);
            e.call(
                RIGID_BODY_SET_MOTION_TYPE,
                &args![body, motion as u32, 1u32, 0u32],
            );
            lock(e, this);
        }
        return true;
    }
    if current == motion {
        return false;
    }
    if (4..=5).contains(&current) {
        let dynamic = e.with_stack(4, |e, slot| {
            e.mem.set_i32(slot.addr(), motion);
            fn_00c8cce0(e, slot)
        });
        if dynamic {
            let mass = e.call(GET_DYNAMIC_MASS, &args![this]).f32();
            if mass == 0.0 {
                return false;
            }
        }
    }
    lock(e, this);
    e.call(
        RIGID_BODY_SET_MOTION_TYPE,
        &args![body, motion as u32, 1u32, 0u32],
    );
    lock(e, this);
    true
}

/// Four single-precision lanes (an `hkVector4` / `hkQuaternion` in an XMM register).
type Vector = [f32; 4];

fn load_vector(e: &Engine, address: u32) -> Vector {
    [0, 4, 8, 12].map(|lane| e.mem.f32(address + lane))
}

fn store_vector(e: &mut Engine, address: u32, vector: Vector) {
    for (lane, value) in vector.into_iter().enumerate() {
        e.mem.set_f32(address + 4 * lane as u32, value);
    }
}

fn splat(value: f32) -> Vector {
    [value; 4]
}

fn multiply(a: Vector, b: Vector) -> Vector {
    [a[0] * b[0], a[1] * b[1], a[2] * b[2], a[3] * b[3]]
}

fn add_vectors(a: Vector, b: Vector) -> Vector {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2], a[3] + b[3]]
}

fn subtract(a: Vector, b: Vector) -> Vector {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2], a[3] - b[3]]
}

/// The sum of the squares of the first three lanes the way the SSE code adds them:
/// `z + (y + x)`.
fn squared_length(vector: Vector) -> f32 {
    let squares = multiply(vector, vector);
    squares[2] + (squares[1] + squares[0])
}

/// Half the rotation of the vector `c` by the quaternion `q` (`xyzw`), exactly as the SSE
/// code in `00c8de20` and `00c8e160` computes it: `cross(q.xyz, c) * w + (w * w + bias) * c
/// + dot(c, q) * q` with `bias` the vector `010c71e0` (-0.5 in every lane). Callers double it.
fn rotate_half(c: Vector, q: Vector, bias: Vector) -> Vector {
    let product = multiply(c, q);
    let dot = product[2] + (product[1] + product[0]);
    let forward = multiply([c[2], c[0], c[1], c[3]], [q[1], q[2], q[0], q[3]]);
    let backward = multiply([c[1], c[2], c[0], c[3]], [q[2], q[0], q[1], q[3]]);
    let scalar = splat(q[3]);
    let cross = multiply(subtract(forward, backward), scalar);
    let along = multiply(add_vectors(multiply(scalar, scalar), bias), c);
    let toward = multiply(splat(dot), q);
    add_vectors(cross, add_vectors(along, toward))
}

/// `hkpRigidBody` pointer of `this`, or 0.
fn hk_body(e: &Engine, this: Ptr<BhkRigidBody>) -> u32 {
    if this.is_null() {
        0
    } else {
        e.get(this, BhkRigidBody::phkObject).addr()
    }
}

// Translated from 00c8de20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c8de20 - /GS stack-cookie check (00ec408c) is not modeled
/// Unnamed (`this`, RET 0x10): moves the body towards a target transform by setting its
/// velocities. `position` and `rotation` point at 16-byte vectors (the rotation a
/// quaternion `xyzw`); `rate` multiplies the displacements (an inverse time step) and
/// `blend` mixes the new velocities with the body's current ones.
///
/// The target centre of mass is the local centre of mass (virtual slot `0xEC`) rotated by
/// `rotation` and moved by `position`; the linear velocity is `rate` times its distance
/// from the world centre of mass (slot `0xF0`), limited to the maximum linear velocity
/// (`00577b50`) and with `gravity.z * blend / rate` taken off the world's gravity
/// (`world + 0x10`, lane 2). The angular velocity is the axis and angle of the difference
/// `rotation * conjugate(current rotation)` (slot `0xD8`, `00c66320`, `00c74c20`,
/// `00c74c90`) times `rate`, used only above an angle of 0.001, limited to the maximum
/// angular velocity. Both are blended with the current velocities (`body + 0x1B0`,
/// `+0x1C0`) as `(1 - blend) * old + blend * new` and set between empty locks.
pub fn fn_00c8de20(
    e: &mut Engine,
    this: Ptr<BhkRigidBody>,
    position: Ptr,
    rotation: Ptr,
    rate: f32,
    blend: f32,
) {
    let object = this.addr();
    let bias = load_vector(e, HALF_VECTOR);
    let (mut linear, mut angular) = e.with_stack(0x60, |e, scratch| {
        let area = scratch.addr();
        let (vector, current, conjugate, difference, axis) =
            (area, area + 0x10, area + 0x20, area + 0x30, area + 0x40);
        let centre = e
            .vcall(object, VSLOT_GET_CENTER_OF_MASS_LOCAL, &args![vector])
            .u32();
        let centre = load_vector(e, centre);
        let q = load_vector(e, rotation.addr());
        let rotated = rotate_half(centre, q, bias);
        let target = add_vectors(
            add_vectors(rotated, rotated),
            load_vector(e, position.addr()),
        );
        let world_centre = e
            .vcall(object, VSLOT_GET_CENTER_OF_MASS_IN_WORLD, &args![vector])
            .u32();
        let world_centre = load_vector(e, world_centre);
        let mut linear = multiply(splat(rate), subtract(target, world_centre));
        let length = squared_length(linear).sqrt();
        let limit = e.call(GET_MAX_LINEAR_VELOCITY, &args![this]).f32();
        if limit < length {
            let scale = (limit as f64 / length as f64) as f32;
            linear = multiply(splat(scale), linear);
        }
        let current_rotation = e
            .vcall(object, VSLOT_GET_ROTATION_OV2, &args![current])
            .u32();
        let current_rotation = load_vector(e, current_rotation);
        let sign_mask = [0, 4, 8, 12].map(|lane| e.mem.u32(SIGN_MASK + lane));
        let conjugated: Vector = [0, 1, 2, 3]
            .map(|lane| f32::from_bits(current_rotation[lane].to_bits() ^ sign_mask[lane]));
        store_vector(e, conjugate, conjugated);
        e.call(QUATERNION_SET_MUL, &args![difference, rotation, conjugate]);
        e.call(QUATERNION_NORMALIZE, &args![difference]);
        let default = load_vector(e, DEFAULT_VECTOR);
        store_vector(e, axis, default);
        let angle = e.call(QUATERNION_GET_ANGLE, &args![difference]).f32();
        let mut angular = default;
        if angle as f64 > e.global::<f64>(ANGLE_THRESHOLD) {
            e.call(QUATERNION_GET_AXIS, &args![difference, axis]);
            let scaled = (angle as f64 * rate as f64) as f32;
            angular = multiply(splat(scaled), load_vector(e, axis));
        }
        let length = squared_length(angular).sqrt();
        let limit = bhk_rigid_body_get_max_angular_velocity(e, this);
        if limit < length {
            let scale = (limit as f64 / length as f64) as f32;
            angular = multiply(splat(scale), angular);
        }
        (linear, angular)
    });
    let world = e.call(WORLD_OBJECT_GET_WORLD, &args![this]).u32();
    if world != 0 {
        let gravity = e.mem.f32(world + 0x10 + 8);
        linear[2] = (linear[2] as f64 - (gravity as f64 * blend as f64) / rate as f64) as f32;
    }
    let body = hk_body(e, this);
    let old_linear = load_vector(
        e,
        if body == 0 {
            DEFAULT_VECTOR
        } else {
            body + 0x1b0
        },
    );
    let ones = load_vector(e, ONES_VECTOR);
    let mix = splat(blend);
    linear = add_vectors(
        multiply(subtract(ones, mix), old_linear),
        multiply(mix, linear),
    );
    let old_angular = load_vector(
        e,
        if body == 0 {
            DEFAULT_VECTOR
        } else {
            body + 0x1c0
        },
    );
    angular = add_vectors(
        multiply(subtract(ones, mix), old_angular),
        multiply(mix, angular),
    );
    if body != 0 {
        e.with_stack(0x10, |e, velocity| {
            store_vector(e, velocity.addr(), linear);
            lock(e, this);
            e.call(RIGID_BODY_SET_LINEAR_VELOCITY, &args![body, velocity]);
            lock(e, this);
        });
    }
    let body = hk_body(e, this);
    if body != 0 {
        e.with_stack(0x10, |e, velocity| {
            store_vector(e, velocity.addr(), angular);
            lock(e, this);
            e.call(RIGID_BODY_SET_ANGULAR_VELOCITY, &args![body, velocity]);
            lock(e, this);
        });
    }
}

/// Whether every lane of `velocity - 0` stays within `tolerance` in magnitude (the SSE
/// `CMPLEPS` against the sign-cleared difference from the default vector).
fn velocity_is_small(e: &Engine, velocity: Vector, tolerance: f32) -> bool {
    let zero = load_vector(e, DEFAULT_VECTOR);
    let mask = [0, 4, 8, 12].map(|lane| e.mem.u32(ABSOLUTE_MASK + lane));
    (0..4).all(|lane| {
        let difference = velocity[lane] - zero[lane];
        f32::from_bits(difference.to_bits() & mask[lane]) <= tolerance
    })
}

// Translated from 00c8e160 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c8e160 - /GS stack-cookie check (00ec408c) is not modeled
/// `bhkRigidBody::ApplyHardKeyframe` (Xbox PDB): drives the body to the keyframe
/// (`position` scaled by the float at `012681C8`, `rotation` reordered `wxyz` to `xyzw`)
/// by velocities, or by teleporting when the step is too large. Does nothing without a
/// world carrying a non-zero word at `+0x350`.
///
/// Near keyframes (squared distance not above the float at `010c6090`, and the rotation
/// dot product within the float at `010959f4` of 1) only zero the velocities that are not
/// already within 0.001 of zero (`setLinearVelocity`, `setAngularVelocity` of the Havok
/// body). Otherwise, with the global step `011afe64` (nothing when it is 0): the velocity
/// reaching the target centre of mass in one step is computed as in `00c8de20`; when its
/// squared length exceeds the squared maximum linear velocity the object is teleported
/// (`004a3e00` converts `position`, virtual slot `0xE8` `SetPosRot`). Otherwise the linear
/// velocity (zero with the `guard` bookkeeping when it exceeds 1e6) and the angular velocity
/// of the rotation difference (zero below an angle of 0.001) are set through `00561690`
/// and `005617c0`.
pub fn bhk_rigid_body_apply_hard_keyframe(
    e: &mut Engine,
    this: Ptr<BhkRigidBody>,
    position: Ptr,
    rotation: Ptr,
    guard: Ptr,
) {
    let body = hk_body(e, this);
    let world = e.mem.u32(body + 8);
    if world == 0 || e.mem.u32(world + 0x350) == 0 {
        return;
    }
    let scale = e.mem.f32(POSITION_SCALE);
    let [x, y, z] =
        [0, 4, 8].map(|lane| (e.mem.f32(position.addr() + lane) as f64 * scale as f64) as f32);
    let target = [x, y, z, 0.0];
    let keyframe_rotation_bits = [
        x87_quiet_bits(e.mem.u32(rotation.addr() + 4)),
        x87_quiet_bits(e.mem.u32(rotation.addr() + 8)),
        x87_quiet_bits(e.mem.u32(rotation.addr() + 0xc)),
        x87_quiet_bits(e.mem.u32(rotation.addr())),
    ];
    let quaternion = keyframe_rotation_bits.map(f32::from_bits);
    e.with_stack(0x90, |e, scratch| {
        let area = scratch.addr();
        let current_position = area;
        let current_rotation = area + 0x10;
        let local = area + 0x20;
        let wanted = area + 0x30;
        let difference = area + 0x40;
        let velocity_at = area + 0x50;
        let axis = area + 0x60;
        let converted = area + 0x70;
        let converted_rotation = area + 0x80;
        store_vector(e, wanted, quaternion);
        e.vcall(
            this.addr(),
            VSLOT_GET_POSITION_OV2,
            &args![current_position],
        );
        e.vcall(
            this.addr(),
            VSLOT_GET_ROTATION_OV2,
            &args![current_rotation],
        );
        let near_limit = x87_quiet(e.global::<f32>(NEAR_DISTANCE_SQUARED));
        let distance = squared_length(subtract(target, load_vector(e, current_position)));
        let mut moving = near_limit < distance;
        if !moving {
            let product = multiply(quaternion, load_vector(e, current_rotation));
            let swapped = [product[2], product[3], product[0], product[1]];
            let pairs = add_vectors(swapped, product);
            let dot = pairs[1] + pairs[0];
            let from_one = (dot as f64 - 1.0) as f32;
            moving = from_one.abs() as f64 > x87_quiet(e.global::<f32>(ROTATION_TOLERANCE)) as f64;
        }
        if !moving {
            let tolerance = e.global::<f32>(VELOCITY_TOLERANCE);
            let body = hk_body(e, this);
            let linear = load_vector(
                e,
                if body == 0 {
                    DEFAULT_VECTOR
                } else {
                    body + 0x1b0
                },
            );
            if !velocity_is_small(e, linear, tolerance) && body != 0 {
                lock(e, this);
                e.call(RIGID_BODY_SET_LINEAR_VELOCITY, &args![body, DEFAULT_VECTOR]);
                lock(e, this);
            }
            let body = hk_body(e, this);
            let angular = load_vector(
                e,
                if body == 0 {
                    DEFAULT_VECTOR
                } else {
                    body + 0x1c0
                },
            );
            if !velocity_is_small(e, angular, tolerance) && body != 0 {
                lock(e, this);
                e.call(
                    RIGID_BODY_SET_ANGULAR_VELOCITY,
                    &args![body, DEFAULT_VECTOR],
                );
                lock(e, this);
            }
            return;
        }
        let step = x87_quiet(e.global::<f32>(STEP_TIME));
        if step == 0.0 {
            return;
        }
        let inverse_step = (1.0f64 / step as f64) as f32;
        let centre = e
            .vcall(this.addr(), VSLOT_GET_CENTER_OF_MASS_LOCAL, &args![local])
            .u32();
        let centre = load_vector(e, centre);
        let rotated = rotate_half(centre, quaternion, load_vector(e, HALF_VECTOR));
        let world_centre = e
            .vcall(
                this.addr(),
                VSLOT_GET_CENTER_OF_MASS_IN_WORLD,
                &args![local],
            )
            .u32();
        let world_centre = load_vector(e, world_centre);
        let velocity = multiply(
            subtract(
                add_vectors(add_vectors(rotated, rotated), target),
                world_centre,
            ),
            splat(inverse_step),
        );
        store_vector(e, velocity_at, velocity);
        let squared = squared_length(velocity);
        let limit = e.call(GET_MAX_LINEAR_VELOCITY, &args![this]).f32();
        if squared as f64 > limit as f64 * limit as f64 {
            e.call(KEYFRAME_POSITION_CONVERT, &args![converted, position]);
            for (lane, bits) in keyframe_rotation_bits.into_iter().enumerate() {
                e.mem.set_u32(converted_rotation + 4 * lane as u32, bits);
            }
            e.vcall(
                this.addr(),
                VSLOT_SET_POS_ROT,
                &args![converted, converted_rotation],
            );
            return;
        }
        if squared.sqrt() as f64 > e.global::<f64>(SPEED_LIMIT) {
            if !guard.is_null() {
                e.call(GUARD_PREPARE, &args![guard, 0u32]);
            }
            e.call(KEYFRAME_TELEPORT_NOTIFY, &args![this, position]);
            if !guard.is_null() {
                let count = e.mem.u32(guard.addr() + 4).wrapping_sub(1);
                e.mem.set_u32(guard.addr() + 4, count);
                if count == 0 {
                    e.mem.set_u32(guard.addr(), 0);
                }
            }
            e.call(OBJECT_SET_LINEAR_VELOCITY, &args![this, DEFAULT_VECTOR]);
        } else {
            e.call(OBJECT_SET_LINEAR_VELOCITY, &args![this, velocity_at]);
        }
        e.call(
            QUATERNION_DIFFERENCE,
            &args![difference, wanted, current_rotation],
        );
        e.call(QUATERNION_NORMALIZE, &args![difference]);
        let angle = e.call(QUATERNION_GET_ANGLE, &args![difference]).f32();
        if x87_quiet(e.global::<f32>(VELOCITY_TOLERANCE)) > angle {
            store_vector(e, axis, [0.0; 4]);
        } else {
            e.call(QUATERNION_GET_AXIS, &args![difference, axis]);
            let scaled = (angle as f64 * inverse_step as f64) as f32;
            let spin = multiply(splat(scaled), load_vector(e, axis));
            store_vector(e, axis, spin);
        }
        e.call(OBJECT_SET_ANGULAR_VELOCITY, &args![this, axis]);
    });
}

// Translated from 00c8e670 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c8e670 - /GS stack-cookie check (00ec408c) is not modeled
/// `bhkRigidBody::GetAabb` (Xbox PDB): the world bounding box of the body's shape. Gets
/// the transform (virtual slot `0xF4`) and the shape (between two empty locks), then
/// `shape->virtual 0x1C(transform, 0.001, out)`; without a shape `out` gets the default
/// box (`-0.001` / `0.001` in the first three lanes, `0` in the last) transformed by
/// `hkVector4::setTransformedPos` (`00c7f640`) into `out` and `out + 0x10`.
pub fn bhk_rigid_body_get_aabb(e: &mut Engine, this: Ptr<BhkRigidBody>, out: Ptr) {
    e.with_stack(0x60, |e, scratch| {
        let transform = scratch.addr();
        let low = transform + 0x40;
        let high = transform + 0x50;
        e.vcall(this.addr(), VSLOT_GET_TRANSFORM, &args![transform]);
        let mut shape = 0;
        let body = e.get(this, BhkRigidBody::phkObject);
        if !body.is_null() {
            lock(e, this);
            lock(e, this);
            shape = e.mem.u32(body.addr() + 0x10);
        }
        if shape != 0 {
            let tolerance = x87_quiet_bits(e.mem.u32(AABB_HIGH_FILL));
            e.vcall(shape, 0x1c, &args![transform, tolerance, out]);
            return;
        }
        let (low_fill, high_fill, w_fill) = (
            e.mem.u32(AABB_LOW_FILL),
            e.mem.u32(AABB_HIGH_FILL),
            e.mem.u32(AABB_W_FILL),
        );
        for (lane, value) in [low_fill, low_fill, low_fill, w_fill]
            .into_iter()
            .enumerate()
        {
            e.mem.set_u32(low + 4 * lane as u32, value);
        }
        for (lane, value) in [high_fill, high_fill, high_fill, w_fill]
            .into_iter()
            .enumerate()
        {
            e.mem.set_u32(high + 4 * lane as u32, value);
        }
        e.call(TRANSFORM_POINT, &args![out, transform, low]);
        e.call(TRANSFORM_POINT, &args![out.addr() + 0x10, transform, high]);
    });
}

/// The type of the `bhkWorldObject` list items is one of: constraint, action (by RTTI
/// chain), or neither. Returns `(is constraint, is action)` after the exe's two separate
/// `GetRTTI` reads: the action test only runs when the constraint test failed.
fn item_kind(e: &mut Engine, item: u32) -> (bool, bool) {
    if is_kind_of(e, item, CONSTRAINT_RTTI) {
        (true, false)
    } else {
        (false, is_kind_of(e, item, ACTION_RTTI))
    }
}

// Translated from 00c8e770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRigidBody::RemoveActionConstraintsFromWorld` (Xbox PDB): for each item of the
/// action/constraint list that is a `bhkConstraint` (world from `00c89a00`) or a
/// `bhkAction` (world from `bhkWorldObject::GethkWorld`, `00c87a40`) and is in a world,
/// calls its virtual slot `0xA0` (`Remove`). The loop ends at a node with neither item
/// nor successor.
pub fn bhk_rigid_body_remove_action_constraints_from_world(
    e: &mut Engine,
    this: Ptr<BhkRigidBody>,
) {
    let mut node = this.at(BhkRigidBody::ActionConstraint);
    loop {
        if e.get(node, BSSimpleList::m_pkNext) == 0 && e.get(node, BSSimpleList::m_item) == 0 {
            return;
        }
        let item = e.get(node, BSSimpleList::m_item);
        if item != 0 {
            let in_world = match item_kind(e, item) {
                (true, _) => e.call(CONSTRAINT_GET_WORLD, &args![item]).u32() != 0,
                (false, true) => e.call(WORLD_OBJECT_GET_WORLD, &args![item]).u32() != 0,
                (false, false) => false,
            };
            if in_world {
                e.vcall(item, VSLOT_REMOVE, &args![]);
            }
        }
        node = Ptr::new(e.get(node, BSSimpleList::m_pkNext));
        if node.is_null() {
            return;
        }
    }
}

// Translated from 00c8e800 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c8e800 - /GS stack-cookie check (00ec408c) is not modeled
/// Unnamed (cdecl): accumulates the mass-weighted centre of mass of every rigid body in
/// the scene graph below `node` (children through `IsNode` and the children array at
/// `+0xA0`/`+0xA6` first, then the node's own collision object at `+0x1C` when its RTTI
/// chain contains the `bhkCollisionObject` type). For a collision object with a rigid body
/// (`+0x10`) `total += mass` (`GetDynamicMass`, `0062ab20`) and `centre = (1 - r) * centre
/// + r * body centre of mass in world` with `r = mass / total` (single precision stores,
/// x87 arithmetic, SSE blend).
pub fn fn_00c8e800(e: &mut Engine, node: Ptr, centre: Ptr, total: Ptr) {
    if node.is_null() {
        return;
    }
    let as_node = e.vcall(node.addr(), VSLOT_IS_NODE, &args![]).u32();
    if as_node != 0 {
        let mut index = 0u32;
        if e.mem.u16(as_node + 0xa6) != 0 {
            loop {
                let children = e.mem.u32(as_node + 0xa0);
                let child = e.mem.u32(children + 4 * index);
                fn_00c8e800(e, Ptr::new(child), centre, total);
                index += 1;
                if index >= u32::from(e.mem.u16(as_node + 0xa6)) {
                    break;
                }
            }
        }
    }
    let collision = e.mem.u32(node.addr() + 0x1c);
    if collision == 0 {
        return;
    }
    if !is_kind_of(e, collision, COLLISION_OBJECT_KEY) {
        return;
    }
    let body = e.mem.u32(collision + 0x10);
    if body == 0 {
        return;
    }
    let mass = e.call(GET_DYNAMIC_MASS, &args![body]).f32();
    e.with_stack(0x10, |e, world_centre| {
        e.vcall(
            body,
            VSLOT_GET_CENTER_OF_MASS_IN_WORLD,
            &args![world_centre],
        );
        let sum = (e.mem.f32(total.addr()) as f64 + mass as f64) as f32;
        e.mem.set_f32(total.addr(), sum);
        let ratio = (mass as f64 / sum as f64) as f32;
        let blend = splat(ratio);
        let old = load_vector(e, centre.addr());
        let new = load_vector(e, world_centre.addr());
        let mixed = add_vectors(
            multiply(subtract(load_vector(e, ONES_VECTOR), blend), old),
            multiply(new, blend),
        );
        store_vector(e, centre.addr(), mixed);
    });
}

// Translated from 00c8e940 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c8e940 - /GS stack-cookie check (00ec408c) is not modeled
/// Unnamed (cdecl): the mass-weighted centre of mass and total mass of every rigid body
/// below `root` (`fn_00c8e800` starting from a zero vector and a zero total), written to
/// `out_centre` (16 bytes) and `out_total`.
pub fn fn_00c8e940(e: &mut Engine, root: Ptr, out_centre: Ptr, out_total: Ptr) {
    let (centre, total) = e.with_stack(0x20, |e, scratch| {
        let centre = scratch.addr();
        let total = centre + 0x10;
        let zero = e.mem.u32(AABB_W_FILL);
        for lane in 0..4 {
            e.mem.set_u32(centre + 4 * lane, zero);
        }
        e.mem.set_f32(total, 0.0);
        fn_00c8e800(e, root, Ptr::new(centre), Ptr::new(total));
        (load_vector(e, centre), e.mem.u32(total))
    });
    store_vector(e, out_centre.addr(), centre);
    e.mem.set_u32(out_total.addr(), x87_quiet_bits(total));
}

// Translated from 00c8e9c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c8e9c0 - exception-unwinding frame (FS:[0]) is not modeled
/// `bhkRigidBody::~bhkRigidBody` (Xbox PDB, `this` in ECX): installs the class vtable, runs
/// `Kill`, decrements the `bhkRigidBody` object counter (`012681bc`), destroys the
/// action/constraint list (`004ee840`) and runs `bhkEntity::~bhkEntity`.
pub fn bhk_rigid_body_destruct(e: &mut Engine, this: Ptr<BhkRigidBody>) {
    e.mem.set_u32(this.addr(), BHK_RIGID_BODY_VTABLE);
    bhk_rigid_body_kill(e, this);
    let count = e.mem.u32(RIGID_BODY_COUNT);
    e.mem.set_u32(RIGID_BODY_COUNT, count.wrapping_sub(1));
    e.call(
        ACTION_LIST_DESTRUCT,
        &args![this.at(BhkRigidBody::ActionConstraint)],
    );
    e.call(ENTITY_DESTRUCT, &args![this]);
}

// Translated from 00c8ea30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRigidBody::LoadBinary` (Xbox PDB): `bhkEntity::LoadBinary`, then the cinfo
/// (virtual slot `0xB4`, byte-swapped when `stream + 0x1E5 != 1`). A maximum linear
/// velocity above 1000.0 becomes 1000.0; with zero cinfo mass and zero dynamic mass
/// (`0062ab20`) the motion type becomes 5 (fixed) unless the filter layer (low 7 bits of
/// the cinfo's first word) is 2 or 0x1C, and 4 (keyframed) when it is 0xE. After
/// `00a64670(stream)` it reads `iFlags` through the stream's input object
/// (`stream + 0x24C`, read callback at `+8`, position at `+4`).
pub fn bhk_rigid_body_load_binary(e: &mut Engine, this: Ptr<BhkRigidBody>, stream: u32) {
    e.call(ENTITY_LOAD_BINARY, &args![this, stream]);
    let cinfo = e
        .with_stack(4, |e, flag| {
            e.vcall(this.addr(), VSLOT_CREATE_CINFO, &args![flag])
        })
        .u32();
    if e.mem.u8(stream + 0x1e5) != 1 {
        bhk_rigid_body_cinfo_endian(e, Ptr::new(cinfo));
    }
    if e.mem.f32(cinfo + 0xc8) as f64 > e.global::<f64>(MAX_LINEAR_VELOCITY_FLOOR) {
        copy_float(e, cinfo + 0xc8, DEFAULT_MAX_LINEAR_VELOCITY);
    }
    if e.mem.f32(cinfo + 0xb0) == 0.0 && e.call(GET_DYNAMIC_MASS, &args![this]).f32() == 0.0 {
        let layer = e.mem.u32(cinfo) & 0x7f;
        if layer != 2 && layer != 0x1c {
            e.mem.set_u8(cinfo + 0xd4, 5);
        }
        if layer == 0xe {
            e.mem.set_u8(cinfo + 0xd4, 4);
        }
    }
    e.call(STREAM_READ_MULTIPLE_LINK_IDS, &args![stream]);
    let input = e.mem.u32(stream + 0x24c);
    let read = e.mem.u32(input + 8);
    let flags = this.addr() + BhkRigidBody::iFlags.off;
    let count = e.with_stack(4, |e, four| {
        e.mem.set_u32(four.addr(), 4);
        e.call(read, &args![input, flags, 4u32, four, 1u32]).u32()
    });
    let position = e.mem.u32(input + 4);
    e.mem.set_u32(input + 4, position.wrapping_add(count));
}

// Translated from 00c8eb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRigidBody::Remove` (Xbox PDB): clears bits 3 and 7 of `iFlags`
/// (`& 0xFFFFFF77`), removes the action/constraint items from the world and
/// tail-calls `bhkEntity::Remove`, whose result is returned unchanged.
pub fn bhk_rigid_body_remove(e: &mut Engine, this: Ptr<BhkRigidBody>) -> u32 {
    let flags = e.get(this, BhkRigidBody::iFlags);
    e.set(this, BhkRigidBody::iFlags, flags & 0xffff_ff77);
    bhk_rigid_body_remove_action_constraints_from_world(e, this);
    e.call(ENTITY_REMOVE, &args![this]).u32()
}

// Translated from 00c8eb20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c8eb20 - exception-unwinding frame (FS:[0]) is not modeled
/// `bhkRigidBody::UnFix` (Xbox PDB): when bit 2 of `iFlags` is set and the object's world
/// (`bhkWorldObject::GethkWorld`) has a non-zero word at `+0x350`, walks the
/// action/constraint list: each `bhkConstraint` that is in a world (`00c89a00`) is removed
/// (virtual slot `0xA0`) and its list node released (`00c88990` on the list head); one
/// that is not in a world is, when `add_to_world` is set, added to the world (slot `0x9C`
/// with the `+0x350` word). Then bit 2 is cleared. The allocation-scope word is 0x10
/// meanwhile. A list item that is not a `bhkConstraint` makes the exe loop forever (the
/// node is never advanced); this translation panics there instead.
pub fn bhk_rigid_body_un_fix(e: &mut Engine, this: Ptr<BhkRigidBody>, add_to_world: bool) {
    let (word, saved) = enter_scope(e);
    if (e.get(this, BhkRigidBody::iFlags) >> 2) & 1 != 0 {
        let world = e.call(WORLD_OBJECT_GET_WORLD, &args![this]).u32();
        let world_field = if world == 0 {
            0
        } else {
            e.mem.u32(world + 0x350)
        };
        if world_field != 0 {
            let mut node = this.at(BhkRigidBody::ActionConstraint);
            loop {
                let item = e.get(node, BSSimpleList::m_item);
                if item == 0 {
                    break;
                }
                if !is_kind_of(e, item, CONSTRAINT_RTTI) {
                    panic!("00c8eb20 loops forever on a list item that is not a bhkConstraint");
                }
                if e.call(CONSTRAINT_GET_WORLD, &args![item]).u32() != 0 {
                    e.vcall(item, VSLOT_REMOVE, &args![]);
                    e.call(
                        ACTION_LIST_REMOVE_HEAD,
                        &args![this.at(BhkRigidBody::ActionConstraint)],
                    );
                } else {
                    node = Ptr::new(e.get(node, BSSimpleList::m_pkNext));
                    if add_to_world {
                        e.vcall(item, VSLOT_ADD, &args![world_field]);
                    }
                }
                if node.is_null() {
                    break;
                }
            }
            let flags = e.get(this, BhkRigidBody::iFlags);
            e.set(this, BhkRigidBody::iFlags, flags & 0xffff_fffb);
        }
    }
    e.mem.set_u32(word, saved);
}

// Translated from 00c8ec60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`this` in ECX): empties the action/constraint list. Each item that is a
/// `bhkConstraint` (world from `00c89a00`) or a `bhkAction` (`bhkWorldObject::GethkWorld`,
/// `00c87a40`) and is in a world is removed from it (virtual slot `0xA0`); every item is
/// then dropped from the list with `00c88990` on the list head (which advances it).
pub fn fn_00c8ec60(e: &mut Engine, this: Ptr<BhkRigidBody>) {
    let head = this.at(BhkRigidBody::ActionConstraint);
    loop {
        let item = e.get(head, BSSimpleList::m_item);
        if item == 0 {
            return;
        }
        let in_world = match item_kind(e, item) {
            (true, _) => e.call(CONSTRAINT_GET_WORLD, &args![item]).u32() != 0,
            (false, true) => e.call(WORLD_OBJECT_GET_WORLD, &args![item]).u32() != 0,
            (false, false) => false,
        };
        if in_world {
            e.vcall(item, VSLOT_REMOVE, &args![]);
        }
        e.call(ACTION_LIST_REMOVE_HEAD, &args![head]);
    }
}

// Translated from 00c8ecf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl): `fn_00c8ec60` for the rigid body of the `bhkCollisionObject` (the
/// cast `00653290` with the key `012043f8`) at `node + 0x1C`, then the same for every child
/// node, recursively.
pub fn fn_00c8ecf0(e: &mut Engine, node: Ptr) {
    if node.is_null() {
        return;
    }
    let collision = e.mem.u32(node.addr() + 0x1c);
    if collision != 0 {
        let cast = e
            .call(
                CAST_TO_COLLISION_OBJECT,
                &args![collision, COLLISION_OBJECT_KEY],
            )
            .u32();
        if cast != 0 {
            let body = e.mem.u32(cast + 0x10);
            if body != 0 {
                fn_00c8ec60(e, Ptr::new(body));
            }
        }
    }
    let as_node = e.vcall(node.addr(), VSLOT_IS_NODE, &args![]).u32();
    if as_node != 0 && e.mem.u16(as_node + 0xa6) != 0 {
        let mut index = 0u32;
        loop {
            let array = e.mem.u32(as_node + 0xa0);
            let child = e.mem.u32(array + 4 * index);
            fn_00c8ecf0(e, Ptr::new(child));
            index += 1;
            if index >= u32::from(e.mem.u16(as_node + 0xa6)) {
                break;
            }
        }
    }
}

/// Adds one to the global object counter at `counter`.
fn count_object(e: &mut Engine, counter: u32) {
    let count = e.mem.u32(counter);
    e.mem.set_u32(counter, count.wrapping_add(1));
}

/// Allocates and constructs an empty `bhkRigidBody` (0x1C bytes from the game heap,
/// `bhkRefObject` base constructor `00c87c30`, then the three class constructors' vtables
/// and object counters in turn). Returns 0 when the allocation fails.
fn construct_rigid_body(e: &mut Engine) -> u32 {
    let object = e.call(HEAP_ALLOCATE, &args![0x1cu32]).u32();
    if object == 0 {
        return 0;
    }
    e.call(REF_OBJECT_CONSTRUCT, &args![object]);
    e.mem.set_u32(object, BHK_SERIALIZABLE_VTABLE);
    e.mem.set_u32(object + 0xc, 0);
    e.mem.set_u32(object + 0x10, 0);
    count_object(e, SERIALIZABLE_COUNT);
    e.mem.set_u32(object, BHK_ENTITY_VTABLE);
    count_object(e, ENTITY_COUNT);
    e.mem.set_u32(object, BHK_RIGID_BODY_VTABLE);
    e.mem.set_u32(object + 0x14, 0);
    e.mem.set_u32(object + 0x18, 0);
    count_object(e, RIGID_BODY_COUNT);
    object
}

// Translated from 00c8ed70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c8ed70 - exception-unwinding frame (FS:[0]) is not modeled
/// Unnamed (cdecl): creates an empty `bhkRigidBody` (see `construct_rigid_body`); 0 when
/// the allocation fails.
pub fn fn_00c8ed70(e: &mut Engine) -> u32 {
    construct_rigid_body(e)
}

// Translated from 00c8ee20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRigidBody::AddMotionType` (Xbox PDB, cdecl): appends the viewer string `MOTION` /
/// the name of the motion type (1 `DYNAMIC`, 2 `SPHERE`, 3 `BOX`, 4 `KEYFRAMED`, 5 `FIXED`,
/// 6 `THIN BOX`, anything else `INVALID`) to the array `strings`. The listed types are added
/// with `0096a610`; `INVALID` the way the other viewer strings are (grow when full, then
/// set at the old size).
pub fn bhk_rigid_body_add_motion_type(e: &mut Engine, strings: Ptr, motion: i32) {
    let value = match motion {
        1 => 0x010c_60d0,
        2 => 0x010c_60c0,
        3 => 0x010c_60bc,
        4 => 0x010c_60b0,
        5 => 0x010c_60a8,
        6 => 0x010c_609c,
        _ => 0x010c_6094,
    };
    let text = e
        .call(NI_GET_VIEWER_STRING_TEXT, &args![0x010c_60c8u32, value])
        .u32();
    if (1..=6).contains(&motion) {
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), text);
            e.call(VIEWER_STRINGS_ADD, &args![strings, slot]);
        });
    } else {
        push_viewer_string(e, strings.addr(), text);
    }
}

// Translated from 00c8ef80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c8ef80 - exception-unwinding frame (FS:[0]) is not modeled
/// `bhkRigidBody::AddActCon` (Xbox PDB): when `item` is a `bhkConstraint` or a `bhkAction`,
/// holds a reference to it (`InterlockedIncrement` of `item + 4`), adds it to the head of
/// the action/constraint list (`00c80a30`) and drops the temporary reference again,
/// destroying the item (virtual slot 4) if that was the last one. The allocation-scope
/// word is 0x10 meanwhile.
pub fn bhk_rigid_body_add_act_con(e: &mut Engine, this: Ptr<BhkRigidBody>, item: u32) {
    let (word, saved) = enter_scope(e);
    if item != 0 {
        let (constraint, action) = item_kind(e, item);
        if constraint || action {
            e.call(INTERLOCKED_INCREMENT, &args![item + 4]);
            e.with_stack(4, |e, held| {
                e.mem.set_u32(held.addr(), item);
                e.call(
                    ACTION_LIST_ADD_HEAD,
                    &args![this.at(BhkRigidBody::ActionConstraint), held],
                );
            });
            if e.call(INTERLOCKED_DECREMENT, &args![item + 4]).u32() == 0 {
                e.vcall(item, 4, &args![]);
            }
        }
    }
    e.mem.set_u32(word, saved);
}

// Translated from 00c8f080 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c8f080 - exception-unwinding frame (FS:[0]) is not modeled
/// `bhkRigidBody::Fix` (Xbox PDB, `this` in ECX): when bit 2 of `iFlags` is clear and the
/// world has a non-zero word at `+0x350`, replaces every `bhkConstraint` of the list by
/// `bhkFixedConstraint::FixConstraint` (`00cc81c0`, with the float 1.0, or the float at
/// `01017870` when the body's collision layer is 8 or 0x1D and its group bits are 0x11, 0x15
/// or 0x16): the new constraint is added to the list (`AddActCon`), the old one removed
/// (slot `0xA0`) and the new one added to the world (slot `0x9C`). Then bit 2 is set. The
/// allocation-scope word is 0x10 meanwhile.
pub fn bhk_rigid_body_fix(e: &mut Engine, this: Ptr<BhkRigidBody>) {
    let (word, saved) = enter_scope(e);
    if (e.get(this, BhkRigidBody::iFlags) >> 2) & 1 == 0 {
        let world = e.call(WORLD_OBJECT_GET_WORLD, &args![this]).u32();
        let world_field = if world == 0 {
            0
        } else {
            e.mem.u32(world + 0x350)
        };
        if world_field != 0 {
            let mut node = this.at(BhkRigidBody::ActionConstraint);
            loop {
                let item = e.get(node, BSSimpleList::m_item);
                if item == 0 {
                    break;
                }
                let next = e.get(node, BSSimpleList::m_pkNext);
                if is_kind_of(e, item, CONSTRAINT_RTTI) {
                    let body = e.get(this, BhkRigidBody::phkObject);
                    let filter = if body.is_null() {
                        0
                    } else {
                        e.mem.u32(body.addr() + 0x2c)
                    };
                    let mut strength = 1.0f32.to_bits();
                    let layer = filter & 0x7f;
                    if layer == 8 || layer == 0x1d {
                        let group = (filter >> 8) & 0x1f;
                        if group == 0x11 || group.wrapping_sub(0x15) <= 1 {
                            strength = x87_quiet_bits(e.mem.u32(FIXED_CONSTRAINT_STRENGTH));
                        }
                    }
                    let fixed = e.call(FIX_CONSTRAINT, &args![item, strength]).u32();
                    if fixed != 0 {
                        bhk_rigid_body_add_act_con(e, this, fixed);
                        e.vcall(item, VSLOT_REMOVE, &args![]);
                        e.vcall(fixed, VSLOT_ADD, &args![world_field]);
                    }
                }
                node = Ptr::new(next);
                if node.is_null() {
                    break;
                }
            }
            let flags = e.get(this, BhkRigidBody::iFlags);
            e.set(this, BhkRigidBody::iFlags, flags | 4);
        }
    }
    e.mem.set_u32(word, saved);
}

// Translated from 00c8f210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRigidBody::SetFixed` (Xbox PDB, cdecl): after `00a68bf0(node)`, `bhkRigidBody::Fix`
/// (when `fix`) or `UnFix` (low byte of `extra` as its flag) on the rigid body of the
/// `bhkCollisionObject` at `node + 0x1C` (cast `00653290`, key `012043f8`), then the same
/// for every child node, recursively.
pub fn bhk_rigid_body_set_fixed(e: &mut Engine, node: Ptr, fix: bool, extra: u32) {
    if node.is_null() {
        return;
    }
    e.call(NODE_PREPARE, &args![node]);
    let collision = e.mem.u32(node.addr() + 0x1c);
    if collision != 0 {
        let cast = e
            .call(
                CAST_TO_COLLISION_OBJECT,
                &args![collision, COLLISION_OBJECT_KEY],
            )
            .u32();
        if cast != 0 {
            let body = e.mem.u32(cast + 0x10);
            if body != 0 {
                if fix {
                    bhk_rigid_body_fix(e, Ptr::new(body));
                } else {
                    bhk_rigid_body_un_fix(e, Ptr::new(body), extra & 0xff != 0);
                }
            }
        }
    }
    let as_node = e.vcall(node.addr(), VSLOT_IS_NODE, &args![]).u32();
    if as_node != 0 && e.mem.u16(as_node + 0xa6) != 0 {
        let mut index = 0u32;
        loop {
            let array = e.mem.u32(as_node + 0xa0);
            let child = e.mem.u32(array + 4 * index);
            bhk_rigid_body_set_fixed(e, Ptr::new(child), fix, extra);
            index += 1;
            if index >= u32::from(e.mem.u16(as_node + 0xa6)) {
                break;
            }
        }
    }
}

// Translated from 00c8f2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRigidBody::CopyMembers` (Xbox PDB): copies this object into `destination`.
/// `arg` is the clone context whose float at `+0x10` is the scale. Takes the cinfo (slot
/// `0xB4`); with `iFlags` bit 9 its motion type becomes 1; a scale other than 1.0 scales it
/// (`bhkRigidBodyCinfo::Scale`); the Havok body's local frame (`+0x210`) goes to the cinfo
/// (`+0x28`); `bhkWorldObject::CopyMembers`; the destination's `iFlags` become
/// `(destination | this) & 0xFFFFFC07`. When the body's filter word has bit 13 or the scale
/// is not 1.0, bit 13 is set on the destination body's word and `UpdateCFilter` (slot
/// `0xC4`) runs. Finally every item of the list is cloned (virtual slot `0x48` with `arg`)
/// and added to the destination.
pub fn bhk_rigid_body_copy_members(
    e: &mut Engine,
    this: Ptr<BhkRigidBody>,
    destination: Ptr<BhkRigidBody>,
    arg: Ptr,
) {
    let cinfo = e
        .with_stack(4, |e, flag| {
            e.vcall(this.addr(), VSLOT_CREATE_CINFO, &args![flag])
        })
        .u32();
    if (e.get(this, BhkRigidBody::iFlags) >> 9) & 1 != 0 {
        e.mem.set_u8(cinfo + 0xd4, 1);
    }
    let scale = f32::from_bits(x87_quiet_bits(e.mem.u32(arg.addr() + 0x10)));
    if scale != 1.0 {
        bhk_rigid_body_cinfo_scale(e, Ptr::new(cinfo), scale);
    }
    let body = e.get(this, BhkRigidBody::phkObject);
    let frame_link = e.mem.u32(body.addr() + 0x210);
    e.mem.set_u32(cinfo + 0x28, frame_link);
    e.call(WORLD_OBJECT_COPY_MEMBERS, &args![this, destination, arg]);
    let merged = (e.get(destination, BhkRigidBody::iFlags) | e.get(this, BhkRigidBody::iFlags))
        & 0xffff_fc07;
    e.set(destination, BhkRigidBody::iFlags, merged);
    let body = hk_body(e, this);
    let filter = if body == 0 { 0 } else { e.mem.u32(body + 0x2c) };
    if (filter >> 13) & 1 != 0 || 1.0f32 != e.mem.f32(arg.addr() + 0x10) {
        let target = hk_body(e, destination);
        let filter = if target == 0 {
            0
        } else {
            e.mem.u32(target + 0x2c)
        };
        if target != 0 {
            lock(e, destination);
            lock(e, destination);
            e.mem.set_u32(target + 0x2c, filter | 0x2000);
        }
        e.vcall(destination.addr(), VSLOT_UPDATE_COLLISION_FILTER, &args![]);
    }
    let mut node = this.at(BhkRigidBody::ActionConstraint);
    loop {
        let item = e.get(node, BSSimpleList::m_item);
        if item == 0 {
            break;
        }
        let clone = e.vcall(item, VSLOT_CREATE_CLONE, &args![arg]).u32();
        bhk_rigid_body_add_act_con(e, destination, clone);
        node = Ptr::new(e.get(node, BSSimpleList::m_pkNext));
        if node.is_null() {
            break;
        }
    }
}

// Translated from 00c8f3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRigidBody::LinkObject` (Xbox PDB): `bhkWorldObject::LinkObject`, then for each of
/// the stream's link ids (`NiStream::GetNumberOfLinkIDs`) the object it names
/// (`NiStream::GetObjectFromLinkID`) is added with `AddActCon` when it exists.
pub fn bhk_rigid_body_link_object(e: &mut Engine, this: Ptr<BhkRigidBody>, stream: u32) {
    e.call(WORLD_OBJECT_LINK_OBJECT, &args![this, stream]);
    let mut remaining = e.call(STREAM_GET_NUMBER_OF_LINK_IDS, &args![stream]).u32();
    if remaining != 0 {
        loop {
            let linked = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
            if linked != 0 {
                bhk_rigid_body_add_act_con(e, this, linked);
            }
            remaining -= 1;
            if remaining == 0 {
                break;
            }
        }
    }
}

// Translated from 00c8f400 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c8f400 - exception-unwinding frame (FS:[0]) is not modeled
/// `bhkRigidBody::CreateClone` (Xbox PDB): with the allocation-scope word at 0x10 and the
/// clone lock (`00c876f0`, `00c87700`) held, calls `SetHavokAssertState(0)` (virtual slot
/// `0xFC`), creates an empty rigid body, runs `CopyMembers(clone, arg)` and
/// `SetHavokAssertState(1)`. Returns the clone (0 when the allocation failed).
pub fn bhk_rigid_body_create_clone(e: &mut Engine, this: Ptr<BhkRigidBody>, arg: Ptr) -> u32 {
    let (word, saved) = enter_scope(e);
    e.call(CLONE_LOCK, &args![]);
    e.vcall(this.addr(), VSLOT_SET_HAVOK_ASSERT_STATE, &args![0u32]);
    let clone = construct_rigid_body(e);
    bhk_rigid_body_copy_members(e, this, Ptr::new(clone), arg);
    e.vcall(this.addr(), VSLOT_SET_HAVOK_ASSERT_STATE, &args![1u32]);
    e.call(CLONE_UNLOCK, &args![]);
    e.mem.set_u32(word, saved);
    clone
}

// Translated from 00c8f510 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c8f510 - exception-unwinding frame (FS:[0]) is not modeled
/// `bhkRigidBodyCinfo::bhkRigidBodyCinfo` (Xbox PDB): the property array (`+0xC`, empty,
/// capacity word `0x80000000`), broadphase type 1 (`+8`), filter and shape 0, response
/// 1 (`+0x18`) and the contact callback delay `0xFFFF` (`+0x1A`); the Havok half's
/// constructor (`00cf5ea0` on `this + 0x20`); maximum linear velocity 10000.0 (`01022958`),
/// quality -1 and maximum angular velocity 10 pi (`010c60d8`). Returns `this`.
pub fn bhk_rigid_body_cinfo_construct(
    e: &mut Engine,
    this: Ptr<BhkRigidBodyCinfo>,
) -> Ptr<BhkRigidBodyCinfo> {
    let base = this.addr();
    e.mem.set_u32(base + 0xc, 0);
    e.mem.set_u32(base + 0x10, 0);
    e.mem.set_u32(base + 0x14, 0x8000_0000);
    e.mem.set_u8(base + 8, 1);
    e.mem.set_u32(base, 0);
    e.mem.set_u32(base + 4, 0);
    e.mem.set_u8(base + 0x18, 1);
    e.mem.set_u16(base + 0x1a, 0xffff);
    e.call(RIGID_BODY_CINFO_HK_CONSTRUCT, &args![base + 0x20]);
    copy_float(e, base + 0xc8, DEFAULT_CINFO_MAX_LINEAR_VELOCITY);
    e.mem.set_u8(base + 0xd7, 0xff);
    copy_float(e, base + 0xcc, DEFAULT_CINFO_MAX_ANGULAR_VELOCITY);
    this
}

// Translated from 00c8f5a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c8f5a0 - exception-unwinding frame (FS:[0]) is not modeled
/// `bhkRigidBody::CreateCinfo` (Xbox PDB): `*created` is set to whether `pInfo` (`+0xC`)
/// was empty. If so, a 0xF0-byte block is taken from the memory manager (`011f6238`,
/// `00aa3e40`), aligned to 16 bytes (the shift is stored in the byte before the cinfo),
/// constructed (`bhkRigidBodyCinfo` constructor) and stored as `pInfo`, and with a Havok
/// body filled by `CopyCinfo`. Returns `pInfo`. The allocation-scope word is 0x10
/// meanwhile.
pub fn bhk_rigid_body_create_cinfo(
    e: &mut Engine,
    this: Ptr<BhkRigidBody>,
    created: Ptr,
) -> Ptr<BhkRigidBodyCinfo> {
    let (word, saved) = enter_scope(e);
    let missing = e.get(this, BhkRigidBody::pInfo).is_null();
    e.mem.set_u8(created.addr(), u8::from(missing));
    if missing {
        let block = e
            .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, 0xf0u32])
            .u32();
        let shift = 0x10 - (block & 0xf) as u8;
        let cinfo = block + u32::from(shift);
        e.mem.set_u8(cinfo - 1, shift);
        let cinfo = bhk_rigid_body_cinfo_construct(e, Ptr::new(cinfo));
        e.set(this, BhkRigidBody::pInfo, cinfo.cast());
        if !e.get(this, BhkRigidBody::phkObject).is_null() {
            bhk_rigid_body_copy_cinfo(e, this, cinfo);
        }
    }
    let info = e.get(this, BhkRigidBody::pInfo);
    e.mem.set_u32(word, saved);
    info.cast()
}

// Translated from 00c8f670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`this` is a cinfo, RET 4): frees the property array (`+0xC` data, `+0x10` size,
/// `+0x14` capacity word; storage is freed through the Havok allocator unless the high
/// bit of the capacity word says it is not owned), resets it to empty, and when bit 0 of
/// `flags` is set also frees the cinfo's own aligned block (the shift is in the byte before
/// it). Returns `this`; the scalar deleting destructor of `bhkRigidBodyCinfo` by its shape.
pub fn fn_00c8f670(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let allocator = memory_allocator(e);
    let base = this.addr();
    let capacity = e.mem.u32(base + 0x14);
    e.mem.set_u32(base + 0x10, 0);
    if (capacity as i32) >= 0 {
        let bytes = (capacity & 0x3fff_ffff) * 8;
        let data = e.mem.u32(base + 0xc);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(base + 0xc, 0);
    e.mem.set_u32(base + 0x14, 0x8000_0000);
    if flags & 1 != 0 && base != 0 {
        let shift = u32::from(e.mem.u8(base - 1));
        e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER, base - shift]);
    }
    this
}

/// Frees the storage of the property array of the cinfo at `cinfo` and sets its size to
/// 0 (the way `GetViewerStrings` and `LoadCInfo` clean their temporary cinfos).
fn release_property_array(e: &mut Engine, cinfo: u32) {
    let allocator = memory_allocator(e);
    let capacity = e.mem.u32(cinfo + 0x14);
    e.mem.set_u32(cinfo + 0x10, 0);
    if (capacity as i32) >= 0 {
        let bytes = (capacity & 0x3fff_ffff) * 8;
        let data = e.mem.u32(cinfo + 0xc);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
}

// Translated from 00c8f6e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c8f6e0 - exception-unwinding frame (FS:[0]) and /GS stack-cookie
// check (00ec408c) are not modeled
/// `bhkRigidBody::LoadCInfo` (Xbox PDB): returns `QCinfoSize` (virtual slot `0xA8`). The cinfo
/// is made through slot `0xB4`. For stream versions (`stream + 4`) from 0x23 on, the
/// `bhkSerializable` loader (`00c87710`) reads it. For older streams the 0xE0-byte old-format
/// cinfo is read from the stream's input object (`stream + 0x24C`, read callback at `+8`),
/// converted into the cinfo by `fn_00c8d510`, its property array released, and a Havok
/// error message (`01268360`, slot `0xC`, id `0x234F2250`) reports the old file (formatted
/// from `stream + 0xE0`).
pub fn bhk_rigid_body_load_cinfo(e: &mut Engine, this: Ptr<BhkRigidBody>, stream: u32) -> u32 {
    let size = e.vcall(this.addr(), VSLOT_QUERY_CINFO_SIZE, &args![]).u32();
    let cinfo = e
        .with_stack(4, |e, flag| {
            e.vcall(this.addr(), VSLOT_CREATE_CINFO, &args![flag])
        })
        .u32();
    if e.mem.u32(stream + 4) >= 0x23 {
        e.call(SERIALIZABLE_LOAD_CINFO, &args![this, stream]);
        return size;
    }
    e.with_stack(0x1e0, |e, area| {
        let old = area.addr();
        let text = old + 0xe0;
        e.mem.set_u8(old + 8, 1);
        e.mem.set_u8(old + 0x18, 1);
        e.mem.set_u32(old + 0xc, 0);
        e.mem.set_u32(old + 0x10, 0);
        e.mem.set_u32(old + 0x14, 0x8000_0000);
        e.mem.set_u32(old, 0);
        e.mem.set_u32(old + 4, 0);
        e.mem.set_u16(old + 0x1a, 0xffff);
        let input = e.mem.u32(stream + 0x24c);
        let read = e.mem.u32(input + 8);
        let count = e.call(read, &args![input, old, 0xe0u32, 0u32, 0u32]).u32();
        let position = e.mem.u32(input + 4);
        e.mem.set_u32(input + 4, position.wrapping_add(count));
        fn_00c8d510(e, Ptr::new(cinfo), Ptr::new(old));
        release_property_array(e, old);
        e.call(
            FORMAT_TEXT,
            &args![text, 0x100u32, OLD_CINFO_MESSAGE, stream + 0xe0],
        );
        let reporter = e.mem.u32(HAVOK_ERROR_OBJECT);
        e.vcall(
            reporter,
            HAVOK_ERROR_SLOT_MESSAGE,
            &args![0u32, 0x234f_2250u32, text, HAVOK_SOURCE_FILE, 0x220u32],
        );
    });
    size
}

/// Sets the element at the old size of the viewer-string array the way the exe's inline
/// copy in `GetViewerStrings` does: grows when full, then updates the `u16` size (`+0xA`)
/// and the count of non-null strings (`+0xC`) before storing `text`.
fn set_viewer_string_counted(e: &mut Engine, strings: u32, text: u32) {
    let index = u32::from(e.mem.u16(strings + 0xa));
    if index >= u32::from(e.mem.u16(strings + 8)) {
        let step = u32::from(e.mem.u16(strings + 0xe));
        e.call(STRINGS_GROW, &args![strings, step + index]);
    }
    let size = u32::from(e.mem.u16(strings + 0xa));
    if index >= size {
        e.mem.set_u16(strings + 0xa, (index + 1) as u16);
        if text != 0 {
            let count = e.mem.u16(strings + 0xc);
            e.mem.set_u16(strings + 0xc, count.wrapping_add(1));
        }
    } else {
        let slot = e.mem.u32(strings + 4) + 4 * index;
        let old = e.mem.u32(slot);
        let count = e.mem.u16(strings + 0xc);
        if text != 0 {
            if old == 0 {
                e.mem.set_u16(strings + 0xc, count.wrapping_add(1));
            }
        } else if old != 0 {
            e.mem.set_u16(strings + 0xc, count.wrapping_add(0xffff));
        }
    }
    let base = e.mem.u32(strings + 4);
    e.mem.set_u32(base + 4 * index, text);
}

// Translated from 00c8f870 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c8f870 - exception-unwinding frame (FS:[0]) and /GS stack-cookie
// check (00ec408c) are not modeled
/// `bhkRigidBody::GetViewerStrings` (Xbox PDB): with the allocation-scope word at 0x10,
/// appends to `strings` the `bhkEntity` strings, the class string (`NiGetViewerString` of
/// the pointer in `012681c0`), then describes a temporary cinfo made by the cinfo
/// constructor and `CopyCinfo`: `Position`, `Rotation` (as `w, x, y, z`), `COM`, `MASS`,
/// `LINDAMP`, `ANGDAMP`, `FRICTION`, `REST`, the motion type (`AddMotionType`), `MAXLINVEL`,
/// `MAXANGVEL`, `LinVel`, `AngVel`, `InertiaTensor` (the diagonal), `PENDEPTH`,
/// `QUALITYTYPE` (name string), `ACTCONCOUNT` and one `ACTCON` string `"<RTTI name>: 0x<item>"`
/// per list item (stored without growing the count of valid strings twice). The cinfo's
/// property array is released at the end.
pub fn bhk_rigid_body_get_viewer_strings(e: &mut Engine, this: Ptr<BhkRigidBody>, strings: Ptr) {
    let (word, saved) = enter_scope(e);
    let array = strings.addr();
    e.call(ENTITY_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.global::<u32>(RIGID_BODY_CLASS_NAME);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    push_viewer_string(e, array, text);
    e.with_stack(0x200, |e, area| {
        let cinfo = area.addr();
        let point = cinfo + 0xe0;
        let quaternion = cinfo + 0xf0;
        let diagonal = cinfo + 0x100;
        let buffer = cinfo + 0x110;
        bhk_rigid_body_cinfo_construct(e, Ptr::new(cinfo));
        bhk_rigid_body_copy_cinfo(e, this, Ptr::new(cinfo));
        e.call(NI_POINT3_FROM_VECTOR, &args![point, cinfo + 0x30]);
        let text = e
            .call(NI_POINT3_GET_VIEWER_STRING, &args![point, POSITION_NAME])
            .u32();
        push_viewer_string(e, array, text);
        for (lane, from) in [0x4cu32, 0x40, 0x44, 0x48].into_iter().enumerate() {
            copy_float(e, quaternion + 4 * lane as u32, cinfo + from);
        }
        let text = e
            .call(
                NI_QUATERNION_GET_VIEWER_STRING,
                &args![quaternion, ROTATION_NAME],
            )
            .u32();
        push_viewer_string(e, array, text);
        e.call(NI_POINT3_FROM_VECTOR, &args![point, cinfo + 0xa0]);
        let text = e
            .call(
                NI_POINT3_GET_VIEWER_STRING,
                &args![point, CENTER_OF_MASS_NAME],
            )
            .u32();
        push_viewer_string(e, array, text);
        let float_string = |e: &mut Engine, name: u32, offset: u32| {
            let bits = x87_quiet_bits(e.mem.u32(cinfo + offset));
            e.call(NI_GET_VIEWER_STRING_FLOAT, &args![name, bits]).u32()
        };
        for (name, offset) in [
            (MASS_NAME, 0xb0),
            (LINEAR_DAMPING_NAME, 0xb4),
            (ANGULAR_DAMPING_NAME, 0xb8),
            (FRICTION_NAME, 0xc0),
            (RESTITUTION_NAME, 0xc4),
        ] {
            let text = float_string(e, name, offset);
            push_viewer_string(e, array, text);
        }
        let motion = i32::from(e.mem.i8(cinfo + 0xd4));
        bhk_rigid_body_add_motion_type(e, strings, motion);
        for (name, offset) in [
            (MAX_LINEAR_VELOCITY_NAME, 0xc8),
            (MAX_ANGULAR_VELOCITY_NAME, 0xcc),
        ] {
            let text = float_string(e, name, offset);
            push_viewer_string(e, array, text);
        }
        for (name, offset) in [
            (LINEAR_VELOCITY_NAME, 0x50u32),
            (ANGULAR_VELOCITY_NAME, 0x60),
        ] {
            e.call(NI_POINT3_FROM_VECTOR, &args![point, cinfo + offset]);
            let text = e
                .call(NI_POINT3_GET_VIEWER_STRING, &args![point, name])
                .u32();
            push_viewer_string(e, array, text);
        }
        for (lane, from) in [0x70u32, 0x84, 0x98].into_iter().enumerate() {
            copy_float(e, diagonal + 4 * lane as u32, cinfo + from);
        }
        let text = e
            .call(
                NI_POINT3_GET_VIEWER_STRING,
                &args![diagonal, INERTIA_TENSOR_NAME],
            )
            .u32();
        push_viewer_string(e, array, text);
        let text = float_string(e, PENETRATION_DEPTH_NAME, 0xd0);
        push_viewer_string(e, array, text);
        let quality = bhk_rigid_body_cinfo_get_quality_type_string(e, Ptr::new(cinfo));
        let text = e
            .call(
                NI_GET_VIEWER_STRING_TEXT,
                &args![QUALITY_TYPE_NAME, quality],
            )
            .u32();
        push_viewer_string(e, array, text);
        let count = fn_00c8d0d0(e, this);
        let text = e
            .call(NI_GET_VIEWER_STRING_INT, &args![ACTION_COUNT_NAME, count])
            .u32();
        push_viewer_string(e, array, text);
        let mut node = this.at(BhkRigidBody::ActionConstraint);
        loop {
            let item = e.get(node, BSSimpleList::m_item);
            if item == 0 {
                break;
            }
            let rtti = e.vcall(item, VSLOT_GET_RTTI, &args![]).u32();
            let name = e.mem.u32(rtti);
            e.call(
                FORMAT_TEXT,
                &args![buffer, 0x80u32, ITEM_FORMAT, name, item],
            );
            let text = e
                .call(NI_GET_VIEWER_STRING_TEXT, &args![ACTION_ITEM_NAME, buffer])
                .u32();
            set_viewer_string_counted(e, array, text);
            node = Ptr::new(e.get(node, BSSimpleList::m_pkNext));
            if node.is_null() {
                break;
            }
        }
        release_property_array(e, cinfo);
    });
    e.mem.set_u32(word, saved);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c8_ccc0,
            bhk_rigid_body_is_equal(Ptr<BhkRigidBody>, Ptr<BhkRigidBody>) -> bool
        ),
        entry!(0x00c8_cce0, fn_00c8cce0(Ptr) -> bool),
        entry!(
            0x00c8_cd20,
            bhk_rigid_body_get_max_angular_velocity(Ptr<BhkRigidBody>) -> f32
        ),
        entry!(0x00c8_cdb0, fn_00c8cdb0(Ptr)),
        entry!(
            0x00c8_cdf0,
            bhk_rigid_body_cinfo_scale(Ptr<BhkRigidBodyCinfo>, f32)
        ),
        entry!(
            0x00c8_ce70,
            bhk_rigid_body_cinfo_endian(Ptr<BhkRigidBodyCinfo>)
        ),
        entry!(
            0x00c8_cfd0,
            bhk_rigid_body_copy_cinfo(Ptr<BhkRigidBody>, Ptr<BhkRigidBodyCinfo>)
        ),
        entry!(
            0x00c8_d090,
            bhk_rigid_body_register_streamables(Ptr<BhkRigidBody>, u32) -> bool
        ),
        entry!(0x00c8_d0d0, fn_00c8d0d0(Ptr<BhkRigidBody>) -> u32),
        entry!(
            0x00c8_d0f0,
            bhk_rigid_body_get_position_ov2(Ptr<BhkRigidBody>, Ptr) -> Ptr
        ),
        entry!(
            0x00c8_d130,
            bhk_rigid_body_get_rotation_ov2(Ptr<BhkRigidBody>, Ptr) -> Ptr
        ),
        entry!(
            0x00c8_d170,
            bhk_rigid_body_set_position(Ptr<BhkRigidBody>, Ptr)
        ),
        entry!(
            0x00c8_d1a0,
            bhk_rigid_body_set_rotation(Ptr<BhkRigidBody>, Ptr)
        ),
        entry!(
            0x00c8_d1d0,
            bhk_rigid_body_set_pos_rot(Ptr<BhkRigidBody>, Ptr, Ptr)
        ),
        entry!(
            0x00c8_d200,
            bhk_rigid_body_get_center_of_mass_local(Ptr<BhkRigidBody>, Ptr) -> Ptr
        ),
        entry!(
            0x00c8_d240,
            bhk_rigid_body_get_center_of_mass_in_world(Ptr<BhkRigidBody>, Ptr) -> Ptr
        ),
        entry!(
            0x00c8_d280,
            bhk_rigid_body_get_transform(Ptr<BhkRigidBody>, Ptr) -> Ptr
        ),
        entry!(
            0x00c8_d2f0,
            bhk_rigid_body_create_display_geometry(Ptr<BhkRigidBody>, u32) -> u32
        ),
        entry!(
            0x00c8_d390,
            bhk_rigid_body_get_aabb_local(Ptr<BhkRigidBody>, Ptr)
        ),
        entry!(0x00c8_d440, bhk_rigid_body_set_havok_assert_state(Ptr, u32)),
        entry!(
            0x00c8_d4a0,
            bhk_rigid_body_cinfo_get_quality_type_string(Ptr<BhkRigidBodyCinfo>) -> u32
        ),
        entry!(0x00c8_d510, fn_00c8d510(Ptr<BhkRigidBodyCinfo>, Ptr)),
        entry!(
            0x00c8_d650,
            bhk_rigid_body_init(Ptr<BhkRigidBody>, Ptr<BhkRigidBodyCinfo>)
        ),
        entry!(
            0x00c8_d960,
            bhk_rigid_body_save_binary(Ptr<BhkRigidBody>, u32)
        ),
        entry!(0x00c8_da40, bhk_rigid_body_kill(Ptr<BhkRigidBody>)),
        entry!(
            0x00c8_dac0,
            bhk_rigid_body_add_act_con_to_world(Ptr<BhkRigidBody>, u32)
        ),
        entry!(
            0x00c8_db50,
            bhk_rigid_body_add(Ptr<BhkRigidBody>, u32) -> bool
        ),
        entry!(
            0x00c8_dcb0,
            bhk_rigid_body_force_add(Ptr<BhkRigidBody>, u32) -> bool
        ),
        entry!(
            0x00c8_dd10,
            bhk_rigid_body_set_motion_type(Ptr<BhkRigidBody>, i32) -> bool
        ),
        entry!(
            0x00c8_de20,
            fn_00c8de20(Ptr<BhkRigidBody>, Ptr, Ptr, f32, f32)
        ),
        entry!(
            0x00c8_e160,
            bhk_rigid_body_apply_hard_keyframe(Ptr<BhkRigidBody>, Ptr, Ptr, Ptr)
        ),
        entry!(0x00c8_e670, bhk_rigid_body_get_aabb(Ptr<BhkRigidBody>, Ptr)),
        entry!(
            0x00c8_e770,
            bhk_rigid_body_remove_action_constraints_from_world(Ptr<BhkRigidBody>)
        ),
        entry!(0x00c8_e800, fn_00c8e800(Ptr, Ptr, Ptr)),
        entry!(0x00c8_e940, fn_00c8e940(Ptr, Ptr, Ptr)),
        entry!(0x00c8_e9c0, bhk_rigid_body_destruct(Ptr<BhkRigidBody>)),
        entry!(
            0x00c8_ea30,
            bhk_rigid_body_load_binary(Ptr<BhkRigidBody>, u32)
        ),
        entry!(0x00c8_eb00, bhk_rigid_body_remove(Ptr<BhkRigidBody>) -> u32),
        entry!(0x00c8_eb20, bhk_rigid_body_un_fix(Ptr<BhkRigidBody>, bool)),
        entry!(0x00c8_ec60, fn_00c8ec60(Ptr<BhkRigidBody>)),
        entry!(0x00c8_ecf0, fn_00c8ecf0(Ptr)),
        entry!(0x00c8_ed70, fn_00c8ed70() -> u32),
        entry!(0x00c8_ee20, bhk_rigid_body_add_motion_type(Ptr, i32)),
        entry!(
            0x00c8_ef80,
            bhk_rigid_body_add_act_con(Ptr<BhkRigidBody>, u32)
        ),
        entry!(0x00c8_f080, bhk_rigid_body_fix(Ptr<BhkRigidBody>)),
        entry!(0x00c8_f210, bhk_rigid_body_set_fixed(Ptr, bool, u32)),
        entry!(
            0x00c8_f2a0,
            bhk_rigid_body_copy_members(Ptr<BhkRigidBody>, Ptr<BhkRigidBody>, Ptr)
        ),
        entry!(
            0x00c8_f3c0,
            bhk_rigid_body_link_object(Ptr<BhkRigidBody>, u32)
        ),
        entry!(
            0x00c8_f400,
            bhk_rigid_body_create_clone(Ptr<BhkRigidBody>, Ptr) -> u32
        ),
        entry!(
            0x00c8_f510,
            bhk_rigid_body_cinfo_construct(Ptr<BhkRigidBodyCinfo>) -> Ptr<BhkRigidBodyCinfo>
        ),
        entry!(
            0x00c8_f5a0,
            bhk_rigid_body_create_cinfo(Ptr<BhkRigidBody>, Ptr) -> Ptr<BhkRigidBodyCinfo>
        ),
        entry!(0x00c8_f670, fn_00c8f670(Ptr, u32) -> Ptr),
        entry!(
            0x00c8_f6e0,
            bhk_rigid_body_load_cinfo(Ptr<BhkRigidBody>, u32) -> u32
        ),
        entry!(
            0x00c8_f870,
            bhk_rigid_body_get_viewer_strings(Ptr<BhkRigidBody>, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    /// A shared log of the vectors a double saw.
    type VectorLog = Rc<RefCell<Vec<[f32; 4]>>>;

    /// `GetRTTI` double shared by the list items built by `item_of`.
    const GET_RTTI_DOUBLE: u32 = 0x0900_0008;
    /// Allocator doubles used by several tests.
    const ALLOCATE_DOUBLE: u32 = 0x0900_0010;
    const FREE_BYTES_DOUBLE: u32 = 0x0900_0013;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn float_ret(value: f32) -> Ret {
        Ret {
            st0: f64::from(value),
            ..Ret::default()
        }
    }

    /// The addresses of the calls recorded so far, in order.
    fn order(e: &Engine) -> Vec<u32> {
        e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect()
    }

    /// The argument words of every logged call to `address`.
    fn args_of(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    /// An object with a fresh vtable holding the given `(byte offset, target)` slots.
    fn object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let table = e.mem.alloc(0x100);
        for (offset, target) in slots {
            e.mem.set_u32(table + offset, *target);
        }
        let object = e.mem.alloc(0x400);
        e.mem.set_u32(object, table);
        object
    }

    /// A `bhkRigidBody` with the given vtable slots and a 0x400-byte Havok body.
    fn rigid_body(e: &mut Engine, slots: &[(u32, u32)]) -> (Ptr<BhkRigidBody>, u32) {
        let body = Ptr::new(object(e, slots));
        let hk = e.mem.alloc(0x400);
        e.set(body, BhkRigidBody::phkObject, Ptr::new(hk));
        (body, hk)
    }

    fn set_vector(e: &mut Engine, address: u32, vector: [f32; 4]) {
        for (lane, value) in vector.into_iter().enumerate() {
            e.mem.set_f32(address + 4 * lane as u32, value);
        }
    }

    fn vector(e: &Engine, address: u32) -> [f32; 4] {
        [0, 4, 8, 12].map(|lane| e.mem.f32(address + lane))
    }

    /// Stores `items` in the body's action/constraint list.
    fn set_items(e: &mut Engine, body: Ptr<BhkRigidBody>, items: &[u32]) {
        let mut node = body.at(BhkRigidBody::ActionConstraint);
        for (index, item) in items.iter().enumerate() {
            e.set(node, BSSimpleList::m_item, *item);
            if index + 1 < items.len() {
                let next = e.new_object::<BSSimpleList>();
                e.set(node, BSSimpleList::m_pkNext, next.addr());
                node = next;
            }
        }
    }

    /// The list items' virtual slots: `Add` (0x9c), `Remove` (0xa0), destroy (4), `Clone`
    /// (0x48).
    const ITEM_ADD: u32 = 0x0900_009c;
    const ITEM_REMOVE: u32 = 0x0900_00a0;
    const ITEM_DESTROY: u32 = 0x0900_0004;
    const ITEM_CLONE: u32 = 0x0900_0048;

    /// A list item whose `GetRTTI` returns `rtti`; its other slots are logging doubles.
    fn item_of(e: &mut Engine, rtti: u32) -> u32 {
        e.register(GET_RTTI_DOUBLE, |e, a| ret(e.mem.u32(a[0] + 0x20)));
        for double in [ITEM_ADD, ITEM_REMOVE, ITEM_DESTROY, ITEM_CLONE] {
            e.register(double, |_, _| Ret::default());
        }
        let item = object(
            e,
            &[
                (VSLOT_GET_RTTI, GET_RTTI_DOUBLE),
                (VSLOT_ADD, ITEM_ADD),
                (VSLOT_REMOVE, ITEM_REMOVE),
                (4, ITEM_DESTROY),
                (VSLOT_CREATE_CLONE, ITEM_CLONE),
            ],
        );
        e.mem.set_u32(item + 0x20, rtti);
        item
    }

    fn constraint(e: &mut Engine) -> u32 {
        item_of(e, CONSTRAINT_RTTI)
    }

    fn action(e: &mut Engine) -> u32 {
        item_of(e, ACTION_RTTI)
    }

    /// A `bhkFixedConstraint`: its type chain continues in `bhkConstraint`.
    fn fixed_constraint(e: &mut Engine) -> u32 {
        e.mem.set_u32(FIXED_CONSTRAINT_RTTI + 4, CONSTRAINT_RTTI);
        item_of(e, FIXED_CONSTRAINT_RTTI)
    }

    /// An item that is neither a constraint nor an action.
    fn other(e: &mut Engine) -> u32 {
        let rtti = e.mem.alloc(8);
        item_of(e, rtti)
    }

    /// A fake memory router whose allocator vtable slots are the doubles `0x0900_0010`
    /// (allocate, `a[1]` bytes) and `0x0900_0013` (free with bytes).
    fn install_heap(e: &mut Engine) {
        let allocator = object(e, &[(4, ALLOCATE_DOUBLE), (0x10, FREE_BYTES_DOUBLE)]);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| ret(router));
        e.register(ALLOCATE_DOUBLE, |e, a| ret(e.mem.alloc(a[1])));
        e.register(FREE_BYTES_DOUBLE, |_, _| Ret::default());
    }

    /// Makes the 256-entry quantised table hold `index / 2`.
    fn fill_table(e: &mut Engine) {
        for index in 0..256u32 {
            e.mem
                .set_f32(QUANTISED_FLOAT_TABLE + 4 * index, index as f32 * 0.5);
        }
    }

    fn quiet_lock(e: &mut Engine) {
        e.register(EMPTY_LOCK, |_, _| Ret::default());
    }

    /// Singly linked list double of `00c88990`: the head takes over the next node.
    fn install_remove_head(e: &mut Engine) {
        e.register(ACTION_LIST_REMOVE_HEAD, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            if next == 0 {
                e.mem.set_u32(a[0], 0);
            } else {
                let item = e.mem.u32(next);
                let after = e.mem.u32(next + 4);
                e.mem.set_u32(a[0], item);
                e.mem.set_u32(a[0] + 4, after);
            }
            Ret::default()
        });
    }

    /// Doubles for the reference counting calls of `AddActCon`.
    fn install_reference_counting(e: &mut Engine) {
        e.register(INTERLOCKED_INCREMENT, |_, _| ret(2));
        e.register(INTERLOCKED_DECREMENT, |_, _| ret(1));
        e.register(ACTION_LIST_ADD_HEAD, |_, _| Ret::default());
    }

    #[test]
    fn is_equal_needs_the_base_and_equal_flags() {
        let mut e = engine();
        let a = e.new_object::<BhkRigidBody>();
        let b = e.new_object::<BhkRigidBody>();
        e.set(a, BhkRigidBody::iFlags, 0x44);
        e.set(b, BhkRigidBody::iFlags, 0x44);
        e.register(WORLD_OBJECT_IS_EQUAL, |_, _| ret(1));
        assert!(e.call(0x00c8_ccc0, &args![a, b]).bool());
        e.set(b, BhkRigidBody::iFlags, 0x40);
        assert!(!e.call(0x00c8_ccc0, &args![a, b]).bool());
        e.set(b, BhkRigidBody::iFlags, 0x44);
        e.register(WORLD_OBJECT_IS_EQUAL, |_, _| ret(0));
        assert!(!e.call(0x00c8_ccc0, &args![a, b]).bool());
    }

    #[test]
    fn motion_type_test_accepts_one_to_three_and_six() {
        let mut e = engine();
        let value = e.mem.alloc(4);
        for (motion, expected) in [
            (-1i32, false),
            (0, false),
            (1, true),
            (2, true),
            (3, true),
            (4, false),
            (5, false),
            (6, true),
            (7, false),
        ] {
            e.mem.set_i32(value, motion);
            assert_eq!(
                e.call(0x00c8_cce0, &args![value]).bool(),
                expected,
                "motion {motion}"
            );
        }
    }

    #[test]
    fn get_max_angular_velocity_looks_the_quantised_value_up() {
        let mut e = engine();
        fill_table(&mut e);
        let (body, hk) = rigid_body(&mut e, &[]);
        e.mem.set_u8(hk + 0x19d, 10);
        e.register(RIGID_BODY_GET_MOTION, |_, _| ret(0));
        e.register_double(HK_UFLOAT8_ASSIGN, |e, a| {
            assert_eq!(e.mem.f32(a[1]), 5.0, "the table value of index 10");
            e.mem.set_u8(a[0], 7);
            Ret::default()
        });
        assert_eq!(e.call(0x00c8_cd20, &args![body]).f32(), 3.5);
        let motion = e.mem.alloc(0x100);
        e.mem.set_u8(motion + 0xbd, 4);
        e.register_double(RIGID_BODY_GET_MOTION, move |_, a| {
            assert_eq!(a, &[hk]);
            ret(motion)
        });
        assert_eq!(e.call(0x00c8_cd20, &args![body]).f32(), 2.0);
        e.set(body, BhkRigidBody::phkObject, Ptr::NULL);
        assert_eq!(e.call(0x00c8_cd20, &args![body]).f32(), 0.0);
        assert_eq!(
            e.call(0x00c8_cd20, &args![Ptr::<BhkRigidBody>::NULL]).f32(),
            0.0
        );
    }

    #[test]
    fn swapping_the_inertia_tensor_swaps_twelve_words() {
        let mut e = engine();
        let matrix = e.mem.alloc(0x40);
        for word in 0..16 {
            e.mem.set_u32(matrix + 4 * word, 0x0102_0304 + word);
        }
        e.call(0x00c8_cdb0, &args![matrix]);
        for word in 0..12 {
            assert_eq!(
                e.mem.u32(matrix + 4 * word),
                (0x0102_0304 + word).swap_bytes()
            );
        }
        for word in 12..16 {
            assert_eq!(e.mem.u32(matrix + 4 * word), 0x0102_0304 + word);
        }
    }

    #[test]
    fn cinfo_scale_scales_centre_inertia_and_mass() {
        let mut e = engine();
        let cinfo = e.new_object::<BhkRigidBodyCinfo>();
        let base = cinfo.addr();
        set_vector(&mut e, base + 0xa0, [1.0, 2.0, 3.0, 4.0]);
        for (column, first) in [(0x70u32, 1.0f32), (0x80, 10.0), (0x90, 100.0)] {
            set_vector(
                &mut e,
                base + column,
                [first, first + 1.0, first + 2.0, 7.0],
            );
        }
        e.mem.set_f32(base + 0xb0, 1.5);
        e.call(0x00c8_cdf0, &args![cinfo, 2.0f32]);
        assert_eq!(vector(&e, base + 0xa0), [2.0, 4.0, 6.0, 8.0]);
        assert_eq!(vector(&e, base + 0x70), [8.0, 16.0, 24.0, 7.0]);
        assert_eq!(vector(&e, base + 0x80), [80.0, 88.0, 96.0, 7.0]);
        assert_eq!(vector(&e, base + 0x90), [800.0, 808.0, 816.0, 7.0]);
        assert_eq!(e.mem.f32(base + 0xb0), 3.0);
    }

    #[test]
    fn cinfo_endian_swaps_every_multibyte_field() {
        let mut e = engine();
        let cinfo = e.mem.alloc(0xe0);
        for word in 0..0x38u32 {
            e.mem.set_u32(cinfo + 4 * word, 0x0102_0300 + word);
        }
        e.call(0x00c8_ce70, &args![cinfo]);
        for word in 0..0x38u32 {
            let original = 0x0102_0300 + word;
            let address = cinfo + 4 * word;
            let expected = match 4 * word {
                0x30..=0xd3 => original.swap_bytes(),
                0x2c => {
                    (original & 0xffff) | (u32::from(((original >> 16) as u16).swap_bytes()) << 16)
                }
                _ => original,
            };
            assert_eq!(e.mem.u32(address), expected, "offset {:#x}", 4 * word);
        }
    }

    #[test]
    fn copy_cinfo_reads_the_motion_and_the_velocity_limits() {
        let mut e = engine();
        fill_table(&mut e);
        let (body, hk) = rigid_body(&mut e, &[]);
        let cinfo = e.new_object::<BhkRigidBodyCinfo>();
        let motion = object(&mut e, &[(0x14, 0x0900_0014)]);
        set_vector(&mut e, motion + 0x90, [1.0, 2.0, 3.0, 4.0]);
        e.mem.set_f32(motion + 0xb4, 0.25);
        e.mem.set_f32(motion + 0xb8, 0.5);
        e.mem.set_u8(motion + 0xbd, 20);
        e.register(WORLD_OBJECT_COPY_CINFO, |_, _| Ret::default());
        e.register(RIGID_BODY_GET_CINFO, |_, _| Ret::default());
        e.register_double(RIGID_BODY_GET_MOTION, move |_, _| ret(motion));
        e.register(MOTION_GET_MASS, |_, _| float_ret(5.5));
        e.register(0x0900_0014, |e, a| {
            e.mem.set_u32(a[1], 0x77);
            Ret::default()
        });
        e.register(GET_MAX_LINEAR_VELOCITY, |_, _| float_ret(123.0));
        e.call_log = Some(vec![]);
        e.call(0x00c8_cfd0, &args![body, cinfo]);
        assert_eq!(
            order(&e),
            [
                0x00c8_cfd0,
                WORLD_OBJECT_COPY_CINFO,
                RIGID_BODY_GET_CINFO,
                RIGID_BODY_GET_MOTION,
                MOTION_GET_MASS,
                0x0900_0014,
                GET_MAX_LINEAR_VELOCITY,
                RIGID_BODY_GET_MOTION
            ]
        );
        assert_eq!(
            args_of(&e, RIGID_BODY_GET_CINFO),
            [[hk, cinfo.addr() + 0x20]]
        );
        let base = cinfo.addr();
        assert_eq!(e.mem.f32(base + 0xb0), 5.5);
        assert_eq!(vector(&e, base + 0xa0), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(e.mem.f32(base + 0xb4), 0.25);
        assert_eq!(e.mem.f32(base + 0xb8), 0.5);
        assert_eq!(e.mem.u32(base + 0x70), 0x77);
        assert_eq!(e.mem.f32(base + 0xc8), 123.0);
        assert_eq!(e.mem.f32(base + 0xcc), 10.0);
        // Without a Havok body only the base copy runs.
        e.set(body, BhkRigidBody::phkObject, Ptr::NULL);
        e.call_log = Some(vec![]);
        e.call(0x00c8_cfd0, &args![body, cinfo]);
        assert_eq!(order(&e), [0x00c8_cfd0, WORLD_OBJECT_COPY_CINFO]);
    }

    #[test]
    fn register_streamables_registers_the_items_when_the_base_succeeds() {
        let mut e = engine();
        e.register(0x0900_0054, |_, _| Ret::default());
        let first = object(&mut e, &[(0x54, 0x0900_0054)]);
        let second = object(&mut e, &[(0x54, 0x0900_0054)]);
        let (body, _) = rigid_body(&mut e, &[]);
        set_items(&mut e, body, &[first, second]);
        e.register(WORLD_OBJECT_REGISTER_STREAMABLES, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00c8_d090, &args![body, 0x5000u32]).bool());
        assert!(args_of(&e, 0x0900_0054).is_empty());
        e.register(WORLD_OBJECT_REGISTER_STREAMABLES, |_, _| ret(1));
        assert!(e.call(0x00c8_d090, &args![body, 0x5000u32]).bool());
        assert_eq!(
            args_of(&e, 0x0900_0054),
            [[first, 0x5000], [second, 0x5000]]
        );
    }

    #[test]
    fn item_count_counts_until_the_empty_node() {
        let mut e = engine();
        let (body, _) = rigid_body(&mut e, &[]);
        assert_eq!(e.call(0x00c8_d0d0, &args![body]).u32(), 0);
        set_items(&mut e, body, &[5, 6, 7]);
        assert_eq!(e.call(0x00c8_d0d0, &args![body]).u32(), 3);
    }

    /// Checks a getter that copies 16 bytes from `body + offset`, or from `default` without
    /// a body, to `out` and returns `out`.
    fn check_vector_getter(address: u32, offset: u32, default: u32) {
        let mut e = engine();
        let (body, hk) = rigid_body(&mut e, &[]);
        let out = e.mem.alloc(0x10);
        set_vector(&mut e, hk + offset, [1.0, 2.0, 3.0, 4.0]);
        set_vector(&mut e, default, [5.0, 6.0, 7.0, 8.0]);
        assert_eq!(e.call(address, &args![body, out]).u32(), out);
        assert_eq!(vector(&e, out), [1.0, 2.0, 3.0, 4.0]);
        e.set(body, BhkRigidBody::phkObject, Ptr::NULL);
        assert_eq!(e.call(address, &args![body, out]).u32(), out);
        assert_eq!(vector(&e, out), [5.0, 6.0, 7.0, 8.0]);
        assert_eq!(
            e.call(address, &args![Ptr::<BhkRigidBody>::NULL, out])
                .u32(),
            out
        );
    }

    #[test]
    fn get_position_copies_the_body_position() {
        check_vector_getter(0x00c8_d0f0, 0x120, DEFAULT_VECTOR);
    }

    #[test]
    fn get_rotation_copies_the_body_rotation() {
        check_vector_getter(0x00c8_d130, 0x160, DEFAULT_ROTATION);
    }

    #[test]
    fn get_center_of_mass_local_reads_the_body_centre() {
        check_vector_getter(0x00c8_d200, 0x170, DEFAULT_VECTOR);
    }

    #[test]
    fn get_center_of_mass_in_world_copies_the_world_centre() {
        check_vector_getter(0x00c8_d240, 0x140, DEFAULT_VECTOR);
    }

    /// Checks a setter that calls `setter(body, vector)` between two empty locks.
    fn check_locked_setter(address: u32, setter: u32) {
        let mut e = engine();
        let (body, hk) = rigid_body(&mut e, &[]);
        quiet_lock(&mut e);
        e.register(setter, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(address, &args![body, 0x7000u32]);
        assert_eq!(order(&e), [address, EMPTY_LOCK, setter, EMPTY_LOCK]);
        assert_eq!(args_of(&e, setter), [[hk, 0x7000]]);
        assert_eq!(args_of(&e, EMPTY_LOCK), [[body.addr()], [body.addr()]]);
        // No Havok body, no body object: nothing happens.
        e.set(body, BhkRigidBody::phkObject, Ptr::NULL);
        e.call_log = Some(vec![]);
        e.call(address, &args![body, 0x7000u32]);
        e.call(address, &args![Ptr::<BhkRigidBody>::NULL, 0x7000u32]);
        assert_eq!(order(&e), [address, address]);
    }

    #[test]
    fn set_position_sets_the_position_between_locks() {
        check_locked_setter(0x00c8_d170, RIGID_BODY_SET_POSITION);
    }

    #[test]
    fn set_rotation_sets_the_rotation_between_locks() {
        check_locked_setter(0x00c8_d1a0, RIGID_BODY_SET_ROTATION);
    }

    #[test]
    fn set_pos_rot_sets_both_after_fetching_the_writable_world() {
        let mut e = engine();
        let (body, hk) = rigid_body(&mut e, &[]);
        e.register(WORLD_OBJECT_GET_WORLD_RW, |_, _| ret(0));
        e.register(RIGID_BODY_SET_POSITION_AND_ROTATION, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c8_d1d0, &args![body, 0x7000u32, 0x7100u32]);
        assert_eq!(
            order(&e),
            [
                0x00c8_d1d0,
                WORLD_OBJECT_GET_WORLD_RW,
                RIGID_BODY_SET_POSITION_AND_ROTATION
            ]
        );
        assert_eq!(
            args_of(&e, RIGID_BODY_SET_POSITION_AND_ROTATION),
            [[hk, 0x7000, 0x7100]]
        );
        e.set(body, BhkRigidBody::phkObject, Ptr::NULL);
        e.call_log = Some(vec![]);
        e.call(0x00c8_d1d0, &args![body, 0x7000u32, 0x7100u32]);
        assert_eq!(order(&e), [0x00c8_d1d0]);
    }

    #[test]
    fn get_transform_copies_the_body_transform_or_the_identity() {
        let mut e = engine();
        let (body, hk) = rigid_body(&mut e, &[]);
        let out = e.mem.alloc(0x40);
        for row in 0..4 {
            set_vector(&mut e, hk + 0xf0 + 0x10 * row, [row as f32; 4]);
        }
        assert_eq!(e.call(0x00c8_d280, &args![body, out]).u32(), out);
        for row in 0..4 {
            assert_eq!(vector(&e, out + 0x10 * row), [row as f32; 4]);
        }
        e.set(body, BhkRigidBody::phkObject, Ptr::NULL);
        assert_eq!(e.call(0x00c8_d280, &args![body, out]).u32(), out);
        assert_eq!(vector(&e, out), [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(vector(&e, out + 0x10), [0.0, 1.0, 0.0, 0.0]);
        assert_eq!(vector(&e, out + 0x20), [0.0, 0.0, 1.0, 0.0]);
        assert_eq!(vector(&e, out + 0x30), [0.0; 4]);
    }

    #[test]
    fn create_display_geometry_attaches_a_jack_at_the_centre_of_mass() {
        let mut e = engine();
        let (body, _) = rigid_body(&mut e, &[(0xec, 0x0900_00ec)]);
        e.register(0x0900_00ec, |e, a| {
            set_vector(e, a[1], [1.0, 2.0, 3.0, 9.0]);
            Ret::default()
        });
        e.register(NI_POINT3_FROM_VECTOR, |e, a| {
            for lane in 0..3 {
                let value = e.mem.u32(a[1] + 4 * lane);
                e.mem.set_u32(a[0] + 4 * lane, value);
            }
            Ret::default()
        });
        e.set_global(COORDINATE_JACK_SIZE, 6.0f32);
        let jack = e.mem.alloc(0x80);
        e.register_double(CREATE_COORDINATE_JACK, move |_, _| ret(jack));
        e.register(0x0900_00dc, |_, _| Ret::default());
        let node = object(&mut e, &[(0xdc, 0x0900_00dc)]);
        e.register(WORLD_OBJECT_CREATE_DISPLAY_GEOMETRY, |_, _| ret(0));
        assert_eq!(e.call(0x00c8_d2f0, &args![body, 3u32]).u32(), 0);
        e.register_double(WORLD_OBJECT_CREATE_DISPLAY_GEOMETRY, move |_, _| ret(node));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c8_d2f0, &args![body, 3u32]).u32(), node);
        assert_eq!(args_of(&e, CREATE_COORDINATE_JACK), [[6.0f32.to_bits()]]);
        assert_eq!(args_of(&e, 0x0900_00dc), [[node, jack, 1]]);
        assert_eq!(
            [0, 4, 8].map(|lane| e.mem.f32(jack + 0x58 + lane)),
            [1.0, 2.0, 3.0]
        );
        // A failed jack is not attached.
        e.register(CREATE_COORDINATE_JACK, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c8_d2f0, &args![body, 3u32]).u32(), node);
        assert!(args_of(&e, 0x0900_00dc).is_empty());
    }

    #[test]
    fn get_aabb_local_queries_the_shape_or_fills_the_default_box() {
        let mut e = engine();
        quiet_lock(&mut e);
        e.set_global(AABB_LOW_FILL, -0.5f32);
        e.set_global(AABB_HIGH_FILL, 0.25f32);
        e.set_global(AABB_W_FILL, 0.0f32);
        let (body, hk) = rigid_body(&mut e, &[]);
        let out = e.mem.alloc(0x20);
        e.register(0x0900_001c, |_, _| Ret::default());
        let shape = object(&mut e, &[(0x1c, 0x0900_001c)]);
        e.mem.set_u32(hk + 0x10, shape);
        e.call_log = Some(vec![]);
        e.call(0x00c8_d390, &args![body, out]);
        assert_eq!(
            order(&e),
            [0x00c8_d390, EMPTY_LOCK, EMPTY_LOCK, 0x0900_001c]
        );
        assert_eq!(
            args_of(&e, 0x0900_001c),
            [[shape, SHAPE_QUERY_CONTEXT, 0.25f32.to_bits(), out]]
        );
        e.mem.set_u32(hk + 0x10, 0);
        e.call(0x00c8_d390, &args![body, out]);
        assert_eq!(vector(&e, out), [-0.5, -0.5, -0.5, 0.0]);
        assert_eq!(vector(&e, out + 0x10), [0.25, 0.25, 0.25, 0.0]);
        e.mem.set_f32(out, 9.0);
        e.set(body, BhkRigidBody::phkObject, Ptr::NULL);
        e.call(0x00c8_d390, &args![body, out]);
        assert_eq!(vector(&e, out), [-0.5, -0.5, -0.5, 0.0]);
    }

    #[test]
    fn set_havok_assert_state_sets_the_four_message_ids() {
        let mut e = engine();
        e.register(0x0900_0010, |_, _| Ret::default());
        let reporter = object(&mut e, &[(HAVOK_ERROR_SLOT_SET_ENABLED, 0x0900_0010)]);
        e.set_global(HAVOK_ERROR_OBJECT, reporter);
        e.call_log = Some(vec![]);
        e.call(0x00c8_d440, &args![0u32, 0x101u32]);
        assert_eq!(
            args_of(&e, 0x0900_0010),
            [
                [reporter, 0xf043_d534, 1],
                [reporter, 0xf02e_32e1, 1],
                [reporter, 0xf02e_32e2, 1],
                [reporter, 0xf021_32df, 1]
            ]
        );
    }

    #[test]
    fn quality_type_string_names_each_quality() {
        let mut e = engine();
        let cinfo = e.new_object::<BhkRigidBodyCinfo>();
        for (quality, expected) in [
            (0xffu8, 0x0104_8858u32),
            (0, 0x010c_6084),
            (1, 0x010c_6078),
            (2, 0x0102_9400),
            (3, 0),
            (4, 0x0109_25bc),
            (5, 0x010c_606c),
            (6, 0x010c_6064),
            (7, 0x010c_605c),
            (8, 0),
        ] {
            e.mem.set_u8(cinfo.addr() + 0xd7, quality);
            assert_eq!(
                e.call(0x00c8_d4a0, &args![cinfo]).u32(),
                expected,
                "quality {quality}"
            );
        }
    }

    #[test]
    fn old_cinfo_conversion_renumbers_the_motion_and_moves_the_floats() {
        let mut e = engine();
        let old = e.mem.alloc(0xe0);
        let new = e.new_object::<BhkRigidBodyCinfo>();
        e.mem.set_u32(old, 0x1234);
        e.mem.set_u32(old + 0x24, 0x5678);
        e.mem.set_u8(old + 0x28, 3);
        e.mem.set_u16(old + 0x2a, 0x99);
        for block in 0..8u32 {
            set_vector(&mut e, old + 0x30 + 0x10 * block, [block as f32; 4]);
        }
        for (index, offset) in [0xb0u32, 0xb4, 0xb8, 0xbc, 0xc0, 0xc4, 0xc8, 0xcc]
            .into_iter()
            .enumerate()
        {
            e.mem.set_f32(old + offset, 1.0 + index as f32);
        }
        e.mem.set_u8(old + 0xd0, 5);
        e.mem.set_u8(old + 0xd1, 2);
        e.mem.set_u8(old + 0xd2, 4);
        e.mem.set_u8(old + 0xd3, 2);
        e.call(0x00c8_d510, &args![new, old]);
        let base = new.addr();
        assert_eq!(e.mem.u32(base), 0x1234);
        assert_eq!(e.mem.u32(base + 0x20), 0x1234);
        assert_eq!(e.mem.u32(base + 0x24), 0x5678);
        assert_eq!(e.mem.u8(base + 0x2c), 3);
        assert_eq!(e.mem.u16(base + 0x2e), 0x99);
        assert_eq!(e.mem.u32(base + 0x28), 0);
        assert_eq!(vector(&e, base + 0x30), [0.0; 4]);
        assert_eq!(vector(&e, base + 0xa0), [7.0; 4]);
        assert_eq!(e.mem.f32(base + 0xb0), 1.0);
        assert_eq!(e.mem.f32(base + 0xb8), 3.0);
        assert_eq!(e.mem.f32(base + 0xbc), 1.0, "gravity factor is 1.0");
        assert_eq!(e.mem.f32(base + 0xc0), 4.0);
        assert_eq!(e.mem.f32(base + 0xc4), 5.0);
        assert_eq!(e.mem.f32(base + 0xc8), 6.0);
        assert_eq!(e.mem.f32(base + 0xcc), 7.0);
        assert_eq!(e.mem.f32(base + 0xd0), 8.0);
        assert_eq!(e.mem.u8(base + 0xd4), 3, "motion 5 becomes 3");
        assert_eq!(e.mem.u8(base + 0xd5), 1);
        assert_eq!(e.mem.u8(base + 0xd6), 4);
        assert_eq!(e.mem.u8(base + 0xd7), 1, "quality 2 becomes 1");
        e.mem.set_u8(old + 0xd0, 3);
        e.mem.set_u8(old + 0xd1, 1);
        e.mem.set_u8(old + 0xd3, 5);
        e.call(0x00c8_d510, &args![new, old]);
        assert_eq!(e.mem.u8(base + 0xd4), 2, "motion 3 becomes 2");
        assert_eq!(e.mem.u8(base + 0xd5), 0);
        assert_eq!(e.mem.u8(base + 0xd7), 5);
    }

    const INIT_SET_HK_OBJECT: u32 = 0x0900_008c;
    const INIT_POST_INIT: u32 = 0x0900_00bc;

    /// A rigid body without a Havok body, the new Havok body `hk_body` (with a shape that
    /// reports the box `(0,0,0)..(1,2,3)`) and the cinfo for the `Init` tests.
    fn init_fixture(
        e: &mut Engine,
        mass: f32,
        motion: u8,
    ) -> (Ptr<BhkRigidBody>, u32, Ptr<BhkRigidBodyCinfo>) {
        install_heap(e);
        quiet_lock(e);
        e.set_global(PENETRATION_DEPTH_FLOOR, 0.001f32);
        e.set_global(PENETRATION_DEPTH_CEILING, 0.2f64);
        e.set_global(DEFAULT_PENETRATION_DEPTH, 0.15f32);
        e.set_global(MAX_LINEAR_VELOCITY_FLOOR, 1000.0f64);
        e.set_global(DEFAULT_MAX_LINEAR_VELOCITY, 1000.0f32);
        e.set_global(RESTITUTION_CEILING, 1.99f64);
        e.set_global(DEFAULT_RESTITUTION, 1.98f32);
        e.set_global(QUALITY_EXTENT_LIMIT, 0.5f32);
        e.set_global(QUALITY_VELOCITY_SCALE, 2.0f32);
        e.set_global(QUALITY_VELOCITY_LIMIT, 1.0f32);
        e.set_global(AABB_HIGH_FILL, 0.001f32);
        e.register(INIT_SET_HK_OBJECT, |e, a| {
            e.mem.set_u32(a[0] + 8, a[1]);
            Ret::default()
        });
        e.register(INIT_POST_INIT, |_, _| Ret::default());
        e.register(REMOVE_REFERENCE, |_, _| Ret::default());
        e.register(RIGID_BODY_SET_MOTION_TYPE, |_, _| Ret::default());
        e.register(0x0900_001c, |e, a| {
            set_vector(e, a[3], [0.0; 4]);
            set_vector(e, a[3] + 0x10, [1.0, 2.0, 3.0, 0.0]);
            Ret::default()
        });
        let shape = object(e, &[(0x1c, 0x0900_001c)]);
        let hk_body = e.mem.alloc(0x400);
        e.mem.set_u32(hk_body + 0x10, shape);
        e.register_double(RIGID_BODY_CONSTRUCT, move |_, _| ret(hk_body));
        let (body, _) = rigid_body(e, &[(0x8c, INIT_SET_HK_OBJECT), (0xbc, INIT_POST_INIT)]);
        e.set(body, BhkRigidBody::phkObject, Ptr::NULL);
        let cinfo = e.new_object::<BhkRigidBodyCinfo>();
        let base = cinfo.addr();
        e.mem.set_u32(base, 0x11);
        e.mem.set_u32(base + 4, 0x22);
        e.mem.set_f32(base + 0xb0, mass);
        e.mem.set_u8(base + 0xd4, motion);
        e.mem.set_u8(base + 0xd7, 0xff);
        e.mem.set_u8(base + 0xd6, 9);
        e.mem.set_f32(base + 0xd0, 0.0);
        e.mem.set_f32(base + 0xc8, 10.0);
        e.mem.set_f32(base + 0xc4, 2.5);
        (body, hk_body, cinfo)
    }

    #[test]
    fn init_clamps_the_cinfo_and_keeps_a_keyframed_motion_for_later() {
        let mut e = engine();
        let (body, hk_body, cinfo) = init_fixture(&mut e, 2.0, 4);
        let word = e.tls() + SCOPE_WORD;
        e.mem.set_u32(word, 0x55);
        e.register_double(INIT_POST_INIT, move |e, _| {
            assert_eq!(e.mem.u32(word), 0x10, "the scope word is raised");
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00c8_d650, &args![body, cinfo]);
        assert_eq!(e.mem.u32(word), 0x55);
        assert_eq!(
            order(&e),
            [
                0x00c8_d650,
                MEMORY_ROUTER,
                ALLOCATE_DOUBLE,
                RIGID_BODY_CONSTRUCT,
                RIGID_BODY_SET_MOTION_TYPE,
                INIT_SET_HK_OBJECT,
                REMOVE_REFERENCE,
                INIT_POST_INIT
            ]
        );
        let base = cinfo.addr();
        assert_eq!(args_of(&e, ALLOCATE_DOUBLE).len(), 1);
        assert_eq!(args_of(&e, ALLOCATE_DOUBLE)[0][1], 0x220);
        let raw = args_of(&e, RIGID_BODY_CONSTRUCT)[0][0];
        assert_eq!(args_of(&e, RIGID_BODY_CONSTRUCT), [[raw, base + 0x20]]);
        assert_eq!(e.mem.u16(raw + 4), 0x220);
        assert_eq!(
            args_of(&e, RIGID_BODY_SET_MOTION_TYPE),
            [[hk_body, 4, 1, 0]]
        );
        assert_eq!(args_of(&e, INIT_SET_HK_OBJECT), [[body.addr(), hk_body]]);
        assert_eq!(args_of(&e, REMOVE_REFERENCE), [[hk_body]]);
        assert_eq!(args_of(&e, INIT_POST_INIT), [[body.addr(), base]]);
        assert_eq!(e.mem.f32(base + 0xd0), 0.15);
        assert_eq!(e.mem.f32(base + 0xc8), 1000.0);
        assert_eq!(e.mem.f32(base + 0xc4), 1.98);
        assert_eq!(e.mem.u8(base + 0xd6), 2);
        assert_eq!(e.mem.u8(base + 0xd4), 4, "the keyframed type is restored");
        assert_eq!(e.mem.u8(base + 0xd7), 4);
        assert_eq!(e.mem.u8(base + 0xda), 3);
        assert_eq!(e.mem.u32(base + 0x20), 0x11);
        assert_eq!(e.mem.u32(base + 0x24), 0x22);
        assert_eq!(e.get(body, BhkRigidBody::phkObject).addr(), hk_body);
    }

    #[test]
    fn init_picks_the_quality_of_a_dynamic_body_from_its_box() {
        let mut e = engine();
        let (body, hk_body, cinfo) = init_fixture(&mut e, 1.0, 1);
        e.mem.set_f32(cinfo.addr() + 0xd0, 0.1);
        e.call_log = Some(vec![]);
        e.call(0x00c8_d650, &args![body, cinfo]);
        let base = cinfo.addr();
        assert!(args_of(&e, RIGID_BODY_SET_MOTION_TYPE).is_empty());
        assert_eq!(e.mem.f32(base + 0xd0), 0.1, "a valid depth is kept");
        // The smallest extent (1.0) is above the 0.5 limit: quality 2.
        assert_eq!(e.mem.u8(base + 0xd7), 2);
        assert_eq!(e.mem.u8(hk_body + 0x2a), 2);
        assert_eq!(args_of(&e, EMPTY_LOCK).len(), 4);
        // A small box (0.1 per side) is below the 0.5 extent limit. The velocity test
        // (2.0 * 100.0 = 200) is passed by a limit of 1000: quality 2 again.
        e.register(0x0900_001c, |e, a| {
            set_vector(e, a[3], [0.0; 4]);
            set_vector(e, a[3] + 0x10, [0.1, 0.1, 0.1, 0.0]);
            Ret::default()
        });
        e.register(GET_MAX_LINEAR_VELOCITY, |_, _| float_ret(100.0));
        e.set_global(QUALITY_VELOCITY_LIMIT, 1000.0f32);
        e.set(body, BhkRigidBody::phkObject, Ptr::NULL);
        e.mem.set_u8(base + 0xd7, 0xff);
        e.mem.set_u8(hk_body + 0x2a, 0);
        e.call(0x00c8_d650, &args![body, cinfo]);
        assert_eq!(e.mem.u8(base + 0xd7), 2);
        assert_eq!(e.mem.u8(hk_body + 0x2a), 2);
        // Above the velocity limit a quality of 2 is raised to 4; other qualities stay.
        e.set_global(QUALITY_VELOCITY_LIMIT, 1.0f32);
        e.set(body, BhkRigidBody::phkObject, Ptr::NULL);
        e.mem.set_u8(base + 0xd7, 2);
        e.call(0x00c8_d650, &args![body, cinfo]);
        assert_eq!(e.mem.u8(base + 0xd7), 4);
        assert_eq!(e.mem.u8(hk_body + 0x2a), 4);
        e.set(body, BhkRigidBody::phkObject, Ptr::NULL);
        e.mem.set_u8(base + 0xd7, 5);
        e.call(0x00c8_d650, &args![body, cinfo]);
        assert_eq!(e.mem.u8(base + 0xd7), 5);
        assert_eq!(e.mem.u8(hk_body + 0x2a), 4, "not rewritten");
    }

    #[test]
    fn save_binary_writes_the_counts_and_swaps_the_cinfo_around_the_base() {
        let mut e = engine();
        let cinfo = e.mem.alloc(0xe0);
        e.mem.set_u32(cinfo + 0x30, 0x0102_0304);
        let (body, _) = rigid_body(&mut e, &[(0xb4, 0x0900_00b4)]);
        e.register_double(0x0900_00b4, move |_, _| ret(cinfo));
        e.set(body, BhkRigidBody::iFlags, 0xabcd);
        set_items(&mut e, body, &[10, 20]);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(ENTITY_SAVE_BINARY, move |e, _| {
            sink.borrow_mut().push(e.mem.u32(cinfo + 0x30));
            Ret::default()
        });
        let written = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&written);
        e.register_double(0x0900_0020, move |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (4, 4, 1));
            sink.borrow_mut().push(e.mem.u32(a[1]));
            ret(4)
        });
        e.register(0x0900_0030, |_, _| Ret::default());
        let out = e.mem.alloc(0x20);
        e.mem.set_u32(out + 0xc, 0x0900_0020);
        let stream = object(&mut e, &[(0x30, 0x0900_0030)]);
        e.mem.set_u32(stream + 0x250, out);
        e.call_log = Some(vec![]);
        e.call(0x00c8_d960, &args![body, stream]);
        assert_eq!(*seen.borrow(), [0x0403_0201]);
        assert_eq!(e.mem.u32(cinfo + 0x30), 0x0102_0304, "swapped back");
        assert_eq!(*written.borrow(), [2, 0xabcd]);
        assert_eq!(e.mem.u32(out + 4), 8);
        assert_eq!(args_of(&e, 0x0900_0030), [[stream, 10], [stream, 20]]);
        assert_eq!(args_of(&e, ENTITY_SAVE_BINARY), [[body.addr(), stream]]);
        // A stream in PC byte order leaves the cinfo alone.
        e.mem.set_u8(stream + 0x1e4, 1);
        e.call(0x00c8_d960, &args![body, stream]);
        assert_eq!(*seen.borrow(), [0x0403_0201, 0x0102_0304]);
    }

    #[test]
    fn kill_clears_the_list_then_kills_the_entity_in_the_alloc_scope() {
        let mut e = engine();
        let (body, _) = rigid_body(&mut e, &[]);
        let word = e.tls() + SCOPE_WORD;
        e.mem.set_u32(word, 0x55);
        for address in [ACTION_LIST_CLEAR, ENTITY_KILL] {
            e.register_double(address, move |e, _| {
                assert_eq!(e.mem.u32(word), 0x10);
                Ret::default()
            });
        }
        e.call_log = Some(vec![]);
        e.call(0x00c8_da40, &args![body]);
        assert_eq!(order(&e), [0x00c8_da40, ACTION_LIST_CLEAR, ENTITY_KILL]);
        assert_eq!(
            args_of(&e, ACTION_LIST_CLEAR),
            [[body.addr() + BhkRigidBody::ActionConstraint.off]]
        );
        assert_eq!(args_of(&e, ENTITY_KILL), [[body.addr()]]);
        assert_eq!(e.mem.u32(word), 0x55);
    }

    #[test]
    fn add_act_con_to_world_adds_every_item_or_only_fixed_constraints() {
        let mut e = engine();
        let (body, _) = rigid_body(&mut e, &[]);
        let items = [
            constraint(&mut e),
            fixed_constraint(&mut e),
            action(&mut e),
            other(&mut e),
        ];
        set_items(&mut e, body, &items);
        e.call_log = Some(vec![]);
        e.call(0x00c8_dac0, &args![body, 0x7777u32]);
        assert_eq!(
            args_of(&e, ITEM_ADD),
            [
                [items[0], 0x7777],
                [items[1], 0x7777],
                [items[2], 0x7777],
                [items[3], 0x7777]
            ]
        );
        e.call_log = Some(vec![]);
        e.set(body, BhkRigidBody::iFlags, 4);
        e.call(0x00c8_dac0, &args![body, 0x7777u32]);
        assert_eq!(args_of(&e, ITEM_ADD), [[items[1], 0x7777]]);
    }

    /// Installs `GetProperty` so that it returns a property whose record is `record`
    /// (no property at all when `property` is false).
    fn install_property(e: &mut Engine, property: bool, record: u32) {
        let property_object = object(e, &[(0x3c, 0x0900_003c)]);
        e.register_double(0x0900_003c, move |_, _| ret(record));
        e.register_double(WORLD_OBJECT_GET_PROPERTY, move |e, a| {
            e.mem
                .set_u32(a[0], if property { property_object } else { 0 });
            ret(a[0])
        });
    }

    #[test]
    fn add_adds_the_items_after_the_base_add() {
        let mut e = engine();
        quiet_lock(&mut e);
        let (body, hk) = rigid_body(&mut e, &[(0xa0, ITEM_REMOVE)]);
        let item = constraint(&mut e);
        set_items(&mut e, body, &[item]);
        install_property(&mut e, false, 0);
        e.register(WORLD_OBJECT_ADD, |_, _| ret(1));
        e.call_log = Some(vec![]);
        assert!(e.call(0x00c8_db50, &args![body, 0x7777u32]).bool());
        assert_eq!(args_of(&e, WORLD_OBJECT_ADD), [[body.addr(), 0x7777]]);
        assert_eq!(args_of(&e, ITEM_ADD), [[item, 0x7777]]);
        // The base add fails: the items follow only for a body already in a world whose
        // motion type is dynamic.
        e.register(WORLD_OBJECT_ADD, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00c8_db50, &args![body, 0x7777u32]).bool());
        assert!(args_of(&e, ITEM_ADD).is_empty());
        e.mem.set_u32(hk + 8, 0x4444);
        e.mem.set_u8(hk + 0xe8, 1);
        assert!(!e.call(0x00c8_db50, &args![body, 0x7777u32]).bool());
        assert_eq!(args_of(&e, ITEM_ADD), [[item, 0x7777]]);
        e.mem.set_u8(hk + 0xe8, 5);
        e.call_log = Some(vec![]);
        e.call(0x00c8_db50, &args![body, 0x7777u32]);
        assert!(args_of(&e, ITEM_ADD).is_empty());
    }

    #[test]
    fn add_removes_an_unwanted_object_and_records_the_world() {
        let mut e = engine();
        quiet_lock(&mut e);
        let (body, _) = rigid_body(&mut e, &[(0xa0, 0x0900_00a1)]);
        e.register(0x0900_00a1, |_, _| Ret::default());
        let record = e.mem.alloc(0x40);
        e.mem.set_f32(record + 0x18, 1.0);
        e.mem.set_f32(record + 0x1c, 1.0);
        e.mem.set_u32(record + 0x28, 0);
        install_property(&mut e, true, record);
        e.register(WORLD_OBJECT_ADD, |_, _| panic!("not added"));
        e.register(WORLD_OBJECT_GET_WORLD, |_, _| ret(0x4444));
        e.call_log = Some(vec![]);
        // Marked with no count and the assert byte clear: never added.
        assert!(e.call(0x00c8_db50, &args![body, 0x7777u32]).bool());
        assert_eq!(args_of(&e, 0x0900_00a1), [[body.addr()]]);
        assert_eq!(e.mem.u32(record + 0x24), 0x7777);
        // Bit 6 of the flags also keeps it out.
        e.mem.set_u32(record + 0x24, 0);
        e.mem.set_i32(record + 0x28, 3);
        e.set(body, BhkRigidBody::iFlags, 0x40);
        assert!(e.call(0x00c8_db50, &args![body, 0x7777u32]).bool());
        assert_eq!(e.mem.u32(record + 0x24), 0x7777);
        // Not in a world any more: nothing to remove.
        e.register(WORLD_OBJECT_GET_WORLD, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert!(e.call(0x00c8_db50, &args![body, 0x7777u32]).bool());
        assert!(args_of(&e, 0x0900_00a1).is_empty());
    }

    #[test]
    fn force_add_adds_the_items_and_records_the_world() {
        let mut e = engine();
        let (body, _) = rigid_body(&mut e, &[]);
        let item = constraint(&mut e);
        set_items(&mut e, body, &[item]);
        let record = e.mem.alloc(0x40);
        install_property(&mut e, true, record);
        e.register(WORLD_OBJECT_ADD, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00c8_dcb0, &args![body, 0x7777u32]).bool());
        assert!(args_of(&e, ITEM_ADD).is_empty());
        assert_eq!(e.mem.u32(record + 0x24), 0x7777);
        e.register(WORLD_OBJECT_ADD, |_, _| ret(1));
        e.mem.set_u32(record + 0x24, 0);
        assert!(e.call(0x00c8_dcb0, &args![body, 0x7778u32]).bool());
        assert_eq!(args_of(&e, ITEM_ADD), [[item, 0x7778]]);
        assert_eq!(e.mem.u32(record + 0x24), 0x7778);
        // No property: nothing is recorded.
        install_property(&mut e, false, 0);
        assert!(e.call(0x00c8_dcb0, &args![body, 0x7779u32]).bool());
    }

    #[test]
    fn set_motion_type_switches_the_layer_and_the_havok_motion() {
        let mut e = engine();
        quiet_lock(&mut e);
        e.register(RIGID_BODY_SET_MOTION_TYPE, |_, _| Ret::default());
        e.register(FIX_LAYER, |_, _| Ret::default());
        e.register(GET_DYNAMIC_MASS, |_, _| float_ret(0.0));
        let (body, hk) = rigid_body(&mut e, &[]);
        // Layer 8 asked for a dynamic type: layer 0x1d and the Havok call.
        e.mem.set_u32(hk + 0x2c, 0x0300 | 8);
        e.mem.set_u8(hk + 0xe8, 4);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00c8_dd10, &args![body, 2i32]).bool());
        assert_eq!(e.mem.u32(hk + 0x2c), 0x0300 | 0x1d);
        assert_eq!(
            order(&e),
            [
                0x00c8_dd10,
                EMPTY_LOCK,
                RIGID_BODY_SET_MOTION_TYPE,
                EMPTY_LOCK
            ]
        );
        assert_eq!(args_of(&e, RIGID_BODY_SET_MOTION_TYPE), [[hk, 2, 1, 0]]);
        // The same type again: only the layer fix runs.
        e.mem.set_u8(hk + 0xe8, 2);
        e.mem.set_u32(hk + 0x2c, 0x0300 | 8);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00c8_dd10, &args![body, 2i32]).bool());
        assert_eq!(args_of(&e, FIX_LAYER), [[body.addr(), 0x0300 | 0x1d]]);
        assert!(args_of(&e, RIGID_BODY_SET_MOTION_TYPE).is_empty());
        // Layer 0x1d asked for a fixed type goes back to layer 8.
        e.mem.set_u32(hk + 0x2c, 0x0300 | 0x1d);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00c8_dd10, &args![body, 5i32]).bool());
        assert_eq!(e.mem.u32(hk + 0x2c), 0x0300 | 8);
        // Any other layer: unchanged type is refused; keyframed to dynamic needs mass.
        e.mem.set_u32(hk + 0x2c, 3);
        e.mem.set_u8(hk + 0xe8, 5);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00c8_dd10, &args![body, 5i32]).bool());
        assert!(!e.call(0x00c8_dd10, &args![body, 1i32]).bool());
        assert!(args_of(&e, RIGID_BODY_SET_MOTION_TYPE).is_empty());
        e.register(GET_DYNAMIC_MASS, |_, _| float_ret(2.0));
        assert!(e.call(0x00c8_dd10, &args![body, 1i32]).bool());
        assert_eq!(args_of(&e, RIGID_BODY_SET_MOTION_TYPE), [[hk, 1, 1, 0]]);
        e.mem.set_u8(hk + 0xe8, 1);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00c8_dd10, &args![body, 5i32]).bool());
        assert_eq!(args_of(&e, RIGID_BODY_SET_MOTION_TYPE), [[hk, 5, 1, 0]]);
        // No Havok body.
        e.set(body, BhkRigidBody::phkObject, Ptr::NULL);
        assert!(!e.call(0x00c8_dd10, &args![body, 1i32]).bool());
    }

    /// Doubles shared by the `00c8de20` tests: identity rotation, zero centres, and the
    /// velocity setters recording the vectors they get.
    fn follow_fixture(e: &mut Engine) -> (Ptr<BhkRigidBody>, VectorLog, VectorLog) {
        fill_table(e);
        quiet_lock(e);
        set_vector(e, ONES_VECTOR, [1.0; 4]);
        set_vector(e, HALF_VECTOR, [-0.5; 4]);
        e.set_global(ANGLE_THRESHOLD, 0.001f64);
        e.register(0x0900_00ec, |e, a| {
            set_vector(e, a[1], [0.0; 4]);
            ret(a[1])
        });
        e.register(0x0900_00d8, |e, a| {
            set_vector(e, a[1], [0.0, 0.0, 0.0, 1.0]);
            ret(a[1])
        });
        let (body, hk) = rigid_body(
            e,
            &[
                (0xec, 0x0900_00ec),
                (0xf0, 0x0900_00ec),
                (0xd8, 0x0900_00d8),
            ],
        );
        let motion = e.mem.alloc(0x100);
        e.mem.set_u8(motion + 0xbd, 3);
        e.register_double(RIGID_BODY_GET_MOTION, move |_, _| ret(motion));
        e.register(QUATERNION_SET_MUL, |e, a| {
            set_vector(e, a[0], [0.0, 0.0, 0.0, 1.0]);
            Ret::default()
        });
        e.register(QUATERNION_NORMALIZE, |_, _| Ret::default());
        e.register(QUATERNION_GET_ANGLE, |_, _| float_ret(0.0));
        e.register(QUATERNION_GET_AXIS, |_, _| Ret::default());
        e.register(GET_MAX_LINEAR_VELOCITY, |_, _| float_ret(100.0));
        e.register(WORLD_OBJECT_GET_WORLD, |_, _| ret(0));
        let linear = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&linear);
        e.register_double(RIGID_BODY_SET_LINEAR_VELOCITY, move |e, a| {
            assert_eq!(a[0], hk);
            sink.borrow_mut().push(vector(e, a[1]));
            Ret::default()
        });
        let angular = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&angular);
        e.register_double(RIGID_BODY_SET_ANGULAR_VELOCITY, move |e, a| {
            sink.borrow_mut().push(vector(e, a[1]));
            Ret::default()
        });
        (body, linear, angular)
    }

    #[test]
    fn fn_00c8de20_blends_the_velocity_towards_the_target() {
        let mut e = engine();
        let (body, linear, angular) = follow_fixture(&mut e);
        let position = e.mem.alloc(0x10);
        set_vector(&mut e, position, [2.0, 0.0, 0.0, 0.0]);
        let rotation = e.mem.alloc(0x10);
        set_vector(&mut e, rotation, [0.0, 0.0, 0.0, 1.0]);
        let call = |e: &mut Engine| {
            e.call(
                0x00c8_de20,
                &args![body, position, rotation, 10.0f32, 0.5f32],
            );
        };
        call(&mut e);
        // A world with gravity (0, 0, -10) pulls the vertical velocity.
        let world = e.mem.alloc(0x400);
        e.mem.set_f32(world + 0x18, -10.0);
        e.register_double(WORLD_OBJECT_GET_WORLD, move |_, _| ret(world));
        call(&mut e);
        // A speed limit of 10 scales the 20 down.
        e.register(WORLD_OBJECT_GET_WORLD, |_, _| ret(0));
        e.register(GET_MAX_LINEAR_VELOCITY, |_, _| float_ret(10.0));
        call(&mut e);
        assert_eq!(
            *linear.borrow(),
            [
                [10.0, 0.0, 0.0, 0.0],
                [10.0, 0.0, 0.25, 0.0],
                [5.0, 0.0, 0.0, 0.0]
            ]
        );
        assert_eq!(*angular.borrow(), [[0.0; 4]; 3]);
        // The existing velocity is mixed in with the weight 1 - blend.
        let hk = e.get(body, BhkRigidBody::phkObject).addr();
        set_vector(&mut e, hk + 0x1b0, [4.0, 0.0, 0.0, 0.0]);
        set_vector(&mut e, hk + 0x1c0, [0.0, 6.0, 0.0, 0.0]);
        call(&mut e);
        assert_eq!(linear.borrow()[3], [7.0, 0.0, 0.0, 0.0]);
        assert_eq!(angular.borrow()[3], [0.0, 3.0, 0.0, 0.0]);
    }

    /// The `IsNode` doubles: not a node (`0x0900_000c`), or the node itself (`0x0900_000d`).
    const IS_NODE_NONE: u32 = 0x0900_000c;
    const IS_NODE_SELF: u32 = 0x0900_000d;
    /// `getCenterOfMassInWorld`-like double copying 16 bytes from `this + 0x40`.
    const COPY_FROM_OBJECT: u32 = 0x0900_00f0;

    fn install_node_doubles(e: &mut Engine) {
        e.register(IS_NODE_NONE, |_, _| ret(0));
        e.register(IS_NODE_SELF, |_, a| ret(a[0]));
        e.register(GET_RTTI_DOUBLE, |e, a| ret(e.mem.u32(a[0] + 0x20)));
        e.register(COPY_FROM_OBJECT, |e, a| {
            for word in 0..4 {
                let value = e.mem.u32(a[0] + 0x40 + 4 * word);
                e.mem.set_u32(a[1] + 4 * word, value);
            }
            ret(a[1])
        });
    }

    /// A scene-graph leaf with a collision object (type `bhkCollisionObject`) whose rigid
    /// body is the object `rigid`. Returns `(node, collision)`.
    fn leaf_with_body(e: &mut Engine, rigid: u32) -> (u32, u32) {
        install_node_doubles(e);
        let collision = object(e, &[(VSLOT_GET_RTTI, GET_RTTI_DOUBLE)]);
        e.mem.set_u32(collision + 0x20, COLLISION_OBJECT_KEY);
        e.mem.set_u32(collision + 0x10, rigid);
        let node = object(e, &[(VSLOT_IS_NODE, IS_NODE_NONE)]);
        e.mem.set_u32(node + 0x1c, collision);
        (node, collision)
    }

    /// A scene-graph node with `children` and no collision object of its own.
    fn node_with_children(e: &mut Engine, children: &[u32]) -> u32 {
        install_node_doubles(e);
        let node = object(e, &[(VSLOT_IS_NODE, IS_NODE_SELF)]);
        let array = e.mem.alloc(4 * children.len() as u32 + 4);
        for (index, child) in children.iter().enumerate() {
            e.mem.set_u32(array + 4 * index as u32, *child);
        }
        e.mem.set_u32(node + 0xa0, array);
        e.mem.set_u16(node + 0xa6, children.len() as u16);
        node
    }

    /// A rigid body object for the mass-centre tests: world centre of mass `centre`
    /// (copied from `+0x40` by slot `0xf0`) and dynamic mass `mass` (at `+0x20`).
    fn weighted_body(e: &mut Engine, mass: f32, centre: [f32; 4]) -> u32 {
        install_node_doubles(e);
        e.register(GET_DYNAMIC_MASS, |e, a| float_ret(e.mem.f32(a[0] + 0x20)));
        let body = object(e, &[(0xf0, COPY_FROM_OBJECT)]);
        e.mem.set_f32(body + 0x20, mass);
        set_vector(e, body + 0x40, centre);
        body
    }

    /// Sets up the keyframe tests: a body in a world, a current position `(0, 0, 0)` and
    /// rotation `(0, 0, 0, 1)`, the thresholds and logging velocity doubles. Returns the
    /// body, its Havok body and the shared logs of the object velocity setters.
    fn keyframe_fixture(e: &mut Engine) -> (Ptr<BhkRigidBody>, u32, VectorLog, VectorLog) {
        quiet_lock(e);
        set_vector(e, HALF_VECTOR, [-0.5; 4]);
        for word in 0..4 {
            e.mem.set_u32(ABSOLUTE_MASK + 4 * word, 0x7fff_ffff);
        }
        e.set_global(POSITION_SCALE, 1.0f32);
        e.set_global(NEAR_DISTANCE_SQUARED, 1.0e-6f32);
        e.set_global(ROTATION_TOLERANCE, 1.0e-6f32);
        e.set_global(VELOCITY_TOLERANCE, 0.001f32);
        e.set_global(STEP_TIME, 0.5f32);
        e.set_global(SPEED_LIMIT, 1.0e6f64);
        e.register(0x0900_00d4, |e, a| {
            for word in 0..4 {
                let value = e.mem.u32(a[0] + 0x40 + 4 * word);
                e.mem.set_u32(a[1] + 4 * word, value);
            }
            ret(a[1])
        });
        e.register(0x0900_00d8, |e, a| {
            for word in 0..4 {
                let value = e.mem.u32(a[0] + 0x50 + 4 * word);
                e.mem.set_u32(a[1] + 4 * word, value);
            }
            ret(a[1])
        });
        e.register(0x0900_00ec, |e, a| {
            set_vector(e, a[1], [0.0; 4]);
            ret(a[1])
        });
        e.register(0x0900_00e8, |_, _| Ret::default());
        let (body, hk) = rigid_body(
            e,
            &[
                (0xd4, 0x0900_00d4),
                (0xd8, 0x0900_00d8),
                (0xec, 0x0900_00ec),
                (0xf0, 0x0900_00ec),
                (0xe8, 0x0900_00e8),
            ],
        );
        set_vector(e, body.addr() + 0x50, [0.0, 0.0, 0.0, 1.0]);
        let world = e.mem.alloc(0x400);
        e.mem.set_u32(world + 0x350, 1);
        e.mem.set_u32(hk + 8, world);
        e.register(GET_MAX_LINEAR_VELOCITY, |_, _| float_ret(100.0));
        e.register(QUATERNION_DIFFERENCE, |_, _| Ret::default());
        e.register(QUATERNION_NORMALIZE, |_, _| Ret::default());
        e.register(QUATERNION_GET_ANGLE, |_, _| float_ret(0.0005));
        e.register(QUATERNION_GET_AXIS, |e, a| {
            set_vector(e, a[1], [0.0, 1.0, 0.0, 0.0]);
            Ret::default()
        });
        let linear = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&linear);
        e.register_double(OBJECT_SET_LINEAR_VELOCITY, move |e, a| {
            sink.borrow_mut().push(vector(e, a[1]));
            Ret::default()
        });
        let angular = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&angular);
        e.register_double(OBJECT_SET_ANGULAR_VELOCITY, move |e, a| {
            sink.borrow_mut().push(vector(e, a[1]));
            Ret::default()
        });
        (body, hk, linear, angular)
    }

    /// A keyframe position block and a keyframe rotation block (`w, x, y, z`).
    fn keyframe_arguments(e: &mut Engine, position: [f32; 4], rotation: [f32; 4]) -> (u32, u32) {
        let position_block = e.mem.alloc(0x10);
        set_vector(e, position_block, position);
        let rotation_block = e.mem.alloc(0x10);
        set_vector(e, rotation_block, rotation);
        (position_block, rotation_block)
    }

    #[test]
    fn apply_hard_keyframe_only_stops_the_velocity_of_a_body_already_there() {
        let mut e = engine();
        let (body, hk, linear, _) = keyframe_fixture(&mut e);
        set_vector(&mut e, body.addr() + 0x40, [1.0, 2.0, 3.0, 0.0]);
        let (position, rotation) =
            keyframe_arguments(&mut e, [1.0, 2.0, 3.0, 0.0], [1.0, 0.0, 0.0, 0.0]);
        e.register(RIGID_BODY_SET_LINEAR_VELOCITY, |_, _| Ret::default());
        e.register(RIGID_BODY_SET_ANGULAR_VELOCITY, |_, _| Ret::default());
        // A moving body is stopped through the Havok setter between two locks.
        set_vector(&mut e, hk + 0x1b0, [5.0, 0.0, 0.0, 0.0]);
        e.call_log = Some(vec![]);
        e.call(0x00c8_e160, &args![body, position, rotation, 0u32]);
        assert_eq!(
            order(&e),
            [
                0x00c8_e160,
                0x0900_00d4,
                0x0900_00d8,
                EMPTY_LOCK,
                RIGID_BODY_SET_LINEAR_VELOCITY,
                EMPTY_LOCK
            ]
        );
        assert_eq!(
            args_of(&e, RIGID_BODY_SET_LINEAR_VELOCITY),
            [[hk, DEFAULT_VECTOR]]
        );
        // A body within the tolerance of rest is left alone.
        set_vector(&mut e, hk + 0x1b0, [0.0005, 0.0, 0.0, 0.0]);
        set_vector(&mut e, hk + 0x1c0, [0.0, -0.0005, 0.0, 0.0]);
        e.call_log = Some(vec![]);
        e.call(0x00c8_e160, &args![body, position, rotation, 0u32]);
        assert_eq!(order(&e), [0x00c8_e160, 0x0900_00d4, 0x0900_00d8]);
        // A spinning one is stopped through the angular setter.
        set_vector(&mut e, hk + 0x1c0, [0.0, 0.0, 2.0, 0.0]);
        e.call_log = Some(vec![]);
        e.call(0x00c8_e160, &args![body, position, rotation, 0u32]);
        assert_eq!(
            args_of(&e, RIGID_BODY_SET_ANGULAR_VELOCITY),
            [[hk, DEFAULT_VECTOR]]
        );
        assert!(linear.borrow().is_empty());
        // No world, or a world without the word at +0x350: nothing at all.
        e.mem.set_u32(e.mem.u32(hk + 8) + 0x350, 0);
        e.call_log = Some(vec![]);
        e.call(0x00c8_e160, &args![body, position, rotation, 0u32]);
        assert_eq!(order(&e), [0x00c8_e160]);
        e.mem.set_u32(hk + 8, 0);
        e.call(0x00c8_e160, &args![body, position, rotation, 0u32]);
        assert_eq!(order(&e), [0x00c8_e160, 0x00c8_e160]);
    }

    #[test]
    fn apply_hard_keyframe_sets_the_velocity_that_reaches_the_target() {
        let mut e = engine();
        let (body, _, linear, angular) = keyframe_fixture(&mut e);
        let (position, rotation) =
            keyframe_arguments(&mut e, [1.0, 0.0, 0.0, 0.0], [1.0, 0.0, 0.0, 0.0]);
        let differences = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&differences);
        e.register_double(QUATERNION_DIFFERENCE, move |e, a| {
            sink.borrow_mut().push((vector(e, a[1]), vector(e, a[2])));
            Ret::default()
        });
        e.call(0x00c8_e160, &args![body, position, rotation, 0u32]);
        // 1 unit in a 0.5 step: speed 2, and no spin below the angle tolerance.
        assert_eq!(*linear.borrow(), [[2.0, 0.0, 0.0, 0.0]]);
        assert_eq!(*angular.borrow(), [[0.0; 4]]);
        assert_eq!(
            *differences.borrow(),
            [([0.0, 0.0, 0.0, 1.0], [0.0, 0.0, 0.0, 1.0])]
        );
        // A larger angle gives the axis scaled by the angle over the step.
        e.register(QUATERNION_GET_ANGLE, |_, _| float_ret(0.5));
        e.call(0x00c8_e160, &args![body, position, rotation, 0u32]);
        assert_eq!(angular.borrow()[1], [0.0, 1.0, 0.0, 0.0]);
        // A zero step does nothing further.
        e.set_global(STEP_TIME, 0.0f32);
        e.call(0x00c8_e160, &args![body, position, rotation, 0u32]);
        assert_eq!(linear.borrow().len(), 2);
    }

    #[test]
    fn apply_hard_keyframe_teleports_when_the_step_is_too_large() {
        let mut e = engine();
        let (body, _, linear, _) = keyframe_fixture(&mut e);
        let (position, rotation) =
            keyframe_arguments(&mut e, [1.0, 0.0, 0.0, 0.0], [1.0, 0.0, 0.0, 0.0]);
        let sink = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&sink);
        e.register_double(0x0900_00e8, move |e, a| {
            log.borrow_mut().push((vector(e, a[1]), vector(e, a[2])));
            Ret::default()
        });
        e.register(KEYFRAME_POSITION_CONVERT, |e, a| {
            set_vector(e, a[0], [9.0, 9.0, 9.0, 0.0]);
            Ret::default()
        });
        e.register(GET_MAX_LINEAR_VELOCITY, |_, _| float_ret(1.0));
        e.call_log = Some(vec![]);
        e.call(0x00c8_e160, &args![body, position, rotation, 0u32]);
        assert_eq!(
            *sink.borrow(),
            [([9.0, 9.0, 9.0, 0.0], [0.0, 0.0, 0.0, 1.0])]
        );
        assert_eq!(args_of(&e, KEYFRAME_POSITION_CONVERT).len(), 1);
        assert_eq!(args_of(&e, KEYFRAME_POSITION_CONVERT)[0][1], position);
        assert!(linear.borrow().is_empty());
    }

    #[test]
    fn apply_hard_keyframe_stops_and_notifies_above_the_speed_limit() {
        let mut e = engine();
        let (body, _, linear, angular) = keyframe_fixture(&mut e);
        let (position, rotation) =
            keyframe_arguments(&mut e, [1.0, 0.0, 0.0, 0.0], [1.0, 0.0, 0.0, 0.0]);
        e.set_global(SPEED_LIMIT, 1.0f64);
        e.register(GUARD_PREPARE, |_, _| Ret::default());
        e.register(KEYFRAME_TELEPORT_NOTIFY, |_, _| Ret::default());
        let guard = e.mem.alloc(8);
        e.mem.set_u32(guard, 0x1234);
        e.mem.set_u32(guard + 4, 1);
        e.call_log = Some(vec![]);
        e.call(0x00c8_e160, &args![body, position, rotation, guard]);
        assert_eq!(args_of(&e, GUARD_PREPARE), [[guard, 0]]);
        assert_eq!(
            args_of(&e, KEYFRAME_TELEPORT_NOTIFY),
            [[body.addr(), position]]
        );
        assert_eq!(e.mem.u32(guard + 4), 0);
        assert_eq!(e.mem.u32(guard), 0, "the last reference clears the guard");
        assert_eq!(*linear.borrow(), [[0.0; 4]]);
        assert_eq!(angular.borrow().len(), 1);
        // Without a guard the bookkeeping is skipped.
        e.call_log = Some(vec![]);
        e.call(0x00c8_e160, &args![body, position, rotation, 0u32]);
        assert!(args_of(&e, GUARD_PREPARE).is_empty());
        assert_eq!(args_of(&e, KEYFRAME_TELEPORT_NOTIFY).len(), 1);
    }

    #[test]
    fn get_aabb_asks_the_shape_with_the_transform_or_transforms_the_default_box() {
        let mut e = engine();
        quiet_lock(&mut e);
        e.set_global(AABB_LOW_FILL, -0.5f32);
        e.set_global(AABB_HIGH_FILL, 0.25f32);
        e.set_global(AABB_W_FILL, 0.0f32);
        e.register(0x0900_00f4, |_, _| Ret::default());
        e.register(0x0900_001c, |_, _| Ret::default());
        let (body, hk) = rigid_body(&mut e, &[(0xf4, 0x0900_00f4)]);
        let shape = object(&mut e, &[(0x1c, 0x0900_001c)]);
        e.mem.set_u32(hk + 0x10, shape);
        let out = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        e.call(0x00c8_e670, &args![body, out]);
        assert_eq!(
            order(&e),
            [
                0x00c8_e670,
                0x0900_00f4,
                EMPTY_LOCK,
                EMPTY_LOCK,
                0x0900_001c
            ]
        );
        let transform = args_of(&e, 0x0900_00f4)[0][1];
        assert_eq!(
            args_of(&e, 0x0900_001c),
            [[shape, transform, 0.25f32.to_bits(), out]]
        );
        let boxes = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&boxes);
        e.register_double(TRANSFORM_POINT, move |e, a| {
            sink.borrow_mut().push((a[0], a[1], vector(e, a[2])));
            Ret::default()
        });
        e.mem.set_u32(hk + 0x10, 0);
        e.call_log = Some(vec![]);
        e.call(0x00c8_e670, &args![body, out]);
        let transform = args_of(&e, 0x0900_00f4)[0][1];
        assert_eq!(
            *boxes.borrow(),
            [
                (out, transform, [-0.5, -0.5, -0.5, 0.0]),
                (out + 0x10, transform, [0.25, 0.25, 0.25, 0.0])
            ]
        );
    }

    #[test]
    fn remove_action_constraints_removes_items_that_are_in_a_world() {
        let mut e = engine();
        let (body, _) = rigid_body(&mut e, &[]);
        let in_world_constraint = constraint(&mut e);
        let in_world_action = action(&mut e);
        let items = [
            in_world_constraint,
            in_world_action,
            constraint(&mut e),
            other(&mut e),
            action(&mut e),
        ];
        set_items(&mut e, body, &items);
        e.register_double(CONSTRAINT_GET_WORLD, move |_, a| {
            ret(u32::from(a[0] == in_world_constraint))
        });
        e.register_double(WORLD_OBJECT_GET_WORLD, move |_, a| {
            ret(u32::from(a[0] == in_world_action))
        });
        e.call_log = Some(vec![]);
        e.call(0x00c8_e770, &args![body]);
        assert_eq!(
            args_of(&e, ITEM_REMOVE),
            [[in_world_constraint], [in_world_action]]
        );
        assert_eq!(args_of(&e, CONSTRAINT_GET_WORLD), [[items[0]], [items[2]]]);
        assert_eq!(
            args_of(&e, WORLD_OBJECT_GET_WORLD),
            [[items[1]], [items[4]]]
        );
    }

    #[test]
    fn fn_00c8e800_accumulates_the_mass_weighted_centre() {
        let mut e = engine();
        set_vector(&mut e, ONES_VECTOR, [1.0; 4]);
        let heavy = weighted_body(&mut e, 2.0, [4.0, 4.0, 4.0, 0.0]);
        let light = weighted_body(&mut e, 2.0, [0.0; 4]);
        let ignored = weighted_body(&mut e, 100.0, [50.0; 4]);
        let (first, _) = leaf_with_body(&mut e, heavy);
        let (second, _) = leaf_with_body(&mut e, light);
        let (third, collision) = leaf_with_body(&mut e, ignored);
        // The third body's collision object is of another type.
        let unrelated = e.mem.alloc(8);
        e.mem.set_u32(collision + 0x20, unrelated);
        let (empty, _) = leaf_with_body(&mut e, 0);
        let root = node_with_children(&mut e, &[first, second, third, empty]);
        let centre = e.mem.alloc(0x10);
        let total = e.mem.alloc(4);
        e.call(0x00c8_e800, &args![root, centre, total]);
        assert_eq!(e.mem.f32(total), 4.0);
        assert_eq!(vector(&e, centre), [2.0, 2.0, 2.0, 0.0]);
        e.call(0x00c8_e800, &args![Ptr::<()>::NULL, centre, total]);
        assert_eq!(e.mem.f32(total), 4.0);
    }

    #[test]
    fn fn_00c8e940_returns_the_centre_and_total_of_a_tree() {
        let mut e = engine();
        set_vector(&mut e, ONES_VECTOR, [1.0; 4]);
        let body = weighted_body(&mut e, 3.0, [1.0, 2.0, 3.0, 0.0]);
        let (leaf, _) = leaf_with_body(&mut e, body);
        let centre = e.mem.alloc(0x10);
        set_vector(&mut e, centre, [9.0; 4]);
        let total = e.mem.alloc(4);
        e.mem.set_f32(total, 9.0);
        e.call(0x00c8_e940, &args![leaf, centre, total]);
        assert_eq!(vector(&e, centre), [1.0, 2.0, 3.0, 0.0]);
        assert_eq!(e.mem.f32(total), 3.0);
        // Nothing below the root: zero.
        e.call(0x00c8_e940, &args![Ptr::<()>::NULL, centre, total]);
        assert_eq!(vector(&e, centre), [0.0; 4]);
        assert_eq!(e.mem.f32(total), 0.0);
    }

    #[test]
    fn destruct_kills_the_body_and_destroys_the_list_and_entity() {
        let mut e = engine();
        let body = e.new_object::<BhkRigidBody>();
        e.set_global(RIGID_BODY_COUNT, 5u32);
        e.register(ACTION_LIST_CLEAR, |_, _| Ret::default());
        e.register(ENTITY_KILL, |_, _| Ret::default());
        e.register(ACTION_LIST_DESTRUCT, |_, _| Ret::default());
        e.register(ENTITY_DESTRUCT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c8_e9c0, &args![body]);
        assert_eq!(e.mem.u32(body.addr()), BHK_RIGID_BODY_VTABLE);
        assert_eq!(e.global::<u32>(RIGID_BODY_COUNT), 4);
        assert_eq!(
            order(&e),
            [
                0x00c8_e9c0,
                ACTION_LIST_CLEAR,
                ENTITY_KILL,
                ACTION_LIST_DESTRUCT,
                ENTITY_DESTRUCT
            ]
        );
        assert_eq!(args_of(&e, ACTION_LIST_DESTRUCT), [[body.addr() + 0x14]]);
        assert_eq!(args_of(&e, ENTITY_DESTRUCT), [[body.addr()]]);
    }

    #[test]
    fn load_binary_limits_the_velocity_and_fixes_massless_bodies() {
        let mut e = engine();
        e.set_global(MAX_LINEAR_VELOCITY_FLOOR, 1000.0f64);
        e.set_global(DEFAULT_MAX_LINEAR_VELOCITY, 1000.0f32);
        let cinfo = e.mem.alloc(0xe0);
        e.mem.set_f32(cinfo + 0xc8, 5000.0);
        e.mem.set_u32(cinfo, 0x20);
        e.register_double(0x0900_00b4, move |_, _| ret(cinfo));
        let (body, _) = rigid_body(&mut e, &[(0xb4, 0x0900_00b4)]);
        e.register(ENTITY_LOAD_BINARY, |_, _| Ret::default());
        e.register(GET_DYNAMIC_MASS, |_, _| float_ret(0.0));
        e.register(STREAM_READ_MULTIPLE_LINK_IDS, |_, _| Ret::default());
        e.register(0x0900_0040, |e, a| {
            e.mem.set_u32(a[1], 0xabcd);
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (4, 4, 1));
            ret(4)
        });
        let input = e.mem.alloc(0x20);
        e.mem.set_u32(input + 8, 0x0900_0040);
        let stream = e.mem.alloc(0x300);
        e.mem.set_u32(stream + 0x24c, input);
        e.mem.set_u8(stream + 0x1e5, 1);
        e.call_log = Some(vec![]);
        e.call(0x00c8_ea30, &args![body, stream]);
        assert_eq!(e.mem.f32(cinfo + 0xc8), 1000.0);
        assert_eq!(e.mem.u8(cinfo + 0xd4), 5);
        assert_eq!(e.get(body, BhkRigidBody::iFlags), 0xabcd);
        assert_eq!(e.mem.u32(input + 4), 4);
        assert_eq!(
            order(&e),
            [
                0x00c8_ea30,
                ENTITY_LOAD_BINARY,
                0x0900_00b4,
                GET_DYNAMIC_MASS,
                STREAM_READ_MULTIPLE_LINK_IDS,
                0x0900_0040
            ]
        );
        assert_eq!(args_of(&e, ENTITY_LOAD_BINARY), [[body.addr(), stream]]);
        // Layer 0xE is keyframed; layers 2 and 0x1C are left as they were.
        for (layer, motion) in [(0xe, 4u8), (2, 0xff), (0x1c, 0xff)] {
            e.mem.set_u32(cinfo, layer);
            e.mem.set_u8(cinfo + 0xd4, 0xff);
            e.call(0x00c8_ea30, &args![body, stream]);
            assert_eq!(e.mem.u8(cinfo + 0xd4), motion, "layer {layer:#x}");
        }
        // A body with mass keeps its motion type.
        e.register(GET_DYNAMIC_MASS, |_, _| float_ret(1.0));
        e.mem.set_u32(cinfo, 0x20);
        e.call(0x00c8_ea30, &args![body, stream]);
        assert_eq!(e.mem.u8(cinfo + 0xd4), 0xff);
        // A stream not in PC byte order swaps the cinfo first.
        e.mem.set_u8(stream + 0x1e5, 0);
        e.mem.set_u32(cinfo + 0x30, 0x0102_0304);
        e.call(0x00c8_ea30, &args![body, stream]);
        assert_eq!(e.mem.u32(cinfo + 0x30), 0x0403_0201);
    }

    #[test]
    fn remove_clears_two_flag_bits_and_removes_the_items() {
        let mut e = engine();
        let (body, _) = rigid_body(&mut e, &[]);
        let item = constraint(&mut e);
        set_items(&mut e, body, &[item]);
        e.set(body, BhkRigidBody::iFlags, 0xffff);
        e.register(CONSTRAINT_GET_WORLD, |_, _| ret(1));
        e.register(ENTITY_REMOVE, |_, _| ret(0x1234));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c8_eb00, &args![body]).u32(), 0x1234);
        assert_eq!(e.get(body, BhkRigidBody::iFlags), 0xff77);
        assert_eq!(args_of(&e, ITEM_REMOVE), [[item]]);
        assert_eq!(args_of(&e, ENTITY_REMOVE), [[body.addr()]]);
    }

    #[test]
    fn un_fix_removes_constraints_in_the_world_and_adds_the_others() {
        let mut e = engine();
        install_remove_head(&mut e);
        let (body, _) = rigid_body(&mut e, &[]);
        let in_world = constraint(&mut e);
        let out_of_world = constraint(&mut e);
        set_items(&mut e, body, &[in_world, out_of_world]);
        let world = e.mem.alloc(0x400);
        e.mem.set_u32(world + 0x350, 7);
        e.register_double(WORLD_OBJECT_GET_WORLD, move |_, _| ret(world));
        e.register_double(CONSTRAINT_GET_WORLD, move |_, a| {
            ret(u32::from(a[0] == in_world))
        });
        let scope = e.tls() + SCOPE_WORD;
        e.mem.set_u32(scope, 0x55);
        // Bit 2 clear: nothing to do.
        e.call_log = Some(vec![]);
        e.call(0x00c8_eb20, &args![body, true]);
        assert_eq!(order(&e), [0x00c8_eb20]);
        e.set(body, BhkRigidBody::iFlags, 4);
        e.call(0x00c8_eb20, &args![body, true]);
        assert_eq!(args_of(&e, ITEM_REMOVE), [[in_world]]);
        assert_eq!(args_of(&e, ACTION_LIST_REMOVE_HEAD), [[body.addr() + 0x14]]);
        assert_eq!(args_of(&e, ITEM_ADD), [[out_of_world, 7]]);
        assert_eq!(e.get(body, BhkRigidBody::iFlags), 0);
        assert_eq!(e.mem.u32(scope), 0x55);
        // Without adding, the constraint outside the world is only skipped.
        set_items(&mut e, body, &[out_of_world]);
        e.set(body, BhkRigidBody::iFlags, 4);
        e.call_log = Some(vec![]);
        e.call(0x00c8_eb20, &args![body, false]);
        assert!(args_of(&e, ITEM_ADD).is_empty());
        assert_eq!(e.get(body, BhkRigidBody::iFlags), 0);
        // A world without the word at +0x350 leaves the flag set.
        e.mem.set_u32(world + 0x350, 0);
        e.set(body, BhkRigidBody::iFlags, 4);
        e.call(0x00c8_eb20, &args![body, true]);
        assert_eq!(e.get(body, BhkRigidBody::iFlags), 4);
    }

    #[test]
    fn fn_00c8ec60_empties_the_list_removing_items_that_are_in_a_world() {
        let mut e = engine();
        install_remove_head(&mut e);
        let (body, _) = rigid_body(&mut e, &[]);
        let in_world_constraint = constraint(&mut e);
        let in_world_action = action(&mut e);
        let items = [
            in_world_constraint,
            in_world_action,
            constraint(&mut e),
            other(&mut e),
        ];
        set_items(&mut e, body, &items);
        e.register_double(CONSTRAINT_GET_WORLD, move |_, a| {
            ret(u32::from(a[0] == in_world_constraint))
        });
        e.register_double(WORLD_OBJECT_GET_WORLD, move |_, a| {
            ret(u32::from(a[0] == in_world_action))
        });
        e.call_log = Some(vec![]);
        e.call(0x00c8_ec60, &args![body]);
        assert_eq!(
            args_of(&e, ITEM_REMOVE),
            [[in_world_constraint], [in_world_action]]
        );
        let head = body.addr() + 0x14;
        assert_eq!(args_of(&e, ACTION_LIST_REMOVE_HEAD), [[head]; 4]);
        assert_eq!(e.mem.u32(head), 0);
        // An empty list does nothing.
        e.call_log = Some(vec![]);
        e.call(0x00c8_ec60, &args![body]);
        assert_eq!(order(&e), [0x00c8_ec60]);
    }

    /// A rigid body whose list holds one constraint that is not in a world; returns the
    /// body and its list head.
    fn body_with_one_constraint(e: &mut Engine) -> (u32, u32) {
        let (body, _) = rigid_body(e, &[]);
        let item = constraint(e);
        set_items(e, body, &[item]);
        (body.addr(), body.addr() + 0x14)
    }

    #[test]
    fn fn_00c8ecf0_empties_the_lists_of_a_whole_tree() {
        let mut e = engine();
        install_remove_head(&mut e);
        e.register(CONSTRAINT_GET_WORLD, |_, _| ret(0));
        e.register(CAST_TO_COLLISION_OBJECT, |_, a| ret(a[0]));
        let (root_body, root_head) = body_with_one_constraint(&mut e);
        let (first_body, first_head) = body_with_one_constraint(&mut e);
        let (second_body, second_head) = body_with_one_constraint(&mut e);
        let (first, first_collision) = leaf_with_body(&mut e, first_body);
        let (second, second_collision) = leaf_with_body(&mut e, second_body);
        let root = node_with_children(&mut e, &[first, second]);
        let root_collision = e.mem.alloc(0x20);
        e.mem.set_u32(root_collision + 0x10, root_body);
        e.mem.set_u32(root + 0x1c, root_collision);
        e.call_log = Some(vec![]);
        e.call(0x00c8_ecf0, &args![root]);
        assert_eq!(
            args_of(&e, CAST_TO_COLLISION_OBJECT),
            [
                [root_collision, COLLISION_OBJECT_KEY],
                [first_collision, COLLISION_OBJECT_KEY],
                [second_collision, COLLISION_OBJECT_KEY]
            ]
        );
        assert_eq!(
            args_of(&e, ACTION_LIST_REMOVE_HEAD),
            [[root_head], [first_head], [second_head]]
        );
        e.call_log = Some(vec![]);
        e.call(0x00c8_ecf0, &args![Ptr::<()>::NULL]);
        assert_eq!(order(&e), [0x00c8_ecf0]);
    }

    #[test]
    fn fn_00c8ed70_constructs_an_empty_rigid_body() {
        let mut e = engine();
        let block = e.mem.alloc(0x1c);
        for offset in [0xc, 0x10, 0x14, 0x18] {
            e.mem.set_u32(block + offset, 0xdead);
        }
        e.register_double(HEAP_ALLOCATE, move |_, _| ret(block));
        e.register(REF_OBJECT_CONSTRUCT, |_, _| Ret::default());
        e.set_global(SERIALIZABLE_COUNT, 5u32);
        e.set_global(ENTITY_COUNT, 6u32);
        e.set_global(RIGID_BODY_COUNT, 7u32);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c8_ed70, &args![]).u32(), block);
        assert_eq!(args_of(&e, HEAP_ALLOCATE), [[0x1c]]);
        assert_eq!(args_of(&e, REF_OBJECT_CONSTRUCT), [[block]]);
        assert_eq!(e.mem.u32(block), BHK_RIGID_BODY_VTABLE);
        for offset in [0xc, 0x10, 0x14, 0x18] {
            assert_eq!(e.mem.u32(block + offset), 0);
        }
        assert_eq!(e.global::<u32>(SERIALIZABLE_COUNT), 6);
        assert_eq!(e.global::<u32>(ENTITY_COUNT), 7);
        assert_eq!(e.global::<u32>(RIGID_BODY_COUNT), 8);
        // A failed allocation gives null and constructs nothing.
        e.register(HEAP_ALLOCATE, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c8_ed70, &args![]).u32(), 0);
        assert!(args_of(&e, REF_OBJECT_CONSTRUCT).is_empty());
        assert_eq!(e.global::<u32>(RIGID_BODY_COUNT), 8);
    }

    #[test]
    fn add_motion_type_adds_the_name_of_the_motion() {
        let mut e = engine();
        let handles = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&handles);
        e.register_double(VIEWER_STRINGS_ADD, move |e, a| {
            sink.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        e.register(NI_GET_VIEWER_STRING_TEXT, |_, _| ret(0x1234));
        let sets = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&sets);
        e.register_double(STRINGS_SET_AT, move |e, a| {
            sink.borrow_mut().push((a[0], a[1], e.mem.u32(a[2])));
            Ret::default()
        });
        e.register(STRINGS_GROW, |_, _| Ret::default());
        let strings = e.mem.alloc(0x20);
        e.mem.set_u16(strings + 0xe, 4);
        e.call_log = Some(vec![]);
        for (motion, value) in [
            (1i32, 0x010c_60d0u32),
            (2, 0x010c_60c0),
            (3, 0x010c_60bc),
            (4, 0x010c_60b0),
            (5, 0x010c_60a8),
            (6, 0x010c_609c),
        ] {
            e.call(0x00c8_ee20, &args![strings, motion]);
            let logged = args_of(&e, NI_GET_VIEWER_STRING_TEXT);
            assert_eq!(logged.last().unwrap(), &[0x010c_60c8, value]);
        }
        assert_eq!(*handles.borrow(), [(strings, 0x1234); 6]);
        assert!(sets.borrow().is_empty());
        // Anything else is the invalid string, appended after growing the full array.
        for motion in [0i32, 7, -1] {
            e.call(0x00c8_ee20, &args![strings, motion]);
            let logged = args_of(&e, NI_GET_VIEWER_STRING_TEXT);
            assert_eq!(logged.last().unwrap(), &[0x010c_60c8, 0x010c_6094]);
        }
        assert_eq!(handles.borrow().len(), 6);
        assert_eq!(*sets.borrow(), [(strings, 0, 0x1234); 3]);
        assert_eq!(args_of(&e, STRINGS_GROW), [[strings, 4]; 3]);
    }

    #[test]
    fn add_act_con_references_a_constraint_or_action_and_adds_it_to_the_list() {
        let mut e = engine();
        let (body, _) = rigid_body(&mut e, &[]);
        let scope = e.tls() + SCOPE_WORD;
        e.mem.set_u32(scope, 0x55);
        let remaining = Rc::new(Cell::new(1u32));
        let count = Rc::clone(&remaining);
        e.register(INTERLOCKED_INCREMENT, |_, _| ret(2));
        e.register_double(INTERLOCKED_DECREMENT, move |_, _| ret(count.get()));
        let logged = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&logged);
        e.register_double(ACTION_LIST_ADD_HEAD, move |e, a| {
            assert_eq!(e.mem.u32(scope), 0x10);
            sink.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        let head = body.addr() + 0x14;
        let kept = constraint(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00c8_ef80, &args![body, kept]);
        assert_eq!(*logged.borrow(), [(head, kept)]);
        assert_eq!(args_of(&e, INTERLOCKED_INCREMENT), [[kept + 4]]);
        assert_eq!(args_of(&e, INTERLOCKED_DECREMENT), [[kept + 4]]);
        assert!(args_of(&e, ITEM_DESTROY).is_empty());
        assert_eq!(e.mem.u32(scope), 0x55);
        // The last reference destroys the item through its virtual slot 4.
        remaining.set(0);
        let dropped = action(&mut e);
        e.call(0x00c8_ef80, &args![body, dropped]);
        assert_eq!(logged.borrow()[1], (head, dropped));
        assert_eq!(args_of(&e, ITEM_DESTROY), [[dropped]]);
        // Neither a constraint nor an action, or no item at all: nothing happens.
        let stranger = other(&mut e);
        e.call(0x00c8_ef80, &args![body, stranger]);
        e.call(0x00c8_ef80, &args![body, 0u32]);
        assert_eq!(logged.borrow().len(), 2);
        assert_eq!(e.mem.u32(scope), 0x55);
    }

    #[test]
    fn fix_replaces_constraints_by_fixed_constraints() {
        let mut e = engine();
        install_reference_counting(&mut e);
        let (body, hk) = rigid_body(&mut e, &[]);
        let old = constraint(&mut e);
        let stranger = other(&mut e);
        set_items(&mut e, body, &[old, stranger]);
        let replacement = constraint(&mut e);
        let world = e.mem.alloc(0x400);
        e.mem.set_u32(world + 0x350, 7);
        e.register_double(WORLD_OBJECT_GET_WORLD, move |_, _| ret(world));
        e.register_double(FIX_CONSTRAINT, move |_, _| ret(replacement));
        e.set_global(FIXED_CONSTRAINT_STRENGTH, 0.3f32);
        let scope = e.tls() + SCOPE_WORD;
        e.mem.set_u32(scope, 0x55);
        for (filter, strength) in [
            (0x1108u32, 0.3f32),
            (0x151d, 0.3),
            (0x161d, 0.3),
            (0x1708, 1.0),
            (0x1103, 1.0),
            (0x0008, 1.0),
        ] {
            e.mem.set_u32(hk + 0x2c, filter);
            e.set(body, BhkRigidBody::iFlags, 0);
            e.call_log = Some(vec![]);
            e.call(0x00c8_f080, &args![body]);
            assert_eq!(
                args_of(&e, FIX_CONSTRAINT),
                [[old, strength.to_bits()]],
                "filter {filter:#x}"
            );
            assert_eq!(args_of(&e, ITEM_REMOVE), [[old]]);
            assert_eq!(args_of(&e, ITEM_ADD), [[replacement, 7]]);
            assert_eq!(args_of(&e, ACTION_LIST_ADD_HEAD).len(), 1);
            assert_eq!(e.get(body, BhkRigidBody::iFlags), 4);
        }
        assert_eq!(e.mem.u32(scope), 0x55);
        // Already fixed: nothing happens.
        e.call_log = Some(vec![]);
        e.call(0x00c8_f080, &args![body]);
        assert_eq!(order(&e), [0x00c8_f080]);
        // A constraint without a replacement is left alone.
        e.set(body, BhkRigidBody::iFlags, 0);
        e.register(FIX_CONSTRAINT, |_, _| ret(0));
        e.call(0x00c8_f080, &args![body]);
        assert!(args_of(&e, ITEM_REMOVE).is_empty());
        // A world without the word at +0x350: the flag stays clear.
        e.set(body, BhkRigidBody::iFlags, 0);
        e.mem.set_u32(world + 0x350, 0);
        e.call(0x00c8_f080, &args![body]);
        assert_eq!(e.get(body, BhkRigidBody::iFlags), 0);
    }

    #[test]
    fn set_fixed_fixes_or_unfixes_the_bodies_of_a_tree() {
        let mut e = engine();
        e.register(NODE_PREPARE, |_, _| Ret::default());
        e.register(CAST_TO_COLLISION_OBJECT, |_, a| ret(a[0]));
        e.register(WORLD_OBJECT_GET_WORLD, |_, _| ret(0));
        let (root_body, _) = rigid_body(&mut e, &[]);
        let (child_body, _) = rigid_body(&mut e, &[]);
        let (child, _) = leaf_with_body(&mut e, child_body.addr());
        let root = node_with_children(&mut e, &[child]);
        let collision = e.mem.alloc(0x20);
        e.mem.set_u32(collision + 0x10, root_body.addr());
        e.mem.set_u32(root + 0x1c, collision);
        // Fixing runs `Fix` on both bodies; with the flags clear it asks for the world.
        e.call_log = Some(vec![]);
        e.call(0x00c8_f210, &args![root, true, 0u32]);
        assert_eq!(args_of(&e, NODE_PREPARE), [[root], [child]]);
        assert_eq!(
            args_of(&e, WORLD_OBJECT_GET_WORLD),
            [[root_body.addr()], [child_body.addr()]]
        );
        // Unfixing bodies whose constraints are not fixed does nothing.
        e.call_log = Some(vec![]);
        e.call(0x00c8_f210, &args![root, false, 0u32]);
        assert!(args_of(&e, WORLD_OBJECT_GET_WORLD).is_empty());
        // ... and with bit 2 set it asks for the world, on both bodies.
        e.set(root_body, BhkRigidBody::iFlags, 4);
        e.set(child_body, BhkRigidBody::iFlags, 4);
        e.call(0x00c8_f210, &args![root, false, 1u32]);
        assert_eq!(args_of(&e, WORLD_OBJECT_GET_WORLD).len(), 2);
        e.call_log = Some(vec![]);
        e.call(0x00c8_f210, &args![Ptr::<()>::NULL, true, 0u32]);
        assert_eq!(order(&e), [0x00c8_f210]);
    }

    #[test]
    fn copy_members_copies_the_cinfo_flags_and_cloned_items() {
        let mut e = engine();
        quiet_lock(&mut e);
        install_reference_counting(&mut e);
        e.register(WORLD_OBJECT_COPY_MEMBERS, |_, _| Ret::default());
        e.register(0x0900_00c4, |_, _| Ret::default());
        let cinfo = e.mem.alloc(0xe0);
        e.mem.set_f32(cinfo + 0xb0, 1.5);
        e.register_double(0x0900_00b4, move |_, _| ret(cinfo));
        let (this, this_hk) = rigid_body(&mut e, &[(0xb4, 0x0900_00b4)]);
        let (destination, destination_hk) = rigid_body(&mut e, &[(0xc4, 0x0900_00c4)]);
        e.mem.set_u32(this_hk + 0x210, 0x5150);
        e.mem.set_u32(destination_hk + 0x2c, 0x11);
        e.set(this, BhkRigidBody::iFlags, 0x239);
        e.set(destination, BhkRigidBody::iFlags, 0x4);
        let original = constraint(&mut e);
        set_items(&mut e, this, &[original]);
        let clone = constraint(&mut e);
        e.register_double(ITEM_CLONE, move |_, _| ret(clone));
        let context = e.mem.alloc(0x20);
        e.mem.set_f32(context + 0x10, 2.0);
        e.call_log = Some(vec![]);
        e.call(0x00c8_f2a0, &args![this, destination, context]);
        assert_eq!(e.mem.f32(cinfo + 0xb0), 3.0, "scaled by 2");
        assert_eq!(e.mem.u8(cinfo + 0xd4), 1, "bit 9 of the flags");
        assert_eq!(e.mem.u32(cinfo + 0x28), 0x5150);
        assert_eq!(
            args_of(&e, WORLD_OBJECT_COPY_MEMBERS),
            [[this.addr(), destination.addr(), context]]
        );
        assert_eq!(e.get(destination, BhkRigidBody::iFlags), 5);
        assert_eq!(e.mem.u32(destination_hk + 0x2c), 0x2011);
        assert_eq!(args_of(&e, EMPTY_LOCK).len(), 2);
        assert_eq!(args_of(&e, 0x0900_00c4), [[destination.addr()]]);
        assert_eq!(args_of(&e, ITEM_CLONE), [[original, context]]);
        assert_eq!(
            args_of(&e, ACTION_LIST_ADD_HEAD)[0][0],
            destination.addr() + 0x14
        );
        // The scale 1.0 and a filter without bit 13: no filter update.
        e.mem.set_f32(context + 0x10, 1.0);
        e.set(this, BhkRigidBody::iFlags, 0);
        e.call_log = Some(vec![]);
        e.call(0x00c8_f2a0, &args![this, destination, context]);
        assert!(args_of(&e, 0x0900_00c4).is_empty());
        assert_eq!(e.mem.u8(cinfo + 0xd4), 1, "left as it was");
        // Bit 13 of the body's filter word forces the update.
        e.mem.set_u32(this_hk + 0x2c, 0x2000);
        e.mem.set_u32(destination_hk + 0x2c, 0);
        e.call(0x00c8_f2a0, &args![this, destination, context]);
        assert_eq!(args_of(&e, 0x0900_00c4).len(), 1);
        assert_eq!(e.mem.u32(destination_hk + 0x2c), 0x2000);
    }

    #[test]
    fn link_object_adds_every_linked_object_that_exists() {
        let mut e = engine();
        install_reference_counting(&mut e);
        let (body, _) = rigid_body(&mut e, &[]);
        let first = constraint(&mut e);
        let second = action(&mut e);
        e.register(WORLD_OBJECT_LINK_OBJECT, |_, _| Ret::default());
        e.register(STREAM_GET_NUMBER_OF_LINK_IDS, |_, _| ret(3));
        let objects = Rc::new(RefCell::new(vec![second, 0, first]));
        let queue = Rc::clone(&objects);
        e.register_double(STREAM_GET_OBJECT_FROM_LINK_ID, move |_, _| {
            ret(queue.borrow_mut().pop().unwrap())
        });
        let logged = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&logged);
        e.register_double(ACTION_LIST_ADD_HEAD, move |e, a| {
            sink.borrow_mut().push(e.mem.u32(a[1]));
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00c8_f3c0, &args![body, 0x5000u32]);
        assert_eq!(
            args_of(&e, WORLD_OBJECT_LINK_OBJECT),
            [[body.addr(), 0x5000]]
        );
        assert_eq!(args_of(&e, STREAM_GET_OBJECT_FROM_LINK_ID), [[0x5000]; 3]);
        assert_eq!(*logged.borrow(), [first, second]);
        // No link ids: no object is asked for.
        e.register(STREAM_GET_NUMBER_OF_LINK_IDS, |_, _| ret(0));
        e.call_log = Some(vec![]);
        e.call(0x00c8_f3c0, &args![body, 0x5000u32]);
        assert!(args_of(&e, STREAM_GET_OBJECT_FROM_LINK_ID).is_empty());
    }

    #[test]
    fn create_clone_builds_and_fills_a_new_rigid_body() {
        let mut e = engine();
        quiet_lock(&mut e);
        let cinfo = e.mem.alloc(0xe0);
        e.register_double(0x0900_00b4, move |_, _| ret(cinfo));
        e.register(0x0900_00fc, |_, _| Ret::default());
        let (this, this_hk) = rigid_body(&mut e, &[(0xb4, 0x0900_00b4), (0xfc, 0x0900_00fc)]);
        e.mem.set_u32(this_hk + 0x210, 0x77);
        let block = e.mem.alloc(0x40);
        e.register_double(HEAP_ALLOCATE, move |_, _| ret(block));
        e.register(REF_OBJECT_CONSTRUCT, |_, _| Ret::default());
        e.register(WORLD_OBJECT_COPY_MEMBERS, |_, _| Ret::default());
        let scope = e.tls() + SCOPE_WORD;
        e.mem.set_u32(scope, 0x55);
        for address in [CLONE_LOCK, CLONE_UNLOCK] {
            e.register_double(address, move |e, _| {
                assert_eq!(e.mem.u32(scope), 0x10);
                Ret::default()
            });
        }
        let context = e.mem.alloc(0x20);
        e.mem.set_f32(context + 0x10, 1.0);
        e.set_global(RIGID_BODY_COUNT, 0u32);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c8_f400, &args![this, context]).u32(), block);
        assert_eq!(
            order(&e),
            [
                0x00c8_f400,
                CLONE_LOCK,
                0x0900_00fc,
                HEAP_ALLOCATE,
                REF_OBJECT_CONSTRUCT,
                0x0900_00b4,
                WORLD_OBJECT_COPY_MEMBERS,
                0x0900_00fc,
                CLONE_UNLOCK
            ]
        );
        assert_eq!(
            args_of(&e, 0x0900_00fc),
            [[this.addr(), 0], [this.addr(), 1]]
        );
        assert_eq!(
            args_of(&e, WORLD_OBJECT_COPY_MEMBERS),
            [[this.addr(), block, context]]
        );
        assert_eq!(e.mem.u32(block), BHK_RIGID_BODY_VTABLE);
        assert_eq!(e.global::<u32>(RIGID_BODY_COUNT), 1);
        assert_eq!(e.mem.u32(scope), 0x55);
    }

    #[test]
    fn cinfo_construct_sets_the_defaults() {
        let mut e = engine();
        e.set_global(DEFAULT_CINFO_MAX_LINEAR_VELOCITY, 10000.0f32);
        e.set_global(DEFAULT_CINFO_MAX_ANGULAR_VELOCITY, 31.4159f32);
        e.register(RIGID_BODY_CINFO_HK_CONSTRUCT, |_, _| Ret::default());
        let cinfo = e.mem.alloc(0xe0);
        for word in 0..0x38 {
            e.mem.set_u32(cinfo + 4 * word, 0xaaaa_aaaa);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c8_f510, &args![cinfo]).u32(), cinfo);
        assert_eq!(args_of(&e, RIGID_BODY_CINFO_HK_CONSTRUCT), [[cinfo + 0x20]]);
        assert_eq!(e.mem.u32(cinfo + 0xc), 0);
        assert_eq!(e.mem.u32(cinfo + 0x10), 0);
        assert_eq!(e.mem.u32(cinfo + 0x14), 0x8000_0000);
        assert_eq!(e.mem.u8(cinfo + 8), 1);
        assert_eq!(e.mem.u32(cinfo), 0);
        assert_eq!(e.mem.u32(cinfo + 4), 0);
        assert_eq!(e.mem.u8(cinfo + 0x18), 1);
        assert_eq!(e.mem.u16(cinfo + 0x1a), 0xffff);
        assert_eq!(e.mem.f32(cinfo + 0xc8), 10000.0);
        assert_eq!(e.mem.u8(cinfo + 0xd7), 0xff);
        assert_eq!(e.mem.f32(cinfo + 0xcc), 31.4159);
    }

    #[test]
    fn create_cinfo_allocates_an_aligned_cinfo_once() {
        let mut e = engine();
        e.register(RIGID_BODY_CINFO_HK_CONSTRUCT, |_, _| Ret::default());
        let raw = e.mem.alloc(0x120);
        let raw = raw + ((4 + 16 - (raw & 0xf)) & 0xf);
        e.register_double(MEMORY_MANAGER_ALLOCATE, move |_, _| ret(raw));
        let (body, hk) = rigid_body(&mut e, &[]);
        e.set(body, BhkRigidBody::phkObject, Ptr::NULL);
        let scope = e.tls() + SCOPE_WORD;
        e.mem.set_u32(scope, 0x55);
        let created = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        let cinfo = e.call(0x00c8_f5a0, &args![body, created]).u32();
        assert_eq!(cinfo, raw + 0xc);
        assert_eq!(e.mem.u8(created), 1);
        assert_eq!(e.mem.u8(cinfo - 1), 0xc);
        assert_eq!(cinfo & 0xf, 0);
        assert_eq!(e.get(body, BhkRigidBody::pInfo).addr(), cinfo);
        assert_eq!(
            args_of(&e, MEMORY_MANAGER_ALLOCATE),
            [[MEMORY_MANAGER, 0xf0]]
        );
        assert_eq!(e.mem.u32(cinfo + 0x14), 0x8000_0000);
        assert_eq!(e.mem.u32(scope), 0x55);
        // The second call returns the same cinfo.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c8_f5a0, &args![body, created]).u32(), cinfo);
        assert_eq!(e.mem.u8(created), 0);
        assert!(args_of(&e, MEMORY_MANAGER_ALLOCATE).is_empty());
        // With a Havok body the new cinfo is filled from it.
        e.set(body, BhkRigidBody::pInfo, Ptr::NULL);
        e.set(body, BhkRigidBody::phkObject, Ptr::new(hk));
        e.register(WORLD_OBJECT_COPY_CINFO, |_, _| Ret::default());
        e.register(RIGID_BODY_GET_CINFO, |_, _| Ret::default());
        e.register(RIGID_BODY_GET_MOTION, |_, _| ret(0));
        e.register(GET_MAX_LINEAR_VELOCITY, |_, _| float_ret(1.0));
        e.register(HK_UFLOAT8_ASSIGN, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c8_f5a0, &args![body, created]);
        assert_eq!(args_of(&e, WORLD_OBJECT_COPY_CINFO), [[body.addr(), cinfo]]);
    }

    #[test]
    fn fn_00c8f670_frees_the_property_array_and_the_block() {
        let mut e = engine();
        install_heap(&mut e);
        e.register(MEMORY_MANAGER_FREE, |_, _| Ret::default());
        let raw = e.mem.alloc(0x100);
        let cinfo = raw + 8;
        e.mem.set_u8(cinfo - 1, 8);
        e.mem.set_u32(cinfo + 0xc, 0x1234);
        e.mem.set_u32(cinfo + 0x10, 9);
        e.mem.set_u32(cinfo + 0x14, 4);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c8_f670, &args![cinfo, 0u32]).u32(), cinfo);
        let freed = args_of(&e, FREE_BYTES_DOUBLE);
        assert_eq!(freed.len(), 1);
        assert_eq!(freed[0][1..], [0x1234, 0x20]);
        assert_eq!(e.mem.u32(cinfo + 0xc), 0);
        assert_eq!(e.mem.u32(cinfo + 0x10), 0);
        assert_eq!(e.mem.u32(cinfo + 0x14), 0x8000_0000);
        assert!(args_of(&e, MEMORY_MANAGER_FREE).is_empty());
        // Bit 0 of the flags also frees the block itself (the shift is stored in front).
        e.call_log = Some(vec![]);
        e.call(0x00c8_f670, &args![cinfo, 1u32]);
        assert!(args_of(&e, FREE_BYTES_DOUBLE).is_empty(), "unowned storage");
        assert_eq!(args_of(&e, MEMORY_MANAGER_FREE), [[MEMORY_MANAGER, raw]]);
    }

    #[test]
    fn load_cinfo_uses_the_serializable_loader_for_new_streams() {
        let mut e = engine();
        let cinfo = e.mem.alloc(0xe0);
        e.register(0x0900_00a8, |_, _| ret(0x99));
        e.register_double(0x0900_00b4, move |_, _| ret(cinfo));
        let (body, _) = rigid_body(&mut e, &[(0xa8, 0x0900_00a8), (0xb4, 0x0900_00b4)]);
        e.register(SERIALIZABLE_LOAD_CINFO, |_, _| Ret::default());
        let stream = e.mem.alloc(0x300);
        e.mem.set_u32(stream + 4, 0x23);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c8_f6e0, &args![body, stream]).u32(), 0x99);
        assert_eq!(
            order(&e),
            [
                0x00c8_f6e0,
                0x0900_00a8,
                0x0900_00b4,
                SERIALIZABLE_LOAD_CINFO
            ]
        );
        assert_eq!(
            args_of(&e, SERIALIZABLE_LOAD_CINFO),
            [[body.addr(), stream]]
        );
    }

    #[test]
    fn load_cinfo_converts_an_old_cinfo_and_reports_it() {
        let mut e = engine();
        install_heap(&mut e);
        let cinfo = e.mem.alloc(0xe0);
        e.register(0x0900_00a8, |_, _| ret(0x99));
        e.register_double(0x0900_00b4, move |_, _| ret(cinfo));
        let (body, _) = rigid_body(&mut e, &[(0xa8, 0x0900_00a8), (0xb4, 0x0900_00b4)]);
        e.register(0x0900_00aa, |e, a| {
            assert_eq!((a[2], a[3], a[4]), (0xe0, 0, 0));
            e.mem.set_u32(a[1], 0x1234);
            e.mem.set_u32(a[1] + 0x24, 0x5678);
            ret(0xe0)
        });
        let input = e.mem.alloc(0x20);
        e.mem.set_u32(input + 8, 0x0900_00aa);
        let stream = e.mem.alloc(0x300);
        e.mem.set_u32(stream + 4, 0x22);
        e.mem.set_u32(stream + 0x24c, input);
        e.register(FORMAT_TEXT, |_, _| Ret::default());
        e.register(0x0900_000c, |_, _| Ret::default());
        let reporter = object(&mut e, &[(HAVOK_ERROR_SLOT_MESSAGE, 0x0900_000c)]);
        e.set_global(HAVOK_ERROR_OBJECT, reporter);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c8_f6e0, &args![body, stream]).u32(), 0x99);
        assert_eq!(e.mem.u32(cinfo), 0x1234);
        assert_eq!(e.mem.u32(cinfo + 0x20), 0x1234);
        assert_eq!(e.mem.u32(cinfo + 0x24), 0x5678);
        assert_eq!(e.mem.u32(input + 4), 0xe0);
        let formatted = args_of(&e, FORMAT_TEXT);
        assert_eq!(formatted.len(), 1);
        let text = formatted[0][0];
        assert_eq!(formatted[0][1..], [0x100, OLD_CINFO_MESSAGE, stream + 0xe0]);
        assert_eq!(
            args_of(&e, 0x0900_000c),
            [[reporter, 0, 0x234f_2250, text, HAVOK_SOURCE_FILE, 0x220]]
        );
    }

    #[test]
    fn get_viewer_strings_describes_the_body_and_its_actions() {
        let mut e = engine();
        install_heap(&mut e);
        e.register(ENTITY_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(RIGID_BODY_CINFO_HK_CONSTRUCT, |_, _| Ret::default());
        e.register(WORLD_OBJECT_COPY_CINFO, |e, a| {
            let cinfo = a[1];
            set_vector(e, cinfo + 0x30, [1.0, 2.0, 3.0, 9.0]);
            set_vector(e, cinfo + 0x40, [0.5, 0.25, 0.125, 1.0]);
            set_vector(e, cinfo + 0xa0, [4.0, 5.0, 6.0, 9.0]);
            set_vector(e, cinfo + 0x50, [7.0, 8.0, 9.0, 9.0]);
            set_vector(e, cinfo + 0x60, [10.0, 11.0, 12.0, 9.0]);
            for (offset, value) in [(0x70, 13.0), (0x84, 14.0), (0x98, 15.0)] {
                e.mem.set_f32(cinfo + offset, value);
            }
            for (offset, value) in [
                (0xb0, 7.5),
                (0xb4, 0.25),
                (0xb8, 0.5),
                (0xc0, 0.75),
                (0xc4, 0.125),
                (0xc8, 100.0),
                (0xcc, 50.0),
                (0xd0, 0.5),
            ] {
                e.mem.set_f32(cinfo + offset, value);
            }
            e.mem.set_u8(cinfo + 0xd4, 2);
            e.mem.set_u8(cinfo + 0xd7, 2);
            Ret::default()
        });
        e.register(NI_POINT3_FROM_VECTOR, |e, a| {
            for lane in 0..3 {
                let value = e.mem.u32(a[1] + 4 * lane);
                e.mem.set_u32(a[0] + 4 * lane, value);
            }
            Ret::default()
        });
        let next = Rc::new(Cell::new(0x100u32));
        let points = Rc::new(RefCell::new(Vec::new()));
        let rotations = Rc::new(RefCell::new(Vec::new()));
        for address in [
            NI_GET_VIEWER_STRING,
            NI_GET_VIEWER_STRING_TEXT,
            NI_GET_VIEWER_STRING_FLOAT,
            NI_GET_VIEWER_STRING_INT,
        ] {
            let next = Rc::clone(&next);
            e.register_double(address, move |_, _| {
                next.set(next.get() + 1);
                ret(next.get() - 1)
            });
        }
        let counter = Rc::clone(&next);
        let sink = Rc::clone(&points);
        e.register_double(NI_POINT3_GET_VIEWER_STRING, move |e, a| {
            sink.borrow_mut()
                .push((a[1], [0, 4, 8].map(|lane| e.mem.f32(a[0] + lane))));
            counter.set(counter.get() + 1);
            ret(counter.get() - 1)
        });
        let counter = Rc::clone(&next);
        let sink = Rc::clone(&rotations);
        e.register_double(NI_QUATERNION_GET_VIEWER_STRING, move |e, a| {
            sink.borrow_mut().push((a[1], vector(e, a[0])));
            counter.set(counter.get() + 1);
            ret(counter.get() - 1)
        });
        e.register(VIEWER_STRINGS_ADD, |e, a| {
            let handle = e.mem.u32(a[1]);
            push_into_array(e, a[0], handle);
            Ret::default()
        });
        e.register(STRINGS_SET_AT, |e, a| {
            let handle = e.mem.u32(a[2]);
            push_into_array(e, a[0], handle);
            Ret::default()
        });
        e.register(STRINGS_GROW, |_, _| panic!("the array is large enough"));
        e.register(FORMAT_TEXT, |_, _| Ret::default());
        e.mem.set_u32(CONSTRAINT_RTTI, 0x5555);
        let (body, _) = rigid_body(&mut e, &[]);
        e.set(body, BhkRigidBody::phkObject, Ptr::NULL);
        let first = constraint(&mut e);
        let second = constraint(&mut e);
        set_items(&mut e, body, &[first, second]);
        let strings = e.mem.alloc(0x20);
        let storage = e.mem.alloc(0x200);
        e.mem.set_u32(strings + 4, storage);
        e.mem.set_u16(strings + 8, 100);
        e.mem.set_u16(strings + 0xe, 4);
        let scope = e.tls() + SCOPE_WORD;
        e.mem.set_u32(scope, 0x55);
        e.call_log = Some(vec![]);
        e.call(0x00c8_f870, &args![body, strings]);
        let base = e.mem.u32(strings + 4);
        let handles: Vec<u32> = (0..20).map(|index| e.mem.u32(base + 4 * index)).collect();
        assert_eq!(handles, (0x100..0x114).collect::<Vec<u32>>());
        assert_eq!(e.mem.u16(strings + 0xa), 20);
        assert_eq!(
            e.mem.u16(strings + 0xc),
            2,
            "the two ACTCON strings are counted"
        );
        assert_eq!(e.mem.u32(scope), 0x55);
        assert_eq!(
            args_of(&e, ENTITY_GET_VIEWER_STRINGS),
            [[body.addr(), strings]]
        );
        let floats: Vec<Vec<u32>> = args_of(&e, NI_GET_VIEWER_STRING_FLOAT);
        let expected = [
            (MASS_NAME, 7.5f32),
            (LINEAR_DAMPING_NAME, 0.25),
            (ANGULAR_DAMPING_NAME, 0.5),
            (FRICTION_NAME, 0.75),
            (RESTITUTION_NAME, 0.125),
            (MAX_LINEAR_VELOCITY_NAME, 100.0),
            (MAX_ANGULAR_VELOCITY_NAME, 50.0),
            (PENETRATION_DEPTH_NAME, 0.5),
        ];
        assert_eq!(
            floats,
            expected
                .iter()
                .map(|(name, value)| vec![*name, value.to_bits()])
                .collect::<Vec<_>>()
        );
        assert_eq!(
            *points.borrow(),
            [
                (POSITION_NAME, [1.0, 2.0, 3.0]),
                (CENTER_OF_MASS_NAME, [4.0, 5.0, 6.0]),
                (LINEAR_VELOCITY_NAME, [7.0, 8.0, 9.0]),
                (ANGULAR_VELOCITY_NAME, [10.0, 11.0, 12.0]),
                (INERTIA_TENSOR_NAME, [13.0, 14.0, 15.0])
            ]
        );
        assert_eq!(
            *rotations.borrow(),
            [(ROTATION_NAME, [1.0, 0.5, 0.25, 0.125])]
        );
        assert_eq!(
            args_of(&e, NI_GET_VIEWER_STRING_TEXT),
            [
                vec![0x010c_60c8, 0x010c_60c0],
                vec![QUALITY_TYPE_NAME, 0x0102_9400],
                vec![ACTION_ITEM_NAME, args_of(&e, FORMAT_TEXT)[0][0]],
                vec![ACTION_ITEM_NAME, args_of(&e, FORMAT_TEXT)[1][0]]
            ]
        );
        assert_eq!(
            args_of(&e, NI_GET_VIEWER_STRING_INT),
            [[ACTION_COUNT_NAME, 2]]
        );
        assert_eq!(
            args_of(&e, NI_GET_VIEWER_STRING),
            [[e.global::<u32>(RIGID_BODY_CLASS_NAME)]]
        );
        let formatted = args_of(&e, FORMAT_TEXT);
        assert_eq!(formatted[0][1..], [0x80, ITEM_FORMAT, 0x5555, first]);
        assert_eq!(formatted[1][1..], [0x80, ITEM_FORMAT, 0x5555, second]);
    }

    /// The array double of `GetViewerStrings`: stores `handle` at the old size and grows the
    /// size by one.
    fn push_into_array(e: &mut Engine, strings: u32, handle: u32) {
        let size = u32::from(e.mem.u16(strings + 0xa));
        let base = e.mem.u32(strings + 4);
        e.mem.set_u32(base + 4 * size, handle);
        e.mem.set_u16(strings + 0xa, (size + 1) as u16);
    }
}
